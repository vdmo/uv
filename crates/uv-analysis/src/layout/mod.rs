//! Layout: the size and alignment of every type, the offsets of fields, and how sums
//! (enums, unions, modals) are represented.

mod aggregates;
mod dispatch;
mod modal;
mod records;
mod unions;

use uv_project::target_profile::{ptr_size_bytes, TargetProfile};

use crate::context::ScopeContext;
use crate::typing::types::*;

pub use aggregates::{
    enum_layout_of, enum_record_payload_member_layout, enum_tuple_payload_member_layout, range_layout_of,
};
pub use dispatch::{
    align_of, layout_of, lower_async_type, lower_async_type_of, lower_type_for_layout, resolve_enum_layout_options,
    resolve_record_layout_options, size_of,
};
pub use modal::modal_layout_of;
pub use records::{record_layout_of, tuple_layout_of};
pub use unions::union_layout_of;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub size: u64,
    pub align: u64,
}

/// An asynchronous type as the state machine it compiles to.
#[derive(Debug, Clone)]
pub struct LoweredAsyncType {
    pub states: Vec<String>,
    pub resume_type: TypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordLayout {
    pub layout: Layout,
    pub offsets: Vec<u64>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecordLayoutOptions {
    pub packed: bool,
    pub min_align: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumLayout {
    pub layout: Layout,
    pub disc_type: String,
    pub payload_size: u64,
    pub payload_align: u64,
}

#[derive(Debug, Clone)]
pub struct EnumPayloadMemberLayout {
    pub r#type: TypeRef,
    pub offset: u64,
    pub payload_size: u64,
    pub payload_align: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnumLayoutOptions {
    pub disc_type: Option<String>,
    pub min_align: Option<u64>,
}

/// A sum type either has a discriminant or hides its alternatives in the invalid values
/// of one payload (a niche).
#[derive(Debug, Clone)]
pub struct UnionLayout {
    pub layout: Layout,
    pub niche: bool,
    pub niche_payload_layout: Option<Layout>,
    pub disc_type: Option<String>,
    pub payload_size: u64,
    pub payload_align: u64,
    pub member_list: Vec<TypeRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalLayout {
    pub layout: Layout,
    pub niche: bool,
    pub niche_payload_layout: Option<Layout>,
    pub disc_type: Option<String>,
    pub payload_size: u64,
    pub payload_align: u64,
}

/// A dynamic class object: a data pointer and a table pointer.
#[derive(Debug, Clone)]
pub struct DynLayout {
    pub layout: Layout,
    pub fields: Vec<TypeRef>,
}

pub fn target_profile_of(ctx: &ScopeContext<'_>) -> TargetProfile {
    ctx.target_profile.unwrap_or(TargetProfile::X86_64SysV)
}

pub fn ptr_size(ctx: &ScopeContext<'_>) -> u64 {
    ptr_size_bytes(target_profile_of(ctx)) as u64
}

/// Pointers are aligned to their size.
pub fn ptr_align(ctx: &ScopeContext<'_>) -> u64 {
    ptr_size(ctx)
}

fn prim_size_with(ptr: u64, name: &str) -> Option<u64> {
    Some(match name {
        "i8" | "u8" | "bool" => 1,
        "i16" | "u16" | "f16" => 2,
        "i32" | "u32" | "f32" | "char" => 4,
        "i64" | "u64" | "f64" => 8,
        "i128" | "u128" => 16,
        "usize" | "isize" => ptr,
        "()" | "!" => 0,
        _ => return None,
    })
}

pub fn prim_size(ctx: &ScopeContext<'_>, name: &str) -> Option<u64> {
    prim_size_with(ptr_size(ctx), name)
}

/// Every primitive is aligned to its size; the zero-sized ones to 1.
pub fn prim_align(ctx: &ScopeContext<'_>, name: &str) -> Option<u64> {
    prim_size(ctx, name).map(|size| size.max(1))
}

pub(crate) fn align_up(value: u64, align: u64) -> u64 {
    match value.checked_rem(align) {
        None | Some(0) => value,
        Some(rem) => value + (align - rem),
    }
}

/// The smallest unsigned type that holds a discriminant.
pub(crate) fn disc_type_name(max_disc: u64) -> &'static str {
    match max_disc {
        0..=0xFF => "u8",
        0x100..=0xFFFF => "u16",
        0x1_0000..=0xFFFF_FFFF => "u32",
        _ => "u64",
    }
}

/// Discriminants chosen by the compiler are laid out as on the 64-bit default target.
pub(crate) fn disc_type_layout(max_disc: u64) -> Layout {
    let size = prim_size_with(8, disc_type_name(max_disc)).expect("discriminant types are primitives");
    Layout { size, align: size }
}

pub(crate) fn is_never_type(ty: &TypeRef) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!")
}

/// `()` written as the primitive or as the empty tuple.
pub(crate) fn is_unit_type(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Prim(name)) => name == "()",
        Some(TypeNode::Tuple(elements)) => elements.is_empty(),
        _ => false,
    }
}

pub fn dyn_layout_of(ctx: &ScopeContext<'_>) -> DynLayout {
    DynLayout {
        layout: Layout { size: 2 * ptr_size(ctx), align: ptr_align(ctx) },
        fields: vec![
            make_type_raw_ptr(RawPtrQual::Imm, make_type_prim("()")),
            make_type_raw_ptr(RawPtrQual::Imm, make_type_path(vec!["VTable".to_string()])),
        ],
    }
}
