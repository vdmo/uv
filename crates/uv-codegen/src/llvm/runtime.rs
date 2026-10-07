//! The runtime's procedures as the compiler declares them (`GetRuntimeFuncInfo`,
//! `LLVMEmitter::DeclareRuntime`). Only the procedures a ported construct calls are listed.

use super::*;

/// `GetRuntimeFuncInfo`: the signature of a runtime procedure.
pub(super) fn runtime_func_info(symbol: &str) -> Option<ProcSig> {
    let param = |name: &str, ty: TypeRef| IrParam { mode: None, name: name.to_string(), stable_name: String::new(), ty };
    if symbol == runtime_path_sig(&["panic"]) {
        return Some(ProcSig { params: vec![param("code", make_type_prim("u32"))], ret: make_type_prim("()") });
    }
    None
}

/// `DeclAttrs`: every runtime procedure is `nounwind`, and those that panic or abort do not return.
fn decl_attrs(symbol: &str) -> Vec<FuncAttr> {
    let mut attrs = vec![FuncAttr::NoUnwind];
    if symbol.contains("panic") || symbol.contains("abort") {
        attrs.push(FuncAttr::NoReturn);
    }
    attrs
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `DeclareRuntime`: the runtime procedures the module refers to, in the order of their names.
    pub(super) fn declare_runtime(&mut self, refs: &[String]) {
        for symbol in refs {
            if self.b.find_function(symbol).is_some() {
                continue;
            }
            let Some(info) = runtime_func_info(symbol) else {
                continue;
            };
            let abi = self.compute_call_abi(&info.params, &info.ret, false, false, false);
            let (true, Some(fn_ty)) = (abi.valid, abi.func_type.clone()) else {
                continue;
            };
            let func = self.b.function(symbol, fn_ty, Linkage::External);
            self.b.set_call_conv(func, CallConv::C);
            for (index, attrs) in abi.param_attrs.iter().enumerate() {
                for attr in attrs {
                    self.b.add_param_attr(func, index, attr.clone());
                }
            }
            for attr in decl_attrs(symbol) {
                self.b.add_attr(func, attr);
            }
        }
    }
}
