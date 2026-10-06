//! The asynchronous forms: `yield`, `yield from`, `sync`, `race`, `all` and `wait`.

use uv_source::ast;

use crate::caps::builtin_paths::path_matches_builtin_name;
use crate::context::ScopeContext;
use crate::resolve::scopes::id_key_of;
use crate::typing::callbacks::{ExprTypeFn, PlaceTypeFn};
use crate::typing::closure_capture::analyze_yield_usage;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::outcome::make_outcome_type;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::{mark_shared_derived_bindings_stale, push_scope, type_pattern, TypeBinding, TypeEnv};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::type_expr;
use crate::typing::type_lookup::async_sig_of;
use crate::typing::type_predicates::{strip_perm, strip_perm_and_refine};
use crate::typing::types::*;

fn failed(diag_id: &'static str) -> ExprTypeResult {
    ExprTypeResult::failed(Some(diag_id))
}

fn is_prim(ty: &TypeRef, name: &str) -> bool {
    matches!(strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(prim)) if prim == name)
}

/// Releasing the keys at a suspension point makes what was read under them stale.
fn release_keys(type_ctx: &StmtTypeContext<'_>, release: bool) {
    if let (true, Some(cell)) = (release, &type_ctx.env_ref) {
        mark_shared_derived_bindings_stale(&mut cell.borrow_mut());
    }
}

/// `yield value`: hands a value out of an asynchronous procedure and has the type of
/// what is sent back in.
pub fn type_yield_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::YieldExpr, type_expr_fn: ExprTypeFn<'_>) -> ExprTypeResult {
    let Some(async_sig) = async_sig_of(ctx, &type_ctx.return_type) else {
        return failed("E-CON-0210");
    };
    let value_result = type_expr_fn(&expr.value);
    if !value_result.ok {
        return ExprTypeResult::failed(value_result.diag_id);
    }
    let sub = subtyping(ctx, &value_result.r#type, &async_sig.out);
    if !sub.ok {
        return ExprTypeResult::failed(sub.diag_id);
    }
    if !sub.subtype {
        return failed("E-CON-0211");
    }
    if type_ctx.keys_held && !expr.release {
        return failed("E-CON-0213");
    }
    release_keys(type_ctx, expr.release);
    ExprTypeResult::typed(async_sig.input)
}

/// `yield from child`: runs a nested computation with the same values out and in, and
/// has the type of its result.
pub fn type_yield_from_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::YieldFromExpr,
    type_expr_fn: ExprTypeFn<'_>,
) -> ExprTypeResult {
    let Some(async_sig) = async_sig_of(ctx, &type_ctx.return_type) else {
        return failed("E-CON-0220");
    };
    let value_result = type_expr_fn(&expr.value);
    if !value_result.ok {
        return ExprTypeResult::failed(value_result.diag_id);
    }
    let Some(child_sig) = async_sig_of(ctx, &value_result.r#type) else {
        return failed("E-CON-0221");
    };
    if !type_equiv(&async_sig.out, &child_sig.out) {
        return failed("E-CON-0221");
    }
    if !type_equiv(&async_sig.input, &child_sig.input) {
        return failed("E-CON-0222");
    }
    let err_sub = subtyping(ctx, &child_sig.err, &async_sig.err);
    if !err_sub.ok {
        return ExprTypeResult::failed(err_sub.diag_id);
    }
    if !err_sub.subtype {
        return failed("E-CON-0225");
    }
    if type_ctx.keys_held && !expr.release {
        return failed("E-CON-0224");
    }
    release_keys(type_ctx, expr.release);
    ExprTypeResult::typed(child_sig.result)
}

/// `sync computation`: runs a computation that neither yields values nor takes input
/// to completion, outside asynchronous code.
pub fn type_sync_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::SyncExpr, type_expr_fn: ExprTypeFn<'_>) -> ExprTypeResult {
    let (has_yield, has_yield_from) = analyze_yield_usage(&expr.value);
    if has_yield {
        return failed("E-CON-0212");
    }
    if has_yield_from {
        return failed("E-CON-0223");
    }
    if async_sig_of(ctx, &type_ctx.return_type).is_some() {
        return failed("E-CON-0250");
    }
    let value_result = type_expr_fn(&expr.value);
    if !value_result.ok {
        return ExprTypeResult::failed(value_result.diag_id);
    }
    let Some(async_sig) = async_sig_of(ctx, &value_result.r#type).filter(|sig| is_prim(&sig.out, "()")) else {
        return failed("E-CON-0251");
    };
    if !is_prim(&async_sig.input, "()") {
        return failed("E-CON-0252");
    }
    ExprTypeResult::typed(make_outcome_type(async_sig.result, async_sig.err))
}

/// `all { a, b, … }`: every computation's result, or the first error.
pub fn type_all_expr(ctx: &ScopeContext<'_>, expr: &ast::AllExpr, type_expr_fn: ExprTypeFn<'_>) -> ExprTypeResult {
    let mut result_types = Vec::with_capacity(expr.exprs.len());
    let mut error_types = Vec::with_capacity(expr.exprs.len());
    for elem in &expr.exprs {
        let elem_result = type_expr_fn(elem);
        if !elem_result.ok {
            return ExprTypeResult::failed(elem_result.diag_id);
        }
        let Some(async_sig) = async_sig_of(ctx, &elem_result.r#type).filter(|sig| is_prim(&sig.out, "()")) else {
            return failed("E-CON-0270");
        };
        if !is_prim(&async_sig.input, "()") {
            return failed("E-CON-0271");
        }
        result_types.push(async_sig.result);
        error_types.push(async_sig.err);
    }
    let error_union = if error_types.is_empty() { make_type_prim("!") } else { make_type_union(error_types) };
    ExprTypeResult::typed(make_outcome_type(make_type_tuple(result_types), error_union))
}

/// A union as the type of a value known to be its first member. When the member is not
/// found the reference leaves the result marked ok without a type, and so does this.
fn union_of(first_member: &TypeRef, union_type: TypeRef) -> ExprTypeResult {
    let Some(TypeNode::Union(members)) = union_type.as_deref().map(|ty| &ty.node) else {
        return ExprTypeResult::typed(union_type);
    };
    if first_member.is_some() && members.iter().any(|member| type_equiv(first_member, member)) {
        return ExprTypeResult::typed(union_type);
    }
    ExprTypeResult { ok: true, ..Default::default() }
}

/// `race { a -> |x| handler, … }`: the handler of whichever computation finishes (or
/// yields) first. Handlers all return, or all yield.
pub fn type_race_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::RaceExpr,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> ExprTypeResult {
    if expr.arms.len() < 2 {
        return failed("E-CON-0260");
    }
    let yield_mode = expr.arms.iter().any(|arm| arm.handler.kind == ast::RaceHandlerKind::Yield);
    if yield_mode && expr.arms.iter().any(|arm| arm.handler.kind != ast::RaceHandlerKind::Yield) {
        return failed("E-CON-0263");
    }
    let mut handler_types: Vec<TypeRef> = Vec::with_capacity(expr.arms.len());
    let mut error_types: Vec<TypeRef> = Vec::with_capacity(expr.arms.len());
    for arm in &expr.arms {
        let expr_result = type_expr_fn(&arm.expr);
        if !expr_result.ok {
            return ExprTypeResult::failed(expr_result.diag_id);
        }
        let Some(async_sig) = async_sig_of(ctx, &expr_result.r#type) else {
            return failed("E-CON-0261");
        };
        if !yield_mode && !is_prim(&async_sig.out, "()") {
            return failed("E-CON-0262");
        }
        if !is_prim(&async_sig.input, "()") {
            return failed("E-CON-0261");
        }
        // The handler sees the result, or each value yielded.
        let pat_type = if yield_mode { &async_sig.out } else { &async_sig.result };
        let bindings = match type_pattern(ctx, &arm.pattern, pat_type) {
            Ok(bindings) => bindings,
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        };
        let mut inner = push_scope(env);
        if let Some(innermost) = inner.scopes.last_mut() {
            for (name, ty) in bindings {
                let key = id_key_of(&name);
                if !innermost.contains_key(&key) {
                    innermost.insert(key, TypeBinding { r#mut: ast::Mutability::Let, r#type: ty, ..Default::default() });
                }
            }
        }
        let handler_result = type_expr(ctx, type_ctx, &arm.handler.value, &inner);
        if !handler_result.ok {
            return ExprTypeResult::failed(handler_result.diag_id);
        }
        handler_types.push(handler_result.r#type);
        error_types.push(async_sig.err);
    }
    let first_handler = handler_types[0].clone();
    if !handler_types[1..].iter().all(|ty| type_equiv(&first_handler, ty)) {
        return failed("E-CON-0261");
    }
    if !yield_mode {
        return ExprTypeResult::typed(make_outcome_type(first_handler, make_type_union(error_types)));
    }
    let first_error = error_types[0].clone();
    let err_union = union_of(&first_error, make_type_union(error_types));
    if err_union.r#type.is_none() {
        return ExprTypeResult { ok: true, ..Default::default() };
    }
    ExprTypeResult::typed(make_type_path_with(vec!["Stream".to_string()], vec![first_handler, err_union.r#type]))
}

/// `wait handle`: the value of a spawned task, or of a tracked one or its error.
pub fn type_wait_expr(type_ctx: &StmtTypeContext<'_>, expr: &ast::WaitExpr, type_expr_fn: ExprTypeFn<'_>, type_place_fn: PlaceTypeFn<'_>) -> ExprTypeResult {
    if type_ctx.in_speculative {
        return failed("E-CON-0092");
    }
    if type_ctx.keys_held {
        return failed("E-CON-0133");
    }
    let handle_result = type_place_fn(&expr.handle);
    let handle_type = if handle_result.ok {
        handle_result.r#type
    } else {
        let value_result = type_expr_fn(&expr.handle);
        if !value_result.ok {
            return ExprTypeResult::failed(value_result.diag_id);
        }
        value_result.r#type
    };
    let stripped = strip_perm(&handle_type);
    let applied = stripped.as_deref().and_then(|ty| applied_type_path(ty).zip(applied_type_args(ty)));
    match applied {
        Some((path, [inner])) if path_matches_builtin_name(path, "Spawned") => ExprTypeResult::typed(inner.clone()),
        Some((path, [value, error])) if path_matches_builtin_name(path, "Tracked") => {
            union_of(value, make_type_union(vec![value.clone(), error.clone()]))
        }
        _ => failed("E-CON-0132"),
    }
}
