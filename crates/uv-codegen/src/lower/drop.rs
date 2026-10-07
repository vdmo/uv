//! Lowering: which values have something to drop. See `TypeNeedsDrop` in `drop_hooks`.

use std::collections::HashSet;

use uv_analysis::context::TypeDecl;
use uv_analysis::generics::monomorphize::{build_substitution, instantiate_type};
use uv_analysis::resolve::scopes::path_key_of as type_path_key;
use uv_analysis::typing::types::{is_range_type, type_to_string, StringState, BytesState};

use super::*;

/// `LowerTypeForDrop`: a written type lowered with the program and the module, without names.
fn lower_type_for_drop(written: &Option<Arc<ast::Type>>, ctx: &LowerCtx) -> Option<TypeRef> {
    written.as_ref()?;
    let scope = layout_scope(ctx, &ctx.module_path);
    lower_type_for_layout(&scope, written)
}

fn instantiate_if_generic(ty: TypeRef, generic_params: &Option<ast::GenericParams>, generic_args: &[TypeRef]) -> TypeRef {
    let Some(params) = generic_params.as_ref().filter(|params| !params.params.is_empty()) else {
        return ty;
    };
    if generic_args.len() > params.params.len() {
        return None;
    }
    let subst = build_substitution(&params.params, generic_args);
    if subst.is_empty() {
        return ty;
    }
    instantiate_type(&ty, &subst)
}

/// `IsBuiltinDropType`: managed strings and bytes.
fn is_builtin_drop_type(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::String(state)) => *state == Some(StringState::Managed),
        Some(TypeNode::Bytes(state)) => *state == Some(BytesState::Managed),
        _ => false,
    }
}

/// `LookupDropMethodSymbol`: whether the type implements `Drop` with a method to call.
fn has_drop_method(stripped: &TypeRef, ctx: &LowerCtx) -> bool {
    let scope = layout_scope(ctx, &ctx.module_path);
    uv_analysis::composite::classes::type_implements_class(&scope, stripped, &["Drop".to_string()]) && uv_analysis::composite::record_methods::lookup_method_static(&scope, stripped, "drop").is_ok()
}

fn state_fields_need_drop(state: &ast::StateBlock, generic_params: &Option<ast::GenericParams>, generic_args: &[TypeRef], ctx: &LowerCtx, active: &mut HashSet<String>) -> bool {
    for member in &state.members {
        let ast::StateMember::StateFieldDecl(field) = member else {
            continue;
        };
        let Some(lowered) = lower_type_for_drop(&field.r#type, ctx) else {
            return true;
        };
        let field_type = instantiate_if_generic(lowered, generic_params, generic_args);
        if field_type.is_none() || needs_drop(&field_type, ctx, active) {
            return true;
        }
    }
    false
}

fn nominal_needs_drop(path: &[String], generic_args: &[TypeRef], ctx: &LowerCtx, active: &mut HashSet<String>) -> bool {
    let Some(decl) = ctx.scope.sigma.types.get(&type_path_key(path)) else {
        return true;
    };
    match decl {
        TypeDecl::TypeAlias(alias) => {
            let Some(lowered) = lower_type_for_drop(&alias.r#type, ctx) else {
                return true;
            };
            let alias_type = instantiate_if_generic(lowered, &alias.generic_params, generic_args);
            alias_type.is_none() || needs_drop(&alias_type, ctx, active)
        }
        TypeDecl::Record(record) => {
            for member in &record.members {
                let ast::RecordMember::FieldDecl(field) = member else {
                    continue;
                };
                let Some(lowered) = lower_type_for_drop(&field.r#type, ctx) else {
                    return true;
                };
                let field_type = instantiate_if_generic(lowered, &record.generic_params, generic_args);
                if field_type.is_none() || needs_drop(&field_type, ctx, active) {
                    return true;
                }
            }
            false
        }
        TypeDecl::Enum(enum_decl) => {
            for variant in &enum_decl.variants {
                let types: Vec<Option<Arc<ast::Type>>> = match &variant.payload_opt {
                    None => continue,
                    Some(ast::VariantPayload::VariantPayloadTuple(tuple)) => tuple.elements.clone(),
                    Some(ast::VariantPayload::VariantPayloadRecord(record)) => record.fields.iter().map(|field| field.r#type.clone()).collect(),
                };
                for written in types {
                    let Some(lowered) = lower_type_for_drop(&written, ctx) else {
                        return true;
                    };
                    let payload_type = instantiate_if_generic(lowered, &enum_decl.generic_params, generic_args);
                    if payload_type.is_none() || needs_drop(&payload_type, ctx, active) {
                        return true;
                    }
                }
            }
            false
        }
        TypeDecl::Modal(modal) => modal.states.iter().any(|state| state_fields_need_drop(state, &modal.generic_params, generic_args, ctx, active)),
    }
}

fn needs_drop_impl(ty: &TypeRef, ctx: &LowerCtx, active: &mut HashSet<String>) -> bool {
    let Some(stripped) = strip_perm(ty) else {
        return false;
    };
    if let TypeNode::Refine { base, .. } = &stripped.node {
        return needs_drop(base, ctx, active);
    }
    let stripped_ref: TypeRef = Some(stripped.clone());
    let key = type_to_string(&stripped_ref);
    if !active.insert(key.clone()) {
        return false;
    }
    let result = (|| {
        if is_builtin_drop_type(&stripped_ref) || has_drop_method(&stripped_ref, ctx) {
            return true;
        }
        match &stripped.node {
            TypeNode::Prim(_) | TypeNode::RawPtr { .. } | TypeNode::Ptr { .. } | TypeNode::Func { .. } | TypeNode::Slice(_) | TypeNode::Dynamic(_) => false,
            _ if is_range_type(&stripped_ref) => false,
            TypeNode::String(state) if state.is_none_or(|state| state == StringState::View) => false,
            TypeNode::Bytes(state) if state.is_none_or(|state| state == BytesState::View) => false,
            TypeNode::Array { element, length, .. } => *length != 0 && needs_drop(element, ctx, active),
            TypeNode::Tuple(elements) => elements.iter().any(|element| needs_drop(element, ctx, active)),
            TypeNode::Union(members) => members.iter().any(|member| needs_drop(member, ctx, active)),
            TypeNode::Path { path, generic_args } => nominal_needs_drop(path, generic_args, ctx, active),
            TypeNode::Apply { path, args } => nominal_needs_drop(path, args, ctx, active),
            TypeNode::ModalState(state) => {
                let Some(TypeDecl::Modal(modal)) = ctx.scope.sigma.types.get(&type_path_key(&state.path)) else {
                    return true;
                };
                let Some(block) = modal.states.iter().find(|block| id_eq(&block.name, &state.state)) else {
                    return true;
                };
                state_fields_need_drop(block, &modal.generic_params, &state.generic_args, ctx, active)
            }
            _ => false,
        }
    })();
    active.remove(&key);
    result
}

fn needs_drop(ty: &TypeRef, ctx: &LowerCtx, active: &mut HashSet<String>) -> bool {
    // A refinement is looked through, and a type that is being asked about already is not asked again.
    let mut stripped = strip_perm(ty);
    if let Some(TypeNode::Refine { base, .. }) = stripped.as_deref().map(|ty| &ty.node) {
        stripped = base.clone();
    }
    if stripped.is_some() && active.contains(&type_to_string(&stripped)) {
        return false;
    }
    needs_drop_impl(ty, ctx, active)
}

/// `TypeNeedsDrop`.
pub(super) fn type_needs_drop(ty: &TypeRef, ctx: &LowerCtx) -> bool {
    let mut active = HashSet::new();
    needs_drop(ty, ctx, &mut active)
}
