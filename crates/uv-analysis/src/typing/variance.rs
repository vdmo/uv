//! Variance of type parameters: how a parameter's position in a type restricts
//! subtyping between instantiations.

use std::collections::BTreeMap;

use uv_source::ast;

use super::type_equiv::type_equiv;
use super::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    /// Output position.
    Covariant,
    /// Input position.
    Contravariant,
    /// Both; instantiations must agree exactly.
    Invariant,
    /// Unused.
    Bivariant,
}

pub fn combine_variance(outer: Variance, inner: Variance) -> Variance {
    match (outer, inner) {
        (Variance::Bivariant, other) | (other, Variance::Bivariant) => other,
        (a, b) if a == b => a,
        _ => Variance::Invariant,
    }
}

pub fn invert_variance(v: Variance) -> Variance {
    match v {
        Variance::Covariant => Variance::Contravariant,
        Variance::Contravariant => Variance::Covariant,
        other => other,
    }
}

pub fn variance_of(type_ref: &TypeRef, param_name: &str) -> Variance {
    let Some(ty) = type_ref else {
        return Variance::Bivariant;
    };
    let combined = |types: &mut dyn Iterator<Item = Variance>| types.fold(Variance::Bivariant, combine_variance);
    match &ty.node {
        TypeNode::Path { path, generic_args } => {
            if matches!(&path[..], [only] if only == param_name) {
                return Variance::Covariant;
            }
            // Arguments of a nominal type that disagree make the parameter invariant.
            let mut result = Variance::Bivariant;
            for arg in generic_args {
                let arg_var = variance_of(arg, param_name);
                if arg_var != Variance::Bivariant {
                    if result == Variance::Bivariant {
                        result = arg_var;
                    } else if result != arg_var {
                        result = Variance::Invariant;
                    }
                }
            }
            result
        }
        TypeNode::Perm { perm: Permission::Const, base } => var_const(base, param_name),
        TypeNode::Perm { base, .. } => var_mut(base, param_name),
        TypeNode::Func { params, ret } => combine_variance(
            combined(&mut params.iter().map(|param| invert_variance(variance_of(&param.r#type, param_name)))),
            variance_of(ret, param_name),
        ),
        TypeNode::Closure { params, ret, deps_opt } => {
            let signature = combine_variance(
                combined(&mut params.iter().map(|(_, param)| invert_variance(variance_of(param, param_name)))),
                variance_of(ret, param_name),
            );
            let deps = deps_opt.iter().flatten().map(|dep| variance_of(&dep.r#type, param_name));
            deps.fold(signature, combine_variance)
        }
        TypeNode::Tuple(types) | TypeNode::Union(types) => {
            combined(&mut types.iter().map(|ty| variance_of(ty, param_name)))
        }
        TypeNode::Array { element, .. } | TypeNode::Slice(element) => var_mut(element, param_name),
        TypeNode::Ptr { element, .. } => variance_of(element, param_name),
        _ => Variance::Bivariant,
    }
}

/// Through mutable storage a used parameter is invariant.
pub fn var_mut(ty: &TypeRef, param_name: &str) -> Variance {
    match variance_of(ty, param_name) {
        Variance::Bivariant => Variance::Bivariant,
        _ => Variance::Invariant,
    }
}

pub fn var_const(ty: &TypeRef, param_name: &str) -> Variance {
    variance_of(ty, param_name)
}

/// Variance per parameter, keyed (and later iterated) by parameter name.
#[derive(Debug, Clone, Default)]
pub struct VarianceContext {
    pub param_variance: BTreeMap<String, Variance>,
}

pub fn compute_variance_context(params: &[ast::TypeParam], member_types: &[TypeRef]) -> VarianceContext {
    let mut ctx = VarianceContext::default();
    for param in params {
        let combined =
            member_types.iter().map(|ty| variance_of(ty, &param.name)).fold(Variance::Bivariant, combine_variance);
        ctx.param_variance.insert(param.name.clone(), combined);
    }
    ctx
}

/// Whether two argument lists may be related by subtyping. Only invariant parameters
/// are checked here, and arguments are paired with parameters in name order, as the
/// reference does.
pub fn check_generic_subtyping(ctx: &VarianceContext, args1: &[TypeRef], args2: &[TypeRef]) -> bool {
    if args1.len() != args2.len() {
        return false;
    }
    ctx.param_variance
        .values()
        .zip(args1.iter().zip(args2))
        .all(|(variance, (a1, a2))| *variance != Variance::Invariant || type_equiv(a1, a2))
}
