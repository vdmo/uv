//! Class declarations: superclasses, member names, method signatures and the bodies of
//! default methods, associated types, fields, and abstract states. See `TypeClassDecl`.
//!
//! A method contract is checked by code that is not ported: a default method with one
//! makes the declaration pending.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uv_core::diagnostics::DiagnosticStream;
use uv_source::ast;
use uv_source::attributes::{attrs, has_attribute, validate_attributes, validate_unsupported_attribute_target, AttributeTarget};

use super::dynamic_context::{compute_dynamic_context, DynamicScopeAncestor};
use super::item_generic_params::process_generic_params;
use super::item_procedure::{DeclFailure, DeclOutcome};
use super::pending::{reset_scaffolding, take_pending};
use super::signature::build_method_signature;
use super::stmt::block::type_block;
use super::stmt_context::StmtTypeContext;
use super::type_env::{TypeBinding, TypeEnv};
use super::type_expr::{type_expr, type_identifier_expr, type_place};
use super::type_lower::lower_type;
use super::type_wf::{type_wf, REFINEMENT_WF_PENDING};
use super::types::*;
use crate::composite::classes::{class_field_table, class_method_table};
use crate::composite::record_methods::recv_mode_of;
use crate::context::ScopeContext;
use crate::memory::borrow_bind::{bind_check_body, BindSelfParam};
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};

type Diag = Option<&'static str>;

fn failed(diag_id: Diag) -> DeclOutcome {
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

fn distinct<T: std::hash::Hash + Eq>(names: impl IntoIterator<Item = T>) -> bool {
    let mut seen = HashSet::new();
    names.into_iter().all(|name| seen.insert(name))
}

/// See `TypeClassDecl`.
pub fn type_class_decl(ctx: &ScopeContext<'_>, decl: &ast::ClassDecl, module_path: &[String], diags: &Rc<RefCell<DiagnosticStream>>) -> DeclOutcome {
    let attr_validation = validate_unsupported_attribute_target(&decl.attrs, "class declarations");
    if !attr_validation.ok {
        return failed(attr_validation.diag_id);
    }
    let class_path: Vec<String> = module_path.iter().cloned().chain([decl.name.clone()]).collect();
    let self_type = self_var_type();
    let type_params = decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..]);
    if let Err(diag_id) = process_generic_params(ctx, type_params) {
        return failed(diag_id);
    }
    let mut super_keys: Vec<_> = decl.supers.iter().map(|super_path| path_key_of(super_path)).collect();
    super_keys.sort();
    if super_keys.windows(2).any(|pair| pair[0] == pair[1]) {
        return failed(Some("Impl-Duplicate-Class-Err"));
    }
    if decl.supers.iter().any(|super_path| !ctx.sigma.classes.contains_key(&path_key_of(super_path))) {
        return failed(Some("Superclass-Undefined"));
    }
    let methods: Vec<&ast::ClassMethodDecl> = decl.items.iter().filter_map(|item| if let ast::ClassItem::ClassMethodDecl(method) = item { Some(method) } else { None }).collect();
    let method_names: Vec<String> = methods.iter().map(|method| id_key_of(&method.name)).collect();
    if !distinct(&method_names) {
        return failed(Some("E-TYP-2500"));
    }
    for method in &methods {
        let method_attr_validation = validate_attributes(&method.attrs, AttributeTarget::Method);
        if !method_attr_validation.ok {
            return failed(method_attr_validation.diag_id);
        }
        if !distinct(method.params.iter().map(|param| &param.name)) {
            return failed(Some("E-SEM-2713"));
        }
        if method.params.iter().any(|param| id_eq(&param.name, "self")) {
            return failed(Some("E-SEM-3011"));
        }
        let sig = match build_method_signature(ctx, &self_type, &method.receiver, &method.params, &method.return_type_opt, None) {
            Ok(sig) => sig,
            Err(diag_id) => return failed(diag_id),
        };
        let Some(body) = method.body_opt.as_deref() else {
            continue;
        };
        if method.contract.is_some() {
            return DeclOutcome::Pending("ContractWF".to_string());
        }
        let recv_perm = match &method.receiver {
            ast::Receiver::ReceiverShorthand(shorthand) => Some(match shorthand.perm {
                ast::ReceiverPerm::Unique => Permission::Unique,
                ast::ReceiverPerm::Shared => Permission::Shared,
                _ => Permission::Const,
            }),
            _ => None,
        };
        let self_param = BindSelfParam { r#type: self_type.clone(), mode: recv_mode_of(&method.receiver), recv_perm };
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
            current_class_path: Some(class_path.clone()),
            contract_dynamic: compute_dynamic_context(&body.span, &ancestors),
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
            return failed(body_result.diag_id);
        }
        let bind = bind_check_body(ctx, module_path, &method.params, &method.body_opt, Some(&self_param));
        if let Some(what) = take_pending() {
            return DeclOutcome::Pending(what.to_string());
        }
        if !bind.ok {
            return failed(bind.diag_id);
        }
    }

    let assoc_types: Vec<&ast::AssociatedTypeDecl> = decl.items.iter().filter_map(|item| if let ast::ClassItem::AssociatedTypeDecl(assoc) = item { Some(assoc) } else { None }).collect();
    if !distinct(assoc_types.iter().map(|assoc| &assoc.name)) {
        return failed(Some("E-TYP-2504"));
    }
    for assoc in &assoc_types {
        let assoc_attr_validation = validate_attributes(&assoc.attrs, AttributeTarget::TypeAlias);
        if !assoc_attr_validation.ok {
            return failed(assoc_attr_validation.diag_id);
        }
        if assoc.default_type.is_some() {
            if let Err(diag_id) = lower_type_with_wf(ctx, &assoc.default_type) {
                return failed(diag_id);
            }
        }
    }
    let fields: Vec<&ast::ClassFieldDecl> = decl.items.iter().filter_map(|item| if let ast::ClassItem::ClassFieldDecl(field) = item { Some(field) } else { None }).collect();
    let field_names: Vec<String> = fields.iter().map(|field| id_key_of(&field.name)).collect();
    let abstract_states: Vec<&ast::AbstractStateDecl> = decl.items.iter().filter_map(|item| if let ast::ClassItem::AbstractStateDecl(state) = item { Some(state) } else { None }).collect();
    if !distinct(&field_names) {
        return failed(Some("E-TYP-2408"));
    }
    if method_names.iter().any(|name| field_names.contains(name)) {
        return failed(Some("E-TYP-2505"));
    }
    if !distinct(abstract_states.iter().map(|state| &state.name)) {
        return failed(Some("E-TYP-2409"));
    }
    for field in &fields {
        if has_attribute(&field.attrs, attrs::DYNAMIC) {
            return failed(Some("E-CON-0412"));
        }
        let field_attr_validation = validate_attributes(&field.attrs, AttributeTarget::Field);
        if !field_attr_validation.ok {
            return failed(field_attr_validation.diag_id);
        }
        if let Err(diag_id) = lower_type_with_wf(ctx, &field.r#type) {
            return failed(diag_id);
        }
    }
    // No two members share a name, whatever their kinds.
    let mut seen: HashSet<String> = method_names.iter().cloned().collect();
    let member_names = assoc_types.iter().map(|assoc| id_key_of(&assoc.name)).chain(fields.iter().map(|field| id_key_of(&field.name))).chain(abstract_states.iter().map(|state| id_key_of(&state.name)));
    for key in member_names {
        if !seen.insert(key) {
            return failed(Some("E-TYP-2505"));
        }
    }
    if let Err(diag_id) = class_method_table(ctx, &class_path) {
        return failed(Some(diag_id));
    }
    if let Err(diag_id) = class_field_table(ctx, &class_path) {
        return failed(Some(diag_id));
    }
    DeclOutcome::Ok
}
