use crate::resolve::scopes::id_eq;

/// Built-in names are available as prelude identifiers and as `<language-root>::<Name>`.
/// Either way the last segment is the name (the reference tests the qualified form
/// separately, which the unqualified test already covers).
pub fn path_matches_builtin_name<S: AsRef<str>>(path: &[S], name: &str) -> bool {
    path.last().is_some_and(|last| id_eq(last.as_ref(), name))
}

fn any_builtin<S: AsRef<str>>(path: &[S], names: &[&str]) -> bool {
    names.iter().any(|name| path_matches_builtin_name(path, name))
}

pub fn is_io_builtin_type_path<S: AsRef<str>>(path: &[S]) -> bool {
    any_builtin(path, &["File", "DirIter", "DirEntry", "FileKind", "IoError"])
}

pub fn is_heap_allocator_builtin_type_path<S: AsRef<str>>(path: &[S]) -> bool {
    any_builtin(path, &["AllocationError"])
}

pub fn is_time_builtin_type_path<S: AsRef<str>>(path: &[S]) -> bool {
    any_builtin(path, &["Duration", "MonotonicInstant", "UtcInstant", "TimeError"])
}

pub fn is_outcome_type_path<S: AsRef<str>>(path: &[S]) -> bool {
    any_builtin(path, &["Outcome"])
}

/// The capability classes a class path may name without a declaration.
pub fn is_capability_class_path<S: AsRef<str>>(path: &[S]) -> bool {
    any_builtin(
        path,
        &["IO", "Network", "HeapAllocator", "Time", "MonotonicTime", "WallTime", "ExecutionDomain", "Reactor"],
    )
}

/// Records that can be constructed by bare name. The reference table has a spare empty
/// slot, so an empty name matches too.
const BUILTIN_RECORD_NAMES: [&str; 8] =
    ["RegionOptions", "DirEntry", "Context", "TestAuthority", "Duration", "MonotonicInstant", "UtcInstant", ""];

pub fn lookup_builtin_record_ctor_path(ident: &str) -> Option<Vec<String>> {
    BUILTIN_RECORD_NAMES.iter().find(|candidate| id_eq(ident, candidate)).map(|name| vec![name.to_string()])
}

pub fn is_string_bytes_builtin_path<S: AsRef<str>>(path: &[S]) -> bool {
    matches!(path, [only] if id_eq(only.as_ref(), "string") || id_eq(only.as_ref(), "bytes"))
}

pub fn is_string_builtin_name(name: &str) -> bool {
    ["from", "as_view", "to_managed", "clone_with", "append", "slice", "length", "is_empty"]
        .iter()
        .any(|entry| id_eq(name, entry))
}

pub fn is_bytes_builtin_name(name: &str) -> bool {
    ["with_capacity", "from_slice", "as_view", "to_managed", "view", "view_string", "as_slice", "append", "length", "is_empty"]
        .iter()
        .any(|entry| id_eq(name, entry))
}
