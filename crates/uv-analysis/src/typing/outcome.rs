//! `Outcome<TValue, TError>`: a value may be returned where an outcome is expected when
//! it is of exactly one of the two types.

use super::subtyping::subtyping;
use super::type_predicates::strip_perm;
use super::types::*;
use crate::caps::builtin_paths::is_outcome_type_path;
use crate::context::ScopeContext;

#[derive(Debug, Clone)]
pub struct OutcomeSig {
    pub value: TypeRef,
    pub error: TypeRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeIntro {
    None,
    Value,
    Error,
    Ambiguous,
}

pub fn make_outcome_type(value_type: TypeRef, error_type: TypeRef) -> TypeRef {
    make_type_path_with(vec!["Outcome".to_string()], vec![value_type, error_type])
}

pub fn outcome_sig_of(ty: &TypeRef) -> Option<OutcomeSig> {
    let stripped = strip_perm(ty);
    let stripped = stripped.as_deref()?;
    let (path, args) = applied_type_path(stripped).zip(applied_type_args(stripped))?;
    match args {
        [value, error] if is_outcome_type_path(path) => Some(OutcomeSig { value: value.clone(), error: error.clone() }),
        _ => None,
    }
}

/// How a value of type `from` becomes an outcome `to`, when it is not one already.
pub fn classify_outcome_intro(ctx: &ScopeContext<'_>, from: &TypeRef, to: &TypeRef) -> OutcomeIntro {
    let Some(sig) = outcome_sig_of(to).filter(|_| from.is_some()) else {
        return OutcomeIntro::None;
    };
    if subtyping(ctx, from, to).subtype {
        return OutcomeIntro::None;
    }
    match (subtyping(ctx, from, &sig.value).subtype, subtyping(ctx, from, &sig.error).subtype) {
        (true, true) => OutcomeIntro::Ambiguous,
        (true, false) => OutcomeIntro::Value,
        (false, true) => OutcomeIntro::Error,
        (false, false) => OutcomeIntro::None,
    }
}
