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
use crate::keys::key_paths::KeyPath;

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

/// A key held by an enclosing key block, as nested accesses are checked against it.
#[derive(Debug, Clone)]
pub struct HeldKeyTypingInfo {
    pub path: KeyPath,
    pub mode: ast::KeyMode,
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
    /// The body returns an opaque type, whose underlying type the first `return` fixes.
    pub opaque_return: bool,
    pub in_parallel: bool,
    pub parallel_domain: TypeRef,
    pub keys_held: bool,
    pub key_mode: Option<ast::KeyMode>,
    pub held_key_paths: Vec<HeldKeyTypingInfo>,
    pub shared_access_mode: Option<ast::KeyMode>,
    pub suppress_shared_access_check: bool,
    pub in_shared_capturing_closure: bool,
    pub in_speculative: bool,
    pub contract_phase: ContractPhase,
    pub require_pure: bool,
    pub contract: Option<&'t ast::ContractClause>,
    pub contract_dynamic: bool,
    /// The facts known at this point, which static proofs may use.
    pub proof_ctx: Option<Rc<crate::contracts::verification::StaticProofContext>>,
    pub test_postcondition_runtime: bool,
    pub current_class_path: Option<TypePath>,
    /// The range each loop variable in scope runs over, by name.
    pub loop_iteration_ranges: Option<Rc<std::collections::HashMap<crate::context::IdKey, ast::ExprPtr>>>,
}

/// The context for an access that needs the given mode of key, unless an enclosing
/// access already needs a stronger one.
pub fn with_shared_access_mode<'t>(
    ctx: &StmtTypeContext<'t>,
    mode: ast::KeyMode,
) -> StmtTypeContext<'t> {
    let mut out = ctx.clone();
    if out.shared_access_mode.is_none()
        || (mode == ast::KeyMode::Write && out.shared_access_mode != Some(ast::KeyMode::Write))
    {
        out.shared_access_mode = Some(mode);
    }
    out
}

pub fn suppress_shared_access_check<'t>(ctx: &StmtTypeContext<'t>) -> StmtTypeContext<'t> {
    let mut out = ctx.clone();
    out.suppress_shared_access_check = true;
    out
}
