//! Typing `return`.
//!
//! The value is checked against the return type of the enclosing body (for an
//! asynchronous body, against what it completes with). A place is returned by move. A
//! value that is not of an expected `Outcome` type but is of exactly one of its two
//! types is accepted as that alternative.

use std::sync::Arc;

use uv_source::ast::{self, ExprNode, ExprPtr};

use super::block::StmtTypeResult;
use super::postcondition::verify_postcondition_at_return;
use crate::context::ScopeContext;
use crate::typing::closure_capture::{analyze_closure_capture_info, closure_type_has_shared_deps};
use crate::typing::callbacks::ExprTypeFn;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::outcome::{classify_outcome_intro, OutcomeIntro};
use crate::typing::pending::pending;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, TypeEnv};
use crate::typing::type_expr::{check_expr_against, type_expr};
use crate::typing::type_lookup::async_sig_of;
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

/// A place expression, which `return` moves out of.
fn is_return_move_place(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IdentifierExpr(_) | ExprNode::DerefExpr(_)) => true,
        Some(ExprNode::FieldAccessExpr(node)) => is_return_move_place(&node.base),
        Some(ExprNode::TupleAccessExpr(node)) => is_return_move_place(&node.base),
        Some(ExprNode::IndexAccessExpr(node)) => is_return_move_place(&node.base),
        Some(ExprNode::AttributedExpr(node)) => is_return_move_place(&node.expr),
        _ => false,
    }
}

/// The returned expression as it is checked: a bare place becomes `move place`.
fn return_dest_expr(value: &ExprPtr) -> ExprPtr {
    let Some(expr) = value.as_deref() else {
        return value.clone();
    };
    if matches!(expr.node, ExprNode::CopyExpr(_) | ExprNode::MoveExpr(_)) || !is_return_move_place(value) {
        return value.clone();
    }
    let node = ExprNode::MoveExpr(ast::MoveExpr { place: value.clone() });
    Some(Arc::new(ast::Expr { span: expr.span.clone(), node }))
}

fn type_expr_with_current_env(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    expr: &ExprPtr,
) -> ExprTypeResult {
    if expr.is_none() {
        return ExprTypeResult::default();
    }
    let via_env = type_expr(ctx, type_ctx, expr, env);
    if via_env.ok || via_env.diag_id.is_some() {
        return via_env;
    }
    type_expr_fn(expr)
}

fn is_safe_ptr_type(ty: &TypeRef) -> bool {
    let inner = match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => base,
        _ => ty,
    };
    matches!(inner.as_deref().map(|ty| &ty.node), Some(TypeNode::Ptr { .. }))
}

fn type_contains_safe_pointer(ty: &TypeRef) -> bool {
    let stripped = strip_perm_and_refine(ty);
    if is_safe_ptr_type(&stripped) {
        return true;
    }
    matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Union(members))
        if members.iter().any(type_contains_safe_pointer))
}

/// Whether the type could turn out to be of the wanted kind: it is, a member of a union
/// could be, or it is a type that is not known here (a name, a parameter, `$Class`).
fn type_may_be(ty: &TypeRef, wanted: fn(&TypeNode) -> bool) -> bool {
    let stripped = strip_perm_and_refine(ty);
    let Some(node) = stripped.as_deref().map(|ty| &ty.node) else {
        return true;
    };
    match node {
        node if wanted(node) => true,
        TypeNode::Union(members) => members.iter().any(|member| type_may_be(member, wanted)),
        TypeNode::Var(_) | TypeNode::Dynamic(_) | TypeNode::Apply { .. } | TypeNode::Path { .. } | TypeNode::Opaque { .. } => true,
        _ => false,
    }
}

/// A returned closure must not capture shared data without declaring it.
fn check_escaping_closure_return(ret_expr: &ExprPtr, env: &TypeEnv, expected: &TypeRef) -> Option<&'static str> {
    if !type_may_be(expected, |node| matches!(node, TypeNode::Closure { .. })) {
        return None;
    }
    let expects_shared_deps = closure_type_has_shared_deps(expected);
    let info = match ret_expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IdentifierExpr(ident)) => {
            let binding = bind_of(env, &ident.name)?;
            let mut info = binding.closure_capture_info?;
            info.has_shared_deps = info.has_shared_deps || closure_type_has_shared_deps(&binding.r#type) || expects_shared_deps;
            info
        }
        Some(_) => {
            let mut info = analyze_closure_capture_info(ret_expr, env, expected)?;
            info.has_shared_deps = info.has_shared_deps || expects_shared_deps;
            info
        }
        None => return None,
    };
    if !info.captures_any {
        return None;
    }
    if expects_shared_deps && info.contains_spawn {
        return Some("E-CON-0131");
    }
    if info.captures_shared && !info.has_shared_deps {
        return Some("E-CON-0085");
    }
    None
}

/// A raw pointer into a region must not leave through an exported procedure.
fn check_ffi_boundary_region_local_raw_pointer_return(type_ctx: &StmtTypeContext<'_>, ret_expr: &ExprPtr, expected: &TypeRef) -> Option<&'static str> {
    if !type_ctx.ffi_export_boundary || ret_expr.is_none() || !type_may_be(expected, |node| matches!(node, TypeNode::RawPtr { .. })) {
        return None;
    }
    pending("FfiBoundaryReturn");
    None
}

/// A safe pointer to a local must not be returned.
fn check_returned_safe_pointer_provenance(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    ret_expr: &ExprPtr,
    expected: &TypeRef,
) -> Option<&'static str> {
    let expr = ret_expr.as_deref()?;
    if !type_contains_safe_pointer(expected) || matches!(expr.node, ExprNode::PtrNullExpr(_)) {
        return None;
    }
    let typed = type_expr_with_current_env(ctx, type_ctx, env, type_expr_fn, ret_expr);
    if !typed.ok {
        return typed.diag_id;
    }
    if is_safe_ptr_type(&typed.r#type) {
        pending("ReturnProvenance");
    }
    None
}

fn failed(diag_id: Option<&'static str>) -> StmtTypeResult {
    StmtTypeResult { diag_id, ..Default::default() }
}

pub fn type_return_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::ReturnStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> StmtTypeResult {
    let unit = |ty: &TypeRef| matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "()");
    // The checks every accepted value goes through, in the reference's order.
    let after_checks = |expected: &TypeRef| -> StmtTypeResult {
        let diag = check_escaping_closure_return(&node.value_opt, env, expected)
            .or_else(|| check_ffi_boundary_region_local_raw_pointer_return(type_ctx, &node.value_opt, expected))
            .or_else(|| check_returned_safe_pointer_provenance(ctx, type_ctx, env, type_expr_fn, &node.value_opt, expected))
            .or_else(|| verify_postcondition_at_return(ctx, type_ctx, env, type_expr_fn, &node.value_opt));
        match diag {
            Some(diag_id) => failed(Some(diag_id)),
            None => StmtTypeResult::typed(env.clone()),
        }
    };
    let without_value = |expected: &TypeRef, mismatch: &'static str| -> StmtTypeResult {
        if !unit(expected) {
            return failed(Some(mismatch));
        }
        match verify_postcondition_at_return(ctx, type_ctx, env, type_expr_fn, &node.value_opt) {
            Some(diag_id) => failed(Some(diag_id)),
            None => StmtTypeResult::typed(env.clone()),
        }
    };
    if let Some(async_sig) = async_sig_of(ctx, &type_ctx.return_type) {
        if node.value_opt.is_none() {
            return without_value(&async_sig.result, "E-CON-0203");
        }
        let check = check_expr_against(ctx, type_ctx, &return_dest_expr(&node.value_opt), &async_sig.result, env);
        if !check.ok {
            return if matches!(check.diag_id, None | Some("E-SEM-2526")) {
                StmtTypeResult { diag_id: Some("E-CON-0203"), diag_detail: check.diag_detail, diag_span: check.diag_span, ..Default::default() }
            } else {
                StmtTypeResult {
                    diag_id: check.diag_id,
                    diag_detail: check.diag_detail,
                    diag_span: check.diag_span,
                    diagnostic_obligation_ids: check.diagnostic_obligation_ids,
                    ..Default::default()
                }
            };
        }
        return after_checks(&async_sig.result);
    }
    if node.value_opt.is_none() {
        return without_value(&type_ctx.return_type, "E-SEM-3161");
    }
    if type_ctx.opaque_return {
        pending("OpaqueReturn");
        return failed(None);
    }
    let return_expr = return_dest_expr(&node.value_opt);
    let check = check_expr_against(ctx, type_ctx, &return_expr, &type_ctx.return_type, env);
    if !check.ok {
        let typed_ret = type_expr_with_current_env(ctx, type_ctx, env, type_expr_fn, &return_expr);
        if typed_ret.ok {
            match classify_outcome_intro(ctx, &typed_ret.r#type, &type_ctx.return_type) {
                OutcomeIntro::Ambiguous => {
                    return StmtTypeResult { diag_id: Some("E-TYP-2261"), diag_span: check.diag_span, ..Default::default() };
                }
                OutcomeIntro::Value | OutcomeIntro::Error => return StmtTypeResult::typed(env.clone()),
                OutcomeIntro::None => {}
            }
        }
        return if matches!(check.diag_id, None | Some("E-SEM-2526")) {
            StmtTypeResult { diag_id: Some("E-SEM-3161"), diag_detail: check.diag_detail, diag_span: check.diag_span, ..Default::default() }
        } else {
            StmtTypeResult {
                diag_id: check.diag_id,
                diag_detail: check.diag_detail,
                diag_span: check.diag_span,
                diagnostic_obligation_ids: check.diagnostic_obligation_ids,
                ..Default::default()
            }
        };
    }
    after_checks(&type_ctx.return_type)
}
