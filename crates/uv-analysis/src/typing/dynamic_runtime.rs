//! Whether a `#dynamic` procedure has anything that may need a run-time check, and the
//! warning when it has not. See `EmitDynamicNoRuntimeWarningIfNeeded`.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_source::ast::{self, ExprNode as E, ExprPtr, Stmt};
use uv_source::attributes::{attrs, has_attribute};

use crate::contracts::verification::{static_proof, StaticProofContext};

fn needs_runtime_check(predicate: &ExprPtr) -> bool {
    predicate.is_some() && !static_proof(&StaticProofContext::default(), predicate).provable
}

fn invariant_needs_runtime_check(invariant: &Option<ast::LoopInvariant>) -> bool {
    invariant.as_ref().is_some_and(|invariant| needs_runtime_check(&invariant.predicate))
}

fn any_expr(exprs: &[ExprPtr]) -> bool {
    exprs.iter().any(expr_may_need_dynamic_runtime)
}

fn args_may_need_runtime(args: &[ast::Arg]) -> bool {
    args.iter().any(|arg| expr_may_need_dynamic_runtime(&arg.value))
}

fn field_inits_may_need_runtime(fields: &[ast::FieldInit]) -> bool {
    fields.iter().any(|field| expr_may_need_dynamic_runtime(&field.value))
}

fn block_opt(block: &Option<std::sync::Arc<ast::Block>>) -> bool {
    block.as_deref().is_some_and(block_may_need_dynamic_runtime)
}

/// Calls, indexing and key blocks always may; the rest depend on what they contain.
fn expr_may_need_dynamic_runtime(expr: &ExprPtr) -> bool {
    let Some(expr) = expr.as_deref() else {
        return false;
    };
    let one = expr_may_need_dynamic_runtime;
    match &expr.node {
        E::QualifiedApplyExpr(node) => match &node.args {
            ast::ApplyArgs::ParenArgs(args) => args_may_need_runtime(&args.args),
            ast::ApplyArgs::BraceArgs(args) => field_inits_may_need_runtime(&args.fields),
        },
        E::EnumLiteralExpr(node) => match &node.payload_opt {
            None => false,
            Some(ast::EnumPayload::EnumPayloadParen(payload)) => any_expr(&payload.elements),
            Some(ast::EnumPayload::EnumPayloadBrace(payload)) => field_inits_may_need_runtime(&payload.fields),
        },
        E::RangeExpr(node) => one(&node.lhs) || one(&node.rhs),
        E::BinaryExpr(node) => one(&node.lhs) || one(&node.rhs),
        E::CastExpr(node) => one(&node.value),
        E::UnaryExpr(node) => one(&node.value),
        E::DerefExpr(node) => one(&node.value),
        E::AddressOfExpr(node) => one(&node.place),
        E::MoveExpr(node) => one(&node.place),
        E::AllocExpr(node) => one(&node.value),
        E::EntryExpr(node) => one(&node.expr),
        E::YieldExpr(node) => one(&node.value),
        E::YieldFromExpr(node) => one(&node.value),
        E::SyncExpr(node) => one(&node.value),
        E::WaitExpr(node) => one(&node.handle),
        E::AttributedExpr(node) => one(&node.expr),
        E::TransmuteExpr(node) => one(&node.value),
        E::TupleExpr(node) => any_expr(&node.elements),
        E::ArrayExpr(node) => node.elements.iter().any(|segment| match segment {
            ast::ArraySegment::ArrayElemSegment(segment) => one(&segment.value),
            ast::ArraySegment::ArrayRepeatSegment(segment) => one(&segment.value) || one(&segment.count),
        }),
        E::ArrayRepeatExpr(node) => one(&node.value) || one(&node.count),
        E::RecordExpr(node) => field_inits_may_need_runtime(&node.fields),
        E::IfExpr(node) => one(&node.cond) || one(&node.then_expr) || one(&node.else_expr),
        E::IfCaseExpr(node) => one(&node.scrutinee) || node.cases.iter().any(|arm| one(&arm.body)) || one(&node.else_expr),
        E::IfIsExpr(node) => one(&node.scrutinee) || one(&node.then_expr) || one(&node.else_expr),
        E::LoopInfiniteExpr(node) => invariant_needs_runtime_check(&node.invariant_opt) || block_opt(&node.body),
        E::LoopConditionalExpr(node) => invariant_needs_runtime_check(&node.invariant_opt) || one(&node.cond) || block_opt(&node.body),
        E::LoopIterExpr(node) => invariant_needs_runtime_check(&node.invariant_opt) || one(&node.iter) || block_opt(&node.body),
        E::BlockExpr(node) => block_opt(&node.block),
        E::UnsafeBlockExpr(node) => block_opt(&node.block),
        E::ClosureExpr(node) => one(&node.body),
        E::PipelineExpr(node) => one(&node.lhs) || one(&node.rhs),
        E::FieldAccessExpr(node) => one(&node.base),
        E::TupleAccessExpr(node) => one(&node.base),
        E::PropagateExpr(node) => one(&node.value),
        E::IndexAccessExpr(_) | E::CallExpr(_) | E::MethodCallExpr(_) => true,
        E::RaceExpr(node) => node.arms.iter().any(|arm| one(&arm.expr) || one(&arm.handler.value)),
        E::AllExpr(node) => any_expr(&node.exprs),
        E::ParallelExpr(node) => one(&node.domain) || node.opts.iter().any(|opt| one(&opt.value)) || block_opt(&node.body),
        E::SpawnExpr(node) => node.opts.iter().any(|opt| one(&opt.value)) || block_opt(&node.body),
        E::DispatchExpr(node) => {
            one(&node.range)
                || node.key_clause.iter().flat_map(|clause| &clause.key_path.segs).any(|seg| matches!(seg, ast::KeySeg::KeySegIndex(index) if one(&index.expr)))
                || node.opts.iter().any(|opt| one(&opt.chunk_expr) || one(&opt.workgroup_expr))
                || block_opt(&node.body)
        }
        _ => false,
    }
}

fn block_may_need_dynamic_runtime(block: &ast::Block) -> bool {
    let one = expr_may_need_dynamic_runtime;
    let stmt_needs_runtime = |stmt: &Stmt| match stmt {
        Stmt::LetStmt(node) => one(&node.binding.init),
        Stmt::VarStmt(node) => one(&node.binding.init),
        Stmt::AssignStmt(node) => one(&node.place) || one(&node.value),
        Stmt::CompoundAssignStmt(node) => one(&node.place) || one(&node.value),
        Stmt::ExprStmt(node) => one(&node.value),
        Stmt::DeferStmt(node) => block_opt(&node.body),
        Stmt::RegionStmt(node) => one(&node.opts_opt) || block_opt(&node.body),
        Stmt::FrameStmt(node) => block_opt(&node.body),
        Stmt::ReturnStmt(node) => one(&node.value_opt),
        Stmt::BreakStmt(node) => one(&node.value_opt),
        Stmt::UnsafeBlockStmt(node) => block_opt(&node.body),
        Stmt::KeyBlockStmt(_) => true,
        _ => false,
    };
    block.stmts.iter().any(stmt_needs_runtime) || one(&block.tail_opt)
}

/// The warning for a `#dynamic` procedure that has nothing which may need a check at run time.
pub fn emit_dynamic_no_runtime_warning_if_needed(decl: &ast::ProcedureDecl, diags: &mut DiagnosticStream) {
    if !has_attribute(&decl.attrs, attrs::DYNAMIC) {
        return;
    }
    let contract_needs = decl.contract.as_ref().is_some_and(|contract| needs_runtime_check(&contract.precondition) || needs_runtime_check(&contract.postcondition));
    if contract_needs || decl.body.as_deref().is_some_and(block_may_need_dynamic_runtime) {
        return;
    }
    if let Some(diag) = make_diagnostic_by_id("W-CON-0401", Some(decl.span.clone())) {
        emit(diags, diag);
    }
}
