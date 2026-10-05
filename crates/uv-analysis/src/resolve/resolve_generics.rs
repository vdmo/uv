//! Generic parameter lists: the parameters come into scope, then their bounds and
//! defaults are resolved.

use uv_source::ast::*;

use super::resolve_types::{resolve_class_path, resolve_type};
use super::resolver::*;
use super::scopes::id_key_of;
use crate::context::*;

fn type_param_entity(param: &TypeParam) -> Entity {
    let mut entity = Entity::new(EntityKind::Type, None, Some(param.name.clone()), EntitySource::Decl);
    entity.type_param_class_bounds = param.bounds.clone();
    entity
}

/// A scope list with the parameters in a new innermost scope. A repeated name keeps its
/// last declaration.
pub fn bind_type_params(ctx: &ScopeContext<'_>, params: &GenericParams) -> ScopeList {
    let mut scope = Scope::new();
    for param in &params.params {
        scope.insert(id_key_of(&param.name), type_param_entity(param));
    }
    let mut scopes = Vec::with_capacity(ctx.scopes.len() + 1);
    scopes.push(scope);
    scopes.extend(ctx.scopes.iter().cloned());
    scopes
}

fn resolve_type_bound(ctx: &mut ResolveContext<'_, '_>, bound: &TypeBound) -> Res<TypeBound> {
    let class_path = resolve_class_path(ctx, &bound.class_path).map_err(ResError::id_span)?;
    let generic_args = bound
        .generic_args
        .iter()
        .map(|arg| resolve_type(ctx, arg).map_err(ResError::id_span))
        .collect::<Res<Vec<_>>>()?;
    Ok(TypeBound { class_path, generic_args })
}

fn resolve_type_param(ctx: &mut ResolveContext<'_, '_>, param: &TypeParam) -> Res<TypeParam> {
    let bounds = param.bounds.iter().map(|bound| resolve_type_bound(ctx, bound)).collect::<Res<Vec<_>>>()?;
    let default_type = match &param.default_type {
        Some(_) => resolve_type(ctx, &param.default_type).map_err(ResError::id_span)?,
        None => None,
    };
    Ok(TypeParam { bounds, default_type, ..param.clone() })
}

/// The parameters stay in scope after this returns, for the rest of the declaration; the
/// caller's scope override ends their visibility.
pub fn resolve_generic_params_opt(
    ctx: &mut ResolveContext<'_, '_>,
    params_opt: &Option<GenericParams>,
) -> Res<Option<GenericParams>> {
    let Some(params) = params_opt else {
        return Ok(None);
    };
    if params.params.is_empty() {
        return Ok(Some(params.clone()));
    }
    ctx.ctx.scopes = bind_type_params(ctx.ctx, params);
    let resolved = params.params.iter().map(|param| resolve_type_param(ctx, param)).collect::<Res<Vec<_>>>()?;
    Ok(Some(GenericParams { params: resolved, span: params.span.clone() }))
}
