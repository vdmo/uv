//! Substituting type arguments for type parameters.

use std::collections::BTreeMap;

use uv_source::ast;

use crate::typing::type_lower::{lower_bytes_state, lower_permission, lower_ptr_state, lower_raw_ptr_qual, lower_string_state};
use crate::typing::types::*;

pub type TypeSubst = BTreeMap<String, TypeRef>;

/// The type with parameters replaced. A bare one-segment path that names a parameter is
/// replaced whole; unions are canonicalised again, since members may now coincide.
pub fn instantiate_type(type_ref: &TypeRef, subst: &TypeSubst) -> TypeRef {
    let Some(ty) = type_ref else {
        return None;
    };
    let inst = |ty: &TypeRef| instantiate_type(ty, subst);
    let list = |types: &[TypeRef]| types.iter().map(inst).collect::<Vec<_>>();
    match &ty.node {
        TypeNode::Path { path, generic_args } => {
            if let [only] = &path[..] {
                if let Some(replacement) = subst.get(only) {
                    return replacement.clone();
                }
            }
            if generic_args.is_empty() {
                return type_ref.clone();
            }
            make_type_path_with(path.clone(), list(generic_args))
        }
        TypeNode::Apply { path, args } => make_type_apply(path.clone(), list(args)),
        TypeNode::Perm { perm, base } => make_type_perm(*perm, inst(base)),
        TypeNode::Tuple(elements) => make_type_tuple(list(elements)),
        TypeNode::Array { element, length, length_expr_text } => {
            make_type_array(inst(element), *length, length_expr_text.clone())
        }
        TypeNode::Slice(element) => make_type_slice(inst(element)),
        TypeNode::Ptr { element, state } => make_type_ptr(inst(element), *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, inst(element)),
        TypeNode::Union(members) => make_type_union(list(members)),
        TypeNode::Func { params, ret } => make_type_func(
            params.iter().map(|param| TypeFuncParam { mode: param.mode, r#type: inst(&param.r#type) }).collect(),
            inst(ret),
        ),
        TypeNode::Closure { params, ret, deps_opt } => make_type_closure(
            params.iter().map(|(is_move, param)| (*is_move, inst(param))).collect(),
            inst(ret),
            deps_opt.as_ref().map(|deps| {
                deps.iter().map(|dep| SharedDep { name: dep.name.clone(), r#type: inst(&dep.r#type) }).collect()
            }),
        ),
        TypeNode::ModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)),
        TypeNode::Refine { base, predicate } => make_type_refine(inst(base), predicate.clone()),
        TypeNode::Range(base) => make_type(TypeNode::Range(inst(base))),
        TypeNode::RangeInclusive(base) => make_type(TypeNode::RangeInclusive(inst(base))),
        TypeNode::RangeFrom(base) => make_type(TypeNode::RangeFrom(inst(base))),
        TypeNode::RangeTo(base) => make_type(TypeNode::RangeTo(inst(base))),
        TypeNode::RangeToInclusive(base) => make_type(TypeNode::RangeToInclusive(inst(base))),
        TypeNode::RangeFull => make_type(TypeNode::RangeFull),
        TypeNode::Prim(_)
        | TypeNode::Var(_)
        | TypeNode::String(_)
        | TypeNode::Bytes(_)
        | TypeNode::Dynamic(_)
        | TypeNode::Opaque { .. } => type_ref.clone(),
    }
}

/// Lowers the default of a type parameter. This runs without a scope, so it differs
/// from ordinary lowering: array lengths are not evaluated (the length is 0), a generic
/// path stays a path with arguments, and range types are not supported.
fn lower_default_type(type_ptr: &ast::TypePtr) -> Option<TypeRef> {
    let ty = type_ptr.as_deref()?;
    let list = |types: &[ast::TypePtr]| types.iter().map(lower_default_type).collect::<Option<Vec<_>>>();
    use ast::TypeNode as N;
    Some(match &ty.node {
        N::TypePrim(node) => make_type_prim(&node.name),
        N::TypePathType(node) if node.generic_args.is_empty() => make_type_path(node.path.clone()),
        N::TypePathType(node) => make_type_path_with(node.path.clone(), list(&node.generic_args)?),
        N::TypeTuple(node) => make_type_tuple(list(&node.elements)?),
        N::TypeArray(node) => make_type_array(lower_default_type(&node.element)?, 0, None),
        N::TypeSlice(node) => make_type_slice(lower_default_type(&node.element)?),
        N::TypePermType(node) => make_type_perm(lower_permission(node.perm), lower_default_type(&node.base)?),
        N::TypeUnion(node) => make_type_union(list(&node.types)?),
        N::TypeFunc(node) => {
            let mut params = Vec::with_capacity(node.params.len());
            for param in &node.params {
                let mode = param.mode.map(|_| ParamMode::Move);
                params.push(TypeFuncParam { mode, r#type: lower_default_type(&param.r#type)? });
            }
            make_type_func(params, lower_default_type(&node.ret)?)
        }
        N::TypeClosure(node) => {
            let mut params = Vec::with_capacity(node.params.len());
            for param in &node.params {
                params.push((param.mode.is_some(), lower_default_type(&param.r#type)?));
            }
            let ret = lower_default_type(&node.ret)?;
            let deps_opt = match &node.deps_opt {
                Some(deps) => Some(
                    deps.iter()
                        .map(|dep| Some(SharedDep { name: dep.name.clone(), r#type: lower_default_type(&dep.r#type)? }))
                        .collect::<Option<Vec<_>>>()?,
                ),
                None => None,
            };
            make_type_closure(params, ret, deps_opt)
        }
        N::TypeSafePtr(node) => make_type_ptr(lower_default_type(&node.element)?, lower_ptr_state(node.state)),
        N::TypeRawPtr(node) => make_type_raw_ptr(lower_raw_ptr_qual(node.qual), lower_default_type(&node.element)?),
        N::TypeString(node) => make_type_string(lower_string_state(node.state)),
        N::TypeBytes(node) => make_type_bytes(lower_bytes_state(node.state)),
        N::TypeDynamic(node) => make_type_dynamic(node.path.clone()),
        N::TypeModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)?),
        N::TypeOpaque(node) => {
            make_type(TypeNode::Opaque { class_path: node.path.clone(), origin: None, origin_span: Default::default() })
        }
        N::TypeRefine(node) => make_type_refine(lower_default_type(&node.base)?, node.predicate.clone()),
        _ => return None,
    })
}

/// Parameters to arguments. A parameter without an argument takes its default, which
/// may mention earlier parameters; a default that cannot be lowered becomes `!`. A
/// parameter with neither is left out.
pub fn build_substitution(params: &[ast::TypeParam], args: &[TypeRef]) -> TypeSubst {
    let mut subst = TypeSubst::new();
    for (param, arg) in params.iter().zip(args) {
        subst.insert(param.name.clone(), arg.clone());
    }
    for param in params.iter().skip(args.len()) {
        if param.default_type.is_some() {
            let default = lower_default_type(&param.default_type)
                .and_then(|default| instantiate_type(&default, &subst).map(Some))
                .unwrap_or_else(|| make_type_prim("!"));
            subst.insert(param.name.clone(), default);
        }
    }
    subst
}
