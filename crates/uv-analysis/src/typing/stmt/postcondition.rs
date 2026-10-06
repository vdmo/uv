//! The postcondition at `return`: the returned value takes the place of `@result`, the
//! predicate is simplified where the value decides a branch, and what remains must
//! follow from the facts in scope.

use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr};

use crate::context::ScopeContext;
use crate::contracts::verification::{add_predicate_facts, evaluate_constant, static_proof_at, ConstValue, StaticProofContext};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::callbacks::ExprTypeFn;
use crate::typing::pattern::type_pattern_against_type;
use crate::typing::stmt::assign_stmt::type_expr_with_current_env;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{collect_pat_names, mut_of, TypeEnv};
use crate::typing::type_predicates::strip_perm_and_refine;
use crate::typing::types::*;

fn make(span: &Span, node: ExprNode) -> ExprPtr {
    Some(Arc::new(ast::Expr { span: span.clone(), node }))
}

fn pattern_binds_name(pattern: &ast::PatternPtr, name: &str) -> bool {
    let Some(pattern) = pattern.as_deref() else {
        return false;
    };
    let mut names = Vec::new();
    collect_pat_names(pattern, &mut names);
    names.contains(&id_key_of(name))
}

fn map_args(args: &ast::ApplyArgs, f: &dyn Fn(&ExprPtr) -> ExprPtr) -> ast::ApplyArgs {
    match args {
        ast::ApplyArgs::ParenArgs(paren) => {
            let mut paren = paren.clone();
            for arg in &mut paren.args {
                arg.value = f(&arg.value);
            }
            ast::ApplyArgs::ParenArgs(paren)
        }
        ast::ApplyArgs::BraceArgs(brace) => {
            let mut brace = brace.clone();
            for field in &mut brace.fields {
                field.value = f(&field.value);
            }
            ast::ApplyArgs::BraceArgs(brace)
        }
    }
}

fn with_tail(block_expr: &ast::BlockExpr, f: &dyn Fn(&ExprPtr) -> ExprPtr) -> ast::BlockExpr {
    let mut out = block_expr.clone();
    if let Some(block) = out.block.as_deref() {
        let mut block = block.clone();
        block.tail_opt = f(&block.tail_opt);
        out.block = Some(Arc::new(block));
    }
    out
}

/// The expression with every use of the name replaced, except where a pattern rebinds
/// it.
pub fn substitute_identifier_expr(expr: &ExprPtr, name: &str, replacement: &ExprPtr) -> ExprPtr {
    substitute_ident(expr, name, replacement, false)
}

/// The predicate of a refinement with the value for `self`. This substitution reaches
/// into copies and enum payloads but not into blocks, and does not look at patterns.
pub fn substitute_refinement_self(predicate: &ExprPtr, value: &ExprPtr) -> ExprPtr {
    substitute_ident(predicate, "self", value, true)
}

fn substitute_ident(expr: &ExprPtr, name: &str, replacement: &ExprPtr, for_refinement: bool) -> ExprPtr {
    let Some(e) = expr.as_deref() else {
        return expr.clone();
    };
    let sub = |inner: &ExprPtr| substitute_ident(inner, name, replacement, for_refinement);
    let pattern_binds_name = |pattern: &ast::PatternPtr, name: &str| !for_refinement && pattern_binds_name(pattern, name);
    let sub_args = |args: &[ast::Arg]| -> Vec<ast::Arg> { args.iter().map(|arg| ast::Arg { value: sub(&arg.value), ..arg.clone() }).collect() };
    let node = match &e.node {
        ExprNode::IdentifierExpr(ident) => return if id_eq(&ident.name, name) { replacement.clone() } else { expr.clone() },
        ExprNode::BinaryExpr(node) => ExprNode::BinaryExpr(ast::BinaryExpr { lhs: sub(&node.lhs), rhs: sub(&node.rhs), ..node.clone() }),
        ExprNode::UnaryExpr(node) => ExprNode::UnaryExpr(ast::UnaryExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::FieldAccessExpr(node) => ExprNode::FieldAccessExpr(ast::FieldAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::TupleAccessExpr(node) => ExprNode::TupleAccessExpr(ast::TupleAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::IndexAccessExpr(node) => ExprNode::IndexAccessExpr(ast::IndexAccessExpr { base: sub(&node.base), index: sub(&node.index) }),
        ExprNode::CallExpr(node) => ExprNode::CallExpr(ast::CallExpr { callee: sub(&node.callee), args: sub_args(&node.args), ..node.clone() }),
        ExprNode::QualifiedApplyExpr(node) => ExprNode::QualifiedApplyExpr(ast::QualifiedApplyExpr { args: map_args(&node.args, &sub), ..node.clone() }),
        ExprNode::MethodCallExpr(node) => {
            ExprNode::MethodCallExpr(ast::MethodCallExpr { receiver: sub(&node.receiver), args: sub_args(&node.args), ..node.clone() })
        }
        ExprNode::IfExpr(node) => {
            ExprNode::IfExpr(ast::IfExpr { cond: sub(&node.cond), then_expr: sub(&node.then_expr), else_expr: sub(&node.else_expr) })
        }
        ExprNode::IfIsExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee);
            if !pattern_binds_name(&node.pattern, name) {
                out.then_expr = sub(&node.then_expr);
            }
            out.else_expr = sub(&node.else_expr);
            ExprNode::IfIsExpr(out)
        }
        ExprNode::IfCaseExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee);
            for arm in &mut out.cases {
                if !pattern_binds_name(&arm.pattern, name) {
                    arm.body = sub(&arm.body);
                }
            }
            out.else_expr = sub(&node.else_expr);
            ExprNode::IfCaseExpr(out)
        }
        ExprNode::CastExpr(node) => ExprNode::CastExpr(ast::CastExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::RangeExpr(node) => ExprNode::RangeExpr(ast::RangeExpr { lhs: sub(&node.lhs), rhs: sub(&node.rhs), ..node.clone() }),
        ExprNode::DerefExpr(node) => ExprNode::DerefExpr(ast::DerefExpr { value: sub(&node.value) }),
        ExprNode::AddressOfExpr(node) => ExprNode::AddressOfExpr(ast::AddressOfExpr { place: sub(&node.place) }),
        ExprNode::MoveExpr(node) => ExprNode::MoveExpr(ast::MoveExpr { place: sub(&node.place) }),
        ExprNode::AllocExpr(node) => ExprNode::AllocExpr(ast::AllocExpr { value: sub(&node.value), ..node.clone() }),
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
        ExprNode::BlockExpr(node) if !for_refinement => ExprNode::BlockExpr(with_tail(node, &sub)),
        ExprNode::CopyExpr(node) if for_refinement => ExprNode::CopyExpr(ast::CopyExpr { value: sub(&node.value) }),
        ExprNode::EnumLiteralExpr(node) if for_refinement => {
            let mut out = node.clone();
            match &mut out.payload_opt {
                Some(ast::EnumPayload::EnumPayloadParen(paren)) => {
                    for element in &mut paren.elements {
                        *element = sub(element);
                    }
                }
                Some(ast::EnumPayload::EnumPayloadBrace(brace)) => {
                    for field in &mut brace.fields {
                        field.value = sub(&field.value);
                    }
                }
                None => {}
            }
            ExprNode::EnumLiteralExpr(out)
        }
        ExprNode::PropagateExpr(node) => ExprNode::PropagateExpr(ast::PropagateExpr { value: sub(&node.value) }),
        ExprNode::EntryExpr(node) => ExprNode::EntryExpr(ast::EntryExpr { expr: sub(&node.expr) }),
        _ => return expr.clone(),
    };
    make(&e.span, node)
}

/// Whether `@entry(expr)` equals `expr` now: only immutable bindings are involved.
fn entry_expr_current_value_stable(expr: &ExprPtr, env: Option<&TypeEnv>) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    match &e.node {
        ExprNode::LiteralExpr(_) | ExprNode::PtrNullExpr(_) | ExprNode::ResultExpr(_) => true,
        ExprNode::IdentifierExpr(node) => env.is_some_and(|env| mut_of(env, &node.name) == Some(ast::Mutability::Let)),
        ExprNode::FieldAccessExpr(node) => entry_expr_current_value_stable(&node.base, env),
        ExprNode::TupleAccessExpr(node) => entry_expr_current_value_stable(&node.base, env),
        ExprNode::IndexAccessExpr(node) => entry_expr_current_value_stable(&node.base, env) && entry_expr_current_value_stable(&node.index, env),
        ExprNode::UnaryExpr(node) => entry_expr_current_value_stable(&node.value, env),
        ExprNode::BinaryExpr(node) => entry_expr_current_value_stable(&node.lhs, env) && entry_expr_current_value_stable(&node.rhs, env),
        _ => false,
    }
}

/// The postcondition with the returned value for `@result`, and `@entry(e)` replaced
/// by `e` where its value cannot have changed.
fn substitute_result_entry(expr: &ExprPtr, result_expr: &ExprPtr, env: Option<&TypeEnv>) -> ExprPtr {
    let Some(e) = expr.as_deref() else {
        return expr.clone();
    };
    let sub = |inner: &ExprPtr| substitute_result_entry(inner, result_expr, env);
    let sub_args = |args: &[ast::Arg]| -> Vec<ast::Arg> { args.iter().map(|arg| ast::Arg { value: sub(&arg.value), ..arg.clone() }).collect() };
    let node = match &e.node {
        ExprNode::ResultExpr(_) => return result_expr.clone(),
        ExprNode::BinaryExpr(node) => ExprNode::BinaryExpr(ast::BinaryExpr { lhs: sub(&node.lhs), rhs: sub(&node.rhs), ..node.clone() }),
        ExprNode::UnaryExpr(node) => ExprNode::UnaryExpr(ast::UnaryExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::EntryExpr(node) => {
            return if entry_expr_current_value_stable(&node.expr, env) { sub(&node.expr) } else { expr.clone() };
        }
        ExprNode::FieldAccessExpr(node) => ExprNode::FieldAccessExpr(ast::FieldAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::TupleAccessExpr(node) => ExprNode::TupleAccessExpr(ast::TupleAccessExpr { base: sub(&node.base), ..node.clone() }),
        ExprNode::IndexAccessExpr(node) => ExprNode::IndexAccessExpr(ast::IndexAccessExpr { base: sub(&node.base), index: sub(&node.index) }),
        ExprNode::IfExpr(node) => {
            ExprNode::IfExpr(ast::IfExpr { cond: sub(&node.cond), then_expr: sub(&node.then_expr), else_expr: sub(&node.else_expr) })
        }
        ExprNode::IfIsExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee);
            out.then_expr = sub(&node.then_expr);
            out.else_expr = sub(&node.else_expr);
            ExprNode::IfIsExpr(out)
        }
        ExprNode::IfCaseExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee);
            for arm in &mut out.cases {
                arm.body = sub(&arm.body);
            }
            out.else_expr = sub(&node.else_expr);
            ExprNode::IfCaseExpr(out)
        }
        ExprNode::CallExpr(node) => ExprNode::CallExpr(ast::CallExpr { callee: sub(&node.callee), args: sub_args(&node.args), ..node.clone() }),
        ExprNode::QualifiedApplyExpr(node) => ExprNode::QualifiedApplyExpr(ast::QualifiedApplyExpr { args: map_args(&node.args, &sub), ..node.clone() }),
        ExprNode::MethodCallExpr(node) => {
            ExprNode::MethodCallExpr(ast::MethodCallExpr { receiver: sub(&node.receiver), args: sub_args(&node.args), ..node.clone() })
        }
        ExprNode::CastExpr(node) => ExprNode::CastExpr(ast::CastExpr { value: sub(&node.value), ..node.clone() }),
        ExprNode::RecordExpr(node) => {
            let mut out = node.clone();
            for field in &mut out.fields {
                field.value = sub(&field.value);
            }
            ExprNode::RecordExpr(out)
        }
        ExprNode::BlockExpr(node) => ExprNode::BlockExpr(with_tail(node, &sub)),
        _ => return expr.clone(),
    };
    make(&e.span, node)
}

struct Simplifier<'a, 'c, 't> {
    ctx: &'a ScopeContext<'c>,
    type_ctx: &'a StmtTypeContext<'t>,
    env: &'a TypeEnv,
    type_expr: ExprTypeFn<'a>,
}

impl Simplifier<'_, '_, '_> {
    /// Whether a pattern is known to match the scrutinee, from its type alone, and
    /// what it binds when it does. `None` when the type does not decide.
    fn match_pattern_for_known_scrutinee(&self, scrutinee: &ExprPtr, pattern: &ast::PatternPtr) -> Option<Option<Vec<(String, TypeRef)>>> {
        if scrutinee.is_none() || pattern.is_none() {
            return None;
        }
        let typed = type_expr_with_current_env(self.ctx, self.type_ctx, self.env, self.type_expr, scrutinee);
        if !typed.ok {
            return None;
        }
        let base = strip_perm_and_refine(&typed.r#type);
        // A union, type variable, dynamic or opaque type may hold more than one shape.
        if matches!(
            base.as_deref().map(|ty| &ty.node),
            None | Some(TypeNode::Union(_) | TypeNode::Var(_) | TypeNode::Dynamic(_) | TypeNode::Opaque { .. })
        ) {
            return None;
        }
        match type_pattern_against_type(self.ctx, pattern, &base) {
            Ok(bindings) => Some(Some(bindings)),
            Err(None) => Some(None),
            Err(Some(_)) => None,
        }
    }

    fn simplify_matched_pattern_body(&self, pattern: &ast::PatternPtr, scrutinee: &ExprPtr, body: &ExprPtr, bindings: &[(String, TypeRef)]) -> ExprPtr {
        let binds_whole_value = matches!(
            pattern.as_deref().map(|pattern| &pattern.node),
            Some(ast::PatternNode::IdentifierPattern(_) | ast::PatternNode::TypedPattern(_) | ast::PatternNode::WildcardPattern(_))
        );
        let mut bound = body.clone();
        if binds_whole_value {
            for (name, _) in bindings {
                bound = substitute_identifier_expr(&bound, name, scrutinee);
            }
        }
        self.simplify(&bound)
    }

    /// Reduces the predicate where the returned value decides it: a field of a record
    /// literal, a branch on a constant, a pattern its type settles.
    fn simplify(&self, expr: &ExprPtr) -> ExprPtr {
        let Some(e) = expr.as_deref() else {
            return expr.clone();
        };
        let node = match &e.node {
            ExprNode::BinaryExpr(node) => {
                ExprNode::BinaryExpr(ast::BinaryExpr { lhs: self.simplify(&node.lhs), rhs: self.simplify(&node.rhs), ..node.clone() })
            }
            ExprNode::UnaryExpr(node) => ExprNode::UnaryExpr(ast::UnaryExpr { value: self.simplify(&node.value), ..node.clone() }),
            ExprNode::FieldAccessExpr(node) => {
                let base = self.simplify(&node.base);
                if let Some(ExprNode::RecordExpr(record)) = base.as_deref().map(|base| &base.node) {
                    if let Some(field) = record.fields.iter().find(|field| id_eq(&field.name, &node.name)) {
                        return self.simplify(&field.value);
                    }
                }
                ExprNode::FieldAccessExpr(ast::FieldAccessExpr { base, ..node.clone() })
            }
            ExprNode::TupleAccessExpr(node) => ExprNode::TupleAccessExpr(ast::TupleAccessExpr { base: self.simplify(&node.base), ..node.clone() }),
            ExprNode::IndexAccessExpr(node) => {
                ExprNode::IndexAccessExpr(ast::IndexAccessExpr { base: self.simplify(&node.base), index: self.simplify(&node.index) })
            }
            ExprNode::BlockExpr(node) => {
                if let Some(block) = node.block.as_deref().filter(|block| block.stmts.is_empty() && block.tail_opt.is_some()) {
                    return self.simplify(&block.tail_opt);
                }
                ExprNode::BlockExpr(with_tail(node, &|tail| self.simplify(tail)))
            }
            ExprNode::IfExpr(node) => {
                let cond = self.simplify(&node.cond);
                if let ConstValue::Bool(holds) = evaluate_constant(&cond) {
                    return self.simplify(if holds { &node.then_expr } else { &node.else_expr });
                }
                ExprNode::IfExpr(ast::IfExpr { cond, then_expr: self.simplify(&node.then_expr), else_expr: self.simplify(&node.else_expr) })
            }
            ExprNode::IfIsExpr(node) => {
                let scrutinee = self.simplify(&node.scrutinee);
                match self.match_pattern_for_known_scrutinee(&scrutinee, &node.pattern) {
                    Some(Some(bindings)) => return self.simplify_matched_pattern_body(&node.pattern, &scrutinee, &node.then_expr, &bindings),
                    Some(None) if node.else_expr.is_some() => return self.simplify(&node.else_expr),
                    _ => {}
                }
                let mut out = node.clone();
                out.scrutinee = scrutinee;
                out.then_expr = self.simplify(&node.then_expr);
                out.else_expr = self.simplify(&node.else_expr);
                ExprNode::IfIsExpr(out)
            }
            ExprNode::IfCaseExpr(node) => {
                let scrutinee = self.simplify(&node.scrutinee);
                let mut all_arms_decidable_false = true;
                for arm in &node.cases {
                    match self.match_pattern_for_known_scrutinee(&scrutinee, &arm.pattern) {
                        None => {
                            all_arms_decidable_false = false;
                            break;
                        }
                        Some(Some(bindings)) => return self.simplify_matched_pattern_body(&arm.pattern, &scrutinee, &arm.body, &bindings),
                        Some(None) => {}
                    }
                }
                if all_arms_decidable_false && node.else_expr.is_some() {
                    return self.simplify(&node.else_expr);
                }
                let mut out = node.clone();
                out.scrutinee = scrutinee;
                for arm in &mut out.cases {
                    arm.body = self.simplify(&arm.body);
                }
                out.else_expr = self.simplify(&node.else_expr);
                ExprNode::IfCaseExpr(out)
            }
            _ => return expr.clone(),
        };
        make(&e.span, node)
    }
}

/// Proves the enclosing procedure's postcondition for the value returned here.
pub fn verify_postcondition_at_return(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    env: &TypeEnv,
    type_expr: ExprTypeFn<'_>,
    return_value: &ExprPtr,
) -> Option<&'static str> {
    let contract = type_ctx.contract?;
    let postcondition = contract.postcondition.as_deref()?;
    if type_ctx.test_postcondition_runtime {
        return None;
    }
    // Returning nothing returns the unit value.
    let result_expr = match return_value {
        Some(_) => return_value.clone(),
        None => make(&postcondition.span, ExprNode::TupleExpr(ast::TupleExpr::default())),
    };
    let live_env = type_ctx.env_ref.as_ref().map(|cell| cell.borrow().clone());
    let pred = substitute_result_entry(&contract.postcondition, &result_expr, live_env.as_ref());
    let simplified = Simplifier { ctx, type_ctx, env, type_expr }.simplify(&pred);
    let mut proof_ctx: StaticProofContext = type_ctx.proof_ctx.as_deref().cloned().unwrap_or_default();
    if contract.precondition.is_some() {
        add_predicate_facts(&mut proof_ctx, &contract.precondition);
    }
    let location = return_value.as_deref().or(simplified.as_deref()).map(|expr| expr.span.clone()).unwrap_or_default();
    let proof = static_proof_at(&proof_ctx, &location, &simplified);
    (!proof.provable && !type_ctx.contract_dynamic).then_some("E-SEM-2801")
}
