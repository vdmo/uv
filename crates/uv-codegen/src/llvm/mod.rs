//! LLVM emission: the lowered IR of a module as textual LLVM IR (`05_codegen/llvm`).
//!
//! The port is in progress: helpers for what is not emitted yet are written ahead of their users.
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use uv_analysis::context::{ScopeContext, TypeDecl};
use uv_analysis::memory::regions::ProvenanceKind;
use uv_analysis::generics::monomorphize::{build_substitution, instantiate_type, TypeSubst};
use uv_analysis::layout::{
    align_of, enum_layout_of, layout_of, lower_type_for_layout, modal_layout_of, record_layout_of, resolve_enum_layout_options, resolve_record_layout_options, size_of, tuple_layout_of, union_layout_of, Layout,
    RecordLayoutOptions,
};
use uv_analysis::modal::builtin_modal_intrinsics::{is_builtin_runtime_handle_modal_type_path, lookup_builtin_modal_layout};
use uv_analysis::modal::lookup::lookup_modal_decl;
use uv_analysis::modal::modal_widen::payload_state;
use uv_analysis::resolve::scopes::{id_eq, path_key_of};
use uv_analysis::typing::type_equiv::type_equiv;
use uv_analysis::typing::type_lookup::{async_sig_of, lookup_type_decl};
use uv_analysis::typing::type_predicates::{perm_of_type, strip_perm};
use uv_analysis::typing::types::{
    applied_type_args, applied_type_path, is_async_type, make_type_path, make_type_perm, make_type_prim, make_type_raw_ptr, type_to_string, ParamMode, Permission, PtrState, RawPtrQual, StringState, BytesState, TypeModalState, TypeNode,
    TypeRef,
};
use uv_project::language_profile::runtime_path_sig;
use uv_llvm::{Builder, CallConv, FnTy, FuncId, FuncAttr, Linkage, Module, ParamAttr, Ty, Value};
use uv_project::target_profile::{llvm_data_layout_of, llvm_triple_of, ptr_size_bytes, TargetProfile};
use uv_source::ast;

use crate::ir::*;
use crate::lower::{LowerCtx, ProcSig, PANIC_OUT_NAME};
use crate::symbols::scoped_sym;
use uv_core::symbols::string_of_path;

mod abi;
mod call;
mod entry;
mod eval;
mod library;
mod literals;
mod module;
mod panic;
mod proc;
mod runtime;
mod types;

pub use self::module::emit_module;
use self::abi::*;
use self::runtime::runtime_func_info;
use self::types::*;

/// What a module is emitted with.
pub struct EmitConfig {
    pub module_name: String,
    pub profile: TargetProfile,
    /// The entry procedure when this module holds the program's entry point.
    pub main_symbol: Option<String>,
    /// Whether the assembly is a shared library, and whether this module is its root module.
    pub shared_library: bool,
    pub entry_module: bool,
    /// The symbols the shared library exports; the others it defines are hidden.
    pub export_symbols: Vec<String>,
    /// Whether every declaration of the module was lowered. An incomplete module is not given
    /// the entry point of the program.
    pub complete: bool,
}

/// `LLVMEmitter`: the module being written and what is known about it while writing.
pub struct Emitter<'e, 'a, 'b> {
    pub(super) b: Builder,
    pub(super) ctx: &'e mut LowerCtx<'a, 'b>,
    pub(super) profile: TargetProfile,
    pub(super) main_symbol: Option<String>,
    pub(super) config_shared_library: bool,
    pub(super) config_entry_module: bool,
    pub(super) config_complete: bool,
    pub(super) config_exports: std::collections::HashSet<String>,
    type_cache: HashMap<String, Ty>,
    active_types: Vec<String>,
    /// The functions made so far, by symbol.
    pub(super) functions: HashMap<String, FuncId>,
    /// The storage of each local of the procedure being emitted (an `alloca` or a parameter
    /// passed by reference), and the type it was declared with.
    pub(super) locals: HashMap<String, Value>,
    pub(super) local_types: HashMap<String, TypeRef>,
    /// The values of the IR values that have no name in the source, by the name they have in the IR.
    pub(super) values: HashMap<String, Value>,
    /// The scratch slots of the procedure, by name and type (`AcquireReusableEntryAlloca`).
    pub(super) scratch_slots: HashMap<(String, Ty), Vec<Value>>,
    /// `SetSymbolAlias`: the symbol a path read in the IR stands for.
    pub(super) symbol_aliases: HashMap<String, String>,
    /// The first thing that could not be emitted.
    pub failure: Option<String>,
    /// Each procedure that could not be emitted, and why.
    pub failed_procs: Vec<(String, String)>,
    pub(super) failed_symbols: std::collections::HashSet<String>,
    /// What stopped the procedure being emitted.
    pub(super) proc_failure: Option<String>,
    /// Whether something could not be emitted since the current procedure began.
    pub(super) proc_failed: bool,
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    pub fn new(ctx: &'e mut LowerCtx<'a, 'b>, config: &EmitConfig) -> Self {
        let module = Module::new(&config.module_name, llvm_triple_of(config.profile), llvm_data_layout_of(config.profile));
        Emitter {
            b: Builder::new(module),
            ctx,
            profile: config.profile,
            main_symbol: config.main_symbol.clone(),
            config_shared_library: config.shared_library,
            config_entry_module: config.entry_module,
            config_complete: config.complete,
            config_exports: config.export_symbols.iter().cloned().collect(),
            type_cache: HashMap::new(),
            active_types: Vec::new(),
            functions: HashMap::new(),
            locals: HashMap::new(),
            local_types: HashMap::new(),
            values: HashMap::new(),
            scratch_slots: HashMap::new(),
            symbol_aliases: HashMap::new(),
            failure: None,
            failed_procs: Vec::new(),
            failed_symbols: std::collections::HashSet::new(),
            proc_failure: None,
            proc_failed: false,
        }
    }

    /// `ReportCodegenFailure`: something could not be emitted; the first one is kept.
    pub(super) fn fail(&mut self, what: &str) {
        self.proc_failed = true;
        if self.proc_failure.is_none() {
            self.proc_failure = Some(what.to_string());
        }
        if self.failure.is_none() {
            self.failure = Some(what.to_string());
        }
    }
}
