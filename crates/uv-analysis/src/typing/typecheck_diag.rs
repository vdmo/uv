//! From the id a typing rule fails with to the diagnostic reported: most ids are rule
//! names, which resolve to a code through a few overrides, the diagnostic tables, or
//! the registry of static rules.

use uv_core::behavior_model::{is_static_rule, static_undefined_code_for_rule};
use uv_core::diagnostic_codes::{resolve_diag_code, spec_diag_code_map, uv_diag_code_map};
use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, Diagnostic, DiagnosticStream, Severity, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;

fn is_diagnostic_code(diag_id: &str) -> bool {
    let b = diag_id.as_bytes();
    b.len() > 2 && matches!(b[0], b'E' | b'W' | b'I' | b'P') && b[1] == b'-'
}

pub fn make_internal_typecheck_diagnostic(severity: Severity, span: Option<Span>, message: String) -> Diagnostic {
    Diagnostic { severity, span, message, ..Default::default() }
}

fn make_uncoded_static_typecheck_diagnostic(span: Option<Span>, rule_id: &str) -> Diagnostic {
    let message = match rule_id {
        "WF-ProcedureDecl-MissingReturnType" => "Procedure declaration requires explicit return type annotation".to_string(),
        "WF-ExternProcDecl-MissingReturnType" => "Extern procedure declaration requires explicit return type annotation".to_string(),
        _ => format!("Static rule failed without assigned diagnostic code: {rule_id}"),
    };
    make_internal_typecheck_diagnostic(Severity::Error, span, message)
}

pub fn lookup_typecheck_diag_code(diag_id: &str) -> Option<String> {
    let fixed = match diag_id {
        "Deref-Raw-Unsafe" => "E-TYP-2103",
        "Frame-NoActiveRegion-Err" => "E-MEM-1207",
        "Frame-Target-NotActive-Err" => "E-MEM-1208",
        "If-Cond-NotBool" => "E-SEM-2526",
        "LookupMethod-Ambig" => "E-MOD-1307",
        "Transmute-Unsafe-Err" => "E-MEM-3030",
        "WF-Contract" => "E-SEM-2808",
        "WF-ExternProcDecl-MissingReturnType" | "WF-ProcedureDecl-MissingReturnType" => "E-TYP-1508",
        "LookupMethod-NotFound" => "E-SEM-2536",
        "Superclass-Undefined" | "WF-Dynamic-Err" | "WF-Opaque-Err" => "E-TYP-2509",
        "Dynamic-NonDispatchable" => "E-TYP-2541",
        "Infer-Closure-Params-Err" => "E-SEM-2591",
        "T-Transmute-SizeEq" => "E-MEM-3031",
        "T-Transmute-AlignEq" => "E-UNS-0104",
        "PtrNull-Infer-Err" | "NullLiteral-Infer-Err" | "Syn-Call-Err" => "E-TYP-1530",
        "FieldAccess-NotVisible" => "E-TYP-1905",
        "Widen-NonModal" => "E-TYP-2071",
        "Widen-AlreadyGeneral" => "E-TYP-2072",
        "Record-Default-Init-Err" => "E-TYP-1911",
        "TypeAlias-Recursive-Err" => "E-TYP-1506",
        "WF-Async-ArgCount-Err" | "WF-Async-Arg-WF-Err" | "WF-Async-Path-Err" => "E-CON-0201",
        _ => "",
    };
    if !fixed.is_empty() {
        return Some(fixed.to_string());
    }
    resolve_diag_code(diag_id)
        .or_else(|| static_undefined_code_for_rule(spec_diag_code_map(), uv_diag_code_map(), diag_id))
        .or_else(|| is_diagnostic_code(diag_id).then(|| diag_id.to_string()))
}

pub fn build_resolved_typecheck_diagnostic(diag_id: &str, span: Option<Span>) -> Diagnostic {
    if let Some(code) = lookup_typecheck_diag_code(diag_id) {
        return make_diagnostic_by_id(&code, span.clone()).unwrap_or_else(|| {
            make_internal_typecheck_diagnostic(Severity::Error, span, format!("Internal error: unresolved diagnostic code '{code}'"))
        });
    }
    if is_static_rule(diag_id) {
        return make_uncoded_static_typecheck_diagnostic(span, diag_id);
    }
    make_internal_typecheck_diagnostic(Severity::Error, span, format!("Internal error: unknown diagnostic id '{diag_id}'"))
}

fn note(message: &str) -> SubDiagnostic {
    SubDiagnostic { kind: SubDiagnosticKind::Note, message: message.to_string(), ..Default::default() }
}

pub fn emit_resolved_typecheck_diagnostic(diags: &mut DiagnosticStream, diag_id: &str, span: Option<Span>, detail: &str) {
    let mut diag = build_resolved_typecheck_diagnostic(diag_id, span);
    if !detail.is_empty() {
        diag.children.push(note(detail));
    }
    emit(diags, diag);
}

/// What a declaration that failed typing reports: the diagnostic of its rule, with the
/// detail as a note, further children, and the obligations it discharges.
pub fn emit_decl_diag(
    diags: &mut DiagnosticStream,
    diag_id: Option<&str>,
    span: Option<Span>,
    detail: &str,
    children: Vec<SubDiagnostic>,
    obligation_ids: &[String],
) {
    let Some(diag_id) = diag_id else {
        return;
    };
    let mut diag = build_resolved_typecheck_diagnostic(diag_id, span.clone());
    if !detail.is_empty() {
        diag.children.push(note(detail));
    }
    diag.children.extend(children);
    for obligation_id in obligation_ids.iter().filter(|id| !id.is_empty()) {
        if !diag.obligation_ids.contains(obligation_id) {
            diag.obligation_ids.push(obligation_id.clone());
        }
    }
    emit(diags, diag);
    if diag_id == "E-SEM-2534" {
        emit(diags, build_resolved_typecheck_diagnostic("E-MOD-2411", span));
    }
}
