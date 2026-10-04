use crate::generated::diag_registry::DIAG_TABLE_OBLIGATIONS;
use crate::span::Span;
use crate::spec_rule;
use crate::spec_trace::Conformance;

pub type DiagCode = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Info,
    Panic,
    Note,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Panic => "panic",
            Severity::Note => "note",
        }
    }
}

/// Sub-diagnostic attached to a primary diagnostic (notes, help, fix-its).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubDiagnosticKind {
    #[default]
    Note,
    Help,
    FixIt,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SubDiagnostic {
    pub kind: SubDiagnosticKind,
    pub message: String,
    pub span: Option<Span>,
    /// Replacement text (FixIt only).
    pub fix_text: Option<String>,
    /// Inline label for the caret line.
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostic {
    /// Empty for auxiliary diagnostics with no code.
    pub code: DiagCode,
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    /// Inline label for the primary caret line.
    pub label: Option<String>,
    pub obligation_ids: Vec<String>,
    pub children: Vec<SubDiagnostic>,
}

pub type DiagnosticStream = Vec<Diagnostic>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileStatusResult {
    Ok,
    Fail,
}

fn diag_payload(diag: &Diagnostic) -> String {
    let mut payload = String::with_capacity(diag.code.len() + diag.message.len() + 32);
    payload.push_str("code=");
    payload.push_str(if diag.code.is_empty() { "<none>" } else { &diag.code });
    payload.push_str(";severity=");
    payload.push_str(diag.severity.label());
    payload.push_str(";message=");
    payload.push_str(&diag.message);
    payload
}

fn record_diagnostic_obligation(diag: &Diagnostic, obligation_id: &str, payload: &str) {
    if !obligation_id.is_empty() {
        Conformance::record_at(obligation_id, diag.span.as_ref(), payload);
    }
}

fn record_diagnostic_table_obligations(diag: &Diagnostic, payload: &str) {
    for (codes, obligation_ids) in DIAG_TABLE_OBLIGATIONS {
        if codes.contains(&diag.code.as_str()) {
            for obligation_id in obligation_ids.iter() {
                record_diagnostic_obligation(diag, obligation_id, payload);
            }
        }
    }
}

pub fn emit(stream: &mut DiagnosticStream, diag: Diagnostic) {
    spec_rule!("Emit-Append");
    if Conformance::enabled() {
        let payload = diag_payload(&diag);
        for obligation_id in &diag.obligation_ids {
            record_diagnostic_obligation(&diag, obligation_id, &payload);
        }
        record_diagnostic_table_obligations(&diag, &payload);
        Conformance::record_at("Diag-Emit", diag.span.as_ref(), &payload);
    }
    stream.push(diag);
}

pub fn emit_list(stream: &mut DiagnosticStream, diags: &[Diagnostic]) -> bool {
    for diag in diags {
        emit(stream, diag.clone());
    }
    true
}

pub fn has_error(stream: &[Diagnostic]) -> bool {
    stream.iter().any(|diag| diag.severity == Severity::Error)
}

pub fn compile_status(stream: &[Diagnostic]) -> CompileStatusResult {
    if has_error(stream) {
        CompileStatusResult::Fail
    } else {
        CompileStatusResult::Ok
    }
}
