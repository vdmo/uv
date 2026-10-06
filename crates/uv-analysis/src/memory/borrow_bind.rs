//! The borrow check of a body: which bindings are moved, partially moved or still
//! valid at each point, and which places are inactive while something borrowed from
//! them is alive. See `BindCheckBody`.
//!
//! Below: the state the check works on and the operations on it (the reference's
//! `BindEnv` and `PermEnv`, their joins at control-flow merges, the reading of places),
//! then the walk over statements and expressions.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use super::calls::{has_source_provenance, is_place_expr_for_call};
use super::return_responsibility::{call_result_has_responsibility, method_call_result_has_responsibility};
use crate::composite::classes::{class_method_table, lookup_class_method};
use crate::composite::function_types::value_path_type;
use crate::composite::record_methods::recv_mode_of;
use crate::context::{IdKey, ScopeContext, TypeDecl};
use crate::modal::lookup::lookup_transition_decl;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};
use crate::typing::check_expr::infer_expr;
use crate::typing::const_len::const_len;
use crate::typing::expr::small::is_place_expr;
use crate::typing::expr_store::{selected_call_target, stored_expr_type};
use crate::typing::pattern::type_pattern_against_type;
use crate::typing::signature::subst_self_type;
use crate::typing::stmt::scoped::fresh_region_name;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, collect_pat_names, type_pattern, TypeBinding, TypeEnv};
use crate::typing::type_expr::{type_expr, type_identifier_expr, type_place};
use crate::typing::type_lower::{
    lower_bytes_state, lower_param_mode, lower_permission, lower_ptr_state, lower_raw_ptr_qual, lower_string_state,
};
use crate::typing::type_predicates::{bitcopy_type, perm_of_type, strip_perm};
use crate::typing::types::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BindStateKind {
    #[default]
    Valid,
    Moved,
    PartiallyMoved,
}

/// Whether a binding still holds its value; for a partial move, the fields moved out.
#[derive(Clone, Default)]
pub struct BindState {
    pub kind: BindStateKind,
    pub fields: BTreeSet<IdKey>,
}

impl PartialEq for BindState {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && (self.kind != BindStateKind::PartiallyMoved || self.fields == other.fields)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Movability {
    #[default]
    Mov,
    Immov,
}

/// Whether the binding is responsible for its value or only an alias of it.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Responsibility {
    Resp,
    #[default]
    Alias,
}

#[derive(Clone, PartialEq, Default)]
pub struct BindInfo {
    pub state: BindState,
    pub mov: Movability,
    pub r#mut: ast::Mutability,
    pub resp: Responsibility,
}

pub type BindScope = BTreeMap<IdKey, BindInfo>;
/// Outermost scope first.
pub type BindEnv = Vec<BindScope>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Active,
    Inactive,
}

/// A place by its root binding and field path. The derived order is the reference's:
/// by root, then path elements in turn, then length.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PermKey {
    pub root: IdKey,
    pub path: Vec<IdKey>,
}

pub type PermScope = BTreeMap<PermKey, ActiveState>;
pub type PermEnv = Vec<PermScope>;

#[derive(Clone, Default)]
pub struct BindStateBundle {
    pub binds: BindEnv,
    pub perms: PermEnv,
    pub env: TypeEnv,
    pub keys_held: bool,
    pub key_mode: Option<ast::KeyMode>,
}

pub fn lookup_b(env: &BindEnv, name: &str) -> Option<BindInfo> {
    let key = id_key_of(name);
    env.iter().rev().find_map(|scope| scope.get(&key).cloned())
}

/// Replaces the innermost binding of the name; false when there is none.
pub fn update_b(env: &mut BindEnv, name: &str, info: &BindInfo) -> bool {
    let key = id_key_of(name);
    match env.iter_mut().rev().find_map(|scope| scope.get_mut(&key)) {
        Some(found) => {
            *found = info.clone();
            true
        }
        None => false,
    }
}

pub fn intro_b(env: &mut BindEnv, name: &str, info: BindInfo) {
    if env.is_empty() {
        env.push(BindScope::new());
    }
    env.last_mut().expect("a scope").insert(id_key_of(name), info);
}

pub fn intro_all_b(env: &mut BindEnv, scope: &BindScope) {
    if env.is_empty() {
        env.push(BindScope::new());
    }
    let merged = env.last_mut().expect("a scope");
    for (name, info) in scope {
        merged.insert(name.clone(), info.clone());
    }
}

/// Replaces each name's innermost binding; false as soon as one is not bound.
pub fn shadow_all_b(env: &mut BindEnv, scope: &BindScope) -> bool {
    scope.iter().all(|(name, info)| update_b(env, name, info))
}

pub fn join_state(lhs: &BindState, rhs: &BindState) -> BindState {
    use BindStateKind::{Moved, PartiallyMoved, Valid};
    match (lhs.kind, rhs.kind) {
        (Moved, _) | (_, Moved) => BindState { kind: Moved, fields: BTreeSet::new() },
        (PartiallyMoved, PartiallyMoved) => {
            BindState { kind: PartiallyMoved, fields: lhs.fields.union(&rhs.fields).cloned().collect() }
        }
        (PartiallyMoved, Valid) => lhs.clone(),
        (Valid, PartiallyMoved) => rhs.clone(),
        (Valid, Valid) => BindState::default(),
    }
}

fn join_bind_info(lhs: &BindInfo, rhs: &BindInfo) -> Option<BindInfo> {
    (lhs.mov == rhs.mov && lhs.r#mut == rhs.r#mut && lhs.resp == rhs.resp)
        .then(|| BindInfo { state: join_state(&lhs.state, &rhs.state), ..lhs.clone() })
}

fn join_scope_b(lhs: &BindScope, rhs: &BindScope) -> Option<BindScope> {
    if lhs.len() != rhs.len() {
        return None;
    }
    lhs.iter()
        .zip(rhs)
        .map(|((lhs_name, lhs_info), (rhs_name, rhs_info))| {
            if lhs_name != rhs_name {
                return None;
            }
            Some((lhs_name.clone(), join_bind_info(lhs_info, rhs_info)?))
        })
        .collect()
}

/// The environment after two branches meet: the same bindings, each in the weaker of
/// its two states. None when the branches do not bind the same names the same way.
pub fn join_b(lhs: &BindEnv, rhs: &BindEnv) -> Option<BindEnv> {
    if lhs.len() != rhs.len() {
        return None;
    }
    lhs.iter().zip(rhs).map(|(lhs_scope, rhs_scope)| join_scope_b(lhs_scope, rhs_scope)).collect()
}

fn perm_at(scope: &PermScope, key: &PermKey) -> ActiveState {
    scope.get(key).copied().unwrap_or(ActiveState::Active)
}

fn join_scope_pi(lhs: &PermScope, rhs: &PermScope) -> PermScope {
    let keys: BTreeSet<&PermKey> = lhs.keys().chain(rhs.keys()).collect();
    keys.into_iter()
        .map(|key| {
            let both_active = perm_at(lhs, key) == ActiveState::Active && perm_at(rhs, key) == ActiveState::Active;
            (key.clone(), if both_active { ActiveState::Active } else { ActiveState::Inactive })
        })
        .collect()
}

/// A place is active after two branches only when it is active after both.
pub fn join_perm(lhs: &PermEnv, rhs: &PermEnv) -> Option<PermEnv> {
    (lhs.len() == rhs.len()).then(|| lhs.iter().zip(rhs).map(|(lhs_scope, rhs_scope)| join_scope_pi(lhs_scope, rhs_scope)).collect())
}

pub fn join_all_b(envs: &[BindEnv]) -> Option<BindEnv> {
    let (first, rest) = envs.split_first()?;
    rest.iter().try_fold(first.clone(), |current, env| join_b(&current, env))
}

pub fn join_all_perm(envs: &[PermEnv]) -> Option<PermEnv> {
    let (first, rest) = envs.split_first()?;
    rest.iter().try_fold(first.clone(), |current, env| join_perm(&current, env))
}

/// A place is inactive when any scope marks it so.
pub fn lookup_pi(env: &PermEnv, key: &PermKey) -> ActiveState {
    let inactive = env.iter().rev().any(|scope| scope.get(key) == Some(&ActiveState::Inactive));
    if inactive {
        ActiveState::Inactive
    } else {
        ActiveState::Active
    }
}

pub fn inactivate_scope(scope: &mut PermScope, keys: &BTreeSet<PermKey>) {
    for key in keys {
        scope.insert(key.clone(), ActiveState::Inactive);
    }
}

/// The places the innermost scope made inactive that were active before.
pub fn roots(after: &PermEnv, before: &PermEnv) -> BTreeSet<PermKey> {
    let Some(top) = after.last() else {
        return BTreeSet::new();
    };
    top.iter()
        .filter(|(key, state)| **state == ActiveState::Inactive && lookup_pi(before, key) == ActiveState::Active)
        .map(|(key, _)| key.clone())
        .collect()
}

pub fn reactivate(env: &mut PermEnv, keys: &BTreeSet<PermKey>) {
    if let Some(top) = env.last_mut() {
        for key in keys {
            top.remove(key);
        }
    }
}

/// The binding a place is rooted in.
pub fn place_root(expr: &ExprPtr) -> Option<IdKey> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(id_key_of(&node.name)),
        ExprNode::FieldAccessExpr(node) => place_root(&node.base),
        ExprNode::TupleAccessExpr(node) => place_root(&node.base),
        ExprNode::IndexAccessExpr(node) => place_root(&node.base),
        ExprNode::DerefExpr(node) => place_root(&node.value),
        _ => None,
    }
}

pub fn is_root_identifier_place(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::AttributedExpr(node)) => is_root_identifier_place(&node.expr),
        Some(ExprNode::IdentifierExpr(_)) => true,
        _ => false,
    }
}

pub fn place_writes_through_deref(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::DerefExpr(_)) => true,
        Some(ExprNode::FieldAccessExpr(node)) => place_writes_through_deref(&node.base),
        Some(ExprNode::TupleAccessExpr(node)) => place_writes_through_deref(&node.base),
        Some(ExprNode::IndexAccessExpr(node)) => place_writes_through_deref(&node.base),
        _ => false,
    }
}

/// The first field on the way from the root to the place.
pub fn field_head(expr: &ExprPtr) -> Option<IdKey> {
    match &expr.as_deref()?.node {
        ExprNode::FieldAccessExpr(node) => field_head(&node.base).or_else(|| Some(id_key_of(&node.name))),
        ExprNode::TupleAccessExpr(node) => field_head(&node.base),
        ExprNode::IndexAccessExpr(node) => field_head(&node.base),
        _ => None,
    }
}

/// The fields on the way from the root to the place; tuple and index steps add none.
pub fn field_path_of(expr: &ExprPtr) -> Vec<IdKey> {
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::FieldAccessExpr(node)) => {
            let mut path = field_path_of(&node.base);
            path.push(id_key_of(&node.name));
            path
        }
        Some(ExprNode::TupleAccessExpr(node)) => field_path_of(&node.base),
        Some(ExprNode::IndexAccessExpr(node)) => field_path_of(&node.base),
        _ => Vec::new(),
    }
}

// ---- the walk ----

type Diag = Option<&'static str>;

/// The receiver of a method whose body is checked.
#[derive(Clone)]
pub struct BindSelfParam {
    pub r#type: TypeRef,
    pub mode: Option<ParamMode>,
    pub recv_perm: Option<Permission>,
}

pub struct BindCheckResult {
    pub ok: bool,
    pub diag_id: Diag,
    pub span: Option<Span>,
}

/// A failed check: the rule, when one names it, and where.
struct BindError {
    diag_id: Diag,
    span: Option<Span>,
}

/// The state after an expression or statement, and whether control goes on from it.
type BindResult = Result<(BindStateBundle, bool), BindError>;

fn error<T>(diag_id: Diag, span: Option<Span>) -> Result<T, BindError> {
    Err(BindError { diag_id, span })
}

fn span_of(expr: &ExprPtr) -> Option<Span> {
    expr.as_deref().map(|expr| expr.span.clone())
}

/// A type as written, lowered without resolving names: enough to read permissions.
fn local_lower_type(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Result<TypeRef, Diag> {
    use ast::TypeNode as T;
    let Some(t) = ty.as_deref() else {
        return Err(None);
    };
    let lower = |inner: &ast::TypePtr| local_lower_type(ctx, inner);
    Ok(match &t.node {
        T::TypePrim(node) => make_type_prim(&node.name),
        T::TypePermType(node) => make_type_perm(lower_permission(node.perm), lower(&node.base)?),
        T::TypeUnion(node) => make_type_union(node.types.iter().map(lower).collect::<Result<_, _>>()?),
        T::TypeFunc(node) => {
            let params = node
                .params
                .iter()
                .map(|param| Ok(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: lower(&param.r#type)? }))
                .collect::<Result<_, Diag>>()?;
            make_type_func(params, lower(&node.ret)?)
        }
        T::TypeClosure(node) => {
            let params = node
                .params
                .iter()
                .map(|param| Ok((param.mode == Some(ast::ParamMode::Move), lower(&param.r#type)?)))
                .collect::<Result<_, Diag>>()?;
            let ret = lower(&node.ret)?;
            let deps_opt = match &node.deps_opt {
                Some(deps) => Some(
                    deps.iter()
                        .map(|dep| Ok(SharedDep { name: dep.name.clone(), r#type: lower(&dep.r#type)? }))
                        .collect::<Result<_, Diag>>()?,
                ),
                None => None,
            };
            make_type_closure(params, ret, deps_opt)
        }
        T::TypeTuple(node) => make_type_tuple(node.elements.iter().map(lower).collect::<Result<_, _>>()?),
        T::TypeArray(node) => {
            let element = lower(&node.element)?;
            make_type_array(element, const_len(ctx, &node.length)?, None)
        }
        T::TypeSlice(node) => make_type_slice(lower(&node.element)?),
        T::TypeSafePtr(node) => make_type_ptr(lower(&node.element)?, lower_ptr_state(node.state)),
        T::TypeRawPtr(node) => make_type_raw_ptr(lower_raw_ptr_qual(node.qual), lower(&node.element)?),
        T::TypeString(node) => make_type_string(lower_string_state(node.state)),
        T::TypeBytes(node) => make_type_bytes(lower_bytes_state(node.state)),
        T::TypeDynamic(node) => make_type_dynamic(node.path.clone()),
        T::TypeOpaque(node) => make_type(TypeNode::Opaque { class_path: node.path.clone(), origin: ty.clone(), origin_span: t.span.clone() }),
        T::TypeRefine(node) => make_type_refine(lower(&node.base)?, node.predicate.clone()),
        T::TypeModalState(node) => {
            make_type_modal_state(node.path.clone(), &node.state, node.generic_args.iter().map(lower).collect::<Result<_, _>>()?)
        }
        T::TypePathType(node) if !node.generic_args.is_empty() => {
            make_type_path_with(node.path.clone(), node.generic_args.iter().map(lower).collect::<Result<_, _>>()?)
        }
        T::TypePathType(node) => make_type_path(node.path.clone()),
        _ => return Err(None),
    })
}

/// The typing context the check asks its type questions under.
fn make_type_ctx() -> StmtTypeContext<'static> {
    StmtTypeContext { return_type: make_type_prim("()"), ..Default::default() }
}

fn expr_type_of(ctx: &ScopeContext<'_>, env: &TypeEnv, expr: &ExprPtr) -> Option<TypeRef> {
    expr.as_ref()?;
    if let Some(cached) = stored_expr_type(ctx, expr) {
        return Some(cached);
    }
    let type_ctx = make_type_ctx();
    if is_place_expr(expr) {
        let place = type_place(ctx, &type_ctx, expr, env);
        if place.ok {
            return Some(place.r#type);
        }
    }
    let typed = type_expr(ctx, &type_ctx, expr, env);
    typed.ok.then_some(typed.r#type)
}

fn place_type_of(ctx: &ScopeContext<'_>, env: &TypeEnv, expr: &ExprPtr) -> Option<TypeRef> {
    expr.as_ref()?;
    if let Some(cached) = stored_expr_type(ctx, expr) {
        return Some(cached);
    }
    let place = type_place(ctx, &make_type_ctx(), expr, env);
    place.ok.then_some(place.r#type)
}

/// The place and each place above it on the way from the root.
fn anc_paths(expr: &ExprPtr) -> BTreeSet<PermKey> {
    let Some(root) = place_root(expr) else {
        return BTreeSet::new();
    };
    let path = field_path_of(expr);
    (0..=path.len()).map(|len| PermKey { root: root.clone(), path: path[..len].to_vec() }).collect()
}

fn access_path_ok(env: &PermEnv, expr: &ExprPtr) -> bool {
    anc_paths(expr).iter().all(|key| lookup_pi(env, key) == ActiveState::Active)
}

fn access_ok(ctx: &ScopeContext<'_>, state: &BindStateBundle, expr: &ExprPtr) -> bool {
    let binding_ok = place_root(expr).and_then(|root| lookup_b(&state.binds, &root)).is_some_and(|info| match info.state.kind {
        BindStateKind::Valid => true,
        BindStateKind::Moved => false,
        BindStateKind::PartiallyMoved => field_head(expr).is_some_and(|head| !info.state.fields.contains(&head)),
    });
    if !binding_ok {
        return false;
    }
    match place_type_of(ctx, &state.env, expr) {
        Some(place_type) if perm_of_type(&place_type) == Permission::Unique => access_path_ok(&state.perms, expr),
        _ => true,
    }
}

fn is_move_expr(expr: &ExprPtr) -> bool {
    matches!(expr.as_deref().map(|expr| &expr.node), Some(ExprNode::MoveExpr(_)))
}

/// `:=` binds immovably.
fn mov_of(op: &uv_source::lexer::Token) -> Movability {
    if op.lexeme == ":=" {
        Movability::Immov
    } else {
        Movability::Mov
    }
}

/// A binding initialised from a place without `move` only aliases it; a call result
/// is the caller's unless the callee hands back something it was lent.
fn resp_of_init(ctx: &ScopeContext<'_>, init: &ExprPtr) -> Responsibility {
    let of = |has_resp: bool| if has_resp { Responsibility::Resp } else { Responsibility::Alias };
    if !is_place_expr(init) {
        return match init.as_deref().map(|init| &init.node) {
            Some(ExprNode::CallExpr(call)) => call_result_has_responsibility(ctx, call).map_or(Responsibility::Resp, of),
            Some(ExprNode::MethodCallExpr(method)) => method_call_result_has_responsibility(ctx, method).map_or(Responsibility::Resp, of),
            _ => Responsibility::Resp,
        };
    }
    of(is_move_expr(init))
}

fn bind_info_map<'n>(names: impl IntoIterator<Item = &'n String>, resp: Responsibility, mv: Movability, r#mut: ast::Mutability) -> BindScope {
    let mov = if resp == Responsibility::Alias { Movability::Immov } else { mv };
    names.into_iter().map(|name| (id_key_of(name), BindInfo { state: BindState::default(), mov, r#mut, resp })).collect()
}

/// A `move` of a whole binding leaves it moved.
fn consume_on_move(env: &mut BindEnv, expr: &ExprPtr) {
    let Some(ExprNode::MoveExpr(node)) = expr.as_deref().map(|expr| &expr.node) else {
        return;
    };
    let Some(root) = place_root(&node.place) else {
        return;
    };
    if let Some(mut info) = lookup_b(env, &root) {
        info.state = BindState { kind: BindStateKind::Moved, fields: BTreeSet::new() };
        update_b(env, &root, &info);
    }
}

/// A returned place that the binding owns and that cannot simply be copied is moved.
fn return_dest_expr_for_bind(ctx: &ScopeContext<'_>, state: &BindStateBundle, expr: &ExprPtr) -> ExprPtr {
    let Some(e) = expr.as_deref() else {
        return expr.clone();
    };
    if matches!(e.node, ExprNode::MoveExpr(_) | ExprNode::CopyExpr(_)) || !is_place_expr(expr) {
        return expr.clone();
    }
    let aliases = place_root(expr).and_then(|root| lookup_b(&state.binds, &root)).is_some_and(|info| info.resp == Responsibility::Alias);
    if aliases || place_type_of(ctx, &state.env, expr).is_some_and(|ty| bitcopy_type(ctx, &ty)) {
        return expr.clone();
    }
    Some(Arc::new(ast::Expr { span: e.span.clone(), node: ExprNode::MoveExpr(ast::MoveExpr { place: expr.clone() }) }))
}

/// Lending a `unique` place makes it, and the places above it, inactive.
fn downgrade_unique(ctx: &ScopeContext<'_>, env: &TypeEnv, perms: &mut PermEnv, mode: Option<ParamMode>, expr: &ExprPtr) {
    if mode.is_some() || !is_place_expr(expr) {
        return;
    }
    if !place_type_of(ctx, env, expr).is_some_and(|ty| perm_of_type(&ty) == Permission::Unique) {
        return;
    }
    if perms.is_empty() {
        perms.push(PermScope::new());
    }
    inactivate_scope(perms.last_mut().expect("a scope"), &anc_paths(expr));
}

fn downgrade_unique_bind(ctx: &ScopeContext<'_>, env: &TypeEnv, perms: &mut PermEnv, init: &ExprPtr, bind_type: &TypeRef) {
    if !is_place_expr(init) {
        return;
    }
    if !place_type_of(ctx, env, init).is_some_and(|ty| perm_of_type(&ty) == Permission::Unique) {
        return;
    }
    if !matches!(perm_of_type(bind_type), Permission::Const | Permission::Shared) || bitcopy_type(ctx, bind_type) {
        return;
    }
    downgrade_unique(ctx, env, perms, None, init);
}

fn bind_type_for_binding(ctx: &ScopeContext<'_>, env: &TypeEnv, binding: &ast::Binding) -> Result<TypeRef, Diag> {
    let ann_type = ast::binding_annotation_type_opt(binding);
    if ann_type.is_some() {
        return local_lower_type(ctx, &ann_type);
    }
    if let Some(cached) = stored_expr_type(ctx, &binding.init) {
        return Ok(cached);
    }
    let type_ctx = make_type_ctx();
    let type_expr_fn = |expr: &ExprPtr| type_expr(ctx, &type_ctx, expr, env);
    let type_ident_fn = |name: &str| type_identifier_expr(ctx, env, name);
    let inferred = infer_expr(ctx, &binding.init, &type_expr_fn, &type_ident_fn);
    if inferred.ok {
        Ok(inferred.r#type)
    } else {
        Err(inferred.diag_id)
    }
}

fn binding_moved_err_cond(info: &BindInfo, expr: &ExprPtr) -> bool {
    match info.state.kind {
        BindStateKind::Moved => true,
        BindStateKind::PartiallyMoved => field_head(expr).is_none_or(|head| info.state.fields.contains(&head)),
        BindStateKind::Valid => false,
    }
}

/// Assigning to a whole `var` binding that was moved from gives it a value again.
fn allow_moved_root_assign_lhs(state: &BindStateBundle, place: &ExprPtr) -> bool {
    if !is_place_expr(place) || field_head(place).is_some() {
        return false;
    }
    place_root(place)
        .and_then(|root| lookup_b(&state.binds, &root))
        .is_some_and(|info| info.r#mut == ast::Mutability::Var && info.state.kind != BindStateKind::Valid)
}

/// The sub-expressions evaluated in order, for the forms without a rule of their own.
fn children_ltr(expr: &ast::Expr) -> Vec<&ExprPtr> {
    match &expr.node {
        ExprNode::QualifiedApplyExpr(node) => match &node.args {
            ast::ApplyArgs::ParenArgs(args) => args.args.iter().map(|arg| &arg.value).collect(),
            ast::ApplyArgs::BraceArgs(args) => args.fields.iter().map(|field| &field.value).collect(),
        },
        ExprNode::RangeExpr(node) => [&node.lhs, &node.rhs].into_iter().filter(|side| side.is_some()).collect(),
        ExprNode::BinaryExpr(node) => vec![&node.lhs, &node.rhs],
        ExprNode::CastExpr(node) => vec![&node.value],
        ExprNode::UnaryExpr(node) => vec![&node.value],
        ExprNode::DerefExpr(node) => vec![&node.value],
        ExprNode::AllocExpr(node) => vec![&node.value],
        ExprNode::TransmuteExpr(node) => vec![&node.value],
        ExprNode::PropagateExpr(node) => vec![&node.value],
        ExprNode::AddressOfExpr(node) => vec![&node.place],
        ExprNode::MoveExpr(node) => vec![&node.place],
        ExprNode::TupleExpr(node) => node.elements.iter().collect(),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node),
        ExprNode::ArrayRepeatExpr(node) => vec![&node.value, &node.count],
        ExprNode::RecordExpr(node) => node.fields.iter().map(|field| &field.value).collect(),
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            Some(ast::EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().collect(),
            Some(ast::EnumPayload::EnumPayloadBrace(payload)) => payload.fields.iter().map(|field| &field.value).collect(),
            None => Vec::new(),
        },
        ExprNode::FieldAccessExpr(node) => vec![&node.base],
        ExprNode::TupleAccessExpr(node) => vec![&node.base],
        ExprNode::IndexAccessExpr(node) => vec![&node.base, &node.index],
        ExprNode::CallExpr(node) => std::iter::once(&node.callee).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::MethodCallExpr(node) => std::iter::once(&node.receiver).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        _ => Vec::new(),
    }
}

fn push_scope(state: &mut BindStateBundle) {
    state.binds.push(BindScope::new());
    state.perms.push(PermScope::new());
    state.env.scopes.push(Default::default());
}

fn pop_scope(state: &mut BindStateBundle) {
    state.binds.pop();
    state.perms.pop();
    state.env.scopes.pop();
}

fn intro_typed(state: &mut BindStateBundle, bindings: &[(String, TypeRef)], resp: Responsibility, mv: Movability, r#mut: ast::Mutability) {
    intro_all_b(&mut state.binds, &bind_info_map(bindings.iter().map(|(name, _)| name), resp, mv, r#mut));
    if state.env.scopes.is_empty() {
        state.env.scopes.push(Default::default());
    }
    let scope = state.env.scopes.last_mut().expect("a scope");
    for (name, ty) in bindings {
        scope.insert(id_key_of(name), TypeBinding { r#mut, r#type: ty.clone(), ..Default::default() });
    }
}

/// The names a closure takes from outside itself, and those it takes by `move`.
#[derive(Default)]
struct ClosureCaptureCollector {
    captures: BTreeSet<IdKey>,
    move_captures: BTreeSet<IdKey>,
    locals: Vec<HashSet<IdKey>>,
}

impl ClosureCaptureCollector {
    fn is_local(&self, key: &IdKey) -> bool {
        self.locals.iter().any(|scope| scope.contains(key))
    }

    fn add(&mut self, key: IdKey) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(key);
        }
    }

    fn add_pattern(&mut self, pattern: &ast::PatternPtr) {
        let mut names = Vec::new();
        if let Some(pattern) = pattern.as_deref() {
            collect_pat_names(pattern, &mut names);
        }
        for name in names {
            self.add(name);
        }
    }

    fn record_capture(&mut self, key: IdKey, by_move: bool) {
        if self.is_local(&key) {
            return;
        }
        if by_move {
            self.move_captures.insert(key.clone());
        }
        self.captures.insert(key);
    }

    fn visit_all<'x>(&mut self, exprs: impl IntoIterator<Item = &'x ExprPtr>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }

    fn visit_block(&mut self, block: &ast::BlockPtr) {
        let Some(block) = block.as_deref() else {
            return;
        };
        self.locals.push(HashSet::new());
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        self.visit_expr(&block.tail_opt);
        self.locals.pop();
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(node) => {
                self.visit_expr(&node.binding.init);
                self.add_pattern(&node.binding.pat);
            }
            Stmt::VarStmt(node) => {
                self.visit_expr(&node.binding.init);
                self.add_pattern(&node.binding.pat);
            }
            Stmt::UsingLocalStmt(node) => self.add(id_key_of(&node.alias)),
            Stmt::AssignStmt(node) => self.visit_all([&node.place, &node.value]),
            Stmt::CompoundAssignStmt(node) => self.visit_all([&node.place, &node.value]),
            Stmt::ExprStmt(node) => self.visit_expr(&node.value),
            Stmt::ReturnStmt(node) => self.visit_expr(&node.value_opt),
            Stmt::BreakStmt(node) => self.visit_expr(&node.value_opt),
            Stmt::DeferStmt(node) => self.visit_block(&node.body),
            Stmt::RegionStmt(node) => self.visit_block(&node.body),
            Stmt::FrameStmt(node) => self.visit_block(&node.body),
            Stmt::UnsafeBlockStmt(node) => self.visit_block(&node.body),
            Stmt::KeyBlockStmt(node) => self.visit_block(&node.body),
            _ => {}
        }
    }

    fn visit_scoped(&mut self, pattern: &ast::PatternPtr, visit: impl FnOnce(&mut Self)) {
        self.locals.push(HashSet::new());
        self.add_pattern(pattern);
        visit(self);
        self.locals.pop();
    }

    fn visit_expr(&mut self, expr: &ExprPtr) {
        let Some(e) = expr.as_deref() else {
            return;
        };
        match &e.node {
            ExprNode::IdentifierExpr(node) => self.record_capture(id_key_of(&node.name), false),
            ExprNode::AttributedExpr(node) => self.visit_expr(&node.expr),
            ExprNode::MoveExpr(node) => {
                if let Some(root) = place_root(&node.place) {
                    self.record_capture(root, true);
                }
                self.visit_expr(&node.place);
            }
            ExprNode::IfCaseExpr(node) => {
                self.visit_expr(&node.scrutinee);
                for arm in &node.cases {
                    self.visit_scoped(&arm.pattern, |collector| collector.visit_expr(&arm.body));
                }
                self.visit_expr(&node.else_expr);
            }
            ExprNode::IfIsExpr(node) => {
                self.visit_expr(&node.scrutinee);
                self.visit_scoped(&node.pattern, |collector| collector.visit_expr(&node.then_expr));
                self.visit_expr(&node.else_expr);
            }
            ExprNode::IfExpr(node) => self.visit_all([&node.cond, &node.then_expr, &node.else_expr]),
            ExprNode::LoopInfiniteExpr(node) => self.visit_block(&node.body),
            ExprNode::LoopConditionalExpr(node) => {
                self.visit_expr(&node.cond);
                self.visit_block(&node.body);
            }
            ExprNode::LoopIterExpr(node) => {
                self.visit_expr(&node.iter);
                self.visit_scoped(&node.pattern, |collector| collector.visit_block(&node.body));
            }
            ExprNode::DispatchExpr(node) => {
                self.visit_expr(&node.range);
                self.visit_scoped(&node.pattern, |collector| collector.visit_block(&node.body));
            }
            ExprNode::BlockExpr(node) => self.visit_block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.visit_block(&node.block),
            ExprNode::ParallelExpr(node) => {
                self.visit_expr(&node.domain);
                self.visit_block(&node.body);
            }
            ExprNode::SpawnExpr(node) => self.visit_block(&node.body),
            ExprNode::WaitExpr(node) => self.visit_expr(&node.handle),
            ExprNode::YieldExpr(node) => self.visit_expr(&node.value),
            ExprNode::YieldFromExpr(node) => self.visit_expr(&node.value),
            ExprNode::SyncExpr(node) => self.visit_expr(&node.value),
            ExprNode::RaceExpr(node) => self.visit_all(node.arms.iter().map(|arm| &arm.expr)),
            ExprNode::AllExpr(node) => self.visit_all(&node.exprs),
            ExprNode::PipelineExpr(node) => self.visit_all([&node.lhs, &node.rhs]),
            // A nested closure is judged where it is itself bound.
            ExprNode::ClosureExpr(_) => {}
            _ => self.visit_all(children_ltr(e)),
        }
    }
}

struct Checker<'c, 'x> {
    ctx: &'c ScopeContext<'x>,
}

type Params = Vec<Option<ParamMode>>;

impl Checker<'_, '_> {
    fn stmt_seq(&self, stmts: &[Stmt], state: BindStateBundle) -> BindResult {
        let mut current = state;
        for stmt in stmts {
            let (next, falls_through) = self.stmt(stmt, current)?;
            if !falls_through {
                return Ok((next, false));
            }
            current = next;
        }
        Ok((current, true))
    }

    fn block(&self, block: &ast::Block, state: BindStateBundle) -> BindResult {
        let mut scoped = state;
        push_scope(&mut scoped);
        let (mut current, mut falls_through) = self.stmt_seq(&block.stmts, scoped)?;
        if falls_through && block.tail_opt.is_some() {
            (current, falls_through) = self.expr(&block.tail_opt, current)?;
        }
        pop_scope(&mut current);
        Ok((current, falls_through))
    }

    fn block_ptr(&self, block: &ast::BlockPtr, state: BindStateBundle) -> BindResult {
        match block.as_deref() {
            Some(block) => self.block(block, state),
            None => Ok((state, true)),
        }
    }

    /// A block typed with names of its own in scope, which are dropped after it.
    fn scoped_block(&self, block: &ast::BlockPtr, state: BindStateBundle) -> BindResult {
        let (mut out, falls_through) = self.block_ptr(block, state)?;
        pop_scope(&mut out);
        Ok((out, falls_through))
    }

    /// The arguments in order; each lent `unique` place stays inactive for the rest.
    /// Returns the places to reactivate once the call is over.
    fn arg_pass(&self, params: &Params, args: &[ast::Arg], state: BindStateBundle, idx: usize) -> Result<(BindStateBundle, BTreeSet<PermKey>, bool), BindError> {
        if idx >= params.len() || idx >= args.len() {
            return Ok((state, BTreeSet::new(), true));
        }
        let mode = params[idx];
        let arg = &args[idx];
        let has_source = has_source_provenance(&arg.value);
        if mode.is_some() && arg.pass == ast::ArgPassKind::Ref && !is_move_expr(&arg.value) && has_source {
            return error(Some("E-MOD-2411"), Some(arg.span.clone()));
        }
        let eval_expr = match (&arg.value, mode.is_some() && arg.pass == ast::ArgPassKind::Move && !is_move_expr(&arg.value)) {
            (Some(value), true) => {
                let span = if arg.span.file.is_empty() { value.span.clone() } else { arg.span.clone() };
                Some(Arc::new(ast::Expr { span, node: ExprNode::MoveExpr(ast::MoveExpr { place: arg.value.clone() }) }))
            }
            _ => arg.value.clone(),
        };
        let (mut next, falls_through) = self.expr(&eval_expr, state)?;
        if !falls_through {
            return Ok((next, BTreeSet::new(), false));
        }
        if mode.is_none() {
            if has_source && !is_place_expr_for_call(&arg.value) {
                return error(Some("E-TYP-1603"), Some(arg.span.clone()));
            }
            if !has_source {
                return self.arg_pass(params, args, next, idx + 1);
            }
        }
        let perms_before = next.perms.clone();
        downgrade_unique(self.ctx, &next.env.clone(), &mut next.perms, mode, &arg.value);
        let new_roots = roots(&next.perms, &perms_before);
        let (out, mut all_roots, falls_through) = self.arg_pass(params, args, next, idx + 1)?;
        all_roots.extend(new_roots);
        Ok((out, all_roots, falls_through))
    }

    fn move_expr(&self, node: &ast::MoveExpr, state: BindStateBundle) -> BindResult {
        let place = &node.place;
        if !is_place_expr(place) {
            return Ok((state, true));
        }
        let at = span_of(place);
        let place_type = place_type_of(self.ctx, &state.env, place);
        let unique = place_type.as_ref().is_some_and(|ty| perm_of_type(ty) == Permission::Unique);
        if unique && !access_path_ok(&state.perms, place) {
            return error(Some("E-TYP-1602"), at);
        }
        let Some(root) = place_root(place) else {
            return Ok((state, true));
        };
        let Some(mut info) = lookup_b(&state.binds, &root) else {
            return Ok((state, true));
        };
        if info.mov == Movability::Immov {
            return error(Some("E-MEM-3006"), at);
        }
        let mut out = state;
        match field_head(place) {
            None => {
                if info.state.kind != BindStateKind::Valid {
                    return error(Some("E-MEM-3001"), at);
                }
                info.state = BindState { kind: BindStateKind::Moved, fields: BTreeSet::new() };
            }
            Some(head) => {
                if !unique {
                    return error(Some("E-MEM-3004"), at);
                }
                match info.state.kind {
                    BindStateKind::Moved => return error(Some("E-MEM-3001"), at),
                    BindStateKind::PartiallyMoved if info.state.fields.contains(&head) => return error(Some("E-MEM-3001"), at),
                    BindStateKind::PartiallyMoved => {
                        info.state.fields.insert(head);
                    }
                    BindStateKind::Valid => info.state = BindState { kind: BindStateKind::PartiallyMoved, fields: BTreeSet::from([head]) },
                }
            }
        }
        update_b(&mut out.binds, &root, &info);
        Ok((out, true))
    }

    fn place_expr(&self, expr: &ExprPtr, state: BindStateBundle) -> BindResult {
        if access_ok(self.ctx, &state, expr) {
            return Ok((state, true));
        }
        let at = span_of(expr);
        let unique = place_type_of(self.ctx, &state.env, expr).is_some_and(|ty| perm_of_type(&ty) == Permission::Unique);
        if unique && !access_path_ok(&state.perms, expr) {
            return error(Some("E-TYP-1602"), at);
        }
        let moved = place_root(expr).and_then(|root| lookup_b(&state.binds, &root)).is_some_and(|info| binding_moved_err_cond(&info, expr));
        error(moved.then_some("E-MEM-3001"), at)
    }

    fn params_of_proc(proc: &ast::ProcedureDecl) -> Params {
        proc.params.iter().map(|param| lower_param_mode(param.mode)).collect()
    }

    fn find_procedure<'m>(module: &'m ast::ASTModule, name: &str) -> Option<&'m ast::ProcedureDecl> {
        module.items.iter().find_map(|item| match item {
            ast::ASTItem::ProcedureDecl(proc) if id_eq(&proc.name, name) => Some(proc),
            _ => None,
        })
    }

    fn lookup_value_params(&self, path: &[String], name: &str) -> Option<Params> {
        let ctx = self.ctx;
        if let Some(proc) = ctx.sigma.mods.iter().find(|module| module.path == path).and_then(|module| Self::find_procedure(module, name)) {
            return Some(Self::params_of_proc(proc));
        }
        if path.is_empty() {
            // A name that only one procedure of the project has.
            let mut matched: Option<&ast::ProcedureDecl> = None;
            let mut ambiguous = false;
            for proc in ctx.sigma.mods.iter().filter_map(|module| Self::find_procedure(module, name)) {
                if matched.is_some_and(|seen| !std::ptr::eq(seen, proc)) {
                    ambiguous = true;
                    break;
                }
                matched = Some(proc);
            }
            if let (false, Some(proc)) = (ambiguous, matched) {
                return Some(Self::params_of_proc(proc));
            }
        }
        let value_type = value_path_type(ctx, path, name);
        match value_type.r#type.as_deref().filter(|_| value_type.ok).map(|ty| &ty.node) {
            Some(TypeNode::Func { params, .. }) => Some(params.iter().map(|param| param.mode).collect()),
            _ => None,
        }
    }

    fn params_for_call(&self, env: &TypeEnv, call: &ast::CallExpr) -> Option<Params> {
        let ctx = self.ctx;
        if let Some(selected) = selected_call_target(ctx, call) {
            let key = path_key_of(&selected.module_path);
            let proc = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key).and_then(|module| {
                module.items.iter().find_map(|item| match item {
                    ast::ASTItem::ProcedureDecl(proc) if proc.name == selected.proc_name && proc.span == selected.proc_span => Some(proc),
                    _ => None,
                })
            });
            if let Some(proc) = proc {
                return Some(Self::params_of_proc(proc));
            }
        }
        if let Some(callee_type) = expr_type_of(ctx, env, &call.callee).filter(|ty| ty.is_some()) {
            let mut callee_type = callee_type;
            while let Some(TypeNode::Perm { base, .. }) = callee_type.as_deref().map(|ty| &ty.node) {
                callee_type = base.clone();
            }
            if let Some(TypeNode::Func { params, .. }) = callee_type.as_deref().map(|ty| &ty.node) {
                return Some(params.iter().map(|param| param.mode).collect());
            }
        }
        let current = &ctx.current_module;
        match &call.callee.as_deref()?.node {
            ExprNode::IdentifierExpr(node) => self.lookup_value_params(current, &node.name),
            ExprNode::QualifiedNameExpr(node) => self.lookup_value_params(if node.path.is_empty() { current } else { &node.path }, &node.name),
            ExprNode::PathExpr(node) => self.lookup_value_params(if node.path.is_empty() { current } else { &node.path }, &node.name),
            _ => None,
        }
    }

    /// The parameter modes of the method a call names, and its receiver's mode.
    fn params_for_method(&self, env: &TypeEnv, receiver: &ExprPtr, name: &str) -> Option<(Params, Option<ParamMode>)> {
        let ctx = self.ctx;
        let recv_type = expr_type_of(ctx, env, receiver).filter(|ty| ty.is_some())?;
        let base = strip_perm(&recv_type);
        let modes = |params: &[ast::Param]| params.iter().map(|param| lower_param_mode(param.mode)).collect::<Params>();
        let path = match &base.as_deref()?.node {
            TypeNode::Dynamic(path) => {
                let method = lookup_class_method(ctx, path, name)?;
                return Some((modes(&method.params), recv_mode_of(&method.receiver)));
            }
            TypeNode::Path { path, .. } => path,
            _ => return None,
        };
        let implements: &[Vec<String>] = match ctx.sigma.types.get(&path_key_of(path)) {
            Some(TypeDecl::Record(record)) => {
                let own = record.members.iter().find_map(|member| match member {
                    ast::RecordMember::MethodDecl(method) if id_eq(&method.name, name) => Some(method),
                    _ => None,
                });
                if let Some(method) = own {
                    return Some((modes(&method.params), recv_mode_of(&method.receiver)));
                }
                &record.implements
            }
            Some(TypeDecl::Enum(decl)) => &decl.implements,
            Some(TypeDecl::Modal(decl)) => &decl.implements,
            _ => &[],
        };
        let mut default_method: Option<&ast::ClassMethodDecl> = None;
        for class_path in implements {
            let Ok(table) = class_method_table(ctx, class_path) else {
                continue;
            };
            for entry in table {
                let method = entry.method;
                if method.body_opt.is_none() || !id_eq(&method.name, name) {
                    continue;
                }
                if default_method.is_some_and(|seen| !std::ptr::eq(seen, method)) {
                    return None;
                }
                default_method = Some(method);
            }
        }
        let method = default_method?;
        Some((modes(&method.params), recv_mode_of(&method.receiver)))
    }

    /// The transition a call on a modal state names, if it is one.
    fn modal_transition<'c>(&'c self, env: &TypeEnv, receiver: &ExprPtr, name: &str) -> Option<&'c ast::TransitionDecl> {
        let recv_type = expr_type_of(self.ctx, env, receiver).filter(|ty| ty.is_some())?;
        let base = strip_perm(&recv_type);
        let TypeNode::ModalState(modal) = &base.as_deref()?.node else {
            return None;
        };
        let Some(TypeDecl::Modal(modal_decl)) = self.ctx.sigma.types.get(&path_key_of(&modal.path)) else {
            return None;
        };
        lookup_transition_decl(modal_decl, &modal.state, name)
    }

    fn call_expr(&self, call: &ast::CallExpr, state: BindStateBundle) -> BindResult {
        let direct = matches!(
            call.callee.as_deref().map(|callee| &callee.node),
            Some(ExprNode::IdentifierExpr(_) | ExprNode::PathExpr(_) | ExprNode::QualifiedNameExpr(_))
        );
        let mut callee_state = state;
        if !direct {
            let (next, falls_through) = self.expr(&call.callee, callee_state)?;
            if !falls_through {
                return Ok((next, false));
            }
            callee_state = next;
        }
        let Some(params) = self.params_for_call(&callee_state.env, call) else {
            return self.exprs_in_order(call.args.iter().map(|arg| &arg.value), callee_state);
        };
        let (mut out, lent, falls_through) = self.arg_pass(&params, &call.args, callee_state, 0)?;
        if falls_through {
            reactivate(&mut out.perms, &lent);
        }
        Ok((out, falls_through))
    }

    fn exprs_in_order<'x>(&self, exprs: impl IntoIterator<Item = &'x ExprPtr>, state: BindStateBundle) -> BindResult {
        let mut current = state;
        for expr in exprs {
            let (next, falls_through) = self.expr(expr, current)?;
            if !falls_through {
                return Ok((next, false));
            }
            current = next;
        }
        Ok((current, true))
    }

    fn method_call_expr(&self, call: &ast::MethodCallExpr, state: BindStateBundle) -> BindResult {
        let found = self.params_for_method(&state.env, &call.receiver, &call.name);
        let transition = self.modal_transition(&state.env, &call.receiver, &call.name);
        let found = found.or_else(|| {
            transition.map(|transition| (transition.params.iter().map(|param| lower_param_mode(param.mode)).collect(), Some(ParamMode::Move)))
        });
        let Some((params, recv_mode)) = found else {
            return self.exprs_in_order(std::iter::once(&call.receiver).chain(call.args.iter().map(|arg| &arg.value)), state);
        };
        let receiver_has_source = has_source_provenance(&call.receiver);
        if recv_mode.is_some() && transition.is_none() && !is_move_expr(&call.receiver) && receiver_has_source {
            return error(Some("E-MOD-2411"), span_of(&call.receiver));
        }
        if recv_mode.is_none() && receiver_has_source && !is_place_expr_for_call(&call.receiver) {
            return error(Some("E-TYP-1603"), span_of(&call.receiver));
        }
        let (mut recv_state, falls_through) = self.expr(&call.receiver, state)?;
        if !falls_through {
            return Ok((recv_state, false));
        }
        let perms_before = recv_state.perms.clone();
        downgrade_unique(self.ctx, &recv_state.env.clone(), &mut recv_state.perms, recv_mode, &call.receiver);
        let mut lent = roots(&recv_state.perms, &perms_before);
        let (mut out, arg_roots, falls_through) = self.arg_pass(&params, &call.args, recv_state, 0)?;
        if falls_through {
            lent.extend(arg_roots);
            reactivate(&mut out.perms, &lent);
        }
        Ok((out, falls_through))
    }

    /// Where two branches meet: the one that goes on, or the join of both.
    fn merge(
        &self,
        base: BindStateBundle,
        lhs: (BindStateBundle, bool),
        rhs: (BindStateBundle, bool),
        span: Option<Span>,
    ) -> BindResult {
        match (lhs.1, rhs.1) {
            (false, false) => Ok((base, false)),
            (true, false) => Ok((lhs.0, true)),
            (false, true) => Ok((rhs.0, true)),
            (true, true) => {
                let (Some(binds), Some(perms)) = (join_b(&lhs.0.binds, &rhs.0.binds), join_perm(&lhs.0.perms, &rhs.0.perms)) else {
                    return error(None, span);
                };
                Ok((BindStateBundle { binds, perms, ..base }, true))
            }
        }
    }

    fn if_expr(&self, expr: &ast::IfExpr, state: BindStateBundle) -> BindResult {
        let (cond, falls_through) = self.expr(&expr.cond, state)?;
        if !falls_through {
            return Ok((cond, false));
        }
        let then_res = self.expr(&expr.then_expr, cond.clone())?;
        let else_res = self.expr(&expr.else_expr, cond.clone())?;
        let span = span_of(&expr.cond).or_else(|| span_of(&expr.else_expr));
        self.merge(cond, then_res, else_res, span)
    }

    /// The body of a pattern arm: its bindings in a scope of their own.
    fn arm(&self, base: &BindStateBundle, bindings: &[(String, TypeRef)], moved: bool, body: &ExprPtr) -> BindResult {
        let mut arm_state = base.clone();
        push_scope(&mut arm_state);
        let resp = if moved { Responsibility::Resp } else { Responsibility::Alias };
        intro_typed(&mut arm_state, bindings, resp, Movability::Mov, ast::Mutability::Let);
        let (mut out, falls_through) = self.expr(body, arm_state)?;
        pop_scope(&mut out);
        Ok((out, falls_through))
    }

    fn scrutinee(&self, scrutinee: &ExprPtr, state: BindStateBundle) -> Result<Result<(BindStateBundle, TypeRef), BindStateBundle>, BindError> {
        let (mut base, falls_through) = self.expr(scrutinee, state)?;
        if !falls_through {
            return Ok(Err(base));
        }
        let Some(scrut_type) = expr_type_of(self.ctx, &base.env, scrutinee) else {
            return error(None, span_of(scrutinee));
        };
        consume_on_move(&mut base.binds, scrutinee);
        Ok(Ok((base, scrut_type)))
    }

    fn if_case_expr(&self, expr: &ast::IfCaseExpr, state: BindStateBundle) -> BindResult {
        let (base, scrut_type) = match self.scrutinee(&expr.scrutinee, state)? {
            Ok(found) => found,
            Err(diverged) => return Ok((diverged, false)),
        };
        let moved = is_move_expr(&expr.scrutinee);
        let mut bind_envs = Vec::new();
        let mut perm_envs = Vec::new();
        for arm in &expr.cases {
            if arm.pattern.is_none() || arm.body.is_none() {
                return error(None, arm.pattern.as_deref().map(|pattern| pattern.span.clone()));
            }
            let bindings = match type_pattern_against_type(self.ctx, &arm.pattern, &scrut_type) {
                Ok(bindings) => bindings,
                Err(diag_id) => return error(diag_id, arm.pattern.as_deref().map(|pattern| pattern.span.clone())),
            };
            let (arm_state, falls_through) = self.arm(&base, &bindings, moved, &arm.body)?;
            if falls_through {
                bind_envs.push(arm_state.binds);
                perm_envs.push(arm_state.perms);
            }
        }
        if bind_envs.is_empty() {
            return Ok((base, false));
        }
        let (Some(binds), Some(perms)) = (join_all_b(&bind_envs), join_all_perm(&perm_envs)) else {
            return error(None, span_of(&expr.scrutinee));
        };
        Ok((BindStateBundle { binds, perms, ..base }, true))
    }

    fn if_is_expr(&self, expr: &ast::IfIsExpr, state: BindStateBundle) -> BindResult {
        if expr.pattern.is_none() || expr.then_expr.is_none() {
            return error(None, span_of(&expr.scrutinee));
        }
        let (base, scrut_type) = match self.scrutinee(&expr.scrutinee, state)? {
            Ok(found) => found,
            Err(diverged) => return Ok((diverged, false)),
        };
        let bindings = match type_pattern_against_type(self.ctx, &expr.pattern, &scrut_type) {
            Ok(bindings) => bindings,
            Err(diag_id) => return error(diag_id, expr.pattern.as_deref().map(|pattern| pattern.span.clone())),
        };
        let then_res = self.arm(&base, &bindings, is_move_expr(&expr.scrutinee), &expr.then_expr)?;
        let else_res = self.expr(&expr.else_expr, base.clone())?;
        self.merge(base, then_res, else_res, span_of(&expr.scrutinee))
    }

    /// A loop body is checked until the state before it stops changing.
    fn loop_fix(&self, init: BindStateBundle, step: &dyn Fn(BindStateBundle) -> BindResult) -> BindResult {
        let mut current = init.clone();
        for _ in 0..128 {
            let (body, falls_through) = step(current.clone())?;
            if !falls_through {
                return Ok((body, false));
            }
            let (Some(binds), Some(perms)) = (join_b(&init.binds, &body.binds), join_perm(&init.perms, &body.perms)) else {
                return error(None, None);
            };
            let settled = binds == current.binds && perms == current.perms;
            current = BindStateBundle { binds, perms, ..current };
            if settled {
                return Ok((current, true));
            }
        }
        error(None, None)
    }

    fn loop_iter(&self, node: &ast::LoopIterExpr, state: BindStateBundle) -> BindResult {
        let (iter_state, _) = self.expr(&node.iter, state)?;
        let pat_type = if node.type_opt.is_some() {
            match local_lower_type(self.ctx, &node.type_opt) {
                Ok(lowered) => lowered,
                Err(diag_id) => return error(diag_id, node.type_opt.as_deref().map(|ty| ty.span.clone())),
            }
        } else {
            let element = expr_type_of(self.ctx, &iter_state.env, &node.iter).and_then(|iter_type| {
                match strip_perm(&iter_type).as_deref().map(|ty| &ty.node) {
                    Some(TypeNode::Slice(element)) => Some(element.clone()),
                    Some(TypeNode::Array { element, .. }) => Some(element.clone()),
                    _ => None,
                }
            });
            match element {
                Some(element) => element,
                None => return error(None, span_of(&node.iter)),
            }
        };
        let bindings = match type_pattern(self.ctx, &node.pattern, &pat_type) {
            Ok(bindings) => bindings,
            Err(diag_id) => return error(diag_id, node.pattern.as_deref().map(|pattern| pattern.span.clone())),
        };
        self.loop_fix(iter_state, &|state| {
            let mut scoped = state;
            push_scope(&mut scoped);
            intro_typed(&mut scoped, &bindings, Responsibility::Resp, Movability::Mov, ast::Mutability::Let);
            let (out, _) = self.scoped_block(&node.body, scoped)?;
            Ok((out, true))
        })
    }

    fn closure_expr(&self, expr: &ast::Expr, closure: &ast::ClosureExpr, state: BindStateBundle) -> BindResult {
        let mut collector = ClosureCaptureCollector::default();
        collector.locals.push(closure.params.iter().map(|param| id_key_of(&param.name)).collect());
        collector.visit_expr(&closure.body);
        if collector.captures.is_empty() {
            return Ok((state, true));
        }
        let at = Some(expr.span.clone());
        let mut move_caps = collector.move_captures.clone();
        for param in closure.params.iter().filter(|param| param.move_capture) {
            let key = id_key_of(&param.name);
            if collector.captures.contains(&key) {
                move_caps.insert(key);
            }
        }
        for name in &collector.captures {
            let unique = bind_of(&state.env, name).is_some_and(|binding| binding.r#type.is_some() && perm_of_type(&binding.r#type) == Permission::Unique);
            if unique && !move_caps.contains(name) {
                return error(Some("E-CON-0120"), at);
            }
        }
        for name in &move_caps {
            if let Some(info) = lookup_b(&state.binds, name) {
                if info.mov == Movability::Immov {
                    return error(Some("E-MEM-3006"), at);
                }
                if info.state.kind != BindStateKind::Valid {
                    return error(Some("E-CON-0121"), at);
                }
            }
        }
        for name in collector.captures.iter().filter(|name| !move_caps.contains(*name)) {
            if lookup_b(&state.binds, name).is_some_and(|info| info.state.kind != BindStateKind::Valid) {
                return error(Some("E-MEM-3001"), at);
            }
        }
        let mut out = state;
        for name in &move_caps {
            if let Some(mut info) = lookup_b(&out.binds, name).filter(|info| info.mov != Movability::Immov) {
                info.state = BindState { kind: BindStateKind::Moved, fields: BTreeSet::new() };
                update_b(&mut out.binds, name, &info);
            }
        }
        Ok((out, true))
    }

    fn expr(&self, expr: &ExprPtr, state: BindStateBundle) -> BindResult {
        let Some(e) = expr.as_deref() else {
            return Ok((state, true));
        };
        match &e.node {
            ExprNode::MoveExpr(node) => return self.move_expr(node, state),
            ExprNode::AttributedExpr(node) => return self.expr(&node.expr, state),
            ExprNode::ClosureExpr(node) => return self.closure_expr(e, node, state),
            _ => {}
        }
        if is_place_expr(expr) {
            return self.place_expr(expr, state);
        }
        match &e.node {
            ExprNode::CallExpr(node) => self.call_expr(node, state),
            ExprNode::MethodCallExpr(node) => self.method_call_expr(node, state),
            ExprNode::IfExpr(node) => self.if_expr(node, state),
            ExprNode::IfIsExpr(node) => self.if_is_expr(node, state),
            ExprNode::IfCaseExpr(node) => self.if_case_expr(node, state),
            ExprNode::BlockExpr(node) => self.block_ptr(&node.block, state),
            ExprNode::UnsafeBlockExpr(node) => self.block_ptr(&node.block, state),
            ExprNode::LoopInfiniteExpr(node) => self.loop_fix(state, &|state| self.block_ptr(&node.body, state)),
            ExprNode::LoopConditionalExpr(node) => self.loop_fix(state, &|state| {
                let (cond, _) = self.expr(&node.cond, state)?;
                self.block_ptr(&node.body, cond)
            }),
            ExprNode::LoopIterExpr(node) => self.loop_iter(node, state),
            ExprNode::PipelineExpr(node) => self.exprs_in_order([&node.lhs, &node.rhs], state),
            _ => self.exprs_in_order(children_ltr(e), state),
        }
    }

    fn place_perm_is(&self, env: &TypeEnv, place: &ExprPtr, wanted: Permission) -> bool {
        matches!(place_type_of(self.ctx, env, place).flatten().as_deref().map(|ty| &ty.node), Some(TypeNode::Perm { perm, .. }) if *perm == wanted)
    }

    fn binding_stmt(&self, binding: &ast::Binding, r#mut: ast::Mutability, state: BindStateBundle) -> BindResult {
        let bind_type = match bind_type_for_binding(self.ctx, &state.env, binding) {
            Ok(bind_type) => bind_type,
            Err(diag_id) => return error(diag_id, Some(binding.span.clone())),
        };
        if perm_of_type(&bind_type) == Permission::Unique && is_place_expr(&binding.init) && !is_move_expr(&binding.init) {
            return error(Some("E-MEM-3007"), span_of(&binding.init));
        }
        let (mut current, _) = self.expr(&binding.init, state)?;
        downgrade_unique_bind(self.ctx, &current.env.clone(), &mut current.perms, &binding.init, &bind_type);
        consume_on_move(&mut current.binds, &binding.init);
        let bindings = match type_pattern(self.ctx, &binding.pat, &bind_type) {
            Ok(bindings) => bindings,
            Err(diag_id) => return error(diag_id, binding.pat.as_deref().map(|pat| pat.span.clone())),
        };
        intro_typed(&mut current, &bindings, resp_of_init(self.ctx, &binding.init), mov_of(&binding.op), r#mut);
        Ok((current, true))
    }

    fn assign_stmt(&self, place: &ExprPtr, value: &ExprPtr, compound: bool, span: &Span, state: BindStateBundle) -> BindResult {
        if is_place_expr(place) {
            if self.place_perm_is(&state.env, place, Permission::Const) {
                return error(Some("E-TYP-1601"), Some(span.clone()));
            }
            if let (false, Some(root)) = (place_writes_through_deref(place), place_root(place)) {
                let immutable = lookup_b(&state.binds, &root).is_some_and(|info| info.r#mut == ast::Mutability::Let);
                let shared_write =
                    state.keys_held && state.key_mode == Some(ast::KeyMode::Write) && self.place_perm_is(&state.env, place, Permission::Shared);
                if immutable && !shared_write {
                    return error(Some("E-MOD-2401"), Some(span.clone()));
                }
            }
        }
        let mut current = if compound {
            let (lhs, _) = self.expr(place, state)?;
            self.expr(value, lhs)?.0
        } else {
            let (val, _) = self.expr(value, state)?;
            if allow_moved_root_assign_lhs(&val, place) {
                val
            } else {
                self.expr(place, val)?.0
            }
        };
        if is_place_expr(place) {
            if let Some(root) = place_root(place) {
                if let Some(mut info) = lookup_b(&current.binds, &root).filter(|info| info.r#mut == ast::Mutability::Var) {
                    info.state = BindState::default();
                    update_b(&mut current.binds, &root, &info);
                }
            }
        }
        Ok((current, true))
    }

    /// A block entered with a fresh region in scope under the given name.
    fn region_block(&self, name: &str, body: &ast::BlockPtr, mut scoped: BindStateBundle) -> BindResult {
        let region_type = make_type_perm(Permission::Unique, make_type_modal_state(vec!["Region".to_string()], "Active", Vec::new()));
        intro_typed(&mut scoped, &[(name.to_string(), region_type)], Responsibility::Resp, Movability::Mov, ast::Mutability::Let);
        let (out, _) = self.scoped_block(body, scoped)?;
        Ok((out, true))
    }

    fn stmt(&self, stmt: &Stmt, state: BindStateBundle) -> BindResult {
        match stmt {
            Stmt::LetStmt(node) => self.binding_stmt(&node.binding, ast::Mutability::Let, state),
            Stmt::VarStmt(node) => self.binding_stmt(&node.binding, ast::Mutability::Var, state),
            Stmt::AssignStmt(node) => self.assign_stmt(&node.place, &node.value, false, &node.span, state),
            Stmt::CompoundAssignStmt(node) => self.assign_stmt(&node.place, &node.value, true, &node.span, state),
            Stmt::ExprStmt(node) => self.expr(&node.value, state),
            Stmt::ReturnStmt(node) => {
                if node.value_opt.is_none() {
                    return Ok((state, false));
                }
                let dest = return_dest_expr_for_bind(self.ctx, &state, &node.value_opt);
                let (out, _) = self.expr(&dest, state)?;
                Ok((out, false))
            }
            Stmt::BreakStmt(node) if node.value_opt.is_some() => self.expr(&node.value_opt, state),
            Stmt::UnsafeBlockStmt(node) => self.block_ptr(&node.body, state),
            Stmt::DeferStmt(node) => {
                // A deferred block may not change what is moved or lent.
                let (after, _) = self.block_ptr(&node.body, state.clone())?;
                if after.binds != state.binds || after.perms != state.perms {
                    return error(None, Some(node.span.clone()));
                }
                Ok((state, true))
            }
            Stmt::RegionStmt(node) => {
                let mut current = state;
                if node.opts_opt.is_some() {
                    current = self.expr(&node.opts_opt, current)?.0;
                }
                let name = node.alias_opt.clone().unwrap_or_else(|| fresh_region_name(&current.env));
                let mut scoped = current;
                push_scope(&mut scoped);
                self.region_block(&name, &node.body, scoped)
            }
            Stmt::FrameStmt(node) => {
                let mut scoped = state;
                push_scope(&mut scoped);
                let name = fresh_region_name(&scoped.env);
                self.region_block(&name, &node.body, scoped)
            }
            Stmt::KeyBlockStmt(node) => {
                let mut scoped = state;
                push_scope(&mut scoped);
                scoped.keys_held = true;
                scoped.key_mode = Some(node.mode);
                let (out, _) = self.scoped_block(&node.body, scoped)?;
                Ok((out, true))
            }
            _ => Ok((state, true)),
        }
    }
}

/// The module's statics: their types for the environment and how they are bound.
fn static_bind_map(ctx: &ScopeContext<'_>, module_path: &[String], env: &mut TypeEnv) -> BindScope {
    let mut out = BindScope::new();
    let key = path_key_of(module_path);
    let Some(module) = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key) else {
        return out;
    };
    for item in &module.items {
        let ast::ASTItem::StaticDecl(decl) = item else {
            continue;
        };
        let ann_type = ast::binding_annotation_type_opt(&decl.binding);
        let Ok(ann) = local_lower_type(ctx, &ann_type) else {
            continue;
        };
        let Ok(bindings) = type_pattern_against_type(ctx, &decl.binding.pat, &ann) else {
            continue;
        };
        if env.scopes.is_empty() {
            env.scopes.push(Default::default());
        }
        for (name, ty) in &bindings {
            env.scopes[0].insert(id_key_of(name), TypeBinding { r#mut: decl.r#mut, r#type: ty.clone(), ..Default::default() });
        }
        let infos = bind_info_map(bindings.iter().map(|(name, _)| name), resp_of_init(ctx, &decl.binding.init), mov_of(&decl.binding.op), decl.r#mut);
        for (name, info) in infos {
            out.entry(name).or_insert(info);
        }
    }
    out
}

fn param_bind_map(params: &[ast::Param], self_param: Option<&BindSelfParam>) -> BindScope {
    let info = |by_value: bool, r#mut: ast::Mutability| BindInfo {
        state: BindState::default(),
        mov: if by_value { Movability::Mov } else { Movability::Immov },
        r#mut,
        resp: if by_value { Responsibility::Resp } else { Responsibility::Alias },
    };
    let mut out = BindScope::new();
    if let Some(self_param) = self_param {
        let r#mut = if self_param.recv_perm == Some(Permission::Unique) { ast::Mutability::Var } else { ast::Mutability::Let };
        out.insert(id_key_of("self"), info(self_param.mode.is_some(), r#mut));
    }
    for param in params {
        out.entry(id_key_of(&param.name)).or_insert_with(|| info(param.mode.is_some(), ast::Mutability::Let));
    }
    out
}

/// The types of `self` and the parameters, in the innermost scope.
pub fn param_type_map(ctx: &ScopeContext<'_>, params: &[ast::Param], self_param: Option<&BindSelfParam>, env: &mut TypeEnv) {
    if env.scopes.is_empty() {
        env.scopes.push(Default::default());
    }
    let self_base = self_param.map(|self_param| strip_perm(&self_param.r#type));
    if let Some(self_param) = self_param {
        let r#mut = if self_param.recv_perm == Some(Permission::Unique) { ast::Mutability::Var } else { ast::Mutability::Let };
        let binding = TypeBinding { r#mut, r#type: self_param.r#type.clone(), ..Default::default() };
        env.scopes.last_mut().expect("a scope").insert(id_key_of("self"), binding);
    }
    for param in params {
        let Ok(lowered) = local_lower_type(ctx, &param.r#type) else {
            continue;
        };
        let ty = match &self_base {
            Some(self_base) if self_base.is_some() => subst_self_type(self_base, &lowered, None),
            _ => lowered,
        };
        env.scopes.last_mut().expect("a scope").insert(id_key_of(&param.name), TypeBinding { r#type: ty, ..Default::default() });
    }
}

/// See `BindCheckBody`.
pub fn bind_check_body(
    ctx: &ScopeContext<'_>,
    module_path: &[String],
    params: &[ast::Param],
    body: &ast::BlockPtr,
    self_param: Option<&BindSelfParam>,
) -> BindCheckResult {
    let Some(body) = body.as_deref() else {
        return BindCheckResult { ok: true, diag_id: None, span: None };
    };
    let mut env = TypeEnv::default();
    env.scopes.push(Default::default());
    let static_info = static_bind_map(ctx, module_path, &mut env);
    env.scopes.push(Default::default());
    param_type_map(ctx, params, self_param, &mut env);
    let state = BindStateBundle {
        binds: vec![static_info, param_bind_map(params, self_param)],
        perms: vec![PermScope::new(), PermScope::new()],
        env,
        keys_held: false,
        key_mode: None,
    };
    match (Checker { ctx }).block(body, state) {
        Ok(_) => BindCheckResult { ok: true, diag_id: None, span: None },
        Err(failure) => BindCheckResult { ok: false, diag_id: failure.diag_id, span: failure.span },
    }
}
