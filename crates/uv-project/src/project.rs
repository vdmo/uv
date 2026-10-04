use crate::language_profile::SourceLanguage;
use crate::module_discovery::ModuleInfo;
use crate::outputs::OutputPaths;
use crate::target_profile::TargetProfile;

#[derive(Debug, Clone, Default)]
pub struct ToolchainConfig {
    pub llvm_bin: Option<String>,
    pub runtime_lib: Option<String>,
    pub target_profile: Option<TargetProfile>,
}

#[derive(Debug, Clone, Copy)]
pub struct BuildConfig {
    pub incremental: bool,
    pub progress: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig { incremental: false, progress: true }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValidatedAssembly {
    pub name: String,
    pub kind: String,
    pub link_kind: Option<String>,
    pub root: String,
    pub out_dir: Option<String>,
    pub emit_ir: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Assembly {
    pub name: String,
    pub kind: String,
    pub link_kind: Option<String>,
    pub root: String,
    pub out_dir: Option<String>,
    pub emit_ir: Option<String>,
    pub source_root: String,
    pub outputs: OutputPaths,
    pub modules: Vec<ModuleInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct Project {
    pub language: SourceLanguage,
    pub root: String,
    pub assemblies: Vec<Assembly>,
    pub assembly: Assembly,
    pub source_root: String,
    pub outputs: OutputPaths,
    pub modules: Vec<ModuleInfo>,
    pub lifecycle_modules: Vec<ModuleInfo>,
    pub test_harness_entry_module: Option<String>,
    pub toolchain: ToolchainConfig,
    pub build: BuildConfig,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyTarget {
    pub name: Option<String>,
}
