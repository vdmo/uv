//! Angle-bracket scanning and `>>` splitting for generic argument lists.

use std::rc::Rc;


use super::state::Parser;
use crate::lexer::{Token, TokenKind};

/// Marks the current `>>` token so that it reads as two `>` tokens.
pub fn split_shift_r(parser: &Parser) -> Parser {
    if !parser.is_op(">>") {
        return parser.clone();
    }
    let existing: &[usize] = parser.split_shift_right_indices.as_ref().map_or(&[], |s| s.as_slice());
    let mut split_count_before = 0usize;
    for &split_index in existing {
        if parser.index <= split_index + split_count_before + 1 {
            break;
        }
        split_count_before += 1;
    }
    let underlying_index = parser.index - split_count_before;
    let mut split_indices = existing.to_vec();
    if let Err(insert_at) = split_indices.binary_search(&underlying_index) {
        split_indices.insert(insert_at, underlying_index);
    }
    let mut out = parser.clone();
    out.split_shift_right_indices = Some(Rc::new(split_indices));
    out
}

pub fn angle_delta(tok: &Token) -> i32 {
    if tok.kind != TokenKind::Operator {
        return 0;
    }
    match tok.lexeme.as_str() {
        "<" => 1,
        ">" => -1,
        ">>" => -2,
        _ => 0,
    }
}

/// Advances past a balanced angle-bracket group; returns `start` when it never closes.
pub fn angle_scan(start: &Parser, parser: &Parser, depth: i32) -> Parser {
    let mut current = parser.clone();
    let mut d = depth;
    loop {
        if current.at_eof() {
            return start.clone();
        }
        d += angle_delta(&current.tok());
        current.advance();
        if d == 0 {
            return current;
        }
    }
}

pub fn skip_angles(parser: &Parser) -> Parser {
    angle_scan(parser, parser, 0)
}
