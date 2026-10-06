//! Resolving type and class paths, and the types that contain them.

use std::sync::Arc;

use uv_core::symbols::string_of_path;
use uv_source::ast::*;
use uv_source::parser::make_type_modal_ref;

use super::resolve_expr::resolve_expr;
use super::resolver::*;
use super::scopes::id_eq;
use super::scopes_intro::intro;
use super::scopes_lookup::{resolve_class_name, resolve_qualified, resolve_type_name};
use crate::caps::builtin_paths::*;
use crate::context::*;

fn is_foundational_class_path(path: &[String]) -> bool {
    matches!(path, [only] if ["Drop", "Bitcopy", "Clone", "Eq", "Hasher", "Hash", "Iterator", "Discrete", "FfiSafe"]
        .iter()
        .any(|name| id_eq(only, name)))
}

fn is_self_associated_type_path(path: &[String]) -> bool {
    path.len() == 2 && id_eq(&path[0], "Self")
}

fn is_gpu_ptr_path(path: &[String]) -> bool {
    matches!(path, [only] if id_eq(only, "GpuPtr"))
}

/// `GpuPtr<T, Global>`: the second argument names an address space, not a type.
fn is_gpu_ptr_address_space_arg(ty: &TypePtr) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::TypePathType(path)) if path.generic_args.is_empty() => {
            matches!(&path.path[..], [only] if ["Global", "Shared", "Private"].iter().any(|name| id_eq(only, name)))
        }
        _ => false,
    }
}

fn unresolved_type_path(diag_id: Option<&'static str>, path: &[String]) -> ResError {
    ResError::detailed(diag_id, None, format!("unresolved type path `{}`", string_of_path(path)))
}

pub fn resolve_type_path(ctx: &mut ResolveContext<'_, '_>, path: &[String]) -> Res<Vec<String>> {
    let Some((name, prefix)) = path.split_last() else {
        return Err(ResError::default());
    };
    if prefix.is_empty() {
        if is_gpu_ptr_path(path)
            || is_io_builtin_type_path(path)
            || is_heap_allocator_builtin_type_path(path)
            || is_time_builtin_type_path(path)
            || is_outcome_type_path(path)
        {
            return Ok(vec![name.clone()]);
        }
        let Some(ent) = resolve_type_name(ctx.ctx, name) else {
            return Err(unresolved_type_path(Some("ResolveExpr-Ident-Err"), path));
        };
        let resolved_name = ent.target_opt.unwrap_or_else(|| name.clone());
        return Ok(match ent.origin_opt {
            Some(origin) => full_path(&origin, &resolved_name),
            None => vec![resolved_name],
        });
    }
    if is_self_associated_type_path(path) {
        return Ok(path.to_vec());
    }
    let qualified =
        resolve_qualified(ctx.ctx, ctx.name_maps, ctx.module_names, prefix, name, EntityKind::Type, ctx.can_access);
    let Some(ent) = qualified.entity.filter(|_| qualified.ok) else {
        return Err(unresolved_type_path(qualified.diag_id, path));
    };
    let Some(origin) = ent.origin_opt else {
        return Err(unresolved_type_path(Some("ResolveExpr-Ident-Err"), path));
    };
    Ok(full_path(&origin, ent.target_opt.as_ref().unwrap_or(name)))
}

/// A class path that names nothing is left as written; the type checker reports it.
pub fn resolve_class_path(ctx: &mut ResolveContext<'_, '_>, path: &[String]) -> Res<Vec<String>> {
    let Some((name, prefix)) = path.split_last() else {
        return Err(ResError::default());
    };
    if prefix.is_empty() {
        if is_class_path_resolved_without_declaration(path) || is_foundational_class_path(path) {
            return Ok(vec![name.clone()]);
        }
        return Ok(match resolve_class_name(ctx.ctx, name) {
            Some(Entity { origin_opt: Some(origin), target_opt, .. }) => {
                full_path(&origin, target_opt.as_ref().unwrap_or(name))
            }
            _ => vec![name.clone()],
        });
    }
    let qualified =
        resolve_qualified(ctx.ctx, ctx.name_maps, ctx.module_names, prefix, name, EntityKind::Class, ctx.can_access);
    let Some(ent) = qualified.entity.filter(|_| qualified.ok) else {
        return Err(ResError::from_id(qualified.diag_id, None));
    };
    let Some(origin) = ent.origin_opt else {
        return Err(ResError::new("ResolveExpr-Ident-Err", None));
    };
    Ok(full_path(&origin, ent.target_opt.as_ref().unwrap_or(name)))
}

fn rebuilt(ty: &Type, node: TypeNode) -> Res<TypePtr> {
    Ok(Some(Arc::new(Type { span: ty.span.clone(), node })))
}

/// Resolves a nested type; a failure inside a type keeps only its rule.
fn nested(ctx: &mut ResolveContext<'_, '_>, ty: &TypePtr) -> Res<TypePtr> {
    resolve_type(ctx, ty).map_err(ResError::id_only)
}

fn nested_list(ctx: &mut ResolveContext<'_, '_>, types: &[TypePtr]) -> Res<Vec<TypePtr>> {
    types.iter().map(|ty| nested(ctx, ty)).collect()
}

fn func_params(ctx: &mut ResolveContext<'_, '_>, params: &[TypeFuncParam]) -> Res<Vec<TypeFuncParam>> {
    params.iter().map(|param| Ok(TypeFuncParam { mode: param.mode, r#type: nested(ctx, &param.r#type)? })).collect()
}

fn nested_opt(ctx: &mut ResolveContext<'_, '_>, ty: &TypePtr) -> Res<TypePtr> {
    match ty {
        Some(_) => nested(ctx, ty),
        None => Ok(None),
    }
}

pub fn resolve_type(ctx: &mut ResolveContext<'_, '_>, type_ptr: &TypePtr) -> Res<TypePtr> {
    let Some(ty) = type_ptr.as_deref() else {
        return Err(ResError::default());
    };
    // A path that does not resolve is reported at the type that names it.
    let named = |ctx: &mut ResolveContext<'_, '_>, path: &[String]| -> Res<Vec<String>> {
        let resolved = resolve_type_path(ctx, path).map_err(|err| err.span_or(&ty.span).no_children())?;
        ctx.record_type_path_reference(&resolved, &ty.span);
        Ok(resolved)
    };
    let class_named = |ctx: &mut ResolveContext<'_, '_>, path: &[String]| -> Res<Vec<String>> {
        let resolved = resolve_class_path(ctx, path).map_err(ResError::id_only)?;
        if let Some((last, origin)) = resolved.split_last() {
            let entity = Entity::new(EntityKind::Class, Some(origin.to_vec()), Some(last.clone()), EntitySource::Decl);
            ctx.record_reference(last, &ty.span, &entity);
        }
        Ok(resolved)
    };
    match &ty.node {
        TypeNode::TypePathType(node) => {
            let path = named(ctx, &node.path)?;
            let generic_args = nested_list(ctx, &node.generic_args)?;
            rebuilt(ty, TypeNode::TypePathType(TypePathType { path, generic_args }))
        }
        TypeNode::TypeApply(node) => {
            let path = named(ctx, &node.path)?;
            let mut generic_args = Vec::with_capacity(node.args.len());
            for (index, arg) in node.args.iter().enumerate() {
                if is_gpu_ptr_path(&path) && index == 1 && is_gpu_ptr_address_space_arg(arg) {
                    generic_args.push(arg.clone());
                } else {
                    generic_args.push(nested(ctx, arg)?);
                }
            }
            rebuilt(ty, TypeNode::TypePathType(TypePathType { path, generic_args }))
        }
        TypeNode::TypeDynamic(node) => {
            let path = class_named(ctx, &node.path)?;
            rebuilt(ty, TypeNode::TypeDynamic(TypeDynamic { path }))
        }
        TypeNode::TypeOpaque(node) => {
            let path = class_named(ctx, &node.path)?;
            rebuilt(ty, TypeNode::TypeOpaque(TypeOpaque { path }))
        }
        TypeNode::TypeModalState(node) => {
            let path = named(ctx, &node.path)?;
            let generic_args = nested_list(ctx, &node.generic_args)?;
            let modal_ref = make_type_modal_ref(path.clone(), generic_args.clone());
            rebuilt(
                ty,
                TypeNode::TypeModalState(TypeModalState { modal_ref, path, generic_args, state: node.state.clone() }),
            )
        }
        TypeNode::TypePermType(node) => {
            let base = nested(ctx, &node.base)?;
            rebuilt(ty, TypeNode::TypePermType(TypePermType { perm: node.perm, base }))
        }
        TypeNode::TypeUnion(node) => {
            let types = nested_list(ctx, &node.types)?;
            rebuilt(ty, TypeNode::TypeUnion(TypeUnion { types }))
        }
        TypeNode::TypeFunc(node) => {
            let params = func_params(ctx, &node.params)?;
            let ret = nested_opt(ctx, &node.ret)?;
            rebuilt(ty, TypeNode::TypeFunc(TypeFunc { params, ret }))
        }
        TypeNode::TypeClosure(node) => {
            let params = func_params(ctx, &node.params)?;
            let ret = nested_opt(ctx, &node.ret)?;
            let deps_opt = match &node.deps_opt {
                Some(deps) => Some(
                    deps.iter()
                        .map(|dep| Ok(SharedDep { name: dep.name.clone(), r#type: nested(ctx, &dep.r#type)? }))
                        .collect::<Res<Vec<_>>>()?,
                ),
                None => None,
            };
            rebuilt(ty, TypeNode::TypeClosure(TypeClosure { params, ret, deps_opt }))
        }
        TypeNode::TypeTuple(node) => {
            let elements = nested_list(ctx, &node.elements)?;
            rebuilt(ty, TypeNode::TypeTuple(TypeTuple { elements }))
        }
        TypeNode::TypeArray(node) => {
            let element = nested(ctx, &node.element)?;
            let length = match &node.length {
                Some(_) => resolve_expr(ctx, &node.length).map_err(ResError::id_only)?,
                None => None,
            };
            rebuilt(ty, TypeNode::TypeArray(TypeArray { element, length }))
        }
        TypeNode::TypeSlice(node) => {
            let element = nested(ctx, &node.element)?;
            rebuilt(ty, TypeNode::TypeSlice(TypeSlice { element }))
        }
        TypeNode::TypeSafePtr(node) => {
            let element = nested(ctx, &node.element)?;
            rebuilt(ty, TypeNode::TypeSafePtr(TypeSafePtr { element, state: node.state }))
        }
        TypeNode::TypeRawPtr(node) => {
            let element = nested(ctx, &node.element)?;
            rebuilt(ty, TypeNode::TypeRawPtr(TypeRawPtr { qual: node.qual, element }))
        }
        TypeNode::TypeRefine(node) => {
            let base = resolve_type(ctx, &node.base).map_err(ResError::id_span)?;
            let predicate = match &node.predicate {
                // The predicate sees the refined value as `self`.
                Some(predicate) => with_scope(ctx, Scope::new(), |ctx| {
                    let introduced = intro(ctx.ctx, "self", &value_entity());
                    if !introduced.ok {
                        return Err(ResError::from_id(introduced.diag_id, Some(predicate.span.clone())));
                    }
                    resolve_expr(ctx, &node.predicate).map_err(ResError::id_span)
                })?,
                None => None,
            };
            rebuilt(ty, TypeNode::TypeRefine(TypeRefine { base, predicate }))
        }
        TypeNode::TypeRange(node) => {
            let base = nested(ctx, &node.base)?;
            rebuilt(ty, TypeNode::TypeRange(TypeRange { base }))
        }
        TypeNode::TypeRangeInclusive(node) => {
            let base = nested(ctx, &node.base)?;
            rebuilt(ty, TypeNode::TypeRangeInclusive(TypeRangeInclusive { base }))
        }
        TypeNode::TypeRangeFrom(node) => {
            let base = nested(ctx, &node.base)?;
            rebuilt(ty, TypeNode::TypeRangeFrom(TypeRangeFrom { base }))
        }
        TypeNode::TypeRangeTo(node) => {
            let base = nested(ctx, &node.base)?;
            rebuilt(ty, TypeNode::TypeRangeTo(TypeRangeTo { base }))
        }
        TypeNode::TypeRangeToInclusive(node) => {
            let base = nested(ctx, &node.base)?;
            rebuilt(ty, TypeNode::TypeRangeToInclusive(TypeRangeToInclusive { base }))
        }
        TypeNode::TypePrim(_)
        | TypeNode::TypeString(_)
        | TypeNode::TypeBytes(_)
        | TypeNode::SpliceExprNode(_)
        | TypeNode::TypeRangeFull(_) => Ok(type_ptr.clone()),
    }
}
