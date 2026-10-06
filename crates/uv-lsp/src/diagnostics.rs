//! The diagnostics of one file as the protocol carries them, with the text of each file
//! read once per request.

use std::collections::HashMap;
use std::path::Path;

use serde_json::{json, Value};
use uv_core::diagnostics::{Severity, SubDiagnosticKind};
use uv_core::source_load::load_source;
use uv_tooling::analysis::AnalysisSnapshot;
use uv_tooling::document_store::DocumentStore;
use uv_tooling::line_index::{LineIndex, LineRange};
use uv_tooling::uri::{path_key, path_to_file_uri};

use crate::jsonx::range_json;

pub type DiagnosticTextCache = HashMap<String, Option<String>>;

fn loaded_text(path: &Path, bytes: &[u8]) -> Option<String> {
    load_source(&path.to_string_lossy(), bytes).source.map(|source| source.text)
}

/// The text of a file: the open document's, or what is on disk.
pub fn text_for_path(documents: &DocumentStore, path: &Path) -> Option<String> {
    if let Some(overlay) = documents.find_by_path(path) {
        return Some(loaded_text(path, overlay.text_utf8.as_bytes()).unwrap_or_else(|| overlay.text_utf8.clone()));
    }
    let bytes = std::fs::read(path).ok()?;
    loaded_text(path, &bytes)
}

fn cached_text(documents: &DocumentStore, path: &Path, cache: &mut DiagnosticTextCache) -> Option<String> {
    let key = path_key(path);
    if let Some(existing) = cache.get(&key) {
        return existing.clone();
    }
    let text = text_for_path(documents, path);
    cache.insert(key, text.clone());
    text
}

fn severity_to_lsp(severity: Severity) -> i64 {
    match severity {
        Severity::Error | Severity::Panic => 1,
        Severity::Warning => 2,
        Severity::Info => 3,
        Severity::Note => 4,
    }
}

pub fn diagnostics_for_path(snapshot: &AnalysisSnapshot, documents: &DocumentStore, path: &Path, cache: &mut DiagnosticTextCache) -> Vec<Value> {
    let text = cached_text(documents, path, cache);
    let index = LineIndex::new(text.as_deref().unwrap_or(""));
    let key = path_key(path);
    let mut out = Vec::new();
    for diag in &snapshot.diagnostics {
        if diag.span.as_ref().is_some_and(|span| path_key(Path::new(&*span.file)) != key) {
            continue;
        }
        let mut item = json!({
            "range": match &diag.span { Some(span) => range_json(index.range_for(span)), None => range_json(LineRange::default()) },
            "severity": severity_to_lsp(diag.severity),
            "source": "ultraviolet",
            "message": diag.message,
        });
        if !diag.code.is_empty() {
            item["code"] = Value::String(diag.code.clone());
        }
        let mut related = Vec::new();
        let mut fixits = Vec::new();
        for child in &diag.children {
            let Some(span) = &child.span else {
                continue;
            };
            let child_path = Path::new(&*span.file);
            let child_text = cached_text(documents, child_path, cache);
            let child_index = LineIndex::new(child_text.as_deref().unwrap_or(""));
            if child.kind == SubDiagnosticKind::FixIt {
                let Some(fix_text) = &child.fix_text else {
                    continue;
                };
                fixits.push(json!({
                    "title": if child.message.is_empty() { "Apply fix" } else { child.message.as_str() },
                    "uri": path_to_file_uri(child_path),
                    "range": range_json(child_index.range_for(span)),
                    "newText": fix_text,
                }));
                continue;
            }
            related.push(json!({
                "location": { "uri": path_to_file_uri(child_path), "range": range_json(child_index.range_for(span)) },
                "message": child.message,
            }));
        }
        if !related.is_empty() {
            item["relatedInformation"] = Value::Array(related);
        }
        if !fixits.is_empty() {
            item["data"] = json!({ "fixits": fixits });
        }
        out.push(item);
    }
    out
}
