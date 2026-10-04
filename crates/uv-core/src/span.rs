use std::sync::Arc;

use crate::source_text::SourceFile;
use crate::spec_rule;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: Arc<str>,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub file: Arc<str>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

pub fn span_range(sp: &Span) -> (usize, usize) {
    (sp.start_offset, sp.end_offset)
}

fn locate_line_col(source: &SourceFile, offset: usize) -> (usize, usize, usize) {
    let o_prime = offset.min(source.byte_len);
    let line_starts = &source.line_starts;
    let mut k = 0usize;
    if !line_starts.is_empty() {
        let upper = line_starts.partition_point(|&start| start <= o_prime);
        if upper != 0 {
            k = upper - 1;
        }
    }
    let line_start = if line_starts.is_empty() { 0 } else { line_starts[k] };
    (o_prime, k + 1, (o_prime - line_start) + 1)
}

pub fn locate(source: &SourceFile, offset: usize) -> SourceLocation {
    spec_rule!("WF-Location");
    let (offset, line, column) = locate_line_col(source, offset);
    SourceLocation { file: source.path.clone(), offset, line, column }
}

pub fn clamp_span(source: &SourceFile, start: usize, end: usize) -> (usize, usize) {
    let s = start.min(source.byte_len);
    let e = end.max(s).min(source.byte_len);
    (s, e)
}

pub fn span_of(source: &SourceFile, start: usize, end: usize) -> Span {
    spec_rule!("Span-Of");
    spec_rule!("WF-Span");
    let (s, e) = clamp_span(source, start, end);
    spec_rule!("WF-Location");
    let (_, start_line, start_col) = locate_line_col(source, s);
    let (_, end_line, end_col) = locate_line_col(source, e);
    Span {
        file: source.path.clone(),
        start_offset: s,
        end_offset: e,
        start_line,
        start_col,
        end_line,
        end_col,
    }
}
