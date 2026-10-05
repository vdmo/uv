//! Typing expressions: the dispatch over expression forms.

use uv_source::ast::{self, ExprNode, ExprPtr};

use super::callbacks::PlaceTypeResult;
use super::expr_result::ExprTypeResult;
use super::literals::type_literal_expr;
use super::pending::pending;
use super::stmt_context::StmtTypeContext;
use super::type_env::{bind_of, TypeEnv};
use crate::context::ScopeContext;

/// The type of a name. Only names bound in the environment so far; names of
/// module-level declarations are pending.
pub fn type_identifier_expr(env: &TypeEnv, name: &str) -> ExprTypeResult {
    match bind_of(env, name) {
        Some(binding) => ExprTypeResult::typed(binding.r#type.clone()),
        None => {
            pending("IdentifierExpr");
            ExprTypeResult::default()
        }
    }
}

pub fn type_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ExprPtr, env: &TypeEnv) -> ExprTypeResult {
    let Some(e) = expr.as_deref() else {
        return ExprTypeResult::default();
    };
    let _ = (ctx, type_ctx, env);
    match &e.node {
        ExprNode::LiteralExpr(node) => type_literal_expr(node),
        ExprNode::PtrNullExpr(_) => ExprTypeResult::failed(Some("PtrNull-Infer-Err")),
        ExprNode::QualifiedNameExpr(_) | ExprNode::QualifiedApplyExpr(_) => {
            ExprTypeResult::failed(Some("ResolveExpr-Ident-Err"))
        }
        _ => {
            pending(ast::expr_kind(e));
            ExprTypeResult::default()
        }
    }
}

pub fn type_place(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ExprPtr, env: &TypeEnv) -> PlaceTypeResult {
    let _ = (ctx, type_ctx, expr, env);
    pending("TypePlace");
    PlaceTypeResult::default()
}
