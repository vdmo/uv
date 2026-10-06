//! The keys each iteration of a `dispatch` needs: written in a key clause, or inferred
//! from the shared data the body uses. Two uses that may touch the same data, one of
//! them writing, make the iterations conflict.

use std::collections::{HashMap, HashSet};

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use crate::context::IdKey;
use crate::keys::key_paths::{build_key_path, format_tuple_index};
use crate::resolve::scopes::id_key_of;
use crate::typing::type_env::{bind_of, collect_pat_names, TypeEnv};
use crate::typing::type_predicates::perm_of_type;
use crate::typing::types::Permission;

/// One step of an access: a field, or an index with its canonical text.
#[derive(Clone)]
struct SchemaSeg {
    is_index: bool,
    key: String,
    index_expr: ExprPtr,
}

#[derive(Clone)]
struct Schema {
    root: String,
    segs: Vec<SchemaSeg>,
}

pub struct DispatchAccess {
    schema: Schema,
    writes: bool,
}

fn strip_attrs(expr: &ExprPtr) -> &ExprPtr {
    let mut current = expr;
    while let Some(ExprNode::AttributedExpr(attributed)) = current.as_deref().map(|expr| &expr.node) {
        current = &attributed.expr;
    }
    current
}

pub fn dispatch_pattern_name_set(pattern: &ast::PatternPtr) -> HashSet<IdKey> {
    let mut names = Vec::new();
    if let Some(pattern) = pattern.as_deref() {
        collect_pat_names(pattern, &mut names);
    }
    names.into_iter().collect()
}

/// The leading digits of a literal, ignoring `_` separators.
fn parse_dispatch_int_literal(lexeme: &str) -> Option<i64> {
    let digits: String = lexeme.chars().take_while(|ch| ch.is_ascii_digit() || *ch == '_').filter(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// An index of the form `var + k` over an index variable, or a constant.
fn try_affine_dispatch_index(expr: &ExprPtr, pattern_names: &HashSet<IdKey>) -> Option<(Option<IdKey>, i64)> {
    let pattern_var = |name: &str| {
        let key = id_key_of(name);
        pattern_names.contains(&key).then_some((Some(key), 0))
    };
    match &strip_attrs(expr).as_deref()?.node {
        ExprNode::LiteralExpr(node) => parse_dispatch_int_literal(&node.literal.lexeme).map(|value| (None, value)),
        ExprNode::IdentifierExpr(node) => pattern_var(&node.name),
        ExprNode::PathExpr(node) if node.path.is_empty() => pattern_var(&node.name),
        ExprNode::UnaryExpr(node) => {
            let (var, offset) = try_affine_dispatch_index(&node.value, pattern_names)?;
            match node.op.as_str() {
                "+" => Some((var, offset)),
                "-" if var.is_none() => Some((None, offset.wrapping_neg())),
                _ => None,
            }
        }
        ExprNode::BinaryExpr(node) => {
            let (lhs_var, lhs_offset) = try_affine_dispatch_index(&node.lhs, pattern_names)?;
            let (rhs_var, rhs_offset) = try_affine_dispatch_index(&node.rhs, pattern_names)?;
            match (node.op.as_str(), lhs_var, rhs_var) {
                ("+", lhs_var, None) => Some((lhs_var, lhs_offset.wrapping_add(rhs_offset))),
                ("+", None, rhs_var) => Some((rhs_var, lhs_offset.wrapping_add(rhs_offset))),
                ("-", lhs_var, None) => Some((lhs_var, lhs_offset.wrapping_sub(rhs_offset))),
                _ => None,
            }
        }
        ExprNode::CastExpr(node) => try_affine_dispatch_index(&node.value, pattern_names),
        _ => None,
    }
}

/// Whether an index means the same in every iteration apart from the index variables:
/// it is built from literals, index variables and constant bindings only.
fn dispatch_index_expr_invariant(expr: &ExprPtr, pattern_names: &HashSet<IdKey>, env: &TypeEnv, local_names: &HashSet<IdKey>) -> bool {
    let Some(e) = strip_attrs(expr).as_deref() else {
        return false;
    };
    let invariant = |inner: &ExprPtr| dispatch_index_expr_invariant(inner, pattern_names, env, local_names);
    let name_invariant = |name: &str| {
        let key = id_key_of(name);
        if pattern_names.contains(&key) {
            return true;
        }
        !local_names.contains(&key) && bind_of(env, name).is_some_and(|binding| perm_of_type(&binding.r#type) == Permission::Const)
    };
    match &e.node {
        ExprNode::LiteralExpr(_) | ExprNode::QualifiedNameExpr(_) => true,
        ExprNode::IdentifierExpr(node) => name_invariant(&node.name),
        ExprNode::PathExpr(node) => !node.path.is_empty() || name_invariant(&node.name),
        ExprNode::FieldAccessExpr(node) => invariant(&node.base),
        ExprNode::TupleAccessExpr(node) => invariant(&node.base),
        ExprNode::IndexAccessExpr(node) => invariant(&node.base) && invariant(&node.index),
        ExprNode::UnaryExpr(node) => invariant(&node.value),
        ExprNode::BinaryExpr(node) => invariant(&node.lhs) && invariant(&node.rhs),
        ExprNode::CastExpr(node) => invariant(&node.value),
        _ => false,
    }
}

/// Whether a use is a path whose indices are all invariant.
fn dispatch_invariant_key_path_expr(expr: &ExprPtr, pattern_names: &HashSet<IdKey>, env: &TypeEnv, local_names: &HashSet<IdKey>) -> bool {
    let Some(e) = strip_attrs(expr).as_deref() else {
        return false;
    };
    let path = |inner: &ExprPtr| dispatch_invariant_key_path_expr(inner, pattern_names, env, local_names);
    match &e.node {
        ExprNode::IdentifierExpr(_) | ExprNode::PathExpr(_) | ExprNode::QualifiedNameExpr(_) => true,
        ExprNode::FieldAccessExpr(node) => path(&node.base),
        ExprNode::TupleAccessExpr(node) => path(&node.base),
        ExprNode::IndexAccessExpr(node) => path(&node.base) && dispatch_index_expr_invariant(&node.index, pattern_names, env, local_names),
        ExprNode::MethodCallExpr(node) => path(&node.receiver),
        ExprNode::MoveExpr(node) => path(&node.place),
        ExprNode::DerefExpr(node) => path(&node.value),
        _ => false,
    }
}

/// The text two indices share exactly when they are written alike; index variables
/// are marked with `$`.
fn canonical_dispatch_index_expr(expr: &ExprPtr, pattern_names: &HashSet<IdKey>) -> Option<String> {
    let canon = |inner: &ExprPtr| canonical_dispatch_index_expr(inner, pattern_names);
    let name = |name: &str| if pattern_names.contains(&id_key_of(name)) { format!("${name}") } else { name.to_string() };
    Some(match &strip_attrs(expr).as_deref()?.node {
        ExprNode::LiteralExpr(node) => node.literal.lexeme.clone(),
        ExprNode::IdentifierExpr(node) => name(&node.name),
        ExprNode::QualifiedNameExpr(node) => format!("{}::{}", node.path.join("::"), node.name),
        ExprNode::PathExpr(node) if node.path.is_empty() => name(&node.name),
        ExprNode::PathExpr(node) => format!("{}::{}", node.path.join("::"), node.name),
        ExprNode::FieldAccessExpr(node) => format!("{}.{}", canon(&node.base)?, node.name),
        ExprNode::TupleAccessExpr(node) => format!("{}.{}", canon(&node.base)?, format_tuple_index(node.index)),
        ExprNode::IndexAccessExpr(node) => {
            let base = canon(&node.base);
            let index = canon(&node.index);
            format!("{}[{}]", base?, index?)
        }
        ExprNode::UnaryExpr(node) => format!("({}{})", node.op, canon(&node.value)?),
        ExprNode::BinaryExpr(node) => {
            let lhs = canon(&node.lhs);
            let rhs = canon(&node.rhs);
            format!("({}{}{})", lhs?, node.op, rhs?)
        }
        ExprNode::CastExpr(node) => format!("cast({})", canon(&node.value)?),
        _ => return None,
    })
}

fn provably_disjoint_dispatch_index_expr(lhs: &ExprPtr, rhs: &ExprPtr, pattern_names: &HashSet<IdKey>) -> bool {
    if let (Some(lhs_key), Some(rhs_key)) = (canonical_dispatch_index_expr(lhs, pattern_names), canonical_dispatch_index_expr(rhs, pattern_names)) {
        if lhs_key == rhs_key {
            return false;
        }
    }
    match (try_affine_dispatch_index(lhs, pattern_names), try_affine_dispatch_index(rhs, pattern_names)) {
        (Some((lhs_var, lhs_offset)), Some((rhs_var, rhs_offset))) => lhs_var == rhs_var && lhs_offset != rhs_offset,
        _ => false,
    }
}

/// A use as a root and steps, when it is a path that can be described.
fn try_build_dispatch_schema(expr: &ExprPtr, pattern_names: &HashSet<IdKey>) -> Option<Schema> {
    let build = |inner: &ExprPtr| try_build_dispatch_schema(inner, pattern_names);
    let field = |mut base: Schema, key: String| {
        base.segs.push(SchemaSeg { is_index: false, key, index_expr: None });
        base
    };
    match &strip_attrs(expr).as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(Schema { root: node.name.clone(), segs: Vec::new() }),
        ExprNode::PathExpr(node) if node.path.is_empty() => Some(Schema { root: node.name.clone(), segs: Vec::new() }),
        ExprNode::FieldAccessExpr(node) => Some(field(build(&node.base)?, node.name.clone())),
        ExprNode::TupleAccessExpr(node) => Some(field(build(&node.base)?, format_tuple_index(node.index))),
        ExprNode::IndexAccessExpr(node) => {
            let mut base = build(&node.base)?;
            let key = canonical_dispatch_index_expr(&node.index, pattern_names)?;
            base.segs.push(SchemaSeg { is_index: true, key, index_expr: node.index.clone() });
            Some(base)
        }
        ExprNode::MethodCallExpr(node) => build(&node.receiver),
        ExprNode::MoveExpr(node) => build(&node.place),
        ExprNode::DerefExpr(node) => build(&node.value),
        _ => None,
    }
}

/// The access a written key clause declares.
pub fn access_of_key_clause(key_clause: &ast::DispatchKeyClause, pattern_names: &HashSet<IdKey>) -> DispatchAccess {
    let segs = key_clause
        .key_path
        .segs
        .iter()
        .map(|seg| match seg {
            ast::KeySeg::KeySegField(field) => SchemaSeg { is_index: false, key: field.name.clone(), index_expr: None },
            ast::KeySeg::KeySegIndex(index) => SchemaSeg {
                is_index: true,
                key: canonical_dispatch_index_expr(&index.expr, pattern_names).unwrap_or_else(|| "<?>".to_string()),
                index_expr: index.expr.clone(),
            },
        })
        .collect();
    DispatchAccess { schema: Schema { root: key_clause.key_path.root.clone(), segs }, writes: key_clause.mode != ast::KeyMode::Read }
}

fn schema_key(schema: &Schema) -> String {
    let mut key = schema.root.clone();
    for seg in &schema.segs {
        key += &if seg.is_index { format!("[{}]", seg.key) } else { format!(".{}", seg.key) };
    }
    key
}

fn seg_eq_for_dispatch(lhs: &SchemaSeg, rhs: &SchemaSeg, pattern_names: &HashSet<IdKey>) -> bool {
    if lhs.is_index != rhs.is_index {
        return false;
    }
    if !lhs.is_index || lhs.index_expr.is_none() || rhs.index_expr.is_none() {
        return lhs.key == rhs.key;
    }
    match (canonical_dispatch_index_expr(&lhs.index_expr, pattern_names), canonical_dispatch_index_expr(&rhs.index_expr, pattern_names)) {
        (Some(lhs_key), Some(rhs_key)) => lhs_key == rhs_key,
        _ => false,
    }
}

fn segment_provably_disjoint(lhs: &SchemaSeg, rhs: &SchemaSeg, pattern_names: &HashSet<IdKey>) -> bool {
    if lhs.is_index != rhs.is_index {
        return false;
    }
    if !lhs.is_index {
        return lhs.key != rhs.key;
    }
    lhs.index_expr.is_some() && rhs.index_expr.is_some() && provably_disjoint_dispatch_index_expr(&lhs.index_expr, &rhs.index_expr, pattern_names)
}

/// Whether two accesses cannot touch the same data: different roots, or equal steps
/// up to one where they provably part.
fn provably_disjoint_path(lhs: &Schema, rhs: &Schema, pattern_names: &HashSet<IdKey>) -> bool {
    if lhs.root != rhs.root {
        return true;
    }
    for (lhs_seg, rhs_seg) in lhs.segs.iter().zip(&rhs.segs) {
        if segment_provably_disjoint(lhs_seg, rhs_seg, pattern_names) {
            return true;
        }
        if !seg_eq_for_dispatch(lhs_seg, rhs_seg, pattern_names) {
            return false;
        }
    }
    false
}

fn is_static_dispatch_index_expr(expr: &ExprPtr, pattern_names: &HashSet<IdKey>) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    let is_static = |inner: &ExprPtr| is_static_dispatch_index_expr(inner, pattern_names);
    match &e.node {
        ExprNode::LiteralExpr(_) => true,
        ExprNode::IdentifierExpr(node) => pattern_names.contains(&id_key_of(&node.name)),
        ExprNode::PathExpr(node) => node.path.is_empty() && pattern_names.contains(&id_key_of(&node.name)),
        ExprNode::FieldAccessExpr(node) => is_static(&node.base),
        ExprNode::TupleAccessExpr(node) => is_static(&node.base),
        ExprNode::IndexAccessExpr(node) => is_static(&node.base) && is_static(&node.index),
        ExprNode::UnaryExpr(node) => is_static(&node.value),
        ExprNode::BinaryExpr(node) => is_static(&node.lhs) && is_static(&node.rhs),
        ExprNode::CastExpr(node) => is_static(&node.value),
        ExprNode::AttributedExpr(node) => is_static(&node.expr),
        _ => false,
    }
}

/// Whether some index depends on more than literals and the index variables, so that
/// the keys can only be computed at run time.
pub fn dynamic_key_pattern(spec: &[DispatchAccess], pattern_names: &HashSet<IdKey>) -> bool {
    spec.iter()
        .flat_map(|access| &access.schema.segs)
        .any(|seg| seg.is_index && seg.index_expr.is_some() && !is_static_dispatch_index_expr(&seg.index_expr, pattern_names))
}

/// A use of shared data in the body, with the names local to the body where it occurs.
struct CandidateUse {
    expr: ExprPtr,
    writes: bool,
    local_names: HashSet<IdKey>,
}

struct ImplicitUseCollector<'e> {
    env: &'e TypeEnv,
    local_scopes: Vec<HashSet<IdKey>>,
    uses: Vec<CandidateUse>,
}

impl ImplicitUseCollector<'_> {
    fn declare_name(&mut self, name: &str) {
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
        if let Some(innermost) = self.local_scopes.last_mut() {
            innermost.extend(names);
        }
    }

    /// Records the expression when it is a path into shared data bound outside.
    fn record_access(&mut self, expr: &ExprPtr, writes: bool) {
        if expr.is_none() {
            return;
        }
        let built = build_key_path(expr);
        if !built.success || built.path.root.is_empty() {
            return;
        }
        let root = id_key_of(&built.path.root);
        if self.local_scopes.iter().any(|scope| scope.contains(&root)) {
            return;
        }
        if !bind_of(self.env, &built.path.root).is_some_and(|binding| perm_of_type(&binding.r#type) == Permission::Shared) {
            return;
        }
        let local_names = self.local_scopes.iter().flatten().cloned().collect();
        self.uses.push(CandidateUse { expr: expr.clone(), writes, local_names });
    }

    fn visit_scoped_block(&mut self, block: &ast::BlockPtr) {
        if let Some(block) = block.as_deref() {
            self.local_scopes.push(HashSet::new());
            self.visit_block_contents(block);
            self.local_scopes.pop();
        }
    }

    fn visit_block_contents(&mut self, block: &ast::Block) {
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        self.visit_expr(&block.tail_opt, false);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(node) => {
                self.visit_expr(&node.binding.init, false);
                self.declare_pattern(&node.binding.pat);
            }
            Stmt::VarStmt(node) => {
                self.visit_expr(&node.binding.init, false);
                self.declare_pattern(&node.binding.pat);
            }
            Stmt::UsingLocalStmt(node) => self.declare_name(&node.alias),
            Stmt::AssignStmt(node) => {
                self.visit_expr(&node.place, true);
                self.visit_expr(&node.value, false);
            }
            Stmt::CompoundAssignStmt(node) => {
                self.visit_expr(&node.place, true);
                self.visit_expr(&node.value, false);
            }
            Stmt::ExprStmt(node) => self.visit_expr(&node.value, false),
            Stmt::ReturnStmt(node) => self.visit_expr(&node.value_opt, false),
            Stmt::BreakStmt(node) => self.visit_expr(&node.value_opt, false),
            Stmt::DeferStmt(node) => self.visit_scoped_block(&node.body),
            Stmt::UnsafeBlockStmt(node) => self.visit_scoped_block(&node.body),
            Stmt::RegionStmt(node) => self.visit_scoped_block(&node.body),
            Stmt::FrameStmt(node) => self.visit_scoped_block(&node.body),
            // A nested key block names its own keys.
            _ => {}
        }
    }

    fn visit_reads<'x>(&mut self, exprs: impl IntoIterator<Item = &'x ExprPtr>) {
        for expr in exprs {
            self.visit_expr(expr, false);
        }
    }

    fn visit_expr(&mut self, expr: &ExprPtr, writes: bool) {
        let stripped = strip_attrs(expr);
        let Some(e) = stripped.as_deref() else {
            return;
        };
        match &e.node {
            ExprNode::IdentifierExpr(_)
            | ExprNode::PathExpr(_)
            | ExprNode::FieldAccessExpr(_)
            | ExprNode::TupleAccessExpr(_)
            | ExprNode::DerefExpr(_) => self.record_access(stripped, writes),
            ExprNode::IndexAccessExpr(node) => {
                self.record_access(stripped, writes);
                self.visit_expr(&node.index, false);
            }
            ExprNode::MethodCallExpr(node) => {
                self.record_access(stripped, writes);
                self.visit_reads(node.args.iter().map(|arg| &arg.value));
            }
            ExprNode::MoveExpr(node) => self.record_access(&node.place, writes),
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::ParenArgs(args) => self.visit_reads(args.args.iter().map(|arg| &arg.value)),
                ast::ApplyArgs::BraceArgs(args) => self.visit_reads(args.fields.iter().map(|field| &field.value)),
            },
            ExprNode::BinaryExpr(node) => self.visit_reads([&node.lhs, &node.rhs]),
            ExprNode::PipelineExpr(node) => self.visit_reads([&node.lhs, &node.rhs]),
            ExprNode::UnaryExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::CastExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::PropagateExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::AllocExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::TransmuteExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::YieldExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::YieldFromExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::SyncExpr(node) => self.visit_expr(&node.value, false),
            ExprNode::EntryExpr(node) => self.visit_expr(&node.expr, false),
            ExprNode::AddressOfExpr(node) => self.visit_expr(&node.place, false),
            ExprNode::IfExpr(node) => self.visit_reads([&node.cond, &node.then_expr, &node.else_expr]),
            ExprNode::TupleExpr(node) => self.visit_reads(&node.elements),
            ExprNode::ArrayExpr(node) => self.visit_reads(ast::array_expr_subexprs(node)),
            ExprNode::ArrayRepeatExpr(node) => self.visit_reads([&node.value, &node.count]),
            ExprNode::RecordExpr(node) => self.visit_reads(node.fields.iter().map(|field| &field.value)),
            ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
                Some(ast::EnumPayload::EnumPayloadParen(payload)) => self.visit_reads(&payload.elements),
                Some(ast::EnumPayload::EnumPayloadBrace(payload)) => self.visit_reads(payload.fields.iter().map(|field| &field.value)),
                None => {}
            },
            ExprNode::CallExpr(node) => {
                self.visit_expr(&node.callee, false);
                self.visit_reads(node.args.iter().map(|arg| &arg.value));
            }
            ExprNode::IfCaseExpr(node) => {
                self.visit_expr(&node.scrutinee, false);
                for clause in &node.cases {
                    self.local_scopes.push(HashSet::new());
                    self.declare_pattern(&clause.pattern);
                    self.visit_expr(&clause.body, false);
                    self.local_scopes.pop();
                }
                self.visit_expr(&node.else_expr, false);
            }
            ExprNode::IfIsExpr(node) => {
                self.visit_expr(&node.scrutinee, false);
                self.local_scopes.push(HashSet::new());
                self.declare_pattern(&node.pattern);
                self.visit_expr(&node.then_expr, false);
                self.local_scopes.pop();
                self.visit_expr(&node.else_expr, false);
            }
            ExprNode::LoopInfiniteExpr(node) => {
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate, false);
                }
                self.visit_scoped_block(&node.body);
            }
            ExprNode::LoopConditionalExpr(node) => {
                self.visit_expr(&node.cond, false);
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate, false);
                }
                self.visit_scoped_block(&node.body);
            }
            ExprNode::LoopIterExpr(node) => {
                self.visit_expr(&node.iter, false);
                self.local_scopes.push(HashSet::new());
                self.declare_pattern(&node.pattern);
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate, false);
                }
                if let Some(body) = node.body.as_deref() {
                    self.visit_block_contents(body);
                }
                self.local_scopes.pop();
            }
            ExprNode::BlockExpr(node) => self.visit_scoped_block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.visit_scoped_block(&node.block),
            ExprNode::ClosureExpr(node) => {
                self.local_scopes.push(HashSet::new());
                for param in &node.params {
                    self.declare_name(&param.name);
                }
                self.visit_expr(&node.body, false);
                self.local_scopes.pop();
            }
            // Nested tasks account for their own bodies.
            ExprNode::ParallelExpr(node) => {
                self.visit_expr(&node.domain, false);
                self.visit_reads(node.opts.iter().map(|opt| &opt.value));
            }
            ExprNode::SpawnExpr(node) => self.visit_reads(node.opts.iter().map(|opt| &opt.value)),
            ExprNode::WaitExpr(node) => self.visit_expr(&node.handle, false),
            ExprNode::DispatchExpr(node) => {
                self.visit_expr(&node.range, false);
                for opt in &node.opts {
                    self.visit_reads([&opt.chunk_expr, &opt.workgroup_expr]);
                }
            }
            ExprNode::RaceExpr(node) => {
                for arm in &node.arms {
                    self.visit_expr(&arm.expr, false);
                    self.local_scopes.push(HashSet::new());
                    self.declare_pattern(&arm.pattern);
                    self.visit_expr(&arm.handler.value, false);
                    self.local_scopes.pop();
                }
            }
            ExprNode::AllExpr(node) => self.visit_reads(&node.exprs),
            _ => {}
        }
    }
}

/// The accesses a body makes to shared data, one per distinct path; a path both read
/// and written counts as written. `E-CON-0141` when a use cannot be described, and
/// `E-CON-0142` when two accesses may conflict.
pub fn infer_dispatch_accesses(pattern: &ast::PatternPtr, body: &ast::Block, env: &TypeEnv) -> Result<Vec<DispatchAccess>, &'static str> {
    let pattern_names = dispatch_pattern_name_set(pattern);
    let mut collector = ImplicitUseCollector { env, local_scopes: vec![HashSet::new(), HashSet::new()], uses: Vec::new() };
    collector.declare_pattern(pattern);
    collector.visit_block_contents(body);

    let mut merged: Vec<DispatchAccess> = Vec::new();
    let mut by_key: HashMap<String, usize> = HashMap::new();
    for candidate in &collector.uses {
        if !dispatch_invariant_key_path_expr(&candidate.expr, &pattern_names, env, &candidate.local_names) {
            return Err("E-CON-0141");
        }
        let schema = try_build_dispatch_schema(&candidate.expr, &pattern_names).ok_or("E-CON-0141")?;
        match by_key.get(&schema_key(&schema)) {
            Some(&index) => merged[index].writes |= candidate.writes,
            None => {
                by_key.insert(schema_key(&schema), merged.len());
                merged.push(DispatchAccess { schema, writes: candidate.writes });
            }
        }
    }
    for (index, lhs) in merged.iter().enumerate() {
        for rhs in &merged[index + 1..] {
            if (lhs.writes || rhs.writes) && !provably_disjoint_path(&lhs.schema, &rhs.schema, &pattern_names) {
                return Err("E-CON-0142");
            }
        }
    }
    Ok(merged)
}
