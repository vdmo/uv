//! The call under the cursor, and the items of type and call hierarchies.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use uv_analysis::language_service::{LanguageSymbolInfo, LanguageSymbolKind};
use uv_analysis::typing::types::{applied_type_path, TypeNode, TypeRef};
use uv_core::span::Span;
use uv_source::ast::{ASTItem, ClassDecl};
use uv_source::lexer::token::{Token, TokenKind};
use uv_tooling::analysis::AnalysisSnapshot;
use uv_tooling::line_index::LineIndex;
use uv_tooling::uri::{file_uri_to_path, path_key, path_to_file_uri};

use crate::jsonx::{get_str, range_json};
use crate::navigation::{is_callable_symbol_kind, is_type_hierarchy_symbol_kind, is_type_symbol_kind, span_contains};

pub fn symbol_kind_to_lsp(kind: LanguageSymbolKind) -> i64 {
    use LanguageSymbolKind as K;
    match kind {
        K::Module => 2,
        K::Function => 12,
        K::Method => 6,
        K::Variable | K::Parameter => 13,
        K::Constant => 14,
        K::Field => 8,
        K::Record => 23,
        K::Enum => 10,
        K::EnumMember => 22,
        K::Modal => 5,
        K::Class => 11,
        K::TypeAlias => 5,
        K::State => 7,
    }
}

fn previous_significant_token(tokens: &[Token], mut index: usize) -> Option<usize> {
    while index > 0 {
        index -= 1;
        if !matches!(tokens[index].kind, TokenKind::Newline | TokenKind::Eof) {
            return Some(index);
        }
    }
    None
}

fn callee_token_index(tokens: &[Token], lparen_index: usize) -> Option<usize> {
    let previous = previous_significant_token(tokens, lparen_index)?;
    (tokens[previous].kind == TokenKind::Identifier).then_some(previous)
}

#[derive(Debug, Clone, Copy)]
pub struct CallContext {
    pub callee_index: usize,
    pub active_parameter: usize,
}

fn active_parameter_index(tokens: &[Token], lparen_index: usize, cursor_offset: usize) -> usize {
    let mut active = 0;
    let (mut paren, mut bracket, mut brace) = (0i32, 0i32, 0i32);
    for token in &tokens[lparen_index + 1..] {
        if token.span.start_offset >= cursor_offset {
            break;
        }
        if token.kind != TokenKind::Punctuator {
            continue;
        }
        match token.lexeme.as_str() {
            "(" => paren += 1,
            ")" => {
                if paren == 0 {
                    break;
                }
                paren -= 1;
            }
            "[" => bracket += 1,
            "]" => bracket = (bracket - 1).max(0),
            "{" => brace += 1,
            "}" => brace = (brace - 1).max(0),
            "," if paren == 0 && bracket == 0 && brace == 0 => active += 1,
            _ => {}
        }
    }
    active
}

pub fn call_context_at(tokens: &[Token], cursor_offset: usize) -> Option<CallContext> {
    let mut lparen_stack: Vec<usize> = Vec::new();
    let mut best: Option<CallContext> = None;
    for (i, token) in tokens.iter().enumerate() {
        if token.span.start_offset > cursor_offset {
            break;
        }
        if token.kind != TokenKind::Punctuator {
            continue;
        }
        if token.lexeme == "(" {
            lparen_stack.push(i);
        } else if token.lexeme == ")" {
            if let Some(lparen) = lparen_stack.pop() {
                if token.span.start_offset >= cursor_offset {
                    if let Some(callee) = callee_token_index(tokens, lparen) {
                        best = Some(CallContext { callee_index: callee, active_parameter: active_parameter_index(tokens, lparen, cursor_offset) });
                    }
                    break;
                }
            }
        }
    }
    for &lparen in lparen_stack.iter().rev() {
        if let Some(callee) = callee_token_index(tokens, lparen) {
            best = Some(CallContext { callee_index: callee, active_parameter: active_parameter_index(tokens, lparen, cursor_offset) });
            break;
        }
    }
    best
}

pub fn matching_right_paren(tokens: &[Token], lparen_index: usize) -> Option<usize> {
    let mut depth = 0;
    for (i, token) in tokens.iter().enumerate().skip(lparen_index) {
        if token.kind != TokenKind::Punctuator {
            continue;
        }
        if token.lexeme == "(" {
            depth += 1;
        } else if token.lexeme == ")" {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

pub fn argument_start_token_indices(tokens: &[Token], lparen_index: usize, rparen_index: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let (mut paren, mut bracket, mut brace) = (0i32, 0i32, 0i32);
    let mut expect_argument = true;
    for (i, token) in tokens.iter().enumerate().take(rparen_index).skip(lparen_index + 1) {
        if matches!(token.kind, TokenKind::Newline | TokenKind::Eof) {
            continue;
        }
        if token.kind == TokenKind::Punctuator {
            if token.lexeme == "," && paren == 0 && bracket == 0 && brace == 0 {
                expect_argument = true;
                continue;
            }
            if expect_argument {
                out.push(i);
                expect_argument = false;
            }
            match token.lexeme.as_str() {
                "(" => paren += 1,
                ")" => paren = (paren - 1).max(0),
                "[" => bracket += 1,
                "]" => bracket = (bracket - 1).max(0),
                "{" => brace += 1,
                "}" => brace = (brace - 1).max(0),
                _ => {}
            }
            continue;
        }
        if expect_argument {
            out.push(i);
            expect_argument = false;
        }
    }
    out
}

pub struct CallHierarchyCallGroup<'a> {
    pub symbol: &'a LanguageSymbolInfo,
    pub ranges: Vec<Value>,
}

pub fn hierarchy_item_for_symbol(symbol: &LanguageSymbolInfo, text: &str) -> Value {
    let index = LineIndex::new(text);
    let uri = path_to_file_uri(Path::new(&*symbol.selection_range.file));
    let mut item = json!({
        "name": symbol.name,
        "kind": symbol_kind_to_lsp(symbol.kind),
        "uri": uri,
        "range": range_json(index.range_for(&symbol.range)),
        "selectionRange": range_json(index.range_for(&symbol.selection_range)),
        "data": { "symbolId": symbol.id, "uri": uri },
    });
    if !symbol.detail.is_empty() {
        item["detail"] = Value::String(symbol.detail.clone());
    } else if !symbol.qualified_name.is_empty() {
        item["detail"] = Value::String(symbol.qualified_name.clone());
    }
    item
}

pub fn hierarchy_item_symbol_id(params: Option<&Value>) -> Option<String> {
    get_str(params?.get("item")?.get("data")?, "symbolId").map(str::to_string)
}

pub fn hierarchy_item_path(params: Option<&Value>) -> Option<PathBuf> {
    let item = params?.get("item")?;
    let uri = item.get("data").and_then(|data| get_str(data, "uri")).or_else(|| get_str(item, "uri"))?;
    file_uri_to_path(uri)
}

pub fn hierarchy_item_path_matches_symbol(symbol: &LanguageSymbolInfo, path: &Path) -> bool {
    path_key(Path::new(&*symbol.selection_range.file)) == path_key(path)
}

pub fn class_symbol_id_for_path(class_path: &[String]) -> Option<String> {
    (!class_path.is_empty()).then(|| format!("class:{}", class_path.join("::")))
}

pub fn class_symbol_id_for_decl(module_path: &[String], name: &str) -> String {
    let mut class_path = module_path.to_vec();
    class_path.push(name.to_string());
    format!("class:{}", class_path.join("::"))
}

pub fn class_decl_by_symbol_id<'a>(snapshot: &'a AnalysisSnapshot, symbol_id: &str) -> Option<&'a ClassDecl> {
    snapshot.modules.iter().find_map(|module| {
        module.items.iter().find_map(|item| match item {
            ASTItem::ClassDecl(node) if class_symbol_id_for_decl(&module.path, &node.name) == symbol_id => Some(node),
            _ => None,
        })
    })
}

pub fn class_symbol_for_path<'a>(snapshot: &'a AnalysisSnapshot, class_path: &[String]) -> Option<&'a LanguageSymbolInfo> {
    let symbol = snapshot.language_service.symbol_by_id(&class_symbol_id_for_path(class_path)?)?;
    is_type_hierarchy_symbol_kind(symbol.kind).then_some(symbol)
}

fn type_symbol_id_for_path(path: &[String]) -> Option<String> {
    (!path.is_empty()).then(|| format!("type:{}", path.join("::")))
}

fn type_symbol_ids_for_type(ty: &TypeRef) -> Vec<String> {
    let Some(node) = ty.as_deref().map(|ty| &ty.node) else {
        return Vec::new();
    };
    match node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => type_symbol_ids_for_type(base),
        TypeNode::Opaque { class_path, .. } => class_symbol_id_for_path(class_path).into_iter().collect(),
        TypeNode::ModalState(modal) => {
            let mut ids = Vec::new();
            if !modal.path.is_empty() && !modal.state.is_empty() {
                ids.push(format!("member:{}::{}", modal.path.join("::"), modal.state));
            }
            ids.extend(type_symbol_id_for_path(&modal.path));
            ids
        }
        _ => ty.as_deref().and_then(applied_type_path).and_then(|path| type_symbol_id_for_path(path)).into_iter().collect(),
    }
}

pub fn type_definition_symbol_for_type<'a>(snapshot: &'a AnalysisSnapshot, ty: &TypeRef) -> Option<&'a LanguageSymbolInfo> {
    type_symbol_ids_for_type(ty).iter().filter_map(|id| snapshot.language_service.symbol_by_id(id)).find(|symbol| is_type_symbol_kind(symbol.kind))
}

pub fn enclosing_callable_symbol<'a>(snapshot: &'a AnalysisSnapshot, span: &Span) -> Option<&'a LanguageSymbolInfo> {
    let mut best: Option<(&LanguageSymbolInfo, usize)> = None;
    for symbol in snapshot.language_service.symbols_in_file(&span.file) {
        if !is_callable_symbol_kind(symbol.kind) || !span_contains(&symbol.range, span) {
            continue;
        }
        let width = symbol.range.end_offset - symbol.range.start_offset;
        if best.is_none_or(|(_, best_width)| width < best_width) {
            best = Some((symbol, width));
        }
    }
    best.map(|(symbol, _)| symbol)
}
