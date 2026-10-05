//! `layout_of`, `size_of` and `align_of` for any type, and the layout attributes.

use std::cell::RefCell;

use uv_source::ast;
use uv_source::attributes::attrs;
use uv_source::lexer::token::TokenKind;

use super::aggregates::{decl_substitution, lower_with_subst};
use super::modal::ASYNC_FRAME_PTR_PAYLOAD_OFFSET;
use super::*;
use crate::context::TypeDecl;
use crate::generics::monomorphize::instantiate_type;
use crate::modal::builtin_modal_intrinsics::{is_builtin_runtime_handle_modal_type_path, lookup_builtin_modal_layout};
use crate::resolve::scopes::path_key_of;
use crate::typing::type_lookup::{async_sig_of, lookup_type_decl};
use crate::typing::type_lower::{lower_type_as, LOWER_FOR_LAYOUT};

fn is_single_segment(path: &[String], name: &str) -> bool {
    matches!(path, [only] if only == name)
}

/// `GpuPtr<T, Space>` is a pointer; `Space` is one of the address-space markers.
fn is_gpu_ptr_layout_type(path: &[String], args: &[TypeRef]) -> bool {
    let is_address_space = |ty: &TypeRef| {
        ty.as_deref().is_some_and(|ty| {
            applied_type_args(ty).is_some_and(|args| args.is_empty())
                && applied_type_path(ty)
                    .is_some_and(|path| ["Global", "Shared", "Private"].iter().any(|name| is_single_segment(path, name)))
        })
    };
    is_single_segment(path, "GpuPtr") && matches!(args, [Some(_), space] if is_address_space(space))
}

fn opaque_underlying(ctx: &ScopeContext<'_>, class_path: &[String]) -> Option<TypeRef> {
    ctx.sigma.opaque_underlying_by_class_path.get(&path_key_of(class_path)).filter(|ty| ty.is_some()).cloned()
}

thread_local! {
    /// The types whose layout is being computed, as text.
    static ACTIVE_LAYOUT_QUERIES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Marks a type as being laid out for as long as it lives. A type that is already being
/// laid out is recursive.
struct LayoutQuery {
    key: Option<String>,
}

impl LayoutQuery {
    fn enter(ty: &TypeRef) -> LayoutQuery {
        let key = type_to_string(ty);
        ACTIVE_LAYOUT_QUERIES.with(|queries| {
            let mut queries = queries.borrow_mut();
            if queries.contains(&key) {
                LayoutQuery { key: None }
            } else {
                queries.push(key.clone());
                LayoutQuery { key: Some(key) }
            }
        })
    }

    fn recursive(&self) -> bool {
        self.key.is_none()
    }
}

impl Drop for LayoutQuery {
    fn drop(&mut self) {
        if let Some(key) = &self.key {
            ACTIVE_LAYOUT_QUERIES.with(|queries| {
                let mut queries = queries.borrow_mut();
                if queries.last() == Some(key) {
                    queries.pop();
                }
            });
        }
    }
}

/// A decimal integer literal without a suffix; digit separators are allowed.
fn parse_u64_literal(tok: &uv_source::lexer::token::Token) -> Option<u64> {
    if tok.kind != TokenKind::IntLiteral {
        return None;
    }
    let text: String = tok.lexeme.chars().filter(|c| *c != '_').collect();
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

/// `align(N)` inside `#layout(..)`.
fn parse_layout_align_arg(arg: &ast::AttributeArg) -> Option<u64> {
    if arg.key.as_deref() != Some("align") {
        return None;
    }
    match &arg.value {
        ast::AttributeArgValue::AttributeArgList(nested) => match &nested[..] {
            [ast::AttributeArg { value: ast::AttributeArgValue::Token(token), .. }] => parse_u64_literal(token),
            _ => None,
        },
        ast::AttributeArgValue::Token(_) => None,
    }
}

fn normalize_attr_literal(value: &str) -> &str {
    let quoted = value.len() >= 2
        && ((value.starts_with('"') && value.ends_with('"')) || (value.starts_with('\'') && value.ends_with('\'')));
    if quoted {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

/// Reads `#layout(..)` and `#align(N)`: the alignment, and each bare word of `#layout`
/// through `word`.
fn layout_attribute_args(attr_list: &[ast::AttributeItem], mut word: impl FnMut(&str)) -> Option<u64> {
    let mut min_align = None;
    for attr in attr_list {
        if attr.name.full_name == attrs::LAYOUT {
            for arg in &attr.args {
                if let Some(align) = parse_layout_align_arg(arg) {
                    min_align = Some(align);
                } else if let (None, ast::AttributeArgValue::Token(token)) = (&arg.key, &arg.value) {
                    word(normalize_attr_literal(&token.lexeme));
                }
            }
        } else if attr.name.full_name == attrs::ALIGN {
            if let Some(ast::AttributeArg { value: ast::AttributeArgValue::Token(token), .. }) = attr.args.first() {
                if let Some(align) = parse_u64_literal(token) {
                    min_align = Some(align);
                }
            }
        }
    }
    min_align
}

/// `packed` wins over any requested alignment.
pub fn resolve_record_layout_options(attr_list: &[ast::AttributeItem]) -> RecordLayoutOptions {
    let mut packed = false;
    let min_align = layout_attribute_args(attr_list, |word| packed |= word == "packed");
    RecordLayoutOptions { packed, min_align: if packed { None } else { min_align } }
}

pub fn resolve_enum_layout_options(attr_list: &[ast::AttributeItem]) -> EnumLayoutOptions {
    let mut disc_type = None;
    let min_align = layout_attribute_args(attr_list, |word| {
        if ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"].contains(&word) {
            disc_type = Some(word.to_string());
        }
    });
    EnumLayoutOptions { disc_type, min_align }
}

pub fn lower_type_for_layout(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Option<TypeRef> {
    lower_type_as(ctx, ty, LOWER_FOR_LAYOUT).ok()
}

/// The states of the state machine and the type `resume` returns: one of the states.
pub fn lower_async_type(sig: &AsyncSig) -> LoweredAsyncType {
    let args = vec![sig.out.clone(), sig.input.clone(), sig.result.clone(), sig.err.clone()];
    let mut states = vec!["Suspended", "Completed"];
    if !is_never_type(&sig.err) {
        states.push("Failed");
    }
    let members = states.iter().map(|state| make_type_modal_state(vec!["Async".to_string()], state, args.clone()));
    LoweredAsyncType {
        resume_type: make_type_union(members.collect()),
        states: states.iter().map(|state| state.to_string()).collect(),
    }
}

pub fn lower_async_type_of(ty: &TypeRef) -> Option<LoweredAsyncType> {
    get_async_sig(ty).map(|sig| lower_async_type(&sig))
}

/// An asynchronous value: a one-byte state and room for the largest of the suspended
/// state (yielded value and frame pointer), the result and the error.
fn async_layout(ctx: &ScopeContext<'_>, sig: &AsyncSig) -> Option<Layout> {
    let frame_ptr = make_type_ptr(make_type_prim("u8"), Some(PtrState::Valid));
    let suspended = record_layout_of(ctx, &[sig.out.clone(), frame_ptr], &RecordLayoutOptions::default())?.layout;
    let mut payloads = vec![Some(suspended)];
    for payload in [&sig.result, &sig.err] {
        if payload.is_some() && !is_never_type(payload) && !is_unit_type(payload) {
            payloads.push(layout_of(ctx, payload));
        }
    }
    let mut size = 0;
    let mut align = 1;
    for layout in payloads.into_iter().flatten().filter(|layout| layout.size != 0) {
        size = size.max(layout.size);
        align = align.max(layout.align);
    }
    size = size.max(ASYNC_FRAME_PTR_PAYLOAD_OFFSET + ptr_size(ctx));
    align = align.max(ptr_align(ctx));
    Some(Layout { size: align_up(1 + size, align), align })
}

/// `string` and `bytes` without a state can be either: a discriminant and the larger.
fn modal_string_bytes_layout(ctx: &ScopeContext<'_>) -> Layout {
    let disc = disc_type_layout(1);
    let align = disc.align.max(ptr_align(ctx));
    Layout { size: align_up(disc.size + 3 * ptr_size(ctx), align), align }
}

/// Managed text owns a pointer, a length and a capacity; a view is a pointer and a length.
fn string_bytes_layout(ctx: &ScopeContext<'_>, managed: Option<bool>) -> Layout {
    match managed {
        None => modal_string_bytes_layout(ctx),
        Some(true) => Layout { size: 3 * ptr_size(ctx), align: ptr_align(ctx) },
        Some(false) => Layout { size: 2 * ptr_size(ctx), align: ptr_align(ctx) },
    }
}

fn text_state(node: &TypeNode) -> Option<Option<bool>> {
    match node {
        TypeNode::String(state) => Some(state.map(|state| state == StringState::Managed)),
        TypeNode::Bytes(state) => Some(state.map(|state| state == BytesState::Managed)),
        _ => None,
    }
}

/// The alias's definition with the given arguments substituted.
fn alias_instance(ctx: &ScopeContext<'_>, alias: &ast::TypeAliasDecl, args: &[TypeRef]) -> Option<TypeRef> {
    let lowered = lower_type_for_layout(ctx, &alias.r#type)?;
    match &alias.generic_params {
        Some(params) if !params.params.is_empty() => {
            Some(instantiate_type(&lowered, &decl_substitution(&alias.generic_params, args)?))
        }
        _ => Some(lowered),
    }
}

fn state_fields(ctx: &ScopeContext<'_>, node: &TypeModalState) -> Option<Vec<TypeRef>> {
    let Some(TypeDecl::Modal(decl)) = ctx.sigma.types.get(&node.path) else {
        return None;
    };
    let subst = decl_substitution(&decl.generic_params, &node.generic_args)?;
    let state = decl.states.iter().find(|state| state.name == node.state)?;
    let mut fields = Vec::new();
    for member in &state.members {
        if let ast::StateMember::StateFieldDecl(field) = member {
            fields.push(lower_with_subst(ctx, &field.r#type, &subst)?);
        }
    }
    Some(fields)
}

/// Size and alignment of a type; none when a part of it has no layout (an unknown name,
/// an opaque type not yet revealed). A type met again while it is being laid out is
/// taken to be behind a pointer.
pub fn layout_of(ctx: &ScopeContext<'_>, type_ref: &TypeRef) -> Option<Layout> {
    let ty = type_ref.as_deref()?;
    let ptr = Layout { size: ptr_size(ctx), align: ptr_align(ctx) };
    let two_ptrs = Layout { size: 2 * ptr.size, align: ptr.align };
    let query = LayoutQuery::enter(type_ref);
    if query.recursive() {
        return Some(ptr);
    }
    match &ty.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => return layout_of(ctx, base),
        TypeNode::Opaque { class_path, .. } => return layout_of(ctx, &opaque_underlying(ctx, class_path)?),
        TypeNode::Prim(name) => return Some(Layout { size: prim_size(ctx, name)?, align: prim_align(ctx, name)? }),
        TypeNode::Ptr { .. } | TypeNode::RawPtr { .. } | TypeNode::Func { .. } => return Some(ptr),
        TypeNode::Closure { .. } => return Some(two_ptrs),
        _ => {}
    }
    if let Some(sig) = get_async_sig(type_ref) {
        return async_layout(ctx, &sig);
    }
    if let Some(managed) = text_state(&ty.node) {
        return Some(string_bytes_layout(ctx, managed));
    }
    if is_range_type(type_ref) {
        return Some(range_layout_of(ctx, type_ref)?.layout);
    }
    match &ty.node {
        TypeNode::Dynamic(_) => Some(dyn_layout_of(ctx).layout),
        TypeNode::Slice(_) => Some(two_ptrs),
        TypeNode::Array { element, length, .. } => {
            let element = layout_of(ctx, element)?;
            Some(Layout { size: element.size.wrapping_mul(*length), align: element.align })
        }
        TypeNode::Tuple(elements) => Some(tuple_layout_of(ctx, elements)?.layout),
        TypeNode::Union(members) => Some(union_layout_of(ctx, members)?.layout),
        TypeNode::ModalState(node) => {
            if is_builtin_runtime_handle_modal_type_path(&node.path) {
                return Some(ptr);
            }
            if let Some(builtin) = lookup_builtin_modal_layout(&node.path) {
                return Some(Layout { size: builtin.size, align: builtin.align });
            }
            Some(record_layout_of(ctx, &state_fields(ctx, node)?, &RecordLayoutOptions::default())?.layout)
        }
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args } => {
            if is_builtin_runtime_handle_modal_type_path(path) {
                return Some(ptr);
            }
            if let Some(builtin) = lookup_builtin_modal_layout(path) {
                return Some(Layout { size: builtin.size, align: builtin.align });
            }
            if is_gpu_ptr_layout_type(path, args) {
                return Some(ptr);
            }
            match lookup_type_decl(ctx, path)? {
                TypeDecl::Record(record) => {
                    let subst = decl_substitution(&record.generic_params, args)?;
                    let mut fields = Vec::new();
                    for member in &record.members {
                        if let ast::RecordMember::FieldDecl(field) = member {
                            fields.push(lower_with_subst(ctx, &field.r#type, &subst)?);
                        }
                    }
                    Some(record_layout_of(ctx, &fields, &resolve_record_layout_options(&record.attrs))?.layout)
                }
                TypeDecl::Enum(decl) => {
                    Some(enum_layout_of(ctx, decl, args, &resolve_enum_layout_options(&decl.attrs))?.layout)
                }
                TypeDecl::Modal(decl) => Some(modal_layout_of(ctx, decl, args)?.layout),
                TypeDecl::TypeAlias(alias) => layout_of(ctx, &alias_instance(ctx, alias, args)?),
            }
        }
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum Measure {
    Size,
    Align,
}

/// `size_of` and `align_of` share their structure: simple types are answered directly,
/// everything nominal goes through `layout_of`. `aliases` holds the aliases being
/// expanded: an alias defined through itself has no size (the reference recurses until
/// it crashes there).
fn measure<'c>(
    ctx: &'c ScopeContext<'_>,
    type_ref: &TypeRef,
    what: Measure,
    aliases: &mut Vec<&'c ast::TypeAliasDecl>,
) -> Option<u64> {
    let ty = type_ref.as_deref()?;
    let pick = |layout: Layout| match what {
        Measure::Size => layout.size,
        Measure::Align => layout.align,
    };
    let ptr = Layout { size: ptr_size(ctx), align: ptr_align(ctx) };
    let two_ptrs = Layout { size: 2 * ptr.size, align: ptr.align };
    match &ty.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => return measure(ctx, base, what, aliases),
        TypeNode::Opaque { class_path, .. } => {
            return measure(ctx, &opaque_underlying(ctx, class_path)?, what, aliases)
        }
        TypeNode::Prim(name) => {
            return match what {
                Measure::Size => prim_size(ctx, name),
                Measure::Align => prim_align(ctx, name),
            }
        }
        TypeNode::Ptr { .. } | TypeNode::RawPtr { .. } | TypeNode::Func { .. } => return Some(pick(ptr)),
        TypeNode::Closure { .. } | TypeNode::Dynamic(_) | TypeNode::Slice(_) => return Some(pick(two_ptrs)),
        _ => {}
    }
    // Unlike `layout_of`, user aliases of the asynchronous types are recognised here.
    if let Some(sig) = async_sig_of(ctx, type_ref) {
        return Some(pick(async_layout(ctx, &sig)?));
    }
    if let Some(managed) = text_state(&ty.node) {
        return Some(pick(string_bytes_layout(ctx, managed)));
    }
    if is_range_type(type_ref) {
        return Some(pick(layout_of(ctx, type_ref)?));
    }
    match &ty.node {
        TypeNode::Array { element, length, .. } => {
            let element = measure(ctx, element, what, aliases)?;
            Some(match what {
                Measure::Size => element.wrapping_mul(*length),
                Measure::Align => element,
            })
        }
        TypeNode::Tuple(_) => Some(pick(layout_of(ctx, type_ref)?)),
        TypeNode::Union(members) => Some(pick(union_layout_of(ctx, members)?.layout)),
        TypeNode::ModalState(node) => {
            if is_builtin_runtime_handle_modal_type_path(&node.path) {
                return Some(pick(ptr));
            }
            Some(pick(layout_of(ctx, type_ref)?))
        }
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args } => {
            if is_builtin_runtime_handle_modal_type_path(path) || is_gpu_ptr_layout_type(path, args) {
                return Some(pick(ptr));
            }
            match lookup_type_decl(ctx, path)? {
                TypeDecl::TypeAlias(alias) => {
                    if aliases.iter().any(|expanding| std::ptr::eq(*expanding, alias)) {
                        return None;
                    }
                    aliases.push(alias);
                    let measured = measure(ctx, &alias_instance(ctx, alias, args)?, what, aliases);
                    aliases.pop();
                    measured
                }
                _ => Some(pick(layout_of(ctx, type_ref)?)),
            }
        }
        _ => None,
    }
}

pub fn size_of(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<u64> {
    measure(ctx, ty, Measure::Size, &mut Vec::new())
}

pub fn align_of(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<u64> {
    measure(ctx, ty, Measure::Align, &mut Vec::new())
}
