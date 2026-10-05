//! The types of literals.
//!
//! An integer literal has the type of its suffix, or `i32`, and must fit it. A float
//! literal has the type of its suffix, or `f32`. Against an expected type an unsuffixed
//! literal takes that type instead, and `null` is a raw pointer.

use uv_core::numeric_literals::{parse_int_core, strip_int_suffix};
use uv_source::ast;
use uv_source::lexer::token::{Token, TokenKind};

use super::expr_result::ExprTypeResult;
use super::types::*;

const POINTER_SIZE_BITS: u32 = 64;

const INT_SUFFIXES: [&str; 12] =
    ["i128", "u128", "isize", "usize", "i64", "u64", "i32", "u32", "i16", "u16", "i8", "u8"];
const FLOAT_SUFFIXES: [&str; 4] = ["f16", "f32", "f64", "f"];

/// The first suffix that ends the lexeme and leaves something before it.
fn match_suffix(lexeme: &str, suffixes: &[&'static str]) -> Option<&'static str> {
    suffixes.iter().copied().find(|suffix| lexeme.len() > suffix.len() && lexeme.ends_with(suffix))
}

fn int_suffix(lit: &Token) -> Option<&'static str> {
    (lit.kind == TokenKind::IntLiteral).then(|| match_suffix(&lit.lexeme, &INT_SUFFIXES)).flatten()
}

fn float_suffix(lit: &Token) -> Option<&'static str> {
    (lit.kind == TokenKind::FloatLiteral).then(|| match_suffix(&lit.lexeme, &FLOAT_SUFFIXES)).flatten()
}

/// Digits with underscores between them, at least one digit.
fn match_digit_run_syntax(text: &str, is_digit: fn(u8) -> bool) -> bool {
    let mut digits = text.bytes().filter(|&c| c != b'_').peekable();
    digits.peek().is_some() && digits.all(is_digit)
}

/// Whether the lexeme is written as an integer literal at all; one that is but has no
/// value is merely too large.
fn is_integer_literal_syntax(lexeme: &str) -> bool {
    let core = match match_suffix(lexeme, &INT_SUFFIXES) {
        Some(suffix) => &lexeme[..lexeme.len() - suffix.len()],
        None => lexeme,
    };
    let underscores_ok = !core.is_empty()
        && !core.starts_with('_')
        && !core.ends_with('_')
        && !["0x_", "0o_", "0b_"].iter().any(|prefix| core.starts_with(prefix));
    if !underscores_ok {
        return false;
    }
    match core.as_bytes() {
        [b'0', b'x', ..] => match_digit_run_syntax(&core[2..], |c| c.is_ascii_hexdigit()),
        [b'0', b'o', ..] => match_digit_run_syntax(&core[2..], |c| (b'0'..=b'7').contains(&c)),
        [b'0', b'b', ..] => match_digit_run_syntax(&core[2..], |c| c == b'0' || c == b'1'),
        _ => match_digit_run_syntax(core, |c| c.is_ascii_digit()),
    }
}

fn int_value(lit: &Token) -> Option<u128> {
    (lit.kind == TokenKind::IntLiteral).then(|| parse_int_core(strip_int_suffix(&lit.lexeme))).flatten()
}

fn is_int_type_name(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "isize" | "usize")
}

/// Whether a non-negative literal fits the integer type. A signed type takes values up
/// to its maximum: the minimum is only reachable by negating.
fn in_range_int(value: u128, name: &str) -> bool {
    let width = match name {
        "i8" | "u8" => 8,
        "i16" | "u16" => 16,
        "i32" | "u32" => 32,
        "i64" | "u64" => 64,
        "i128" | "u128" => 128,
        "isize" | "usize" => POINTER_SIZE_BITS,
        _ => return false,
    };
    let value_bits = if name.starts_with('u') { width } else { width - 1 };
    value_bits >= 128 || value < (1u128 << value_bits)
}

pub fn type_literal_expr(expr: &ast::LiteralExpr) -> ExprTypeResult {
    let lit = &expr.literal;
    let prim = |name: &str| ExprTypeResult::typed(make_type_prim(name));
    match lit.kind {
        TokenKind::IntLiteral => {
            let Some(value) = int_value(lit) else {
                let malformed = !is_integer_literal_syntax(&lit.lexeme);
                return ExprTypeResult::failed(malformed.then_some("E-SRC-0304"));
            };
            let name = int_suffix(lit).unwrap_or("i32");
            if in_range_int(value, name) {
                prim(name)
            } else {
                ExprTypeResult::failed(None)
            }
        }
        TokenKind::FloatLiteral => match float_suffix(lit) {
            Some(suffix) if suffix != "f" => prim(suffix),
            _ => prim("f32"),
        },
        TokenKind::BoolLiteral => prim("bool"),
        TokenKind::CharLiteral => prim("char"),
        TokenKind::StringLiteral => ExprTypeResult::typed(make_type_string(Some(StringState::View))),
        _ => ExprTypeResult::failed(None),
    }
}

fn strip_perm_refine(ty: &TypeRef) -> &TypeRef {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    current
}

/// `null` is expected where a raw pointer is.
pub fn null_literal_expected(expected: &TypeRef) -> bool {
    matches!(strip_perm_refine(expected).as_deref().map(|ty| &ty.node), Some(TypeNode::RawPtr { .. }))
}

/// Whether a literal may stand where the type is expected; a failure may name a rule.
pub fn check_literal_expr(expr: &ast::LiteralExpr, expected: &TypeRef) -> Result<(), Option<&'static str>> {
    let Some(base) = strip_perm_refine(expected).as_deref() else {
        return Err(None);
    };
    let lit = &expr.literal;
    let prim_name = match &base.node {
        TypeNode::Prim(name) => Some(name.as_str()),
        _ => None,
    };
    let accept = |ok: bool| if ok { Ok(()) } else { Err(None) };
    match lit.kind {
        TokenKind::IntLiteral => {
            let Some(name) = prim_name.filter(|name| is_int_type_name(name)) else {
                return Err(None);
            };
            match int_value(lit) {
                Some(value) => accept(in_range_int(value, name)),
                None => Err((!is_integer_literal_syntax(&lit.lexeme)).then_some("E-SRC-0304")),
            }
        }
        TokenKind::FloatLiteral => {
            let Some(name) = prim_name.filter(|name| matches!(*name, "f16" | "f32" | "f64")) else {
                return Err(None);
            };
            match float_suffix(lit) {
                Some(suffix) if suffix != "f" && suffix != name => Err(Some("E-TYP-1531")),
                _ => Ok(()),
            }
        }
        TokenKind::BoolLiteral => accept(prim_name == Some("bool")),
        TokenKind::CharLiteral => accept(prim_name == Some("char")),
        TokenKind::StringLiteral => accept(matches!(&base.node, TypeNode::String(Some(StringState::View)))),
        TokenKind::NullLiteral => {
            if null_literal_expected(expected) {
                Ok(())
            } else {
                Err(Some("NullLiteral-Infer-Err"))
            }
        }
        _ => Err(None),
    }
}
