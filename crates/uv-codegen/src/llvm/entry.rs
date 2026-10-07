//! `LLVMEmitter::EmitEntryPoint`: the `main` function of the program, which starts the runtime,
//! initialises the modules, calls the program's entry procedure and tears everything down.

use super::module::lifecycle_bridge_symbol;
use super::*;
use uv_project::language_profile::runtime_path_sig;

fn context_type() -> TypeRef {
    make_type_path(vec!["Context".to_string()])
}

/// `ComputeEntryInitOrder`: the modules in the order they are initialised: the order of the
/// plan, then the edges that make one module wait for another (a stable topological sort).
fn init_order(ctx: &LowerCtx) -> Vec<Vec<String>> {
    let (modules, edges) = ctx.init_plan();
    if modules.is_empty() {
        return Vec::new();
    }
    if edges.is_empty() {
        return modules.to_vec();
    }
    let n = modules.len();
    let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indegree = vec![0usize; n];
    for &(from, to) in edges {
        if from >= n || to >= n {
            continue;
        }
        outgoing[from].push(to);
        indegree[to] += 1;
    }
    let mut ready: std::collections::BTreeSet<usize> = (0..n).filter(|index| indegree[*index] == 0).collect();
    let mut order = Vec::new();
    while let Some(&current) = ready.iter().next() {
        ready.remove(&current);
        order.push(modules[current].clone());
        for &successor in &outgoing[current] {
            if indegree[successor] == 0 {
                continue;
            }
            indegree[successor] -= 1;
            if indegree[successor] == 0 {
                ready.insert(successor);
            }
        }
    }
    if order.len() == n {
        order
    } else {
        modules.to_vec()
    }
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `EntryLifecycleCallSym`: the procedure that initialises or tears down a module, as this
    /// module calls it: directly when it is this module, through its bridge otherwise.
    fn lifecycle_call_symbol(&self, module_path: &[String], is_init: bool) -> String {
        if module_path != self.ctx.module_path.as_slice() {
            return lifecycle_bridge_symbol(module_path, is_init);
        }
        if is_init {
            crate::symbols::init_sym(module_path)
        } else {
            crate::symbols::deinit_sym(module_path)
        }
    }

    /// The function a lifecycle symbol names, declared as taking the panic out-parameter when
    /// no module defined it.
    fn lifecycle_function(&mut self, symbol: &str) -> FuncId {
        if let Some(&func) = self.functions.get(symbol) {
            return func;
        }
        if let Some(func) = self.b.find_function(symbol) {
            return func;
        }
        self.b.function(symbol, Ty::func(Ty::Void, vec![Ty::Ptr], false), Linkage::External)
    }

    /// An `alloca` at the start of the entry block that holds a value so it can be passed by
    /// reference (`MaterializeRuntimeValueRef`).
    fn materialize_value_ref(&mut self, value: &Value, ty: &Ty, name: &str) -> Value {
        let func = self.b.current_func().expect("a function");
        let slot = self.b.alloca_entry(func, ty, name);
        self.b.store(value, &slot);
        slot
    }

    /// `EmitContextFieldValue`: a capability of the context the runtime made, read from its
    /// slot.
    fn context_field_value(&mut self, ctx_ptr: &Value, field: &str) -> Option<Value> {
        let slot = ["io", "net", "heap", "sys", "reactor", "time"].iter().position(|name| *name == field)?;
        let dyn_size = 2 * self.ctx.scope.target_profile.map(ptr_size_bytes).unwrap_or(8) as i128;
        let ty = dynamic_type();
        let offset = Value::int(Ty::i64(), slot as i128 * dyn_size);
        let ptr = self.b.byte_gep(ctx_ptr, &offset, "");
        Some(self.b.load(&ty, &ptr, ""))
    }

    /// The context a main procedure is given, built from the one the runtime made: a record of
    /// capabilities has each field copied from its slot.
    fn entry_context_bundle(&mut self, target_type: &TypeRef, ctx_ptr: &Value) -> Option<Value> {
        let stripped = strip_perm(target_type).or_else(|| target_type.clone());
        let TypeNode::Path { path, generic_args } = &stripped.as_deref()?.node else {
            return None;
        };
        if !generic_args.is_empty() {
            return None;
        }
        let record = uv_analysis::typing::type_lookup::lookup_record_decl(&self.ctx.scope, path)?.clone();
        let target_ll = self.llvm_type(&stripped);
        let Ty::Struct { .. } = &target_ll else {
            return None;
        };
        let func = self.b.current_func()?;
        let mut fields = Vec::new();
        for member in &record.members {
            if let ast::RecordMember::FieldDecl(field) = member {
                let lowered = uv_analysis::typing::type_lower::lower_type(&self.ctx.scope, &field.r#type).ok().flatten()?;
                fields.push((field.name.clone(), Some(lowered)));
            }
        }
        let field_types: Vec<TypeRef> = fields.iter().map(|(_, ty)| ty.clone()).collect();
        let options = resolve_record_layout_options(&record.attrs);
        let layout = self.layout_record(&stripped, &field_types, &options)?;
        if layout.fields.len() < fields.len() {
            return None;
        }
        let slot = self.b.alloca_entry(func, &target_ll, "entry_ctx_bundle");
        self.b.store(&Value::zero(target_ll.clone()), &slot);
        for (index, (name, ty)) in fields.iter().enumerate() {
            let layout_field = &layout.fields[index];
            if layout_field.size == 0 {
                continue;
            }
            let field_ll = layout_field.llvm.clone();
            if field_ll.is_void() {
                continue;
            }
            let _ = ty;
            let value = match self.context_field_value(ctx_ptr, name) {
                Some(value) => self.coerce_to(&value, &field_ll).unwrap_or_else(|| Value::zero(field_ll.clone())),
                None => Value::zero(field_ll.clone()),
            };
            let offset = Value::int(Ty::i64(), layout_field.offset as i128);
            let ptr = self.b.byte_gep(&slot, &offset, "");
            self.b.store(&value, &ptr);
        }
        Some(self.b.load(&target_ll, &slot, ""))
    }

    /// `EmitEntryPoint`.
    pub(super) fn emit_entry_point(&mut self) {
        let Some(main_symbol) = self.main_symbol.clone() else {
            return;
        };
        // An entry procedure that could not be emitted leaves no program to start.
        if self.failed_symbols.contains(&main_symbol) {
            return;
        }
        let main_ty = Ty::func(Ty::i32(), Vec::new(), false);
        let main_fn = self.b.function("main", main_ty, Linkage::External);
        let entry = self.b.block(main_fn, "entry");
        self.b.set_insert_point(entry);

        // The entry procedure, as the module declared it.
        let uv_main = match self.functions.get(&main_symbol).copied() {
            Some(func) => Some(func),
            None => match self.ctx.proc_sig(&main_symbol).cloned() {
                Some(sig) => {
                    let abi = self.compute_proc_abi(&main_symbol, &sig.params, &sig.ret);
                    abi.func_type.map(|ty| self.b.function(&main_symbol, ty, Linkage::External))
                }
                None => None,
            },
        };
        let Some(uv_main) = uv_main else {
            self.b.ret(&Value::int(Ty::i32(), 1));
            return;
        };

        let panic_record_ty = self.llvm_type(&panic_record_type());
        let panic_storage = self.b.alloca(&panic_record_ty, "entry_panic");
        self.b.store(&Value::zero(panic_record_ty.clone()), &panic_storage);
        let panic_ptr = panic_storage.clone();
        let panic_out_slot = self.b.alloca(&Ty::Ptr, "entry_panic_out");
        self.b.store(&panic_ptr, &panic_out_slot);

        // The runtime's panic procedure.
        let panic_symbol = runtime_path_sig(&["panic"]);
        let panic_fn = match self.b.find_function(&panic_symbol) {
            Some(func) => func,
            None => {
                let func = self.b.function(&panic_symbol, Ty::func(Ty::Void, vec![Ty::Ptr], false), Linkage::External);
                self.b.set_call_conv(func, CallConv::C);
                func
            }
        };
        let panic_bb = self.b.block(main_fn, "entry.panic");

        // The context the runtime makes.
        let ctx_ty = self.llvm_type(&context_type());
        let ctx_storage = self.b.alloca(&ctx_ty, "entry_ctx");
        self.b.store(&Value::zero(ctx_ty.clone()), &ctx_storage);
        let context_init = runtime_path_sig(&["context_init"]);
        let init_fn = match self.b.find_function(&context_init) {
            Some(func) => func,
            None => self.b.function(&context_init, Ty::func(Ty::Void, vec![Ty::Ptr], false), Linkage::External),
        };
        let init_ty = self.b.func_ty(init_fn).clone();
        let init_cc = self.b.func_cc(init_fn);
        let callee = self.b.func_value(init_fn);
        self.b.call(&init_ty, &callee, std::slice::from_ref(&ctx_storage), init_cc, "");

        // Every module is initialised, in order; a panic in one stops the program.
        let order = init_order(self.ctx);
        for module in &order {
            let symbol = self.lifecycle_call_symbol(module, true);
            let func = self.lifecycle_function(&symbol);
            let ty = self.b.func_ty(func).clone();
            let cc = self.b.func_cc(func);
            let callee = self.b.func_value(func);
            self.b.call(&ty, &callee, std::slice::from_ref(&panic_out_slot), cc, "");
            if let Some(has_panic) = self.load_panic_flag(Some(&panic_ptr)) {
                let fail_bb = self.b.block(main_fn, "entry.init.fail");
                let cont_bb = self.b.block(main_fn, "entry.init.cont");
                self.b.cond_br(&has_panic, fail_bb, cont_bb);
                self.b.set_insert_point(fail_bb);
                self.b.br(panic_bb);
                self.b.set_insert_point(cont_bb);
            }
        }

        // The context the entry procedure is given.
        let sig = self.ctx.proc_sig(&main_symbol).cloned();
        let root_ctx_value = self.b.load(&ctx_ty, &ctx_storage, "");
        let callee_ty = self.b.func_ty(uv_main).clone();
        let mut call_args: Vec<Value> = callee_ty.params.iter().map(|ty| Value::zero(ty.clone())).collect();
        if let (Some(first), Some(sig)) = (call_args.first_mut(), &sig) {
            let ctx_param_ty = sig.params.first().map(|param| param.ty.clone());
            let bundle = ctx_param_ty.as_ref().and_then(|ty| self.entry_context_bundle(ty, &ctx_storage));
            let _ = (bundle, &root_ctx_value);
            *first = ctx_storage.clone();
        }
        if call_args.len() >= 2 {
            call_args[1] = panic_out_slot.clone();
        }
        let main_cc = self.b.func_cc(uv_main);
        let callee = self.b.func_value(uv_main);
        let result = self.b.call(&callee_ty, &callee, &call_args, main_cc, "");
        let exit_code = if callee_ty.ret.is_void() { Value::int(Ty::i32(), 0) } else { self.coerce_to(&result, &Ty::i32()).unwrap_or_else(|| Value::int(Ty::i32(), 0)) };

        // After the entry procedure: a panic ends the program, otherwise the modules are torn
        // down in the opposite order, and a panic of a teardown is kept.
        let deinit_seen = self.b.alloca(&Ty::i8(), "entry_deinit_panic_seen");
        let deinit_code = self.b.alloca(&Ty::i32(), "entry_deinit_panic_code");
        self.b.store(&Value::int(Ty::i8(), 0), &deinit_seen);
        self.b.store(&Value::int(Ty::i32(), 0), &deinit_code);
        let deinit_bb = self.b.block(main_fn, "entry.deinit");
        match self.load_panic_flag(Some(&panic_ptr)) {
            Some(has_panic) => self.b.cond_br(&has_panic, panic_bb, deinit_bb),
            None => self.b.br(deinit_bb),
        }

        // The panic block: leave with the runtime's panic procedure.
        self.b.set_insert_point(panic_bb);
        let code = self.load_panic_code_value(Some(&panic_ptr)).unwrap_or_else(|| Value::int(Ty::i32(), 1));
        let code = self.coerce_to(&code, &Ty::i32()).unwrap_or_else(|| Value::int(Ty::i32(), 1));
        let code_arg = self.materialize_value_ref(&code, &Ty::i32(), "entry_panic_code_arg");
        let panic_ty = self.b.func_ty(panic_fn).clone();
        let panic_cc = self.b.func_cc(panic_fn);
        let callee = self.b.func_value(panic_fn);
        self.b.call(&panic_ty, &callee, &[code_arg], panic_cc, "");
        self.b.unreachable();

        self.b.set_insert_point(deinit_bb);
        let clear_record = |this: &mut Self| {
            this.clear_panic_record_at(Some(&panic_ptr));
        };
        for module in order.iter().rev() {
            let symbol = self.lifecycle_call_symbol(module, false);
            let func = self.lifecycle_function(&symbol);
            let ty = self.b.func_ty(func).clone();
            let cc = self.b.func_cc(func);
            let callee = self.b.func_value(func);
            self.b.call(&ty, &callee, std::slice::from_ref(&panic_out_slot), cc, "");
            // A panic of a teardown is remembered once, and cleared so the others still run.
            if let Some(has_panic) = self.load_panic_flag(Some(&panic_ptr)) {
                let capture_bb = self.b.block(main_fn, "entry.deinit.panic.capture");
                let cont_bb = self.b.block(main_fn, "entry.deinit.panic.cont");
                self.b.cond_br(&has_panic, capture_bb, cont_bb);
                self.b.set_insert_point(capture_bb);
                let seen = self.b.load(&Ty::i8(), &deinit_seen, "");
                let already_seen = self.b.icmp("ne", &seen, &Value::int(Ty::i8(), 0), "");
                let code = self.load_panic_code_value(Some(&panic_ptr)).unwrap_or_else(|| Value::int(Ty::i32(), 1));
                let store_bb = self.b.block(main_fn, "entry.deinit.panic.store");
                let clear_bb = self.b.block(main_fn, "entry.deinit.panic.clear");
                self.b.cond_br(&already_seen, clear_bb, store_bb);
                self.b.set_insert_point(store_bb);
                self.b.store(&Value::int(Ty::i8(), 1), &deinit_seen);
                let stored = self.coerce_to(&code, &Ty::i32()).unwrap_or_else(|| Value::int(Ty::i32(), 1));
                self.b.store(&stored, &deinit_code);
                self.b.br(clear_bb);
                self.b.set_insert_point(clear_bb);
                clear_record(self);
                self.b.br(cont_bb);
                self.b.set_insert_point(cont_bb);
            }
        }
        let seen = self.b.load(&Ty::i8(), &deinit_seen, "");
        let had_panic = self.b.icmp("ne", &seen, &Value::int(Ty::i8(), 0), "");
        let restore_bb = self.b.block(main_fn, "entry.deinit.panic.restore");
        let ret_bb = self.b.block(main_fn, "entry.ret");
        self.b.cond_br(&had_panic, restore_bb, ret_bb);
        self.b.set_insert_point(restore_bb);
        let captured = self.b.load(&Ty::i32(), &deinit_code, "");
        self.store_at_offset_flag_and_code(&panic_ptr, &captured);
        self.b.br(panic_bb);
        self.b.set_insert_point(ret_bb);
        self.b.ret(&exit_code);
    }

    /// `restore_entry_panic_record`: the record says a panic happened, with the given code.
    fn store_at_offset_flag_and_code(&mut self, panic_ptr: &Value, code: &Value) {
        let (flag_offset, code_offset) = self.panic_record_offsets().unwrap_or((0, 4));
        let flag = self.coerce_to(&Value::int(Ty::i8(), 1), &Ty::i8()).unwrap();
        self.store_at_offset(panic_ptr, flag_offset, &flag);
        self.store_at_offset(panic_ptr, code_offset, code);
    }
}
