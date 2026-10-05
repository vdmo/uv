//! Whether a modal can be represented without a discriminant.

use uv_source::ast;

use crate::context::ScopeContext;
use crate::resolve::scopes::id_eq;
use crate::typing::type_lower::{lower_type_as, LOWER_FOR_MODAL};
use crate::typing::types::{PtrState, TypeNode};

fn state_fields(state: &ast::StateBlock) -> impl Iterator<Item = &ast::StateFieldDecl> {
    state.members.iter().filter_map(|member| match member {
        ast::StateMember::StateFieldDecl(field) => Some(field),
        _ => None,
    })
}

/// The state whose single field is a valid pointer, when every other state is empty: the
/// pointer's null value can then stand for the other state. A pointer has one spare
/// value, so this works for modals of at most two states.
pub fn payload_state<'d>(ctx: &ScopeContext<'_>, decl: &'d ast::ModalDecl) -> Option<&'d str> {
    let mut candidate = None;
    for state in &decl.states {
        let mut fields = state_fields(state);
        let (Some(payload), None) = (fields.next(), fields.next()) else {
            continue;
        };
        let lowered = lower_type_as(ctx, &payload.r#type, LOWER_FOR_MODAL).ok()?;
        let is_valid_ptr =
            matches!(lowered.as_deref().map(|ty| &ty.node), Some(TypeNode::Ptr { state: Some(PtrState::Valid), .. }));
        if !is_valid_ptr {
            continue;
        }
        if candidate.is_some() {
            return None;
        }
        candidate = Some(state.name.as_str());
    }
    let candidate = candidate?;
    let others_empty =
        decl.states.iter().filter(|state| !id_eq(&state.name, candidate)).all(|state| state_fields(state).next().is_none());
    (others_empty && decl.states.len() <= 2).then_some(candidate)
}

pub fn niche_applies(ctx: &ScopeContext<'_>, decl: &ast::ModalDecl) -> bool {
    payload_state(ctx, decl).is_some()
}
