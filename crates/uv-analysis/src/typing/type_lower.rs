//! Lowering written types to semantic types.

use uv_source::ast;

use super::const_len::const_len;
use super::types::*;
use crate::context::ScopeContext;

/// Why a type could not be lowered; only array lengths name a rule.
pub type LowerError = Option<&'static str>;

pub fn lower_permission(perm: ast::TypePerm) -> Permission {
    match perm {
        ast::TypePerm::Const => Permission::Const,
        ast::TypePerm::Unique => Permission::Unique,
        ast::TypePerm::Shared => Permission::Shared,
    }
}

pub fn lower_param_mode(mode: Option<ast::ParamMode>) -> Option<ParamMode> {
    mode.map(|ast::ParamMode::Move| ParamMode::Move)
}

pub fn lower_raw_ptr_qual(qual: ast::RawPtrQual) -> RawPtrQual {
    match qual {
        ast::RawPtrQual::Imm => RawPtrQual::Imm,
        ast::RawPtrQual::Mut => RawPtrQual::Mut,
    }
}

pub fn lower_string_state(state: Option<ast::StringState>) -> Option<StringState> {
    state.map(|state| match state {
        ast::StringState::Managed => StringState::Managed,
        ast::StringState::View => StringState::View,
    })
}

pub fn lower_bytes_state(state: Option<ast::BytesState>) -> Option<BytesState> {
    state.map(|state| match state {
        ast::BytesState::Managed => BytesState::Managed,
        ast::BytesState::View => BytesState::View,
    })
}

pub fn lower_ptr_state(state: Option<ast::PtrState>) -> Option<PtrState> {
    state.map(|state| match state {
        ast::PtrState::Valid => PtrState::Valid,
        ast::PtrState::Null => PtrState::Null,
        ast::PtrState::Expired => PtrState::Expired,
    })
}

/// The reference lowers written types in three places that differ in small ways; each
/// difference is a field here.
#[derive(Debug, Clone, Copy)]
pub struct LowerFlavor {
    /// `Foo<T>` becomes an application rather than a path with arguments.
    generic_path_as_apply: bool,
    /// Arrays remember how their length was written.
    array_length_text: bool,
    /// Explicit applications and range types are accepted.
    apply_and_ranges: bool,
}

/// Ordinary lowering, as the type checker sees types.
pub const LOWER_TYPE: LowerFlavor =
    LowerFlavor { generic_path_as_apply: true, array_length_text: true, apply_and_ranges: true };

/// Lowering for layout queries.
pub const LOWER_FOR_LAYOUT: LowerFlavor =
    LowerFlavor { generic_path_as_apply: false, array_length_text: false, apply_and_ranges: true };

/// Lowering of modal state fields when deciding how a modal is represented.
pub const LOWER_FOR_MODAL: LowerFlavor =
    LowerFlavor { generic_path_as_apply: false, array_length_text: false, apply_and_ranges: false };

/// The semantic type a written type denotes. Paths are taken as written: lowering runs on
/// resolved syntax. A union is canonicalised.
pub fn lower_type(ctx: &ScopeContext<'_>, type_ptr: &ast::TypePtr) -> Result<TypeRef, LowerError> {
    lower_type_as(ctx, type_ptr, LOWER_TYPE)
}

pub fn lower_type_as(ctx: &ScopeContext<'_>, type_ptr: &ast::TypePtr, flavor: LowerFlavor) -> Result<TypeRef, LowerError> {
    let Some(ty) = type_ptr else {
        return Err(None);
    };
    let lower = |ty: &ast::TypePtr| lower_type_as(ctx, ty, flavor);
    let list = |types: &[ast::TypePtr]| types.iter().map(lower).collect::<Result<Vec<TypeRef>, LowerError>>();
    let range = |base: &ast::TypePtr, make: fn(TypeRef) -> TypeNode| {
        if flavor.apply_and_ranges {
            Ok(make_type(make(lower(base)?)))
        } else {
            Err(None)
        }
    };
    use ast::TypeNode as N;
    Ok(match &ty.node {
        N::TypePrim(node) => make_type_prim(&node.name),
        N::TypePermType(node) => make_type_perm(lower_permission(node.perm), lower(&node.base)?),
        N::TypeUnion(node) => make_type_union(list(&node.types)?),
        N::TypeFunc(node) => {
            let mut params = Vec::with_capacity(node.params.len());
            for param in &node.params {
                params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: lower(&param.r#type)? });
            }
            make_type_func(params, lower(&node.ret)?)
        }
        N::TypeClosure(node) => {
            let mut params = Vec::with_capacity(node.params.len());
            for param in &node.params {
                params.push((param.mode.is_some(), lower(&param.r#type)?));
            }
            let ret = lower(&node.ret)?;
            let deps_opt = match &node.deps_opt {
                Some(deps) => Some(
                    deps.iter()
                        .map(|dep| Ok(SharedDep { name: dep.name.clone(), r#type: lower(&dep.r#type)? }))
                        .collect::<Result<Vec<_>, LowerError>>()?,
                ),
                None => None,
            };
            make_type_closure(params, ret, deps_opt)
        }
        N::TypeTuple(node) => make_type_tuple(list(&node.elements)?),
        N::TypeArray(node) => {
            let element = lower(&node.element)?;
            let length = const_len(ctx, &node.length)?;
            // The length is displayed as the kind of expression it was written as.
            let text = node.length.as_deref().filter(|_| flavor.array_length_text);
            make_type_array(element, length, text.map(|expr| ast::expr_kind(expr).to_string()))
        }
        N::TypeSlice(node) => make_type_slice(lower(&node.element)?),
        N::TypeSafePtr(node) => make_type_ptr(lower(&node.element)?, lower_ptr_state(node.state)),
        N::TypeRawPtr(node) => make_type_raw_ptr(lower_raw_ptr_qual(node.qual), lower(&node.element)?),
        N::TypeString(node) => make_type_string(lower_string_state(node.state)),
        N::TypeBytes(node) => make_type_bytes(lower_bytes_state(node.state)),
        N::TypeDynamic(node) => make_type_dynamic(node.path.clone()),
        N::TypeOpaque(node) => make_type(TypeNode::Opaque {
            class_path: node.path.clone(),
            origin: type_ptr.clone(),
            origin_span: ty.span.clone(),
        }),
        N::TypeRefine(node) => make_type_refine(lower(&node.base)?, node.predicate.clone()),
        N::TypeModalState(node) => {
            let (path, args) = match &node.modal_ref {
                ast::TypeModalRef::TypePathType(modal) => (&modal.path, &modal.generic_args),
                ast::TypeModalRef::TypeApply(modal) => (&modal.path, &modal.args),
            };
            make_type_modal_state(path.clone(), &node.state, list(args)?)
        }
        N::TypePathType(node) if node.generic_args.is_empty() => make_type_path(node.path.clone()),
        N::TypePathType(node) if flavor.generic_path_as_apply => {
            make_type_apply(node.path.clone(), list(&node.generic_args)?)
        }
        N::TypePathType(node) => make_type_path_with(node.path.clone(), list(&node.generic_args)?),
        N::TypeApply(node) if flavor.apply_and_ranges => make_type_apply(node.path.clone(), list(&node.args)?),
        N::TypeApply(_) => return Err(None),
        N::TypeRange(node) => range(&node.base, TypeNode::Range)?,
        N::TypeRangeInclusive(node) => range(&node.base, TypeNode::RangeInclusive)?,
        N::TypeRangeFrom(node) => range(&node.base, TypeNode::RangeFrom)?,
        N::TypeRangeTo(node) => range(&node.base, TypeNode::RangeTo)?,
        N::TypeRangeToInclusive(node) => range(&node.base, TypeNode::RangeToInclusive)?,
        N::TypeRangeFull(_) if flavor.apply_and_ranges => make_type(TypeNode::RangeFull),
        N::TypeRangeFull(_) | N::SpliceExprNode(_) => return Err(None),
    })
}
