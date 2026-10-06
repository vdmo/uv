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
use uv_core::diagnostics::{DiagnosticStream, Severity};
use uv_core::process_config::max_errors_override;
use uv_core::span::Span;
use uv_source::ast::{self, ASTItem};
use uv_source::attributes::{attrs, get_attribute_value, has_attribute};

use super::item_procedure::{type_procedure_decl, DeclOutcome};
use super::expr_store::TypeStores;
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
                emit_decl_diag(&mut shared.borrow_mut(), failure.diag_id, span, &failure.diag_detail, Vec::new(), &failure.diagnostic_obligation_ids);
                Ok(())
            }
        };
    }
    let diags = &mut *shared.borrow_mut();
    match item {
        // The reference has no case for an item that failed to parse.
        ASTItem::ErrorItem(_) => Ok(()),
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
        // Procedures of one name whose signatures erase to the same one are reported
        // before the module's declarations. That pass is not ported; it can only
        // concern procedures that share a name.
        let mut names_seen: Vec<&str> = Vec::new();
        let mut shared_names: Vec<&str> = Vec::new();
        for item in &module.items {
            if let ASTItem::ProcedureDecl(proc) = item {
                if id_eq(&proc.name, "main") {
                    continue;
                }
                if names_seen.iter().any(|seen| id_eq(seen, &proc.name)) {
                    shared_names.push(&proc.name);
                }
                names_seen.push(&proc.name);
            }
        }
        for (item_index, item) in module.items.iter().enumerate() {
            let overloaded = matches!(item, ASTItem::ProcedureDecl(proc) if shared_names.iter().any(|name| id_eq(name, &proc.name)));
            let typed = if overloaded { Err("ErasedOverloads".to_string()) } else { type_item(ctx, item, &module.path, &shared) };
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

/// See `TypecheckModules`. The modules are the context's own.
pub fn typecheck_modules(ctx: &mut ScopeContext<'_>, name_maps: &NameMapTable) -> TypecheckResult {
    let mut result = TypecheckResult::default();
    let sigma = ctx.sigma.clone();
    // What typing records about expressions lives for the whole check, and is read
    // back by it; the previous stores return afterwards.
    let prev_stores = ctx.stores.replace(Rc::new(TypeStores::default()));
    decl_typing_modules(ctx, &sigma.mods, name_maps, &mut result);
    result.stores = std::mem::replace(&mut ctx.stores, prev_stores);
    if !result.pending_items.is_empty() && result.pending_tail.is_none() {
        result.pending_tail = Some("declarations".to_string());
    }
    if result.pending_tail.is_none() {
        result.pending_tail = Some("InitPlan".to_string());
    }
    result.ok = !uv_core::diagnostics::has_error(&result.diags);
    result
}
