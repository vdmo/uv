//! Calls: how arguments are passed and checked against parameters.
//!
//! An argument to a `move` parameter that names storage must say `move`; an argument
//! to a reference parameter must be a place when it names storage, and is then typed as
//! a place. Everything else is checked as a value of the parameter's type.

use std::sync::Arc;

use uv_core::span::{span_range, Span};
use uv_source::ast::{self, Arg, ArgPassKind, ExprNode, ExprPtr, Stmt};

use crate::context::{ScopeContext, TypeDecl};
use crate::generics::monomorphize::{instantiate_type, TypeSubst};
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::resolve::scopes_lookup::{resolve_type_name, resolve_value_name};
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::callbacks::{CheckResult, ExprTypeFn, PlaceTypeFn};
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::subtyping::argument_type_compatible;
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::{strip_perm, strip_perm_and_refine};
use crate::typing::types::*;

/// Checks an argument expression against a parameter type.
pub type ArgCheckFn<'f> = &'f dyn Fn(&ExprPtr, &TypeRef) -> CheckResult;

#[derive(Debug, Clone, Default)]
pub struct CallTypeResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub r#type: TypeRef,
    /// The callee is a record named without arguments: its default construction.
    pub record_callee: bool,
    pub diag_detail: String,
    pub diag_span: Option<Span>,
}

/// What the caller already knows about the callee.
#[derive(Debug, Clone, Default)]
pub struct CallCalleeFacts {
    pub extern_callee: Option<bool>,
    pub callee_type: Option<ExprTypeResult>,
}

fn failed(diag_id: Option<&'static str>) -> CallTypeResult {
    CallTypeResult { diag_id, ..Default::default() }
}

/// A failed check that names no rule, or only the general type mismatch.
fn is_expected_type_mismatch(diag_id: Option<&'static str>) -> bool {
    matches!(diag_id, None | Some("E-SEM-2526"))
}

/// Whether the span lies within an `unsafe` block of its file.
pub fn is_in_unsafe_span(ctx: &ScopeContext<'_>, span: &Span) -> bool {
    if span.file.is_empty() {
        return false;
    }
    let range = span_range(span);
    ctx.sigma.unsafe_spans_by_file.get(&*span.file).is_some_and(|spans| {
        spans.iter().map(span_range).any(|unsafe_range| range.0 >= unsafe_range.0 && range.1 <= unsafe_range.1)
    })
}

pub(crate) fn find_module<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::ASTModule> {
    let key = path_key_of(path);
    ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key)
}

pub(crate) fn find_procedure_in_module<'m>(module: &'m ast::ASTModule, name: &str) -> Option<&'m ast::ProcedureDecl> {
    module.items.iter().find_map(|item| match item {
        ast::ASTItem::ProcedureDecl(proc) if id_eq(&proc.name, name) => Some(proc),
        _ => None,
    })
}

pub(crate) fn find_extern_procedure_in_module<'m>(module: &'m ast::ASTModule, name: &str) -> Option<&'m ast::ExternProcDecl> {
    module
        .items
        .iter()
        .filter_map(|item| match item {
            ast::ASTItem::ExternBlock(block) => Some(block),
            _ => None,
        })
        .flat_map(|block| &block.items)
        .map(|ast::ExternItem::ExternProcDecl(proc)| proc)
        .find(|proc| id_eq(&proc.name, name))
}

/// Whether the callee names a foreign procedure: one in its module wins over an
/// ordinary procedure of the same name, and failing both any module's will do.
pub fn is_extern_callee(ctx: &ScopeContext<'_>, callee: &ExprPtr) -> bool {
    let Some(callee) = callee.as_deref() else {
        return false;
    };
    let mut candidate_names: Vec<&str> = Vec::new();
    let entity;
    let module_path = match &callee.node {
        ExprNode::IdentifierExpr(ident) => {
            candidate_names.push(&ident.name);
            entity = resolve_value_name(ctx, &ident.name);
            match &entity {
                Some(entity) if entity.origin_opt.is_some() => {
                    if let Some(target) = entity.target_opt.as_ref().filter(|target| !id_eq(target, &ident.name)) {
                        candidate_names.push(target);
                    }
                    entity.origin_opt.clone().unwrap_or_default()
                }
                _ => ctx.current_module.clone(),
            }
        }
        ExprNode::QualifiedNameExpr(node) => {
            candidate_names.push(&node.name);
            node.path.clone()
        }
        ExprNode::QualifiedApplyExpr(node) => {
            candidate_names.push(&node.name);
            node.path.clone()
        }
        ExprNode::PathExpr(node) => {
            candidate_names.push(&node.name);
            if node.path.is_empty() {
                ctx.current_module.clone()
            } else {
                node.path.clone()
            }
        }
        _ => return false,
    };
    let has_extern =
        |module: &ast::ASTModule| candidate_names.iter().any(|name| find_extern_procedure_in_module(module, name).is_some());
    if let Some(module) = find_module(ctx, &module_path) {
        if has_extern(module) {
            return true;
        }
        if candidate_names.iter().any(|name| find_procedure_in_module(module, name).is_some()) {
            return false;
        }
    }
    ctx.sigma.mods.iter().any(has_extern)
}

/// A place as argument passing sees it: a name, a field or element, or a dereference
/// of one.
pub fn is_place_expr_for_call(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(
            ExprNode::IdentifierExpr(_)
            | ExprNode::FieldAccessExpr(_)
            | ExprNode::TupleAccessExpr(_)
            | ExprNode::IndexAccessExpr(_),
        ) => true,
        Some(ExprNode::DerefExpr(deref)) => is_place_expr_for_call(&deref.value),
        _ => false,
    }
}

/// Whether the expression's value lives in storage that outlasts the call: a place, or
/// a conditional or block whose every result is one.
pub fn has_source_provenance(expr: &ExprPtr) -> bool {
    if is_place_expr_for_call(expr) {
        return true;
    }
    let block = |block: &ast::BlockPtr| block.as_deref().is_some_and(|block| has_source_provenance(&block.tail_opt));
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::IfExpr(node)) => {
            node.else_expr.is_some() && has_source_provenance(&node.then_expr) && has_source_provenance(&node.else_expr)
        }
        Some(ExprNode::IfCaseExpr(node)) => {
            node.cases.iter().all(|arm| has_source_provenance(&arm.body)) && has_source_provenance(&node.else_expr)
        }
        Some(ExprNode::IfIsExpr(node)) => has_source_provenance(&node.then_expr) && has_source_provenance(&node.else_expr),
        Some(ExprNode::BlockExpr(node)) => block(&node.block),
        Some(ExprNode::UnsafeBlockExpr(node)) => block(&node.block),
        Some(ExprNode::AttributedExpr(node)) => has_source_provenance(&node.expr),
        _ => false,
    }
}

/// The same for a statement, by the expressions it holds.
pub fn stmt_has_source_provenance(stmt: &Stmt) -> bool {
    let block = |block: &ast::BlockPtr| block.as_deref().is_some_and(|block| has_source_provenance(&block.tail_opt));
    match stmt {
        Stmt::LetStmt(node) => has_source_provenance(&node.binding.init),
        Stmt::VarStmt(node) => has_source_provenance(&node.binding.init),
        Stmt::AssignStmt(node) => has_source_provenance(&node.place) || has_source_provenance(&node.value),
        Stmt::CompoundAssignStmt(node) => has_source_provenance(&node.place) || has_source_provenance(&node.value),
        Stmt::ExprStmt(node) => has_source_provenance(&node.value),
        Stmt::DeferStmt(node) => block(&node.body),
        Stmt::RegionStmt(node) => has_source_provenance(&node.opts_opt) || block(&node.body),
        Stmt::FrameStmt(node) => block(&node.body),
        Stmt::ReturnStmt(node) => has_source_provenance(&node.value_opt),
        Stmt::BreakStmt(node) => has_source_provenance(&node.value_opt),
        Stmt::UnsafeBlockStmt(node) => block(&node.body),
        Stmt::KeyBlockStmt(node) => block(&node.body),
        _ => false,
    }
}

pub fn missing_required_move_for_consuming(mode: Option<ParamMode>, arg: &Arg) -> bool {
    mode == Some(ParamMode::Move) && arg.pass == ArgPassKind::Ref && has_source_provenance(&arg.value)
}

pub fn uses_call_temp_for_consuming(mode: Option<ParamMode>, arg: &Arg) -> bool {
    mode == Some(ParamMode::Move) && arg.pass == ArgPassKind::Ref && !has_source_provenance(&arg.value)
}

fn make_expr(span: Span, node: ExprNode) -> ExprPtr {
    Some(Arc::new(ast::Expr { span, node }))
}

/// The argument as the expression that is typed: `copy value`, `move place`, or the
/// value itself.
pub fn arg_pass_expr(arg: &Arg) -> ExprPtr {
    let value_span = || arg.value.as_deref().map(|value| value.span.clone());
    let span = || if arg.span.file.is_empty() { value_span().unwrap_or_else(|| arg.span.clone()) } else { arg.span.clone() };
    match arg.pass {
        ArgPassKind::Copy => make_expr(span(), ExprNode::CopyExpr(ast::CopyExpr { value: arg.value.clone() })),
        ArgPassKind::Move if is_place_expr_for_call(&arg.value) => {
            let span = if arg.span.file.is_empty() { value_span().unwrap_or_default() } else { arg.span.clone() };
            make_expr(span, ExprNode::MoveExpr(ast::MoveExpr { place: arg.value.clone() }))
        }
        _ => arg.value.clone(),
    }
}

fn strip_perm_once(ty: &TypeRef) -> &TypeRef {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. }) => base,
        _ => ty,
    }
}

/// `#layout(packed)` on the record.
pub(crate) fn is_packed_record(ctx: &ScopeContext<'_>, path: &[String]) -> bool {
    let Some(TypeDecl::Record(record)) = ctx.sigma.types.get(&path_key_of(path)) else {
        return false;
    };
    record.attrs.iter().filter(|attr| attr.name.full_name == "layout").flat_map(|attr| &attr.args).any(|arg| {
        matches!(&arg.value, ast::AttributeArgValue::Token(token) if token.lexeme == "packed")
    })
}

/// Whether a reference may be taken to the place: not to a field of a packed record
/// outside `unsafe`, and an indexed place must be indexed by `usize`.
fn addr_of_ok(
    ctx: &ScopeContext<'_>,
    expr: &ExprPtr,
    type_expr: ExprTypeFn<'_>,
    check_expr: Option<ArgCheckFn<'_>>,
) -> Result<(), Option<&'static str>> {
    let Some(e) = expr.as_deref().filter(|_| is_place_expr_for_call(expr)) else {
        return Err(None);
    };
    if let ExprNode::FieldAccessExpr(field) = &e.node {
        let base_type = type_expr(&field.base);
        if !base_type.ok {
            return Err(base_type.diag_id);
        }
        if let Some(TypeNode::Path { path, .. }) = strip_perm_once(&base_type.r#type).as_deref().map(|ty| &ty.node) {
            if is_packed_record(ctx, path) && !is_in_unsafe_span(ctx, &e.span) {
                return Err(Some("E-TYP-2105"));
            }
        }
    }
    let ExprNode::IndexAccessExpr(index) = &e.node else {
        return Ok(());
    };
    if let Some(check_expr) = check_expr {
        let checked = check_expr(&index.index, &make_type_prim("usize"));
        if checked.ok {
            return Ok(());
        }
        if !is_expected_type_mismatch(checked.diag_id) {
            return Err(checked.diag_id);
        }
    }
    let idx_type = type_expr(&index.index);
    if !idx_type.ok {
        return Err(idx_type.diag_id);
    }
    if matches!(strip_perm_once(&idx_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "usize") {
        return Ok(());
    }
    let base_type = type_expr(&index.base);
    if !base_type.ok {
        return Err(base_type.diag_id);
    }
    Err(Some(match strip_perm_once(&base_type.r#type).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Array { .. }) => "Index-Array-NonUsize",
        Some(TypeNode::Slice(_)) => "Index-Slice-NonUsize",
        _ => "Index-NonIndexable",
    }))
}

/// The record a callee without arguments names, if it names one.
fn record_callee_decl<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr, args: &[Arg]) -> Option<&'c ast::RecordDecl> {
    if !args.is_empty() {
        return None;
    }
    let full = match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => {
            let entity = resolve_type_name(ctx, &node.name)?;
            let mut full = entity.origin_opt?;
            full.push(entity.target_opt.unwrap_or_else(|| node.name.clone()));
            full
        }
        ExprNode::PathExpr(node) => [&node.path[..], std::slice::from_ref(&node.name)].concat(),
        _ => return None,
    };
    match ctx.sigma.types.get(&path_key_of(&full))? {
        TypeDecl::Record(record) => Some(record),
        _ => None,
    }
}

pub fn is_record_callee(ctx: &ScopeContext<'_>, callee: &ExprPtr, args: &[Arg]) -> bool {
    record_callee_decl(ctx, callee, args).is_some()
}

pub fn arg_diagnostic_span(arg: &Arg) -> Option<Span> {
    if !arg.span.file.is_empty() {
        return Some(arg.span.clone());
    }
    arg.value.as_deref().map(|value| value.span.clone())
}

/// A function type, directly or through aliases without parameters.
pub(crate) fn is_function_value_type(ctx: &ScopeContext<'_>, ty: &TypeRef, depth: u32) -> bool {
    if depth > 32 {
        return false;
    }
    let stripped = strip_perm(ty);
    let (path, generic_args) = match stripped.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Func { .. }) => return true,
        Some(TypeNode::Path { path, generic_args }) => (path, generic_args),
        _ => return false,
    };
    if !generic_args.is_empty() {
        return false;
    }
    let decl = match &path[..] {
        [] => return false,
        [name] => {
            let Some(entity) = resolve_type_name(ctx, name) else {
                return false;
            };
            let Some(mut resolved) = entity.origin_opt else {
                return false;
            };
            resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
            ctx.sigma.types.get(&path_key_of(&resolved))
        }
        _ => ctx.sigma.types.get(&path_key_of(path)),
    };
    let Some(TypeDecl::TypeAlias(alias)) = decl.filter(|decl| matches!(decl, TypeDecl::TypeAlias(alias) if alias.generic_params.is_none())) else {
        return false;
    };
    lower_type(ctx, &alias.r#type).is_ok_and(|lowered| is_function_value_type(ctx, &lowered, depth + 1))
}

/// The callee's type under permissions, refinements and aliases.
fn normalize_callee_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<TypeRef, Option<&'static str>> {
    let mut current = strip_perm_and_refine(ty);
    for _ in 0..16 {
        let Some(TypeNode::Path { path, generic_args }) = current.as_deref().map(|ty| &ty.node) else {
            break;
        };
        let expanded = expand_type_alias_apply(ctx, path, generic_args)?;
        if expanded.is_none() {
            break;
        }
        current = strip_perm_and_refine(&expanded);
    }
    Ok(current)
}

fn check_failure(checked: CheckResult, arg: &Arg) -> CallTypeResult {
    CallTypeResult {
        diag_id: if is_expected_type_mismatch(checked.diag_id) { Some("E-SEM-2533") } else { checked.diag_id },
        diag_detail: checked.diag_detail,
        diag_span: checked.diag_span.or_else(|| arg_diagnostic_span(arg)),
        ..Default::default()
    }
}

fn type_failure(typed: ExprTypeResult, arg: &Arg) -> CallTypeResult {
    CallTypeResult {
        diag_id: typed.diag_id,
        diag_detail: typed.diag_detail,
        diag_span: typed.diag_span.or_else(|| arg_diagnostic_span(arg)),
        ..Default::default()
    }
}

/// Checks the arguments of a call against its parameters. The result is `ok` without a
/// type; the caller knows the return type.
pub fn check_call_arguments(
    ctx: &ScopeContext<'_>,
    params: &[TypeFuncParam],
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    check_expr: Option<ArgCheckFn<'_>>,
) -> CallTypeResult {
    if params.len() != args.len() {
        return CallTypeResult {
            diag_id: Some("E-SEM-2532"),
            diag_detail: format!("expected {} args, found {}", params.len(), args.len()),
            ..Default::default()
        };
    }
    let at = |diag_id: Option<&'static str>, arg: &Arg| CallTypeResult { diag_id, diag_span: arg_diagnostic_span(arg), ..Default::default() };
    for (param, arg) in params.iter().zip(args) {
        if missing_required_move_for_consuming(param.mode, arg) {
            return at(Some("E-SEM-2534"), arg);
        }
    }
    for (param, arg) in params.iter().zip(args) {
        if param.mode.is_none() && arg.pass == ArgPassKind::Move {
            return at(Some("E-SEM-2535"), arg);
        }
    }
    let function_value: Vec<bool> = params.iter().map(|param| is_function_value_type(ctx, &param.r#type, 0)).collect();
    let mut arg_types: Vec<TypeRef> = Vec::with_capacity(args.len());
    for (index, (param, arg)) in params.iter().zip(args).enumerate() {
        if param.mode.is_none() && arg.pass != ArgPassKind::Copy {
            let has_source_prov = has_source_provenance(&arg.value);
            let by_reference = has_source_prov && !function_value[index];
            if by_reference && !is_place_expr_for_call(&arg.value) {
                return at(Some("E-TYP-1603"), arg);
            }
            if let (true, Some(type_place)) = (by_reference, type_place) {
                let place_type = type_place(&arg.value);
                if !place_type.ok {
                    return at(place_type.diag_id, arg);
                }
                arg_types.push(place_type.r#type);
                continue;
            }
            if let (false, Some(check_expr)) = (by_reference, check_expr) {
                let checked = check_expr(&arg.value, &param.r#type);
                if !checked.ok {
                    return check_failure(checked, arg);
                }
                arg_types.push(param.r#type.clone());
                continue;
            }
            let arg_type = type_expr(&arg.value);
            if !arg_type.ok {
                return type_failure(arg_type, arg);
            }
            arg_types.push(arg_type.r#type);
            continue;
        }
        // A copied argument, or one to a `move` parameter.
        let arg_expr = arg_pass_expr(arg);
        if let (Some(_), true, Some(check_expr)) = (param.mode, uses_call_temp_for_consuming(param.mode, arg), check_expr) {
            let checked = check_expr(&arg_expr, &param.r#type);
            if !checked.ok {
                return check_failure(checked, arg);
            }
            arg_types.push(param.r#type.clone());
            continue;
        }
        let arg_type = type_expr(&arg_expr);
        if !arg_type.ok {
            return type_failure(arg_type, arg);
        }
        arg_types.push(arg_type.r#type);
    }
    for ((param, arg), arg_type) in params.iter().zip(args).zip(&arg_types) {
        let sub = argument_type_compatible(ctx, arg_type, &param.r#type, param.mode);
        if !sub.ok {
            return at(sub.diag_id, arg);
        }
        if !sub.subtype {
            return CallTypeResult {
                diag_id: Some("E-SEM-2533"),
                diag_detail: format!("expected type {}, found {}", type_to_string(&param.r#type), type_to_string(arg_type)),
                diag_span: arg_diagnostic_span(arg),
                ..Default::default()
            };
        }
    }
    for (index, (param, arg)) in params.iter().zip(args).enumerate() {
        if param.mode == Some(ParamMode::Move) || function_value[index] || !has_source_provenance(&arg.value) {
            continue;
        }
        if let Err(diag_id) = addr_of_ok(ctx, &arg.value, type_expr, check_expr) {
            return at(diag_id, arg);
        }
    }
    CallTypeResult { ok: true, ..Default::default() }
}

/// The callee as a function: its parameters and return type.
enum Callable {
    Func(Vec<TypeFuncParam>, TypeRef),
    NotCallable,
}

fn resolve_callee(
    ctx: &ScopeContext<'_>,
    callee: &ExprPtr,
    type_expr: ExprTypeFn<'_>,
    callee_facts: Option<&CallCalleeFacts>,
    closures_too: bool,
) -> Result<Callable, CallTypeResult> {
    let Some(callee_expr) = callee.as_deref() else {
        return Err(CallTypeResult::default());
    };
    let is_extern =
        callee_facts.and_then(|facts| facts.extern_callee).unwrap_or_else(|| is_extern_callee(ctx, callee));
    if is_extern && !is_in_unsafe_span(ctx, &callee_expr.span) {
        return Err(failed(Some("Call-Extern-Unsafe-Err")));
    }
    let callee_type = callee_facts.and_then(|facts| facts.callee_type.clone()).unwrap_or_else(|| type_expr(callee));
    if !callee_type.ok {
        return Err(CallTypeResult { diag_id: callee_type.diag_id, diag_span: callee_type.diag_span, ..Default::default() });
    }
    let normalized = normalize_callee_type(ctx, &callee_type.r#type).map_err(failed)?;
    Ok(match normalized.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Func { params, ret }) => Callable::Func(params.clone(), ret.clone()),
        Some(TypeNode::Closure { params, ret, .. }) if closures_too => {
            let params = params
                .iter()
                .map(|(is_move, ty)| TypeFuncParam { mode: is_move.then_some(ParamMode::Move), r#type: ty.clone() })
                .collect();
            Callable::Func(params, ret.clone())
        }
        _ => Callable::NotCallable,
    })
}

/// Types a call of a function or closure value.
pub fn type_call(
    ctx: &ScopeContext<'_>,
    callee: &ExprPtr,
    args: &[Arg],
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    check_expr: Option<ArgCheckFn<'_>>,
    callee_facts: Option<&CallCalleeFacts>,
) -> CallTypeResult {
    let (params, ret) = match resolve_callee(ctx, callee, type_expr, callee_facts, true) {
        Err(failure) => return failure,
        Ok(Callable::Func(params, ret)) => (params, ret),
        Ok(Callable::NotCallable) => {
            if record_callee_decl(ctx, callee, args).is_some() {
                return CallTypeResult { record_callee: true, ..Default::default() };
            }
            return failed(Some("E-SEM-2531"));
        }
    };
    let checked = check_call_arguments(ctx, &params, args, type_expr, type_place, check_expr);
    if !checked.ok {
        return checked;
    }
    CallTypeResult { ok: true, r#type: ret, ..Default::default() }
}

/// Types a call of a generic function with its type arguments known.
#[allow(clippy::too_many_arguments)]
pub fn type_call_with_subst(
    ctx: &ScopeContext<'_>,
    callee: &ExprPtr,
    args: &[Arg],
    subst: &TypeSubst,
    type_expr: ExprTypeFn<'_>,
    type_place: Option<PlaceTypeFn<'_>>,
    check_expr: Option<ArgCheckFn<'_>>,
    callee_facts: Option<&CallCalleeFacts>,
) -> CallTypeResult {
    let (params, ret) = match resolve_callee(ctx, callee, type_expr, callee_facts, false) {
        Err(failure) => return failure,
        Ok(Callable::Func(params, ret)) => (params, ret),
        Ok(Callable::NotCallable) => return failed(Some("E-SEM-2531")),
    };
    if params.len() != args.len() {
        return CallTypeResult {
            diag_id: Some("E-SEM-2532"),
            diag_detail: format!("expected {} args, found {}", params.len(), args.len()),
            ..Default::default()
        };
    }
    let subst_params: Vec<TypeFuncParam> =
        params.iter().map(|param| TypeFuncParam { mode: param.mode, r#type: instantiate_type(&param.r#type, subst) }).collect();
    let checked = check_call_arguments(ctx, &subst_params, args, type_expr, type_place, check_expr);
    if !checked.ok {
        return checked;
    }
    CallTypeResult { ok: true, r#type: instantiate_type(&ret, subst), ..Default::default() }
}
