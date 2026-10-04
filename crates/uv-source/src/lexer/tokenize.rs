use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::process_config::is_debug_enabled;
use uv_core::source_text::{SourceFile, UnicodeScalar, LF};
use uv_core::span::{span_of, Span};
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;
use uv_core::unicode::is_sensitive;

use super::literals::scan_int_literal;
use super::next_token::next_token;
use super::security::{confusable_check, lex_secure, lex_sensitive_pos};
use super::token::{attach_spans, no_unknown_ok, token_range, DocKind, RawToken, Token, TokenKind};
use super::ws::{is_line_feed, is_whitespace, scan_block_comment, scan_doc_comment, scan_line_comment};
use super::{
    ch, find_terminator, match_prefix, LexSmallStepResult, LexerOutput, ScalarRange,
    TokenizeDiagnosticResult,
};

pub fn lexeme_slice_scalars(scalars: &[UnicodeScalar], i: usize, j: usize) -> Vec<UnicodeScalar> {
    if j < i || j > scalars.len() {
        return Vec::new();
    }
    scalars[i..j].to_vec()
}

fn debug_lex_fail(source: &SourceFile, index: usize) {
    if !is_debug_enabled("lex") {
        return;
    }
    let scalars = &source.scalars;
    let offsets = &source.offsets;
    let n = scalars.len();
    let byte_index = if index < offsets.len() { offsets[index] } else { 0 };
    let cp = if index < n { scalars[index] } else { 0 };
    eprintln!("[uv] lex: Max-Munch-Err at scalar={index} byte={byte_index} codepoint=U+{cp:04X}");
    let lo = index.saturating_sub(16);
    let hi = n.min(index + 17);
    let mut context = String::new();
    for &c in &scalars[lo..hi] {
        if c == LF {
            context.push_str("\\n");
        } else if (0x20..=0x7E).contains(&c) {
            context.push(c as u8 as char);
        } else {
            context.push('.');
        }
    }
    eprintln!("[uv] lex: context=\"{context}\"");
    let window: Vec<String> = (lo..hi)
        .map(|i| format!("U+{:04X}{}", scalars[i], if i == index { "*" } else { "" }))
        .collect();
    eprintln!("[uv] lex: window=[{}]", window.join(" "));
}

fn span_of_text(source: &SourceFile, i: usize, j: usize) -> Span {
    span_of(source, source.offsets[i], source.offsets[j])
}

fn lexeme_slice(source: &SourceFile, i: usize, j: usize) -> String {
    let start = source.offsets[i];
    let end = source.offsets[j];
    if end < start || end > source.byte_len {
        return String::new();
    }
    source.text[start..end].to_string()
}

fn append_diags(out: &mut DiagnosticStream, add: DiagnosticStream) {
    for diag in add {
        emit(out, diag);
    }
}

fn append_sensitive_in_span(scalars: &[UnicodeScalar], i: usize, j: usize, sens: &mut Vec<usize>) {
    sens.extend((i..j).filter(|&p| is_sensitive(scalars[p])));
}

fn prev_significant_token_is_dot(tokens: &[Token]) -> bool {
    tokens
        .iter()
        .rev()
        .find(|tok| tok.kind != TokenKind::Newline)
        .is_some_and(|tok| tok.kind == TokenKind::Punctuator && tok.lexeme == ".")
}

fn to_raw_tokens(tokens: Vec<Token>) -> Vec<RawToken> {
    tokens
        .into_iter()
        .map(|tok| RawToken {
            kind: tok.kind,
            lexeme: tok.lexeme,
            start_offset: tok.span.start_offset,
            end_offset: tok.span.end_offset,
        })
        .collect()
}

struct StreamHash(u64);

impl StreamHash {
    fn byte(&mut self, byte: u8) {
        self.0 ^= byte as u64;
        self.0 = self.0.wrapping_mul(1099511628211);
    }

    fn size(&mut self, value: usize) {
        for byte in (value as u64).to_le_bytes() {
            self.byte(byte);
        }
    }

    fn text(&mut self, text: &str) {
        self.size(text.len());
        for byte in text.bytes() {
            self.byte(byte);
        }
    }

    fn span(&mut self, span: &Span) {
        self.size(span.start_offset);
        self.size(span.end_offset);
        self.size(span.start_line);
        self.size(span.start_col);
        self.size(span.end_line);
        self.size(span.end_col);
    }
}

fn lexer_output_hash(output: &LexerOutput) -> u64 {
    let mut hash = StreamHash(1469598103934665603);
    hash.size(output.tokens.len());
    for token in &output.tokens {
        hash.text(token.kind.name());
        hash.text(&token.lexeme);
        hash.span(&token.span);
    }
    hash.size(output.docs.len());
    for doc in &output.docs {
        hash.text(doc.kind.name());
        hash.text(&doc.text);
        hash.span(&doc.span);
    }
    hash.0
}

fn lexer_output_span(output: &LexerOutput) -> Option<&Span> {
    output.tokens.first().map(|tok| &tok.span).or_else(|| output.docs.first().map(|doc| &doc.span))
}

fn newline_token_count(tokens: &[Token]) -> usize {
    tokens.iter().filter(|token| token.kind == TokenKind::Newline).count()
}

fn doc_comment_count(output: &LexerOutput, kind: DocKind) -> usize {
    output.docs.iter().filter(|doc| doc.kind == kind).count()
}

fn line_comment_end(scalars: &[UnicodeScalar], start: usize) -> usize {
    scalars[start..].iter().position(|&c| c == LF).map_or(scalars.len(), |p| start + p)
}

fn block_comment_end(scalars: &[UnicodeScalar], start: usize) -> usize {
    let mut depth = 1usize;
    let mut index = start + 2;
    while index < scalars.len() {
        if match_prefix(scalars, index, "/*") {
            depth += 1;
            index += 2;
            continue;
        }
        if match_prefix(scalars, index, "*/") {
            depth -= 1;
            index += 2;
            if depth == 0 {
                return index;
            }
            continue;
        }
        index += 1;
    }
    scalars.len()
}

fn quoted_span_end(scalars: &[UnicodeScalar], start: usize, quote: u32) -> usize {
    let terminator = find_terminator(scalars, start, quote);
    if terminator.closed {
        return terminator.index + 1;
    }
    (start + 1).max(terminator.index)
}

fn comment_ranges(scalars: &[UnicodeScalar]) -> Vec<ScalarRange> {
    let mut ranges = Vec::new();
    let mut index = 0usize;
    while index < scalars.len() {
        if scalars[index] == ch('"') {
            index = quoted_span_end(scalars, index, ch('"'));
            continue;
        }
        if scalars[index] == ch('\'') {
            index = quoted_span_end(scalars, index, ch('\''));
            continue;
        }
        if match_prefix(scalars, index, "//") {
            let end = line_comment_end(scalars, index);
            ranges.push(ScalarRange { start: index, end });
            index = end;
            continue;
        }
        if match_prefix(scalars, index, "/*") {
            let end = block_comment_end(scalars, index);
            ranges.push(ScalarRange { start: index, end });
            index = end;
            continue;
        }
        index += 1;
    }
    ranges
}

fn record_lexer_output_evidence(source: &SourceFile, output: &LexerOutput) {
    if !Conformance::enabled() {
        return;
    }
    let payload = format!(
        "tokens={};docs={};line_docs={};module_docs={};newlines={};source_bytes={};stream_hash={}",
        output.tokens.len(),
        output.docs.len(),
        doc_comment_count(output, DocKind::LineDoc),
        doc_comment_count(output, DocKind::ModuleDoc),
        newline_token_count(&output.tokens),
        source.byte_len,
        lexer_output_hash(output)
    );
    Conformance::record_at("def.LexerOutput", lexer_output_span(output), &payload);
}

fn record_tokenize_output_constraints_evidence(source: &SourceFile, output: &LexerOutput) {
    if !Conformance::enabled() {
        return;
    }
    let comments = comment_ranges(&source.scalars);
    let in_comment = |index: usize| comments.iter().any(|r| r.start <= index && index < r.end);
    let mut token_ranges = Vec::with_capacity(output.tokens.len());
    for token in &output.tokens {
        match token_range(source, token) {
            Some(range) => token_ranges.push((token.kind, range)),
            None => return,
        }
    }
    let newline_starts: std::collections::HashSet<usize> = token_ranges
        .iter()
        .filter(|(kind, (i, j))| *kind == TokenKind::Newline && *j == *i + 1)
        .map(|(_, (i, _))| *i)
        .collect();
    let mut required_newlines = 0usize;
    for (index, &c) in source.scalars.iter().enumerate() {
        if c != LF || in_comment(index) {
            continue;
        }
        required_newlines += 1;
        if !newline_starts.contains(&index) {
            return;
        }
    }
    let overlaps_comment = token_ranges
        .iter()
        .any(|(_, (start, end))| comments.iter().any(|r| *start < r.end && r.start < *end));
    if overlaps_comment {
        return;
    }
    let payload = format!(
        "lex_newline=true;lex_no_comments=true;required_newlines={};newline_tokens={};comment_ranges={};tokens={};docs={};stream_hash={}",
        required_newlines,
        newline_token_count(&output.tokens),
        comments.len(),
        output.tokens.len(),
        output.docs.len(),
        lexer_output_hash(output)
    );
    Conformance::record_at("def.TokenizeOutputConstraints", lexer_output_span(output), &payload);
}

pub fn lex_small_step(source: &SourceFile) -> LexSmallStepResult {
    spec_rule!("Lex-Start");
    let mut result = LexSmallStepResult { ok: true, ..LexSmallStepResult::default() };
    let scalars = &source.scalars;
    let mut i = 0usize;
    let mut tokens: Vec<Token> = Vec::new();
    let mut docs = Vec::new();
    let mut sensitive: Vec<usize> = Vec::new();
    loop {
        if i >= scalars.len() {
            spec_rule!("Lex-End");
            result.ok = true;
            break;
        }
        let c = scalars[i];
        if is_whitespace(c) {
            spec_rule!("Lex-Whitespace");
            i += 1;
            continue;
        }
        if is_line_feed(c) {
            spec_rule!("Lex-Newline");
            tokens.push(Token {
                kind: TokenKind::Newline,
                lexeme: lexeme_slice(source, i, i + 1),
                span: span_of_text(source, i, i + 1),
            });
            i += 1;
            continue;
        }
        if c == ch('/') {
            if match_prefix(scalars, i, "///") || match_prefix(scalars, i, "//!") {
                spec_rule!("Lex-Doc-Comment");
                let doc = scan_doc_comment(source, i);
                append_diags(&mut result.diags, doc.diags);
                if doc.ok {
                    if let Some(comment) = doc.doc {
                        docs.push(comment);
                        i = doc.next;
                        continue;
                    }
                }
            }
            if match_prefix(scalars, i, "//") {
                spec_rule!("Lex-Line-Comment");
                let line = scan_line_comment(source, i);
                append_diags(&mut result.diags, line.diags);
                if line.ok {
                    i = line.next;
                    continue;
                }
            }
            if match_prefix(scalars, i, "/*") {
                spec_rule!("Lex-Block-Comment");
                let block = scan_block_comment(source, i);
                append_diags(&mut result.diags, block.diags);
                if !block.ok {
                    result.ok = false;
                    result.error_code = "E-SRC-0306".to_string();
                    break;
                }
                i = block.next;
                continue;
            }
        }
        if c == ch('"') {
            let term = find_terminator(scalars, i, ch('"'));
            if !term.closed {
                spec_rule!("Lex-String-Unterminated-Recover");
                if let Some(diag) =
                    make_diagnostic_by_id("E-SRC-0301", Some(span_of_text(source, i, i + 1)))
                {
                    emit(&mut result.diags, diag);
                }
                i = term.index;
                continue;
            }
        }
        if c == ch('\'') {
            let term = find_terminator(scalars, i, ch('\''));
            if !term.closed {
                spec_rule!("Lex-Char-Unterminated-Recover");
                if let Some(diag) =
                    make_diagnostic_by_id("E-SRC-0303", Some(span_of_text(source, i, i + 1)))
                {
                    emit(&mut result.diags, diag);
                }
                i = term.index;
                continue;
            }
        }
        if is_sensitive(c) {
            spec_rule!("Lex-Sensitive");
            sensitive.push(i);
            i += 1;
            continue;
        }
        let next = next_token(source, i);
        if !next.ok {
            debug_lex_fail(source, i);
            spec_rule!("Lex-Token-Err");
            result.error_code = next
                .diags
                .first()
                .map_or_else(|| "E-SRC-0309".to_string(), |diag| diag.code.clone());
            append_diags(&mut result.diags, next.diags);
            result.ok = false;
            break;
        }
        spec_rule!("Lex-Token");
        append_diags(&mut result.diags, next.diags);
        if next.kind == TokenKind::FloatLiteral && prev_significant_token_is_dot(&tokens) {
            let int_scan = scan_int_literal(source, i);
            if (int_scan.ok || int_scan.next > i) && int_scan.next < next.next {
                append_diags(&mut result.diags, int_scan.diags);
                tokens.push(Token {
                    kind: TokenKind::IntLiteral,
                    lexeme: lexeme_slice(source, i, int_scan.next),
                    span: span_of_text(source, i, int_scan.next),
                });
                append_sensitive_in_span(scalars, i, int_scan.next, &mut sensitive);
                i = int_scan.next;
                continue;
            }
        }
        tokens.push(Token {
            kind: next.kind,
            lexeme: lexeme_slice(source, i, next.next),
            span: span_of_text(source, i, next.next),
        });
        if next.kind != TokenKind::StringLiteral && next.kind != TokenKind::CharLiteral {
            append_sensitive_in_span(scalars, i, next.next, &mut sensitive);
        }
        i = next.next;
    }
    result.output.tokens = tokens;
    result.output.docs = docs;
    if result.ok {
        record_lexer_output_evidence(source, &result.output);
    }
    result.sensitive = sensitive;
    result
}

pub fn tokenize_with_diagnostics(source: &SourceFile) -> TokenizeDiagnosticResult {
    let mut result = TokenizeDiagnosticResult::default();
    let mut lexed = lex_small_step(source);
    result.diags = std::mem::take(&mut lexed.diags);
    if !lexed.ok {
        spec_rule!("Tokenize-Err");
        return result;
    }
    let secure = lex_secure(source, &lexed.output.tokens, &lex_sensitive_pos(source));
    append_diags(&mut result.diags, secure.diags);
    if !secure.ok {
        spec_rule!("Tokenize-Secure-Err");
        return result;
    }
    if is_debug_enabled("lex") || is_debug_enabled("parse") {
        eprintln!(
            "[uv] unicode: token-count={} running ConfusableCheck",
            lexed.output.tokens.len()
        );
    }
    let confusable = confusable_check(source, &lexed.output.tokens);
    append_diags(&mut result.diags, confusable.diags);
    if !confusable.ok {
        spec_rule!("Tokenize-Secure-Err");
        return result;
    }
    let attached_tokens =
        attach_spans(source, to_raw_tokens(std::mem::take(&mut lexed.output.tokens)));
    if !no_unknown_ok(&attached_tokens) {
        if let Some(unknown) = attached_tokens.iter().find(|tok| tok.kind == TokenKind::Unknown) {
            if let Some(diag) = make_diagnostic_by_id("E-SRC-0309", Some(unknown.span.clone())) {
                emit(&mut result.diags, diag);
            }
        }
        spec_rule!("Tokenize-Err");
        return result;
    }
    lexed.output.tokens = attached_tokens;
    spec_rule!("Tokenize-Ok");
    record_tokenize_output_constraints_evidence(source, &lexed.output);
    result.output = Some(lexed.output);
    result
}

pub fn tokenize(source: &SourceFile) -> Option<LexerOutput> {
    tokenize_with_diagnostics(source).output
}
