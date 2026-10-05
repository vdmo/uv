//! Quoted syntax: parsing the tokens of a `quote`, and building the quoted tree by
//! evaluating and substituting every splice in it.

use std::sync::Arc;

use uv_core::ident::is_name;
use uv_core::span::Span;
use uv_core::spec_rule_at;
use uv_core::spec_trace::Conformance;
use uv_source::ast::*;
use uv_source::lexer::{Token, TokenKind};
use uv_source::parser::{parse_expr, parse_item, parse_pattern, parse_stmt, parse_type, Parser};

use crate::eval::eval_expr;
use crate::value::*;

fn make_token_parser(tokens: &[Token], quote_mode: bool) -> Parser {
    let mut parser = Parser::from_tokens(tokens);
    parser.quote_mode = quote_mode;
    parser
}

fn at_eof_after_trailing_newlines(mut parser: Parser) -> bool {
    while !parser.at_eof() && parser.tok().kind == TokenKind::Newline {
        parser.advance();
    }
    parser.at_eof()
}

fn quote_splice_trace_name(expr: &ExprPtr) -> String {
    match expr.as_deref() {
        Some(Expr { node: ExprNode::IdentifierExpr(ident), .. }) => ident.name.clone(),
        _ => "splice".to_string(),
    }
}

fn record_quote_build_result(result: Option<CtAst>, quote: &QuoteExpr, env: &CtEnv) -> Option<CtAst> {
    if result.is_some() && !env.quote_splice_trace.is_empty() {
        Conformance::record_at(
            "requirement.22.QuoteBuildSpliceOrder",
            quote.tokens.first().map(|token| &token.span),
            &format!("splices:{}", env.quote_splice_trace.join(",")),
        );
    }
    result
}

fn emit_splice_context_diag(env: &CtEnv, span: &Span) {
    spec_rule_at!("requirement.22.SpliceContextAndTypeCompatibility", span);
    emit_comptime_diag(env, "E-CTE-0230", span);
}

fn is_quoted_statement_form(stmt: &Stmt) -> bool {
    !matches!(stmt, Stmt::ErrorStmt(_))
}

/// When the whole quote body is one `$(expr)`, the expression inside the splice.
fn parse_whole_body_splice_expr(quote: &QuoteExpr) -> Option<ExprPtr> {
    let parser = make_token_parser(&quote.tokens, true);
    let start_index = parser.index;
    let (after, parsed) = parse_expr(parser);
    let parsed = parsed?;
    if after.index == start_index || !at_eof_after_trailing_newlines(after) {
        return None;
    }
    match &parsed.node {
        ExprNode::SpliceExprNode(splice) => Some(splice.expr.clone()),
        _ => None,
    }
}

fn quote_parses_as_kind(quote: &QuoteExpr, kind: QuoteKind) -> bool {
    if kind == QuoteKind::Item && parse_whole_body_splice_expr(quote).is_some() {
        return true;
    }
    let parser = make_token_parser(&quote.tokens, true);
    let start_index = parser.index;
    match kind {
        QuoteKind::Expr => {
            let (after, parsed) = parse_expr(parser);
            after.index != start_index && parsed.is_some() && at_eof_after_trailing_newlines(after)
        }
        QuoteKind::Stmt => {
            let (after, parsed) = parse_stmt(parser);
            after.index != start_index && is_quoted_statement_form(&parsed) && at_eof_after_trailing_newlines(after)
        }
        QuoteKind::Unspecified => false,
        QuoteKind::Item => {
            let (after, parsed) = parse_item(parser);
            after.index != start_index
                && !matches!(parsed, ASTItem::ErrorItem(_))
                && at_eof_after_trailing_newlines(after)
        }
        QuoteKind::Type => {
            let (after, parsed) = parse_type(parser);
            after.index != start_index && parsed.is_some() && at_eof_after_trailing_newlines(after)
        }
        QuoteKind::Pattern => {
            let (after, parsed) = parse_pattern(parser);
            after.index != start_index && parsed.is_some() && at_eof_after_trailing_newlines(after)
        }
    }
}

fn eval_splice_value(expr: &ExprPtr, env: &mut CtEnv, span: &Span) -> Option<CtValue> {
    env.quote_splice_trace.push(quote_splice_trace_name(expr));
    let value = eval_expr(expr, env);
    if !value.ok {
        emit_comptime_diag(env, "E-CTE-0231", span);
        return None;
    }
    Some(value.value)
}

fn render_expr_splice(value: &CtValue, span: &Span) -> ExprPtr {
    match value {
        CtValue::Ast(CtAst { kind: CtAstKind::Expr, payload: CtAstPayload::Expr(expr), .. }) => expr.clone(),
        CtValue::Ast(_) => None,
        _ => literalize_value(value, span),
    }
}

fn render_stmt_splice(value: &CtValue) -> Option<Stmt> {
    match value {
        CtValue::Ast(CtAst { kind: CtAstKind::Stmt, payload: CtAstPayload::Stmt(stmt), .. }) => Some((**stmt).clone()),
        CtValue::Ast(CtAst { kind: CtAstKind::Expr, payload: CtAstPayload::Expr(Some(expr)), .. }) => {
            Some(ExprStmt { value: Some(expr.clone()), span: expr.span.clone() }.into())
        }
        _ => None,
    }
}

fn render_item_splice(value: &CtValue) -> Option<ASTItem> {
    match value {
        CtValue::Ast(CtAst { kind: CtAstKind::Item, payload: CtAstPayload::Item(item), .. }) => Some((**item).clone()),
        _ => None,
    }
}

fn render_type_splice(value: &CtValue) -> TypePtr {
    match value {
        CtValue::Ast(CtAst { kind: CtAstKind::Type, payload: CtAstPayload::Type(ty), .. }) => ty.clone(),
        CtValue::Type(ty) => ty.clone(),
        _ => None,
    }
}

fn render_pattern_splice(value: &CtValue) -> PatternPtr {
    match value {
        CtValue::Ast(CtAst { kind: CtAstKind::Pattern, payload: CtAstPayload::Pattern(pattern), .. }) => {
            pattern.clone()
        }
        _ => None,
    }
}

/// Replaces a spliced identifier (`$name`) by the string its expression evaluates to.
fn build_spliced_identifier(splice_opt: &Option<SpliceIdentNode>, name: &mut Identifier, env: &mut CtEnv) -> bool {
    let Some(splice) = splice_opt else {
        return true;
    };
    let Some(value) = eval_splice_value(&splice.name_expr, env, &splice.span) else {
        return false;
    };
    let CtValue::String(text) = &value else {
        emit_splice_context_diag(env, &splice.span);
        return false;
    };
    if !is_name(text) {
        emit_comptime_diag(env, "E-CTE-0232", &splice.span);
        return false;
    }
    *name = text.clone();
    spec_rule_at!("requirement.22.StringSpliceIdentifierHygiene", &splice.span);
    true
}

fn build_expr_in_place(expr: &mut ExprPtr, env: &mut CtEnv) -> bool {
    if expr.is_none() {
        return true;
    }
    match build_expr(expr, env) {
        Some(built) => {
            *expr = built;
            true
        }
        None => false,
    }
}

fn build_type_in_place(ty: &mut TypePtr, env: &mut CtEnv) -> bool {
    if ty.is_none() {
        return true;
    }
    match build_type(ty, env) {
        Some(built) => {
            *ty = built;
            true
        }
        None => false,
    }
}

fn build_pattern_in_place(pattern: &mut PatternPtr, env: &mut CtEnv) -> bool {
    if pattern.is_none() {
        return true;
    }
    match build_pattern(pattern, env) {
        Some(built) => {
            *pattern = built;
            true
        }
        None => false,
    }
}

fn build_exprs_in_place(exprs: &mut [ExprPtr], env: &mut CtEnv) -> bool {
    exprs.iter_mut().all(|expr| build_expr_in_place(expr, env))
}

fn build_types_in_place(types: &mut [TypePtr], env: &mut CtEnv) -> bool {
    types.iter_mut().all(|ty| build_type_in_place(ty, env))
}

fn build_patterns_in_place(patterns: &mut [PatternPtr], env: &mut CtEnv) -> bool {
    patterns.iter_mut().all(|pattern| build_pattern_in_place(pattern, env))
}

fn build_args_in_place(args: &mut [Arg], env: &mut CtEnv) -> bool {
    args.iter_mut().all(|arg| build_expr_in_place(&mut arg.value, env))
}

fn build_field_inits_in_place(fields: &mut [FieldInit], env: &mut CtEnv) -> bool {
    fields.iter_mut().all(|field| build_expr_in_place(&mut field.value, env))
}

fn build_field_patterns_in_place(fields: &mut [FieldPattern], env: &mut CtEnv) -> bool {
    fields.iter_mut().all(|field| build_pattern_in_place(&mut field.pattern_opt, env))
}

fn build_apply_args_in_place(args: &mut ApplyArgs, env: &mut CtEnv) -> bool {
    match args {
        ApplyArgs::ParenArgs(paren) => build_args_in_place(&mut paren.args, env),
        ApplyArgs::BraceArgs(brace) => build_field_inits_in_place(&mut brace.fields, env),
    }
}

fn build_enum_payload_in_place(payload_opt: &mut Option<EnumPayload>, env: &mut CtEnv) -> bool {
    match payload_opt {
        None => true,
        Some(EnumPayload::EnumPayloadParen(paren)) => build_exprs_in_place(&mut paren.elements, env),
        Some(EnumPayload::EnumPayloadBrace(brace)) => build_field_inits_in_place(&mut brace.fields, env),
    }
}

fn build_loop_invariant_in_place(invariant_opt: &mut Option<LoopInvariant>, env: &mut CtEnv) -> bool {
    invariant_opt.as_mut().is_none_or(|invariant| build_expr_in_place(&mut invariant.predicate, env))
}

fn build_key_path_in_place(path: &mut KeyPathExpr, env: &mut CtEnv) -> bool {
    path.segs.iter_mut().all(|seg| match seg {
        KeySeg::KeySegIndex(index) => build_expr_in_place(&mut index.expr, env),
        KeySeg::KeySegField(_) => true,
    })
}

fn build_block_in_place(block: &mut BlockPtr, env: &mut CtEnv) -> bool {
    let source = block.as_deref().expect("block expected in quoted syntax");
    match build_block(source, env) {
        Some(built) => {
            *block = Some(Arc::new(built));
            true
        }
        None => false,
    }
}

fn build_opt_block_in_place(block: &mut BlockPtr, env: &mut CtEnv) -> bool {
    block.is_none() || build_block_in_place(block, env)
}

fn build_type(ty_ptr: &TypePtr, env: &mut CtEnv) -> Option<TypePtr> {
    let Some(ty) = ty_ptr else {
        return Some(None);
    };
    let make = |node: TypeNode| Some(Some(Arc::new(Type { span: ty.span.clone(), node })));
    match &ty.node {
        TypeNode::SpliceExprNode(node) => {
            let value = eval_splice_value(&node.expr, env, &node.span)?;
            let rendered = render_type_splice(&value);
            if rendered.is_none() {
                emit_splice_context_diag(env, &node.span);
                return None;
            }
            Some(rendered)
        }
        TypeNode::TypePermType(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.base, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeUnion(node) => {
            let mut out = node.clone();
            build_types_in_place(&mut out.types, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeFunc(node) => {
            let mut out = node.clone();
            // As in the reference, the return type is built once per parameter, and not at
            // all for a parameterless function type.
            for index in 0..out.params.len() {
                if !build_type_in_place(&mut out.params[index].r#type, env) || !build_type_in_place(&mut out.ret, env) {
                    return None;
                }
            }
            make(out.into())
        }
        TypeNode::TypeClosure(node) => {
            let mut out = node.clone();
            for param in &mut out.params {
                build_type_in_place(&mut param.r#type, env).then_some(())?;
            }
            build_type_in_place(&mut out.ret, env).then_some(())?;
            if let Some(deps) = &mut out.deps_opt {
                for dep in deps {
                    build_type_in_place(&mut dep.r#type, env).then_some(())?;
                }
            }
            make(out.into())
        }
        TypeNode::TypeTuple(node) => {
            let mut out = node.clone();
            build_types_in_place(&mut out.elements, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeArray(node) => {
            let mut out = node.clone();
            (build_type_in_place(&mut out.element, env) && build_expr_in_place(&mut out.length, env)).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeSlice(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.element, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeSafePtr(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.element, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRawPtr(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.element, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeModalState(node) => {
            let mut out = node.clone();
            build_types_in_place(&mut out.generic_args, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypePathType(node) => {
            let mut out = node.clone();
            build_types_in_place(&mut out.generic_args, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeApply(node) => {
            let mut out = node.clone();
            build_types_in_place(&mut out.args, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRefine(node) => {
            let mut out = node.clone();
            (build_type_in_place(&mut out.base, env) && build_expr_in_place(&mut out.predicate, env)).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRange(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.base, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRangeInclusive(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.base, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRangeFrom(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.base, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRangeTo(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.base, env).then_some(())?;
            make(out.into())
        }
        TypeNode::TypeRangeToInclusive(node) => {
            let mut out = node.clone();
            build_type_in_place(&mut out.base, env).then_some(())?;
            make(out.into())
        }
        _ => Some(ty_ptr.clone()),
    }
}

fn build_pattern(pattern_ptr: &PatternPtr, env: &mut CtEnv) -> Option<PatternPtr> {
    let Some(pattern) = pattern_ptr else {
        return Some(None);
    };
    let make = |node: PatternNode| Some(Some(Arc::new(Pattern { span: pattern.span.clone(), node })));
    match &pattern.node {
        PatternNode::SpliceExprNode(node) => {
            let value = eval_splice_value(&node.expr, env, &node.span)?;
            let rendered = render_pattern_splice(&value);
            if rendered.is_none() {
                emit_splice_context_diag(env, &node.span);
                return None;
            }
            Some(rendered)
        }
        PatternNode::IdentifierPattern(node) => {
            let mut out = node.clone();
            build_spliced_identifier(&node.name_splice_opt, &mut out.name, env).then_some(())?;
            make(out.into())
        }
        PatternNode::TypedPattern(node) => {
            let mut out = node.clone();
            (build_spliced_identifier(&node.name_splice_opt, &mut out.name, env)
                && build_type_in_place(&mut out.r#type, env))
            .then_some(())?;
            make(out.into())
        }
        PatternNode::TuplePattern(node) => {
            let mut out = node.clone();
            build_patterns_in_place(&mut out.elements, env).then_some(())?;
            make(out.into())
        }
        PatternNode::RecordPattern(node) => {
            let mut out = node.clone();
            build_field_patterns_in_place(&mut out.fields, env).then_some(())?;
            make(out.into())
        }
        PatternNode::EnumPattern(node) => {
            let mut out = node.clone();
            let ok = match &mut out.payload_opt {
                None => true,
                Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                    build_patterns_in_place(&mut payload.elements, env)
                }
                Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                    build_field_patterns_in_place(&mut payload.fields, env)
                }
            };
            ok.then_some(())?;
            make(out.into())
        }
        PatternNode::ModalPattern(node) => {
            let mut out = node.clone();
            if let Some(payload) = &mut out.fields_opt {
                build_field_patterns_in_place(&mut payload.fields, env).then_some(())?;
            }
            make(out.into())
        }
        PatternNode::RangePattern(node) => {
            let mut out = node.clone();
            (build_pattern_in_place(&mut out.lo, env) && build_pattern_in_place(&mut out.hi, env)).then_some(())?;
            make(out.into())
        }
        _ => Some(pattern_ptr.clone()),
    }
}

/// A statement-position splice: the spliced value must render as a statement.
fn build_stmt_splice(splice: &SpliceExprNode, env: &mut CtEnv) -> Option<Stmt> {
    let value = eval_splice_value(&splice.expr, env, &splice.span)?;
    let rendered = render_stmt_splice(&value);
    if rendered.is_none() {
        emit_splice_context_diag(env, &splice.span);
    }
    rendered
}

fn build_binding_in_place(binding: &mut Binding, env: &mut CtEnv) -> bool {
    build_pattern_in_place(&mut binding.pat, env)
        && build_type_in_place(&mut binding.type_opt, env)
        && build_expr_in_place(&mut binding.init, env)
}

fn build_stmt(stmt: &Stmt, env: &mut CtEnv) -> Option<Stmt> {
    match stmt {
        Stmt::LetStmt(node) => {
            let mut out = node.clone();
            build_binding_in_place(&mut out.binding, env).then_some(())?;
            Some(out.into())
        }
        Stmt::VarStmt(node) => {
            let mut out = node.clone();
            build_binding_in_place(&mut out.binding, env).then_some(())?;
            Some(out.into())
        }
        Stmt::UsingLocalStmt(node) => {
            let mut out = node.clone();
            (build_spliced_identifier(&node.source_splice_opt, &mut out.source, env)
                && build_spliced_identifier(&node.alias_splice_opt, &mut out.alias, env))
            .then_some(())?;
            Some(out.into())
        }
        Stmt::AssignStmt(node) => {
            let mut out = node.clone();
            (build_expr_in_place(&mut out.place, env) && build_expr_in_place(&mut out.value, env)).then_some(())?;
            Some(out.into())
        }
        Stmt::CompoundAssignStmt(node) => {
            let mut out = node.clone();
            (build_expr_in_place(&mut out.place, env) && build_expr_in_place(&mut out.value, env)).then_some(())?;
            Some(out.into())
        }
        Stmt::ExprStmt(node) => {
            if let Some(Expr { node: ExprNode::SpliceExprNode(splice), .. }) = node.value.as_deref() {
                return build_stmt_splice(splice, env);
            }
            let mut out = node.clone();
            build_expr_in_place(&mut out.value, env).then_some(())?;
            Some(out.into())
        }
        Stmt::DeferStmt(node) => {
            let mut out = node.clone();
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
        Stmt::RegionStmt(node) => {
            let mut out = node.clone();
            build_expr_in_place(&mut out.opts_opt, env).then_some(())?;
            if node.alias_splice_opt.is_some() {
                let mut alias = "_".to_string();
                build_spliced_identifier(&node.alias_splice_opt, &mut alias, env).then_some(())?;
                out.alias_opt = Some(alias);
            }
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
        Stmt::FrameStmt(node) => {
            let mut out = node.clone();
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
        Stmt::ReturnStmt(node) => {
            let mut out = node.clone();
            build_expr_in_place(&mut out.value_opt, env).then_some(())?;
            Some(out.into())
        }
        Stmt::BreakStmt(node) => {
            let mut out = node.clone();
            build_expr_in_place(&mut out.value_opt, env).then_some(())?;
            Some(out.into())
        }
        Stmt::ContinueStmt(_) | Stmt::ErrorStmt(_) => Some(stmt.clone()),
        Stmt::UnsafeBlockStmt(node) => {
            let mut out = node.clone();
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
        Stmt::CtStmt(node) => {
            let mut out = node.clone();
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
        Stmt::KeyBlockStmt(node) => {
            let mut out = node.clone();
            for path in &mut out.paths {
                build_key_path_in_place(path, env).then_some(())?;
            }
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
    }
}

fn build_block(block: &Block, env: &mut CtEnv) -> Option<Block> {
    let mut out = block.clone();
    for stmt in &mut out.stmts {
        *stmt = build_stmt(stmt, env)?;
    }
    // A splice in tail position renders as the block's last statement.
    if let Some(Expr { node: ExprNode::SpliceExprNode(splice), .. }) = out.tail_opt.as_deref() {
        let rendered = build_stmt_splice(splice, env)?;
        out.stmts.push(rendered);
        out.tail_opt = None;
        return Some(out);
    }
    build_expr_in_place(&mut out.tail_opt, env).then_some(())?;
    Some(out)
}

fn build_param_in_place(param: &mut Param, env: &mut CtEnv) -> bool {
    let splice = param.name_splice_opt.clone();
    build_spliced_identifier(&splice, &mut param.name, env) && build_type_in_place(&mut param.r#type, env)
}

fn build_params_in_place(params: &mut [Param], env: &mut CtEnv) -> bool {
    params.iter_mut().all(|param| build_param_in_place(param, env))
}

fn build_generic_params_in_place(params_opt: &mut Option<GenericParams>, env: &mut CtEnv) -> bool {
    params_opt
        .as_mut()
        .is_none_or(|params| params.params.iter_mut().all(|param| build_type_in_place(&mut param.default_type, env)))
}

fn build_contract_clause_in_place(clause_opt: &mut Option<ContractClause>, env: &mut CtEnv) -> bool {
    clause_opt.as_mut().is_none_or(|clause| {
        build_expr_in_place(&mut clause.precondition, env) && build_expr_in_place(&mut clause.postcondition, env)
    })
}

fn build_type_invariant_in_place(invariant_opt: &mut Option<TypeInvariant>, env: &mut CtEnv) -> bool {
    invariant_opt.as_mut().is_none_or(|invariant| build_expr_in_place(&mut invariant.predicate, env))
}

fn build_receiver_in_place(receiver: &mut Receiver, env: &mut CtEnv) -> bool {
    match receiver {
        Receiver::ReceiverExplicit(explicit) => build_type_in_place(&mut explicit.r#type, env),
        Receiver::ReceiverShorthand(_) => true,
    }
}

fn build_field_decl_in_place(field: &mut FieldDecl, env: &mut CtEnv) -> bool {
    build_type_in_place(&mut field.r#type, env) && build_expr_in_place(&mut field.init_opt, env)
}

/// The parts shared by every procedure-like declaration, in the reference's order.
struct SignatureParts<'a> {
    generic_params: Option<&'a mut Option<GenericParams>>,
    receiver: Option<&'a mut Receiver>,
    params: &'a mut [Param],
    return_type_opt: &'a mut TypePtr,
    contract: &'a mut Option<ContractClause>,
}

fn build_signature_in_place(parts: SignatureParts<'_>, env: &mut CtEnv) -> bool {
    parts.generic_params.is_none_or(|generics| build_generic_params_in_place(generics, env))
        && parts.receiver.is_none_or(|receiver| build_receiver_in_place(receiver, env))
        && build_params_in_place(parts.params, env)
        && build_type_in_place(parts.return_type_opt, env)
        && build_contract_clause_in_place(parts.contract, env)
}

fn build_record_member_in_place(member: &mut RecordMember, env: &mut CtEnv) -> bool {
    match member {
        RecordMember::FieldDecl(node) => build_field_decl_in_place(node, env),
        RecordMember::MethodDecl(node) => {
            build_signature_in_place(
                SignatureParts {
                    generic_params: Some(&mut node.generic_params),
                    receiver: Some(&mut node.receiver),
                    params: &mut node.params,
                    return_type_opt: &mut node.return_type_opt,
                    contract: &mut node.contract,
                },
                env,
            ) && build_block_in_place(&mut node.body, env)
        }
        RecordMember::AssociatedTypeDecl(node) => build_type_in_place(&mut node.default_type, env),
    }
}

fn build_state_member_in_place(member: &mut StateMember, env: &mut CtEnv) -> bool {
    match member {
        StateMember::StateFieldDecl(node) => build_type_in_place(&mut node.r#type, env),
        StateMember::StateMethodDecl(node) => {
            build_signature_in_place(
                SignatureParts {
                    generic_params: Some(&mut node.generic_params),
                    receiver: Some(&mut node.receiver),
                    params: &mut node.params,
                    return_type_opt: &mut node.return_type_opt,
                    contract: &mut node.contract,
                },
                env,
            ) && build_block_in_place(&mut node.body, env)
        }
        StateMember::TransitionDecl(node) => {
            build_params_in_place(&mut node.params, env) && build_block_in_place(&mut node.body, env)
        }
    }
}

fn build_class_item_in_place(item: &mut ClassItem, env: &mut CtEnv) -> bool {
    match item {
        ClassItem::ClassFieldDecl(node) => build_type_in_place(&mut node.r#type, env),
        ClassItem::ClassMethodDecl(node) => {
            build_signature_in_place(
                SignatureParts {
                    generic_params: Some(&mut node.generic_params),
                    receiver: Some(&mut node.receiver),
                    params: &mut node.params,
                    return_type_opt: &mut node.return_type_opt,
                    contract: &mut node.contract,
                },
                env,
            ) && build_opt_block_in_place(&mut node.body_opt, env)
        }
        ClassItem::AssociatedTypeDecl(node) => build_type_in_place(&mut node.default_type, env),
        ClassItem::AbstractFieldDecl(node) => build_type_in_place(&mut node.r#type, env),
        ClassItem::AbstractStateDecl(node) => {
            node.fields.iter_mut().all(|field| build_type_in_place(&mut field.r#type, env))
        }
    }
}

fn build_item(item: &ASTItem, env: &mut CtEnv) -> Option<ASTItem> {
    match item {
        ASTItem::UsingDecl(_) | ASTItem::ImportDecl(_) | ASTItem::ErrorItem(_) => Some(item.clone()),
        ASTItem::ExternBlock(node) => {
            let mut out = node.clone();
            for ExternItem::ExternProcDecl(proc) in &mut out.items {
                build_signature_in_place(
                    SignatureParts {
                        generic_params: Some(&mut proc.generic_params),
                        receiver: None,
                        params: &mut proc.params,
                        return_type_opt: &mut proc.return_type_opt,
                        contract: &mut proc.contract,
                    },
                    env,
                )
                .then_some(())?;
                for clause in proc.foreign_contracts_opt.iter_mut().flatten() {
                    build_exprs_in_place(&mut clause.predicates, env).then_some(())?;
                }
            }
            Some(out.into())
        }
        ASTItem::StaticDecl(node) => {
            let mut out = node.clone();
            build_binding_in_place(&mut out.binding, env).then_some(())?;
            Some(out.into())
        }
        ASTItem::ProcedureDecl(node) => {
            let mut out = node.clone();
            (build_signature_in_place(
                SignatureParts {
                    generic_params: Some(&mut out.generic_params),
                    receiver: None,
                    params: &mut out.params,
                    return_type_opt: &mut out.return_type_opt,
                    contract: &mut out.contract,
                },
                env,
            ) && build_block_in_place(&mut out.body, env))
            .then_some(())?;
            Some(out.into())
        }
        ASTItem::ComptimeProcedureDecl(node) => {
            let mut out = node.clone();
            (build_signature_in_place(
                SignatureParts {
                    generic_params: Some(&mut out.generic_params),
                    receiver: None,
                    params: &mut out.params,
                    return_type_opt: &mut out.return_type_opt,
                    contract: &mut out.contract,
                },
                env,
            ) && build_block_in_place(&mut out.body, env))
            .then_some(())?;
            Some(out.into())
        }
        ASTItem::RecordDecl(node) => {
            let mut out = node.clone();
            (build_generic_params_in_place(&mut out.generic_params, env)
                && build_type_invariant_in_place(&mut out.invariant_opt, env)
                && out.members.iter_mut().all(|member| build_record_member_in_place(member, env)))
            .then_some(())?;
            Some(out.into())
        }
        ASTItem::EnumDecl(node) => {
            let mut out = node.clone();
            (build_generic_params_in_place(&mut out.generic_params, env)
                && build_type_invariant_in_place(&mut out.invariant_opt, env))
            .then_some(())?;
            for variant in &mut out.variants {
                let ok = match &mut variant.payload_opt {
                    None => true,
                    Some(VariantPayload::VariantPayloadTuple(payload)) => {
                        build_types_in_place(&mut payload.elements, env)
                    }
                    Some(VariantPayload::VariantPayloadRecord(payload)) => {
                        payload.fields.iter_mut().all(|field| build_field_decl_in_place(field, env))
                    }
                };
                ok.then_some(())?;
            }
            Some(out.into())
        }
        ASTItem::ModalDecl(node) => {
            let mut out = node.clone();
            (build_generic_params_in_place(&mut out.generic_params, env)
                && build_type_invariant_in_place(&mut out.invariant_opt, env))
            .then_some(())?;
            for state in &mut out.states {
                for member in &mut state.members {
                    build_state_member_in_place(member, env).then_some(())?;
                }
            }
            Some(out.into())
        }
        ASTItem::ClassDecl(node) => {
            let mut out = node.clone();
            (build_generic_params_in_place(&mut out.generic_params, env)
                && out.items.iter_mut().all(|class_item| build_class_item_in_place(class_item, env)))
            .then_some(())?;
            Some(out.into())
        }
        ASTItem::TypeAliasDecl(node) => {
            let mut out = node.clone();
            (build_generic_params_in_place(&mut out.generic_params, env) && build_type_in_place(&mut out.r#type, env))
                .then_some(())?;
            Some(out.into())
        }
        ASTItem::DeriveTargetDecl(node) => {
            let mut out = node.clone();
            build_block_in_place(&mut out.body, env).then_some(())?;
            Some(out.into())
        }
    }
}

fn build_expr(expr_ptr: &ExprPtr, env: &mut CtEnv) -> Option<ExprPtr> {
    let Some(expr) = expr_ptr else {
        return Some(None);
    };
    let make = |node: ExprNode| Some(Some(Arc::new(Expr { span: expr.span.clone(), node })));
    // Clones the node, rebuilds the listed parts in order and wraps the result.
    macro_rules! rebuild {
        ($node:expr, |$out:ident| $ok:expr) => {{
            let mut $out = $node.clone();
            if !($ok) {
                return None;
            }
            make($out.into())
        }};
    }
    match &expr.node {
        ExprNode::SpliceExprNode(node) => {
            let value = eval_splice_value(&node.expr, env, &node.span)?;
            let rendered = render_expr_splice(&value, &node.span);
            if rendered.is_none() {
                emit_splice_context_diag(env, &node.span);
                return None;
            }
            Some(rendered)
        }
        ExprNode::SpliceIdentNode(node) => {
            let value = eval_splice_value(&node.name_expr, env, &node.span)?;
            let CtValue::String(text) = &value else {
                emit_splice_context_diag(env, &node.span);
                return None;
            };
            if !is_name(text) {
                emit_comptime_diag(env, "E-CTE-0232", &node.span);
                return None;
            }
            make(IdentifierExpr { name: text.clone(), from_splice: true }.into())
        }
        ExprNode::BinaryExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.lhs, env) && build_expr_in_place(&mut out.rhs, env))
        }
        ExprNode::QualifiedApplyExpr(node) => rebuild!(node, |out| build_apply_args_in_place(&mut out.args, env)),
        ExprNode::CallExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.callee, env)
            && build_types_in_place(&mut out.generic_args, env)
            && build_args_in_place(&mut out.args, env)),
        ExprNode::MethodCallExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.receiver, env) && build_args_in_place(&mut out.args, env))
        }
        ExprNode::RangeExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.lhs, env) && build_expr_in_place(&mut out.rhs, env))
        }
        ExprNode::CastExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.value, env) && build_type_in_place(&mut out.r#type, env))
        }
        ExprNode::DerefExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::AddressOfExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.place, env)),
        ExprNode::MoveExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.place, env)),
        ExprNode::AllocExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::TupleExpr(node) => rebuild!(node, |out| build_exprs_in_place(&mut out.elements, env)),
        ExprNode::ArrayExpr(node) => rebuild!(node, |out| out.elements.iter_mut().all(|segment| match segment {
            ArraySegment::ArrayElemSegment(elem) => build_expr_in_place(&mut elem.value, env),
            ArraySegment::ArrayRepeatSegment(repeat) => {
                build_expr_in_place(&mut repeat.value, env) && build_expr_in_place(&mut repeat.count, env)
            }
        })),
        ExprNode::ArrayRepeatExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.value, env) && build_expr_in_place(&mut out.count, env))
        }
        ExprNode::RecordExpr(node) => rebuild!(node, |out| build_field_inits_in_place(&mut out.fields, env)
            && match &mut out.target {
                RecordExprTarget::ModalStateRef(target) => build_types_in_place(&mut target.generic_args, env),
                RecordExprTarget::Path(_) => true,
            }),
        ExprNode::EnumLiteralExpr(node) => rebuild!(node, |out| build_enum_payload_in_place(&mut out.payload_opt, env)),
        ExprNode::SizeofExpr(node) => rebuild!(node, |out| build_type_in_place(&mut out.r#type, env)),
        ExprNode::AlignofExpr(node) => rebuild!(node, |out| build_type_in_place(&mut out.r#type, env)),
        ExprNode::IfExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.cond, env)
            && build_expr_in_place(&mut out.then_expr, env)
            && build_expr_in_place(&mut out.else_expr, env)),
        ExprNode::IfIsExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.scrutinee, env)
            && build_pattern_in_place(&mut out.pattern, env)
            && build_expr_in_place(&mut out.then_expr, env)
            && build_expr_in_place(&mut out.else_expr, env)),
        ExprNode::IfCaseExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.scrutinee, env)
            && out.cases.iter_mut().all(|clause| {
                build_pattern_in_place(&mut clause.pattern, env) && build_expr_in_place(&mut clause.body, env)
            })
            && build_expr_in_place(&mut out.else_expr, env)),
        ExprNode::LoopInfiniteExpr(node) => rebuild!(node, |out| build_loop_invariant_in_place(
            &mut out.invariant_opt,
            env
        ) && build_block_in_place(&mut out.body, env)),
        ExprNode::LoopConditionalExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.cond, env)
            && build_loop_invariant_in_place(&mut out.invariant_opt, env)
            && build_block_in_place(&mut out.body, env)),
        ExprNode::LoopIterExpr(node) => rebuild!(node, |out| build_pattern_in_place(&mut out.pattern, env)
            && build_type_in_place(&mut out.type_opt, env)
            && build_expr_in_place(&mut out.iter, env)
            && build_loop_invariant_in_place(&mut out.invariant_opt, env)
            && build_block_in_place(&mut out.body, env)),
        ExprNode::UnsafeBlockExpr(node) => rebuild!(node, |out| build_block_in_place(&mut out.block, env)),
        ExprNode::BlockExpr(node) => rebuild!(node, |out| build_block_in_place(&mut out.block, env)),
        ExprNode::ComptimeExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.body, env)),
        ExprNode::CtIfExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.cond, env)
            && build_block_in_place(&mut out.then_block, env)
            && build_opt_block_in_place(&mut out.else_block_opt, env)),
        ExprNode::CtLoopIterExpr(node) => rebuild!(node, |out| build_pattern_in_place(&mut out.pattern, env)
            && build_type_in_place(&mut out.type_opt, env)
            && build_expr_in_place(&mut out.iter, env)
            && build_block_in_place(&mut out.body, env)),
        ExprNode::AttributedExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.expr, env)),
        ExprNode::EntryExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.expr, env)),
        ExprNode::TypeLiteralExpr(node) => rebuild!(node, |out| build_type_in_place(&mut out.r#type, env)),
        ExprNode::TransmuteExpr(node) => rebuild!(node, |out| build_type_in_place(&mut out.from, env)
            && build_type_in_place(&mut out.to, env)
            && build_expr_in_place(&mut out.value, env)),
        ExprNode::ClosureExpr(node) => rebuild!(node, |out| out
            .params
            .iter_mut()
            .all(|param| build_type_in_place(&mut param.type_opt, env))
            && build_type_in_place(&mut out.ret_type_opt, env)
            && build_expr_in_place(&mut out.body, env)),
        ExprNode::PipelineExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.lhs, env) && build_expr_in_place(&mut out.rhs, env))
        }
        ExprNode::FieldAccessExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.base, env)),
        ExprNode::TupleAccessExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.base, env)),
        ExprNode::IndexAccessExpr(node) => {
            rebuild!(node, |out| build_expr_in_place(&mut out.base, env) && build_expr_in_place(&mut out.index, env))
        }
        ExprNode::CallTypeArgsExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.callee, env)
            && build_types_in_place(&mut out.type_args, env)
            && build_args_in_place(&mut out.args, env)),
        ExprNode::UnaryExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::PropagateExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::YieldExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::YieldFromExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::SyncExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.value, env)),
        ExprNode::RaceExpr(node) => rebuild!(node, |out| out.arms.iter_mut().all(|arm| {
            build_expr_in_place(&mut arm.expr, env)
                && build_pattern_in_place(&mut arm.pattern, env)
                && build_expr_in_place(&mut arm.handler.value, env)
        })),
        ExprNode::AllExpr(node) => rebuild!(node, |out| build_exprs_in_place(&mut out.exprs, env)),
        ExprNode::ParallelExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.domain, env)
            && out.opts.iter_mut().all(|opt| build_expr_in_place(&mut opt.value, env))
            && build_block_in_place(&mut out.body, env)),
        ExprNode::SpawnExpr(node) => rebuild!(node, |out| out
            .opts
            .iter_mut()
            .all(|opt| build_expr_in_place(&mut opt.value, env))
            && build_block_in_place(&mut out.body, env)),
        ExprNode::WaitExpr(node) => rebuild!(node, |out| build_expr_in_place(&mut out.handle, env)),
        ExprNode::DispatchExpr(node) => rebuild!(node, |out| build_pattern_in_place(&mut out.pattern, env)
            && build_expr_in_place(&mut out.range, env)
            && out.key_clause.as_mut().is_none_or(|clause| build_key_path_in_place(&mut clause.key_path, env))
            && out.opts.iter_mut().all(|opt| {
                build_expr_in_place(&mut opt.chunk_expr, env) && build_expr_in_place(&mut opt.workgroup_expr, env)
            })
            && build_block_in_place(&mut out.body, env)),
        _ => Some(expr_ptr.clone()),
    }
}

/// Why a quote produced no syntax.
enum QuoteFailure {
    /// The tokens do not parse as the requested kind: the quote itself is invalid.
    Parse,
    /// A splice failed; its diagnostic has been reported.
    Build,
}

fn append_parser_diags(env: &CtEnv, parser: &Parser) {
    if let Some(diags) = &env.diags {
        append_diags(&mut diags.borrow_mut(), &parser.diags);
    }
}

fn parse_quoted_ast_as_kind_scoped(quote: &QuoteExpr, kind: QuoteKind, env: &mut CtEnv) -> Result<CtAst, QuoteFailure> {
    let build = QuoteFailure::Build;
    if let Some(whole_splice) = parse_whole_body_splice_expr(quote) {
        let splice_span = quote.tokens.first().map(|token| token.span.clone()).unwrap_or_default();
        let value = eval_splice_value(&whole_splice, env, &splice_span).ok_or(QuoteFailure::Build)?;
        let rendered = match kind {
            QuoteKind::Expr => render_expr_splice(&value, &splice_span)
                .map(|expr| ast_of(CtAstKind::Expr, CtAstPayload::Expr(Some(expr)))),
            QuoteKind::Stmt => {
                render_stmt_splice(&value).map(|stmt| ast_of(CtAstKind::Stmt, CtAstPayload::Stmt(Box::new(stmt))))
            }
            QuoteKind::Unspecified => return Err(build),
            QuoteKind::Item => {
                render_item_splice(&value).map(|item| ast_of(CtAstKind::Item, CtAstPayload::Item(Box::new(item))))
            }
            QuoteKind::Type => {
                render_type_splice(&value).map(|ty| ast_of(CtAstKind::Type, CtAstPayload::Type(Some(ty))))
            }
            QuoteKind::Pattern => render_pattern_splice(&value)
                .map(|pattern| ast_of(CtAstKind::Pattern, CtAstPayload::Pattern(Some(pattern)))),
        };
        if rendered.is_none() {
            emit_splice_context_diag(env, &splice_span);
        }
        return rendered.ok_or(build);
    }
    let parser = make_token_parser(&quote.tokens, true);
    let start_index = parser.index;
    match kind {
        QuoteKind::Expr => {
            let (after, parsed) = parse_expr(parser);
            append_parser_diags(env, &after);
            if after.index == start_index || parsed.is_none() || !at_eof_after_trailing_newlines(after) {
                return Err(QuoteFailure::Parse);
            }
            let payload = build_expr(&parsed, env).ok_or(build)?;
            Ok(ast_of(CtAstKind::Expr, CtAstPayload::Expr(payload)))
        }
        QuoteKind::Stmt => {
            let (after, parsed) = parse_stmt(parser);
            append_parser_diags(env, &after);
            if after.index == start_index || !is_quoted_statement_form(&parsed) || !at_eof_after_trailing_newlines(after)
            {
                return Err(QuoteFailure::Parse);
            }
            let payload = build_stmt(&parsed, env).ok_or(build)?;
            Ok(ast_of(CtAstKind::Stmt, CtAstPayload::Stmt(Box::new(payload))))
        }
        QuoteKind::Unspecified => Err(QuoteFailure::Parse),
        QuoteKind::Item => {
            let (after, parsed) = parse_item(parser);
            append_parser_diags(env, &after);
            if after.index == start_index
                || matches!(parsed, ASTItem::ErrorItem(_))
                || !at_eof_after_trailing_newlines(after)
            {
                return Err(QuoteFailure::Parse);
            }
            let payload = build_item(&parsed, env).ok_or(build)?;
            Ok(ast_of(CtAstKind::Item, CtAstPayload::Item(Box::new(payload))))
        }
        QuoteKind::Type => {
            let (after, parsed) = parse_type(parser);
            append_parser_diags(env, &after);
            if after.index == start_index || parsed.is_none() || !at_eof_after_trailing_newlines(after) {
                return Err(QuoteFailure::Parse);
            }
            let payload = build_type(&parsed, env).ok_or(build)?;
            Ok(ast_of(CtAstKind::Type, CtAstPayload::Type(payload)))
        }
        QuoteKind::Pattern => {
            let (after, parsed) = parse_pattern(parser);
            append_parser_diags(env, &after);
            if after.index == start_index || parsed.is_none() || !at_eof_after_trailing_newlines(after) {
                return Err(QuoteFailure::Parse);
            }
            let payload = build_pattern(&parsed, env).ok_or(build)?;
            Ok(ast_of(CtAstKind::Pattern, CtAstPayload::Pattern(payload)))
        }
    }
}

/// Parses and builds a quote as one kind, with the quote context and the splice trace
/// scoped to this quote.
fn parse_quoted_ast_as_kind(quote: &QuoteExpr, kind: QuoteKind, env: &mut CtEnv) -> Result<CtAst, QuoteFailure> {
    let saved_ctx = env.quote_ctx.replace(CtQuoteCtx { kind, quote_site: env.site.clone() });
    let saved_trace = std::mem::take(&mut env.quote_splice_trace);
    let result = parse_quoted_ast_as_kind_scoped(quote, kind, env);
    let result = match result {
        Ok(ast) => record_quote_build_result(Some(ast), quote, env).ok_or(QuoteFailure::Build),
        failure => failure,
    };
    env.quote_ctx = saved_ctx;
    env.quote_splice_trace = saved_trace;
    result
}

/// Turns a quote into syntax. The kind is the one written on the quote, else the one the
/// context expects, else the single kind among expression, statement and item that the
/// tokens parse as.
pub fn parse_quoted_ast(quote: &QuoteExpr, env: &mut CtEnv, expected_kind: Option<QuoteKind>) -> Option<CtAst> {
    let emit_invalid_quote = |env: &CtEnv| {
        let span = quote.tokens.first().map(|token| token.span.clone());
        match &span {
            Some(span) => spec_rule_at!("requirement.22.QuotedContentValidity", span),
            None => uv_core::spec_rule!("requirement.22.QuotedContentValidity"),
        }
        if let (Some(diags), Some(diag)) =
            (&env.diags, uv_core::diagnostic_messages::make_diagnostic_by_id("E-CTE-0220", span))
        {
            uv_core::diagnostics::emit(&mut diags.borrow_mut(), diag);
        }
    };
    let kind = if quote.kind != QuoteKind::Unspecified {
        quote.kind
    } else if let Some(expected) = expected_kind {
        expected
    } else {
        let matches: Vec<QuoteKind> = [QuoteKind::Expr, QuoteKind::Stmt, QuoteKind::Item]
            .into_iter()
            .filter(|kind| quote_parses_as_kind(quote, *kind))
            .collect();
        let [only] = matches.as_slice() else {
            emit_invalid_quote(env);
            return None;
        };
        *only
    };
    match parse_quoted_ast_as_kind(quote, kind, env) {
        Ok(ast) => Some(ast),
        Err(QuoteFailure::Parse) => {
            emit_invalid_quote(env);
            None
        }
        Err(QuoteFailure::Build) => None,
    }
}
