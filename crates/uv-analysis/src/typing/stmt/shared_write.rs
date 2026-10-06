//! Writes to shared data: which key covers the place written, and whether the value
//! written reads that same place (a read followed by a write of one location).

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::keys::key_paths::{build_key_path, is_prefix, KeyPath};
use crate::typing::stmt_context::StmtTypeContext;

pub fn try_build_key_path(expr: &ExprPtr) -> Option<KeyPath> {
    let built = build_key_path(expr);
    built.success.then_some(built.path)
}

pub fn has_covering_write_key(type_ctx: &StmtTypeContext<'_>, path: &KeyPath) -> bool {
    type_ctx.held_key_paths.iter().any(|held| held.mode == ast::KeyMode::Write && is_prefix(&held.path, path))
}

/// The mode of a held key that covers the path; a write key wins over a read key.
pub fn covering_key_mode(type_ctx: &StmtTypeContext<'_>, path: &KeyPath) -> Option<ast::KeyMode> {
    let mut best = None;
    for held in type_ctx.held_key_paths.iter().filter(|held| is_prefix(&held.path, path)) {
        if best.is_none() || held.mode == ast::KeyMode::Write {
            best = Some(held.mode);
        }
    }
    best
}

fn key_path_indices_read(path: &ast::KeyPathExpr, target: &KeyPath) -> bool {
    path.segs.iter().any(|seg| matches!(seg, ast::KeySeg::KeySegIndex(index) if expr_reads_exact_path(&index.expr, target)))
}

fn block_reads_exact_path(block: &ast::BlockPtr, path: &KeyPath) -> bool {
    block.as_deref().is_some_and(|block| {
        block.stmts.iter().any(|stmt| stmt_reads_exact_path(stmt, path)) || expr_reads_exact_path(&block.tail_opt, path)
    })
}

fn stmt_reads_exact_path(stmt: &Stmt, path: &KeyPath) -> bool {
    let expr = |expr: &ExprPtr| expr_reads_exact_path(expr, path);
    let block = |block: &ast::BlockPtr| block_reads_exact_path(block, path);
    match stmt {
        Stmt::LetStmt(node) => expr(&node.binding.init),
        Stmt::VarStmt(node) => expr(&node.binding.init),
        Stmt::AssignStmt(node) => expr(&node.place) || expr(&node.value),
        Stmt::CompoundAssignStmt(node) => expr(&node.place) || expr(&node.value),
        Stmt::ExprStmt(node) => expr(&node.value),
        Stmt::DeferStmt(node) => block(&node.body),
        Stmt::UnsafeBlockStmt(node) => block(&node.body),
        Stmt::CtStmt(node) => block(&node.body),
        Stmt::RegionStmt(node) => expr(&node.opts_opt) || block(&node.body),
        Stmt::FrameStmt(node) => block(&node.body),
        Stmt::ReturnStmt(node) => expr(&node.value_opt),
        Stmt::BreakStmt(node) => expr(&node.value_opt),
        Stmt::KeyBlockStmt(node) => block(&node.body) || node.paths.iter().any(|key_path| key_path_indices_read(key_path, path)),
        _ => false,
    }
}

/// Whether the expression reads exactly the place the path names.
pub fn expr_reads_exact_path(expr: &ExprPtr, path: &KeyPath) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    if try_build_key_path(expr).is_some_and(|expr_path| expr_path == *path) {
        return true;
    }
    let reads = |inner: &ExprPtr| expr_reads_exact_path(inner, path);
    let block = |block: &ast::BlockPtr| block_reads_exact_path(block, path);
    let args = |args: &[ast::Arg]| args.iter().any(|arg| reads(&arg.value));
    let fields = |fields: &[ast::FieldInit]| fields.iter().any(|field| reads(&field.value));
    let invariant = |invariant: &Option<ast::LoopInvariant>| invariant.as_ref().is_some_and(|invariant| reads(&invariant.predicate));
    match &e.node {
        ExprNode::QualifiedApplyExpr(node) => match &node.args {
            ast::ApplyArgs::ParenArgs(paren) => args(&paren.args),
            ast::ApplyArgs::BraceArgs(brace) => fields(&brace.fields),
        },
        ExprNode::RangeExpr(node) => reads(&node.lhs) || reads(&node.rhs),
        ExprNode::BinaryExpr(node) => reads(&node.lhs) || reads(&node.rhs),
        ExprNode::PipelineExpr(node) => reads(&node.lhs) || reads(&node.rhs),
        ExprNode::CastExpr(node) => reads(&node.value),
        ExprNode::UnaryExpr(node) => reads(&node.value),
        ExprNode::DerefExpr(node) => reads(&node.value),
        ExprNode::AllocExpr(node) => reads(&node.value),
        ExprNode::TransmuteExpr(node) => reads(&node.value),
        ExprNode::PropagateExpr(node) => reads(&node.value),
        ExprNode::YieldExpr(node) => reads(&node.value),
        ExprNode::YieldFromExpr(node) => reads(&node.value),
        ExprNode::SyncExpr(node) => reads(&node.value),
        ExprNode::AddressOfExpr(node) => reads(&node.place),
        ExprNode::MoveExpr(node) => reads(&node.place),
        ExprNode::TupleExpr(node) => node.elements.iter().any(reads),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node).into_iter().any(reads),
        ExprNode::ArrayRepeatExpr(node) => reads(&node.value) || reads(&node.count),
        ExprNode::RecordExpr(node) => fields(&node.fields),
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            Some(ast::EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().any(reads),
            Some(ast::EnumPayload::EnumPayloadBrace(payload)) => fields(&payload.fields),
            None => false,
        },
        ExprNode::IfExpr(node) => reads(&node.cond) || reads(&node.then_expr) || reads(&node.else_expr),
        ExprNode::IfCaseExpr(node) => {
            reads(&node.scrutinee) || reads(&node.else_expr) || node.cases.iter().any(|case_clause| reads(&case_clause.body))
        }
        ExprNode::IfIsExpr(node) => reads(&node.scrutinee) || reads(&node.then_expr) || reads(&node.else_expr),
        ExprNode::LoopInfiniteExpr(node) => invariant(&node.invariant_opt) || block(&node.body),
        ExprNode::LoopConditionalExpr(node) => reads(&node.cond) || invariant(&node.invariant_opt) || block(&node.body),
        ExprNode::LoopIterExpr(node) => reads(&node.iter) || invariant(&node.invariant_opt) || block(&node.body),
        ExprNode::BlockExpr(node) => block(&node.block),
        ExprNode::UnsafeBlockExpr(node) => block(&node.block),
        ExprNode::ComptimeExpr(node) => reads(&node.body),
        ExprNode::CtIfExpr(node) => reads(&node.cond) || block(&node.then_block) || block(&node.else_block_opt),
        ExprNode::CtLoopIterExpr(node) => reads(&node.iter) || block(&node.body),
        ExprNode::AttributedExpr(node) => reads(&node.expr),
        ExprNode::ClosureExpr(node) => reads(&node.body),
        ExprNode::FieldAccessExpr(node) => reads(&node.base),
        ExprNode::TupleAccessExpr(node) => reads(&node.base),
        ExprNode::IndexAccessExpr(node) => reads(&node.base) || reads(&node.index),
        ExprNode::CallExpr(node) => reads(&node.callee) || args(&node.args),
        ExprNode::CallTypeArgsExpr(node) => reads(&node.callee) || args(&node.args),
        ExprNode::MethodCallExpr(node) => reads(&node.receiver) || args(&node.args),
        ExprNode::RaceExpr(node) => node.arms.iter().any(|arm| reads(&arm.expr) || reads(&arm.handler.value)),
        ExprNode::AllExpr(node) => node.exprs.iter().any(reads),
        ExprNode::EntryExpr(node) => reads(&node.expr),
        ExprNode::ParallelExpr(node) => reads(&node.domain) || node.opts.iter().any(|opt| reads(&opt.value)) || block(&node.body),
        ExprNode::SpawnExpr(node) => node.opts.iter().any(|opt| reads(&opt.value)) || block(&node.body),
        ExprNode::WaitExpr(node) => reads(&node.handle),
        ExprNode::DispatchExpr(node) => {
            reads(&node.range)
                || node.key_clause.as_ref().is_some_and(|key_clause| key_path_indices_read(&key_clause.key_path, path))
                || node.opts.iter().any(|opt| reads(&opt.chunk_expr) || reads(&opt.workgroup_expr))
                || block(&node.body)
        }
        _ => false,
    }
}

/// `place = place op value`, which a compound assignment would say directly.
pub fn is_compound_rewrite_candidate(value: &ExprPtr, path: &KeyPath) -> bool {
    let mut stripped = value;
    while let Some(ExprNode::AttributedExpr(attributed)) = stripped.as_deref().map(|expr| &expr.node) {
        stripped = &attributed.expr;
    }
    match stripped.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::BinaryExpr(binary)) if matches!(binary.op.as_str(), "+" | "-" | "*" | "/" | "%") => {
            try_build_key_path(&binary.lhs).is_some_and(|lhs_path| lhs_path == *path)
        }
        _ => false,
    }
}
