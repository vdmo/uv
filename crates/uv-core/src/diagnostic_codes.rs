use std::collections::HashMap;
use std::sync::OnceLock;

use crate::diagnostics::DiagCode;
use crate::generated::diag_registry::DIAG_ID_CODE_MAP;
use crate::spec_rule;

pub type DiagId = str;
pub type DiagCodeMap = HashMap<&'static str, &'static str>;

fn is_diag_code_like(value: &str) -> bool {
    let b = value.as_bytes();
    let n = b.len();
    (n == 10 || n == 11)
        && matches!(b[0], b'E' | b'W' | b'I' | b'P')
        && b[1] == b'-'
        && b[2].is_ascii_uppercase()
        && b[3].is_ascii_uppercase()
        && b[4].is_ascii_uppercase()
        && (if n == 10 { b[5] == b'-' } else { b[5].is_ascii_uppercase() })
        && b[n - 5] == b'-'
        && b[n - 4].is_ascii_digit()
        && b[n - 3].is_ascii_digit()
        && b[n - 2].is_ascii_digit()
        && b[n - 1].is_ascii_digit()
}

pub fn spec_code(spec_map: &DiagCodeMap, id: &DiagId) -> Option<DiagCode> {
    spec_map.get(id).map(|code| code.to_string())
}

pub fn uv_code(uv_map: &DiagCodeMap, id: &DiagId) -> Option<DiagCode> {
    uv_map.get(id).map(|code| code.to_string())
}

pub fn code(spec_map: &DiagCodeMap, _uv_map: &DiagCodeMap, id: &DiagId) -> Option<DiagCode> {
    if let Some(spec) = spec_code(spec_map, id) {
        spec_rule!("Code");
        return Some(spec);
    }
    None
}

fn build_map(code_like: bool) -> DiagCodeMap {
    let mut out = DiagCodeMap::new();
    for &(diag_id, code) in DIAG_ID_CODE_MAP {
        if is_diag_code_like(diag_id) == code_like {
            out.entry(diag_id).or_insert(code);
        }
    }
    out
}

pub fn spec_diag_code_map() -> &'static DiagCodeMap {
    static MAP: OnceLock<DiagCodeMap> = OnceLock::new();
    MAP.get_or_init(|| build_map(true))
}

pub fn uv_diag_code_map() -> &'static DiagCodeMap {
    static MAP: OnceLock<DiagCodeMap> = OnceLock::new();
    MAP.get_or_init(|| build_map(false))
}

pub fn resolve_diag_code(id: &DiagId) -> Option<DiagCode> {
    if let Some(resolved) = code(spec_diag_code_map(), uv_diag_code_map(), id) {
        return Some(resolved);
    }
    if let Some(c0) = uv_code(uv_diag_code_map(), id) {
        return Some(c0);
    }
    if is_diag_code_like(id) {
        return Some(id.to_string());
    }
    None
}

pub fn is_diag_category(category: &str) -> bool {
    category.len() == 3 && category.bytes().all(|c| c.is_ascii_uppercase())
}

pub fn is_diag_digits(digits: &str) -> bool {
    digits.len() == 4 && digits.bytes().all(|c| c.is_ascii_digit())
}

pub fn diag_bucket(digits: &str) -> Option<String> {
    is_diag_digits(digits).then(|| digits[0..2].to_string())
}

pub fn diag_seq(digits: &str) -> Option<String> {
    is_diag_digits(digits).then(|| digits[2..4].to_string())
}
