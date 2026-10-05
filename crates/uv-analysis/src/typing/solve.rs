//! Solving the constraints that inference collects: each equates two types or asks for
//! one to be a subtype of the other, and solving binds the type variables in them.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::subtyping::subtyping;
use super::type_equiv::type_equiv;
use super::types::*;
use crate::context::ScopeContext;
use crate::contracts::struct_equal::expr_struct_equal;
use crate::resolve::scopes::{id_eq, id_key_of};

#[derive(Debug, Clone)]
pub struct Constraint {
    pub lhs: TypeRef,
    pub rhs: TypeRef,
    pub requires_subtyping: bool,
}

pub type ConstraintSet = Vec<Constraint>;

/// Type variables to what they stand for.
pub type TypeSubstitution = HashMap<u32, TypeRef>;

const UNSOLVABLE: &str = "Syn-Call-Err";

fn same(a: &TypeRef, b: &TypeRef) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// Applies the substitution to each type; nothing when none of them changed.
fn apply_all(types: &[TypeRef], subst: &TypeSubstitution, active: &mut HashSet<u32>) -> Option<Vec<TypeRef>> {
    let applied: Vec<TypeRef> = types.iter().map(|ty| apply_impl(ty, subst, active)).collect();
    applied.iter().zip(types).any(|(new, old)| !same(new, old)).then_some(applied)
}

/// The type with bound variables replaced, transitively; a variable bound through
/// itself is left as it is. A type in which nothing changes is returned as it was, so
/// its unions keep the order they had.
fn apply_impl(type_ref: &TypeRef, subst: &TypeSubstitution, active: &mut HashSet<u32>) -> TypeRef {
    let ty = type_ref.as_deref()?;
    let unchanged = || type_ref.clone();
    let mut one = |inner: &TypeRef, rebuild: &dyn Fn(TypeRef) -> TypeRef| {
        let applied = apply_impl(inner, subst, active);
        if same(&applied, inner) {
            type_ref.clone()
        } else {
            rebuild(applied)
        }
    };
    match &ty.node {
        TypeNode::Var(id) => {
            let Some(bound @ Some(_)) = subst.get(id) else {
                return unchanged();
            };
            if !active.insert(*id) {
                return unchanged();
            }
            let out = apply_impl(bound, subst, active);
            active.remove(id);
            out.or_else(unchanged)
        }
        TypeNode::Prim(_)
        | TypeNode::String(_)
        | TypeNode::Bytes(_)
        | TypeNode::Dynamic(_)
        | TypeNode::Opaque { .. }
        | TypeNode::RangeFull => unchanged(),
        TypeNode::Path { path, generic_args } => apply_all(generic_args, subst, active)
            .map_or_else(unchanged, |args| make_type_path_with(path.clone(), args)),
        TypeNode::Apply { path, args } => {
            apply_all(args, subst, active).map_or_else(unchanged, |args| make_type_apply(path.clone(), args))
        }
        TypeNode::ModalState(node) => apply_all(&node.generic_args, subst, active)
            .map_or_else(unchanged, |args| make_type_modal_state(node.path.clone(), &node.state, args)),
        TypeNode::Union(members) => apply_all(members, subst, active).map_or_else(unchanged, make_type_union),
        TypeNode::Tuple(elements) => apply_all(elements, subst, active).map_or_else(unchanged, make_type_tuple),
        TypeNode::Func { params, ret } => {
            let types: Vec<TypeRef> = params.iter().map(|param| param.r#type.clone()).chain([ret.clone()]).collect();
            apply_all(&types, subst, active).map_or_else(unchanged, |mut types| {
                let ret = types.pop().expect("the return type");
                let params = params.iter().zip(types).map(|(param, ty)| TypeFuncParam { mode: param.mode, r#type: ty });
                make_type_func(params.collect(), ret)
            })
        }
        TypeNode::Closure { params, ret, deps_opt } => {
            let deps = deps_opt.as_deref().unwrap_or(&[]);
            let types: Vec<TypeRef> = params
                .iter()
                .map(|(_, ty)| ty.clone())
                .chain([ret.clone()])
                .chain(deps.iter().map(|dep| dep.r#type.clone()))
                .collect();
            apply_all(&types, subst, active).map_or_else(unchanged, |types| {
                let (param_types, rest) = types.split_at(params.len());
                let new_params = params.iter().zip(param_types).map(|((is_move, _), ty)| (*is_move, ty.clone())).collect();
                let new_deps = deps_opt.as_ref().map(|deps| {
                    deps.iter()
                        .zip(&rest[1..])
                        .map(|(dep, ty)| SharedDep { name: dep.name.clone(), r#type: ty.clone() })
                        .collect()
                });
                make_type_closure(new_params, rest[0].clone(), new_deps)
            })
        }
        TypeNode::Perm { perm, base } => one(base, &|base| make_type_perm(*perm, base)),
        TypeNode::Array { element, length, length_expr_text } => {
            one(element, &|element| make_type_array(element, *length, length_expr_text.clone()))
        }
        TypeNode::Slice(element) => one(element, &make_type_slice),
        TypeNode::Ptr { element, state } => one(element, &|element| make_type_ptr(element, *state)),
        TypeNode::RawPtr { qual, element } => one(element, &|element| make_type_raw_ptr(*qual, element)),
        TypeNode::Refine { base, predicate } => one(base, &|base| make_type_refine(base, predicate.clone())),
        TypeNode::Range(base) => one(base, &|base| make_type(TypeNode::Range(base))),
        TypeNode::RangeInclusive(base) => one(base, &|base| make_type(TypeNode::RangeInclusive(base))),
        TypeNode::RangeFrom(base) => one(base, &|base| make_type(TypeNode::RangeFrom(base))),
        TypeNode::RangeTo(base) => one(base, &|base| make_type(TypeNode::RangeTo(base))),
        TypeNode::RangeToInclusive(base) => one(base, &|base| make_type(TypeNode::RangeToInclusive(base))),
    }
}

pub fn apply_substitution(ty: &TypeRef, subst: &TypeSubstitution) -> TypeRef {
    apply_impl(ty, subst, &mut HashSet::new())
}

/// Whether the variable occurs in the type, looking through bound variables. The
/// arguments of a type application are not searched.
fn occurs_in_type(id: u32, type_ref: &TypeRef, subst: &TypeSubstitution, seen: &mut HashSet<u32>) -> bool {
    let Some(ty) = type_ref.as_deref() else {
        return false;
    };
    let mut occurs = |ty: &TypeRef| occurs_in_type(id, ty, subst, seen);
    match &ty.node {
        TypeNode::Var(var) => {
            *var == id || (seen.insert(*var) && subst.get(var).is_some_and(|bound| occurs_in_type(id, bound, subst, seen)))
        }
        TypeNode::Path { generic_args, .. } => generic_args.iter().any(occurs),
        TypeNode::ModalState(node) => node.generic_args.iter().any(occurs),
        TypeNode::Union(types) | TypeNode::Tuple(types) => types.iter().any(occurs),
        TypeNode::Func { params, ret } => params.iter().any(|param| occurs(&param.r#type)) || occurs(ret),
        TypeNode::Closure { params, ret, deps_opt } => {
            params.iter().any(|(_, ty)| occurs(ty)) || occurs(ret) || deps_opt.iter().flatten().any(|dep| occurs(&dep.r#type))
        }
        TypeNode::Perm { base, .. }
        | TypeNode::Refine { base, .. }
        | TypeNode::Range(base)
        | TypeNode::RangeInclusive(base)
        | TypeNode::RangeFrom(base)
        | TypeNode::RangeTo(base)
        | TypeNode::RangeToInclusive(base) => occurs(base),
        TypeNode::Array { element, .. }
        | TypeNode::Slice(element)
        | TypeNode::Ptr { element, .. }
        | TypeNode::RawPtr { element, .. } => occurs(element),
        _ => false,
    }
}

/// Binds the variable unless that would make it stand for a type containing itself.
fn bind_type_var(id: u32, rhs: &TypeRef, subst: &mut TypeSubstitution) -> bool {
    match rhs.as_deref().map(|ty| &ty.node) {
        None => return false,
        Some(TypeNode::Var(var)) if *var == id => return true,
        Some(_) => {}
    }
    if occurs_in_type(id, rhs, subst, &mut HashSet::new()) {
        return false;
    }
    subst.insert(id, rhs.clone());
    true
}

fn type_path_eq(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| id_key_of(a) == id_key_of(b))
}

type Unified = Result<(), Option<&'static str>>;

fn unify_all(ctx: &ScopeContext<'_>, lhs: &[TypeRef], rhs: &[TypeRef], subst: &mut TypeSubstitution) -> Unified {
    if lhs.len() != rhs.len() {
        return Err(Some(UNSOLVABLE));
    }
    lhs.iter().zip(rhs).try_for_each(|(l, r)| unify_eq(ctx, l, r, subst))
}

/// Makes two types equal by binding variables; the shapes must agree everywhere else.
fn unify_eq(ctx: &ScopeContext<'_>, lhs_raw: &TypeRef, rhs_raw: &TypeRef, subst: &mut TypeSubstitution) -> Unified {
    let lhs_ref = apply_substitution(lhs_raw, subst);
    let rhs_ref = apply_substitution(rhs_raw, subst);
    let fail = Err(Some(UNSOLVABLE));
    let (Some(lhs), Some(rhs)) = (lhs_ref.as_deref(), rhs_ref.as_deref()) else {
        return fail;
    };
    let require = |holds: bool| if holds { Ok(()) } else { Err(Some(UNSOLVABLE)) };
    use TypeNode as N;
    if let N::Var(id) = &lhs.node {
        return require(bind_type_var(*id, &rhs_ref, subst));
    }
    if let N::Var(id) = &rhs.node {
        return require(bind_type_var(*id, &lhs_ref, subst));
    }
    if let N::Refine { base: lbase, predicate: lpred } = &lhs.node {
        let N::Refine { base: rbase, predicate: rpred } = &rhs.node else {
            return fail;
        };
        let predicates_equal = match (lpred, rpred) {
            (Some(_), Some(_)) => expr_struct_equal(lpred, rpred),
            (None, None) => true,
            _ => false,
        };
        require(predicates_equal)?;
        return unify_eq(ctx, lbase, rbase, subst);
    }
    if type_equiv(&lhs_ref, &rhs_ref) {
        return Ok(());
    }
    let rhs_applied = applied_type_path(rhs).zip(applied_type_args(rhs));
    match (&lhs.node, &rhs.node) {
        (N::Perm { perm: lp, base: lb }, N::Perm { perm: rp, base: rb }) if lp == rp => unify_eq(ctx, lb, rb, subst),
        (N::Tuple(l), N::Tuple(r)) => unify_all(ctx, l, r, subst),
        (N::Array { element: l, length: ll, .. }, N::Array { element: r, length: rl, .. }) if ll == rl => {
            unify_eq(ctx, l, r, subst)
        }
        (N::Slice(l), N::Slice(r)) => unify_eq(ctx, l, r, subst),
        (N::Func { params: lp, ret: lr }, N::Func { params: rp, ret: rr }) if lp.len() == rp.len() => {
            for (l, r) in lp.iter().zip(rp) {
                require(l.mode == r.mode)?;
                unify_eq(ctx, &l.r#type, &r.r#type, subst)?;
            }
            unify_eq(ctx, lr, rr, subst)
        }
        (N::Path { path, generic_args: args } | N::Apply { path, args }, _) => match rhs_applied {
            Some((rpath, rargs)) if type_path_eq(path, rpath) => unify_all(ctx, args, rargs, subst),
            _ => fail,
        },
        (N::ModalState(l), N::ModalState(r)) if type_path_eq(&l.path, &r.path) && id_eq(&l.state, &r.state) => {
            unify_all(ctx, &l.generic_args, &r.generic_args, subst)
        }
        (N::Ptr { element: l, state: ls }, N::Ptr { element: r, state: rs }) if ls == rs => unify_eq(ctx, l, r, subst),
        (N::RawPtr { qual: lq, element: l }, N::RawPtr { qual: rq, element: r }) if lq == rq => unify_eq(ctx, l, r, subst),
        (N::Range(l), N::Range(r))
        | (N::RangeInclusive(l), N::RangeInclusive(r))
        | (N::RangeFrom(l), N::RangeFrom(r))
        | (N::RangeTo(l), N::RangeTo(r))
        | (N::RangeToInclusive(l), N::RangeToInclusive(r)) => unify_eq(ctx, l, r, subst),
        (N::RangeFull, N::RangeFull) => Ok(()),
        (N::Closure { params: lp, ret: lr, deps_opt: ld }, N::Closure { params: rp, ret: rr, deps_opt: rd })
            if lp.len() == rp.len() =>
        {
            for (l, r) in lp.iter().zip(rp) {
                require(l.0 == r.0)?;
                unify_eq(ctx, &l.1, &r.1, subst)?;
            }
            unify_eq(ctx, lr, rr, subst)?;
            match (ld, rd) {
                (None, None) => Ok(()),
                (Some(ld), Some(rd)) if ld.len() == rd.len() => {
                    for (l, r) in ld.iter().zip(rd) {
                        require(id_eq(&l.name, &r.name))?;
                        unify_eq(ctx, &l.r#type, &r.r#type, subst)?;
                    }
                    Ok(())
                }
                _ => fail,
            }
        }
        // Members are paired in type order.
        (N::Union(l), N::Union(r)) if l.len() == r.len() => {
            unify_all(ctx, &sort_union_members(l), &sort_union_members(r), subst)
        }
        _ => fail,
    }
}

/// Solves the constraints in order. A subtyping constraint with a variable on either
/// side binds the variable to the other side; otherwise the relation must hold as the
/// types stand.
pub fn solve(ctx: &ScopeContext<'_>, constraints: &[Constraint]) -> Result<TypeSubstitution, Option<&'static str>> {
    let mut subst = TypeSubstitution::new();
    for constraint in constraints {
        let lhs = apply_substitution(&constraint.lhs, &subst);
        let rhs = apply_substitution(&constraint.rhs, &subst);
        let (Some(lhs_ty), Some(rhs_ty)) = (lhs.as_deref(), rhs.as_deref()) else {
            return Err(Some(UNSOLVABLE));
        };
        if !constraint.requires_subtyping {
            unify_eq(ctx, &lhs, &rhs, &mut subst).map_err(|diag_id| diag_id.or(Some(UNSOLVABLE)))?;
            continue;
        }
        let bound = match (&lhs_ty.node, &rhs_ty.node) {
            (TypeNode::Var(id), _) => Some(bind_type_var(*id, &rhs, &mut subst)),
            (_, TypeNode::Var(id)) => Some(bind_type_var(*id, &lhs, &mut subst)),
            _ => None,
        };
        match bound {
            Some(true) => {}
            Some(false) => return Err(Some(UNSOLVABLE)),
            None => {
                let sub = subtyping(ctx, &lhs, &rhs);
                if !sub.ok {
                    return Err(sub.diag_id);
                }
                if !sub.subtype {
                    return Err(sub.diag_id.or(Some(UNSOLVABLE)));
                }
            }
        }
    }
    Ok(subst)
}
