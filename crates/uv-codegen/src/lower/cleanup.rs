//! Lowering: cleanup.

use super::*;

/// `InitPanicHandle`: in the initialisation of a module, a panic poisons the modules that
/// depend on it.
pub(super) fn init_panic_handle(module: &str, ctx: &mut LowerCtx) -> IrPtr {
    let trace_ir = emit_runtime_trace(ctx);
    let cleanup_ir = emit_cleanup(&[], true, ctx);
    let handle = Arc::new(Ir::InitPanicHandle { module: module.to_string(), poison_modules: ctx.poison_set_for(module), cleanup_ir: Some(cleanup_ir) });
    seq_ir(vec![Some(trace_ir), Some(handle)])
}

/// `PanicFollowup`.
pub(super) fn panic_check(ctx: &mut LowerCtx) -> IrPtr {
    if let Some(module) = ctx.active_static_init_module.clone() {
        return init_panic_handle(&module, ctx);
    }
    let trace_ir = emit_runtime_trace(ctx);
    let plan = cleanup_plan_to_function_root(ctx);
    let cleanup_ir = emit_cleanup(&plan, true, ctx);
    seq_ir(vec![Some(trace_ir), Some(Arc::new(Ir::CleanupPanicCheck { cleanup_ir: Some(cleanup_ir) }))])
}

/// `CleanupTemps`: the drops of the temporaries of a condition or of a branch.
pub(super) fn cleanup_temps_ir(temps: &[TempValue], ctx: &mut LowerCtx) -> IrPtr {
    if temps.is_empty() {
        return empty_ir();
    }
    let plan: Vec<CleanupAction> = temps.iter().rev().filter(|temp| temp.has_responsibility).map(|temp| CleanupAction::DropTemp { ty: temp.ty.clone() }).collect();
    emit_cleanup(&plan, false, ctx)
}

pub(super) fn is_noop_ir(ir: &IrPtr) -> bool {
    matches!(ir.as_ref(), Ir::Opaque)
}

/// `EmitRuntimeTrace`: a call to the conformance runtime when logging, `nop` otherwise.
pub(super) fn emit_runtime_trace(ctx: &mut LowerCtx) -> IrPtr {
    if ctx.log_enabled {
        ctx.unported("runtime traces");
    }
    empty_ir()
}

/// `AppendScopeCleanupItems`: the key scopes are released before any binding is dropped, and
/// the keys taken again after that; each kind from the last registered to the first.
pub(super) fn append_scope_cleanup(scope: &ScopeInfo, ctx: &LowerCtx, plan: &mut Vec<CleanupAction>) {
    for item in scope.cleanup_items.iter().rev() {
        if let CleanupItem::ReleaseKeyScope(name) = item {
            plan.push(CleanupAction::ReleaseKeyScope(name.clone()));
        }
    }
    for item in scope.cleanup_items.iter().rev() {
        if let CleanupItem::ReacquireReleasedKey(name) = item {
            plan.push(CleanupAction::ReacquireReleasedKey(name.clone()));
        }
    }
    for item in scope.cleanup_items.iter().rev() {
        let CleanupItem::DropBinding { name, binding_id } = item else {
            continue;
        };
        let state = ctx.binding_states.get(name).and_then(|states| states.iter().rev().find(|state| state.binding_id == *binding_id));
        let Some(state) = state else {
            continue;
        };
        if !state.has_responsibility || state.is_moved {
            continue;
        }
        plan.push(CleanupAction::DropVar { ty: state.ty.clone(), skip_fields: state.moved_fields.clone() });
    }
}

/// `ComputeCleanupPlanForCurrentScope`.
pub(super) fn cleanup_plan_current_scope(ctx: &LowerCtx) -> Vec<CleanupAction> {
    let mut plan = Vec::new();
    if let Some(scope) = ctx.scope_stack.last() {
        append_scope_cleanup(scope, ctx, &mut plan);
    }
    plan
}

/// `ComputeCleanupPlanToFunctionRoot`.
pub(super) fn cleanup_plan_to_function_root(ctx: &LowerCtx) -> Vec<CleanupAction> {
    let mut plan = Vec::new();
    for scope in ctx.scope_stack.iter().rev() {
        append_scope_cleanup(scope, ctx, &mut plan);
    }
    plan
}

/// `BuildPanicAccess`: the addresses of the fields of the panic record behind `__panic`.
pub(super) fn build_panic_access(ctx: &mut LowerCtx) -> (IrValue, IrValue) {
    let panic_ptr = ctx.fresh_temp_value("panic_ptr");
    let mut deref = DerivedValueInfo::new(DerivedKind::AddrDeref);
    deref.base = IrValue { kind: IrValueKind::Local, name: PANIC_OUT_NAME.to_string(), ..Default::default() };
    ctx.register_derived_value(&panic_ptr, deref);
    ctx.register_value_type(&panic_ptr, panic_out_type());
    let field_ptr = |ctx: &mut LowerCtx, prefix: &str, field: &str, element: &str| {
        let ptr = ctx.fresh_temp_value(prefix);
        let mut info = DerivedValueInfo::new(DerivedKind::AddrField);
        info.base = panic_ptr.clone();
        info.field = field.to_string();
        ctx.register_derived_value(&ptr, info);
        ctx.register_value_type(&ptr, make_type_raw_ptr(RawPtrQual::Mut, make_type_prim(element)));
        ptr
    };
    let flag_ptr = field_ptr(ctx, "panic_flag_ptr", "panic", "bool");
    let code_ptr = field_ptr(ctx, "panic_code_ptr", "code", "u32");
    (flag_ptr, code_ptr)
}

/// The call that carries out one cleanup action, or `nop` when it does nothing: the drop of a
/// value of a primitive type. A drop of any other type is not ported.
fn emit_cleanup_action(action: &CleanupAction, ctx: &mut LowerCtx) -> IrPtr {
    let key_call = |ctx: &mut LowerCtx, symbol: &str, name: &str, result: &str| {
        let result_value = ctx.fresh_temp_value(result);
        ctx.register_value_type(&result_value, make_type_prim("()"));
        let arg = IrValue { kind: IrValueKind::Local, name: name.to_string(), ..Default::default() };
        Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: symbol.to_string(), ..Default::default() }, args: vec![arg], result: result_value })
    };
    match action {
        CleanupAction::DropVar { ty, .. } | CleanupAction::DropTemp { ty } => {
            if !matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
                ctx.unported("drops of values of this type");
            }
            empty_ir()
        }
        CleanupAction::ReleaseKeyScope(name) => key_call(ctx, &concurrency_sym("key_scope_exit"), name, "key_scope_exit"),
        CleanupAction::ReacquireReleasedKey(name) => key_call(ctx, &concurrency_sym("key_reacquire_one"), name, "key_reacquire_one"),
    }
}

/// `EmitCleanup`, and `EmitCleanupOnPanic` when `on_panic`. An empty plan is a trace. Of a
/// plan, the actions that do nothing are skipped, and the others run between the traces
/// that open and close the cleanup; on a panic the record is read first. Actions that
/// can panic (drops of values that are not primitive) are not ported.
pub(super) fn emit_cleanup(plan: &[CleanupAction], on_panic: bool, ctx: &mut LowerCtx) -> IrPtr {
    if plan.is_empty() {
        return emit_runtime_trace(ctx);
    }
    let (flag_ptr, code_ptr) = build_panic_access(ctx);
    let mut parts = Vec::new();
    if on_panic {
        let flag = ctx.fresh_temp_value("panic_flag");
        ctx.register_value_type(&flag, make_type_prim("bool"));
        let code = ctx.fresh_temp_value("panic_code");
        ctx.register_value_type(&code, make_type_prim("u32"));
        let snapshot = seq_ir(vec![Some(Arc::new(Ir::ReadPtr { ptr: flag_ptr, result: flag })), Some(Arc::new(Ir::ReadPtr { ptr: code_ptr, result: code }))]);
        parts.push(Some(snapshot));
    }
    parts.push(Some(emit_runtime_trace(ctx)));
    for action in plan {
        let action_ir = emit_cleanup_action(action, ctx);
        if is_noop_ir(&action_ir) {
            continue;
        }
        parts.push(Some(emit_runtime_trace(ctx)));
        parts.push(Some(action_ir));
        parts.push(Some(emit_runtime_trace(ctx)));
    }
    parts.push(Some(emit_runtime_trace(ctx)));
    seq_ir(parts)
}

/// `TempCleanupIR`: the drops of the temporaries of a statement. Only values of primitive
/// types are ported, whose drop is `nop`.
pub(super) fn temp_cleanup(temps: &[TempValue], ctx: &mut LowerCtx) -> IrPtr {
    let mut parts = Vec::new();
    for temp in temps.iter().rev() {
        if temp.has_responsibility && temp.ty.is_some() {
            if !matches!(temp.ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
                ctx.unported("drops of temporaries of this type");
            }
            parts.push(Some(empty_ir()));
        }
    }
    seq_ir(parts)
}
