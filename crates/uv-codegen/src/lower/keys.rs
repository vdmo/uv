//! Lowering: the key system. A place of shared permission is read and written under a key;
//! a key block takes keys for the length of its body, and an access outside one takes its
//! key for the rest of the enclosing scope.

use uv_analysis::contracts::verification::{evaluate_constant, ConstValue};
use uv_analysis::keys::key_paths::{build_key_path, is_place_expression, is_prefix, key_path_less, parse_key_path_spec};
use uv_project::language_profile::active_language_profile;

use super::*;

/// `ConcurrencySym`: a symbol of the concurrency runtime.
pub(super) fn concurrency_sym(name: &str) -> String {
    format!("{}{name}", active_language_profile().concurrency_symbol_prefix)
}

fn string_immediate(text: &str) -> IrValue {
    IrValue { kind: IrValueKind::Immediate, name: format!("\"{text}\""), bytes: text.as_bytes().to_vec(), ..Default::default() }
}

fn u8_immediate(value: u8) -> IrValue {
    IrValue { kind: IrValueKind::Immediate, name: value.to_string(), bytes: vec![value], ..Default::default() }
}

fn key_scope_type() -> TypeRef {
    make_type_raw_ptr(RawPtrQual::Mut, make_type_prim("u8"))
}

/// `MemoryOrderFromAttrs`.
pub(super) fn memory_order_from_attrs(attrs: &[ast::AttributeItem]) -> Option<AccessOrdering> {
    attrs.iter().find_map(|attr| match attr.name.full_name.as_str() {
        "relaxed" => Some(AccessOrdering::Relaxed),
        "acquire" => Some(AccessOrdering::Acquire),
        "release" => Some(AccessOrdering::Release),
        "acqrel" => Some(AccessOrdering::AcqRel),
        "seqcst" => Some(AccessOrdering::SeqCst),
        _ => None,
    })
}

/// `FormatIndexExpr`: the text of an index in a key path.
fn format_index_expr(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> String {
    match &expr.node {
        ExprNode::AttributedExpr(node) => match &node.expr {
            Some(inner) => format_index_expr(inner, ctx),
            None => "?".to_string(),
        },
        ExprNode::LiteralExpr(lit) => lit.literal.lexeme.clone(),
        ExprNode::RangeExpr(_) => {
            ctx.unported("key paths indexed by a range");
            "?".to_string()
        }
        _ => "?".to_string(),
    }
}

fn index_is_static(expr: &Option<Arc<Expr>>) -> bool {
    evaluate_constant(expr) != ConstValue::Unknown
}

/// `EncodeRuntimeSharedAccessPath`: the path of a place as the runtime is told it.
fn encode_runtime_shared_access_path(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> Option<String> {
    match &expr.node {
        ExprNode::IdentifierExpr(node) => Some(node.name.clone()),
        ExprNode::AttributedExpr(node) => encode_runtime_shared_access_path(node.expr.as_ref()?, ctx),
        ExprNode::FieldAccessExpr(node) => {
            let mut encoded = encode_runtime_shared_access_path(node.base.as_ref()?, ctx)?;
            encoded.push_str(".f:");
            encoded.push_str(&node.name);
            Some(encoded)
        }
        ExprNode::TupleAccessExpr(node) => {
            let mut encoded = encode_runtime_shared_access_path(node.base.as_ref()?, ctx)?;
            encoded.push_str(".f:");
            encoded.push_str(&uv_analysis::keys::key_paths::format_tuple_index(node.index));
            Some(encoded)
        }
        ExprNode::IndexAccessExpr(node) => {
            let mut encoded = encode_runtime_shared_access_path(node.base.as_ref()?, ctx)?;
            if !index_is_static(&node.index) {
                return Some(encoded);
            }
            encoded.push_str(".i:");
            encoded.push_str(&format_index_expr(node.index.as_ref()?, ctx));
            Some(encoded)
        }
        _ => None,
    }
}

/// `EncodeLoweredKeyPath`.
fn encode_lowered_key_path(path: &KeyPath) -> String {
    let mut encoded = path.root.clone();
    for seg in &path.segs {
        encoded.push('.');
        encoded.push_str(if seg.is_index { "i:" } else { "f:" });
        encoded.push_str(&seg.name);
    }
    encoded
}

/// `KeyModeSufficient`: a write key covers a read, a read key covers only reads.
fn key_mode_sufficient(held: u8, required: ast::KeyMode) -> bool {
    held == 1 || required == ast::KeyMode::Read
}

/// `HasCoveringActiveKey`.
fn has_covering_active_key(path: &KeyPath, required: ast::KeyMode, ctx: &LowerCtx) -> bool {
    ctx.active_key_scopes.iter().rev().any(|scope| scope.acquired.iter().any(|acquired| is_prefix(&acquired.path, path) && key_mode_sufficient(acquired.mode, required)))
}

/// `EnsureImplicitKeyScope`: the key scope of the enclosing scope, entered on first use.
fn ensure_implicit_key_scope(ctx: &mut LowerCtx) -> (IrPtr, IrValue) {
    let scope_runtime_id = ctx.current_runtime_scope_id().unwrap_or(0);
    if scope_runtime_id == 0 {
        return (empty_ir(), IrValue::default());
    }
    if let Some(name) = ctx.implicit_key_scope_names.get(&scope_runtime_id).cloned() {
        let scope_local = IrValue { kind: IrValueKind::Local, name, ..Default::default() };
        ctx.register_value_type(&scope_local, key_scope_type());
        return (empty_ir(), scope_local);
    }
    let enter_result = ctx.fresh_temp_value("implicit_key_scope_enter");
    ctx.register_value_type(&enter_result, key_scope_type());
    let enter = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_scope_enter"), ..Default::default() }, args: Vec::new(), result: enter_result.clone() });
    let scope_local_name = ctx.fresh_temp_value("__uv_implicit_key_scope").name;
    let bind = Arc::new(Ir::BindVar { name: scope_local_name.clone(), stable_name: String::new(), value: enter_result, ty: key_scope_type(), prov: ProvenanceKind::Bottom, prov_region: None, prov_region_tag: None });
    ctx.register_var(&scope_local_name, key_scope_type(), false, ProvenanceKind::Bottom, None, None);
    ctx.register_key_scope_exit(&scope_local_name);
    ctx.implicit_key_scope_names.insert(scope_runtime_id, scope_local_name.clone());
    ctx.active_key_scopes.push(ActiveKeyScope { scope_runtime_id, scope_name: scope_local_name.clone(), acquired: Vec::new() });
    let scope_local = IrValue { kind: IrValueKind::Local, name: scope_local_name, ..Default::default() };
    ctx.register_value_type(&scope_local, key_scope_type());
    (seq_ir(vec![Some(enter), Some(bind)]), scope_local)
}

/// `LowerImplicitKeyAccess`: the key an access to a place of shared permission needs, unless
/// a key already held covers it.
pub(super) fn lower_implicit_key_access(expr: &Arc<Expr>, mode: ast::KeyMode, ctx: &mut LowerCtx) -> IrPtr {
    let expr_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if perm_of_type(&expr_type) != Permission::Shared {
        return empty_ir();
    }
    let place = Some(expr.clone());
    if !is_place_expression(&place) {
        return empty_ir();
    }
    let built = build_key_path(&place);
    if !built.success {
        return empty_ir();
    }
    if has_covering_active_key(&built.path, mode, ctx) {
        return empty_ir();
    }
    let (scope_setup, scope_local) = ensure_implicit_key_scope(ctx);
    if scope_local.name.is_empty() {
        ctx.unported("implicit keys outside a scope");
        return empty_ir();
    }
    let encoded_path = match encode_runtime_shared_access_path(expr, ctx) {
        Some(path) if !path.is_empty() => path,
        _ => encode_lowered_key_path(&built.path),
    };
    let mode_byte = u8::from(mode == ast::KeyMode::Write);
    let check_result = ctx.fresh_temp_value("implicit_key_conflict_check");
    ctx.register_value_type(&check_result, make_type_prim("()"));
    let check = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_check_conflict"), ..Default::default() }, args: vec![string_immediate(&encoded_path), u8_immediate(mode_byte)], result: check_result });
    let acquire_result = ctx.fresh_temp_value("implicit_key_acquire");
    ctx.register_value_type(&acquire_result, make_type_prim("()"));
    let acquire = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_acquire"), ..Default::default() }, args: vec![scope_local.clone(), string_immediate(&encoded_path), u8_immediate(mode_byte)], result: acquire_result });
    if let Some(scope) = ctx.active_key_scopes.iter_mut().rev().find(|scope| scope.scope_name == scope_local.name) {
        scope.acquired.push(AcquiredKey { path: built.path, encoded: encoded_path, mode: mode_byte });
    }
    seq_ir(vec![Some(scope_setup), Some(check), Some(acquire)])
}

/// `IsSharedAccessExpr`: an access whose place or receiver is of shared permission.
pub(super) fn is_shared_access_expr(expr: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    let shared = |of: &Arc<Expr>| perm_of_type(&stored_expr_type(&ctx.scope, &Some(of.clone())).flatten()) == Permission::Shared;
    match &expr.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().is_some_and(|inner| is_shared_access_expr(inner, ctx)),
        ExprNode::IdentifierExpr(_) | ExprNode::FieldAccessExpr(_) | ExprNode::TupleAccessExpr(_) | ExprNode::IndexAccessExpr(_) | ExprNode::DerefExpr(_) => stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten().is_some() && shared(expr),
        ExprNode::MethodCallExpr(node) => node.receiver.as_ref().is_some_and(shared),
        _ => false,
    }
}

fn fence_ir(order: IrFenceOrder, ctx: &mut LowerCtx) -> IrPtr {
    let result = ctx.fresh_temp_value("ordered_access_fence");
    ctx.register_value_type(&result, make_type_prim("()"));
    Arc::new(Ir::Fence { order, result })
}

/// `ApplyEffectiveOrdering`: fences around an access to shared data, as the enclosing
/// key block orders it (sequentially consistent when it says nothing).
pub(super) fn apply_effective_ordering(expr: &Arc<Expr>, mut result: LowerResult, ctx: &mut LowerCtx) -> LowerResult {
    if !is_shared_access_expr(expr, ctx) {
        return result;
    }
    let order = ctx.current_access_order.unwrap_or(AccessOrdering::SeqCst);
    result.ir = match order {
        AccessOrdering::Relaxed => return result,
        AccessOrdering::Acquire => {
            let after = fence_ir(IrFenceOrder::Acquire, ctx);
            seq_ir(vec![Some(result.ir), Some(after)])
        }
        AccessOrdering::Release => {
            let before = fence_ir(IrFenceOrder::Release, ctx);
            seq_ir(vec![Some(before), Some(result.ir)])
        }
        AccessOrdering::AcqRel => {
            let before = fence_ir(IrFenceOrder::Release, ctx);
            let after = fence_ir(IrFenceOrder::Acquire, ctx);
            seq_ir(vec![Some(before), Some(result.ir), Some(after)])
        }
        AccessOrdering::SeqCst => {
            let before = fence_ir(IrFenceOrder::SeqCst, ctx);
            let after = fence_ir(IrFenceOrder::SeqCst, ctx);
            seq_ir(vec![Some(before), Some(result.ir), Some(after)])
        }
    };
    result
}

/// `EncodeKeyPath` of a key block: a path ends at a marked segment or at an index that is
/// only known at run time.
fn encode_key_path(path: &ast::KeyPathExpr, ctx: &mut LowerCtx) -> String {
    let mut encoded = path.root.clone();
    for seg in &path.segs {
        encoded.push('.');
        match seg {
            ast::KeySeg::KeySegField(field) => {
                encoded.push_str("f:");
                encoded.push_str(&field.name);
                if field.marked {
                    break;
                }
            }
            ast::KeySeg::KeySegIndex(index) => {
                if !index_is_static(&index.expr) {
                    break;
                }
                encoded.push_str("i:");
                match &index.expr {
                    Some(expr) => encoded.push_str(&format_index_expr(expr, ctx)),
                    None => encoded.push('?'),
                }
                if index.marked {
                    break;
                }
            }
        }
    }
    encoded
}

fn conflict_check(path: &str, mode: ast::KeyMode, ctx: &mut LowerCtx) -> IrPtr {
    let result = ctx.fresh_temp_value("key_conflict_check");
    ctx.register_value_type(&result, make_type_prim("()"));
    Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_check_conflict"), ..Default::default() }, args: vec![string_immediate(path), u8_immediate(u8::from(mode == ast::KeyMode::Write))], result })
}

/// `LowerKeyBlockStmt`: the keys are taken in their canonical order for the body, and
/// released when the scope that holds them ends, however it ends.
pub(super) fn lower_key_block_stmt(stmt: &ast::KeyBlockStmt, ctx: &mut LowerCtx) -> IrPtr {
    let Some(body) = &stmt.body else {
        return empty_ir();
    };
    if stmt.kind == ast::KeyBlockKind::SpeculativeWrite {
        ctx.unported("speculative key blocks");
        return empty_ir();
    }
    ctx.push_scope();
    let unit_type = make_type_prim("()");
    let scope_type = key_scope_type();
    let encoded_paths: Vec<String> = stmt.paths.iter().map(|path| encode_key_path(path, ctx)).collect();
    let mut setup_parts: Vec<Option<IrPtr>> = Vec::new();

    // A release block first lets go of the keys of the enclosing scopes that it names.
    if stmt.kind == ast::KeyBlockKind::Release {
        let mut outer_keys: Vec<(String, String)> = Vec::new();
        for encoded in &encoded_paths {
            let held = ctx.active_key_scopes.iter().rev().find_map(|scope| scope.acquired.iter().find(|acquired| acquired.encoded == *encoded).map(|acquired| (scope.scope_name.clone(), acquired.encoded.clone())));
            outer_keys.extend(held);
        }
        for (scope_name, path) in outer_keys.iter().rev() {
            let outer_scope_local = IrValue { kind: IrValueKind::Local, name: scope_name.clone(), ..Default::default() };
            ctx.register_value_type(&outer_scope_local, scope_type.clone());
            let released = ctx.fresh_temp_value("key_release_one");
            ctx.register_value_type(&released, scope_type.clone());
            setup_parts.push(Some(Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_release_one"), ..Default::default() }, args: vec![outer_scope_local, string_immediate(path)], result: released.clone() })));
            let handle_name = ctx.fresh_temp_value("__uv_released_key").name;
            setup_parts.push(Some(Arc::new(Ir::BindVar { name: handle_name.clone(), stable_name: String::new(), value: released, ty: scope_type.clone(), prov: ProvenanceKind::Bottom, prov_region: None, prov_region_tag: None })));
            ctx.register_var(&handle_name, scope_type.clone(), false, ProvenanceKind::Bottom, None, None);
            ctx.register_released_key_reacquire(&handle_name);
        }
    }

    let enter_result = ctx.fresh_temp_value("key_scope_enter");
    ctx.register_value_type(&enter_result, scope_type.clone());
    setup_parts.push(Some(Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_scope_enter"), ..Default::default() }, args: Vec::new(), result: enter_result.clone() })));
    let scope_local_name = ctx.fresh_temp_value("__uv_key_scope").name;
    setup_parts.push(Some(Arc::new(Ir::BindVar { name: scope_local_name.clone(), stable_name: String::new(), value: enter_result, ty: scope_type.clone(), prov: ProvenanceKind::Bottom, prov_region: None, prov_region_tag: None })));
    ctx.register_var(&scope_local_name, scope_type.clone(), false, ProvenanceKind::Bottom, None, None);
    let scope_local = IrValue { kind: IrValueKind::Local, name: scope_local_name.clone(), ..Default::default() };
    ctx.register_value_type(&scope_local, scope_type);
    ctx.register_key_scope_exit(&scope_local_name);
    let mut active_scope = ActiveKeyScope { scope_runtime_id: ctx.current_runtime_scope_id().unwrap_or(0), scope_name: scope_local_name, acquired: Vec::new() };

    let mode = stmt.mode;
    let key_mode = u8::from(mode == ast::KeyMode::Write);
    let mut sorted: Vec<(KeyPath, String)> = stmt.paths.iter().zip(&encoded_paths).map(|(path, encoded)| (parse_key_path_spec(path), encoded.clone())).collect();
    sorted.sort_by(|a, b| if key_path_less(&a.0, &b.0) { std::cmp::Ordering::Less } else if key_path_less(&b.0, &a.0) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal });
    for (path, encoded) in sorted {
        setup_parts.push(Some(conflict_check(&encoded, mode, ctx)));
        let acquire_result = ctx.fresh_temp_value("key_acquire");
        ctx.register_value_type(&acquire_result, unit_type.clone());
        setup_parts.push(Some(Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: concurrency_sym("key_acquire"), ..Default::default() }, args: vec![scope_local.clone(), string_immediate(&encoded), u8_immediate(key_mode)], result: acquire_result })));
        active_scope.acquired.push(AcquiredKey { path, encoded, mode: key_mode });
    }
    ctx.active_key_scopes.push(active_scope);

    let prev_order = ctx.current_access_order;
    if let Some(order) = memory_order_from_attrs(&stmt.attrs) {
        ctx.current_access_order = Some(order);
    }
    let body_result = lower_block(body, ctx);
    ctx.current_access_order = prev_order;
    ctx.active_key_scopes.pop();

    let plan = cleanup_plan_current_scope(ctx);
    let cleanup_ir = emit_cleanup(&plan, false, ctx);
    ctx.pop_scope();

    if ctx.temp_sink.is_some() {
        let result_type = match &body.tail_opt {
            Some(tail) => stored_expr_type(&ctx.scope, &Some(tail.clone())).flatten(),
            None => make_type_prim("()"),
        };
        ctx.register_temp_value(&body_result.value, &result_type, true);
    }
    let body_ir = seq_ir(vec![Some(body_result.ir), Some(cleanup_ir)]);
    Arc::new(Ir::Block { setup: Some(seq_ir(setup_parts)), body: Some(body_ir), value: body_result.value })
}
