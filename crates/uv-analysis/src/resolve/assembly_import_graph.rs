//! The imports between the assemblies of a project, and what the graph must satisfy:
//! no executable is imported, no linked library imports itself through others, and a
//! hosted library imports no linked library. See `assembly_import_graph.cpp`.
//!
//! The graph keeps its tables in the iteration order of the reference's hash tables,
//! because that order decides which violation is named when there are several.

use std::collections::{HashMap, HashSet};

use uv_core::diagnostics::{emit, DiagnosticStream, SubDiagnostic, SubDiagnosticKind};
use uv_core::diagnostic_messages::make_external_diagnostic;
use uv_core::std_unordered::UnorderedMap;
use uv_core::symbols::string_of_path;
use uv_project::project::{Assembly, Project};
use uv_source::ast::{ASTItem, ASTModule};
use uv_source::attributes::{attrs, has_attribute};
use uv_source::module_paths::{resolve_import_module_path, ModuleNames};

pub struct AssemblyImportGraph<'p> {
    pub assemblies: UnorderedMap<&'p Assembly>,
    pub imports: UnorderedMap<Vec<String>>,
}

fn is_executable(assembly: &Assembly) -> bool {
    assembly.kind == "executable"
}

fn is_library(assembly: &Assembly) -> bool {
    assembly.kind == "library"
}

fn is_dependency(assembly: &Assembly) -> bool {
    assembly.kind == "dependency"
}

fn sort_strings_deterministically(values: &mut Vec<String>) {
    values.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    values.dedup();
}

fn emit_assembly_graph_diag(diags: &mut DiagnosticStream, note: String) -> bool {
    if let Some(mut diag) = make_external_diagnostic("E-PRJ-0209") {
        if !note.is_empty() {
            diag.children.push(SubDiagnostic { kind: SubDiagnosticKind::Note, message: note, ..Default::default() });
        }
        emit(diags, diag);
    }
    false
}

pub fn build_assembly_import_graph<'p>(project: &'p Project, modules: &[ASTModule]) -> AssemblyImportGraph<'p> {
    let mut graph = AssemblyImportGraph { assemblies: UnorderedMap::new(), imports: UnorderedMap::new() };
    graph.assemblies.reserve(project.assemblies.len());
    graph.imports.reserve(project.assemblies.len());
    let module_names: ModuleNames = project.assemblies.iter().flat_map(|assembly| assembly.modules.iter().map(|module| module.path.clone())).collect();
    let mut module_owner: HashMap<&str, &str> = HashMap::new();
    for assembly in &project.assemblies {
        for module in &assembly.modules {
            module_owner.entry(module.path.as_str()).or_insert(assembly.name.as_str());
        }
    }
    for assembly in &project.assemblies {
        graph.assemblies.emplace(assembly.name.clone(), assembly);
        graph.imports.emplace(assembly.name.clone(), Vec::new());
    }
    for module in modules {
        let Some(owner) = module_owner.get(string_of_path(&module.path).as_str()).copied() else {
            continue;
        };
        let mut found: Vec<String> = Vec::new();
        for item in &module.items {
            let ASTItem::ImportDecl(import) = item else {
                continue;
            };
            let Some(resolved) = resolve_import_module_path(&module.path, &module_names, &import.path) else {
                continue;
            };
            let Some(imported) = module_owner.get(string_of_path(&resolved).as_str()).copied() else {
                continue;
            };
            if imported == owner || !graph.assemblies.contains_key(imported) {
                continue;
            }
            found.push(imported.to_string());
        }
        // `graph.imports[owner]` creates the entry when it is not there.
        if graph.imports.get(owner).is_none() {
            graph.imports.insert(owner.to_string(), Vec::new());
        }
        let imports = graph.imports.get_mut(owner).expect("entry exists");
        imports.extend(found);
        sort_strings_deterministically(imports);
    }
    graph
}

fn reachable_assemblies(root: &str, graph: &AssemblyImportGraph<'_>) -> HashSet<String> {
    let mut reachable = HashSet::new();
    let mut queue = vec![root.to_string()];
    let mut index = 0;
    while index < queue.len() {
        let current = queue[index].clone();
        index += 1;
        if !reachable.insert(current.clone()) {
            continue;
        }
        let Some(deps) = graph.imports.get(&current) else {
            continue;
        };
        for dep in deps {
            if graph.assemblies.contains_key(dep) {
                queue.push(dep.clone());
            }
        }
    }
    reachable
}

pub fn validate_assembly_import_graph_structure(project: &Project, graph: &AssemblyImportGraph<'_>, diags: &mut DiagnosticStream) -> bool {
    if !graph.assemblies.contains_key(&project.assembly.name) {
        return true;
    }
    let reachable = reachable_assemblies(&project.assembly.name, graph);
    for (assembly_name, deps) in graph.imports.iter() {
        if !reachable.contains(assembly_name) {
            continue;
        }
        for dep in deps {
            if !reachable.contains(dep) {
                continue;
            }
            let Some(dep_assembly) = graph.assemblies.get(dep) else {
                continue;
            };
            if is_executable(dep_assembly) {
                return emit_assembly_graph_diag(diags, format!("imported executable assembly: {dep}"));
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Visit {
        Unseen,
        Visiting,
        Done,
    }
    fn dfs(
        name: &str,
        graph: &AssemblyImportGraph<'_>,
        state: &mut HashMap<String, Visit>,
        stack: &mut Vec<String>,
        diags: &mut DiagnosticStream,
    ) -> bool {
        state.insert(name.to_string(), Visit::Visiting);
        stack.push(name.to_string());
        if let Some(deps) = graph.imports.get(name) {
            for dep in deps {
                let Some(dep_assembly) = graph.assemblies.get(dep) else {
                    continue;
                };
                if !is_library(dep_assembly) {
                    continue;
                }
                match state.get(dep).copied().unwrap_or(Visit::Unseen) {
                    Visit::Unseen => {
                        if !dfs(dep, graph, state, stack, diags) {
                            return false;
                        }
                    }
                    Visit::Done => {}
                    Visit::Visiting => {
                        let cycle = match stack.iter().position(|entry| entry == dep) {
                            Some(start) => format!("{} -> {dep}", stack[start..].join(" -> ")),
                            None => format!("{name} -> {dep}"),
                        };
                        let has_library = stack.iter().any(|entry| entry == dep);
                        if has_library {
                            return emit_assembly_graph_diag(diags, format!("linked-library cycle: {cycle}"));
                        }
                    }
                }
            }
        }
        stack.pop();
        state.insert(name.to_string(), Visit::Done);
        true
    }
    let mut state: HashMap<String, Visit> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    for (assembly_name, assembly) in graph.assemblies.iter() {
        if !reachable.contains(assembly_name) || !is_library(assembly) {
            continue;
        }
        if state.get(assembly_name).copied().unwrap_or(Visit::Unseen) != Visit::Unseen {
            continue;
        }
        if !dfs(assembly_name, graph, &mut state, &mut stack, diags) {
            return false;
        }
    }
    true
}

/// The assemblies whose modules are emitted with `assembly_name`: itself and the
/// dependency assemblies it imports, transitively.
fn compute_emit_assembly_names(assembly_name: &str, graph: &AssemblyImportGraph<'_>) -> Vec<String> {
    fn visit(current: &str, graph: &AssemblyImportGraph<'_>, seen: &mut HashSet<String>, out: &mut Vec<String>) {
        if !seen.insert(current.to_string()) {
            return;
        }
        out.push(current.to_string());
        let Some(deps) = graph.imports.get(current) else {
            return;
        };
        for dep in deps {
            if graph.assemblies.get(dep).is_some_and(|assembly| is_dependency(assembly)) {
                visit(dep, graph, seen, out);
            }
        }
    }
    let mut out = Vec::new();
    visit(assembly_name, graph, &mut HashSet::new(), &mut out);
    out
}

fn compute_direct_library_imports(assembly_name: &str, graph: &AssemblyImportGraph<'_>) -> Vec<String> {
    let mut libraries = Vec::new();
    let mut seen = HashSet::new();
    for emit_name in compute_emit_assembly_names(assembly_name, graph) {
        let Some(deps) = graph.imports.get(&emit_name) else {
            continue;
        };
        for dep in deps {
            let Some(dep_assembly) = graph.assemblies.get(dep) else {
                continue;
            };
            if !is_library(dep_assembly) || dep == assembly_name || !seen.insert(dep.clone()) {
                continue;
            }
            libraries.push(dep.clone());
        }
    }
    sort_strings_deterministically(&mut libraries);
    libraries
}

/// The libraries `assembly_name` imports, transitively, each after those it imports.
pub fn compute_library_closure(assembly_name: &str, graph: &AssemblyImportGraph<'_>) -> Vec<String> {
    let mut libraries = Vec::new();
    let mut discovered = HashSet::new();
    let mut pending = compute_direct_library_imports(assembly_name, graph);
    let mut index = 0;
    while index < pending.len() {
        let current = pending[index].clone();
        index += 1;
        if !discovered.insert(current.clone()) {
            continue;
        }
        libraries.push(current.clone());
        for nested in compute_direct_library_imports(&current, graph) {
            if !discovered.contains(&nested) {
                pending.push(nested);
            }
        }
    }
    sort_strings_deterministically(&mut libraries);
    let library_set: HashSet<&String> = libraries.iter().collect();
    let mut order: Vec<String> = Vec::new();
    let mut emitted: HashSet<String> = HashSet::new();
    while order.len() < libraries.len() {
        let ready = libraries.iter().find(|candidate| {
            !emitted.contains(*candidate)
                && compute_direct_library_imports(candidate, graph).iter().all(|dep| !library_set.contains(dep) || emitted.contains(dep))
        });
        let Some(candidate) = ready else {
            break;
        };
        emitted.insert(candidate.clone());
        order.push(candidate.clone());
    }
    order
}

/// A library that exports to the host (`host_export`) may not import a linked library.
pub fn validate_hosted_library_import_graph(project: &Project, graph: &AssemblyImportGraph<'_>, modules: &[ASTModule], diags: &mut DiagnosticStream) -> bool {
    let Some(selected) = graph.assemblies.get(&project.assembly.name) else {
        return true;
    };
    if !is_library(selected) {
        return true;
    }
    let selected_paths: HashSet<&str> = project.modules.iter().map(|module| module.path.as_str()).collect();
    let hosted = modules.iter().filter(|module| selected_paths.contains(string_of_path(&module.path).as_str())).any(|module| {
        module.items.iter().any(|item| matches!(item, ASTItem::ProcedureDecl(proc) if has_attribute(&proc.attrs, attrs::HOST_EXPORT)))
    });
    if !hosted {
        return true;
    }
    let libraries = compute_library_closure(&project.assembly.name, graph);
    let Some(first) = libraries.first() else {
        return true;
    };
    if let Some(mut diag) = make_external_diagnostic("E-PRJ-0210") {
        diag.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::Note,
            message: format!("hosted library `{}` imports linked library assembly `{first}`", project.assembly.name),
            ..Default::default()
        });
        emit(diags, diag);
    }
    false
}
