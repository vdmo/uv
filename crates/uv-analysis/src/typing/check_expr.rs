//! Inferring the type of an expression and checking it against an expected type.
//!
//! Checking tries, in order: the forms that take their type from what is expected
//! (closures, `null`, literals, array literals, negated literals), then the inferred
//! type against the expectation by union membership, array-to-slice coercion,
//! equivalence and subtyping, and last a refinement of the expected type by proof.

use uv_source::ast::{self, ArraySegment, ExprNode, ExprPtr};
use uv_source::lexer::token::TokenKind;

use super::alias_normalize::normalize_alias_type;
use super::attributed::has_memory_order_attribute;
use uv_source::attributes::{validate_attributes, AttributeTarget};
use super::callbacks::{CheckResult, ExprTypeFn, IdentTypeFn, PlaceTypeFn};
use super::const_len::const_len;
use super::expr_result::ExprTypeResult;
use super::literals::{check_literal_expr, type_literal_expr};
use super::subtyping::subtyping;
use super::type_equiv::type_equiv;
use super::type_lookup::async_sig_of;
use super::type_predicates::{bitcopy_type, perm_of_type, strip_perm};
use super::types::*;
use crate::composite::arrays_slices::coerce_array_to_slice;
use crate::context::ScopeContext;
use crate::contracts::verification::{static_proof_at, StaticProofContext};
use crate::typing::stmt::postcondition::substitute_refinement_self;
use crate::caps::builtin_paths::path_matches_builtin_name;
use crate::memory::calls::{has_source_provenance, is_in_unsafe_span, is_place_expr_for_call, type_call_with_subst};
use crate::typing::expr::method_call::addr_of_ok;
use crate::typing::expr::call::infer_generic_call_subst;
use crate::modal::lookup::{has_state, lookup_modal_decl};
use crate::modal::modal_widen::niche_compatible;
use crate::resolve::scopes::{id_eq, id_key_of};

/// Checks an `if … is { … }` expression against an expected type.
pub type IfCaseCheckFn<'f> = &'f dyn Fn(&ast::IfCaseExpr, &TypeRef) -> CheckResult;

fn ok() -> CheckResult {
    CheckResult { ok: true, ..Default::default() }
}

fn no(diag_id: Option<&'static str>) -> CheckResult {
    CheckResult { diag_id, ..Default::default() }
}

/// The primitive type under permissions and refinements.
pub fn get_prim_name(ty: &TypeRef) -> Option<&str> {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    match &current.as_deref()?.node {
        TypeNode::Prim(name) => Some(name),
        _ => None,
    }
}

pub fn is_int_type(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize")
}

/// `Ptr::null()` may stand where a pointer of unknown or null state is expected.
fn ptr_null_expected(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => ptr_null_expected(base),
        Some(TypeNode::Ptr { state, .. }) => matches!(state, None | Some(PtrState::Null)),
        _ => false,
    }
}

/// `GpuPtr<T, A>` against `GpuPtr<T, B>` with different address spaces.
fn is_gpu_ptr_addr_space_mismatch(actual: &TypeRef, expected: &TypeRef) -> bool {
    let view = |ty: &TypeRef| {
        let stripped = strip_perm(ty);
        let stripped = stripped.as_deref()?;
        match (&applied_type_path(stripped)?[..], applied_type_args(stripped)?) {
            ([name], [pointee, space]) if name == "GpuPtr" => Some((pointee.clone(), space.clone())),
            _ => None,
        }
    };
    match (view(actual), view(expected)) {
        (Some((ap, aspace)), Some((ep, espace))) => type_equiv(&ap, &ep) && !type_equiv(&aspace, &espace),
        _ => false,
    }
}

/// A modal state against its modal, where the state does not share the modal's
/// representation: widening is then a conversion that must be written.
fn modal_non_niche(ctx: &ScopeContext<'_>, source: &TypeRef, target: &TypeRef) -> bool {
    let (source, target) = (strip_perm(source), strip_perm(target));
    let (Some(source), Some(target)) = (source.as_deref(), target.as_deref()) else {
        return false;
    };
    let (TypeNode::ModalState(modal), Some((path, args))) =
        (&source.node, applied_type_path(target).zip(applied_type_args(target)))
    else {
        return false;
    };
    let same_path =
        modal.path.len() == path.len() && modal.path.iter().zip(path).all(|(a, b)| id_key_of(a) == id_key_of(b));
    if !same_path || modal.generic_args.len() != args.len() {
        return false;
    }
    lookup_modal_decl(ctx, &modal.path).is_some_and(|decl| has_state(decl, &modal.state))
        && !niche_compatible(ctx, &modal.path, &modal.state)
}

/// The type of an expression with nothing expected of it.
pub fn infer_expr(expr: &ExprPtr, type_expr: ExprTypeFn<'_>, type_ident: IdentTypeFn<'_>) -> ExprTypeResult {
    let Some(e) = expr.as_deref() else {
        return ExprTypeResult::default();
    };
    match &e.node {
        ExprNode::PtrNullExpr(_) => ExprTypeResult::failed(Some("PtrNull-Infer-Err")),
        ExprNode::LiteralExpr(literal) if literal.literal.kind == TokenKind::NullLiteral => {
            ExprTypeResult::failed(Some("NullLiteral-Infer-Err"))
        }
        ExprNode::LiteralExpr(literal) => {
            let typed = type_literal_expr(literal);
            if typed.ok {
                ExprTypeResult::typed(typed.r#type)
            } else {
                ExprTypeResult::failed(typed.diag_id)
            }
        }
        ExprNode::IdentifierExpr(ident) => {
            let typed = type_ident(&ident.name);
            if typed.ok {
                ExprTypeResult::typed(typed.r#type)
            } else {
                ExprTypeResult { diag_id: typed.diag_id, diag_detail: typed.diag_detail, ..Default::default() }
            }
        }
        ExprNode::TupleExpr(tuple) if tuple.elements.is_empty() => ExprTypeResult::typed(make_type_prim("()")),
        ExprNode::TupleExpr(tuple) => {
            let mut elements = Vec::with_capacity(tuple.elements.len());
            for elem in &tuple.elements {
                let typed = infer_expr(elem, type_expr, type_ident);
                if !typed.ok {
                    return ExprTypeResult::failed(typed.diag_id);
                }
                elements.push(typed.r#type);
            }
            ExprTypeResult::typed(make_type_tuple(elements))
        }
        ExprNode::CallExpr(_) | ExprNode::CallTypeArgsExpr(_) => {
            let typed = type_expr(expr);
            if typed.ok {
                ExprTypeResult::typed(typed.r#type)
            } else {
                // A failed call does not carry its span along.
                ExprTypeResult { diag_span: None, r#type: None, ..typed }
            }
        }
        _ => {
            let typed = type_expr(expr);
            if typed.ok {
                ExprTypeResult::typed(typed.r#type)
            } else {
                ExprTypeResult { r#type: None, ..typed }
            }
        }
    }
}

struct Checker<'c, 'a, 'f> {
    ctx: &'c ScopeContext<'a>,
    type_expr: ExprTypeFn<'f>,
    type_place: Option<PlaceTypeFn<'f>>,
    type_ident: IdentTypeFn<'f>,
    if_case_check: Option<IfCaseCheckFn<'f>>,
    /// The facts in scope, from which refinement predicates are proved.
    proof_ctx: Option<&'f StaticProofContext>,
}

impl Checker<'_, '_, '_> {
    /// A repeat count: an integer expression with a constant value.
    fn repeat_len(&self, count: &ExprPtr) -> Result<u64, Option<&'static str>> {
        let count_type = (self.type_expr)(count);
        if !count_type.ok {
            return Err(count_type.diag_id);
        }
        if !get_prim_name(&count_type.r#type).is_some_and(is_int_type) {
            return Err(Some("E-TYP-1812"));
        }
        const_len(self.ctx, count).map_err(|diag_id| Some(diag_id.unwrap_or("E-TYP-1812")))
    }

    /// The segments of an array literal against an element type; the total length.
    fn array_segments(&self, array: &ast::ArrayExpr, element: &TypeRef) -> Result<u64, Option<&'static str>> {
        let mut total_length: u64 = 0;
        for segment in &array.elements {
            match segment {
                ArraySegment::ArrayElemSegment(elem) => {
                    let checked = self.check(&elem.value, element);
                    if !checked.ok {
                        return Err(checked.diag_id);
                    }
                    total_length = total_length.wrapping_add(1);
                }
                ArraySegment::ArrayRepeatSegment(repeat) => {
                    if repeat.value.is_none() || repeat.count.is_none() {
                        return Err(None);
                    }
                    let len = self.repeat_len(&repeat.count)?;
                    let checked = self.check(&repeat.value, element);
                    if !checked.ok {
                        return Err(checked.diag_id);
                    }
                    if !bitcopy_type(self.ctx, element) {
                        return Err(Some("E-UNS-0107"));
                    }
                    total_length = total_length.wrapping_add(len);
                }
            }
        }
        Ok(total_length)
    }

    /// `heap~>alloc_raw(count)` where a mutable raw pointer is expected: the element type
    /// comes from the expectation, so only the receiver and the count are checked.
    /// Nothing when the receiver is not a heap allocator.
    fn check_heap_alloc_raw(&self, e: &ast::Expr, method: &ast::MethodCallExpr) -> Option<CheckResult> {
        let ctx = self.ctx;
        let mut recv_type = (self.type_expr)(&method.receiver);
        if !recv_type.ok && recv_type.diag_id == Some("ValueUse-NonBitcopyPlace") {
            let span = method.receiver.as_deref().map(|receiver| receiver.span.clone()).unwrap_or_default();
            let move_expr = ast::Expr { span, node: ExprNode::MoveExpr(ast::MoveExpr { place: method.receiver.clone() }) };
            recv_type = (self.type_expr)(&Some(std::sync::Arc::new(move_expr)));
        }
        if !recv_type.ok {
            return Some(no(recv_type.diag_id));
        }
        let recv_strip = strip_perm(&recv_type.r#type);
        let Some(TypeNode::Dynamic(path)) = recv_strip.as_deref().map(|ty| &ty.node) else {
            return None;
        };
        if !path_matches_builtin_name(path, "HeapAllocator") {
            return None;
        }
        if !is_in_unsafe_span(ctx, &e.span) {
            return Some(no(Some("E-MEM-3030")));
        }
        let required = make_type_perm(Permission::Const, make_type_dynamic(vec!["HeapAllocator".to_string()]));
        let recv_sub = subtyping(ctx, &recv_type.r#type, &required);
        if !recv_sub.ok {
            return Some(no(recv_sub.diag_id));
        }
        if !recv_sub.subtype {
            return Some(no(Some("E-TYP-1605")));
        }
        if has_source_provenance(&method.receiver) {
            if !is_place_expr_for_call(&method.receiver) {
                return Some(no(Some("E-TYP-1603")));
            }
            if let Err(diag_id) = addr_of_ok(&method.receiver, self.type_expr, None) {
                return Some(no(diag_id));
            }
        }
        let [arg] = method.args.as_slice() else {
            return Some(no(Some("E-SEM-2532")));
        };
        if arg.pass == ast::ArgPassKind::Move {
            return Some(no(Some("E-SEM-2535")));
        }
        if has_source_provenance(&arg.value) && !is_place_expr_for_call(&arg.value) {
            return Some(no(Some("E-TYP-1603")));
        }
        let arg_type = (self.type_expr)(&arg.value);
        if !arg_type.ok {
            return Some(no(arg_type.diag_id));
        }
        let count_sub = subtyping(ctx, &arg_type.r#type, &make_type_prim("usize"));
        if !count_sub.ok {
            return Some(no(count_sub.diag_id));
        }
        if !count_sub.subtype {
            return Some(no(Some("E-SEM-2533")));
        }
        Some(ok())
    }

    fn check(&self, expr: &ExprPtr, expected: &TypeRef) -> CheckResult {
        let ctx = self.ctx;
        let (Some(e), Some(expected_ty)) = (expr.as_deref(), expected.as_deref()) else {
            return CheckResult::default();
        };
        if let (Some(if_case_check), ExprNode::IfCaseExpr(if_case)) = (self.if_case_check, &e.node) {
            return if_case_check(if_case, expected);
        }
        let expected_strip = strip_perm(expected);
        let expected_strip_node = expected_strip.as_deref().map(|ty| &ty.node);
        match &e.node {
            ExprNode::AttributedExpr(attributed) => {
                let no = |diag_id| CheckResult { diag_id, ..Default::default() };
                let validation = validate_attributes(&attributed.attrs, AttributeTarget::Expression);
                if !validation.ok {
                    return no(validation.diag_id);
                }
                if has_memory_order_attribute(&attributed.attrs) {
                    let observed_attr = (self.type_expr)(expr);
                    if !observed_attr.ok {
                        return no(observed_attr.diag_id);
                    }
                }
                let observed = (self.type_expr)(expr);
                if observed.ok {
                    let sub = subtyping(ctx, &observed.r#type, expected);
                    if !sub.ok {
                        return no(sub.diag_id);
                    }
                    if sub.subtype {
                        return CheckResult { ok: true, ..Default::default() };
                    }
                }
                let checked_inner = self.check(&attributed.expr, expected);
                if !checked_inner.ok {
                    return no(checked_inner.diag_id);
                }
                return CheckResult { ok: true, ..Default::default() };
            }
            ExprNode::ClosureExpr(closure) => {
                let normalized = match normalize_alias_type(ctx, expected) {
                    Ok(normalized) => normalized,
                    Err(diag_id) => return no(diag_id),
                };
                let target = strip_perm(if normalized.is_some() { &normalized } else { expected });
                let arity = match target.as_deref().map(|ty| &ty.node) {
                    Some(TypeNode::Func { params, .. }) => Some(params.len()),
                    Some(TypeNode::Closure { params, .. }) => Some(params.len()),
                    _ => None,
                };
                if let Some(arity) = arity {
                    return if closure.params.len() == arity { ok() } else { no(Some("Infer-Closure-Params-Err")) };
                }
            }
            ExprNode::PtrNullExpr(_) => {
                return if ptr_null_expected(expected) { ok() } else { no(Some("PtrNull-Infer-Err")) };
            }
            ExprNode::LiteralExpr(literal) => match check_literal_expr(literal, expected) {
                Ok(()) => return ok(),
                Err(diag_id @ Some(_)) => return no(diag_id),
                Err(None) => {}
            },
            ExprNode::ArrayExpr(array) => match expected_strip_node {
                Some(TypeNode::Array { element, length, .. }) => {
                    return match self.array_segments(array, element) {
                        Ok(total) if total == *length => ok(),
                        Ok(_) => no(None),
                        Err(diag_id) => no(diag_id),
                    };
                }
                Some(TypeNode::Slice(element)) => {
                    return match self.array_segments(array, element) {
                        Ok(_) => ok(),
                        Err(diag_id) => no(diag_id),
                    };
                }
                _ => {}
            },
            ExprNode::ArrayRepeatExpr(repeat) => {
                let len = match self.repeat_len(&repeat.count) {
                    Ok(len) => len,
                    Err(diag_id) => return no(diag_id),
                };
                let element = match expected_strip_node {
                    Some(TypeNode::Array { element, length, .. }) => {
                        if len != *length {
                            return no(None);
                        }
                        Some(element)
                    }
                    Some(TypeNode::Slice(element)) => Some(element),
                    _ => None,
                };
                if let Some(element) = element {
                    let checked = self.check(&repeat.value, element);
                    if !checked.ok {
                        return no(checked.diag_id);
                    }
                    return if bitcopy_type(ctx, element) { ok() } else { no(Some("E-UNS-0107")) };
                }
            }
            // A negated literal takes a signed or floating expected type as the literal would.
            ExprNode::UnaryExpr(unary) if id_eq(&unary.op, "-") => {
                let signed = matches!(expected_strip_node, Some(TypeNode::Prim(name))
                    if matches!(name.as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "f16" | "f32" | "f64"));
                if signed {
                    let inner = self.check(&unary.value, expected);
                    if inner.ok {
                        return ok();
                    }
                    if inner.diag_id.is_some() {
                        return no(inner.diag_id);
                    }
                }
            }
            ExprNode::MethodCallExpr(method) if id_eq(&method.name, "alloc_raw") => {
                if matches!(expected_strip_node, Some(TypeNode::RawPtr { qual: RawPtrQual::Mut, .. })) {
                    if let Some(result) = self.check_heap_alloc_raw(e, method) {
                        return result;
                    }
                }
            }
            // A call whose type arguments may follow from the expected type.
            _ => {}
        }
        // A call whose type arguments may follow from the expected type.
        let mut inferred_call_with_expected: Option<ExprTypeResult> = None;
        if let ExprNode::CallExpr(call) = &e.node {
            if call.generic_args.is_empty() {
                let inferred_subst =
                    infer_generic_call_subst(ctx, &call.callee, &call.args, expected, self.type_expr, self.type_place);
                if inferred_subst.ok {
                    let typed_call = type_call_with_subst(
                        ctx,
                        &call.callee,
                        &call.args,
                        &inferred_subst.subst,
                        self.type_expr,
                        self.type_place,
                        None,
                        None,
                    );
                    if !typed_call.ok {
                        return CheckResult { diag_id: typed_call.diag_id, ..Default::default() };
                    }
                    inferred_call_with_expected =
                        Some(ExprTypeResult { diag_detail: typed_call.diag_detail, ..ExprTypeResult::typed(typed_call.r#type) });
                }
            }
        }
        let inferred =
            inferred_call_with_expected.unwrap_or_else(|| infer_expr(expr, self.type_expr, self.type_ident));
        if !inferred.ok {
            return CheckResult {
                ok: false,
                diag_id: inferred.diag_id,
                diag_detail: inferred.diag_detail,
                diag_span: inferred.diag_span,
                diagnostic_obligation_ids: inferred.diagnostic_obligation_ids,
            };
        }
        let inferred_type = &inferred.r#type;
        // Membership in an expected union, by subtyping against each member.
        let expected_for_union = match normalize_alias_type(ctx, expected) {
            Ok(normalized @ Some(_)) => normalized,
            _ => expected.clone(),
        };
        if let Some(TypeNode::Union(members)) = strip_perm(&expected_for_union).as_deref().map(|ty| &ty.node) {
            for member in members {
                let member_sub = subtyping(ctx, inferred_type, member);
                if !member_sub.ok {
                    return no(member_sub.diag_id);
                }
                if member_sub.subtype {
                    return ok();
                }
            }
        }
        if modal_non_niche(ctx, inferred_type, expected) {
            return no(Some("Chk-Subsumption-Modal-NonNiche"));
        }
        // An array where a slice is expected; a bare array coerces as a `const` one.
        let try_coerce = |source: &TypeRef| -> Option<CheckResult> {
            let coerced = coerce_array_to_slice(ctx, source).ok()?;
            let sub = subtyping(ctx, &coerced, expected);
            if !sub.ok {
                return Some(no(sub.diag_id));
            }
            if sub.subtype {
                return Some(ok());
            }
            sub.diag_id.map(|diag_id| no(Some(diag_id)))
        };
        if let Some(decided) = try_coerce(inferred_type) {
            return decided;
        }
        if matches!(inferred_type.as_deref().map(|ty| &ty.node), Some(TypeNode::Array { .. })) {
            if let Some(decided) = try_coerce(&make_type_perm(Permission::Const, inferred_type.clone())) {
                return decided;
            }
        }
        // A freshly built value may take any permission that is expected of it.
        if let TypeNode::Perm { base: expected_base, .. } = &expected_ty.node {
            let is_aggregate_literal = matches!(
                e.node,
                ExprNode::RecordExpr(_)
                    | ExprNode::EnumLiteralExpr(_)
                    | ExprNode::TupleExpr(_)
                    | ExprNode::ArrayExpr(_)
                    | ExprNode::ArrayRepeatExpr(_)
            );
            let is_async_create = matches!(
                e.node,
                ExprNode::CallExpr(_) | ExprNode::CallTypeArgsExpr(_) | ExprNode::MethodCallExpr(_) | ExprNode::RaceExpr(_)
            ) && async_sig_of(ctx, inferred_type).is_some()
                && async_sig_of(ctx, expected_base).is_some();
            let mut target = expected_base.clone();
            if is_async_create {
                if let Some(TypeNode::ModalState(modal)) = strip_perm(expected_base).as_deref().map(|ty| &ty.node) {
                    target = if modal.modal_ref.args().is_empty() {
                        make_type_path(modal.modal_ref.path().to_vec())
                    } else {
                        make_type_apply(modal.modal_ref.path().to_vec(), modal.modal_ref.args().to_vec())
                    };
                }
            }
            if is_aggregate_literal || is_async_create {
                let base_sub = subtyping(ctx, inferred_type, &target);
                if !base_sub.ok {
                    return no(base_sub.diag_id);
                }
                if base_sub.subtype {
                    return ok();
                }
                if base_sub.diag_id.is_some() {
                    return no(base_sub.diag_id);
                }
            }
        }
        if type_equiv(inferred_type, expected) {
            return ok();
        }
        let sub = subtyping(ctx, inferred_type, expected);
        if !sub.ok {
            return no(sub.diag_id);
        }
        if sub.subtype {
            return ok();
        }
        // A moved unique value is no longer bound by its permission.
        if matches!(e.node, ExprNode::MoveExpr(_)) && perm_of_type(inferred_type) == Permission::Unique {
            let moved_value_sub = subtyping(ctx, &strip_perm(inferred_type), expected);
            if !moved_value_sub.ok {
                return no(moved_value_sub.diag_id);
            }
            if moved_value_sub.subtype {
                return ok();
            }
        }
        if sub.diag_id.is_some() {
            return no(sub.diag_id);
        }
        if is_gpu_ptr_addr_space_mismatch(inferred_type, expected) {
            return no(Some("E-TYP-2641"));
        }
        let expected_norm = match normalize_alias_type(ctx, expected) {
            Ok(normalized) => normalized,
            Err(diag_id) => return no(diag_id),
        };
        if let Some(TypeNode::Refine { base, .. }) = expected_norm.as_deref().map(|ty| &ty.node) {
            if matches!(inferred_type.as_deref().map(|ty| &ty.node), Some(TypeNode::Refine { .. })) {
                let refine_sub = subtyping(ctx, inferred_type, &expected_norm);
                if !refine_sub.ok {
                    return no(refine_sub.diag_id);
                }
                if refine_sub.subtype {
                    return ok();
                }
            }
            let base_check = self.check(expr, base);
            if !base_check.ok {
                return no(base_check.diag_id);
            }
            // The predicate, with the value for `self`, is proved from the facts in scope.
            let Some(TypeNode::Refine { predicate, .. }) = expected_norm.as_deref().map(|ty| &ty.node) else {
                return no(None);
            };
            if predicate.is_none() {
                return no(None);
            }
            let substituted = substitute_refinement_self(predicate, expr);
            let empty_proof_ctx = StaticProofContext::default();
            let location = expr.as_deref().or(substituted.as_deref()).map(|expr| expr.span.clone()).unwrap_or_default();
            if !static_proof_at(self.proof_ctx.unwrap_or(&empty_proof_ctx), &location, &substituted).provable {
                return no(Some("E-TYP-1953"));
            }
            return ok();
        }
        CheckResult {
            ok: false,
            diag_id: Some("E-SEM-2526"),
            diag_detail: format!("expected {}, found {}", type_to_string(expected), type_to_string(inferred_type)),
            ..Default::default()
        }
    }
}

/// Whether the expression has the expected type.
#[allow(clippy::too_many_arguments)]
pub fn check_expr(
    ctx: &ScopeContext<'_>,
    expr: &ExprPtr,
    expected: &TypeRef,
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    type_ident: IdentTypeFn<'_>,
    if_case_check: Option<IfCaseCheckFn<'_>>,
    proof_ctx: Option<&StaticProofContext>,
) -> CheckResult {
    Checker { ctx, type_expr, type_place, type_ident, if_case_check, proof_ctx }.check(expr, expected)
}
