//! Type grammar.

use std::sync::Arc;

use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use super::angle::split_shift_r;
use super::consume::{
    emit_trailing_comma_err, match_operator, match_punct, token_in_end_set, trailing_comma_allowed,
    TokenMatch,
};
use super::expr::{parse_expr, parse_predicate_expr};
use super::paths::{parse_ident, parse_type_path};
use super::recovery::{
    emit_generic_parse_syntax_err, emit_parse_syntax_err, emit_splice_outside_quote_err, sync_type,
};
use super::state::{span_between, Parsed, Parser};
use crate::ast::*;
use crate::lexer::TokenKind;

pub(crate) fn skip_newlines_type(parser: &mut Parser) {
    while parser.is_newline() {
        parser.advance();
    }
}

pub(crate) fn make_type(span: Span, node: impl Into<TypeNode>) -> TypePtr {
    Some(Arc::new(Type { span, node: node.into() }))
}

pub(crate) fn make_type_prim(span: Span, name: &str) -> TypePtr {
    make_type(span, TypePrim { name: name.to_string() })
}

/// Reports a syntax error at the current token and yields the never type over `start..here`.
fn type_error(mut parser: Parser, start: &Parser) -> Parsed<TypePtr> {
    let span = parser.tok_span();
    emit_parse_syntax_err(&mut parser, span);
    let ty = make_type_prim(span_between(start, &parser), "!");
    (parser, ty)
}

pub fn is_int_type_lexeme(lexeme: &str) -> bool {
    matches!(
        lexeme,
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "isize" | "usize"
    )
}

pub fn is_float_type_lexeme(lexeme: &str) -> bool {
    matches!(lexeme, "f16" | "f32" | "f64")
}

pub fn is_prim_lexeme_set(lexeme: &str) -> bool {
    is_int_type_lexeme(lexeme) || is_float_type_lexeme(lexeme) || lexeme == "bool" || lexeme == "char"
}

pub(crate) fn parse_perm_opt(mut parser: Parser) -> Parsed<Option<TypePerm>> {
    let tok = parser.tok();
    if tok.kind == TokenKind::Keyword {
        let perm = match tok.lexeme.as_str() {
            "const" => Some((TypePerm::Const, "Parse-Perm-Const")),
            "unique" => Some((TypePerm::Unique, "Parse-Perm-Unique")),
            "shared" => Some((TypePerm::Shared, "Parse-Perm-Shared")),
            _ => None,
        };
        if let Some((perm, rule)) = perm {
            spec_rule!(rule);
            parser.advance();
            return (parser, Some(perm));
        }
    }
    spec_rule!("Parse-Perm-None");
    (parser, None)
}

/// `@State` suffix shared by `string`, `bytes` and `Ptr<T>`.
fn parse_state_suffix<S: Copy>(
    mut parser: Parser,
    family: &str,
    states: &[(&str, S)],
) -> Parsed<Option<S>> {
    if !parser.is_op("@") {
        spec_rule!(format!("Parse-{family}-None").as_str());
        return (parser, None);
    }
    parser.advance();
    let tok = parser.tok();
    if tok.kind == TokenKind::Identifier {
        if let Some((name, state)) = states.iter().find(|(name, _)| tok.lexeme == *name) {
            spec_rule!(format!("Parse-{family}-{name}").as_str());
            if family == "PtrState" && *name == "Expired" {
                spec_rule!("rule.13.Parse-PtrState-Expired");
            }
            parser.advance();
            return (parser, Some(*state));
        }
    }
    let span = parser.tok_span();
    emit_parse_syntax_err(&mut parser, span);
    (parser, None)
}

fn parse_string_state(parser: Parser) -> Parsed<Option<StringState>> {
    parse_state_suffix(
        parser,
        "StringState",
        &[("Managed", StringState::Managed), ("View", StringState::View)],
    )
}

fn parse_bytes_state(parser: Parser) -> Parsed<Option<BytesState>> {
    parse_state_suffix(
        parser,
        "BytesState",
        &[("Managed", BytesState::Managed), ("View", BytesState::View)],
    )
}

fn parse_ptr_state(parser: Parser) -> Parsed<Option<PtrState>> {
    parse_state_suffix(
        parser,
        "PtrState",
        &[("Valid", PtrState::Valid), ("Null", PtrState::Null), ("Expired", PtrState::Expired)],
    )
}

fn parse_safe_pointer_type(parser: Parser) -> Parsed<TypePtr> {
    let start = parser.clone();
    let mut after_ident = parser;
    after_ident.advance();
    if !after_ident.is_op("<") {
        return type_error(after_ident, &start);
    }
    after_ident.advance();
    let (elem_parser, element) = parse_type(after_ident);
    let after_close = if elem_parser.is_op(">>") {
        spec_rule!("Parse-Safe-Pointer-Type-ShiftSplit");
        split_shift_r(&elem_parser).advance_or_eof()
    } else if elem_parser.is_op(">") {
        spec_rule!("Parse-Safe-Pointer-Type");
        elem_parser.advance_or_eof()
    } else {
        return type_error(elem_parser, &start);
    };
    let (parser, state) = parse_ptr_state(after_close);
    let ty = make_type(span_between(&start, &parser), TypeSafePtr { element, state });
    (parser, ty)
}

fn parse_raw_ptr_type(parser: Parser) -> Parsed<TypePtr> {
    let start = parser.clone();
    let mut next = parser;
    next.advance();
    let qual = next.tok();
    if qual.kind == TokenKind::Keyword && (qual.lexeme == "imm" || qual.lexeme == "mut") {
        spec_rule!("Parse-Raw-Pointer-Type");
        let qual = if qual.lexeme == "imm" { RawPtrQual::Imm } else { RawPtrQual::Mut };
        next.advance();
        let (parser, element) = parse_type(next);
        let ty = make_type(span_between(&start, &parser), TypeRawPtr { qual, element });
        return (parser, ty);
    }
    type_error(next, &start)
}

fn parse_dynamic_type(parser: Parser) -> Parsed<TypePtr> {
    spec_rule!("Parse-Dynamic-Type");
    let start = parser.clone();
    let (parser, path) = parse_type_path(parser.advance_or_eof());
    let ty = make_type(span_between(&start, &parser), TypeDynamic { path });
    (parser, ty)
}

fn parse_opaque_type(parser: Parser) -> Parsed<TypePtr> {
    let start = parser.clone();
    let (parser, path) = parse_type_path(parser.advance_or_eof());
    spec_rule!("Parse-Opaque-Type");
    let ty = make_type(span_between(&start, &parser), TypeOpaque { path });
    (parser, ty)
}

fn parse_array_type(parser: Parser, start: &Parser, element: TypePtr) -> Parsed<TypePtr> {
    spec_rule!("Parse-Array-Type");
    let (len_parser, length) = parse_expr(parser.advance_or_eof());
    if !len_parser.is_punct("]") {
        return type_error(len_parser, start);
    }
    let after_r = len_parser.advance_or_eof();
    let ty = make_type(span_between(start, &after_r), TypeArray { element, length });
    (after_r, ty)
}

fn parse_slice_type(parser: Parser, start: &Parser, element: TypePtr) -> Parsed<TypePtr> {
    spec_rule!("Parse-Slice-Type");
    let after_r = parser.advance_or_eof();
    let ty = make_type(span_between(start, &after_r), TypeSlice { element });
    (after_r, ty)
}

const TUPLE_END_SET: [TokenMatch; 2] = [match_punct(")"), match_punct("}")];
const GENERIC_END_SET: [TokenMatch; 2] = [match_operator(">"), match_operator(">>")];

fn parse_tuple_type_elems(mut parser: Parser) -> Parsed<Vec<TypePtr>> {
    skip_newlines_type(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-TupleTypeElems-Empty");
        return (parser, Vec::new());
    }
    let (mut after_first, first) = parse_type(parser);
    skip_newlines_type(&mut after_first);
    if after_first.is_punct(";") {
        spec_rule!("Parse-TupleTypeElems-One");
        return (after_first.advance_or_eof(), vec![first]);
    }
    if after_first.is_punct(",") {
        let mut after = after_first.advance_or_eof();
        skip_newlines_type(&mut after);
        if after.is_punct(")") {
            // The reference reports this on a parser copy that it then discards.
            let mut discarded = after_first.clone();
            let span = discarded.tok_span();
            emit_parse_syntax_err(&mut discarded, span);
            return (after, vec![first]);
        }
        spec_rule!("Parse-TupleTypeElems-Many");
        let (second_parser, second) = parse_type(after);
        let (parser, tail) = parse_type_list_tail(second_parser, vec![second], &TUPLE_END_SET);
        let mut elems = Vec::with_capacity(1 + tail.len());
        elems.push(first);
        elems.extend(tail);
        return (parser, elems);
    }
    let span = after_first.tok_span();
    emit_parse_syntax_err(&mut after_first, span);
    (after_first, vec![first])
}

pub(crate) fn parse_type_list_tail(
    mut parser: Parser,
    mut xs: Vec<TypePtr>,
    end_set: &[TokenMatch],
) -> Parsed<Vec<TypePtr>> {
    loop {
        skip_newlines_type(&mut parser);
        if token_in_end_set(&parser.tok(), end_set) {
            spec_rule!("Parse-TypeListTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines_type(&mut after);
            if token_in_end_set(&after.tok(), end_set) {
                if trailing_comma_allowed(&parser, end_set) {
                    spec_rule!("Parse-TypeListTail-TrailingComma");
                }
                emit_trailing_comma_err(&mut parser, end_set);
                after.diags = parser.diags;
                return (after, xs);
            }
            spec_rule!("Parse-TypeListTail-Comma");
            let (next, elem) = parse_type(after);
            xs.push(elem);
            parser = next;
            continue;
        }
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        return (parser, xs);
    }
}

fn parse_union_tail(mut parser: Parser, allow_union: bool) -> Parsed<Vec<TypePtr>> {
    let mut elems = Vec::new();
    loop {
        if !allow_union || !parser.is_op("|") {
            spec_rule!("Parse-UnionTail-None");
            return (parser, elems);
        }
        spec_rule!("Parse-UnionTail-Cons");
        let (next, head) = parse_non_perm_type(parser.advance_or_eof());
        elems.push(head);
        parser = next;
    }
}

fn record_generic_args_rule(rule_id: &str, span: Span, args_opt: &str, arg_count: usize) {
    if Conformance::enabled() {
        Conformance::record_at(
            rule_id,
            Some(&span),
            &format!("args_opt={args_opt};arg_count={arg_count}"),
        );
    }
}

pub(crate) fn parse_generic_args(parser: Parser) -> Parsed<Vec<TypePtr>> {
    let (first_parser, first_arg) = parse_type(parser.advance_or_eof());
    let (mut cur, args) = parse_type_list_tail(first_parser, vec![first_arg], &GENERIC_END_SET);
    if cur.is_op(">>") {
        cur = split_shift_r(&cur);
    }
    if cur.is_op(">") {
        cur.advance();
    } else {
        let span = cur.tok_span();
        emit_parse_syntax_err(&mut cur, span);
    }
    record_generic_args_rule("Parse-GenericArgs", span_between(&parser, &cur), "required", args.len());
    (cur, args)
}

pub(crate) fn parse_generic_args_opt(parser: Parser) -> Parsed<Option<Vec<TypePtr>>> {
    if !parser.is_op("<") {
        record_generic_args_rule("Parse-GenericArgsOpt-None", parser.tok_span(), "none", 0);
        return (parser, None);
    }
    let (parsed, args) = parse_generic_args(parser.clone());
    record_generic_args_rule(
        "Parse-GenericArgsOpt-Yes",
        span_between(&parser, &parsed),
        "some",
        args.len(),
    );
    (parsed, Some(args))
}

pub fn make_type_modal_ref(path: Path, generic_args: Vec<TypePtr>) -> TypeModalRef {
    if generic_args.is_empty() {
        TypeModalRef::TypePathType(TypePathType { path, generic_args: Vec::new() })
    } else {
        TypeModalRef::TypeApply(TypeApply { path, args: generic_args })
    }
}

fn parse_modal_state_type(
    parser: Parser,
    start: &Parser,
    path: Path,
    generic_args: Vec<TypePtr>,
) -> Parsed<TypePtr> {
    spec_rule!("Parse-Modal-State-Type");
    let (parser, state) = parse_ident(parser.advance_or_eof());
    let is_async_state = matches!(path.as_slice(), [head] if head == "Async")
        && matches!(state.as_str(), "Suspended" | "Completed" | "Failed");
    if is_async_state {
        spec_rule!("requirement.21.AsyncTypeNoAdditionalConcreteGrammar");
        spec_rule!("requirement.21.ReservedAsyncStates");
    }
    let modal = TypeModalState {
        modal_ref: make_type_modal_ref(path.clone(), generic_args.clone()),
        path,
        generic_args,
        state,
    };
    let ty = make_type(span_between(start, &parser), modal);
    (parser, ty)
}

pub(crate) fn parse_refinement_opt(parser: Parser) -> Parsed<ExprPtr> {
    if !parser.is_op("|:") {
        spec_rule!("Parse-RefinementOpt-None");
        return (parser, None);
    }
    let after_clause = parser.advance_or_eof();
    if !after_clause.is_punct("{") {
        spec_rule!("Parse-RefinementOpt-None");
        return (parser, None);
    }
    let (mut pred_parser, pred) = parse_predicate_expr(after_clause.advance_or_eof());
    if !pred_parser.is_punct("}") {
        let span = pred_parser.tok_span();
        emit_parse_syntax_err(&mut pred_parser, span);
        sync_type(&mut pred_parser);
        return (pred_parser, None);
    }
    spec_rule!("Parse-RefinementOpt-Yes");
    (pred_parser.advance_or_eof(), pred)
}

fn parse_refinement_clause(parser: Parser, start: &Parser, base: TypePtr) -> Parsed<TypePtr> {
    let (parser, predicate) = parse_refinement_opt(parser);
    if predicate.is_none() {
        return (parser, base);
    }
    spec_rule!("Parse-Refinement-Type");
    let ty = make_type(span_between(start, &parser), TypeRefine { base, predicate });
    (parser, ty)
}

fn has_func_arrow(parser: &Parser) -> bool {
    if !parser.is_punct("(") {
        return false;
    }
    let mut cur = parser.clone_without_diags();
    let mut depth = 0i32;
    while !cur.at_eof() {
        if cur.is_punct("(") {
            depth += 1;
        } else if cur.is_punct(")") {
            depth -= 1;
            if depth == 0 {
                return cur.advance_or_eof().is_op("->");
            }
        }
        cur.advance();
    }
    false
}

fn parse_param_type(parser: Parser) -> Parsed<TypeFuncParam> {
    if parser.is_kw("move") {
        spec_rule!("Parse-ParamType-Move");
        let (parser, ty) = parse_type(parser.advance_or_eof());
        return (parser, TypeFuncParam { mode: Some(ParamMode::Move), r#type: ty });
    }
    spec_rule!("Parse-ParamType-Plain");
    let (parser, ty) = parse_type(parser);
    (parser, TypeFuncParam { mode: None, r#type: ty })
}

/// Comma-separated parameter types up to (not including) the closing token.
fn parse_param_type_list(
    mut parser: Parser,
    close: TokenMatch,
    rules: [&str; 5],
    parse_param: fn(Parser) -> Parsed<TypeFuncParam>,
) -> Parsed<Vec<TypeFuncParam>> {
    let [empty, cons, tail_end, tail_trailing, tail_cons] = rules;
    let end_set = [close];
    skip_newlines_type(&mut parser);
    if token_in_end_set(&parser.tok(), &end_set) {
        spec_rule!(empty);
        return (parser, Vec::new());
    }
    spec_rule!(cons);
    let (mut parser, first) = parse_param(parser);
    let mut params = vec![first];
    loop {
        skip_newlines_type(&mut parser);
        if token_in_end_set(&parser.tok(), &end_set) {
            spec_rule!(tail_end);
            return (parser, params);
        }
        if !parser.is_punct(",") {
            let span = parser.tok_span();
            emit_parse_syntax_err(&mut parser, span);
            return (parser, params);
        }
        let mut after = parser.advance_or_eof();
        skip_newlines_type(&mut after);
        if token_in_end_set(&after.tok(), &end_set) {
            if trailing_comma_allowed(&parser, &end_set) {
                spec_rule!(tail_trailing);
            }
            emit_trailing_comma_err(&mut parser, &end_set);
            after.diags = parser.diags;
            return (after, params);
        }
        spec_rule!(tail_cons);
        let (next, param) = parse_param(after);
        params.push(param);
        parser = next;
    }
}

fn parse_func_type(parser: Parser) -> Parsed<TypePtr> {
    spec_rule!("Parse-Func-Type");
    let start = parser.clone();
    let (params_parser, params) = parse_param_type_list(
        parser.advance_or_eof(),
        match_punct(")"),
        [
            "Parse-ParamTypeList-Empty",
            "Parse-ParamTypeList-Cons",
            "Parse-ParamTypeListTail-End",
            "Parse-ParamTypeListTail-TrailingComma",
            "Parse-ParamTypeListTail-Cons",
        ],
        parse_param_type,
    );
    if !params_parser.is_punct(")") {
        return type_error(params_parser, &start);
    }
    let after_rparen = params_parser.advance_or_eof();
    if !after_rparen.is_op("->") {
        return type_error(after_rparen, &start);
    }
    let (parser, ret) = parse_type(after_rparen.advance_or_eof());
    let ty = make_type(span_between(&start, &parser), TypeFunc { params, ret });
    (parser, ty)
}

fn parse_closure_param_type_inner(parser: Parser) -> Parsed<TypePtr> {
    if parser.is_punct("(") {
        spec_rule!("Parse-ClosureParamType-Grouped");
        let (mut ty_parser, ty) = parse_type(parser.advance_or_eof());
        if !ty_parser.is_punct(")") {
            let span = ty_parser.tok_span();
            emit_parse_syntax_err(&mut ty_parser, span);
            return (ty_parser, ty);
        }
        return (ty_parser.advance_or_eof(), ty);
    }
    spec_rule!("Parse-ClosureParamType-Plain");
    parse_type_no_union(parser)
}

fn parse_closure_param_type(parser: Parser) -> Parsed<TypeFuncParam> {
    if parser.is_kw("move") {
        spec_rule!("Parse-ParamType-Move");
        let (parser, ty) = parse_closure_param_type_inner(parser.advance_or_eof());
        return (parser, TypeFuncParam { mode: Some(ParamMode::Move), r#type: ty });
    }
    spec_rule!("Parse-ParamType-Plain");
    let (parser, ty) = parse_closure_param_type_inner(parser);
    (parser, TypeFuncParam { mode: None, r#type: ty })
}

fn parse_shared_dep(mut parser: Parser) -> Parsed<SharedDep> {
    spec_rule!("Parse-SharedDep");
    let tok = parser.tok();
    if tok.kind != TokenKind::Identifier {
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        return (parser, SharedDep { name: "_".to_string(), r#type: None });
    }
    let mut next = parser.advance_or_eof();
    if next.is_punct(":") {
        next.advance();
    } else {
        let span = next.tok_span();
        emit_parse_syntax_err(&mut next, span);
    }
    let (parser, ty) = parse_type(next);
    (parser, SharedDep { name: tok.lexeme.clone(), r#type: ty })
}

fn parse_shared_dep_list(parser: Parser) -> Parsed<Vec<SharedDep>> {
    if parser.is_punct("}") {
        spec_rule!("Parse-SharedDepList-Empty");
        return (parser, Vec::new());
    }
    let mut deps = Vec::new();
    let mut cur = parser;
    loop {
        let (first_parser, first) = parse_shared_dep(cur);
        deps.push(first);
        if !first_parser.is_punct(",") {
            spec_rule!("Parse-SharedDepList-Single");
            return (first_parser, deps);
        }
        spec_rule!("Parse-SharedDepList-Cons");
        cur = first_parser.advance_or_eof();
        if cur.is_punct("}") {
            spec_rule!("Parse-SharedDepList-Empty");
            return (cur, deps);
        }
    }
}

fn parse_closure_deps_opt(parser: Parser) -> Parsed<Option<Vec<SharedDep>>> {
    if !parser.is_punct("[") {
        spec_rule!("Parse-ClosureDepsOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-ClosureDepsOpt-Some");
    let fail = |mut parser: Parser| {
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        (parser, None)
    };
    let mut next = parser.advance_or_eof();
    if !next.is_kw("shared") {
        return fail(next);
    }
    next.advance();
    if !next.is_punct(":") {
        return fail(next);
    }
    next.advance();
    if !next.is_punct("{") {
        return fail(next);
    }
    let (mut after_deps, deps) = parse_shared_dep_list(next.advance_or_eof());
    if !after_deps.is_punct("}") {
        return fail(after_deps);
    }
    after_deps.advance();
    if !after_deps.is_punct("]") {
        return fail(after_deps);
    }
    (after_deps.advance_or_eof(), Some(deps))
}

fn parse_closure_type(parser: Parser) -> Parsed<TypePtr> {
    let start = parser.clone();
    let (after_bar, params) = if parser.is_op("||") {
        spec_rule!("Parse-ClosureParamTypeList-Empty");
        spec_rule!("Parse-Closure-Type-Empty");
        (parser.advance_or_eof(), Vec::new())
    } else if !parser.is_op("|") {
        let mut parser = parser;
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span.clone());
        return (parser, make_type_prim(span, "!"));
    } else {
        let after_l = parser.advance_or_eof();
        if after_l.is_op("|") {
            spec_rule!("Parse-ClosureParamTypeList-Empty");
            spec_rule!("Parse-Closure-Type-Empty");
            (after_l.advance_or_eof(), Vec::new())
        } else {
            spec_rule!("Parse-Closure-Type");
            let (params_parser, params) = parse_param_type_list(
                after_l,
                match_operator("|"),
                [
                    "Parse-ClosureParamTypeList-Empty",
                    "Parse-ClosureParamTypeList-Cons",
                    "Parse-ClosureParamTypeListTail-End",
                    "Parse-ClosureParamTypeListTail-TrailingComma",
                    "Parse-ClosureParamTypeListTail-Comma",
                ],
                parse_closure_param_type,
            );
            if !params_parser.is_op("|") {
                return type_error(params_parser, &start);
            }
            (params_parser.advance_or_eof(), params)
        }
    };
    if !after_bar.is_op("->") {
        return type_error(after_bar, &start);
    }
    let (ret_parser, ret) = parse_type(after_bar.advance_or_eof());
    let (parser, deps_opt) = parse_closure_deps_opt(ret_parser);
    let ty = make_type(span_between(&start, &parser), TypeClosure { params, ret, deps_opt });
    (parser, ty)
}

fn try_parse_splice_type(parser: &Parser) -> Option<Parsed<TypePtr>> {
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
        sync_type(&mut after_dollar);
        let ty = make_type_prim(span_between(parser, &after_dollar), "!");
        return Some((after_dollar, ty));
    }
    let mut inner = after_dollar.advance_or_eof();
    inner.quote_mode = false;
    let (mut after_expr, expr) = parse_expr(inner);
    after_expr.quote_mode = parser.quote_mode;
    if !after_expr.is_punct(")") {
        return Some(type_error(after_expr, parser));
    }
    let after_r = after_expr.advance_or_eof();
    let span = span_between(parser, &after_r);
    let ty = make_type(span.clone(), SpliceExprNode { expr, span });
    Some((after_r, ty))
}

fn build_builtin_range_type(span: Span, path: &[String], args: &[TypePtr]) -> TypePtr {
    let [name] = path else {
        return None;
    };
    if name == "RangeFull" {
        return if args.is_empty() { make_type(span, TypeRangeFull {}) } else { None };
    }
    let [base] = args else {
        return None;
    };
    let base = base.clone();
    match name.as_str() {
        "Range" => make_type(span, TypeRange { base }),
        "RangeInclusive" => make_type(span, TypeRangeInclusive { base }),
        "RangeFrom" => make_type(span, TypeRangeFrom { base }),
        "RangeTo" => make_type(span, TypeRangeTo { base }),
        "RangeToInclusive" => make_type(span, TypeRangeToInclusive { base }),
        _ => None,
    }
}

pub(crate) fn parse_non_perm_type(parser: Parser) -> Parsed<TypePtr> {
    let tok = parser.tok();
    let start = parser.clone();
    match tok.kind {
        TokenKind::Operator if tok.lexeme == "|" || tok.lexeme == "||" => {
            return parse_closure_type(parser);
        }
        TokenKind::Punctuator if tok.lexeme == "(" => {
            if has_func_arrow(&parser) {
                return parse_func_type(parser);
            }
            let (elems_parser, elems) = parse_tuple_type_elems(parser.advance_or_eof());
            if !elems_parser.is_punct(")") {
                return type_error(elems_parser, &start);
            }
            let after_r = elems_parser.advance_or_eof();
            let span = span_between(&start, &after_r);
            if elems.is_empty() {
                spec_rule!("Parse-Unit-Type");
                return (after_r, make_type_prim(span, "()"));
            }
            spec_rule!("Parse-Tuple-Type");
            return (after_r, make_type(span, TypeTuple { elements: elems }));
        }
        TokenKind::Punctuator if tok.lexeme == "[" => {
            let (elem_parser, elem) = parse_type(parser.advance_or_eof());
            if elem_parser.is_punct(";") {
                return parse_array_type(elem_parser, &start, elem);
            }
            if elem_parser.is_punct("]") {
                return parse_slice_type(elem_parser, &start, elem);
            }
            return type_error(elem_parser, &start);
        }
        TokenKind::Operator if tok.lexeme == "!" => {
            spec_rule!("Parse-Never-Type");
            let next = parser.advance_or_eof();
            let ty = make_type_prim(span_between(&start, &next), "!");
            return (next, ty);
        }
        TokenKind::Operator if tok.lexeme == "*" => return parse_raw_ptr_type(parser),
        _ => {}
    }
    if let Some(splice) = try_parse_splice_type(&parser) {
        return splice;
    }
    if tok.kind == TokenKind::Operator && tok.lexeme == "$" {
        return parse_dynamic_type(parser);
    }
    if tok.kind == TokenKind::Identifier {
        let lexeme = tok.lexeme.as_str();
        if lexeme == "opaque" {
            return parse_opaque_type(parser);
        }
        if is_prim_lexeme_set(lexeme) {
            spec_rule!("Parse-Prim-Type");
            let next = parser.advance_or_eof();
            let ty = make_type_prim(span_between(&start, &next), lexeme);
            return (next, ty);
        }
        if lexeme == "string" {
            spec_rule!("Parse-String-Type");
            let (parser, state) = parse_string_state(parser.advance_or_eof());
            let ty = make_type(span_between(&start, &parser), TypeString { state });
            return (parser, ty);
        }
        if lexeme == "bytes" {
            spec_rule!("Parse-Bytes-Type");
            let (parser, state) = parse_bytes_state(parser.advance_or_eof());
            let ty = make_type(span_between(&start, &parser), TypeBytes { state });
            return (parser, ty);
        }
        if lexeme == "Ptr" && parser.advance_or_eof().is_op("<") {
            return parse_safe_pointer_type(parser);
        }
        let (path_parser, path) = parse_type_path(parser);
        let (gen_parser, args) = parse_generic_args_opt(path_parser);
        let has_args = args.is_some();
        let generic_args = args.unwrap_or_default();
        if gen_parser.is_op("@") {
            return parse_modal_state_type(gen_parser, &start, path, generic_args);
        }
        let span = span_between(&start, &gen_parser);
        let builtin_range = build_builtin_range_type(span.clone(), &path, &generic_args);
        if builtin_range.is_some() {
            return (gen_parser, builtin_range);
        }
        if has_args {
            spec_rule!("Parse-Type-Apply");
            return (gen_parser, make_type(span, TypeApply { path, args: generic_args }));
        }
        spec_rule!("Parse-Type-Path");
        return (gen_parser, make_type(span, TypePathType { path, generic_args: Vec::new() }));
    }
    let mut parser = parser;
    let span = parser.tok_span();
    emit_parse_syntax_err(&mut parser, span.clone());
    (parser, make_type_prim(span, "!"))
}

fn parse_type_impl(parser: Parser, allow_union: bool) -> Parsed<TypePtr> {
    let start = parser.clone();
    let (mut after_perm, perm) = parse_perm_opt(parser);
    let with_perm = |base: TypePtr, span: Span| match perm {
        Some(perm) => make_type(span, TypePermType { perm, base }),
        None => base,
    };
    let tok = after_perm.tok();
    let non_perm_start = match tok.kind {
        TokenKind::Identifier => true,
        TokenKind::Punctuator => tok.lexeme == "(" || tok.lexeme == "[",
        TokenKind::Operator => matches!(tok.lexeme.as_str(), "*" | "$" | "!" | "|" | "||"),
        _ => false,
    };
    if !non_perm_start {
        spec_rule!("Parse-Type-Err");
        let span = after_perm.tok_span();
        emit_generic_parse_syntax_err(&mut after_perm, span);
        let span = span_between(&start, &after_perm);
        let ty = with_perm(make_type_prim(span.clone(), "!"), span);
        return (after_perm, ty);
    }
    let (base_parser, base) = parse_non_perm_type(after_perm);
    let (out, tail) = if allow_union {
        parse_union_tail(base_parser, true)
    } else {
        // Recorded for its trace only; a union tail is not consumed here.
        spec_rule!("Parse-UnionTail-None");
        (base_parser, Vec::new())
    };
    spec_rule!("Parse-Type");
    let mut merged = base;
    if !tail.is_empty() {
        let mut types = Vec::with_capacity(1 + tail.len());
        types.push(merged);
        types.extend(tail);
        merged = make_type(span_between(&start, &out), TypeUnion { types });
    }
    let merged = with_perm(merged, span_between(&start, &out));
    parse_refinement_clause(out, &start, merged)
}

pub fn parse_type(parser: Parser) -> Parsed<TypePtr> {
    parse_type_impl(parser, true)
}

pub fn parse_type_no_union(parser: Parser) -> Parsed<TypePtr> {
    parse_type_impl(parser, false)
}

pub fn parse_type_annot_opt(parser: Parser) -> Parsed<TypePtr> {
    if !parser.is_punct(":") {
        spec_rule!("Parse-TypeAnnotOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-TypeAnnotOpt-Yes");
    parse_type(parser.advance_or_eof())
}
