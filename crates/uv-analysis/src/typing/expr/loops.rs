//! The loops, and the `break` and `continue` statements inside them. A loop has the
//! type of the values its `break`s give.

use std::collections::HashMap;
use std::rc::Rc;

use uv_source::ast::{self, ExprPtr};

use super::loop_invariant::{loop_invariant_maintained_by_body, validate_loop_invariant_expr, violates_loop_invariant_maintenance};
use crate::context::ScopeContext;
use crate::generics::where_bounds::check_class_bound;
use crate::resolve::scopes::id_key_of;
use crate::typing::callbacks::{ExprTypeFn, IdentTypeFn};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt::assign_stmt::type_expr_with_current_env;
use crate::typing::stmt::block::{loop_type_fin, loop_type_inf, type_block_info, BlockInfoResult, FlowInfo, StmtTypeResult};
use crate::typing::stmt_context::{with_shared_access_mode, LoopFlag, StmtTypeContext};
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::{push_scope, type_pattern, TypeBinding, TypeEnv};
use crate::typing::type_expr::{type_expr, type_place};
use crate::typing::type_lookup::async_sig_of;
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

fn is_prim(ty: &TypeRef, name: &str) -> bool {
    matches!(strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(prim)) if prim == name)
}

/// Types a loop body, in which `break` and `continue` are allowed.
fn type_loop_body(
    ctx: &ScopeContext<'_>,
    loop_ctx: &StmtTypeContext<'_>,
    body: &ast::Block,
    env: &TypeEnv,
    type_ident: IdentTypeFn<'_>,
) -> BlockInfoResult {
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, loop_ctx, inner, env);
    let place_fn = |inner: &ExprPtr| type_place(ctx, loop_ctx, inner, env);
    type_block_info(ctx, loop_ctx, body, env, &expr_fn, type_ident, &place_fn, None)
}

fn body_failure(info: BlockInfoResult) -> ExprTypeResult {
    ExprTypeResult { diag_id: info.diag_id, diag_detail: info.diag_detail, diag_span: info.diag_span, ..Default::default() }
}

/// Checks a loop's invariant, if it has one: the invariant itself, and that the body
/// leaves alone the names it speaks of.
fn check_invariant(
    ctx: &ScopeContext<'_>,
    loop_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    invariant: &Option<ast::LoopInvariant>,
    body: &ast::BlockPtr,
) -> Option<&'static str> {
    let invariant = invariant.as_ref()?;
    if let diag @ Some(_) = validate_loop_invariant_expr(ctx, loop_ctx, env, invariant) {
        return diag;
    }
    (!loop_ctx.contract_dynamic && violates_loop_invariant_maintenance(invariant, body)).then_some("E-SEM-2831")
}

fn loop_result(loop_type: Option<TypeRef>, rule: &'static str) -> ExprTypeResult {
    match loop_type {
        Some(ty) => ExprTypeResult::typed(ty),
        None => ExprTypeResult::failed(Some(rule)),
    }
}

pub fn type_loop_infinite_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::LoopInfiniteExpr,
    env: &TypeEnv,
    type_ident: IdentTypeFn<'_>,
) -> ExprTypeResult {
    let Some(body) = expr.body.as_deref() else {
        return ExprTypeResult::default();
    };
    let mut loop_ctx = type_ctx.clone();
    loop_ctx.loop_flag = LoopFlag::Loop;
    let body_info = type_loop_body(ctx, &loop_ctx, body, env, type_ident);
    if !body_info.ok {
        return body_failure(body_info);
    }
    if let Some(diag_id) = check_invariant(ctx, &loop_ctx, env, &expr.invariant_opt, &expr.body) {
        return ExprTypeResult::failed(Some(diag_id));
    }
    loop_result(loop_type_inf(&body_info.breaks, body_info.break_void), "T-Loop-Infinite")
}

pub fn type_loop_conditional_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::LoopConditionalExpr,
    env: &TypeEnv,
    type_ident: IdentTypeFn<'_>,
) -> ExprTypeResult {
    let (Some(cond), Some(body)) = (expr.cond.as_deref(), expr.body.as_deref()) else {
        return ExprTypeResult::default();
    };
    let cond_type = type_expr(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), &expr.cond, env);
    if !cond_type.ok {
        return ExprTypeResult {
            diag_id: cond_type.diag_id,
            diag_detail: cond_type.diag_detail,
            diag_span: cond_type.diag_span.or_else(|| Some(cond.span.clone())),
            ..Default::default()
        };
    }
    if !is_prim(&cond_type.r#type, "bool") {
        return ExprTypeResult::failed(Some("Loop-Cond-NotBool"));
    }
    let mut loop_ctx = type_ctx.clone();
    loop_ctx.loop_flag = LoopFlag::Loop;
    let body_info = type_loop_body(ctx, &loop_ctx, body, env, type_ident);
    if !body_info.ok {
        return body_failure(body_info);
    }
    // A conditional loop may assign to the invariant's names if the invariant still
    // follows afterwards.
    if let Some(invariant) = &expr.invariant_opt {
        if let Some(diag_id) = validate_loop_invariant_expr(ctx, &loop_ctx, env, invariant) {
            return ExprTypeResult::failed(Some(diag_id));
        }
        if !loop_ctx.contract_dynamic && !loop_invariant_maintained_by_body(&loop_ctx, invariant, &expr.cond, &expr.body) {
            return ExprTypeResult::failed(Some("E-SEM-2831"));
        }
    }
    loop_result(loop_type_fin(&body_info.breaks, body_info.break_void), "T-Loop-Conditional")
}

/// The element a loop takes from an array, slice or range each time round.
fn iterable_element_type(ty: &TypeRef) -> Option<TypeRef> {
    match &strip_perm_and_refine(ty).as_deref()?.node {
        TypeNode::Array { element, .. } | TypeNode::Slice(element) => Some(element.clone()),
        TypeNode::Range(base) | TypeNode::RangeInclusive(base) | TypeNode::RangeFrom(base) => Some(base.clone()),
        _ => None,
    }
}

/// The environment with the pattern's bindings. The reference pushes a scope for them
/// and then writes them into the outermost scope; so does this.
fn intro_pattern_bindings(env: &TypeEnv, bindings: &[(String, TypeRef)]) -> TypeEnv {
    if bindings.is_empty() {
        return env.clone();
    }
    let mut new_env = push_scope(env);
    if let Some(outermost) = new_env.scopes.first_mut() {
        for (name, ty) in bindings {
            let binding = TypeBinding { r#mut: ast::Mutability::Let, r#type: ty.clone(), storage_type: ty.clone(), ..Default::default() };
            outermost.insert(name.clone(), binding);
        }
    }
    new_env
}

pub fn type_loop_iter_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::LoopIterExpr,
    env: &TypeEnv,
    type_ident: IdentTypeFn<'_>,
) -> ExprTypeResult {
    let failed = |diag_id: Option<&'static str>| ExprTypeResult::failed(diag_id);
    let Some(body) = expr.body.as_deref().filter(|_| expr.iter.is_some() && expr.pattern.is_some()) else {
        return ExprTypeResult::default();
    };
    let iter_type = type_expr(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), &expr.iter, env);
    if !iter_type.ok {
        return failed(iter_type.diag_id);
    }
    let stripped_iter = strip_perm_and_refine(&iter_type.r#type);
    let iter_node = stripped_iter.as_deref().map(|ty| &ty.node);
    let is_bounded_range = matches!(iter_node, Some(TypeNode::Range(_) | TypeNode::RangeInclusive(_)));
    let is_loop_range = is_bounded_range || matches!(iter_node, Some(TypeNode::RangeFrom(_)));

    let mut element_type = match async_sig_of(ctx, &iter_type.r#type) {
        // Looping over an asynchronous stream awaits each value, which only an
        // asynchronous procedure that can fail the same way may do.
        Some(iter_async_sig) => {
            if expr.type_opt.is_some() || expr.invariant_opt.is_some() {
                return failed(Some("E-CON-0240"));
            }
            let Some(proc_async_sig) = async_sig_of(ctx, &type_ctx.return_type).filter(|_| is_prim(&iter_async_sig.input, "()")) else {
                return failed(Some("E-CON-0240"));
            };
            let err_sub = subtyping(ctx, &iter_async_sig.err, &proc_async_sig.err);
            if !err_sub.ok {
                return failed(err_sub.diag_id);
            }
            if !err_sub.subtype {
                return failed(Some(err_sub.diag_id.unwrap_or("Loop-Async-Err")));
            }
            iter_async_sig.out
        }
        None => {
            let Some(element) = iterable_element_type(&iter_type.r#type) else {
                return failed(Some("E-SEM-3133"));
            };
            // A range is stepped through, and a bounded one compared with its end.
            if is_loop_range && !check_class_bound(ctx, &element, &["Discrete".to_string()]) {
                return failed(Some("E-SEM-3133"));
            }
            if is_bounded_range && !check_class_bound(ctx, &element, &["Eq".to_string()]) {
                return failed(Some("E-SEM-3133"));
            }
            element
        }
    };
    if expr.type_opt.is_some() {
        let annotation_type = match lower_type(ctx, &expr.type_opt) {
            Ok(lowered) => lowered,
            Err(diag_id) => return failed(diag_id),
        };
        let sub = subtyping(ctx, &element_type, &annotation_type);
        if !sub.ok {
            return failed(sub.diag_id);
        }
        if !sub.subtype {
            return failed(Some(sub.diag_id.unwrap_or("T-LetStmt-Ann-Mismatch")));
        }
        element_type = annotation_type;
    }
    let bindings = match type_pattern(ctx, &expr.pattern, &element_type) {
        Ok(bindings) => bindings,
        Err(diag_id) => return failed(diag_id),
    };
    let extended_env = intro_pattern_bindings(env, &bindings);

    let mut loop_ctx = type_ctx.clone();
    loop_ctx.loop_flag = LoopFlag::Loop;
    // Key blocks in the body may use the ranges the loop variables run over.
    let mut loop_iteration_ranges: HashMap<_, _> = type_ctx.loop_iteration_ranges.as_deref().cloned().unwrap_or_default();
    if is_loop_range {
        for (name, _) in &bindings {
            loop_iteration_ranges.insert(id_key_of(name), expr.iter.clone());
        }
    }
    loop_ctx.loop_iteration_ranges = Some(Rc::new(loop_iteration_ranges));

    let body_info = type_loop_body(ctx, &loop_ctx, body, &extended_env, type_ident);
    if !body_info.ok {
        return body_failure(body_info);
    }
    if let Some(diag_id) = check_invariant(ctx, &loop_ctx, &extended_env, &expr.invariant_opt, &expr.body) {
        return ExprTypeResult::failed(Some(diag_id));
    }
    loop_result(loop_type_fin(&body_info.breaks, body_info.break_void), "T-Loop-Iter")
}

pub fn type_break_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::BreakStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> StmtTypeResult {
    if type_ctx.loop_flag != LoopFlag::Loop {
        return StmtTypeResult { diag_id: Some("E-SEM-3162"), ..Default::default() };
    }
    let mut flow = FlowInfo::default();
    if node.value_opt.is_some() {
        let typed = type_expr_with_current_env(ctx, type_ctx, env, type_expr_fn, &node.value_opt);
        if !typed.ok {
            return StmtTypeResult { diag_id: typed.diag_id, diag_detail: typed.diag_detail, ..Default::default() };
        }
        flow.breaks.push(typed.r#type);
    } else {
        flow.break_void = true;
    }
    StmtTypeResult { ok: true, env: env.clone(), flow, ..Default::default() }
}

pub fn type_continue_stmt(type_ctx: &StmtTypeContext<'_>, env: &TypeEnv) -> StmtTypeResult {
    if type_ctx.loop_flag != LoopFlag::Loop {
        return StmtTypeResult { diag_id: Some("E-SEM-3163"), ..Default::default() };
    }
    StmtTypeResult::typed(env.clone())
}
