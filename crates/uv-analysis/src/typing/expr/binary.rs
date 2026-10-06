//! Typing binary expressions.
//!
//! Operands are compared through their aliases: two types count as the same when each
//! is a subtype of the other. Where the operand types differ, one operand may still be
//! checked against the type of the other, which lets a literal take the other's type.

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr};
use uv_source::lexer::token::TokenKind;

use crate::caps::builtin_paths::is_capability_class_path;
use crate::composite::classes::type_implements_class;
use crate::context::ScopeContext;
use crate::resolve::scopes::path_key_of;
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::check_expr::{get_prim_name, is_int_type};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::TypeEnv;
use crate::typing::type_expr::{check_expr_against, type_expr};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::{eq_type_in, ord_type, strip_perm_and_refine};
use crate::typing::types::*;

const OPERAND_TYPE_MISMATCH: &str = "E-SEM-2525";

fn is_float_type(name: &str) -> bool {
    matches!(name, "f16" | "f32" | "f64")
}

fn is_numeric_type(name: &str) -> bool {
    is_int_type(name) || is_float_type(name)
}

fn is_arith_op(op: &str) -> bool {
    matches!(op, "+" | "-" | "*" | "/" | "%" | "**")
}

fn is_bit_op(op: &str) -> bool {
    matches!(op, "&" | "|" | "^")
}

fn is_shift_op(op: &str) -> bool {
    matches!(op, "<<" | ">>")
}

fn is_eq_op(op: &str) -> bool {
    matches!(op, "==" | "!=")
}

fn is_ord_op(op: &str) -> bool {
    matches!(op, "<" | "<=" | ">" | ">=")
}

fn is_logic_op(op: &str) -> bool {
    matches!(op, "&&" | "||")
}

/// The path an expression spells, when it is a name or a qualified name.
fn path_from_expr(expr: &ExprPtr) -> Option<Vec<String>> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(vec![node.name.clone()]),
        ExprNode::QualifiedNameExpr(node) => Some([&node.path[..], std::slice::from_ref(&node.name)].concat()),
        ExprNode::PathExpr(node) => Some([&node.path[..], std::slice::from_ref(&node.name)].concat()),
        _ => None,
    }
}

/// `Type <: Class`: whether a type implements a class, as a `bool`. The answer is not
/// part of the type; only that both sides name what they should is checked.
fn type_class_satisfaction_expr(ctx: &ScopeContext<'_>, expr: &ast::BinaryExpr) -> ExprTypeResult {
    let lhs_type_ast = match expr.lhs.as_deref().map(|lhs| &lhs.node) {
        Some(ExprNode::TypeLiteralExpr(node)) => Some(node.r#type.clone()),
        _ => path_from_expr(&expr.lhs).map(|path| {
            let node = ast::TypeNode::TypePathType(ast::TypePathType { path, generic_args: Vec::new() });
            Some(std::sync::Arc::new(ast::Type { span: Span::default(), node }))
        }),
    };
    let (Some(lhs_type_ast), Some(class_path)) = (lhs_type_ast, path_from_expr(&expr.rhs)) else {
        return ExprTypeResult::failed(Some(OPERAND_TYPE_MISMATCH));
    };
    if !is_capability_class_path(&class_path) && !ctx.sigma.classes.contains_key(&path_key_of(&class_path)) {
        return ExprTypeResult::failed(Some("E-TYP-2305"));
    }
    match lower_type(ctx, &lhs_type_ast) {
        Ok(lowered) => {
            let _ = type_implements_class(ctx, &lowered, &class_path);
            ExprTypeResult::typed(make_type_prim("bool"))
        }
        Err(diag_id) => ExprTypeResult::failed(diag_id),
    }
}

type Related = Result<bool, Option<&'static str>>;

/// Each a subtype of the other.
fn alias_transparent_equiv(ctx: &ScopeContext<'_>, lhs: &TypeRef, rhs: &TypeRef) -> Related {
    let l2r = subtyping(ctx, lhs, rhs);
    if !l2r.ok {
        return Err(l2r.diag_id);
    }
    if !l2r.subtype {
        return Ok(false);
    }
    let r2l = subtyping(ctx, rhs, lhs);
    if !r2l.ok {
        return Err(r2l.diag_id);
    }
    Ok(r2l.subtype)
}

/// The primitive type a type is, directly or through aliases.
fn resolve_alias_transparent_prim(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<Option<String>, Option<&'static str>> {
    if ty.is_none() {
        return Ok(None);
    }
    if let Some(direct) = get_prim_name(ty) {
        return Ok(Some(direct.to_string()));
    }
    const CANDIDATES: [&str; 18] = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f16", "f32", "f64",
        "bool", "char", "()",
    ];
    for candidate in CANDIDATES {
        if alias_transparent_equiv(ctx, ty, &make_type_prim(candidate))? {
            return Ok(Some(candidate.to_string()));
        }
    }
    Ok(None)
}

/// The operand's type under permissions, refinements and aliases.
fn normalize_binary_core_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Option<&'static str>> {
    let mut current = strip_perm_and_refine(ty);
    for _ in 0..16 {
        let Some(TypeNode::Path { path, generic_args }) = current.as_deref().map(|ty| &ty.node) else {
            break;
        };
        let expanded = expand_type_alias_apply(ctx, path, generic_args)?;
        if expanded.is_none() {
            break;
        }
        current = strip_perm_and_refine(&expanded);
    }
    Ok(current)
}

struct OperandInfo {
    typed: ExprTypeResult,
    core: TypeRef,
    prim_name: Option<String>,
}

/// Why an operand could not be loaded: the rule, and for a logical operand the detail
/// and span of what failed.
struct OperandFailure {
    diag_id: Option<&'static str>,
    logical_detail: Option<(String, Option<Span>)>,
}

fn expr_kind_or_missing(expr: &ExprPtr) -> &'static str {
    expr.as_deref().map_or("<missing>", ast::expr_kind)
}

fn load_operand(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    env: &TypeEnv,
    resolve_prim: bool,
) -> Result<OperandInfo, OperandFailure> {
    let typed = type_expr(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), expr, env);
    if !typed.ok {
        let mut detail = format!("logical operand {} failed to type", expr_kind_or_missing(expr));
        if !typed.diag_detail.is_empty() {
            detail.push_str(": ");
            detail.push_str(&typed.diag_detail);
        }
        let span = typed.diag_span.clone().or_else(|| expr.as_deref().map(|expr| expr.span.clone()));
        return Err(OperandFailure { diag_id: typed.diag_id, logical_detail: Some((detail, span)) });
    }
    let plain = |diag_id| OperandFailure { diag_id, logical_detail: None };
    let core = normalize_binary_core_type(ctx, &typed.r#type).map_err(plain)?;
    let prim_name = if resolve_prim { resolve_alias_transparent_prim(ctx, &core).map_err(plain)? } else { None };
    Ok(OperandInfo { typed, core, prim_name })
}

fn operand_type_mismatch(op: &str) -> ExprTypeResult {
    ExprTypeResult {
        diag_id: Some(OPERAND_TYPE_MISMATCH),
        diag_detail: format!("operator '{op}' requires operands compatible with the operator's type rules"),
        ..Default::default()
    }
}

/// Whether the operand may be taken at the expected type: its type is a subtype, or the
/// expression checks against it.
fn try_check_operand_against(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ExprPtr,
    actual: &TypeRef,
    expected: &TypeRef,
    env: &TypeEnv,
) -> bool {
    if expected.is_none() {
        return false;
    }
    if actual.is_some() {
        let rel = subtyping(ctx, actual, expected);
        if rel.ok && rel.subtype {
            return true;
        }
    }
    check_expr_against(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), expr, expected, env).ok
}

/// The operands of a chain of one logical operator, left to right.
fn collect_logical_operands(expr: &ExprPtr, op: &str, operands: &mut Vec<ExprPtr>) {
    let mut stack = vec![expr.clone()];
    while let Some(current) = stack.pop() {
        let Some(e) = current.as_deref() else {
            continue;
        };
        match &e.node {
            ExprNode::BinaryExpr(binary) if binary.op == op && binary.lhs.is_some() && binary.rhs.is_some() => {
                stack.push(binary.rhs.clone());
                stack.push(binary.lhs.clone());
            }
            _ => operands.push(current),
        }
    }
}

/// `&&` and `||`: every operand of the chain is a `bool`.
fn type_logical_binary_chain(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::BinaryExpr, env: &TypeEnv) -> ExprTypeResult {
    let mut operands = Vec::new();
    collect_logical_operands(&expr.lhs, &expr.op, &mut operands);
    collect_logical_operands(&expr.rhs, &expr.op, &mut operands);
    let bool_type = make_type_prim("bool");
    for operand_expr in &operands {
        // What went wrong with this operand: a rule, or just where and what it was.
        let (diag_id, detail, span) = match load_operand(ctx, type_ctx, operand_expr, env, false) {
            Err(failure) => {
                let (detail, span) = failure.logical_detail.unwrap_or_default();
                (failure.diag_id, detail, span)
            }
            Ok(operand) => match alias_transparent_equiv(ctx, &operand.core, &bool_type) {
                Err(diag_id) => (diag_id, String::new(), None),
                Ok(true) => continue,
                Ok(false) if try_check_operand_against(ctx, type_ctx, operand_expr, &operand.core, &bool_type, env) => continue,
                Ok(false) => (
                    None,
                    format!(
                        "logical operand {} has type {}",
                        expr_kind_or_missing(operand_expr),
                        type_to_string(&operand.typed.r#type)
                    ),
                    operand_expr.as_deref().map(|expr| expr.span.clone()),
                ),
            },
        };
        let mut result = ExprTypeResult { diag_id, diag_detail: detail, diag_span: span, ..Default::default() };
        if result.diag_id.is_none() {
            let mismatch = operand_type_mismatch(&expr.op);
            result.diag_id = mismatch.diag_id;
            if result.diag_detail.is_empty() {
                result.diag_detail = mismatch.diag_detail;
            }
        }
        return result;
    }
    ExprTypeResult::typed(bool_type)
}

fn is_null_literal(expr: &ExprPtr) -> bool {
    matches!(expr.as_deref().map(|expr| &expr.node), Some(ExprNode::LiteralExpr(literal)) if literal.literal.kind == TokenKind::NullLiteral)
}

fn is_ord_prim(name: &str) -> bool {
    is_numeric_type(name) || name == "char"
}

pub fn type_binary_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::BinaryExpr, env: &TypeEnv) -> ExprTypeResult {
    let op = expr.op.as_str();
    if op == "<:" {
        return type_class_satisfaction_expr(ctx, expr);
    }
    // A raw pointer compared with `null`.
    if is_eq_op(op) {
        let (lhs_null, rhs_null) = (is_null_literal(&expr.lhs), is_null_literal(&expr.rhs));
        if lhs_null != rhs_null {
            let typed = type_expr(ctx, type_ctx, if lhs_null { &expr.rhs } else { &expr.lhs }, env);
            if !typed.ok {
                return ExprTypeResult::failed(typed.diag_id);
            }
            let raw = matches!(strip_perm_and_refine(&typed.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::RawPtr { .. }));
            return if raw { ExprTypeResult::typed(make_type_prim("bool")) } else { ExprTypeResult::default() };
        }
    }
    if is_logic_op(op) {
        return type_logical_binary_chain(ctx, type_ctx, expr, env);
    }
    let resolve_prims = is_arith_op(op) || is_bit_op(op) || is_shift_op(op) || is_ord_op(op);
    let lhs = match load_operand(ctx, type_ctx, &expr.lhs, env, resolve_prims) {
        Ok(info) => info,
        Err(failure) => return ExprTypeResult::failed(failure.diag_id),
    };
    let rhs = match load_operand(ctx, type_ctx, &expr.rhs, env, resolve_prims) {
        Ok(info) => info,
        Err(failure) => return ExprTypeResult::failed(failure.diag_id),
    };
    let prim = |name: &str| ExprTypeResult::typed(make_type_prim(name));
    let rhs_as_lhs = || try_check_operand_against(ctx, type_ctx, &expr.rhs, &rhs.core, &lhs.core, env);
    let lhs_as_rhs = || try_check_operand_against(ctx, type_ctx, &expr.lhs, &lhs.core, &rhs.core, env);
    // Operators whose result is the operand type: the operands agree on a type of the
    // wanted kind, or one of them can be taken at the other's.
    let same_kind = |wanted: fn(&str) -> bool| -> ExprTypeResult {
        let equiv = match alias_transparent_equiv(ctx, &lhs.core, &rhs.core) {
            Ok(equiv) => equiv,
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        };
        let lhs_prim = lhs.prim_name.as_deref().filter(|name| wanted(name));
        let rhs_prim = rhs.prim_name.as_deref().filter(|name| wanted(name));
        match (lhs_prim, rhs_prim) {
            (Some(name), _) if equiv => prim(name),
            (Some(name), _) if rhs_as_lhs() => prim(name),
            (_, Some(name)) if lhs_as_rhs() => prim(name),
            _ => operand_type_mismatch(op),
        }
    };
    // Operators whose result is `bool`, for operand types that support the comparison.
    let comparison = |lhs_ok: &dyn Fn() -> bool, rhs_ok: &dyn Fn() -> bool| -> ExprTypeResult {
        let equiv = match alias_transparent_equiv(ctx, &lhs.core, &rhs.core) {
            Ok(equiv) => equiv,
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        };
        let lhs_ok = lhs_ok();
        if lhs_ok && (equiv || rhs_as_lhs()) {
            return prim("bool");
        }
        if rhs_ok() && lhs_as_rhs() {
            return prim("bool");
        }
        operand_type_mismatch(op)
    };
    if is_arith_op(op) {
        return same_kind(is_numeric_type);
    }
    if is_bit_op(op) {
        return same_kind(is_int_type);
    }
    if is_shift_op(op) {
        let Some(name) = lhs.prim_name.as_deref().filter(|name| is_int_type(name)) else {
            return operand_type_mismatch(op);
        };
        let u32_type = make_type_prim("u32");
        return match alias_transparent_equiv(ctx, &rhs.core, &u32_type) {
            Err(diag_id) => ExprTypeResult::failed(diag_id),
            Ok(true) => prim(name),
            Ok(false) if try_check_operand_against(ctx, type_ctx, &expr.rhs, &rhs.core, &u32_type, env) => prim(name),
            Ok(false) => operand_type_mismatch(op),
        };
    }
    if is_eq_op(op) {
        return comparison(&|| eq_type_in(ctx, &lhs.core), &|| eq_type_in(ctx, &rhs.core));
    }
    if is_ord_op(op) {
        let ord = |info: &OperandInfo| ord_type(&info.core) || info.prim_name.as_deref().is_some_and(is_ord_prim);
        return comparison(&|| ord(&lhs), &|| ord(&rhs));
    }
    ExprTypeResult::default()
}
