//! The foreign-interface attributes of procedures (`export`, `host_export`, `mangle`,
//! `unwind`) and what they demand of the declaration and its signature. See
//! `ValidateProcedureFfiAttributes` and the export checks of `TypeProcedureDecl`.

use uv_core::diagnostics::DiagnosticStream;
use uv_project::target_profile::TargetProfile;
use uv_source::ast::{self, AttributeArgValue, AttributeItem};
use uv_project::target_profile::library_kind_supported;
use uv_source::attributes::{attrs, get_attribute_value, has_attribute, validate_attributes, AttributeTarget};
use uv_source::lexer::token::TokenKind;

use super::item_procedure::Signature;
use super::type_predicates::{ffi_by_value_ok, ffi_safe_diag_for_type, ffi_safe_type, zeroable_type};
use super::typecheck_diag::emit_resolved_typecheck_diagnostic;
use super::type_lower::lower_type;
use super::type_wf::{type_wf, REFINEMENT_WF_PENDING};
use super::types::{PtrState, TypeNode, TypeRef};
use crate::caps::cap_requirements::type_has_capabilities;
use crate::caps::context_caps::{is_context_bundle_type, is_hosted_context_bundle_type};
use crate::context::ScopeContext;
use crate::resolve::scopes::id_eq;

fn normalize_attr_literal(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"') || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'')) {
        return value[1..value.len() - 1].to_string();
    }
    value.to_string()
}

#[derive(Default)]
struct UnwindAttrCheck {
    has_attr: bool,
    duplicate: bool,
    invalid: bool,
    mode: String,
}

fn check_unwind_attr(attr_list: &[AttributeItem]) -> UnwindAttrCheck {
    let mut check = UnwindAttrCheck::default();
    let unwind: Vec<_> = attr_list.iter().filter(|attr| id_eq(&attr.name.full_name, attrs::UNWIND)).collect();
    let Some(attr) = unwind.first() else {
        return check;
    };
    check.has_attr = true;
    if unwind.len() > 1 {
        check.duplicate = true;
        return check;
    }
    let [arg] = &attr.args[..] else {
        check.invalid = true;
        return check;
    };
    let token = match (&arg.key, &arg.value) {
        (None, AttributeArgValue::Token(token)) if token.kind == TokenKind::StringLiteral => token,
        _ => {
            check.invalid = true;
            return check;
        }
    };
    let mode = normalize_attr_literal(&token.lexeme);
    if mode != "abort" && mode != "catch" {
        check.invalid = true;
        return check;
    }
    check.mode = mode;
    check
}

fn is_catch_unwind(attr_list: &[AttributeItem]) -> bool {
    let check = check_unwind_attr(attr_list);
    check.has_attr && !check.duplicate && !check.invalid && check.mode == "catch"
}

#[derive(Default)]
struct MangleAttrCheck {
    has_attr: bool,
    invalid: bool,
    conflicting: bool,
    none_mode: bool,
    explicit_name: String,
}

fn check_mangle_attr(attr_list: &[AttributeItem]) -> MangleAttrCheck {
    let mut check = MangleAttrCheck::default();
    let mut has_none_mode = false;
    let mut symbol_mode_value: Option<String> = None;
    for attr in attr_list.iter().filter(|attr| id_eq(&attr.name.full_name, attrs::MANGLE)) {
        check.has_attr = true;
        let [arg] = &attr.args[..] else {
            check.invalid = true;
            return check;
        };
        if arg.key.as_deref().is_some_and(|key| key != "mode") {
            check.invalid = true;
            return check;
        }
        let AttributeArgValue::Token(token) = &arg.value else {
            check.invalid = true;
            return check;
        };
        let raw = normalize_attr_literal(&token.lexeme);
        if raw.is_empty() {
            check.invalid = true;
            return check;
        }
        let is_string = token.kind == TokenKind::StringLiteral;
        if raw == "none" && !is_string {
            if symbol_mode_value.is_some() {
                check.conflicting = true;
                return check;
            }
            has_none_mode = true;
        } else if is_string {
            if has_none_mode || symbol_mode_value.as_ref().is_some_and(|value| *value != raw) {
                check.conflicting = true;
                return check;
            }
            symbol_mode_value = Some(raw);
        } else {
            check.invalid = true;
            return check;
        }
    }
    if has_none_mode {
        check.none_mode = true;
    } else if let Some(name) = symbol_mode_value {
        check.explicit_name = name;
    }
    check
}

fn is_valid_ffi_abi(abi: &str) -> bool {
    matches!(abi, "C" | "C-unwind" | "system" | "stdcall" | "fastcall" | "vectorcall")
}

fn is_supported_ffi_abi_for_profile(abi: &str, profile: TargetProfile) -> bool {
    !matches!(abi, "stdcall" | "fastcall" | "vectorcall") || profile == TargetProfile::X86_64Win64
}

fn abi_value(attr_list: &[AttributeItem], name: &str) -> Option<String> {
    get_attribute_value(attr_list, name, "").map(|abi| normalize_attr_literal(&abi))
}

fn current_assembly<'c>(ctx: &'c ScopeContext<'_>) -> Option<&'c uv_project::project::Assembly> {
    let project = ctx.project?;
    if let Some(first) = ctx.current_module.first() {
        if let Some(assembly) = project.assemblies.iter().find(|assembly| id_eq(&assembly.name, first)) {
            return Some(assembly);
        }
    }
    Some(&project.assembly)
}

fn current_assembly_name(ctx: &ScopeContext<'_>) -> String {
    match current_assembly(ctx) {
        Some(assembly) => assembly.name.clone(),
        None => ctx.current_module.first().cloned().unwrap_or_default(),
    }
}

fn assembly_contains_procedure_attr(ctx: &ScopeContext<'_>, assembly_name: &str, attr_name: &str) -> bool {
    ctx.sigma.mods.iter().filter(|module| module.path.first().is_some_and(|first| id_eq(first, assembly_name))).any(|module| {
        module.items.iter().any(|item| matches!(item, ast::ASTItem::ProcedureDecl(proc) if has_attribute(&proc.attrs, attr_name)))
    })
}

fn assembly_has_mixed_foreign_export_modes(ctx: &ScopeContext<'_>) -> bool {
    let name = current_assembly_name(ctx);
    !name.is_empty() && assembly_contains_procedure_attr(ctx, &name, attrs::EXPORT) && assembly_contains_procedure_attr(ctx, &name, attrs::HOST_EXPORT)
}

/// The attributes a foreign-facing procedure may carry, and where; the diagnostic of the
/// first violation.
pub fn validate_procedure_ffi_attributes(ctx: &ScopeContext<'_>, decl: &ast::ProcedureDecl) -> Option<&'static str> {
    let has_export = has_attribute(&decl.attrs, attrs::EXPORT);
    let has_host_export = has_attribute(&decl.attrs, attrs::HOST_EXPORT);
    let has_foreign_export = has_export || has_host_export;
    let mangle = check_mangle_attr(&decl.attrs);
    let unwind = check_unwind_attr(&decl.attrs);
    let foreign_abi = if has_host_export { abi_value(&decl.attrs, attrs::HOST_EXPORT) } else { abi_value(&decl.attrs, attrs::EXPORT) };

    if has_foreign_export && decl.vis != ast::Visibility::Public {
        return Some("E-SYS-3353");
    }
    if has_foreign_export {
        let Some(profile) = ctx.target_profile else {
            return Some("Internal-MissingTargetProfile");
        };
        match &foreign_abi {
            Some(abi) if !abi.is_empty() && is_valid_ffi_abi(abi) && is_supported_ffi_abi_for_profile(abi, profile) => {}
            _ => return Some("E-SYS-3352"),
        }
    }
    if mangle.has_attr {
        if mangle.conflicting {
            return Some("E-SYS-3351");
        }
        if mangle.invalid {
            return Some("E-SYS-3341");
        }
        if !has_foreign_export {
            return Some(if mangle.none_mode { "E-SYS-3350" } else { "E-SYS-3340" });
        }
        if !mangle.none_mode && mangle.explicit_name.is_empty() {
            return Some("E-SYS-3341");
        }
    }
    if unwind.has_attr {
        if !has_foreign_export {
            return Some("E-SYS-3356");
        }
        if unwind.duplicate {
            return Some("E-FFI-0350");
        }
        if unwind.invalid {
            return Some("E-SYS-3355");
        }
        if unwind.mode == "catch" && foreign_abi.as_deref() != Some("C-unwind") {
            return Some("E-SYS-3355");
        }
    }
    if has_foreign_export && assembly_has_mixed_foreign_export_modes(ctx) {
        return Some("E-SYS-3358");
    }
    if has_host_export {
        if current_assembly(ctx).is_none_or(|assembly| assembly.kind != "library") {
            return Some("E-SYS-3357");
        }
        if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
            return Some("E-TYP-2634");
        }
        let context = decl.params.first().filter(|param| param.r#type.is_some());
        if !context.is_some_and(|param| is_context_bundle_type(ctx, &param.r#type)) {
            return Some("E-TYP-2632");
        }
        let context = context?;
        if !is_hosted_context_bundle_type(ctx, &context.r#type) {
            return Some("E-TYP-2636");
        }
        if context.mode.is_some() {
            return Some("E-TYP-2633");
        }
    }
    None
}

/// The warnings the foreign attributes call for, once they are valid.
pub fn emit_procedure_ffi_warnings(decl: &ast::ProcedureDecl, diags: &mut DiagnosticStream) {
    let has_export = has_attribute(&decl.attrs, attrs::EXPORT);
    let has_foreign_export = has_export || has_attribute(&decl.attrs, attrs::HOST_EXPORT);
    let mangle = check_mangle_attr(&decl.attrs);
    if has_export && mangle.has_attr && !mangle.invalid && mangle.none_mode && abi_value(&decl.attrs, attrs::EXPORT).is_some_and(|abi| abi == "C") {
        emit_resolved_typecheck_diagnostic(diags, "W-SYS-3350", Some(decl.span.clone()), "");
    }
    let unwind = check_unwind_attr(&decl.attrs);
    if has_foreign_export && unwind.has_attr && !unwind.duplicate && !unwind.invalid && unwind.mode == "abort" {
        emit_resolved_typecheck_diagnostic(diags, "W-SYS-3355", Some(decl.span.clone()), "");
    }
}

/// What the signature of an exported procedure must satisfy: foreign-safe types, no
/// capabilities, values that may cross by value, and a zeroable result when unwinding is caught.
pub fn check_export_signature(ctx: &ScopeContext<'_>, module_path: &[String], decl: &ast::ProcedureDecl, sig: &Signature) -> Option<&'static str> {
    let has_export = has_attribute(&decl.attrs, attrs::EXPORT);
    let has_host_export = has_attribute(&decl.attrs, attrs::HOST_EXPORT);
    if !has_export && !has_host_export {
        return None;
    }
    let unsafe_diag = |ty: &TypeRef| {
        (!ffi_safe_type(ctx, ty)).then(|| ffi_safe_diag_for_type(ctx, ty).unwrap_or("E-TYP-2623"))
    };
    if let Some(diag) = unsafe_diag(&sig.return_type) {
        return Some(diag);
    }
    if type_has_capabilities(ctx, module_path, &sig.return_type) {
        return Some("E-TYP-2623");
    }
    let visible_begin = usize::from(has_host_export);
    let Some(TypeNode::Func { params, .. }) = sig.func_type.as_deref().map(|ty| &ty.node) else {
        return Some("E-TYP-2623");
    };
    for param in params.iter().skip(visible_begin) {
        if let Some(diag) = unsafe_diag(&param.r#type) {
            return Some(diag);
        }
        if type_has_capabilities(ctx, module_path, &param.r#type) {
            return Some("E-TYP-2623");
        }
    }
    let by_value_ok = ffi_by_value_ok(ctx, &sig.return_type) && params.iter().skip(visible_begin).all(|param| ffi_by_value_ok(ctx, &param.r#type));
    if !by_value_ok {
        return Some("E-TYP-2630");
    }
    if is_catch_unwind(&decl.attrs) && !zeroable_type(ctx, &sig.return_type) {
        return Some(if has_host_export { "E-TYP-2635" } else { "Export-Return-NotZeroable-Err" });
    }
    None
}

// ---- extern blocks -------------------------------------------------------------------

/// The name and kind a `library` attribute on an extern block declares.
fn normalize_library_attribute(attr: &AttributeItem) -> Option<(String, String)> {
    if attr.name.full_name != "library" {
        return None;
    }
    let (mut name, mut kind): (Option<String>, Option<String>) = (None, None);
    for (i, arg) in attr.args.iter().enumerate() {
        let key = arg.key.as_deref()?;
        let AttributeArgValue::Token(token) = &arg.value else {
            return None;
        };
        let normalized = normalize_attr_literal(&token.lexeme);
        match key {
            "name" => {
                if name.is_some() || normalized.is_empty() || i != 0 {
                    return None;
                }
                name = Some(normalized);
            }
            "kind" => {
                if kind.is_some() || normalized.is_empty() || name.is_none() || i != 1 {
                    return None;
                }
                kind = Some(normalized);
            }
            _ => return None,
        }
    }
    let name = name.filter(|name| !name.is_empty())?;
    Some((name, kind.unwrap_or_else(|| "dylib".to_string())))
}

fn extern_abi_string(abi: &Option<ast::ExternAbi>) -> String {
    match abi {
        None => "C".to_string(),
        Some(ast::ExternAbi::ExternAbiString(abi)) => normalize_attr_literal(&abi.literal.lexeme),
        Some(ast::ExternAbi::ExternAbiIdent(abi)) => abi.name.clone(),
    }
}

/// The type of an extern signature: lowered, well formed, and when it is not, the foreign
/// diagnostic for it if there is one. `Err(Err(_))` is a check that is not ported.
fn lower_extern_signature_type(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Result<TypeRef, Result<Option<&'static str>, ()>> {
    let lowered = lower_type(ctx, ty).map_err(Ok)?;
    match type_wf(ctx, &lowered) {
        Ok(()) => Ok(lowered),
        Err(Some(REFINEMENT_WF_PENDING)) => Err(Err(())),
        Err(diag_id) => match ffi_safe_diag_for_type(ctx, &lowered) {
            Some("E-TYP-2629") => Err(Ok(Some("E-TYP-2629"))),
            _ => Err(Ok(diag_id)),
        },
    }
}

/// The first violation of a foreign predicate: names it may use, `result` only in an
/// `ensures`, and nothing but pure forms.
fn validate_foreign_predicate(expr: &ast::ExprPtr, allowed: &[&str], allow_result: bool, impurity_diag: &'static str) -> Option<&'static str> {
    use ast::ExprNode as E;
    let expr = expr.as_deref()?;
    match &expr.node {
        E::LiteralExpr(_) | E::PtrNullExpr(_) | E::TupleExpr(_) => None,
        E::IdentifierExpr(node) => (!allowed.iter().any(|name| id_eq(&node.name, name))).then_some("E-SEM-2852"),
        E::ResultExpr(_) => (!allow_result).then_some("E-SEM-2854"),
        E::BinaryExpr(node) => validate_foreign_predicate(&node.lhs, allowed, allow_result, impurity_diag).or_else(|| validate_foreign_predicate(&node.rhs, allowed, allow_result, impurity_diag)),
        E::UnaryExpr(node) => validate_foreign_predicate(&node.value, allowed, allow_result, impurity_diag),
        E::FieldAccessExpr(node) => validate_foreign_predicate(&node.base, allowed, allow_result, impurity_diag),
        _ => Some(impurity_diag),
    }
}

fn is_unit_type_for_foreign_contracts(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. }) => is_unit_type_for_foreign_contracts(base),
        Some(TypeNode::Prim(name)) => name == "()",
        _ => false,
    }
}

fn is_nullable_foreign_ptr_type(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. }) => is_nullable_foreign_ptr_type(base),
        Some(TypeNode::Ptr { state, .. }) => *state == Some(PtrState::Null),
        Some(TypeNode::RawPtr { .. }) => true,
        _ => false,
    }
}

/// `Ok(Some(diag))` is a failure, `Err(what)` something not ported.
type ExternCheck = Result<Option<&'static str>, String>;

fn check_extern_proc(ctx: &ScopeContext<'_>, block_abi: &str, proc: &ast::ExternProcDecl, module_path: &[String]) -> ExternCheck {
    let attr_validation = validate_attributes(&proc.attrs, AttributeTarget::Procedure);
    if !attr_validation.ok {
        return Ok(attr_validation.diag_id);
    }
    let mangle = check_mangle_attr(&proc.attrs);
    if mangle.conflicting {
        return Ok(Some("E-SYS-3351"));
    }
    if mangle.invalid {
        return Ok(Some("E-SYS-3341"));
    }
    let unwind = check_unwind_attr(&proc.attrs);
    if unwind.duplicate {
        return Ok(Some("E-FFI-0350"));
    }
    if unwind.invalid || (unwind.has_attr && unwind.mode == "catch" && block_abi != "C-unwind") {
        return Ok(Some("E-SYS-3355"));
    }
    if proc.return_type_opt.is_none() {
        return Ok(Some("WF-ExternProcDecl-MissingReturnType"));
    }
    if proc.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
        return Ok(Some("E-TYP-2306"));
    }
    let boundary = |ty: &TypeRef| -> Option<&'static str> {
        if !ffi_safe_type(ctx, ty) {
            return Some(ffi_safe_diag_for_type(ctx, ty).unwrap_or("E-TYP-2623"));
        }
        if type_has_capabilities(ctx, module_path, ty) {
            return Some("E-TYP-2623");
        }
        (!ffi_by_value_ok(ctx, ty)).then_some("E-TYP-2630")
    };
    let lower = |ty: &ast::TypePtr| match lower_extern_signature_type(ctx, ty) {
        Ok(lowered) => Ok(Ok(lowered)),
        Err(Ok(diag_id)) => Ok(Err(diag_id)),
        Err(Err(())) => Err("RefinementWF".to_string()),
    };
    for param in &proc.params {
        match lower(&param.r#type)? {
            Err(diag_id) => return Ok(diag_id),
            Ok(lowered) => {
                if let Some(diag_id) = boundary(&lowered) {
                    return Ok(Some(diag_id));
                }
            }
        }
    }
    let return_type = match lower(&proc.return_type_opt)? {
        Err(diag_id) => return Ok(diag_id),
        Ok(lowered) => lowered,
    };
    if let Some(diag_id) = boundary(&return_type) {
        return Ok(Some(diag_id));
    }
    for clause in proc.foreign_contracts_opt.iter().flatten() {
        let is_assumes = clause.kind == ast::ForeignContractKind::Assumes;
        let impurity = if is_assumes { "E-SEM-2851" } else { "E-SEM-2853" };
        let names: Vec<&str> = proc.params.iter().map(|param| param.name.as_str()).collect();
        for predicate in &clause.predicates {
            if let Some(diag_id) = validate_foreign_predicate(predicate, &names, !is_assumes, impurity) {
                return Ok(Some(diag_id));
            }
        }
        match clause.kind {
            ast::ForeignContractKind::Assumes => {}
            ast::ForeignContractKind::EnsuresError if is_unit_type_for_foreign_contracts(&return_type) => return Ok(Some("E-SEM-2855")),
            ast::ForeignContractKind::EnsuresNullResult if !is_nullable_foreign_ptr_type(&return_type) => return Ok(Some("E-SEM-2856")),
            _ => {}
        }
    }
    Ok(None)
}

/// See `TypeExternBlock`: the block's attributes, library kinds, ABI and unwind mode,
/// then each procedure's signature.
pub fn type_extern_block(ctx: &ScopeContext<'_>, block: &ast::ExternBlock, module_path: &[String]) -> ExternCheck {
    let block_attrs = block.attrs_opt.as_deref().unwrap_or(&[]);
    let abi = extern_abi_string(&block.abi_opt);
    let attr_validation = validate_attributes(block_attrs, AttributeTarget::ExternBlock);
    if !attr_validation.ok {
        return Ok(attr_validation.diag_id);
    }
    let Some(profile) = ctx.target_profile else {
        return Ok(Some("Internal-MissingTargetProfile"));
    };
    for attr in block_attrs {
        if let Some((_, kind)) = normalize_library_attribute(attr) {
            if !library_kind_supported(&kind, profile) {
                return Ok(Some("E-SYS-3346"));
            }
        }
    }
    if !is_valid_ffi_abi(&abi) || !is_supported_ffi_abi_for_profile(&abi, profile) {
        return Ok(Some("E-SYS-3352"));
    }
    let unwind = check_unwind_attr(block_attrs);
    if unwind.duplicate {
        return Ok(Some("E-FFI-0350"));
    }
    if unwind.invalid || (unwind.has_attr && unwind.mode == "catch" && abi != "C-unwind") {
        return Ok(Some("E-SYS-3355"));
    }
    for ast::ExternItem::ExternProcDecl(proc) in &block.items {
        if let Some(diag_id) = check_extern_proc(ctx, &abi, proc, module_path)? {
            return Ok(Some(diag_id));
        }
    }
    Ok(None)
}
