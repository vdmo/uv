//! Collecting the names each module declares or brings in, to a fixed point across modules.

use std::collections::{BTreeMap, HashMap};

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::symbols::string_of_path;
use uv_source::ast::*;
use uv_source::module_paths::{resolve_import_module_path, ModuleNames};

use super::resolve_using::using_names;
use super::scopes::*;
use super::scopes_lookup::{find_context_module_by_path, module_names_of};
use crate::context::*;

#[derive(Debug, Clone)]
pub struct BoundName {
    pub name: IdKey,
    pub ent: Entity,
    pub span: Option<Span>,
}

pub type BindingList = Vec<BoundName>;

#[derive(Debug, Clone, Default)]
pub struct BindingsResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub bindings: BindingList,
}

impl BindingsResult {
    fn bound(bindings: BindingList) -> BindingsResult {
        BindingsResult { ok: true, diag_id: None, span: None, bindings }
    }

    fn failed(diag_id: &'static str, span: &Span) -> BindingsResult {
        BindingsResult { ok: false, diag_id: Some(diag_id), span: Some(span.clone()), bindings: Vec::new() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CollectNamesResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub names: NameMap,
    pub name_spans: HashMap<IdKey, Option<Span>>,
}

#[derive(Debug, Clone, Default)]
pub struct NameMapBuildResult {
    pub name_maps: NameMapTable,
    pub diags: DiagnosticStream,
}

fn make_decl_entity(kind: EntityKind, module_path: &[String], visibility: Visibility) -> Entity {
    let mut entity = Entity::new(kind, Some(module_path.to_vec()), None, EntitySource::Decl);
    entity.visibility = Some(visibility);
    entity
}

fn decl_binding(kind: EntityKind, module_path: &[String], name: &str, vis: Visibility, span: &Span) -> BoundName {
    BoundName { name: id_key_of(name), ent: make_decl_entity(kind, module_path, vis), span: Some(span.clone()) }
}

fn no_dup(bindings: &[BoundName]) -> bool {
    let mut names: Vec<&IdKey> = bindings.iter().map(|binding| &binding.name).collect();
    names.sort();
    names.windows(2).all(|pair| pair[0] != pair[1])
}

fn disjoint_names(bindings: &[BoundName], names: &NameMap) -> bool {
    bindings.iter().all(|binding| !names.contains_key(&binding.name))
}

/// Free procedures other than `main` may be overloaded, so a repeated name is not a
/// duplicate.
fn free_procedure_overload_name(item: &ASTItem) -> Option<IdKey> {
    match item {
        ASTItem::ProcedureDecl(proc) if !id_eq(&proc.name, "main") => Some(id_key_of(&proc.name)),
        _ => None,
    }
}

fn bindings_are_single_overload_procedure(item: &ASTItem, bindings: &[BoundName]) -> bool {
    match (free_procedure_overload_name(item), bindings) {
        (Some(name), [binding]) => {
            binding.name == name && binding.ent.kind == EntityKind::Value && binding.ent.source == EntitySource::Decl
        }
        _ => false,
    }
}

fn using_import_conflict(bindings: &[BoundName], names: &NameMap) -> bool {
    let mut counts: HashMap<&IdKey, (usize, bool)> = HashMap::new();
    for binding in bindings {
        let entry = counts.entry(&binding.name).or_insert((0, false));
        entry.0 += 1;
        if binding.ent.source == EntitySource::Using {
            entry.1 = true;
        }
    }
    if counts.values().any(|(count, has_using)| *count > 1 && *has_using) {
        return true;
    }
    bindings.iter().any(|binding| {
        names.get(&binding.name).is_some_and(|existing| {
            binding.ent.source == EntitySource::Using || existing.source == EntitySource::Using
        })
    })
}

/// Adds bindings without replacing names that are already bound.
fn insert_bindings(out: &mut NameMap, bindings: &[BoundName]) {
    for binding in bindings {
        out.entry(binding.name.clone()).or_insert_with(|| binding.ent.clone());
    }
}

fn comptime_procedure_bindings(module: &ASTModule) -> BindingList {
    module
        .comptime_procedures
        .iter()
        .map(|proc| decl_binding(EntityKind::Value, &module.path, &proc.name, proc.vis, &proc.span))
        .collect()
}

/// Equality for the fixed point: where a declaration is does not matter, what it is does.
fn entity_equals(lhs: &Entity, rhs: &Entity) -> bool {
    let origin_eq = match (&lhs.origin_opt, &rhs.origin_opt) {
        (Some(a), Some(b)) => path_eq(a, b),
        (None, None) => true,
        _ => false,
    };
    let target_eq = match (&lhs.target_opt, &rhs.target_opt) {
        (Some(a), Some(b)) => id_eq(a, b),
        (None, None) => true,
        _ => false,
    };
    lhs.kind == rhs.kind && lhs.source == rhs.source && origin_eq && target_eq && lhs.visibility == rhs.visibility
}

fn name_map_equals(lhs: &NameMap, rhs: &NameMap) -> bool {
    lhs.len() == rhs.len() && lhs.iter().all(|(key, ent)| rhs.get(key).is_some_and(|other| entity_equals(ent, other)))
}

fn name_map_table_equals(lhs: &NameMapTable, rhs: &NameMapTable) -> bool {
    lhs.len() == rhs.len()
        && lhs.iter().all(|(key, names)| rhs.get(key).is_some_and(|other| name_map_equals(names, other)))
}

fn code_for_collect_diag(diag_id: &str) -> Option<&'static str> {
    Some(match diag_id {
        "Import-Using-Missing" => "E-MOD-1201",
        "Resolve-Import-Err" => "E-MOD-1202",
        "Resolve-Using-None" => "E-MOD-1204",
        "Resolve-Using-Ambig" => "E-MOD-1208",
        "Using-List-Dup" => "E-MOD-1206",
        "Import-Using-Name-Conflict" => "E-MOD-1203",
        "Using-Path-Item-Public-Err" | "Using-List-Public-Err" => "E-MOD-1205",
        "Collect-Dup" | "Names-Step-Dup" => "E-MOD-1302",
        "Access-Err" => "E-MOD-1207",
        _ => return None,
    })
}

fn emit_collect_diag(diags: &mut DiagnosticStream, diag_id: &str, span: &Option<Span>) {
    let Some(code) = code_for_collect_diag(diag_id) else {
        return;
    };
    if let Some(diag) = make_diagnostic_by_id(code, span.clone()) {
        emit(diags, diag);
    }
}

/// The names a module declares itself, before `import` and `using` are looked at.
fn build_decl_name_map(module: &ASTModule) -> NameMap {
    let mut names = NameMap::new();
    let path = &module.path;
    let mut add = |name: &str, kind: EntityKind, vis: Visibility| {
        names.entry(id_key_of(name)).or_insert_with(|| make_decl_entity(kind, path, vis));
    };
    for item in &module.items {
        match item {
            ASTItem::ProcedureDecl(node) => add(&node.name, EntityKind::Value, node.vis),
            ASTItem::ComptimeProcedureDecl(node) => add(&node.name, EntityKind::Value, node.vis),
            ASTItem::DeriveTargetDecl(node) => add(&node.name, EntityKind::Value, Visibility::Internal),
            ASTItem::RecordDecl(node) => add(&node.name, EntityKind::Type, node.vis),
            ASTItem::EnumDecl(node) => add(&node.name, EntityKind::Type, node.vis),
            ASTItem::ModalDecl(node) => add(&node.name, EntityKind::Type, node.vis),
            ASTItem::ClassDecl(node) => add(&node.name, EntityKind::Class, node.vis),
            ASTItem::TypeAliasDecl(node) => add(&node.name, EntityKind::Type, node.vis),
            ASTItem::ExternBlock(node) => {
                for ExternItem::ExternProcDecl(ext_decl) in &node.items {
                    add(&ext_decl.name, EntityKind::Value, ext_decl.vis);
                }
            }
            ASTItem::StaticDecl(node) => {
                for name in pat_names_ptr(&node.binding.pat) {
                    add(&name, EntityKind::Value, node.vis);
                }
            }
            ASTItem::UsingDecl(_) | ASTItem::ImportDecl(_) | ASTItem::ErrorItem(_) => {}
        }
    }
    insert_bindings(&mut names, &comptime_procedure_bindings(module));
    names
}

fn module_names_from_context(ctx: &ScopeContext<'_>) -> ModuleNames {
    let names: ModuleNames = ctx.sigma.mods.iter().map(|module| string_of_path(&module.path)).collect();
    match ctx.project {
        Some(project) if names.is_empty() => module_names_of(project),
        _ => names,
    }
}

fn decl_name_maps(ctx: &ScopeContext<'_>) -> NameMapTable {
    let mut maps = NameMapTable::new();
    for module in &ctx.sigma.mods {
        maps.entry(path_key_of(&module.path)).or_insert_with(|| build_decl_name_map(module));
    }
    maps
}

fn path_prefix(prefix: &[String], path: &[String]) -> bool {
    prefix.len() <= path.len() && prefix.iter().zip(path).all(|(a, b)| id_eq(a, b))
}

/// Modules of another assembly must be imported before their items can be named.
pub fn import_required(current_module: &[String], path: &[String]) -> bool {
    match (current_module.first(), path.first()) {
        (Some(current), Some(target)) => {
            spec_rule!("ImportRequired");
            !id_eq(current, target)
        }
        _ => false,
    }
}

pub fn import_covers(
    ctx: &ScopeContext<'_>,
    module_names: &ModuleNames,
    current_module: &[String],
    path: &[String],
) -> bool {
    let Some(module) = find_context_module_by_path(ctx, current_module) else {
        return false;
    };
    spec_rule!("ImportCovers");
    module.items.iter().any(|item| match item {
        ASTItem::ImportDecl(import_decl) => {
            resolve_import_module_path(current_module, module_names, &import_decl.path)
                .is_some_and(|import_path| path_prefix(&import_path, path))
        }
        _ => false,
    })
}

pub fn import_ok(ctx: &ScopeContext<'_>, module_names: &ModuleNames, current_module: &[String], path: &[String]) -> bool {
    if !import_required(current_module, path) {
        spec_rule!("Import-Ok-Local");
        return true;
    }
    if import_covers(ctx, module_names, current_module, path) {
        spec_rule!("Import-Ok-Covered");
        return true;
    }
    spec_rule!("Import-Ok-Err");
    false
}

fn field_pattern_names(fields: &[FieldPattern], out: &mut Vec<Identifier>) {
    for field in fields {
        match &field.pattern_opt {
            Some(pattern) => {
                spec_rule!("Pat-Record-Field-Explicit");
                out.extend(pat_names(pattern));
            }
            None => {
                spec_rule!("Pat-Record-Field-Implicit");
                out.push(field.name.clone());
            }
        }
    }
}

/// The names a pattern binds, in source order.
pub fn pat_names(pat: &Pattern) -> Vec<Identifier> {
    let mut out = Vec::new();
    match &pat.node {
        PatternNode::IdentifierPattern(node) => out.push(node.name.clone()),
        PatternNode::TypedPattern(node) => {
            spec_rule!("Pat-Typed");
            if node.name != "_" {
                out.push(node.name.clone());
            }
        }
        PatternNode::WildcardPattern(_) => {
            spec_rule!("Pat-Wild");
        }
        PatternNode::LiteralPattern(_) => {
            spec_rule!("Pat-Lit");
        }
        PatternNode::TuplePattern(node) => {
            for elem in &node.elements {
                out.extend(pat_names_ptr(elem));
            }
        }
        PatternNode::RecordPattern(node) => field_pattern_names(&node.fields, &mut out),
        PatternNode::EnumPattern(node) => match &node.payload_opt {
            None => {
                spec_rule!("Pat-Enum-None");
            }
            Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                spec_rule!("Pat-Enum-Tuple");
                for elem in &payload.elements {
                    out.extend(pat_names_ptr(elem));
                }
            }
            Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                spec_rule!("Pat-Enum-Record");
                field_pattern_names(&payload.fields, &mut out);
            }
        },
        PatternNode::ModalPattern(node) => {
            if let Some(payload) = &node.fields_opt {
                for field in &payload.fields {
                    match &field.pattern_opt {
                        Some(pattern) => out.extend(pat_names(pattern)),
                        None => out.push(field.name.clone()),
                    }
                }
            }
        }
        PatternNode::RangePattern(node) => {
            spec_rule!("Pat-Range");
            out.extend(pat_names_ptr(&node.lo));
            out.extend(pat_names_ptr(&node.hi));
        }
        PatternNode::SpliceExprNode(_) => {}
    }
    out
}

pub fn pat_names_ptr(pat: &PatternPtr) -> Vec<Identifier> {
    pat.as_deref().map(pat_names).unwrap_or_default()
}

/// The names one top-level item binds in its module.
pub fn item_bindings(
    ctx: &ScopeContext<'_>,
    name_maps: &NameMapTable,
    module_names: &ModuleNames,
    item: &ASTItem,
    module_path: &[String],
) -> BindingsResult {
    let single = |kind, name: &str, vis, span: &Span| {
        BindingsResult::bound(vec![decl_binding(kind, module_path, name, vis, span)])
    };
    match item {
        ASTItem::UsingDecl(node) => {
            let names = using_names(ctx, name_maps, module_names, node);
            if !names.ok {
                spec_rule!("Bind-Using-Err");
                return BindingsResult { ok: false, diag_id: names.diag_id, span: names.span, bindings: Vec::new() };
            }
            spec_rule!("Bind-Using");
            BindingsResult::bound(names.bindings)
        }
        ASTItem::ImportDecl(node) => {
            spec_rule!("Bind-Import");
            if node.path.is_empty() {
                spec_rule!("Import-Path-Err");
                spec_rule!("Bind-Import-Err");
                return BindingsResult::failed("Import-Using-Missing", &node.span);
            }
            let Some(resolved_path) = resolve_import_module_path(module_path, module_names, &node.path) else {
                spec_rule!("Import-Path-Err");
                spec_rule!("Bind-Import-Err");
                return BindingsResult::failed("Resolve-Import-Err", &node.span);
            };
            let alias_name =
                node.alias_opt.clone().unwrap_or_else(|| resolved_path.last().cloned().unwrap_or_default());
            if alias_name.is_empty() {
                spec_rule!("Import-Path-Err");
                spec_rule!("Bind-Import-Err");
                return BindingsResult::failed("Import-Using-Missing", &node.span);
            }
            spec_rule!("Import-Path");
            BindingsResult::bound(vec![BoundName {
                name: id_key_of(&alias_name),
                ent: Entity::new(EntityKind::ModuleAlias, Some(resolved_path), None, EntitySource::Import),
                span: Some(node.span.clone()),
            }])
        }
        ASTItem::ProcedureDecl(node) => {
            spec_rule!("Bind-Procedure");
            single(EntityKind::Value, &node.name, node.vis, &node.span)
        }
        ASTItem::ComptimeProcedureDecl(node) => {
            spec_rule!("Bind-Procedure");
            single(EntityKind::Value, &node.name, node.vis, &node.span)
        }
        ASTItem::DeriveTargetDecl(node) => {
            spec_rule!("Bind-Procedure");
            single(EntityKind::Value, &node.name, Visibility::Internal, &node.span)
        }
        ASTItem::ExternBlock(node) => {
            spec_rule!("Bind-Procedure");
            BindingsResult::bound(
                node.items
                    .iter()
                    .map(|ExternItem::ExternProcDecl(ext_decl)| {
                        decl_binding(EntityKind::Value, module_path, &ext_decl.name, ext_decl.vis, &ext_decl.span)
                    })
                    .collect(),
            )
        }
        ASTItem::RecordDecl(node) => {
            spec_rule!("Bind-Record");
            single(EntityKind::Type, &node.name, node.vis, &node.span)
        }
        ASTItem::EnumDecl(node) => {
            spec_rule!("Bind-Enum");
            single(EntityKind::Type, &node.name, node.vis, &node.span)
        }
        ASTItem::ModalDecl(node) => single(EntityKind::Type, &node.name, node.vis, &node.span),
        ASTItem::ClassDecl(node) => {
            spec_rule!("Bind-Class");
            single(EntityKind::Class, &node.name, node.vis, &node.span)
        }
        ASTItem::TypeAliasDecl(node) => {
            spec_rule!("Bind-TypeAlias");
            single(EntityKind::Type, &node.name, node.vis, &node.span)
        }
        ASTItem::StaticDecl(node) => {
            spec_rule!("Bind-Static");
            BindingsResult::bound(
                pat_names_ptr(&node.binding.pat)
                    .iter()
                    .map(|name| decl_binding(EntityKind::Value, module_path, name, node.vis, &node.span))
                    .collect(),
            )
        }
        ASTItem::ErrorItem(_) => {
            spec_rule!("Bind-ErrorItem");
            BindingsResult::bound(Vec::new())
        }
    }
}

fn collect_failed(diag_id: Option<&'static str>, span: Option<Span>) -> CollectNamesResult {
    CollectNamesResult { ok: false, diag_id, span, ..Default::default() }
}

/// One pass over a module's items, against the other modules' name maps as known so far.
pub fn collect_names(
    ctx: &ScopeContext<'_>,
    name_maps: &NameMapTable,
    module_names: &ModuleNames,
    module: &ASTModule,
) -> CollectNamesResult {
    let mut names = NameMap::new();
    let mut name_spans: HashMap<IdKey, Option<Span>> = HashMap::new();
    let comptime_bindings = comptime_procedure_bindings(module);
    if !no_dup(&comptime_bindings) {
        spec_rule!("Collect-Dup");
        return collect_failed(Some("Collect-Dup"), None);
    }
    insert_bindings(&mut names, &comptime_bindings);
    for binding in &comptime_bindings {
        name_spans.entry(binding.name.clone()).or_insert_with(|| binding.span.clone());
    }
    for item in &module.items {
        let bindings = item_bindings(ctx, name_maps, module_names, item, &module.path);
        if !bindings.ok {
            spec_rule!("Collect-Err");
            return collect_failed(bindings.diag_id, bindings.span);
        }
        let merges_free_procedure_overload = bindings_are_single_overload_procedure(item, &bindings.bindings)
            && free_procedure_overload_name(item).is_some_and(|name| names.contains_key(&name));
        if (!disjoint_names(&bindings.bindings, &names) || !no_dup(&bindings.bindings))
            && !merges_free_procedure_overload
        {
            let span = Some(item_span(item).clone());
            let is_main = matches!(item, ASTItem::ProcedureDecl(proc) if id_eq(&proc.name, "main"));
            if is_main && names.contains_key(&id_key_of("main")) {
                spec_rule!("Main-Multiple");
                spec_rule!("rule.15.Main-Multiple");
                spec_rule!("def.15.MainDiagRefs");
                return collect_failed(Some("E-MOD-2430"), span);
            }
            if using_import_conflict(&bindings.bindings, &names) {
                spec_rule!("Collect-Using-Import-Dup");
                return collect_failed(Some("Import-Using-Name-Conflict"), span);
            }
            spec_rule!("Collect-Dup");
            return collect_failed(Some("Collect-Dup"), span);
        }
        spec_rule!("Collect-Scan");
        for binding in &bindings.bindings {
            name_spans.entry(binding.name.clone()).or_insert_with(|| binding.span.clone());
        }
        if !merges_free_procedure_overload {
            insert_bindings(&mut names, &bindings.bindings);
        }
    }
    spec_rule!("Collect-Ok");
    CollectNamesResult { ok: true, diag_id: None, span: None, names, name_spans }
}

/// Name maps for every module. `using` makes one module's names depend on another's, so
/// the collection repeats until nothing changes; a module that fails keeps its last map.
pub fn collect_name_maps(ctx: &mut ScopeContext<'_>) -> NameMapBuildResult {
    let mut result = NameMapBuildResult::default();
    let saved_module = ctx.current_module.clone();
    let module_names = module_names_from_context(ctx);
    let mut current = decl_name_maps(ctx);
    let mut last_results: BTreeMap<PathKey, CollectNamesResult> = BTreeMap::new();
    loop {
        last_results.clear();
        let mut next = current.clone();
        for index in 0..ctx.sigma.mods.len() {
            ctx.current_module = ctx.sigma.mods[index].path.clone();
            let module = &ctx.sigma.mods[index];
            let collected = collect_names(ctx, &current, &module_names, module);
            let key = path_key_of(&module.path);
            if collected.ok && next.get(&key).is_none_or(|existing| !name_map_equals(existing, &collected.names)) {
                next.insert(key.clone(), collected.names.clone());
            }
            last_results.entry(key).or_insert(collected);
        }
        let changed = !name_map_table_equals(&current, &next);
        current = next;
        if !changed {
            break;
        }
    }
    for module in &ctx.sigma.mods {
        if let Some(collected) = last_results.get(&path_key_of(&module.path)) {
            if let (false, Some(diag_id)) = (collected.ok, collected.diag_id) {
                emit_collect_diag(&mut result.diags, diag_id, &collected.span);
            }
        }
    }
    ctx.current_module = saved_module;
    result.name_maps = current;
    result
}
