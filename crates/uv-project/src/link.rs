//! The tools that turn LLVM IR into programs: tool resolution, assembly of IR to objects, and
//! the linker driver (`01_project/tool_resolution.cpp`, `ir_assembly.cpp`, `link.cpp`), for the
//! targets that use ELF objects.
//!
//! The reference compiles each module in process with LLVM. Here a module is assembled to
//! bitcode with `llvm-as` and turned into a relocatable object by `ld.lld -r`, which runs the
//! code generator of LLVM's link-time optimisation.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::language_profile::active_language_profile;
use crate::project::Project;
use crate::target_profile::{object_format_of, ObjectFormat, TargetProfile};

/// The directory the compiler's own files are found under: the one that holds the platform
/// directories (`linux/tools`, `linux/runtime`, `linux/lib`) beside the compiler, or the legacy
/// layout (`tools`, `runtime`, `lib` beside the compiler or its parent).
pub fn compiler_support_root() -> Option<PathBuf> {
    if let Some(root) = std::env::var_os("UVC_SUPPORT_ROOT") {
        return Some(PathBuf::from(root));
    }
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?.to_path_buf();
    let packaged = |dir: &Path| ["windows", "macos", "linux"].iter().any(|platform| ["tools", "bin", "lib"].iter().any(|sub| dir.join(platform).join(sub).is_dir()));
    let legacy = |dir: &Path| ["runtime", "tools", "bin", "lib"].iter().any(|sub| dir.join(sub).is_dir());
    if packaged(&dir) || legacy(&dir) {
        return Some(dir);
    }
    dir.parent().filter(|parent| legacy(parent)).map(Path::to_path_buf)
}

fn platform_dir(profile: TargetProfile) -> &'static str {
    match object_format_of(profile) {
        ObjectFormat::Elf => "linux",
        ObjectFormat::MachO => "macos",
        ObjectFormat::Coff => "windows",
    }
}

/// The directory of the compiler's tools for a target.
fn support_subdir(profile: TargetProfile, sub: &str) -> Option<PathBuf> {
    let root = compiler_support_root()?;
    let packaged = root.join(platform_dir(profile)).join(sub);
    if packaged.is_dir() {
        return Some(packaged);
    }
    let legacy = root.join(sub);
    legacy.is_dir().then_some(legacy)
}

/// `ResolveTool`: where a tool of the toolchain is: the `llvm_bin` of the manifest when it
/// names a directory, else the compiler's tool directory, else the search path.
pub fn resolve_tool(project: &Project, profile: TargetProfile, name: &str) -> Option<PathBuf> {
    let candidates: Vec<String> = if cfg!(windows) { vec![format!("{name}.exe"), name.to_string()] } else { vec![name.to_string()] };
    let mut dirs: Vec<PathBuf> = Vec::new();
    match project.toolchain.llvm_bin.as_deref().filter(|dir| !dir.is_empty()) {
        Some(dir) => {
            let path = Path::new(dir);
            dirs.push(if path.is_absolute() { path.to_path_buf() } else { Path::new(&project.root).join(path) });
        }
        None => {
            if let Some(dir) = support_subdir(profile, "tools") {
                dirs.push(dir);
            }
            if let Some(path) = std::env::var_os("PATH") {
                dirs.extend(std::env::split_paths(&path));
            }
        }
    }
    dirs.iter().flat_map(|dir| candidates.iter().map(move |candidate| dir.join(candidate))).find(|path| path.is_file())
}

/// The name of the linker for a target.
pub fn linker_tool_name(profile: TargetProfile) -> &'static str {
    match object_format_of(profile) {
        ObjectFormat::Coff => "lld-link",
        ObjectFormat::Elf => "ld.lld",
        ObjectFormat::MachO => "ld64.lld",
    }
}

/// `RuntimeLibPath`: the command line, then the manifest, then the compiler's own.
pub fn runtime_lib_path(project: &Project, profile: TargetProfile) -> PathBuf {
    if let Some(path) = uv_core::process_config::runtime_lib_override().filter(|path| !path.is_empty()) {
        return PathBuf::from(path);
    }
    if let Some(path) = project.toolchain.runtime_lib.as_deref().filter(|path| !path.is_empty()) {
        let path = Path::new(path);
        return if path.is_absolute() { path.to_path_buf() } else { Path::new(&project.root).join(path) };
    }
    let name = match object_format_of(profile) {
        ObjectFormat::Coff => active_language_profile().runtime_static_lib_coff,
        _ => active_language_profile().runtime_static_lib_elf,
    };
    let exe_dir = std::env::current_exe().ok().and_then(|exe| exe.parent().map(Path::to_path_buf)).unwrap_or_default();
    exe_dir.join(name)
}

/// The shared files an executable needs next to it: the runtime's support library and ICU.
/// `None` when one is missing.
pub fn runtime_sidecars(profile: TargetProfile, runtime_lib: &Path) -> Option<Vec<PathBuf>> {
    if object_format_of(profile) != ObjectFormat::Elf {
        return Some(Vec::new());
    }
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(dir) = support_subdir(profile, "lib") {
        roots.push(dir);
    }
    if let Some(runtime_dir) = runtime_lib.parent() {
        for dir in [runtime_dir.join("linux").join("lib"), runtime_dir.join("lib"), runtime_dir.parent().map(|parent| parent.join("lib")).unwrap_or_default()] {
            if dir.is_dir() && !roots.contains(&dir) {
                roots.push(dir);
            }
        }
    }
    let mut out = Vec::new();
    for name in [active_language_profile().linux_runtime_support_sidecar, "libicui18n.so.72", "libicuuc.so.72", "libicudata.so.72", "icudt72l.dat"] {
        out.push(roots.iter().map(|root| root.join(name)).find(|path| path.is_file())?);
    }
    Some(out)
}

/// The object the runtime starts programs with.
pub fn startup_object(project: &Project, profile: TargetProfile, runtime_lib: &Path) -> Option<PathBuf> {
    if profile != TargetProfile::X86_64SysV {
        return None;
    }
    let name = active_language_profile().linux_startup_object_x86_64_sysv;
    let mut candidates = Vec::new();
    if let Some(dir) = runtime_lib.parent() {
        candidates.push(dir.join(name));
        candidates.push(dir.join("runtime").join(name));
        candidates.push(dir.join("linux").join("runtime").join(name));
    }
    if let Some(dir) = support_subdir(profile, "runtime") {
        candidates.push(dir.join(name));
    }
    candidates.push(Path::new(&project.outputs.root).join("runtime").join(name));
    candidates.into_iter().find(|path| path.is_file())
}

/// The directories libraries are searched in (`PosixLibrarySearchDirs`).
fn library_search_dirs(profile: TargetProfile) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    let mut add = |dir: PathBuf, out: &mut Vec<PathBuf>| {
        if dir.is_dir() && !out.contains(&dir) {
            out.push(dir);
        }
    };
    match profile {
        TargetProfile::X86_64SysV => {
            let gcc_root = Path::new("/usr/lib/gcc/x86_64-linux-gnu");
            let mut versions: Vec<PathBuf> = std::fs::read_dir(gcc_root).map(|entries| entries.flatten().map(|entry| entry.path()).filter(|path| path.is_dir()).collect()).unwrap_or_default();
            versions.sort();
            for version in versions {
                add(version.clone(), &mut out);
                add(normalize(&version.join("../../../../lib64")), &mut out);
            }
            for dir in ["/lib/x86_64-linux-gnu", "/usr/lib/x86_64-linux-gnu", "/lib64", "/usr/lib64", "/lib", "/usr/lib", "/usr/local/lib"] {
                add(PathBuf::from(dir), &mut out);
            }
        }
        TargetProfile::AArch64AAPCS64 => {
            for dir in ["/lib/aarch64-linux-gnu", "/usr/lib/aarch64-linux-gnu", "/lib64", "/usr/lib64", "/lib", "/usr/lib", "/usr/local/lib"] {
                add(PathBuf::from(dir), &mut out);
            }
        }
        _ => {}
    }
    out
}

fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// What a tool run gave.
#[derive(Debug, Default)]
pub struct ToolRun {
    pub launched: bool,
    pub exit_code: i32,
    pub output: String,
}

fn run(program: &Path, args: &[String], cwd: Option<&Path>) -> ToolRun {
    let mut command = Command::new(program);
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    match command.output() {
        Ok(output) => ToolRun {
            launched: true,
            exit_code: output.status.code().unwrap_or(-1),
            output: format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)),
        },
        Err(error) => ToolRun { launched: false, exit_code: -1, output: error.to_string() },
    }
}

/// `AssembleIR` and the code generation of a module: the object file made from its LLVM IR.
pub fn compile_ir_to_object(llvm_as: &Path, linker: &Path, ir_text: &str, object: &Path, opt_level: &str) -> Result<(), String> {
    let stamp = std::process::id();
    let dir = object.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(dir).map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    let stem = object.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_else(|| "module".to_string());
    let ir_file = dir.join(format!("{stem}.{stamp}.ll"));
    let bitcode = dir.join(format!("{stem}.{stamp}.bc"));
    std::fs::write(&ir_file, ir_text).map_err(|error| format!("cannot write {}: {error}", ir_file.display()))?;
    let assembled = run(llvm_as, &[ir_file.to_string_lossy().into_owned(), "-o".to_string(), bitcode.to_string_lossy().into_owned()], None);
    let _ = std::fs::remove_file(&ir_file);
    if !assembled.launched || assembled.exit_code != 0 {
        let _ = std::fs::remove_file(&bitcode);
        return Err(format!("llvm-as failed: {}", assembled.output.trim()));
    }
    let lto_level = match opt_level {
        "O1" => "1",
        "O2" | "Os" | "Oz" => "2",
        "O3" => "3",
        _ => "0",
    };
    let compiled = run(linker, &["-r".to_string(), format!("--lto-O{lto_level}"), "-o".to_string(), object.to_string_lossy().into_owned(), bitcode.to_string_lossy().into_owned()], None);
    let _ = std::fs::remove_file(&bitcode);
    if !compiled.launched || compiled.exit_code != 0 {
        return Err(format!("code generation failed: {}", compiled.output.trim()));
    }
    Ok(())
}

/// How a link failed.
#[derive(Debug)]
pub enum LinkError {
    LinkerNotFound,
    RuntimeMissing(String),
    Failed { exit_code: i32, output: String },
}

/// `Link` for an executable: the objects, the runtime's startup object and library, and the
/// runtime's shared libraries, with the shared libraries copied next to the program.
pub fn link_executable(project: &Project, profile: TargetProfile, objects: &[PathBuf], output: &Path) -> Result<(), LinkError> {
    if object_format_of(profile) != ObjectFormat::Elf {
        return Err(LinkError::RuntimeMissing("linking for this target is not ported".to_string()));
    }
    let linker = resolve_tool(project, profile, linker_tool_name(profile)).ok_or(LinkError::LinkerNotFound)?;
    let runtime_lib = runtime_lib_path(project, profile);
    if !runtime_lib.is_file() {
        return Err(LinkError::RuntimeMissing(format!("searched for runtime library at: {}", runtime_lib.display())));
    }
    let sidecars = runtime_sidecars(profile, &runtime_lib).ok_or_else(|| LinkError::RuntimeMissing("missing Linux runtime sidecar assets under the compiler support lib directory".to_string()))?;
    let startup = if profile == TargetProfile::X86_64SysV {
        Some(startup_object(project, profile, &runtime_lib).ok_or_else(|| LinkError::RuntimeMissing(format!("missing Linux runtime startup object `{}`", active_language_profile().linux_startup_object_x86_64_sysv)))?)
    } else {
        None
    };
    let sysv = profile == TargetProfile::X86_64SysV;
    let mut args: Vec<String> = vec!["-o".to_string(), output.to_string_lossy().into_owned(), if sysv { "--entry=_start" } else { "--entry=main" }.to_string()];
    if sysv {
        args.push("--undefined=_start".to_string());
    }
    args.push("--nostdlib".to_string());
    args.push("-rpath=$ORIGIN".to_string());
    let interpreter = match profile {
        TargetProfile::X86_64SysV => Some("/lib64/ld-linux-x86-64.so.2"),
        TargetProfile::AArch64AAPCS64 => Some("/lib/ld-linux-aarch64.so.1"),
        _ => None,
    };
    if let Some(interpreter) = interpreter {
        args.push(format!("--dynamic-linker={interpreter}"));
    }
    for dir in library_search_dirs(profile) {
        args.push(format!("-L{}", dir.display()));
    }
    args.extend(objects.iter().map(|object| object.to_string_lossy().into_owned()));
    if let Some(startup) = &startup {
        args.push(startup.to_string_lossy().into_owned());
    }
    args.push(runtime_lib.to_string_lossy().into_owned());
    let linkable: Vec<String> = sidecars.iter().filter(|path| path.to_string_lossy().contains(".so")).map(|path| path.to_string_lossy().into_owned()).collect();
    if !linkable.is_empty() {
        args.push("--no-as-needed".to_string());
        args.extend(linkable);
        args.push("--as-needed".to_string());
    }
    args.push("-lm".to_string());
    args.push("-lc".to_string());
    if let Some(parent) = output.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let result = run(&linker, &args, output.parent());
    if !result.launched || result.exit_code != 0 {
        return Err(LinkError::Failed { exit_code: result.exit_code, output: result.output });
    }
    // The shared libraries the program loads are looked for next to it.
    if let Some(dir) = output.parent() {
        for sidecar in &sidecars {
            if let Some(name) = sidecar.file_name() {
                let _ = std::fs::copy(sidecar, dir.join(name));
            }
        }
    }
    Ok(())
}
