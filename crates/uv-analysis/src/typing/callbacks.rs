//! The results and callbacks the parts of the typer hand each other. Expressions,
//! places, statements and blocks type one another; each part receives the others as
//! functions bound to the current context and environment.

use uv_core::span::Span;
use uv_source::ast::ExprPtr;

use super::expr_result::ExprTypeResult;
use super::types::TypeRef;

/// The type of a place expression, or why it is not one.
#[derive(Debug, Clone, Default)]
pub struct PlaceTypeResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub r#type: TypeRef,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
}

/// Whether an expression has the expected type.
#[derive(Debug, Clone, Default)]
pub struct CheckResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
    pub diagnostic_obligation_ids: Vec<&'static str>,
}

pub type ExprTypeFn<'f> = &'f dyn Fn(&ExprPtr) -> ExprTypeResult;
pub type IdentTypeFn<'f> = &'f dyn Fn(&str) -> ExprTypeResult;
pub type PlaceTypeFn<'f> = &'f dyn Fn(&ExprPtr) -> PlaceTypeResult;
