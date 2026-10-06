//! `using` and `import` declarations: what they name must exist and be visible.

use uv_project::language_profile::active_language_profile;
use uv_source::ast::{self, ASTItem, PatternNode, Visibility};
use uv_source::attributes::validate_unsupported_attribute_target;
use uv_source::module_paths::{resolve_import_module_path, ModuleNames};

use crate::context::{ScopeContext, Sigma};
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::resolve::scopes_lookup::module_names_of;

type Diag = Option<&'static str>;

fn is_reserved_module_path(path: &[String]) -> bool {
    path.first().is_some_and(|head| id_eq(head, active_language_profile().runtime_root))
}

fn find_module<'s>(sigma: &'s Sigma, path: &[String]) -> Option<&'s ast::ASTModule> {
    sigma.mods.iter().find(|module| module.path == path)
}

/// A module path as written, or under the current module's assembly.
fn resolve_local_module_path(sigma: &Sigma, path: &[String], current_module: &[String]) -> Vec<String> {
    if find_module(sigma, path).is_some() || path.is_empty() || current_module.is_empty() {
        return path.to_vec();
    }
    let candidate: Vec<String> = current_module.iter().take(1).chain(path).cloned().collect();
    if find_module(sigma, &candidate).is_some() {
        candidate
    } else {
        path.to_vec()
    }
}

fn same_assembly(lhs: &[String], rhs: &[String]) -> bool {
    matches!((lhs.first(), rhs.first()), (Some(lhs), Some(rhs)) if lhs == rhs)
}

/// A module of another assembly is visible only through an `import` that covers it.
pub fn is_module_visible(ctx: &ScopeContext<'_>, module_path: &[String], from_module: &[String]) -> bool {
    if is_reserved_module_path(module_path) {
        return true;
    }
    let resolved = resolve_local_module_path(&ctx.sigma, module_path, from_module);
    if find_module(&ctx.sigma, &resolved).is_none() {
        return false;
    }
    if from_module.is_empty() || resolved.is_empty() || same_assembly(from_module, &resolved) {
        return true;
    }
    let Some(from_decl) = find_module(&ctx.sigma, from_module) else {
        return false;
    };
    from_decl.items.iter().any(|item| match item {
        ASTItem::ImportDecl(import) => resolved.starts_with(&resolve_local_module_path(&ctx.sigma, &import.path, from_module)),
        _ => false,
    })
}

/// The name a static declaration binds, when its pattern is a plain one.
fn static_name(decl: &ast::StaticDecl, typed_underscore_binds: bool) -> Option<&str> {
    match decl.binding.pat.as_deref().map(|pat| &pat.node) {
        Some(PatternNode::IdentifierPattern(pat)) if typed_underscore_binds || pat.name != "_" => Some(&pat.name),
        Some(PatternNode::TypedPattern(pat)) if pat.name != "_" => Some(&pat.name),
        _ => None,
    }
}

fn extern_procs(block: &ast::ExternBlock) -> impl Iterator<Item = &ast::ExternProcDecl> {
    block.items.iter().map(|ast::ExternItem::ExternProcDecl(proc)| proc)
}

fn is_value_item(sigma: &Sigma, module_path: &[String], item_name: &str) -> bool {
    let Some(module) = find_module(sigma, module_path) else {
        return false;
    };
    module.items.iter().any(|item| match item {
        ASTItem::ProcedureDecl(node) => node.name == item_name,
        ASTItem::ExternBlock(node) => extern_procs(node).any(|proc| proc.name == item_name),
        ASTItem::StaticDecl(node) => static_name(node, true) == Some(item_name),
        _ => false,
    })
}

pub fn is_item_visible(ctx: &ScopeContext<'_>, item_path: &[String], from_module: &[String]) -> bool {
    let Some((item_name, module_path)) = item_path.split_last() else {
        return false;
    };
    let module_path = resolve_local_module_path(&ctx.sigma, module_path, from_module);
    let Some(found_module) = find_module(&ctx.sigma, &module_path) else {
        return is_reserved_module_path(&module_path);
    };
    for item in &found_module.items {
        let vis = match item {
            ASTItem::ProcedureDecl(node) if node.name == *item_name => Some(node.vis),
            ASTItem::ExternBlock(node) => extern_procs(node).find(|proc| proc.name == *item_name).map(|proc| proc.vis),
            ASTItem::StaticDecl(node) if static_name(node, false) == Some(item_name.as_str()) => Some(node.vis),
            ASTItem::RecordDecl(node) if node.name == *item_name => Some(node.vis),
            ASTItem::EnumDecl(node) if node.name == *item_name => Some(node.vis),
            ASTItem::ModalDecl(node) if node.name == *item_name => Some(node.vis),
            ASTItem::ClassDecl(node) if node.name == *item_name => Some(node.vis),
            ASTItem::TypeAliasDecl(node) if node.name == *item_name => Some(node.vis),
            _ => None,
        };
        if let Some(vis) = vis {
            return is_module_visible(ctx, &module_path, from_module)
                && (vis == Visibility::Public
                    || (vis == Visibility::Internal && same_assembly(&module_path, from_module))
                    || module_path == from_module);
        }
    }
    false
}

fn resolve_using_item(ctx: &ScopeContext<'_>, module_path: &[String], item_name: &str, current_module: &[String]) -> Result<(), Diag> {
    let resolved_module_path = resolve_local_module_path(&ctx.sigma, module_path, current_module);
    let item_path: Vec<String> = resolved_module_path.iter().cloned().chain([item_name.to_string()]).collect();
    let item_key = path_key_of(&item_path);
    let resolved = ctx.sigma.types.contains_key(&item_key)
        || ctx.sigma.classes.contains_key(&item_key)
        || is_value_item(&ctx.sigma, &resolved_module_path, item_name);
    if !resolved {
        return if is_reserved_module_path(&resolved_module_path) { Ok(()) } else { Err(Some("Import-Using-Missing")) };
    }
    if !is_item_visible(ctx, &item_path, current_module) {
        return Err(Some("E-MOD-1207"));
    }
    Ok(())
}

/// See `TypeUsingDecl`. The names a wildcard brings in are not needed to judge it: it
/// fails only when the module is unknown.
pub fn type_using_decl(ctx: &ScopeContext<'_>, decl: &ast::UsingDecl, current_module: &[String]) -> Result<(), Diag> {
    let attr_validation = validate_unsupported_attribute_target(decl.attrs_opt.as_deref().unwrap_or(&[]), "using declarations");
    if !attr_validation.ok {
        return Err(attr_validation.diag_id);
    }
    match &decl.clause {
        ast::UsingClause::UsingItem(clause) => resolve_using_item(ctx, &clause.module_path, &clause.name, current_module),
        ast::UsingClause::UsingList(clause) => {
            clause.specs.iter().try_for_each(|spec| resolve_using_item(ctx, &clause.module_path, &spec.name, current_module))
        }
        ast::UsingClause::UsingWildcard(clause) => {
            let resolved = resolve_local_module_path(&ctx.sigma, &clause.module_path, current_module);
            if find_module(&ctx.sigma, &resolved).is_none() && !is_reserved_module_path(&resolved) {
                return Err(Some("Resolve-Using-None"));
            }
            Ok(())
        }
    }
}

fn module_names_for_context(ctx: &ScopeContext<'_>) -> ModuleNames {
    let names: ModuleNames = ctx.sigma.mods.iter().map(|module| uv_core::symbols::string_of_path(&module.path)).collect();
    match ctx.project {
        Some(project) if names.is_empty() => module_names_of(project),
        _ => names,
    }
}

/// See `TypeImportDecl`.
pub fn type_import_decl(ctx: &ScopeContext<'_>, decl: &ast::ImportDecl, current_module: &[String]) -> Result<(), Diag> {
    let attr_validation = validate_unsupported_attribute_target(decl.attrs_opt.as_deref().unwrap_or(&[]), "import declarations");
    if !attr_validation.ok {
        return Err(attr_validation.diag_id);
    }
    let alias = decl.alias_opt.as_deref().filter(|alias| !alias.is_empty()).or(decl.path.last().map(String::as_str)).unwrap_or("");
    if decl.path.is_empty() || alias.is_empty() {
        return Err(Some("Import-Using-Missing"));
    }
    let resolved_path = resolve_import_module_path(current_module, &module_names_for_context(ctx), &decl.path).ok_or(Some("Resolve-Import-Err"))?;
    if find_module(&ctx.sigma, &resolved_path).is_none() || !is_module_visible(ctx, &resolved_path, current_module) {
        return Err(Some("Resolve-Import-Err"));
    }
    Ok(())
}
