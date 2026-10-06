//! Typing expression statements.
//!
//! The expression must type; its value is dropped. A call of a transition on a named
//! modal value moves the binding to the transition's target state.

use uv_source::ast::{self, ExprNode, ExprPtr};

use super::block::{collect_break_flow, StmtTypeResult};
use crate::context::{ScopeContext, TypeDecl};
use crate::modal::lookup::lookup_transition_decl;
use crate::resolve::scopes::id_key_of;
use crate::typing::callbacks::ExprTypeFn;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, TypeEnv};
use crate::typing::type_expr::type_expr;
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

/// How the reference names the expression in the detail of a failure.
fn expr_kind_name(expr: &ExprPtr) -> String {
    let Some(e) = expr.as_deref() else {
        return "null".to_string();
    };
    match &e.node {
        ExprNode::AttributedExpr(node) if node.expr.is_some() => {
            format!("AttributedExpr(inner-present)->{}", expr_kind_name(&node.expr))
        }
        ExprNode::AttributedExpr(_) => "AttributedExpr(inner-null)".to_string(),
        ExprNode::IfExpr(_) => "IfExpr".to_string(),
        ExprNode::BlockExpr(_) => "BlockExpr".to_string(),
        ExprNode::ComptimeExpr(_) => "ComptimeExpr".to_string(),
        ExprNode::IdentifierExpr(node) => format!("IdentifierExpr({})", node.name),
        ExprNode::LiteralExpr(_) => "LiteralExpr".to_string(),
        ExprNode::BinaryExpr(_) => "BinaryExpr".to_string(),
        ExprNode::CallExpr(_) => "CallExpr".to_string(),
        ExprNode::MethodCallExpr(_) => "MethodCallExpr".to_string(),
        _ => "Expr".to_string(),
    }
}

/// The same permissions and refinements around another base.
fn reapply_qualifiers(original: &TypeRef, transitioned_base: &TypeRef) -> TypeRef {
    match original.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, base }) => make_type_perm(*perm, reapply_qualifiers(base, transitioned_base)),
        Some(TypeNode::Refine { base, predicate }) => {
            make_type_refine(reapply_qualifiers(base, transitioned_base), predicate.clone())
        }
        _ => transitioned_base.clone(),
    }
}

/// The type a modal state value has after the named transition, if it is one.
fn transition_type_for_method(ctx: &ScopeContext<'_>, current_type: &TypeRef, method_name: &str) -> Option<TypeRef> {
    let stripped = strip_perm_and_refine(current_type);
    let TypeNode::ModalState(modal) = &stripped.as_deref()?.node else {
        return None;
    };
    // The path is looked up as written.
    let TypeDecl::Modal(decl) = ctx.sigma.types.get(&modal.path)? else {
        return None;
    };
    let transition = lookup_transition_decl(decl, &modal.state, method_name)?;
    let target = make_type_modal_state(modal.path.clone(), &transition.target_state, modal.generic_args.clone());
    Some(reapply_qualifiers(current_type, &target))
}

fn update_binding_type(env: &mut TypeEnv, name: &str, ty: &TypeRef) {
    let key = id_key_of(name);
    if let Some(binding) = env.scopes.iter_mut().rev().find_map(|scope| scope.get_mut(&key)) {
        binding.r#type = ty.clone();
        binding.storage_type = ty.clone();
    }
}

pub fn type_expr_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::ExprStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> StmtTypeResult {
    let typed = if node.value.is_none() {
        Default::default()
    } else {
        let via_env = type_expr(ctx, type_ctx, &node.value, env);
        if via_env.ok || via_env.diag_id.is_some() {
            via_env
        } else {
            type_expr_fn(&node.value)
        }
    };
    if !typed.ok {
        let mut detail = typed.diag_detail;
        if !detail.is_empty() {
            detail.push_str("; ");
        }
        detail.push_str("expr-kind=");
        detail.push_str(&expr_kind_name(&node.value));
        if let Some(ExprNode::AttributedExpr(attributed)) = node.value.as_deref().map(|expr| &expr.node) {
            if let Some(ExprNode::IdentifierExpr(ident)) = attributed.expr.as_deref().map(|expr| &expr.node) {
                detail.push_str("; ident-binding=");
                detail.push_str(if bind_of(env, &ident.name).is_some() { "present" } else { "absent" });
            }
        }
        return StmtTypeResult { diag_id: typed.diag_id, diag_detail: detail, ..Default::default() };
    }
    let mut out_env = match &type_ctx.env_ref {
        Some(cell) => cell.borrow().clone(),
        None => env.clone(),
    };
    if let Some(ExprNode::MethodCallExpr(method)) = node.value.as_deref().map(|expr| &expr.node) {
        let receiver = match method.receiver.as_deref().map(|expr| &expr.node) {
            Some(ExprNode::MoveExpr(moved)) => &moved.place,
            _ => &method.receiver,
        };
        if let Some(ExprNode::IdentifierExpr(ident)) = receiver.as_deref().map(|expr| &expr.node) {
            let transitioned =
                bind_of(&out_env, &ident.name).and_then(|binding| transition_type_for_method(ctx, &binding.r#type, &method.name));
            if let Some(transitioned) = transitioned {
                update_binding_type(&mut out_env, &ident.name, &transitioned);
            }
        }
    }
    let flow = collect_break_flow(ctx, type_ctx, &out_env, type_expr_fn, &node.value, false);
    StmtTypeResult { ok: true, env: out_env, flow, ..Default::default() }
}
