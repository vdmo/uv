//! `IRCall` (`emit/ir/call/direct.cpp`) and `EmitABICall` (`llvm_call.cpp`) for calls of
//! procedures whose signature is known.

use super::*;

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `AcquireReusableEntryAlloca`: the scratch slot of a type, name and ordinal in the entry
    /// block, made when the function first asks for it.
    pub(super) fn reusable_entry_alloca(&mut self, func: FuncId, ty: &Ty, name: &str, ordinal: usize) -> Value {
        let key = (name.to_string(), ty.clone());
        while self.scratch_slots.get(&key).map_or(0, Vec::len) <= ordinal {
            let slot = self.b.alloca_entry(func, ty, name);
            self.scratch_slots.entry(key.clone()).or_default().push(slot);
        }
        self.scratch_slots[&key][ordinal].clone()
    }

    /// `operator()(const IRCall &)`.
    pub(super) fn emit_call(&mut self, callee: &IrValue, args: &[IrValue], result: &IrValue) {
        if callee.kind != IrValueKind::Symbol {
            self.fail("calls of values that are not symbols");
            return;
        }
        let symbol = self.symbol_aliases.get(&callee.name).cloned().unwrap_or_else(|| callee.name.clone());
        let sig = runtime_func_info(&symbol).or_else(|| self.ctx.proc_sig(&symbol).or_else(|| self.ctx.proc_sig(&callee.name)).cloned());
        let Some(sig) = sig else {
            self.fail(&format!("calls of {symbol}, which has no known signature"));
            return;
        };
        let params = self.build_proc_abi_params(&symbol, &sig.params);
        let abi = self.compute_call_abi(&params, &sig.ret, false, false, false);
        if !abi.valid {
            return;
        }
        let Some(fn_ty) = abi.func_type.clone() else {
            return;
        };
        if abi.has_sret {
            self.fail("calls that return through a pointer");
            return;
        }
        let callee_func = match self.functions.get(&symbol).copied().or_else(|| self.b.find_function(&symbol)) {
            Some(func) => func,
            None => self.b.function(&symbol, fn_ty.clone(), Linkage::External),
        };
        let callee_value = self.b.func_value(callee_func);
        if let Some(module) = self.ctx.proc_module(&symbol).cloned() {
            if !module.is_empty() {
                self.emit_poison_check(&module);
            }
        }
        let Some(func) = self.b.current_func() else {
            return;
        };
        let mut call_args: Vec<Option<Value>> = vec![None; fn_ty.params.len()];
        let mut ordinals: HashMap<(String, Ty), usize> = HashMap::new();
        let mut source_index = 0usize;
        for (index, param) in params.iter().enumerate() {
            let is_panic_out = param.name == PANIC_OUT_NAME;
            let slot = abi.param_indices.get(index).copied().flatten();
            let source = if is_panic_out { None } else { args.get(source_index) };
            if !is_panic_out {
                source_index += 1;
            }
            let Some(slot) = slot else {
                continue;
            };
            if is_panic_out {
                let Some(panic_slot) = self.locals.get(PANIC_OUT_NAME).cloned() else {
                    self.fail("calls from a procedure without a panic record parameter");
                    return;
                };
                call_args[slot] = Some(panic_slot);
                continue;
            }
            let Some(source) = source else {
                self.fail("a call with fewer arguments than parameters");
                return;
            };
            let Some(value) = self.materialize_call_arg(func, param, source, abi.param_kinds[index], &fn_ty.params[slot], &mut ordinals) else {
                return;
            };
            call_args[slot] = Some(value);
        }
        let Some(call_args) = call_args.into_iter().collect::<Option<Vec<_>>>() else {
            self.fail("a call argument that was not made");
            return;
        };
        let cc = self.b.func_cc(callee_func);
        let value = self.b.call_with_attrs(&fn_ty, &callee_value, &call_args, cc, &abi.param_attrs, "");
        let value = if fn_ty.ret.is_void() { self.default_for(result) } else { value };
        self.values.insert(result.name.clone(), value);
        if is_never_type(&sig.ret) && !self.b.current_terminated() {
            self.b.unreachable();
        }
    }

    /// The argument of a call for one parameter that has an LLVM parameter.
    fn materialize_call_arg(&mut self, func: FuncId, param: &IrParam, source: &IrValue, kind: PassKind, target: &Ty, ordinals: &mut HashMap<(String, Ty), usize>) -> Option<Value> {
        let source_ty = self.lookup_value_type(source);
        let pointer_like = |ty: &TypeRef| matches!(strip_perm(ty).or(ty.clone()).as_deref().map(|ty| &ty.node), Some(TypeNode::Ptr { .. } | TypeNode::RawPtr { .. } | TypeNode::Func { .. } | TypeNode::Slice(_)));
        if pointer_like(&param.ty) || pointer_like(&source_ty) {
            self.fail("pointer, function and slice arguments");
            return None;
        }
        let elem_ty = self.llvm_type(&param.ty);
        if matches!(elem_ty, Ty::Struct { .. } | Ty::Array(..) | Ty::Named(_)) {
            self.fail("aggregate arguments");
            return None;
        }
        match kind {
            PassKind::ByRef => {
                if source.kind == IrValueKind::Local {
                    if let Some(storage) = self.locals.get(&source.name).cloned() {
                        let stored = self.local_types.get(&source.name).cloned().flatten();
                        let stored = if stored.is_some() { self.llvm_type(&stored) } else { elem_ty.clone() };
                        if stored == elem_ty {
                            return Some(storage);
                        }
                    }
                }
                if self.ctx.derived_value(source).is_some() {
                    self.fail("arguments that are derived values");
                    return None;
                }
                let value = self.evaluate(source)?;
                let ordinal = ordinals.entry(("byref_arg".to_string(), elem_ty.clone())).or_insert(0);
                let index = *ordinal;
                *ordinal += 1;
                let slot = self.reusable_entry_alloca(func, &elem_ty, "byref_arg", index);
                let stored = self.coerce_to(&value, &elem_ty)?;
                self.b.store(&stored, &slot);
                Some(slot)
            }
            PassKind::ByValue => {
                let value = self.evaluate(source)?;
                self.coerce_to(&value, target)
            }
            PassKind::SRet => {
                self.fail("arguments passed through a pointer to a copy");
                None
            }
        }
    }
}

/// `IsNeverType`.
fn is_never_type(ty: &TypeRef) -> bool {
    matches!(strip_perm(ty).or(ty.clone()).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!")
}
