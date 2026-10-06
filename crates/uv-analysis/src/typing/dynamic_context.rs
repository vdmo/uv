//! Whether a piece of code is in a dynamic context: inside a declaration or expression
//! marked `#dynamic`, where what cannot be proved is checked at run time instead.

use uv_core::span::Span;
use uv_source::ast::AttributeItem;
use uv_source::attributes::{attrs, has_attribute};

/// An enclosing declaration or expression: its attributes and its extent.
pub struct DynamicScopeAncestor<'a> {
    pub attrs: &'a [AttributeItem],
    pub span: &'a Span,
}

fn span_contains(outer: &Span, inner: &Span) -> bool {
    outer.file == inner.file && outer.start_offset <= inner.start_offset && inner.end_offset <= outer.end_offset
}

/// The innermost enclosing scope marked dynamic, if any.
pub fn find_innermost_dynamic<'a>(current_span: &Span, ancestors: &[DynamicScopeAncestor<'a>]) -> Option<&'a Span> {
    let width = |span: &Span| span.end_offset.saturating_sub(span.start_offset);
    let mut innermost: Option<&'a Span> = None;
    for ancestor in ancestors {
        if !has_attribute(ancestor.attrs, attrs::DYNAMIC) || !span_contains(ancestor.span, current_span) {
            continue;
        }
        if innermost.is_none_or(|found| width(ancestor.span) < width(found)) {
            innermost = Some(ancestor.span);
        }
    }
    innermost
}

pub fn compute_dynamic_context(current_span: &Span, ancestors: &[DynamicScopeAncestor<'_>]) -> bool {
    find_innermost_dynamic(current_span, ancestors).is_some()
}
