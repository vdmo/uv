//! Statements and blocks.

use std::sync::Arc;

use uv_core::span::Span;
use uv_core::spec_trace::Conformance;
use uv_core::{spec_rule, spec_rule_at};

use super::expr::{
    is_literal_token, is_place, parse_expr, parse_expr_opt, parse_key_mode, parse_place, skip_newlines,
};
use super::item::attributes::parse_attribute_list_opt;
use super::item::parse_binding_after_let_var;
use super::paths::parse_ident;
use super::recovery::{
    consume_terminator_opt, emit_generic_parse_syntax_err, emit_parse_syntax_err, sync_stmt,
    TerminatorPolicy,
};
use super::state::{merge_diag, span_between, Parsed, Parser};
use crate::ast::*;
use crate::lexer::{Token, TokenKind};

/// Span of a statement.
pub fn stmt_span(stmt: &Stmt) -> &Span {
    match stmt {
        Stmt::LetStmt(s) => &s.span,
        Stmt::VarStmt(s) => &s.span,
        Stmt::UsingLocalStmt(s) => &s.span,
        Stmt::AssignStmt(s) => &s.span,
        Stmt::CompoundAssignStmt(s) => &s.span,
        Stmt::ExprStmt(s) => &s.span,
        Stmt::DeferStmt(s) => &s.span,
        Stmt::RegionStmt(s) => &s.span,
        Stmt::FrameStmt(s) => &s.span,
        Stmt::ReturnStmt(s) => &s.span,
        Stmt::BreakStmt(s) => &s.span,
        Stmt::ContinueStmt(s) => &s.span,
        Stmt::UnsafeBlockStmt(s) => &s.span,
        Stmt::CtStmt(s) => &s.span,
        Stmt::KeyBlockStmt(s) => &s.span,
        Stmt::ErrorStmt(s) => &s.span,
    }
}

/// Tokens that can begin an expression statement.
fn is_expr_start_token(tok: &Token) -> bool {
    match tok.kind {
        TokenKind::Identifier => true,
        _ if is_literal_token(tok) => true,
        TokenKind::Punctuator => matches!(tok.lexeme.as_str(), "(" | "[" | "{"),
        TokenKind::Operator => matches!(tok.lexeme.as_str(), "!" | "-" | "&" | "*" | "^" | "|" | "$"),
        TokenKind::Keyword => matches!(
            tok.lexeme.as_str(),
            "if" | "loop"
                | "unsafe"
                | "move"
                | "transmute"
                | "widen"
                | "comptime"
                | "quote"
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

fn requires_terminator(stmt: &Stmt) -> bool {
    let form = match stmt {
        Stmt::LetStmt(_) => Some("LetStmt"),
        Stmt::VarStmt(_) => Some("VarStmt"),
        Stmt::UsingLocalStmt(_) => Some("UsingLocalStmt"),
        Stmt::AssignStmt(_) => Some("AssignStmt"),
        Stmt::CompoundAssignStmt(_) => Some("CompoundAssignStmt"),
        Stmt::ExprStmt(_) => Some("ExprStmt"),
        _ => None,
    };
    if Conformance::enabled() {
        let payload = format!(
            "source=RequiresTerminator;stmt_form={};requires_terminator={};required_set=LetStmt,VarStmt,UsingLocalStmt,AssignStmt,CompoundAssignStmt,ExprStmt",
            form.unwrap_or("NonReqTermStmt"),
            form.is_some()
        );
        Conformance::record_at("def.18.RequiredStatementTerminators", Some(stmt_span(stmt)), &payload);
    }
    form.is_some()
}

fn wrap_attr_expr(attrs: &AttributeList, expr: &ExprPtr) -> ExprPtr {
    match expr {
        Some(inner) if !attrs.is_empty() => Some(Arc::new(Expr {
            span: inner.span.clone(),
            node: AttributedExpr { attrs: attrs.clone(), expr: expr.clone() }.into(),
        })),
        _ => expr.clone(),
    }
}

fn make_block(span: Span, stmts: Vec<Stmt>, tail_opt: ExprPtr) -> BlockPtr {
    Some(Arc::new(Block { stmts, tail_opt, span }))
}

fn parse_stmt_seq(parser: Parser) -> (Parser, Vec<Stmt>, ExprPtr) {
    let mut stmts = Vec::new();
    let mut cur = parser;
    loop {
        skip_newlines(&mut cur);
        if cur.is_punct("}") {
            spec_rule!("ParseStmtSeq-End");
            return (cur, stmts, None);
        }
        // The reference never terminates on a block left open at end of file; stop here
        // and let the caller report the missing brace.
        if cur.at_eof() {
            return (cur, stmts, None);
        }
        if is_expr_start_token(&cur.tok()) {
            let (tail_parser, tail) = parse_expr(cur.clone_without_diags());
            let tail_advanced = tail_parser.index > cur.index;
            let tail_is_error =
                matches!(tail.as_deref(), Some(Expr { node: ExprNode::ErrorExpr(_), .. }));
            if tail_advanced && !tail_is_error && tail_parser.diags.is_empty() {
                let mut tail_end = tail_parser.clone_without_diags();
                skip_newlines(&mut tail_end);
                if tail_end.is_punct("}") {
                    spec_rule!("ParseStmtSeq-TailExpr");
                    let merged = merge_diag(&cur, &tail_parser, &tail_end);
                    return (merged, stmts, tail);
                }
            }
        }
        spec_rule!("ParseStmtSeq-Cons");
        let (next, stmt) = parse_stmt(cur);
        stmts.push(stmt);
        cur = next;
    }
}

pub fn parse_block(mut parser: Parser) -> Parsed<BlockPtr> {
    skip_newlines(&mut parser);
    if !parser.is_punct("{") {
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span.clone());
        return (parser, make_block(span, Vec::new(), None));
    }
    spec_rule!("Parse-Block");
    let start = parser.clone();
    let (mut seq_parser, stmts, tail_opt) = parse_stmt_seq(parser.advance_or_eof());
    if !seq_parser.is_punct("}") {
        let span = seq_parser.tok_span();
        emit_parse_syntax_err(&mut seq_parser, span);
        let span = span_between(&start, &seq_parser);
        return (seq_parser, make_block(span, stmts, tail_opt));
    }
    let after = seq_parser.advance_or_eof();
    let span = span_between(&start, &after);
    (after, make_block(span, stmts, tail_opt))
}

fn apply_stmt_attrs(attrs: &AttributeList, stmt: &mut Stmt) {
    if attrs.is_empty() {
        return;
    }
    match stmt {
        Stmt::LetStmt(s) => s.binding.attrs = attrs.clone(),
        Stmt::VarStmt(s) => s.binding.attrs = attrs.clone(),
        Stmt::AssignStmt(s) => {
            s.place = wrap_attr_expr(attrs, &s.place);
            s.value = wrap_attr_expr(attrs, &s.value);
        }
        Stmt::CompoundAssignStmt(s) => {
            s.place = wrap_attr_expr(attrs, &s.place);
            s.value = wrap_attr_expr(attrs, &s.value);
        }
        Stmt::ExprStmt(s) => s.value = wrap_attr_expr(attrs, &s.value),
        Stmt::ReturnStmt(s) => s.value_opt = wrap_attr_expr(attrs, &s.value_opt),
        Stmt::BreakStmt(s) => s.value_opt = wrap_attr_expr(attrs, &s.value_opt),
        Stmt::CtStmt(s) => s.attrs = attrs.clone(),
        Stmt::KeyBlockStmt(s) => s.attrs = attrs.clone(),
        _ => {}
    }
}

/// When a binding pattern is `name: T`, the annotation moves to the binding's type slot.
pub(crate) fn normalize_binding_pattern(pat: &mut PatternPtr, type_opt: &mut TypePtr) {
    super::item::normalize_binding_pattern(pat, type_opt);
}

// ---- key blocks -------------------------------------------------------------

fn parse_key_marker_opt(parser: Parser) -> Parsed<bool> {
    if parser.is_op("#") {
        spec_rule!("Parse-KeyMarkerOpt-Yes");
        return (parser.advance_or_eof(), true);
    }
    spec_rule!("Parse-KeyMarkerOpt-No");
    (parser, false)
}

/// True when `[` starts a key option list (`[ordered] {`) rather than an index segment.
fn is_key_options_ahead(parser: &Parser) -> bool {
    if !parser.is_punct("[") {
        return false;
    }
    let mut probe = parser.advance_or_eof();
    loop {
        let tok = probe.tok();
        if tok.kind != TokenKind::Identifier || tok.lexeme != "ordered" {
            return false;
        }
        probe.advance();
        if probe.is_punct(",") {
            probe.advance();
            if probe.is_punct("]") {
                break;
            }
            continue;
        }
        break;
    }
    if !probe.is_punct("]") {
        return false;
    }
    probe.advance();
    skip_newlines(&mut probe);
    probe.is_punct("{")
}

pub(crate) fn parse_key_path_expr(parser: Parser) -> Parsed<KeyPathExpr> {
    spec_rule!("Parse-KeyPathExpr");
    let start = parser.clone();
    let (mut cur, root) = parse_ident(parser);
    let mut segs: Vec<KeySeg> = Vec::new();
    loop {
        if cur.is_punct(".") {
            spec_rule!("Parse-KeySegs-Field");
            spec_rule!("Parse-KeyField");
            let (marker_parser, marked) = parse_key_marker_opt(cur.advance_or_eof());
            let (name_parser, name) = parse_ident(marker_parser);
            segs.push(KeySegField { marked, name }.into());
            cur = name_parser;
            continue;
        }
        if cur.is_punct("[") && !is_key_options_ahead(&cur) {
            spec_rule!("Parse-KeySegs-Index");
            spec_rule!("Parse-KeyIndex");
            let (marker_parser, marked) = parse_key_marker_opt(cur.advance_or_eof());
            let (mut index_parser, expr) = parse_expr(marker_parser);
            if !index_parser.is_punct("]") {
                let span = index_parser.tok_span();
                emit_parse_syntax_err(&mut index_parser, span);
                cur = index_parser;
                break;
            }
            segs.push(KeySegIndex { marked, expr }.into());
            cur = index_parser.advance_or_eof();
            continue;
        }
        spec_rule!("Parse-KeySegs-End");
        break;
    }
    let span = span_between(&start, &cur);
    (cur, KeyPathExpr { root, segs, span })
}

fn parse_key_path_list(parser: Parser) -> Parsed<Vec<KeyPathExpr>> {
    spec_rule!("Parse-KeyPathList-Cons");
    let (mut cur, first) = parse_key_path_expr(parser);
    let mut paths = vec![first];
    while cur.is_punct(",") {
        spec_rule!("Parse-KeyPathListTail-Comma");
        let (next, path) = parse_key_path_expr(cur.advance_or_eof());
        paths.push(path);
        cur = next;
    }
    spec_rule!("Parse-KeyPathListTail-End");
    (cur, paths)
}

fn parse_key_block_head(parser: Parser) -> (Parser, KeyBlockKind, KeyMode) {
    let err = |mut parser: Parser| {
        spec_rule!("Parse-KeyBlockHead-Err");
        let span = parser.tok_span();
        emit_parse_syntax_err(&mut parser, span);
        (parser, KeyBlockKind::Read, KeyMode::Read)
    };
    let percent = parser.tok();
    if percent.kind != TokenKind::Operator || percent.lexeme != "%" {
        return err(parser);
    }
    let after_percent = parser.advance_or_eof();
    let head = after_percent.tok();
    let adjacent =
        percent.span.file == head.span.file && percent.span.end_offset == head.span.start_offset;
    if head.kind != TokenKind::Identifier || !adjacent {
        return err(after_percent);
    }
    match head.lexeme.as_str() {
        "read" => {
            spec_rule!("Parse-KeyBlockHead-Read");
            (after_percent.advance_or_eof(), KeyBlockKind::Read, KeyMode::Read)
        }
        "write" => {
            spec_rule!("Parse-KeyBlockHead-Write");
            (after_percent.advance_or_eof(), KeyBlockKind::Write, KeyMode::Write)
        }
        "release" => {
            spec_rule!("Parse-KeyBlockHead-Release");
            let (mode_parser, mode) = parse_key_mode(after_percent.advance_or_eof());
            (mode_parser, KeyBlockKind::Release, mode)
        }
        "speculative" => {
            spec_rule!("Parse-KeyBlockHead-SpeculativeWrite");
            let mut after_speculative = after_percent.advance_or_eof();
            let mode = after_speculative.tok();
            if mode.kind != TokenKind::Identifier || mode.lexeme != "write" {
                spec_rule!("Parse-KeyBlockHead-Err");
                spec_rule!("rule.19.K-Spec-Write-Required");
                let span = after_speculative.tok_span();
                after_speculative.emit("E-CON-0095", span);
            }
            (after_speculative.advance_or_eof(), KeyBlockKind::SpeculativeWrite, KeyMode::Write)
        }
        _ => err(after_percent),
    }
}

fn parse_key_options_opt(parser: Parser) -> Parsed<KeyBlockOptions> {
    let mut options = KeyBlockOptions::default();
    if !parser.is_punct("[") {
        spec_rule!("Parse-KeyOptionsOpt-None");
        return (parser, options);
    }
    let mut probe = parser.advance_or_eof();
    let err = |mut probe: Parser, options: KeyBlockOptions| {
        let span = probe.tok_span();
        emit_parse_syntax_err(&mut probe, span);
        (probe, options)
    };
    loop {
        let tok = probe.tok();
        if tok.kind != TokenKind::Identifier || tok.lexeme != "ordered" {
            return err(probe, options);
        }
        options.ordered = true;
        spec_rule!("Parse-KeyOption-Ordered");
        probe.advance();
        if probe.is_punct(",") {
            probe.advance();
            if probe.is_punct("]") {
                break;
            }
            continue;
        }
        break;
    }
    if !probe.is_punct("]") {
        return err(probe, options);
    }
    spec_rule!("Parse-KeyOptionsOpt-Some");
    (probe.advance_or_eof(), options)
}

fn parse_key_block_stmt(parser: Parser) -> Parsed<Stmt> {
    spec_rule!("Parse-KeyBlock-Stmt");
    let start = parser.clone();
    let (head_parser, kind, mode) = parse_key_block_head(parser);
    let (mut paths_parser, paths) = parse_key_path_list(head_parser);
    if kind == KeyBlockKind::Release {
        let release_tail = paths_parser.tok();
        if release_tail.kind == TokenKind::Identifier && release_tail.lexeme == "speculative" {
            spec_rule!("rule.19.K-Spec-No-Release");
            paths_parser.emit("E-CON-0094", release_tail.span.clone());
            paths_parser.advance();
        }
    }
    let (options_parser, options) = parse_key_options_opt(paths_parser);
    let (body_parser, body) = parse_block(options_parser);
    let span = span_between(&start, &body_parser);
    if Conformance::enabled() {
        let mut records = vec![
            ("requirement.19.ConflictDetectionNoAdditionalSyntax", "source=ParseKeyBlockStmt;surface=key_block_stmt;additional_syntax=false"),
            ("requirement.19.ConflictDetectionNoAdditionalParsingRules", "source=ParseKeyBlockStmt;parser=key_block_stmt;additional_parsing_rules=false"),
        ];
        if kind == KeyBlockKind::Release {
            records.push(("requirement.19.NestedReleaseNoAdditionalSyntax", "source=ParseKeyBlockStmt;head=release;additional_syntax=false"));
            records.push(("requirement.19.NestedReleaseNoAdditionalParsingRules", "source=ParseKeyBlockStmt;head=release;additional_parsing_rules=false"));
        }
        for (rule, payload) in records {
            Conformance::record_at(rule, Some(&span), payload);
        }
    }
    let stmt = KeyBlockStmt { attrs: Vec::new(), kind, paths, mode, options, body, span };
    (body_parser, stmt.into())
}

// ---- statements -------------------------------------------------------------

fn parse_using_local_stmt(parser: Parser) -> Parsed<Stmt> {
    let start = parser.clone();
    let (mut source_parser, source) = parse_ident(parser.advance_or_eof());
    if !source_parser.is_kw("as") {
        let span = source_parser.tok_span();
        emit_parse_syntax_err(&mut source_parser, span);
        let span = span_between(&start, &source_parser);
        return (source_parser, ErrorStmt { span }.into());
    }
    let (alias_parser, alias) = parse_ident(source_parser.advance_or_eof());
    spec_rule!("Parse-UsingLocal-Stmt");
    let span = span_between(&start, &alias_parser);
    let stmt = UsingLocalStmt { source, source_splice_opt: None, alias, alias_splice_opt: None, span };
    (alias_parser, stmt.into())
}

fn is_assign_op(tok: &Token) -> bool {
    tok.kind == TokenKind::Operator && matches!(tok.lexeme.as_str(), "=" | "+=" | "-=" | "*=" | "/=" | "%=")
}

/// Parses one statement without its terminator. `None` when no statement starts here.
fn parse_stmt_core(parser: Parser) -> Option<Parsed<Stmt>> {
    let start = parser.clone();
    let tok = parser.tok();
    let body_stmt = |next: Parser, build: fn(BlockPtr, Span) -> Stmt| {
        let (block_parser, block) = parse_block(next);
        let span = span_between(&start, &block_parser);
        Some((block_parser, build(block, span)))
    };
    if tok.kind == TokenKind::Keyword && (tok.lexeme == "let" || tok.lexeme == "var") {
        spec_rule!("Parse-Binding-Stmt");
        let (binding_parser, binding) = parse_binding_after_let_var(parser);
        let span = binding.span.clone();
        let stmt = if tok.lexeme == "let" {
            spec_rule!("LetOrVarStmt-Let");
            LetStmt { binding, span }.into()
        } else {
            spec_rule!("LetOrVarStmt-Var");
            VarStmt { binding, span }.into()
        };
        return Some((binding_parser, stmt));
    }
    if parser.is_kw("using") && parser.advance_or_eof().is_ident() {
        return Some(parse_using_local_stmt(parser));
    }
    if parser.is_kw("return") {
        spec_rule!("Parse-Return-Stmt");
        let (expr_parser, value_opt) = parse_expr_opt(parser.advance_or_eof());
        let span = span_between(&start, &expr_parser);
        return Some((expr_parser, ReturnStmt { value_opt, span }.into()));
    }
    if parser.is_kw("break") {
        spec_rule!("Parse-Break-Stmt");
        let (expr_parser, value_opt) = parse_expr_opt(parser.advance_or_eof());
        let span = span_between(&start, &expr_parser);
        return Some((expr_parser, BreakStmt { value_opt, span }.into()));
    }
    if parser.is_kw("continue") {
        spec_rule!("Parse-Continue-Stmt");
        let next = parser.advance_or_eof();
        let span = span_between(&start, &next);
        return Some((next, ContinueStmt { span }.into()));
    }
    if parser.is_kw("unsafe") {
        spec_rule!("Parse-Unsafe-Block");
        return body_stmt(parser.advance_or_eof(), |body, span| UnsafeBlockStmt { body, span }.into());
    }
    if parser.is_kw("defer") {
        spec_rule!("Parse-Defer-Stmt");
        return body_stmt(parser.advance_or_eof(), |body, span| DeferStmt { body, span }.into());
    }
    if parser.is_kw("region") {
        spec_rule!("Parse-Region-Stmt");
        let next = parser.advance_or_eof();
        let (opts_parser, opts_opt) = if next.is_punct("(") {
            let (mut expr_parser, expr) = parse_expr(next.advance_or_eof());
            if expr_parser.is_punct(")") {
                expr_parser.advance();
            } else {
                let span = expr_parser.tok_span();
                emit_parse_syntax_err(&mut expr_parser, span);
            }
            (expr_parser, expr)
        } else {
            (next, None)
        };
        let (alias_parser, alias_opt) = if opts_parser.is_kw("as") {
            let (name_parser, name) = parse_ident(opts_parser.advance_or_eof());
            (name_parser, Some(name))
        } else {
            (opts_parser, None)
        };
        let (block_parser, body) = parse_block(alias_parser);
        let span = span_between(&start, &block_parser);
        let stmt = RegionStmt { opts_opt, alias_opt, alias_splice_opt: None, body, span };
        return Some((block_parser, stmt.into()));
    }
    if parser.is_kw("frame") {
        spec_rule!("Parse-Frame-Stmt");
        return body_stmt(parser.advance_or_eof(), |body, span| {
            FrameStmt { target_opt: None, body, span }.into()
        });
    }
    if parser.is_kw("comptime") && parser.advance_or_eof().is_punct("{") {
        spec_rule!("Parse-CtStmt");
        return body_stmt(parser.advance_or_eof(), |body, span| {
            CtStmt { attrs: Vec::new(), body, span }.into()
        });
    }
    if parser.is_op("%") {
        return Some(parse_key_block_stmt(parser));
    }
    if tok.kind == TokenKind::Identifier {
        let after_name = parser.advance_or_eof();
        if after_name.is_punct(".") {
            let after_dot = after_name.advance_or_eof();
            if after_dot.is_kw("frame") {
                spec_rule!("Parse-Frame-Explicit");
                let (block_parser, body) = parse_block(after_dot.advance_or_eof());
                let span = span_between(&start, &block_parser);
                let stmt = FrameStmt { target_opt: Some(tok.lexeme.clone()), body, span };
                return Some((block_parser, stmt.into()));
            }
        }
    }
    if !is_expr_start_token(&tok) {
        return None;
    }
    let (place_parser, place) = parse_place(parser.clone_without_diags(), true);
    let op = place_parser.tok();
    if is_assign_op(&op) {
        spec_rule!("Parse-Assign-Stmt");
        let mut place_end = merge_diag(&parser, &place_parser, &place_parser);
        if !is_place(&place) {
            emit_parse_syntax_err(&mut place_end, parser.tok_span());
            sync_stmt(&mut place_end);
            let span = span_between(&start, &place_end);
            return Some((place_end, ErrorStmt { span }.into()));
        }
        let (rhs_parser, value) = parse_expr(place_end.advance_or_eof());
        let span = span_between(&start, &rhs_parser);
        let stmt = if op.lexeme != "=" {
            spec_rule!("AssignOrCompound-Compound");
            CompoundAssignStmt { place, op: op.lexeme.clone(), value, span }.into()
        } else {
            spec_rule!("AssignOrCompound-Assign");
            AssignStmt { place, value, span }.into()
        };
        return Some((rhs_parser, stmt));
    }
    let (expr_parser, value) = parse_expr(parser);
    let span = span_between(&start, &expr_parser);
    if matches!(value.as_deref(), Some(Expr { node: ExprNode::ErrorExpr(_), .. })) {
        return Some((expr_parser, ErrorStmt { span }.into()));
    }
    spec_rule!("Parse-Expr-Stmt");
    Some((expr_parser, ExprStmt { value, span }.into()))
}

pub fn parse_stmt(parser: Parser) -> Parsed<Stmt> {
    let (mut parser, attrs) = parse_attribute_list_opt(parser);
    if attrs.is_some() {
        skip_newlines(&mut parser);
    }
    let Some((core_parser, mut stmt)) = parse_stmt_core(parser.clone()) else {
        let span = parser.tok_span();
        spec_rule_at!("Parse-Statement-Err", span);
        spec_rule_at!("rule.18.Parse-Statement-Err", span);
        spec_rule_at!("diag.18.Blocks", span);
        spec_rule_at!("req.16.ControlExpressionDiagnosticOwnership", span);
        emit_generic_parse_syntax_err(&mut parser, span);
        let mut sync = parser.advance_or_eof();
        sync_stmt(&mut sync);
        let span = span_between(&parser, &sync);
        return (sync, ErrorStmt { span }.into());
    };
    if let Some(attrs) = &attrs {
        apply_stmt_attrs(attrs, &mut stmt);
    }
    spec_rule_at!("Parse-Statement", parser.tok_span());
    let policy = if requires_terminator(&stmt) {
        TerminatorPolicy::Required
    } else {
        TerminatorPolicy::Optional
    };
    let mut next = core_parser;
    consume_terminator_opt(&mut next, policy);
    (next, stmt)
}
