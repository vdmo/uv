//! Core infrastructure: source text, spans, diagnostics, Unicode, paths.

pub mod behavior_model;
pub mod diagnostic_codes;
pub mod diagnostic_messages;
pub mod diagnostic_render;
pub mod diagnostics;
mod generated;
pub mod hash;
pub mod host;
pub mod ident;
pub mod keywords;
pub mod numeric_literals;
pub mod path;
pub mod process_config;
pub mod source_load;
pub mod source_text;
pub mod span;
pub mod std_unordered;
pub mod spec_trace;
pub mod symbols;
pub mod terminal;
pub mod unicode;

/// Conformance traceability hook. Records the rule only when tracing is enabled.
#[macro_export]
macro_rules! spec_rule {
    ($id:expr) => {
        if $crate::spec_trace::Conformance::enabled() {
            $crate::spec_trace::Conformance::record($id);
        }
    };
}

/// Conformance traceability hook carrying a span.
#[macro_export]
macro_rules! spec_rule_at {
    ($id:expr, $span:expr) => {
        if $crate::spec_trace::Conformance::enabled() {
            $crate::spec_trace::Conformance::record_at($id, Some(&$span), "");
        }
    };
}
