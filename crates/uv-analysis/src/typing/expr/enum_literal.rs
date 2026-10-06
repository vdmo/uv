//! Enum literals: `Enum::Variant`, with a tuple or record payload.

use std::collections::{HashMap, HashSet};

use uv_core::span::Span;
use uv_source::ast;

use super::call::emit_deprecated_reference_warning_from_attrs;
use crate::context::ScopeContext;
use crate::generics::generic_params::{bind_type_params, required_param_count, total_param_count};
use crate::generics::monomorphize::{build_substitution, instantiate_type, TypeSubst};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::callbacks::CheckResult;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::TypeEnv;
use crate::typing::type_expr::check_expr_against;
use crate::typing::type_lookup::lookup_enum_decl;
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm;
use crate::typing::types::*;

struct EnumLiteralTarget<'c> {
    enum_path: TypePath,
    enum_decl: &'c ast::EnumDecl,
    variant: &'c ast::VariantDecl,
}

/// Where a deprecation warning for the enum points: the first payload expression.
fn enum_literal_ref_span(expr: &ast::EnumLiteralExpr) -> Option<Span> {
    match expr.payload_opt.as_ref()? {
        ast::EnumPayload::EnumPayloadParen(payload) => payload.elements.first()?.as_deref().map(|element| element.span.clone()),
        ast::EnumPayload::EnumPayloadBrace(payload) => payload.fields.first()?.value.as_deref().map(|value| value.span.clone()),
    }
}

fn resolve_enum_literal_target<'c>(ctx: &'c ScopeContext<'_>, expr: &ast::EnumLiteralExpr) -> Option<EnumLiteralTarget<'c>> {
    let (variant_name, enum_path) = expr.path.split_last().filter(|(_, enum_path)| !enum_path.is_empty())?;
    let enum_decl = lookup_enum_decl(ctx, enum_path)?;
    let variant = enum_decl.variants.iter().find(|variant| id_eq(&variant.name, variant_name))?;
    Some(EnumLiteralTarget { enum_path: enum_path.to_vec(), enum_decl, variant })
}

/// Checks the payload written against the variant's. A mismatch of shape fails without
/// naming a rule.
fn check_enum_literal_payload_against(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::EnumLiteralExpr,
    target: &EnumLiteralTarget<'_>,
    payload_type_ctx: &ScopeContext<'_>,
    subst: &TypeSubst,
    env: &TypeEnv,
) -> CheckResult {
    let failed = |diag_id: Option<&'static str>| CheckResult { diag_id, ..Default::default() };
    let ok = CheckResult { ok: true, ..Default::default() };
    let (decl_payload, expr_payload) = match (&target.variant.payload_opt, &expr.payload_opt) {
        (None, None) => return ok,
        (Some(decl_payload), Some(expr_payload)) => (decl_payload, expr_payload),
        _ => return failed(None),
    };
    match decl_payload {
        ast::VariantPayload::VariantPayloadTuple(tuple_payload) => {
            let ast::EnumPayload::EnumPayloadParen(paren) = expr_payload else {
                return failed(None);
            };
            if paren.elements.len() != tuple_payload.elements.len() {
                return failed(Some("E-TYP-2008"));
            }
            for (element, element_type) in paren.elements.iter().zip(&tuple_payload.elements) {
                let element_type = match lower_type(payload_type_ctx, element_type) {
                    Ok(lowered) => instantiate_type(&lowered, subst),
                    Err(diag_id) => return failed(diag_id),
                };
                let check = check_expr_against(ctx, type_ctx, element, &element_type, env);
                if !check.ok {
                    return check;
                }
            }
            ok
        }
        ast::VariantPayload::VariantPayloadRecord(record_payload) => {
            let ast::EnumPayload::EnumPayloadBrace(brace) = expr_payload else {
                return failed(None);
            };
            let mut seen = HashSet::new();
            if brace.fields.iter().any(|field_init| !seen.insert(id_key_of(&field_init.name))) {
                return failed(None);
            }
            let mut field_types = HashMap::new();
            for field_decl in &record_payload.fields {
                match lower_type(payload_type_ctx, &field_decl.r#type) {
                    Ok(lowered) => {
                        field_types.entry(id_key_of(&field_decl.name)).or_insert_with(|| instantiate_type(&lowered, subst));
                    }
                    Err(diag_id) => return failed(diag_id),
                }
            }
            if field_types.len() != seen.len() {
                return failed(Some("E-TYP-2009"));
            }
            for field_init in &brace.fields {
                let Some(field_type) = field_types.get(&id_key_of(&field_init.name)) else {
                    return CheckResult {
                        diag_id: Some("E-TYP-2009"),
                        diag_span: field_init.value.as_deref().map(|value| value.span.clone()),
                        ..Default::default()
                    };
                };
                let check = check_expr_against(ctx, type_ctx, &field_init.value, field_type, env);
                if !check.ok {
                    return check;
                }
            }
            ok
        }
    }
}

/// Types an enum literal on its own. A literal of a generic enum has no type without
/// an expected one.
pub fn type_enum_literal_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::EnumLiteralExpr,
    env: &TypeEnv,
) -> ExprTypeResult {
    let Some(target) = resolve_enum_literal_target(ctx, expr) else {
        return ExprTypeResult::default();
    };
    emit_deprecated_reference_warning_from_attrs(&target.enum_decl.attrs, type_ctx, enum_literal_ref_span(expr));
    if total_param_count(&target.enum_decl.generic_params) > 0 {
        return ExprTypeResult::default();
    }
    let check = check_enum_literal_payload_against(ctx, type_ctx, expr, &target, ctx, &TypeSubst::new(), env);
    if !check.ok {
        return ExprTypeResult { diag_id: check.diag_id, diag_detail: check.diag_detail, diag_span: check.diag_span, ..Default::default() };
    }
    ExprTypeResult::typed(make_type_path(target.enum_path))
}

/// Checks an enum literal against an expected application of its enum, which supplies
/// the type arguments.
pub fn check_enum_literal_expr_against(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::EnumLiteralExpr,
    expected: &TypeRef,
    env: &TypeEnv,
) -> CheckResult {
    let Some(target) = resolve_enum_literal_target(ctx, expr) else {
        return CheckResult::default();
    };
    let expected_base = strip_perm(expected);
    let Some(base) = expected_base.as_deref() else {
        return CheckResult::default();
    };
    let (Some(expected_path), Some(expected_args)) = (applied_type_path(base), applied_type_args(base)) else {
        return CheckResult::default();
    };
    if expected_path.len() != target.enum_path.len()
        || !expected_path.iter().zip(&target.enum_path).all(|(l, r)| id_key_of(l) == id_key_of(r))
    {
        return CheckResult::default();
    }
    emit_deprecated_reference_warning_from_attrs(&target.enum_decl.attrs, type_ctx, enum_literal_ref_span(expr));

    // The payload types are lowered with the enum's type parameters in scope.
    let mut payload_ctx = ctx.clone();
    payload_ctx.scopes = bind_type_params(ctx, &target.enum_decl.generic_params);
    let mut subst = TypeSubst::new();
    let generics = &target.enum_decl.generic_params;
    match generics {
        Some(params) => {
            let provided = expected_args.len();
            if provided < required_param_count(generics) || provided > total_param_count(generics) {
                return CheckResult { diag_id: Some("E-TYP-2303"), ..Default::default() };
            }
            subst = build_substitution(&params.params, expected_args);
        }
        None if !expected_args.is_empty() => return CheckResult { diag_id: Some("E-TYP-2303"), ..Default::default() },
        None => {}
    }
    check_enum_literal_payload_against(ctx, type_ctx, expr, &target, &payload_ctx, &subst, env)
}
