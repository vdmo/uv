//! Generic parameter lists: `<T <: Bound; U = Default>`.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use crate::ast::*;
use crate::parser::paths::{parse_class_path, parse_ident};
use crate::parser::recovery::emit_parse_syntax_err;
use crate::parser::state::{span_between, Parsed, Parser};
use crate::parser::types::{parse_generic_args_opt, parse_type};

fn record_generic_params_rule(rule_id: &str, span: &Span, params_opt: &str, count: usize, terminator: &str) {
    if !Conformance::enabled() {
        return;
    }
    let mut payload = format!("params_opt={params_opt};param_count={count}");
    if !terminator.is_empty() {
        payload.push_str(";terminator=");
        payload.push_str(terminator);
    }
    Conformance::record_at(rule_id, Some(span), &payload);
}

/// A syntax error carrying a machine-applicable replacement.
pub(crate) fn emit_fix_it(parser: &mut Parser, span: Span, message: &str, fix_text: &str) {
    if let Some(mut diag) = make_diagnostic_by_id("E-SRC-0520", Some(span.clone())) {
        diag.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::FixIt,
            message: message.to_string(),
            span: Some(span),
            fix_text: Some(fix_text.to_string()),
            label: None,
        });
        emit(&mut parser.diags, diag);
    }
}

fn parse_class_bound(parser: Parser) -> Parsed<TypeBound> {
    let (path_parser, class_path) = parse_class_path(parser.clone());
    let (args_parser, args) = parse_generic_args_opt(path_parser);
    let bound = TypeBound { class_path, generic_args: args.unwrap_or_default() };
    if Conformance::enabled() {
        Conformance::record_at(
            "Parse-ClassBound",
            Some(&span_between(&parser, &args_parser)),
            &format!(
                "path={};args_opt={};arg_count={}",
                bound.class_path.join("::"),
                if bound.generic_args.is_empty() { "none" } else { "some" },
                bound.generic_args.len()
            ),
        );
    }
    (args_parser, bound)
}

pub(crate) fn parse_type_bounds(parser: Parser) -> Parsed<Vec<TypeBound>> {
    if !parser.is_op("<:") {
        spec_rule!("Parse-TypeBoundsOpt-None");
        return (parser, Vec::new());
    }
    let (mut next, first) = parse_class_bound(parser.advance_or_eof());
    let mut bounds = vec![first];
    spec_rule!("Parse-ClassBoundList-Cons");
    while next.is_punct(",") {
        spec_rule!("Parse-ClassBoundListTail-Cons");
        let (bound_parser, bound) = parse_class_bound(next.advance_or_eof());
        bounds.push(bound);
        next = bound_parser;
    }
    spec_rule!("Parse-ClassBoundListTail-End");
    spec_rule!("Parse-TypeBoundsOpt-Yes");
    (next, bounds)
}

fn parse_type_param(parser: Parser) -> Parsed<TypeParam> {
    let start = parser.clone();
    let (name_parser, name) = parse_ident(parser);
    let (bounds_parser, bounds) = parse_type_bounds(name_parser);
    let (after, default_type) = if bounds_parser.is_op("=") {
        spec_rule!("Parse-TypeDefaultOpt-Yes");
        parse_type(bounds_parser.advance_or_eof())
    } else {
        spec_rule!("Parse-TypeDefaultOpt-None");
        (bounds_parser, None)
    };
    let param = TypeParam { name, bounds, default_type, variance: None, span: span_between(&start, &after) };
    if Conformance::enabled() {
        Conformance::record_at(
            "TypeParam",
            Some(&param.span),
            &format!(
                "name={};bound_count={};default_opt={};variance=none",
                param.name,
                param.bounds.len(),
                if param.default_type.is_some() { "some" } else { "none" }
            ),
        );
    }
    (after, param)
}

pub(crate) fn parse_generic_params(parser: Parser) -> Parsed<GenericParams> {
    let start = parser.clone();
    let mut next = parser;
    if next.is_op("<") {
        next.advance();
    } else {
        let span = next.tok_span();
        emit_parse_syntax_err(&mut next, span);
    }
    let (mut next, first) = parse_type_param(next);
    let mut params = vec![first];
    let mut separators = Vec::new();
    while next.is_punct(";") {
        if Conformance::enabled() {
            separators.push(next.clone_without_diags());
        }
        let (param_parser, param) = parse_type_param(next.advance_or_eof());
        params.push(param);
        next = param_parser;
    }
    record_generic_params_rule(
        "Parse-TypeParamTail-End",
        &next.tok_span(),
        "tail_end",
        params.len(),
        &next.tok().lexeme,
    );
    for separator in separators.iter().rev() {
        record_generic_params_rule(
            "Parse-TypeParamTail-Cons",
            &span_between(separator, &next),
            "tail_cons",
            params.len(),
            "",
        );
    }
    if next.is_punct(",") {
        let span = next.tok_span();
        emit_fix_it(&mut next, span, "replace `,` with `;`", ";");
    }
    if next.is_op(">") {
        next.advance();
    } else {
        let span = next.tok_span();
        emit_parse_syntax_err(&mut next, span);
    }
    let span = span_between(&start, &next);
    if Conformance::enabled() {
        let names: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
        Conformance::record_at(
            "TypeParamNames(params)",
            Some(&span),
            &format!("name_count={};names={}", names.len(), names.join(",")),
        );
    }
    record_generic_params_rule("Parse-GenericParams", &span, "required", params.len(), "");
    (next, GenericParams { params, span })
}

pub(crate) fn parse_generic_params_opt(parser: Parser) -> Parsed<Option<GenericParams>> {
    if !parser.is_op("<") {
        record_generic_params_rule("Parse-GenericParamsOpt-None", &parser.tok_span(), "none", 0, "");
        return (parser, None);
    }
    let (params_parser, params) = parse_generic_params(parser.clone());
    record_generic_params_rule(
        "Parse-GenericParamsOpt-Yes",
        &span_between(&parser, &params_parser),
        "some",
        params.params.len(),
        "",
    );
    (params_parser, Some(params))
}
