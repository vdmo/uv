//! Expression grammar: precedence climbing, postfix chains and primary expressions.

mod forms;

use uv_core::numeric_literals::{parse_int_core, strip_int_suffix};
use uv_core::span::Span;
use uv_core::spec_rule;

pub(crate) use self::forms::*;
pub use self::forms::make_modal_ref;
use super::angle::{skip_angles, split_shift_r};
use super::consume::{emit_trailing_comma_err, match_operator, match_punct, trailing_comma_allowed, TokenMatch};
use super::item::attributes::parse_attribute_list_opt;
pub(crate) use super::paths::make_expr;
use super::paths::parse_ident;
pub(crate) use super::pattern::is_literal_token;
use super::recovery::{emit_parse_syntax_err, emit_splice_outside_quote_err, sync_stmt};
use super::state::{span_between, span_from, Parsed, Parser};
use super::stmt::parse_block;
use super::types::parse_type;
use crate::ast::*;
use crate::lexer::keyword_policy::ctx;
use crate::lexer::{Token, TokenKind};

pub(crate) use self::forms::try_parse_pattern_in;

pub(crate) fn skip_newlines(parser: &mut Parser) {
    while parser.is_newline() {
        parser.advance();
    }
}

/// Span of a non-null expression (the default span for a null one).
pub(crate) fn espan(expr: &ExprPtr) -> Span {
    expr.as_ref().map(|e| e.span.clone()).unwrap_or_default()
}

pub(crate) fn span_cover(start: &Span, end: &Span) -> Span {
    span_from(start, end)
}

/// Reports a syntax error at the current token, resynchronizes to the end of the
/// statement and yields an error expression spanning `start..here`.
pub(crate) fn expr_error(mut parser: Parser, start: &Parser) -> Parsed<ExprPtr> {
    let span = parser.tok_span();
    emit_parse_syntax_err(&mut parser, span);
    sync_stmt(&mut parser);
    let expr = make_expr(span_between(start, &parser), ErrorExpr {});
    (parser, expr)
}

pub(crate) fn is_expr_start(tok: &Token) -> bool {
    match tok.kind {
        TokenKind::Identifier => true,
        _ if is_literal_token(tok) => true,
        TokenKind::Punctuator => matches!(tok.lexeme.as_str(), "(" | "[" | "{"),
        TokenKind::Operator => matches!(tok.lexeme.as_str(), "!" | "-" | "&" | "*" | "^" | "@" | "|"),
        TokenKind::Keyword => matches!(
            tok.lexeme.as_str(),
            "if" | "loop"
                | "unsafe"
                | "move"
                | "transmute"
                | "widen"
                | "comptime"
                | "quote"
                | "sizeof"
                | "alignof"
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

fn is_postfix_start(tok: &Token) -> bool {
    match tok.kind {
        TokenKind::Punctuator => matches!(tok.lexeme.as_str(), "." | "[" | "("),
        TokenKind::Operator => matches!(tok.lexeme.as_str(), "~>" | "?"),
        _ => false,
    }
}

pub(crate) fn is_place(expr: &ExprPtr) -> bool {
    let Some(expr) = expr else {
        return false;
    };
    match &expr.node {
        ExprNode::IdentifierExpr(_)
        | ExprNode::FieldAccessExpr(_)
        | ExprNode::TupleAccessExpr(_)
        | ExprNode::IndexAccessExpr(_) => true,
        ExprNode::AttributedExpr(attr) => is_place(&attr.expr),
        ExprNode::DerefExpr(deref) => is_place(&deref.value),
        _ => false,
    }
}

fn parse_expr_with_leading_attrs(parser: Parser, allow_brace: bool) -> Parsed<ExprPtr> {
    let (next, attrs) = parse_attribute_list_opt(parser.clone());
    let (expr_parser, expr) = parse_range(next, allow_brace, true);
    match attrs {
        Some(attrs) => {
            let span = span_between(&parser, &expr_parser);
            let attached = attach_expr_attrs(&expr, attrs, &span);
            (expr_parser, attached)
        }
        None => (expr_parser, expr),
    }
}

pub fn parse_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Expr");
    parse_expr_with_leading_attrs(parser, true)
}

pub(crate) fn parse_expr_no_brace(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Expr-NoBrace");
    parse_expr_with_leading_attrs(parser, false)
}

pub fn parse_predicate_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("ParsePredicateExpr");
    spec_rule!("Parse-Predicate-Expr");
    parse_expr_with_leading_attrs(parser, true)
}

pub fn parse_expr_opt(parser: Parser) -> Parsed<ExprPtr> {
    let tok = parser.tok();
    let none = tok.kind == TokenKind::Newline
        || (tok.kind == TokenKind::Punctuator && (tok.lexeme == ";" || tok.lexeme == "}"))
        || parser.at_eof();
    if none {
        spec_rule!("Parse-ExprOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-ExprOpt-Yes");
    parse_expr(parser)
}

fn range_expr(kind: RangeKind, lhs: ExprPtr, rhs: ExprPtr, start: &Parser, end: &Parser) -> ExprPtr {
    make_expr(span_between(start, end), RangeExpr { kind, lhs, rhs })
}

pub(crate) fn parse_range(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("ParseRangeFamily");
    let start = parser.clone();
    if parser.is_op("..=") {
        spec_rule!("Parse-Range-ToInc");
        let (rhs_parser, rhs) = parse_logical_or(parser.advance_or_eof(), allow_brace, allow_bracket);
        let expr = range_expr(RangeKind::ToInclusive, None, rhs, &start, &rhs_parser);
        return (rhs_parser, expr);
    }
    if parser.is_op("..") {
        let next = parser.advance_or_eof();
        if !is_expr_start(&next.tok()) {
            spec_rule!("Parse-Range-Full");
            let expr = range_expr(RangeKind::Full, None, None, &start, &next);
            return (next, expr);
        }
        spec_rule!("Parse-Range-To");
        let (rhs_parser, rhs) = parse_logical_or(next, allow_brace, allow_bracket);
        let expr = range_expr(RangeKind::To, None, rhs, &start, &rhs_parser);
        return (rhs_parser, expr);
    }
    spec_rule!("Parse-Range-Lhs");
    let (lhs_parser, lhs) = parse_logical_or(parser, allow_brace, allow_bracket);
    if !(lhs_parser.is_op("..") || lhs_parser.is_op("..=")) {
        spec_rule!("Parse-RangeTail-None");
        return (lhs_parser, lhs);
    }
    if lhs_parser.is_op("..") {
        let after = lhs_parser.advance_or_eof();
        if !is_expr_start(&after.tok()) {
            spec_rule!("Parse-RangeTail-From");
            let expr = range_expr(RangeKind::From, lhs, None, &start, &after);
            return (after, expr);
        }
        spec_rule!("Parse-RangeTail-Exclusive");
        let (rhs_parser, rhs) = parse_logical_or(after, allow_brace, allow_bracket);
        let expr = range_expr(RangeKind::Exclusive, lhs, rhs, &start, &rhs_parser);
        return (rhs_parser, expr);
    }
    spec_rule!("Parse-RangeTail-Inclusive");
    let (rhs_parser, rhs) = parse_logical_or(lhs_parser.advance_or_eof(), allow_brace, allow_bracket);
    let expr = range_expr(RangeKind::Inclusive, lhs, rhs, &start, &rhs_parser);
    (rhs_parser, expr)
}

/// Binding strength of a binary operator; 0 when the lexeme is not one.
fn binary_precedence(op: &str) -> u8 {
    match op {
        "||" => 1,
        "&&" => 2,
        "==" | "!=" | "<" | "<:" | "<=" | ">" | ">=" => 3,
        "|" => 4,
        "^" => 5,
        "&" => 6,
        "<<" | ">>" => 7,
        "+" | "-" => 8,
        "*" | "/" | "%" => 9,
        _ => 0,
    }
}

fn binary(op: &str, lhs: ExprPtr, rhs: ExprPtr) -> ExprPtr {
    let span = span_cover(&espan(&lhs), &espan(&rhs));
    make_expr(span, BinaryExpr { op: op.to_string(), lhs, rhs })
}

fn parse_power(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("ParsePowerFamily");
    spec_rule!("Parse-Power");
    let (lhs_parser, lhs) = parse_cast(parser, allow_brace, allow_bracket);
    if !lhs_parser.is_op("**") {
        spec_rule!("Parse-PowerTail-None");
        return (lhs_parser, lhs);
    }
    spec_rule!("Parse-PowerTail-Cons");
    let (rhs_parser, rhs) = parse_power(lhs_parser.advance_or_eof(), allow_brace, allow_bracket);
    (rhs_parser, binary("**", lhs, rhs))
}

fn parse_binary_expr(parser: Parser, min_prec: u8, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("ParseLeftChainFamily");
    let (mut parser, mut expr) = parse_power(parser, allow_brace, allow_bracket);
    loop {
        let tok = parser.tok();
        if tok.kind != TokenKind::Operator {
            break;
        }
        let prec = binary_precedence(&tok.lexeme);
        if prec == 0 || prec < min_prec {
            break;
        }
        spec_rule!("Parse-LeftChain-Cons");
        let (rhs_parser, rhs) =
            parse_binary_expr(parser.advance_or_eof(), prec + 1, allow_brace, allow_bracket);
        expr = binary(&tok.lexeme, expr, rhs);
        parser = rhs_parser;
    }
    spec_rule!("Parse-LeftChain-Stop");
    (parser, expr)
}

pub(crate) fn parse_logical_or(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    parse_binary_expr(parser, 1, allow_brace, allow_bracket)
}

fn parse_cast(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Cast");
    let (lhs_parser, lhs) = parse_unary(parser, allow_brace, allow_bracket);
    if !lhs_parser.is_kw("as") {
        spec_rule!("Parse-CastTail-None");
        return (lhs_parser, lhs);
    }
    spec_rule!("Parse-CastTail-As");
    let (ty_parser, ty) = parse_type(lhs_parser.advance_or_eof());
    let ty_span = ty.as_ref().map(|t| t.span.clone()).unwrap_or_default();
    let span = span_cover(&espan(&lhs), &ty_span);
    (ty_parser, make_expr(span, CastExpr { value: lhs, r#type: ty }))
}

pub(crate) fn parse_place(parser: Parser, allow_brace: bool) -> Parsed<ExprPtr> {
    if parser.is_op("*") {
        spec_rule!("Parse-Place-Deref");
        let (inner_parser, inner) = parse_place(parser.advance_or_eof(), allow_brace);
        let span = span_cover(&parser.tok_span(), &espan(&inner));
        return (inner_parser, make_expr(span, DerefExpr { value: inner }));
    }
    let (mut expr_parser, expr) = parse_postfix(parser.clone(), allow_brace, true);
    if is_place(&expr) {
        spec_rule!("Parse-Place-Postfix");
        return (expr_parser, expr);
    }
    spec_rule!("Parse-Place-Err");
    spec_rule!("rule.16.Parse-Place-Err");
    emit_parse_syntax_err(&mut expr_parser, parser.tok_span());
    sync_stmt(&mut expr_parser);
    let error = make_expr(span_between(&parser, &expr_parser), ErrorExpr {});
    (expr_parser, error)
}

fn effectful_core_rules(rule: &str) {
    spec_rule!(rule);
    spec_rule!(format!("rule.16.{rule}").as_str());
    spec_rule!("grammar.16.EffectfulCoreExpressions");
}

pub(crate) fn parse_unary(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    let head_span = parser.tok_span();
    let cover = |operand: &ExprPtr| span_cover(&head_span, &espan(operand));
    if parser.is_op("!") || parser.is_op("-") {
        spec_rule!("Parse-Unary-Prefix");
        let op = parser.tok().lexeme.clone();
        let (rhs_parser, rhs) = parse_unary(parser.advance_or_eof(), allow_brace, allow_bracket);
        let span = cover(&rhs);
        return (rhs_parser, make_expr(span, UnaryExpr { op, value: rhs }));
    }
    if parser.is_op("*") {
        effectful_core_rules("Parse-Unary-Deref");
        let (rhs_parser, rhs) = parse_unary(parser.advance_or_eof(), allow_brace, allow_bracket);
        spec_rule!("def.16.EffectfulCoreExprAst");
        let span = cover(&rhs);
        return (rhs_parser, make_expr(span, DerefExpr { value: rhs }));
    }
    if parser.is_op("&") {
        effectful_core_rules("Parse-Unary-AddressOf");
        let (place_parser, place) = parse_place(parser.advance_or_eof(), allow_brace);
        spec_rule!("def.16.EffectfulCoreExprAst");
        let span = cover(&place);
        return (place_parser, make_expr(span, AddressOfExpr { place }));
    }
    if parser.is_kw("move") {
        effectful_core_rules("Parse-Unary-Move");
        let (place_parser, place) = parse_place(parser.advance_or_eof(), allow_brace);
        spec_rule!("def.16.EffectfulCoreExprAst");
        let span = cover(&place);
        return (place_parser, make_expr(span, MoveExpr { place }));
    }
    if parser.is_kw("copy") {
        effectful_core_rules("Parse-Unary-Copy");
        let (rhs_parser, rhs) = parse_unary(parser.advance_or_eof(), allow_brace, allow_bracket);
        spec_rule!("def.16.EffectfulCoreExprAst");
        let span = cover(&rhs);
        return (rhs_parser, make_expr(span, CopyExpr { value: rhs }));
    }
    if ctx(&parser.tok(), "new") {
        let next = parser.advance_or_eof();
        if is_expr_start(&next.tok()) {
            effectful_core_rules("Parse-New-Expr");
            let (rhs_parser, rhs) = parse_unary(next, allow_brace, allow_bracket);
            spec_rule!("def.16.EffectfulCoreExprAst");
            let span = cover(&rhs);
            return (rhs_parser, make_expr(span, AllocExpr { region_opt: None, value: rhs }));
        }
    }
    if parser.is_kw("widen") {
        spec_rule!("Parse-Unary-Widen");
        let (rhs_parser, rhs) = parse_unary(parser.advance_or_eof(), allow_brace, allow_bracket);
        let span = cover(&rhs);
        return (rhs_parser, make_expr(span, UnaryExpr { op: "widen".to_string(), value: rhs }));
    }
    spec_rule!("Parse-Unary-Postfix");
    parse_postfix(parser, allow_brace, allow_bracket)
}

// ---- call arguments ---------------------------------------------------------

const PAREN_END: [TokenMatch; 1] = [match_punct(")")];
const GT_END: [TokenMatch; 1] = [match_operator(">")];

fn parse_arg(parser: Parser) -> Parsed<Arg> {
    spec_rule!("Parse-Arg");
    let start = parser.clone();
    let (pass_parser, pass) = if parser.is_kw("move") {
        spec_rule!("Parse-ArgPassOpt-Move");
        (parser.advance_or_eof(), ArgPassKind::Move)
    } else if parser.is_kw("copy") {
        spec_rule!("Parse-ArgPassOpt-Copy");
        (parser.advance_or_eof(), ArgPassKind::Copy)
    } else {
        spec_rule!("Parse-ArgPassOpt-None");
        (parser, ArgPassKind::Ref)
    };
    let (expr_parser, value) = parse_expr(pass_parser);
    let span = span_between(&start, &expr_parser);
    (expr_parser, Arg { pass, value, span })
}

pub(crate) fn parse_arg_list(mut parser: Parser) -> Parsed<Vec<Arg>> {
    spec_rule!("ArgumentListParsingFamily");
    skip_newlines(&mut parser);
    spec_rule!("List-Start");
    if parser.is_punct(")") {
        spec_rule!("List-Done");
        spec_rule!("Parse-ArgList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-ArgList-Cons");
    spec_rule!("List-Cons");
    let (mut parser, first) = parse_arg(parser);
    let mut args = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct(")") {
            spec_rule!("List-Done");
            spec_rule!("Parse-ArgTail-End");
            return (parser, args);
        }
        if !parser.is_punct(",") {
            let span = parser.tok_span();
            emit_parse_syntax_err(&mut parser, span);
            return (parser, args);
        }
        let mut after = parser.advance_or_eof();
        skip_newlines(&mut after);
        if after.is_punct(")") {
            if trailing_comma_allowed(&parser, &PAREN_END) {
                spec_rule!("Parse-ArgTail-TrailingComma");
            }
            emit_trailing_comma_err(&mut parser, &PAREN_END);
            after.diags = parser.diags;
            return (after, args);
        }
        spec_rule!("Parse-ArgTail-Comma");
        spec_rule!("List-Cons");
        let (next, arg) = parse_arg(after);
        args.push(arg);
        parser = next;
    }
}

/// `( args )` with the opening parenthesis at `parser`. `Err` carries the error result.
fn parse_paren_args(parser: Parser, start: &Parser) -> Result<(Parser, Vec<Arg>, Span), Parsed<ExprPtr>> {
    let (args_parser, args) = parse_arg_list(parser.advance_or_eof());
    if !args_parser.is_punct(")") {
        return Err(expr_error(args_parser, start));
    }
    let end_span = args_parser.tok_span();
    Ok((args_parser.advance_or_eof(), args, end_span))
}

fn try_parse_type_args(parser: &Parser) -> Option<Parsed<Vec<TypePtr>>> {
    if !parser.is_op("<") {
        return None;
    }
    let after_lt = parser.advance_or_eof();
    let (first_parser, first) = parse_type(after_lt.clone());
    if first_parser.index == after_lt.index {
        return None;
    }
    let mut targs = vec![first];
    let mut cur = first_parser;
    while cur.is_punct(",") {
        let mut after_comma = cur.advance_or_eof();
        skip_newlines(&mut after_comma);
        if after_comma.is_op(">") {
            if trailing_comma_allowed(&cur, &GT_END) {
                spec_rule!("Parse-TypeListTail-TrailingComma");
            }
            emit_trailing_comma_err(&mut cur, &GT_END);
            after_comma.diags = cur.diags;
            cur = after_comma;
            break;
        }
        let (arg_parser, arg) = parse_type(after_comma.clone());
        if arg_parser.index == after_comma.index {
            return None;
        }
        targs.push(arg);
        cur = arg_parser;
    }
    if cur.is_op(">>") {
        cur = split_shift_r(&cur);
    }
    if !cur.is_op(">") {
        return None;
    }
    cur.advance();
    Some((cur, targs))
}

fn call_type_args_start(parser: &Parser) -> bool {
    if !parser.is_op("<") {
        return false;
    }
    try_parse_type_args(&parser.clone_without_diags()).is_some_and(|(after, _)| after.is_punct("("))
}

fn parse_call_type_args_step(parser: Parser, expr: ExprPtr) -> Parsed<ExprPtr> {
    spec_rule!("Postfix-Call-TypeArgs");
    let Some((targs_parser, type_args)) =
        try_parse_type_args(&parser).filter(|(after, _)| after.is_punct("("))
    else {
        return expr_error(parser.clone(), &parser);
    };
    match parse_paren_args(targs_parser, &parser) {
        Err(error) => error,
        Ok((after, args, end_span)) => {
            let span = span_cover(&espan(&expr), &end_span);
            (after, make_expr(span, CallTypeArgsExpr { callee: expr, type_args, args }))
        }
    }
}

// ---- postfix ----------------------------------------------------------------

fn is_parallel_option_list_start(parser: &Parser) -> bool {
    if !parser.is_punct("[") {
        return false;
    }
    let mut cur = parser.advance_or_eof();
    skip_newlines(&mut cur);
    let name = cur.tok();
    let is_name = matches!(name.kind, TokenKind::Identifier | TokenKind::Keyword)
        && matches!(name.lexeme.as_str(), "cancel" | "name" | "workgroup" | "workgroups");
    is_name && cur.advance_or_eof().is_punct(":")
}

fn postfix_step(parser: Parser, expr: ExprPtr, allow_bracket: bool) -> Parsed<ExprPtr> {
    if call_type_args_start(&parser) {
        return parse_call_type_args_step(parser, expr);
    }
    let base_span = espan(&expr);
    if parser.is_punct(".") {
        let next = parser.advance_or_eof();
        let tok = next.tok();
        if matches!(tok.kind, TokenKind::Identifier | TokenKind::Keyword) {
            spec_rule!("Postfix-Field");
            let field = FieldAccessExpr { base: expr, name: tok.lexeme.clone() };
            return (next.advance_or_eof(), make_expr(span_cover(&base_span, &tok.span), field));
        }
        if tok.kind == TokenKind::IntLiteral {
            if let Some(index) = parse_int_core(strip_int_suffix(&tok.lexeme)) {
                spec_rule!("Postfix-TupleIndex");
                let access = TupleAccessExpr { base: expr, index };
                return (next.advance_or_eof(), make_expr(span_cover(&base_span, &tok.span), access));
            }
        }
        return expr_error(next, &parser);
    }
    if parser.is_punct("[") && allow_bracket {
        spec_rule!("Postfix-Index");
        let (index_parser, index) = parse_expr(parser.advance_or_eof());
        if !index_parser.is_punct("]") {
            return expr_error(index_parser, &parser);
        }
        let span = span_cover(&base_span, &index_parser.tok_span());
        return (index_parser.advance_or_eof(), make_expr(span, IndexAccessExpr { base: expr, index }));
    }
    if parser.is_punct("(") {
        spec_rule!("Postfix-Call");
        return match parse_paren_args(parser.clone(), &parser) {
            Err(error) => error,
            Ok((after, args, end_span)) => {
                let call = CallExpr { callee: expr, generic_args: Vec::new(), args };
                (after, make_expr(span_cover(&base_span, &end_span), call))
            }
        };
    }
    if parser.is_op("~>") {
        spec_rule!("Postfix-MethodCall");
        let (name_parser, name) = parse_ident(parser.advance_or_eof());
        if !name_parser.is_punct("(") {
            return expr_error(name_parser, &parser);
        }
        return match parse_paren_args(name_parser, &parser) {
            Err(error) => error,
            Ok((after, args, end_span)) => {
                let method = MethodCallExpr { receiver: expr, name, args };
                (after, make_expr(span_cover(&base_span, &end_span), method))
            }
        };
    }
    if parser.is_op("?") {
        effectful_core_rules("Postfix-Propagate");
        let span = span_cover(&base_span, &parser.tok_span());
        spec_rule!("def.16.EffectfulCoreExprAst");
        return (parser.advance_or_eof(), make_expr(span, PropagateExpr { value: expr }));
    }
    (parser, expr)
}

pub(crate) fn parse_postfix_tail(
    mut parser: Parser,
    mut expr: ExprPtr,
    allow_bracket: bool,
) -> Parsed<ExprPtr> {
    loop {
        let tok = parser.tok();
        let postfix_start = call_type_args_start(&parser) || is_postfix_start(&tok);
        let stop = !postfix_start
            || (parser.stop_before_parallel_options && is_parallel_option_list_start(&parser))
            || (!allow_bracket && tok.kind == TokenKind::Punctuator && tok.lexeme == "[");
        if stop {
            spec_rule!("Parse-PostfixTail-Stop");
            return (parser, expr);
        }
        spec_rule!("Parse-PostfixTail-Cons");
        let (next, stepped) = postfix_step(parser, expr, allow_bracket);
        parser = next;
        expr = stepped;
    }
}

pub(crate) fn parse_base_postfix(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("Parse-BasePostfix");
    let (primary_parser, primary) = parse_primary(parser, allow_brace);
    parse_postfix_tail(primary_parser, primary, allow_bracket)
}

pub(crate) fn parse_postfix(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Postfix");
    parse_pipeline(parser, allow_brace, allow_bracket)
}

// ---- primary ----------------------------------------------------------------

/// Decides whether a parenthesized form is a tuple: a top-level `,` or `;` before the
/// closing parenthesis, except for a lone trailing comma.
fn tuple_scan(parser: &Parser) -> bool {
    let mut cur = parser.clone_without_diags();
    let (mut paren, mut bracket, mut brace) = (1i32, 0i32, 0i32);
    loop {
        if cur.at_eof() {
            return false;
        }
        let tok = cur.tok();
        if tok.kind == TokenKind::Punctuator {
            let lexeme = tok.lexeme.as_str();
            if lexeme == ")" && paren == 1 {
                return false;
            }
            if (lexeme == "," || lexeme == ";") && paren == 1 && bracket == 0 && brace == 0 {
                if lexeme == "," {
                    let mut after_sep = cur.advance_or_eof();
                    skip_newlines(&mut after_sep);
                    if after_sep.is_punct(")") {
                        return false;
                    }
                }
                return true;
            }
            match lexeme {
                "(" => paren += 1,
                ")" => paren -= 1,
                "[" => bracket += 1,
                "]" if bracket > 0 => bracket -= 1,
                "{" => brace += 1,
                "}" if brace > 0 => brace -= 1,
                _ => {}
            }
        }
        cur.advance();
    }
}

fn tuple_paren(parser: &Parser) -> bool {
    if !parser.is_punct("(") {
        return false;
    }
    let next = parser.advance_or_eof();
    next.is_punct(")") || tuple_scan(&next)
}

fn scan_record_literal_start(parser: &Parser) -> bool {
    if !parser.is_ident() {
        return false;
    }
    let mut after_path = parser.advance_or_eof();
    let mut path_segments = 1usize;
    while after_path.is_op("::") {
        let after_separator = after_path.advance_or_eof();
        if !after_separator.is_ident() {
            return false;
        }
        after_path = after_separator.advance_or_eof();
        path_segments += 1;
    }
    if after_path.is_op("@") {
        return true;
    }
    if after_path.is_punct("{") && path_segments == 1 {
        return true;
    }
    after_path.is_op("<") && skip_angles(&after_path).is_op("@")
}

fn try_parse_splice_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
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
        let expr = make_expr(span_between(parser, &after_dollar), ErrorExpr {});
        return Some((after_dollar, expr));
    }
    let mut inner = after_dollar.advance_or_eof();
    inner.quote_mode = false;
    let (mut after_inner, splice_expr) = parse_expr(inner);
    after_inner.quote_mode = parser.quote_mode;
    if !after_inner.is_punct(")") {
        return Some(expr_error(after_inner, parser));
    }
    let after_r = after_inner.advance_or_eof();
    let span = span_between(parser, &after_r);
    let expr = make_expr(span.clone(), SpliceExprNode { expr: splice_expr, span });
    Some((after_r, expr))
}

fn try_parse_splice_ident_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    if !parser.is_op("$") {
        return None;
    }
    let after_dollar = parser.advance_or_eof();
    let tok = after_dollar.tok();
    if tok.kind != TokenKind::Identifier {
        return None;
    }
    let mut after_ident = after_dollar.advance_or_eof();
    let span = span_between(parser, &after_ident);
    if !parser.quote_mode {
        emit_splice_outside_quote_err(&mut after_ident, span.clone());
        return Some((after_ident, make_expr(span, ErrorExpr {})));
    }
    let ident = IdentifierExpr { name: tok.lexeme.clone(), ..Default::default() };
    let splice = SpliceIdentNode { name_expr: make_expr(tok.span.clone(), ident), span: span.clone() };
    Some((after_ident, make_expr(span, splice)))
}

fn parse_loop_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Loop-Expr");
    let next = parser.advance_or_eof();
    if next.is_punct("{") || next.is_op("|:") {
        return parse_loop_infinite_expr(next);
    }
    match try_parse_pattern_in(&next) {
        Some(try_in) => parse_loop_iter_expr(next, try_in),
        None => parse_loop_conditional_expr(next),
    }
}

fn composition_rules(rule: &str) {
    spec_rule!("grammar.21.CompositionForms");
    spec_rule!("parse.21.CompositionPrimaryExpressions");
    spec_rule!(rule);
    spec_rule!(format!("rule.21.{rule}").as_str());
}

pub(crate) fn parse_primary(parser: Parser, allow_brace: bool) -> Parsed<ExprPtr> {
    let (attrs_parser, attrs) = parse_attribute_list_opt(parser.clone());
    if let Some(attrs) = attrs {
        let (expr_parser, expr) = parse_primary(attrs_parser, allow_brace);
        let span = span_between(&parser, &expr_parser);
        let attached = attach_expr_attrs(&expr, attrs, &span);
        return (expr_parser, attached);
    }
    let tok = parser.tok();
    if let Some(splice) = try_parse_splice_expr(&parser) {
        return splice;
    }
    if let Some(splice_ident) = try_parse_splice_ident_expr(&parser) {
        return splice_ident;
    }
    let is_kw = |keyword: &str| tok.kind == TokenKind::Keyword && tok.lexeme == keyword;
    if tok.kind == TokenKind::Operator && tok.lexeme == "@" {
        let next = parser.advance_or_eof();
        let name_tok = next.tok();
        if matches!(name_tok.kind, TokenKind::Identifier | TokenKind::Keyword) {
            let after_name = next.advance_or_eof();
            if name_tok.lexeme == "result" {
                spec_rule!("Parse-Contract-Result");
                let expr = make_expr(span_between(&parser, &after_name), ResultExpr {});
                return (after_name, expr);
            }
            if name_tok.lexeme == "entry" {
                spec_rule!("Parse-Contract-Entry");
                if !after_name.is_punct("(") {
                    return expr_error(after_name, &parser);
                }
                let (expr_parser, expr) = parse_expr(after_name.advance_or_eof());
                if !expr_parser.is_punct(")") {
                    return expr_error(expr_parser, &parser);
                }
                let after_r = expr_parser.advance_or_eof();
                let entry = make_expr(span_between(&parser, &after_r), EntryExpr { expr });
                return (after_r, entry);
            }
        }
        return expr_error(next, &parser);
    }
    if is_kw("yield") {
        let mut next = parser.advance_or_eof();
        let maybe_release = next.tok();
        let release = maybe_release.kind == TokenKind::Identifier && maybe_release.lexeme == "release";
        if release {
            spec_rule!("requirement.21.AsyncKeySyntaxSurface");
            spec_rule!("requirement.21.AsyncKeyParsingSurface");
            spec_rule!("def.21.AsyncKeyExistingAstForms");
            spec_rule!("requirement.21.AsyncKeyNoAdditionalAstVariants");
            next.advance();
        }
        if next.is_kw("from") {
            spec_rule!("Parse-Yield-From-Expr");
            let (expr_parser, value) = parse_expr(next.advance_or_eof());
            let expr = make_expr(span_between(&parser, &expr_parser), YieldFromExpr { release, value });
            return (expr_parser, expr);
        }
        spec_rule!("Parse-Yield-Expr");
        let (expr_parser, value) = parse_expr(next);
        let expr = make_expr(span_between(&parser, &expr_parser), YieldExpr { release, value });
        return (expr_parser, expr);
    }
    if is_kw("sync") {
        composition_rules("Parse-Sync-Expr");
        let (expr_parser, value) = parse_expr(parser.advance_or_eof());
        let expr = make_expr(span_between(&parser, &expr_parser), SyncExpr { value });
        return (expr_parser, expr);
    }
    if is_kw("race") {
        composition_rules("Parse-Race-Expr");
        return parse_race_expr(parser);
    }
    if is_kw("all") {
        composition_rules("Parse-All-Expr");
        return parse_all_expr(parser);
    }
    if is_kw("spawn") {
        spec_rule!("Parse-Spawn-Expr");
        return parse_spawn_expr(parser);
    }
    if is_kw("dispatch") {
        spec_rule!("Parse-Dispatch-Expr");
        return parse_dispatch_expr(parser);
    }
    if is_kw("parallel") {
        spec_rule!("Parse-Parallel-Expr");
        return parse_parallel_expr(parser);
    }
    if let Some(wait) = try_parse_wait_expr(&parser) {
        return wait;
    }
    let after_head = parser.advance_or_eof();
    if tok.kind == TokenKind::Identifier && tok.lexeme == "fence" && after_head.is_punct("(") {
        let cur = after_head.advance_or_eof();
        let order_tok = cur.tok();
        let order = match (order_tok.kind, order_tok.lexeme.as_str()) {
            (TokenKind::Identifier, "acquire") => FenceOrder::Acquire,
            (TokenKind::Identifier, "release") => FenceOrder::Release,
            (TokenKind::Identifier, "seqcst") => FenceOrder::SeqCst,
            _ => return expr_error(cur, &parser),
        };
        let cur = cur.advance_or_eof();
        if !cur.is_punct(")") {
            return expr_error(cur, &parser);
        }
        let after_rparen = cur.advance_or_eof();
        let expr = make_expr(span_between(&parser, &after_rparen), FenceExpr { order });
        return (after_rparen, expr);
    }
    if let Some(closure) = try_parse_closure_expr(&parser) {
        return closure;
    }
    if let Some(recv) = try_parse_receiver_ref(&parser) {
        return recv;
    }
    if let Some(lit) = try_parse_literal_expr(&parser) {
        return lit;
    }
    if let Some(ptr_null) = try_parse_ptr_null_expr(&parser) {
        return ptr_null;
    }
    if is_kw("if") {
        return parse_if_expr(parser);
    }
    if is_kw("loop") {
        return parse_loop_expr(parser);
    }
    let is_sizeof_ident = tok.kind == TokenKind::Identifier && tok.lexeme == "sizeof";
    let is_alignof_ident = tok.kind == TokenKind::Identifier && tok.lexeme == "alignof";
    let is_contextual_intrinsic = after_head.is_punct("(") && (is_sizeof_ident || is_alignof_ident);
    if is_kw("sizeof") || is_kw("alignof") || is_contextual_intrinsic {
        let is_sizeof = is_kw("sizeof") || is_sizeof_ident;
        if !after_head.is_punct("(") {
            return expr_error(after_head, &parser);
        }
        let (type_parser, ty) = parse_type(after_head.advance_or_eof());
        if !type_parser.is_punct(")") {
            return expr_error(type_parser, &parser);
        }
        let after_rparen = type_parser.advance_or_eof();
        let span = span_between(&parser, &after_rparen);
        let expr = if is_sizeof {
            make_expr(span, SizeofExpr { r#type: ty })
        } else {
            make_expr(span, AlignofExpr { r#type: ty })
        };
        return (after_rparen, expr);
    }
    if let Some(transmute) = try_parse_transmute_expr(&parser) {
        return transmute;
    }
    if is_kw("unsafe") {
        spec_rule!("Parse-Unsafe-Block-Expr");
        effectful_core_rules("Parse-Unsafe-Expr");
        let (block_parser, block) = parse_block(after_head);
        spec_rule!("def.16.EffectfulCoreExprAst");
        let expr = make_expr(span_between(&parser, &block_parser), UnsafeBlockExpr { block });
        return (block_parser, expr);
    }
    if let Some(comptime) = try_parse_comptime_expr(&parser) {
        spec_rule!("Parse-Comptime-Expr");
        return comptime;
    }
    if let Some(type_lit) = try_parse_type_literal_expr(&parser) {
        spec_rule!("Parse-Type-Literal");
        return type_lit;
    }
    if let Some(quote) = try_parse_quote_expr(&parser) {
        spec_rule!("Parse-Quote");
        return quote;
    }
    if allow_brace && parser.is_punct("{") {
        spec_rule!("Parse-Block-Expr");
        let (block_parser, block) = parse_block(parser.clone());
        let expr = make_expr(span_between(&parser, &block_parser), BlockExpr { block });
        return (block_parser, expr);
    }
    if parser.is_punct("(") {
        if tuple_paren(&parser) {
            spec_rule!("Parse-Tuple-Literal");
            let (elems_parser, elements) = parse_tuple_expr_elems(after_head);
            if !elems_parser.is_punct(")") {
                return expr_error(elems_parser, &parser);
            }
            let after = elems_parser.advance_or_eof();
            let expr = make_expr(span_between(&parser, &after), TupleExpr { elements });
            return (after, expr);
        }
        spec_rule!("Parse-Parenthesized-Expr");
        let (inner_parser, inner) = parse_expr(after_head);
        if !inner_parser.is_punct(")") {
            return expr_error(inner_parser, &parser);
        }
        return (inner_parser.advance_or_eof(), inner);
    }
    if parser.is_punct("[") {
        spec_rule!("Parse-Array-Literal");
        return parse_array_literal_expr(parser);
    }
    if tok.kind == TokenKind::Identifier {
        if allow_brace && scan_record_literal_start(&parser) {
            return parse_record_literal(parser, allow_brace);
        }
        if let Some(ident) = try_parse_identifier_expr(&parser, allow_brace) {
            return ident;
        }
        return parse_qualified_apply(parser, allow_brace);
    }
    expr_error(parser.clone(), &parser)
}
