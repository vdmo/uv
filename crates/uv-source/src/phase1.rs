//! Phase 1 for a whole project: the order in which assemblies are parsed and the checks
//! that gate each of them.

use std::collections::{HashMap, HashSet};

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, has_error, Diagnostic, DiagnosticStream, Severity};
use uv_core::source_text::SourceFile;
use uv_core::span::Span;
use uv_core::symbols::string_of_path;
use uv_project::module_discovery::ModuleInfo;
use uv_project::project::{Assembly, Project};

use crate::ast::{ASTItem, ASTModule, UsingClause};
use crate::attributes::{
    attrs, has_attribute, validate_attributes, AttributeTarget, AttributeValidationResult,
};
use crate::module_paths::{resolve_import_module_path, ModuleNames};
use crate::parse_modules::{parse_modules, ParseModuleDeps, UnsafeSpanMap};

pub fn emit_internal_diagnostic(diags: &mut DiagnosticStream, span: Option<Span>, message: &str) {
    let diag = Diagnostic {
        severity: Severity::Error,
        span,
        message: message.to_string(),
        ..Diagnostic::default()
    };
    emit(diags, diag);
}

pub fn emit_attribute_validation_diagnostic(diags: &mut DiagnosticStream, err: AttributeValidationResult) {
    let code = err.diag_id.unwrap_or("E-MOD-2450");
    match make_diagnostic_by_id(code, err.span.clone()) {
        Some(mut diag) => {
            if !err.message.is_empty() {
                diag.message = err.message;
            }
            emit(diags, diag);
        }
        None => {
            let message =
                if err.message.is_empty() { "Attribute validation failed." } else { &err.message };
            emit_internal_diagnostic(diags, err.span, message);
        }
    }
}

/// Validates the attribute lists of type declarations that carry `#derive`, which the
/// compile-time phase reads before semantic analysis sees them.
pub fn validate_parsed_type_attribute_lists(modules: &[ASTModule], diags: &mut DiagnosticStream) -> bool {
    for item in modules.iter().flat_map(|module| &module.items) {
        let (attr_list, target) = match item {
            ASTItem::RecordDecl(decl) => (&decl.attrs, AttributeTarget::Record),
            ASTItem::EnumDecl(decl) => (&decl.attrs, AttributeTarget::Enum),
            ASTItem::ModalDecl(decl) => (&decl.attrs, AttributeTarget::Modal),
            _ => continue,
        };
        if !has_attribute(attr_list, attrs::DERIVE) {
            continue;
        }
        let result = validate_attributes(attr_list, target);
        if !result.ok {
            emit_attribute_validation_diagnostic(diags, result);
            return false;
        }
    }
    true
}

/// What happened to one assembly, for the driver's log.
pub struct AssemblyOutcome {
    pub ok: bool,
    pub attribute_validation_failed: bool,
    pub parsed_modules: usize,
    pub emitted_diags: usize,
}

/// Receives progress events; the driver turns them into log lines.
pub trait Phase1Observer {
    fn assembly_start(&self, assembly: &Assembly);
    fn assembly_finish(&self, assembly: &Assembly, outcome: AssemblyOutcome);
}

/// Parses one assembly and runs the checks that gate it. Returns its modules, or `None`
/// when the assembly stops phase 1.
fn parse_assembly(
    assembly: &Assembly,
    observer: &dyn Phase1Observer,
    result: &mut Phase1Result,
) -> Option<Vec<ASTModule>> {
    observer.assembly_start(assembly);
    let inspect_source = |_: &SourceFile| DiagnosticStream::new();
    let deps = ParseModuleDeps { inspect_source: Some(&inspect_source), ..ParseModuleDeps::default() };
    let parsed = parse_modules(&assembly.modules, &assembly.source_root, &assembly.name, &deps);
    let emitted_diags = parsed.diags.len();
    let has_errors = has_error(&parsed.diags);
    for diag in parsed.diags {
        emit(&mut result.diags, diag);
    }
    let finish = |ok: bool, attribute_validation_failed: bool, parsed_modules: usize| {
        observer.assembly_finish(
            assembly,
            AssemblyOutcome { ok, attribute_validation_failed, parsed_modules, emitted_diags },
        );
    };
    let stage_modules = match parsed.modules {
        Some(modules) if !has_errors => modules,
        modules => {
            finish(false, false, modules.map_or(0, |modules| modules.len()));
            return None;
        }
    };
    result.unsafe_spans_by_file.extend(parsed.unsafe_spans_by_file);
    if !validate_parsed_type_attribute_lists(&stage_modules, &mut result.diags) {
        finish(false, true, stage_modules.len());
        return None;
    }
    finish(true, false, stage_modules.len());
    Some(stage_modules)
}

/// The assembly that owns the module an `import` or `using` declaration names.
fn imported_assembly_name<'a>(
    item: &ASTItem,
    current_module: &[String],
    module_names: &ModuleNames,
    module_owner: &'a HashMap<String, String>,
) -> Option<&'a str> {
    let path = match item {
        ASTItem::ImportDecl(import) => &import.path,
        ASTItem::UsingDecl(using) => match &using.clause {
            UsingClause::UsingItem(clause) => &clause.module_path,
            UsingClause::UsingList(clause) => &clause.module_path,
            UsingClause::UsingWildcard(clause) => &clause.module_path,
        },
        _ => return None,
    };
    let resolved = resolve_import_module_path(current_module, module_names, path)?;
    module_owner.get(&string_of_path(&resolved)).map(String::as_str)
}

#[derive(Default)]
pub struct Phase1Result {
    pub ok: bool,
    /// Every parsed module: first those of the assemblies reachable from the selected
    /// assembly, in parse order, then those of the remaining assemblies.
    pub project_modules: Vec<ASTModule>,
    /// The project description of each entry of `project_modules`.
    pub project_module_infos: Vec<ModuleInfo>,
    /// How many leading entries of `project_modules` are reachable.
    pub reachable_count: usize,
    pub unsafe_spans_by_file: UnsafeSpanMap,
    pub diags: DiagnosticStream,
}

impl Phase1Result {
    pub fn reachable_modules(&self) -> &[ASTModule] {
        &self.project_modules[..self.reachable_count]
    }

    pub fn reachable_module_infos(&self) -> &[ModuleInfo] {
        &self.project_module_infos[..self.reachable_count]
    }
}

/// Parses the selected assembly, then every assembly its modules import (transitively),
/// then the remaining assemblies in manifest order.
pub fn run_phase1(project: &Project, observer: &dyn Phase1Observer) -> Phase1Result {
    let assembly_by_name: HashMap<&str, &Assembly> =
        project.assemblies.iter().map(|assembly| (assembly.name.as_str(), assembly)).collect();
    let mut all_module_names = ModuleNames::new();
    let mut module_owner: HashMap<String, String> = HashMap::new();
    for assembly in &project.assemblies {
        for module in &assembly.modules {
            all_module_names.insert(module.path.clone());
            module_owner.entry(module.path.clone()).or_insert_with(|| assembly.name.clone());
        }
    }
    let mut result = Phase1Result { ok: true, ..Phase1Result::default() };
    let mut pending: Vec<&str> = vec![project.assembly.name.as_str()];
    let mut seen: HashSet<&str> = HashSet::from([project.assembly.name.as_str()]);
    let mut index = 0;
    while index < pending.len() {
        let Some(assembly) = assembly_by_name.get(pending[index]) else {
            index += 1;
            continue;
        };
        index += 1;
        let Some(stage_modules) = parse_assembly(assembly, observer, &mut result) else {
            result.ok = false;
            return result;
        };
        for module in &stage_modules {
            for item in &module.items {
                let imported =
                    imported_assembly_name(item, &module.path, &all_module_names, &module_owner);
                if let Some(name) = imported.filter(|name| assembly_by_name.contains_key(name)) {
                    if seen.insert(name) {
                        pending.push(name);
                    }
                }
            }
        }
        result.project_modules.extend(stage_modules);
        result.project_module_infos.extend(assembly.modules.iter().cloned());
    }
    result.reachable_count = result.project_modules.len();
    for assembly in &project.assemblies {
        if seen.contains(assembly.name.as_str()) {
            continue;
        }
        let Some(stage_modules) = parse_assembly(assembly, observer, &mut result) else {
            result.ok = false;
            return result;
        };
        result.project_modules.extend(stage_modules);
        result.project_module_infos.extend(assembly.modules.iter().cloned());
    }
    result
}
