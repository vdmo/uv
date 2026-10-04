//! Identifiers, paths, visibility and small optional markers.

use std::sync::Arc;

use uv_core::span::Span;
use uv_core::{spec_rule, spec_rule_at};

use super::recovery::{emit_generic_parse_syntax_err, emit_parse_syntax_err, emit_splice_outside_quote_err};
use super::state::{span_between, Parsed, Parser};
use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub(crate) fn make_expr(span: Span, node: impl Into<ExprNode>) -> ExprPtr {
    Some(Arc::new(Expr { span, node: node.into() }))
}

fn is_identifier_slot_token(tok: &Token) -> bool {
    tok.kind == TokenKind::Identifier || tok.kind == TokenKind::Keyword
}

fn record_restricted_splice_ident_if_present(parser: &Parser) {
    if !parser.quote_mode || !parser.is_op("$") {
        return;
    }
    let after_dollar = parser.advance_or_eof();
    if !is_identifier_slot_token(&after_dollar.tok()) {
        return;
    }
    let after_ident = after_dollar.advance_or_eof();
    spec_rule_at!(
        "requirement.22.SpliceIdentifierPositionRestrictions",
        span_between(parser, &after_ident)
    );
}

fn emit_reserved_keyword_identifier_err(parser: &mut Parser, span: Span) {
    parser.emit("E-CNF-0401", span);
}

struct QualifiedSegment {
    name: Identifier,
    span: Span,
    is_keyword: bool,
}

fn parse_qualified_segment(mut parser: Parser) -> Parsed<QualifiedSegment> {
    let tok = parser.tok();
    if is_identifier_slot_token(&tok) {
        let segment = QualifiedSegment {
            name: tok.lexeme.clone(),
            span: tok.span.clone(),
            is_keyword: tok.kind == TokenKind::Keyword,
        };
        parser.advance();
        return (parser, segment);
    }
    record_restricted_splice_ident_if_present(&parser);
    spec_rule!("Parse-Ident-Err");
    let span = parser.tok_span();
    emit_generic_parse_syntax_err(&mut parser, span.clone());
    (parser, QualifiedSegment { name: "_".to_string(), span, is_keyword: false })
}

pub fn parse_ident(mut parser: Parser) -> Parsed<Identifier> {
    let tok = parser.tok();
    if tok.kind == TokenKind::Identifier {
        spec_rule!("Parse-Ident");
        parser.advance();
        return (parser, tok.lexeme.clone());
    }
    if tok.kind == TokenKind::Keyword {
        emit_reserved_keyword_identifier_err(&mut parser, tok.span.clone());
        parser.advance();
        return (parser, tok.lexeme.clone());
    }
    record_restricted_splice_ident_if_present(&parser);
    spec_rule!("Parse-Ident-Err");
    let span = parser.tok_span();
    emit_generic_parse_syntax_err(&mut parser, span);
    (parser, "_".to_string())
}

pub struct LocalIdent {
    pub name: Identifier,
    pub splice_opt: Option<SpliceIdentNode>,
}

/// An identifier in a binding position; inside `quote` it may be a `$name` splice.
pub fn parse_local_ident(parser: Parser) -> Parsed<LocalIdent> {
    let none = |name: &str| LocalIdent { name: name.to_string(), splice_opt: None };
    if parser.is_op("$") {
        let mut after_dollar = parser.clone();
        after_dollar.advance();
        let tok = after_dollar.tok();
        if !parser.quote_mode {
            if is_identifier_slot_token(&tok) {
                let mut after_ident = after_dollar;
                if tok.kind == TokenKind::Keyword {
                    emit_reserved_keyword_identifier_err(&mut after_ident, tok.span.clone());
                }
                after_ident.advance();
                let span = span_between(&parser, &after_ident);
                emit_splice_outside_quote_err(&mut after_ident, span);
                return (after_ident, none("_"));
            }
        } else {
            if !is_identifier_slot_token(&tok) {
                let span = after_dollar.tok_span();
                emit_parse_syntax_err(&mut after_dollar, span);
                return (after_dollar, none("_"));
            }
            let mut after_ident = after_dollar;
            if tok.kind == TokenKind::Keyword {
                emit_reserved_keyword_identifier_err(&mut after_ident, tok.span.clone());
            }
            after_ident.advance();
            let ident = IdentifierExpr { name: tok.lexeme.clone(), ..Default::default() };
            let splice = SpliceIdentNode {
                name_expr: make_expr(tok.span.clone(), ident),
                span: span_between(&parser, &after_ident),
            };
            return (after_ident, LocalIdent { name: "_".to_string(), splice_opt: Some(splice) });
        }
    }
    let (parser, name) = parse_ident(parser);
    (parser, LocalIdent { name, splice_opt: None })
}

fn parse_path_tail(mut parser: Parser, mut xs: Path, end_rule: &str, cons_rule: &str) -> Parsed<Path> {
    loop {
        if !parser.is_op("::") {
            spec_rule!(end_rule);
            return (parser, xs);
        }
        spec_rule!(cons_rule);
        parser.advance();
        let (next, id) = parse_ident(parser);
        xs.push(id);
        parser = next;
    }
}

pub fn parse_module_path(parser: Parser) -> Parsed<Path> {
    spec_rule!("Parse-ModulePath");
    let (parser, head) = parse_ident(parser);
    parse_path_tail(parser, vec![head], "Parse-ModulePathTail-End", "Parse-ModulePathTail-Cons")
}

pub fn parse_type_path(parser: Parser) -> Parsed<Path> {
    spec_rule!("Parse-TypePath");
    let (parser, head) = parse_ident(parser);
    parse_path_tail(parser, vec![head], "Parse-TypePathTail-End", "Parse-TypePathTail-Cons")
}

pub fn parse_class_path(parser: Parser) -> Parsed<Path> {
    spec_rule!("Parse-ClassPath");
    parse_type_path(parser)
}

pub struct QualifiedHead {
    pub module_path: Path,
    pub name: Identifier,
}

/// `a::b::name`: at least two segments; keywords are reported except in `string::from`.
pub fn parse_qualified_head(parser: Parser) -> Parsed<QualifiedHead> {
    spec_rule!("Parse-QualifiedHead");
    let bad = || QualifiedHead { module_path: Vec::new(), name: "_".to_string() };
    let (head_parser, head) = parse_qualified_segment(parser.clone());
    if !head_parser.is_op("::") {
        let mut parser = parser;
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        return (parser, bad());
    }
    let mut segments = vec![head];
    let mut cur = head_parser;
    while cur.is_op("::") {
        cur.advance();
        let (next, segment) = parse_qualified_segment(cur);
        segments.push(segment);
        cur = next;
    }
    for (i, segment) in segments.iter().enumerate() {
        if !segment.is_keyword {
            continue;
        }
        let spec_defined = segments.len() == 2
            && i == 1
            && segments[0].name == "string"
            && segments[1].name == "from";
        if !spec_defined {
            emit_reserved_keyword_identifier_err(&mut cur, segment.span.clone());
        }
    }
    let mut full: Path = segments.into_iter().map(|segment| segment.name).collect();
    let name = full.pop().unwrap_or_else(|| "_".to_string());
    (cur, QualifiedHead { module_path: full, name })
}

pub fn parse_vis(mut parser: Parser) -> Parsed<Visibility> {
    let tok = parser.tok();
    if tok.kind == TokenKind::Keyword {
        let vis = match tok.lexeme.as_str() {
            "public" => Some(Visibility::Public),
            "internal" => Some(Visibility::Internal),
            "private" => Some(Visibility::Private),
            _ => None,
        };
        if let Some(vis) = vis {
            spec_rule!("Parse-Vis-Opt");
            parser.advance();
            return (parser, vis);
        }
    }
    spec_rule!("Parse-Vis-Default");
    (parser, Visibility::Internal)
}

pub fn parse_key_boundary_opt(mut parser: Parser) -> Parsed<bool> {
    if parser.is_op("#") {
        spec_rule!("Parse-KeyBoundaryOpt-Yes");
        parser.advance();
        return (parser, true);
    }
    spec_rule!("Parse-KeyBoundaryOpt-No");
    (parser, false)
}

pub fn parse_modal_opt(mut parser: Parser) -> Parsed<bool> {
    if parser.is_kw("modal") {
        spec_rule!("Parse-ModalOpt-Yes");
        parser.advance();
        return (parser, true);
    }
    spec_rule!("Parse-ModalOpt-No");
    (parser, false)
}

pub fn parse_alias_opt(mut parser: Parser) -> Parsed<Option<Identifier>> {
    if parser.is_kw("as") {
        spec_rule!("Parse-AliasOpt-Yes");
        parser.advance();
        let (parser, id) = parse_ident(parser);
        return (parser, Some(id));
    }
    spec_rule!("Parse-AliasOpt-None");
    (parser, None)
}
