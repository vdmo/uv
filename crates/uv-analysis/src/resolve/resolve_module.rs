//! Resolving whole modules, and turning resolver failures into diagnostics.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, has_error, Diagnostic, DiagnosticStream, Severity, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;
use uv_source::ast::*;

use super::collect_toplevel::collect_names;
use super::resolve_items::resolve_item;
use super::resolver::*;
use super::scopes::{reserved_module_path, universe_bindings};
use super::scopes_intro::validate_module_names;
use super::visibility::top_level_vis;
use crate::context::*;

#[derive(Debug, Clone, Default)]
pub struct ResolveModulesResult {
    pub ok: bool,
    pub diags: DiagnosticStream,
    pub modules: Vec<ASTModule>,
}

/// The diagnostic code a resolver rule is reported as.
fn code_for_resolve_diag(diag_id: &str) -> Option<&'static str> {
    Some(match diag_id {
        "ResolveExpr-Ident-Err"
        | "ResolveQual-Name-Err"
        | "ResolveQual-Apply-Err"
        | "ResolveQual-Apply-Brace-Err"
        | "Expr-Unresolved-Err" => "E-MOD-1301",
        "E-CON-0031" => "E-CON-0031",
        "E-TYP-2007" => "E-TYP-2007",
        "E-SEM-2852" => "E-SEM-2852",
        "ResolveModulePath-Err" => "E-MOD-1107",
        "Import-Using-Missing" => "E-MOD-1201",
        "Resolve-Import-Err" => "E-MOD-1202",
        "Import-Using-Name-Conflict" => "E-MOD-1203",
        "Resolve-Using-None" => "E-MOD-1204",
        "Using-Path-Item-Public-Err" | "Using-List-Public-Err" => "E-MOD-1205",
        "Using-List-Dup" => "E-MOD-1206",
        "Resolve-Using-Ambig" => "E-MOD-1208",
        "Validate-ModulePath-Reserved-Err" => "E-CNF-0402",
        "Validate-Module-Keyword-Err" => "E-CNF-0401",
        "Intro-Reserved-Id-Err" | "Shadow-Reserved-Id-Err" => "E-CNF-0401",
        "Validate-Module-Prim-Shadow-Err"
        | "Validate-Module-Special-Shadow-Err"
        | "Validate-Module-Async-Shadow-Err" => "E-MOD-1304",
        "Protected-TopLevel-Err" => "E-MOD-2440",
        "Intro-Reserved-Gen-Err" | "Shadow-Reserved-Gen-Err" => "E-CNF-0406",
        "Intro-Reserved-Ultraviolet-Err" | "Shadow-Reserved-Ultraviolet-Err" => "E-CNF-0402",
        "Intro-Outer-Err" => "E-MOD-1304",
        "Collect-Dup" | "Names-Step-Dup" => "E-MOD-1302",
        "E-MOD-2430" => "E-MOD-2430",
        "Intro-Dup" => "E-MOD-1302",
        "Shadow-Unnecessary" => "E-MOD-1306",
        "Pat-Dup-Err" => "E-SEM-2713",
        "IfIs-BareTypePattern-Err" => "E-SEM-2761",
        "Access-Err" => "E-MOD-1207",
        _ => return None,
    })
}

fn resolve_item_detail(item: &ASTItem) -> String {
    let (kind, name) = match item {
        ASTItem::ProcedureDecl(it) => ("procedure", &it.name),
        ASTItem::ComptimeProcedureDecl(it) => ("comptime procedure", &it.name),
        ASTItem::RecordDecl(it) => ("record", &it.name),
        ASTItem::EnumDecl(it) => ("enum", &it.name),
        ASTItem::ModalDecl(it) => ("modal", &it.name),
        ASTItem::ClassDecl(it) => ("class", &it.name),
        ASTItem::TypeAliasDecl(it) => ("type alias", &it.name),
        _ => return "while resolving item".to_string(),
    };
    format!("while resolving {kind} `{name}`")
}

fn internal_error(message: String, span: Option<Span>) -> Diagnostic {
    Diagnostic { severity: Severity::Error, span, message, ..Default::default() }
}

/// The failure's detail becomes a note, followed by its suggestions.
fn attach_failure(diag: &mut Diagnostic, detail: &str, children: &[SubDiagnostic]) {
    if !detail.is_empty() {
        diag.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::Note,
            message: detail.to_string(),
            ..Default::default()
        });
    }
    diag.children.extend(children.iter().cloned());
}

fn emit_resolve_diag(diags: &mut DiagnosticStream, diag_id: &str, err: &ResError) {
    let mut diag = match code_for_resolve_diag(diag_id) {
        None => internal_error(
            format!("Internal error: resolver failed with unmapped diagnostic id `{diag_id}`."),
            err.span.clone(),
        ),
        Some(code) => make_diagnostic_by_id(code, err.span.clone()).unwrap_or_else(|| {
            internal_error(
                format!(
                    "Internal error: resolver diagnostic id `{diag_id}` mapped to unregistered diagnostic code `{code}`."
                ),
                err.span.clone(),
            )
        }),
    };
    attach_failure(&mut diag, &err.detail, &err.children);
    emit(diags, diag);
}

/// Resolves every module. Each module stops at its first failure; the others go on.
pub fn resolve_modules(ctx: &mut ResolveContext<'_, '_>) -> ResolveModulesResult {
    let mut result = ResolveModulesResult::default();
    if !ctx.parse_ok || ctx.parse_diags.is_some_and(|diags| has_error(diags)) {
        result.diags = ctx.parse_diags.cloned().unwrap_or_default();
        if !has_error(&result.diags) {
            let message = "Internal error: resolver was asked to continue after parse failure \
                           without a parse diagnostic.";
            emit(&mut result.diags, internal_error(message.to_string(), None));
        }
        return result;
    }
    result.ok = true;
    for index in 0..ctx.ctx.sigma.mods.len() {
        let module = ctx.ctx.sigma.mods[index].clone();
        ctx.ctx.current_module = module.path.clone();
        match resolve_module(ctx, &module) {
            Ok(resolved) => result.modules.push(resolved),
            Err(err) => {
                result.ok = false;
                match err.diag_id {
                    Some(diag_id) => emit_resolve_diag(&mut result.diags, diag_id, &err),
                    None => {
                        let mut diag = internal_error(
                            format!(
                                "Internal error: module resolution failed without diagnostic for `{}`.",
                                module.path.join("::")
                            ),
                            err.span.clone(),
                        );
                        attach_failure(&mut diag, &err.detail, &err.children);
                        emit(&mut result.diags, diag);
                    }
                }
            }
        }
    }
    result
}

pub fn resolve_module(ctx: &mut ResolveContext<'_, '_>, module: &ASTModule) -> Res<ASTModule> {
    let collected = collect_names(ctx.ctx, ctx.name_maps, ctx.module_names, module);
    if !collected.ok {
        return Err(ResError::from_id(collected.diag_id, collected.span));
    }
    if reserved_module_path(&module.path) {
        return Err(ResError::new("Validate-ModulePath-Reserved-Err", None));
    }
    let names_ok = validate_module_names(&collected.names, &collected.name_spans);
    if !names_ok.ok {
        return Err(ResError::from_id(names_ok.diag_id, names_ok.span));
    }
    let saved_module = std::mem::replace(&mut ctx.ctx.current_module, module.path.clone());
    let scopes = vec![Scope::new(), collected.names, universe_bindings()];
    let items = with_scopes(ctx, scopes, |ctx| resolve_items(ctx, &module.items));
    ctx.ctx.current_module = saved_module;
    Ok(ASTModule { items: items?, ..module.clone() })
}

/// A failure without a detail of its own says which item was being resolved.
pub fn resolve_items(ctx: &mut ResolveContext<'_, '_>, items: &[ASTItem]) -> Res<Vec<ASTItem>> {
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let vis = top_level_vis(item);
        if !vis.ok {
            return Err(ResError::from_id(vis.diag_id, Some(item_span(item).clone())));
        }
        out.push(resolve_item(ctx, item).map_err(|mut err| {
            if err.detail.is_empty() {
                err.detail = resolve_item_detail(item);
            }
            err
        })?);
    }
    Ok(out)
}
