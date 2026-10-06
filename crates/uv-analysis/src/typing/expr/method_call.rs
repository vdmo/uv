//! Method calls `receiver~>name(args)`: the receiver's type decides where the method is
//! looked up (a modal state's members, a class's methods for a dynamic or opaque type,
//! a record's own methods and those of the classes it implements), the receiver's
//! permission must admit the method's, and the arguments are checked like a call's.
//!
//! The methods the language builds in for capabilities, strings and bytes, the built-in
//! modals and asynchronous computations are not ported yet; a call that could name one
//! is left pending.

use std::collections::BTreeMap;

use uv_core::span::Span;
use uv_source::ast::{self, Arg, ArgPassKind, ExprNode, ExprPtr};

use super::call::emit_deprecated_reference_warning_from_attrs;
use super::small::is_place_expr;
use crate::caps::builtin_paths::{is_capability_class_path, path_matches_builtin_name};
use crate::caps::cap_methods::{lookup_capability_class_method_sig, lookup_capability_type_method_sig, CapMethodSig};
use crate::composite::classes::{lookup_class_method, type_implements_class, vtable_eligible};
use crate::composite::record_methods::{lookup_method_static, recv_type_for_receiver};
use crate::context::{ScopeContext, TypeDecl};
use crate::generics::generic_params::bind_type_params as bind_type_params_in_scope;
use crate::generics::monomorphize::{build_substitution, instantiate_type, TypeSubst};
use crate::keys::key_paths::{build_key_path, is_prefix};
use crate::memory::string_bytes::lookup_string_bytes_builtin_method_sig;
use crate::memory::calls::{
    arg_pass_expr, has_source_provenance, is_in_unsafe_span, is_place_expr_for_call, missing_required_move_for_consuming,
    uses_call_temp_for_consuming, ArgCheckFn,
};
use crate::modal::builtin_modal_intrinsics::is_builtin_modal_general_member;
use crate::modal::lookup::state_member_visible;
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::resolve::scopes_lookup::resolve_type_name;
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::callbacks::{CheckResult, ExprTypeFn, PlaceTypeFn};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::subtyping::{argument_type_compatible, permission_admits, subtyping};
use crate::typing::type_env::TypeEnv;
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, type_expr, type_place};
use crate::typing::type_lookup::async_sig_of;
use crate::typing::type_lower::{lower_param_mode, lower_type};
use crate::typing::type_predicates::{lookup_foundational_builtin_method_sig, perm_of_type, strip_perm, strip_perm_and_refine};
use crate::typing::types::*;

type Diag = Option<&'static str>;
type LowerTypeFn<'f> = &'f dyn Fn(&ast::TypePtr) -> Result<TypeRef, Diag>;

/// The type with aliases at its top expanded; nothing when it is no alias.
fn normalize_alias_top_level_for_method_call(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<Option<TypeRef>, Diag> {
    let mut current = ty.clone();
    let mut expanded_any = false;
    for _ in 0..16 {
        let Some(t) = current.as_deref() else {
            break;
        };
        let (Some(path), Some(args)) = (applied_type_path(t), applied_type_args(t)) else {
            break;
        };
        let expanded = expand_type_alias_apply(ctx, path, args)?;
        if expanded.is_none() {
            break;
        }
        current = expanded;
        expanded_any = true;
    }
    Ok(expanded_any.then_some(current).filter(|ty| ty.is_some()))
}

fn type_param_of<'t>(params: &[ast::TypeParam], ty: &'t Type) -> Option<&'t String> {
    match &ty.node {
        TypeNode::Path { path, .. } if path.len() == 1 && params.iter().any(|param| param.name == path[0]) => Some(&path[0]),
        _ => None,
    }
}

/// Matches a parameter type against an argument type, binding the type parameters the
/// parameter type names. A parameter met twice must be bound to equivalent types.
fn bind_type_params(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    expected: &TypeRef,
    actual: &TypeRef,
    bindings: &mut BTreeMap<String, TypeRef>,
) -> bool {
    let (Some(e), Some(a)) = (expected.as_deref(), actual.as_deref()) else {
        return false;
    };
    if let Some(name) = type_param_of(params, e) {
        return match bindings.get(name) {
            None => {
                bindings.insert(name.clone(), actual.clone());
                true
            }
            Some(bound) => type_equiv(bound, actual),
        };
    }
    // Either side may be an alias for what the other is written as.
    match normalize_alias_top_level_for_method_call(ctx, expected) {
        Err(_) => return false,
        Ok(Some(expanded)) => return bind_type_params(ctx, params, &expanded, actual, bindings),
        Ok(None) => {}
    }
    match normalize_alias_top_level_for_method_call(ctx, actual) {
        Err(_) => return false,
        Ok(Some(expanded)) => return bind_type_params(ctx, params, expected, &expanded, bindings),
        Ok(None) => {}
    }
    match (&e.node, &a.node) {
        (TypeNode::Perm { perm, base }, TypeNode::Perm { perm: other_perm, base: other }) => perm == other_perm && bind_type_params(ctx, params, base, other, bindings),
        (TypeNode::Tuple(elements), TypeNode::Tuple(other)) => bind_all(ctx, params, elements, other, bindings),
        (TypeNode::Array { element, length, .. }, TypeNode::Array { element: other, length: other_length, .. }) => {
            length == other_length && bind_type_params(ctx, params, element, other, bindings)
        }
        (TypeNode::Slice(element), TypeNode::Slice(other)) => bind_type_params(ctx, params, element, other, bindings),
        (TypeNode::Ptr { element, state }, TypeNode::Ptr { element: other, state: other_state }) => {
            state == other_state && bind_type_params(ctx, params, element, other, bindings)
        }
        (TypeNode::RawPtr { qual, element }, TypeNode::RawPtr { qual: other_qual, element: other }) => {
            qual == other_qual && bind_type_params(ctx, params, element, other, bindings)
        }
        (TypeNode::Union(members), TypeNode::Union(other)) => bind_all(ctx, params, members, other, bindings),
        (TypeNode::Func { params: func_params, ret }, TypeNode::Func { params: other_params, ret: other_ret }) => {
            func_params.len() == other_params.len()
                && func_params.iter().zip(other_params).all(|(param, other)| {
                    param.mode == other.mode && bind_type_params(ctx, params, &param.r#type, &other.r#type, bindings)
                })
                && bind_type_params(ctx, params, ret, other_ret, bindings)
        }
        (TypeNode::Path { path, generic_args: args }, _) | (TypeNode::Apply { path, args }, _) => {
            match (applied_type_path(a), applied_type_args(a)) {
                (Some(other_path), Some(other_args)) => path == other_path && bind_all(ctx, params, args, other_args, bindings),
                _ => false,
            }
        }
        (TypeNode::ModalState(modal), TypeNode::ModalState(other)) => {
            modal.path == other.path
                && modal.state == other.state
                && bind_all(ctx, params, &modal.generic_args, &other.generic_args, bindings)
        }
        (TypeNode::Dynamic(path), TypeNode::Dynamic(other)) => path == other,
        (TypeNode::Refine { base, .. }, TypeNode::Refine { base: other, .. }) => bind_type_params(ctx, params, base, other, bindings),
        (TypeNode::Closure { params: closure_params, ret, .. }, TypeNode::Closure { params: other_params, ret: other_ret, .. }) => {
            closure_params.len() == other_params.len()
                && closure_params
                    .iter()
                    .zip(other_params)
                    .all(|(param, other)| param.0 == other.0 && bind_type_params(ctx, params, &param.1, &other.1, bindings))
                && bind_type_params(ctx, params, ret, other_ret, bindings)
        }
        (TypeNode::Opaque { class_path, .. }, TypeNode::Opaque { class_path: other, .. }) => class_path == other,
        (TypeNode::String(state), TypeNode::String(other)) => state == other,
        (TypeNode::Bytes(state), TypeNode::Bytes(other)) => state == other,
        (TypeNode::Prim(name), TypeNode::Prim(other)) => name == other,
        (TypeNode::Range(base), TypeNode::Range(other))
        | (TypeNode::RangeInclusive(base), TypeNode::RangeInclusive(other))
        | (TypeNode::RangeFrom(base), TypeNode::RangeFrom(other))
        | (TypeNode::RangeTo(base), TypeNode::RangeTo(other))
        | (TypeNode::RangeToInclusive(base), TypeNode::RangeToInclusive(other)) => bind_type_params(ctx, params, base, other, bindings),
        (TypeNode::RangeFull, TypeNode::RangeFull) => true,
        (TypeNode::Var(_), _) => type_equiv(expected, actual),
        _ => false,
    }
}

fn bind_all(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    expected: &[TypeRef],
    actual: &[TypeRef],
    bindings: &mut BTreeMap<String, TypeRef>,
) -> bool {
    expected.len() == actual.len() && expected.iter().zip(actual).all(|(e, a)| bind_type_params(ctx, params, e, a, bindings))
}

/// A function or closure type as its parameters and return type.
fn callable_sig_of(ty: &TypeRef) -> Option<(Vec<TypeFuncParam>, TypeRef)> {
    match &strip_perm_and_refine(ty).as_deref()?.node {
        TypeNode::Func { params, ret } => Some((params.clone(), ret.clone())),
        TypeNode::Closure { params, ret, .. } => Some((
            params.iter().map(|(is_move, ty)| TypeFuncParam { mode: is_move.then_some(ParamMode::Move), r#type: ty.clone() }).collect(),
            ret.clone(),
        )),
        _ => None,
    }
}

/// A type path as declared: itself when it names a declaration, or what a single name
/// resolves to.
fn resolve_comparable_type_path(ctx: &ScopeContext<'_>, path: &[String]) -> Option<TypePath> {
    if ctx.sigma.types.contains_key(&path_key_of(path)) {
        return Some(path.to_vec());
    }
    let [name] = path else {
        return None;
    };
    let resolved = resolve_type_name(ctx, name)?;
    let mut full_path = resolved.origin_opt?;
    full_path.push(resolved.target_opt.unwrap_or_else(|| name.clone()));
    ctx.sigma.types.contains_key(&path_key_of(&full_path)).then_some(full_path)
}

/// Equivalence that also holds between a type named in full and by a name in scope.
fn type_equiv_in_scope(ctx: &ScopeContext<'_>, lhs: &TypeRef, rhs: &TypeRef) -> bool {
    if type_equiv(lhs, rhs) {
        return true;
    }
    let (Some(l), Some(r)) = (lhs.as_deref(), rhs.as_deref()) else {
        return false;
    };
    let perm_of = |ty: &Type| match &ty.node {
        TypeNode::Perm { perm, base } => Some((*perm, base.clone())),
        _ => None,
    };
    let (lhs_perm, rhs_perm) = (perm_of(l), perm_of(r));
    if lhs_perm.is_some() || rhs_perm.is_some() {
        let (lhs_permission, lhs_base) = lhs_perm.unwrap_or((Permission::Const, lhs.clone()));
        let (rhs_permission, rhs_base) = rhs_perm.unwrap_or((Permission::Const, rhs.clone()));
        return lhs_permission == rhs_permission && type_equiv_in_scope(ctx, &lhs_base, &rhs_base);
    }
    let (Some(lhs_path), Some(rhs_path), Some(lhs_args), Some(rhs_args)) =
        (applied_type_path(l), applied_type_path(r), applied_type_args(l), applied_type_args(r))
    else {
        return false;
    };
    let same_path = lhs_path == rhs_path
        || matches!(
            (resolve_comparable_type_path(ctx, lhs_path), resolve_comparable_type_path(ctx, rhs_path)),
            (Some(lhs_resolved), Some(rhs_resolved)) if lhs_resolved == rhs_resolved
        );
    same_path && lhs_args.len() == rhs_args.len() && lhs_args.iter().zip(rhs_args).all(|(l, r)| type_equiv_in_scope(ctx, l, r))
}

const MISMATCH: Diag = Some("E-SEM-2533");

fn typed_arg(arg: &Arg, type_expr: ExprTypeFn<'_>) -> Result<TypeRef, Diag> {
    let typed = type_expr(&arg.value);
    if typed.ok {
        Ok(typed.r#type)
    } else {
        Err(typed.diag_id)
    }
}

/// A callable argument taking `arity` parameters by reference.
fn callable_arg(arg: &Arg, arity: usize, type_expr: ExprTypeFn<'_>) -> Result<(Vec<TypeFuncParam>, TypeRef), Diag> {
    let typed = typed_arg(arg, type_expr)?;
    callable_sig_of(&typed).filter(|(params, _)| params.len() == arity && params.iter().all(|param| param.mode.is_none())).ok_or(MISMATCH)
}

fn sub_or_mismatch(ctx: &ScopeContext<'_>, lhs: &TypeRef, rhs: &TypeRef) -> Result<(), Diag> {
    let sub = subtyping(ctx, lhs, rhs);
    if !sub.ok {
        return Err(sub.diag_id);
    }
    if sub.subtype {
        Ok(())
    } else {
        Err(MISMATCH)
    }
}

fn async_type(out: TypeRef, input: TypeRef, result: TypeRef, err: TypeRef) -> TypeRef {
    make_type_path_with(vec!["Async".to_string()], vec![out, input, result, err])
}

/// `shared~>until(predicate, action)`: waits until the predicate holds of the shared
/// value, then runs the action on it.
fn type_until_call(ctx: &ScopeContext<'_>, lookup_base: &TypeRef, args: &[Arg], type_expr: ExprTypeFn<'_>) -> Result<TypeRef, Diag> {
    let [pred_arg, action_arg] = args else {
        return Err(Some("E-SEM-2532"));
    };
    if pred_arg.pass == ArgPassKind::Move || action_arg.pass == ArgPassKind::Move {
        return Err(Some("E-SEM-2535"));
    }
    let pred_typed = typed_arg(pred_arg, type_expr)?;
    let action_typed = typed_arg(action_arg, type_expr)?;
    let (Some(pred_sig), Some(action_sig)) = (callable_sig_of(&pred_typed), callable_sig_of(&action_typed)) else {
        return Err(MISMATCH);
    };
    let ([pred_param], [action_param]) = (&pred_sig.0[..], &action_sig.0[..]) else {
        return Err(MISMATCH);
    };
    if pred_param.mode.is_some() || action_param.mode.is_some() {
        return Err(MISMATCH);
    }
    if !type_equiv_in_scope(ctx, &pred_param.r#type, &make_type_perm(Permission::Const, lookup_base.clone()))
        || !type_equiv_in_scope(ctx, &action_param.r#type, &make_type_perm(Permission::Unique, lookup_base.clone()))
        || !type_equiv(&pred_sig.1, &make_type_prim("bool"))
    {
        return Err(MISMATCH);
    }
    Ok(async_type(make_type_prim("()"), make_type_prim("()"), action_sig.1, make_type_prim("!")))
}

/// The combinators of an asynchronous computation: `map`, `filter`, `take`, `fold` and
/// `chain`.
fn type_async_combinator_call(
    ctx: &ScopeContext<'_>,
    async_sig: &AsyncSig,
    lookup_base: &TypeRef,
    name: &str,
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
) -> Result<TypeRef, Diag> {
    let unit_type = make_type_prim("()");
    let is_unit = |ty: &TypeRef| type_equiv(ty, &unit_type);
    let one_arg = || -> Result<&Arg, Diag> {
        match args {
            [arg] if arg.value.is_some() => {
                if arg.pass == ArgPassKind::Move {
                    Err(Some("E-SEM-2535"))
                } else {
                    Ok(arg)
                }
            }
            _ => Err(Some("E-SEM-2532")),
        }
    };
    // A stream of values: nothing is sent in and nothing comes back at the end.
    let require_stream = || if is_unit(&async_sig.input) && is_unit(&async_sig.result) { Ok(()) } else { Err(MISMATCH) };
    if id_eq(name, "map") {
        let (params, ret) = callable_arg(one_arg()?, 1, type_expr)?;
        sub_or_mismatch(ctx, &async_sig.out, &params[0].r#type)?;
        return Ok(async_type(ret, async_sig.input.clone(), async_sig.result.clone(), async_sig.err.clone()));
    }
    if id_eq(name, "filter") {
        let arg = one_arg()?;
        require_stream()?;
        let (params, ret) = callable_arg(arg, 1, type_expr)?;
        if !type_equiv(&params[0].r#type, &make_type_perm(Permission::Const, async_sig.out.clone()))
            || !type_equiv(&ret, &make_type_prim("bool"))
        {
            return Err(MISMATCH);
        }
        return Ok(lookup_base.clone());
    }
    if id_eq(name, "take") {
        let arg = one_arg()?;
        require_stream()?;
        let n_typed = typed_arg(arg, type_expr)?;
        sub_or_mismatch(ctx, &n_typed, &make_type_prim("usize"))?;
        return Ok(lookup_base.clone());
    }
    if id_eq(name, "fold") {
        let [init_arg, fn_arg] = args else {
            return Err(Some("E-SEM-2532"));
        };
        if init_arg.value.is_none() || fn_arg.value.is_none() {
            return Err(Some("E-SEM-2532"));
        }
        if init_arg.pass == ArgPassKind::Move || fn_arg.pass == ArgPassKind::Move {
            return Err(Some("E-SEM-2535"));
        }
        require_stream()?;
        let init_typed = typed_arg(init_arg, type_expr)?;
        let (params, ret) = callable_arg(fn_arg, 2, type_expr)?;
        if !type_equiv(&params[0].r#type, &init_typed) || !type_equiv(&params[1].r#type, &async_sig.out) || !type_equiv(&ret, &init_typed) {
            return Err(MISMATCH);
        }
        return Ok(async_type(unit_type.clone(), unit_type, init_typed, async_sig.err.clone()));
    }
    // `chain`: the result of this computation starts the next one.
    let arg = one_arg()?;
    if !is_unit(&async_sig.out) || !is_unit(&async_sig.input) {
        return Err(MISMATCH);
    }
    let (params, ret) = callable_arg(arg, 1, type_expr)?;
    sub_or_mismatch(ctx, &async_sig.result, &params[0].r#type)?;
    let chained_sig = get_async_sig(&ret).ok_or(MISMATCH)?;
    if !is_unit(&chained_sig.out) || !is_unit(&chained_sig.input) || !type_equiv(&chained_sig.err, &async_sig.err) {
        return Err(MISMATCH);
    }
    Ok(ret)
}

/// A member the language builds into a modal state.
struct BuiltinModalMemberSig {
    recv_perm: Permission,
    /// A parameter without a type takes any argument.
    params: Vec<(Option<ParamMode>, TypeRef)>,
    ret: TypeRef,
    /// The call has the type of its first argument.
    ret_from_first_arg: bool,
    /// The diagnostic outside `unsafe`, for a member that needs it.
    unsafe_diag: Option<&'static str>,
}

fn builtin_member(recv_perm: Permission, ret: TypeRef) -> BuiltinModalMemberSig {
    BuiltinModalMemberSig { recv_perm, params: Vec::new(), ret, ret_from_first_arg: false, unsafe_diag: None }
}

fn lookup_builtin_modal_member_sig(modal_path: &[String], state: &str, member_name: &str) -> Option<BuiltinModalMemberSig> {
    let member_in = |member: &str, in_state: &str| id_eq(member_name, member) && id_eq(state, in_state);
    if path_matches_single(modal_path, "Region") {
        let region = |state: &str| make_type_perm(Permission::Unique, make_type_modal_state(vec!["Region".to_string()], state, Vec::new()));
        let unchecked = |sig: BuiltinModalMemberSig| BuiltinModalMemberSig { unsafe_diag: Some("E-MEM-3030"), ..sig };
        if member_in("alloc", "Active") {
            return Some(BuiltinModalMemberSig {
                params: vec![(None, None)],
                ret_from_first_arg: true,
                ..builtin_member(Permission::Unique, None)
            });
        }
        if member_in("reset_unchecked", "Active") {
            return Some(unchecked(builtin_member(Permission::Unique, region("Active"))));
        }
        if member_in("freeze", "Active") {
            return Some(builtin_member(Permission::Unique, region("Frozen")));
        }
        if member_in("thaw", "Frozen") {
            return Some(builtin_member(Permission::Unique, region("Active")));
        }
        if member_in("free_unchecked", "Active") || member_in("free_unchecked", "Frozen") {
            return Some(unchecked(builtin_member(Permission::Unique, region("Freed"))));
        }
        return None;
    }
    if path_matches_single(modal_path, "CancelToken") {
        if member_in("cancel", "Active") {
            return Some(builtin_member(Permission::Const, make_type_prim("()")));
        }
        if member_in("is_cancelled", "Active") {
            return Some(builtin_member(Permission::Const, make_type_prim("bool")));
        }
        if member_in("child", "Active") {
            return Some(builtin_member(Permission::Const, make_type_modal_state(vec!["CancelToken".to_string()], "Active", Vec::new())));
        }
        if member_in("wait_cancelled", "Active") {
            return Some(builtin_member(Permission::Const, make_type_path_with(vec!["Async".to_string()], vec![make_type_prim("()")])));
        }
    }
    None
}

/// `resume` on a suspended asynchronous computation sends a value in and gives the
/// computation back in whichever state it reached.
fn lookup_async_resume_member_sig(
    ctx: &ScopeContext<'_>,
    lookup_base: &TypeRef,
    modal: &TypeModalState,
    member_name: &str,
) -> Option<BuiltinModalMemberSig> {
    if !is_async_modal_path(&modal.path) || !id_eq(&modal.state, "Suspended") || !id_eq(member_name, "resume") {
        return None;
    }
    let sig = async_sig_of(ctx, lookup_base)?;
    let state = |state: &str| {
        let args = vec![sig.out.clone(), sig.input.clone(), sig.result.clone(), sig.err.clone()];
        make_type_perm(Permission::Unique, make_type_modal_state(vec!["Async".to_string()], state, args))
    };
    let mut members = vec![state("Suspended"), state("Completed")];
    let never_fails = matches!(strip_perm(&sig.err).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if id_eq(name, "!"));
    if !never_fails {
        members.push(state("Failed"));
    }
    Some(BuiltinModalMemberSig { params: vec![(None, sig.input.clone())], ..builtin_member(Permission::Unique, make_type_union(members)) })
}

/// Checks the arguments of a built-in modal member and gives the call's type.
fn check_builtin_modal_member_args(
    ctx: &ScopeContext<'_>,
    sig: &BuiltinModalMemberSig,
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: PlaceTypeFn<'_>,
    check_expr: ArgCheckFn<'_>,
) -> Result<TypeRef, Box<ExprTypeResult>> {
    let fail = |diag_id: Diag| Box::new(ExprTypeResult::failed(diag_id));
    if args.len() != sig.params.len() {
        return Err(fail(Some("E-SEM-2532")));
    }
    if sig.params.iter().zip(args).any(|((mode, _), arg)| missing_required_move_for_consuming(*mode, arg)) {
        return Err(fail(Some("E-SEM-2534")));
    }
    if sig.params.iter().zip(args).any(|((mode, _), arg)| mode.is_none() && arg.pass == ArgPassKind::Move) {
        return Err(fail(Some("E-SEM-2535")));
    }
    let typed = |expr: &ExprPtr| {
        let typed = type_expr(expr);
        if typed.ok {
            Ok(typed.r#type)
        } else {
            Err(fail(typed.diag_id))
        }
    };
    // An argument that names no storage is first checked against the parameter type.
    let checked_as_param = |expr: &ExprPtr, param_type: &TypeRef| -> Result<bool, Box<ExprTypeResult>> {
        let checked = check_expr(expr, param_type);
        if !checked.ok && checked.diag_id.is_some() {
            return Err(fail(checked.diag_id));
        }
        Ok(checked.ok)
    };
    let mut arg_types: Vec<TypeRef> = Vec::with_capacity(args.len());
    for ((mode, param_type), arg) in sig.params.iter().zip(args) {
        if arg.value.is_none() {
            return Err(fail(MISMATCH));
        }
        let arg_type = if mode.is_none() {
            if arg.pass == ArgPassKind::Copy {
                arg_types.push(typed(&arg_pass_expr(arg))?);
                continue;
            }
            let has_source_prov = has_source_provenance(&arg.value);
            if has_source_prov && !is_place_expr_for_call(&arg.value) {
                return Err(fail(Some("E-TYP-1603")));
            }
            if param_type.is_some() && !has_source_prov && checked_as_param(&arg.value, param_type)? {
                arg_types.push(param_type.clone());
                continue;
            }
            if has_source_prov {
                let place_type = type_place(&arg.value);
                if !place_type.ok {
                    return Err(Box::new(ExprTypeResult {
                        diag_id: place_type.diag_id,
                        diag_detail: place_type.diag_detail,
                        diag_span: place_type.diag_span,
                        ..Default::default()
                    }));
                }
                place_type.r#type
            } else {
                typed(&arg.value)?
            }
        } else {
            let moved = arg_pass_expr(arg);
            if param_type.is_some() && uses_call_temp_for_consuming(*mode, arg) && checked_as_param(&moved, param_type)? {
                arg_types.push(param_type.clone());
                continue;
            }
            typed(&moved)?
        };
        if param_type.is_some() {
            let sub = argument_type_compatible(ctx, &arg_type, param_type, *mode);
            if !sub.ok {
                return Err(fail(sub.diag_id));
            }
            if !sub.subtype {
                return Err(fail(MISMATCH));
            }
        }
        arg_types.push(arg_type);
    }
    if sig.ret_from_first_arg {
        return arg_types.into_iter().next().ok_or_else(|| fail(Some("E-SEM-2532")));
    }
    Ok(sig.ret.clone())
}

/// Whether a reference may be taken to an argument: an indexed place must be indexed by
/// `usize`.
fn addr_of_ok(expr: &ExprPtr, type_expr: ExprTypeFn<'_>, check_expr: Option<ArgCheckFn<'_>>) -> Result<(), Diag> {
    let Some(e) = expr.as_deref().filter(|_| is_place_expr(expr)) else {
        return Err(None);
    };
    let ExprNode::IndexAccessExpr(index) = &e.node else {
        return Ok(());
    };
    if let Some(check_expr) = check_expr {
        let checked = check_expr(&index.index, &make_type_prim("usize"));
        if checked.ok {
            return Ok(());
        }
        if !matches!(checked.diag_id, None | Some("E-SEM-2526")) {
            return Err(checked.diag_id);
        }
    }
    let idx_type = type_expr(&index.index);
    if !idx_type.ok {
        return Err(idx_type.diag_id);
    }
    if matches!(strip_perm(&idx_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize") {
        return Ok(());
    }
    let base_type = type_expr(&index.base);
    if !base_type.ok {
        return Err(base_type.diag_id);
    }
    Err(Some(match strip_perm(&base_type.r#type).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Array { .. }) => "Index-Array-NonUsize",
        Some(TypeNode::Slice(_)) => "Index-Slice-NonUsize",
        _ => "Index-NonIndexable",
    }))
}

/// Checks the arguments of a method call against its lowered parameters.
fn check_lowered_args_ok(
    ctx: &ScopeContext<'_>,
    lowered_params: &[TypeFuncParam],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    check_expr: Option<ArgCheckFn<'_>>,
) -> Result<(), Diag> {
    if lowered_params.len() != args.len() {
        return Err(Some("E-SEM-2532"));
    }
    if lowered_params.iter().zip(args).any(|(param, arg)| missing_required_move_for_consuming(param.mode, arg)) {
        return Err(Some("E-SEM-2534"));
    }
    if lowered_params.iter().zip(args).any(|(param, arg)| param.mode.is_none() && arg.pass == ArgPassKind::Move) {
        return Err(Some("E-SEM-2535"));
    }
    let typed = |expr: &ExprPtr| {
        let typed = type_expr(expr);
        if typed.ok {
            Ok(typed.r#type)
        } else {
            Err(typed.diag_id)
        }
    };
    // An argument that names no storage is first checked against the parameter type.
    let checked_as_param = |expr: &ExprPtr, param: &TypeFuncParam| -> Result<bool, Diag> {
        let Some(check_expr) = check_expr else {
            return Ok(false);
        };
        let checked = check_expr(expr, &param.r#type);
        if !checked.ok && checked.diag_id.is_some() {
            return Err(checked.diag_id);
        }
        Ok(checked.ok)
    };
    let mut arg_types: Vec<TypeRef> = Vec::with_capacity(args.len());
    for (param, arg) in lowered_params.iter().zip(args) {
        let has_source_prov = has_source_provenance(&arg.value);
        if param.mode.is_some() {
            let arg_expr = arg_pass_expr(arg);
            if !has_source_prov && checked_as_param(&arg_expr, param)? {
                arg_types.push(param.r#type.clone());
            } else {
                arg_types.push(typed(&arg_expr)?);
            }
            continue;
        }
        if arg.pass == ArgPassKind::Copy {
            arg_types.push(typed(&arg_pass_expr(arg))?);
            continue;
        }
        if has_source_prov && !is_place_expr_for_call(&arg.value) {
            return Err(Some("E-TYP-1603"));
        }
        if let (true, Some(type_place)) = (has_source_prov, type_place) {
            let place_type = type_place(&arg.value);
            if !place_type.ok {
                return Err(place_type.diag_id);
            }
            arg_types.push(place_type.r#type);
        } else if !has_source_prov && checked_as_param(&arg.value, param)? {
            arg_types.push(param.r#type.clone());
        } else {
            arg_types.push(typed(&arg.value)?);
        }
    }
    for (param, arg_type) in lowered_params.iter().zip(&arg_types) {
        let sub = argument_type_compatible(ctx, arg_type, &param.r#type, param.mode);
        if !sub.ok {
            return Err(sub.diag_id);
        }
        if !sub.subtype {
            return Err(Some("E-SEM-2533"));
        }
    }
    for (param, arg) in lowered_params.iter().zip(args) {
        if param.mode != Some(ParamMode::Move) && has_source_provenance(&arg.value) {
            addr_of_ok(&arg.value, type_expr, check_expr)?;
        }
    }
    Ok(())
}

/// Checks the arguments against the declared parameters, with the method's type
/// arguments substituted when it has any.
#[allow(clippy::too_many_arguments)]
pub fn args_ok(
    ctx: &ScopeContext<'_>,
    params: &[ast::Param],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    lower_type_fn: LowerTypeFn<'_>,
    subst: Option<&TypeSubst>,
    check_expr: Option<ArgCheckFn<'_>>,
) -> Result<(), Diag> {
    if params.len() != args.len() {
        return Err(Some("E-SEM-2532"));
    }
    let mut lowered_params = Vec::with_capacity(params.len());
    for param in params {
        let mut param_type = lower_type_fn(&param.r#type)?;
        if let Some(subst) = subst.filter(|subst| !subst.is_empty()) {
            param_type = instantiate_type(&param_type, subst);
        }
        lowered_params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: param_type });
    }
    check_lowered_args_ok(ctx, &lowered_params, args, type_expr, type_place, check_expr)
}

/// The types of the arguments as inference of a method's type arguments sees them.
fn collect_arg_types(
    params: &[ast::Param],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    lower_type_fn: LowerTypeFn<'_>,
) -> Result<Vec<TypeRef>, Diag> {
    if params.len() != args.len() {
        return Err(Some("E-SEM-2532"));
    }
    let mut modes = Vec::with_capacity(params.len());
    for param in params {
        lower_type_fn(&param.r#type)?;
        modes.push(lower_param_mode(param.mode));
    }
    if modes.iter().zip(args).any(|(mode, arg)| missing_required_move_for_consuming(*mode, arg)) {
        return Err(Some("E-SEM-2534"));
    }
    if modes.iter().zip(args).any(|(mode, arg)| mode.is_none() && arg.pass == ArgPassKind::Move) {
        return Err(Some("E-SEM-2535"));
    }
    let typed = |expr: &ExprPtr| {
        let typed = type_expr(expr);
        if typed.ok {
            Ok(typed.r#type)
        } else {
            Err(typed.diag_id)
        }
    };
    let mut out_types = Vec::with_capacity(args.len());
    for (mode, arg) in modes.iter().zip(args) {
        if mode.is_some() || arg.pass == ArgPassKind::Copy {
            out_types.push(typed(&arg_pass_expr(arg))?);
            continue;
        }
        let has_source_prov = has_source_provenance(&arg.value);
        if has_source_prov && !is_place_expr_for_call(&arg.value) {
            return Err(Some("E-TYP-1603"));
        }
        match type_place.filter(|_| has_source_prov) {
            Some(type_place) => {
                let place_type = type_place(&arg.value);
                if !place_type.ok {
                    return Err(place_type.diag_id);
                }
                out_types.push(place_type.r#type);
            }
            None => out_types.push(typed(&arg.value)?),
        }
    }
    Ok(out_types)
}

/// Infers a generic method's type arguments from the arguments of the call.
fn infer_method_subst(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    method_params: &[ast::Param],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    lower_type_fn: LowerTypeFn<'_>,
) -> Result<TypeSubst, Diag> {
    let expected_param_types = method_params.iter().map(|param| lower_type_fn(&param.r#type)).collect::<Result<Vec<_>, _>>()?;
    let actual_arg_types = collect_arg_types(method_params, args, type_expr, type_place, lower_type_fn)?;
    let mut bindings: BTreeMap<String, TypeRef> = BTreeMap::new();
    for (expected, actual) in expected_param_types.iter().zip(&actual_arg_types) {
        if !bind_type_params(ctx, params, expected, actual, &mut bindings) {
            return Err(Some("E-SEM-2533"));
        }
    }
    let mut inferred_args: Vec<TypeRef> = Vec::with_capacity(params.len());
    for (index, param) in params.iter().enumerate() {
        if let Some(bound) = bindings.get(&param.name) {
            inferred_args.push(bound.clone());
            continue;
        }
        if param.default_type.is_none() {
            return Err(Some("E-TYP-2301"));
        }
        let mut value = lower_type_fn(&param.default_type)?;
        if index > 0 {
            value = instantiate_type(&value, &build_substitution(&params[..index], &inferred_args[..index]));
        }
        inferred_args.push(value);
    }
    for (param, arg) in params.iter().zip(&inferred_args) {
        for bound in &param.bounds {
            if !is_capability_class_path(&bound.class_path) && !ctx.sigma.classes.contains_key(&path_key_of(&bound.class_path)) {
                return Err(Some("E-TYP-2305"));
            }
            if !type_implements_class(ctx, arg, &bound.class_path) {
                return Err(Some("E-TYP-2302"));
            }
        }
    }
    Ok(build_substitution(params, &inferred_args))
}

/// A method or transition of a modal by name: the one of the current state, or whether
/// another state has one.
#[derive(Default)]
struct ModalMemberLookupResult<'c> {
    method: Option<&'c ast::StateMethodDecl>,
    transition: Option<&'c ast::TransitionDecl>,
    method_in_other_state: bool,
    transition_in_other_state: bool,
}

fn lookup_modal_member<'c>(ctx: &'c ScopeContext<'_>, modal: &TypeModalState, member_name: &str) -> ModalMemberLookupResult<'c> {
    let mut result = ModalMemberLookupResult::default();
    let Some(TypeDecl::Modal(modal_decl)) = ctx.sigma.types.get(&path_key_of(&modal.path)) else {
        return result;
    };
    for state in &modal_decl.states {
        let in_current_state = id_eq(&state.name, &modal.state);
        for member in &state.members {
            match member {
                ast::StateMember::StateMethodDecl(method) if id_eq(&method.name, member_name) => {
                    if in_current_state {
                        result.method = Some(method);
                        return result;
                    }
                    result.method_in_other_state = true;
                }
                ast::StateMember::TransitionDecl(transition) if id_eq(&transition.name, member_name) => {
                    if in_current_state {
                        result.transition = Some(transition);
                        return result;
                    }
                    result.transition_in_other_state = true;
                }
                _ => {}
            }
        }
    }
    result
}

fn check_builtin_method_args(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    params: &[TypeFuncParam],
    args: &[Arg],
    env: &TypeEnv,
) -> Result<(), Diag> {
    if args.len() != params.len() {
        return Err(Some("E-SEM-2532"));
    }
    for (expected, arg) in params.iter().zip(args) {
        if arg.pass == ArgPassKind::Move && expected.mode.is_none() {
            return Err(Some("E-SEM-2535"));
        }
        if arg.pass == ArgPassKind::Ref && expected.mode == Some(ParamMode::Move) {
            return Err(Some("E-SEM-2534"));
        }
        let checked = check_expr_against(ctx, &arg_ctx_for(type_ctx, &expected.r#type), &arg.value, &expected.r#type, env);
        if !checked.ok {
            return Err(checked.diag_id);
        }
    }
    Ok(())
}

/// The context an argument is typed in: handing shared data to a `unique` parameter
/// writes it, to a `shared` or `const` one reads it.
fn arg_ctx_for<'t>(type_ctx: &StmtTypeContext<'t>, expected: &TypeRef) -> StmtTypeContext<'t> {
    if expected.is_none() {
        return type_ctx.clone();
    }
    match perm_of_type(expected) {
        Permission::Unique => with_shared_access_mode(type_ctx, ast::KeyMode::Write),
        Permission::Shared | Permission::Const => with_shared_access_mode(type_ctx, ast::KeyMode::Read),
    }
}

/// A class method named through a dynamic or opaque type: by the path as written, by
/// its last segment, or by that segment in the current module.
fn lookup_class_method_by_any_path<'c>(ctx: &'c ScopeContext<'_>, path: &[String], name: &str) -> Option<&'c ast::ClassMethodDecl> {
    lookup_class_method(ctx, path, name).or_else(|| {
        let last = path.last()?;
        lookup_class_method(ctx, std::slice::from_ref(last), name).or_else(|| {
            let mut module_path = ctx.current_module.clone();
            module_path.push(last.clone());
            lookup_class_method(ctx, &module_path, name)
        })
    })
}

/// The signature of a method as a call needs it.
struct MethodSig<'m> {
    receiver: &'m ast::Receiver,
    generic_params: &'m Option<ast::GenericParams>,
    params: &'m [ast::Param],
    return_type_opt: &'m ast::TypePtr,
    attrs: &'m [ast::AttributeItem],
}

impl<'m> MethodSig<'m> {
    fn of_class(method: &'m ast::ClassMethodDecl) -> Self {
        MethodSig {
            receiver: &method.receiver,
            generic_params: &method.generic_params,
            params: &method.params,
            return_type_opt: &method.return_type_opt,
            attrs: &method.attrs,
        }
    }

    fn of_record(method: &'m ast::MethodDecl) -> Self {
        MethodSig {
            receiver: &method.receiver,
            generic_params: &method.generic_params,
            params: &method.params,
            return_type_opt: &method.return_type_opt,
            attrs: &method.attrs,
        }
    }
}

pub fn type_method_call_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::MethodCallExpr,
    env: &TypeEnv,
    span: &Span,
) -> ExprTypeResult {
    let failed = |diag_id: Diag| ExprTypeResult::failed(diag_id);
    let Some(receiver_expr) = expr.receiver.as_deref() else {
        return ExprTypeResult::default();
    };
    if id_eq(&expr.name, "drop") {
        return failed(Some(if type_ctx.contract_dynamic { "Drop-Call-Err-Dyn" } else { "Drop-Call-Err" }));
    }
    let type_expr_fn = |inner: &ExprPtr| type_expr(ctx, type_ctx, inner, env);
    let type_place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let check_expr_fn = |inner: &ExprPtr, expected: &TypeRef| -> CheckResult {
        let checked = check_expr_against(ctx, &arg_ctx_for(type_ctx, expected), inner, expected, env);
        CheckResult {
            ok: checked.ok,
            diag_id: checked.diag_id,
            diag_detail: checked.diag_detail,
            diag_span: checked.diag_span,
            ..Default::default()
        }
    };
    let lower_type_fn = |ty: &ast::TypePtr| lower_type(ctx, ty);

    // The receiver is a place when it can be, so that the method sees its permission.
    let place = type_place(ctx, type_ctx, &expr.receiver, env);
    let receiver_type = if place.ok {
        place.r#type
    } else {
        let base_expr = type_expr(ctx, type_ctx, &expr.receiver, env);
        if !base_expr.ok {
            return failed(base_expr.diag_id);
        }
        base_expr.r#type
    };
    let mut lookup_base = strip_perm_and_refine(&receiver_type);
    let caller_perm = perm_of_type(&receiver_type);

    // A shared receiver under a held key needs the key in the mode the method asks for.
    let check_shared_receiver_access = |receiver_requirement: Permission| -> Result<(), Diag> {
        if caller_perm != Permission::Shared {
            return Ok(());
        }
        let built = build_key_path(&expr.receiver);
        if !built.success {
            return Err(Some("E-CON-0034"));
        }
        let mut covering: Option<ast::KeyMode> = None;
        for held in type_ctx.held_key_paths.iter().filter(|held| is_prefix(&held.path, &built.path)) {
            if covering.is_none() || held.mode == ast::KeyMode::Write {
                covering = Some(held.mode);
            }
        }
        let required = if receiver_requirement == Permission::Const { ast::KeyMode::Read } else { ast::KeyMode::Write };
        match covering {
            Some(held) if held != ast::KeyMode::Write && held != required => Err(Some("E-CON-0005")),
            _ => Ok(()),
        }
    };

    if lookup_base.is_none() {
        return ExprTypeResult::default();
    }
    match normalize_alias_top_level_for_method_call(ctx, &lookup_base) {
        Err(diag_id) => return failed(diag_id),
        Ok(Some(expanded)) => lookup_base = strip_perm_and_refine(&expanded),
        Ok(None) => {}
    }
    let not_found = || ExprTypeResult {
        diag_id: Some("LookupMethod-NotFound"),
        diag_detail: format!("method '{}' on type '{}'", expr.name, type_to_string(&lookup_base)),
        ..Default::default()
    };
    if id_eq(&expr.name, "until") {
        if !permission_admits(caller_perm, Permission::Shared) {
            return failed(Some("E-TYP-1605"));
        }
        return match type_until_call(ctx, &lookup_base, &expr.args, &type_expr_fn) {
            Ok(ty) => ExprTypeResult::typed(ty),
            Err(diag_id) => failed(diag_id),
        };
    }
    if let Some(async_sig) = async_sig_of(ctx, &lookup_base) {
        if is_builtin_modal_general_member(&["Async".to_string()], &expr.name) {
            if !permission_admits(caller_perm, Permission::Const) {
                return failed(Some("E-TYP-1605"));
            }
            return match type_async_combinator_call(ctx, &async_sig, &lookup_base, &expr.name, &expr.args, &type_expr_fn) {
                Ok(ty) => ExprTypeResult::typed(ty),
                Err(diag_id) => failed(diag_id),
            };
        }
    }
    let Some(base) = lookup_base.as_deref() else {
        return ExprTypeResult::default();
    };
    let builtin_sig = lookup_string_bytes_builtin_method_sig(&lookup_base, &expr.name)
        .or_else(|| lookup_foundational_builtin_method_sig(Some(ctx), &lookup_base, &expr.name));
    if let Some(builtin_sig) = builtin_sig {
        if !permission_admits(caller_perm, builtin_sig.recv_perm) {
            return failed(Some("E-TYP-1605"));
        }
        if let Err(diag_id) = check_shared_receiver_access(builtin_sig.recv_perm) {
            return failed(diag_id);
        }
        let recv_sub = subtyping(ctx, &lookup_base, &builtin_sig.recv_type);
        if !recv_sub.ok {
            return failed(recv_sub.diag_id);
        }
        if !recv_sub.subtype {
            return not_found();
        }
        if let Err(diag_id) = check_builtin_method_args(ctx, type_ctx, &builtin_sig.params, &expr.args, env) {
            return failed(diag_id);
        }
        return ExprTypeResult::typed(builtin_sig.ret);
    }

    if applied_type_path(base).is_none() && !matches!(base.node, TypeNode::ModalState(_) | TypeNode::Dynamic(_) | TypeNode::Opaque { .. }) {
        return not_found();
    }

    // A method found: the receiver must admit it, the arguments must fit, and the
    // call has its return type, with inferred type arguments substituted.
    let call_method = |sig: MethodSig<'_>, method_lower_type: LowerTypeFn<'_>| -> ExprTypeResult {
        let recv_type = match recv_type_for_receiver(&lookup_base, sig.receiver, method_lower_type) {
            Ok(recv_type) => recv_type,
            Err(diag_id) => return failed(diag_id),
        };
        let method_perm = perm_of_type(&recv_type);
        if !permission_admits(caller_perm, method_perm) {
            return failed(Some("E-TYP-1605"));
        }
        if let Err(diag_id) = check_shared_receiver_access(method_perm) {
            return failed(diag_id);
        }
        let mut subst: Option<TypeSubst> = None;
        if let Some(generics) = sig.generic_params.as_ref().filter(|generics| !generics.params.is_empty()) {
            match infer_method_subst(ctx, &generics.params, sig.params, &expr.args, &type_expr_fn, Some(&type_place_fn), method_lower_type) {
                Ok(inferred) => subst = Some(inferred),
                Err(diag_id) => return failed(diag_id),
            }
        }
        if let Err(diag_id) = args_ok(
            ctx,
            sig.params,
            &expr.args,
            &type_expr_fn,
            Some(&type_place_fn),
            method_lower_type,
            subst.as_ref(),
            Some(&check_expr_fn),
        ) {
            return failed(diag_id);
        }
        let mut ret_type = if sig.return_type_opt.is_none() {
            make_type_prim("()")
        } else {
            match method_lower_type(sig.return_type_opt) {
                Ok(lowered) => lowered,
                Err(diag_id) => return failed(diag_id),
            }
        };
        if let Some(subst) = &subst {
            ret_type = instantiate_type(&ret_type, subst);
        }
        emit_deprecated_reference_warning_from_attrs(sig.attrs, type_ctx, Some(receiver_expr.span.clone()));
        ExprTypeResult::typed(ret_type)
    };

    // A built-in method of a capability: its parameters are checked as declared ones.
    let call_cap_method = |sig: CapMethodSig| -> ExprTypeResult {
        if !permission_admits(caller_perm, sig.recv_perm) {
            return failed(Some("E-TYP-1605"));
        }
        if let Err(diag_id) = check_shared_receiver_access(sig.recv_perm) {
            return failed(diag_id);
        }
        match args_ok(ctx, &sig.params, &expr.args, &type_expr_fn, Some(&type_place_fn), &lower_type_fn, None, Some(&check_expr_fn)) {
            Ok(()) => ExprTypeResult::typed(sig.ret),
            Err(diag_id) => failed(diag_id),
        }
    };

    match &base.node {
        TypeNode::ModalState(modal) => {
            let builtin_sig = lookup_async_resume_member_sig(ctx, &lookup_base, modal, &expr.name)
                .or_else(|| lookup_builtin_modal_member_sig(&modal.path, &modal.state, &expr.name));
            if let Some(builtin_sig) = builtin_sig {
                if !permission_admits(caller_perm, builtin_sig.recv_perm) {
                    return failed(Some("E-TYP-1605"));
                }
                if let Err(diag_id) = check_shared_receiver_access(builtin_sig.recv_perm) {
                    return failed(diag_id);
                }
                if let Some(unsafe_diag) = builtin_sig.unsafe_diag.filter(|_| !is_in_unsafe_span(ctx, span)) {
                    return failed(Some(unsafe_diag));
                }
                return match check_builtin_modal_member_args(ctx, &builtin_sig, &expr.args, &type_expr_fn, &type_place_fn, &check_expr_fn) {
                    Ok(ty) => ExprTypeResult::typed(ty),
                    Err(failure) => *failure,
                };
            }
            let modal_subst = match ctx.sigma.types.get(&path_key_of(&modal.path)) {
                Some(TypeDecl::Modal(modal_decl)) => modal_decl
                    .generic_params
                    .as_ref()
                    .filter(|generics| generics.params.len() == modal.generic_args.len())
                    .map(|generics| build_substitution(&generics.params, &modal.generic_args)),
                _ => None,
            };
            let modal_lower_type = |ty: &ast::TypePtr| -> Result<TypeRef, Diag> {
                let lowered = lower_type(ctx, ty)?;
                Ok(match &modal_subst {
                    Some(subst) => instantiate_type(&lowered, subst),
                    None => lowered,
                })
            };
            let modal_member = lookup_modal_member(ctx, modal, &expr.name);
            if let Some(transition) = modal_member.transition {
                if !permission_admits(caller_perm, Permission::Unique) {
                    return failed(Some("E-TYP-2056"));
                }
                if let Err(diag_id) = check_shared_receiver_access(Permission::Unique) {
                    return failed(diag_id);
                }
                if !state_member_visible(ctx, &modal.path, transition.vis) {
                    return failed(Some("E-TYP-2064"));
                }
                if let Err(diag_id) = args_ok(
                    ctx,
                    &transition.params,
                    &expr.args,
                    &type_expr_fn,
                    Some(&type_place_fn),
                    &modal_lower_type,
                    None,
                    Some(&check_expr_fn),
                ) {
                    return failed(diag_id);
                }
                return ExprTypeResult::typed(make_type_modal_state(modal.path.clone(), &transition.target_state, modal.generic_args.clone()));
            }
            if let Some(method) = modal_member.method {
                if !state_member_visible(ctx, &modal.path, method.vis) {
                    return failed(Some("E-TYP-2064"));
                }
                let recv_type = match recv_type_for_receiver(&lookup_base, &method.receiver, modal_lower_type) {
                    Ok(recv_type) => recv_type,
                    Err(diag_id) => return failed(diag_id),
                };
                let method_perm = perm_of_type(&recv_type);
                if !permission_admits(caller_perm, method_perm) {
                    return failed(Some("E-TYP-1605"));
                }
                if let Err(diag_id) = check_shared_receiver_access(method_perm) {
                    return failed(diag_id);
                }
                if let Err(diag_id) = args_ok(
                    ctx,
                    &method.params,
                    &expr.args,
                    &type_expr_fn,
                    Some(&type_place_fn),
                    &modal_lower_type,
                    None,
                    Some(&check_expr_fn),
                ) {
                    return failed(diag_id);
                }
                let ret_type = if method.return_type_opt.is_none() {
                    make_type_prim("()")
                } else {
                    match modal_lower_type(&method.return_type_opt) {
                        Ok(lowered) => lowered,
                        Err(diag_id) => return failed(diag_id),
                    }
                };
                emit_deprecated_reference_warning_from_attrs(&method.attrs, type_ctx, Some(receiver_expr.span.clone()));
                return ExprTypeResult::typed(ret_type);
            }
            return failed(Some(if modal_member.transition_in_other_state { "E-TYP-2056" } else { "E-TYP-2053" }));
        }
        TypeNode::Dynamic(path) => {
            if let Some(sig) = lookup_capability_class_method_sig(path, &expr.name) {
                if sig.raw_heap && !is_in_unsafe_span(ctx, span) {
                    return failed(Some("E-MEM-3030"));
                }
                return call_cap_method(sig);
            }
            // `Reactor` is declared in source; its methods are called without a vtable.
            if path_matches_builtin_name(path, "Reactor") {
                if let Some(method) = lookup_class_method(ctx, path, &expr.name) {
                    return call_method(MethodSig::of_class(method), &lower_type_fn);
                }
            }
            let Some(method) = lookup_class_method_by_any_path(ctx, path, &expr.name) else {
                return not_found();
            };
            if !vtable_eligible(method) {
                return failed(Some("E-TYP-2540"));
            }
            return call_method(MethodSig::of_class(method), &lower_type_fn);
        }
        TypeNode::Opaque { class_path, .. } => {
            let Some(method) = lookup_class_method_by_any_path(ctx, class_path, &expr.name) else {
                return ExprTypeResult {
                    diag_id: Some("E-TYP-2510"),
                    diag_detail: format!("method '{}' on type '{}'", expr.name, type_to_string(&lookup_base)),
                    ..Default::default()
                };
            };
            return call_method(MethodSig::of_class(method), &lower_type_fn);
        }
        TypeNode::Path { path, .. } => {
            if let Some(sig) = lookup_capability_type_method_sig(path, &expr.name, expr.args.len()) {
                return call_cap_method(sig);
            }
        }
        _ => {}
    }

    let lookup = match lookup_method_static(ctx, &lookup_base, &expr.name) {
        Ok(lookup) => lookup,
        Err(diag_id) => return failed(diag_id),
    };
    if let (Some(record_method), Some(record_decl)) = (lookup.record_method, lookup.record_decl) {
        // The method's types are lowered with the record's type parameters in scope
        // and then take the record's arguments.
        let mut record_method_ctx = ctx.clone();
        record_method_ctx.scopes = bind_type_params_in_scope(ctx, &record_decl.generic_params);
        let record_subst = match &record_decl.generic_params {
            Some(generics) if lookup.record_generic_args.len() > generics.params.len() => return failed(Some("LookupMethod-NotFound")),
            Some(generics) => Some(build_substitution(&generics.params, &lookup.record_generic_args)),
            None if !lookup.record_generic_args.is_empty() => return failed(Some("LookupMethod-NotFound")),
            None => None,
        };
        let method_lower_type = |ty: &ast::TypePtr| -> Result<TypeRef, Diag> {
            let lowered = lower_type(&record_method_ctx, ty)?;
            Ok(match &record_subst {
                Some(subst) => instantiate_type(&lowered, subst),
                None => lowered,
            })
        };
        return call_method(MethodSig::of_record(record_method), &method_lower_type);
    }
    if let Some(record_method) = lookup.record_method {
        return call_method(MethodSig::of_record(record_method), &lower_type_fn);
    }
    match lookup.class_method {
        Some(class_method) => call_method(MethodSig::of_class(class_method), &lower_type_fn),
        None => not_found(),
    }
}

fn path_matches_single(path: &[String], name: &str) -> bool {
    matches!(path, [only] if id_eq(only, name))
}
