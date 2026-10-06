//! Text helpers of the server: words, lines, and the indentation formatter.

use uv_source::lexer::token::{Token, TokenKind};
use uv_tooling::line_index::{LineIndex, LineRange};

pub fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 0x80
}

pub struct WordRange {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

fn is_utf8_continuation(byte: u8) -> bool {
    byte & 0xC0 == 0x80
}

/// The identifier at or just before the offset.
pub fn word_at(text: &str, offset: usize) -> Option<WordRange> {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut pos = offset.min(bytes.len() - 1);
    while pos > 0 && is_utf8_continuation(bytes[pos]) {
        pos -= 1;
    }
    if !is_ident_byte(bytes[pos]) && pos > 0 && is_ident_byte(bytes[pos - 1]) {
        pos -= 1;
    }
    if !is_ident_byte(bytes[pos]) {
        return None;
    }
    let mut start = pos;
    while start > 0 && is_ident_byte(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = pos + 1;
    while end < bytes.len() && is_ident_byte(bytes[end]) {
        end += 1;
    }
    Some(WordRange { start, end, text: String::from_utf8_lossy(&bytes[start..end]).into_owned() })
}

#[derive(Debug, Clone)]
pub struct TextLine {
    pub content: String,
    pub line_ending: &'static str,
    pub start_offset: usize,
    pub end_offset: usize,
}

pub fn split_text_lines(text: &str) -> Vec<TextLine> {
    let bytes = text.as_bytes();
    let mut lines = Vec::new();
    let mut line_start = 0;
    let mut offset = 0;
    while offset < bytes.len() {
        if bytes[offset] == b'\r' && offset + 1 < bytes.len() && bytes[offset + 1] == b'\n' {
            lines.push(TextLine { content: text[line_start..offset].to_string(), line_ending: "\r\n", start_offset: line_start, end_offset: offset + 2 });
            offset += 2;
            line_start = offset;
        } else if bytes[offset] == b'\n' {
            lines.push(TextLine { content: text[line_start..offset].to_string(), line_ending: "\n", start_offset: line_start, end_offset: offset + 1 });
            offset += 1;
            line_start = offset;
        } else {
            offset += 1;
        }
    }
    if line_start < bytes.len() || text.is_empty() {
        lines.push(TextLine { content: text[line_start..].to_string(), line_ending: "", start_offset: line_start, end_offset: bytes.len() });
    }
    lines
}

pub fn trim_outer_whitespace(line: &str) -> &str {
    line.trim_matches([' ', '\t'])
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LineLayoutInfo {
    pub brace_delta: i32,
    pub starts_with_closing_brace: bool,
    pub has_significant_token: bool,
}

pub fn build_line_layout_info(tokens: &[Token], index: &LineIndex, line_count: usize) -> Vec<LineLayoutInfo> {
    let mut lines = vec![LineLayoutInfo::default(); line_count];
    for token in tokens {
        if matches!(token.kind, TokenKind::Newline | TokenKind::Eof) {
            continue;
        }
        let line = index.position_at(token.span.start_offset).line;
        let Some(info) = lines.get_mut(line) else {
            continue;
        };
        let first = !info.has_significant_token;
        info.has_significant_token = true;
        if token.kind != TokenKind::Punctuator {
            continue;
        }
        if token.lexeme == "{" {
            info.brace_delta += 1;
        } else if token.lexeme == "}" {
            info.brace_delta -= 1;
            if first {
                info.starts_with_closing_brace = true;
            }
        }
    }
    lines
}

pub fn join_text_lines(lines: &[TextLine]) -> String {
    lines.iter().map(|line| format!("{}{}", line.content, line.line_ending)).collect()
}

pub struct FormatOptions {
    pub indent_unit: String,
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions { indent_unit: "    ".to_string() }
    }
}

pub fn format_options_from_params(params: Option<&serde_json::Value>) -> FormatOptions {
    let Some(options) = params.and_then(|params| params.get("options")).and_then(|options| options.as_object()) else {
        return FormatOptions::default();
    };
    let insert_spaces = options.get("insertSpaces").and_then(|value| value.as_bool()).unwrap_or(true);
    let mut tab_size = options.get("tabSize").and_then(|value| value.as_i64()).unwrap_or(4);
    if tab_size <= 0 {
        tab_size = 4;
    }
    FormatOptions { indent_unit: if insert_spaces { " ".repeat(tab_size as usize) } else { "\t".to_string() } }
}

pub fn format_ultraviolet_text(tokens: &[Token], text: &str, options: &FormatOptions) -> String {
    let mut lines = split_text_lines(text);
    if lines.is_empty() {
        return text.to_string();
    }
    let index = LineIndex::new(text);
    let layout = build_line_layout_info(tokens, &index, lines.len());
    let mut brace_depth: i32 = 0;
    for (line_index, line) in lines.iter_mut().enumerate() {
        let trimmed = trim_outer_whitespace(&line.content).to_string();
        if trimmed.is_empty() {
            line.content.clear();
        } else {
            let mut indent_depth = brace_depth;
            if layout[line_index].starts_with_closing_brace && indent_depth > 0 {
                indent_depth -= 1;
            }
            line.content = format!("{}{}", options.indent_unit.repeat(indent_depth as usize), trimmed);
        }
        brace_depth = (brace_depth + layout[line_index].brace_delta).max(0);
    }
    join_text_lines(&lines)
}

/// The first and last line a range formats; the last is the one before when the range ends at a line start.
pub fn format_line_selection(line_count: usize, range: &LineRange) -> Option<(usize, usize)> {
    if line_count == 0 {
        return None;
    }
    let start_line = range.start.line.min(line_count - 1);
    let mut end_line = range.end.line;
    if range.end.character == 0 && end_line > 0 {
        end_line -= 1;
    }
    end_line = end_line.min(line_count - 1);
    if end_line < start_line {
        end_line = start_line;
    }
    Some((start_line, end_line))
}

pub fn line_slice_text(lines: &[TextLine], start_line: usize, end_line: usize) -> String {
    (start_line..=end_line).map(|line| format!("{}{}", lines[line].content, lines[line].line_ending)).collect()
}

pub fn expected_indent_for_line(layout: &[LineLayoutInfo], line: usize, indent_unit: &str) -> String {
    let mut brace_depth: i32 = 0;
    for info in layout.iter().take(line) {
        brace_depth = (brace_depth + info.brace_delta).max(0);
    }
    if layout.get(line).is_some_and(|info| info.starts_with_closing_brace) && brace_depth > 0 {
        brace_depth -= 1;
    }
    indent_unit.repeat(brace_depth as usize)
}
