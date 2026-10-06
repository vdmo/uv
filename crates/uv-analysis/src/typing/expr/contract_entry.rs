//! `@entry(expr)`: the value of an expression as the procedure was entered. Valid only
//! in a postcondition, over bindings that exist at entry, without capabilities or side
//! effects, and of a `Bitcopy` type so the entry value can be kept.

use uv_source::ast::{self, ExprNode, ExprPtr};

use crate::context::ScopeContext;
use crate::contracts::intrinsics::{is_context_capability_call, is_likely_capability_receiver, type_contains_capability};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::{ContractPhase, StmtTypeContext};
use crate::typing::type_env::{collect_pat_names, TypeBinding, TypeEnv};
use crate::typing::type_expr::type_expr;
use crate::typing::type_predicates::bitcopy_type;

/// The sub-expressions the capability and side-effect walks share.
fn core_children(e: &ast::Expr) -> Vec<&ExprPtr> {
    match &e.node {
        ExprNode::BinaryExpr(node) => vec![&node.lhs, &node.rhs],
        ExprNode::UnaryExpr(node) => vec![&node.value],
        ExprNode::FieldAccessExpr(node) => vec![&node.base],
        ExprNode::TupleAccessExpr(node) => vec![&node.base],
        ExprNode::IndexAccessExpr(node) => vec![&node.base, &node.index],
        ExprNode::CallExpr(node) => std::iter::once(&node.callee).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::MethodCallExpr(node) => std::iter::once(&node.receiver).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::DerefExpr(node) => vec![&node.value],
        ExprNode::CastExpr(node) => vec![&node.value],
        ExprNode::IfExpr(node) => vec![&node.cond, &node.then_expr, &node.else_expr],
        ExprNode::TupleExpr(node) => node.elements.iter().collect(),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node),
        ExprNode::ArrayRepeatExpr(node) => vec![&node.value, &node.count],
        ExprNode::RecordExpr(node) => node.fields.iter().map(|field| &field.value).collect(),
        ExprNode::IfCaseExpr(node) => std::iter::once(&node.scrutinee).chain(node.cases.iter().map(|clause| &clause.body)).chain([&node.else_expr]).collect(),
        ExprNode::IfIsExpr(node) => vec![&node.scrutinee, &node.then_expr, &node.else_expr],
        ExprNode::AttributedExpr(node) => vec![&node.expr],
        ExprNode::AddressOfExpr(node) => vec![&node.place],
        ExprNode::EntryExpr(node) => vec![&node.expr],
        _ => Vec::new(),
    }
}

fn expr_contains_capability_op(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, env: &TypeEnv, expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    if let ExprNode::MethodCallExpr(node) = &e.node {
        if is_likely_capability_receiver(&node.receiver) || is_context_capability_call(&node.receiver, &node.name) {
            return true;
        }
        if node.receiver.is_some() {
            let typed = type_expr(ctx, type_ctx, &node.receiver, env);
            if typed.ok && type_contains_capability(&typed.r#type) {
                return true;
            }
        }
    }
    let mut children = core_children(e);
    match &e.node {
        ExprNode::MoveExpr(node) => children.push(&node.place),
        ExprNode::AllocExpr(node) => children.push(&node.value),
        ExprNode::PropagateExpr(node) => children.push(&node.value),
        ExprNode::SyncExpr(node) => children.push(&node.value),
        ExprNode::YieldExpr(node) => children.push(&node.value),
        ExprNode::YieldFromExpr(node) => children.push(&node.value),
        ExprNode::WaitExpr(node) => children.push(&node.handle),
        _ => {}
    }
    children.into_iter().any(|child| expr_contains_capability_op(ctx, type_ctx, env, child))
}

fn expr_contains_side_effect_op(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    if matches!(
        e.node,
        ExprNode::SyncExpr(_)
            | ExprNode::YieldExpr(_)
            | ExprNode::YieldFromExpr(_)
            | ExprNode::SpawnExpr(_)
            | ExprNode::WaitExpr(_)
            | ExprNode::FenceExpr(_)
            | ExprNode::ParallelExpr(_)
            | ExprNode::DispatchExpr(_)
            | ExprNode::RaceExpr(_)
            | ExprNode::AllExpr(_)
            | ExprNode::MoveExpr(_)
            | ExprNode::AllocExpr(_)
            | ExprNode::TransmuteExpr(_)
            | ExprNode::PropagateExpr(_)
            | ExprNode::LoopInfiniteExpr(_)
            | ExprNode::LoopConditionalExpr(_)
            | ExprNode::LoopIterExpr(_)
            | ExprNode::BlockExpr(_)
            | ExprNode::UnsafeBlockExpr(_)
    ) {
        return true;
    }
    core_children(e).into_iter().any(expr_contains_side_effect_op)
}

/// Pattern bindings of an arm are in scope, without a type, for what follows.
fn env_with_pattern_bindings(env: &TypeEnv, pattern: &ast::PatternPtr) -> TypeEnv {
    let Some(pattern) = pattern.as_deref() else {
        return env.clone();
    };
    let mut extended = env.clone();
    if extended.scopes.is_empty() {
        extended.scopes.push(Default::default());
    }
    let mut names = Vec::new();
    collect_pat_names(pattern, &mut names);
    for name in names {
        extended.scopes.last_mut().expect("a scope").entry(name).or_insert_with(TypeBinding::default);
    }
    extended
}

/// Whether every name the expression uses is bound at entry.
fn expr_uses_only_entry_env_bindings(expr: &ExprPtr, env: &TypeEnv) -> bool {
    let Some(e) = expr.as_deref() else {
        return true;
    };
    match &e.node {
        ExprNode::IdentifierExpr(node) => return crate::typing::type_env::bind_of(env, &node.name).is_some(),
        ExprNode::ResultExpr(_) => return false,
        ExprNode::IfCaseExpr(node) => {
            return expr_uses_only_entry_env_bindings(&node.scrutinee, env)
                && node.cases.iter().all(|clause| expr_uses_only_entry_env_bindings(&clause.body, &env_with_pattern_bindings(env, &clause.pattern)))
                && expr_uses_only_entry_env_bindings(&node.else_expr, env);
        }
        ExprNode::IfIsExpr(node) => {
            let then_env = env_with_pattern_bindings(env, &node.pattern);
            return expr_uses_only_entry_env_bindings(&node.scrutinee, env)
                && expr_uses_only_entry_env_bindings(&node.then_expr, &then_env)
                && expr_uses_only_entry_env_bindings(&node.else_expr, env);
        }
        ExprNode::BinaryExpr(_)
        | ExprNode::UnaryExpr(_)
        | ExprNode::FieldAccessExpr(_)
        | ExprNode::TupleAccessExpr(_)
        | ExprNode::IndexAccessExpr(_)
        | ExprNode::CallExpr(_)
        | ExprNode::MethodCallExpr(_)
        | ExprNode::DerefExpr(_)
        | ExprNode::CastExpr(_)
        | ExprNode::IfExpr(_) => {}
        _ => return true,
    }
    core_children(e).into_iter().all(|child| expr_uses_only_entry_env_bindings(child, env))
}

/// See `TypeEntryExprImpl`.
pub fn type_entry_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, node: &ast::EntryExpr, env: &TypeEnv) -> ExprTypeResult {
    if type_ctx.contract_phase != ContractPhase::Postcondition {
        return ExprTypeResult::failed(Some("E-SEM-2852"));
    }
    if node.expr.is_none() {
        return ExprTypeResult::default();
    }
    if !expr_uses_only_entry_env_bindings(&node.expr, env) {
        return ExprTypeResult::failed(Some("E-SEM-2852"));
    }
    if expr_contains_capability_op(ctx, type_ctx, env, &node.expr) {
        return ExprTypeResult::failed(Some("E-CON-0415"));
    }
    if expr_contains_side_effect_op(&node.expr) {
        return ExprTypeResult::failed(Some("E-CON-0416"));
    }
    let typed = type_expr(ctx, type_ctx, &node.expr, env);
    if !typed.ok {
        return ExprTypeResult::failed(typed.diag_id);
    }
    if !bitcopy_type(ctx, &typed.r#type) {
        return ExprTypeResult::failed(Some("E-SEM-2805"));
    }
    ExprTypeResult::typed(typed.r#type)
}
