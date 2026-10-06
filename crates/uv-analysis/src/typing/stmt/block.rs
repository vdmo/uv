//! Typing blocks and statement sequences.
//!
//! A block opens a scope, types its statements in order threading the environment
//! through them, and has the type of its tail expression; without a tail it has the
//! type its statements result in, `!` after a final `return`, or `()`.

use std::cell::RefCell;
use std::rc::Rc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::ScopeContext;
use crate::typing::callbacks::{CheckResult, ExprTypeFn, IdentTypeFn, PlaceTypeFn, PlaceTypeResult};
use crate::typing::subtyping::subtyping;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::pending::pending;
use crate::typing::stmt::binding_stmt::type_binding_stmt;
use crate::typing::stmt::return_stmt::type_return_stmt;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, push_scope, TypeEnv};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, emit_stale_binding_reference_warning, type_expr, type_identifier_expr, type_place};
use crate::typing::type_predicates::bitcopy_type;
use crate::typing::types::{make_type_prim, type_to_string, TypeRef};

/// What leaves a statement other than by falling through: the types that reach the
/// block's result, and the values of `break`s.
#[derive(Debug, Clone, Default)]
pub struct FlowInfo {
    pub results: Vec<TypeRef>,
    pub breaks: Vec<TypeRef>,
    pub break_void: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StmtTypeResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub env: TypeEnv,
    pub flow: FlowInfo,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
    pub diagnostic_obligation_ids: Vec<&'static str>,
}

impl StmtTypeResult {
    pub fn typed(env: TypeEnv) -> Self {
        StmtTypeResult {
            ok: true,
            env,
            ..Default::default()
        }
    }

    pub fn failed(diag_id: Option<&'static str>, env: &TypeEnv) -> Self {
        StmtTypeResult {
            diag_id,
            env: env.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StmtSeqResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub env: TypeEnv,
    pub flow: FlowInfo,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
    pub diagnostic_obligation_ids: Vec<&'static str>,
    /// The facts known after the last statement.
    pub proof_ctx: super::proof_facts::ProofCtx,
}

#[derive(Debug, Clone, Default)]
pub struct BlockInfoResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub r#type: TypeRef,
    pub breaks: Vec<TypeRef>,
    pub break_void: bool,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
    pub diagnostic_obligation_ids: Vec<&'static str>,
}

type EnvRef<'r> = Option<&'r Rc<RefCell<TypeEnv>>>;

/// Publishes the environment to whoever asked to follow it.
fn publish_env(type_ctx: &StmtTypeContext<'_>, env_ref: EnvRef<'_>, env: &TypeEnv) {
    if let Some(cell) = &type_ctx.env_ref {
        *cell.borrow_mut() = env.clone();
    }
    if let Some(cell) = env_ref {
        *cell.borrow_mut() = env.clone();
    }
}

pub fn stmt_kind(stmt: &Stmt) -> &'static str {
    match stmt {
        Stmt::LetStmt(_) => "LetStmt",
        Stmt::VarStmt(_) => "VarStmt",
        Stmt::UsingLocalStmt(_) => "UsingLocalStmt",
        Stmt::AssignStmt(_) => "AssignStmt",
        Stmt::CompoundAssignStmt(_) => "CompoundAssignStmt",
        Stmt::ExprStmt(_) => "ExprStmt",
        Stmt::DeferStmt(_) => "DeferStmt",
        Stmt::RegionStmt(_) => "RegionStmt",
        Stmt::FrameStmt(_) => "FrameStmt",
        Stmt::ReturnStmt(_) => "ReturnStmt",
        Stmt::BreakStmt(_) => "BreakStmt",
        Stmt::ContinueStmt(_) => "ContinueStmt",
        Stmt::UnsafeBlockStmt(_) => "UnsafeBlockStmt",
        Stmt::CtStmt(_) => "CtStmt",
        Stmt::KeyBlockStmt(_) => "KeyBlockStmt",
        Stmt::ErrorStmt(_) => "ErrorStmt",
    }
}

pub fn span_of_stmt(stmt: &Stmt) -> &Span {
    match stmt {
        Stmt::LetStmt(node) => &node.span,
        Stmt::VarStmt(node) => &node.span,
        Stmt::UsingLocalStmt(node) => &node.span,
        Stmt::AssignStmt(node) => &node.span,
        Stmt::CompoundAssignStmt(node) => &node.span,
        Stmt::ExprStmt(node) => &node.span,
        Stmt::DeferStmt(node) => &node.span,
        Stmt::RegionStmt(node) => &node.span,
        Stmt::FrameStmt(node) => &node.span,
        Stmt::ReturnStmt(node) => &node.span,
        Stmt::BreakStmt(node) => &node.span,
        Stmt::ContinueStmt(node) => &node.span,
        Stmt::UnsafeBlockStmt(node) => &node.span,
        Stmt::CtStmt(node) => &node.span,
        Stmt::KeyBlockStmt(node) => &node.span,
        Stmt::ErrorStmt(node) => &node.span,
    }
}

/// The one type all the given types are equivalent to: the first of them.
pub fn res_type(types: &[TypeRef]) -> Option<TypeRef> {
    let (base, rest) = types.split_first()?;
    rest.iter()
        .all(|ty| type_equiv(base, ty))
        .then(|| base.clone())
}

/// The type of a loop that only ends by `break`: `!` when nothing breaks.
pub fn loop_type_inf(breaks: &[TypeRef], break_void: bool) -> Option<TypeRef> {
    match (breaks.is_empty(), break_void) {
        (true, false) => Some(make_type_prim("!")),
        (true, true) => Some(make_type_prim("()")),
        (false, false) => res_type(breaks),
        (false, true) => None,
    }
}

/// The type of a loop that may also run out: `()` when nothing breaks with a value.
pub fn loop_type_fin(breaks: &[TypeRef], break_void: bool) -> Option<TypeRef> {
    match (breaks.is_empty(), break_void) {
        (true, _) => Some(make_type_prim("()")),
        (false, false) => res_type(breaks),
        (false, true) => None,
    }
}

/// A bound name used as a value must be of a type that is copied.
fn type_ident_expr(ctx: &ScopeContext<'_>, env: &TypeEnv, name: &str) -> ExprTypeResult {
    match bind_of(env, name) {
        None => ExprTypeResult::failed(Some("ResolveExpr-Ident-Err")),
        Some(binding) if !bitcopy_type(ctx, &binding.r#type) => {
            ExprTypeResult::failed(Some("ValueUse-NonBitcopyPlace"))
        }
        Some(binding) => ExprTypeResult::typed(binding.r#type.clone()),
    }
}

/// Types an expression in a given environment. The callback, bound to whatever
/// environment its maker had, is only asked when that says nothing at all.
pub fn type_expr_with_env(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    expr: &ExprPtr,
) -> ExprTypeResult {
    let Some(e) = expr.as_deref() else {
        return ExprTypeResult::default();
    };
    if let ExprNode::IdentifierExpr(ident) = &e.node {
        if bind_of(env, &ident.name).is_some() {
            return type_ident_expr(ctx, env, &ident.name);
        }
    }
    let via_env = type_expr(ctx, type_ctx, expr, env);
    if via_env.ok || via_env.diag_id.is_some() {
        return via_env;
    }
    type_expr_fn(expr)
}

/// A bound name is a place of its type; anything else is asked of the callback.
pub fn type_place_with_env(
    env: &TypeEnv,
    type_place_fn: PlaceTypeFn<'_>,
    expr: &ExprPtr,
) -> PlaceTypeResult {
    let Some(e) = expr.as_deref() else {
        return PlaceTypeResult::default();
    };
    if let ExprNode::IdentifierExpr(ident) = &e.node {
        if let Some(binding) = bind_of(env, &ident.name) {
            return PlaceTypeResult {
                ok: true,
                r#type: binding.r#type.clone(),
                ..Default::default()
            };
        }
    }
    type_place_fn(expr)
}

fn merge_break_flow(dst: &mut FlowInfo, src: FlowInfo) {
    dst.breaks.extend(src.breaks);
    dst.break_void |= src.break_void;
}

/// The `break`s of a block that are not inside a loop of their own: those in nested
/// blocks and in the branches of conditionals.
struct BreakCollector<'c, 'a, 't, 'f> {
    ctx: &'c ScopeContext<'a>,
    type_ctx: &'c StmtTypeContext<'t>,
    env: &'c TypeEnv,
    type_expr_fn: ExprTypeFn<'f>,
    /// Whether a bound name broken with is typed as a value use, which requires a type
    /// that is copied. Blocks do; expression statements do not.
    ident_as_value: bool,
}

/// The `break`s reachable from an expression without entering a loop.
pub fn collect_break_flow(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    expr: &ExprPtr,
    ident_as_value: bool,
) -> FlowInfo {
    BreakCollector { ctx, type_ctx, env, type_expr_fn, ident_as_value }.expr(expr)
}

impl BreakCollector<'_, '_, '_, '_> {
    fn block(&self, block: &ast::Block) -> FlowInfo {
        let mut flow = FlowInfo::default();
        for stmt in &block.stmts {
            merge_break_flow(&mut flow, self.stmt(stmt));
        }
        merge_break_flow(&mut flow, self.expr(&block.tail_opt));
        flow
    }

    fn block_ptr(&self, block: &ast::BlockPtr) -> FlowInfo {
        block
            .as_deref()
            .map(|block| self.block(block))
            .unwrap_or_default()
    }

    fn stmt(&self, stmt: &Stmt) -> FlowInfo {
        match stmt {
            Stmt::BreakStmt(node) => {
                let mut flow = FlowInfo::default();
                if node.value_opt.is_none() {
                    flow.break_void = true;
                } else {
                    let typed = if self.ident_as_value {
                        type_expr_with_env(self.ctx, self.type_ctx, self.env, self.type_expr_fn, &node.value_opt)
                    } else {
                        let via_env = type_expr(self.ctx, self.type_ctx, &node.value_opt, self.env);
                        if via_env.ok || via_env.diag_id.is_some() {
                            via_env
                        } else {
                            (self.type_expr_fn)(&node.value_opt)
                        }
                    };
                    if typed.ok {
                        flow.breaks.push(typed.r#type);
                    }
                }
                flow
            }
            Stmt::ExprStmt(node) => self.expr(&node.value),
            Stmt::UnsafeBlockStmt(node) => self.block_ptr(&node.body),
            Stmt::RegionStmt(node) => self.block_ptr(&node.body),
            Stmt::FrameStmt(node) => self.block_ptr(&node.body),
            Stmt::KeyBlockStmt(node) => self.block_ptr(&node.body),
            _ => FlowInfo::default(),
        }
    }

    fn expr(&self, expr: &ExprPtr) -> FlowInfo {
        let mut out = FlowInfo::default();
        match expr.as_deref().map(|expr| &expr.node) {
            Some(ExprNode::AttributedExpr(node)) => return self.expr(&node.expr),
            Some(ExprNode::BlockExpr(node)) => return self.block_ptr(&node.block),
            Some(ExprNode::UnsafeBlockExpr(node)) => return self.block_ptr(&node.block),
            Some(ExprNode::IfExpr(node)) => {
                merge_break_flow(&mut out, self.expr(&node.then_expr));
                merge_break_flow(&mut out, self.expr(&node.else_expr));
            }
            Some(ExprNode::IfIsExpr(node)) => {
                merge_break_flow(&mut out, self.expr(&node.then_expr));
                merge_break_flow(&mut out, self.expr(&node.else_expr));
            }
            Some(ExprNode::IfCaseExpr(node)) => {
                for clause in &node.cases {
                    merge_break_flow(&mut out, self.expr(&clause.body));
                }
                merge_break_flow(&mut out, self.expr(&node.else_expr));
            }
            _ => {}
        }
        out
    }
}

/// Types one statement. The statement kinds are ported one at a time.
#[allow(clippy::too_many_arguments)]
pub fn type_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    stmt: &Stmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
    env_ref: EnvRef<'_>,
) -> StmtTypeResult {
    let _ = env_ref;
    match stmt {
        Stmt::LetStmt(node) => type_binding_stmt(
            ctx,
            type_ctx,
            &node.binding,
            ast::Mutability::Let,
            env,
            type_ident_fn,
            type_place_fn,
        ),
        Stmt::VarStmt(node) => type_binding_stmt(
            ctx,
            type_ctx,
            &node.binding,
            ast::Mutability::Var,
            env,
            type_ident_fn,
            type_place_fn,
        ),
        Stmt::ReturnStmt(node) => type_return_stmt(ctx, type_ctx, node, env, type_expr_fn),
        Stmt::AssignStmt(node) => {
            super::assign_stmt::type_assign_stmt(ctx, type_ctx, node, env, type_expr_fn, type_ident_fn, type_place_fn)
        }
        Stmt::UnsafeBlockStmt(node) => crate::typing::expr::access::type_unsafe_block_stmt(
            ctx,
            type_ctx,
            node,
            env,
            type_expr_fn,
            type_ident_fn,
            type_place_fn,
        ),
        Stmt::RegionStmt(node) => super::scoped::type_region_stmt(ctx, type_ctx, node, env, type_expr_fn, type_ident_fn, type_place_fn),
        Stmt::FrameStmt(node) => super::scoped::type_frame_stmt(ctx, type_ctx, node, env, type_expr_fn, type_ident_fn, type_place_fn),
        Stmt::DeferStmt(node) => super::scoped::type_defer_stmt(ctx, type_ctx, node, env, type_expr_fn, type_ident_fn, type_place_fn),
        Stmt::UsingLocalStmt(node) => super::scoped::type_using_local_stmt(node, env),
        Stmt::BreakStmt(node) => crate::typing::expr::loops::type_break_stmt(ctx, type_ctx, node, env, type_expr_fn),
        Stmt::ContinueStmt(_) => crate::typing::expr::loops::type_continue_stmt(type_ctx, env),
        Stmt::ExprStmt(node) => super::expr_stmt::type_expr_stmt(ctx, type_ctx, node, env, type_expr_fn),
        _ => {
            pending(stmt_kind(stmt));
            StmtTypeResult::failed(None, env)
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn type_stmt_seq(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    stmts: &[Stmt],
    env: &TypeEnv,
    _type_expr_fn: ExprTypeFn<'_>,
    _type_ident_fn: IdentTypeFn<'_>,
    _type_place_fn: PlaceTypeFn<'_>,
    env_ref: EnvRef<'_>,
) -> StmtSeqResult {
    let mut current = env.clone();
    let mut current_proof_ctx = type_ctx.proof_ctx.clone();
    publish_env(type_ctx, env_ref, &current);
    let mut flow = FlowInfo::default();
    for stmt in stmts {
        publish_env(type_ctx, env_ref, &current);
        // Each statement is typed by functions bound to the environment before it,
        // with the facts the statements before it left.
        let stmt_ctx = StmtTypeContext { proof_ctx: current_proof_ctx.clone(), ..type_ctx.clone() };
        let type_ctx = &stmt_ctx;
        let typed = {
            let expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, &current);
            let ident_fn = |name: &str| {
                let typed = type_identifier_expr(ctx, &current, name);
                if typed.ok {
                    if let Some(binding) = bind_of(&current, name) {
                        emit_stale_binding_reference_warning(binding, type_ctx, None);
                    }
                }
                typed
            };
            let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, &current);
            type_stmt(
                ctx, type_ctx, stmt, &current, &expr_fn, &ident_fn, &place_fn, env_ref,
            )
        };
        if !typed.ok {
            let diag_detail = if typed.diag_detail.is_empty() {
                format!("statement failed typing: {}", stmt_kind(stmt))
            } else {
                typed.diag_detail
            };
            if let Some(cell) = env_ref {
                *cell.borrow_mut() = current.clone();
            }
            return StmtSeqResult {
                ok: false,
                diag_id: typed.diag_id,
                env: TypeEnv::default(),
                flow: FlowInfo::default(),
                diag_detail,
                diag_span: typed.diag_span.or_else(|| Some(span_of_stmt(stmt).clone())),
                diagnostic_obligation_ids: typed.diagnostic_obligation_ids,
                proof_ctx: None,
            };
        }
        current = typed.env;
        current_proof_ctx = super::proof_facts::fallthrough_proof_context_for_stmt(ctx, &current, &current_proof_ctx, stmt);
        publish_env(type_ctx, env_ref, &current);
        flow.results.extend(typed.flow.results);
        merge_break_flow(
            &mut flow,
            FlowInfo {
                results: Vec::new(),
                ..typed.flow
            },
        );
    }
    publish_env(type_ctx, env_ref, &current);
    StmtSeqResult { ok: true, env: current, flow, proof_ctx: current_proof_ctx, ..Default::default() }
}

#[allow(clippy::too_many_arguments)]
pub fn type_block_info(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    block: &ast::Block,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
    env_ref: EnvRef<'_>,
) -> BlockInfoResult {
    let pushed = push_scope(env);
    let stmts_typed = type_stmt_seq(
        ctx,
        type_ctx,
        &block.stmts,
        &pushed,
        type_expr_fn,
        type_ident_fn,
        type_place_fn,
        env_ref,
    );
    if !stmts_typed.ok {
        return BlockInfoResult {
            diag_id: stmts_typed.diag_id,
            diag_detail: stmts_typed.diag_detail,
            diag_span: stmts_typed.diag_span,
            diagnostic_obligation_ids: stmts_typed.diagnostic_obligation_ids,
            ..Default::default()
        };
    }
    let mut break_flow = FlowInfo {
        results: Vec::new(),
        breaks: stmts_typed.flow.breaks.clone(),
        break_void: stmts_typed.flow.break_void,
    };
    let collector = BreakCollector {
        ctx,
        type_ctx,
        env: &stmts_typed.env,
        type_expr_fn, ident_as_value: true };
    merge_break_flow(&mut break_flow, collector.block(block));
    let done = |ty: TypeRef| BlockInfoResult {
        ok: true,
        r#type: ty,
        breaks: break_flow.breaks.clone(),
        break_void: break_flow.break_void,
        ..Default::default()
    };
    // The tail is typed even when the statements already decide the block's type.
    let mut tail_type = None;
    if let Some(tail) = block.tail_opt.as_deref() {
        publish_env(type_ctx, env_ref, &stmts_typed.env);
        let tail_ctx = StmtTypeContext { proof_ctx: stmts_typed.proof_ctx.clone(), ..type_ctx.clone() };
        let tail_fn = |inner: &ExprPtr| type_expr(ctx, &tail_ctx, inner, &stmts_typed.env);
        let typed = type_expr_with_env(ctx, &tail_ctx, &stmts_typed.env, &tail_fn, &block.tail_opt);
        if !typed.ok {
            return BlockInfoResult {
                diag_id: typed.diag_id,
                diag_detail: typed.diag_detail,
                diag_span: typed.diag_span.or_else(|| Some(tail.span.clone())),
                ..Default::default()
            };
        }
        tail_type = Some(typed.r#type);
    }
    if let Some(ty) = res_type(&stmts_typed.flow.results) {
        return done(ty);
    }
    if !stmts_typed.flow.results.is_empty() {
        return BlockInfoResult {
            diag_id: Some("BlockInfo-Res-Err"),
            ..Default::default()
        };
    }
    if let Some(ty) = tail_type {
        return done(ty);
    }
    if matches!(block.stmts.last(), Some(Stmt::ReturnStmt(_))) {
        return done(make_type_prim("!"));
    }
    done(make_type_prim("()"))
}

/// The type of a block.
#[allow(clippy::too_many_arguments)]
pub fn type_block(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    block: &ast::Block,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
    env_ref: EnvRef<'_>,
) -> ExprTypeResult {
    let info = type_block_info(
        ctx,
        type_ctx,
        block,
        env,
        type_expr_fn,
        type_ident_fn,
        type_place_fn,
        env_ref,
    );
    ExprTypeResult {
        ok: info.ok,
        diag_id: info.diag_id,
        r#type: if info.ok { info.r#type } else { None },
        diag_detail: info.diag_detail,
        diag_span: info.diag_span,
        diagnostic_obligation_ids: info.diagnostic_obligation_ids,
    }
}

/// Whether a block has the expected type: its result paths, else its tail, else a
/// final `return`, else nothing where `()` is expected.
#[allow(clippy::too_many_arguments)]
pub fn check_block(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    block: &ast::Block,
    env: &TypeEnv,
    expected: &TypeRef,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
    env_ref: EnvRef<'_>,
) -> CheckResult {
    if expected.is_none() {
        return CheckResult::default();
    }
    let fail = |diag_id: Option<&'static str>, diag_detail: String| CheckResult { diag_id, diag_detail, ..Default::default() };
    let pushed = push_scope(env);
    let stmts_typed =
        type_stmt_seq(ctx, type_ctx, &block.stmts, &pushed, type_expr_fn, type_ident_fn, type_place_fn, env_ref);
    if !stmts_typed.ok {
        return CheckResult {
            diag_id: stmts_typed.diag_id,
            diag_detail: stmts_typed.diag_detail,
            diag_span: stmts_typed.diag_span,
            ..Default::default()
        };
    }
    if let Some(result_type) = res_type(&stmts_typed.flow.results) {
        let sub = subtyping(ctx, &result_type, expected);
        if !sub.ok {
            return fail(sub.diag_id, String::new());
        }
        if !sub.subtype {
            let detail = format!(
                "block result type {} is not compatible with expected {}",
                type_to_string(&result_type),
                type_to_string(expected)
            );
            return fail(sub.diag_id, detail);
        }
        return CheckResult { ok: true, ..Default::default() };
    }
    if !stmts_typed.flow.results.is_empty() {
        return fail(Some("BlockInfo-Res-Err"), "block result paths do not have one equivalent type".to_string());
    }
    if let Some(tail) = block.tail_opt.as_deref() {
        publish_env(type_ctx, env_ref, &stmts_typed.env);
        let tail_ctx = StmtTypeContext { proof_ctx: stmts_typed.proof_ctx.clone(), ..type_ctx.clone() };
        let check = check_expr_against(ctx, &tail_ctx, &block.tail_opt, expected, &stmts_typed.env);
        if check.ok {
            return CheckResult { ok: true, ..Default::default() };
        }
        let diag_detail = if check.diag_detail.is_empty() {
            format!("block tail expression failed checking against expected {}", type_to_string(expected))
        } else {
            check.diag_detail
        };
        return CheckResult {
            diag_id: check.diag_id,
            diag_detail,
            diag_span: check.diag_span.or_else(|| Some(tail.span.clone())),
            ..Default::default()
        };
    }
    let unit = matches!(expected.as_deref().map(|ty| &ty.node), Some(crate::typing::types::TypeNode::Prim(name)) if name == "()");
    if matches!(block.stmts.last(), Some(Stmt::ReturnStmt(_))) || unit {
        return CheckResult { ok: true, ..Default::default() };
    }
    fail(None, format!("block has no tail expression or explicit return for expected {}", type_to_string(expected)))
}

/// A block as an expression: typed without publishing its environment.
pub fn type_block_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::BlockExpr, env: &TypeEnv) -> ExprTypeResult {
    let Some(block) = expr.block.as_deref() else {
        return ExprTypeResult::default();
    };
    let mut body_ctx = type_ctx.clone();
    body_ctx.env_ref = None;
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env);
    let ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    type_block(ctx, &body_ctx, block, env, &expr_fn, &ident_fn, &place_fn, None)
}

pub fn check_block_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::BlockExpr,
    env: &TypeEnv,
    expected: &TypeRef,
) -> CheckResult {
    let Some(block) = expr.block.as_deref() else {
        return CheckResult::default();
    };
    let mut body_ctx = type_ctx.clone();
    body_ctx.env_ref = None;
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env);
    let ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    check_block(ctx, &body_ctx, block, env, expected, &expr_fn, &ident_fn, &place_fn, None)
}
