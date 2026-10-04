use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::keywords::is_keyword;
use uv_core::source_text::SourceFile;
use uv_core::span::span_of;
use uv_core::spec_rule;
use uv_core::unicode::{is_ident_continue, is_ident_start, is_non_character};

use super::token::TokenKind;
use super::IdentScanResult;

pub fn scan_ident_token(source: &SourceFile, start: usize) -> IdentScanResult {
    spec_rule!("Lex-Identifier");
    spec_rule!("Lex-Ident-Token");
    let scalars = &source.scalars;
    let mut result = IdentScanResult {
        ok: true,
        next: start,
        kind: TokenKind::Identifier,
        lexeme: String::new(),
        diags: Vec::new(),
    };
    if start >= scalars.len() || !is_ident_start(scalars[start]) {
        result.ok = false;
        return result;
    }
    let mut end = start + 1;
    while end < scalars.len() && is_ident_continue(scalars[end]) {
        end += 1;
    }
    let offsets = &source.offsets;
    result.lexeme = source.text[offsets[start]..offsets[end]].to_string();
    result.next = end;
    if let Some(i) = (start..end).find(|&i| is_non_character(scalars[i])) {
        spec_rule!("Lex-Ident-InvalidUnicode");
        let span = span_of(source, offsets[i], offsets[i + 1]);
        if let Some(diag) = make_diagnostic_by_id("E-SRC-0307", Some(span)) {
            emit(&mut result.diags, diag);
        }
    }
    result.kind = match result.lexeme.as_str() {
        "true" | "false" => TokenKind::BoolLiteral,
        "null" => TokenKind::NullLiteral,
        lexeme if is_keyword(lexeme) => TokenKind::Keyword,
        _ => TokenKind::Identifier,
    };
    result
}
