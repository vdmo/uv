//! What a `using` declaration brings into its module.

use std::collections::BTreeSet;

use uv_core::span::Span;
use uv_source::ast::*;
use uv_source::module_paths::{resolve_import_module_path, ModuleNames};

use super::collect_toplevel::{import_ok, BindingList, BoundName};
use super::scopes::*;
use super::scopes_lookup::find_context_module_by_path;
use super::visibility::{can_access, name_matches, pattern_ptr_binds_name, using_clause_binds_name};
use crate::context::*;

#[derive(Debug, Clone, Default)]
pub struct ResolveUsingPathResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub module_path: Vec<String>,
    pub item: Option<Identifier>,
}

#[derive(Debug, Clone, Default)]
pub struct ItemOfPathResult {
    pub ok: bool,
    pub module_path: Vec<String>,
    pub name: Identifier,
}

#[derive(Debug, Clone, Default)]
pub struct UsingNamesResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub bindings: BindingList,
}

fn failed(diag_id: Option<&'static str>, decl: &UsingDecl) -> UsingNamesResult {
    UsingNamesResult { ok: false, diag_id, span: Some(decl.span.clone()), bindings: Vec::new() }
}

fn using_binding(bind_name: &str, kind: EntityKind, module_path: &[String], target: &str, decl: &UsingDecl) -> BoundName {
    let mut ent = Entity::new(kind, Some(module_path.to_vec()), Some(target.to_string()), EntitySource::Using);
    ent.visibility = Some(decl.vis);
    BoundName { name: id_key_of(bind_name), ent, span: Some(decl.span.clone()) }
}

fn resolve_visible_module_path(ctx: &ScopeContext<'_>, module_names: &ModuleNames, path: &[String]) -> Option<Vec<String>> {
    resolve_import_module_path(&ctx.current_module, module_names, path)
}

fn is_item_kind(kind: EntityKind) -> bool {
    matches!(kind, EntityKind::Value | EntityKind::Type | EntityKind::Class)
}

/// The visibility of the declaration a name refers to, for re-export checks. Compile-time
/// procedures and derive targets cannot be re-exported, so they have none here.
fn decl_visibility(item: &ASTItem, name: &str) -> Option<Visibility> {
    let key = id_key_of(name);
    let named = |candidate: &str, vis: Visibility| name_matches(&key, candidate).then_some(vis);
    match item {
        ASTItem::ExternBlock(it) => it
            .items
            .iter()
            .map(|ExternItem::ExternProcDecl(proc)| proc)
            .find(|proc| proc.name == name)
            .map(|proc| proc.vis),
        ASTItem::StaticDecl(it) => pattern_ptr_binds_name(&it.binding.pat, &key).then_some(it.vis),
        ASTItem::UsingDecl(it) => using_clause_binds_name(&it.clause, &key).then_some(it.vis),
        ASTItem::ProcedureDecl(it) => named(&it.name, it.vis),
        ASTItem::RecordDecl(it) => named(&it.name, it.vis),
        ASTItem::EnumDecl(it) => named(&it.name, it.vis),
        ASTItem::ModalDecl(it) => named(&it.name, it.vis),
        ASTItem::ClassDecl(it) => named(&it.name, it.vis),
        ASTItem::TypeAliasDecl(it) => named(&it.name, it.vis),
        ASTItem::ImportDecl(_)
        | ASTItem::ComptimeProcedureDecl(_)
        | ASTItem::DeriveTargetDecl(_)
        | ASTItem::ErrorItem(_) => None,
    }
}

fn find_decl_visibility(ctx: &ScopeContext<'_>, module_path: &[String], name: &str) -> Option<Visibility> {
    let module = find_context_module_by_path(ctx, module_path)?;
    module.items.iter().find_map(|item| decl_visibility(item, name))
}

fn is_public_item(ctx: &ScopeContext<'_>, module_path: &[String], name: &str) -> bool {
    find_decl_visibility(ctx, module_path, name) == Some(Visibility::Public)
}

fn distinct_using_spec_names(specs: &[UsingSpec]) -> bool {
    let mut seen = BTreeSet::new();
    specs.iter().all(|spec| seen.insert(id_key_of(spec.alias_opt.as_ref().unwrap_or(&spec.name))))
}

pub fn resolve_using_path(
    ctx: &ScopeContext<'_>,
    name_maps: &NameMapTable,
    module_names: &ModuleNames,
    path: &[String],
) -> ResolveUsingPathResult {
    let item = item_of_path(ctx, name_maps, module_names, path);
    if item.ok {
        return ResolveUsingPathResult { ok: true, diag_id: None, module_path: item.module_path, item: Some(item.name) };
    }
    ResolveUsingPathResult { ok: false, diag_id: Some("Resolve-Using-None"), ..Default::default() }
}

/// Splits `a::b::name` into a module that exists and an item its name map has.
pub fn item_of_path(
    ctx: &ScopeContext<'_>,
    name_maps: &NameMapTable,
    module_names: &ModuleNames,
    path: &[String],
) -> ItemOfPathResult {
    let none = ItemOfPathResult::default();
    let Some((name, module_path)) = path.split_last().filter(|_| path.len() >= 2) else {
        return none;
    };
    let Some(resolved_module) = resolve_visible_module_path(ctx, module_names, module_path) else {
        return none;
    };
    if find_context_module_by_path(ctx, &resolved_module).is_none() {
        return none;
    }
    let is_item = name_maps
        .get(&path_key_of(&resolved_module))
        .and_then(|names| names.get(&id_key_of(name)))
        .is_some_and(|ent| is_item_kind(ent.kind));
    if !is_item {
        return none;
    }
    ItemOfPathResult { ok: true, module_path: resolved_module, name: name.clone() }
}

pub fn using_names(
    ctx: &ScopeContext<'_>,
    name_maps: &NameMapTable,
    module_names: &ModuleNames,
    decl: &UsingDecl,
) -> UsingNamesResult {
    let bound = |bindings| UsingNamesResult { ok: true, diag_id: None, span: None, bindings };
    let none = || failed(Some("Resolve-Using-None"), decl);
    match &decl.clause {
        UsingClause::UsingItem(clause) => {
            let mut raw_path = clause.module_path.clone();
            raw_path.push(clause.name.clone());
            let resolved = resolve_using_path(ctx, name_maps, module_names, &raw_path);
            if !resolved.ok {
                return failed(resolved.diag_id, decl);
            }
            let Some(item) = &resolved.item else {
                return none();
            };
            if !import_ok(ctx, module_names, &ctx.current_module, &resolved.module_path) {
                return failed(Some("Import-Using-Missing"), decl);
            }
            let access = can_access(ctx, &resolved.module_path, item);
            if !access.ok {
                return failed(access.diag_id, decl);
            }
            if decl.vis == Visibility::Public && !is_public_item(ctx, &resolved.module_path, item) {
                return failed(Some("Using-Path-Item-Public-Err"), decl);
            }
            let Some(ent) = name_maps.get(&path_key_of(&resolved.module_path)).and_then(|m| m.get(&id_key_of(item)))
            else {
                return none();
            };
            if !is_item_kind(ent.kind) {
                return none();
            }
            let bind_name = clause.alias_opt.as_ref().unwrap_or(item);
            bound(vec![using_binding(bind_name, ent.kind, &resolved.module_path, item, decl)])
        }
        UsingClause::UsingWildcard(clause) => {
            let Some(module_path) = resolve_visible_module_path(ctx, module_names, &clause.module_path) else {
                return none();
            };
            if !import_ok(ctx, module_names, &ctx.current_module, &module_path) {
                return failed(Some("Import-Using-Missing"), decl);
            }
            let Some(names) = name_maps.get(&path_key_of(&module_path)) else {
                return none();
            };
            let items: Vec<(&IdKey, EntityKind)> = names
                .iter()
                .filter(|(name, ent)| is_item_kind(ent.kind) && can_access(ctx, &module_path, name).ok)
                .map(|(name, ent)| (name, ent.kind))
                .collect();
            if decl.vis == Visibility::Public && !items.iter().all(|(name, _)| is_public_item(ctx, &module_path, name)) {
                return failed(Some("Using-List-Public-Err"), decl);
            }
            bound(items.iter().map(|(name, kind)| using_binding(name, *kind, &module_path, name, decl)).collect())
        }
        UsingClause::UsingList(clause) => {
            let Some(module_path) = resolve_visible_module_path(ctx, module_names, &clause.module_path) else {
                return none();
            };
            if !import_ok(ctx, module_names, &ctx.current_module, &module_path) {
                return failed(Some("Import-Using-Missing"), decl);
            }
            if !distinct_using_spec_names(&clause.specs) {
                return failed(Some("Using-List-Dup"), decl);
            }
            let Some(names) = name_maps.get(&path_key_of(&module_path)) else {
                return none();
            };
            let mut bindings = BindingList::new();
            for spec in &clause.specs {
                let Some(ent) = names.get(&id_key_of(&spec.name)).filter(|ent| is_item_kind(ent.kind)) else {
                    return none();
                };
                let access = can_access(ctx, &module_path, &spec.name);
                if !access.ok {
                    return failed(access.diag_id, decl);
                }
                if decl.vis == Visibility::Public && !is_public_item(ctx, &module_path, &spec.name) {
                    return failed(Some("Using-List-Public-Err"), decl);
                }
                let bind_name = spec.alias_opt.as_ref().unwrap_or(&spec.name);
                bindings.push(using_binding(bind_name, ent.kind, &module_path, &spec.name, decl));
            }
            bound(bindings)
        }
    }
}
