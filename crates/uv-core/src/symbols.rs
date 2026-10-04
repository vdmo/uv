//! Symbol-name mangling shared by output paths and code generation.

use crate::unicode::nfc;

pub fn string_of_path<S: AsRef<str>>(comps: &[S]) -> String {
    comps.iter().map(AsRef::as_ref).collect::<Vec<_>>().join("::")
}

pub fn path_string<S: AsRef<str>>(comps: &[S]) -> String {
    string_of_path(comps)
}

/// ASCII alphanumerics pass through; every other byte of the NFC form becomes `_xHH`.
pub fn path_to_prefix(s: &str) -> String {
    let normalized = nfc(s);
    let mut out = String::with_capacity(normalized.len());
    for byte in normalized.bytes() {
        if byte.is_ascii_alphanumeric() {
            out.push(byte as char);
        } else {
            out.push_str(&format!("_x{byte:02x}"));
        }
    }
    out
}

pub fn mangle(s: &str) -> String {
    path_to_prefix(s)
}

pub fn path_sig<S: AsRef<str>>(comps: &[S]) -> String {
    mangle(&path_string(comps))
}

pub fn mangle_module_path(module_path: &str) -> String {
    let parts: Vec<String> = module_path.split("::").map(nfc).collect();
    mangle(&path_string(&parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mangles_module_paths() {
        assert_eq!(mangle_module_path("Shapes::Geometry"), "Shapes_x3a_x3aGeometry");
        assert_eq!(mangle("a_b"), "a_x5fb");
    }
}
