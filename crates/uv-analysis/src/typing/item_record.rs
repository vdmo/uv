//! Record declarations: fields, the classes a record implements, and its methods and
//! their bodies. See `TypeRecordDecl`.
//!
//! A type invariant, and a method contract (its own or one it inherits from a class),
//! are checked by code that is not ported; a record with one makes its declaration
//! pending.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uv_core::diagnostics::DiagnosticStream;
use uv_source::ast::{self, AttributeArgValue};
use uv_source::attributes::{attrs, has_attribute, resolve_verification_mode_attribute, validate_attributes, AttributeTarget, VerificationModeAttribute};
use uv_source::lexer::token::TokenKind;

use crate::contracts::contract_check::{check_behavioral_subtyping, check_contract_well_formed, check_type_invariant};
use crate::contracts::purity::ContractContext;
use crate::contracts::verification::extend_proof_context_with_predicate_at;
use super::dynamic_context::{compute_dynamic_context, DynamicScopeAncestor};
use super::item_generic_params::process_generic_params;
use super::item_procedure::{has_explicit_return, DeclFailure, DeclOutcome};
use super::pending::{reset_scaffolding, take_pending};
use super::signature::{build_method_signature, subst_self_type};
use super::stmt::block::type_block;
use super::stmt_context::StmtTypeContext;
use super::subtyping::subtyping;
use super::type_env::{TypeBinding, TypeEnv};
use super::type_equiv::type_equiv;
use super::type_expr::{type_expr, type_identifier_expr, type_place};
use super::type_lookup::field_type;
use super::type_lower::lower_type;
use super::type_predicates::{bitcopy_type, eq_type_in};
use super::type_wf::{type_wf, REFINEMENT_WF_PENDING};
use super::typecheck_diag::emit_resolved_typecheck_diagnostic;
use super::types::*;
use crate::composite::classes::{check_orphan_rule, class_field_table, class_method_table, is_modal_class};
use crate::composite::record_methods::recv_mode_of;
use crate::context::ScopeContext;
use crate::generics::monomorphize::TypeSubst;
use crate::layout::align_of;
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

fn lower_type_with_wf(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Result<TypeRef, Diag> {
    let lowered = lower_type(ctx, ty)?;
    type_wf(ctx, &lowered)?;
    Ok(lowered)
}

fn vis_rank(vis: ast::Visibility) -> u8 {
    match vis {
        ast::Visibility::Public => 4,
        ast::Visibility::Internal => 3,
        ast::Visibility::Private => 1,
    }
}

fn has_impl(impls: &[Vec<String>], name: &str) -> bool {
    impls.iter().any(|impl_path| matches!(impl_path.as_slice(), [only] if only == name))
}

fn distinct_param_names(params: &[ast::Param]) -> bool {
    let mut names = HashSet::new();
    params.iter().all(|param| names.insert(&param.name))
}

/// The associated types the record names, each with its default, as the types `Self::Name` stands for.
fn collect_record_associated_type_bindings(ctx: &ScopeContext<'_>, record: &ast::RecordDecl, self_type: &TypeRef) -> Result<TypeSubst, Diag> {
    let mut subst = TypeSubst::new();
    for member in &record.members {
        let ast::RecordMember::AssociatedTypeDecl(assoc) = member else {
            continue;
        };
        if assoc.default_type.is_none() {
            if !record.implements.is_empty() {
                return Err(Some("E-TYP-2503"));
            }
            continue;
        }
        let lowered = lower_type_with_wf(ctx, &assoc.default_type)?;
        let substituted = subst_self_type(self_type, &lowered, Some(&subst));
        subst.insert(assoc.name.clone(), substituted);
    }
    Ok(subst)
}

/// The associated types of a class, bound by the record's own or the class's default.
fn build_class_associated_type_bindings(
    ctx: &ScopeContext<'_>,
    record: &ast::RecordDecl,
    class_decl: &ast::ClassDecl,
    self_type: &TypeRef,
    record_assoc_subst: &TypeSubst,
) -> Result<TypeSubst, Diag> {
    let mut out = record_assoc_subst.clone();
    for item in &class_decl.items {
        let ast::ClassItem::AssociatedTypeDecl(assoc) = item else {
            continue;
        };
        if out.contains_key(&assoc.name) {
            continue;
        }
        let record_assoc = record.members.iter().find_map(|member| match member {
            ast::RecordMember::AssociatedTypeDecl(candidate) if id_eq(&candidate.name, &assoc.name) => Some(candidate),
            _ => None,
        });
        let binding_type = match record_assoc.filter(|assoc| assoc.default_type.is_some()) {
            Some(record_assoc) => &record_assoc.default_type,
            None if assoc.default_type.is_some() => &assoc.default_type,
            None => return Err(Some("E-TYP-2503")),
        };
        let lowered = lower_type_with_wf(ctx, binding_type)?;
        let substituted = subst_self_type(self_type, &lowered, Some(&out));
        out.insert(assoc.name.clone(), substituted);
    }
    Ok(out)
}

fn parse_u64_literal(token: &uv_source::lexer::Token) -> Option<u64> {
    if token.kind != TokenKind::IntLiteral {
        return None;
    }
    let text: String = token.lexeme.chars().filter(|ch| *ch != '_').collect();
    text.parse().ok()
}

/// The alignment a `layout(align(n))` or `align(n)` attribute asks for, with its span.
fn record_requested_align(attr_list: &[ast::AttributeItem]) -> Option<(u64, uv_core::span::Span)> {
    for attr in attr_list {
        if attr.name.full_name == attrs::LAYOUT {
            for arg in &attr.args {
                if arg.key.as_deref() != Some("align") {
                    continue;
                }
                let AttributeArgValue::AttributeArgList(nested) = &arg.value else {
                    continue;
                };
                let [only] = nested.as_slice() else {
                    continue;
                };
                let AttributeArgValue::Token(token) = &only.value else {
                    continue;
                };
                if let Some(parsed) = parse_u64_literal(token) {
                    return Some((parsed, attr.span.clone()));
                }
            }
        }
        if attr.name.full_name == attrs::ALIGN {
            if let Some(AttributeArgValue::Token(token)) = attr.args.first().map(|arg| &arg.value) {
                if let Some(parsed) = parse_u64_literal(token) {
                    return Some((parsed, attr.span.clone()));
                }
            }
        }
    }
    None
}

/// See `TypeRecordDecl`.
pub fn type_record_decl(ctx: &ScopeContext<'_>, decl: &ast::RecordDecl, module_path: &[String], diags: &Rc<RefCell<DiagnosticStream>>) -> DeclOutcome {
    let attr_validation = validate_attributes(&decl.attrs, AttributeTarget::Record);
    if !attr_validation.ok {
        return failed_with(attr_validation.diag_id);
    }
    let type_path: Vec<String> = module_path.iter().cloned().chain([decl.name.clone()]).collect();
    let self_type = make_type_path(type_path.clone());
    let type_params = decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..]);
    if let Err(diag_id) = process_generic_params(ctx, type_params) {
        return failed_with(diag_id);
    }
    let mut impl_keys: Vec<_> = decl.implements.iter().map(|impl_path| path_key_of(impl_path)).collect();
    impl_keys.sort();
    if impl_keys.windows(2).any(|pair| pair[0] == pair[1]) {
        return failed("E-TYP-2506");
    }
    if has_impl(&decl.implements, "Bitcopy") && has_impl(&decl.implements, "Drop") {
        return failed("E-TYP-2621");
    }
    // A `Hash` implementation needs `Eq`, declared or derived.
    if has_impl(&decl.implements, "Hash") && !(eq_type_in(ctx, &self_type) || has_impl(&decl.implements, "Eq")) {
        return failed("E-TYP-2503");
    }
    let fields: Vec<&ast::FieldDecl> = decl.members.iter().filter_map(|member| if let ast::RecordMember::FieldDecl(field) = member { Some(field) } else { None }).collect();
    let methods: Vec<&ast::MethodDecl> = decl.members.iter().filter_map(|member| if let ast::RecordMember::MethodDecl(method) = member { Some(method) } else { None }).collect();
    if fields.iter().any(|field| vis_rank(field.vis) > vis_rank(decl.vis)) {
        return failed("E-TYP-1906");
    }
    let mut seen = HashSet::new();
    if !fields.iter().all(|field| seen.insert(&field.name)) {
        return DeclOutcome::Failed(DeclFailure {
            diag_id: Some("E-TYP-1901"),
            diagnostic_obligation_ids: ["WF-Record-DupField", "diagnostics.Records"].map(String::from).to_vec(),
            ..Default::default()
        });
    }
    let mut seen = HashSet::new();
    if !methods.iter().all(|method| seen.insert(&method.name)) {
        return failed("Record-Method-Dup");
    }
    let record_assoc_subst = match collect_record_associated_type_bindings(ctx, decl, &self_type) {
        Ok(subst) => subst,
        Err(diag_id) => return failed_with(diag_id),
    };
    let mut field_types: Vec<(String, TypeRef)> = Vec::new();
    for field in &fields {
        if has_attribute(&field.attrs, attrs::DYNAMIC) {
            return failed("E-CON-0412");
        }
        let field_attr_validation = validate_attributes(&field.attrs, AttributeTarget::Field);
        if !field_attr_validation.ok {
            return failed_with(field_attr_validation.diag_id);
        }
        let lowered = match lower_type_with_wf(ctx, &field.r#type) {
            Ok(lowered) => lowered,
            Err(diag_id) => return failed_with(diag_id),
        };
        let field_type = subst_self_type(&self_type, &lowered, Some(&record_assoc_subst));
        field_types.push((field.name.clone(), field_type.clone()));
        if field.init_opt.is_some() {
            let mut env = TypeEnv::default();
            env.scopes.push(Default::default());
            let type_ctx = StmtTypeContext { return_type: make_type_prim("()"), ..Default::default() };
            reset_scaffolding();
            let init_result = type_expr(ctx, &type_ctx, &field.init_opt, &env);
            if let Some(what) = take_pending() {
                return DeclOutcome::Pending(what.to_string());
            }
            if !init_result.ok {
                return failed_with(init_result.diag_id);
            }
            let sub = subtyping(ctx, &init_result.r#type, &field_type);
            if !sub.ok {
                return failed_with(sub.diag_id);
            }
            if !sub.subtype {
                return failed("E-MOD-2402");
            }
        }
    }
    if let Some((requested, span)) = record_requested_align(&decl.attrs) {
        let natural: Option<u64> = field_types.iter().try_fold(1u64, |natural, (_, ty)| align_of(ctx, ty).map(|align| natural.max(align)));
        if natural.is_some_and(|natural| requested < natural) {
            emit_resolved_typecheck_diagnostic(&mut diags.borrow_mut(), "W-MOD-2451", Some(span), "");
        }
    }
    if has_impl(&decl.implements, "Bitcopy") && field_types.iter().any(|(_, ty)| !bitcopy_type(ctx, ty)) {
        return failed("E-TYP-2622");
    }
    if methods.iter().any(|method| method.name == "drop") && field_types.iter().all(|(_, ty)| bitcopy_type(ctx, ty)) {
        return failed("E-TYP-2621");
    }
    if let Some(invariant) = &decl.invariant_opt {
        if fields.iter().any(|field| field.vis == ast::Visibility::Public) {
            return failed("E-SEM-2824");
        }
        let contract_ctx = ContractContext { scope_ctx: Some(ctx), receiver_type: self_type.clone(), ..Default::default() };
        let inv_result = check_type_invariant(&contract_ctx, invariant);
        if !inv_result.ok {
            return failed_with(inv_result.diag_id);
        }
    }

    let mut concrete_class_methods: HashSet<String> = HashSet::new();
    let mut inherited_dynamic_methods: HashSet<String> = HashSet::new();
    for impl_path in &decl.implements {
        let Some(class_decl) = ctx.sigma.classes.get(&path_key_of(impl_path)) else {
            return failed("Superclass-Undefined");
        };
        if !check_orphan_rule(ctx, &type_path, impl_path, &ctx.current_module) {
            return failed("E-TYP-2507");
        }
        if is_modal_class(class_decl) {
            return failed("E-TYP-2401");
        }
        let class_assoc_subst = match build_class_associated_type_bindings(ctx, decl, class_decl, &self_type, &record_assoc_subst) {
            Ok(subst) => subst,
            Err(diag_id) => return failed_with(diag_id),
        };
        let method_table = match class_method_table(ctx, impl_path) {
            Ok(table) => table,
            Err(diag_id) => return failed(diag_id),
        };
        let class_fields = match class_field_table(ctx, impl_path) {
            Ok(table) => table,
            Err(diag_id) => return failed(diag_id),
        };
        for class_field in class_fields {
            let Some(impl_field) = fields.iter().find(|field| id_eq(&field.name, &class_field.name)) else {
                return failed("E-TYP-2402");
            };
            let class_field_type = match lower_type_with_wf(ctx, &class_field.r#type) {
                Ok(lowered) => lowered,
                Err(diag_id) => return failed_with(diag_id),
            };
            let Some(impl_field_type) = field_type(decl, &impl_field.name, ctx, &[]) else {
                return failed("E-TYP-2404");
            };
            let class_field_subst = subst_self_type(&self_type, &class_field_type, Some(&class_assoc_subst));
            let impl_field_subst = subst_self_type(&self_type, &impl_field_type, Some(&class_assoc_subst));
            let sub = subtyping(ctx, &impl_field_subst, &class_field_subst);
            if !sub.ok {
                return failed_with(sub.diag_id);
            }
            if !sub.subtype {
                return failed("E-TYP-2404");
            }
        }
        for entry in method_table {
            let class_method = entry.method;
            if class_method.body_opt.is_some() {
                concrete_class_methods.insert(id_key_of(&class_method.name));
            }
            let impl_method = methods.iter().find(|method| id_eq(&method.name, &class_method.name));
            match (class_method.body_opt.is_some(), impl_method) {
                (false, None) => return failed("E-TYP-2503"),
                (false, Some(method)) if method.override_flag => return failed("E-TYP-2501"),
                (true, Some(method)) if !method.override_flag => return failed("E-TYP-2502"),
                _ => {}
            }
            let Some(impl_method) = impl_method else {
                continue;
            };
            if resolve_verification_mode_attribute(&class_method.attrs) == Some(VerificationModeAttribute::Dynamic) {
                inherited_dynamic_methods.insert(id_key_of(&impl_method.name));
            }
            let class_sig = build_method_signature(ctx, &self_type, &class_method.receiver, &class_method.params, &class_method.return_type_opt, Some(&class_assoc_subst));
            let impl_sig = build_method_signature(ctx, &self_type, &impl_method.receiver, &impl_method.params, &impl_method.return_type_opt, Some(&class_assoc_subst));
            let (class_sig, impl_sig) = match (class_sig, impl_sig) {
                (Ok(class_sig), Ok(impl_sig)) => (class_sig, impl_sig),
                (Err(diag_id), _) | (Ok(_), Err(diag_id)) => return failed_with(diag_id),
            };
            if !type_equiv(&class_sig.func_type, &impl_sig.func_type) {
                return failed("E-TYP-2503");
            }
            let class_contract = class_method.contract.clone().unwrap_or_default();
            let impl_contract = impl_method.contract.clone().unwrap_or_default();
            let behavioral = check_behavioral_subtyping(&class_contract, &impl_contract);
            if !behavioral.ok {
                return failed_with(behavioral.diag_id);
            }
        }
    }
    if methods.iter().any(|method| method.override_flag && !concrete_class_methods.contains(&id_key_of(&method.name))) {
        return failed("E-UNS-0105");
    }

    for method in &methods {
        let method_attr_validation = validate_attributes(&method.attrs, AttributeTarget::Method);
        if !method_attr_validation.ok {
            return failed_with(method_attr_validation.diag_id);
        }
        if !distinct_param_names(&method.params) {
            return failed("E-SEM-2713");
        }
        if method.params.iter().any(|param| id_eq(&param.name, "self")) {
            return failed("E-SEM-3011");
        }
        if has_attribute(&method.attrs, attrs::STATIC) {
            return failed("E-MOD-2452");
        }
        let (recv_perm, const_receiver) = match &method.receiver {
            ast::Receiver::ReceiverShorthand(shorthand) => match shorthand.perm {
                ast::ReceiverPerm::Unique => (Some(Permission::Unique), false),
                ast::ReceiverPerm::Shared => (Some(Permission::Shared), false),
                _ => (Some(Permission::Const), shorthand.mode_opt.is_none()),
            },
            _ => (None, false),
        };
        let self_param = BindSelfParam { r#type: self_type.clone(), mode: recv_mode_of(&method.receiver), recv_perm };
        let sig = match build_method_signature(ctx, &self_type, &method.receiver, &method.params, &method.return_type_opt, Some(&record_assoc_subst)) {
            Ok(sig) => sig,
            Err(diag_id) => return failed_with(diag_id),
        };
        if let Some(contract) = &method.contract {
            let contract_ctx = ContractContext {
                scope_ctx: Some(ctx),
                receiver_type: self_type.clone(),
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
        let is_unit = type_equiv(&sig.return_type, &make_type_prim("()"));
        if !is_unit && !has_explicit_return(body) {
            return failed("E-TYP-1507");
        }
        let mut env = TypeEnv::default();
        env.scopes.push(Default::default());
        for (name, ty) in &sig.bindings {
            let unique_self = name == "self" && matches!(&method.receiver, ast::Receiver::ReceiverShorthand(shorthand) if shorthand.perm == ast::ReceiverPerm::Unique);
            let r#mut = if unique_self { ast::Mutability::Var } else { ast::Mutability::Let };
            env.scopes[0].insert(id_key_of(name), TypeBinding { r#mut, r#type: ty.clone(), ..Default::default() });
        }
        let env = Rc::new(RefCell::new(env));
        let ancestors = [
            DynamicScopeAncestor { attrs: &decl.attrs, span: &decl.span },
            DynamicScopeAncestor { attrs: &method.attrs, span: &method.span },
        ];
        let type_ctx = StmtTypeContext {
            return_type: sig.return_type.clone(),
            diags: Some(diags.clone()),
            env_ref: Some(env.clone()),
            contract_dynamic: inherited_dynamic_methods.contains(&id_key_of(&method.name)) || compute_dynamic_context(&body.span, &ancestors),
            contract: method.contract.as_ref(),
            proof_ctx: match &decl.invariant_opt {
                Some(invariant) if const_receiver => extend_proof_context_with_predicate_at(None, &invariant.predicate, &body.span).map(Rc::new),
                _ => None,
            },
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
        let bind = bind_check_body(ctx, module_path, &method.params, &method.body, Some(&self_param));
        if let Some(what) = take_pending() {
            return DeclOutcome::Pending(what.to_string());
        }
        if !bind.ok {
            return failed_with(bind.diag_id);
        }
    }
    DeclOutcome::Ok
}
