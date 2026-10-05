//! The `files` capability: a snapshot of the project directory taken once before
//! compile-time execution, and the methods that read from it.

use std::collections::HashMap;
use std::io::ErrorKind;
use std::rc::Rc;

use uv_core::path::{canon, join, join_comp, normalize, path_comps, resolve};
use uv_core::source_load::decode;
use uv_core::span::Span;
use uv_core::{spec_rule, spec_rule_at};
use uv_source::ast::{ExprNode, MethodCallExpr};

use crate::eval::eval_expr;
use crate::value::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectFileSnapshotKind {
    File,
    Directory,
    #[default]
    Other,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectFileSnapshotEntry {
    pub kind: ProjectFileSnapshotKind,
    pub exists_error: Option<String>,
    pub read_error: Option<String>,
    pub read_bytes_error: Option<String>,
    pub list_dir_error: Option<String>,
    pub bytes: Vec<u8>,
    pub dir_entries: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectFileSnapshot {
    pub root_text: String,
    pub entries: HashMap<String, ProjectFileSnapshotEntry>,
    pub captured_file_count: u64,
    pub captured_directory_count: u64,
    pub captured_byte_count: u64,
}

fn make_outcome(variant: &str, value: CtValue) -> CtValue {
    CtValue::Enum(Rc::new(CtEnum {
        path: vec!["Outcome".to_string()],
        variant: variant.to_string(),
        payload: CtPayload::Tuple(vec![value]),
    }))
}

fn is_io_error_value(value: &CtValue) -> bool {
    matches!(value, CtValue::Enum(e) if e.path.len() == 1 && e.path[0] == "IoError")
}

fn make_project_file_outcome(value: CtValue) -> CtValue {
    if is_io_error_value(&value) {
        make_outcome("Error", value)
    } else {
        make_outcome("Value", value)
    }
}

fn map_io_error_variant(error: &std::io::Error) -> String {
    match error.kind() {
        ErrorKind::PermissionDenied => "PermissionDenied",
        ErrorKind::DirectoryNotEmpty => "DirectoryNotEmpty",
        _ => "IoFailure",
    }
    .to_string()
}

fn snapshot_root_text(project_root: &str) -> String {
    canon(project_root).unwrap_or_else(|| normalize(project_root))
}

fn populate_file_entry(
    entry: &mut ProjectFileSnapshotEntry,
    snapshot: &mut ProjectFileSnapshot,
    host_path: &std::path::Path,
) {
    entry.kind = ProjectFileSnapshotKind::File;
    snapshot.captured_file_count += 1;
    let Ok(bytes) = std::fs::read(host_path) else {
        entry.read_bytes_error = Some("IoFailure".to_string());
        entry.read_error = Some("IoFailure".to_string());
        return;
    };
    snapshot.captured_byte_count += bytes.len() as u64;
    if !decode(&bytes).ok {
        entry.read_error = Some("IoFailure".to_string());
    }
    entry.bytes = bytes;
}

fn populate_directory_entry(
    snapshot: &mut ProjectFileSnapshot,
    entry: &mut ProjectFileSnapshotEntry,
    host_path: &std::path::Path,
    canonical_text: &str,
) {
    entry.kind = ProjectFileSnapshotKind::Directory;
    snapshot.captured_directory_count += 1;
    let iter = match std::fs::read_dir(host_path) {
        Ok(iter) => iter,
        Err(error) => {
            entry.list_dir_error = Some(map_io_error_variant(&error));
            return;
        }
    };
    let mut children = Vec::new();
    for child in iter {
        match child {
            Ok(child) => {
                children.push((child.file_name().to_string_lossy().into_owned(), child.path()));
            }
            Err(error) => {
                entry.list_dir_error = Some(map_io_error_variant(&error));
                return;
            }
        }
    }
    children.sort_by(|lhs, rhs| lhs.0.as_bytes().cmp(rhs.0.as_bytes()));
    entry.dir_entries = children.iter().map(|child| child.0.clone()).collect();
    for (name, path) in &children {
        capture_path_recursive(snapshot, path, &join(canonical_text, name));
    }
}

fn capture_path_recursive(snapshot: &mut ProjectFileSnapshot, host_path: &std::path::Path, canonical_text: &str) {
    let mut entry = ProjectFileSnapshotEntry::default();
    let metadata = match std::fs::metadata(host_path) {
        Ok(metadata) => metadata,
        // A path that does not exist (including a dangling link) has no entry.
        Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => return,
        Err(error) => {
            let variant = map_io_error_variant(&error);
            entry.exists_error = Some(variant.clone());
            entry.read_error = Some(variant.clone());
            entry.read_bytes_error = Some(variant.clone());
            entry.list_dir_error = Some(variant);
            snapshot.entries.insert(canonical_text.to_string(), entry);
            return;
        }
    };
    if metadata.is_dir() {
        populate_directory_entry(snapshot, &mut entry, host_path, canonical_text);
    } else if metadata.is_file() {
        populate_file_entry(&mut entry, snapshot, host_path);
    } else {
        entry.kind = ProjectFileSnapshotKind::Other;
        entry.read_error = Some("IoFailure".to_string());
        entry.read_bytes_error = Some("IoFailure".to_string());
        entry.list_dir_error = Some("IoFailure".to_string());
    }
    snapshot.entries.insert(canonical_text.to_string(), entry);
}

fn restrict_project_path(snapshot: &ProjectFileSnapshot, raw_path: &str) -> Option<String> {
    spec_rule!("CtProjectPath");
    resolve(&snapshot.root_text, raw_path).map(|resolved| resolved.path)
}

/// The access error recorded on the nearest captured ancestor of a path, if any.
fn find_ancestor_access_error(snapshot: &ProjectFileSnapshot, canonical_text: &str) -> Option<String> {
    let mut comps = path_comps(canonical_text);
    while !comps.is_empty() {
        if let Some(entry) = snapshot.entries.get(&join_comp(&comps)) {
            if let Some(error) = entry.list_dir_error.as_ref().or(entry.exists_error.as_ref()) {
                return Some(error.clone());
            }
        }
        comps.pop();
    }
    None
}

/// Looks up an entry that must exist; a missing one yields the error value to return.
fn existing_entry<'a>(
    snapshot: &'a ProjectFileSnapshot,
    canonical_text: &str,
) -> Result<&'a ProjectFileSnapshotEntry, CtValue> {
    let Some(entry) = snapshot.entries.get(canonical_text) else {
        let variant = find_ancestor_access_error(snapshot, canonical_text);
        return Err(make_io_error_value(variant.as_deref().unwrap_or("NotFound")));
    };
    match &entry.exists_error {
        Some(error) => Err(make_io_error_value(error)),
        None => Ok(entry),
    }
}

fn file_entry<'a>(
    snapshot: &'a ProjectFileSnapshot,
    canonical_text: &str,
) -> Result<&'a ProjectFileSnapshotEntry, CtValue> {
    let entry = existing_entry(snapshot, canonical_text)?;
    if entry.kind != ProjectFileSnapshotKind::File {
        return Err(make_io_error_value("IoFailure"));
    }
    Ok(entry)
}

fn snapshot_read_text_result(snapshot: &ProjectFileSnapshot, canonical_text: &str) -> CtValue {
    spec_rule!("requirement.22.ProjectFileSnapshotStability");
    match file_entry(snapshot, canonical_text) {
        Err(error) => error,
        Ok(entry) => match &entry.read_error {
            Some(error) => make_io_error_value(error),
            None => CtValue::String(String::from_utf8_lossy(&entry.bytes).into_owned()),
        },
    }
}

fn snapshot_read_bytes_result(snapshot: &ProjectFileSnapshot, canonical_text: &str) -> CtValue {
    spec_rule!("requirement.22.ProjectFileSnapshotStability");
    match file_entry(snapshot, canonical_text) {
        Err(error) => error,
        Ok(entry) => match &entry.read_bytes_error {
            Some(error) => make_io_error_value(error),
            None => CtValue::Bytes(entry.bytes.clone()),
        },
    }
}

fn snapshot_exists_result(snapshot: &ProjectFileSnapshot, canonical_text: &str) -> CtValue {
    spec_rule!("CtExistsResult(io, q)");
    spec_rule!("requirement.22.ProjectFileSnapshotStability");
    let Some(entry) = snapshot.entries.get(canonical_text) else {
        return match find_ancestor_access_error(snapshot, canonical_text) {
            Some(error) => make_io_error_value(&error),
            None => make_ct_bool(false),
        };
    };
    match &entry.exists_error {
        Some(error) => make_io_error_value(error),
        None => make_ct_bool(true),
    }
}

fn snapshot_list_dir_result(snapshot: &ProjectFileSnapshot, canonical_text: &str) -> CtValue {
    spec_rule!("CtListDirResult(io, q)");
    spec_rule!("requirement.22.ProjectFileSnapshotStability");
    let entry = match existing_entry(snapshot, canonical_text) {
        Ok(entry) => entry,
        Err(error) => return error,
    };
    if entry.kind != ProjectFileSnapshotKind::Directory {
        return make_io_error_value("IoFailure");
    }
    if let Some(error) = &entry.list_dir_error {
        return make_io_error_value(error);
    }
    CtValue::Slice(Rc::new(entry.dir_entries.iter().cloned().map(CtValue::String).collect()))
}

pub fn make_io_error_value(variant: &str) -> CtValue {
    CtValue::Enum(Rc::new(CtEnum {
        path: vec!["IoError".to_string()],
        variant: variant.to_string(),
        payload: CtPayload::None,
    }))
}

pub fn capture_project_file_snapshot(project_root: &str) -> ProjectFileSnapshot {
    let mut snapshot = ProjectFileSnapshot {
        root_text: snapshot_root_text(project_root),
        ..ProjectFileSnapshot::default()
    };
    let root_text = snapshot.root_text.clone();
    capture_path_recursive(&mut snapshot, std::path::Path::new(project_root), &root_text);
    snapshot
}

/// Evaluates `files~>method(..)`. `None` means the call is not a project-files method.
pub fn eval_project_files_method(call: &MethodCallExpr, env: &mut CtEnv) -> Option<EvalResult> {
    let receiver = call.receiver.as_ref()?;
    let ExprNode::IdentifierExpr(ident) = &receiver.node else {
        return None;
    };
    if ident.name != "files" {
        return None;
    }
    let call_span: &Span = &receiver.span;
    spec_rule_at!("requirement.22.CtCapMethodCallParsing", call_span);
    spec_rule_at!("def.22.CtCapabilityDynamicHelpers", call_span);
    spec_rule_at!("requirement.22.ProjectFilesAvailability", call_span);
    let ok = |value: CtValue| Some(EvalResult { ok: true, value, returned: false });
    if call.name == "project_root" && call.args.is_empty() {
        spec_rule!("CtBuiltin-ProjectRoot");
        return ok(CtValue::String(env.project_root.clone()));
    }
    let Some(snapshot) = env.files.clone().filter(|_| call.args.len() == 1) else {
        return Some(EvalResult::default());
    };
    let value = eval_expr(&call.args[0].value, env);
    if !value.ok {
        return Some(value);
    }
    let CtValue::String(path_arg) = &value.value else {
        return Some(EvalResult::default());
    };
    let Some(restricted) = restrict_project_path(&snapshot, path_arg) else {
        spec_rule_at!("requirement.22.ProjectFilesPathRestrictions", call_span);
        match call.name.as_str() {
            "read" => spec_rule!("CtBuiltin-Read-InvalidPath"),
            "read_bytes" => spec_rule!("CtBuiltin-ReadBytes-InvalidPath"),
            "exists" => spec_rule!("CtBuiltin-Exists-InvalidPath"),
            "list_dir" => {
                spec_rule!("CtBuiltin-ListDir-InvalidPath");
                spec_rule_at!("rule.22.CtBuiltin-ListDir-InvalidPath", call_span);
            }
            _ => return None,
        }
        return ok(make_outcome("Error", make_io_error_value("InvalidPath")));
    };
    let result = match call.name.as_str() {
        "read" => {
            spec_rule!("CtBuiltin-Read");
            snapshot_read_text_result(&snapshot, &restricted)
        }
        "read_bytes" => {
            spec_rule!("CtBuiltin-ReadBytes");
            snapshot_read_bytes_result(&snapshot, &restricted)
        }
        "exists" => {
            spec_rule!("CtBuiltin-Exists");
            snapshot_exists_result(&snapshot, &restricted)
        }
        "list_dir" => {
            spec_rule!("CtBuiltin-ListDir");
            spec_rule_at!("rule.22.CtBuiltin-ListDir", call_span);
            snapshot_list_dir_result(&snapshot, &restricted)
        }
        _ => return None,
    };
    ok(make_project_file_outcome(result))
}
