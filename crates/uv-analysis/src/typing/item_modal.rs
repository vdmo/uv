//! Modal declarations: the states, their fields, methods and transitions with their
//! bodies, the invariant, and the abstract states of the classes the type implements.
//! See `TypeModalDecl`.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use uv_core::diagnostics::DiagnosticStream;
use uv_source::ast;

use super::dynamic_context::{compute_dynamic_context, DynamicScopeAncestor};
use super::item_generic_params::process_generic_params;
use super::item_procedure::{DeclFailure, DeclOutcome};
use super::pending::{reset_scaffolding, take_pending};
use super::signature::{build_method_signature, build_transition_signature, subst_self_type};
use super::stmt::block::type_block;
use super::stmt_context::StmtTypeContext;
use super::subtyping::subtyping;
use super::type_env::{TypeBinding, TypeEnv};
use super::type_equiv::type_equiv;
use super::type_expr::{type_expr, type_identifier_expr, type_place};
use super::type_lower::lower_type;
use super::type_predicates::eq_type_in;
use super::type_wf::{type_wf, REFINEMENT_WF_PENDING};
use super::types::*;
use crate::composite::class_linearization::linearize_class;
use crate::composite::classes::{check_orphan_rule, class_abstract_states, class_field_table, is_modal_class};
use crate::composite::record_methods::recv_mode_of;
use crate::contracts::contract_check::{check_contract_well_formed, check_type_invariant};
use crate::contracts::purity::ContractContext;
use crate::contracts::verification::extend_proof_context_with_predicate_at;
use crate::context::ScopeContext;
use crate::memory::borrow_bind::{bind_check_body, BindSelfParam};
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};

type Diag = Option<&'static str>;

fn failed(diag_id: &'static str) -> DeclOutcome {
    DeclOutcome::Failed(DeclFailure::rule(diag_id))
}

fn failed_with(diag_id: Diag) -> DeclOutcome {
    if diag_id == Some(REFINEMENT_WF_PENDING) {
        return DeclOutcome::Pending("RefinementWF".to_string());
    }
    DeclOutcome::Failed(DeclFailure { diag_id, ..Default::default() })
}

fn failed_detail(diag_id: &'static str, detail: String) -> DeclOutcome {
    DeclOutcome::Failed(DeclFailure { diag_id: Some(diag_id), diag_detail: detail, ..Default::default() })
}

fn lower_type_with_wf(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Result<TypeRef, Diag> {
    let lowered = lower_type(ctx, ty)?;
    type_wf(ctx, &lowered)?;
    Ok(lowered)
}

fn all_distinct<'a>(names: impl Iterator<Item = &'a String>) -> bool {
    let mut seen = HashSet::new();
    names.into_iter().all(|name| seen.insert(name))
}

fn vis_rank(vis: ast::Visibility) -> u8 {
    match vis {
        ast::Visibility::Public => 4,
        ast::Visibility::Internal => 3,
        ast::Visibility::Private => 1,
    }
}

fn state_member_vis_ok(modal: &ast::ModalDecl) -> bool {
    modal.states.iter().flat_map(|state| &state.members).all(|member| {
        let vis = match member {
            ast::StateMember::StateFieldDecl(field) => field.vis,
            ast::StateMember::StateMethodDecl(method) => method.vis,
            ast::StateMember::TransitionDecl(transition) => transition.vis,
        };
        vis_rank(vis) <= vis_rank(modal.vis)
    })
}

/// The invariant that holds in a state: the arm for it when the invariant is an `if case`
/// over `self`, the whole predicate otherwise.
fn state_invariant_predicate_for(invariant: &ast::TypeInvariant, state_name: &str) -> ast::ExprPtr {
    let predicate = invariant.predicate.as_ref()?;
    let ast::ExprNode::IfCaseExpr(if_case) = &predicate.node else {
        return invariant.predicate.clone();
    };
    let on_self = matches!(if_case.scrutinee.as_deref().map(|expr| &expr.node), Some(ast::ExprNode::IdentifierExpr(ident)) if id_eq(&ident.name, "self"));
    if !on_self {
        return invariant.predicate.clone();
    }
    for arm in &if_case.cases {
        if let Some(pattern) = &arm.pattern {
            if matches!(&pattern.node, ast::PatternNode::ModalPattern(modal) if id_eq(&modal.state, state_name)) {
                return arm.body.clone();
            }
        }
    }
    invariant.predicate.clone()
}

struct RequiredStateInfo {
    name: String,
    fields: Vec<(String, TypeRef)>,
}

fn required_state_shape_compatible(lhs: &RequiredStateInfo, rhs: &RequiredStateInfo) -> bool {
    lhs.fields.len() == rhs.fields.len()
        && lhs.fields.iter().all(|(name, ty)| rhs.fields.iter().find(|(other, _)| id_eq(name, other)).is_some_and(|(_, other_ty)| type_equiv(ty, other_ty)))
}

fn lower_required_state_info(ctx: &ScopeContext<'_>, impl_self_type: &TypeRef, state: &ast::AbstractStateDecl) -> Result<RequiredStateInfo, Diag> {
    let mut info = RequiredStateInfo { name: state.name.clone(), fields: Vec::new() };
    let mut seen = HashSet::new();
    for field in &state.fields {
        if !seen.insert(&field.name) {
            return Err(Some("Class-AbstractField-Dup"));
        }
        let lowered = lower_type_with_wf(ctx, &field.r#type)?;
        info.fields.push((field.name.clone(), subst_self_type(impl_self_type, &lowered, None)));
    }
    Ok(info)
}

/// The abstract states a modal class asks for, through its superclasses.
fn collect_modal_required_states(ctx: &ScopeContext<'_>, impl_self_type: &TypeRef, class_path: &[String]) -> Result<Vec<RequiredStateInfo>, Diag> {
    let order = linearize_class(ctx, class_path).map_err(Some)?;
    let mut states: Vec<RequiredStateInfo> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    for lin_path in &order {
        let Some(class_decl) = ctx.sigma.classes.get(&path_key_of(lin_path)) else {
            return Err(Some("Superclass-Undefined"));
        };
        for abstract_state in class_abstract_states(class_decl) {
            let lowered = lower_required_state_info(ctx, impl_self_type, abstract_state)?;
            match index.get(&lowered.name) {
                None => {
                    index.insert(lowered.name.clone(), states.len());
                    states.push(lowered);
                }
                Some(&existing) => {
                    if !required_state_shape_compatible(&states[existing], &lowered) {
                        return Err(Some("E-TYP-2407"));
                    }
                }
            }
        }
    }
    Ok(states)
}

/// See `TypeModalDecl`.
pub fn type_modal_decl(ctx: &ScopeContext<'_>, decl: &ast::ModalDecl, module_path: &[String], diags: &Rc<RefCell<DiagnosticStream>>) -> DeclOutcome {
    let type_path: Vec<String> = module_path.iter().cloned().chain([decl.name.clone()]).collect();
    let type_params = decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..]);
    let gen_params = match process_generic_params(ctx, type_params) {
        Ok(params) => params,
        Err(diag_id) => return failed_with(diag_id),
    };
    let self_generic_args: Vec<TypeRef> = gen_params.iter().map(|param| make_type_path(vec![param.name.clone()])).collect();
    let self_type = make_type_path_with(type_path.clone(), self_generic_args.clone());

    let mut impl_keys: Vec<_> = decl.implements.iter().map(|impl_path| path_key_of(impl_path)).collect();
    impl_keys.sort();
    if impl_keys.windows(2).any(|pair| pair[0] == pair[1]) {
        return failed("E-TYP-2506");
    }
    let has_impl = |name: &str| decl.implements.iter().any(|impl_path| matches!(impl_path.as_slice(), [only] if only == name));
    if has_impl("Hash") && !(eq_type_in(ctx, &self_type) || has_impl("Eq")) {
        return failed("E-TYP-2503");
    }
    if !state_member_vis_ok(decl) {
        return failed("StateMemberVisOk-Err");
    }
    if !all_distinct(decl.states.iter().map(|state| &state.name)) {
        return failed("E-TYP-2051");
    }
    if decl.states.iter().any(|state| state.name == decl.name) {
        return failed("E-TYP-2054");
    }
    if decl.states.is_empty() {
        return failed("E-TYP-2050");
    }
    let state_names: HashSet<&String> = decl.states.iter().map(|state| &state.name).collect();

    for state in &decl.states {
        let fields: Vec<&ast::StateFieldDecl> = state.members.iter().filter_map(|m| if let ast::StateMember::StateFieldDecl(f) = m { Some(f) } else { None }).collect();
        let methods: Vec<&ast::StateMethodDecl> = state.members.iter().filter_map(|m| if let ast::StateMember::StateMethodDecl(f) = m { Some(f) } else { None }).collect();
        let transitions: Vec<&ast::TransitionDecl> = state.members.iter().filter_map(|m| if let ast::StateMember::TransitionDecl(f) = m { Some(f) } else { None }).collect();
        if !all_distinct(fields.iter().map(|field| &field.name)) {
            return failed("E-TYP-2058");
        }
        for field in &fields {
            if let Err(diag_id) = lower_type_with_wf(ctx, &field.r#type) {
                return failed_with(diag_id);
            }
        }
        if !all_distinct(methods.iter().map(|method| &method.name)) {
            return failed("StateMethod-Dup");
        }
        if !all_distinct(transitions.iter().map(|transition| &transition.name)) {
            return failed("Transition-Dup");
        }
        let method_names: HashSet<&String> = methods.iter().map(|method| &method.name).collect();
        let methods_clash_transitions = !methods.is_empty() && transitions.iter().any(|transition| method_names.contains(&transition.name));
        let all_names = fields.iter().map(|f| &f.name).chain(methods.iter().map(|m| &m.name)).chain(transitions.iter().map(|t| &t.name));
        if methods_clash_transitions || !all_distinct(all_names) {
            return failed("StateMember-Name-Conflict");
        }

        for method in &methods {
            if !all_distinct(method.params.iter().map(|param| &param.name)) {
                return failed("E-SEM-2713");
            }
            if method.params.iter().any(|param| id_eq(&param.name, "self")) {
                return failed("E-SEM-3011");
            }
            let state_type = make_type_modal_state(type_path.clone(), &state.name, self_generic_args.clone());
            let sig = match build_method_signature(ctx, &state_type, &method.receiver, &method.params, &method.return_type_opt, None) {
                Ok(sig) => sig,
                Err(diag_id) => return failed_with(diag_id),
            };
            if let Some(contract) = &method.contract {
                let contract_ctx = ContractContext {
                    scope_ctx: Some(ctx),
                    receiver_type: state_type.clone(),
                    return_type: sig.return_type.clone(),
                    params: sig.bindings.iter().filter(|(name, _)| name != "self").cloned().collect(),
                    ..Default::default()
                };
                let contract_check = check_contract_well_formed(&contract_ctx, contract);
                if !contract_check.ok {
                    return failed_with(contract_check.diag_id);
                }
            }
            let Some(body) = method.body.as_deref() else {
                continue;
            };
            let unique_receiver = matches!(&method.receiver, ast::Receiver::ReceiverShorthand(shorthand) if shorthand.perm == ast::ReceiverPerm::Unique);
            let mut env = TypeEnv::default();
            env.scopes.push(Default::default());
            for (name, ty) in &sig.bindings {
                let r#mut = if name == "self" && unique_receiver { ast::Mutability::Var } else { ast::Mutability::Let };
                env.scopes[0].insert(id_key_of(name), TypeBinding { r#mut, r#type: ty.clone(), ..Default::default() });
            }
            let env = Rc::new(RefCell::new(env));
            let ancestors = [
                DynamicScopeAncestor { attrs: &decl.attrs, span: &decl.span },
                DynamicScopeAncestor { attrs: &method.attrs, span: &method.span },
            ];
            let const_receiver = matches!(&method.receiver, ast::Receiver::ReceiverShorthand(shorthand) if shorthand.perm == ast::ReceiverPerm::Const && shorthand.mode_opt.is_none());
            let proof_ctx = match &decl.invariant_opt {
                Some(invariant) if const_receiver => {
                    extend_proof_context_with_predicate_at(None, &state_invariant_predicate_for(invariant, &state.name), &body.span).map(Rc::new)
                }
                _ => None,
            };
            let type_ctx = StmtTypeContext {
                return_type: sig.return_type.clone(),
                diags: Some(diags.clone()),
                env_ref: Some(env.clone()),
                contract_dynamic: compute_dynamic_context(&body.span, &ancestors),
                contract: method.contract.as_ref(),
                proof_ctx,
                ..Default::default()
            };
            let type_expr_fn = |inner: &ast::ExprPtr| type_expr(ctx, &type_ctx, inner, &env.borrow().clone());
            let type_ident_fn = |name: &str| type_identifier_expr(ctx, &env.borrow(), name);
            let type_place_fn = |inner: &ast::ExprPtr| type_place(ctx, &type_ctx, inner, &env.borrow().clone());
            let start_env = env.borrow().clone();
            reset_scaffolding();
            let body_result = type_block(ctx, &type_ctx, body, &start_env, &type_expr_fn, &type_ident_fn, &type_place_fn, Some(&env));
            if let Some(what) = take_pending() {
                return DeclOutcome::Pending(what.to_string());
            }
            if !body_result.ok {
                return failed_with(body_result.diag_id);
            }
            let recv_perm = match &method.receiver {
                ast::Receiver::ReceiverShorthand(shorthand) => Some(match shorthand.perm {
                    ast::ReceiverPerm::Unique => Permission::Unique,
                    ast::ReceiverPerm::Shared => Permission::Shared,
                    _ => Permission::Const,
                }),
                _ => None,
            };
            let self_param = BindSelfParam { r#type: state_type, mode: recv_mode_of(&method.receiver), recv_perm };
            let bind = bind_check_body(ctx, module_path, &method.params, &method.body, Some(&self_param));
            if let Some(what) = take_pending() {
                return DeclOutcome::Pending(what.to_string());
            }
            if !bind.ok {
                return failed_with(bind.diag_id);
            }
        }

        for transition in &transitions {
            if !all_distinct(transition.params.iter().map(|param| &param.name)) {
                return failed("E-SEM-2713");
            }
            if transition.params.iter().any(|param| id_eq(&param.name, "self")) {
                return failed("E-SEM-3011");
            }
            if !state_names.contains(&transition.target_state) {
                return failed("E-TYP-2059");
            }
            let source_type = make_type_modal_state(type_path.clone(), &state.name, self_generic_args.clone());
            let target_type = make_type_modal_state(type_path.clone(), &transition.target_state, self_generic_args.clone());
            let sig = match build_transition_signature(ctx, &source_type, &target_type, &transition.params, None) {
                Ok(sig) => sig,
                Err(diag_id) => return failed_with(diag_id),
            };
            let Some(body) = transition.body.as_deref() else {
                continue;
            };
            let mut env = TypeEnv::default();
            env.scopes.push(Default::default());
            for (name, ty) in &sig.bindings {
                env.scopes[0].insert(id_key_of(name), TypeBinding { r#mut: ast::Mutability::Let, r#type: ty.clone(), ..Default::default() });
            }
            let ancestors = [
                DynamicScopeAncestor { attrs: &decl.attrs, span: &decl.span },
                DynamicScopeAncestor { attrs: &transition.attrs, span: &transition.span },
            ];
            let type_ctx = StmtTypeContext {
                return_type: target_type.clone(),
                contract_dynamic: compute_dynamic_context(&body.span, &ancestors),
                ..Default::default()
            };
            let type_expr_fn = |inner: &ast::ExprPtr| type_expr(ctx, &type_ctx, inner, &env);
            let type_ident_fn = |name: &str| type_identifier_expr(ctx, &env, name);
            let type_place_fn = |inner: &ast::ExprPtr| type_place(ctx, &type_ctx, inner, &env);
            reset_scaffolding();
            let body_result = type_block(ctx, &type_ctx, body, &env, &type_expr_fn, &type_ident_fn, &type_place_fn, None);
            if let Some(what) = take_pending() {
                return DeclOutcome::Pending(what.to_string());
            }
            if !body_result.ok {
                return DeclOutcome::Failed(DeclFailure { diag_id: body_result.diag_id, diag_detail: body_result.diag_detail, ..Default::default() });
            }
            let sub = subtyping(ctx, &body_result.r#type, &target_type);
            if !sub.ok || !sub.subtype {
                return failed_detail("E-TYP-2055", "transition body type is not the declared target state".to_string());
            }
            let self_param = BindSelfParam { r#type: source_type, mode: Some(ParamMode::Move), recv_perm: Some(Permission::Unique) };
            let bind = bind_check_body(ctx, module_path, &transition.params, &transition.body, Some(&self_param));
            if let Some(what) = take_pending() {
                return DeclOutcome::Pending(what.to_string());
            }
            if !bind.ok {
                return failed_with(bind.diag_id);
            }
        }
    }

    if let Some(invariant) = &decl.invariant_opt {
        let contract_ctx = ContractContext { scope_ctx: Some(ctx), receiver_type: self_type.clone(), ..Default::default() };
        let inv_result = check_type_invariant(&contract_ctx, invariant);
        if !inv_result.ok {
            return failed_with(inv_result.diag_id);
        }
    }

    let mut aggregate: Vec<RequiredStateInfo> = Vec::new();
    let mut aggregate_index: HashMap<String, usize> = HashMap::new();
    for impl_path in &decl.implements {
        let Some(class_decl) = ctx.sigma.classes.get(&path_key_of(impl_path)) else {
            return failed("Superclass-Undefined");
        };
        if !check_orphan_rule(ctx, &type_path, impl_path, &ctx.current_module) {
            return failed("E-TYP-2507");
        }
        let class_fields = match class_field_table(ctx, impl_path) {
            Ok(table) => table,
            Err(diag_id) => return failed(diag_id),
        };
        if let Some(field) = class_fields.first() {
            return failed_detail("Impl-Field-Missing", format!("modal type '{}' cannot satisfy required class field '{}'", decl.name, field.name));
        }
        if !is_modal_class(class_decl) {
            continue;
        }
        let required_states = match collect_modal_required_states(ctx, &self_type, impl_path) {
            Ok(states) => states,
            Err(diag_id) => return failed_with(diag_id),
        };
        for required in required_states {
            match aggregate_index.get(&required.name) {
                None => {
                    aggregate_index.insert(required.name.clone(), aggregate.len());
                    aggregate.push(required);
                }
                Some(&existing) => {
                    if !required_state_shape_compatible(&aggregate[existing], &required) {
                        return failed_detail("E-TYP-2407", format!("conflicting abstract state requirements for '{}'", required.name));
                    }
                }
            }
        }
    }
    for required in &aggregate {
        let Some(impl_state) = decl.states.iter().find(|state| id_eq(&state.name, &required.name)) else {
            return failed_detail("E-TYP-2403", format!("missing required state '{}'", required.name));
        };
        let impl_fields: Vec<&ast::StateFieldDecl> = impl_state.members.iter().filter_map(|m| if let ast::StateMember::StateFieldDecl(f) = m { Some(f) } else { None }).collect();
        for (field_name, field_type) in &required.fields {
            let Some(impl_field) = impl_fields.iter().find(|field| id_eq(&field.name, field_name)) else {
                return failed_detail("E-TYP-2405", format!("state '{}' missing payload field '{}'", required.name, field_name));
            };
            let lowered = match lower_type_with_wf(ctx, &impl_field.r#type) {
                Ok(lowered) => lowered,
                Err(diag_id) => return failed_with(diag_id),
            };
            if !type_equiv(field_type, &subst_self_type(&self_type, &lowered, None)) {
                return failed_detail("Impl-Field-Type-Err", format!("state '{}', field '{}' has incompatible type", required.name, field_name));
            }
        }
    }
    DeclOutcome::Ok
}
