//! Field access: a field of a record, of a modal state, or of `Self` inside a class.

use uv_source::ast;

use super::call::emit_deprecated_reference_warning_from_attrs;
use crate::composite::classes::class_field_table;
use crate::context::{ScopeContext, TypeDecl};
use crate::generics::monomorphize::{build_modal_ref_substitution, instantiate_type, TypeSubst};
use crate::modal::lookup::modal_field_visible;
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::callbacks::PlaceTypeResult;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::signature::subst_self_type;
use crate::typing::stmt_context::{suppress_shared_access_check, StmtTypeContext};
use crate::typing::type_env::TypeEnv;
use crate::typing::type_expr::type_expr;
use crate::typing::type_lookup::{field_type, field_visible, lookup_field_decl, lookup_record_decl};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

/// The base type with aliases expanded, at most sixteen times.
fn normalize_field_base_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Option<&'static str>> {
    let mut current = ty.clone();
    for _ in 0..16 {
        let Some(t) = current.as_deref() else {
            break;
        };
        let (Some(path), Some(args)) = (applied_type_path(t), applied_type_args(t)) else {
            break;
        };
        let expanded = expand_type_alias_apply(ctx, path, args)?;
        if expanded.is_none() {
            break;
        }
        current = strip_perm_and_refine(&expanded);
    }
    Ok(current)
}

/// A field of the class whose body is being typed, reached through `Self`.
fn lookup_class_self_field<'c>(
    ctx: &'c ScopeContext<'_>,
    class_path: &[String],
    field_name: &str,
) -> Result<(TypeRef, &'c ast::ClassFieldDecl), Option<&'static str>> {
    let table = class_field_table(ctx, class_path).map_err(Some)?;
    let field = table.into_iter().find(|candidate| id_eq(&candidate.name, field_name)).ok_or(Some("E-TYP-1904"))?;
    let lowered = lower_type(ctx, &field.r#type)?;
    if lowered.is_none() {
        return Err(None);
    }
    Ok((subst_self_type(&self_var_type(), &lowered, None), field))
}

/// A field of a modal state, for the modal's arguments.
fn lookup_modal_field<'c>(
    ctx: &'c ScopeContext<'_>,
    modal_state: &TypeModalState,
    field_name: &str,
) -> Option<(TypeRef, &'c ast::StateFieldDecl)> {
    let Some(TypeDecl::Modal(modal)) = ctx.sigma.types.get(&path_key_of(&modal_state.path)) else {
        return None;
    };
    let mut modal_subst = TypeSubst::new();
    if let Some(generics) = &modal.generic_params {
        if modal_state.generic_args.len() > generics.params.len() {
            return None;
        }
        modal_subst = build_modal_ref_substitution(&generics.params, &modal_state.generic_args);
    }
    let state_decl = modal.states.iter().find(|state| id_eq(&state.name, &modal_state.state))?;
    let field = state_decl.members.iter().find_map(|member| match member {
        ast::StateMember::StateFieldDecl(field) if id_eq(&field.name, field_name) => Some(field),
        _ => None,
    })?;
    let lowered = lower_type(ctx, &field.r#type).ok()?;
    Some((instantiate_type(&lowered, &modal_subst), field))
}

pub fn type_field_access_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::FieldAccessExpr,
    env: &TypeEnv,
) -> ExprTypeResult {
    let failed = |diag_id: &'static str| ExprTypeResult::failed(Some(diag_id));
    if expr.base.is_none() {
        return ExprTypeResult::default();
    }
    let base_type = type_expr(ctx, &suppress_shared_access_check(type_ctx), &expr.base, env);
    if !base_type.ok {
        return ExprTypeResult::failed(base_type.diag_id);
    }
    // The field has the permission the base is held under.
    let perm = match base_type.r#type.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, .. }) => Some(*perm),
        _ => None,
    };
    let with_perm = |field_type: TypeRef| {
        ExprTypeResult::typed(match perm {
            Some(perm) => make_type_perm(perm, field_type),
            None => field_type,
        })
    };
    let base_span = || expr.base.as_deref().map(|base| base.span.clone());
    let stripped_base = match normalize_field_base_type(ctx, &strip_perm_and_refine(&base_type.r#type)) {
        Ok(normalized) => normalized,
        Err(diag_id) => return ExprTypeResult::failed(diag_id),
    };
    let Some(base) = stripped_base.as_deref() else {
        return failed("E-TYP-1904");
    };
    if matches!(base.node, TypeNode::Union(_)) {
        return ExprTypeResult {
            diag_id: Some("E-TYP-2202"),
            diagnostic_obligation_ids: vec!["Union-DirectAccess-Err", "rule.16.Union-DirectAccess-Err", "diagnostics.DataTypesSupplement"],
            ..Default::default()
        };
    }
    if let Some(path) = applied_type_path(base) {
        if let (true, Some(class_path)) = (is_self_var_path(path), &type_ctx.current_class_path) {
            return match lookup_class_self_field(ctx, class_path, &expr.name) {
                Ok((field_type, decl)) => {
                    emit_deprecated_reference_warning_from_attrs(&decl.attrs, type_ctx, base_span());
                    with_perm(field_type)
                }
                Err(diag_id) => ExprTypeResult::failed(diag_id),
            };
        }
        match ctx.sigma.types.get(&path_key_of(path)) {
            Some(TypeDecl::Enum(_)) => return failed("E-TYP-1904"),
            Some(TypeDecl::Modal(_)) => return failed("E-TYP-2057"),
            _ => {}
        }
        let Some(record) = lookup_record_decl(ctx, path) else {
            return failed("E-TYP-1904");
        };
        let Some(field_decl) = lookup_field_decl(record, &expr.name) else {
            return failed("E-TYP-1904");
        };
        let Some(field_type) = field_type(record, &expr.name, ctx, applied_type_args(base).unwrap_or(&[])) else {
            return failed("E-TYP-1904");
        };
        if !field_visible(ctx, record, &expr.name, path) {
            return failed("FieldAccess-NotVisible");
        }
        emit_deprecated_reference_warning_from_attrs(&field_decl.attrs, type_ctx, base_span());
        return with_perm(field_type);
    }
    match &base.node {
        TypeNode::ModalState(modal) => {
            let Some((field_type, decl)) = lookup_modal_field(ctx, modal, &expr.name) else {
                return failed("E-TYP-2052");
            };
            if !modal_field_visible(ctx, &modal.path) {
                return failed("E-TYP-2064");
            }
            emit_deprecated_reference_warning_from_attrs(&decl.attrs, type_ctx, base_span());
            with_perm(field_type)
        }
        TypeNode::Tuple(_) => failed("TupleIndex-NonConst"),
        _ => failed("E-TYP-1904"),
    }
}

pub fn type_field_access_place(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::FieldAccessExpr,
    env: &TypeEnv,
) -> PlaceTypeResult {
    let typed = type_field_access_expr(ctx, type_ctx, expr, env);
    if !typed.ok {
        return PlaceTypeResult { diag_id: typed.diag_id, ..Default::default() };
    }
    PlaceTypeResult { ok: true, r#type: typed.r#type, ..Default::default() }
}
