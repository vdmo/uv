//! Lowering: stmt.

use super::*;

pub(super) fn same_ir_value(a: &IrValue, b: &IrValue) -> bool {
    a.kind == b.kind && a.name == b.name && a.bytes == b.bytes && a.literal_id == b.literal_id && a.vtable_sym == b.vtable_sym
}

/// `ReturnDestExpr`.
fn return_dest_expr(value: &Arc<Expr>, ctx: &LowerCtx) -> Arc<Expr> {
    if matches!(value.node, ExprNode::MoveExpr(_) | ExprNode::CopyExpr(_)) || !is_place_expr(&Some(value.clone())) {
        return value.clone();
    }
    if let Some(root) = place_root(value) {
        if ctx.binding_state(&root).is_some_and(|state| !state.has_responsibility) {
            return value.clone();
        }
    }
    Arc::new(Expr { span: value.span.clone(), node: ExprNode::MoveExpr(ast::MoveExpr { place: Some(value.clone()) }) })
}

pub(super) fn lower_return_stmt(stmt: &ast::ReturnStmt, ctx: &mut LowerCtx) -> IrPtr {
    let mut parts: Vec<Option<IrPtr>> = Vec::new();
    let (value, value_type) = match &stmt.value_opt {
        Some(expr) => {
            // `ReturnDestExpr`: a place is returned as a move unless its binding holds no responsibility.
            let expr = &return_dest_expr(expr, ctx);
            let prev_suppress = ctx.suppress_temp_at_depth;
            ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
            let result = lower_expr(expr, ctx);
            ctx.suppress_temp_at_depth = prev_suppress;
            parts.push(Some(result.ir));
            let mut ty = ctx.lookup_value_type(&result.value);
            if ty.is_none() {
                ty = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
            }
            (result.value, ty)
        }
        None => (ctx.fresh_temp_value("unit"), make_type_prim("()")),
    };
    if stmt.value_opt.is_some() && (outcome_sig_of(&ctx.proc_ret_type).is_some() || outcome_sig_of(&value_type).is_some()) {
        ctx.unported("returning into an Outcome");
    }
    if value_type.is_some() {
        ctx.register_value_type(&value, value_type.clone());
    }
    if async_sig_of(&ctx.scope, &ctx.proc_ret_type).is_some() {
        ctx.unported("returns of async procedures");
    }
    // `SnapshotReturnValueForPostcondition`: a value that is not copied bit by bit is bound to
    // a local before the cleanup that may drop what it was made from. There is no
    // postcondition to read it.
    let mut value = value;
    let needs_snapshot = value_type.is_some() && !is_unit_type(&value_type) && !bitcopy_type(&ctx.scope, &value_type);
    if needs_snapshot && value.kind != IrValueKind::Immediate {
        let name = ctx.fresh_temp_value("return_snapshot").name;
        let stable_name = ctx.register_var(&name, value_type.clone(), false, ProvenanceKind::Bottom, None, None);
        parts.push(Some(Arc::new(Ir::BindVar { name: name.clone(), stable_name, value: value.clone(), ty: value_type.clone(), prov: ProvenanceKind::Stack, prov_region: None, prov_region_tag: None })));
        value = IrValue { kind: IrValueKind::Local, name, ..Default::default() };
        ctx.register_value_type(&value, value_type.clone());
    }
    if stmt.value_opt.is_some() {
        // The check for a refinement of the returned expression: `nop` when there is none.
        parts.push(Some(empty_ir()));
    }
    // The temporaries of the statement, but the returned value, are dropped before the scopes unwind.
    let temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = Some(Vec::new());
    let cleanup_temps: Vec<TempValue> = temps.into_iter().filter(|temp| !same_ir_value(&temp.value, &value)).collect();
    if !cleanup_temps.is_empty() {
        parts.push(Some(temp_cleanup(&cleanup_temps, ctx)));
    }
    let plan = cleanup_plan_to_function_root(ctx);
    parts.push(Some(emit_cleanup(&plan, false, ctx)));
    parts.push(Some(Arc::new(Ir::Return { value })));
    seq_ir(parts)
}

/// `LowerWritePlace` of a local: the binding holds a value again, and the old one is dropped by the store.
fn lower_write_place(place: &Arc<Expr>, value: &IrValue, ctx: &mut LowerCtx) -> IrPtr {
    let ExprNode::IdentifierExpr(ident) = &place.node else {
        ctx.unported(&format!("writes to the place {}", variant_name(&place.node)));
        return empty_ir();
    };
    if ctx.binding_state(&ident.name).is_none() {
        ctx.unported("writes to places that are not local");
        return empty_ir();
    }
    if let Some(state) = ctx.binding_states.get_mut(&ident.name).and_then(|states| states.last_mut()) {
        state.is_moved = false;
        state.moved_fields.clear();
    }
    let key_ir = lower_implicit_key_access(place, ast::KeyMode::Write, ctx);
    seq_ir(vec![Some(key_ir), Some(Arc::new(Ir::StoreVar { name: ident.name.clone(), value: value.clone() }))])
}

/// `LowerAssignStmt`.
fn lower_assign_stmt(stmt: &ast::AssignStmt, ctx: &mut LowerCtx) -> IrPtr {
    let (Some(place), Some(value)) = (&stmt.place, &stmt.value) else {
        ctx.unported("assignments without a place or a value");
        return empty_ir();
    };
    let rhs_result = lower_expr(value, ctx);
    // A bare value assigned to a place of an Outcome type is introduced into it.
    let place_type = stored_expr_type(&ctx.scope, &Some(place.clone())).flatten();
    if outcome_sig_of(&place_type).is_some() {
        ctx.unported("assigning into a place of an Outcome type");
    }
    let write_ir = lower_write_place(place, &rhs_result.value, ctx);
    seq_ir(vec![Some(rhs_result.ir), Some(write_ir)])
}

/// `LowerCompoundAssignStmt`: the place is read, combined, and written back.
fn lower_compound_assign_stmt(stmt: &ast::CompoundAssignStmt, ctx: &mut LowerCtx) -> IrPtr {
    let (Some(place), Some(value)) = (&stmt.place, &stmt.value) else {
        ctx.unported("assignments without a place or a value");
        return empty_ir();
    };
    let lhs_result = lower_read_place(place, ctx);
    let rhs_result = lower_expr(value, ctx);
    let op = stmt.op.strip_suffix('=').unwrap_or(&stmt.op);
    let new_value = ctx.fresh_temp_value("binop");
    let mut op_parts: Vec<Option<IrPtr>> = Vec::new();
    let reason = match op {
        "/" | "%" => Some("DivZero"),
        "<<" | ">>" => Some("Shift"),
        "+" | "-" | "*" | "**" => Some("Overflow"),
        _ => None,
    };
    if let Some(reason) = reason {
        op_parts.push(Some(Arc::new(Ir::CheckOp { op: op.to_string(), reason: reason.to_string(), lhs: lhs_result.value.clone(), rhs: Some(rhs_result.value.clone()) })));
        op_parts.push(Some(panic_check(ctx)));
    }
    op_parts.push(Some(Arc::new(Ir::BinaryOp { op: op.to_string(), lhs: lhs_result.value, rhs: rhs_result.value.clone(), result: new_value.clone() })));
    let op_ir = seq_ir(op_parts);
    let write_ir = lower_write_place(place, &new_value, ctx);
    seq_ir(vec![Some(lhs_result.ir), Some(rhs_result.ir), Some(op_ir), Some(write_ir)])
}

pub(super) fn lower_expr_stmt(stmt: &ast::ExprStmt, ctx: &mut LowerCtx) -> IrPtr {
    let Some(value) = &stmt.value else {
        return empty_ir();
    };
    if matches!(value.node, ExprNode::MethodCallExpr(_)) {
        ctx.unported("method calls as statements");
    }
    lower_expr(value, ctx).ir
}

/// `PlaceRoot`: the name a place starts from.
pub(super) fn place_root(expr: &Arc<Expr>) -> Option<String> {
    match &expr.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().and_then(place_root),
        ExprNode::IdentifierExpr(node) => Some(node.name.clone()),
        ExprNode::FieldAccessExpr(node) => node.base.as_ref().and_then(place_root),
        ExprNode::TupleAccessExpr(node) => node.base.as_ref().and_then(place_root),
        ExprNode::IndexAccessExpr(node) => node.base.as_ref().and_then(place_root),
        ExprNode::DerefExpr(node) => node.value.as_ref().and_then(place_root),
        _ => None,
    }
}

/// `BindingInitializerHasResponsibility`.
pub(super) fn binding_initializer_has_responsibility(init: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    match &init.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().is_none_or(|inner| binding_initializer_has_responsibility(inner, ctx)),
        ExprNode::MoveExpr(_) | ExprNode::CopyExpr(_) => true,
        ExprNode::LiteralExpr(_) => false,
        _ if is_place_expr(&Some(init.clone())) => false,
        ExprNode::CallExpr(call) => call_result_has_responsibility(&ctx.scope, call).unwrap_or(true),
        _ => true,
    }
}

/// `LowerLetStmt` and `LowerVarStmt`: they differ only in the implicit introduction of an Outcome.
pub(super) fn lower_binding_stmt(binding: &ast::Binding, is_let: bool, ctx: &mut LowerCtx) -> IrPtr {
    let Some(init) = &binding.init else {
        return empty_ir();
    };
    let typed_pattern_type = match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::TypedPattern(typed)) => typed.r#type.clone(),
        _ => None,
    };
    let annotation = binding.type_opt.clone().or(typed_pattern_type);
    let mut var_type = annotation.and_then(|written| lower_type_for_layout(&ctx.scope, &Some(written))).flatten();
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let init_result = lower_expr(init, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    if var_type.is_none() {
        var_type = stored_expr_type(&ctx.scope, &Some(init.clone())).flatten();
    }
    if var_type.is_none() {
        var_type = ctx.lookup_value_type(&init_result.value);
    }
    if is_let && outcome_sig_of(&var_type).is_some() {
        ctx.unported("implicit introduction of an Outcome");
    }
    let stripped = strip_perm(&var_type).or_else(|| var_type.clone());
    if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Dynamic(_))) {
        ctx.unported("widening to a dynamic class type");
    }
    // `BindProvInfo`: a binding takes the provenance of its initializer, and a value whose
    // provenance is bottom lives on the stack.
    let init_key = Arc::as_ptr(init) as usize;
    let init_kind = ctx.expr_prov.as_ref().and_then(|maps| maps.prov.get(&init_key).copied()).unwrap_or(ProvenanceKind::Bottom);
    let (prov, prov_region, prov_region_tag) = if init_kind == ProvenanceKind::Bottom {
        (ProvenanceKind::Stack, None, None)
    } else if init_kind == ProvenanceKind::Region {
        let maps = ctx.expr_prov.as_ref();
        let region = maps.and_then(|maps| maps.region_targets.get(&init_key)).cloned();
        let tag = maps.and_then(|maps| maps.region_tags.get(&init_key)).cloned();
        // `StableRegionProv`: the region is named by the stable name of its binding.
        let region = region.map(|name| ctx.binding_state(&name).map_or(name, |state| state.stable_name.clone()));
        (init_kind, region, tag)
    } else {
        (init_kind, None, None)
    };
    let has_responsibility = binding_initializer_has_responsibility(init, ctx);
    let mut bind_ir = empty_ir();
    let mut checked_value = init_result.value.clone();
    match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::IdentifierPattern(pat)) => {
            let stable_name = ctx.register_var(&pat.name, var_type.clone(), has_responsibility, prov, prov_region.clone(), prov_region_tag.clone());
            bind_ir = Arc::new(Ir::BindVar { name: pat.name.clone(), stable_name, value: init_result.value.clone(), ty: var_type.clone(), prov, prov_region: prov_region.clone(), prov_region_tag: prov_region_tag.clone() });
            checked_value = IrValue { kind: IrValueKind::Local, name: pat.name.clone(), ..Default::default() };
            ctx.register_value_type(&checked_value, var_type.clone());
        }
        Some(ast::PatternNode::TypedPattern(pat)) if pat.name != "_" => {
            if matches!(strip_perm(&ctx.lookup_value_type(&init_result.value)).as_deref().map(|ty| &ty.node), Some(TypeNode::Union(_))) {
                ctx.unported("binding a union value to a typed pattern");
            }
            let stable_name = ctx.register_var(&pat.name, var_type.clone(), has_responsibility, prov, prov_region.clone(), prov_region_tag.clone());
            bind_ir = Arc::new(Ir::BindVar { name: pat.name.clone(), stable_name, value: init_result.value.clone(), ty: var_type.clone(), prov, prov_region: prov_region.clone(), prov_region_tag: prov_region_tag.clone() });
            checked_value = IrValue { kind: IrValueKind::Local, name: pat.name.clone(), ..Default::default() };
            ctx.register_value_type(&checked_value, var_type.clone());
        }
        Some(ast::PatternNode::WildcardPattern(_)) => {}
        _ => ctx.unported("this kind of binding pattern"),
    }
    let _ = checked_value;
    // The checks for a refinement of the initializer: `nop` when there are none.
    let refine_ir = empty_ir();
    seq_ir(vec![Some(init_result.ir), Some(bind_ir), Some(refine_ir)])
}

pub(super) fn lower_stmt(stmt: &Stmt, ctx: &mut LowerCtx) -> IrPtr {
    let prev_sink = ctx.temp_sink.replace(Vec::new());
    let mut temps_handled = false;
    let ir = match stmt {
        Stmt::ReturnStmt(node) => {
            temps_handled = true;
            lower_return_stmt(node, ctx)
        }
        Stmt::ExprStmt(node) => lower_expr_stmt(node, ctx),
        Stmt::LetStmt(node) => lower_binding_stmt(&node.binding, true, ctx),
        Stmt::VarStmt(node) => lower_binding_stmt(&node.binding, false, ctx),
        Stmt::KeyBlockStmt(node) => lower_key_block_stmt(node, ctx),
        Stmt::AssignStmt(node) => lower_assign_stmt(node, ctx),
        Stmt::CompoundAssignStmt(node) => lower_compound_assign_stmt(node, ctx),
        _ => {
            ctx.unported(&format!("statement {}", variant_name(stmt)));
            empty_ir()
        }
    };
    let temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = prev_sink;
    if temps_handled {
        return ir;
    }
    let cleanup = temp_cleanup(&temps, ctx);
    if matches!(cleanup.as_ref(), Ir::Opaque) {
        ir
    } else {
        seq_ir(vec![Some(ir), Some(cleanup)])
    }
}

pub(super) fn lower_stmt_list(stmts: &[Stmt], ctx: &mut LowerCtx) -> IrPtr {
    if stmts.is_empty() {
        return empty_ir();
    }
    let parts = stmts.iter().map(|stmt| Some(lower_stmt(stmt, ctx))).collect();
    seq_ir(parts)
}

pub fn lower_block(block: &ast::Block, ctx: &mut LowerCtx) -> LowerResult {
    ctx.push_scope();
    let stmts_ir = lower_stmt_list(&block.stmts, ctx);
    let (tail_ir, value) = match &block.tail_opt {
        Some(tail) => {
            let prev_suppress = ctx.suppress_temp_at_depth;
            ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
            let result = lower_expr(tail, ctx);
            ctx.suppress_temp_at_depth = prev_suppress;
            (result.ir, result.value)
        }
        None => {
            let value = ctx.fresh_temp_value("unit");
            ctx.register_value_type(&value, make_type_prim("()"));
            (empty_ir(), value)
        }
    };
    let scope_enter_ir = empty_ir();
    let plan = cleanup_plan_current_scope(ctx);
    let cleanup_ir = emit_cleanup(&plan, false, ctx);
    ctx.pop_scope();
    let setup = seq_ir(vec![Some(scope_enter_ir), Some(stmts_ir)]);
    let body = if !ir_flow_may_fall_through(&Some(tail_ir.clone())) { tail_ir } else { seq_ir(vec![Some(tail_ir), Some(cleanup_ir)]) };
    LowerResult { ir: Arc::new(Ir::Block { setup: Some(setup), body: Some(body), value: value.clone() }), value }
}
