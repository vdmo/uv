//! `@result` and `@entry` inside contracts: where they may appear, and what an `@entry`
//! expression may hold. See `ValidateContractIntrinsics`.

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr};

use super::purity::{ContractCheckResult, ContractContext};
use crate::caps::cap_methods::lookup_context_method_sig;
use crate::resolve::scopes::id_eq;
use crate::typing::types::{TypeNode, TypeRef};

/// Whether a type is, or holds, a capability.
pub fn type_contains_capability(ty: &TypeRef) -> bool {
    if crate::typing::type_predicates::is_capability_type(ty) {
        return true;
    }
    let Some(t) = ty.as_deref() else {
        return false;
    };
    match &t.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => type_contains_capability(base),
        TypeNode::Union(members) => members.iter().any(type_contains_capability),
        TypeNode::Func { params, ret } => params.iter().any(|param| type_contains_capability(&param.r#type)) || type_contains_capability(ret),
        TypeNode::Closure { params, ret, deps_opt } => {
            params.iter().any(|(_, ty)| type_contains_capability(ty))
                || type_contains_capability(ret)
                || deps_opt.as_ref().is_some_and(|deps| deps.iter().any(|dep| type_contains_capability(&dep.r#type)))
        }
        TypeNode::Tuple(elements) => elements.iter().any(type_contains_capability),
        TypeNode::Array { element, .. } | TypeNode::Ptr { element, .. } => type_contains_capability(element),
        TypeNode::Slice(element) => type_contains_capability(element),
        _ => false,
    }
}

/// The fields a context bundle has.
pub fn is_context_field_name(name: &str) -> bool {
    ["io", "net", "heap", "sys", "reactor", "time", "cpu", "gpu", "inline"].iter().any(|field| id_eq(name, field))
}

fn is_capability_receiver_name(name: &str) -> bool {
    name == "ctx" || name == "context" || is_context_field_name(name) || name == "system"
}

fn is_ctx_name(name: &str) -> bool {
    name == "ctx" || name == "context"
}

/// A receiver that by its name looks like a capability or the context.
pub fn is_likely_capability_receiver(receiver: &ExprPtr) -> bool {
    match receiver.as_deref().map(|receiver| &receiver.node) {
        Some(ExprNode::IdentifierExpr(node)) => is_capability_receiver_name(&node.name),
        Some(ExprNode::PathExpr(node)) => node.path.is_empty() && is_capability_receiver_name(&node.name),
        Some(ExprNode::FieldAccessExpr(node)) => {
            let on_ctx = match node.base.as_deref().map(|base| &base.node) {
                Some(ExprNode::IdentifierExpr(base)) => is_ctx_name(&base.name),
                Some(ExprNode::PathExpr(base)) => base.path.is_empty() && is_ctx_name(&base.name),
                _ => false,
            };
            (on_ctx && is_context_field_name(&node.name)) || is_capability_receiver_name(&node.name)
        }
        _ => false,
    }
}

/// A method call on the context that hands out a capability.
pub fn is_context_capability_call(receiver: &ExprPtr, name: &str) -> bool {
    let recv_is_ctx = match receiver.as_deref().map(|receiver| &receiver.node) {
        Some(ExprNode::IdentifierExpr(node)) => is_ctx_name(&node.name),
        Some(ExprNode::PathExpr(node)) => node.path.is_empty() && is_ctx_name(&node.name),
        _ => false,
    };
    recv_is_ctx && lookup_context_method_sig(name, None).is_some()
}

/// The sub-expressions the intrinsic analyses all look into, in evaluation order.
fn shared_children(e: &ast::Expr) -> Vec<&ExprPtr> {
    match &e.node {
        ExprNode::BinaryExpr(node) => vec![&node.lhs, &node.rhs],
        ExprNode::UnaryExpr(node) => vec![&node.value],
        ExprNode::FieldAccessExpr(node) => vec![&node.base],
        ExprNode::TupleAccessExpr(node) => vec![&node.base],
        ExprNode::IndexAccessExpr(node) => vec![&node.base, &node.index],
        ExprNode::CallExpr(node) => std::iter::once(&node.callee).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::MethodCallExpr(node) => std::iter::once(&node.receiver).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::CastExpr(node) => vec![&node.value],
        ExprNode::IfExpr(node) => vec![&node.cond, &node.then_expr, &node.else_expr],
        ExprNode::IfCaseExpr(node) => std::iter::once(&node.scrutinee).chain(node.cases.iter().map(|clause| &clause.body)).chain([&node.else_expr]).collect(),
        ExprNode::IfIsExpr(node) => vec![&node.scrutinee, &node.then_expr, &node.else_expr],
        ExprNode::TupleExpr(node) => node.elements.iter().collect(),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node),
        _ => Vec::new(),
    }
}

/// The shared forms plus the ones a given analysis also looks into.
fn with_extras(e: &ast::Expr, deref: bool, repeat_and_record: bool) -> Vec<&ExprPtr> {
    let mut out = shared_children(e);
    match &e.node {
        ExprNode::DerefExpr(node) if deref => out.push(&node.value),
        ExprNode::ArrayRepeatExpr(node) if repeat_and_record => out.extend([&node.value, &node.count]),
        ExprNode::RecordExpr(node) if repeat_and_record => out.extend(node.fields.iter().map(|field| &field.value)),
        _ => {}
    }
    out
}

pub fn find_result_exprs<'e>(expr: &'e ExprPtr, out: &mut Vec<&'e ast::Expr>) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    if matches!(e.node, ExprNode::ResultExpr(_)) {
        out.push(e);
        return;
    }
    let mut children = with_extras(e, true, false);
    match &e.node {
        ExprNode::EntryExpr(node) => children.push(&node.expr),
        ExprNode::RangeExpr(node) => children.extend([&node.lhs, &node.rhs]),
        ExprNode::AddressOfExpr(node) => children.push(&node.place),
        _ => {}
    }
    for child in children {
        find_result_exprs(child, out);
    }
}

pub fn find_entry_exprs<'e>(expr: &'e ExprPtr, out: &mut Vec<(&'e ast::EntryExpr, &'e ast::Expr)>) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    if let ExprNode::EntryExpr(node) = &e.node {
        out.push((node, e));
    }
    let mut children = with_extras(e, false, false);
    if let ExprNode::RangeExpr(node) = &e.node {
        children.extend([&node.lhs, &node.rhs]);
    }
    for child in children {
        find_entry_exprs(child, out);
    }
}

/// Whether an `@entry` expression uses a capability.
fn entry_expr_has_capability_op(expr: &ExprPtr, ctx: Option<&ContractContext<'_, '_>>) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    if let ExprNode::MethodCallExpr(node) = &e.node {
        let receiver_name = match node.receiver.as_deref().map(|receiver| &receiver.node) {
            Some(ExprNode::IdentifierExpr(ident)) => Some(&ident.name),
            Some(ExprNode::PathExpr(path)) if path.path.is_empty() => Some(&path.name),
            _ => None,
        };
        if let (Some(ctx), Some(name)) = (ctx, receiver_name) {
            if ctx.params.get(name).is_some_and(type_contains_capability) {
                return true;
            }
        }
        if is_likely_capability_receiver(&node.receiver) || is_context_capability_call(&node.receiver, &node.name) {
            return true;
        }
    }
    let mut children = with_extras(e, true, true);
    match &e.node {
        ExprNode::SyncExpr(node) => children.push(&node.value),
        ExprNode::YieldExpr(node) => children.push(&node.value),
        ExprNode::YieldFromExpr(node) => children.push(&node.value),
        ExprNode::WaitExpr(node) => children.push(&node.handle),
        _ => {}
    }
    children.into_iter().any(|child| entry_expr_has_capability_op(child, ctx))
}

/// Whether an `@entry` expression does something that is not a pure read.
fn entry_expr_has_side_effect_op(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    if matches!(
        e.node,
        ExprNode::SyncExpr(_)
            | ExprNode::YieldExpr(_)
            | ExprNode::YieldFromExpr(_)
            | ExprNode::SpawnExpr(_)
            | ExprNode::WaitExpr(_)
            | ExprNode::FenceExpr(_)
            | ExprNode::ParallelExpr(_)
            | ExprNode::DispatchExpr(_)
            | ExprNode::RaceExpr(_)
            | ExprNode::AllExpr(_)
            | ExprNode::AllocExpr(_)
            | ExprNode::MoveExpr(_)
            | ExprNode::TransmuteExpr(_)
            | ExprNode::PropagateExpr(_)
            | ExprNode::LoopInfiniteExpr(_)
            | ExprNode::LoopConditionalExpr(_)
            | ExprNode::LoopIterExpr(_)
            | ExprNode::BlockExpr(_)
            | ExprNode::UnsafeBlockExpr(_)
    ) {
        return true;
    }
    with_extras(e, true, true).into_iter().any(entry_expr_has_side_effect_op)
}

/// The first mention of a moved parameter in an `@entry` expression.
fn entry_expr_references_moved_param(expr: &ExprPtr, moved_params: &std::collections::HashSet<String>) -> Option<Span> {
    let e = expr.as_deref()?;
    if moved_params.is_empty() {
        return None;
    }
    match &e.node {
        ExprNode::IdentifierExpr(node) => return moved_params.contains(&node.name).then(|| e.span.clone()),
        ExprNode::PathExpr(node) => return (node.path.is_empty() && moved_params.contains(&node.name)).then(|| e.span.clone()),
        _ => {}
    }
    let mut children = with_extras(e, true, true);
    match &e.node {
        ExprNode::EntryExpr(node) => children.push(&node.expr),
        ExprNode::AddressOfExpr(node) => children.push(&node.place),
        ExprNode::RangeExpr(node) => children.extend([&node.lhs, &node.rhs]),
        _ => {}
    }
    children.into_iter().find_map(|child| entry_expr_references_moved_param(child, moved_params))
}

/// Whether the same inputs always give the same value.
fn is_deterministic_expr(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return true;
    };
    let det = is_deterministic_expr;
    match &e.node {
        ExprNode::FieldAccessExpr(node) => det(&node.base),
        ExprNode::TupleAccessExpr(node) => det(&node.base),
        ExprNode::IndexAccessExpr(node) => det(&node.base) && det(&node.index),
        ExprNode::DerefExpr(node) => det(&node.value),
        ExprNode::BinaryExpr(node) => det(&node.lhs) && det(&node.rhs),
        ExprNode::UnaryExpr(node) => det(&node.value),
        ExprNode::CastExpr(node) => det(&node.value),
        ExprNode::IfExpr(node) => det(&node.cond) && det(&node.then_expr) && (node.else_expr.is_none() || det(&node.else_expr)),
        ExprNode::CallExpr(_) | ExprNode::MethodCallExpr(_) | ExprNode::YieldExpr(_) | ExprNode::SpawnExpr(_) | ExprNode::WaitExpr(_) => false,
        _ => true,
    }
}

fn fail(diag_id: &'static str, span: Option<Span>) -> ContractCheckResult {
    ContractCheckResult { ok: false, diag_id: Some(diag_id), span }
}

fn ok() -> ContractCheckResult {
    ContractCheckResult { ok: true, diag_id: None, span: None }
}

fn validate_result_intrinsic(expr: &ExprPtr, ctx: &ContractContext<'_, '_>) -> ContractCheckResult {
    if !ctx.is_postcondition {
        return fail("E-SEM-2806", expr.as_deref().map(|expr| expr.span.clone()));
    }
    ok()
}

fn validate_entry_intrinsic(expr: &ExprPtr, ctx: &ContractContext<'_, '_>) -> ContractCheckResult {
    if !ctx.is_postcondition {
        return fail("E-SEM-2852", expr.as_deref().map(|expr| expr.span.clone()));
    }
    let mut entries = Vec::new();
    find_entry_exprs(expr, &mut entries);
    for (entry, _) in entries {
        let Some(inner) = entry.expr.as_deref() else {
            return fail("E-SEM-2852", None);
        };
        if let Some(span) = entry_expr_references_moved_param(&entry.expr, &ctx.moved_params) {
            return fail("E-SEM-2807", Some(span));
        }
        if entry_expr_has_capability_op(&entry.expr, Some(ctx)) {
            return fail("E-CON-0415", Some(inner.span.clone()));
        }
        if entry_expr_has_side_effect_op(&entry.expr) || !is_deterministic_expr(&entry.expr) {
            return fail("E-CON-0416", Some(inner.span.clone()));
        }
    }
    ok()
}

/// See `ValidateContractIntrinsics`.
pub fn validate_contract_intrinsics(contract: &ast::ContractClause, ctx: &ContractContext<'_, '_>) -> ContractCheckResult {
    if contract.precondition.is_some() {
        let mut results = Vec::new();
        find_result_exprs(&contract.precondition, &mut results);
        if let Some(first) = results.first() {
            return fail("E-SEM-2806", Some(first.span.clone()));
        }
        let mut entries = Vec::new();
        find_entry_exprs(&contract.precondition, &mut entries);
        if let Some((entry, _)) = entries.first() {
            return fail("E-SEM-2852", entry.expr.as_deref().map(|inner| inner.span.clone()));
        }
    }
    if contract.postcondition.is_some() {
        let post_ctx = ContractContext { is_postcondition: true, ..ctx.clone() };
        let result_validation = validate_result_intrinsic(&contract.postcondition, &post_ctx);
        if !result_validation.ok {
            return result_validation;
        }
        let entry_validation = validate_entry_intrinsic(&contract.postcondition, &post_ctx);
        if !entry_validation.ok {
            return entry_validation;
        }
    }
    ok()
}

/// Whether the expression holds `@result` or `@entry` anywhere it is looked for.
pub fn contains_result(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    match &e.node {
        ExprNode::ResultExpr(_) => true,
        ExprNode::BinaryExpr(node) => contains_result(&node.lhs) || contains_result(&node.rhs),
        ExprNode::UnaryExpr(node) => contains_result(&node.value),
        ExprNode::PipelineExpr(node) => contains_result(&node.lhs) || contains_result(&node.rhs),
        ExprNode::CallExpr(node) => contains_result(&node.callee) || node.args.iter().any(|arg| contains_result(&arg.value)),
        ExprNode::MethodCallExpr(node) => contains_result(&node.receiver) || node.args.iter().any(|arg| contains_result(&arg.value)),
        _ => false,
    }
}

pub fn contains_entry(expr: &ExprPtr) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    match &e.node {
        ExprNode::EntryExpr(_) => true,
        ExprNode::BinaryExpr(node) => contains_entry(&node.lhs) || contains_entry(&node.rhs),
        ExprNode::UnaryExpr(node) => contains_entry(&node.value),
        ExprNode::PipelineExpr(node) => contains_entry(&node.lhs) || contains_entry(&node.rhs),
        ExprNode::CallExpr(node) => contains_entry(&node.callee) || node.args.iter().any(|arg| contains_entry(&arg.value)),
        _ => false,
    }
}
