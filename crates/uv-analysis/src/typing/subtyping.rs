//! Subtyping.
//!
//! Aliases are expanded first. Then: equivalent types, `!` below everything, refinements
//! by proof, nominal types by the variance of their parameters, asynchronous types by
//! their signatures, class implementation against `$Class`, stated text below unstated
//! text, and the structural rules for the remaining forms.

use uv_source::ast;

use super::type_equiv::type_equiv;
use super::type_lookup::async_sig_of;
use super::type_lower::lower_type;
use super::types::*;
use super::variance::{combine_variance, variance_of, Variance};
use crate::composite::classes::type_implements_class;
use crate::context::*;
use crate::contracts::verification::{add_fact, static_proof, StaticProofContext};
use crate::generics::monomorphize::{build_substitution, instantiate_type};
use crate::modal::modal_widen::niche_compatible;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};
use crate::resolve::scopes_lookup::resolve_type_name;

/// `ok` is false when a type involved could not be lowered. A relation that does not
/// hold may still name the rule that says why.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubtypingResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub subtype: bool,
}

const YES: SubtypingResult = SubtypingResult { ok: true, diag_id: None, subtype: true };
const NO: SubtypingResult = SubtypingResult { ok: true, diag_id: None, subtype: false };

fn no_because(diag_id: &'static str) -> SubtypingResult {
    SubtypingResult { ok: true, diag_id: Some(diag_id), subtype: false }
}

fn failed(diag_id: Option<&'static str>) -> SubtypingResult {
    SubtypingResult { ok: false, diag_id, subtype: false }
}

fn holds(subtype: bool) -> SubtypingResult {
    if subtype {
        YES
    } else {
        NO
    }
}

/// All of the given relations hold; the first that does not (or fails) decides, losing
/// its rule unless it failed.
fn all_hold(mut relations: impl Iterator<Item = SubtypingResult>) -> SubtypingResult {
    match relations.find(|res| !res.ok || !res.subtype) {
        None => YES,
        Some(res) if res.ok => NO,
        Some(res) => res,
    }
}

fn type_path_eq(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| id_key_of(a) == id_key_of(b))
}

fn is_never_type(ty: &Type) -> bool {
    matches!(&ty.node, TypeNode::Prim(name) if name == "!")
}

fn is_int_name(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "isize" | "usize")
}

/// Two different integer types, or two different floating-point types.
fn numeric_mismatch(lhs: &Type, rhs: &Type) -> bool {
    let (TypeNode::Prim(l), TypeNode::Prim(r)) = (&lhs.node, &rhs.node) else {
        return false;
    };
    let is_float = |name: &str| matches!(name, "f16" | "f32" | "f64");
    l != r && ((is_int_name(l) && is_int_name(r)) || (is_float(l) && is_float(r)))
}

/// By path, or for a bare name through the scopes.
fn lookup_type_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c TypeDecl> {
    if let Some(decl) = ctx.sigma.types.get(&path_key_of(path)) {
        return Some(decl);
    }
    let [name] = path else {
        return None;
    };
    let entity = resolve_type_name(ctx, name)?;
    let mut resolved = entity.origin_opt?;
    resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
    ctx.sigma.types.get(&path_key_of(&resolved))
}

/// A generic declaration's parameters and the lowered types of its members, from which
/// the variance of each parameter follows.
fn generic_variance_info(
    ctx: &ScopeContext<'_>,
    decl: &TypeDecl,
) -> Result<(Vec<ast::TypeParam>, Vec<TypeRef>), Option<&'static str>> {
    let (generic_params, written): (_, Vec<&ast::TypePtr>) = match decl {
        TypeDecl::TypeAlias(node) => (&node.generic_params, vec![&node.r#type]),
        TypeDecl::Record(node) => (
            &node.generic_params,
            node.members
                .iter()
                .filter_map(|member| match member {
                    ast::RecordMember::FieldDecl(field) => Some(&field.r#type),
                    _ => None,
                })
                .collect(),
        ),
        TypeDecl::Enum(node) => (
            &node.generic_params,
            node.variants
                .iter()
                .flat_map(|variant| match &variant.payload_opt {
                    None => Vec::new(),
                    Some(ast::VariantPayload::VariantPayloadTuple(tuple)) => tuple.elements.iter().collect(),
                    Some(ast::VariantPayload::VariantPayloadRecord(record)) => {
                        record.fields.iter().map(|field| &field.r#type).collect()
                    }
                })
                .collect(),
        ),
        TypeDecl::Modal(node) => (
            &node.generic_params,
            node.states
                .iter()
                .flat_map(|state| &state.members)
                .filter_map(|member| match member {
                    ast::StateMember::StateFieldDecl(field) => Some(&field.r#type),
                    _ => None,
                })
                .collect(),
        ),
    };
    let Some(params) = generic_params else {
        return Ok((Vec::new(), Vec::new()));
    };
    let mut member_types = Vec::new();
    for ty in written.into_iter().filter(|ty| ty.is_some()) {
        member_types.push(lower_type(ctx, ty)?);
    }
    Ok((params.params.clone(), member_types))
}

/// The alias's definition for the given arguments, when the path names an alias that
/// takes them.
fn expand_type_alias_apply(ctx: &ScopeContext<'_>, path: &[String], args: &[TypeRef]) -> Result<TypeRef, Option<&'static str>> {
    let Some(TypeDecl::TypeAlias(alias)) = lookup_type_decl(ctx, path) else {
        return Ok(None);
    };
    let lowered = lower_type(ctx, &alias.r#type)?;
    Ok(match &alias.generic_params {
        None if args.is_empty() => lowered,
        None => None,
        Some(params) if args.len() > params.params.len() => None,
        Some(params) => instantiate_type(&lowered, &build_substitution(&params.params, args)),
    })
}

type Normalized = Result<(TypeRef, bool), Option<&'static str>>;

/// Expands aliases at the top of the type, a bounded number of times.
fn normalize_alias_top_level(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Normalized {
    let mut current = ty.clone();
    let mut expanded = false;
    for _ in 0..16 {
        let Some((path, args)) = current.as_deref().and_then(|ty| applied_type_path(ty).zip(applied_type_args(ty))) else {
            break;
        };
        let target = expand_type_alias_apply(ctx, path, args)?;
        if target.is_none() {
            break;
        }
        current = target;
        expanded = true;
    }
    Ok((current, expanded))
}

/// Expands aliases everywhere in the type. The flag says whether anything changed; an
/// unchanged type is returned as it was.
fn normalize_alias_deep(ctx: &ScopeContext<'_>, ty: &TypeRef, depth: usize) -> Normalized {
    if depth > 64 {
        return Ok((ty.clone(), false));
    }
    let (cur, mut changed) = normalize_alias_top_level(ctx, ty)?;
    let Some(node) = cur.as_deref().map(|ty| &ty.node) else {
        return Ok((cur, changed));
    };
    let mut deep = |ty: &TypeRef| -> Result<TypeRef, Option<&'static str>> {
        let (normalized, expanded) = normalize_alias_deep(ctx, ty, depth + 1)?;
        changed |= expanded;
        Ok(normalized)
    };
    let list = |types: &[TypeRef], deep: &mut dyn FnMut(&TypeRef) -> Result<TypeRef, Option<&'static str>>| {
        types.iter().map(deep).collect::<Result<Vec<_>, _>>()
    };
    let rebuilt = match node {
        TypeNode::Perm { perm, base } => make_type_perm(*perm, deep(base)?),
        TypeNode::Union(members) => make_type_union(list(members, &mut deep)?),
        TypeNode::Func { params, ret } => {
            let mut new_params = Vec::with_capacity(params.len());
            for param in params {
                new_params.push(TypeFuncParam { mode: param.mode, r#type: deep(&param.r#type)? });
            }
            make_type_func(new_params, deep(ret)?)
        }
        TypeNode::Tuple(elements) => make_type_tuple(list(elements, &mut deep)?),
        // A rebuilt array does not keep how its length was written.
        TypeNode::Array { element, length, .. } => make_type_array(deep(element)?, *length, None),
        TypeNode::Slice(element) => make_type_slice(deep(element)?),
        TypeNode::Ptr { element, state } => make_type_ptr(deep(element)?, *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, deep(element)?),
        TypeNode::ModalState(modal) => {
            make_type_modal_state(modal.path.clone(), &modal.state, list(&modal.generic_args, &mut deep)?)
        }
        TypeNode::Path { path, generic_args } => make_type_path_with(path.clone(), list(generic_args, &mut deep)?),
        TypeNode::Apply { path, args } => make_type_apply(path.clone(), list(args, &mut deep)?),
        TypeNode::Refine { base, predicate } => make_type_refine(deep(base)?, predicate.clone()),
        TypeNode::Closure { params, ret, deps_opt } => {
            let mut new_params = Vec::with_capacity(params.len());
            for (is_move, param) in params {
                new_params.push((*is_move, deep(param)?));
            }
            let new_ret = deep(ret)?;
            let new_deps = match deps_opt {
                Some(deps) => {
                    let mut out = Vec::with_capacity(deps.len());
                    for dep in deps {
                        out.push(SharedDep { name: dep.name.clone(), r#type: deep(&dep.r#type)? });
                    }
                    Some(out)
                }
                None => None,
            };
            make_type_closure(new_params, new_ret, new_deps)
        }
        _ => return Ok((cur, changed)),
    };
    Ok(if changed { (rebuilt, true) } else { (cur, false) })
}

/// Whether the type is one of the union's members: equivalent to it, or a subtype in
/// both directions.
fn member(ctx: &ScopeContext<'_>, ty: &TypeRef, members: &[TypeRef]) -> SubtypingResult {
    for candidate in members {
        if type_equiv(ty, candidate) {
            return YES;
        }
        let forward = subtyping(ctx, ty, candidate);
        if !forward.ok {
            return failed(forward.diag_id);
        }
        if !forward.subtype {
            continue;
        }
        let reverse = subtyping(ctx, candidate, ty);
        if !reverse.ok {
            return failed(reverse.diag_id);
        }
        if reverse.subtype {
            return YES;
        }
    }
    NO
}

/// Arguments of one nominal type, related according to the variance of each parameter.
fn generic_args_related(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
    member_types: &[TypeRef],
    lhs_args: &[TypeRef],
    rhs_args: &[TypeRef],
) -> SubtypingResult {
    for ((param, lhs_arg), rhs_arg) in params.iter().zip(lhs_args).zip(rhs_args) {
        // A declaration without members says nothing about its parameters.
        let variance = if member_types.is_empty() {
            Variance::Invariant
        } else {
            member_types.iter().map(|ty| variance_of(ty, &param.name)).fold(Variance::Bivariant, combine_variance)
        };
        let related = match variance {
            Variance::Covariant => subtyping(ctx, lhs_arg, rhs_arg),
            Variance::Contravariant => subtyping(ctx, rhs_arg, lhs_arg),
            Variance::Invariant if type_equiv(lhs_arg, rhs_arg) => YES,
            Variance::Invariant => return no_because("E-TYP-1520"),
            Variance::Bivariant => YES,
        };
        if !related.ok {
            return failed(related.diag_id);
        }
        if !related.subtype {
            return no_because("E-TYP-1521");
        }
    }
    YES
}

fn text_widens<S: PartialEq + Copy>(lhs: Option<S>, rhs: Option<S>) -> bool {
    lhs.is_some() && rhs.is_none()
}

thread_local! {
    static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The reference recurses without bound on an alias that expands to itself and crashes;
/// here such a question is answered "no".
const MAX_DEPTH: usize = 256;

pub fn subtyping(ctx: &ScopeContext<'_>, lhs: &TypeRef, rhs: &TypeRef) -> SubtypingResult {
    let depth = DEPTH.get();
    if depth >= MAX_DEPTH {
        return NO;
    }
    DEPTH.set(depth + 1);
    let result = subtyping_at(ctx, lhs, rhs);
    DEPTH.set(depth);
    result
}

fn subtyping_at(ctx: &ScopeContext<'_>, lhs_in: &TypeRef, rhs_in: &TypeRef) -> SubtypingResult {
    let (lhs_ref, rhs_ref) = match (normalize_alias_deep(ctx, lhs_in, 0), normalize_alias_deep(ctx, rhs_in, 0)) {
        (Err(diag_id), _) => return failed(diag_id),
        (_, Err(diag_id)) => return failed(diag_id),
        (Ok((lhs, _)), Ok((rhs, _))) => (lhs, rhs),
    };
    let (Some(lhs), Some(rhs)) = (lhs_ref.as_deref(), rhs_ref.as_deref()) else {
        return NO;
    };
    if type_equiv(&lhs_ref, &rhs_ref) || is_never_type(lhs) {
        return YES;
    }
    use TypeNode as N;
    // Refinements: the left predicate must prove the right one.
    match (&lhs.node, &rhs.node) {
        (N::Refine { base: lbase, predicate: lpred }, N::Refine { base: rbase, predicate: rpred }) => {
            let (true, Some(_), Some(goal)) = (type_equiv(lbase, rbase), lpred, rpred) else {
                return NO;
            };
            let mut proof_ctx = StaticProofContext::default();
            add_fact(&mut proof_ctx, lpred, &goal.span);
            return if static_proof(&proof_ctx, rpred).provable { YES } else { no_because("E-TYP-1953") };
        }
        (N::Refine { base, .. }, _) => return subtyping(ctx, base, &rhs_ref),
        (_, N::Refine { .. }) => return NO,
        (N::Opaque { class_path: l, .. }, N::Opaque { class_path: r, .. }) => {
            return if type_path_eq(l, r) { YES } else { no_because("E-TYP-2512") };
        }
        (N::Opaque { .. }, _) | (_, N::Opaque { .. }) => return NO,
        _ => {}
    }
    if numeric_mismatch(lhs, rhs) {
        return NO;
    }
    let lhs_applied = applied_type_path(lhs).zip(applied_type_args(lhs));
    let rhs_applied = applied_type_path(rhs).zip(applied_type_args(rhs));
    if let (Some((lpath, largs)), Some((rpath, rargs))) = (lhs_applied, rhs_applied) {
        if type_path_eq(lpath, rpath) && largs.len() == rargs.len() {
            if let Some(decl) = lookup_type_decl(ctx, lpath) {
                let (params, member_types) = match generic_variance_info(ctx, decl) {
                    Ok(info) => info,
                    Err(diag_id) => return failed(diag_id),
                };
                if params.len() == largs.len() {
                    return generic_args_related(ctx, &params, &member_types, largs, rargs);
                }
            }
        }
    }
    if let Some((path, args)) = lhs_applied {
        match expand_type_alias_apply(ctx, path, args) {
            Err(diag_id) => return failed(diag_id),
            Ok(expanded @ Some(_)) => return subtyping(ctx, &expanded, &rhs_ref),
            Ok(None) => {}
        }
    }
    if let Some((path, args)) = rhs_applied {
        match expand_type_alias_apply(ctx, path, args) {
            Err(diag_id) => return failed(diag_id),
            Ok(expanded @ Some(_)) => return subtyping(ctx, &lhs_ref, &expanded),
            Ok(None) => {}
        }
    }
    // Asynchronous types: covariant in what they yield, complete with and fail with,
    // contravariant in what they take.
    if let (Some(l), Some(r)) = (async_sig_of(ctx, &lhs_ref), async_sig_of(ctx, &rhs_ref)) {
        let parts = [(&l.out, &r.out), (&r.input, &l.input), (&l.result, &r.result), (&l.err, &r.err)];
        for (sub, sup) in parts {
            let res = subtyping(ctx, sub, sup);
            if !res.ok {
                return failed(res.diag_id);
            }
            if !res.subtype {
                return NO;
            }
        }
        return YES;
    }
    if let N::Dynamic(class_path) = &rhs.node {
        if type_implements_class(ctx, &lhs_ref, class_path) {
            return YES;
        }
    }
    let stated_text_widens = match (&lhs.node, &rhs.node) {
        (N::String(l), N::String(r)) => text_widens(*l, *r),
        (N::Bytes(l), N::Bytes(r)) => text_widens(*l, *r),
        _ => false,
    };
    if stated_text_widens {
        return YES;
    }
    // Permissions must agree; under `const` the arguments of one nominal type may vary.
    let split = |ty: &'_ Type, whole: &TypeRef| match &ty.node {
        N::Perm { perm, base } => (Some(*perm), base.clone()),
        _ => (None, whole.clone()),
    };
    let (lperm, lbase) = split(lhs, &lhs_ref);
    let (rperm, rbase) = split(rhs, &rhs_ref);
    if lperm.is_some() || rperm.is_some() {
        let (lperm, rperm) = (lperm.unwrap_or(Permission::Const), rperm.unwrap_or(Permission::Const));
        if lperm != rperm {
            return NO;
        }
        if lperm == Permission::Const {
            let applied = |ty: &TypeRef| {
                ty.as_deref().and_then(|ty| applied_type_path(ty).zip(applied_type_args(ty))).map(|(p, a)| (p.clone(), a.to_vec()))
            };
            if let (Some((lpath, largs)), Some((rpath, rargs))) = (applied(&lbase), applied(&rbase)) {
                if type_path_eq(&lpath, &rpath) && largs.len() == rargs.len() {
                    for (larg, rarg) in largs.iter().zip(&rargs) {
                        let sub = subtyping(ctx, larg, rarg);
                        if !sub.ok {
                            return failed(sub.diag_id);
                        }
                        if !sub.subtype {
                            return NO;
                        }
                    }
                    return YES;
                }
            }
        }
        return subtyping(ctx, &lbase, &rbase);
    }
    match (&lhs.node, &rhs.node) {
        (N::Tuple(l), N::Tuple(r)) => {
            if l.len() != r.len() {
                return NO;
            }
            return all_hold(l.iter().zip(r).map(|(a, b)| subtyping(ctx, a, b)));
        }
        (N::Array { element: l, .. }, N::Slice(r)) | (N::Slice(l), N::Slice(r)) => return subtyping(ctx, l, r),
        (N::Array { element: l, length: m, .. }, N::Array { element: r, length: n, .. }) => {
            return if m == n { subtyping(ctx, l, r) } else { NO };
        }
        (N::Range(l), N::Range(r))
        | (N::RangeInclusive(l), N::RangeInclusive(r))
        | (N::RangeFrom(l), N::RangeFrom(r))
        | (N::RangeTo(l), N::RangeTo(r))
        | (N::RangeToInclusive(l), N::RangeToInclusive(r)) => return subtyping(ctx, l, r),
        (N::Range(_) | N::RangeInclusive(_) | N::RangeFrom(_) | N::RangeTo(_) | N::RangeToInclusive(_), _) => return NO,
        (N::RangeFull, N::RangeFull) => return YES,
        // A pointer known to be valid or null is a pointer of unknown state.
        (N::Ptr { element: l, state: lstate }, N::Ptr { element: r, state: rstate }) => {
            let widens = rstate.is_none() && matches!(lstate, Some(PtrState::Valid | PtrState::Null));
            return holds(widens && type_equiv(l, r));
        }
        (N::ModalState(l), N::ModalState(r)) if type_path_eq(&l.path, &r.path) => {
            if !id_eq(&l.state, &r.state) {
                return NO;
            }
            if l.state == r.state {
                let same_args = l.generic_args.len() == r.generic_args.len()
                    && l.generic_args.iter().zip(&r.generic_args).all(|(a, b)| type_equiv(a, b));
                return holds(same_args);
            }
        }
        _ => {}
    }
    // A state is a subtype of its modal when it has the modal's representation.
    if let (N::ModalState(l), Some((rpath, rargs))) = (&lhs.node, rhs_applied) {
        let same = type_path_eq(&l.path, rpath)
            && l.generic_args.len() == rargs.len()
            && l.generic_args.iter().zip(rargs).all(|(a, b)| type_equiv(a, b));
        return holds(same && niche_compatible(ctx, &l.path, &l.state));
    }
    match (&lhs.node, &rhs.node) {
        (N::Func { params: lp, ret: lret }, N::Func { params: rp, ret: rret }) => {
            if lp.len() != rp.len() {
                return NO;
            }
            for (a, b) in lp.iter().zip(rp) {
                if a.mode != b.mode {
                    return NO;
                }
                let res = subtyping(ctx, &b.r#type, &a.r#type);
                if !res.ok || !res.subtype {
                    return if res.ok { NO } else { res };
                }
            }
            subtyping(ctx, lret, rret)
        }
        (N::Closure { params: lp, ret: lret, deps_opt: ld }, N::Closure { params: rp, ret: rret, deps_opt: rd }) => {
            if lp.len() != rp.len() {
                return NO;
            }
            for (a, b) in lp.iter().zip(rp) {
                if a.0 != b.0 {
                    return NO;
                }
                let res = subtyping(ctx, &b.1, &a.1);
                if !res.ok || !res.subtype {
                    return if res.ok { NO } else { res };
                }
            }
            let deps_agree = match (ld, rd) {
                (None, None) => true,
                (Some(l), Some(r)) => {
                    l.len() == r.len() && l.iter().zip(r).all(|(a, b)| a.name == b.name && type_equiv(&a.r#type, &b.r#type))
                }
                _ => false,
            };
            if !deps_agree {
                return NO;
            }
            subtyping(ctx, lret, rret)
        }
        (N::Union(l), N::Union(r)) => all_hold(l.iter().map(|ty| member(ctx, ty, r))),
        (_, N::Union(r)) => member(ctx, &lhs_ref, r),
        _ => NO,
    }
}

/// `unique` may be used where `shared` or `const` is asked for, `shared` where `const` is.
pub fn permission_admits(caller: Permission, required: Permission) -> bool {
    match required {
        Permission::Const => true,
        Permission::Shared => caller != Permission::Const,
        Permission::Unique => caller == Permission::Unique,
    }
}

/// Whether an argument may be passed for a parameter. Beyond subtyping, an argument
/// passed by reference may have a stronger permission than the parameter asks for.
pub fn argument_type_compatible(
    ctx: &ScopeContext<'_>,
    actual: &TypeRef,
    expected: &TypeRef,
    mode: Option<ParamMode>,
) -> SubtypingResult {
    let sub = subtyping(ctx, actual, expected);
    if !sub.ok || sub.subtype || mode.is_some() || actual.is_none() || expected.is_none() {
        return sub;
    }
    let split = |ty: &TypeRef| match ty.as_deref().map(|ty| &ty.node) {
        Some(N::Perm { perm, base }) => (*perm, base.clone()),
        _ => (Permission::Const, ty.clone()),
    };
    use TypeNode as N;
    let (actual_perm, actual_base) = split(actual);
    let (expected_perm, expected_base) = split(expected);
    let strip = super::type_predicates::strip_perm_and_refine;
    let (actual_base, expected_base) = (strip(&actual_base), strip(&expected_base));
    if !permission_admits(actual_perm, expected_perm) || actual_base.is_none() || expected_base.is_none() {
        return sub;
    }
    let base = subtyping(ctx, &actual_base, &expected_base);
    if base.ok && base.subtype {
        YES
    } else {
        sub
    }
}
