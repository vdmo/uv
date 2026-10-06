//! Semantic tokens: each token of a file classified, from its syntax alone or, when an
//! analysis is at hand, from the symbol it names.

use std::path::Path;

use serde_json::{json, Value};
use uv_analysis::language_service::{LanguageReference, LanguageSymbolInfo, LanguageSymbolKind};
use uv_core::source_load::load_source;
use uv_source::lexer::token::{Token, TokenKind};
use uv_source::lexer::tokenize_with_diagnostics;
use uv_tooling::analysis::AnalysisSnapshot;
use uv_tooling::line_index::{LineIndex, LinePosition, LineRange};

pub const SEMANTIC_TOKEN_TYPES: [&str; 17] = [
    "namespace", "type", "class", "enum", "struct", "function", "method", "variable", "parameter", "property", "enumMember", "string", "number", "keyword", "operator", "comment", "macro",
];
pub const SEMANTIC_TOKEN_MODIFIERS: [&str; 3] = ["declaration", "readonly", "documentation"];

fn type_index(name: &str) -> i64 {
    SEMANTIC_TOKEN_TYPES.iter().position(|candidate| *candidate == name).or_else(|| SEMANTIC_TOKEN_TYPES.iter().position(|candidate| *candidate == "variable")).unwrap_or(0) as i64
}

fn modifier_mask(name: &str) -> i64 {
    SEMANTIC_TOKEN_MODIFIERS.iter().position(|candidate| *candidate == name).map_or(0, |at| 1 << at)
}

struct SemanticToken {
    line: usize,
    start: usize,
    length: usize,
    token_type: i64,
    modifiers: i64,
}

struct TokenClass {
    kind: &'static str,
    modifiers: i64,
}

fn is_builtin_type(lexeme: &str) -> bool {
    const BUILTIN: [&str; 40] = [
        "bool", "bytes", "char", "f32", "f64", "i8", "i16", "i32", "i64", "isize", "string", "u8", "u16", "u32", "u64", "usize", "Context", "System", "IO", "Network", "HeapAllocator",
        "ExecutionDomain", "Reactor", "Region", "RegionOptions", "CancelToken", "File", "DirIter", "DirEntry", "FileKind", "IoError", "Async", "Future", "Sequence", "Stream", "Pipe", "Exchange",
        "Spawned", "Tracked", "Outcome",
    ];
    BUILTIN.contains(&lexeme)
}

fn is_readonly_name(lexeme: &str) -> bool {
    let mut has_letter = false;
    for ch in lexeme.chars() {
        if ch.is_ascii_lowercase() {
            return false;
        }
        if ch.is_ascii_uppercase() {
            has_letter = true;
        }
    }
    has_letter && lexeme.len() > 1
}

fn classify_identifier_syntax_only(lexeme: &str) -> TokenClass {
    if is_builtin_type(lexeme) {
        return TokenClass { kind: "type", modifiers: 0 };
    }
    if lexeme.chars().next().is_some_and(|ch| ch.is_ascii_uppercase()) {
        if is_readonly_name(lexeme) {
            return TokenClass { kind: "variable", modifiers: modifier_mask("readonly") };
        }
        return TokenClass { kind: "type", modifiers: 0 };
    }
    TokenClass { kind: "variable", modifiers: 0 }
}

fn type_for_symbol_kind(kind: LanguageSymbolKind) -> &'static str {
    use LanguageSymbolKind as K;
    match kind {
        K::Module => "namespace",
        K::Record => "struct",
        K::Modal | K::TypeAlias => "type",
        K::Class => "class",
        K::Enum => "enum",
        K::EnumMember => "enumMember",
        K::Function => "function",
        K::Method => "method",
        K::Field | K::State => "property",
        K::Parameter => "parameter",
        K::Constant | K::Variable => "variable",
    }
}

fn symbol_for_identifier<'a>(path: &Path, token: &Token, snapshot: &'a AnalysisSnapshot) -> (Option<&'a LanguageSymbolInfo>, Option<&'a LanguageReference>) {
    let file = path.to_string_lossy();
    let reference = snapshot.language_service.reference_at(&file, token.span.start_offset);
    if let Some(symbol) = reference.and_then(|reference| snapshot.language_service.symbol_by_id(&reference.symbol_id)) {
        return (Some(symbol), reference);
    }
    let symbol = snapshot.language_service.symbol_at(&file, token.span.start_offset).filter(|symbol| symbol.name == token.lexeme);
    (symbol, reference)
}

fn classify_identifier(path: &Path, token: &Token, snapshot: Option<&AnalysisSnapshot>) -> TokenClass {
    if is_builtin_type(&token.lexeme) {
        return TokenClass { kind: "type", modifiers: 0 };
    }
    let Some(snapshot) = snapshot else {
        return classify_identifier_syntax_only(&token.lexeme);
    };
    let (symbol, reference) = symbol_for_identifier(path, token, snapshot);
    let Some(symbol) = symbol else {
        return TokenClass { kind: "variable", modifiers: 0 };
    };
    let mut modifiers = 0;
    if reference.is_some_and(|reference| reference.is_declaration) && symbol.name == token.lexeme {
        modifiers |= modifier_mask("declaration");
    }
    if symbol.kind == LanguageSymbolKind::Constant {
        modifiers |= modifier_mask("readonly");
    }
    TokenClass { kind: type_for_symbol_kind(symbol.kind), modifiers }
}

fn classify_token(path: &Path, token: &Token, snapshot: Option<&AnalysisSnapshot>) -> TokenClass {
    match token.kind {
        TokenKind::Keyword | TokenKind::BoolLiteral | TokenKind::NullLiteral => TokenClass { kind: "keyword", modifiers: 0 },
        TokenKind::Identifier => classify_identifier(path, token, snapshot),
        TokenKind::IntLiteral | TokenKind::FloatLiteral => TokenClass { kind: "number", modifiers: 0 },
        TokenKind::StringLiteral | TokenKind::CharLiteral => TokenClass { kind: "string", modifiers: 0 },
        TokenKind::Operator | TokenKind::Punctuator => TokenClass { kind: "operator", modifiers: 0 },
        _ => TokenClass { kind: "", modifiers: 0 },
    }
}

/// One entry per line a span covers.
fn add_token(out: &mut Vec<SemanticToken>, index: &LineIndex, span: &uv_core::span::Span, token_type: i64, modifiers: i64) {
    let text = index.text_bytes();
    let start_offset = span.start_offset.min(text.len());
    let end_offset = span.end_offset.min(text.len());
    let mut segment_start = start_offset;
    while segment_start < end_offset {
        let newline = text[segment_start..].iter().position(|byte| *byte == b'\n').map(|at| at + segment_start);
        let ends_on_this_line = newline.is_none_or(|newline| newline >= end_offset);
        let mut segment_end = if ends_on_this_line { end_offset } else { newline.unwrap_or(end_offset) };
        if segment_end > segment_start && text[segment_end - 1] == b'\r' {
            segment_end -= 1;
        }
        if segment_end > segment_start {
            let start = index.position_at(segment_start);
            let end = index.position_at(segment_end);
            if start.line == end.line && end.character > start.character {
                out.push(SemanticToken { line: start.line, start: start.character, length: end.character - start.character, token_type, modifiers });
            }
        }
        if ends_on_this_line {
            break;
        }
        segment_start = newline.unwrap_or(end_offset) + 1;
    }
}

fn position_less(lhs: LinePosition, rhs: LinePosition) -> bool {
    (lhs.line, lhs.character) < (rhs.line, rhs.character)
}

fn intersects_range(token: &SemanticToken, range: &Option<LineRange>) -> bool {
    let Some(range) = range else {
        return true;
    };
    position_less(range.start, LinePosition { line: token.line, character: token.start + token.length }) && position_less(LinePosition { line: token.line, character: token.start }, range.end)
}

pub fn semantic_tokens_for_range(path: &Path, text_utf8: &str, snapshot: Option<&AnalysisSnapshot>, requested_range: Option<LineRange>) -> Value {
    let loaded = load_source(&path.to_string_lossy(), text_utf8.as_bytes());
    let Some(source) = loaded.source else {
        return json!({ "data": [] });
    };
    let index = LineIndex::new(&source.text);
    let tokenized = tokenize_with_diagnostics(&source);
    let range_bytes = requested_range.map(|range| (index.byte_offset_at(range.start), index.byte_offset_at(range.end)));
    let outside = |span: &uv_core::span::Span| range_bytes.is_some_and(|(first, second)| span.end_offset <= first || span.start_offset >= second);
    let mut tokens: Vec<SemanticToken> = Vec::new();
    if let Some(output) = &tokenized.output {
        for token in output.tokens.iter().filter(|token| !outside(&token.span)) {
            let class = classify_token(path, token, snapshot);
            if class.kind.is_empty() {
                continue;
            }
            add_token(&mut tokens, &index, &token.span, type_index(class.kind), class.modifiers);
        }
        for doc in output.docs.iter().filter(|doc| !outside(&doc.span)) {
            add_token(&mut tokens, &index, &doc.span, type_index("comment"), modifier_mask("documentation"));
        }
    }
    tokens.sort_by_key(|token| (token.line, token.start));
    let mut data: Vec<Value> = Vec::new();
    let (mut prev_line, mut prev_start, mut first) = (0usize, 0usize, true);
    for token in tokens.iter().filter(|token| intersects_range(token, &requested_range)) {
        let delta_line = if first { token.line } else { token.line - prev_line };
        let delta_start = if first || delta_line != 0 { token.start } else { token.start - prev_start };
        data.extend([json!(delta_line as i64), json!(delta_start as i64), json!(token.length as i64), json!(token.token_type), json!(token.modifiers)]);
        prev_line = token.line;
        prev_start = token.start;
        first = false;
    }
    json!({ "data": data })
}
