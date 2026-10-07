//! Lowering: proc.

use super::*;
use uv_analysis::composite::classes::type_implements_class;
use uv_analysis::composite::record_methods::lookup_method_static;
use uv_analysis::typing::types::type_key_of;

pub(super) fn lower_proc(decl: &ProcedureDecl, module_path: &[String], symbol: String, ctx: &mut LowerCtx) -> ProcIr {
    let mut ir = ProcIr { symbol: symbol.clone(), defining_module_path: module_path.to_vec(), ..Default::default() };
    ctx.module_path = module_path.to_vec();
    ctx.current_proc_symbol = Some(symbol);
    ctx.proc_ret_type = None;
    ctx.pending = None;
    // `inline` and `cold` set what the emitter reads; any other attribute changes the symbol,
    // the ABI or the checks and is not ported.
    ctx.dynamic_checks = false;
    for attr in &decl.attrs {
        match attr.name.full_name.as_str() {
            "inline" => ir.inline_mode = inline_mode_for(attr),
            "cold" => ir.cold = true,
            "dynamic" => ctx.dynamic_checks = true,
            // These change the symbol, the signature or the unwinding, and are read below.
            "export" | "host_export" | "mangle" | "unwind" => {}
            other => ctx.unported(&format!("attribute {other} on procedures")),
        }
    }
    let is_export_proc = has_attr(&decl.attrs, "export");
    // A contract is checked as the procedure runs only under `#dynamic`.
    if decl.contract.is_some() && ctx.dynamic_checks {
        ctx.unported("dynamic contract checks");
    }
    ctx.expr_prov = compute_expr_provenance_map(&ctx.scope, module_path, &decl.params, &decl.body, None);
    ctx.push_scope();
    for param in &decl.params {
        let mut p = IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), ..Default::default() };
        if let Some(written) = &param.r#type {
            if let Ok(ty) = lower_type(&ctx.scope, &Some(written.clone())) {
                p.ty = ty;
            }
        }
        p.stable_name = ctx.register_var(&param.name, p.ty.clone(), param.mode.is_some(), ProvenanceKind::Param, None, None);
        ir.params.push(p);
    }
    match &decl.return_type_opt {
        Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
            Ok(ty) if ty.is_some() => ir.ret = ty,
            _ => ctx.unported("a return type that does not lower"),
        },
        None => ctx.unported("procedures without a return type"),
    }
    ctx.proc_ret_type = ir.ret.clone();
    // An exported procedure is called from outside: its parameters are passed by value, it
    // keeps the signature it declares and takes no panic out-parameter.
    if is_export_proc {
        for param in &mut ir.params {
            param.mode = Some(ParamMode::Move);
        }
        ir.abi = attr_value(&decl.attrs, "export").map(|token| normalize_attr_literal(&token.lexeme));
        let symbol = ir.symbol.clone();
        ctx.export_unwind_modes.insert(symbol, export_unwind_catches(&decl.attrs));
    } else if needs_panic_out_for_symbol(&ir.symbol, ctx) {
        ir.params.push(panic_out_param());
    }
    let Some(body) = &decl.body else {
        ctx.unported("procedures without a body");
        ctx.pop_scope();
        return ir;
    };
    let body_res = lower_block(body, ctx);
    let scope_enter_ir = empty_ir();
    let plan = cleanup_plan_current_scope(ctx);
    let cleanup_ir = emit_cleanup(&plan, false, ctx);
    ctx.pop_scope();
    let mut body_seq: Vec<Option<IrPtr>> = vec![Some(scope_enter_ir), Some(body_res.ir.clone())];
    let may_fall_through = ir_flow_may_fall_through(&Some(body_res.ir.clone()));
    if may_fall_through {
        body_seq.push(Some(cleanup_ir));
    }
    if body.tail_opt.is_some() || (may_fall_through && is_unit_type(&ir.ret)) {
        body_seq.push(Some(Arc::new(Ir::Return { value: body_res.value })));
    }
    ir.body = Some(seq_ir(body_seq));
    // `FinalizeProcIR`: the copy elision analysis and the async state machine are
    // not ported; a procedure that returns an aggregate or an async type is pending.
    if async_sig_of(&ctx.scope, &ir.ret).is_some() {
        ctx.unported("async procedures");
    }
    // `AnalyzeAggregateCopyElision` looks into procedures that return an aggregate copied bit
    // by bit, and gives up at once on any other. Its analysis is not ported.
    let stripped = strip_perm(&ir.ret).or_else(|| ir.ret.clone());
    let aggregate = matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Array { .. } | TypeNode::Tuple(_) | TypeNode::Union(_) | TypeNode::Path { .. } | TypeNode::Apply { .. } | TypeNode::ModalState(_)));
    if ir.abi.is_none() && aggregate && bitcopy_type(&ctx.scope, &ir.ret) {
        ctx.unported("aggregate copy elision of returns");
    }
    ctx.dynamic_checks = false;
    ctx.current_proc_symbol = None;
    ir
}

/// `SubstSelfType` of the lowering: `Self` replaced by the type of the record, through the
/// forms the reference walks.
pub(super) fn subst_self(self_type: &TypeRef, ty: &TypeRef) -> TypeRef {
    let (Some(_), Some(node)) = (self_type, ty.as_deref()) else {
        return ty.clone();
    };
    let sub = |inner: &TypeRef| subst_self(self_type, inner);
    match &node.node {
        TypeNode::Path { path, .. } if is_self_var_path(path) => self_type.clone(),
        TypeNode::Perm { perm, base } => make_type_perm(*perm, sub(base)),
        TypeNode::Tuple(elements) => make_type_tuple(elements.iter().map(sub).collect()),
        TypeNode::Array { element, length, .. } => make_type_array(sub(element), *length, None),
        TypeNode::Slice(element) => make_type_slice(sub(element)),
        TypeNode::Union(members) => make_type_union(members.iter().map(sub).collect()),
        TypeNode::Func { params, ret } => make_type_func(params.iter().map(|param| uv_analysis::typing::types::TypeFuncParam { mode: param.mode, r#type: sub(&param.r#type) }).collect(), sub(ret)),
        TypeNode::Closure { params, ret, deps_opt } => make_type_closure(
            params.iter().map(|(is_move, ty)| (*is_move, sub(ty))).collect(),
            sub(ret),
            deps_opt.as_ref().map(|deps| deps.iter().map(|dep| uv_analysis::typing::types::SharedDep { name: dep.name.clone(), r#type: sub(&dep.r#type) }).collect()),
        ),
        TypeNode::Ptr { element, state } => make_type_ptr(sub(element), *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, sub(element)),
        TypeNode::Refine { base, predicate } => make_type_refine(sub(base), predicate.clone()),
        _ => ty.clone(),
    }
}

/// `LowerParam`: the parameter as the IR declares it.
pub(super) fn lower_param(param: &ast::Param, self_type: &TypeRef, ctx: &LowerCtx) -> IrParam {
    let mut out = IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), stable_name: param.name.clone(), ty: None };
    if param.r#type.is_some() {
        if let Ok(ty) = lower_type(&ctx.scope, &param.r#type) {
            out.ty = subst_self(self_type, &ty);
        }
    }
    out
}

/// `LowerProcLike`: a procedure made of parts: a method or a transition. It reads no
/// attributes and has no contract, and the expressions have no provenance map.
pub(super) fn lower_proc_like(symbol: &str, params: &[IrParam], ret_type: &TypeRef, body: &ast::Block, module_path: &[String], ctx: &mut LowerCtx) -> ProcIr {
    let mut ir = ProcIr { symbol: symbol.to_string(), ..Default::default() };
    ctx.module_path = module_path.to_vec();
    ctx.current_proc_symbol = Some(symbol.to_string());
    ctx.expr_prov = None;
    ctx.push_scope();
    for param in params {
        let mut lowered = param.clone();
        lowered.stable_name = ctx.register_var(&param.name, param.ty.clone(), param.mode.is_some(), ProvenanceKind::Bottom, None, None);
        ir.params.push(lowered);
    }
    ir.ret = if ret_type.is_some() { ret_type.clone() } else { make_type_prim("()") };
    ctx.proc_ret_type = ir.ret.clone();
    ir.params.push(panic_out_param());
    let body_res = lower_block(body, ctx);
    let scope_enter_ir = empty_ir();
    let plan = cleanup_plan_current_scope(ctx);
    let cleanup_ir = emit_cleanup(&plan, false, ctx);
    ctx.pop_scope();
    let mut body_seq: Vec<Option<IrPtr>> = vec![Some(scope_enter_ir), Some(body_res.ir.clone())];
    let may_fall_through = ir_flow_may_fall_through(&Some(body_res.ir.clone()));
    if may_fall_through {
        body_seq.push(Some(cleanup_ir));
    }
    if body.tail_opt.is_some() || (may_fall_through && is_unit_type(&ir.ret)) {
        body_seq.push(Some(Arc::new(Ir::Return { value: body_res.value })));
    }
    ir.body = Some(seq_ir(body_seq));
    ctx.current_proc_symbol = None;
    ir
}

/// `LowerRecordMethod`.
pub(super) fn lower_record_method(record: &ast::RecordDecl, method: &ast::MethodDecl, module_path: &[String], ctx: &mut LowerCtx) -> Option<ProcIr> {
    let Some(body) = &method.body else {
        ctx.unported("methods without a body");
        return None;
    };
    let prev_dynamic_checks = ctx.dynamic_checks;
    ctx.dynamic_checks = method.attrs.iter().chain(&record.attrs).any(|attr| attr.name.full_name == "dynamic");
    let mut record_path = module_path.to_vec();
    record_path.push(record.name.clone());
    let self_type = make_type_path(record_path.clone());
    let recv_type = recv_type_for_receiver(&self_type, &method.receiver, |written| match lower_type_for_layout(&ctx.scope, written).flatten() {
        Some(ty) => Ok(Some(ty)),
        None => Err(None),
    });
    let recv_mode = recv_mode_of(&method.receiver);
    let mut params = vec![IrParam { mode: recv_mode, name: "self".to_string(), stable_name: "self".to_string(), ty: recv_type.ok().filter(Option::is_some).unwrap_or_else(|| self_type.clone()) }];
    for param in &method.params {
        params.push(lower_param(param, &self_type, ctx));
    }
    let ret_type = match &method.return_type_opt {
        None => make_type_prim("()"),
        Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
            Ok(ty) if ty.is_some() => subst_self(&self_type, &ty),
            _ => None,
        },
    };
    let symbol = scoped_sym(&item_path_proc(&record_path, &method.name));
    let mut proc = lower_proc_like(&symbol, &params, &ret_type, body, module_path, ctx);
    ctx.dynamic_checks = prev_dynamic_checks;
    // `ApplyProcAttrs`.
    for attr in &method.attrs {
        match attr.name.full_name.as_str() {
            "inline" => proc.inline_mode = inline_mode_for(attr),
            "cold" => proc.cold = true,
            "dynamic" => {}
            other => ctx.unported(&format!("attribute {other} on methods")),
        }
    }
    Some(proc)
}

// ----------------------------------------------------------------------------- extern

/// `NormalizeExternAbi` of the ABI an extern block names.
fn extern_abi_name(abi: &Option<ast::ExternAbi>) -> String {
    match abi {
        None => "C".to_string(),
        Some(ast::ExternAbi::ExternAbiString(abi)) => normalize_attr_literal(&abi.literal.lexeme),
        Some(ast::ExternAbi::ExternAbiIdent(abi)) => abi.name.clone(),
    }
}

/// `ExternAbiUsesRawName`: the C ABIs use the name as written.
pub(super) fn extern_abi_uses_raw_name(abi: &Option<ast::ExternAbi>) -> bool {
    matches!(extern_abi_name(abi).as_str(), "C" | "C-unwind")
}

/// Whether a `library` attribute asks for a raw dynamic library, which is not ported.
fn names_raw_dylib(attrs: &[ast::AttributeItem]) -> bool {
    attrs.iter().filter(|attr| attr.name.full_name == "library").any(|attr| {
        attr.args.iter().any(|arg| arg.key.as_deref() == Some("kind") && matches!(&arg.value, ast::AttributeArgValue::Token(token) if normalize_attr_literal(&token.lexeme) == "raw-dylib"))
    })
}

/// `LowerModule` of an extern block: the declaration of each foreign procedure, and its
/// signature for the calls that follow.
pub(super) fn lower_extern_block(block: &ast::ExternBlock, module_path: &[String], ctx: &mut LowerCtx) -> IrDecls {
    let block_attrs = block.attrs_opt.clone().unwrap_or_default();
    if names_raw_dylib(&block_attrs) {
        ctx.unported("extern blocks of raw dynamic libraries");
    }
    let mut decls = Vec::new();
    for item in &block.items {
        let ast::ExternItem::ExternProcDecl(proc) = item;
        if proc.foreign_contracts_opt.as_ref().is_some_and(|clauses| !clauses.is_empty()) {
            ctx.unported("foreign contracts");
        }
        let symbol = match link_name(&proc.attrs, &proc.name) {
            Some(name) => name,
            None if extern_abi_uses_raw_name(&block.abi_opt) => proc.name.clone(),
            None => scoped_sym(&item_path_proc(module_path, &proc.name)),
        };
        let self_type: TypeRef = None;
        let mut params: Vec<IrParam> = proc.params.iter().map(|param| lower_param(param, &self_type, ctx)).collect();
        // The visible parameters of a foreign declaration are passed by value.
        for param in &mut params {
            param.mode = Some(ParamMode::Move);
        }
        let ret = match &proc.return_type_opt {
            None => make_type_prim("()"),
            Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
                Ok(ty) if ty.is_some() => ty,
                _ => None,
            },
        };
        let abi = Some(extern_abi_name(&block.abi_opt));
        ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: params.clone(), ret: ret.clone() });
        ctx.ffi_imports.insert(symbol.clone());
        decls.push(IrDecl::ExternProc(ExternProcIr { symbol, params, ret, abi, ..Default::default() }));
    }
    decls
}

// ------------------------------------------------------------------- class defaults

/// `PathOfType`: the path a type is named by in a symbol.
fn path_of_type(ty: &TypeRef) -> Vec<String> {
    let state_name = |view: bool| if view { "view" } else { "managed" };
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Prim(name)) => vec!["prim".to_string(), name.clone()],
        Some(TypeNode::String(state)) => vec!["string".to_string(), state.map_or("modal", |state| state_name(state == uv_analysis::typing::types::StringState::View)).to_string()],
        Some(TypeNode::Bytes(state)) => vec!["bytes".to_string(), state.map_or("modal", |state| state_name(state == uv_analysis::typing::types::BytesState::View)).to_string()],
        Some(TypeNode::Path { path, .. }) => path.clone(),
        Some(TypeNode::ModalState(modal)) => {
            let mut path = modal.path.clone();
            path.push(modal.state.clone());
            path
        }
        _ => Vec::new(),
    }
}

/// `MangleDefaultImpl`: the symbol of a class method's default body for one implementing type.
fn mangle_default_impl(ty: &TypeRef, class_path: &[String], method_name: &str) -> String {
    let mut path = vec!["default".to_string()];
    path.extend(path_of_type(ty));
    path.push("cl".to_string());
    path.extend(class_path.iter().cloned());
    path.push(method_name.to_string());
    scoped_sym(&path)
}

/// `DefaultUserList`: the types, in their canonical order, that take a class method's default body.
fn default_user_list(scope: &ScopeContext<'_>, class_path: &[String], method: &ast::ClassMethodDecl) -> Vec<TypeRef> {
    let mut types: Vec<TypeRef> = Vec::new();
    for (key, decl) in &scope.sigma.types {
        if !matches!(decl, uv_analysis::context::TypeDecl::Record(_) | uv_analysis::context::TypeDecl::Enum(_) | uv_analysis::context::TypeDecl::Modal(_)) {
            continue;
        }
        let ty = make_type_path(key.iter().map(ToString::to_string).collect());
        if type_implements_class(scope, &ty, class_path) {
            types.push(ty);
        }
    }
    types.sort_by_key(type_key_of);
    types
        .into_iter()
        .filter(|ty| lookup_method_static(scope, ty, &method.name).is_ok_and(|lookup| lookup.class_method.is_some() && lookup.owner_class == class_path))
        .collect()
}

/// `LowerClassMethodBody`: the default body of a method, once for each type that uses it.
pub(super) fn lower_class_method_body(class: &ast::ClassDecl, method: &ast::ClassMethodDecl, module_path: &[String], ctx: &mut LowerCtx) -> Vec<(String, ProcIr)> {
    let Some(body) = &method.body_opt else {
        return Vec::new();
    };
    let mut class_path = module_path.to_vec();
    class_path.push(class.name.clone());
    let prev_dynamic_checks = ctx.dynamic_checks;
    ctx.dynamic_checks = has_attr(&method.attrs, "dynamic") || has_attr(&class.attrs, "dynamic");
    let scope = layout_scope(ctx, module_path);
    let mut procs = Vec::new();
    for self_type in default_user_list(&scope, &class_path, method) {
        let recv_type = recv_type_for_receiver(&self_type, &method.receiver, |written| match lower_type_for_layout(&ctx.scope, written).flatten() {
            Some(ty) => Ok(Some(ty)),
            None => Err(None),
        });
        let mut params = vec![IrParam { mode: recv_mode_of(&method.receiver), name: "self".to_string(), stable_name: "self".to_string(), ty: recv_type.ok().filter(Option::is_some).unwrap_or_else(|| self_type.clone()) }];
        for param in &method.params {
            params.push(lower_param(param, &self_type, ctx));
        }
        let ret_type = match &method.return_type_opt {
            None => make_type_prim("()"),
            Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
                Ok(ty) if ty.is_some() => subst_self(&self_type, &ty),
                _ => None,
            },
        };
        let symbol = mangle_default_impl(&self_type, &class_path, &method.name);
        let mut proc = lower_proc_like(&symbol, &params, &ret_type, body, module_path, ctx);
        for attr in &method.attrs {
            match attr.name.full_name.as_str() {
                "inline" => proc.inline_mode = inline_mode_for(attr),
                "cold" => proc.cold = true,
                "dynamic" => {}
                other => ctx.unported(&format!("attribute {other} on methods")),
            }
        }
        procs.push((symbol, proc));
    }
    ctx.dynamic_checks = prev_dynamic_checks;
    procs
}

// -------------------------------------------------------------------------- modals

fn apply_method_attrs(attrs: &[ast::AttributeItem], proc: &mut ProcIr, ctx: &mut LowerCtx) {
    for attr in attrs {
        match attr.name.full_name.as_str() {
            "inline" => proc.inline_mode = inline_mode_for(attr),
            "cold" => proc.cold = true,
            "dynamic" => {}
            other => ctx.unported(&format!("attribute {other} on methods")),
        }
    }
}

/// `LowerStateMethod`: a method of one state of a modal type, which takes `self` in that state.
pub(super) fn lower_state_method(modal: &ast::ModalDecl, state: &ast::StateBlock, method: &ast::StateMethodDecl, module_path: &[String], ctx: &mut LowerCtx) -> Option<ProcIr> {
    let Some(body) = &method.body else {
        ctx.unported("state methods without a body");
        return None;
    };
    let mut modal_path = module_path.to_vec();
    modal_path.push(modal.name.clone());
    let state_type = make_type_modal_state(modal_path.clone(), &state.name, Vec::new());
    let recv_type = recv_type_for_receiver(&state_type, &method.receiver, |written| match lower_type_for_layout(&ctx.scope, written).flatten() {
        Some(ty) => Ok(Some(ty)),
        None => Err(None),
    });
    let self_type = recv_type.ok().filter(Option::is_some).unwrap_or_else(|| make_type_perm(Permission::Const, state_type.clone()));
    let mut params = vec![IrParam { mode: recv_mode_of(&method.receiver), name: "self".to_string(), stable_name: "self".to_string(), ty: self_type }];
    for param in &method.params {
        params.push(lower_param(param, &state_type, ctx));
    }
    let ret_type = match &method.return_type_opt {
        None => make_type_prim("()"),
        Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
            Ok(ty) if ty.is_some() => subst_self(&state_type, &ty),
            _ => None,
        },
    };
    let mut symbol_path = modal_path;
    symbol_path.push(state.name.clone());
    symbol_path.push(method.name.clone());
    let symbol = scoped_sym(&symbol_path);
    let prev_dynamic_checks = ctx.dynamic_checks;
    ctx.dynamic_checks = has_attr(&method.attrs, "dynamic") || has_attr(&modal.attrs, "dynamic");
    let mut proc = lower_proc_like(&symbol, &params, &ret_type, body, module_path, ctx);
    ctx.dynamic_checks = prev_dynamic_checks;
    apply_method_attrs(&method.attrs, &mut proc, ctx);
    Some(proc)
}

/// `LowerTransition`: the change of a modal value from one state to another.
pub(super) fn lower_transition(modal: &ast::ModalDecl, state: &ast::StateBlock, trans: &ast::TransitionDecl, module_path: &[String], ctx: &mut LowerCtx) -> Option<ProcIr> {
    let Some(body) = &trans.body else {
        ctx.unported("transitions without a body");
        return None;
    };
    let mut modal_path = module_path.to_vec();
    modal_path.push(modal.name.clone());
    let state_type = make_type_modal_state(modal_path.clone(), &state.name, Vec::new());
    let recv_type = make_type_perm(Permission::Unique, state_type);
    let none: TypeRef = None;
    let mut params = vec![IrParam { mode: Some(ParamMode::Move), name: "self".to_string(), stable_name: "self".to_string(), ty: recv_type }];
    for param in &trans.params {
        params.push(lower_param(param, &none, ctx));
    }
    let ret_type = make_type_modal_state(modal_path.clone(), &trans.target_state, Vec::new());
    let mut symbol_path = modal_path;
    symbol_path.push(state.name.clone());
    symbol_path.push(trans.name.clone());
    let symbol = scoped_sym(&symbol_path);
    let prev_dynamic_checks = ctx.dynamic_checks;
    ctx.dynamic_checks = has_attr(&trans.attrs, "dynamic") || has_attr(&modal.attrs, "dynamic");
    let mut proc = lower_proc_like(&symbol, &params, &ret_type, body, module_path, ctx);
    ctx.dynamic_checks = prev_dynamic_checks;
    apply_method_attrs(&trans.attrs, &mut proc, ctx);
    Some(proc)
}
