//! The warning for `inline(always)` on a procedure that cannot always be inlined: one that
//! calls itself, or whose address is taken. See `ProcedureWarningInlineAlways`.

use std::sync::Arc;

use uv_source::ast::{self, ApplyArgs, ArraySegment, AttributeArgValue, AttributeItem, EnumPayload, ExprNode as E, ExprPtr, Stmt};
use uv_source::attributes::attrs;

use crate::context::ScopeContext;
use crate::resolve::scopes::id_eq;

type BlockPtr = Option<Arc<ast::Block>>;

fn normalize_attr_literal(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"') || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'')) {
        return &value[1..value.len() - 1];
    }
    value
}

fn has_inline_always(attr_list: &[AttributeItem]) -> bool {
    for attr in attr_list.iter().filter(|attr| attr.name.full_name == attrs::INLINE) {
        if attr.args.is_empty() {
            return false;
        }
        let always = attr.args.iter().any(|arg| {
            arg.key.is_none() && matches!(&arg.value, AttributeArgValue::Token(token) if normalize_attr_literal(&token.lexeme) == "always")
        });
        if always {
            return true;
        }
    }
    false
}

/// The expressions and blocks directly inside an expression, as far as the two analyses
/// look. `callee_names` is whether a callee that is only a name counts as a child.
fn expr_any(expr: &ExprPtr, pred: &dyn Fn(&ExprNode) -> bool, callee_names: bool) -> bool {
    let Some(expr) = expr.as_deref() else {
        return false;
    };
    if pred(&expr.node) {
        return true;
    }
    let one = |e: &ExprPtr| expr_any(e, pred, callee_names);
    let many = |es: &[ExprPtr]| es.iter().any(&one);
    let args = |args: &[ast::Arg]| args.iter().any(|arg| one(&arg.value));
    let fields = |fields: &[ast::FieldInit]| fields.iter().any(|field| one(&field.value));
    let block = |b: &BlockPtr| block_any(b, pred, callee_names);
    match &expr.node {
        E::CallExpr(node) => {
            let name_only = matches!(node.callee.as_deref().map(|callee| &callee.node), Some(E::IdentifierExpr(_) | E::PathExpr(_) | E::QualifiedNameExpr(_)));
            ((callee_names || !name_only) && one(&node.callee)) || args(&node.args)
        }
        E::QualifiedApplyExpr(node) => match &node.args {
            ApplyArgs::ParenArgs(paren) => args(&paren.args),
            ApplyArgs::BraceArgs(brace) => fields(&brace.fields),
        },
        E::MethodCallExpr(node) => one(&node.receiver) || args(&node.args),
        E::BinaryExpr(node) => one(&node.lhs) || one(&node.rhs),
        E::UnaryExpr(node) => one(&node.value),
        E::FieldAccessExpr(node) => one(&node.base),
        E::TupleAccessExpr(node) => one(&node.base),
        E::IndexAccessExpr(node) => one(&node.base) || one(&node.index),
        E::CastExpr(node) => one(&node.value),
        E::RangeExpr(node) => one(&node.lhs) || one(&node.rhs),
        E::DerefExpr(node) => one(&node.value),
        E::AddressOfExpr(node) => one(&node.place),
        E::MoveExpr(node) => one(&node.place),
        E::AllocExpr(node) => one(&node.value),
        E::TupleExpr(node) => many(&node.elements),
        E::ArrayExpr(node) => node.elements.iter().any(|segment| match segment {
            ArraySegment::ArrayElemSegment(segment) => one(&segment.value),
            ArraySegment::ArrayRepeatSegment(segment) => one(&segment.value) || one(&segment.count),
        }),
        E::ArrayRepeatExpr(node) => one(&node.value) || one(&node.count),
        E::RecordExpr(node) => fields(&node.fields),
        E::EnumLiteralExpr(node) => match &node.payload_opt {
            None => false,
            Some(EnumPayload::EnumPayloadParen(payload)) => many(&payload.elements),
            Some(EnumPayload::EnumPayloadBrace(payload)) => fields(&payload.fields),
        },
        E::IfExpr(node) => one(&node.cond) || one(&node.then_expr) || one(&node.else_expr),
        E::IfCaseExpr(node) => one(&node.scrutinee) || node.cases.iter().any(|arm| one(&arm.body)) || one(&node.else_expr),
        E::IfIsExpr(node) => one(&node.scrutinee) || one(&node.then_expr) || one(&node.else_expr),
        E::BlockExpr(node) => block(&node.block),
        E::UnsafeBlockExpr(node) => block(&node.block),
        E::PropagateExpr(node) => one(&node.value),
        E::EntryExpr(node) => one(&node.expr),
        _ => false,
    }
}

type ExprNode = E;

fn stmt_any(stmt: &Stmt, pred: &dyn Fn(&ExprNode) -> bool, callee_names: bool) -> bool {
    let one = |e: &ExprPtr| expr_any(e, pred, callee_names);
    let block = |b: &BlockPtr| block_any(b, pred, callee_names);
    match stmt {
        Stmt::LetStmt(node) => one(&node.binding.init),
        Stmt::VarStmt(node) => one(&node.binding.init),
        Stmt::AssignStmt(node) => one(&node.place) || one(&node.value),
        Stmt::CompoundAssignStmt(node) => one(&node.place) || one(&node.value),
        Stmt::ExprStmt(node) => one(&node.value),
        Stmt::ReturnStmt(node) => one(&node.value_opt),
        Stmt::BreakStmt(node) => one(&node.value_opt),
        Stmt::DeferStmt(node) => block(&node.body),
        Stmt::RegionStmt(node) => one(&node.opts_opt) || block(&node.body),
        Stmt::FrameStmt(node) => block(&node.body),
        Stmt::UnsafeBlockStmt(node) => block(&node.body),
        Stmt::KeyBlockStmt(node) => block(&node.body),
        _ => false,
    }
}

fn block_any(block: &BlockPtr, pred: &dyn Fn(&ExprNode) -> bool, callee_names: bool) -> bool {
    let Some(block) = block.as_deref() else {
        return false;
    };
    block.stmts.iter().any(|stmt| stmt_any(stmt, pred, callee_names)) || expr_any(&block.tail_opt, pred, callee_names)
}

fn path_eq(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| id_eq(a, b))
}

/// Whether the body names the procedure in a call.
fn contains_direct_call(body: &BlockPtr, proc_name: &str) -> bool {
    let pred = |node: &ExprNode| match node {
        E::CallExpr(call) => match call.callee.as_deref().map(|callee| &callee.node) {
            Some(E::IdentifierExpr(ident)) => id_eq(&ident.name, proc_name),
            Some(E::PathExpr(path)) => path.path.is_empty() && id_eq(&path.name, proc_name),
            _ => false,
        },
        E::QualifiedApplyExpr(apply) => id_eq(&apply.name, proc_name),
        _ => false,
    };
    block_any(body, &pred, true)
}

fn item_contains_value_reference(item: &ast::ASTItem, current_module: &[String], target_module: &[String], proc_name: &str) -> bool {
    // A name used as a value, not as the callee of a call.
    let pred = |node: &ExprNode| match node {
        E::IdentifierExpr(ident) => path_eq(current_module, target_module) && id_eq(&ident.name, proc_name),
        E::PathExpr(path) => {
            id_eq(&path.name, proc_name) && if path.path.is_empty() { path_eq(current_module, target_module) } else { path_eq(&path.path, target_module) }
        }
        E::QualifiedNameExpr(name) => id_eq(&name.name, proc_name) && path_eq(&name.path, target_module),
        _ => false,
    };
    let block = |b: &BlockPtr| block_any(b, &pred, false);
    match item {
        ast::ASTItem::ProcedureDecl(node) => block(&node.body),
        ast::ASTItem::StaticDecl(node) => expr_any(&node.binding.init, &pred, false),
        ast::ASTItem::RecordDecl(node) => node.members.iter().any(|member| match member {
            ast::RecordMember::FieldDecl(field) => expr_any(&field.init_opt, &pred, false),
            ast::RecordMember::MethodDecl(method) => block(&method.body),
            _ => false,
        }),
        ast::ASTItem::ClassDecl(node) => node.items.iter().any(|class_item| matches!(class_item, ast::ClassItem::ClassMethodDecl(method) if block(&method.body_opt))),
        ast::ASTItem::ModalDecl(node) => node.states.iter().flat_map(|state| &state.members).any(|member| match member {
            ast::StateMember::StateMethodDecl(method) => block(&method.body),
            ast::StateMember::TransitionDecl(transition) => block(&transition.body),
            _ => false,
        }),
        _ => false,
    }
}

fn has_procedure_address_taken(ctx: &ScopeContext<'_>, target_module: &[String], proc_name: &str) -> bool {
    ctx.sigma.mods.iter().any(|module| module.items.iter().any(|item| item_contains_value_reference(item, &module.path, target_module, proc_name)))
}

/// `W-MOD-2452` when the procedure is `inline(always)` but recursive or has its address taken.
pub fn procedure_warning_inline_always(ctx: &ScopeContext<'_>, decl: &ast::ProcedureDecl, module_path: &[String]) -> Option<&'static str> {
    if !has_inline_always(&decl.attrs) {
        return None;
    }
    let recursive = contains_direct_call(&decl.body, &decl.name);
    (recursive || has_procedure_address_taken(ctx, module_path, &decl.name)).then_some("W-MOD-2452")
}
