use std::sync::Arc;

/// A Unicode scalar value (never a surrogate, never above U+10FFFF).
pub type UnicodeScalar = u32;
pub type Scalars = Vec<UnicodeScalar>;

pub const LF: UnicodeScalar = 0x0A;
pub const CR: UnicodeScalar = 0x0D;
pub const BOM: UnicodeScalar = 0xFEFF;

pub fn is_scalar_value(value: u32) -> bool {
    value <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&value)
}

pub fn utf8_len(u: UnicodeScalar) -> usize {
    if u <= 0x7F {
        1
    } else if u <= 0x7FF {
        2
    } else if u <= 0xFFFF {
        3
    } else {
        4
    }
}

pub fn utf8_offsets(scalars: &[UnicodeScalar]) -> Vec<usize> {
    let mut offsets = Vec::with_capacity(scalars.len() + 1);
    let mut acc = 0usize;
    offsets.push(0);
    for &u in scalars {
        acc += utf8_len(u);
        offsets.push(acc);
    }
    offsets
}

pub fn line_starts(scalars: &[UnicodeScalar]) -> Vec<usize> {
    let mut starts = vec![0usize];
    let mut acc = 0usize;
    for &u in scalars {
        if u == LF {
            starts.push(acc + 1);
        }
        acc += utf8_len(u);
    }
    starts
}

pub fn byte_len_from_scalars(scalars: &[UnicodeScalar]) -> usize {
    scalars.iter().map(|&u| utf8_len(u)).sum()
}

pub fn append_utf8(out: &mut String, u: UnicodeScalar) {
    out.push(char::from_u32(u).expect("UnicodeScalar holds a scalar value"));
}

pub fn encode_utf8(scalars: &[UnicodeScalar]) -> String {
    let mut out = String::with_capacity(byte_len_from_scalars(scalars));
    for &u in scalars {
        append_utf8(&mut out, u);
    }
    out
}

#[derive(Debug, Clone, Default)]
pub struct SourceFile {
    pub path: Arc<str>,
    pub bytes: Vec<u8>,
    pub scalars: Scalars,
    pub text: String,
    pub byte_len: usize,
    pub line_starts: Vec<usize>,
    pub line_count: usize,
    /// UTF-8 byte offset of every scalar, plus the end offset (`scalars.len() + 1` entries).
    pub offsets: Vec<usize>,
}
