//! Enums and ranges.

use uv_source::ast;

use super::*;
use crate::composite::enums::enum_discriminants;
use crate::generics::monomorphize::{build_substitution, instantiate_type, TypeSubst};

/// The substitution for a declaration's parameters; none when there are too many
/// arguments.
pub(crate) fn decl_substitution(generic_params: &Option<ast::GenericParams>, generic_args: &[TypeRef]) -> Option<TypeSubst> {
    match generic_params {
        Some(params) if !params.params.is_empty() => {
            (generic_args.len() <= params.params.len()).then(|| build_substitution(&params.params, generic_args))
        }
        _ => Some(TypeSubst::new()),
    }
}

pub(crate) fn lower_with_subst(ctx: &ScopeContext<'_>, ty: &ast::TypePtr, subst: &TypeSubst) -> Option<TypeRef> {
    let lowered = lower_type_for_layout(ctx, ty)?;
    Some(if subst.is_empty() { lowered } else { instantiate_type(&lowered, subst) })
}

/// A range is a record of its bounds.
pub fn range_layout_of(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<RecordLayout> {
    let mut stripped = ty;
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = stripped.as_deref().map(|ty| &ty.node) {
        stripped = base;
    }
    let fields = match &stripped.as_deref()?.node {
        TypeNode::Range(base) | TypeNode::RangeInclusive(base) => vec![base.clone(), base.clone()],
        TypeNode::RangeFrom(base) | TypeNode::RangeTo(base) | TypeNode::RangeToInclusive(base) => vec![base.clone()],
        TypeNode::RangeFull => Vec::new(),
        _ => return None,
    };
    record_layout_of(ctx, &fields, &RecordLayoutOptions::default())
}

fn disc_max_value(name: &str) -> Option<u64> {
    Some(match name {
        "u8" => 0xFF,
        "u16" => 0xFFFF,
        "u32" => 0xFFFF_FFFF,
        "u64" => u64::MAX,
        "i8" => 0x7F,
        "i16" => 0x7FFF,
        "i32" => 0x7FFF_FFFF,
        "i64" => 0x7FFF_FFFF_FFFF_FFFF,
        _ => return None,
    })
}

/// The lowered member types of a variant's payload, in order.
fn payload_types(ctx: &ScopeContext<'_>, payload: &ast::VariantPayload, subst: &TypeSubst) -> Option<Vec<TypeRef>> {
    match payload {
        ast::VariantPayload::VariantPayloadTuple(tuple) => {
            tuple.elements.iter().map(|elem| lower_with_subst(ctx, elem, subst)).collect()
        }
        ast::VariantPayload::VariantPayloadRecord(record) => {
            record.fields.iter().map(|field| lower_with_subst(ctx, &field.r#type, subst)).collect()
        }
    }
}

/// An enum is its discriminant followed by room for the largest payload.
pub fn enum_layout_of(
    ctx: &ScopeContext<'_>,
    decl: &ast::EnumDecl,
    generic_args: &[TypeRef],
    options: &EnumLayoutOptions,
) -> Option<EnumLayout> {
    if decl.variants.is_empty() {
        return None;
    }
    let subst = decl_substitution(&decl.generic_params, generic_args)?;
    let discs = enum_discriminants(decl).ok()?;
    let disc_type = options.disc_type.clone().unwrap_or_else(|| disc_type_name(discs.max_disc).to_string());
    let disc = Layout { size: prim_size(ctx, &disc_type)?, align: prim_align(ctx, &disc_type)? };
    if discs.max_disc > disc_max_value(&disc_type)? {
        return None;
    }
    let mut payload_size = 0;
    let mut payload_align = 1;
    for variant in &decl.variants {
        if let Some(payload) = &variant.payload_opt {
            let layout = tuple_layout_of(ctx, &payload_types(ctx, payload, &subst)?)?.layout;
            payload_size = payload_size.max(layout.size);
            payload_align = payload_align.max(layout.align);
        }
    }
    let align = disc.align.max(payload_align).max(options.min_align.unwrap_or(1));
    let size = align_up(disc.size + payload_size, align);
    Some(EnumLayout { layout: Layout { size, align }, disc_type, payload_size, payload_align })
}

fn payload_member(
    ctx: &ScopeContext<'_>,
    decl: &ast::EnumDecl,
    variant: &ast::VariantDecl,
    generic_args: &[TypeRef],
    index_of: impl FnOnce(&ast::VariantPayload) -> Option<usize>,
) -> Option<EnumPayloadMemberLayout> {
    let enum_layout = enum_layout_of(ctx, decl, generic_args, &resolve_enum_layout_options(&decl.attrs))?;
    let payload = variant.payload_opt.as_ref()?;
    let index = index_of(payload)?;
    let subst = decl_substitution(&decl.generic_params, generic_args)?;
    let types = payload_types(ctx, payload, &subst)?;
    let layout = tuple_layout_of(ctx, &types)?;
    Some(EnumPayloadMemberLayout {
        r#type: types.get(index)?.clone(),
        offset: *layout.offsets.get(index)?,
        payload_size: enum_layout.payload_size,
        payload_align: enum_layout.payload_align,
    })
}

/// Where the `index`-th element of a tuple payload sits within the payload.
pub fn enum_tuple_payload_member_layout(
    ctx: &ScopeContext<'_>,
    decl: &ast::EnumDecl,
    variant: &ast::VariantDecl,
    generic_args: &[TypeRef],
    index: usize,
) -> Option<EnumPayloadMemberLayout> {
    payload_member(ctx, decl, variant, generic_args, |payload| match payload {
        ast::VariantPayload::VariantPayloadTuple(tuple) if index < tuple.elements.len() => Some(index),
        _ => None,
    })
}

/// Where a named field of a record payload sits within the payload.
pub fn enum_record_payload_member_layout(
    ctx: &ScopeContext<'_>,
    decl: &ast::EnumDecl,
    variant: &ast::VariantDecl,
    generic_args: &[TypeRef],
    field_name: &str,
) -> Option<EnumPayloadMemberLayout> {
    payload_member(ctx, decl, variant, generic_args, |payload| match payload {
        ast::VariantPayload::VariantPayloadRecord(record) => {
            record.fields.iter().position(|field| field.name == field_name)
        }
        ast::VariantPayload::VariantPayloadTuple(_) => None,
    })
}
