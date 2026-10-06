//! The provenance check of a body: where each value's storage comes from (a region, the
//! stack, the heap, a global, a parameter), and that nothing is stored in or captured
//! by something that outlives it. See `ProvBindCheck` in the reference's `regions`.
//!
//! The maps of expression provenance the reference also fills for later phases are not
//! kept yet. Their presence still decides one thing, mirrored by `infer`: while they
//! are filled, an expression's type comes from the stores only, and inside a loop body
//! (walked without them) it may also be inferred.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::span::Span;
use uv_core::std_unordered::UnorderedMap;
use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use super::borrow_bind::BindSelfParam;
use super::calls::arg_pass_expr;
use super::regions::{innermost_active_region, region_active_type};
use crate::caps::cap_methods::lookup_heap_allocator_method_sig;
use crate::composite::function_types::value_path_type;
use crate::composite::record_methods::lookup_method_static;
use crate::context::{IdKey, ScopeContext};
use crate::layout::size_of;
use crate::memory::string_bytes::lookup_string_bytes_builtin_method_sig;
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};
use crate::resolve::scopes_lookup::resolve_value_name;
use crate::typing::check_expr::infer_expr;
use crate::typing::expr::small::is_place_expr;
use crate::typing::expr_store::{selected_call_target, stored_expr_type};
use crate::typing::signature::subst_self_type;
use crate::typing::stmt::scoped::fresh_region_name;
use crate::typing::stmt_context::StmtTypeContext;
use crate::typing::type_env::{bind_of, collect_pat_names, type_pattern, TypeBinding, TypeEnv};
use crate::typing::type_expr::{type_expr, type_identifier_expr};
use crate::typing::type_lookup::async_sig_of;
use crate::typing::type_lower::{lower_param_mode, lower_type};
use crate::typing::type_predicates::strip_perm;
use crate::typing::types::*;

#[derive(Clone, PartialEq, Eq, Default)]
enum Tag {
    Global,
    Stack(usize),
    Heap,
    Region(IdKey),
    #[default]
    Bottom,
    Param(usize),
}

#[derive(Clone, Default)]
struct Scope {
    id: usize,
    map: HashMap<IdKey, Tag>,
}

/// A region in scope: the tag values allocated in it carry, the binding that names it,
/// and whether it is a frame.
#[derive(Clone)]
struct RegionEntry {
    tag: IdKey,
    target: IdKey,
    frame_active: bool,
}

#[derive(Clone, Default)]
struct Env {
    scopes: Vec<Scope>,
    regions: Vec<RegionEntry>,
    next_scope_id: usize,
}

/// What leaves a statement other than by falling through.
#[derive(Default)]
struct Flow {
    results: Vec<Tag>,
    breaks: Vec<Tag>,
    break_void: bool,
}

struct Fail {
    diag_id: Option<&'static str>,
    span: Option<Span>,
}

type ExprResult = Result<Tag, Fail>;

pub struct ProvCheckResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
}

fn fail<T>(diag_id: &'static str, span: Span) -> Result<T, Fail> {
    Err(Fail { diag_id: Some(diag_id), span: Some(span) })
}

fn region_nesting(env: &Env, inner: &str, outer: &str) -> bool {
    let index = |name: &str| env.regions.iter().position(|entry| entry.tag == name);
    matches!((index(inner), index(outer)), (Some(inner), Some(outer)) if inner > outer)
}

/// Strictly shorter-lived: an inner region before an outer one, a region before the
/// stack, the stack before the heap, the heap before globals.
fn prov_less(env: &Env, lhs: &Tag, rhs: &Tag) -> bool {
    match (lhs, rhs) {
        (Tag::Param(_), _) | (_, Tag::Param(_)) => false,
        (Tag::Region(lhs), Tag::Region(rhs)) => region_nesting(env, lhs, rhs),
        (Tag::Region(_), Tag::Stack(_)) | (Tag::Stack(_), Tag::Heap) | (Tag::Heap, Tag::Global) | (Tag::Global, Tag::Bottom) => true,
        _ => false,
    }
}

fn prov_rank(tag: &Tag) -> i32 {
    match tag {
        Tag::Region(_) => 0,
        Tag::Stack(_) => 1,
        Tag::Heap => 2,
        Tag::Global => 3,
        Tag::Bottom => 4,
        Tag::Param(_) => -1,
    }
}

fn prov_leq(env: &Env, lhs: &Tag, rhs: &Tag) -> bool {
    if lhs == rhs {
        return true;
    }
    match (lhs, rhs) {
        (Tag::Param(_), _) | (_, Tag::Param(_)) => false,
        (Tag::Region(lhs), Tag::Region(rhs)) => region_nesting(env, lhs, rhs),
        _ => prov_rank(lhs) < prov_rank(rhs),
    }
}

fn join_prov(env: &Env, lhs: &Tag, rhs: &Tag) -> Tag {
    if prov_leq(env, lhs, rhs) {
        lhs.clone()
    } else if prov_leq(env, rhs, lhs) {
        rhs.clone()
    } else {
        Tag::Bottom
    }
}

fn join_all_prov(env: &Env, tags: &[Tag]) -> Tag {
    match tags.split_first() {
        Some((first, rest)) => rest.iter().fold(first.clone(), |current, tag| join_prov(env, &current, tag)),
        None => Tag::Bottom,
    }
}

fn push_scope(env: &mut Env) {
    env.scopes.push(Scope { id: env.next_scope_id, map: HashMap::new() });
    env.next_scope_id += 1;
}

fn lookup_pi(env: &Env, name: &str) -> Option<Tag> {
    let key = id_key_of(name);
    env.scopes.iter().rev().find_map(|scope| scope.map.get(&key).cloned())
}

fn intro_pi(env: &mut Env, name: &str, tag: &Tag) {
    if let Some(scope) = env.scopes.last_mut() {
        scope.map.insert(id_key_of(name), tag.clone());
    }
}

fn stack_prov(env: &Env) -> Tag {
    env.scopes.last().map_or(Tag::Bottom, |scope| Tag::Stack(scope.id))
}

/// A binding of a value without provenance lives on the stack of its scope.
fn bind_prov(env: &Env, init: Tag) -> Tag {
    if init == Tag::Bottom {
        stack_prov(env)
    } else {
        init
    }
}

fn pat_names(pattern: &ast::PatternPtr) -> Vec<IdKey> {
    let mut names = Vec::new();
    if let Some(pattern) = pattern.as_deref() {
        collect_pat_names(pattern, &mut names);
    }
    names
}

/// The environment of a pattern arm: its names bound on the stack.
fn case_env(env: &Env, pattern: &ast::PatternPtr) -> Env {
    let mut out = env.clone();
    let tag = bind_prov(&out, Tag::Bottom);
    for name in pat_names(pattern) {
        intro_pi(&mut out, &name, &tag);
    }
    out
}

fn alloc_tag(env: &Env, target: &Option<String>) -> Option<IdKey> {
    match target {
        None => env.regions.last().map(|entry| entry.tag.clone()),
        Some(target) => {
            let key = id_key_of(target);
            env.regions.iter().rev().find(|entry| entry.target == key).map(|entry| entry.tag.clone())
        }
    }
}

/// What an escaping value must outlive: the innermost frame, or else the stack.
fn frame_prov(env: &Env) -> Tag {
    match env.regions.iter().rev().find(|entry| entry.frame_active) {
        Some(entry) => Tag::Region(entry.tag.clone()),
        None => stack_prov(env),
    }
}

fn region_active_type_ref() -> TypeRef {
    make_type_perm(Permission::Unique, make_type_modal_state(vec!["Region".to_string()], "Active", Vec::new()))
}

fn is_heap_allocator_type(ty: &TypeRef) -> bool {
    matches!(strip_perm(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Dynamic(path)) if path.len() == 1 && id_eq(&path[0], "HeapAllocator"))
}

fn is_managed_heap_value_type(ty: &TypeRef) -> bool {
    match strip_perm(ty).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::String(state)) => *state == Some(StringState::Managed),
        Some(TypeNode::Bytes(state)) => *state == Some(BytesState::Managed),
        Some(TypeNode::Union(members)) => members.iter().any(is_managed_heap_value_type),
        _ => false,
    }
}

/// A callable that takes the heap allocator and returns a managed value allocates.
fn callable_introduces_heap_provenance(callable_type: &TypeRef) -> bool {
    match strip_perm(callable_type).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Func { params, ret }) => params.iter().any(|param| is_heap_allocator_type(&param.r#type)) && is_managed_heap_value_type(ret),
        Some(TypeNode::Closure { params, ret, .. }) => params.iter().any(|(_, ty)| is_heap_allocator_type(ty)) && is_managed_heap_value_type(ret),
        _ => false,
    }
}

/// The type of a callee named directly, looked up as a value of its module.
fn direct_callee_type(ctx: &ScopeContext<'_>, callee: &ExprPtr) -> Option<TypeRef> {
    let local = |name: &str| {
        let found = value_path_type(ctx, &ctx.current_module, name);
        (found.ok && found.r#type.is_some()).then_some(found.r#type)
    };
    let (origin, target_name, fallback_name) = match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => match resolve_value_name(ctx, &node.name) {
            Some(entity) if entity.origin_opt.is_some() => {
                (entity.origin_opt.unwrap_or_default(), entity.target_opt.unwrap_or_else(|| node.name.clone()), Some(&node.name))
            }
            _ => return local(&node.name),
        },
        ExprNode::PathExpr(node) => (node.path.clone(), node.name.clone(), node.path.is_empty().then_some(&node.name)),
        ExprNode::QualifiedNameExpr(node) => (node.path.clone(), node.name.clone(), None),
        _ => return None,
    };
    let found = value_path_type(ctx, &origin, &target_name);
    if found.ok && found.r#type.is_some() {
        return Some(found.r#type);
    }
    fallback_name.and_then(|name| local(name))
}

fn is_fresh_region_expr(expr: &ExprPtr) -> bool {
    let region_new_scoped = |path: &[String], name: &str| path.len() == 1 && id_eq(&path[0], "Region") && id_eq(name, "new_scoped");
    match expr.as_deref().map(|expr| &expr.node) {
        Some(ExprNode::QualifiedApplyExpr(node)) => region_new_scoped(&node.path, &node.name),
        Some(ExprNode::CallExpr(node)) => {
            matches!(node.callee.as_deref().map(|callee| &callee.node), Some(ExprNode::PathExpr(path)) if region_new_scoped(&path.path, &path.name))
        }
        Some(ExprNode::AttributedExpr(node)) => is_fresh_region_expr(&node.expr),
        _ => false,
    }
}

fn ast_region_active_type(ty: &ast::TypePtr) -> bool {
    let mut current = ty.as_deref();
    while let Some(ty) = current {
        match &ty.node {
            ast::TypeNode::TypePermType(node) => current = node.base.as_deref(),
            ast::TypeNode::TypeRefine(node) => current = node.base.as_deref(),
            ast::TypeNode::TypeModalState(node) => {
                return node.path.last().is_some_and(|last| id_eq(last, "Region")) && id_eq(&node.state, "Active");
            }
            _ => return false,
        }
    }
    false
}

fn strip_perm_once(ty: &TypeRef) -> TypeRef {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => base.clone(),
        _ => ty.clone(),
    }
}

fn param_type_map(ctx: &ScopeContext<'_>, params: &[ast::Param], self_param: Option<&BindSelfParam>, env: &mut TypeEnv) {
    if env.scopes.is_empty() {
        env.scopes.push(Default::default());
    }
    let self_base = self_param.map(|self_param| strip_perm_once(&self_param.r#type));
    if let Some(self_param) = self_param {
        let binding = TypeBinding { r#type: self_param.r#type.clone(), ..Default::default() };
        env.scopes.last_mut().expect("a scope").insert(id_key_of("self"), binding);
    }
    for param in params {
        let Ok(lowered) = lower_type(ctx, &param.r#type) else {
            continue;
        };
        let ty = match &self_base {
            Some(self_base) if self_base.is_some() => subst_self_type(self_base, &lowered, None),
            _ => lowered,
        };
        env.scopes.last_mut().expect("a scope").insert(id_key_of(&param.name), TypeBinding { r#type: ty, ..Default::default() });
    }
}

/// The names a pattern binds with their types; when the pattern does not fit the type,
/// each name gets the whole type.
fn pattern_bindings(ctx: &ScopeContext<'_>, pattern: &ast::PatternPtr, expected: &TypeRef) -> Vec<(String, TypeRef)> {
    if pattern.is_none() || expected.is_none() {
        return Vec::new();
    }
    match type_pattern(ctx, pattern, expected) {
        Ok(bindings) => bindings,
        Err(_) => pat_names(pattern).into_iter().map(|name| (name, expected.clone())).collect(),
    }
}

/// The statics of a module: name, type and mutability.
fn static_bindings(ctx: &ScopeContext<'_>, module_path: &[String]) -> Vec<(String, TypeRef, ast::Mutability)> {
    let key = path_key_of(module_path);
    let Some(module) = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in &module.items {
        let ast::ASTItem::StaticDecl(decl) = item else {
            continue;
        };
        let ann_type = ast::binding_annotation_type_opt(&decl.binding);
        if ann_type.is_none() {
            continue;
        }
        let Ok(ann) = lower_type(ctx, &ann_type) else {
            continue;
        };
        out.extend(pattern_bindings(ctx, &decl.binding.pat, &ann).into_iter().map(|(name, ty)| (name, ty, decl.r#mut)));
    }
    out
}

/// The environments a body starts in: statics outermost and global, then parameters.
fn initial_envs(
    ctx: &ScopeContext<'_>,
    module_path: &[String],
    params: &[ast::Param],
    self_param: Option<&BindSelfParam>,
    param_tags: Vec<(String, Tag)>,
    regions: Vec<RegionEntry>,
) -> (Env, TypeEnv) {
    let mut env = Env::default();
    let mut scope = Scope { id: 0, map: HashMap::new() };
    env.next_scope_id = 1;
    for (name, tag) in param_tags {
        scope.map.entry(id_key_of(&name)).or_insert(tag);
    }
    env.scopes.push(scope);
    env.regions = regions;
    let statics = static_bindings(ctx, module_path);
    let mut gamma = TypeEnv::default();
    gamma.scopes.push(Default::default());
    if !statics.is_empty() {
        let mut static_scope = Scope { id: env.next_scope_id, map: HashMap::new() };
        env.next_scope_id += 1;
        for (name, ty, r#mut) in &statics {
            static_scope.map.entry(id_key_of(name)).or_insert(Tag::Global);
            gamma.scopes[0].insert(id_key_of(name), TypeBinding { r#mut: *r#mut, r#type: ty.clone(), ..Default::default() });
        }
        env.scopes.insert(0, static_scope);
    }
    gamma.scopes.push(Default::default());
    param_type_map(ctx, params, self_param, &mut gamma);
    (env, gamma)
}

/// The names a closure takes from the bindings around it.
struct ClosureCaptureCollector<'e> {
    env: &'e TypeEnv,
    local_scopes: Vec<HashSet<IdKey>>,
    captures: UnorderedMap<()>,
}

impl ClosureCaptureCollector<'_> {
    fn declare_name(&mut self, name: &str) {
        if self.local_scopes.is_empty() {
            self.local_scopes.push(HashSet::new());
        }
        self.local_scopes.last_mut().expect("a scope").insert(id_key_of(name));
    }

    fn declare_pattern(&mut self, pattern: &ast::PatternPtr) {
        for name in pat_names(pattern).into_iter().filter(|name| !name.is_empty()) {
            self.declare_name(&name);
        }
    }

    fn capture_if_outer(&mut self, name: &str) {
        let key = id_key_of(name);
        if self.local_scopes.iter().any(|scope| scope.contains(&key)) {
            return;
        }
        if bind_of(self.env, name).is_some() {
            self.captures.emplace(key, ());
        }
    }

    fn scoped(&mut self, visit: impl FnOnce(&mut Self)) {
        self.local_scopes.push(HashSet::new());
        visit(self);
        self.local_scopes.pop();
    }

    fn visit_key_path(&mut self, path: &ast::KeyPathExpr) {
        self.capture_if_outer(&path.root);
        for seg in &path.segs {
            if let ast::KeySeg::KeySegIndex(index) = seg {
                self.visit_expr(&index.expr);
            }
        }
    }

    fn visit_block(&mut self, block: &ast::BlockPtr) {
        let Some(block) = block.as_deref() else {
            return;
        };
        self.scoped(|collector| {
            for stmt in &block.stmts {
                collector.visit_stmt(stmt);
            }
            collector.visit_expr(&block.tail_opt);
        });
    }

    fn visit_all<'x>(&mut self, exprs: impl IntoIterator<Item = &'x ExprPtr>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(node) => {
                self.visit_expr(&node.binding.init);
                self.declare_pattern(&node.binding.pat);
            }
            Stmt::VarStmt(node) => {
                self.visit_expr(&node.binding.init);
                self.declare_pattern(&node.binding.pat);
            }
            Stmt::UsingLocalStmt(node) => self.declare_name(&node.alias),
            Stmt::AssignStmt(node) => self.visit_all([&node.place, &node.value]),
            Stmt::CompoundAssignStmt(node) => self.visit_all([&node.place, &node.value]),
            Stmt::ExprStmt(node) => self.visit_expr(&node.value),
            Stmt::DeferStmt(node) => self.visit_block(&node.body),
            Stmt::RegionStmt(node) => {
                self.visit_expr(&node.opts_opt);
                self.scoped(|collector| {
                    if let Some(alias) = &node.alias_opt {
                        collector.declare_name(alias);
                    }
                    collector.visit_block(&node.body);
                });
            }
            Stmt::FrameStmt(node) => {
                if let Some(target) = &node.target_opt {
                    self.capture_if_outer(target);
                }
                self.visit_block(&node.body);
            }
            Stmt::ReturnStmt(node) => self.visit_expr(&node.value_opt),
            Stmt::BreakStmt(node) => self.visit_expr(&node.value_opt),
            Stmt::KeyBlockStmt(node) => {
                for path in &node.paths {
                    self.visit_key_path(path);
                }
                self.visit_block(&node.body);
            }
            Stmt::UnsafeBlockStmt(node) => self.visit_block(&node.body),
            _ => {}
        }
    }

    fn visit_expr(&mut self, expr: &ExprPtr) {
        let Some(e) = expr.as_deref() else {
            return;
        };
        let invariant = |invariant: &Option<ast::LoopInvariant>| invariant.as_ref().map(|invariant| invariant.predicate.clone());
        match &e.node {
            ExprNode::IdentifierExpr(node) => self.capture_if_outer(&node.name),
            ExprNode::IfExpr(node) => self.visit_all([&node.cond, &node.then_expr, &node.else_expr]),
            ExprNode::IfCaseExpr(node) => {
                self.visit_expr(&node.scrutinee);
                for clause in &node.cases {
                    self.scoped(|collector| {
                        collector.declare_pattern(&clause.pattern);
                        collector.visit_expr(&clause.body);
                    });
                }
                self.visit_expr(&node.else_expr);
            }
            ExprNode::IfIsExpr(node) => {
                self.visit_expr(&node.scrutinee);
                self.scoped(|collector| {
                    collector.declare_pattern(&node.pattern);
                    collector.visit_expr(&node.then_expr);
                });
                self.visit_expr(&node.else_expr);
            }
            ExprNode::LoopInfiniteExpr(node) => {
                self.visit_all(invariant(&node.invariant_opt).iter());
                self.visit_block(&node.body);
            }
            ExprNode::LoopConditionalExpr(node) => {
                self.visit_expr(&node.cond);
                self.visit_all(invariant(&node.invariant_opt).iter());
                self.visit_block(&node.body);
            }
            ExprNode::LoopIterExpr(node) => {
                self.visit_expr(&node.iter);
                self.scoped(|collector| {
                    collector.declare_pattern(&node.pattern);
                    collector.visit_all(invariant(&node.invariant_opt).iter());
                    collector.visit_block(&node.body);
                });
            }
            ExprNode::BlockExpr(node) => self.visit_block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.visit_block(&node.block),
            ExprNode::AttributedExpr(node) => self.visit_expr(&node.expr),
            ExprNode::ClosureExpr(node) => self.scoped(|collector| {
                for param in &node.params {
                    collector.declare_name(&param.name);
                }
                collector.visit_expr(&node.body);
            }),
            ExprNode::PipelineExpr(node) => self.visit_all([&node.lhs, &node.rhs]),
            ExprNode::EntryExpr(node) => self.visit_expr(&node.expr),
            ExprNode::YieldExpr(node) => self.visit_expr(&node.value),
            ExprNode::YieldFromExpr(node) => self.visit_expr(&node.value),
            ExprNode::SyncExpr(node) => self.visit_expr(&node.value),
            ExprNode::RaceExpr(node) => {
                for arm in &node.arms {
                    self.visit_expr(&arm.expr);
                    if arm.pattern.is_some() {
                        self.scoped(|collector| {
                            collector.declare_pattern(&arm.pattern);
                            collector.visit_expr(&arm.handler.value);
                        });
                    } else {
                        self.visit_expr(&arm.handler.value);
                    }
                }
            }
            ExprNode::AllExpr(node) => self.visit_all(&node.exprs),
            ExprNode::ParallelExpr(node) => {
                self.visit_expr(&node.domain);
                self.visit_all(node.opts.iter().map(|opt| &opt.value));
                self.visit_block(&node.body);
            }
            ExprNode::SpawnExpr(node) => {
                self.visit_all(node.opts.iter().map(|opt| &opt.value));
                self.visit_block(&node.body);
            }
            ExprNode::WaitExpr(node) => self.visit_expr(&node.handle),
            ExprNode::DispatchExpr(node) => {
                self.visit_expr(&node.range);
                if let Some(key_clause) = &node.key_clause {
                    self.visit_key_path(&key_clause.key_path);
                }
                for opt in &node.opts {
                    self.visit_all([&opt.chunk_expr, &opt.workgroup_expr]);
                }
                self.scoped(|collector| {
                    collector.declare_pattern(&node.pattern);
                    collector.visit_block(&node.body);
                });
            }
            _ => self.visit_all(children_ltr(e)),
        }
    }
}

/// The sub-expressions evaluated in order, for the forms without a rule of their own.
fn children_ltr(expr: &ast::Expr) -> Vec<&ExprPtr> {
    match &expr.node {
        ExprNode::QualifiedApplyExpr(node) => match &node.args {
            ast::ApplyArgs::ParenArgs(args) => args.args.iter().map(|arg| &arg.value).collect(),
            ast::ApplyArgs::BraceArgs(args) => args.fields.iter().map(|field| &field.value).collect(),
        },
        ExprNode::RangeExpr(node) => [&node.lhs, &node.rhs].into_iter().filter(|side| side.is_some()).collect(),
        ExprNode::BinaryExpr(node) => vec![&node.lhs, &node.rhs],
        ExprNode::CastExpr(node) => vec![&node.value],
        ExprNode::UnaryExpr(node) => vec![&node.value],
        ExprNode::DerefExpr(node) => vec![&node.value],
        ExprNode::AllocExpr(node) => vec![&node.value],
        ExprNode::TransmuteExpr(node) => vec![&node.value],
        ExprNode::PropagateExpr(node) => vec![&node.value],
        ExprNode::AddressOfExpr(node) => vec![&node.place],
        ExprNode::MoveExpr(node) => vec![&node.place],
        ExprNode::TupleExpr(node) => node.elements.iter().collect(),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node),
        ExprNode::ArrayRepeatExpr(node) => vec![&node.value, &node.count],
        ExprNode::RecordExpr(node) => node.fields.iter().map(|field| &field.value).collect(),
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            Some(ast::EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().collect(),
            Some(ast::EnumPayload::EnumPayloadBrace(payload)) => payload.fields.iter().map(|field| &field.value).collect(),
            None => Vec::new(),
        },
        ExprNode::FieldAccessExpr(node) => vec![&node.base],
        ExprNode::TupleAccessExpr(node) => vec![&node.base],
        ExprNode::IndexAccessExpr(node) => vec![&node.base, &node.index],
        ExprNode::CallExpr(node) => std::iter::once(&node.callee).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::MethodCallExpr(node) => std::iter::once(&node.receiver).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        _ => Vec::new(),
    }
}

thread_local! {
    /// The procedures whose bodies are being followed for the provenance of a call.
    static CALL_RETURN_STACK: RefCell<Vec<(Vec<IdKey>, usize)>> = const { RefCell::new(Vec::new()) };
}

struct Prov<'d> {
    diags: Option<&'d Rc<RefCell<DiagnosticStream>>>,
}

impl Prov<'_> {
    fn expr_type(&self, ctx: &ScopeContext<'_>, gamma: &TypeEnv, expr: &ExprPtr, infer: bool) -> Option<TypeRef> {
        expr.as_ref()?;
        if let Some(cached) = stored_expr_type(ctx, expr) {
            return Some(cached);
        }
        if !infer {
            return None;
        }
        let typed = type_expr(ctx, &StmtTypeContext::default(), expr, gamma);
        (typed.ok && typed.r#type.is_some()).then_some(typed.r#type)
    }

    fn binding_type(&self, ctx: &ScopeContext<'_>, binding: &ast::Binding, gamma: &TypeEnv, infer: bool) -> Option<TypeRef> {
        let ann_type = ast::binding_annotation_type_opt(binding);
        if ann_type.is_some() {
            return lower_type(ctx, &ann_type).ok();
        }
        if let Some(cached) = stored_expr_type(ctx, &binding.init) {
            return Some(cached);
        }
        if !infer {
            return None;
        }
        let type_ctx = StmtTypeContext { diags: Some(Rc::new(RefCell::new(Vec::new()))), ..Default::default() };
        let type_expr_fn = |expr: &ExprPtr| type_expr(ctx, &type_ctx, expr, gamma);
        let type_ident_fn = |name: &str| type_identifier_expr(ctx, gamma, name);
        let inferred = infer_expr(ctx, &binding.init, &type_expr_fn, &type_ident_fn);
        inferred.ok.then_some(inferred.r#type)
    }

    fn async_sig_from_type(&self, ctx: &ScopeContext<'_>, ty: &TypeRef) -> Option<AsyncSig> {
        ty.as_ref()?;
        let stripped = strip_perm(ty);
        match &stripped.as_deref()?.node {
            TypeNode::Func { ret, .. } => async_sig_of(ctx, ret),
            TypeNode::Closure { ret, .. } => async_sig_of(ctx, ret),
            _ => async_sig_of(ctx, &stripped),
        }
    }

    fn async_sig_for_expr(&self, ctx: &ScopeContext<'_>, gamma: &TypeEnv, expr: &ExprPtr, infer: bool) -> Option<AsyncSig> {
        let e = expr.as_deref()?;
        // Unlike elsewhere, a recorded type that is not asynchronous is final.
        let typed_sig = |inner: &ExprPtr| match stored_expr_type(ctx, inner) {
            Some(cached) => self.async_sig_from_type(ctx, &cached),
            None if infer => {
                let typed = type_expr(ctx, &StmtTypeContext::default(), inner, gamma);
                (typed.ok && typed.r#type.is_some()).then(|| self.async_sig_from_type(ctx, &typed.r#type)).flatten()
            }
            None => None,
        };
        if let Some(sig) = typed_sig(expr) {
            return Some(sig);
        }
        let ExprNode::CallExpr(call) = &e.node else {
            return None;
        };
        typed_sig(&call.callee).or_else(|| direct_callee_type(ctx, &call.callee).and_then(|ty| self.async_sig_from_type(ctx, &ty)))
    }

    fn call_introduces_heap(&self, ctx: &ScopeContext<'_>, gamma: &TypeEnv, call: &ast::CallExpr, infer: bool) -> bool {
        self.expr_type(ctx, gamma, &call.callee, infer).is_some_and(|ty| callable_introduces_heap_provenance(&ty))
            || direct_callee_type(ctx, &call.callee).is_some_and(|ty| callable_introduces_heap_provenance(&ty))
    }

    fn method_call_introduces_heap(&self, ctx: &ScopeContext<'_>, gamma: &TypeEnv, call: &ast::MethodCallExpr, infer: bool) -> bool {
        let Some(receiver_type) = self.expr_type(ctx, gamma, &call.receiver, infer) else {
            return false;
        };
        let receiver_base = strip_perm(&receiver_type);
        if receiver_base.is_none() {
            return false;
        }
        if let Some(sig) = lookup_string_bytes_builtin_method_sig(&receiver_base, &call.name) {
            return sig.params.iter().any(|param| is_heap_allocator_type(&param.r#type)) && is_managed_heap_value_type(&sig.ret);
        }
        if is_heap_allocator_type(&receiver_base) {
            return lookup_heap_allocator_method_sig(&call.name).is_some() && id_eq(&call.name, "alloc_raw");
        }
        let Ok(lookup) = lookup_method_static(ctx, &receiver_base, &call.name) else {
            return false;
        };
        let lowered = |params: &[ast::Param], return_type_opt: &ast::TypePtr| {
            let mut takes_heap = false;
            for param in params {
                match lower_type(ctx, &param.r#type) {
                    Ok(ty) if ty.is_some() => takes_heap |= is_heap_allocator_type(&ty),
                    _ => return false,
                }
            }
            let _ = lower_param_mode;
            let ret = if return_type_opt.is_some() {
                match lower_type(ctx, return_type_opt) {
                    Ok(ty) if ty.is_some() => ty,
                    _ => return false,
                }
            } else {
                make_type_prim("()")
            };
            takes_heap && is_managed_heap_value_type(&ret)
        };
        if let Some(method) = lookup.record_method {
            return lowered(&method.params, &method.return_type_opt);
        }
        lookup.class_method.is_some_and(|method| lowered(&method.params, &method.return_type_opt))
    }

    fn place(&self, ctx: &ScopeContext<'_>, place: &ExprPtr, env: &Env, gamma: &TypeEnv, infer: bool) -> ExprResult {
        match place.as_deref().map(|place| &place.node) {
            Some(ExprNode::IdentifierExpr(node)) => Ok(lookup_pi(env, &node.name).unwrap_or_default()),
            Some(ExprNode::FieldAccessExpr(node)) => self.place(ctx, &node.base, env, gamma, infer),
            Some(ExprNode::TupleAccessExpr(node)) => self.place(ctx, &node.base, env, gamma, infer),
            Some(ExprNode::IndexAccessExpr(node)) => self.place(ctx, &node.base, env, gamma, infer),
            Some(ExprNode::DerefExpr(node)) => self.expr(ctx, &node.value, env, gamma, infer),
            _ => Ok(Tag::Bottom),
        }
    }

    fn stmt_seq(&self, ctx: &ScopeContext<'_>, stmts: &[Stmt], env: Env, gamma: TypeEnv, infer: bool) -> Result<(Env, TypeEnv, Flow), Fail> {
        let mut current_env = env;
        let mut current_gamma = gamma;
        let mut merged = Flow::default();
        for stmt in stmts {
            let flow = self.stmt(ctx, stmt, &mut current_env, &mut current_gamma, infer)?;
            merged.results.extend(flow.results);
            merged.breaks.extend(flow.breaks);
            merged.break_void |= flow.break_void;
        }
        Ok((current_env, current_gamma, merged))
    }

    fn inner_scope(env: &Env, gamma: &TypeEnv) -> (Env, TypeEnv) {
        let mut inner_env = env.clone();
        push_scope(&mut inner_env);
        let mut inner_gamma = gamma.clone();
        inner_gamma.scopes.push(Default::default());
        (inner_env, inner_gamma)
    }

    /// A block's provenance: that of what it returns, or else of its tail.
    fn block(&self, ctx: &ScopeContext<'_>, block: &ast::Block, env: &Env, gamma: &TypeEnv, infer: bool) -> ExprResult {
        let (inner_env, inner_gamma) = Self::inner_scope(env, gamma);
        let (seq_env, seq_gamma, flow) = self.stmt_seq(ctx, &block.stmts, inner_env, inner_gamma, infer)?;
        let tail = if block.tail_opt.is_some() { Some(self.expr(ctx, &block.tail_opt, &seq_env, &seq_gamma, infer)?) } else { None };
        if !flow.results.is_empty() {
            return Ok(join_all_prov(&seq_env, &flow.results));
        }
        Ok(tail.unwrap_or_default())
    }

    fn block_ptr(&self, ctx: &ScopeContext<'_>, block: &ast::BlockPtr, env: &Env, gamma: &TypeEnv, infer: bool) -> ExprResult {
        match block.as_deref() {
            Some(block) => self.block(ctx, block, env, gamma, infer),
            None => Ok(Tag::Bottom),
        }
    }

    /// The provenance of a loop: the join of what its `break`s carry. The body is
    /// walked with inference allowed.
    fn loop_prov(&self, ctx: &ScopeContext<'_>, body: &ast::BlockPtr, body_env: &Env, env: &Env, gamma: &TypeEnv) -> ExprResult {
        let Some(body) = body.as_deref() else {
            return Ok(Tag::Bottom);
        };
        let (inner_env, inner_gamma) = Self::inner_scope(body_env, gamma);
        let (seq_env, seq_gamma, flow) = self.stmt_seq(ctx, &body.stmts, inner_env, inner_gamma, true)?;
        if body.tail_opt.is_some() {
            self.expr(ctx, &body.tail_opt, &seq_env, &seq_gamma, true)?;
        }
        if flow.breaks.is_empty() || flow.break_void {
            return Ok(Tag::Bottom);
        }
        Ok(join_all_prov(env, &flow.breaks))
    }

    fn exprs(&self, ctx: &ScopeContext<'_>, exprs: &[&ExprPtr], env: &Env, gamma: &TypeEnv, infer: bool) -> Result<(), Fail> {
        for expr in exprs {
            self.expr(ctx, expr, env, gamma, infer)?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn closure(&self, ctx: &ScopeContext<'_>, expr: &ExprPtr, e: &ast::Expr, closure: &ast::ClosureExpr, env: &Env, gamma: &TypeEnv, infer: bool) -> ExprResult {
        let (mut body_env, mut body_gamma) = Self::inner_scope(env, gamma);
        let param_prov = bind_prov(&body_env, Tag::Bottom);
        let cached = stored_expr_type(ctx, expr).map(|ty| strip_perm(&ty));
        for (index, param) in closure.params.iter().enumerate() {
            intro_pi(&mut body_env, &param.name, &param_prov);
            let written = if param.type_opt.is_some() { lower_type(ctx, &param.type_opt).ok().filter(|ty| ty.is_some()) } else { None };
            let param_type = written.or_else(|| match cached.as_ref()?.as_deref().map(|ty| &ty.node) {
                Some(TypeNode::Func { params, .. }) => params.get(index).map(|param| param.r#type.clone()),
                Some(TypeNode::Closure { params, .. }) => params.get(index).map(|(_, ty)| ty.clone()),
                _ => None,
            });
            if let Some(param_type) = param_type {
                let scope = body_gamma.scopes.last_mut().expect("a scope");
                scope.insert(id_key_of(&param.name), TypeBinding { r#type: param_type, ..Default::default() });
            }
        }
        self.expr(ctx, &closure.body, &body_env, &body_gamma, infer)?;
        let mut collector = ClosureCaptureCollector { env: gamma, local_scopes: vec![HashSet::new()], captures: UnorderedMap::default() };
        collector.scoped(|collector| {
            for param in &closure.params {
                collector.declare_name(&param.name);
            }
            collector.visit_expr(&closure.body);
        });
        let captures: Vec<IdKey> = collector.captures.keys().cloned().collect();
        if captures.is_empty() {
            return Ok(Tag::Global);
        }
        // A closure that declares its shared dependencies may outlive the frame.
        let escaping = matches!(cached.as_ref().and_then(|ty| ty.as_deref()).map(|ty| &ty.node), Some(TypeNode::Closure { deps_opt: Some(_), .. }));
        let target = if escaping { frame_prov(env) } else { stack_prov(env) };
        let mut capture_provs = Vec::with_capacity(captures.len());
        for capture in &captures {
            let Some(prov) = lookup_pi(env, capture) else {
                continue;
            };
            if escaping {
                // A shared local captured by something that outlives it.
                let key = id_key_of(capture);
                let local_shared = gamma.scopes.iter().enumerate().rev().find_map(|(index, scope)| scope.get(&key).map(|binding| (index, binding)));
                if local_shared.is_some_and(|(index, binding)| index >= 2 && is_shared_binding_type(&binding.r#type)) {
                    return fail("E-CON-0086", e.span.clone());
                }
            }
            if prov_less(env, &prov, &target) {
                return fail("E-CON-0086", e.span.clone());
            }
            capture_provs.push(prov);
        }
        Ok(join_all_prov(env, &capture_provs))
    }

    /// The provenance of a call to an overload typing selected, by following its body
    /// with the arguments' provenance for the parameters.
    fn resolved_call_return(&self, ctx: &ScopeContext<'_>, call: &ast::CallExpr, env: &Env, gamma: &TypeEnv, infer: bool) -> Option<ExprResult> {
        let selected = selected_call_target(ctx, call)?;
        let key = path_key_of(&selected.module_path);
        let module = ctx.sigma.mods.iter().find(|module| path_key_of(&module.path) == key)?;
        let proc = module.items.iter().find_map(|item| match item {
            ast::ASTItem::ProcedureDecl(proc) if proc.name == selected.proc_name && proc.span == selected.proc_span => Some(proc),
            _ => None,
        })?;
        let body = proc.body.as_deref()?;
        if proc.params.len() != call.args.len() {
            return None;
        }
        let frame = (key, std::ptr::from_ref(proc) as usize);
        if CALL_RETURN_STACK.with_borrow(|stack| stack.contains(&frame)) {
            return None;
        }
        let follow = || -> ExprResult {
            if call.callee.is_some() {
                self.expr(ctx, &call.callee, env, gamma, infer)?;
            }
            let mut arg_provs = Vec::with_capacity(call.args.len());
            for arg in &call.args {
                arg_provs.push(self.expr(ctx, &arg_pass_expr(arg), env, gamma, infer)?);
            }
            let mut proc_ctx = ctx.clone();
            proc_ctx.current_module = selected.module_path.clone();
            proc_ctx.scopes.clear();
            let mut param_tags = Vec::with_capacity(proc.params.len());
            let mut regions = Vec::new();
            for (param, arg_prov) in proc.params.iter().zip(arg_provs) {
                let mut tag = arg_prov;
                if lower_type(&proc_ctx, &param.r#type).is_ok_and(|ty| ty.is_some() && region_active_type(&ty)) {
                    if !matches!(tag, Tag::Region(_)) {
                        tag = Tag::Region(id_key_of(&param.name));
                    }
                    if let Tag::Region(region) = &tag {
                        regions.push(RegionEntry { tag: region.clone(), target: id_key_of(&param.name), frame_active: false });
                    }
                }
                param_tags.push((param.name.clone(), tag));
            }
            let (proc_env, proc_gamma) = initial_envs(&proc_ctx, &selected.module_path, &proc.params, None, param_tags, regions);
            CALL_RETURN_STACK.with_borrow_mut(|stack| stack.push(frame.clone()));
            let result = self.block(&proc_ctx, body, &proc_env, &proc_gamma, infer);
            CALL_RETURN_STACK.with_borrow_mut(|stack| stack.pop());
            result
        };
        Some(follow())
    }

    fn expr(&self, ctx: &ScopeContext<'_>, expr: &ExprPtr, env: &Env, gamma: &TypeEnv, infer: bool) -> ExprResult {
        let Some(e) = expr.as_deref() else {
            return Ok(Tag::Bottom);
        };
        match &e.node {
            ExprNode::LiteralExpr(_) => return Ok(Tag::Bottom),
            ExprNode::MoveExpr(node) => return self.place(ctx, &node.place, env, gamma, infer),
            ExprNode::CopyExpr(node) => {
                self.expr(ctx, &node.value, env, gamma, infer)?;
                return Ok(Tag::Bottom);
            }
            ExprNode::AddressOfExpr(node) => return self.place(ctx, &node.place, env, gamma, infer),
            ExprNode::AllocExpr(node) => {
                self.expr(ctx, &node.value, env, gamma, infer)?;
                return match alloc_tag(env, &node.region_opt) {
                    Some(tag) => Ok(Tag::Region(tag)),
                    // An allocation that names no region needs one in scope.
                    None if node.region_opt.is_none() => fail("E-MEM-3021", e.span.clone()),
                    None => Ok(Tag::Bottom),
                };
            }
            ExprNode::MethodCallExpr(call) => {
                let recv = self.expr(ctx, &call.receiver, env, gamma, infer)?;
                // `region~>alloc(value)` allocates in the receiver.
                let receiver = self.expr_type(ctx, gamma, &call.receiver, infer).map(|ty| strip_perm(&ty));
                let allocates = matches!(receiver.as_ref().and_then(|ty| ty.as_deref()).map(|ty| &ty.node),
                    Some(TypeNode::ModalState(modal)) if modal.path.len() == 1 && modal.path[0] == "Region" && id_eq(&modal.state, "Active") && id_eq(&call.name, "alloc"));
                if allocates {
                    self.exprs(ctx, &call.args.iter().map(|arg| &arg.value).collect::<Vec<_>>(), env, gamma, infer)?;
                    return Ok(if matches!(recv, Tag::Region(_)) { recv } else { Tag::Bottom });
                }
            }
            ExprNode::IfExpr(node) => {
                self.expr(ctx, &node.cond, env, gamma, infer)?;
                let then_prov = self.expr(ctx, &node.then_expr, env, gamma, infer)?;
                if node.else_expr.is_none() {
                    return Ok(Tag::Bottom);
                }
                let else_prov = self.expr(ctx, &node.else_expr, env, gamma, infer)?;
                return Ok(join_prov(env, &then_prov, &else_prov));
            }
            ExprNode::IfCaseExpr(node) => {
                self.expr(ctx, &node.scrutinee, env, gamma, infer)?;
                let mut arm_provs = Vec::new();
                for clause in node.cases.iter().filter(|clause| clause.pattern.is_some()) {
                    arm_provs.push(self.expr(ctx, &clause.body, &case_env(env, &clause.pattern), gamma, infer)?);
                }
                if node.else_expr.is_some() {
                    arm_provs.push(self.expr(ctx, &node.else_expr, env, gamma, infer)?);
                }
                return Ok(join_all_prov(env, &arm_provs));
            }
            ExprNode::IfIsExpr(node) => {
                self.expr(ctx, &node.scrutinee, env, gamma, infer)?;
                let mut branch_provs = Vec::new();
                if node.pattern.is_some() && node.then_expr.is_some() {
                    branch_provs.push(self.expr(ctx, &node.then_expr, &case_env(env, &node.pattern), gamma, infer)?);
                }
                if node.else_expr.is_none() {
                    return Ok(Tag::Bottom);
                }
                branch_provs.push(self.expr(ctx, &node.else_expr, env, gamma, infer)?);
                return Ok(join_all_prov(env, &branch_provs));
            }
            ExprNode::BlockExpr(node) => return self.block_ptr(ctx, &node.block, env, gamma, infer),
            ExprNode::UnsafeBlockExpr(node) => return self.block_ptr(ctx, &node.block, env, gamma, infer),
            ExprNode::LoopInfiniteExpr(node) => return self.loop_prov(ctx, &node.body, env, env, gamma),
            ExprNode::LoopConditionalExpr(node) => {
                self.expr(ctx, &node.cond, env, gamma, infer)?;
                return self.loop_prov(ctx, &node.body, env, env, gamma);
            }
            ExprNode::LoopIterExpr(node) => {
                let iter = self.expr(ctx, &node.iter, env, gamma, infer)?;
                let mut arm_env = env.clone();
                for name in pat_names(&node.pattern) {
                    intro_pi(&mut arm_env, &name, &iter);
                }
                return self.loop_prov(ctx, &node.body, &arm_env, env, gamma);
            }
            ExprNode::PipelineExpr(node) => {
                let lhs = self.expr(ctx, &node.lhs, env, gamma, infer)?;
                let rhs = self.expr(ctx, &node.rhs, env, gamma, infer)?;
                return Ok(join_prov(env, &lhs, &rhs));
            }
            ExprNode::ClosureExpr(node) => return self.closure(ctx, expr, e, node, env, gamma, infer),
            _ => {}
        }
        // Creating an asynchronous computation captures its arguments for the frame.
        let async_form = matches!(e.node, ExprNode::CallExpr(_) | ExprNode::MethodCallExpr(_) | ExprNode::RaceExpr(_));
        if async_form && self.async_sig_for_expr(ctx, gamma, expr, infer).is_some() {
            let args: Vec<&ExprPtr> = match &e.node {
                ExprNode::CallExpr(node) => node.args.iter().map(|arg| &arg.value).collect(),
                ExprNode::MethodCallExpr(node) => std::iter::once(&node.receiver).chain(node.args.iter().map(|arg| &arg.value)).collect(),
                ExprNode::RaceExpr(node) => node.arms.iter().map(|arm| &arm.expr).collect(),
                _ => Vec::new(),
            };
            let frame = frame_prov(env);
            for arg in &args {
                let arg_prov = self.expr(ctx, arg, env, gamma, infer)?;
                if prov_less(env, &arg_prov, &frame) {
                    return fail("E-CON-0280", e.span.clone());
                }
            }
            // More than 256 bytes of captures is worth a warning.
            let mut capture_size = 0u64;
            let large = args.iter().any(|arg| {
                let size = self.expr_type(ctx, gamma, arg, true).filter(|ty| ty.is_some()).and_then(|ty| size_of(ctx, &ty));
                capture_size += size.unwrap_or(0);
                capture_size > 256
            });
            if let (true, Some(diags)) = (large, self.diags) {
                if let Some(diag) = make_diagnostic_by_id("W-CON-0201", Some(e.span.clone())) {
                    emit(&mut diags.borrow_mut(), diag);
                }
            }
            return Ok(frame);
        }
        match &e.node {
            ExprNode::CallExpr(call) => {
                if let Some(resolved) = self.resolved_call_return(ctx, call, env, gamma, infer) {
                    return resolved;
                }
                if self.call_introduces_heap(ctx, gamma, call, infer) {
                    self.exprs(ctx, &children_ltr(e), env, gamma, infer)?;
                    return Ok(Tag::Heap);
                }
            }
            ExprNode::MethodCallExpr(call) if self.method_call_introduces_heap(ctx, gamma, call, infer) => {
                self.exprs(ctx, &children_ltr(e), env, gamma, infer)?;
                return Ok(Tag::Heap);
            }
            _ => {}
        }
        if is_place_expr(expr) {
            return self.place(ctx, expr, env, gamma, infer);
        }
        let mut joined: Option<Tag> = None;
        for child in children_ltr(e) {
            let child_prov = self.expr(ctx, child, env, gamma, infer)?;
            joined = Some(match joined {
                Some(current) => join_prov(env, &current, &child_prov),
                None => child_prov,
            });
        }
        Ok(joined.unwrap_or_default())
    }

    fn binding_stmt(&self, ctx: &ScopeContext<'_>, binding: &ast::Binding, r#mut: ast::Mutability, env: &mut Env, gamma: &mut TypeEnv, infer: bool) -> Result<(), Fail> {
        let init = self.expr(ctx, &binding.init, env, gamma, infer)?;
        let names = pat_names(&binding.pat);
        let mut bind_pi = bind_prov(env, init);
        let Some(bind_type) = self.binding_type(ctx, binding, gamma, infer) else {
            for name in &names {
                intro_pi(env, name, &bind_pi);
            }
            return Ok(());
        };
        let binds = pattern_bindings(ctx, &binding.pat, &bind_type);
        let fresh_region = is_fresh_region_expr(&binding.init);
        // A fresh region bound to one name is known by that name.
        if let (true, false, [(name, ty)]) = (fresh_region, matches!(bind_pi, Tag::Region(_)), binds.as_slice()) {
            if region_active_type(ty) {
                bind_pi = Tag::Region(id_key_of(name));
            }
        }
        for name in &names {
            intro_pi(env, name, &bind_pi);
        }
        if gamma.scopes.is_empty() {
            gamma.scopes.push(Default::default());
        }
        for (name, ty) in &binds {
            let scope = gamma.scopes.last_mut().expect("a scope");
            scope.insert(id_key_of(name), TypeBinding { r#mut, r#type: ty.clone(), ..Default::default() });
            if !region_active_type(ty) {
                continue;
            }
            match &bind_pi {
                Tag::Region(region) => env.regions.push(RegionEntry { tag: region.clone(), target: id_key_of(name), frame_active: false }),
                _ if fresh_region => env.regions.push(RegionEntry { tag: id_key_of(name), target: id_key_of(name), frame_active: false }),
                _ => {}
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn assign(&self, ctx: &ScopeContext<'_>, place: &ExprPtr, value: &ExprPtr, span: &Span, env: &Env, gamma: &TypeEnv, infer: bool) -> Result<(), Fail> {
        let place_prov = self.place(ctx, place, env, gamma, infer)?;
        let value_prov = self.expr(ctx, value, env, gamma, infer)?;
        if prov_less(env, &value_prov, &place_prov) {
            let diag_id = if self.async_sig_for_expr(ctx, gamma, value, infer).is_some() { "E-CON-0281" } else { "E-MEM-3020" };
            return fail(diag_id, span.clone());
        }
        Ok(())
    }

    /// A block entered with a region in scope: its binding on the stack of a new scope.
    #[allow(clippy::too_many_arguments)]
    fn region_block(&self, ctx: &ScopeContext<'_>, name: &str, target: &str, frame_active: bool, body: &ast::BlockPtr, env: &Env, gamma: &TypeEnv, infer: bool) -> Result<(), Fail> {
        let (mut inner_env, mut inner_gamma) = Self::inner_scope(env, gamma);
        let bind_pi = bind_prov(&inner_env, Tag::Bottom);
        intro_pi(&mut inner_env, name, &bind_pi);
        inner_env.regions.push(RegionEntry { tag: id_key_of(name), target: id_key_of(target), frame_active });
        let scope = inner_gamma.scopes.last_mut().expect("a scope");
        scope.insert(id_key_of(name), TypeBinding { r#type: region_active_type_ref(), ..Default::default() });
        self.block_ptr(ctx, body, &inner_env, &inner_gamma, infer)?;
        Ok(())
    }

    fn stmt(&self, ctx: &ScopeContext<'_>, stmt: &Stmt, env: &mut Env, gamma: &mut TypeEnv, infer: bool) -> Result<Flow, Fail> {
        let mut flow = Flow::default();
        match stmt {
            Stmt::LetStmt(node) => self.binding_stmt(ctx, &node.binding, ast::Mutability::Let, env, gamma, infer)?,
            Stmt::VarStmt(node) => self.binding_stmt(ctx, &node.binding, ast::Mutability::Var, env, gamma, infer)?,
            Stmt::AssignStmt(node) => self.assign(ctx, &node.place, &node.value, &node.span, env, gamma, infer)?,
            Stmt::CompoundAssignStmt(node) => self.assign(ctx, &node.place, &node.value, &node.span, env, gamma, infer)?,
            Stmt::ExprStmt(node) => {
                self.expr(ctx, &node.value, env, gamma, infer)?;
            }
            Stmt::DeferStmt(node) => {
                self.block_ptr(ctx, &node.body, env, gamma, infer)?;
            }
            Stmt::KeyBlockStmt(node) => {
                self.block_ptr(ctx, &node.body, env, gamma, infer)?;
            }
            Stmt::UnsafeBlockStmt(node) => {
                self.block_ptr(ctx, &node.body, env, gamma, infer)?;
            }
            Stmt::RegionStmt(node) => {
                // Without options the region takes `RegionOptions()`.
                let opts = if node.opts_opt.is_some() {
                    node.opts_opt.clone()
                } else {
                    let callee = ast::Expr { node: ExprNode::IdentifierExpr(ast::IdentifierExpr { name: "RegionOptions".to_string(), ..Default::default() }), ..Default::default() };
                    let call = ast::CallExpr { callee: Some(std::sync::Arc::new(callee)), ..Default::default() };
                    Some(std::sync::Arc::new(ast::Expr { node: ExprNode::CallExpr(call), ..Default::default() }))
                };
                self.expr(ctx, &opts, env, gamma, infer)?;
                let name = node.alias_opt.clone().unwrap_or_else(|| fresh_region_name(gamma));
                self.region_block(ctx, &name, &name, false, &node.body, env, gamma, infer)?;
            }
            Stmt::FrameStmt(node) => {
                let target = match &node.target_opt {
                    None => innermost_active_region(gamma),
                    Some(target) => {
                        let found = bind_of(gamma, target).map(|binding| binding.r#type.clone()).or_else(|| {
                            let entity = resolve_value_name(ctx, target).filter(|entity| entity.origin_opt.is_some())?;
                            let value_type = value_path_type(ctx, &entity.origin_opt.unwrap_or_default(), entity.target_opt.as_deref().unwrap_or(target));
                            (value_type.ok && value_type.r#type.is_some()).then_some(value_type.r#type)
                        });
                        found.filter(region_active_type).map(|_| target.clone())
                    }
                };
                if let Some(target) = target {
                    let fresh = fresh_region_name(gamma);
                    self.region_block(ctx, &fresh, &target, true, &node.body, env, gamma, infer)?;
                }
            }
            Stmt::ReturnStmt(node) if node.value_opt.is_some() => flow.results.push(self.expr(ctx, &node.value_opt, env, gamma, infer)?),
            Stmt::BreakStmt(node) => {
                if node.value_opt.is_some() {
                    flow.breaks.push(self.expr(ctx, &node.value_opt, env, gamma, infer)?);
                } else {
                    flow.break_void = true;
                }
            }
            _ => {}
        }
        Ok(flow)
    }
}

fn is_shared_binding_type(ty: &TypeRef) -> bool {
    let mut current = ty.clone();
    loop {
        match current.as_deref().map(|ty| &ty.node) {
            Some(TypeNode::Refine { base, .. }) => current = base.clone(),
            Some(TypeNode::Perm { perm, .. }) => return *perm == Permission::Shared,
            _ => return false,
        }
    }
}

/// See `ProvBindCheck`.
pub fn prov_bind_check(
    ctx: &ScopeContext<'_>,
    module_path: &[String],
    params: &[ast::Param],
    body: &ast::BlockPtr,
    self_param: Option<&BindSelfParam>,
    diags: Option<&Rc<RefCell<DiagnosticStream>>>,
) -> ProvCheckResult {
    let Some(body) = body.as_deref() else {
        return ProvCheckResult { ok: true, diag_id: None, span: None };
    };
    let mut prov_ctx = ctx.clone();
    prov_ctx.current_module = module_path.to_vec();
    prov_ctx.scopes.clear();
    let mut param_tags: Vec<(String, Tag)> = Vec::new();
    let mut regions = Vec::new();
    if self_param.is_some() {
        param_tags.push(("self".to_string(), Tag::Param(0)));
    }
    for param in params {
        let is_region = lower_type(&prov_ctx, &param.r#type).is_ok_and(|ty| ty.is_some() && region_active_type(&ty)) || ast_region_active_type(&param.r#type);
        let tag = if is_region {
            let region_key = id_key_of(&param.name);
            regions.push(RegionEntry { tag: region_key.clone(), target: region_key.clone(), frame_active: false });
            Tag::Region(region_key)
        } else {
            Tag::Param(param_tags.len())
        };
        param_tags.push((param.name.clone(), tag));
    }
    let (env, gamma) = initial_envs(&prov_ctx, module_path, params, self_param, param_tags, regions);
    // The maps of expression provenance are filled during a type check, so types come
    // from the stores only.
    match (Prov { diags }).block(&prov_ctx, body, &env, &gamma, false) {
        Ok(_) => ProvCheckResult { ok: true, diag_id: None, span: None },
        Err(failure) => ProvCheckResult { ok: false, diag_id: failure.diag_id, span: failure.span },
    }
}
