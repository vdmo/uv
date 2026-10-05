//! What typing a statement or expression knows about where it stands: the return type
//! of the enclosing body, whether it is inside a loop or `unsafe`, the contract being
//! checked, and where diagnostics go. Fields are added as the constructs that read them
//! are ported.

use std::cell::RefCell;
use std::rc::Rc;

use uv_core::diagnostics::DiagnosticStream;
use uv_source::ast;

use super::type_env::TypeEnv;
use super::types::{TypePath, TypeRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoopFlag {
    #[default]
    None,
    Loop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContractPhase {
    #[default]
    None,
    Precondition,
    Postcondition,
}

#[derive(Clone, Default)]
pub struct StmtTypeContext<'t> {
    pub return_type: TypeRef,
    pub loop_flag: LoopFlag,
    pub in_unsafe: bool,
    pub ffi_export_boundary: bool,
    /// Where warnings and supplemental diagnostics are emitted.
    pub diags: Option<Rc<RefCell<DiagnosticStream>>>,
    /// The caller's environment, which typing a statement or block updates in place:
    /// the callbacks that type sub-expressions read it too.
    pub env_ref: Option<Rc<RefCell<TypeEnv>>>,
    pub in_parallel: bool,
    pub parallel_domain: TypeRef,
    pub keys_held: bool,
    pub key_mode: Option<ast::KeyMode>,
    pub shared_access_mode: Option<ast::KeyMode>,
    pub suppress_shared_access_check: bool,
    pub in_shared_capturing_closure: bool,
    pub in_speculative: bool,
    pub contract_phase: ContractPhase,
    pub require_pure: bool,
    pub contract: Option<&'t ast::ContractClause>,
    pub contract_dynamic: bool,
    pub test_postcondition_runtime: bool,
    pub current_class_path: Option<TypePath>,
}
