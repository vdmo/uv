//! Looking names up: unqualified through the scope list, qualified through module paths
//! and the per-module name maps.

use std::collections::{BTreeMap, HashSet};

use uv_core::spec_rule;
use uv_core::symbols::string_of_path;
use uv_project::project::Project;
use uv_source::ast::*;
use uv_source::module_paths::{resolve_import_module_path, ModuleNames};

use super::scopes::*;
use crate::caps::cap_concurrency::is_gpu_intrinsic_name;
use crate::context::*;
use crate::modal::builtin_modal_intrinsics::{is_builtin_modal_member_name, is_builtin_modal_type_path};

pub type AliasMap = BTreeMap<IdKey, Vec<String>>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AccessResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
}

pub type CanAccessFn = fn(&ScopeContext<'_>, &[String], &str) -> AccessResult;

fn resolve_in_current_assembly(ctx: &ScopeContext<'_>, path: &[String], module_names: &ModuleNames) -> Option<Vec<String>> {
    if path.is_empty() {
        return None;
    }
    let assembly = ctx.current_module.first()?;
    let mut candidate = Vec::with_capacity(path.len() + 1);
    candidate.push(assembly.clone());
    candidate.extend(path.iter().cloned());
    module_names.contains(&string_of_path(&candidate)).then_some(candidate)
}

fn resolve_builtin_value_name(name: &str) -> Option<Entity> {
    is_gpu_intrinsic_name(name)
        .then(|| Entity::new(EntityKind::Value, None, Some(name.to_string()), EntitySource::Decl))
}

fn alias_expand(path: &[String], alias: &AliasMap) -> Vec<String> {
    let Some(head) = path.first() else {
        return Vec::new();
    };
    match alias.get(&id_key_of(head)) {
        None => {
            spec_rule!("AliasExpand-None");
            path.to_vec()
        }
        Some(target) => {
            spec_rule!("AliasExpand-Yes");
            let mut out = target.clone();
            out.extend(path[1..].iter().cloned());
            out
        }
    }
}

fn alias_map_for_current_module(ctx: &ScopeContext<'_>, name_maps: &NameMapTable) -> AliasMap {
    name_maps.get(&path_key_of(&ctx.current_module)).map(alias_map_of).unwrap_or_default()
}

fn visible_module_names_from_sigma(ctx: &ScopeContext<'_>, visible_assemblies: &HashSet<String>) -> ModuleNames {
    ctx.sigma
        .mods
        .iter()
        .filter(|module| module.path.first().is_some_and(|assembly| visible_assemblies.contains(assembly)))
        .map(|module| string_of_path(&module.path))
        .collect()
}

/// The modules a module may name: those of its own assembly and of every assembly it
/// imports from.
fn compute_visible_module_names(ctx: &ScopeContext<'_>, all_module_names: &ModuleNames) -> ModuleNames {
    let Some(current_assembly) = ctx.current_module.first() else {
        return all_module_names.clone();
    };
    let mut visible_assemblies: HashSet<String> = HashSet::from([current_assembly.clone()]);
    let Some(current_module) = find_context_module_by_path(ctx, &ctx.current_module) else {
        return all_module_names.clone();
    };
    for item in &current_module.items {
        let ASTItem::ImportDecl(import_decl) = item else {
            continue;
        };
        if let Some(assembly) = resolve_import_module_path(&ctx.current_module, all_module_names, &import_decl.path)
            .and_then(|resolved| resolved.into_iter().next())
        {
            visible_assemblies.insert(assembly);
        }
    }
    let Some(project) = ctx.project else {
        return visible_module_names_from_sigma(ctx, &visible_assemblies);
    };
    let visible: ModuleNames = project
        .modules
        .iter()
        .filter(|module| {
            let assembly_name = module.path.split("::").next().unwrap_or("");
            !assembly_name.is_empty() && visible_assemblies.contains(assembly_name)
        })
        .map(|module| module.path.clone())
        .collect();
    if !visible.is_empty() {
        return visible;
    }
    visible_module_names_from_sigma(ctx, &visible_assemblies)
}

fn precomputed_visible_module_names<'a>(ctx: &ScopeContext<'a>, module_names: &ModuleNames) -> Option<&'a ModuleNames> {
    let tables = ctx.name_resolution_tables.as_ref()?;
    if !tables.module_names.is_some_and(|known| std::ptr::eq(known, module_names)) {
        return None;
    }
    tables.visible_module_names?.get(&path_key_of(&ctx.current_module))
}

pub fn alias_map_of(names: &NameMap) -> AliasMap {
    names
        .iter()
        .filter(|(_, ent)| ent.kind == EntityKind::ModuleAlias)
        .filter_map(|(key, ent)| ent.origin_opt.clone().map(|origin| (key.clone(), origin)))
        .collect()
}

pub fn module_names_of(project: &Project) -> ModuleNames {
    project.modules.iter().map(|module| module.path.clone()).collect()
}

pub fn find_context_module_by_path<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ASTModule> {
    ctx.sigma.mods.iter().find(|module| path_eq(&module.path, path))
}

pub fn visible_module_names_of(ctx: &ScopeContext<'_>, all_module_names: &ModuleNames) -> ModuleNames {
    compute_visible_module_names(ctx, all_module_names)
}

pub fn region_alias(ent: &Entity) -> bool {
    ent.source == EntitySource::RegionAlias
}

/// The innermost binding of a name.
pub fn lookup(ctx: &ScopeContext<'_>, name: &str) -> Option<Entity> {
    let key = id_key_of(name);
    match ctx.scopes.iter().find_map(|scope| scope.get(&key)) {
        Some(ent) => {
            spec_rule!("Lookup-Unqualified");
            Some(ent.clone())
        }
        None => {
            spec_rule!("Lookup-Unqualified-None");
            None
        }
    }
}

fn lookup_of_kind(ctx: &ScopeContext<'_>, name: &str, kind: EntityKind, rule: &'static str) -> Option<Entity> {
    let ent = lookup(ctx, name).filter(|ent| ent.kind == kind)?;
    uv_core::spec_trace::Conformance::record_if_enabled(rule);
    Some(ent)
}

/// A value binding; `~` names the receiver, and the GPU intrinsics are always values.
pub fn resolve_value_name(ctx: &ScopeContext<'_>, name: &str) -> Option<Entity> {
    let lookup_name = if name == "~" { "self" } else { name };
    lookup_of_kind(ctx, lookup_name, EntityKind::Value, "Resolve-Value-Name")
        .or_else(|| resolve_builtin_value_name(lookup_name))
}

pub fn resolve_type_name(ctx: &ScopeContext<'_>, name: &str) -> Option<Entity> {
    lookup_of_kind(ctx, name, EntityKind::Type, "Resolve-Type-Name")
}

pub fn resolve_class_name(ctx: &ScopeContext<'_>, name: &str) -> Option<Entity> {
    lookup_of_kind(ctx, name, EntityKind::Class, "Resolve-Class-Name")
}

pub fn resolve_module_name(ctx: &ScopeContext<'_>, name: &str) -> Option<Entity> {
    lookup_of_kind(ctx, name, EntityKind::ModuleAlias, "Resolve-Module-Name")
}

pub fn region_alias_name(ctx: &ScopeContext<'_>, name: &str) -> bool {
    resolve_value_name(ctx, name).is_some_and(|ent| region_alias(&ent))
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolveModulePathResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub path: Option<Vec<String>>,
}

/// Resolves a written module path: aliases are expanded, then the path is tried as
/// written and relative to the current assembly.
pub fn resolve_module_path(
    ctx: &ScopeContext<'_>,
    path: &[String],
    alias: &AliasMap,
    module_names: &ModuleNames,
) -> ResolveModulePathResult {
    let computed;
    let visible_module_names = match precomputed_visible_module_names(ctx, module_names) {
        Some(precomputed) => precomputed,
        None => {
            computed = visible_module_names_of(ctx, module_names);
            &computed
        }
    };
    let expanded = alias_expand(path, alias);
    if visible_module_names.contains(&string_of_path(&expanded)) {
        spec_rule!("Resolve-ModulePath-Direct");
        return ResolveModulePathResult { ok: true, diag_id: None, path: Some(expanded) };
    }
    if let Some(local) = resolve_in_current_assembly(ctx, &expanded, visible_module_names) {
        spec_rule!("Resolve-ModulePath-Current");
        return ResolveModulePathResult { ok: true, diag_id: None, path: Some(local) };
    }
    spec_rule!("ResolveModulePath-Err");
    ResolveModulePathResult { ok: false, diag_id: Some("ResolveModulePath-Err"), path: None }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolveQualifiedResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub entity: Option<Entity>,
}

/// Resolves `path::name` to an entity of the wanted kind that the current module may access.
pub fn resolve_qualified(
    ctx: &ScopeContext<'_>,
    name_maps: &NameMapTable,
    module_names: &ModuleNames,
    path: &[String],
    name: &str,
    kind: EntityKind,
    can_access: Option<CanAccessFn>,
) -> ResolveQualifiedResult {
    let failed = |diag_id| ResolveQualifiedResult { ok: false, diag_id, entity: None };
    if kind == EntityKind::Value
        && path.len() == 1
        && is_builtin_modal_type_path(path)
        && is_builtin_modal_member_name(path, name)
    {
        spec_rule!("Resolve-Qualified");
        let ent = Entity::new(EntityKind::Value, Some(vec![path[0].clone()]), Some(name.to_string()), EntitySource::Decl);
        return ResolveQualifiedResult { ok: true, diag_id: None, entity: Some(ent) };
    }
    let alias = alias_map_for_current_module(ctx, name_maps);
    let module_result = resolve_module_path(ctx, path, &alias, module_names);
    let Some(module_path) = module_result.path.filter(|_| module_result.ok) else {
        return failed(module_result.diag_id);
    };
    let Some(entity) = name_maps
        .get(&path_key_of(&module_path))
        .and_then(|names| names.get(&id_key_of(name)))
        .filter(|ent| ent.kind == kind)
    else {
        return failed(None);
    };
    if let Some(can_access) = can_access {
        let access = can_access(ctx, &module_path, name);
        if !access.ok {
            return failed(access.diag_id);
        }
    }
    spec_rule!("Resolve-Qualified");
    ResolveQualifiedResult { ok: true, diag_id: None, entity: Some(entity.clone()) }
}
