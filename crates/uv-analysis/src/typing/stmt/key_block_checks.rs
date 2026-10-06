//! The checks of the less common key blocks: `ordered` paths, paths indexed by values
//! known only at run time, and speculative blocks.

use std::collections::HashSet;
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::{IdKey, ScopeContext};
use crate::contracts::purity::{check_purity, ContractContext};
use crate::contracts::struct_equal::expr_struct_equal;
use crate::contracts::verification::{evaluate_constant, static_proof, ConstValue};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::const_len::const_len;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, TypeEnv};

/// The root and field segments of a path, without its indices.
fn ordered_base_matches(lhs: &ast::KeyPathExpr, rhs: &ast::KeyPathExpr) -> bool {
    let fields = |path: &'_ ast::KeyPathExpr| -> Vec<(bool, String)> {
        path.segs
            .iter()
            .filter_map(|seg| match seg {
                ast::KeySeg::KeySegField(field) => Some((field.marked, field.name.clone())),
                ast::KeySeg::KeySegIndex(_) => None,
            })
            .collect()
    };
    let (lhs_fields, rhs_fields) = (fields(lhs), fields(rhs));
    id_eq(&lhs.root, &rhs.root)
        && lhs_fields.len() == rhs_fields.len()
        && lhs_fields.iter().zip(&rhs_fields).all(|(l, r)| l.0 == r.0 && id_eq(&l.1, &r.1))
}

/// `ordered` asks for the keys in index order, so the paths must differ only in their
/// indices.
pub fn ordered_comparable_paths(paths: &[ast::KeyPathExpr]) -> bool {
    match paths.split_first() {
        Some((first, rest)) => rest.iter().all(|path| ordered_base_matches(path, first)),
        None => true,
    }
}

/// Whether the order is already known at compile time, which makes `ordered` redundant.
pub fn statically_comparable_ordered_paths(ctx: &ScopeContext<'_>, paths: &[ast::KeyPathExpr]) -> bool {
    let Some(first) = paths.first().filter(|_| ordered_comparable_paths(paths)) else {
        return false;
    };
    paths.iter().all(|path| {
        path.segs.len() == first.segs.len()
            && first.segs.iter().zip(&path.segs).all(|(lhs, rhs)| match (lhs, rhs) {
                (ast::KeySeg::KeySegField(lhs), ast::KeySeg::KeySegField(rhs)) => lhs.marked == rhs.marked && id_eq(&lhs.name, &rhs.name),
                (ast::KeySeg::KeySegIndex(lhs), ast::KeySeg::KeySegIndex(rhs)) => {
                    const_len(ctx, &lhs.expr).is_ok() && const_len(ctx, &rhs.expr).is_ok()
                }
                _ => false,
            })
    })
}

/// Whether the expression mentions one of the names. The wide form also looks into
/// calls, literals, conditionals and suspension points.
fn expr_uses_name_from_set(expr: &ExprPtr, names: &HashSet<IdKey>, wide: bool) -> bool {
    let Some(e) = expr.as_deref().filter(|_| !names.is_empty()) else {
        return false;
    };
    let uses = |inner: &ExprPtr| expr_uses_name_from_set(inner, names, wide);
    match &e.node {
        ExprNode::IdentifierExpr(node) => names.contains(&id_key_of(&node.name)),
        ExprNode::AttributedExpr(node) => uses(&node.expr),
        ExprNode::UnaryExpr(node) => uses(&node.value),
        ExprNode::CastExpr(node) => uses(&node.value),
        ExprNode::DerefExpr(node) => uses(&node.value),
        ExprNode::PropagateExpr(node) => uses(&node.value),
        ExprNode::AddressOfExpr(node) => uses(&node.place),
        ExprNode::MoveExpr(node) => uses(&node.place),
        ExprNode::BinaryExpr(node) => uses(&node.lhs) || uses(&node.rhs),
        ExprNode::FieldAccessExpr(node) => uses(&node.base),
        ExprNode::TupleAccessExpr(node) => uses(&node.base),
        ExprNode::IndexAccessExpr(node) => uses(&node.base) || uses(&node.index),
        _ if !wide => false,
        ExprNode::YieldExpr(node) => uses(&node.value),
        ExprNode::YieldFromExpr(node) => uses(&node.value),
        ExprNode::SyncExpr(node) => uses(&node.value),
        ExprNode::CallExpr(node) => uses(&node.callee) || node.args.iter().any(|arg| uses(&arg.value)),
        ExprNode::MethodCallExpr(node) => uses(&node.receiver) || node.args.iter().any(|arg| uses(&arg.value)),
        ExprNode::TupleExpr(node) => node.elements.iter().any(uses),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node).into_iter().any(uses),
        ExprNode::ArrayRepeatExpr(node) => uses(&node.value) || uses(&node.count),
        ExprNode::RecordExpr(node) => node.fields.iter().any(|field| uses(&field.value)),
        ExprNode::IfExpr(node) => uses(&node.cond) || uses(&node.then_expr) || uses(&node.else_expr),
        ExprNode::IfCaseExpr(node) => uses(&node.scrutinee) || node.cases.iter().any(|arm| uses(&arm.body)) || uses(&node.else_expr),
        ExprNode::IfIsExpr(node) => uses(&node.scrutinee) || uses(&node.then_expr) || uses(&node.else_expr),
        ExprNode::RangeExpr(node) => uses(&node.lhs) || uses(&node.rhs),
        _ => false,
    }
}

/// Whether a key path is indexed by a binding of the enclosing `parallel` block,
/// which each task sees its own value of.
pub fn paths_indexed_by_parallel_binding(paths: &[ast::KeyPathExpr], parallel_bindings: &HashSet<IdKey>) -> bool {
    paths
        .iter()
        .flat_map(|path| &path.segs)
        .any(|seg| matches!(seg, ast::KeySeg::KeySegIndex(index) if expr_uses_name_from_set(&index.expr, parallel_bindings, true)))
}

fn const_i64(expr: &ExprPtr) -> Option<i64> {
    match evaluate_constant(expr) {
        ConstValue::Int(value) => Some(value),
        _ => None,
    }
}

/// The constant bounds of a loop range, with the end exclusive.
fn loop_range_bounds(expr: &ExprPtr) -> Option<(i64, i64)> {
    let ExprNode::RangeExpr(range) = &expr.as_deref()?.node else {
        return None;
    };
    let (lo, hi) = (const_i64(&range.lhs)?, const_i64(&range.rhs)?);
    match range.kind {
        ast::RangeKind::Exclusive => Some((lo, hi)),
        ast::RangeKind::Inclusive => Some((lo, hi.wrapping_add(1))),
        _ => None,
    }
}

/// `name`, `name + k`, `name - k` or `k + name`, as the name and the offset.
fn parse_affine_index_expr(expr: &ExprPtr) -> Option<(&str, i64)> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => Some((&ident.name, 0)),
        ExprNode::PathExpr(path) => path.path.is_empty().then_some((path.name.as_str(), 0)),
        ExprNode::AttributedExpr(attributed) => parse_affine_index_expr(&attributed.expr),
        ExprNode::BinaryExpr(binary) if binary.op == "+" || binary.op == "-" => {
            if let Some(rhs_const) = const_i64(&binary.rhs) {
                let (name, offset) = parse_affine_index_expr(&binary.lhs)?;
                let delta = if binary.op == "+" { rhs_const } else { rhs_const.wrapping_neg() };
                return Some((name, offset.wrapping_add(delta)));
            }
            let lhs_const = const_i64(&binary.lhs).filter(|_| binary.op == "+")?;
            let (name, offset) = parse_affine_index_expr(&binary.rhs)?;
            Some((name, offset.wrapping_add(lhs_const)))
        }
        _ => None,
    }
}

/// Whether two index expressions are known to name the same element.
fn provably_equivalent_index_expr(env: &TypeEnv, lhs: &ExprPtr, rhs: &ExprPtr) -> bool {
    let (Some(l), Some(r)) = (lhs.as_deref(), rhs.as_deref()) else {
        return false;
    };
    if expr_struct_equal(lhs, rhs) {
        return true;
    }
    let lhs_const = evaluate_constant(lhs);
    if lhs_const != ConstValue::Unknown && lhs_const == evaluate_constant(rhs) {
        return true;
    }
    match (&l.node, &r.node) {
        (ExprNode::IdentifierExpr(lhs_ident), ExprNode::IdentifierExpr(rhs_ident)) => {
            id_eq(&lhs_ident.name, &rhs_ident.name) && bind_of(env, &lhs_ident.name).is_some() && bind_of(env, &rhs_ident.name).is_some()
        }
        (ExprNode::IdentifierExpr(_), _) => false,
        (ExprNode::PathExpr(lhs_path), ExprNode::PathExpr(rhs_path)) => lhs_path.path == rhs_path.path && id_eq(&lhs_path.name, &rhs_path.name),
        _ => false,
    }
}

/// Whether two index expressions are known to name different elements: different
/// constants, a proved inequality, different offsets from one name, an index particular
/// to each parallel task, or loop variables over ranges that do not meet.
fn provably_disjoint_index_expr(type_ctx: &StmtTypeContext<'_>, lhs: &ExprPtr, rhs: &ExprPtr) -> bool {
    let (Some(l), Some(r)) = (lhs.as_deref(), rhs.as_deref()) else {
        return false;
    };
    match (evaluate_constant(lhs), evaluate_constant(rhs)) {
        (ConstValue::Int(lhs_value), ConstValue::Int(rhs_value)) => return lhs_value != rhs_value,
        (ConstValue::Bool(lhs_value), ConstValue::Bool(rhs_value)) => return lhs_value != rhs_value,
        _ => {}
    }
    if let Some(proof_ctx) = type_ctx.proof_ctx.as_deref() {
        let not_equal = Some(Arc::new(ast::Expr {
            span: l.span.clone(),
            node: ExprNode::BinaryExpr(ast::BinaryExpr { op: "!=".to_string(), lhs: lhs.clone(), rhs: rhs.clone() }),
        }));
        if static_proof(proof_ctx, &not_equal).provable {
            return true;
        }
    }
    if let (Some((lhs_name, lhs_offset)), Some((rhs_name, rhs_offset))) = (parse_affine_index_expr(lhs), parse_affine_index_expr(rhs)) {
        if id_eq(lhs_name, rhs_name) && lhs_offset != rhs_offset {
            return true;
        }
    }
    if let Some(scope) = type_ctx.parallel_capture_scopes.as_ref().and_then(|scopes| scopes.last()) {
        let parallel_bindings = scope.bindings.borrow();
        if expr_uses_name_from_set(lhs, &parallel_bindings, false) || expr_uses_name_from_set(rhs, &parallel_bindings, false) {
            return true;
        }
    }
    if let (Some(ranges), ExprNode::IdentifierExpr(lhs_ident), ExprNode::IdentifierExpr(rhs_ident)) =
        (type_ctx.loop_iteration_ranges.as_deref(), &l.node, &r.node)
    {
        if !id_eq(&lhs_ident.name, &rhs_ident.name) {
            let bounds = |name: &str| ranges.get(&id_key_of(name)).and_then(loop_range_bounds);
            if let (Some((lhs_lo, lhs_hi)), Some((rhs_lo, rhs_hi))) = (bounds(&lhs_ident.name), bounds(&rhs_ident.name)) {
                return lhs_hi <= rhs_lo || rhs_hi <= lhs_lo;
            }
        }
    }
    false
}

/// The root a place is reached from and the indices applied on the way.
fn extract_root_and_indices<'e>(expr: &'e ExprPtr, indices: &mut Vec<&'e ExprPtr>) -> Option<&'e str> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => Some(&ident.name),
        ExprNode::FieldAccessExpr(field) => extract_root_and_indices(&field.base, indices),
        ExprNode::IndexAccessExpr(index) => {
            let root = extract_root_and_indices(&index.base, indices)?;
            indices.push(&index.index);
            Some(root)
        }
        _ => None,
    }
}

/// The indices an expression applies to places reached from the root.
fn collect_index_accesses_on_root<'e>(expr: &'e ExprPtr, root: &str, out: &mut Vec<&'e ExprPtr>) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    match &e.node {
        ExprNode::IndexAccessExpr(node) => {
            if extract_root_and_indices(&node.base, &mut Vec::new()).is_some_and(|base_root| id_eq(base_root, root)) {
                out.push(&node.index);
            }
            collect_index_accesses_on_root(&node.base, root, out);
            collect_index_accesses_on_root(&node.index, root, out);
        }
        ExprNode::FieldAccessExpr(node) => collect_index_accesses_on_root(&node.base, root, out),
        ExprNode::UnaryExpr(node) => collect_index_accesses_on_root(&node.value, root, out),
        ExprNode::CastExpr(node) => collect_index_accesses_on_root(&node.value, root, out),
        ExprNode::BinaryExpr(node) => {
            collect_index_accesses_on_root(&node.lhs, root, out);
            collect_index_accesses_on_root(&node.rhs, root, out);
        }
        ExprNode::CallExpr(node) => {
            collect_index_accesses_on_root(&node.callee, root, out);
            node.args.iter().for_each(|arg| collect_index_accesses_on_root(&arg.value, root, out));
        }
        ExprNode::MethodCallExpr(node) => {
            collect_index_accesses_on_root(&node.receiver, root, out);
            node.args.iter().for_each(|arg| collect_index_accesses_on_root(&arg.value, root, out));
        }
        _ => {}
    }
}

/// Whether an assignment in the body writes one element of the root and reads another
/// that might be the same, with at least one of the two indices not a constant.
pub fn body_has_dynamic_index_conflict(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, env: &TypeEnv, body: &ast::Block, root: &str) -> bool {
    let is_dynamic = |index: &ExprPtr| const_len(ctx, index).is_err();
    body.stmts.iter().any(|stmt| {
        let Stmt::AssignStmt(assign) = stmt else {
            return false;
        };
        let mut lhs_indices = Vec::new();
        if !extract_root_and_indices(&assign.place, &mut lhs_indices).is_some_and(|place_root| id_eq(place_root, root)) {
            return false;
        }
        let mut rhs_indices = Vec::new();
        collect_index_accesses_on_root(&assign.value, root, &mut rhs_indices);
        lhs_indices.iter().any(|idx| {
            rhs_indices.iter().any(|rhs_idx| {
                (is_dynamic(idx) || is_dynamic(rhs_idx))
                    && !provably_equivalent_index_expr(env, idx, rhs_idx)
                    && !provably_disjoint_index_expr(type_ctx, idx, rhs_idx)
            })
        })
    })
}

/// The first call in the expression that is not pure, which a speculative block may
/// not make since the block may run more than once.
pub fn find_impure_speculative_call_expr(ctx: &ScopeContext<'_>, expr: &ExprPtr) -> Option<Span> {
    let e = expr.as_deref()?;
    if matches!(e.node, ExprNode::CallExpr(_) | ExprNode::MethodCallExpr(_)) {
        let purity = check_purity(&ContractContext { scope_ctx: Some(ctx), ..Default::default() }, expr);
        if !purity.ok {
            return purity.span.or_else(|| Some(e.span.clone()));
        }
    }
    let find = |inner: &ExprPtr| find_impure_speculative_call_expr(ctx, inner);
    let first = |exprs: &mut dyn Iterator<Item = &ExprPtr>| -> Option<Span> {
        for inner in exprs {
            if let found @ Some(_) = find(inner) {
                return found;
            }
        }
        None
    };
    match &e.node {
        ExprNode::AttributedExpr(node) => find(&node.expr),
        ExprNode::UnaryExpr(node) => find(&node.value),
        ExprNode::CastExpr(node) => find(&node.value),
        ExprNode::DerefExpr(node) => find(&node.value),
        ExprNode::PropagateExpr(node) => find(&node.value),
        ExprNode::YieldExpr(node) => find(&node.value),
        ExprNode::YieldFromExpr(node) => find(&node.value),
        ExprNode::SyncExpr(node) => find(&node.value),
        ExprNode::AddressOfExpr(node) => find(&node.place),
        ExprNode::MoveExpr(node) => find(&node.place),
        ExprNode::BinaryExpr(node) => find(&node.lhs).or_else(|| find(&node.rhs)),
        ExprNode::PipelineExpr(node) => find(&node.lhs).or_else(|| find(&node.rhs)),
        ExprNode::FieldAccessExpr(node) => find(&node.base),
        ExprNode::TupleAccessExpr(node) => find(&node.base),
        ExprNode::IndexAccessExpr(node) => find(&node.base).or_else(|| find(&node.index)),
        ExprNode::CallExpr(node) => find(&node.callee).or_else(|| first(&mut node.args.iter().map(|arg| &arg.value))),
        ExprNode::MethodCallExpr(node) => find(&node.receiver).or_else(|| first(&mut node.args.iter().map(|arg| &arg.value))),
        ExprNode::TupleExpr(node) => first(&mut node.elements.iter()),
        ExprNode::ArrayExpr(node) => first(&mut ast::array_expr_subexprs(node).into_iter()),
        ExprNode::ArrayRepeatExpr(node) => find(&node.value).or_else(|| find(&node.count)),
        ExprNode::RecordExpr(node) => first(&mut node.fields.iter().map(|field| &field.value)),
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            Some(ast::EnumPayload::EnumPayloadParen(payload)) => first(&mut payload.elements.iter()),
            Some(ast::EnumPayload::EnumPayloadBrace(payload)) => first(&mut payload.fields.iter().map(|field| &field.value)),
            None => None,
        },
        ExprNode::IfExpr(node) => find(&node.cond).or_else(|| find(&node.then_expr)).or_else(|| find(&node.else_expr)),
        ExprNode::IfCaseExpr(node) => {
            find(&node.scrutinee).or_else(|| first(&mut node.cases.iter().map(|arm| &arm.body))).or_else(|| find(&node.else_expr))
        }
        ExprNode::IfIsExpr(node) => find(&node.scrutinee).or_else(|| find(&node.then_expr)).or_else(|| find(&node.else_expr)),
        ExprNode::BlockExpr(node) => find_impure_speculative_call_block(ctx, &node.block),
        ExprNode::UnsafeBlockExpr(node) => find_impure_speculative_call_block(ctx, &node.block),
        _ => None,
    }
}

/// The same over a block: its expression statements, then its tail.
pub fn find_impure_speculative_call_block(ctx: &ScopeContext<'_>, block: &ast::BlockPtr) -> Option<Span> {
    let block = block.as_deref()?;
    block
        .stmts
        .iter()
        .find_map(|stmt| match stmt {
            Stmt::ExprStmt(expr_stmt) => find_impure_speculative_call_expr(ctx, &expr_stmt.value),
            _ => None,
        })
        .or_else(|| find_impure_speculative_call_expr(ctx, &block.tail_opt))
}

fn expr_expensive(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    match &e.node {
        ExprNode::CallExpr(_)
        | ExprNode::MethodCallExpr(_)
        | ExprNode::LoopInfiniteExpr(_)
        | ExprNode::LoopConditionalExpr(_)
        | ExprNode::LoopIterExpr(_)
        | ExprNode::ParallelExpr(_)
        | ExprNode::DispatchExpr(_)
        | ExprNode::SpawnExpr(_)
        | ExprNode::RaceExpr(_)
        | ExprNode::AllExpr(_) => true,
        ExprNode::AttributedExpr(node) => expr_expensive(&node.expr),
        ExprNode::UnaryExpr(node) => expr_expensive(&node.value),
        ExprNode::CastExpr(node) => expr_expensive(&node.value),
        ExprNode::DerefExpr(node) => expr_expensive(&node.value),
        ExprNode::PropagateExpr(node) => expr_expensive(&node.value),
        ExprNode::YieldExpr(node) => expr_expensive(&node.value),
        ExprNode::YieldFromExpr(node) => expr_expensive(&node.value),
        ExprNode::SyncExpr(node) => expr_expensive(&node.value),
        ExprNode::MoveExpr(node) => expr_expensive(&node.place),
        ExprNode::AddressOfExpr(node) => expr_expensive(&node.place),
        ExprNode::BinaryExpr(node) => expr_expensive(&node.lhs) || expr_expensive(&node.rhs),
        ExprNode::RangeExpr(node) => expr_expensive(&node.lhs) || expr_expensive(&node.rhs),
        ExprNode::FieldAccessExpr(node) => expr_expensive(&node.base),
        ExprNode::TupleAccessExpr(node) => expr_expensive(&node.base),
        ExprNode::IndexAccessExpr(node) => expr_expensive(&node.base) || expr_expensive(&node.index),
        ExprNode::TupleExpr(node) => node.elements.iter().any(expr_expensive),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node).into_iter().any(expr_expensive),
        ExprNode::ArrayRepeatExpr(node) => expr_expensive(&node.value) || expr_expensive(&node.count),
        ExprNode::RecordExpr(node) => node.fields.iter().any(|field| expr_expensive(&field.value)),
        ExprNode::IfExpr(node) => expr_expensive(&node.cond) || expr_expensive(&node.then_expr) || expr_expensive(&node.else_expr),
        ExprNode::IfCaseExpr(node) => {
            expr_expensive(&node.scrutinee) || node.cases.iter().any(|arm| expr_expensive(&arm.body)) || expr_expensive(&node.else_expr)
        }
        ExprNode::IfIsExpr(node) => expr_expensive(&node.scrutinee) || expr_expensive(&node.then_expr) || expr_expensive(&node.else_expr),
        ExprNode::BlockExpr(node) => block_may_be_expensive_to_reexecute(&node.block),
        ExprNode::UnsafeBlockExpr(node) => block_may_be_expensive_to_reexecute(&node.block),
        _ => false,
    }
}

/// Whether running the block again would repeat a call, a loop or a task.
pub fn block_may_be_expensive_to_reexecute(block: &ast::BlockPtr) -> bool {
    let Some(block) = block.as_deref() else {
        return false;
    };
    block.stmts.iter().any(|stmt| match stmt {
        Stmt::ExprStmt(node) => expr_expensive(&node.value),
        Stmt::LetStmt(node) => expr_expensive(&node.binding.init),
        Stmt::VarStmt(node) => expr_expensive(&node.binding.init),
        Stmt::AssignStmt(node) => expr_expensive(&node.place) || expr_expensive(&node.value),
        Stmt::CompoundAssignStmt(node) => expr_expensive(&node.place) || expr_expensive(&node.value),
        Stmt::ReturnStmt(node) => expr_expensive(&node.value_opt),
        Stmt::BreakStmt(node) => expr_expensive(&node.value_opt),
        Stmt::DeferStmt(node) => block_may_be_expensive_to_reexecute(&node.body),
        Stmt::UnsafeBlockStmt(node) => block_may_be_expensive_to_reexecute(&node.body),
        Stmt::CtStmt(node) => block_may_be_expensive_to_reexecute(&node.body),
        Stmt::RegionStmt(node) => expr_expensive(&node.opts_opt) || block_may_be_expensive_to_reexecute(&node.body),
        Stmt::FrameStmt(node) => block_may_be_expensive_to_reexecute(&node.body),
        _ => false,
    }) || expr_expensive(&block.tail_opt)
}
