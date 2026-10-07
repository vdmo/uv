//! `LLVMEmitter::EmitModule`: the declarations of one module as a module of LLVM IR.

use super::literals::{is_literal_symbol, literal_refs, literal_symbol};
use super::*;
use uv_project::language_profile::runtime_path_sig;

/// The symbol of the lifecycle bridge of a module (`LifecycleBridgeSym`): a public wrapper over
/// the module's initialisation or deinitialisation, for the entry point of another module.
pub(super) fn lifecycle_bridge_symbol(module_path: &[String], is_init: bool) -> String {
    let mut path = String::from("__cx_lifecycle_");
    path.push_str(if is_init { "init_" } else { "deinit_" });
    path.push_str(&uv_core::symbols::mangle(&uv_core::symbols::string_of_path(module_path)));
    path
}

/// What emitting a module gives: its text, and why it is not complete when it is not.
pub struct EmittedModule {
    pub text: String,
    pub failure: Option<String>,
    pub failed_procs: Vec<(String, String)>,
}

/// `ExpandIR`: the declarations with the data of the literals they use.
fn expand_decls(decls: &IrDecls) -> IrDecls {
    let mut expanded = decls.clone();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for decl in decls {
        match decl {
            IrDecl::Proc(proc) => seen.insert(proc.symbol.clone()),
            IrDecl::GlobalConst(global) => seen.insert(global.symbol.clone()),
            IrDecl::GlobalZero(global) => seen.insert(global.symbol.clone()),
            IrDecl::GlobalVTable(vtable) => seen.insert(vtable.symbol.clone()),
            IrDecl::ExternProc(proc) => seen.insert(proc.symbol.clone()),
        };
    }
    for lit in literal_refs(decls) {
        let symbol = literal_symbol(lit.kind, &lit.bytes);
        if seen.insert(symbol.clone()) {
            expanded.push(IrDecl::GlobalConst(GlobalConst { symbol, bytes: lit.bytes, align: 0, externally_visible: false, export_from_shared_library: false }));
        }
    }
    expanded
}

/// `LLVMEmitter::EmitModule`.
pub fn emit_module(decls: &IrDecls, ctx: &mut LowerCtx, config: &EmitConfig) -> EmittedModule {
    let expanded = expand_decls(decls);
    let mut emitter = Emitter::new(ctx, config);
    emitter.emit_decls(&expanded);
    let failure = emitter.failure.clone();
    let failed_procs = std::mem::take(&mut emitter.failed_procs);
    EmittedModule { text: emitter.b.finish(), failure, failed_procs }
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    fn emit_decls(&mut self, decls: &IrDecls) {
        // The poison flag of the module is defined here.
        let module_path = self.ctx.module_path.clone();
        self.poison_flag(&module_path);

        // First every procedure is declared, in order, so calls can name any of them.
        for decl in decls {
            if let IrDecl::Proc(proc) = decl {
                self.declare_proc(proc);
            }
        }
        for decl in decls {
            match decl {
                IrDecl::GlobalConst(global) => self.emit_global_const(global),
                IrDecl::GlobalZero(global) => self.emit_global_zero(global),
                _ => {}
            }
        }
        for decl in decls {
            match decl {
                IrDecl::GlobalConst(_) | IrDecl::GlobalZero(_) => {}
                IrDecl::Proc(proc) => self.emit_proc(proc),
                IrDecl::ExternProc(_) => self.fail("foreign procedure declarations"),
                IrDecl::GlobalVTable(_) => self.fail("vtables"),
            }
        }
        self.emit_lifecycle_bridges();
        let elf_like = !matches!(uv_project::target_profile::object_format_of(self.profile), uv_project::target_profile::ObjectFormat::Coff);
        if self.config_shared_library && self.config_entry_module {
            self.emit_library_entry_point();
            if elf_like {
                self.emit_ctor_dtor_hooks();
            }
        }
        if self.main_symbol.is_some() && self.config_complete {
            self.emit_entry_point();
        }
        if self.config_shared_library && !self.config_exports.is_empty() && elf_like {
            let exports = self.config_exports.clone();
            self.apply_shared_library_visibility(&exports);
        }
    }

    /// The first pass over a procedure: its function, with its signature and attributes.
    fn declare_proc(&mut self, proc: &ProcIr) {
        let new_module = if proc.defining_module_path.is_empty() { self.ctx.module_path.clone() } else { proc.defining_module_path.clone() };
        let saved_module = std::mem::replace(&mut self.ctx.module_path, new_module);
        let abi = self.compute_proc_abi(&proc.symbol, &proc.params, &proc.ret);
        self.ctx.module_path = saved_module;
        let (true, Some(func_type)) = (abi.valid, abi.func_type.clone()) else {
            self.fail("the signature of a procedure");
            return;
        };
        let linkage = self.proc_linkage(&proc.symbol);
        let func = self.b.function(&proc.symbol, func_type, linkage);
        self.b.set_linkage(func, linkage);
        match &proc.abi {
            None => {}
            Some(_) => self.fail("procedures with a foreign calling convention"),
        }
        match proc.inline_mode {
            IrInlineMode::Always => self.b.add_attr(func, FuncAttr::AlwaysInline),
            IrInlineMode::Never => self.b.add_attr(func, FuncAttr::NoInline),
            IrInlineMode::Default => {}
        }
        if proc.cold {
            self.b.add_attr(func, FuncAttr::Cold);
        }
        for (index, attrs) in abi.param_attrs.iter().enumerate() {
            for attr in attrs {
                self.b.add_param_attr(func, index, attr.clone());
            }
        }
        self.functions.insert(proc.symbol.clone(), func);
    }

    /// `ProcLLVMLinkageFor`.
    fn proc_linkage(&self, symbol: &str) -> Linkage {
        let init_prefix = runtime_path_sig(&["init"]);
        let deinit_prefix = runtime_path_sig(&["deinit"]);
        if symbol.starts_with(&init_prefix) || symbol.starts_with(&deinit_prefix) {
            return Linkage::Internal;
        }
        match self.ctx.proc_is_external(symbol) {
            Some(false) => Linkage::Internal,
            _ => Linkage::External,
        }
    }

    /// `EmitGlobalConst`.
    fn emit_global_const(&mut self, global: &GlobalConst) {
        let static_type = if is_literal_symbol(&global.symbol) { make_array_u8(global.bytes.len() as u64) } else { self.ctx.static_type(&global.symbol) };
        let (ty, align) = match &static_type {
            Some(_) => {
                let ty = self.llvm_type(&static_type);
                let align = align_of(&self.ctx.scope, &static_type).unwrap_or(1);
                (ty, align)
            }
            None => (Ty::array(global.bytes.len() as u64, Ty::i8()), global.align.max(1)),
        };
        let Some(init) = constant_bytes_as_llvm(&ty, &global.bytes) else {
            self.fail("the data of a global");
            return;
        };
        let linkage = if is_literal_symbol(&global.symbol) || !global.externally_visible { Linkage::Internal } else { Linkage::External };
        let id = self.b.module.add_global(&global.symbol, ty, Some(init), true, linkage);
        self.b.module.set_global_align(id, align);
        if is_literal_symbol(&global.symbol) {
            // `LinkageOfLiteral` is internal; the data has no address of its own.
        }
    }

    /// `EmitGlobalZero`.
    fn emit_global_zero(&mut self, global: &GlobalZero) {
        let static_type = self.ctx.static_type(&global.symbol);
        let ty = match &static_type {
            Some(_) => self.llvm_type(&static_type),
            None => Ty::array(global.size, Ty::i8()),
        };
        let align = match &static_type {
            Some(_) => align_of(&self.ctx.scope, &static_type).unwrap_or(1),
            None => global.align.max(1),
        };
        let linkage = if global.externally_visible { Linkage::External } else { Linkage::Internal };
        let init = Value::zero(ty.clone()).text;
        let id = self.b.module.add_global(&global.symbol, ty, Some(init), false, linkage);
        self.b.module.set_global_align(id, align);
    }

    /// `EmitLifecycleBridges`.
    fn emit_lifecycle_bridges(&mut self) {
        let module_path = self.ctx.module_path.clone();
        for (target, is_init) in [(crate::symbols::init_sym(&module_path), true), (crate::symbols::deinit_sym(&module_path), false)] {
            let bridge = lifecycle_bridge_symbol(&module_path, is_init);
            let Some(&target_fn) = self.functions.get(&target) else {
                if !self.failed_symbols.contains(&target) {
                    self.fail("a lifecycle procedure that is not declared");
                }
                continue;
            };
            let ty = self.b.func_ty(target_fn).clone();
            if !ty.ret.is_void() {
                self.fail("a lifecycle procedure that returns");
                continue;
            }
            let cc = self.b.func_cc(target_fn);
            let func = self.b.function(&bridge, ty.clone(), Linkage::External);
            self.b.set_linkage(func, Linkage::External);
            self.b.set_call_conv(func, cc);
            let saved = self.b.insert_block();
            let entry = self.b.block(func, "entry");
            self.b.set_insert_point(entry);
            let args: Vec<Value> = (0..ty.params.len()).map(|index| self.b.param(func, index)).collect();
            let callee = self.b.func_value(target_fn);
            self.b.call(&ty, &callee, &args, cc, "");
            self.b.ret_void();
            if let Some(saved) = saved {
                self.b.set_insert_point(saved);
            }
        }
    }
}

fn make_array_u8(len: u64) -> TypeRef {
    uv_analysis::typing::types::make_type_array(make_type_prim("u8"), len, None)
}

/// `ConstantBytesAsLLVM`: the initializer of a global that holds `bytes`, as text.
fn constant_bytes_as_llvm(ty: &Ty, bytes: &[u8]) -> Option<String> {
    if bytes.is_empty() {
        return Some(Value::zero(ty.clone()).text);
    }
    match ty {
        Ty::Array(count, element) if **element == Ty::i8() && *count as usize == bytes.len() => {
            if bytes.iter().all(|byte| *byte == 0) {
                return Some("zeroinitializer".to_string());
            }
            Some(format!("c\"{}\"", llvm_escape(bytes)))
        }
        Ty::Int(bits) if *bits as usize == bytes.len() * 8 => {
            let mut value: u128 = 0;
            for (index, byte) in bytes.iter().enumerate() {
                value |= u128::from(*byte) << (index * 8);
            }
            Some(Value::uint(*bits, value).text)
        }
        Ty::Half | Ty::Float | Ty::Double => {
            let mut raw: u64 = 0;
            for (index, byte) in bytes.iter().take(8).enumerate() {
                raw |= u64::from(*byte) << (index * 8);
            }
            Some(match ty {
                Ty::Double => Value::double(f64::from_bits(raw)).text,
                Ty::Float => Value::float(f32::from_bits(raw as u32)).text,
                _ => Value::half_bits(raw as u16).text,
            })
        }
        Ty::Ptr if bytes.iter().all(|byte| *byte == 0) => Some("null".to_string()),
        _ => None,
    }
}

/// How LLVM writes the bytes of a string constant.
fn llvm_escape(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &byte in bytes {
        match byte {
            b'"' | b'\\' => out.push_str(&format!("\\{byte:02X}")),
            0x20..=0x7e => out.push(byte as char),
            _ => out.push_str(&format!("\\{byte:02X}")),
        }
    }
    out
}
