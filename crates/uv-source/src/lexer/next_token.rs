use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::keywords::{ULTRAVIOLET_OPERATORS, ULTRAVIOLET_PUNCTUATORS};
use uv_core::source_text::SourceFile;
use uv_core::span::span_of;
use uv_core::spec_rule;
use uv_core::unicode::is_ident_start;

use super::ident::scan_ident_token;
use super::literals::{scan_char_literal, scan_float_literal, scan_int_literal, scan_string_literal};
use super::token::TokenKind;
use super::{ch, match_prefix, NextTokenResult};

struct Candidate {
    kind: TokenKind,
    next: usize,
    diags: DiagnosticStream,
}

fn kind_priority(kind: TokenKind) -> i32 {
    match kind {
        TokenKind::IntLiteral
        | TokenKind::FloatLiteral
        | TokenKind::StringLiteral
        | TokenKind::CharLiteral
        | TokenKind::BoolLiteral
        | TokenKind::NullLiteral => 3,
        TokenKind::Identifier | TokenKind::Keyword => 2,
        TokenKind::Operator => 1,
        TokenKind::Punctuator => 0,
        _ => -1,
    }
}

/// Longest-match token selection at scalar index `start`.
pub fn next_token(source: &SourceFile, start: usize) -> NextTokenResult {
    spec_rule!("Max-Munch");
    spec_rule!("Max-Munch-Err");
    let mut result =
        NextTokenResult { ok: false, next: start, kind: TokenKind::Unknown, diags: Vec::new() };
    let scalars = &source.scalars;
    if start >= scalars.len() {
        return result;
    }
    let mut candidates: Vec<Candidate> = Vec::new();
    let first = scalars[start];
    if first == ch('"') || first == ch('\'') {
        let string = scan_string_literal(source, start);
        if string.ok {
            candidates.push(Candidate {
                kind: TokenKind::StringLiteral,
                next: string.next,
                diags: string.diags,
            });
        }
        let character = scan_char_literal(source, start);
        if character.ok {
            candidates.push(Candidate {
                kind: TokenKind::CharLiteral,
                next: character.next,
                diags: character.diags,
            });
        }
    } else if (ch('0')..=ch('9')).contains(&first) {
        let float = scan_float_literal(source, start);
        if float.ok || float.next > start {
            candidates.push(Candidate {
                kind: TokenKind::FloatLiteral,
                next: float.next,
                diags: float.diags,
            });
        }
        let integer = scan_int_literal(source, start);
        if integer.ok || integer.next > start {
            candidates.push(Candidate {
                kind: TokenKind::IntLiteral,
                next: integer.next,
                diags: integer.diags,
            });
        }
    } else if is_ident_start(first) {
        let ident = scan_ident_token(source, start);
        if ident.ok {
            candidates.push(Candidate { kind: ident.kind, next: ident.next, diags: ident.diags });
        }
    } else {
        for op in ULTRAVIOLET_OPERATORS {
            if match_prefix(scalars, start, op) {
                candidates.push(Candidate {
                    kind: TokenKind::Operator,
                    next: start + op.len(),
                    diags: Vec::new(),
                });
            }
        }
        for punc in ULTRAVIOLET_PUNCTUATORS {
            if match_prefix(scalars, start, punc) {
                candidates.push(Candidate {
                    kind: TokenKind::Punctuator,
                    next: start + punc.len(),
                    diags: Vec::new(),
                });
            }
        }
    }
    if candidates.is_empty() {
        let offsets = &source.offsets;
        if start + 1 < offsets.len() {
            let span = span_of(source, offsets[start], offsets[start + 1]);
            if let Some(diag) = make_diagnostic_by_id("E-SRC-0309", Some(span)) {
                emit(&mut result.diags, diag);
            }
        }
        return result;
    }
    let mut best = 0usize;
    for (index, cand) in candidates.iter().enumerate() {
        let current = &candidates[best];
        if cand.next > current.next
            || (cand.next == current.next && kind_priority(cand.kind) > kind_priority(current.kind))
        {
            best = index;
        }
    }
    let best = candidates.swap_remove(best);
    result.ok = true;
    result.kind = best.kind;
    result.next = best.next;
    result.diags = best.diags;
    result
}
