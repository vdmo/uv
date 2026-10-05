//! Typing expressions: the dispatch over expression forms.

use uv_source::ast::{self, ExprNode, ExprPtr};

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;

use super::callbacks::{CheckResult, PlaceTypeResult};
use super::check_expr::check_expr;
use super::expr::path::{type_identifier_place, type_path_expr};
use super::expr_result::ExprTypeResult;
use super::literals::type_literal_expr;
use super::pending::pending;
use super::stmt_context::{ContractPhase, StmtTypeContext};
use super::type_env::{bind_of, TypeBinding, TypeEnv};
use super::type_predicates::{bitcopy_type, perm_of_type, strip_perm};
use super::types::{make_type_prim, Permission, TypeNode, TypeRef};
use crate::context::ScopeContext;
use crate::keys::key_paths::{build_key_path, is_place_expression, is_prefix};

/// The type of a name as a value: a binding, else a module-level static or procedure.
pub fn type_identifier_expr(ctx: &ScopeContext<'_>, env: &TypeEnv, name: &str) -> ExprTypeResult {
    super::expr::path::type_identifier_expr(ctx, env, name)
}

/// A binding derived from shared data is out of date once the keys were released.
pub fn emit_stale_binding_reference_warning(
    binding: &TypeBinding,
    type_ctx: &StmtTypeContext<'_>,
    span: Option<&Span>,
) {
    let Some(diags) = &type_ctx.diags else {
        return;
    };
    if !binding.stale_after_release || binding.stale_ok {
        return;
    }
    if let Some(diag) = make_diagnostic_by_id("W-CON-0011", Some(span.cloned().unwrap_or_default()))
    {
        emit(&mut diags.borrow_mut(), diag);
    }
}

/// A reference to a binding marked `[[deprecated]]` warns, with the message if any.
pub fn emit_deprecated_binding_reference_warning(
    binding: &TypeBinding,
    type_ctx: &StmtTypeContext<'_>,
    span: Option<&Span>,
) {
    let Some(diags) = &type_ctx.diags else {
        return;
    };
    if !binding.deprecated {
        return;
    }
    let Some(mut diag) = make_diagnostic_by_id("W-CNF-0601", span.cloned()) else {
        return;
    };
    if let Some(message) = binding
        .deprecated_message
        .as_ref()
        .filter(|message| !message.is_empty())
    {
        diag.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::Note,
            message: format!("deprecated message: {message}"),
            span: None,
            fix_text: None,
            label: None,
        });
    }
    emit(&mut diags.borrow_mut(), diag);
}

fn shared_access_mode_sufficient(held: ast::KeyMode, required: ast::KeyMode) -> bool {
    held == ast::KeyMode::Write || held == required
}

/// A shared place read or written under held keys must be covered by them in a mode
/// that allows the access.
fn check_shared_access_requirement(
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    ty: &TypeRef,
) -> Option<&'static str> {
    let required = type_ctx.shared_access_mode?;
    if expr.is_none()
        || ty.is_none()
        || type_ctx.suppress_shared_access_check
        || perm_of_type(ty) != Permission::Shared
        || !is_place_expression(expr)
    {
        return None;
    }
    let built = build_key_path(expr);
    if !built.success {
        return Some("E-CON-0034");
    }
    let mut covering = None;
    for held in &type_ctx.held_key_paths {
        if !is_prefix(&held.path, &built.path) {
            continue;
        }
        if covering.is_none() || held.mode == ast::KeyMode::Write {
            covering = Some(held.mode);
        }
    }
    match covering {
        Some(held) if !shared_access_mode_sufficient(held, required) => Some("E-CON-0005"),
        _ => None,
    }
}

pub fn type_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    env: &TypeEnv,
) -> ExprTypeResult {
    let Some(e) = expr.as_deref() else {
        return ExprTypeResult::default();
    };
    let mut result = type_expr_form(ctx, type_ctx, expr, e, env);
    if result.ok {
        if let Some(diag_id) = check_shared_access_requirement(type_ctx, expr, &result.r#type) {
            result.ok = false;
            result.diag_id = Some(diag_id);
        }
    }
    result
}

fn type_expr_form(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    e: &ast::Expr,
    env: &TypeEnv,
) -> ExprTypeResult {
    let _ = expr;
    match &e.node {
        ExprNode::ErrorExpr(_) => ExprTypeResult::typed(make_type_prim("!")),
        ExprNode::LiteralExpr(node) => type_literal_expr(node),
        ExprNode::PtrNullExpr(_) => ExprTypeResult::failed(Some("PtrNull-Infer-Err")),
        ExprNode::IdentifierExpr(node) => {
            let typed = type_identifier_expr(ctx, env, &node.name);
            if typed.ok {
                if let Some(binding) = bind_of(env, &node.name) {
                    emit_stale_binding_reference_warning(binding, type_ctx, Some(&e.span));
                    emit_deprecated_binding_reference_warning(binding, type_ctx, Some(&e.span));
                }
            }
            typed
        }
        ExprNode::PathExpr(node) => type_path_expr(ctx, node, env),
        ExprNode::MoveExpr(node) => {
            // Moving is an effect, which a contract predicate may not have.
            if type_ctx.require_pure {
                return ExprTypeResult::failed(Some("E-SEM-2802"));
            }
            let place = type_place(ctx, type_ctx, &node.place, env);
            ExprTypeResult {
                ok: place.ok,
                diag_id: place.diag_id,
                r#type: if place.ok { place.r#type } else { None },
                diag_detail: place.diag_detail,
                diag_span: place.diag_span,
                ..Default::default()
            }
        }
        ExprNode::CopyExpr(node) => {
            let value = type_expr(ctx, type_ctx, &node.value, env);
            if !value.ok {
                return ExprTypeResult {
                    diag_id: value.diag_id,
                    diag_detail: value.diag_detail,
                    diag_span: value.diag_span,
                    ..Default::default()
                };
            }
            if !bitcopy_type(ctx, &value.r#type) {
                return ExprTypeResult {
                    diag_id: Some("E-UNS-0107"),
                    diag_span: node.value.as_deref().map(|value| value.span.clone()),
                    ..Default::default()
                };
            }
            ExprTypeResult::typed(value.r#type)
        }
        ExprNode::ResultExpr(_) => {
            if type_ctx.contract_phase != ContractPhase::Postcondition {
                return ExprTypeResult::failed(Some("E-SEM-2806"));
            }
            let ty = if type_ctx.return_type.is_some() {
                type_ctx.return_type.clone()
            } else {
                make_type_prim("()")
            };
            ExprTypeResult::typed(ty)
        }
        ExprNode::FenceExpr(_) => {
            if type_ctx.in_speculative {
                return ExprTypeResult::failed(Some("E-CON-0096"));
            }
            ExprTypeResult::typed(make_type_prim("()"))
        }
        ExprNode::QualifiedNameExpr(_) | ExprNode::QualifiedApplyExpr(_) => {
            ExprTypeResult::failed(Some("ResolveExpr-Ident-Err"))
        }
        // Splices are gone before typing; the reference types them as nothing.
        ExprNode::SpliceExprNode(_) | ExprNode::SpliceIdentNode(_) => ExprTypeResult::default(),
        _ => {
            pending(ast::expr_kind(e));
            ExprTypeResult::default()
        }
    }
}

/// The type of a place: a name, or a field, element or target reached from one.
pub fn type_place(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    env: &TypeEnv,
) -> PlaceTypeResult {
    let Some(e) = expr.as_deref() else {
        return PlaceTypeResult::default();
    };
    let mut result = match &e.node {
        ExprNode::IdentifierExpr(node) => {
            let typed = type_identifier_place(ctx, env, &node.name);
            if typed.ok {
                if let Some(binding) = bind_of(env, &node.name) {
                    emit_stale_binding_reference_warning(binding, type_ctx, Some(&e.span));
                    emit_deprecated_binding_reference_warning(binding, type_ctx, Some(&e.span));
                }
            }
            typed
        }
        ExprNode::AttributedExpr(_)
        | ExprNode::FieldAccessExpr(_)
        | ExprNode::TupleAccessExpr(_)
        | ExprNode::DerefExpr(_)
        | ExprNode::IndexAccessExpr(_) => {
            pending(match &e.node {
                ExprNode::AttributedExpr(_) => "AttributedExpr place",
                ExprNode::FieldAccessExpr(_) => "FieldAccessExpr place",
                ExprNode::TupleAccessExpr(_) => "TupleAccessExpr place",
                ExprNode::DerefExpr(_) => "DerefExpr place",
                _ => "IndexAccessExpr place",
            });
            PlaceTypeResult::default()
        }
        _ => PlaceTypeResult::default(),
    };
    if result.ok {
        if let Some(diag_id) = check_shared_access_requirement(type_ctx, expr, &result.r#type) {
            result.ok = false;
            result.diag_id = Some(diag_id);
            result.diag_span = Some(e.span.clone());
        }
    }
    result
}

fn closure_type_has_shared_deps(hint: &TypeRef) -> bool {
    matches!(
        strip_perm(hint).as_deref().map(|ty| &ty.node),
        Some(TypeNode::Closure {
            deps_opt: Some(_),
            ..
        })
    )
}

/// Whether the expression has the expected type. The forms that are checked against
/// the expectation in their own way are ported with their typing.
pub fn check_expr_against(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    expected: &TypeRef,
    env: &TypeEnv,
) -> CheckResult {
    let Some(e) = expr.as_deref().filter(|_| expected.is_some()) else {
        return CheckResult::default();
    };
    // A closure expected to declare its shared dependencies must not spawn.
    if closure_type_has_shared_deps(expected) {
        pending("ClosureCapture");
        return CheckResult::default();
    }
    match &e.node {
        ExprNode::AttributedExpr(_)
        | ExprNode::ComptimeExpr(_)
        | ExprNode::IfExpr(_)
        | ExprNode::IfIsExpr(_)
        | ExprNode::BlockExpr(_)
        | ExprNode::RecordExpr(_)
        | ExprNode::QuoteExpr(_)
        | ExprNode::EnumLiteralExpr(_)
        | ExprNode::UnsafeBlockExpr(_) => {
            pending(ast::expr_kind(e));
            return CheckResult::default();
        }
        ExprNode::ClosureExpr(_)
            if matches!(
                strip_perm(expected).as_deref().map(|ty| &ty.node),
                Some(TypeNode::Closure { .. })
            ) =>
        {
            pending("ClosureExpr");
            return CheckResult::default();
        }
        _ => {}
    }
    let type_expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env);
    let type_ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let if_case_check = |_: &ast::IfCaseExpr, _: &TypeRef| {
        pending("IfCaseExpr");
        CheckResult::default()
    };
    let check = check_expr(
        ctx,
        expr,
        expected,
        &type_expr_fn,
        &type_ident_fn,
        Some(&if_case_check),
    );
    if !check.ok {
        // In a dynamic context a refinement that cannot be proved is checked at run time.
        if type_ctx.contract_dynamic {
            pending("DynamicRefinement");
        }
        return check;
    }
    if let ExprNode::IdentifierExpr(ident) = &e.node {
        if let Some(binding) = bind_of(env, &ident.name) {
            emit_deprecated_binding_reference_warning(binding, type_ctx, Some(&e.span));
        }
    }
    check
}
