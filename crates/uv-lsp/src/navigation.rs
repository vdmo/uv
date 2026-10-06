//! Tokenizing text for the editor, code lenses and snippets, and the targets of module paths.

use std::path::Path;

use serde_json::{json, Value};
use uv_analysis::language_service::{LanguageSymbolInfo, LanguageSymbolKind};
use uv_core::source_load::load_source;
use uv_core::span::Span;
use uv_source::lexer::token::{Token, TokenKind};
use uv_source::lexer::{scan_ident_token, tokenize_with_diagnostics, LexerOutput};
use uv_source::module_paths::resolve_import_module_path;
use uv_tooling::analysis::AnalysisSnapshot;
use uv_tooling::line_index::LineIndex;
use uv_tooling::uri::{path_key, path_to_file_uri};

use crate::completion::{completion_placeholder_text, escape_completion_snippet_text};
use crate::jsonx::range_json;
use crate::modules::*;

pub const SHOW_REFERENCES_COMMAND: &str = "uv-lsp.showReferences";

pub fn tokenize_text(path: &Path, text: &str) -> Option<LexerOutput> {
    let loaded = load_source(&path.to_string_lossy(), text.as_bytes());
    tokenize_with_diagnostics(loaded.source.as_ref()?).output
}

pub fn is_callable_symbol_kind(kind: LanguageSymbolKind) -> bool {
    matches!(kind, LanguageSymbolKind::Function | LanguageSymbolKind::Method)
}

fn supports_code_lens_symbol_kind(kind: LanguageSymbolKind) -> bool {
    use LanguageSymbolKind as K;
    matches!(kind, K::Function | K::Method | K::Record | K::Enum | K::Modal | K::Class | K::TypeAlias)
}

pub fn supports_reference_code_lens(symbol: &LanguageSymbolInfo) -> bool {
    symbol.include_in_outline && !symbol.is_local && supports_code_lens_symbol_kind(symbol.kind)
}

pub fn reference_count_title(count: usize) -> String {
    if count == 1 {
        "1 reference".to_string()
    } else {
        format!("{count} references")
    }
}

pub fn code_lens_data_for_symbol(symbol: &LanguageSymbolInfo) -> Value {
    json!({ "symbolId": symbol.id, "uri": path_to_file_uri(Path::new(&*symbol.selection_range.file)) })
}

pub fn code_lens_for_symbol(symbol: &LanguageSymbolInfo, index: &LineIndex) -> Value {
    json!({ "range": range_json(index.range_for(&symbol.selection_range)), "data": code_lens_data_for_symbol(symbol) })
}

pub fn completion_snippet_for_symbol(symbol: &LanguageSymbolInfo) -> Option<String> {
    if !is_callable_symbol_kind(symbol.kind) || symbol.signature_label.is_empty() {
        return None;
    }
    let placeholders: Vec<String> = symbol
        .parameters
        .iter()
        .enumerate()
        .map(|(i, parameter)| format!("${{{}:{}}}", i + 1, escape_completion_snippet_text(&completion_placeholder_text(parameter))))
        .collect();
    Some(format!("{}({})$0", symbol.name, placeholders.join(", ")))
}

pub fn is_type_symbol_kind(kind: LanguageSymbolKind) -> bool {
    use LanguageSymbolKind as K;
    matches!(kind, K::Record | K::Class | K::Enum | K::Modal | K::State | K::TypeAlias)
}

pub fn is_type_hierarchy_symbol_kind(kind: LanguageSymbolKind) -> bool {
    kind == LanguageSymbolKind::Class
}

/// Whether the text is exactly one identifier.
pub fn is_valid_rename_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let loaded = load_source("<rename>", name.as_bytes());
    let Some(source) = loaded.source else {
        return false;
    };
    let ident = scan_ident_token(&source, 0);
    ident.ok && ident.next == source.scalars.len() && ident.kind == TokenKind::Identifier && ident.lexeme == name
}

pub fn position_in_range(offset: usize, start: usize, end: usize) -> bool {
    offset >= start && offset < end
}

pub fn span_contains(outer: &Span, inner: &Span) -> bool {
    path_key(Path::new(&*outer.file)) == path_key(Path::new(&*inner.file)) && outer.start_offset <= inner.start_offset && inner.end_offset <= outer.end_offset
}

fn is_module_path_separator(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Operator | TokenKind::Punctuator) && matches!(token.lexeme.as_str(), "::" | "." | ":")
}

/// Where the path of a `using` or `import` is written.
pub fn module_path_span_for_document_link(tokens: &[Token], item_span: &Span, module_path: &[String]) -> Option<Span> {
    let first_name = module_path.first()?;
    for (token_index, first) in tokens.iter().enumerate() {
        if first.kind != TokenKind::Identifier || first.lexeme != *first_name || !span_contains(item_span, &first.span) {
            continue;
        }
        let mut matched_index = token_index;
        let mut end_offset = first.span.end_offset;
        let mut matched = true;
        for component in &module_path[1..] {
            let mut found_component = false;
            matched_index += 1;
            while matched_index < tokens.len() {
                let token = &tokens[matched_index];
                if !span_contains(item_span, &token.span) {
                    matched = false;
                    break;
                }
                if is_module_path_separator(token) {
                    matched_index += 1;
                    continue;
                }
                if token.kind != TokenKind::Identifier || token.lexeme != *component {
                    matched = false;
                } else {
                    end_offset = token.span.end_offset;
                }
                found_component = true;
                break;
            }
            if !found_component || !matched {
                matched = false;
                break;
            }
        }
        if !matched {
            continue;
        }
        let mut result = item_span.clone();
        result.start_offset = first.span.start_offset;
        result.end_offset = end_offset;
        return Some(result);
    }
    None
}

pub fn module_navigation_target_at(snapshot: &AnalysisSnapshot, path: &Path, tokens: &[Token], offset: usize) -> Option<ModuleNavigationTarget> {
    let project = snapshot.project.as_ref()?;
    let lookup = build_module_lookup(project);
    let key = path_key(path);
    for module in &snapshot.modules {
        for item in &module.items {
            let Some(item_path) = module_path_for_link_item(item) else {
                continue;
            };
            let Some(item_span) = span_for_item(item) else {
                continue;
            };
            if path_key(Path::new(&*item_span.file)) != key {
                continue;
            }
            let Some(origin_span) = module_path_span_for_document_link(tokens, item_span, &item_path) else {
                continue;
            };
            if !position_in_range(offset, origin_span.start_offset, origin_span.end_offset) {
                continue;
            }
            let Some(resolved) = resolve_import_module_path(&module.path, &lookup.names, &item_path) else {
                continue;
            };
            let Some(target) = document_link_target_for_module(&lookup, &resolved) else {
                continue;
            };
            return Some(ModuleNavigationTarget { target, origin_span, module_path: resolved.join("::") });
        }
    }
    None
}
