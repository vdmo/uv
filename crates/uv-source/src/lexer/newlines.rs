use uv_core::source_text::{SourceFile, LF};
use uv_core::span::span_of;
use uv_core::spec_rule;

use super::token::{Token, TokenKind};
use super::ScalarRange;

fn is_punc(tok: &Token, lexeme: &str) -> bool {
    tok.kind == TokenKind::Punctuator && tok.lexeme == lexeme
}

fn is_kw(tok: &Token, lexeme: &str) -> bool {
    tok.kind == TokenKind::Keyword && tok.lexeme == lexeme
}

fn begins_operand(tok: &Token) -> bool {
    match tok.kind {
        TokenKind::Identifier
        | TokenKind::IntLiteral
        | TokenKind::FloatLiteral
        | TokenKind::StringLiteral
        | TokenKind::CharLiteral
        | TokenKind::BoolLiteral
        | TokenKind::NullLiteral => true,
        TokenKind::Punctuator => matches!(tok.lexeme.as_str(), "(" | "[" | "{"),
        TokenKind::Operator => matches!(tok.lexeme.as_str(), "!" | "-" | "&" | "*" | "^"),
        TokenKind::Keyword => matches!(
            tok.lexeme.as_str(),
            "if" | "loop"
                | "unsafe"
                | "comptime"
                | "quote"
                | "move"
                | "transmute"
                | "widen"
                | "parallel"
                | "spawn"
                | "dispatch"
                | "yield"
                | "sync"
                | "race"
                | "all"
        ),
        _ => false,
    }
}

fn is_ambig_op(lexeme: &str) -> bool {
    matches!(lexeme, "+" | "-" | "*" | "&" | "|")
}

fn is_range_cont_op(lexeme: &str) -> bool {
    matches!(lexeme, ".." | "..=")
}

fn is_unary_only(lexeme: &str) -> bool {
    matches!(lexeme, "!" | "~" | "?")
}

struct NewlineContext {
    paren_depth: Vec<i32>,
    bracket_depth: Vec<i32>,
    prev_index: Vec<Option<usize>>,
    next_index: Vec<Option<usize>>,
}

fn build_newline_context(tokens: &[Token]) -> NewlineContext {
    let n = tokens.len();
    let mut paren_depth = vec![0i32; n + 1];
    let mut bracket_depth = vec![0i32; n + 1];
    for (i, tok) in tokens.iter().enumerate() {
        paren_depth[i + 1] = paren_depth[i];
        bracket_depth[i + 1] = bracket_depth[i];
        if tok.kind == TokenKind::Punctuator {
            match tok.lexeme.as_str() {
                "(" => paren_depth[i + 1] += 1,
                ")" => paren_depth[i + 1] -= 1,
                "[" => bracket_depth[i + 1] += 1,
                "]" => bracket_depth[i + 1] -= 1,
                _ => {}
            }
        }
    }
    let mut prev_index = vec![None; n];
    let mut prev_non_newline = None;
    for (i, tok) in tokens.iter().enumerate() {
        prev_index[i] = prev_non_newline;
        if tok.kind != TokenKind::Newline {
            prev_non_newline = Some(i);
        }
    }
    let mut next_index = vec![None; n];
    let mut next_non_newline = None;
    for (i, tok) in tokens.iter().enumerate().rev() {
        next_index[i] = next_non_newline;
        if tok.kind != TokenKind::Newline {
            next_non_newline = Some(i);
        }
    }
    NewlineContext { paren_depth, bracket_depth, prev_index, next_index }
}

fn is_attribute_name_token(token: &Token) -> bool {
    token.kind == TokenKind::Identifier || token.kind == TokenKind::Keyword
}

fn parse_attribute_spec_line_end(tokens: &[Token], mut pos: usize, end: usize) -> Option<usize> {
    if pos >= end || tokens[pos].kind != TokenKind::Operator || tokens[pos].lexeme != "#" {
        return None;
    }
    pos += 1;
    if pos >= end || !is_attribute_name_token(&tokens[pos]) {
        return None;
    }
    pos += 1;
    while pos + 1 < end
        && tokens[pos].kind == TokenKind::Operator
        && tokens[pos].lexeme == "::"
        && is_attribute_name_token(&tokens[pos + 1])
    {
        pos += 2;
    }
    if pos < end && is_punc(&tokens[pos], "(") {
        let mut depth = 0i32;
        loop {
            if is_punc(&tokens[pos], "(") {
                depth += 1;
            } else if is_punc(&tokens[pos], ")") {
                depth -= 1;
            }
            pos += 1;
            if !(pos < end && depth > 0) {
                break;
            }
        }
        if depth != 0 {
            return None;
        }
    }
    Some(pos)
}

fn line_is_only_attribute_list(tokens: &[Token], line_start: usize, line_end: usize) -> bool {
    let mut pos = line_start;
    while pos < line_end {
        match parse_attribute_spec_line_end(tokens, pos, line_end) {
            Some(next) if next != pos => pos = next,
            _ => return false,
        }
    }
    pos == line_end
}

fn continues_line_impl(tokens: &[Token], i: usize, ctx: &NewlineContext) -> bool {
    if i >= tokens.len() || tokens[i].kind != TokenKind::Newline {
        return false;
    }
    if ctx.paren_depth[i] > 0 || ctx.bracket_depth[i] > 0 {
        return true;
    }
    let prev = ctx.prev_index[i].map(|index| &tokens[index]);
    let next = ctx.next_index[i].map(|index| &tokens[index]);
    if let Some(prev) = prev {
        if is_punc(prev, ",") {
            return true;
        }
        if prev.kind == TokenKind::Operator {
            if is_ambig_op(&prev.lexeme) || is_range_cont_op(&prev.lexeme) {
                if let Some(next) = next {
                    if begins_operand(next) {
                        return true;
                    }
                }
            }
            if !is_unary_only(&prev.lexeme) && !is_range_cont_op(&prev.lexeme) {
                return true;
            }
        }
    }
    if let Some(prev_index) = ctx.prev_index[i] {
        let mut line_start = prev_index;
        while line_start > 0 && tokens[line_start - 1].kind != TokenKind::Newline {
            line_start -= 1;
        }
        if line_is_only_attribute_list(tokens, line_start, prev_index + 1) {
            return true;
        }
    }
    if let (Some(prev), Some(next)) = (prev, next) {
        if is_punc(prev, "}") && is_kw(next, "else") {
            spec_rule!("req.ElseContinuationAcrossNewline");
            return true;
        }
    }
    if let Some(next) = next {
        if matches!(next.lexeme.as_str(), "." | "::" | "~>") {
            return true;
        }
    }
    false
}

fn required_terminator_impl(tokens: &[Token], i: usize, ctx: &NewlineContext) -> bool {
    i < tokens.len() && tokens[i].kind == TokenKind::Newline && !continues_line_impl(tokens, i, ctx)
}

pub fn lex_newlines(source: &SourceFile, suppressed: &[ScalarRange]) -> Vec<Token> {
    spec_rule!("Lex-Newline");
    let scalars = &source.scalars;
    let offsets = &source.offsets;
    let mut out = Vec::new();
    for (i, &c) in scalars.iter().enumerate() {
        if c != LF {
            continue;
        }
        if suppressed.iter().any(|range| i >= range.start && i < range.end) {
            continue;
        }
        out.push(Token {
            kind: TokenKind::Newline,
            lexeme: "\n".to_string(),
            span: span_of(source, offsets[i], offsets[i + 1]),
        });
    }
    out
}

/// Drops newline tokens that do not terminate a statement.
pub fn filter_newlines(tokens: &[Token]) -> Vec<Token> {
    let ctx = build_newline_context(tokens);
    tokens
        .iter()
        .enumerate()
        .filter(|(i, tok)| {
            tok.kind != TokenKind::Newline || required_terminator_impl(tokens, *i, &ctx)
        })
        .map(|(_, tok)| tok.clone())
        .collect()
}

pub fn required_terminator(tokens: &[Token], index: usize) -> bool {
    required_terminator_impl(tokens, index, &build_newline_context(tokens))
}

pub fn continues_line(tokens: &[Token], index: usize) -> bool {
    continues_line_impl(tokens, index, &build_newline_context(tokens))
}
