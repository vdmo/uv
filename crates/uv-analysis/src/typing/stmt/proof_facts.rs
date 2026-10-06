//! The facts a statement leaves for the statements after it: an assignment forgets
//! what was known about the name it writes, a binding learns what it was bound to and
//! what a called procedure promises of its result, and an `if` that leaves when its
//! condition holds leaves the condition false behind it.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};
use uv_source::attributes::{resolve_verification_mode_attribute, VerificationModeAttribute};
use uv_source::lexer::token::{Token, TokenKind};

use crate::typing::expr_store::selected_call_target;
use crate::context::{IdKey, ScopeContext};
use crate::contracts::purity::is_pure_in_scope;
use crate::contracts::verification::{extend_proof_context_with_predicate_at, negated_predicate, StaticProofContext};
use crate::resolve::scopes::{id_key_of, path_key_of};
use crate::resolve::scopes_lookup::resolve_value_name;
use crate::typing::type_env::{mut_of, TypeEnv};

pub type ProofCtx = Option<Rc<StaticProofContext>>;

fn make_proof_expr(span: &Span, node: ExprNode) -> ExprPtr {
    Some(Arc::new(ast::Expr { span: span.clone(), node }))
}

fn strip_attributed_expr(expr: &ExprPtr) -> &ExprPtr {
    let mut current = expr;
    while let Some(ExprNode::AttributedExpr(attributed)) = current.as_deref().map(|expr| &expr.node) {
        current = &attributed.expr;
    }
    current
}

fn extend(base: &ProofCtx, predicate: &ExprPtr, location: &Span) -> ProofCtx {
    extend_proof_context_with_predicate_at(base.as_deref(), predicate, location).map(Rc::new)
}

fn identifier_fact_expr(name: &str, span: &Span) -> ExprPtr {
    make_proof_expr(span, ExprNode::IdentifierExpr(ast::IdentifierExpr { name: name.to_string(), ..Default::default() }))
}

/// The name a binding pattern binds, when it binds exactly one.
fn simple_binding_name(pat: &ast::PatternPtr) -> Option<&str> {
    match &pat.as_deref()?.node {
        ast::PatternNode::IdentifierPattern(ident) => Some(&ident.name),
        ast::PatternNode::TypedPattern(typed) => Some(&typed.name),
        _ => None,
    }
}

fn binary(span: &Span, op: &str, lhs: ExprPtr, rhs: ExprPtr) -> ExprPtr {
    make_proof_expr(span, ExprNode::BinaryExpr(ast::BinaryExpr { op: op.to_string(), lhs, rhs }))
}

fn binding_has_unsigned_annotation(binding: &ast::Binding) -> bool {
    let annotation = ast::binding_annotation_type_opt(binding);
    matches!(
        annotation.as_deref().map(|ty| &ty.node),
        Some(ast::TypeNode::TypePrim(prim)) if matches!(prim.name.as_str(), "usize" | "u8" | "u16" | "u32" | "u64")
    )
}

/// The names a fact speaks of.
fn collect_proof_expr_names(expr: &ExprPtr, out: &mut HashSet<IdKey>) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    match &e.node {
        ExprNode::IdentifierExpr(node) => {
            out.insert(id_key_of(&node.name));
        }
        ExprNode::PathExpr(node) if node.path.is_empty() => {
            out.insert(id_key_of(&node.name));
        }
        ExprNode::BinaryExpr(node) => {
            collect_proof_expr_names(&node.lhs, out);
            collect_proof_expr_names(&node.rhs, out);
        }
        ExprNode::UnaryExpr(node) => collect_proof_expr_names(&node.value, out),
        ExprNode::FieldAccessExpr(node) => collect_proof_expr_names(&node.base, out),
        ExprNode::TupleAccessExpr(node) => collect_proof_expr_names(&node.base, out),
        ExprNode::IndexAccessExpr(node) => {
            collect_proof_expr_names(&node.base, out);
            collect_proof_expr_names(&node.index, out);
        }
        ExprNode::CastExpr(node) => collect_proof_expr_names(&node.value, out),
        ExprNode::AttributedExpr(node) => collect_proof_expr_names(&node.expr, out),
        _ => {}
    }
}

fn mutated_root_name(place: &ExprPtr) -> Option<IdKey> {
    match &place.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(id_key_of(&node.name)),
        ExprNode::FieldAccessExpr(node) => mutated_root_name(&node.base),
        ExprNode::TupleAccessExpr(node) => mutated_root_name(&node.base),
        ExprNode::IndexAccessExpr(node) => mutated_root_name(&node.base),
        ExprNode::AttributedExpr(node) => mutated_root_name(&node.expr),
        _ => None,
    }
}

/// An assignment drops every fact that mentions the name it writes.
fn invalidate_assigned_proof_facts(current: &ProofCtx, stmt: &Stmt) -> ProofCtx {
    let mutated = match stmt {
        Stmt::AssignStmt(node) => mutated_root_name(&node.place),
        Stmt::CompoundAssignStmt(node) => mutated_root_name(&node.place),
        _ => None,
    };
    let (Some(ctx), Some(mutated)) = (current, mutated) else {
        return current.clone();
    };
    let mut proof_ctx = (**ctx).clone();
    proof_ctx.facts.retain(|fact| {
        let mut fact_names = HashSet::new();
        collect_proof_expr_names(&fact.predicate, &mut fact_names);
        !fact_names.contains(&mutated)
    });
    Some(Rc::new(proof_ctx))
}

/// A call whose callee is named directly, perhaps inside a bare `unsafe { … }`.
fn direct_call_fact_view(expr: &ExprPtr) -> Option<(&ExprPtr, &[ast::Arg], Option<&ast::CallExpr>)> {
    let mut stripped = strip_attributed_expr(expr);
    if let Some(ExprNode::UnsafeBlockExpr(unsafe_block)) = stripped.as_deref().map(|expr| &expr.node) {
        if let Some(block) = unsafe_block.block.as_deref().filter(|block| block.stmts.is_empty() && block.tail_opt.is_some()) {
            stripped = strip_attributed_expr(&block.tail_opt);
        }
    }
    match &stripped.as_deref()?.node {
        ExprNode::CallExpr(call) => Some((&call.callee, &call.args, Some(call))),
        ExprNode::CallTypeArgsExpr(call) => Some((&call.callee, &call.args, None)),
        _ => None,
    }
}

fn static_callee_module<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr) -> Option<(&'c ast::ASTModule, String)> {
    let (path, name) = match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => match resolve_value_name(ctx, &ident.name) {
            Some(entity) if entity.origin_opt.is_some() => {
                (entity.origin_opt.unwrap_or_default(), entity.target_opt.unwrap_or_else(|| ident.name.clone()))
            }
            _ => (ctx.current_module.clone(), ident.name.clone()),
        },
        ExprNode::QualifiedNameExpr(qualified) => (qualified.path.clone(), qualified.name.clone()),
        ExprNode::PathExpr(path) => (if path.path.is_empty() { ctx.current_module.clone() } else { path.path.clone() }, path.name.clone()),
        _ => return None,
    };
    let key = path_key_of(&path);
    let module = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key)?;
    Some((module, name))
}

/// The procedure a direct call names: the overload typing selected, when it had to pick.
fn static_callee_procedure<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr, call_expr: Option<&ast::CallExpr>) -> Option<&'c ast::ProcedureDecl> {
    if let Some(selected) = call_expr.and_then(|call| selected_call_target(ctx, call)) {
        let key = path_key_of(&selected.module_path);
        let module = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key)?;
        return module.items.iter().find_map(|item| match item {
            ast::ASTItem::ProcedureDecl(proc) if proc.name == selected.proc_name && proc.span == selected.proc_span => Some(proc),
            _ => None,
        });
    }
    let (module, name) = static_callee_module(ctx, callee)?;
    let key = id_key_of(&name);
    module.items.iter().find_map(|item| match item {
        ast::ASTItem::ProcedureDecl(proc) if id_key_of(&proc.name) == key => Some(proc),
        _ => None,
    })
}

fn static_callee_extern_procedure<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr) -> Option<&'c ast::ExternProcDecl> {
    let (module, name) = static_callee_module(ctx, callee)?;
    let key = id_key_of(&name);
    module
        .items
        .iter()
        .filter_map(|item| match item {
            ast::ASTItem::ExternBlock(block) => Some(block),
            _ => None,
        })
        .flat_map(|block| &block.items)
        .map(|ast::ExternItem::ExternProcDecl(proc)| proc)
        .find(|proc| id_key_of(&proc.name) == key)
}

/// An expression whose value cannot change after the call: literals and immutable
/// bindings, and fields, elements and arithmetic of them.
fn expr_stable_for_call_postcondition(expr: &ExprPtr, env: &TypeEnv) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    match &e.node {
        ExprNode::LiteralExpr(_) | ExprNode::PtrNullExpr(_) => true,
        ExprNode::IdentifierExpr(node) => mut_of(env, &node.name) == Some(ast::Mutability::Let),
        ExprNode::FieldAccessExpr(node) => expr_stable_for_call_postcondition(&node.base, env),
        ExprNode::TupleAccessExpr(node) => expr_stable_for_call_postcondition(&node.base, env),
        ExprNode::IndexAccessExpr(node) => {
            expr_stable_for_call_postcondition(&node.base, env) && expr_stable_for_call_postcondition(&node.index, env)
        }
        ExprNode::UnaryExpr(node) => expr_stable_for_call_postcondition(&node.value, env),
        ExprNode::BinaryExpr(node) => {
            expr_stable_for_call_postcondition(&node.lhs, env) && expr_stable_for_call_postcondition(&node.rhs, env)
        }
        _ => false,
    }
}

/// A callee's postcondition with the arguments for its parameters and the binding for
/// `@result`. A form that cannot be carried over blocks the whole predicate.
fn substitute_call_postcondition_expr(
    expr: &ExprPtr,
    substitutions: &HashMap<String, ExprPtr>,
    result_expr: &ExprPtr,
    env: &TypeEnv,
    blocked: &mut bool,
) -> ExprPtr {
    let Some(e) = expr.as_deref().filter(|_| !*blocked) else {
        return expr.clone();
    };
    let sub = |inner: &ExprPtr, blocked: &mut bool| substitute_call_postcondition_expr(inner, substitutions, result_expr, env, blocked);
    let node = match &e.node {
        ExprNode::ResultExpr(_) => {
            if result_expr.is_none() {
                *blocked = true;
                return expr.clone();
            }
            return result_expr.clone();
        }
        ExprNode::LiteralExpr(_) | ExprNode::PtrNullExpr(_) => return expr.clone(),
        ExprNode::IdentifierExpr(node) => return substitutions.get(&node.name).cloned().unwrap_or_else(|| expr.clone()),
        ExprNode::EntryExpr(node) => {
            let inner = sub(&node.expr, blocked);
            if *blocked || !expr_stable_for_call_postcondition(&inner, env) {
                *blocked = true;
                return expr.clone();
            }
            return inner;
        }
        ExprNode::BinaryExpr(node) => {
            let lhs = sub(&node.lhs, blocked);
            let rhs = sub(&node.rhs, blocked);
            ExprNode::BinaryExpr(ast::BinaryExpr { lhs, rhs, ..node.clone() })
        }
        ExprNode::UnaryExpr(node) => ExprNode::UnaryExpr(ast::UnaryExpr { value: sub(&node.value, blocked), ..node.clone() }),
        ExprNode::FieldAccessExpr(node) => ExprNode::FieldAccessExpr(ast::FieldAccessExpr { base: sub(&node.base, blocked), ..node.clone() }),
        ExprNode::TupleAccessExpr(node) => ExprNode::TupleAccessExpr(ast::TupleAccessExpr { base: sub(&node.base, blocked), ..node.clone() }),
        ExprNode::IndexAccessExpr(node) => {
            let base = sub(&node.base, blocked);
            let index = sub(&node.index, blocked);
            ExprNode::IndexAccessExpr(ast::IndexAccessExpr { base, index })
        }
        ExprNode::TupleExpr(node) => {
            let mut out = node.clone();
            for element in &mut out.elements {
                *element = sub(element, blocked);
            }
            ExprNode::TupleExpr(out)
        }
        ExprNode::RecordExpr(node) => {
            let mut out = node.clone();
            for field in &mut out.fields {
                field.value = sub(&field.value, blocked);
            }
            ExprNode::RecordExpr(out)
        }
        ExprNode::IfExpr(node) => {
            let mut out = node.clone();
            out.cond = sub(&node.cond, blocked);
            out.then_expr = sub(&node.then_expr, blocked);
            out.else_expr = sub(&node.else_expr, blocked);
            ExprNode::IfExpr(out)
        }
        ExprNode::IfIsExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee, blocked);
            out.then_expr = sub(&node.then_expr, blocked);
            out.else_expr = sub(&node.else_expr, blocked);
            ExprNode::IfIsExpr(out)
        }
        ExprNode::IfCaseExpr(node) => {
            let mut out = node.clone();
            out.scrutinee = sub(&node.scrutinee, blocked);
            for arm in &mut out.cases {
                arm.body = sub(&arm.body, blocked);
            }
            out.else_expr = sub(&node.else_expr, blocked);
            ExprNode::IfCaseExpr(out)
        }
        ExprNode::CallExpr(node) => {
            let mut out = node.clone();
            out.callee = sub(&node.callee, blocked);
            for arg in &mut out.args {
                arg.value = sub(&arg.value, blocked);
            }
            ExprNode::CallExpr(out)
        }
        ExprNode::CallTypeArgsExpr(node) => {
            let mut out = node.clone();
            out.callee = sub(&node.callee, blocked);
            for arg in &mut out.args {
                arg.value = sub(&arg.value, blocked);
            }
            ExprNode::CallTypeArgsExpr(out)
        }
        ExprNode::MethodCallExpr(node) => {
            let mut out = node.clone();
            out.receiver = sub(&node.receiver, blocked);
            for arg in &mut out.args {
                arg.value = sub(&arg.value, blocked);
            }
            ExprNode::MethodCallExpr(out)
        }
        ExprNode::CastExpr(node) => ExprNode::CastExpr(ast::CastExpr { value: sub(&node.value, blocked), ..node.clone() }),
        _ => {
            *blocked = true;
            return expr.clone();
        }
    };
    make_proof_expr(&e.span, node)
}

/// The arguments by parameter name, when every argument is stable.
fn call_substitutions(params: &[ast::Param], args: &[ast::Arg], env: &TypeEnv) -> Option<HashMap<String, ExprPtr>> {
    if args.len() != params.len() {
        return None;
    }
    let mut substitutions = HashMap::new();
    for (param, arg) in params.iter().zip(args) {
        if !expr_stable_for_call_postcondition(&arg.value, env) {
            return None;
        }
        substitutions.entry(param.name.clone()).or_insert_with(|| arg.value.clone());
    }
    Some(substitutions)
}

/// What the called procedure's postcondition says of the bound result.
fn call_postcondition_proof_context_for_let(
    ctx: &ScopeContext<'_>,
    env: &TypeEnv,
    current: &ProofCtx,
    binding: &ast::Binding,
    span: &Span,
    binding_name: &str,
) -> ProofCtx {
    let Some((callee, args, call_expr)) = direct_call_fact_view(&binding.init) else {
        return current.clone();
    };
    let Some(proc) = static_callee_procedure(ctx, callee, call_expr) else {
        return current.clone();
    };
    let Some(postcondition) = proc.contract.as_ref().map(|contract| &contract.postcondition).filter(|post| post.is_some()) else {
        return current.clone();
    };
    let Some(substitutions) = call_substitutions(&proc.params, args, env) else {
        return current.clone();
    };
    let mut blocked = false;
    let fact = substitute_call_postcondition_expr(postcondition, &substitutions, &identifier_fact_expr(binding_name, span), env, &mut blocked);
    if blocked {
        return current.clone();
    }
    extend(current, &fact, span)
}

/// What a foreign procedure's `ensures` clauses say of the bound result, unless it
/// also declares how it signals errors.
fn foreign_postcondition_proof_context_for_let(
    ctx: &ScopeContext<'_>,
    env: &TypeEnv,
    current: &ProofCtx,
    binding: &ast::Binding,
    span: &Span,
    binding_name: &str,
) -> ProofCtx {
    let Some((callee, args, _)) = direct_call_fact_view(&binding.init) else {
        return current.clone();
    };
    let Some(proc) = static_callee_extern_procedure(ctx, callee) else {
        return current.clone();
    };
    let Some(clauses) = &proc.foreign_contracts_opt else {
        return current.clone();
    };
    if resolve_verification_mode_attribute(&proc.attrs) == Some(VerificationModeAttribute::Dynamic) {
        return current.clone();
    }
    let Some(substitutions) = call_substitutions(&proc.params, args, env) else {
        return current.clone();
    };
    if clauses.iter().any(|clause| clause.kind == ast::ForeignContractKind::EnsuresError && !clause.predicates.is_empty()) {
        return current.clone();
    }
    let mut proof_ctx = current.clone();
    let result_expr = identifier_fact_expr(binding_name, span);
    for clause in clauses.iter().filter(|clause| clause.kind == ast::ForeignContractKind::Ensures) {
        for predicate in &clause.predicates {
            let mut blocked = false;
            let fact = substitute_call_postcondition_expr(predicate, &substitutions, &result_expr, env, &mut blocked);
            if !blocked {
                proof_ctx = extend(&proof_ctx, &fact, span);
            }
        }
    }
    proof_ctx
}

/// A binding to a pure expression is known to equal it, and one annotated with an
/// unsigned type to be non-negative.
fn let_binding_proof_context_for_stmt(ctx: &ScopeContext<'_>, env: &TypeEnv, current: &ProofCtx, stmt: &Stmt) -> ProofCtx {
    let (binding, span, is_let) = match stmt {
        Stmt::LetStmt(node) => (&node.binding, &node.span, true),
        Stmt::VarStmt(node) => (&node.binding, &node.span, false),
        _ => return current.clone(),
    };
    let Some(init) = binding.init.as_deref() else {
        return current.clone();
    };
    let Some(binding_name) = simple_binding_name(&binding.pat) else {
        return current.clone();
    };
    let mut proof_ctx = call_postcondition_proof_context_for_let(ctx, env, current, binding, span, binding_name);
    if is_let {
        proof_ctx = foreign_postcondition_proof_context_for_let(ctx, env, &proof_ctx, binding, span, binding_name);
    }
    if !is_pure_in_scope(ctx, &binding.init) {
        return proof_ctx;
    }
    let fact_span = &init.span;
    let equality = binary(fact_span, "==", identifier_fact_expr(binding_name, fact_span), binding.init.clone());
    proof_ctx = extend(&proof_ctx, &equality, fact_span);
    if binding_has_unsigned_annotation(binding) {
        let zero = make_proof_expr(
            fact_span,
            ExprNode::LiteralExpr(ast::LiteralExpr { literal: Token { kind: TokenKind::IntLiteral, lexeme: "0".to_string(), span: fact_span.clone() } }),
        );
        let non_negative = binary(fact_span, "<=", zero, identifier_fact_expr(binding_name, fact_span));
        proof_ctx = extend(&proof_ctx, &non_negative, fact_span);
    }
    proof_ctx
}

fn has_non_local_ctrl_block(block: &ast::BlockPtr, in_loop: bool) -> bool {
    block.as_deref().is_some_and(|block| {
        block.stmts.iter().any(|stmt| has_non_local_ctrl_stmt(stmt, in_loop)) || has_non_local_ctrl_expr(&block.tail_opt, in_loop)
    })
}

fn has_non_local_ctrl_stmt(stmt: &Stmt, in_loop: bool) -> bool {
    match stmt {
        Stmt::ReturnStmt(_) => true,
        Stmt::BreakStmt(_) | Stmt::ContinueStmt(_) => !in_loop,
        Stmt::LetStmt(node) => has_non_local_ctrl_expr(&node.binding.init, in_loop),
        Stmt::VarStmt(node) => has_non_local_ctrl_expr(&node.binding.init, in_loop),
        Stmt::AssignStmt(node) => has_non_local_ctrl_expr(&node.place, in_loop) || has_non_local_ctrl_expr(&node.value, in_loop),
        Stmt::CompoundAssignStmt(node) => has_non_local_ctrl_expr(&node.place, in_loop) || has_non_local_ctrl_expr(&node.value, in_loop),
        Stmt::ExprStmt(node) => has_non_local_ctrl_expr(&node.value, in_loop),
        Stmt::DeferStmt(node) => has_non_local_ctrl_block(&node.body, in_loop),
        Stmt::RegionStmt(node) => has_non_local_ctrl_expr(&node.opts_opt, in_loop) || has_non_local_ctrl_block(&node.body, in_loop),
        Stmt::FrameStmt(node) => has_non_local_ctrl_block(&node.body, in_loop),
        Stmt::UnsafeBlockStmt(node) => has_non_local_ctrl_block(&node.body, in_loop),
        Stmt::KeyBlockStmt(node) => has_non_local_ctrl_block(&node.body, in_loop),
        _ => false,
    }
}

/// Whether control may leave the expression other than by finishing it: a `return`,
/// or a `break` or `continue` of a loop outside it.
pub fn has_non_local_ctrl_expr(expr: &ExprPtr, in_loop: bool) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    let any = |exprs: &[&ExprPtr]| exprs.iter().any(|inner| has_non_local_ctrl_expr(inner, in_loop));
    match &e.node {
        ExprNode::LoopInfiniteExpr(node) => has_non_local_ctrl_block(&node.body, true),
        ExprNode::LoopConditionalExpr(node) => any(&[&node.cond]) || has_non_local_ctrl_block(&node.body, true),
        ExprNode::LoopIterExpr(node) => any(&[&node.iter]) || has_non_local_ctrl_block(&node.body, true),
        ExprNode::BlockExpr(node) => has_non_local_ctrl_block(&node.block, in_loop),
        ExprNode::UnsafeBlockExpr(node) => has_non_local_ctrl_block(&node.block, in_loop),
        ExprNode::IfExpr(node) => any(&[&node.cond, &node.then_expr, &node.else_expr]),
        ExprNode::IfCaseExpr(node) => {
            any(&[&node.scrutinee]) || node.cases.iter().any(|case_clause| any(&[&case_clause.body])) || any(&[&node.else_expr])
        }
        ExprNode::IfIsExpr(node) => any(&[&node.scrutinee, &node.then_expr, &node.else_expr]),
        ExprNode::BinaryExpr(node) => any(&[&node.lhs, &node.rhs]),
        ExprNode::UnaryExpr(node) => any(&[&node.value]),
        ExprNode::CallExpr(node) => any(&[&node.callee]) || node.args.iter().any(|arg| any(&[&arg.value])),
        ExprNode::MethodCallExpr(node) => any(&[&node.receiver]) || node.args.iter().any(|arg| any(&[&arg.value])),
        _ => false,
    }
}

/// The facts known after a statement, given those known before it.
pub fn fallthrough_proof_context_for_stmt(ctx: &ScopeContext<'_>, env: &TypeEnv, current: &ProofCtx, stmt: &Stmt) -> ProofCtx {
    let invalidated = invalidate_assigned_proof_facts(current, stmt);
    let with_binding_fact = let_binding_proof_context_for_stmt(ctx, env, &invalidated, stmt);
    let Stmt::ExprStmt(expr_stmt) = stmt else {
        return with_binding_fact;
    };
    // `if cond { return … }` leaves `cond` false behind it.
    let Some(ExprNode::IfExpr(if_expr)) = strip_attributed_expr(&expr_stmt.value).as_deref().map(|expr| &expr.node) else {
        return with_binding_fact;
    };
    if if_expr.else_expr.is_some() || if_expr.then_expr.is_none() || !has_non_local_ctrl_expr(&if_expr.then_expr, false) {
        return with_binding_fact;
    }
    if !is_pure_in_scope(ctx, &if_expr.cond) {
        return with_binding_fact;
    }
    match negated_predicate(&if_expr.cond) {
        Some(negated) => {
            let location = negated.as_deref().map(|expr| expr.span.clone()).unwrap_or_default();
            extend(&with_binding_fact, &negated, &location)
        }
        None => with_binding_fact,
    }
}
