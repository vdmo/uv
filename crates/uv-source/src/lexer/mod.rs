//! Lexical analysis: tokens, comments, literals, newline policy and identifier security.

mod ident;
pub mod keyword_policy;
mod literals;
mod newlines;
mod next_token;
mod security;
pub mod token;
mod tokenize;
mod ws;

use uv_core::diagnostics::DiagnosticStream;

pub use ident::scan_ident_token;
pub use literals::{scan_char_literal, scan_float_literal, scan_int_literal, scan_string_literal};
pub use newlines::{continues_line, filter_newlines, lex_newlines, required_terminator};
pub use next_token::next_token;
pub use security::{confusable_check, lex_secure, lex_sensitive_pos, unsafe_spans};
pub use token::{DocComment, DocKind, RawToken, Token, TokenKind};
pub use tokenize::{lex_small_step, lexeme_slice_scalars, tokenize, tokenize_with_diagnostics};
pub use ws::{
    is_line_feed, is_whitespace, scan_block_comment, scan_doc_comment, scan_line_comment,
    token_in_comment,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScalarRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LexerOutput {
    pub tokens: Vec<Token>,
    pub docs: Vec<DocComment>,
}

#[derive(Debug, Clone, Default)]
pub struct CommentScanResult {
    pub ok: bool,
    pub next: usize,
    pub range: ScalarRange,
    pub doc: Option<DocComment>,
    pub diags: DiagnosticStream,
}

#[derive(Debug, Clone, Default)]
pub struct LiteralScanResult {
    pub ok: bool,
    pub next: usize,
    pub range: Option<ScalarRange>,
    pub diags: DiagnosticStream,
}

#[derive(Debug, Clone)]
pub struct IdentScanResult {
    pub ok: bool,
    pub next: usize,
    pub kind: TokenKind,
    pub lexeme: String,
    pub diags: DiagnosticStream,
}

#[derive(Debug, Clone)]
pub struct NextTokenResult {
    pub ok: bool,
    pub next: usize,
    pub kind: TokenKind,
    pub diags: DiagnosticStream,
}

#[derive(Debug, Clone)]
pub struct LexSecureResult {
    pub ok: bool,
    pub diags: DiagnosticStream,
}

#[derive(Debug, Clone, Default)]
pub struct LexSmallStepResult {
    pub ok: bool,
    pub error_code: String,
    pub output: LexerOutput,
    pub sensitive: Vec<usize>,
    pub diags: DiagnosticStream,
}

#[derive(Debug, Clone, Default)]
pub struct TokenizeDiagnosticResult {
    pub output: Option<LexerOutput>,
    pub diags: DiagnosticStream,
}

pub(crate) const fn ch(c: char) -> u32 {
    c as u32
}

/// True when `scalars[start..]` begins with the ASCII text `lexeme`.
pub(crate) fn match_prefix(scalars: &[u32], start: usize, lexeme: &str) -> bool {
    let bytes = lexeme.as_bytes();
    start + bytes.len() <= scalars.len()
        && bytes.iter().enumerate().all(|(i, &b)| scalars[start + i] == b as u32)
}

pub(crate) struct TerminatorResult {
    pub index: usize,
    pub closed: bool,
}

/// Finds the closing quote of a literal opened at `start`, stopping at a line feed.
pub(crate) fn find_terminator(scalars: &[u32], start: usize, quote: u32) -> TerminatorResult {
    let mut backslashes = 0usize;
    for (p, &c) in scalars.iter().enumerate().skip(start + 1) {
        if c == uv_core::source_text::LF {
            return TerminatorResult { index: p, closed: false };
        }
        if c == ch('\\') {
            backslashes += 1;
            continue;
        }
        if c == quote && backslashes.is_multiple_of(2) {
            return TerminatorResult { index: p, closed: true };
        }
        backslashes = 0;
    }
    TerminatorResult { index: scalars.len(), closed: false }
}
