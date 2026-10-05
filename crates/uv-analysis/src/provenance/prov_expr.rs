//! The provenance of an expression's value: literals have none, places take their
//! root's, allocations take their region's, and other forms join their operands'.

use uv_core::span::Span;
use uv_source::ast::{self, ApplyArgs, EnumPayload, ExprNode, ExprPtr};

use crate::caps::builtin_paths::path_matches_builtin_name;
use crate::context::{IdKey, ScopeContext};
use crate::keys::key_paths::is_place_expression;
use crate::memory::regions::{region_active_type, ProvenanceKind};
use crate::resolve::scopes::{id_eq, id_key_of};
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{BindingProvenanceSeedKind, TypeBinding, TypeEnv};
use crate::typing::type_expr::type_expr;
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm;
use crate::typing::types::{TypeNode, TypeRef};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum ProvTag {
    Global,
    Stack(usize),
    Heap,
    Region(IdKey),
    #[default]
    Bottom,
    Param(usize),
}

impl ProvTag {
    pub(crate) fn kind(&self) -> ProvenanceKind {
        match self {
            ProvTag::Global => ProvenanceKind::Global,
            ProvTag::Stack(_) => ProvenanceKind::Stack,
            ProvTag::Heap => ProvenanceKind::Heap,
            ProvTag::Region(_) => ProvenanceKind::Region,
            ProvTag::Bottom => ProvenanceKind::Bottom,
            ProvTag::Param(_) => ProvenanceKind::Param,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProvScope {
    pub id: usize,
    pub map: std::collections::HashMap<IdKey, ProvTag>,
}

#[derive(Debug, Clone)]
pub(crate) struct RegionEntry {
    pub tag: IdKey,
    pub target: IdKey,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProvEnv {
    pub scopes: Vec<ProvScope>,
    pub regions: Vec<RegionEntry>,
    pub next_scope_id: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProvExprResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub prov: ProvTag,
}

fn ok(prov: ProvTag) -> ProvExprResult {
    ProvExprResult {
        ok: true,
        prov,
        ..Default::default()
    }
}

fn region_index(env: &ProvEnv, name: &str) -> Option<usize> {
    env.regions.iter().position(|entry| entry.tag == name)
}

fn region_nesting(env: &ProvEnv, inner: &str, outer: &str) -> bool {
    match (region_index(env, inner), region_index(env, outer)) {
        (Some(inner), Some(outer)) => inner > outer,
        _ => false,
    }
}

fn prov_rank(tag: &ProvTag) -> i32 {
    match tag {
        ProvTag::Region(_) => 0,
        ProvTag::Stack(_) => 1,
        ProvTag::Heap => 2,
        ProvTag::Global => 3,
        ProvTag::Bottom => 4,
        ProvTag::Param(_) => -1,
    }
}

pub(crate) fn prov_leq(env: &ProvEnv, lhs: &ProvTag, rhs: &ProvTag) -> bool {
    if lhs == rhs {
        return true;
    }
    if matches!(lhs, ProvTag::Param(_)) || matches!(rhs, ProvTag::Param(_)) {
        return false;
    }
    if let (ProvTag::Region(lhs), ProvTag::Region(rhs)) = (lhs, rhs) {
        return region_nesting(env, lhs, rhs);
    }
    let (lhs_rank, rhs_rank) = (prov_rank(lhs), prov_rank(rhs));
    lhs_rank >= 0 && rhs_rank >= 0 && lhs_rank < rhs_rank
}

fn join_prov(env: &ProvEnv, lhs: &ProvTag, rhs: &ProvTag) -> ProvTag {
    if prov_leq(env, lhs, rhs) {
        return lhs.clone();
    }
    if prov_leq(env, rhs, lhs) {
        return rhs.clone();
    }
    ProvTag::Bottom
}

fn join_all_prov(env: &ProvEnv, tags: &[ProvTag]) -> ProvTag {
    let Some((first, rest)) = tags.split_first() else {
        return ProvTag::Bottom;
    };
    rest.iter()
        .fold(first.clone(), |current, tag| join_prov(env, &current, tag))
}

fn lookup_pi(env: &ProvEnv, name: &str) -> Option<ProvTag> {
    let key = id_key_of(name);
    env.scopes
        .iter()
        .rev()
        .find_map(|scope| scope.map.get(&key).cloned())
}

pub(crate) fn stack_prov(env: &ProvEnv) -> ProvTag {
    env.scopes
        .last()
        .map_or(ProvTag::Bottom, |scope| ProvTag::Stack(scope.id))
}

fn binding_seed_tag(env: &ProvEnv, binding: &TypeBinding) -> ProvTag {
    match binding.provenance_kind {
        BindingProvenanceSeedKind::Global => ProvTag::Global,
        BindingProvenanceSeedKind::Stack => stack_prov(env),
        BindingProvenanceSeedKind::Heap => ProvTag::Heap,
        BindingProvenanceSeedKind::Region => binding
            .provenance_region
            .clone()
            .map_or(ProvTag::Bottom, ProvTag::Region),
        BindingProvenanceSeedKind::Bottom => ProvTag::Bottom,
        BindingProvenanceSeedKind::Param => ProvTag::Param(0),
    }
}

/// One scope holding every binding of the typing environment, with the active regions
/// in the order the environment iterates them.
pub(crate) fn seed_minimal_prov_env(gamma: &TypeEnv) -> ProvEnv {
    let mut env = ProvEnv::default();
    let mut scope = ProvScope {
        id: env.next_scope_id,
        ..Default::default()
    };
    env.next_scope_id += 1;
    for type_scope in &gamma.scopes {
        for (key, binding) in type_scope.iter() {
            if region_active_type(&binding.r#type) {
                let tag = binding
                    .provenance_region
                    .clone()
                    .unwrap_or_else(|| key.clone());
                env.regions.push(RegionEntry {
                    tag: tag.clone(),
                    target: key.clone(),
                });
                scope.map.insert(key.clone(), ProvTag::Region(tag));
                continue;
            }
            // The seed of a stack binding is read before the scope is pushed.
            let tag = binding_seed_tag(&env, binding);
            scope.map.insert(key.clone(), tag);
        }
    }
    env.scopes.push(scope);
    env
}

fn alloc_tag(env: &ProvEnv, target: &Option<String>) -> Option<IdKey> {
    let Some(target) = target else {
        return env.regions.last().map(|entry| entry.tag.clone());
    };
    let key = id_key_of(target);
    env.regions
        .iter()
        .rev()
        .find(|entry| entry.target == key)
        .map(|entry| entry.tag.clone())
}

fn resolve_region_target(env: &ProvEnv, tag: &ProvTag) -> Option<IdKey> {
    let ProvTag::Region(region) = tag else {
        return None;
    };
    env.regions
        .iter()
        .rev()
        .find(|entry| &entry.tag == region)
        .map(|entry| entry.target.clone())
}

/// The type of an expression typed on its own, outside any statement.
fn expr_type_for_provenance(
    ctx: &ScopeContext<'_>,
    env: &TypeEnv,
    expr: &ExprPtr,
) -> Option<TypeRef> {
    expr.as_ref()?;
    let type_ctx = StmtTypeContext::default();
    let typed = type_expr(ctx, &type_ctx, expr, env);
    (typed.ok && typed.r#type.is_some()).then_some(typed.r#type)
}

fn strip_perm_deep(ty: &TypeRef) -> TypeRef {
    let mut current = ty.clone();
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) =
        current.as_deref().map(|ty| &ty.node)
    {
        current = base.clone();
    }
    current
}

fn is_pointer_carrier_type(ty: &TypeRef) -> bool {
    matches!(
        strip_perm_deep(ty).as_deref().map(|ty| &ty.node),
        Some(TypeNode::Ptr { .. } | TypeNode::RawPtr { .. })
    )
}

fn children_ltr(expr: &ExprPtr) -> Vec<ExprPtr> {
    let Some(e) = expr.as_deref() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    match &e.node {
        ExprNode::QualifiedApplyExpr(node) => match &node.args {
            ApplyArgs::ParenArgs(paren) => {
                out.extend(paren.args.iter().map(|arg| arg.value.clone()))
            }
            ApplyArgs::BraceArgs(brace) => {
                out.extend(brace.fields.iter().map(|field| field.value.clone()))
            }
        },
        ExprNode::RangeExpr(node) => {
            if node.lhs.is_some() {
                out.push(node.lhs.clone());
            }
            if node.rhs.is_some() {
                out.push(node.rhs.clone());
            }
        }
        ExprNode::BinaryExpr(node) => {
            out.push(node.lhs.clone());
            out.push(node.rhs.clone());
        }
        ExprNode::CastExpr(node) => out.push(node.value.clone()),
        ExprNode::UnaryExpr(node) => out.push(node.value.clone()),
        ExprNode::DerefExpr(node) => out.push(node.value.clone()),
        ExprNode::AddressOfExpr(node) => out.push(node.place.clone()),
        ExprNode::MoveExpr(node) => out.push(node.place.clone()),
        ExprNode::AllocExpr(node) => out.push(node.value.clone()),
        ExprNode::TupleExpr(node) => out.extend(node.elements.iter().cloned()),
        ExprNode::ArrayExpr(node) => {
            out.extend(ast::array_expr_subexprs(node).into_iter().cloned())
        }
        ExprNode::ArrayRepeatExpr(node) => {
            out.push(node.value.clone());
            out.push(node.count.clone());
        }
        ExprNode::RecordExpr(node) => {
            out.extend(node.fields.iter().map(|field| field.value.clone()))
        }
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            Some(EnumPayload::EnumPayloadParen(tuple)) => {
                out.extend(tuple.elements.iter().cloned())
            }
            Some(EnumPayload::EnumPayloadBrace(rec)) => {
                out.extend(rec.fields.iter().map(|field| field.value.clone()))
            }
            None => {}
        },
        ExprNode::TransmuteExpr(node) => out.push(node.value.clone()),
        ExprNode::FieldAccessExpr(node) => out.push(node.base.clone()),
        ExprNode::TupleAccessExpr(node) => out.push(node.base.clone()),
        ExprNode::IndexAccessExpr(node) => {
            out.push(node.base.clone());
            out.push(node.index.clone());
        }
        ExprNode::CallExpr(node) => {
            out.push(node.callee.clone());
            out.extend(node.args.iter().map(|arg| arg.value.clone()));
        }
        ExprNode::MethodCallExpr(node) => {
            out.push(node.receiver.clone());
            out.extend(node.args.iter().map(|arg| arg.value.clone()));
        }
        ExprNode::PropagateExpr(node) => out.push(node.value.clone()),
        _ => {}
    }
    out
}

pub(crate) fn prov_place(
    ctx: &ScopeContext<'_>,
    place: &ExprPtr,
    env: &ProvEnv,
    gamma: &TypeEnv,
) -> ProvExprResult {
    let Some(e) = place.as_deref() else {
        return ok(ProvTag::Bottom);
    };
    match &e.node {
        ExprNode::IdentifierExpr(node) => ok(lookup_pi(env, &node.name).unwrap_or_default()),
        ExprNode::FieldAccessExpr(node) => prov_place(ctx, &node.base, env, gamma),
        ExprNode::TupleAccessExpr(node) => prov_place(ctx, &node.base, env, gamma),
        ExprNode::IndexAccessExpr(node) => prov_place(ctx, &node.base, env, gamma),
        ExprNode::DerefExpr(node) => prov_expr(ctx, &node.value, env, gamma),
        _ => ok(ProvTag::Bottom),
    }
}

fn block_tail_only(block: &ast::BlockPtr) -> Option<&ExprPtr> {
    block
        .as_deref()
        .filter(|block| block.tail_opt.is_some() && block.stmts.is_empty())
        .map(|block| &block.tail_opt)
}

pub(crate) fn prov_expr(
    ctx: &ScopeContext<'_>,
    expr: &ExprPtr,
    env: &ProvEnv,
    gamma: &TypeEnv,
) -> ProvExprResult {
    let Some(e) = expr.as_deref() else {
        return ok(ProvTag::Bottom);
    };
    match &e.node {
        ExprNode::LiteralExpr(_) => return ok(ProvTag::Bottom),
        ExprNode::MoveExpr(node) => return prov_place(ctx, &node.place, env, gamma),
        ExprNode::AddressOfExpr(node) => return prov_place(ctx, &node.place, env, gamma),
        ExprNode::AllocExpr(alloc) => {
            let inner = prov_expr(ctx, &alloc.value, env, gamma);
            if !inner.ok {
                return inner;
            }
            if let Some(tag) = alloc_tag(env, &alloc.region_opt) {
                return ok(ProvTag::Region(tag));
            }
            if alloc.region_opt.is_none() {
                return ProvExprResult {
                    ok: false,
                    diag_id: Some("E-MEM-3021"),
                    span: Some(e.span.clone()),
                    prov: ProvTag::Bottom,
                };
            }
            return ok(ProvTag::Bottom);
        }
        ExprNode::MethodCallExpr(call) => {
            if id_eq(&call.name, "alloc") {
                let recv = prov_expr(ctx, &call.receiver, env, gamma);
                if !recv.ok {
                    return recv;
                }
                if expr_type_for_provenance(ctx, gamma, &call.receiver)
                    .is_some_and(|ty| region_active_type(&ty))
                {
                    for arg in &call.args {
                        let arg_res = prov_expr(ctx, &arg.value, env, gamma);
                        if !arg_res.ok {
                            return arg_res;
                        }
                    }
                    let prov = if matches!(recv.prov, ProvTag::Region(_)) {
                        recv.prov
                    } else {
                        ProvTag::Bottom
                    };
                    return ok(prov);
                }
            }
            if id_eq(&call.name, "alloc_raw") {
                let ty = expr_type_for_provenance(ctx, gamma, &call.receiver);
                let stripped = ty.map(|ty| strip_perm(&ty)).unwrap_or(None);
                if let Some(TypeNode::Dynamic(path)) = stripped.as_deref().map(|ty| &ty.node) {
                    if path_matches_builtin_name(path, "HeapAllocator") {
                        for arg in &call.args {
                            let arg_res = prov_expr(ctx, &arg.value, env, gamma);
                            if !arg_res.ok {
                                return arg_res;
                            }
                        }
                        return ok(ProvTag::Heap);
                    }
                }
            }
        }
        ExprNode::TransmuteExpr(transmute) => {
            let from = match lower_type(ctx, &transmute.from) {
                Ok(ty) if transmute.from.is_some() => ty,
                Ok(_) => return ProvExprResult::default(),
                Err(diag_id) => {
                    return ProvExprResult {
                        diag_id,
                        ..Default::default()
                    }
                }
            };
            let to = match lower_type(ctx, &transmute.to) {
                Ok(ty) if transmute.to.is_some() => ty,
                Ok(_) => return ProvExprResult::default(),
                Err(diag_id) => {
                    return ProvExprResult {
                        diag_id,
                        ..Default::default()
                    }
                }
            };
            if !is_pointer_carrier_type(&from) || !is_pointer_carrier_type(&to) {
                return ok(ProvTag::Bottom);
            }
            return prov_expr(ctx, &transmute.value, env, gamma);
        }
        ExprNode::AttributedExpr(node) => return prov_expr(ctx, &node.expr, env, gamma),
        ExprNode::UnsafeBlockExpr(node) => {
            return match block_tail_only(&node.block) {
                Some(tail) => prov_expr(ctx, tail, env, gamma),
                None => ok(ProvTag::Bottom),
            };
        }
        ExprNode::BlockExpr(node) => {
            return match block_tail_only(&node.block) {
                Some(tail) => prov_expr(ctx, tail, env, gamma),
                None => ok(ProvTag::Bottom),
            };
        }
        ExprNode::IfExpr(node) => {
            let cond = prov_expr(ctx, &node.cond, env, gamma);
            if !cond.ok {
                return cond;
            }
            let then_res = prov_expr(ctx, &node.then_expr, env, gamma);
            if !then_res.ok {
                return then_res;
            }
            if node.else_expr.is_some() {
                let else_res = prov_expr(ctx, &node.else_expr, env, gamma);
                if !else_res.ok {
                    return else_res;
                }
                return ok(join_prov(env, &then_res.prov, &else_res.prov));
            }
            return ok(ProvTag::Bottom);
        }
        _ => {}
    }
    if is_place_expression(expr) {
        return prov_place(ctx, expr, env, gamma);
    }
    let mut child_provs = Vec::new();
    for child in children_ltr(expr) {
        let child_res = prov_expr(ctx, &child, env, gamma);
        if !child_res.ok {
            return child_res;
        }
        child_provs.push(child_res.prov);
    }
    ok(join_all_prov(env, &child_provs))
}

#[derive(Debug, Clone)]
pub struct ProvExprTrackResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub kind: ProvenanceKind,
    pub region: Option<IdKey>,
    pub region_target: Option<IdKey>,
}

pub fn track_expr_provenance(
    ctx: &ScopeContext<'_>,
    expr: &ExprPtr,
    gamma: &TypeEnv,
) -> ProvExprTrackResult {
    let env = seed_minimal_prov_env(gamma);
    let result = prov_expr(ctx, expr, &env, gamma);
    let (region, region_target) = match &result.prov {
        ProvTag::Region(region) => (
            Some(region.clone()),
            resolve_region_target(&env, &result.prov),
        ),
        _ => (None, None),
    };
    ProvExprTrackResult {
        ok: result.ok,
        diag_id: result.diag_id,
        span: result.span,
        kind: result.prov.kind(),
        region,
        region_target,
    }
}
