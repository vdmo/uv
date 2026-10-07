//! Phases 2 and 3 of the driver: the compile-time pass, then name resolution, type
//! checking and the whole-program checks, in the order of the reference driver.
//!
//! A phase that is not ported yet stops the run with `pending` set; the driver then
//! reports the build as not implemented instead of a result that would not mean anything.

use std::collections::HashSet;

use uv_analysis::caps::authority_model::validate_module_authority;
use uv_analysis::caps::callgraph_caps::{build_call_graph, propagate_capability_requirements, validate_capability_chain};
use uv_analysis::context::{Scope, ScopeContext};
use uv_analysis::resolve::assembly_import_graph::{build_assembly_import_graph, validate_assembly_import_graph_structure, validate_hosted_library_import_graph};
use uv_analysis::resolve::collect_toplevel::collect_name_maps;
use uv_analysis::resolve::populate_sigma::populate_sigma;
use uv_analysis::resolve::resolve_module::resolve_modules;
use uv_analysis::resolve::resolver::ResolveContext;
use uv_analysis::resolve::scopes_lookup::module_names_of;
use uv_analysis::resolve::visibility::{can_access, check_module_visibility};
use uv_analysis::typing::comptime_avail::validate_comptime_procedure_signatures;
use uv_analysis::typing::typecheck::typecheck_modules;
use uv_comptime::{execute_comptime, ComptimePassOptions};
use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, has_error, DiagnosticStream};
use uv_project::project::Project;
use uv_project::target_profile::TargetProfile;
use uv_source::ast::ASTModule;
use uv_source::phase1::{emit_internal_diagnostic, Phase1Result};

#[derive(Default)]
pub struct SemaOutcome {
    /// What is not ported and was reached, if anything.
    pub pending: Option<String>,
    /// The IR of the declarations that lower, when the run got that far.
    pub ir: Option<uv_codegen::ir::IrDecls>,
    /// Each declaration that could not be lowered, with what stopped it.
    pub pending_decls: Vec<(String, String)>,
}

fn comptime_options(project: &Project) -> ComptimePassOptions {
    ComptimePassOptions {
        project_root: project.root.clone(),
        fallback_source_root: Some(project.source_root.clone()),
        source_roots_by_assembly: project.assemblies.iter().map(|assembly| (assembly.name.clone(), assembly.source_root.clone())).collect(),
    }
}

fn pending(what: &str) -> SemaOutcome {
    SemaOutcome { pending: Some(what.to_string()), ir: None, pending_decls: Vec::new() }
}

pub fn run_sema(project: &Project, phase1: &Phase1Result, target_profile: TargetProfile, diags: &mut DiagnosticStream, mut progress: impl FnMut(&str, &str)) -> SemaOutcome {
    // Phase 2: the compile-time pass over every parsed module.
    let signature_diags = {
        let mut signature_project = project.clone();
        signature_project.modules = phase1.reachable_module_infos().to_vec();
        let mut signature_ctx = ScopeContext {
            project: Some(&signature_project),
            target_profile: Some(target_profile),
            scopes: vec![Scope::new(), Scope::new(), Scope::new()],
            ..Default::default()
        };
        std::sync::Arc::make_mut(&mut signature_ctx.sigma).mods = phase1.project_modules.clone();
        std::sync::Arc::make_mut(&mut signature_ctx.sigma).unsafe_spans_by_file = phase1.unsafe_spans_by_file.clone();
        validate_comptime_procedure_signatures(&mut signature_ctx, &phase1.project_modules)
    };
    let signatures_failed = has_error(&signature_diags);
    for diag in signature_diags {
        emit(diags, diag);
    }
    if signatures_failed {
        return SemaOutcome::default();
    }
    let expanded = execute_comptime(&phase1.project_modules, &comptime_options(project));
    let comptime_failed = has_error(&expanded.diags);
    for diag in expanded.diags {
        emit(diags, diag);
    }
    let Some(project_modules) = expanded.modules.filter(|_| !comptime_failed) else {
        return SemaOutcome::default();
    };
    // The graph of imports between assemblies reads every module, reachable or not.
    let graph_modules = project_modules.clone();
    let reachable: HashSet<String> = phase1.reachable_module_infos().iter().map(|info| info.path.clone()).collect();
    let parsed_modules: Vec<_> =
        project_modules.into_iter().filter(|module| reachable.contains(&uv_core::symbols::string_of_path(&module.path))).collect();

    if has_error(diags) {
        return SemaOutcome::default();
    }
    // Phase 3.
    progress("Checking", &project.assembly.name);
    let mut sema_project = project.clone();
    sema_project.modules = phase1.reachable_module_infos().to_vec();
    let mut ctx = ScopeContext {
        project: Some(&sema_project),
        target_profile: Some(target_profile),
        scopes: vec![Scope::new(), Scope::new(), Scope::new()],
        ..Default::default()
    };
    std::sync::Arc::make_mut(&mut ctx.sigma).mods = parsed_modules;
    std::sync::Arc::make_mut(&mut ctx.sigma).unsafe_spans_by_file = phase1.unsafe_spans_by_file.clone();
    for index in 0..ctx.sigma.mods.len() {
        ctx.current_module = ctx.sigma.mods[index].path.clone();
        for diag in check_module_visibility(&ctx, &ctx.sigma.mods[index]) {
            emit(diags, diag);
        }
    }
    let name_maps = collect_name_maps(&mut ctx);
    for diag in name_maps.diags {
        emit(diags, diag);
    }
    if has_error(diags) {
        return SemaOutcome::default();
    }
    populate_sigma(&mut ctx);
    let module_names = module_names_of(&sema_project);
    let resolved = {
        let mut res_ctx = ResolveContext {
            ctx: &mut ctx,
            name_maps: &name_maps.name_maps,
            module_names: &module_names,
            can_access: Some(can_access),
            parse_ok: true,
            parse_diags: Some(&Vec::new()),
            language_service: None,
        };
        resolve_modules(&mut res_ctx)
    };
    for diag in resolved.diags {
        emit(diags, diag);
    }
    if !resolved.ok {
        return SemaOutcome::default();
    }
    std::sync::Arc::make_mut(&mut ctx.sigma).mods = resolved.modules;
    populate_sigma(&mut ctx);
    if has_error(diags) {
        return SemaOutcome::default();
    }
    // The imports between assemblies, and what they must satisfy.
    let graph = build_assembly_import_graph(&sema_project, &graph_modules);
    if !validate_assembly_import_graph_structure(&sema_project, &graph, diags)
        || !validate_hosted_library_import_graph(&sema_project, &graph, &graph_modules, diags)
    {
        return SemaOutcome::default();
    }
    let checked = typecheck_modules(&mut ctx, &name_maps.name_maps);
    let typecheck_ok = checked.ok;
    let checked_stores = checked.stores.clone();
    let incomplete = !checked.pending_items.is_empty() || checked.pending_tail.is_some();
    for diag in checked.diags {
        emit(diags, diag);
    }
    if incomplete {
        return pending("declarations the type check cannot type yet");
    }
    if !typecheck_ok {
        return SemaOutcome::default();
    }
    // What each caller passes to each callee, and whose authority it is.
    let cap_modules: Vec<&ASTModule> = ctx.sigma.mods.iter().collect();
    let mut call_graph = build_call_graph(&ctx, &cap_modules);
    propagate_capability_requirements(&mut call_graph);
    let stores = checked_stores.as_deref();
    let chain = validate_capability_chain(&ctx, &call_graph, stores);
    let mut capability_ok = chain.valid;
    for error in &chain.errors {
        emit_message_diagnostic(diags, if error.code.is_empty() { "E-CON-0020" } else { &error.code }, error.span.clone(), &error.message, "Capability chain validation failed.");
    }
    for leak in &chain.leaks {
        let span = Some(leak.leak_span.clone()).filter(|span| !span.file.is_empty());
        emit_message_diagnostic(diags, leak.code, span, &leak.message, "Capability leaked to extern procedure.");
    }
    let authority = validate_module_authority(&ctx, &cap_modules, stores);
    if !authority.valid {
        capability_ok = false;
        for error in &authority.errors {
            let span = Some(error.span.clone()).filter(|span| !span.file.is_empty());
            emit_message_diagnostic(diags, if error.error_code.is_empty() { "E-CON-0020" } else { &error.error_code }, span, &error.error_message, "Authority validation failed.");
        }
    }
    if !capability_ok {
        return SemaOutcome::default();
    }
    // Lowering every module; a declaration that is not ported yet leaves the check pending.
    let mut decls = Vec::new();
    let mut first_pending: Option<String> = None;
    let mut pending_decls = Vec::new();
    let mut base = ctx.clone();
    base.stores = checked_stores.clone();
    let mut lower_ctx = uv_codegen::lower::LowerCtx::new(&base, &name_maps.name_maps);
    for module in ctx.sigma.mods.iter() {
        let lowered = uv_codegen::lower::lower_module(module, &mut lower_ctx);
        decls.extend(lowered.decls);
        if first_pending.is_none() {
            first_pending = lowered.pending.first().map(|(_, what)| format!("the lowerability check ({what})"));
        }
        pending_decls.extend(lowered.pending);
    }
    SemaOutcome { pending: first_pending, ir: Some(decls), pending_decls }
}

/// A diagnostic of the registry with the message of the check that found it; the
/// registry has no entry for a code it does not know, and then an internal error is reported.
fn emit_message_diagnostic(diags: &mut DiagnosticStream, code: &str, span: Option<uv_core::span::Span>, message: &str, fallback: &str) {
    match make_diagnostic_by_id(code, span.clone()) {
        Some(mut diag) => {
            if !message.is_empty() {
                diag.message = message.to_string();
            }
            emit(diags, diag);
        }
        None => emit_internal_diagnostic(diags, span, if message.is_empty() { fallback } else { message }),
    }
}
