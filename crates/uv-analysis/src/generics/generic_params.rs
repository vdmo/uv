//! Generic parameter lists: counting, validation, and the scope that binds them.

use std::collections::BTreeSet;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{Diagnostic, DiagnosticStream, Severity};
use uv_core::span::Span;
use uv_source::ast;

use crate::context::*;
use crate::resolve::scopes::id_key_of;
use crate::typing::type_lower::{
    lower_bytes_state, lower_permission, lower_ptr_state, lower_raw_ptr_qual, lower_string_state,
};
use crate::typing::types::*;

fn params(params_opt: &Option<ast::GenericParams>) -> &[ast::TypeParam] {
    params_opt.as_ref().map_or(&[], |params| &params.params)
}

/// A diagnostic without a code, for failures that no rule describes.
pub(crate) fn internal_generic_diagnostic(span: &Span, message: String) -> Diagnostic {
    Diagnostic { severity: Severity::Error, span: Some(span.clone()), message, ..Default::default() }
}

/// The diagnostic of a rule with its message replaced, or an internal one when the rule
/// is unknown.
pub(crate) fn rule_diagnostic(diag_id: &str, span: &Span, message: String) -> Diagnostic {
    match make_diagnostic_by_id(diag_id, Some(span.clone())) {
        Some(mut diag) => {
            diag.message = message;
            diag
        }
        None => internal_generic_diagnostic(span, format!("Internal error: unresolved diagnostic id '{diag_id}'")),
    }
}

#[derive(Debug, Clone)]
pub struct TypeParamInfo {
    pub name: String,
    pub class_bounds: Vec<ast::TypeBound>,
    pub default_type: Option<TypeRef>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ConstParamInfo {
    pub name: String,
    /// The parameter's type when it is an integer type.
    pub r#type: TypeRef,
    pub default_value: Option<i64>,
    pub span: Span,
}

/// Why a parameter list was rejected.
#[derive(Debug, Clone, Default)]
pub struct GenericParamError {
    pub diag_id: Option<&'static str>,
    /// The parameter at fault.
    pub param_name: String,
    pub diagnostics: DiagnosticStream,
}

/// A decimal integer as `std::from_chars` reads it: an optional `-`, then digits only.
fn parse_i64_literal(text: &str) -> Option<i64> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

fn parse_const_default_value(default_type: &ast::TypePtr) -> Option<i64> {
    match &default_type.as_deref()?.node {
        ast::TypeNode::TypePrim(node) => parse_i64_literal(&node.name),
        ast::TypeNode::TypePathType(node) => match &node.path[..] {
            [only] if node.generic_args.is_empty() => parse_i64_literal(only),
            _ => None,
        },
        _ => None,
    }
}

/// Lowers a default type without a scope, only to see that it has a supported shape:
/// names are not looked up, array lengths are not evaluated, and an unsupported form
/// inside the type becomes a hole rather than a failure.
fn lower_type_for_validation(type_ptr: &ast::TypePtr) -> TypeRef {
    let ty = type_ptr.as_deref()?;
    let lower = lower_type_for_validation;
    let list = |types: &[ast::TypePtr]| types.iter().map(lower).collect::<Vec<_>>();
    use ast::TypeNode as N;
    match &ty.node {
        N::TypePrim(node) => make_type_prim(&node.name),
        N::TypePathType(node) => make_type_path_with(node.path.clone(), list(&node.generic_args)),
        N::TypeTuple(node) => make_type_tuple(list(&node.elements)),
        N::TypeArray(node) => make_type_array(lower(&node.element), 0, None),
        N::TypeSlice(node) => make_type_slice(lower(&node.element)),
        N::TypePermType(node) => make_type_perm(lower_permission(node.perm), lower(&node.base)),
        N::TypeUnion(node) => make_type_union(list(&node.types)),
        N::TypeFunc(node) => {
            let params = node
                .params
                .iter()
                .map(|param| TypeFuncParam { mode: param.mode.map(|_| ParamMode::Move), r#type: lower(&param.r#type) })
                .collect();
            make_type_func(params, lower(&node.ret))
        }
        N::TypeClosure(node) => {
            let params = node.params.iter().map(|param| (param.mode.is_some(), lower(&param.r#type))).collect();
            let ret = lower(&node.ret);
            let deps_opt = node.deps_opt.as_ref().map(|deps| {
                deps.iter().map(|dep| SharedDep { name: dep.name.clone(), r#type: lower(&dep.r#type) }).collect()
            });
            make_type_closure(params, ret, deps_opt)
        }
        N::TypeSafePtr(node) => make_type_ptr(lower(&node.element), lower_ptr_state(node.state)),
        N::TypeRawPtr(node) => make_type_raw_ptr(lower_raw_ptr_qual(node.qual), lower(&node.element)),
        N::TypeString(node) => make_type_string(lower_string_state(node.state)),
        N::TypeBytes(node) => make_type_bytes(lower_bytes_state(node.state)),
        N::TypeDynamic(node) => make_type_dynamic(node.path.clone()),
        N::TypeModalState(node) => make_type_modal_state(node.path.clone(), &node.state, list(&node.generic_args)),
        N::TypeOpaque(node) => {
            make_type(TypeNode::Opaque { class_path: node.path.clone(), origin: None, origin_span: Span::default() })
        }
        N::TypeRefine(node) => make_type_refine(lower(&node.base), node.predicate.clone()),
        _ => None,
    }
}

/// Names must be unique and defaults must come last; then each parameter is described.
pub fn validate_generic_params(
    params_opt: &Option<ast::GenericParams>,
) -> Result<Vec<TypeParamInfo>, GenericParamError> {
    check_param_uniqueness(params_opt)?;
    validate_default_values(params_opt)?;
    Ok(params(params_opt).iter().map(parse_type_param).collect())
}

pub fn check_param_uniqueness(params_opt: &Option<ast::GenericParams>) -> Result<(), GenericParamError> {
    let mut seen_names = BTreeSet::new();
    for param in params(params_opt) {
        if !seen_names.insert(&param.name) {
            let message = format!("Duplicate type parameter name: {}", param.name);
            return Err(GenericParamError {
                diag_id: Some("E-TYP-2304"),
                param_name: param.name.clone(),
                diagnostics: vec![rule_diagnostic("E-TYP-2304", &param.span, message)],
            });
        }
    }
    Ok(())
}

pub fn parse_type_param(param: &ast::TypeParam) -> TypeParamInfo {
    TypeParamInfo {
        name: param.name.clone(),
        class_bounds: param.bounds.clone(),
        default_type: param.default_type.is_some().then(|| lower_type_for_validation(&param.default_type)),
        span: param.span.clone(),
    }
}

pub fn parse_const_param(param: &ast::TypeParam, param_type: &TypeRef) -> ConstParamInfo {
    ConstParamInfo {
        name: param.name.clone(),
        r#type: if is_valid_const_param_type(param_type) { param_type.clone() } else { None },
        default_value: parse_const_default_value(&param.default_type),
        span: param.span.clone(),
    }
}

/// A parameter without a default may not follow one with a default, and every default
/// must have a supported shape.
pub fn validate_default_values(params_opt: &Option<ast::GenericParams>) -> Result<(), GenericParamError> {
    let mut seen_default = false;
    for param in params(params_opt) {
        let has_default = param.default_type.is_some();
        if seen_default && !has_default {
            let message = format!("Parameter '{}' without default follows parameter with default", param.name);
            return Err(GenericParamError {
                diag_id: Some("E-TYP-2303"),
                param_name: param.name.clone(),
                diagnostics: vec![rule_diagnostic("E-TYP-2303", &param.span, message)],
            });
        }
        if has_default {
            if lower_type_for_validation(&param.default_type).is_none() {
                let message =
                    format!("Internal error: unable to lower default type for generic parameter '{}'", param.name);
                return Err(GenericParamError {
                    diag_id: None,
                    param_name: param.name.clone(),
                    diagnostics: vec![internal_generic_diagnostic(&param.span, message)],
                });
            }
            seen_default = true;
        }
    }
    Ok(())
}

/// The scope of a parameter list: each name is a type that names itself and carries its
/// class bounds. Of several parameters with one name the last counts.
pub fn build_param_scope(params_opt: &Option<ast::GenericParams>) -> Scope {
    let mut scope = Scope::new();
    for param in params(params_opt) {
        let mut entity = Entity::new(EntityKind::Type, None, Some(param.name.clone()), EntitySource::Decl);
        entity.type_param_class_bounds = param.bounds.clone();
        scope.insert(id_key_of(&param.name), entity);
    }
    scope
}

/// The scopes with the parameters innermost; unchanged when there is no parameter list.
pub fn bind_type_params(ctx: &ScopeContext<'_>, params_opt: &Option<ast::GenericParams>) -> ScopeList {
    if params_opt.is_none() {
        return ctx.scopes.clone();
    }
    let mut scopes = ScopeList::with_capacity(ctx.scopes.len() + 1);
    scopes.push(build_param_scope(params_opt));
    scopes.extend(ctx.scopes.iter().cloned());
    scopes
}

/// The integer types.
pub fn is_valid_const_param_type(ty: &TypeRef) -> bool {
    const INTEGRAL: [&str; 12] =
        ["i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "isize", "usize"];
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if INTEGRAL.contains(&name.as_str()))
}

/// Parameters without a default: the fewest arguments a use may give.
pub fn required_param_count(params_opt: &Option<ast::GenericParams>) -> usize {
    params(params_opt).iter().filter(|param| param.default_type.is_none()).count()
}

pub fn total_param_count(params_opt: &Option<ast::GenericParams>) -> usize {
    params(params_opt).len()
}

pub fn has_default_params(params_opt: &Option<ast::GenericParams>) -> bool {
    params(params_opt).iter().any(|param| param.default_type.is_some())
}
