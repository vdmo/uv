//! Declarations: procedures, records, enums, modals, classes, type aliases, `using`,
//! `import`, `extern` blocks and derive targets.

use std::sync::Arc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use super::attributes::parse_attribute_list_opt;
use super::generics::parse_generic_params_opt;
use super::signature::{
    expect_punct, parse_method_signature, parse_param_list, parse_signature,
    parse_state_method_signature, MethodSignature,
};
use super::{error_item, is_lexeme_token};
use crate::ast::*;
use crate::lexer::TokenKind;
use crate::parser::consume::{emit_trailing_comma_err, match_punct, trailing_comma_allowed, TokenMatch};
use crate::parser::expr::{parse_expr, parse_predicate_expr, skip_newlines};
use crate::parser::paths::{
    parse_alias_opt, parse_class_path, parse_ident, parse_key_boundary_opt, parse_module_path,
    parse_vis,
};
use crate::parser::recovery::{
    consume_terminator_opt, consume_terminator_req, emit_parse_syntax_err, sync_item, sync_stmt,
    TerminatorPolicy,
};
use crate::parser::state::{span_between, Parsed, Parser};
use crate::parser::stmt::parse_block;
use crate::parser::types::{parse_type, parse_type_list_tail};

const BRACE_END: [TokenMatch; 1] = [match_punct("}")];
const OPEN_BRACE_END: [TokenMatch; 1] = [match_punct("{")];
const TYPE_LIST_END: [TokenMatch; 2] = [match_punct(")"), match_punct("}")];

fn syntax_err_here(parser: &mut Parser) {
    let span = parser.tok_span();
    emit_parse_syntax_err(parser, span);
}

fn expect_op(parser: &mut Parser, op: &str) {
    if parser.is_op(op) {
        parser.advance();
    } else {
        syntax_err_here(parser);
    }
}

fn expect_kw(parser: &mut Parser, keyword: &str) {
    if parser.is_kw(keyword) {
        parser.advance();
    } else {
        syntax_err_here(parser);
    }
}

/// Leading attributes and visibility of a member, with newlines allowed between them.
fn parse_member_prefix(parser: Parser) -> (Parser, Visibility, AttributeList) {
    let (mut after_attrs, attrs) = parse_attribute_list_opt(parser);
    skip_newlines(&mut after_attrs);
    let (vis_parser, vis) = parse_vis(after_attrs);
    (vis_parser, vis, attrs.unwrap_or_default())
}

fn record_generic_owner_clause(owner: &str, name: &str, params: &Option<GenericParams>, span: &Span) {
    if !Conformance::enabled() {
        return;
    }
    let names: Vec<&str> = params
        .as_ref()
        .map(|p| p.params.iter().map(|param| param.name.as_str()).collect())
        .unwrap_or_default();
    Conformance::record_at(
        "GenericParamsOnOwnerDecl",
        Some(span),
        &format!(
            "owner={owner};name={name};generic_params={};param_count={};param_names={}",
            if params.is_some() { "some" } else { "none" },
            names.len(),
            names.join(",")
        ),
    );
}

fn record_nominal_relation(owner: &str, name: &str, relation: &str, relations: &[Path], span: &Span) {
    if !Conformance::enabled() {
        return;
    }
    let paths: Vec<String> = relations.iter().map(|path| path.join("::")).collect();
    Conformance::record_at(
        "NominalRelationFormOnOwnerDecl",
        Some(span),
        &format!(
            "owner={owner};name={name};relation={relation};count={};paths={}",
            relations.len(),
            paths.join(",")
        ),
    );
}

// ---- contracts --------------------------------------------------------------

/// `|: @foreign_assumes` / `|: @foreign_ensures`.
fn is_foreign_contract_start(parser: &Parser) -> bool {
    if !parser.is_op("|:") {
        return false;
    }
    let after_bar = parser.advance_or_eof();
    if !after_bar.is_op("@") {
        return false;
    }
    let tok = after_bar.advance_or_eof().tok();
    tok.kind == TokenKind::Identifier
        && (tok.lexeme == "foreign_assumes" || tok.lexeme == "foreign_ensures")
}

pub(crate) fn parse_contract_clause_opt(parser: Parser) -> Parsed<Option<ContractClause>> {
    let mut probe = parser.clone();
    skip_newlines(&mut probe);
    if !probe.is_op("|:") || is_foreign_contract_start(&probe) {
        spec_rule!("Parse-ContractClauseOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-ContractClauseOpt-Yes");
    spec_rule!("Parse-Contract-Clause");
    let start = probe.clone();
    let next = probe.advance_or_eof();
    if next.is_op("|=") {
        spec_rule!("Parse-ContractBody-PostOnly");
        let (post_parser, postcondition) = parse_predicate_expr(next.advance_or_eof());
        let span = span_between(&start, &post_parser);
        return (post_parser, Some(ContractClause { precondition: None, postcondition, span }));
    }
    let mut pre_parser = next;
    if !pre_parser.is_punct("{") {
        pre_parser.stop_before_contract_post_separator = true;
    }
    let (mut next, precondition) = parse_predicate_expr(pre_parser);
    next.stop_before_contract_post_separator = false;
    let mut postcondition = None;
    if next.is_op("|=") {
        spec_rule!("Parse-ContractBody-PrePost");
        let (post_parser, post) = parse_predicate_expr(next.advance_or_eof());
        postcondition = post;
        next = post_parser;
    } else {
        spec_rule!("Parse-ContractBody-PreOnly");
    }
    let span = span_between(&start, &next);
    (next, Some(ContractClause { precondition, postcondition, span }))
}

fn parse_foreign_contract_clause(mut parser: Parser) -> Parsed<ForeignContractClause> {
    skip_newlines(&mut parser);
    let start = parser.clone();
    let mut clause = ForeignContractClause::default();
    if !parser.is_op("|:") {
        clause.span = parser.tok_span();
        syntax_err_here(&mut parser);
        return (parser, clause);
    }
    let fail = |mut parser: Parser, mut clause: ForeignContractClause| {
        syntax_err_here(&mut parser);
        clause.span = span_between(&start, &parser);
        (parser, clause)
    };
    parser.advance();
    if !parser.is_op("@") {
        return fail(parser, clause);
    }
    parser.advance();
    let head = parser.tok();
    if head.kind != TokenKind::Identifier {
        return fail(parser, clause);
    }
    parser.advance();
    if !parser.is_punct("(") {
        return fail(parser, clause);
    }
    parser.advance();
    let (mut parser, kind, predicate) = match head.lexeme.as_str() {
        "foreign_assumes" => {
            spec_rule!("Parse-ForeignContractClause-Assumes");
            let (pred_parser, pred) = parse_predicate_expr(parser);
            (pred_parser, ForeignContractKind::Assumes, pred)
        }
        "foreign_ensures" => {
            spec_rule!("Parse-ForeignContractClause-Ensures");
            let qualifier = parser.advance_or_eof().tok();
            let qualified = parser.is_op("@")
                && qualifier.kind == TokenKind::Identifier
                && (qualifier.lexeme == "error" || qualifier.lexeme == "null_result");
            if qualified {
                let kind = if qualifier.lexeme == "error" {
                    spec_rule!("Parse-EnsuresPredicate-Error");
                    ForeignContractKind::EnsuresError
                } else {
                    spec_rule!("Parse-EnsuresPredicate-NullResult");
                    ForeignContractKind::EnsuresNullResult
                };
                let mut next = parser.advance_or_eof().advance_or_eof();
                expect_punct(&mut next, ":");
                let (pred_parser, pred) = parse_predicate_expr(next);
                (pred_parser, kind, pred)
            } else {
                spec_rule!("Parse-EnsuresPredicate-Plain");
                let (pred_parser, pred) = parse_predicate_expr(parser);
                (pred_parser, ForeignContractKind::Ensures, pred)
            }
        }
        _ => return fail(parser, clause),
    };
    if !parser.is_punct(")") {
        return fail(parser, clause);
    }
    parser.advance();
    clause.kind = kind;
    clause.predicates.push(predicate);
    clause.span = span_between(&start, &parser);
    (parser, clause)
}

fn parse_foreign_contract_clause_list_opt(parser: Parser) -> Parsed<Option<Vec<ForeignContractClause>>> {
    let mut probe = parser.clone();
    skip_newlines(&mut probe);
    if !is_foreign_contract_start(&probe) {
        spec_rule!("Parse-ForeignContractClauseListOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-ForeignContractClauseListOpt-Yes");
    spec_rule!("Parse-ForeignContractClauseList-Cons");
    let (mut cur, first) = parse_foreign_contract_clause(probe);
    let mut clauses = vec![first];
    loop {
        let mut probe = cur.clone();
        skip_newlines(&mut probe);
        if !is_foreign_contract_start(&probe) {
            spec_rule!("Parse-ForeignContractClauseListTail-End");
            return (cur, Some(clauses));
        }
        spec_rule!("Parse-ForeignContractClauseListTail-Cons");
        let (next, clause) = parse_foreign_contract_clause(probe);
        clauses.push(clause);
        cur = next;
    }
}

// ---- procedures -------------------------------------------------------------

struct ProcedureParts {
    name: Identifier,
    generic_params: Option<GenericParams>,
    params: Vec<Param>,
    return_type_opt: TypePtr,
    contract: Option<ContractClause>,
    body: BlockPtr,
}

/// `procedure name<generics>(params) -> ret |: contract { body }`, at the keyword.
fn parse_procedure_parts(parser: Parser) -> Parsed<ProcedureParts> {
    let (parser, name) = parse_ident(parser.advance_or_eof());
    let (parser, generic_params) = parse_generic_params_opt(parser);
    let (parser, sig) = parse_signature(parser);
    let (parser, contract) = parse_contract_clause_opt(parser);
    let (parser, body) = parse_block(parser);
    let parts = ProcedureParts {
        name,
        generic_params,
        params: sig.params,
        return_type_opt: sig.return_type_opt,
        contract,
        body,
    };
    (parser, parts)
}

pub(crate) fn parse_procedure_decl(
    parser: Parser,
    vis: Visibility,
    attrs: AttributeList,
    visibility_explicit: bool,
) -> Parsed<ASTItem> {
    spec_rule!("Parse-Procedure");
    let start = parser.clone();
    let (parser, parts) = parse_procedure_parts(parser);
    let decl = ProcedureDecl {
        attrs,
        vis,
        visibility_explicit,
        name: parts.name,
        generic_params: parts.generic_params,
        params: parts.params,
        return_type_opt: parts.return_type_opt,
        contract: parts.contract,
        body: parts.body,
        span: span_between(&start, &parser),
        doc: Vec::new(),
    };
    (parser, decl.into())
}

pub(crate) fn parse_comptime_procedure_decl(parser: Parser, attrs: AttributeList) -> Parsed<ASTItem> {
    spec_rule!("Parse-CtProc");
    let start = parser.clone();
    let (mut cur, vis) = parse_vis(parser.advance_or_eof());
    if !cur.is_kw("procedure") {
        syntax_err_here(&mut cur);
        sync_item(&mut cur);
        return error_item(&start, cur);
    }
    let (parser, parts) = parse_procedure_parts(cur);
    let decl = ComptimeProcedureDecl {
        attrs,
        vis,
        name: parts.name,
        generic_params: parts.generic_params,
        params: parts.params,
        return_type_opt: parts.return_type_opt,
        contract: parts.contract,
        body: parts.body,
        span: span_between(&start, &parser),
        doc: Vec::new(),
    };
    (parser, decl.into())
}

// ---- records ----------------------------------------------------------------

fn parse_record_field_decl_after_vis(parser: Parser, vis: Visibility, attrs: AttributeList) -> Parsed<FieldDecl> {
    spec_rule!("Parse-RecordFieldDeclAfterVis");
    let start = parser.clone();
    let (boundary_parser, key_boundary) = parse_key_boundary_opt(parser);
    let (mut name_parser, name) = parse_ident(boundary_parser);
    expect_punct(&mut name_parser, ":");
    let (ty_parser, ty) = parse_type(name_parser);
    let (init_parser, init_opt) = if ty_parser.is_op("=") {
        spec_rule!("Parse-RecordFieldInitOpt-Yes");
        parse_expr(ty_parser.advance_or_eof())
    } else {
        spec_rule!("Parse-RecordFieldInitOpt-None");
        (ty_parser, None)
    };
    let span = span_between(&start, &init_parser);
    let field = FieldDecl { attrs, vis, key_boundary, name, r#type: ty, init_opt, span, doc_opt: None };
    (init_parser, field)
}

fn parse_method_def_after_vis(parser: Parser, vis: Visibility, attrs: AttributeList) -> Parsed<MethodDecl> {
    spec_rule!("Parse-MethodDefAfterVis");
    let start = parser.clone();
    let override_flag = parser.is_kw("override");
    let mut cur = if override_flag {
        spec_rule!("Parse-Override-Yes");
        parser.advance_or_eof()
    } else {
        spec_rule!("Parse-Override-No");
        parser
    };
    expect_kw(&mut cur, "procedure");
    let (name_parser, name) = parse_ident(cur);
    let (generics_parser, generic_params) = parse_generic_params_opt(name_parser);
    let (sig_parser, sig) = parse_method_signature(generics_parser);
    let (contract_parser, contract) = parse_contract_clause_opt(sig_parser);
    let mut body_probe = contract_parser.clone();
    skip_newlines(&mut body_probe);
    let (body_parser, body) = if body_probe.is_punct("{") {
        parse_block(contract_parser)
    } else {
        let empty = Block { stmts: Vec::new(), tail_opt: None, span: contract_parser.tok_span() };
        (contract_parser, Some(Arc::new(empty)))
    };
    let MethodSignature { receiver, params, return_type_opt } = sig;
    let method = MethodDecl {
        attrs,
        vis,
        override_flag,
        name,
        generic_params,
        receiver,
        params,
        return_type_opt,
        contract,
        body,
        span: span_between(&start, &body_parser),
        doc_opt: None,
    };
    (body_parser, method)
}

fn parse_record_member(parser: Parser) -> Parsed<RecordMember> {
    let (vis_parser, vis, attrs) = parse_member_prefix(parser.clone());
    if vis_parser.is_kw("procedure") || vis_parser.is_kw("override") {
        spec_rule!("Parse-RecordMember-Method");
        let (method_parser, method) = parse_method_def_after_vis(vis_parser, vis, attrs);
        return (method_parser, method.into());
    }
    if vis_parser.is_kw("type") {
        spec_rule!("Parse-RecordMember-AssociatedType");
        let (name_parser, name) = parse_ident(vis_parser.advance_or_eof());
        let (after, default_type) = if name_parser.is_op("=") {
            spec_rule!("Parse-AssocTypeOpt-Yes");
            spec_rule!("Parse-AssocTypeDefaultOpt");
            parse_type(name_parser.advance_or_eof())
        } else {
            spec_rule!("Parse-AssocTypeOpt-None");
            spec_rule!("Parse-AssocTypeDefaultOpt");
            (name_parser, None)
        };
        let span = span_between(&parser, &after);
        let assoc = AssociatedTypeDecl { attrs, vis, name, default_type, span, doc_opt: None };
        return (after, assoc.into());
    }
    spec_rule!("Parse-RecordMember-Field");
    let (field_parser, field) = parse_record_field_decl_after_vis(vis_parser, vis, attrs);
    (field_parser, field.into())
}

/// Members of a braced body, one per line. `parse_member` must consume its own separator.
///
/// The reference loops forever when the body is not closed before end of file; this
/// stops at end of file and leaves the missing brace to the caller.
fn parse_member_list<T>(
    mut cur: Parser,
    mut parse_member: impl FnMut(Parser) -> Parsed<Option<T>>,
) -> Parsed<Vec<T>> {
    let mut members = Vec::new();
    while !cur.is_punct("}") {
        skip_newlines(&mut cur);
        if cur.is_punct("}") || cur.at_eof() {
            break;
        }
        let before = cur.index;
        let (next, member) = parse_member(cur);
        cur = next;
        members.extend(member);
        if cur.index == before {
            syntax_err_here(&mut cur);
            cur.advance();
        }
    }
    (cur, members)
}

/// `{ members }` after optional newlines; the closing brace is consumed when present.
fn parse_braced_body<T>(
    mut parser: Parser,
    parse_list: impl FnOnce(Parser) -> Parsed<Vec<T>>,
) -> Parsed<Vec<T>> {
    skip_newlines(&mut parser);
    if !parser.is_punct("{") {
        syntax_err_here(&mut parser);
        return (parser, Vec::new());
    }
    let (mut members_parser, members) = parse_list(parser.advance_or_eof());
    expect_punct(&mut members_parser, "}");
    (members_parser, members)
}

fn parse_record_member_list(mut parser: Parser) -> Parsed<Vec<RecordMember>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-RecordMemberList-End");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-RecordMemberList-Cons");
    parse_member_list(parser, |cur| {
        let (mut next, member) = parse_record_member(cur);
        if next.is_punct("}") {
            spec_rule!("Parse-RecordMemberSep-End");
        } else {
            spec_rule!("Parse-RecordMemberSep-Terminator");
            consume_terminator_req(&mut next);
        }
        (next, Some(member))
    })
}

pub(crate) fn parse_implements_opt(parser: Parser) -> Parsed<Vec<Path>> {
    if !parser.is_op("<:") {
        spec_rule!("Parse-Implements-None");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-Implements-Yes");
    spec_rule!("Parse-ClassList-Cons");
    let (mut parser, first) = parse_class_path(parser.advance_or_eof());
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if !parser.is_punct(",") {
            spec_rule!("Parse-ClassListTail-End");
            return (parser, xs);
        }
        let mut after = parser.advance_or_eof();
        skip_newlines(&mut after);
        if after.is_punct("{") {
            emit_trailing_comma_err(&mut parser, &OPEN_BRACE_END);
            after.diags = parser.diags;
            return (after, xs);
        }
        spec_rule!("Parse-ClassListTail-Comma");
        let (next, class_path) = parse_class_path(after);
        xs.push(class_path);
        parser = next;
    }
}

pub(crate) fn parse_invariant_opt(parser: Parser) -> Parsed<Option<TypeInvariant>> {
    if !parser.is_op("|:") {
        spec_rule!("Parse-InvariantOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-InvariantOpt-Yes");
    let fail = |mut parser: Parser| {
        syntax_err_here(&mut parser);
        sync_item(&mut parser);
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
    (after, Some(TypeInvariant { predicate, span }))
}

pub(crate) fn parse_record_decl(parser: Parser, vis: Visibility, attrs: AttributeList) -> Parsed<ASTItem> {
    spec_rule!("Parse-Record");
    let start = parser.clone();
    let (parser, name) = parse_ident(parser.advance_or_eof());
    let (parser, generic_params) = parse_generic_params_opt(parser);
    let (parser, implements) = parse_implements_opt(parser);
    spec_rule!("Parse-RecordBody");
    let (parser, members) = parse_braced_body(parser, parse_record_member_list);
    let (parser, invariant_opt) = parse_invariant_opt(parser);
    let span = span_between(&start, &parser);
    record_generic_owner_clause("RecordDecl", &name, &generic_params, &span);
    record_nominal_relation("RecordDecl", &name, "implements", &implements, &span);
    let decl = RecordDecl {
        attrs,
        vis,
        name,
        generic_params,
        implements,
        members,
        invariant_opt,
        span,
        doc: Vec::new(),
    };
    (parser, decl.into())
}

// ---- enums ------------------------------------------------------------------

fn parse_type_list(mut parser: Parser) -> Parsed<Vec<TypePtr>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-TypeList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-TypeList-Cons");
    let (first_parser, first) = parse_type(parser);
    parse_type_list_tail(first_parser, vec![first], &TYPE_LIST_END)
}

fn parse_field_decl(parser: Parser) -> Parsed<FieldDecl> {
    spec_rule!("Parse-FieldDecl");
    let start = parser.clone();
    let (vis_parser, vis) = parse_vis(parser);
    let (boundary_parser, key_boundary) = parse_key_boundary_opt(vis_parser);
    let (mut name_parser, name) = parse_ident(boundary_parser);
    expect_punct(&mut name_parser, ":");
    let (ty_parser, ty) = parse_type(name_parser);
    let span = span_between(&start, &ty_parser);
    let field = FieldDecl {
        attrs: Vec::new(),
        vis,
        key_boundary,
        name,
        r#type: ty,
        init_opt: None,
        span,
        doc_opt: None,
    };
    (ty_parser, field)
}

fn parse_field_decl_list(mut parser: Parser) -> Parsed<Vec<FieldDecl>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-FieldDeclList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-FieldDeclList-Cons");
    let (mut parser, first) = parse_field_decl(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct("}") {
            spec_rule!("Parse-FieldDeclTail-End");
            return (parser, xs);
        }
        if parser.is_punct(",") {
            let mut after = parser.advance_or_eof();
            skip_newlines(&mut after);
            if after.is_punct("}") {
                spec_rule!("Parse-FieldDeclTail-TrailingComma");
                return (after, xs);
            }
            spec_rule!("Parse-FieldDeclTail-Comma");
            let (next, field) = parse_field_decl(after);
            xs.push(field);
            parser = next;
            continue;
        }
        syntax_err_here(&mut parser);
        return (parser, xs);
    }
}

fn parse_variant_payload_opt(parser: Parser) -> Parsed<Option<VariantPayload>> {
    if parser.is_punct("(") {
        spec_rule!("Parse-VariantPayloadOpt-Tuple");
        let (mut types_parser, elements) = parse_type_list(parser.advance_or_eof());
        expect_punct(&mut types_parser, ")");
        return (types_parser, Some(VariantPayloadTuple { elements }.into()));
    }
    if parser.is_punct("{") {
        spec_rule!("Parse-VariantPayloadOpt-Record");
        let (mut fields_parser, fields) = parse_field_decl_list(parser.advance_or_eof());
        expect_punct(&mut fields_parser, "}");
        return (fields_parser, Some(VariantPayloadRecord { fields }.into()));
    }
    spec_rule!("Parse-VariantPayloadOpt-None");
    (parser, None)
}

fn parse_variant(parser: Parser) -> Parsed<VariantDecl> {
    spec_rule!("Parse-Variant");
    let start = parser.clone();
    let (name_parser, name) = parse_ident(parser);
    let (payload_parser, payload_opt) = parse_variant_payload_opt(name_parser);
    let (disc_parser, discriminant_opt) = if !payload_parser.is_op("=") {
        spec_rule!("Parse-VariantDiscriminantOpt-None");
        (payload_parser, None)
    } else {
        spec_rule!("Parse-VariantDiscriminantOpt-Yes");
        let mut next = payload_parser.advance_or_eof();
        let tok = next.tok();
        if tok.kind == TokenKind::IntLiteral {
            next.advance();
            (next, Some((*tok).clone()))
        } else {
            syntax_err_here(&mut next);
            (next, None)
        }
    };
    let span = span_between(&start, &disc_parser);
    let variant = VariantDecl { name, payload_opt, discriminant_opt, span, doc_opt: None };
    (disc_parser, variant)
}

fn emit_enum_top_level_comma_separator_err(parser: &mut Parser) {
    spec_rule!("req.EnumTopLevelCommaSeparatorRejected");
    spec_rule!("Parse-Syntax-Err");
    if parser.quote_mode {
        return;
    }
    if let Some(mut diag) = make_diagnostic_by_id("E-SRC-0520", Some(parser.tok_span())) {
        diag.obligation_ids.push("req.EnumTopLevelCommaSeparatorRejected".to_string());
        diag.obligation_ids.push("Parse-Syntax-Err".to_string());
        emit(&mut parser.diags, diag);
    }
}

fn parse_variant_members(mut parser: Parser) -> Parsed<Vec<VariantDecl>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-VariantMembers-Empty");
        return (parser, Vec::new());
    }
    parse_member_list(parser, |mut cur| {
        if cur.is_punct(",") {
            emit_enum_top_level_comma_separator_err(&mut cur);
            cur.advance();
            return (cur, None);
        }
        spec_rule!("Parse-VariantMembers-Cons");
        let (mut next, variant) = parse_variant(cur);
        if next.is_punct(",") {
            emit_enum_top_level_comma_separator_err(&mut next);
            next.advance();
        } else if next.is_punct("}") {
            spec_rule!("Parse-VariantSep-End");
        } else {
            spec_rule!("Parse-VariantSep-Terminator");
            consume_terminator_req(&mut next);
        }
        (next, Some(variant))
    })
}

pub(crate) fn parse_enum_decl(parser: Parser, vis: Visibility, attrs: AttributeList) -> Parsed<ASTItem> {
    spec_rule!("Parse-Enum");
    let start = parser.clone();
    let (parser, name) = parse_ident(parser.advance_or_eof());
    let (parser, generic_params) = parse_generic_params_opt(parser);
    let (parser, implements) = parse_implements_opt(parser);
    spec_rule!("Parse-EnumBody");
    let (parser, variants) = parse_braced_body(parser, parse_variant_members);
    let (parser, invariant_opt) = parse_invariant_opt(parser);
    let span = span_between(&start, &parser);
    record_generic_owner_clause("EnumDecl", &name, &generic_params, &span);
    record_nominal_relation("EnumDecl", &name, "implements", &implements, &span);
    let decl = EnumDecl {
        attrs,
        vis,
        name,
        generic_params,
        implements,
        variants,
        invariant_opt,
        span,
        doc: Vec::new(),
    };
    (parser, decl.into())
}

// ---- modals -----------------------------------------------------------------

fn parse_state_member(parser: Parser) -> Parsed<StateMember> {
    let start = parser.clone();
    let (cur, vis, attrs) = parse_member_prefix(parser);
    if cur.is_kw("procedure") {
        spec_rule!("Parse-StateMember-Method");
        let (name_parser, name) = parse_ident(cur.advance_or_eof());
        let (generics_parser, generic_params) = parse_generic_params_opt(name_parser);
        let (sig_parser, sig) = parse_state_method_signature(generics_parser);
        let (contract_parser, contract) = parse_contract_clause_opt(sig_parser);
        let (body_parser, body) = parse_block(contract_parser);
        let method = StateMethodDecl {
            attrs,
            vis,
            name,
            generic_params,
            receiver: sig.receiver,
            params: sig.params,
            return_type_opt: sig.return_type_opt,
            contract,
            body,
            span: span_between(&start, &body_parser),
            doc_opt: None,
        };
        return (body_parser, method.into());
    }
    if cur.is_kw("transition") {
        spec_rule!("Parse-StateMember-Transition");
        let (mut cur, name) = parse_ident(cur.advance_or_eof());
        expect_punct(&mut cur, "(");
        let (mut cur, params) = parse_param_list(cur);
        expect_punct(&mut cur, ")");
        expect_op(&mut cur, "->");
        expect_op(&mut cur, "@");
        let (target_parser, target_state) = parse_ident(cur);
        let (body_parser, body) = parse_block(target_parser);
        let transition = TransitionDecl {
            attrs,
            vis,
            name,
            params,
            target_state,
            body,
            span: span_between(&start, &body_parser),
            doc_opt: None,
        };
        return (body_parser, transition.into());
    }
    spec_rule!("Parse-StateMember-Field");
    let (boundary_parser, key_boundary) = parse_key_boundary_opt(cur);
    let (mut name_parser, name) = parse_ident(boundary_parser);
    expect_punct(&mut name_parser, ":");
    let (ty_parser, ty) = parse_type(name_parser);
    let span = span_between(&start, &ty_parser);
    let field = StateFieldDecl { attrs, vis, key_boundary, name, r#type: ty, span, doc_opt: None };
    (ty_parser, field.into())
}

fn parse_state_member_list(mut parser: Parser) -> Parsed<Vec<StateMember>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-StateMemberList-End");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-StateMemberList-Cons");
    parse_member_list(parser, |mut cur| {
        if cur.is_punct(",") {
            syntax_err_here(&mut cur);
            cur.advance();
            return (cur, None);
        }
        let (mut next, member) = parse_state_member(cur);
        skip_newlines(&mut next);
        (next, Some(member))
    })
}

fn parse_state_block(parser: Parser) -> Parsed<StateBlock> {
    spec_rule!("Parse-StateBlock");
    let start = parser.clone();
    if !parser.is_op("@") {
        let mut parser = parser;
        syntax_err_here(&mut parser);
        let next = parser.advance_or_eof();
        let span = span_between(&start, &next);
        let block = StateBlock { name: "_".to_string(), members: Vec::new(), span, doc_opt: None };
        return (next, block);
    }
    let (mut name_parser, name) = parse_ident(parser.advance_or_eof());
    expect_punct(&mut name_parser, "{");
    let (mut members_parser, members) = parse_state_member_list(name_parser);
    let span = span_between(&start, &members_parser);
    expect_punct(&mut members_parser, "}");
    (members_parser, StateBlock { name, members, span, doc_opt: None })
}

fn parse_state_block_list(mut parser: Parser) -> Parsed<Vec<StateBlock>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-StateBlockList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-StateBlockList-Cons");
    let mut blocks = Vec::new();
    let mut cur = parser;
    loop {
        let (next, block) = parse_state_block(cur);
        blocks.push(block);
        cur = next;
        skip_newlines(&mut cur);
        // The reference does not terminate when the body is unclosed at end of file.
        if cur.is_punct("}") || cur.at_eof() {
            return (cur, blocks);
        }
    }
}

pub(crate) fn parse_modal_decl(parser: Parser, vis: Visibility, attrs: AttributeList) -> Parsed<ASTItem> {
    spec_rule!("Parse-Modal");
    let start = parser.clone();
    let (parser, name) = parse_ident(parser.advance_or_eof());
    let (parser, generic_params) = parse_generic_params_opt(parser);
    let (parser, implements) = parse_implements_opt(parser);
    spec_rule!("Parse-ModalBody");
    let (parser, states) = parse_braced_body(parser, parse_state_block_list);
    let (parser, invariant_opt) = parse_invariant_opt(parser);
    let span = span_between(&start, &parser);
    record_generic_owner_clause("ModalDecl", &name, &generic_params, &span);
    record_nominal_relation("ModalDecl", &name, "implements", &implements, &span);
    let decl = ModalDecl {
        attrs,
        vis,
        name,
        generic_params,
        implements,
        states,
        invariant_opt,
        span,
        doc: Vec::new(),
    };
    (parser, decl.into())
}

// ---- classes ----------------------------------------------------------------

/// Generic parameters of the `Async` class: names with optional defaults, no bounds.
fn parse_async_type_params_opt(parser: Parser) -> Parsed<Option<GenericParams>> {
    let start = parser.clone();
    let mut next = parser;
    let mut params = Vec::new();
    loop {
        let param_start = next.advance_or_eof();
        let (name_parser, name) = parse_ident(param_start.clone());
        let (after, default_type) = if name_parser.is_op("=") {
            parse_type(name_parser.advance_or_eof())
        } else {
            (name_parser, None)
        };
        let span = span_between(&param_start, &after);
        params.push(TypeParam { name, bounds: Vec::new(), default_type, variance: None, span });
        next = after;
        if !next.is_punct(";") {
            break;
        }
    }
    expect_op(&mut next, ">");
    let span = span_between(&start, &next);
    (next, Some(GenericParams { params, span }))
}

fn parse_abstract_field_list(mut cur: Parser) -> Parsed<Vec<AbstractFieldDecl>> {
    let mut fields = Vec::new();
    skip_newlines(&mut cur);
    // The reference does not terminate when the body is unclosed at end of file.
    while !cur.is_punct("}") && !cur.at_eof() {
        let before = cur.clone();
        let (vis_parser, vis, attrs) = parse_member_prefix(cur);
        let (boundary_parser, key_boundary) = parse_key_boundary_opt(vis_parser);
        let (mut name_parser, name) = parse_ident(boundary_parser);
        expect_punct(&mut name_parser, ":");
        let (mut after_field, ty) = parse_type(name_parser);
        if !after_field.is_punct("}") {
            consume_terminator_req(&mut after_field);
        }
        let span = span_between(&before, &after_field);
        fields.push(AbstractFieldDecl { attrs, vis, key_boundary, name, r#type: ty, span, doc_opt: None });
        cur = after_field;
        skip_newlines(&mut cur);
        if cur.index == before.index {
            syntax_err_here(&mut cur);
            cur.advance();
        }
    }
    (cur, fields)
}

fn parse_class_item(parser: Parser) -> Parsed<ClassItem> {
    let start = parser.clone();
    let (cur, vis, attrs) = parse_member_prefix(parser);
    if cur.is_op("@") {
        spec_rule!("Parse-ClassItem-AbstractState");
        let (after_name, name) = parse_ident(cur.advance_or_eof());
        let fail = |mut sync: Parser, attrs: AttributeList, name: Identifier, fields| {
            syntax_err_here(&mut sync);
            sync_stmt(&mut sync);
            let span = span_between(&start, &sync);
            let state = AbstractStateDecl { attrs, vis, name, fields, span, doc_opt: None };
            (sync, ClassItem::from(state))
        };
        if !after_name.is_punct("{") {
            return fail(after_name, attrs, String::new(), Vec::new());
        }
        let (fields_parser, fields) = parse_abstract_field_list(after_name.advance_or_eof());
        if !fields_parser.is_punct("}") {
            return fail(fields_parser, attrs, name, fields);
        }
        let mut after_r = fields_parser.advance_or_eof();
        consume_terminator_opt(&mut after_r, TerminatorPolicy::Optional);
        let span = span_between(&start, &after_r);
        let state = AbstractStateDecl { attrs, vis, name, fields, span, doc_opt: None };
        return (after_r, state.into());
    }
    if cur.is_kw("procedure") {
        spec_rule!("Parse-ClassItem-Method");
        let (name_parser, name) = parse_ident(cur.advance_or_eof());
        let (generics_parser, generic_params) = parse_generic_params_opt(name_parser);
        let (sig_parser, sig) = parse_method_signature(generics_parser);
        let (after_contract, contract) = parse_contract_clause_opt(sig_parser);
        let (after_sig, body_opt) = if after_contract.is_punct("{") {
            spec_rule!("Parse-ClassMethodBody-Concrete");
            parse_block(after_contract)
        } else {
            spec_rule!("Parse-ClassMethodBody-Abstract");
            let mut after = after_contract;
            consume_terminator_req(&mut after);
            (after, None)
        };
        let method = ClassMethodDecl {
            attrs,
            vis,
            name,
            generic_params,
            receiver: sig.receiver,
            params: sig.params,
            return_type_opt: sig.return_type_opt,
            contract,
            body_opt,
            span: span_between(&start, &after_sig),
            doc_opt: None,
        };
        return (after_sig, method.into());
    }
    if cur.is_kw("type") {
        spec_rule!("Parse-ClassItem-AssociatedType");
        let (name_parser, name) = parse_ident(cur.advance_or_eof());
        let (mut after_name, default_type) = if name_parser.is_op("=") {
            spec_rule!("Parse-AssocTypeOpt-Yes");
            parse_type(name_parser.advance_or_eof())
        } else {
            spec_rule!("Parse-AssocTypeOpt-None");
            (name_parser, None)
        };
        consume_terminator_req(&mut after_name);
        let span = span_between(&start, &after_name);
        let assoc = AssociatedTypeDecl { attrs, vis, name, default_type, span, doc_opt: None };
        return (after_name, assoc.into());
    }
    spec_rule!("Parse-ClassItem-Field");
    let (boundary_parser, key_boundary) = parse_key_boundary_opt(cur);
    let (mut name_parser, name) = parse_ident(boundary_parser);
    expect_punct(&mut name_parser, ":");
    let (mut after_type, ty) = parse_type(name_parser);
    consume_terminator_req(&mut after_type);
    let span = span_between(&start, &after_type);
    let field = ClassFieldDecl { attrs, vis, key_boundary, name, r#type: ty, span, doc_opt: None };
    (after_type, field.into())
}

fn parse_class_item_list(mut parser: Parser) -> Parsed<Vec<ClassItem>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-ClassItemList-End");
        spec_rule!("Parse-ClassItemList-Empty");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-ClassItemList-Cons");
    let result = parse_member_list(parser, |cur| {
        let (mut next, item) = parse_class_item(cur);
        skip_newlines(&mut next);
        (next, Some(item))
    });
    spec_rule!("Parse-ClassItemList-End");
    result
}

/// A superclass bound; generic arguments are parsed and discarded.
fn parse_class_bound(parser: Parser) -> Parsed<Path> {
    let (mut cur, class_path) = parse_class_path(parser);
    if cur.is_op("<") {
        cur.advance();
        if !cur.is_op(">") {
            let (arg_parser, _) = parse_type(cur);
            cur = arg_parser;
            while cur.is_punct(",") {
                let (next_parser, _) = parse_type(cur.advance_or_eof());
                cur = next_parser;
            }
        }
        expect_op(&mut cur, ">");
    }
    (cur, class_path)
}

fn parse_superclass_opt(parser: Parser) -> Parsed<Vec<Path>> {
    if !parser.is_op("<:") {
        spec_rule!("Parse-Superclass-None");
        return (parser, Vec::new());
    }
    spec_rule!("Parse-Superclass-Yes");
    spec_rule!("Parse-SuperclassBounds-Cons");
    let (mut cur, first) = parse_class_bound(parser.advance_or_eof());
    let mut xs = vec![first];
    while cur.is_op("+") {
        spec_rule!("Parse-SuperclassBoundsTail-Plus");
        let (next, bound) = parse_class_bound(cur.advance_or_eof());
        xs.push(bound);
        cur = next;
    }
    spec_rule!("Parse-SuperclassBoundsTail-End");
    (cur, xs)
}

pub(crate) fn parse_class_decl(
    parser: Parser,
    vis: Visibility,
    attrs: AttributeList,
    is_modal: bool,
) -> Parsed<ASTItem> {
    spec_rule!("Parse-Class");
    let start = parser.clone();
    let (mut parser, name) = parse_ident(parser.advance_or_eof());
    let (parser, generic_params) = if name == "Async" {
        if parser.is_op("<") {
            parse_async_type_params_opt(parser)
        } else {
            syntax_err_here(&mut parser);
            (parser, None)
        }
    } else {
        parse_generic_params_opt(parser)
    };
    let (parser, supers) = parse_superclass_opt(parser);
    spec_rule!("Parse-ClassBody");
    let (parser, items) = parse_braced_body(parser, parse_class_item_list);
    let span = span_between(&start, &parser);
    record_generic_owner_clause("ClassDecl", &name, &generic_params, &span);
    record_nominal_relation("ClassDecl", &name, "supers", &supers, &span);
    let decl = ClassDecl {
        attrs,
        vis,
        modal: is_modal,
        name,
        generic_params,
        supers,
        items,
        span,
        doc: Vec::new(),
    };
    (parser, decl.into())
}

// ---- type aliases, using, import -------------------------------------------

pub(crate) fn parse_type_alias_decl(parser: Parser, vis: Visibility, attrs: AttributeList) -> Parsed<ASTItem> {
    spec_rule!("Parse-Type-Alias");
    let start = parser.clone();
    let (parser, name) = parse_ident(parser.advance_or_eof());
    let (mut parser, generic_params) = parse_generic_params_opt(parser);
    expect_op(&mut parser, "=");
    let (parser, ty) = parse_type(parser);
    let span = span_between(&start, &parser);
    record_generic_owner_clause("TypeAliasDecl", &name, &generic_params, &span);
    let decl = TypeAliasDecl { attrs, vis, name, generic_params, r#type: ty, span, doc: Vec::new() };
    (parser, decl.into())
}

/// The module part of a `using` path: every segment that is followed by another `::`.
fn parse_using_module_path(parser: Parser) -> Parsed<Path> {
    spec_rule!("Parse-Using-Path");
    let (mut cur, head) = parse_ident(parser);
    let mut path = vec![head];
    while cur.is_op("::") {
        let after_colons = cur.advance_or_eof();
        if after_colons.is_punct("{") || after_colons.is_op("*") {
            break;
        }
        let (probe, _) = parse_ident(after_colons.clone_without_diags());
        if !probe.is_op("::") {
            break;
        }
        let (seg_parser, seg) = parse_ident(after_colons);
        path.push(seg);
        cur = seg_parser;
    }
    (cur, path)
}

fn parse_using_spec(parser: Parser) -> Parsed<UsingSpec> {
    spec_rule!("Parse-UsingSpec");
    let (name_parser, name) = parse_ident(parser);
    let (alias_parser, alias_opt) = parse_alias_opt(name_parser);
    (alias_parser, UsingSpec { name, alias_opt })
}

fn parse_using_list(mut parser: Parser) -> Parsed<Vec<UsingSpec>> {
    skip_newlines(&mut parser);
    if parser.is_punct("}") {
        spec_rule!("Parse-UsingList-Empty");
        return (parser.advance_or_eof(), Vec::new());
    }
    spec_rule!("Parse-UsingList-Cons");
    let (mut parser, first) = parse_using_spec(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct("}") {
            spec_rule!("Parse-UsingListTail-End");
            break;
        }
        if !parser.is_punct(",") {
            syntax_err_here(&mut parser);
            break;
        }
        let mut after_comma = parser.advance_or_eof();
        skip_newlines(&mut after_comma);
        if after_comma.is_punct("}") {
            if trailing_comma_allowed(&parser, &BRACE_END) {
                spec_rule!("Parse-UsingListTail-TrailingComma");
            }
            emit_trailing_comma_err(&mut parser, &BRACE_END);
            after_comma.diags = parser.diags;
            parser = after_comma;
            break;
        }
        spec_rule!("Parse-UsingListTail-Comma");
        let (next, spec) = parse_using_spec(after_comma);
        xs.push(spec);
        parser = next;
    }
    expect_punct(&mut parser, "}");
    (parser, xs)
}

pub(crate) fn parse_using_decl(
    item_start: &Parser,
    parser: Parser,
    vis: Visibility,
    attrs_opt: AttrOpt,
) -> Parsed<ASTItem> {
    let (mut parser, module_path) = parse_using_module_path(parser.advance_or_eof());
    if !parser.is_op("::") {
        syntax_err_here(&mut parser);
        sync_item(&mut parser);
        return error_item(item_start, parser);
    }
    let mut after_colons = parser.advance_or_eof();
    let (end, clause): (Parser, UsingClause) = if after_colons.is_punct("{") {
        spec_rule!("Parse-Using-List");
        let (specs_parser, specs) = parse_using_list(after_colons.advance_or_eof());
        (specs_parser, UsingList { module_path, specs }.into())
    } else if after_colons.is_op("*") {
        spec_rule!("Parse-Using-Wildcard");
        (after_colons.advance_or_eof(), UsingWildcard { module_path }.into())
    } else {
        if !matches!(after_colons.tok().kind, TokenKind::Identifier | TokenKind::Keyword) {
            syntax_err_here(&mut after_colons);
            sync_item(&mut after_colons);
            return error_item(item_start, after_colons);
        }
        spec_rule!("Parse-Using-Item");
        let (name_parser, name) = parse_ident(after_colons);
        let (alias_parser, alias_opt) = parse_alias_opt(name_parser);
        (alias_parser, UsingItem { module_path, name, alias_opt }.into())
    };
    let span = span_between(item_start, &end);
    (end, UsingDecl { attrs_opt, vis, clause, span, doc: Vec::new() }.into())
}

pub(crate) fn parse_import_decl(parser: Parser, vis: Visibility, attrs_opt: AttrOpt) -> Parsed<ASTItem> {
    spec_rule!("Parse-Import");
    let start = parser.clone();
    let (parser, path) = parse_module_path(parser.advance_or_eof());
    let (parser, alias_opt) = parse_alias_opt(parser);
    let span = span_between(&start, &parser);
    (parser, ImportDecl { attrs_opt, vis, path, alias_opt, span, doc: Vec::new() }.into())
}

// ---- extern blocks ----------------------------------------------------------

fn parse_extern_proc_decl(parser: Parser) -> Parsed<ExternProcDecl> {
    spec_rule!("Parse-ExternProcDecl");
    let start = parser.clone();
    let (mut parser, vis, attrs) = parse_member_prefix(parser);
    expect_kw(&mut parser, "procedure");
    let (parser, name) = parse_ident(parser);
    let (parser, generic_params) = parse_generic_params_opt(parser);
    let (parser, sig) = parse_signature(parser);
    let (parser, contract) = parse_contract_clause_opt(parser);
    let (mut parser, foreign_contracts_opt) = parse_foreign_contract_clause_list_opt(parser);
    consume_terminator_req(&mut parser);
    let proc = ExternProcDecl {
        attrs,
        vis,
        name,
        generic_params,
        params: sig.params,
        return_type_opt: sig.return_type_opt,
        contract,
        foreign_contracts_opt,
        span: span_between(&start, &parser),
        doc: Vec::new(),
    };
    (parser, proc)
}

fn parse_extern_item_list(mut parser: Parser) -> Parsed<Vec<ExternItem>> {
    let mut items = Vec::new();
    loop {
        skip_newlines(&mut parser);
        if parser.is_punct("}") {
            spec_rule!("Parse-ExternItemList-End");
            return (parser, items);
        }
        if parser.at_eof() {
            return (parser, items);
        }
        spec_rule!("Parse-ExternItemList-Cons");
        let (next, proc) = parse_extern_proc_decl(parser);
        items.push(proc.into());
        parser = next;
    }
}

pub(crate) fn parse_extern_block(
    item_start: &Parser,
    parser: Parser,
    vis: Visibility,
    attrs_opt: AttrOpt,
) -> Parsed<ASTItem> {
    spec_rule!("Parse-ExternBlock");
    let mut parser = parser.advance_or_eof();
    let tok = parser.tok();
    let abi_opt: Option<ExternAbi> = match tok.kind {
        TokenKind::StringLiteral => {
            spec_rule!("Parse-ExternAbiOpt-String");
            parser.advance();
            Some(ExternAbiString { literal: (*tok).clone() }.into())
        }
        TokenKind::Identifier => {
            spec_rule!("Parse-ExternAbiOpt-Ident");
            parser.advance();
            Some(ExternAbiIdent { name: tok.lexeme.clone() }.into())
        }
        _ => {
            spec_rule!("Parse-ExternAbiOpt-None");
            None
        }
    };
    if !parser.is_punct("{") {
        syntax_err_here(&mut parser);
        sync_item(&mut parser);
        return error_item(item_start, parser);
    }
    let (mut parser, items) = parse_extern_item_list(parser.advance_or_eof());
    expect_punct(&mut parser, "}");
    let span = span_between(item_start, &parser);
    (parser, ExternBlock { attrs_opt, vis, abi_opt, items, span, doc: Vec::new() }.into())
}

// ---- derive targets ---------------------------------------------------------

fn parse_derive_clause(mut parser: Parser) -> Parsed<DeriveClause> {
    let start = parser.clone();
    let kind = if is_lexeme_token(&parser, "requires") {
        spec_rule!("Parse-DeriveClause-Requires");
        spec_rule!("rule.22.Parse-DeriveClause-Requires");
        DeriveClauseKind::Requires
    } else if is_lexeme_token(&parser, "emits") {
        spec_rule!("Parse-DeriveClause-Emits");
        DeriveClauseKind::Emits
    } else {
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span.clone());
        let clause = DeriveClause { kind: DeriveClauseKind::Requires, name: "_".to_string(), span };
        return (parser, clause);
    };
    let (name_parser, name) = parse_ident(parser.advance_or_eof());
    let span = span_between(&start, &name_parser);
    (name_parser, DeriveClause { kind, name, span })
}

fn parse_derive_contract_opt(parser: Parser) -> Parsed<Vec<DeriveClause>> {
    if !parser.is_op("|:") {
        spec_rule!("Parse-DeriveContractOpt-None");
        return (parser, Vec::new());
    }
    let (mut cur, first) = parse_derive_clause(parser.advance_or_eof());
    spec_rule!("Parse-DeriveClauseList-Cons");
    let mut xs = vec![first];
    while cur.is_punct(",") {
        spec_rule!("Parse-DeriveClauseTail-Comma");
        let (next, clause) = parse_derive_clause(cur.advance_or_eof());
        xs.push(clause);
        cur = next;
    }
    spec_rule!("Parse-DeriveClauseTail-End");
    spec_rule!("Parse-DeriveContractOpt-Yes");
    (cur, xs)
}

/// `derive target Name(target: Type) |: clauses { body }`.
pub(crate) fn parse_derive_target_decl(parser: Parser) -> Parsed<ASTItem> {
    let start = parser.clone();
    let fail = |mut parser: Parser| {
        syntax_err_here(&mut parser);
        sync_item(&mut parser);
        error_item(&start, parser)
    };
    let mut parser = parser.advance_or_eof();
    if !is_lexeme_token(&parser, "target") {
        return fail(parser);
    }
    parser.advance();
    let (mut parser, name) = parse_ident(parser);
    if !parser.is_punct("(") {
        return fail(parser);
    }
    parser.advance();
    if !is_lexeme_token(&parser, "target") {
        return fail(parser);
    }
    parser.advance();
    if !parser.is_punct(":") {
        return fail(parser);
    }
    parser.advance();
    if !is_lexeme_token(&parser, "Type") {
        return fail(parser);
    }
    parser.advance();
    if !parser.is_punct(")") {
        return fail(parser);
    }
    parser.advance();
    let (parser, contract_opt) = parse_derive_contract_opt(parser);
    let (parser, body) = parse_block(parser);
    let span = span_between(&start, &parser);
    spec_rule!("Parse-DeriveTargetDecl");
    (parser, DeriveTargetDecl { name, contract_opt, body, span, doc: Vec::new() }.into())
}
