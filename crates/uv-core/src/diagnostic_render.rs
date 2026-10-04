use crate::diagnostics::{Diagnostic, DiagnosticStream, Severity, SubDiagnosticKind};
use crate::span::Span;
use crate::spec_rule;
use crate::terminal::{colorize, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticOrigin {
    Internal,
    External,
}

pub fn order(stream: &[Diagnostic]) -> DiagnosticStream {
    spec_rule!("Order");
    stream.to_vec()
}

pub fn apply_no_span_external(mut diag: Diagnostic, origin: DiagnosticOrigin) -> Diagnostic {
    spec_rule!("NoSpan-External");
    if origin == DiagnosticOrigin::External {
        diag.span = None;
    }
    diag
}

pub fn render(diag: &Diagnostic) -> String {
    let mut out = String::new();
    let label = diag.severity.label();
    if diag.code.is_empty() {
        out.push_str(label);
    } else {
        out.push_str(&diag.code);
        out.push_str(" (");
        out.push_str(label);
        out.push(')');
    }
    if !diag.message.is_empty() {
        out.push_str(": ");
        out.push_str(&diag.message);
    }
    if let Some(span) = &diag.span {
        out.push_str(" @");
        out.push_str(&span.file);
        out.push(':');
        out.push_str(&span.start_line.to_string());
        out.push(':');
        out.push_str(&span.start_col.to_string());
    }
    out
}

pub struct RenderOptions {
    pub color: bool,
    /// Lines of context before/after the error line.
    pub context_lines: i32,
    /// 0 = no truncation.
    pub terminal_width: i32,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions { color: false, context_lines: 1, terminal_width: 0 }
    }
}

fn severity_color(sev: Severity) -> Color {
    match sev {
        Severity::Error => Color::BoldRed,
        Severity::Warning => Color::BoldYellow,
        Severity::Info => Color::BoldBlue,
        Severity::Panic => Color::BoldMagenta,
        Severity::Note => Color::BoldCyan,
    }
}

pub fn sub_diag_label(kind: SubDiagnosticKind) -> &'static str {
    match kind {
        SubDiagnosticKind::Note => "note",
        SubDiagnosticKind::Help | SubDiagnosticKind::FixIt => "help",
    }
}

pub fn sub_diag_color(kind: SubDiagnosticKind) -> Color {
    match kind {
        SubDiagnosticKind::Note => Color::BoldCyan,
        SubDiagnosticKind::Help | SubDiagnosticKind::FixIt => Color::BoldGreen,
    }
}

fn get_source_line(source: &str, line_number: usize) -> &str {
    if line_number == 0 {
        return "";
    }
    let bytes = source.as_bytes();
    let mut current_line = 1usize;
    let mut pos = 0usize;
    while pos < bytes.len() {
        let newline = bytes[pos..].iter().position(|&b| b == b'\n').map(|p| pos + p);
        if current_line == line_number {
            let end = newline.unwrap_or(bytes.len());
            let mut line_end = end;
            if line_end > pos && bytes[line_end - 1] == b'\r' {
                line_end -= 1;
            }
            return &source[pos..line_end];
        }
        match newline {
            None => break,
            Some(nl) => {
                pos = nl + 1;
                current_line += 1;
            }
        }
    }
    ""
}

fn underline_for_span(span: &Span) -> String {
    let spaces = span.start_col.saturating_sub(1);
    let carets = span.end_col.saturating_sub(span.start_col);
    let mut out = " ".repeat(spaces);
    out.push_str(&"^".repeat(carets));
    out
}

/// Maps a file path to its full text content.
pub type SourceRegistry<'a> = &'a dyn Fn(&str) -> Option<String>;

/// Rich diagnostic rendering with source context and optional ANSI color.
pub fn render_rich(
    diag: &Diagnostic,
    sources: Option<SourceRegistry<'_>>,
    opts: &RenderOptions,
) -> String {
    let c = opts.color;
    let sev_color = severity_color(diag.severity);
    let mut head = colorize(diag.severity.label(), sev_color, c);
    if !diag.code.is_empty() {
        head.push_str(&colorize("[", sev_color, c));
        head.push_str(&colorize(&diag.code, sev_color, c));
        head.push_str(&colorize("]", sev_color, c));
    }
    if !diag.message.is_empty() {
        head.push_str(": ");
        head.push_str(&diag.message);
    }
    if let (Some(sp), Some(sources)) = (&diag.span, sources) {
        if let Some(file_content) = sources(&sp.file) {
            let line_text = get_source_line(&file_content, sp.start_line);
            let gutter = sp.start_line.to_string();
            let mut out = head;
            out.push('\n');
            out.push_str(&colorize("  --> ", Color::Blue, c));
            out.push_str(&sp.file);
            out.push(':');
            out.push_str(&sp.start_line.to_string());
            out.push(':');
            out.push_str(&sp.start_col.to_string());
            out.push('\n');
            for body in [line_text.to_string(), colorize(&underline_for_span(sp), sev_color, c)] {
                out.push_str(&colorize(&gutter, Color::Blue, c));
                out.push(' ');
                out.push_str(&colorize("|", Color::Blue, c));
                out.push(' ');
                out.push_str(&body);
                out.push('\n');
            }
            out.pop();
            return out;
        }
    }
    head
}

/// Produce a summary line like "3 errors, 1 warning emitted".
pub fn diagnostic_summary(stream: &[Diagnostic], color: bool) -> String {
    let count = |sev: Severity| stream.iter().filter(|d| d.severity == sev).count();
    let parts = [
        (count(Severity::Error), "error", Color::BoldRed),
        (count(Severity::Warning), "warning", Color::BoldYellow),
        (count(Severity::Info), "info", Color::BoldBlue),
        (count(Severity::Panic), "panic", Color::BoldMagenta),
        (count(Severity::Note), "note", Color::BoldCyan),
    ];
    if parts.iter().all(|(n, _, _)| *n == 0) {
        return String::new();
    }
    let mut out = String::new();
    for (n, singular, count_color) in parts {
        if n == 0 {
            continue;
        }
        if !out.is_empty() {
            out.push_str(", ");
        }
        let text = format!("{n} {singular}{}", if n != 1 { "s" } else { "" });
        out.push_str(&colorize(&text, count_color, color));
    }
    out.push_str(" emitted");
    out
}
