//! The type checker's entry point: every declaration of every module, then the passes
//! over the whole project.
//!
//! The port of declaration typing is in progress. A declaration of a kind that is not
//! ported is recorded as pending instead of being answered, and so is the tail of the
//! check (initialisation order, the `main` check) while anything before it is pending.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uv_core::behavior_model::{abort_on_error_count, default_error_recovery_policy};
use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream, Severity};
use uv_core::process_config::max_errors_override;
use uv_core::span::Span;
use uv_source::ast::{self, ASTItem};
use uv_source::attributes::{attrs, get_attribute_value, has_attribute};

use super::item_procedure::{type_procedure_decl, DeclOutcome};
use super::expr_store::TypeStores;
use super::item_class::type_class_decl;
use super::item_procedure::{build_procedure_signature, main_sig_ok, main_signature_fix_its};
use super::typecheck_diag::build_resolved_typecheck_diagnostic;
use crate::memory::init_planner::build_init_plan;
use uv_core::symbols::string_of_path;
use super::types::{make_type, type_key_of, ParamMode, TypeKey, TypeNode};
use crate::generics::generic_params::bind_type_params;
use crate::generics::monomorphize::{instantiate_type, TypeSubst};
use super::item_record::type_record_decl;
use super::item_simple::{type_enum_decl, type_static_decl, type_type_alias_decl};
use super::item_ffi::type_extern_block;
use super::item_modal::type_modal_decl;
use super::item_using::{type_import_decl, type_using_decl};
use super::typecheck_diag::{emit_decl_diag, emit_resolved_typecheck_diagnostic};
use crate::resolve::scopes::id_eq;

use crate::context::{NameMapTable, Scope, ScopeContext};
use crate::resolve::scopes::{path_key_of, universe_bindings};

/// A declaration the port cannot type yet, and what it waits for.
pub struct PendingItem {
    pub module: usize,
    pub item: usize,
    pub what: String,
}

#[derive(Default)]
pub struct TypecheckResult {
    pub ok: bool,
    pub diags: DiagnosticStream,
    pub has_init_plan: bool,
    pub pending_items: Vec<PendingItem>,
    /// Set when the passes over the whole project could not be run or trusted.
    pub pending_tail: Option<String>,
    /// The error limit was reached while declarations were typed.
    pub aborted: bool,
    /// What typing recorded about expressions, for the passes after it.
    pub stores: Option<Rc<TypeStores>>,
}

fn item_kind(item: &ASTItem) -> &'static str {
    match item {
        ASTItem::ProcedureDecl(_) => "procedure",
        ASTItem::ComptimeProcedureDecl(_) => "comptime procedure",
        ASTItem::DeriveTargetDecl(_) => "derive target",
        ASTItem::RecordDecl(_) => "record",
        ASTItem::EnumDecl(_) => "enum",
        ASTItem::ModalDecl(_) => "modal",
        ASTItem::ClassDecl(_) => "class",
        ASTItem::TypeAliasDecl(_) => "type",
        ASTItem::StaticDecl(_) => "static",
        ASTItem::ImportDecl(_) => "import",
        ASTItem::UsingDecl(_) => "using",
        ASTItem::ExternBlock(_) => "extern",
        ASTItem::ErrorItem(_) => "unknown",
    }
}

/// Types one declaration, reporting into `diags`. `Err` names what is not ported.
fn type_item(ctx: &ScopeContext<'_>, item: &ASTItem, module_path: &[String], shared: &Rc<RefCell<DiagnosticStream>>) -> Result<(), String> {
    if let ASTItem::ProcedureDecl(node) = item {
        return match type_procedure_decl(ctx, node, module_path, shared) {
            DeclOutcome::Ok => Ok(()),
            DeclOutcome::Pending(what) => Err(what),
            DeclOutcome::Failed(failure) => {
                let span = failure.diag_span.or_else(|| Some(node.span.clone()));
                emit_decl_diag(&mut shared.borrow_mut(), failure.diag_id, span, &failure.diag_detail, failure.children, &failure.diagnostic_obligation_ids);
                Ok(())
            }
        };
    }
    let with_diags = match item {
        ASTItem::RecordDecl(node) => Some((type_record_decl(ctx, node, module_path, shared), &node.span)),
        ASTItem::ClassDecl(node) => Some((type_class_decl(ctx, node, module_path, shared), &node.span)),
        ASTItem::ModalDecl(node) => Some((type_modal_decl(ctx, node, module_path, shared), &node.span)),
        _ => None,
    };
    if let Some((outcome, span)) = with_diags {
        return match outcome {
            DeclOutcome::Ok => Ok(()),
            DeclOutcome::Pending(what) => Err(what),
            DeclOutcome::Failed(failure) => {
                emit_decl_diag(&mut shared.borrow_mut(), failure.diag_id, Some(span.clone()), &failure.diag_detail, Vec::new(), &failure.diagnostic_obligation_ids);
                Ok(())
            }
        };
    }
    let outcome = match item {
        ASTItem::TypeAliasDecl(node) => Some((type_type_alias_decl(ctx, node, module_path), &node.span)),
        ASTItem::EnumDecl(node) => Some((type_enum_decl(ctx, node, module_path), &node.span)),
        ASTItem::StaticDecl(node) => Some((type_static_decl(ctx, node, module_path, &mut shared.borrow_mut()), &node.span)),
        _ => None,
    };
    if let Some((outcome, span)) = outcome {
        return match outcome {
            DeclOutcome::Ok => Ok(()),
            DeclOutcome::Pending(what) => Err(what),
            DeclOutcome::Failed(failure) => {
                // A static's failure carries neither detail nor obligations.
                let obligations = if matches!(item, ASTItem::StaticDecl(_)) { Vec::new() } else { failure.diagnostic_obligation_ids };
                emit_decl_diag(&mut shared.borrow_mut(), failure.diag_id, Some(span.clone()), "", Vec::new(), &obligations);
                Ok(())
            }
        };
    }
    let diags = &mut *shared.borrow_mut();
    match item {
        // The reference has no case for an item that failed to parse.
        ASTItem::ErrorItem(_) => Ok(()),
        ASTItem::ExternBlock(node) => match type_extern_block(ctx, node, module_path) {
            Err(what) => Err(what),
            Ok(diag_id) => {
                emit_decl_diag(diags, diag_id, Some(node.span.clone()), "", Vec::new(), &[]);
                Ok(())
            }
        },
        ASTItem::UsingDecl(node) => {
            if let Err(diag_id) = type_using_decl(ctx, node, module_path) {
                emit_decl_diag(diags, diag_id, Some(node.span.clone()), "", Vec::new(), &[]);
            }
            Ok(())
        }
        ASTItem::ImportDecl(node) => {
            if let Err(diag_id) = type_import_decl(ctx, node, module_path) {
                emit_decl_diag(diags, diag_id, Some(node.span.clone()), "", Vec::new(), &[]);
            }
            Ok(())
        }
        other => Err(item_kind(other).to_string()),
    }
}

fn normalize_attr_literal(value: String) -> String {
    let quoted = value.len() >= 2
        && ((value.starts_with('"') && value.ends_with('"')) || (value.starts_with('\'') && value.ends_with('\'')));
    if quoted {
        value[1..value.len() - 1].to_string()
    } else {
        value
    }
}

/// The symbol a `#mangle` attribute gives a procedure; `none` keeps its own name.
fn explicit_link_name(attr_list: &[ast::AttributeItem], name: &str) -> Option<String> {
    if !has_attribute(attr_list, attrs::MANGLE) {
        return None;
    }
    let mode_value = get_attribute_value(attr_list, attrs::MANGLE, "mode").or_else(|| get_attribute_value(attr_list, attrs::MANGLE, ""))?;
    let mode = normalize_attr_literal(mode_value);
    Some(if mode == "none" { name.to_string() } else { mode })
}

/// Two procedures of the project may not be given the same symbol.
fn emit_duplicate_symbol_diags(modules: &[ast::ASTModule], diags: &mut DiagnosticStream) {
    let mut seen_symbols = HashSet::new();
    let mut declare = |link_name: Option<String>, span: &Span, diags: &mut DiagnosticStream| {
        if let Some(link_name) = link_name.filter(|name| !name.is_empty()) {
            if !seen_symbols.insert(link_name) {
                emit_resolved_typecheck_diagnostic(diags, "E-SYS-3342", Some(span.clone()), "");
            }
        }
    };
    for module in modules {
        for item in &module.items {
            match item {
                ASTItem::ProcedureDecl(proc) => declare(explicit_link_name(&proc.attrs, &proc.name), &proc.span, diags),
                ASTItem::ComptimeProcedureDecl(proc) => declare(explicit_link_name(&proc.attrs, &proc.name), &proc.span, diags),
                _ => {}
            }
        }
        for item in &module.items {
            if let ASTItem::ExternBlock(block) = item {
                for ast::ExternItem::ExternProcDecl(proc) in &block.items {
                    declare(explicit_link_name(&proc.attrs, &proc.name), &proc.span, diags);
                }
            }
        }
    }
}

/// A procedure's signature with its generic parameters erased: the name, and for each
/// parameter its mode and type key.
struct ErasedOverloadSignature {
    name: String,
    params: Vec<(Option<ParamMode>, TypeKey)>,
}

fn build_erased_overload_signature(ctx: &ScopeContext<'_>, proc: &ast::ProcedureDecl) -> Option<ErasedOverloadSignature> {
    let mut proc_ctx = ctx.clone();
    proc_ctx.scopes = bind_type_params(ctx, &proc.generic_params);
    let sig = build_procedure_signature(&proc_ctx, &proc.params, &proc.return_type_opt).ok()?;
    let TypeNode::Func { params, .. } = &sig.func_type.as_deref()?.node else {
        return None;
    };
    let erased_param = make_type(TypeNode::Var(0));
    let erasure: TypeSubst = proc.generic_params.iter().flat_map(|params| &params.params).map(|param| (param.name.clone(), erased_param.clone())).collect();
    Some(ErasedOverloadSignature {
        name: proc.name.clone(),
        params: params.iter().map(|param| (param.mode, type_key_of(&instantiate_type(&param.r#type, &erasure)))).collect(),
    })
}

/// Two procedures of a module that differ only in their generic parameters are one
/// declaration twice.
fn emit_duplicate_erased_overload_signature_diags(ctx: &ScopeContext<'_>, module: &ast::ASTModule, diags: &mut DiagnosticStream) {
    let mut seen: Vec<ErasedOverloadSignature> = Vec::new();
    for item in &module.items {
        let ASTItem::ProcedureDecl(proc) = item else {
            continue;
        };
        if id_eq(&proc.name, "main") {
            continue;
        }
        let Some(signature) = build_erased_overload_signature(ctx, proc) else {
            continue;
        };
        if seen.iter().any(|other| id_eq(&other.name, &signature.name) && other.params == signature.params) {
            if let Some(mut diag) = make_diagnostic_by_id("E-MOD-1302", Some(proc.span.clone())) {
                diag.obligation_ids.push("Collect-Dup".to_string());
                emit(diags, diag);
            }
            emit_resolved_typecheck_diagnostic(diags, "E-SEM-3032", Some(proc.span.clone()), "");
        }
        seen.push(signature);
    }
}

fn count_error_diagnostics(diags: &DiagnosticStream) -> usize {
    diags.iter().filter(|diag| diag.severity == Severity::Error).count()
}

/// See `DeclTypingModules`.
fn decl_typing_modules(ctx: &mut ScopeContext<'_>, modules: &[ast::ASTModule], name_maps: &NameMapTable, result: &mut TypecheckResult) {
    let error_policy = max_errors_override().unwrap_or_else(default_error_recovery_policy);
    let shared = Rc::new(RefCell::new(Vec::new()));
    emit_duplicate_symbol_diags(modules, &mut shared.borrow_mut());
    if abort_on_error_count(&error_policy, count_error_diagnostics(&shared.borrow())) {
        result.aborted = true;
        result.diags = shared.take();
        return;
    }
    let universe_scope = universe_bindings();
    for (module_index, module) in modules.iter().enumerate() {
        ctx.current_module = module.path.clone();
        let module_scope = name_maps.get(&path_key_of(&module.path)).cloned().unwrap_or_default();
        ctx.scopes = vec![Scope::new(), module_scope, universe_scope.clone()];
        emit_duplicate_erased_overload_signature_diags(ctx, module, &mut shared.borrow_mut());
        if abort_on_error_count(&error_policy, count_error_diagnostics(&shared.borrow())) {
            result.aborted = true;
            result.diags = shared.take();
            return;
        }
        for (item_index, item) in module.items.iter().enumerate() {
            let typed = type_item(ctx, item, &module.path, &shared);
            if let Err(what) = typed {
                result.pending_items.push(PendingItem { module: module_index, item: item_index, what });
            }
            if abort_on_error_count(&error_policy, count_error_diagnostics(&shared.borrow())) {
                result.aborted = true;
                result.diags = shared.take();
                return;
            }
        }
    }
    result.diags = shared.take();
}

/// There is exactly one `main`, generic-free, with the signature of an entry point.
fn main_check_project(ctx: &mut ScopeContext<'_>, modules: &[ast::ASTModule], name_maps: &NameMapTable, diags: &mut DiagnosticStream) {
    let harness_entry_module = ctx.project.and_then(|project| project.test_harness_entry_module.clone());
    let mut mains: Vec<(&ast::ProcedureDecl, &Vec<String>)> = Vec::new();
    for module in modules {
        if harness_entry_module.as_ref().is_some_and(|entry| string_of_path(&module.path) != *entry) {
            continue;
        }
        for item in &module.items {
            if let ASTItem::ProcedureDecl(proc) = item {
                if id_eq(&proc.name, "main") {
                    mains.push((proc, &module.path));
                }
            }
        }
    }
    let Some(&(main_decl, main_module_path)) = mains.first() else {
        emit_resolved_typecheck_diagnostic(diags, "E-MOD-2434", None, "");
        return;
    };
    if mains.len() > 1 {
        emit_resolved_typecheck_diagnostic(diags, "E-MOD-2430", Some(main_decl.span.clone()), "");
        return;
    }
    if main_decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
        emit_resolved_typecheck_diagnostic(diags, "E-MOD-2432", Some(main_decl.span.clone()), "");
        return;
    }
    let saved_module = std::mem::replace(&mut ctx.current_module, main_module_path.clone());
    let module_scope = name_maps.get(&path_key_of(main_module_path)).cloned().unwrap_or_default();
    let saved_scopes = std::mem::replace(&mut ctx.scopes, vec![Scope::new(), module_scope, universe_bindings()]);
    if !main_sig_ok(ctx, main_decl) {
        let mut diag = build_resolved_typecheck_diagnostic("E-MOD-2431", Some(main_decl.span.clone()));
        diag.children.extend(main_signature_fix_its(main_decl));
        emit(diags, diag);
    }
    ctx.current_module = saved_module;
    ctx.scopes = saved_scopes;
}

/// See `TypecheckModules`. The modules are the context's own.
pub fn typecheck_modules(ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) -> TypecheckResult {
    let mut result = TypecheckResult::default();
    let sigma = ctx.sigma.clone();
    // What typing records about expressions lives for the whole check, and is read
    // back by it; the previous stores return afterwards.
    let prev_stores = ctx.stores.replace(Rc::new(TypeStores::default()));
    decl_typing_modules(ctx, &sigma.mods, name_maps, &mut result);
    result.stores = std::mem::replace(&mut ctx.stores, prev_stores);
    if !result.pending_items.is_empty() {
        result.pending_tail = Some("declarations".to_string());
        result.ok = !uv_core::diagnostics::has_error(&result.diags);
        return result;
    }
    // What follows runs only when the declarations raised no error.
    if !uv_core::diagnostics::has_error(&result.diags) {
        let init_plan = build_init_plan(ctx, name_maps);
        result.diags.extend(init_plan.diags);
        result.has_init_plan = init_plan.ok;
    }
    if !uv_core::diagnostics::has_error(&result.diags) && ctx.project.is_none_or(|project| project.assembly.is_executable()) {
        main_check_project(ctx, &sigma.mods, name_maps, &mut result.diags);
    }
    result.ok = !uv_core::diagnostics::has_error(&result.diags);
    result
}
