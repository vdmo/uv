//! Statements with a body of their own: `region`, `frame` and `defer`; and `using`,
//! which gives a local binding another name.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use super::block::{check_block, type_block_info, FlowInfo, StmtTypeResult};
use super::stmt_common::block_needs_key_access;
use crate::context::ScopeContext;
use crate::memory::regions::{innermost_active_region, region_active_type};
use crate::resolve::scopes::id_key_of;
use crate::typing::callbacks::{ExprTypeFn, IdentTypeFn, PlaceTypeFn};
use crate::typing::check_expr::check_expr;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, intro_all, project_type_env_to_depth, push_scope, TypeEnv};
use crate::typing::types::*;

fn failed(diag_id: Option<&'static str>) -> StmtTypeResult {
    StmtTypeResult { diag_id, ..Default::default() }
}

/// `region$0`, `region$1`, …: the first not bound in any scope.
fn fresh_region_name(env: &TypeEnv) -> String {
    (0usize..)
        .map(|index| format!("region${index}"))
        .find(|name| {
            let key = id_key_of(name);
            !env.scopes.iter().any(|scope| scope.contains_key(&key))
        })
        .unwrap_or_default()
}

fn region_active_type_ref() -> TypeRef {
    make_type_perm(Permission::Unique, make_type_modal_state(vec!["Region".to_string()], "Active", Vec::new()))
}

/// Types the body in its scope; what the body binds stays inside it.
#[allow(clippy::too_many_arguments)]
fn type_scoped_stmt_body(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    body: &ast::Block,
    scoped_env: &TypeEnv,
    outer_scope_depth: usize,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let body_env = Rc::new(RefCell::new(TypeEnv::default()));
    let info = type_block_info(ctx, type_ctx, body, scoped_env, type_expr_fn, type_ident_fn, type_place_fn, Some(&body_env));
    if !info.ok {
        return StmtTypeResult { diag_id: info.diag_id, diag_detail: info.diag_detail, diag_span: info.diag_span, ..Default::default() };
    }
    let projected_env = project_type_env_to_depth(&body_env.borrow(), outer_scope_depth);
    if let Some(cell) = &type_ctx.env_ref {
        *cell.borrow_mut() = projected_env.clone();
    }
    let mut flow = FlowInfo { breaks: info.breaks, break_void: info.break_void, ..Default::default() };
    if matches!(info.r#type.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!") {
        flow.results.push(info.r#type);
    }
    StmtTypeResult { ok: true, env: projected_env, flow, ..Default::default() }
}

/// `RegionOptions()`, the options of a region that gives none.
fn make_default_region_options_expr() -> ExprPtr {
    let callee = Some(Arc::new(ast::Expr {
        span: Default::default(),
        node: ExprNode::IdentifierExpr(ast::IdentifierExpr { name: "RegionOptions".to_string(), ..Default::default() }),
    }));
    Some(Arc::new(ast::Expr { span: Default::default(), node: ExprNode::CallExpr(ast::CallExpr { callee, ..Default::default() }) }))
}

/// `region [opts] [as name] { … }`: the body runs with a new active region bound.
pub fn type_region_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::RegionStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let opts_expr = if node.opts_opt.is_some() { node.opts_opt.clone() } else { make_default_region_options_expr() };
    let options_type = make_type_path(vec!["RegionOptions".to_string()]);
    let check = check_expr(ctx, &opts_expr, &options_type, type_expr_fn, Some(type_place_fn), type_ident_fn, None, None);
    if !check.ok {
        return failed(check.diag_id);
    }
    let region_env = push_scope(env);
    let name = node.alias_opt.clone().unwrap_or_else(|| fresh_region_name(&region_env));
    let intro_env = match intro_all(&region_env, &[(name, region_active_type_ref())], ast::Mutability::Let, false) {
        Ok(intro_env) => intro_env,
        Err(diag_id) => return failed(diag_id),
    };
    let Some(body) = node.body.as_deref() else {
        return failed(None);
    };
    type_scoped_stmt_body(ctx, type_ctx, body, &intro_env, env.scopes.len(), type_expr_fn, type_ident_fn, type_place_fn)
}

/// `frame [target] { … }`: a nested frame of the named region, or of the innermost
/// active one.
pub fn type_frame_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::FrameStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let frame_env = push_scope(env);
    match &node.target_opt {
        None => {
            if innermost_active_region(&frame_env).is_none() {
                return failed(Some("Frame-NoActiveRegion-Err"));
            }
        }
        Some(target) => match bind_of(&frame_env, target) {
            None => return failed(Some("ResolveExpr-Ident-Err")),
            Some(binding) if !region_active_type(&binding.r#type) => return failed(Some("Frame-Target-NotActive-Err")),
            Some(_) => {}
        },
    }
    let fresh = fresh_region_name(&frame_env);
    let intro_env = match intro_all(&frame_env, &[(fresh, region_active_type_ref())], ast::Mutability::Let, false) {
        Ok(intro_env) => intro_env,
        Err(_) => return failed(None),
    };
    let Some(body) = node.body.as_deref() else {
        return failed(None);
    };
    type_scoped_stmt_body(ctx, type_ctx, body, &intro_env, env.scopes.len(), type_expr_fn, type_ident_fn, type_place_fn)
}

fn defer_has_non_local_ctrl_block(block: &ast::BlockPtr, in_loop: bool) -> bool {
    block.as_deref().is_some_and(|block| {
        block.stmts.iter().any(|stmt| match stmt {
            Stmt::ReturnStmt(_) => true,
            Stmt::BreakStmt(_) | Stmt::ContinueStmt(_) => !in_loop,
            Stmt::DeferStmt(node) => defer_has_non_local_ctrl_block(&node.body, in_loop),
            _ => false,
        }) || defer_has_non_local_ctrl_expr(&block.tail_opt, in_loop)
    })
}

/// As deferral sees it: only what a block's statements and its tail do directly.
fn defer_has_non_local_ctrl_expr(expr: &ExprPtr, in_loop: bool) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::LoopInfiniteExpr(node)) => defer_has_non_local_ctrl_block(&node.body, true),
        Some(ExprNode::LoopConditionalExpr(node)) => defer_has_non_local_ctrl_block(&node.body, true),
        Some(ExprNode::LoopIterExpr(node)) => defer_has_non_local_ctrl_block(&node.body, true),
        Some(ExprNode::BlockExpr(node)) => defer_has_non_local_ctrl_block(&node.block, in_loop),
        _ => false,
    }
}

/// `defer { … }`: a unit block that takes no keys and does not leave by itself.
pub fn type_defer_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::DeferStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let Some(body) = node.body.as_deref() else {
        return failed(None);
    };
    if type_ctx.in_speculative {
        return failed(Some("E-CON-0093"));
    }
    if block_needs_key_access(ctx, type_ctx, body, env) {
        return failed(Some("E-CON-0006"));
    }
    let env_ref = type_ctx.env_ref.clone();
    let check = check_block(ctx, type_ctx, body, env, &make_type_prim("()"), type_expr_fn, type_ident_fn, type_place_fn, env_ref.as_ref());
    if !check.ok {
        return failed(match check.diag_id {
            None | Some("E-SEM-2526") => Some("E-SEM-3151"),
            other => other,
        });
    }
    if defer_has_non_local_ctrl_block(&node.body, false) {
        return failed(Some("E-SEM-3152"));
    }
    StmtTypeResult::typed(env.clone())
}

/// `using source as alias`: the alias is the same binding under another name.
pub fn type_using_local_stmt(stmt: &ast::UsingLocalStmt, env: &TypeEnv) -> StmtTypeResult {
    let Some(source_binding) = bind_of(env, &stmt.source).filter(|_| !env.scopes.is_empty()) else {
        return StmtTypeResult::failed(Some("ResolveExpr-Ident-Err"), env);
    };
    let mut out_env = env.clone();
    if let Some(innermost) = out_env.scopes.last_mut() {
        innermost.insert(stmt.alias.clone(), source_binding.clone());
    }
    StmtTypeResult::typed(out_env)
}
