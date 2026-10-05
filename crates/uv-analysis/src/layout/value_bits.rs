//! The bytes of values: encoding literals and structured constants in the layout of
//! their type, and judging whether given bytes are a value of a type. Little-endian
//! throughout.

use uv_source::ast;
use uv_source::lexer::token::{Token, TokenKind};

use super::{
    dyn_layout_of, enum_layout_of, enum_record_payload_member_layout, enum_tuple_payload_member_layout, prim_size,
    prim_size_with, ptr_size, range_layout_of, resolve_enum_layout_options, size_of, tuple_layout_of,
};
use crate::composite::enums::enum_discriminants;
use crate::context::ScopeContext;
use crate::resolve::scopes::id_eq;
use crate::typing::type_lookup::lookup_enum_decl;
use crate::typing::types::*;

/// The pointer size constant encoding assumes when it has no target to ask.
const DEFAULT_PTR_SIZE: u64 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueRangeKind {
    To,
    ToInclusive,
    Full,
    From,
    Exclusive,
    Inclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawPtrVal {
    pub qual: RawPtrQual,
    pub addr: u64,
}

#[derive(Debug, Clone)]
pub enum EnumPayloadVal {
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
}

/// A value, as far as its bytes are concerned.
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Char(u32),
    Int { r#type: String, value: u128 },
    Float { r#type: String, bits: u64 },
    Unit,
    Ptr { state: PtrState, addr: u64 },
    RawPtr(RawPtrVal),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    Slice { ptr: RawPtrVal, length: u64 },
    Range { kind: ValueRangeKind, lo: Option<u64>, hi: Option<u64> },
    Record(Vec<(String, Value)>),
    Enum { variant: String, payload: Option<EnumPayloadVal> },
    Modal { state: String, payload: Option<Box<Value>> },
    Union { member: TypeRef, value: Option<Box<Value>> },
    Dynamic { data: u64, vtable: u64 },
    String(Vec<u8>),
    Bytes(Vec<u8>),
}

fn align_up(value: u64, align: u64) -> u64 {
    match value.checked_rem(align) {
        None | Some(0) => value,
        Some(rem) => value + (align - rem),
    }
}

/// The low `n` bytes; bytes beyond the sixteenth are zero.
fn le_bytes(value: u128, n: u64) -> Vec<u8> {
    (0..n).map(|i| if i < 16 { (value >> (8 * i)) as u8 } else { 0 }).collect()
}

fn le_bytes_u64(value: u64, n: u64) -> Vec<u8> {
    le_bytes(u128::from(value), n)
}

fn bits_to_uint(bits: &[u8]) -> Option<u64> {
    (bits.len() <= 8).then(|| bits.iter().enumerate().fold(0, |out, (i, byte)| out | (u64::from(*byte) << (8 * i))))
}

fn is_unicode_scalar(value: u64) -> bool {
    value <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&value)
}

fn is_int_type_name(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize")
}

fn is_float_type_name(name: &str) -> bool {
    matches!(name, "f16" | "f32" | "f64")
}

/// Without the first suffix that ends the lexeme and leaves something before it.
fn strip_suffix_of<'a>(lexeme: &'a str, suffixes: &[&str]) -> &'a str {
    suffixes
        .iter()
        .find_map(|suffix| lexeme.strip_suffix(suffix).filter(|core| !core.is_empty()))
        .unwrap_or(lexeme)
}

fn strip_int_suffix(lexeme: &str) -> &str {
    strip_suffix_of(lexeme, &["i128", "u128", "isize", "usize", "i64", "u64", "i32", "u32", "i16", "u16", "i8", "u8"])
}

fn strip_float_suffix(lexeme: &str) -> &str {
    strip_suffix_of(lexeme, &["f16", "f32", "f64", "f"])
}

fn digit_value(c: u8, base: u32) -> Option<u32> {
    let digit = match c {
        b'0'..=b'9' => u32::from(c - b'0'),
        b'a'..=b'f' if base > 10 => 10 + u32::from(c - b'a'),
        b'A'..=b'F' if base > 10 => 10 + u32::from(c - b'A'),
        _ => return None,
    };
    (digit < base).then_some(digit)
}

/// One scalar value at the offset and the number of bytes it takes. Overlong forms and
/// surrogates are rejected.
fn decode_utf8_one(data: &[u8], offset: usize) -> Option<(u32, usize)> {
    let b0 = u32::from(*data.get(offset)?);
    let continuation = |index: usize| {
        let byte = u32::from(*data.get(offset + index)?);
        (byte & 0xC0 == 0x80).then_some(byte & 0x3F)
    };
    let (cp, len, min) = if b0 < 0x80 {
        return Some((b0, 1));
    } else if b0 & 0xE0 == 0xC0 {
        (((b0 & 0x1F) << 6) | continuation(1)?, 2, 0x80)
    } else if b0 & 0xF0 == 0xE0 {
        // Both continuation bytes must be present before either is examined.
        data.get(offset + 2)?;
        let (b1, b2) = (continuation(1)?, continuation(2)?);
        (((b0 & 0x0F) << 12) | (b1 << 6) | b2, 3, 0x800)
    } else if b0 & 0xF8 == 0xF0 {
        data.get(offset + 3)?;
        let (b1, b2, b3) = (continuation(1)?, continuation(2)?, continuation(3)?);
        (((b0 & 0x07) << 18) | (b1 << 12) | (b2 << 6) | b3, 4, 0x10000)
    } else {
        return None;
    };
    (cp >= min && is_unicode_scalar(u64::from(cp))).then_some((cp, len))
}

fn encode_utf8(value: u32, out: &mut Vec<u8>) {
    if value <= 0x7F {
        out.push(value as u8);
    } else if value <= 0x7FF {
        out.extend([(0xC0 | (value >> 6)) as u8, (0x80 | (value & 0x3F)) as u8]);
    } else if value <= 0xFFFF {
        out.extend([(0xE0 | (value >> 12)) as u8, (0x80 | ((value >> 6) & 0x3F)) as u8, (0x80 | (value & 0x3F)) as u8]);
    } else {
        out.extend([
            (0xF0 | (value >> 18)) as u8,
            (0x80 | ((value >> 12) & 0x3F)) as u8,
            (0x80 | ((value >> 6) & 0x3F)) as u8,
            (0x80 | (value & 0x3F)) as u8,
        ]);
    }
}

fn parse_hex_scalar(digits: &[u8]) -> Option<u32> {
    if digits.is_empty() {
        return None;
    }
    let mut value: u32 = 0;
    for &c in digits {
        let digit = digit_value(c, 16)?;
        if value > (0x10FFFF - digit) / 16 {
            return None;
        }
        value = value * 16 + digit;
    }
    is_unicode_scalar(u64::from(value)).then_some(value)
}

/// The bytes between the quotes of a character or string literal, with escapes
/// replaced: `\\ \" \' \n \r \t \0`, `\xHH` (any byte), `\u{H…}` (a scalar value).
fn decode_literal_bytes(inner: &[u8]) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut i = 0;
    while i < inner.len() {
        if inner[i] != b'\\' {
            let (_, len) = decode_utf8_one(inner, i)?;
            bytes.extend_from_slice(&inner[i..i + len]);
            i += len;
            continue;
        }
        let simple = match *inner.get(i + 1)? {
            b'\\' => Some(0x5C),
            b'"' => Some(0x22),
            b'\'' => Some(0x27),
            b'n' => Some(0x0A),
            b'r' => Some(0x0D),
            b't' => Some(0x09),
            b'0' => Some(0x00),
            b'x' | b'u' => None,
            _ => return None,
        };
        if let Some(byte) = simple {
            bytes.push(byte);
            i += 2;
        } else if inner[i + 1] == b'x' {
            let (d1, d2) = (digit_value(*inner.get(i + 2)?, 16)?, digit_value(*inner.get(i + 3)?, 16)?);
            bytes.push(((d1 << 4) | d2) as u8);
            i += 4;
        } else {
            if inner.get(i + 2) != Some(&b'{') {
                return None;
            }
            let start = i + 3;
            let close = start + inner.get(start..)?.iter().position(|&c| c == b'}')?;
            encode_utf8(parse_hex_scalar(&inner[start..close])?, &mut bytes);
            i = close + 1;
        }
    }
    Some(bytes)
}

fn quoted(lexeme: &str, quote: u8) -> Option<&[u8]> {
    let bytes = lexeme.as_bytes();
    (bytes.len() >= 2 && bytes[0] == quote && bytes[bytes.len() - 1] == quote).then(|| &bytes[1..bytes.len() - 1])
}

/// The scalar value of a character literal: its decoded bytes must be exactly one.
fn decode_char_literal(lexeme: &str) -> Option<u32> {
    let bytes = decode_literal_bytes(quoted(lexeme, b'\'')?)?;
    let (scalar, len) = decode_utf8_one(&bytes, 0)?;
    (len == bytes.len()).then_some(scalar)
}

/// The bytes of a string literal. `\xHH` may put any byte there, so the result need not
/// be UTF-8.
pub fn decode_string_literal_bytes(lexeme: &str) -> Option<Vec<u8>> {
    decode_literal_bytes(quoted(lexeme, b'"')?)
}

fn parse_int_core(core: &str) -> Option<u128> {
    let bytes = core.as_bytes();
    let (base, digits) = match bytes {
        [b'0', b'x' | b'X', rest @ ..] => (16, rest),
        [b'0', b'o' | b'O', rest @ ..] => (8, rest),
        [b'0', b'b' | b'B', rest @ ..] => (2, rest),
        _ => (10, bytes),
    };
    let mut value: u128 = 0;
    let mut saw_digit = false;
    for &c in digits.iter().filter(|&&c| c != b'_') {
        let digit = u128::from(digit_value(c, base)?);
        saw_digit = true;
        if value > (u128::MAX - digit) / u128::from(base) {
            return None;
        }
        value = value * u128::from(base) + digit;
    }
    saw_digit.then_some(value)
}

pub(crate) fn parse_int_literal_value(lexeme: &str) -> Option<u128> {
    if lexeme.is_empty() || lexeme.starts_with('-') {
        return None;
    }
    parse_int_core(strip_int_suffix(lexeme))
}

/// The reference reads the digits with `strtod`. Float literals are decimal, for which
/// Rust's parser rounds the same way; the forms only `strtod` takes (hexadecimal
/// floats, leading white space) do not occur in a literal.
fn parse_float_literal_value(lexeme: &str) -> Option<f64> {
    if lexeme.is_empty() || lexeme.starts_with('-') {
        return None;
    }
    let cleaned: String = strip_float_suffix(lexeme).chars().filter(|&c| c != '_').collect();
    let decimal = cleaned.bytes().all(|c| c.is_ascii_digit() || matches!(c, b'.' | b'e' | b'E' | b'+' | b'-'));
    if cleaned.is_empty() || !decimal {
        return None;
    }
    cleaned.parse().ok()
}

/// Single to half precision as the reference converts it. It differs from IEEE
/// conversion in places: a single-precision subnormal becomes zero, a value that rounds
/// up out of the subnormal range is rounded by the reference's own rule, and every NaN
/// has its lowest payload bit set.
fn float_to_half(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = (bits >> 16) & 0x8000;
    let exp = (bits >> 23) & 0xFF;
    let mantissa = bits & 0x7F_FFFF;
    if exp == 0xFF {
        let payload = if mantissa == 0 { 0 } else { (mantissa >> 13) | 1 };
        return (sign | 0x7C00 | payload) as u16;
    }
    if exp == 0 {
        return sign as u16;
    }
    let exp_adjusted = exp as i32 - 127 + 15;
    if exp_adjusted >= 31 {
        return (sign | 0x7C00) as u16;
    }
    if exp_adjusted <= 0 {
        if exp_adjusted < -10 {
            return sign as u16;
        }
        let mant = mantissa | 0x80_0000;
        let shift = (14 - exp_adjusted) as u32;
        let rounded = (mant + (1 << (shift - 1)) + ((mant >> shift) & 1)) >> shift;
        return (sign | rounded) as u16;
    }
    let rounded = (mantissa + 0xFFF + ((mantissa >> 13) & 1)) >> 13;
    if rounded == 0x400 {
        return (sign | (((exp_adjusted + 1) as u32) << 10)) as u16;
    }
    (sign | ((exp_adjusted as u32) << 10) | (rounded & 0x3FF)) as u16
}

/// Fields at their offsets in a zeroed block.
fn struct_bits(field_bits: &[Vec<u8>], offsets: &[u64], size: u64) -> Option<Vec<u8>> {
    if field_bits.len() != offsets.len() {
        return None;
    }
    let mut bits = vec![0; size as usize];
    for (field, &offset) in field_bits.iter().zip(offsets) {
        if !copy_bits_at(&mut bits, offset, field) {
            return None;
        }
    }
    Some(bits)
}

fn copy_bits_at(out: &mut [u8], offset: u64, bits: &[u8]) -> bool {
    let offset = offset as usize;
    if offset > out.len() || bits.len() > out.len() - offset {
        return false;
    }
    out[offset..offset + bits.len()].copy_from_slice(bits);
    true
}

/// A discriminant followed, at the payload's alignment, by the payload.
fn tagged_bits(disc_bits: &[u8], payload_bits: &[u8], disc_size: u64, payload_size: u64, payload_align: u64, size: u64) -> Option<Vec<u8>> {
    let payload_off = align_up(disc_size, payload_align);
    if size < payload_off + payload_size
        || disc_bits.len() as u64 != disc_size
        || payload_bits.len() as u64 != payload_size
    {
        return None;
    }
    let mut out = vec![0; size as usize];
    out[..disc_bits.len()].copy_from_slice(disc_bits);
    out[payload_off as usize..payload_off as usize + payload_bits.len()].copy_from_slice(payload_bits);
    Some(out)
}

fn strip_perm_refine(ty: &TypeRef) -> &TypeRef {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    current
}

/// The kind of range value a range type holds and the types of its bounds.
fn range_shape(ty: &TypeRef) -> Option<(ValueRangeKind, Vec<TypeRef>)> {
    Some(match &strip_perm_refine(ty).as_deref()?.node {
        TypeNode::Range(base) => (ValueRangeKind::Exclusive, vec![base.clone(), base.clone()]),
        TypeNode::RangeInclusive(base) => (ValueRangeKind::Inclusive, vec![base.clone(), base.clone()]),
        TypeNode::RangeFrom(base) => (ValueRangeKind::From, vec![base.clone()]),
        TypeNode::RangeTo(base) => (ValueRangeKind::To, vec![base.clone()]),
        TypeNode::RangeToInclusive(base) => (ValueRangeKind::ToInclusive, vec![base.clone()]),
        TypeNode::RangeFull => (ValueRangeKind::Full, Vec::new()),
        _ => return None,
    })
}

/// A range bound, given as a number, as a value of the bound's type.
fn range_bound_value_for_type(bound_type: &TypeRef, raw: u64) -> Option<Value> {
    let TypeNode::Prim(name) = &strip_perm_refine(bound_type).as_deref()?.node else {
        return None;
    };
    match name.as_str() {
        name if is_int_type_name(name) => Some(Value::Int { r#type: name.to_string(), value: u128::from(raw) }),
        "char" => is_unicode_scalar(raw).then_some(Value::Char(raw as u32)),
        "bool" => (raw <= 1).then_some(Value::Bool(raw != 0)),
        _ => None,
    }
}

fn enum_payload_bits(
    ctx: &ScopeContext<'_>,
    decl: &ast::EnumDecl,
    generic_args: &[TypeRef],
    variant: &ast::VariantDecl,
    payload: &Option<EnumPayloadVal>,
    payload_size: u64,
) -> Option<Vec<u8>> {
    let mut out = vec![0; payload_size as usize];
    match (&variant.payload_opt, payload) {
        (None, None) => {}
        (Some(ast::VariantPayload::VariantPayloadTuple(tuple)), Some(EnumPayloadVal::Tuple(elements)))
            if elements.len() == tuple.elements.len() =>
        {
            for (index, element) in elements.iter().enumerate() {
                let member = enum_tuple_payload_member_layout(ctx, decl, variant, generic_args, index)?;
                if !copy_bits_at(&mut out, member.offset, &value_bits(ctx, &member.r#type, element)?) {
                    return None;
                }
            }
        }
        (Some(ast::VariantPayload::VariantPayloadRecord(record)), Some(EnumPayloadVal::Record(fields))) => {
            for field in &record.fields {
                let (_, field_value) = fields.iter().find(|(name, _)| id_eq(name, &field.name))?;
                let member = enum_record_payload_member_layout(ctx, decl, variant, generic_args, &field.name)?;
                if !copy_bits_at(&mut out, member.offset, &value_bits(ctx, &member.r#type, field_value)?) {
                    return None;
                }
            }
        }
        _ => return None,
    }
    Some(out)
}

/// Every member's bytes are a value of its type, and the bytes between members are zero.
fn valid_struct_bits(ctx: &ScopeContext<'_>, types: &[TypeRef], offsets: &[u64], size: u64, bits: &[u8]) -> bool {
    if types.len() != offsets.len() || bits.len() as u64 != size {
        return false;
    }
    let mut covered = vec![false; bits.len()];
    for (ty, &offset) in types.iter().zip(offsets) {
        let (Some(elem_size), offset) = (size_of(ctx, ty), offset as usize) else {
            return false;
        };
        let elem_size = elem_size as usize;
        if offset > bits.len() || elem_size > bits.len() - offset {
            return false;
        }
        if !valid_value(ctx, ty, &bits[offset..offset + elem_size]) {
            return false;
        }
        covered[offset..offset + elem_size].fill(true);
    }
    bits.iter().zip(covered).all(|(byte, covered)| covered || *byte == 0)
}

/// The bytes of a literal of a primitive type, or of `null` as a raw pointer.
pub fn encode_const(type_ref: &TypeRef, lit: &Token) -> Option<Vec<u8>> {
    let name = match &type_ref.as_deref()?.node {
        TypeNode::Prim(name) => name.as_str(),
        TypeNode::RawPtr { .. } if lit.kind == TokenKind::NullLiteral => return Some(le_bytes_u64(0, DEFAULT_PTR_SIZE)),
        _ => return None,
    };
    match name {
        "bool" => {
            return (lit.kind == TokenKind::BoolLiteral).then(|| le_bytes_u64(u64::from(lit.lexeme == "true"), 1));
        }
        "char" => {
            if lit.kind != TokenKind::CharLiteral {
                return None;
            }
            return Some(le_bytes_u64(u64::from(decode_char_literal(&lit.lexeme)?), 4));
        }
        "()" | "!" => return Some(Vec::new()),
        _ => {}
    }
    match lit.kind {
        TokenKind::IntLiteral if is_int_type_name(name) => {
            let value = parse_int_literal_value(&lit.lexeme)?;
            Some(le_bytes(value, prim_size_with(DEFAULT_PTR_SIZE, name)?))
        }
        TokenKind::FloatLiteral if is_float_type_name(name) => {
            let value = parse_float_literal_value(&lit.lexeme)?;
            Some(match name {
                "f64" => le_bytes_u64(value.to_bits(), 8),
                "f32" => le_bytes_u64(u64::from((value as f32).to_bits()), 4),
                _ => le_bytes_u64(u64::from(float_to_half(value as f32)), 2),
            })
        }
        _ => None,
    }
}

/// Whether the bytes are a value of the type. Nominal types, unions, functions and
/// refinements are never accepted: the reference has no rule for them.
pub fn valid_value(ctx: &ScopeContext<'_>, type_ref: &TypeRef, bits: &[u8]) -> bool {
    let Some(ty) = type_ref.as_deref() else {
        return false;
    };
    let ptr = ptr_size(ctx);
    let len = bits.len() as u64;
    match &ty.node {
        TypeNode::Prim(name) => match name.as_str() {
            "bool" => matches!(bits, [0] | [1]),
            "char" => len == 4 && bits_to_uint(bits).is_some_and(is_unicode_scalar),
            "()" => bits.is_empty(),
            "!" => false,
            name => prim_size(ctx, name) == Some(len),
        },
        TypeNode::Perm { base, .. } => valid_value(ctx, base, bits),
        TypeNode::Ptr { state, .. } => {
            let zero = bits.iter().all(|byte| *byte == 0);
            match state {
                Some(PtrState::Valid) => len == ptr && !zero,
                Some(PtrState::Null) => len == ptr && zero,
                _ => len == ptr,
            }
        }
        TypeNode::RawPtr { .. } => len == ptr,
        TypeNode::Tuple(elements) => tuple_layout_of(ctx, elements)
            .is_some_and(|layout| valid_struct_bits(ctx, elements, &layout.offsets, layout.layout.size, bits)),
        TypeNode::Array { element, length, .. } => {
            let Some(elem_size) = size_of(ctx, element) else {
                return false;
            };
            if Some(len) != elem_size.checked_mul(*length) {
                return false;
            }
            // Elements without size are still each judged, on no bytes.
            (0..*length).all(|i| {
                let start = (i * elem_size) as usize;
                valid_value(ctx, element, &bits[start..start + elem_size as usize])
            })
        }
        TypeNode::Slice(_) => len == 2 * ptr,
        _ if is_range_type(type_ref) => match (range_layout_of(ctx, type_ref), range_shape(type_ref)) {
            (Some(layout), Some((_, fields))) => {
                valid_struct_bits(ctx, &fields, &layout.offsets, layout.layout.size, bits)
            }
            _ => false,
        },
        TypeNode::Dynamic(_) => len == dyn_layout_of(ctx).layout.size,
        TypeNode::String(_) | TypeNode::Bytes(_) => size_of(ctx, type_ref) == Some(len),
        _ => false,
    }
}

/// The bytes of a value in the layout of its type; nothing when the value is not of the
/// type. Text encodes as zeroes of the right size, since its bytes live elsewhere.
/// Records, modals and unions have no encoding here.
pub fn value_bits(ctx: &ScopeContext<'_>, type_ref: &TypeRef, value: &Value) -> Option<Vec<u8>> {
    let ty = type_ref.as_deref()?;
    let ptr = ptr_size(ctx);
    match &ty.node {
        TypeNode::Prim(name) => match (name.as_str(), value) {
            ("bool", Value::Bool(value)) => Some(le_bytes_u64(u64::from(*value), 1)),
            ("char", Value::Char(value)) => Some(le_bytes_u64(u64::from(*value), 4)),
            ("()", Value::Unit) => Some(Vec::new()),
            ("!", _) => None,
            (name, Value::Int { r#type, value }) if r#type == name => Some(le_bytes(*value, prim_size(ctx, name)?)),
            (name, Value::Float { r#type, bits }) if r#type == name => Some(le_bytes_u64(*bits, prim_size(ctx, name)?)),
            _ => None,
        },
        TypeNode::Perm { base, .. } => value_bits(ctx, base, value),
        TypeNode::Ptr { state: expected, .. } => {
            let Value::Ptr { state, addr } = value else {
                return None;
            };
            let consistent = expected.is_none_or(|expected| expected == *state)
                && !(*state == PtrState::Valid && *addr == 0)
                && !(*state == PtrState::Null && *addr != 0);
            consistent.then(|| le_bytes_u64(*addr, ptr))
        }
        TypeNode::RawPtr { qual, .. } => match value {
            Value::RawPtr(raw) if raw.qual == *qual => Some(le_bytes_u64(raw.addr, ptr)),
            _ => None,
        },
        TypeNode::Tuple(types) => {
            let Value::Tuple(elements) = value else {
                return None;
            };
            if elements.len() != types.len() {
                return None;
            }
            let bits = types.iter().zip(elements).map(|(ty, value)| value_bits(ctx, ty, value)).collect::<Option<Vec<_>>>()?;
            let layout = tuple_layout_of(ctx, types)?;
            struct_bits(&bits, &layout.offsets, layout.layout.size)
        }
        TypeNode::Array { element, length, .. } => {
            let Value::Array(elements) = value else {
                return None;
            };
            if elements.len() as u64 != *length {
                return None;
            }
            size_of(ctx, element)?;
            let mut out = Vec::new();
            for value in elements {
                out.extend(value_bits(ctx, element, value)?);
            }
            Some(out)
        }
        TypeNode::Slice(_) => match value {
            Value::Slice { ptr: raw, length } if raw.qual == RawPtrQual::Imm => {
                Some([le_bytes_u64(raw.addr, ptr), le_bytes_u64(*length, ptr)].concat())
            }
            _ => None,
        },
        _ if is_range_type(type_ref) => {
            let Value::Range { kind, lo, hi } = value else {
                return None;
            };
            let (expected_kind, field_types) = range_shape(type_ref)?;
            if expected_kind != *kind {
                return None;
            }
            let layout = range_layout_of(ctx, type_ref)?;
            let bounds: Vec<&Option<u64>> = match kind {
                ValueRangeKind::Full => Vec::new(),
                ValueRangeKind::From => vec![lo],
                ValueRangeKind::To | ValueRangeKind::ToInclusive => vec![hi],
                ValueRangeKind::Exclusive | ValueRangeKind::Inclusive => vec![lo, hi],
            };
            let mut field_bits = Vec::with_capacity(bounds.len());
            for (bound, field_type) in bounds.into_iter().zip(&field_types) {
                let bound_value = range_bound_value_for_type(field_type, (*bound)?)?;
                field_bits.push(value_bits(ctx, field_type, &bound_value)?);
            }
            struct_bits(&field_bits, &layout.offsets, layout.layout.size)
        }
        TypeNode::Path { path, generic_args } | TypeNode::Apply { path, args: generic_args } => {
            let decl = lookup_enum_decl(ctx, path)?;
            let Value::Enum { variant: variant_name, payload } = value else {
                return None;
            };
            let variant_index = decl.variants.iter().position(|variant| id_eq(&variant.name, variant_name))?;
            let variant = &decl.variants[variant_index];
            let disc = *enum_discriminants(decl).ok()?.discs.get(variant_index)?;
            let layout = enum_layout_of(ctx, decl, generic_args, &resolve_enum_layout_options(&decl.attrs))?;
            let payload_bits = enum_payload_bits(ctx, decl, generic_args, variant, payload, layout.payload_size)?;
            let disc_size = prim_size(ctx, &layout.disc_type)?;
            let disc_value = Value::Int { r#type: layout.disc_type.clone(), value: u128::from(disc) };
            let disc_bits = value_bits(ctx, &make_type_prim(&layout.disc_type), &disc_value)?;
            tagged_bits(&disc_bits, &payload_bits, disc_size, layout.payload_size, layout.payload_align, layout.layout.size)
        }
        TypeNode::Dynamic(_) => {
            let Value::Dynamic { data, vtable } = value else {
                return None;
            };
            let bits = [le_bytes_u64(*data, ptr), le_bytes_u64(*vtable, ptr)];
            struct_bits(&bits, &[0, ptr], dyn_layout_of(ctx).layout.size)
        }
        TypeNode::String(_) => matches!(value, Value::String(_)).then_some(vec![0; size_of(ctx, type_ref)? as usize]),
        TypeNode::Bytes(_) => matches!(value, Value::Bytes(_)).then_some(vec![0; size_of(ctx, type_ref)? as usize]),
        _ => None,
    }
}

pub fn tuple_value(elements: &[Value], index: usize) -> Option<Value> {
    elements.get(index).cloned()
}

/// The tuple or array with one element replaced.
pub fn index_update(elements: &[Value], index: usize, value: Value) -> Option<Vec<Value>> {
    let mut updated = elements.to_vec();
    *updated.get_mut(index)? = value;
    Some(updated)
}

pub fn field_value<'v>(fields: &'v [(String, Value)], name: &str) -> Option<&'v Value> {
    fields.iter().find(|(field, _)| id_eq(field, name)).map(|(_, value)| value)
}

/// The record with the first field of that name replaced.
pub fn field_update(fields: &[(String, Value)], name: &str, value: Value) -> Option<Vec<(String, Value)>> {
    let mut updated = fields.to_vec();
    updated.iter_mut().find(|(field, _)| id_eq(field, name))?.1 = value;
    Some(updated)
}

/// The number of elements of an array or slice value.
pub fn slice_len(value: &Value) -> Option<usize> {
    match value {
        Value::Array(elements) => Some(elements.len()),
        Value::Slice { length, .. } => Some(*length as usize),
        _ => None,
    }
}
