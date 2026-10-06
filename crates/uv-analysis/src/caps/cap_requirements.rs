//! Which capabilities a type carries: a capability class behind `$`, the context
//! bundle, or a nominal type with such a thing in its fields.
//!
//! The set of capability kinds a type carries, and the capability signature of a
//! callable: what its parameters require and what it provides to its callees.

use std::collections::HashSet;

use uv_source::ast;

use super::builtin_paths::{is_context_type_path, path_matches_builtin_name};
use crate::context::{ScopeContext, TypeDecl};
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::typing::signature::build_method_signature;
use crate::typing::types::{make_type_modal_state, make_type_path, TypeNode, TypeRef};

/// The declaration a type path names: as written, under the current module, under the
/// current assembly, or else the only declaration the path is a suffix of.
fn lookup_nominal_type_decl<'c>(ctx: &'c ScopeContext<'_>, current_module: &[String], path: &[String]) -> Option<(Vec<String>, &'c TypeDecl)> {
    if path.is_empty() {
        return None;
    }
    let lookup = |candidate: Vec<String>| ctx.sigma.types.get(&path_key_of(&candidate)).map(|decl| (candidate, decl));
    if let Some(found) = lookup(path.to_vec()) {
        return Some(found);
    }
    if let Some(found) = lookup(current_module.iter().chain(path).cloned().collect()) {
        return Some(found);
    }
    if let ([only], Some(root)) = (path, current_module.first()) {
        if let Some(found) = lookup(vec![root.clone(), only.clone()]) {
            return Some(found);
        }
    }
    let mut unique_match = None;
    for (key, decl) in &ctx.sigma.types {
        if key.len() < path.len() || !key[key.len() - path.len()..].iter().zip(path).all(|(lhs, rhs)| id_eq(lhs, rhs)) {
            continue;
        }
        if unique_match.is_some() {
            return None;
        }
        unique_match = Some((key.to_vec(), decl));
    }
    unique_match
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityKind {
    IO,
    Network,
    HeapAllocator,
    ExecutionDomain,
    Reactor,
    System,
    Time,
    MonotonicTime,
    WallTime,
}

pub fn capability_kind_name(kind: CapabilityKind) -> &'static str {
    match kind {
        CapabilityKind::IO => "IO",
        CapabilityKind::Network => "Network",
        CapabilityKind::HeapAllocator => "HeapAllocator",
        CapabilityKind::ExecutionDomain => "ExecutionDomain",
        CapabilityKind::Reactor => "Reactor",
        CapabilityKind::System => "System",
        CapabilityKind::Time => "Time",
        CapabilityKind::MonotonicTime => "MonotonicTime",
        CapabilityKind::WallTime => "WallTime",
    }
}

pub fn capability_kind_from_path<S: AsRef<str>>(path: &[S]) -> Option<CapabilityKind> {
    const KINDS: [(&str, CapabilityKind); 9] = [
        ("IO", CapabilityKind::IO),
        ("Network", CapabilityKind::Network),
        ("HeapAllocator", CapabilityKind::HeapAllocator),
        ("ExecutionDomain", CapabilityKind::ExecutionDomain),
        ("Reactor", CapabilityKind::Reactor),
        ("System", CapabilityKind::System),
        ("Time", CapabilityKind::Time),
        ("MonotonicTime", CapabilityKind::MonotonicTime),
        ("WallTime", CapabilityKind::WallTime),
    ];
    KINDS.iter().find(|(name, _)| path_matches_builtin_name(path, name)).map(|(_, kind)| *kind)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CapabilitySet {
    pub has_io: bool,
    pub has_network: bool,
    pub has_heap: bool,
    pub has_execution_domain: bool,
    pub has_reactor: bool,
    pub has_system: bool,
    pub has_time: bool,
    pub has_monotonic_time: bool,
    pub has_wall_time: bool,
}

impl CapabilitySet {
    pub fn from_context() -> Self {
        CapabilitySet { has_io: true, has_network: true, has_heap: true, has_execution_domain: true, has_reactor: true, has_system: true, has_time: true, ..Default::default() }
    }

    pub fn add(&mut self, kind: CapabilityKind) {
        match kind {
            CapabilityKind::IO => self.has_io = true,
            CapabilityKind::Network => self.has_network = true,
            CapabilityKind::HeapAllocator => self.has_heap = true,
            CapabilityKind::ExecutionDomain => self.has_execution_domain = true,
            CapabilityKind::Reactor => self.has_reactor = true,
            CapabilityKind::System => self.has_system = true,
            CapabilityKind::Time => self.has_time = true,
            CapabilityKind::MonotonicTime => self.has_monotonic_time = true,
            CapabilityKind::WallTime => self.has_wall_time = true,
        }
    }

    /// Holding `Time` covers the two clocks.
    pub fn has(&self, kind: CapabilityKind) -> bool {
        match kind {
            CapabilityKind::IO => self.has_io,
            CapabilityKind::Network => self.has_network,
            CapabilityKind::HeapAllocator => self.has_heap,
            CapabilityKind::ExecutionDomain => self.has_execution_domain,
            CapabilityKind::Reactor => self.has_reactor,
            CapabilityKind::System => self.has_system,
            CapabilityKind::Time => self.has_time,
            CapabilityKind::MonotonicTime => self.has_monotonic_time || self.has_time,
            CapabilityKind::WallTime => self.has_wall_time || self.has_time,
        }
    }

    fn flags(&self) -> [(CapabilityKind, bool); 9] {
        [
            (CapabilityKind::IO, self.has_io),
            (CapabilityKind::Network, self.has_network),
            (CapabilityKind::HeapAllocator, self.has_heap),
            (CapabilityKind::ExecutionDomain, self.has_execution_domain),
            (CapabilityKind::Reactor, self.has_reactor),
            (CapabilityKind::System, self.has_system),
            (CapabilityKind::Time, self.has_time),
            (CapabilityKind::MonotonicTime, self.has_monotonic_time),
            (CapabilityKind::WallTime, self.has_wall_time),
        ]
    }

    pub fn is_subset_of(&self, other: &CapabilitySet) -> bool {
        self.flags().iter().all(|(kind, held)| !held || other.has(*kind))
    }

    pub fn union(&self, other: &CapabilitySet) -> CapabilitySet {
        CapabilitySet {
            has_io: self.has_io || other.has_io,
            has_network: self.has_network || other.has_network,
            has_heap: self.has_heap || other.has_heap,
            has_execution_domain: self.has_execution_domain || other.has_execution_domain,
            has_reactor: self.has_reactor || other.has_reactor,
            has_system: self.has_system || other.has_system,
            has_time: self.has_time || other.has_time,
            has_monotonic_time: self.has_monotonic_time || other.has_monotonic_time,
            has_wall_time: self.has_wall_time || other.has_wall_time,
        }
    }

    pub fn intersection(&self, other: &CapabilitySet) -> CapabilitySet {
        CapabilitySet {
            has_io: self.has_io && other.has_io,
            has_network: self.has_network && other.has_network,
            has_heap: self.has_heap && other.has_heap,
            has_execution_domain: self.has_execution_domain && other.has_execution_domain,
            has_reactor: self.has_reactor && other.has_reactor,
            has_system: self.has_system && other.has_system,
            has_time: self.has_time && other.has_time,
            has_monotonic_time: self.has_monotonic_time && other.has_monotonic_time,
            has_wall_time: self.has_wall_time && other.has_wall_time,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == CapabilitySet::default()
    }
}

impl std::fmt::Display for CapabilitySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&str> = self.flags().iter().filter(|(_, held)| *held).map(|(kind, _)| capability_kind_name(*kind)).collect();
        write!(f, "{{{}}}", names.join(", "))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilitySignature {
    pub required: CapabilitySet,
    pub provided: CapabilitySet,
    pub capability_params: Vec<usize>,
}

struct Walk<'c, 'x> {
    ctx: Option<&'c ScopeContext<'x>>,
    visiting: HashSet<String>,
}

impl Walk<'_, '_> {
    fn nominal(&mut self, current_module: &[String], path: &[String]) -> CapabilitySet {
        let mut out = CapabilitySet::default();
        let Some(ctx) = self.ctx else {
            return out;
        };
        let Some((resolved, decl)) = lookup_nominal_type_decl(ctx, current_module, path) else {
            return out;
        };
        let visit_key = resolved.join("::");
        if !self.visiting.insert(visit_key.clone()) {
            return out;
        }
        let decl_module = &resolved[..resolved.len() - 1];
        match decl {
            TypeDecl::Record(node) => {
                for member in &node.members {
                    if let ast::RecordMember::FieldDecl(field) = member {
                        out = out.union(&self.ast_type(decl_module, &field.r#type));
                    }
                }
            }
            TypeDecl::Enum(node) => {
                for variant in &node.variants {
                    match &variant.payload_opt {
                        Some(ast::VariantPayload::VariantPayloadTuple(payload)) => {
                            for elem in &payload.elements {
                                out = out.union(&self.ast_type(decl_module, elem));
                            }
                        }
                        Some(ast::VariantPayload::VariantPayloadRecord(payload)) => {
                            for field in &payload.fields {
                                out = out.union(&self.ast_type(decl_module, &field.r#type));
                            }
                        }
                        None => {}
                    }
                }
            }
            TypeDecl::Modal(node) => {
                for member in node.states.iter().flat_map(|state| &state.members) {
                    if let ast::StateMember::StateFieldDecl(field) = member {
                        out = out.union(&self.ast_type(decl_module, &field.r#type));
                    }
                }
            }
            TypeDecl::TypeAlias(node) => out = self.ast_type(decl_module, &node.r#type),
        }
        self.visiting.remove(&visit_key);
        out
    }

    fn ast_type(&mut self, current_module: &[String], ty: &ast::TypePtr) -> CapabilitySet {
        use ast::TypeNode as T;
        let Some(t) = ty.as_deref() else {
            return CapabilitySet::default();
        };
        let many = |walk: &mut Self, types: &mut dyn Iterator<Item = &ast::TypePtr>| {
            types.fold(CapabilitySet::default(), |acc, ty| acc.union(&walk.ast_type(current_module, ty)))
        };
        match &t.node {
            T::TypeDynamic(node) => capability_kind_from_path(&node.path).map(single).unwrap_or_default(),
            T::TypePathType(node) => {
                let mut out = if is_context_type_path(&node.path) { CapabilitySet::from_context() } else { CapabilitySet::default() };
                out = out.union(&many(self, &mut node.generic_args.iter()));
                out.union(&self.nominal(current_module, &node.path))
            }
            T::TypeModalState(node) => {
                let out = many(self, &mut node.generic_args.iter());
                out.union(&self.nominal(current_module, &node.path))
            }
            T::TypePermType(node) => self.ast_type(current_module, &node.base),
            T::TypeRefine(node) => self.ast_type(current_module, &node.base),
            T::TypeUnion(node) => many(self, &mut node.types.iter()),
            T::TypeTuple(node) => many(self, &mut node.elements.iter()),
            T::TypeArray(node) => self.ast_type(current_module, &node.element),
            T::TypeSlice(node) => self.ast_type(current_module, &node.element),
            T::TypeSafePtr(node) => self.ast_type(current_module, &node.element),
            T::TypeRawPtr(node) => self.ast_type(current_module, &node.element),
            T::TypeFunc(node) => {
                let out = many(self, &mut node.params.iter().map(|param| &param.r#type));
                out.union(&self.ast_type(current_module, &node.ret))
            }
            T::TypeClosure(node) => {
                let mut out = many(self, &mut node.params.iter().map(|param| &param.r#type));
                out = out.union(&self.ast_type(current_module, &node.ret));
                if let Some(deps) = &node.deps_opt {
                    out = out.union(&many(self, &mut deps.iter().map(|dep| &dep.r#type)));
                }
                out
            }
            _ => CapabilitySet::default(),
        }
    }

    fn lowered(&mut self, current_module: &[String], ty: &TypeRef) -> CapabilitySet {
        let Some(t) = ty.as_deref() else {
            return CapabilitySet::default();
        };
        let many = |walk: &mut Self, types: &mut dyn Iterator<Item = &TypeRef>| {
            types.fold(CapabilitySet::default(), |acc, ty| acc.union(&walk.lowered(current_module, ty)))
        };
        match &t.node {
            TypeNode::Dynamic(path) => capability_kind_from_path(path).map(single).unwrap_or_default(),
            TypeNode::Path { path, generic_args } => {
                let mut out = if is_context_type_path(path) { CapabilitySet::from_context() } else { CapabilitySet::default() };
                out = out.union(&many(self, &mut generic_args.iter()));
                out.union(&self.nominal(current_module, path))
            }
            TypeNode::ModalState(modal) => {
                let out = many(self, &mut modal.generic_args.iter());
                out.union(&self.nominal(current_module, &modal.path))
            }
            TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => self.lowered(current_module, base),
            TypeNode::Union(members) => many(self, &mut members.iter()),
            TypeNode::Tuple(elements) => many(self, &mut elements.iter()),
            TypeNode::Array { element, .. } | TypeNode::Ptr { element, .. } | TypeNode::RawPtr { element, .. } => self.lowered(current_module, element),
            TypeNode::Slice(element) => self.lowered(current_module, element),
            TypeNode::Func { params, ret } => {
                let out = many(self, &mut params.iter().map(|param| &param.r#type));
                out.union(&self.lowered(current_module, ret))
            }
            TypeNode::Closure { params, ret, deps_opt } => {
                let mut out = many(self, &mut params.iter().map(|(_, ty)| ty));
                out = out.union(&self.lowered(current_module, ret));
                if let Some(deps) = deps_opt {
                    out = out.union(&many(self, &mut deps.iter().map(|dep| &dep.r#type)));
                }
                out
            }
            TypeNode::Range(base)
            | TypeNode::RangeInclusive(base)
            | TypeNode::RangeFrom(base)
            | TypeNode::RangeTo(base)
            | TypeNode::RangeToInclusive(base) => self.lowered(current_module, base),
            _ => CapabilitySet::default(),
        }
    }
}

fn single(kind: CapabilityKind) -> CapabilitySet {
    let mut set = CapabilitySet::default();
    set.add(kind);
    set
}

/// `InferCapabilitiesFromType`, with the declarations of the program to look into.
pub fn infer_capabilities_from_type(ctx: &ScopeContext<'_>, current_module: &[String], ty: &TypeRef) -> CapabilitySet {
    Walk { ctx: Some(ctx), visiting: HashSet::new() }.lowered(current_module, ty)
}

/// The same without a program: nominal types contribute nothing.
pub fn infer_capabilities_from_type_alone(ty: &TypeRef) -> CapabilitySet {
    Walk { ctx: None, visiting: HashSet::new() }.lowered(&[], ty)
}

pub fn infer_capabilities_from_ast_type(ctx: &ScopeContext<'_>, current_module: &[String], ty: &ast::TypePtr) -> CapabilitySet {
    Walk { ctx: Some(ctx), visiting: HashSet::new() }.ast_type(current_module, ty)
}

pub fn infer_capabilities_from_ast_type_alone(ty: &ast::TypePtr) -> CapabilitySet {
    Walk { ctx: None, visiting: HashSet::new() }.ast_type(&[], ty)
}

/// Whether `InferCapabilitiesFromType` would find any capability in the type.
pub fn type_has_capabilities(ctx: &ScopeContext<'_>, current_module: &[String], ty: &TypeRef) -> bool {
    !infer_capabilities_from_type(ctx, current_module, ty).is_empty()
}

fn signature_from_sets(sets: impl Iterator<Item = (usize, CapabilitySet)>) -> CapabilitySignature {
    let mut sig = CapabilitySignature::default();
    for (index, caps) in sets {
        if caps.is_empty() {
            continue;
        }
        sig.required = sig.required.union(&caps);
        sig.capability_params.push(index);
    }
    sig.provided = sig.required;
    sig
}

fn signature_from_bindings(ctx: &ScopeContext<'_>, current_module: &[String], bindings: &[(String, TypeRef)]) -> CapabilitySignature {
    signature_from_sets(bindings.iter().enumerate().filter(|(_, (_, ty))| ty.is_some()).map(|(i, (_, ty))| (i, infer_capabilities_from_type(ctx, current_module, ty))))
}

/// What `self` and the parameters require, read from the written types.
fn signature_from_receiver_and_params(self_type: &TypeRef, receiver: &ast::Receiver, params: &[ast::Param]) -> CapabilitySignature {
    let mut sets: Vec<(usize, CapabilitySet)> = Vec::new();
    let mut index = 0;
    match receiver {
        ast::Receiver::ReceiverShorthand(_) => {
            sets.push((index, infer_capabilities_from_type_alone(self_type)));
            index += 1;
        }
        ast::Receiver::ReceiverExplicit(explicit) => {
            if explicit.r#type.is_some() {
                sets.push((index, infer_capabilities_from_ast_type_alone(&explicit.r#type)));
                index += 1;
            }
        }
    }
    for param in params {
        if param.r#type.is_some() {
            sets.push((index, infer_capabilities_from_ast_type_alone(&param.r#type)));
        }
        index += 1;
    }
    signature_from_sets(sets.into_iter())
}

fn method_signature(
    ctx: &ScopeContext<'_>,
    current_module: &[String],
    self_type: &TypeRef,
    receiver: &ast::Receiver,
    params: &[ast::Param],
    return_type_opt: &ast::TypePtr,
) -> CapabilitySignature {
    match build_method_signature(ctx, self_type, receiver, params, return_type_opt, None) {
        Ok(sig) => signature_from_bindings(ctx, current_module, &sig.bindings),
        Err(_) => signature_from_receiver_and_params(self_type, receiver, params),
    }
}

fn nominal_type_path(current_module: &[String], name: &str) -> Vec<String> {
    current_module.iter().cloned().chain([name.to_string()]).collect()
}

pub fn build_procedure_capability_signature(ctx: &ScopeContext<'_>, current_module: &[String], proc: &ast::ProcedureDecl) -> CapabilitySignature {
    signature_from_sets(proc.params.iter().enumerate().filter(|(_, param)| param.r#type.is_some()).map(|(i, param)| (i, infer_capabilities_from_ast_type(ctx, current_module, &param.r#type))))
}

pub fn build_record_method_capability_signature(ctx: &ScopeContext<'_>, current_module: &[String], record_name: &str, method: &ast::MethodDecl) -> CapabilitySignature {
    let self_type = make_type_path(nominal_type_path(current_module, record_name));
    method_signature(ctx, current_module, &self_type, &method.receiver, &method.params, &method.return_type_opt)
}

pub fn build_class_method_capability_signature(ctx: &ScopeContext<'_>, current_module: &[String], class_name: &str, method: &ast::ClassMethodDecl) -> CapabilitySignature {
    let self_type = make_type_path(nominal_type_path(current_module, class_name));
    method_signature(ctx, current_module, &self_type, &method.receiver, &method.params, &method.return_type_opt)
}

pub fn build_state_method_capability_signature(
    ctx: &ScopeContext<'_>,
    current_module: &[String],
    modal_name: &str,
    state_name: &str,
    method: &ast::StateMethodDecl,
) -> CapabilitySignature {
    let self_type = make_type_modal_state(nominal_type_path(current_module, modal_name), state_name, Vec::new());
    method_signature(ctx, current_module, &self_type, &method.receiver, &method.params, &method.return_type_opt)
}

pub fn build_transition_capability_signature(
    ctx: &ScopeContext<'_>,
    current_module: &[String],
    modal_name: &str,
    state_name: &str,
    transition: &ast::TransitionDecl,
) -> CapabilitySignature {
    let self_type = make_type_modal_state(nominal_type_path(current_module, modal_name), state_name, Vec::new());
    let receiver = ast::Receiver::ReceiverShorthand(ast::ReceiverShorthand { perm: ast::ReceiverPerm::Unique, ..Default::default() });
    method_signature(ctx, current_module, &self_type, &receiver, &transition.params, &None)
}

pub struct CapabilityValidation {
    pub valid: bool,
    pub error_code: &'static str,
    pub error_message: String,
    pub missing: CapabilitySet,
}

/// Whether what a call needs is held, and when it is not, what is missing.
pub fn validate_capability_satisfied(provided: &CapabilitySet, required: &CapabilitySet) -> CapabilityValidation {
    if required.is_subset_of(provided) {
        return CapabilityValidation { valid: true, error_code: "", error_message: String::new(), missing: CapabilitySet::default() };
    }
    let mut missing = CapabilitySet::default();
    for (kind, needed) in required.flags() {
        if needed && !provided.has(kind) {
            missing.add(kind);
        }
    }
    CapabilityValidation {
        valid: false,
        error_code: "E-CON-0020",
        error_message: format!("Missing required capability: {missing}; provided: {provided}"),
        missing,
    }
}

/// The capability a method call on a capability value uses, when the method is one of its own.
pub fn method_call_requires_capability(receiver_type: &TypeRef, method_name: &str) -> Option<CapabilityKind> {
    use super::cap_methods::*;
    match &receiver_type.as_deref()?.node {
        TypeNode::Dynamic(path) => {
            let named = |name: &str| path_matches_builtin_name(path, name);
            if named("IO") && lookup_io_method_sig(method_name).is_some() {
                return Some(CapabilityKind::IO);
            }
            if named("Network") && lookup_network_method_sig(method_name).is_some() {
                return Some(CapabilityKind::Network);
            }
            if named("HeapAllocator") && lookup_heap_allocator_method_sig(method_name).is_some() {
                return Some(CapabilityKind::HeapAllocator);
            }
            if named("Time") && lookup_time_method_sig(method_name).is_some() {
                return Some(CapabilityKind::Time);
            }
            if named("MonotonicTime") && lookup_monotonic_time_method_sig(method_name).is_some() {
                return Some(CapabilityKind::MonotonicTime);
            }
            if named("WallTime") && lookup_wall_time_method_sig(method_name).is_some() {
                return Some(CapabilityKind::WallTime);
            }
            if named("System") && lookup_system_method_sig(method_name).is_some() {
                return Some(CapabilityKind::System);
            }
            if named("ExecutionDomain") && lookup_execution_domain_method_sig(method_name).is_some() {
                return Some(CapabilityKind::ExecutionDomain);
            }
            None
        }
        TypeNode::Path { path, .. } if is_context_type_path(path) && lookup_context_method_sig(method_name, None).is_some() => Some(CapabilityKind::ExecutionDomain),
        _ => None,
    }
}

fn stmt_uses_capabilities(stmt: &ast::Stmt) -> CapabilitySet {
    use ast::Stmt as S;
    let expr = |e: &ast::ExprPtr| e.as_deref().map(expression_uses_capabilities).unwrap_or_default();
    let block = |b: &Option<std::sync::Arc<ast::Block>>| b.as_deref().map(block_uses_capabilities).unwrap_or_default();
    match stmt {
        S::LetStmt(node) => expr(&node.binding.init),
        S::VarStmt(node) => expr(&node.binding.init),
        S::ExprStmt(node) => expr(&node.value),
        S::AssignStmt(node) => expr(&node.value),
        S::ReturnStmt(node) => expr(&node.value_opt),
        S::DeferStmt(node) => block(&node.body),
        S::UnsafeBlockStmt(node) => block(&node.body),
        S::RegionStmt(node) => block(&node.body),
        S::FrameStmt(node) => block(&node.body),
        S::KeyBlockStmt(node) => block(&node.body),
        _ => CapabilitySet::default(),
    }
}

pub fn block_uses_capabilities(block: &ast::Block) -> CapabilitySet {
    let mut caps = block.stmts.iter().fold(CapabilitySet::default(), |acc, stmt| acc.union(&stmt_uses_capabilities(stmt)));
    if let Some(tail) = block.tail_opt.as_deref() {
        caps = caps.union(&expression_uses_capabilities(tail));
    }
    caps
}

/// The capabilities an expression uses by itself: a `parallel`, `spawn` or `dispatch`
/// uses the execution domain, and the rest is the union over what it contains.
pub fn expression_uses_capabilities(expr: &ast::Expr) -> CapabilitySet {
    use ast::ExprNode as E;
    let one = |e: &ast::ExprPtr| e.as_deref().map(expression_uses_capabilities).unwrap_or_default();
    let args = |args: &[ast::Arg]| args.iter().fold(CapabilitySet::default(), |acc, arg| acc.union(&one(&arg.value)));
    let block = |b: &Option<std::sync::Arc<ast::Block>>| b.as_deref().map(block_uses_capabilities).unwrap_or_default();
    let domain = |b: &Option<std::sync::Arc<ast::Block>>| single(CapabilityKind::ExecutionDomain).union(&block(b));
    match &expr.node {
        E::CallExpr(node) => one(&node.callee).union(&args(&node.args)),
        E::QualifiedApplyExpr(node) => match &node.args {
            ast::ApplyArgs::ParenArgs(paren) => args(&paren.args),
            ast::ApplyArgs::BraceArgs(brace) => brace.fields.iter().fold(CapabilitySet::default(), |acc, field| acc.union(&one(&field.value))),
        },
        E::MethodCallExpr(node) => one(&node.receiver).union(&args(&node.args)),
        E::FieldAccessExpr(node) => one(&node.base),
        E::IndexAccessExpr(node) => one(&node.base).union(&one(&node.index)),
        E::BlockExpr(node) => block(&node.block),
        E::IfExpr(node) => one(&node.cond).union(&one(&node.then_expr)).union(&one(&node.else_expr)),
        E::IfCaseExpr(node) => {
            let arms = node.cases.iter().fold(CapabilitySet::default(), |acc, arm| acc.union(&one(&arm.body)));
            one(&node.scrutinee).union(&arms).union(&one(&node.else_expr))
        }
        E::IfIsExpr(node) => one(&node.scrutinee).union(&one(&node.then_expr)).union(&one(&node.else_expr)),
        E::LoopInfiniteExpr(node) => block(&node.body),
        E::LoopConditionalExpr(node) => one(&node.cond).union(&block(&node.body)),
        E::LoopIterExpr(node) => one(&node.iter).union(&block(&node.body)),
        E::BinaryExpr(node) => one(&node.lhs).union(&one(&node.rhs)),
        E::UnaryExpr(node) => one(&node.value),
        E::ParallelExpr(node) => domain(&node.body),
        E::SpawnExpr(node) => domain(&node.body),
        E::DispatchExpr(node) => domain(&node.body),
        _ => CapabilitySet::default(),
    }
}
