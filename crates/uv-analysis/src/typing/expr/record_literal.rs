//! Record literals: `Path { field: value, .. }` for a record, `Modal@State { .. }` for a
//! modal state.

use std::collections::{HashMap, HashSet};

use uv_source::ast::{self, ExprNode};

use super::call::emit_deprecated_reference_warning_from_attrs;
use super::small::is_place_expr;
use crate::context::{ScopeContext, TypeDecl};
use crate::generics::generic_params::{required_param_count, total_param_count};
use crate::generics::monomorphize::{build_modal_ref_substitution, build_substitution, instantiate_type, TypeSubst};
use crate::modal::builtin_modal_intrinsics::is_builtin_modal_record_literal_forbidden;
use crate::modal::lookup::lookup_modal_state;
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::outcome::{classify_outcome_intro, OutcomeIntro};
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::TypeEnv;
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, type_expr};
use crate::typing::type_lookup::{field_exists, field_type, field_visible, lookup_record_decl, lookup_type_decl_resolved, record_fields};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::{bitcopy_type, strip_perm};
use crate::typing::types::*;

fn type_path_eq_local(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(l, r)| id_eq(l, r))
}

/// The arguments completed with the defaults of the parameters they leave out.
pub(crate) fn expand_literal_type_args_with_defaults(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    provided_args: &[TypeRef],
) -> Result<Vec<TypeRef>, Option<&'static str>> {
    let mut out_args = provided_args.to_vec();
    if out_args.len() > params.len() {
        return Err(Some("E-TYP-2303"));
    }
    for index in out_args.len()..params.len() {
        if params[index].default_type.is_none() {
            return Err(Some("E-SEM-2533"));
        }
        let mut value = lower_type(ctx, &params[index].default_type)?;
        if index > 0 {
            value = instantiate_type(&value, &build_substitution(&params[..index], &out_args[..index]));
        }
        out_args.push(value);
    }
    Ok(out_args)
}

fn first_field_span(fields: &[ast::FieldInit]) -> Option<uv_core::span::Span> {
    fields.first().and_then(|field| field.value.as_deref()).map(|value| value.span.clone())
}

fn type_modal_state_literal(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    modal: &ast::ModalStateRef,
    fields: &[ast::FieldInit],
    env: &TypeEnv,
    expected_type: Option<&TypeRef>,
) -> ExprTypeResult {
    let failed = |diag_id: &'static str| ExprTypeResult::failed(Some(diag_id));
    if is_builtin_modal_record_literal_forbidden(&modal.path) {
        return failed("E-TYP-2073");
    }
    let Some((TypeDecl::Modal(decl), resolved_modal_path)) = lookup_type_decl_resolved(ctx, &modal.path) else {
        return ExprTypeResult::default();
    };
    emit_deprecated_reference_warning_from_attrs(&decl.attrs, type_ctx, first_field_span(fields));
    let Some(state) = lookup_modal_state(decl, &modal.state) else {
        return ExprTypeResult::default();
    };
    let mut lowered_args = Vec::with_capacity(modal.generic_args.len());
    for arg in &modal.generic_args {
        match lower_type(ctx, arg) {
            Ok(lowered) => lowered_args.push(lowered),
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        }
    }
    // The expected type supplies the modal's arguments when it is this state.
    let mut expected_modal_application = false;
    if let (Some(generics), Some(expected)) = (&decl.generic_params, expected_type) {
        let stripped = strip_perm(expected);
        if let Some(TypeNode::ModalState(expected_modal)) = stripped.as_deref().map(|ty| &ty.node) {
            if expected_modal.state == modal.state && type_path_eq_local(&expected_modal.path, &resolved_modal_path) {
                let expected_args = match expand_literal_type_args_with_defaults(ctx, &generics.params, &expected_modal.generic_args) {
                    Ok(args) => args,
                    Err(diag_id) => return ExprTypeResult::failed(diag_id),
                };
                expected_modal_application = true;
                if !lowered_args.is_empty()
                    && lowered_args.len() == expected_args.len()
                    && !lowered_args.iter().zip(&expected_args).all(|(written, expected)| type_equiv(written, expected))
                {
                    return failed("E-SEM-2533");
                }
                lowered_args = expected_args;
            }
        }
    }
    let mut modal_subst = TypeSubst::new();
    if let Some(generics) = &decl.generic_params {
        let provided = lowered_args.len();
        if provided < required_param_count(&decl.generic_params) || provided > total_param_count(&decl.generic_params) {
            return failed("E-TYP-2303");
        }
        modal_subst = build_modal_ref_substitution(&generics.params, &lowered_args);
    } else if !lowered_args.is_empty() {
        return failed("E-TYP-2303");
    }

    let mut seen = HashSet::new();
    if fields.iter().any(|field_init| !seen.insert(id_key_of(&field_init.name))) {
        return failed("E-TYP-1903");
    }
    let mut payload_fields: HashMap<_, &ast::StateFieldDecl> = HashMap::new();
    let state_fields = || {
        state.members.iter().filter_map(|member| match member {
            ast::StateMember::StateFieldDecl(field) => Some(field),
            _ => None,
        })
    };
    for field in state_fields() {
        payload_fields.entry(id_key_of(&field.name)).or_insert(field);
    }
    if fields.iter().any(|field_init| !payload_fields.contains_key(&id_key_of(&field_init.name))) {
        return failed("E-TYP-1904");
    }
    if state_fields().any(|field| !seen.contains(&id_key_of(&field.name))) {
        return failed("E-TYP-1902");
    }
    for field_init in fields {
        let field = payload_fields[&id_key_of(&field_init.name)];
        let field_type = match lower_type(ctx, &field.r#type) {
            Ok(lowered) => instantiate_type(&lowered, &modal_subst),
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        };
        let check = check_expr_against(ctx, type_ctx, &field_init.value, &field_type, env);
        if !check.ok {
            return ExprTypeResult { diag_id: check.diag_id, diag_detail: check.diag_detail, diag_span: check.diag_span, ..Default::default() };
        }
    }
    ExprTypeResult::typed(match expected_type.filter(|_| expected_modal_application) {
        Some(expected) => expected.clone(),
        None => make_type_modal_state(resolved_modal_path, &modal.state, lowered_args),
    })
}

/// Types a record literal. The result is neither ok nor a diagnostic when the target is
/// not a record or modal state, or a field's type cannot be found.
pub fn type_record_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::RecordExpr,
    env: &TypeEnv,
    expected_type: Option<&TypeRef>,
) -> ExprTypeResult {
    let failed = |diag_id: &'static str| ExprTypeResult::failed(Some(diag_id));
    let type_path = match &expr.target {
        ast::RecordExprTarget::ModalStateRef(modal) => {
            return type_modal_state_literal(ctx, type_ctx, modal, &expr.fields, env, expected_type);
        }
        ast::RecordExprTarget::Path(path) => path,
    };
    let Some(record) = lookup_record_decl(ctx, type_path) else {
        return ExprTypeResult::default();
    };
    emit_deprecated_reference_warning_from_attrs(&record.attrs, type_ctx, first_field_span(&expr.fields));

    // The expected type supplies the record's arguments when it is this record.
    let mut record_generic_args: Vec<TypeRef> = Vec::new();
    let mut expected_record_application = false;
    if let (Some(generics), Some(expected)) = (&record.generic_params, expected_type) {
        let stripped = strip_perm(expected);
        if let Some(t) = stripped.as_deref() {
            if let (Some(expected_path), Some(expected_args)) = (applied_type_path(t), applied_type_args(t)) {
                if type_path_eq_local(expected_path, type_path) {
                    match expand_literal_type_args_with_defaults(ctx, &generics.params, expected_args) {
                        Ok(args) => {
                            record_generic_args = args;
                            expected_record_application = true;
                        }
                        Err(diag_id) => return ExprTypeResult::failed(diag_id),
                    }
                }
            }
        }
    }

    let mut seen = HashSet::new();
    if expr.fields.iter().any(|field_init| !seen.insert(id_key_of(&field_init.name))) {
        return failed("E-TYP-1903");
    }
    for field_init in &expr.fields {
        if !field_exists(record, &field_init.name) {
            return failed("E-TYP-1904");
        }
        if !field_visible(ctx, record, &field_init.name, type_path) {
            return failed("E-TYP-1905");
        }
    }
    if record_fields(record).iter().any(|field| !seen.contains(&id_key_of(&field.name))) {
        return failed("E-TYP-1902");
    }
    for field_init in &expr.fields {
        let Some(field_type) = field_type(record, &field_init.name, ctx, &record_generic_args) else {
            return ExprTypeResult::default();
        };
        // A value that cannot be copied must be moved into the field.
        let is_move = matches!(field_init.value.as_deref().map(|value| &value.node), Some(ExprNode::MoveExpr(_)));
        if !bitcopy_type(ctx, &field_type) && is_place_expr(&field_init.value) && !is_move {
            return failed("E-TYP-1907");
        }
        let check = check_expr_against(ctx, type_ctx, &field_init.value, &field_type, env);
        if !check.ok {
            let typed_field = type_expr(ctx, type_ctx, &field_init.value, env);
            if typed_field.ok {
                match classify_outcome_intro(ctx, &typed_field.r#type, &field_type) {
                    OutcomeIntro::Ambiguous => {
                        return ExprTypeResult {
                            diag_id: Some("E-TYP-2261"),
                            diag_span: field_init.value.as_deref().map(|value| value.span.clone()),
                            ..Default::default()
                        };
                    }
                    OutcomeIntro::Value | OutcomeIntro::Error => continue,
                    OutcomeIntro::None => {}
                }
            }
            return ExprTypeResult { diag_id: check.diag_id, diag_detail: check.diag_detail, diag_span: check.diag_span, ..Default::default() };
        }
    }
    ExprTypeResult::typed(match expected_type.filter(|_| expected_record_application) {
        Some(expected) => expected.clone(),
        None if !record_generic_args.is_empty() => make_type_path_with(type_path.clone(), record_generic_args),
        None => make_type_path(type_path.clone()),
    })
}
