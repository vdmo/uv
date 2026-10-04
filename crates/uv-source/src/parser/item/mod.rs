//! Module-level items.

pub(crate) mod attributes;
mod decls;
mod generics;
mod signature;

use std::sync::Arc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;
use uv_core::{spec_rule, spec_rule_at};

use self::attributes::parse_attribute_list_opt;
use self::decls::*;
use self::generics::emit_fix_it;
use super::expr::{parse_expr, skip_newlines};
use super::paths::{parse_modal_opt, parse_vis};
use super::pattern::parse_pattern;
use super::recovery::{emit_generic_parse_syntax_err, emit_parse_syntax_err, sync_item};
use super::state::{span_between, Parsed, Parser};
use super::types::parse_type_annot_opt;
use crate::ast::*;
use crate::lexer::{Token, TokenKind};

/// When a binding pattern is `name: T`, the annotation moves to the binding's type slot.
pub(crate) fn normalize_binding_pattern(pat: &mut PatternPtr, type_opt: &mut TypePtr) {
    if type_opt.is_some() {
        return;
    }
    let Some(pattern) = pat else {
        return;
    };
    let PatternNode::TypedPattern(typed) = &pattern.node else {
        return;
    };
    *type_opt = typed.r#type.clone();
    let normalized = Pattern {
        span: pattern.span.clone(),
        node: IdentifierPattern { name: typed.name.clone(), name_splice_opt: None }.into(),
    };
    *pat = Some(Arc::new(normalized));
}

pub(crate) fn parse_binding_after_let_var(parser: Parser) -> Parsed<Binding> {
    spec_rule!("Parse-BindingAfterLetVar");
    let start = parser.clone();
    let (pat_parser, pat) = parse_pattern(parser.advance_or_eof());
    let (mut ty_parser, type_opt) = parse_type_annot_opt(pat_parser);
    let tok = ty_parser.tok();
    let mut op = Token::default();
    if tok.kind == TokenKind::Operator && (tok.lexeme == "=" || tok.lexeme == ":=") {
        op = (*tok).clone();
        ty_parser.advance();
    } else {
        let span = ty_parser.tok_span();
        emit_parse_syntax_err(&mut ty_parser, span);
    }
    let (init_parser, init) = parse_expr(ty_parser);
    let span = span_between(&start, &init_parser);
    (init_parser, Binding { attrs: Vec::new(), pat, type_opt, op, init, span })
}

fn parse_static_decl(parser: Parser, vis: Visibility, attrs_opt: AttrOpt) -> Parsed<ASTItem> {
    spec_rule!("Parse-Static-Decl");
    let start = parser.clone();
    let mutability = if parser.is_kw("let") { Mutability::Let } else { Mutability::Var };
    let (parser, binding) = parse_binding_after_let_var(parser);
    let span = span_between(&start, &parser);
    let decl = StaticDecl { attrs_opt, vis, r#mut: mutability, binding, span, doc: Vec::new() };
    (parser, decl.into())
}

fn is_lexeme_token(parser: &Parser, lexeme: &str) -> bool {
    let tok = parser.tok();
    matches!(tok.kind, TokenKind::Identifier | TokenKind::Keyword) && tok.lexeme == lexeme
}

fn error_item(start: &Parser, end: Parser) -> Parsed<ASTItem> {
    let span = span_between(start, &end);
    (end, ErrorItem { span, doc: Vec::new() }.into())
}

/// Advances past the current token, resynchronizes and yields an error item.
fn skip_to_next_item(start: &Parser, parser: &Parser) -> Parsed<ASTItem> {
    let mut next = parser.advance_or_eof();
    sync_item(&mut next);
    error_item(start, next)
}

/// Keywords borrowed from other languages, with the replacement to suggest.
fn foreign_keyword_fix(lexeme: &str) -> Option<(&'static str, Option<&'static str>)> {
    match lexeme {
        "pub" => Some(("replace `pub` with `public`", Some("public"))),
        "fn" => Some(("replace `fn` with `procedure`", Some("procedure"))),
        "struct" => Some(("replace `struct` with `record`", Some("record"))),
        "trait" => Some(("replace `trait` with `class`", Some("class"))),
        "impl" => Some(("define methods inside the `record` body instead", None)),
        _ => None,
    }
}

fn emit_foreign_keyword_err(parser: &mut Parser, span: Span, message: &str, fix: Option<&str>) {
    match fix {
        Some(fix) => emit_fix_it(parser, span, message, fix),
        None => {
            if let Some(mut diag) = make_diagnostic_by_id("E-SRC-0520", Some(span)) {
                diag.children.push(SubDiagnostic {
                    kind: SubDiagnosticKind::Help,
                    message: message.to_string(),
                    ..SubDiagnostic::default()
                });
                emit(&mut parser.diags, diag);
            }
        }
    }
}

pub fn parse_item(mut parser: Parser) -> Parsed<ASTItem> {
    skip_newlines(&mut parser);
    let start = parser.clone();
    if parser.at_eof() {
        return (parser, ErrorItem::default().into());
    }
    let (attrs_parser, mut attrs_opt) = parse_attribute_list_opt(parser);
    let mut attrs_list = attrs_opt.clone().unwrap_or_default();
    let mut parser = attrs_parser;
    loop {
        let mut next_attrs = parser.clone();
        skip_newlines(&mut next_attrs);
        let (more_parser, more_attrs) = parse_attribute_list_opt(next_attrs.clone());
        match more_attrs {
            None => {
                parser = next_attrs;
                break;
            }
            Some(more) => {
                attrs_list.extend(more);
                attrs_opt = Some(attrs_list.clone());
                parser = more_parser;
            }
        }
    }
    skip_newlines(&mut parser);
    let tok = parser.tok();
    let is_ident = |lexeme: &str| tok.kind == TokenKind::Identifier && tok.lexeme == lexeme;
    if is_ident("where") {
        spec_rule!("Parse-Stray-Where");
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        sync_item(&mut parser);
        return error_item(&start, parser);
    }
    if parser.is_kw("import") {
        spec_rule!("Parse-Import");
        return parse_import_decl(parser, Visibility::Internal, attrs_opt);
    }
    if parser.is_kw("modal") {
        let (modal_parser, _) = parse_modal_opt(parser.clone());
        if is_lexeme_token(&modal_parser, "class") {
            spec_rule!("Parse-Modal-Class");
            return parse_class_decl(modal_parser, Visibility::Internal, attrs_list, true);
        }
    }
    if is_ident("use") {
        spec_rule!("Parse-Item-Err");
        let span = parser.tok_span();
        emit_generic_parse_syntax_err(&mut parser, span);
        return skip_to_next_item(&start, &parser);
    }
    if parser.is_kw("return") {
        spec_rule!("Return-At-Module-Err");
        if !parser.quote_mode {
            if let Some(diag) = make_diagnostic_by_id("E-SEM-3165", None) {
                let span = parser.tok_span();
                spec_rule_at!("diag.18.ControlTransferStatements", span);
                spec_rule_at!("diag.18.StatementDiagnosticsSupplement", span);
                emit(&mut parser.diags, diag);
            }
        }
        return skip_to_next_item(&start, &parser);
    }
    if parser.is_kw("comptime") {
        let (vis_parser, _) = parse_vis(parser.advance_or_eof());
        if vis_parser.is_kw("procedure") {
            spec_rule!("Parse-CtProc");
            return parse_comptime_procedure_decl(parser, attrs_list);
        }
    }
    if tok.kind == TokenKind::Identifier {
        if let Some((message, fix)) = foreign_keyword_fix(&tok.lexeme) {
            let span = parser.tok_span();
            emit_foreign_keyword_err(&mut parser, span, message, fix);
            return skip_to_next_item(&start, &parser);
        }
    }
    let (cur, vis) = parse_vis(parser.clone());
    let visibility_explicit = cur.index != parser.index;
    if is_lexeme_token(&cur, "derive") {
        if !attrs_list.is_empty() || visibility_explicit {
            // The reference reports this on a parser copy that it then discards.
            let mut discarded = parser.clone();
            let span = discarded.tok_span();
            emit_parse_syntax_err(&mut discarded, span);
            let mut next = cur;
            sync_item(&mut next);
            return error_item(&start, next);
        }
        spec_rule!("Parse-Derive-Target");
        return parse_derive_target_decl(cur);
    }
    if cur.is_kw("import") {
        spec_rule!("Parse-Import");
        return parse_import_decl(cur, vis, attrs_opt);
    }
    if cur.is_kw("extern") {
        spec_rule!("Parse-ExternBlock");
        return parse_extern_block(&start, cur, vis, attrs_opt);
    }
    let cur_tok = cur.tok();
    if cur_tok.kind == TokenKind::Identifier && cur_tok.lexeme == "extern" {
        let mut cur = cur;
        let span = cur.tok_span();
        emit_parse_syntax_err(&mut cur, span);
        return skip_to_next_item(&start, &cur);
    }
    if cur.is_kw("using") {
        return parse_using_decl(&start, cur, vis, attrs_opt);
    }
    if cur.is_kw("let") || cur.is_kw("var") {
        spec_rule!("Parse-Static-Decl");
        return parse_static_decl(cur, vis, attrs_opt);
    }
    if cur.is_kw("procedure") {
        spec_rule!("Parse-Procedure");
        return parse_procedure_decl(cur, vis, attrs_list, visibility_explicit);
    }
    if cur.is_kw("record") {
        spec_rule!("Parse-Record");
        return parse_record_decl(cur, vis, attrs_list);
    }
    if cur.is_kw("enum") {
        spec_rule!("Parse-Enum");
        return parse_enum_decl(cur, vis, attrs_list);
    }
    if cur.is_kw("modal") || cur.is_kw("class") {
        let (class_parser, is_modal) = parse_modal_opt(cur.clone());
        let is_class_decl = is_lexeme_token(&class_parser, "class");
        if is_modal && is_class_decl {
            spec_rule!("Parse-Modal-Class");
            return parse_class_decl(class_parser, vis, attrs_list, true);
        }
        if is_modal {
            spec_rule!("Parse-Modal");
            return parse_modal_decl(cur, vis, attrs_list);
        }
        if is_class_decl {
            spec_rule!("Parse-Class");
            return parse_class_decl(class_parser, vis, attrs_list, false);
        }
    }
    if cur.is_kw("type") {
        spec_rule!("Parse-Type-Alias");
        return parse_type_alias_decl(cur, vis, attrs_list);
    }
    if cur_tok.kind == TokenKind::Identifier
        && matches!(cur_tok.lexeme.as_str(), "fn" | "struct" | "impl" | "trait")
    {
        // After a visibility modifier the reference reports this on a parser that it
        // then discards, so only the error item survives.
        if let Some((message, fix)) = foreign_keyword_fix(&cur_tok.lexeme) {
            let mut discarded = parser.clone();
            emit_foreign_keyword_err(&mut discarded, cur.tok_span(), message, fix);
        }
        return skip_to_next_item(&start, &cur);
    }
    spec_rule!("Parse-Item-Err");
    let span = parser.tok_span();
    emit_generic_parse_syntax_err(&mut parser, span);
    skip_to_next_item(&start, &parser)
}
