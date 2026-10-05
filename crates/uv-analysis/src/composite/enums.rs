//! Enum discriminants.

use std::collections::HashSet;

use uv_core::numeric_literals::parse_unsigned_int_literal;
use uv_core::span::Span;
use uv_source::ast;
use uv_source::lexer::token::TokenKind;

#[derive(Debug, Clone)]
pub struct EnumDiscError {
    pub diag_id: &'static str,
    pub span: Span,
}

#[derive(Debug, Clone, Default)]
pub struct EnumDiscs {
    pub discs: Vec<u64>,
    pub max_disc: u64,
}

/// The discriminant of every variant: the written value, or one more than the previous
/// variant's. Values must be distinct, and nothing may follow the largest `u64`.
pub fn enum_discriminants(decl: &ast::EnumDecl) -> Result<EnumDiscs, EnumDiscError> {
    let fail = |diag_id: &'static str, span: &Span| Err(EnumDiscError { diag_id, span: span.clone() });
    let mut out = EnumDiscs::default();
    let mut seen = HashSet::new();
    let mut next: u64 = 0;
    for (index, variant) in decl.variants.iter().enumerate() {
        let is_last = index + 1 == decl.variants.len();
        let disc = match &variant.discriminant_opt {
            Some(tok) => {
                if tok.kind != TokenKind::IntLiteral {
                    return fail("Enum-Disc-NotInt", &tok.span);
                }
                if tok.lexeme.starts_with('-') {
                    return fail("Enum-Disc-Negative", &tok.span);
                }
                match parse_unsigned_int_literal(&tok.lexeme) {
                    Some(disc) if disc != u64::MAX || is_last => disc,
                    _ => return fail("E-TYP-1921", &tok.span),
                }
            }
            None if next == u64::MAX && !is_last => return fail("E-TYP-1921", &decl.span),
            None => next,
        };
        next = disc.wrapping_add(1);
        if !seen.insert(disc) {
            return fail("E-TYP-1923", &variant.span);
        }
        out.discs.push(disc);
        out.max_disc = out.max_disc.max(disc);
    }
    Ok(out)
}
