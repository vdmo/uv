//! `parallel` blocks and the tasks they `spawn`.

use std::cell::RefCell;
use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::caps::builtin_paths::{is_context_type_path, is_execution_domain_class_path, path_matches_builtin_name};
use crate::caps::cap_methods::lookup_context_method_sig;
use crate::context::{IdKey, ScopeContext};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::callbacks::ExprTypeFn;
use super::dispatch_keys::{access_of_key_clause, dispatch_pattern_name_set, dynamic_key_pattern, infer_dispatch_accesses};
use crate::typing::closure_capture::block_captures;
use crate::typing::const_len::const_len;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt::block::{type_block, type_block_info};
use crate::typing::stmt_context::{ParallelCaptureScope, StmtTypeContext};
use crate::typing::type_env::{bind_of, collect_pat_names, gpu_context, BindingProvenanceSeedKind, ParallelContextKind, TypeEnv};
use crate::typing::type_expr::{type_expr, type_identifier_expr, type_place};
use crate::typing::type_predicates::{gpu_safe_diag_for_type, perm_of_type, strip_perm, strip_perm_and_refine};
use crate::typing::types::*;

fn failed(diag_id: &'static str) -> ExprTypeResult {
    ExprTypeResult::failed(Some(diag_id))
}

fn strip_attributed(expr: &ExprPtr) -> &ExprPtr {
    let mut current = expr;
    while let Some(ExprNode::AttributedExpr(attributed)) = current.as_deref().map(|expr| &expr.node) {
        current = &attributed.expr;
    }
    current
}

fn is_execution_domain_type(ty: &TypeRef) -> bool {
    match strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Dynamic(path)) | Some(TypeNode::Path { path, .. }) => is_execution_domain_class_path(path),
        _ => false,
    }
}

/// A path type whose last segment is the name; `exact` compares the bytes.
fn is_named_type_path(ty: &TypeRef, name: &str, deep_strip: bool, exact: bool) -> bool {
    let stripped = if deep_strip { strip_perm_and_refine(ty) } else { strip_perm(ty) };
    match stripped.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) => path.last().is_some_and(|last| if exact { last == name } else { id_eq(last, name) }),
        _ => false,
    }
}

/// Which kind of domain the expression names: `ctx~>cpu()` and the like, or a binding
/// that holds one.
fn parallel_context_kind_of(expr: &ExprPtr, env: &TypeEnv) -> Option<ParallelContextKind> {
    match &strip_attributed(expr).as_deref()?.node {
        ExprNode::MethodCallExpr(method) => match method.name.as_str() {
            "cpu" => Some(ParallelContextKind::Cpu),
            "gpu" => Some(ParallelContextKind::Gpu),
            "inline" => Some(ParallelContextKind::Inline),
            _ => None,
        },
        ExprNode::IdentifierExpr(ident) => bind_of(env, &ident.name)?.parallel_context_kind,
        _ => None,
    }
}

/// When the domain fails to type because a context's domain constructor was called
/// with the wrong arguments, that is the error to report.
fn parallel_domain_param_diag(domain_expr: &ExprPtr, type_expr_fn: ExprTypeFn<'_>) -> Option<&'static str> {
    let ExprNode::MethodCallExpr(call) = &strip_attributed(domain_expr).as_deref()?.node else {
        return None;
    };
    call.receiver.as_ref()?;
    let receiver_type = type_expr_fn(&call.receiver);
    if !receiver_type.ok {
        return None;
    }
    match strip_perm_and_refine(&receiver_type.r#type).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) if is_context_type_path(path) => {}
        _ => return None,
    }
    if lookup_context_method_sig(&call.name, Some(call.args.len())).is_none() {
        return Some("E-CON-0103");
    }
    for (index, arg) in call.args.iter().enumerate() {
        if arg.value.is_none() {
            return Some("E-CON-0103");
        }
        let arg_type = type_expr_fn(&arg.value);
        if !arg_type.ok {
            return arg_type.diag_id;
        }
        if id_eq(&call.name, "cpu") {
            let expected = ["CpuSet", "Priority"].get(index);
            if expected.is_some_and(|name| !is_named_type_path(&arg_type.r#type, name, true, true)) {
                return Some("E-CON-0103");
            }
        }
    }
    None
}

fn is_cancel_token_type(ty: &TypeRef) -> bool {
    match strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) => path_matches_builtin_name(path, "CancelToken"),
        Some(TypeNode::ModalState(modal)) => path_matches_builtin_name(&modal.path, "CancelToken") && modal.state == "Active",
        _ => false,
    }
}

/// Three constant, non-zero `usize` dimensions.
pub(crate) fn extract_dim3_const(ctx: &ScopeContext<'_>, expr: &ExprPtr, type_expr_fn: ExprTypeFn<'_>) -> Option<[u64; 3]> {
    let ExprNode::TupleExpr(tuple) = &expr.as_deref()?.node else {
        return None;
    };
    let [x, y, z] = &tuple.elements[..] else {
        return None;
    };
    let dim = |element: &ExprPtr| -> Option<u64> {
        let elem_type = type_expr_fn(element);
        let is_usize = matches!(strip_perm_and_refine(&elem_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize");
        if !elem_type.ok || !is_usize {
            return None;
        }
        const_len(ctx, element).ok().filter(|value| *value != 0)
    };
    Some([dim(x)?, dim(y)?, dim(z)?])
}

/// A workgroup may hold at most 1024 invocations.
pub(crate) fn exceeds_max_workgroup_size([x, y, z]: [u64; 3]) -> bool {
    const MAX_WORKGROUP_SIZE: u64 = 1024;
    if x == 0 || y == 0 || z == 0 || x > MAX_WORKGROUP_SIZE {
        return true;
    }
    let xy_limit = MAX_WORKGROUP_SIZE / x;
    y > xy_limit || z > xy_limit / y
}

fn dispatch_has_reduce(dispatch: &ast::DispatchExpr) -> bool {
    dispatch.opts.iter().any(|opt| opt.kind == ast::DispatchOptionKind::Reduce)
}

/// A `spawn`, or a `dispatch` that reduces: the forms whose values a `parallel` block
/// collects as its result.
fn is_collectable_parallel_expr(expr: &ExprPtr) -> bool {
    match strip_attributed(expr).as_deref().map(|expr| &expr.node) {
        Some(ExprNode::SpawnExpr(_)) => true,
        Some(ExprNode::DispatchExpr(dispatch)) => dispatch_has_reduce(dispatch),
        _ => false,
    }
}

/// The value inside `Spawned<T>`, as a path or a modal state.
fn extract_spawned_inner(ty: &TypeRef) -> Option<TypeRef> {
    match &ty.as_deref()?.node {
        TypeNode::Path { path, generic_args } if path_matches_builtin_name(path, "Spawned") => generic_args.first().cloned(),
        TypeNode::ModalState(modal) if path_matches_builtin_name(&modal.path, "Spawned") => modal.generic_args.first().cloned(),
        _ => None,
    }
}

fn collect_parallel_result_expr(type_expr_fn: ExprTypeFn<'_>, expr: &ExprPtr, types: &mut Vec<TypeRef>) -> Result<(), Option<&'static str>> {
    let stripped = strip_attributed(expr);
    let typed = match stripped.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::SpawnExpr(_)) => type_expr_fn(stripped),
        Some(ExprNode::DispatchExpr(dispatch)) if dispatch_has_reduce(dispatch) => type_expr_fn(stripped),
        _ => return Ok(()),
    };
    if !typed.ok {
        return Err(typed.diag_id);
    }
    let is_spawn = matches!(stripped.as_deref().map(|expr| &expr.node), Some(ExprNode::SpawnExpr(_)));
    let collected = is_spawn.then(|| extract_spawned_inner(&strip_perm_and_refine(&typed.r#type))).flatten().unwrap_or(typed.r#type);
    types.push(collected);
    Ok(())
}

pub(crate) fn emit_supplemental_type_diag(type_ctx: &StmtTypeContext<'_>, code: &str) {
    if let (Some(diags), Some(diag)) = (&type_ctx.diags, make_diagnostic_by_id(code, None)) {
        emit(&mut diags.borrow_mut(), diag);
    }
}

/// What may be captured into GPU code: nothing shared, nothing on the heap, and only
/// types a GPU can hold.
pub(crate) fn check_gpu_capture(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, env: &TypeEnv, name: &str) -> Option<&'static str> {
    let binding = bind_of(env, name)?;
    if perm_of_type(&binding.r#type) == Permission::Shared {
        return Some("E-CON-0151");
    }
    if binding.provenance_kind == BindingProvenanceSeedKind::Heap {
        return Some("E-CON-0150");
    }
    if let Some(gpu_diag) = gpu_safe_diag_for_type(ctx, &binding.r#type) {
        emit_supplemental_type_diag(type_ctx, gpu_diag);
        return Some("E-CON-0153");
    }
    None
}

/// Records what a statement binds directly in the innermost `parallel` block, so that
/// tasks know which bindings they share with their siblings.
pub fn record_parallel_stmt_bindings(type_ctx: &StmtTypeContext<'_>, stmt: &Stmt) {
    let Some(scope) = type_ctx.parallel_capture_scopes.as_ref().and_then(|scopes| scopes.last()) else {
        return;
    };
    let pattern = match stmt {
        Stmt::LetStmt(node) => &node.binding.pat,
        Stmt::VarStmt(node) => &node.binding.pat,
        Stmt::UsingLocalStmt(node) => {
            scope.bindings.borrow_mut().insert(id_key_of(&node.alias));
            return;
        }
        _ => return,
    };
    if let Some(pattern) = pattern.as_deref() {
        let mut names: Vec<IdKey> = Vec::new();
        collect_pat_names(pattern, &mut names);
        scope.bindings.borrow_mut().extend(names);
    }
}

/// `parallel domain [opts] { … }`: the body runs on the domain; its value is its tail,
/// or else the values of the tasks it spawns.
pub fn type_parallel_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::ParallelExpr,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> ExprTypeResult {
    let Some(body) = expr.body.as_deref().filter(|_| expr.domain.is_some()) else {
        return ExprTypeResult::default();
    };
    let domain_type = type_expr_fn(&expr.domain);
    if !domain_type.ok {
        return ExprTypeResult::failed(parallel_domain_param_diag(&expr.domain, type_expr_fn).or(domain_type.diag_id));
    }
    if !is_execution_domain_type(&domain_type.r#type) {
        return failed("E-CON-0102");
    }
    let domain_kind = parallel_context_kind_of(&expr.domain, env);
    let gpu_domain = domain_kind == Some(ParallelContextKind::Gpu);
    if gpu_context(env) && gpu_domain {
        return failed("E-CON-0152");
    }
    for opt in &expr.opts {
        if opt.kind == ast::ParallelOptionKind::Name || opt.value.is_none() {
            continue;
        }
        let opt_type = type_expr_fn(&opt.value);
        if !opt_type.ok {
            return ExprTypeResult::failed(opt_type.diag_id);
        }
        match opt.kind {
            ast::ParallelOptionKind::Cancel => {
                if !is_cancel_token_type(&opt_type.r#type) {
                    return failed("E-CON-0103");
                }
            }
            ast::ParallelOptionKind::Workgroup | ast::ParallelOptionKind::Workgroups => {
                let Some(dims) = extract_dim3_const(ctx, &opt.value, type_expr_fn) else {
                    return failed("E-CON-0159");
                };
                if opt.kind == ast::ParallelOptionKind::Workgroup && gpu_domain && exceeds_max_workgroup_size(dims) {
                    return failed("E-CON-0157");
                }
            }
            ast::ParallelOptionKind::Name => {}
        }
    }

    let mut parallel_env = env.clone();
    parallel_env.parallel_context = domain_kind;
    let live_env = Rc::new(RefCell::new(parallel_env.clone()));
    let mut capture_scopes: Vec<ParallelCaptureScope> = type_ctx.parallel_capture_scopes.as_deref().cloned().unwrap_or_default();
    capture_scopes.push(ParallelCaptureScope::default());
    let parallel_ctx = StmtTypeContext {
        in_parallel: true,
        parallel_domain: domain_type.r#type.clone(),
        parallel_capture_scopes: Some(Rc::new(capture_scopes)),
        env_ref: Some(live_env.clone()),
        ..type_ctx.clone()
    };
    let parallel_type_expr = |inner: &ExprPtr| type_expr(ctx, &parallel_ctx, inner, &parallel_env);
    let parallel_type_place = |inner: &ExprPtr| type_place(ctx, &parallel_ctx, inner, &parallel_env);
    let parallel_type_ident = |name: &str| type_identifier_expr(ctx, &parallel_env, name);
    let body_info = type_block_info(
        ctx,
        &parallel_ctx,
        body,
        &parallel_env,
        &parallel_type_expr,
        &parallel_type_ident,
        &parallel_type_place,
        Some(&live_env),
    );
    if !body_info.ok {
        return ExprTypeResult { diag_id: body_info.diag_id, diag_detail: body_info.diag_detail, diag_span: body_info.diag_span, ..Default::default() };
    }
    if gpu_domain {
        let (captures, _) = block_captures(&expr.body, env);
        for captured_name in &captures {
            if let Some(diag_id) = check_gpu_capture(ctx, type_ctx, env, captured_name) {
                return failed(diag_id);
            }
        }
    }
    // A tail that is not itself a task is the block's value.
    if body.tail_opt.is_some() && !is_collectable_parallel_expr(&body.tail_opt) {
        return ExprTypeResult::typed(body_info.r#type);
    }
    // The tasks are typed again in the environment the body ended with.
    let end_env = live_env.borrow().clone();
    let collect_type_expr = |inner: &ExprPtr| type_expr(ctx, &parallel_ctx, inner, &end_env);
    let mut collected: Vec<TypeRef> = Vec::new();
    for stmt in &body.stmts {
        let value = match stmt {
            Stmt::ExprStmt(node) => &node.value,
            Stmt::LetStmt(node) => &node.binding.init,
            Stmt::VarStmt(node) => &node.binding.init,
            Stmt::AssignStmt(node) => &node.value,
            Stmt::CompoundAssignStmt(node) => &node.value,
            _ => continue,
        };
        if let Err(diag_id) = collect_parallel_result_expr(&collect_type_expr, value, &mut collected) {
            return ExprTypeResult::failed(diag_id);
        }
    }
    if let Err(diag_id) = collect_parallel_result_expr(&collect_type_expr, &body.tail_opt, &mut collected) {
        return ExprTypeResult::failed(diag_id);
    }
    ExprTypeResult::typed(match collected.len() {
        0 => make_type_prim("()"),
        1 => collected.remove(0),
        _ => make_type_tuple(collected),
    })
}

/// `spawn [opts] { … }`: a task of the enclosing `parallel` block. A `unique` binding
/// must be moved into it, and only one sibling may take a binding of the block.
pub fn type_spawn_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::SpawnExpr,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> ExprTypeResult {
    let spawned = |inner: TypeRef| ExprTypeResult::typed(make_type_path_with(vec!["Spawned".to_string()], vec![inner]));
    if !type_ctx.in_parallel {
        return failed("E-CON-0101");
    }
    for opt in &expr.opts {
        if opt.value.is_none() {
            return failed("E-CON-0130");
        }
        let opt_type = type_expr_fn(&opt.value);
        if !opt_type.ok {
            return ExprTypeResult::failed(opt_type.diag_id);
        }
        let fits = match opt.kind {
            ast::SpawnOptionKind::Name => matches!(strip_perm(&opt_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::String(_))),
            ast::SpawnOptionKind::Affinity => is_named_type_path(&opt_type.r#type, "CpuSet", false, false),
            ast::SpawnOptionKind::Priority => is_named_type_path(&opt_type.r#type, "Priority", false, false),
        };
        if !fits {
            return failed("E-CON-0130");
        }
    }
    let Some(body) = expr.body.as_deref() else {
        return spawned(make_type_prim("()"));
    };
    let body_ctx = StmtTypeContext { env_ref: None, ..type_ctx.clone() };
    let ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let body_result = type_block(ctx, &body_ctx, body, env, type_expr_fn, &ident_fn, &place_fn, None);
    if !body_result.ok {
        return ExprTypeResult::failed(body_result.diag_id);
    }
    let (captures, explicit_moves) = block_captures(&expr.body, env);
    let scopes = type_ctx.parallel_capture_scopes.as_deref().map(|scopes| &scopes[..]).unwrap_or(&[]);
    // The captures come in the order of the reference's hash set, which decides which
    // one is reported first.
    for captured_name in &captures {
        let Some(binding) = bind_of(env, captured_name) else {
            continue;
        };
        let is_explicit_move = explicit_moves.contains(captured_name);
        if is_explicit_move {
            // A binding of an enclosing parallel block may be moved into one child only.
            if let Some(scope) = scopes.iter().rev().find(|scope| scope.bindings.borrow().contains(captured_name)) {
                if !scope.first_child_moves.borrow_mut().insert(captured_name.clone()) {
                    return failed("E-CON-0122");
                }
            }
        }
        if binding.r#type.is_some() && perm_of_type(&binding.r#type) == Permission::Unique && !is_explicit_move {
            return failed("E-CON-0120");
        }
        if gpu_context(env) {
            if let Some(diag_id) = check_gpu_capture(ctx, type_ctx, env, captured_name) {
                return failed(diag_id);
            }
        }
    }
    spawned(body_result.r#type)
}

/// `dispatch pattern in range [key path mode] [opts] { … }`: the body runs once per
/// index, in parallel. Without a key clause the keys each iteration needs are inferred
/// from the shared data the body uses.
pub fn type_dispatch_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::DispatchExpr,
    env: &TypeEnv,
    type_expr_fn: ExprTypeFn<'_>,
) -> ExprTypeResult {
    if !type_ctx.in_parallel {
        return failed("E-CON-0140");
    }
    let range_result = type_expr_fn(&expr.range);
    if !range_result.ok {
        return ExprTypeResult::failed(range_result.diag_id);
    }
    let stripped_range = strip_perm(&range_result.r#type);
    if !is_range_type(&stripped_range) || matches!(stripped_range.as_deref().map(|ty| &ty.node), Some(TypeNode::RangeFull)) {
        return failed("E-SEM-3133");
    }
    let index_type = range_element_type(&stripped_range).unwrap_or_else(|| make_type_prim("usize"));
    if let Some(key_clause) = &expr.key_clause {
        if bind_of(env, &key_clause.key_path.root).is_none() {
            return failed("ResolveExpr-Ident-Err");
        }
    }
    let mut body_env = crate::typing::type_env::push_scope(env);
    if expr.pattern.is_some() {
        let bindings = match crate::typing::type_env::type_pattern(ctx, &expr.pattern, &index_type) {
            Ok(bindings) => bindings,
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
        };
        if let Some(innermost) = body_env.scopes.last_mut() {
            for (name, ty) in bindings {
                innermost.insert(name, crate::typing::type_env::TypeBinding { r#mut: ast::Mutability::Let, r#type: ty, ..Default::default() });
            }
        }
    }
    let Some(body) = expr.body.as_deref() else {
        return ExprTypeResult::typed(make_type_prim("()"));
    };
    let body_ctx = StmtTypeContext { env_ref: None, ..type_ctx.clone() };
    let ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, type_ctx, inner, env);
    let body_result = type_block(ctx, &body_ctx, body, &body_env, type_expr_fn, &ident_fn, &place_fn, None);
    if !body_result.ok {
        return ExprTypeResult { diag_id: body_result.diag_id, diag_detail: body_result.diag_detail, diag_span: body_result.diag_span, ..Default::default() };
    }
    let (captures, explicit_moves) = crate::typing::closure_capture::dispatch_captures(&expr.body, &expr.pattern, env);
    let scopes = type_ctx.parallel_capture_scopes.as_deref().map(|scopes| &scopes[..]).unwrap_or(&[]);
    for captured_name in &captures {
        let Some(binding) = bind_of(env, captured_name) else {
            continue;
        };
        let is_explicit_move = explicit_moves.contains(captured_name);
        if is_explicit_move {
            if let Some(scope) = scopes.iter().rev().find(|scope| scope.bindings.borrow().contains(captured_name)) {
                if !scope.first_child_moves.borrow_mut().insert(captured_name.clone()) {
                    return failed("E-CON-0122");
                }
            }
        }
        if perm_of_type(&binding.r#type) == Permission::Unique && !is_explicit_move {
            return failed("E-CON-0120");
        }
        if gpu_context(env) {
            if let Some(diag_id) = check_gpu_capture(ctx, type_ctx, env, captured_name) {
                return failed(diag_id);
            }
        }
    }
    let mut has_ordered = false;
    let mut has_reduce = false;
    let mut non_associative_reduce = false;
    for opt in &expr.opts {
        match opt.kind {
            ast::DispatchOptionKind::Ordered => has_ordered = true,
            ast::DispatchOptionKind::Chunk => {
                let chunk_typed = type_expr_fn(&opt.chunk_expr);
                if !chunk_typed.ok {
                    return ExprTypeResult::failed(chunk_typed.diag_id);
                }
                if !matches!(strip_perm_and_refine(&chunk_typed.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize") {
                    return failed("E-SEM-3133");
                }
            }
            ast::DispatchOptionKind::Workgroup => {
                let workgroup_typed = type_expr_fn(&opt.workgroup_expr);
                if !workgroup_typed.ok {
                    return ExprTypeResult::failed(workgroup_typed.diag_id);
                }
                let Some(dims) = extract_dim3_const(ctx, &opt.workgroup_expr, type_expr_fn) else {
                    return failed("E-CON-0159");
                };
                if gpu_context(env) && exceeds_max_workgroup_size(dims) {
                    return failed("E-CON-0157");
                }
            }
            ast::DispatchOptionKind::Reduce => {
                has_reduce = true;
                // Only the built-in reductions are known to be associative.
                if opt.reduce_op == ast::ReduceOp::Custom || !opt.custom_reduce_name.is_empty() {
                    non_associative_reduce = true;
                }
            }
        }
    }
    if non_associative_reduce && !has_ordered {
        return failed("E-CON-0143");
    }
    // The keys each iteration needs: the written clause, or what the body uses.
    let pattern_names = dispatch_pattern_name_set(&expr.pattern);
    let partition_spec = match &expr.key_clause {
        Some(key_clause) => vec![access_of_key_clause(key_clause, &pattern_names)],
        None => match infer_dispatch_accesses(&expr.pattern, body, &body_env) {
            Ok(spec) => spec,
            Err(diag_id) => return failed(diag_id),
        },
    };
    if let (false, Some(diags), true) = (partition_spec.is_empty(), &type_ctx.diags, dynamic_key_pattern(&partition_spec, &pattern_names)) {
        let warn_span = expr.key_clause.as_ref().map_or(&body.span, |key_clause| &key_clause.span);
        if let Some(diag) = make_diagnostic_by_id("W-CON-0140", Some(warn_span.clone())) {
            emit(&mut diags.borrow_mut(), diag);
        }
    }
    if !has_reduce {
        return ExprTypeResult::typed(make_type_prim("()"));
    }
    if gpu_context(env) {
        if let Some(gpu_diag) = gpu_safe_diag_for_type(ctx, &body_result.r#type) {
            emit_supplemental_type_diag(type_ctx, gpu_diag);
            return failed(gpu_diag);
        }
    }
    ExprTypeResult::typed(body_result.r#type)
}
