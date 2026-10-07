//! The entry points of a shared library (`EmitLibraryEntryPoint`, `EmitCtorDtorLibraryLifecycleHooks`)
//! and the visibility of what it defines (`ApplySharedLibraryDefinitionVisibility`).

use super::panic::panic_code;
use super::*;
use uv_project::language_profile::runtime_path_sig;

/// The name of the library's entry procedure, and of what it keeps.
pub const LIBRARY_ENTRY_SYMBOL: &str = "__ultraviolet_library_entry";
pub const LIBRARY_ATTACHED_SYMBOL: &str = "__uv_library_attached";
pub const IMAGE_PANIC_RECORD_SYMBOL: &str = "__uv_image_panic_record";
pub const LIBRARY_CTOR_SYMBOL: &str = "__uv_library_ctor";
pub const LIBRARY_DTOR_SYMBOL: &str = "__uv_library_dtor";

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `GetSharedLibraryImagePanicPtr`: the panic record of the library image.
    pub(super) fn image_panic_record(&mut self) -> Value {
        let panic_ty = self.llvm_type(&panic_record_type());
        if self.b.module.global(IMAGE_PANIC_RECORD_SYMBOL).is_none() {
            let id = self.b.module.add_global(IMAGE_PANIC_RECORD_SYMBOL, panic_ty.clone(), Some(Value::zero(panic_ty).text), false, Linkage::Common);
            self.b.module.set_global_align(id, 4);
        }
        Value::global(IMAGE_PANIC_RECORD_SYMBOL)
    }

    /// A field of the panic record at a byte offset, from the pointer to the record.
    fn field_ptr(&mut self, panic_ptr: &Value, offset: u64) -> Value {
        if offset == 0 {
            return panic_ptr.clone();
        }
        let index = Value::int(Ty::i64(), i128::from(offset));
        self.b.byte_gep(panic_ptr, &index, "")
    }

    fn lib_clear_panic_record(&mut self, panic_ptr: &Value, offsets: (u64, u64)) {
        let flag = self.field_ptr(panic_ptr, offsets.0);
        self.b.store(&Value::int(Ty::i8(), 0), &flag);
        let code = self.field_ptr(panic_ptr, offsets.1);
        self.b.store(&Value::int(Ty::i32(), 0), &code);
    }

    fn lib_load_panic_flag(&mut self, panic_ptr: &Value, offsets: (u64, u64)) -> Value {
        let flag = self.field_ptr(panic_ptr, offsets.0);
        let loaded = self.b.load(&Ty::i8(), &flag, "");
        self.b.icmp("ne", &loaded, &Value::int(Ty::i8(), 0), "")
    }

    fn lib_panic_out_slot(&mut self, panic_ptr: &Value, name: &str) -> Value {
        let slot = self.b.alloca(&Ty::Ptr, name);
        self.b.store(panic_ptr, &slot);
        slot
    }

    /// The call of the initialisation or the teardown of a module with the panic out-parameter.
    fn lib_call_lifecycle(&mut self, module_path: &[String], is_init: bool, panic_out: &Value) {
        let symbol = if module_path != self.ctx.module_path.as_slice() {
            super::module::lifecycle_bridge_symbol(module_path, is_init)
        } else if is_init {
            crate::symbols::init_sym(module_path)
        } else {
            crate::symbols::deinit_sym(module_path)
        };
        let target = if is_init { crate::symbols::init_sym(module_path) } else { crate::symbols::deinit_sym(module_path) };
        let sig = self.ctx.proc_sig(&symbol).or_else(|| self.ctx.proc_sig(&target)).cloned();
        let func = match self.b.find_function(&symbol) {
            Some(func) => func,
            None => {
                let ty = match &sig {
                    Some(sig) => {
                        let abi = self.compute_call_abi(&sig.params, &sig.ret, false, false, false);
                        abi.func_type
                    }
                    None => None,
                };
                // A lifecycle procedure takes the panic record and returns nothing.
                let ty = ty.unwrap_or_else(|| Ty::func(Ty::Void, vec![Ty::Ptr], false));
                self.b.function(&symbol, ty, Linkage::External)
            }
        };
        let ty = self.b.func_ty(func).clone();
        let cc = self.b.func_cc(func);
        let callee = self.b.func_value(func);
        let args: Vec<Value> = ty.params.iter().map(|_| panic_out.clone()).collect();
        self.b.call(&ty, &callee, &args, cc, "");
    }

    /// `EmitLibraryEntryPoint`.
    pub(super) fn emit_library_entry_point(&mut self) {
        let entry_ty = Ty::func(Ty::i32(), vec![Ty::Ptr, Ty::i32(), Ty::Ptr], false);
        let entry_fn = self.b.function(LIBRARY_ENTRY_SYMBOL, entry_ty, Linkage::External);
        self.b.set_call_conv(entry_fn, CallConv::C);
        if !self.b.is_declaration(entry_fn) {
            return;
        }
        let attached = Value::global(LIBRARY_ATTACHED_SYMBOL);
        if self.b.module.global(LIBRARY_ATTACHED_SYMBOL).is_none() {
            self.b.module.add_global(LIBRARY_ATTACHED_SYMBOL, Ty::i1(), Some("false".to_string()), false, Linkage::Internal);
        }
        let entry_bb = self.b.block(entry_fn, "entry");
        let attach_bb = self.b.block(entry_fn, "dll.attach");
        let detach_bb = self.b.block(entry_fn, "dll.detach");
        let other_bb = self.b.block(entry_fn, "dll.other");
        self.b.set_insert_point(entry_bb);
        self.b.set_param_name(entry_fn, 1, "fdwReason");
        let reason = self.b.param(entry_fn, 1);
        self.b.switch(&reason, other_bb, &[(Value::int(Ty::i32(), 1), attach_bb), (Value::int(Ty::i32(), 0), detach_bb)]);
        let offsets = self.panic_record_offsets().unwrap_or((0, 4));
        let order: Vec<Vec<String>> = self.ctx.init_order().to_vec();

        // Attach: the modules are initialised in order; a panic undoes what was done.
        self.b.set_insert_point(attach_bb);
        let attached_before = self.b.load(&Ty::i1(), &attached, "");
        let attach_work = self.b.block(entry_fn, "dll.attach.work");
        let attach_done = self.b.block(entry_fn, "dll.attach.done");
        self.b.cond_br(&attached_before, attach_done, attach_work);
        self.b.set_insert_point(attach_done);
        self.b.ret(&Value::int(Ty::i32(), 1));
        self.b.set_insert_point(attach_work);
        let panic_record = self.image_panic_record();
        self.lib_clear_panic_record(&panic_record, offsets);
        let attach_out = self.lib_panic_out_slot(&panic_record, "dll_attach_panic_out");
        for (index, module) in order.iter().enumerate() {
            self.lib_call_lifecycle(module, true, &attach_out);
            let cont_bb = self.b.block(entry_fn, "dll.attach.cont");
            let fail_bb = self.b.block(entry_fn, "dll.attach.fail");
            let flag = self.lib_load_panic_flag(&panic_record, offsets);
            self.b.cond_br(&flag, fail_bb, cont_bb);
            self.b.set_insert_point(fail_bb);
            self.lib_clear_panic_record(&panic_record, offsets);
            for deinit_index in (1..=index).rev() {
                let module = order[deinit_index - 1].clone();
                self.lib_call_lifecycle(&module, false, &attach_out);
                self.lib_clear_panic_record(&panic_record, offsets);
            }
            self.b.store(&Value::bool(false), &attached);
            self.b.ret(&Value::int(Ty::i32(), 0));
            self.b.set_insert_point(cont_bb);
        }
        self.b.store(&Value::bool(true), &attached);
        self.b.ret(&Value::int(Ty::i32(), 1));

        // Detach: every module is torn down; the first panic is kept.
        self.b.set_insert_point(detach_bb);
        let attached_now = self.b.load(&Ty::i1(), &attached, "");
        let detach_work = self.b.block(entry_fn, "dll.detach.work");
        let detach_done = self.b.block(entry_fn, "dll.detach.done");
        self.b.cond_br(&attached_now, detach_work, detach_done);
        self.b.set_insert_point(detach_work);
        self.lib_clear_panic_record(&panic_record, offsets);
        let detach_out = self.lib_panic_out_slot(&panic_record, "dll_detach_panic_out");
        let seen = self.b.alloca(&Ty::i1(), "dll_detach_panic_seen");
        let code_slot = self.b.alloca(&Ty::i32(), "dll_detach_panic_code");
        self.b.store(&Value::bool(false), &seen);
        self.b.store(&Value::int(Ty::i32(), 0), &code_slot);
        for module in order.iter().rev() {
            self.lib_call_lifecycle(module, false, &detach_out);
            let capture_bb = self.b.block(entry_fn, "dll.detach.panic.capture");
            let cont_bb = self.b.block(entry_fn, "dll.detach.cont");
            let flag = self.lib_load_panic_flag(&panic_record, offsets);
            self.b.cond_br(&flag, capture_bb, cont_bb);
            self.b.set_insert_point(capture_bb);
            let already_seen = self.b.load(&Ty::i1(), &seen, "");
            let code_ptr = self.field_ptr(&panic_record, offsets.1);
            let code = self.b.load(&Ty::i32(), &code_ptr, "");
            let store_bb = self.b.block(entry_fn, "dll.detach.panic.store");
            let clear_bb = self.b.block(entry_fn, "dll.detach.panic.clear");
            self.b.cond_br(&already_seen, clear_bb, store_bb);
            self.b.set_insert_point(store_bb);
            self.b.store(&Value::bool(true), &seen);
            self.b.store(&code, &code_slot);
            self.b.br(clear_bb);
            self.b.set_insert_point(clear_bb);
            self.lib_clear_panic_record(&panic_record, offsets);
            self.b.br(cont_bb);
            self.b.set_insert_point(cont_bb);
        }
        self.b.store(&Value::bool(false), &attached);
        let detach_fail = self.b.block(entry_fn, "dll.detach.fail");
        let detach_success = self.b.block(entry_fn, "dll.detach.success");
        let seen_now = self.b.load(&Ty::i1(), &seen, "");
        self.b.cond_br(&seen_now, detach_fail, detach_success);
        self.b.set_insert_point(detach_fail);
        let code = self.b.load(&Ty::i32(), &code_slot, "");
        let flag_ptr = self.field_ptr(&panic_record, offsets.0);
        self.b.store(&Value::int(Ty::i8(), 1), &flag_ptr);
        let code_ptr = self.field_ptr(&panic_record, offsets.1);
        self.b.store(&code, &code_ptr);
        self.b.ret(&Value::int(Ty::i32(), 0));
        self.b.set_insert_point(detach_success);
        self.b.ret(&Value::int(Ty::i32(), 1));
        self.b.set_insert_point(detach_done);
        self.b.ret(&Value::int(Ty::i32(), 1));
        self.b.set_insert_point(other_bb);
        self.b.ret(&Value::int(Ty::i32(), 1));
    }

    /// `EmitCtorDtorLibraryLifecycleHooks`: the library attaches when it is loaded.
    pub(super) fn emit_ctor_dtor_hooks(&mut self) {
        let Some(entry_fn) = self.b.find_function(LIBRARY_ENTRY_SYMBOL) else {
            self.fail("the library entry point");
            return;
        };
        let panic_symbol = runtime_path_sig(&["panic"]);
        for (symbol, reason, attach, ok_label, fail_label, slot_name) in [
            (LIBRARY_CTOR_SYMBOL, "ForeignPre", 1, "ctor.ok", "ctor.fail", "library_ctor_panic_code"),
            (LIBRARY_DTOR_SYMBOL, "ForeignPost", 0, "dtor.ok", "dtor.fail", "library_dtor_panic_code"),
        ] {
            let hook = self.b.function(symbol, Ty::func(Ty::Void, Vec::new(), false), Linkage::Internal);
            self.b.set_linkage(hook, Linkage::Internal);
            self.b.set_call_conv(hook, CallConv::C);
            if !self.b.is_declaration(hook) {
                continue;
            }
            let entry_bb = self.b.block(hook, "entry");
            let ok_bb = self.b.block(hook, &ok_label.to_string());
            let fail_bb = self.b.block(hook, &fail_label.to_string());
            self.b.set_insert_point(entry_bb);
            let ty = self.b.func_ty(entry_fn).clone();
            let cc = self.b.func_cc(entry_fn);
            let callee = self.b.func_value(entry_fn);
            let result = self.b.call(&ty, &callee, &[Value::null(), Value::int(Ty::i32(), attach), Value::null()], cc, "");
            let ok = self.b.icmp("ne", &result, &Value::int(Ty::i32(), 0), "");
            self.b.cond_br(&ok, ok_bb, fail_bb);
            self.b.set_insert_point(fail_bb);
            let panic_fn = match self.b.find_function(&panic_symbol) {
                Some(func) => func,
                None => {
                    let func = self.b.function(&panic_symbol, Ty::func(Ty::Void, vec![Ty::Ptr], false), Linkage::External);
                    self.b.set_call_conv(func, CallConv::C);
                    func
                }
            };
            let code = Value::int(Ty::i32(), i128::from(panic_code(reason)));
            let code_ref = {
                let slot = self.b.alloca_entry(hook, &Ty::i32(), slot_name);
                self.b.store(&code, &slot);
                slot
            };
            let panic_ty = self.b.func_ty(panic_fn).clone();
            let panic_cc = self.b.func_cc(panic_fn);
            let callee = self.b.func_value(panic_fn);
            self.b.call(&panic_ty, &callee, &[code_ref], panic_cc, "");
            self.b.unreachable();
            self.b.set_insert_point(ok_bb);
            self.b.ret_void();
        }
        self.b.module.add_ctor(65535, LIBRARY_CTOR_SYMBOL);
        self.b.module.add_dtor(65535, LIBRARY_DTOR_SYMBOL);
    }

    /// `ApplySharedLibraryDefinitionVisibility`: what a shared library defines and does not
    /// export is hidden.
    pub(super) fn apply_shared_library_visibility(&mut self, export_symbols: &std::collections::HashSet<String>) {
        let (globals, funcs) = self.b.module.defined_non_local_symbols();
        for name in funcs {
            if !export_symbols.contains(&name) {
                self.b.module.set_function_hidden_by_name(&name);
            }
        }
        for name in globals {
            if !export_symbols.contains(&name) {
                self.b.module.set_global_hidden_by_name(&name);
            }
        }
        // The lists of constructors and destructors are globals like the others.
        self.b.module.set_ctor_lists_hidden();
    }
}
