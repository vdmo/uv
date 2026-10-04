use uv_core::path::{normalize, prefix, relative};
use uv_core::process_config::out_dir_override;
use uv_core::spec_trace::Conformance;
use uv_core::symbols::mangle_module_path;

use crate::fs_path::fs_join;
use crate::module_discovery::{compilation_unit, ModuleInfo};
use crate::project::{Assembly, Project, ValidatedAssembly};
use crate::target_profile::{
    emits_import_lib, exe_suffix, import_lib_suffix, library_prefix, obj_ext, object_format_of,
    shared_lib_suffix, static_lib_suffix, target_profile_name, ObjectFormat, TargetProfile,
};

const DEFAULT_OUTPUT_ROOT: &str = "Build";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutputPaths {
    pub root: String,
    pub intermediate_dir: String,
    pub obj_dir: String,
    pub ir_dir: String,
    pub bin_dir: String,
    pub lib_dir: String,
    pub logs_dir: String,
    pub incremental_dir: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkOutputKind {
    Executable,
    SharedLibrary,
}

fn record(rule: &str, payload: &str) {
    if Conformance::enabled() {
        Conformance::record(rule);
        Conformance::record_at(rule, None, payload);
    }
}

fn opt(value: Option<&str>) -> &str {
    value.unwrap_or("<bottom>")
}

fn first_path(paths: &[String]) -> &str {
    paths.first().map_or("<bottom>", String::as_str)
}

fn emit_ir_mode(project: &Project) -> &str {
    project.assembly.emit_ir.as_deref().unwrap_or("none")
}

fn emits_ir(emit_ir: &str) -> bool {
    emit_ir == "ll" || emit_ir == "bc"
}

fn module_output_relative_dir(project: &Project, module: &ModuleInfo) -> String {
    relative(&module.dir, &project.root).unwrap_or_default()
}

fn artifact_library_name(project: &Project, target_profile: TargetProfile) -> String {
    format!("{}{}", library_prefix(target_profile), project.assembly.name)
}

fn output_root_source(assembly_out_dir: Option<&str>) -> &'static str {
    if out_dir_override().is_some() {
        "cli_override"
    } else if assembly_out_dir.is_some() {
        "assembly_out_dir"
    } else {
        "default"
    }
}

fn record_output_path_layout(assembly_name: &str, paths: &OutputPaths, root_source: &str) {
    if !Conformance::enabled() {
        return;
    }
    let root = format!("assembly={assembly_name};root_source={root_source};value={}", paths.root);
    record("def.OutputRoot", &root);
    record("def.OutputPathsRoot", &root);
    record(
        "def.OutputPathsDirectories",
        &format!(
            "assembly={assembly_name};intermediate={};obj={};ir={};bin={};lib={};logs={};incremental={}",
            paths.intermediate_dir,
            paths.obj_dir,
            paths.ir_dir,
            paths.bin_dir,
            paths.lib_dir,
            paths.logs_dir,
            paths.incremental_dir
        ),
    );
}

pub fn output_paths_for_root(root: &str) -> OutputPaths {
    let intermediate_dir = fs_join(root, "Intermediate");
    OutputPaths {
        root: root.to_string(),
        obj_dir: fs_join(&intermediate_dir, "Obj"),
        ir_dir: fs_join(&intermediate_dir, "IR"),
        bin_dir: fs_join(root, "Binary"),
        lib_dir: fs_join(root, "Library"),
        logs_dir: fs_join(root, "Logs"),
        incremental_dir: fs_join(&intermediate_dir, "Incremental"),
        intermediate_dir,
    }
}

pub fn compute_output_paths(project_root: &str, assembly: &ValidatedAssembly) -> OutputPaths {
    let cli_out_dir = out_dir_override();
    let root_source = output_root_source(assembly.out_dir.as_deref());
    let out_dir =
        cli_out_dir.as_deref().or(assembly.out_dir.as_deref()).unwrap_or(DEFAULT_OUTPUT_ROOT);
    let paths = output_paths_for_root(&fs_join(project_root, out_dir));
    record_output_path_layout(&assembly.name, &paths, root_source);
    paths
}

/// The project viewed through one of its assemblies.
pub fn assembly_project(base_project: &Project, assembly: &Assembly) -> Project {
    let mut project = base_project.clone();
    project.assembly = assembly.clone();
    project.source_root = assembly.source_root.clone();
    project.outputs = assembly.outputs.clone();
    project.modules = assembly.modules.clone();
    project.lifecycle_modules = assembly.modules.clone();
    record(
        "def.ProjectOutputBinding",
        &format!("assembly={};outputs_bound=true;root={}", assembly.name, project.outputs.root),
    );
    project
}

pub fn obj_path(project: &Project, target_profile: TargetProfile, module: &ModuleInfo) -> String {
    let mangled = mangle_module_path(&module.path);
    let rel_dir = module_output_relative_dir(project, module);
    let path = fs_join(
        &fs_join(&project.outputs.obj_dir, &rel_dir),
        &format!("{mangled}{}", obj_ext(target_profile)),
    );
    if Conformance::enabled() {
        let payload = format!(
            "assembly={};module={};relative_dir={rel_dir};mangled={mangled};target={};value={path}",
            project.assembly.name,
            module.path,
            target_profile_name(target_profile)
        );
        record("def.ObjectPath", &payload);
        record("def.ObjPath", &payload);
    }
    path
}

pub fn ir_path(project: &Project, module: &ModuleInfo, emit_ir: &str) -> String {
    let ext = if emit_ir == "bc" { ".bc" } else { ".ll" };
    let mangled = mangle_module_path(&module.path);
    let rel_dir = module_output_relative_dir(project, module);
    let path = fs_join(&fs_join(&project.outputs.ir_dir, &rel_dir), &format!("{mangled}{ext}"));
    if Conformance::enabled() {
        record("def.EmitIRExtension", &format!("emit_ir={emit_ir};extension={ext}"));
        record(
            "def.IRPath",
            &format!(
                "assembly={};module={};relative_dir={rel_dir};mangled={mangled};emit_ir={emit_ir};value={path}",
                project.assembly.name, module.path
            ),
        );
    }
    path
}

pub fn exe_path(project: &Project, target_profile: TargetProfile) -> String {
    let file = format!("{}{}", project.assembly.name, exe_suffix(target_profile));
    let path = fs_join(&project.outputs.bin_dir, &file);
    record("def.ExePath", &format!("assembly={};value={path}", project.assembly.name));
    path
}

pub fn shared_lib_path(project: &Project, target_profile: TargetProfile) -> String {
    let file = format!(
        "{}{}",
        artifact_library_name(project, target_profile),
        shared_lib_suffix(target_profile)
    );
    let path = fs_join(&project.outputs.bin_dir, &file);
    record("def.SharedLibPath", &format!("assembly={};value={path}", project.assembly.name));
    path
}

pub fn static_lib_path(project: &Project, target_profile: TargetProfile) -> String {
    let file = format!(
        "{}{}",
        artifact_library_name(project, target_profile),
        static_lib_suffix(target_profile)
    );
    let path = fs_join(&project.outputs.lib_dir, &file);
    record("def.StaticLibPath", &format!("assembly={};value={path}", project.assembly.name));
    path
}

pub fn import_lib_path(project: &Project, target_profile: TargetProfile) -> Option<String> {
    let shared = project.assembly.is_shared_library();
    let import_lib = (shared && emits_import_lib(target_profile)).then(|| {
        let file = format!(
            "{}{}",
            artifact_library_name(project, target_profile),
            import_lib_suffix(target_profile)
        );
        fs_join(&project.outputs.lib_dir, &file)
    });
    if Conformance::enabled() {
        let payload = format!(
            "assembly={};shared_library={shared};emits_import_lib={};value={}",
            project.assembly.name,
            emits_import_lib(target_profile),
            opt(import_lib.as_deref())
        );
        record("def.ImportLibPath", &payload);
        record("def.LinkImportLibOpt", &payload);
    }
    import_lib
}

pub fn primary_artifact_path(project: &Project, target_profile: TargetProfile) -> Option<String> {
    let (artifact, branch) = if project.assembly.is_executable() {
        (Some(exe_path(project, target_profile)), "executable")
    } else if project.assembly.is_shared_library() {
        (Some(shared_lib_path(project, target_profile)), "shared")
    } else if project.assembly.is_static_library() {
        (Some(static_lib_path(project, target_profile)), "static")
    } else {
        (None, "<bottom>")
    };
    record(
        "def.PrimaryArtifact",
        &format!(
            "assembly={};branch={branch};value={}",
            project.assembly.name,
            opt(artifact.as_deref())
        ),
    );
    artifact
}

pub fn map_path(project: &Project, target_profile: TargetProfile) -> Option<String> {
    if object_format_of(target_profile) != ObjectFormat::Coff {
        return None;
    }
    if !(project.assembly.is_executable() || project.assembly.is_shared_library()) {
        return None;
    }
    let primary = primary_artifact_path(project, target_profile)?;
    Some(std::path::Path::new(&primary).with_extension("map").to_string_lossy().into_owned())
}

pub fn library_artifact_inputs(inputs: Vec<String>) -> Vec<String> {
    record(
        "def.LibraryArtifactInputs",
        &format!("count={};first={}", inputs.len(), first_path(&inputs)),
    );
    inputs
}

pub fn link_mode(project: &Project) -> Option<LinkOutputKind> {
    let (mode, name) = if project.assembly.is_executable() {
        (Some(LinkOutputKind::Executable), "exe")
    } else if project.assembly.is_shared_library() {
        (Some(LinkOutputKind::SharedLibrary), "shared")
    } else {
        (None, "<bottom>")
    };
    record("def.LinkMode", &format!("assembly={};mode={name}", project.assembly.name));
    mode
}

pub fn link_output_path(project: &Project, target_profile: TargetProfile) -> Option<String> {
    let (output_path, mode) = if project.assembly.is_executable() {
        (Some(exe_path(project, target_profile)), "exe")
    } else if project.assembly.is_shared_library() {
        (Some(shared_lib_path(project, target_profile)), "shared")
    } else {
        (None, "<bottom>")
    };
    record(
        "def.LinkOutputPath",
        &format!(
            "assembly={};mode={mode};value={}",
            project.assembly.name,
            opt(output_path.as_deref())
        ),
    );
    output_path
}

pub fn uses_bin_dir(project: &Project) -> bool {
    project.assembly.is_executable() || project.assembly.is_shared_library()
}

pub fn uses_lib_dir(project: &Project, target_profile: TargetProfile) -> bool {
    project.assembly.is_static_library() || import_lib_path(project, target_profile).is_some()
}

pub fn obj_paths(
    project: &Project,
    target_profile: TargetProfile,
    modules: &[ModuleInfo],
) -> Vec<String> {
    let out: Vec<String> =
        modules.iter().map(|module| obj_path(project, target_profile, module)).collect();
    record(
        "def.ObjPaths",
        &format!(
            "assembly={};modules={};count={};first={}",
            project.assembly.name,
            modules.len(),
            out.len(),
            first_path(&out)
        ),
    );
    out
}

pub fn ir_paths(project: &Project, modules: &[ModuleInfo], emit_ir: &str) -> Vec<String> {
    if !emits_ir(emit_ir) {
        return Vec::new();
    }
    let out: Vec<String> = modules.iter().map(|module| ir_path(project, module, emit_ir)).collect();
    record(
        "def.IRPaths",
        &format!(
            "assembly={};emit_ir={emit_ir};modules={};count={};first={}",
            project.assembly.name,
            modules.len(),
            out.len(),
            first_path(&out)
        ),
    );
    out
}

pub fn required_outputs(project: &Project, target_profile: TargetProfile) -> Vec<String> {
    let objs = obj_paths(project, target_profile, &project.modules);
    let obj_count = objs.len();
    let mut out = objs;
    let emit_ir = emit_ir_mode(project);
    let ir_count = if emits_ir(emit_ir) { project.modules.len() } else { 0 };
    out.extend(ir_paths(project, &project.modules, emit_ir));
    out.extend(primary_artifact_path(project, target_profile));
    out.extend(import_lib_path(project, target_profile));
    if Conformance::enabled() {
        let name = &project.assembly.name;
        let primary = primary_artifact_path(project, target_profile);
        let import_lib = import_lib_path(project, target_profile);
        let libname = artifact_library_name(project, target_profile);
        record("def.FinalArtifactLibraryName", &format!("assembly={name};libname={libname}"));
        record(
            "def.FinalArtifactNames",
            &format!(
                "assembly={name};exe={};shared={};static={};import={}",
                fs_join(&project.outputs.bin_dir, &format!("{name}{}", exe_suffix(target_profile))),
                fs_join(
                    &project.outputs.bin_dir,
                    &format!("{libname}{}", shared_lib_suffix(target_profile))
                ),
                fs_join(
                    &project.outputs.lib_dir,
                    &format!("{libname}{}", static_lib_suffix(target_profile))
                ),
                fs_join(
                    &project.outputs.lib_dir,
                    &format!("{libname}{}", import_lib_suffix(target_profile))
                ),
            ),
        );
        record("def.ArtifactPathContext", &format!("assembly={name};root={}", project.outputs.root));
        record("def.IRSet", &format!("assembly={name};emit_ir={emit_ir};count={ir_count}"));
        record(
            "def.PrimaryArtifactSet",
            &format!(
                "assembly={name};linkable={};count={};value={}",
                project.assembly.is_linkable(),
                primary.is_some() as u8,
                opt(primary.as_deref())
            ),
        );
        record(
            "def.ImportLibSet",
            &format!(
                "assembly={name};shared_library={};emits_import_lib={};count={};value={}",
                project.assembly.is_shared_library(),
                emits_import_lib(target_profile),
                import_lib.is_some() as u8,
                opt(import_lib.as_deref())
            ),
        );
        record(
            "def.ArtifactOutputDirectoryUse",
            &format!(
                "assembly={name};uses_bin={};uses_lib={}",
                uses_bin_dir(project),
                project.assembly.is_static_library() || import_lib.is_some()
            ),
        );
        record(
            "def.RequiredOutputs",
            &format!(
                "assembly={name};objs={obj_count};irs={ir_count};primary={};import_lib={};total={};first={}",
                primary.is_some() as u8,
                import_lib.is_some() as u8,
                out.len(),
                first_path(&out)
            ),
        );
    }
    out
}

/// True when every required output lies under the assembly's output root.
pub fn output_hygiene(project: &Project, target_profile: TargetProfile) -> bool {
    let name = &project.assembly.name;
    let root = &project.outputs.root;
    record_output_path_layout(
        name,
        &project.outputs,
        output_root_source(project.assembly.out_dir.as_deref()),
    );
    let required = required_outputs(project, target_profile);
    let root_norm = normalize(root);
    if let Some(failed) = required.iter().find(|path| !prefix(&root_norm, &normalize(path))) {
        record(
            "def.OutputHygiene",
            &format!("assembly={name};all_under_root=false;root={root};failed={failed}"),
        );
        return false;
    }
    record(
        "def.OutputHygiene",
        &format!("assembly={name};all_under_root=true;root={root};count={}", required.len()),
    );
    true
}

/// The `--dump` report: project summary, per-module outputs, link output, and source files.
pub fn dump_project(project: &Project, target_profile: TargetProfile, dump_files: bool) -> Vec<String> {
    let assembly = &project.assembly;
    let mut out = Vec::with_capacity(9 + project.modules.len());
    record(
        "def.DumpProjectOutput",
        if dump_files {
            "sections=ProjectSummary,OutputSummary,LinkOutputSummary,Files"
        } else {
            "sections=ProjectSummary,OutputSummary,LinkOutputSummary"
        },
    );
    record(
        "def.AssemblyAndLinkKinds",
        &format!(
            "assembly={};assembly_kind={};link_kind={};executable={};library={};dependency={};linkable={};shared_library={};static_library={}",
            assembly.name,
            assembly.kind,
            opt(assembly.link_kind.as_deref()),
            assembly.is_executable(),
            assembly.is_library(),
            assembly.is_dependency(),
            assembly.is_linkable(),
            assembly.is_shared_library(),
            assembly.is_static_library()
        ),
    );
    let assembly_names: Vec<&str> = project.assemblies.iter().map(|a| a.name.as_str()).collect();
    let module_names: Vec<&str> = project.modules.iter().map(|m| m.path.as_str()).collect();
    out.push(format!("<project_root, {}>", project.root));
    out.push(format!("<assemblies, [{}]>", assembly_names.join(", ")));
    out.push(format!("<assembly_name, {}>", assembly.name));
    out.push(format!("<assembly_kind, {}>", assembly.kind));
    out.push(format!("<link_kind, {}>", opt(assembly.link_kind.as_deref())));
    out.push(format!("<source_root, {}>", project.source_root));
    out.push(format!("<output_root, {}>", project.outputs.root));
    out.push(format!("<module_list, [{}]>", module_names.join(", ")));
    record(
        "def.ProjectSummaryOutput",
        "fields=project_root,assemblies,assembly_name,assembly_kind,link_kind,source_root,output_root,module_list",
    );
    let emit_ir = assembly.emit_ir.as_deref().filter(|mode| *mode != "none");
    let emit_ir_mode = emit_ir.unwrap_or("none");
    record("def.OutputSummary", &format!("fields=module,obj,ir;rows={}", project.modules.len()));
    for module in &project.modules {
        let obj = obj_path(project, target_profile, module);
        let ir_value =
            emit_ir.map_or_else(|| "<bottom>".to_string(), |mode| ir_path(project, module, mode));
        record(
            "def.IROpt",
            &format!("module={};emit_ir={emit_ir_mode};value={ir_value}", module.path),
        );
        out.push(format!("<module, {}, obj, {obj}, ir, {ir_value}>", module.path));
    }
    if assembly.is_executable() || assembly.is_library() {
        let primary = primary_artifact_path(project, target_profile);
        let import_lib = import_lib_path(project, target_profile);
        let primary_value = opt(primary.as_deref());
        let import_lib_value = opt(import_lib.as_deref());
        record(
            "def.LinkOutputSummary",
            &format!("fields=artifact,import_lib;artifact={primary_value};import_lib={import_lib_value}"),
        );
        record("def.ImportLibOpt", &format!("value={import_lib_value}"));
        out.push(format!("<artifact, {primary_value}, import_lib, {import_lib_value}>"));
    } else {
        record("def.LinkOutputSummary", "fields=artifact,import_lib;rows=0");
    }
    if dump_files {
        for module in &project.modules {
            for file in compilation_unit(&module.dir).files {
                out.push(format!("file:{file}"));
            }
        }
    }
    out
}
