//! The compile-time evaluator: a small interpreter over the syntax tree.
//!
//! A failed evaluation is an `EvalResult` with `ok == false`; the caller decides which
//! diagnostic that becomes.

use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, Diagnostic, Severity};
use uv_core::span::Span;
use uv_core::{spec_rule, spec_rule_at};
use uv_source::ast::*;
use uv_source::lexer::{Token, TokenKind};

use crate::files::eval_project_files_method;
use crate::hygiene::prepare_ast_for_insertion;
use crate::quote::parse_quoted_ast;
use crate::reflect::eval_introspect_method;
use crate::util::stoull;
use crate::value::*;

fn failed() -> EvalResult {
    EvalResult::default()
}

fn succeeded(value: CtValue) -> EvalResult {
    EvalResult { ok: true, value, returned: false }
}

/// The decimal digits a literal starts with, as a number.
fn parse_u64_literal(lexeme: &str) -> Option<u64> {
    let digits = lexeme.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    lexeme[..digits].parse().ok()
}

/// What follows the digits and underscores of a literal; `u64` when nothing does.
fn literal_suffix(lexeme: &str) -> &str {
    let body = lexeme.bytes().take_while(|b| b.is_ascii_digit() || *b == b'_').count();
    if body >= lexeme.len() {
        "u64"
    } else {
        &lexeme[body..]
    }
}

fn strip_quotes(lexeme: &str) -> &str {
    lexeme
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .filter(|_| lexeme.len() >= 2)
        .unwrap_or(lexeme)
}

/// `Ast::Expr`, `Ast::Stmt`, ... behind any permission or refinement.
pub(crate) fn expected_quote_kind_of_type(ty: &TypePtr) -> Option<QuoteKind> {
    let ty = ty.as_ref()?;
    match &ty.node {
        TypeNode::TypePermType(node) => return expected_quote_kind_of_type(&node.base),
        TypeNode::TypeRefine(node) => return expected_quote_kind_of_type(&node.base),
        _ => {}
    }
    let TypeNode::TypePathType(path) = &ty.node else {
        return None;
    };
    if !path.generic_args.is_empty() || path.path.len() != 2 || path.path[0] != "Ast" {
        return None;
    }
    match path.path[1].as_str() {
        "Expr" => Some(QuoteKind::Expr),
        "Stmt" => Some(QuoteKind::Stmt),
        "Item" => Some(QuoteKind::Item),
        "Type" => Some(QuoteKind::Type),
        "Pattern" => Some(QuoteKind::Pattern),
        _ => None,
    }
}

fn binding_annotation_type_opt(binding: &Binding) -> TypePtr {
    if binding.type_opt.is_some() {
        return binding.type_opt.clone();
    }
    match binding.pat.as_deref() {
        Some(Pattern { node: PatternNode::TypedPattern(typed), .. }) => typed.r#type.clone(),
        _ => None,
    }
}

fn eval_quote_expr(quote: &QuoteExpr, env: &mut CtEnv, span: &Span, expected_kind: Option<QuoteKind>) -> EvalResult {
    spec_rule_at!("CtEval-Quote", span);
    let Some(mut parsed) = parse_quoted_ast(quote, env, expected_kind) else {
        return failed();
    };
    if parsed.span.is_none() {
        parsed.span = Some(span.clone());
    }
    if parsed.hygiene.is_none() {
        parsed.hygiene =
            Some(Box::new(CtHygiene { quote_site: env.site.clone(), emit_site: env.site.clone(), mark: 0 }));
    }
    succeeded(CtValue::Ast(parsed))
}

fn eval_expr_with_expected_quote_kind(expr: &ExprPtr, env: &mut CtEnv, expected_kind: Option<QuoteKind>) -> EvalResult {
    let (Some(_), Some(node)) = (expected_kind, expr) else {
        return eval_expr(expr, env);
    };
    match &node.node {
        ExprNode::QuoteExpr(quote) => eval_quote_expr(quote, env, &node.span, expected_kind),
        ExprNode::AttributedExpr(attributed) => {
            eval_expr_with_expected_quote_kind(&attributed.expr, env, expected_kind)
        }
        _ => eval_expr(expr, env),
    }
}

fn ct_value_equals_literal(value: &CtValue, literal: &Token) -> bool {
    match literal.kind {
        TokenKind::BoolLiteral => try_get_ct_bool(value).is_some_and(|b| (literal.lexeme == "true") == b),
        TokenKind::IntLiteral => match (try_get_ct_int(value), stoull(&literal.lexeme)) {
            (Some(int), Some(literal_value)) => int.value == literal_value,
            _ => false,
        },
        TokenKind::StringLiteral => {
            matches!(value, CtValue::String(text) if text == strip_quotes(&literal.lexeme))
        }
        _ => false,
    }
}

/// Enum values compare equal only when neither carries a payload.
fn ct_enum_equals(lhs: &CtEnum, rhs: &CtEnum) -> bool {
    lhs.path == rhs.path
        && lhs.variant == rhs.variant
        && lhs.payload == CtPayload::None
        && rhs.payload == CtPayload::None
}

fn find_ct_field<'a>(fields: &'a CtFields, name: &str) -> Option<&'a CtValue> {
    fields.iter().find(|field| field.0 == name).map(|field| &field.1)
}

/// Runs a binding attempt that only takes effect when it succeeds as a whole.
fn bind_atomically(env: &mut CtEnv, bind: impl FnOnce(&mut CtEnv) -> bool) -> bool {
    let mut next = env.clone();
    if !bind(&mut next) {
        return false;
    }
    *env = next;
    true
}

fn bind_eval_patterns(env: &mut CtEnv, patterns: &[PatternPtr], values: &[CtValue]) -> bool {
    if patterns.len() != values.len() {
        return false;
    }
    bind_atomically(env, |next| {
        patterns.iter().zip(values).all(|(pattern, value)| bind_eval_pattern_value(next, pattern, value))
    })
}

fn bind_eval_field_patterns(env: &mut CtEnv, patterns: &[FieldPattern], fields: &CtFields) -> bool {
    bind_atomically(env, |next| {
        patterns.iter().all(|field_pattern| {
            let Some(field_value) = find_ct_field(fields, &field_pattern.name) else {
                return false;
            };
            if field_pattern.pattern_opt.is_some() {
                return bind_eval_pattern_value(next, &field_pattern.pattern_opt, field_value);
            }
            next.values.insert(field_pattern.name.clone(), field_value.clone());
            true
        })
    })
}

fn bind_eval_range_pattern(range: &RangePattern, value: &CtValue) -> bool {
    let (Some(int_value), Some(lo), Some(hi)) = (try_get_ct_int(value), &range.lo, &range.hi) else {
        return false;
    };
    let (PatternNode::LiteralPattern(lo), PatternNode::LiteralPattern(hi)) = (&lo.node, &hi.node) else {
        return false;
    };
    let (Some(lo), Some(hi)) = (stoull(&lo.literal.lexeme), stoull(&hi.literal.lexeme)) else {
        return false;
    };
    let upper_ok =
        if range.kind == RangeKind::Inclusive { int_value.value <= hi } else { int_value.value < hi };
    int_value.value >= lo && upper_ok
}

fn bind_eval_pattern_value(env: &mut CtEnv, pattern: &PatternPtr, value: &CtValue) -> bool {
    let Some(pattern) = pattern else {
        return false;
    };
    match &pattern.node {
        PatternNode::WildcardPattern(_) => true,
        PatternNode::IdentifierPattern(node) => {
            env.values.insert(node.name.clone(), value.clone());
            true
        }
        PatternNode::TypedPattern(node) => {
            env.values.insert(node.name.clone(), value.clone());
            true
        }
        PatternNode::LiteralPattern(node) => ct_value_equals_literal(value, &node.literal),
        PatternNode::TuplePattern(tuple) => {
            matches!(value, CtValue::Tuple(elements) if bind_eval_patterns(env, &tuple.elements, elements))
        }
        PatternNode::RecordPattern(node) => {
            matches!(value, CtValue::Record(record)
                if record.path == node.path && bind_eval_field_patterns(env, &node.fields, &record.fields))
        }
        PatternNode::EnumPattern(node) => {
            let CtValue::Enum(enum_value) = value else {
                return false;
            };
            if enum_value.path != node.path || enum_value.variant != node.name {
                return false;
            }
            match (&node.payload_opt, &enum_value.payload) {
                (None, payload) => *payload == CtPayload::None,
                (Some(EnumPayloadPattern::TuplePayloadPattern(pattern)), CtPayload::Tuple(elements)) => {
                    bind_eval_patterns(env, &pattern.elements, elements)
                }
                (Some(EnumPayloadPattern::RecordPayloadPattern(pattern)), CtPayload::Record(fields)) => {
                    bind_eval_field_patterns(env, &pattern.fields, fields)
                }
                _ => false,
            }
        }
        PatternNode::ModalPattern(node) => {
            let CtValue::ModalState(modal) = value else {
                return false;
            };
            if modal.target.state != node.state {
                return false;
            }
            match &node.fields_opt {
                None => true,
                Some(payload) => bind_eval_field_patterns(env, &payload.fields, &modal.fields),
            }
        }
        PatternNode::RangePattern(range) => bind_eval_range_pattern(range, value),
        _ => false,
    }
}

fn append_ct_diagnostic(env: &CtEnv, diag: Diagnostic) {
    if let Some(diags) = &env.diags {
        emit(&mut diags.borrow_mut(), diag);
    }
}

fn append_ct_coded_diagnostic(env: &CtEnv, diag_id: &str, fallback_severity: Severity, message: &str) {
    let diag = match make_diagnostic_by_id(diag_id, Some(env.current_span.clone())) {
        Some(mut diag) => {
            diag.message = message.to_string();
            diag
        }
        None => Diagnostic {
            severity: fallback_severity,
            message: format!("Internal error: unresolved diagnostic id '{diag_id}': {message}"),
            span: Some(env.current_span.clone()),
            ..Diagnostic::default()
        },
    };
    append_ct_diagnostic(env, diag);
}

fn append_ct_user_note_diagnostic(env: &CtEnv, message: &str) {
    append_ct_diagnostic(
        env,
        Diagnostic {
            severity: Severity::Note,
            message: message.to_string(),
            span: Some(env.current_span.clone()),
            ..Diagnostic::default()
        },
    );
}

/// Evaluates a contract predicate of a compile-time procedure. `Err` is the result the
/// call fails with.
fn check_contract(predicate: &ExprPtr, pred_env: &mut CtEnv, env: &CtEnv, callee_span: &Span) -> Result<(), EvalResult> {
    let pred = eval_expr(predicate, pred_env);
    if !pred.ok {
        return Err(pred);
    }
    let Some(holds) = try_get_ct_bool(&pred.value) else {
        return Err(failed());
    };
    spec_rule_at!("requirement.22.CompileTimeProcedureContracts", callee_span);
    if !holds {
        emit_comptime_diag(env, "E-CTE-0033", callee_span);
        return Err(failed());
    }
    Ok(())
}

fn eval_call(call: &CallExpr, env: &mut CtEnv) -> EvalResult {
    let Some(callee) = &call.callee else {
        return failed();
    };
    let ExprNode::IdentifierExpr(ident) = &callee.node else {
        return failed();
    };
    let Some(proc) = env.procs.get(&ident.name).cloned() else {
        return failed();
    };
    if call.args.len() != proc.params.len() {
        return failed();
    }
    let mut proc_env = env.clone();
    proc_env.contract_result_value = None;
    proc_env.return_quote_kind = expected_quote_kind_of_type(&proc.return_type_opt);
    for (arg, param) in call.args.iter().zip(&proc.params) {
        let value = eval_expr(&arg.value, env);
        if !value.ok {
            return value;
        }
        proc_env.values.insert(param.name.clone(), value.value);
    }
    proc_env.contract_entry_values = Some(Rc::new(proc_env.values.clone()));
    let contract = proc.contract.as_ref();
    if let Some(precondition) = contract.map(|c| &c.precondition).filter(|p| p.is_some()) {
        if let Err(result) = check_contract(precondition, &mut proc_env, env, &callee.span) {
            return result;
        }
    }
    let body = proc.body.as_deref().expect("compile-time procedure without a body");
    let mut result = eval_block(body, &mut proc_env);
    if !result.ok {
        return result;
    }
    result.returned = false;
    if let Some(postcondition) = contract.map(|c| &c.postcondition).filter(|p| p.is_some()) {
        let mut post_env = proc_env.clone();
        post_env.contract_result_value = Some(result.value.clone());
        if let Err(failure) = check_contract(postcondition, &mut post_env, env, &callee.span) {
            return failure;
        }
    }
    result
}

fn eval_diagnostics_method(call: &MethodCallExpr, env: &mut CtEnv, call_span: &Span) -> Option<EvalResult> {
    spec_rule_at!("requirement.22.CtCapMethodCallParsing", call_span);
    spec_rule_at!("def.22.CtCapabilityDynamicHelpers", call_span);
    if call.name == "current_module" && call.args.is_empty() {
        spec_rule_at!("rule.22.CtBuiltin-Diagnostics-CurrentModule", &env.current_span);
        return Some(succeeded(CtValue::String(env.site.module_path.join("::"))));
    }
    if call.name == "current_span" && call.args.is_empty() {
        spec_rule_at!("rule.22.CtBuiltin-Diagnostics-CurrentSpan", &env.current_span);
        return Some(succeeded(make_span_value(&env.site.span)));
    }
    if call.args.len() != 1 {
        return None;
    }
    let value = eval_expr(&call.args[0].value, env);
    if !value.ok {
        return Some(value);
    }
    let CtValue::String(message) = &value.value else {
        return Some(failed());
    };
    match call.name.as_str() {
        "error" => {
            spec_rule_at!("rule.22.CtBuiltin-Diagnostics-Error", &env.current_span);
            spec_rule_at!("requirement.22.UserDiagnosticBuiltinEmission", &env.current_span);
            append_ct_coded_diagnostic(env, "E-CTE-0070", Severity::Error, message);
        }
        "warning" => {
            spec_rule_at!("rule.22.CtBuiltin-Diagnostics-Warning", &env.current_span);
            spec_rule_at!("requirement.22.UserDiagnosticBuiltinEmission", &env.current_span);
            append_ct_coded_diagnostic(env, "W-CTE-0071", Severity::Warning, message);
        }
        "note" => {
            spec_rule_at!("rule.22.CtBuiltin-Diagnostics-Note", &env.current_span);
            spec_rule_at!("requirement.22.UserDiagnosticBuiltinEmission", &env.current_span);
            append_ct_user_note_diagnostic(env, message);
        }
        _ => return None,
    }
    Some(succeeded(make_ct_unit()))
}

fn eval_method_call(call: &MethodCallExpr, env: &mut CtEnv) -> EvalResult {
    let Some(receiver) = &call.receiver else {
        return failed();
    };
    let call_span = receiver.span.clone();
    if let Some(result) = eval_project_files_method(call, env) {
        return result;
    }
    if let Some(result) = eval_introspect_method(call, env) {
        return result;
    }
    let receiver_name = match &receiver.node {
        ExprNode::IdentifierExpr(ident) => Some(ident.name.as_str()),
        _ => None,
    };
    if receiver_name == Some("diagnostics") {
        if let Some(result) = eval_diagnostics_method(call, env, &call_span) {
            return result;
        }
    }
    if call.name != "emit" || call.args.len() != 1 {
        return failed();
    }
    if receiver_name == Some("emitter") {
        spec_rule_at!("requirement.22.CtCapMethodCallParsing", &call_span);
        spec_rule_at!("def.22.CtCapabilityDynamicHelpers", &call_span);
        spec_rule_at!("requirement.22.TypeEmitterAvailability", &call_span);
    }
    let recv_value = eval_expr(&call.receiver, env);
    if !recv_value.ok {
        return recv_value;
    }
    let value = eval_expr(&call.args[0].value, env);
    if !value.ok {
        return value;
    }
    let CtValue::Ast(ast_value) = &value.value else {
        return failed();
    };
    if ast_value.kind != CtAstKind::Item {
        let emit_span = call.args[0].value.as_ref().map_or(&call_span, |arg| &arg.span).clone();
        spec_rule_at!("requirement.22.TypeEmitterEmitTypeRequirement", &emit_span);
        spec_rule_at!("requirement.22.EmitterEmitWellFormedness", &emit_span);
        emit_comptime_diag(env, "E-CTE-0251", &emit_span);
        return failed();
    }
    let site = env.site.clone();
    let Some(CtAst { payload: CtAstPayload::Item(item), .. }) = prepare_ast_for_insertion(ast_value, &site, env)
    else {
        return failed();
    };
    if let Some(pending) = &env.pending_emits {
        pending.borrow_mut().push(*item);
    }
    spec_rule!("CtBuiltin-Emit");
    spec_rule_at!("requirement.22.TypeEmitterAvailability", &call_span);
    succeeded(make_ct_unit())
}

fn stmt_span(stmt: &Stmt) -> &Span {
    uv_source::parser::stmt_span(stmt)
}

/// Evaluates a block in a copy of the environment: bindings made inside do not escape.
pub fn eval_block(block: &Block, env: &mut CtEnv) -> EvalResult {
    let mut local = env.clone();
    for stmt in &block.stmts {
        local.current_span = stmt_span(stmt).clone();
        match stmt {
            Stmt::LetStmt(LetStmt { binding, .. }) | Stmt::VarStmt(VarStmt { binding, .. }) => {
                if binding.init.is_none() {
                    continue;
                }
                let expected = expected_quote_kind_of_type(&binding_annotation_type_opt(binding));
                let value = eval_expr_with_expected_quote_kind(&binding.init, &mut local, expected);
                if !value.ok {
                    return value;
                }
                if !bind_eval_pattern_value(&mut local, &binding.pat, &value.value) {
                    return failed();
                }
            }
            Stmt::ExprStmt(expr_stmt) => {
                let value = eval_expr(&expr_stmt.value, &mut local);
                if !value.ok || value.returned {
                    return value;
                }
            }
            Stmt::ReturnStmt(ret_stmt) => {
                let expected = local.return_quote_kind;
                let mut value = eval_expr_with_expected_quote_kind(&ret_stmt.value_opt, &mut local, expected);
                value.returned = true;
                return value;
            }
            _ => {}
        }
    }
    if block.tail_opt.is_some() {
        return eval_expr(&block.tail_opt, &mut local);
    }
    succeeded(make_ct_unit())
}

fn eval_all(exprs: &[ExprPtr], env: &mut CtEnv) -> Result<Vec<CtValue>, EvalResult> {
    let mut values = Vec::with_capacity(exprs.len());
    for expr in exprs {
        let value = eval_expr(expr, env);
        if !value.ok {
            return Err(value);
        }
        values.push(value.value);
    }
    Ok(values)
}

fn eval_field_inits(fields: &[FieldInit], env: &mut CtEnv) -> Result<CtFields, EvalResult> {
    let mut values = Vec::with_capacity(fields.len());
    for field in fields {
        let value = eval_expr(&field.value, env);
        if !value.ok {
            return Err(value);
        }
        values.push((field.name.clone(), value.value));
    }
    Ok(values)
}

fn eval_binary(node: &BinaryExpr, env: &mut CtEnv) -> EvalResult {
    let lhs = eval_expr(&node.lhs, env);
    if !lhs.ok {
        return lhs;
    }
    let rhs = eval_expr(&node.rhs, env);
    if !rhs.ok {
        return rhs;
    }
    let op = node.op.as_str();
    let boolean = |value: Option<bool>| value.map_or_else(failed, |b| succeeded(make_ct_bool(b)));
    if let (Some(lb), Some(rb)) = (try_get_ct_bool(&lhs.value), try_get_ct_bool(&rhs.value)) {
        return boolean(match op {
            "&&" => Some(lb && rb),
            "||" => Some(lb || rb),
            "==" => Some(lb == rb),
            "!=" => Some(lb != rb),
            _ => None,
        });
    }
    if let (CtValue::String(ls), CtValue::String(rs)) = (&lhs.value, &rhs.value) {
        return boolean(match op {
            "==" => Some(ls == rs),
            "!=" => Some(ls != rs),
            _ => None,
        });
    }
    if let (CtValue::Enum(le), CtValue::Enum(re)) = (&lhs.value, &rhs.value) {
        return boolean(match op {
            "==" => Some(ct_enum_equals(le, re)),
            "!=" => Some(!ct_enum_equals(le, re)),
            _ => None,
        });
    }
    let (Some(li), Some(ri)) = (try_get_ct_int(&lhs.value), try_get_ct_int(&rhs.value)) else {
        return failed();
    };
    match op {
        "+" => succeeded(make_ct_int(li.value.wrapping_add(ri.value), &li.suffix, "")),
        "*" => succeeded(make_ct_int(li.value.wrapping_mul(ri.value), &li.suffix, "")),
        "==" => boolean(Some(li.value == ri.value)),
        "!=" => boolean(Some(li.value != ri.value)),
        ">" => boolean(Some(li.value > ri.value)),
        ">=" => boolean(Some(li.value >= ri.value)),
        "<" => boolean(Some(li.value < ri.value)),
        "<=" => boolean(Some(li.value <= ri.value)),
        _ => failed(),
    }
}

fn eval_node(expr: &Expr, env: &mut CtEnv) -> EvalResult {
    macro_rules! try_eval {
        ($result:expr) => {
            match $result {
                Ok(value) => value,
                Err(failure) => return failure,
            }
        };
    }
    match &expr.node {
        ExprNode::LiteralExpr(node) => {
            let lexeme = node.literal.lexeme.as_str();
            match node.literal.kind {
                TokenKind::BoolLiteral => succeeded(make_ct_bool(lexeme == "true")),
                TokenKind::StringLiteral => succeeded(CtValue::String(strip_quotes(lexeme).to_string())),
                TokenKind::CharLiteral => succeeded(make_ct_char(lexeme)),
                TokenKind::IntLiteral => match parse_u64_literal(lexeme) {
                    Some(value) => succeeded(make_ct_int(value, literal_suffix(lexeme), lexeme)),
                    None => failed(),
                },
                TokenKind::FloatLiteral => succeeded(make_ct_float(lexeme)),
                _ => failed(),
            }
        }
        ExprNode::IdentifierExpr(node) => env.values.get(&node.name).cloned().map_or_else(failed, succeeded),
        ExprNode::QualifiedNameExpr(node) => succeeded(CtValue::Enum(Rc::new(CtEnum {
            path: node.path.clone(),
            variant: node.name.clone(),
            payload: CtPayload::None,
        }))),
        ExprNode::QualifiedApplyExpr(node) => {
            // A failing argument fails the application without propagating its result.
            let payload = match &node.args {
                ApplyArgs::ParenArgs(paren) => {
                    let exprs: Vec<ExprPtr> = paren.args.iter().map(|arg| arg.value.clone()).collect();
                    eval_all(&exprs, env).map(CtPayload::Tuple)
                }
                ApplyArgs::BraceArgs(brace) => eval_field_inits(&brace.fields, env).map(CtPayload::Record),
            };
            let Ok(payload) = payload else {
                return failed();
            };
            succeeded(CtValue::Enum(Rc::new(CtEnum {
                path: node.path.clone(),
                variant: node.name.clone(),
                payload,
            })))
        }
        ExprNode::ResultExpr(_) => env.contract_result_value.clone().map_or_else(failed, succeeded),
        ExprNode::EntryExpr(node) => {
            let Some(entry_values) = env.contract_entry_values.clone() else {
                return failed();
            };
            let mut entry_env = env.clone();
            entry_env.values = (*entry_values).clone();
            entry_env.contract_result_value = None;
            eval_expr(&node.expr, &mut entry_env)
        }
        ExprNode::BinaryExpr(node) => eval_binary(node, env),
        ExprNode::IfExpr(node) => {
            let cond = eval_expr(&node.cond, env);
            if !cond.ok {
                return cond;
            }
            match try_get_ct_bool(&cond.value) {
                None => failed(),
                Some(true) => eval_expr(&node.then_expr, env),
                Some(false) if node.else_expr.is_some() => eval_expr(&node.else_expr, env),
                Some(false) => succeeded(make_ct_unit()),
            }
        }
        ExprNode::IfIsExpr(node) => {
            let scrutinee = eval_expr(&node.scrutinee, env);
            if !scrutinee.ok {
                return scrutinee;
            }
            let mut then_env = env.clone();
            if bind_eval_pattern_value(&mut then_env, &node.pattern, &scrutinee.value) {
                return eval_expr(&node.then_expr, &mut then_env);
            }
            if node.else_expr.is_some() {
                return eval_expr(&node.else_expr, env);
            }
            succeeded(make_ct_unit())
        }
        ExprNode::IfCaseExpr(node) => {
            let scrutinee = eval_expr(&node.scrutinee, env);
            if !scrutinee.ok {
                return scrutinee;
            }
            for clause in &node.cases {
                let mut case_env = env.clone();
                if bind_eval_pattern_value(&mut case_env, &clause.pattern, &scrutinee.value) {
                    return eval_expr(&clause.body, &mut case_env);
                }
            }
            if node.else_expr.is_some() {
                return eval_expr(&node.else_expr, env);
            }
            succeeded(make_ct_unit())
        }
        ExprNode::BlockExpr(node) => {
            eval_block(node.block.as_deref().expect("block expression without a block"), env)
        }
        ExprNode::ComptimeExpr(node) => eval_expr(&node.body, env),
        ExprNode::TupleExpr(node) => {
            succeeded(CtValue::Tuple(Rc::new(try_eval!(eval_all(&node.elements, env)))))
        }
        ExprNode::ArrayExpr(node) => {
            let mut elements = Vec::new();
            for segment in &node.elements {
                match segment {
                    ArraySegment::ArrayElemSegment(elem) => {
                        let value = eval_expr(&elem.value, env);
                        if !value.ok {
                            return value;
                        }
                        elements.push(value.value);
                    }
                    ArraySegment::ArrayRepeatSegment(repeat) => {
                        let value = eval_expr(&repeat.value, env);
                        if !value.ok {
                            return value;
                        }
                        let count_value = eval_expr(&repeat.count, env);
                        if !count_value.ok {
                            return count_value;
                        }
                        let Some(count) = try_get_ct_int(&count_value.value) else {
                            return failed();
                        };
                        for _ in 0..count.value {
                            elements.push(value.value.clone());
                        }
                    }
                }
            }
            succeeded(CtValue::Array(Rc::new(elements)))
        }
        ExprNode::RecordExpr(node) => {
            let fields = try_eval!(eval_field_inits(&node.fields, env));
            succeeded(match &node.target {
                RecordExprTarget::Path(path) => CtValue::Record(Rc::new(CtRecord { path: path.clone(), fields })),
                RecordExprTarget::ModalStateRef(target) => {
                    CtValue::ModalState(Rc::new(CtModalState { target: target.clone(), fields }))
                }
            })
        }
        ExprNode::EnumLiteralExpr(node) => {
            let Some((variant, path)) = node.path.split_last() else {
                return failed();
            };
            let payload = match &node.payload_opt {
                None => Ok(CtPayload::None),
                Some(EnumPayload::EnumPayloadParen(paren)) => eval_all(&paren.elements, env).map(CtPayload::Tuple),
                Some(EnumPayload::EnumPayloadBrace(brace)) => {
                    eval_field_inits(&brace.fields, env).map(CtPayload::Record)
                }
            };
            let Ok(payload) = payload else {
                return failed();
            };
            succeeded(CtValue::Enum(Rc::new(CtEnum { path: path.to_vec(), variant: variant.clone(), payload })))
        }
        ExprNode::FieldAccessExpr(node) => {
            let base = eval_expr(&node.base, env);
            if !base.ok {
                return base;
            }
            let fields = match &base.value {
                CtValue::Record(record) => &record.fields,
                CtValue::ModalState(state) => &state.fields,
                _ => return failed(),
            };
            find_ct_field(fields, &node.name).cloned().map_or_else(failed, succeeded)
        }
        ExprNode::AttributedExpr(node) => eval_expr(&node.expr, env),
        ExprNode::CallExpr(node) => eval_call(node, env),
        ExprNode::MethodCallExpr(node) => eval_method_call(node, env),
        ExprNode::TypeLiteralExpr(node) => {
            spec_rule_at!("CtEval-TypeLiteral", &expr.span);
            succeeded(CtValue::Type(node.r#type.clone()))
        }
        ExprNode::QuoteExpr(node) => eval_quote_expr(node, env, &expr.span, None),
        _ => failed(),
    }
}

pub fn eval_expr(expr: &ExprPtr, env: &mut CtEnv) -> EvalResult {
    let Some(expr) = expr else {
        return failed();
    };
    // The expression's span is the current span and site while it is evaluated.
    let saved = std::mem::replace(&mut env.current_span, expr.span.clone());
    env.site.span = expr.span.clone();
    spec_rule_at!("requirement.22.CtEvalOrdinarySemantics", &expr.span);
    let result = eval_node(expr, env);
    env.site.span = saved.clone();
    env.current_span = saved;
    result
}
