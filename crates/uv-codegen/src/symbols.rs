//! Symbol names of items. See `symbols/mangle.cpp` and `globals/init.cpp`.

use uv_core::symbols::{mangle, string_of_path};
use uv_project::language_profile::active_language_profile;

/// `ScopedSym`: the mangled `::`-path of an item.
pub fn scoped_sym<S: AsRef<str>>(path: &[S]) -> String {
    mangle(&string_of_path(path))
}

pub fn item_path_proc(module_path: &[String], name: &str) -> Vec<String> {
    module_path.iter().cloned().chain([name.to_string()]).collect()
}

fn runtime_path(kind: &str, module_path: &[String]) -> Vec<String> {
    [active_language_profile().runtime_root.to_string(), "runtime".to_string(), kind.to_string()].into_iter().chain(module_path.iter().cloned()).collect()
}

/// The initialisation procedure of a module.
pub fn init_sym(module_path: &[String]) -> String {
    mangle(&string_of_path(&runtime_path("init", module_path)))
}

/// The deinitialisation procedure of a module.
pub fn deinit_sym(module_path: &[String]) -> String {
    mangle(&string_of_path(&runtime_path("deinit", module_path)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_the_reference() {
        let module = vec!["ExecutableMain".to_string()];
        assert_eq!(scoped_sym(&item_path_proc(&module, "main")), "ExecutableMain_x3a_x3amain");
        assert_eq!(init_sym(&module), "ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExecutableMain");
        assert_eq!(deinit_sym(&module), "ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExecutableMain");
    }
}
