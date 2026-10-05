//! Signatures of methods and transitions as function types.

use uv_source::ast;

use super::type_lower::{lower_param_mode, lower_type, LowerError};
use super::type_wf::type_wf;
use super::types::*;
use crate::composite::record_methods::{recv_mode_of, recv_type_for_receiver};
use crate::context::ScopeContext;
use crate::generics::monomorphize::TypeSubst;

/// The type with `Self` replaced by `self_type`, and `Self::Name` by the associated type
/// bound to `Name`, if any.
pub fn subst_self_type(self_type: &TypeRef, type_ref: &TypeRef, assoc_subst: Option<&TypeSubst>) -> TypeRef {
    let (Some(ty), Some(_)) = (type_ref.as_deref(), self_type) else {
        return type_ref.clone();
    };
    let subst = |ty: &TypeRef| subst_self_type(self_type, ty, assoc_subst);
    let list = |types: &[TypeRef]| types.iter().map(subst).collect::<Vec<_>>();
    match &ty.node {
        TypeNode::Path { path, generic_args } => {
            if is_self_var_path(path) {
                return self_type.clone();
            }
            if let ([head, name], Some(assoc)) = (&path[..], assoc_subst) {
                if head == "Self" {
                    if let Some(bound) = assoc.get(name) {
                        return bound.clone();
                    }
                }
            }
            if generic_args.is_empty() {
                return type_ref.clone();
            }
            make_type_path_with(path.clone(), list(generic_args))
        }
        TypeNode::Apply { path, args } => make_type_apply(path.clone(), list(args)),
        TypeNode::Perm { perm, base } => make_type_perm(*perm, subst(base)),
        TypeNode::Union(members) => make_type_union(list(members)),
        TypeNode::Func { params, ret } => make_type_func(
            params.iter().map(|param| TypeFuncParam { mode: param.mode, r#type: subst(&param.r#type) }).collect(),
            subst(ret),
        ),
        TypeNode::Closure { params, ret, deps_opt } => make_type_closure(
            params.iter().map(|(is_move, param)| (*is_move, subst(param))).collect(),
            subst(ret),
            deps_opt.as_ref().map(|deps| {
                deps.iter().map(|dep| SharedDep { name: dep.name.clone(), r#type: subst(&dep.r#type) }).collect()
            }),
        ),
        TypeNode::Tuple(elements) => make_type_tuple(list(elements)),
        TypeNode::Array { element, length, length_expr_text } => {
            make_type_array(subst(element), *length, length_expr_text.clone())
        }
        TypeNode::Slice(element) => make_type_slice(subst(element)),
        TypeNode::Ptr { element, state } => make_type_ptr(subst(element), *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, subst(element)),
        TypeNode::ModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)),
        TypeNode::Refine { base, predicate } => make_type_refine(subst(base), predicate.clone()),
        TypeNode::Range(base) => make_type(TypeNode::Range(subst(base))),
        TypeNode::RangeInclusive(base) => make_type(TypeNode::RangeInclusive(subst(base))),
        TypeNode::RangeFrom(base) => make_type(TypeNode::RangeFrom(subst(base))),
        TypeNode::RangeTo(base) => make_type(TypeNode::RangeTo(subst(base))),
        TypeNode::RangeToInclusive(base) => make_type(TypeNode::RangeToInclusive(subst(base))),
        TypeNode::RangeFull => make_type(TypeNode::RangeFull),
        TypeNode::Prim(_)
        | TypeNode::Var(_)
        | TypeNode::String(_)
        | TypeNode::Bytes(_)
        | TypeNode::Dynamic(_)
        | TypeNode::Opaque { .. } => type_ref.clone(),
    }
}

/// A callable's type, and its parameters by name for the body.
#[derive(Debug, Clone)]
pub struct Signature {
    pub func_type: TypeRef,
    pub return_type: TypeRef,
    pub bindings: Vec<(String, TypeRef)>,
}

fn lower_type_with_wf(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Result<TypeRef, LowerError> {
    let lowered = lower_type(ctx, ty)?;
    type_wf(ctx, &lowered)?;
    Ok(lowered)
}

/// The receiver comes first among the parameters; a method without a written return
/// type returns `()`.
pub fn build_method_signature(
    ctx: &ScopeContext<'_>,
    self_type: &TypeRef,
    receiver: &ast::Receiver,
    params: &[ast::Param],
    return_type_opt: &ast::TypePtr,
    assoc_subst: Option<&TypeSubst>,
) -> Result<Signature, LowerError> {
    let recv = recv_type_for_receiver(self_type, receiver, |ty| lower_type_with_wf(ctx, ty))?;
    let mut func_params = Vec::with_capacity(params.len() + 1);
    let mut bindings = Vec::with_capacity(params.len() + 1);
    if recv.is_some() {
        let recv = subst_self_type(self_type, &recv, assoc_subst);
        bindings.push(("self".to_string(), recv.clone()));
        func_params.push(TypeFuncParam { mode: recv_mode_of(receiver), r#type: recv });
    }
    for param in params {
        let ty = subst_self_type(self_type, &lower_type_with_wf(ctx, &param.r#type)?, assoc_subst);
        func_params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: ty.clone() });
        bindings.push((param.name.clone(), ty));
    }
    let return_type = match return_type_opt {
        Some(_) => subst_self_type(self_type, &lower_type_with_wf(ctx, return_type_opt)?, assoc_subst),
        None => make_type_prim("()"),
    };
    Ok(Signature { func_type: make_type_func(func_params, return_type.clone()), return_type, bindings })
}

/// A transition consumes the source state and returns the target state.
pub fn build_transition_signature(
    ctx: &ScopeContext<'_>,
    source_self_type: &TypeRef,
    target_self_type: &TypeRef,
    params: &[ast::Param],
    assoc_subst: Option<&TypeSubst>,
) -> Result<Signature, LowerError> {
    let recv_type = make_type_perm(Permission::Unique, source_self_type.clone());
    let mut bindings = vec![("self".to_string(), recv_type.clone())];
    let mut func_params = vec![TypeFuncParam { mode: Some(ParamMode::Move), r#type: recv_type }];
    for param in params {
        let ty = subst_self_type(source_self_type, &lower_type_with_wf(ctx, &param.r#type)?, assoc_subst);
        func_params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: ty.clone() });
        bindings.push((param.name.clone(), ty));
    }
    Ok(Signature {
        func_type: make_type_func(func_params, target_self_type.clone()),
        return_type: target_self_type.clone(),
        bindings,
    })
}
