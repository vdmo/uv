use std::collections::{HashMap, HashSet};

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::process_config::is_debug_enabled;
use uv_core::source_text::SourceFile;
use uv_core::span::{span_of, span_range, Span};
use uv_core::spec_rule;
use uv_core::unicode::{analyze_identifier_security, is_sensitive};

use super::literals::{scan_char_literal, scan_string_literal};
use super::token::{Token, TokenKind};
use super::ws::{scan_block_comment, scan_line_comment};
use super::{ch, match_prefix, LexSecureResult};

fn is_lbrace(tok: &Token) -> bool {
    tok.kind == TokenKind::Punctuator && tok.lexeme == "{"
}

fn is_rbrace(tok: &Token) -> bool {
    tok.kind == TokenKind::Punctuator && tok.lexeme == "}"
}

fn match_brace(tokens: &[Token], open_index: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, tok) in tokens.iter().enumerate().skip(open_index) {
        if is_lbrace(tok) {
            depth += 1;
        } else if is_rbrace(tok) {
            depth -= 1;
            if depth == 0 && i > open_index {
                return Some(i);
            }
        }
    }
    None
}

fn span_from(start: &Token, end: &Token) -> Span {
    let mut span = start.span.clone();
    span.end_offset = end.span.end_offset;
    span.end_line = end.span.end_line;
    span.end_col = end.span.end_col;
    span
}

/// Spans of `unsafe { ... }` blocks, from the opening to the matching closing brace.
pub fn unsafe_spans(tokens: &[Token]) -> Vec<Span> {
    let mut spans = Vec::new();
    for (i, tok) in tokens.iter().enumerate() {
        if tok.kind != TokenKind::Keyword || tok.lexeme != "unsafe" {
            continue;
        }
        let Some(next) = (i + 1..tokens.len()).find(|&k| tokens[k].kind != TokenKind::Newline)
        else {
            continue;
        };
        if !is_lbrace(&tokens[next]) {
            continue;
        }
        if let Some(close) = match_brace(tokens, next) {
            spans.push(span_from(&tokens[next], &tokens[close]));
        }
    }
    spans
}

pub fn lex_sensitive_pos(source: &SourceFile) -> Vec<usize> {
    let scalars = &source.scalars;
    let mut sensitive = Vec::new();
    let mut i = 0usize;
    while i < scalars.len() {
        if match_prefix(scalars, i, "//") {
            let line = scan_line_comment(source, i);
            if line.ok && line.next > i {
                i = line.next;
                continue;
            }
        }
        if match_prefix(scalars, i, "/*") {
            let block = scan_block_comment(source, i);
            if block.next > i {
                i = block.next;
                continue;
            }
        }
        if scalars[i] == ch('"') {
            let lit = scan_string_literal(source, i);
            if lit.ok || lit.next > i {
                i = lit.next;
                continue;
            }
        }
        if scalars[i] == ch('\'') {
            let lit = scan_char_literal(source, i);
            if lit.ok || lit.next > i {
                i = lit.next;
                continue;
            }
        }
        if is_sensitive(scalars[i]) {
            sensitive.push(i);
        }
        i += 1;
    }
    sensitive
}

pub fn lex_secure(source: &SourceFile, tokens: &[Token], sensitive: &[usize]) -> LexSecureResult {
    spec_rule!("LexSecure-Err");
    spec_rule!("LexSecure-Warn");
    let mut result = LexSecureResult { ok: true, diags: Vec::new() };
    if sensitive.is_empty() {
        return result;
    }
    let offsets = &source.offsets;
    let unsafe_blocks = unsafe_spans(tokens);
    let span_at = |p: usize| span_of(source, offsets[p], offsets[p + 1]);
    for &p in sensitive {
        let offset = offsets[p];
        let in_unsafe = unsafe_blocks.iter().any(|sp| {
            let (start, end) = span_range(sp);
            offset >= start && offset < end
        });
        if !in_unsafe {
            if let Some(diag) = make_diagnostic_by_id("E-SRC-0308", Some(span_at(p))) {
                emit(&mut result.diags, diag);
            }
            result.ok = false;
            return result;
        }
    }
    for &p in sensitive {
        if let Some(diag) = make_diagnostic_by_id("W-SRC-0308", Some(span_at(p))) {
            emit(&mut result.diags, diag);
        }
    }
    result
}

pub fn confusable_check(_source: &SourceFile, tokens: &[Token]) -> LexSecureResult {
    let mut result = LexSecureResult { ok: true, diags: Vec::new() };
    let debug = is_debug_enabled("lex") || is_debug_enabled("parse");
    // skeleton -> normalized form of the first identifier seen with that skeleton
    let mut seen_by_skeleton: HashMap<String, String> = HashMap::new();
    // ASCII identifiers are their own NFC form, and repeats of one lexeme can never conflict
    // with themselves, so each distinct lexeme is analyzed once.
    let mut analyzed: HashSet<&str> = HashSet::new();
    for tok in tokens {
        if tok.kind != TokenKind::Identifier {
            continue;
        }
        if debug {
            eprintln!("[uv] unicode: ConfusableCheck token=\"{}\"", tok.lexeme);
        }
        if !debug && !analyzed.insert(tok.lexeme.as_str()) {
            continue;
        }
        let security = analyze_identifier_security(&tok.lexeme);
        if security.mixed_script {
            spec_rule!("MixedScript-Err");
            if let Some(diag) = make_diagnostic_by_id("E-SRC-0311", Some(tok.span.clone())) {
                emit(&mut result.diags, diag);
            }
            result.ok = false;
            return result;
        }
        if let Some(seen) = seen_by_skeleton.get(&security.skeleton) {
            if *seen != security.normalized {
                spec_rule!("Confusable-Err");
                if let Some(diag) = make_diagnostic_by_id("E-SRC-0310", Some(tok.span.clone())) {
                    emit(&mut result.diags, diag);
                }
                result.ok = false;
                return result;
            }
        }
        seen_by_skeleton.entry(security.skeleton).or_insert(security.normalized);
    }
    result
}
