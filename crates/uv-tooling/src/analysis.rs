//! One analysis of the workspace for the language server: the project, every module
//! parsed (with the open documents in place of the files), the compile-time pass, name
//! resolution with the language service's facts, and type checking. See `AnalyzeWorkspace`.

use std::cell::RefCell;
use std::collections::HashMap;

use uv_analysis::context::{Scope, ScopeContext};
use uv_analysis::language_service::{build_language_service_declarations, LanguageServiceIndex};
use uv_analysis::resolve::collect_toplevel::collect_name_maps;
use uv_analysis::resolve::populate_sigma::populate_sigma;
use uv_analysis::resolve::resolve_module::resolve_modules;
use uv_analysis::resolve::resolver::ResolveContext;
use uv_analysis::resolve::scopes_lookup::module_names_of;
use uv_analysis::resolve::visibility::{can_access, check_module_visibility};
use uv_analysis::typing::types::TypeRef;
use uv_core::span::Span;
use uv_analysis::typing::typecheck::typecheck_modules;
use uv_comptime::{execute_comptime, ComptimePassOptions};
use uv_core::diagnostic_messages::emit_external_diagnostic;
use uv_core::diagnostics::{emit, has_error, Diagnostic, DiagnosticStream, Severity};
use uv_project::load_project::{load_project, parse_assembly_target};
use uv_project::project::Project;
use uv_project::target_profile::TargetProfile;
use uv_source::ast::ASTModule;
use uv_source::parse_modules::{parse_modules, read_bytes_default, ParseModuleDeps, ReadBytesResult, UnsafeSpanMap};

use super::document_store::DocumentOverlay;
use super::uri::{normalize_path, path_key};

#[derive(Default, Clone)]
pub struct ToolingAnalysisOptions {
    pub project_root: String,
    pub assembly_target: Option<String>,
    pub target_profile: Option<TargetProfile>,
    pub fallback_target_profile: Option<TargetProfile>,
    pub run_comptime: bool,
    pub semantic: bool,
}

#[derive(Default)]
pub struct AnalysisSnapshot {
    pub project_ok: bool,
    pub parse_ok: bool,
    pub comptime_ok: bool,
    pub resolve_ok: bool,
    pub typecheck_ok: bool,
    pub project: Option<Project>,
    pub modules: Vec<ASTModule>,
    pub diagnostics: DiagnosticStream,
    pub language_service: LanguageServiceIndex,
    /// The type of each expression, as span and type, for answering "type at".
    pub expr_types: Vec<(Span, TypeRef)>,
}

fn emit_internal(diags: &mut DiagnosticStream, message: &str) {
    emit(diags, Diagnostic { severity: Severity::Error, message: message.to_string(), ..Default::default() });
}

fn comptime_options(project: &Project) -> ComptimePassOptions {
    ComptimePassOptions {
        project_root: project.root.clone(),
        fallback_source_root: Some(project.source_root.clone()),
        source_roots_by_assembly: project.assemblies.iter().map(|assembly| (assembly.name.clone(), assembly.source_root.clone())).collect(),
    }
}

pub fn analyze_workspace(options: &ToolingAnalysisOptions, overlays: &[DocumentOverlay]) -> AnalysisSnapshot {
    let mut snapshot = AnalysisSnapshot::default();
    let Some(assembly_target) = parse_assembly_target(options.assembly_target.as_deref()) else {
        emit_internal(&mut snapshot.diagnostics, "Invalid Ultraviolet assembly target.");
        return snapshot;
    };
    let root = if options.project_root.is_empty() {
        std::env::current_dir().map(|cwd| cwd.to_string_lossy().into_owned()).unwrap_or_default()
    } else {
        normalize_path(std::path::Path::new(&options.project_root)).to_string_lossy().into_owned()
    };
    let loaded = load_project(&root, &assembly_target);
    for diag in loaded.diags.iter().cloned() {
        emit(&mut snapshot.diagnostics, diag);
    }
    let Some(project) = loaded.project.filter(|_| !has_error(&loaded.diags)) else {
        snapshot.project_ok = false;
        return snapshot;
    };
    snapshot.project_ok = true;
    let mut sema_project = project.clone();
    snapshot.project = Some(sema_project.clone());
    let selected = options.target_profile.or(sema_project.toolchain.target_profile).or(options.fallback_target_profile);
    let Some(target_profile) = selected else {
        emit_external_diagnostic(&mut snapshot.diagnostics, "E-PRJ-0112");
        return snapshot;
    };

    let overlay_text: HashMap<String, &str> = overlays.iter().map(|overlay| (path_key(&overlay.path), overlay.text_utf8.as_str())).collect();
    let read_bytes = |path: &str| -> ReadBytesResult {
        match overlay_text.get(&path_key(std::path::Path::new(path))) {
            Some(text) => ReadBytesResult { bytes: Some(text.as_bytes().to_vec()), ..Default::default() },
            None => read_bytes_default(path),
        }
    };
    let deps = ParseModuleDeps { read_bytes: &read_bytes, inspect_source: None };
    let mut parsed_modules: Vec<ASTModule> = Vec::new();
    let mut unsafe_spans: UnsafeSpanMap = HashMap::new();
    let mut parse_ok = true;
    for assembly in &sema_project.assemblies {
        let parsed = parse_modules(&assembly.modules, &assembly.source_root, &assembly.name, &deps);
        let failed = has_error(&parsed.diags) || parsed.modules.is_none();
        for diag in parsed.diags {
            emit(&mut snapshot.diagnostics, diag);
        }
        unsafe_spans.extend(parsed.unsafe_spans_by_file);
        if failed {
            parse_ok = false;
            break;
        }
        parsed_modules.extend(parsed.modules.unwrap_or_default());
    }
    snapshot.parse_ok = parse_ok;
    if !parse_ok {
        snapshot.language_service = build_language_service_declarations(&parsed_modules);
        snapshot.modules = parsed_modules;
        return snapshot;
    }

    let mut analysis_modules = parsed_modules;
    snapshot.comptime_ok = true;
    if options.run_comptime {
        let expanded = execute_comptime(&analysis_modules, &comptime_options(&sema_project));
        let failed = has_error(&expanded.diags) || expanded.modules.is_none();
        for diag in expanded.diags {
            emit(&mut snapshot.diagnostics, diag);
        }
        if failed {
            snapshot.comptime_ok = false;
            snapshot.language_service = build_language_service_declarations(&analysis_modules);
            snapshot.modules = analysis_modules;
            return snapshot;
        }
        analysis_modules = expanded.modules.unwrap_or_default();
    }
    if !options.semantic {
        snapshot.language_service = build_language_service_declarations(&analysis_modules);
        snapshot.modules = analysis_modules;
        return snapshot;
    }

    sema_project.modules = sema_project.assemblies.iter().flat_map(|assembly| assembly.modules.iter().cloned()).collect();
    let mut ctx = ScopeContext {
        project: Some(&sema_project),
        target_profile: Some(target_profile),
        scopes: vec![Scope::new(), Scope::new(), Scope::new()],
        ..Default::default()
    };
    std::sync::Arc::make_mut(&mut ctx.sigma).mods = analysis_modules;
    std::sync::Arc::make_mut(&mut ctx.sigma).unsafe_spans_by_file = unsafe_spans;
    let language_service = RefCell::new(build_language_service_declarations(&ctx.sigma.mods));
    for index in 0..ctx.sigma.mods.len() {
        ctx.current_module = ctx.sigma.mods[index].path.clone();
        for diag in check_module_visibility(&ctx, &ctx.sigma.mods[index]) {
            emit(&mut snapshot.diagnostics, diag);
        }
    }
    let name_maps = collect_name_maps(&mut ctx);
    for diag in name_maps.diags {
        emit(&mut snapshot.diagnostics, diag);
    }
    if has_error(&snapshot.diagnostics) {
        snapshot.modules = ctx.sigma.mods.clone();
        snapshot.language_service = language_service.into_inner();
        return snapshot;
    }
    populate_sigma(&mut ctx);
    let module_names = module_names_of(&sema_project);
    let no_diags = snapshot.diagnostics.clone();
    let resolved = {
        let mut res_ctx = ResolveContext {
            ctx: &mut ctx,
            name_maps: &name_maps.name_maps,
            module_names: &module_names,
            can_access: Some(can_access),
            parse_ok: snapshot.parse_ok,
            parse_diags: Some(&no_diags),
            language_service: Some(&language_service),
        };
        resolve_modules(&mut res_ctx)
    };
    snapshot.resolve_ok = resolved.ok;
    for diag in resolved.diags {
        emit(&mut snapshot.diagnostics, diag);
    }
    if resolved.ok {
        std::sync::Arc::make_mut(&mut ctx.sigma).mods = resolved.modules;
        populate_sigma(&mut ctx);
    }
    if !has_error(&snapshot.diagnostics) && snapshot.resolve_ok {
        let checked = typecheck_modules(&mut ctx, &name_maps.name_maps);
        snapshot.typecheck_ok = checked.ok;
        for diag in checked.diags {
            emit(&mut snapshot.diagnostics, diag);
        }
        if let Some(stores) = &checked.stores {
            snapshot.expr_types = stores.expr_types.borrow().values().map(|(expr, ty)| (expr.span.clone(), ty.clone())).collect();
        }
    }
    snapshot.modules = ctx.sigma.mods.clone();
    snapshot.language_service = language_service.into_inner();
    snapshot.project = Some(sema_project.clone());
    snapshot
}

/// The type of the smallest expression that covers the point.
pub fn type_at(snapshot: &AnalysisSnapshot, path: &str, offset: usize) -> TypeRef {
    let key = super::uri::path_key(std::path::Path::new(path));
    let mut best: Option<(usize, &TypeRef)> = None;
    for (span, ty) in &snapshot.expr_types {
        if ty.is_none() || super::uri::path_key(std::path::Path::new(&*span.file)) != key || span.start_offset > offset || offset > span.end_offset {
            continue;
        }
        let width = span.end_offset - span.start_offset;
        if best.is_none_or(|(best_width, _)| width < best_width) {
            best = Some((width, ty));
        }
    }
    best.and_then(|(_, ty)| ty.clone())
}
