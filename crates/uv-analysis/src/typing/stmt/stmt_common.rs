//! What statements share: whether an expression reaches shared data, and the capture
//! facts recorded for bindings of closures.

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::ScopeContext;
use crate::typing::pending::pending;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{ClosureCaptureInfo, TypeEnv};
use crate::typing::type_expr::{type_expr, type_place};
use crate::typing::type_predicates::perm_of_type;
use crate::typing::types::{Permission, TypeRef};

/// A copy of the context for queries: they emit nothing and update no environment.
pub fn read_only<'t>(type_ctx: &StmtTypeContext<'t>) -> StmtTypeContext<'t> {
    StmtTypeContext {
        diags: None,
        env_ref: None,
        ..type_ctx.clone()
    }
}

fn expr_has_shared_permission(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    env: &TypeEnv,
) -> bool {
    if expr.is_none() {
        return false;
    }
    let query_ctx = read_only(type_ctx);
    let place = type_place(ctx, &query_ctx, expr, env);
    if place.ok && place.r#type.is_some() && perm_of_type(&place.r#type) == Permission::Shared {
        return true;
    }
    let value = type_expr(ctx, &query_ctx, expr, env);
    value.ok && value.r#type.is_some() && perm_of_type(&value.r#type) == Permission::Shared
}

fn stmt_needs_key_access(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    stmt: &Stmt,
    env: &TypeEnv,
) -> bool {
    let block = |body: &ast::BlockPtr| {
        body.as_deref()
            .is_some_and(|body| block_needs_key_access(ctx, type_ctx, body, env))
    };
    let expr = |value: &ExprPtr| expr_needs_key_access(ctx, type_ctx, value, env);
    match stmt {
        Stmt::LetStmt(node) => expr(&node.binding.init),
        Stmt::VarStmt(node) => expr(&node.binding.init),
        Stmt::UsingLocalStmt(_) => false,
        Stmt::AssignStmt(node) => {
            expr_has_shared_permission(ctx, type_ctx, &node.place, env)
                || expr(&node.place)
                || expr(&node.value)
        }
        Stmt::CompoundAssignStmt(node) => {
            expr_has_shared_permission(ctx, type_ctx, &node.place, env)
                || expr(&node.place)
                || expr(&node.value)
        }
        Stmt::ExprStmt(node) => expr(&node.value),
        Stmt::DeferStmt(node) => block(&node.body),
        Stmt::UnsafeBlockStmt(node) => block(&node.body),
        Stmt::CtStmt(node) => block(&node.body),
        Stmt::RegionStmt(node) => expr(&node.opts_opt) || block(&node.body),
        Stmt::FrameStmt(node) => block(&node.body),
        Stmt::ReturnStmt(node) => expr(&node.value_opt),
        Stmt::BreakStmt(node) => expr(&node.value_opt),
        Stmt::KeyBlockStmt(_) => true,
        Stmt::ContinueStmt(_) | Stmt::ErrorStmt(_) => false,
    }
}

fn block_needs_key_access(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    block: &ast::Block,
    env: &TypeEnv,
) -> bool {
    block
        .stmts
        .iter()
        .any(|stmt| stmt_needs_key_access(ctx, type_ctx, stmt, env))
        || expr_needs_key_access(ctx, type_ctx, &block.tail_opt, env)
}

/// Whether evaluating the expression reads or writes shared data, so that a binding of
/// its value goes stale when the keys are released.
pub fn expr_needs_key_access(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    env: &TypeEnv,
) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    if expr_has_shared_permission(ctx, type_ctx, expr, env) {
        return true;
    }
    let sub = |value: &ExprPtr| expr_needs_key_access(ctx, type_ctx, value, env);
    let block = |body: &ast::BlockPtr| {
        body.as_deref()
            .is_some_and(|body| block_needs_key_access(ctx, type_ctx, body, env))
    };
    match &e.node {
        ExprNode::CallExpr(node) => {
            sub(&node.callee) || node.args.iter().any(|arg| sub(&arg.value))
        }
        ExprNode::MethodCallExpr(node) => {
            sub(&node.receiver) || node.args.iter().any(|arg| sub(&arg.value))
        }
        ExprNode::BinaryExpr(node) => sub(&node.lhs) || sub(&node.rhs),
        ExprNode::PipelineExpr(node) => sub(&node.lhs) || sub(&node.rhs),
        ExprNode::UnaryExpr(node) => sub(&node.value),
        ExprNode::CastExpr(node) => sub(&node.value),
        ExprNode::DerefExpr(node) => sub(&node.value),
        ExprNode::PropagateExpr(node) => sub(&node.value),
        ExprNode::YieldExpr(node) => sub(&node.value),
        ExprNode::YieldFromExpr(node) => sub(&node.value),
        ExprNode::SyncExpr(node) => sub(&node.value),
        ExprNode::AddressOfExpr(node) => sub(&node.place),
        ExprNode::MoveExpr(node) => sub(&node.place),
        ExprNode::FieldAccessExpr(node) => sub(&node.base),
        ExprNode::TupleAccessExpr(node) => sub(&node.base),
        ExprNode::IndexAccessExpr(node) => sub(&node.base) || sub(&node.index),
        ExprNode::TupleExpr(node) => node.elements.iter().any(sub),
        ExprNode::ArrayExpr(node) => {
            // Every element is visited even after one is found, as the reference does.
            let mut found = false;
            for elem in ast::array_expr_subexprs(node) {
                found = found || sub(elem);
            }
            found
        }
        ExprNode::ArrayRepeatExpr(node) => sub(&node.value) || sub(&node.count),
        ExprNode::RecordExpr(node) => node.fields.iter().any(|field| sub(&field.value)),
        ExprNode::IfExpr(node) => sub(&node.cond) || sub(&node.then_expr) || sub(&node.else_expr),
        ExprNode::IfCaseExpr(node) => {
            sub(&node.scrutinee)
                || node.cases.iter().any(|arm| sub(&arm.body))
                || sub(&node.else_expr)
        }
        ExprNode::IfIsExpr(node) => {
            sub(&node.scrutinee) || sub(&node.then_expr) || sub(&node.else_expr)
        }
        ExprNode::BlockExpr(node) => block(&node.block),
        ExprNode::UnsafeBlockExpr(node) => block(&node.block),
        _ => false,
    }
}

/// What a closure bound by `let` or `var` captures. Only closures have capture facts;
/// their analysis is not ported yet.
pub fn analyze_closure_capture_info(
    expr: &ExprPtr,
    env: &TypeEnv,
    closure_type_hint: &TypeRef,
) -> Option<ClosureCaptureInfo> {
    let _ = (env, closure_type_hint);
    if let Some(ExprNode::ClosureExpr(_)) = expr.as_deref().map(|e| &e.node) {
        pending("ClosureCapture");
    }
    None
}
