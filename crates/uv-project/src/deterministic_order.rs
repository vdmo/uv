use std::cmp::Ordering;
use std::path::Path;

use uv_core::diagnostic_messages::emit_external_diagnostic;
use uv_core::diagnostics::DiagnosticStream;
use uv_core::path::{basename, join_comp, path_comps, relative};
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;
use uv_core::unicode::{case_fold, nfc};

use crate::fs_path::fs_join;

const BOTTOM_ORDER_KEY: &str = "<bottom>";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderKey {
    pub folded: String,
    pub raw: String,
}

pub fn fold_path(path: &str) -> String {
    let folded: Vec<String> = path_comps(path).iter().map(|comp| case_fold(&nfc(comp))).collect();
    join_comp(&folded)
}

pub fn fold(module_path: &str) -> String {
    module_path.split("::").map(|comp| case_fold(&nfc(comp))).collect::<Vec<_>>().join("::")
}

/// Total order on keys: the bottom key sorts first, then folded text, then raw text (bytewise).
pub fn key_cmp(a: &OrderKey, b: &OrderKey) -> Ordering {
    let a_is_bottom = a.folded == BOTTOM_ORDER_KEY;
    let b_is_bottom = b.folded == BOTTOM_ORDER_KEY;
    if a_is_bottom != b_is_bottom {
        return if a_is_bottom { Ordering::Less } else { Ordering::Greater };
    }
    a.folded.as_bytes().cmp(b.folded.as_bytes()).then_with(|| a.raw.as_bytes().cmp(b.raw.as_bytes()))
}

pub fn file_key(file: &str, dir: &str, diags: &mut DiagnosticStream) -> OrderKey {
    match relative(file, dir) {
        None => {
            spec_rule!("FileOrder-Rel-Fail");
            emit_external_diagnostic(diags, "E-PRJ-0303");
            OrderKey { folded: BOTTOM_ORDER_KEY.to_string(), raw: basename(file) }
        }
        Some(rel) => OrderKey { folded: fold_path(&rel), raw: rel },
    }
}

fn record_dir_key(dir: &str, base: &str, rel: Option<&str>, key: &OrderKey, fallback: bool) {
    if !Conformance::enabled() {
        return;
    }
    spec_rule!("def.DirKey");
    Conformance::record_at(
        "def.DirKey",
        None,
        &format!(
            "dir={dir};base={base};relative={};folded={};raw={};fallback={fallback}",
            rel.unwrap_or("<bottom>"),
            key.folded,
            key.raw
        ),
    );
}

pub fn dir_key(dir: &str, base: &str, diags: &mut DiagnosticStream) -> OrderKey {
    match relative(dir, base) {
        None => {
            spec_rule!("DirSeq-Rel-Fail");
            emit_external_diagnostic(diags, "E-PRJ-0303");
            let key = OrderKey { folded: BOTTOM_ORDER_KEY.to_string(), raw: basename(dir) };
            record_dir_key(dir, base, None, &key, true);
            key
        }
        Some(rel) => {
            let key = OrderKey { folded: fold_path(&rel), raw: rel.clone() };
            record_dir_key(dir, base, Some(&rel), &key, false);
            key
        }
    }
}

#[derive(Default)]
pub struct DirSeqResult {
    pub dirs: Vec<String>,
    pub diags: DiagnosticStream,
}

pub fn dir_seq_from(root: &str, dirs: &[String]) -> DirSeqResult {
    let mut result = DirSeqResult::default();
    let mut entries: Vec<(String, OrderKey)> =
        dirs.iter().map(|dir| (dir.clone(), dir_key(dir, root, &mut result.diags))).collect();
    entries.sort_by(|lhs, rhs| key_cmp(&lhs.1, &rhs.1));
    if Conformance::enabled() {
        let ordered: Vec<String> = entries
            .iter()
            .map(|(path, key)| format!("{path}=<{},{}>", key.folded, key.raw))
            .collect();
        spec_rule!("def.DirectoryOrdering");
        Conformance::record_at(
            "def.DirectoryOrdering",
            None,
            &format!("root={root};count={};ordered_keys=[{}]", entries.len(), ordered.join(",")),
        );
    }
    result.dirs = entries.into_iter().map(|(path, _)| path).collect();
    if Conformance::enabled() {
        spec_rule!("def.DirSeq");
        Conformance::record_at(
            "def.DirSeq",
            None,
            &format!("root={root};count={};dirs=[{}]", result.dirs.len(), result.dirs.join(",")),
        );
    }
    result
}

/// Collects `dir` entries that are directories, descending into real (non-symlink) directories.
fn collect_dirs(dir: &str, dirs: &mut Vec<String>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = fs_join(dir, &entry.file_name().to_string_lossy());
        let is_directory = Path::new(&path).is_dir();
        if is_directory {
            dirs.push(path.clone());
            if entry.file_type()?.is_dir() {
                collect_dirs(&path, dirs)?;
            }
        }
    }
    Ok(())
}

/// The source root and every directory below it, in deterministic order.
pub fn dir_seq(root: &str) -> DirSeqResult {
    let mut dirs = vec![root.to_string()];
    if collect_dirs(root, &mut dirs).is_err() {
        let mut result = DirSeqResult::default();
        spec_rule!("DirSeq-Read-Err");
        emit_external_diagnostic(&mut result.diags, "E-PRJ-0305");
        return result;
    }
    dir_seq_from(root, &dirs)
}
