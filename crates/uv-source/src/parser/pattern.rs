//! Pattern grammar.

use std::sync::Arc;

use uv_core::span::Span;
use uv_core::spec_rule;

use super::consume::{emit_trailing_comma_err, match_punct, trailing_comma_allowed, TokenMatch};
use super::expr::{parse_expr, skip_newlines};
use super::paths::{parse_ident, parse_local_ident, parse_qualified_head, parse_type_path};
use super::recovery::{
    emit_generic_parse_syntax_err, emit_parse_syntax_err, emit_splice_outside_quote_err, sync_stmt,
};
use super::state::{span_between, Parsed, Parser};
use super::types::parse_type;
use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub(crate) fn is_literal_token(tok: &Token) -> bool {
    matches!(
        tok.kind,
        TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::StringLiteral
            | TokenKind::CharLiteral
            | TokenKind::BoolLiteral
            | TokenKind::NullLiteral
    )
}

fn is_identifier_slot_token(tok: &Token) -> bool {
    tok.kind == TokenKind::Identifier || tok.kind == TokenKind::Keyword
}

pub(crate) fn make_pattern(span: Span, node: impl Into<PatternNode>) -> PatternPtr {
    Some(Arc::new(Pattern { span, node: node.into() }))
}

fn wildcard(span: Span) -> PatternPtr {
    make_pattern(span, WildcardPattern {})
}

const PAREN_END: [TokenMatch; 1] = [match_punct(")")];
const BRACE_END: [TokenMatch; 1] = [match_punct("}")];

fn try_parse_splice_pattern(parser: &Parser) -> Option<Parsed<PatternPtr>> {
    if !parser.is_op("$") {
        return None;
    }
    let mut after_dollar = parser.advance_or_eof();
    if !after_dollar.is_punct("(") {
        return None;
    }
    if !parser.quote_mode {
        let span = span_between(parser, &after_dollar);
        emit_splice_outside_quote_err(&mut after_dollar, span);
        sync_stmt(&mut after_dollar);
        let pattern = wildcard(span_between(parser, &after_dollar));
        return Some((after_dollar, pattern));
    }
    let mut inner = after_dollar.advance_or_eof();
    inner.quote_mode = false;
    let (mut after_expr, expr) = parse_expr(inner);
    after_expr.quote_mode = parser.quote_mode;
    if !after_expr.is_punct(")") {
        let span = after_expr.tok_span();
        emit_parse_syntax_err(&mut after_expr, span);
        let pattern = wildcard(span_between(parser, &after_expr));
        return Some((after_expr, pattern));
    }
    let after_r = after_expr.advance_or_eof();
    let span = span_between(parser, &after_r);
    let pattern = make_pattern(span.clone(), SpliceExprNode { expr, span });
    Some((after_r, pattern))
}

fn is_pattern_start(tok: &Token) -> bool {
    if is_literal_token(tok) || is_identifier_slot_token(tok) {
        return true;
    }
    match tok.kind {
        TokenKind::Punctuator => tok.lexeme == "(",
        TokenKind::Operator => tok.lexeme == "@",
        _ => false,
    }
}

fn parse_pattern_list_tail(mut parser: Parser, mut xs: Vec<PatternPtr>) -> Parsed<Vec<PatternPtr>> {
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct(")") {
            spec_rule!("Parse-PatternListTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_punct(")") {
                if trailing_comma_allowed(&parser, &PAREN_END) {
                    spec_rule!("Parse-PatternListTail-TrailingComma");
                }
                emit_trailing_comma_err(&mut parser, &PAREN_END);
                after.diags = parser.diags;
                return (after, xs);
            }
            spec_rule!("Parse-PatternListTail-Comma");
            let (next, elem) = parse_pattern(after);
            xs.push(elem);
            parser = next;
            continue;
        }
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        return (parser, xs);
    }
}

fn prepend(first: PatternPtr, tail: Vec<PatternPtr>) -> Vec<PatternPtr> {
    let mut elems = Vec::with_capacity(1 + tail.len());
    elems.push(first);
    elems.extend(tail);
    elems
}

fn parse_tuple_pattern_elems(mut parser: Parser) -> Parsed<Vec<PatternPtr>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-TuplePatternElems-Empty");
        return (parser, Vec::new());
    }
    let (mut after_first, first) = parse_pattern(parser);
    skip_newlines(&mut after_first);
    if after_first.is_punct(";") {
        spec_rule!("Parse-TuplePatternElems-Single");
        return (after_first.advance_or_eof(), vec![first]);
    }
    if after_first.is_punct(",") {
        let mut after = after_first.advance_or_eof();
        skip_newlines(&mut after);
        if after.is_punct(")") {
            // The reference reports this on a parser copy that it then discards.
            let mut discarded = after_first.clone();
            let span = discarded.tok_span();
            emit_parse_syntax_err(&mut discarded, span);
            return (after, vec![first]);
        }
        spec_rule!("Parse-TuplePatternElems-Many");
        let (second_parser, second) = parse_pattern(after);
        let (parser, tail) = parse_pattern_list_tail(second_parser, vec![second]);
        return (parser, prepend(first, tail));
    }
    let span = after_first.tok_span();
    emit_parse_syntax_err(&mut after_first, span);
    (after_first, vec![first])
}

fn parse_enum_payload_pattern_elems(mut parser: Parser) -> Parsed<Vec<PatternPtr>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-EnumPayloadPatternElems-Empty");
        return (parser, Vec::new());
    }
    let (mut after_first, first) = parse_pattern(parser);
    skip_newlines(&mut after_first);
    if after_first.is_punct(",") {
        let mut after = after_first.advance_or_eof();
        skip_newlines(&mut after);
        if after.is_punct(")") {
            if trailing_comma_allowed(&after_first, &PAREN_END) {
                spec_rule!("Parse-EnumPayloadPatternElems-TrailingComma");
            }
            emit_trailing_comma_err(&mut after_first, &PAREN_END);
            after.diags = after_first.diags;
            return (after, vec![first]);
        }
        spec_rule!("Parse-EnumPayloadPatternElems-Many");
        let (second_parser, second) = parse_pattern(after);
        let (parser, tail) = parse_pattern_list_tail(second_parser, vec![second]);
        return (parser, prepend(first, tail));
    }
    spec_rule!("Parse-EnumPayloadPatternElems-One");
    (after_first, vec![first])
}

fn parse_field_pattern(parser: Parser) -> Parsed<FieldPattern> {
    spec_rule!("Parse-FieldPattern");
    let start = parser.clone();
    let (name_parser, name) = parse_ident(parser);
    let (parser, pattern_opt) = if name_parser.is_punct(":") {
        spec_rule!("Parse-FieldPatternTailOpt-Yes");
        parse_pattern(name_parser.advance_or_eof())
    } else {
        spec_rule!("Parse-FieldPatternTailOpt-None");
        (name_parser, None)
    };
    let span = span_between(&start, &parser);
    (parser, FieldPattern { name, pattern_opt, span })
}

pub(crate) fn parse_field_pattern_list(mut parser: Parser) -> Parsed<Vec<FieldPattern>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-FieldPatternList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-FieldPatternList-Cons");
    let (mut parser, first) = parse_field_pattern(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct("}") {
            spec_rule!("Parse-FieldPatternTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_punct("}") {
                if trailing_comma_allowed(&parser, &BRACE_END) {
                    spec_rule!("Parse-FieldPatternTail-TrailingComma");
                }
                emit_trailing_comma_err(&mut parser, &BRACE_END);
                after.diags = parser.diags;
                return (after, xs);
            }
            spec_rule!("Parse-FieldPatternTail-Comma");
            let (next, field) = parse_field_pattern(after);
            xs.push(field);
            parser = next;
            continue;
        }
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        return (parser, xs);
    }
}

/// `{ fields }` after the opening brace was seen at `parser`. `None` on a missing `}`.
fn parse_braced_field_patterns(parser: Parser) -> Parsed<Option<Vec<FieldPattern>>> {
    let (mut fields_parser, fields) = parse_field_pattern_list(parser.advance_or_eof());
    if !fields_parser.is_punct("}") {
        let span = fields_parser.tok_span();
        emit_parse_syntax_err(&mut fields_parser, span);
        return (fields_parser, None);
    }
    (fields_parser.advance_or_eof(), Some(fields))
}

fn parse_enum_pattern_payload_opt(parser: Parser) -> Parsed<Option<EnumPayloadPattern>> {
    if parser.is_punct("(") {
        spec_rule!("Parse-EnumPatternPayloadOpt-Tuple");
        let (mut elems_parser, elements) = parse_enum_payload_pattern_elems(parser.advance_or_eof());
        if !elems_parser.is_punct(")") {
            let span = elems_parser.tok_span();
            emit_parse_syntax_err(&mut elems_parser, span);
            return (elems_parser, None);
        }
        return (elems_parser.advance_or_eof(), Some(TuplePayloadPattern { elements }.into()));
    }
    if parser.is_punct("{") {
        spec_rule!("Parse-EnumPatternPayloadOpt-Record");
        let (parser, fields) = parse_braced_field_patterns(parser);
        return (parser, fields.map(|fields| RecordPayloadPattern { fields }.into()));
    }
    spec_rule!("Parse-EnumPatternPayloadOpt-None");
    (parser, None)
}

fn parse_modal_pattern_payload_opt(parser: Parser) -> Parsed<Option<ModalRecordPayload>> {
    if !parser.is_punct("{") {
        spec_rule!("Parse-ModalPatternPayloadOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-ModalPatternPayloadOpt-Record");
    let (parser, fields) = parse_braced_field_patterns(parser);
    (parser, fields.map(|fields| ModalRecordPayload { fields }))
}

fn parse_pattern_atom(parser: Parser) -> Parsed<PatternPtr> {
    if let Some(splice) = try_parse_splice_pattern(&parser) {
        return splice;
    }
    if parser.quote_mode && parser.is_op("$") {
        let (name_parser, name) = parse_local_ident(parser.clone());
        if name_parser.is_punct(":") {
            spec_rule!("Parse-Pattern-Typed");
            let (ty_parser, ty) = parse_type(name_parser.advance_or_eof());
            let pat = TypedPattern { name: name.name, r#type: ty, name_splice_opt: name.splice_opt };
            let pattern = make_pattern(span_between(&parser, &ty_parser), pat);
            return (ty_parser, pattern);
        }
        spec_rule!("Parse-Pattern-Identifier");
        let pat = IdentifierPattern { name: name.name, name_splice_opt: name.splice_opt };
        let pattern = make_pattern(span_between(&parser, &name_parser), pat);
        return (name_parser, pattern);
    }
    let tok = parser.tok();
    if is_literal_token(&tok) {
        spec_rule!("Parse-Pattern-Literal");
        let pattern = make_pattern(tok.span.clone(), LiteralPattern { literal: (*tok).clone() });
        return (parser.advance_or_eof(), pattern);
    }
    let slot = is_identifier_slot_token(&tok);
    let after_tok = parser.advance_or_eof();
    if slot && after_tok.is_punct(":") {
        spec_rule!("Parse-Pattern-Typed");
        let (name_parser, name) = parse_ident(parser.clone());
        let (ty_parser, ty) = parse_type(name_parser.advance_or_eof());
        let pat = TypedPattern { name, r#type: ty, name_splice_opt: None };
        let pattern = make_pattern(span_between(&parser, &ty_parser), pat);
        return (ty_parser, pattern);
    }
    if tok.kind == TokenKind::Identifier && tok.lexeme == "_" {
        spec_rule!("Parse-Pattern-Wildcard");
        return (after_tok, wildcard(tok.span.clone()));
    }
    if slot && after_tok.is_op("::") {
        spec_rule!("Parse-Pattern-Enum");
        let (head_parser, head) = parse_qualified_head(parser.clone());
        let (payload_parser, payload_opt) = parse_enum_pattern_payload_opt(head_parser);
        let pat = EnumPattern { path: head.module_path, name: head.name, payload_opt };
        let pattern = make_pattern(span_between(&parser, &payload_parser), pat);
        return (payload_parser, pattern);
    }
    if parser.is_punct("(") {
        spec_rule!("Parse-Pattern-Tuple");
        let (mut elems_parser, elements) = parse_tuple_pattern_elems(after_tok);
        if !elems_parser.is_punct(")") {
            let span = elems_parser.tok_span();
            emit_parse_syntax_err(&mut elems_parser, span);
            let pattern = wildcard(span_between(&parser, &elems_parser));
            return (elems_parser, pattern);
        }
        let after = elems_parser.advance_or_eof();
        let pattern = make_pattern(span_between(&parser, &after), TuplePattern { elements });
        return (after, pattern);
    }
    if slot {
        let (probe, _) = parse_type_path(parser.clone_without_diags());
        if probe.is_punct("{") {
            spec_rule!("Parse-Pattern-Record");
            let (path_parser, path) = parse_type_path(parser.clone());
            let (mut fields_parser, fields) = parse_field_pattern_list(path_parser.advance_or_eof());
            if !fields_parser.is_punct("}") {
                let span = fields_parser.tok_span();
                emit_parse_syntax_err(&mut fields_parser, span);
                let pattern = wildcard(span_between(&parser, &fields_parser));
                return (fields_parser, pattern);
            }
            let done = fields_parser.advance_or_eof();
            let pattern = make_pattern(span_between(&parser, &done), RecordPattern { path, fields });
            return (done, pattern);
        }
    }
    if parser.is_op("@") {
        spec_rule!("Parse-Pattern-Modal");
        let (name_parser, state) = parse_ident(after_tok);
        let (payload_parser, fields_opt) = parse_modal_pattern_payload_opt(name_parser);
        let pattern =
            make_pattern(span_between(&parser, &payload_parser), ModalPattern { state, fields_opt });
        return (payload_parser, pattern);
    }
    if slot {
        spec_rule!("Parse-Pattern-Identifier");
        let (name_parser, name) = parse_ident(parser.clone());
        let pat = IdentifierPattern { name, name_splice_opt: None };
        let pattern = make_pattern(span_between(&parser, &name_parser), pat);
        return (name_parser, pattern);
    }
    let mut parser = parser;
    let span = parser.tok_span();
    emit_parse_syntax_err(&mut parser, span.clone());
    (parser, wildcard(span))
}

fn parse_pattern_range(parser: Parser) -> Parsed<PatternPtr> {
    let (lhs_parser, lhs) = parse_pattern_atom(parser.clone());
    if lhs_parser.is_op("..") || lhs_parser.is_op("..=") {
        spec_rule!("Parse-Pattern-Range");
        let kind = if lhs_parser.is_op("..=") { RangeKind::Inclusive } else { RangeKind::Exclusive };
        let (rhs_parser, rhs) = parse_pattern_atom(lhs_parser.advance_or_eof());
        let pattern = make_pattern(
            span_between(&parser, &rhs_parser),
            RangePattern { kind, lo: lhs, hi: rhs },
        );
        return (rhs_parser, pattern);
    }
    spec_rule!("Parse-Pattern-Range-None");
    (lhs_parser, lhs)
}

pub fn parse_pattern(parser: Parser) -> Parsed<PatternPtr> {
    if let Some(splice) = try_parse_splice_pattern(&parser) {
        spec_rule!("Parse-Pattern");
        return splice;
    }
    if parser.quote_mode && parser.is_op("$") {
        spec_rule!("Parse-Pattern");
        return parse_pattern_range(parser);
    }
    if !is_pattern_start(&parser.tok()) {
        spec_rule!("Parse-Pattern-Err");
        spec_rule!("rule.17.Parse-Pattern-Err");
        let mut parser = parser;
        let span = parser.tok_span();
        emit_generic_parse_syntax_err(&mut parser, span.clone());
        return (parser, wildcard(span));
    }
    spec_rule!("Parse-Pattern");
    parse_pattern_range(parser)
}
