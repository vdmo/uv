//! Classes: their method and field tables, dispatchability, and which types implement
//! them.

use std::collections::HashMap;

use uv_source::ast;

use super::class_linearization::linearize_class;
use super::record_methods::lower_receiver_perm;
use crate::caps::builtin_paths::is_execution_domain_class_path;
use crate::caps::context_caps::is_capability_class;
use crate::context::*;
use crate::resolve::scopes::{id_eq, id_key_of, path_eq, path_key_of};
use crate::resolve::scopes_lookup::resolve_class_name;
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_lower::{lower_param_mode, lower_type_as, LOWER_FOR_MODAL};
use crate::typing::type_predicates::*;
use crate::typing::types::*;

/// By path, or for a bare name through the scopes.
fn lookup_class_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::ClassDecl> {
    if let Some(decl) = ctx.sigma.classes.get(&path_key_of(path)) {
        return Some(decl);
    }
    let [name] = path else {
        return None;
    };
    let entity = resolve_class_name(ctx, name)?;
    let mut resolved = entity.origin_opt?;
    resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
    ctx.sigma.classes.get(&path_key_of(&resolved))
}

fn class_methods(decl: &ast::ClassDecl) -> impl Iterator<Item = &ast::ClassMethodDecl> {
    decl.items.iter().filter_map(|item| match item {
        ast::ClassItem::ClassMethodDecl(method) => Some(method),
        _ => None,
    })
}

fn class_fields(decl: &ast::ClassDecl) -> impl Iterator<Item = &ast::ClassFieldDecl> {
    decl.items.iter().filter_map(|item| match item {
        ast::ClassItem::ClassFieldDecl(field) => Some(field),
        _ => None,
    })
}

pub fn class_associated_types(decl: &ast::ClassDecl) -> Vec<&ast::AssociatedTypeDecl> {
    decl.items
        .iter()
        .filter_map(|item| match item {
            ast::ClassItem::AssociatedTypeDecl(assoc) => Some(assoc),
            _ => None,
        })
        .collect()
}

pub fn class_abstract_states(decl: &ast::ClassDecl) -> Vec<&ast::AbstractStateDecl> {
    decl.items
        .iter()
        .filter_map(|item| match item {
            ast::ClassItem::AbstractStateDecl(state) => Some(state),
            _ => None,
        })
        .collect()
}

pub fn is_modal_class(decl: &ast::ClassDecl) -> bool {
    decl.modal || !class_abstract_states(decl).is_empty()
}

fn lower(ctx: &ScopeContext<'_>, ty: &ast::TypePtr) -> Option<TypeRef> {
    lower_type_as(ctx, ty, LOWER_FOR_MODAL).ok()
}

/// `Self` replaced inside a class member's type. Unlike the general substitution this
/// leaves applications and ranges alone.
fn subst_self_in_class(self_type: &TypeRef, type_ref: &TypeRef) -> TypeRef {
    let ty = type_ref.as_deref()?;
    let subst = |ty: &TypeRef| subst_self_in_class(self_type, ty);
    let list = |types: &[TypeRef]| types.iter().map(subst).collect::<Vec<_>>();
    match &ty.node {
        TypeNode::Path { path, .. } if is_self_var_path(path) => self_type.clone(),
        TypeNode::Path { generic_args, .. } if generic_args.is_empty() => type_ref.clone(),
        TypeNode::Path { path, generic_args } => make_type_path_with(path.clone(), list(generic_args)),
        TypeNode::Perm { perm, base } => make_type_perm(*perm, subst(base)),
        TypeNode::Tuple(elements) => make_type_tuple(list(elements)),
        TypeNode::Array { element, length, length_expr_text } => {
            make_type_array(subst(element), *length, length_expr_text.clone())
        }
        TypeNode::Slice(element) => make_type_slice(subst(element)),
        TypeNode::Union(members) => make_type_union(list(members)),
        TypeNode::Func { params, ret } => make_type_func(
            params.iter().map(|param| TypeFuncParam { mode: param.mode, r#type: subst(&param.r#type) }).collect(),
            subst(ret),
        ),
        TypeNode::Closure { params, ret, deps_opt } => make_type_closure(
            params.iter().map(|(is_move, param)| (*is_move, subst(param))).collect(),
            subst(ret),
            deps_opt.as_ref().map(|deps| {
                deps.iter().map(|dep| SharedDep { name: dep.name.clone(), r#type: subst(&dep.r#type) }).collect()
            }),
        ),
        TypeNode::Ptr { element, state } => make_type_ptr(subst(element), *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, subst(element)),
        TypeNode::Refine { base, predicate } => make_type_refine(subst(base), predicate.clone()),
        TypeNode::ModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)),
        _ => type_ref.clone(),
    }
}

/// A class method's signature in terms of `Self`.
struct MethodSig {
    recv_type: TypeRef,
    recv_mode: Option<ParamMode>,
    params: Vec<TypeFuncParam>,
    ret: TypeRef,
}

fn method_sig_self(ctx: &ScopeContext<'_>, method: &ast::ClassMethodDecl) -> Option<MethodSig> {
    let self_type = self_var_type();
    let (recv_type, recv_mode) = match &method.receiver {
        ast::Receiver::ReceiverShorthand(recv) => {
            (make_type_perm(lower_receiver_perm(recv.perm), self_type.clone()), lower_param_mode(recv.mode_opt))
        }
        ast::Receiver::ReceiverExplicit(recv) => {
            (subst_self_in_class(&self_type, &lower(ctx, &recv.r#type)?), lower_param_mode(recv.mode_opt))
        }
    };
    let mut params = Vec::with_capacity(method.params.len());
    for param in &method.params {
        let ty = subst_self_in_class(&self_type, &lower(ctx, &param.r#type)?);
        params.push(TypeFuncParam { mode: lower_param_mode(param.mode), r#type: ty });
    }
    let ret = match &method.return_type_opt {
        Some(_) => lower(ctx, &method.return_type_opt)?,
        None => make_type_prim("()"),
    };
    Some(MethodSig { recv_type, recv_mode, params, ret: subst_self_in_class(&self_type, &ret) })
}

fn sig_equal(lhs: &MethodSig, rhs: &MethodSig) -> bool {
    lhs.recv_mode == rhs.recv_mode
        && type_equiv(&lhs.recv_type, &rhs.recv_type)
        && lhs.params.len() == rhs.params.len()
        && lhs.params.iter().zip(&rhs.params).all(|(a, b)| a.mode == b.mode && type_equiv(&a.r#type, &b.r#type))
        && type_equiv(&lhs.ret, &rhs.ret)
}

/// Whether `Self` occurs in a written type. Type arguments are not looked into.
fn self_occurs(type_ptr: &ast::TypePtr) -> bool {
    let Some(ty) = type_ptr.as_deref() else {
        return false;
    };
    use ast::TypeNode as N;
    match &ty.node {
        N::TypePathType(node) => matches!(&node.path[..], [only] if id_eq(only, "Self")),
        N::TypePermType(node) => self_occurs(&node.base),
        N::TypeTuple(node) => node.elements.iter().any(self_occurs),
        N::TypeArray(node) => self_occurs(&node.element),
        N::TypeSlice(node) => self_occurs(&node.element),
        N::TypeUnion(node) => node.types.iter().any(self_occurs),
        N::TypeFunc(node) => node.params.iter().any(|param| self_occurs(&param.r#type)) || self_occurs(&node.ret),
        N::TypeClosure(node) => {
            node.params.iter().any(|param| self_occurs(&param.r#type))
                || self_occurs(&node.ret)
                || node.deps_opt.iter().flatten().any(|dep| self_occurs(&dep.r#type))
        }
        N::TypeSafePtr(node) => self_occurs(&node.element),
        N::TypeRawPtr(node) => self_occurs(&node.element),
        N::TypeRefine(node) => self_occurs(&node.base),
        _ => false,
    }
}

fn has_own_type_params(method: &ast::ClassMethodDecl) -> bool {
    method.generic_params.as_ref().is_some_and(|params| !params.params.is_empty())
}

/// The strict rule: not generic, and `Self` nowhere in parameters or return type.
fn vtable_eligible_strict(method: &ast::ClassMethodDecl) -> bool {
    !has_own_type_params(method)
        && !self_occurs(&method.return_type_opt)
        && !method.params.iter().any(|param| self_occurs(&param.r#type))
}

/// Whether a method can be called through a dynamic class object: it is not generic and
/// takes no parameter of type exactly `Self`.
pub fn vtable_eligible(method: &ast::ClassMethodDecl) -> bool {
    let takes_self_by_value = |param: &ast::Param| {
        matches!(param.r#type.as_deref().map(|ty| &ty.node), Some(ast::TypeNode::TypePathType(path))
            if is_self_var_path(&path.path))
    };
    !has_own_type_params(method) && !method.params.iter().any(takes_self_by_value)
}

pub fn dispatchable(decl: &ast::ClassDecl) -> bool {
    class_methods(decl).all(vtable_eligible)
}

pub struct ClassMethodEntry<'c> {
    pub method: &'c ast::ClassMethodDecl,
    pub owner: Vec<String>,
}

/// The methods of a class and its superclasses, nearest first, one per name. A name
/// that reappears with the same signature is the same method; with another signature it
/// is a conflict. Methods whose signature cannot be lowered are left out.
pub fn class_method_table<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Result<Vec<ClassMethodEntry<'c>>, &'static str> {
    let mut seen: HashMap<IdKey, MethodSig> = HashMap::new();
    let mut methods = Vec::new();
    for cls_path in linearize_class(ctx, path)? {
        let decl = lookup_class_decl(ctx, &cls_path).ok_or("Superclass-Undefined")?;
        for method in class_methods(decl) {
            let Some(sig) = method_sig_self(ctx, method) else {
                continue;
            };
            match seen.get(&id_key_of(&method.name)) {
                None => {
                    seen.insert(id_key_of(&method.name), sig);
                    methods.push(ClassMethodEntry { method, owner: cls_path.clone() });
                }
                Some(first) if sig_equal(first, &sig) => {}
                Some(_) => return Err("E-TYP-2505"),
            }
        }
    }
    Ok(methods)
}

/// The fields of a class and its superclasses, with the same rule for repeated names.
pub fn class_field_table<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Result<Vec<&'c ast::ClassFieldDecl>, &'static str> {
    let mut all = Vec::new();
    for cls_path in linearize_class(ctx, path)? {
        all.extend(class_fields(lookup_class_decl(ctx, &cls_path).ok_or("Superclass-Undefined")?));
    }
    let mut seen: HashMap<IdKey, TypeRef> = HashMap::new();
    let mut fields = Vec::new();
    for field in all {
        let Some(lowered) = lower(ctx, &field.r#type) else {
            continue;
        };
        match seen.get(&id_key_of(&field.name)) {
            None => {
                seen.insert(id_key_of(&field.name), lowered);
                fields.push(field);
            }
            Some(first) if type_equiv(first, &lowered) => {}
            Some(_) => return Err("E-TYP-2505"),
        }
    }
    Ok(fields)
}

pub fn lookup_class_method<'c>(ctx: &'c ScopeContext<'_>, path: &[String], name: &str) -> Option<&'c ast::ClassMethodDecl> {
    class_method_table(ctx, path).ok()?.into_iter().map(|entry| entry.method).find(|method| id_eq(&method.name, name))
}

fn undispatchable(method: &ast::ClassMethodDecl) -> &'static str {
    if has_own_type_params(method) {
        "E-TYP-2542"
    } else {
        "E-TYP-2541"
    }
}

/// Why a class cannot be used as a dynamic type, if it cannot. Each class in the
/// hierarchy is checked with the lenient rule, then the merged method table with the
/// strict one.
pub fn class_dispatchability_diagnostic(ctx: &ScopeContext<'_>, path: &[String]) -> Option<&'static str> {
    let order = match linearize_class(ctx, path) {
        Ok(order) => order,
        Err(diag_id) => return Some(diag_id),
    };
    for cls_path in &order {
        let Some(decl) = lookup_class_decl(ctx, cls_path) else {
            return Some("Superclass-Undefined");
        };
        if let Some(method) = class_methods(decl).find(|method| !vtable_eligible(method)) {
            return Some(undispatchable(method));
        }
    }
    match class_method_table(ctx, path) {
        Err(diag_id) => Some(diag_id),
        Ok(table) => table.iter().find(|entry| !vtable_eligible_strict(entry.method)).map(|entry| undispatchable(entry.method)),
    }
}

pub fn class_dispatchable(ctx: &ScopeContext<'_>, path: &[String]) -> bool {
    class_dispatchability_diagnostic(ctx, path).is_none()
}

/// Whether `sub` is `sup` or has it among its superclasses.
pub fn class_subtypes(ctx: &ScopeContext<'_>, sub: &[String], sup: &[String]) -> bool {
    path_eq(sub, sup) || linearize_class(ctx, sub).is_ok_and(|order| order.iter().any(|entry| path_eq(entry, sup)))
}

fn bound_reaches(ctx: &ScopeContext<'_>, bounds: &[ast::TypeBound], class: &[String]) -> bool {
    bounds.iter().any(|bound| path_eq(&bound.class_path, class) || class_subtypes(ctx, &bound.class_path, class))
}

/// A type parameter in scope, by name: the innermost type entity of that name decides.
fn type_param_bound_satisfies_class(ctx: &ScopeContext<'_>, type_path: &[String], class: &[String]) -> bool {
    let [name] = type_path else {
        return false;
    };
    let key = id_key_of(name);
    ctx.scopes
        .iter()
        .filter_map(|scope| scope.get(&key))
        .find(|entity| {
            entity.kind == EntityKind::Type && entity.target_opt.as_ref().is_none_or(|target| id_eq(target, name))
        })
        .is_some_and(|entity| bound_reaches(ctx, &entity.type_param_class_bounds, class))
}

fn dynamic_class_path_reaches(ctx: &ScopeContext<'_>, from: &[String], target: &[String], depth: usize) -> bool {
    if depth > 64 {
        return false;
    }
    if path_eq(from, target) {
        return true;
    }
    lookup_class_decl(ctx, from)
        .is_some_and(|decl| decl.supers.iter().any(|sup| dynamic_class_path_reaches(ctx, sup, target, depth + 1)))
}

/// `$C` implements a capability class that `C` is or inherits from.
fn dynamic_type_implements_class(ctx: &ScopeContext<'_>, dynamic_path: &[String], class: &[String]) -> bool {
    is_capability_class(ctx, class)
        && ((is_execution_domain_class_path(class) && is_execution_domain_class_path(dynamic_path))
            || dynamic_class_path_reaches(ctx, dynamic_path, class, 0))
}

/// Whether a type implements a class: intrinsically for the foundational classes,
/// through a type parameter's bound, or by declaring it.
pub fn type_implements_class(ctx: &ScopeContext<'_>, ty: &TypeRef, class: &[String]) -> bool {
    let stripped = strip_perm_and_refine(ty);
    let Some(node) = stripped.as_deref().map(|ty| &ty.node) else {
        return false;
    };
    if let [name] = class {
        let intrinsic = match name.as_str() {
            _ if id_eq(name, "Bitcopy") => Some(bitcopy_type(ctx, &stripped)),
            _ if id_eq(name, "Clone") => Some(clone_type(ctx, &stripped)),
            _ if id_eq(name, "Drop") => Some(drop_type(ctx, &stripped)),
            _ if id_eq(name, "FfiSafe") => Some(ffi_safe_type(ctx, &stripped)),
            _ if id_eq(name, "GpuSafe") => Some(gpu_safe_diag_for_type(ctx, &stripped).is_none()),
            // A type that is not intrinsically comparable or discrete may still declare it.
            _ if id_eq(name, "Eq") => eq_type_in(ctx, &stripped).then_some(true),
            _ if id_eq(name, "Discrete") => builtin_discrete_type(&stripped).then_some(true),
            _ => None,
        };
        if let Some(answer) = intrinsic {
            return answer;
        }
    }
    let (path, generic_args) = match node {
        TypeNode::Dynamic(dynamic_path) => return dynamic_type_implements_class(ctx, dynamic_path, class),
        TypeNode::Path { path, generic_args } => (path, generic_args),
        _ => return false,
    };
    if generic_args.is_empty() && type_param_bound_satisfies_class(ctx, path, class) {
        return true;
    }
    let implements = match ctx.sigma.types.get(&path_key_of(path)) {
        Some(TypeDecl::Record(decl)) => &decl.implements,
        Some(TypeDecl::Enum(decl)) => &decl.implements,
        Some(TypeDecl::Modal(decl)) => &decl.implements,
        _ => return false,
    };
    implements.iter().any(|implemented| path_eq(implemented, class) || class_subtypes(ctx, implemented, class))
}

/// The class methods without a body that a record does not define.
pub fn missing_impl_methods(ctx: &ScopeContext<'_>, class_path: &[String], implementor: &ast::RecordDecl) -> Option<Vec<String>> {
    let class_decl = lookup_class_decl(ctx, class_path)?;
    let defines = |name: &str| {
        implementor.members.iter().any(|member| matches!(member, ast::RecordMember::MethodDecl(method) if id_eq(&method.name, name)))
    };
    Some(
        class_methods(class_decl)
            .filter(|method| method.body_opt.is_none() && !defines(&method.name))
            .map(|method| method.name.clone())
            .collect(),
    )
}

/// An implementation is allowed where the type or the class is declared in the current
/// assembly.
pub fn check_orphan_rule(ctx: &ScopeContext<'_>, type_path: &[String], class_path: &[String], current_module: &[String]) -> bool {
    let local = |path: &[String]| matches!((path.first(), current_module.first()), (Some(a), Some(b)) if id_eq(a, b));
    (ctx.sigma.types.contains_key(&path_key_of(type_path)) && local(type_path))
        || (ctx.sigma.classes.contains_key(&path_key_of(class_path)) && local(class_path))
}
