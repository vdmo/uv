//! Names of the GPU intrinsics. Their types arrive with the type checker.

use crate::resolve::scopes::id_eq;

const GPU_TRIPLET_INTRINSICS: [&str; 6] = [
    "gpu_global_id",
    "gpu_local_id",
    "gpu_workgroup_id",
    "gpu_workgroup_size",
    "gpu_global_size",
    "gpu_num_workgroups",
];
const GPU_SCALAR_INTRINSICS: [&str; 1] = ["gpu_linear_id"];
const GPU_BARRIER_INTRINSICS: [&str; 3] = ["gpu_barrier", "gpu_memory_barrier", "gpu_workgroup_barrier"];

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
