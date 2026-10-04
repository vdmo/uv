//! Parameter lists, receivers and return types.

use uv_core::spec_rule;

use crate::ast::*;
use crate::parser::consume::{emit_trailing_comma_err, match_punct, trailing_comma_allowed, TokenMatch};
use crate::parser::expr::skip_newlines;
use crate::parser::paths::{parse_ident, parse_local_ident};
use crate::parser::recovery::emit_parse_syntax_err;
use crate::parser::state::{span_between, Parsed, Parser};
use crate::parser::types::parse_type;

const PAREN_END: [TokenMatch; 1] = [match_punct(")")];

pub(crate) struct Signature {
    pub params: Vec<Param>,
    pub return_type_opt: TypePtr,
}

pub(crate) struct MethodSignature {
    pub receiver: Receiver,
    pub params: Vec<Param>,
    pub return_type_opt: TypePtr,
}

fn syntax_err_here(parser: &mut Parser) {
    let span = parser.tok_span();
    emit_parse_syntax_err(parser, span);
}

/// Consumes the expected punctuator, or reports a syntax error and stays in place.
pub(crate) fn expect_punct(parser: &mut Parser, punct: &str) {
    if parser.is_punct(punct) {
        parser.advance();
    } else {
        syntax_err_here(parser);
    }
}

pub(crate) fn parse_return_opt(parser: Parser) -> Parsed<TypePtr> {
    if !parser.is_op("->") {
        spec_rule!("Parse-ReturnOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-ReturnOpt-Arrow");
    parse_type(parser.advance_or_eof())
}

fn parse_param_mode_opt(parser: Parser) -> Parsed<Option<ParamMode>> {
    if parser.is_kw("move") {
        spec_rule!("Parse-ParamMode-Move");
        return (parser.advance_or_eof(), Some(ParamMode::Move));
    }
    spec_rule!("Parse-ParamMode-None");
    (parser, None)
}

fn parse_param(parser: Parser) -> Parsed<Param> {
    spec_rule!("Parse-Param");
    let start = parser.clone();
    let (mode_parser, mode) = parse_param_mode_opt(parser);
    let (mut name_parser, name) = parse_local_ident(mode_parser);
    expect_punct(&mut name_parser, ":");
    let (ty_parser, ty) = parse_type(name_parser);
    let span = span_between(&start, &ty_parser);
    let param = Param { mode, name: name.name, name_splice_opt: name.splice_opt, r#type: ty, span };
    (ty_parser, param)
}

pub(crate) fn parse_param_list(mut parser: Parser) -> Parsed<Vec<Param>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-ParamList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-ParamList-Cons");
    let (mut parser, first) = parse_param(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct(")") {
            spec_rule!("Parse-ParamTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_punct(")") {
                if trailing_comma_allowed(&parser, &PAREN_END) {
                    spec_rule!("Parse-ParamTail-TrailingComma");
                }
                emit_trailing_comma_err(&mut parser, &PAREN_END);
                after.diags = parser.diags;
                return (after, xs);
            }
            spec_rule!("Parse-ParamTail-Comma");
            let (next, param) = parse_param(after);
            xs.push(param);
            parser = next;
            continue;
        }
        syntax_err_here(&mut parser);
        return (parser, xs);
    }
}

fn parse_method_params(mut parser: Parser) -> Parsed<Vec<Param>> {
    if parser.is_punct(")") {
        spec_rule!("Parse-MethodParams-None");
        return (parser, Vec::new());
    }
    if parser.is_punct(",") {
        spec_rule!("Parse-MethodParams-Comma");
        return parse_param_list(parser.advance_or_eof());
    }
    syntax_err_here(&mut parser);
    (parser, Vec::new())
}

fn parse_receiver(parser: Parser) -> Parsed<Receiver> {
    let (mode_parser, mode_opt) = parse_param_mode_opt(parser.clone());
    let shorthand = [
        ("~", ReceiverPerm::Const, "Parse-Receiver-Short-Const"),
        ("~!", ReceiverPerm::Unique, "Parse-Receiver-Short-Unique"),
        ("~%", ReceiverPerm::Shared, "Parse-Receiver-Short-Shared"),
    ];
    for (op, perm, rule) in shorthand {
        if mode_parser.is_op(op) {
            spec_rule!(rule);
            return (mode_parser.advance_or_eof(), ReceiverShorthand { perm, mode_opt }.into());
        }
    }
    spec_rule!("Parse-Receiver-Explicit");
    let (mut name_parser, name) = parse_ident(mode_parser);
    if name != "self" {
        // The reference reports this on a parser copy that it then discards.
        let mut discarded = parser.clone();
        syntax_err_here(&mut discarded);
    }
    expect_punct(&mut name_parser, ":");
    let (ty_parser, ty) = parse_type(name_parser);
    (ty_parser, ReceiverExplicit { mode_opt, r#type: ty }.into())
}

fn default_receiver() -> Receiver {
    ReceiverShorthand { perm: ReceiverPerm::Const, mode_opt: None }.into()
}

pub(crate) fn parse_method_signature(mut parser: Parser) -> Parsed<MethodSignature> {
    spec_rule!("Parse-MethodSignature");
    if !parser.is_punct("(") {
        syntax_err_here(&mut parser);
        let signature =
            MethodSignature { receiver: default_receiver(), params: Vec::new(), return_type_opt: None };
        return (parser, signature);
    }
    let mut next = parser.advance_or_eof();
    skip_newlines(&mut next);
    let (receiver_parser, receiver) = parse_receiver(next);
    let (mut params_parser, params) = parse_method_params(receiver_parser);
    expect_punct(&mut params_parser, ")");
    let (ret_parser, return_type_opt) = parse_return_opt(params_parser);
    (ret_parser, MethodSignature { receiver, params, return_type_opt })
}

pub(crate) fn parse_state_method_signature(parser: Parser) -> Parsed<MethodSignature> {
    spec_rule!("Parse-StateMethodSignature-Receiver");
    parse_method_signature(parser)
}

pub(crate) fn parse_signature(mut parser: Parser) -> Parsed<Signature> {
    spec_rule!("Parse-Signature");
    if !parser.is_punct("(") {
        syntax_err_here(&mut parser);
        return (parser, Signature { params: Vec::new(), return_type_opt: None });
    }
    let (mut params_parser, params) = parse_param_list(parser.advance_or_eof());
    expect_punct(&mut params_parser, ")");
    let (ret_parser, return_type_opt) = parse_return_opt(params_parser);
    (ret_parser, Signature { params, return_type_opt })
}
