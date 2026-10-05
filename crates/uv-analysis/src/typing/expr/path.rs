//! Names as values and places: bindings of the environment first, then the statics and
//! procedures of modules.

use uv_source::ast::{self, ASTItem};

use crate::composite::function_types::{lookup_module_static, value_path_type};
use crate::context::ScopeContext;
use crate::resolve::scopes::id_eq;
use crate::resolve::scopes_lookup::resolve_value_name;
use crate::typing::callbacks::PlaceTypeResult;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::type_env::{bind_of, TypeEnv};

/// The environment of a compile-time body binds the compile-time capabilities.
fn is_comptime_typing_env(env: &TypeEnv) -> bool {
    ["diagnostics", "introspect", "emitter", "files", "target"]
        .iter()
        .any(|name| bind_of(env, name).is_some())
}

fn module_has_comptime_procedure(module: &ast::ASTModule, name: &str) -> bool {
    module
        .comptime_procedures
        .iter()
        .any(|proc| id_eq(&proc.name, name))
        || module.items.iter().any(
            |item| matches!(item, ASTItem::ComptimeProcedureDecl(proc) if id_eq(&proc.name, name)),
        )
}

/// A compile-time procedure named outside compile-time code.
fn is_runtime_comptime_procedure_ref(
    ctx: &ScopeContext<'_>,
    origin: &[String],
    name: &str,
    env: &TypeEnv,
) -> bool {
    if is_comptime_typing_env(env) {
        return false;
    }
    ctx.sigma
        .mods
        .iter()
        .find(|module| module.path == origin)
        .is_some_and(|module| module_has_comptime_procedure(module, name))
}

fn unresolved(name: &str) -> ExprTypeResult {
    ExprTypeResult {
        diag_id: Some("ResolveExpr-Ident-Err"),
        diag_detail: format!("identifier '{name}'"),
        ..Default::default()
    }
}

/// `path::name` as a value.
pub fn type_path_expr(
    ctx: &ScopeContext<'_>,
    expr: &ast::PathExpr,
    env: &TypeEnv,
) -> ExprTypeResult {
    let origin: &[String] = if expr.path.is_empty() {
        &ctx.current_module
    } else {
        &expr.path
    };
    if is_runtime_comptime_procedure_ref(ctx, origin, &expr.name, env) {
        return ExprTypeResult::failed(Some("E-CTE-0034"));
    }
    let value_type = value_path_type(ctx, &expr.path, &expr.name);
    if !value_type.ok {
        return ExprTypeResult::failed(value_type.diag_id);
    }
    if value_type.r#type.is_some() {
        return ExprTypeResult::typed(value_type.r#type);
    }
    unresolved(&expr.name)
}

/// A name as a value: a binding, else a static or procedure of the current module.
pub fn type_identifier_expr(ctx: &ScopeContext<'_>, env: &TypeEnv, name: &str) -> ExprTypeResult {
    if let Some(binding) = bind_of(env, name) {
        return ExprTypeResult::typed(binding.r#type.clone());
    }
    match resolve_value_name(ctx, name) {
        Some(ent) if ent.origin_opt.is_some() => {
            let origin = ent.origin_opt.as_deref().unwrap_or_default();
            let resolved_name = ent.target_opt.as_deref().unwrap_or(name);
            if is_runtime_comptime_procedure_ref(ctx, origin, resolved_name, env) {
                return ExprTypeResult::failed(Some("E-CTE-0034"));
            }
        }
        _ => {
            if is_runtime_comptime_procedure_ref(ctx, &ctx.current_module, name, env) {
                return ExprTypeResult::failed(Some("E-CTE-0034"));
            }
        }
    }
    let value_type = value_path_type(ctx, &ctx.current_module, name);
    if !value_type.ok {
        return ExprTypeResult::failed(value_type.diag_id);
    }
    if value_type.r#type.is_some() {
        return ExprTypeResult::typed(value_type.r#type);
    }
    unresolved(name)
}

/// A name as a place: a binding, else a static.
pub fn type_identifier_place(ctx: &ScopeContext<'_>, env: &TypeEnv, name: &str) -> PlaceTypeResult {
    if let Some(binding) = bind_of(env, name) {
        return PlaceTypeResult {
            ok: true,
            r#type: binding.r#type.clone(),
            ..Default::default()
        };
    }
    let failed = |diag_id| PlaceTypeResult {
        diag_id,
        ..Default::default()
    };
    if let Some(ent) = resolve_value_name(ctx, name) {
        if let Some(origin) = &ent.origin_opt {
            let resolved_name = ent.target_opt.as_deref().unwrap_or(name);
            let resolved_static = lookup_module_static(ctx, origin, resolved_name);
            if !resolved_static.ok {
                return failed(resolved_static.diag_id);
            }
            if resolved_static.r#type.is_some() {
                return PlaceTypeResult {
                    ok: true,
                    r#type: resolved_static.r#type,
                    ..Default::default()
                };
            }
        }
    }
    let static_lookup = lookup_module_static(ctx, &ctx.current_module, name);
    if !static_lookup.ok {
        return failed(static_lookup.diag_id);
    }
    if static_lookup.r#type.is_some() {
        return PlaceTypeResult {
            ok: true,
            r#type: static_lookup.r#type,
            ..Default::default()
        };
    }
    PlaceTypeResult {
        diag_id: Some("ResolveExpr-Ident-Err"),
        diag_detail: format!("identifier '{name}'"),
        ..Default::default()
    }
}
