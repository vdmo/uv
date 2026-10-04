use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::source_text::{encode_utf8, SourceFile, UnicodeScalar, LF};
use uv_core::span::{span_of, Span};
use uv_core::spec_rule;

use super::token::{token_range, DocComment, DocKind, Token};
use super::{ch, match_prefix, CommentScanResult, ScalarRange};

fn doc_marker(scalars: &[UnicodeScalar], i: usize) -> Option<DocKind> {
    if i + 2 >= scalars.len() {
        return None;
    }
    if match_prefix(scalars, i, "///") {
        return Some(DocKind::LineDoc);
    }
    if match_prefix(scalars, i, "//!") {
        return Some(DocKind::ModuleDoc);
    }
    None
}

fn doc_body(scalars: &[UnicodeScalar], i: usize, j: usize) -> String {
    if j <= i + 3 || i + 3 > scalars.len() {
        return String::new();
    }
    let end = j.min(scalars.len());
    let mut body = &scalars[i + 3..end];
    if body.first() == Some(&0x20) {
        body = &body[1..];
    }
    encode_utf8(body)
}

fn span_of_text(source: &SourceFile, i: usize, j: usize) -> Span {
    span_of(source, source.offsets[i], source.offsets[j])
}

pub fn is_whitespace(c: UnicodeScalar) -> bool {
    c == 0x20 || c == 0x09 || c == 0x0C
}

pub fn is_line_feed(c: UnicodeScalar) -> bool {
    c == LF
}

pub fn scan_line_comment(source: &SourceFile, start: usize) -> CommentScanResult {
    spec_rule!("Scan-Line-Comment");
    let scalars = &source.scalars;
    let mut result = CommentScanResult { ok: true, ..CommentScanResult::default() };
    if !match_prefix(scalars, start, "//") {
        result.ok = false;
        result.next = start;
        result.range = ScalarRange { start, end: start };
        return result;
    }
    let j = scalars[start..].iter().position(|&c| c == LF).map_or(scalars.len(), |p| start + p);
    result.next = j;
    result.range = ScalarRange { start, end: j };
    result
}

pub fn scan_doc_comment(source: &SourceFile, start: usize) -> CommentScanResult {
    spec_rule!("Doc-Comment");
    let mut result = scan_line_comment(source, start);
    let scalars = &source.scalars;
    let Some(kind) = doc_marker(scalars, start) else {
        result.ok = false;
        return result;
    };
    result.doc = Some(DocComment {
        kind,
        text: doc_body(scalars, start, result.next),
        span: span_of_text(source, start, result.next),
    });
    result
}

pub fn scan_block_comment(source: &SourceFile, start: usize) -> CommentScanResult {
    spec_rule!("Block-Start");
    spec_rule!("Block-End");
    spec_rule!("Block-Done");
    spec_rule!("Block-Step");
    let scalars = &source.scalars;
    let n = scalars.len();
    let mut result = CommentScanResult { ok: true, ..CommentScanResult::default() };
    if !match_prefix(scalars, start, "/*") {
        result.ok = false;
        return result;
    }
    let mut depth = 1usize;
    let mut index = start + 2;
    while index < n {
        if match_prefix(scalars, index, "/*") {
            depth += 1;
            index += 2;
            continue;
        }
        if match_prefix(scalars, index, "*/") {
            if depth == 1 {
                let next = index + 2;
                result.next = next;
                result.range = ScalarRange { start, end: next };
                return result;
            }
            depth -= 1;
            index += 2;
            continue;
        }
        index += 1;
    }
    spec_rule!("Block-Comment-Unterminated");
    let span = span_of_text(source, start, start + 2);
    if let Some(diag) = make_diagnostic_by_id("E-SRC-0306", Some(span)) {
        emit(&mut result.diags, diag);
    }
    result.ok = false;
    result.next = n;
    result.range = ScalarRange { start, end: n };
    result
}

pub fn token_in_comment(source: &SourceFile, token: &Token) -> bool {
    let Some((i, j)) = token_range(source, token) else {
        return false;
    };
    let scalars = &source.scalars;
    let mut p = 0usize;
    while p < scalars.len() {
        if scalars[p] == ch('/') {
            if match_prefix(scalars, p, "//") {
                let line = scan_line_comment(source, p);
                if line.ok && line.next > p {
                    if p <= i && j <= line.next {
                        return true;
                    }
                    p = line.next;
                    continue;
                }
            }
            if match_prefix(scalars, p, "/*") {
                let block = scan_block_comment(source, p);
                if block.next > p {
                    if p <= i && j <= block.next {
                        return true;
                    }
                    p = block.next;
                    continue;
                }
            }
        }
        p += 1;
    }
    false
}
