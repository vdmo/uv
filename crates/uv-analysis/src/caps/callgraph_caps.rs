//! The call graph of the program, with the capabilities each callable requires and
//! provides, and the validation that every call is made by a caller that holds what its
//! callee asks for. See `callgraph_caps.cpp`.
//!
//! The reference keeps its nodes in a hash table and visits them in its order; the port
//! visits them in the order they were added. The diagnostics are put in source order by
//! the driver before they are shown, so only the relative order of two reports at the
//! same place could differ.

use std::collections::HashMap;
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ApplyArgs, ASTItem, ASTModule, ExprNode as E, Stmt};

use super::cap_requirements::*;
use crate::context::ScopeContext;
use crate::resolve::scopes::id_eq;
use crate::typing::expr_store::TypeStores;
use crate::typing::types::{TypeNode, TypeRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcId {
    pub module_path: String,
    pub name: String,
}

pub struct CallGraphNode {
    pub id: ProcId,
    pub is_extern: bool,
    pub cap_sig: CapabilitySignature,
    pub span: Span,
}

pub struct CallGraphEdge {
    pub caller: ProcId,
    pub callee: ProcId,
    pub capabilities_passed: CapabilitySet,
    pub call_span: Span,
}

pub struct UnresolvedDirectCall<'m> {
    pub caller: ProcId,
    pub callee_expr: &'m ast::Expr,
    pub call_span: Span,
}

pub struct UnresolvedMethodCall<'m> {
    pub caller: ProcId,
    pub receiver_expr: &'m ast::Expr,
    pub arg_exprs: Vec<&'m ast::Expr>,
    pub method_name: String,
    pub call_span: Span,
}

pub struct CapabilityLeak {
    pub code: &'static str,
    pub capability: CapabilityKind,
    pub sink: ProcId,
    pub leak_span: Span,
    pub message: String,
}

pub struct CapabilityChainError {
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
}

pub struct CapabilityChainResult {
    pub valid: bool,
    pub errors: Vec<CapabilityChainError>,
    pub leaks: Vec<CapabilityLeak>,
}

#[derive(Default)]
pub struct CallGraph<'m> {
    nodes: Vec<CallGraphNode>,
    index: HashMap<ProcId, usize>,
    edges: Vec<CallGraphEdge>,
    unresolved_direct_calls: Vec<UnresolvedDirectCall<'m>>,
    unresolved_method_calls: Vec<UnresolvedMethodCall<'m>>,
}

impl CallGraph<'_> {
    fn add_node(&mut self, node: CallGraphNode) {
        match self.index.get(&node.id) {
            Some(&at) => self.nodes[at] = node,
            None => {
                self.index.insert(node.id.clone(), self.nodes.len());
                self.nodes.push(node);
            }
        }
    }

    pub fn node(&self, id: &ProcId) -> Option<&CallGraphNode> {
        self.index.get(id).map(|&at| &self.nodes[at])
    }

    fn has_node(&self, id: &ProcId) -> bool {
        self.index.contains_key(id)
    }

    fn incoming<'a>(&'a self, callee: &'a ProcId) -> impl Iterator<Item = &'a CallGraphEdge> {
        self.edges.iter().filter(move |edge| edge.callee == *callee)
    }
}

fn module_path_to_string(path: &[String]) -> String {
    path.join("::")
}

fn module_path_from_string(path: &str) -> Vec<String> {
    if path.is_empty() {
        return Vec::new();
    }
    path.split("::").map(str::to_string).collect()
}

struct Target<'m> {
    module_path: Option<String>,
    name: String,
    call_span: Span,
    callee_expr: Option<&'m ast::Expr>,
}

#[derive(Default)]
struct Collected<'m> {
    targets: Vec<Target<'m>>,
    unresolved_calls: Vec<(&'m ast::Expr, Span)>,
    unresolved_method_calls: Vec<UnresolvedMethodCall<'m>>,
}

impl<'m> Collected<'m> {
    fn apply_args(&mut self, args: &'m ApplyArgs) {
        match args {
            ApplyArgs::ParenArgs(paren) => paren.args.iter().for_each(|arg| self.expr_opt(&arg.value)),
            ApplyArgs::BraceArgs(brace) => brace.fields.iter().for_each(|field| self.expr_opt(&field.value)),
        }
    }

    fn expr_opt(&mut self, expr: &'m Option<Arc<ast::Expr>>) {
        if let Some(expr) = expr.as_deref() {
            self.expr(expr);
        }
    }

    fn block_opt(&mut self, block: &'m Option<Arc<ast::Block>>) {
        if let Some(block) = block.as_deref() {
            self.block(block);
        }
    }

    fn expr(&mut self, expr: &'m ast::Expr) {
        match &expr.node {
            E::CallExpr(node) => {
                if let Some(callee) = node.callee.as_deref() {
                    match &callee.node {
                        E::PathExpr(path) => self.targets.push(Target {
                            module_path: (!path.path.is_empty()).then(|| module_path_to_string(&path.path)),
                            name: path.name.clone(),
                            call_span: expr.span.clone(),
                            callee_expr: Some(callee),
                        }),
                        E::QualifiedNameExpr(qname) => self.targets.push(Target {
                            module_path: (!qname.path.is_empty()).then(|| module_path_to_string(&qname.path)),
                            name: qname.name.clone(),
                            call_span: expr.span.clone(),
                            callee_expr: Some(callee),
                        }),
                        E::IdentifierExpr(ident) => {
                            self.targets.push(Target { module_path: None, name: ident.name.clone(), call_span: expr.span.clone(), callee_expr: Some(callee) })
                        }
                        _ => {
                            self.unresolved_calls.push((callee, expr.span.clone()));
                            self.expr(callee);
                        }
                    }
                }
                node.args.iter().for_each(|arg| self.expr_opt(&arg.value));
            }
            E::QualifiedApplyExpr(node) => {
                self.targets.push(Target {
                    module_path: (!node.path.is_empty()).then(|| module_path_to_string(&node.path)),
                    name: node.name.clone(),
                    call_span: expr.span.clone(),
                    callee_expr: None,
                });
                self.apply_args(&node.args);
            }
            E::MethodCallExpr(node) => {
                if let Some(receiver) = node.receiver.as_deref() {
                    self.unresolved_method_calls.push(UnresolvedMethodCall {
                        caller: ProcId { module_path: String::new(), name: String::new() },
                        receiver_expr: receiver,
                        arg_exprs: node.args.iter().filter_map(|arg| arg.value.as_deref()).collect(),
                        method_name: node.name.clone(),
                        call_span: expr.span.clone(),
                    });
                }
                self.expr_opt(&node.receiver);
                node.args.iter().for_each(|arg| self.expr_opt(&arg.value));
            }
            E::BlockExpr(node) => self.block_opt(&node.block),
            E::IfExpr(node) => {
                self.expr_opt(&node.cond);
                self.expr_opt(&node.then_expr);
                self.expr_opt(&node.else_expr);
            }
            E::IfCaseExpr(node) => {
                self.expr_opt(&node.scrutinee);
                node.cases.iter().for_each(|arm| self.expr_opt(&arm.body));
                self.expr_opt(&node.else_expr);
            }
            E::IfIsExpr(node) => {
                self.expr_opt(&node.scrutinee);
                self.expr_opt(&node.then_expr);
                self.expr_opt(&node.else_expr);
            }
            E::LoopInfiniteExpr(node) => self.block_opt(&node.body),
            E::LoopConditionalExpr(node) => {
                self.expr_opt(&node.cond);
                self.block_opt(&node.body);
            }
            E::LoopIterExpr(node) => {
                self.expr_opt(&node.iter);
                self.block_opt(&node.body);
            }
            E::BinaryExpr(node) => {
                self.expr_opt(&node.lhs);
                self.expr_opt(&node.rhs);
            }
            E::UnaryExpr(node) => self.expr_opt(&node.value),
            E::ParallelExpr(node) => self.block_opt(&node.body),
            E::SpawnExpr(node) => self.block_opt(&node.body),
            E::DispatchExpr(node) => self.block_opt(&node.body),
            _ => {}
        }
    }

    fn stmt(&mut self, stmt: &'m Stmt) {
        match stmt {
            Stmt::LetStmt(node) => self.expr_opt(&node.binding.init),
            Stmt::VarStmt(node) => self.expr_opt(&node.binding.init),
            Stmt::ExprStmt(node) => self.expr_opt(&node.value),
            Stmt::AssignStmt(node) => self.expr_opt(&node.value),
            Stmt::ReturnStmt(node) => self.expr_opt(&node.value_opt),
            Stmt::DeferStmt(node) => self.block_opt(&node.body),
            Stmt::UnsafeBlockStmt(node) => self.block_opt(&node.body),
            Stmt::RegionStmt(node) => self.block_opt(&node.body),
            Stmt::FrameStmt(node) => self.block_opt(&node.body),
            Stmt::KeyBlockStmt(node) => self.block_opt(&node.body),
            _ => {}
        }
    }

    fn block(&mut self, block: &'m ast::Block) {
        block.stmts.iter().for_each(|stmt| self.stmt(stmt));
        self.expr_opt(&block.tail_opt);
    }
}

/// The callee a call names: qualified, in the caller's module, or else the one module of
/// the program that has a callable of that name.
fn resolve_callee(target: &Target<'_>, caller_module_path: &str, modules: &[&ASTModule], cg: &CallGraph<'_>) -> Option<ProcId> {
    if let Some(module_path) = &target.module_path {
        let qualified = ProcId { module_path: module_path.clone(), name: target.name.clone() };
        return cg.has_node(&qualified).then_some(qualified);
    }
    let same_module = ProcId { module_path: caller_module_path.to_string(), name: target.name.clone() };
    if cg.has_node(&same_module) {
        return Some(same_module);
    }
    let mut callee: Option<ProcId> = None;
    for other in modules {
        let candidate = ProcId { module_path: module_path_to_string(&other.path), name: target.name.clone() };
        if !cg.has_node(&candidate) {
            continue;
        }
        match &callee {
            None => callee = Some(candidate),
            Some(found) if *found != candidate => return None,
            Some(_) => {}
        }
    }
    callee
}

fn add_calls_from_body<'m>(cg: &mut CallGraph<'m>, caller: ProcId, caller_module_path: &str, body: &'m ast::Block, modules: &[&ASTModule]) {
    let mut collected = Collected::default();
    collected.block(body);
    for target in &collected.targets {
        match resolve_callee(target, caller_module_path, modules, cg) {
            Some(callee) => cg.edges.push(CallGraphEdge { caller: caller.clone(), callee, capabilities_passed: CapabilitySet::default(), call_span: target.call_span.clone() }),
            None => {
                if let Some(callee_expr) = target.callee_expr {
                    cg.unresolved_direct_calls.push(UnresolvedDirectCall { caller: caller.clone(), callee_expr, call_span: target.call_span.clone() });
                }
            }
        }
    }
    for (callee_expr, call_span) in collected.unresolved_calls {
        cg.unresolved_direct_calls.push(UnresolvedDirectCall { caller: caller.clone(), callee_expr, call_span });
    }
    for mut call in collected.unresolved_method_calls {
        call.caller = caller.clone();
        cg.unresolved_method_calls.push(call);
    }
}

fn record_method_node_name(owner: &str, method: &str) -> String {
    format!("{owner}::{method}")
}

fn state_method_node_name(modal: &str, state: &str, method: &str) -> String {
    format!("{modal}@{state}::{method}")
}

fn transition_node_name(modal: &str, state: &str, transition: &str) -> String {
    format!("{modal}@{state}->{transition}")
}

fn callable(module_path: &str, name: String, cap_sig: CapabilitySignature, span: &Span) -> CallGraphNode {
    CallGraphNode { id: ProcId { module_path: module_path.to_string(), name }, is_extern: false, cap_sig, span: span.clone() }
}

/// The graph of every callable of the modules, with an edge for each call whose callee it can name.
pub fn build_call_graph<'m>(ctx: &ScopeContext<'_>, modules: &[&'m ASTModule]) -> CallGraph<'m> {
    let mut cg = CallGraph::default();
    for module in modules {
        let module_path = module_path_to_string(&module.path);
        for item in &module.items {
            match item {
                ASTItem::ProcedureDecl(node) => {
                    cg.add_node(callable(&module_path, node.name.clone(), build_procedure_capability_signature(ctx, &module.path, node), &node.span))
                }
                ASTItem::ExternBlock(node) => {
                    for ast::ExternItem::ExternProcDecl(proc) in &node.items {
                        cg.add_node(CallGraphNode {
                            id: ProcId { module_path: module_path.clone(), name: proc.name.clone() },
                            is_extern: true,
                            cap_sig: CapabilitySignature::default(),
                            span: proc.span.clone(),
                        });
                    }
                }
                ASTItem::RecordDecl(node) => {
                    for member in &node.members {
                        if let ast::RecordMember::MethodDecl(method) = member {
                            let sig = build_record_method_capability_signature(ctx, &module.path, &node.name, method);
                            cg.add_node(callable(&module_path, record_method_node_name(&node.name, &method.name), sig, &method.span));
                        }
                    }
                }
                ASTItem::ClassDecl(node) => {
                    for class_item in &node.items {
                        if let ast::ClassItem::ClassMethodDecl(method) = class_item {
                            if method.body_opt.is_none() {
                                continue;
                            }
                            let sig = build_class_method_capability_signature(ctx, &module.path, &node.name, method);
                            cg.add_node(callable(&module_path, record_method_node_name(&node.name, &method.name), sig, &method.span));
                        }
                    }
                }
                ASTItem::ModalDecl(node) => {
                    for state in &node.states {
                        for member in &state.members {
                            match member {
                                ast::StateMember::StateMethodDecl(method) => {
                                    let sig = build_state_method_capability_signature(ctx, &module.path, &node.name, &state.name, method);
                                    cg.add_node(callable(&module_path, state_method_node_name(&node.name, &state.name, &method.name), sig, &method.span));
                                }
                                ast::StateMember::TransitionDecl(transition) => {
                                    let sig = build_transition_capability_signature(ctx, &module.path, &node.name, &state.name, transition);
                                    cg.add_node(callable(&module_path, transition_node_name(&node.name, &state.name, &transition.name), sig, &transition.span));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    for module in modules {
        let module_path = module_path_to_string(&module.path);
        let id = |name: String| ProcId { module_path: module_path.clone(), name };
        for item in &module.items {
            match item {
                ASTItem::ProcedureDecl(node) => {
                    if let Some(body) = node.body.as_deref() {
                        add_calls_from_body(&mut cg, id(node.name.clone()), &module_path, body, modules);
                    }
                }
                ASTItem::RecordDecl(node) => {
                    for member in &node.members {
                        if let ast::RecordMember::MethodDecl(method) = member {
                            if let Some(body) = method.body.as_deref() {
                                add_calls_from_body(&mut cg, id(record_method_node_name(&node.name, &method.name)), &module_path, body, modules);
                            }
                        }
                    }
                }
                ASTItem::ClassDecl(node) => {
                    for class_item in &node.items {
                        if let ast::ClassItem::ClassMethodDecl(method) = class_item {
                            if let Some(body) = method.body_opt.as_deref() {
                                add_calls_from_body(&mut cg, id(record_method_node_name(&node.name, &method.name)), &module_path, body, modules);
                            }
                        }
                    }
                }
                ASTItem::ModalDecl(node) => {
                    for state in &node.states {
                        for member in &state.members {
                            match member {
                                ast::StateMember::StateMethodDecl(method) => {
                                    if let Some(body) = method.body.as_deref() {
                                        add_calls_from_body(&mut cg, id(state_method_node_name(&node.name, &state.name, &method.name)), &module_path, body, modules);
                                    }
                                }
                                ast::StateMember::TransitionDecl(transition) => {
                                    if let Some(body) = transition.body.as_deref() {
                                        add_calls_from_body(&mut cg, id(transition_node_name(&node.name, &state.name, &transition.name)), &module_path, body, modules);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    cg
}

/// For each edge, the capabilities the caller holds that the callee asks for.
pub fn annotate_capability_flow(cg: &mut CallGraph<'_>) {
    let passed: Vec<Option<CapabilitySet>> = cg
        .edges
        .iter()
        .map(|edge| {
            let (caller, callee) = (cg.node(&edge.caller)?, cg.node(&edge.callee)?);
            Some(caller.cap_sig.provided.intersection(&callee.cap_sig.required))
        })
        .collect();
    for (edge, caps) in cg.edges.iter_mut().zip(passed) {
        if let Some(caps) = caps {
            edge.capabilities_passed = caps;
        }
    }
}

/// The two steps of `PropagateCapabilityRequirements`: the flow along each edge.
pub fn propagate_capability_requirements(cg: &mut CallGraph<'_>) {
    annotate_capability_flow(cg);
}

fn detect_capability_leaks(cg: &CallGraph<'_>) -> Vec<CapabilityLeak> {
    let mut leaks = Vec::new();
    for node in cg.nodes.iter().filter(|node| node.is_extern) {
        for edge in cg.incoming(&node.id) {
            let passed = &edge.capabilities_passed;
            if passed.is_empty() {
                continue;
            }
            // The first capability held, in the order of the declaration.
            let capability = [
                (passed.has_io, CapabilityKind::IO),
                (passed.has_network, CapabilityKind::Network),
                (passed.has_heap, CapabilityKind::HeapAllocator),
                (passed.has_execution_domain, CapabilityKind::ExecutionDomain),
                (passed.has_reactor, CapabilityKind::Reactor),
                (passed.has_system, CapabilityKind::System),
                (passed.has_time, CapabilityKind::Time),
                (passed.has_monotonic_time, CapabilityKind::MonotonicTime),
                (passed.has_wall_time, CapabilityKind::WallTime),
            ]
            .iter()
            .find(|(held, _)| *held)
            .map(|(_, kind)| *kind)
            .unwrap_or(CapabilityKind::IO);
            leaks.push(CapabilityLeak {
                code: "E-TYP-2623",
                capability,
                sink: node.id.clone(),
                leak_span: edge.call_span.clone(),
                message: format!("Capability {} leaked to extern procedure '{}'", capability_kind_name(capability), node.id.name),
            });
        }
    }
    leaks
}

/// The capabilities the parameters of a callable type ask for.
fn infer_callable_param_capabilities(ctx: &ScopeContext<'_>, callee_type: &TypeRef, current_module: &[String]) -> CapabilitySet {
    let infer = |ty: &TypeRef| infer_capabilities_from_type(ctx, current_module, ty);
    let Some(ty) = callee_type.as_deref() else {
        return CapabilitySet::default();
    };
    match &ty.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => infer_callable_param_capabilities(ctx, base, current_module),
        TypeNode::Func { params, .. } => params.iter().fold(CapabilitySet::default(), |acc, param| acc.union(&infer(&param.r#type))),
        TypeNode::Closure { params, .. } => params.iter().fold(CapabilitySet::default(), |acc, (_, ty)| acc.union(&infer(ty))),
        TypeNode::Union(members) => members.iter().fold(CapabilitySet::default(), |acc, member| acc.union(&infer_callable_param_capabilities(ctx, member, current_module))),
        _ => CapabilitySet::default(),
    }
}

fn expr_type(stores: &TypeStores, expr: &ast::Expr) -> Option<TypeRef> {
    stores.expr_types.borrow().get(&(std::ptr::from_ref(expr) as usize)).map(|(_, ty)| ty.clone())
}

/// Every callee must be callable with what its caller provides: edges, then calls whose
/// callee is not a name, judged by the type of the callee or of the receiver and arguments.
pub fn validate_capability_chain(ctx: &ScopeContext<'_>, cg: &CallGraph<'_>, stores: Option<&TypeStores>) -> CapabilityChainResult {
    let mut result = CapabilityChainResult { valid: true, errors: Vec::new(), leaks: detect_capability_leaks(cg) };
    if !result.leaks.is_empty() {
        result.valid = false;
    }
    for node in cg.nodes.iter().filter(|node| !node.is_extern) {
        let incoming: Vec<&CallGraphEdge> = cg.incoming(&node.id).collect();
        if incoming.is_empty() && !id_eq(&node.id.name, "main") {
            continue;
        }
        for edge in incoming {
            let Some(caller) = cg.node(&edge.caller) else {
                continue;
            };
            if node.cap_sig.required.is_subset_of(&caller.cap_sig.provided) {
                continue;
            }
            result.valid = false;
            result.errors.push(CapabilityChainError {
                code: "E-CON-0020".to_string(),
                message: format!(
                    "Procedure '{}' requires capabilities {} but caller '{}' only provides {}",
                    node.id.name, node.cap_sig.required, edge.caller.name, caller.cap_sig.provided
                ),
                span: Some(edge.call_span.clone()),
            });
        }
    }
    let Some(stores) = stores else {
        return result;
    };
    for unresolved in &cg.unresolved_direct_calls {
        let Some(caller) = cg.node(&unresolved.caller) else {
            continue;
        };
        let Some(callee_type) = expr_type(stores, unresolved.callee_expr) else {
            continue;
        };
        let caller_module = module_path_from_string(&unresolved.caller.module_path);
        let required = infer_callable_param_capabilities(ctx, &callee_type, &caller_module);
        if required.is_empty() {
            continue;
        }
        let check = validate_capability_satisfied(&caller.cap_sig.provided, &required);
        if check.valid {
            continue;
        }
        result.valid = false;
        let mut message = format!(
            "Unresolved direct call in procedure '{}' requires capabilities {} inferred from callee expression type, but caller only provides {}",
            unresolved.caller.name, required, caller.cap_sig.provided
        );
        if !check.error_message.is_empty() {
            message.push_str("; ");
            message.push_str(&check.error_message);
        }
        result.errors.push(CapabilityChainError { code: check.error_code.to_string(), message, span: Some(unresolved.call_span.clone()) });
    }
    for unresolved in &cg.unresolved_method_calls {
        let Some(caller) = cg.node(&unresolved.caller) else {
            continue;
        };
        let caller_module = module_path_from_string(&unresolved.caller.module_path);
        let mut required = CapabilitySet::default();
        if let Some(ty) = expr_type(stores, unresolved.receiver_expr) {
            required = required.union(&infer_capabilities_from_type(ctx, &caller_module, &ty));
        }
        for arg in &unresolved.arg_exprs {
            if let Some(ty) = expr_type(stores, arg) {
                required = required.union(&infer_capabilities_from_type(ctx, &caller_module, &ty));
            }
        }
        if required.is_empty() {
            continue;
        }
        let check = validate_capability_satisfied(&caller.cap_sig.provided, &required);
        if check.valid {
            continue;
        }
        result.valid = false;
        let mut message = format!(
            "Method call '{}' in procedure '{}' requires capabilities {} inferred from receiver/argument types, but caller only provides {}",
            unresolved.method_name, unresolved.caller.name, required, caller.cap_sig.provided
        );
        if !check.error_message.is_empty() {
            message.push_str("; ");
            message.push_str(&check.error_message);
        }
        result.errors.push(CapabilityChainError { code: check.error_code.to_string(), message, span: Some(unresolved.call_span.clone()) });
    }
    result
}
