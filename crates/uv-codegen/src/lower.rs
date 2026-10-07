//! Lowering of procedures to IR: the context, literals, blocks, `return`, procedures and
//! modules. A construct that is not ported yet is recorded in `LowerCtx::pending` and
//! the declaration that contains it is not produced. See `05_codegen/lower`.

use std::collections::HashMap;
use std::sync::Arc;

use uv_analysis::context::{NameMapTable, Scope, ScopeContext};
use uv_analysis::layout::lower_type_for_layout;
use uv_analysis::layout::value_bits::encode_const;
use uv_analysis::memory::regions::ProvenanceKind;
use uv_analysis::memory::return_responsibility::call_result_has_responsibility;
use uv_analysis::resolve::scopes::{path_key_of, universe_bindings};
use uv_analysis::typing::expr::small::is_place_expr;
use uv_analysis::typing::expr_store::{selected_call_target, stored_expr_type};
use uv_analysis::typing::type_predicates::{perm_of_type, strip_perm};
use uv_analysis::typing::types::Permission;
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
    /// The names bound in the scope, in order.
    variables: Vec<String>,
    /// What leaving the scope must release: the bindings that hold responsibility.
    cleanup_items: Vec<CleanupItem>,
}

struct CleanupItem {
    name: String,
    binding_id: u64,
}

/// What lowering knows about a name bound in the procedure.
#[derive(Clone)]
struct BindingState {
    ty: TypeRef,
    stable_name: String,
    binding_id: u64,
    has_responsibility: bool,
    is_moved: bool,
}

/// One step of a cleanup plan: the drop of a binding.
struct CleanupAction {
    ty: TypeRef,
}

/// The signature a call is lowered against: the parameters (with the panic out-parameter
/// when there is one) and the return type.
#[derive(Clone, Default)]
pub struct ProcSig {
    pub params: Vec<IrParam>,
    pub ret: TypeRef,
}

/// A value that exists only for the statement that made it.
struct TempValue {
    value: IrValue,
    ty: TypeRef,
    has_responsibility: bool,
}

pub struct LowerCtx<'a, 'b> {
    base: &'a ScopeContext<'b>,
    name_maps: &'a NameMapTable,
    universe: Scope,
    /// The scope of the module being lowered.
    pub scope: ScopeContext<'b>,
    pub module_path: Vec<String>,
    pub proc_ret_type: TypeRef,
    pub current_proc_symbol: Option<String>,
    /// `log_enabled`: false when emitting IR, so runtime traces are `nop`.
    pub log_enabled: bool,
    temp_counter: u64,
    next_binding_id: u64,
    /// False once something that binds names was skipped: the numbers of later bindings,
    /// which show in the IR, would not be those of the reference.
    binding_ids_exact: bool,
    scope_stack: Vec<ScopeInfo>,
    binding_states: HashMap<String, Vec<BindingState>>,
    temp_depth: u32,
    suppress_temp_at_depth: Option<u32>,
    temp_sink: Option<Vec<TempValue>>,
    value_types: HashMap<String, TypeRef>,
    proc_sigs: HashMap<String, ProcSig>,
    /// The first construct met that is not ported, which stops the declaration.
    pub pending: Option<String>,
}

impl<'a, 'b> LowerCtx<'a, 'b> {
    /// `base` carries what typing recorded; `name_maps` give the scope of each module.
    pub fn new(base: &'a ScopeContext<'b>, name_maps: &'a NameMapTable) -> Self {
        LowerCtx {
            base,
            name_maps,
            universe: universe_bindings(),
            scope: base.clone(),
            module_path: Vec::new(),
            proc_ret_type: None,
            current_proc_symbol: None,
            log_enabled: false,
            temp_counter: 0,
            next_binding_id: 1,
            binding_ids_exact: true,
            scope_stack: Vec::new(),
            binding_states: HashMap::new(),
            temp_depth: 0,
            suppress_temp_at_depth: None,
            temp_sink: None,
            value_types: HashMap::new(),
            proc_sigs: HashMap::new(),
            pending: None,
        }
    }

    /// The scope in which the names of a module are read.
    fn scope_for(&self, module_path: &[String]) -> ScopeContext<'b> {
        let mut scope = self.base.clone();
        scope.current_module = module_path.to_vec();
        let module_scope = self.name_maps.get(&path_key_of(module_path)).cloned().unwrap_or_default();
        scope.scopes = vec![Scope::new(), module_scope, self.universe.clone()];
        scope
    }

    fn enter_module(&mut self, module_path: &[String]) {
        self.scope = self.scope_for(module_path);
        self.module_path = module_path.to_vec();
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
        let Some(scope) = self.scope_stack.pop() else {
            return;
        };
        for name in scope.variables.iter().rev() {
            if let Some(states) = self.binding_states.get_mut(name) {
                states.pop();
                if states.is_empty() {
                    self.binding_states.remove(name);
                }
            }
        }
    }

    fn binding_state(&self, name: &str) -> Option<&BindingState> {
        self.binding_states.get(name).and_then(|states| states.last())
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
    fn register_var(&mut self, name: &str, ty: TypeRef, has_responsibility: bool) -> String {
        if !self.binding_ids_exact {
            self.unported("numbering of bindings after a declaration that is not ported");
        }
        let binding_id = self.next_binding_id;
        self.next_binding_id += 1;
        let stable_name = format!("__bind_{binding_id}_{name}");
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.variables.push(name.to_string());
            if has_responsibility {
                scope.cleanup_items.push(CleanupItem { name: name.to_string(), binding_id });
            }
        }
        self.binding_states.entry(name.to_string()).or_default().push(BindingState { ty, stable_name: stable_name.clone(), binding_id, has_responsibility, is_moved: false });
        stable_name
    }

    fn next_literal_id(&mut self) -> u64 {
        self.temp_counter += 1;
        self.temp_counter
    }

    fn value_key(value: &IrValue) -> String {
        match value.kind {
            IrValueKind::Opaque => format!("o:{}", value.name),
            IrValueKind::Local => format!("l:{}", value.name),
            IrValueKind::Symbol => format!("s:{}", value.name),
            IrValueKind::Immediate => format!("i:{}:{}", value.literal_id, value.name),
        }
    }

    fn register_value_type(&mut self, value: &IrValue, ty: TypeRef) {
        self.value_types.insert(Self::value_key(value), ty);
    }

    fn lookup_value_type(&self, value: &IrValue) -> TypeRef {
        self.value_types.get(&Self::value_key(value)).cloned().flatten()
    }

    fn register_temp_value(&mut self, value: &IrValue, ty: &TypeRef, has_responsibility: bool) {
        if !has_responsibility {
            return;
        }
        if let Some(sink) = &mut self.temp_sink {
            sink.push(TempValue { value: value.clone(), ty: ty.clone(), has_responsibility });
        }
    }
}

/// `InlineModeFor`: the mode an `inline` attribute asks for.
fn inline_mode_for(attr: &ast::AttributeItem) -> IrInlineMode {
    for arg in &attr.args {
        if arg.key.as_deref().is_some_and(|key| key != "kind") {
            continue;
        }
        let ast::AttributeArgValue::Token(token) = &arg.value else {
            continue;
        };
        let lexeme = token.lexeme.as_str();
        let mode = if lexeme.len() >= 2 && ((lexeme.starts_with('"') && lexeme.ends_with('"')) || (lexeme.starts_with('\'') && lexeme.ends_with('\''))) { &lexeme[1..lexeme.len() - 1] } else { lexeme };
        return match mode {
            "always" => IrInlineMode::Always,
            "never" => IrInlineMode::Never,
            _ => IrInlineMode::Default,
        };
    }
    IrInlineMode::Default
}

/// The name of an enum variant, for what is reported as not ported.
fn variant_name(node: &impl std::fmt::Debug) -> String {
    let text = format!("{node:?}");
    text.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
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
    let lit_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    match &lit_type {
        Some(ty) if matches!(ty.node, TypeNode::Union(_)) => ctx.unported("literals typed by a union"),
        Some(_) => {
            if let Some(bytes) = encode_const(&lit_type, &lit.literal) {
                value.bytes = bytes;
            }
            ctx.register_value_type(&value, lit_type.clone());
        }
        None => ctx.unported("literals without a recorded type"),
    }
    LowerResult { ir: empty_ir(), value }
}

/// What a call to a procedure of the program is lowered against: its symbol and signature.
struct Callee {
    symbol: String,
    module_path: Vec<String>,
}

/// `PopulateSourceCallSignature` and `RegisterResolvedSourceSignatureIfMissing`.
fn ensure_source_signature(callee: &Callee, decl: &ProcedureDecl, ctx: &mut LowerCtx) {
    if ctx.proc_sigs.contains_key(&callee.symbol) {
        return;
    }
    let scope = ctx.scope_for(&callee.module_path);
    let lower_written = |written: &Option<Arc<ast::Type>>| match written {
        Some(_) => match lower_type(&scope, written) {
            Ok(ty) if ty.is_some() => ty,
            _ => make_type_prim("()"),
        },
        None => make_type_prim("()"),
    };
    let mut sig = ProcSig { params: Vec::new(), ret: lower_written(&decl.return_type_opt) };
    for param in &decl.params {
        sig.params.push(IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), stable_name: String::new(), ty: lower_written(&param.r#type) });
    }
    if needs_panic_out_for_symbol(&callee.symbol) {
        sig.params.push(panic_out_param());
    }
    ctx.proc_sigs.insert(callee.symbol.clone(), sig);
}

/// `NeedsPanicOut` of a symbol of a user procedure: every symbol made from a module path
/// differs from the entry symbol, is not a runtime symbol and is not a record constructor.
fn needs_panic_out_for_symbol(symbol: &str) -> bool {
    symbol != "main"
}

/// `PanicCheck`: after a call, control leaves through the cleanup when a panic is set.
fn panic_check(ctx: &mut LowerCtx) -> IrPtr {
    let trace_ir = emit_runtime_trace(ctx);
    let plan = cleanup_plan_to_function_root(ctx);
    let cleanup_ir = emit_cleanup(&plan, true, ctx);
    seq_ir(vec![Some(trace_ir), Some(Arc::new(Ir::CleanupPanicCheck { cleanup_ir: Some(cleanup_ir) }))])
}

fn lower_call(expr: &Arc<Expr>, call: &ast::CallExpr, ctx: &mut LowerCtx) -> LowerResult {
    let failed = |ctx: &mut LowerCtx, what: &str| {
        ctx.unported(what);
        LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("call") }
    };
    if let Some(callee_expr) = &call.callee {
        let callee_type = stored_expr_type(&ctx.scope, &Some(callee_expr.clone())).flatten();
        if matches!(strip_perm(&callee_type).as_deref().map(|ty| &ty.node), Some(TypeNode::Closure { .. })) {
            return failed(ctx, "calls of closures");
        }
    }
    if !call.generic_args.is_empty() {
        return failed(ctx, "calls with type arguments");
    }
    let Some(target) = selected_call_target(&ctx.scope, call) else {
        return failed(ctx, "calls without a selected procedure");
    };
    if target.generic {
        return failed(ctx, "calls of generic procedures");
    }
    let sigma = ctx.scope.sigma.clone();
    let found = sigma
        .mods
        .iter()
        .filter(|module| module.path == target.module_path)
        .flat_map(|module| module.items.iter())
        .find_map(|item| match item {
            ASTItem::ProcedureDecl(decl) if decl.name == target.proc_name && decl.span == target.proc_span => Some(decl),
            _ => None,
        });
    let Some(decl) = found else {
        return failed(ctx, "calls of procedures that are not in the program");
    };
    if !decl.attrs.is_empty() {
        return failed(ctx, "calls of procedures with attributes");
    }
    if decl.contract.is_some() {
        return failed(ctx, "calls of procedures with contracts");
    }
    if target.module_path.len() == 1 && (target.module_path[0].eq_ignore_ascii_case("string") || target.module_path[0].eq_ignore_ascii_case("bytes")) {
        return failed(ctx, "calls of built-in procedures");
    }
    let callee = Callee { symbol: scoped_sym(&item_path_proc(&target.module_path, &decl.name)), module_path: target.module_path.clone() };
    ensure_source_signature(&callee, decl, ctx);
    if !call.args.is_empty() {
        return failed(ctx, "call arguments");
    }
    let sig = ctx.proc_sigs.get(&callee.symbol).cloned().unwrap_or_default();
    let result_value = ctx.fresh_temp_value("call");
    if sig.ret.is_some() {
        ctx.register_value_type(&result_value, sig.ret.clone());
    }
    let mut args = Vec::new();
    let needs_panic_out = sig.params.last().is_some_and(|param| param.name == PANIC_OUT_NAME);
    if needs_panic_out {
        args.push(IrValue { kind: IrValueKind::Local, name: PANIC_OUT_NAME.to_string(), ..Default::default() });
    }
    let call_ir = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: callee.symbol, ..Default::default() }, args, result: result_value.clone() });
    let mut parts = vec![Some(empty_ir()), Some(empty_ir()), Some(call_ir)];
    if needs_panic_out {
        parts.push(Some(panic_check(ctx)));
    }
    let _ = expr;
    LowerResult { ir: seq_ir(parts), value: result_value }
}

fn needs_refinement_check(expr: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    let key = Arc::as_ptr(expr) as usize;
    ctx.scope.stores.as_ref().is_some_and(|stores| stores.dynamic_refine_checks.borrow().contains_key(&key))
}

/// `LowerIdentifier` for a name bound in the procedure: `ReadVar` of its stable name.
fn lower_identifier(expr: &Arc<Expr>, ident: &ast::IdentifierExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(state) = ctx.binding_state(&ident.name).cloned() else {
        ctx.unported("reads of names that are not local");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") };
    };
    let ir_name = if state.stable_name.is_empty() { ident.name.clone() } else { state.stable_name.clone() };
    let value = IrValue { kind: IrValueKind::Local, name: ir_name.clone(), ..Default::default() };
    let expr_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if state.ty.is_some() {
        ctx.register_value_type(&value, state.ty.clone());
    } else {
        ctx.register_value_type(&value, expr_type.clone());
    }
    // `LowerImplicitKeyAccess`: a key is taken only for a place of shared permission.
    if perm_of_type(&expr_type) == Permission::Shared {
        ctx.unported("implicit key access");
    }
    let key_ir = empty_ir();
    LowerResult { ir: seq_ir(vec![Some(key_ir), Some(Arc::new(Ir::ReadVar { name: ir_name }))]), value }
}

fn lower_expr_impl(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    match &expr.node {
        ExprNode::LiteralExpr(lit) => lower_literal(expr, lit, ctx),
        ExprNode::IdentifierExpr(ident) => lower_identifier(expr, ident, ctx),
        ExprNode::CallExpr(call) => lower_call(expr, call, ctx),
        _ => {
            ctx.unported(&format!("expression {}", variant_name(&expr.node)));
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
        }
    }
}

/// `LowerExpr`: the expression, then the bookkeeping every expression shares.
fn lower_expr(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    ctx.temp_depth += 1;
    let result = lower_expr_impl(expr, ctx);
    let depth = ctx.temp_depth;
    let mut value_type = ctx.lookup_value_type(&result.value);
    if value_type.is_none() {
        if let Some(inferred) = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten() {
            ctx.register_value_type(&result.value, Some(inferred.clone()));
            value_type = Some(inferred);
        }
    }
    ctx.temp_depth -= 1;
    let suppress = ctx.suppress_temp_at_depth == Some(depth);
    if suppress {
        ctx.suppress_temp_at_depth = None;
    }
    if !suppress && ctx.temp_sink.is_some() && !is_place_expr(&Some(expr.clone())) {
        ctx.register_temp_value(&result.value, &value_type, true);
    }
    if needs_refinement_check(expr, ctx) {
        ctx.unported("checks of refinement types");
    }
    result
}

// --------------------------------------------------------------------- cleanup

/// `EmitRuntimeTrace`: a call to the conformance runtime when logging, `nop` otherwise.
fn emit_runtime_trace(ctx: &mut LowerCtx) -> IrPtr {
    if ctx.log_enabled {
        ctx.unported("runtime traces");
    }
    empty_ir()
}

fn append_scope_cleanup(scope: &ScopeInfo, ctx: &LowerCtx, plan: &mut Vec<CleanupAction>) {
    for item in scope.cleanup_items.iter().rev() {
        let state = ctx.binding_states.get(&item.name).and_then(|states| states.iter().rev().find(|state| state.binding_id == item.binding_id));
        let Some(state) = state else {
            continue;
        };
        if !state.has_responsibility || state.is_moved {
            continue;
        }
        plan.push(CleanupAction { ty: state.ty.clone() });
    }
}

/// `ComputeCleanupPlanForCurrentScope`.
fn cleanup_plan_current_scope(ctx: &LowerCtx) -> Vec<CleanupAction> {
    let mut plan = Vec::new();
    if let Some(scope) = ctx.scope_stack.last() {
        append_scope_cleanup(scope, ctx, &mut plan);
    }
    plan
}

/// `ComputeCleanupPlanToFunctionRoot`.
fn cleanup_plan_to_function_root(ctx: &LowerCtx) -> Vec<CleanupAction> {
    let mut plan = Vec::new();
    for scope in ctx.scope_stack.iter().rev() {
        append_scope_cleanup(scope, ctx, &mut plan);
    }
    plan
}

/// `EmitCleanup`, and `EmitCleanupOnPanic` when `on_panic`. An empty plan is a trace. Of a
/// plan, only drops of primitive values are ported, which emit nothing, so what remains
/// is the traces that open and close the cleanup.
fn emit_cleanup(plan: &[CleanupAction], on_panic: bool, ctx: &mut LowerCtx) -> IrPtr {
    if plan.is_empty() {
        return emit_runtime_trace(ctx);
    }
    if on_panic {
        ctx.unported("cleanup on panic");
    }
    let mut parts = vec![Some(emit_runtime_trace(ctx))];
    for action in plan {
        if !matches!(action.ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
            ctx.unported("drops of bindings of this type");
        }
    }
    parts.push(Some(emit_runtime_trace(ctx)));
    seq_ir(parts)
}

/// `TempCleanupIR`: the drops of the temporaries of a statement. Only values of primitive
/// types are ported, whose drop is `nop`.
fn temp_cleanup(temps: &[TempValue], ctx: &mut LowerCtx) -> IrPtr {
    let mut parts = Vec::new();
    for temp in temps.iter().rev() {
        if temp.has_responsibility && temp.ty.is_some() {
            if !matches!(temp.ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
                ctx.unported("drops of temporaries of this type");
            }
            parts.push(Some(empty_ir()));
        }
    }
    seq_ir(parts)
}

// ------------------------------------------------------------------ statements

fn same_ir_value(a: &IrValue, b: &IrValue) -> bool {
    a.kind == b.kind && a.name == b.name && a.bytes == b.bytes && a.literal_id == b.literal_id && a.vtable_sym == b.vtable_sym
}

fn lower_return_stmt(stmt: &ast::ReturnStmt, ctx: &mut LowerCtx) -> IrPtr {
    let mut parts: Vec<Option<IrPtr>> = Vec::new();
    let (value, value_type) = match &stmt.value_opt {
        Some(expr) => {
            match &expr.node {
                ExprNode::LiteralExpr(_) | ExprNode::CallExpr(_) => {}
                // `ReturnDestExpr`: a place is returned as a move unless its binding holds no responsibility.
                ExprNode::IdentifierExpr(ident) => {
                    if ctx.binding_state(&ident.name).is_none_or(|state| state.has_responsibility) {
                        ctx.unported("returning a binding that holds responsibility");
                    }
                }
                _ => ctx.unported(&format!("returning {}", variant_name(&expr.node))),
            }
            let prev_suppress = ctx.suppress_temp_at_depth;
            ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
            let result = lower_expr(expr, ctx);
            ctx.suppress_temp_at_depth = prev_suppress;
            parts.push(Some(result.ir));
            let mut ty = ctx.lookup_value_type(&result.value);
            if ty.is_none() {
                ty = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
            }
            (result.value, ty)
        }
        None => (ctx.fresh_temp_value("unit"), make_type_prim("()")),
    };
    if stmt.value_opt.is_some() && (outcome_sig_of(&ctx.proc_ret_type).is_some() || outcome_sig_of(&value_type).is_some()) {
        ctx.unported("returning into an Outcome");
    }
    if value_type.is_some() {
        ctx.register_value_type(&value, value_type.clone());
    }
    if async_sig_of(&ctx.scope, &ctx.proc_ret_type).is_some() {
        ctx.unported("returns of async procedures");
    }
    // A value that is not immediate is snapshotted when it is not copied bit by bit; the
    // returned values ported so far are immediates, or results of calls of primitive type.
    if value.kind != IrValueKind::Immediate && !matches!(value_type.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) && !is_unit_type(&value_type) {
        ctx.unported("snapshots of returned values");
    }
    if stmt.value_opt.is_some() {
        // The check for a refinement of the returned expression: `nop` when there is none.
        parts.push(Some(empty_ir()));
    }
    // The temporaries of the statement, but the returned value, are dropped before the scopes unwind.
    let temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = Some(Vec::new());
    let cleanup_temps: Vec<TempValue> = temps.into_iter().filter(|temp| !same_ir_value(&temp.value, &value)).collect();
    if !cleanup_temps.is_empty() {
        parts.push(Some(temp_cleanup(&cleanup_temps, ctx)));
    }
    let plan = cleanup_plan_to_function_root(ctx);
    parts.push(Some(emit_cleanup(&plan, false, ctx)));
    parts.push(Some(Arc::new(Ir::Return { value })));
    seq_ir(parts)
}

fn lower_expr_stmt(stmt: &ast::ExprStmt, ctx: &mut LowerCtx) -> IrPtr {
    let Some(value) = &stmt.value else {
        return empty_ir();
    };
    if matches!(value.node, ExprNode::MethodCallExpr(_)) {
        ctx.unported("method calls as statements");
    }
    lower_expr(value, ctx).ir
}

/// `BindingInitializerHasResponsibility`.
fn binding_initializer_has_responsibility(init: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    match &init.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().is_none_or(|inner| binding_initializer_has_responsibility(inner, ctx)),
        ExprNode::MoveExpr(_) | ExprNode::CopyExpr(_) => true,
        ExprNode::LiteralExpr(_) => false,
        _ if is_place_expr(&Some(init.clone())) => false,
        ExprNode::CallExpr(call) => call_result_has_responsibility(&ctx.scope, call).unwrap_or(true),
        _ => true,
    }
}

/// `LowerLetStmt` and `LowerVarStmt`: they differ only in the implicit introduction of an Outcome.
fn lower_binding_stmt(binding: &ast::Binding, is_let: bool, ctx: &mut LowerCtx) -> IrPtr {
    let Some(init) = &binding.init else {
        return empty_ir();
    };
    let typed_pattern_type = match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::TypedPattern(typed)) => typed.r#type.clone(),
        _ => None,
    };
    let annotation = binding.type_opt.clone().or(typed_pattern_type);
    let mut var_type = annotation.and_then(|written| lower_type_for_layout(&ctx.scope, &Some(written))).flatten();
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let init_result = lower_expr(init, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    if var_type.is_none() {
        var_type = stored_expr_type(&ctx.scope, &Some(init.clone())).flatten();
    }
    if var_type.is_none() {
        var_type = ctx.lookup_value_type(&init_result.value);
    }
    if is_let && outcome_sig_of(&var_type).is_some() {
        ctx.unported("implicit introduction of an Outcome");
    }
    let stripped = strip_perm(&var_type).or_else(|| var_type.clone());
    if matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Dynamic(_))) {
        ctx.unported("widening to a dynamic class type");
    }
    // The provenance of the initializer decides that of the binding; only the provenance of
    // a literal, which is bottom and so gives a stack binding, is known without the
    // provenance map of the procedure.
    if !matches!(init.node, ExprNode::LiteralExpr(_)) {
        ctx.unported("provenance of initializers that are not literals");
    }
    let prov = ProvenanceKind::Stack;
    let has_responsibility = binding_initializer_has_responsibility(init, ctx);
    let mut bind_ir = empty_ir();
    let mut checked_value = init_result.value.clone();
    match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::IdentifierPattern(pat)) => {
            let stable_name = ctx.register_var(&pat.name, var_type.clone(), has_responsibility);
            bind_ir = Arc::new(Ir::BindVar { name: pat.name.clone(), stable_name, value: init_result.value.clone(), ty: var_type.clone(), prov, prov_region: None, prov_region_tag: None });
            checked_value = IrValue { kind: IrValueKind::Local, name: pat.name.clone(), ..Default::default() };
            ctx.register_value_type(&checked_value, var_type.clone());
        }
        Some(ast::PatternNode::TypedPattern(pat)) if pat.name != "_" => {
            if matches!(strip_perm(&ctx.lookup_value_type(&init_result.value)).as_deref().map(|ty| &ty.node), Some(TypeNode::Union(_))) {
                ctx.unported("binding a union value to a typed pattern");
            }
            let stable_name = ctx.register_var(&pat.name, var_type.clone(), has_responsibility);
            bind_ir = Arc::new(Ir::BindVar { name: pat.name.clone(), stable_name, value: init_result.value.clone(), ty: var_type.clone(), prov, prov_region: None, prov_region_tag: None });
            checked_value = IrValue { kind: IrValueKind::Local, name: pat.name.clone(), ..Default::default() };
            ctx.register_value_type(&checked_value, var_type.clone());
        }
        Some(ast::PatternNode::WildcardPattern(_)) => {}
        _ => ctx.unported("this kind of binding pattern"),
    }
    let _ = checked_value;
    // The checks for a refinement of the initializer: `nop` when there are none.
    let refine_ir = empty_ir();
    seq_ir(vec![Some(init_result.ir), Some(bind_ir), Some(refine_ir)])
}

fn lower_stmt(stmt: &Stmt, ctx: &mut LowerCtx) -> IrPtr {
    let prev_sink = ctx.temp_sink.replace(Vec::new());
    let mut temps_handled = false;
    let ir = match stmt {
        Stmt::ReturnStmt(node) => {
            temps_handled = true;
            lower_return_stmt(node, ctx)
        }
        Stmt::ExprStmt(node) => lower_expr_stmt(node, ctx),
        Stmt::LetStmt(node) => lower_binding_stmt(&node.binding, true, ctx),
        Stmt::VarStmt(node) => lower_binding_stmt(&node.binding, false, ctx),
        _ => {
            ctx.unported(&format!("statement {}", variant_name(stmt)));
            empty_ir()
        }
    };
    let temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = prev_sink;
    if temps_handled {
        return ir;
    }
    let cleanup = temp_cleanup(&temps, ctx);
    if matches!(cleanup.as_ref(), Ir::Opaque) {
        ir
    } else {
        seq_ir(vec![Some(ir), Some(cleanup)])
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
            let prev_suppress = ctx.suppress_temp_at_depth;
            ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
            let result = lower_expr(tail, ctx);
            ctx.suppress_temp_at_depth = prev_suppress;
            (result.ir, result.value)
        }
        None => {
            let value = ctx.fresh_temp_value("unit");
            ctx.register_value_type(&value, make_type_prim("()"));
            (empty_ir(), value)
        }
    };
    let scope_enter_ir = empty_ir();
    let plan = cleanup_plan_current_scope(ctx);
    let cleanup_ir = emit_cleanup(&plan, false, ctx);
    ctx.pop_scope();
    let setup = seq_ir(vec![Some(scope_enter_ir), Some(stmts_ir)]);
    let body = if !ir_flow_may_fall_through(&Some(tail_ir.clone())) { tail_ir } else { seq_ir(vec![Some(tail_ir), Some(cleanup_ir)]) };
    LowerResult { ir: Arc::new(Ir::Block { setup: Some(setup), body: Some(body), value: value.clone() }), value }
}

// ------------------------------------------------------------------ procedures

fn lower_proc(decl: &ProcedureDecl, module_path: &[String], symbol: String, ctx: &mut LowerCtx) -> ProcIr {
    let mut ir = ProcIr { symbol: symbol.clone(), defining_module_path: module_path.to_vec(), ..Default::default() };
    ctx.module_path = module_path.to_vec();
    ctx.current_proc_symbol = Some(symbol);
    ctx.proc_ret_type = None;
    ctx.pending = None;
    // `inline` and `cold` set what the emitter reads; any other attribute changes the symbol,
    // the ABI or the checks and is not ported.
    for attr in &decl.attrs {
        match attr.name.full_name.as_str() {
            "inline" => ir.inline_mode = inline_mode_for(attr),
            "cold" => ir.cold = true,
            other => ctx.unported(&format!("attribute {other} on procedures")),
        }
    }
    if decl.contract.is_some() {
        ctx.unported("procedure contracts");
    }
    ctx.push_scope();
    for param in &decl.params {
        let mut p = IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), ..Default::default() };
        if let Some(written) = &param.r#type {
            if let Ok(ty) = lower_type(&ctx.scope, &Some(written.clone())) {
                p.ty = ty;
            }
        }
        p.stable_name = ctx.register_var(&param.name, p.ty.clone(), param.mode.is_some());
        ir.params.push(p);
    }
    match &decl.return_type_opt {
        Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
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
        return ir;
    };
    let body_res = lower_block(body, ctx);
    let scope_enter_ir = empty_ir();
    let plan = cleanup_plan_current_scope(ctx);
    let cleanup_ir = emit_cleanup(&plan, false, ctx);
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
    if async_sig_of(&ctx.scope, &ir.ret).is_some() {
        ctx.unported("async procedures");
    }
    if !matches!(ir.ret.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
        ctx.unported("aggregate copy elision for non-primitive returns");
    }
    ctx.current_proc_symbol = None;
    ir
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
    ctx.enter_module(&module.path);
    for item in &module.items {
        let pending_before = out.pending.len();
        match item {
            ASTItem::UsingDecl(_) | ASTItem::TypeAliasDecl(_) | ASTItem::ImportDecl(_) | ASTItem::EnumDecl(_) | ASTItem::ErrorItem(_) => {}
            ASTItem::ProcedureDecl(decl) => {
                if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
                    continue;
                }
                let symbol = scoped_sym(&item_path_proc(&module.path, &decl.name));
                let proc = lower_proc(decl, &module.path, symbol.clone(), ctx);
                ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: proc.params.clone(), ret: proc.ret.clone() });
                match ctx.pending.take() {
                    None => out.decls.push(IrDecl::Proc(proc)),
                    Some(what) => {
                        ctx.binding_ids_exact = false;
                        out.pending.push((symbol, what));
                    }
                }
            }
            ASTItem::StaticDecl(_) => out.pending.push((scoped_sym(&module.path), "statics".to_string())),
            ASTItem::RecordDecl(decl) => out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "records".to_string())),
            ASTItem::ModalDecl(decl) => out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "modals".to_string())),
            ASTItem::ClassDecl(decl) => out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "classes".to_string())),
            ASTItem::ExternBlock(_) => out.pending.push((scoped_sym(&module.path), "extern blocks".to_string())),
            ASTItem::ComptimeProcedureDecl(_) | ASTItem::DeriveTargetDecl(_) => {}
        }
        if out.pending.len() > pending_before {
            ctx.binding_ids_exact = false;
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
