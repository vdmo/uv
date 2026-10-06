//! What the client says about itself and its settings, and the locations the server
//! answers navigation requests with.

use serde_json::{json, Value};
use uv_analysis::language_service::{LanguageReference, LanguageSymbolInfo};
use uv_core::span::Span;
use uv_project::target_profile::{parse_target_profile, TargetProfile};
use uv_tooling::line_index::LineIndex;
use uv_tooling::uri::path_to_file_uri;

use crate::jsonx::{get_bool, get_str, nested, range_json};
use crate::text::word_at;

#[derive(Debug, Clone, Default)]
pub struct TargetProfileSetting {
    pub present: bool,
    pub valid: bool,
    pub value: Option<TargetProfile>,
    pub raw_value: String,
}

fn capture_target_profile_string(setting: &mut TargetProfileSetting, value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    setting.present = true;
    setting.raw_value = value.to_string();
    if value.is_empty() {
        setting.valid = true;
        setting.value = None;
        return true;
    }
    match parse_target_profile(value) {
        Some(profile) => {
            setting.valid = true;
            setting.value = Some(profile);
        }
        None => {
            setting.valid = false;
            setting.value = None;
        }
    }
    true
}

pub fn target_profile_setting_from_settings(settings: Option<&Value>) -> TargetProfileSetting {
    let mut setting = TargetProfileSetting { valid: true, ..Default::default() };
    let Some(settings) = settings.filter(|settings| settings.is_object()) else {
        return setting;
    };
    if capture_target_profile_string(&mut setting, get_str(settings, "targetProfile")) {
        return setting;
    }
    if let Some(language_server) = settings.get("languageServer").filter(|value| value.is_object()) {
        if capture_target_profile_string(&mut setting, get_str(language_server, "targetProfile")) {
            return setting;
        }
    }
    if let Some(ultraviolet) = settings.get("ultraviolet").filter(|value| value.is_object()) {
        if capture_target_profile_string(&mut setting, get_str(ultraviolet, "targetProfile")) {
            return setting;
        }
        if let Some(language_server) = ultraviolet.get("languageServer").filter(|value| value.is_object()) {
            if capture_target_profile_string(&mut setting, get_str(language_server, "targetProfile")) {
                return setting;
            }
        }
    }
    setting
}

pub fn target_profile_setting_from_configuration(params: Option<&Value>) -> TargetProfileSetting {
    target_profile_setting_from_settings(params.and_then(|params| params.get("settings")))
}

pub fn host_default_target_profile() -> TargetProfile {
    if cfg!(target_os = "windows") {
        TargetProfile::X86_64Win64
    } else if cfg!(target_os = "macos") {
        TargetProfile::AArch64Darwin
    } else if cfg!(target_arch = "aarch64") {
        TargetProfile::AArch64AAPCS64
    } else {
        TargetProfile::X86_64SysV
    }
}

pub fn target_profile_setting_from_configuration_response(message: &Value) -> TargetProfileSetting {
    if let Some(result) = message.get("result").and_then(|result| result.as_array()) {
        return match result.first() {
            Some(first) => target_profile_setting_from_settings(Some(first)),
            None => TargetProfileSetting { valid: true, ..Default::default() },
        };
    }
    target_profile_setting_from_settings(message.get("result"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

pub fn is_opening_delimiter(lexeme: &str) -> bool {
    matches!(lexeme, "{" | "[" | "(")
}

pub fn is_closing_delimiter(lexeme: &str) -> bool {
    matches!(lexeme, "}" | "]" | ")")
}

fn delimiters_match(opening: &str, closing: &str) -> bool {
    matches!((opening, closing), ("{", "}") | ("[", "]") | ("(", ")"))
}

pub fn contains_offset(range: ByteRange, offset: usize) -> bool {
    range.start <= offset && offset < range.end
}

pub fn add_distinct_range(ranges: &mut Vec<ByteRange>, range: ByteRange) {
    if range.end > range.start && !ranges.contains(&range) {
        ranges.push(range);
    }
}

/// The span of every matched pair of brackets, in the order the pairs close.
pub fn delimited_byte_ranges(tokens: &[uv_source::lexer::token::Token]) -> Vec<ByteRange> {
    use uv_source::lexer::token::TokenKind;
    let mut stack: Vec<&uv_source::lexer::token::Token> = Vec::new();
    let mut ranges = Vec::new();
    for token in tokens.iter().filter(|token| token.kind == TokenKind::Punctuator) {
        if is_opening_delimiter(&token.lexeme) {
            stack.push(token);
        } else if is_closing_delimiter(&token.lexeme) {
            if let Some(at) = stack.iter().rposition(|opening| delimiters_match(&opening.lexeme, &token.lexeme)) {
                let opening = stack.remove(at);
                ranges.push(ByteRange { start: opening.span.start_offset, end: token.span.end_offset });
            }
        }
    }
    ranges
}

pub fn location_for_symbol(symbol: &LanguageSymbolInfo, text: &str) -> Value {
    let index = LineIndex::new(text);
    json!({ "uri": path_to_file_uri(std::path::Path::new(&*symbol.selection_range.file)), "range": range_json(index.range_for(&symbol.selection_range)) })
}

pub fn location_link_for_symbol(symbol: &LanguageSymbolInfo, target_text: &str, origin_text: &str, origin_span: Option<&Span>) -> Value {
    let target_index = LineIndex::new(target_text);
    let mut link = json!({
        "targetUri": path_to_file_uri(std::path::Path::new(&*symbol.range.file)),
        "targetRange": range_json(target_index.range_for(&symbol.range)),
        "targetSelectionRange": range_json(target_index.range_for(&symbol.selection_range)),
    });
    if let Some(origin) = origin_span {
        link["originSelectionRange"] = range_json(LineIndex::new(origin_text).range_for(origin));
    }
    link
}

pub fn navigation_location_for_symbol(symbol: &LanguageSymbolInfo, target_text: &str, origin_text: &str, origin_span: Option<&Span>, location_link_support: bool) -> Value {
    if !location_link_support {
        return location_for_symbol(symbol, target_text);
    }
    json!([location_link_for_symbol(symbol, target_text, origin_text, origin_span)])
}

pub fn origin_span_for_position(path: &std::path::Path, text: &str, offset: usize, reference: Option<&LanguageReference>) -> Option<Span> {
    if let Some(reference) = reference {
        return Some(reference.range.clone());
    }
    let word = word_at(text, offset)?;
    Some(Span { file: path.to_string_lossy().as_ref().into(), start_offset: word.start, end_offset: word.end, ..Default::default() })
}

fn flag(params: Option<&Value>, path: &[&str], name: &str) -> bool {
    nested(params, path).and_then(|object| get_bool(object, name)).unwrap_or(false)
}

pub fn client_supports_location_links(params: Option<&Value>, feature: &str) -> bool {
    flag(params, &["capabilities", "textDocument", feature], "linkSupport")
}

pub fn client_supports_hierarchical_document_symbols(params: Option<&Value>) -> bool {
    flag(params, &["capabilities", "textDocument", "documentSymbol"], "hierarchicalDocumentSymbolSupport")
}

pub fn client_supports_completion_snippets(params: Option<&Value>) -> bool {
    flag(params, &["capabilities", "textDocument", "completion", "completionItem"], "snippetSupport")
}

pub fn client_supports_watched_file_dynamic_registration(params: Option<&Value>) -> bool {
    flag(params, &["capabilities", "workspace", "didChangeWatchedFiles"], "dynamicRegistration")
}

pub fn client_supports_workspace_configuration(params: Option<&Value>) -> bool {
    flag(params, &["capabilities", "workspace"], "configuration")
}

pub fn client_supports_workspace_edit_document_changes(params: Option<&Value>) -> bool {
    flag(params, &["capabilities", "workspace", "workspaceEdit"], "documentChanges")
}

pub fn client_supports_workspace_refresh(params: Option<&Value>, capability: &str) -> bool {
    flag(params, &["capabilities", "workspace", capability], "refreshSupport")
}

pub fn client_allows_utf16_position_encoding(params: Option<&Value>) -> bool {
    let encodings = params.and_then(|params| params.get("capabilities")).and_then(|c| c.get("general")).and_then(|g| g.get("positionEncodings")).and_then(|e| e.as_array());
    match encodings {
        None => true,
        Some(list) => list.iter().any(|encoding| encoding.as_str() == Some("utf-16")),
    }
}
