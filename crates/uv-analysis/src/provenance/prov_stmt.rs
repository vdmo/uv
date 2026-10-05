//! The provenance statements give their bindings and check on assignment and return.

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr};

use super::prov_expr::track_expr_provenance;
use crate::context::{IdKey, ScopeContext};
use crate::memory::regions::{region_active_type, ProvenanceKind};
use crate::resolve::collect_toplevel::pat_names;
use crate::resolve::scopes::id_eq;
use crate::typing::check_expr::infer_expr;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::TypeEnv;
use crate::typing::type_expr::{type_expr, type_identifier_expr};
use crate::typing::type_lower::lower_type;
use crate::typing::types::TypeRef;

#[derive(Debug, Clone)]
pub struct ProvStmtTrackResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub kind: ProvenanceKind,
    pub region: Option<IdKey>,
    pub region_target: Option<IdKey>,
    pub fresh_region: bool,
}

/// `Region::new_scoped(...)`, possibly under attributes.
fn is_fresh_region_expr(expr: &ExprPtr) -> bool {
    let is_new_scoped = |path: &[String], name: &str| {
        path.len() == 1 && id_eq(&path[0], "Region") && id_eq(name, "new_scoped")
    };
    match expr.as_deref().map(|e| &e.node) {
        Some(ExprNode::QualifiedApplyExpr(node)) => is_new_scoped(&node.path, &node.name),
        Some(ExprNode::CallExpr(node)) => match node.callee.as_deref().map(|callee| &callee.node) {
            Some(ExprNode::PathExpr(path)) => is_new_scoped(&path.path, &path.name),
            _ => false,
        },
        Some(ExprNode::AttributedExpr(node)) => is_fresh_region_expr(&node.expr),
        _ => false,
    }
}

/// The binding's declared type, or its initializer's type typed on its own.
fn binding_type(ctx: &ScopeContext<'_>, binding: &ast::Binding, env: &TypeEnv) -> Option<TypeRef> {
    let ann_type = ast::binding_annotation_type_opt(binding);
    if ann_type.is_some() {
        return lower_type(ctx, &ann_type).ok();
    }
    let type_ctx = StmtTypeContext::default();
    let type_expr_fn = |expr: &ExprPtr| type_expr(ctx, &type_ctx, expr, env);
    let type_ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let inferred = infer_expr(&binding.init, &type_expr_fn, &type_ident_fn);
    inferred.ok.then_some(inferred.r#type)
}

/// The provenance a `let` or `var` gives its names: a fresh region names itself,
/// anything else takes its initializer's.
pub fn track_binding_provenance(
    ctx: &ScopeContext<'_>,
    binding: &ast::Binding,
    gamma: &TypeEnv,
) -> ProvStmtTrackResult {
    let init_prov = track_expr_provenance(ctx, &binding.init, gamma);
    let mut result = ProvStmtTrackResult {
        ok: init_prov.ok,
        diag_id: init_prov.diag_id,
        span: init_prov.span.clone(),
        kind: ProvenanceKind::Bottom,
        region: None,
        region_target: None,
        fresh_region: false,
    };
    if !result.ok {
        return result;
    }
    let lowered_type = binding_type(ctx, binding, gamma);
    let names = binding.pat.as_deref().map(pat_names).unwrap_or_default();
    let fresh_region = lowered_type.as_ref().is_some_and(region_active_type)
        && is_fresh_region_expr(&binding.init)
        && names.len() == 1;
    if fresh_region {
        result.kind = ProvenanceKind::Region;
        result.region = Some(names[0].clone());
        result.region_target = Some(names[0].clone());
        result.fresh_region = true;
        return result;
    }
    result.kind = init_prov.kind;
    if init_prov.kind == ProvenanceKind::Region {
        result.region = init_prov.region;
        result.region_target = init_prov.region_target;
    }
    result
}
