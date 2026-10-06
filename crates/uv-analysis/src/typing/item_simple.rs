//! Type alias and static declarations.

use std::collections::BTreeSet;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_source::ast;
use uv_source::attributes::{attrs, get_attribute_value, has_attribute, validate_attributes, validate_unsupported_attribute_target, AttributeTarget};

use super::item_generic_params::process_generic_params;
use super::item_procedure::{DeclFailure, DeclOutcome};
use super::pending::{reset_scaffolding, take_pending};
use super::stmt_context::StmtTypeContext;
use super::type_env::TypeEnv;
use super::type_expr::check_expr_against;
use super::type_lower::lower_type;
use super::type_wf::{type_wf, REFINEMENT_WF_PENDING};
use super::signature::subst_self_type;
use super::type_predicates::{bitcopy_type, eq_type_in};
use super::types::{make_type_path, make_type_prim, TypeRef};
use crate::composite::classes::{check_orphan_rule, class_field_table, class_method_table, is_modal_class};
use crate::composite::enums::enum_discriminants;
use crate::caps::cap_requirements::type_has_capabilities;
use crate::contracts::contract_check::check_type_invariant;
use crate::contracts::purity::ContractContext;
use crate::context::{PathKey, ScopeContext, TypeDecl};
use crate::resolve::scopes::path_key_of;

fn failed(diag_id: Option<&'static str>) -> DeclOutcome {
    if diag_id == Some(REFINEMENT_WF_PENDING) {
        return DeclOutcome::Pending("RefinementWF".to_string());
    }
    DeclOutcome::Failed(DeclFailure { diag_id, ..Default::default() })
}

fn each(ctx: &ScopeContext<'_>, deps: &mut Vec<PathKey>, types: &mut dyn Iterator<Item = &ast::TypePtr>) {
    for inner in types {
        collect_alias_deps(ctx, inner, deps);
    }
}

/// The aliases a written type names.
fn collect_alias_deps(ctx: &ScopeContext<'_>, ty: &ast::TypePtr, deps: &mut Vec<PathKey>) {
    use ast::TypeNode as T;
    let Some(t) = ty.as_deref() else {
        return;
    };
    match &t.node {
        T::TypePathType(node) => {
            let key = path_key_of(&node.path);
            if matches!(ctx.sigma.types.get(&key), Some(TypeDecl::TypeAlias(_))) {
                deps.push(key);
            }
            each(ctx, deps, &mut node.generic_args.iter());
        }
        T::TypePermType(node) => each(ctx, deps, &mut std::iter::once(&node.base)),
        T::TypeUnion(node) => each(ctx, deps, &mut node.types.iter()),
        T::TypeFunc(node) => each(ctx, deps, &mut node.params.iter().map(|param| &param.r#type).chain([&node.ret])),
        T::TypeClosure(node) => {
            each(ctx, deps, &mut node.params.iter().map(|param| &param.r#type).chain([&node.ret]));
            if let Some(shared) = &node.deps_opt {
                for dep in shared {
                    collect_alias_deps(ctx, &dep.r#type, deps);
                }
            }
        }
        T::TypeTuple(node) => each(ctx, deps, &mut node.elements.iter()),
        T::TypeArray(node) => each(ctx, deps, &mut std::iter::once(&node.element)),
        T::TypeSlice(node) => each(ctx, deps, &mut std::iter::once(&node.element)),
        T::TypeSafePtr(node) => each(ctx, deps, &mut std::iter::once(&node.element)),
        T::TypeRawPtr(node) => each(ctx, deps, &mut std::iter::once(&node.element)),
        T::TypeRefine(node) => each(ctx, deps, &mut std::iter::once(&node.base)),
        T::TypeModalState(node) => each(ctx, deps, &mut node.generic_args.iter()),
        _ => {}
    }
}

fn type_alias_cycle_from(ctx: &ScopeContext<'_>, start: &PathKey, active: &mut BTreeSet<PathKey>, done: &mut BTreeSet<PathKey>) -> bool {
    if done.contains(start) {
        return false;
    }
    if !active.insert(start.clone()) {
        return true;
    }
    if let Some(TypeDecl::TypeAlias(alias)) = ctx.sigma.types.get(start) {
        let mut deps = Vec::new();
        collect_alias_deps(ctx, &alias.r#type, &mut deps);
        if deps.iter().any(|dep| type_alias_cycle_from(ctx, dep, active, done)) {
            return true;
        }
    }
    active.remove(start);
    done.insert(start.clone());
    false
}

/// See `TypeTypeAliasDecl`.
pub fn type_type_alias_decl(ctx: &ScopeContext<'_>, decl: &ast::TypeAliasDecl, module_path: &[String]) -> DeclOutcome {
    let attr_validation = validate_attributes(&decl.attrs, AttributeTarget::TypeAlias);
    if !attr_validation.ok {
        return failed(attr_validation.diag_id);
    }
    let type_path: Vec<String> = module_path.iter().cloned().chain([decl.name.clone()]).collect();
    if type_alias_cycle_from(ctx, &path_key_of(&type_path), &mut BTreeSet::new(), &mut BTreeSet::new()) {
        return DeclOutcome::Failed(DeclFailure {
            diag_id: Some("TypeAlias-Recursive-Err"),
            diagnostic_obligation_ids: ["def.AliasCycle", "TypeAlias-Recursive-Err", "req.TypeAliasDiagnosticOwnership"].map(String::from).to_vec(),
            ..Default::default()
        });
    }
    let type_params = decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..]);
    if let Err(diag_id) = process_generic_params(ctx, type_params) {
        return failed(diag_id);
    }
    match lower_type(ctx, &decl.r#type).and_then(|lowered| type_wf(ctx, &lowered)) {
        Ok(()) => DeclOutcome::Ok,
        Err(diag_id) => failed(diag_id),
    }
}

/// See `TypeStaticDecl`.
pub fn type_static_decl(ctx: &ScopeContext<'_>, decl: &ast::StaticDecl, module_path: &[String], diags: &mut DiagnosticStream) -> DeclOutcome {
    let attr_validation = validate_unsupported_attribute_target(decl.attrs_opt.as_deref().unwrap_or(&[]), "static declarations");
    if !attr_validation.ok {
        return failed(attr_validation.diag_id);
    }
    let ann_type = ast::binding_annotation_type_opt(&decl.binding);
    // A public static may not be mutable.
    let vis_error = decl.vis == ast::Visibility::Public && decl.r#mut == ast::Mutability::Var;
    if ann_type.is_none() {
        if vis_error {
            if let Some(diag) = make_diagnostic_by_id("E-MOD-2433", Some(decl.span.clone())) {
                emit(diags, diag);
            }
        }
        return failed(Some("E-TYP-1505"));
    }
    if vis_error {
        return failed(Some("E-MOD-2433"));
    }
    let lowered = match lower_type(ctx, &ann_type).and_then(|lowered| type_wf(ctx, &lowered).map(|()| lowered)) {
        Ok(lowered) => lowered,
        Err(diag_id) => return failed(diag_id),
    };
    if type_has_capabilities(ctx, module_path, &lowered) {
        return failed(Some("E-CON-0020"));
    }
    if decl.binding.init.is_none() {
        return failed(Some("Parse-Syntax-Err"));
    }
    let mut env = TypeEnv::default();
    env.scopes.push(Default::default());
    let type_ctx = StmtTypeContext { return_type: make_type_prim("()"), ..Default::default() };
    reset_scaffolding();
    let init_result = check_expr_against(ctx, &type_ctx, &decl.binding.init, &lowered, &env);
    if let Some(what) = take_pending() {
        return DeclOutcome::Pending(what.to_string());
    }
    if !init_result.ok {
        return failed(init_result.diag_id.or(Some("E-MOD-2402")));
    }
    DeclOutcome::Ok
}

fn normalize_attr_literal(value: String) -> String {
    let quoted = value.len() >= 2 && ((value.starts_with('"') && value.ends_with('"')) || (value.starts_with('\'') && value.ends_with('\'')));
    if quoted {
        value[1..value.len() - 1].to_string()
    } else {
        value
    }
}

fn has_impl(impls: &[Vec<String>], name: &str) -> bool {
    impls.iter().any(|impl_path| matches!(impl_path.as_slice(), [only] if only == name))
}

/// See `TypeEnumDecl`.
pub fn type_enum_decl(ctx: &ScopeContext<'_>, decl: &ast::EnumDecl, module_path: &[String]) -> DeclOutcome {
    let attr_validation = validate_attributes(&decl.attrs, AttributeTarget::Enum);
    if !attr_validation.ok {
        return failed(attr_validation.diag_id);
    }
    if get_attribute_value(&decl.attrs, attrs::LAYOUT, "").is_some_and(|kind| normalize_attr_literal(kind) == "packed") {
        return failed(Some("Attr-Packed-NonRecord"));
    }
    let type_path: Vec<String> = module_path.iter().cloned().chain([decl.name.clone()]).collect();
    let self_type = make_type_path(type_path.clone());
    let type_params = decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..]);
    if let Err(diag_id) = process_generic_params(ctx, type_params) {
        return failed(diag_id);
    }
    let mut impl_keys: Vec<_> = decl.implements.iter().map(|impl_path| path_key_of(impl_path)).collect();
    impl_keys.sort();
    if impl_keys.windows(2).any(|pair| pair[0] == pair[1]) {
        return failed(Some("E-TYP-2506"));
    }
    if has_impl(&decl.implements, "Bitcopy") && has_impl(&decl.implements, "Drop") {
        return failed(Some("E-TYP-2621"));
    }
    if has_impl(&decl.implements, "Hash") && !(eq_type_in(ctx, &self_type) || has_impl(&decl.implements, "Eq")) {
        return failed(Some("E-TYP-2503"));
    }
    if decl.variants.is_empty() {
        return DeclOutcome::Failed(DeclFailure {
            diag_id: Some("E-TYP-2001"),
            diagnostic_obligation_ids: ["Enum-Empty-Err", "diagnostics.Enums"].map(String::from).to_vec(),
            ..Default::default()
        });
    }
    let mut names = std::collections::HashSet::new();
    if !decl.variants.iter().all(|variant| names.insert(&variant.name)) {
        return failed(Some("E-TYP-2002"));
    }
    match enum_discriminants(decl) {
        Ok(discs) if discs.discs.len() != decl.variants.len() => return failed(Some("E-TYP-1921")),
        Ok(_) => {}
        Err(error) => return failed(Some(error.diag_id)),
    }
    let lower_payload = |ty: &ast::TypePtr| -> Result<TypeRef, Option<&'static str>> {
        let lowered = lower_type(ctx, ty).and_then(|lowered| type_wf(ctx, &lowered).map(|()| lowered))?;
        Ok(subst_self_type(&self_type, &lowered, None))
    };
    let mut payloads: Vec<TypeRef> = Vec::new();
    for variant in &decl.variants {
        match &variant.payload_opt {
            Some(ast::VariantPayload::VariantPayloadTuple(payload)) => {
                for payload_type in &payload.elements {
                    match lower_payload(payload_type) {
                        Ok(lowered) => payloads.push(lowered),
                        Err(diag_id) => return failed(diag_id),
                    }
                }
            }
            Some(ast::VariantPayload::VariantPayloadRecord(payload)) => {
                for field in &payload.fields {
                    if has_attribute(&field.attrs, attrs::DYNAMIC) {
                        return failed(Some("E-CON-0412"));
                    }
                    let field_attr_validation = validate_attributes(&field.attrs, AttributeTarget::Field);
                    if !field_attr_validation.ok {
                        return failed(field_attr_validation.diag_id);
                    }
                    match lower_payload(&field.r#type) {
                        Ok(lowered) => payloads.push(lowered),
                        Err(diag_id) => return failed(diag_id),
                    }
                }
            }
            None => {}
        }
    }
    if has_impl(&decl.implements, "Bitcopy") && payloads.iter().any(|payload| !bitcopy_type(ctx, payload)) {
        return failed(Some("E-TYP-2622"));
    }
    if let Some(invariant) = &decl.invariant_opt {
        let contract_ctx = ContractContext { scope_ctx: Some(ctx), receiver_type: self_type.clone(), ..Default::default() };
        let inv_result = check_type_invariant(&contract_ctx, invariant);
        if !inv_result.ok {
            return failed(inv_result.diag_id);
        }
    }
    for impl_path in &decl.implements {
        let Some(class_decl) = ctx.sigma.classes.get(&path_key_of(impl_path)) else {
            return failed(Some("Superclass-Undefined"));
        };
        if !check_orphan_rule(ctx, &type_path, impl_path, &ctx.current_module) {
            return failed(Some("E-TYP-2507"));
        }
        if is_modal_class(class_decl) {
            return failed(Some("E-TYP-2401"));
        }
        let methods = match class_method_table(ctx, impl_path) {
            Ok(table) => table,
            Err(diag_id) => return failed(Some(diag_id)),
        };
        let fields = match class_field_table(ctx, impl_path) {
            Ok(table) => table,
            Err(diag_id) => return failed(Some(diag_id)),
        };
        if !fields.is_empty() {
            return failed(Some("Impl-Field-Missing"));
        }
        if methods.iter().any(|entry| entry.method.body_opt.is_none()) {
            return failed(Some("E-TYP-2503"));
        }
    }
    DeclOutcome::Ok
}
