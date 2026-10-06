//! Typing expressions: the dispatch over expression forms.

use uv_source::ast::{self, ExprNode, ExprPtr};

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;

use super::callbacks::{CheckResult, PlaceTypeResult};
use super::check_expr::check_expr;
use super::expr::small;
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
        ExprNode::BinaryExpr(node) => super::expr::binary::type_binary_expr(ctx, type_ctx, node, env),
        ExprNode::IfExpr(node) => super::expr::if_expr::type_if_expr(ctx, type_ctx, node, env),
        ExprNode::BlockExpr(node) => super::stmt::block::type_block_expr(ctx, type_ctx, node, env),
        ExprNode::UnaryExpr(node) => small::type_unary_expr(ctx, type_ctx, node, env, &e.span),
        ExprNode::CastExpr(node) => small::type_cast_expr(ctx, type_ctx, node, env),
        ExprNode::TupleExpr(node) => small::type_tuple_expr(ctx, type_ctx, node, env),
        ExprNode::ArrayExpr(node) => {
            small::type_array_expr(ctx, node, &|inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env))
        }
        ExprNode::ArrayRepeatExpr(node) => {
            small::type_array_repeat_expr(ctx, node, &|inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env))
        }
        ExprNode::SizeofExpr(node) => small::type_layout_query_expr(ctx, &node.r#type),
        ExprNode::AlignofExpr(node) => small::type_layout_query_expr(ctx, &node.r#type),
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
        ExprNode::FieldAccessExpr(node) => super::expr::field_access::type_field_access_expr(ctx, type_ctx, node, env),
        ExprNode::TupleAccessExpr(node) => super::expr::tuple_access::type_tuple_access_expr(ctx, type_ctx, node, env),
        ExprNode::DerefExpr(node) => super::expr::access::type_deref_expr(ctx, type_ctx, node, env, &e.span),
        ExprNode::IndexAccessExpr(node) => {
            super::expr::access::type_index_access_expr(ctx, type_ctx, node, env, &|inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env))
        }
        ExprNode::AddressOfExpr(node) => super::expr::access::type_address_of_expr(ctx, type_ctx, node, env),
        ExprNode::RangeExpr(node) => super::expr::access::type_range_expr(ctx, type_ctx, node, env),
        ExprNode::UnsafeBlockExpr(node) => super::expr::access::type_unsafe_block_expr(ctx, type_ctx, node, env),
        ExprNode::LoopInfiniteExpr(node) => {
            super::expr::loops::type_loop_infinite_expr(ctx, type_ctx, node, env, &|name: &str| type_identifier_expr(ctx, env, name))
        }
        ExprNode::LoopConditionalExpr(node) => {
            super::expr::loops::type_loop_conditional_expr(ctx, type_ctx, node, env, &|name: &str| type_identifier_expr(ctx, env, name))
        }
        ExprNode::LoopIterExpr(node) => {
            super::expr::loops::type_loop_iter_expr(ctx, type_ctx, node, env, &|name: &str| type_identifier_expr(ctx, env, name))
        }
        ExprNode::IfIsExpr(node) => super::expr::if_case::type_if_is_expr(ctx, type_ctx, node, env),
        ExprNode::IfCaseExpr(node) => super::expr::if_case::type_if_case_expr(ctx, type_ctx, node, env),
        ExprNode::RecordExpr(node) => super::expr::record_literal::type_record_expr(ctx, type_ctx, node, env, None),
        ExprNode::EnumLiteralExpr(node) => super::expr::enum_literal::type_enum_literal_expr(ctx, type_ctx, node, env),
        ExprNode::MethodCallExpr(node) => super::expr::method_call::type_method_call_expr(ctx, type_ctx, node, env, &e.span),
        ExprNode::CallExpr(node) => super::expr::call::type_call_expr(ctx, type_ctx, node, env),
        ExprNode::CallTypeArgsExpr(node) => super::expr::call::type_call_type_args_expr(ctx, type_ctx, node, env),
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
        ExprNode::FieldAccessExpr(node) => super::expr::field_access::type_field_access_place(ctx, type_ctx, node, env),
        ExprNode::TupleAccessExpr(node) => super::expr::tuple_access::type_tuple_access_place(ctx, type_ctx, node, env),
        ExprNode::DerefExpr(node) => super::expr::access::type_deref_place(ctx, type_ctx, node, env, &e.span),
        ExprNode::IndexAccessExpr(node) => {
            super::expr::access::type_index_access_place(ctx, type_ctx, node, env, &|inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env))
        }
        ExprNode::AttributedExpr(_) => {
            pending("AttributedExpr place");
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
    // A record or enum literal takes its type arguments from the expected type; when it
    // neither fits nor names a rule, the general check below decides.
    match &e.node {
        ExprNode::RecordExpr(node) => {
            let typed_record = super::expr::record_literal::type_record_expr(ctx, type_ctx, node, env, Some(expected));
            if typed_record.ok {
                let sub = super::subtyping::subtyping(ctx, &typed_record.r#type, expected);
                if !sub.ok {
                    return CheckResult { diag_id: sub.diag_id, ..Default::default() };
                }
                if sub.subtype {
                    return CheckResult { ok: true, ..Default::default() };
                }
            }
            if typed_record.diag_id.is_some() {
                return CheckResult {
                    diag_id: typed_record.diag_id,
                    diag_detail: typed_record.diag_detail,
                    diag_span: typed_record.diag_span,
                    ..Default::default()
                };
            }
        }
        ExprNode::EnumLiteralExpr(node) => {
            let enum_check = super::expr::enum_literal::check_enum_literal_expr_against(ctx, type_ctx, node, expected, env);
            if enum_check.ok || enum_check.diag_id.is_some() {
                return CheckResult {
                    ok: enum_check.ok,
                    diag_id: enum_check.diag_id,
                    diag_detail: enum_check.diag_detail,
                    diag_span: enum_check.diag_span,
                    ..Default::default()
                };
            }
        }
        _ => {}
    }
    match &e.node {
        ExprNode::IfExpr(node) => {
            let result = super::expr::if_expr::check_if_expr(ctx, type_ctx, node, expected, env);
            // In a dynamic context a refinement that cannot be proved is checked at run time.
            if !result.ok && type_ctx.contract_dynamic {
                pending("DynamicRefinement");
            }
            return result;
        }
        ExprNode::IfIsExpr(node) => {
            let result = super::expr::if_case::check_if_is_expr(ctx, type_ctx, node, env, expected);
            if !result.ok && type_ctx.contract_dynamic {
                pending("DynamicRefinement");
            }
            return result;
        }
        ExprNode::UnsafeBlockExpr(node) => {
            let checked = super::expr::access::check_unsafe_block_expr(ctx, type_ctx, node, env, expected);
            return CheckResult {
                ok: checked.ok,
                diag_id: checked.diag_id,
                diag_detail: checked.diag_detail,
                diag_span: checked.diag_span,
                ..Default::default()
            };
        }
        ExprNode::BlockExpr(node) => {
            let checked = super::stmt::block::check_block_expr(ctx, type_ctx, node, env, expected);
            return CheckResult {
                ok: checked.ok,
                diag_id: checked.diag_id,
                diag_detail: checked.diag_detail,
                diag_span: checked.diag_span,
                ..Default::default()
            };
        }
        ExprNode::AttributedExpr(_)
        | ExprNode::ComptimeExpr(_)
        | ExprNode::QuoteExpr(_) => {
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
    let type_place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let type_ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let if_case_check = |if_case: &ast::IfCaseExpr, expected_type: &TypeRef| {
        super::expr::if_case::check_if_case_expr(ctx, type_ctx, if_case, env, expected_type)
    };
    let check = check_expr(
        ctx,
        expr,
        expected,
        &type_expr_fn,
        Some(&type_place_fn),
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
