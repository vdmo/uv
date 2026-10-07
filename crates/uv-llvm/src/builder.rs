//! The builder: functions, blocks and instructions, written as text.

use crate::module::{Block, CallConv, FuncAttr, Function, Linkage, Module, ParamAttr};
use crate::types::{quote, DataLayout, FnTy, Ty, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId {
    pub(crate) func: usize,
    pub(crate) block: usize,
}

/// A phi node whose incoming values are added after it is made.
#[derive(Debug, Clone, Copy)]
pub struct PhiId {
    block: BlockId,
    inst: usize,
}

/// A phi node and its incoming `(value, block)` pairs.
type PhiEntry = (PhiId, String, Ty, Vec<(String, String)>);

pub struct Builder {
    pub module: Module,
    pub layout: DataLayout,
    cur: Option<BlockId>,
    /// Phi nodes: type text, and the incoming (value text, block name) pairs.
    phis: Vec<PhiEntry>,
}

impl Builder {
    pub fn new(module: Module) -> Builder {
        let layout = DataLayout::parse(&module.data_layout);
        Builder { module, layout, cur: None, phis: Vec::new() }
    }

    // ---- functions -------------------------------------------------------------------

    /// The function of that name, made when there is none.
    pub fn function(&mut self, name: &str, ty: FnTy, linkage: Linkage) -> FuncId {
        if let Some(index) = self.module.func_index.get(name) {
            return FuncId(*index);
        }
        let index = self.module.funcs.len();
        let params = ty.params.len();
        self.module.funcs.push(Function { name: name.to_string(), ty, linkage, cc: CallConv::C, attrs: Vec::new(), param_attrs: vec![Vec::new(); params], ret_attrs: Vec::new(), blocks: Vec::new(), hidden: false, next_reg: 0, param_names: vec![None; params], removed: false });
        self.module.func_index.insert(name.to_string(), index);
        FuncId(index)
    }

    /// Removes a function and everything it holds (a function that could not be emitted). The
    /// function is not printed and its name is free again; other functions keep their ids.
    pub fn remove_function(&mut self, func: FuncId) {
        let name = self.module.funcs[func.0].name.clone();
        self.module.funcs[func.0].removed = true;
        self.module.funcs[func.0].blocks.clear();
        self.module.func_index.remove(&name);
        self.cur = None;
    }

    pub fn find_function(&self, name: &str) -> Option<FuncId> {
        self.module.func_index.get(name).map(|index| FuncId(*index))
    }

    pub fn set_linkage(&mut self, func: FuncId, linkage: Linkage) {
        self.module.funcs[func.0].linkage = linkage;
    }

    pub fn set_call_conv(&mut self, func: FuncId, cc: CallConv) {
        self.module.funcs[func.0].cc = cc;
    }

    pub fn set_hidden(&mut self, func: FuncId) {
        self.module.funcs[func.0].hidden = true;
    }

    pub fn add_attr(&mut self, func: FuncId, attr: FuncAttr) {
        if !self.module.funcs[func.0].attrs.contains(&attr) {
            self.module.funcs[func.0].attrs.push(attr);
        }
    }

    pub fn has_attr(&self, func: FuncId, attr: &FuncAttr) -> bool {
        self.module.funcs[func.0].attrs.contains(attr)
    }

    pub fn add_param_attr(&mut self, func: FuncId, index: usize, attr: ParamAttr) {
        if let Some(attrs) = self.module.funcs[func.0].param_attrs.get_mut(index) {
            if !attrs.contains(&attr) {
                attrs.push(attr);
            }
        }
    }

    pub fn add_ret_attr(&mut self, func: FuncId, attr: ParamAttr) {
        self.module.funcs[func.0].ret_attrs.push(attr);
    }

    pub fn func_ty(&self, func: FuncId) -> &FnTy {
        &self.module.funcs[func.0].ty
    }

    pub fn func_name(&self, func: FuncId) -> &str {
        &self.module.funcs[func.0].name
    }

    pub fn func_cc(&self, func: FuncId) -> CallConv {
        self.module.funcs[func.0].cc
    }

    pub fn is_declaration(&self, func: FuncId) -> bool {
        self.module.funcs[func.0].blocks.is_empty()
    }

    /// The function as a value: a pointer to it.
    pub fn func_value(&self, func: FuncId) -> Value {
        Value::global(&self.module.funcs[func.0].name)
    }

    /// `Argument::setName`.
    pub fn set_param_name(&mut self, func: FuncId, index: usize, name: &str) {
        if let Some(slot) = self.module.funcs[func.0].param_names.get_mut(index) {
            *slot = Some(name.to_string());
        }
    }

    pub fn param(&self, func: FuncId, index: usize) -> Value {
        let function = &self.module.funcs[func.0];
        let text = match function.param_names.get(index).and_then(Option::as_ref) {
            Some(name) => format!("%{}", quote(name)),
            None => format!("%{}", function.param_names[..index].iter().filter(|name| name.is_none()).count()),
        };
        Value::new(text, function.ty.params[index].clone())
    }

    // ---- blocks ----------------------------------------------------------------------

    /// A new block, at the end of the function. Names are made unique.
    pub fn block(&mut self, func: FuncId, name: &str) -> BlockId {
        let blocks = &self.module.funcs[func.0].blocks;
        let mut unique = name.to_string();
        let mut suffix = 0;
        while blocks.iter().any(|block| block.name == unique) {
            suffix += 1;
            unique = format!("{name}{suffix}");
        }
        let index = blocks.len();
        self.module.funcs[func.0].blocks.push(Block { name: unique, insts: Vec::new(), terminated: false });
        BlockId { func: func.0, block: index }
    }

    pub fn set_insert_point(&mut self, block: BlockId) {
        self.cur = Some(block);
    }

    pub fn insert_block(&self) -> Option<BlockId> {
        self.cur
    }

    pub fn block_name(&self, block: BlockId) -> &str {
        &self.module.funcs[block.func].blocks[block.block].name
    }

    /// The blocks of the function that do not end in a terminator.
    pub fn open_blocks(&self, func: FuncId) -> Vec<BlockId> {
        self.module.funcs[func.0].blocks.iter().enumerate().filter(|(_, block)| !block.terminated).map(|(block, _)| BlockId { func: func.0, block }).collect()
    }

    pub fn is_terminated(&self, block: BlockId) -> bool {
        self.module.funcs[block.func].blocks[block.block].terminated
    }

    pub fn current_terminated(&self) -> bool {
        self.cur.is_none_or(|block| self.is_terminated(block))
    }

    pub fn current_func(&self) -> Option<FuncId> {
        self.cur.map(|block| FuncId(block.func))
    }

    fn push(&mut self, text: String, terminator: bool) {
        let Some(block) = self.cur else {
            return;
        };
        let target = &mut self.module.funcs[block.func].blocks[block.block];
        if target.terminated {
            // Instructions after a terminator are unreachable; they are dropped as LLVM's
            // builder would put them in a block nothing reaches.
            return;
        }
        target.insts.push(text);
        if terminator {
            target.terminated = true;
        }
    }

    /// A fresh register in the current function: `%hint.N`.
    fn reg(&mut self, hint: &str) -> String {
        let Some(block) = self.cur else {
            return "%tmp".to_string();
        };
        let func = &mut self.module.funcs[block.func];
        let n = func.next_reg;
        func.next_reg += 1;
        if hint.is_empty() {
            format!("%v{n}")
        } else {
            let hint = hint.replace(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '$' | '-')), "_");
            format!("%{hint}.{n}")
        }
    }

    fn def(&mut self, hint: &str, ty: Ty, body: impl FnOnce(&str) -> String) -> Value {
        let name = self.reg(hint);
        let text = body(&name);
        self.push(text, false);
        Value::new(name, ty)
    }

    // ---- memory ----------------------------------------------------------------------

    /// An `alloca` at the insertion point, with the preferred alignment of the type, as
    /// `IRBuilder::CreateAlloca` makes it.
    pub fn alloca(&mut self, ty: &Ty, name: &str) -> Value {
        let align = self.alloca_align(ty);
        self.alloca_aligned(ty, align, name)
    }

    /// The alignment an `alloca` of the type ends with: the preferred one, raised to what the
    /// reference requires of every alloca when it finishes the module.
    pub fn alloca_align(&self, ty: &Ty) -> u64 {
        self.layout.pref_align(ty).max(self.layout.required_alloca_align(ty))
    }

    pub fn alloca_aligned(&mut self, ty: &Ty, align: u64, name: &str) -> Value {
        let reg = self.reg(name);
        self.push(format!("{reg} = alloca {ty}, align {align}"), false);
        Value::new(reg, Ty::Ptr)
    }

    /// An `alloca` put before everything else in the entry block of the function.
    pub fn alloca_entry(&mut self, func: FuncId, ty: &Ty, name: &str) -> Value {
        let align = self.alloca_align(ty);
        let reg = self.reg(name);
        let text = format!("{reg} = alloca {ty}, align {align}");
        if let Some(entry) = self.module.funcs[func.0].blocks.first_mut() {
            entry.insts.insert(0, text);
        }
        Value::new(reg, Ty::Ptr)
    }

    /// An entry-block `alloca` whose alignment is the given one, raised to what the reference
    /// requires of every alloca.
    pub fn alloca_entry_aligned(&mut self, func: FuncId, ty: &Ty, align: u64, name: &str) -> Value {
        let align = align.max(self.layout.required_alloca_align(ty));
        let reg = self.reg(name);
        let text = format!("{reg} = alloca {ty}, align {align}");
        if let Some(entry) = self.module.funcs[func.0].blocks.first_mut() {
            entry.insts.insert(0, text);
        }
        Value::new(reg, Ty::Ptr)
    }

    /// A load aligned as the ABI aligns the type.
    pub fn load(&mut self, ty: &Ty, ptr: &Value, name: &str) -> Value {
        let align = self.layout.abi_align(ty);
        self.load_aligned(ty, ptr, align, name)
    }

    pub fn load_aligned(&mut self, ty: &Ty, ptr: &Value, align: u64, name: &str) -> Value {
        let ptr = ptr.clone();
        self.def(name, ty.clone(), |reg| format!("{reg} = load {ty}, ptr {}, align {align}", ptr.text))
    }

    /// A store aligned as the ABI aligns the type of the value.
    pub fn store(&mut self, value: &Value, ptr: &Value) {
        let align = self.layout.abi_align(&value.ty);
        self.store_aligned(value, ptr, align);
    }

    pub fn store_aligned(&mut self, value: &Value, ptr: &Value, align: u64) {
        self.push(format!("store {}, ptr {}, align {align}", value.typed(), ptr.text), false);
    }

    /// `getelementptr ty, ptr, indices`.
    pub fn gep(&mut self, ty: &Ty, ptr: &Value, indices: &[Value], name: &str) -> Value {
        self.gep_flags(false, ty, ptr, indices, name)
    }

    /// `getelementptr inbounds ty, ptr, indices`.
    pub fn inbounds_gep(&mut self, ty: &Ty, ptr: &Value, indices: &[Value], name: &str) -> Value {
        self.gep_flags(true, ty, ptr, indices, name)
    }

    fn gep_flags(&mut self, inbounds: bool, ty: &Ty, ptr: &Value, indices: &[Value], name: &str) -> Value {
        // `ConstantFolder`: the address of a global plus constants is a constant expression, and
        // the global itself when every index is zero.
        if (ptr.text.starts_with('@') || ptr.text.starts_with("getelementptr")) && indices.iter().all(|index| index.const_bits().is_some()) {
            if indices.iter().all(|index| index.const_bits() == Some(0)) {
                return ptr.clone();
            }
            let flag = if inbounds { "inbounds " } else { "" };
            let typed: Vec<String> = indices.iter().map(Value::typed).collect();
            return Value::new(format!("getelementptr {flag}({ty}, ptr {}, {})", ptr.text, typed.join(", ")), Ty::Ptr);
        }
        let indices: Vec<String> = indices.iter().map(Value::typed).collect();
        let ptr = ptr.clone();
        let flag = if inbounds { "inbounds " } else { "" };
        self.def(name, Ty::Ptr, |reg| format!("{reg} = getelementptr {flag}{ty}, ptr {}, {}", ptr.text, indices.join(", ")))
    }

    /// The address of field `index` of a struct.
    pub fn struct_gep(&mut self, ty: &Ty, ptr: &Value, index: u32, name: &str) -> Value {
        self.inbounds_gep(ty, ptr, &[Value::int(Ty::i32(), 0), Value::int(Ty::i32(), i128::from(index))], name)
    }

    /// The address `offset` bytes past `ptr`.
    pub fn byte_gep(&mut self, ptr: &Value, offset: &Value, name: &str) -> Value {
        self.gep(&Ty::i8(), ptr, std::slice::from_ref(offset), name)
    }

    // ---- arithmetic ------------------------------------------------------------------

    pub fn binary(&mut self, op: &str, lhs: &Value, rhs: &Value, name: &str) -> Value {
        if let Some(folded) = fold_binary(op, lhs, rhs) {
            return folded;
        }
        let (lhs, rhs) = (lhs.clone(), rhs.clone());
        self.def(name, lhs.ty.clone(), |reg| format!("{reg} = {op} {} {}, {}", lhs.ty, lhs.text, rhs.text))
    }

    pub fn icmp(&mut self, pred: &str, lhs: &Value, rhs: &Value, name: &str) -> Value {
        if let Some(folded) = fold_icmp(pred, lhs, rhs) {
            return folded;
        }
        let (lhs, rhs) = (lhs.clone(), rhs.clone());
        self.def(name, Ty::i1(), |reg| format!("{reg} = icmp {pred} {} {}, {}", lhs.ty, lhs.text, rhs.text))
    }

    pub fn fcmp(&mut self, pred: &str, lhs: &Value, rhs: &Value, name: &str) -> Value {
        let (lhs, rhs) = (lhs.clone(), rhs.clone());
        self.def(name, Ty::i1(), |reg| format!("{reg} = fcmp {pred} {} {}, {}", lhs.ty, lhs.text, rhs.text))
    }

    pub fn not(&mut self, value: &Value, name: &str) -> Value {
        let all_ones = Value::int(value.ty.clone(), -1);
        self.binary("xor", value, &all_ones, name)
    }

    pub fn neg(&mut self, value: &Value, name: &str) -> Value {
        let zero = Value::zero(value.ty.clone());
        self.binary("sub", &zero, value, name)
    }

    pub fn fneg(&mut self, value: &Value, name: &str) -> Value {
        let value = value.clone();
        self.def(name, value.ty.clone(), |reg| format!("{reg} = fneg {} {}", value.ty, value.text))
    }

    /// A conversion: `trunc`, `zext`, `sext`, `fptrunc`, `fpext`, `fptoui`, `fptosi`,
    /// `uitofp`, `sitofp`, `ptrtoint`, `inttoptr`, `bitcast`.
    pub fn cast(&mut self, op: &str, value: &Value, to: &Ty, name: &str) -> Value {
        if let Some(folded) = fold_cast(op, value, to) {
            return folded;
        }
        let value = value.clone();
        self.def(name, to.clone(), |reg| format!("{reg} = {op} {} {} to {to}", value.ty, value.text))
    }

    pub fn select(&mut self, cond: &Value, then: &Value, otherwise: &Value, name: &str) -> Value {
        if let Some(bits) = cond.const_bits() {
            return if bits != 0 { then.clone() } else { otherwise.clone() };
        }
        let (cond, then, otherwise) = (cond.clone(), then.clone(), otherwise.clone());
        self.def(name, then.ty.clone(), |reg| format!("{reg} = select i1 {}, {}, {}", cond.text, then.typed(), otherwise.typed()))
    }

    pub fn freeze(&mut self, value: &Value, name: &str) -> Value {
        let value = value.clone();
        self.def(name, value.ty.clone(), |reg| format!("{reg} = freeze {} {}", value.ty, value.text))
    }

    pub fn extract_value(&mut self, aggregate: &Value, index: u32, ty: &Ty, name: &str) -> Value {
        let aggregate = aggregate.clone();
        self.def(name, ty.clone(), |reg| format!("{reg} = extractvalue {} {}, {index}", aggregate.ty, aggregate.text))
    }

    pub fn insert_value(&mut self, aggregate: &Value, value: &Value, index: u32, name: &str) -> Value {
        let (aggregate, value) = (aggregate.clone(), value.clone());
        self.def(name, aggregate.ty.clone(), |reg| format!("{reg} = insertvalue {} {}, {}, {index}", aggregate.ty, aggregate.text, value.typed()))
    }

    // ---- calls -----------------------------------------------------------------------

    /// A call. `callee` is a function value (`@name`) or a pointer register. A call of
    /// `void` gives an `undef` of `void`.
    pub fn call(&mut self, ty: &FnTy, callee: &Value, args: &[Value], cc: CallConv, name: &str) -> Value {
        self.call_with_attrs(ty, callee, args, cc, &[], name)
    }

    /// A call whose arguments carry attributes (`sret` and the like).
    pub fn call_with_attrs(&mut self, ty: &FnTy, callee: &Value, args: &[Value], cc: CallConv, arg_attrs: &[Vec<ParamAttr>], name: &str) -> Value {
        let mut parts = Vec::new();
        for (index, arg) in args.iter().enumerate() {
            let mut text = arg.ty.to_string();
            for attr in arg_attrs.get(index).into_iter().flatten() {
                text.push(' ');
                text.push_str(&attr.text());
            }
            parts.push(format!("{text} {}", arg.text));
        }
        let callee_ty = if ty.vararg { format!("{} ", Ty::Func(Box::new(ty.clone()))) } else { format!("{} ", ty.ret) };
        let args_text = parts.join(", ");
        if ty.ret.is_void() {
            self.push(format!("call {}{callee_ty}{}({args_text})", cc.text(), callee.text), false);
            return Value::undef(Ty::Void);
        }
        let ret = ty.ret.clone();
        self.def(name, ret, |reg| format!("{reg} = call {}{callee_ty}{}({args_text})", cc.text(), callee.text))
    }

    // ---- control flow ----------------------------------------------------------------

    pub fn br(&mut self, target: BlockId) {
        let name = quote(self.block_name(target));
        self.push(format!("br label %{name}"), true);
    }

    pub fn cond_br(&mut self, cond: &Value, then: BlockId, otherwise: BlockId) {
        let (then, otherwise) = (quote(self.block_name(then)), quote(self.block_name(otherwise)));
        self.push(format!("br i1 {}, label %{then}, label %{otherwise}", cond.text), true);
    }

    pub fn switch(&mut self, value: &Value, default: BlockId, cases: &[(Value, BlockId)]) {
        let default = quote(self.block_name(default));
        let mut text = format!("switch {}, label %{default} [", value.typed());
        for (case, block) in cases {
            text.push_str(&format!("\n    {}, label %{}", case.typed(), quote(self.block_name(*block))));
        }
        text.push_str("\n  ]");
        self.push(text, true);
    }

    pub fn ret(&mut self, value: &Value) {
        self.push(format!("ret {}", value.typed()), true);
    }

    pub fn ret_void(&mut self) {
        self.push("ret void".to_string(), true);
    }

    pub fn unreachable(&mut self) {
        self.push("unreachable".to_string(), true);
    }

    pub fn phi(&mut self, ty: &Ty, name: &str) -> (PhiId, Value) {
        let reg = self.reg(name);
        let block = self.cur.expect("a phi has a block");
        let target = &mut self.module.funcs[block.func].blocks[block.block];
        // Phi nodes come first in a block.
        let at = target.insts.iter().take_while(|inst| inst.contains("= phi ")).count();
        target.insts.insert(at, String::new());
        let id = PhiId { block, inst: at };
        // Phi nodes already in the block shift by one.
        for (other, ..) in &mut self.phis {
            if other.block == block && other.inst >= at {
                other.inst += 1;
            }
        }
        self.phis.push((id, reg.clone(), ty.clone(), Vec::new()));
        (id, Value::new(reg, ty.clone()))
    }

    pub fn add_incoming(&mut self, phi: PhiId, value: &Value, from: BlockId) {
        let from = quote(self.block_name(from));
        if let Some((_, _, _, incoming)) = self.phis.iter_mut().find(|(id, ..)| id.block == phi.block && id.inst == phi.inst) {
            incoming.push((value.text.clone(), from));
        }
    }

    /// Writes the text of the phi nodes; to be called once the function is complete.
    pub fn finish_phis(&mut self) {
        for (id, reg, ty, incoming) in std::mem::take(&mut self.phis) {
            let pairs: Vec<String> = incoming.iter().map(|(value, from)| format!("[ {value}, %{from} ]")).collect();
            self.module.funcs[id.block.func].blocks[id.block.block].insts[id.inst] = format!("{reg} = phi {ty} {}", pairs.join(", "));
        }
    }

    /// The text of the module.
    pub fn finish(mut self) -> String {
        self.finish_phis();
        self.module.print()
    }
}

fn width_mask(bits: u32) -> u128 {
    if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    }
}

fn sign_extend(value: u128, bits: u32) -> i128 {
    if bits >= 128 {
        return value as i128;
    }
    let shift = 128 - bits;
    ((value << shift) as i128) >> shift
}

/// `ConstantFolder` for the integer operations: both operands constant gives a constant.
fn fold_binary(op: &str, lhs: &Value, rhs: &Value) -> Option<Value> {
    let (Ty::Int(bits), a, b) = (lhs.ty.clone(), lhs.const_bits()?, rhs.const_bits()?) else {
        return None;
    };
    let mask = width_mask(bits);
    let (sa, sb) = (sign_extend(a, bits), sign_extend(b, bits));
    let result = match op {
        "add" => a.wrapping_add(b),
        "sub" => a.wrapping_sub(b),
        "mul" => a.wrapping_mul(b),
        "and" => a & b,
        "or" => a | b,
        "xor" => a ^ b,
        "shl" if b < u128::from(bits) => a << b,
        "lshr" if b < u128::from(bits) => a >> b,
        "ashr" if b < u128::from(bits) => (sa >> b) as u128,
        "udiv" if b != 0 => a / b,
        "urem" if b != 0 => a % b,
        "sdiv" if b != 0 && !(sb == -1 && sa == sign_extend(1u128 << (bits - 1), bits)) => (sa / sb) as u128,
        "srem" if b != 0 && !(sb == -1 && sa == sign_extend(1u128 << (bits - 1), bits)) => (sa % sb) as u128,
        _ => return None,
    };
    Some(Value::const_int(bits, result & mask))
}

fn fold_icmp(pred: &str, lhs: &Value, rhs: &Value) -> Option<Value> {
    let (Ty::Int(bits), a, b) = (lhs.ty.clone(), lhs.const_bits()?, rhs.const_bits()?) else {
        return None;
    };
    let (sa, sb) = (sign_extend(a, bits), sign_extend(b, bits));
    let result = match pred {
        "eq" => a == b,
        "ne" => a != b,
        "ult" => a < b,
        "ule" => a <= b,
        "ugt" => a > b,
        "uge" => a >= b,
        "slt" => sa < sb,
        "sle" => sa <= sb,
        "sgt" => sa > sb,
        "sge" => sa >= sb,
        _ => return None,
    };
    Some(Value::bool(result))
}

fn fold_cast(op: &str, value: &Value, to: &Ty) -> Option<Value> {
    let (Ty::Int(from_bits), Ty::Int(to_bits)) = (value.ty.clone(), to.clone()) else {
        return None;
    };
    let bits = value.const_bits()?;
    let result = match op {
        "trunc" => bits & width_mask(to_bits),
        "zext" => bits,
        "sext" => (sign_extend(bits, from_bits) as u128) & width_mask(to_bits),
        _ => return None,
    };
    Some(Value::const_int(to_bits, result))
}
