//! Assignment: the place must be writable and the value must have its type.

use uv_source::ast::{self, ExprNode, ExprPtr};

use super::block::StmtTypeResult;
use super::stmt_common::expr_needs_key_access;
use crate::composite::function_types::lookup_module_static;
use crate::context::ScopeContext;
use crate::provenance::prov_expr::track_expr_provenance;
use crate::resolve::scopes::id_key_of;
use crate::typing::callbacks::{ExprTypeFn, IdentTypeFn, PlaceTypeFn, PlaceTypeResult};
use crate::typing::check_expr::infer_expr;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::outcome::{classify_outcome_intro, OutcomeIntro};
use crate::typing::pending::pending;
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::type_env::{apply_binding_provenance_seed, bind_of, mut_of, stable_binding_type, TypeBinding, TypeEnv};
use crate::typing::type_expr::{check_expr_against, type_expr, type_place};
use crate::typing::types::*;

/// The binding a place is reached from.
pub fn place_root_name(expr: &ExprPtr) -> Option<&str> {
    match &expr.as_deref()?.node {
        ExprNode::AttributedExpr(node) => place_root_name(&node.expr),
        ExprNode::IdentifierExpr(node) => Some(&node.name),
        ExprNode::FieldAccessExpr(node) => place_root_name(&node.base),
        ExprNode::TupleAccessExpr(node) => place_root_name(&node.base),
        ExprNode::IndexAccessExpr(node) => place_root_name(&node.base),
        ExprNode::DerefExpr(node) => place_root_name(&node.value),
        _ => None,
    }
}

fn binding_mut<'e>(env: &'e mut TypeEnv, name: &str) -> Option<&'e mut TypeBinding> {
    let key = id_key_of(name);
    env.scopes.iter_mut().rev().find_map(|scope| scope.get_mut(&key))
}

fn is_root_identifier_place(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::AttributedExpr(node)) => is_root_identifier_place(&node.expr),
        Some(ExprNode::IdentifierExpr(_)) => true,
        _ => false,
    }
}

/// Whether the write goes through a pointer, and so not to the root binding itself.
pub fn place_writes_through_deref(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::AttributedExpr(node)) => place_writes_through_deref(&node.expr),
        Some(ExprNode::DerefExpr(_)) => true,
        Some(ExprNode::FieldAccessExpr(node)) => place_writes_through_deref(&node.base),
        Some(ExprNode::TupleAccessExpr(node)) => place_writes_through_deref(&node.base),
        Some(ExprNode::IndexAccessExpr(node)) => place_writes_through_deref(&node.base),
        _ => false,
    }
}

/// A place as assignment sees it: a name or a dereference, or a field or element of one.
pub fn is_place_expr_local(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IdentifierExpr(_) | ExprNode::DerefExpr(_)) => true,
        Some(ExprNode::AttributedExpr(node)) => is_place_expr_local(&node.expr),
        Some(ExprNode::FieldAccessExpr(node)) => is_place_expr_local(&node.base),
        Some(ExprNode::TupleAccessExpr(node)) => is_place_expr_local(&node.base),
        Some(ExprNode::IndexAccessExpr(node)) => is_place_expr_local(&node.base),
        _ => false,
    }
}

/// Whether the root of a place may be written: a local by its binding, a module-level
/// static by its declaration. `Ok(None)` when the name is neither.
pub fn lookup_root_mutability(
    ctx: &ScopeContext<'_>,
    env: &TypeEnv,
    name: &str,
) -> Result<Option<ast::Mutability>, Option<&'static str>> {
    if let Some(local_mut) = mut_of(env, name) {
        return Ok(Some(local_mut));
    }
    let static_lookup = lookup_module_static(ctx, &ctx.current_module, name);
    if !static_lookup.ok {
        return Err(static_lookup.diag_id);
    }
    Ok(static_lookup.r#type.is_some().then_some(if static_lookup.is_var { ast::Mutability::Var } else { ast::Mutability::Let }))
}

fn expr_kind_name(expr: &ExprPtr) -> String {
    match expr.as_deref().map(|expr| &expr.node) {
        None => "null".to_string(),
        Some(ExprNode::IdentifierExpr(node)) => format!("IdentifierExpr({})", node.name),
        Some(ExprNode::DerefExpr(_)) => "DerefExpr".to_string(),
        Some(ExprNode::FieldAccessExpr(_)) => "FieldAccessExpr".to_string(),
        Some(ExprNode::TupleAccessExpr(_)) => "TupleAccessExpr".to_string(),
        Some(ExprNode::IndexAccessExpr(_)) => "IndexAccessExpr".to_string(),
        Some(ExprNode::AttributedExpr(_)) => "AttributedExpr".to_string(),
        Some(_) => "Expr".to_string(),
    }
}

/// The place typed in the environment as it stands; a bound name has its binding's
/// type without further checks.
pub fn type_place_with_current_env(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_place_fn: PlaceTypeFn<'_>,
    expr: &ExprPtr,
) -> PlaceTypeResult {
    let Some(e) = expr.as_deref() else {
        return PlaceTypeResult::default();
    };
    if let ExprNode::IdentifierExpr(ident) = &e.node {
        if let Some(binding) = bind_of(env, &ident.name) {
            return PlaceTypeResult { ok: true, r#type: binding.r#type.clone(), ..Default::default() };
        }
    }
    let via_env = type_place(ctx, type_ctx, expr, env);
    if via_env.ok || via_env.diag_id.is_some() {
        return via_env;
    }
    type_place_fn(expr)
}

pub fn type_expr_with_current_env(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    expr: &ExprPtr,
) -> ExprTypeResult {
    if expr.is_none() {
        return ExprTypeResult::default();
    }
    let via_env = type_expr(ctx, type_ctx, expr, env);
    if via_env.ok || via_env.diag_id.is_some() {
        return via_env;
    }
    type_expr_fn(expr)
}

/// The declared type of the binding a place names, rather than a narrowed one.
pub fn stable_place_type_for_assign(place: &ExprPtr, env: &TypeEnv, fallback: &TypeRef) -> TypeRef {
    match place.as_deref().map(|place| &place.node) {
        Some(ExprNode::AttributedExpr(node)) => stable_place_type_for_assign(&node.expr, env, fallback),
        Some(ExprNode::IdentifierExpr(node)) => match bind_of(env, &node.name) {
            Some(binding) => stable_binding_type(binding).clone(),
            None => fallback.clone(),
        },
        _ => fallback.clone(),
    }
}

fn perm_parts(ty: &TypeRef) -> Option<(Permission, &TypeRef)> {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, base }) => Some((*perm, base)),
        _ => None,
    }
}

pub fn type_assign_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::AssignStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let failed = |diag_id: Option<&'static str>| StmtTypeResult { diag_id, ..Default::default() };
    let read_ctx = with_shared_access_mode(type_ctx, ast::KeyMode::Read);
    let write_ctx = with_shared_access_mode(type_ctx, ast::KeyMode::Write);
    if !is_place_expr_local(&node.place) {
        return failed(Some("E-SEM-3131"));
    }
    let place_type = type_place_with_current_env(ctx, &write_ctx, env, type_place_fn, &node.place);
    if !place_type.ok {
        let mut detail = place_type.diag_detail;
        if detail.is_empty() {
            detail = format!("assign-place-kind={}", expr_kind_name(&node.place));
            if let Some(ExprNode::DerefExpr(deref)) = node.place.as_deref().map(|place| &place.node) {
                if let Some(ExprNode::IdentifierExpr(ident)) = deref.value.as_deref().map(|value| &value.node) {
                    let binding = if bind_of(env, &ident.name).is_some() { "present" } else { "absent" };
                    detail += &format!("; deref-ident={}; deref-ident-binding={binding}", ident.name);
                }
            }
        }
        return StmtTypeResult { diag_id: place_type.diag_id, diag_detail: detail, diag_span: place_type.diag_span, ..Default::default() };
    }
    let place_perm = perm_parts(&place_type.r#type);
    match place_perm.map(|(perm, _)| perm) {
        Some(Permission::Const) => return failed(Some("E-TYP-1601")),
        Some(Permission::Shared) => {
            // Writing shared data depends on the keys held and on whether the value
            // reads the place it writes; that analysis is not ported.
            pending("SharedAssign");
            return failed(None);
        }
        _ => {}
    }

    let writes_through_deref = place_writes_through_deref(&node.place);
    let root = place_root_name(&node.place);
    if let (false, Some(root)) = (writes_through_deref, root) {
        match lookup_root_mutability(ctx, env, root) {
            Err(diag_id) => return failed(diag_id),
            Ok(Some(ast::Mutability::Let)) => return failed(Some("E-MOD-2401")),
            Ok(_) => {}
        }
    }

    let mut assign_target_type = match place_perm {
        Some((_, base)) => base.clone(),
        None => stable_place_type_for_assign(&node.place, env, &place_type.r#type),
    };
    if let Some((_, base)) = perm_parts(&assign_target_type) {
        assign_target_type = base.clone();
    }

    let check = check_expr_against(ctx, &read_ctx, &node.value, &assign_target_type, env);
    if !check.ok {
        let value_expr = |inner: &ExprPtr| type_expr_with_current_env(ctx, &read_ctx, env, type_expr_fn, inner);
        let value_ident = |name: &str| match bind_of(env, name) {
            Some(binding) => ExprTypeResult::typed(binding.r#type.clone()),
            None => type_ident_fn(name),
        };
        let inferred_intro = infer_expr(&node.value, &value_expr, &value_ident);
        let mut outcome_intro_ok = false;
        if inferred_intro.ok {
            match classify_outcome_intro(ctx, &inferred_intro.r#type, &assign_target_type) {
                OutcomeIntro::Ambiguous => return failed(Some("E-TYP-2261")),
                OutcomeIntro::Value | OutcomeIntro::Error => outcome_intro_ok = true,
                OutcomeIntro::None => {}
            }
        }
        if !outcome_intro_ok {
            return failed(match check.diag_id {
                None | Some("E-SEM-2526") => Some("E-SEM-3133"),
                other => other,
            });
        }
    }

    let mut out_env = env.clone();
    if let (false, Some(root)) = (writes_through_deref, root) {
        let value_prov = track_expr_provenance(ctx, &node.value, env);
        if value_prov.ok {
            if let Some(binding) = binding_mut(&mut out_env, root) {
                apply_binding_provenance_seed(binding, value_prov.kind, value_prov.region.as_deref());
            }
        }
        if is_root_identifier_place(&node.place) {
            let derived_from_shared = expr_needs_key_access(ctx, &read_ctx, &node.value, env);
            if let Some(binding) = binding_mut(&mut out_env, root) {
                binding.derived_from_shared = derived_from_shared;
                binding.stale_after_release = false;
            }
        }
    }
    StmtTypeResult::typed(out_env)
}
