//! Whether the value a call returns is one the caller becomes responsible for, judged
//! from the callee's body: a callee that only hands back something it was lent returns
//! an alias. See `CallResultHasResponsibility`.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::composite::record_methods::{lookup_method_static, recv_mode_of};
use crate::context::{ScopeContext, TypeDecl};
use crate::modal::lookup::lookup_state_method_decl;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};
use crate::typing::expr_store::{selected_call_target, stored_expr_type};
use crate::typing::type_env::collect_pat_names;
use crate::typing::type_predicates::strip_perm;
use crate::typing::types::{TypeNode, TypeRef};

/// For each name in scope, whether it holds a value it is responsible for.
type ResponsibilityEnv = HashMap<String, bool>;

thread_local! {
    /// Bodies being analysed; a recursive call says nothing.
    static ACTIVE_BODIES: RefCell<HashSet<usize>> = RefCell::default();
}

fn place_root(expr: &ExprPtr) -> Option<String> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(id_key_of(&node.name)),
        ExprNode::FieldAccessExpr(node) => place_root(&node.base),
        ExprNode::TupleAccessExpr(node) => place_root(&node.base),
        ExprNode::IndexAccessExpr(node) => place_root(&node.base),
        ExprNode::DerefExpr(node) => place_root(&node.value),
        _ => None,
    }
}

fn analyze_block_returns(ctx: &ScopeContext<'_>, body: &ast::BlockPtr, mut env: ResponsibilityEnv) -> Option<bool> {
    let body = body.as_deref()?;
    let mut returns = Vec::new();
    for stmt in &body.stmts {
        analyze_stmt_returns(ctx, stmt, &mut env, &mut returns);
    }
    if body.tail_opt.is_some() {
        returns.push(expr_result_has_responsibility_with_env(ctx, &body.tail_opt, &env).unwrap_or(true));
    }
    (!returns.is_empty()).then(|| returns.contains(&true))
}

fn analyze_stmt_returns(ctx: &ScopeContext<'_>, stmt: &Stmt, env: &mut ResponsibilityEnv, returns: &mut Vec<bool>) {
    let add_binding = |binding: &ast::Binding, env: &mut ResponsibilityEnv| {
        let mut names = Vec::new();
        if let Some(pat) = binding.pat.as_deref() {
            collect_pat_names(pat, &mut names);
        }
        if names.is_empty() {
            return;
        }
        let has_resp = expr_result_has_responsibility_with_env(ctx, &binding.init, env).unwrap_or(true);
        for name in names {
            env.insert(name, has_resp);
        }
    };
    let block = match stmt {
        Stmt::LetStmt(node) => return add_binding(&node.binding, env),
        Stmt::VarStmt(node) => return add_binding(&node.binding, env),
        Stmt::ReturnStmt(node) => {
            returns.push(expr_result_has_responsibility_with_env(ctx, &node.value_opt, env).unwrap_or(true));
            return;
        }
        Stmt::RegionStmt(node) => &node.body,
        Stmt::FrameStmt(node) => &node.body,
        Stmt::UnsafeBlockStmt(node) => &node.body,
        Stmt::CtStmt(node) => &node.body,
        Stmt::KeyBlockStmt(node) => &node.body,
        Stmt::DeferStmt(node) => &node.body,
        _ => return,
    };
    returns.extend(analyze_block_returns(ctx, block, env.clone()));
}

fn callable_return_has_responsibility(ctx: &ScopeContext<'_>, body: &ast::BlockPtr, env: ResponsibilityEnv) -> Option<bool> {
    let key = std::sync::Arc::as_ptr(body.as_ref()?) as usize;
    if !ACTIVE_BODIES.with_borrow_mut(|active| active.insert(key)) {
        return None;
    }
    let result = analyze_block_returns(ctx, body, env);
    ACTIVE_BODIES.with_borrow_mut(|active| active.remove(&key));
    result
}

fn params_env(params: &[ast::Param]) -> ResponsibilityEnv {
    params.iter().map(|param| (id_key_of(&param.name), param.mode.is_some())).collect()
}

fn method_body_return_has_responsibility(
    ctx: &ScopeContext<'_>,
    receiver: &ast::Receiver,
    params: &[ast::Param],
    body: &ast::BlockPtr,
) -> Option<bool> {
    let mut env = ResponsibilityEnv::new();
    env.insert(id_key_of("self"), recv_mode_of(receiver).is_some());
    env.extend(params_env(params));
    callable_return_has_responsibility(ctx, body, env)
}

pub fn procedure_return_has_responsibility(ctx: &ScopeContext<'_>, decl: &ast::ProcedureDecl) -> Option<bool> {
    callable_return_has_responsibility(ctx, &decl.body, params_env(&decl.params))
}

fn resolve_named_procedure<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr) -> Option<&'c ast::ProcedureDecl> {
    let (module_path, name) = match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => (&ctx.current_module, &node.name),
        ExprNode::QualifiedNameExpr(node) => (if node.path.is_empty() { &ctx.current_module } else { &node.path }, &node.name),
        ExprNode::PathExpr(node) => (if node.path.is_empty() { &ctx.current_module } else { &node.path }, &node.name),
        _ => return None,
    };
    let module = ctx.sigma.mods.iter().find(|module| module.path == *module_path)?;
    module.items.iter().find_map(|item| match item {
        ast::ASTItem::ProcedureDecl(proc) if id_eq(&proc.name, name) => Some(proc),
        _ => None,
    })
}

fn method_call_from_receiver_type_has_responsibility(ctx: &ScopeContext<'_>, receiver_type: &TypeRef, name: &str) -> Option<bool> {
    let base = strip_perm(receiver_type);
    if let TypeNode::ModalState(modal) = &base.as_deref()?.node {
        let Some(TypeDecl::Modal(modal_decl)) = ctx.sigma.types.get(&path_key_of(&modal.path)) else {
            return None;
        };
        let method = lookup_state_method_decl(modal_decl, &modal.state, name)?;
        return method_body_return_has_responsibility(ctx, &method.receiver, &method.params, &method.body);
    }
    let lookup = lookup_method_static(ctx, &base, name).ok()?;
    if let Some(method) = lookup.record_method {
        return method_body_return_has_responsibility(ctx, &method.receiver, &method.params, &method.body);
    }
    let method = lookup.class_method?;
    method_body_return_has_responsibility(ctx, &method.receiver, &method.params, &method.body_opt)
}

fn expr_result_has_responsibility_with_env(ctx: &ScopeContext<'_>, expr: &ExprPtr, env: &ResponsibilityEnv) -> Option<bool> {
    let Some(e) = expr.as_deref() else {
        return Some(false);
    };
    match &e.node {
        ExprNode::AttributedExpr(node) => return expr_result_has_responsibility_with_env(ctx, &node.expr, env),
        ExprNode::MoveExpr(_) | ExprNode::CopyExpr(_) => return Some(true),
        _ => {}
    }
    if let Some(root) = place_root(expr) {
        return Some(env.get(&root).copied().unwrap_or(true));
    }
    Some(match &e.node {
        ExprNode::CallExpr(call) => call_result_has_responsibility(ctx, call).unwrap_or(true),
        ExprNode::MethodCallExpr(method) => method_call_result_has_responsibility(ctx, method).unwrap_or(true),
        ExprNode::IfExpr(node) => {
            if node.then_expr.is_none() || node.else_expr.is_none() {
                return Some(true);
            }
            let then_resp = expr_result_has_responsibility_with_env(ctx, &node.then_expr, env).unwrap_or(true);
            let else_resp = expr_result_has_responsibility_with_env(ctx, &node.else_expr, env).unwrap_or(true);
            then_resp || else_resp
        }
        ExprNode::BlockExpr(node) => analyze_block_returns(ctx, &node.block, env.clone()).unwrap_or(true),
        ExprNode::UnsafeBlockExpr(node) => analyze_block_returns(ctx, &node.block, env.clone()).unwrap_or(true),
        ExprNode::PropagateExpr(node) => return expr_result_has_responsibility_with_env(ctx, &node.value, env),
        _ => true,
    })
}

/// None when the callee is not a procedure whose body can be read.
pub fn call_result_has_responsibility(ctx: &ScopeContext<'_>, call: &ast::CallExpr) -> Option<bool> {
    let selected = selected_call_target(ctx, call).and_then(|selected| {
        let key = path_key_of(&selected.module_path);
        let module = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key)?;
        module.items.iter().find_map(|item| match item {
            ast::ASTItem::ProcedureDecl(proc) if proc.name == selected.proc_name && proc.span == selected.proc_span => Some(proc),
            _ => None,
        })
    });
    let proc = selected.or_else(|| resolve_named_procedure(ctx, &call.callee))?;
    procedure_return_has_responsibility(ctx, proc)
}

pub fn method_call_result_has_responsibility(ctx: &ScopeContext<'_>, call: &ast::MethodCallExpr) -> Option<bool> {
    let receiver_type = stored_expr_type(ctx, &call.receiver)?;
    method_call_from_receiver_type_has_responsibility(ctx, &receiver_type, &call.name)
}
