//! What typing an expression yields.

use uv_core::span::Span;

use super::types::TypeRef;

/// The type of an expression, or why it has none. A failure may name no rule: the
/// caller then reports one of its own.
#[derive(Debug, Clone, Default)]
pub struct ExprTypeResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub r#type: TypeRef,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
    pub diagnostic_obligation_ids: Vec<&'static str>,
}

impl ExprTypeResult {
    pub fn typed(ty: TypeRef) -> Self {
        ExprTypeResult { ok: true, r#type: ty, ..Default::default() }
    }

    pub fn failed(diag_id: Option<&'static str>) -> Self {
        ExprTypeResult { diag_id, ..Default::default() }
    }
}
