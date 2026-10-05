//! Regions, and where a value's storage comes from.

use crate::typing::type_env::TypeEnv;
use crate::typing::types::{TypeNode, TypeRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceKind {
    Global,
    Stack,
    Heap,
    Region,
    Bottom,
    Param,
}

/// One permission or refinement is looked through, not more.
fn strip_perm_once(ty: &TypeRef) -> TypeRef {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => base.clone(),
        _ => ty.clone(),
    }
}

/// `Region@Active`, possibly under one permission.
pub fn region_active_type(ty: &TypeRef) -> bool {
    match strip_perm_once(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::ModalState(modal)) => {
            modal.path.len() == 1 && modal.path[0] == "Region" && modal.state == "Active"
        }
        _ => false,
    }
}

/// The active region bound innermost; within a scope, the least name.
pub fn innermost_active_region(env: &TypeEnv) -> Option<String> {
    for scope in env.scopes.iter().rev() {
        let best = scope
            .iter()
            .filter(|(_, binding)| region_active_type(&binding.r#type))
            .map(|(key, _)| key)
            .min();
        if let Some(best) = best {
            return Some(best.clone());
        }
    }
    None
}
