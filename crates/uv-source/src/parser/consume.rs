//! Token matching, list termination and trailing-comma rules.

use uv_core::spec_rule;

use super::state::Parser;
use crate::lexer::{Token, TokenKind};

/// A token kind, with a required lexeme for keywords, operators and punctuators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenMatch {
    pub kind: TokenKind,
    pub lexeme: &'static str,
}

pub const fn match_operator(lexeme: &'static str) -> TokenMatch {
    TokenMatch { kind: TokenKind::Operator, lexeme }
}

pub const fn match_punct(lexeme: &'static str) -> TokenMatch {
    TokenMatch { kind: TokenKind::Punctuator, lexeme }
}

pub fn token_matches(tok: &Token, expected: &TokenMatch) -> bool {
    if tok.kind != expected.kind {
        return false;
    }
    match expected.kind {
        TokenKind::Keyword | TokenKind::Operator | TokenKind::Punctuator => {
            tok.lexeme == expected.lexeme
        }
        _ => true,
    }
}

pub fn token_in_end_set(tok: &Token, end_set: &[TokenMatch]) -> bool {
    end_set.iter().any(|expected| token_matches(tok, expected))
}

/// True when the current token is a comma directly followed by a list terminator.
pub fn trailing_comma(parser: &Parser, end_set: &[TokenMatch]) -> bool {
    if !parser.is_punct(",") {
        return false;
    }
    token_in_end_set(&parser.advance_or_eof().tok(), end_set)
}

/// A trailing comma is allowed only when the terminator is on a later line.
pub fn trailing_comma_allowed(parser: &Parser, end_set: &[TokenMatch]) -> bool {
    if !trailing_comma(parser, end_set) {
        return false;
    }
    parser.tok().span.start_line < parser.advance_or_eof().tok().span.start_line
}

/// Emits `E-SRC-0521` for a trailing comma on the terminator's own line.
pub fn emit_trailing_comma_err(parser: &mut Parser, end_set: &[TokenMatch]) -> bool {
    if !parser.is_punct(",") {
        return false;
    }
    let comma = parser.tok();
    let end_tok = parser.advance_or_eof().tok();
    if !token_in_end_set(&end_tok, end_set) {
        return false;
    }
    if comma.span.start_line < end_tok.span.start_line {
        return false;
    }
    spec_rule!("Trailing-Comma-Err");
    parser.emit("E-SRC-0521", comma.span.clone());
    true
}
