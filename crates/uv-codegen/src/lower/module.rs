//! Lowering: module.

use super::*;

pub(super) fn module_init_fn(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> Option<ProcIr> {
    ctx.pending = None;
    let saved = ctx.active_static_init_module.replace(module_path.join("::"));
    let body = lower_static_init(module_path, module, ctx);
    ctx.active_static_init_module = saved;
    ctx.pending.is_none().then(|| ProcIr { symbol: init_sym(module_path), params: vec![panic_out_param()], ret: make_type_prim("()"), body: Some(body), ..Default::default() })
}

pub(super) fn module_deinit_fn(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> Option<ProcIr> {
    ctx.pending = None;
    let body = lower_static_deinit(module_path, module, ctx);
    ctx.pending.is_none().then(|| ProcIr { symbol: deinit_sym(module_path), params: vec![panic_out_param()], ret: make_type_prim("()"), body: Some(body), ..Default::default() })
}

/// What lowering a module gave: the declarations that are ported, and for each that
/// is not, its symbol and what stopped it.
#[derive(Default)]
pub struct LoweredModule {
    pub decls: IrDecls,
    pub pending: Vec<(String, String)>,
    /// The symbol of the procedure `main`, when the module of an executable has one.
    pub main_symbol: Option<String>,
}

pub fn lower_module(module: &ASTModule, ctx: &mut LowerCtx) -> LoweredModule {
    let mut out = LoweredModule::default();
    ctx.enter_module(&module.path);
    for item in &module.items {
        let pending_before = out.pending.len();
        match item {
            ASTItem::UsingDecl(_) | ASTItem::TypeAliasDecl(_) | ASTItem::ImportDecl(_) | ASTItem::EnumDecl(_) | ASTItem::ErrorItem(_) => {}
            ASTItem::ProcedureDecl(decl) => {
                if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
                    continue;
                }
                let symbol = mangle_proc_in_module(module, decl);
                let proc = lower_proc(decl, &module.path, symbol.clone(), ctx);
                ctx.register_linkage(&symbol, decl.vis);
                if ctx.executable_project && decl.name == "main" {
                    out.main_symbol = Some(symbol.clone());
                }
                // What later calls check before the call: the precondition, as the callee declares it.
                if decl.contract.as_ref().is_some_and(|contract| contract.precondition.is_some()) {
                    ctx.local_preconditions.insert(symbol.clone());
                }
                ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: proc.params.clone(), ret: proc.ret.clone() });
                match ctx.pending.take() {
                    None => out.decls.push(IrDecl::Proc(proc)),
                    Some(what) => {
                        ctx.binding_ids_exact = false;
                        out.pending.push((symbol, what));
                    }
                }
            }
            ASTItem::StaticDecl(decl) => {
                ctx.pending = None;
                let decls = emit_global(decl, &module.path, ctx);
                match ctx.pending.take() {
                    None => out.decls.extend(decls),
                    Some(what) => out.pending.push((scoped_sym(&module.path), what)),
                }
            }
            // A record, a modal or a class gives declarations only through its methods.
            ASTItem::RecordDecl(decl) => {
                let record_symbol = scoped_sym(&item_path_proc(&module.path, &decl.name));
                ctx.record_ctors.insert(record_symbol, module.path.clone());
                for member in &decl.members {
                    let ast::RecordMember::MethodDecl(method) = member else {
                        continue;
                    };
                    ctx.pending = None;
                    let proc = lower_record_method(decl, method, &module.path, ctx);
                    let symbol = scoped_sym(&item_path_proc(&item_path_proc(&module.path, &decl.name), &method.name));
                    if let Some(proc) = &proc {
                        ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: proc.params.clone(), ret: proc.ret.clone() });
                        ctx.register_linkage(&symbol, method.vis);
                    }
                    match (proc, ctx.pending.take()) {
                        (Some(proc), None) => out.decls.push(IrDecl::Proc(proc)),
                        (_, what) => out.pending.push((symbol, what.unwrap_or_else(|| "record methods".to_string()))),
                    }
                }
            }
            ASTItem::ModalDecl(decl) => {
                let generic = decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty());
                if generic {
                    continue;
                }
                let mut modal_path = module.path.clone();
                modal_path.push(decl.name.clone());
                for state in &decl.states {
                    // The methods of a state first, then its transitions.
                    let mut lowered: Vec<(String, Option<ProcIr>, Option<String>, ast::Visibility)> = Vec::new();
                    for member in &state.members {
                        if let ast::StateMember::StateMethodDecl(method) = member {
                            ctx.pending = None;
                            let proc = lower_state_method(decl, state, method, &module.path, ctx);
                            let mut symbol_path = modal_path.clone();
                            symbol_path.push(state.name.clone());
                            symbol_path.push(method.name.clone());
                            lowered.push((scoped_sym(&symbol_path), proc, ctx.pending.take(), method.vis));
                        }
                    }
                    for member in &state.members {
                        if let ast::StateMember::TransitionDecl(trans) = member {
                            ctx.pending = None;
                            let proc = lower_transition(decl, state, trans, &module.path, ctx);
                            let mut symbol_path = modal_path.clone();
                            symbol_path.push(state.name.clone());
                            symbol_path.push(trans.name.clone());
                            lowered.push((scoped_sym(&symbol_path), proc, ctx.pending.take(), trans.vis));
                        }
                    }
                    for (symbol, proc, pending, vis) in lowered {
                        if let Some(proc) = &proc {
                            ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: proc.params.clone(), ret: proc.ret.clone() });
                            ctx.register_linkage(&symbol, vis);
                        }
                        match (proc, pending) {
                            (Some(proc), None) => out.decls.push(IrDecl::Proc(proc)),
                            (_, what) => out.pending.push((symbol, what.unwrap_or_else(|| "modal methods and transitions".to_string()))),
                        }
                    }
                }
            }
            ASTItem::ClassDecl(decl) => {
                for item in &decl.items {
                    let ast::ClassItem::ClassMethodDecl(method) = item else {
                        continue;
                    };
                    ctx.pending = None;
                    let procs = lower_class_method_body(decl, method, &module.path, ctx);
                    let pending = ctx.pending.take();
                    for (symbol, proc) in procs {
                        ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: proc.params.clone(), ret: proc.ret.clone() });
                        ctx.register_linkage(&symbol, method.vis);
                        match &pending {
                            None => out.decls.push(IrDecl::Proc(proc)),
                            Some(what) => out.pending.push((symbol, what.clone())),
                        }
                    }
                }
            }
            ASTItem::ExternBlock(block) => {
                ctx.pending = None;
                let decls = lower_extern_block(block, &module.path, ctx);
                match ctx.pending.take() {
                    None => out.decls.extend(decls),
                    Some(what) => out.pending.push((scoped_sym(&module.path), what)),
                }
            }
            ASTItem::ComptimeProcedureDecl(_) | ASTItem::DeriveTargetDecl(_) => {}
        }
        if out.pending.len() > pending_before {
            ctx.binding_ids_exact = false;
        }
    }
    match module_init_fn(&module.path, module, ctx) {
        Some(proc) => out.decls.push(IrDecl::Proc(proc)),
        None => out.pending.push((init_sym(&module.path), ctx.pending.take().unwrap_or_default())),
    }
    match module_deinit_fn(&module.path, module, ctx) {
        Some(proc) => out.decls.push(IrDecl::Proc(proc)),
        None => out.pending.push((deinit_sym(&module.path), ctx.pending.take().unwrap_or_default())),
    }
    out
}
