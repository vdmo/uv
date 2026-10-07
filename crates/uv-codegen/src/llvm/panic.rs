//! The panic record and the module poison flags (`llvm_ir_panic.cpp`).

use super::*;
use uv_project::language_profile::runtime_path_sig;

/// `PanicCode(PanicReason)`.
pub fn panic_code(reason: &str) -> u64 {
    match reason {
        "ErrorExpr" => 0x01,
        "ErrorStmt" => 0x02,
        "DivZero" => 0x03,
        "Overflow" => 0x04,
        "Shift" => 0x05,
        "Bounds" => 0x06,
        "Cast" => 0x07,
        "NullDeref" => 0x08,
        "ExpiredDeref" => 0x09,
        "InitPanic" => 0x0A,
        "ContractPre" => 0x0B,
        "ContractPost" => 0x0C,
        "AsyncFailed" => 0x0D,
        "ForeignPre" => 0x0E,
        "ForeignPost" => 0x0F,
        "TypeInv" => 0x10,
        "LoopInv" => 0x11,
        "MatchFail" => 0x12,
        _ => 0xFF,
    }
}

/// The symbol of the poison flag of a module.
pub fn poison_symbol(module_path: &[String]) -> String {
    let mut tail = vec!["poison"];
    tail.extend(module_path.iter().map(String::as_str));
    runtime_path_sig(&tail)
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `PanicRecordOffsets`: where the flag and the code sit in the panic record.
    pub(super) fn panic_record_offsets(&self) -> Option<(u64, u64)> {
        let fields = [make_type_prim("bool"), make_type_prim("u32")];
        let layout = record_layout_of(&self.ctx.scope, &fields, &RecordLayoutOptions::default())?;
        (layout.offsets.len() >= 2).then(|| (layout.offsets[0], layout.offsets[1]))
    }

    /// `LoadPanicOutPtr`: the panic record pointer the procedure was given.
    pub(super) fn load_panic_out_ptr(&mut self) -> Option<Value> {
        let slot = self.locals.get(PANIC_OUT_NAME).cloned()?;
        Some(self.b.load(&Ty::Ptr, &slot, ""))
    }

    fn byte_gep(&mut self, base: &Value, offset: u64) -> Value {
        let index = Value::int(Ty::i64(), i128::from(offset));
        self.b.byte_gep(base, &index, "")
    }

    /// `StoreAtOffset`.
    pub(super) fn store_at_offset(&mut self, base: &Value, offset: u64, value: &Value) {
        let ptr = if offset == 0 { base.clone() } else { self.byte_gep(base, offset) };
        self.b.store(value, &ptr);
    }

    fn load_at_offset(&mut self, base: &Value, offset: u64, ty: &Ty) -> Value {
        let ptr = if offset == 0 { base.clone() } else { self.byte_gep(base, offset) };
        self.b.load(ty, &ptr, "")
    }

    /// `LoadPanicFlag`: whether the record says a panic happened.
    pub(super) fn load_panic_flag(&mut self, panic_ptr: Option<&Value>) -> Option<Value> {
        let ptr = match panic_ptr {
            Some(ptr) => ptr.clone(),
            None => self.load_panic_out_ptr()?,
        };
        let (flag_offset, _) = self.panic_record_offsets()?;
        let flag = self.load_at_offset(&ptr, flag_offset, &Ty::i8());
        Some(self.b.icmp("ne", &flag, &Value::int(Ty::i8(), 0), ""))
    }

    /// `LoadPanicCodeValue`.
    pub(super) fn load_panic_code_value(&mut self, panic_ptr: Option<&Value>) -> Option<Value> {
        let ptr = match panic_ptr {
            Some(ptr) => ptr.clone(),
            None => self.load_panic_out_ptr()?,
        };
        let (_, code_offset) = self.panic_record_offsets()?;
        Some(self.load_at_offset(&ptr, code_offset, &Ty::i32()))
    }

    /// `StorePanicRecordValue`: a panic with the given code.
    pub(super) fn store_panic_record_value(&mut self, panic_ptr: Option<&Value>, code: &Value) {
        let ptr = match panic_ptr {
            Some(ptr) => ptr.clone(),
            None => match self.load_panic_out_ptr() {
                Some(ptr) => ptr,
                None => return,
            },
        };
        let Some((flag_offset, code_offset)) = self.panic_record_offsets() else {
            return;
        };
        let code = match code.ty.int_bits() {
            Some(bits) if bits < 32 => self.b.cast("zext", code, &Ty::i32(), ""),
            Some(bits) if bits > 32 => self.b.cast("trunc", code, &Ty::i32(), ""),
            Some(_) => code.clone(),
            None => Value::int(Ty::i32(), 0),
        };
        self.store_at_offset(&ptr, flag_offset, &Value::int(Ty::i8(), 1));
        self.store_at_offset(&ptr, code_offset, &code);
    }

    /// `StorePanicRecord(code)`.
    pub(super) fn store_panic_record(&mut self, code: u64) {
        let Some(ptr) = self.load_panic_out_ptr() else {
            return;
        };
        let code = Value::int(Ty::i32(), i128::from(code));
        self.store_panic_record_value(Some(&ptr), &code);
    }

    /// `ClearPanicRecordAt`.
    pub(super) fn clear_panic_record_at(&mut self, panic_ptr: Option<&Value>) {
        let ptr = match panic_ptr {
            Some(ptr) => ptr.clone(),
            None => match self.load_panic_out_ptr() {
                Some(ptr) => ptr,
                None => return,
            },
        };
        let Some((flag_offset, code_offset)) = self.panic_record_offsets() else {
            return;
        };
        self.store_at_offset(&ptr, flag_offset, &Value::int(Ty::i8(), 0));
        self.store_at_offset(&ptr, code_offset, &Value::int(Ty::i32(), 0));
    }

    /// `EmitReturn`: leaves the procedure with the panic code, or a null value.
    pub(super) fn emit_return(&mut self) {
        let Some(func) = self.b.current_func() else {
            return;
        };
        let ret_ty = self.b.func_ty(func).ret.clone();
        if ret_ty.is_void() {
            self.b.ret_void();
            return;
        }
        if ret_ty.is_int() {
            if let Some(code) = self.load_panic_code_value(None) {
                let code = if code.ty != ret_ty { self.int_cast(&code, &ret_ty, false) } else { code };
                self.b.ret(&code);
                return;
            }
        }
        self.b.ret(&Value::zero(ret_ty));
    }

    /// `CreateIntCast`.
    pub(super) fn int_cast(&mut self, value: &Value, to: &Ty, signed: bool) -> Value {
        let (Some(from_bits), Some(to_bits)) = (value.ty.int_bits(), to.int_bits()) else {
            return value.clone();
        };
        if from_bits == to_bits {
            value.clone()
        } else if from_bits > to_bits {
            self.b.cast("trunc", value, to, "")
        } else if signed {
            self.b.cast("sext", value, to, "")
        } else {
            self.b.cast("zext", value, to, "")
        }
    }

    /// `AsBool`: a value as a truth value.
    pub(super) fn as_bool(&mut self, value: &Value) -> Value {
        if value.ty == Ty::i1() {
            return value.clone();
        }
        let zero = Value::zero(value.ty.clone());
        self.b.icmp("ne", value, &zero, "")
    }

    /// `GetOrCreatePoisonFlag`: the flag of a module, defined here when the module is this one.
    pub(super) fn poison_flag(&mut self, module_path: &[String]) -> Value {
        let symbol = poison_symbol(module_path);
        let define = self.ctx.module_path == module_path;
        let bool_ty = self.llvm_type(&make_type_prim("bool"));
        if self.b.module.global(&symbol).is_none() {
            let init = define.then(|| "0".to_string());
            self.b.module.add_global(&symbol, bool_ty, init, false, Linkage::External);
        } else if define {
            if let Some(id) = self.b.module.global(&symbol) {
                self.b.module.set_global_init(id, Some("0".to_string()));
            }
        }
        Value::global(&symbol)
    }

    /// `EmitPoisonCheck`: a procedure of a poisoned module panics at once.
    pub(super) fn emit_poison_check(&mut self, module_path: &[String]) {
        if self.b.current_terminated() {
            return;
        }
        let flag_ptr = self.poison_flag(module_path);
        let bool_ty = self.llvm_type(&make_type_prim("bool"));
        let poisoned = self.b.load(&bool_ty, &flag_ptr, "");
        let Some(func) = self.b.current_func() else {
            return;
        };
        let panic_bb = self.b.block(func, "poison.take");
        let cont_bb = self.b.block(func, "poison.cont");
        let cond = self.as_bool(&poisoned);
        self.b.cond_br(&cond, panic_bb, cont_bb);
        self.b.set_insert_point(panic_bb);
        self.store_panic_record(panic_code("InitPanic"));
        if !self.b.current_terminated() {
            self.emit_return();
        }
        self.b.set_insert_point(cont_bb);
    }
}
