//! Predicates on types: which foundational classes a type satisfies intrinsically
//! (`Bitcopy`, `Clone`, `Drop`, `Eq`, `Discrete`), whether it may cross the FFI or go to
//! the GPU, whether all-zero bytes are a value of it, and which casts exist.

use std::collections::BTreeSet;

use uv_source::ast;

use super::signature::build_method_signature;
use super::type_equiv::type_equiv;
use super::type_lookup::{lookup_enum_decl, lookup_record_decl};
use super::type_lower::lower_type;
use super::types::*;
use crate::caps::builtin_paths::{is_capability_class_path, is_context_type_path, is_execution_domain_class_path};
use crate::composite::classes::class_subtypes;
use crate::composite::enums::enum_discriminants;
use crate::context::*;
use crate::generics::monomorphize::apply_generic_substitution;
use crate::modal::lookup::lookup_modal_decl;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};

type Active = BTreeSet<PathKey>;

/// The type under any permissions.
pub fn strip_perm(ty: &TypeRef) -> TypeRef {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    current.clone()
}

/// The type under any permissions and refinements.
pub fn strip_perm_and_refine(ty: &TypeRef) -> TypeRef {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    current.clone()
}

/// The outermost permission; `const` when none is written.
pub fn perm_of_type(ty: &TypeRef) -> Permission {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { perm, .. }) => *perm,
        _ => Permission::Const,
    }
}

/// A dynamic capability class, or `Context`.
pub fn is_capability_type(ty: &TypeRef) -> bool {
    match strip_perm(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Dynamic(path)) => is_capability_class_path(path) || is_execution_domain_class_path(path),
        Some(TypeNode::Path { path, .. }) => is_context_type_path(path),
        _ => false,
    }
}

fn prim_name(ty: &TypeRef) -> Option<&str> {
    match &ty.as_deref()?.node {
        TypeNode::Prim(name) => Some(name),
        _ => None,
    }
}

fn is_int_prim(name: &str) -> bool {
    matches!(name, "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize")
}

fn is_float_prim(name: &str) -> bool {
    matches!(name, "f16" | "f32" | "f64")
}

/// An enum all of whose variants are without payload.
pub fn unit_enum_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    let base = strip_perm_and_refine(ty);
    base.as_deref()
        .and_then(applied_type_path)
        .and_then(|path| lookup_enum_decl(ctx, path))
        .is_some_and(|decl| !decl.variants.is_empty() && decl.variants.iter().all(|variant| variant.payload_opt.is_none()))
}

/// `#layout(C)`, with `C` first.
fn has_layout_c(attr_list: &[ast::AttributeItem]) -> bool {
    attr_list.iter().any(|attr| {
        attr.name.full_name == "layout"
            && matches!(attr.args.first(), Some(ast::AttributeArg { value: ast::AttributeArgValue::Token(token), .. })
                if token.lexeme == "C")
    })
}

fn ast_type_mentions_param(type_ptr: &ast::TypePtr, param_name: &str) -> bool {
    let Some(ty) = type_ptr.as_deref() else {
        return false;
    };
    let mentions = |ty: &ast::TypePtr| ast_type_mentions_param(ty, param_name);
    let names_param = |path: &[String]| matches!(path, [only] if id_eq(only, param_name));
    use ast::TypeNode as N;
    match &ty.node {
        N::TypePathType(node) => names_param(&node.path) || node.generic_args.iter().any(mentions),
        N::TypeApply(node) => names_param(&node.path) || node.args.iter().any(mentions),
        N::TypePermType(node) => mentions(&node.base),
        N::TypeTuple(node) => node.elements.iter().any(mentions),
        N::TypeArray(node) => mentions(&node.element),
        N::TypeSlice(node) => mentions(&node.element),
        N::TypeSafePtr(node) => mentions(&node.element),
        N::TypeRawPtr(node) => mentions(&node.element),
        N::TypeUnion(node) => node.types.iter().any(mentions),
        N::TypeFunc(node) => node.params.iter().any(|param| mentions(&param.r#type)) || mentions(&node.ret),
        N::TypeClosure(node) => {
            node.params.iter().any(|param| mentions(&param.r#type))
                || mentions(&node.ret)
                || node.deps_opt.iter().flatten().any(|dep| mentions(&dep.r#type))
        }
        N::TypeModalState(node) => node.generic_args.iter().any(mentions),
        N::TypeRefine(node) => mentions(&node.base),
        N::TypeRange(node) => mentions(&node.base),
        N::TypeRangeInclusive(node) => mentions(&node.base),
        N::TypeRangeFrom(node) => mentions(&node.base),
        N::TypeRangeTo(node) => mentions(&node.base),
        N::TypeRangeToInclusive(node) => mentions(&node.base),
        _ => false,
    }
}

fn bound_names_class(ctx: &ScopeContext<'_>, bound: &ast::TypeBound, class_name: &str) -> bool {
    matches!(&bound.class_path[..], [only] if id_eq(only, class_name))
        || class_subtypes(ctx, &bound.class_path, &[class_name.to_string()])
}

/// The written types of a declaration's members: record fields, or enum payloads.
fn record_field_types(decl: &ast::RecordDecl) -> Vec<&ast::TypePtr> {
    decl.members
        .iter()
        .filter_map(|member| match member {
            ast::RecordMember::FieldDecl(field) if field.r#type.is_some() => Some(&field.r#type),
            _ => None,
        })
        .collect()
}

fn enum_payload_types(decl: &ast::EnumDecl) -> Vec<&ast::TypePtr> {
    decl.variants.iter().flat_map(variant_payload_types).collect()
}

fn variant_payload_types(variant: &ast::VariantDecl) -> Vec<&ast::TypePtr> {
    match &variant.payload_opt {
        None => Vec::new(),
        Some(ast::VariantPayload::VariantPayloadTuple(tuple)) => tuple.elements.iter().collect(),
        Some(ast::VariantPayload::VariantPayloadRecord(record)) => record.fields.iter().map(|field| &field.r#type).collect(),
    }
}

/// Whether a type parameter that the members mention lacks a bound on the class.
fn generic_params_missing_reqs(
    ctx: &ScopeContext<'_>,
    generic_params: &Option<ast::GenericParams>,
    member_types: &[&ast::TypePtr],
    class_name: &str,
) -> bool {
    let Some(params) = generic_params else {
        return false;
    };
    let has_bound = |name: &str| {
        params
            .params
            .iter()
            .find(|param| id_eq(&param.name, name))
            .is_some_and(|param| param.bounds.iter().any(|bound| bound_names_class(ctx, bound, class_name)))
    };
    params
        .params
        .iter()
        .filter(|param| member_types.iter().any(|ty| ast_type_mentions_param(ty, &param.name)))
        .any(|param| !has_bound(&param.name))
}

/// A nominal declaration for the FFI check: by exact path, relative to the given module
/// or its assembly, or else the only declaration whose path ends in the given one.
fn resolve_nominal_type_decl_for_ffi<'c>(
    ctx: &'c ScopeContext<'_>,
    current_module: Option<&[String]>,
    path: &[String],
) -> Option<(&'c TypeDecl, Vec<String>)> {
    if path.is_empty() {
        return None;
    }
    let at = |candidate: Vec<String>| ctx.sigma.types.get(&path_key_of(&candidate)).map(|decl| (decl, candidate));
    if let Some(found) = at(path.to_vec()) {
        return Some(found);
    }
    if let Some(module) = current_module.filter(|module| !module.is_empty()) {
        if let Some(found) = at([module, path].concat()) {
            return Some(found);
        }
        if let [name] = path {
            if let Some(found) = at(vec![module[0].clone(), name.clone()]) {
                return Some(found);
            }
        }
    }
    let mut matches = ctx.sigma.types.iter().filter(|(key, _)| {
        key.len() >= path.len() && key[key.len() - path.len()..].iter().zip(path).all(|(a, b)| id_eq(a, b))
    });
    match (matches.next(), matches.next()) {
        (Some((key, decl)), None) => Some((decl, key.clone())),
        _ => None,
    }
}

fn is_builtin_bitcopy_path(path: &[String]) -> bool {
    const NAMES: [&str; 8] =
        ["FileKind", "IoError", "TimeError", "Duration", "MonotonicInstant", "UtcInstant", "Context", "System"];
    matches!(path, [only] if NAMES.iter().any(|name| id_eq(only, name)))
}

/// A type parameter in scope whose bounds name the class or a subclass of it.
fn type_param_has_class_bound(ctx: &ScopeContext<'_>, path: &[String], class_name: &str) -> bool {
    let [name] = path else {
        return false;
    };
    let key = id_key_of(name);
    ctx.scopes
        .iter()
        .filter_map(|scope| scope.get(&key))
        .find(|entity| {
            entity.kind == EntityKind::Type && entity.target_opt.as_ref().is_none_or(|target| id_eq(target, name))
        })
        .is_some_and(|entity| entity.type_param_class_bounds.iter().any(|bound| bound_names_class(ctx, bound, class_name)))
}

fn generic_param_names(generic_params: &Option<ast::GenericParams>) -> Vec<String> {
    generic_params.iter().flat_map(|params| &params.params).map(|param| param.name.clone()).collect()
}

/// The arguments of a declaration, with defaults for those not given; none when there
/// are too many or a parameter has neither argument nor default.
fn resolve_decl_generic_args(
    ctx: &ScopeContext<'_>,
    generic_params: &Option<ast::GenericParams>,
    provided: &[TypeRef],
) -> Option<Vec<TypeRef>> {
    let Some(params) = generic_params else {
        return provided.is_empty().then(Vec::new);
    };
    if provided.len() > params.params.len() {
        return None;
    }
    let mut out = provided.to_vec();
    for param in params.params.iter().skip(provided.len()) {
        param.default_type.as_ref()?;
        out.push(lower_type(ctx, &param.default_type).ok()?);
    }
    Some(out)
}

/// A member's written type, lowered and instantiated with the declaration's arguments.
fn member_type(
    ctx: &ScopeContext<'_>,
    written: &ast::TypePtr,
    generic_params: &Option<ast::GenericParams>,
    args: &[TypeRef],
) -> Option<TypeRef> {
    let lowered = lower_type(ctx, written).ok()?;
    let names = generic_param_names(generic_params);
    Some(if names.is_empty() { lowered } else { apply_generic_substitution(&lowered, &names, args) })
}

fn is_unit_type(ty: &TypeRef) -> bool {
    prim_name(&strip_perm_and_refine(ty)) == Some("()")
}

/// A record method `name(~perm) -> ret` without further parameters.
fn has_nullary_method(
    ctx: &ScopeContext<'_>,
    stripped: &TypeRef,
    name: &str,
    receiver_perm: Permission,
    returns: impl Fn(&TypeRef) -> bool,
) -> bool {
    let Some(record) = stripped.as_deref().and_then(applied_type_path).and_then(|path| lookup_record_decl(ctx, path))
    else {
        return false;
    };
    let expected_self = make_type_perm(receiver_perm, stripped.clone());
    record.members.iter().any(|member| {
        let ast::RecordMember::MethodDecl(method) = member else {
            return false;
        };
        if !id_eq(&method.name, name) || !method.params.is_empty() {
            return false;
        }
        let Ok(sig) = build_method_signature(ctx, stripped, &method.receiver, &method.params, &method.return_type_opt, None)
        else {
            return false;
        };
        match sig.func_type.as_deref().map(|ty| &ty.node) {
            Some(TypeNode::Func { params, .. }) => {
                matches!(&params[..], [recv] if recv.mode.is_none() && type_equiv(&recv.r#type, &expected_self))
                    && returns(&sig.return_type)
            }
            _ => false,
        }
    })
}

/// Whether every member type of the declaration is `Bitcopy`.
fn members_bitcopy(
    ctx: &ScopeContext<'_>,
    written: &[&ast::TypePtr],
    generic_params: &Option<ast::GenericParams>,
    args: &[TypeRef],
    active: &mut Active,
) -> bool {
    written
        .iter()
        .all(|ty| member_type(ctx, ty, generic_params, args).is_some_and(|ty| bitcopy_impl(ctx, &ty, active)))
}

fn state_field_types<'d>(decl: &'d ast::ModalDecl, state_filter: Option<&str>) -> Vec<&'d ast::TypePtr> {
    decl.states
        .iter()
        .filter(|state| state_filter.is_none_or(|filter| id_eq(&state.name, filter)))
        .flat_map(|state| &state.members)
        .filter_map(|member| match member {
            ast::StateMember::StateFieldDecl(field) if field.r#type.is_some() => Some(&field.r#type),
            _ => None,
        })
        .collect()
}

fn bitcopy_impl(ctx: &ScopeContext<'_>, type_ref: &TypeRef, active: &mut Active) -> bool {
    let Some(ty) = type_ref.as_deref() else {
        return false;
    };
    match &ty.node {
        TypeNode::Prim(_)
        | TypeNode::Ptr { .. }
        | TypeNode::RawPtr { .. }
        | TypeNode::Slice(_)
        | TypeNode::Func { .. }
        | TypeNode::Dynamic(_)
        | TypeNode::RangeFull => true,
        TypeNode::Perm { perm: Permission::Unique, .. } => false,
        TypeNode::Perm { base, .. }
        | TypeNode::Refine { base, .. }
        | TypeNode::Range(base)
        | TypeNode::RangeInclusive(base)
        | TypeNode::RangeFrom(base)
        | TypeNode::RangeTo(base)
        | TypeNode::RangeToInclusive(base) => bitcopy_impl(ctx, base, active),
        TypeNode::Tuple(types) | TypeNode::Union(types) => types.iter().all(|ty| bitcopy_impl(ctx, ty, active)),
        TypeNode::Array { element, .. } => bitcopy_impl(ctx, element, active),
        TypeNode::String(state) => *state == Some(StringState::View),
        TypeNode::Bytes(state) => *state == Some(BytesState::View),
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args } => {
            if is_builtin_bitcopy_path(path) {
                return true;
            }
            let is_bare_path = matches!(&ty.node, TypeNode::Path { .. }) && args.is_empty();
            if is_bare_path && type_param_has_class_bound(ctx, path, "Bitcopy") {
                return true;
            }
            let key = path_key_of(path);
            if active.contains(&key) {
                return false;
            }
            let Some(decl) = ctx.sigma.types.get(&key) else {
                return false;
            };
            active.insert(key.clone());
            let ok = match decl {
                TypeDecl::Record(decl) => resolve_decl_generic_args(ctx, &decl.generic_params, args).is_some_and(|args| {
                    members_bitcopy(ctx, &record_field_types(decl), &decl.generic_params, &args, active)
                }),
                TypeDecl::Enum(decl) => resolve_decl_generic_args(ctx, &decl.generic_params, args).is_some_and(|args| {
                    members_bitcopy(ctx, &enum_payload_types(decl), &decl.generic_params, &args, active)
                }),
                TypeDecl::Modal(decl) => resolve_decl_generic_args(ctx, &decl.generic_params, args).is_some_and(|args| {
                    members_bitcopy(ctx, &state_field_types(decl, None), &decl.generic_params, &args, active)
                }),
                TypeDecl::TypeAlias(decl) => {
                    decl.r#type.is_some()
                        && lower_type(ctx, &decl.r#type).is_ok()
                        && resolve_decl_generic_args(ctx, &decl.generic_params, args)
                            .and_then(|args| member_type(ctx, &decl.r#type, &decl.generic_params, &args))
                            .is_some_and(|target| bitcopy_impl(ctx, &target, active))
                }
            };
            active.remove(&key);
            ok
        }
        TypeNode::ModalState(node) => {
            let Some(decl) = lookup_modal_decl(ctx, &node.path) else {
                return false;
            };
            let key = path_key_of(&node.path);
            if active.contains(&key) {
                return false;
            }
            let Some(args) = resolve_decl_generic_args(ctx, &decl.generic_params, &node.generic_args) else {
                return false;
            };
            active.insert(key.clone());
            let fields = state_field_types(decl, Some(&node.state));
            let ok = members_bitcopy(ctx, &fields, &decl.generic_params, &args, active);
            active.remove(&key);
            ok
        }
        TypeNode::Var(_) | TypeNode::Closure { .. } | TypeNode::Opaque { .. } => false,
    }
}

/// Whether values of the type are copied bit for bit: no unique permission, no managed
/// text, and only such members.
pub fn bitcopy_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    bitcopy_impl(ctx, ty, &mut Active::new())
}

/// `Bitcopy`, or a record with `clone(~) -> Self`.
pub fn clone_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    if ty.is_none() {
        return false;
    }
    if bitcopy_type(ctx, ty) {
        return true;
    }
    let stripped = strip_perm_and_refine(ty);
    has_nullary_method(ctx, &stripped, "clone", Permission::Const, |ret| type_equiv(ret, &stripped))
}

/// Managed text, or a record with `drop(~!) -> ()`.
pub fn drop_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    let stripped = strip_perm_and_refine(ty);
    match stripped.as_deref().map(|ty| &ty.node) {
        None => false,
        Some(TypeNode::String(state)) => *state == Some(StringState::Managed),
        Some(TypeNode::Bytes(state)) => *state == Some(BytesState::Managed),
        Some(_) => has_nullary_method(ctx, &stripped, "drop", Permission::Unique, is_unit_type),
    }
}

fn is_ffi_safe_prim(name: &str) -> bool {
    is_int_prim(name) || is_float_prim(name) || name == "char" || name == "()"
}

/// The first member that is not FFI-safe decides: `E-TYP-2628` (incomplete) passes
/// through, anything else becomes `member_diag`.
fn ffi_members_diag(
    ctx: &ScopeContext<'_>,
    written: &[&ast::TypePtr],
    generic_params: &Option<ast::GenericParams>,
    args: &[TypeRef],
    decl_module: &[String],
    member_diag: &'static str,
    active: &mut Active,
) -> Option<&'static str> {
    for ty in written {
        let Some(instantiated) = member_type(ctx, ty, generic_params, args) else {
            return Some("E-TYP-2628");
        };
        if let Some(diag) = ffi_safe_diag_impl(ctx, Some(decl_module), &instantiated, active) {
            return Some(if diag == "E-TYP-2628" { diag } else { member_diag });
        }
    }
    None
}

fn ffi_safe_diag_impl(
    ctx: &ScopeContext<'_>,
    current_module: Option<&[String]>,
    type_ref: &TypeRef,
    active: &mut Active,
) -> Option<&'static str> {
    let Some(ty) = type_ref.as_deref() else {
        return Some("E-TYP-2628");
    };
    match &ty.node {
        TypeNode::Prim(name) => (!is_ffi_safe_prim(name)).then_some("E-TYP-2623"),
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => ffi_safe_diag_impl(ctx, current_module, base, active),
        TypeNode::RawPtr { .. } => None,
        TypeNode::Array { element, .. } => ffi_safe_diag_impl(ctx, current_module, element, active),
        TypeNode::Func { params, ret } => params
            .iter()
            .find_map(|param| ffi_safe_diag_impl(ctx, current_module, &param.r#type, active))
            .or_else(|| ffi_safe_diag_impl(ctx, current_module, ret, active)),
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args } => {
            if matches!(&path[..], [only] if id_eq(only, "Context")) {
                return Some("E-TYP-2623");
            }
            let Some((decl, resolved_path)) = resolve_nominal_type_decl_for_ffi(ctx, current_module, path) else {
                return Some("E-TYP-2628");
            };
            let key = path_key_of(&resolved_path);
            if active.contains(&key) {
                return Some("E-TYP-2628");
            }
            active.insert(key.clone());
            let decl_module = &resolved_path[..resolved_path.len() - 1];
            let diag = match decl {
                TypeDecl::Record(decl) => {
                    let fields = record_field_types(decl);
                    let unbounded = generic_params_missing_reqs(ctx, &decl.generic_params, &fields, "FfiSafe");
                    if args.is_empty() && unbounded {
                        Some("E-TYP-2629")
                    } else {
                        match resolve_decl_generic_args(ctx, &decl.generic_params, args) {
                            None => Some("E-TYP-2628"),
                            Some(_) if !has_layout_c(&decl.attrs) => Some("E-TYP-2624"),
                            Some(_) if unbounded => Some("E-TYP-2629"),
                            Some(args) => {
                                ffi_members_diag(ctx, &fields, &decl.generic_params, &args, decl_module, "E-TYP-2626", active)
                            }
                        }
                    }
                }
                TypeDecl::Enum(decl) => {
                    let payloads = enum_payload_types(decl);
                    let unbounded = generic_params_missing_reqs(ctx, &decl.generic_params, &payloads, "FfiSafe");
                    if args.is_empty() && unbounded {
                        Some("E-TYP-2629")
                    } else {
                        match resolve_decl_generic_args(ctx, &decl.generic_params, args) {
                            None => Some("E-TYP-2628"),
                            Some(_) if !has_layout_c(&decl.attrs) => Some("E-TYP-2625"),
                            Some(_) if unbounded => Some("E-TYP-2629"),
                            Some(args) => {
                                ffi_members_diag(ctx, &payloads, &decl.generic_params, &args, decl_module, "E-TYP-2627", active)
                            }
                        }
                    }
                }
                TypeDecl::TypeAlias(decl) => {
                    let target = decl
                        .r#type
                        .as_ref()
                        .and_then(|_| resolve_decl_generic_args(ctx, &decl.generic_params, args))
                        .and_then(|args| member_type(ctx, &decl.r#type, &decl.generic_params, &args));
                    match target {
                        Some(target) => ffi_safe_diag_impl(ctx, Some(decl_module), &target, active),
                        None => Some("E-TYP-2628"),
                    }
                }
                TypeDecl::Modal(_) => Some("E-TYP-2623"),
            };
            active.remove(&key);
            diag
        }
        _ => Some("E-TYP-2623"),
    }
}

/// Why a type may not appear in a foreign signature, if it may not.
pub fn ffi_safe_diag_for_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<&'static str> {
    let current_module = (!ctx.current_module.is_empty()).then_some(&ctx.current_module[..]);
    ffi_safe_diag_impl(ctx, current_module, ty, &mut Active::new())
}

pub fn ffi_safe_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    ffi_safe_diag_for_type(ctx, ty).is_none()
}

fn zeroable_impl(ctx: &ScopeContext<'_>, ty: &TypeRef, active: &mut Active) -> bool {
    let stripped = strip_perm_and_refine(ty);
    let Some(node) = stripped.as_deref().map(|ty| &ty.node) else {
        return false;
    };
    match node {
        TypeNode::Prim(name) => name != "!",
        TypeNode::Tuple(elements) => elements.iter().all(|elem| zeroable_impl(ctx, elem, active)),
        TypeNode::Array { element, .. } => zeroable_impl(ctx, element, active),
        TypeNode::Ptr { state, .. } => state.is_some_and(|state| state != PtrState::Valid),
        TypeNode::RawPtr { .. } => true,
        TypeNode::Path { path, generic_args } => {
            let key = path_key_of(path);
            let Some(decl) = ctx.sigma.types.get(&key).filter(|_| !active.contains(&key)) else {
                return false;
            };
            active.insert(key.clone());
            let mut all_zeroable = |written: &[&ast::TypePtr], params: &Option<ast::GenericParams>, args: &[TypeRef]| {
                written.iter().all(|ty| {
                    ty.is_some()
                        && member_type(ctx, ty, params, args)
                            .is_some_and(|ty| ty.is_some() && zeroable_impl(ctx, &ty, active))
                })
            };
            let zeroable = match decl {
                TypeDecl::Record(decl) => resolve_decl_generic_args(ctx, &decl.generic_params, generic_args)
                    .is_some_and(|args| all_zeroable(&record_field_types(decl), &decl.generic_params, &args)),
                // The variant with discriminant 0 must have a zeroable payload.
                TypeDecl::Enum(decl) => resolve_decl_generic_args(ctx, &decl.generic_params, generic_args)
                    .zip(enum_discriminants(decl).ok())
                    .is_some_and(|(args, discs)| {
                        decl.variants.iter().zip(&discs.discs).any(|(variant, disc)| {
                            *disc == 0 && all_zeroable(&variant_payload_types(variant), &decl.generic_params, &args)
                        })
                    }),
                TypeDecl::TypeAlias(decl) => {
                    decl.r#type.is_some()
                        && lower_type(ctx, &decl.r#type).is_ok()
                        && resolve_decl_generic_args(ctx, &decl.generic_params, generic_args)
                            .is_some_and(|args| all_zeroable(&[&decl.r#type], &decl.generic_params, &args))
                }
                TypeDecl::Modal(_) => false,
            };
            active.remove(&key);
            zeroable
        }
        _ => false,
    }
}

/// Whether the all-zero bit pattern is a value of the type.
pub fn zeroable_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    zeroable_impl(ctx, ty, &mut Active::new())
}

fn is_gpu_safe_prim(name: &str) -> bool {
    matches!(
        name,
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize" | "usize" | "f16" | "f32" | "f64" | "bool" | "()"
    )
}

const NOT_GPU_SAFE: Option<&str> = Some("E-TYP-2640");

const MAX_GPU_ALIAS_DEPTH: usize = 64;

thread_local! {
    static GPU_ALIAS_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A record or enum is GPU-safe when it is `Bitcopy`, its mentioned type parameters are
/// bounded by `GpuSafe`, and its members are GPU-safe.
fn gpu_decl_diag(
    ctx: &ScopeContext<'_>,
    self_type: &TypeRef,
    written: &[&ast::TypePtr],
    generic_params: &Option<ast::GenericParams>,
    args: &[TypeRef],
    active: &mut Active,
) -> Option<&'static str> {
    if generic_params_missing_reqs(ctx, generic_params, written, "GpuSafe") {
        return Some("E-TYP-2642");
    }
    if !bitcopy_type(ctx, self_type) {
        return NOT_GPU_SAFE;
    }
    let all_safe = written.iter().all(|ty| {
        member_type(ctx, ty, generic_params, args).is_some_and(|ty| gpu_safe_diag_impl(ctx, &ty, active).is_none())
    });
    if all_safe {
        None
    } else {
        NOT_GPU_SAFE
    }
}

fn gpu_safe_diag_impl(ctx: &ScopeContext<'_>, ty: &TypeRef, active: &mut Active) -> Option<&'static str> {
    let stripped = strip_perm_and_refine(ty);
    let node = &stripped.as_deref()?.node;
    if is_capability_type(&stripped) {
        return NOT_GPU_SAFE;
    }
    let needs_bitcopy = |then: &mut dyn FnMut() -> Option<&'static str>| {
        if bitcopy_type(ctx, &stripped) {
            then()
        } else {
            NOT_GPU_SAFE
        }
    };
    match node {
        TypeNode::Prim(name) => (!is_gpu_safe_prim(name)).then_some("E-TYP-2640"),
        TypeNode::RawPtr { element, .. } => gpu_safe_diag_impl(ctx, element, active),
        TypeNode::Array { element, .. } | TypeNode::Slice(element) => {
            needs_bitcopy(&mut || gpu_safe_diag_impl(ctx, element, active))
        }
        TypeNode::Tuple(types) | TypeNode::Union(types) => {
            needs_bitcopy(&mut || types.iter().find_map(|ty| gpu_safe_diag_impl(ctx, ty, active)))
        }
        TypeNode::String(state) => (*state != Some(StringState::View)).then_some("E-TYP-2640"),
        TypeNode::Bytes(state) => (*state != Some(BytesState::View)).then_some("E-TYP-2640"),
        TypeNode::Ptr { state: Some(PtrState::Valid), .. } => NOT_GPU_SAFE,
        TypeNode::Ptr { .. } | TypeNode::RangeFull | TypeNode::Func { .. } | TypeNode::Opaque { .. } => {
            needs_bitcopy(&mut || None)
        }
        TypeNode::Range(base)
        | TypeNode::RangeInclusive(base)
        | TypeNode::RangeFrom(base)
        | TypeNode::RangeTo(base)
        | TypeNode::RangeToInclusive(base) => needs_bitcopy(&mut || gpu_safe_diag_impl(ctx, base, active)),
        TypeNode::Path { path, generic_args: args } | TypeNode::Apply { path, args } => {
            let key = path_key_of(path);
            let record = lookup_record_decl(ctx, path);
            let enumeration = lookup_enum_decl(ctx, path);
            let nominal = match (record, enumeration) {
                (Some(decl), _) => Some((record_field_types(decl), &decl.generic_params)),
                (None, Some(decl)) => Some((enum_payload_types(decl), &decl.generic_params)),
                (None, None) => None,
            };
            if let Some((written, generic_params)) = nominal {
                if active.contains(&key) {
                    return NOT_GPU_SAFE;
                }
                let Some(args) = resolve_decl_generic_args(ctx, generic_params, args) else {
                    return NOT_GPU_SAFE;
                };
                active.insert(key.clone());
                let diag = gpu_decl_diag(ctx, &stripped, &written, generic_params, &args, active);
                active.remove(&key);
                return diag;
            }
            if lookup_modal_decl(ctx, path).is_some() {
                return NOT_GPU_SAFE;
            }
            if let Some(TypeDecl::TypeAlias(alias)) = ctx.sigma.types.get(&key) {
                // The reference follows an alias that names itself until it crashes. An
                // alias may legitimately appear within its own arguments, so only depth
                // tells the two apart.
                let depth = GPU_ALIAS_DEPTH.get();
                if depth >= MAX_GPU_ALIAS_DEPTH {
                    return NOT_GPU_SAFE;
                }
                let target = alias
                    .r#type
                    .as_ref()
                    .and_then(|_| lower_type(ctx, &alias.r#type).ok())
                    .and_then(|_| resolve_decl_generic_args(ctx, &alias.generic_params, args))
                    .and_then(|args| member_type(ctx, &alias.r#type, &alias.generic_params, &args));
                let Some(target) = target else {
                    return NOT_GPU_SAFE;
                };
                GPU_ALIAS_DEPTH.set(depth + 1);
                let diag = gpu_safe_diag_impl(ctx, &target, active);
                GPU_ALIAS_DEPTH.set(depth);
                return diag;
            }
            needs_bitcopy(&mut || None)
        }
        TypeNode::Dynamic(_)
        | TypeNode::ModalState(_)
        | TypeNode::Closure { .. }
        | TypeNode::Var(_)
        | TypeNode::Perm { .. }
        | TypeNode::Refine { .. } => NOT_GPU_SAFE,
    }
}

/// Why a type may not be used in GPU code, if it may not.
pub fn gpu_safe_diag_for_type(ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<&'static str> {
    gpu_safe_diag_impl(ctx, ty, &mut Active::new())
}

/// Types with built-in equality: numbers, `bool`, `char`, pointers and text.
pub fn eq_type(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Prim(name)) => is_int_prim(name) || is_float_prim(name) || name == "bool" || name == "char",
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => eq_type(base),
        Some(TypeNode::Ptr { .. } | TypeNode::RawPtr { .. } | TypeNode::String(_) | TypeNode::Bytes(_)) => true,
        _ => false,
    }
}

/// Also enums without payloads, and type parameters bounded by `Eq`.
pub fn eq_type_in(ctx: &ScopeContext<'_>, ty: &TypeRef) -> bool {
    if eq_type(ty) || unit_enum_type(ctx, ty) {
        return true;
    }
    match strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) => type_param_has_class_bound(ctx, path, "Eq"),
        _ => false,
    }
}

/// Integers and `char`: types with a successor and predecessor.
pub fn builtin_discrete_type(ty: &TypeRef) -> bool {
    prim_name(&strip_perm_and_refine(ty)).is_some_and(|name| is_int_prim(name) || name == "char")
}

/// Numbers and `char`; refinements are not looked through.
pub fn ord_type(ty: &TypeRef) -> bool {
    prim_name(&strip_perm(ty)).is_some_and(|name| is_int_prim(name) || is_float_prim(name) || name == "char")
}

/// Casts exist between numbers, between `bool` and integers, and between `char` and `u32`.
pub fn cast_valid(source: &TypeRef, target: &TypeRef) -> bool {
    let (source, target) = (strip_perm_and_refine(source), strip_perm_and_refine(target));
    let (Some(s), Some(t)) = (prim_name(&source), prim_name(&target)) else {
        return false;
    };
    let numeric = |name: &str| is_int_prim(name) || is_float_prim(name);
    (numeric(s) && numeric(t))
        || (s == "bool" && is_int_prim(t))
        || (is_int_prim(s) && t == "bool")
        || matches!((s, t), ("char", "u32") | ("u32", "char"))
}

/// The signature of a method that a type has intrinsically.
#[derive(Debug, Clone)]
pub struct FoundationalBuiltinMethodSig {
    pub recv_perm: Permission,
    pub recv_type: TypeRef,
    pub params: Vec<TypeFuncParam>,
    pub ret: TypeRef,
}

/// `eq` on types with equality; `successor` and `predecessor` on discrete types.
pub fn lookup_foundational_builtin_method_sig(
    ctx: Option<&ScopeContext<'_>>,
    recv_base: &TypeRef,
    name: &str,
) -> Option<FoundationalBuiltinMethodSig> {
    let base = strip_perm_and_refine(recv_base);
    base.as_ref()?;
    let sig = |params, ret| FoundationalBuiltinMethodSig { recv_perm: Permission::Const, recv_type: base.clone(), params, ret };
    let has_eq = ctx.map_or_else(|| eq_type(&base), |ctx| eq_type_in(ctx, &base));
    if id_eq(name, "eq") && has_eq {
        let other = TypeFuncParam { mode: None, r#type: make_type_perm(Permission::Const, base.clone()) };
        return Some(sig(vec![other], make_type_prim("bool")));
    }
    if (id_eq(name, "successor") || id_eq(name, "predecessor")) && builtin_discrete_type(&base) {
        return Some(sig(Vec::new(), make_type_union(vec![base.clone(), make_type_prim("()")])));
    }
    None
}
