//! Which classes are capabilities.

use super::builtin_paths::{is_capability_class_path, is_context_type_path};
use uv_source::ast;
use crate::context::{PathKey, ScopeContext};
use crate::resolve::scopes::path_key_of;

fn is_capability_class_from(ctx: &ScopeContext<'_>, path: &[String], visiting: &mut Vec<PathKey>) -> bool {
    if is_capability_class_path(path) {
        return true;
    }
    let key = path_key_of(path);
    // A superclass cycle is reported elsewhere; here it is simply not a capability.
    if visiting.contains(&key) {
        return false;
    }
    let Some(decl) = ctx.sigma.classes.get(&key) else {
        return false;
    };
    visiting.push(key);
    let found = decl.supers.iter().any(|sup| is_capability_class_from(ctx, sup, visiting));
    visiting.pop();
    found
}

/// A built-in capability class, or a class whose superclass chain reaches one.
pub fn is_capability_class(ctx: &ScopeContext<'_>, path: &[String]) -> bool {
    is_capability_class_from(ctx, path, &mut Vec::new())
}

// ---- the context bundle: a record of the capabilities a program starts with ----

fn strip_ast_perm_and_refine(ty: &ast::TypePtr) -> Option<&ast::Type> {
    let mut current = ty.as_deref()?;
    loop {
        match &current.node {
            ast::TypeNode::TypePermType(node) => current = node.base.as_deref()?,
            ast::TypeNode::TypeRefine(node) => current = node.base.as_deref()?,
            _ => return Some(current),
        }
    }
}

fn matches_dynamic_builtin_type(ty: &ast::TypePtr, builtin_name: &str) -> bool {
    matches!(strip_ast_perm_and_refine(ty).map(|ty| &ty.node),
        Some(ast::TypeNode::TypeDynamic(dynamic)) if super::builtin_paths::path_matches_builtin_name(&dynamic.path, builtin_name))
}

/// Whether a field of a context bundle has the type its name calls for.
fn matches_context_bundle_field_type(ty: &ast::TypePtr, field_name: &str) -> bool {
    let wanted = [
        ("io", "IO"),
        ("net", "Network"),
        ("heap", "HeapAllocator"),
        ("sys", "System"),
        ("reactor", "Reactor"),
        ("time", "Time"),
        ("cpu", "ExecutionDomain"),
        ("gpu", "ExecutionDomain"),
        ("inline", "ExecutionDomain"),
    ]
    .into_iter()
    .find(|(name, _)| crate::resolve::scopes::id_eq(field_name, name));
    wanted.is_some_and(|(_, builtin)| matches_dynamic_builtin_type(ty, builtin))
}

/// The declaration a type path names, as written or through the names in scope.
fn resolve_visible_type_decl<'c>(ctx: &'c crate::context::ScopeContext<'_>, path: &[String]) -> Option<(Vec<String>, &'c crate::context::TypeDecl)> {
    use crate::resolve::scopes::path_key_of;
    if path.is_empty() {
        return None;
    }
    if let Some(decl) = ctx.sigma.types.get(&path_key_of(path)) {
        return Some((path.to_vec(), decl));
    }
    let [only] = path else {
        return None;
    };
    let entity = crate::resolve::scopes_lookup::resolve_type_name(ctx, only)?;
    let mut resolved = entity.origin_opt?;
    resolved.push(entity.target_opt.unwrap_or_else(|| only.clone()));
    let decl = ctx.sigma.types.get(&path_key_of(&resolved))?;
    Some((resolved, decl))
}

fn is_context_bundle_type_impl(ctx: &crate::context::ScopeContext<'_>, ty: &ast::TypePtr, visiting: &mut std::collections::HashSet<String>) -> bool {
    use crate::context::TypeDecl;
    let Some(ast::TypeNode::TypePathType(path)) = strip_ast_perm_and_refine(ty).map(|ty| &ty.node) else {
        return false;
    };
    if !path.generic_args.is_empty() {
        return false;
    }
    if is_context_type_path(&path.path) {
        return true;
    }
    let Some((resolved, decl)) = resolve_visible_type_decl(ctx, &path.path) else {
        return false;
    };
    let visit_key = resolved.join("::");
    if !visiting.insert(visit_key.clone()) {
        return false;
    }
    let ok = match decl {
        TypeDecl::TypeAlias(alias) => alias.generic_params.as_ref().is_none_or(|params| params.params.is_empty()) && is_context_bundle_type_impl(ctx, &alias.r#type, visiting),
        TypeDecl::Record(record) => record.members.iter().all(|member| match member {
            ast::RecordMember::FieldDecl(field) => {
                field.r#type.is_some() && (matches_context_bundle_field_type(&field.r#type, &field.name) || is_context_bundle_type_impl(ctx, &field.r#type, visiting))
            }
            _ => true,
        }),
        _ => false,
    };
    visiting.remove(&visit_key);
    ok
}

/// Whether the type is `Context`, or a record of capabilities (possibly behind aliases).
pub fn is_context_bundle_type(ctx: &crate::context::ScopeContext<'_>, ty: &ast::TypePtr) -> bool {
    is_context_bundle_type_impl(ctx, ty, &mut std::collections::HashSet::new())
}

fn is_alias_normalized_context_type_impl(ctx: &crate::context::ScopeContext<'_>, ty: &ast::TypePtr, visiting: &mut std::collections::HashSet<String>) -> bool {
    use crate::context::TypeDecl;
    let Some(ast::TypeNode::TypePathType(path)) = strip_ast_perm_and_refine(ty).map(|ty| &ty.node) else {
        return false;
    };
    if !path.generic_args.is_empty() {
        return false;
    }
    if is_context_type_path(&path.path) {
        return true;
    }
    let Some((resolved, decl)) = resolve_visible_type_decl(ctx, &path.path) else {
        return false;
    };
    let visit_key = resolved.join("::");
    if !visiting.insert(visit_key.clone()) {
        return false;
    }
    let is_context = match decl {
        TypeDecl::TypeAlias(alias) => {
            alias.generic_params.as_ref().is_none_or(|params| params.params.is_empty()) && alias.r#type.is_some() && is_alias_normalized_context_type_impl(ctx, &alias.r#type, visiting)
        }
        _ => false,
    };
    visiting.remove(&visit_key);
    is_context
}

/// A context bundle that is not `Context` itself (even behind aliases): the host supplies it.
pub fn is_hosted_context_bundle_type(ctx: &crate::context::ScopeContext<'_>, ty: &ast::TypePtr) -> bool {
    !is_alias_normalized_context_type_impl(ctx, ty, &mut std::collections::HashSet::new()) && is_context_bundle_type(ctx, ty)
}
