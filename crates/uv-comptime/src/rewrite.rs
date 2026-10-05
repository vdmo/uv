//! Phase-2 expansion of a module: compile-time statements and expressions are evaluated
//! and replaced, derive attributes are expanded, and emitted items are queued for
//! expansion right after the item that emitted them.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::span::Span;
use uv_core::spec_trace::Conformance;
use uv_core::{spec_rule, spec_rule_at};
use uv_source::ast::*;
use uv_source::attributes::{attrs, validate_attributes, AttributeTarget};
use uv_source::lexer::{Token, TokenKind};

use crate::derive::{expand_derives, is_derive_annotated_item};
use crate::eval::{eval_block, eval_expr};
use crate::hygiene::prepare_ast_for_insertion;
use crate::util::stoull;
use crate::value::*;

fn emit_runtime_ct_procedure_reference_if_needed(env: &CtEnv, expr: &ExprPtr) {
    let Some(expr) = expr else {
        return;
    };
    if let ExprNode::IdentifierExpr(ident) = &expr.node {
        if env.procs.contains_key(&ident.name) {
            spec_rule_at!("requirement.22.CompileTimeProcedureContextRestriction", &expr.span);
            emit_comptime_diag(env, "E-CTE-0034", &expr.span);
        }
    }
}

fn emitted_item_name(item: &ASTItem) -> &str {
    match item {
        ASTItem::ProcedureDecl(d) => &d.name,
        ASTItem::ComptimeProcedureDecl(d) => &d.name,
        ASTItem::RecordDecl(d) => &d.name,
        ASTItem::EnumDecl(d) => &d.name,
        ASTItem::ModalDecl(d) => &d.name,
        ASTItem::ClassDecl(d) => &d.name,
        ASTItem::TypeAliasDecl(d) => &d.name,
        ASTItem::DeriveTargetDecl(d) => &d.name,
        _ => "item",
    }
}

fn record_emission_order(span: &Span, emitted: &[ASTItem]) {
    if emitted.is_empty() || !Conformance::enabled() {
        return;
    }
    let names: Vec<&str> = emitted.iter().map(emitted_item_name).collect();
    Conformance::record_at(
        "requirement.22.EmissionOrder",
        Some(span),
        &format!("items:{}", names.join(",")),
    );
}

/// Finds the first construct a compile-time block may not contain (regions, frames,
/// `unsafe`, key blocks, dereferences, transmutes and the concurrency forms).
#[derive(Default)]
pub(crate) struct CtProhibitedConstructFinder {
    found: Option<Span>,
}

impl CtProhibitedConstructFinder {
    pub(crate) fn find(block: &Block) -> Option<Span> {
        let mut finder = CtProhibitedConstructFinder::default();
        finder.block(block);
        finder.found
    }

    fn mark(&mut self, span: &Span) {
        if self.found.is_none() {
            self.found = Some(span.clone());
        }
    }

    fn block_ptr(&mut self, block: &BlockPtr) {
        if let Some(block) = block {
            self.block(block);
        }
    }

    fn block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.stmt(stmt);
            if self.found.is_some() {
                return;
            }
        }
        self.expr(&block.tail_opt);
    }

    fn stmt(&mut self, stmt: &Stmt) {
        if self.found.is_some() {
            return;
        }
        match stmt {
            Stmt::RegionStmt(s) => self.mark(&s.span),
            Stmt::FrameStmt(s) => self.mark(&s.span),
            Stmt::UnsafeBlockStmt(s) => self.mark(&s.span),
            Stmt::KeyBlockStmt(s) => self.mark(&s.span),
            Stmt::LetStmt(s) => self.expr(&s.binding.init),
            Stmt::VarStmt(s) => self.expr(&s.binding.init),
            Stmt::AssignStmt(s) => {
                self.expr(&s.place);
                self.expr(&s.value);
            }
            Stmt::CompoundAssignStmt(s) => {
                self.expr(&s.place);
                self.expr(&s.value);
            }
            Stmt::ExprStmt(s) => self.expr(&s.value),
            Stmt::DeferStmt(s) => self.block_ptr(&s.body),
            Stmt::CtStmt(s) => self.block_ptr(&s.body),
            Stmt::ReturnStmt(s) => self.expr(&s.value_opt),
            Stmt::BreakStmt(s) => self.expr(&s.value_opt),
            _ => {}
        }
    }

    fn args(&mut self, args: &[Arg]) {
        for arg in args {
            self.expr(&arg.value);
        }
    }

    fn expr(&mut self, expr: &ExprPtr) {
        let Some(expr) = expr else {
            return;
        };
        if self.found.is_some() {
            return;
        }
        match &expr.node {
            ExprNode::UnsafeBlockExpr(_)
            | ExprNode::DerefExpr(_)
            | ExprNode::TransmuteExpr(_)
            | ExprNode::YieldExpr(_)
            | ExprNode::YieldFromExpr(_)
            | ExprNode::SyncExpr(_)
            | ExprNode::RaceExpr(_)
            | ExprNode::AllExpr(_)
            | ExprNode::ParallelExpr(_)
            | ExprNode::SpawnExpr(_)
            | ExprNode::WaitExpr(_)
            | ExprNode::DispatchExpr(_) => self.mark(&expr.span),
            ExprNode::BlockExpr(n) => self.block_ptr(&n.block),
            ExprNode::ComptimeExpr(n) => self.expr(&n.body),
            ExprNode::CtIfExpr(n) => {
                self.expr(&n.cond);
                self.block_ptr(&n.then_block);
                self.block_ptr(&n.else_block_opt);
            }
            ExprNode::CtLoopIterExpr(n) => {
                self.expr(&n.iter);
                self.block_ptr(&n.body);
            }
            ExprNode::AttributedExpr(n) => self.expr(&n.expr),
            ExprNode::RangeExpr(n) => {
                self.expr(&n.lhs);
                self.expr(&n.rhs);
            }
            ExprNode::BinaryExpr(n) => {
                self.expr(&n.lhs);
                self.expr(&n.rhs);
            }
            ExprNode::UnaryExpr(n) => self.expr(&n.value),
            ExprNode::CastExpr(n) => self.expr(&n.value),
            ExprNode::AllocExpr(n) => self.expr(&n.value),
            ExprNode::PropagateExpr(n) => self.expr(&n.value),
            ExprNode::EntryExpr(n) => self.expr(&n.expr),
            ExprNode::AddressOfExpr(n) => self.expr(&n.place),
            ExprNode::MoveExpr(n) => self.expr(&n.place),
            ExprNode::FieldAccessExpr(n) => self.expr(&n.base),
            ExprNode::TupleAccessExpr(n) => self.expr(&n.base),
            ExprNode::IndexAccessExpr(n) => {
                self.expr(&n.base);
                self.expr(&n.index);
            }
            ExprNode::CallExpr(n) => {
                self.expr(&n.callee);
                self.args(&n.args);
            }
            ExprNode::CallTypeArgsExpr(n) => {
                self.expr(&n.callee);
                self.args(&n.args);
            }
            ExprNode::MethodCallExpr(n) => {
                self.expr(&n.receiver);
                self.args(&n.args);
            }
            ExprNode::TupleExpr(n) => {
                for elem in &n.elements {
                    self.expr(elem);
                }
            }
            ExprNode::ArrayExpr(n) => {
                for segment in &n.elements {
                    match segment {
                        ArraySegment::ArrayElemSegment(elem) => self.expr(&elem.value),
                        ArraySegment::ArrayRepeatSegment(repeat) => {
                            self.expr(&repeat.value);
                            self.expr(&repeat.count);
                        }
                    }
                }
            }
            ExprNode::ArrayRepeatExpr(n) => {
                self.expr(&n.value);
                self.expr(&n.count);
            }
            ExprNode::RecordExpr(n) => {
                for field in &n.fields {
                    self.expr(&field.value);
                }
            }
            ExprNode::IfExpr(n) => {
                self.expr(&n.cond);
                self.expr(&n.then_expr);
                self.expr(&n.else_expr);
            }
            ExprNode::IfCaseExpr(n) => {
                self.expr(&n.scrutinee);
                for arm in &n.cases {
                    self.expr(&arm.body);
                }
                self.expr(&n.else_expr);
            }
            ExprNode::IfIsExpr(n) => {
                self.expr(&n.scrutinee);
                self.expr(&n.then_expr);
                self.expr(&n.else_expr);
            }
            ExprNode::LoopInfiniteExpr(n) => self.block_ptr(&n.body),
            ExprNode::LoopConditionalExpr(n) => {
                self.expr(&n.cond);
                self.block_ptr(&n.body);
            }
            ExprNode::LoopIterExpr(n) => {
                self.expr(&n.iter);
                self.block_ptr(&n.body);
            }
            ExprNode::ClosureExpr(n) => self.expr(&n.body),
            ExprNode::PipelineExpr(n) => {
                self.expr(&n.lhs);
                self.expr(&n.rhs);
            }
            _ => {}
        }
    }
}

fn validate_phase2_attribute_list(env: &CtEnv, attr_list: &[AttributeItem], target: AttributeTarget) -> bool {
    let Some(first) = attr_list.first() else {
        return true;
    };
    let validation = validate_attributes(attr_list, target);
    if validation.ok {
        if has_attribute(attr_list, "emit") || has_attribute(attr_list, "files") {
            spec_rule_at!("requirement.22.CompileTimeCapabilitiesSyntaxSurface", &first.span);
        }
        return true;
    }
    if let Some(diags) = &env.diags {
        let code = validation.diag_id.unwrap_or("E-MOD-2450");
        if let Some(mut diag) = make_diagnostic_by_id(code, validation.span) {
            if !validation.message.is_empty() {
                diag.message = validation.message;
            }
            emit(&mut diags.borrow_mut(), diag);
        }
    }
    false
}

fn reject_invalid_derive_attribute_target_kind(env: &CtEnv, item: &ASTItem) -> bool {
    let is_derive_target_kind =
        matches!(item, ASTItem::RecordDecl(_) | ASTItem::EnumDecl(_) | ASTItem::ModalDecl(_));
    if !has_attribute(attr_list_of(item), attrs::DERIVE) || is_derive_target_kind {
        return false;
    }
    if env.diags.is_some() {
        let span = span_of_item(item);
        spec_rule_at!("requirement.22.DeriveAttributeTargetKinds", &span);
        emit_comptime_diag(env, "E-CTE-0311", &span);
    }
    true
}

fn make_unit_expr(span: &Span) -> ExprPtr {
    Some(Arc::new(Expr { span: span.clone(), node: TupleExpr::default().into() }))
}

fn make_block_expr(block: Block, span: &Span) -> ExprPtr {
    let node = BlockExpr { block: Some(Arc::new(block)) }.into();
    Some(Arc::new(Expr { span: span.clone(), node }))
}

fn make_fallback_empty_block(span: &Span) -> Block {
    Block { stmts: Vec::new(), tail_opt: make_unit_expr(span), span: span.clone() }
}

/// Appends a block's statements, turning its tail expression into a statement.
fn append_unit_block_stmts(block: Block, out: &mut Vec<Stmt>) {
    out.extend(block.stmts);
    if let Some(tail) = block.tail_opt {
        let span = tail.span.clone();
        out.push(ExprStmt { value: Some(tail), span }.into());
    }
}

fn ct_value_equals_literal(value: &CtValue, literal: &Token) -> bool {
    match literal.kind {
        TokenKind::BoolLiteral => {
            try_get_ct_bool(value).is_some_and(|b| (literal.lexeme == "true") == b)
        }
        TokenKind::IntLiteral => match (try_get_ct_int(value), stoull(&literal.lexeme)) {
            (Some(int), Some(literal_value)) => int.value == literal_value,
            _ => false,
        },
        TokenKind::StringLiteral => {
            let CtValue::String(text) = value else {
                return false;
            };
            let lexeme = literal.lexeme.as_str();
            let unquoted = lexeme
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))
                .filter(|_| lexeme.len() >= 2)
                .unwrap_or(lexeme);
            text == unquoted
        }
        _ => false,
    }
}

fn bind_ct_range_pattern(range: &RangePattern, value: &CtValue) -> bool {
    let (Some(int_value), Some(lo), Some(hi)) = (try_get_ct_int(value), &range.lo, &range.hi) else {
        return false;
    };
    let (PatternNode::LiteralPattern(lo), PatternNode::LiteralPattern(hi)) = (&lo.node, &hi.node) else {
        return false;
    };
    let (Some(lo), Some(hi)) = (stoull(&lo.literal.lexeme), stoull(&hi.literal.lexeme)) else {
        return false;
    };
    let upper_ok =
        if range.kind == RangeKind::Inclusive { int_value.value <= hi } else { int_value.value < hi };
    int_value.value >= lo && upper_ok
}

pub(crate) fn bind_ct_pattern_value(env: &mut CtEnv, pattern: &PatternPtr, value: &CtValue) -> bool {
    let Some(pattern) = pattern else {
        return false;
    };
    match &pattern.node {
        PatternNode::WildcardPattern(_) => true,
        PatternNode::IdentifierPattern(node) => {
            env.values.insert(node.name.clone(), value.clone());
            true
        }
        PatternNode::TypedPattern(node) => {
            env.values.insert(node.name.clone(), value.clone());
            true
        }
        PatternNode::LiteralPattern(node) => ct_value_equals_literal(value, &node.literal),
        PatternNode::TuplePattern(tuple) => {
            let CtValue::Tuple(elements) = value else {
                return false;
            };
            if elements.len() != tuple.elements.len() {
                return false;
            }
            // Bindings take effect only when every element matches.
            let mut next = env.clone();
            for (sub_pattern, element) in tuple.elements.iter().zip(elements.iter()) {
                if !bind_ct_pattern_value(&mut next, sub_pattern, element) {
                    return false;
                }
            }
            *env = next;
            true
        }
        PatternNode::RangePattern(range) => bind_ct_range_pattern(range, value),
        _ => false,
    }
}

fn rewrite_stmt(stmt: &Stmt, env: &mut CtEnv) -> Stmt {
    spec_rule!("requirement.22.CtExpandOrdinaryTraversal");
    match stmt {
        Stmt::LetStmt(node) => {
            let mut out = node.clone();
            out.binding.init = rewrite_expr(&node.binding.init, env);
            out.into()
        }
        Stmt::VarStmt(node) => {
            let mut out = node.clone();
            out.binding.init = rewrite_expr(&node.binding.init, env);
            out.into()
        }
        Stmt::ExprStmt(node) => {
            let mut out = node.clone();
            out.value = rewrite_expr(&node.value, env);
            out.into()
        }
        Stmt::ReturnStmt(node) => {
            let mut out = node.clone();
            out.value_opt = rewrite_expr(&node.value_opt, env);
            out.into()
        }
        Stmt::AssignStmt(node) => {
            let mut out = node.clone();
            out.place = rewrite_expr(&node.place, env);
            out.value = rewrite_expr(&node.value, env);
            out.into()
        }
        Stmt::CompoundAssignStmt(node) => {
            let mut out = node.clone();
            out.place = rewrite_expr(&node.place, env);
            out.value = rewrite_expr(&node.value, env);
            out.into()
        }
        _ => stmt.clone(),
    }
}

/// Moves what a nested evaluation emitted into the enclosing pending list.
fn transfer_pending_emits(env: &CtEnv, span: &Span, emitted: &Rc<RefCell<Vec<ASTItem>>>) {
    let Some(pending) = &env.pending_emits else {
        return;
    };
    let emitted = emitted.borrow();
    record_emission_order(span, &emitted);
    if !emitted.is_empty() {
        spec_rule_at!("requirement.22.CtPendingEmitsTransfer", span);
    }
    pending.borrow_mut().extend(emitted.iter().cloned());
}

fn rewrite_block(block: &Block, env: &mut CtEnv) -> Block {
    spec_rule_at!("CtExpandBlock", &block.span);
    let mut stmts = Vec::with_capacity(block.stmts.len());
    if block.stmts.is_empty() {
        spec_rule!("CtExpandStmtSeq-Empty");
    }
    for stmt in &block.stmts {
        spec_rule!("CtExpandStmtSeq-Cons");
        let Stmt::CtStmt(comptime) = stmt else {
            stmts.push(rewrite_stmt(stmt, env));
            continue;
        };
        if !validate_phase2_attribute_list(env, &comptime.attrs, AttributeTarget::Statement) {
            continue;
        }
        spec_rule_at!("CtExpandStmt-CtStmt", &comptime.span);
        let body = comptime.body.as_deref().expect("comptime statement without a body");
        if let Some(span) = CtProhibitedConstructFinder::find(body) {
            spec_rule_at!("requirement.22.CompileTimeProhibitedConstructs", &span);
            emit_comptime_diag(env, "E-CTE-0020", &span);
            continue;
        }
        let emitted = Rc::new(RefCell::new(Vec::new()));
        let mut stmt_env = with_ct_caps(env.clone(), &comptime.attrs, false);
        stmt_env.pending_emits = Some(emitted.clone());
        if eval_block(body, &mut stmt_env).ok {
            transfer_pending_emits(env, &comptime.span, &emitted);
        }
    }
    let tail_opt = rewrite_expr(&block.tail_opt, env);
    Block { stmts, tail_opt, span: block.span.clone() }
}

fn rewrite_block_ptr(block: &BlockPtr, env: &mut CtEnv) -> BlockPtr {
    let block = block.as_deref().expect("block expected during compile-time expansion");
    Some(Arc::new(rewrite_block(block, env)))
}

/// Evaluates a `comptime` expression and returns what replaces it: the quoted expression
/// it produced, or the literal form of its value. `None` leaves the expression in place.
fn expand_comptime_expr(expr: &Arc<Expr>, attr_list: &[AttributeItem], env: &mut CtEnv) -> ExprPtr {
    let mut ct_env = with_ct_caps(env.clone(), attr_list, false);
    let emitted = Rc::new(RefCell::new(Vec::new()));
    ct_env.pending_emits = Some(emitted.clone());
    let value = eval_expr(&Some(expr.clone()), &mut ct_env);
    if !value.ok {
        return None;
    }
    transfer_pending_emits(env, &expr.span, &emitted);
    if let CtValue::Ast(ast) = &value.value {
        let site = env.site.clone();
        if let Some(CtAst { kind: CtAstKind::Expr, payload: CtAstPayload::Expr(quoted), .. }) =
            prepare_ast_for_insertion(ast, &site, env)
        {
            return quoted;
        }
        emit_comptime_diag(env, "E-CTE-0210", &expr.span);
        return None;
    }
    literalize_value(&value.value, &expr.span)
}

fn rewrite_args(args: &mut [Arg], env: &mut CtEnv) {
    for arg in args {
        arg.value = rewrite_expr(&arg.value, env);
    }
}

fn rewrite_expr(expr_ptr: &ExprPtr, env: &mut CtEnv) -> ExprPtr {
    let expr = expr_ptr.as_ref()?;
    let with_node = |node: ExprNode| Some(Arc::new(Expr { span: expr.span.clone(), node }));
    match &expr.node {
        ExprNode::AttributedExpr(node) => {
            let wraps_comptime =
                matches!(node.expr.as_deref(), Some(Expr { node: ExprNode::ComptimeExpr(_), .. }));
            if wraps_comptime {
                spec_rule_at!("CtExpandExpr-CtExpr", &expr.span);
                if !validate_phase2_attribute_list(env, &node.attrs, AttributeTarget::Expression) {
                    return expr_ptr.clone();
                }
                if let Some(replacement) = expand_comptime_expr(expr, &node.attrs, env) {
                    return Some(replacement);
                }
            }
            let mut rewritten = node.clone();
            rewritten.expr = rewrite_expr(&node.expr, env);
            with_node(rewritten.into())
        }
        ExprNode::ComptimeExpr(node) => {
            spec_rule_at!("CtExpandExpr-CtExpr", &expr.span);
            let attr_list = node.attrs_opt.as_deref().unwrap_or(&[]);
            if !validate_phase2_attribute_list(env, attr_list, AttributeTarget::Expression) {
                return expr_ptr.clone();
            }
            if let Some(replacement) = expand_comptime_expr(expr, attr_list, env) {
                return Some(replacement);
            }
            let mut rewritten = node.clone();
            rewritten.body = rewrite_expr(&node.body, env);
            with_node(rewritten.into())
        }
        ExprNode::CtIfExpr(node) => {
            let mut ct_env = env.clone();
            let cond = eval_expr(&node.cond, &mut ct_env);
            if !cond.ok {
                emit_comptime_diag(env, "E-CTE-0080", &expr.span);
                return expr_ptr.clone();
            }
            let Some(cond_bool) = try_get_ct_bool(&cond.value) else {
                emit_comptime_diag(env, "E-CTE-0081", &expr.span);
                return expr_ptr.clone();
            };
            if cond_bool {
                spec_rule_at!("CtExpandExpr-CtIf-True", &expr.span);
            } else {
                spec_rule_at!("CtExpandExpr-CtIf-False", &expr.span);
            }
            spec_rule_at!("requirement.22.ComptimeIfSelectedBranchOnly", &expr.span);
            let selected = if cond_bool { &node.then_block } else { &node.else_block_opt };
            let rewritten = match selected.as_deref() {
                Some(block) => rewrite_block(block, &mut ct_env),
                None => make_fallback_empty_block(&expr.span),
            };
            make_block_expr(rewritten, &expr.span)
        }
        ExprNode::CtLoopIterExpr(node) => {
            spec_rule_at!("CtExpandExpr-CtLoopIter", &expr.span);
            spec_rule_at!("requirement.22.ComptimeLoopIterationSemantics", &expr.span);
            let mut ct_env = env.clone();
            let iter = eval_expr(&node.iter, &mut ct_env);
            if !iter.ok {
                emit_comptime_diag(env, "E-CTE-0082", &expr.span);
                return expr_ptr.clone();
            }
            let Some(elems) = ct_elems(&iter.value) else {
                emit_comptime_diag(env, "E-CTE-0083", &expr.span);
                return expr_ptr.clone();
            };
            let mut unrolled = make_fallback_empty_block(&expr.span);
            let mut loop_env = ct_env;
            if elems.is_empty() {
                spec_rule_at!("CtLoopIterUnroll-Empty", &expr.span);
            }
            let body = node.body.as_deref().expect("comptime loop without a body");
            for elem in elems {
                spec_rule_at!("CtLoopIterUnroll-Cons", &expr.span);
                let mut iter_env = loop_env.clone();
                if !bind_ct_pattern_value(&mut iter_env, &node.pattern, elem) {
                    emit_comptime_diag(env, "E-CTE-0083", &expr.span);
                    return expr_ptr.clone();
                }
                let rewritten = rewrite_block(body, &mut iter_env);
                append_unit_block_stmts(rewritten, &mut unrolled.stmts);
                loop_env = iter_env;
            }
            make_block_expr(unrolled, &expr.span)
        }
        ExprNode::BlockExpr(node) => {
            with_node(BlockExpr { block: rewrite_block_ptr(&node.block, env) }.into())
        }
        ExprNode::BinaryExpr(node) => {
            let mut rewritten = node.clone();
            rewritten.lhs = rewrite_expr(&node.lhs, env);
            rewritten.rhs = rewrite_expr(&node.rhs, env);
            with_node(rewritten.into())
        }
        ExprNode::IfExpr(node) => {
            let mut rewritten = node.clone();
            rewritten.cond = rewrite_expr(&node.cond, env);
            rewritten.then_expr = rewrite_expr(&node.then_expr, env);
            rewritten.else_expr = rewrite_expr(&node.else_expr, env);
            with_node(rewritten.into())
        }
        ExprNode::IfIsExpr(node) => {
            let mut rewritten = node.clone();
            rewritten.scrutinee = rewrite_expr(&node.scrutinee, env);
            rewritten.then_expr = rewrite_expr(&node.then_expr, env);
            rewritten.else_expr = rewrite_expr(&node.else_expr, env);
            with_node(rewritten.into())
        }
        ExprNode::IfCaseExpr(node) => {
            let mut rewritten = node.clone();
            rewritten.scrutinee = rewrite_expr(&node.scrutinee, env);
            for case_clause in &mut rewritten.cases {
                case_clause.body = rewrite_expr(&case_clause.body, env);
            }
            rewritten.else_expr = rewrite_expr(&node.else_expr, env);
            with_node(rewritten.into())
        }
        ExprNode::MethodCallExpr(node) => {
            let mut rewritten = node.clone();
            rewritten.receiver = rewrite_expr(&node.receiver, env);
            rewrite_args(&mut rewritten.args, env);
            with_node(rewritten.into())
        }
        ExprNode::CallExpr(node) => {
            emit_runtime_ct_procedure_reference_if_needed(env, &node.callee);
            let mut rewritten = node.clone();
            rewritten.callee = rewrite_expr(&node.callee, env);
            rewrite_args(&mut rewritten.args, env);
            with_node(rewritten.into())
        }
        _ => expr_ptr.clone(),
    }
}

fn rewrite_item(item: &ASTItem, env: &mut CtEnv) -> ASTItem {
    spec_rule!("requirement.22.CtExpandOrdinaryTraversal");
    match item {
        ASTItem::StaticDecl(node) => {
            let mut out = node.clone();
            out.attrs_opt = out
                .attrs_opt
                .map(|attr_list| strip_attribute(&attr_list, "derive"))
                .filter(|attr_list| !attr_list.is_empty());
            out.binding.init = rewrite_expr(&node.binding.init, env);
            out.into()
        }
        ASTItem::ProcedureDecl(node) => {
            let mut out = node.clone();
            out.attrs = strip_attribute(&out.attrs, "derive");
            out.body = rewrite_block_ptr(&node.body, env);
            out.into()
        }
        ASTItem::RecordDecl(node) => {
            let mut out = node.clone();
            out.attrs = strip_attribute(&out.attrs, "derive");
            out.into()
        }
        ASTItem::EnumDecl(node) => {
            let mut out = node.clone();
            out.attrs = strip_attribute(&out.attrs, "derive");
            out.into()
        }
        ASTItem::ModalDecl(node) => {
            let mut out = node.clone();
            out.attrs = strip_attribute(&out.attrs, "derive");
            out.into()
        }
        _ => item.clone(),
    }
}

/// Expands the items of one module. Items emitted while expanding an item are expanded
/// directly after it. `None` means expansion cannot continue.
pub fn expand_module_items(items: &[ASTItem], env: &mut CtEnv) -> Option<Vec<ASTItem>> {
    let mut queue: Vec<ASTItem> = items.to_vec();
    let visible_current_items = Rc::new(RefCell::new(items.to_vec()));
    env.current_module_items = Some(visible_current_items.clone());
    let mut out = Vec::new();
    if queue.is_empty() {
        spec_rule!("CtExpandItemSeq-Empty");
    }
    let mut index = 0;
    while index < queue.len() {
        spec_rule!("CtExpandItemSeq-Cons");
        *env = with_ct_site(std::mem::take(env), index, &Span::default());
        let item = queue[index].clone();
        index += 1;
        if reject_invalid_derive_attribute_target_kind(env, &item) {
            return None;
        }
        match &item {
            ASTItem::ComptimeProcedureDecl(proc) => {
                if validate_phase2_attribute_list(env, &proc.attrs, AttributeTarget::Procedure) {
                    spec_rule_at!("CtExpandItem-CtProc", &proc.span);
                    *env = bind_ct_proc(std::mem::take(env), proc);
                }
                continue;
            }
            ASTItem::DeriveTargetDecl(derive) => {
                spec_rule_at!("CtExpandItem-DeriveTargetDecl", &derive.span);
                spec_rule_at!("requirement.22.DeriveTargetDeclPhase2Lifetime", &derive.span);
                continue;
            }
            _ => {}
        }
        let explicit_emits = Rc::new(RefCell::new(Vec::new()));
        let mut item_env = env.clone();
        item_env.pending_emits = Some(explicit_emits.clone());
        out.push(rewrite_item(&item, &mut item_env));
        let mut emitted: Vec<ASTItem> = Vec::new();
        if is_derive_annotated_item(&item) {
            let span = span_of_item(&item);
            spec_rule_at!("CtExpandItem-DeriveAnnotatedDecl", &span);
            spec_rule_at!("requirement.22.DeriveTargetExecutionTiming", &span);
            emitted.extend(expand_derives(&item, env)?);
        }
        emitted.extend(explicit_emits.borrow().iter().cloned());
        if !emitted.is_empty() {
            spec_rule_at!("requirement.22.CtPendingEmitsTransfer", &span_of_item(&item));
            visible_current_items.borrow_mut().extend(emitted.iter().cloned());
            queue.splice(index..index, emitted);
        }
    }
    Some(out)
}
