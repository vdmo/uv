//! What a call must prove: the callee's precondition, and a foreign procedure's
//! `assumes` clauses, with the arguments in place of the parameters.

use std::collections::HashMap;
use std::sync::Arc;

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};
use uv_source::attributes::{resolve_verification_mode_attribute, VerificationModeAttribute};
use uv_source::lexer::token::TokenKind;

use super::call::{lookup_procedure_for_callee, ProcLike};
use crate::context::ScopeContext;
use crate::contracts::verification::{add_predicate_facts, static_proof_at, StaticProofContext};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::collect_pat_names;

type Bindings = Vec<(String, ExprPtr)>;

fn find_binding_replacement<'b>(ident: &str, bindings: &'b Bindings) -> Option<&'b ExprPtr> {
    bindings.iter().find(|(name, _)| id_eq(name, ident)).map(|(_, value)| value)
}

/// The literal a module-level `let` of this name is initialised with, in any module.
fn find_module_const_literal_replacement(ctx: &ScopeContext<'_>, ident: &str) -> ExprPtr {
    for item in ctx.sigma.mods.iter().flat_map(|module| &module.items) {
        let ast::ASTItem::StaticDecl(decl) = item else {
            continue;
        };
        if decl.r#mut != ast::Mutability::Let {
            continue;
        }
        let Some(ast::PatternNode::IdentifierPattern(ident_pat)) = decl.binding.pat.as_deref().map(|pat| &pat.node) else {
            continue;
        };
        if id_eq(&ident_pat.name, ident) && matches!(decl.binding.init.as_deref().map(|init| &init.node), Some(ExprNode::LiteralExpr(_))) {
            return decl.binding.init.clone();
        }
    }
    None
}

/// The leading decimal digits of a literal, as the reference reads them.
fn parse_simple_int_literal(text: &str) -> Option<i64> {
    let (negative, rest) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    format!("{}{digits}", if negative { "-" } else { "" }).parse().ok()
}

fn const_int_value(ctx: &ScopeContext<'_>, expr: &ExprPtr) -> Option<i64> {
    let name = match &expr.as_deref()?.node {
        ExprNode::LiteralExpr(lit) if lit.literal.kind == TokenKind::IntLiteral => return parse_simple_int_literal(&lit.literal.lexeme),
        ExprNode::IdentifierExpr(node) => &node.name,
        ExprNode::PathExpr(node) => &node.name,
        ExprNode::QualifiedNameExpr(node) => &node.name,
        _ => return None,
    };
    let replacement = find_module_const_literal_replacement(ctx, name);
    replacement.as_ref()?;
    const_int_value(ctx, &replacement)
}

fn bool_literal_value(expr: &ExprPtr) -> Option<bool> {
    match &expr.as_deref()?.node {
        ExprNode::LiteralExpr(lit) if lit.literal.kind == TokenKind::BoolLiteral => match lit.literal.lexeme.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

/// The values of the `let`s in a compile-time procedure's body so far.
#[derive(Default)]
struct ComptimeBoolProofEnv {
    ints: HashMap<String, i64>,
    bools: HashMap<String, bool>,
}

fn compare(op: &str, left: i64, right: i64) -> Option<bool> {
    Some(match op {
        "==" => left == right,
        "!=" => left != right,
        "<" => left < right,
        "<=" => left <= right,
        ">" => left > right,
        ">=" => left >= right,
        _ => return None,
    })
}

fn comptime_proof_bool_value(ctx: &ScopeContext<'_>, expr: &ExprPtr, env: &ComptimeBoolProofEnv) -> Option<bool> {
    if let literal @ Some(_) = bool_literal_value(expr) {
        return literal;
    }
    let binary = match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => return env.bools.get(&ident.name).copied(),
        ExprNode::UnaryExpr(unary) if unary.op == "!" => return comptime_proof_bool_value(ctx, &unary.value, env).map(|inner| !inner),
        ExprNode::BinaryExpr(binary) => binary,
        _ => return None,
    };
    if binary.op == "&&" || binary.op == "||" {
        let left = comptime_proof_bool_value(ctx, &binary.lhs, env)?;
        let right = comptime_proof_bool_value(ctx, &binary.rhs, env)?;
        return Some(if binary.op == "&&" { left && right } else { left || right });
    }
    if binary.op == "==" || binary.op == "!=" {
        if let (Some(left), Some(right)) = (comptime_proof_bool_value(ctx, &binary.lhs, env), comptime_proof_bool_value(ctx, &binary.rhs, env)) {
            return Some((left == right) == (binary.op == "=="));
        }
    }
    let left = comptime_proof_int_value(ctx, &binary.lhs, env)?;
    let right = comptime_proof_int_value(ctx, &binary.rhs, env)?;
    compare(&binary.op, left, right)
}

fn comptime_proof_int_value(ctx: &ScopeContext<'_>, expr: &ExprPtr, env: &ComptimeBoolProofEnv) -> Option<i64> {
    if let literal @ Some(_) = const_int_value(ctx, expr) {
        return literal;
    }
    let binary = match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => return env.ints.get(&ident.name).copied(),
        ExprNode::BinaryExpr(binary) => binary,
        _ => return None,
    };
    let left = comptime_proof_int_value(ctx, &binary.lhs, env)?;
    let right = comptime_proof_int_value(ctx, &binary.rhs, env)?;
    match binary.op.as_str() {
        "+" => Some(left.wrapping_add(right)),
        "-" => Some(left.wrapping_sub(right)),
        "*" => Some(left.wrapping_mul(right)),
        "/" if right != 0 => Some(left.wrapping_div(right)),
        _ => None,
    }
}

/// What a compile-time procedure returns, when its body is a boolean expression or
/// `let`s of constants followed by a `return` of one.
fn comptime_procedure_literal_bool_return(ctx: &ScopeContext<'_>, body: &ast::BlockPtr) -> Option<bool> {
    let body = body.as_deref()?;
    let mut env = ComptimeBoolProofEnv::default();
    if body.stmts.is_empty() && body.tail_opt.is_some() {
        return comptime_proof_bool_value(ctx, &body.tail_opt, &env);
    }
    let (last, lets) = body.stmts.split_last().filter(|_| body.tail_opt.is_none())?;
    for stmt in lets {
        let Stmt::LetStmt(let_stmt) = stmt else {
            return None;
        };
        let name = match &let_stmt.binding.pat.as_deref()?.node {
            ast::PatternNode::IdentifierPattern(ident) => &ident.name,
            ast::PatternNode::TypedPattern(typed) => &typed.name,
            _ => return None,
        };
        if let Some(int_value) = comptime_proof_int_value(ctx, &let_stmt.binding.init, &env) {
            env.ints.insert(name.clone(), int_value);
            env.bools.remove(name);
        } else if let Some(bool_value) = comptime_proof_bool_value(ctx, &let_stmt.binding.init, &env) {
            env.bools.insert(name.clone(), bool_value);
            env.ints.remove(name);
        } else {
            return None;
        }
    }
    let Stmt::ReturnStmt(ret) = last else {
        return None;
    };
    comptime_proof_bool_value(ctx, &ret.value_opt, &env)
}

fn prove_comptime_bool_predicate(ctx: &ScopeContext<'_>, expr: &ExprPtr) -> Option<bool> {
    if let literal @ Some(_) = bool_literal_value(expr) {
        return literal;
    }
    match &expr.as_deref()?.node {
        ExprNode::UnaryExpr(unary) if unary.op == "!" => prove_comptime_bool_predicate(ctx, &unary.value).map(|inner| !inner),
        ExprNode::BinaryExpr(binary) if binary.op == "&&" || binary.op == "||" => {
            let left = prove_comptime_bool_predicate(ctx, &binary.lhs)?;
            let right = prove_comptime_bool_predicate(ctx, &binary.rhs)?;
            Some(if binary.op == "&&" { left && right } else { left || right })
        }
        // A call of a compile-time procedure without arguments.
        ExprNode::CallExpr(call) if call.args.is_empty() => {
            let lookup = lookup_procedure_for_callee(ctx, &call.callee)?;
            let ProcLike { proc: None, comptime_proc: Some(proc) } = lookup.proc else {
                return None;
            };
            comptime_procedure_literal_bool_return(ctx, &proc.body)
        }
        _ => None,
    }
}

/// A predicate decided by constants alone.
fn prove_simple_predicate(ctx: &ScopeContext<'_>, expr: &ExprPtr) -> Option<bool> {
    if let decided @ Some(_) = prove_comptime_bool_predicate(ctx, expr) {
        return decided;
    }
    let ExprNode::BinaryExpr(binary) = &expr.as_deref()?.node else {
        return None;
    };
    if binary.op == "&&" {
        return Some(prove_simple_predicate(ctx, &binary.lhs)? && prove_simple_predicate(ctx, &binary.rhs)?);
    }
    compare(&binary.op, const_int_value(ctx, &binary.lhs)?, const_int_value(ctx, &binary.rhs)?)
}

fn any_pattern_binds_name(pattern: &ast::PatternPtr, bindings: &Bindings) -> bool {
    let Some(pattern) = pattern.as_deref() else {
        return false;
    };
    let mut names = Vec::new();
    collect_pat_names(pattern, &mut names);
    bindings.iter().any(|(name, _)| names.contains(&id_key_of(name)))
}

/// The predicate with the arguments for the parameters, and module-level literal
/// constants for their names.
fn substitute_foreign_predicate(ctx: &ScopeContext<'_>, expr: &ExprPtr, bindings: &Bindings) -> ExprPtr {
    let Some(e) = expr.as_deref() else {
        return expr.clone();
    };
    let sub = |inner: &ExprPtr| substitute_foreign_predicate(ctx, inner, bindings);
    let sub_args = |args: &[ast::Arg]| -> Vec<ast::Arg> { args.iter().map(|arg| ast::Arg { value: sub(&arg.value), ..arg.clone() }).collect() };
    let by_name = |name: &str| -> ExprPtr {
        if let Some(replacement) = find_binding_replacement(name, bindings) {
            return replacement.clone();
        }
        match find_module_const_literal_replacement(ctx, name) {
            replacement @ Some(_) => replacement,
            None => expr.clone(),
        }
    };
    let node = match &e.node {
        ExprNode::IdentifierExpr(node) => return by_name(&node.name),
        ExprNode::PathExpr(node) => return by_name(&node.name),
        ExprNode::QualifiedNameExpr(node) => return by_name(&node.name),
        ExprNode::BinaryExpr(node) => ExprNode::BinaryExpr(ast::BinaryExpr { lhs: sub(&node.lhs), rhs: sub(&node.rhs), ..node.clone() }),
        ExprNode::UnaryExpr(node) => ExprNode::UnaryExpr(ast::UnaryExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::FieldAccessExpr(node) => ExprNode::FieldAccessExpr(ast::FieldAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::TupleAccessExpr(node) => ExprNode::TupleAccessExpr(ast::TupleAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::IndexAccessExpr(node) => ExprNode::IndexAccessExpr(ast::IndexAccessExpr { base: sub(&node.base), index: sub(&node.index) }),
        ExprNode::CallExpr(node) => ExprNode::CallExpr(ast::CallExpr { callee: sub(&node.callee), args: sub_args(&node.args), ..node.clone() }),
        ExprNode::QualifiedApplyExpr(node) => {
            let args = match &node.args {
                ast::ApplyArgs::ParenArgs(paren) => ast::ApplyArgs::ParenArgs(ast::ParenArgs { args: sub_args(&paren.args) }),
                ast::ApplyArgs::BraceArgs(brace) => {
                    let mut brace = brace.clone();
                    for field in &mut brace.fields {
                        field.value = sub(&field.value);
                    }
                    ast::ApplyArgs::BraceArgs(brace)
                }
            };
            ExprNode::QualifiedApplyExpr(ast::QualifiedApplyExpr { args, ..node.clone() })
        }
        ExprNode::MethodCallExpr(node) => {
            ExprNode::MethodCallExpr(ast::MethodCallExpr { receiver: sub(&node.receiver), args: sub_args(&node.args), ..node.clone() })
        }
        ExprNode::CastExpr(node) => ExprNode::CastExpr(ast::CastExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::TupleExpr(node) => ExprNode::TupleExpr(ast::TupleExpr { elements: node.elements.iter().map(&sub).collect() }),
        ExprNode::ArrayExpr(node) => {
            let mut out = node.clone();
            for segment in &mut out.elements {
                match segment {
                    ast::ArraySegment::ArrayElemSegment(elem) => elem.value = sub(&elem.value),
                    ast::ArraySegment::ArrayRepeatSegment(repeat) => {
                        repeat.value = sub(&repeat.value);
                        repeat.count = sub(&repeat.count);
                    }
                }
            }
            ExprNode::ArrayExpr(out)
        }
        ExprNode::ArrayRepeatExpr(node) => ExprNode::ArrayRepeatExpr(ast::ArrayRepeatExpr { value: sub(&node.value), count: sub(&node.count) }),
        ExprNode::RecordExpr(node) => {
            let mut out = node.clone();
            for field in &mut out.fields {
                field.value = sub(&field.value);
            }
            ExprNode::RecordExpr(out)
        }
        ExprNode::IfExpr(node) => {
            ExprNode::IfExpr(ast::IfExpr { cond: sub(&node.cond), then_expr: sub(&node.then_expr), else_expr: sub(&node.else_expr) })
        }
        ExprNode::IfIsExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee);
            if !any_pattern_binds_name(&node.pattern, bindings) {
                out.then_expr = sub(&node.then_expr);
            }
            out.else_expr = sub(&node.else_expr);
            ExprNode::IfIsExpr(out)
        }
        ExprNode::IfCaseExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee);
            for arm in &mut out.cases {
                if !any_pattern_binds_name(&arm.pattern, bindings) {
                    arm.body = sub(&arm.body);
                }
            }
            out.else_expr = sub(&node.else_expr);
            ExprNode::IfCaseExpr(out)
        }
        ExprNode::BlockExpr(node) => {
            let mut out = node.clone();
            if let Some(block) = out.block.as_deref() {
                let mut block = block.clone();
                block.tail_opt = sub(&block.tail_opt);
                out.block = Some(Arc::new(block));
            }
            ExprNode::BlockExpr(out)
        }
        ExprNode::RangeExpr(node) => ExprNode::RangeExpr(ast::RangeExpr { lhs: sub(&node.lhs), rhs: sub(&node.rhs), ..node.clone() }),
        ExprNode::EntryExpr(node) => ExprNode::EntryExpr(ast::EntryExpr { expr: sub(&node.expr) }),
        _ => return expr.clone(),
    };
    Some(Arc::new(ast::Expr { span: e.span.clone(), node }))
}

/// Whether the predicate follows from the facts in scope and the enclosing
/// procedure's precondition, or from constants alone.
fn provable_at_call(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, call: &ast::CallExpr, predicate: &ExprPtr) -> bool {
    let mut proof_ctx: StaticProofContext = type_ctx.proof_ctx.as_deref().cloned().unwrap_or_default();
    if let Some(precondition) = type_ctx.contract.map(|contract| &contract.precondition).filter(|pre| pre.is_some()) {
        add_predicate_facts(&mut proof_ctx, precondition);
    }
    let location = call.callee.as_deref().or(predicate.as_deref()).map(|expr| expr.span.clone()).unwrap_or_default();
    static_proof_at(&proof_ctx, &location, predicate).provable || prove_simple_predicate(ctx, predicate) == Some(true)
}

fn argument_bindings(params: &[ast::Param], args: &[ast::Arg]) -> Bindings {
    params.iter().zip(args).map(|(param, arg)| (param.name.clone(), arg.value.clone())).collect()
}

/// The callee's precondition must hold of the arguments, unless it is checked at run
/// time.
pub fn check_call_site_precondition(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    call: &ast::CallExpr,
    callee: Option<ProcLike<'_>>,
) -> Option<&'static str> {
    let callee = callee?;
    let precondition = callee.contract().as_ref().map(|contract| &contract.precondition).filter(|pre| pre.is_some())?;
    if callee.params().len() != call.args.len() {
        return None;
    }
    let pre_subst = substitute_foreign_predicate(ctx, precondition, &argument_bindings(callee.params(), &call.args));
    (!provable_at_call(ctx, type_ctx, call, &pre_subst) && !type_ctx.contract_dynamic).then_some("E-SEM-2801")
}

/// A foreign procedure's `assumes` clauses must hold of the arguments, unless the
/// procedure asks for them to be checked at run time.
pub fn check_foreign_static_assumes(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    call: &ast::CallExpr,
    module: &ast::ASTModule,
    proc: &ast::ExternProcDecl,
) -> Option<&'static str> {
    if resolve_verification_mode_attribute(&proc.attrs) == Some(VerificationModeAttribute::Dynamic) {
        return None;
    }
    let clauses = proc.foreign_contracts_opt.as_ref()?;
    if proc.params.len() != call.args.len() {
        return None;
    }
    // The clauses may also speak of the module's literal constants.
    let mut bindings = argument_bindings(&proc.params, &call.args);
    for item in &module.items {
        let ast::ASTItem::StaticDecl(decl) = item else {
            continue;
        };
        if decl.r#mut != ast::Mutability::Let {
            continue;
        }
        if let (Some(ast::PatternNode::IdentifierPattern(ident_pat)), Some(ExprNode::LiteralExpr(_))) =
            (decl.binding.pat.as_deref().map(|pat| &pat.node), decl.binding.init.as_deref().map(|init| &init.node))
        {
            bindings.push((ident_pat.name.clone(), decl.binding.init.clone()));
        }
    }
    for clause in clauses.iter().filter(|clause| clause.kind == ast::ForeignContractKind::Assumes) {
        for pred in clause.predicates.iter().filter(|pred| pred.is_some()) {
            let substituted = substitute_foreign_predicate(ctx, pred, &bindings);
            if !provable_at_call(ctx, type_ctx, call, &substituted) {
                return Some("E-SEM-2850");
            }
        }
    }
    None
}
