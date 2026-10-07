//! `LLVMEmitter::EvaluateIRValue`: the LLVM value of a value of the IR, and the coercions
//! between LLVM types (`CoerceTo`, `CoerceValue`).

use super::*;

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// The type the lowering gave a value, or the type of the local it names.
    pub(super) fn lookup_value_type(&self, value: &IrValue) -> TypeRef {
        if let Some(ty) = self.ctx.value_type(value) {
            return Some(ty);
        }
        if value.kind == IrValueKind::Local {
            if let Some(ty) = self.local_types.get(&value.name).cloned().flatten() {
                return Some(ty);
            }
        }
        None
    }

    /// `DefaultFor`: the zero of the type of a value, or a 64-bit zero.
    pub(super) fn default_for(&mut self, value: &IrValue) -> Value {
        match self.lookup_value_type(value) {
            Some(ty) => {
                let llvm = self.llvm_type(&Some(ty));
                Value::zero(llvm)
            }
            None => Value::int(Ty::i64(), 0),
        }
    }

    pub(super) fn evaluate_or_default(&mut self, value: &IrValue) -> Value {
        match self.evaluate(value) {
            Some(value) => value,
            None => self.default_for(value),
        }
    }

    /// `EvaluateIRValue`.
    pub(super) fn evaluate(&mut self, value: &IrValue) -> Option<Value> {
        match value.kind {
            IrValueKind::Opaque => {
                if let Some(cached) = self.values.get(&value.name) {
                    return Some(cached.clone());
                }
                if value.name == "null" {
                    return Some(Value::null());
                }
                self.fail(&format!("the value {} of the IR", value.name));
                None
            }
            IrValueKind::Local => {
                let storage = self.locals.get(&value.name).cloned()?;
                let ty = self.local_types.get(&value.name).cloned().flatten();
                let llvm = self.llvm_type(&ty);
                Some(self.b.load(&llvm, &storage, ""))
            }
            IrValueKind::Symbol => {
                let symbol = value.name.as_str();
                if let Some(func) = self.functions.get(symbol).copied().or_else(|| self.b.find_function(symbol)) {
                    return Some(self.b.func_value(func));
                }
                self.fail(&format!("the symbol {symbol}"));
                None
            }
            IrValueKind::Immediate => self.evaluate_immediate(value),
        }
    }

    /// `EvaluateIRValue` of an immediate: the bytes of the literal as a constant of its type.
    fn evaluate_immediate(&mut self, value: &IrValue) -> Option<Value> {
        let ty = self.lookup_value_type(value);
        let stripped = strip_perm(&ty).or(ty.clone());
        if value.literal_kind == Some(IrImmediateLiteralKind::String) || value.literal_kind == Some(IrImmediateLiteralKind::Bytes) || (value.name.len() >= 2 && value.name.starts_with('"') && value.name.ends_with('"')) {
            self.fail("string and bytes literals");
            return None;
        }
        if value.name == "true" {
            return Some(Value::bool(true));
        }
        if value.name == "false" {
            return Some(Value::bool(false));
        }
        if let Some(TypeNode::Prim(_)) = stripped.as_deref().map(|ty| &ty.node) {
            let llvm = self.llvm_type(&stripped);
            let mut raw: u64 = 0;
            for (index, byte) in value.bytes.iter().take(8).enumerate() {
                raw |= u64::from(*byte) << (8 * index);
            }
            match &llvm {
                Ty::Double => return Some(Value::new(format!("0x{raw:016X}"), Ty::Double)),
                Ty::Float => return Some(Value::float(f32::from_bits(raw as u32))),
                Ty::Half => return Some(Value::new(format!("0xH{:04X}", raw as u16), Ty::Half)),
                Ty::Int(bits) => {
                    let mut wide: u128 = 0;
                    for (index, byte) in value.bytes.iter().take(16).enumerate() {
                        wide |= u128::from(*byte) << (8 * index);
                    }
                    return Some(Value::const_int(*bits, wide));
                }
                _ => {}
            }
        }
        if value.bytes.is_empty() {
            return Some(Value::int(Ty::i64(), 0));
        }
        let bits = (value.bytes.len() * 8) as u32;
        let mut word: u128 = 0;
        for (index, byte) in value.bytes.iter().take(8).enumerate() {
            word |= u128::from(*byte) << (8 * index);
        }
        Some(Value::const_int(bits.max(1), word))
    }

    /// `CoerceTo`.
    pub(super) fn coerce_to(&mut self, value: &Value, target: &Ty) -> Option<Value> {
        if &value.ty == target {
            return Some(value.clone());
        }
        Some(self.coerce_value(value, target))
    }

    /// `CoerceValue`.
    fn coerce_value(&mut self, value: &Value, target: &Ty) -> Value {
        if &value.ty == target {
            return value.clone();
        }
        if let Ty::Struct { fields, .. } = target {
            if fields.is_empty() {
                return Value::zero(target.clone());
            }
        }
        if value.ty.is_ptr() && target.is_ptr() {
            return value.clone();
        }
        if let (Some(from), Some(to)) = (value.ty.int_bits(), target.int_bits()) {
            if from < to {
                return self.b.cast("zext", value, target, "");
            }
            if from > to {
                return self.b.cast("trunc", value, target, "");
            }
        }
        if value.ty.is_float() && target.is_float() {
            let rank = |ty: &Ty| match ty {
                Ty::Half => 16,
                Ty::Float => 32,
                _ => 64,
            };
            if rank(&value.ty) < rank(target) {
                return self.b.cast("fpext", value, target, "");
            }
            if rank(&value.ty) > rank(target) {
                return self.b.cast("fptrunc", value, target, "");
            }
        }
        self.fail("a conversion between these LLVM types");
        Value::zero(target.clone())
    }
}
