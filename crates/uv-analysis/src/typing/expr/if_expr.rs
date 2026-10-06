//! Typing `if` expressions.
//!
//! The condition is a `bool`. When it is pure, each branch is typed knowing it: a
//! comparison of a binding narrows the binding's type by a refinement, and the
//! condition (negated for `else`) joins the facts proofs may use. Without `else` the
//! then-branch must be `()`; with it the branches must agree, `!` yielding to the other.

use std::rc::Rc;
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr};

use crate::context::{IdKey, ScopeContext};
use crate::contracts::purity::is_pure_in_scope;
use crate::contracts::verification::{extend_proof_context_with_predicate_at, negated_predicate};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::callbacks::CheckResult;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::pending::pending;
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::{gpu_context, TypeEnv};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, type_expr};
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

fn make_expr(span: &Span, node: ExprNode) -> ExprPtr {
    Some(Arc::new(ast::Expr { span: span.clone(), node }))
}

fn is_comparison_operator(op: &str) -> bool {
    ["==", "!=", "<", "<=", ">", ">="].iter().any(|candidate| id_eq(op, candidate))
}

/// The operator with its operands exchanged.
fn flip_comparison_operator(op: &str) -> Option<String> {
    let flipped = match op {
        _ if id_eq(op, "<") => ">",
        _ if id_eq(op, "<=") => ">=",
        _ if id_eq(op, ">") => "<",
        _ if id_eq(op, ">=") => "<=",
        _ if id_eq(op, "==") || id_eq(op, "!=") => op,
        _ => return None,
    };
    Some(flipped.to_string())
}

/// What a condition says about bindings when it holds: for each conjunct that compares
/// a name with something, a predicate on that name, written over `self`.
fn collect_then_branch_facts(cond: &ExprPtr, out: &mut Vec<(IdKey, ExprPtr)>) {
    let Some(cond_expr) = cond.as_deref() else {
        return;
    };
    let ExprNode::BinaryExpr(binary) = &cond_expr.node else {
        return;
    };
    if id_eq(&binary.op, "&&") {
        collect_then_branch_facts(&binary.lhs, out);
        collect_then_branch_facts(&binary.rhs, out);
        return;
    }
    if !is_comparison_operator(&binary.op) {
        return;
    }
    let ident = |expr: &ExprPtr| match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IdentifierExpr(ident)) => Some(id_key_of(&ident.name)),
        _ => None,
    };
    let (binding_name, op, other) = if let Some(name) = ident(&binary.lhs) {
        (name, binary.op.clone(), &binary.rhs)
    } else if let Some(name) = ident(&binary.rhs) {
        let Some(flipped) = flip_comparison_operator(&binary.op) else {
            return;
        };
        (name, flipped, &binary.lhs)
    } else {
        return;
    };
    if other.is_none() {
        return;
    }
    let self_expr = make_expr(&cond_expr.span, ExprNode::IdentifierExpr(ast::IdentifierExpr { name: "self".to_string(), from_splice: false }));
    let predicate = ast::BinaryExpr { op, lhs: self_expr, rhs: other.clone() };
    out.push((binding_name, make_expr(&cond_expr.span, ExprNode::BinaryExpr(predicate))));
}

/// The type refined by one more predicate.
fn add_refinement_fact(original: &TypeRef, predicate: &ExprPtr) -> TypeRef {
    if original.is_none() || predicate.is_none() {
        return original.clone();
    }
    match original.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Refine { base, predicate: existing }) => {
            let joined = match existing.as_deref() {
                None => predicate.clone(),
                Some(lhs) => {
                    let and = ast::BinaryExpr { op: "&&".to_string(), lhs: existing.clone(), rhs: predicate.clone() };
                    make_expr(&lhs.span, ExprNode::BinaryExpr(and))
                }
            };
            make_type_refine(base.clone(), joined)
        }
        _ => make_type_refine(original.clone(), predicate.clone()),
    }
}

fn refine_env_from_condition_facts(env: &TypeEnv, cond: &ExprPtr) -> TypeEnv {
    let mut out = env.clone();
    let mut facts = Vec::new();
    collect_then_branch_facts(cond, &mut facts);
    for (binding_name, predicate) in &facts {
        if let Some(binding) = out.scopes.iter_mut().rev().find_map(|scope| scope.get_mut(binding_name)) {
            binding.r#type = add_refinement_fact(&binding.r#type, predicate);
        }
    }
    out
}

fn is_never(ty: &TypeRef) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!")
}

fn is_bool_type(ty: &TypeRef) -> bool {
    matches!(strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "bool")
}

/// The type both branches have: `!` yields to the other, otherwise the wider of the two.
fn unify_branch_types(ctx: &ScopeContext<'_>, then_type: &TypeRef, else_type: &TypeRef) -> TypeRef {
    if is_never(then_type) {
        return else_type.clone();
    }
    if is_never(else_type) || type_equiv(then_type, else_type) {
        return then_type.clone();
    }
    let sub = subtyping(ctx, then_type, else_type);
    if sub.ok && sub.subtype {
        return else_type.clone();
    }
    let sub = subtyping(ctx, else_type, then_type);
    if sub.ok && sub.subtype {
        return then_type.clone();
    }
    None
}

/// A then-branch that is not `()` where there is no `else`.
fn if_no_else_diag_or_fallback(diag_id: Option<&'static str>) -> Option<&'static str> {
    diag_id.filter(|diag_id| *diag_id != "E-SEM-2526").or(Some("If-Branch-Mismatch"))
}

/// The condition typed, and the context and environment each branch is typed in.
struct Branches<'t> {
    then_ctx: StmtTypeContext<'t>,
    else_ctx: StmtTypeContext<'t>,
    then_env: TypeEnv,
    else_env: TypeEnv,
}

fn prepare_branches<'t>(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'t>,
    expr: &ast::IfExpr,
    env: &TypeEnv,
) -> Result<Branches<'t>, (Option<&'static str>, String)> {
    let cond_type = type_expr(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), &expr.cond, env);
    if !cond_type.ok {
        return Err((cond_type.diag_id, cond_type.diag_detail));
    }
    if !is_bool_type(&cond_type.r#type) {
        return Err((Some("If-Cond-NotBool"), String::new()));
    }
    // On a GPU both branches must reach the same barriers.
    if gpu_context(env) {
        pending("GpuBarrier");
    }
    let mut branches =
        Branches { then_ctx: type_ctx.clone(), else_ctx: type_ctx.clone(), then_env: env.clone(), else_env: env.clone() };
    if is_pure_in_scope(ctx, &expr.cond) {
        let extend = |predicate: &ExprPtr| {
            let location = predicate.as_deref().map(|predicate| predicate.span.clone()).unwrap_or_default();
            extend_proof_context_with_predicate_at(type_ctx.proof_ctx.as_deref(), predicate, &location).map(Rc::new)
        };
        branches.then_env = refine_env_from_condition_facts(env, &expr.cond);
        branches.then_ctx.proof_ctx = extend(&expr.cond);
        if let Some(else_cond @ Some(_)) = negated_predicate(&expr.cond) {
            branches.else_env = refine_env_from_condition_facts(env, &else_cond);
            branches.else_ctx.proof_ctx = extend(&else_cond);
        }
    }
    Ok(branches)
}

pub fn type_if_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::IfExpr, env: &TypeEnv) -> ExprTypeResult {
    let Some(then_expr) = expr.then_expr.as_deref().filter(|_| expr.cond.is_some()) else {
        return ExprTypeResult::default();
    };
    let branches = match prepare_branches(ctx, type_ctx, expr, env) {
        Ok(branches) => branches,
        Err((diag_id, diag_detail)) => return ExprTypeResult { diag_id, diag_detail, ..Default::default() },
    };
    if expr.else_expr.is_none() {
        let then_check = check_expr_against(ctx, &branches.then_ctx, &expr.then_expr, &make_type_prim("()"), &branches.then_env);
        if !then_check.ok {
            return ExprTypeResult {
                diag_id: if_no_else_diag_or_fallback(then_check.diag_id),
                diag_detail: then_check.diag_detail,
                diag_span: then_check.diag_span.or_else(|| Some(then_expr.span.clone())),
                ..Default::default()
            };
        }
        return ExprTypeResult::typed(make_type_prim("()"));
    }
    let then_type = type_expr(ctx, &branches.then_ctx, &expr.then_expr, &branches.then_env);
    if !then_type.ok {
        return ExprTypeResult { diag_id: then_type.diag_id, diag_detail: then_type.diag_detail, ..Default::default() };
    }
    let else_type = type_expr(ctx, &branches.else_ctx, &expr.else_expr, &branches.else_env);
    if !else_type.ok {
        return ExprTypeResult { diag_id: else_type.diag_id, diag_detail: else_type.diag_detail, ..Default::default() };
    }
    match unify_branch_types(ctx, &then_type.r#type, &else_type.r#type) {
        unified @ Some(_) => ExprTypeResult::typed(unified),
        None => ExprTypeResult::failed(Some("If-Branch-Mismatch")),
    }
}

/// A branch that failed its check, with which branch it was and what was expected.
fn branch_failure(which: &str, expected: &TypeRef, check: CheckResult) -> CheckResult {
    let mut diag_detail = format!("if {which} branch failed checking against expected {}", type_to_string(expected));
    if !check.diag_detail.is_empty() {
        diag_detail.push_str(": ");
        diag_detail.push_str(&check.diag_detail);
    }
    CheckResult { ok: false, diag_id: check.diag_id, diag_detail, diag_span: check.diag_span, ..Default::default() }
}

pub fn check_if_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::IfExpr,
    expected: &TypeRef,
    env: &TypeEnv,
) -> CheckResult {
    let Some(then_expr) = expr.then_expr.as_deref().filter(|_| expr.cond.is_some() && expected.is_some()) else {
        return CheckResult::default();
    };
    let branches = match prepare_branches(ctx, type_ctx, expr, env) {
        Ok(branches) => branches,
        Err((diag_id, diag_detail)) => return CheckResult { diag_id, diag_detail, ..Default::default() },
    };
    let then_check = check_expr_against(ctx, &branches.then_ctx, &expr.then_expr, expected, &branches.then_env);
    if !then_check.ok {
        return branch_failure("then", expected, then_check);
    }
    if expr.else_expr.is_none() {
        // Without `else` the value is `()` whenever the condition fails.
        let unit_type = make_type_prim("()");
        let then_check = check_expr_against(ctx, &branches.then_ctx, &expr.then_expr, &unit_type, &branches.then_env);
        if !then_check.ok {
            return CheckResult {
                diag_id: if_no_else_diag_or_fallback(then_check.diag_id),
                diag_detail: then_check.diag_detail,
                diag_span: then_check.diag_span.or_else(|| Some(then_expr.span.clone())),
                ..Default::default()
            };
        }
        let sub = subtyping(ctx, &unit_type, expected);
        if !sub.ok {
            return CheckResult { diag_id: sub.diag_id, ..Default::default() };
        }
        if !sub.subtype {
            return CheckResult { diag_id: if_no_else_diag_or_fallback(sub.diag_id), ..Default::default() };
        }
        return CheckResult { ok: true, ..Default::default() };
    }
    let else_check = check_expr_against(ctx, &branches.else_ctx, &expr.else_expr, expected, &branches.else_env);
    if !else_check.ok {
        return branch_failure("else", expected, else_check);
    }
    CheckResult { ok: true, ..Default::default() }
}
