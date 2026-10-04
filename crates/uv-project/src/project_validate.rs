use std::collections::HashSet;
use std::path::Path;

use toml::Value;
use uv_core::diagnostic_messages::emit_external_diagnostic;
use uv_core::diagnostics::DiagnosticStream;
use uv_core::ident::is_name;
use uv_core::path::{canon, is_relative, prefix, resolve};
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use crate::fs_path::fs_join;
use crate::language_profile::active_language_profile;
use crate::manifest::TomlTable;
use crate::project::{BuildConfig, ToolchainConfig, ValidatedAssembly};
use crate::target_profile::{parse_target_profile, target_profile_name};

#[derive(Default)]
pub struct ManifestValidationResult {
    pub assemblies: Vec<ValidatedAssembly>,
    pub toolchain: ToolchainConfig,
    pub build: BuildConfig,
    pub diags: DiagnosticStream,
}

fn toolchain_config_of(table: &TomlTable) -> ToolchainConfig {
    let mut config = ToolchainConfig::default();
    let Some(Value::Table(toolchain)) = table.get("toolchain") else {
        return config;
    };
    if let Some(Value::String(value)) = toolchain.get("llvm_bin") {
        config.llvm_bin = Some(value.clone());
    }
    if let Some(Value::String(value)) = toolchain.get("runtime_lib") {
        config.runtime_lib = Some(value.clone());
    }
    if let Some(Value::String(value)) = toolchain.get("target_profile") {
        config.target_profile = parse_target_profile(value);
    }
    config
}

fn build_config_of(table: &TomlTable) -> BuildConfig {
    let mut config = BuildConfig::default();
    let Some(Value::Table(build)) = table.get("build") else {
        return config;
    };
    if let Some(Value::Boolean(value)) = build.get("incremental") {
        config.incremental = *value;
    }
    if let Some(Value::Boolean(value)) = build.get("progress") {
        config.progress = *value;
    }
    config
}

fn asm_link_kind(kind: &str, link_kind: Option<&str>) -> Option<String> {
    if kind != "library" {
        return None;
    }
    match link_kind {
        None => Some("shared".to_string()),
        Some(value @ ("shared" | "static")) => Some(value.to_string()),
        Some(_) => None,
    }
}

fn render_opt(value: Option<&str>) -> &str {
    value.unwrap_or("<bottom>")
}

fn has_toolchain_table(table: &TomlTable) -> bool {
    matches!(table.get("toolchain"), Some(Value::Table(_)))
}

fn record_toolchain_keys(
    toolchain_present: bool,
    toolchain_table: bool,
    has_llvm_bin: bool,
    has_runtime_lib: bool,
    has_target_profile: bool,
    known_only: bool,
) {
    if !Conformance::enabled() {
        return;
    }
    spec_rule!("def.ToolchainKeys");
    Conformance::record_at(
        "def.ToolchainKeys",
        None,
        &format!(
            "allowed=llvm_bin,runtime_lib,target_profile;toolchain_present={toolchain_present};toolchain_table={toolchain_table};has_llvm_bin={has_llvm_bin};has_runtime_lib={has_runtime_lib};has_target_profile={has_target_profile};known_only={known_only}"
        ),
    );
}

fn record_toolchain_target_profile_ok(value: &str, provided: bool, is_string: bool, ok: bool) {
    if !Conformance::enabled() {
        return;
    }
    spec_rule!("def.ToolchainTargetProfileOk");
    Conformance::record_at(
        "def.ToolchainTargetProfileOk",
        None,
        &format!("provided={provided};is_string={is_string};value={value};ok={ok}"),
    );
}

fn record_toolchain_summary(table: &TomlTable, config: &ToolchainConfig) {
    if !Conformance::enabled() {
        return;
    }
    let fields = format!(
        "llvm_bin={};runtime_lib={};target_profile={}",
        render_opt(config.llvm_bin.as_deref()),
        render_opt(config.runtime_lib.as_deref()),
        config.target_profile.map_or("<bottom>", target_profile_name)
    );
    spec_rule!("WF-Toolchain");
    Conformance::record_at(
        "WF-Toolchain",
        None,
        &format!(
            "toolchain_present={};toolchain_table={};{fields}",
            table.contains_key("toolchain"),
            has_toolchain_table(table)
        ),
    );
    spec_rule!("def.ToolchainConfig");
    Conformance::record_at(
        "def.ToolchainConfig",
        None,
        &format!(
            "branch={};{fields}",
            if has_toolchain_table(table) { "provided" } else { "defaults" }
        ),
    );
}

fn record_asm_link_kind(kind: &str, link_kind: Option<&str>, effective: Option<&str>) {
    if !Conformance::enabled() {
        return;
    }
    spec_rule!("def.AsmLinkKind");
    Conformance::record_at(
        "def.AsmLinkKind",
        None,
        &format!(
            "kind={kind};link_kind={};effective={}",
            render_opt(link_kind),
            render_opt(effective)
        ),
    );
}

fn record_first_fail(result: &str, first_obligation: &str, diagnostic_code: &str) {
    if !Conformance::enabled() {
        return;
    }
    spec_rule!("def.FirstFail");
    Conformance::record_at(
        "def.FirstFail",
        None,
        &format!(
            "source=ValidateManifest;result={result};first={first_obligation};diagnostic={diagnostic_code}"
        ),
    );
}

enum RelPathStatus {
    Ok,
    RelPathErr,
    ResolveErr,
}

fn check_rel_path(p: &str, root: &str) -> RelPathStatus {
    if !is_relative(p) || canon(p).is_none() {
        spec_rule!("WF-RelPath-Err");
        return RelPathStatus::RelPathErr;
    }
    let Some(resolved) = resolve(root, p) else {
        return RelPathStatus::ResolveErr;
    };
    if !prefix(&resolved.root, &resolved.path) {
        spec_rule!("WF-RelPath-Err");
        return RelPathStatus::RelPathErr;
    }
    spec_rule!("WF-RelPath");
    RelPathStatus::Ok
}

/// The `[[assembly]]` tables, or `None` when the `assembly` value has the wrong shape.
fn asm_tables(assembly_node: &Value) -> Option<Vec<&TomlTable>> {
    match assembly_node {
        Value::Table(table) => Some(vec![table]),
        Value::Array(array) => array
            .iter()
            .map(|entry| match entry {
                Value::Table(table) => Some(table),
                _ => None,
            })
            .collect(),
        _ => None,
    }
}

struct Failure {
    code: &'static str,
    rule: &'static str,
}

fn fail(code: &'static str, rule: &'static str) -> Failure {
    Failure { code, rule }
}

fn validate_toolchain(table: &TomlTable) -> Result<(), Failure> {
    const ERR: Failure = Failure { code: "E-PRJ-0110", rule: "WF-Toolchain-Err" };
    let Some(toolchain_node) = table.get("toolchain") else {
        record_toolchain_keys(false, false, false, false, false, true);
        record_toolchain_target_profile_ok("<bottom>", false, false, true);
        return Ok(());
    };
    let Value::Table(toolchain) = toolchain_node else {
        record_toolchain_keys(true, false, false, false, false, false);
        record_toolchain_target_profile_ok("<non-table>", true, false, false);
        return Err(ERR);
    };
    let has_llvm_bin = toolchain.contains_key("llvm_bin");
    let has_runtime_lib = toolchain.contains_key("runtime_lib");
    let has_target_profile = toolchain.contains_key("target_profile");
    let known_only = toolchain
        .keys()
        .all(|key| matches!(key.as_str(), "llvm_bin" | "runtime_lib" | "target_profile"));
    record_toolchain_keys(true, true, has_llvm_bin, has_runtime_lib, has_target_profile, known_only);
    if !known_only {
        return Err(ERR);
    }
    for key in ["llvm_bin", "runtime_lib"] {
        if toolchain.get(key).is_some_and(|node| !node.is_str()) {
            return Err(ERR);
        }
    }
    match toolchain.get("target_profile") {
        None => record_toolchain_target_profile_ok("<bottom>", false, false, true),
        Some(Value::String(value)) => {
            let ok = parse_target_profile(value).is_some();
            record_toolchain_target_profile_ok(value, true, true, ok);
            if !ok {
                return Err(ERR);
            }
        }
        Some(_) => {
            record_toolchain_target_profile_ok("<non-string>", true, false, false);
            return Err(ERR);
        }
    }
    Ok(())
}

fn validate_build(table: &TomlTable) -> Result<(), Failure> {
    const ERR: Failure = Failure { code: "E-PRJ-0111", rule: "WF-Build-Err" };
    let Some(build_node) = table.get("build") else {
        return Ok(());
    };
    let Value::Table(build) = build_node else {
        return Err(ERR);
    };
    if !build.keys().all(|key| matches!(key.as_str(), "incremental" | "progress")) {
        return Err(ERR);
    }
    for key in ["incremental", "progress"] {
        if build.get(key).is_some_and(|node| !node.is_bool()) {
            return Err(ERR);
        }
    }
    Ok(())
}

fn optional_string(
    assembly: &TomlTable,
    key: &str,
    code: &'static str,
    rule: &'static str,
) -> Result<Option<String>, Failure> {
    match assembly.get(key) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(fail(code, rule)),
    }
}

fn validate_assembly(assembly: &TomlTable, root_str: &str) -> Result<ValidatedAssembly, Failure> {
    let known = |key: &String| {
        matches!(key.as_str(), "name" | "kind" | "root" | "out_dir" | "emit_ir" | "link_kind")
    };
    if !assembly.keys().all(known) {
        return Err(fail("E-PRJ-0104", "WF-Assembly-Keys-Err"));
    }
    spec_rule!("WF-Assembly-Keys");
    let (Some(Value::String(name)), Some(Value::String(kind)), Some(Value::String(root))) =
        (assembly.get("name"), assembly.get("kind"), assembly.get("root"))
    else {
        return Err(fail("E-PRJ-0103", "WF-Assembly-Required-Types-Err"));
    };
    spec_rule!("WF-Assembly-Required-Types");
    let out_dir = optional_string(assembly, "out_dir", "E-PRJ-0301", "WF-Assembly-OutDirType-Err")?;
    let emit_ir = optional_string(assembly, "emit_ir", "E-PRJ-0204", "WF-Assembly-EmitIRType-Err")?;
    spec_rule!("WF-Assembly-EmitIRType");
    let link_kind =
        optional_string(assembly, "link_kind", "E-PRJ-0207", "WF-Assembly-LinkKindType-Err")?;
    spec_rule!("WF-Assembly-LinkKindType");
    spec_rule!("WF-Assembly-Optional-Types");
    if !is_name(name) {
        return Err(fail("E-PRJ-0203", "WF-Assembly-Name-Err"));
    }
    spec_rule!("WF-Assembly-Name");
    if !matches!(kind.as_str(), "executable" | "library" | "dependency") {
        return Err(fail("E-PRJ-0201", "WF-Assembly-Kind-Err"));
    }
    spec_rule!("WF-Assembly-Kind");
    if kind != "library" && link_kind.is_some() {
        return Err(fail("E-PRJ-0208", "WF-Assembly-LinkKind-Use-Err"));
    }
    if kind == "library" && link_kind.as_deref().is_some_and(|v| v != "shared" && v != "static") {
        return Err(fail("E-PRJ-0207", "WF-Assembly-LinkKind-Err"));
    }
    spec_rule!("WF-Assembly-LinkKind");
    if emit_ir.as_deref().is_some_and(|v| !matches!(v, "none" | "ll" | "bc")) {
        return Err(fail("E-PRJ-0204", "WF-Assembly-EmitIR-Err"));
    }
    spec_rule!("WF-Assembly-EmitIR");
    match check_rel_path(root, root_str) {
        RelPathStatus::RelPathErr => {
            spec_rule!("WF-Assembly-Root-Path-Err");
            return Err(Failure { code: "E-PRJ-0301", rule: "WF-Assembly-Root-Path-Err" });
        }
        RelPathStatus::ResolveErr => {
            return Err(Failure { code: "E-PRJ-0304", rule: "WF-Assembly-Root-Path-Err" });
        }
        RelPathStatus::Ok => {}
    }
    spec_rule!("WF-Assembly-Root-Path");
    if let Some(out_dir) = &out_dir {
        match check_rel_path(out_dir, root_str) {
            RelPathStatus::RelPathErr => {
                spec_rule!("WF-Assembly-OutDir-Path-Err");
                return Err(Failure { code: "E-PRJ-0301", rule: "WF-Assembly-OutDir-Path-Err" });
            }
            RelPathStatus::ResolveErr => {
                return Err(Failure { code: "E-PRJ-0304", rule: "WF-Assembly-OutDir-Path-Err" });
            }
            RelPathStatus::Ok => {}
        }
    }
    spec_rule!("WF-Assembly-OutDir-Path");
    let effective_link_kind = asm_link_kind(kind, link_kind.as_deref());
    record_asm_link_kind(kind, link_kind.as_deref(), effective_link_kind.as_deref());
    spec_rule!("WF-Assembly");
    Ok(ValidatedAssembly {
        name: name.clone(),
        kind: kind.clone(),
        link_kind: effective_link_kind,
        root: root.clone(),
        out_dir,
        emit_ir,
    })
}

fn validate_into(
    project_root: &str,
    table: &TomlTable,
    result: &mut ManifestValidationResult,
) -> Result<(), Failure> {
    if !table.keys().all(|key| matches!(key.as_str(), "assembly" | "toolchain" | "build")) {
        return Err(fail("E-PRJ-0104", "WF-TopKeys-Err"));
    }
    spec_rule!("WF-TopKeys");
    validate_toolchain(table)?;
    record_toolchain_summary(table, &result.toolchain);
    validate_build(table)?;
    spec_rule!("WF-Build");
    let tables = table
        .get("assembly")
        .and_then(asm_tables)
        .ok_or(fail("E-PRJ-0103", "WF-Assembly-Table-Err"))?;
    spec_rule!("WF-Assembly-Table");
    if tables.is_empty() {
        return Err(fail("E-PRJ-0103", "WF-Assembly-Count-Err"));
    }
    spec_rule!("WF-Assembly-Count");
    let mut names = HashSet::new();
    for assembly in &tables {
        if let Some(Value::String(name)) = assembly.get("name") {
            if !names.insert(name.as_str()) {
                return Err(fail("E-PRJ-0202", "WF-Assembly-Name-Dup-Err"));
            }
        }
    }
    spec_rule!("WF-Assembly-Name-Dup");
    for assembly in tables {
        let validated = validate_assembly(assembly, project_root)?;
        result.assemblies.push(validated);
    }
    Ok(())
}

pub fn validate_manifest(project_root: &str, table: &TomlTable) -> ManifestValidationResult {
    let mut result = ManifestValidationResult {
        toolchain: toolchain_config_of(table),
        build: build_config_of(table),
        ..ManifestValidationResult::default()
    };
    match validate_into(project_root, table, &mut result) {
        Ok(()) => {
            spec_rule!("ValidateManifest-Ok");
            record_first_fail("ok", "<bottom>", "<bottom>");
        }
        Err(failure) => {
            // The failing rule is recorded before the first-fail summary, as in the reference.
            if failure.rule != "WF-Assembly-Root-Path-Err" && failure.rule != "WF-Assembly-OutDir-Path-Err" {
                Conformance::record_if_enabled(failure.rule);
            }
            record_first_fail("error", failure.rule, failure.code);
            spec_rule!("ValidateManifest-Err");
            emit_external_diagnostic(&mut result.diags, failure.code);
        }
    }
    result
}

pub fn is_project_root(root: &str) -> bool {
    let manifest = fs_join(root, active_language_profile().manifest_name);
    match Path::new(&manifest).try_exists() {
        Ok(true) => {
            spec_rule!("WF-Project-Root");
            true
        }
        _ => false,
    }
}
