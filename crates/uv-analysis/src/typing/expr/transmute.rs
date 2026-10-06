//! `transmute` and region allocation.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::ScopeContext;
use crate::layout::layout_of;
use crate::memory::calls::is_in_unsafe_span;
use crate::memory::regions::{innermost_active_region, region_active_type};
use crate::resolve::scopes::id_eq;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, TypeEnv};
use crate::typing::type_expr::{check_expr_against, type_expr};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm;
use crate::typing::type_wf::type_wf;
use crate::typing::types::*;

fn is_numeric_prim(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f16" | "f32" | "f64")
}

/// A type some of whose bit patterns are not values: anything but numbers, raw
/// pointers and arrays of them.
fn known_invalid_transmute_target(ty: &TypeRef) -> bool {
    match strip_perm(ty).as_deref().map(|ty| &ty.node) {
        None => false,
        Some(TypeNode::Prim(name)) => !is_numeric_prim(name) && (id_eq(name, "bool") || id_eq(name, "char")),
        Some(TypeNode::RawPtr { .. } | TypeNode::Var(_)) => false,
        Some(TypeNode::Array { element, .. }) => known_invalid_transmute_target(element),
        Some(TypeNode::Perm { base, .. }) => known_invalid_transmute_target(base),
        Some(_) => true,
    }
}

/// Warns once per transmute about a target with invalid values.
fn emit_invalid_target_warning(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, span: &Span) {
    let Some(diags) = type_ctx.diags.as_ref().or(ctx.diagnostics.as_ref()) else {
        return;
    };
    if diags.borrow().iter().any(|diag| diag.code == "W-SAF-0100" && diag.span.as_ref() == Some(span)) {
        return;
    }
    if let Some(diag) = make_diagnostic_by_id("W-SAF-0100", Some(span.clone())) {
        emit(&mut diags.borrow_mut(), diag);
    }
}

fn warn_in_stmt(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, stmt: &Stmt) {
    let expr = |expr: &ExprPtr| warn_in_expr(ctx, type_ctx, expr);
    let block = |block: &ast::BlockPtr| {
        if let Some(block) = block.as_deref() {
            emit_invalid_transmute_target_warnings_in_block(ctx, type_ctx, block);
        }
    };
    match stmt {
        Stmt::LetStmt(node) => expr(&node.binding.init),
        Stmt::VarStmt(node) => expr(&node.binding.init),
        Stmt::AssignStmt(node) => {
            expr(&node.place);
            expr(&node.value);
        }
        Stmt::CompoundAssignStmt(node) => {
            expr(&node.place);
            expr(&node.value);
        }
        Stmt::ExprStmt(node) => expr(&node.value),
        Stmt::DeferStmt(node) => block(&node.body),
        Stmt::FrameStmt(node) => block(&node.body),
        Stmt::UnsafeBlockStmt(node) => block(&node.body),
        Stmt::RegionStmt(node) => {
            block(&node.body);
            expr(&node.opts_opt);
        }
        Stmt::ReturnStmt(node) => expr(&node.value_opt),
        Stmt::BreakStmt(node) => expr(&node.value_opt),
        _ => {}
    }
}

fn warn_in_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ExprPtr) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    let visit = |inner: &ExprPtr| warn_in_expr(ctx, type_ctx, inner);
    let block = |block: &ast::BlockPtr| {
        if let Some(block) = block.as_deref() {
            emit_invalid_transmute_target_warnings_in_block(ctx, type_ctx, block);
        }
    };
    match &e.node {
        ExprNode::TransmuteExpr(node) => {
            if lower_type(ctx, &node.to).is_ok_and(|lowered| known_invalid_transmute_target(&lowered)) {
                emit_invalid_target_warning(ctx, type_ctx, &e.span);
            }
            visit(&node.value);
        }
        ExprNode::AttributedExpr(node) => visit(&node.expr),
        ExprNode::UnsafeBlockExpr(node) => block(&node.block),
        ExprNode::BlockExpr(node) => block(&node.block),
        ExprNode::BinaryExpr(node) => {
            visit(&node.lhs);
            visit(&node.rhs);
        }
        ExprNode::PipelineExpr(node) => {
            visit(&node.lhs);
            visit(&node.rhs);
        }
        ExprNode::UnaryExpr(node) => visit(&node.value),
        ExprNode::PropagateExpr(node) => visit(&node.value),
        ExprNode::YieldExpr(node) => visit(&node.value),
        ExprNode::YieldFromExpr(node) => visit(&node.value),
        ExprNode::CastExpr(node) => visit(&node.value),
        ExprNode::MoveExpr(node) => visit(&node.place),
        ExprNode::TupleExpr(node) => node.elements.iter().for_each(visit),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node).into_iter().for_each(visit),
        ExprNode::ArrayRepeatExpr(node) => {
            visit(&node.value);
            visit(&node.count);
        }
        ExprNode::RecordExpr(node) => node.fields.iter().for_each(|field| visit(&field.value)),
        ExprNode::FieldAccessExpr(node) => visit(&node.base),
        ExprNode::TupleAccessExpr(node) => visit(&node.base),
        ExprNode::IndexAccessExpr(node) => {
            visit(&node.base);
            visit(&node.index);
        }
        ExprNode::CallExpr(node) => {
            visit(&node.callee);
            node.args.iter().for_each(|arg| visit(&arg.value));
        }
        ExprNode::MethodCallExpr(node) => {
            visit(&node.receiver);
            node.args.iter().for_each(|arg| visit(&arg.value));
        }
        ExprNode::IfExpr(node) => {
            visit(&node.cond);
            visit(&node.then_expr);
            visit(&node.else_expr);
        }
        ExprNode::IfCaseExpr(node) => {
            visit(&node.scrutinee);
            node.cases.iter().for_each(|case_clause| visit(&case_clause.body));
            visit(&node.else_expr);
        }
        _ => {}
    }
}

/// After an `unsafe` block is typed, warns about every transmute in it whose target
/// has invalid values, including those typing did not reach.
pub fn emit_invalid_transmute_target_warnings_in_block(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, block: &ast::Block) {
    for stmt in &block.stmts {
        warn_in_stmt(ctx, type_ctx, stmt);
    }
    warn_in_expr(ctx, type_ctx, &block.tail_opt);
}

/// `transmute::<From, To>(value)`: allowed in `unsafe` between types of one size and
/// alignment.
pub fn type_transmute_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::TransmuteExpr,
    env: &TypeEnv,
    span: &Span,
) -> ExprTypeResult {
    let failed = |diag_id: Option<&'static str>| ExprTypeResult::failed(diag_id);
    if type_ctx.require_pure {
        return failed(Some("E-SEM-2802"));
    }
    if !is_in_unsafe_span(ctx, span) {
        return failed(Some("Transmute-Unsafe-Err"));
    }
    if expr.from.is_none() || expr.to.is_none() || expr.value.is_none() {
        return failed(None);
    }
    let lowered = |ty: &ast::TypePtr| -> Result<TypeRef, Option<&'static str>> {
        let lowered = lower_type(ctx, ty)?;
        type_wf(ctx, &lowered)?;
        Ok(lowered)
    };
    let from = match lowered(&expr.from) {
        Ok(from) => from,
        Err(diag_id) => return failed(diag_id),
    };
    let to = match lowered(&expr.to) {
        Ok(to) => to,
        Err(diag_id) => return failed(diag_id),
    };
    let (Some(from_layout), Some(to_layout)) = (layout_of(ctx, &from), layout_of(ctx, &to)) else {
        return failed(None);
    };
    if from_layout.size != to_layout.size {
        return failed(Some("T-Transmute-SizeEq"));
    }
    if from_layout.align != to_layout.align {
        return failed(Some("T-Transmute-AlignEq"));
    }
    let value_check = check_expr_against(ctx, type_ctx, &expr.value, &from, env);
    if !value_check.ok {
        return ExprTypeResult { diag_id: value_check.diag_id, diag_detail: value_check.diag_detail, diag_span: value_check.diag_span, ..Default::default() };
    }
    if known_invalid_transmute_target(&to) {
        emit_invalid_target_warning(ctx, type_ctx, span);
    }
    ExprTypeResult::typed(to)
}

/// `^value` or `region^value`: the value allocated in the named region, or in the
/// innermost active one.
pub fn type_alloc_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::AllocExpr, env: &TypeEnv) -> ExprTypeResult {
    let failed = |diag_id: &'static str| ExprTypeResult::failed(Some(diag_id));
    if type_ctx.require_pure {
        return failed("E-SEM-2802");
    }
    if expr.value.is_none() {
        return ExprTypeResult::default();
    }
    match &expr.region_opt {
        Some(region) => match bind_of(env, region) {
            None => return failed("ResolveExpr-Ident-Err"),
            Some(binding) if !region_active_type(&binding.r#type) => return failed("E-MEM-1206"),
            Some(_) => {}
        },
        None => {
            if innermost_active_region(env).is_none() {
                return failed("E-MEM-3021");
            }
        }
    }
    let inner = type_expr(ctx, type_ctx, &expr.value, env);
    if !inner.ok {
        return ExprTypeResult::failed(inner.diag_id);
    }
    ExprTypeResult::typed(inner.r#type)
}
