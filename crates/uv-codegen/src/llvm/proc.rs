//! `LLVMEmitter::EmitProc` and the emission of the IR of a procedure.

use super::*;

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `EmitProc`.
    pub(super) fn emit_proc(&mut self, proc: &ProcIr) {
        let Some(&func) = self.functions.get(&proc.symbol) else {
            return;
        };
        self.proc_failed = false;
        self.proc_failure = None;
        let declared_before = self.b.function_count();
        let generated = is_generated_proc_symbol(&proc.symbol);
        let saved_module = if proc.defining_module_path.is_empty() || self.ctx.module_path == proc.defining_module_path {
            None
        } else {
            Some(std::mem::replace(&mut self.ctx.module_path, proc.defining_module_path.clone()))
        };
        self.locals.clear();
        self.local_types.clear();
        self.values.clear();
        self.scratch_slots.clear();
        let entry = self.b.block(func, "entry");
        self.b.set_insert_point(entry);
        let abi_params = self.build_proc_abi_params(&proc.symbol, &proc.params);
        let abi = self.compute_proc_abi(&proc.symbol, &proc.params, &proc.ret);
        if !abi.valid {
            if !self.b.current_terminated() {
                self.b.unreachable();
            }
            self.restore_module(saved_module);
            return;
        }
        self.bind_params(func, &abi_params, &abi);
        // The panic record: a procedure that is not given one makes its own.
        if !self.locals.contains_key(PANIC_OUT_NAME) {
            self.fail("a procedure without a panic record parameter");
        }
        if !generated && !proc.defining_module_path.is_empty() {
            let module_path = proc.defining_module_path.clone();
            self.emit_poison_check(&module_path);
        }
        if self.ctx.needs_panic_out_for_symbol(&proc.symbol) {
            self.clear_panic_record_at(None);
        }
        self.emit_ir(&proc.body);
        // A block the body left open returns a null value.
        let ret_ty = self.b.func_ty(func).ret.clone();
        let open: Vec<_> = self.b.open_blocks(func);
        for block in open {
            self.b.set_insert_point(block);
            if ret_ty.is_void() {
                self.b.ret_void();
            } else {
                self.b.ret(&Value::zero(ret_ty.clone()));
            }
        }
        self.locals.clear();
        self.local_types.clear();
        self.values.clear();
        self.restore_module(saved_module);
        if self.proc_failed {
            // A procedure that could not be emitted leaves nothing behind.
            self.b.remove_function(func);
            self.b.remove_declarations_since(declared_before);
            self.functions.remove(&proc.symbol);
            self.proc_failed = false;
            let why = self.proc_failure.take().unwrap_or_default();
            self.failed_symbols.insert(proc.symbol.clone());
            self.failed_procs.push((proc.symbol.clone(), why));
        }
    }

    fn restore_module(&mut self, saved: Option<Vec<String>>) {
        if let Some(path) = saved {
            self.ctx.module_path = path;
        }
    }

    /// The parameters of the procedure as locals: by their address when they are passed by
    /// reference, in a stack slot when by value.
    fn bind_params(&mut self, func: FuncId, params: &[IrParam], abi: &AbiCall) {
        for (index, param) in params.iter().enumerate() {
            let slot = abi.param_indices.get(index).copied().flatten();
            let Some(arg_index) = slot else {
                self.bind_zero_sized_param(func, param);
                continue;
            };
            self.b.set_param_name(func, arg_index, &param.name);
            let arg = self.b.param(func, arg_index);
            let kind = abi.param_kinds.get(index).copied().unwrap_or(PassKind::ByValue);
            if kind == PassKind::ByRef {
                self.register_local(&param.name, &param.stable_name, arg, &param.ty);
                continue;
            }
            let llvm_ty = self.llvm_type(&param.ty);
            let alloca = self.b.alloca_entry(func, &llvm_ty, &param.name);
            self.b.store(&arg, &alloca);
            self.register_local(&param.name, &param.stable_name, alloca, &param.ty);
        }
    }

    fn bind_zero_sized_param(&mut self, func: FuncId, param: &IrParam) {
        let ty = self.llvm_type(&param.ty);
        let alloca = self.b.alloca_entry(func, &ty, &param.name);
        self.b.store(&Value::zero(ty), &alloca);
        self.register_local(&param.name, &param.stable_name, alloca, &param.ty);
    }

    /// `RegisterLocalBindStorage` of a name, and of its stable name when that differs.
    pub(super) fn register_local(&mut self, name: &str, stable_name: &str, storage: Value, ty: &TypeRef) {
        self.locals.insert(name.to_string(), storage.clone());
        self.local_types.insert(name.to_string(), ty.clone());
        if !stable_name.is_empty() && stable_name != name {
            self.locals.insert(stable_name.to_string(), storage);
            self.local_types.insert(stable_name.to_string(), ty.clone());
        }
    }

    /// `EmitIR`.
    pub(super) fn emit_ir(&mut self, ir: &Option<IrPtr>) {
        let Some(ir) = ir else {
            return;
        };
        if self.b.current_terminated() {
            return;
        }
        match ir.as_ref() {
            Ir::Opaque => {}
            Ir::Seq { items } => {
                for item in items {
                    self.emit_ir(&Some(item.clone()));
                }
            }
            Ir::Block { setup, body, value } => {
                self.emit_ir(setup);
                if self.b.current_terminated() {
                    return;
                }
                self.emit_ir(body);
                if self.b.current_terminated() {
                    return;
                }
                self.set_result(value);
            }
            Ir::Return { value } => self.emit_return_ir(value),
            Ir::ReadVar { .. } => {}
            Ir::ClearPanic => self.clear_panic_record_at(None),
            Ir::PanicCheck => self.emit_panic_check(&None),
            Ir::CleanupPanicCheck { cleanup_ir } => self.emit_panic_check(cleanup_ir),
            Ir::LowerPanic { reason, cleanup_ir } => self.emit_lower_panic(reason, cleanup_ir),
            Ir::Call { callee, args, result } => self.emit_call(callee, args, result),
            Ir::ReadPath { path, name } => self.emit_read_path(path, name),
            Ir::BindVar { name, stable_name, value, ty, prov, prov_region, prov_region_tag } => self.emit_bind_var(name, stable_name, value, ty, *prov, prov_region, prov_region_tag),
            other => self.fail(&format!("the IR form {}", ir_form_name(other))),
        }
    }

    /// `IRReadPath`: the symbol a path names, after the poison check of the module that owns it.
    fn emit_read_path(&mut self, path: &[String], name: &str) {
        let mut full = path.to_vec();
        full.push(name.to_string());
        let mangled = scoped_sym(&full);
        let qualified = string_of_path(&full);
        let known_proc = self.ctx.proc_sig(&mangled).is_some();
        let known_static = self.ctx.static_type(&mangled).is_some();
        if let Some(owner) = self.ctx.record_ctor_module(&mangled).cloned() {
            self.emit_poison_check_of_user_module(&owner);
            self.alias_symbol(name, &qualified, &mangled);
            return;
        }
        if self.functions.contains_key(&mangled) || self.b.find_function(&mangled).is_some() || self.b.module.global(&mangled).is_some() || known_proc || known_static {
            if known_proc {
                if let Some(owner) = self.ctx.proc_module(&mangled).cloned() {
                    self.emit_poison_check_of_user_module(&owner);
                }
            } else if known_static {
                if let Some(owner) = self.ctx.static_module(&mangled).cloned() {
                    self.emit_poison_check_of_user_module(&owner);
                }
            }
            self.alias_symbol(name, &qualified, &mangled);
            return;
        }
        if self.functions.contains_key(name) || self.b.find_function(name).is_some() || self.b.module.global(name).is_some() {
            self.alias_symbol(name, &qualified, name);
        }
    }

    fn emit_poison_check_of_user_module(&mut self, module_path: &[String]) {
        if !module_path.is_empty() {
            self.emit_poison_check(module_path);
        }
    }

    fn alias_symbol(&mut self, name: &str, qualified: &str, symbol: &str) {
        self.symbol_aliases.insert(name.to_string(), symbol.to_string());
        self.symbol_aliases.insert(qualified.to_string(), symbol.to_string());
    }

    /// `EmitBindVar`, for bindings whose storage is a stack slot of a scalar.
    #[allow(clippy::too_many_arguments)]
    fn emit_bind_var(&mut self, name: &str, stable_name: &str, value: &IrValue, bind_ty: &TypeRef, prov: ProvenanceKind, region: &Option<String>, region_tag: &Option<String>) {
        let source_type = self.lookup_value_type(value);
        let slot_ty = bind_ty.clone().or(source_type.clone());
        let handle = matches!(strip_perm(&slot_ty).or(slot_ty.clone()).as_deref().map(|ty| &ty.node), Some(TypeNode::Path { path, .. }) if path.len() == 1 && path[0] == "Region");
        let region_slot = !handle && prov == ProvenanceKind::Region && region.as_deref().is_some_and(|r| !r.is_empty()) && region_tag.as_deref().is_some_and(|t| !t.is_empty());
        if region_slot {
            self.fail("bindings in a region slot");
            return;
        }
        if slot_ty.is_none() {
            self.fail("a binding without a type");
            return;
        }
        let mut llvm_ty = self.llvm_type(bind_ty);
        if llvm_ty.is_void() {
            llvm_ty = Ty::i64();
        }
        if matches!(llvm_ty, Ty::Struct { .. } | Ty::Array(..) | Ty::Named(_)) {
            self.fail("bindings of aggregates");
            return;
        }
        let Some(func) = self.b.current_func() else {
            return;
        };
        let align = bind_ty.as_ref().and_then(|_| align_of(&self.ctx.scope, bind_ty)).map_or(1, |align| align.max(1));
        let slot = self.b.alloca_entry_aligned(func, &llvm_ty, align, name);
        let pointer_valued = is_pointer_value_type(&source_type) || is_pointer_value_type(bind_ty);
        let source_storage = if value.kind == IrValueKind::Local { self.locals.get(&value.name).cloned() } else { None };
        let init = match source_storage {
            Some(storage) if !pointer_valued => Some(self.b.load_aligned(&llvm_ty, &storage, 1, "")),
            Some(_) => self.evaluate(value),
            None => self.evaluate(value),
        };
        let init = match init {
            Some(init) => self.coerce_to(&init, &llvm_ty).unwrap_or_else(|| Value::zero(llvm_ty.clone())),
            None => Value::zero(llvm_ty.clone()),
        };
        self.b.store(&init, &slot);
        self.register_local(name, stable_name, slot, bind_ty);
    }

    /// `SetForwardedOrMaterializedResult`.
    fn set_result(&mut self, value: &IrValue) {
        if value.kind != IrValueKind::Opaque {
            return;
        }
        if let Some(evaluated) = self.evaluate(value) {
            self.values.insert(value.name.clone(), evaluated);
        }
    }

    /// `IRReturn`.
    fn emit_return_ir(&mut self, ret: &IrValue) {
        let Some(func) = self.b.current_func() else {
            return;
        };
        let ret_ty = self.b.func_ty(func).ret.clone();
        if ret_ty.is_void() {
            self.fail("procedures that return through a pointer");
            self.b.ret_void();
            return;
        }
        let value = self.evaluate_or_default(ret);
        let value = self.coerce_to(&value, &ret_ty).unwrap_or_else(|| Value::zero(ret_ty.clone()));
        self.b.ret(&value);
    }
}

/// `IsPointerValueType`.
fn is_pointer_value_type(ty: &TypeRef) -> bool {
    let ty = strip_perm(ty).or(ty.clone());
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Ptr { .. } | TypeNode::RawPtr { .. }) => true,
        Some(TypeNode::Path { path, .. }) => path.last().is_some_and(|tail| tail == "Ptr" || tail == "RawPtr" || tail == "GpuPtr"),
        _ => false,
    }
}

/// `IsGeneratedProcSymbol`: procedures the compiler makes up.
fn is_generated_proc_symbol(symbol: &str) -> bool {
    symbol == "main" || symbol.starts_with(&runtime_path_sig(&["init"])) || symbol.starts_with(&runtime_path_sig(&["deinit"])) || symbol.starts_with(&runtime_path_sig(&["drop"]))
}

fn ir_form_name(ir: &Ir) -> String {
    let text = format!("{ir:?}");
    text.split(|c: char| !c.is_alphanumeric()).next().unwrap_or("").to_string()
}
