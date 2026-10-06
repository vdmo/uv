//! Symbols as the protocol shows them: workspace symbols, document symbols (flat or
//! nested by range), and the spans a rename touches.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use uv_analysis::language_service::{LanguageReference, LanguageSymbolInfo};
use uv_core::span::Span;
use uv_source::lexer::token::TokenKind;
use uv_tooling::line_index::{LineIndex, LineRange};
use uv_tooling::uri::{file_uri_to_path, path_to_file_uri};

use crate::config::{location_for_symbol, ByteRange};
use crate::hierarchy::symbol_kind_to_lsp;
use crate::jsonx::{get_str, range_json};
use crate::navigation::{span_contains, tokenize_text};

fn span_strictly_contains(outer: &Span, inner: &Span) -> bool {
    span_contains(outer, inner) && (outer.start_offset < inner.start_offset || inner.end_offset < outer.end_offset)
}

pub fn workspace_symbol_for_symbol(symbol: &LanguageSymbolInfo, text: &str) -> Value {
    let uri = path_to_file_uri(Path::new(&*symbol.selection_range.file));
    let mut item = json!({
        "name": symbol.name,
        "kind": symbol_kind_to_lsp(symbol.kind),
        "location": location_for_symbol(symbol, text),
        "data": { "symbolId": symbol.id, "uri": uri },
    });
    if !symbol.module_path.is_empty() {
        item["containerName"] = Value::String(symbol.module_path.clone());
    }
    item
}

fn document_symbol_for_symbol(symbol: &LanguageSymbolInfo, index: &LineIndex) -> Value {
    let mut item = json!({
        "name": symbol.name,
        "kind": symbol_kind_to_lsp(symbol.kind),
        "range": range_json(index.range_for(&symbol.range)),
        "selectionRange": range_json(index.range_for(&symbol.selection_range)),
    });
    if !symbol.detail.is_empty() {
        item["detail"] = Value::String(symbol.detail.clone());
    }
    item
}

fn document_symbol_for_tree(symbols: &[&LanguageSymbolInfo], children: &[Vec<usize>], at: usize, index: &LineIndex) -> Value {
    let mut item = document_symbol_for_symbol(symbols[at], index);
    if !children[at].is_empty() {
        item["children"] = Value::Array(children[at].iter().map(|&child| document_symbol_for_tree(symbols, children, child, index)).collect());
    }
    item
}

/// The symbols of a file nested by the ranges that contain one another.
pub fn hierarchical_document_symbols(symbols: &[&LanguageSymbolInfo], line_index: &LineIndex) -> Vec<Value> {
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); symbols.len()];
    let mut roots = Vec::new();
    let mut order: Vec<usize> = (0..symbols.len()).collect();
    order.sort_by(|&lhs, &rhs| {
        let (a, b) = (&symbols[lhs].range, &symbols[rhs].range);
        a.start_offset.cmp(&b.start_offset).then(b.end_offset.cmp(&a.end_offset))
    });
    let mut stack: Vec<usize> = Vec::new();
    for &symbol_index in &order {
        while stack.last().is_some_and(|&top| !span_strictly_contains(&symbols[top].range, &symbols[symbol_index].range)) {
            stack.pop();
        }
        match stack.last() {
            None => roots.push(symbol_index),
            Some(&top) => children[top].push(symbol_index),
        }
        stack.push(symbol_index);
    }
    roots.iter().map(|&root| document_symbol_for_tree(symbols, &children, root, line_index)).collect()
}

pub fn symbol_information_for_symbol(symbol: &LanguageSymbolInfo, text: &str) -> Value {
    let mut item = json!({ "name": symbol.name, "kind": symbol_kind_to_lsp(symbol.kind), "location": location_for_symbol(symbol, text) });
    if !symbol.module_path.is_empty() {
        item["containerName"] = Value::String(symbol.module_path.clone());
    }
    item
}

pub fn workspace_symbol_path(params: Option<&Value>) -> Option<PathBuf> {
    let params = params?;
    let uri = params
        .get("data")
        .and_then(|data| get_str(data, "uri"))
        .or_else(|| params.get("location").and_then(|location| get_str(location, "uri")))?;
    file_uri_to_path(uri)
}

fn narrow_identifier_span(path: &Path, text: &str, search_span: &Span, name: &str) -> Option<Span> {
    let tokenized = tokenize_text(path, text)?;
    tokenized.tokens.iter().find(|token| token.kind == TokenKind::Identifier && token.lexeme == name && span_contains(search_span, &token.span)).map(|token| token.span.clone())
}

pub fn rename_span_for_reference(path: &Path, text: &str, symbol: &LanguageSymbolInfo, reference: &LanguageReference) -> Option<Span> {
    if !reference.is_declaration {
        return Some(reference.range.clone());
    }
    narrow_identifier_span(path, text, &reference.range, &symbol.name)
}

pub fn selection_range_object(index: &LineIndex, ranges: &[ByteRange], range_index: usize) -> Value {
    let mut object = json!({ "range": range_json(LineRange { start: index.position_at(ranges[range_index].start), end: index.position_at(ranges[range_index].end) }) });
    if range_index + 1 < ranges.len() {
        object["parent"] = selection_range_object(index, ranges, range_index + 1);
    }
    object
}
