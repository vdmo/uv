//! Closures and pipelines. A closure that captures nothing is a plain function.

use std::cell::RefCell;
use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_source::ast;

use crate::context::ScopeContext;
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::closure_capture::analyze_closure_capture_info;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::{intro_all, push_scope, TypeEnv};
use crate::typing::type_expr::{check_expr_against, type_expr};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm;
use crate::typing::types::*;

/// Types a closure. An expected closure type supplies parameter and return types the
/// closure leaves out.
pub fn type_closure_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::ClosureExpr,
    whole: &ast::ExprPtr,
    env: &TypeEnv,
    expected_type: Option<&TypeRef>,
) -> ExprTypeResult {
    let failed = |diag_id: Option<&'static str>| ExprTypeResult::failed(diag_id);
    let expected_stripped = expected_type.map(strip_perm).unwrap_or_default();
    let expected_closure = match expected_stripped.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Closure { params, ret, deps_opt }) => Some((params, ret, deps_opt)),
        _ => None,
    };
    if expected_closure.is_some_and(|(params, _, _)| params.len() != expr.params.len()) {
        return failed(Some("E-SEM-2532"));
    }
    let mut param_types: Vec<(bool, TypeRef)> = Vec::with_capacity(expr.params.len());
    let mut param_binds: Vec<(String, TypeRef)> = Vec::with_capacity(expr.params.len());
    for (index, param) in expr.params.iter().enumerate() {
        let param_type = if param.type_opt.is_some() {
            lower_type(ctx, &param.type_opt).unwrap_or_default()
        } else {
            expected_closure.map(|(params, _, _)| params[index].1.clone()).unwrap_or_default()
        };
        if param_type.is_none() {
            return failed(Some("Infer-Closure-Params-Err"));
        }
        param_types.push((param.move_capture, param_type.clone()));
        param_binds.push((param.name.clone(), param_type));
    }
    let capture_info = analyze_closure_capture_info(whole, env, &None).unwrap_or_default();

    let body_env = match intro_all(&push_scope(env), &param_binds, ast::Mutability::Let, false) {
        Ok(body_env) => body_env,
        Err(error) => return failed(error),
    };
    let mut ret_type: TypeRef = None;
    if expr.ret_type_opt.is_some() {
        match lower_type(ctx, &expr.ret_type_opt) {
            Ok(lowered) if lowered.is_some() => ret_type = lowered,
            Ok(_) => return failed(None),
            Err(diag_id) => return failed(diag_id),
        }
    }
    let mut body_type_ctx = type_ctx.clone();
    body_type_ctx.env_ref = Some(Rc::new(RefCell::new(body_env.clone())));
    body_type_ctx.in_shared_capturing_closure = capture_info.captures_shared;
    if ret_type.is_none() {
        if let Some((_, ret, _)) = expected_closure {
            ret_type = ret.clone();
        }
    }
    if ret_type.is_some() {
        body_type_ctx.return_type = ret_type.clone();
        let body_check = check_expr_against(ctx, &body_type_ctx, &expr.body, &ret_type, &body_env);
        if !body_check.ok {
            return failed(body_check.diag_id);
        }
    } else {
        let body_type = type_expr(ctx, &body_type_ctx, &expr.body, &body_env);
        if !body_type.ok {
            return failed(body_type.diag_id);
        }
        ret_type = body_type.r#type;
    }

    // Capturing shared data is worth a warning, once per closure body.
    if let (true, Some(diags), Some(body)) = (capture_info.captures_shared, &type_ctx.diags, expr.body.as_deref()) {
        let already = diags.borrow().iter().any(|diag| diag.code == "W-CON-0009" && diag.span.as_ref() == Some(&body.span));
        if !already {
            if let Some(diag) = make_diagnostic_by_id("W-CON-0009", Some(body.span.clone())) {
                emit(&mut diags.borrow_mut(), diag);
            }
        }
    }

    if capture_info.captures_any {
        return ExprTypeResult::typed(make_type(TypeNode::Closure { params: param_types, ret: ret_type, deps_opt: None }));
    }
    if let Some((_, _, deps_opt)) = expected_closure {
        return ExprTypeResult::typed(make_type(TypeNode::Closure { params: param_types, ret: ret_type, deps_opt: deps_opt.clone() }));
    }
    let fn_params = param_types
        .into_iter()
        .map(|(is_move, param_type)| TypeFuncParam { mode: is_move.then_some(ParamMode::Move), r#type: param_type })
        .collect();
    ExprTypeResult::typed(make_type_func(fn_params, ret_type))
}

/// `value => callable`: the callable takes the value as its one argument.
pub fn type_pipeline_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::PipelineExpr, env: &TypeEnv) -> ExprTypeResult {
    let failed = |diag_id: &'static str| ExprTypeResult::failed(Some(diag_id));
    if expr.lhs.is_none() || expr.rhs.is_none() {
        return failed("E-SEM-2538");
    }
    let lhs_result = type_expr(ctx, type_ctx, &expr.lhs, env);
    if lhs_result.r#type.is_none() {
        return failed("E-SEM-2538");
    }
    let rhs_result = type_expr(ctx, type_ctx, &expr.rhs, env);
    let mut callable = strip_perm(&rhs_result.r#type);
    if callable.is_none() {
        return failed("E-SEM-2538");
    }
    for _ in 0..16 {
        let Some(t) = callable.as_deref() else {
            break;
        };
        let (Some(path), Some(args)) = (applied_type_path(t), applied_type_args(t)) else {
            break;
        };
        match expand_type_alias_apply(ctx, path, args) {
            Err(diag_id) => return ExprTypeResult::failed(diag_id),
            Ok(None) => break,
            Ok(expanded) => callable = strip_perm(&expanded),
        }
    }
    let (param_type, ret) = match callable.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Func { params, ret }) => match &params[..] {
            [param] => (param.r#type.clone(), ret.clone()),
            _ => return failed("E-SEM-2539"),
        },
        Some(TypeNode::Closure { params, ret, .. }) => match &params[..] {
            [param] => (param.1.clone(), ret.clone()),
            _ => return failed("E-SEM-2539"),
        },
        _ => return failed("E-SEM-2538"),
    };
    let sub = subtyping(ctx, &lhs_result.r#type, &param_type);
    if !sub.ok || !sub.subtype {
        return failed("E-SEM-2539");
    }
    ExprTypeResult::typed(ret)
}
