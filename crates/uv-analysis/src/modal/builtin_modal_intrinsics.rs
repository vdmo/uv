//! The built-in modal types and the names of their members. Member signatures arrive
//! with the type checker; name resolution only needs to know which names exist.

use crate::caps::builtin_paths::path_matches_builtin_name;
use crate::resolve::scopes::id_eq;
use crate::typing::types::{
    make_type_modal_state, make_type_path, make_type_perm, Permission, TypeFuncParam, TypeRef,
};

const BUILTIN_MODAL_NAMES: [&str; 7] = [
    "Region",
    "File",
    "DirIter",
    "CancelToken",
    "Spawned",
    "Tracked",
    "Async",
];
const BUILTIN_MODAL_NO_RECORD_LITERAL_NAMES: [&str; 6] = [
    "File",
    "DirIter",
    "CancelToken",
    "Spawned",
    "Tracked",
    "Async",
];
const ASYNC_COMBINATORS: [&str; 5] = ["map", "filter", "take", "fold", "chain"];

fn is_single_segment(path: &[String], name: &str) -> bool {
    matches!(path, [only] if id_eq(only, name))
}

fn is_single_segment_one_of(path: &[String], names: &[&str]) -> bool {
    matches!(path, [only] if names.iter().any(|name| id_eq(only, name)))
}

fn is_any(name: &str, names: &[&str]) -> bool {
    names.iter().any(|candidate| id_eq(name, candidate))
}

pub fn is_async_combinator_name(member_name: &str) -> bool {
    is_any(member_name, &ASYNC_COMBINATORS)
}

fn is_region_static_member(member_name: &str) -> bool {
    id_eq(member_name, "new_scoped")
}

/// Members of `Region` in any of its states (`Active`, `Frozen`, `Freed`).
fn is_region_state_member(member_name: &str) -> bool {
    is_any(
        member_name,
        &[
            "alloc",
            "reset_unchecked",
            "freeze",
            "thaw",
            "free_unchecked",
        ],
    )
}

fn is_cancel_token_static_member(member_name: &str) -> bool {
    id_eq(member_name, "new")
}

fn is_cancel_token_active_member(member_name: &str) -> bool {
    is_any(
        member_name,
        &["cancel", "is_cancelled", "child", "wait_cancelled"],
    )
}

pub fn is_builtin_modal_type_path(modal_path: &[String]) -> bool {
    is_single_segment_one_of(modal_path, &BUILTIN_MODAL_NAMES)
}

pub fn is_builtin_modal_record_literal_forbidden(modal_path: &[String]) -> bool {
    is_single_segment_one_of(modal_path, &BUILTIN_MODAL_NO_RECORD_LITERAL_NAMES)
}

pub fn is_builtin_runtime_handle_modal_type_path(modal_path: &[String]) -> bool {
    path_matches_builtin_name(modal_path, "Spawned")
        || path_matches_builtin_name(modal_path, "Tracked")
}

pub fn is_builtin_modal_general_member(modal_path: &[String], member_name: &str) -> bool {
    is_single_segment(modal_path, "Async") && is_async_combinator_name(member_name)
}

pub fn is_builtin_modal_member_name(modal_path: &[String], member_name: &str) -> bool {
    if is_single_segment(modal_path, "Region") {
        return is_region_static_member(member_name) || is_region_state_member(member_name);
    }
    if is_single_segment(modal_path, "CancelToken") {
        return is_cancel_token_static_member(member_name)
            || is_cancel_token_active_member(member_name);
    }
    if is_single_segment(modal_path, "Async") {
        return is_async_combinator_name(member_name) || id_eq(member_name, "resume");
    }
    false
}

pub fn is_builtin_modal_static_member_name(modal_path: &[String], member_name: &str) -> bool {
    if is_single_segment(modal_path, "Region") {
        return is_region_static_member(member_name);
    }
    if is_single_segment(modal_path, "CancelToken") {
        return is_cancel_token_static_member(member_name);
    }
    false
}

/// The signature of a static procedure of a built-in modal: `Region::new_scoped` and
/// `CancelToken::new`.
pub fn lookup_builtin_modal_static_func_sig(
    modal_path: &[String],
    member_name: &str,
) -> Option<(Vec<TypeFuncParam>, TypeRef)> {
    if is_single_segment(modal_path, "Region") {
        if !id_eq(member_name, "new_scoped") {
            return None;
        }
        let params = vec![TypeFuncParam {
            mode: None,
            r#type: make_type_path(vec!["RegionOptions".to_string()]),
        }];
        let ret = make_type_perm(
            Permission::Unique,
            make_type_modal_state(vec!["Region".to_string()], "Active", Vec::new()),
        );
        return Some((params, ret));
    }
    if is_single_segment(modal_path, "CancelToken") {
        if !id_eq(member_name, "new") {
            return None;
        }
        return Some((
            Vec::new(),
            make_type_modal_state(vec!["CancelToken".to_string()], "Active", Vec::new()),
        ));
    }
    None
}

/// The representation of a built-in modal whose layout is fixed by the runtime.
#[derive(Debug, Clone, Copy)]
pub struct BuiltinModalLayoutInfo {
    pub size: u64,
    pub align: u64,
    pub disc_prim: &'static str,
    pub payload_size: u64,
    pub payload_align: u64,
}

pub fn lookup_builtin_modal_layout(modal_path: &[String]) -> Option<BuiltinModalLayoutInfo> {
    is_single_segment(modal_path, "Region").then_some(BuiltinModalLayoutInfo {
        size: 16,
        align: 8,
        disc_prim: "u8",
        payload_size: 8,
        payload_align: 8,
    })
}
