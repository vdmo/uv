//! Attribute lists: `#name`, `#vendor::name(args)`, with dedicated argument grammars for
//! `#layout(...)` and `#inline(...)`.

use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;

use crate::ast::*;
use crate::lexer::TokenKind;
use crate::parser::consume::{emit_trailing_comma_err, match_punct, trailing_comma_allowed, TokenMatch};
use crate::parser::expr::{is_literal_token, skip_newlines};
use crate::parser::paths::parse_ident;
use crate::parser::state::{span_between, Parsed, Parser};

const PAREN_END: [TokenMatch; 1] = [match_punct(")")];

fn emit_attr_syntax_err(parser: &mut Parser, span: Span) {
    parser.emit("E-MOD-2450", span);
}

fn attr_err_here(parser: &mut Parser) {
    let span = parser.tok_span();
    emit_attr_syntax_err(parser, span);
}

fn is_builtin(name: &AttrName, leaf: &str) -> bool {
    name.vendor_prefix_opt.is_none() && name.leaf_name == leaf
}

fn record_ffi_attribute_parsing(item: &AttributeItem, arg_form: &str) {
    let is_ffi = matches!(
        item.name.full_name.as_str(),
        "export" | "host_export" | "mangle" | "library" | "unwind" | "ffi_pass_by_value"
    );
    if Conformance::enabled() && is_ffi {
        Conformance::record_at(
            "requirement.23.FfiAttributesParsing",
            Some(&item.span),
            &format!(
                "source=ParseAttributeItem;parser=attribute_list;attribute={};arg_form={arg_form};ordinary_attribute_entry=true",
                item.name.full_name
            ),
        );
    }
}

fn record_builtin_attribute_parsing(item: &AttributeItem) {
    if !Conformance::enabled() {
        return;
    }
    let args = item.args.len();
    if is_builtin(&item.name, "derive") {
        Conformance::record_at(
            "requirement.22.DeriveAttributeParsingReference",
            Some(&item.span),
            &format!("source=ParseAttributeItem;parser=attribute_list;args={args};attribute=derive"),
        );
    }
    if is_builtin(&item.name, "layout") {
        let payload =
            format!("source=ParseAttributeItem;parser=attribute_list;args={args};ast_attachment=true");
        for rule in [
            "grammar.LayoutAttributeSyntax",
            "req.LayoutAttributeParserReuse",
            "def.LayoutAttributeAstAttachment",
        ] {
            Conformance::record_at(rule, Some(&item.span), &payload);
        }
    }
}

fn token_arg(key: Option<Identifier>, token: &crate::lexer::Token) -> AttributeArg {
    AttributeArg { key, value: AttributeArgValue::Token(token.clone()) }
}

fn parse_attr_arg(mut parser: Parser) -> Parsed<AttributeArg> {
    let arg = AttributeArg::default();
    skip_newlines(&mut parser);
    let tok = parser.tok();
    if tok.kind == TokenKind::Identifier {
        let (after_name, name) = parse_ident(parser);
        if after_name.is_punct(":") {
            let mut after_colon = after_name.advance_or_eof();
            let value = after_colon.tok();
            if is_literal_token(&value) {
                spec_rule!("Parse-AttrArg-Named-Literal");
            } else if value.kind == TokenKind::Identifier {
                spec_rule!("Parse-AttrArg-Named-Ident");
            } else {
                attr_err_here(&mut after_colon);
                return (after_colon, arg);
            }
            after_colon.advance();
            return (after_colon, token_arg(Some(name), &value));
        }
        if after_name.is_punct("(") {
            spec_rule!("Parse-AttrArg-Named-Call");
            let (mut after_args, nested) = parse_attr_arg_list(after_name.advance_or_eof());
            if !after_args.is_punct(")") {
                attr_err_here(&mut after_args);
                return (after_args, arg);
            }
            after_args.advance();
            let arg = AttributeArg { key: Some(name), value: AttributeArgValue::AttributeArgList(nested) };
            return (after_args, arg);
        }
        spec_rule!("Parse-AttrArg-Ident");
        return (after_name, token_arg(None, &tok));
    }
    if is_literal_token(&tok) {
        spec_rule!("Parse-AttrArg-Literal");
        return (parser.advance_or_eof(), token_arg(None, &tok));
    }
    attr_err_here(&mut parser);
    (parser, arg)
}

fn parse_attr_arg_list(mut parser: Parser) -> Parsed<Vec<AttributeArg>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        spec_rule!("Parse-AttrArgsOpt-Empty");
        return (parser, Vec::new());
    }
    let (mut parser, first) = parse_attr_arg(parser);
    spec_rule!("Parse-AttrArgList-Cons");
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if !parser.is_punct(",") {
            spec_rule!("Parse-AttrArgListTail-End");
            return (parser, xs);
        }
        let mut after = parser.advance_or_eof();
        skip_newlines(&mut after);
        if after.is_punct(")") {
            if trailing_comma_allowed(&parser, &PAREN_END) {
                spec_rule!("Parse-AttrArgListTail-TrailingComma");
            }
            emit_trailing_comma_err(&mut parser, &PAREN_END);
            after.diags = parser.diags;
            return (after, xs);
        }
        spec_rule!("Parse-AttrArgListTail-Comma");
        let (next, arg) = parse_attr_arg(after);
        xs.push(arg);
        parser = next;
    }
}

/// `#inline(always | never | default)`.
fn parse_inline_arg_list(mut parser: Parser) -> Parsed<Vec<AttributeArg>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        return (parser, Vec::new());
    }
    let tok = parser.tok();
    let is_mode = matches!(tok.kind, TokenKind::Identifier | TokenKind::Keyword)
        && matches!(tok.lexeme.as_str(), "always" | "never" | "default");
    if !is_mode {
        attr_err_here(&mut parser);
        return (parser, Vec::new());
    }
    let mut after = parser.advance_or_eof();
    skip_newlines(&mut after);
    if !after.is_punct(")") {
        attr_err_here(&mut after);
        return (after, Vec::new());
    }
    (after, vec![token_arg(None, &tok)])
}

/// One `#layout` argument: `C`, `packed`, a fixed-width integer type, or `align(N)`.
fn parse_layout_arg(mut parser: Parser) -> Parsed<AttributeArg> {
    let arg = AttributeArg::default();
    skip_newlines(&mut parser);
    let tok = parser.tok();
    if !matches!(tok.kind, TokenKind::Identifier | TokenKind::Keyword) {
        attr_err_here(&mut parser);
        return (parser, arg);
    }
    if tok.lexeme == "align" {
        let mut after_name = parser.advance_or_eof();
        skip_newlines(&mut after_name);
        if !after_name.is_punct("(") {
            attr_err_here(&mut after_name);
            return (after_name, arg);
        }
        let mut after_open = after_name.advance_or_eof();
        skip_newlines(&mut after_open);
        let value = after_open.tok();
        if value.kind != TokenKind::IntLiteral {
            attr_err_here(&mut after_open);
            return (after_open, arg);
        }
        let mut after_value = after_open.advance_or_eof();
        skip_newlines(&mut after_value);
        if !after_value.is_punct(")") {
            attr_err_here(&mut after_value);
            return (after_value, arg);
        }
        let arg = AttributeArg {
            key: Some("align".to_string()),
            value: AttributeArgValue::AttributeArgList(vec![token_arg(None, &value)]),
        };
        return (after_value.advance_or_eof(), arg);
    }
    let is_layout_word = matches!(
        tok.lexeme.as_str(),
        "C" | "packed" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64"
    );
    if is_layout_word {
        return (parser.advance_or_eof(), token_arg(None, &tok));
    }
    attr_err_here(&mut parser);
    (parser, arg)
}

fn parse_layout_arg_list(mut parser: Parser) -> Parsed<Vec<AttributeArg>> {
    skip_newlines(&mut parser);
    if parser.is_punct(")") {
        attr_err_here(&mut parser);
        return (parser, Vec::new());
    }
    let (mut parser, first) = parse_layout_arg(parser);
    let mut xs = vec![first];
    loop {
        skip_newlines(&mut parser);
        if !parser.is_punct(",") {
            return (parser, xs);
        }
        let mut after = parser.advance_or_eof();
        skip_newlines(&mut after);
        if after.is_punct(")") {
            emit_trailing_comma_err(&mut parser, &PAREN_END);
            after.diags = parser.diags;
            return (after, xs);
        }
        let (next, arg) = parse_layout_arg(after);
        xs.push(arg);
        parser = next;
    }
}

/// An identifier or keyword used as an attribute name segment.
fn parse_attr_leaf_segment(mut parser: Parser) -> Parsed<Identifier> {
    let tok = parser.tok();
    if matches!(tok.kind, TokenKind::Identifier | TokenKind::Keyword) {
        parser.advance();
        return (parser, tok.lexeme.clone());
    }
    attr_err_here(&mut parser);
    (parser, "_".to_string())
}

fn plain_name(leaf: Identifier) -> AttrName {
    AttrName { vendor_prefix_opt: None, full_name: leaf.clone(), leaf_name: leaf }
}

fn parse_attr_name(parser: Parser) -> Parsed<AttrName> {
    let first_is_identifier = parser.tok().kind == TokenKind::Identifier;
    let (mut first_parser, first) = parse_attr_leaf_segment(parser);
    if !first_parser.is_punct(".") && !first_parser.is_op("::") {
        spec_rule!("Parse-AttrName-Plain");
        return (first_parser, plain_name(first));
    }
    if first_parser.is_punct(".") || !first_is_identifier {
        attr_err_here(&mut first_parser);
        return (first_parser, plain_name(first));
    }
    spec_rule!("Parse-AttrName-Vendor");
    let mut prefix = vec![first.clone()];
    let mut cur = first_parser;
    loop {
        // A segment joins the vendor prefix only when another `::` follows it.
        let after_colons = cur.advance_or_eof();
        let seg = after_colons.tok();
        let after_seg = after_colons.advance_or_eof();
        if !cur.is_op("::") || seg.kind != TokenKind::Identifier || !after_seg.is_op("::") {
            spec_rule!("Parse-VendorPrefixTail-End");
            break;
        }
        spec_rule!("Parse-VendorPrefixTail-Cons");
        prefix.push(seg.lexeme.clone());
        cur = after_seg;
    }
    if !cur.is_op("::") {
        attr_err_here(&mut cur);
        return (cur, plain_name(first));
    }
    let (leaf_parser, leaf) = parse_attr_leaf_segment(cur.advance_or_eof());
    let full_name = format!("{}::{leaf}", prefix.join("::"));
    (leaf_parser, AttrName { vendor_prefix_opt: Some(prefix), leaf_name: leaf, full_name })
}

fn parse_attribute_item(mut parser: Parser) -> Parsed<AttributeItem> {
    let mut item = AttributeItem::default();
    spec_rule!("Parse-AttrSpec");
    if !matches!(parser.tok().kind, TokenKind::Identifier | TokenKind::Keyword) {
        attr_err_here(&mut parser);
        return (parser, item);
    }
    let (mut next, name) = parse_attr_name(parser.clone());
    item.name = name;
    let requires_args = is_builtin(&item.name, "layout");
    if !next.is_punct("(") {
        item.span = span_between(&parser, &next);
        if requires_args {
            emit_attr_syntax_err(&mut next, item.span.clone());
            return (next, item);
        }
        spec_rule!("Parse-AttrArgsOpt-None");
        record_ffi_attribute_parsing(&item, "none");
        return (next, item);
    }
    spec_rule!("Parse-AttrArgsOpt-Yes");
    let after_open = next.advance_or_eof();
    let (mut after_args, args) = if requires_args {
        parse_layout_arg_list(after_open)
    } else if is_builtin(&item.name, "inline") {
        parse_inline_arg_list(after_open)
    } else {
        parse_attr_arg_list(after_open)
    };
    if !after_args.is_punct(")") {
        attr_err_here(&mut after_args);
        item.span = span_between(&parser, &after_args);
        return (after_args, item);
    }
    let after_close = after_args.advance_or_eof();
    item.args = args;
    item.span = span_between(&parser, &after_close);
    record_builtin_attribute_parsing(&item);
    record_ffi_attribute_parsing(&item, "parenthesized");
    (after_close, item)
}

/// A run of `#attr` items, or `None` when no attribute starts here.
pub(crate) fn parse_attribute_list_opt(parser: Parser) -> Parsed<Option<AttributeList>> {
    if !parser.is_op("#") {
        spec_rule!("Parse-AttrListOpt-None");
        return (parser, None);
    }
    spec_rule!("Parse-AttrListOpt-Yes");
    spec_rule!("Parse-AttrList-Cons");
    let mut attrs = Vec::new();
    let mut cur = parser;
    loop {
        spec_rule!("Parse-Attribute");
        let (attr_parser, mut attr) = parse_attribute_item(cur.advance_or_eof());
        attr.span = span_between(&cur, &attr_parser);
        attrs.push(attr);
        cur = attr_parser;
        if !cur.is_op("#") {
            spec_rule!("Parse-AttrListTail-End");
            return (cur, Some(attrs));
        }
        spec_rule!("Parse-AttrListTail-Cons");
    }
}
