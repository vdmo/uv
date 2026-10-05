//! The GPU intrinsics: their names and types.

use crate::resolve::scopes::id_eq;
use crate::typing::types::{make_type_func, make_type_prim, make_type_tuple, TypeRef};

const GPU_TRIPLET_INTRINSICS: [&str; 6] = [
    "gpu_global_id",
    "gpu_local_id",
    "gpu_workgroup_id",
    "gpu_workgroup_size",
    "gpu_global_size",
    "gpu_num_workgroups",
];
const GPU_SCALAR_INTRINSICS: [&str; 1] = ["gpu_linear_id"];
const GPU_BARRIER_INTRINSICS: [&str; 3] =
    ["gpu_barrier", "gpu_memory_barrier", "gpu_workgroup_barrier"];

fn is_name_in(name: &str, names: &[&str]) -> bool {
    names.iter().any(|candidate| id_eq(name, candidate))
}

pub fn is_gpu_intrinsic_name(name: &str) -> bool {
    is_name_in(name, &GPU_TRIPLET_INTRINSICS)
        || is_name_in(name, &GPU_SCALAR_INTRINSICS)
        || is_name_in(name, &GPU_BARRIER_INTRINSICS)
}

pub fn is_gpu_execution_barrier_name(name: &str) -> bool {
    is_name_in(name, &["gpu_barrier", "gpu_workgroup_barrier"])
}

/// The type of a GPU intrinsic as a procedure value.
pub fn lookup_gpu_intrinsic_type(name: &str) -> Option<TypeRef> {
    if is_name_in(name, &GPU_TRIPLET_INTRINSICS) {
        let usize3 = make_type_tuple(vec![
            make_type_prim("usize"),
            make_type_prim("usize"),
            make_type_prim("usize"),
        ]);
        return Some(make_type_func(Vec::new(), usize3));
    }
    if is_name_in(name, &GPU_SCALAR_INTRINSICS) {
        return Some(make_type_func(Vec::new(), make_type_prim("usize")));
    }
    if is_name_in(name, &GPU_BARRIER_INTRINSICS) {
        return Some(make_type_func(Vec::new(), make_type_prim("()")));
    }
    None
}
