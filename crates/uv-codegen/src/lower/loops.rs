//! Lowering: loops (`LowerLoopConditional`, `LowerLoopInfinite`).

use super::*;

/// `EmitLoopInvariantCheck`: the check of a loop invariant, `nop` unless the procedure checks
/// as it runs. The checks are not ported.
fn loop_invariant_check(invariant: &ast::LoopInvariant, ctx: &mut LowerCtx) -> IrPtr {
    if ctx.dynamic_checks && invariant.predicate.is_some() {
        ctx.unported("checks of loop invariants");
    }
    empty_ir()
}

fn finish_loop(expr: &Arc<Expr>, mut loop_ir: Ir, prefix: &str, ctx: &mut LowerCtx) -> LowerResult {
    let result = ctx.fresh_temp_value(prefix);
    if let Some(ty) = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten() {
        ctx.register_value_type(&result, Some(ty));
    }
    if let Ir::Loop { result: slot, .. } = &mut loop_ir {
        *slot = result.clone();
    }
    LowerResult { ir: Arc::new(loop_ir), value: result }
}

/// `LowerLoopConditional`.
pub(super) fn lower_loop_conditional(expr: &Arc<Expr>, node: &ast::LoopConditionalExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(cond), Some(body)) = (&node.cond, &node.body) else {
        ctx.unported("conditional loops without a condition or a body");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("while") };
    };
    ctx.push_scope_kind(true);
    let cond_result = lower_expr(cond, ctx);
    let body_result = lower_block(body, ctx);
    let (entry, backedge) = match &node.invariant_opt {
        Some(invariant) => {
            let entry = loop_invariant_check(invariant, ctx);
            let backedge = loop_invariant_check(invariant, ctx);
            (Some(entry), Some(backedge))
        }
        None => (None, None),
    };
    ctx.pop_scope();
    let loop_ir = Ir::Loop {
        kind: IrLoopKind::Conditional,
        pattern: Default::default(),
        iter_ir: None,
        iter_value: None,
        cond_ir: Some(cond_result.ir),
        cond_value: Some(cond_result.value),
        body_ir: Some(body_result.ir),
        invariant_entry_ir: entry,
        invariant_backedge_ir: backedge,
        body_value: body_result.value,
        result: IrValue::default(),
    };
    finish_loop(expr, loop_ir, "while", ctx)
}

/// `LowerLoopInfinite`.
pub(super) fn lower_loop_infinite(expr: &Arc<Expr>, node: &ast::LoopInfiniteExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(body) = &node.body else {
        ctx.unported("infinite loops without a body");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("loop") };
    };
    ctx.push_scope_kind(true);
    let body_result = lower_block(body, ctx);
    let (entry, backedge) = match &node.invariant_opt {
        Some(invariant) => {
            let entry = loop_invariant_check(invariant, ctx);
            let backedge = loop_invariant_check(invariant, ctx);
            (Some(entry), Some(backedge))
        }
        None => (None, None),
    };
    ctx.pop_scope();
    let loop_ir = Ir::Loop {
        kind: IrLoopKind::Infinite,
        pattern: Default::default(),
        iter_ir: None,
        iter_value: None,
        cond_ir: None,
        cond_value: None,
        body_ir: Some(body_result.ir),
        invariant_entry_ir: entry,
        invariant_backedge_ir: backedge,
        body_value: body_result.value,
        result: IrValue::default(),
    };
    finish_loop(expr, loop_ir, "loop", ctx)
}

/// `LoopPatternType`: the type of what an iteration binds.
fn loop_pattern_type(iter_type: &TypeRef) -> TypeRef {
    match iter_type.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Array { element, .. } | TypeNode::Slice(element)) => element.clone(),
        Some(TypeNode::Range(base) | TypeNode::RangeInclusive(base) | TypeNode::RangeFrom(base) | TypeNode::RangeTo(base) | TypeNode::RangeToInclusive(base)) => base.clone(),
        _ => None,
    }
}

/// `LowerLoopIter`.
pub(super) fn lower_loop_iter(expr: &Arc<Expr>, node: &ast::LoopIterExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(pattern), Some(iter), Some(body)) = (&node.pattern, &node.iter, &node.body) else {
        ctx.unported("iterating loops without a pattern, an iterator or a body");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("for") };
    };
    ctx.push_scope_kind(true);
    let iter_result = lower_expr(iter, ctx);
    let iter_type = stored_expr_type(&ctx.scope, &Some(iter.clone())).flatten();
    if iter_type.as_ref().is_some_and(|ty| async_sig_of(&ctx.scope, &Some(ty.clone())).is_some()) {
        ctx.unported("loops over async values");
    }
    let pattern_type = loop_pattern_type(&iter_type);
    let (prov, prov_region, prov_region_tag) = bind_prov_info(iter, ctx);
    register_pattern_bindings(pattern, &pattern_type, &BindingKind { is_immovable: false, prov, prov_region, prov_region_tag, has_responsibility: true }, ctx);
    let body_result = lower_block(body, ctx);
    let (entry, backedge) = match &node.invariant_opt {
        Some(invariant) => {
            let entry = loop_invariant_check(invariant, ctx);
            let backedge = loop_invariant_check(invariant, ctx);
            (Some(entry), Some(backedge))
        }
        None => (None, None),
    };
    let loop_pattern = Some(lower_ir_pattern(pattern, ctx));
    ctx.pop_scope();
    let loop_ir = Ir::Loop {
        kind: IrLoopKind::Iter,
        pattern: loop_pattern,
        iter_ir: Some(iter_result.ir),
        iter_value: Some(iter_result.value),
        cond_ir: None,
        cond_value: None,
        body_ir: Some(body_result.ir),
        invariant_entry_ir: entry,
        invariant_backedge_ir: backedge,
        body_value: body_result.value,
        result: IrValue::default(),
    };
    finish_loop(expr, loop_ir, "for", ctx)
}
