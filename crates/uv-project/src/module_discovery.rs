use std::collections::HashMap;

use uv_core::diagnostic_messages::{emit_external_diagnostic, make_external_diagnostic};
use uv_core::diagnostics::{emit, has_error, DiagnosticStream};
use uv_core::host::host_get_env_utf8;
use uv_core::ident::is_identifier;
use uv_core::keywords::is_keyword;
use uv_core::path::{file_ext, path_comps, relative};
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use crate::deterministic_order::{dir_seq, file_key, fold, key_cmp};
use crate::fs_path::fs_join;
use crate::language_profile::{active_language_profile, LanguageProfile};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModuleInfo {
    pub path: String,
    pub dir: String,
}

#[derive(Default)]
pub struct ModulesResult {
    pub modules: Vec<ModuleInfo>,
    pub diags: DiagnosticStream,
}

#[derive(Default)]
pub struct CompilationUnitResult {
    pub files: Vec<String>,
    pub diags: DiagnosticStream,
}

fn emit_external_for_obligations(diags: &mut DiagnosticStream, code: &str, obligation_ids: &[&str]) {
    let Some(mut diag) = make_external_diagnostic(code) else {
        return;
    };
    diag.obligation_ids.extend(obligation_ids.iter().map(|id| id.to_string()));
    emit(diags, diag);
}

fn read_err(diags: &mut DiagnosticStream) {
    spec_rule!("DirSeq-Read-Err");
    emit_external_diagnostic(diags, "E-PRJ-0305");
}

/// Regular files in `dir` carrying the source extension, in directory order.
fn source_files(dir: &str, language: &LanguageProfile) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = fs_join(dir, &entry.file_name().to_string_lossy());
        if !std::path::Path::new(&path).is_file() {
            continue;
        }
        if file_ext(&path) == language.source_extension {
            files.push(path);
        }
    }
    Ok(files)
}

struct ModulePath {
    path: String,
    components: Vec<String>,
}

fn module_path_for(
    dir: &str,
    source_root: &str,
    assembly_name: &str,
    diags: &mut DiagnosticStream,
) -> Option<ModulePath> {
    let Some(rel) = relative(dir, source_root) else {
        spec_rule!("Module-Path-Rel-Fail");
        spec_rule!("Disc-Rel-Fail");
        emit_external_diagnostic(diags, "E-PRJ-0304");
        return None;
    };
    if rel.is_empty() {
        spec_rule!("Module-Path-Root");
        return Some(ModulePath {
            path: assembly_name.to_string(),
            components: assembly_name.split("::").map(str::to_string).collect(),
        });
    }
    spec_rule!("Module-Path-Rel");
    let mut components = path_comps(&rel);
    components.insert(0, assembly_name.to_string());
    Some(ModulePath { path: components.join("::"), components })
}

fn validate_module_path(components: &[String], diags: &mut DiagnosticStream) -> bool {
    for comp in components {
        if !is_identifier(comp) {
            spec_rule!("WF-Module-Path-Ident-Err");
            emit_external_for_obligations(
                diags,
                "E-MOD-1106",
                &["WF-Module-Path-Ident-Err", "Disc-Invalid-Component"],
            );
            return false;
        }
        if is_keyword(comp) {
            spec_rule!("WF-Module-Path-Reserved");
            emit_external_for_obligations(
                diags,
                "E-MOD-1105",
                &["WF-Module-Path-Reserved", "Disc-Invalid-Component"],
            );
            return false;
        }
    }
    spec_rule!("WF-Module-Path-Ok");
    true
}

/// Source files of one module directory, in deterministic order.
pub fn compilation_unit(module_dir: &str) -> CompilationUnitResult {
    let language = active_language_profile();
    let mut result = CompilationUnitResult::default();
    if host_get_env_utf8("UV_TEST_COMPILATION_UNIT_FAIL").is_some_and(|force| !force.is_empty()) {
        spec_rule!("CompilationUnit-Rel-Fail");
        emit_external_diagnostic(&mut result.diags, "E-PRJ-0303");
        return result;
    }
    let Ok(files) = source_files(module_dir, language) else {
        read_err(&mut result.diags);
        return result;
    };
    let mut entries = Vec::with_capacity(files.len());
    for file in files {
        let key = file_key(&file, module_dir, &mut result.diags);
        if result.diags.iter().any(|diag| diag.code == "E-PRJ-0303") {
            spec_rule!("CompilationUnit-Rel-Fail");
            return result;
        }
        entries.push((file, key));
    }
    entries.sort_by(|lhs, rhs| key_cmp(&lhs.1, &rhs.1));
    result.files = entries.into_iter().map(|(file, _)| file).collect();
    if Conformance::enabled() {
        spec_rule!("def.ModuleDirectoryFiles");
        Conformance::record_at(
            "def.ModuleDirectoryFiles",
            None,
            &format!(
                "module_dir={module_dir};count={};files=[{}]",
                result.files.len(),
                result.files.join(",")
            ),
        );
    }
    result
}

/// Discovers the modules of an assembly: every directory under the source root that
/// directly contains at least one source file.
pub fn modules(source_root: &str, assembly_name: &str) -> ModulesResult {
    let language = active_language_profile();
    let mut result = ModulesResult::default();
    spec_rule!("Disc-Start");
    let dirs = dir_seq(source_root);
    for diag in &dirs.diags {
        emit(&mut result.diags, diag.clone());
    }
    if has_error(&dirs.diags) {
        spec_rule!("Modules-Err");
        return result;
    }
    if Conformance::enabled() {
        spec_rule!("def.SourceRootDirectories");
        Conformance::record_at(
            "def.SourceRootDirectories",
            None,
            &format!(
                "source_root={source_root};count={};dirs=[{}]",
                dirs.dirs.len(),
                dirs.dirs.join(",")
            ),
        );
    }
    let mut seen: HashMap<String, String> = HashMap::new();
    for dir in &dirs.dirs {
        let Ok(files) = source_files(dir, language) else {
            read_err(&mut result.diags);
            spec_rule!("Modules-Err");
            return result;
        };
        let Some(source_file) = files.first() else {
            spec_rule!("Disc-Skip");
            continue;
        };
        if Conformance::enabled() {
            spec_rule!("Module-Dir");
            Conformance::record_at(
                "Module-Dir",
                None,
                &format!(
                    "module_dir={dir};source_extension={};source_file={source_file};is_module=true",
                    language.source_extension
                ),
            );
        }
        let Some(module_path) = module_path_for(dir, source_root, assembly_name, &mut result.diags)
        else {
            spec_rule!("Modules-Err");
            return result;
        };
        if !validate_module_path(&module_path.components, &mut result.diags) {
            spec_rule!("Disc-Invalid-Component");
            spec_rule!("Modules-Err");
            return result;
        }
        let folded = fold(&module_path.path);
        if seen.get(&folded).is_some_and(|existing| *existing != module_path.path) {
            spec_rule!("Disc-Collision");
            spec_rule!("WF-Module-Path-Collision");
            emit_external_diagnostic(&mut result.diags, "E-MOD-1104");
            emit_external_diagnostic(&mut result.diags, "W-MOD-1101");
            spec_rule!("Modules-Err");
            return result;
        }
        seen.entry(folded).or_insert_with(|| module_path.path.clone());
        if Conformance::enabled() {
            Conformance::record_at(
                "def.ModuleDirOf",
                None,
                &format!(
                    "source=Modules;module_path={};module_dir={dir};source_root={source_root}",
                    module_path.path
                ),
            );
        }
        result.modules.push(ModuleInfo { path: module_path.path, dir: dir.clone() });
        spec_rule!("Disc-Add");
    }
    spec_rule!("Disc-Done");
    spec_rule!("Modules-Ok");
    result
}
