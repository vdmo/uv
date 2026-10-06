//! Which types compile-time code may take and return, and the check of the signatures of
//! compile-time procedures against it. See `CtAvailType`, `CtForbiddenTypeDiag` and
//! `ValidateComptimeProcedureSignatures`.

use std::collections::BTreeSet;

use uv_core::diagnostics::DiagnosticStream;
use uv_source::ast;

use super::item_procedure::build_procedure_signature;
use super::type_lookup::{lookup_enum_decl, lookup_record_decl};
use super::type_lower::lower_type;
use super::type_predicates::strip_perm;
use super::typecheck_diag::emit_decl_diag;
use super::types::{BytesState, StringState, TypeNode, TypeRef};
use crate::caps::cap_requirements::infer_capabilities_from_type;
use crate::context::ScopeContext;
use crate::generics::monomorphize::apply_generic_substitution;
use crate::resolve::collect_toplevel::collect_name_maps;
use crate::resolve::scopes::{path_key_of, universe_bindings};

fn is_comptime_meta_type_path(path: &[String]) -> bool {
    let parts: Vec<&str> = path.iter().map(String::as_str).collect();
    matches!(parts.as_slice(), ["Type"] | ["Ast"] | ["Ast", "Expr"] | ["Ast", "Stmt"] | ["Ast", "Item"] | ["Ast", "Type"] | ["Ast", "Pattern"])
}

/// The arguments a declaration is applied to: those given, then the defaults of the rest.
fn resolve_decl_generic_args(ctx: &ScopeContext<'_>, generic_params: &Option<ast::GenericParams>, provided: &[TypeRef]) -> Option<Vec<TypeRef>> {
    let Some(params) = generic_params else {
        return provided.is_empty().then(Vec::new);
    };
    if provided.len() > params.params.len() {
        return None;
    }
    let mut out = Vec::with_capacity(params.params.len());
    for (index, param) in params.params.iter().enumerate() {
        if let Some(arg) = provided.get(index) {
            out.push(arg.clone());
            continue;
        }
        param.default_type.as_ref()?;
        let lowered = lower_type(ctx, &param.default_type).ok().filter(|ty| ty.is_some())?;
        out.push(lowered);
    }
    Some(out)
}

fn param_names(generic_params: &Option<ast::GenericParams>) -> Vec<String> {
    generic_params.iter().flat_map(|params| params.params.iter().map(|param| param.name.clone())).collect()
}

/// Whether the member types of a declaration, with the arguments applied, are all available.
fn members_available(ctx: &ScopeContext<'_>, written: &[&ast::TypePtr], names: &[String], args: &[TypeRef], active: &mut BTreeSet<String>) -> bool {
    written.iter().all(|ty| {
        let Some(lowered) = lower_type(ctx, ty).ok().filter(|lowered| lowered.is_some()) else {
            return false;
        };
        ct_avail_type(ctx, &strip_perm(&apply_generic_substitution(&lowered, names, args)), active)
    })
}

fn ct_avail_type(ctx: &ScopeContext<'_>, ty: &TypeRef, active: &mut BTreeSet<String>) -> bool {
    let Some(node) = ty.as_deref().map(|ty| &ty.node) else {
        return false;
    };
    match node {
        TypeNode::Prim(_) => true,
        TypeNode::String(state) => matches!(state, Some(StringState::View | StringState::Managed)),
        TypeNode::Bytes(state) => matches!(state, Some(BytesState::View | BytesState::Managed)),
        TypeNode::Tuple(elements) => elements.iter().all(|elem| ct_avail_type(ctx, elem, active)),
        TypeNode::Array { element, .. } | TypeNode::Slice(element) => ct_avail_type(ctx, element, active),
        TypeNode::Perm { base, .. } => ct_avail_type(ctx, base, active),
        TypeNode::Path { path, generic_args } => {
            if is_comptime_meta_type_path(path) {
                return true;
            }
            let key = path.join("::");
            if !active.insert(key.clone()) {
                return false;
            }
            let available = if let Some(record) = lookup_record_decl(ctx, path) {
                let fields: Vec<&ast::TypePtr> = record
                    .members
                    .iter()
                    .filter_map(|member| match member {
                        ast::RecordMember::FieldDecl(field) if field.r#type.is_some() => Some(&field.r#type),
                        _ => None,
                    })
                    .collect();
                resolve_decl_generic_args(ctx, &record.generic_params, generic_args)
                    .is_some_and(|args| members_available(ctx, &fields, &param_names(&record.generic_params), &args, active))
            } else if let Some(decl) = lookup_enum_decl(ctx, path) {
                let payloads: Vec<&ast::TypePtr> = decl
                    .variants
                    .iter()
                    .flat_map(|variant| match &variant.payload_opt {
                        None => Vec::new(),
                        Some(ast::VariantPayload::VariantPayloadTuple(tuple)) => tuple.elements.iter().collect(),
                        Some(ast::VariantPayload::VariantPayloadRecord(record)) => record.fields.iter().map(|field| &field.r#type).collect(),
                    })
                    .collect();
                resolve_decl_generic_args(ctx, &decl.generic_params, generic_args)
                    .is_some_and(|args| members_available(ctx, &payloads, &param_names(&decl.generic_params), &args, active))
            } else {
                false
            };
            active.remove(&key);
            available
        }
        _ => false,
    }
}

/// `E-CTE-0012` for a type that carries a capability, `E-CTE-0011` for one compile-time
/// code may not hold at all.
fn ct_forbidden_type_diag(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<&'static str> {
    ty.as_ref()?;
    if !infer_capabilities_from_type(ctx, &ctx.current_module, ty).is_empty() {
        return Some("E-CTE-0012");
    }
    match strip_perm(ty).as_deref().map(|ty| &ty.node)? {
        TypeNode::ModalState(_) | TypeNode::Dynamic(_) | TypeNode::Ptr { .. } | TypeNode::RawPtr { .. } | TypeNode::Func { .. } => Some("E-CTE-0011"),
        _ => None,
    }
}

/// The diagnostic for a type compile-time code may not use, or `unavailable_diag_id` when
/// it is merely not available there.
pub fn comptime_type_availability_diag(ctx: &ScopeContext<'_>, ty: &TypeRef, unavailable_diag_id: &'static str) -> Option<&'static str> {
    if let Some(forbidden) = ct_forbidden_type_diag(ctx, ty) {
        return Some(forbidden);
    }
    (!ct_avail_type(ctx, ty, &mut BTreeSet::new())).then_some(unavailable_diag_id)
}

/// The types in the signature of a compile-time procedure: `Err(Some(..))` is the diagnostic.
fn validate_signature_types(ctx: &ScopeContext<'_>, decl: &ast::ComptimeProcedureDecl) -> Option<(Option<&'static str>, uv_core::span::Span)> {
    let sig = match build_procedure_signature(ctx, &decl.params, &decl.return_type_opt) {
        Ok(sig) => sig,
        Err(diag_id) => return Some((diag_id, decl.span.clone())),
    };
    if let Some(TypeNode::Func { params, .. }) = sig.func_type.as_deref().map(|ty| &ty.node) {
        for (param, written) in params.iter().zip(&decl.params) {
            if let Some(diag) = comptime_type_availability_diag(ctx, &param.r#type, "E-CTE-0030") {
                let span = written.r#type.as_deref().map(|ty| ty.span.clone()).unwrap_or_else(|| written.span.clone());
                return Some((Some(diag), span));
            }
        }
    }
    if let Some(diag) = comptime_type_availability_diag(ctx, &sig.return_type, "E-CTE-0031") {
        let span = decl.return_type_opt.as_deref().map(|ty| ty.span.clone()).unwrap_or_else(|| decl.span.clone());
        return Some((Some(diag), span));
    }
    None
}

/// Checks the signature of every compile-time procedure, before any of them runs.
pub fn validate_comptime_procedure_signatures(ctx: &mut ScopeContext<'_>, modules: &[ast::ASTModule]) -> DiagnosticStream {
    let mut diags = DiagnosticStream::new();
    let collected = collect_name_maps(ctx);
    for diag in collected.diags {
        uv_core::diagnostics::emit(&mut diags, diag);
    }
    if uv_core::diagnostics::has_error(&diags) {
        return diags;
    }
    let universe = universe_bindings();
    for module in modules {
        ctx.current_module = module.path.clone();
        let module_scope = collected.name_maps.get(&path_key_of(&module.path)).cloned().unwrap_or_default();
        ctx.scopes = vec![Default::default(), module_scope, universe.clone()];
        for item in &module.items {
            let ast::ASTItem::ComptimeProcedureDecl(node) = item else {
                continue;
            };
            if let Some((diag_id, span)) = validate_signature_types(ctx, node) {
                emit_decl_diag(&mut diags, diag_id, Some(span), "", Vec::new(), &[]);
            }
        }
    }
    diags
}
