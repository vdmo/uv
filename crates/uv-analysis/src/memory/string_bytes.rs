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

/// The built-in methods of strings and bytes, by the receiver's state.
pub fn lookup_string_bytes_builtin_method_sig(
    recv_base: &TypeRef,
    name: &str,
) -> Option<crate::typing::type_predicates::FoundationalBuiltinMethodSig> {
    use crate::resolve::scopes::id_eq;
    use crate::typing::outcome::make_outcome_type;
    use crate::typing::type_predicates::FoundationalBuiltinMethodSig;
    let sig = |recv_perm: Permission, recv_type: TypeRef, params: Vec<TypeRef>, ret: TypeRef| {
        Some(FoundationalBuiltinMethodSig {
            recv_perm,
            recv_type,
            params: params.into_iter().map(|ty| TypeFuncParam { mode: None, r#type: ty }).collect(),
            ret,
        })
    };
    let usize_type = || make_type_prim("usize");
    let alloc_outcome = |value: TypeRef| make_outcome_type(value, alloc_error_type());
    match &recv_base.as_deref()?.node {
        TypeNode::String(state) => {
            let any_string = make_type_string(None);
            let managed = *state == Some(StringState::Managed);
            let view = *state == Some(StringState::View);
            if id_eq(name, "length") {
                return sig(Permission::Const, any_string, Vec::new(), usize_type());
            }
            if id_eq(name, "is_empty") {
                return sig(Permission::Const, any_string, Vec::new(), make_type_prim("bool"));
            }
            if id_eq(name, "as_view") && managed {
                return sig(Permission::Const, string_managed(), Vec::new(), string_view());
            }
            if id_eq(name, "slice") && view {
                return sig(Permission::Const, string_view(), vec![usize_type(), usize_type()], string_view());
            }
            if id_eq(name, "to_managed") && view {
                return sig(Permission::Const, string_view(), vec![heap_allocator_type()], alloc_outcome(unique_of(string_managed())));
            }
            if id_eq(name, "clone_with") && managed {
                return sig(Permission::Const, string_managed(), vec![heap_allocator_type()], alloc_outcome(unique_of(string_managed())));
            }
            if id_eq(name, "append") && managed {
                let params = vec![string_view(), heap_allocator_type()];
                return sig(Permission::Unique, string_managed(), params, alloc_outcome(make_type_prim("()")));
            }
            None
        }
        TypeNode::Bytes(state) => {
            let any_bytes = make_type_bytes(None);
            let managed = *state == Some(BytesState::Managed);
            let view = *state == Some(BytesState::View);
            if id_eq(name, "length") {
                return sig(Permission::Const, any_bytes, Vec::new(), usize_type());
            }
            if id_eq(name, "is_empty") {
                return sig(Permission::Const, any_bytes, Vec::new(), make_type_prim("bool"));
            }
            if id_eq(name, "as_slice") {
                return sig(Permission::Const, any_bytes, Vec::new(), const_of(slice_u8()));
            }
            if id_eq(name, "as_view") && managed {
                return sig(Permission::Const, bytes_managed(), Vec::new(), bytes_view());
            }
            if id_eq(name, "to_managed") && view {
                return sig(Permission::Const, bytes_view(), vec![heap_allocator_type()], alloc_outcome(unique_of(bytes_managed())));
            }
            if id_eq(name, "append") && managed {
                let params = vec![bytes_view(), heap_allocator_type()];
                return sig(Permission::Unique, bytes_managed(), params, alloc_outcome(make_type_prim("()")));
            }
            None
        }
        _ => None,
    }
}
