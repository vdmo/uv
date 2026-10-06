//! Attributed expressions: the attributes are validated, a memory ordering must sit on
//! an access to shared data, and `[[dynamic]]` makes the expression a dynamic context.

use uv_source::ast::{self, AttributeItem, ExprNode, ExprPtr, Stmt};
use uv_source::attributes::{attrs, validate_attributes, AttributeTarget};

use super::dynamic_context::{compute_dynamic_context, DynamicScopeAncestor};
use super::stmt_context::StmtTypeContext;
use super::type_env::TypeEnv;
use super::type_expr::{type_expr, type_place};
use super::type_predicates::perm_of_type;
use super::types::Permission;
use crate::context::ScopeContext;

fn is_memory_order_attribute(attr: &AttributeItem) -> bool {
    [attrs::RELAXED, attrs::ACQUIRE, attrs::RELEASE, attrs::ACQ_REL, attrs::SEQ_CST].contains(&attr.name.full_name.as_str())
}

pub fn has_memory_order_attribute(attr_list: &[AttributeItem]) -> bool {
    attr_list.iter().any(is_memory_order_attribute)
}

/// Whether the expression is in a dynamic context: inherited, or marked on itself.
pub fn compute_expr_dynamic_context(expr: &ast::Expr, inherited: bool) -> bool {
    let ancestors = [DynamicScopeAncestor { attrs: ast::expr_attr_list(expr), span: &expr.span }];
    inherited || compute_dynamic_context(&expr.span, &ancestors)
}

/// The checks on the attributes alone, before the inner expression is typed.
pub fn attribute_list_diag(type_ctx: &StmtTypeContext<'_>, attr_list: &[AttributeItem]) -> Option<&'static str> {
    if type_ctx.in_speculative && has_memory_order_attribute(attr_list) {
        return Some("E-CON-0096");
    }
    let validation = validate_attributes(attr_list, AttributeTarget::Expression);
    if !validation.ok {
        return validation.diag_id;
    }
    (attr_list.iter().filter(|attr| is_memory_order_attribute(attr)).count() > 1).then_some("E-MOD-2450")
}

/// A memory ordering means something only where shared data is accessed.
pub fn memory_order_placement_diag(
    ctx: &ScopeContext<'_>,
    type_ctx: &StmtTypeContext<'_>,
    attr_list: &[AttributeItem],
    expr: &ExprPtr,
    env: &TypeEnv,
) -> Option<&'static str> {
    let scan = SharedAccessScan { ctx, type_ctx, env };
    (has_memory_order_attribute(attr_list) && !scan.expr(expr)).then_some("E-MOD-2450")
}

struct SharedAccessScan<'a, 't> {
    ctx: &'a ScopeContext<'a>,
    type_ctx: &'a StmtTypeContext<'t>,
    env: &'a TypeEnv,
}

impl SharedAccessScan<'_, '_> {
    fn shared_place(&self, expr: &ExprPtr) -> bool {
        if expr.is_none() {
            return false;
        }
        let place = type_place(self.ctx, self.type_ctx, expr, self.env);
        place.ok && place.r#type.is_some() && perm_of_type(&place.r#type) == Permission::Shared
    }

    fn shared_receiver(&self, receiver: &ExprPtr) -> bool {
        if receiver.is_none() {
            return false;
        }
        if self.shared_place(receiver) {
            return true;
        }
        let value = type_expr(self.ctx, self.type_ctx, receiver, self.env);
        value.ok && value.r#type.is_some() && perm_of_type(&value.r#type) == Permission::Shared
    }

    fn any<'x>(&self, exprs: impl IntoIterator<Item = &'x ExprPtr>) -> bool {
        exprs.into_iter().any(|expr| self.expr(expr))
    }

    fn block(&self, block: &ast::BlockPtr) -> bool {
        block.as_deref().is_some_and(|block| block.stmts.iter().any(|stmt| self.stmt(stmt)) || self.expr(&block.tail_opt))
    }

    fn stmt(&self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::LetStmt(node) => self.expr(&node.binding.init),
            Stmt::VarStmt(node) => self.expr(&node.binding.init),
            Stmt::AssignStmt(node) => self.shared_place(&node.place) || self.expr(&node.place) || self.expr(&node.value),
            Stmt::CompoundAssignStmt(node) => self.shared_place(&node.place) || self.expr(&node.place) || self.expr(&node.value),
            Stmt::ExprStmt(node) => self.expr(&node.value),
            Stmt::DeferStmt(node) => self.block(&node.body),
            Stmt::UnsafeBlockStmt(node) => self.block(&node.body),
            Stmt::CtStmt(node) => self.block(&node.body),
            Stmt::RegionStmt(node) => self.expr(&node.opts_opt) || self.block(&node.body),
            Stmt::FrameStmt(node) => self.block(&node.body),
            Stmt::ReturnStmt(node) => self.expr(&node.value_opt),
            Stmt::BreakStmt(node) => self.expr(&node.value_opt),
            Stmt::KeyBlockStmt(_) => true,
            _ => false,
        }
    }

    fn expr(&self, expr: &ExprPtr) -> bool {
        let Some(e) = expr.as_deref() else {
            return false;
        };
        match &e.node {
            ExprNode::IdentifierExpr(_)
            | ExprNode::FieldAccessExpr(_)
            | ExprNode::TupleAccessExpr(_)
            | ExprNode::IndexAccessExpr(_)
            | ExprNode::DerefExpr(_) => self.shared_place(expr),
            ExprNode::MethodCallExpr(node) => {
                self.shared_receiver(&node.receiver) || self.expr(&node.receiver) || self.any(node.args.iter().map(|arg| &arg.value))
            }
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::ParenArgs(args) => self.any(args.args.iter().map(|arg| &arg.value)),
                ast::ApplyArgs::BraceArgs(args) => self.any(args.fields.iter().map(|field| &field.value)),
            },
            ExprNode::RangeExpr(node) => self.any([&node.lhs, &node.rhs]),
            ExprNode::BinaryExpr(node) => self.any([&node.lhs, &node.rhs]),
            ExprNode::PipelineExpr(node) => self.any([&node.lhs, &node.rhs]),
            ExprNode::CastExpr(node) => self.expr(&node.value),
            ExprNode::UnaryExpr(node) => self.expr(&node.value),
            ExprNode::AllocExpr(node) => self.expr(&node.value),
            ExprNode::TransmuteExpr(node) => self.expr(&node.value),
            ExprNode::PropagateExpr(node) => self.expr(&node.value),
            ExprNode::YieldExpr(node) => self.expr(&node.value),
            ExprNode::YieldFromExpr(node) => self.expr(&node.value),
            ExprNode::SyncExpr(node) => self.expr(&node.value),
            ExprNode::EntryExpr(node) => self.expr(&node.expr),
            ExprNode::AddressOfExpr(node) => self.expr(&node.place),
            ExprNode::MoveExpr(node) => self.expr(&node.place),
            ExprNode::TupleExpr(node) => self.any(&node.elements),
            ExprNode::ArrayExpr(node) => self.any(ast::array_expr_subexprs(node)),
            ExprNode::ArrayRepeatExpr(node) => self.any([&node.value, &node.count]),
            ExprNode::RecordExpr(node) => self.any(node.fields.iter().map(|field| &field.value)),
            ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
                Some(ast::EnumPayload::EnumPayloadParen(payload)) => self.any(&payload.elements),
                Some(ast::EnumPayload::EnumPayloadBrace(payload)) => self.any(payload.fields.iter().map(|field| &field.value)),
                None => false,
            },
            ExprNode::IfExpr(node) => self.any([&node.cond, &node.then_expr, &node.else_expr]),
            ExprNode::IfCaseExpr(node) => {
                self.expr(&node.scrutinee) || self.any(node.cases.iter().map(|clause| &clause.body)) || self.expr(&node.else_expr)
            }
            ExprNode::IfIsExpr(node) => self.any([&node.scrutinee, &node.then_expr, &node.else_expr]),
            ExprNode::LoopInfiniteExpr(node) => {
                node.invariant_opt.as_ref().is_some_and(|invariant| self.expr(&invariant.predicate)) || self.block(&node.body)
            }
            ExprNode::LoopConditionalExpr(node) => {
                self.expr(&node.cond)
                    || node.invariant_opt.as_ref().is_some_and(|invariant| self.expr(&invariant.predicate))
                    || self.block(&node.body)
            }
            ExprNode::LoopIterExpr(node) => {
                self.expr(&node.iter)
                    || node.invariant_opt.as_ref().is_some_and(|invariant| self.expr(&invariant.predicate))
                    || self.block(&node.body)
            }
            ExprNode::BlockExpr(node) => self.block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.block(&node.block),
            ExprNode::ComptimeExpr(node) => self.expr(&node.body),
            ExprNode::CtIfExpr(node) => self.expr(&node.cond) || self.block(&node.then_block) || self.block(&node.else_block_opt),
            ExprNode::CtLoopIterExpr(node) => self.expr(&node.iter) || self.block(&node.body),
            ExprNode::AttributedExpr(node) => self.expr(&node.expr),
            ExprNode::ClosureExpr(node) => self.expr(&node.body),
            ExprNode::CallExpr(node) => self.expr(&node.callee) || self.any(node.args.iter().map(|arg| &arg.value)),
            ExprNode::CallTypeArgsExpr(node) => self.expr(&node.callee) || self.any(node.args.iter().map(|arg| &arg.value)),
            ExprNode::RaceExpr(node) => node.arms.iter().any(|arm| self.expr(&arm.expr) || self.expr(&arm.handler.value)),
            ExprNode::AllExpr(node) => self.any(&node.exprs),
            ExprNode::ParallelExpr(node) => {
                self.expr(&node.domain) || self.any(node.opts.iter().map(|opt| &opt.value)) || self.block(&node.body)
            }
            ExprNode::SpawnExpr(node) => self.any(node.opts.iter().map(|opt| &opt.value)) || self.block(&node.body),
            ExprNode::WaitExpr(node) => self.expr(&node.handle),
            ExprNode::DispatchExpr(node) => {
                self.expr(&node.range)
                    || node.opts.iter().any(|opt| self.expr(&opt.chunk_expr) || self.expr(&opt.workgroup_expr))
                    || self.block(&node.body)
            }
            _ => false,
        }
    }
}
