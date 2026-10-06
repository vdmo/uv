//! Tuple element access, as a value and as a place.

use uv_source::ast;

use crate::context::ScopeContext;
use crate::typing::alias_normalize::normalize_index_base_type;
use crate::typing::callbacks::PlaceTypeResult;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::{suppress_shared_access_check, StmtTypeContext};
use crate::typing::type_env::TypeEnv;
use crate::typing::type_expr::{type_expr, type_place};
use crate::typing::type_predicates::{bitcopy_type, strip_perm};
use crate::typing::types::*;

/// The element the access names, under the permission of the base.
fn tuple_element(
    ctx: &ScopeContext<'_>,
    base_type: &TypeRef,
    index: u128,
) -> Result<TypeRef, (&'static str, Vec<&'static str>)> {
    let normalized = normalize_index_base_type(ctx, &strip_perm(base_type)).map_err(|diag_id| (diag_id.unwrap_or(""), Vec::new()))?;
    let elements = match normalized.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Tuple(elements)) => elements,
        Some(TypeNode::Union(_)) => return Err(("Union-DirectAccess-Err", Vec::new())),
        _ => return Err(("TupleAccess-NotTuple", vec!["TupleAccess-NotTuple"])),
    };
    let Some(element) = usize::try_from(index).ok().and_then(|index| elements.get(index)) else {
        return Err(("TupleIndex-OOB", vec!["TupleIndex-OOB", "diagnostics.Tuples"]));
    };
    Ok(match base_type.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, .. }) => make_type_perm(*perm, element.clone()),
        _ => element.clone(),
    })
}

fn diag_of(diag_id: &'static str) -> Option<&'static str> {
    (!diag_id.is_empty()).then_some(diag_id)
}

pub fn type_tuple_access_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::TupleAccessExpr,
    env: &TypeEnv,
) -> ExprTypeResult {
    if expr.base.is_none() {
        return ExprTypeResult::default();
    }
    let base_type = type_expr(ctx, &suppress_shared_access_check(type_ctx), &expr.base, env);
    if !base_type.ok {
        return ExprTypeResult::failed(base_type.diag_id);
    }
    match tuple_element(ctx, &base_type.r#type, expr.index) {
        Err((diag_id, diagnostic_obligation_ids)) => {
            ExprTypeResult { diag_id: diag_of(diag_id), diagnostic_obligation_ids, ..Default::default() }
        }
        // Reading an element copies it.
        Ok(element) if !bitcopy_type(ctx, &element) => ExprTypeResult::failed(Some("ValueUse-NonBitcopyPlace")),
        Ok(element) => ExprTypeResult::typed(element),
    }
}

pub fn type_tuple_access_place(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::TupleAccessExpr,
    env: &TypeEnv,
) -> PlaceTypeResult {
    if expr.base.is_none() {
        return PlaceTypeResult::default();
    }
    let base_type = type_place(ctx, &suppress_shared_access_check(type_ctx), &expr.base, env);
    if !base_type.ok {
        return PlaceTypeResult { diag_id: base_type.diag_id, ..Default::default() };
    }
    match tuple_element(ctx, &base_type.r#type, expr.index) {
        Err((diag_id, _)) => PlaceTypeResult { diag_id: diag_of(diag_id), ..Default::default() },
        Ok(element) => PlaceTypeResult { ok: true, r#type: element, ..Default::default() },
    }
}
