//! Procedure declarations: the attributes and shape of the declaration, its signature,
//! its contract, and its body. See `TypeProcedureDecl`.
//!
//! The port is in progress. A check that is not ported makes the declaration pending
//! when it would apply, rather than letting it pass.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uv_core::diagnostics::{DiagnosticStream, SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt, TypePtr};
use uv_source::attributes::{attrs, has_attribute, validate_attributes, AttributeTarget};

use super::dynamic_context::{compute_dynamic_context, DynamicScopeAncestor};
use super::item_generic_params::process_generic_params;
use super::pending::{reset_scaffolding, take_pending};
use super::stmt::block::type_block;
use super::stmt_context::{ContractPhase, StmtTypeContext};
use super::subtyping::subtyping;
use super::type_env::{BindingProvenanceSeedKind, TypeBinding, TypeEnv};
use super::type_equiv::type_equiv;
use super::type_expr::{type_expr, type_identifier_expr, type_place};
use super::type_lower::{lower_param_mode, lower_type};
use super::type_wf::{type_wf, REFINEMENT_WF_PENDING};
use super::typecheck_diag::emit_resolved_typecheck_diagnostic;
use super::types::{make_type_func, make_type_prim, TypeFuncParam, TypeNode, TypeRef};
use crate::context::ScopeContext;
use crate::caps::builtin_paths::is_context_type_path;
use crate::contracts::contract_check::check_contract_well_formed;
use crate::contracts::intrinsics::validate_contract_intrinsics;
use crate::contracts::purity::ContractContext;
use crate::caps::context_caps::is_context_bundle_type;
use crate::generics::generic_params::bind_type_params;
use crate::memory::borrow_bind::bind_check_body;
use crate::memory::region_prov::prov_bind_check;
use crate::resolve::scopes::{id_eq, id_key_of};

/// How a declaration failed: the rule, a detail, where, and the obligations.
#[derive(Default)]
pub struct DeclFailure {
    pub diag_id: Option<&'static str>,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
    pub diagnostic_obligation_ids: Vec<String>,
    /// Notes and fix-its that go with the diagnostic.
    pub children: Vec<SubDiagnostic>,
}

impl DeclFailure {
    pub(crate) fn rule(diag_id: &'static str) -> Self {
        DeclFailure { diag_id: Some(diag_id), ..Default::default() }
    }

    fn of(diag_id: Option<&'static str>) -> Self {
        DeclFailure { diag_id, ..Default::default() }
    }
}

/// The outcome of typing a declaration: it passed, it failed, or the port cannot say.
pub enum DeclOutcome {
    Ok,
    Failed(DeclFailure),
    Pending(String),
}

pub struct Signature {
    pub func_type: TypeRef,
    pub return_type: TypeRef,
    pub bindings: Vec<(String, TypeRef)>,
}

fn expr_contains_identifier(expr: &ExprPtr, name: &str) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    let has = |inner: &ExprPtr| expr_contains_identifier(inner, name);
    match &e.node {
        ExprNode::IdentifierExpr(node) => id_eq(&node.name, name),
        ExprNode::BinaryExpr(node) => has(&node.lhs) || has(&node.rhs),
        ExprNode::RangeExpr(node) => has(&node.lhs) || has(&node.rhs),
        ExprNode::PipelineExpr(node) => has(&node.lhs) || has(&node.rhs),
        ExprNode::UnaryExpr(node) => has(&node.value),
        ExprNode::CastExpr(node) => has(&node.value),
        ExprNode::DerefExpr(node) => has(&node.value),
        ExprNode::PropagateExpr(node) => has(&node.value),
        ExprNode::FieldAccessExpr(node) => has(&node.base),
        ExprNode::TupleAccessExpr(node) => has(&node.base),
        ExprNode::IndexAccessExpr(node) => has(&node.base) || has(&node.index),
        ExprNode::CallExpr(node) => has(&node.callee) || node.args.iter().any(|arg| has(&arg.value)),
        ExprNode::MethodCallExpr(node) => has(&node.receiver) || node.args.iter().any(|arg| has(&arg.value)),
        ExprNode::TupleExpr(node) => node.elements.iter().any(has),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node).into_iter().any(has),
        ExprNode::ArrayRepeatExpr(node) => has(&node.value) || has(&node.count),
        ExprNode::RecordExpr(node) => node.fields.iter().any(|field| has(&field.value)),
        ExprNode::IfExpr(node) => has(&node.cond) || has(&node.then_expr) || has(&node.else_expr),
        ExprNode::AttributedExpr(node) => has(&node.expr),
        ExprNode::EntryExpr(node) => has(&node.expr),
        ExprNode::MoveExpr(node) => has(&node.place),
        ExprNode::AddressOfExpr(node) => has(&node.place),
        _ => false,
    }
}

/// A refinement written on a parameter's type may not speak of `self`.
fn type_contains_inline_parameter_self_constraint(ty: &TypePtr) -> bool {
    use ast::TypeNode as T;
    let Some(t) = ty.as_deref() else {
        return false;
    };
    let has = type_contains_inline_parameter_self_constraint;
    match &t.node {
        T::TypeRefine(node) => expr_contains_identifier(&node.predicate, "self") || has(&node.base),
        T::TypePermType(node) => has(&node.base),
        T::TypeUnion(node) => node.types.iter().any(has),
        T::TypeFunc(node) => node.params.iter().any(|param| has(&param.r#type)) || has(&node.ret),
        T::TypeClosure(node) => {
            node.params.iter().any(|param| has(&param.r#type))
                || has(&node.ret)
                || node.deps_opt.as_ref().is_some_and(|deps| deps.iter().any(|dep| has(&dep.r#type)))
        }
        T::TypeTuple(node) => node.elements.iter().any(has),
        T::TypeArray(node) => has(&node.element),
        T::TypeSlice(node) => has(&node.element),
        T::TypeSafePtr(node) => has(&node.element),
        T::TypeRawPtr(node) => has(&node.element),
        T::TypePathType(node) => node.generic_args.iter().any(has),
        T::TypeApply(node) => node.args.iter().any(has),
        T::TypeModalState(node) => node.generic_args.iter().any(has),
        T::SpliceExprNode(node) => expr_contains_identifier(&node.expr, "self"),
        T::TypeRange(node) => has(&node.base),
        T::TypeRangeInclusive(node) => has(&node.base),
        T::TypeRangeFrom(node) => has(&node.base),
        T::TypeRangeTo(node) => has(&node.base),
        T::TypeRangeToInclusive(node) => has(&node.base),
        _ => false,
    }
}

fn lower_type_with_wf(ctx: &ScopeContext<'_>, ty: &TypePtr) -> Result<TypeRef, Option<&'static str>> {
    let lowered = lower_type(ctx, ty)?;
    type_wf(ctx, &lowered)?;
    Ok(lowered)
}

/// See `BuildProcedureSignature`.
pub fn build_procedure_signature(ctx: &ScopeContext<'_>, params: &[ast::Param], return_type_opt: &TypePtr) -> Result<Signature, Option<&'static str>> {
    let mut func_params = Vec::with_capacity(params.len());
    let mut bindings = Vec::with_capacity(params.len());
    for param in params {
        if type_contains_inline_parameter_self_constraint(&param.r#type) {
            return Err(Some("E-TYP-1956"));
        }
        let lowered = lower_type_with_wf(ctx, &param.r#type)?;
        func_params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: lowered.clone() });
        bindings.push((param.name.clone(), lowered));
    }
    let return_type = if return_type_opt.is_some() { lower_type_with_wf(ctx, return_type_opt)? } else { make_type_prim("()") };
    Ok(Signature { func_type: make_type_func(func_params, return_type.clone()), return_type, bindings })
}

fn is_bare_test_authority_type(ty: &TypePtr) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node),
        Some(ast::TypeNode::TypePathType(path)) if path.generic_args.is_empty() && path.path == ["TestAuthority"])
}

fn validate_test_procedure_shape(decl: &ast::ProcedureDecl) -> Option<&'static str> {
    if !has_attribute(&decl.attrs, attrs::TEST) {
        return None;
    }
    let has_type_params = decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty());
    if decl.body.is_none() || has_type_params || !decl.visibility_explicit || decl.return_type_opt.is_none() || decl.params.len() > 1 {
        return Some("E-TST-0104");
    }
    if decl.contract.as_ref().is_none_or(|contract| contract.postcondition.is_none()) {
        return Some("E-TST-0106");
    }
    if decl.params.len() == 1 && !is_bare_test_authority_type(&decl.params[0].r#type) {
        return Some("E-TST-0105");
    }
    None
}

/// A body that has to produce a value ends in a `return`, possibly inside key blocks.
pub fn has_explicit_return(block: &ast::Block) -> bool {
    if block.tail_opt.is_some() {
        return false;
    }
    match block.stmts.last() {
        Some(Stmt::ReturnStmt(_)) => true,
        Some(Stmt::KeyBlockStmt(node)) => node.body.as_deref().is_some_and(has_explicit_return),
        _ => false,
    }
}

fn procedure_body_failure_span(body: &ast::Block) -> Span {
    match (body.stmts.first(), body.tail_opt.as_deref()) {
        (Some(first), _) => super::stmt::block::span_of_stmt(first).clone(),
        (None, Some(tail)) => tail.span.clone(),
        (None, None) => body.span.clone(),
    }
}

/// A call that needed `move` is also reported as a missing move of the binding, once.
fn emit_borrow_move_missing_from_recent_diags(diags: &mut DiagnosticStream, begin_index: usize) {
    let recent = &diags[begin_index.min(diags.len())..];
    let saw_call_move_missing = recent.iter().any(|diag| diag.code == "E-SEM-2534");
    let saw_borrow_move_missing = recent.iter().any(|diag| diag.code == "E-MOD-2411");
    if saw_call_move_missing && !saw_borrow_move_missing {
        emit_resolved_typecheck_diagnostic(diags, "E-MOD-2411", None, "");
    }
}

/// A body that failed typing is also borrow-checked, and what that finds is reported
/// alongside unless it is the same rule.
fn emit_supplemental_borrow_diag(
    ctx: &ScopeContext<'_>,
    module_path: &[String],
    params: &[ast::Param],
    body: &ast::BlockPtr,
    diags: &mut DiagnosticStream,
    primary_diag_id: Option<&'static str>,
) {
    if body.is_none() {
        return;
    }
    let primary_is_call_move_missing = primary_diag_id == Some("E-SEM-2534") || diags.last().is_some_and(|diag| diag.code == "E-SEM-2534");
    let bind = bind_check_body(ctx, module_path, params, body, None);
    if let (false, Some(diag_id)) = (bind.ok, bind.diag_id) {
        if primary_diag_id != Some(diag_id) {
            emit_resolved_typecheck_diagnostic(diags, diag_id, bind.span, "");
        }
        return;
    }
    if primary_is_call_move_missing {
        emit_resolved_typecheck_diagnostic(diags, "E-MOD-2411", None, "");
    }
}

fn type_params_of(decl: &ast::ProcedureDecl) -> &[ast::TypeParam] {
    decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..])
}

/// `public procedure main(move ctx: Context) -> i32`, or a record of capabilities.
pub(crate) fn main_sig_ok(ctx: &ScopeContext<'_>, decl: &ast::ProcedureDecl) -> bool {
    if decl.vis != ast::Visibility::Public {
        return false;
    }
    let [param] = decl.params.as_slice() else {
        return false;
    };
    if param.mode.is_some_and(|mode| mode != ast::ParamMode::Move) {
        return false;
    }
    if param.r#type.is_none() || !is_context_bundle_type(ctx, &param.r#type) {
        return false;
    }
    matches!(decl.return_type_opt.as_deref().map(|ty| &ty.node), Some(ast::TypeNode::TypePrim(prim)) if prim.name == "i32")
}

/// The one-line fix for a `main` that only misspells its signature.
pub(crate) fn main_signature_fix_its(decl: &ast::ProcedureDecl) -> Vec<SubDiagnostic> {
    let is_context_syntax = |ty: &ast::Type| match &ty.node {
        ast::TypeNode::TypePathType(path) => path.generic_args.is_empty() && is_context_type_path(&path.path),
        ast::TypeNode::TypePrim(prim) => id_eq(&prim.name, "Context"),
        _ => false,
    };
    let [param] = decl.params.as_slice() else {
        return Vec::new();
    };
    let (Some(param_type), Some(ret)) = (param.r#type.as_deref(), decl.return_type_opt.as_deref()) else {
        return Vec::new();
    };
    let ret_is_i32 = matches!(&ret.node, ast::TypeNode::TypePrim(prim) if id_eq(&prim.name, "i32"));
    if decl.name != "main" || !decl.attrs.is_empty() || !type_params_of(decl).is_empty() || !is_context_syntax(param_type) || !ret_is_i32 {
        return Vec::new();
    }
    let mut span = decl.span.clone();
    span.end_offset = param_type.span.end_offset;
    span.end_line = param_type.span.end_line;
    span.end_col = param_type.span.end_col;
    if span.end_offset <= span.start_offset {
        return Vec::new();
    }
    vec![SubDiagnostic {
        kind: SubDiagnosticKind::FixIt,
        message: "Fix main signature".to_string(),
        span: Some(span),
        fix_text: Some(format!("public procedure main(move {}: Context)", param.name)),
        ..Default::default()
    }]
}

/// A `#dynamic` written on a predicate itself, not on the declaration.
pub(crate) fn contract_clause_has_direct_dynamic_attr(contract: &ast::ContractClause) -> bool {
    let has_dynamic = |expr: &ExprPtr| {
        matches!(expr.as_deref().map(|expr| &expr.node), Some(ExprNode::AttributedExpr(attributed)) if has_attribute(&attributed.attrs, attrs::DYNAMIC))
    };
    has_dynamic(&contract.precondition) || has_dynamic(&contract.postcondition)
}

/// See `TypeProcedureDecl`.
pub fn type_procedure_decl(
    ctx: &ScopeContext<'_>,
    decl: &ast::ProcedureDecl,
    module_path: &[String],
    diags: &Rc<RefCell<DiagnosticStream>>,
) -> DeclOutcome {
    let failed = |diag_id: &'static str| DeclOutcome::Failed(DeclFailure::rule(diag_id));
    let pending = |what: &str| DeclOutcome::Pending(what.to_string());

    let attr_validation = validate_attributes(&decl.attrs, AttributeTarget::Procedure);
    if !attr_validation.ok {
        return DeclOutcome::Failed(DeclFailure::of(attr_validation.diag_id));
    }
    if let Some(diag_id) = validate_test_procedure_shape(decl) {
        return failed(diag_id);
    }
    if decl.return_type_opt.is_none() {
        return failed("WF-ProcedureDecl-MissingReturnType");
    }
    let mut names = HashSet::new();
    if !decl.params.iter().all(|param| names.insert(&param.name)) {
        return failed("E-SEM-2713");
    }
    if decl.params.iter().any(|param| id_eq(&param.name, "self")) {
        return failed("E-SEM-3011");
    }
    // The attributes of the foreign interface, and what they demand of the signature.
    let foreign = [attrs::EXPORT, attrs::HOST_EXPORT, attrs::MANGLE, attrs::UNWIND].iter().any(|name| has_attribute(&decl.attrs, name));
    if foreign {
        return pending("ProcFfiAttrs");
    }
    if has_attribute(&decl.attrs, attrs::STATIC) {
        return failed("E-MOD-2452");
    }
    if has_attribute(&decl.attrs, attrs::INLINE) {
        return pending("InlineAlways");
    }
    if decl.name == "main" {
        if !type_params_of(decl).is_empty() {
            return failed("E-MOD-2432");
        }
        if !main_sig_ok(ctx, decl) {
            return DeclOutcome::Failed(DeclFailure { diag_id: Some("E-MOD-2431"), children: main_signature_fix_its(decl), ..Default::default() });
        }
    }
    let type_params = decl.generic_params.as_ref().map_or(&[][..], |params| &params.params[..]);
    if let Err(diag_id) = process_generic_params(ctx, type_params) {
        return DeclOutcome::Failed(DeclFailure::of(diag_id));
    }
    let mut proc_ctx = ctx.clone();
    proc_ctx.scopes = bind_type_params(ctx, &decl.generic_params);
    let sig = match build_procedure_signature(&proc_ctx, &decl.params, &decl.return_type_opt) {
        Ok(sig) => sig,
        Err(Some(REFINEMENT_WF_PENDING)) => return pending("RefinementWF"),
        Err(diag_id) => return DeclOutcome::Failed(DeclFailure::of(diag_id)),
    };

    let mut env = TypeEnv::default();
    env.scopes.push(Default::default());
    for (name, ty) in &sig.bindings {
        let binding =
            TypeBinding { r#type: ty.clone(), storage_type: ty.clone(), provenance_kind: BindingProvenanceSeedKind::Param, ..Default::default() };
        env.scopes[0].insert(id_key_of(name), binding);
    }

    if let Some(contract) = &decl.contract {
        if contract_clause_has_direct_dynamic_attr(contract) {
            return failed("E-CON-0410");
        }
        let contract_ctx = ContractContext {
            scope_ctx: Some(&proc_ctx),
            params: sig.bindings.iter().cloned().collect(),
            moved_params: decl.params.iter().filter(|param| param.mode == Some(ast::ParamMode::Move)).map(|param| param.name.clone()).collect(),
            return_type: sig.return_type.clone(),
            ..Default::default()
        };
        let intrinsics_check = validate_contract_intrinsics(contract, &contract_ctx);
        if !intrinsics_check.ok {
            return DeclOutcome::Failed(DeclFailure::of(intrinsics_check.diag_id));
        }
        let contract_check = check_contract_well_formed(&contract_ctx, contract);
        if !contract_check.ok {
            return DeclOutcome::Failed(DeclFailure::of(contract_check.diag_id));
        }
    }

    if let Some(body) = decl.body.as_deref() {
        let is_unit = type_equiv(&sig.return_type, &make_type_prim("()"));
        if !is_unit && !has_explicit_return(body) {
            return failed("E-TYP-1507");
        }
        if matches!(sig.return_type.as_deref().map(|ty| &ty.node), Some(TypeNode::Opaque { .. })) {
            return pending("OpaqueReturn");
        }
        proc_ctx.diagnostics = Some(diags.clone());
        let env = Rc::new(RefCell::new(env));
        let ancestors = [DynamicScopeAncestor { attrs: &decl.attrs, span: &decl.span }];
        let type_ctx = StmtTypeContext {
            return_type: sig.return_type.clone(),
            ffi_export_boundary: has_attribute(&decl.attrs, attrs::EXPORT) || has_attribute(&decl.attrs, attrs::HOST_EXPORT),
            diags: Some(diags.clone()),
            env_ref: Some(env.clone()),
            contract_dynamic: compute_dynamic_context(&body.span, &ancestors),
            test_postcondition_runtime: has_attribute(&decl.attrs, attrs::TEST),
            contract: decl.contract.as_ref(),
            ..Default::default()
        };
        // The predicates are typed as expressions: pure, and `bool`.
        if let Some(contract) = &decl.contract {
            let check_predicate = |predicate: &ExprPtr, phase: ContractPhase| -> Option<&'static str> {
                predicate.as_ref()?;
                let contract_type_ctx = StmtTypeContext { contract_phase: phase, require_pure: true, ..type_ctx.clone() };
                reset_scaffolding();
                let typed = type_expr(&proc_ctx, &contract_type_ctx, predicate, &env.borrow());
                if !typed.ok {
                    return Some(typed.diag_id.unwrap_or("WF-Contract"));
                }
                (!type_equiv(&typed.r#type, &make_type_prim("bool"))).then_some("WF-Contract")
            };
            for (predicate, phase) in [(&contract.precondition, ContractPhase::Precondition), (&contract.postcondition, ContractPhase::Postcondition)] {
                let diag = check_predicate(predicate, phase);
                if let Some(what) = take_pending() {
                    return DeclOutcome::Pending(what.to_string());
                }
                if let Some(diag_id) = diag {
                    return failed(diag_id);
                }
            }
        }
        let type_expr_fn = |inner: &ExprPtr| type_expr(&proc_ctx, &type_ctx, inner, &env.borrow().clone());
        let type_ident_fn = |ident: &str| type_identifier_expr(&proc_ctx, &env.borrow(), ident);
        let type_place_fn = |inner: &ExprPtr| type_place(&proc_ctx, &type_ctx, inner, &env.borrow().clone());
        let diag_count_before_body = diags.borrow().len();
        let start_env = env.borrow().clone();
        reset_scaffolding();
        let body_result = type_block(&proc_ctx, &type_ctx, body, &start_env, &type_expr_fn, &type_ident_fn, &type_place_fn, Some(&env));
        if let Some(what) = take_pending() {
            return DeclOutcome::Pending(what.to_string());
        }
        emit_borrow_move_missing_from_recent_diags(&mut diags.borrow_mut(), diag_count_before_body);
        if !body_result.ok {
            let diag_id = body_result.diag_id.or(Some("E-TYP-1530"));
            emit_supplemental_borrow_diag(&proc_ctx, module_path, &decl.params, &decl.body, &mut diags.borrow_mut(), diag_id);
            if let Some(what) = take_pending() {
                return DeclOutcome::Pending(what.to_string());
            }
            let diag_detail = if body_result.diag_detail.is_empty() {
                "procedure body typing failed without statement-level diagnostic".to_string()
            } else {
                body_result.diag_detail
            };
            return DeclOutcome::Failed(DeclFailure {
                diag_id,
                diag_detail,
                diag_span: body_result.diag_span.or_else(|| Some(procedure_body_failure_span(body))),
                diagnostic_obligation_ids: body_result.diagnostic_obligation_ids.iter().map(|id| id.to_string()).collect(),
                children: Vec::new(),
            });
        }
        if body_result.r#type.is_some() {
            let sub = subtyping(&proc_ctx, &body_result.r#type, &sig.return_type);
            if !sub.ok {
                return DeclOutcome::Failed(DeclFailure::of(sub.diag_id));
            }
            if !sub.subtype {
                return failed("E-SEM-3161");
            }
        }
        let bind = bind_check_body(&proc_ctx, module_path, &decl.params, &decl.body, None);
        if let Some(what) = take_pending() {
            return DeclOutcome::Pending(what.to_string());
        }
        if !bind.ok {
            return DeclOutcome::Failed(DeclFailure::of(bind.diag_id));
        }
        let prov = prov_bind_check(&proc_ctx, module_path, &decl.params, &decl.body, None, Some(diags));
        if let Some(what) = take_pending() {
            return DeclOutcome::Pending(what.to_string());
        }
        if !prov.ok {
            return DeclOutcome::Failed(DeclFailure::of(prov.diag_id));
        }
    }
    if has_attribute(&decl.attrs, attrs::DYNAMIC) {
        return pending("DynamicNoRuntime");
    }
    DeclOutcome::Ok
}
