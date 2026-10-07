//! The calling convention (`abi/` and `ComputeCallABI`): how each parameter and the result
//! of a procedure are passed, and the LLVM function type that follows.

use super::*;

/// The largest value passed or returned by value.
pub const BY_VAL_MAX: u64 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassKind {
    ByValue,
    ByRef,
    SRet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamPolicy {
    ModeAware,
    ForeignBoundary,
}

/// `ByValOk`.
pub fn by_val_ok(scope: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    let (Some(size), Some(align)) = (size_of(scope, ty), align_of(scope, ty)) else {
        return false;
    };
    let ptr = scope.target_profile.map(ptr_size_bytes).unwrap_or(8) as u64;
    size <= BY_VAL_MAX && align <= ptr
}

/// `ABIParam`.
pub fn abi_param(scope: &ScopeContext<'_>, ty: &TypeRef, policy: ParamPolicy) -> Option<PassKind> {
    ty.as_ref()?;
    let size = size_of(scope, ty)?;
    if size == 0 {
        return Some(PassKind::ByValue);
    }
    if policy == ParamPolicy::ModeAware {
        return Some(PassKind::ByRef);
    }
    Some(if by_val_ok(scope, ty) { PassKind::ByValue } else { PassKind::ByRef })
}

/// `ABIRet`.
pub fn abi_ret(scope: &ScopeContext<'_>, ty: &TypeRef) -> Option<PassKind> {
    ty.as_ref()?;
    let size = size_of(scope, ty)?;
    Some(if size == 0 || by_val_ok(scope, ty) { PassKind::ByValue } else { PassKind::SRet })
}

/// `PanicRecordType`: the record a panic is written into.
pub fn panic_record_type() -> TypeRef {
    make_type_path(vec!["PanicRecord".to_string()])
}

/// `PanicOutType`: a pointer to the panic record.
pub fn panic_out_type() -> TypeRef {
    make_type_raw_ptr(RawPtrQual::Mut, panic_record_type())
}

/// `PanicOutParam`: the parameter that carries the panic record.
pub fn panic_out_param() -> IrParam {
    IrParam { mode: Some(ParamMode::Move), name: PANIC_OUT_NAME.to_string(), stable_name: String::new(), ty: panic_out_type() }
}

/// What `ComputeCallABI` finds: the LLVM function type, the attributes of each parameter, and
/// how each IR parameter is carried.
#[derive(Debug, Clone, Default)]
pub struct AbiCall {
    pub func_type: Option<FnTy>,
    pub param_types: Vec<Ty>,
    pub param_attrs: Vec<Vec<ParamAttr>>,
    pub ret_type: Option<Ty>,
    pub has_sret: bool,
    pub out_param_uses_sret_attr: bool,
    pub valid: bool,
    pub param_kinds: Vec<PassKind>,
    pub ret_kind: Option<PassKind>,
    /// The LLVM argument each IR parameter is, when it is passed at all.
    pub param_indices: Vec<Option<usize>>,
}

/// The attributes of a pointer to a value (`ComputePtrAttrs` of a valid safe pointer):
/// `noundef nonnull align A dereferenceable(S)`.
fn ptr_attrs(scope: &ScopeContext<'_>, element: &TypeRef) -> Vec<ParamAttr> {
    let mut attrs = vec![ParamAttr::NonNull, ParamAttr::NoUndef];
    match layout_for_pointer_element(scope, element) {
        Some(layout) => {
            attrs.push(ParamAttr::Dereferenceable(layout.size));
            if layout.align > 0 {
                attrs.push(ParamAttr::Align(layout.align as u32));
            }
        }
        None => {
            if let Some(size) = size_of(scope, element) {
                attrs.push(ParamAttr::Dereferenceable(size));
            }
            if let Some(align) = align_of(scope, element).filter(|align| *align > 0) {
                attrs.push(ParamAttr::Align(align as u32));
            }
        }
    }
    attrs
}

fn layout_for_pointer_element(scope: &ScopeContext<'_>, element: &TypeRef) -> Option<Layout> {
    let stripped = strip_perm(element);
    if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Dynamic(_))) {
        // `ABITy` of a dynamic type: a data pointer and a table pointer.
        let ptr = scope.target_profile.map(ptr_size_bytes).unwrap_or(8) as u64;
        return Some(Layout { size: 2 * ptr, align: ptr });
    }
    layout_of(scope, element)
}

/// `ComputeLoweredParamAttrs` of a parameter passed by reference: the pointer is to a
/// constant `T` (`ByRefAccess` decides nothing more: both access kinds wrap it the same way).
fn by_ref_param_attrs(scope: &ScopeContext<'_>, ty: &TypeRef) -> Vec<ParamAttr> {
    let mut attrs = ptr_attrs(scope, &make_type_perm(Permission::Const, ty.clone()));
    // `ComputeArgAttrsExt`: only pointers and functions get argument attributes.
    let stripped = strip_perm(ty);
    if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Ptr { .. } | TypeNode::Func { .. })) {
        match perm_of_type(ty) {
            Permission::Unique => attrs.push(ParamAttr::NoAlias),
            Permission::Const => attrs.push(ParamAttr::ReadOnly),
            Permission::Shared => {}
        }
    }
    attrs
}

/// `ComputeLoweredParamAttrs` of a parameter passed by value.
fn by_value_param_attrs(scope: &ScopeContext<'_>, ty: &TypeRef) -> Vec<ParamAttr> {
    let stripped = strip_perm(ty);
    let mut attrs = Vec::new();
    if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Ptr { .. } | TypeNode::Func { .. })) {
        match perm_of_type(ty) {
            Permission::Unique => attrs.push(ParamAttr::NoAlias),
            Permission::Const => attrs.push(ParamAttr::ReadOnly),
            Permission::Shared => {}
        }
    }
    if let Some(TypeNode::Ptr { element, state: Some(PtrState::Valid) }) = stripped.as_deref().map(|ty| &ty.node) {
        for attr in ptr_attrs(scope, element) {
            if !attrs.contains(&attr) {
                attrs.push(attr);
            }
        }
    }
    attrs
}

/// `ComputeSRetParamAttrs`.
fn sret_param_attrs(scope: &ScopeContext<'_>, ret: &TypeRef, llvm: Ty) -> Vec<ParamAttr> {
    let mut attrs = ptr_attrs(scope, &make_type_perm(Permission::Unique, ret.clone()));
    attrs.push(ParamAttr::SRet(llvm));
    attrs.push(ParamAttr::NoAlias);
    attrs
}

/// `ComputeExplicitOutParamAttrs`.
fn explicit_out_param_attrs(scope: &ScopeContext<'_>, ret: &TypeRef) -> Vec<ParamAttr> {
    ptr_attrs(scope, &make_type_perm(Permission::Unique, ret.clone()))
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `ComputeCallABI` for calls inside the language (the foreign boundary is not ported).
    pub(super) fn compute_call_abi(&mut self, params: &[IrParam], ret: &TypeRef, c_abi: bool, foreign: bool, explicit_out: bool) -> AbiCall {
        let mut result = AbiCall::default();
        if c_abi || foreign {
            self.fail("calls across the C boundary");
            return result;
        }
        let policy = ParamPolicy::ModeAware;
        let mut kinds = Vec::new();
        for param in params {
            match abi_param(&self.ctx.scope, &param.ty, policy) {
                Some(kind) => kinds.push(kind),
                None => {
                    self.fail("the ABI of a parameter");
                    return result;
                }
            }
        }
        let Some(ret_kind) = abi_ret(&self.ctx.scope, ret) else {
            self.fail("the ABI of a result");
            return result;
        };
        result.param_kinds = kinds;
        result.ret_kind = Some(ret_kind);
        result.has_sret = explicit_out || ret_kind == PassKind::SRet;
        result.out_param_uses_sret_attr = result.has_sret && !explicit_out;
        let ret_llvm = if ret.is_some() { Some(self.llvm_type(ret)) } else { None };
        let Some(ret_size) = size_of(&self.ctx.scope, ret) else {
            self.fail("the size of a result");
            return result;
        };
        if result.has_sret {
            if ret_llvm.as_ref().is_none_or(Ty::is_void) {
                self.fail("a result without a type");
                return result;
            }
            result.ret_type = Some(Ty::Void);
        } else if ret_size == 0 {
            result.ret_type = Some(Ty::Void);
        } else {
            result.ret_type = ret_llvm.clone();
        }
        result.param_indices = vec![None; params.len()];
        if result.has_sret {
            result.param_types.push(Ty::Ptr);
            let attrs = if result.out_param_uses_sret_attr { sret_param_attrs(&self.ctx.scope, ret, ret_llvm.clone().unwrap_or(Ty::Void)) } else { explicit_out_param_attrs(&self.ctx.scope, ret) };
            result.param_attrs.push(attrs);
        }
        let mut llvm_index = usize::from(result.has_sret);
        for (index, param) in params.iter().enumerate() {
            let mut kind = result.param_kinds[index];
            if (param.name == "__env" || param.name == "__dyn_receiver") && matches!(strip_perm(&param.ty).as_deref().map(|ty| &ty.node), Some(TypeNode::RawPtr { .. })) {
                kind = PassKind::ByValue;
                result.param_kinds[index] = kind;
            }
            let Some(size) = size_of(&self.ctx.scope, &param.ty) else {
                self.fail("the size of a parameter");
                return AbiCall::default();
            };
            if size == 0 {
                continue;
            }
            if kind == PassKind::ByRef {
                result.param_types.push(Ty::Ptr);
                result.param_attrs.push(by_ref_param_attrs(&self.ctx.scope, &param.ty));
                result.param_indices[index] = Some(llvm_index);
                llvm_index += 1;
                continue;
            }
            let llvm = self.llvm_type(&param.ty);
            result.param_types.push(llvm);
            result.param_attrs.push(by_value_param_attrs(&self.ctx.scope, &param.ty));
            result.param_indices[index] = Some(llvm_index);
            llvm_index += 1;
        }
        result.func_type = Some(Ty::func(result.ret_type.clone().unwrap_or(Ty::Void), result.param_types.clone(), false));
        result.valid = true;
        result
    }

    /// `BuildProcABIParams`: the parameters of a procedure with the panic record added.
    pub(super) fn build_proc_abi_params(&self, symbol: &str, params: &[IrParam]) -> Vec<IrParam> {
        let mut augmented = params.to_vec();
        if self.ctx.needs_panic_out_for_symbol(symbol) && augmented.last().is_none_or(|last| last.name != PANIC_OUT_NAME) {
            augmented.push(panic_out_param());
        }
        augmented
    }

    /// `ComputeProcABI`.
    pub(super) fn compute_proc_abi(&mut self, symbol: &str, params: &[IrParam], ret: &TypeRef) -> AbiCall {
        let augmented = self.build_proc_abi_params(symbol, params);
        self.compute_call_abi(&augmented, ret, false, false, false)
    }
}
