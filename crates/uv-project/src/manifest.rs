use std::path::{Component, Path, PathBuf};

use uv_core::diagnostic_messages::make_external_diagnostic;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::spec_rule;

use crate::fs_path::{fs_join, generic_string, parent_path};
use crate::language_profile::{active_language_profile, LanguageProfile, SourceLanguage};

pub type TomlTable = toml::Table;

#[derive(Default)]
pub struct ManifestParseResult {
    pub table: Option<TomlTable>,
    pub diags: DiagnosticStream,
}

/// Lexical normalization: drops `.` and resolves `..` against preceding components.
fn lexically_normal(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() && !out.has_root() {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Directory from which the manifest search starts, and whether the input was a directory.
pub fn start_dir_for_input(input_path: &str) -> (String, bool) {
    let mut dir = PathBuf::from(if input_path.is_empty() { "." } else { input_path });
    if dir.is_relative() {
        if let Ok(cwd) = std::env::current_dir() {
            dir = cwd.join(dir);
        }
    }
    dir = lexically_normal(&dir);
    let mut input_is_directory = false;
    if dir.exists() {
        input_is_directory = dir.is_dir();
        if !input_is_directory && dir.file_name().is_some() {
            dir.pop();
        }
    }
    if dir.as_os_str().is_empty() {
        dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    }
    (generic_string(&dir), input_is_directory)
}

fn emit_manifest_diagnostic(diags: &mut DiagnosticStream, code: &str, language: &LanguageProfile) {
    let Some(mut diag) = make_external_diagnostic(code) else {
        return;
    };
    if language.language == SourceLanguage::Ultraviolet {
        if code == "E-PRJ-0101" {
            diag.message = "`Ultraviolet.toml` not found at project root".to_string();
        } else if code == "E-PRJ-0102" {
            diag.message = "`Ultraviolet.toml` is not valid TOML".to_string();
        }
    }
    emit(diags, diag);
}

/// Walks up from the input path to the nearest directory holding a manifest.
pub fn find_project_root(input_path: &str) -> String {
    let language = active_language_profile();
    let (start, input_is_directory) = start_dir_for_input(input_path);
    if input_is_directory {
        spec_rule!("req.ProjectRootDirectoryInputStartsAtResolvedPath");
    }
    let mut current = start.clone();
    loop {
        match Path::new(&fs_join(&current, language.manifest_name)).try_exists() {
            Err(_) | Ok(true) => return current,
            Ok(false) => {}
        }
        let parent = parent_path(&current);
        if parent.is_empty() || parent == current {
            break;
        }
        current = parent;
    }
    start
}

pub fn parse_manifest(project_root: &str) -> ManifestParseResult {
    let language = active_language_profile();
    let mut result = ManifestParseResult::default();
    let manifest_path = fs_join(project_root, language.manifest_name);
    spec_rule!("req.ManifestHostPathResolution");
    match Path::new(&manifest_path).try_exists() {
        Err(_) => {
            spec_rule!("Parse-Manifest-Err");
            emit_manifest_diagnostic(&mut result.diags, "E-PRJ-0102", language);
            return result;
        }
        Ok(false) => {
            spec_rule!("Parse-Manifest-Missing");
            spec_rule!("conformance.ManifestRequired");
            emit_manifest_diagnostic(&mut result.diags, "E-PRJ-0101", language);
            return result;
        }
        Ok(true) => {}
    }
    let parsed = std::fs::read_to_string(&manifest_path)
        .ok()
        .and_then(|text| text.parse::<TomlTable>().ok());
    match parsed {
        Some(table) => {
            result.table = Some(table);
            spec_rule!("Parse-Manifest-Ok");
        }
        None => {
            spec_rule!("Parse-Manifest-Err");
            emit_manifest_diagnostic(&mut result.diags, "E-PRJ-0102", language);
        }
    }
    result
}
