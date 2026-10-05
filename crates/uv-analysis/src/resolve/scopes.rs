//! Identifier keys, reserved names and the universe scope.

use uv_core::unicode::nfc;
use uv_project::language_profile::active_language_profile;
use uv_source::lexer::keyword_policy::is_keyword;

use crate::context::{Entity, EntityKind, EntitySource, IdKey, PathKey, Scope};

const UNIVERSE_PROTECTED_NAMES: [&str; 60] = [
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f16", "f32", "f64", "bool", "char",
    "usize", "isize", "Self", "Drop", "Bitcopy", "Clone", "Eq", "Hash", "Hasher", "Iterator", "Discrete",
    "FfiSafe", "GpuSafe", "string", "bytes", "Modal", "Region", "RegionOptions", "CancelToken", "Context",
    "TestAuthority", "System", "IO", "HeapAllocator", "ExecutionDomain", "Reactor", "Network", "Time",
    "MonotonicTime", "WallTime", "Duration", "MonotonicInstant", "UtcInstant", "TimeError", "Outcome", "CpuSet",
    "Priority", "Async", "Future", "Sequence", "Stream", "Pipe", "Exchange", "Tracked", "Spawned",
];

pub const PRIM_TYPE_NAMES: [&str; 17] = [
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f16", "f32", "f64", "bool", "char",
    "usize", "isize",
];

pub const SPECIAL_TYPE_NAMES: [&str; 34] = [
    "Self", "Drop", "Bitcopy", "Clone", "Eq", "Hash", "Hasher", "Iterator", "Discrete", "FfiSafe", "string",
    "bytes", "Modal", "Region", "RegionOptions", "CancelToken", "Context", "TestAuthority", "System", "IO",
    "HeapAllocator", "ExecutionDomain", "CpuSet", "Priority", "Reactor", "Network", "Time", "MonotonicTime",
    "WallTime", "Duration", "MonotonicInstant", "UtcInstant", "TimeError", "Outcome",
];

pub const ASYNC_TYPE_NAMES: [&str; 8] =
    ["Async", "Future", "Sequence", "Stream", "Pipe", "Exchange", "Tracked", "Spawned"];

pub fn id_key_of(s: &str) -> IdKey {
    nfc(s)
}

pub fn id_eq(s1: &str, s2: &str) -> bool {
    s1 == s2 || id_key_of(s1) == id_key_of(s2)
}

pub fn path_key_of<S: AsRef<str>>(path: &[S]) -> PathKey {
    path.iter().map(|comp| id_key_of(comp.as_ref())).collect()
}

pub fn path_eq<A: AsRef<str>, B: AsRef<str>>(p: &[A], q: &[B]) -> bool {
    p.len() == q.len() && p.iter().zip(q).all(|(a, b)| id_eq(a.as_ref(), b.as_ref()))
}

pub fn scope_key(scope: &Scope) -> bool {
    scope.keys().all(|key| nfc(key) == *key)
}

/// Names starting with `gen_` are reserved for generated code.
pub fn reserved_gen(x: &str) -> bool {
    id_key_of(x).starts_with("gen_")
}

pub fn reserved_module_path(path: &[String]) -> bool {
    path.first().is_some_and(|head| id_eq(head, active_language_profile().runtime_root))
        || path.iter().any(|comp| reserved_gen(comp))
}

pub fn prim_type_keys() -> Vec<IdKey> {
    PRIM_TYPE_NAMES.iter().map(|name| id_key_of(name)).collect()
}

pub fn special_type_keys() -> Vec<IdKey> {
    SPECIAL_TYPE_NAMES.iter().map(|name| id_key_of(name)).collect()
}

pub fn async_type_keys() -> Vec<IdKey> {
    ASYNC_TYPE_NAMES.iter().map(|name| id_key_of(name)).collect()
}

pub fn keyword_key(idkey: &str) -> bool {
    is_keyword(idkey)
}

/// The outermost scope: the built-in type names and the language's own root module.
pub fn universe_bindings() -> Scope {
    let mut scope = Scope::new();
    let language_root = active_language_profile().runtime_root.to_string();
    for name in UNIVERSE_PROTECTED_NAMES {
        scope
            .entry(id_key_of(name))
            .or_insert_with(|| Entity::new(EntityKind::Type, None, None, EntitySource::Decl));
    }
    scope.entry(language_root.clone()).or_insert_with(|| {
        Entity::new(EntityKind::ModuleAlias, Some(vec![language_root]), None, EntitySource::Decl)
    });
    scope
}
