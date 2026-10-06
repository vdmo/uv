//! What a closure captures from the scopes around it, and whether its body spawns.

use std::collections::HashSet;

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::IdKey;
use crate::resolve::scopes::id_key_of;
use crate::typing::type_env::{bind_of, collect_pat_names, ClosureCaptureInfo, TypeEnv};
use crate::typing::type_predicates::{perm_of_type, strip_perm};
use crate::typing::types::*;

/// Walks an expression with the names it declares itself. With an environment it
/// collects the names used that are bound outside; without one it only looks for a
/// `spawn`, and then does not look inside one.
struct Walker<'e> {
    env: Option<&'e TypeEnv>,
    local_scopes: Vec<HashSet<IdKey>>,
    captures: HashSet<IdKey>,
    found_spawn: bool,
}

impl Walker<'_> {
    fn push_scope(&mut self) {
        self.local_scopes.push(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.local_scopes.pop();
    }

    fn declare_name(&mut self, name: &str) {
        if self.local_scopes.is_empty() {
            self.push_scope();
        }
        if let Some(innermost) = self.local_scopes.last_mut() {
            innermost.insert(id_key_of(name));
        }
    }

    fn declare_pattern(&mut self, pattern: &ast::PatternPtr) {
        let Some(pattern) = pattern.as_deref() else {
            return;
        };
        let mut names = Vec::new();
        collect_pat_names(pattern, &mut names);
        for name in names {
            self.declare_name(&name);
        }
    }

    fn capture_if_outer(&mut self, name: &str) {
        let Some(env) = self.env else {
            return;
        };
        let key = id_key_of(name);
        if self.local_scopes.iter().any(|scope| scope.contains(&key)) {
            return;
        }
        if bind_of(env, name).is_some() {
            self.captures.insert(key);
        }
    }

    fn visit_key_path(&mut self, path: &ast::KeyPathExpr) {
        self.capture_if_outer(&path.root);
        for seg in &path.segs {
            if let ast::KeySeg::KeySegIndex(index) = seg {
                self.visit_expr(&index.expr);
            }
        }
    }

    /// The statements and tail of a block, in the scope the caller set up.
    fn visit_block_contents(&mut self, block: &ast::Block) {
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        self.visit_expr(&block.tail_opt);
    }

    fn visit_block(&mut self, block: &ast::BlockPtr) {
        if let Some(block) = block.as_deref() {
            self.push_scope();
            self.visit_block_contents(block);
            self.pop_scope();
        }
    }

    /// A body whose pattern's names are in scope together with its own bindings.
    fn visit_body_under_pattern(&mut self, pattern: &ast::PatternPtr, before_body: &ExprPtr, body: &ast::BlockPtr) {
        self.push_scope();
        self.declare_pattern(pattern);
        self.visit_expr(before_body);
        if let Some(body) = body.as_deref() {
            self.visit_block_contents(body);
        }
        self.pop_scope();
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        if self.found_spawn {
            return;
        }
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
            Stmt::AssignStmt(node) => {
                self.visit_expr(&node.place);
                self.visit_expr(&node.value);
            }
            Stmt::CompoundAssignStmt(node) => {
                self.visit_expr(&node.place);
                self.visit_expr(&node.value);
            }
            Stmt::ExprStmt(node) => self.visit_expr(&node.value),
            Stmt::DeferStmt(node) => self.visit_block(&node.body),
            Stmt::UnsafeBlockStmt(node) => self.visit_block(&node.body),
            // Only the search for a spawn looks into a compile-time statement.
            Stmt::CtStmt(node) if self.env.is_none() => self.visit_block(&node.body),
            Stmt::RegionStmt(node) => {
                self.visit_expr(&node.opts_opt);
                if let Some(body) = node.body.as_deref() {
                    self.push_scope();
                    if let Some(alias) = &node.alias_opt {
                        self.declare_name(alias);
                    }
                    self.visit_block_contents(body);
                    self.pop_scope();
                }
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
            _ => {}
        }
    }

    fn visit_all<'x>(&mut self, exprs: impl IntoIterator<Item = &'x ExprPtr>) {
        for expr in exprs {
            self.visit_expr(expr);
        }
    }

    fn visit_expr(&mut self, expr: &ExprPtr) {
        if self.found_spawn {
            return;
        }
        let Some(e) = expr.as_deref() else {
            return;
        };
        let collecting = self.env.is_some();
        match &e.node {
            ExprNode::IdentifierExpr(node) => self.capture_if_outer(&node.name),
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::ParenArgs(args) => self.visit_all(args.args.iter().map(|arg| &arg.value)),
                ast::ApplyArgs::BraceArgs(args) => self.visit_all(args.fields.iter().map(|field| &field.value)),
            },
            ExprNode::RangeExpr(node) => self.visit_all([&node.lhs, &node.rhs]),
            ExprNode::BinaryExpr(node) => self.visit_all([&node.lhs, &node.rhs]),
            ExprNode::PipelineExpr(node) => self.visit_all([&node.lhs, &node.rhs]),
            ExprNode::CastExpr(node) => self.visit_expr(&node.value),
            ExprNode::UnaryExpr(node) => self.visit_expr(&node.value),
            ExprNode::DerefExpr(node) => self.visit_expr(&node.value),
            ExprNode::AllocExpr(node) => self.visit_expr(&node.value),
            ExprNode::TransmuteExpr(node) => self.visit_expr(&node.value),
            ExprNode::PropagateExpr(node) => self.visit_expr(&node.value),
            ExprNode::YieldExpr(node) => self.visit_expr(&node.value),
            ExprNode::YieldFromExpr(node) => self.visit_expr(&node.value),
            ExprNode::SyncExpr(node) => self.visit_expr(&node.value),
            ExprNode::EntryExpr(node) => self.visit_expr(&node.expr),
            ExprNode::AttributedExpr(node) => self.visit_expr(&node.expr),
            ExprNode::AddressOfExpr(node) => self.visit_expr(&node.place),
            ExprNode::MoveExpr(node) => self.visit_expr(&node.place),
            ExprNode::TupleExpr(node) => self.visit_all(&node.elements),
            ExprNode::ArrayExpr(node) => self.visit_all(ast::array_expr_subexprs(node)),
            ExprNode::ArrayRepeatExpr(node) => self.visit_all([&node.value, &node.count]),
            ExprNode::RecordExpr(node) => self.visit_all(node.fields.iter().map(|field| &field.value)),
            ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
                Some(ast::EnumPayload::EnumPayloadParen(payload)) => self.visit_all(&payload.elements),
                Some(ast::EnumPayload::EnumPayloadBrace(payload)) => self.visit_all(payload.fields.iter().map(|field| &field.value)),
                None => {}
            },
            ExprNode::IfExpr(node) => self.visit_all([&node.cond, &node.then_expr, &node.else_expr]),
            ExprNode::IfCaseExpr(node) => {
                self.visit_expr(&node.scrutinee);
                for case_clause in &node.cases {
                    self.push_scope();
                    self.declare_pattern(&case_clause.pattern);
                    self.visit_expr(&case_clause.body);
                    self.pop_scope();
                }
                self.visit_expr(&node.else_expr);
            }
            ExprNode::IfIsExpr(node) => {
                self.visit_expr(&node.scrutinee);
                self.push_scope();
                self.declare_pattern(&node.pattern);
                self.visit_expr(&node.then_expr);
                self.pop_scope();
                self.visit_expr(&node.else_expr);
            }
            ExprNode::LoopInfiniteExpr(node) => {
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate);
                }
                self.visit_block(&node.body);
            }
            ExprNode::LoopConditionalExpr(node) => {
                self.visit_expr(&node.cond);
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate);
                }
                self.visit_block(&node.body);
            }
            ExprNode::LoopIterExpr(node) => {
                self.visit_expr(&node.iter);
                let invariant = node.invariant_opt.as_ref().map(|invariant| invariant.predicate.clone()).unwrap_or_default();
                self.visit_body_under_pattern(&node.pattern, &invariant, &node.body);
            }
            ExprNode::BlockExpr(node) => self.visit_block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.visit_block(&node.block),
            ExprNode::ClosureExpr(node) => {
                // A nested closure's parameters are its own.
                self.push_scope();
                for param in &node.params {
                    self.declare_name(&param.name);
                }
                self.visit_expr(&node.body);
                self.pop_scope();
            }
            ExprNode::FieldAccessExpr(node) => self.visit_expr(&node.base),
            ExprNode::TupleAccessExpr(node) => self.visit_expr(&node.base),
            ExprNode::IndexAccessExpr(node) => self.visit_all([&node.base, &node.index]),
            ExprNode::CallExpr(node) => {
                self.visit_expr(&node.callee);
                self.visit_all(node.args.iter().map(|arg| &arg.value));
            }
            ExprNode::MethodCallExpr(node) => {
                self.visit_expr(&node.receiver);
                self.visit_all(node.args.iter().map(|arg| &arg.value));
            }
            ExprNode::RaceExpr(node) => {
                for arm in &node.arms {
                    self.visit_expr(&arm.expr);
                    self.push_scope();
                    self.declare_pattern(&arm.pattern);
                    self.visit_expr(&arm.handler.value);
                    self.pop_scope();
                }
            }
            ExprNode::AllExpr(node) => self.visit_all(&node.exprs),
            ExprNode::ParallelExpr(node) => {
                self.visit_expr(&node.domain);
                self.visit_all(node.opts.iter().map(|opt| &opt.value));
                self.visit_block(&node.body);
            }
            ExprNode::SpawnExpr(node) => {
                if !collecting {
                    self.found_spawn = true;
                    return;
                }
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
                self.visit_body_under_pattern(&node.pattern, &None, &node.body);
            }
            _ => {}
        }
    }
}

/// The names a closure uses that are bound outside it.
fn outer_captures(closure: &ast::ClosureExpr, env: &TypeEnv) -> HashSet<IdKey> {
    let mut walker = Walker { env: Some(env), local_scopes: vec![HashSet::new()], captures: HashSet::new(), found_spawn: false };
    walker.push_scope();
    for param in &closure.params {
        walker.declare_name(&param.name);
    }
    walker.visit_expr(&closure.body);
    walker.captures
}

fn contains_spawn_expr(expr: &ExprPtr) -> bool {
    let mut walker = Walker { env: None, local_scopes: Vec::new(), captures: HashSet::new(), found_spawn: false };
    walker.visit_expr(expr);
    walker.found_spawn
}

/// A closure type that declares the shared data it depends on.
pub fn closure_type_has_shared_deps(hint: &TypeRef) -> bool {
    matches!(strip_perm(hint).as_deref().map(|ty| &ty.node), Some(TypeNode::Closure { deps_opt: Some(_), .. }))
}

/// What a closure expression captures; nothing for any other expression.
pub fn analyze_closure_capture_info(expr: &ExprPtr, env: &TypeEnv, closure_type_hint: &TypeRef) -> Option<ClosureCaptureInfo> {
    let ExprNode::ClosureExpr(closure) = &expr.as_deref()?.node else {
        return None;
    };
    let captures = outer_captures(closure, env);
    let captures_shared = captures
        .iter()
        .filter_map(|captured| bind_of(env, captured))
        .any(|binding| perm_of_type(&binding.r#type) == Permission::Shared);
    Some(ClosureCaptureInfo {
        captures_any: !captures.is_empty(),
        captures_shared,
        has_shared_deps: closure_type_has_shared_deps(closure_type_hint),
        contains_spawn: contains_spawn_expr(&closure.body),
    })
}

/// The capture facts of a value that is a closure or a binding holding one, looking
/// through attributes and `move`.
pub fn closure_capture_info_of_value(expr: &ExprPtr, env: &TypeEnv, closure_type_hint: &TypeRef) -> Option<ClosureCaptureInfo> {
    match &expr.as_deref()?.node {
        ExprNode::AttributedExpr(node) => closure_capture_info_of_value(&node.expr, env, closure_type_hint),
        ExprNode::MoveExpr(node) => closure_capture_info_of_value(&node.place, env, closure_type_hint),
        ExprNode::IdentifierExpr(ident) => {
            let binding = bind_of(env, &ident.name)?;
            let mut info = binding.closure_capture_info?;
            info.has_shared_deps =
                info.has_shared_deps || closure_type_has_shared_deps(&binding.r#type) || closure_type_has_shared_deps(closure_type_hint);
            Some(info)
        }
        _ => analyze_closure_capture_info(expr, env, closure_type_hint),
    }
}

/// A closure handed to something that expects declared shared dependencies must not
/// spawn.
pub fn check_escaping_closure_spawn(expr: &ExprPtr, env: &TypeEnv, expected_closure_type: &TypeRef) -> Option<&'static str> {
    if !closure_type_has_shared_deps(expected_closure_type) {
        return None;
    }
    closure_capture_info_of_value(expr, env, expected_closure_type).filter(|info| info.contains_spawn).map(|_| "E-CON-0131")
}
