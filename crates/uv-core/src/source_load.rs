use std::sync::Arc;

use crate::diagnostic_messages::make_diagnostic_by_id;
use crate::diagnostics::{emit, DiagnosticStream};
use crate::source_text::{
    encode_utf8, line_starts, utf8_offsets, Scalars, SourceFile, UnicodeScalar, BOM, CR, LF,
};
use crate::span::{span_of, Span};
use crate::spec_rule;
use crate::unicode::{first_prohibited_outside_literal, no_prohibited};

fn is_continuation(byte: u8) -> bool {
    (byte & 0xC0) == 0x80
}

fn is_surrogate(u: u32) -> bool {
    (0xD800..=0xDFFF).contains(&u)
}

fn decode_utf8_internal(bytes: &[u8]) -> Option<Scalars> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        if b0 <= 0x7F {
            out.push(b0);
            i += 1;
            continue;
        }
        if (b0 & 0xE0) == 0xC0 {
            if i + 1 >= bytes.len() {
                return None;
            }
            let b1 = bytes[i + 1];
            if !is_continuation(b1) {
                return None;
            }
            let u = ((b0 & 0x1F) << 6) | (b1 as u32 & 0x3F);
            if u < 0x80 {
                return None;
            }
            out.push(u);
            i += 2;
            continue;
        }
        if (b0 & 0xF0) == 0xE0 {
            if i + 2 >= bytes.len() {
                return None;
            }
            let (b1, b2) = (bytes[i + 1], bytes[i + 2]);
            if !is_continuation(b1) || !is_continuation(b2) {
                return None;
            }
            let u = ((b0 & 0x0F) << 12) | ((b1 as u32 & 0x3F) << 6) | (b2 as u32 & 0x3F);
            if u < 0x800 || is_surrogate(u) {
                return None;
            }
            out.push(u);
            i += 3;
            continue;
        }
        if (b0 & 0xF8) == 0xF0 {
            if i + 3 >= bytes.len() {
                return None;
            }
            let (b1, b2, b3) = (bytes[i + 1], bytes[i + 2], bytes[i + 3]);
            if !is_continuation(b1) || !is_continuation(b2) || !is_continuation(b3) {
                return None;
            }
            let u = ((b0 & 0x07) << 18)
                | ((b1 as u32 & 0x3F) << 12)
                | ((b2 as u32 & 0x3F) << 6)
                | (b3 as u32 & 0x3F);
            if !(0x10000..=0x10FFFF).contains(&u) {
                return None;
            }
            out.push(u);
            i += 4;
            continue;
        }
        return None;
    }
    Some(out)
}

pub struct DecodeResult {
    pub scalars: Scalars,
    pub ok: bool,
}

pub fn decode(bytes: &[u8]) -> DecodeResult {
    match decode_utf8_internal(bytes) {
        None => {
            spec_rule!("Decode-Err");
            DecodeResult { scalars: Vec::new(), ok: false }
        }
        Some(scalars) => {
            spec_rule!("Decode-Ok");
            DecodeResult { scalars, ok: true }
        }
    }
}

#[derive(Default)]
pub struct StripBomResult {
    pub scalars: Scalars,
    pub had_bom: bool,
    pub embedded_index: Option<usize>,
}

pub fn strip_bom(scalars: &[UnicodeScalar]) -> StripBomResult {
    let mut result = StripBomResult::default();
    if scalars.is_empty() {
        spec_rule!("StripBOM-Empty");
        return result;
    }
    let has_lead = scalars[0] == BOM;
    result.had_bom = has_lead;
    result.scalars = if has_lead { scalars[1..].to_vec() } else { scalars.to_vec() };
    if let Some(index) = result.scalars.iter().position(|&u| u == BOM) {
        result.embedded_index = Some(index);
        spec_rule!("StripBOM-Embedded");
        return result;
    }
    if has_lead {
        spec_rule!("StripBOM-Start");
    } else {
        spec_rule!("StripBOM-None");
    }
    result
}

pub fn normalize_lf(scalars: &[UnicodeScalar]) -> Scalars {
    if scalars.is_empty() {
        spec_rule!("Norm-Empty");
        return Vec::new();
    }
    let mut out = Vec::with_capacity(scalars.len());
    let mut i = 0usize;
    while i < scalars.len() {
        let c = scalars[i];
        if c == CR {
            if i + 1 < scalars.len() && scalars[i + 1] == LF {
                spec_rule!("Norm-CRLF");
                out.push(LF);
                i += 2;
            } else {
                spec_rule!("Norm-CR");
                out.push(LF);
                i += 1;
            }
            continue;
        }
        if c == LF {
            spec_rule!("Norm-LF");
        } else {
            spec_rule!("Norm-Other");
        }
        out.push(c);
        i += 1;
    }
    out
}

fn build_span_source(path: &Arc<str>, bytes: &[u8], scalars: Scalars) -> SourceFile {
    let text = encode_utf8(&scalars);
    let starts = line_starts(&scalars);
    let offsets = utf8_offsets(&scalars);
    SourceFile {
        path: path.clone(),
        bytes: bytes.to_vec(),
        byte_len: text.len(),
        line_count: starts.len(),
        line_starts: starts,
        offsets,
        scalars,
        text,
    }
}

fn span_at_index(source: &SourceFile, index: usize) -> Span {
    span_of(source, source.offsets[index], source.offsets[index + 1])
}

#[derive(Default)]
pub struct SourceLoadResult {
    pub source: Option<SourceFile>,
    pub diags: DiagnosticStream,
}

pub fn load_source(path: &str, bytes: &[u8]) -> SourceLoadResult {
    let mut result = SourceLoadResult::default();
    let path: Arc<str> = Arc::from(path);
    spec_rule!("Step-Size");
    let decode_result = decode(bytes);
    if !decode_result.ok {
        spec_rule!("Step-Decode-Err");
        spec_rule!("NoSpan-Decode");
        if let Some(diag) = make_diagnostic_by_id("E-SRC-0101", None) {
            emit(&mut result.diags, diag);
        }
        spec_rule!("LoadSource-Err");
        return result;
    }
    spec_rule!("Step-Decode");
    let stripped = strip_bom(&decode_result.scalars);
    spec_rule!("Step-BOM");
    let had_bom = stripped.had_bom;
    let normalized_scalars = normalize_lf(&stripped.scalars);
    spec_rule!("Step-Norm");
    let source = build_span_source(&path, bytes, normalized_scalars);
    if had_bom {
        spec_rule!("req.LeadingBOMWarningPersistence");
        spec_rule!("Span-BOM-Warn");
        let end = source.byte_len.min(1);
        if let Some(diag) = make_diagnostic_by_id("W-SRC-0101", Some(span_of(&source, 0, end))) {
            emit(&mut result.diags, diag);
        }
    }
    if stripped.embedded_index.is_some() {
        spec_rule!("Step-EmbeddedBOM-Err");
        let bom_index = source.scalars.iter().position(|&u| u == BOM).unwrap_or(0);
        spec_rule!("Span-BOM-Embedded");
        if let Some(diag) =
            make_diagnostic_by_id("E-SRC-0103", Some(span_at_index(&source, bom_index)))
        {
            emit(&mut result.diags, diag);
        }
        spec_rule!("LoadSource-Err");
        return result;
    }
    spec_rule!("Step-LineMap");
    if !no_prohibited(&source.scalars) {
        spec_rule!("Step-Prohibited-Err");
        let prohibited_index = first_prohibited_outside_literal(&source.scalars).unwrap_or(0);
        spec_rule!("Span-Prohibited");
        if let Some(diag) =
            make_diagnostic_by_id("E-SRC-0104", Some(span_at_index(&source, prohibited_index)))
        {
            emit(&mut result.diags, diag);
        }
        spec_rule!("LoadSource-Err");
        return result;
    }
    spec_rule!("Step-Prohibited");
    result.source = Some(source);
    spec_rule!("LoadSource-Ok");
    result
}
