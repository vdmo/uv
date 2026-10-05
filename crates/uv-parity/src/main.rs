//! Emits the same dumps as the reference oracle (`tools/oracle/oracle_main.cpp`) so the
//! two implementations can be compared byte for byte.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write as _;

use uv_core::diagnostics::Diagnostic;
use uv_core::source_load::load_source;
use uv_core::span::Span;
use uv_core::unicode::{analyze_identifier_security, case_fold, is_xid_continue, is_xid_start, nfc};
use uv_comptime::{execute_comptime, ComptimePassOptions};
use uv_analysis::context::{Entity, NameMapTable, Scope, ScopeContext, TypeDecl};
use uv_analysis::resolve::collect_toplevel::collect_name_maps;
use uv_analysis::resolve::populate_sigma::populate_sigma;
use uv_analysis::resolve::resolve_module::resolve_modules;
use uv_analysis::resolve::resolver::ResolveContext;
use uv_analysis::resolve::scopes_lookup::module_names_of;
use uv_analysis::generics::monomorphize::{build_substitution, instantiate_type};
use uv_analysis::composite::enums::enum_discriminants;
use uv_analysis::layout::{
    align_of, dyn_layout_of, enum_layout_of, enum_record_payload_member_layout, enum_tuple_payload_member_layout,
    layout_of, lower_async_type_of, lower_type_for_layout, modal_layout_of, range_layout_of, record_layout_of,
    resolve_enum_layout_options, resolve_record_layout_options, size_of, tuple_layout_of, union_layout_of,
    EnumPayloadMemberLayout, Layout, RecordLayout,
};
use uv_analysis::modal::modal_widen::payload_state;
use uv_analysis::resolve::scopes::{path_key_of, universe_bindings};
use uv_analysis::resolve::visibility::{can_access, check_module_visibility};
use uv_analysis::typing::type_equiv::type_equiv;
use uv_analysis::typing::type_lookup::{async_sig_of, field_type, field_visible, lookup_type_decl_resolved, record_fields};
use uv_analysis::typing::type_lower::lower_type;
use uv_analysis::typing::types::{
    is_range_index_type, is_range_type, make_type_prim, make_type_string, make_type_tuple, type_key_of, type_paths,
    type_to_string, KeyAtom, StringState, TypeKey, TypeNode, TypeRef,
};
use uv_analysis::typing::variance::{compute_variance_context, Variance};
use uv_source::ast::{
    ASTItem, ClassItem, ExternItem, GenericParams, Param, Receiver, RecordMember, StateMember, TypeParam, TypePtr,
    VariantPayload,
};
use uv_core::diagnostics::{emit, has_error};
use uv_core::symbols::string_of_path;
use uv_project::target_profile::TargetProfile;
use uv_source::ast::ASTModule;
use uv_project::load_project::load_project;
use uv_project::manifest::find_project_root;
use uv_project::module_discovery::compilation_unit;
use uv_project::project::{Assembly, AssemblyTarget};
use uv_source::ast::dump::AstDump;
use uv_source::phase1::{run_phase1, AssemblyOutcome, Phase1Observer};
use uv_source::lexer::tokenize_with_diagnostics;
use uv_source::parser::parse_file;

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
        span.start_offset, span.end_offset, span.start_line, span.start_col, span.end_line, span.end_col
    )
}

fn print_diags(out: &mut String, diags: &[Diagnostic]) {
    for diag in diags {
        let span = diag.span.as_ref().map_or_else(|| "-".to_string(), span_text);
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
        let _ = writeln!(out, "D\t{}\t{}\t{}", doc.kind.name(), escape(&doc.text), span_text(&doc.span));
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
    span.as_ref().map_or_else(|| "-".to_string(), |span| format!("{}\t{}", span.file, span_text(span)))
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
                out.options.source_roots_by_assembly.insert(name.to_string(), source_root.to_string());
            }
            ["R", count, ..] => out.reachable = count.parse().unwrap_or(0),
            ["M", path, files @ ..] => {
                let mut module = ASTModule { path: path.split("::").map(str::to_string).collect(), ..ASTModule::default() };
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
                    out.unsafe_spans_by_file.insert(source.path.to_string(), parsed.unsafe_spans);
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
    let ProjectBlock { label, options, modules, .. } = match load_project_block(block) {
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
fn dump_resolve(out: &mut String, block: &[&str], types_mode: bool) {
    let input = match load_project_block(block) {
        Ok(project) => project,
        Err(failure) => {
            out.push_str(&failure);
            return;
        }
    };
    let _ = writeln!(out, "F\t{}", input.label);
    let Some(project) = load_project(&input.options.project_root, &AssemblyTarget::default()).project else {
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
                assembly.modules.iter().filter(move |info| info.path == path).cloned()
            })
        })
        .collect();

    let mut ctx = ScopeContext {
        project: Some(&sema_project),
        target_profile: Some(TargetProfile::X86_64SysV),
        scopes: vec![Scope::new(), Scope::new(), Scope::new()],
        ..Default::default()
    };
    ctx.sigma.mods = parsed_modules;
    ctx.sigma.unsafe_spans_by_file = input.unsafe_spans_by_file;
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
    };
    let resolved = resolve_modules(&mut res_ctx);
    if types_mode {
        if !resolved.ok {
            out.push_str("STOP\tresolve\n");
            return;
        }
        ctx.sigma.mods = resolved.modules;
        populate_sigma(&mut ctx);
        dump_types(out, &mut ctx, &name_maps.name_maps);
        return;
    }
    let _ = writeln!(out, "RESOLVE\t{}", if resolved.ok { "ok" } else { "failed" });
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
            out.push((format!("{owner}<{}=>", param.name), param.default_type.clone()));
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
                            out.push((format!("{}.{}", node.name, field.name), field.r#type.clone()));
                        }
                        RecordMember::MethodDecl(method) => {
                            let owner = format!("{}::{}", node.name, method.name);
                            collect_params(&owner, &method.params, &method.return_type_opt, &mut out);
                            if let Receiver::ReceiverExplicit(recv) = &method.receiver {
                                out.push((format!("{owner}(self)"), recv.r#type.clone()));
                            }
                        }
                        RecordMember::AssociatedTypeDecl(assoc) => {
                            if assoc.default_type.is_some() {
                                out.push((format!("{}::{}", node.name, assoc.name), assoc.default_type.clone()));
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
                                out.push((format!("{}::{}", node.name, variant.name), element.clone()));
                            }
                        }
                        Some(VariantPayload::VariantPayloadRecord(record)) => {
                            for field in &record.fields {
                                let label = format!("{}::{}.{}", node.name, variant.name, field.name);
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
                                collect_params(&owner, &method.params, &method.return_type_opt, &mut out);
                            }
                            StateMember::TransitionDecl(trans) => {
                                collect_params(&format!("{owner}::{}", trans.name), &trans.params, &None, &mut out);
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
                            out.push((format!("{}.{}", node.name, field.name), field.r#type.clone()));
                        }
                        ClassItem::ClassMethodDecl(method) => {
                            let owner = format!("{}::{}", node.name, method.name);
                            collect_params(&owner, &method.params, &method.return_type_opt, &mut out);
                        }
                        ClassItem::AssociatedTypeDecl(assoc) => {
                            if assoc.default_type.is_some() {
                                out.push((format!("{}::{}", node.name, assoc.name), assoc.default_type.clone()));
                            }
                        }
                        ClassItem::AbstractFieldDecl(field) => {
                            out.push((format!("{}.{}", node.name, field.name), field.r#type.clone()));
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
            ASTItem::UsingDecl(_) | ASTItem::ImportDecl(_) | ASTItem::DeriveTargetDecl(_) | ASTItem::ErrorItem(_) => {}
        }
    }
    out
}

fn opt_u64(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_string(), |value| value.to_string())
}

fn layout_text(layout: Option<Layout>) -> String {
    layout.map_or_else(|| "-".to_string(), |layout| format!("{}/{}", layout.size, layout.align))
}

fn offsets_text(offsets: &[u64]) -> String {
    offsets.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
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
            let _ = writeln!(out, "{tag}\t{}\t{}", layout_text(Some(layout.layout)), offsets_text(&layout.offsets));
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
            let mut all: Vec<TypeRef> = (0..params.params.len()).map(|index| pool[index % pool.len()].clone()).collect();
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
            let fields: Option<Vec<TypeRef>> =
                record_fields(node).iter().map(|field| lower_type_for_layout(ctx, &field.r#type)).collect();
            match fields.and_then(|fields| record_layout_of(ctx, &fields, &options)) {
                Some(layout) => {
                    let offsets = offsets_text(&layout.offsets);
                    let _ = writeln!(out, "RL\t{}\t{}\t{offsets}", node.name, layout_text(Some(layout.layout)));
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
                    let _ = writeln!(out, "ED\t{}\t{}\t{}", node.name, discs.max_disc, offsets_text(&discs.discs));
                }
                Err(err) => {
                    let _ = writeln!(out, "ED\t{}\tfail\t{}\t{}", node.name, err.diag_id, span_text(&err.span));
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
                    let mut member_line = |label: String, member: Option<EnumPayloadMemberLayout>| {
                        let _ = write!(out, "EM\t{}::{}{label}\t{}\t", node.name, variant.name, args.len());
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
                                let member = enum_tuple_payload_member_layout(ctx, node, variant, &args, index);
                                member_line(format!(".{index}"), member);
                            }
                            member_line(".named".to_string(), enum_record_payload_member_layout(ctx, node, variant, &args, "x"));
                        }
                        VariantPayload::VariantPayloadRecord(record) => {
                            for field in &record.fields {
                                let member = enum_record_payload_member_layout(ctx, node, variant, &args, &field.name);
                                member_line(format!(".{}", field.name), member);
                            }
                            let missing = enum_record_payload_member_layout(ctx, node, variant, &args, "no_such_field");
                            member_line(".missing".to_string(), missing);
                            member_line(".0".to_string(), enum_tuple_payload_member_layout(ctx, node, variant, &args, 0));
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
        let args: Vec<TypeRef> = (0..count).map(|index| pool[index % pool.len()].clone()).collect();
        let subst = build_substitution(params, &args);
        let _ = write!(out, "S\t{name}\t{count}");
        for (param, ty) in &subst {
            let _ = write!(out, "\t{param}={}", escape(&type_to_string(ty)));
        }
        out.push('\n');
        for member in members {
            let _ = writeln!(out, "I\t{name}\t{count}\t{}", escape(&type_to_string(&instantiate_type(member, &subst))));
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
        let names = name_maps.get(&path_key_of(&module.path)).cloned().unwrap_or_default();
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
            out.push_str(&paths.iter().map(|path| string_of_path(path)).collect::<Vec<_>>().join(" "));
            let _ = write!(
                out,
                "\t{}{}",
                if is_range_type(&lowered) { 'r' } else { '-' },
                if is_range_index_type(&lowered) { 'i' } else { '-' }
            );
            out.push('\t');
            out.push_str(&layout_text(layout_of(ctx, &lowered)));
            let _ = write!(out, " {} {}", opt_u64(size_of(ctx, &lowered)), opt_u64(align_of(ctx, &lowered)));
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
                out.push(if type_equiv(&lowered_types[i], &lowered_types[j]) { '1' } else { '0' });
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
                            if field_visible(ctx, node, &field.name, &elsewhere) { "visible" } else { "hidden" }
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
                    if let (Some(params), Ok(lowered)) = (&node.generic_params, lower_type(ctx, &node.r#type)) {
                        dump_instantiations(out, &node.name, &params.params, &[lowered]);
                    }
                }
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
    let _ = writeln!(out, "P\t{manifest}\t{}\t{}", project.root, project.source_root);
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
        Some(mode @ ("tokens" | "ast")) if args.len() >= 3 => {
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
        Some(mode @ ("comptime" | "resolve" | "types")) if args.len() >= 3 => {
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            let mut block: Vec<&str> = Vec::new();
            for line in list.lines() {
                if line != "E" {
                    block.push(line);
                    continue;
                }
                let mut out = String::new();
                if mode != "comptime" {
                    dump_resolve(&mut out, &block, mode == "types");
                } else {
                    dump_comptime(&mut out, &block);
                }
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
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
    let worker = std::thread::Builder::new().stack_size(1 << 30).spawn(run).expect("spawn worker");
    match worker.join() {
        Ok(Ok(())) => {}
        Ok(Err(message)) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
        Err(_) => std::process::exit(101),
    }
}
