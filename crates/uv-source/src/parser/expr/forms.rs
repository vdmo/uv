//! Individual expression forms: literals, names, control flow, closures, records, arrays,
//! tuples, compile-time forms and the structured-concurrency constructs.

use std::sync::Arc;

use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use super::{
    espan, expr_error, is_expr_start, is_literal_token, make_expr, parse_arg_list,
    parse_base_postfix, parse_expr, parse_expr_no_brace, parse_predicate_expr, parse_range,
    skip_newlines, span_cover,
};
use crate::ast::*;
use crate::lexer::keyword_policy::ctx;
use crate::lexer::{Token, TokenKind};
use crate::parser::angle::{skip_angles, split_shift_r};
use crate::parser::consume::{emit_trailing_comma_err, match_operator, match_punct, trailing_comma_allowed, TokenMatch};
use crate::parser::paths::{parse_ident, parse_qualified_head, parse_type_path};
use crate::parser::pattern::{make_pattern, parse_pattern};
use crate::parser::recovery::{emit_parse_syntax_err, sync_stmt};
use crate::parser::state::{merge_diag, span_between, Parsed, Parser};
use crate::parser::stmt::{normalize_binding_pattern, parse_block, parse_key_path_expr};
use crate::parser::types::{
    parse_generic_args_opt, parse_type, parse_type_annot_opt, parse_type_no_union,
};

const BRACE_END: [TokenMatch; 1] = [match_punct("}")];
const BRACKET_END: [TokenMatch; 1] = [match_punct("]")];
const PAREN_END: [TokenMatch; 1] = [match_punct(")")];
const BAR_END: [TokenMatch; 1] = [match_operator("|")];

fn syntax_err_here(parser: &mut Parser) {
    let span = parser.tok_span();
    emit_parse_syntax_err(parser, span);
}

fn block_expr(start: &Parser, block_parser: &Parser, block: BlockPtr) -> ExprPtr {
    make_expr(span_between(start, block_parser), BlockExpr { block })
}

// ---- pipeline ---------------------------------------------------------------

pub(crate) fn parse_pipeline(parser: Parser, allow_brace: bool, allow_bracket: bool) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Pipeline");
    let (mut parser, mut lhs) = parse_base_postfix(parser, allow_brace, allow_bracket);
    loop {
        let stop = (parser.stop_before_contract_post_separator && parser.is_op("|="))
            || !parser.is_op("=>");
        if stop {
            spec_rule!("Parse-PipelineTail-Stop");
            return (parser, lhs);
        }
        spec_rule!("Parse-PipelineTail-Cons");
        let (rhs_parser, rhs) = parse_base_postfix(parser.advance_or_eof(), allow_brace, allow_bracket);
        let lhs_span = espan(&lhs);
        let end_span = rhs.as_ref().map_or_else(|| lhs_span.clone(), |e| e.span.clone());
        lhs = make_expr(span_cover(&lhs_span, &end_span), PipelineExpr { lhs, rhs });
        parser = rhs_parser;
    }
}

// ---- closures ---------------------------------------------------------------

fn parse_closure_param_annot_type(parser: Parser) -> Parsed<TypePtr> {
    if parser.is_punct("(") {
        spec_rule!("Parse-ClosureParamType-Grouped");
        let (mut ty_parser, ty) = parse_type(parser.advance_or_eof());
        if !ty_parser.is_punct(")") {
            syntax_err_here(&mut ty_parser);
            return (ty_parser, ty);
        }
        return (ty_parser.advance_or_eof(), ty);
    }
    spec_rule!("Parse-ClosureParamType-Plain");
    parse_type_no_union(parser)
}

fn parse_closure_param(mut parser: Parser) -> Parsed<ClosureParam> {
    spec_rule!("Parse-ClosureParam");
    let move_capture = parser.is_kw("move");
    if move_capture {
        parser.advance();
    }
    let (mut cur, name) = parse_ident(parser);
    let mut type_opt = None;
    if cur.is_punct(":") {
        let (ty_parser, ty) = parse_closure_param_annot_type(cur.advance_or_eof());
        type_opt = ty;
        cur = ty_parser;
    }
    match (move_capture, type_opt.is_some()) {
        (true, true) => spec_rule!("Parse-ClosureParam-MoveTyped"),
        (true, false) => spec_rule!("Parse-ClosureParam-MoveUntyped"),
        (false, true) => spec_rule!("Parse-ClosureParam-Typed"),
        (false, false) => spec_rule!("Parse-ClosureParam-Untyped"),
    }
    (cur, ClosureParam { move_capture, name, type_opt })
}

fn parse_closure_param_list(mut parser: Parser) -> Parsed<Vec<ClosureParam>> {
    skip_newlines(&mut parser);
    spec_rule!("Parse-ClosureParams-Single");
    let (mut parser, first) = parse_closure_param(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_op("|") {
            spec_rule!("Parse-ClosureParamListTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_op("|") {
                emit_trailing_comma_err(&mut parser, &BAR_END);
                after.diags = parser.diags;
                return (after, xs);
            }
            spec_rule!("Parse-ClosureParams-Cons");
            let (next, param) = parse_closure_param(after);
            xs.push(param);
            parser = next;
            continue;
        }
        syntax_err_here(&mut parser);
        return (parser, xs);
    }
}

pub(crate) fn try_parse_closure_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    if !(parser.is_op("|") || parser.is_op("||")) {
        return None;
    }
    let start = parser;
    let mut params = Vec::new();
    let after_bar = if parser.is_op("||") {
        spec_rule!("Parse-Closure-Expr-Empty");
        parser.advance_or_eof()
    } else {
        let next = parser.advance_or_eof();
        if next.is_op("|") {
            spec_rule!("Parse-Closure-Expr-Empty");
            next.advance_or_eof()
        } else {
            spec_rule!("Parse-Closure-Expr");
            let (params_parser, parsed_params) = parse_closure_param_list(next);
            if !params_parser.is_op("|") {
                return Some(expr_error(params_parser, start));
            }
            params = parsed_params;
            params_parser.advance_or_eof()
        }
    };
    let (cur, ret_type_opt) = if after_bar.is_op("->") {
        spec_rule!("Parse-ClosureRetOpt-Some");
        parse_type(after_bar.advance_or_eof())
    } else {
        spec_rule!("Parse-ClosureRetOpt-None");
        (after_bar, None)
    };
    let (body_parser, body) = if cur.is_punct("{") {
        spec_rule!("Parse-ClosureBody-Block");
        let (block_parser, block) = parse_block(cur.clone());
        let body = block_expr(&cur, &block_parser, block);
        (block_parser, body)
    } else {
        spec_rule!("Parse-ClosureBody-Expr");
        parse_expr(cur)
    };
    let closure = ClosureExpr { params, ret_type_opt, body };
    let expr = make_expr(span_between(start, &body_parser), closure);
    Some((body_parser, expr))
}

// ---- literals and names -----------------------------------------------------

pub(crate) fn try_parse_literal_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    let tok = parser.tok();
    if !is_literal_token(&tok) {
        return None;
    }
    spec_rule!("Parse-Literal-Expr");
    let expr = make_expr(tok.span.clone(), LiteralExpr { literal: (*tok).clone() });
    Some((parser.advance_or_eof(), expr))
}

fn parse_literal_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Literal-Expr");
    let tok = parser.tok();
    let expr = make_expr(tok.span.clone(), LiteralExpr { literal: (*tok).clone() });
    (parser.advance_or_eof(), expr)
}

/// `Ptr::null()`.
pub(crate) fn try_parse_ptr_null_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    let tok = parser.tok();
    if tok.kind != TokenKind::Identifier || tok.lexeme != "Ptr" {
        return None;
    }
    let next = parser.advance_or_eof();
    if !next.is_op("::") {
        return None;
    }
    let after_colon = next.advance_or_eof();
    if after_colon.tok().kind != TokenKind::NullLiteral {
        return None;
    }
    let after_lit = after_colon.advance_or_eof();
    if !after_lit.is_punct("(") {
        return None;
    }
    let after_l = after_lit.advance_or_eof();
    if !after_l.is_punct(")") {
        return None;
    }
    spec_rule!("Parse-Null-Ptr");
    let after = after_l.advance_or_eof();
    let expr = make_expr(span_between(parser, &after), PtrNullExpr {});
    Some((after, expr))
}

pub(crate) fn try_parse_receiver_ref(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    if !parser.is_op("~") {
        return None;
    }
    spec_rule!("Parse-Receiver-Ref");
    let ident = IdentifierExpr { name: "~".to_string(), ..Default::default() };
    Some((parser.advance_or_eof(), make_expr(parser.tok_span(), ident)))
}

pub(crate) fn try_parse_identifier_expr(parser: &Parser, allow_brace: bool) -> Option<Parsed<ExprPtr>> {
    let tok = parser.tok();
    if tok.kind != TokenKind::Identifier {
        return None;
    }
    let next = parser.advance_or_eof();
    if next.is_op("::") || (allow_brace && (next.is_op("@") || next.is_punct("{"))) {
        return None;
    }
    spec_rule!("Parse-Identifier-Expr");
    if matches!(tok.lexeme.as_str(), "emitter" | "introspect" | "files" | "diagnostics") {
        spec_rule!("Parse-CtCapRef");
    }
    let ident = IdentifierExpr { name: tok.lexeme.clone(), ..Default::default() };
    Some((next, make_expr(tok.span.clone(), ident)))
}

pub(crate) fn parse_qualified_apply(parser: Parser, allow_brace: bool) -> Parsed<ExprPtr> {
    if !parser.is_ident() {
        let mut parser = parser;
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span.clone());
        return (parser, make_expr(span, ErrorExpr {}));
    }
    let (mut head_parser, head) = parse_qualified_head(parser.clone());
    if head_parser.is_op("<") {
        syntax_err_here(&mut head_parser);
        let mut skip = skip_angles(&head_parser);
        sync_stmt(&mut skip);
        let expr = make_expr(span_between(&parser, &skip), ErrorExpr {});
        return (skip, expr);
    }
    if head_parser.is_punct("(") {
        spec_rule!("Parse-Qualified-Apply-Paren");
        let (args_parser, args) = parse_arg_list(head_parser.advance_or_eof());
        if !args_parser.is_punct(")") {
            return expr_error(args_parser, &parser);
        }
        let after = args_parser.advance_or_eof();
        let app = QualifiedApplyExpr {
            path: head.module_path,
            name: head.name,
            args: ParenArgs { args }.into(),
        };
        let expr = make_expr(span_between(&parser, &after), app);
        return (after, expr);
    }
    if allow_brace && head_parser.is_punct("{") {
        spec_rule!("Parse-Qualified-Apply-Brace");
        let (mut fields_parser, fields) = parse_field_init_list(head_parser.advance_or_eof());
        if fields.is_empty() && fields_parser.is_punct("}") {
            syntax_err_here(&mut fields_parser);
            let after = fields_parser.advance_or_eof();
            let expr = make_expr(span_between(&parser, &after), ErrorExpr {});
            return (after, expr);
        }
        if !fields_parser.is_punct("}") {
            return expr_error(fields_parser, &parser);
        }
        let after = fields_parser.advance_or_eof();
        let app = QualifiedApplyExpr {
            path: head.module_path,
            name: head.name,
            args: BraceArgs { fields }.into(),
        };
        let expr = make_expr(span_between(&parser, &after), app);
        return (after, expr);
    }
    spec_rule!("Parse-Qualified-Name");
    let qname = QualifiedNameExpr { path: head.module_path, name: head.name };
    let expr = make_expr(span_between(&parser, &head_parser), qname);
    (head_parser, expr)
}

// ---- if ---------------------------------------------------------------------

enum PendingIfKind {
    Plain,
    IsSingle,
}

struct PendingIfArm {
    start: Parser,
    kind: PendingIfKind,
    cond_or_scrutinee: ExprPtr,
    pattern_opt: PatternPtr,
    then_expr: ExprPtr,
}

enum IfHead {
    Completed(ExprPtr),
    Pending(PendingIfArm),
}

fn parse_case_body_block(parser: Parser) -> Parsed<ExprPtr> {
    let (block_parser, block) = parse_block(parser.clone());
    let body = block_expr(&parser, &block_parser, block);
    (block_parser, body)
}

/// A case pattern read without a payload, for when `{` must start the case body.
fn try_parse_brace_disambiguated_if_case_pattern(parser: Parser) -> Option<Parsed<PatternPtr>> {
    if parser.is_op("@") {
        spec_rule!("Parse-IfCase-Pattern-ModalHead");
        let (name_parser, state) = parse_ident(parser.advance_or_eof());
        let pattern = make_pattern(
            span_between(&parser, &name_parser),
            ModalPattern { state, fields_opt: None },
        );
        return Some((name_parser, pattern));
    }
    let tok = parser.tok();
    if tok.kind != TokenKind::Identifier {
        return None;
    }
    let next = parser.advance_or_eof();
    if !next.is_op("::") {
        spec_rule!("Parse-Pattern-Identifier");
        let pat = IdentifierPattern { name: tok.lexeme.clone(), name_splice_opt: None };
        return Some((next, make_pattern(tok.span.clone(), pat)));
    }
    spec_rule!("Parse-IfCase-Pattern-EnumHead");
    let (head_parser, head) = parse_qualified_head(parser.clone());
    let pat = EnumPattern { path: head.module_path, name: head.name, payload_opt: None };
    let pattern = make_pattern(span_between(&parser, &head_parser), pat);
    Some((head_parser, pattern))
}

fn parse_if_case_clause(parser: Parser) -> Parsed<IfCaseClause> {
    if parser.is_punct(":") {
        spec_rule!("Parse-If-Is-TypeTest");
        let (type_parser, ty) = parse_type(parser.advance_or_eof());
        let pat = TypedPattern { name: "_".to_string(), r#type: ty, name_splice_opt: None };
        let pattern = make_pattern(span_between(&parser, &type_parser), pat);
        let (body_parser, body) = parse_case_body_block(type_parser);
        return (body_parser, IfCaseClause { pattern, body });
    }
    let (full_parser, full_pattern) = parse_pattern(parser.clone_without_diags());
    if full_parser.is_punct("{") {
        let (body_parser, body) = parse_case_body_block(full_parser);
        let merged = merge_diag(&parser, &body_parser, &body_parser);
        return (merged, IfCaseClause { pattern: full_pattern, body });
    }
    if let Some((fallback_parser, pattern)) =
        try_parse_brace_disambiguated_if_case_pattern(parser.clone_without_diags())
            .filter(|(after, _)| after.is_punct("{"))
    {
        let (body_parser, body) = parse_case_body_block(fallback_parser);
        let merged = merge_diag(&parser, &body_parser, &body_parser);
        return (merged, IfCaseClause { pattern, body });
    }
    let mut merged = merge_diag(&parser, &full_parser, &full_parser);
    syntax_err_here(&mut merged);
    (merged, IfCaseClause::default())
}

struct IfCaseList {
    parser: Parser,
    cases: Vec<IfCaseClause>,
    else_opt: ExprPtr,
}

fn parse_if_case_list(parser: Parser) -> IfCaseList {
    let mut cur = parser;
    let mut cases = Vec::new();
    let mut else_opt = None;
    skip_newlines(&mut cur);
    while !cur.at_eof() && !cur.is_punct("}") {
        if cur.is_kw("else") {
            if cases.is_empty() {
                syntax_err_here(&mut cur);
            } else {
                spec_rule!("Parse-IfCasesTail-Else");
            }
            let after_else = cur.advance_or_eof();
            let (else_parser, else_block) = parse_block(after_else.clone());
            else_opt = block_expr(&after_else, &else_parser, else_block);
            cur = else_parser;
            skip_newlines(&mut cur);
            break;
        }
        if cases.is_empty() {
            spec_rule!("Parse-IfCases-Cons");
        } else {
            spec_rule!("Parse-IfCasesTail-Cons");
        }
        spec_rule!("Parse-IfCase");
        let (clause_parser, clause) = parse_if_case_clause(cur.clone());
        if clause_parser.index <= cur.index {
            let mut sync = cur.clone();
            sync_stmt(&mut sync);
            if sync.index <= cur.index {
                break;
            }
            cur = sync;
            skip_newlines(&mut cur);
            continue;
        }
        cases.push(clause);
        cur = clause_parser;
        skip_newlines(&mut cur);
    }
    if !cases.is_empty() && else_opt.is_none() && cur.is_punct("}") {
        spec_rule!("Parse-IfCasesTail-End");
    }
    IfCaseList { parser: cur, cases, else_opt }
}

fn parse_if_head_no_else(parser: Parser) -> (Parser, IfHead) {
    let start = parser.clone();
    let (first_parser, first) = parse_expr_no_brace(parser.advance_or_eof());
    if first_parser.is_kw("is") {
        let after_is = first_parser.advance_or_eof();
        if after_is.is_punct("{") {
            spec_rule!("Parse-If-Is-CaseList");
            let case_list = parse_if_case_list(after_is.advance_or_eof());
            if !case_list.parser.is_punct("}") {
                let (sync, error) = expr_error(case_list.parser, &start);
                return (sync, IfHead::Completed(error));
            }
            let after_rbrace = case_list.parser.advance_or_eof();
            if case_list.cases.is_empty() {
                // The reference reports this on a parser copy that it then discards.
                let mut discarded = after_is.clone();
                syntax_err_here(&mut discarded);
                let mut sync = after_rbrace;
                sync_stmt(&mut sync);
                let error = make_expr(span_between(&start, &sync), ErrorExpr {});
                return (sync, IfHead::Completed(error));
            }
            let if_case = IfCaseExpr {
                scrutinee: first,
                cases: case_list.cases,
                else_expr: case_list.else_opt,
            };
            let expr = make_expr(span_between(&start, &after_rbrace), if_case);
            return (after_rbrace, IfHead::Completed(expr));
        }
        spec_rule!("Parse-If-Is-Single");
        let (clause_parser, clause) = parse_if_case_clause(after_is);
        let arm = PendingIfArm {
            start,
            kind: PendingIfKind::IsSingle,
            cond_or_scrutinee: first,
            pattern_opt: clause.pattern,
            then_expr: clause.body,
        };
        return (clause_parser, IfHead::Pending(arm));
    }
    let (then_parser, then_block) = parse_block(first_parser.clone());
    let then_expr = block_expr(&first_parser, &then_parser, then_block);
    let arm = PendingIfArm {
        start,
        kind: PendingIfKind::Plain,
        cond_or_scrutinee: first,
        pattern_opt: None,
        then_expr,
    };
    (then_parser, IfHead::Pending(arm))
}

pub(crate) fn parse_if_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("ControlExpressionParsingRemainderFamily");
    spec_rule!("Parse-If-Expr");
    let (mut cur, head) = parse_if_head_no_else(parser);
    let mut chain = match head {
        IfHead::Completed(expr) => return (cur, expr),
        IfHead::Pending(arm) => vec![arm],
    };
    let mut else_opt = None;
    while cur.is_kw("else") {
        let after_else = cur.advance_or_eof();
        if !after_else.is_kw("if") {
            spec_rule!("Parse-ElseOpt-Block");
            let (block_parser, block) = parse_block(after_else.clone());
            else_opt = block_expr(&after_else, &block_parser, block);
            cur = block_parser;
            break;
        }
        spec_rule!("Parse-ElseOpt-If");
        let (next_parser, next_head) = parse_if_head_no_else(after_else);
        cur = next_parser;
        match next_head {
            IfHead::Completed(expr) => {
                else_opt = expr;
                break;
            }
            IfHead::Pending(arm) => chain.push(arm),
        }
    }
    if !cur.is_kw("else") {
        spec_rule!("Parse-ElseOpt-None");
    }
    let mut result = else_opt;
    for arm in chain.into_iter().rev() {
        let span = span_between(&arm.start, &cur);
        result = match arm.kind {
            PendingIfKind::Plain => make_expr(
                span,
                IfExpr { cond: arm.cond_or_scrutinee, then_expr: arm.then_expr, else_expr: result },
            ),
            PendingIfKind::IsSingle => make_expr(
                span,
                IfIsExpr {
                    scrutinee: arm.cond_or_scrutinee,
                    pattern: arm.pattern_opt,
                    then_expr: arm.then_expr,
                    else_expr: result,
                },
            ),
        };
    }
    (cur, result)
}

// ---- loops ------------------------------------------------------------------

pub(crate) fn parse_loop_invariant_opt(parser: Parser) -> Parsed<Option<LoopInvariant>> {
    spec_rule!("ParseLoopInvariantOpt");
    if !parser.is_op("|:") {
        spec_rule!("Parse-LoopInvariantOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-LoopInvariantOpt-Yes");
    let fail = |mut parser: Parser| {
        syntax_err_here(&mut parser);
        sync_stmt(&mut parser);
        (parser, None)
    };
    let next = parser.advance_or_eof();
    if !next.is_punct("{") {
        return fail(next);
    }
    let (after_pred, predicate) = parse_predicate_expr(next.advance_or_eof());
    if !after_pred.is_punct("}") {
        return fail(after_pred);
    }
    let after = after_pred.advance_or_eof();
    let span = span_between(&parser, &after);
    (after, Some(LoopInvariant { predicate, span }))
}

pub(crate) fn parse_loop_infinite_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-LoopTail-Infinite");
    let start = parser.clone();
    let (inv_parser, invariant_opt) = parse_loop_invariant_opt(parser);
    let (body_parser, body) = parse_block(inv_parser);
    let expr = make_expr(span_between(&start, &body_parser), LoopInfiniteExpr { invariant_opt, body });
    (body_parser, expr)
}

pub(crate) fn parse_loop_conditional_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-LoopTail-Cond");
    let start = parser.clone();
    let (cond_parser, cond) = parse_expr_no_brace(parser);
    let (inv_parser, invariant_opt) = parse_loop_invariant_opt(cond_parser);
    let (body_parser, body) = parse_block(inv_parser);
    let expr = make_expr(
        span_between(&start, &body_parser),
        LoopConditionalExpr { cond, invariant_opt, body },
    );
    (body_parser, expr)
}

/// Speculatively reads `pattern [: type] in`. On success the returned parser sits after
/// the pattern, carrying any diagnostics produced while reading it.
pub(crate) fn try_parse_pattern_in(parser: &Parser) -> Option<Parsed<PatternPtr>> {
    let (pat_parser, pattern) = parse_pattern(parser.clone_without_diags());
    let (ty_parser, _) = parse_type_annot_opt(pat_parser.clone());
    if ctx(&ty_parser.tok(), "in") {
        spec_rule!("TryParsePatternIn-Ok");
        return Some((merge_diag(parser, &ty_parser, &pat_parser), pattern));
    }
    spec_rule!("TryParsePatternIn-Fail");
    None
}

pub(crate) fn parse_loop_iter_expr(parser: Parser, try_in: Parsed<PatternPtr>) -> Parsed<ExprPtr> {
    spec_rule!("requirement.21.AsyncIterationSyntax");
    spec_rule!("Parse-LoopTail-Iter");
    spec_rule!("rule.21.Parse-LoopTail-Iter");
    let (pattern_parser, mut pattern) = try_in;
    let (ty_parser, mut type_opt) = parse_type_annot_opt(pattern_parser);
    if !ctx(&ty_parser.tok(), "in") {
        return expr_error(ty_parser, &parser);
    }
    let (iter_parser, iter) = parse_expr_no_brace(ty_parser.advance_or_eof());
    let (inv_parser, invariant_opt) = parse_loop_invariant_opt(iter_parser);
    let (body_parser, body) = parse_block(inv_parser);
    normalize_binding_pattern(&mut pattern, &mut type_opt);
    let expr = make_expr(
        span_between(&parser, &body_parser),
        LoopIterExpr { pattern, type_opt, iter, invariant_opt, body },
    );
    (body_parser, expr)
}

// ---- records, arrays, tuples -----------------------------------------------

fn parse_field_init(parser: Parser) -> Parsed<FieldInit> {
    let start = parser.clone();
    let (name_parser, name) = parse_ident(parser);
    if name_parser.is_punct(":") {
        spec_rule!("Parse-FieldInit-Explicit");
        let (expr_parser, value) = parse_expr(name_parser.advance_or_eof());
        let span = span_between(&start, &expr_parser);
        return (expr_parser, FieldInit { name, value, span });
    }
    spec_rule!("Parse-FieldInit-Shorthand");
    let span = span_between(&start, &name_parser);
    let value = make_expr(span.clone(), IdentifierExpr { name: name.clone(), ..Default::default() });
    (name_parser, FieldInit { name, value, span })
}

pub(crate) fn parse_field_init_list(mut parser: Parser) -> Parsed<Vec<FieldInit>> {
    spec_rule!("ConstructionListAndShorthandParsingFamily");
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-FieldInitList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-FieldInitList-Cons");
    let (mut parser, first) = parse_field_init(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct("}") {
            spec_rule!("Parse-FieldInitTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_punct("}") {
                if trailing_comma_allowed(&parser, &BRACE_END) {
                    spec_rule!("Parse-FieldInitTail-TrailingComma");
                }
                emit_trailing_comma_err(&mut parser, &BRACE_END);
                after.diags = parser.diags;
                return (after, xs);
            }
            spec_rule!("Parse-FieldInitTail-Comma");
            let (next, field) = parse_field_init(after);
            xs.push(field);
            parser = next;
            continue;
        }
        syntax_err_here(&mut parser);
        return (parser, xs);
    }
}

pub(crate) fn make_modal_ref(path: Path, generic_args: Vec<TypePtr>) -> ModalRef {
    if generic_args.is_empty() {
        ModalRef::Path(path)
    } else {
        ModalRef::GenericTypeRef(GenericTypeRef { path, generic_args })
    }
}

pub(crate) fn parse_record_literal(parser: Parser, allow_brace: bool) -> Parsed<ExprPtr> {
    let start = parser.clone();
    let (path_parser, path) = parse_type_path(parser);
    let (gen_parser, gen_args) = parse_generic_args_opt(path_parser);
    let has_args = gen_args.is_some();
    if gen_parser.is_op("@") {
        spec_rule!("Parse-Record-Literal-ModalState");
        let generic_args = gen_args.unwrap_or_default();
        let (state_parser, state) = parse_ident(gen_parser.advance_or_eof());
        if !state_parser.is_punct("{") {
            return expr_error(state_parser, &start);
        }
        let (fields_parser, fields) = parse_field_init_list(state_parser.advance_or_eof());
        if !fields_parser.is_punct("}") {
            return expr_error(fields_parser, &start);
        }
        let after = fields_parser.advance_or_eof();
        let modal = ModalStateRef {
            modal_ref: make_modal_ref(path.clone(), generic_args.clone()),
            path,
            generic_args,
            state,
        };
        let rec = RecordExpr { target: modal.into(), fields };
        let expr = make_expr(span_between(&start, &after), rec);
        return (after, expr);
    }
    if !allow_brace || !gen_parser.is_punct("{") || path.len() != 1 || has_args {
        return expr_error(gen_parser, &start);
    }
    spec_rule!("Parse-Record-Literal");
    let (mut fields_parser, fields) = parse_field_init_list(gen_parser.advance_or_eof());
    if fields.is_empty() && fields_parser.is_punct("}") {
        syntax_err_here(&mut fields_parser);
        let after = fields_parser.advance_or_eof();
        let expr = make_expr(span_between(&start, &after), ErrorExpr {});
        return (after, expr);
    }
    if !fields_parser.is_punct("}") {
        return expr_error(fields_parser, &start);
    }
    let after = fields_parser.advance_or_eof();
    let rec = RecordExpr { target: RecordExprTarget::Path(path), fields };
    let expr = make_expr(span_between(&start, &after), rec);
    (after, expr)
}

fn parse_array_segment(parser: Parser) -> Parsed<ArraySegment> {
    let (mut after_value, value) = parse_expr(parser);
    skip_newlines(&mut after_value);
    if after_value.is_punct(";") {
        spec_rule!("Parse-Array-Segment-Repeat");
        let (count_parser, count) = parse_expr(after_value.advance_or_eof());
        return (count_parser, ArrayRepeatSegment { value, count }.into());
    }
    spec_rule!("Parse-Array-Segment-Elem");
    (after_value, ArrayElemSegment { value }.into())
}

fn parse_array_segment_list(mut parser: Parser) -> Parsed<Vec<ArraySegment>> {
    skip_newlines(&mut parser);
    spec_rule!("List-Start");
    if parser.is_punct("]") {
        spec_rule!("List-Done");
        spec_rule!("Parse-Array-Segment-List-Empty");
        return (parser, Vec::new());
    }
    let mut elems = Vec::new();
    loop {
        spec_rule!("List-Cons");
        let (mut after_segment, segment) = parse_array_segment(parser);
        elems.push(segment);
        skip_newlines(&mut after_segment);
        if !after_segment.is_punct(",") {
            if after_segment.is_punct("]") {
                spec_rule!("List-Done");
            }
            spec_rule!("Parse-Array-Segment-List-Single");
            return (after_segment, elems);
        }
        spec_rule!("Parse-Array-Segment-List-Comma");
        let mut after_comma = after_segment.advance_or_eof();
        skip_newlines(&mut after_comma);
        if after_comma.is_punct("]") {
            emit_trailing_comma_err(&mut after_segment, &BRACKET_END);
            after_comma.diags = after_segment.diags;
            return (after_comma, elems);
        }
        parser = after_comma;
    }
}

pub(crate) fn parse_array_literal_expr(parser: Parser) -> Parsed<ExprPtr> {
    let start = parser.clone();
    let mut next = parser.advance_or_eof();
    skip_newlines(&mut next);
    let (segments_parser, elements) = parse_array_segment_list(next);
    if !segments_parser.is_punct("]") {
        return expr_error(segments_parser, &start);
    }
    spec_rule!("Parse-Array-Literal");
    let after = segments_parser.advance_or_eof();
    let expr = make_expr(span_between(&start, &after), ArrayExpr { elements });
    (after, expr)
}

fn parse_expr_list_tail(mut parser: Parser, mut elems: Vec<ExprPtr>) -> Parsed<Vec<ExprPtr>> {
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct(")") {
            spec_rule!("List-Done");
            spec_rule!("Parse-ExprListTail-End");
            return (parser, elems);
        }
        if !parser.is_punct(",") {
            syntax_err_here(&mut parser);
            return (parser, elems);
        }
        let mut after_comma = parser.advance_or_eof();
        skip_newlines(&mut after_comma);
        if after_comma.is_punct(")") {
            if trailing_comma_allowed(&parser, &PAREN_END) {
                spec_rule!("Parse-ExprListTail-TrailingComma");
            }
            emit_trailing_comma_err(&mut parser, &PAREN_END);
            after_comma.diags = parser.diags;
            return (after_comma, elems);
        }
        spec_rule!("Parse-ExprListTail-Comma");
        spec_rule!("List-Cons");
        let (next, elem) = parse_expr(after_comma);
        elems.push(elem);
        parser = next;
    }
}

pub(crate) fn parse_tuple_expr_elems(mut parser: Parser) -> Parsed<Vec<ExprPtr>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-TupleExprElems-Empty");
        return (parser, Vec::new());
    }
    let (mut first_parser, first) = parse_expr(parser);
    let mut after_first = first_parser.clone();
    skip_newlines(&mut after_first);
    if after_first.is_punct(";") {
        spec_rule!("Parse-TupleExprElems-Single");
        return (after_first.advance_or_eof(), vec![first]);
    }
    if after_first.is_punct(",") {
        let mut after_comma = after_first.advance_or_eof();
        skip_newlines(&mut after_comma);
        if after_comma.is_punct(")") {
            // The reference reports this on a parser copy that it then discards.
            let mut discarded = after_first.clone();
            syntax_err_here(&mut discarded);
            return (after_comma, vec![first]);
        }
        spec_rule!("Parse-TupleExprElems-Many");
        let (second_parser, second) = parse_expr(after_comma);
        spec_rule!("List-Start");
        spec_rule!("List-Cons");
        let (parser, tail) = parse_expr_list_tail(second_parser, vec![second]);
        let mut elems = Vec::with_capacity(1 + tail.len());
        elems.push(first);
        elems.extend(tail);
        return (parser, elems);
    }
    syntax_err_here(&mut first_parser);
    (first_parser, Vec::new())
}

// ---- wait, transmute, compile-time forms -----------------------------------

pub(crate) fn try_parse_wait_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    if !ctx(&parser.tok(), "wait") {
        return None;
    }
    let next = parser.advance_or_eof();
    if !is_expr_start(&next.tok()) {
        return None;
    }
    spec_rule!("Parse-Wait-Expr");
    let (handle_parser, handle) = parse_expr(next);
    let expr = make_expr(span_between(parser, &handle_parser), WaitExpr { handle });
    Some((handle_parser, expr))
}

pub(crate) fn try_parse_transmute_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    if !parser.is_kw("transmute") {
        return None;
    }
    spec_rule!("ParseTransmuteExprFamily");
    let next = parser.advance_or_eof();
    if !next.is_op("<") {
        return Some(expr_error(next, parser));
    }
    let (t1_parser, from) = parse_type(next.advance_or_eof());
    if !t1_parser.is_punct(",") {
        return Some(expr_error(t1_parser, parser));
    }
    let (t2_parser, to) = parse_type(t1_parser.advance_or_eof());
    let after_gt = if t2_parser.is_op(">>") {
        spec_rule!("Parse-Transmute-Expr-ShiftSplit");
        let at_second = split_shift_r(&t2_parser).advance_or_eof();
        if !at_second.is_op(">") {
            return Some(expr_error(at_second, parser));
        }
        at_second.advance_or_eof()
    } else if t2_parser.is_op(">") {
        spec_rule!("Parse-Transmute-Expr");
        t2_parser.advance_or_eof()
    } else {
        return Some(expr_error(t2_parser, parser));
    };
    if !after_gt.is_punct("(") {
        return Some(expr_error(after_gt, parser));
    }
    let (expr_parser, value) = parse_expr(after_gt.advance_or_eof());
    if !expr_parser.is_punct(")") {
        return Some(expr_error(expr_parser, parser));
    }
    let after = expr_parser.advance_or_eof();
    let expr = make_expr(span_between(parser, &after), TransmuteExpr { from, to, value });
    Some((after, expr))
}

fn make_else_if_block(expr: ExprPtr, span: Span) -> BlockPtr {
    Some(Arc::new(Block { stmts: Vec::new(), tail_opt: expr, span }))
}

fn parse_ct_else_opt(parser: Parser) -> Parsed<BlockPtr> {
    if !parser.is_kw("else") {
        spec_rule!("Parse-CtElseOpt-None");
        return (parser, None);
    }
    let after_else = parser.advance_or_eof();
    if after_else.is_kw("comptime") && after_else.advance_or_eof().is_kw("if") {
        spec_rule!("Parse-CtElseOpt-ElseIf");
        return match try_parse_comptime_expr(&after_else) {
            None => {
                let mut sync = after_else;
                syntax_err_here(&mut sync);
                sync_stmt(&mut sync);
                let span = span_between(&parser, &sync);
                let block = make_else_if_block(make_expr(span.clone(), ErrorExpr {}), span);
                (sync, block)
            }
            Some((nested_parser, nested)) => {
                let span = nested
                    .as_ref()
                    .map_or_else(|| span_between(&parser, &nested_parser), |e| e.span.clone());
                (nested_parser, make_else_if_block(nested, span))
            }
        };
    }
    spec_rule!("Parse-CtElseOpt-Block");
    parse_block(after_else)
}

fn parse_ct_block_expr(parser: &Parser) -> Parsed<ExprPtr> {
    let next = parser.advance_or_eof();
    if !next.is_punct("{") {
        return expr_error(next, parser);
    }
    let mut after_l = next.advance_or_eof();
    skip_newlines(&mut after_l);
    let (mut before_r, body) = parse_expr(after_l);
    skip_newlines(&mut before_r);
    if !before_r.is_punct("}") {
        return expr_error(before_r, parser);
    }
    let after_r = before_r.advance_or_eof();
    spec_rule!("Parse-CtExpr");
    let expr = make_expr(span_between(parser, &after_r), ComptimeExpr { body, attrs_opt: None });
    (after_r, expr)
}

fn parse_ct_if_expr(parser: &Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-CtIf");
    let after_if = parser.advance_or_eof().advance_or_eof();
    let (cond_parser, cond) = parse_expr_no_brace(after_if);
    let (then_parser, then_block) = parse_block(cond_parser);
    let (else_parser, else_block_opt) = parse_ct_else_opt(then_parser);
    let expr = make_expr(
        span_between(parser, &else_parser),
        CtIfExpr { cond, then_block, else_block_opt },
    );
    (else_parser, expr)
}

fn parse_ct_loop_iter_expr(parser: &Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-CtLoopIter");
    let after_loop = parser.advance_or_eof().advance_or_eof();
    let Some((pattern_parser, mut pattern)) = try_parse_pattern_in(&after_loop) else {
        return expr_error(after_loop, parser);
    };
    let (ty_parser, mut type_opt) = parse_type_annot_opt(pattern_parser);
    if !ctx(&ty_parser.tok(), "in") {
        return expr_error(ty_parser, parser);
    }
    let (iter_parser, iter) = parse_expr_no_brace(ty_parser.advance_or_eof());
    let (body_parser, body) = parse_block(iter_parser);
    normalize_binding_pattern(&mut pattern, &mut type_opt);
    let expr = make_expr(
        span_between(parser, &body_parser),
        CtLoopIterExpr { pattern, type_opt, iter, body },
    );
    (body_parser, expr)
}

pub(crate) fn try_parse_comptime_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    let tok = parser.tok();
    let is_comptime = matches!(tok.kind, TokenKind::Keyword | TokenKind::Identifier)
        && tok.lexeme == "comptime";
    if !is_comptime {
        return None;
    }
    let next = parser.advance_or_eof();
    if next.is_punct("{") {
        return Some(parse_ct_block_expr(parser));
    }
    if next.is_kw("if") {
        return Some(parse_ct_if_expr(parser));
    }
    if next.is_kw("loop") {
        return Some(parse_ct_loop_iter_expr(parser));
    }
    None
}

/// `Type::<T>`.
pub(crate) fn try_parse_type_literal_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    let tok = parser.tok();
    if tok.kind != TokenKind::Identifier || tok.lexeme != "Type" {
        return None;
    }
    let mut cur = parser.advance_or_eof();
    if !cur.is_op("::") {
        return None;
    }
    cur.advance();
    if !cur.is_op("<") {
        return Some(expr_error(cur, parser));
    }
    let (mut cur, ty) = parse_type(cur.advance_or_eof());
    if cur.is_op(">>") {
        cur = split_shift_r(&cur);
    }
    if !cur.is_op(">") {
        return Some(expr_error(cur, parser));
    }
    cur.advance();
    spec_rule!("Parse-TypeLiteral");
    let expr = make_expr(span_between(parser, &cur), TypeLiteralExpr { r#type: ty });
    Some((cur, expr))
}

/// Advances to the `}` that closes the quoted body; false when it is never closed.
fn scan_quoted_body(parser: &mut Parser) -> bool {
    let mut brace_depth = 0usize;
    while !parser.at_eof() {
        if parser.is_punct("{") {
            brace_depth += 1;
        } else if parser.is_punct("}") {
            if brace_depth == 0 {
                return true;
            }
            brace_depth -= 1;
        }
        parser.advance();
    }
    false
}

pub(crate) fn try_parse_quote_expr(parser: &Parser) -> Option<Parsed<ExprPtr>> {
    let tok = parser.tok();
    let is_quote_head = matches!(tok.kind, TokenKind::Keyword | TokenKind::Identifier)
        && tok.lexeme == "quote";
    if !is_quote_head {
        return None;
    }
    let mut cur = parser.advance_or_eof();
    let mut kind = QuoteKind::Unspecified;
    let next: std::rc::Rc<Token> = cur.tok();
    if cur.is_kw("type") {
        kind = QuoteKind::Type;
        cur.advance();
    } else if matches!(next.kind, TokenKind::Identifier | TokenKind::Keyword) && next.lexeme == "pattern" {
        kind = QuoteKind::Pattern;
        cur.advance();
    }
    if !cur.is_punct("{") {
        return Some(expr_error(cur, parser));
    }
    cur.advance();
    let mut content_end = cur.clone_without_diags();
    if !scan_quoted_body(&mut content_end) || !content_end.is_punct("}") {
        return Some(expr_error(cur, parser));
    }
    let mut tokens = Vec::with_capacity(content_end.index.saturating_sub(cur.index));
    let mut walk = cur.clone_without_diags();
    while walk.index < content_end.index && !walk.at_eof() {
        tokens.push((*walk.tok()).clone());
        walk.advance();
    }
    match kind {
        QuoteKind::Type => spec_rule!("Parse-Quote-Type"),
        QuoteKind::Pattern => spec_rule!("Parse-Quote-Pattern"),
        _ => spec_rule!("Parse-Quote-Raw"),
    }
    content_end.advance();
    let merged = merge_diag(parser, &content_end, &content_end);
    let expr = make_expr(span_between(parser, &merged), QuoteExpr { kind, tokens });
    Some((merged, expr))
}

// ---- race, all --------------------------------------------------------------

fn rule_21(rule: &str) {
    spec_rule!(rule);
    spec_rule!(format!("rule.21.{rule}").as_str());
}

fn parse_race_arm(parser: Parser) -> Parsed<RaceArm> {
    rule_21("Parse-RaceArm");
    let fail = |mut parser: Parser| {
        syntax_err_here(&mut parser);
        sync_stmt(&mut parser);
        (parser, RaceArm::default())
    };
    let is_bar = |parser: &Parser| parser.is_op("|") || parser.is_punct("|");
    let (expr_parser, expr) = parse_expr(parser);
    if !expr_parser.is_op("->") {
        return fail(expr_parser);
    }
    let after_arrow = expr_parser.advance_or_eof();
    if !is_bar(&after_arrow) {
        return fail(after_arrow);
    }
    let (pat_parser, pattern) = parse_pattern(after_arrow.advance_or_eof());
    if !is_bar(&pat_parser) {
        return fail(pat_parser);
    }
    let after_bar = pat_parser.advance_or_eof();
    let (handler_parser, handler) = if after_bar.is_kw("yield") {
        rule_21("Parse-RaceHandler-Yield");
        let (parser, value) = parse_expr(after_bar.advance_or_eof());
        (parser, RaceHandler { kind: RaceHandlerKind::Yield, value })
    } else {
        rule_21("Parse-RaceHandler-Return");
        let (parser, value) = parse_expr(after_bar);
        (parser, RaceHandler { kind: RaceHandlerKind::Return, value })
    };
    (handler_parser, RaceArm { expr, pattern, handler })
}

/// A brace-delimited, comma-separated list with at least one element; a trailing comma
/// is accepted without a diagnostic.
fn parse_composition_list<T>(
    mut parser: Parser,
    family: &str,
    parse_elem: fn(Parser) -> Parsed<T>,
) -> Parsed<Vec<T>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        syntax_err_here(&mut parser);
        return (parser, Vec::new());
    }
    rule_21(&format!("Parse-{family}-Cons"));
    let (mut parser, first) = parse_elem(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct("}") {
            rule_21(&format!("Parse-{family}Tail-End"));
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_punct("}") {
                rule_21(&format!("Parse-{family}Tail-TrailingComma"));
                return (after, xs);
            }
            rule_21(&format!("Parse-{family}Tail-Comma"));
            let (next, elem) = parse_elem(after);
            xs.push(elem);
            parser = next;
            continue;
        }
        syntax_err_here(&mut parser);
        return (parser, xs);
    }
}

fn parse_composition_expr<T>(
    parser: Parser,
    rule: &str,
    family: &str,
    parse_elem: fn(Parser) -> Parsed<T>,
    build: fn(Vec<T>) -> ExprNode,
) -> Parsed<ExprPtr> {
    rule_21(rule);
    let next = parser.advance_or_eof();
    if !next.is_punct("{") {
        return expr_error(next, &parser);
    }
    let (elems_parser, elems) = parse_composition_list(next.advance_or_eof(), family, parse_elem);
    if elems.is_empty() && elems_parser.is_punct("}") {
        let after_r = elems_parser.advance_or_eof();
        let expr = make_expr(span_between(&parser, &after_r), ErrorExpr {});
        return (after_r, expr);
    }
    if !elems_parser.is_punct("}") {
        return expr_error(elems_parser, &parser);
    }
    let after_r = elems_parser.advance_or_eof();
    let expr = make_expr(span_between(&parser, &after_r), build(elems));
    (after_r, expr)
}

pub(crate) fn parse_race_expr(parser: Parser) -> Parsed<ExprPtr> {
    parse_composition_expr(parser, "Parse-Race-Expr", "RaceArms", parse_race_arm, |arms| {
        ExprNode::RaceExpr(RaceExpr { arms })
    })
}

pub(crate) fn parse_all_expr(parser: Parser) -> Parsed<ExprPtr> {
    parse_composition_expr(parser, "Parse-All-Expr", "AllExprList", parse_expr, |exprs| {
        ExprNode::AllExpr(AllExpr { exprs })
    })
}

// ---- spawn, parallel, dispatch ---------------------------------------------

/// `[opt, opt, ...]` before a block. An empty list is an error; on a malformed list the
/// parser skips to the closing bracket or the block.
fn parse_opts_opt<T>(parser: Parser, family: &str, parse_opt: fn(Parser) -> Parsed<T>) -> Parsed<Vec<T>> {
    if !parser.is_punct("[") {
        spec_rule!(format!("Parse-{family}OptsOpt-None").as_str());
        return (parser, Vec::new());
    }
    spec_rule!(format!("Parse-{family}OptsOpt-Yes").as_str());
    let mut cur = parser.advance_or_eof();
    skip_newlines(&mut cur);
    let mut opts = Vec::new();
    if cur.is_punct("]") {
        spec_rule!(format!("Parse-{family}OptList-Empty").as_str());
        syntax_err_here(&mut cur);
    } else {
        spec_rule!(format!("Parse-{family}OptList-Cons").as_str());
        let (next, first) = parse_opt(cur);
        opts.push(first);
        cur = next;
        loop {
            skip_newlines(&mut cur);
            if cur.is_punct("]") {
                spec_rule!(format!("Parse-{family}OptListTail-End").as_str());
                break;
            }
            if !cur.is_punct(",") {
                syntax_err_here(&mut cur);
                break;
            }
            let mut after_comma = cur.advance_or_eof();
            skip_newlines(&mut after_comma);
            if after_comma.is_punct("]") {
                if trailing_comma_allowed(&cur, &BRACKET_END) {
                    spec_rule!(format!("Parse-{family}OptListTail-TrailingComma").as_str());
                }
                emit_trailing_comma_err(&mut cur, &BRACKET_END);
                after_comma.diags = cur.diags;
                cur = after_comma;
                break;
            }
            spec_rule!(format!("Parse-{family}OptListTail-Comma").as_str());
            let (next, opt) = parse_opt(after_comma);
            opts.push(opt);
            cur = next;
        }
    }
    if !cur.is_punct("]") {
        syntax_err_here(&mut cur);
        while !cur.at_eof() && !cur.is_punct("]") && !cur.is_punct("{") {
            cur.advance();
        }
        if cur.is_punct("]") {
            cur.advance();
        }
        return (cur, opts);
    }
    (cur.advance_or_eof(), opts)
}

fn parse_spawn_opt(mut parser: Parser) -> Parsed<SpawnOption> {
    let tok = parser.tok();
    let opt_start = parser.tok_span();
    let kind = match (tok.kind, tok.lexeme.as_str()) {
        (TokenKind::Identifier, "name") => {
            spec_rule!("Parse-SpawnOpt-Name");
            SpawnOptionKind::Name
        }
        (TokenKind::Identifier, "affinity") => {
            spec_rule!("Parse-SpawnOpt-Affinity");
            SpawnOptionKind::Affinity
        }
        (TokenKind::Identifier, "priority") => {
            spec_rule!("Parse-SpawnOpt-Priority");
            SpawnOptionKind::Priority
        }
        _ => {
            syntax_err_here(&mut parser);
            return (parser, SpawnOption { kind: SpawnOptionKind::Name, value: None, span: opt_start });
        }
    };
    let mut cur = parser.advance_or_eof();
    let fail = |mut cur: Parser, opt_start: &Span| {
        syntax_err_here(&mut cur);
        let span = span_cover(opt_start, &cur.tok_span());
        (cur, SpawnOption { kind, value: None, span })
    };
    if !cur.is_punct(":") {
        return fail(cur, &opt_start);
    }
    cur.advance();
    let (value_parser, value) = if kind == SpawnOptionKind::Name {
        if cur.tok().kind != TokenKind::StringLiteral {
            return fail(cur, &opt_start);
        }
        parse_literal_expr(cur)
    } else {
        parse_expr(cur)
    };
    let span = span_cover(&opt_start, &value_parser.tok_span());
    (value_parser, SpawnOption { kind, value, span })
}

pub(crate) fn parse_spawn_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Spawn-Expr");
    let (opts_parser, opts) = parse_opts_opt(parser.advance_or_eof(), "Spawn", parse_spawn_opt);
    let (body_parser, body) = parse_block(opts_parser);
    let span = span_between(&parser, &body_parser);
    if Conformance::enabled() {
        Conformance::record_at(
            "requirement.20.CaptureSemanticsNoAdditionalSyntax",
            Some(&span),
            "source=ParseSpawnExpr;body_parser=ParseBlock;capture_surface=spawn_body;additional_syntax=false",
        );
        Conformance::record_at(
            "requirement.20.CaptureSemanticsNoAdditionalParsingRules",
            Some(&span),
            "source=ParseSpawnExpr;parser=spawn_expr;body_parser=ParseBlock;capture_parser=none;additional_parsing_rules=false",
        );
    }
    (body_parser, make_expr(span, SpawnExpr { opts, body }))
}

/// `(a, b, c)`: exactly three expressions.
fn parse_dim3_const(mut parser: Parser) -> Parsed<ExprPtr> {
    if !parser.is_punct("(") {
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span.clone());
        return (parser, make_expr(span, ErrorExpr {}));
    }
    let start = parser.tok_span();
    let fail = |mut cur: Parser, start: &Span| {
        syntax_err_here(&mut cur);
        let expr = make_expr(span_cover(start, &cur.tok_span()), ErrorExpr {});
        (cur, expr)
    };
    let mut cur = parser.advance_or_eof();
    skip_newlines(&mut cur);
    let mut elements = Vec::with_capacity(3);
    for i in 0..3 {
        let (next, elem) = parse_expr(cur);
        elements.push(elem);
        cur = next;
        skip_newlines(&mut cur);
        if i < 2 {
            if !cur.is_punct(",") {
                return fail(cur, &start);
            }
            cur.advance();
            skip_newlines(&mut cur);
        }
    }
    if !cur.is_punct(")") {
        return fail(cur, &start);
    }
    let span = span_cover(&start, &cur.tok_span());
    (cur.advance_or_eof(), make_expr(span, TupleExpr { elements }))
}

fn parse_parallel_opt(mut parser: Parser) -> Parsed<ParallelOption> {
    let tok = parser.tok();
    let opt_start = parser.tok_span();
    let named = matches!(tok.kind, TokenKind::Identifier | TokenKind::Keyword);
    let kind = match tok.lexeme.as_str() {
        "cancel" if named => {
            spec_rule!("Parse-ParallelOpt-Cancel");
            ParallelOptionKind::Cancel
        }
        "name" if named => {
            spec_rule!("Parse-ParallelOpt-Name");
            ParallelOptionKind::Name
        }
        "workgroup" if named => {
            spec_rule!("Parse-ParallelOpt-Workgroup");
            ParallelOptionKind::Workgroup
        }
        "workgroups" if named => {
            spec_rule!("Parse-ParallelOpt-Workgroups");
            ParallelOptionKind::Workgroups
        }
        _ => {
            syntax_err_here(&mut parser);
            let err = ParallelOption { kind: ParallelOptionKind::Cancel, value: None, span: opt_start };
            return (parser, err);
        }
    };
    let fail = |mut cur: Parser, opt_start: &Span| {
        syntax_err_here(&mut cur);
        let span = span_cover(opt_start, &cur.tok_span());
        (cur, ParallelOption { kind, value: None, span })
    };
    let mut cur = parser.advance_or_eof();
    if !cur.is_punct(":") {
        return fail(cur, &opt_start);
    }
    cur.advance();
    skip_newlines(&mut cur);
    let (value_parser, value) = match kind {
        ParallelOptionKind::Name => {
            if cur.tok().kind != TokenKind::StringLiteral {
                return fail(cur, &opt_start);
            }
            parse_literal_expr(cur)
        }
        ParallelOptionKind::Workgroup | ParallelOptionKind::Workgroups => parse_dim3_const(cur),
        ParallelOptionKind::Cancel => parse_expr(cur),
    };
    let span = span_cover(&opt_start, &value_parser.tok_span());
    (value_parser, ParallelOption { kind, value, span })
}

fn record_parallel_expression_parser_evidence(par: &ParallelExpr, span: &Span) {
    if !Conformance::enabled() {
        return;
    }
    for opt in par.opts.iter().filter(|opt| opt.kind == ParallelOptionKind::Cancel) {
        Conformance::record_at(
            "requirement.20.CancellationSyntax",
            Some(&opt.span),
            "source=ParseParallelOpt;option=cancel;surface=parallel_option;ordinary_cancel_token_call_syntax=true;additional_syntax=false",
        );
        Conformance::record_at(
            "requirement.20.CancellationNoAdditionalParsingRules",
            Some(&opt.span),
            "source=ParseParallelOpt;option=cancel;parser=parallel_option;additional_parsing_rules=false",
        );
    }
    const RECORDS: [(&str, &str); 6] = [
        ("requirement.20.ExecutionDomainSyntax", "source=ParseParallelExpr;domain_parser=ParseExprNoBrace;method_call_syntax=ordinary_postfix_method_call;call_syntax=ordinary_postfix_call;additional_domain_syntax=false"),
        ("requirement.20.ExecutionDomainsNoAdditionalParsingProductions", "source=ParseParallelExpr;domain_parser=ParseExprNoBrace;parallel_option_boundary=stop_before_parallel_options;additional_domain_productions=false"),
        ("requirement.20.DeterminismNestingNoAdditionalSyntax", "source=ParseParallelExpr;surface=parallel_spawn_dispatch;determinism_nesting_syntax=none;additional_syntax=false"),
        ("requirement.20.DeterminismNestingNoAdditionalParsingRules", "source=ParseParallelExpr;parser=parallel_expr;nesting=recursive_block_parse;additional_parsing_rules=false"),
        ("requirement.20.PanicHandlingNoAdditionalSyntax", "source=ParseParallelExpr;panic_surface=none;additional_syntax=false"),
        ("requirement.20.PanicHandlingNoAdditionalParsingRules", "source=ParseParallelExpr;panic_parser=none;additional_parsing_rules=false"),
    ];
    for (rule, payload) in RECORDS {
        Conformance::record_at(rule, Some(span), payload);
    }
}

pub(crate) fn parse_parallel_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Parallel-Expr");
    let mut domain_start = parser.advance_or_eof();
    domain_start.stop_before_parallel_options = true;
    let (mut after_domain, domain) = parse_expr_no_brace(domain_start);
    after_domain.stop_before_parallel_options = false;
    let (opts_parser, opts) = parse_opts_opt(after_domain, "Parallel", parse_parallel_opt);
    let (body_parser, body) = parse_block(opts_parser);
    let par = ParallelExpr { domain, opts, body };
    let span = span_between(&parser, &body_parser);
    record_parallel_expression_parser_evidence(&par, &span);
    (body_parser, make_expr(span, par))
}

pub(crate) fn parse_key_mode(mut parser: Parser) -> Parsed<KeyMode> {
    let tok = parser.tok();
    if tok.kind == TokenKind::Identifier {
        if tok.lexeme == "read" {
            spec_rule!("Parse-KeyMode-Read");
            return (parser.advance_or_eof(), KeyMode::Read);
        }
        if tok.lexeme == "write" {
            spec_rule!("Parse-KeyMode-Write");
            return (parser.advance_or_eof(), KeyMode::Write);
        }
    }
    spec_rule!("Parse-KeyMode-Err");
    syntax_err_here(&mut parser);
    (parser, KeyMode::Read)
}

fn parse_key_clause_opt(parser: Parser) -> Parsed<Option<DispatchKeyClause>> {
    if !ctx(&parser.tok(), "key") {
        spec_rule!("Parse-KeyClauseOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-KeyClauseOpt-Yes");
    let key_start = parser.tok_span();
    let (path_parser, key_path) = parse_key_path_expr(parser.advance_or_eof());
    let (mode_parser, mode) = parse_key_mode(path_parser);
    let span = span_cover(&key_start, &mode_parser.tok_span());
    (mode_parser, Some(DispatchKeyClause { key_path, mode, span }))
}

fn parse_dispatch_opt(mut parser: Parser) -> Parsed<DispatchOption> {
    let tok = parser.tok();
    let opt_start = parser.tok_span();
    let mut opt = DispatchOption { span: opt_start.clone(), ..Default::default() };
    if tok.kind != TokenKind::Identifier {
        syntax_err_here(&mut parser);
        opt.kind = DispatchOptionKind::Ordered;
        return (parser, opt);
    }
    // `name:` prefix shared by the valued options.
    let colon = |parser: &Parser| -> Result<Parser, Parser> {
        let cur = parser.advance_or_eof();
        if cur.is_punct(":") {
            Ok(cur.advance_or_eof())
        } else {
            Err(cur)
        }
    };
    let finish = |mut opt: DispatchOption, cur: Parser| {
        opt.span = span_cover(&opt_start, &cur.tok_span());
        (cur, opt)
    };
    let missing_colon = |opt: DispatchOption, mut cur: Parser| {
        syntax_err_here(&mut cur);
        finish(opt, cur)
    };
    match tok.lexeme.as_str() {
        "reduce" => {
            spec_rule!("Parse-DispatchOpt-Reduce");
            opt.kind = DispatchOptionKind::Reduce;
            let mut cur = match colon(&parser) {
                Ok(cur) => cur,
                Err(cur) => return missing_colon(opt, cur),
            };
            let op_tok = cur.tok();
            match (op_tok.kind, op_tok.lexeme.as_str()) {
                (TokenKind::Operator, "+" | "*") => {
                    spec_rule!("Parse-ReduceOp-Op");
                    opt.reduce_op = if op_tok.lexeme == "+" { ReduceOp::Add } else { ReduceOp::Mul };
                    cur.advance();
                }
                (TokenKind::Identifier, name) => {
                    spec_rule!("Parse-ReduceOp-Ident");
                    opt.reduce_op = match name {
                        "min" => ReduceOp::Min,
                        "max" => ReduceOp::Max,
                        "and" => ReduceOp::And,
                        "or" => ReduceOp::Or,
                        _ => {
                            opt.custom_reduce_name = name.to_string();
                            ReduceOp::Custom
                        }
                    };
                    cur.advance();
                }
                _ => {
                    syntax_err_here(&mut cur);
                    opt.reduce_op = ReduceOp::Add;
                }
            }
            finish(opt, cur)
        }
        "ordered" => {
            spec_rule!("Parse-DispatchOpt-Ordered");
            opt.kind = DispatchOptionKind::Ordered;
            finish(opt, parser.advance_or_eof())
        }
        "chunk" => {
            spec_rule!("Parse-DispatchOpt-Chunk");
            opt.kind = DispatchOptionKind::Chunk;
            match colon(&parser) {
                Err(cur) => missing_colon(opt, cur),
                Ok(cur) => {
                    let (cur, chunk_expr) = parse_expr(cur);
                    opt.chunk_expr = chunk_expr;
                    finish(opt, cur)
                }
            }
        }
        "workgroup" => {
            spec_rule!("Parse-DispatchOpt-Workgroup");
            opt.kind = DispatchOptionKind::Workgroup;
            match colon(&parser) {
                Err(cur) => missing_colon(opt, cur),
                Ok(cur) => {
                    let (cur, dims) = parse_dim3_const(cur);
                    opt.workgroup_expr = dims;
                    finish(opt, cur)
                }
            }
        }
        _ => {
            syntax_err_here(&mut parser);
            opt.kind = DispatchOptionKind::Ordered;
            (parser, opt)
        }
    }
}

pub(crate) fn parse_dispatch_expr(parser: Parser) -> Parsed<ExprPtr> {
    spec_rule!("Parse-Dispatch-Expr");
    let (pat_parser, pattern) = parse_pattern(parser.advance_or_eof());
    if !ctx(&pat_parser.tok(), "in") {
        return expr_error(pat_parser, &parser);
    }
    let (range_parser, range) = parse_range(pat_parser.advance_or_eof(), false, false);
    let (key_parser, key_clause) = parse_key_clause_opt(range_parser);
    let (opts_parser, opts) = parse_opts_opt(key_parser, "Dispatch", parse_dispatch_opt);
    let (body_parser, body) = parse_block(opts_parser);
    let span = span_between(&parser, &body_parser);
    if Conformance::enabled() {
        Conformance::record_at(
            "requirement.20.CaptureSemanticsNoAdditionalSyntax",
            Some(&span),
            "source=ParseDispatchExpr;body_parser=ParseBlock;capture_surface=dispatch_body;additional_syntax=false",
        );
        Conformance::record_at(
            "requirement.20.CaptureSemanticsNoAdditionalParsingRules",
            Some(&span),
            "source=ParseDispatchExpr;parser=dispatch_expr;body_parser=ParseBlock;capture_parser=none;additional_parsing_rules=false",
        );
    }
    let dispatch = DispatchExpr { pattern, range, key_clause, opts, body };
    (body_parser, make_expr(span, dispatch))
}
