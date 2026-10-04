//! Syntax-error reporting, resynchronization and statement terminators.

use uv_core::span::Span;
use uv_core::{spec_rule, spec_rule_at};

use super::state::Parser;
use crate::lexer::{Token, TokenKind};

fn is_sync_item_token(tok: &Token) -> bool {
    match tok.kind {
        TokenKind::Keyword => matches!(
            tok.lexeme.as_str(),
            "procedure" | "record" | "enum" | "modal" | "class" | "type" | "using" | "let" | "var"
        ),
        TokenKind::Punctuator => tok.lexeme == "}",
        _ => false,
    }
}

fn is_terminator_token(tok: &Token) -> bool {
    tok.kind == TokenKind::Newline || (tok.kind == TokenKind::Punctuator && tok.lexeme == ";")
}

pub fn emit_parse_syntax_err(parser: &mut Parser, span: Span) {
    if parser.quote_mode {
        return;
    }
    parser.emit("E-SRC-0520", span);
}

pub fn emit_generic_parse_syntax_err(parser: &mut Parser, span: Span) {
    if parser.quote_mode {
        return;
    }
    spec_rule!("Parse-Syntax-Err");
    emit_parse_syntax_err(parser, span);
}

pub fn emit_splice_outside_quote_err(parser: &mut Parser, span: Span) {
    parser.emit("E-CTE-0233", span);
}

pub fn sync_stmt(parser: &mut Parser) {
    spec_rule!("def.18.SyncStmt");
    loop {
        if parser.at_eof() || parser.is_punct("}") {
            spec_rule!("Sync-Stmt-Stop");
            return;
        }
        if is_terminator_token(&parser.tok()) {
            spec_rule!("Sync-Stmt-Consume");
            parser.advance();
            return;
        }
        spec_rule!("Sync-Stmt-Advance");
        parser.advance();
    }
}

pub fn sync_item(parser: &mut Parser) {
    loop {
        if parser.at_eof() || is_sync_item_token(&parser.tok()) {
            spec_rule!("Sync-Item-Stop");
            return;
        }
        spec_rule!("Sync-Item-Advance");
        parser.advance();
    }
}

pub fn sync_type(parser: &mut Parser) {
    loop {
        if parser.at_eof() {
            spec_rule!("Sync-Type-Stop");
            return;
        }
        let tok = parser.tok();
        if tok.kind == TokenKind::Punctuator {
            if matches!(tok.lexeme.as_str(), ")" | "]" | "}") {
                spec_rule!("Sync-Type-Stop");
                return;
            }
            if matches!(tok.lexeme.as_str(), "," | ";") {
                spec_rule!("Sync-Type-Consume");
                parser.advance();
                return;
            }
        }
        if tok.kind == TokenKind::Newline {
            spec_rule!("Sync-Type-Consume");
            parser.advance();
            return;
        }
        spec_rule!("Sync-Type-Advance");
        parser.advance();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminatorPolicy {
    Required,
    Optional,
}

fn emit_missing_terminator(parser: &mut Parser, span: Span) {
    spec_rule_at!("Missing-Terminator-Err", span);
    spec_rule_at!("diag.18.Blocks", span);
    spec_rule_at!("req.16.ControlExpressionDiagnosticOwnership", span);
    parser.emit("E-SRC-0510", span);
}

pub fn consume_terminator_opt(parser: &mut Parser, policy: TerminatorPolicy) {
    let is_term = is_terminator_token(&parser.tok());
    if policy == TerminatorPolicy::Required {
        if is_term {
            spec_rule!("ConsumeTerminatorOpt-Req-Yes");
            parser.advance();
            return;
        }
        spec_rule!("ConsumeTerminatorOpt-Req-No");
        let span = parser.tok_span();
        spec_rule_at!("rule.18.ConsumeTerminatorOpt-Req-No", span);
        emit_missing_terminator(parser, span);
        sync_stmt(parser);
        return;
    }
    if is_term {
        spec_rule!("ConsumeTerminatorOpt-Opt-Yes");
        parser.advance();
    } else {
        spec_rule!("ConsumeTerminatorOpt-Opt-No");
    }
}

pub fn consume_terminator_req(parser: &mut Parser) {
    if is_terminator_token(&parser.tok()) {
        spec_rule!("ConsumeTerminatorReq-Yes");
        parser.advance();
        return;
    }
    spec_rule!("ConsumeTerminatorReq-No");
    let span = parser.tok_span();
    emit_missing_terminator(parser, span);
    sync_stmt(parser);
}
