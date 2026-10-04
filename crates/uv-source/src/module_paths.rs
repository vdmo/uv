//! Resolution of module paths named by `import` and `using` declarations.

use std::collections::HashSet;

use uv_core::spec_rule;
use uv_core::symbols::string_of_path;

pub type ModuleNames = HashSet<String>;

pub fn has_module_name(module_names: &ModuleNames, path: &[String]) -> bool {
    module_names.contains(&string_of_path(path))
}

/// A path names a module directly, or relative to the assembly of the current module.
pub fn resolve_import_module_path(
    current_module: &[String],
    module_names: &ModuleNames,
    path: &[String],
) -> Option<Vec<String>> {
    if path.is_empty() {
        return None;
    }
    if has_module_name(module_names, path) {
        spec_rule!("Resolve-Import-Direct");
        return Some(path.to_vec());
    }
    if let Some(assembly) = current_module.first() {
        let mut candidate = Vec::with_capacity(path.len() + 1);
        candidate.push(assembly.clone());
        candidate.extend(path.iter().cloned());
        if has_module_name(module_names, &candidate) {
            spec_rule!("Resolve-Import-Current");
            return Some(candidate);
        }
    }
    spec_rule!("Resolve-Import-Err");
    None
}
