//! Finding modal declarations, their states and the members of a state.

use std::collections::HashSet;

use uv_source::ast;

use crate::context::{IdKey, ScopeContext, TypeDecl};
use crate::resolve::scopes::{id_eq, id_key_of, path_eq, path_key_of};

pub fn lookup_modal_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::ModalDecl> {
    match ctx.sigma.types.get(&path_key_of(path))? {
        TypeDecl::Modal(decl) => Some(decl),
        _ => None,
    }
}

pub fn lookup_modal_state<'d>(decl: &'d ast::ModalDecl, state: &str) -> Option<&'d ast::StateBlock> {
    let key = id_key_of(state);
    decl.states.iter().find(|block| id_key_of(&block.name) == key)
}

pub fn has_state(decl: &ast::ModalDecl, state: &str) -> bool {
    lookup_modal_state(decl, state).is_some()
}

pub fn state_name_set(decl: &ast::ModalDecl) -> HashSet<IdKey> {
    decl.states.iter().map(|state| id_key_of(&state.name)).collect()
}

pub fn lookup_modal_field_decl<'d>(decl: &'d ast::ModalDecl, state: &str, name: &str) -> Option<&'d ast::StateFieldDecl> {
    lookup_modal_state(decl, state)?.members.iter().find_map(|member| match member {
        ast::StateMember::StateFieldDecl(field) if id_eq(&field.name, name) => Some(field),
        _ => None,
    })
}

pub fn lookup_state_method_decl<'d>(decl: &'d ast::ModalDecl, state: &str, name: &str) -> Option<&'d ast::StateMethodDecl> {
    lookup_modal_state(decl, state)?.members.iter().find_map(|member| match member {
        ast::StateMember::StateMethodDecl(method) if id_eq(&method.name, name) => Some(method),
        _ => None,
    })
}

pub fn lookup_transition_decl<'d>(decl: &'d ast::ModalDecl, state: &str, name: &str) -> Option<&'d ast::TransitionDecl> {
    lookup_modal_state(decl, state)?.members.iter().find_map(|member| match member {
        ast::StateMember::TransitionDecl(transition) if id_eq(&transition.name, name) => Some(transition),
        _ => None,
    })
}

fn in_declaring_module(ctx: &ScopeContext<'_>, modal_path: &[String]) -> bool {
    let module = modal_path.split_last().map_or(&[][..], |(_, module)| module);
    path_eq(module, &ctx.current_module)
}

/// State fields are visible in the module that declares the modal.
pub fn modal_field_visible(ctx: &ScopeContext<'_>, modal_path: &[String]) -> bool {
    in_declaring_module(ctx, modal_path)
}

/// Private methods and transitions are visible in the module that declares the modal.
pub fn state_member_visible(ctx: &ScopeContext<'_>, modal_path: &[String], vis: ast::Visibility) -> bool {
    vis != ast::Visibility::Private || in_declaring_module(ctx, modal_path)
}
