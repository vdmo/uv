//! Whether the key accesses a procedure makes through its shared parameters can be
//! known: they cannot when it has no body, or hands such a parameter to a method or to
//! a callee that is itself unknown.
//!
//! The reference also collects the accesses themselves; nothing reads them during
//! typing, so only this answer is computed.

use std::collections::HashSet;

use uv_source::ast::{self, ExprNode, ExprPtr, Stmt};

use super::call::{lookup_procedure_for_callee, type_is_shared_param_surface};
use crate::context::{IdKey, ScopeContext};
use crate::keys::key_paths::build_key_path;
use crate::resolve::scopes::id_key_of;

/// The local names that stand for (a part of) a shared parameter.
type Aliases = HashSet<IdKey>;

pub struct KeyAccessSummaryBuilder<'c, 'x> {
    ctx: &'c ScopeContext<'x>,
    /// Procedures whose summary is being computed; a recursive call is unknown.
    active: Vec<*const ast::ProcedureDecl>,
    cache: Vec<(*const ast::ProcedureDecl, bool)>,
}

struct Walk<'p> {
    shared_params: &'p HashSet<IdKey>,
    unknown: bool,
}

impl<'c, 'x> KeyAccessSummaryBuilder<'c, 'x> {
    pub fn new(ctx: &'c ScopeContext<'x>) -> Self {
        Self { ctx, active: Vec::new(), cache: Vec::new() }
    }

    pub fn accesses_unknown(&mut self, proc: &ast::ProcedureDecl) -> bool {
        let id: *const ast::ProcedureDecl = proc;
        if let Some((_, unknown)) = self.cache.iter().find(|(cached, _)| *cached == id) {
            return *unknown;
        }
        if self.active.contains(&id) {
            return true;
        }
        self.active.push(id);
        let unknown = match proc.body.as_deref() {
            None => true,
            Some(body) => {
                let shared_params =
                    proc.params.iter().filter(|param| type_is_shared_param_surface(&param.r#type)).map(|param| id_key_of(&param.name)).collect();
                let mut walk = Walk { shared_params: &shared_params, unknown: false };
                self.visit_block(body, &mut Aliases::new(), &mut walk);
                walk.unknown
            }
        };
        self.active.retain(|active| *active != id);
        self.cache.push((id, unknown));
        unknown
    }

    fn resolves_to_formal(expr: &ExprPtr, aliases: &Aliases, walk: &Walk<'_>) -> bool {
        let built = build_key_path(expr);
        built.success && (walk.shared_params.contains(&built.path.root) || aliases.contains(&built.path.root))
    }

    fn handle_call(&mut self, call: &ast::CallExpr, aliases: &Aliases, walk: &mut Walk<'_>) {
        if call.callee.is_none() {
            return;
        }
        let Some(callee) = lookup_procedure_for_callee(self.ctx, &call.callee).and_then(|lookup| lookup.proc.proc) else {
            walk.unknown = true;
            return;
        };
        if self.accesses_unknown(callee)
            && call
                .args
                .iter()
                .zip(&callee.params)
                .any(|(arg, param)| type_is_shared_param_surface(&param.r#type) && Self::resolves_to_formal(&arg.value, aliases, walk))
        {
            walk.unknown = true;
        }
    }

    fn visit_scoped_block(&mut self, block: &ast::BlockPtr, aliases: &Aliases, walk: &mut Walk<'_>) {
        if let Some(block) = block.as_deref() {
            self.visit_block(block, &mut aliases.clone(), walk);
        }
    }

    fn visit_block(&mut self, block: &ast::Block, aliases: &mut Aliases, walk: &mut Walk<'_>) {
        for stmt in &block.stmts {
            self.visit_stmt(stmt, aliases, walk);
        }
        self.visit_expr(&block.tail_opt, aliases, walk);
    }

    fn visit_binding(&mut self, binding: &ast::Binding, aliases: &mut Aliases, walk: &mut Walk<'_>) {
        self.visit_expr(&binding.init, aliases, walk);
        let name = match binding.pat.as_deref().map(|pat| &pat.node) {
            Some(ast::PatternNode::IdentifierPattern(ident)) => Some(id_key_of(&ident.name)),
            Some(ast::PatternNode::TypedPattern(typed)) if typed.name != "_" => Some(id_key_of(&typed.name)),
            _ => None,
        };
        if let Some(name) = name {
            if Self::resolves_to_formal(&binding.init, aliases, walk) {
                aliases.insert(name);
            } else {
                aliases.remove(&name);
            }
        }
    }

    fn visit_assign(&mut self, place: &ExprPtr, value: &ExprPtr, aliases: &mut Aliases, walk: &mut Walk<'_>) {
        self.visit_expr(place, aliases, walk);
        self.visit_expr(value, aliases, walk);
        if let Some(ExprNode::IdentifierExpr(ident)) = place.as_deref().map(|place| &place.node) {
            aliases.remove(&id_key_of(&ident.name));
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt, aliases: &mut Aliases, walk: &mut Walk<'_>) {
        match stmt {
            Stmt::LetStmt(node) => self.visit_binding(&node.binding, aliases, walk),
            Stmt::VarStmt(node) => self.visit_binding(&node.binding, aliases, walk),
            Stmt::UsingLocalStmt(node) => {
                let alias = id_key_of(&node.alias);
                if aliases.contains(&id_key_of(&node.source)) {
                    aliases.insert(alias);
                } else {
                    aliases.remove(&alias);
                }
            }
            Stmt::ExprStmt(node) => self.visit_expr(&node.value, aliases, walk),
            Stmt::AssignStmt(node) => self.visit_assign(&node.place, &node.value, aliases, walk),
            Stmt::CompoundAssignStmt(node) => self.visit_assign(&node.place, &node.value, aliases, walk),
            Stmt::ReturnStmt(node) => self.visit_expr(&node.value_opt, aliases, walk),
            Stmt::BreakStmt(node) => self.visit_expr(&node.value_opt, aliases, walk),
            Stmt::DeferStmt(node) => self.visit_scoped_block(&node.body, aliases, walk),
            Stmt::UnsafeBlockStmt(node) => self.visit_scoped_block(&node.body, aliases, walk),
            Stmt::RegionStmt(node) => {
                self.visit_expr(&node.opts_opt, aliases, walk);
                self.visit_scoped_block(&node.body, aliases, walk);
            }
            Stmt::FrameStmt(node) => self.visit_scoped_block(&node.body, aliases, walk),
            Stmt::CtStmt(node) => self.visit_scoped_block(&node.body, aliases, walk),
            Stmt::KeyBlockStmt(node) => self.visit_scoped_block(&node.body, aliases, walk),
            _ => {}
        }
    }

    fn visit_all<'e>(&mut self, exprs: impl IntoIterator<Item = &'e ExprPtr>, aliases: &mut Aliases, walk: &mut Walk<'_>) {
        for expr in exprs {
            self.visit_expr(expr, aliases, walk);
        }
    }

    fn visit_expr(&mut self, expr: &ExprPtr, aliases: &mut Aliases, walk: &mut Walk<'_>) {
        let Some(e) = expr.as_deref() else {
            return;
        };
        match &e.node {
            ExprNode::CallExpr(node) => {
                self.handle_call(node, aliases, walk);
                self.visit_expr(&node.callee, aliases, walk);
                self.visit_all(node.args.iter().map(|arg| &arg.value), aliases, walk);
            }
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::ParenArgs(args) => self.visit_all(args.args.iter().map(|arg| &arg.value), aliases, walk),
                ast::ApplyArgs::BraceArgs(args) => self.visit_all(args.fields.iter().map(|field| &field.value), aliases, walk),
            },
            ExprNode::MethodCallExpr(node) => {
                if Self::resolves_to_formal(&node.receiver, aliases, walk) {
                    walk.unknown = true;
                }
                self.visit_expr(&node.receiver, aliases, walk);
                self.visit_all(node.args.iter().map(|arg| &arg.value), aliases, walk);
            }
            ExprNode::BlockExpr(node) => self.visit_scoped_block(&node.block, aliases, walk),
            ExprNode::UnsafeBlockExpr(node) => self.visit_scoped_block(&node.block, aliases, walk),
            ExprNode::IfExpr(node) => self.visit_all([&node.cond, &node.then_expr, &node.else_expr], aliases, walk),
            ExprNode::IfIsExpr(node) => self.visit_all([&node.scrutinee, &node.then_expr, &node.else_expr], aliases, walk),
            ExprNode::IfCaseExpr(node) => {
                self.visit_expr(&node.scrutinee, aliases, walk);
                self.visit_all(node.cases.iter().map(|clause| &clause.body), aliases, walk);
                self.visit_expr(&node.else_expr, aliases, walk);
            }
            ExprNode::LoopInfiniteExpr(node) => {
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate, aliases, walk);
                }
                self.visit_scoped_block(&node.body, aliases, walk);
            }
            ExprNode::LoopConditionalExpr(node) => {
                self.visit_expr(&node.cond, aliases, walk);
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate, aliases, walk);
                }
                self.visit_scoped_block(&node.body, aliases, walk);
            }
            ExprNode::LoopIterExpr(node) => {
                self.visit_expr(&node.iter, aliases, walk);
                if let Some(invariant) = &node.invariant_opt {
                    self.visit_expr(&invariant.predicate, aliases, walk);
                }
                self.visit_scoped_block(&node.body, aliases, walk);
            }
            ExprNode::BinaryExpr(node) => self.visit_all([&node.lhs, &node.rhs], aliases, walk),
            ExprNode::RangeExpr(node) => self.visit_all([&node.lhs, &node.rhs], aliases, walk),
            ExprNode::IndexAccessExpr(node) => self.visit_all([&node.base, &node.index], aliases, walk),
            ExprNode::ArrayRepeatExpr(node) => self.visit_all([&node.value, &node.count], aliases, walk),
            ExprNode::UnaryExpr(node) => self.visit_expr(&node.value, aliases, walk),
            ExprNode::CastExpr(node) => self.visit_expr(&node.value, aliases, walk),
            ExprNode::DerefExpr(node) => self.visit_expr(&node.value, aliases, walk),
            ExprNode::PropagateExpr(node) => self.visit_expr(&node.value, aliases, walk),
            ExprNode::MoveExpr(node) => self.visit_expr(&node.place, aliases, walk),
            ExprNode::AddressOfExpr(node) => self.visit_expr(&node.place, aliases, walk),
            ExprNode::FieldAccessExpr(node) => self.visit_expr(&node.base, aliases, walk),
            ExprNode::TupleAccessExpr(node) => self.visit_expr(&node.base, aliases, walk),
            ExprNode::AttributedExpr(node) => self.visit_expr(&node.expr, aliases, walk),
            ExprNode::ComptimeExpr(node) => self.visit_expr(&node.body, aliases, walk),
            ExprNode::TupleExpr(node) => self.visit_all(&node.elements, aliases, walk),
            ExprNode::ArrayExpr(node) => self.visit_all(ast::array_expr_subexprs(node), aliases, walk),
            ExprNode::RecordExpr(node) => self.visit_all(node.fields.iter().map(|field| &field.value), aliases, walk),
            ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
                Some(ast::EnumPayload::EnumPayloadParen(payload)) => self.visit_all(&payload.elements, aliases, walk),
                Some(ast::EnumPayload::EnumPayloadBrace(payload)) => self.visit_all(payload.fields.iter().map(|field| &field.value), aliases, walk),
                None => {}
            },
            ExprNode::ParallelExpr(node) => self.visit_scoped_block(&node.body, aliases, walk),
            ExprNode::SpawnExpr(node) => self.visit_scoped_block(&node.body, aliases, walk),
            ExprNode::DispatchExpr(node) => {
                self.visit_expr(&node.range, aliases, walk);
                self.visit_scoped_block(&node.body, aliases, walk);
            }
            ExprNode::CtIfExpr(node) => {
                self.visit_expr(&node.cond, aliases, walk);
                self.visit_scoped_block(&node.then_block, aliases, walk);
                self.visit_scoped_block(&node.else_block_opt, aliases, walk);
            }
            ExprNode::CtLoopIterExpr(node) => {
                self.visit_expr(&node.iter, aliases, walk);
                self.visit_scoped_block(&node.body, aliases, walk);
            }
            _ => {}
        }
    }
}
