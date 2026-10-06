//! Loop invariants: a pure `bool` that holds on entry and that the body maintains.

use std::collections::HashSet;
use std::sync::Arc;

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::{IdKey, ScopeContext};
use crate::contracts::purity::is_pure_in_scope;
use crate::contracts::verification::{add_predicate_facts_at, static_proof, static_proof_at, StaticProofContext};
use crate::resolve::scopes::id_key_of;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::TypeEnv;
use crate::typing::type_expr::type_expr;
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

fn contains_result(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    match &e.node {
        ExprNode::ResultExpr(_) => true,
        ExprNode::BinaryExpr(node) => contains_result(&node.lhs) || contains_result(&node.rhs),
        ExprNode::UnaryExpr(node) => contains_result(&node.value),
        ExprNode::PipelineExpr(node) => contains_result(&node.lhs) || contains_result(&node.rhs),
        ExprNode::CallExpr(node) => contains_result(&node.callee) || node.args.iter().any(|arg| contains_result(&arg.value)),
        ExprNode::MethodCallExpr(node) => contains_result(&node.receiver) || node.args.iter().any(|arg| contains_result(&arg.value)),
        _ => false,
    }
}

/// The invariant itself: no `@result`, pure, a `bool`, and provable where the loop
/// starts unless contracts are checked at run time here.
pub fn validate_loop_invariant_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    invariant: &ast::LoopInvariant,
) -> Option<&'static str> {
    if contains_result(&invariant.predicate) {
        return Some("E-SEM-2854");
    }
    if !is_pure_in_scope(ctx, &invariant.predicate) {
        return Some("E-SEM-3004");
    }
    let inv_type = type_expr(ctx, type_ctx, &invariant.predicate, env);
    if !inv_type.ok {
        return inv_type.diag_id;
    }
    if !matches!(strip_perm_and_refine(&inv_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "bool") {
        return Some("E-SEM-2851");
    }
    if !type_ctx.contract_dynamic {
        let proof_ctx = type_ctx.proof_ctx.as_deref().cloned().unwrap_or_default();
        if !static_proof(&proof_ctx, &invariant.predicate).provable {
            return Some("E-SEM-2830");
        }
    }
    None
}

/// The names an invariant speaks of.
fn collect_invariant_names(expr: &ExprPtr, out: &mut HashSet<IdKey>) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    match &e.node {
        ExprNode::IdentifierExpr(node) => {
            out.insert(id_key_of(&node.name));
        }
        ExprNode::PathExpr(node) if node.path.is_empty() => {
            out.insert(id_key_of(&node.name));
        }
        ExprNode::BinaryExpr(node) => {
            collect_invariant_names(&node.lhs, out);
            collect_invariant_names(&node.rhs, out);
        }
        ExprNode::UnaryExpr(node) => collect_invariant_names(&node.value, out),
        ExprNode::FieldAccessExpr(node) => collect_invariant_names(&node.base, out),
        ExprNode::TupleAccessExpr(node) => collect_invariant_names(&node.base, out),
        ExprNode::IndexAccessExpr(node) => {
            collect_invariant_names(&node.base, out);
            collect_invariant_names(&node.index, out);
        }
        ExprNode::CallExpr(node) => {
            collect_invariant_names(&node.callee, out);
            node.args.iter().for_each(|arg| collect_invariant_names(&arg.value, out));
        }
        ExprNode::MethodCallExpr(node) => {
            collect_invariant_names(&node.receiver, out);
            node.args.iter().for_each(|arg| collect_invariant_names(&arg.value, out));
        }
        ExprNode::CastExpr(node) => collect_invariant_names(&node.value, out),
        ExprNode::RangeExpr(node) => {
            collect_invariant_names(&node.lhs, out);
            collect_invariant_names(&node.rhs, out);
        }
        ExprNode::EntryExpr(node) => collect_invariant_names(&node.expr, out),
        _ => {}
    }
}

fn place_mutates_invariant_name(place: &ExprPtr, names: &HashSet<IdKey>) -> bool {
    let Some(e) = place.as_deref().filter(|_| !names.is_empty()) else {
        return false;
    };
    match &e.node {
        ExprNode::IdentifierExpr(node) => names.contains(&id_key_of(&node.name)),
        ExprNode::PathExpr(node) => node.path.is_empty() && names.contains(&id_key_of(&node.name)),
        ExprNode::FieldAccessExpr(node) => place_mutates_invariant_name(&node.base, names),
        ExprNode::TupleAccessExpr(node) => place_mutates_invariant_name(&node.base, names),
        ExprNode::IndexAccessExpr(node) => place_mutates_invariant_name(&node.base, names),
        ExprNode::MoveExpr(node) => place_mutates_invariant_name(&node.place, names),
        ExprNode::DerefExpr(node) => place_mutates_invariant_name(&node.value, names),
        _ => false,
    }
}

/// The block a statement holds, for the statements whose body may assign.
fn nested_body(stmt: &Stmt) -> Option<&ast::BlockPtr> {
    match stmt {
        Stmt::DeferStmt(node) => Some(&node.body),
        Stmt::RegionStmt(node) => Some(&node.body),
        Stmt::FrameStmt(node) => Some(&node.body),
        Stmt::UnsafeBlockStmt(node) => Some(&node.body),
        Stmt::KeyBlockStmt(node) => Some(&node.body),
        _ => None,
    }
}

fn block_mutates_invariant_name(block: &ast::BlockPtr, names: &HashSet<IdKey>) -> bool {
    let Some(block) = block.as_deref().filter(|_| !names.is_empty()) else {
        return false;
    };
    block.stmts.iter().any(|stmt| match stmt {
        Stmt::AssignStmt(node) => place_mutates_invariant_name(&node.place, names),
        Stmt::CompoundAssignStmt(node) => place_mutates_invariant_name(&node.place, names),
        other => nested_body(other).is_some_and(|body| block_mutates_invariant_name(body, names)),
    })
}

/// Whether the body assigns to a name the invariant speaks of.
pub fn violates_loop_invariant_maintenance(invariant: &ast::LoopInvariant, body: &ast::BlockPtr) -> bool {
    let mut names = HashSet::new();
    collect_invariant_names(&invariant.predicate, &mut names);
    block_mutates_invariant_name(body, &names)
}

fn assignment_root_name(place: &ExprPtr) -> Option<IdKey> {
    match &place.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(id_key_of(&node.name)),
        ExprNode::FieldAccessExpr(node) => assignment_root_name(&node.base),
        ExprNode::TupleAccessExpr(node) => assignment_root_name(&node.base),
        ExprNode::IndexAccessExpr(node) => assignment_root_name(&node.base),
        ExprNode::AttributedExpr(node) => assignment_root_name(&node.expr),
        _ => None,
    }
}

/// The obligation with the assigned value for the name. A form the substitution does
/// not reach into blocks it when it mentions the name.
fn substitute_identifier_in_proof_expr(expr: &ExprPtr, name: &IdKey, replacement: &ExprPtr, blocked: &mut bool) -> ExprPtr {
    let Some(e) = expr.as_deref().filter(|_| !*blocked) else {
        return expr.clone();
    };
    let mut sub = |inner: &ExprPtr| substitute_identifier_in_proof_expr(inner, name, replacement, blocked);
    let node = match &e.node {
        ExprNode::IdentifierExpr(node) => return if id_key_of(&node.name) == *name { replacement.clone() } else { expr.clone() },
        ExprNode::LiteralExpr(_) | ExprNode::PtrNullExpr(_) => return expr.clone(),
        ExprNode::BinaryExpr(node) => {
            let lhs = sub(&node.lhs);
            let rhs = sub(&node.rhs);
            ExprNode::BinaryExpr(ast::BinaryExpr { lhs, rhs, ..node.clone() })
        }
        ExprNode::UnaryExpr(node) => ExprNode::UnaryExpr(ast::UnaryExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::FieldAccessExpr(node) => ExprNode::FieldAccessExpr(ast::FieldAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::TupleAccessExpr(node) => ExprNode::TupleAccessExpr(ast::TupleAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::IndexAccessExpr(node) => {
            let base = sub(&node.base);
            let index = sub(&node.index);
            ExprNode::IndexAccessExpr(ast::IndexAccessExpr { base, index })
        }
        ExprNode::CastExpr(node) => ExprNode::CastExpr(ast::CastExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::AttributedExpr(node) => ExprNode::AttributedExpr(ast::AttributedExpr { expr: sub(&node.expr), ..node.clone() }),
        _ => {
            let mut expr_names = HashSet::new();
            collect_invariant_names(expr, &mut expr_names);
            if expr_names.contains(name) {
                *blocked = true;
            }
            return expr.clone();
        }
    };
    Some(Arc::new(ast::Expr { span: e.span.clone(), node }))
}

/// Whether a conditional loop's body maintains the invariant: either it assigns to
/// none of its names, or the invariant with the body's assignments applied follows
/// from the invariant and the loop condition.
pub fn loop_invariant_maintained_by_body(
    type_ctx: &StmtTypeContext<'_>,
    invariant: &ast::LoopInvariant,
    condition: &ExprPtr,
    body: &ast::BlockPtr,
) -> bool {
    let mut invariant_names = HashSet::new();
    collect_invariant_names(&invariant.predicate, &mut invariant_names);
    if !block_mutates_invariant_name(body, &invariant_names) {
        return true;
    }
    let Some(block) = body.as_deref() else {
        return true;
    };
    // The assignments are applied backwards, from the last statement to the first.
    let mut proof_obligation = invariant.predicate.clone();
    for stmt in block.stmts.iter().rev() {
        let supported = match stmt {
            Stmt::AssignStmt(node) => match assignment_root_name(&node.place) {
                Some(root) if invariant_names.contains(&root) => {
                    let plain = matches!(node.place.as_deref().map(|place| &place.node), Some(ExprNode::IdentifierExpr(ident)) if id_key_of(&ident.name) == root);
                    let mut blocked = false;
                    if plain {
                        proof_obligation = substitute_identifier_in_proof_expr(&proof_obligation, &root, &node.value, &mut blocked);
                    }
                    plain && !blocked
                }
                _ => true,
            },
            Stmt::CompoundAssignStmt(node) => !place_mutates_invariant_name(&node.place, &invariant_names),
            other => !nested_body(other).is_some_and(|nested| block_mutates_invariant_name(nested, &invariant_names)),
        };
        if !supported {
            return false;
        }
    }
    let mut proof_ctx: StaticProofContext = type_ctx.proof_ctx.as_deref().cloned().unwrap_or_default();
    add_predicate_facts_at(&mut proof_ctx, &invariant.predicate, &invariant.span);
    let condition_span = condition.as_deref().map_or(&invariant.span, |condition| &condition.span);
    add_predicate_facts_at(&mut proof_ctx, condition, condition_span);
    static_proof_at(&proof_ctx, &invariant.span, &proof_obligation).provable
}
