//! Checks that `import` declarations name modules that exist.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_source::ast::*;
use uv_source::module_paths::{resolve_import_module_path, ModuleNames};

/// A module cannot import itself, and the path must resolve.
pub fn validate_import_decl(import: &ImportDecl, current_module: &[String], module_names: &ModuleNames) -> Option<&'static str> {
    let resolves = !import.path.is_empty()
        && import.path != current_module
        && resolve_import_module_path(current_module, module_names, &import.path).is_some();
    (!resolves).then_some("Resolve-Import-Err")
}

pub fn validate_module_imports(module: &ASTModule, module_names: &ModuleNames) -> DiagnosticStream {
    let mut diags = DiagnosticStream::new();
    for item in &module.items {
        let ASTItem::ImportDecl(import) = item else {
            continue;
        };
        if let Some(diag_id) = validate_import_decl(import, &module.path, module_names) {
            if let Some(diag) = make_diagnostic_by_id(diag_id, Some(import.span.clone())) {
                emit(&mut diags, diag);
            }
        }
    }
    diags
}
