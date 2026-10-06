//! Expanding type aliases at the top of a type, as checking against an expected type
//! and indexing do. A one-segment path is looked up through the scopes only.

use uv_source::ast;

use super::type_lower::lower_type;
use super::types::*;
use crate::context::{ScopeContext, TypeDecl};
use crate::generics::monomorphize::{build_substitution, instantiate_type};
use crate::resolve::scopes::path_key_of;
use crate::resolve::scopes_lookup::resolve_type_name;

fn lookup_type_alias_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::TypeAliasDecl> {
    let decl = match path {
        [] => return None,
        [name] => {
            let entity = resolve_type_name(ctx, name)?;
            let mut resolved = entity.origin_opt?;
            resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
            ctx.sigma.types.get(&path_key_of(&resolved))?
        }
        _ => ctx.sigma.types.get(&path_key_of(path))?,
    };
    match decl {
        TypeDecl::TypeAlias(alias) => Some(alias),
        _ => None,
    }
}

/// The alias's definition for the arguments; nothing when the path is no alias or does
/// not take them.
pub(crate) fn expand_type_alias_apply(ctx: &ScopeContext<'_>, path: &[String], args: &[TypeRef]) -> Result<TypeRef, Option<&'static str>> {
    let Some(alias) = lookup_type_alias_decl(ctx, path) else {
        return Ok(None);
    };
    let lowered = lower_type(ctx, &alias.r#type)?;
    Ok(match &alias.generic_params {
        None if args.is_empty() => lowered,
        None => None,
        Some(params) if args.len() > params.params.len() => None,
        Some(params) => instantiate_type(&lowered, &build_substitution(&params.params, args)),
    })
}

fn normalize(ctx: &ScopeContext<'_>, ty: &TypeRef, applications_too: bool) -> Result<TypeRef, Option<&'static str>> {
    let mut current = ty.clone();
    for _ in 0..16 {
        let Some(node) = current.as_deref().map(|ty| &ty.node) else {
            break;
        };
        let (path, args) = match node {
            TypeNode::Path { path, generic_args } => (path, generic_args),
            TypeNode::Apply { path, args } if applications_too => (path, args),
            _ => break,
        };
        let expanded = expand_type_alias_apply(ctx, path, args)?;
        if expanded.is_none() {
            break;
        }
        current = expanded;
    }
    Ok(current)
}

/// The type with aliases at its top expanded, at most sixteen times.
pub fn normalize_alias_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Option<&'static str>> {
    normalize(ctx, ty, true)
}

/// The same for the base of an index or a coercion, where only plain paths are followed.
pub fn normalize_index_base_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Option<&'static str>> {
    normalize(ctx, ty, false)
}
