//! The authority model: capabilities reach a procedure only through its parameters
//! (no ambient authority), attenuated capabilities do not outlive the capability they
//! derive from, and extern procedures take no capability. See `authority_model.cpp`.

use std::collections::HashSet;
use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::{self, ApplyArgs, ArraySegment, EnumPayload, ExprNode as E, PatternNode as P, Stmt};

use super::cap_requirements::*;
use crate::context::ScopeContext;
use crate::typing::expr_store::TypeStores;
use crate::typing::types::{TypeNode, TypeRef};

pub struct AuthorityValidationResult {
    pub valid: bool,
    pub error_code: String,
    pub error_message: String,
    pub span: Span,
}

impl AuthorityValidationResult {
    fn ok(span: &Span) -> Self {
        AuthorityValidationResult { valid: true, error_code: String::new(), error_message: String::new(), span: span.clone() }
    }
}

pub struct ModuleAuthorityResult {
    pub valid: bool,
    pub errors: Vec<AuthorityValidationResult>,
}

struct AmbientAuthorityContext<'a, 'x> {
    stores: Option<&'a TypeStores>,
    scope_ctx: Option<&'a ScopeContext<'x>>,
    current_module: &'a [String],
    local_scopes: Vec<HashSet<String>>,
}

impl AmbientAuthorityContext<'_, '_> {
    fn push(&mut self) {
        self.local_scopes.push(HashSet::new());
    }

    fn pop(&mut self) {
        self.local_scopes.pop();
    }

    fn bind(&mut self, name: &str) {
        if self.local_scopes.is_empty() {
            self.push();
        }
        self.local_scopes.last_mut().expect("a scope").insert(name.to_string());
    }

    fn is_locally_bound(&self, name: &str) -> bool {
        self.local_scopes.iter().any(|scope| scope.contains(name))
    }

    /// Whether the expression has a type that carries a capability (a function that
    /// merely takes one does not).
    fn has_capability_type(&self, expr: &ast::Expr) -> bool {
        let Some(stores) = self.stores else {
            return false;
        };
        let Some(ty) = stores.expr_types.borrow().get(&(std::ptr::from_ref(expr) as usize)).map(|(_, ty)| ty.clone()) else {
            return false;
        };
        if matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Func { .. } | TypeNode::Closure { .. })) {
            return false;
        }
        match self.scope_ctx {
            Some(ctx) => !infer_capabilities_from_type(ctx, self.current_module, &ty).is_empty(),
            None => !infer_capabilities_from_type_alone(&ty).is_empty(),
        }
    }

    fn bind_pattern(&mut self, pattern: &ast::Pattern) {
        match &pattern.node {
            P::IdentifierPattern(node) => self.bind(&node.name),
            P::TypedPattern(node) => {
                if node.name != "_" {
                    self.bind(&node.name);
                }
            }
            P::TuplePattern(node) => node.elements.iter().flatten().for_each(|element| self.bind_pattern(element)),
            P::RecordPattern(node) => node.fields.iter().for_each(|field| self.bind_field(field)),
            P::EnumPattern(node) => match &node.payload_opt {
                Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => payload.elements.iter().flatten().for_each(|element| self.bind_pattern(element)),
                Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => payload.fields.iter().for_each(|field| self.bind_field(field)),
                None => {}
            },
            P::ModalPattern(node) => {
                if let Some(fields) = &node.fields_opt {
                    fields.fields.iter().for_each(|field| self.bind_field(field));
                }
            }
            P::RangePattern(node) => {
                for bound in [&node.lo, &node.hi].into_iter().flatten() {
                    self.bind_pattern(bound);
                }
            }
            _ => {}
        }
    }

    fn bind_field(&mut self, field: &ast::FieldPattern) {
        match &field.pattern_opt {
            Some(pattern) => self.bind_pattern(pattern),
            None => self.bind(&field.name),
        }
    }

    fn args(&mut self, args: &[ast::Arg]) -> bool {
        args.iter().any(|arg| self.expr_opt(&arg.value))
    }

    fn exprs(&mut self, exprs: &[ast::ExprPtr]) -> bool {
        exprs.iter().any(|expr| self.expr_opt(expr))
    }

    fn fields(&mut self, fields: &[ast::FieldInit]) -> bool {
        fields.iter().any(|field| self.expr_opt(&field.value))
    }

    fn key_path(&mut self, key_path: &ast::KeyPathExpr) -> bool {
        key_path.segs.iter().any(|seg| matches!(seg, ast::KeySeg::KeySegIndex(index) if self.expr_opt(&index.expr)))
    }

    fn expr_opt(&mut self, expr: &ast::ExprPtr) -> bool {
        expr.as_deref().is_some_and(|expr| self.expr(expr))
    }

    fn block_opt(&mut self, block: &Option<Arc<ast::Block>>) -> bool {
        block.as_deref().is_some_and(|block| self.block(block))
    }

    fn invariant(&mut self, invariant: &Option<ast::LoopInvariant>) -> bool {
        invariant.as_ref().is_some_and(|invariant| self.expr_opt(&invariant.predicate))
    }

    /// Whether the expression names a capability that was not passed in.
    fn expr(&mut self, expr: &ast::Expr) -> bool {
        match &expr.node {
            E::IdentifierExpr(node) => !self.is_locally_bound(&node.name) && self.has_capability_type(expr),
            E::PathExpr(_) | E::QualifiedNameExpr(_) => self.has_capability_type(expr),
            E::QualifiedApplyExpr(node) => {
                self.has_capability_type(expr)
                    || match &node.args {
                        ApplyArgs::ParenArgs(paren) => self.args(&paren.args),
                        ApplyArgs::BraceArgs(brace) => self.fields(&brace.fields),
                    }
            }
            E::RangeExpr(node) => self.expr_opt(&node.lhs) || self.expr_opt(&node.rhs),
            E::BinaryExpr(node) => self.expr_opt(&node.lhs) || self.expr_opt(&node.rhs),
            E::CastExpr(node) => self.expr_opt(&node.value),
            E::UnaryExpr(node) => self.expr_opt(&node.value),
            E::DerefExpr(node) => self.expr_opt(&node.value),
            E::AddressOfExpr(node) => self.expr_opt(&node.place),
            E::MoveExpr(node) => self.expr_opt(&node.place),
            E::AllocExpr(node) => self.expr_opt(&node.value),
            E::TupleExpr(node) => self.exprs(&node.elements),
            E::ArrayExpr(node) => node.elements.iter().any(|segment| match segment {
                ArraySegment::ArrayElemSegment(segment) => self.expr_opt(&segment.value),
                ArraySegment::ArrayRepeatSegment(segment) => self.expr_opt(&segment.value) || self.expr_opt(&segment.count),
            }),
            E::ArrayRepeatExpr(node) => self.expr_opt(&node.value) || self.expr_opt(&node.count),
            E::RecordExpr(node) => self.fields(&node.fields),
            E::EnumLiteralExpr(node) => match &node.payload_opt {
                None => false,
                Some(EnumPayload::EnumPayloadParen(payload)) => self.exprs(&payload.elements),
                Some(EnumPayload::EnumPayloadBrace(payload)) => self.fields(&payload.fields),
            },
            E::IfExpr(node) => self.expr_opt(&node.cond) || self.expr_opt(&node.then_expr) || self.expr_opt(&node.else_expr),
            E::IfCaseExpr(node) => {
                if self.expr_opt(&node.scrutinee) {
                    return true;
                }
                for arm in &node.cases {
                    self.push();
                    if let Some(pattern) = &arm.pattern {
                        self.bind_pattern(pattern);
                    }
                    let found = self.expr_opt(&arm.body);
                    self.pop();
                    if found {
                        return true;
                    }
                }
                self.expr_opt(&node.else_expr)
            }
            E::IfIsExpr(node) => {
                if self.expr_opt(&node.scrutinee) {
                    return true;
                }
                self.push();
                if let Some(pattern) = &node.pattern {
                    self.bind_pattern(pattern);
                }
                let found = self.expr_opt(&node.then_expr);
                self.pop();
                found || self.expr_opt(&node.else_expr)
            }
            E::LoopInfiniteExpr(node) => self.invariant(&node.invariant_opt) || self.block_opt(&node.body),
            E::LoopConditionalExpr(node) => self.expr_opt(&node.cond) || self.invariant(&node.invariant_opt) || self.block_opt(&node.body),
            E::LoopIterExpr(node) => {
                if self.expr_opt(&node.iter) {
                    return true;
                }
                self.push();
                if let Some(pattern) = &node.pattern {
                    self.bind_pattern(pattern);
                }
                let found = self.invariant(&node.invariant_opt) || self.block_opt(&node.body);
                self.pop();
                found
            }
            E::BlockExpr(node) => self.block_opt(&node.block),
            E::UnsafeBlockExpr(node) => self.block_opt(&node.block),
            E::AttributedExpr(node) => self.expr_opt(&node.expr),
            E::TransmuteExpr(node) => self.expr_opt(&node.value),
            E::ClosureExpr(node) => {
                self.push();
                for param in &node.params {
                    self.bind(&param.name);
                }
                let found = self.expr_opt(&node.body);
                self.pop();
                found
            }
            E::PipelineExpr(node) => self.expr_opt(&node.lhs) || self.expr_opt(&node.rhs),
            E::FieldAccessExpr(node) => self.expr_opt(&node.base),
            E::TupleAccessExpr(node) => self.expr_opt(&node.base),
            E::IndexAccessExpr(node) => self.expr_opt(&node.base) || self.expr_opt(&node.index),
            E::CallExpr(node) => self.expr_opt(&node.callee) || self.args(&node.args),
            E::MethodCallExpr(node) => self.expr_opt(&node.receiver) || self.args(&node.args),
            E::PropagateExpr(node) => self.expr_opt(&node.value),
            E::EntryExpr(node) => self.expr_opt(&node.expr),
            E::YieldExpr(node) => self.expr_opt(&node.value),
            E::YieldFromExpr(node) => self.expr_opt(&node.value),
            E::SyncExpr(node) => self.expr_opt(&node.value),
            E::RaceExpr(node) => {
                for arm in &node.arms {
                    if self.expr_opt(&arm.expr) {
                        return true;
                    }
                    self.push();
                    if let Some(pattern) = &arm.pattern {
                        self.bind_pattern(pattern);
                    }
                    let found = self.expr_opt(&arm.handler.value);
                    self.pop();
                    if found {
                        return true;
                    }
                }
                false
            }
            E::AllExpr(node) => self.exprs(&node.exprs),
            E::ParallelExpr(node) => self.expr_opt(&node.domain) || node.opts.iter().any(|opt| self.expr_opt(&opt.value)) || self.block_opt(&node.body),
            E::SpawnExpr(node) => node.opts.iter().any(|opt| self.expr_opt(&opt.value)) || self.block_opt(&node.body),
            E::WaitExpr(node) => self.expr_opt(&node.handle),
            E::DispatchExpr(node) => {
                if self.expr_opt(&node.range) || node.key_clause.as_ref().is_some_and(|clause| self.key_path(&clause.key_path)) {
                    return true;
                }
                if node.opts.iter().any(|opt| self.expr_opt(&opt.chunk_expr) || self.expr_opt(&opt.workgroup_expr)) {
                    return true;
                }
                self.push();
                if let Some(pattern) = &node.pattern {
                    self.bind_pattern(pattern);
                }
                let found = self.block_opt(&node.body);
                self.pop();
                found
            }
            _ => false,
        }
    }

    fn stmt(&mut self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::LetStmt(node) => {
                if self.expr_opt(&node.binding.init) {
                    return true;
                }
                if let Some(pattern) = &node.binding.pat {
                    self.bind_pattern(pattern);
                }
                false
            }
            Stmt::VarStmt(node) => {
                if self.expr_opt(&node.binding.init) {
                    return true;
                }
                if let Some(pattern) = &node.binding.pat {
                    self.bind_pattern(pattern);
                }
                false
            }
            Stmt::UsingLocalStmt(node) => {
                self.bind(&node.alias);
                false
            }
            Stmt::AssignStmt(node) => self.expr_opt(&node.place) || self.expr_opt(&node.value),
            Stmt::CompoundAssignStmt(node) => self.expr_opt(&node.place) || self.expr_opt(&node.value),
            Stmt::ExprStmt(node) => self.expr_opt(&node.value),
            Stmt::DeferStmt(node) => self.block_opt(&node.body),
            Stmt::RegionStmt(node) => {
                if self.expr_opt(&node.opts_opt) {
                    return true;
                }
                self.push();
                if let Some(alias) = &node.alias_opt {
                    self.bind(alias);
                }
                let found = self.block_opt(&node.body);
                self.pop();
                found
            }
            Stmt::FrameStmt(node) => self.block_opt(&node.body),
            Stmt::ReturnStmt(node) => self.expr_opt(&node.value_opt),
            Stmt::BreakStmt(node) => self.expr_opt(&node.value_opt),
            Stmt::UnsafeBlockStmt(node) => self.block_opt(&node.body),
            Stmt::KeyBlockStmt(node) => node.paths.iter().any(|path| self.key_path(path)) || self.block_opt(&node.body),
            _ => false,
        }
    }

    fn block(&mut self, block: &ast::Block) -> bool {
        self.push();
        let found = block.stmts.iter().any(|stmt| self.stmt(stmt)) || self.expr_opt(&block.tail_opt);
        self.pop();
        found
    }
}

/// A procedure may not use a capability it was not handed through a parameter.
fn check_ambient_authority(ctx: &ScopeContext<'_>, current_module: &[String], proc: &ast::ProcedureDecl, stores: Option<&TypeStores>) -> AuthorityValidationResult {
    let mut result = AuthorityValidationResult::ok(&proc.span);
    let Some(body) = proc.body.as_deref() else {
        return result;
    };
    let mut ambient = AmbientAuthorityContext { stores, scope_ctx: Some(ctx), current_module, local_scopes: Vec::new() };
    ambient.push();
    for param in &proc.params {
        ambient.bind(&param.name);
    }
    if ambient.block(body) {
        result.valid = false;
        result.error_code = "E-CON-0020".to_string();
        result.error_message =
            format!("Procedure '{}' uses ambient authority; all capabilities must be passed explicitly through parameters", proc.name);
    }
    result
}

// ---- attenuation: a derived capability may not outlive its parent ----

#[derive(Clone)]
struct AttenuationOrigin {
    parent: String,
    span: Span,
}

struct AttenuationBinding {
    name: String,
    depth: i32,
    parameter: bool,
    parent: Option<String>,
    span: Span,
}

struct AttenuationContext<'a> {
    stores: Option<&'a TypeStores>,
    scopes: Vec<Vec<String>>,
    bindings: Vec<AttenuationBinding>,
    result: AuthorityValidationResult,
}

fn strip_perm_and_refine(ty: &TypeRef) -> TypeRef {
    let mut current = ty.clone();
    loop {
        match current.as_deref().map(|ty| &ty.node) {
            Some(TypeNode::Perm { base, .. }) | Some(TypeNode::Refine { base, .. }) => current = base.clone(),
            _ => return current,
        }
    }
}

fn type_path_equals(path: &[String], name: &str) -> bool {
    matches!(path, [only] if crate::resolve::scopes::id_eq(only, name))
}

fn is_dynamic_type_named(ty: &TypeRef, name: &str) -> bool {
    match strip_perm_and_refine(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Dynamic(path)) => type_path_equals(path, name),
        Some(TypeNode::Path { path, .. }) => type_path_equals(path, name),
        _ => false,
    }
}

fn capability_kind_of_dynamic(ty: &TypeRef) -> Option<CapabilityKind> {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Dynamic(path)) => capability_kind_from_path(path),
        _ => None,
    }
}

fn is_context_type(ty: &TypeRef) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Path { path, .. }) if super::builtin_paths::is_context_type_path(path))
}

/// A method that derives a weaker capability from the one it is called on.
fn is_attenuation_method(name: &str, receiver_type: &TypeRef) -> bool {
    let stripped = strip_perm_and_refine(receiver_type);
    if stripped.is_none() {
        return false;
    }
    let kind = capability_kind_of_dynamic(&stripped);
    match name {
        "restrict" => kind == Some(CapabilityKind::IO),
        "restrict_to_host" => kind == Some(CapabilityKind::Network),
        "with_quota" => kind == Some(CapabilityKind::HeapAllocator),
        "monotonic" | "wall" => kind == Some(CapabilityKind::Time),
        "coarsen" => is_dynamic_type_named(&stripped, "MonotonicTime") || is_dynamic_type_named(&stripped, "WallTime"),
        "cpu" | "gpu" | "inline" => is_context_type(&stripped),
        "child" => matches!(
            stripped.as_deref().map(|ty| &ty.node),
            Some(TypeNode::ModalState(modal)) if type_path_equals(&modal.path, "CancelToken") && crate::resolve::scopes::id_eq(&modal.state, "Active")
        ),
        _ => false,
    }
}

fn direct_identifier_name(expr: &ast::ExprPtr) -> Option<String> {
    match &expr.as_deref()?.node {
        E::IdentifierExpr(ident) => Some(ident.name.clone()),
        E::AttributedExpr(node) => direct_identifier_name(&node.expr),
        E::MoveExpr(node) => direct_identifier_name(&node.place),
        _ => None,
    }
}

fn collect_pattern_names(pattern: &ast::Pattern, names: &mut Vec<String>) {
    let field = |field: &ast::FieldPattern, names: &mut Vec<String>| match &field.pattern_opt {
        Some(pattern) => collect_pattern_names(pattern, names),
        None if field.name != "_" => names.push(field.name.clone()),
        None => {}
    };
    match &pattern.node {
        P::IdentifierPattern(node) if node.name != "_" => names.push(node.name.clone()),
        P::TypedPattern(node) if node.name != "_" => names.push(node.name.clone()),
        P::TuplePattern(node) => node.elements.iter().flatten().for_each(|element| collect_pattern_names(element, names)),
        P::RecordPattern(node) => node.fields.iter().for_each(|f| field(f, names)),
        P::EnumPattern(node) => match &node.payload_opt {
            Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => payload.elements.iter().flatten().for_each(|element| collect_pattern_names(element, names)),
            Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => payload.fields.iter().for_each(|f| field(f, names)),
            None => {}
        },
        P::ModalPattern(node) => {
            if let Some(fields) = &node.fields_opt {
                fields.fields.iter().for_each(|f| field(f, names));
            }
        }
        P::RangePattern(node) => {
            for bound in [&node.lo, &node.hi].into_iter().flatten() {
                collect_pattern_names(bound, names);
            }
        }
        _ => {}
    }
}

fn first_origin(left: Option<AttenuationOrigin>, right: impl FnOnce() -> Option<AttenuationOrigin>) -> Option<AttenuationOrigin> {
    // Both sides are always analysed, as the reference does, so `right` runs regardless.
    let right = right();
    left.or(right)
}

impl AttenuationContext<'_> {
    fn failed(&self) -> bool {
        !self.result.valid
    }

    fn depth(&self) -> i32 {
        if self.scopes.is_empty() {
            0
        } else {
            self.scopes.len() as i32 - 1
        }
    }

    fn binding(&self, name: &str) -> Option<&AttenuationBinding> {
        self.bindings.iter().find(|binding| binding.name == name)
    }

    fn report_escape(&mut self, span: &Span, parent: &str) {
        if self.failed() {
            return;
        }
        self.result.valid = false;
        self.result.error_code = "E-MEM-3020".to_string();
        self.result.span = span.clone();
        self.result.error_message = format!(
            "Derived capability escapes the lifetime of parent capability '{parent}'; dropping a parent capability while a derived child remains live is ill-formed"
        );
    }

    fn parent_is_local_to_depth(&self, parent: &str, depth: i32) -> bool {
        self.binding(parent).is_some_and(|binding| !binding.parameter && binding.depth >= depth)
    }

    fn check_origin_escapes_scope(&mut self, origin: &Option<AttenuationOrigin>, scope_depth: i32) {
        if let Some(origin) = origin {
            if self.parent_is_local_to_depth(&origin.parent, scope_depth) {
                self.report_escape(&origin.span, &origin.parent);
            }
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        if self.scopes.is_empty() {
            return;
        }
        let depth = self.depth();
        let locals = self.scopes.last().cloned().unwrap_or_default();
        for local in &locals {
            if self.binding(local).is_none() {
                continue;
            }
            let escaped = self.bindings.iter().find(|child| child.parent.as_deref() == Some(local.as_str()) && child.depth < depth).map(|child| child.span.clone());
            if let Some(span) = escaped {
                self.report_escape(&span, local);
            }
        }
        for local in &locals {
            self.bindings.retain(|binding| binding.name != *local);
        }
        self.scopes.pop();
    }

    fn bind(&mut self, name: &str, origin: Option<AttenuationOrigin>, parameter: bool, span: &Span) {
        if self.scopes.is_empty() {
            self.push_scope();
        }
        self.scopes.last_mut().expect("a scope").push(name.to_string());
        let info = AttenuationBinding { name: name.to_string(), depth: self.depth(), parameter, parent: origin.map(|origin| origin.parent), span: span.clone() };
        match self.bindings.iter().position(|binding| binding.name == name) {
            Some(at) => self.bindings[at] = info,
            None => self.bindings.push(info),
        }
    }

    fn args(&mut self, args: &[ast::Arg]) {
        for arg in args {
            self.expr(&arg.value);
        }
    }

    fn block_opt(&mut self, block: &Option<Arc<ast::Block>>) {
        if let Some(block) = block.as_deref() {
            self.block(block);
        }
    }

    fn expr(&mut self, expr: &ast::ExprPtr) -> Option<AttenuationOrigin> {
        let node = expr.as_deref()?;
        if self.failed() {
            return None;
        }
        let span = node.span.clone();
        match &node.node {
            E::IdentifierExpr(ident) => {
                let parent = self.binding(&ident.name).and_then(|binding| binding.parent.clone())?;
                Some(AttenuationOrigin { parent, span })
            }
            E::CastExpr(n) => self.expr(&n.value),
            E::UnaryExpr(n) => self.expr(&n.value),
            E::DerefExpr(n) => self.expr(&n.value),
            E::AddressOfExpr(n) => self.expr(&n.place),
            E::MoveExpr(n) => self.expr(&n.place),
            E::CopyExpr(n) => self.expr(&n.value),
            E::AllocExpr(n) => self.expr(&n.value),
            E::AttributedExpr(n) => self.expr(&n.expr),
            E::TransmuteExpr(n) => self.expr(&n.value),
            E::PropagateExpr(n) => self.expr(&n.value),
            E::TupleExpr(n) => n.elements.iter().fold(None, |origin, element| first_origin(origin, || self.expr(element))),
            E::ArrayExpr(n) => {
                let mut origin = None;
                for segment in &n.elements {
                    match segment {
                        ArraySegment::ArrayElemSegment(segment) => origin = first_origin(origin, || self.expr(&segment.value)),
                        ArraySegment::ArrayRepeatSegment(segment) => {
                            origin = first_origin(origin, || self.expr(&segment.value));
                            origin = first_origin(origin, || self.expr(&segment.count));
                        }
                    }
                }
                origin
            }
            E::ArrayRepeatExpr(n) => {
                let value = self.expr(&n.value);
                first_origin(value, || self.expr(&n.count))
            }
            E::RecordExpr(n) => n.fields.iter().fold(None, |origin, field| first_origin(origin, || self.expr(&field.value))),
            E::EnumLiteralExpr(n) => match &n.payload_opt {
                None => None,
                Some(EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().fold(None, |origin, element| first_origin(origin, || self.expr(element))),
                Some(EnumPayload::EnumPayloadBrace(payload)) => payload.fields.iter().fold(None, |origin, field| first_origin(origin, || self.expr(&field.value))),
            },
            E::BinaryExpr(n) => {
                let lhs = self.expr(&n.lhs);
                first_origin(lhs, || self.expr(&n.rhs))
            }
            E::RangeExpr(n) => {
                let lhs = self.expr(&n.lhs);
                first_origin(lhs, || self.expr(&n.rhs))
            }
            E::IfExpr(n) => {
                self.expr(&n.cond);
                let then = self.expr(&n.then_expr);
                first_origin(then, || self.expr(&n.else_expr))
            }
            E::IfIsExpr(n) => {
                self.expr(&n.scrutinee);
                let then = self.expr(&n.then_expr);
                first_origin(then, || self.expr(&n.else_expr))
            }
            E::IfCaseExpr(n) => {
                let mut origin = self.expr(&n.scrutinee);
                for clause in &n.cases {
                    origin = first_origin(origin, || self.expr(&clause.body));
                }
                first_origin(origin, || self.expr(&n.else_expr))
            }
            E::BlockExpr(n) => n.block.as_deref().and_then(|block| self.block(block)),
            E::UnsafeBlockExpr(n) => n.block.as_deref().and_then(|block| self.block(block)),
            E::LoopInfiniteExpr(n) => {
                self.block_opt(&n.body);
                None
            }
            E::LoopConditionalExpr(n) => {
                self.expr(&n.cond);
                self.block_opt(&n.body);
                None
            }
            E::LoopIterExpr(n) => {
                self.expr(&n.iter);
                self.block_opt(&n.body);
                None
            }
            E::ClosureExpr(n) => self.expr(&n.body),
            E::PipelineExpr(n) => {
                let lhs = self.expr(&n.lhs);
                first_origin(lhs, || self.expr(&n.rhs))
            }
            E::FieldAccessExpr(n) => self.expr(&n.base),
            E::TupleAccessExpr(n) => self.expr(&n.base),
            E::IndexAccessExpr(n) => {
                let base = self.expr(&n.base);
                first_origin(base, || self.expr(&n.index))
            }
            E::CallExpr(n) => {
                self.expr(&n.callee);
                self.args(&n.args);
                None
            }
            E::CallTypeArgsExpr(n) => {
                self.expr(&n.callee);
                self.args(&n.args);
                None
            }
            E::MethodCallExpr(n) => {
                self.expr(&n.receiver);
                self.args(&n.args);
                let receiver_type = n.receiver.as_deref().and_then(|receiver| {
                    self.stores?.expr_types.borrow().get(&(std::ptr::from_ref(receiver) as usize)).map(|(_, ty)| ty.clone())
                });
                if let Some(receiver_type) = receiver_type {
                    if is_attenuation_method(&n.name, &receiver_type) {
                        if let Some(parent) = direct_identifier_name(&n.receiver) {
                            if self.binding(&parent).is_some() {
                                return Some(AttenuationOrigin { parent, span });
                            }
                        }
                    }
                }
                None
            }
            E::YieldExpr(n) => self.expr(&n.value),
            E::YieldFromExpr(n) => self.expr(&n.value),
            E::SyncExpr(n) => self.expr(&n.value),
            E::RaceExpr(n) => {
                let mut origin = None;
                for arm in &n.arms {
                    origin = first_origin(origin, || self.expr(&arm.expr));
                    origin = first_origin(origin, || self.expr(&arm.handler.value));
                }
                origin
            }
            E::AllExpr(n) => n.exprs.iter().fold(None, |origin, value| first_origin(origin, || self.expr(value))),
            E::ParallelExpr(n) => {
                self.expr(&n.domain);
                n.opts.iter().for_each(|opt| {
                    self.expr(&opt.value);
                });
                self.block_opt(&n.body);
                None
            }
            E::SpawnExpr(n) => {
                n.opts.iter().for_each(|opt| {
                    self.expr(&opt.value);
                });
                self.block_opt(&n.body);
                None
            }
            E::WaitExpr(n) => self.expr(&n.handle),
            E::DispatchExpr(n) => {
                self.expr(&n.range);
                for opt in &n.opts {
                    self.expr(&opt.chunk_expr);
                    self.expr(&opt.workgroup_expr);
                }
                self.block_opt(&n.body);
                None
            }
            _ => None,
        }
    }

    fn binding_stmt(&mut self, binding: &ast::Binding) {
        let origin = self.expr(&binding.init);
        let mut names = Vec::new();
        if let Some(pattern) = &binding.pat {
            collect_pattern_names(pattern, &mut names);
        }
        for name in names {
            self.bind(&name, origin.clone(), false, &binding.span);
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        if self.failed() {
            return;
        }
        match stmt {
            Stmt::LetStmt(n) => self.binding_stmt(&n.binding),
            Stmt::VarStmt(n) => self.binding_stmt(&n.binding),
            Stmt::AssignStmt(n) => {
                let Some(origin) = self.expr(&n.value) else {
                    return;
                };
                let Some(target) = direct_identifier_name(&n.place) else {
                    return;
                };
                let escapes = match (self.binding(&target), self.binding(&origin.parent)) {
                    (Some(target), Some(parent)) => !parent.parameter && target.depth < parent.depth,
                    _ => false,
                };
                if escapes {
                    self.report_escape(&origin.span, &origin.parent);
                }
            }
            Stmt::CompoundAssignStmt(n) => {
                self.expr(&n.place);
                self.expr(&n.value);
            }
            Stmt::ExprStmt(n) => {
                self.expr(&n.value);
            }
            Stmt::DeferStmt(n) => self.block_opt(&n.body),
            Stmt::RegionStmt(n) => {
                self.expr(&n.opts_opt);
                self.block_opt(&n.body);
            }
            Stmt::FrameStmt(n) => self.block_opt(&n.body),
            Stmt::ReturnStmt(n) => self.escape_by_value(&n.value_opt),
            Stmt::BreakStmt(n) => self.escape_by_value(&n.value_opt),
            Stmt::UnsafeBlockStmt(n) => self.block_opt(&n.body),
            Stmt::CtStmt(n) => self.block_opt(&n.body),
            Stmt::KeyBlockStmt(n) => self.block_opt(&n.body),
            _ => {}
        }
    }

    /// A value that leaves the procedure may not derive from a local capability.
    fn escape_by_value(&mut self, value: &ast::ExprPtr) {
        if let Some(origin) = self.expr(value) {
            if self.binding(&origin.parent).is_some_and(|binding| !binding.parameter) {
                self.report_escape(&origin.span, &origin.parent);
            }
        }
    }

    fn block(&mut self, block: &ast::Block) -> Option<AttenuationOrigin> {
        self.push_scope();
        let scope_depth = self.depth();
        for stmt in &block.stmts {
            self.stmt(stmt);
            if self.failed() {
                self.pop_scope();
                return None;
            }
        }
        let origin = self.expr(&block.tail_opt);
        self.check_origin_escapes_scope(&origin, scope_depth);
        self.pop_scope();
        if self.failed() {
            return None;
        }
        origin
    }
}

fn check_attenuation_parent_liveness(proc: &ast::ProcedureDecl, stores: Option<&TypeStores>) -> AuthorityValidationResult {
    let ok = AuthorityValidationResult::ok(&proc.span);
    let Some(body) = proc.body.as_deref() else {
        return ok;
    };
    let mut ctx = AttenuationContext { stores, scopes: Vec::new(), bindings: Vec::new(), result: AuthorityValidationResult::ok(&Span::default()) };
    ctx.push_scope();
    for param in &proc.params {
        ctx.bind(&param.name, None, true, &param.span);
    }
    for stmt in &body.stmts {
        ctx.stmt(stmt);
        if ctx.failed() {
            return ctx.result;
        }
    }
    let tail = ctx.expr(&body.tail_opt);
    let depth = ctx.depth();
    ctx.check_origin_escapes_scope(&tail, depth);
    if ctx.failed() {
        return ctx.result;
    }
    ctx.pop_scope();
    ok
}

// ---- extern procedures take and return no capability ----

fn check_extern_block_isolation(ctx: &ScopeContext<'_>, current_module: &[String], block: &ast::ExternBlock) -> AuthorityValidationResult {
    for ast::ExternItem::ExternProcDecl(proc) in &block.items {
        let carries = |ty: &ast::TypePtr| ty.is_some() && !infer_capabilities_from_ast_type(ctx, current_module, ty).is_empty();
        let failure = |message: String| AuthorityValidationResult { valid: false, error_code: "E-TYP-2623".to_string(), error_message: message, span: proc.span.clone() };
        if let Some(param) = proc.params.iter().find(|param| carries(&param.r#type)) {
            return failure(format!(
                "Extern procedure '{}' has capability-bearing parameter '{}'; foreign code cannot receive capabilities",
                proc.name, param.name
            ));
        }
        if carries(&proc.return_type_opt) {
            return failure(format!("Extern procedure '{}' returns capability-bearing type; foreign code cannot return capabilities", proc.name));
        }
    }
    AuthorityValidationResult::ok(&block.span)
}

/// The authority checks of every procedure and extern block of the modules.
pub fn validate_module_authority(ctx: &ScopeContext<'_>, modules: &[&ast::ASTModule], stores: Option<&TypeStores>) -> ModuleAuthorityResult {
    let mut result = ModuleAuthorityResult { valid: true, errors: Vec::new() };
    for module in modules {
        for item in &module.items {
            let mut checks: Vec<AuthorityValidationResult> = Vec::new();
            match item {
                ast::ASTItem::ProcedureDecl(proc) => {
                    checks.push(check_ambient_authority(ctx, &module.path, proc, stores));
                    checks.push(check_attenuation_parent_liveness(proc, stores));
                }
                ast::ASTItem::ExternBlock(block) => checks.push(check_extern_block_isolation(ctx, &module.path, block)),
                _ => {}
            }
            for check in checks.into_iter().filter(|check| !check.valid) {
                result.valid = false;
                result.errors.push(check);
            }
        }
    }
    result
}
