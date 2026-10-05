//! Receivers of methods.

use uv_source::ast;

use crate::typing::signature::subst_self_type;
use crate::typing::type_lower::{lower_param_mode, LowerError};
use crate::typing::types::*;

pub fn lower_receiver_perm(perm: ast::ReceiverPerm) -> Permission {
    match perm {
        ast::ReceiverPerm::Const => Permission::Const,
        ast::ReceiverPerm::Unique => Permission::Unique,
        ast::ReceiverPerm::Shared => Permission::Shared,
    }
}

/// An explicit receiver must be `Self`, possibly under permissions and refinements.
fn is_explicit_self_receiver_type(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) => is_self_var_path(path),
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => is_explicit_self_receiver_type(base),
        _ => false,
    }
}

/// The type of `self` in a method of `base`. `lower_type` lowers an explicit receiver's
/// written type.
pub fn recv_type_for_receiver(
    base: &TypeRef,
    receiver: &ast::Receiver,
    lower_type: impl FnOnce(&ast::TypePtr) -> Result<TypeRef, LowerError>,
) -> Result<TypeRef, LowerError> {
    match receiver {
        ast::Receiver::ReceiverShorthand(recv) => Ok(make_type_perm(lower_receiver_perm(recv.perm), base.clone())),
        ast::Receiver::ReceiverExplicit(recv) => {
            let lowered = lower_type(&recv.r#type)?;
            if !is_explicit_self_receiver_type(&lowered) {
                return Err(Some("Record-Method-RecvSelf-Err"));
            }
            Ok(subst_self_type(base, &lowered, None))
        }
    }
}

pub fn recv_mode_of(receiver: &ast::Receiver) -> Option<ParamMode> {
    match receiver {
        ast::Receiver::ReceiverShorthand(recv) => lower_param_mode(recv.mode_opt),
        ast::Receiver::ReceiverExplicit(recv) => lower_param_mode(recv.mode_opt),
    }
}
