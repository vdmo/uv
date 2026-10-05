//! Arrays and slices.

use crate::context::ScopeContext;
use crate::typing::alias_normalize::normalize_index_base_type;
use crate::typing::types::*;

/// The slice an array coerces to, keeping its permission; `const` when it has none.
/// Nothing for other types; a failure names the rule that expanding an alias failed with.
pub fn coerce_array_to_slice(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Option<&'static str>> {
    if ty.is_none() {
        return Err(None);
    }
    let normalized = normalize_index_base_type(ctx, ty)?;
    let (perm, base) = match normalized.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, base }) if base.is_some() => (*perm, base),
        _ => (Permission::Const, &normalized),
    };
    match base.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Array { element, .. }) => Ok(make_type_perm(perm, make_type_slice(element.clone()))),
        _ => Err(None),
    }
}
