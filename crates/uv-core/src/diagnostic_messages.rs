use crate::diagnostic_codes::resolve_diag_code;
use crate::diagnostic_render::{apply_no_span_external, DiagnosticOrigin};
use crate::diagnostics::{emit, Diagnostic, DiagnosticStream, Severity};
use crate::generated::diag_registry::{DiagRegistryRow, DIAG_REGISTRY_ROWS};
use crate::span::Span;
use crate::spec_rule;

pub struct MessageArg<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

fn find_entry(code: &str) -> Option<&'static DiagRegistryRow> {
    DIAG_REGISTRY_ROWS
        .binary_search_by(|row| row.code.cmp(code))
        .ok()
        .map(|index| &DIAG_REGISTRY_ROWS[index])
}

pub fn message_for_code(code: &str) -> Option<&'static str> {
    find_entry(code).map(|entry| entry.condition)
}

pub fn severity_for_code(code: &str) -> Option<Severity> {
    find_entry(code).map(|entry| entry.severity)
}

pub fn format_message(message_template: &str, args: &[MessageArg<'_>]) -> String {
    if !message_template.contains('{') {
        return message_template.to_string();
    }
    let bytes = message_template.as_bytes();
    let mut out = String::with_capacity(message_template.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'{' {
            let next = message_template[i..].find('{').map_or(bytes.len(), |p| i + p);
            out.push_str(&message_template[i..next]);
            i = next;
            continue;
        }
        let Some(close) = message_template[i + 1..].find('}').map(|p| i + 1 + p) else {
            out.push_str(&message_template[i..]);
            break;
        };
        let key = &message_template[i + 1..close];
        match args.iter().find(|arg| arg.key == key) {
            None => out.push_str(&message_template[i..=close]),
            Some(arg) => out.push_str(arg.value),
        }
        i = close + 1;
    }
    out
}

pub fn make_diagnostic(code: &str, span: Option<Span>) -> Option<Diagnostic> {
    let entry = find_entry(code)?;
    Some(Diagnostic {
        code: entry.code.to_string(),
        severity: entry.severity,
        message: entry.condition.to_string(),
        span,
        ..Diagnostic::default()
    })
}

pub fn make_diagnostic_by_id_with_origin(
    diag_id: &str,
    span: Option<Span>,
    origin: DiagnosticOrigin,
) -> Option<Diagnostic> {
    let resolved = resolve_diag_code(diag_id)?;
    spec_rule!("DiagId-Code");
    let mut diag = make_diagnostic(&resolved, span)?;
    if diag_id != diag.code {
        diag.obligation_ids.push(diag_id.to_string());
    }
    Some(apply_no_span_external(diag, origin))
}

pub fn make_diagnostic_by_id(diag_id: &str, span: Option<Span>) -> Option<Diagnostic> {
    make_diagnostic_by_id_with_origin(diag_id, span, DiagnosticOrigin::Internal)
}

pub fn make_external_diagnostic(code: &str) -> Option<Diagnostic> {
    make_diagnostic_by_id_with_origin(code, None, DiagnosticOrigin::External)
}

pub fn emit_external_diagnostic(stream: &mut DiagnosticStream, code: &str) {
    if let Some(diag) = make_external_diagnostic(code) {
        emit(stream, diag);
    }
}

pub fn emit_diagnostic_by_id(stream: &mut DiagnosticStream, diag_id: &str, span: Option<Span>) {
    if let Some(diag) = make_diagnostic_by_id(diag_id, span) {
        emit(stream, diag);
    }
}
