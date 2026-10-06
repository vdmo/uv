//! Calls of procedures and function values: which procedure a callee names, which of
//! several overloads applies, the type arguments a generic call leaves to be inferred,
//! and the checks at the call site.

use std::cell::OnceCell;
use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;
use uv_source::ast::{self, Arg, ArgPassKind, ExprNode, ExprPtr};
use uv_source::attributes::{attrs, has_attribute};

use super::call_contracts::{check_call_site_precondition, check_foreign_static_assumes};
use crate::caps::builtin_paths::{is_capability_class_path, lookup_builtin_record_ctor_path, path_matches_builtin_name};
use crate::caps::cap_concurrency::is_gpu_intrinsic_name;
use crate::composite::classes::type_implements_class;
use crate::composite::function_types::{proc_type_of, value_path_type};
use crate::context::ScopeContext;
use crate::generics::monomorphize::{build_substitution, instantiate_type, TypeSubst};
use crate::keys::key_paths::{build_key_path, is_prefix};
use crate::memory::calls::{
    arg_diagnostic_span, arg_pass_expr, find_module, has_source_provenance, is_function_value_type, is_in_unsafe_span,
    is_place_expr_for_call, missing_required_move_for_consuming, type_call, type_call_with_subst, ArgCheckFn,
    CallCalleeFacts, CallTypeResult,
};
use crate::memory::regions::ProvenanceKind;
use crate::provenance::prov_expr::track_expr_provenance;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};
use crate::resolve::scopes_lookup::{resolve_type_name, resolve_value_name};
use crate::typing::callbacks::{CheckResult, ExprTypeFn, PlaceTypeFn};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::outcome::{classify_outcome_intro, OutcomeIntro};
use crate::typing::pending::pending;
use crate::typing::stmt::binding_stmt::normalize_deprecated_message;
use crate::typing::stmt_context::{with_shared_access_mode, ContractPhase, StmtTypeContext};
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::{bind_of, gpu_context, BindingProvenanceSeedKind, TypeBinding, TypeEnv};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, type_expr, type_place};
use crate::typing::type_lookup::{lookup_record_decl, record_fields};
use crate::typing::type_lower::{lower_param_mode, lower_type};
use crate::typing::type_predicates::{is_capability_type, perm_of_type, strip_perm, strip_perm_and_refine};
use crate::typing::types::*;

/// A procedure or a compile-time procedure, which calls treat alike.
#[derive(Clone, Copy)]
pub struct ProcLike<'m> {
    pub proc: Option<&'m ast::ProcedureDecl>,
    pub comptime_proc: Option<&'m ast::ComptimeProcedureDecl>,
}

impl<'m> ProcLike<'m> {
    pub fn is_comptime(&self) -> bool {
        self.proc.is_none() && self.comptime_proc.is_some()
    }

    fn pick<T: ?Sized>(
        &self,
        of_proc: impl FnOnce(&'m ast::ProcedureDecl) -> &'m T,
        of_comptime: impl FnOnce(&'m ast::ComptimeProcedureDecl) -> &'m T,
    ) -> &'m T {
        match (self.proc, self.comptime_proc) {
            (Some(proc), _) => of_proc(proc),
            (None, Some(proc)) => of_comptime(proc),
            (None, None) => unreachable!("a procedure lookup entry names a declaration"),
        }
    }

    pub fn attrs(&self) -> &'m [ast::AttributeItem] {
        self.pick(|proc| &proc.attrs[..], |proc| &proc.attrs[..])
    }

    pub fn generic_params(&self) -> &'m Option<ast::GenericParams> {
        self.pick(|proc| &proc.generic_params, |proc| &proc.generic_params)
    }

    pub fn params(&self) -> &'m [ast::Param] {
        self.pick(|proc| &proc.params[..], |proc| &proc.params[..])
    }

    pub fn return_type_opt(&self) -> &'m ast::TypePtr {
        self.pick(|proc| &proc.return_type_opt, |proc| &proc.return_type_opt)
    }

    pub fn contract(&self) -> &'m Option<ast::ContractClause> {
        self.pick(|proc| &proc.contract, |proc| &proc.contract)
    }

    /// The type parameters, if the procedure has any.
    fn type_params(&self) -> &'m [ast::TypeParam] {
        self.generic_params().as_ref().map_or(&[], |generics| &generics.params[..])
    }
}

/// The procedures of a module with the given name: those the compile-time pass left
/// behind first, then the declared ones in order.
fn find_procedures_in_module<'m>(module: &'m ast::ASTModule, name: &str) -> Vec<ProcLike<'m>> {
    let key = id_key_of(name);
    let mut out: Vec<ProcLike<'m>> = module
        .comptime_procedures
        .iter()
        .filter(|proc| id_key_of(&proc.name) == key)
        .map(|proc| ProcLike { proc: None, comptime_proc: Some(proc) })
        .collect();
    for item in &module.items {
        match item {
            ast::ASTItem::ProcedureDecl(proc) if id_key_of(&proc.name) == key => {
                out.push(ProcLike { proc: Some(proc), comptime_proc: None });
            }
            ast::ASTItem::ComptimeProcedureDecl(proc) if id_key_of(&proc.name) == key => {
                out.push(ProcLike { proc: None, comptime_proc: Some(proc) });
            }
            _ => {}
        }
    }
    out
}

fn find_procedure_in_module<'m>(module: &'m ast::ASTModule, name: &str) -> Option<ProcLike<'m>> {
    find_procedures_in_module(module, name).into_iter().next()
}

#[derive(Clone)]
pub struct CalleeNameResolution {
    pub origin: Vec<String>,
    pub name: String,
}

#[derive(Clone)]
pub struct CalleeProcedureLookupResult<'m> {
    pub proc: ProcLike<'m>,
    pub is_comptime_proc: bool,
    pub origin: Vec<String>,
    pub name: String,
}

fn is_comptime_typing_env(env: &TypeEnv) -> bool {
    ["diagnostics", "introspect", "emitter", "files", "target"].iter().any(|name| bind_of(env, name).is_some())
}

fn allows_comptime_procedure_call(type_ctx: &StmtTypeContext<'_>, env: &TypeEnv) -> bool {
    is_comptime_typing_env(env) || (type_ctx.require_pure && type_ctx.contract_phase != ContractPhase::None)
}

/// The module and name a callee refers to, when it is a name.
pub fn resolve_callee_procedure_name(ctx: &ScopeContext<'_>, callee: &ExprPtr) -> Option<CalleeNameResolution> {
    match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => {
            let (origin, name) = match resolve_value_name(ctx, &ident.name) {
                Some(entity) if entity.origin_opt.is_some() => {
                    (entity.origin_opt.unwrap_or_default(), entity.target_opt.unwrap_or_else(|| ident.name.clone()))
                }
                _ => (ctx.current_module.clone(), ident.name.clone()),
            };
            Some(CalleeNameResolution { origin, name })
        }
        ExprNode::QualifiedNameExpr(node) => Some(CalleeNameResolution { origin: node.path.clone(), name: node.name.clone() }),
        ExprNode::PathExpr(node) => Some(CalleeNameResolution {
            origin: if node.path.is_empty() { ctx.current_module.clone() } else { node.path.clone() },
            name: node.name.clone(),
        }),
        _ => None,
    }
}

fn lookup_procedure_for_resolved_callee<'c>(
    ctx: &'c ScopeContext<'_>,
    callee: &CalleeNameResolution,
) -> Option<CalleeProcedureLookupResult<'c>> {
    let module = find_module(ctx, &callee.origin)?;
    let proc = find_procedure_in_module(module, &callee.name)?;
    Some(CalleeProcedureLookupResult {
        proc,
        is_comptime_proc: proc.is_comptime(),
        origin: callee.origin.clone(),
        name: callee.name.clone(),
    })
}

pub fn lookup_procedure_for_callee<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr) -> Option<CalleeProcedureLookupResult<'c>> {
    lookup_procedure_for_resolved_callee(ctx, &resolve_callee_procedure_name(ctx, callee)?)
}

struct OverloadCandidateCheck {
    viable: bool,
    hard_error: bool,
    diag_id: Option<&'static str>,
    return_type: TypeRef,
    exact_matches: usize,
}

/// A failed check that names no rule, or only the general type mismatch.
fn is_plain_mismatch(diag_id: Option<&'static str>) -> bool {
    matches!(diag_id, None | Some("E-SEM-2526"))
}

fn check_free_procedure_overload_candidate(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    proc: ProcLike<'_>,
    args: &[Arg],
    env: &TypeEnv,
    type_expr: ExprTypeFn<'_>,
    check_expr: ArgCheckFn<'_>,
) -> OverloadCandidateCheck {
    let mut out =
        OverloadCandidateCheck { viable: false, hard_error: false, diag_id: None, return_type: None, exact_matches: 0 };
    if proc.is_comptime() && !allows_comptime_procedure_call(type_ctx, env) {
        out.hard_error = true;
        out.diag_id = Some("E-CTE-0034");
        return out;
    }
    if !proc.type_params().is_empty() {
        return out;
    }
    let proc_type = proc_type_of(ctx, proc.params(), proc.return_type_opt());
    if !proc_type.ok {
        out.hard_error = true;
        out.diag_id = proc_type.diag_id;
        return out;
    }
    let stripped = strip_perm(&proc_type.r#type);
    let Some(TypeNode::Func { params, ret }) = stripped.as_deref().map(|ty| &ty.node) else {
        return out;
    };
    if params.len() != args.len() {
        return out;
    }
    for (param, arg) in params.iter().zip(args) {
        if missing_required_move_for_consuming(param.mode, arg) {
            return out;
        }
        if param.mode.is_none() && arg.pass == ArgPassKind::Move {
            return out;
        }
        let arg_expr = if param.mode == Some(ParamMode::Move) || arg.pass == ArgPassKind::Copy {
            arg_pass_expr(arg)
        } else {
            arg.value.clone()
        };
        let checked = check_expr(&arg_expr, &param.r#type);
        if !checked.ok {
            if !is_plain_mismatch(checked.diag_id) {
                out.hard_error = true;
                out.diag_id = checked.diag_id;
            }
            return out;
        }
        let typed = type_expr(&arg_expr);
        if !typed.ok {
            out.hard_error = true;
            out.diag_id = typed.diag_id;
            return out;
        }
        if type_equiv(&strip_perm(&typed.r#type), &strip_perm(&param.r#type)) {
            out.exact_matches += 1;
        }
    }
    out.viable = true;
    out.return_type = ret.clone();
    out
}

struct FreeProcedureOverloadResolution<'m> {
    applies: bool,
    selected: Option<ProcLike<'m>>,
    return_type: TypeRef,
    diag_id: Option<&'static str>,
}

/// Picks among several procedures of one name: the viable one with the most exactly
/// matching arguments.
fn resolve_free_procedure_overload<'c>(
    ctx: &'c ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    resolved_callee: Option<&CalleeNameResolution>,
    args: &[Arg],
    env: &TypeEnv,
    type_expr: ExprTypeFn<'_>,
    check_expr: ArgCheckFn<'_>,
) -> FreeProcedureOverloadResolution<'c> {
    let mut out = FreeProcedureOverloadResolution { applies: false, selected: None, return_type: None, diag_id: None };
    let candidates = resolved_callee
        .and_then(|callee| find_module(ctx, &callee.origin).map(|module| find_procedures_in_module(module, &callee.name)))
        .unwrap_or_default();
    if candidates.len() <= 1 {
        return out;
    }
    out.applies = true;
    let mut viable: Vec<(ProcLike<'c>, TypeRef, usize)> = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let checked = check_free_procedure_overload_candidate(ctx, type_ctx, candidate, args, env, type_expr, check_expr);
        if checked.hard_error {
            out.diag_id = checked.diag_id;
            return out;
        }
        if checked.viable {
            viable.push((candidate, checked.return_type, checked.exact_matches));
        }
    }
    if viable.is_empty() {
        out.diag_id = Some("E-SEM-3031");
        return out;
    }
    let best_exact = viable.iter().map(|candidate| candidate.2).max().unwrap_or(0);
    viable.retain(|candidate| candidate.2 == best_exact);
    if viable.iter().any(|candidate| candidate.0.type_params().is_empty()) {
        viable.retain(|candidate| candidate.0.type_params().is_empty());
    }
    if viable.len() != 1 {
        out.diag_id = Some("E-SEM-3030");
        return out;
    }
    let (selected, return_type, _) = viable.remove(0);
    out.selected = Some(selected);
    out.return_type = return_type;
    out
}

fn type_is_shared_param_surface(ty: &ast::TypePtr) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(ast::TypeNode::TypePermType(node)) => node.perm == ast::TypePerm::Shared,
        Some(ast::TypeNode::TypeRefine(node)) => type_is_shared_param_surface(&node.base),
        _ => false,
    }
}

fn is_unique_param_surface(ty: &ast::TypePtr) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(ast::TypeNode::TypePermType(node)) => node.perm == ast::TypePerm::Unique,
        Some(ast::TypeNode::TypeRefine(node)) => is_unique_param_surface(&node.base),
        _ => false,
    }
}

/// A shared argument to a `unique` parameter under a held key needs the key for
/// writing.
fn check_shared_arg_write_requirement(
    type_ctx: &StmtTypeContext<'_>,
    params: &[ast::Param],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
) -> Option<&'static str> {
    for (param, arg) in params.iter().zip(args) {
        if !is_unique_param_surface(&param.r#type) {
            continue;
        }
        let arg_expr = if arg.pass != ArgPassKind::Ref { arg_pass_expr(arg) } else { arg.value.clone() };
        let arg_typed = type_expr(&arg_expr);
        if !arg_typed.ok || arg_typed.r#type.is_none() || perm_of_type(&arg_typed.r#type) != Permission::Shared {
            continue;
        }
        let built = build_key_path(&arg_expr);
        if !built.success {
            continue;
        }
        let mut covering: Option<ast::KeyMode> = None;
        for held in &type_ctx.held_key_paths {
            if !is_prefix(&held.path, &built.path) {
                continue;
            }
            if covering.is_none() || held.mode == ast::KeyMode::Write {
                covering = Some(held.mode);
            }
        }
        if covering.is_some_and(|mode| mode != ast::KeyMode::Write) {
            return Some("E-CON-0005");
        }
    }
    None
}

/// A call under held keys of a procedure whose key accesses are unknown warns when it
/// is handed shared data the keys cover. The summary of a procedure's accesses is not
/// ported, so a call that could warn is left pending.
fn emit_unknown_callee_access_warning_if_needed(
    type_ctx: &StmtTypeContext<'_>,
    node: &ast::CallExpr,
    lookup: Option<&CalleeProcedureLookupResult<'_>>,
) {
    if !type_ctx.keys_held || type_ctx.diags.is_none() || type_ctx.held_key_paths.is_empty() {
        return;
    }
    let Some(proc) = lookup.and_then(|lookup| lookup.proc.proc) else {
        return;
    };
    let could_warn = node.args.iter().zip(&proc.params).any(|(arg, param)| {
        if !type_is_shared_param_surface(&param.r#type) {
            return false;
        }
        let built = build_key_path(&arg.value);
        built.success && type_ctx.held_key_paths.iter().any(|held| is_prefix(&held.path, &built.path))
    });
    if could_warn {
        pending("CalleeKeyAccessSummary");
    }
}

/// A reference to a declaration marked `[[deprecated]]` warns, with the message if any.
pub fn emit_deprecated_reference_warning_from_attrs(
    attrs_list: &[ast::AttributeItem],
    type_ctx: &StmtTypeContext<'_>,
    span: Option<Span>,
) {
    let Some(diags) = &type_ctx.diags else {
        return;
    };
    if !has_attribute(attrs_list, attrs::DEPRECATED) {
        return;
    }
    let Some(mut diag) = make_diagnostic_by_id("W-CNF-0601", span) else {
        return;
    };
    if let Some(message) = normalize_deprecated_message(attrs_list).filter(|message| !message.is_empty()) {
        diag.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::Note,
            message: format!("deprecated message: {message}"),
            span: None,
            fix_text: None,
            label: None,
        });
    }
    emit(&mut diags.borrow_mut(), diag);
}

fn required_type_arg_count(params: &[ast::TypeParam]) -> usize {
    params.iter().filter(|param| param.default_type.is_none()).count()
}

fn is_type_param_name(params: &[ast::TypeParam], name: &str) -> bool {
    params.iter().any(|param| id_eq(&param.name, name))
}

fn type_path_eq_local(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(l, r)| id_eq(l, r))
}

fn type_param_of<'t>(params: &[ast::TypeParam], ty: &'t Type) -> Option<&'t String> {
    match &ty.node {
        TypeNode::Path { path, .. } if path.len() == 1 && is_type_param_name(params, &path[0]) => Some(&path[0]),
        _ => None,
    }
}

fn contains_type_param_for_call(params: &[ast::TypeParam], ty: &TypeRef) -> bool {
    let Some(t) = ty.as_deref() else {
        return false;
    };
    if type_param_of(params, t).is_some() {
        return true;
    }
    let any = |types: &[TypeRef]| types.iter().any(|inner| contains_type_param_for_call(params, inner));
    match &t.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => contains_type_param_for_call(params, base),
        TypeNode::Tuple(elements) => any(elements),
        TypeNode::Array { element, .. } | TypeNode::Slice(element) => contains_type_param_for_call(params, element),
        TypeNode::Ptr { element, .. } | TypeNode::RawPtr { element, .. } => contains_type_param_for_call(params, element),
        TypeNode::Union(members) => any(members),
        TypeNode::Func { params: func_params, ret } => {
            func_params.iter().any(|param| contains_type_param_for_call(params, &param.r#type))
                || contains_type_param_for_call(params, ret)
        }
        TypeNode::Path { generic_args, .. } => any(generic_args),
        TypeNode::Apply { args, .. } => any(args),
        TypeNode::ModalState(modal) => any(&modal.generic_args),
        TypeNode::Closure { params: closure_params, ret, .. } => {
            closure_params.iter().any(|param| contains_type_param_for_call(params, &param.1))
                || contains_type_param_for_call(params, ret)
        }
        _ => false,
    }
}

/// Matches a parameter type against an argument type, binding the type parameters the
/// parameter type names. A parameter met twice must be bound to equivalent types.
fn bind_type_params_for_call(
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
    match (&e.node, &a.node) {
        (TypeNode::Perm { perm, base }, TypeNode::Perm { perm: other_perm, base: other }) => perm == other_perm && bind_type_params_for_call(params, base, other, bindings),
        (TypeNode::Tuple(elements), TypeNode::Tuple(other)) => bind_all_for_call(params, elements, other, bindings),
        (TypeNode::Array { element, length, .. }, TypeNode::Array { element: other, length: other_length, .. }) => {
            length == other_length && bind_type_params_for_call(params, element, other, bindings)
        }
        (TypeNode::Slice(element), TypeNode::Slice(other)) => bind_type_params_for_call(params, element, other, bindings),
        (TypeNode::Ptr { element, state }, TypeNode::Ptr { element: other, state: other_state }) => {
            state == other_state && bind_type_params_for_call(params, element, other, bindings)
        }
        (TypeNode::RawPtr { qual, element }, TypeNode::RawPtr { qual: other_qual, element: other }) => {
            qual == other_qual && bind_type_params_for_call(params, element, other, bindings)
        }
        (TypeNode::Union(members), TypeNode::Union(other)) => bind_all_for_call(params, members, other, bindings),
        (TypeNode::Func { params: func_params, ret }, TypeNode::Func { params: other_params, ret: other_ret }) => {
            func_params.len() == other_params.len()
                && func_params.iter().zip(other_params).all(|(param, other)| {
                    param.mode == other.mode && bind_type_params_for_call(params, &param.r#type, &other.r#type, bindings)
                })
                && bind_type_params_for_call(params, ret, other_ret, bindings)
        }
        (TypeNode::Path { path, generic_args: args }, _) | (TypeNode::Apply { path, args }, _) => {
            match (applied_type_path(a), applied_type_args(a)) {
                (Some(other_path), Some(other_args)) => type_path_eq_local(path, other_path) && bind_all_for_call(params, args, other_args, bindings),
                _ => false,
            }
        }
        (TypeNode::ModalState(modal), TypeNode::ModalState(other)) => {
            type_path_eq_local(&modal.path, &other.path)
                && id_eq(&modal.state, &other.state)
                && bind_all_for_call(params, &modal.generic_args, &other.generic_args, bindings)
        }
        (TypeNode::Dynamic(path), TypeNode::Dynamic(other)) => type_path_eq_local(path, other),
        (TypeNode::Refine { base, .. }, TypeNode::Refine { base: other, .. }) => bind_type_params_for_call(params, base, other, bindings),
        (TypeNode::Closure { params: closure_params, ret, .. }, TypeNode::Closure { params: other_params, ret: other_ret, .. }) => {
            closure_params.len() == other_params.len()
                && closure_params
                    .iter()
                    .zip(other_params)
                    .all(|(param, other)| param.0 == other.0 && bind_type_params_for_call(params, &param.1, &other.1, bindings))
                && bind_type_params_for_call(params, ret, other_ret, bindings)
        }
        (TypeNode::Opaque { class_path, .. }, TypeNode::Opaque { class_path: other, .. }) => type_path_eq_local(class_path, other),
        (TypeNode::String(state), TypeNode::String(other)) => state == other,
        (TypeNode::Bytes(state), TypeNode::Bytes(other)) => state == other,
        (TypeNode::Prim(name), TypeNode::Prim(other)) => id_eq(name, other),
        (TypeNode::Range(base), TypeNode::Range(other))
        | (TypeNode::RangeInclusive(base), TypeNode::RangeInclusive(other))
        | (TypeNode::RangeFrom(base), TypeNode::RangeFrom(other))
        | (TypeNode::RangeTo(base), TypeNode::RangeTo(other))
        | (TypeNode::RangeToInclusive(base), TypeNode::RangeToInclusive(other)) => bind_type_params_for_call(params, base, other, bindings),
        (TypeNode::RangeFull, TypeNode::RangeFull) => true,
        (TypeNode::Var(_), _) => type_equiv(expected, actual),
        _ => false,
    }
}

fn bind_all_for_call(
    params: &[ast::TypeParam],
    expected: &[TypeRef],
    actual: &[TypeRef],
    bindings: &mut BTreeMap<String, TypeRef>,
) -> bool {
    expected.len() == actual.len() && expected.iter().zip(actual).all(|(e, a)| bind_type_params_for_call(params, e, a, bindings))
}

/// The value of a defaulted type parameter, given the arguments before it.
fn default_type_arg(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    index: usize,
    args_so_far: &[TypeRef],
) -> Result<TypeRef, Option<&'static str>> {
    let value = lower_type(ctx, &params[index].default_type)?;
    if index == 0 {
        return Ok(value);
    }
    Ok(instantiate_type(&value, &build_substitution(&params[..index], &args_so_far[..index])))
}

fn expand_type_args_with_defaults(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    provided_args: Vec<TypeRef>,
) -> Result<Vec<TypeRef>, Option<&'static str>> {
    let mut out_args = provided_args;
    if out_args.len() > params.len() {
        return Err(Some("E-TYP-2303"));
    }
    for index in out_args.len()..params.len() {
        if params[index].default_type.is_none() {
            return Err(Some("E-SEM-2533"));
        }
        let value = default_type_arg(ctx, params, index, &out_args)?;
        out_args.push(value);
    }
    Ok(out_args)
}

fn validate_procedure_type_arg_constraints(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    subst: &TypeSubst,
) -> Option<&'static str> {
    for param in params {
        let Some(arg) = subst.get(&param.name).filter(|arg| arg.is_some()) else {
            return Some("E-TYP-2302");
        };
        for bound in &param.bounds {
            if !is_capability_class_path(&bound.class_path) && !ctx.sigma.classes.contains_key(&path_key_of(&bound.class_path)) {
                return Some("E-TYP-2305");
            }
            if !type_implements_class(ctx, arg, &bound.class_path) {
                return Some("E-TYP-2302");
            }
        }
    }
    None
}

#[derive(Default)]
pub struct GenericCallSubstResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub diag_span: Option<Span>,
    pub subst: TypeSubst,
}

fn subst_failure(diag_id: Option<&'static str>, diag_span: Option<Span>) -> GenericCallSubstResult {
    GenericCallSubstResult { diag_id, diag_span, ..Default::default() }
}

/// Why the argument types of a call could not be collected: the rule and where.
type ArgCollectionError = (Option<&'static str>, Option<Span>);

/// The types of the arguments as inference sees them, with the lowered parameter types.
fn collect_call_arg_types_for_inference(
    ctx: &ScopeContext<'_>,
    params: &[ast::Param],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
) -> Result<(Vec<TypeRef>, Vec<TypeRef>), ArgCollectionError> {
    if params.len() != args.len() {
        return Err((Some("E-SEM-2532"), None));
    }
    let mut lowered_params: Vec<TypeFuncParam> = Vec::with_capacity(params.len());
    for param in params {
        let lowered = lower_type(ctx, &param.r#type).map_err(|diag_id| (diag_id, None))?;
        lowered_params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: lowered });
    }
    for (param, arg) in lowered_params.iter().zip(args) {
        if missing_required_move_for_consuming(param.mode, arg) {
            return Err((Some("E-SEM-2534"), arg_diagnostic_span(arg)));
        }
        if param.mode.is_none() && arg.pass == ArgPassKind::Move {
            return Err((Some("E-SEM-2535"), arg_diagnostic_span(arg)));
        }
    }
    let typed = |expr: &ExprPtr, arg: &Arg| {
        let typed = type_expr(expr);
        if typed.ok {
            Ok(typed.r#type)
        } else {
            Err((typed.diag_id, typed.diag_span.or_else(|| arg_diagnostic_span(arg))))
        }
    };
    let mut actual: Vec<TypeRef> = Vec::with_capacity(args.len());
    for (param, arg) in lowered_params.iter().zip(args) {
        if param.mode.is_some() || arg.pass == ArgPassKind::Copy {
            actual.push(typed(&arg_pass_expr(arg), arg)?);
            continue;
        }
        let by_reference = has_source_provenance(&arg.value) && !is_function_value_type(ctx, &param.r#type, 0);
        if by_reference && !is_place_expr_for_call(&arg.value) {
            return Err((Some("E-TYP-1603"), arg_diagnostic_span(arg)));
        }
        match type_place.filter(|_| by_reference) {
            Some(type_place) => {
                let place = type_place(&arg.value);
                if !place.ok {
                    return Err((place.diag_id, arg_diagnostic_span(arg)));
                }
                actual.push(place.r#type);
            }
            None => actual.push(typed(&arg.value, arg)?),
        }
    }
    Ok((actual, lowered_params.into_iter().map(|param| param.r#type).collect()))
}

/// Infers the type arguments of a call of a generic procedure from its arguments and,
/// when known, the type the call is expected to have.
fn infer_generic_call_subst_for_proc(
    ctx: &ScopeContext<'_>,
    proc: ProcLike<'_>,
    args: &[Arg],
    expected_return: &TypeRef,
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
) -> GenericCallSubstResult {
    let type_params = proc.type_params();
    if type_params.is_empty() {
        return GenericCallSubstResult::default();
    }
    let (actual_arg_types, expected_param_types) =
        match collect_call_arg_types_for_inference(ctx, proc.params(), args, type_expr, type_place) {
            Ok(collected) => collected,
            Err((diag_id, diag_span)) => return subst_failure(diag_id, diag_span),
        };
    let mut bindings: BTreeMap<String, TypeRef> = BTreeMap::new();
    for (index, (expected, actual)) in expected_param_types.iter().zip(&actual_arg_types).enumerate() {
        if !bind_type_params_for_call(type_params, expected, actual, &mut bindings) {
            return subst_failure(Some("E-SEM-2533"), arg_diagnostic_span(&args[index]));
        }
    }
    if expected_return.is_some() && proc.return_type_opt().is_some() {
        let lowered_return = match lower_type(ctx, proc.return_type_opt()) {
            Ok(lowered) => lowered,
            Err(diag_id) => return subst_failure(diag_id, None),
        };
        let contains_type_param = contains_type_param_for_call(type_params, &lowered_return);
        let matched = bind_type_params_for_call(type_params, &lowered_return, expected_return, &mut bindings);
        if contains_type_param && !matched {
            return subst_failure(Some("E-SEM-2533"), None);
        }
    }
    let mut inferred_args: Vec<TypeRef> = Vec::with_capacity(type_params.len());
    for (index, param) in type_params.iter().enumerate() {
        if let Some(bound) = bindings.get(&param.name) {
            inferred_args.push(bound.clone());
            continue;
        }
        if param.default_type.is_none() {
            return subst_failure(Some("E-TYP-2301"), None);
        }
        match default_type_arg(ctx, type_params, index, &inferred_args) {
            Ok(value) => inferred_args.push(value),
            Err(diag_id) => return subst_failure(diag_id, None),
        }
    }
    let subst = build_substitution(type_params, &inferred_args);
    if let Some(diag_id) = validate_procedure_type_arg_constraints(ctx, type_params, &subst) {
        return GenericCallSubstResult { diag_id: Some(diag_id), subst, ..Default::default() };
    }
    GenericCallSubstResult { ok: true, subst, ..Default::default() }
}

pub fn infer_generic_call_subst(
    ctx: &ScopeContext<'_>,
    callee: &ExprPtr,
    args: &[Arg],
    expected_return: &TypeRef,
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
) -> GenericCallSubstResult {
    match lookup_procedure_for_callee(ctx, callee) {
        Some(lookup) if !lookup.proc.type_params().is_empty() => {
            infer_generic_call_subst_for_proc(ctx, lookup.proc, args, expected_return, type_expr, type_place)
        }
        _ => GenericCallSubstResult::default(),
    }
}

/// The substitution of a call that writes its type arguments.
fn build_generic_call_subst_checked(
    ctx: &ScopeContext<'_>,
    lookup: Option<&CalleeProcedureLookupResult<'_>>,
    generic_args: &[ast::TypePtr],
) -> Result<TypeSubst, Option<&'static str>> {
    let Some(generics) = lookup.and_then(|lookup| lookup.proc.generic_params().as_ref()) else {
        return Err(Some("E-SEM-2533"));
    };
    let params = &generics.params[..];
    if generic_args.len() < required_type_arg_count(params) || generic_args.len() > params.len() {
        return Err(Some("E-TYP-2303"));
    }
    let provided = generic_args.iter().map(|arg| lower_type(ctx, arg)).collect::<Result<Vec<_>, _>>()?;
    let expanded = expand_type_args_with_defaults(ctx, params, provided)?;
    let subst = build_substitution(params, &expanded);
    match validate_procedure_type_arg_constraints(ctx, params, &subst) {
        Some(diag_id) => Err(Some(diag_id)),
        None => Ok(subst),
    }
}

fn generic_arg_count_mismatch(lookup: Option<&CalleeProcedureLookupResult<'_>>, arg_count: usize) -> bool {
    let Some(generics) = lookup.and_then(|lookup| lookup.proc.generic_params().as_ref()) else {
        return false;
    };
    arg_count < required_type_arg_count(&generics.params) || arg_count > generics.params.len()
}

struct ExternProcLookupResult<'m> {
    module: &'m ast::ASTModule,
    proc: &'m ast::ExternProcDecl,
}

struct ExternCalleeQuery {
    module_path: Vec<String>,
    candidate_names: Vec<String>,
}

fn find_extern_in_module<'m>(module: &'m ast::ASTModule, name: &str) -> Option<&'m ast::ExternProcDecl> {
    crate::memory::calls::find_extern_procedure_in_module(module, name)
}

fn build_extern_callee_query(
    ctx: &ScopeContext<'_>,
    callee: &ExprPtr,
    resolved_callee: Option<&CalleeNameResolution>,
) -> Option<ExternCalleeQuery> {
    Some(match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => {
            let mut candidate_names = vec![ident.name.clone()];
            let module_path = match resolved_callee {
                Some(resolved) => {
                    if !id_eq(&resolved.name, &ident.name) {
                        candidate_names.push(resolved.name.clone());
                    }
                    resolved.origin.clone()
                }
                None => ctx.current_module.clone(),
            };
            ExternCalleeQuery { module_path, candidate_names }
        }
        ExprNode::QualifiedNameExpr(node) => ExternCalleeQuery { module_path: node.path.clone(), candidate_names: vec![node.name.clone()] },
        ExprNode::QualifiedApplyExpr(node) => ExternCalleeQuery { module_path: node.path.clone(), candidate_names: vec![node.name.clone()] },
        ExprNode::PathExpr(node) => ExternCalleeQuery {
            module_path: if node.path.is_empty() { ctx.current_module.clone() } else { node.path.clone() },
            candidate_names: vec![node.name.clone()],
        },
        _ => return None,
    })
}

/// The foreign procedure a callee names: one in its module, unless an ordinary
/// procedure there has the name, and failing both the first in any module.
fn lookup_extern_procedure_for_query<'c>(ctx: &'c ScopeContext<'_>, query: &ExternCalleeQuery) -> Option<ExternProcLookupResult<'c>> {
    let find_in_module = |module: &'c ast::ASTModule| {
        query.candidate_names.iter().find_map(|name| find_extern_in_module(module, name)).map(|proc| ExternProcLookupResult { module, proc })
    };
    let find_anywhere = || ctx.sigma.mods.iter().find_map(find_in_module);
    let Some(module) = find_module(ctx, &query.module_path) else {
        return find_anywhere();
    };
    if let Some(found) = find_in_module(module) {
        return Some(found);
    }
    if query.candidate_names.iter().any(|name| find_procedure_in_module(module, name).is_some()) {
        return None;
    }
    find_anywhere()
}

fn binding_for_ffi_boundary_expr<'e>(env: &'e TypeEnv, expr: &ExprPtr) -> Option<&'e TypeBinding> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => bind_of(env, &node.name),
        ExprNode::FieldAccessExpr(node) => binding_for_ffi_boundary_expr(env, &node.base),
        ExprNode::TupleAccessExpr(node) => binding_for_ffi_boundary_expr(env, &node.base),
        ExprNode::IndexAccessExpr(node) => binding_for_ffi_boundary_expr(env, &node.base),
        ExprNode::DerefExpr(node) => binding_for_ffi_boundary_expr(env, &node.value),
        ExprNode::MoveExpr(node) => binding_for_ffi_boundary_expr(env, &node.place),
        ExprNode::AttributedExpr(node) => binding_for_ffi_boundary_expr(env, &node.expr),
        _ => None,
    }
}

/// A raw pointer into a region may not be handed to a foreign procedure.
fn check_ffi_boundary_region_local_raw_pointer_args(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    call: &ast::CallExpr,
    env: &TypeEnv,
    callee_is_extern: bool,
) -> Option<&'static str> {
    if !callee_is_extern {
        return None;
    }
    for arg in call.args.iter().filter(|arg| arg.value.is_some()) {
        let typed = type_expr(ctx, type_ctx, &arg.value, env);
        if !typed.ok {
            return typed.diag_id;
        }
        if !matches!(strip_perm_and_refine(&typed.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::RawPtr { .. })) {
            continue;
        }
        if binding_for_ffi_boundary_expr(env, &arg.value)
            .is_some_and(|binding| binding.provenance_kind == BindingProvenanceSeedKind::Region)
        {
            return Some("E-SYS-3360");
        }
        let prov = track_expr_provenance(ctx, &arg.value, env);
        if !prov.ok {
            return prov.diag_id;
        }
        if prov.kind == ProvenanceKind::Region {
            return Some("E-SYS-3360");
        }
    }
    None
}

fn extract_direct_callee_name(callee: &ExprPtr) -> Option<&str> {
    match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(&node.name),
        ExprNode::PathExpr(node) => Some(&node.name),
        ExprNode::QualifiedNameExpr(node) => Some(&node.name),
        ExprNode::QualifiedApplyExpr(node) => Some(&node.name),
        _ => None,
    }
}

fn is_gpu_barrier_name(name: &str) -> bool {
    matches!(name, "gpu_barrier" | "gpu_memory_barrier" | "gpu_workgroup_barrier")
}

fn params_pure(params: &[TypeFuncParam]) -> bool {
    !params.iter().any(|param| is_capability_type(&param.r#type))
}

fn same_expr(lhs: &ExprPtr, rhs: &ExprPtr) -> bool {
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => Arc::ptr_eq(lhs, rhs),
        (None, None) => true,
        _ => false,
    }
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

struct RecordCallee<'c> {
    record: &'c ast::RecordDecl,
    path: Vec<String>,
}

fn resolve_record_callee<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr, args: &[Arg]) -> Option<RecordCallee<'c>> {
    if !args.is_empty() {
        return None;
    }
    let full = match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => {
            if let Some(full) = lookup_builtin_record_ctor_path(&node.name) {
                if let Some(record) = lookup_record_decl(ctx, &full) {
                    return Some(RecordCallee { record, path: full });
                }
            }
            let entity = resolve_type_name(ctx, &node.name)?;
            let mut full = entity.origin_opt.unwrap_or_default();
            full.push(entity.target_opt.unwrap_or_else(|| node.name.clone()));
            full
        }
        ExprNode::PathExpr(node) => [&node.path[..], std::slice::from_ref(&node.name)].concat(),
        _ => return None,
    };
    lookup_record_decl(ctx, &full).map(|record| RecordCallee { record, path: full })
}

/// A record named without arguments constructs it from its field defaults. The result
/// is neither ok nor a diagnostic when the callee is not such a record.
fn type_record_default_call(ctx: &ScopeContext<'_>, callee: &ExprPtr, args: &[Arg], type_expr: ExprTypeFn<'_>) -> ExprTypeResult {
    let Some(found) = resolve_record_callee(ctx, callee, args) else {
        return ExprTypeResult::default();
    };
    let fields = record_fields(found.record);
    let mut seen = HashSet::with_capacity(fields.len());
    if fields.iter().any(|field| !seen.insert(id_key_of(&field.name))) {
        return ExprTypeResult::failed(Some("E-TYP-1901"));
    }
    for field in fields.iter().filter(|field| field.init_opt.is_some()) {
        let init_type = type_expr(&field.init_opt);
        if !init_type.ok {
            return ExprTypeResult::failed(init_type.diag_id);
        }
        let field_type = match lower_type(ctx, &field.r#type) {
            Ok(lowered) => lowered,
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        };
        let sub = subtyping(ctx, &init_type.r#type, &field_type);
        if !sub.ok {
            return ExprTypeResult::failed(sub.diag_id);
        }
        if !sub.subtype {
            return ExprTypeResult::default();
        }
    }
    if fields.iter().any(|field| field.init_opt.is_none()) {
        return ExprTypeResult::failed(Some("Record-Default-Init-Err"));
    }
    ExprTypeResult::typed(make_type_path(found.path))
}

fn call_failure(call: CallTypeResult) -> ExprTypeResult {
    ExprTypeResult { diag_id: call.diag_id, diag_detail: call.diag_detail, diag_span: call.diag_span, ..Default::default() }
}

/// Types a call expression.
pub fn type_call_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, node: &ast::CallExpr, env: &TypeEnv) -> ExprTypeResult {
    let failed = |diag_id: &'static str| ExprTypeResult::failed(Some(diag_id));
    let callee_span = || node.callee.as_deref().map(|callee| callee.span.clone()).unwrap_or_default();

    let resolved_callee = resolve_callee_procedure_name(ctx, &node.callee);
    let callee_proc = resolved_callee.as_ref().and_then(|resolved| lookup_procedure_for_resolved_callee(ctx, resolved));
    let comptime_out_of_place =
        callee_proc.as_ref().is_some_and(|lookup| lookup.is_comptime_proc) && !allows_comptime_procedure_call(type_ctx, env);

    // A callee that names a procedure rather than a local binding has the procedure's
    // type; anything else is typed as an expression.
    let callee_value_type = || -> Option<ExprTypeResult> {
        let callee = node.callee.as_deref()?;
        let (path, name, _rule) = match &callee.node {
            ExprNode::IdentifierExpr(ident) => {
                if bind_of(env, &ident.name).is_some() {
                    return None;
                }
                (&ctx.current_module[..], &ident.name, "T-Ident")
            }
            ExprNode::PathExpr(path) => (&path.path[..], &path.name, "T-Path-Value"),
            _ => return None,
        };
        if comptime_out_of_place {
            return Some(failed("E-CTE-0034"));
        }
        let value_type = value_path_type(ctx, path, name);
        if !value_type.ok {
            return Some(ExprTypeResult::failed(value_type.diag_id));
        }
        if value_type.r#type.is_none() {
            return Some(ExprTypeResult {
                diag_id: Some("ResolveExpr-Ident-Err"),
                diag_detail: format!("identifier '{name}'"),
                ..Default::default()
            });
        }
        Some(ExprTypeResult::typed(value_type.r#type))
    };
    let callee_type: OnceCell<ExprTypeResult> = OnceCell::new();
    let callee_type_for_call =
        || callee_type.get_or_init(|| callee_value_type().unwrap_or_else(|| type_expr(ctx, type_ctx, &node.callee, env))).clone();
    let type_expr_fn = |inner: &ExprPtr| {
        if same_expr(inner, &node.callee) {
            return callee_type_for_call();
        }
        type_expr(ctx, type_ctx, inner, env)
    };

    // Built-in records are constructed by bare name.
    if let (Some(callee), true) = (node.callee.as_deref(), node.args.is_empty()) {
        if let ExprNode::IdentifierExpr(ident) = &callee.node {
            if let Some(builtin_path) = lookup_builtin_record_ctor_path(&ident.name) {
                if path_matches_builtin_name(&builtin_path, "System") && !is_in_unsafe_span(ctx, &callee.span) {
                    return failed("E-CON-0020");
                }
                return ExprTypeResult::typed(make_type_path(builtin_path));
            }
        }
    }

    let record = type_record_default_call(ctx, &node.callee, &node.args, &type_expr_fn);
    if record.ok || record.diag_id.is_some() {
        return record;
    }

    let extern_lookup = build_extern_callee_query(ctx, &node.callee, resolved_callee.as_ref())
        .and_then(|query| lookup_extern_procedure_for_query(ctx, &query));
    let callee_is_extern = extern_lookup.is_some();
    if callee_is_extern && !is_in_unsafe_span(ctx, &callee_span()) {
        return failed("E-TYP-2106");
    }

    if let Some(callee_name) = extract_direct_callee_name(&node.callee).filter(|name| is_gpu_intrinsic_name(name)) {
        if !gpu_context(env) {
            return failed(if is_gpu_barrier_name(callee_name) { "E-CON-0156" } else { "E-CON-0154" });
        }
    }

    let type_place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let check_expr_fn = |inner: &ExprPtr, expected: &TypeRef| -> CheckResult {
        let arg_ctx = arg_ctx_for(type_ctx, expected);
        let checked = check_expr_against(ctx, &arg_ctx, inner, expected, env);
        if !checked.ok && expected.is_some() {
            // A bare argument introduces into an `Outcome` parameter when it can only
            // be its value or only its error.
            let typed = type_expr(ctx, &arg_ctx, inner, env);
            if typed.ok {
                match classify_outcome_intro(ctx, &typed.r#type, expected) {
                    OutcomeIntro::Ambiguous => return CheckResult { diag_id: Some("E-TYP-2261"), ..Default::default() },
                    OutcomeIntro::Value | OutcomeIntro::Error => return CheckResult { ok: true, ..Default::default() },
                    OutcomeIntro::None => {}
                }
            }
        }
        CheckResult {
            ok: checked.ok,
            diag_id: checked.diag_id,
            diag_detail: checked.diag_detail,
            diag_span: checked.diag_span,
            ..Default::default()
        }
    };

    if comptime_out_of_place {
        return failed("E-CTE-0034");
    }

    // The checks every call ends with, once its arguments are typed.
    let callee_params_pure = || {
        let callee = type_expr_fn(&node.callee);
        !callee.ok
            || !matches!(strip_perm(&callee.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Func { params, .. }) if !params_pure(params))
    };
    let foreign_checks = || -> Option<&'static str> {
        if let Some(found) = &extern_lookup {
            if let Some(diag_id) = check_foreign_static_assumes(ctx, type_ctx, node, found.module, found.proc) {
                return Some(diag_id);
            }
        }
        check_ffi_boundary_region_local_raw_pointer_args(ctx, type_ctx, node, env, callee_is_extern)
    };
    let finish = |call_type: TypeRef| -> ExprTypeResult {
        if type_ctx.require_pure && !callee_params_pure() {
            return failed("E-SEM-2802");
        }
        if let Some(diag_id) = check_call_site_precondition(ctx, type_ctx, node, callee_proc.as_ref().map(|lookup| lookup.proc)) {
            return failed(diag_id);
        }
        if let Some(lookup) = &callee_proc {
            if let Some(diag_id) = check_shared_arg_write_requirement(type_ctx, lookup.proc.params(), &node.args, &type_expr_fn) {
                return failed(diag_id);
            }
        }
        if let Some(diag_id) = foreign_checks() {
            return failed(diag_id);
        }
        emit_unknown_callee_access_warning_if_needed(type_ctx, node, callee_proc.as_ref());
        if let Some(lookup) = &callee_proc {
            emit_deprecated_reference_warning_from_attrs(lookup.proc.attrs(), type_ctx, node.callee.as_deref().map(|callee| callee.span.clone()));
        }
        ExprTypeResult::typed(call_type)
    };
    let callee_facts = || CallCalleeFacts { extern_callee: Some(callee_is_extern), callee_type: callee_type.get().cloned() };

    if node.generic_args.is_empty() {
        let overload =
            resolve_free_procedure_overload(ctx, type_ctx, resolved_callee.as_ref(), &node.args, env, &type_expr_fn, &check_expr_fn);
        if overload.applies {
            if let Some(diag_id) = overload.diag_id {
                return failed(diag_id);
            }
            let Some(selected) = overload.selected else {
                return failed("E-SEM-3031");
            };
            if type_ctx.require_pure {
                let selected_type = proc_type_of(ctx, selected.params(), selected.return_type_opt());
                let stripped = if selected_type.ok { strip_perm(&selected_type.r#type) } else { None };
                if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Func { params, .. }) if !params_pure(params)) {
                    return failed("E-SEM-2802");
                }
            }
            if let Some(diag_id) = check_shared_arg_write_requirement(type_ctx, selected.params(), &node.args, &type_expr_fn) {
                return failed(diag_id);
            }
            if let Some(diag_id) = foreign_checks() {
                return failed(diag_id);
            }
            return ExprTypeResult::typed(overload.return_type);
        }
    }

    // A generic call that writes its type arguments.
    if !node.generic_args.is_empty() {
        let subst = match build_generic_call_subst_checked(ctx, callee_proc.as_ref(), &node.generic_args) {
            Ok(subst) => subst,
            Err(diag_id) => {
                if diag_id == Some("E-TYP-2303") || generic_arg_count_mismatch(callee_proc.as_ref(), node.generic_args.len()) {
                    return failed("E-TYP-2303");
                }
                return ExprTypeResult::failed(diag_id);
            }
        };
        let call = type_call_with_subst(
            ctx,
            &node.callee,
            &node.args,
            &subst,
            &type_expr_fn,
            Some(&type_place_fn),
            Some(&check_expr_fn),
            Some(&callee_facts()),
        );
        if !call.ok {
            return call_failure(call);
        }
        return finish(call.r#type);
    }

    // A generic call whose type arguments follow from its arguments.
    if let Some(lookup) = callee_proc.as_ref().filter(|lookup| !lookup.proc.type_params().is_empty()) {
        let inferred = infer_generic_call_subst_for_proc(ctx, lookup.proc, &node.args, &None, &type_expr_fn, Some(&type_place_fn));
        if !inferred.ok {
            return ExprTypeResult {
                diag_id: Some(inferred.diag_id.unwrap_or("E-SEM-2533")),
                diag_span: inferred.diag_span,
                ..Default::default()
            };
        }
        let call = type_call_with_subst(
            ctx,
            &node.callee,
            &node.args,
            &inferred.subst,
            &type_expr_fn,
            Some(&type_place_fn),
            Some(&check_expr_fn),
            Some(&callee_facts()),
        );
        if !call.ok {
            return call_failure(call);
        }
        return finish(call.r#type);
    }

    let call = type_call(ctx, &node.callee, &node.args, &type_expr_fn, Some(&type_place_fn), Some(&check_expr_fn), Some(&callee_facts()));
    if !call.ok {
        return call_failure(call);
    }
    finish(call.r#type)
}

/// The substitution of a call written `callee::<T>(args)`.
fn build_call_type_args_subst_checked(
    ctx: &ScopeContext<'_>,
    callee: &ExprPtr,
    type_args: &[ast::TypePtr],
) -> Result<TypeSubst, Option<&'static str>> {
    let (origin, name) = match callee.as_deref().map(|callee| &callee.node) {
        Some(ExprNode::IdentifierExpr(ident)) => match resolve_value_name(ctx, &ident.name) {
            Some(entity) if entity.origin_opt.is_some() => {
                (entity.origin_opt.unwrap_or_default(), entity.target_opt.unwrap_or_else(|| ident.name.clone()))
            }
            _ => return Err(None),
        },
        Some(ExprNode::PathExpr(path)) => (path.path.clone(), path.name.clone()),
        _ => return Err(None),
    };
    let proc = ctx
        .sigma
        .mods
        .iter()
        .find(|module| module.path == origin)
        .and_then(|module| crate::memory::calls::find_procedure_in_module(module, &name));
    let Some(generics) = proc.and_then(|proc| proc.generic_params.as_ref()) else {
        return Err(None);
    };
    let params = &generics.params[..];
    if type_args.len() < required_type_arg_count(params) || type_args.len() > params.len() {
        return Err(Some("E-TYP-2303"));
    }
    let lowered = type_args.iter().map(|arg| lower_type(ctx, arg)).collect::<Result<Vec<_>, _>>()?;
    Ok(build_substitution(params, &lowered))
}

/// Types a call with explicit type arguments.
pub fn type_call_type_args_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::CallTypeArgsExpr,
    env: &TypeEnv,
) -> ExprTypeResult {
    if expr.type_args.is_empty() {
        return ExprTypeResult::failed(Some("CallTypeArgs-No-TypeArgs"));
    }
    let subst = match build_call_type_args_subst_checked(ctx, &expr.callee, &expr.type_args) {
        Ok(subst) => subst,
        Err(diag_id) => return ExprTypeResult::failed(Some(diag_id.unwrap_or("E-SEM-2533"))),
    };
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
    let call =
        type_call_with_subst(ctx, &expr.callee, &expr.args, &subst, &type_expr_fn, Some(&type_place_fn), Some(&check_expr_fn), None);
    if !call.ok {
        return ExprTypeResult { diag_id: call.diag_id, diag_detail: call.diag_detail, ..Default::default() };
    }
    if type_ctx.require_pure {
        let callee_type = type_expr_fn(&expr.callee);
        if callee_type.ok
            && matches!(strip_perm(&callee_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Func { params, .. }) if !params_pure(params))
        {
            return ExprTypeResult::failed(Some("E-SEM-2802"));
        }
    }
    ExprTypeResult::typed(call.r#type)
}
