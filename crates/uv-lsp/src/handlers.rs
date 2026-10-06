//! The requests: initialization, navigation, symbols, completion, hierarchies, tokens.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use serde_json::{json, Value};
use uv_analysis::language_service::{language_symbol_kind_name, LanguageSymbolInfo};
use uv_analysis::typing::types::type_to_string;
use uv_core::span::Span;
use uv_tooling::analysis::type_at;
use uv_tooling::line_index::{LineIndex, LineRange};
use uv_tooling::uri::{file_uri_to_path, normalize_path, path_key, path_to_file_uri};

use crate::completion::*;
use crate::config::*;
use crate::hierarchy::*;
use crate::jsonx::{get_bool, get_str, position_param, range_json};
use crate::modules::*;
use crate::navigation::*;
use crate::semantic_tokens::{semantic_tokens_for_range, SEMANTIC_TOKEN_MODIFIERS, SEMANTIC_TOKEN_TYPES};
use crate::server::{ClientFlags, Server};
use crate::symbols::*;
use crate::text::word_at;

const SEMANTIC_TOKEN_RESULT_CACHE_LIMIT: usize = 128;

fn version_string() -> String {
    uvc_version()
}

fn uvc_version() -> String {
    format!("Ultraviolet {}", env!("CARGO_PKG_VERSION"))
}

impl Server {
    pub fn dispatch_request(&mut self, method: &str, params: Option<&Value>) -> Option<Value> {
        Some(match method {
            "textDocument/documentSymbol" => self.handle_document_symbol(params),
            "textDocument/hover" => self.handle_hover(params),
            "textDocument/declaration" => self.handle_declaration(params),
            "textDocument/definition" => self.handle_definition(params),
            "textDocument/implementation" => self.handle_resolved_symbol_location(params, false, self.flags().implementation_link_support),
            "textDocument/semanticTokens/full" => self.handle_semantic_tokens(params),
            "textDocument/semanticTokens/full/delta" => self.handle_semantic_tokens_delta(params),
            "textDocument/semanticTokens/range" => self.handle_semantic_tokens_range(params),
            "textDocument/typeDefinition" => self.handle_type_definition(params),
            "textDocument/prepareTypeHierarchy" => self.handle_prepare_type_hierarchy(params),
            "typeHierarchy/supertypes" => self.handle_type_hierarchy_supertypes(params),
            "typeHierarchy/subtypes" => self.handle_type_hierarchy_subtypes(params),
            "textDocument/moniker" => self.handle_moniker(params),
            "textDocument/documentLink" => self.handle_document_link(params),
            "documentLink/resolve" => self.handle_document_link_resolve(params),
            "workspace/symbol" => self.handle_workspace_symbol(params),
            "workspaceSymbol/resolve" => self.handle_workspace_symbol_resolve(params),
            "textDocument/documentHighlight" => self.handle_document_highlight(params),
            "textDocument/linkedEditingRange" => self.handle_linked_editing_range(params),
            "textDocument/references" => self.handle_references(params),
            "textDocument/completion" => self.handle_completion(params),
            "completionItem/resolve" => self.handle_completion_item_resolve(params),
            "textDocument/diagnostic" => self.handle_text_document_diagnostic(params),
            "workspace/diagnostic" => self.handle_workspace_diagnostic(params),
            "workspace/willCreateFiles" | "workspace/willRenameFiles" | "workspace/willDeleteFiles" => self.handle_will_change_files(params),
            "textDocument/codeAction" => self.handle_code_action(params),
            "codeAction/resolve" => self.handle_code_action_resolve(params),
            "textDocument/codeLens" => self.handle_code_lens(params),
            "codeLens/resolve" => self.handle_code_lens_resolve(params),
            "textDocument/prepareRename" => self.handle_prepare_rename(params),
            "textDocument/rename" => self.handle_rename(params),
            "textDocument/signatureHelp" => self.handle_signature_help(params),
            "textDocument/inlayHint" => self.handle_inlay_hint(params),
            "inlayHint/resolve" => self.handle_inlay_hint_resolve(params),
            "textDocument/foldingRange" => self.handle_folding_range(params),
            "textDocument/selectionRange" => self.handle_selection_range(params),
            "textDocument/willSaveWaitUntil" => self.handle_will_save_wait_until(params),
            "textDocument/formatting" => self.handle_formatting(params),
            "textDocument/rangeFormatting" => self.handle_range_formatting(params),
            "textDocument/onTypeFormatting" => self.handle_on_type_formatting(params),
            "textDocument/prepareCallHierarchy" => self.handle_prepare_call_hierarchy(params),
            "callHierarchy/incomingCalls" => self.handle_call_hierarchy_incoming_calls(params),
            "callHierarchy/outgoingCalls" => self.handle_call_hierarchy_outgoing_calls(params),
            _ => return None,
        })
    }

    pub fn handle_initialize(&mut self, params: Option<&Value>) -> Value {
        self.workspace_roots.clear();
        self.set_flags(ClientFlags {
            declaration_link_support: client_supports_location_links(params, "declaration"),
            definition_link_support: client_supports_location_links(params, "definition"),
            implementation_link_support: client_supports_location_links(params, "implementation"),
            type_definition_link_support: client_supports_location_links(params, "typeDefinition"),
            hierarchical_document_symbol_support: client_supports_hierarchical_document_symbols(params),
            workspace_edit_document_changes_support: client_supports_workspace_edit_document_changes(params),
            completion_snippet_support: client_supports_completion_snippets(params),
            watched_files_dynamic_registration: client_supports_watched_file_dynamic_registration(params),
            workspace_configuration_supported: client_supports_workspace_configuration(params),
            semantic_token_refresh_supported: client_supports_workspace_refresh(params, "semanticTokens"),
            inlay_hint_refresh_supported: client_supports_workspace_refresh(params, "inlayHint"),
            code_lens_refresh_supported: client_supports_workspace_refresh(params, "codeLens"),
            diagnostic_refresh_supported: client_supports_workspace_refresh(params, "diagnostics"),
        });
        if let Some(params) = params {
            for folder in params.get("workspaceFolders").and_then(Value::as_array).into_iter().flatten() {
                if let Some(path) = get_str(folder, "uri").and_then(file_uri_to_path) {
                    self.workspace_roots.push(normalize_path(&path));
                }
            }
            if self.workspace_roots.is_empty() {
                if let Some(path) = get_str(params, "rootUri").and_then(file_uri_to_path) {
                    self.workspace_roots.push(normalize_path(&path));
                }
            }
            if self.workspace_roots.is_empty() {
                if let Some(root_path) = get_str(params, "rootPath") {
                    self.workspace_roots.push(normalize_path(&PathBuf::from(root_path)));
                }
            }
        }
        if self.workspace_roots.is_empty() {
            self.workspace_roots.push(normalize_path(&std::env::current_dir().unwrap_or_default()));
        }
        json!({
            "capabilities": {
                "positionEncoding": "utf-16",
                "textDocumentSync": { "openClose": true, "change": 2, "willSaveWaitUntil": true, "save": { "includeText": true } },
                "hoverProvider": true,
                "declarationProvider": true,
                "definitionProvider": true,
                "implementationProvider": true,
                "typeDefinitionProvider": true,
                "typeHierarchyProvider": true,
                "monikerProvider": true,
                "documentSymbolProvider": true,
                "documentLinkProvider": { "resolveProvider": true },
                "workspaceSymbolProvider": { "resolveProvider": true },
                "documentHighlightProvider": true,
                "linkedEditingRangeProvider": true,
                "referencesProvider": true,
                "completionProvider": { "resolveProvider": true, "triggerCharacters": [".", ":", "~"] },
                "codeActionProvider": { "codeActionKinds": ["quickfix"], "resolveProvider": true },
                "codeLensProvider": { "resolveProvider": true },
                "diagnosticProvider": { "identifier": "ultraviolet", "interFileDependencies": true, "workspaceDiagnostics": true },
                "renameProvider": { "prepareProvider": true },
                "signatureHelpProvider": { "triggerCharacters": ["(", ","], "retriggerCharacters": [","] },
                "inlayHintProvider": { "resolveProvider": true },
                "foldingRangeProvider": true,
                "selectionRangeProvider": true,
                "documentFormattingProvider": true,
                "documentRangeFormattingProvider": true,
                "documentOnTypeFormattingProvider": { "firstTriggerCharacter": "\n", "moreTriggerCharacter": ["}"] },
                "callHierarchyProvider": true,
                "semanticTokensProvider": {
                    "legend": { "tokenTypes": SEMANTIC_TOKEN_TYPES, "tokenModifiers": SEMANTIC_TOKEN_MODIFIERS },
                    "full": { "delta": true },
                    "range": true,
                },
                "workspace": {
                    "workspaceFolders": { "supported": true, "changeNotifications": true },
                    "fileOperations": {
                        "willCreate": file_operation_registration_options(),
                        "willRename": file_operation_registration_options(),
                        "willDelete": file_operation_registration_options(),
                        "didCreate": file_operation_registration_options(),
                        "didRename": file_operation_registration_options(),
                        "didDelete": file_operation_registration_options(),
                    },
                },
            },
            "serverInfo": { "name": "uv-lsp", "version": version_string() },
        })
    }

    /// The text and its line index, and the offset of a request's position when valid.
    fn position_context(&self, params: Option<&Value>) -> Option<(PathBuf, String, LineIndex, usize)> {
        let path = self.path_from_text_document(params)?;
        let position = position_param(params)?;
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let offset = validated_byte_offset_at(&index, position)?;
        Some((path, text, index, offset))
    }

    fn handle_document_symbol(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let outline: Vec<&LanguageSymbolInfo> = project.snapshot.language_service.symbols_in_file(&path.to_string_lossy()).into_iter().filter(|symbol| symbol.include_in_outline).collect();
        if self.flags().hierarchical_document_symbol_support {
            return Value::Array(hierarchical_document_symbols(&outline, &index));
        }
        Value::Array(outline.iter().map(|symbol| symbol_information_for_symbol(symbol, &text)).collect())
    }

    fn handle_hover(&mut self, params: Option<&Value>) -> Value {
        let Some((path, text, index, offset)) = self.position_context(params) else {
            return Value::Null;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return Value::Null;
        };
        let file = path.to_string_lossy();
        let symbol = project.snapshot.language_service.resolved_symbol_at(&file, offset);
        let hover_type = match symbol.map(|symbol| symbol.r#type.clone()).filter(|ty| ty.is_some()) {
            Some(ty) => ty,
            None => type_at(&project.snapshot, &file, offset),
        };
        if symbol.is_none() && hover_type.is_none() {
            return self.module_path_hover(params).unwrap_or(Value::Null);
        }
        let reference = project.snapshot.language_service.reference_at(&file, offset);
        let word = word_at(&text, offset);
        let mut markdown = "```ultraviolet\n".to_string();
        match symbol {
            Some(symbol) => {
                markdown.push_str(language_symbol_kind_name(symbol.kind));
                markdown.push(' ');
                markdown.push_str(if symbol.signature_label.is_empty() { &symbol.qualified_name } else { &symbol.signature_label });
            }
            None => markdown.push_str("type"),
        }
        if hover_type.is_some() {
            markdown.push_str(": ");
            markdown.push_str(&type_to_string(&hover_type));
        }
        markdown.push_str("\n```");
        if let Some(symbol) = symbol.filter(|symbol| !symbol.documentation.is_empty()) {
            markdown.push('\n');
            markdown.push_str(&symbol.documentation);
        }
        let mut result = json!({ "contents": { "kind": "markdown", "value": markdown } });
        if let Some(reference) = reference {
            result["range"] = range_json(index.range_for(&reference.range));
        } else if let Some(word) = word {
            result["range"] = range_json(LineRange { start: index.position_at(word.start), end: index.position_at(word.end) });
        }
        result
    }

    fn module_navigation(&self, params: Option<&Value>) -> Option<(ModuleNavigationTarget, String, LineIndex)> {
        let path = self.path_from_text_document(params)?;
        let position = position_param(params)?;
        let project = self.snapshot_for_path(&path)?;
        project.snapshot.project.as_ref()?;
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let offset = validated_byte_offset_at(&index, position)?;
        let tokenized = tokenize_text(&path, &text)?;
        let target = module_navigation_target_at(&project.snapshot, &path, &tokenized.tokens, offset)?;
        Some((target, text, index))
    }

    fn module_path_hover(&self, params: Option<&Value>) -> Option<Value> {
        let (target, _, index) = self.module_navigation(params)?;
        let markdown = format!("```ultraviolet\nmodule {}\n```\n{}", target.module_path, path_to_file_uri(&target.target));
        Some(json!({ "contents": { "kind": "markdown", "value": markdown }, "range": range_json(index.range_for(&target.origin_span)) }))
    }

    fn module_path_location(&self, params: Option<&Value>, location_link_support: bool) -> Option<Value> {
        let (target, text, _) = self.module_navigation(params)?;
        let target_text = self.text_for_path(&target.target);
        if !location_link_support {
            return Some(module_target_location(&target.target, &target_text));
        }
        Some(json!([module_target_location_link(&target.target, &target_text, &text, &target.origin_span)]))
    }

    fn handle_declaration(&mut self, params: Option<&Value>) -> Value {
        let support = self.flags().declaration_link_support;
        self.module_path_location(params, support).unwrap_or_else(|| self.handle_resolved_symbol_location(params, false, support))
    }

    fn handle_definition(&mut self, params: Option<&Value>) -> Value {
        let support = self.flags().definition_link_support;
        self.module_path_location(params, support).unwrap_or_else(|| self.handle_resolved_symbol_location(params, false, support))
    }

    fn handle_resolved_symbol_location(&self, params: Option<&Value>, require_type_symbol: bool, location_link_support: bool) -> Value {
        let Some((path, text, _, offset)) = self.position_context(params) else {
            return Value::Null;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return Value::Null;
        };
        let file = path.to_string_lossy();
        let reference = project.snapshot.language_service.reference_at(&file, offset);
        let Some(symbol) = project.snapshot.language_service.resolved_symbol_at(&file, offset) else {
            return Value::Null;
        };
        if require_type_symbol && !is_type_symbol_kind(symbol.kind) {
            return Value::Null;
        }
        navigation_location_for_symbol(
            symbol,
            &self.text_for_path(std::path::Path::new(&*symbol.range.file)),
            &text,
            origin_span_for_position(&path, &text, offset, reference).as_ref(),
            location_link_support,
        )
    }

    fn handle_type_definition(&mut self, params: Option<&Value>) -> Value {
        let support = self.flags().type_definition_link_support;
        let direct = self.handle_resolved_symbol_location(params, true, support);
        if !direct.is_null() {
            return direct;
        }
        let Some((path, text, _, offset)) = self.position_context(params) else {
            return Value::Null;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return Value::Null;
        };
        let file = path.to_string_lossy();
        let ty = type_at(&project.snapshot, &file, offset);
        let Some(symbol) = type_definition_symbol_for_type(&project.snapshot, &ty) else {
            return Value::Null;
        };
        let reference = project.snapshot.language_service.reference_at(&file, offset);
        navigation_location_for_symbol(symbol, &self.text_for_path(std::path::Path::new(&*symbol.range.file)), &text, origin_span_for_position(&path, &text, offset, reference).as_ref(), support)
    }

    fn handle_moniker(&mut self, params: Option<&Value>) -> Value {
        let Some((path, _, _, offset)) = self.position_context(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        match project.snapshot.language_service.resolved_symbol_at(&path.to_string_lossy(), offset) {
            Some(symbol) if !(symbol.id.is_empty() && symbol.qualified_name.is_empty()) => json!([moniker_for_symbol(symbol)]),
            _ => json!([]),
        }
    }

    fn handle_document_link(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let Some(project_info) = project.snapshot.project.as_ref() else {
            return json!([]);
        };
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let tokenized = tokenize_text(&path, &text);
        let lookup = build_module_lookup(project_info);
        let key = path_key(&path);
        let mut links = Vec::new();
        for module in &project.snapshot.modules {
            for item in &module.items {
                let Some(item_path) = module_path_for_link_item(item) else {
                    continue;
                };
                let Some(span) = span_for_item(item) else {
                    continue;
                };
                if path_key(std::path::Path::new(&*span.file)) != key {
                    continue;
                }
                let Some(resolved) = uv_source::module_paths::resolve_import_module_path(&module.path, &lookup.names, &item_path) else {
                    continue;
                };
                let Some(target) = document_link_target_for_module(&lookup, &resolved) else {
                    continue;
                };
                let link_span: Span = tokenized.as_ref().and_then(|output| module_path_span_for_document_link(&output.tokens, span, &item_path)).unwrap_or_else(|| span.clone());
                links.push(document_link_for_span(&link_span, &index, &target, &resolved.join("::")));
            }
        }
        Value::Array(links)
    }

    fn handle_document_link_resolve(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return Value::Null;
        };
        let mut link = params.clone();
        let data = link.get("data").cloned();
        let field = |name: &str| data.as_ref().and_then(|data| get_str(data, name)).map(str::to_string);
        let (origin_uri, target_uri, module_path) = (field("originUri"), field("targetUri"), field("modulePath"));
        let origin_path = origin_uri.as_deref().and_then(file_uri_to_path);
        let target_path = target_uri.as_deref().and_then(file_uri_to_path);
        let range = range_from_json(link.get("range"));
        let (Some(origin_path), Some(target_path), Some(range), Some(module_path)) = (origin_path, target_path, range, module_path) else {
            return link;
        };
        let Some(project) = self.snapshot_for_path(&origin_path) else {
            return link;
        };
        let Some(project_info) = project.snapshot.project.as_ref() else {
            return link;
        };
        let origin_text = self.text_for_path(&origin_path);
        if validated_byte_offsets_for_range(&LineIndex::new(&origin_text), range).is_none() {
            return link;
        }
        let expected = document_link_target_for_module(&build_module_lookup(project_info), &split_module_path_string(&module_path));
        if expected.is_none_or(|expected| path_key(&expected) != path_key(&target_path)) {
            return link;
        }
        if link.get("target").is_none() {
            link["target"] = json!(target_uri);
        }
        if link.get("tooltip").is_none() {
            link["tooltip"] = json!(document_link_tooltip(&module_path));
        }
        link
    }

    fn handle_workspace_symbol(&mut self, params: Option<&Value>) -> Value {
        let query = params.and_then(|params| get_str(params, "query")).unwrap_or("");
        let mut results = Vec::new();
        for project in self.core.project_snapshots() {
            for symbol in project.snapshot.language_service.symbols() {
                if !symbol.include_in_workspace || (!name_matches_query(&symbol.name, query) && !name_matches_query(&symbol.qualified_name, query)) {
                    continue;
                }
                results.push(workspace_symbol_for_symbol(symbol, &self.text_for_path(std::path::Path::new(&*symbol.selection_range.file))));
            }
        }
        Value::Array(results)
    }

    fn handle_workspace_symbol_resolve(&mut self, params: Option<&Value>) -> Value {
        let Some(params) = params else {
            return Value::Null;
        };
        let Some(symbol_id) = params.get("data").and_then(|data| get_str(data, "symbolId")) else {
            return params.clone();
        };
        let path = workspace_symbol_path(Some(params));
        let project = path.as_ref().and_then(|path| self.snapshot_for_path(path));
        let (Some(path), Some(project)) = (path, project) else {
            return params.clone();
        };
        match project.snapshot.language_service.symbol_by_id(symbol_id) {
            Some(symbol) if symbol.include_in_workspace && path_key(std::path::Path::new(&*symbol.selection_range.file)) == path_key(&path) => {
                workspace_symbol_for_symbol(symbol, &self.text_for_path(std::path::Path::new(&*symbol.selection_range.file)))
            }
            _ => params.clone(),
        }
    }

    fn handle_document_highlight(&mut self, params: Option<&Value>) -> Value {
        let Some((path, _, index, offset)) = self.position_context(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let file = path.to_string_lossy();
        let reference = project.snapshot.language_service.reference_at(&file, offset);
        let Some(symbol) = project.snapshot.language_service.resolved_symbol_at(&file, offset) else {
            return json!([]);
        };
        let key = path_key(&path);
        let kind = |is_declaration: bool| if is_declaration { 3 } else { 2 };
        let mut highlights: Vec<Value> = project
            .snapshot
            .language_service
            .references_for_symbol(&symbol.id, true)
            .into_iter()
            .filter(|reference| path_key(std::path::Path::new(&*reference.range.file)) == key)
            .map(|reference| json!({ "range": range_json(index.range_for(&reference.range)), "kind": kind(reference.is_declaration) }))
            .collect();
        if highlights.is_empty() {
            if let Some(reference) = reference {
                highlights.push(json!({ "range": range_json(index.range_for(&reference.range)), "kind": kind(reference.is_declaration) }));
            }
        }
        Value::Array(highlights)
    }

    fn handle_linked_editing_range(&mut self, params: Option<&Value>) -> Value {
        let Some((path, text, index, offset)) = self.position_context(params) else {
            return Value::Null;
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return Value::Null;
        };
        let Some(symbol) = project.snapshot.language_service.resolved_symbol_at(&path.to_string_lossy(), offset).filter(|symbol| symbol.is_local) else {
            return Value::Null;
        };
        let key = path_key(&path);
        let ranges: Vec<Value> = project
            .snapshot
            .language_service
            .references_for_symbol(&symbol.id, true)
            .into_iter()
            .filter(|reference| path_key(std::path::Path::new(&*reference.range.file)) == key)
            .filter_map(|reference| rename_span_for_reference(&path, &text, symbol, reference))
            .map(|span| range_json(index.range_for(&span)))
            .collect();
        if ranges.len() < 2 {
            return Value::Null;
        }
        json!({ "ranges": ranges })
    }

    fn handle_references(&mut self, params: Option<&Value>) -> Value {
        let Some((path, _, _, offset)) = self.position_context(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let Some(symbol) = project.snapshot.language_service.resolved_symbol_at(&path.to_string_lossy(), offset) else {
            return json!([]);
        };
        let include_declaration = params.and_then(|params| params.get("context")).and_then(|context| get_bool(context, "includeDeclaration")).unwrap_or(true);
        Value::Array(self.reference_locations_for_symbol(symbol, include_declaration))
    }

    fn handle_completion(&mut self, params: Option<&Value>) -> Value {
        let empty = || json!({ "isIncomplete": false, "items": [] });
        let Some(path) = self.path_from_text_document(params) else {
            return empty();
        };
        let Some(position) = position_param(params) else {
            return empty();
        };
        let project = self.snapshot_for_path(&path);
        let text = self.text_for_path(&path);
        let index = LineIndex::new(&text);
        let Some(offset) = validated_byte_offset_at(&index, position) else {
            return empty();
        };
        let edit_range = completion_edit_range(&text, &index, position);
        let word = word_at(&text, offset).map(|word| word.text);
        let mut items: Vec<Value> = Vec::new();
        if let Some(project) = &project {
            if let Some(tokenized) = tokenize_text(&path, &text) {
                append_module_completion_items(&mut items, &project.snapshot, &path, &text, &index, &tokenized.tokens, offset);
            }
        }
        const KEYWORDS: [&str; 25] = [
            "public", "internal", "private", "procedure", "record", "class", "modal", "enum", "let", "var", "return", "move", "const", "shared", "unique", "if", "is", "else", "loop", "break",
            "continue", "unsafe", "extern", "using", "import",
        ];
        for (keyword_index, keyword) in KEYWORDS.iter().enumerate() {
            items.push(json!({
                "label": keyword, "kind": 14, "filterText": keyword,
                "sortText": completion_sort_text('9', keyword_index, keyword),
                "textEdit": text_edit(edit_range, keyword),
            }));
        }
        let mut items_incomplete = false;
        if let Some(project) = &project {
            const MAX_COMPLETION_SYMBOLS: usize = 500;
            let mut completion_symbols = project.snapshot.language_service.completion_symbols(&path.to_string_lossy(), offset);
            if completion_symbols.is_empty() {
                items_incomplete = true;
                completion_symbols = project.snapshot.language_service.symbols().iter().collect();
            }
            let mut seen = HashSet::new();
            let (mut symbol_ordinal, mut emitted) = (0, 0);
            let snippets = self.flags().completion_snippet_support;
            for symbol in completion_symbols {
                if !seen.insert(symbol.name.clone()) {
                    continue;
                }
                if emitted >= MAX_COMPLETION_SYMBOLS {
                    items_incomplete = true;
                    break;
                }
                emitted += 1;
                let mut item = json!({
                    "label": symbol.name,
                    "kind": completion_kind_for_symbol(symbol.kind),
                    "detail": completion_detail_for_symbol(symbol),
                    "data": completion_data_for_symbol(symbol),
                    "filterText": symbol.name,
                    "sortText": completion_sort_text(if symbol.is_local { '0' } else { '1' }, symbol_ordinal, &symbol.name),
                });
                if word.as_deref() == Some(symbol.name.as_str()) {
                    item["preselect"] = json!(true);
                }
                symbol_ordinal += 1;
                let mut new_text = symbol.name.clone();
                if snippets {
                    if let Some(snippet) = completion_snippet_for_symbol(symbol) {
                        item["insertText"] = json!(snippet);
                        item["insertTextFormat"] = json!(2);
                        new_text = snippet;
                    }
                }
                item["textEdit"] = text_edit(edit_range, &new_text);
                items.push(item);
            }
        }
        json!({ "isIncomplete": items_incomplete, "items": items })
    }

    // ---- hierarchies ----

    fn handle_prepare_call_hierarchy(&mut self, params: Option<&Value>) -> Value {
        let Some((path, _, _, offset)) = self.position_context(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        match project.snapshot.language_service.resolved_symbol_at(&path.to_string_lossy(), offset) {
            Some(symbol) if is_callable_symbol_kind(symbol.kind) => json!([hierarchy_item_for_symbol(symbol, &self.text_for_path(std::path::Path::new(&*symbol.range.file)))]),
            _ => json!([]),
        }
    }

    fn handle_call_hierarchy_incoming_calls(&mut self, params: Option<&Value>) -> Value {
        let (Some(symbol_id), Some(path)) = (hierarchy_item_symbol_id(params), hierarchy_item_path(params)) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let Some(target) = project.snapshot.language_service.symbol_by_id(&symbol_id).filter(|target| hierarchy_item_path_matches_symbol(target, &path) && is_callable_symbol_kind(target.kind)) else {
            return json!([]);
        };
        let mut calls_by_caller: BTreeMap<String, CallHierarchyCallGroup<'_>> = BTreeMap::new();
        for reference in project.snapshot.language_service.references_for_symbol(&target.id, false) {
            let Some(caller) = enclosing_callable_symbol(&project.snapshot, &reference.range) else {
                continue;
            };
            let file = std::path::Path::new(&*reference.range.file);
            let index = LineIndex::new(&self.text_for_path(file));
            let group = calls_by_caller.entry(caller.id.clone()).or_insert_with(|| CallHierarchyCallGroup { symbol: caller, ranges: Vec::new() });
            group.ranges.push(range_json(index.range_for(&reference.range)));
        }
        Value::Array(
            calls_by_caller
                .into_values()
                .map(|group| json!({ "from": hierarchy_item_for_symbol(group.symbol, &self.text_for_path(std::path::Path::new(&*group.symbol.range.file))), "fromRanges": group.ranges }))
                .collect(),
        )
    }

    fn handle_call_hierarchy_outgoing_calls(&mut self, params: Option<&Value>) -> Value {
        let (Some(symbol_id), Some(path)) = (hierarchy_item_symbol_id(params), hierarchy_item_path(params)) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let Some(caller) = project.snapshot.language_service.symbol_by_id(&symbol_id).filter(|caller| hierarchy_item_path_matches_symbol(caller, &path) && is_callable_symbol_kind(caller.kind)) else {
            return json!([]);
        };
        let caller_index = LineIndex::new(&self.text_for_path(std::path::Path::new(&*caller.range.file)));
        let mut calls_by_target: BTreeMap<String, CallHierarchyCallGroup<'_>> = BTreeMap::new();
        for reference in project.snapshot.language_service.references() {
            if reference.is_declaration || !span_contains(&caller.range, &reference.range) {
                continue;
            }
            let Some(target) = project.snapshot.language_service.symbol_by_id(&reference.symbol_id).filter(|target| is_callable_symbol_kind(target.kind)) else {
                continue;
            };
            let group = calls_by_target.entry(target.id.clone()).or_insert_with(|| CallHierarchyCallGroup { symbol: target, ranges: Vec::new() });
            group.ranges.push(range_json(caller_index.range_for(&reference.range)));
        }
        Value::Array(
            calls_by_target
                .into_values()
                .map(|group| json!({ "to": hierarchy_item_for_symbol(group.symbol, &self.text_for_path(std::path::Path::new(&*group.symbol.range.file))), "fromRanges": group.ranges }))
                .collect(),
        )
    }

    fn handle_prepare_type_hierarchy(&mut self, params: Option<&Value>) -> Value {
        let Some((path, _, _, offset)) = self.position_context(params) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        match project.snapshot.language_service.resolved_symbol_at(&path.to_string_lossy(), offset) {
            Some(symbol) if is_type_hierarchy_symbol_kind(symbol.kind) => json!([hierarchy_item_for_symbol(symbol, &self.text_for_path(std::path::Path::new(&*symbol.range.file)))]),
            _ => json!([]),
        }
    }

    fn handle_type_hierarchy_supertypes(&mut self, params: Option<&Value>) -> Value {
        let (Some(symbol_id), Some(path)) = (hierarchy_item_symbol_id(params), hierarchy_item_path(params)) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let Some(symbol) = project.snapshot.language_service.symbol_by_id(&symbol_id).filter(|symbol| hierarchy_item_path_matches_symbol(symbol, &path) && is_type_hierarchy_symbol_kind(symbol.kind)) else {
            return json!([]);
        };
        let Some(class_decl) = class_decl_by_symbol_id(&project.snapshot, &symbol.id) else {
            return json!([]);
        };
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for super_path in &class_decl.supers {
            let Some(super_symbol) = class_symbol_for_path(&project.snapshot, super_path) else {
                continue;
            };
            if seen.insert(super_symbol.id.clone()) {
                result.push(hierarchy_item_for_symbol(super_symbol, &self.text_for_path(std::path::Path::new(&*super_symbol.range.file))));
            }
        }
        Value::Array(result)
    }

    fn handle_type_hierarchy_subtypes(&mut self, params: Option<&Value>) -> Value {
        let (Some(symbol_id), Some(path)) = (hierarchy_item_symbol_id(params), hierarchy_item_path(params)) else {
            return json!([]);
        };
        let Some(project) = self.snapshot_for_path(&path) else {
            return json!([]);
        };
        let Some(symbol) = project.snapshot.language_service.symbol_by_id(&symbol_id).filter(|symbol| hierarchy_item_path_matches_symbol(symbol, &path) && is_type_hierarchy_symbol_kind(symbol.kind)) else {
            return json!([]);
        };
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for module in &project.snapshot.modules {
            for item in &module.items {
                let uv_source::ast::ASTItem::ClassDecl(node) = item else {
                    continue;
                };
                if !node.supers.iter().any(|super_path| class_symbol_id_for_path(super_path).as_deref() == Some(symbol.id.as_str())) {
                    continue;
                }
                let Some(subtype) = project.snapshot.language_service.symbol_by_id(&class_symbol_id_for_decl(&module.path, &node.name)) else {
                    continue;
                };
                if seen.insert(subtype.id.clone()) {
                    result.push(hierarchy_item_for_symbol(subtype, &self.text_for_path(std::path::Path::new(&*subtype.range.file))));
                }
            }
        }
        Value::Array(result)
    }

    // ---- semantic tokens ----

    fn compute_semantic_tokens_result(&self, path: &std::path::Path) -> Value {
        let project = self.snapshot_for_path(path);
        let mut result = semantic_tokens_for_range(path, &self.text_for_path(path), project.as_ref().map(|project| &project.snapshot), None);
        if let Some(data) = result.get("data").and_then(Value::as_array) {
            let id = semantic_tokens_result_id_for_path(path, data);
            result["resultId"] = json!(id);
        }
        result
    }

    fn store_semantic_token_result(&mut self, result_id: &str, data: &[Value]) {
        self.semantic_token_result_id_order.retain(|id| id != result_id);
        self.semantic_token_result_id_order.push_back(result_id.to_string());
        self.semantic_token_data_by_result_id.insert(result_id.to_string(), semantic_token_data_values(data));
        while self.semantic_token_result_id_order.len() > SEMANTIC_TOKEN_RESULT_CACHE_LIMIT {
            if let Some(oldest) = self.semantic_token_result_id_order.pop_front() {
                self.semantic_token_data_by_result_id.remove(&oldest);
            }
        }
    }

    fn handle_semantic_tokens(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!({ "data": [] });
        };
        let result = self.compute_semantic_tokens_result(&path);
        if let (Some(data), Some(id)) = (result.get("data").and_then(Value::as_array), get_str(&result, "resultId")) {
            let (data, id) = (data.clone(), id.to_string());
            self.store_semantic_token_result(&id, &data);
        }
        result
    }

    fn handle_semantic_tokens_delta(&mut self, params: Option<&Value>) -> Value {
        let Some(path) = self.path_from_text_document(params) else {
            return json!({ "edits": [] });
        };
        let previous_id = params.and_then(|params| get_str(params, "previousResultId")).map(str::to_string);
        let result = self.compute_semantic_tokens_result(&path);
        let (Some(data), Some(current_id)) = (result.get("data").and_then(Value::as_array).cloned(), get_str(&result, "resultId").map(str::to_string)) else {
            return result;
        };
        let previous = previous_id.and_then(|id| self.semantic_token_data_by_result_id.get(&id).cloned());
        let current = semantic_token_data_values(&data);
        self.store_semantic_token_result(&current_id, &data);
        match previous {
            None => result,
            Some(previous) => json!({ "resultId": current_id, "edits": semantic_token_edits(&previous, &current) }),
        }
    }

    fn handle_semantic_tokens_range(&mut self, params: Option<&Value>) -> Value {
        let path = self.path_from_text_document(params);
        let range = range_from_json(params.and_then(|params| params.get("range")));
        let (Some(path), Some(range)) = (path, range) else {
            return json!({ "data": [] });
        };
        let project = self.snapshot_for_path(&path);
        let text = self.text_for_path(&path);
        if validated_byte_offsets_for_range(&LineIndex::new(&text), range).is_none() {
            return json!({ "data": [] });
        }
        semantic_tokens_for_range(&path, &text, project.as_ref().map(|project| &project.snapshot), Some(range))
    }
}
