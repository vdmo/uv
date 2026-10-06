//! What typing records about expressions for the passes after it, and reads back
//! itself: the type each expression was given, the substitution of each generic call,
//! the overload each call selected, and the refinements left to a run-time check.
//!
//! The reference keys these by the address of the syntax node. Entries for expressions
//! keep the node alive, so that an address is never reused by a later temporary.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprPtr};

use super::types::TypeRef;
use crate::context::ScopeContext;
use crate::generics::monomorphize::TypeSubst;

/// The procedure a call with several candidates resolved to.
#[derive(Clone)]
pub struct SelectedCallTarget {
    pub module_path: Vec<String>,
    pub proc_name: String,
    pub proc_span: Span,
    pub generic: bool,
}

type ExprMap<V> = RefCell<HashMap<usize, (Arc<ast::Expr>, V)>>;

#[derive(Default)]
pub struct TypeStores {
    /// The type of each expression, as last typed or checked (as a value or a place).
    pub expr_types: ExprMap<TypeRef>,
    /// The same for expressions typed as values only.
    pub expr_value_types: ExprMap<TypeRef>,
    pub dynamic_refine_checks: ExprMap<Vec<TypeRef>>,
    pub generic_call_substs: RefCell<HashMap<usize, TypeSubst>>,
    pub selected_call_targets: RefCell<HashMap<usize, SelectedCallTarget>>,
}

fn key_of(expr: &Arc<ast::Expr>) -> usize {
    Arc::as_ptr(expr) as usize
}

fn call_key(call: &ast::CallExpr) -> usize {
    std::ptr::from_ref(call) as usize
}

/// Records the type of an expression; `as_value` when it was typed as a value.
pub fn store_expr_type(ctx: &ScopeContext<'_>, expr: &ExprPtr, ty: &TypeRef, as_value: bool) {
    let (Some(stores), Some(expr)) = (&ctx.stores, expr) else {
        return;
    };
    stores.expr_types.borrow_mut().insert(key_of(expr), (expr.clone(), ty.clone()));
    if as_value {
        stores.expr_value_types.borrow_mut().insert(key_of(expr), (expr.clone(), ty.clone()));
    }
}

pub fn stored_expr_type(ctx: &ScopeContext<'_>, expr: &ExprPtr) -> Option<TypeRef> {
    let found = ctx.stores.as_ref()?.expr_types.borrow().get(&key_of(expr.as_ref()?)).map(|(_, ty)| ty.clone())?;
    found.is_some().then_some(found)
}

fn expr_may_have_distinct_place_typing(expr: &ExprPtr) -> bool {
    use ast::ExprNode;
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::AttributedExpr(node)) => expr_may_have_distinct_place_typing(&node.expr),
        Some(
            ExprNode::IdentifierExpr(_)
            | ExprNode::FieldAccessExpr(_)
            | ExprNode::TupleAccessExpr(_)
            | ExprNode::IndexAccessExpr(_)
            | ExprNode::DerefExpr(_),
        ) => true,
        _ => false,
    }
}

/// The type an expression was given as a value. For a form that may also be typed as a
/// place, only a type recorded for the value counts.
pub fn stored_value_expr_type(ctx: &ScopeContext<'_>, expr: &ExprPtr) -> Option<TypeRef> {
    let stores = ctx.stores.as_ref()?;
    let found = stores.expr_value_types.borrow().get(&key_of(expr.as_ref()?)).map(|(_, ty)| ty.clone());
    if let Some(found) = found.filter(|ty| ty.is_some()) {
        return Some(found);
    }
    if expr_may_have_distinct_place_typing(expr) {
        return None;
    }
    stored_expr_type(ctx, expr)
}

pub fn record_dynamic_refine_check(ctx: &ScopeContext<'_>, expr: &ExprPtr, refinement: &TypeRef) {
    let (Some(stores), Some(expr)) = (&ctx.stores, expr) else {
        return;
    };
    stores.dynamic_refine_checks.borrow_mut().entry(key_of(expr)).or_insert_with(|| (expr.clone(), Vec::new())).1.push(refinement.clone());
}

pub fn record_generic_call_subst(ctx: &ScopeContext<'_>, call: &ast::CallExpr, subst: &TypeSubst) {
    if let Some(stores) = &ctx.stores {
        stores.generic_call_substs.borrow_mut().insert(call_key(call), subst.clone());
    }
}

pub fn record_selected_call_target(ctx: &ScopeContext<'_>, call: &ast::CallExpr, module_path: &[String], proc: &ast::ProcedureDecl) {
    if let Some(stores) = &ctx.stores {
        let target = SelectedCallTarget {
            module_path: module_path.to_vec(),
            proc_name: proc.name.clone(),
            proc_span: proc.span.clone(),
            generic: proc.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()),
        };
        stores.selected_call_targets.borrow_mut().insert(call_key(call), target);
    }
}

pub fn selected_call_target(ctx: &ScopeContext<'_>, call: &ast::CallExpr) -> Option<SelectedCallTarget> {
    ctx.stores.as_ref()?.selected_call_targets.borrow().get(&call_key(call)).cloned()
}
