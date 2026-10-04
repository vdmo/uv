use std::path::Path;

use uv_core::diagnostic_messages::{emit_external_diagnostic, make_external_diagnostic};
use uv_core::diagnostics::{emit, has_error, DiagnosticStream, SubDiagnostic, SubDiagnosticKind};
use uv_core::ident::is_name;
use uv_core::path::{path_comps, relative, resolve};
use uv_core::process_config::{
    set_manifest_build_progress, set_manifest_incremental, set_manifest_runtime_lib,
};
use uv_core::spec_rule;

use crate::deterministic_order::fold;
use crate::language_profile::active_language_profile;
use crate::manifest::parse_manifest;
use crate::module_discovery::modules;
use crate::outputs::compute_output_paths;
use crate::project::{Assembly, AssemblyTarget, Project, ValidatedAssembly};
use crate::project_validate::validate_manifest;

#[derive(Default)]
pub struct LoadProjectResult {
    pub project: Option<Project>,
    pub diags: DiagnosticStream,
}

fn build_assembly(
    project_root: &str,
    spec: &ValidatedAssembly,
    diags: &mut DiagnosticStream,
) -> Option<Assembly> {
    let Some(resolved) = resolve(project_root, &spec.root) else {
        spec_rule!("BuildAssembly-Err-Resolve");
        emit_external_diagnostic(diags, "E-PRJ-0304");
        return None;
    };
    let source_root = resolved.path;
    if !Path::new(&source_root).is_dir() {
        spec_rule!("WF-Source-Root-Err");
        spec_rule!("BuildAssembly-Err-Root");
        emit_external_diagnostic(diags, "E-PRJ-0302");
        return None;
    }
    spec_rule!("WF-Source-Root");
    let modules_result = modules(&source_root, &spec.name);
    let failed = has_error(&modules_result.diags);
    for diag in modules_result.diags {
        emit(diags, diag);
    }
    if failed {
        spec_rule!("ModuleList-Err");
        spec_rule!("BuildAssembly-Err-Modules");
        return None;
    }
    let mut module_list = modules_result.modules;
    module_list.sort_by_cached_key(|module| (fold(&module.path), module.path.clone()));
    spec_rule!("ModuleList-Ok");
    let assembly = Assembly {
        name: spec.name.clone(),
        kind: spec.kind.clone(),
        link_kind: spec.link_kind.clone(),
        root: spec.root.clone(),
        out_dir: spec.out_dir.clone(),
        emit_ir: spec.emit_ir.clone(),
        outputs: compute_output_paths(project_root, spec),
        source_root,
        modules: module_list,
    };
    spec_rule!("BuildAssembly-Ok");
    Some(assembly)
}

/// Index of the assembly whose source root is the deepest ancestor of `dir`.
fn owner_assembly_for_dir(
    dir: &str,
    assemblies: &[Assembly],
    diags: &mut DiagnosticStream,
) -> Option<usize> {
    let mut owner: Option<(usize, usize)> = None;
    for (i, assembly) in assemblies.iter().enumerate() {
        if relative(dir, &assembly.source_root).is_none() {
            continue;
        }
        let depth = path_comps(&assembly.source_root).len();
        match owner {
            Some((_, owner_depth)) if depth <= owner_depth => {
                if depth == owner_depth {
                    spec_rule!("WF-Assembly-Root-Owner-Ambiguous");
                    emit_external_diagnostic(diags, "E-PRJ-0206");
                    return None;
                }
            }
            _ => owner = Some((i, depth)),
        }
    }
    if owner.is_none() {
        spec_rule!("WF-Assembly-Root-Owner-Ambiguous");
        emit_external_diagnostic(diags, "E-PRJ-0206");
    }
    owner.map(|(index, _)| index)
}

fn apply_assembly_root_ownership(assemblies: &mut [Assembly], diags: &mut DiagnosticStream) -> bool {
    for i in 0..assemblies.len() {
        let mut owned_modules = Vec::with_capacity(assemblies[i].modules.len());
        for module in &assemblies[i].modules {
            match owner_assembly_for_dir(&module.dir, assemblies, diags) {
                None => return false,
                Some(owner) if owner == i => owned_modules.push(module.clone()),
                Some(_) => {}
            }
        }
        assemblies[i].modules = owned_modules;
    }
    true
}

fn emit_select_error(assemblies: &[Assembly], diags: &mut DiagnosticStream) {
    spec_rule!("Select-Err");
    let names: Vec<&str> = assemblies.iter().map(|assembly| assembly.name.as_str()).collect();
    if let Some(mut diag) = make_external_diagnostic("E-PRJ-0205") {
        diag.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::Note,
            message: format!("available assemblies: {}", names.join(", ")),
            ..SubDiagnostic::default()
        });
        emit(diags, diag);
    }
}

fn select_assembly(
    assemblies: &[Assembly],
    target: &AssemblyTarget,
    diags: &mut DiagnosticStream,
) -> Option<Assembly> {
    let Some(name) = &target.name else {
        if assemblies.len() == 1 {
            spec_rule!("Select-Only");
            return assemblies.first().cloned();
        }
        let mut executables = assemblies.iter().filter(|a| a.kind == "executable");
        if let (Some(only), None) = (executables.next(), executables.next()) {
            spec_rule!("Select-Only-Exe");
            return Some(only.clone());
        }
        emit_select_error(assemblies, diags);
        return None;
    };
    if !is_name(name) {
        spec_rule!("Select-Err");
        emit_external_diagnostic(diags, "E-PRJ-0205");
        return None;
    }
    if let Some(assembly) = assemblies.iter().find(|assembly| assembly.name == *name) {
        spec_rule!("Select-By-Name");
        return Some(assembly.clone());
    }
    emit_select_error(assemblies, diags);
    None
}

fn load_project_impl(
    project_root: &str,
    target: &AssemblyTarget,
    require_selected_assembly: bool,
) -> LoadProjectResult {
    let mut result = LoadProjectResult::default();
    let parsed = parse_manifest(project_root);
    for diag in parsed.diags {
        emit(&mut result.diags, diag);
    }
    let Some(table) = parsed.table else {
        spec_rule!("Step-Parse-Err");
        spec_rule!("LoadProject-Err");
        return result;
    };
    spec_rule!("Step-Parse");
    let validated = validate_manifest(project_root, &table);
    let validation_failed = has_error(&validated.diags) || validated.assemblies.is_empty();
    for diag in validated.diags {
        emit(&mut result.diags, diag);
    }
    if validation_failed {
        spec_rule!("Step-Validate-Err");
        spec_rule!("LoadProject-Err");
        return result;
    }
    spec_rule!("Step-Validate");
    spec_rule!("Step-Asm-Init");
    let mut assemblies = Vec::with_capacity(validated.assemblies.len());
    for spec in &validated.assemblies {
        spec_rule!("Step-Asm-Cons");
        let Some(built) = build_assembly(project_root, spec, &mut result.diags) else {
            spec_rule!("Step-Asm-Err");
            spec_rule!("LoadProject-Err");
            return result;
        };
        assemblies.push(built);
    }
    if !apply_assembly_root_ownership(&mut assemblies, &mut result.diags) {
        spec_rule!("Step-Asm-Own-Err");
        spec_rule!("LoadProject-Err");
        return result;
    }
    let selected = if require_selected_assembly || target.name.is_some() {
        select_assembly(&assemblies, target, &mut result.diags)
    } else {
        assemblies.first().cloned()
    };
    let Some(selected) = selected else {
        spec_rule!("Step-Asm-Done-Err");
        spec_rule!("LoadProject-Err");
        return result;
    };
    let project = Project {
        language: active_language_profile().language,
        root: project_root.to_string(),
        assemblies,
        source_root: selected.source_root.clone(),
        outputs: selected.outputs.clone(),
        modules: selected.modules.clone(),
        lifecycle_modules: selected.modules.clone(),
        assembly: selected,
        test_harness_entry_module: None,
        toolchain: validated.toolchain,
        build: validated.build,
    };
    set_manifest_build_progress(Some(project.build.progress));
    set_manifest_incremental(Some(project.build.incremental));
    set_manifest_runtime_lib(project.toolchain.runtime_lib.clone());
    result.project = Some(project);
    spec_rule!("Step-Asm-Done");
    spec_rule!("LoadProject-Ok");
    result
}

pub fn parse_assembly_target(target: Option<&str>) -> Option<AssemblyTarget> {
    match target {
        None => Some(AssemblyTarget::default()),
        Some(name) if is_name(name) => Some(AssemblyTarget { name: Some(name.to_string()) }),
        Some(_) => None,
    }
}

/// Loads the project and selects exactly one assembly to build.
pub fn load_project(project_root: &str, target: &AssemblyTarget) -> LoadProjectResult {
    load_project_impl(project_root, target, true)
}

/// Loads the project for tooling: without a named target the first assembly is selected.
pub fn load_project_all_assemblies(project_root: &str, target: &AssemblyTarget) -> LoadProjectResult {
    load_project_impl(project_root, target, false)
}
