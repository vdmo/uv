//! Emits the same dumps as the reference oracle (`tools/oracle/oracle_main.cpp`) so the
//! two implementations can be compared byte for byte.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write as _;

use std::cell::RefCell;
use std::rc::Rc;
use uv_analysis::caps::context_caps::is_capability_class;
use uv_analysis::composite::class_linearization::linearize_class;
use uv_analysis::composite::classes::{
    check_orphan_rule, class_abstract_states, class_associated_types,
    class_dispatchability_diagnostic, class_dispatchable, class_field_table, class_method_table,
    class_subtypes, dispatchable, is_modal_class, lookup_class_method, missing_impl_methods,
    type_implements_class, vtable_eligible,
};
use uv_analysis::composite::enums::enum_discriminants;
use uv_analysis::composite::record_methods::recv_mode_of;
use uv_analysis::context::{Entity, NameMapTable, Scope, ScopeContext, TypeDecl};
use uv_analysis::contracts::verification::{
    add_predicate_facts, ent_fact, ent_linear, ent_true, evaluate_constant, get_type_bounds,
    negated_predicate, static_proof, ConstValue, StaticProofContext, StaticProofResult,
};
use uv_analysis::generics::generic_params::{
    bind_type_params, build_param_scope, has_default_params, is_valid_const_param_type,
    parse_const_param, required_param_count, total_param_count, validate_generic_params,
};
use uv_analysis::generics::monomorphize::{build_substitution, instantiate_type};
use uv_analysis::generics::monomorphize::{
    check_bounds_satisfied, infer_type_arguments, InstantiationKey, MonomorphizeContext,
};
use uv_analysis::layout::value_bits::{
    decode_string_literal_bytes, encode_const, valid_value, value_bits, EnumPayloadVal, RawPtrVal,
    Value, ValueRangeKind,
};
use uv_analysis::layout::{
    align_of, dyn_layout_of, enum_layout_of, enum_record_payload_member_layout,
    enum_tuple_payload_member_layout, layout_of, lower_async_type_of, lower_type_for_layout,
    modal_layout_of, range_layout_of, record_layout_of, resolve_enum_layout_options,
    resolve_record_layout_options, size_of, tuple_layout_of, union_layout_of,
    EnumPayloadMemberLayout, Layout, RecordLayout,
};
use uv_analysis::memory::regions::ProvenanceKind;
use uv_analysis::modal::lookup::{
    has_state, lookup_modal_field_decl, lookup_state_method_decl, lookup_transition_decl,
};
use uv_analysis::modal::modal_widen::payload_state;
use uv_analysis::modal::modal_widen::{niche_compatible, widen_warn_cond};
use uv_analysis::resolve::collect_toplevel::collect_name_maps;
use uv_analysis::resolve::populate_sigma::populate_sigma;
use uv_analysis::resolve::resolve_module::resolve_modules;
use uv_analysis::resolve::resolver::ResolveContext;
use uv_analysis::resolve::scopes::id_key_of;
use uv_analysis::resolve::scopes::{path_key_of, universe_bindings};
use uv_analysis::resolve::scopes_lookup::module_names_of;
use uv_analysis::resolve::visibility::{can_access, check_module_visibility};
use uv_analysis::typing::item_generic_params::{check_generic_args, process_generic_params};
use uv_analysis::typing::literals::{check_literal_expr, null_literal_expected, type_literal_expr};
use uv_analysis::typing::pattern::{
    enum_pattern_covers_variant, irrefutable_pattern, modal_pattern_covers_state,
    type_pattern_against_type,
};
use uv_analysis::typing::pending::{reset_scaffolding, take_pending};
use uv_analysis::typing::signature::{
    build_method_signature, build_transition_signature, Signature,
};
use uv_analysis::typing::solve::{apply_substitution, solve, Constraint};
use uv_analysis::typing::stmt::block::type_block;
use uv_analysis::typing::stmt_context::StmtTypeContext;
use uv_analysis::typing::subtyping::{argument_type_compatible, subtyping, SubtypingResult};
use uv_analysis::typing::type_env::{
    apply_binding_provenance_seed, bind_of, collect_pat_names, distinct_names, gpu_context,
    has_heap_provenance, intro_all, mark_shared_derived_bindings_stale, mut_of,
    normalize_binding_provenance_seed, pop_scope, project_type_env_to_depth, push_scope,
    stable_binding_type, type_pattern, BindingProvenanceSeedKind, ParallelContextKind, TypeBinding,
    TypeEnv,
};
use uv_analysis::typing::type_equiv::type_equiv;
use uv_analysis::typing::type_expr::{type_expr, type_identifier_expr, type_place};
use uv_analysis::typing::type_lookup::lookup_enum_decl;
use uv_analysis::typing::type_lookup::{
    async_sig_of, field_type, field_visible, lookup_type_decl_resolved, record_fields,
};
use uv_analysis::typing::type_lower::lower_type;
use uv_analysis::typing::type_predicates::{
    bitcopy_type, builtin_discrete_type, cast_valid, clone_type, drop_type, eq_type, eq_type_in,
    ffi_safe_diag_for_type, ffi_safe_type, gpu_safe_diag_for_type, is_capability_type,
    lookup_foundational_builtin_method_sig, ord_type, perm_of_type, strip_perm, zeroable_type,
};
use uv_analysis::typing::type_wf::type_wf;
use uv_analysis::typing::types::{
    is_range_index_type, is_range_type, make_type_prim, make_type_string, make_type_tuple,
    type_key_of, type_paths, type_to_string, KeyAtom, StringState, TypeKey, TypeNode, TypeRef,
};
use uv_analysis::typing::types::{
    make_type, make_type_array, make_type_closure, make_type_func, make_type_slice,
    make_type_union, TypeFuncParam,
};
use uv_analysis::typing::types::{make_type_modal_state, make_type_path, Permission};
use uv_analysis::typing::types::{make_type_perm, make_type_ptr};
use uv_analysis::typing::types::{make_type_raw_ptr, PtrState, RawPtrQual};
use uv_analysis::typing::variance::{compute_variance_context, Variance};
use uv_comptime::{execute_comptime, ComptimePassOptions};
use uv_core::diagnostics::Diagnostic;
use uv_core::diagnostics::{emit, has_error};
use uv_core::source_load::load_source;
use uv_core::span::Span;
use uv_core::symbols::string_of_path;
use uv_core::unicode::{
    analyze_identifier_security, case_fold, is_xid_continue, is_xid_start, nfc,
};
use uv_project::load_project::load_project;
use uv_project::manifest::find_project_root;
use uv_project::module_discovery::compilation_unit;
use uv_project::project::{Assembly, AssemblyTarget};
use uv_project::target_profile::TargetProfile;
use uv_source::ast::dump::AstDump;
use uv_analysis::typing::typecheck::typecheck_modules;
use uv_source::ast::item_span;
use uv_analysis::typing::dynamic_context::{compute_dynamic_context, DynamicScopeAncestor};
use uv_source::attributes::{attrs, has_attribute, resolve_verification_mode_attribute, VerificationModeAttribute};
use uv_source::ast::AttributeItem;
use uv_source::ast::walk::ExprWalk;
use uv_analysis::typing::expr_store::TypeStores;
use uv_analysis::contracts::verification::extend_proof_context_with_predicate_at;
use uv_analysis::generics::monomorphize::TypeSubst;
use uv_analysis::resolve::scopes::id_eq;
use uv_analysis::typing::signature::subst_self_type;
use uv_analysis::typing::types::self_var_type;
use uv_source::ast::{Block, ClassDecl, ModalDecl, ReceiverPerm, RecordDecl, TypeInvariant};
use uv_source::ast::ASTModule;
use uv_source::ast::ContractClause;
use uv_source::ast::LiteralExpr;
use uv_source::ast::Mutability;
use uv_analysis::contracts::purity::is_pure_in_scope;
use uv_source::ast::{
    ASTItem, ClassItem, ExternItem, GenericParams, Param, Receiver, RecordMember, StateMember,
    TypeParam, TypePtr, VariantPayload,
};
use uv_source::ast::{BlockPtr, ExprNode, ExprPtr, PatternNode, PatternPtr, Stmt};
use uv_source::lexer::tokenize_with_diagnostics;
use uv_source::parser::parse_file;
use uv_source::phase1::{run_phase1, AssemblyOutcome, Phase1Observer};

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out
}

fn span_text(span: &Span) -> String {
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        span.start_offset,
        span.end_offset,
        span.start_line,
        span.start_col,
        span.end_line,
        span.end_col
    )
}

fn print_diags(out: &mut String, diags: &[Diagnostic]) {
    for diag in diags {
        let span = diag
            .span
            .as_ref()
            .map_or_else(|| "-".to_string(), span_text);
        let _ = writeln!(
            out,
            "G\t{}\t{}\t{}\t{}\t{}",
            diag.code,
            diag.severity.label(),
            escape(&diag.message),
            span,
            diag.obligation_ids.join(",")
        );
    }
}

fn dump_tokens(out: &mut String, label: &str, path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|err| format!("cannot read {path}: {err}"))?;
    let _ = writeln!(out, "F\t{label}");
    let loaded = load_source(label, &bytes);
    print_diags(out, &loaded.diags);
    let Some(source) = loaded.source else {
        out.push_str("NOSOURCE\n");
        return Ok(());
    };
    let result = tokenize_with_diagnostics(&source);
    print_diags(out, &result.diags);
    let Some(output) = result.output else {
        out.push_str("NOTOKENS\n");
        return Ok(());
    };
    for token in &output.tokens {
        let _ = writeln!(
            out,
            "T\t{}\t{}\t{}",
            token.kind.name(),
            escape(&token.lexeme),
            span_text(&token.span)
        );
    }
    for doc in &output.docs {
        let _ = writeln!(
            out,
            "D\t{}\t{}\t{}",
            doc.kind.name(),
            escape(&doc.text),
            span_text(&doc.span)
        );
    }
    Ok(())
}

fn dump_ast(out: &mut String, label: &str, path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|err| format!("cannot read {path}: {err}"))?;
    let _ = writeln!(out, "F\t{label}");
    let loaded = load_source(label, &bytes);
    print_diags(out, &loaded.diags);
    let Some(source) = loaded.source else {
        out.push_str("NOSOURCE\n");
        return Ok(());
    };
    let parsed = parse_file(&source);
    print_diags(out, &parsed.diags);
    for span in &parsed.unsafe_spans {
        let _ = writeln!(out, "U\t{}", span_text(span));
    }
    let Some(file) = parsed.file else {
        out.push_str("NOFILE\n");
        return Ok(());
    };
    for doc in &file.module_doc {
        out.push_str("M\t");
        doc.dump(out);
        out.push('\n');
    }
    for item in &file.items {
        out.push_str("I\t");
        item.dump(out);
        out.push('\n');
    }
    Ok(())
}

fn opt_text(text: &Option<String>) -> String {
    text.as_deref().map_or_else(|| "-".to_string(), escape)
}

fn file_span_text(span: &Option<Span>) -> String {
    span.as_ref().map_or_else(
        || "-".to_string(),
        |span| format!("{}\t{}", span.file, span_text(span)),
    )
}

/// Diagnostics with the span's file and the attached notes; see `PrintDiagsFull` in the oracle.
fn print_diags_full(out: &mut String, diags: &[Diagnostic]) {
    for diag in diags {
        let _ = writeln!(
            out,
            "G\t{}\t{}\t{}\t{}\t{}\t{}",
            diag.code,
            diag.severity.label(),
            escape(&diag.message),
            file_span_text(&diag.span),
            diag.obligation_ids.join(","),
            opt_text(&diag.label)
        );
        for child in &diag.children {
            let _ = writeln!(
                out,
                "N\t{}\t{}\t{}\t{}\t{}",
                child.kind as i32,
                escape(&child.message),
                file_span_text(&child.span),
                opt_text(&child.fix_text),
                opt_text(&child.label)
            );
        }
    }
}

/// One project block of a project list; see `LoadProjectBlock` in the oracle.
#[derive(Default)]
struct ProjectBlock {
    label: String,
    options: ComptimePassOptions,
    modules: Vec<ASTModule>,
    reachable: usize,
    unsafe_spans_by_file: HashMap<String, Vec<Span>>,
}

/// Loads a block, or returns the lines to print when one of its files cannot be used.
fn load_project_block(block: &[&str]) -> Result<ProjectBlock, String> {
    let mut out = ProjectBlock::default();
    for line in block {
        let fields: Vec<&str> = line.split('\t').collect();
        match fields.as_slice() {
            ["P", name, root, fallback, ..] => {
                out.label = name.to_string();
                out.options.project_root = root.to_string();
                out.options.fallback_source_root = Some(fallback.to_string());
            }
            ["A", name, source_root, ..] => {
                out.options
                    .source_roots_by_assembly
                    .insert(name.to_string(), source_root.to_string());
            }
            ["R", count, ..] => out.reachable = count.parse().unwrap_or(0),
            ["M", path, files @ ..] => {
                let mut module = ASTModule {
                    path: path.split("::").map(str::to_string).collect(),
                    ..ASTModule::default()
                };
                for file in files {
                    let bytes = std::fs::read(file).unwrap_or_default();
                    let Some(source) = load_source(file, &bytes).source else {
                        return Err(format!("F\t{}\nNOSOURCE\t{file}\n", out.label));
                    };
                    let parsed = parse_file(&source);
                    let Some(parsed_file) = parsed.file else {
                        return Err(format!("F\t{}\nNOFILE\t{file}\n", out.label));
                    };
                    module.items.extend(parsed_file.items);
                    module.module_doc.extend(parsed_file.module_doc);
                    out.unsafe_spans_by_file
                        .insert(source.path.to_string(), parsed.unsafe_spans);
                }
                out.modules.push(module);
            }
            _ => {}
        }
    }
    Ok(out)
}

fn dump_line(tag: &str, value: &dyn AstDump, out: &mut String) {
    out.push_str(tag);
    out.push('\t');
    value.dump(out);
    out.push('\n');
}

fn dump_modules(out: &mut String, modules: &[ASTModule]) {
    for module in modules {
        dump_line("X", &module.path, out);
        for doc in &module.module_doc {
            dump_line("M", doc, out);
        }
        for item in &module.items {
            dump_line("I", item, out);
        }
        for proc in &module.comptime_procedures {
            dump_line("C", proc, out);
        }
    }
}

/// Runs the compile-time pass on one project block of a comptime list.
fn dump_comptime(out: &mut String, block: &[&str]) {
    let ProjectBlock {
        label,
        options,
        modules,
        ..
    } = match load_project_block(block) {
        Ok(project) => project,
        Err(failure) => {
            out.push_str(&failure);
            return;
        }
    };
    let _ = writeln!(out, "F\t{label}");
    let result = execute_comptime(&modules, &options);
    print_diags_full(out, &result.diags);
    let Some(expanded) = result.modules else {
        out.push_str("NOMODULES\n");
        return;
    };
    dump_modules(out, &expanded);
}

fn dump_name_maps(out: &mut String, table: &NameMapTable) {
    for (module_key, name_map) in table {
        dump_line("NM", module_key, out);
        let mut entries: Vec<(&String, &Entity)> = name_map.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        for (name, ent) in entries {
            out.push_str("NE\t");
            name.dump(out);
            let _ = write!(out, "\t{:?}\t{:?}\t", ent.kind, ent.source);
            ent.origin_opt.dump(out);
            out.push('\t');
            ent.target_opt.dump(out);
            out.push('\t');
            ent.declaration_span.dump(out);
            out.push('\t');
            ent.language_symbol_id.dump(out);
            out.push('\t');
            ent.type_param_class_bounds.dump(out);
            out.push('\t');
            ent.visibility.dump(out);
            out.push('\n');
        }
    }
}

/// Runs the front end through name resolution on one project block, in the order the
/// driver does; see `DumpResolve` in the oracle.
fn dump_resolve(out: &mut String, block: &[&str], mode: &str) {
    let types_mode = mode != "resolve";
    let input = match load_project_block(block) {
        Ok(project) => project,
        Err(failure) => {
            out.push_str(&failure);
            return;
        }
    };
    let _ = writeln!(out, "F\t{}", input.label);
    let Some(project) =
        load_project(&input.options.project_root, &AssemblyTarget::default()).project
    else {
        out.push_str("NOPROJECT\n");
        return;
    };
    let comptime = execute_comptime(&input.modules, &input.options);
    let Some(mut parsed_modules) = comptime.modules.filter(|_| !has_error(&comptime.diags)) else {
        out.push_str("STOP\tcomptime\n");
        return;
    };
    let mut diags = Vec::new();
    for diag in comptime.diags {
        emit(&mut diags, diag);
    }
    parsed_modules.truncate(input.reachable);

    let mut sema_project = project.clone();
    sema_project.modules = parsed_modules
        .iter()
        .flat_map(|module| {
            let path = string_of_path(&module.path);
            project.assemblies.iter().flat_map(move |assembly| {
                let path = path.clone();
                assembly
                    .modules
                    .iter()
                    .filter(move |info| info.path == path)
                    .cloned()
            })
        })
        .collect();

    let mut ctx = ScopeContext {
        project: Some(&sema_project),
        target_profile: Some(TargetProfile::X86_64SysV),
        scopes: vec![Scope::new(), Scope::new(), Scope::new()],
        ..Default::default()
    };
    std::sync::Arc::make_mut(&mut ctx.sigma).mods = parsed_modules;
    std::sync::Arc::make_mut(&mut ctx.sigma).unsafe_spans_by_file = input.unsafe_spans_by_file;
    for index in 0..ctx.sigma.mods.len() {
        ctx.current_module = ctx.sigma.mods[index].path.clone();
        for diag in check_module_visibility(&ctx, &ctx.sigma.mods[index]) {
            emit(&mut diags, diag);
        }
    }
    let name_maps = collect_name_maps(&mut ctx);
    for diag in name_maps.diags {
        emit(&mut diags, diag);
    }
    if !types_mode {
        print_diags_full(out, &diags);
        dump_name_maps(out, &name_maps.name_maps);
    }
    if has_error(&diags) {
        out.push_str("STOP\tnames\n");
        return;
    }
    populate_sigma(&mut ctx);
    let module_names = module_names_of(&sema_project);
    let no_parse_diags = Vec::new();
    let mut res_ctx = ResolveContext {
        ctx: &mut ctx,
        name_maps: &name_maps.name_maps,
        module_names: &module_names,
        can_access: Some(can_access),
        parse_ok: true,
        parse_diags: Some(&no_parse_diags),
        language_service: None,
    };
    let resolved = resolve_modules(&mut res_ctx);
    if types_mode {
        if !resolved.ok {
            out.push_str("STOP\tresolve\n");
            return;
        }
        std::sync::Arc::make_mut(&mut ctx.sigma).mods = resolved.modules;
        populate_sigma(&mut ctx);
        if mode == "typecheck" {
            dump_typecheck(out, &mut ctx, &name_maps.name_maps);
        } else if mode == "bodies" {
            dump_bodies(out, &mut ctx, &name_maps.name_maps);
        } else if mode == "patterns" {
            dump_patterns(out, &mut ctx, &name_maps.name_maps);
        } else if mode == "values" {
            dump_values(out, &mut ctx, &name_maps.name_maps);
        } else if mode == "relations" {
            dump_relations(out, &mut ctx, &name_maps.name_maps);
        } else {
            dump_types(out, &mut ctx, &name_maps.name_maps);
        }
        return;
    }
    let _ = writeln!(
        out,
        "RESOLVE\t{}",
        if resolved.ok { "ok" } else { "failed" }
    );
    print_diags_full(out, &resolved.diags);
    dump_modules(out, &resolved.modules);
}

// ---- type core; see `DumpTypes` in the oracle ----

fn print_key(out: &mut String, key: &TypeKey) {
    out.push('(');
    for (index, atom) in key.atoms.iter().enumerate() {
        if index != 0 {
            out.push(' ');
        }
        match atom {
            KeyAtom::Number(number) => {
                let _ = write!(out, "n{number}");
            }
            KeyAtom::String(text) => {
                let _ = write!(out, "s\"{}\"", escape(text));
            }
            KeyAtom::Key(key) => print_key(out, key),
            KeyAtom::KeyList(keys) => {
                out.push('[');
                for (index, key) in keys.iter().enumerate() {
                    if index != 0 {
                        out.push(' ');
                    }
                    print_key(out, key);
                }
                out.push(']');
            }
        }
    }
    out.push(')');
}

type Written = Vec<(String, TypePtr)>;

fn collect_params(owner: &str, params: &[Param], ret: &TypePtr, out: &mut Written) {
    for param in params {
        out.push((format!("{owner}({})", param.name), param.r#type.clone()));
    }
    if ret.is_some() {
        out.push((format!("{owner}->"), ret.clone()));
    }
}

fn collect_generics(owner: &str, params: &Option<GenericParams>, out: &mut Written) {
    for param in params.iter().flat_map(|params| &params.params) {
        if param.default_type.is_some() {
            out.push((
                format!("{owner}<{}=>", param.name),
                param.default_type.clone(),
            ));
        }
    }
}

/// Every type written in a declaration of the module, in source order.
fn collect_written_types(module: &ASTModule) -> Written {
    let mut out = Written::new();
    for item in &module.items {
        match item {
            ASTItem::StaticDecl(node) => {
                if node.binding.type_opt.is_some() {
                    out.push(("static".to_string(), node.binding.type_opt.clone()));
                }
            }
            ASTItem::ProcedureDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                collect_params(&node.name, &node.params, &node.return_type_opt, &mut out);
            }
            ASTItem::ComptimeProcedureDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                collect_params(&node.name, &node.params, &node.return_type_opt, &mut out);
            }
            ASTItem::ExternBlock(node) => {
                for ExternItem::ExternProcDecl(proc) in &node.items {
                    collect_params(&proc.name, &proc.params, &proc.return_type_opt, &mut out);
                }
            }
            ASTItem::RecordDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                for member in &node.members {
                    match member {
                        RecordMember::FieldDecl(field) => {
                            out.push((
                                format!("{}.{}", node.name, field.name),
                                field.r#type.clone(),
                            ));
                        }
                        RecordMember::MethodDecl(method) => {
                            let owner = format!("{}::{}", node.name, method.name);
                            collect_params(
                                &owner,
                                &method.params,
                                &method.return_type_opt,
                                &mut out,
                            );
                            if let Receiver::ReceiverExplicit(recv) = &method.receiver {
                                out.push((format!("{owner}(self)"), recv.r#type.clone()));
                            }
                        }
                        RecordMember::AssociatedTypeDecl(assoc) => {
                            if assoc.default_type.is_some() {
                                out.push((
                                    format!("{}::{}", node.name, assoc.name),
                                    assoc.default_type.clone(),
                                ));
                            }
                        }
                    }
                }
            }
            ASTItem::EnumDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                for variant in &node.variants {
                    match &variant.payload_opt {
                        None => {}
                        Some(VariantPayload::VariantPayloadTuple(tuple)) => {
                            for element in &tuple.elements {
                                out.push((
                                    format!("{}::{}", node.name, variant.name),
                                    element.clone(),
                                ));
                            }
                        }
                        Some(VariantPayload::VariantPayloadRecord(record)) => {
                            for field in &record.fields {
                                let label =
                                    format!("{}::{}.{}", node.name, variant.name, field.name);
                                out.push((label, field.r#type.clone()));
                            }
                        }
                    }
                }
            }
            ASTItem::ModalDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                for state in &node.states {
                    let owner = format!("{}@{}", node.name, state.name);
                    for member in &state.members {
                        match member {
                            StateMember::StateFieldDecl(field) => {
                                out.push((format!("{owner}.{}", field.name), field.r#type.clone()));
                            }
                            StateMember::StateMethodDecl(method) => {
                                let owner = format!("{owner}::{}", method.name);
                                collect_params(
                                    &owner,
                                    &method.params,
                                    &method.return_type_opt,
                                    &mut out,
                                );
                            }
                            StateMember::TransitionDecl(trans) => {
                                collect_params(
                                    &format!("{owner}::{}", trans.name),
                                    &trans.params,
                                    &None,
                                    &mut out,
                                );
                            }
                        }
                    }
                }
            }
            ASTItem::ClassDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                for class_item in &node.items {
                    match class_item {
                        ClassItem::ClassFieldDecl(field) => {
                            out.push((
                                format!("{}.{}", node.name, field.name),
                                field.r#type.clone(),
                            ));
                        }
                        ClassItem::ClassMethodDecl(method) => {
                            let owner = format!("{}::{}", node.name, method.name);
                            collect_params(
                                &owner,
                                &method.params,
                                &method.return_type_opt,
                                &mut out,
                            );
                        }
                        ClassItem::AssociatedTypeDecl(assoc) => {
                            if assoc.default_type.is_some() {
                                out.push((
                                    format!("{}::{}", node.name, assoc.name),
                                    assoc.default_type.clone(),
                                ));
                            }
                        }
                        ClassItem::AbstractFieldDecl(field) => {
                            out.push((
                                format!("{}.{}", node.name, field.name),
                                field.r#type.clone(),
                            ));
                        }
                        ClassItem::AbstractStateDecl(state) => {
                            for field in &state.fields {
                                let label = format!("{}@{}.{}", node.name, state.name, field.name);
                                out.push((label, field.r#type.clone()));
                            }
                        }
                    }
                }
            }
            ASTItem::TypeAliasDecl(node) => {
                collect_generics(&node.name, &node.generic_params, &mut out);
                out.push((node.name.clone(), node.r#type.clone()));
            }
            ASTItem::UsingDecl(_)
            | ASTItem::ImportDecl(_)
            | ASTItem::DeriveTargetDecl(_)
            | ASTItem::ErrorItem(_) => {}
        }
    }
    out
}

fn opt_u64(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_string(), |value| value.to_string())
}

fn layout_text(layout: Option<Layout>) -> String {
    layout.map_or_else(
        || "-".to_string(),
        |layout| format!("{}/{}", layout.size, layout.align),
    )
}

fn offsets_text(offsets: &[u64]) -> String {
    offsets
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// Layout details of a lowered type that is a union, a range, a tuple or asynchronous.
fn dump_shape_layouts(out: &mut String, ctx: &ScopeContext<'_>, lowered: &TypeRef) {
    let Some(ty) = lowered.as_deref() else {
        return;
    };
    if let TypeNode::Union(members) = &ty.node {
        out.push_str("UL\t");
        match union_layout_of(ctx, members) {
            Some(layout) => {
                let _ = write!(
                    out,
                    "{}\t{}\t{}\t{}\t{}/{}",
                    layout_text(Some(layout.layout)),
                    if layout.niche { "niche" } else { "tagged" },
                    layout_text(layout.niche_payload_layout),
                    layout.disc_type.as_deref().unwrap_or("-"),
                    layout.payload_size,
                    layout.payload_align
                );
                for member in &layout.member_list {
                    let _ = write!(out, "\t{}", escape(&type_to_string(member)));
                }
            }
            None => out.push('-'),
        }
        out.push('\n');
    }
    let record_line = |out: &mut String, tag: &str, layout: Option<RecordLayout>| match layout {
        Some(layout) => {
            let _ = writeln!(
                out,
                "{tag}\t{}\t{}",
                layout_text(Some(layout.layout)),
                offsets_text(&layout.offsets)
            );
        }
        None => {
            let _ = writeln!(out, "{tag}\t-");
        }
    };
    if is_range_type(lowered) {
        record_line(out, "GL", range_layout_of(ctx, lowered));
    }
    if let TypeNode::Tuple(elements) = &ty.node {
        record_line(out, "TL", tuple_layout_of(ctx, elements));
    }
    if let Some(lowered_async) = lower_async_type_of(lowered) {
        out.push_str("AL");
        for state in &lowered_async.states {
            let _ = write!(out, "\t{state}");
        }
        let _ = writeln!(
            out,
            "\t{}\t{}",
            escape(&type_to_string(&lowered_async.resume_type)),
            layout_text(layout_of(ctx, &lowered_async.resume_type))
        );
    }
}

/// See `DumpDeclLayouts` in the oracle.
fn dump_decl_layouts(out: &mut String, ctx: &ScopeContext<'_>, item: &ASTItem) {
    let pool = [
        make_type_prim("i32"),
        make_type_prim("bool"),
        make_type_string(Some(StringState::View)),
        make_type_tuple(vec![make_type_prim("u8"), make_type_prim("u8")]),
    ];
    let arg_lists = |params: &Option<GenericParams>| {
        let mut lists: Vec<Vec<TypeRef>> = vec![Vec::new()];
        if let Some(params) = params.as_ref().filter(|params| !params.params.is_empty()) {
            let mut all: Vec<TypeRef> = (0..params.params.len())
                .map(|index| pool[index % pool.len()].clone())
                .collect();
            lists.push(all.clone());
            all.push(pool[0].clone());
            lists.push(all);
        }
        lists
    };
    match item {
        ASTItem::RecordDecl(node) => {
            let options = resolve_record_layout_options(&node.attrs);
            let _ = writeln!(
                out,
                "RO\t{}\t{}\t{}",
                node.name,
                if options.packed { "packed" } else { "-" },
                opt_u64(options.min_align)
            );
            let fields: Option<Vec<TypeRef>> = record_fields(node)
                .iter()
                .map(|field| lower_type_for_layout(ctx, &field.r#type))
                .collect();
            match fields.and_then(|fields| record_layout_of(ctx, &fields, &options)) {
                Some(layout) => {
                    let offsets = offsets_text(&layout.offsets);
                    let _ = writeln!(
                        out,
                        "RL\t{}\t{}\t{offsets}",
                        node.name,
                        layout_text(Some(layout.layout))
                    );
                }
                None => {
                    let _ = writeln!(out, "RL\t{}\t-\t-", node.name);
                }
            }
        }
        ASTItem::EnumDecl(node) => {
            let options = resolve_enum_layout_options(&node.attrs);
            let _ = writeln!(
                out,
                "EO\t{}\t{}\t{}",
                node.name,
                options.disc_type.as_deref().unwrap_or("-"),
                opt_u64(options.min_align)
            );
            match enum_discriminants(node) {
                Ok(discs) => {
                    let _ = writeln!(
                        out,
                        "ED\t{}\t{}\t{}",
                        node.name,
                        discs.max_disc,
                        offsets_text(&discs.discs)
                    );
                }
                Err(err) => {
                    let _ = writeln!(
                        out,
                        "ED\t{}\tfail\t{}\t{}",
                        node.name,
                        err.diag_id,
                        span_text(&err.span)
                    );
                }
            }
            for args in arg_lists(&node.generic_params) {
                match enum_layout_of(ctx, node, &args, &options) {
                    Some(layout) => {
                        let _ = writeln!(
                            out,
                            "EL\t{}\t{}\t{}\t{}\t{}/{}",
                            node.name,
                            args.len(),
                            layout_text(Some(layout.layout)),
                            layout.disc_type,
                            layout.payload_size,
                            layout.payload_align
                        );
                    }
                    None => {
                        let _ = writeln!(out, "EL\t{}\t{}\t-\t-\t-", node.name, args.len());
                    }
                }
                for variant in &node.variants {
                    let Some(payload) = &variant.payload_opt else {
                        continue;
                    };
                    let mut member_line =
                        |label: String, member: Option<EnumPayloadMemberLayout>| {
                            let _ = write!(
                                out,
                                "EM\t{}::{}{label}\t{}\t",
                                node.name,
                                variant.name,
                                args.len()
                            );
                            match member {
                                Some(member) => {
                                    let _ = writeln!(
                                        out,
                                        "{}\t{}\t{}/{}",
                                        escape(&type_to_string(&member.r#type)),
                                        member.offset,
                                        member.payload_size,
                                        member.payload_align
                                    );
                                }
                                None => out.push_str("-\t-\t-\n"),
                            }
                        };
                    match payload {
                        VariantPayload::VariantPayloadTuple(tuple) => {
                            for index in 0..=tuple.elements.len() {
                                let member = enum_tuple_payload_member_layout(
                                    ctx, node, variant, &args, index,
                                );
                                member_line(format!(".{index}"), member);
                            }
                            member_line(
                                ".named".to_string(),
                                enum_record_payload_member_layout(ctx, node, variant, &args, "x"),
                            );
                        }
                        VariantPayload::VariantPayloadRecord(record) => {
                            for field in &record.fields {
                                let member = enum_record_payload_member_layout(
                                    ctx,
                                    node,
                                    variant,
                                    &args,
                                    &field.name,
                                );
                                member_line(format!(".{}", field.name), member);
                            }
                            let missing = enum_record_payload_member_layout(
                                ctx,
                                node,
                                variant,
                                &args,
                                "no_such_field",
                            );
                            member_line(".missing".to_string(), missing);
                            member_line(
                                ".0".to_string(),
                                enum_tuple_payload_member_layout(ctx, node, variant, &args, 0),
                            );
                        }
                    }
                }
            }
        }
        ASTItem::ModalDecl(node) => {
            let payload = payload_state(ctx, node).unwrap_or("-");
            for args in arg_lists(&node.generic_params) {
                let _ = write!(out, "ML\t{}\t{}\t{payload}\t", node.name, args.len());
                match modal_layout_of(ctx, node, &args) {
                    Some(modal) => {
                        let _ = writeln!(
                            out,
                            "{}\t{}\t{}\t{}\t{}/{}",
                            layout_text(Some(modal.layout)),
                            if modal.niche { "niche" } else { "tagged" },
                            layout_text(modal.niche_payload_layout),
                            modal.disc_type.as_deref().unwrap_or("-"),
                            modal.payload_size,
                            modal.payload_align
                        );
                    }
                    None => out.push_str("-\t-\t-\t-\t-\n"),
                }
            }
        }
        _ => {}
    }
}

fn dump_instantiations(out: &mut String, name: &str, params: &[TypeParam], members: &[TypeRef]) {
    let pool = [
        make_type_prim("i32"),
        make_type_prim("bool"),
        make_type_string(Some(StringState::View)),
        make_type_tuple(vec![make_type_prim("u8"), make_type_prim("u8")]),
    ];
    for count in [0, 1, params.len()] {
        let args: Vec<TypeRef> = (0..count)
            .map(|index| pool[index % pool.len()].clone())
            .collect();
        let subst = build_substitution(params, &args);
        let _ = write!(out, "S\t{name}\t{count}");
        for (param, ty) in &subst {
            let _ = write!(out, "\t{param}={}", escape(&type_to_string(ty)));
        }
        out.push('\n');
        for member in members {
            let _ = writeln!(
                out,
                "I\t{name}\t{count}\t{}",
                escape(&type_to_string(&instantiate_type(member, &subst)))
            );
        }
    }
    let _ = write!(out, "V\t{name}");
    for (param, variance) in &compute_variance_context(params, members).param_variance {
        let sign = match variance {
            Variance::Covariant => "+",
            Variance::Contravariant => "-",
            Variance::Invariant => "=",
            Variance::Bivariant => "*",
        };
        let _ = write!(out, "\t{param}{sign}");
    }
    out.push('\n');
}

fn dump_types(out: &mut String, ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) {
    for index in 0..ctx.sigma.mods.len() {
        let module = ctx.sigma.mods[index].clone();
        dump_line("X", &module.path, out);
        ctx.current_module = module.path.clone();
        let names = name_maps
            .get(&path_key_of(&module.path))
            .cloned()
            .unwrap_or_default();
        ctx.scopes = vec![Scope::new(), names, universe_bindings()];
        let mut lowered_types = Vec::new();
        for (label, written) in collect_written_types(&module) {
            let _ = write!(out, "L\t{}\t", escape(&label));
            let lowered = match lower_type(ctx, &written) {
                Ok(lowered) => lowered,
                Err(diag_id) => {
                    let _ = writeln!(out, "fail\t{}", diag_id.unwrap_or("-"));
                    continue;
                }
            };
            lowered_types.push(lowered.clone());
            let _ = write!(out, "ok\t{}\t", escape(&type_to_string(&lowered)));
            print_key(out, &type_key_of(&lowered));
            out.push('\t');
            let paths = type_paths(&lowered);
            out.push_str(
                &paths
                    .iter()
                    .map(|path| string_of_path(path))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            let _ = write!(
                out,
                "\t{}{}",
                if is_range_type(&lowered) { 'r' } else { '-' },
                if is_range_index_type(&lowered) {
                    'i'
                } else {
                    '-'
                }
            );
            out.push('\t');
            out.push_str(&layout_text(layout_of(ctx, &lowered)));
            let _ = write!(
                out,
                " {} {}",
                opt_u64(size_of(ctx, &lowered)),
                opt_u64(align_of(ctx, &lowered))
            );
            match lower_type_for_layout(ctx, &written) {
                Some(for_layout) => {
                    let layout = layout_text(layout_of(ctx, &for_layout));
                    let _ = write!(out, " {} {layout}", escape(&type_to_string(&for_layout)));
                }
                None => out.push_str(" nolower"),
            }
            match async_sig_of(ctx, &lowered) {
                Some(sig) => {
                    let text = |ty: &TypeRef| escape(&type_to_string(ty));
                    let _ = write!(
                        out,
                        "\tasync {} / {} / {} / {}",
                        text(&sig.out),
                        text(&sig.input),
                        text(&sig.result),
                        text(&sig.err)
                    );
                }
                None => out.push_str("\t-"),
            }
            out.push('\n');
            dump_shape_layouts(out, ctx, &lowered);
            for path in &paths {
                let _ = write!(out, "P\t{}\t", string_of_path(path));
                match lookup_type_decl_resolved(ctx, path) {
                    Some((decl, resolved)) => {
                        let kind = match decl {
                            TypeDecl::Record(_) => 0,
                            TypeDecl::Enum(_) => 1,
                            TypeDecl::Modal(_) => 2,
                            TypeDecl::TypeAlias(_) => 3,
                        };
                        let _ = writeln!(out, "{kind}\t{}", string_of_path(&resolved));
                    }
                    None => out.push_str("-\t-\n"),
                }
            }
        }
        let count = lowered_types.len().min(48);
        for i in 0..count {
            let _ = write!(out, "Q\t{i}\t");
            for j in 0..count {
                out.push(if type_equiv(&lowered_types[i], &lowered_types[j]) {
                    '1'
                } else {
                    '0'
                });
            }
            out.push('\n');
        }
        let dynamic = dyn_layout_of(ctx);
        let _ = write!(out, "DL\t{}", layout_text(Some(dynamic.layout)));
        for field in &dynamic.fields {
            let _ = write!(out, "\t{}", escape(&type_to_string(field)));
        }
        out.push('\n');
        for item in &module.items {
            dump_decl_layouts(out, ctx, item);
        }
        for item in &module.items {
            match item {
                ASTItem::RecordDecl(node) => {
                    let mut members = Vec::new();
                    for field in record_fields(node) {
                        let ty = field_type(node, &field.name, ctx, &[]);
                        let elsewhere = ["Elsewhere".to_string(), node.name.clone()];
                        let _ = writeln!(
                            out,
                            "FT\t{}.{}\t{}\t{}",
                            node.name,
                            field.name,
                            ty.map_or_else(|| "-".to_string(), |ty| escape(&type_to_string(&ty))),
                            if field_visible(ctx, node, &field.name, &elsewhere) {
                                "visible"
                            } else {
                                "hidden"
                            }
                        );
                        if let Ok(lowered) = lower_type(ctx, &field.r#type) {
                            members.push(lowered);
                        }
                    }
                    if let Some(params) = &node.generic_params {
                        dump_instantiations(out, &node.name, &params.params, &members);
                    }
                }
                ASTItem::TypeAliasDecl(node) => {
                    if let (Some(params), Ok(lowered)) =
                        (&node.generic_params, lower_type(ctx, &node.r#type))
                    {
                        dump_instantiations(out, &node.name, &params.params, &[lowered]);
                    }
                }
                _ => {}
            }
        }
    }
}

// ---- relations between types; see `DumpRelations` in the oracle ----

fn diag_or(diag_id: Option<&str>) -> &str {
    diag_id.unwrap_or("-")
}

fn print_signature(out: &mut String, sig: &Result<Signature, Option<&'static str>>) {
    match sig {
        Err(diag_id) => {
            let _ = write!(out, "fail\t{}", diag_or(*diag_id));
        }
        Ok(sig) => {
            let _ = write!(
                out,
                "ok\t{}\t{}\t",
                escape(&type_to_string(&sig.func_type)),
                escape(&type_to_string(&sig.return_type))
            );
            for (name, ty) in &sig.bindings {
                let _ = write!(out, "{name}:{};", escape(&type_to_string(ty)));
            }
        }
    }
}

fn print_proof(out: &mut String, proof: &StaticProofResult) {
    let _ = write!(
        out,
        "{}/{}/{}",
        if proof.provable { '1' } else { '0' },
        diag_or(proof.diag_id),
        escape(proof.explanation)
    );
}

fn print_const(out: &mut String, value: ConstValue) {
    match value {
        ConstValue::Unknown => out.push('?'),
        ConstValue::Bool(value) => out.push_str(if value { "true" } else { "false" }),
        ConstValue::Int(value) => {
            let _ = write!(out, "{value}");
        }
    }
}

fn dump_contract(out: &mut String, owner: &str, contract: &Option<ContractClause>) {
    let Some(contract) = contract else {
        return;
    };
    let _ = write!(out, "PC\t{owner}");
    for clause in [&contract.precondition, &contract.postcondition] {
        out.push('\t');
        if clause.is_none() {
            out.push('-');
            continue;
        }
        print_proof(out, &static_proof(&StaticProofContext::default(), clause));
        out.push(' ');
        print_const(out, evaluate_constant(clause));
        out.push(' ');
        out.push(if ent_true(clause) { 't' } else { '-' });
        out.push(
            if negated_predicate(clause).is_some_and(|negated| negated.is_some()) {
                'n'
            } else {
                '-'
            },
        );
    }
    out.push('\t');
    if contract.precondition.is_some() && contract.postcondition.is_some() {
        let mut proof_ctx = StaticProofContext::default();
        add_predicate_facts(&mut proof_ctx, &contract.precondition);
        let _ = write!(out, "{} ", proof_ctx.facts.len());
        print_proof(out, &static_proof(&proof_ctx, &contract.postcondition));
        out.push(' ');
        out.push(if ent_fact(&proof_ctx, &contract.postcondition) {
            'f'
        } else {
            '-'
        });
        out.push(if ent_linear(&proof_ctx, &contract.postcondition) {
            'l'
        } else {
            '-'
        });
        if let Some(negated) =
            negated_predicate(&contract.precondition).filter(|negated| negated.is_some())
        {
            let mut neg_ctx = StaticProofContext::default();
            add_predicate_facts(&mut neg_ctx, &negated);
            out.push(' ');
            print_proof(out, &static_proof(&neg_ctx, &contract.postcondition));
        }
    } else {
        out.push('-');
    }
    out.push('\n');
}

fn flag_bit(out: &mut String, set: bool) {
    out.push(if set { '1' } else { '0' });
}

fn flag(out: &mut String, set: bool, mark: char) {
    out.push(if set { mark } else { '-' });
}

fn print_diag_list(out: &mut String, diags: &[Diagnostic]) {
    for diag in diags {
        let _ = write!(out, " [{}|{}|", diag.code, escape(&diag.message));
        match &diag.span {
            Some(span) => {
                let _ = write!(out, "{}", span.start_offset);
            }
            None => out.push('-'),
        }
        out.push(']');
    }
}

fn type_text(ty: &TypeRef) -> String {
    if ty.is_some() {
        escape(&type_to_string(ty))
    } else {
        "-".to_string()
    }
}

fn dump_generic_params(
    out: &mut String,
    ctx: &ScopeContext<'_>,
    name: &str,
    generic_params: &Option<GenericParams>,
    types: &[TypeRef],
) {
    let params: &[TypeParam] = generic_params.as_ref().map_or(&[], |params| &params.params);
    let _ = write!(
        out,
        "GP\t{name}\t{}/{}{}\t",
        required_param_count(generic_params),
        total_param_count(generic_params),
        if has_default_params(generic_params) {
            'd'
        } else {
            '-'
        }
    );
    match validate_generic_params(generic_params) {
        Ok(infos) => {
            out.push_str("ok -");
            for info in &infos {
                let default = info
                    .default_type
                    .as_ref()
                    .map_or_else(|| "none".to_string(), type_text);
                let _ = write!(out, " {}/{}/{default}", info.name, info.class_bounds.len());
            }
        }
        Err(error) => {
            let _ = write!(out, "fail {}", diag_or(error.diag_id));
            print_diag_list(out, &error.diagnostics);
        }
    }
    out.push('\t');
    let processed = process_generic_params(ctx, params);
    match &processed {
        Ok(infos) => {
            out.push_str("ok -");
            for info in infos {
                let _ = write!(
                    out,
                    " {}/{}/{}",
                    info.name,
                    info.class_bounds.len(),
                    type_text(&info.default_type)
                );
            }
        }
        Err(diag_id) => {
            let _ = write!(out, "fail {}", diag_or(*diag_id));
        }
    }
    let _ = write!(out, "\t{}", bind_type_params(ctx, generic_params).len());
    let scope = build_param_scope(generic_params);
    let mut keys: Vec<&String> = scope.keys().collect();
    keys.sort();
    for key in keys {
        let entity = &scope.get(key).expect("key of the scope");
        let _ = write!(
            out,
            " {key}={}/{}",
            entity.target_opt.as_deref().unwrap_or("-"),
            entity.type_param_class_bounds.len()
        );
    }
    out.push('\t');
    for param in params {
        let info = parse_const_param(param, &make_type_prim("u8"));
        let _ = write!(out, "{}=", info.name);
        match info.default_value {
            Some(value) => {
                let _ = write!(out, "{value}");
            }
            None => out.push('-'),
        }
        out.push(
            if parse_const_param(param, &make_type_prim("bool"))
                .r#type
                .is_some()
            {
                '!'
            } else {
                ' '
            },
        );
    }
    out.push('\n');
    for n in 0..=params.len() + 1 {
        for shift in [0usize, 7] {
            if types.is_empty() && (n != 0 || shift != 0) {
                continue;
            }
            let args: Vec<TypeRef> = (0..n)
                .map(|i| types[(shift + i * 3) % types.len()].clone())
                .collect();
            let _ = write!(out, "GB\t{name}\t{n}.{shift}\t");
            match check_bounds_satisfied(params, &args) {
                Ok(()) => out.push_str("ok"),
                Err(error) => {
                    let _ = write!(
                        out,
                        "fail {}|{}|{}|{}",
                        diag_or(error.diag_id),
                        error.param_name,
                        escape(&error.type_name),
                        error.bound_name
                    );
                    print_diag_list(out, &error.diagnostics);
                }
            }
            out.push('\t');
            match &processed {
                Ok(infos) => match check_generic_args(ctx, infos, &args) {
                    Ok(()) => out.push_str("ok -"),
                    Err(diag_id) => {
                        let _ = write!(out, "fail {diag_id}");
                    }
                },
                Err(_) => out.push('-'),
            }
            out.push('\n');
        }
    }
}

fn dump_inference(
    out: &mut String,
    ctx: &ScopeContext<'_>,
    name: &str,
    generic_params: &Option<GenericParams>,
    proc_params: &[Param],
    types: &[TypeRef],
) {
    let params: &[TypeParam] = generic_params.as_ref().map_or(&[], |params| &params.params);
    let mut bound_ctx = ctx.clone();
    bound_ctx.scopes = bind_type_params(ctx, generic_params);
    let expected: Vec<TypeRef> = proc_params
        .iter()
        .map(|param| lower_type(&bound_ctx, &param.r#type).unwrap_or(None))
        .collect();
    for variant in 0..3 {
        if variant != 0 && types.is_empty() {
            continue;
        }
        let actual: Vec<TypeRef> = match variant {
            0 => expected.clone(),
            1 => {
                let args: Vec<TypeRef> = (0..params.len())
                    .map(|j| types[(j * 5 + 1) % types.len()].clone())
                    .collect();
                let subst = build_substitution(params, &args);
                expected
                    .iter()
                    .map(|ty| {
                        if ty.is_some() {
                            instantiate_type(ty, &subst)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            _ => (0..expected.len())
                .map(|i| types[i % types.len()].clone())
                .collect(),
        };
        let inferred = infer_type_arguments(params, &expected, &actual);
        let _ = write!(
            out,
            "IN\t{name}\t{variant}\t{} {}",
            if inferred.ok { "ok" } else { "fail" },
            diag_or(inferred.diag_id)
        );
        for arg in &inferred.inferred_args {
            let _ = write!(out, "\t{}", type_text(arg));
        }
        out.push('\n');
    }
}

fn dump_instantiation_set(out: &mut String, types: &[TypeRef]) {
    let mut mono = MonomorphizeContext::default();
    let count = types.len().min(12);
    let key = |name: &str, args: Vec<TypeRef>| InstantiationKey {
        decl_path: vec!["M".to_string(), name.to_string()],
        args,
    };
    for ty in &types[..count] {
        mono.demand(&key("G", vec![ty.clone()]));
    }
    for i in 0..count {
        mono.demand(&key(
            "G",
            vec![types[i].clone(), types[count - 1 - i].clone()],
        ));
    }
    for i in 0..count {
        mono.demand(&key("F", vec![types[count - 1 - i].clone()]));
    }
    mono.demand(&key("A", Vec::new()));
    mono.demand(&key("A", Vec::new()));
    let _ = write!(out, "MI\t{}", mono.instantiations().len());
    for (inst, entry) in mono.instantiations() {
        let _ = write!(out, "\t{}<", string_of_path(&inst.decl_path));
        for arg in &inst.args {
            let _ = write!(out, "{};", type_text(arg));
        }
        let _ = write!(out, ">{}", if entry.processed { 'p' } else { '-' });
    }
    out.push('\t');
    out.push(if mono.has_instantiation(&key("A", Vec::new())) {
        '1'
    } else {
        '0'
    });
    out.push(if mono.has_instantiation(&key("B", Vec::new())) {
        '1'
    } else {
        '0'
    });
    out.push(if mono.process_to_fixed_point() {
        'T'
    } else {
        'F'
    });
    let _ = writeln!(
        out,
        "{}",
        mono.instantiations()
            .values()
            .filter(|entry| entry.processed)
            .count()
    );
}

fn dump_solving(out: &mut String, ctx: &ScopeContext<'_>, types: &[TypeRef]) {
    let count = types.len().min(24);
    let var = |id: u32| make_type(TypeNode::Var(id));
    let eq = |lhs: TypeRef, rhs: TypeRef| Constraint {
        lhs,
        rhs,
        requires_subtyping: false,
    };
    let sub = |lhs: TypeRef, rhs: TypeRef| Constraint {
        lhs,
        rhs,
        requires_subtyping: true,
    };
    let range = |base: TypeRef| make_type(TypeNode::Range(base));
    let func = |param: TypeRef, ret: TypeRef| {
        make_type_func(
            vec![TypeFuncParam {
                mode: None,
                r#type: param,
            }],
            ret,
        )
    };
    let closure = |param: TypeRef, ret: TypeRef| make_type_closure(vec![(false, param)], ret, None);
    let unique = |base: TypeRef| make_type_perm(Permission::Unique, base);
    for i in 0..count {
        let a = &types[i];
        let b = &types[(i + 1) % types.len()];
        let (a, b) = (|| a.clone(), || b.clone());
        let valid = Some(PtrState::Valid);
        let sets: Vec<Vec<Constraint>> = vec![
            vec![eq(var(0), a())],
            vec![eq(
                make_type_tuple(vec![var(0), var(1)]),
                make_type_tuple(vec![a(), b()]),
            )],
            vec![eq(var(0), a()), eq(var(0), b())],
            vec![sub(var(0), a()), sub(b(), var(1)), eq(var(1), var(0))],
            vec![eq(var(0), make_type_tuple(vec![var(0), a()]))],
            vec![sub(a(), b())],
            vec![
                eq(make_type_slice(var(0)), make_type_slice(a())),
                eq(
                    make_type_raw_ptr(RawPtrQual::Imm, var(1)),
                    make_type_raw_ptr(RawPtrQual::Imm, b()),
                ),
                eq(make_type_ptr(var(2), valid), make_type_ptr(a(), valid)),
            ],
            vec![eq(func(var(0), var(1)), func(a(), b())), eq(var(0), var(1))],
            vec![
                eq(var(0), var(1)),
                eq(var(1), var(2)),
                eq(var(2), a()),
                eq(unique(var(3)), unique(var(0))),
            ],
            vec![eq(
                make_type_union(vec![var(0), a()]),
                make_type_union(vec![b(), a()]),
            )],
            vec![
                eq(range(var(0)), range(a())),
                eq(a(), var(1)),
                eq(
                    make_type_array(var(0), 3, None),
                    make_type_array(b(), 3, None),
                ),
            ],
            vec![eq(closure(var(0), var(1)), closure(a(), b()))],
            vec![eq(var(1), var(0)), eq(var(0), var(1)), eq(var(0), a())],
            vec![
                eq(unique(var(0)), a()),
                eq(make_type_array(var(1), 2, None), b()),
                eq(make_type_tuple(vec![var(2), var(3)]), a()),
            ],
        ];
        let _ = write!(out, "SV\t{i}");
        for set in &sets {
            out.push('\t');
            match solve(ctx, set) {
                Err(diag_id) => {
                    let _ = write!(out, "fail {}", diag_or(diag_id));
                }
                Ok(subst) => {
                    out.push_str("ok");
                    for id in 0..4u32 {
                        if let Some(bound) = subst.get(&id) {
                            let _ = write!(out, " {id}={}", type_text(bound));
                        }
                    }
                    let all = make_type_tuple(vec![var(0), var(1), var(2), var(3)]);
                    let _ = write!(out, " => {}", type_text(&apply_substitution(&all, &subst)));
                }
            }
        }
        let _ = write!(out, "\nSU\t{i}\t");
        for other in &types[..count] {
            out.push(match solve(ctx, &[eq(a(), other.clone())]) {
                Ok(_) => '1',
                Err(Some("Syn-Call-Err")) => '0',
                Err(_) => '?',
            });
        }
        out.push('\n');
    }
}

fn dump_purity(out: &mut String, ctx: &ScopeContext<'_>, name: &str, contract: &Option<ContractClause>, body: &BlockPtr) {
    let mark = |expr: &ExprPtr| match expr {
        None => '-',
        Some(_) if is_pure_in_scope(ctx, expr) => '1',
        Some(_) => '0',
    };
    let mut marks = String::new();
    if let Some(contract) = contract {
        marks.push(mark(&contract.precondition));
        marks.push(mark(&contract.postcondition));
    }
    marks.push('|');
    if let Some(body) = body.as_deref() {
        for stmt in &body.stmts {
            marks.push(match stmt {
                Stmt::LetStmt(node) => mark(&node.binding.init),
                Stmt::VarStmt(node) => mark(&node.binding.init),
                Stmt::ExprStmt(node) => mark(&node.value),
                Stmt::ReturnStmt(node) => mark(&node.value_opt),
                Stmt::AssignStmt(node) => mark(&node.value),
                _ => '.',
            });
        }
        marks.push(mark(&body.tail_opt));
    }
    let _ = writeln!(out, "PU\t{name}\t{marks}");
}

fn dump_relations(out: &mut String, ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) {
    const FOUNDATIONAL: [&str; 9] = [
        "Bitcopy", "Clone", "Drop", "FfiSafe", "GpuSafe", "Eq", "Discrete", "Hash", "Iterator",
    ];
    for index in 0..ctx.sigma.mods.len() {
        let module = ctx.sigma.mods[index].clone();
        dump_line("X", &module.path, out);
        ctx.current_module = module.path.clone();
        let names = name_maps
            .get(&path_key_of(&module.path))
            .cloned()
            .unwrap_or_default();
        ctx.scopes = vec![Scope::new(), names, universe_bindings()];
        let ctx = &*ctx;
        let mut types: Vec<TypeRef> = Vec::new();
        for (label, written) in collect_written_types(&module) {
            let Ok(ty) = lower_type(ctx, &written) else {
                continue;
            };
            types.push(ty.clone());
            let text = type_to_string(&ty);
            let _ = write!(out, "W\t{}\t{}\t", escape(&label), escape(&text));
            if text.contains(" where { ... }") {
                out.push_str("pending");
            } else {
                match type_wf(ctx, &ty) {
                    Ok(()) => out.push_str("ok"),
                    Err(diag_id) => {
                        let _ = write!(out, "fail:{}", diag_or(diag_id));
                    }
                }
            }
            out.push('\t');
            flag(out, bitcopy_type(ctx, &ty), 'b');
            flag(out, clone_type(ctx, &ty), 'c');
            flag(out, drop_type(ctx, &ty), 'd');
            flag(out, zeroable_type(ctx, &ty), 'z');
            flag(out, eq_type(&ty), 'e');
            flag(out, eq_type_in(ctx, &ty), 'E');
            flag(out, builtin_discrete_type(&ty), 'i');
            flag(out, ord_type(&ty), 'o');
            flag(out, is_capability_type(&ty), 'C');
            flag(out, ffi_safe_type(ctx, &ty), 'f');
            flag(out, is_valid_const_param_type(&ty), 'k');
            let _ = write!(
                out,
                "{}\t{}\t{}\t{}",
                match perm_of_type(&ty) {
                    Permission::Const => 0,
                    Permission::Unique => 1,
                    Permission::Shared => 2,
                },
                diag_or(ffi_safe_diag_for_type(ctx, &ty)),
                diag_or(gpu_safe_diag_for_type(ctx, &ty)),
                escape(&type_to_string(&strip_perm(&ty)))
            );
            for method in ["eq", "successor", "predecessor"] {
                out.push('\t');
                match lookup_foundational_builtin_method_sig(Some(ctx), &ty, method) {
                    Some(sig) => {
                        let _ = write!(out, "{}(", escape(&type_to_string(&sig.recv_type)));
                        for param in &sig.params {
                            out.push_str(&escape(&type_to_string(&param.r#type)));
                        }
                        let _ = write!(out, ")->{}", escape(&type_to_string(&sig.ret)));
                    }
                    None => out.push('-'),
                }
                out.push(
                    if lookup_foundational_builtin_method_sig(None, &ty, method).is_some() {
                        '+'
                    } else {
                        '-'
                    },
                );
            }
            out.push('\n');
            if let Some(TypeNode::Refine { base, predicate }) = ty.as_deref().map(|ty| &ty.node) {
                if predicate.is_some() {
                    out.push_str("PR\t");
                    print_proof(
                        out,
                        &static_proof(&StaticProofContext::default(), predicate),
                    );
                    out.push('\t');
                    print_const(out, evaluate_constant(predicate));
                    let mut self_ctx = StaticProofContext::default();
                    add_predicate_facts(&mut self_ctx, predicate);
                    let _ = write!(out, "\t{} ", self_ctx.facts.len());
                    print_proof(out, &static_proof(&self_ctx, predicate));
                    match get_type_bounds(base) {
                        Some((min, max)) => {
                            let _ = writeln!(out, "\t11 {min} {max}");
                        }
                        None => out.push_str("\t00 0 0\n"),
                    }
                }
            }
        }
        let count = types.len().min(40);
        for i in 0..count {
            let (mut sub, mut arg, mut cast, mut diags) =
                (String::new(), String::new(), String::new(), String::new());
            let mark = |res: SubtypingResult| match (res.ok, res.subtype) {
                (false, _) => 'e',
                (true, true) => '1',
                (true, false) => '0',
            };
            for j in 0..count {
                let res = subtyping(ctx, &types[i], &types[j]);
                sub.push(mark(res));
                if let Some(diag_id) = res.diag_id {
                    let _ = write!(diags, " {j}={diag_id}");
                }
                arg.push(mark(argument_type_compatible(
                    ctx, &types[i], &types[j], None,
                )));
                cast.push(if cast_valid(&types[i], &types[j]) {
                    '1'
                } else {
                    '0'
                });
            }
            let _ = writeln!(out, "SB\t{i}\t{sub}\t{arg}\t{cast}\t{diags}");
        }
        let implements = |out: &mut String, class: &[String]| {
            for ty in &types[..count] {
                out.push(if type_implements_class(ctx, ty, class) {
                    '1'
                } else {
                    '0'
                });
            }
        };
        dump_instantiation_set(out, &types);
        dump_solving(out, ctx, &types);
        for name in FOUNDATIONAL {
            let _ = write!(out, "FI\t{name}\t");
            implements(out, &[name.to_string()]);
            out.push('\n');
        }
        let child = |name: &str| [&module.path[..], &[name.to_string()]].concat();
        let classes: Vec<Vec<String>> = module
            .items
            .iter()
            .filter_map(|item| match item {
                ASTItem::ClassDecl(decl) => Some(child(&decl.name)),
                _ => None,
            })
            .collect();
        for item in &module.items {
            let named: Option<(&str, &Option<GenericParams>)> = match item {
                ASTItem::ProcedureDecl(node) => Some((&node.name, &node.generic_params)),
                ASTItem::ComptimeProcedureDecl(node) => Some((&node.name, &node.generic_params)),
                ASTItem::RecordDecl(node) => Some((&node.name, &node.generic_params)),
                ASTItem::EnumDecl(node) => Some((&node.name, &node.generic_params)),
                ASTItem::ModalDecl(node) => Some((&node.name, &node.generic_params)),
                ASTItem::ClassDecl(node) => Some((&node.name, &node.generic_params)),
                ASTItem::TypeAliasDecl(node) => Some((&node.name, &node.generic_params)),
                _ => None,
            };
            if let Some((name, generic_params @ Some(_))) = named {
                dump_generic_params(out, ctx, name, generic_params, &types);
            }
            match item {
                ASTItem::ProcedureDecl(node) if node.generic_params.is_some() => {
                    dump_inference(
                        out,
                        ctx,
                        &node.name,
                        &node.generic_params,
                        &node.params,
                        &types,
                    );
                }
                ASTItem::ComptimeProcedureDecl(node) if node.generic_params.is_some() => {
                    dump_inference(
                        out,
                        ctx,
                        &node.name,
                        &node.generic_params,
                        &node.params,
                        &types,
                    );
                }
                _ => {}
            }
            match item {
                ASTItem::ProcedureDecl(node) => {
                    dump_contract(out, &node.name, &node.contract);
                    dump_purity(out, ctx, &node.name, &node.contract, &node.body);
                }
                ASTItem::ClassDecl(node) => {
                    let path = child(&node.name);
                    let _ = write!(out, "CL\t{}\t", node.name);
                    match linearize_class(ctx, &path) {
                        Ok(order) => {
                            out.push_str("ok");
                            for entry in &order {
                                let _ = write!(out, " {}", string_of_path(entry));
                            }
                        }
                        Err(diag_id) => {
                            let _ = write!(out, "fail {diag_id}");
                        }
                    }
                    out.push('\t');
                    match class_method_table(ctx, &path) {
                        Ok(methods) => {
                            out.push_str("ok");
                            for entry in &methods {
                                let _ = write!(
                                    out,
                                    " {}::{}{}",
                                    string_of_path(&entry.owner),
                                    entry.method.name,
                                    if vtable_eligible(entry.method) {
                                        '+'
                                    } else {
                                        '-'
                                    }
                                );
                            }
                        }
                        Err(diag_id) => {
                            let _ = write!(out, "fail {diag_id}");
                        }
                    }
                    out.push('\t');
                    match class_field_table(ctx, &path) {
                        Ok(fields) => {
                            out.push_str("ok");
                            for field in &fields {
                                let _ = write!(out, " {}", field.name);
                            }
                        }
                        Err(diag_id) => {
                            let _ = write!(out, "fail {diag_id}");
                        }
                    }
                    let _ = write!(
                        out,
                        "\t{}\t",
                        diag_or(class_dispatchability_diagnostic(ctx, &path))
                    );
                    flag(out, class_dispatchable(ctx, &path), 'd');
                    flag(out, dispatchable(node), 'D');
                    flag(out, is_modal_class(node), 'm');
                    flag(out, is_capability_class(ctx, &path), 'c');
                    let _ = write!(
                        out,
                        "\t{}/{}\t",
                        class_associated_types(node).len(),
                        class_abstract_states(node).len()
                    );
                    for other in &classes {
                        out.push(if class_subtypes(ctx, &path, other) {
                            '1'
                        } else {
                            '0'
                        });
                    }
                    out.push('\t');
                    implements(out, &path);
                    out.push('\t');
                    for class_item in &node.items {
                        if let ClassItem::ClassMethodDecl(method) = class_item {
                            out.push(match lookup_class_method(ctx, &path, &method.name) {
                                Some(found) if std::ptr::eq(found, method) => '=',
                                Some(_) => '^',
                                None => '!',
                            });
                        }
                    }
                    out.push('\n');
                }
                ASTItem::RecordDecl(node) => {
                    let path = child(&node.name);
                    let self_type = make_type_path(path.clone());
                    for class_path in &node.implements {
                        let _ = write!(out, "IM\t{}\t{}\t", node.name, string_of_path(class_path));
                        match missing_impl_methods(ctx, class_path, node) {
                            None => out.push_str("fail -"),
                            Some(missing) if missing.is_empty() => out.push_str("ok -"),
                            Some(missing) => {
                                out.push_str("fail E-TYP-IMPL-INCOMPLETE");
                                for name in &missing {
                                    let _ = write!(out, " {name}");
                                }
                            }
                        }
                        out.push('\t');
                        out.push(if check_orphan_rule(ctx, &path, class_path, &module.path) {
                            '1'
                        } else {
                            '0'
                        });
                        out.push(
                            if check_orphan_rule(ctx, &path, class_path, &["Elsewhere".to_string()])
                            {
                                '1'
                            } else {
                                '0'
                            },
                        );
                        out.push(if type_implements_class(ctx, &self_type, class_path) {
                            '1'
                        } else {
                            '0'
                        });
                        out.push('\n');
                    }
                    for member in &node.members {
                        let RecordMember::MethodDecl(method) = member else {
                            continue;
                        };
                        let _ = write!(out, "MS\t{}::{}\t", node.name, method.name);
                        let sig = build_method_signature(
                            ctx,
                            &self_type,
                            &method.receiver,
                            &method.params,
                            &method.return_type_opt,
                            None,
                        );
                        print_signature(out, &sig);
                        let _ = writeln!(
                            out,
                            "\t{}",
                            if recv_mode_of(&method.receiver).is_some() {
                                "move"
                            } else {
                                "-"
                            }
                        );
                        dump_contract(
                            out,
                            &format!("{}::{}", node.name, method.name),
                            &method.contract,
                        );
                        let owner = format!("{}::{}", node.name, method.name);
                        dump_purity(out, ctx, &owner, &method.contract, &method.body);
                    }
                }
                ASTItem::ModalDecl(node) => {
                    let path = child(&node.name);
                    for state in &node.states {
                        let state_type =
                            make_type_modal_state(path.clone(), &state.name, Vec::new());
                        let _ = write!(out, "MW\t{}@{}\t", node.name, state.name);
                        flag(out, niche_compatible(ctx, &path, &state.name), 'n');
                        flag(out, widen_warn_cond(ctx, &path, &state.name), 'w');
                        flag(out, has_state(node, &state.name), 's');
                        out.push('\n');
                        let same = |found: bool| if found { '=' } else { '!' };
                        for member in &state.members {
                            match member {
                                StateMember::StateMethodDecl(method) => {
                                    let _ = write!(
                                        out,
                                        "MS\t{}@{}::{}\t",
                                        node.name, state.name, method.name
                                    );
                                    let sig = build_method_signature(
                                        ctx,
                                        &state_type,
                                        &method.receiver,
                                        &method.params,
                                        &method.return_type_opt,
                                        None,
                                    );
                                    print_signature(out, &sig);
                                    let found =
                                        lookup_state_method_decl(node, &state.name, &method.name);
                                    let _ = writeln!(
                                        out,
                                        "\t{}",
                                        same(
                                            found.is_some_and(|found| std::ptr::eq(found, method))
                                        )
                                    );
                                }
                                StateMember::TransitionDecl(transition) => {
                                    let _ = write!(
                                        out,
                                        "TS\t{}@{}::{}\t",
                                        node.name, state.name, transition.name
                                    );
                                    let target = make_type_modal_state(
                                        path.clone(),
                                        &transition.target_state,
                                        Vec::new(),
                                    );
                                    print_signature(
                                        out,
                                        &build_transition_signature(
                                            ctx,
                                            &state_type,
                                            &target,
                                            &transition.params,
                                            None,
                                        ),
                                    );
                                    let found =
                                        lookup_transition_decl(node, &state.name, &transition.name);
                                    let _ =
                                        writeln!(
                                            out,
                                            "\t{}",
                                            same(found.is_some_and(|found| std::ptr::eq(
                                                found, transition
                                            )))
                                        );
                                }
                                StateMember::StateFieldDecl(field) => {
                                    let found =
                                        lookup_modal_field_decl(node, &state.name, &field.name);
                                    let _ = writeln!(
                                        out,
                                        "MF\t{}@{}.{}\t{}",
                                        node.name,
                                        state.name,
                                        field.name,
                                        same(found.is_some_and(|found| std::ptr::eq(found, field)))
                                    );
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// ---- the bytes of values; see `DumpConsts` and `DumpValues` in the oracle ----

fn hex(bits: &Option<Vec<u8>>) -> String {
    match bits {
        None => "-".to_string(),
        Some(bits) => {
            let mut out = String::with_capacity(1 + 2 * bits.len());
            out.push('x');
            for byte in bits {
                let _ = write!(out, "{byte:02x}");
            }
            out
        }
    }
}

fn dump_consts(out: &mut String, label: &str, path: &str) -> Result<(), String> {
    const PRIMS: [&str; 20] = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f16", "f32", "f64", "bool", "char", "()", "!", "string",
    ];
    let bytes = std::fs::read(path).map_err(|err| format!("cannot read {path}: {err}"))?;
    let _ = writeln!(out, "F\t{label}");
    let loaded = load_source(label, &bytes);
    let Some(source) = loaded.source else {
        out.push_str("NOSOURCE\n");
        return Ok(());
    };
    let result = tokenize_with_diagnostics(&source);
    let Some(output) = result.output else {
        out.push_str("NOTOKENS\n");
        return Ok(());
    };
    let raw = make_type_raw_ptr(RawPtrQual::Imm, make_type_prim("u8"));
    for token in &output.tokens {
        use uv_source::lexer::token::TokenKind as K;
        if !matches!(
            token.kind,
            K::IntLiteral
                | K::FloatLiteral
                | K::CharLiteral
                | K::BoolLiteral
                | K::NullLiteral
                | K::StringLiteral
        ) {
            continue;
        }
        let _ = write!(out, "C\t{}\t{}", token.kind.name(), escape(&token.lexeme));
        for prim in PRIMS {
            let bits = encode_const(&make_type_prim(prim), token);
            if bits.is_some() {
                let _ = write!(out, "\t{prim}={}", hex(&bits));
            }
        }
        let bits = encode_const(&raw, token);
        if bits.is_some() {
            let _ = write!(out, "\traw={}", hex(&bits));
        }
        let literal = LiteralExpr {
            literal: token.clone(),
        };
        let typed = type_literal_expr(&literal);
        if typed.ok {
            let _ = write!(out, "\ttype={}", escape(&type_to_string(&typed.r#type)));
        } else {
            let _ = write!(out, "\ttype=fail:{}", diag_or(typed.diag_id));
        }
        let mut expected: Vec<(&str, TypeRef)> = PRIMS
            .iter()
            .map(|prim| (*prim, make_type_prim(prim)))
            .collect();
        let view = make_type_string(Some(StringState::View));
        expected.extend([
            ("view", view.clone()),
            ("managed", make_type_string(Some(StringState::Managed))),
            ("text", make_type_string(None)),
            ("raw", raw.clone()),
            ("uniq-raw", make_type_perm(Permission::Unique, raw.clone())),
            ("ptr", make_type_ptr(make_type_prim("u8"), None)),
            (
                "uniq-i64",
                make_type_perm(Permission::Unique, make_type_prim("i64")),
            ),
            (
                "const-f64",
                make_type_perm(Permission::Const, make_type_prim("f64")),
            ),
            ("const-view", make_type_perm(Permission::Const, view)),
            ("none", None),
        ]);
        let (mut accepted, mut rejected) = (String::new(), String::new());
        for (name, ty) in &expected {
            match check_literal_expr(&literal, ty) {
                Ok(()) => {
                    let _ = write!(accepted, " {name}");
                }
                Err(Some(diag_id)) => {
                    let _ = write!(rejected, " {name}:{diag_id}");
                }
                Err(None) => {}
            }
        }
        let _ = write!(out, "\tok={accepted}\tno={rejected}\tnull=");
        flag_bit(out, null_literal_expected(&raw));
        flag_bit(out, null_literal_expected(&expected[0].1));
        if token.kind == K::StringLiteral {
            let _ = write!(
                out,
                "\tstr={}",
                hex(&decode_string_literal_bytes(&token.lexeme))
            );
        }
        out.push('\n');
    }
    Ok(())
}

/// A value of the type, chosen by the seed, as the oracle's `GenValue` chooses it.
fn gen_value(ctx: &ScopeContext<'_>, type_ref: &TypeRef, seed: u64, depth: u32) -> Value {
    let Some(ty) = type_ref.as_deref().filter(|_| depth <= 6) else {
        return Value::Unit;
    };
    let mixed = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
    let range = |kind| Value::Range {
        kind,
        lo: Some(seed % 100),
        hi: Some(seed % 100 + 5),
    };
    match &ty.node {
        TypeNode::Prim(name) => match name.as_str() {
            "bool" => Value::Bool(seed & 1 != 0),
            "char" => Value::Char((0x41 + seed % 26) as u32),
            "f16" | "f32" | "f64" => Value::Float {
                r#type: name.clone(),
                bits: mixed,
            },
            "()" | "!" => Value::Unit,
            _ => Value::Int {
                r#type: name.clone(),
                value: u128::from(mixed),
            },
        },
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => {
            gen_value(ctx, base, seed, depth + 1)
        }
        TypeNode::Ptr { state, .. } => {
            let state = state.unwrap_or(
                [PtrState::Valid, PtrState::Null, PtrState::Expired][(seed % 3) as usize],
            );
            Value::Ptr {
                state,
                addr: if state == PtrState::Null {
                    0
                } else {
                    0x1000 + seed
                },
            }
        }
        TypeNode::RawPtr { qual, .. } => Value::RawPtr(RawPtrVal {
            qual: *qual,
            addr: 0x2000 + seed,
        }),
        TypeNode::Tuple(elements) => Value::Tuple(
            elements
                .iter()
                .enumerate()
                .map(|(i, ty)| gen_value(ctx, ty, seed + i as u64 + 1, depth + 1))
                .collect(),
        ),
        TypeNode::Array {
            element, length, ..
        } => {
            if *length > 16 {
                return Value::Unit;
            }
            Value::Array(
                (0..*length)
                    .map(|i| gen_value(ctx, element, seed + i, depth + 1))
                    .collect(),
            )
        }
        TypeNode::Slice(_) => Value::Slice {
            ptr: RawPtrVal {
                qual: RawPtrQual::Imm,
                addr: 0x3000 + seed,
            },
            length: seed,
        },
        TypeNode::Range(_) => range(ValueRangeKind::Exclusive),
        TypeNode::RangeInclusive(_) => range(ValueRangeKind::Inclusive),
        TypeNode::RangeFrom(_) => range(ValueRangeKind::From),
        TypeNode::RangeTo(_) => range(ValueRangeKind::To),
        TypeNode::RangeToInclusive(_) => range(ValueRangeKind::ToInclusive),
        TypeNode::RangeFull => range(ValueRangeKind::Full),
        TypeNode::Path {
            path,
            generic_args: args,
        }
        | TypeNode::Apply { path, args } => {
            let Some(decl) = lookup_enum_decl(ctx, path).filter(|decl| !decl.variants.is_empty())
            else {
                return Value::Unit;
            };
            let variant = &decl.variants[(seed % decl.variants.len() as u64) as usize];
            let member_value = |member: Option<EnumPayloadMemberLayout>, seed| {
                gen_value(
                    ctx,
                    &member.and_then(|member| member.r#type),
                    seed,
                    depth + 1,
                )
            };
            let payload = match &variant.payload_opt {
                None => None,
                Some(VariantPayload::VariantPayloadTuple(tuple)) => Some(EnumPayloadVal::Tuple(
                    (0..tuple.elements.len())
                        .map(|i| {
                            member_value(
                                enum_tuple_payload_member_layout(ctx, decl, variant, args, i),
                                seed + i as u64 + 1,
                            )
                        })
                        .collect(),
                )),
                Some(VariantPayload::VariantPayloadRecord(record)) => Some(EnumPayloadVal::Record(
                    record
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, field)| {
                            let member = enum_record_payload_member_layout(
                                ctx,
                                decl,
                                variant,
                                args,
                                &field.name,
                            );
                            (
                                field.name.clone(),
                                member_value(member, seed + i as u64 + 1),
                            )
                        })
                        .collect(),
                )),
            };
            Value::Enum {
                variant: variant.name.clone(),
                payload,
            }
        }
        TypeNode::Dynamic(_) => Value::Dynamic {
            data: seed,
            vtable: seed + 1,
        },
        TypeNode::String(_) => Value::String(Vec::new()),
        TypeNode::Bytes(_) => Value::Bytes(Vec::new()),
        _ => Value::Unit,
    }
}

fn dump_values(out: &mut String, ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) {
    for index in 0..ctx.sigma.mods.len() {
        let module = ctx.sigma.mods[index].clone();
        dump_line("X", &module.path, out);
        ctx.current_module = module.path.clone();
        let names = name_maps
            .get(&path_key_of(&module.path))
            .cloned()
            .unwrap_or_default();
        ctx.scopes = vec![Scope::new(), names, universe_bindings()];
        let ctx = &*ctx;
        let types: Vec<TypeRef> = collect_written_types(&module)
            .iter()
            .filter_map(|(_, written)| lower_type(ctx, written).ok())
            .collect();
        for (i, ty) in types.iter().enumerate().take(80) {
            let _ = write!(out, "V\t{i}\t{}", escape(&type_to_string(ty)));
            for seed in [i as u64, i as u64 + 17] {
                let bits = value_bits(ctx, ty, &gen_value(ctx, ty, seed, 0));
                let _ = write!(out, "\t{}", hex(&bits));
                if let Some(bits) = &bits {
                    out.push_str(if valid_value(ctx, ty, bits) {
                        " v"
                    } else {
                        " n"
                    });
                }
            }
            let next = &types[(i + 1) % types.len()];
            let _ = write!(
                out,
                "\t{}\t",
                hex(&value_bits(ctx, ty, &gen_value(ctx, next, i as u64, 0)))
            );
            match size_of(ctx, ty).filter(|size| *size <= 4096) {
                Some(size) => {
                    let size = size as usize;
                    let mixed: Vec<u8> = (0..size).map(|j| (j * 7 + 1) as u8).collect();
                    let _ = write!(out, "{size}");
                    flag(out, valid_value(ctx, ty, &vec![0; size]), 'z');
                    flag(out, valid_value(ctx, ty, &vec![0xFF; size]), 'f');
                    flag(out, valid_value(ctx, ty, &mixed), 'm');
                    flag(out, valid_value(ctx, ty, &vec![1; size]), 'o');
                    flag(out, valid_value(ctx, ty, &vec![0; size + 1]), 'l');
                }
                None => out.push('-'),
            }
            out.push('\n');
        }
    }
}

// ---- patterns; see `DumpPatterns` in the oracle ----

fn collect_expr_patterns(expr: &ExprPtr, out: &mut Vec<PatternPtr>) {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IfCaseExpr(node)) => {
            out.extend(node.cases.iter().map(|clause| clause.pattern.clone()))
        }
        Some(ExprNode::IfIsExpr(node)) => out.push(node.pattern.clone()),
        Some(ExprNode::LoopIterExpr(node)) => out.push(node.pattern.clone()),
        _ => {}
    }
}

fn collect_block_patterns(block: &BlockPtr, out: &mut Vec<PatternPtr>) {
    let Some(block) = block.as_deref() else {
        return;
    };
    for stmt in &block.stmts {
        match stmt {
            Stmt::LetStmt(node) => {
                out.push(node.binding.pat.clone());
                collect_expr_patterns(&node.binding.init, out);
            }
            Stmt::VarStmt(node) => {
                out.push(node.binding.pat.clone());
                collect_expr_patterns(&node.binding.init, out);
            }
            Stmt::ExprStmt(node) => collect_expr_patterns(&node.value, out),
            _ => {}
        }
    }
    collect_expr_patterns(&block.tail_opt, out);
}

/// The position of a pattern's form among the reference's alternatives.
fn pattern_index(pattern: &PatternPtr) -> usize {
    match pattern.as_deref().map(|pattern| &pattern.node) {
        None => 99,
        Some(PatternNode::LiteralPattern(_)) => 0,
        Some(PatternNode::WildcardPattern(_)) => 1,
        Some(PatternNode::IdentifierPattern(_)) => 2,
        Some(PatternNode::TypedPattern(_)) => 3,
        Some(PatternNode::SpliceExprNode(_)) => 4,
        Some(PatternNode::TuplePattern(_)) => 5,
        Some(PatternNode::RecordPattern(_)) => 6,
        Some(PatternNode::EnumPattern(_)) => 7,
        Some(PatternNode::ModalPattern(_)) => 8,
        Some(PatternNode::RangePattern(_)) => 9,
    }
}

fn intro_text(res: &Result<TypeEnv, Option<&'static str>>) -> String {
    match res {
        Ok(_) => "ok".to_string(),
        Err(diag_id) => format!("fail:{}", diag_or(*diag_id)),
    }
}

fn mut_char(mutability: Mutability) -> char {
    if mutability == Mutability::Var {
        'V'
    } else {
        'L'
    }
}

fn dump_env_script(out: &mut String, i: usize, j: usize, binds: &[(String, TypeRef)]) {
    let e0 = push_scope(&TypeEnv::default());
    let r1 = intro_all(&e0, binds, Mutability::Let, false);
    let outer_env = r1.clone().unwrap_or_else(|_| e0.clone());
    let e1 = push_scope(&outer_env);
    let r2 = intro_all(&e1, binds, Mutability::Var, true);
    let r3 = intro_all(&e1, binds, Mutability::Let, false);
    let env = r2.clone().unwrap_or_else(|_| e1.clone());
    let r4 = intro_all(&env, binds, Mutability::Var, true);
    let r5 = intro_all(&TypeEnv::default(), binds, Mutability::Let, false);
    let _ = write!(
        out,
        "PE\t{i}\t{j}\t{} {} {} {} {}\t",
        intro_text(&r1),
        intro_text(&r2),
        intro_text(&r3),
        intro_text(&r4),
        intro_text(&r5)
    );
    if let Some((name, _)) = binds.first() {
        match bind_of(&env, name) {
            Some(bound) => {
                let _ = write!(
                    out,
                    "{} {} {}",
                    mut_char(bound.r#mut),
                    type_text(&bound.r#type),
                    type_text(stable_binding_type(bound))
                );
            }
            None => out.push_str("unbound"),
        }
        out.push(' ');
        out.push(mut_of(&outer_env, name).map_or('-', mut_char));
        flag(out, has_heap_provenance(&env, name), 'h');
    }
    let _ = write!(
        out,
        "\t{}{}{}{}",
        pop_scope(&env).scopes.len(),
        project_type_env_to_depth(&env, 1).scopes.len(),
        project_type_env_to_depth(&env, 5).scopes.len(),
        pop_scope(&pop_scope(&pop_scope(&env))).scopes.len()
    );
    flag(out, gpu_context(&env), 'g');
    out.push('\n');
}

fn dump_env_fixed(out: &mut String) {
    let reserved = vec![("gen_tmp".to_string(), make_type_prim("i32"))];
    let e0 = push_scope(&TypeEnv::default());
    let _ = write!(
        out,
        "PG\t{} {} {}\t",
        intro_text(&intro_all(&e0, &reserved, Mutability::Let, false)),
        intro_text(&intro_all(&e0, &reserved, Mutability::Let, true)),
        intro_text(&intro_all(&e0, &[], Mutability::Var, true))
    );
    let seed = |kind: BindingProvenanceSeedKind| kind as u8;
    for kind in [
        ProvenanceKind::Global,
        ProvenanceKind::Stack,
        ProvenanceKind::Heap,
        ProvenanceKind::Region,
        ProvenanceKind::Bottom,
        ProvenanceKind::Param,
    ] {
        let mut binding = TypeBinding::default();
        apply_binding_provenance_seed(&mut binding, kind, Some("Arena"));
        let _ = write!(
            out,
            "{}{}{} ",
            seed(binding.provenance_kind),
            binding.provenance_region.as_deref().unwrap_or("-"),
            seed(normalize_binding_provenance_seed(kind))
        );
    }
    let mut env = push_scope(&e0);
    let mut derived = TypeBinding {
        derived_from_shared: true,
        ..Default::default()
    };
    apply_binding_provenance_seed(&mut derived, ProvenanceKind::Heap, None);
    env.scopes[0].emplace("outer".to_string(), derived);
    env.scopes[1].emplace("inner".to_string(), TypeBinding::default());
    env.parallel_context = Some(ParallelContextKind::Gpu);
    mark_shared_derived_bindings_stale(&mut env);
    out.push('\t');
    flag(
        out,
        bind_of(&env, "outer").is_some_and(|bound| bound.stale_after_release),
        's',
    );
    flag(
        out,
        bind_of(&env, "inner").is_some_and(|bound| bound.stale_after_release),
        's',
    );
    flag(out, has_heap_provenance(&env, "outer"), 'h');
    flag(out, has_heap_provenance(&env, "inner"), 'h');
    flag(out, has_heap_provenance(&env, "none"), 'h');
    flag(out, gpu_context(&env), 'g');
    flag(out, env.parallel_context.is_some(), 'p');
    out.push('\n');
}

fn dump_patterns(out: &mut String, ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) {
    dump_env_fixed(out);
    for index in 0..ctx.sigma.mods.len() {
        let module = ctx.sigma.mods[index].clone();
        dump_line("X", &module.path, out);
        ctx.current_module = module.path.clone();
        let names = name_maps
            .get(&path_key_of(&module.path))
            .cloned()
            .unwrap_or_default();
        ctx.scopes = vec![Scope::new(), names, universe_bindings()];
        let ctx = &*ctx;
        let types: Vec<TypeRef> = collect_written_types(&module)
            .iter()
            .filter_map(|(_, written)| lower_type(ctx, written).ok())
            .collect();
        let mut patterns: Vec<PatternPtr> = Vec::new();
        for item in &module.items {
            match item {
                ASTItem::StaticDecl(node) => patterns.push(node.binding.pat.clone()),
                ASTItem::ProcedureDecl(node) => collect_block_patterns(&node.body, &mut patterns),
                ASTItem::RecordDecl(node) => {
                    for member in &node.members {
                        if let RecordMember::MethodDecl(method) = member {
                            collect_block_patterns(&method.body, &mut patterns);
                        }
                    }
                }
                _ => {}
            }
        }
        let types = &types[..types.len().min(48)];
        for (j, ty) in types.iter().enumerate() {
            let _ = writeln!(out, "PY\t{j}\t{}", escape(&type_to_string(ty)));
        }
        for (i, pattern) in patterns.iter().enumerate().take(96) {
            let _ = write!(out, "PT\t{i}\t{}", pattern_index(pattern));
            for (j, ty) in types.iter().enumerate() {
                let typed = type_pattern_against_type(ctx, pattern, ty);
                let irrefutable = irrefutable_pattern(ctx, pattern, ty);
                let covers = enum_pattern_covers_variant(ctx, pattern, ty);
                let covers_state = modal_pattern_covers_state(ctx, pattern, ty);
                if matches!(typed, Err(None)) && !irrefutable && !covers && !covers_state {
                    continue;
                }
                let _ = write!(out, "\t{j}:");
                match &typed {
                    Ok(bindings) => {
                        out.push_str("ok");
                        for (name, ty) in bindings {
                            let _ = write!(out, " {name}={}", escape(&type_to_string(ty)));
                        }
                    }
                    Err(diag_id) => {
                        let _ = write!(out, "fail {}", diag_or(*diag_id));
                    }
                }
                out.push_str(" /");
                flag(out, irrefutable, 'i');
                flag(out, covers, 'e');
                flag(out, covers_state, 'm');
            }
            out.push('\n');
            let mut names = Vec::new();
            if let Some(pattern) = pattern.as_deref() {
                collect_pat_names(pattern, &mut names);
            }
            let _ = write!(out, "PN\t{i}\t");
            for name in &names {
                let _ = write!(out, "{name},");
            }
            out.push('\t');
            flag(out, distinct_names(&names), 'd');
            let _ = write!(out, "\nPS\t{i}");
            let mut first_ok: Option<(usize, Vec<(String, TypeRef)>)> = None;
            for (j, ty) in types.iter().enumerate() {
                let typed = type_pattern(ctx, pattern, ty);
                if matches!(typed, Err(None))
                    || (matches!(typed, Err(Some("Let-Refutable-Pattern-Err"))) && j != 0)
                {
                    continue;
                }
                let _ = write!(out, "\t{j}:");
                match typed {
                    Ok(bindings) => {
                        out.push_str("ok");
                        for (name, ty) in &bindings {
                            let _ = write!(out, " {name}={}", escape(&type_to_string(ty)));
                        }
                        if first_ok.is_none() {
                            first_ok = Some((j, bindings));
                        }
                    }
                    Err(diag_id) => {
                        let _ = write!(out, "fail {}", diag_or(diag_id));
                    }
                }
            }
            out.push('\n');
            if let Some((j, bindings)) = &first_ok {
                dump_env_script(out, i, *j, bindings);
            }
        }
    }
}

// ---- body typing; see `DumpBody` in the oracle ----

#[allow(clippy::too_many_arguments)]
fn dump_body(
    out: &mut String,
    ctx: &ScopeContext<'_>,
    name: &str,
    generic_params: &Option<GenericParams>,
    params: &[Param],
    return_type_opt: &TypePtr,
    contract: &Option<ContractClause>,
    body: &BlockPtr,
) {
    let Some(body) = body.as_deref() else {
        return;
    };
    let _ = write!(out, "B\t{name}\t");
    let mut proc_ctx = ctx.clone();
    proc_ctx.scopes = bind_type_params(ctx, generic_params);
    let mut env = TypeEnv::default();
    env.scopes.push(Default::default());
    for param in params {
        let lowered = match lower_type(&proc_ctx, &param.r#type) {
            Ok(lowered) => lowered,
            Err(diag_id) => {
                let _ = writeln!(out, "param-fail\t{}", diag_or(diag_id));
                return;
            }
        };
        let binding = TypeBinding {
            r#type: lowered.clone(),
            storage_type: lowered,
            provenance_kind: BindingProvenanceSeedKind::Param,
            ..Default::default()
        };
        env.scopes[0].insert(id_key_of(&param.name), binding);
    }
    let return_type = if return_type_opt.is_some() {
        match lower_type(&proc_ctx, return_type_opt) {
            Ok(lowered) => lowered,
            Err(diag_id) => {
                let _ = writeln!(out, "return-fail\t{}", diag_or(diag_id));
                return;
            }
        }
    } else {
        make_type_prim("()")
    };
    dump_typed_body(out, &mut proc_ctx, env, return_type, contract.as_ref(), None, None, body, true);
}

/// The block of a body under its typing context, and the line that reports it. A
/// transition is typed without the environment reference and the diagnostic stream.
#[allow(clippy::too_many_arguments)]
fn dump_typed_body(
    out: &mut String,
    proc_ctx: &mut ScopeContext<'_>,
    env: TypeEnv,
    return_type: TypeRef,
    contract: Option<&ContractClause>,
    class_path: Option<Vec<String>>,
    proof_ctx: Option<StaticProofContext>,
    body: &Block,
    with_env: bool,
) {
    let diags = Rc::new(RefCell::new(Vec::new()));
    let env = Rc::new(RefCell::new(env));
    proc_ctx.diagnostics = Some(diags.clone());
    let flags = BODY_FLAGS.take();
    // The stores the type checker's entry point sets up; they are also read back.
    let stores = Rc::new(TypeStores::default());
    proc_ctx.stores = Some(stores.clone());
    let type_ctx = StmtTypeContext {
        return_type,
        diags: with_env.then(|| diags.clone()),
        env_ref: with_env.then(|| env.clone()),
        contract,
        current_class_path: class_path,
        proof_ctx: proof_ctx.map(Rc::new),
        contract_dynamic: flags.dynamic,
        ffi_export_boundary: flags.ffi_export_boundary,
        test_postcondition_runtime: flags.test_postcondition_runtime,
        ..Default::default()
    };
    // The callbacks read the environment as it stands when they are called.
    let proc_ctx = &*proc_ctx;
    let type_expr_fn =
        |inner: &ExprPtr| type_expr(proc_ctx, &type_ctx, inner, &env.borrow().clone());
    let type_ident_fn = |ident: &str| type_identifier_expr(proc_ctx, &env.borrow(), ident);
    let type_place_fn =
        |inner: &ExprPtr| type_place(proc_ctx, &type_ctx, inner, &env.borrow().clone());
    let start_env = env.borrow().clone();
    reset_scaffolding();
    let result = type_block(
        proc_ctx,
        &type_ctx,
        body,
        &start_env,
        &type_expr_fn,
        &type_ident_fn,
        &type_place_fn,
        with_env.then_some(&env),
    );
    if let Some(what) = take_pending() {
        let _ = writeln!(out, "PENDING\t{what}");
        return;
    }
    let _ = write!(
        out,
        "{}\t{}\t{}\t{}\t",
        if result.ok { "ok" } else { "fail" },
        diag_or(result.diag_id),
        escape(&result.diag_detail),
        type_text(&result.r#type)
    );
    match &result.diag_span {
        Some(span) => {
            let _ = write!(out, "{}-{}", span.start_offset, span.end_offset);
        }
        None => out.push('-'),
    }
    let _ = write!(out, "\t{}\t", env.borrow().scopes.len());
    for id in &result.diagnostic_obligation_ids {
        let _ = write!(out, "{id},");
    }
    out.push('\t');
    print_diag_list(out, &diags.borrow());
    let span_key = |expr: &uv_source::ast::Expr| format!("{}-{}", expr.span.start_offset, expr.span.end_offset);
    let verbose = std::env::var_os("UV_STORE_VERBOSE").is_some();
    let mut store_lines = String::new();
    let mut print_store = |out: &mut String, tag: char, mut entries: Vec<String>| {
        entries.sort();
        let mut hash: u64 = 1469598103934665603;
        for entry in &entries {
            for byte in entry.bytes().chain([0x0a]) {
                hash = (hash ^ u64::from(byte)).wrapping_mul(1099511628211);
            }
            if verbose {
                let _ = writeln!(store_lines, "S\t{tag}\t{entry}");
            }
        }
        let _ = write!(out, "{}:{hash:x}", entries.len());
    };
    out.push('\t');
    // Only the expressions of the project's modules: an entry for any other is one the
    // checker synthesized, which the reference keys by an address it has freed since.
    let in_syntax = |expr: &std::sync::Arc<uv_source::ast::Expr>| SYNTAX_EXPRS.with_borrow(|set| set.contains(&(std::sync::Arc::as_ptr(expr) as usize)));
    let typed = |(expr, ty): &(std::sync::Arc<uv_source::ast::Expr>, TypeRef)| {
        in_syntax(expr).then(|| format!("{}={}", span_key(expr), type_text(ty)))
    };
    print_store(out, 'T', stores.expr_types.borrow().values().filter_map(typed).collect());
    out.push(' ');
    print_store(out, 'V', stores.expr_value_types.borrow().values().filter_map(typed).collect());
    out.push(' ');
    let refinements = stores
        .dynamic_refine_checks
        .borrow()
        .values()
        .filter(|(expr, _)| in_syntax(expr))
        .map(|(expr, types)| format!("{}={}", span_key(expr), types.iter().map(|ty| type_text(ty) + ";").collect::<String>()))
        .collect();
    print_store(out, 'D', refinements);
    out.push(' ');
    let substs = stores
        .generic_call_substs
        .borrow()
        .values()
        .map(|subst| subst.iter().map(|(name, ty)| format!("{name}={};", type_text(ty))).collect::<String>())
        .collect();
    print_store(out, 'G', substs);
    out.push(' ');
    let targets = stores
        .selected_call_targets
        .borrow()
        .values()
        .map(|target| target.module_path.iter().map(|seg| format!("{seg}::")).collect::<String>() + &target.proc_name)
        .collect();
    print_store(out, 'C', targets);
    out.push('\n');
    out.push_str(&store_lines);
}

/// The body of a method or transition under the bindings of its signature, as the typing
/// of record, class and modal declarations sets it up; `self` of a `unique` shorthand
/// receiver is mutable.
#[allow(clippy::too_many_arguments)]
fn dump_signature_body(
    out: &mut String,
    ctx: &ScopeContext<'_>,
    name: &str,
    sig: Result<Signature, Option<&'static str>>,
    self_var: bool,
    return_type: Option<TypeRef>,
    contract: Option<&ContractClause>,
    class_path: Option<Vec<String>>,
    proof_ctx: Option<StaticProofContext>,
    body: &BlockPtr,
    with_env: bool,
) {
    let Some(body) = body.as_deref() else {
        return;
    };
    let _ = write!(out, "B\t{name}\t");
    let sig = match sig {
        Ok(sig) => sig,
        Err(diag_id) => {
            BODY_FLAGS.take();
            let _ = writeln!(out, "sig-fail\t{}", diag_or(diag_id));
            return;
        }
    };
    let mut env = TypeEnv::default();
    env.scopes.push(Default::default());
    for (binding_name, binding_type) in &sig.bindings {
        let r#mut = if self_var && binding_name == "self" { Mutability::Var } else { Mutability::Let };
        env.scopes[0].insert(id_key_of(binding_name), TypeBinding { r#mut, r#type: binding_type.clone(), ..Default::default() });
    }
    let mut method_ctx = ctx.clone();
    dump_typed_body(out, &mut method_ctx, env, return_type.unwrap_or(sig.return_type), contract, class_path, proof_ctx, body, with_env);
}

fn unique_shorthand(receiver: &Receiver) -> bool {
    matches!(receiver, Receiver::ReceiverShorthand(shorthand) if shorthand.perm == ReceiverPerm::Unique)
}

fn const_shorthand(receiver: &Receiver) -> bool {
    matches!(receiver, Receiver::ReceiverShorthand(shorthand) if shorthand.perm == ReceiverPerm::Const && shorthand.mode_opt.is_none())
}

fn decl_path(module_path: &[String], name: &str) -> Vec<String> {
    module_path.iter().cloned().chain([name.to_string()]).collect()
}

/// See `TypeRecordDecl`: the record's associated types stand in for `Self::Name`, and a
/// method with a plain `~` receiver may assume the record's invariant.
fn dump_record_method_bodies(out: &mut String, ctx: &ScopeContext<'_>, decl: &RecordDecl, module_path: &[String]) {
    let self_type = make_type_path(decl_path(module_path, &decl.name));
    let mut assoc_subst = TypeSubst::new();
    let mut assoc_fail: Option<Option<&'static str>> = None;
    for member in &decl.members {
        let RecordMember::AssociatedTypeDecl(assoc) = member else {
            continue;
        };
        if assoc_fail.is_some() {
            continue;
        }
        if assoc.default_type.is_none() {
            if !decl.implements.is_empty() {
                assoc_fail = Some(Some("E-TYP-2503"));
            }
            continue;
        }
        match lower_type(ctx, &assoc.default_type).and_then(|lowered| type_wf(ctx, &lowered).map(|()| lowered)) {
            Ok(lowered) => {
                let substituted = subst_self_type(&self_type, &lowered, Some(&assoc_subst));
                assoc_subst.insert(assoc.name.clone(), substituted);
            }
            Err(diag_id) => assoc_fail = Some(diag_id),
        }
    }
    // A method that implements a class method marked dynamic is dynamic too.
    let mut inherited_dynamic = std::collections::HashSet::new();
    for impl_path in &decl.implements {
        let Ok(method_table) = class_method_table(ctx, impl_path) else {
            continue;
        };
        for entry in method_table {
            if resolve_verification_mode_attribute(&entry.method.attrs) != Some(VerificationModeAttribute::Dynamic) {
                continue;
            }
            let implemented = decl.members.iter().find_map(|member| match member {
                RecordMember::MethodDecl(method) if id_eq(&method.name, &entry.method.name) => Some(id_key_of(&method.name)),
                _ => None,
            });
            inherited_dynamic.extend(implemented);
        }
    }
    for member in &decl.members {
        let RecordMember::MethodDecl(method) = member else {
            continue;
        };
        let Some(body) = method.body.as_deref() else {
            continue;
        };
        let name = format!("{}::{}", decl.name, method.name);
        if let Some(diag_id) = assoc_fail {
            let _ = writeln!(out, "B\t{name}\tassoc-fail\t{}", diag_or(diag_id));
            continue;
        }
        let sig = build_method_signature(ctx, &self_type, &method.receiver, &method.params, &method.return_type_opt, Some(&assoc_subst));
        let proof_ctx = match &decl.invariant_opt {
            Some(invariant) if const_shorthand(&method.receiver) => {
                extend_proof_context_with_predicate_at(None, &invariant.predicate, &body.span)
            }
            _ => None,
        };
        BODY_FLAGS.set(BodyFlags {
            dynamic: inherited_dynamic.contains(&id_key_of(&method.name))
                || dynamic_body(&method.body, (&decl.attrs, &decl.span), Some((&method.attrs, &method.span))),
            ..Default::default()
        });
        dump_signature_body(
            out,
            ctx,
            &name,
            sig,
            unique_shorthand(&method.receiver),
            None,
            method.contract.as_ref(),
            None,
            proof_ctx,
            &method.body,
            true,
        );
    }
}

/// See `TypeClassDecl`: `Self` stays a variable, and the class is the current one.
fn dump_class_method_bodies(out: &mut String, ctx: &ScopeContext<'_>, decl: &ClassDecl, module_path: &[String]) {
    for item in &decl.items {
        let ClassItem::ClassMethodDecl(method) = item else {
            continue;
        };
        if method.body_opt.is_none() {
            continue;
        }
        let sig = build_method_signature(ctx, &self_var_type(), &method.receiver, &method.params, &method.return_type_opt, None);
        BODY_FLAGS.set(BodyFlags {
            dynamic: dynamic_body(&method.body_opt, (&decl.attrs, &decl.span), Some((&method.attrs, &method.span))),
            ..Default::default()
        });
        dump_signature_body(
            out,
            ctx,
            &format!("{}::{}", decl.name, method.name),
            sig,
            unique_shorthand(&method.receiver),
            None,
            method.contract.as_ref(),
            Some(decl_path(module_path, &decl.name)),
            None,
            &method.body_opt,
            true,
        );
    }
}

/// The part of a modal invariant that speaks of one state; see
/// `StateInvariantPredicateFor`.
fn state_invariant_of(invariant: &TypeInvariant, state_name: &str) -> ExprPtr {
    let Some(ExprNode::IfCaseExpr(if_case)) = invariant.predicate.as_deref().map(|predicate| &predicate.node) else {
        return invariant.predicate.clone();
    };
    let on_self = matches!(if_case.scrutinee.as_deref().map(|scrutinee| &scrutinee.node),
        Some(ExprNode::IdentifierExpr(ident)) if id_eq(&ident.name, "self"));
    if !on_self {
        return invariant.predicate.clone();
    }
    for arm in &if_case.cases {
        if let Some(PatternNode::ModalPattern(pattern)) = arm.pattern.as_deref().map(|pattern| &pattern.node) {
            if id_eq(&pattern.state, state_name) {
                return arm.body.clone();
            }
        }
    }
    invariant.predicate.clone()
}

/// See `TypeModalDecl`: the receiver is the state, applied to the modal's own parameters;
/// a transition returns its target state and is typed without the environment reference.
fn dump_modal_bodies(out: &mut String, ctx: &ScopeContext<'_>, decl: &ModalDecl, module_path: &[String]) {
    let type_path = decl_path(module_path, &decl.name);
    let self_args: Vec<TypeRef> =
        decl.generic_params.iter().flat_map(|params| &params.params).map(|param| make_type_path(vec![param.name.clone()])).collect();
    for state in &decl.states {
        let state_type = make_type_modal_state(type_path.clone(), &state.name, self_args.clone());
        for member in &state.members {
            let StateMember::StateMethodDecl(method) = member else {
                continue;
            };
            let Some(body) = method.body.as_deref() else {
                continue;
            };
            let sig = build_method_signature(ctx, &state_type, &method.receiver, &method.params, &method.return_type_opt, None);
            let proof_ctx = match &decl.invariant_opt {
                Some(invariant) if const_shorthand(&method.receiver) => {
                    extend_proof_context_with_predicate_at(None, &state_invariant_of(invariant, &state.name), &body.span)
                }
                _ => None,
            };
            BODY_FLAGS.set(BodyFlags {
                dynamic: dynamic_body(&method.body, (&decl.attrs, &decl.span), Some((&method.attrs, &method.span))),
                ..Default::default()
            });
            dump_signature_body(
                out,
                ctx,
                &format!("{}@{}::{}", decl.name, state.name, method.name),
                sig,
                unique_shorthand(&method.receiver),
                None,
                method.contract.as_ref(),
                None,
                proof_ctx,
                &method.body,
                true,
            );
        }
        for member in &state.members {
            let StateMember::TransitionDecl(transition) = member else {
                continue;
            };
            if transition.body.is_none() {
                continue;
            }
            let target_type = make_type_modal_state(type_path.clone(), &transition.target_state, self_args.clone());
            let sig = build_transition_signature(ctx, &state_type, &target_type, &transition.params, None);
            BODY_FLAGS.set(BodyFlags {
                dynamic: dynamic_body(&transition.body, (&decl.attrs, &decl.span), Some((&transition.attrs, &transition.span))),
                ..Default::default()
            });
            dump_signature_body(
                out,
                ctx,
                &format!("{}@{}->{}::{}", decl.name, state.name, transition.target_state, transition.name),
                sig,
                false,
                Some(target_type),
                None,
                None,
                None,
                &transition.body,
                false,
            );
        }
    }
}

// ---- the type checker's entry point; see `DumpTypecheck` in the oracle ----

fn print_typecheck_diag(out: &mut String, diag: &Diagnostic) {
    let span_text = |span: &Option<Span>| match span {
        Some(span) => format!("{}-{}", span.start_offset, span.end_offset),
        None => "-".to_string(),
    };
    let _ = write!(
        out,
        " [{}|{}|{}|{}|{}|",
        diag.code,
        diag.severity as u8,
        escape(&diag.message),
        span_text(&diag.span),
        diag.label.as_deref().map_or("-".to_string(), escape)
    );
    for id in &diag.obligation_ids {
        let _ = write!(out, "{id},");
    }
    out.push('|');
    for child in &diag.children {
        let _ = write!(out, "{{{}:{}:{}}}", child.kind as u8, escape(&child.message), span_text(&child.span));
    }
    out.push(']');
}

/// Each diagnostic is listed under the declaration whose extent holds its position;
/// the rest follow on the `Z` line. A declaration the port cannot type yet says so.
fn dump_typecheck(out: &mut String, ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) {
    let checked = typecheck_modules(ctx, name_maps);
    let mut used = vec![false; checked.diags.len()];
    for (module_index, module) in ctx.sigma.mods.iter().enumerate() {
        dump_line("X", &module.path, out);
        for (item_index, item) in module.items.iter().enumerate() {
            let span = item_span(item);
            let variant = match item {
                ASTItem::UsingDecl(_) => 0,
                ASTItem::ImportDecl(_) => 1,
                ASTItem::ExternBlock(_) => 2,
                ASTItem::StaticDecl(_) => 3,
                ASTItem::ProcedureDecl(_) => 4,
                ASTItem::ComptimeProcedureDecl(_) => 5,
                ASTItem::RecordDecl(_) => 6,
                ASTItem::EnumDecl(_) => 7,
                ASTItem::ModalDecl(_) => 8,
                ASTItem::ClassDecl(_) => 9,
                ASTItem::TypeAliasDecl(_) => 10,
                ASTItem::DeriveTargetDecl(_) => 11,
                ASTItem::ErrorItem(_) => 12,
            };
            let _ = write!(out, "I\t{variant}\t{}-{}\t", span.start_offset, span.end_offset);
            let pending = checked.pending_items.iter().find(|pending| pending.module == module_index && pending.item == item_index);
            for (index, diag) in checked.diags.iter().enumerate() {
                let inside = diag.span.as_ref().is_some_and(|at| {
                    at.file == span.file && at.start_offset >= span.start_offset && at.start_offset < span.end_offset
                });
                if used[index] || !inside {
                    continue;
                }
                used[index] = true;
                if pending.is_none() {
                    print_typecheck_diag(out, diag);
                }
            }
            if let Some(pending) = pending {
                let _ = write!(out, "PENDING\t{}", pending.what);
            }
            out.push('\n');
        }
    }
    if let Some(what) = &checked.pending_tail {
        let _ = writeln!(out, "Z\tPENDING\t{what}");
        return;
    }
    let _ = write!(out, "Z\t{}\t{}\t", if checked.ok { "ok" } else { "fail" }, if checked.has_init_plan { "plan" } else { "-" });
    for (index, diag) in checked.diags.iter().enumerate() {
        if !used[index] {
            print_typecheck_diag(out, diag);
        }
    }
    out.push('\n');
}

/// What the declaration around a body contributes to its typing context, as the typing
/// of declarations sets it; taken by the next body dumped.
#[derive(Clone, Copy, Default)]
struct BodyFlags {
    dynamic: bool,
    ffi_export_boundary: bool,
    test_postcondition_runtime: bool,
}

fn dynamic_body(body: &BlockPtr, outer: (&[AttributeItem], &Span), inner: Option<(&[AttributeItem], &Span)>) -> bool {
    let Some(body) = body.as_deref() else {
        return false;
    };
    let ancestors: Vec<DynamicScopeAncestor<'_>> =
        [Some(outer), inner].into_iter().flatten().map(|(attrs, span)| DynamicScopeAncestor { attrs, span }).collect();
    compute_dynamic_context(&body.span, &ancestors)
}

thread_local! {
    static BODY_FLAGS: std::cell::Cell<BodyFlags> = const { std::cell::Cell::new(BodyFlags { dynamic: false, ffi_export_boundary: false, test_postcondition_runtime: false }) };
    /// The addresses of the expressions of the project's modules.
    static SYNTAX_EXPRS: RefCell<std::collections::HashSet<usize>> = RefCell::default();
}

fn dump_bodies(out: &mut String, ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) {
    let mut syntax_exprs = Vec::new();
    for module in &ctx.sigma.mods {
        module.items.walk_exprs(&mut syntax_exprs);
    }
    SYNTAX_EXPRS.set(syntax_exprs.into_iter().map(|expr| expr as usize).collect());
    for index in 0..ctx.sigma.mods.len() {
        let module = ctx.sigma.mods[index].clone();
        dump_line("X", &module.path, out);
        ctx.current_module = module.path.clone();
        let names = name_maps
            .get(&path_key_of(&module.path))
            .cloned()
            .unwrap_or_default();
        ctx.scopes = vec![Scope::new(), names, universe_bindings()];
        for item in &module.items {
            if let ASTItem::ProcedureDecl(node) = item {
                BODY_FLAGS.set(BodyFlags {
                    dynamic: dynamic_body(&node.body, (&node.attrs, &node.span), None),
                    ffi_export_boundary: has_attribute(&node.attrs, attrs::EXPORT) || has_attribute(&node.attrs, attrs::HOST_EXPORT),
                    test_postcondition_runtime: has_attribute(&node.attrs, attrs::TEST),
                });
                dump_body(
                    out,
                    ctx,
                    &node.name,
                    &node.generic_params,
                    &node.params,
                    &node.return_type_opt,
                    &node.contract,
                    &node.body,
                );
                BODY_FLAGS.take();
            }
            match item {
                ASTItem::RecordDecl(node) => dump_record_method_bodies(out, ctx, node, &module.path),
                ASTItem::ClassDecl(node) => dump_class_method_bodies(out, ctx, node, &module.path),
                ASTItem::ModalDecl(node) => dump_modal_bodies(out, ctx, node, &module.path),
                _ => {}
            }
        }
    }
}

/// The built-in declarations; see `DumpSigma` in the oracle.
fn dump_sigma(out: &mut String) {
    let mut ctx = ScopeContext::default();
    populate_sigma(&mut ctx);
    for (key, decl) in &ctx.sigma.types {
        out.push_str("T\t");
        key.dump(out);
        out.push('\t');
        match decl {
            TypeDecl::Record(node) => node.dump(out),
            TypeDecl::Enum(node) => node.dump(out),
            TypeDecl::Modal(node) => node.dump(out),
            TypeDecl::TypeAlias(node) => node.dump(out),
        }
        out.push('\n');
    }
    for (key, decl) in &ctx.sigma.classes {
        out.push_str("K\t");
        key.dump(out);
        out.push('\t');
        decl.dump(out);
        out.push('\n');
    }
}

struct Quiet;

impl Phase1Observer for Quiet {
    fn assembly_start(&self, _: &Assembly) {}
    fn assembly_finish(&self, _: &Assembly, _: AssemblyOutcome) {}
}

/// Writes the comptime list block for one manifest, or nothing when the project does not
/// pass phase 1. See `DumpComptime` in the oracle for the format.
fn comptime_list_block(out: &mut String, manifest: &str) {
    let root = find_project_root(manifest);
    let loaded = load_project(&root, &AssemblyTarget::default());
    let Some(project) = loaded.project.filter(|_| !has_error(&loaded.diags)) else {
        return;
    };
    let phase1 = run_phase1(&project, &Quiet);
    if !phase1.ok || has_error(&phase1.diags) {
        return;
    }
    let _ = writeln!(
        out,
        "P\t{manifest}\t{}\t{}",
        project.root, project.source_root
    );
    for assembly in &project.assemblies {
        let _ = writeln!(out, "A\t{}\t{}", assembly.name, assembly.source_root);
    }
    let _ = writeln!(out, "R\t{}", phase1.reachable_count);
    for info in &phase1.project_module_infos {
        let _ = write!(out, "M\t{}", info.path);
        for file in compilation_unit(&info.dir).files {
            let _ = write!(out, "\t{file}");
        }
        out.push('\n');
    }
    out.push_str("E\n");
}

fn dump_unicode(out: &mut impl std::io::Write) -> std::io::Result<()> {
    for c in 0..=0x10FFFFu32 {
        let Some(ch) = char::from_u32(c) else {
            continue;
        };
        let text = ch.to_string();
        let xid_start = is_xid_start(c);
        let xid_continue = is_xid_continue(c);
        let normalized = nfc(&text);
        let folded = case_fold(&text);
        let probe = format!("a{text}");
        let info = analyze_identifier_security(&probe);
        let trivial = !xid_start
            && !xid_continue
            && normalized == text
            && folded == text
            && info.skeleton == probe
            && !info.mixed_script;
        if trivial {
            continue;
        }
        writeln!(
            out,
            "{:X}\t{}\t{}\t{}\t{}\t{}\t{}",
            c,
            xid_start as u8,
            xid_continue as u8,
            escape(&normalized),
            escape(&folded),
            escape(&info.skeleton),
            info.mixed_script as u8
        )?;
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    match args.get(1).map(String::as_str) {
        Some("unicode") => dump_unicode(&mut stdout).map_err(|err| err.to_string()),
        Some(mode @ ("tokens" | "ast" | "consts")) if args.len() >= 3 => {
            // The list file holds one "label<TAB>path" per line. `--root` rewrites the
            // container-side `/w/` prefix used when the list was written for the oracle.
            let root = args.iter().position(|a| a == "--root").and_then(|i| args.get(i + 1));
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            for line in list.lines() {
                let Some((label, path)) = line.split_once('\t') else {
                    continue;
                };
                let path = match (root, path.strip_prefix("/w/")) {
                    (Some(root), Some(rest)) => format!("{root}/{rest}"),
                    _ => path.to_string(),
                };
                let mut out = String::new();
                if mode == "ast" {
                    dump_ast(&mut out, label, &path)?;
                } else if mode == "consts" {
                    dump_consts(&mut out, label, &path)?;
                } else {
                    dump_tokens(&mut out, label, &path)?;
                }
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
                if mode == "ast" {
                    stdout.flush().map_err(|err| err.to_string())?;
                }
            }
            Ok(())
        }
        Some("sigma") => {
            let mut out = String::new();
            dump_sigma(&mut out);
            stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())
        }
        Some(mode @ ("comptime" | "resolve" | "types" | "relations" | "values" | "patterns" | "bodies" | "typecheck")) if args.len() >= 3 => {
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            let mut block: Vec<&str> = Vec::new();
            for line in list.lines() {
                if line != "E" {
                    block.push(line);
                    continue;
                }
                let mut out = String::new();
                if mode != "comptime" {
                    dump_resolve(&mut out, &block, mode);
                } else {
                    dump_comptime(&mut out, &block);
                }
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
                stdout.flush().map_err(|err| err.to_string())?;
                block.clear();
            }
            Ok(())
        }
        Some("comptime-list") if args.len() >= 3 => {
            // The argument lists manifests, one per line, relative to the working directory.
            let cwd = std::env::current_dir().map_err(|err| err.to_string())?;
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            for manifest in list.lines().filter(|line| !line.is_empty()) {
                let mut out = String::new();
                comptime_list_block(&mut out, &cwd.join(manifest).to_string_lossy());
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
            }
            Ok(())
        }
        _ => Err("usage: uv-parity unicode | tokens <list-file> [--root <dir>] | ast <list-file> [--root <dir>]".to_string()),
    }
}

fn main() {
    // Deeply nested inputs recurse far in the parser; the reference runs with an 8 MiB
    // main-thread stack and larger frames, so give the port generous room.
    let worker = std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(run)
        .expect("spawn worker");
    match worker.join() {
        Ok(Ok(())) => {}
        Ok(Err(message)) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
        Err(_) => std::process::exit(101),
    }
}
