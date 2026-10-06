//! The requests that edit or annotate: diagnostics, code actions and lenses, rename,
//! formatting, folding and selection ranges, signature help and inlay hints.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};
use uv_source::lexer::token::TokenKind;
use uv_tooling::line_index::{LineIndex, LinePosition, LineRange};
use uv_tooling::uri::{file_uri_to_path, path_key, path_to_file_uri};

use crate::completion::*;
use crate::config::*;
use crate::diagnostics::DiagnosticTextCache;
use crate::hierarchy::*;
use crate::jsonx::{get_bool, get_i64, get_str, position_json, position_param, range_json};
use crate::modules::*;
use crate::navigation::*;
use crate::server::Server;
use crate::symbols::*;
use crate::text::*;

impl Server {
    pub fn handle_completion_item_resolve(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return json!({});
        };
        let mut item = params.clone();
        let Some(data) = params.get("data") else {
            return item;
        };
        let (Some(symbol_id), Some(uri)) = (get_str(data, "symbolId"), get_str(data, "uri")) else {
            return item;
        };
        let Some(path) = file_uri_to_path(uri) else {
            return item;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return item;
        };
        let Some(symbol) = project.snapshot.language_service.symbol_by_id(symbol_id).filter(|symbol| path_key(Path::new(&*symbol.selection_range.file)) == path_key(&path)) else {
            return item;
        };
        item["detail"] = json!(completion_detail_for_symbol(symbol));
        if !symbol.signature_label.is_empty() || !symbol.documentation.is_empty() {
            item["documentation"] = completion_documentation_for_symbol(symbol);
        }
        item
    }

    pub fn handle_text_document_diagnostic(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return full_diagnostic_report(Vec::new(), None);
        };
        let previous = params.and_then(|params| get_str(params, "previousResultId"));
        self.diagnostic_report_for_path(&path, previous, &mut DiagnosticTextCache::new())
    }

    pub fn handle_workspace_diagnostic(&mut self, params: Option<&Value>) -> Value {
        let previous = previous_workspace_result_ids(params);
        let mut paths_by_key: BTreeMap<String, PathBuf> = BTreeMap::new();
        for overlay in self.documents.overlays() {
            paths_by_key.entry(path_key(&overlay.path)).or_insert(overlay.path.clone());
        }
        for project in self.core.project_snapshots() {
            for module in &project.snapshot.modules {
                for item in &module.items {
                    if let Some(span) = span_for_item(item) {
                        let path = PathBuf::from(&*span.file);
                        paths_by_key.entry(path_key(&path)).or_insert(path);
                    }
                }
            }
            for diagnostic in &project.snapshot.diagnostics {
                if let Some(span) = &diagnostic.span {
                    let path = PathBuf::from(&*span.file);
                    paths_by_key.entry(path_key(&path)).or_insert(path);
                }
            }
        }
        let mut cache = DiagnosticTextCache::new();
        let items: Vec<Value> = paths_by_key.values().map(|path| self.workspace_diagnostic_report_for_path(path, &previous, &mut cache)).collect();
        json!({ "items": items })
    }

    fn has_text(&self, path: &Path) -> bool {
        self.documents.find_by_path(path).is_some() || path.exists()
    }

    pub fn handle_code_action(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return json!([]);
        };
        let Some(uri) = crate::jsonx::text_document_uri(Some(params)) else {
            return json!([]);
        };
        let context = params.get("context").filter(|context| context.is_object());
        if !code_action_context_allows_quickfix(context) {
            return json!([]);
        }
        let Some(diagnostics) = context.and_then(|context| context.get("diagnostics")).and_then(Value::as_array) else {
            return json!([]);
        };
        let mut actions = Vec::new();
        for diag in diagnostics.iter().filter(|diag| diag.is_object()) {
            let Some(fixits) = diag.get("data").and_then(|data| data.get("fixits")).and_then(Value::as_array) else {
                continue;
            };
            for fixit in fixits.iter().filter(|fixit| fixit.is_object()) {
                let title = get_str(fixit, "title").unwrap_or("Apply fix");
                let fix_uri = get_str(fixit, "uri").unwrap_or(&uri).to_string();
                let (Some(new_text), Some(range), Some(fix_path)) = (get_str(fixit, "newText"), range_from_json(fixit.get("range")), file_uri_to_path(&fix_uri)) else {
                    continue;
                };
                if !self.has_text(&fix_path) {
                    continue;
                }
                let fix_index = LineIndex::new(&self.text_for_path(&fix_path));
                if validated_byte_offsets_for_range(&fix_index, range).is_none() {
                    continue;
                }
                actions.push(json!({
                    "title": title,
                    "kind": "quickfix",
                    "diagnostics": [diag],
                    "edit": workspace_edit_for_fix_it(&fix_uri, range, new_text),
                    "data": code_action_data_for_fix_it(&fix_uri, range, new_text),
                }));
            }
        }
        Value::Array(actions)
    }

    pub fn handle_code_action_resolve(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return Value::Null;
        };
        let mut action = params.clone();
        if action.get("edit").is_some_and(Value::is_object) {
            return action;
        }
        let data = action.get("data").cloned();
        let uri = data.as_ref().and_then(|data| get_str(data, "uri")).map(str::to_string);
        let new_text = data.as_ref().and_then(|data| get_str(data, "newText")).map(str::to_string);
        let range = data.as_ref().and_then(|data| range_from_json(data.get("range")));
        let path = uri.as_deref().and_then(file_uri_to_path);
        if let (Some(uri), Some(new_text), Some(range), Some(path)) = (uri, new_text, range, path) {
            if self.has_text(&path) && validated_byte_offsets_for_range(&LineIndex::new(&self.text_for_path(&path)), range).is_some() {
                action["edit"] = workspace_edit_for_fix_it(&uri, range, &new_text);
            }
        }
        action
    }

    pub fn handle_code_lens(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let index = LineIndex::new(&self.text_for_path(&path));
        Value::Array(
            project
                .snapshot
                .language_service
                .symbols_in_file(&path.to_string_lossy())
                .into_iter()
                .filter(|symbol| supports_reference_code_lens(symbol))
                .map(|symbol| code_lens_for_symbol(symbol, &index))
                .collect(),
        )
    }

    pub fn handle_code_lens_resolve(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return Value::Null;
        };
        let mut lens = params.clone();
        let data = params.get("data");
        let (Some(symbol_id), Some(uri)) = (data.and_then(|data| get_str(data, "symbolId")), data.and_then(|data| get_str(data, "uri"))) else {
            return lens;
        };
        let path = file_uri_to_path(uri);
        let project = path.as_ref().and_then(|path| self.snapshot_for_path(path));
        let (Some(path), Some(project)) = (path, project) else {
            return lens;
        };
        let Some(symbol) = project
            .snapshot
            .language_service
            .symbol_by_id(symbol_id)
            .filter(|symbol| path_key(Path::new(&*symbol.selection_range.file)) == path_key(&path) && supports_reference_code_lens(symbol))
        else {
            return lens;
        };
        let locations = self.reference_locations_for_symbol(symbol, false);
        let count = locations.len();
        let index = LineIndex::new(&self.text_for_path(Path::new(&*symbol.selection_range.file)));
        let selection = index.range_for(&symbol.selection_range);
        lens["command"] = json!({
            "title": reference_count_title(count),
            "command": SHOW_REFERENCES_COMMAND,
            "arguments": [uri, position_json(selection.start), locations],
        });
        lens
    }

    pub fn handle_will_change_files(&mut self, _params: Option<&Value>) -> Value {
        json!({ "changes": {} })
    }

    pub fn handle_prepare_rename(&mut self, params: Option<&Value>) -> Value {
        let Some((path, text, index, offset)) = self.rename_context(params) else {
            return Value::Null;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return Value::Null;
        };
        let file = path.to_string_lossy();
        let Some(reference) = project.snapshot.language_service.reference_at(&file, offset) else {
            return Value::Null;
        };
        let Some(symbol) = project.snapshot.language_service.symbol_by_id(&reference.symbol_id).filter(|symbol| !symbol.name.is_empty()) else {
            return Value::Null;
        };
        match rename_span_for_reference(&path, &text, symbol, reference) {
            Some(span) if position_in_range(offset, span.start_offset, span.end_offset) => json!({ "range": range_json(index.range_for(&span)), "placeholder": symbol.name }),
            _ => Value::Null,
        }
    }

    fn rename_context(&self, params: Option<&Value>) -> Option<(PathBuf, String, LineIndex, usize)> {
        let path = self.path_from_text_document(params)?;
        let position = position_param(params)?;
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let offset = validated_byte_offset_at(&index, position)?;
        Some((path, text, index, offset))
    }

    pub fn handle_rename(&mut self, params: Option<&Value>) -> Value {
        let nothing = || json!({ "changes": {} });
        let Some(params) = params else {
            return nothing();
        };
        let Some((path, text, _, offset)) = self.rename_context(Some(params)) else {
            return nothing();
        };
        let Some(new_name) = get_str(params, "newName").filter(|name| is_valid_rename_name(name)) else {
            return nothing();
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return nothing();
        };
        let file = path.to_string_lossy();
        let Some(reference) = project.snapshot.language_service.reference_at(&file, offset) else {
            return nothing();
        };
        let Some(symbol) = project.snapshot.language_service.symbol_by_id(&reference.symbol_id).filter(|symbol| !symbol.name.is_empty()) else {
            return nothing();
        };
        match rename_span_for_reference(&path, &text, symbol, reference) {
            Some(span) if position_in_range(offset, span.start_offset, span.end_offset) => {}
            _ => return nothing(),
        }
        let mut changes_by_uri: BTreeMap<String, Vec<Value>> = BTreeMap::new();
        for reference in project.snapshot.language_service.references_for_symbol(&symbol.id, true) {
            let file = Path::new(&*reference.range.file);
            let file_text = self.text_for_path(file);
            let file_index = LineIndex::new(&file_text);
            let Some(span) = rename_span_for_reference(file, &file_text, symbol, reference) else {
                return nothing();
            };
            changes_by_uri.entry(path_to_file_uri(file)).or_default().push(text_edit(file_index.range_for(&span), new_name));
        }
        if self.flags().workspace_edit_document_changes_support {
            let document_changes: Vec<Value> = changes_by_uri
                .into_iter()
                .map(|(uri, edits)| {
                    let version = file_uri_to_path(&uri).and_then(|file| self.documents.find_by_path(&file).map(|overlay| overlay.version));
                    json!({ "textDocument": { "uri": uri, "version": version }, "edits": edits })
                })
                .collect();
            return json!({ "documentChanges": document_changes });
        }
        let mut changes = Map::new();
        for (uri, edits) in changes_by_uri {
            changes.insert(uri, Value::Array(edits));
        }
        json!({ "changes": changes })
    }

    pub fn handle_folding_range(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let Some(tokenized) = tokenize_text(&path, &text) else {
            return json!([]);
        };
        Value::Array(
            delimited_byte_ranges(&tokenized.tokens)
                .into_iter()
                .filter_map(|range| {
                    let (start, end) = (index.position_at(range.start), index.position_at(range.end));
                    (start.line < end.line).then(|| json!({ "startLine": start.line, "startCharacter": start.character, "endLine": end.line, "endCharacter": end.character, "kind": "region" }))
                })
                .collect(),
        )
    }

    pub fn handle_selection_range(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return json!([]);
        };
        let Some(path) = self.path_from_text_document(Some(params)) else {
            return json!([]);
        };
        let Some(positions) = params.get("positions").and_then(Value::as_array) else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let tokenized = tokenize_text(&path, &text);
        let delimited = tokenized.as_ref().map(|output| delimited_byte_ranges(&output.tokens)).unwrap_or_default();
        let mut result = Vec::new();
        for position_value in positions {
            let offset = position_from_object(Some(position_value)).and_then(|position| validated_byte_offset_at(&index, position));
            let Some(offset) = offset else {
                result.push(Value::Null);
                continue;
            };
            let mut ranges: Vec<ByteRange> = Vec::new();
            if let Some(word) = word_at(&text, offset) {
                let word_range = ByteRange { start: word.start, end: word.end };
                if contains_offset(word_range, offset) {
                    add_distinct_range(&mut ranges, word_range);
                }
            }
            if let Some(output) = &tokenized {
                for token in &output.tokens {
                    if matches!(token.kind, TokenKind::Eof | TokenKind::Newline) {
                        continue;
                    }
                    let token_range = ByteRange { start: token.span.start_offset, end: token.span.end_offset };
                    if contains_offset(token_range, offset) {
                        add_distinct_range(&mut ranges, token_range);
                        break;
                    }
                }
            }
            for range in &delimited {
                if contains_offset(*range, offset) {
                    add_distinct_range(&mut ranges, *range);
                }
            }
            let document_range = ByteRange { start: 0, end: text.len() };
            if contains_offset(document_range, offset) {
                add_distinct_range(&mut ranges, document_range);
            }
            ranges.sort_by(|lhs, rhs| (lhs.end - lhs.start).cmp(&(rhs.end - rhs.start)).then(rhs.start.cmp(&lhs.start)));
            result.push(if ranges.is_empty() { Value::Null } else { selection_range_object(&index, &ranges, 0) });
        }
        Value::Array(result)
    }

    fn formatted_text(&self, path: &Path, text: &str, params: Option<&Value>) -> Option<(String, FormatOptions)> {
        let options = format_options_from_params(params);
        let tokenized = tokenize_text(path, text)?;
        Some((format_ultraviolet_text(&tokenized.tokens, text, &options), options))
    }

    pub fn handle_formatting(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        match self.formatted_text(&path, &text, params) {
            Some((formatted, _)) if formatted != text => json!([text_edit(LineRange { start: LinePosition::default(), end: index.position_at(text.len()) }, &formatted)]),
            _ => json!([]),
        }
    }

    pub fn handle_will_save_wait_until(&mut self, params: Option<&Value>) -> Value {
        self.handle_formatting(params)
    }

    pub fn handle_range_formatting(&mut self, params: Option<&Value>) -> Value {
        let path = self.path_from_text_document(params);
        let range = range_from_json(params.and_then(|params| params.get("range")));
        let (Some(path), Some(range)) = (path, range) else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        if validated_byte_offsets_for_range(&index, range).is_none() {
            return json!([]);
        }
        let Some((formatted, _)) = self.formatted_text(&path, &text, params).filter(|(formatted, _)| *formatted != text) else {
            return json!([]);
        };
        let original_lines = split_text_lines(&text);
        let formatted_lines = split_text_lines(&formatted);
        if original_lines.len() != formatted_lines.len() {
            return json!([]);
        }
        let Some((start_line, end_line)) = format_line_selection(original_lines.len(), &range) else {
            return json!([]);
        };
        let start_offset = original_lines[start_line].start_offset;
        let end_offset = original_lines[end_line].end_offset;
        let replacement = line_slice_text(&formatted_lines, start_line, end_line);
        if text[start_offset..end_offset] == replacement {
            return json!([]);
        }
        json!([text_edit(LineRange { start: index.position_at(start_offset), end: index.position_at(end_offset) }, &replacement)])
    }

    pub fn handle_on_type_formatting(&mut self, params: Option<&Value>) -> Value {
        let path = self.path_from_text_document(params);
        let position = position_param(params);
        let trigger = params.and_then(|params| get_str(params, "ch"));
        let (Some(path), Some(position)) = (path, position) else {
            return json!([]);
        };
        if !matches!(trigger, Some("\n" | "}")) {
            return json!([]);
        }
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        if validated_byte_offset_at(&index, position).is_none() {
            return json!([]);
        }
        let original_lines = split_text_lines(&text);
        let line = position.line;
        if original_lines.is_empty() || line >= original_lines.len() {
            return json!([]);
        }
        let formatted = self.formatted_text(&path, &text, params);
        let indent_unit = format_options_from_params(params).indent_unit;
        if let Some((formatted, _)) = formatted.as_ref().filter(|(formatted, _)| *formatted != text) {
            let formatted_lines = split_text_lines(formatted);
            if formatted_lines.len() == original_lines.len() {
                let replacement = line_slice_text(&formatted_lines, line, line);
                let original = &original_lines[line];
                if replacement != text[original.start_offset..original.end_offset] {
                    return json!([text_edit(LineRange { start: index.position_at(original.start_offset), end: index.position_at(original.end_offset) }, &replacement)]);
                }
            }
        }
        let original = &original_lines[line];
        if !trim_outer_whitespace(&original.content).is_empty() {
            return json!([]);
        }
        let Some(tokenized) = tokenize_text(&path, &text) else {
            return json!([]);
        };
        let layout = build_line_layout_info(&tokenized.tokens, &index, original_lines.len());
        let indent = expected_indent_for_line(&layout, line, &indent_unit);
        if indent == original.content {
            return json!([]);
        }
        let content_end = original.end_offset - original.line_ending.len();
        json!([text_edit(LineRange { start: index.position_at(original.start_offset), end: index.position_at(content_end) }, &indent)])
    }

    pub fn handle_signature_help(&mut self, params: Option<&Value>) -> Value {
        let Some((path, text, _, offset)) = self.rename_context(params) else {
            return Value::Null;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return Value::Null;
        };
        let Some(tokenized) = tokenize_text(&path, &text) else {
            return Value::Null;
        };
        let Some(context) = call_context_at(&tokenized.tokens, offset) else {
            return Value::Null;
        };
        let callee = &tokenized.tokens[context.callee_index];
        let file = path.to_string_lossy();
        let Some(symbol) = project.snapshot.language_service.resolved_symbol_at(&file, callee.span.start_offset).filter(|symbol| is_callable_symbol_kind(symbol.kind) && !symbol.signature_label.is_empty()) else {
            return Value::Null;
        };
        let mut overloads = vec![symbol];
        if !symbol.qualified_name.is_empty() {
            let mut seen_labels = std::collections::HashSet::from([symbol.signature_label.clone()]);
            for candidate in project.snapshot.language_service.symbols() {
                if std::ptr::eq(candidate, symbol)
                    || !is_callable_symbol_kind(candidate.kind)
                    || candidate.signature_label.is_empty()
                    || candidate.qualified_name != symbol.qualified_name
                    || !seen_labels.insert(candidate.signature_label.clone())
                {
                    continue;
                }
                overloads.push(candidate);
            }
        }
        let signatures: Vec<Value> = overloads
            .iter()
            .map(|overload| {
                let mut signature = json!({ "label": overload.signature_label, "parameters": overload.parameters.iter().map(|parameter| json!({ "label": parameter.label })).collect::<Vec<_>>() });
                if !overload.documentation.is_empty() {
                    signature["documentation"] = json!({ "kind": "markdown", "value": overload.documentation });
                }
                signature
            })
            .collect();
        let mut active_signature = overloads.iter().position(|overload| overload.parameters.len() > context.active_parameter).unwrap_or(0);
        if let Some(signature_context) = params.and_then(|params| params.get("context")) {
            if get_bool(signature_context, "isRetrigger").unwrap_or(false) {
                if let Some(previous) = signature_context.get("activeSignatureHelp").and_then(|help| get_i64(help, "activeSignature")) {
                    if previous >= 0 && (previous as usize) < overloads.len() {
                        active_signature = previous as usize;
                    }
                }
            }
        }
        let active_symbol = overloads[active_signature];
        let active = if active_symbol.parameters.is_empty() { 0 } else { context.active_parameter.min(active_symbol.parameters.len() - 1) };
        json!({ "signatures": signatures, "activeSignature": active_signature, "activeParameter": active })
    }

    pub fn handle_inlay_hint(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let Some(tokenized) = tokenize_text(&path, &text) else {
            return json!([]);
        };
        let (mut range_start, mut range_end) = (0, text.len());
        if let Some(range) = range_from_json(params.and_then(|params| params.get("range"))) {
            let Some((start, end)) = validated_byte_offsets_for_range(&index, range) else {
                return json!([]);
            };
            (range_start, range_end) = (start, end);
        }
        let tokens = &tokenized.tokens;
        let file = path.to_string_lossy();
        let mut hints = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            if token.kind != TokenKind::Punctuator || token.lexeme != "(" {
                continue;
            }
            let Some(callee_index) = tokens[..i].iter().rposition(|t| !matches!(t.kind, TokenKind::Newline | TokenKind::Eof)).filter(|&at| tokens[at].kind == TokenKind::Identifier) else {
                continue;
            };
            let Some(rparen) = matching_right_paren(tokens, i) else {
                continue;
            };
            let Some(symbol) = project.snapshot.language_service.resolved_symbol_at(&file, tokens[callee_index].span.start_offset).filter(|symbol| is_callable_symbol_kind(symbol.kind) && !symbol.parameters.is_empty()) else {
                continue;
            };
            let arg_starts = argument_start_token_indices(tokens, i, rparen);
            for arg_index in 0..arg_starts.len().min(symbol.parameters.len()) {
                let parameter = &symbol.parameters[arg_index];
                if parameter.name.is_empty() || parameter.name == "_" {
                    continue;
                }
                let arg_token = &tokens[arg_starts[arg_index]];
                if arg_token.span.start_offset < range_start || arg_token.span.start_offset >= range_end {
                    continue;
                }
                hints.push(json!({
                    "position": position_json(index.position_at(arg_token.span.start_offset)),
                    "label": format!("{}:", parameter.name),
                    "kind": 2,
                    "paddingRight": true,
                    "data": inlay_hint_data_for_parameter(&path, symbol, arg_index),
                }));
            }
        }
        Value::Array(hints)
    }

    pub fn handle_inlay_hint_resolve(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return Value::Null;
        };
        let mut hint = params.clone();
        if hint.get("tooltip").is_some() {
            return hint;
        }
        let data = hint.get("data").cloned();
        let field = |name: &str| data.as_ref().and_then(|data| get_str(data, name)).map(str::to_string);
        let (uri, symbol_uri, symbol_id) = (field("uri"), field("symbolUri"), field("symbolId"));
        let parameter_index = data.as_ref().and_then(|data| get_i64(data, "parameterIndex"));
        let hint_position = position_from_object(hint.get("position"));
        let (Some(uri), Some(symbol_uri), Some(symbol_id), Some(parameter_index)) = (uri, symbol_uri, symbol_id, parameter_index.filter(|index| *index >= 0)) else {
            return hint;
        };
        let (Some(path), Some(symbol_path), Some(hint_position)) = (file_uri_to_path(&uri), file_uri_to_path(&symbol_uri), hint_position) else {
            return hint;
        };
        if !self.has_text(&path) {
            return hint;
        }
        if validated_byte_offset_at(&LineIndex::new(&self.text_for_path(&path)), hint_position).is_none() {
            return hint;
        }
        let Some(project) = self.snapshot_for_path(&path) else {
            return hint;
        };
        let index = parameter_index as usize;
        let Some(symbol) = project
            .snapshot
            .language_service
            .symbol_by_id(&symbol_id)
            .filter(|symbol| path_key(Path::new(&*symbol.selection_range.file)) == path_key(&symbol_path) && index < symbol.parameters.len())
        else {
            return hint;
        };
        hint["tooltip"] = inlay_hint_tooltip_for_parameter(symbol, &symbol.parameters[index]);
        hint
    }
}
