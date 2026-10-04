#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetProfile {
    X86_64SysV,
    X86_64Win64,
    AArch64AAPCS64,
    AArch64Darwin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectFormat {
    Coff,
    Elf,
    MachO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    X86_64,
    AArch64,
}

use TargetProfile::*;

pub fn target_profile_name(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64SysV => "x86_64-sysv",
        X86_64Win64 => "x86_64-win64",
        AArch64AAPCS64 => "aarch64-aapcs64",
        AArch64Darwin => "aarch64-darwin",
    }
}

pub fn parse_target_profile(value: &str) -> Option<TargetProfile> {
    match value {
        "x86_64-sysv" => Some(X86_64SysV),
        "x86_64-win64" => Some(X86_64Win64),
        "aarch64-aapcs64" => Some(AArch64AAPCS64),
        "aarch64-darwin" => Some(AArch64Darwin),
        _ => None,
    }
}

pub fn target_arch_of(profile: TargetProfile) -> TargetArch {
    match profile {
        X86_64SysV | X86_64Win64 => TargetArch::X86_64,
        AArch64AAPCS64 | AArch64Darwin => TargetArch::AArch64,
    }
}

pub fn ptr_size_bytes(_profile: TargetProfile) -> usize {
    8
}

pub fn obj_ext(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64Win64 => ".obj",
        _ => ".o",
    }
}

pub fn exe_suffix(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64Win64 => ".exe",
        _ => "",
    }
}

pub fn library_prefix(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64Win64 => "",
        _ => "lib",
    }
}

pub fn shared_lib_suffix(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64SysV | AArch64AAPCS64 => ".so",
        AArch64Darwin => ".dylib",
        X86_64Win64 => ".dll",
    }
}

pub fn static_lib_suffix(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64Win64 => ".lib",
        _ => ".a",
    }
}

pub fn import_lib_suffix(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64SysV | AArch64AAPCS64 => ".so.import",
        AArch64Darwin => ".dylib.import",
        X86_64Win64 => ".lib",
    }
}

pub fn emits_import_lib(profile: TargetProfile) -> bool {
    profile == X86_64Win64
}

pub fn llvm_triple_of(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64SysV => "x86_64-unknown-linux-gnu",
        X86_64Win64 => "x86_64-pc-windows-msvc",
        AArch64AAPCS64 => "aarch64-unknown-linux-gnu",
        AArch64Darwin => "arm64-apple-macosx14.0.0",
    }
}

pub fn llvm_data_layout_of(profile: TargetProfile) -> &'static str {
    match profile {
        X86_64SysV => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128",
        X86_64Win64 => "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
        AArch64AAPCS64 => "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
        AArch64Darwin => "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32",
    }
}

pub fn object_format_of(profile: TargetProfile) -> ObjectFormat {
    match profile {
        X86_64SysV | AArch64AAPCS64 => ObjectFormat::Elf,
        X86_64Win64 => ObjectFormat::Coff,
        AArch64Darwin => ObjectFormat::MachO,
    }
}

pub fn supports_shared_libraries(_profile: TargetProfile) -> bool {
    true
}

pub fn supports_hosted_libraries(_profile: TargetProfile) -> bool {
    true
}

pub fn library_kind_supported(kind: &str, profile: TargetProfile) -> bool {
    match kind {
        "raw-dylib" => profile == X86_64Win64,
        "framework" => false,
        _ => kind == "dylib" || kind == "static",
    }
}

pub fn resolve_library_name(kind: &str, name: &str, profile: TargetProfile) -> Option<String> {
    if !library_kind_supported(kind, profile) {
        return None;
    }
    match kind {
        "dylib" => Some(format!("{}{name}{}", library_prefix(profile), shared_lib_suffix(profile))),
        "static" => Some(format!("{}{name}{}", library_prefix(profile), static_lib_suffix(profile))),
        "raw-dylib" if profile == X86_64Win64 => Some(format!("{name}.dll")),
        _ => None,
    }
}
