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

/// A state value larger than this is worth a warning when widened by copy.
pub const WIDEN_LARGE_PAYLOAD_THRESHOLD_BYTES: u64 = 256;

/// Whether a state of the modal has the same representation as the modal itself, so that
/// widening it is free.
pub fn niche_compatible(ctx: &ScopeContext<'_>, modal_path: &[String], state: &str) -> bool {
    use crate::layout::{align_of, size_of};
    use crate::modal::lookup::{has_state, lookup_modal_decl};
    use crate::typing::types::{make_type_modal_state, make_type_path};
    let Some(decl) = lookup_modal_decl(ctx, modal_path) else {
        return false;
    };
    if !has_state(decl, state) || !payload_state(ctx, decl).is_some_and(|payload| id_eq(payload, state)) {
        return false;
    }
    let state_type = make_type_modal_state(modal_path.to_vec(), state, Vec::new());
    let modal_type = make_type_path(modal_path.to_vec());
    let same = |measure: fn(&ScopeContext<'_>, &crate::typing::types::TypeRef) -> Option<u64>| {
        matches!((measure(ctx, &state_type), measure(ctx, &modal_type)), (Some(a), Some(b)) if a == b)
    };
    same(size_of) && same(align_of)
}

/// Whether widening this state copies a large payload.
pub fn widen_warn_cond(ctx: &ScopeContext<'_>, modal_path: &[String], state: &str) -> bool {
    use crate::layout::size_of;
    use crate::modal::lookup::{has_state, lookup_modal_decl};
    use crate::typing::types::make_type_modal_state;
    let Some(decl) = lookup_modal_decl(ctx, modal_path) else {
        return false;
    };
    if !has_state(decl, state) {
        return false;
    }
    let state_type = make_type_modal_state(modal_path.to_vec(), state, Vec::new());
    size_of(ctx, &state_type).is_some_and(|size| size > WIDEN_LARGE_PAYLOAD_THRESHOLD_BYTES)
        && !niche_compatible(ctx, modal_path, state)
}
