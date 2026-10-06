//! `let` and `var`: the initializer is checked against the annotation or its type
//! inferred, the pattern is typed against that type, and its names are introduced with
//! what later checks need to know about them.

use uv_source::ast::{self, ExprNode, ExprPtr};
use uv_source::attributes::{
    attrs, get_attribute_value, has_attribute, validate_attributes, AttributeTarget,
};

use super::block::StmtTypeResult;
use super::stmt_common::{analyze_closure_capture_info, expr_needs_key_access};
use crate::context::{IdKey, ScopeContext};
use crate::provenance::prov_stmt::{track_binding_provenance, ProvStmtTrackResult};
use crate::typing::callbacks::{IdentTypeFn, PlaceTypeFn};
use crate::typing::check_expr::infer_expr;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::outcome::{classify_outcome_intro, OutcomeIntro};
use crate::typing::solve::{apply_substitution, solve};
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::type_env::{
    apply_binding_provenance_seed, bind_of, collect_pat_names, distinct_names, intro_all,
    type_pattern, ClosureCaptureInfo, ParallelContextKind, TypeEnv,
};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{
    check_expr_against, emit_stale_binding_reference_warning, type_expr,
};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::{bitcopy_type, perm_of_type, strip_perm};
use crate::typing::types::{type_to_string, Permission, TypeRef};

fn pattern_name(pat: &ast::PatternPtr) -> String {
    match pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::IdentifierPattern(ident)) => ident.name.clone(),
        Some(ast::PatternNode::TypedPattern(typed)) if typed.name != "_" => typed.name.clone(),
        _ => String::new(),
    }
}

/// A `transmute` the initializer evaluates to directly; it is typed once more so that
/// its warnings are reported.
fn contains_transmute_warning_candidate(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|e| &e.node) {
        Some(ExprNode::TransmuteExpr(_)) => true,
        Some(ExprNode::AttributedExpr(node)) => contains_transmute_warning_candidate(&node.expr),
        Some(ExprNode::UnsafeBlockExpr(node)) => node
            .block
            .as_deref()
            .is_some_and(|block| contains_transmute_warning_candidate(&block.tail_opt)),
        Some(ExprNode::BlockExpr(node)) => node
            .block
            .as_deref()
            .is_some_and(|block| contains_transmute_warning_candidate(&block.tail_opt)),
        _ => false,
    }
}

/// A `const` binding of a unique place, or a `unique` binding of a moved place of the
/// same type.
fn is_unique_move_init_compatible(
    annotated: &TypeRef,
    init: &ExprPtr,
    type_place_fn: PlaceTypeFn<'_>,
) -> bool {
    let Some(init_expr) = init.as_deref() else {
        return false;
    };
    match perm_of_type(annotated) {
        Permission::Const => {
            let place = type_place_fn(init);
            place.ok
                && perm_of_type(&place.r#type) == Permission::Unique
                && type_equiv(&strip_perm(&place.r#type), &strip_perm(annotated))
        }
        Permission::Unique => {
            let ExprNode::MoveExpr(move_expr) = &init_expr.node else {
                return false;
            };
            if move_expr.place.is_none() {
                return false;
            }
            let place = type_place_fn(&move_expr.place);
            place.ok && type_equiv(&strip_perm(&place.r#type), &strip_perm(annotated))
        }
        Permission::Shared => false,
    }
}

/// A `shared` binding of a copyable value made fresh for it.
fn is_shared_materialized_init_compatible(
    ctx: &ScopeContext<'_>,
    read_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    annotated: &TypeRef,
    init: &ExprPtr,
) -> bool {
    if init.is_none() || perm_of_type(annotated) != Permission::Shared {
        return false;
    }
    let base = strip_perm(annotated);
    if base.is_none() || !bitcopy_type(ctx, &base) {
        return false;
    }
    check_expr_against(ctx, read_ctx, init, &base, env).ok
}

pub(crate) fn normalize_deprecated_message(attrs_list: &[ast::AttributeItem]) -> Option<String> {
    let message = get_attribute_value(attrs_list, attrs::DEPRECATED, "")?;
    let bytes = message.as_bytes();
    let quoted = bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''));
    if quoted {
        return Some(message[1..message.len() - 1].to_string());
    }
    Some(message)
}

/// The execution domain a binding holds when its initializer names one.
fn binding_parallel_context_kind(expr: &ExprPtr, env: &TypeEnv) -> Option<ParallelContextKind> {
    let mut current = expr.as_deref();
    while let Some(e) = current {
        match &e.node {
            ExprNode::AttributedExpr(attributed) => {
                current = attributed.expr.as_deref();
                continue;
            }
            ExprNode::MethodCallExpr(method) => match method.name.as_str() {
                "cpu" => return Some(ParallelContextKind::Cpu),
                "gpu" => return Some(ParallelContextKind::Gpu),
                "inline" => return Some(ParallelContextKind::Inline),
                _ => {}
            },
            ExprNode::IdentifierExpr(ident) => {
                if let Some(binding) = bind_of(env, &ident.name) {
                    return binding.parallel_context_kind;
                }
            }
            _ => {}
        }
        return None;
    }
    None
}

struct BindingMetadata {
    closure_info: Option<ClosureCaptureInfo>,
    provenance: Option<ProvStmtTrackResult>,
    parallel_context_kind: Option<ParallelContextKind>,
    derived_from_shared: bool,
    stale_ok: bool,
    deprecated: bool,
    deprecated_message: Option<String>,
}

fn apply_binding_metadata(env: &mut TypeEnv, names: &[IdKey], meta: &BindingMetadata) {
    let Some(scope) = env.scopes.last_mut() else {
        return;
    };
    for name in names {
        let Some(binding) = scope.get_mut(name) else {
            continue;
        };
        binding.deprecated = meta.deprecated;
        binding.deprecated_message = meta.deprecated_message.clone();
        binding.derived_from_shared = meta.derived_from_shared;
        binding.stale_ok = meta.stale_ok;
        binding.stale_after_release = false;
        if let Some(kind) = meta.parallel_context_kind.filter(|_| names.len() == 1) {
            binding.parallel_context_kind = Some(kind);
        }
        if let Some(info) = meta.closure_info.filter(|_| names.len() == 1) {
            binding.closure_capture_info = Some(info);
        }
        if let Some(provenance) = &meta.provenance {
            apply_binding_provenance_seed(binding, provenance.kind, provenance.region.as_deref());
        }
    }
}

fn fail(diag_id: Option<&'static str>) -> StmtTypeResult {
    StmtTypeResult {
        diag_id,
        ..Default::default()
    }
}

fn fail_with(diag_id: Option<&'static str>, diag_detail: String) -> StmtTypeResult {
    StmtTypeResult {
        diag_id,
        diag_detail,
        ..Default::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn type_binding_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    binding: &ast::Binding,
    mutability: ast::Mutability,
    env: &TypeEnv,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let is_var = mutability == ast::Mutability::Var;
    let validation = validate_attributes(&binding.attrs, AttributeTarget::Binding);
    if !validation.ok {
        return fail_with(
            validation.diag_id.or(Some("Attr-Target-Err")),
            validation.message,
        );
    }
    let deprecated = has_attribute(&binding.attrs, attrs::DEPRECATED);
    let deprecated_message = normalize_deprecated_message(&binding.attrs);
    let stale_ok = has_attribute(&binding.attrs, attrs::STALE_OK);
    let ann_type = ast::binding_annotation_type_opt(binding);
    let read_ctx = with_shared_access_mode(type_ctx, ast::KeyMode::Read);
    let read_type_expr = |inner: &ExprPtr| type_expr(ctx, &read_ctx, inner, env);
    let read_type_ident = |name: &str| match bind_of(env, name) {
        Some(found) => {
            emit_stale_binding_reference_warning(found, &read_ctx, None);
            ExprTypeResult::typed(found.r#type.clone())
        }
        None => type_ident_fn(name),
    };

    let binding_type = if ann_type.is_some() {
        let ann = match lower_type(ctx, &ann_type) {
            Ok(ann) => ann,
            Err(diag_id) => return fail(diag_id),
        };
        let check = check_expr_against(ctx, &read_ctx, &binding.init, &ann, env);
        let unique_move_ok = is_unique_move_init_compatible(&ann, &binding.init, type_place_fn);
        let shared_materialized_ok =
            is_shared_materialized_init_compatible(ctx, &read_ctx, env, &ann, &binding.init);
        // A bare value or error introduces into an `Outcome` binding when only one fits.
        let mut outcome_intro_ok = false;
        if !is_var && !check.ok && !unique_move_ok && !shared_materialized_ok {
            let inferred = infer_expr(&binding.init, &read_type_expr, &read_type_ident);
            if inferred.ok {
                match classify_outcome_intro(ctx, &inferred.r#type, &ann) {
                    OutcomeIntro::Ambiguous => {
                        return StmtTypeResult {
                            diag_id: Some("E-TYP-2261"),
                            diag_span: binding.init.as_deref().map(|init| init.span.clone()),
                            ..Default::default()
                        };
                    }
                    OutcomeIntro::Value | OutcomeIntro::Error => outcome_intro_ok = true,
                    OutcomeIntro::None => {}
                }
            }
        }
        if !check.ok && !unique_move_ok && !shared_materialized_ok && !outcome_intro_ok {
            if check.diag_id.is_none() || check.diag_id == Some("E-SEM-2526") {
                let diag_detail = if check.diag_detail.is_empty() {
                    format!(
                        "initializer failed checking against annotation {}",
                        type_to_string(&ann)
                    )
                } else {
                    check.diag_detail
                };
                let diag_span = check
                    .diag_span
                    .or_else(|| binding.init.as_deref().map(|init| init.span.clone()));
                let diagnostic_obligation_ids = if is_var {
                    vec![
                        "req.18.VarStmtTypingMirrorsLet",
                        "diag.18.BindingStatements",
                    ]
                } else {
                    Vec::new()
                };
                return StmtTypeResult {
                    diag_id: Some("E-MOD-2402"),
                    diag_detail,
                    diag_span,
                    diagnostic_obligation_ids,
                    ..Default::default()
                };
            }
            if is_var {
                return fail(check.diag_id);
            }
            return StmtTypeResult {
                diag_id: check.diag_id,
                diag_detail: check.diag_detail,
                diag_span: check.diag_span,
                ..Default::default()
            };
        }
        if contains_transmute_warning_candidate(&binding.init) {
            let _ = read_type_expr(&binding.init);
        }
        ann
    } else {
        let infer_err = if is_var {
            "T-VarStmt-Infer-Err"
        } else {
            "T-LetStmt-Infer-Err"
        };
        let inferred = infer_expr(&binding.init, &read_type_expr, &read_type_ident);
        if !inferred.ok {
            if inferred.diag_id.is_some() {
                return fail_with(inferred.diag_id, inferred.diag_detail);
            }
            let name = pattern_name(&binding.pat);
            let detail = if name.is_empty() {
                String::new()
            } else {
                format!("binding '{name}'")
            };
            return fail_with(Some(infer_err), detail);
        }
        // Inference records no constraints, so the solution is empty.
        let subst = match solve(ctx, &[]) {
            Ok(subst) => subst,
            Err(diag_id) => return fail_with(diag_id.or(Some(infer_err)), inferred.diag_detail),
        };
        let inferred_type = apply_substitution(&inferred.r#type, &subst);
        if inferred_type.is_none() {
            return fail_with(Some(infer_err), inferred.diag_detail);
        }
        inferred_type
    };

    let bindings = match type_pattern(ctx, &binding.pat, &binding_type) {
        Ok(bindings) => bindings,
        Err(diag_id) => return fail(diag_id),
    };
    let mut names = Vec::new();
    if let Some(pat) = binding.pat.as_deref() {
        collect_pat_names(pat, &mut names);
    }
    if !distinct_names(&names) {
        return fail(Some("Pat-Dup-Err"));
    }
    let closure_info = analyze_closure_capture_info(&binding.init, env, &binding_type);
    let intro_env = match &type_ctx.env_ref {
        Some(cell) => cell.borrow().clone(),
        None => env.clone(),
    };
    let mut out_env = match intro_all(&intro_env, &bindings, mutability, false) {
        Ok(out_env) => out_env,
        Err(diag_id) => return fail(diag_id.or(Some("Pat-Dup-Err"))),
    };
    let tracked = track_binding_provenance(ctx, binding, env);
    let meta = BindingMetadata {
        closure_info,
        provenance: tracked.ok.then_some(tracked),
        parallel_context_kind: binding_parallel_context_kind(&binding.init, env),
        derived_from_shared: expr_needs_key_access(ctx, &read_ctx, &binding.init, env),
        stale_ok,
        deprecated,
        deprecated_message,
    };
    apply_binding_metadata(&mut out_env, &names, &meta);
    StmtTypeResult::typed(out_env)
}
