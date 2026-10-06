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
use crate::caps::builtin_paths::{is_capability_class_path, is_context_type_path, path_matches_builtin_name};
use crate::composite::classes::{lookup_class_method, type_implements_class, vtable_eligible};
use crate::composite::record_methods::{lookup_method_static, recv_type_for_receiver};
use crate::context::{ScopeContext, TypeDecl};
use crate::generics::generic_params::bind_type_params as bind_type_params_in_scope;
use crate::generics::monomorphize::{build_substitution, instantiate_type, TypeSubst};
use crate::keys::key_paths::{build_key_path, is_prefix};
use crate::memory::calls::{
    arg_pass_expr, has_source_provenance, is_place_expr_for_call, missing_required_move_for_consuming, ArgCheckFn,
};
use crate::modal::builtin_modal_intrinsics::is_builtin_modal_general_member;
use crate::modal::lookup::state_member_visible;
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::callbacks::{CheckResult, ExprTypeFn, PlaceTypeFn};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::pending::pending;
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
    // The span decides whether a built-in method that needs `unsafe` is inside it.
    let _ = span;
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
    let unported = |what: &'static str| {
        pending(what);
        ExprTypeResult::default()
    };

    if id_eq(&expr.name, "until") {
        return unported("WaitUntilMethod");
    }
    if async_sig_of(ctx, &lookup_base).is_some() && is_builtin_modal_general_member(&["Async".to_string()], &expr.name) {
        return unported("AsyncCombinator");
    }
    let Some(base) = lookup_base.as_deref() else {
        return ExprTypeResult::default();
    };
    if matches!(base.node, TypeNode::String(_) | TypeNode::Bytes(_)) {
        return unported("StringBytesMethod");
    }

    if let Some(builtin_sig) = lookup_foundational_builtin_method_sig(Some(ctx), &lookup_base, &expr.name) {
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

    match &base.node {
        TypeNode::ModalState(modal) => {
            if path_matches_single(&modal.path, "Region") || path_matches_single(&modal.path, "CancelToken") {
                return unported("BuiltinModalMember");
            }
            if is_async_modal_path(&modal.path) && id_eq(&modal.state, "Suspended") && id_eq(&expr.name, "resume") {
                return unported("BuiltinModalMember");
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
            // The capability classes other than `Reactor` have built-in methods.
            if is_capability_class_path(path) && !path_matches_builtin_name(path, "Reactor") {
                return unported("CapabilityMethod");
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
            let comptime_capability = ["ProjectFiles", "ComptimeDiagnostics", "Introspect", "TypeEmitter"]
                .iter()
                .any(|name| path_matches_single(path, name));
            if comptime_capability || is_context_type_path(path) {
                return unported("CapabilityMethod");
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
