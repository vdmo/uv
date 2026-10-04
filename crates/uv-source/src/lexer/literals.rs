use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::source_text::{is_scalar_value, SourceFile, UnicodeScalar};
use uv_core::span::{span_of, Span};
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use super::{ch, find_terminator, match_prefix, LiteralScanResult, ScalarRange};

fn is_dec_digit(c: UnicodeScalar) -> bool {
    (ch('0')..=ch('9')).contains(&c)
}

fn is_hex_digit(c: UnicodeScalar) -> bool {
    is_dec_digit(c) || (ch('a')..=ch('f')).contains(&c) || (ch('A')..=ch('F')).contains(&c)
}

fn is_oct_digit(c: UnicodeScalar) -> bool {
    (ch('0')..=ch('7')).contains(&c)
}

fn is_bin_digit(c: UnicodeScalar) -> bool {
    c == ch('0') || c == ch('1')
}

fn hex_digit_value(c: UnicodeScalar) -> u64 {
    if is_dec_digit(c) {
        (c - ch('0')) as u64
    } else if (ch('a')..=ch('f')).contains(&c) {
        10 + (c - ch('a')) as u64
    } else {
        10 + (c - ch('A')) as u64
    }
}

const INT_SUFFIXES: [&str; 12] = [
    "isize", "usize", "i128", "u128", "i64", "u64", "i32", "u32", "i16", "u16", "i8", "u8",
];
const FLOAT_SUFFIXES: [&str; 4] = ["f16", "f32", "f64", "f"];

fn ends_with_underscore_suffix(s: &str, suffix: &str) -> bool {
    s.len() > suffix.len() + 1
        && s.ends_with(suffix)
        && s.as_bytes()[s.len() - suffix.len() - 1] == b'_'
}

fn numeric_underscore_ok(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() {
        return true;
    }
    if b[0] == b'_' || b[b.len() - 1] == b'_' {
        return false;
    }
    if s.starts_with("0x_") || s.starts_with("0o_") || s.starts_with("0b_") {
        return false;
    }
    let has_non_decimal_prefix = ["0x", "0X", "0o", "0O", "0b", "0B"].iter().any(|p| s.starts_with(p));
    if !has_non_decimal_prefix {
        for i in 0..b.len() {
            if b[i] != b'_' {
                continue;
            }
            if i > 0 && (b[i - 1] == b'e' || b[i - 1] == b'E') {
                return false;
            }
            if i + 1 < b.len() && (b[i + 1] == b'e' || b[i + 1] == b'E') {
                return false;
            }
        }
    }
    !INT_SUFFIXES.iter().chain(FLOAT_SUFFIXES.iter()).any(|suf| ends_with_underscore_suffix(s, suf))
}

fn decimal_leading_zero(s: &str) -> bool {
    let mut digits = s.bytes().filter(|&c| c != b'_');
    digits.next() == Some(b'0') && digits.next().is_some()
}

fn scan_run_with_underscore(
    scalars: &[UnicodeScalar],
    start: usize,
    pred: fn(UnicodeScalar) -> bool,
) -> usize {
    let mut p = start;
    while p < scalars.len() && (pred(scalars[p]) || scalars[p] == ch('_')) {
        p += 1;
    }
    p
}

fn has_scalar(scalars: &[UnicodeScalar], start: usize, end: usize, pred: fn(UnicodeScalar) -> bool) -> bool {
    scalars[start..end].iter().any(|&c| pred(c))
}

fn is_exp_marker(c: UnicodeScalar) -> bool {
    c == ch('e') || c == ch('E')
}

fn scan_exponent_tail(scalars: &[UnicodeScalar], exp_marker: usize) -> usize {
    if exp_marker >= scalars.len() || !is_exp_marker(scalars[exp_marker]) {
        return exp_marker;
    }
    let mut p = exp_marker + 1;
    if p < scalars.len() && (scalars[p] == ch('+') || scalars[p] == ch('-')) {
        p += 1;
    }
    scan_run_with_underscore(scalars, p, is_dec_digit)
}

fn match_digit_run_chars(s: &str, pred: fn(u8) -> bool) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut saw_digit = false;
    let mut last_was_underscore = false;
    for c in s.bytes() {
        if c == b'_' {
            if !saw_digit {
                return false;
            }
            last_was_underscore = true;
            continue;
        }
        if !pred(c) {
            return false;
        }
        saw_digit = true;
        last_was_underscore = false;
    }
    saw_digit && !last_was_underscore
}

fn is_dec_digit_char(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_hex_digit_char(c: u8) -> bool {
    c.is_ascii_hexdigit()
}

fn is_oct_digit_char(c: u8) -> bool {
    (b'0'..=b'7').contains(&c)
}

fn is_bin_digit_char(c: u8) -> bool {
    c == b'0' || c == b'1'
}

fn match_int_suffix_lexeme(lexeme: &str) -> Option<&'static str> {
    INT_SUFFIXES
        .iter()
        .copied()
        .find(|suffix| lexeme.len() > suffix.len() && lexeme.ends_with(suffix))
}

fn matches_integer_literal_lexeme(lexeme: &str) -> bool {
    let core = match match_int_suffix_lexeme(lexeme) {
        Some(suffix) => &lexeme[..lexeme.len() - suffix.len()],
        None => lexeme,
    };
    let b = core.as_bytes();
    if b.len() >= 2 && b[0] == b'0' {
        match b[1] {
            b'x' => return match_digit_run_chars(&core[2..], is_hex_digit_char),
            b'o' => return match_digit_run_chars(&core[2..], is_oct_digit_char),
            b'b' => return match_digit_run_chars(&core[2..], is_bin_digit_char),
            _ => {}
        }
    }
    match_digit_run_chars(core, is_dec_digit_char)
}

fn matches_decimal_integer_lexeme(lexeme: &str) -> bool {
    let b = lexeme.as_bytes();
    if b.len() >= 2 && b[0] == b'0' && matches!(b[1], b'x' | b'o' | b'b') {
        return false;
    }
    match_digit_run_chars(lexeme, is_dec_digit_char)
}

fn matches_exponent_lexeme(lexeme: &str) -> bool {
    if lexeme.is_empty() {
        return false;
    }
    let rest = lexeme.strip_prefix(['+', '-']).unwrap_or(lexeme);
    !rest.is_empty() && matches_decimal_integer_lexeme(rest)
}

fn matches_float_core_lexeme(lexeme: &str) -> bool {
    let Some(dot) = lexeme.find('.') else {
        return false;
    };
    if !matches_decimal_integer_lexeme(&lexeme[..dot]) {
        return false;
    }
    let after_dot = &lexeme[dot + 1..];
    let exp_pos = after_dot.find(['e', 'E']);
    let frac_part = exp_pos.map_or(after_dot, |pos| &after_dot[..pos]);
    if !frac_part.is_empty() && !matches_decimal_integer_lexeme(frac_part) {
        return false;
    }
    match exp_pos {
        None => true,
        Some(pos) => matches_exponent_lexeme(&after_dot[pos + 1..]),
    }
}

fn matches_float_literal_lexeme(lexeme: &str) -> bool {
    for suffix in FLOAT_SUFFIXES {
        if lexeme.len() <= suffix.len() || !lexeme.ends_with(suffix) {
            continue;
        }
        if matches_float_core_lexeme(&lexeme[..lexeme.len() - suffix.len()]) {
            return true;
        }
    }
    matches_float_core_lexeme(lexeme)
}

fn scan_escape_match(scalars: &[UnicodeScalar], start: usize) -> Option<usize> {
    if start + 1 >= scalars.len() || scalars[start] != ch('\\') {
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
            if start + 2 >= scalars.len() || scalars[start + 2] != ch('{') {
                return None;
            }
            let digits_start = start + 3;
            let mut p = digits_start;
            while p < scalars.len() && is_hex_digit(scalars[p]) {
                p += 1;
            }
            if p == digits_start {
                return None;
            }
            if p >= scalars.len() || scalars[p] != ch('}') {
                return None;
            }
            // The reference accumulates into 64 bits; digits shifted out are lost.
            let value = scalars[digits_start..p]
                .iter()
                .fold(0u64, |acc, &digit| (acc << 4) | hex_digit_value(digit));
            if value > 0x10FFFF || !is_scalar_value(value as u32) {
                return None;
            }
            Some(p + 1)
        }
        _ => None,
    }
}

fn first_bad_escape(scalars: &[UnicodeScalar], start: usize, terminator: usize) -> Option<usize> {
    let mut p = start + 1;
    while p < terminator {
        if scalars[p] != ch('\\') {
            p += 1;
            continue;
        }
        match scan_escape_match(scalars, p) {
            Some(next) if next <= terminator => p = next,
            _ => return Some(p),
        }
    }
    None
}

fn char_scalar_count(scalars: &[UnicodeScalar], start: usize, terminator: usize) -> usize {
    let mut count = 0usize;
    let mut p = start + 1;
    while p < terminator {
        count += 1;
        if scalars[p] != ch('\\') {
            p += 1;
            continue;
        }
        match scan_escape_match(scalars, p) {
            Some(next) if next <= terminator => p = next,
            _ => p += 1,
        }
    }
    count
}

fn suffix_match(scalars: &[UnicodeScalar], start: usize, suffixes: &[&str]) -> usize {
    suffixes
        .iter()
        .filter(|suffix| match_prefix(scalars, start, suffix))
        .map(|suffix| suffix.len())
        .max()
        .unwrap_or(0)
}

fn span_of_text(source: &SourceFile, i: usize, j: usize) -> Span {
    span_of(source, source.offsets[i], source.offsets[j])
}

fn lexeme_slice(source: &SourceFile, i: usize, j: usize) -> &str {
    let start = source.offsets[i];
    let end = source.offsets[j];
    if start >= source.text.len() || end < start {
        return "";
    }
    &source.text[start..end]
}

fn emit_diag(diags: &mut DiagnosticStream, code: &str, span: Span) {
    if let Some(diag) = make_diagnostic_by_id(code, Some(span)) {
        emit(diags, diag);
    }
}

fn record_first_bad_escape_definition(
    obligation_id: &str,
    literal_kind: &str,
    span: &Span,
    bad_scalar_index: usize,
) {
    if !Conformance::enabled() {
        return;
    }
    let payload =
        format!("source=FirstBadEscape;literal={literal_kind};bad_scalar_index={bad_scalar_index}");
    Conformance::record_at(obligation_id, Some(span), &payload);
}

pub fn scan_int_literal(source: &SourceFile, start: usize) -> LiteralScanResult {
    spec_rule!("Lex-Int");
    spec_rule!("Lex-Numeric-Err");
    let mut result = LiteralScanResult { ok: false, next: start, ..LiteralScanResult::default() };
    let scalars = &source.scalars;
    let n = scalars.len();
    if start >= n || !is_dec_digit(scalars[start]) {
        return result;
    }
    let mut p = start;
    let mut is_based = false;
    let mut saw_exp = false;
    if scalars[start] == ch('0') && start + 1 < n {
        let next = scalars[start + 1];
        let run_start = start + 2;
        if next == ch('x') {
            is_based = true;
            p = scan_run_with_underscore(scalars, run_start, is_hex_digit);
        } else if next == ch('o') {
            is_based = true;
            p = scan_run_with_underscore(scalars, run_start, is_oct_digit);
        } else if next == ch('b') {
            is_based = true;
            p = scan_run_with_underscore(scalars, run_start, is_bin_digit);
        }
    }
    if !is_based {
        p = scan_run_with_underscore(scalars, start, is_dec_digit);
        if p == start {
            return result;
        }
        if p < n && is_exp_marker(scalars[p]) {
            p = scan_exponent_tail(scalars, p);
        }
        saw_exp = has_scalar(scalars, start, p, is_exp_marker);
    }
    let int_suffix_len = suffix_match(scalars, p, &INT_SUFFIXES);
    let float_suffix_len = suffix_match(scalars, p, &FLOAT_SUFFIXES);
    let suffix_len = int_suffix_len.max(float_suffix_len);
    let used_int_suffix = int_suffix_len > 0 && int_suffix_len >= float_suffix_len;
    let used_float_suffix = float_suffix_len > 0 && float_suffix_len > int_suffix_len;
    let j = p + suffix_len;
    result.ok = true;
    result.next = j;
    let lexeme = lexeme_slice(source, start, j);
    let underscore_ok = numeric_underscore_ok(lexeme);
    let int_grammar_ok = matches_integer_literal_lexeme(lexeme);
    if !underscore_ok || !int_grammar_ok {
        emit_diag(&mut result.diags, "E-SRC-0304", span_of_text(source, start, j));
    }
    if !is_based
        && !saw_exp
        && !used_int_suffix
        && !used_float_suffix
        && underscore_ok
        && matches_decimal_integer_lexeme(lexeme)
        && decimal_leading_zero(lexeme)
    {
        spec_rule!("Warn-DecimalLeadingZero");
        emit_diag(&mut result.diags, "W-SRC-0301", span_of_text(source, start, j));
    }
    result
}

pub fn scan_float_literal(source: &SourceFile, start: usize) -> LiteralScanResult {
    spec_rule!("Lex-Float");
    spec_rule!("Lex-Numeric-Err");
    let mut result = LiteralScanResult { ok: false, next: start, ..LiteralScanResult::default() };
    let scalars = &source.scalars;
    let n = scalars.len();
    if start >= n || !is_dec_digit(scalars[start]) {
        return result;
    }
    let mut p = scan_run_with_underscore(scalars, start, is_dec_digit);
    if p == start {
        return result;
    }
    if p >= n || scalars[p] != ch('.') {
        return result;
    }
    if p + 1 < n && scalars[p + 1] == ch('.') {
        return result;
    }
    p += 1;
    p = scan_run_with_underscore(scalars, p, is_dec_digit);
    if p < n && is_exp_marker(scalars[p]) {
        let mut exp_run = p + 1;
        if exp_run < n && (scalars[exp_run] == ch('+') || scalars[exp_run] == ch('-')) {
            exp_run += 1;
        }
        p = scan_run_with_underscore(scalars, exp_run, is_dec_digit);
    }
    let int_suffix_len = suffix_match(scalars, p, &INT_SUFFIXES);
    let float_suffix_len = suffix_match(scalars, p, &FLOAT_SUFFIXES);
    let j = p + int_suffix_len.max(float_suffix_len);
    result.ok = true;
    result.next = j;
    let lexeme = lexeme_slice(source, start, j);
    if !numeric_underscore_ok(lexeme) || !matches_float_literal_lexeme(lexeme) {
        emit_diag(&mut result.diags, "E-SRC-0304", span_of_text(source, start, j));
    }
    result
}

pub fn scan_string_literal(source: &SourceFile, start: usize) -> LiteralScanResult {
    spec_rule!("Lex-String");
    spec_rule!("Lex-String-Unterminated");
    spec_rule!("Lex-String-BadEscape");
    let mut result = LiteralScanResult { ok: false, next: start, ..LiteralScanResult::default() };
    let scalars = &source.scalars;
    if start >= scalars.len() || scalars[start] != ch('"') {
        return result;
    }
    let term = find_terminator(scalars, start, ch('"'));
    if !term.closed {
        emit_diag(&mut result.diags, "E-SRC-0301", span_of_text(source, start, start + 1));
        result.next = term.index;
        return result;
    }
    let j = term.index + 1;
    result.ok = true;
    result.next = j;
    result.range = Some(ScalarRange { start, end: j });
    if let Some(bad) = first_bad_escape(scalars, start, term.index) {
        let bad_span = span_of_text(source, bad, bad + 1);
        record_first_bad_escape_definition("def.FirstBadStringEscape", "string", &bad_span, bad);
        emit_diag(&mut result.diags, "E-SRC-0302", bad_span);
    }
    result
}

pub fn scan_char_literal(source: &SourceFile, start: usize) -> LiteralScanResult {
    spec_rule!("Lex-Char");
    spec_rule!("Lex-Char-Unterminated");
    spec_rule!("Lex-Char-BadEscape");
    spec_rule!("Lex-Char-Invalid");
    let mut result = LiteralScanResult { ok: false, next: start, ..LiteralScanResult::default() };
    let scalars = &source.scalars;
    if start >= scalars.len() || scalars[start] != ch('\'') {
        return result;
    }
    let term = find_terminator(scalars, start, ch('\''));
    if !term.closed {
        emit_diag(&mut result.diags, "E-SRC-0303", span_of_text(source, start, start + 1));
        result.next = term.index;
        return result;
    }
    let j = term.index + 1;
    result.ok = true;
    result.next = j;
    result.range = Some(ScalarRange { start, end: j });
    if let Some(bad) = first_bad_escape(scalars, start, term.index) {
        let bad_span = span_of_text(source, bad, bad + 1);
        record_first_bad_escape_definition("def.FirstBadCharEscape", "char", &bad_span, bad);
        emit_diag(&mut result.diags, "E-SRC-0302", bad_span);
    }
    if char_scalar_count(scalars, start, term.index) != 1 {
        emit_diag(&mut result.diags, "E-SRC-0303", span_of_text(source, start, start + 1));
    }
    result
}
