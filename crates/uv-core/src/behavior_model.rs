/// Error recovery policy: abort compilation after this many errors (`None` = unbounded).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorRecoveryPolicy {
    pub max_error_count: Option<usize>,
}

pub const SUGGESTED_MAX_ERROR_COUNT: usize = 100;

impl Default for ErrorRecoveryPolicy {
    fn default() -> Self {
        ErrorRecoveryPolicy { max_error_count: Some(SUGGESTED_MAX_ERROR_COUNT) }
    }
}

use crate::diagnostic_codes::{code, DiagCodeMap};
use crate::diagnostics::DiagCode;
use crate::generated::static_rules::{STATIC_JUDGMENT_FAMILIES, STATIC_RULES};

pub fn default_error_recovery_policy() -> ErrorRecoveryPolicy {
    ErrorRecoveryPolicy::default()
}

pub fn abort_on_error_count(policy: &ErrorRecoveryPolicy, error_count: usize) -> bool {
    policy.max_error_count.is_some_and(|max| error_count >= max)
}

pub fn is_static_judgment_family(judgment_family: &str) -> bool {
    STATIC_JUDGMENT_FAMILIES.contains(&judgment_family)
}

type StaticRule = (&'static str, &'static str, Option<&'static str>, bool);

fn lookup_static_rule(rule_id: &str) -> Option<&'static StaticRule> {
    STATIC_RULES.iter().find(|(id, family, _, _)| *id == rule_id && is_static_judgment_family(family))
}

pub fn is_static_rule(rule_id: &str) -> bool {
    lookup_static_rule(rule_id).is_some()
}

pub fn conclusion_of_rule(rule_id: &str) -> Option<&'static str> {
    lookup_static_rule(rule_id).map(|(_, family, _, _)| *family)
}

pub fn diag_id_of_rule(rule_id: &str) -> Option<&'static str> {
    lookup_static_rule(rule_id).and_then(|(_, _, diag_id, _)| *diag_id)
}

/// The diagnostic id of the first rule concluding the judgment that names one.
pub fn diag_id_of_judgment(judgment_family: &str) -> Option<&'static str> {
    STATIC_RULES.iter().filter(|(_, family, _, _)| *family == judgment_family).find_map(|(_, _, diag_id, _)| *diag_id)
}

/// Whether failing the judgment is a static error: one of its rules has a bottom premise.
pub fn static_undefined(judgment_family: &str) -> bool {
    is_static_judgment_family(judgment_family)
        && STATIC_RULES.iter().any(|(_, family, _, bottom)| *family == judgment_family && *bottom)
}

fn is_diagnostic_code_like(id: &str) -> bool {
    let b = id.as_bytes();
    b.len() == 10
        && matches!(b[0], b'E' | b'W' | b'I' | b'P')
        && b[1] == b'-'
        && b[2..5].iter().all(u8::is_ascii_uppercase)
        && b[5] == b'-'
        && b[6..10].iter().all(u8::is_ascii_digit)
}

/// The code a failed static rule reports, when the rule or its judgment names one.
pub fn static_undefined_code_for_rule(spec_map: &DiagCodeMap, uv_map: &DiagCodeMap, rule_id: &str) -> Option<DiagCode> {
    let conclusion = conclusion_of_rule(rule_id)?;
    if !static_undefined(conclusion) {
        return None;
    }
    let diag_id = diag_id_of_rule(rule_id).or_else(|| diag_id_of_judgment(conclusion))?;
    if is_diagnostic_code_like(diag_id) {
        return Some(diag_id.to_string());
    }
    code(spec_map, uv_map, diag_id)
}
