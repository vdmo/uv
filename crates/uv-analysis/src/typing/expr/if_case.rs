//! `if … is` and `if … case`: each arm's pattern is typed against the scrutinee, a
//! scrutinee that is a name is narrowed to what the pattern matched (and in `else` to
//! what the patterns rejected), and without `else` the arms must be exhaustive.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uv_source::ast::{self, ExprNode, ExprPtr, PatternNode};

use crate::context::{IdKey, ScopeContext};
use crate::modal::lookup::{has_state, lookup_modal_decl, state_name_set};
use crate::resolve::scopes::{id_eq, id_key_of, reserved_gen};
use crate::typing::alias_normalize::expand_type_alias_apply;
use crate::typing::callbacks::CheckResult;
use crate::typing::expr_result::ExprTypeResult;
use crate::typing::pattern::{enum_pattern_covers_variant, irrefutable_pattern, modal_pattern_covers_state, type_pattern_against_type};
use crate::typing::stmt::block::{check_block, type_block};
use crate::typing::stmt_context::{with_shared_access_mode, StmtTypeContext};
use crate::typing::subtyping::subtyping;
use crate::typing::type_env::{bind_of, push_scope, TypeBinding, TypeEnv};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_expr::{check_expr_against, type_expr, type_identifier_expr, type_place};
use crate::typing::type_lookup::{async_sig_of, lookup_enum_decl};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::strip_perm;
use crate::typing::types::*;

type Diag = Option<&'static str>;

fn is_never(ty: &TypeRef) -> bool {
    matches!(strip_perm(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "!")
}

/// An asynchronous type under any alias is matched as `Async<Out, In, Result, E>`.
fn canonical_async_pattern_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> TypeRef {
    if matches!(ty.as_deref().map(|ty| &ty.node), None | Some(TypeNode::ModalState(_) | TypeNode::Union(_))) {
        return ty.clone();
    }
    match async_sig_of(ctx, ty) {
        Some(sig) => make_type_path_with(vec!["Async".to_string()], vec![sig.out, sig.input, sig.result, sig.err]),
        None => ty.clone(),
    }
}

fn strip_perm_once(ty: &TypeRef) -> &TypeRef {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. }) => base,
        _ => ty,
    }
}

fn type_equiv_ignore_perm(lhs: &TypeRef, rhs: &TypeRef) -> bool {
    type_equiv(strip_perm_once(lhs), strip_perm_once(rhs))
}

/// The type with aliases at its top expanded, and whether any was.
fn normalize_alias_top_level(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Result<(TypeRef, bool), Diag> {
    let mut current = ty.clone();
    let mut expanded_any = false;
    for _ in 0..16 {
        let Some(t) = current.as_deref() else {
            break;
        };
        let (Some(path), Some(args)) = (applied_type_path(t), applied_type_args(t)) else {
            break;
        };
        let expanded = expand_type_alias_apply(ctx, path, args)?;
        if expanded.is_none() {
            break;
        }
        current = expanded;
        expanded_any = true;
    }
    Ok((current, expanded_any))
}

fn scrutinee_binding_type_detail(scrutinee: &ExprPtr, env: &TypeEnv) -> String {
    let place = match scrutinee.as_deref().map(|scrutinee| &scrutinee.node) {
        Some(ExprNode::MoveExpr(node)) => &node.place,
        _ => scrutinee,
    };
    let Some(ExprNode::IdentifierExpr(ident)) = place.as_deref().map(|place| &place.node) else {
        return String::new();
    };
    match bind_of(env, &ident.name) {
        Some(binding) => format!(" with binding {}: {}", ident.name, type_to_string(&binding.r#type)),
        None => format!(" with no binding for {}", ident.name),
    }
}

/// Adds a pattern's bindings to the innermost scope. A name may not be bound twice
/// there, nor reuse a name of an enclosing scope.
fn intro_all(env: TypeEnv, binds: &[(String, TypeRef)], r#mut: ast::Mutability) -> Result<TypeEnv, Diag> {
    let mut current = env;
    for (name, ty) in binds {
        if reserved_gen(name) {
            return Err(Some("Intro-Reserved-Gen-Err"));
        }
        let key = id_key_of(name);
        let Some((innermost, outer)) = current.scopes.split_last_mut() else {
            return Err(None);
        };
        if innermost.contains_key(&key) {
            return Err(None);
        }
        if outer.iter().any(|scope| scope.contains_key(&key)) {
            return Err(Some("Intro-Outer-Err"));
        }
        innermost.insert(key, TypeBinding { r#mut, r#type: ty.clone(), ..Default::default() });
    }
    Ok(current)
}

fn scrutinee_identifier(scrutinee: &ExprPtr) -> Option<IdKey> {
    match &scrutinee.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => Some(id_key_of(&ident.name)),
        _ => None,
    }
}

/// The environment with the scrutinee, when it is a name, narrowed to the type.
fn refine_scrutinee_env(scrutinee: &ExprPtr, env: &TypeEnv, refined_type: &TypeRef) -> TypeEnv {
    let mut out = env.clone();
    if let (Some(ident), true) = (scrutinee_identifier(scrutinee), refined_type.is_some()) {
        if let Some(binding) = out.scopes.iter_mut().rev().find_map(|scope| scope.get_mut(&ident)) {
            binding.r#type = refined_type.clone();
        }
    }
    out
}

fn pattern_node(pattern: &ast::PatternPtr) -> Option<&PatternNode> {
    pattern.as_deref().map(|pattern| &pattern.node)
}

/// `Ok(None)` when the pattern does not match the type.
type Narrowed = Result<Option<TypeRef>, Diag>;

/// A modal pattern narrows a modal to the state it names.
fn concrete_modal_pattern_type(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> Option<TypeRef> {
    let Some(PatternNode::ModalPattern(modal_pattern)) = pattern_node(pattern) else {
        return Some(expected.clone());
    };
    let expected_base = strip_perm_once(expected).as_deref()?;
    if let TypeNode::ModalState(modal_state) = &expected_base.node {
        return id_eq(&modal_state.state, &modal_pattern.state).then(|| expected.clone());
    }
    let path = applied_type_path(expected_base)?;
    let modal_decl = lookup_modal_decl(ctx, path)?;
    if !has_state(modal_decl, &modal_pattern.state) {
        return None;
    }
    Some(make_type_modal_state(path.clone(), &modal_pattern.state, applied_type_args(expected_base).unwrap_or(&[]).to_vec()))
}

/// The part of the type the pattern matches: for a union, the members it matches.
fn pattern_matched_type(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> Narrowed {
    let (Some(_), Some(expected_ty)) = (pattern.as_deref(), expected.as_deref()) else {
        return Ok(None);
    };
    if let TypeNode::Perm { perm, base } = &expected_ty.node {
        return Ok(pattern_matched_type(ctx, pattern, base)?.map(|narrowed| make_type_perm(*perm, narrowed)));
    }
    let (normalized, expanded) = normalize_alias_top_level(ctx, expected)?;
    if expanded && normalized.is_some() {
        return pattern_matched_type(ctx, pattern, &normalized);
    }
    let expected_base = strip_perm_once(&normalized);
    let Some(base) = expected_base.as_deref() else {
        return Ok(None);
    };
    if let TypeNode::Union(members) = &base.node {
        let mut matched_members = Vec::new();
        let mut first_diag: Diag = None;
        for member in members {
            match pattern_matched_type(ctx, pattern, member) {
                Err(diag_id) => first_diag = first_diag.or(diag_id),
                Ok(Some(narrowed)) if narrowed.is_some() => matched_members.push(narrowed),
                Ok(_) => {}
            }
        }
        return match matched_members.len() {
            0 if first_diag.is_some() => Err(first_diag),
            0 => Ok(None),
            1 => Ok(matched_members.pop()),
            _ => Ok(Some(make_type_union(matched_members))),
        };
    }
    type_pattern_against_type(ctx, pattern, expected_base)?;
    if let Some(PatternNode::TypedPattern(typed)) = pattern_node(pattern) {
        return Ok(Some(lower_type(ctx, &typed.r#type)?));
    }
    Ok(concrete_modal_pattern_type(ctx, pattern, expected))
}

fn pattern_contains_typed_pattern(pattern: &ast::PatternPtr) -> bool {
    let fields = |fields: &[ast::FieldPattern]| fields.iter().any(|field| pattern_contains_typed_pattern(&field.pattern_opt));
    match pattern_node(pattern) {
        Some(PatternNode::TypedPattern(_)) => true,
        Some(PatternNode::TuplePattern(node)) => node.elements.iter().any(pattern_contains_typed_pattern),
        Some(PatternNode::RecordPattern(node)) => fields(&node.fields),
        Some(PatternNode::EnumPattern(node)) => match &node.payload_opt {
            Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => payload.elements.iter().any(pattern_contains_typed_pattern),
            Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => fields(&payload.fields),
            None => false,
        },
        Some(PatternNode::ModalPattern(node)) => node.fields_opt.as_ref().is_some_and(|payload| fields(&payload.fields)),
        Some(PatternNode::RangePattern(node)) => pattern_contains_typed_pattern(&node.lo) || pattern_contains_typed_pattern(&node.hi),
        _ => false,
    }
}

/// The part of a union the pattern does not match, when that is some but not all of it.
fn pattern_rejected_type(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> Narrowed {
    let (Some(_), Some(expected_ty)) = (pattern.as_deref(), expected.as_deref()) else {
        return Ok(None);
    };
    if let TypeNode::Perm { perm, base } = &expected_ty.node {
        return Ok(pattern_rejected_type(ctx, pattern, base)?.map(|rejected| make_type_perm(*perm, rejected)));
    }
    let (normalized, expanded) = normalize_alias_top_level(ctx, expected)?;
    if expanded && normalized.is_some() {
        return pattern_rejected_type(ctx, pattern, &normalized);
    }
    let Some(TypeNode::Union(members)) = strip_perm_once(&normalized).as_deref().map(|ty| &ty.node) else {
        return Ok(None);
    };
    let mut rejected_members = Vec::with_capacity(members.len());
    for member in members {
        match pattern_matched_type(ctx, pattern, member) {
            Err(diag_id @ Some(_)) => return Err(diag_id),
            Ok(Some(narrowed)) if narrowed.is_some() => {}
            _ => rejected_members.push(member.clone()),
        }
    }
    if rejected_members.is_empty() || rejected_members.len() == members.len() {
        return Ok(None);
    }
    Ok(Some(if rejected_members.len() == 1 { rejected_members.remove(0) } else { make_type_union(rejected_members) }))
}

/// The environment of an arm: the scrutinee narrowed, and the pattern's bindings in a
/// scope of their own.
fn case_scope_env(
    ctx: &ScopeContext<'_>,
    scrutinee: &ExprPtr,
    env: &TypeEnv,
    pattern: &ast::PatternPtr,
    scrutinee_type: &TypeRef,
) -> Result<TypeEnv, Diag> {
    let bindings = match type_pattern_against_type(ctx, pattern, scrutinee_type) {
        Ok(bindings) => bindings,
        Err(None) if pattern_contains_typed_pattern(pattern) => return Err(Some("IfIs-TypedPattern-Incompatible")),
        Err(diag_id) => return Err(diag_id),
    };
    let case_base = match pattern_matched_type(ctx, pattern, scrutinee_type)? {
        Some(matched) if matched.is_some() => refine_scrutinee_env(scrutinee, env, &matched),
        _ => env.clone(),
    };
    intro_all(push_scope(&case_base), &bindings, ast::Mutability::Let)
}

fn else_scope_env(
    ctx: &ScopeContext<'_>,
    scrutinee: &ExprPtr,
    env: &TypeEnv,
    pattern: &ast::PatternPtr,
    scrutinee_type: &TypeRef,
) -> Result<TypeEnv, Diag> {
    Ok(match pattern_rejected_type(ctx, pattern, scrutinee_type)? {
        Some(rejected) if rejected.is_some() => refine_scrutinee_env(scrutinee, env, &rejected),
        _ => env.clone(),
    })
}

/// In `else` the scrutinee is what every arm rejected.
fn else_scope_for_cases(
    ctx: &ScopeContext<'_>,
    scrutinee: &ExprPtr,
    env: &TypeEnv,
    arms: &[ast::IfCaseClause],
    scrutinee_type: &TypeRef,
) -> Result<TypeEnv, Diag> {
    let mut remaining_type = scrutinee_type.clone();
    let mut narrowed = false;
    for arm in arms {
        if let Some(rejected) = pattern_rejected_type(ctx, &arm.pattern, &remaining_type)?.filter(|rejected| rejected.is_some()) {
            remaining_type = rejected;
            narrowed = true;
        }
    }
    Ok(if narrowed && remaining_type.is_some() { refine_scrutinee_env(scrutinee, env, &remaining_type) } else { env.clone() })
}

fn arm_variants(ctx: &ScopeContext<'_>, arms: &[ast::IfCaseClause], expected_path: &[String], expected: &TypeRef) -> HashSet<IdKey> {
    arms.iter()
        .filter_map(|arm| match pattern_node(&arm.pattern) {
            Some(PatternNode::EnumPattern(enum_pat))
                if enum_pat.path == expected_path && enum_pattern_covers_variant(ctx, &arm.pattern, expected) =>
            {
                Some(id_key_of(&enum_pat.name))
            }
            _ => None,
        })
        .collect()
}

fn arm_states(ctx: &ScopeContext<'_>, arms: &[ast::IfCaseClause], expected: &TypeRef) -> HashSet<IdKey> {
    let mut out = HashSet::new();
    for arm in arms {
        match pattern_node(&arm.pattern) {
            Some(PatternNode::ModalPattern(modal_pat)) => {
                if modal_pattern_covers_state(ctx, &arm.pattern, expected) {
                    out.insert(id_key_of(&modal_pat.state));
                }
            }
            Some(PatternNode::TypedPattern(typed)) => {
                if let Ok(lowered) = lower_type(ctx, &typed.r#type) {
                    if let Some(TypeNode::ModalState(modal_state)) = strip_perm_once(&lowered).as_deref().map(|ty| &ty.node) {
                        out.insert(id_key_of(&modal_state.state));
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn has_irrefutable_arm(ctx: &ScopeContext<'_>, arms: &[ast::IfCaseClause], expected: &TypeRef) -> bool {
    arms.iter().any(|arm| irrefutable_pattern(ctx, &arm.pattern, expected))
}

/// What an arm selects, by which two arms select the same thing.
enum ClauseLabel {
    Enum(Vec<String>, IdKey),
    Modal(IdKey),
    Union(TypeRef),
}

fn arm_case_label_of(ctx: &ScopeContext<'_>, scrutinee: &TypeRef, pattern: &ast::PatternPtr) -> Result<Option<ClauseLabel>, Diag> {
    let (Some(node), Some(base)) = (pattern_node(pattern), strip_perm_once(scrutinee).as_deref()) else {
        return Ok(None);
    };
    let union_members = match &base.node {
        TypeNode::Union(members) => Some(members),
        _ => None,
    };
    // The first member of a union the pattern types against.
    let union_label = || -> Result<Option<ClauseLabel>, Diag> {
        for member in union_members.into_iter().flatten() {
            match type_pattern_against_type(ctx, pattern, member) {
                Ok(_) => return Ok(Some(ClauseLabel::Union(member.clone()))),
                Err(diag_id @ Some(_)) => return Err(diag_id),
                Err(None) => {}
            }
        }
        Ok(None)
    };
    match node {
        PatternNode::EnumPattern(enum_pat) => {
            Ok(Some(union_label()?.unwrap_or_else(|| ClauseLabel::Enum(enum_pat.path.clone(), id_key_of(&enum_pat.name)))))
        }
        PatternNode::ModalPattern(modal_pat) => Ok(Some(union_label()?.unwrap_or_else(|| ClauseLabel::Modal(id_key_of(&modal_pat.state))))),
        PatternNode::TypedPattern(typed) => {
            let Some(members) = union_members else {
                return Ok(None);
            };
            let lowered = lower_type(ctx, &typed.r#type)?;
            if lowered.is_none() {
                return Err(None);
            }
            Ok(members.iter().find(|member| type_equiv_ignore_perm(&lowered, member)).map(|member| ClauseLabel::Union(member.clone())))
        }
        _ => Ok(None),
    }
}

fn arm_case_label_equal(lhs: &ClauseLabel, rhs: &ClauseLabel) -> bool {
    match (lhs, rhs) {
        (ClauseLabel::Enum(lhs_path, lhs_name), ClauseLabel::Enum(rhs_path, rhs_name)) => lhs_path == rhs_path && lhs_name == rhs_name,
        (ClauseLabel::Modal(lhs_name), ClauseLabel::Modal(rhs_name)) => lhs_name == rhs_name,
        (ClauseLabel::Union(lhs_type), ClauseLabel::Union(rhs_type)) => type_equiv_ignore_perm(lhs_type, rhs_type),
        _ => false,
    }
}

/// An arm is unreachable after an irrefutable arm, or after one selecting the same thing.
fn arm_unreachable(ctx: &ScopeContext<'_>, scrutinee: &TypeRef, arms: &[ast::IfCaseClause], index: usize) -> Result<bool, Diag> {
    if scrutinee.is_none() {
        return Ok(false);
    }
    let earlier = || arms[..index].iter().filter(|arm| arm.pattern.is_some());
    if earlier().any(|arm| irrefutable_pattern(ctx, &arm.pattern, scrutinee)) {
        return Ok(true);
    }
    if arms[index].pattern.is_none() {
        return Ok(false);
    }
    let Some(current_label) = arm_case_label_of(ctx, scrutinee, &arms[index].pattern)? else {
        return Ok(false);
    };
    for prev in earlier() {
        if let Some(prev_label) = arm_case_label_of(ctx, scrutinee, &prev.pattern)? {
            if arm_case_label_equal(&prev_label, &current_label) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn find_unreachable_arm(ctx: &ScopeContext<'_>, scrutinee: &TypeRef, arms: &[ast::IfCaseClause]) -> Result<bool, Diag> {
    for index in 0..arms.len() {
        if arm_unreachable(ctx, scrutinee, arms, index)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn unify_branch_types(ctx: &ScopeContext<'_>, lhs: &TypeRef, rhs: &TypeRef) -> TypeRef {
    if is_never(lhs) {
        return rhs.clone();
    }
    if is_never(rhs) {
        return lhs.clone();
    }
    if type_equiv(lhs, rhs) {
        return lhs.clone();
    }
    let lhs_sub = subtyping(ctx, lhs, rhs);
    if lhs_sub.ok && lhs_sub.subtype {
        return rhs.clone();
    }
    let rhs_sub = subtyping(ctx, rhs, lhs);
    if rhs_sub.ok && rhs_sub.subtype {
        return lhs.clone();
    }
    None
}

/// An arm's body: a block is typed with an environment of its own to follow.
fn type_arm_body(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, body: &ExprPtr, env: &TypeEnv) -> ExprTypeResult {
    let Some(ExprNode::BlockExpr(block_expr)) = body.as_deref().map(|body| &body.node) else {
        return if body.is_some() { type_expr(ctx, type_ctx, body, env) } else { ExprTypeResult::default() };
    };
    let Some(block) = block_expr.block.as_deref() else {
        return ExprTypeResult::default();
    };
    let live_env = Rc::new(RefCell::new(env.clone()));
    let mut arm_ctx = type_ctx.clone();
    arm_ctx.env_ref = Some(live_env.clone());
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, &arm_ctx, inner, &live_env.borrow().clone());
    let ident_fn = |name: &str| type_identifier_expr(ctx, &live_env.borrow().clone(), name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, &arm_ctx, inner, &live_env.borrow().clone());
    type_block(ctx, &arm_ctx, block, env, &expr_fn, &ident_fn, &place_fn, Some(&live_env))
}

fn check_arm_body(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, body: &ExprPtr, env: &TypeEnv, expected: &TypeRef) -> CheckResult {
    let strip = |check: CheckResult| CheckResult {
        ok: check.ok,
        diag_id: check.diag_id,
        diag_detail: check.diag_detail,
        diag_span: check.diag_span,
        ..Default::default()
    };
    if body.is_none() || expected.is_none() {
        return CheckResult::default();
    }
    let Some(ExprNode::BlockExpr(block_expr)) = body.as_deref().map(|body| &body.node) else {
        return strip(check_expr_against(ctx, type_ctx, body, expected, env));
    };
    let Some(block) = block_expr.block.as_deref() else {
        return CheckResult::default();
    };
    let live_env = Rc::new(RefCell::new(env.clone()));
    let mut arm_ctx = type_ctx.clone();
    arm_ctx.env_ref = Some(live_env.clone());
    let expr_fn = |inner: &ExprPtr| type_expr(ctx, &arm_ctx, inner, &live_env.borrow().clone());
    let ident_fn = |name: &str| type_identifier_expr(ctx, &live_env.borrow().clone(), name);
    let place_fn = |inner: &ExprPtr| type_place(ctx, &arm_ctx, inner, &live_env.borrow().clone());
    strip(check_block(ctx, &arm_ctx, block, env, expected, &expr_fn, &ident_fn, &place_fn, Some(&live_env)))
}

/// The scrutinee as patterns see it: its type under aliases, and that type under the
/// scrutinee's permission.
struct Scrutinee {
    base: TypeRef,
    match_type: TypeRef,
}

fn type_scrutinee(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    scrutinee: &ExprPtr,
    env: &TypeEnv,
) -> Result<Scrutinee, Box<ExprTypeResult>> {
    let typed = type_expr(ctx, &with_shared_access_mode(type_ctx, ast::KeyMode::Read), scrutinee, env);
    if !typed.ok {
        return Err(Box::new(typed));
    }
    let (normalized, _) =
        normalize_alias_top_level(ctx, &strip_perm(&typed.r#type)).map_err(|diag_id| Box::new(ExprTypeResult::failed(diag_id)))?;
    let base = canonical_async_pattern_type(ctx, &normalized);
    let match_type = match (typed.r#type.as_deref().map(|ty| &ty.node), base.is_some()) {
        (Some(TypeNode::Perm { perm, .. }), true) => make_type_perm(*perm, base.clone()),
        _ => base.clone(),
    };
    Ok(Scrutinee { base, match_type })
}

pub fn type_if_is_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::IfIsExpr, env: &TypeEnv) -> ExprTypeResult {
    if expr.scrutinee.is_none() || expr.pattern.is_none() || expr.then_expr.is_none() {
        return ExprTypeResult::default();
    }
    let scrutinee = match type_scrutinee(ctx, type_ctx, &expr.scrutinee, env) {
        Ok(scrutinee) => scrutinee,
        Err(failure) => return ExprTypeResult::failed(failure.diag_id),
    };
    if scrutinee.base.is_none() {
        return ExprTypeResult { diag_detail: "if-is scrutinee type could not be canonicalized".to_string(), ..Default::default() };
    }
    let then_env = match case_scope_env(ctx, &expr.scrutinee, env, &expr.pattern, &scrutinee.match_type) {
        Ok(then_env) => then_env,
        Err(diag_id) => return ExprTypeResult::failed(diag_id),
    };
    let unit_type = make_type_prim("()");
    if expr.else_expr.is_none() {
        let unit_check = check_arm_body(ctx, type_ctx, &expr.then_expr, &then_env, &unit_type);
        if !unit_check.ok {
            return ExprTypeResult::failed(unit_check.diag_id);
        }
        return ExprTypeResult::typed(unit_type);
    }
    let else_env = match else_scope_env(ctx, &expr.scrutinee, env, &expr.pattern, &scrutinee.base) {
        Ok(else_env) => else_env,
        Err(diag_id) => return ExprTypeResult::failed(diag_id),
    };
    let then_typed = type_arm_body(ctx, type_ctx, &expr.then_expr, &then_env);
    if !then_typed.ok {
        return ExprTypeResult::failed(then_typed.diag_id);
    }
    let else_typed = type_expr(ctx, type_ctx, &expr.else_expr, &else_env);
    if !else_typed.ok {
        return ExprTypeResult::failed(else_typed.diag_id);
    }
    match unify_branch_types(ctx, &then_typed.r#type, &else_typed.r#type) {
        unified @ Some(_) => ExprTypeResult::typed(unified),
        None => ExprTypeResult::failed(Some("If-Branch-Mismatch")),
    }
}

pub fn check_if_is_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::IfIsExpr,
    env: &TypeEnv,
    expected: &TypeRef,
) -> CheckResult {
    let failed = |diag_id: Diag| CheckResult { diag_id, ..Default::default() };
    let or_detail = |check: CheckResult, fallback: &str| CheckResult {
        diag_detail: if check.diag_detail.is_empty() { fallback.to_string() } else { check.diag_detail },
        ..check
    };
    if expr.scrutinee.is_none() || expr.pattern.is_none() || expr.then_expr.is_none() || expected.is_none() {
        return CheckResult::default();
    }
    let scrutinee = match type_scrutinee(ctx, type_ctx, &expr.scrutinee, env) {
        Ok(scrutinee) => scrutinee,
        Err(failure) => return failed(failure.diag_id),
    };
    if scrutinee.base.is_none() {
        return CheckResult::default();
    }
    let then_env = match case_scope_env(ctx, &expr.scrutinee, env, &expr.pattern, &scrutinee.match_type) {
        Ok(then_env) => then_env,
        Err(diag_id) => return failed(diag_id),
    };
    if expr.else_expr.is_none() {
        let unit_type = make_type_prim("()");
        let then_check = check_arm_body(ctx, type_ctx, &expr.then_expr, &then_env, &unit_type);
        if !then_check.ok {
            let diag_id = then_check.diag_id.or(Some("If-Branch-Mismatch"));
            return or_detail(CheckResult { diag_id, ..then_check }, "if-is then branch failed checking");
        }
        let sub = subtyping(ctx, &unit_type, expected);
        if !sub.ok {
            return failed(sub.diag_id);
        }
        if !sub.subtype {
            return failed(sub.diag_id.or(Some("If-Branch-Mismatch")));
        }
        return CheckResult { ok: true, ..Default::default() };
    }
    let then_check = check_arm_body(ctx, type_ctx, &expr.then_expr, &then_env, expected);
    if !then_check.ok {
        return or_detail(then_check, "if-is then branch failed checking");
    }
    let else_env = match else_scope_env(ctx, &expr.scrutinee, env, &expr.pattern, &scrutinee.base) {
        Ok(else_env) => else_env,
        Err(diag_id) => return failed(diag_id),
    };
    let else_check = check_expr_against(ctx, type_ctx, &expr.else_expr, expected, &else_env);
    if !else_check.ok {
        return or_detail(
            CheckResult { ok: false, diag_id: else_check.diag_id, diag_detail: else_check.diag_detail, diag_span: else_check.diag_span, ..Default::default() },
            "if-is else branch failed checking",
        );
    }
    CheckResult { ok: true, ..Default::default() }
}

/// A failure of the checks after the arms: unreachable arms and exhaustiveness.
struct CaseFailure {
    diag_id: Diag,
    diag_detail: String,
    at_scrutinee: bool,
}

fn check_arms_cover(
    ctx: &ScopeContext<'_>,
    expr: &ast::IfCaseExpr,
    scrutinee: &Scrutinee,
) -> Result<(), CaseFailure> {
    let fail = |diag_id: &'static str, diag_detail: &str, at_scrutinee: bool| {
        Err(CaseFailure { diag_id: Some(diag_id), diag_detail: diag_detail.to_string(), at_scrutinee })
    };
    match find_unreachable_arm(ctx, &scrutinee.match_type, &expr.cases) {
        Err(diag_id) => {
            return Err(CaseFailure { diag_id, diag_detail: "if-case unreachable-arm analysis failed".to_string(), at_scrutinee: false })
        }
        Ok(true) => return fail("E-SEM-2751", "if-case contains an unreachable arm", false),
        Ok(false) => {}
    }
    if expr.else_expr.is_some() || has_irrefutable_arm(ctx, &expr.cases, &scrutinee.match_type) {
        return Ok(());
    }
    let Some(base) = scrutinee.base.as_deref() else {
        return Ok(());
    };
    let path_type = applied_type_path(base);
    let enum_decl = path_type.and_then(|path| lookup_enum_decl(ctx, path));
    let modal_decl = path_type.filter(|_| enum_decl.is_none()).and_then(|path| lookup_modal_decl(ctx, path));
    if let (Some(enum_decl), Some(path)) = (enum_decl, path_type) {
        let decl_variants: HashSet<IdKey> = enum_decl.variants.iter().map(|variant| id_key_of(&variant.name)).collect();
        if arm_variants(ctx, &expr.cases, path, &scrutinee.match_type) != decl_variants {
            return fail("E-SEM-2741", "if-case over enum type is not exhaustive and has no else branch", true);
        }
    } else if let Some(modal_decl) = modal_decl {
        // An asynchronous computation that cannot fail has no `Failed` state to cover.
        let mut decl_states = state_name_set(modal_decl);
        if async_sig_of(ctx, &strip_perm(&scrutinee.match_type)).is_some_and(|sig| is_never(&sig.err)) {
            decl_states.remove(&id_key_of("Failed"));
        }
        if arm_states(ctx, &expr.cases, &scrutinee.match_type) != decl_states {
            return fail("E-TYP-2060", "if-case over modal type is not exhaustive and has no else branch", true);
        }
    } else if let TypeNode::Union(members) = &base.node {
        let exhaustive = members.iter().all(|member| expr.cases.iter().any(|arm| irrefutable_pattern(ctx, &arm.pattern, member)));
        if !exhaustive {
            return fail("E-SEM-2705", "if-case over union type is not exhaustive and has no else branch", true);
        }
    } else {
        return Err(CaseFailure {
            diag_id: Some("E-SEM-2741"),
            diag_detail: format!(
                "if-case is not exhaustive for scrutinee type {} and has no else branch",
                type_to_string(&scrutinee.match_type)
            ),
            at_scrutinee: true,
        });
    }
    Ok(())
}

fn case_scope_detail(expr: &ast::IfCaseExpr, scrutinee: &Scrutinee, env: &TypeEnv) -> String {
    format!(
        "if-case pattern failed case-scope binding against scrutinee type {} from scrutinee expression kind {}{}",
        type_to_string(&scrutinee.match_type),
        expr.scrutinee.as_deref().map_or("", ast::expr_kind),
        scrutinee_binding_type_detail(&expr.scrutinee, env)
    )
}

pub fn type_if_case_expr(ctx: &ScopeContext<'_>, type_ctx: &StmtTypeContext<'_>, expr: &ast::IfCaseExpr, env: &TypeEnv) -> ExprTypeResult {
    let Some(scrutinee_expr) = expr.scrutinee.as_deref() else {
        return ExprTypeResult::default();
    };
    let scrutinee = match type_scrutinee(ctx, type_ctx, &expr.scrutinee, env) {
        Ok(scrutinee) => scrutinee,
        Err(failure) => return ExprTypeResult::failed(failure.diag_id),
    };
    if scrutinee.base.is_none() {
        return ExprTypeResult { diag_detail: "if-case scrutinee type could not be canonicalized".to_string(), ..Default::default() };
    }
    let mut arm_types: Vec<TypeRef> = Vec::new();
    for arm in &expr.cases {
        if arm.pattern.is_none() || arm.body.is_none() {
            return ExprTypeResult::default();
        }
        let arm_env = match case_scope_env(ctx, &expr.scrutinee, env, &arm.pattern, &scrutinee.match_type) {
            Ok(arm_env) => arm_env,
            Err(diag_id) => {
                return ExprTypeResult {
                    diag_id,
                    diag_detail: case_scope_detail(expr, &scrutinee, env),
                    diag_span: arm.pattern.as_deref().map(|pattern| pattern.span.clone()),
                    ..Default::default()
                }
            }
        };
        let body = type_arm_body(ctx, type_ctx, &arm.body, &arm_env);
        if !body.ok {
            return ExprTypeResult::failed(body.diag_id);
        }
        arm_types.push(body.r#type);
    }
    if expr.else_expr.is_some() {
        let else_env = match else_scope_for_cases(ctx, &expr.scrutinee, env, &expr.cases, &scrutinee.base) {
            Ok(else_env) => else_env,
            Err(diag_id) => {
                return ExprTypeResult {
                    diag_id,
                    diag_detail: "if-case else scope failed after case pattern refinement".to_string(),
                    diag_span: Some(scrutinee_expr.span.clone()),
                    ..Default::default()
                }
            }
        };
        let else_typed = type_expr(ctx, type_ctx, &expr.else_expr, &else_env);
        if !else_typed.ok {
            return ExprTypeResult::failed(else_typed.diag_id);
        }
        arm_types.push(else_typed.r#type);
    }
    let Some(first) = arm_types.first() else {
        return ExprTypeResult::failed(Some("If-Branch-Mismatch"));
    };
    if !arm_types[1..].iter().all(|ty| type_equiv(first, ty)) {
        return ExprTypeResult::failed(Some("If-Branch-Mismatch"));
    }
    if let Err(failure) = check_arms_cover(ctx, expr, &scrutinee) {
        return ExprTypeResult {
            diag_id: failure.diag_id,
            diag_detail: failure.diag_detail,
            diag_span: failure.at_scrutinee.then(|| scrutinee_expr.span.clone()),
            ..Default::default()
        };
    }
    ExprTypeResult::typed(first.clone())
}

pub fn check_if_case_expr(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    expr: &ast::IfCaseExpr,
    env: &TypeEnv,
    expected: &TypeRef,
) -> CheckResult {
    let detail = |diag_detail: &str| CheckResult { diag_detail: diag_detail.to_string(), ..Default::default() };
    let Some(scrutinee_expr) = expr.scrutinee.as_deref() else {
        return detail("if-case expression is missing a scrutinee");
    };
    if expected.is_none() {
        return detail("if-case expression is missing an expected checking type");
    }
    let scrutinee = match type_scrutinee(ctx, type_ctx, &expr.scrutinee, env) {
        Ok(scrutinee) => scrutinee,
        // A failure to type the scrutinee carries its detail; one of its aliases does not.
        Err(failure) => {
            return CheckResult { diag_id: failure.diag_id, diag_detail: failure.diag_detail, diag_span: failure.diag_span, ..Default::default() }
        }
    };
    if scrutinee.base.is_none() {
        return CheckResult {
            diag_detail: "if-case scrutinee has no canonical pattern type after alias normalization".to_string(),
            diag_span: Some(scrutinee_expr.span.clone()),
            ..Default::default()
        };
    }
    for arm in &expr.cases {
        let Some(pattern) = arm.pattern.as_deref() else {
            return detail("if-case clause is missing a pattern");
        };
        if arm.body.is_none() {
            return detail("if-case clause is missing a body");
        }
        let arm_env = match case_scope_env(ctx, &expr.scrutinee, env, &arm.pattern, &scrutinee.match_type) {
            Ok(arm_env) => arm_env,
            Err(diag_id) => {
                return CheckResult {
                    diag_id,
                    diag_detail: case_scope_detail(expr, &scrutinee, env),
                    diag_span: Some(pattern.span.clone()),
                    ..Default::default()
                }
            }
        };
        let check = check_arm_body(ctx, type_ctx, &arm.body, &arm_env, expected);
        if !check.ok {
            let mut diag_detail = format!("if-case arm body failed checking against expected {}", type_to_string(expected));
            if !check.diag_detail.is_empty() {
                diag_detail = format!("{diag_detail}: {}", check.diag_detail);
            }
            return CheckResult { diag_detail, ..check };
        }
    }
    if expr.else_expr.is_some() {
        let else_env = match else_scope_for_cases(ctx, &expr.scrutinee, env, &expr.cases, &scrutinee.base) {
            Ok(else_env) => else_env,
            Err(diag_id) => {
                return CheckResult {
                    diag_id,
                    diag_detail: "if-case else scope failed after case pattern refinement".to_string(),
                    diag_span: Some(scrutinee_expr.span.clone()),
                    ..Default::default()
                }
            }
        };
        let else_check = check_expr_against(ctx, type_ctx, &expr.else_expr, expected, &else_env);
        if !else_check.ok {
            return CheckResult {
                ok: false,
                diag_id: else_check.diag_id,
                diag_detail: if else_check.diag_detail.is_empty() {
                    "if-case else branch failed checking".to_string()
                } else {
                    else_check.diag_detail
                },
                diag_span: else_check.diag_span,
                ..Default::default()
            };
        }
    }
    if let Err(failure) = check_arms_cover(ctx, expr, &scrutinee) {
        return CheckResult {
            diag_id: failure.diag_id,
            diag_detail: failure.diag_detail,
            diag_span: failure.at_scrutinee.then(|| scrutinee_expr.span.clone()),
            ..Default::default()
        };
    }
    CheckResult { ok: true, ..Default::default() }
}
