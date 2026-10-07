//! Lowering: call.

use super::*;

/// `MangleProcInModule`: the symbol of a procedure; procedures of one name in a module get
/// the index of the declaration as a suffix.
pub(super) fn mangle_proc_in_module(module: &ASTModule, proc: &ProcedureDecl) -> String {
    let participates = |decl: &ProcedureDecl| decl.name != "main" && !has_attr(&decl.attrs, "host_export") && link_name(&decl.attrs, &decl.name).is_none();
    let base = item_path_proc(&module.path, &proc.name);
    // `MangleProc`: a hosted export has the symbol of its body, a `mangle` attribute names the symbol.
    let plain = |proc: &ProcedureDecl| {
        if has_attr(&proc.attrs, "host_export") {
            return scoped_sym(&[scoped_sym(&base), "__host_body".to_string()]);
        }
        if let Some(name) = link_name(&proc.attrs, &proc.name) {
            return name;
        }
        scoped_sym(&base)
    };
    if !participates(proc) {
        return plain(proc);
    }
    let mut overload_count = 0;
    let mut overload_index = 0;
    for item in &module.items {
        let ASTItem::ProcedureDecl(candidate) = item else {
            continue;
        };
        if candidate.name != proc.name || !participates(candidate) {
            continue;
        }
        if std::ptr::eq(candidate, proc) {
            overload_index = overload_count;
        }
        overload_count += 1;
    }
    if overload_count <= 1 {
        return plain(proc);
    }
    let mut path = base;
    path.push("$overload".to_string());
    path.push(overload_index.to_string());
    scoped_sym(&path)
}

/// What a call to a procedure of the program is lowered against: its symbol and signature.
pub(super) struct Callee {
    symbol: String,
    module_path: Vec<String>,
}

/// `PopulateSourceCallSignature` and `RegisterResolvedSourceSignatureIfMissing`.
pub(super) fn ensure_source_signature(callee: &Callee, decl: &ProcedureDecl, ctx: &mut LowerCtx) {
    if ctx.proc_sigs.contains_key(&callee.symbol) {
        return;
    }
    let scope = ctx.scope_for(&callee.module_path);
    let lower_written = |written: &Option<Arc<ast::Type>>| match written {
        Some(_) => match lower_type(&scope, written) {
            Ok(ty) if ty.is_some() => ty,
            _ => make_type_prim("()"),
        },
        None => make_type_prim("()"),
    };
    let mut sig = ProcSig { params: Vec::new(), ret: lower_written(&decl.return_type_opt) };
    for param in &decl.params {
        sig.params.push(IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), stable_name: String::new(), ty: lower_written(&param.r#type) });
    }
    if needs_panic_out_for_symbol(&callee.symbol, ctx) {
        sig.params.push(panic_out_param());
    }
    ctx.proc_sigs.insert(callee.symbol.clone(), sig);
}

/// `NeedsPanicOut` of a symbol of a user procedure: every symbol made from a module path
/// differs from the entry symbol, is not a runtime symbol and is not a record constructor.
pub(super) fn needs_panic_out_for_symbol(symbol: &str, ctx: &LowerCtx) -> bool {
    symbol != "main" && !ctx.record_ctors.contains(symbol)
}

/// `LowerRefArgExprWithTemp` and `LowerMoveArgExprWithTemp`: a value that is not a place is
/// bound to a temporary and its address is passed.
pub(super) fn lower_arg_with_temp(expr: &Arc<Expr>, prefix: &str, expected: &TypeRef, by_move: bool, ctx: &mut LowerCtx) -> LowerResult {
    if !by_move && has_source_provenance(&Some(expr.clone())) {
        return lower_addr_of(expr, ctx);
    }
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let value_result = lower_expr(expr, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let temp_name = ctx.fresh_temp_value(prefix).name;
    let mut temp_type = expected.clone();
    if temp_type.is_none() {
        temp_type = ctx.lookup_value_type(&value_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten());
    }
    if temp_type.is_none() {
        temp_type = make_type_prim("()");
    }
    // An argument bound to an Outcome parameter is introduced into it.
    if outcome_sig_of(&temp_type).is_some() {
        ctx.unported("arguments passed to an Outcome parameter");
    }
    let stable_name = ctx.register_var(&temp_name, temp_type.clone(), false, ProvenanceKind::Stack, None, None);
    let bind = Arc::new(Ir::BindVar { name: temp_name.clone(), stable_name, value: value_result.value.clone(), ty: temp_type.clone(), prov: ProvenanceKind::Stack, prov_region: None, prov_region_tag: None });
    let temp_value = IrValue { kind: IrValueKind::Local, name: temp_name.clone(), ..Default::default() };
    if !by_move {
        ctx.register_value_type(&temp_value, temp_type.clone());
        let call_like = matches!(expr.node, ExprNode::CallExpr(_) | ExprNode::MethodCallExpr(_));
        let no_cleanup = matches!(temp_type.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_)));
        let has_responsibility = !(call_like && no_cleanup) && binding_initializer_has_responsibility(expr, ctx);
        ctx.register_temp_value(&temp_value, &temp_type, has_responsibility);
    }
    let temp_ident = Arc::new(Expr { span: expr.span.clone(), node: ExprNode::IdentifierExpr(ast::IdentifierExpr { name: temp_name, from_splice: false }) });
    let addr_result = lower_addr_of(&temp_ident, ctx);
    LowerResult { ir: seq_ir(vec![Some(value_result.ir), Some(bind), Some(addr_result.ir)]), value: addr_result.value }
}

/// `LowerArgs`: the arguments left to right, each as the mode of its parameter asks.
pub(super) fn lower_args(modes: &[Option<ParamMode>], types: &[TypeRef], args: &[ast::Arg], ctx: &mut LowerCtx) -> (IrPtr, Vec<IrValue>) {
    if modes.is_empty() && args.is_empty() {
        return (empty_ir(), Vec::new());
    }
    if modes.len() != args.len() || types.len() != args.len() {
        ctx.unported("calls whose arguments do not match the parameters");
        return (empty_ir(), Vec::new());
    }
    let mut parts = Vec::new();
    let mut values = Vec::new();
    for (index, arg) in args.iter().enumerate() {
        let Some(value) = &arg.value else {
            continue;
        };
        if arg.pass != ast::ArgPassKind::Ref {
            ctx.unported("arguments passed with move or copy");
        }
        // `RequiredKeyModeForParamType`: the key the parameter's permission asks of the argument.
        let key_ir = match types[index].as_ref().map(|_| perm_of_type(&types[index])) {
            Some(Permission::Unique) => lower_implicit_key_access(value, ast::KeyMode::Write, ctx),
            Some(Permission::Shared | Permission::Const) => lower_implicit_key_access(value, ast::KeyMode::Read, ctx),
            None => empty_ir(),
        };
        let result = if modes[index].is_some() {
            lower_arg_with_temp(value, "call_move_tmp", &types[index], true, ctx)
        } else {
            lower_arg_with_temp(value, "call_ref_tmp", &types[index], false, ctx)
        };
        parts.push(Some(seq_ir(vec![Some(key_ir), Some(result.ir)])));
        values.push(result.value);
    }
    (seq_ir(parts), values)
}

pub(super) fn lower_call(call: &ast::CallExpr, ctx: &mut LowerCtx) -> LowerResult {
    let failed = |ctx: &mut LowerCtx, what: &str| {
        ctx.unported(what);
        LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("call") }
    };
    if let Some(callee_expr) = &call.callee {
        let callee_type = stored_expr_type(&ctx.scope, &Some(callee_expr.clone())).flatten();
        if matches!(strip_perm(&callee_type).as_deref().map(|ty| &ty.node), Some(TypeNode::Closure { .. })) {
            return failed(ctx, "calls of closures");
        }
    }
    if !call.generic_args.is_empty() {
        return failed(ctx, "calls with type arguments");
    }
    let sigma = ctx.scope.sigma.clone();
    // The procedure the call names: the one typing selected, or the one the callee names.
    let mut callee_ir = empty_ir();
    // What the call names: a procedure of the program, or a foreign one declared by an extern block.
    let mut foreign_symbol: Option<(String, Vec<String>)> = None;
    let (module, decl) = match selected_call_target(&ctx.scope, call) {
        Some(target) => {
            if target.generic {
                return failed(ctx, "calls of generic procedures");
            }
            let found = sigma.mods.iter().find(|module| module.path == target.module_path).and_then(|module| {
                module.items.iter().find_map(|item| match item {
                    ASTItem::ProcedureDecl(decl) if decl.name == target.proc_name && decl.span == target.proc_span => Some((module, decl)),
                    _ => None,
                })
            });
            match found {
                Some(found) => found,
                None => return failed(ctx, "calls of procedures that are not in the program"),
            }
        }
        None => {
            let Some(callee_expr) = &call.callee else {
                return failed(ctx, "calls without a callee");
            };
            // `ExtractCalleePath`: the path a callee names, and `LowerExpr` of it.
            let (origin, name) = match &callee_expr.node {
                ExprNode::IdentifierExpr(ident) => {
                    if ctx.binding_state(&ident.name).is_some() {
                        return failed(ctx, "calls of local function values");
                    }
                    // `resolve_name`: the origin and the name of a value entity of the module.
                    let (origin, name) = match ctx.scope.module_scope().get(&id_key_of(&ident.name)) {
                        Some(entity) if entity.kind == EntityKind::Value => match &entity.origin_opt {
                            Some(origin) => (origin.clone(), entity.target_opt.clone().unwrap_or_else(|| ident.name.clone())),
                            None => return failed(ctx, "calls of names without an origin"),
                        },
                        _ => (ctx.module_path.clone(), ident.name.clone()),
                    };
                    (origin, name)
                }
                ExprNode::PathExpr(path) => (path.path.clone(), path.name.clone()),
                other => {
                    let kind = variant_name(other);
                    return failed(ctx, &format!("calls through {kind}"));
                }
            };
            let found = sigma.mods.iter().find(|module| module.path == origin).and_then(|module| {
                module.items.iter().find_map(|item| match item {
                    ASTItem::ProcedureDecl(decl) if id_eq(&decl.name, &name) => Some((module, decl)),
                    _ => None,
                })
            });
            let Some((module, decl)) = found else {
                // A foreign procedure: found in an extern block of the module.
                let module = sigma.mods.iter().find(|module| module.path == origin);
                let foreign = module.and_then(|module| {
                    module.items.iter().filter_map(|item| if let ASTItem::ExternBlock(block) = item { Some(block) } else { None }).find_map(|block| {
                        block.items.iter().find_map(|item| {
                            let ast::ExternItem::ExternProcDecl(proc) = item;
                            id_eq(&proc.name, &name).then(|| match link_name(&proc.attrs, &proc.name) {
                                Some(link) => link,
                                None if extern_abi_uses_raw_name(&block.abi_opt) => proc.name.clone(),
                                None => scoped_sym(&item_path_proc(&module.path, &proc.name)),
                            })
                        })
                    })
                });
                let Some(symbol) = foreign else {
                    return failed(ctx, "calls of names that are not procedures of the program");
                };
                if !ctx.proc_sigs.contains_key(&symbol) {
                    return failed(ctx, "calls of foreign procedures declared later");
                }
                callee_ir = lower_expr(callee_expr, ctx).ir;
                foreign_symbol = Some((symbol, origin));
                return lower_resolved_call(call, ctx, callee_ir, None, foreign_symbol);
            };
            if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
                return failed(ctx, "calls of generic procedures");
            }
            // `LowerExpr` of the callee: a read of the path of the procedure.
            callee_ir = lower_expr(callee_expr, ctx).ir;
            (module, decl)
        }
    };
    if decl.contract.is_some() {
        return failed(ctx, "calls of procedures with contracts");
    }
    if module.path.len() == 1 && (module.path[0].eq_ignore_ascii_case("string") || module.path[0].eq_ignore_ascii_case("bytes")) {
        return failed(ctx, "calls of built-in procedures");
    }
    let callee = Callee { symbol: mangle_proc_in_module(module, decl), module_path: module.path.clone() };
    ensure_source_signature(&callee, decl, ctx);
    lower_resolved_call(call, ctx, callee_ir, Some(callee), foreign_symbol)
}

/// The rest of a call once its callee is known: the arguments by what the signature asks,
/// the call, and what follows it.
fn lower_resolved_call(call: &ast::CallExpr, ctx: &mut LowerCtx, callee_ir: IrPtr, source: Option<Callee>, foreign: Option<(String, Vec<String>)>) -> LowerResult {
    let callee = match (source, foreign) {
        (Some(callee), _) => callee,
        (None, Some((symbol, module_path))) => Callee { symbol, module_path },
        (None, None) => unreachable!("a call has a callee"),
    };
    let sig = ctx.proc_sigs.get(&callee.symbol).cloned().unwrap_or_default();
    let source_params: Vec<&IrParam> = sig.params.iter().filter(|param| param.name != PANIC_OUT_NAME).collect();
    // A procedure exported for C callers takes every argument by value.
    let raw_export_abi = ctx.export_unwind_modes.contains_key(&callee.symbol) || ctx.ffi_imports.contains(&callee.symbol);
    let modes: Vec<Option<ParamMode>> = source_params.iter().map(|param| if raw_export_abi { Some(ParamMode::Move) } else { param.mode }).collect();
    let types: Vec<TypeRef> = source_params.iter().map(|param| param.ty.clone()).collect();
    let (args_ir, arg_values) = lower_args(&modes, &types, &call.args, ctx);
    let result_value = ctx.fresh_temp_value("call");
    if sig.ret.is_some() {
        ctx.register_value_type(&result_value, sig.ret.clone());
    }
    let mut args = arg_values;
    let needs_panic_out = sig.params.last().is_some_and(|param| param.name == PANIC_OUT_NAME);
    if needs_panic_out {
        args.push(IrValue { kind: IrValueKind::Local, name: PANIC_OUT_NAME.to_string(), ..Default::default() });
    }
    let call_ir = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: callee.symbol, ..Default::default() }, args, result: result_value.clone() });
    let mut parts = vec![Some(callee_ir), Some(args_ir), Some(call_ir)];
    if needs_panic_out {
        parts.push(Some(panic_check(ctx)));
    }
    LowerResult { ir: seq_ir(parts), value: result_value }
}
