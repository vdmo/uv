//! Resolving the names used by expressions, statements and blocks.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use uv_core::diagnostics::{SubDiagnostic, SubDiagnosticKind};
use uv_core::span::Span;
use uv_core::symbols::string_of_path;
use uv_source::ast::*;
use uv_source::attributes::{attrs, has_attribute};
use uv_source::parser::make_modal_ref;

use super::collect_toplevel::pat_names_ptr;
use super::resolve_contracts::resolve_loop_invariant_opt;
use super::resolve_pattern::resolve_pattern;
use super::resolve_qual::{resolve_enum_variant, resolve_qualified_form, resolve_record_path, VariantKind};
use super::resolve_types::{resolve_type, resolve_type_path};
use crate::language_service::LanguageSymbolKind;
use super::resolver::*;
use super::scopes::{id_eq, id_key_of, path_key_of};
use super::scopes_intro::intro;
use super::scopes_lookup::{resolve_qualified, resolve_type_name, resolve_value_name};
use crate::caps::builtin_paths::lookup_builtin_record_ctor_path;
use crate::context::*;
use crate::modal::builtin_modal_intrinsics::{is_builtin_modal_member_name, is_builtin_modal_type_path};

/// Whether any compile-time capability is in scope.
fn is_comptime_resolution_env(ctx: &ScopeContext<'_>) -> bool {
    ["diagnostics", "introspect", "emitter", "files", "target"].iter().any(|name| resolve_value_name(ctx, name).is_some())
}

/// The capabilities a compile-time block may name; two of them need an attribute.
pub fn build_comptime_capability_scope(attr_list: &[AttributeItem]) -> Scope {
    let mut scope = Scope::new();
    let mut add = |name: &str| {
        scope.entry(id_key_of(name)).or_insert_with(value_entity);
    };
    add("introspect");
    add("diagnostics");
    if has_attribute(attr_list, attrs::FILES) {
        add("files");
    }
    if has_attribute(attr_list, attrs::EMIT) {
        add("emitter");
    }
    scope
}

fn rebuilt(expr: &Expr, node: ExprNode) -> Res<ExprPtr> {
    Ok(make_expr(&expr.span, node))
}

fn resolve_comptime_expr_with_attrs(
    ctx: &mut ResolveContext<'_, '_>,
    expr_ptr: &ExprPtr,
    expr: &Expr,
    node: &ComptimeExpr,
    outer_attrs: &[AttributeItem],
) -> Res<ExprPtr> {
    if node.body.is_none() {
        return Ok(expr_ptr.clone());
    }
    let mut merged_attrs = node.attrs_opt.clone().unwrap_or_default();
    merged_attrs.extend(outer_attrs.iter().cloned());
    let scope = build_comptime_capability_scope(&merged_attrs);
    let body = with_scope(ctx, scope, |ctx| resolve_expr(ctx, &node.body))?;
    rebuilt(expr, ExprNode::ComptimeExpr(ComptimeExpr { body, attrs_opt: Some(merged_attrs) }))
}

/// A chain of one operator is resolved without recursing along it; chains can be long.
fn resolve_binary_chain(ctx: &mut ResolveContext<'_, '_>, root: &Arc<Expr>, op: &str) -> Res<ExprPtr> {
    let mut stack: Vec<(Arc<Expr>, bool)> = vec![(root.clone(), false)];
    let mut resolved: HashMap<*const Expr, ExprPtr> = HashMap::new();
    while let Some((frame, finish)) = stack.pop() {
        let chain_link = match &frame.node {
            ExprNode::BinaryExpr(binary) if binary.op == op => Some(binary),
            _ => None,
        };
        let (Some(binary), true) =
            (chain_link, chain_link.is_some_and(|binary| binary.lhs.is_some() && binary.rhs.is_some()))
        else {
            if chain_link.is_some() {
                return Err(ResError::with_detail("ResolveExpr-Ident-Err", &frame.span, "malformed binary expression"));
            }
            let leaf = resolve_expr(ctx, &Some(frame.clone()))?;
            resolved.insert(Arc::as_ptr(&frame), leaf);
            continue;
        };
        let (Some(lhs), Some(rhs)) = (&binary.lhs, &binary.rhs) else {
            continue;
        };
        if !finish {
            stack.push((frame.clone(), true));
            stack.push((rhs.clone(), false));
            stack.push((lhs.clone(), false));
            continue;
        }
        let (Some(new_lhs), Some(new_rhs)) = (resolved.get(&Arc::as_ptr(lhs)), resolved.get(&Arc::as_ptr(rhs))) else {
            let detail = "internal error: unresolved binary-chain operand";
            return Err(ResError::detailed(None, Some(frame.span.clone()), detail));
        };
        let node = BinaryExpr { op: binary.op.clone(), lhs: new_lhs.clone(), rhs: new_rhs.clone() };
        resolved.insert(Arc::as_ptr(&frame), make_expr(&frame.span, ExprNode::BinaryExpr(node)));
    }
    match resolved.remove(&Arc::as_ptr(root)) {
        Some(out) => Ok(out),
        None => {
            Err(ResError::detailed(None, Some(root.span.clone()), "internal error: unresolved binary-chain root"))
        }
    }
}

/// Byte-wise edit distance, cut off once it must exceed `max_dist`.
fn edit_distance(a: &[u8], b: &[u8], max_dist: usize) -> usize {
    let (a, b) = if a.len() > b.len() { (b, a) } else { (a, b) };
    if b.len() - a.len() > max_dist {
        return max_dist + 1;
    }
    let mut row: Vec<usize> = (0..=a.len()).collect();
    for j in 1..=b.len() {
        let mut prev = row[0];
        row[0] = j;
        let mut row_min = row[0];
        for i in 1..=a.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let val = (row[i] + 1).min(row[i - 1] + 1).min(prev + cost);
            prev = row[i];
            row[i] = val;
            row_min = row_min.min(val);
        }
        if row_min > max_dist {
            return max_dist + 1;
        }
    }
    row[a.len()]
}

/// The closest name in scope, innermost scope first. Among equally close names the first
/// in the scope's iteration order wins.
fn suggest_name(ctx: &ScopeContext<'_>, name: &str) -> String {
    let threshold = (name.len() / 3).clamp(1, 3);
    let mut best = String::new();
    let mut best_dist = threshold + 1;
    for key in ctx.scopes.iter().flat_map(|scope| scope.keys()) {
        if key == name {
            continue;
        }
        let dist = edit_distance(name.as_bytes(), key.as_bytes(), threshold);
        if dist < best_dist {
            best_dist = dist;
            best = key.clone();
        }
    }
    best
}

fn help(message: String) -> SubDiagnostic {
    SubDiagnostic { kind: SubDiagnosticKind::Help, message, span: None, fix_text: None, label: None }
}

/// An unresolved value name, with a spelling suggestion and a hint for names borrowed
/// from other languages.
fn unresolved_value_name(ctx: &ScopeContext<'_>, name: &str, span: &Span) -> ResError {
    let mut err = ResError::with_detail("ResolveExpr-Ident-Err", span, format!("unresolved name '{name}'"));
    let suggestion = suggest_name(ctx, name);
    if !suggestion.is_empty() {
        err.children.push(SubDiagnostic {
            kind: SubDiagnosticKind::Help,
            message: format!("did you mean `{suggestion}`?"),
            span: Some(span.clone()),
            fix_text: Some(suggestion),
            label: None,
        });
    }
    let hint = match name {
        "Option" | "Some" | "None" => Some("Ultraviolet uses union types (`T | ()`) instead of Option".to_string()),
        "Result" | "Ok" | "Err" => Some("Ultraviolet uses union types (`T | E`) instead of Result".to_string()),
        "Vec" => Some("Ultraviolet uses array types `[T; n]` or slice types `[T]`".to_string()),
        "Box" | "Rc" | "Arc" => Some("Ultraviolet uses `region` and `Ptr<T>` for heap allocation".to_string()),
        "String" => Some("Ultraviolet uses `string@View` and `string@Managed`".to_string()),
        "println" | "print" | "eprintln" | "eprint" => {
            Some(format!("Ultraviolet does not have `{name}!`; use capabilities from Context for I/O"))
        }
        _ => None,
    };
    err.children.extend(hint.map(help));
    err
}

fn qualified_name_detail(path: &[String], name: &str) -> String {
    format!("unresolved qualified name `{}`", string_of_path(&full_path(path, name)))
}

/// Introduces a pattern's names into the innermost scope. A failure is reported at the
/// pattern.
fn bind_pattern(ctx: &mut ResolveContext<'_, '_>, pat: &PatternPtr) -> Res<()> {
    let Some(pattern) = pat.as_deref() else {
        return Ok(());
    };
    let names = pat_names_ptr(pat);
    let mut seen = HashSet::new();
    if !names.iter().all(|name| seen.insert(id_key_of(name))) {
        return Err(ResError::at("Pat-Dup-Err", &pattern.span));
    }
    for name in &names {
        let entity = ctx.local_language_entity(name, &pattern.span, LanguageSymbolKind::Variable, "local binding");
        let introduced = intro(ctx.ctx, name, &entity);
        if !introduced.ok {
            return Err(ResError::from_id(introduced.diag_id, Some(pattern.span.clone())));
        }
    }
    Ok(())
}

/// A bare identifier pattern that names a type, which `if .. is` does not accept.
fn bare_type_pattern_span(scope: &ScopeContext<'_>, pattern: &PatternPtr) -> Option<Span> {
    let pattern = pattern.as_deref()?;
    match &pattern.node {
        PatternNode::IdentifierPattern(ident) if resolve_type_name(scope, &ident.name).is_some() => {
            Some(pattern.span.clone())
        }
        _ => None,
    }
}

fn resolve_expr_opt(ctx: &mut ResolveContext<'_, '_>, expr_opt: &ExprPtr) -> Res<ExprPtr> {
    match expr_opt {
        Some(_) => resolve_expr(ctx, expr_opt),
        None => Ok(None),
    }
}

fn resolve_type_opt(ctx: &mut ResolveContext<'_, '_>, type_opt: &TypePtr) -> Res<TypePtr> {
    match type_opt {
        Some(_) => resolve_type(ctx, type_opt),
        None => Ok(None),
    }
}

fn resolve_block_opt(ctx: &mut ResolveContext<'_, '_>, block_opt: &BlockPtr) -> Res<BlockPtr> {
    match block_opt {
        Some(block) => Ok(Some(Arc::new(resolve_block(ctx, block)?))),
        None => Ok(None),
    }
}

fn resolve_args(ctx: &mut ResolveContext<'_, '_>, args: &[Arg]) -> Res<Vec<Arg>> {
    args.iter()
        .map(|arg| Ok(Arg { pass: arg.pass, value: resolve_expr(ctx, &arg.value)?, span: arg.span.clone() }))
        .collect()
}

fn resolve_exprs(ctx: &mut ResolveContext<'_, '_>, exprs: &[ExprPtr]) -> Res<Vec<ExprPtr>> {
    exprs.iter().map(|expr| resolve_expr(ctx, expr)).collect()
}

/// Type arguments in expressions keep the rule and span of a failure, not its detail.
fn resolve_type_args(ctx: &mut ResolveContext<'_, '_>, types: &[TypePtr]) -> Res<Vec<TypePtr>> {
    types.iter().map(|ty| resolve_type(ctx, ty).map_err(ResError::id_span)).collect()
}

fn resolve_field_inits(ctx: &mut ResolveContext<'_, '_>, fields: &[FieldInit]) -> Res<Vec<FieldInit>> {
    fields
        .iter()
        .map(|field| {
            Ok(FieldInit { name: field.name.clone(), value: resolve_expr(ctx, &field.value)?, span: field.span.clone() })
        })
        .collect()
}

fn resolve_enum_payload(ctx: &mut ResolveContext<'_, '_>, payload_opt: &Option<EnumPayload>) -> Res<Option<EnumPayload>> {
    Ok(match payload_opt {
        None => None,
        Some(EnumPayload::EnumPayloadParen(node)) => {
            Some(EnumPayload::EnumPayloadParen(EnumPayloadParen { elements: resolve_exprs(ctx, &node.elements)? }))
        }
        Some(EnumPayload::EnumPayloadBrace(node)) => {
            Some(EnumPayload::EnumPayloadBrace(EnumPayloadBrace { fields: resolve_field_inits(ctx, &node.fields)? }))
        }
    })
}

fn resolve_key_path_expr(ctx: &mut ResolveContext<'_, '_>, path: &KeyPathExpr) -> Res<KeyPathExpr> {
    let Some(root) = resolve_value_name(ctx.ctx, &path.root) else {
        return Err(unresolved_value_name(ctx.ctx, &path.root, &path.span));
    };
    ctx.record_reference(&path.root, &path.span, &root);
    let mut segs = Vec::with_capacity(path.segs.len());
    for seg in &path.segs {
        segs.push(match seg {
            KeySeg::KeySegField(_) => seg.clone(),
            KeySeg::KeySegIndex(node) => {
                KeySeg::KeySegIndex(KeySegIndex { marked: node.marked, expr: resolve_expr(ctx, &node.expr)? })
            }
        });
    }
    Ok(KeyPathExpr { root: path.root.clone(), segs, span: path.span.clone() })
}

fn resolve_spawn_opt(ctx: &mut ResolveContext<'_, '_>, opt: &SpawnOption) -> Res<SpawnOption> {
    if opt.value.is_none() || opt.kind == SpawnOptionKind::Name {
        return Ok(opt.clone());
    }
    Ok(SpawnOption { kind: opt.kind, value: resolve_expr(ctx, &opt.value)?, span: opt.span.clone() })
}

fn resolve_parallel_opt(ctx: &mut ResolveContext<'_, '_>, opt: &ParallelOption) -> Res<ParallelOption> {
    if opt.value.is_none() || opt.kind == ParallelOptionKind::Name {
        return Ok(opt.clone());
    }
    Ok(ParallelOption { kind: opt.kind, value: resolve_expr(ctx, &opt.value)?, span: opt.span.clone() })
}

fn resolve_dispatch_opt(ctx: &mut ResolveContext<'_, '_>, opt: &DispatchOption) -> Res<DispatchOption> {
    let chunk = match opt.kind {
        DispatchOptionKind::Reduce | DispatchOptionKind::Ordered => return Ok(opt.clone()),
        DispatchOptionKind::Chunk => true,
        DispatchOptionKind::Workgroup => false,
    };
    let value = if chunk { &opt.chunk_expr } else { &opt.workgroup_expr };
    if value.is_none() {
        return Ok(opt.clone());
    }
    let resolved = resolve_expr(ctx, value)?;
    let mut out = opt.clone();
    if chunk {
        out.chunk_expr = resolved;
    } else {
        out.workgroup_expr = resolved;
    }
    Ok(out)
}

fn resolve_record_target(ctx: &mut ResolveContext<'_, '_>, target: &RecordExprTarget) -> Res<RecordExprTarget> {
    match target {
        RecordExprTarget::Path(path) => {
            Ok(RecordExprTarget::Path(resolve_type_path(ctx, path).map_err(ResError::no_children)?))
        }
        RecordExprTarget::ModalStateRef(node) => {
            let path = resolve_type_path(ctx, &node.path).map_err(ResError::no_children)?;
            let generic_args = resolve_type_args(ctx, &node.generic_args)?;
            let modal_ref = make_modal_ref(path.clone(), generic_args.clone());
            Ok(RecordExprTarget::ModalStateRef(ModalStateRef { modal_ref, path, generic_args, state: node.state.clone() }))
        }
    }
}

/// A callee without arguments may also name a record, whose literal form is `Name()`.
fn resolve_callee(ctx: &mut ResolveContext<'_, '_>, callee: &ExprPtr, args: &[Arg]) -> Res<ExprPtr> {
    let Some(callee_expr) = callee.as_deref() else {
        return Err(ResError::new("ResolveExpr-Ident-Err", None));
    };
    match &callee_expr.node {
        ExprNode::IdentifierExpr(node) => {
            if let Some(ent) = resolve_value_name(ctx.ctx, &node.name) {
                ctx.record_reference(&node.name, &callee_expr.span, &ent);
                return Ok(callee.clone());
            }
            if args.is_empty() {
                if lookup_builtin_record_ctor_path(&node.name).is_some() {
                    return Ok(callee.clone());
                }
                if let Some(type_ent) = resolve_type_name(ctx.ctx, &node.name) {
                    let name = type_ent.target_opt.as_ref().unwrap_or(&node.name);
                    let path = full_path(type_ent.origin_opt.as_deref().unwrap_or(&[]), name);
                    if matches!(ctx.ctx.sigma.types.get(&path_key_of(&path)), Some(TypeDecl::Record(_))) {
                        return Ok(callee.clone());
                    }
                }
            }
            resolve_expr(ctx, callee)
        }
        ExprNode::PathExpr(node) => {
            if is_builtin_modal_type_path(&node.path) && is_builtin_modal_member_name(&node.path, &node.name) {
                return Ok(callee.clone());
            }
            let value = resolve_qualified(
                ctx.ctx,
                ctx.name_maps,
                ctx.module_names,
                &node.path,
                &node.name,
                EntityKind::Value,
                ctx.can_access,
            );
            if value.ok {
                if let Some(entity) = &value.entity {
                    ctx.record_reference(&node.name, &callee_expr.span, entity);
                }
                return Ok(callee.clone());
            }
            if args.is_empty() && resolve_record_path(ctx, &node.path, &node.name).is_some() {
                return Ok(callee.clone());
            }
            resolve_expr(ctx, callee)
        }
        _ => resolve_expr(ctx, callee),
    }
}

fn resolve_if_case(ctx: &mut ResolveContext<'_, '_>, arm: &IfCaseClause) -> Res<IfCaseClause> {
    let span_of = |first: &Option<Span>, second: &Option<Span>| first.clone().or_else(|| second.clone()).unwrap_or_default();
    let pattern_span = arm.pattern.as_deref().map(|pattern| pattern.span.clone());
    let body_span = arm.body.as_deref().map(|body| body.span.clone());
    // A failure that carries neither a rule nor a detail gets one saying where it happened.
    let located = |mut err: ResError, what: &str, fallback: Span| {
        if err.diag_id.is_none() && err.detail.is_empty() {
            err.detail = format!("while resolving `if ... is` clause {what}");
        }
        err.span_or(&fallback)
    };
    with_scope(ctx, Scope::new(), |ctx| {
        let pattern = resolve_pattern(ctx, &arm.pattern)
            .map_err(|err| located(err, "pattern", span_of(&pattern_span, &body_span)))?;
        if let Some(bare_type_span) = bare_type_pattern_span(ctx.ctx, &pattern) {
            return Err(ResError::new("IfIs-BareTypePattern-Err", Some(bare_type_span)));
        }
        bind_pattern(ctx, &pattern).map_err(|err| err.span_or(&span_of(&pattern_span, &body_span)))?;
        let body =
            resolve_expr(ctx, &arm.body).map_err(|err| located(err, "body", span_of(&body_span, &pattern_span)))?;
        Ok(IfCaseClause { pattern, body })
    })
}

pub fn resolve_expr(ctx: &mut ResolveContext<'_, '_>, expr_ptr: &ExprPtr) -> Res<ExprPtr> {
    let Some(expr_arc) = expr_ptr else {
        return Err(ResError::default());
    };
    let expr: &Expr = expr_arc;
    match &expr.node {
        ExprNode::IdentifierExpr(node) => match resolve_value_name(ctx.ctx, &node.name) {
            Some(ent) => {
                ctx.record_reference(&node.name, &expr.span, &ent);
                Ok(expr_ptr.clone())
            }
            None => Err(unresolved_value_name(ctx.ctx, &node.name, &expr.span)),
        },
        ExprNode::QualifiedNameExpr(QualifiedNameExpr { path, name })
        | ExprNode::QualifiedApplyExpr(QualifiedApplyExpr { path, name, .. }) => {
            resolve_qualified_form(ctx, expr).map_err(|diag_id| {
                let diag_id = diag_id.unwrap_or("ResolveExpr-Ident-Err");
                ResError::with_detail(diag_id, &expr.span, qualified_name_detail(path, name))
            })
        }
        ExprNode::AttributedExpr(node) => {
            let Some(inner) = node.expr.as_deref() else {
                let detail = "unresolved attributed expression";
                return Err(ResError::with_detail("ResolveExpr-Ident-Err", &expr.span, detail));
            };
            let resolved = match &inner.node {
                ExprNode::ComptimeExpr(comptime) => {
                    resolve_comptime_expr_with_attrs(ctx, &node.expr, inner, comptime, &node.attrs)?
                }
                _ => resolve_expr(ctx, &node.expr)?,
            };
            rebuilt(expr, ExprNode::AttributedExpr(AttributedExpr { attrs: node.attrs.clone(), expr: resolved }))
        }
        ExprNode::EnumLiteralExpr(node) => {
            let payload_opt = resolve_enum_payload(ctx, &node.payload_opt)?;
            let mut path = node.path.clone();
            if let Some((variant_name, enum_path)) = node.path.split_last().filter(|(_, prefix)| !prefix.is_empty()) {
                let kind = match &node.payload_opt {
                    None => VariantKind::Unit,
                    Some(EnumPayload::EnumPayloadParen(_)) => VariantKind::Tuple,
                    Some(EnumPayload::EnumPayloadBrace(_)) => VariantKind::Record,
                };
                if let Some(resolved_enum) = resolve_enum_variant(ctx, enum_path, variant_name, kind) {
                    path = full_path(&resolved_enum, variant_name);
                }
            }
            rebuilt(expr, ExprNode::EnumLiteralExpr(EnumLiteralExpr { path, payload_opt }))
        }
        ExprNode::CallTypeArgsExpr(node) => {
            let args = resolve_args(ctx, &node.args)?;
            let callee = resolve_callee(ctx, &node.callee, &args)?;
            let generic_args = resolve_type_args(ctx, &node.type_args)?;
            rebuilt(expr, ExprNode::CallExpr(CallExpr { callee, generic_args, args }))
        }
        ExprNode::CallExpr(node) => {
            let args = resolve_args(ctx, &node.args)?;
            let callee = resolve_callee(ctx, &node.callee, &args)?;
            let generic_args = resolve_type_args(ctx, &node.generic_args)?;
            rebuilt(expr, ExprNode::CallExpr(CallExpr { callee, generic_args, args }))
        }
        ExprNode::RecordExpr(node) => {
            let target = resolve_record_target(ctx, &node.target).map_err(|err| err.span_or(&expr.span))?;
            let fields = resolve_field_inits(ctx, &node.fields)?;
            rebuilt(expr, ExprNode::RecordExpr(RecordExpr { target, fields }))
        }
        ExprNode::IfIsExpr(node) => {
            let scrutinee = resolve_expr(ctx, &node.scrutinee)?;
            let pattern = resolve_pattern(ctx, &node.pattern).map_err(ResError::id_span)?;
            let then_expr = with_scope(ctx, Scope::new(), |ctx| {
                if let Some(bare_type_span) = bare_type_pattern_span(ctx.ctx, &pattern) {
                    return Err(ResError::new("IfIs-BareTypePattern-Err", Some(bare_type_span)));
                }
                bind_pattern(ctx, &pattern)?;
                resolve_expr(ctx, &node.then_expr)
            })?;
            let else_expr = resolve_expr_opt(ctx, &node.else_expr)?;
            rebuilt(expr, ExprNode::IfIsExpr(IfIsExpr { scrutinee, pattern, then_expr, else_expr }))
        }
        ExprNode::IfCaseExpr(node) => {
            let scrutinee = resolve_expr(ctx, &node.scrutinee)?;
            let cases = node.cases.iter().map(|arm| resolve_if_case(ctx, arm)).collect::<Res<Vec<_>>>()?;
            let else_expr = resolve_expr_opt(ctx, &node.else_expr)?;
            rebuilt(expr, ExprNode::IfCaseExpr(IfCaseExpr { scrutinee, cases, else_expr }))
        }
        ExprNode::LoopIterExpr(node) => {
            let pattern = resolve_pattern(ctx, &node.pattern).map_err(ResError::id_span)?;
            let type_opt = resolve_type_opt(ctx, &node.type_opt).map_err(ResError::id_span)?;
            let iter = resolve_expr(ctx, &node.iter)?;
            with_scope(ctx, Scope::new(), |ctx| {
                bind_pattern(ctx, &pattern)?;
                let invariant_opt = resolve_loop_invariant_opt(ctx, &node.invariant_opt)?;
                let body = resolve_block_opt(ctx, &node.body)?;
                let out = LoopIterExpr { pattern: pattern.clone(), type_opt, iter, invariant_opt, body };
                rebuilt(expr, ExprNode::LoopIterExpr(out))
            })
        }
        ExprNode::BlockExpr(node) => match &node.block {
            None => Ok(expr_ptr.clone()),
            Some(_) => rebuilt(expr, ExprNode::BlockExpr(BlockExpr { block: resolve_block_opt(ctx, &node.block)? })),
        },
        ExprNode::UnsafeBlockExpr(node) => match &node.block {
            None => Ok(expr_ptr.clone()),
            Some(_) => {
                let block = resolve_block_opt(ctx, &node.block)?;
                rebuilt(expr, ExprNode::UnsafeBlockExpr(UnsafeBlockExpr { block }))
            }
        },
        ExprNode::TypeLiteralExpr(node) => {
            let r#type = resolve_type(ctx, &node.r#type)?;
            rebuilt(expr, ExprNode::TypeLiteralExpr(TypeLiteralExpr { r#type }))
        }
        ExprNode::ComptimeExpr(node) => resolve_comptime_expr_with_attrs(ctx, expr_ptr, expr, node, &[]),
        ExprNode::CtIfExpr(node) => {
            let cond = resolve_expr(ctx, &node.cond)?;
            let then_block = resolve_block_opt(ctx, &node.then_block)?;
            let else_block_opt = resolve_block_opt(ctx, &node.else_block_opt)?;
            rebuilt(expr, ExprNode::CtIfExpr(CtIfExpr { cond, then_block, else_block_opt }))
        }
        ExprNode::CtLoopIterExpr(node) => {
            let pattern = resolve_pattern(ctx, &node.pattern).map_err(ResError::id_span)?;
            let type_opt = resolve_type_opt(ctx, &node.type_opt).map_err(ResError::id_span)?;
            let iter = resolve_expr(ctx, &node.iter)?;
            with_scope(ctx, Scope::new(), |ctx| {
                bind_pattern(ctx, &pattern)?;
                let body = resolve_block_opt(ctx, &node.body)?;
                rebuilt(expr, ExprNode::CtLoopIterExpr(CtLoopIterExpr { pattern: pattern.clone(), type_opt, iter, body }))
            })
        }
        ExprNode::AllocExpr(node) => {
            if let Some(region) = &node.region_opt {
                if resolve_value_name(ctx.ctx, region).is_none() {
                    let detail = format!("unresolved name '{region}'");
                    return Err(ResError::with_detail("ResolveExpr-Ident-Err", &expr.span, detail));
                }
            }
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::AllocExpr(AllocExpr { region_opt: node.region_opt.clone(), value }))
        }
        ExprNode::BinaryExpr(node) => resolve_binary_chain(ctx, expr_arc, &node.op),
        ExprNode::UnaryExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::UnaryExpr(UnaryExpr { op: node.op.clone(), value }))
        }
        ExprNode::CastExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            let r#type = resolve_type(ctx, &node.r#type).map_err(ResError::id_span)?;
            rebuilt(expr, ExprNode::CastExpr(CastExpr { value, r#type }))
        }
        ExprNode::RangeExpr(node) => {
            let lhs = resolve_expr_opt(ctx, &node.lhs)?;
            let rhs = resolve_expr_opt(ctx, &node.rhs)?;
            rebuilt(expr, ExprNode::RangeExpr(RangeExpr { kind: node.kind, lhs, rhs }))
        }
        ExprNode::DerefExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::DerefExpr(DerefExpr { value }))
        }
        ExprNode::AddressOfExpr(node) => {
            let place = resolve_expr(ctx, &node.place)?;
            rebuilt(expr, ExprNode::AddressOfExpr(AddressOfExpr { place }))
        }
        ExprNode::MoveExpr(node) => {
            let place = resolve_expr(ctx, &node.place)?;
            rebuilt(expr, ExprNode::MoveExpr(MoveExpr { place }))
        }
        ExprNode::CopyExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::CopyExpr(CopyExpr { value }))
        }
        ExprNode::TupleExpr(node) => {
            let elements = resolve_exprs(ctx, &node.elements)?;
            rebuilt(expr, ExprNode::TupleExpr(TupleExpr { elements }))
        }
        ExprNode::ArrayExpr(node) => {
            let mut elements = Vec::with_capacity(node.elements.len());
            for segment in &node.elements {
                elements.push(match segment {
                    ArraySegment::ArrayElemSegment(seg) => {
                        ArraySegment::ArrayElemSegment(ArrayElemSegment { value: resolve_expr(ctx, &seg.value)? })
                    }
                    ArraySegment::ArrayRepeatSegment(seg) => {
                        let value = resolve_expr(ctx, &seg.value)?;
                        let count = resolve_expr(ctx, &seg.count)?;
                        ArraySegment::ArrayRepeatSegment(ArrayRepeatSegment { value, count })
                    }
                });
            }
            rebuilt(expr, ExprNode::ArrayExpr(ArrayExpr { elements }))
        }
        ExprNode::ArrayRepeatExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            let count = resolve_expr(ctx, &node.count)?;
            rebuilt(expr, ExprNode::ArrayRepeatExpr(ArrayRepeatExpr { value, count }))
        }
        ExprNode::SizeofExpr(node) => match &node.r#type {
            None => Ok(expr_ptr.clone()),
            Some(_) => {
                let r#type = resolve_type(ctx, &node.r#type).map_err(ResError::id_span)?;
                rebuilt(expr, ExprNode::SizeofExpr(SizeofExpr { r#type }))
            }
        },
        ExprNode::AlignofExpr(node) => match &node.r#type {
            None => Ok(expr_ptr.clone()),
            Some(_) => {
                let r#type = resolve_type(ctx, &node.r#type).map_err(ResError::id_span)?;
                rebuilt(expr, ExprNode::AlignofExpr(AlignofExpr { r#type }))
            }
        },
        ExprNode::IfExpr(node) => {
            let cond = resolve_expr(ctx, &node.cond)?;
            let then_expr = resolve_expr(ctx, &node.then_expr)?;
            let else_expr = resolve_expr_opt(ctx, &node.else_expr)?;
            rebuilt(expr, ExprNode::IfExpr(IfExpr { cond, then_expr, else_expr }))
        }
        ExprNode::LoopInfiniteExpr(node) => {
            let invariant_opt = resolve_loop_invariant_opt(ctx, &node.invariant_opt)?;
            let body = resolve_block_opt(ctx, &node.body)?;
            rebuilt(expr, ExprNode::LoopInfiniteExpr(LoopInfiniteExpr { invariant_opt, body }))
        }
        ExprNode::LoopConditionalExpr(node) => {
            let cond = resolve_expr(ctx, &node.cond)?;
            let invariant_opt = resolve_loop_invariant_opt(ctx, &node.invariant_opt)?;
            let body = resolve_block_opt(ctx, &node.body)?;
            rebuilt(expr, ExprNode::LoopConditionalExpr(LoopConditionalExpr { cond, invariant_opt, body }))
        }
        ExprNode::TransmuteExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            let from = resolve_type(ctx, &node.from).map_err(ResError::id_span)?;
            let to = resolve_type(ctx, &node.to).map_err(ResError::id_span)?;
            rebuilt(expr, ExprNode::TransmuteExpr(TransmuteExpr { from, to, value }))
        }
        ExprNode::FieldAccessExpr(node) => {
            let base = resolve_expr(ctx, &node.base)?;
            rebuilt(expr, ExprNode::FieldAccessExpr(FieldAccessExpr { base, name: node.name.clone() }))
        }
        ExprNode::TupleAccessExpr(node) => {
            let base = resolve_expr(ctx, &node.base)?;
            rebuilt(expr, ExprNode::TupleAccessExpr(TupleAccessExpr { base, index: node.index }))
        }
        ExprNode::IndexAccessExpr(node) => {
            let base = resolve_expr(ctx, &node.base)?;
            let index = resolve_expr(ctx, &node.index)?;
            rebuilt(expr, ExprNode::IndexAccessExpr(IndexAccessExpr { base, index }))
        }
        ExprNode::MethodCallExpr(node) => {
            if let Some(receiver) = node.receiver.as_deref() {
                // `emitter~>emit(..)` in compile-time code that was not granted the emitter.
                let is_emitter = matches!(&receiver.node, ExprNode::IdentifierExpr(ident) if id_eq(&ident.name, "emitter"));
                if is_emitter
                    && id_eq(&node.name, "emit")
                    && is_comptime_resolution_env(ctx.ctx)
                    && resolve_value_name(ctx.ctx, "emitter").is_none()
                {
                    let detail = "emit call requires the `TypeEmitter` capability";
                    return Err(ResError::with_detail("E-CTE-0250", &receiver.span, detail));
                }
            }
            let receiver = resolve_expr(ctx, &node.receiver)?;
            let args = resolve_args(ctx, &node.args)?;
            rebuilt(expr, ExprNode::MethodCallExpr(MethodCallExpr { receiver, name: node.name.clone(), args }))
        }
        ExprNode::PropagateExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::PropagateExpr(PropagateExpr { value }))
        }
        ExprNode::EntryExpr(node) => {
            let inner = resolve_expr(ctx, &node.expr)?;
            rebuilt(expr, ExprNode::EntryExpr(EntryExpr { expr: inner }))
        }
        ExprNode::YieldExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::YieldExpr(YieldExpr { release: node.release, value }))
        }
        ExprNode::YieldFromExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::YieldFromExpr(YieldFromExpr { release: node.release, value }))
        }
        ExprNode::SyncExpr(node) => {
            let value = resolve_expr(ctx, &node.value)?;
            rebuilt(expr, ExprNode::SyncExpr(SyncExpr { value }))
        }
        ExprNode::RaceExpr(node) => {
            let mut arms = Vec::with_capacity(node.arms.len());
            for arm in &node.arms {
                let arm_expr = resolve_expr(ctx, &arm.expr)?;
                arms.push(with_scope(ctx, Scope::new(), |ctx| {
                    let pattern = resolve_pattern(ctx, &arm.pattern).map_err(ResError::id_span)?;
                    bind_pattern(ctx, &pattern)?;
                    let handler = RaceHandler { kind: arm.handler.kind, value: resolve_expr(ctx, &arm.handler.value)? };
                    Ok(RaceArm { expr: arm_expr, pattern, handler })
                })?);
            }
            rebuilt(expr, ExprNode::RaceExpr(RaceExpr { arms }))
        }
        ExprNode::AllExpr(node) => {
            let exprs = resolve_exprs(ctx, &node.exprs)?;
            rebuilt(expr, ExprNode::AllExpr(AllExpr { exprs }))
        }
        ExprNode::ParallelExpr(node) => {
            let domain = resolve_expr(ctx, &node.domain)?;
            let opts = node.opts.iter().map(|opt| resolve_parallel_opt(ctx, opt)).collect::<Res<Vec<_>>>()?;
            let body = resolve_block_opt(ctx, &node.body)?;
            rebuilt(expr, ExprNode::ParallelExpr(ParallelExpr { domain, opts, body }))
        }
        ExprNode::SpawnExpr(node) => {
            let opts = node.opts.iter().map(|opt| resolve_spawn_opt(ctx, opt)).collect::<Res<Vec<_>>>()?;
            let body = resolve_block_opt(ctx, &node.body)?;
            rebuilt(expr, ExprNode::SpawnExpr(SpawnExpr { opts, body }))
        }
        ExprNode::WaitExpr(node) => {
            let handle = resolve_expr(ctx, &node.handle)?;
            rebuilt(expr, ExprNode::WaitExpr(WaitExpr { handle }))
        }
        ExprNode::DispatchExpr(node) => {
            let pattern = resolve_pattern(ctx, &node.pattern).map_err(ResError::id_span)?;
            let range = resolve_expr(ctx, &node.range)?;
            let key_clause = match &node.key_clause {
                Some(clause) => Some(DispatchKeyClause {
                    key_path: resolve_key_path_expr(ctx, &clause.key_path)?,
                    mode: clause.mode,
                    span: clause.span.clone(),
                }),
                None => None,
            };
            with_scope(ctx, Scope::new(), |ctx| {
                bind_pattern(ctx, &pattern)?;
                let opts = node.opts.iter().map(|opt| resolve_dispatch_opt(ctx, opt)).collect::<Res<Vec<_>>>()?;
                let body = resolve_block_opt(ctx, &node.body)?;
                rebuilt(expr, ExprNode::DispatchExpr(DispatchExpr { pattern: pattern.clone(), range, key_clause, opts, body }))
            })
        }
        ExprNode::ErrorExpr(_)
        | ExprNode::LiteralExpr(_)
        | ExprNode::PathExpr(_)
        | ExprNode::PtrNullExpr(_)
        | ExprNode::QuoteExpr(_)
        | ExprNode::SpliceExprNode(_)
        | ExprNode::SpliceIdentNode(_)
        | ExprNode::ClosureExpr(_)
        | ExprNode::PipelineExpr(_)
        | ExprNode::ResultExpr(_)
        | ExprNode::FenceExpr(_) => Ok(expr_ptr.clone()),
    }
}

fn resolve_binding(ctx: &mut ResolveContext<'_, '_>, binding: &Binding, bind_detail: &str) -> Res<Binding> {
    let init = resolve_expr(ctx, &binding.init)?;
    let type_opt = resolve_type_opt(ctx, &binding.type_opt)?;
    let mut pat = binding.pat.clone();
    if binding.pat.is_some() {
        pat = resolve_pattern(ctx, &binding.pat)?;
        bind_pattern(ctx, &pat).map_err(|err| err.with_new_detail(bind_detail))?;
    }
    Ok(Binding { pat, type_opt, init, ..binding.clone() })
}

/// A statement that binds names does so in the innermost scope, for the statements after it.
pub fn resolve_stmt(ctx: &mut ResolveContext<'_, '_>, stmt: &Stmt) -> Res<Stmt> {
    Ok(match stmt {
        Stmt::LetStmt(node) => Stmt::LetStmt(LetStmt {
            binding: resolve_binding(ctx, &node.binding, "while binding let pattern")?,
            span: node.span.clone(),
        }),
        Stmt::VarStmt(node) => Stmt::VarStmt(VarStmt {
            binding: resolve_binding(ctx, &node.binding, "while binding var pattern")?,
            span: node.span.clone(),
        }),
        Stmt::UsingLocalStmt(node) => {
            let Some(ent) = resolve_value_name(ctx.ctx, &node.source) else {
                return Err(ResError::at("ResolveExpr-Ident-Err", &node.span));
            };
            ctx.record_reference(&node.source, &node.span, &ent);
            let introduced = intro(ctx.ctx, &node.alias, &ent);
            if !introduced.ok {
                return Err(ResError::from_id(introduced.diag_id, Some(node.span.clone())));
            }
            stmt.clone()
        }
        Stmt::DeferStmt(node) => {
            Stmt::DeferStmt(DeferStmt { body: resolve_block_opt(ctx, &node.body)?, span: node.span.clone() })
        }
        Stmt::FrameStmt(node) => {
            if let Some(target) = &node.target_opt {
                let Some(ent) = resolve_value_name(ctx.ctx, target) else {
                    let detail = format!("unresolved name '{target}'");
                    return Err(ResError::with_detail("ResolveExpr-Ident-Err", &node.span, detail));
                };
                ctx.record_reference(target, &node.span, &ent);
            }
            Stmt::FrameStmt(FrameStmt { body: resolve_block_opt(ctx, &node.body)?, ..node.clone() })
        }
        Stmt::RegionStmt(node) => {
            let opts_opt = resolve_expr_opt(ctx, &node.opts_opt)?;
            // The alias is visible in the body only; a failure leaves it in place, as
            // the reference does.
            let mut saved_scope = None;
            if let Some(alias) = &node.alias_opt {
                let Some(innermost) = ctx.ctx.scopes.first() else {
                    let detail = format!("unresolved name '{alias}'");
                    return Err(ResError::with_detail("ResolveExpr-Ident-Err", &node.span, detail));
                };
                saved_scope = Some(innermost.clone());
                let mut alias_entity = ctx.local_language_entity(alias, &node.span, LanguageSymbolKind::Variable, "region alias");
                alias_entity.source = EntitySource::RegionAlias;
                let introduced = intro(ctx.ctx, alias, &alias_entity);
                if !introduced.ok {
                    return Err(ResError::from_id(introduced.diag_id, Some(node.span.clone())));
                }
            }
            let body = resolve_block_opt(ctx, &node.body)?;
            if let Some(saved_scope) = saved_scope {
                ctx.ctx.scopes[0] = saved_scope;
            }
            Stmt::RegionStmt(RegionStmt { opts_opt, body, ..node.clone() })
        }
        Stmt::AssignStmt(node) => {
            let place = resolve_expr(ctx, &node.place)?;
            let value = resolve_expr(ctx, &node.value)?;
            Stmt::AssignStmt(AssignStmt { place, value, span: node.span.clone() })
        }
        Stmt::CompoundAssignStmt(node) => {
            let place = resolve_expr(ctx, &node.place)?;
            let value = resolve_expr(ctx, &node.value)?;
            Stmt::CompoundAssignStmt(CompoundAssignStmt { place, op: node.op.clone(), value, span: node.span.clone() })
        }
        Stmt::ExprStmt(node) => {
            Stmt::ExprStmt(ExprStmt { value: resolve_expr(ctx, &node.value)?, span: node.span.clone() })
        }
        Stmt::ReturnStmt(node) => {
            Stmt::ReturnStmt(ReturnStmt { value_opt: resolve_expr_opt(ctx, &node.value_opt)?, span: node.span.clone() })
        }
        Stmt::BreakStmt(node) => {
            Stmt::BreakStmt(BreakStmt { value_opt: resolve_expr_opt(ctx, &node.value_opt)?, span: node.span.clone() })
        }
        Stmt::UnsafeBlockStmt(node) => Stmt::UnsafeBlockStmt(UnsafeBlockStmt {
            body: resolve_block_opt(ctx, &node.body)?,
            span: node.span.clone(),
        }),
        Stmt::CtStmt(node) => {
            let body = match &node.body {
                Some(_) => with_scope(ctx, build_comptime_capability_scope(&node.attrs), |ctx| {
                    resolve_block_opt(ctx, &node.body)
                })?,
                None => None,
            };
            Stmt::CtStmt(CtStmt { body, ..node.clone() })
        }
        Stmt::KeyBlockStmt(node) => {
            let paths =
                node.paths.iter().map(|path| resolve_key_path_expr(ctx, path)).collect::<Res<Vec<_>>>()?;
            let body = resolve_block_opt(ctx, &node.body)?;
            Stmt::KeyBlockStmt(KeyBlockStmt { paths, body, ..node.clone() })
        }
        Stmt::ContinueStmt(_) | Stmt::ErrorStmt(_) => stmt.clone(),
    })
}

pub fn resolve_stmt_seq(ctx: &mut ResolveContext<'_, '_>, stmts: &[Stmt]) -> Res<Vec<Stmt>> {
    stmts.iter().map(|stmt| resolve_stmt(ctx, stmt)).collect()
}

/// A block opens a scope for its statements and tail.
pub fn resolve_block(ctx: &mut ResolveContext<'_, '_>, block: &Block) -> Res<Block> {
    with_scope(ctx, Scope::new(), |ctx| {
        let stmts = resolve_stmt_seq(ctx, &block.stmts)?;
        let tail_opt = resolve_expr_opt(ctx, &block.tail_opt)?;
        Ok(Block { stmts, tail_opt, span: block.span.clone() })
    })
}
