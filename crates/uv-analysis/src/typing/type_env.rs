//! The typing environment: the bindings in scope while a body is typed.

use std::collections::HashSet;
use std::sync::Arc;

use uv_core::std_unordered::UnorderedMap;
use uv_source::ast;

use super::subtyping::subtyping;
use super::type_equiv::type_equiv;
use super::type_lower::lower_type;
use super::types::*;
use crate::context::{IdKey, ScopeContext, TypeDecl};
use crate::memory::regions::ProvenanceKind;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of, reserved_gen};

/// Where a binding's storage comes from, as far as typing cares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindingProvenanceSeedKind {
    Global,
    #[default]
    Stack,
    Heap,
    Region,
    Bottom,
    Param,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParallelContextKind {
    Cpu,
    Gpu,
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClosureCaptureInfo {
    pub captures_any: bool,
    pub captures_shared: bool,
    pub has_shared_deps: bool,
    pub contains_spawn: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TypeBinding {
    pub r#mut: ast::Mutability,
    pub r#type: TypeRef,
    /// The declared type, when narrowing has changed `type`.
    pub storage_type: TypeRef,
    pub closure_capture_info: Option<ClosureCaptureInfo>,
    pub deprecated: bool,
    pub deprecated_message: Option<String>,
    pub derived_from_shared: bool,
    pub stale_ok: bool,
    pub stale_after_release: bool,
    pub parallel_context_kind: Option<ParallelContextKind>,
    pub provenance_kind: BindingProvenanceSeedKind,
    pub provenance_region: Option<IdKey>,
}

/// One scope. It iterates in the order of the reference's hash table.
pub type TypeScope = UnorderedMap<TypeBinding>;

/// Scopes from outermost to innermost.
#[derive(Debug, Clone, Default)]
pub struct TypeEnv {
    pub scopes: Vec<TypeScope>,
    pub parallel_context: Option<ParallelContextKind>,
}

/// `Bottom` counts as the stack.
pub fn normalize_binding_provenance_seed(kind: ProvenanceKind) -> BindingProvenanceSeedKind {
    match kind {
        ProvenanceKind::Global => BindingProvenanceSeedKind::Global,
        ProvenanceKind::Stack | ProvenanceKind::Bottom => BindingProvenanceSeedKind::Stack,
        ProvenanceKind::Heap => BindingProvenanceSeedKind::Heap,
        ProvenanceKind::Region => BindingProvenanceSeedKind::Region,
        ProvenanceKind::Param => BindingProvenanceSeedKind::Param,
    }
}

pub fn apply_binding_provenance_seed(binding: &mut TypeBinding, kind: ProvenanceKind, region: Option<&str>) {
    binding.provenance_kind = normalize_binding_provenance_seed(kind);
    binding.provenance_region = region.filter(|_| kind == ProvenanceKind::Region).map(id_key_of);
}

pub fn push_scope(env: &TypeEnv) -> TypeEnv {
    let mut out = env.clone();
    out.scopes.push(TypeScope::new());
    out
}

pub fn pop_scope(env: &TypeEnv) -> TypeEnv {
    let mut out = env.clone();
    out.scopes.pop();
    out
}

/// The environment without the scopes beyond the given depth.
pub fn project_type_env_to_depth(env: &TypeEnv, depth: usize) -> TypeEnv {
    let mut out = env.clone();
    out.scopes.truncate(depth);
    out
}

/// The innermost binding of the name.
pub fn bind_of<'e>(env: &'e TypeEnv, name: &str) -> Option<&'e TypeBinding> {
    let key = id_key_of(name);
    env.scopes.iter().rev().find_map(|scope| scope.get(&key))
}

pub fn stable_binding_type(binding: &TypeBinding) -> &TypeRef {
    if binding.storage_type.is_some() {
        &binding.storage_type
    } else {
        &binding.r#type
    }
}

/// After keys are released, bindings derived from shared data may be out of date.
pub fn mark_shared_derived_bindings_stale(env: &mut TypeEnv) {
    for scope in &mut env.scopes {
        let keys: Vec<String> = scope.keys().cloned().collect();
        for key in keys {
            if let Some(binding) = scope.get_mut(&key).filter(|binding| binding.derived_from_shared) {
                binding.stale_after_release = true;
            }
        }
    }
}

pub fn mut_of(env: &TypeEnv, name: &str) -> Option<ast::Mutability> {
    bind_of(env, name).map(|binding| binding.r#mut)
}

pub fn gpu_context(env: &TypeEnv) -> bool {
    env.parallel_context == Some(ParallelContextKind::Gpu)
}

pub fn has_heap_provenance(env: &TypeEnv, name: &str) -> bool {
    bind_of(env, name).is_some_and(|binding| binding.provenance_kind == BindingProvenanceSeedKind::Heap)
}

fn collect_field_pat_names(field: &ast::FieldPattern, out: &mut Vec<IdKey>) {
    match field.pattern_opt.as_deref() {
        Some(pattern) => collect_pat_names(pattern, out),
        None => out.push(id_key_of(&field.name)),
    }
}

/// The names a pattern binds, in order. A typed pattern named `_` binds nothing; an
/// identifier pattern binds whatever it is called.
pub fn collect_pat_names(pat: &ast::Pattern, out: &mut Vec<IdKey>) {
    use ast::PatternNode as P;
    let each = |patterns: &[ast::PatternPtr], out: &mut Vec<IdKey>| {
        for pattern in patterns.iter().flatten() {
            collect_pat_names(pattern, out);
        }
    };
    match &pat.node {
        P::IdentifierPattern(node) => out.push(id_key_of(&node.name)),
        P::TypedPattern(node) if node.name != "_" => out.push(id_key_of(&node.name)),
        P::TuplePattern(node) => each(&node.elements, out),
        P::RecordPattern(node) => node.fields.iter().for_each(|field| collect_field_pat_names(field, out)),
        P::EnumPattern(node) => match &node.payload_opt {
            Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => each(&payload.elements, out),
            Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                payload.fields.iter().for_each(|field| collect_field_pat_names(field, out));
            }
            None => {}
        },
        P::ModalPattern(node) => {
            node.fields_opt.iter().flat_map(|payload| &payload.fields).for_each(|field| collect_field_pat_names(field, out));
        }
        P::RangePattern(node) => each(&[node.lo.clone(), node.hi.clone()], out),
        _ => {}
    }
}

pub fn distinct_names(names: &[IdKey]) -> bool {
    let mut seen = HashSet::new();
    names.iter().all(|name| seen.insert(name))
}

fn in_scope(env: &TypeEnv, key: &str) -> bool {
    env.scopes.last().is_some_and(|scope| scope.contains_key(key))
}

fn in_outer(env: &TypeEnv, key: &str) -> bool {
    env.scopes.split_last().is_some_and(|(_, outer)| outer.iter().any(|scope| scope.contains_key(key)))
}

/// Why a binding could not be introduced; some failures name no rule.
pub type IntroError = Option<&'static str>;

fn intro_binding(env: &mut TypeEnv, name: &str, binding: TypeBinding, shadow: bool) -> Result<(), IntroError> {
    if reserved_gen(name) {
        return Err(Some(if shadow { "Shadow-Reserved-Gen-Err" } else { "Intro-Reserved-Gen-Err" }));
    }
    let key = id_key_of(name);
    if env.scopes.is_empty() || in_scope(env, &key) {
        return Err(None);
    }
    match (shadow, in_outer(env, &key)) {
        (false, true) => return Err(Some("Intro-Outer-Err")),
        (true, false) => return Err(Some("Shadow-Unnecessary")),
        _ => {}
    }
    env.scopes.last_mut().expect("a scope").emplace(key, binding);
    Ok(())
}

/// Introduces the bindings into the innermost scope, in order. A plain introduction may
/// not reuse a name of an outer scope; a shadowing one must.
pub fn intro_all(env: &TypeEnv, binds: &[(String, TypeRef)], mutability: ast::Mutability, shadow: bool) -> Result<TypeEnv, IntroError> {
    let mut current = env.clone();
    for (name, ty) in binds {
        let binding = TypeBinding { r#mut: mutability, r#type: ty.clone(), storage_type: ty.clone(), ..Default::default() };
        intro_binding(&mut current, name, binding, shadow)?;
    }
    Ok(current)
}

/// The names a binding pattern introduces with their types. Only irrefutable forms are
/// allowed in a binding: wildcards, names, typed names, tuples and records.
pub fn type_pattern(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> Result<Vec<(String, TypeRef)>, Option<&'static str>> {
    let (Some(pattern), Some(expected_ty)) = (pattern.as_deref(), expected.as_deref()) else {
        return Err(None);
    };
    use ast::PatternNode as P;
    match &pattern.node {
        P::WildcardPattern(_) => Ok(Vec::new()),
        P::IdentifierPattern(node) => Ok(vec![(node.name.clone(), expected.clone())]),
        P::TypedPattern(node) => {
            let lowered = lower_type(ctx, &node.r#type)?;
            if !type_equiv(&lowered, expected) {
                let sub = subtyping(ctx, &lowered, expected);
                if !sub.ok || !sub.subtype {
                    return Err(Some("Pat-Typed-Err"));
                }
            }
            Ok(if node.name == "_" { Vec::new() } else { vec![(node.name.clone(), lowered)] })
        }
        P::TuplePattern(node) => {
            if node.elements.is_empty() {
                let unit = matches!(&expected_ty.node, TypeNode::Prim(name) if name == "()");
                return if unit { Ok(Vec::new()) } else { Err(None) };
            }
            let TypeNode::Tuple(elements) = &expected_ty.node else {
                return Err(None);
            };
            if elements.len() != node.elements.len() {
                return Err(Some("E-TYP-1803"));
            }
            let mut binds = Vec::new();
            for (pattern, ty) in node.elements.iter().zip(elements) {
                binds.extend(type_pattern(ctx, pattern, ty)?);
            }
            Ok(binds)
        }
        // The record is the one the pattern names; the matched type only has to be
        // nominal, and field types are taken as declared.
        P::RecordPattern(node) => {
            if !matches!(&expected_ty.node, TypeNode::Path { .. }) {
                return Err(None);
            }
            let Some(TypeDecl::Record(record)) = ctx.sigma.types.get(&path_key_of(&node.path)) else {
                return Err(None);
            };
            let mut binds = Vec::new();
            for field in &node.fields {
                let field_decl = record
                    .members
                    .iter()
                    .find_map(|member| match member {
                        ast::RecordMember::FieldDecl(decl) if id_eq(&decl.name, &field.name) => Some(decl),
                        _ => None,
                    })
                    .ok_or(Some("RecordPattern-UnknownField"))?;
                let field_type = lower_type(ctx, &field_decl.r#type)?;
                let pattern = if field.pattern_opt.is_some() {
                    field.pattern_opt.clone()
                } else {
                    let name = ast::IdentifierPattern { name: field.name.clone(), name_splice_opt: None };
                    Some(Arc::new(ast::Pattern { span: Default::default(), node: P::IdentifierPattern(name) }))
                };
                binds.extend(type_pattern(ctx, &pattern, &field_type)?);
            }
            Ok(binds)
        }
        _ => Err(Some("Let-Refutable-Pattern-Err")),
    }
}
