//! Key blocks: `#path [, path…] [mode] { … }` holds keys on shared data for the body.
//!
//! The ordinary forms are ported. Three variants are not, and leave the body pending:
//! speculative blocks, the `ordered` option, and paths indexed by a value that is not
//! a constant (which need the proof that the body's indices do not conflict).

use std::cell::RefCell;
use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};
use uv_source::attributes::{attrs, validate_attributes, AttributeTarget};

use super::block::{type_block_info, FlowInfo, StmtTypeResult};
use crate::composite::function_types::lookup_module_static;
use crate::context::ScopeContext;
use crate::keys::key_paths::{build_key_path, is_prefix, key_path_less, parse_key_path_spec, KeyPath};
use crate::resolve::scopes::id_eq;
use crate::resolve::scopes_lookup::resolve_value_name;
use crate::typing::const_len::const_len;
use crate::typing::pending::pending;
use crate::typing::stmt_context::{HeldKeyTypingInfo, LoopFlag, StmtTypeContext};
use crate::typing::type_env::{bind_of, gpu_context, mark_shared_derived_bindings_stale, TypeEnv};
use crate::typing::type_expr::{type_expr, type_identifier_expr, type_place};
use crate::typing::type_lookup::{field_type, lookup_record_decl};
use crate::typing::type_predicates::{perm_of_type, strip_perm_and_refine};
use crate::typing::types::*;

fn failed(diag_id: &'static str) -> StmtTypeResult {
    StmtTypeResult { diag_id: Some(diag_id), ..Default::default() }
}

fn key_path_equal(lhs: &KeyPath, rhs: &KeyPath) -> bool {
    !key_path_less(lhs, rhs) && !key_path_less(rhs, lhs)
}

/// The keys of a block in the order they are taken, each once.
fn canonical_held_key_infos(paths: &[ast::KeyPathExpr], mode: ast::KeyMode) -> Vec<HeldKeyTypingInfo> {
    let mut infos: Vec<HeldKeyTypingInfo> = paths.iter().map(|path| HeldKeyTypingInfo { path: parse_key_path_spec(path), mode }).collect();
    infos.sort_by(|lhs, rhs| {
        if key_path_less(&lhs.path, &rhs.path) {
            std::cmp::Ordering::Less
        } else if key_path_less(&rhs.path, &lhs.path) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    infos.dedup_by(|later, earlier| key_path_equal(&earlier.path, &later.path));
    infos
}

fn is_memory_order_attribute_name(name: &str) -> bool {
    [attrs::RELAXED, attrs::ACQUIRE, attrs::RELEASE, attrs::ACQ_REL, attrs::SEQ_CST].contains(&name)
}

fn key_path_has_dynamic_index(ctx: &ScopeContext<'_>, path: &ast::KeyPathExpr) -> bool {
    path.segs.iter().any(|seg| matches!(seg, ast::KeySeg::KeySegIndex(index) if const_len(ctx, &index.expr).is_err()))
}

fn seg_marked(seg: &ast::KeySeg) -> bool {
    match seg {
        ast::KeySeg::KeySegField(field) => field.marked,
        ast::KeySeg::KeySegIndex(index) => index.marked,
    }
}

/// The type a path has after one more segment: a record's field or an element.
/// A segment that does not apply leaves the type as it was.
fn advance_key_path_type(ctx: &ScopeContext<'_>, current: &TypeRef, seg: &ast::KeySeg) -> TypeRef {
    let stripped = strip_perm_and_refine(current);
    let Some(base) = stripped.as_deref() else {
        return current.clone();
    };
    match (seg, &base.node) {
        (ast::KeySeg::KeySegField(field), TypeNode::Path { path, .. }) => lookup_record_decl(ctx, path)
            .and_then(|record| field_type(record, &field.name, ctx, &[]))
            .unwrap_or_else(|| current.clone()),
        (ast::KeySeg::KeySegIndex(_), TypeNode::Array { element, .. } | TypeNode::Slice(element)) => element.clone(),
        _ => current.clone(),
    }
}

/// The type of a key path's root: a local binding, or a module-level static.
fn key_root_type(ctx: &ScopeContext<'_>, env: &TypeEnv, root: &str) -> Option<TypeRef> {
    if let Some(binding) = bind_of(env, root) {
        return Some(binding.r#type.clone());
    }
    if let Some(entity) = resolve_value_name(ctx, root).filter(|entity| entity.origin_opt.is_some()) {
        let resolved_name = entity.target_opt.unwrap_or_else(|| root.to_string());
        let static_lookup = lookup_module_static(ctx, &entity.origin_opt.unwrap_or_default(), &resolved_name);
        if static_lookup.ok && static_lookup.r#type.is_some() {
            return Some(static_lookup.r#type);
        }
    }
    let static_lookup = lookup_module_static(ctx, &ctx.current_module, root);
    (static_lookup.ok && static_lookup.r#type.is_some()).then_some(static_lookup.r#type)
}

fn record_of<'c>(ctx: &'c ScopeContext<'_>, ty: &TypeRef) -> Option<&'c ast::RecordDecl> {
    match strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) => lookup_record_decl(ctx, path),
        _ => None,
    }
}

/// A key path must be rooted in shared data, be marked at most once, and be marked
/// only at a field of a record.
fn validate_key_path_conformance(ctx: &ScopeContext<'_>, path: &ast::KeyPathExpr, env: &TypeEnv) -> Option<&'static str> {
    let Some(root_type) = key_root_type(ctx, env, &path.root) else {
        return Some("E-CON-0031");
    };
    if perm_of_type(&root_type) != Permission::Shared {
        return Some("E-CON-0032");
    }
    if path.segs.iter().filter(|seg| seg_marked(seg)).count() > 1 {
        return Some("E-CON-0003");
    }
    let mut current_type = root_type;
    for seg in &path.segs {
        if let ast::KeySeg::KeySegField(field) = seg {
            if field.marked && record_of(ctx, &current_type).is_none() {
                return Some("E-CON-0033");
            }
        }
        current_type = advance_key_path_type(ctx, &current_type, seg);
    }
    None
}

/// Whether the path is marked at a field the record already declares a key boundary,
/// which makes the mark redundant.
fn path_marker_matches_type_boundary(ctx: &ScopeContext<'_>, path: &ast::KeyPathExpr, env: &TypeEnv) -> bool {
    let Some(mut current_type) = key_root_type(ctx, env, &path.root) else {
        return false;
    };
    for seg in &path.segs {
        if let (ast::KeySeg::KeySegField(field), Some(record)) = (seg, record_of(ctx, &current_type)) {
            let declared = record.members.iter().find_map(|member| match member {
                ast::RecordMember::FieldDecl(field_decl) if id_eq(&field_decl.name, &field.name) => Some(field_decl),
                _ => None,
            });
            if declared.is_some_and(|field_decl| field.marked && field_decl.key_boundary) {
                return true;
            }
        }
        current_type = advance_key_path_type(ctx, &current_type, seg);
    }
    false
}

fn collect_written_paths_from_expr(expr: &ExprPtr, out: &mut Vec<KeyPath>) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    match &e.node {
        ExprNode::AttributedExpr(node) => collect_written_paths_from_expr(&node.expr, out),
        ExprNode::BlockExpr(node) => collect_written_paths_from_block(&node.block, out),
        ExprNode::UnsafeBlockExpr(node) => collect_written_paths_from_block(&node.block, out),
        ExprNode::LoopInfiniteExpr(node) => collect_written_paths_from_block(&node.body, out),
        ExprNode::LoopConditionalExpr(node) => collect_written_paths_from_block(&node.body, out),
        ExprNode::LoopIterExpr(node) => collect_written_paths_from_block(&node.body, out),
        ExprNode::IfExpr(node) => {
            collect_written_paths_from_expr(&node.then_expr, out);
            collect_written_paths_from_expr(&node.else_expr, out);
        }
        ExprNode::IfCaseExpr(node) => {
            for arm in &node.cases {
                collect_written_paths_from_expr(&arm.body, out);
            }
            collect_written_paths_from_expr(&node.else_expr, out);
        }
        ExprNode::IfIsExpr(node) => {
            collect_written_paths_from_expr(&node.then_expr, out);
            collect_written_paths_from_expr(&node.else_expr, out);
        }
        _ => {}
    }
}

/// The places the block assigns to, outside nested key blocks.
fn collect_written_paths_from_block(block: &ast::BlockPtr, out: &mut Vec<KeyPath>) {
    let Some(block) = block.as_deref() else {
        return;
    };
    let assigned = |place: &ExprPtr, value: &ExprPtr, out: &mut Vec<KeyPath>| {
        let built = build_key_path(place);
        if built.success {
            out.push(built.path);
        }
        collect_written_paths_from_expr(value, out);
    };
    for stmt in &block.stmts {
        match stmt {
            Stmt::AssignStmt(node) => assigned(&node.place, &node.value, out),
            Stmt::CompoundAssignStmt(node) => assigned(&node.place, &node.value, out),
            Stmt::ExprStmt(node) => collect_written_paths_from_expr(&node.value, out),
            Stmt::LetStmt(node) => collect_written_paths_from_expr(&node.binding.init, out),
            Stmt::VarStmt(node) => collect_written_paths_from_expr(&node.binding.init, out),
            Stmt::DeferStmt(node) => collect_written_paths_from_block(&node.body, out),
            Stmt::UnsafeBlockStmt(node) => collect_written_paths_from_block(&node.body, out),
            Stmt::CtStmt(node) => collect_written_paths_from_block(&node.body, out),
            Stmt::RegionStmt(node) => {
                collect_written_paths_from_expr(&node.opts_opt, out);
                collect_written_paths_from_block(&node.body, out);
            }
            Stmt::FrameStmt(node) => collect_written_paths_from_block(&node.body, out),
            Stmt::ReturnStmt(node) => collect_written_paths_from_expr(&node.value_opt, out),
            Stmt::BreakStmt(node) => collect_written_paths_from_expr(&node.value_opt, out),
            _ => {}
        }
    }
    collect_written_paths_from_expr(&block.tail_opt, out);
}

fn key_mode_sufficient(held: ast::KeyMode, required: ast::KeyMode) -> bool {
    held == ast::KeyMode::Write || held == required
}

pub fn type_key_block_stmt(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, node: &ast::KeyBlockStmt, env: &TypeEnv) -> StmtTypeResult {
    let Some(body) = node.body.as_deref() else {
        return StmtTypeResult::default();
    };
    if type_ctx.in_speculative {
        return failed("E-CON-0090");
    }
    let warn = |code: &str, span: &Span| {
        if let (Some(diags), Some(diag)) = (&type_ctx.diags, make_diagnostic_by_id(code, Some(span.clone()))) {
            emit(&mut diags.borrow_mut(), diag);
        }
    };

    // Memory orders and `dynamic` belong to the key block; other attributes to the
    // statement.
    let (key_block_attrs, statement_attrs): (Vec<_>, Vec<_>) = node
        .attrs
        .iter()
        .cloned()
        .partition(|attr| is_memory_order_attribute_name(&attr.name.full_name) || attr.name.full_name == attrs::DYNAMIC);
    let memory_order_attr_count = key_block_attrs.iter().filter(|attr| is_memory_order_attribute_name(&attr.name.full_name)).count();
    let has_dynamic_attr = key_block_attrs.iter().any(|attr| attr.name.full_name == attrs::DYNAMIC);
    for (attrs_list, target) in [(&statement_attrs, AttributeTarget::Statement), (&key_block_attrs, AttributeTarget::KeyBlock)] {
        if attrs_list.is_empty() {
            continue;
        }
        let validation = validate_attributes(attrs_list, target);
        if !validation.ok {
            return StmtTypeResult { diag_id: validation.diag_id, diag_detail: validation.message, ..Default::default() };
        }
    }
    if memory_order_attr_count > 1 {
        return failed("E-MOD-2450");
    }
    if gpu_context(env) {
        return failed("E-CON-0155");
    }
    for path in &node.paths {
        if let Some(diag_id) = validate_key_path_conformance(ctx, path, env) {
            return failed(diag_id);
        }
        if type_ctx.diags.is_some() && path_marker_matches_type_boundary(ctx, path, env) {
            warn("W-CON-0003", &path.span);
        }
    }
    let has_speculative_mod = node.kind == ast::KeyBlockKind::SpeculativeWrite;
    let has_release_mod = node.kind == ast::KeyBlockKind::Release;
    let dynamic_key_context = type_ctx.contract_dynamic || has_dynamic_attr;
    let current_key_infos = canonical_held_key_infos(&node.paths, node.mode);
    if memory_order_attr_count > 0 && has_speculative_mod {
        return failed("E-CON-0096");
    }
    if has_speculative_mod {
        pending("SpeculativeKeyBlock");
        return StmtTypeResult::default();
    }
    if node.options.ordered {
        pending("OrderedKeyBlock");
        return StmtTypeResult::default();
    }
    // In a loop, a path of several segments could take a finer key.
    if type_ctx.loop_flag == LoopFlag::Loop && type_ctx.diags.is_some() && !has_release_mod {
        if let Some(path) = node.paths.iter().find(|path| path.segs.len() >= 2 && !path.segs.iter().any(seg_marked)) {
            warn("W-CON-0001", &path.span);
        }
    }

    // A key already held may not be taken again in another mode, except to release it.
    let mut emitted_release_interleaving_warning = false;
    for current_key in &current_key_infos {
        for held_key in type_ctx.held_key_paths.iter().filter(|held_key| key_path_equal(&current_key.path, &held_key.path)) {
            if held_key.mode == current_key.mode {
                warn("W-CON-0002", &node.span);
                if has_release_mod {
                    return failed("E-CON-0018");
                }
                continue;
            }
            if key_mode_sufficient(held_key.mode, current_key.mode) {
                warn("W-CON-0002", &node.span);
            }
            if !has_release_mod {
                return failed("E-CON-0012");
            }
            if !emitted_release_interleaving_warning {
                warn("W-CON-0010", &node.span);
                emitted_release_interleaving_warning = true;
            }
        }
    }
    if has_release_mod && !emitted_release_interleaving_warning {
        let releases_held = current_key_infos
            .iter()
            .any(|current_key| type_ctx.held_key_paths.iter().any(|held_key| key_path_equal(&current_key.path, &held_key.path)));
        if releases_held {
            warn("W-CON-0010", &node.span);
        }
    }

    if node.paths.iter().any(|path| key_path_has_dynamic_index(ctx, path)) {
        pending("DynamicKeyPath");
        return StmtTypeResult::default();
    }
    let explicit_key_paths: Vec<KeyPath> = node.paths.iter().map(parse_key_path_spec).collect();
    let mut written_paths = Vec::new();
    collect_written_paths_from_block(&node.body, &mut written_paths);
    let writes_explicit_key_path = written_paths.iter().any(|path| explicit_key_paths.iter().any(|key| is_prefix(key, path)));
    if writes_explicit_key_path && node.mode != ast::KeyMode::Write {
        return failed("E-CON-0070");
    }

    let key_env = Rc::new(RefCell::new(env.clone()));
    let mut held_key_paths = type_ctx.held_key_paths.clone();
    held_key_paths.extend(current_key_infos);
    let key_ctx = StmtTypeContext {
        keys_held: true,
        contract_dynamic: dynamic_key_context,
        key_mode: Some(node.mode),
        held_key_paths,
        in_speculative: false,
        env_ref: Some(key_env.clone()),
        ..type_ctx.clone()
    };
    let key_type_expr = |inner: &ExprPtr| type_expr(ctx, &key_ctx, inner, &key_env.borrow().clone());
    let key_type_ident = |name: &str| type_identifier_expr(ctx, &key_env.borrow().clone(), name);
    let key_type_place = |inner: &ExprPtr| type_place(ctx, &key_ctx, inner, &key_env.borrow().clone());
    let info = type_block_info(ctx, &key_ctx, body, env, &key_type_expr, &key_type_ident, &key_type_place, Some(&key_env));
    if !info.ok {
        return StmtTypeResult { diag_id: info.diag_id, diag_detail: info.diag_detail, diag_span: info.diag_span, ..Default::default() };
    }
    // Releasing keys makes what was read under them stale.
    let mut out_env = env.clone();
    if has_release_mod {
        mark_shared_derived_bindings_stale(&mut out_env);
    }
    let mut flow = FlowInfo { breaks: info.breaks, break_void: info.break_void, ..Default::default() };
    if matches!(strip_perm_and_refine(&info.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!") {
        flow.results.push(info.r#type);
    }
    StmtTypeResult { ok: true, env: out_env, flow, ..Default::default() }
}
