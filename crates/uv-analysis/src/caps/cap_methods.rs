//! The methods the capability classes and the compile-time capabilities build in.
//! Parameters are written as syntax, as a declared method's are, so that calls check
//! them the same way.

use uv_source::ast::{self, BytesState as AstBytesState, RawPtrQual as AstRawPtrQual, StringState as AstStringState};

use super::builtin_decls::{bytes, param, path, prim, union};
use super::builtin_paths::{is_context_type_path, path_matches_builtin_name};
use crate::resolve::scopes::id_eq;
use crate::typing::outcome::make_outcome_type;
use crate::typing::types::*;

#[derive(Debug, Clone)]
pub struct CapMethodSig {
    pub recv_perm: Permission,
    pub params: Vec<ast::Param>,
    pub ret: TypeRef,
    /// A raw allocation or deallocation, which needs `unsafe`.
    pub raw_heap: bool,
}

fn sig(params: Vec<ast::Param>, ret: TypeRef) -> Option<CapMethodSig> {
    Some(CapMethodSig { recv_perm: Permission::Const, params, ret, raw_heap: false })
}

fn string_view_ast() -> ast::TypePtr {
    Some(std::sync::Arc::new(ast::Type {
        span: Default::default(),
        node: ast::TypeNode::TypeString(ast::TypeString { state: Some(AstStringState::View) }),
    }))
}

fn raw_ptr_mut_u8_ast() -> ast::TypePtr {
    Some(std::sync::Arc::new(ast::Type {
        span: Default::default(),
        node: ast::TypeNode::TypeRawPtr(ast::TypeRawPtr { qual: AstRawPtrQual::Mut, element: prim("u8") }),
    }))
}

fn named(name: &str) -> TypeRef {
    make_type_path(vec![name.to_string()])
}

fn dynamic(name: &str) -> TypeRef {
    make_type_dynamic(vec![name.to_string()])
}

fn unit() -> TypeRef {
    make_type_prim("()")
}

fn string_view() -> TypeRef {
    make_type_string(Some(StringState::View))
}

fn string_managed() -> TypeRef {
    make_type_string(Some(StringState::Managed))
}

fn unique(base: TypeRef) -> TypeRef {
    make_type_perm(Permission::Unique, base)
}

fn io_outcome(value: TypeRef) -> TypeRef {
    make_outcome_type(value, named("IoError"))
}

fn time_outcome(value: TypeRef) -> TypeRef {
    make_outcome_type(value, named("TimeError"))
}

fn modal_state(modal: &str, state: &str) -> TypeRef {
    unique(make_type_modal_state(vec![modal.to_string()], state, Vec::new()))
}

fn is(name: &str, candidates: &[&str]) -> bool {
    candidates.iter().any(|candidate| id_eq(name, candidate))
}

pub fn lookup_io_method_sig(name: &str) -> Option<CapMethodSig> {
    let path_param = || vec![param("path", string_view_ast())];
    let data_param = || vec![param("data", string_view_ast())];
    if id_eq(name, "open_read") {
        return sig(path_param(), io_outcome(modal_state("File", "Read")));
    }
    if is(name, &["open_write", "create_write"]) {
        return sig(path_param(), io_outcome(modal_state("File", "Write")));
    }
    if id_eq(name, "open_append") {
        return sig(path_param(), io_outcome(modal_state("File", "Append")));
    }
    if id_eq(name, "read_file") {
        return sig(path_param(), io_outcome(unique(string_managed())));
    }
    if id_eq(name, "read_bytes") {
        return sig(path_param(), io_outcome(unique(make_type_bytes(Some(BytesState::Managed)))));
    }
    if id_eq(name, "write_file") {
        return sig(vec![param("path", string_view_ast()), param("data", bytes(AstBytesState::View))], io_outcome(unit()));
    }
    if is(name, &["write_stdout", "write_stderr"]) {
        return sig(data_param(), io_outcome(unit()));
    }
    if id_eq(name, "exists") {
        return sig(path_param(), make_type_prim("bool"));
    }
    if is(name, &["remove", "create_dir", "ensure_dir"]) {
        return sig(path_param(), io_outcome(unit()));
    }
    if id_eq(name, "open_dir") {
        return sig(path_param(), io_outcome(modal_state("DirIter", "Open")));
    }
    if id_eq(name, "kind") {
        return sig(path_param(), io_outcome(named("FileKind")));
    }
    if id_eq(name, "restrict") {
        return sig(path_param(), dynamic("IO"));
    }
    None
}

pub fn lookup_heap_allocator_method_sig(name: &str) -> Option<CapMethodSig> {
    if id_eq(name, "with_quota") {
        return sig(vec![param("size", prim("usize"))], dynamic("HeapAllocator"));
    }
    let raw = |sig: Option<CapMethodSig>| sig.map(|sig| CapMethodSig { raw_heap: true, ..sig });
    if id_eq(name, "alloc_raw") {
        return raw(sig(vec![param("count", prim("usize"))], make_type_raw_ptr(RawPtrQual::Mut, make_type_prim("u8"))));
    }
    if id_eq(name, "dealloc_raw") {
        return raw(sig(vec![param("ptr", raw_ptr_mut_u8_ast()), param("count", prim("usize"))], unit()));
    }
    None
}

pub fn lookup_network_method_sig(name: &str) -> Option<CapMethodSig> {
    if id_eq(name, "restrict_to_host") {
        return sig(vec![param("host", string_view_ast())], dynamic("Network"));
    }
    None
}

pub fn lookup_time_method_sig(name: &str) -> Option<CapMethodSig> {
    if id_eq(name, "monotonic") {
        return sig(Vec::new(), dynamic("MonotonicTime"));
    }
    if id_eq(name, "wall") {
        return sig(Vec::new(), dynamic("WallTime"));
    }
    None
}

pub fn lookup_monotonic_time_method_sig(name: &str) -> Option<CapMethodSig> {
    if id_eq(name, "now") {
        return sig(Vec::new(), named("MonotonicInstant"));
    }
    if id_eq(name, "resolution") {
        return sig(Vec::new(), named("Duration"));
    }
    if id_eq(name, "elapsed") {
        let instants = vec![param("start", path(&["MonotonicInstant"])), param("end", path(&["MonotonicInstant"]))];
        return sig(instants, time_outcome(named("Duration")));
    }
    if id_eq(name, "coarsen") {
        return sig(vec![param("resolution", path(&["Duration"]))], time_outcome(dynamic("MonotonicTime")));
    }
    None
}

pub fn lookup_wall_time_method_sig(name: &str) -> Option<CapMethodSig> {
    if id_eq(name, "now_utc") {
        return sig(Vec::new(), time_outcome(named("UtcInstant")));
    }
    if id_eq(name, "resolution") {
        return sig(Vec::new(), time_outcome(named("Duration")));
    }
    if id_eq(name, "coarsen") {
        return sig(vec![param("resolution", path(&["Duration"]))], time_outcome(dynamic("WallTime")));
    }
    None
}

/// These two names are compared exactly, as the reference compares them.
pub fn lookup_execution_domain_method_sig(name: &str) -> Option<CapMethodSig> {
    match name {
        "name" => sig(Vec::new(), string_view()),
        "max_concurrency" => sig(Vec::new(), make_type_prim("usize")),
        _ => None,
    }
}

pub fn lookup_system_method_sig(name: &str) -> Option<CapMethodSig> {
    if id_eq(name, "exit") {
        return sig(vec![param("code", prim("i32"))], make_type_prim("!"));
    }
    if id_eq(name, "get_env") {
        return sig(vec![param("key", string_view_ast())], string_view());
    }
    if is(name, &["executable_path", "current_directory"]) {
        return sig(Vec::new(), string_view());
    }
    if id_eq(name, "argument_count") {
        return sig(Vec::new(), make_type_prim("usize"));
    }
    if id_eq(name, "argument") {
        return sig(vec![param("index", prim("usize"))], string_view());
    }
    if id_eq(name, "run") {
        return sig(vec![param("command", string_view_ast())], make_type_prim("i32"));
    }
    None
}

/// The execution domains a context hands out. `cpu` takes an optional mask and
/// priority; the others take nothing.
pub fn lookup_context_method_sig(name: &str, arg_count: Option<usize>) -> Option<CapMethodSig> {
    let domain = || dynamic("ExecutionDomain");
    if id_eq(name, "cpu") {
        return match arg_count {
            None | Some(0) => sig(Vec::new(), domain()),
            Some(1) => sig(vec![param("mask", path(&["CpuSet"]))], domain()),
            Some(2) => sig(vec![param("mask", path(&["CpuSet"])), param("prio", path(&["Priority"]))], domain()),
            Some(_) => None,
        };
    }
    if is(name, &["gpu", "inline"]) {
        return if arg_count.is_some_and(|count| count != 0) { None } else { sig(Vec::new(), domain()) };
    }
    None
}

fn lookup_project_files_method_sig(name: &str) -> Option<CapMethodSig> {
    let path_param = || vec![param("path", string_view_ast())];
    if id_eq(name, "read") {
        return sig(path_param(), io_outcome(unique(string_managed())));
    }
    if id_eq(name, "read_bytes") {
        return sig(path_param(), io_outcome(unique(make_type_bytes(Some(BytesState::Managed)))));
    }
    if id_eq(name, "exists") {
        return sig(path_param(), io_outcome(make_type_prim("bool")));
    }
    if id_eq(name, "list_dir") {
        return sig(path_param(), io_outcome(make_type_slice(string_managed())));
    }
    if id_eq(name, "project_root") {
        return sig(Vec::new(), string_managed());
    }
    None
}

fn lookup_comptime_diagnostics_method_sig(name: &str) -> Option<CapMethodSig> {
    let message = || vec![param("message", string_view_ast())];
    if id_eq(name, "error") {
        return sig(message(), make_type_prim("!"));
    }
    if is(name, &["warning", "note"]) {
        return sig(message(), unit());
    }
    if id_eq(name, "current_span") {
        return sig(Vec::new(), named("SourceSpan"));
    }
    if id_eq(name, "current_module") {
        return sig(Vec::new(), string_managed());
    }
    None
}

fn lookup_introspect_method_sig(name: &str) -> Option<CapMethodSig> {
    let type_param = || vec![param("ty", path(&["Type"]))];
    if id_eq(name, "category") {
        return sig(type_param(), named("TypeCategory"));
    }
    for (method, info) in [("fields", "FieldInfo"), ("variants", "VariantInfo"), ("states", "StateInfo")] {
        if id_eq(name, method) {
            return sig(type_param(), make_type_slice(named(info)));
        }
    }
    if id_eq(name, "implements_form") {
        return sig(vec![param("ty", path(&["Type"])), param("form", path(&["Type"]))], make_type_prim("bool"));
    }
    if is(name, &["type_name", "module_path"]) {
        return sig(type_param(), string_managed());
    }
    None
}

/// The built-in method of a capability class reached through a dynamic type.
pub fn lookup_capability_class_method_sig(class_path: &[String], name: &str) -> Option<CapMethodSig> {
    let class_is = |class: &str| path_matches_builtin_name(class_path, class);
    if class_is("IO") {
        return lookup_io_method_sig(name);
    }
    if class_is("HeapAllocator") {
        return lookup_heap_allocator_method_sig(name);
    }
    if class_is("Network") {
        return lookup_network_method_sig(name);
    }
    if class_is("Time") {
        return lookup_time_method_sig(name);
    }
    if class_is("MonotonicTime") {
        return lookup_monotonic_time_method_sig(name);
    }
    if class_is("WallTime") {
        return lookup_wall_time_method_sig(name);
    }
    if class_is("ExecutionDomain") {
        return lookup_execution_domain_method_sig(name);
    }
    if class_is("System") {
        return lookup_system_method_sig(name);
    }
    None
}

/// The built-in method of a compile-time capability or of `Context`, which are named
/// by path types.
pub fn lookup_capability_type_method_sig(type_path: &[String], name: &str, arg_count: usize) -> Option<CapMethodSig> {
    let single = |expected: &str| matches!(type_path, [only] if id_eq(only, expected));
    if single("ProjectFiles") {
        if let found @ Some(_) = lookup_project_files_method_sig(name) {
            return found;
        }
    }
    if single("ComptimeDiagnostics") {
        if let found @ Some(_) = lookup_comptime_diagnostics_method_sig(name) {
            return found;
        }
    }
    if single("Introspect") {
        if let found @ Some(_) = lookup_introspect_method_sig(name) {
            return found;
        }
    }
    if single("TypeEmitter") && id_eq(name, "emit") {
        return sig(vec![param("node", union(vec![path(&["Ast"]), path(&["Ast", "Item"])]))], unit());
    }
    if is_context_type_path(type_path) {
        return lookup_context_method_sig(name, Some(arg_count));
    }
    None
}
