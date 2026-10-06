//! The small expression forms: `sizeof`, `alignof`, unary operators, casts, tuple and
//! array literals.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::span::Span;
use uv_source::ast::{self, ArraySegment, ExprNode, ExprPtr};

use crate::composite::classes::{class_dispatchability_diagnostic, type_implements_class};
use crate::context::ScopeContext;
use crate::modal::lookup::{has_state, lookup_modal_decl};
use crate::modal::modal_widen::widen_warn_cond;
use crate::typing::callbacks::ExprTypeFn;
use crate::typing::check_expr::{get_prim_name, is_int_type};
use crate::typing::const_len::const_len;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::type_env::TypeEnv;
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::type_expr;
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::{bitcopy_type, cast_valid, strip_perm};
use crate::typing::type_wf::type_wf;
use crate::typing::types::*;

fn fail(diag_id: &'static str) -> ExprTypeResult {
    ExprTypeResult::failed(Some(diag_id))
}

/// `sizeof(T)` and `alignof(T)`: a `usize`, for a well-formed type.
pub fn type_layout_query_expr(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> ExprTypeResult {
    if ty.is_none() {
        return ExprTypeResult::default();
    }
    match lower_type(ctx, ty).and_then(|lowered| type_wf(ctx, &lowered)) {
        Ok(()) => ExprTypeResult::typed(make_type_prim("usize")),
        Err(diag_id) => ExprTypeResult::failed(diag_id),
    }
}

/// The modal a state belongs to, as a type.
pub fn modal_ref_type(modal: &TypeModalState) -> TypeRef {
    if modal.modal_ref.args().is_empty() {
        make_type_path(modal.modal_ref.path().to_vec())
    } else {
        make_type_apply(modal.modal_ref.path().to_vec(), modal.modal_ref.args().to_vec())
    }
}

/// `!`, `-` and `widen`.
pub fn type_unary_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::UnaryExpr,
    env: &TypeEnv,
    span: &Span,
) -> ExprTypeResult {
    let operand = type_expr(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), &expr.value, env);
    if !operand.ok {
        return ExprTypeResult::failed(operand.diag_id);
    }
    let stripped = strip_perm(&operand.r#type);
    let Some(stripped_ty) = stripped.as_deref() else {
        return ExprTypeResult::default();
    };
    if expr.op == "widen" {
        // A state value becomes a value of its modal, keeping the permission.
        if let TypeNode::ModalState(modal) = &stripped_ty.node {
            if !lookup_modal_decl(ctx, &modal.path).is_some_and(|decl| has_state(decl, &modal.state)) {
                return ExprTypeResult::default();
            }
            if widen_warn_cond(ctx, &modal.path, &modal.state) {
                if let Some(diags) = &type_ctx.diags {
                    if let Some(diag) = make_diagnostic_by_id("W-SYS-4010", Some(span.clone())) {
                        emit(&mut diags.borrow_mut(), diag);
                    }
                }
            }
            let general = modal_ref_type(modal);
            return ExprTypeResult::typed(match operand.r#type.as_deref().map(|ty| &ty.node) {
                Some(TypeNode::Perm { perm, .. }) => make_type_perm(*perm, general),
                _ => general,
            });
        }
        let already_general = applied_type_path(stripped_ty).is_some_and(|path| lookup_modal_decl(ctx, path).is_some());
        return fail(if already_general { "Widen-AlreadyGeneral" } else { "Widen-NonModal" });
    }
    let base = match &stripped_ty.node {
        TypeNode::Refine { base, .. } => strip_perm(base),
        _ => stripped.clone(),
    };
    let Some(TypeNode::Prim(name)) = base.as_deref().map(|ty| &ty.node) else {
        return ExprTypeResult::default();
    };
    let accepted = match expr.op.as_str() {
        "!" => name == "bool" || is_int_type(name),
        "-" => matches!(name.as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "f16" | "f32" | "f64"),
        _ => false,
    };
    if accepted {
        ExprTypeResult::typed(make_type_prim(name))
    } else {
        ExprTypeResult::default()
    }
}

/// A place in the sense of provenance: a name or dereference, or a field or element of one.
pub fn is_place_expr(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IdentifierExpr(_) | ExprNode::DerefExpr(_)) => true,
        Some(ExprNode::FieldAccessExpr(node)) => is_place_expr(&node.base),
        Some(ExprNode::TupleAccessExpr(node)) => is_place_expr(&node.base),
        Some(ExprNode::IndexAccessExpr(node)) => is_place_expr(&node.base),
        _ => false,
    }
}

/// `value as T`: a numeric conversion, or a place viewed as `$Class` it implements.
pub fn type_cast_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::CastExpr, env: &TypeEnv) -> ExprTypeResult {
    if expr.value.is_none() || expr.r#type.is_none() {
        return ExprTypeResult::default();
    }
    let value = type_expr(ctx, type_ctx, &expr.value, env);
    if !value.ok {
        return ExprTypeResult::failed(value.diag_id);
    }
    let target = match lower_type(ctx, &expr.r#type) {
        Ok(target) => target,
        Err(diag_id) => return ExprTypeResult::failed(diag_id),
    };
    if let Some(TypeNode::Dynamic(class_path)) = target.as_deref().map(|ty| &ty.node) {
        if type_implements_class(ctx, &strip_perm(&value.r#type), class_path) {
            if let Some(diag_id) = class_dispatchability_diagnostic(ctx, class_path) {
                return fail(diag_id);
            }
            if is_place_expr(&expr.value) {
                return ExprTypeResult::typed(target);
            }
        }
    }
    if cast_valid(&value.r#type, &target) {
        ExprTypeResult::typed(target)
    } else {
        fail("T-Cast-Invalid")
    }
}

pub fn type_tuple_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::TupleExpr, env: &TypeEnv) -> ExprTypeResult {
    if expr.elements.is_empty() {
        return ExprTypeResult::typed(make_type_prim("()"));
    }
    let mut element_types = Vec::with_capacity(expr.elements.len());
    for elem in &expr.elements {
        let typed = type_expr(ctx, type_ctx, elem, env);
        if !typed.ok {
            return ExprTypeResult::failed(typed.diag_id);
        }
        element_types.push(typed.r#type);
    }
    ExprTypeResult::typed(make_type_tuple(element_types))
}

/// A repeat count: an integer expression with a constant value.
fn repeat_count(ctx: &ScopeContext<'_>, count: &ExprPtr, type_expr_fn: ExprTypeFn<'_>) -> Result<u64, Option<&'static str>> {
    let count_type = type_expr_fn(count);
    if !count_type.ok {
        return Err(count_type.diag_id);
    }
    if !get_prim_name(&count_type.r#type).is_some_and(is_int_type) {
        return Err(Some("E-TYP-1812"));
    }
    const_len(ctx, count).map_err(|diag_id| Some(diag_id.unwrap_or("E-TYP-1812")))
}

/// `[a, b, ..c; n]`: all segments of one element type; an empty literal has no type of
/// its own.
pub fn type_array_expr(ctx: &ScopeContext<'_>, expr: &ast::ArrayExpr, type_expr_fn: ExprTypeFn<'_>) -> ExprTypeResult {
    let mut element_type: TypeRef = None;
    let mut total_length: u64 = 0;
    if expr.elements.is_empty() {
        return ExprTypeResult::default();
    }
    for segment in &expr.elements {
        let segment_type = match segment {
            ArraySegment::ArrayElemSegment(node) => {
                if node.value.is_none() {
                    return ExprTypeResult::default();
                }
                let typed = type_expr_fn(&node.value);
                if !typed.ok {
                    return ExprTypeResult::failed(typed.diag_id);
                }
                typed.r#type
            }
            ArraySegment::ArrayRepeatSegment(node) => {
                if node.value.is_none() || node.count.is_none() {
                    return ExprTypeResult::default();
                }
                let value = type_expr_fn(&node.value);
                if !value.ok {
                    return ExprTypeResult::failed(value.diag_id);
                }
                if !bitcopy_type(ctx, &value.r#type) {
                    return fail("E-UNS-0107");
                }
                match repeat_count(ctx, &node.count, type_expr_fn) {
                    Ok(len) => total_length = total_length.wrapping_add(len),
                    Err(diag_id) => return ExprTypeResult::failed(diag_id),
                }
                value.r#type
            }
        };
        if element_type.is_none() {
            element_type = segment_type;
        } else if !type_equiv(&element_type, &segment_type) {
            return ExprTypeResult::default();
        }
        if matches!(segment, ArraySegment::ArrayElemSegment(_)) {
            total_length = total_length.wrapping_add(1);
        }
    }
    ExprTypeResult::typed(make_type_array(element_type, total_length, None))
}

/// `[value; n]`.
pub fn type_array_repeat_expr(ctx: &ScopeContext<'_>, expr: &ast::ArrayRepeatExpr, type_expr_fn: ExprTypeFn<'_>) -> ExprTypeResult {
    if expr.value.is_none() || expr.count.is_none() {
        return ExprTypeResult::default();
    }
    let value = type_expr_fn(&expr.value);
    if !value.ok {
        return ExprTypeResult::failed(value.diag_id);
    }
    // The count's type is checked before the value's copyability, its value after.
    let count_type = type_expr_fn(&expr.count);
    if !count_type.ok {
        return ExprTypeResult::failed(count_type.diag_id);
    }
    if !get_prim_name(&count_type.r#type).is_some_and(is_int_type) {
        return fail("E-TYP-1812");
    }
    if !bitcopy_type(ctx, &value.r#type) {
        return fail("E-UNS-0107");
    }
    match const_len(ctx, &expr.count) {
        Ok(len) => ExprTypeResult::typed(make_type_array(value.r#type, len, None)),
        Err(diag_id) => fail(diag_id.unwrap_or("E-TYP-1812")),
    }
}
