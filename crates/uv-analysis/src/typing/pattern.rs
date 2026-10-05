//! Typing patterns against the type of the value they match.
//!
//! A pattern checked against a union is checked against each member; the bindings of
//! the members that accept it are merged, a binding whose type differs between members
//! becoming a union. A permission on the matched type carries over to every binding.

use std::collections::HashSet;
use std::sync::Arc;

use uv_source::ast;
use uv_source::lexer::token::TokenKind;

use super::literals::type_literal_expr;
use super::subtyping::subtyping;
use super::type_equiv::type_equiv;
use super::type_lookup::{field_exists, field_type, field_visible, lookup_enum_decl, lookup_record_decl};
use super::type_lower::lower_type;
use super::types::*;
use crate::context::ScopeContext;
use crate::generics::generic_params::{bind_type_params, required_param_count, total_param_count};
use crate::generics::monomorphize::{build_modal_ref_substitution, build_substitution, instantiate_type, TypeSubst};
use crate::layout::value_bits::parse_int_literal_value;
use crate::modal::lookup::{has_state, lookup_modal_decl};
use crate::resolve::collect_toplevel::pat_names_ptr;
use crate::resolve::scopes::{id_eq, id_key_of};

pub type Bindings = Vec<(String, TypeRef)>;

/// The names a pattern binds with their types, or why it does not fit; a failure may
/// name no rule.
pub type PatternTypeResult = Result<Bindings, Option<&'static str>>;

fn strip_perm_once(ty: &TypeRef) -> &TypeRef {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. }) => base,
        _ => ty,
    }
}

fn type_equiv_ignore_perm(lhs: &TypeRef, rhs: &TypeRef) -> bool {
    type_equiv(strip_perm_once(lhs), strip_perm_once(rhs))
}

fn is_int_type_name(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "isize" | "usize")
}

fn is_unit(ty: &TypeRef) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "()")
}

/// The value of a range bound: an integer literal pattern.
fn const_pat_int(pattern: &ast::PatternPtr) -> Option<u128> {
    match &pattern.as_deref()?.node {
        ast::PatternNode::LiteralPattern(lit) if lit.literal.kind == TokenKind::IntLiteral => {
            parse_int_literal_value(&lit.literal.lexeme)
        }
        _ => None,
    }
}

fn type_path_eq_local(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| id_key_of(a) == id_key_of(b))
}

/// What the payload types of a generic enum are lowered and instantiated with.
struct EnumPatternGenericContext<'a> {
    payload_ctx: ScopeContext<'a>,
    subst: TypeSubst,
}

fn build_enum_pattern_generic_context<'a>(
    ctx: &ScopeContext<'a>,
    decl: &ast::EnumDecl,
    generic_args: &[TypeRef],
) -> Result<EnumPatternGenericContext<'a>, &'static str> {
    let mut payload_ctx = ctx.clone();
    payload_ctx.scopes = bind_type_params(ctx, &decl.generic_params);
    let subst = match &decl.generic_params {
        Some(params) => {
            let provided = generic_args.len();
            if provided < required_param_count(&decl.generic_params) || provided > total_param_count(&decl.generic_params) {
                return Err("E-TYP-2303");
            }
            build_substitution(&params.params, generic_args)
        }
        None if !generic_args.is_empty() => return Err("E-TYP-2303"),
        None => TypeSubst::new(),
    };
    Ok(EnumPatternGenericContext { payload_ctx, subst })
}

fn enum_field_type(
    payload: &ast::VariantPayloadRecord,
    ctx: &ScopeContext<'_>,
    name: &str,
    subst: &TypeSubst,
) -> Option<TypeRef> {
    let field = payload.fields.iter().find(|field| id_key_of(&field.name) == id_key_of(name))?;
    Some(instantiate_type(&lower_type(ctx, &field.r#type).ok()?, subst))
}

fn modal_field_type(
    decl: &ast::ModalDecl,
    state: &str,
    ctx: &ScopeContext<'_>,
    name: &str,
    modal_args: &[TypeRef],
) -> Option<TypeRef> {
    let modal_subst = match &decl.generic_params {
        Some(params) if modal_args.len() > params.params.len() => return None,
        Some(params) => build_modal_ref_substitution(&params.params, modal_args),
        None => TypeSubst::new(),
    };
    let state_key = id_key_of(state);
    let field = decl
        .states
        .iter()
        .filter(|block| id_key_of(&block.name) == state_key)
        .flat_map(|block| &block.members)
        .find_map(|member| match member {
            ast::StateMember::StateFieldDecl(field) if id_key_of(&field.name) == id_key_of(name) => Some(field),
            _ => None,
        })?;
    Some(instantiate_type(&lower_type(ctx, &field.r#type).ok()?, &modal_subst))
}

fn distinct_pat_names(names: &[String]) -> bool {
    let mut seen = HashSet::new();
    names.iter().all(|name| seen.insert(id_key_of(name)))
}

/// A typed pattern binds its name unless the name is `_`.
fn typed_pattern_bindings(name: &str, ty: &TypeRef) -> Bindings {
    if name == "_" {
        Vec::new()
    } else {
        vec![(name.to_string(), ty.clone())]
    }
}

/// The pattern of a field: the one written, or a binding of the field's own name.
fn field_pattern(field: &ast::FieldPattern) -> ast::PatternPtr {
    if field.pattern_opt.is_some() {
        return field.pattern_opt.clone();
    }
    let node = ast::PatternNode::IdentifierPattern(ast::IdentifierPattern { name: field.name.clone(), name_splice_opt: None });
    Some(Arc::new(ast::Pattern { span: Default::default(), node }))
}

/// Member bindings must agree in number and names; types that differ become a union.
fn merge_union_pattern_bindings(bindings: &mut Bindings, member_bindings: &Bindings) -> Result<(), Option<&'static str>> {
    if bindings.len() != member_bindings.len()
        || bindings.iter().zip(member_bindings).any(|(have, new)| have.0 != new.0)
    {
        // The reference compares pairwise and may have widened earlier pairs before
        // finding a mismatch; the result is discarded either way.
        return Err(None);
    }
    for (have, new) in bindings.iter_mut().zip(member_bindings) {
        if !type_equiv_ignore_perm(&have.1, &new.1) {
            have.1 = make_type_union(vec![have.1.clone(), new.1.clone()]);
        }
    }
    Ok(())
}

fn find_variant<'d>(decl: &'d ast::EnumDecl, name: &str) -> Option<&'d ast::VariantDecl> {
    decl.variants.iter().find(|variant| id_eq(&variant.name, name))
}

fn type_fields(
    ctx: &ScopeContext<'_>,
    fields: &[ast::FieldPattern],
    mut field_type_of: impl FnMut(&ast::FieldPattern) -> Result<TypeRef, Option<&'static str>>,
) -> PatternTypeResult {
    let mut binds = Bindings::new();
    for field in fields {
        let ty = field_type_of(field)?;
        binds.extend(type_pattern_impl(ctx, &field_pattern(field), &ty)?);
    }
    Ok(binds)
}

fn type_pattern_impl(ctx: &ScopeContext<'_>, pattern_ptr: &ast::PatternPtr, expected: &TypeRef) -> PatternTypeResult {
    let (Some(pattern), Some(expected_ty)) = (pattern_ptr.as_deref(), expected.as_deref()) else {
        return Err(None);
    };
    if !distinct_pat_names(&pat_names_ptr(pattern_ptr)) {
        return Err(Some("Pat-Dup-Err"));
    }
    use ast::PatternNode as P;
    let whole_value = matches!(pattern.node, P::WildcardPattern(_) | P::IdentifierPattern(_));
    if let (false, Some(TypeNode::Union(members))) = (whole_value, strip_perm_once(expected).as_deref().map(|ty| &ty.node)) {
        let mut merged: Option<Bindings> = None;
        let mut first_diag = None;
        for member in members {
            match type_pattern_against_type(ctx, pattern_ptr, member) {
                Err(diag_id) => first_diag = first_diag.or(diag_id),
                Ok(member_bindings) => match &mut merged {
                    None => merged = Some(member_bindings),
                    Some(bindings) => merge_union_pattern_bindings(bindings, &member_bindings)?,
                },
            }
        }
        return merged.ok_or(first_diag);
    }
    match &pattern.node {
        P::WildcardPattern(_) => Ok(Vec::new()),
        P::IdentifierPattern(node) => Ok(vec![(node.name.clone(), expected.clone())]),
        P::LiteralPattern(node) => {
            let typed = type_literal_expr(&ast::LiteralExpr { literal: node.literal.clone() });
            if !typed.ok {
                return Err(typed.diag_id);
            }
            let sub = subtyping(ctx, &typed.r#type, expected);
            if !sub.ok {
                return Err(sub.diag_id);
            }
            if sub.subtype {
                Ok(Vec::new())
            } else {
                Err(None)
            }
        }
        P::TypedPattern(node) => {
            let lowered = lower_type(ctx, &node.r#type)?;
            let bound = || Ok(typed_pattern_bindings(&node.name, &lowered));
            if let TypeNode::Union(members) = &expected_ty.node {
                return if members.iter().any(|member| type_equiv_ignore_perm(&lowered, member)) { bound() } else { Err(None) };
            }
            if type_equiv(&lowered, expected) {
                return bound();
            }
            // A state of the matched modal, with the same arguments.
            if let Some(TypeNode::ModalState(modal)) = lowered.as_deref().map(|ty| &ty.node) {
                if let Some((path, args)) = applied_type_path(expected_ty).zip(applied_type_args(expected_ty)) {
                    let same = modal.path == *path
                        && modal.generic_args.len() == args.len()
                        && modal.generic_args.iter().zip(args).all(|(a, b)| type_equiv(a, b));
                    if same {
                        return bound();
                    }
                }
            }
            Err(None)
        }
        P::TuplePattern(node) => {
            if node.elements.is_empty() {
                return if is_unit(expected) { Ok(Vec::new()) } else { Err(None) };
            }
            let TypeNode::Tuple(elements) = &expected_ty.node else {
                return Err(None);
            };
            if elements.len() != node.elements.len() {
                return Err(Some("E-TYP-1803"));
            }
            let mut binds = Bindings::new();
            for (pattern, ty) in node.elements.iter().zip(elements) {
                binds.extend(type_pattern_impl(ctx, pattern, ty)?);
            }
            Ok(binds)
        }
        P::RecordPattern(node) => {
            let TypeNode::Path { path, generic_args } = &expected_ty.node else {
                return Err(None);
            };
            if *path != node.path {
                return Err(None);
            }
            let record = lookup_record_decl(ctx, &node.path).ok_or(None)?;
            type_fields(ctx, &node.fields, |field| {
                if !field_exists(record, &field.name) {
                    return Err(Some("RecordPattern-UnknownField"));
                }
                if !field_visible(ctx, record, &field.name, &node.path) {
                    return Err(None);
                }
                field_type(record, &field.name, ctx, generic_args).ok_or(None)
            })
        }
        P::EnumPattern(node) => {
            let (path, args) = applied_type_path(expected_ty).zip(applied_type_args(expected_ty)).ok_or(None)?;
            if !type_path_eq_local(path, &node.path) {
                return Err(None);
            }
            let decl = lookup_enum_decl(ctx, path).ok_or(None)?;
            let generic_ctx = build_enum_pattern_generic_context(ctx, decl, args).map_err(Some)?;
            let variant = find_variant(decl, &node.name).ok_or(None)?;
            match (&variant.payload_opt, &node.payload_opt) {
                (None, None) => Ok(Vec::new()),
                (
                    Some(ast::VariantPayload::VariantPayloadTuple(payload)),
                    Some(ast::EnumPayloadPattern::TuplePayloadPattern(tuple)),
                ) if tuple.elements.len() == payload.elements.len() => {
                    let mut binds = Bindings::new();
                    for (pattern, written) in tuple.elements.iter().zip(&payload.elements) {
                        let lowered = lower_type(&generic_ctx.payload_ctx, written)?;
                        let elem_type = instantiate_type(&lowered, &generic_ctx.subst);
                        binds.extend(type_pattern_impl(ctx, pattern, &elem_type)?);
                    }
                    Ok(binds)
                }
                (
                    Some(ast::VariantPayload::VariantPayloadRecord(payload)),
                    Some(ast::EnumPayloadPattern::RecordPayloadPattern(record)),
                ) => type_fields(ctx, &record.fields, |field| {
                    enum_field_type(payload, &generic_ctx.payload_ctx, &field.name, &generic_ctx.subst).ok_or(None)
                }),
                _ => Err(None),
            }
        }
        P::ModalPattern(node) => {
            let (decl, modal_args) = modal_of_pattern(ctx, expected_ty, &node.state).ok_or(None)?;
            let fields = node.fields_opt.as_ref().map_or(&[][..], |payload| &payload.fields);
            type_fields(ctx, fields, |field| modal_field_type(decl, &node.state, ctx, &field.name, &modal_args).ok_or(None))
        }
        P::RangePattern(node) => {
            let (Some(lo), Some(hi)) = (const_pat_int(&node.lo), const_pat_int(&node.hi)) else {
                return Err(Some("RangePattern-NonConst"));
            };
            if !matches!(&expected_ty.node, TypeNode::Prim(name) if is_int_type_name(name)) {
                return Err(None);
            }
            let non_empty = match node.kind {
                ast::RangeKind::Exclusive => lo < hi,
                ast::RangeKind::Inclusive => lo <= hi,
                _ => return Err(None),
            };
            if non_empty {
                Ok(Vec::new())
            } else {
                Err(Some("RangePattern-Empty"))
            }
        }
        P::SpliceExprNode(_) => Err(None),
    }
}

/// The modal a state pattern is matched against, with its arguments: the matched type
/// is the modal itself, or that very state of it.
fn modal_of_pattern<'c>(
    ctx: &'c ScopeContext<'_>,
    expected: &Type,
    state: &str,
) -> Option<(&'c ast::ModalDecl, Vec<TypeRef>)> {
    let (decl, args) = if let Some(path) = applied_type_path(expected) {
        (lookup_modal_decl(ctx, path)?, applied_type_args(expected).map(<[TypeRef]>::to_vec).unwrap_or_default())
    } else if let TypeNode::ModalState(modal) = &expected.node {
        if !id_eq(&modal.state, state) {
            return None;
        }
        (lookup_modal_decl(ctx, &modal.path)?, modal.generic_args.clone())
    } else {
        return None;
    };
    has_state(decl, state).then_some((decl, args))
}

pub fn type_pattern_against_type(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> PatternTypeResult {
    if pattern.is_none() || expected.is_none() {
        return Err(None);
    }
    let mut bindings = type_pattern_impl(ctx, pattern, strip_perm_once(expected))?;
    if let Some(TypeNode::Perm { perm, .. }) = expected.as_deref().map(|ty| &ty.node) {
        for (_, ty) in &mut bindings {
            *ty = make_type_perm(*perm, ty.clone());
        }
    }
    Ok(bindings)
}

/// Whether the pattern matches every value of the type.
pub fn irrefutable_pattern(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> bool {
    let Some(pattern) = pattern.as_deref() else {
        return false;
    };
    let base_ref = strip_perm_once(expected);
    let Some(base) = base_ref.as_deref() else {
        return false;
    };
    use ast::PatternNode as P;
    match &pattern.node {
        P::WildcardPattern(_) | P::IdentifierPattern(_) => true,
        P::TuplePattern(node) => {
            if node.elements.is_empty() {
                return is_unit(base_ref);
            }
            matches!(&base.node, TypeNode::Tuple(elements) if elements.len() == node.elements.len()
                && node.elements.iter().zip(elements).all(|(pattern, ty)| irrefutable_pattern(ctx, pattern, ty)))
        }
        P::RecordPattern(node) => {
            let TypeNode::Path { path, generic_args } = &base.node else {
                return false;
            };
            if *path != node.path {
                return false;
            }
            lookup_record_decl(ctx, &node.path).is_some_and(|record| {
                node.fields.iter().all(|field| {
                    field_type(record, &field.name, ctx, generic_args)
                        .is_some_and(|ty| irrefutable_pattern(ctx, &field_pattern(field), &ty))
                })
            })
        }
        P::ModalPattern(node) => {
            let TypeNode::ModalState(modal) = &base.node else {
                return false;
            };
            if !id_eq(&modal.state, &node.state) {
                return false;
            }
            let Some(decl) = lookup_modal_decl(ctx, &modal.path).filter(|decl| has_state(decl, &node.state)) else {
                return false;
            };
            node.fields_opt.iter().flat_map(|payload| &payload.fields).all(|field| {
                modal_field_type(decl, &node.state, ctx, &field.name, &modal.generic_args)
                    .is_some_and(|ty| irrefutable_pattern(ctx, &field_pattern(field), &ty))
            })
        }
        P::TypedPattern(node) => {
            lower_type(ctx, &node.r#type).is_ok_and(|lowered| lowered.is_some() && type_equiv(&lowered, base_ref))
        }
        _ => false,
    }
}

fn binds_whole_value(pattern: &ast::Pattern) -> bool {
    matches!(pattern.node, ast::PatternNode::IdentifierPattern(_) | ast::PatternNode::WildcardPattern(_))
}

/// Whether an enum pattern matches every value of its variant.
pub fn enum_pattern_covers_variant(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> bool {
    let (Some(pattern), Some(expected)) = (pattern.as_deref(), expected.as_deref()) else {
        return false;
    };
    let ast::PatternNode::EnumPattern(node) = &pattern.node else {
        return false;
    };
    let Some((path, args)) = applied_type_path(expected).zip(applied_type_args(expected)) else {
        return false;
    };
    if !type_path_eq_local(path, &node.path) {
        return false;
    }
    let Some(decl) = lookup_enum_decl(ctx, path) else {
        // Without the declaration, a payload of plain bindings is taken to cover.
        return match &node.payload_opt {
            None => true,
            Some(ast::EnumPayloadPattern::TuplePayloadPattern(tuple)) => {
                tuple.elements.iter().all(|elem| elem.as_deref().is_some_and(binds_whole_value))
            }
            Some(ast::EnumPayloadPattern::RecordPayloadPattern(record)) => {
                record.fields.iter().all(|field| field.pattern_opt.as_deref().is_none_or(binds_whole_value))
            }
        };
    };
    let Ok(generic_ctx) = build_enum_pattern_generic_context(ctx, decl, args) else {
        return false;
    };
    let Some(variant) = find_variant(decl, &node.name) else {
        return false;
    };
    match (&variant.payload_opt, &node.payload_opt) {
        (None, payload) => payload.is_none(),
        (Some(ast::VariantPayload::VariantPayloadTuple(payload)), Some(ast::EnumPayloadPattern::TuplePayloadPattern(tuple))) => {
            tuple.elements.len() == payload.elements.len()
                && tuple.elements.iter().zip(&payload.elements).all(|(pattern, written)| {
                    lower_type(&generic_ctx.payload_ctx, written).is_ok_and(|lowered| {
                        irrefutable_pattern(ctx, pattern, &instantiate_type(&lowered, &generic_ctx.subst))
                    })
                })
        }
        (Some(ast::VariantPayload::VariantPayloadRecord(payload)), Some(ast::EnumPayloadPattern::RecordPayloadPattern(record))) => {
            record.fields.iter().all(|field| {
                enum_field_type(payload, &generic_ctx.payload_ctx, &field.name, &generic_ctx.subst)
                    .is_some_and(|ty| field.pattern_opt.is_none() || irrefutable_pattern(ctx, &field.pattern_opt, &ty))
            })
        }
        _ => false,
    }
}

/// Whether a state pattern matches every value in its state.
pub fn modal_pattern_covers_state(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> bool {
    let (Some(pattern), Some(expected)) = (pattern.as_deref(), expected.as_deref()) else {
        return false;
    };
    let ast::PatternNode::ModalPattern(node) = &pattern.node else {
        return false;
    };
    let refutable: Vec<&ast::FieldPattern> = node
        .fields_opt
        .iter()
        .flat_map(|payload| &payload.fields)
        .filter(|field| field.pattern_opt.as_deref().is_some_and(|pattern| !binds_whole_value(pattern)))
        .collect();
    if refutable.is_empty() {
        return true;
    }
    let Some((decl, modal_args)) = modal_of_pattern(ctx, expected, &node.state) else {
        return false;
    };
    refutable.into_iter().all(|field| {
        modal_field_type(decl, &node.state, ctx, &field.name, &modal_args)
            .is_some_and(|ty| irrefutable_pattern(ctx, &field.pattern_opt, &ty))
    })
}
