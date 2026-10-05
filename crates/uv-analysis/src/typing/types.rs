//! Semantic types: what a written type means once paths are resolved and unions are
//! put in canonical order.

use std::sync::Arc;

use uv_core::span::Span;
use uv_project::deterministic_order::fold;
use uv_source::ast;

use super::type_equiv::type_equiv;
use crate::caps::builtin_paths::path_matches_builtin_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Const,
    Unique,
    Shared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamMode {
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawPtrQual {
    Imm,
    Mut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringState {
    Managed,
    View,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytesState {
    Managed,
    View,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtrState {
    Valid,
    Null,
    Expired,
}

pub type TypePath = Vec<String>;

/// A type, or nothing where the reference has a null type (a missing return type, an
/// operand that failed to lower).
pub type TypeRef = Option<Arc<Type>>;

#[derive(Debug, Clone)]
pub struct TypeFuncParam {
    pub mode: Option<ParamMode>,
    pub r#type: TypeRef,
}

/// The modal a state type belongs to: a bare path or a generic application.
#[derive(Debug, Clone)]
pub enum ModalRef {
    Path(TypePath),
    Apply { path: TypePath, args: Vec<TypeRef> },
}

impl ModalRef {
    pub fn new(path: TypePath, args: Vec<TypeRef>) -> ModalRef {
        if args.is_empty() {
            ModalRef::Path(path)
        } else {
            ModalRef::Apply { path, args }
        }
    }

    pub fn path(&self) -> &TypePath {
        match self {
            ModalRef::Path(path) | ModalRef::Apply { path, .. } => path,
        }
    }

    pub fn args(&self) -> &[TypeRef] {
        match self {
            ModalRef::Path(_) => &[],
            ModalRef::Apply { args, .. } => args,
        }
    }

    pub fn to_type(&self) -> TypeRef {
        match self {
            ModalRef::Path(path) => make_type_path(path.clone()),
            ModalRef::Apply { path, args } => make_type_apply(path.clone(), args.clone()),
        }
    }
}

/// `path` and `generic_args` repeat what `modal_ref` holds.
#[derive(Debug, Clone)]
pub struct TypeModalState {
    pub modal_ref: ModalRef,
    pub path: TypePath,
    pub state: String,
    pub generic_args: Vec<TypeRef>,
}

#[derive(Debug, Clone)]
pub struct SharedDep {
    pub name: String,
    pub r#type: TypeRef,
}

#[derive(Debug, Clone)]
pub enum TypeNode {
    Prim(String),
    Var(u32),
    Perm { perm: Permission, base: TypeRef },
    Union(Vec<TypeRef>),
    Func { params: Vec<TypeFuncParam>, ret: TypeRef },
    Tuple(Vec<TypeRef>),
    /// `length_expr_text` is how the length was written, kept for display.
    Array { element: TypeRef, length: u64, length_expr_text: Option<String> },
    Slice(TypeRef),
    Ptr { element: TypeRef, state: Option<PtrState> },
    RawPtr { qual: RawPtrQual, element: TypeRef },
    String(Option<StringState>),
    Bytes(Option<BytesState>),
    Dynamic(TypePath),
    ModalState(TypeModalState),
    Apply { path: TypePath, args: Vec<TypeRef> },
    Path { path: TypePath, generic_args: Vec<TypeRef> },
    /// `origin` is the written `opaque` type; two occurrences are different types.
    Opaque { class_path: TypePath, origin: ast::TypePtr, origin_span: Span },
    Refine { base: TypeRef, predicate: ast::ExprPtr },
    Range(TypeRef),
    RangeInclusive(TypeRef),
    RangeFrom(TypeRef),
    RangeTo(TypeRef),
    RangeToInclusive(TypeRef),
    RangeFull,
    /// Parameters are `(is_move, type)`.
    Closure { params: Vec<(bool, TypeRef)>, ret: TypeRef, deps_opt: Option<Vec<SharedDep>> },
}

#[derive(Debug, Clone)]
pub struct Type {
    pub node: TypeNode,
}

/// The path of a nominal type, with or without arguments.
pub fn applied_type_path(ty: &Type) -> Option<&TypePath> {
    match &ty.node {
        TypeNode::Path { path, .. } | TypeNode::Apply { path, .. } => Some(path),
        _ => None,
    }
}

pub fn applied_type_args(ty: &Type) -> Option<&[TypeRef]> {
    match &ty.node {
        TypeNode::Path { generic_args, .. } => Some(generic_args),
        TypeNode::Apply { args, .. } => Some(args),
        _ => None,
    }
}

pub fn make_type(node: TypeNode) -> TypeRef {
    Some(Arc::new(Type { node }))
}

pub fn make_type_prim(name: &str) -> TypeRef {
    make_type(TypeNode::Prim(name.to_string()))
}

pub fn make_type_perm(perm: Permission, base: TypeRef) -> TypeRef {
    make_type(TypeNode::Perm { perm, base })
}

fn collect_union_members(member: &TypeRef, out: &mut Vec<TypeRef>) {
    match member.as_deref().map(|ty| &ty.node) {
        None => {}
        Some(TypeNode::Union(nested)) => nested.iter().for_each(|inner| collect_union_members(inner, out)),
        Some(_) => out.push(member.clone()),
    }
}

/// A union is flattened, sorted and stripped of equivalent members; a union of one
/// member is that member.
pub fn make_type_union(members: Vec<TypeRef>) -> TypeRef {
    let mut flat = Vec::with_capacity(members.len());
    for member in &members {
        collect_union_members(member, &mut flat);
    }
    let mut distinct: Vec<TypeRef> = Vec::with_capacity(flat.len());
    for member in sort_union_members(&flat) {
        if !distinct.iter().any(|existing| type_equiv(existing, &member)) {
            distinct.push(member);
        }
    }
    if distinct.len() == 1 {
        return distinct.remove(0);
    }
    make_type(TypeNode::Union(distinct))
}

pub fn make_type_func(params: Vec<TypeFuncParam>, ret: TypeRef) -> TypeRef {
    make_type(TypeNode::Func { params, ret })
}

pub fn make_type_tuple(elements: Vec<TypeRef>) -> TypeRef {
    make_type(TypeNode::Tuple(elements))
}

pub fn make_type_array(element: TypeRef, length: u64, length_expr_text: Option<String>) -> TypeRef {
    make_type(TypeNode::Array { element, length, length_expr_text })
}

pub fn make_type_slice(element: TypeRef) -> TypeRef {
    make_type(TypeNode::Slice(element))
}

pub fn make_type_ptr(element: TypeRef, state: Option<PtrState>) -> TypeRef {
    make_type(TypeNode::Ptr { element, state })
}

pub fn make_type_raw_ptr(qual: RawPtrQual, element: TypeRef) -> TypeRef {
    make_type(TypeNode::RawPtr { qual, element })
}

pub fn make_type_string(state: Option<StringState>) -> TypeRef {
    make_type(TypeNode::String(state))
}

pub fn make_type_bytes(state: Option<BytesState>) -> TypeRef {
    make_type(TypeNode::Bytes(state))
}

pub fn make_type_dynamic(path: TypePath) -> TypeRef {
    make_type(TypeNode::Dynamic(path))
}

pub fn make_type_modal_state(path: TypePath, state: &str, generic_args: Vec<TypeRef>) -> TypeRef {
    let modal_ref = ModalRef::new(path.clone(), generic_args.clone());
    make_type(TypeNode::ModalState(TypeModalState { modal_ref, path, state: state.to_string(), generic_args }))
}

pub fn make_type_path(path: TypePath) -> TypeRef {
    make_type(TypeNode::Path { path, generic_args: Vec::new() })
}

pub fn make_type_path_with(path: TypePath, generic_args: Vec<TypeRef>) -> TypeRef {
    make_type(TypeNode::Path { path, generic_args })
}

pub fn self_var_type() -> TypeRef {
    make_type_path(vec!["Self".to_string()])
}

pub fn is_self_var_path(path: &[String]) -> bool {
    matches!(path, [only] if only == "Self")
}

pub fn make_type_apply(path: TypePath, args: Vec<TypeRef>) -> TypeRef {
    make_type(TypeNode::Apply { path, args })
}

pub fn make_type_refine(base: TypeRef, predicate: ast::ExprPtr) -> TypeRef {
    make_type(TypeNode::Refine { base, predicate })
}

pub fn make_type_closure(params: Vec<(bool, TypeRef)>, ret: TypeRef, deps_opt: Option<Vec<SharedDep>>) -> TypeRef {
    make_type(TypeNode::Closure { params, ret, deps_opt })
}

fn path_to_string(path: &[String]) -> String {
    path.join("::")
}

fn append_opt(out: &mut String, ty: &TypeRef) {
    if let Some(ty) = ty {
        append_type_string(out, ty);
    }
}

fn append_list(out: &mut String, types: &[TypeRef]) {
    for (index, ty) in types.iter().enumerate() {
        if index != 0 {
            out.push_str(", ");
        }
        append_opt(out, ty);
    }
}

fn append_generic_args(out: &mut String, args: &[TypeRef]) {
    if !args.is_empty() {
        out.push('<');
        append_list(out, args);
        out.push('>');
    }
}

fn append_wrapped(out: &mut String, name: &str, base: &TypeRef) {
    out.push_str(name);
    out.push('<');
    append_opt(out, base);
    out.push('>');
}

fn append_type_string(out: &mut String, ty: &Type) {
    match &ty.node {
        TypeNode::Prim(name) => out.push_str(name),
        TypeNode::Var(id) => out.push_str(&format!("$t{id}")),
        TypeNode::Range(base) => append_wrapped(out, "Range", base),
        TypeNode::RangeInclusive(base) => append_wrapped(out, "RangeInclusive", base),
        TypeNode::RangeFrom(base) => append_wrapped(out, "RangeFrom", base),
        TypeNode::RangeTo(base) => append_wrapped(out, "RangeTo", base),
        TypeNode::RangeToInclusive(base) => append_wrapped(out, "RangeToInclusive", base),
        TypeNode::RangeFull => out.push_str("RangeFull"),
        TypeNode::Perm { perm, base } => {
            out.push_str(match perm {
                Permission::Const => "const ",
                Permission::Unique => "unique ",
                Permission::Shared => "shared ",
            });
            append_opt(out, base);
        }
        TypeNode::Union(members) => {
            for (index, member) in members.iter().enumerate() {
                if index > 0 {
                    out.push_str(" | ");
                }
                append_opt(out, member);
            }
        }
        TypeNode::Func { params, ret } => {
            out.push('(');
            for (index, param) in params.iter().enumerate() {
                if index > 0 {
                    out.push_str(", ");
                }
                if param.mode == Some(ParamMode::Move) {
                    out.push_str("move ");
                }
                append_opt(out, &param.r#type);
            }
            out.push_str(") -> ");
            append_opt(out, ret);
        }
        TypeNode::Tuple(elements) => match &elements[..] {
            [] => out.push_str("()"),
            [only] => {
                out.push('(');
                append_opt(out, only);
                out.push_str(";)");
            }
            _ => {
                out.push('(');
                append_list(out, elements);
                out.push(')');
            }
        },
        TypeNode::Array { element, length, length_expr_text } => {
            out.push('[');
            append_opt(out, element);
            out.push_str("; ");
            match length_expr_text.as_deref().filter(|text| !text.is_empty()) {
                Some(text) => out.push_str(text),
                None => out.push_str(&length.to_string()),
            }
            out.push(']');
        }
        TypeNode::Slice(element) => {
            out.push('[');
            append_opt(out, element);
            out.push(']');
        }
        TypeNode::Ptr { element, state } => {
            append_wrapped(out, "Ptr", element);
            out.push_str(match state {
                None => "",
                Some(PtrState::Valid) => "@Valid",
                Some(PtrState::Null) => "@Null",
                Some(PtrState::Expired) => "@Expired",
            });
        }
        TypeNode::RawPtr { qual, element } => {
            out.push_str(if *qual == RawPtrQual::Imm { "* imm " } else { "* mut " });
            append_opt(out, element);
        }
        TypeNode::String(state) => {
            out.push_str("string");
            out.push_str(match state {
                None => "",
                Some(StringState::Managed) => "@Managed",
                Some(StringState::View) => "@View",
            });
        }
        TypeNode::Bytes(state) => {
            out.push_str("bytes");
            out.push_str(match state {
                None => "",
                Some(BytesState::Managed) => "@Managed",
                Some(BytesState::View) => "@View",
            });
        }
        TypeNode::Dynamic(path) => {
            out.push('$');
            out.push_str(&path_to_string(path));
        }
        TypeNode::ModalState(node) => {
            out.push_str(&path_to_string(node.modal_ref.path()));
            append_generic_args(out, node.modal_ref.args());
            out.push('@');
            out.push_str(&node.state);
        }
        TypeNode::Opaque { class_path, .. } => {
            out.push_str("opaque ");
            out.push_str(&path_to_string(class_path));
        }
        TypeNode::Refine { base, .. } => {
            append_opt(out, base);
            out.push_str(" where { ... }");
        }
        TypeNode::Closure { params, ret, .. } => {
            out.push('|');
            for (index, (is_move, param)) in params.iter().enumerate() {
                if index != 0 {
                    out.push_str(", ");
                }
                if *is_move {
                    out.push_str("move ");
                }
                append_opt(out, param);
            }
            out.push_str("| -> ");
            match ret {
                Some(ret) => append_type_string(out, ret),
                None => out.push_str("()"),
            }
        }
        TypeNode::Path { path, generic_args } => {
            out.push_str(&path_to_string(path));
            append_generic_args(out, generic_args);
        }
        TypeNode::Apply { path, args } => {
            out.push_str(&path_to_string(path));
            append_generic_args(out, args);
        }
    }
}

/// The type as diagnostics spell it.
pub fn type_to_string(ty: &TypeRef) -> String {
    let mut out = String::new();
    append_opt(&mut out, ty);
    out
}

/// A structural key that orders types; unions are sorted by it. The derived ordering is
/// the reference's: atoms compare by kind first, text bytewise, lists element by element
/// with the shorter list first.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyAtom {
    Number(u64),
    String(String),
    Key(TypeKey),
    KeyList(Vec<TypeKey>),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeKey {
    pub atoms: Vec<KeyAtom>,
}

/// Paths order by their case-folded spelling, then by the spelling itself.
fn path_order_key(path: &[String]) -> TypeKey {
    let text = path_to_string(path);
    TypeKey { atoms: vec![KeyAtom::String(fold(&text)), KeyAtom::String(text)] }
}

fn tag_key_of(node: &TypeNode) -> u64 {
    match node {
        TypeNode::Prim(_) => 0,
        TypeNode::Tuple(_) => 1,
        TypeNode::Array { .. } => 2,
        TypeNode::Slice(_) => 3,
        TypeNode::Func { .. } => 4,
        TypeNode::Path { .. } | TypeNode::Apply { .. } => 5,
        TypeNode::ModalState(_) => 6,
        TypeNode::String(_) => 7,
        TypeNode::Bytes(_) => 8,
        TypeNode::Dynamic(_) => 9,
        TypeNode::Ptr { .. } => 10,
        TypeNode::RawPtr { .. } => 11,
        TypeNode::Union(_) => 12,
        TypeNode::Perm { .. } => 13,
        TypeNode::Range(_)
        | TypeNode::RangeInclusive(_)
        | TypeNode::RangeFrom(_)
        | TypeNode::RangeTo(_)
        | TypeNode::RangeToInclusive(_)
        | TypeNode::RangeFull => 14,
        TypeNode::Opaque { .. } => 15,
        TypeNode::Refine { .. } => 16,
        TypeNode::Closure { .. } => 17,
        TypeNode::Var(_) => 18,
    }
}

fn key_of(ty: &TypeRef) -> KeyAtom {
    KeyAtom::Key(type_key_of(ty))
}

fn key_list(types: &[TypeRef]) -> KeyAtom {
    KeyAtom::KeyList(types.iter().map(type_key_of).collect())
}

pub fn type_key_of(type_ref: &TypeRef) -> TypeKey {
    let Some(ty) = type_ref else {
        return TypeKey::default();
    };
    use KeyAtom::Number;
    let mut atoms = vec![Number(tag_key_of(&ty.node))];
    match &ty.node {
        TypeNode::Prim(name) => atoms.push(KeyAtom::String(name.clone())),
        TypeNode::Var(id) => atoms.push(Number(u64::from(*id))),
        TypeNode::Range(base) => atoms.extend([Number(0), key_of(base)]),
        TypeNode::RangeInclusive(base) => atoms.extend([Number(1), key_of(base)]),
        TypeNode::RangeFrom(base) => atoms.extend([Number(2), key_of(base)]),
        TypeNode::RangeTo(base) => atoms.extend([Number(3), key_of(base)]),
        TypeNode::RangeToInclusive(base) => atoms.extend([Number(4), key_of(base)]),
        TypeNode::RangeFull => atoms.push(Number(5)),
        TypeNode::Tuple(elements) => {
            atoms.push(Number(elements.len() as u64));
            atoms.extend(elements.iter().map(key_of));
        }
        TypeNode::Array { element, length, .. } => atoms.extend([key_of(element), Number(*length)]),
        TypeNode::Slice(element) => atoms.push(key_of(element)),
        TypeNode::Func { params, ret } => {
            atoms.push(Number(params.len() as u64));
            for param in params {
                atoms.extend([Number(u64::from(param.mode.is_some())), key_of(&param.r#type)]);
            }
            atoms.push(key_of(ret));
        }
        TypeNode::Path { path, generic_args } => {
            atoms.push(KeyAtom::Key(path_order_key(path)));
            if !generic_args.is_empty() {
                atoms.push(key_list(generic_args));
            }
        }
        TypeNode::Apply { path, args } => atoms.extend([KeyAtom::Key(path_order_key(path)), key_list(args)]),
        TypeNode::ModalState(node) => {
            atoms.push(KeyAtom::Key(path_order_key(node.modal_ref.path())));
            if !node.modal_ref.args().is_empty() {
                atoms.push(key_list(node.modal_ref.args()));
            }
            atoms.push(KeyAtom::String(node.state.clone()));
        }
        // An unstated state sorts after the stated ones.
        TypeNode::String(state) => atoms.push(Number(match state {
            Some(StringState::View) => 0,
            Some(StringState::Managed) => 1,
            None => 2,
        })),
        TypeNode::Bytes(state) => atoms.push(Number(match state {
            Some(BytesState::View) => 0,
            Some(BytesState::Managed) => 1,
            None => 2,
        })),
        TypeNode::Dynamic(path) => atoms.push(KeyAtom::Key(path_order_key(path))),
        TypeNode::Opaque { class_path, origin_span, .. } => atoms.extend([
            KeyAtom::Key(path_order_key(class_path)),
            KeyAtom::String(origin_span.file.to_string()),
            Number(origin_span.start_offset as u64),
            Number(origin_span.end_offset as u64),
        ]),
        TypeNode::Refine { base, predicate } => {
            atoms.push(key_of(base));
            if let Some(predicate) = predicate {
                atoms.extend([
                    KeyAtom::String(predicate.span.file.to_string()),
                    Number(predicate.span.start_offset as u64),
                    Number(predicate.span.end_offset as u64),
                ]);
            }
        }
        TypeNode::Ptr { element, state } => atoms.extend([
            Number(match state {
                None => 0,
                Some(PtrState::Valid) => 1,
                Some(PtrState::Null) => 2,
                Some(PtrState::Expired) => 3,
            }),
            key_of(element),
        ]),
        TypeNode::RawPtr { qual, element } => {
            atoms.extend([Number(u64::from(*qual == RawPtrQual::Mut)), key_of(element)]);
        }
        TypeNode::Union(members) => {
            let mut member_keys: Vec<TypeKey> = members.iter().map(type_key_of).collect();
            member_keys.sort();
            atoms.push(KeyAtom::KeyList(member_keys));
        }
        TypeNode::Perm { perm, base } => atoms.extend([
            Number(match perm {
                Permission::Const => 0,
                Permission::Unique => 1,
                Permission::Shared => 2,
            }),
            key_of(base),
        ]),
        TypeNode::Closure { params, ret, deps_opt } => {
            let mut param_keys = Vec::new();
            for (is_move, param) in params {
                param_keys.push(TypeKey { atoms: vec![Number(u64::from(*is_move))] });
                if param.is_some() {
                    param_keys.push(type_key_of(param));
                }
            }
            atoms.push(KeyAtom::KeyList(param_keys));
            if ret.is_some() {
                atoms.push(key_of(ret));
            }
            atoms.push(Number(u64::from(deps_opt.is_some())));
            if let Some(deps) = deps_opt {
                let dep_keys = deps
                    .iter()
                    .map(|dep| TypeKey { atoms: vec![KeyAtom::String(dep.name.clone()), key_of(&dep.r#type)] })
                    .collect();
                atoms.push(KeyAtom::KeyList(dep_keys));
            }
        }
    }
    TypeKey { atoms }
}

/// The members in key order; members with equal keys keep their relative order.
pub fn sort_union_members(members: &[TypeRef]) -> Vec<TypeRef> {
    let mut entries: Vec<(TypeKey, TypeRef)> =
        members.iter().filter(|member| member.is_some()).map(|member| (type_key_of(member), member.clone())).collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries.into_iter().map(|(_, member)| member).collect()
}

fn collect_type_paths(type_ref: &TypeRef, out: &mut Vec<TypePath>) {
    let Some(ty) = type_ref else {
        return;
    };
    let each = |types: &[TypeRef], out: &mut Vec<TypePath>| types.iter().for_each(|ty| collect_type_paths(ty, out));
    match &ty.node {
        TypeNode::Prim(_) | TypeNode::Var(_) | TypeNode::RangeFull | TypeNode::String(_) | TypeNode::Bytes(_) => {}
        TypeNode::Range(base)
        | TypeNode::RangeInclusive(base)
        | TypeNode::RangeFrom(base)
        | TypeNode::RangeTo(base)
        | TypeNode::RangeToInclusive(base)
        | TypeNode::Perm { base, .. }
        | TypeNode::Refine { base, .. } => collect_type_paths(base, out),
        TypeNode::Tuple(types) | TypeNode::Union(types) => each(types, out),
        TypeNode::Array { element, .. }
        | TypeNode::Slice(element)
        | TypeNode::Ptr { element, .. }
        | TypeNode::RawPtr { element, .. } => collect_type_paths(element, out),
        TypeNode::Func { params, ret } => {
            params.iter().for_each(|param| collect_type_paths(&param.r#type, out));
            collect_type_paths(ret, out);
        }
        TypeNode::Dynamic(path) => out.push(path.clone()),
        TypeNode::ModalState(node) => {
            out.push(node.modal_ref.path().clone());
            each(node.modal_ref.args(), out);
        }
        TypeNode::Opaque { class_path, .. } => out.push(class_path.clone()),
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args } => {
            out.push(path.clone());
            each(args, out);
        }
        TypeNode::Closure { params, ret, .. } => {
            params.iter().for_each(|(_, param)| collect_type_paths(param, out));
            collect_type_paths(ret, out);
        }
    }
}

/// The nominal paths a type mentions, in path order, each once.
pub fn type_paths(ty: &TypeRef) -> Vec<TypePath> {
    let mut out = Vec::new();
    collect_type_paths(ty, &mut out);
    out.sort_by_cached_key(|path| path_order_key(path));
    out.dedup();
    out
}

/// The type under any permissions and refinements.
fn strip_perm_and_refine(ty: &TypeRef) -> &TypeRef {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    current
}

pub fn is_range_type(ty: &TypeRef) -> bool {
    matches!(
        strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node),
        Some(
            TypeNode::Range(_)
                | TypeNode::RangeInclusive(_)
                | TypeNode::RangeFrom(_)
                | TypeNode::RangeTo(_)
                | TypeNode::RangeToInclusive(_)
                | TypeNode::RangeFull
        )
    )
}

pub fn range_element_type(ty: &TypeRef) -> Option<TypeRef> {
    match &strip_perm_and_refine(ty).as_deref()?.node {
        TypeNode::Range(base)
        | TypeNode::RangeInclusive(base)
        | TypeNode::RangeFrom(base)
        | TypeNode::RangeTo(base)
        | TypeNode::RangeToInclusive(base) => Some(base.clone()),
        _ => None,
    }
}

/// A range that can index: the full range, or one over `usize`.
pub fn is_range_index_type(ty: &TypeRef) -> bool {
    let stripped = strip_perm_and_refine(ty);
    match stripped.as_deref().map(|ty| &ty.node) {
        None => false,
        Some(TypeNode::RangeFull) => true,
        Some(_) => range_element_type(stripped)
            .is_some_and(|elem| matches!(elem.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize")),
    }
}

pub fn is_async_modal_path(path: &[String]) -> bool {
    path_matches_builtin_name(path, "Async")
}

pub fn is_async_alias_path(path: &[String]) -> bool {
    ["Future", "Sequence", "Stream", "Pipe", "Exchange"].iter().any(|name| path_matches_builtin_name(path, name))
}

/// What an asynchronous computation yields, takes on resume, completes with and fails with.
#[derive(Debug, Clone)]
pub struct AsyncSig {
    pub out: TypeRef,
    pub input: TypeRef,
    pub result: TypeRef,
    pub err: TypeRef,
}

fn async_sig_from_builtin_name(name: &str, args: &[TypeRef]) -> Option<AsyncSig> {
    let unit = || make_type_prim("()");
    let never = || make_type_prim("!");
    let arg = |index: usize| args.get(index).cloned();
    let sig = |out, input, result, err| Some(AsyncSig { out, input, result, err });
    match (name, args.len()) {
        ("Async", 1..=4) => {
            sig(arg(0)?, arg(1).unwrap_or_else(unit), arg(2).unwrap_or_else(unit), arg(3).unwrap_or_else(never))
        }
        ("Future", 1..=2) => sig(unit(), unit(), arg(0)?, arg(1).unwrap_or_else(never)),
        ("Sequence", 1) => sig(arg(0)?, unit(), unit(), never()),
        ("Stream", 2) => sig(arg(0)?, unit(), unit(), arg(1)?),
        ("Pipe", 2) => sig(arg(1)?, arg(0)?, unit(), never()),
        ("Exchange", 1) => sig(arg(0)?, arg(0)?, arg(0)?, never()),
        _ => None,
    }
}

/// The signature of `Async<..>`, one of its states, or one of the built-in aliases.
pub fn get_async_sig(ty: &TypeRef) -> Option<AsyncSig> {
    match &ty.as_deref()?.node {
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args }
            if is_async_modal_path(path) || is_async_alias_path(path) =>
        {
            async_sig_from_builtin_name(path.last()?, args)
        }
        TypeNode::ModalState(node) if is_async_modal_path(node.modal_ref.path()) => {
            async_sig_from_builtin_name("Async", node.modal_ref.args())
        }
        _ => None,
    }
}

pub fn is_async_type(ty: &TypeRef) -> bool {
    get_async_sig(ty).is_some()
}
