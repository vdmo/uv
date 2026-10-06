//! Byte offsets of a text against the line and UTF-16 column positions the protocol uses.

use uv_core::span::Span;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LinePosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LineRange {
    pub start: LinePosition,
    pub end: LinePosition,
}

pub struct LineIndex {
    text: Vec<u8>,
    line_starts: Vec<usize>,
}

/// The scalar value at `offset` and the offset after it; bytes that do not form a
/// sequence count as one unit each.
fn decode_one(text: &[u8], offset: usize) -> (u32, usize) {
    let first = text[offset];
    let cont = |i: usize| u32::from(text[offset + i] & 0x3F);
    if first <= 0x7F {
        (u32::from(first), offset + 1)
    } else if first & 0xE0 == 0xC0 && offset + 1 < text.len() {
        (u32::from(first & 0x1F) << 6 | cont(1), offset + 2)
    } else if first & 0xF0 == 0xE0 && offset + 2 < text.len() {
        (u32::from(first & 0x0F) << 12 | cont(1) << 6 | cont(2), offset + 3)
    } else if first & 0xF8 == 0xF0 && offset + 3 < text.len() {
        (u32::from(first & 0x07) << 18 | cont(1) << 12 | cont(2) << 6 | cont(3), offset + 4)
    } else {
        (u32::from(first), offset + 1)
    }
}

fn utf16_units(scalar: u32) -> usize {
    if scalar > 0xFFFF {
        2
    } else {
        1
    }
}

impl LineIndex {
    pub fn new(text: &str) -> LineIndex {
        let text = text.as_bytes().to_vec();
        let mut line_starts = vec![0];
        line_starts.extend(text.iter().enumerate().filter(|(_, byte)| **byte == b'\n').map(|(i, _)| i + 1));
        LineIndex { text, line_starts }
    }

    pub fn text_bytes(&self) -> &[u8] {
        &self.text
    }

    pub fn byte_offset_at(&self, position: LinePosition) -> usize {
        let line = position.line.min(self.line_starts.len() - 1);
        let start = self.line_starts[line];
        let end = if line + 1 < self.line_starts.len() { self.line_starts[line + 1] } else { self.text.len() };
        let mut offset = start;
        let mut utf16 = 0;
        while offset < end && offset < self.text.len() && self.text[offset] != b'\n' {
            let before = offset;
            let (scalar, next) = decode_one(&self.text, offset);
            offset = next;
            let next_utf16 = utf16 + utf16_units(scalar);
            if next_utf16 > position.character {
                return before;
            }
            utf16 = next_utf16;
            if utf16 >= position.character {
                return offset;
            }
        }
        offset.min(self.text.len())
    }

    pub fn position_at(&self, byte_offset: usize) -> LinePosition {
        let clamped = byte_offset.min(self.text.len());
        let line = self.line_starts.partition_point(|start| *start <= clamped).saturating_sub(1);
        LinePosition { line, character: self.utf16_length(self.line_starts[line], clamped) }
    }

    pub fn range_for(&self, span: &Span) -> LineRange {
        LineRange { start: self.position_at(span.start_offset), end: self.position_at(span.end_offset) }
    }

    fn utf16_length(&self, start_byte: usize, end_byte: usize) -> usize {
        let mut offset = start_byte.min(self.text.len());
        let end = end_byte.min(self.text.len());
        let mut units = 0;
        while offset < end {
            let (scalar, next) = decode_one(&self.text, offset);
            if next > end {
                break;
            }
            offset = next;
            units += utf16_units(scalar);
        }
        units
    }
}
