//! Lowering of procedures to IR: the context, literals, blocks, `return`, procedures and
//! modules. A construct that is not ported yet is recorded in `LowerCtx::pending` and
//! the declaration that contains it is not produced. See `05_codegen/lower`.

use std::sync::Arc;

use uv_analysis::context::ScopeContext;
use uv_analysis::layout::value_bits::encode_const;
use uv_analysis::typing::expr_store::stored_expr_type;
use uv_analysis::typing::outcome::outcome_sig_of;
use uv_analysis::typing::type_lookup::async_sig_of;
use uv_analysis::typing::type_lower::lower_type;
use uv_analysis::typing::types::{make_type_prim, ParamMode, TypeNode, TypeRef};
use uv_source::ast::{self, ASTItem, ASTModule, Expr, ExprNode, ProcedureDecl, Stmt};
use uv_source::lexer::token::TokenKind;

use crate::control_flow::ir_flow_may_fall_through;
use crate::ir::*;
use crate::symbols::{deinit_sym, init_sym, item_path_proc, scoped_sym};

pub const PANIC_OUT_NAME: &str = "__panic";

/// What lowering a block or an expression gives: the IR and the value it produced.
pub struct LowerResult {
    pub ir: IrPtr,
    pub value: IrValue,
}

#[derive(Default)]
struct ScopeInfo {
    /// What leaving the scope must release. Nothing adds to it until variables are lowered.
    cleanup_items: usize,
}

pub struct LowerCtx<'a, 'b> {
    pub scope: &'a ScopeContext<'b>,
    pub module_path: Vec<String>,
    pub proc_ret_type: TypeRef,
    pub current_proc_symbol: Option<String>,
    /// `log_enabled`: false when emitting IR, so runtime traces are `nop`.
    pub log_enabled: bool,
    temp_counter: u64,
    next_binding_id: u64,
    scope_stack: Vec<ScopeInfo>,
    /// The first construct met that is not ported, which stops the declaration.
    pub pending: Option<String>,
}

impl<'a, 'b> LowerCtx<'a, 'b> {
    pub fn new(scope: &'a ScopeContext<'b>) -> Self {
        LowerCtx { scope, module_path: Vec::new(), proc_ret_type: None, current_proc_symbol: None, log_enabled: false, temp_counter: 0, next_binding_id: 1, scope_stack: Vec::new(), pending: None }
    }

    fn unported(&mut self, what: &str) {
        if self.pending.is_none() {
            self.pending = Some(what.to_string());
        }
    }

    fn push_scope(&mut self) {
        self.scope_stack.push(ScopeInfo::default());
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn fresh_temp_value(&mut self, prefix: &str) -> IrValue {
        let id = self.temp_counter;
        self.temp_counter += 1;
        let name = match self.current_proc_symbol.as_deref() {
            Some(symbol) if !symbol.is_empty() => format!("{symbol}$tmp${prefix}_{id}"),
            _ => format!("{prefix}_{id}"),
        };
        IrValue { kind: IrValueKind::Opaque, name, ..Default::default() }
    }

    /// `RegisterVar`: a name bound in the current scope; the stable name it is known by in the IR.
    /// A binding that holds responsibility is released when the scope ends.
    fn register_var(&mut self, name: &str, has_responsibility: bool) -> String {
        let id = self.next_binding_id;
        self.next_binding_id += 1;
        if let Some(scope) = self.scope_stack.last_mut() {
            if has_responsibility {
                scope.cleanup_items += 1;
            }
        }
        format!("__bind_{id}_{name}")
    }

    fn next_literal_id(&mut self) -> u64 {
        self.temp_counter += 1;
        self.temp_counter
    }
}

fn is_unit_type(ty: &TypeRef) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "()")
}

/// `NeedsPanicOut` for the symbol of a user procedure: every symbol that comes from a
/// module path is neither the entry symbol, nor a runtime symbol, nor a record constructor.
fn panic_out_param() -> IrParam {
    IrParam { mode: Some(ParamMode::Move), name: PANIC_OUT_NAME.to_string(), stable_name: String::new(), ty: panic_out_type() }
}

fn panic_out_type() -> TypeRef {
    // `PanicOutType()`: a pointer to the panic record.
    make_type_prim("()")
}

// ---------------------------------------------------------------- expressions

fn lower_literal(expr: &Arc<Expr>, lit: &ast::LiteralExpr, ctx: &mut LowerCtx) -> LowerResult {
    let kind = match lit.literal.kind {
        TokenKind::StringLiteral => {
            ctx.unported("string literals");
            Some(IrImmediateLiteralKind::String)
        }
        TokenKind::CharLiteral => Some(IrImmediateLiteralKind::Char),
        TokenKind::IntLiteral => Some(IrImmediateLiteralKind::Int),
        TokenKind::FloatLiteral => Some(IrImmediateLiteralKind::Float),
        _ => None,
    };
    let mut value = IrValue { kind: IrValueKind::Immediate, name: lit.literal.lexeme.clone(), literal_id: ctx.next_literal_id(), literal_kind: kind, ..Default::default() };
    let lit_type = stored_expr_type(ctx.scope, &Some(expr.clone())).flatten();
    match &lit_type {
        Some(ty) if matches!(ty.node, TypeNode::Union(_)) => ctx.unported("literals typed by a union"),
        Some(_) => {
            if let Some(bytes) = encode_const(&lit_type, &lit.literal) {
                value.bytes = bytes;
            }
        }
        None => ctx.unported("literals without a recorded type"),
    }
    LowerResult { ir: empty_ir(), value }
}

fn lower_expr(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    match &expr.node {
        ExprNode::LiteralExpr(lit) => lower_literal(expr, lit, ctx),
        _ => {
            ctx.unported("this kind of expression");
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
        }
    }
}

// --------------------------------------------------------------------- cleanup

/// `EmitRuntimeTrace`: a call to the conformance runtime when logging, `nop` otherwise.
fn emit_runtime_trace(ctx: &mut LowerCtx) -> IrPtr {
    if ctx.log_enabled {
        ctx.unported("runtime traces");
    }
    empty_ir()
}

/// `EmitCleanup` of the plan for the current scope, or of every scope to the function root.
fn emit_cleanup(items: usize, ctx: &mut LowerCtx) -> IrPtr {
    if items != 0 {
        ctx.unported("cleanup of variables");
    }
    emit_runtime_trace(ctx)
}

fn cleanup_items_to_function_root(ctx: &LowerCtx) -> usize {
    ctx.scope_stack.iter().map(|scope| scope.cleanup_items).sum()
}

// ------------------------------------------------------------------ statements

fn lower_return_stmt(stmt: &ast::ReturnStmt, ctx: &mut LowerCtx) -> IrPtr {
    let mut parts: Vec<Option<IrPtr>> = Vec::new();
    let (value, value_type) = match &stmt.value_opt {
        Some(expr) => {
            if !matches!(expr.node, ExprNode::LiteralExpr(_)) {
                ctx.unported("returning places and computed values");
            }
            let result = lower_expr(expr, ctx);
            parts.push(Some(result.ir));
            let ty = stored_expr_type(ctx.scope, &Some(expr.clone())).flatten();
            (result.value, ty)
        }
        None => (ctx.fresh_temp_value("unit"), make_type_prim("()")),
    };
    if stmt.value_opt.is_some() && (outcome_sig_of(&ctx.proc_ret_type).is_some() || outcome_sig_of(&value_type).is_some()) {
        ctx.unported("returning into an Outcome");
    }
    if async_sig_of(ctx.scope, &ctx.proc_ret_type).is_some() {
        ctx.unported("returns of async procedures");
    }
    // The snapshot for a postcondition is not taken for an immediate value, and a
    // procedure with a contract is not ported; the check for a refinement of the
    // returned expression gives `nop` when there is none.
    if stmt.value_opt.is_some() {
        parts.push(Some(empty_ir()));
    }
    let items = cleanup_items_to_function_root(ctx);
    parts.push(Some(emit_cleanup(items, ctx)));
    parts.push(Some(Arc::new(Ir::Return { value })));
    seq_ir(parts)
}

fn lower_stmt(stmt: &Stmt, ctx: &mut LowerCtx) -> IrPtr {
    match stmt {
        Stmt::ReturnStmt(node) => lower_return_stmt(node, ctx),
        _ => {
            ctx.unported("this kind of statement");
            empty_ir()
        }
    }
}

fn lower_stmt_list(stmts: &[Stmt], ctx: &mut LowerCtx) -> IrPtr {
    if stmts.is_empty() {
        return empty_ir();
    }
    let parts = stmts.iter().map(|stmt| Some(lower_stmt(stmt, ctx))).collect();
    seq_ir(parts)
}

pub fn lower_block(block: &ast::Block, ctx: &mut LowerCtx) -> LowerResult {
    ctx.push_scope();
    let stmts_ir = lower_stmt_list(&block.stmts, ctx);
    let (tail_ir, value) = match &block.tail_opt {
        Some(tail) => {
            let result = lower_expr(tail, ctx);
            (result.ir, result.value)
        }
        None => (empty_ir(), ctx.fresh_temp_value("unit")),
    };
    let scope_enter_ir = empty_ir();
    let cleanup_ir = emit_cleanup(ctx.scope_stack.last().map_or(0, |scope| scope.cleanup_items), ctx);
    ctx.pop_scope();
    let setup = seq_ir(vec![Some(scope_enter_ir), Some(stmts_ir)]);
    let body = if !ir_flow_may_fall_through(&Some(tail_ir.clone())) { tail_ir } else { seq_ir(vec![Some(tail_ir), Some(cleanup_ir)]) };
    LowerResult { ir: Arc::new(Ir::Block { setup: Some(setup), body: Some(body), value: value.clone() }), value }
}

// ------------------------------------------------------------------ procedures

fn lower_proc(decl: &ProcedureDecl, module_path: &[String], symbol: String, ctx: &mut LowerCtx) -> Option<ProcIr> {
    let mut ir = ProcIr { symbol: symbol.clone(), defining_module_path: module_path.to_vec(), ..Default::default() };
    ctx.module_path = module_path.to_vec();
    ctx.current_proc_symbol = Some(symbol);
    ctx.proc_ret_type = None;
    ctx.pending = None;
    if !decl.attrs.is_empty() {
        ctx.unported("attributes on procedures");
    }
    if decl.contract.is_some() {
        ctx.unported("procedure contracts");
    }
    ctx.push_scope();
    for param in &decl.params {
        let mut p = IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), ..Default::default() };
        if let Some(written) = &param.r#type {
            if let Ok(ty) = lower_type(ctx.scope, &Some(written.clone())) {
                p.ty = ty;
            }
        }
        p.stable_name = ctx.register_var(&param.name, param.mode.is_some());
        ir.params.push(p);
    }
    match &decl.return_type_opt {
        Some(written) => match lower_type(ctx.scope, &Some(written.clone())) {
            Ok(ty) if ty.is_some() => ir.ret = ty,
            _ => ctx.unported("a return type that does not lower"),
        },
        None => ctx.unported("procedures without a return type"),
    }
    ctx.proc_ret_type = ir.ret.clone();
    // An exported procedure keeps its declared signature; none is ported.
    ir.params.push(panic_out_param());
    let Some(body) = &decl.body else {
        ctx.unported("procedures without a body");
        ctx.pop_scope();
        return None;
    };
    let body_res = lower_block(body, ctx);
    let scope_enter_ir = empty_ir();
    let cleanup_ir = emit_cleanup(ctx.scope_stack.last().map_or(0, |scope| scope.cleanup_items), ctx);
    ctx.pop_scope();
    let mut body_seq: Vec<Option<IrPtr>> = vec![Some(scope_enter_ir), Some(body_res.ir.clone())];
    let may_fall_through = ir_flow_may_fall_through(&Some(body_res.ir.clone()));
    if may_fall_through {
        body_seq.push(Some(cleanup_ir));
    }
    if body.tail_opt.is_some() || (may_fall_through && is_unit_type(&ir.ret)) {
        body_seq.push(Some(Arc::new(Ir::Return { value: body_res.value })));
    }
    ir.body = Some(seq_ir(body_seq));
    // `FinalizeProcIR`: the copy elision analysis and the async state machine are
    // not ported; a procedure that returns an aggregate or an async type is pending.
    if async_sig_of(ctx.scope, &ir.ret).is_some() {
        ctx.unported("async procedures");
    }
    if !matches!(ir.ret.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
        ctx.unported("aggregate copy elision for non-primitive returns");
    }
    ctx.current_proc_symbol = None;
    ctx.pending.is_none().then_some(ir)
}

// --------------------------------------------------------------------- modules

fn module_init_fn(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> Option<ProcIr> {
    ctx.pending = None;
    if module.items.iter().any(|item| matches!(item, ASTItem::StaticDecl(_))) {
        ctx.unported("module initialisation of statics");
    }
    ctx.pending.is_none().then(|| ProcIr { symbol: init_sym(module_path), params: vec![panic_out_param()], ret: make_type_prim("()"), body: Some(empty_ir()), ..Default::default() })
}

fn module_deinit_fn(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> Option<ProcIr> {
    ctx.pending = None;
    if module.items.iter().any(|item| matches!(item, ASTItem::StaticDecl(_))) {
        ctx.unported("module deinitialisation of statics");
    }
    ctx.pending.is_none().then(|| ProcIr { symbol: deinit_sym(module_path), params: vec![panic_out_param()], ret: make_type_prim("()"), body: Some(empty_ir()), ..Default::default() })
}

/// What lowering a module gave: the declarations that are ported, and for each that
/// is not, its symbol and what stopped it.
#[derive(Default)]
pub struct LoweredModule {
    pub decls: IrDecls,
    pub pending: Vec<(String, String)>,
}

pub fn lower_module(module: &ASTModule, ctx: &mut LowerCtx) -> LoweredModule {
    let mut out = LoweredModule::default();
    ctx.module_path = module.path.clone();
    for item in &module.items {
        match item {
            ASTItem::UsingDecl(_) | ASTItem::TypeAliasDecl(_) | ASTItem::ImportDecl(_) | ASTItem::EnumDecl(_) | ASTItem::ErrorItem(_) => {}
            ASTItem::ProcedureDecl(decl) => {
                if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
                    continue;
                }
                let symbol = scoped_sym(&item_path_proc(&module.path, &decl.name));
                match lower_proc(decl, &module.path, symbol.clone(), ctx) {
                    Some(proc) => out.decls.push(IrDecl::Proc(proc)),
                    None => out.pending.push((symbol, ctx.pending.take().unwrap_or_default())),
                }
            }
            ASTItem::StaticDecl(_) => out.pending.push((scoped_sym(&module.path), "statics".to_string())),
            ASTItem::RecordDecl(decl) => out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "records".to_string())),
            ASTItem::ModalDecl(decl) => out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "modals".to_string())),
            ASTItem::ClassDecl(decl) => out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "classes".to_string())),
            ASTItem::ExternBlock(_) => out.pending.push((scoped_sym(&module.path), "extern blocks".to_string())),
            ASTItem::ComptimeProcedureDecl(_) | ASTItem::DeriveTargetDecl(_) => {}
        }
    }
    match module_init_fn(&module.path, module, ctx) {
        Some(proc) => out.decls.push(IrDecl::Proc(proc)),
        None => out.pending.push((init_sym(&module.path), ctx.pending.take().unwrap_or_default())),
    }
    match module_deinit_fn(&module.path, module, ctx) {
        Some(proc) => out.decls.push(IrDecl::Proc(proc)),
        None => out.pending.push((deinit_sym(&module.path), ctx.pending.take().unwrap_or_default())),
    }
    out
}
