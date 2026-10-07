//! Lowering: places. The address of a place, the read of an element, and the symbols an
//! IR fragment refers to.

use std::collections::BTreeSet;

use uv_project::language_profile::active_language_profile;

use super::*;

/// `PoisonFlagSymForModule`: the flag that says the initialisation of a module failed.
fn poison_flag_sym_for_module(module: &str) -> String {
    let mut full = vec![active_language_profile().runtime_root.to_string(), "runtime".to_string(), "poison".to_string()];
    full.extend(module.split("::").filter(|segment| !segment.is_empty()).map(str::to_string));
    mangle(&string_of_path(&full))
}

fn collect_ref_syms(out: &mut BTreeSet<String>, ir: &Option<IrPtr>) {
    let Some(ir) = ir else {
        return;
    };
    let add_value = |out: &mut BTreeSet<String>, value: &IrValue| {
        if value.kind == IrValueKind::Symbol && !value.name.is_empty() {
            out.insert(value.name.clone());
        }
    };
    match ir.as_ref() {
        Ir::Seq { items } => {
            for item in items {
                collect_ref_syms(out, &Some(item.clone()));
            }
        }
        Ir::Call { callee, .. } => add_value(out, callee),
        Ir::StoreGlobal { symbol, .. } => {
            out.insert(symbol.clone());
        }
        Ir::AddrOf { ref_syms, .. } => out.extend(ref_syms.iter().cloned()),
        Ir::ReadPath { path, name } => {
            let mut full = path.clone();
            full.push(name.clone());
            out.insert(mangle(&string_of_path(&full)));
            out.insert(scoped_sym(&item_path_proc(path, name)));
        }
        Ir::If { then_ir, else_ir, .. } => {
            collect_ref_syms(out, then_ir);
            collect_ref_syms(out, else_ir);
        }
        Ir::Block { setup, body, .. } => {
            collect_ref_syms(out, setup);
            collect_ref_syms(out, body);
        }
        Ir::CleanupPanicCheck { cleanup_ir } | Ir::LowerPanic { cleanup_ir, .. } => collect_ref_syms(out, cleanup_ir),
        Ir::InitPanicHandle { poison_modules, cleanup_ir, .. } | Ir::InitPanicRaise { poison_modules, cleanup_ir, .. } => {
            out.extend(poison_modules.iter().map(|module| poison_flag_sym_for_module(module)));
            collect_ref_syms(out, cleanup_ir);
        }
        Ir::CheckPoison { module } => {
            out.insert(poison_flag_sym_for_module(module));
        }
        // The other nodes name a symbol only when the construct that makes them is lowered.
        _ => {}
    }
}

/// `RefSyms`: the symbols an IR fragment refers to, sorted and without repeats.
pub(super) fn ref_syms(parts: &[Option<IrPtr>]) -> Vec<String> {
    let mut out = BTreeSet::new();
    for part in parts {
        collect_ref_syms(&mut out, part);
    }
    out.into_iter().collect()
}

/// `CheckPoison`: before a static of another module is used, whether its initialisation failed.
pub(super) fn check_poison_ir(module: &str, ctx: &mut LowerCtx) -> IrPtr {
    let trace_ir = emit_runtime_trace(ctx);
    seq_ir(vec![Some(trace_ir), Some(Arc::new(Ir::CheckPoison { module: module.to_string() }))])
}

/// `LowerAddrOf` for the length of a call (`TransientNoEscape`): the address of a place is
/// taken, and no tag is stamped on it.
pub(super) fn lower_addr_of(place: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    // `register_ptr_type`: a pointer to what the place holds.
    let register_ptr_type = |ctx: &mut LowerCtx, ptr: &IrValue| {
        let mut pointee: TypeRef = None;
        if let ExprNode::IdentifierExpr(ident) = &place.node {
            if let Some(state) = ctx.binding_state(&ident.name) {
                pointee = state.ty.clone();
            }
        }
        if pointee.is_none() {
            pointee = stored_expr_type(&ctx.scope, &Some(place.clone())).flatten();
        }
        if pointee.is_some() {
            ctx.register_value_type(ptr, make_type_ptr(pointee, Some(PtrState::Valid)));
        }
    };
    let place_repr = |place: &Arc<Expr>| IrPlace { repr: build_place_repr(place) };
    match &place.node {
        ExprNode::IdentifierExpr(ident) => {
            let ptr_value = ctx.fresh_temp_value("addr_of");
            register_ptr_type(ctx, &ptr_value);
            if ctx.binding_state(&ident.name).is_some() {
                let addr = Arc::new(Ir::AddrOf { place: place_repr(place), result: ptr_value.clone(), ref_syms: Vec::new() });
                return LowerResult { ir: seq_ir(vec![Some(addr)]), value: ptr_value };
            }
            // A static: the module that holds it must not be poisoned.
            let Some(mut full) = resolve_value_path(&ident.name, ctx) else {
                ctx.unported("addresses of names that do not resolve to a path");
                return LowerResult { ir: empty_ir(), value: ptr_value };
            };
            full.pop();
            let poison_ir = check_poison_ir(&full.join("::"), ctx);
            let followup = panic_check(ctx);
            let ref_syms = ref_syms(&[Some(poison_ir.clone()), Some(followup.clone())]);
            let addr = Arc::new(Ir::AddrOf { place: place_repr(place), result: ptr_value.clone(), ref_syms });
            LowerResult { ir: seq_ir(vec![Some(poison_ir), Some(followup), Some(addr)]), value: ptr_value }
        }
        ExprNode::FieldAccessExpr(node) => {
            let Some(base) = &node.base else {
                ctx.unported("field addresses without a base");
                return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("addr_of") };
            };
            let base_result = lower_addr_of(base, ctx);
            let ptr_value = ctx.fresh_temp_value("addr_of");
            register_ptr_type(ctx, &ptr_value);
            let mut info = DerivedValueInfo::new(DerivedKind::AddrField);
            info.base = base_result.value.clone();
            info.field = node.name.clone();
            ctx.register_derived_value(&ptr_value, info);
            let tag_ir = empty_ir();
            let ref_syms = ref_syms(&[Some(base_result.ir.clone()), Some(tag_ir.clone())]);
            let addr = Arc::new(Ir::AddrOf { place: place_repr(place), result: ptr_value.clone(), ref_syms });
            LowerResult { ir: seq_ir(vec![Some(base_result.ir), Some(addr), Some(tag_ir)]), value: ptr_value }
        }
        ExprNode::TupleAccessExpr(node) => {
            let Some(base) = &node.base else {
                ctx.unported("tuple addresses without a base");
                return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("addr_of") };
            };
            let base_result = lower_addr_of(base, ctx);
            let ptr_value = ctx.fresh_temp_value("addr_of");
            register_ptr_type(ctx, &ptr_value);
            let mut info = DerivedValueInfo::new(DerivedKind::AddrTuple);
            info.base = base_result.value.clone();
            info.tuple_index = usize::try_from(node.index).unwrap_or(usize::MAX);
            ctx.register_derived_value(&ptr_value, info);
            let tag_ir = empty_ir();
            let ref_syms = ref_syms(&[Some(base_result.ir.clone()), Some(tag_ir.clone())]);
            let addr = Arc::new(Ir::AddrOf { place: place_repr(place), result: ptr_value.clone(), ref_syms });
            LowerResult { ir: seq_ir(vec![Some(base_result.ir), Some(addr), Some(tag_ir)]), value: ptr_value }
        }
        ExprNode::IndexAccessExpr(node) => {
            let (Some(base), Some(index)) = (&node.base, &node.index) else {
                ctx.unported("index addresses without a base or an index");
                return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("addr_of") };
            };
            let base_result = lower_addr_of(base, ctx);
            let ptr_value = ctx.fresh_temp_value("addr_of");
            register_ptr_type(ctx, &ptr_value);
            if matches!(index.node, ExprNode::RangeExpr(_)) || is_range_index_expr(index, ctx) {
                ctx.unported("addresses of ranges of elements");
                return LowerResult { ir: empty_ir(), value: ptr_value };
            }
            let index_result = lower_expr(index, ctx);
            let needs_check = needs_index_check(base, ctx);
            let mut info = DerivedValueInfo::new(DerivedKind::AddrIndex);
            info.base = base_result.value.clone();
            info.index = index_result.value.clone();
            ctx.register_derived_value(&ptr_value, info);
            let tag_ir = empty_ir();
            let mut seq = vec![Some(base_result.ir), Some(index_result.ir)];
            if needs_check {
                seq.push(Some(Arc::new(Ir::CheckIndex { base: base_result.value, index: index_result.value })));
                seq.push(Some(panic_check(ctx)));
            }
            let mut prereq = seq.clone();
            prereq.push(Some(tag_ir.clone()));
            let ref_syms = ref_syms(&prereq);
            seq.push(Some(Arc::new(Ir::AddrOf { place: place_repr(place), result: ptr_value.clone(), ref_syms })));
            seq.push(Some(tag_ir));
            LowerResult { ir: seq_ir(seq), value: ptr_value }
        }
        other => {
            ctx.unported(&format!("addresses of {}", variant_name(other)));
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("addr_of") }
        }
    }
}

/// `IsRangeIndexExpr`: an index whose type is a range that can index.
fn is_range_index_expr(index: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    matches!(stored_expr_type(&ctx.scope, &Some(index.clone())).flatten().as_deref().map(|ty| &ty.node), Some(TypeNode::Range(_) | TypeNode::RangeInclusive(_) | TypeNode::RangeFrom(_) | TypeNode::RangeTo(_) | TypeNode::RangeToInclusive(_) | TypeNode::RangeFull))
}

/// `NeedsIndexCheck`: an array is checked only when the procedure checks as it runs.
fn needs_index_check(base: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    let base_type = stored_expr_type(&ctx.scope, &Some(base.clone())).flatten();
    let stripped = strip_perm(&base_type);
    if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Array { .. })) {
        return ctx.dynamic_checks;
    }
    true
}

/// `IndexedElementType`.
fn indexed_element_type(access: &Arc<Expr>, base: &Arc<Expr>, ctx: &LowerCtx) -> TypeRef {
    if let Some(ty) = stored_expr_type(&ctx.scope, &Some(access.clone())).flatten() {
        return Some(ty);
    }
    let mut base_type = stored_expr_type(&ctx.scope, &Some(base.clone())).flatten();
    let mut perm = None;
    for _ in 0..8 {
        match base_type.as_deref().map(|ty| &ty.node) {
            Some(TypeNode::Perm { perm: found, base }) => {
                perm = Some(*found);
                base_type = base.clone();
            }
            Some(TypeNode::Refine { base, .. }) => base_type = base.clone(),
            _ => break,
        }
    }
    let element = match base_type.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Array { element, .. }) | Some(TypeNode::Slice(element)) => element.clone(),
        _ => None,
    };
    match (element, perm) {
        (Some(element), Some(perm)) => make_type_perm(perm, Some(element)),
        (element, _) => element,
    }
}

/// `LowerIndexAccess` of one element. The key is taken by the access expression the reference
/// builds as a copy, which typing knows nothing of, so no key is ever taken here.
pub(super) fn lower_index_access(access: &Arc<Expr>, node: &ast::IndexAccessExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(base), Some(index)) = (&node.base, &node.index) else {
        ctx.unported("index accesses without a base or an index");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("index_elem") };
    };
    let key_ir = empty_ir();
    if matches!(index.node, ExprNode::RangeExpr(_)) || is_range_index_expr(index, ctx) {
        ctx.unported("slices");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("slice") };
    }
    if is_place_expr(&Some(base.clone())) {
        let base_addr = lower_addr_of(base, ctx);
        let index_result = lower_expr(index, ctx);
        let needs_check = needs_index_check(base, ctx);
        let elem_ptr = ctx.fresh_temp_value("index_elem_addr");
        let elem_type = indexed_element_type(access, base, ctx);
        if elem_type.is_some() {
            ctx.register_value_type(&elem_ptr, make_type_ptr(elem_type.clone(), Some(PtrState::Valid)));
        }
        let mut addr_info = DerivedValueInfo::new(DerivedKind::AddrIndex);
        addr_info.base = base_addr.value.clone();
        addr_info.index = index_result.value.clone();
        ctx.register_derived_value(&elem_ptr, addr_info);
        let elem_value = ctx.fresh_temp_value("index_elem");
        if elem_type.is_some() {
            ctx.register_value_type(&elem_value, elem_type);
        }
        let mut load_info = DerivedValueInfo::new(DerivedKind::LoadFromAddr);
        load_info.base = elem_ptr;
        ctx.register_derived_value(&elem_value, load_info);
        let mut seq = vec![Some(base_addr.ir), Some(index_result.ir.clone()), Some(key_ir)];
        if needs_check {
            seq.push(Some(Arc::new(Ir::CheckIndex { base: base_addr.value, index: index_result.value })));
            seq.push(Some(panic_check(ctx)));
        }
        return LowerResult { ir: seq_ir(seq), value: elem_value };
    }
    let base_result = lower_expr(base, ctx);
    let index_result = lower_expr(index, ctx);
    let needs_check = needs_index_check(base, ctx);
    let elem_value = ctx.fresh_temp_value("index_elem");
    let mut info = DerivedValueInfo::new(DerivedKind::Index);
    info.base = base_result.value.clone();
    info.index = index_result.value.clone();
    ctx.register_derived_value(&elem_value, info);
    let mut seq = vec![Some(base_result.ir), Some(index_result.ir), Some(key_ir)];
    if needs_check {
        seq.push(Some(Arc::new(Ir::CheckIndex { base: base_result.value, index: index_result.value })));
        seq.push(Some(panic_check(ctx)));
    }
    LowerResult { ir: seq_ir(seq), value: elem_value }
}
