//! Unicode 15.0 services: NFC, case folding, identifier properties and identifier security.

use unicode_normalization::UnicodeNormalization;
use unicode_script::{Script, UnicodeScript};

use crate::generated::case_folding::CASE_FOLDING;
use crate::process_config::is_debug_enabled;
use crate::source_text::{is_scalar_value, utf8_offsets, UnicodeScalar, LF};
use crate::spec_rule;

fn log_unicode_debug(message: &str) {
    if !is_debug_enabled("lex") && !is_debug_enabled("parse") {
        return;
    }
    eprintln!("[uv] unicode: {message}");
}

pub fn nfc(s: &str) -> String {
    if s.is_ascii() {
        return s.to_string();
    }
    s.nfc().collect()
}

pub fn case_fold(s: &str) -> String {
    if s.is_ascii() {
        return s.to_ascii_lowercase();
    }
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match CASE_FOLDING.binary_search_by_key(&(ch as u32), |&(code, _)| code) {
            Ok(index) => {
                for &mapped in CASE_FOLDING[index].1 {
                    out.push(char::from_u32(mapped).expect("case folding maps to scalar values"));
                }
            }
            Err(_) => out.push(ch),
        }
    }
    out
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IdentifierSecurityInfo {
    pub normalized: String,
    pub skeleton: String,
    pub mixed_script: bool,
}

fn has_mixed_identifier_scripts(normalized: &str) -> bool {
    let mut first: Option<Script> = None;
    for ch in normalized.chars() {
        let script = ch.script();
        if script == Script::Common || script == Script::Inherited {
            continue;
        }
        match first {
            None => first = Some(script),
            Some(seen) if seen != script => return true,
            Some(_) => {}
        }
    }
    false
}

pub fn analyze_identifier_security(ident: &str) -> IdentifierSecurityInfo {
    if is_debug_enabled("lex") || is_debug_enabled("parse") {
        log_unicode_debug(&format!("AnalyzeIdentifierSecurity ident=\"{ident}\""));
    }
    let normalized = nfc(ident);
    let skeleton: String = unicode_security::skeleton(&normalized).collect();
    let mixed_script = has_mixed_identifier_scripts(&normalized);
    IdentifierSecurityInfo { normalized, skeleton, mixed_script }
}

pub fn is_xid_start(c: UnicodeScalar) -> bool {
    char::from_u32(c).is_some_and(unicode_ident::is_xid_start)
}

pub fn is_xid_continue(c: UnicodeScalar) -> bool {
    char::from_u32(c).is_some_and(unicode_ident::is_xid_continue)
}

pub fn is_non_character(c: UnicodeScalar) -> bool {
    if c > 0x10FFFF {
        return false;
    }
    if (0xFDD0..=0xFDEF).contains(&c) {
        return true;
    }
    let low = c & 0xFFFF;
    low == 0xFFFE || low == 0xFFFF
}

pub fn is_ident_start(c: UnicodeScalar) -> bool {
    c == '_' as u32 || is_xid_start(c)
}

pub fn is_ident_continue(c: UnicodeScalar) -> bool {
    c == '_' as u32 || is_xid_continue(c)
}

pub fn is_sensitive(c: UnicodeScalar) -> bool {
    (0x202A..=0x202E).contains(&c) || (0x2066..=0x2069).contains(&c) || c == 0x200C || c == 0x200D
}

fn is_hex_digit(c: UnicodeScalar) -> bool {
    char::from_u32(c).is_some_and(|ch| ch.is_ascii_hexdigit())
}

fn hex_value(c: UnicodeScalar) -> u32 {
    char::from_u32(c).and_then(|ch| ch.to_digit(16)).unwrap_or(0)
}

const QUOTE: u32 = '"' as u32;
const APOSTROPHE: u32 = '\'' as u32;
const BACKSLASH: u32 = '\\' as u32;

fn scan_escape(scalars: &[UnicodeScalar], start: usize) -> Option<usize> {
    if start + 1 >= scalars.len() || scalars[start] != BACKSLASH {
        return None;
    }
    match char::from_u32(scalars[start + 1])? {
        '\\' | '"' | '\'' | 'n' | 'r' | 't' | '0' => Some(start + 2),
        'x' => {
            if start + 3 >= scalars.len() {
                return None;
            }
            if !is_hex_digit(scalars[start + 2]) || !is_hex_digit(scalars[start + 3]) {
                return None;
            }
            Some(start + 4)
        }
        'u' => {
            if start + 2 >= scalars.len() || scalars[start + 2] != '{' as u32 {
                return None;
            }
            let mut p = start + 3;
            let mut value: u32 = 0;
            let mut digits = 0usize;
            while p < scalars.len() && is_hex_digit(scalars[p]) {
                if digits == 6 {
                    return None;
                }
                value = (value << 4) | hex_value(scalars[p]);
                digits += 1;
                p += 1;
            }
            if digits == 0 {
                return None;
            }
            if p >= scalars.len() || scalars[p] != '}' as u32 {
                return None;
            }
            if !is_scalar_value(value) {
                return None;
            }
            Some(p + 1)
        }
        _ => None,
    }
}

fn scan_string_literal(scalars: &[UnicodeScalar], start: usize) -> Option<usize> {
    if start >= scalars.len() || scalars[start] != QUOTE {
        return None;
    }
    let mut i = start + 1;
    while i < scalars.len() {
        let c = scalars[i];
        if c == QUOTE {
            return Some(i + 1);
        }
        if c == LF {
            return None;
        }
        if c == BACKSLASH {
            i = scan_escape(scalars, i)?;
            continue;
        }
        i += 1;
    }
    None
}

fn scan_char_literal(scalars: &[UnicodeScalar], start: usize) -> Option<usize> {
    if start >= scalars.len() || scalars[start] != APOSTROPHE {
        return None;
    }
    if start + 1 >= scalars.len() {
        return None;
    }
    if scalars[start + 1] == LF {
        return None;
    }
    let mut i = start + 1;
    if scalars[i] == BACKSLASH {
        i = scan_escape(scalars, i)?;
    } else {
        if scalars[i] == APOSTROPHE {
            return None;
        }
        i += 1;
    }
    if i >= scalars.len() || scalars[i] != APOSTROPHE {
        return None;
    }
    Some(i + 1)
}

/// Byte ranges of well-formed string and character literals.
fn literal_spans(scalars: &[UnicodeScalar], offsets: &[usize]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < scalars.len() {
        let end = if scalars[i] == QUOTE {
            scan_string_literal(scalars, i)
        } else if scalars[i] == APOSTROPHE {
            scan_char_literal(scalars, i)
        } else {
            None
        };
        if let Some(end) = end {
            spans.push((offsets[i], offsets[end]));
            i = end;
            continue;
        }
        i += 1;
    }
    spans
}

pub fn is_prohibited(c: UnicodeScalar) -> bool {
    let is_cc = c <= 0x1F || (0x7F..=0x9F).contains(&c);
    if !is_cc {
        return false;
    }
    c != 0x09 && c != 0x0A && c != 0x0C && c != 0x0D
}

pub fn first_prohibited_outside_literal(scalars: &[UnicodeScalar]) -> Option<usize> {
    if !scalars.iter().any(|&c| is_prohibited(c)) {
        return None;
    }
    let offsets = utf8_offsets(scalars);
    let spans = literal_spans(scalars, &offsets);
    let mut span_index = 0usize;
    for (i, &c) in scalars.iter().enumerate() {
        if !is_prohibited(c) {
            continue;
        }
        let offset = offsets[i];
        while span_index < spans.len() && offset >= spans[span_index].1 {
            span_index += 1;
        }
        let in_literal = span_index < spans.len()
            && offset >= spans[span_index].0
            && offset < spans[span_index].1;
        if !in_literal {
            return Some(i);
        }
    }
    None
}

pub fn no_prohibited(scalars: &[UnicodeScalar]) -> bool {
    spec_rule!("WF-Prohibited");
    first_prohibited_outside_literal(scalars).is_none()
}
