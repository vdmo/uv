//! Completion items, module-path completion in `using` and `import`, and the position
//! and edit helpers the requests share.

use std::collections::HashSet;
use std::path::Path;

use serde_json::{json, Value};
use uv_analysis::language_service::{language_symbol_kind_name, LanguageParameterInfo, LanguageSymbolInfo, LanguageSymbolKind};
use uv_project::project::Project;
use uv_source::lexer::token::{Token, TokenKind};
use uv_source::module_paths::resolve_import_module_path;
use uv_tooling::analysis::AnalysisSnapshot;
use uv_tooling::line_index::{LineIndex, LinePosition, LineRange};
use uv_tooling::uri::{path_key, path_to_file_uri};

use crate::jsonx::{get_i64, range_json};
use crate::modules::*;
use crate::text::word_at;

pub fn completion_kind_for_symbol(kind: LanguageSymbolKind) -> i64 {
    match kind {
        LanguageSymbolKind::Function => 3,
        LanguageSymbolKind::Method => 2,
        LanguageSymbolKind::Record | LanguageSymbolKind::Class => 7,
        LanguageSymbolKind::Enum => 13,
        LanguageSymbolKind::Field => 5,
        LanguageSymbolKind::EnumMember => 20,
        LanguageSymbolKind::Variable | LanguageSymbolKind::Parameter => 6,
        LanguageSymbolKind::Constant => 21,
        LanguageSymbolKind::Modal | LanguageSymbolKind::TypeAlias | LanguageSymbolKind::State => 25,
        LanguageSymbolKind::Module => 9,
    }
}

pub fn completion_detail_for_symbol(symbol: &LanguageSymbolInfo) -> String {
    if !symbol.signature_label.is_empty() {
        return symbol.signature_label.clone();
    }
    if !symbol.detail.is_empty() {
        return symbol.detail.clone();
    }
    language_symbol_kind_name(symbol.kind).to_string()
}

pub fn completion_documentation_for_symbol(symbol: &LanguageSymbolInfo) -> Value {
    let mut markdown = String::new();
    if !symbol.signature_label.is_empty() {
        markdown.push_str("```ultraviolet\n");
        markdown.push_str(&symbol.signature_label);
        markdown.push_str("\n```");
    }
    if !symbol.documentation.is_empty() {
        if !markdown.is_empty() {
            markdown.push('\n');
        }
        markdown.push_str(&symbol.documentation);
    }
    json!({ "kind": "markdown", "value": markdown })
}

pub fn completion_data_for_symbol(symbol: &LanguageSymbolInfo) -> Value {
    json!({ "symbolId": symbol.id, "uri": path_to_file_uri(Path::new(&*symbol.selection_range.file)) })
}

pub fn inlay_hint_data_for_parameter(path: &Path, symbol: &LanguageSymbolInfo, parameter_index: usize) -> Value {
    json!({
        "uri": path_to_file_uri(path),
        "symbolUri": path_to_file_uri(Path::new(&*symbol.selection_range.file)),
        "symbolId": symbol.id,
        "parameterIndex": parameter_index as i64,
    })
}

pub fn inlay_hint_tooltip_for_parameter(symbol: &LanguageSymbolInfo, parameter: &LanguageParameterInfo) -> Value {
    let mut markdown = String::new();
    if !symbol.signature_label.is_empty() {
        markdown.push_str("```ultraviolet\n");
        markdown.push_str(&symbol.signature_label);
        markdown.push_str("\n```");
    } else {
        markdown.push('`');
        markdown.push_str(&symbol.qualified_name);
        markdown.push('`');
    }
    let label = if parameter.label.is_empty() { &parameter.name } else { &parameter.label };
    if !label.is_empty() {
        markdown.push_str("\nParameter: `");
        markdown.push_str(label);
        markdown.push('`');
    }
    json!({ "kind": "markdown", "value": markdown })
}

pub fn split_module_path_string(path: &str) -> Vec<String> {
    path.split("::").map(str::to_string).collect()
}

pub fn module_path_starts_with(path: &[String], prefix: &[String]) -> bool {
    prefix.len() <= path.len() && path[..prefix.len()] == *prefix
}

pub fn current_module_path_for_file(snapshot: &AnalysisSnapshot, path: &Path) -> Option<Vec<String>> {
    let key = path_key(path);
    snapshot
        .modules
        .iter()
        .find(|module| module.items.iter().any(|item| span_for_item(item).is_some_and(|span| path_key(Path::new(&*span.file)) == key)))
        .map(|module| module.path.clone())
}

fn line_start_offset(text: &str, offset: usize) -> usize {
    let offset = offset.min(text.len());
    if offset == 0 {
        return 0;
    }
    text[..offset].rfind('\n').map_or(0, |newline| newline + 1)
}

fn is_module_import_directive(token: &Token) -> bool {
    token.lexeme == "using" || token.lexeme == "import"
}

pub fn text_edit(range: LineRange, new_text: &str) -> Value {
    json!({ "range": range_json(range), "newText": new_text })
}

pub fn module_completion_context_at(
    snapshot: &AnalysisSnapshot,
    path: &Path,
    text: &str,
    index: &LineIndex,
    tokens: &[Token],
    offset: usize,
) -> Option<ModuleCompletionContext> {
    snapshot.project.as_ref()?;
    let current_module = current_module_path_for_file(snapshot, path)?;
    let line_start = line_start_offset(text, offset);
    let mut directive_index: Option<usize> = None;
    let mut directive = String::new();
    for (token_index, token) in tokens.iter().enumerate() {
        if token.kind == TokenKind::Eof || token.span.start_offset > offset {
            break;
        }
        if token.span.end_offset <= line_start {
            continue;
        }
        if token.kind == TokenKind::Newline {
            break;
        }
        if is_module_import_directive(token) {
            directive_index = Some(token_index);
            directive = token.lexeme.clone();
        }
    }
    let directive_index = directive_index?;
    if offset <= tokens[directive_index].span.end_offset {
        return None;
    }
    let mut completed_path: Vec<String> = Vec::new();
    let mut active_identifier: Option<&Token> = None;
    let mut in_using_list = false;
    for token in &tokens[directive_index + 1..] {
        if token.kind == TokenKind::Eof || token.kind == TokenKind::Newline || token.span.start_offset > offset {
            break;
        }
        if token.span.end_offset <= line_start {
            continue;
        }
        match token.lexeme.as_str() {
            "as" | "}" | "*" => return None,
            "{" => {
                if directive != "using" || completed_path.is_empty() {
                    return None;
                }
                in_using_list = true;
                continue;
            }
            "," => {
                if !in_using_list {
                    return None;
                }
                continue;
            }
            _ => {}
        }
        if token.kind != TokenKind::Identifier {
            continue;
        }
        if offset >= token.span.start_offset && offset <= token.span.end_offset {
            active_identifier = Some(token);
            break;
        }
        if token.span.end_offset <= offset && !in_using_list {
            completed_path.push(token.lexeme.clone());
        }
    }
    if let Some(identifier) = active_identifier {
        return Some(ModuleCompletionContext {
            directive,
            current_module,
            completed_path,
            query: identifier.lexeme.clone(),
            edit_range: index.range_for(&identifier.span),
        });
    }
    let position = index.position_at(offset);
    Some(ModuleCompletionContext { directive, current_module, completed_path, query: String::new(), edit_range: LineRange { start: position, end: position } })
}

fn append_module_completion_items_for_base(
    items: &mut Vec<Value>,
    project: &Project,
    context: &ModuleCompletionContext,
    base: &[String],
    seen_labels: &mut HashSet<String>,
    ordinal: &mut usize,
) {
    for module in &project.modules {
        let module_path = split_module_path_string(&module.path);
        if module_path.len() <= base.len() || !module_path_starts_with(&module_path, base) {
            continue;
        }
        let label = &module_path[base.len()];
        if !name_matches_query(label, &context.query) || !seen_labels.insert(label.clone()) {
            continue;
        }
        items.push(json!({
            "label": label,
            "kind": 9,
            "detail": format!("module {}", module_path.join("::")),
            "filterText": label,
            "sortText": completion_sort_text('0', *ordinal, label),
            "textEdit": text_edit(context.edit_range, label),
        }));
        *ordinal += 1;
    }
}

fn append_using_member_completion_items(items: &mut Vec<Value>, snapshot: &AnalysisSnapshot, context: &ModuleCompletionContext) {
    let Some(project) = snapshot.project.as_ref() else {
        return;
    };
    if context.directive != "using" || context.completed_path.is_empty() {
        return;
    }
    let lookup = build_module_lookup(project);
    let Some(resolved) = resolve_import_module_path(&context.current_module, &lookup.names, &context.completed_path) else {
        return;
    };
    let module_path = resolved.join("::");
    let mut seen_labels = HashSet::new();
    let mut ordinal = 0;
    for symbol in snapshot.language_service.symbols() {
        if symbol.module_path != module_path || symbol.is_local || !name_matches_query(&symbol.name, &context.query) || !seen_labels.insert(symbol.name.clone()) {
            continue;
        }
        items.push(json!({
            "label": symbol.name,
            "kind": completion_kind_for_symbol(symbol.kind),
            "detail": completion_detail_for_symbol(symbol),
            "data": completion_data_for_symbol(symbol),
            "filterText": symbol.name,
            "sortText": completion_sort_text('0', ordinal, &symbol.name),
            "textEdit": text_edit(context.edit_range, &symbol.name),
        }));
        ordinal += 1;
    }
}

pub fn append_module_completion_items(items: &mut Vec<Value>, snapshot: &AnalysisSnapshot, path: &Path, text: &str, index: &LineIndex, tokens: &[Token], offset: usize) {
    let Some(context) = module_completion_context_at(snapshot, path, text, index, tokens, offset) else {
        return;
    };
    let Some(project) = snapshot.project.as_ref() else {
        return;
    };
    let mut seen_labels = HashSet::new();
    let mut ordinal = 0;
    append_using_member_completion_items(items, snapshot, &context);
    if let Some(first) = context.current_module.first() {
        let mut base = vec![first.clone()];
        base.extend(context.completed_path.iter().cloned());
        append_module_completion_items_for_base(items, project, &context, &base, &mut seen_labels, &mut ordinal);
    }
    append_module_completion_items_for_base(items, project, &context, &context.completed_path, &mut seen_labels, &mut ordinal);
}

pub fn escape_completion_snippet_text(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if matches!(ch, '$' | '}' | '\\') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

pub fn completion_placeholder_text(parameter: &LanguageParameterInfo) -> String {
    if !parameter.name.is_empty() {
        return parameter.name.clone();
    }
    if !parameter.label.is_empty() {
        return parameter.label.clone();
    }
    "value".to_string()
}

pub fn moniker_for_symbol(symbol: &LanguageSymbolInfo) -> Value {
    json!({
        "scheme": "ultraviolet",
        "identifier": if symbol.id.is_empty() { &symbol.qualified_name } else { &symbol.id },
        "unique": if symbol.is_local { "document" } else { "project" },
        "kind": if symbol.is_local { "local" } else { "export" },
    })
}

pub fn workspace_edit_for_fix_it(uri: &str, range: LineRange, new_text: &str) -> Value {
    json!({ "changes": { uri: [text_edit(range, new_text)] } })
}

pub fn completion_edit_range(text: &str, index: &LineIndex, position: LinePosition) -> LineRange {
    let offset = index.byte_offset_at(position);
    match word_at(text, offset) {
        Some(word) => LineRange { start: index.position_at(word.start), end: index.position_at(word.end) },
        None => LineRange { start: position, end: position },
    }
}

pub fn position_from_object(position: Option<&Value>) -> Option<LinePosition> {
    let position = position?;
    let line = get_i64(position, "line")?;
    let character = get_i64(position, "character")?;
    if line < 0 || character < 0 {
        return None;
    }
    Some(LinePosition { line: line as usize, character: character as usize })
}

pub fn range_from_json(range: Option<&Value>) -> Option<LineRange> {
    let range = range?;
    Some(LineRange { start: position_from_object(range.get("start"))?, end: position_from_object(range.get("end"))? })
}

fn position_less_or_equal(lhs: LinePosition, rhs: LinePosition) -> bool {
    lhs.line < rhs.line || (lhs.line == rhs.line && lhs.character <= rhs.character)
}

/// The offset of a position, if the position is one the text has.
pub fn validated_byte_offset_at(index: &LineIndex, position: LinePosition) -> Option<usize> {
    let offset = index.byte_offset_at(position);
    (index.position_at(offset) == position).then_some(offset)
}

pub fn validated_byte_offsets_for_range(index: &LineIndex, range: LineRange) -> Option<(usize, usize)> {
    if !position_less_or_equal(range.start, range.end) {
        return None;
    }
    let start = validated_byte_offset_at(index, range.start)?;
    let end = validated_byte_offset_at(index, range.end)?;
    (start <= end).then_some((start, end))
}

pub fn code_action_data_for_fix_it(uri: &str, range: LineRange, new_text: &str) -> Value {
    json!({ "uri": uri, "range": range_json(range), "newText": new_text })
}

/// Applies the changes of a `didChange`, each either a whole new text or a replaced range.
pub fn apply_content_changes(mut text: String, changes: &[Value]) -> Option<String> {
    for change in changes {
        let new_text = change.as_object()?.get("text")?.as_str()?;
        let Some(range_object) = change.get("range").filter(|range| range.is_object()) else {
            text = new_text.to_string();
            continue;
        };
        let range = range_from_json(Some(range_object))?;
        let line_count = text.matches('\n').count() + 1;
        if range.start.line >= line_count || range.end.line >= line_count {
            return None;
        }
        let index = LineIndex::new(&text);
        let start = index.byte_offset_at(range.start).min(text.len());
        let end = index.byte_offset_at(range.end).min(text.len()).max(start);
        text.replace_range(start..end, new_text);
    }
    Some(text)
}
