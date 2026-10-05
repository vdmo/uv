//! Modals: a discriminant for the state and room for the largest state, or a niche.

use uv_source::ast;

use super::aggregates::{decl_substitution, lower_with_subst};
use super::*;
use crate::modal::modal_widen::payload_state;
use crate::resolve::scopes::id_eq;

/// Bytes before the frame pointer in a suspended asynchronous computation.
pub(crate) const ASYNC_FRAME_PTR_PAYLOAD_OFFSET: u64 = 8;

/// `Async` is laid out by its arguments, not by its declared states: the suspended state
/// holds the yielded value and the frame pointer, the others the result or the error.
fn async_modal_layout(ctx: &ScopeContext<'_>, decl: &ast::ModalDecl, generic_args: &[TypeRef]) -> Option<ModalLayout> {
    let arg_or = |index: usize, default: &str| generic_args.get(index).cloned().unwrap_or_else(|| make_type_prim(default));
    let frame_ptr = make_type_ptr(make_type_prim("u8"), Some(PtrState::Valid));
    let suspended = record_layout_of(ctx, &[arg_or(0, "()"), frame_ptr], &RecordLayoutOptions::default())?.layout;
    let mut payload_size = suspended.size;
    let mut payload_align = suspended.align;
    for payload in [arg_or(2, "()"), arg_or(3, "!")] {
        if is_unit_type(&payload) || is_never_type(&payload) {
            continue;
        }
        let layout = layout_of(ctx, &payload)?;
        if layout.size != 0 {
            payload_size = payload_size.max(layout.size);
            payload_align = payload_align.max(layout.align);
        }
    }
    payload_size = payload_size.max(ASYNC_FRAME_PTR_PAYLOAD_OFFSET + ptr_size(ctx));
    payload_align = payload_align.max(ptr_align(ctx));
    let max_disc = (decl.states.len() - 1) as u64;
    let disc = disc_type_layout(max_disc);
    let align = disc.align.max(payload_align);
    Some(ModalLayout {
        layout: Layout { size: align_up(disc.size + payload_size, align), align },
        niche: false,
        niche_payload_layout: None,
        disc_type: Some(disc_type_name(max_disc).to_string()),
        payload_size,
        payload_align,
    })
}

pub fn modal_layout_of(ctx: &ScopeContext<'_>, decl: &ast::ModalDecl, generic_args: &[TypeRef]) -> Option<ModalLayout> {
    if decl.states.is_empty() {
        return None;
    }
    if decl.name == "Async" {
        return async_modal_layout(ctx, decl, generic_args);
    }
    let subst = decl_substitution(&decl.generic_params, generic_args)?;
    let mut state_layouts = Vec::with_capacity(decl.states.len());
    for state in &decl.states {
        let mut fields = Vec::new();
        for member in &state.members {
            if let ast::StateMember::StateFieldDecl(field) = member {
                fields.push(lower_with_subst(ctx, &field.r#type, &subst)?);
            }
        }
        state_layouts.push((&state.name, record_layout_of(ctx, &fields, &RecordLayoutOptions::default())?.layout));
    }
    if let Some(payload) = payload_state(ctx, decl) {
        if let Some((_, layout)) = state_layouts.iter().find(|(name, _)| id_eq(name, payload)) {
            return Some(ModalLayout {
                layout: *layout,
                niche: true,
                niche_payload_layout: Some(*layout),
                disc_type: None,
                payload_size: layout.size,
                payload_align: layout.align,
            });
        }
    }
    let payload_size = state_layouts.iter().map(|(_, layout)| layout.size).max().unwrap_or(0);
    let payload_align = state_layouts.iter().map(|(_, layout)| layout.align).max().unwrap_or(1);
    let max_disc = (decl.states.len() - 1) as u64;
    let disc = disc_type_layout(max_disc);
    let align = disc.align.max(payload_align);
    Some(ModalLayout {
        layout: Layout { size: align_up(disc.size + payload_size, align), align },
        niche: false,
        niche_payload_layout: None,
        disc_type: Some(disc_type_name(max_disc).to_string()),
        payload_size,
        payload_align,
    })
}
