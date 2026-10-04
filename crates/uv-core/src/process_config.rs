//! Process-wide configuration set by the CLI and the project manifest.

use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::behavior_model::ErrorRecoveryPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Verbosity {
    #[default]
    Normal,
    Verbose,
}

#[derive(Default)]
struct ProcessConfigState {
    build_progress: Option<bool>,
    incremental: Option<bool>,
    runtime_lib: Option<String>,
    link_debug: Option<bool>,
    out_dir: Option<String>,
    max_errors: Option<ErrorRecoveryPolicy>,
    debug_subsystems: HashSet<String>,
    has_debug: bool,
    manifest_build_progress: Option<bool>,
    manifest_incremental: Option<bool>,
    manifest_runtime_lib: Option<String>,
    verbosity: Verbosity,
}

fn state() -> MutexGuard<'static, ProcessConfigState> {
    static STATE: OnceLock<Mutex<ProcessConfigState>> = OnceLock::new();
    STATE
        .get_or_init(|| Mutex::new(ProcessConfigState::default()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

macro_rules! config_accessors {
    ($($setter:ident, $getter:ident, $field:ident: $ty:ty;)*) => {
        $(
            pub fn $setter(value: Option<$ty>) {
                state().$field = value;
            }

            pub fn $getter() -> Option<$ty> {
                state().$field.clone()
            }
        )*
    };
}

config_accessors! {
    set_build_progress_override, build_progress_override, build_progress: bool;
    set_incremental_override, incremental_override, incremental: bool;
    set_runtime_lib_override, runtime_lib_override, runtime_lib: String;
    set_link_debug_override, link_debug_override, link_debug: bool;
    set_out_dir_override, out_dir_override, out_dir: String;
    set_max_errors_override, max_errors_override, max_errors: ErrorRecoveryPolicy;
    set_manifest_build_progress, manifest_build_progress, manifest_build_progress: bool;
    set_manifest_incremental, manifest_incremental, manifest_incremental: bool;
    set_manifest_runtime_lib, manifest_runtime_lib, manifest_runtime_lib: String;
}

pub fn set_debug_subsystems(subsystems: &[String]) {
    let mut state = state();
    state.debug_subsystems.clear();
    state.has_debug = !subsystems.is_empty();
    state.debug_subsystems.extend(subsystems.iter().cloned());
}

pub fn is_debug_enabled(subsystem: &str) -> bool {
    let state = state();
    if !state.has_debug {
        return false;
    }
    state.debug_subsystems.contains("all") || state.debug_subsystems.contains(subsystem)
}

pub fn has_debug_subsystems() -> bool {
    state().has_debug
}

pub fn set_verbosity(level: Verbosity) {
    state().verbosity = level;
}

pub fn get_verbosity() -> Verbosity {
    state().verbosity
}
