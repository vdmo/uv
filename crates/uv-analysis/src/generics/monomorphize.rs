//! Substituting type arguments for type parameters.

use std::collections::{BTreeMap, BTreeSet};

use uv_core::diagnostics::DiagnosticStream;
use uv_source::ast;

use super::generic_params::{internal_generic_diagnostic, rule_diagnostic};
use crate::typing::type_equiv::type_equiv;
use crate::typing::type_lower::{lower_bytes_state, lower_permission, lower_ptr_state, lower_raw_ptr_qual, lower_string_state};
use crate::typing::types::*;

pub type TypeSubst = BTreeMap<String, TypeRef>;

/// The type with parameters replaced. A bare one-segment path that names a parameter is
/// replaced whole; unions are canonicalised again, since members may now coincide.
pub fn instantiate_type(type_ref: &TypeRef, subst: &TypeSubst) -> TypeRef {
    let Some(ty) = type_ref else {
        return None;
    };
    let inst = |ty: &TypeRef| instantiate_type(ty, subst);
    let list = |types: &[TypeRef]| types.iter().map(inst).collect::<Vec<_>>();
    match &ty.node {
        TypeNode::Path { path, generic_args } => {
            if let [only] = &path[..] {
                if let Some(replacement) = subst.get(only) {
                    return replacement.clone();
                }
            }
            if generic_args.is_empty() {
                return type_ref.clone();
            }
            make_type_path_with(path.clone(), list(generic_args))
        }
        TypeNode::Apply { path, args } => make_type_apply(path.clone(), list(args)),
        TypeNode::Perm { perm, base } => make_type_perm(*perm, inst(base)),
        TypeNode::Tuple(elements) => make_type_tuple(list(elements)),
        TypeNode::Array { element, length, length_expr_text } => {
            make_type_array(inst(element), *length, length_expr_text.clone())
        }
        TypeNode::Slice(element) => make_type_slice(inst(element)),
        TypeNode::Ptr { element, state } => make_type_ptr(inst(element), *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, inst(element)),
        TypeNode::Union(members) => make_type_union(list(members)),
        TypeNode::Func { params, ret } => make_type_func(
            params.iter().map(|param| TypeFuncParam { mode: param.mode, r#type: inst(&param.r#type) }).collect(),
            inst(ret),
        ),
        TypeNode::Closure { params, ret, deps_opt } => make_type_closure(
            params.iter().map(|(is_move, param)| (*is_move, inst(param))).collect(),
            inst(ret),
            deps_opt.as_ref().map(|deps| {
                deps.iter().map(|dep| SharedDep { name: dep.name.clone(), r#type: inst(&dep.r#type) }).collect()
            }),
        ),
        TypeNode::ModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)),
        TypeNode::Refine { base, predicate } => make_type_refine(inst(base), predicate.clone()),
        TypeNode::Range(base) => make_type(TypeNode::Range(inst(base))),
        TypeNode::RangeInclusive(base) => make_type(TypeNode::RangeInclusive(inst(base))),
        TypeNode::RangeFrom(base) => make_type(TypeNode::RangeFrom(inst(base))),
        TypeNode::RangeTo(base) => make_type(TypeNode::RangeTo(inst(base))),
        TypeNode::RangeToInclusive(base) => make_type(TypeNode::RangeToInclusive(inst(base))),
        TypeNode::RangeFull => make_type(TypeNode::RangeFull),
        TypeNode::Prim(_)
        | TypeNode::Var(_)
        | TypeNode::String(_)
        | TypeNode::Bytes(_)
        | TypeNode::Dynamic(_)
        | TypeNode::Opaque { .. } => type_ref.clone(),
    }
}

/// Lowers the default of a type parameter. This runs without a scope, so it differs
/// from ordinary lowering: array lengths are not evaluated (the length is 0), a generic
/// path stays a path with arguments, and range types are not supported.
fn lower_default_type(type_ptr: &ast::TypePtr) -> Option<TypeRef> {
    let ty = type_ptr.as_deref()?;
    let list = |types: &[ast::TypePtr]| types.iter().map(lower_default_type).collect::<Option<Vec<_>>>();
    use ast::TypeNode as N;
    Some(match &ty.node {
        N::TypePrim(node) => make_type_prim(&node.name),
        N::TypePathType(node) if node.generic_args.is_empty() => make_type_path(node.path.clone()),
        N::TypePathType(node) => make_type_path_with(node.path.clone(), list(&node.generic_args)?),
        N::TypeTuple(node) => make_type_tuple(list(&node.elements)?),
        N::TypeArray(node) => make_type_array(lower_default_type(&node.element)?, 0, None),
        N::TypeSlice(node) => make_type_slice(lower_default_type(&node.element)?),
        N::TypePermType(node) => make_type_perm(lower_permission(node.perm), lower_default_type(&node.base)?),
        N::TypeUnion(node) => make_type_union(list(&node.types)?),
        N::TypeFunc(node) => {
            let mut params = Vec::with_capacity(node.params.len());
            for param in &node.params {
                let mode = param.mode.map(|_| ParamMode::Move);
                params.push(TypeFuncParam { mode, r#type: lower_default_type(&param.r#type)? });
            }
            make_type_func(params, lower_default_type(&node.ret)?)
        }
        N::TypeClosure(node) => {
            let mut params = Vec::with_capacity(node.params.len());
            for param in &node.params {
                params.push((param.mode.is_some(), lower_default_type(&param.r#type)?));
            }
            let ret = lower_default_type(&node.ret)?;
            let deps_opt = match &node.deps_opt {
                Some(deps) => Some(
                    deps.iter()
                        .map(|dep| Some(SharedDep { name: dep.name.clone(), r#type: lower_default_type(&dep.r#type)? }))
                        .collect::<Option<Vec<_>>>()?,
                ),
                None => None,
            };
            make_type_closure(params, ret, deps_opt)
        }
        N::TypeSafePtr(node) => make_type_ptr(lower_default_type(&node.element)?, lower_ptr_state(node.state)),
        N::TypeRawPtr(node) => make_type_raw_ptr(lower_raw_ptr_qual(node.qual), lower_default_type(&node.element)?),
        N::TypeString(node) => make_type_string(lower_string_state(node.state)),
        N::TypeBytes(node) => make_type_bytes(lower_bytes_state(node.state)),
        N::TypeDynamic(node) => make_type_dynamic(node.path.clone()),
        N::TypeModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)?),
        N::TypeOpaque(node) => {
            make_type(TypeNode::Opaque { class_path: node.path.clone(), origin: None, origin_span: Default::default() })
        }
        N::TypeRefine(node) => make_type_refine(lower_default_type(&node.base)?, node.predicate.clone()),
        _ => return None,
    })
}

/// Parameters to arguments. A parameter without an argument takes its default, which
/// may mention earlier parameters; a default that cannot be lowered becomes `!`. A
/// parameter with neither is left out.
pub fn build_substitution(params: &[ast::TypeParam], args: &[TypeRef]) -> TypeSubst {
    let mut subst = TypeSubst::new();
    for (param, arg) in params.iter().zip(args) {
        subst.insert(param.name.clone(), arg.clone());
    }
    for param in params.iter().skip(args.len()) {
        if param.default_type.is_some() {
            let default = lower_default_type(&param.default_type)
                .and_then(|default| instantiate_type(&default, &subst).map(Some))
                .unwrap_or_else(|| make_type_prim("!"));
            subst.insert(param.name.clone(), default);
        }
    }
    subst
}

/// Substitution by position: the i-th name stands for the i-th argument. Of several
/// parameters with one name the first counts.
pub fn apply_generic_substitution(ty: &TypeRef, param_names: &[String], args: &[TypeRef]) -> TypeRef {
    if ty.is_none() {
        return None;
    }
    let mut subst = TypeSubst::new();
    for (name, arg) in param_names.iter().zip(args) {
        subst.entry(name.clone()).or_insert_with(|| arg.clone());
    }
    instantiate_type(ty, &subst)
}

/// A declaration with the type arguments it is wanted for.
#[derive(Debug, Clone)]
pub struct InstantiationKey {
    pub decl_path: TypePath,
    pub args: Vec<TypeRef>,
}

impl InstantiationKey {
    /// The reference's equality, which compares arguments by equivalence; the ordering
    /// below compares them by key.
    pub fn equivalent(&self, other: &InstantiationKey) -> bool {
        self.decl_path == other.decl_path
            && self.args.len() == other.args.len()
            && self.args.iter().zip(&other.args).all(|(a, b)| type_equiv(a, b))
    }
}

impl Ord for InstantiationKey {
    /// By path, then by number of arguments, then argument by argument in type order
    /// with a missing type first.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let key = |arg: &TypeRef| arg.as_ref().map(|_| type_key_of(arg));
        self.decl_path
            .cmp(&other.decl_path)
            .then(self.args.len().cmp(&other.args.len()))
            .then_with(|| self.args.iter().map(key).cmp(other.args.iter().map(key)))
    }
}

impl PartialOrd for InstantiationKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for InstantiationKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for InstantiationKey {}

#[derive(Debug, Clone)]
pub struct InstantiationEntry {
    pub key: InstantiationKey,
    pub processed: bool,
    pub dependencies: BTreeSet<InstantiationKey>,
}

/// The instantiations a program asks for.
#[derive(Debug, Default)]
pub struct MonomorphizeContext {
    instantiations: BTreeMap<InstantiationKey, InstantiationEntry>,
    worklist: Vec<InstantiationKey>,
    current_depth: usize,
}

impl MonomorphizeContext {
    pub const MAX_DEPTH: usize = 128;

    /// Records the instantiation unless it is known.
    pub fn demand(&mut self, key: &InstantiationKey) {
        if self.instantiations.contains_key(key) {
            return;
        }
        let entry = InstantiationEntry { key: key.clone(), processed: false, dependencies: BTreeSet::new() };
        self.instantiations.insert(key.clone(), entry);
        self.worklist.push(key.clone());
    }

    pub fn has_instantiation(&self, key: &InstantiationKey) -> bool {
        self.instantiations.contains_key(key)
    }

    pub fn instantiations(&self) -> &BTreeMap<InstantiationKey, InstantiationEntry> {
        &self.instantiations
    }

    /// Marks every demanded instantiation processed, newest first. The reference does
    /// no more than this: processing an entry demands nothing, so the depth limit is
    /// never reached.
    pub fn process_to_fixed_point(&mut self) -> bool {
        while !self.worklist.is_empty() {
            if self.current_depth >= Self::MAX_DEPTH {
                return false;
            }
            let Some(key) = self.worklist.pop() else {
                break;
            };
            if let Some(entry) = self.instantiations.get_mut(&key).filter(|entry| !entry.processed) {
                entry.processed = true;
            }
        }
        true
    }
}

pub fn build_modal_ref_substitution(params: &[ast::TypeParam], args: &[TypeRef]) -> TypeSubst {
    build_substitution(params, args)
}

/// Only nominal types can implement a class: a path, an opaque or dynamic type, or a
/// modal state, under any permissions.
fn can_satisfy_class_bound_by_shape(ty: &TypeRef) -> bool {
    let mut current = ty;
    while let Some(TypeNode::Perm { base, .. }) = current.as_deref().map(|ty| &ty.node) {
        current = base;
    }
    matches!(
        current.as_deref().map(|ty| &ty.node),
        Some(TypeNode::Path { .. } | TypeNode::Opaque { .. } | TypeNode::Dynamic(_) | TypeNode::ModalState(_))
    )
}

/// Why the arguments of a generic use do not fit its bounded parameters.
#[derive(Debug, Clone, Default)]
pub struct BoundCheckError {
    pub diag_id: Option<&'static str>,
    pub param_name: String,
    pub type_name: String,
    pub bound_name: String,
    pub diagnostics: DiagnosticStream,
}

/// Each bounded parameter has an argument or a default (`E-TYP-2303`), its bounds are
/// well formed (`E-TYP-2305`), and the argument is of a kind that can implement a class
/// (`E-TYP-2302`). Whether it does implement the class is checked elsewhere.
pub fn check_bounds_satisfied(params: &[ast::TypeParam], args: &[TypeRef]) -> Result<(), BoundCheckError> {
    for (index, param) in params.iter().enumerate() {
        if param.bounds.is_empty() {
            continue;
        }
        let fail = |diag_id: Option<&'static str>, type_name: String, bound_name: String, diagnostic| BoundCheckError {
            diag_id,
            param_name: param.name.clone(),
            type_name,
            bound_name,
            diagnostics: vec![diagnostic],
        };
        let arg = match args.get(index) {
            Some(arg) => arg.clone(),
            None if param.default_type.is_some() => match lower_default_type(&param.default_type).flatten() {
                Some(default) => Some(default),
                None => {
                    let message = format!(
                        "Internal error: unable to lower default type for bounded parameter '{}'",
                        param.name
                    );
                    let diagnostic = internal_generic_diagnostic(&param.span, message);
                    return Err(fail(None, String::new(), String::new(), diagnostic));
                }
            },
            None => None,
        };
        if arg.is_none() {
            let message = format!("Missing type argument for bounded generic parameter '{}'", param.name);
            let diagnostic = rule_diagnostic("E-TYP-2303", &param.span, message);
            return Err(fail(Some("E-TYP-2303"), String::new(), String::new(), diagnostic));
        }
        let type_name = type_to_string(&arg);
        for bound in &param.bounds {
            if bound.class_path.is_empty() {
                let message = format!("Class bound for parameter '{}' is empty or malformed", param.name);
                let diagnostic = rule_diagnostic("E-TYP-2305", &param.span, message);
                return Err(fail(Some("E-TYP-2305"), type_name, String::new(), diagnostic));
            }
            if !can_satisfy_class_bound_by_shape(&arg) {
                let bound_name = bound.class_path.join("::");
                let message = format!(
                    "Type '{type_name}' cannot satisfy class bound '{bound_name}' for parameter '{}'",
                    param.name
                );
                let diagnostic = rule_diagnostic("E-TYP-2302", &param.span, message);
                return Err(fail(Some("E-TYP-2302"), type_name, bound_name, diagnostic));
            }
        }
    }
    Ok(())
}

fn is_type_param_name(params: &[ast::TypeParam], name: &str) -> bool {
    params.iter().any(|param| param.name == name)
}

fn names_type_param<'t>(params: &[ast::TypeParam], ty: &'t Type) -> Option<&'t str> {
    match &ty.node {
        TypeNode::Path { path, .. } => match &path[..] {
            [only] if is_type_param_name(params, only) => Some(only),
            _ => None,
        },
        _ => None,
    }
}

fn contains_type_param_for_inference(params: &[ast::TypeParam], type_ref: &TypeRef) -> bool {
    let Some(ty) = type_ref.as_deref() else {
        return false;
    };
    if names_type_param(params, ty).is_some() {
        return true;
    }
    let contains = |ty: &TypeRef| contains_type_param_for_inference(params, ty);
    match &ty.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => contains(base),
        TypeNode::Tuple(types) | TypeNode::Union(types) => types.iter().any(contains),
        TypeNode::Array { element, .. }
        | TypeNode::Slice(element)
        | TypeNode::Ptr { element, .. }
        | TypeNode::RawPtr { element, .. } => contains(element),
        TypeNode::Func { params: func_params, ret } => {
            func_params.iter().any(|param| contains(&param.r#type)) || contains(ret)
        }
        TypeNode::Path { generic_args, .. } => generic_args.iter().any(contains),
        TypeNode::ModalState(node) => node.generic_args.iter().any(contains),
        TypeNode::Closure { params: closure_params, ret, deps_opt } => {
            closure_params.iter().any(|(_, param)| contains(param))
                || contains(ret)
                || deps_opt.iter().flatten().any(|dep| contains(&dep.r#type))
        }
        _ => false,
    }
}

/// Matches the expected type against the actual one, binding each type parameter to
/// what stands in its place; a parameter met again must stand for an equivalent type.
fn bind_type_params_for_inference(
    params: &[ast::TypeParam],
    expected_ref: &TypeRef,
    actual_ref: &TypeRef,
    bindings: &mut BTreeMap<String, TypeRef>,
) -> bool {
    let (Some(expected), Some(actual)) = (expected_ref.as_deref(), actual_ref.as_deref()) else {
        return false;
    };
    if let Some(name) = names_type_param(params, expected) {
        return match bindings.get(name) {
            Some(bound) => type_equiv(bound, actual_ref),
            None => {
                bindings.insert(name.to_string(), actual_ref.clone());
                true
            }
        };
    }
    let mut bind = |expected: &TypeRef, actual: &TypeRef| bind_type_params_for_inference(params, expected, actual, bindings);
    let mut bind_all = |expected: &[TypeRef], actual: &[TypeRef]| {
        expected.len() == actual.len() && expected.iter().zip(actual).all(|(e, a)| bind(e, a))
    };
    use TypeNode as N;
    match (&expected.node, &actual.node) {
        (N::Perm { perm: ep, base: eb }, N::Perm { perm: ap, base: ab }) => ep == ap && bind(eb, ab),
        (N::Tuple(e), N::Tuple(a)) | (N::Union(e), N::Union(a)) => bind_all(e, a),
        (N::Array { element: e, length: el, .. }, N::Array { element: a, length: al, .. }) => el == al && bind(e, a),
        (N::Slice(e), N::Slice(a)) => bind(e, a),
        (N::Ptr { element: e, state: es }, N::Ptr { element: a, state: as_ }) => es == as_ && bind(e, a),
        (N::RawPtr { qual: eq, element: e }, N::RawPtr { qual: aq, element: a }) => eq == aq && bind(e, a),
        (N::Func { params: e, ret: er }, N::Func { params: a, ret: ar }) => {
            e.len() == a.len()
                && e.iter().zip(a).all(|(e, a)| e.mode == a.mode && bind(&e.r#type, &a.r#type))
                && bind(er, ar)
        }
        (N::Path { path: e, generic_args: eargs }, N::Path { path: a, generic_args: aargs }) => {
            e == a && bind_all(eargs, aargs)
        }
        (N::ModalState(e), N::ModalState(a)) => {
            e.path == a.path && e.state == a.state && bind_all(&e.generic_args, &a.generic_args)
        }
        (N::Dynamic(e), N::Dynamic(a)) => e == a,
        (N::Refine { base: e, .. }, N::Refine { base: a, .. }) => bind(e, a),
        // Shared dependencies are not compared.
        (N::Closure { params: e, ret: er, .. }, N::Closure { params: a, ret: ar, .. }) => {
            e.len() == a.len() && e.iter().zip(a).all(|(e, a)| e.0 == a.0 && bind(&e.1, &a.1)) && bind(er, ar)
        }
        (N::Opaque { class_path: e, .. }, N::Opaque { class_path: a, .. }) => e == a,
        (N::String(e), N::String(a)) => e == a,
        (N::Bytes(e), N::Bytes(a)) => e == a,
        (N::Prim(e), N::Prim(a)) => e == a,
        (N::Range(e), N::Range(a))
        | (N::RangeInclusive(e), N::RangeInclusive(a))
        | (N::RangeFrom(e), N::RangeFrom(a))
        | (N::RangeTo(e), N::RangeTo(a))
        | (N::RangeToInclusive(e), N::RangeToInclusive(a)) => bind(e, a),
        (N::RangeFull, N::RangeFull) => true,
        // The remaining forms have no parts to bind.
        (N::Var(_) | N::Apply { .. }, _) => type_equiv(expected_ref, actual_ref),
        _ => false,
    }
}

#[derive(Debug, Clone, Default)]
pub struct InferResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    /// One per parameter; a parameter that could not be inferred has none.
    pub inferred_args: Vec<TypeRef>,
}

/// Infers type arguments from the argument types of a call: each parameter type that
/// mentions a type parameter must match its argument (`E-TYP-2301`), and a parameter
/// that no argument determines takes its default or is an error.
pub fn infer_type_arguments(
    params: &[ast::TypeParam],
    expected_param_types: &[TypeRef],
    actual_arg_types: &[TypeRef],
) -> InferResult {
    let mut result = InferResult { ok: true, diag_id: None, inferred_args: vec![None; params.len()] };
    let mut bindings = BTreeMap::new();
    for (expected, actual) in expected_param_types.iter().zip(actual_arg_types) {
        if expected.is_none() || actual.is_none() {
            continue;
        }
        let contains_type_param = contains_type_param_for_inference(params, expected);
        let matched = bind_type_params_for_inference(params, expected, actual, &mut bindings);
        if contains_type_param && !matched {
            result.ok = false;
            result.diag_id = Some("E-TYP-2301");
            return result;
        }
    }
    for (index, param) in params.iter().enumerate() {
        if let Some(bound) = bindings.get(&param.name) {
            result.inferred_args[index] = bound.clone();
        } else if param.default_type.is_some() {
            let Some(default) = lower_default_type(&param.default_type).flatten() else {
                result.ok = false;
                result.diag_id = None;
                return result;
            };
            result.inferred_args[index] = Some(default);
        } else {
            result.ok = false;
            result.diag_id = Some("E-TYP-2301");
        }
    }
    result
}
