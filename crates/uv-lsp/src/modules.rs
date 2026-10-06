//! Modules as the editor navigates them (`using` and `import` paths to files), the
//! quick-fix and result-id helpers, and the semantic-token delta.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use uv_core::span::Span;
use uv_project::module_discovery::compilation_unit;
use uv_project::project::Project;
use uv_source::ast::{self, ASTItem};
use uv_tooling::line_index::{LineIndex, LinePosition, LineRange};
use uv_tooling::uri::{normalize_path, path_key, path_to_file_uri};

use crate::jsonx::range_json;

pub struct ModuleLookup {
    pub names: HashSet<String>,
    pub dirs_by_name: HashMap<String, String>,
}

pub struct ModuleNavigationTarget {
    pub target: PathBuf,
    pub origin_span: Span,
    pub module_path: String,
}

pub struct ModuleCompletionContext {
    pub directive: String,
    pub current_module: Vec<String>,
    pub completed_path: Vec<String>,
    pub query: String,
    pub edit_range: LineRange,
}

pub fn build_module_lookup(project: &Project) -> ModuleLookup {
    let mut lookup = ModuleLookup { names: HashSet::new(), dirs_by_name: HashMap::new() };
    for module in &project.modules {
        lookup.names.insert(module.path.clone());
        lookup.dirs_by_name.entry(module.path.clone()).or_insert_with(|| module.dir.clone());
    }
    lookup
}

pub fn module_path_for_link_item(item: &ASTItem) -> Option<Vec<String>> {
    match item {
        ASTItem::UsingDecl(decl) => Some(match &decl.clause {
            ast::UsingClause::UsingItem(clause) => clause.module_path.clone(),
            ast::UsingClause::UsingList(clause) => clause.module_path.clone(),
            ast::UsingClause::UsingWildcard(clause) => clause.module_path.clone(),
        }),
        ASTItem::ImportDecl(decl) => Some(decl.path.clone()),
        _ => None,
    }
}

pub fn span_for_item(item: &ASTItem) -> Option<&Span> {
    Some(match item {
        ASTItem::UsingDecl(node) => &node.span,
        ASTItem::ImportDecl(node) => &node.span,
        ASTItem::ExternBlock(node) => &node.span,
        ASTItem::StaticDecl(node) => &node.span,
        ASTItem::ProcedureDecl(node) => &node.span,
        ASTItem::ComptimeProcedureDecl(node) => &node.span,
        ASTItem::RecordDecl(node) => &node.span,
        ASTItem::EnumDecl(node) => &node.span,
        ASTItem::ModalDecl(node) => &node.span,
        ASTItem::ClassDecl(node) => &node.span,
        ASTItem::TypeAliasDecl(node) => &node.span,
        ASTItem::DeriveTargetDecl(node) => &node.span,
        ASTItem::ErrorItem(node) => &node.span,
    })
}

pub fn first_source_file_in_module(module_dir: &str) -> Option<PathBuf> {
    let mut unit = compilation_unit(module_dir);
    unit.files.sort();
    unit.files.first().map(|file| normalize_path(Path::new(file)))
}

pub fn document_link_target_for_module(lookup: &ModuleLookup, module_path: &[String]) -> Option<PathBuf> {
    first_source_file_in_module(lookup.dirs_by_name.get(&module_path.join("::"))?)
}

pub fn document_link_for_span(span: &Span, index: &LineIndex, target: &Path, module_path: &str) -> Value {
    json!({
        "range": range_json(index.range_for(span)),
        "target": path_to_file_uri(target),
        "data": { "originUri": path_to_file_uri(Path::new(&*span.file)), "targetUri": path_to_file_uri(target), "modulePath": module_path },
    })
}

pub fn document_link_tooltip(module_path: &str) -> String {
    format!("Open Ultraviolet module {module_path}")
}

pub fn module_target_location(target: &Path, target_text: &str) -> Value {
    let index = LineIndex::new(target_text);
    json!({
        "uri": path_to_file_uri(target),
        "range": range_json(LineRange { start: LinePosition::default(), end: index.position_at(target_text.len()) }),
    })
}

pub fn module_target_location_link(target: &Path, target_text: &str, origin_text: &str, origin_span: &Span) -> Value {
    let target_index = LineIndex::new(target_text);
    let start = LinePosition::default();
    json!({
        "targetUri": path_to_file_uri(target),
        "targetRange": range_json(LineRange { start, end: target_index.position_at(target_text.len()) }),
        "targetSelectionRange": range_json(LineRange { start, end: start }),
        "originSelectionRange": range_json(LineIndex::new(origin_text).range_for(origin_span)),
    })
}

pub fn path_is_within_root(path: &Path, root: &Path) -> bool {
    let key = path_key(path);
    let mut root_key = path_key(root);
    if key == root_key {
        return true;
    }
    if !root_key.is_empty() && !root_key.ends_with('/') {
        root_key.push('/');
    }
    key.starts_with(&root_key)
}

pub fn name_matches_query(name: &str, query: &str) -> bool {
    query.is_empty() || name.to_ascii_lowercase().contains(&query.to_ascii_lowercase())
}

pub fn completion_sort_text(bucket: char, ordinal: usize, label: &str) -> String {
    format!("{bucket}:{ordinal:08}:{}", label.to_ascii_lowercase())
}

pub fn code_action_context_allows_quickfix(context: Option<&Value>) -> bool {
    let Some(only) = context.and_then(|context| context.get("only")).and_then(|only| only.as_array()) else {
        return true;
    };
    only.is_empty() || only.iter().any(|kind| kind.as_str().is_some_and(|kind| kind.is_empty() || kind == "quickfix"))
}

pub fn diagnostic_at_start(message: &str) -> Value {
    json!({ "range": range_json(LineRange::default()), "severity": 1, "source": "ultraviolet", "message": message })
}

fn result_id_for(kind: &str, path: &Path, values: &[Value]) -> String {
    let mut hash: u64 = 1469598103934665603;
    let mut mix = |text: &str| {
        for byte in text.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(1099511628211);
        }
    };
    mix(kind);
    mix("\n");
    mix(&path_key(path));
    mix("\n");
    for value in values {
        mix(&value.to_string());
        mix("\x1e");
    }
    format!("{hash:016x}")
}

pub fn diagnostic_result_id_for_path(path: &Path, diagnostics: &[Value]) -> String {
    result_id_for("diagnostics", path, diagnostics)
}

pub fn semantic_tokens_result_id_for_path(path: &Path, data: &[Value]) -> String {
    result_id_for("semantic-tokens", path, data)
}

pub fn full_diagnostic_report(diagnostics: Vec<Value>, result_id: Option<String>) -> Value {
    let mut report = json!({ "kind": "full", "items": diagnostics });
    if let Some(id) = result_id {
        report["resultId"] = Value::String(id);
    }
    report
}

pub fn unchanged_diagnostic_report(result_id: &str) -> Value {
    json!({ "kind": "unchanged", "resultId": result_id })
}

pub fn previous_workspace_result_ids(params: Option<&Value>) -> HashMap<String, String> {
    let mut ids = HashMap::new();
    for item in params.and_then(|params| params.get("previousResultIds")).and_then(|ids| ids.as_array()).into_iter().flatten() {
        if let (Some(uri), Some(value)) = (item.get("uri").and_then(|v| v.as_str()), item.get("value").and_then(|v| v.as_str())) {
            ids.insert(uri.to_string(), value.to_string());
        }
    }
    ids
}

pub fn semantic_token_data_values(data: &[Value]) -> Vec<i64> {
    data.iter().map(|value| value.as_i64().unwrap_or(0)).collect()
}

/// The one edit that turns the previous token data into the current.
pub fn semantic_token_edits(previous: &[i64], current: &[i64]) -> Vec<Value> {
    let shared = previous.len().min(current.len());
    let mut prefix = 0;
    while prefix < shared && previous[prefix] == current[prefix] {
        prefix += 1;
    }
    let mut suffix = 0;
    let remaining = shared - prefix;
    while suffix < remaining && previous[previous.len() - 1 - suffix] == current[current.len() - 1 - suffix] {
        suffix += 1;
    }
    let delete_count = previous.len() - prefix - suffix;
    let insert_end = current.len() - suffix;
    if delete_count == 0 && prefix == insert_end {
        return Vec::new();
    }
    vec![json!({ "start": prefix as i64, "deleteCount": delete_count as i64, "data": current[prefix..insert_end].to_vec() })]
}

fn file_operation_filter(glob: &str) -> Value {
    json!({ "scheme": "file", "pattern": { "glob": glob, "matches": "file" } })
}

pub fn file_operation_registration_options() -> Value {
    json!({ "filters": [file_operation_filter("**/*.uv"), file_operation_filter("**/Ultraviolet.toml")] })
}

pub fn watched_file_registration_options() -> Value {
    json!({ "watchers": [{ "globPattern": "**/*.uv", "kind": 7 }, { "globPattern": "**/Ultraviolet.toml", "kind": 7 }] })
}
