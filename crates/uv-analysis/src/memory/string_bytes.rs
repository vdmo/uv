//! The built-in procedures of the `string` and `bytes` modules.

use crate::resolve::scopes::id_eq;
use crate::typing::outcome::make_outcome_type;
use crate::typing::types::*;

fn is_string_path(path: &[String]) -> bool {
    matches!(path, [only] if id_eq(only, "string"))
}

fn is_bytes_path(path: &[String]) -> bool {
    matches!(path, [only] if id_eq(only, "bytes"))
}

pub fn is_string_bytes_builtin_path(path: &[String]) -> bool {
    is_string_path(path) || is_bytes_path(path)
}

fn alloc_error_type() -> TypeRef {
    make_type_path(vec!["AllocationError".to_string()])
}

fn heap_allocator_type() -> TypeRef {
    make_type_dynamic(vec!["HeapAllocator".to_string()])
}

fn string_view() -> TypeRef {
    make_type_string(Some(StringState::View))
}

fn string_managed() -> TypeRef {
    make_type_string(Some(StringState::Managed))
}

fn bytes_view() -> TypeRef {
    make_type_bytes(Some(BytesState::View))
}

fn bytes_managed() -> TypeRef {
    make_type_bytes(Some(BytesState::Managed))
}

fn const_of(base: TypeRef) -> TypeRef {
    make_type_perm(Permission::Const, base)
}

fn unique_of(base: TypeRef) -> TypeRef {
    make_type_perm(Permission::Unique, base)
}

fn slice_u8() -> TypeRef {
    make_type_slice(make_type_prim("u8"))
}

fn func(params: Vec<TypeRef>, ret: TypeRef) -> TypeRef {
    make_type_func(
        params
            .into_iter()
            .map(|ty| TypeFuncParam {
                mode: None,
                r#type: ty,
            })
            .collect(),
        ret,
    )
}

fn alloc_outcome(value: TypeRef) -> TypeRef {
    make_outcome_type(value, alloc_error_type())
}

fn string_builtin_type(name: &str) -> Option<TypeRef> {
    let is = |candidate: &str| id_eq(name, candidate);
    Some(if is("from") {
        func(
            vec![string_view(), heap_allocator_type()],
            alloc_outcome(unique_of(string_managed())),
        )
    } else if is("as_view") {
        func(vec![const_of(string_managed())], string_view())
    } else if is("slice") {
        func(
            vec![
                const_of(string_view()),
                make_type_prim("usize"),
                make_type_prim("usize"),
            ],
            string_view(),
        )
    } else if is("to_managed") {
        func(
            vec![const_of(string_view()), heap_allocator_type()],
            alloc_outcome(unique_of(string_managed())),
        )
    } else if is("clone_with") {
        func(
            vec![const_of(string_managed()), heap_allocator_type()],
            alloc_outcome(unique_of(string_managed())),
        )
    } else if is("append") {
        func(
            vec![
                unique_of(string_managed()),
                string_view(),
                heap_allocator_type(),
            ],
            alloc_outcome(make_type_prim("()")),
        )
    } else if is("length") {
        func(vec![const_of(string_view())], make_type_prim("usize"))
    } else if is("is_empty") {
        func(vec![const_of(string_view())], make_type_prim("bool"))
    } else {
        return None;
    })
}

fn bytes_builtin_type(name: &str) -> Option<TypeRef> {
    let is = |candidate: &str| id_eq(name, candidate);
    Some(if is("with_capacity") {
        func(
            vec![make_type_prim("usize"), heap_allocator_type()],
            alloc_outcome(unique_of(bytes_managed())),
        )
    } else if is("from_slice") {
        func(
            vec![const_of(slice_u8()), heap_allocator_type()],
            alloc_outcome(unique_of(bytes_managed())),
        )
    } else if is("as_view") {
        func(vec![const_of(bytes_managed())], bytes_view())
    } else if is("to_managed") {
        func(
            vec![const_of(bytes_view()), heap_allocator_type()],
            alloc_outcome(unique_of(bytes_managed())),
        )
    } else if is("view") {
        func(vec![const_of(slice_u8())], bytes_view())
    } else if is("view_string") {
        func(vec![string_view()], bytes_view())
    } else if is("as_slice") {
        func(vec![const_of(bytes_view())], const_of(slice_u8()))
    } else if is("append") {
        func(
            vec![
                unique_of(bytes_managed()),
                bytes_view(),
                heap_allocator_type(),
            ],
            alloc_outcome(make_type_prim("()")),
        )
    } else if is("length") {
        func(vec![const_of(bytes_view())], make_type_prim("usize"))
    } else if is("is_empty") {
        func(vec![const_of(bytes_view())], make_type_prim("bool"))
    } else {
        return None;
    })
}

/// The type of `string::name` or `bytes::name` as a procedure value.
pub fn lookup_string_bytes_builtin_type(path: &[String], name: &str) -> Option<TypeRef> {
    if is_string_path(path) {
        return string_builtin_type(name);
    }
    if is_bytes_path(path) {
        return bytes_builtin_type(name);
    }
    None
}
