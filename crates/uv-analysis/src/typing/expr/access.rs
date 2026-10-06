//! Dereference, indexing, address-of, ranges and `unsafe` blocks.

use uv_core::numeric_literals::{parse_int_core, strip_int_suffix};
use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr};
use uv_source::lexer::token::TokenKind;

use super::small::is_place_expr;
use super::transmute::emit_invalid_transmute_target_warnings_in_block;
use crate::context::ScopeContext;
use crate::memory::calls::{is_in_unsafe_span, is_packed_record};
use crate::resolve::scopes::id_key_of;
use crate::typing::alias_normalize::{expand_type_alias_apply, normalize_index_base_type};
use crate::typing::callbacks::{CheckResult, ExprTypeFn, IdentTypeFn, PlaceTypeFn, PlaceTypeResult};
use crate::typing::const_len::const_len;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt::block::{check_block, type_block, type_block_info, FlowInfo, StmtTypeResult};
use crate::typing::stmt_context::{suppress_shared_access_check, StmtTypeContext};
use crate::typing::type_env::{gpu_context, TypeEnv};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, type_expr, type_identifier_expr, type_place};
use crate::typing::type_predicates::{bitcopy_type, strip_perm, strip_perm_and_refine};
use crate::typing::types::*;

type Diag = Option<&'static str>;

fn is_single_segment_exact(path: &[String], name: &str) -> bool {
    matches!(path, [only] if only == name)
}

/// The element a `GpuPtr<T, Space>` points to.
fn gpu_ptr_deref_element_type(ty: &Type) -> Option<TypeRef> {
    let (path, args) = applied_type_path(ty).zip(applied_type_args(ty))?;
    let [element, space] = args else {
        return None;
    };
    let space = space.as_deref()?;
    let is_address_space = applied_type_args(space).is_some_and(|args| args.is_empty())
        && applied_type_path(space).is_some_and(|path| ["Global", "Shared", "Private"].iter().any(|name| is_single_segment_exact(path, name)));
    (is_single_segment_exact(path, "GpuPtr") && element.is_some() && is_address_space).then(|| element.clone())
}

/// The pointer type with aliases expanded, under permissions and refinements.
fn normalize_deref_base_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Diag> {
    let mut current = ty.clone();
    for _ in 0..16 {
        let Some(TypeNode::Path { path, generic_args }) = current.as_deref().map(|ty| &ty.node) else {
            break;
        };
        let expanded = expand_type_alias_apply(ctx, path, generic_args)?;
        if expanded.is_none() {
            break;
        }
        current = strip_perm_and_refine(&expanded);
    }
    Ok(current)
}

/// On a GPU a host pointer captured from outside may not be dereferenced.
fn is_gpu_host_pointer_deref(env: &TypeEnv, value: &ExprPtr) -> bool {
    if !gpu_context(env) {
        return false;
    }
    let Some(ExprNode::IdentifierExpr(ident)) = value.as_deref().map(|value| &value.node) else {
        return false;
    };
    let key = id_key_of(&ident.name);
    match env.scopes.split_last() {
        Some((current, outer)) => !current.contains_key(&key) && outer.iter().any(|scope| scope.contains_key(&key)),
        None => false,
    }
}

/// What a dereference reaches.
enum Pointee {
    Gpu(TypeRef),
    Safe(TypeRef),
    Raw(RawPtrQual, TypeRef),
}

fn deref_pointee(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::DerefExpr,
    env: &TypeEnv,
    span: &Span,
) -> Result<Pointee, Diag> {
    let ptr = type_expr(ctx, type_ctx, &expr.value, env);
    if !ptr.ok {
        return Err(ptr.diag_id);
    }
    if is_gpu_host_pointer_deref(env, &expr.value) {
        return Err(Some("E-CON-0150"));
    }
    let stripped_base = normalize_deref_base_type(ctx, &strip_perm_and_refine(&ptr.r#type))?;
    let Some(base) = stripped_base.as_deref() else {
        return Err(None);
    };
    if let Some(element) = gpu_ptr_deref_element_type(base) {
        return if gpu_context(env) { Ok(Pointee::Gpu(element)) } else { Err(Some("E-CON-0150")) };
    }
    match &base.node {
        TypeNode::Ptr { state: Some(PtrState::Null), .. } => Err(Some("Deref-Null")),
        TypeNode::Ptr { state: Some(PtrState::Expired), .. } => Err(Some("Deref-Expired")),
        TypeNode::Ptr { element, .. } => Ok(Pointee::Safe(element.clone())),
        TypeNode::RawPtr { .. } if !is_in_unsafe_span(ctx, span) => Err(Some("Deref-Raw-Unsafe")),
        TypeNode::RawPtr { qual, element } => Ok(Pointee::Raw(*qual, element.clone())),
        _ => Err(None),
    }
}

/// `*pointer` as a value, which copies what it points to.
pub fn type_deref_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::DerefExpr,
    env: &TypeEnv,
    span: &Span,
) -> ExprTypeResult {
    match deref_pointee(ctx, type_ctx, expr, env, span) {
        Err(diag_id) => ExprTypeResult::failed(diag_id),
        Ok(Pointee::Gpu(element) | Pointee::Safe(element) | Pointee::Raw(_, element)) => {
            if !bitcopy_type(ctx, &element) {
                return ExprTypeResult::failed(Some("ValueUse-NonBitcopyPlace"));
            }
            ExprTypeResult::typed(element)
        }
    }
}

/// `*pointer` as a place; a raw pointer gives the permission its qualifier allows.
pub fn type_deref_place(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::DerefExpr,
    env: &TypeEnv,
    span: &Span,
) -> PlaceTypeResult {
    let place = |ty: TypeRef| PlaceTypeResult { ok: true, r#type: ty, ..Default::default() };
    match deref_pointee(ctx, type_ctx, expr, env, span) {
        Err(diag_id) => PlaceTypeResult { diag_id, ..Default::default() },
        Ok(Pointee::Gpu(element) | Pointee::Safe(element)) => place(element),
        Ok(Pointee::Raw(RawPtrQual::Imm, element)) => place(make_type_perm(Permission::Const, element)),
        Ok(Pointee::Raw(_, element)) => place(make_type_perm(Permission::Unique, element)),
    }
}

const NON_CONST_ARRAY_INDEX_DETAIL: &str = "fixed-size array index expression is not compile-time constant; \
runtime fixed-array indexing requires #dynamic, or use a slice for runtime indexing";

/// A failed index check: with a rule when the index itself fails to type, without one
/// when it is merely not what was asked for.
#[derive(Default)]
struct IndexFailure {
    diag_id: Diag,
    diag_detail: String,
    diag_span: Option<Span>,
}

fn expr_span(expr: &ExprPtr) -> Option<Span> {
    expr.as_deref().map(|expr| expr.span.clone())
}

fn int_literal_suffix(lexeme: &str) -> Option<&'static str> {
    ["i128", "u128", "isize", "usize", "i64", "u64", "i32", "u32", "i16", "u16", "i8", "u8"]
        .into_iter()
        .find(|suffix| suffix.len() < lexeme.len() && lexeme.ends_with(suffix))
}

fn is_usize_type(ty: &TypeRef) -> bool {
    matches!(strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize")
}

/// Whether the index is a `usize`: an integer literal that fits, or an expression of
/// that type.
fn check_index_usize_expr(expr: &ExprPtr, type_expr: ExprTypeFn<'_>) -> Result<(), IndexFailure> {
    let Some(e) = expr.as_deref() else {
        return Err(IndexFailure::default());
    };
    match &e.node {
        ExprNode::RangeExpr(_) => return Err(IndexFailure::default()),
        ExprNode::LiteralExpr(literal) if literal.literal.kind == TokenKind::IntLiteral => {
            let lexeme = &literal.literal.lexeme;
            let fits = int_literal_suffix(lexeme).is_none_or(|suffix| suffix == "usize")
                && parse_int_core(strip_int_suffix(lexeme)).is_some_and(|value| u64::try_from(value).is_ok());
            return if fits { Ok(()) } else { Err(IndexFailure::default()) };
        }
        _ => {}
    }
    let typed = type_expr(expr);
    if !typed.ok {
        return Err(IndexFailure { diag_id: typed.diag_id, diag_detail: typed.diag_detail, diag_span: typed.diag_span.or_else(|| expr_span(expr)) });
    }
    if is_usize_type(&typed.r#type) {
        Ok(())
    } else {
        Err(IndexFailure::default())
    }
}

/// Whether the index is a range of `usize`, which selects a slice.
fn check_range_index_expr(expr: &ExprPtr, type_expr: ExprTypeFn<'_>) -> Result<(), IndexFailure> {
    let Some(e) = expr.as_deref() else {
        return Err(IndexFailure::default());
    };
    if let ExprNode::RangeExpr(range) = &e.node {
        let bound = |bound: &ExprPtr| if bound.is_some() { check_index_usize_expr(bound, type_expr) } else { Err(IndexFailure::default()) };
        return match range.kind {
            ast::RangeKind::Full => Ok(()),
            ast::RangeKind::To | ast::RangeKind::ToInclusive => bound(&range.rhs),
            ast::RangeKind::From => bound(&range.lhs),
            ast::RangeKind::Exclusive | ast::RangeKind::Inclusive => bound(&range.lhs).and_then(|()| bound(&range.rhs)),
        };
    }
    let typed = type_expr(expr);
    if !typed.ok {
        return Err(IndexFailure { diag_id: typed.diag_id, diag_detail: typed.diag_detail, diag_span: typed.diag_span.or_else(|| expr_span(expr)) });
    }
    if is_range_type(&typed.r#type) && is_range_index_type(&typed.r#type) {
        Ok(())
    } else {
        Err(IndexFailure::default())
    }
}

/// The type an index selects from a base of the given type, before the permission of
/// the base is applied: an element, or a slice for a range.
fn index_selection(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::IndexAccessExpr,
    base_type: &TypeRef,
    type_expr: ExprTypeFn<'_>,
) -> Result<TypeRef, IndexFailure> {
    let at = |diag_id: &'static str, span: Option<Span>| IndexFailure { diag_id: Some(diag_id), diag_span: span, ..Default::default() };
    let stripped_base = normalize_index_base_type(ctx, &strip_perm_and_refine(base_type))
        .map_err(|diag_id| IndexFailure { diag_id, diag_span: expr_span(&expr.base), ..Default::default() })?;
    let Some(base) = stripped_base.as_deref() else {
        return Err(at("Index-NonIndexable", expr_span(&expr.base)));
    };
    if let Err(failure) = check_index_usize_expr(&expr.index, type_expr) {
        if failure.diag_id.is_some() {
            return Err(failure);
        }
        let base_is_slice = matches!(base.node, TypeNode::Slice(_));
        if let Err(range_failure) = check_range_index_expr(&expr.index, type_expr) {
            if range_failure.diag_id.is_some() {
                return Err(range_failure);
            }
            return Err(at(if base_is_slice { "Index-Slice-NonUsize" } else { "Index-Array-NonUsize" }, expr_span(&expr.index)));
        }
        return match &base.node {
            TypeNode::Array { element, .. } | TypeNode::Slice(element) => Ok(make_type_slice(element.clone())),
            _ => Err(at("Index-NonIndexable", expr_span(&expr.base))),
        };
    }
    match &base.node {
        TypeNode::Array { element, length, .. } => {
            // An array is indexed by a constant within its length, or at run time in
            // a dynamic context.
            match const_len(ctx, &expr.index) {
                Ok(index) if index >= *length => Err(at("E-UNS-0103", expr_span(&expr.index))),
                Ok(_) => Ok(element.clone()),
                Err(_) if type_ctx.contract_dynamic => Ok(element.clone()),
                Err(_) => Err(IndexFailure {
                    diag_id: Some("E-UNS-0102"),
                    diag_detail: NON_CONST_ARRAY_INDEX_DETAIL.to_string(),
                    diag_span: expr_span(&expr.index),
                }),
            }
        }
        TypeNode::Slice(element) => Ok(element.clone()),
        _ => Err(at("Index-NonIndexable", None)),
    }
}

fn with_perm_of(base_type: &TypeRef, selected: TypeRef) -> TypeRef {
    match base_type.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, .. }) => make_type_perm(*perm, selected),
        _ => selected,
    }
}

pub fn type_index_access_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::IndexAccessExpr,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> ExprTypeResult {
    if expr.base.is_none() || expr.index.is_none() {
        return ExprTypeResult::default();
    }
    let base_type = type_expr(ctx, &suppress_shared_access_check(type_ctx), &expr.base, env);
    if !base_type.ok {
        return ExprTypeResult {
            diag_id: base_type.diag_id,
            diag_detail: base_type.diag_detail,
            diag_span: base_type.diag_span.or_else(|| expr_span(&expr.base)),
            ..Default::default()
        };
    }
    match index_selection(ctx, type_ctx, expr, &base_type.r#type, type_expr_fn) {
        Err(failure) => {
            ExprTypeResult { diag_id: failure.diag_id, diag_detail: failure.diag_detail, diag_span: failure.diag_span, ..Default::default() }
        }
        Ok(selected) => {
            // Reading an element copies it.
            let out_type = with_perm_of(&base_type.r#type, selected);
            if !bitcopy_type(ctx, &out_type) {
                return ExprTypeResult::failed(Some("ValueUse-NonBitcopyPlace"));
            }
            ExprTypeResult::typed(out_type)
        }
    }
}

pub fn type_index_access_place(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::IndexAccessExpr,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> PlaceTypeResult {
    if expr.base.is_none() || expr.index.is_none() {
        return PlaceTypeResult::default();
    }
    let base_type = type_place(ctx, &suppress_shared_access_check(type_ctx), &expr.base, env);
    if !base_type.ok {
        return PlaceTypeResult {
            diag_id: base_type.diag_id,
            diag_detail: base_type.diag_detail,
            diag_span: base_type.diag_span.or_else(|| expr_span(&expr.base)),
            ..Default::default()
        };
    }
    match index_selection(ctx, type_ctx, expr, &base_type.r#type, type_expr_fn) {
        Err(failure) => PlaceTypeResult { ok: false, diag_id: failure.diag_id, r#type: None, diag_detail: failure.diag_detail, diag_span: failure.diag_span },
        Ok(selected) => PlaceTypeResult { ok: true, r#type: with_perm_of(&base_type.r#type, selected), ..Default::default() },
    }
}

/// `&place`: a valid pointer to the place.
pub fn type_address_of_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::AddressOfExpr,
    env: &TypeEnv,
) -> ExprTypeResult {
    let failed = |diag_id: Diag| ExprTypeResult::failed(diag_id);
    let Some(place_expr) = expr.place.as_deref().filter(|_| is_place_expr(&expr.place)) else {
        return failed(Some("AddrOf-NonPlace"));
    };
    let place_failure = |place: PlaceTypeResult| ExprTypeResult {
        diag_id: place.diag_id,
        diag_detail: place.diag_detail,
        diag_span: place.diag_span,
        ..Default::default()
    };
    match &place_expr.node {
        // A field of a packed record may be unaligned.
        ExprNode::FieldAccessExpr(field) => {
            let base_type = type_expr(ctx, type_ctx, &field.base, env);
            if base_type.ok {
                if let Some(TypeNode::Path { path, .. }) = strip_perm(&base_type.r#type).as_deref().map(|ty| &ty.node) {
                    if is_packed_record(ctx, path) && !is_in_unsafe_span(ctx, &place_expr.span) {
                        return failed(Some("E-TYP-2105"));
                    }
                }
            }
        }
        ExprNode::IndexAccessExpr(index) => {
            let idx_type = type_expr(ctx, type_ctx, &index.index, env);
            if !idx_type.ok {
                return failed(idx_type.diag_id);
            }
            let is_usize = matches!(strip_perm(&idx_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize");
            if !is_usize {
                let base_type = type_place(ctx, type_ctx, &index.base, env);
                if !base_type.ok {
                    return failed(base_type.diag_id);
                }
                return failed(Some(match strip_perm(&base_type.r#type).as_deref().map(|ty| &ty.node) {
                    Some(TypeNode::Array { .. }) => "Index-Array-NonUsize",
                    Some(TypeNode::Slice(_)) => "Index-Slice-NonUsize",
                    _ => "Index-NonIndexable",
                }));
            }
            if !matches!(index.index.as_deref().map(|index| &index.node), Some(ExprNode::RangeExpr(_))) {
                let base_place = type_place(ctx, type_ctx, &index.base, env);
                if !base_place.ok {
                    return place_failure(base_place);
                }
                if let Some(TypeNode::Array { element, length, .. }) = strip_perm(&base_place.r#type).as_deref().map(|ty| &ty.node) {
                    match const_len(ctx, &index.index) {
                        Ok(value) if value >= *length => {
                            return ExprTypeResult { diag_id: Some("Index-Array-OOB-Err"), diag_span: expr_span(&index.index), ..Default::default() };
                        }
                        Err(_) if !type_ctx.contract_dynamic => {
                            return ExprTypeResult {
                                diag_id: Some("Index-Array-NonConst-Err"),
                                diag_detail: NON_CONST_ARRAY_INDEX_DETAIL.to_string(),
                                diag_span: expr_span(&index.index),
                                ..Default::default()
                            };
                        }
                        _ => {}
                    }
                    let out_type = match base_place.r#type.as_deref().map(|ty| &ty.node) {
                        Some(TypeNode::Perm { perm, .. }) => make_type_perm(*perm, element.clone()),
                        _ => element.clone(),
                    };
                    return ExprTypeResult::typed(make_type_ptr(out_type, Some(PtrState::Valid)));
                }
            }
        }
        _ => {}
    }
    let place = type_place(ctx, type_ctx, &expr.place, env);
    if !place.ok {
        return place_failure(place);
    }
    ExprTypeResult::typed(make_type_ptr(place.r#type, Some(PtrState::Valid)))
}

/// A range has the type of its bounds, which must agree.
pub fn type_range_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::RangeExpr, env: &TypeEnv) -> ExprTypeResult {
    let bound = |bound: &ExprPtr| -> Result<TypeRef, Diag> {
        if bound.is_none() {
            return Ok(None);
        }
        let typed = type_expr(ctx, type_ctx, bound, env);
        if typed.ok {
            Ok(typed.r#type)
        } else {
            Err(typed.diag_id)
        }
    };
    let mut lhs_type = match bound(&expr.lhs) {
        Ok(ty) => ty,
        Err(diag_id) => return ExprTypeResult::failed(diag_id),
    };
    let mut rhs_type = match bound(&expr.rhs) {
        Ok(ty) => ty,
        Err(diag_id) => return ExprTypeResult::failed(diag_id),
    };
    if lhs_type.is_some() && rhs_type.is_some() && !type_equiv(&lhs_type, &rhs_type) {
        if check_expr_against(ctx, type_ctx, &expr.lhs, &rhs_type, env).ok {
            lhs_type = rhs_type.clone();
        } else if check_expr_against(ctx, type_ctx, &expr.rhs, &lhs_type, env).ok {
            rhs_type = lhs_type.clone();
        } else {
            return ExprTypeResult::failed(Some("E-SEM-3133"));
        }
    }
    let of = |bound: TypeRef, node: fn(TypeRef) -> TypeNode| bound.is_some().then(|| make_type(node(bound))).flatten();
    let range_type = match expr.kind {
        ast::RangeKind::Full => make_type(TypeNode::RangeFull),
        ast::RangeKind::To => of(rhs_type, TypeNode::RangeTo),
        ast::RangeKind::ToInclusive => of(rhs_type, TypeNode::RangeToInclusive),
        ast::RangeKind::From => of(lhs_type, TypeNode::RangeFrom),
        ast::RangeKind::Exclusive => of(lhs_type, TypeNode::Range),
        ast::RangeKind::Inclusive => of(lhs_type, TypeNode::RangeInclusive),
    };
    if range_type.is_none() {
        return ExprTypeResult::default();
    }
    ExprTypeResult::typed(range_type)
}

pub fn type_unsafe_block_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::UnsafeBlockExpr, env: &TypeEnv) -> ExprTypeResult {
    let Some(block) = expr.block.as_deref() else {
        return ExprTypeResult::typed(make_type_prim("()"));
    };
    let mut body_ctx = type_ctx.clone();
    body_ctx.env_ref = None;
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env);
    let ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let body_result = type_block(ctx, &body_ctx, block, env, &expr_fn, &ident_fn, &place_fn, None);
    if !body_result.ok {
        return ExprTypeResult::failed(body_result.diag_id);
    }
    emit_invalid_transmute_target_warnings_in_block(ctx, type_ctx, block);
    ExprTypeResult::typed(body_result.r#type)
}

pub fn check_unsafe_block_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::UnsafeBlockExpr,
    env: &TypeEnv,
    expected: &TypeRef,
) -> CheckResult {
    let Some(block) = expr.block.as_deref().filter(|_| expected.is_some()) else {
        return CheckResult::default();
    };
    let mut body_ctx = type_ctx.clone();
    body_ctx.env_ref = None;
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env);
    let ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let checked = check_block(ctx, &body_ctx, block, env, expected, &expr_fn, &ident_fn, &place_fn, None);
    if !checked.ok {
        return checked;
    }
    emit_invalid_transmute_target_warnings_in_block(ctx, type_ctx, block);
    CheckResult { ok: true, ..Default::default() }
}

/// An `unsafe` block as a statement: its bindings stay inside, its breaks and a
/// diverging result flow out.
pub fn type_unsafe_block_stmt(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::UnsafeBlockStmt,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
    type_ident_fn: IdentTypeFn<'_>,
    type_place_fn: PlaceTypeFn<'_>,
) -> StmtTypeResult {
    let Some(body) = node.body.as_deref() else {
        return StmtTypeResult::default();
    };
    let mut unsafe_ctx = type_ctx.clone();
    unsafe_ctx.in_unsafe = true;
    let env_ref = unsafe_ctx.env_ref.clone();
    let info = type_block_info(ctx, &unsafe_ctx, body, env, type_expr_fn, type_ident_fn, type_place_fn, env_ref.as_ref());
    if !info.ok {
        return StmtTypeResult { diag_id: info.diag_id, diag_detail: info.diag_detail, diag_span: info.diag_span, ..Default::default() };
    }
    let mut flow = FlowInfo { breaks: info.breaks, break_void: info.break_void, ..Default::default() };
    if matches!(info.r#type.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!") {
        flow.results.push(info.r#type);
    }
    StmtTypeResult { ok: true, env: env.clone(), flow, ..Default::default() }
}

/// `value?`: an outcome's error, or the members of a union that the enclosing
/// procedure can return, leave through it; the rest is the value.
pub fn type_propagate_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::PropagateExpr, env: &TypeEnv) -> ExprTypeResult {
    let none = ExprTypeResult::default;
    let inner = type_expr(ctx, type_ctx, &expr.value, env);
    if !inner.ok {
        return ExprTypeResult::failed(inner.diag_id);
    }
    let mut source_type = strip_perm(&inner.r#type);
    if source_type.is_none() {
        return none();
    }
    for _ in 0..16 {
        let Some(t) = source_type.as_deref() else {
            break;
        };
        let (Some(path), Some(args)) = (applied_type_path(t), applied_type_args(t)) else {
            break;
        };
        match expand_type_alias_apply(ctx, path, args) {
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
            Ok(None) => break,
            Ok(expanded) => source_type = expanded,
        }
    }
    if source_type.is_none() || type_ctx.return_type.is_none() {
        return none();
    }
    // In an asynchronous procedure the error leaves through the computation's error.
    let mut propagate_target = type_ctx.return_type.clone();
    let mut async_try = false;
    if let Some(async_sig) = crate::typing::type_lookup::async_sig_of(ctx, &type_ctx.return_type) {
        if matches!(async_sig.err.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!") {
            return ExprTypeResult::failed(Some("E-CON-0230"));
        }
        async_try = true;
        propagate_target = async_sig.err;
    }
    if propagate_target.is_none() {
        return none();
    }
    if let Some(outcome_sig) = crate::typing::outcome::outcome_sig_of(&source_type) {
        let error_target = if async_try {
            propagate_target
        } else {
            crate::typing::outcome::outcome_sig_of(&type_ctx.return_type).map(|sig| sig.error).unwrap_or_default()
        };
        if error_target.is_none() {
            return none();
        }
        let sub = crate::typing::subtyping::subtyping(ctx, &outcome_sig.error, &error_target);
        if !sub.ok {
            return ExprTypeResult::failed(sub.diag_id);
        }
        return if sub.subtype { ExprTypeResult::typed(outcome_sig.value) } else { none() };
    }
    let Some(TypeNode::Union(members)) = source_type.as_deref().map(|ty| &ty.node) else {
        return none();
    };
    // Exactly one member must stay behind as the value.
    let mut success: Option<TypeRef> = None;
    for member in members {
        let sub = crate::typing::subtyping::subtyping(ctx, member, &propagate_target);
        if !sub.ok {
            return ExprTypeResult::failed(sub.diag_id);
        }
        if sub.subtype {
            continue;
        }
        if success.is_some() {
            return none();
        }
        success = Some(member.clone());
    }
    match success {
        Some(value) => ExprTypeResult::typed(value),
        None => none(),
    }
}
