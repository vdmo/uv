use std::path::Path;

use uv_core::symbols::{mangle, string_of_path};

use crate::fs_path::{fs_join, parent_path};
use crate::manifest::start_dir_for_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SourceLanguage {
    #[default]
    Ultraviolet,
}

pub struct LanguageProfile {
    pub language: SourceLanguage,
    pub display_name: &'static str,
    pub manifest_name: &'static str,
    pub source_extension: &'static str,
    pub runtime_root: &'static str,
    pub lower_name: &'static str,
    pub runtime_static_lib_elf: &'static str,
    pub runtime_static_lib_coff: &'static str,
    pub linux_startup_object_x86_64_sysv: &'static str,
    pub linux_runtime_support_sidecar: &'static str,
    pub library_entry_symbol: &'static str,
    pub library_ctor_symbol: &'static str,
    pub library_dtor_symbol: &'static str,
    pub library_attached_symbol: &'static str,
    pub image_panic_record_symbol: &'static str,
    pub host_abi_version_symbol: &'static str,
    pub host_session_create_symbol: &'static str,
    pub host_session_destroy_symbol: &'static str,
    pub host_session_owner_token_symbol: &'static str,
    pub host_runtime_alloc_symbol: &'static str,
    pub host_runtime_free_symbol: &'static str,
    pub host_runtime_register_symbol: &'static str,
    pub host_runtime_try_enter_symbol: &'static str,
    pub host_runtime_leave_symbol: &'static str,
    pub host_runtime_try_retire_symbol: &'static str,
    pub host_runtime_abort_live_symbol: &'static str,
    pub host_runtime_current_env_symbol: &'static str,
    pub host_runtime_enter_retired_symbol: &'static str,
    pub host_runtime_leave_retired_symbol: &'static str,
    pub host_runtime_abort_retired_symbol: &'static str,
    pub raw_dylib_resolve_symbol: &'static str,
    pub runtime_init_mangle_prefix: &'static str,
    pub runtime_deinit_mangle_prefix: &'static str,
    pub runtime_symbol_prefix: &'static str,
    pub concurrency_symbol_prefix: &'static str,
    pub region_active_alias: &'static str,
    pub hosted_session_param_name: &'static str,
}

static ULTRAVIOLET_PROFILE: LanguageProfile = LanguageProfile {
    language: SourceLanguage::Ultraviolet,
    display_name: "Ultraviolet",
    manifest_name: "Ultraviolet.toml",
    source_extension: ".uv",
    runtime_root: "ultraviolet",
    lower_name: "ultraviolet",
    runtime_static_lib_elf: "UltravioletRT.a",
    runtime_static_lib_coff: "UltravioletRT.lib",
    linux_startup_object_x86_64_sysv: "uv_start_x86_64_sysv.o",
    linux_runtime_support_sidecar: "libUltravioletRTSupport.so",
    library_entry_symbol: "__ultraviolet_library_entry",
    library_ctor_symbol: "__uv_library_ctor",
    library_dtor_symbol: "__uv_library_dtor",
    library_attached_symbol: "__uv_library_attached",
    image_panic_record_symbol: "__uv_image_panic_record",
    host_abi_version_symbol: "__ultraviolet_host_abi_version",
    host_session_create_symbol: "__ultraviolet_host_session_create",
    host_session_destroy_symbol: "__ultraviolet_host_session_destroy",
    host_session_owner_token_symbol: "__uv_host_session_owner_token",
    host_runtime_alloc_symbol: "uv_host_alloc",
    host_runtime_free_symbol: "uv_host_free",
    host_runtime_register_symbol: "uv_host_session_register",
    host_runtime_try_enter_symbol: "uv_host_session_try_enter",
    host_runtime_leave_symbol: "uv_host_session_leave",
    host_runtime_try_retire_symbol: "uv_host_session_try_retire",
    host_runtime_abort_live_symbol: "uv_host_session_abort_live",
    host_runtime_current_env_symbol: "uv_host_session_current_env",
    host_runtime_enter_retired_symbol: "uv_host_session_enter_retired",
    host_runtime_leave_retired_symbol: "uv_host_session_leave_retired",
    host_runtime_abort_retired_symbol: "uv_host_session_abort_retired",
    raw_dylib_resolve_symbol: "uv_raw_dylib_resolve",
    runtime_init_mangle_prefix: "ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3a",
    runtime_deinit_mangle_prefix: "ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3a",
    runtime_symbol_prefix: "__uv_",
    concurrency_symbol_prefix: "uv_",
    region_active_alias: "__uv_region_active",
    hosted_session_param_name: "__ultraviolet_session",
};

pub fn ultraviolet_language_profile() -> &'static LanguageProfile {
    &ULTRAVIOLET_PROFILE
}

pub fn language_profile_for(language: SourceLanguage) -> &'static LanguageProfile {
    match language {
        SourceLanguage::Ultraviolet => &ULTRAVIOLET_PROFILE,
    }
}

/// Ultraviolet is the only source language, so the active profile is fixed.
pub fn active_language_profile() -> &'static LanguageProfile {
    &ULTRAVIOLET_PROFILE
}

pub fn detect_source_language_for_input(input_path: &str) -> Option<SourceLanguage> {
    let extension = Path::new(input_path)
        .extension()
        .map(|ext| format!(".{}", ext.to_string_lossy()))
        .unwrap_or_default();
    if extension == ULTRAVIOLET_PROFILE.source_extension {
        return Some(SourceLanguage::Ultraviolet);
    }
    let mut current = start_dir_for_input(input_path).0;
    loop {
        if Path::new(&fs_join(&current, ULTRAVIOLET_PROFILE.manifest_name)).exists() {
            return Some(SourceLanguage::Ultraviolet);
        }
        let parent = parent_path(&current);
        if parent.is_empty() || parent == current {
            break;
        }
        current = parent;
    }
    None
}

pub fn language_path_sig(tail: &[&str]) -> String {
    let mut path = vec![active_language_profile().runtime_root];
    path.extend_from_slice(tail);
    mangle(&string_of_path(&path))
}

pub fn runtime_path_sig(tail: &[&str]) -> String {
    let mut path = vec![active_language_profile().runtime_root, "runtime"];
    path.extend_from_slice(tail);
    mangle(&string_of_path(&path))
}
