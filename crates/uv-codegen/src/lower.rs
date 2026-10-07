//! Lowering of procedures to IR: the context, literals, blocks, `return`, procedures and
//! modules. A construct that is not ported yet is recorded in `LowerCtx::pending` and
//! the declaration that contains it is not produced. See `05_codegen/lower`.

use std::collections::HashMap;
use std::sync::Arc;

use uv_analysis::context::{EntityKind, NameMapTable, Scope, ScopeContext};
use uv_analysis::layout::{align_of, layout_of, lower_type_for_layout, size_of};
use uv_analysis::memory::calls::has_source_provenance;
use uv_analysis::memory::region_prov::{compute_expr_provenance_map, ExprProvMaps};
use uv_analysis::layout::value_bits::encode_const;
use uv_analysis::memory::regions::ProvenanceKind;
use uv_analysis::memory::return_responsibility::call_result_has_responsibility;
use uv_analysis::resolve::scopes::{id_eq, id_key_of, path_key_of, universe_bindings};
use uv_analysis::typing::expr::small::is_place_expr;
use uv_analysis::typing::expr_store::{selected_call_target, stored_expr_type};
use uv_analysis::typing::type_predicates::{bitcopy_type, perm_of_type, strip_perm};
use uv_analysis::typing::types::Permission;
use uv_analysis::typing::outcome::outcome_sig_of;
use uv_analysis::typing::type_lookup::async_sig_of;
use uv_analysis::typing::type_lower::lower_type;
use uv_analysis::typing::type_lookup::{field_type, lookup_record_decl};
use uv_analysis::typing::type_equiv::type_equiv;
use uv_analysis::composite::record_methods::{recv_mode_of, recv_type_for_receiver};
use uv_analysis::typing::types::{is_self_var_path, make_type_closure, make_type_func, make_type_perm, make_type_refine, make_type_slice, make_type_union, make_type_array, applied_type_args, applied_type_path, make_type_modal_state, make_type_path, make_type_prim, make_type_ptr, make_type_raw_ptr, make_type_tuple, ParamMode, PtrState, RawPtrQual, TypeNode, TypeRef};
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
#[derive(Clone)]
struct ScopeInfo {
    /// The names bound in the scope, in order.
    variables: Vec<String>,
    /// What leaving the scope must release: the bindings that hold responsibility.
    cleanup_items: Vec<CleanupItem>,
}

#[derive(Clone)]
struct CleanupItem {
    name: String,
    binding_id: u64,
}

/// What lowering knows about a name bound in the procedure. The provenance is read when
/// addresses and regions are lowered.
#[derive(Clone)]
#[allow(dead_code)]
struct BindingState {
    ty: TypeRef,
    stable_name: String,
    binding_id: u64,
    has_responsibility: bool,
    is_moved: bool,
    prov: ProvenanceKind,
    prov_region: Option<String>,
    prov_region_tag: Option<String>,
}

/// One step of a cleanup plan: the drop of a binding.
struct CleanupAction {
    ty: TypeRef,
}

/// How a value that has no storage of its own was made; the emitter builds it from this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedKind {
    RecordLit,
    TupleLit,
    ArraySegments,
    AddrDeref,
    AddrField,
    Field,
    Tuple,
}

/// One run of an array literal: a single element, or an element repeated.
#[derive(Debug, Clone)]
pub struct DerivedArraySegment {
    pub repeat: bool,
    pub value: IrValue,
    pub count: IrValue,
}

#[derive(Debug, Clone)]
pub struct DerivedValueInfo {
    pub kind: DerivedKind,
    pub base: IrValue,
    pub field: String,
    pub tuple_index: usize,
    pub fields: Vec<(String, IrValue)>,
    pub elements: Vec<IrValue>,
    pub array_segments: Vec<DerivedArraySegment>,
}

impl DerivedValueInfo {
    fn new(kind: DerivedKind) -> Self {
        DerivedValueInfo { kind, base: IrValue::default(), field: String::new(), tuple_index: 0, fields: Vec::new(), elements: Vec::new(), array_segments: Vec::new() }
    }
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
    derived_values: HashMap<String, DerivedValueInfo>,
    /// The provenance of the expressions of the procedure being lowered.
    expr_prov: Option<ExprProvMaps>,
    proc_sigs: HashMap<String, ProcSig>,
    record_ctors: std::collections::HashSet<String>,
    static_types: HashMap<String, TypeRef>,
    /// `ctx.active_static_init_module`: the module whose statics are being initialised.
    active_static_init_module: Option<String>,
    /// The modules of the program and what each needs initialised before it (dependent, dependency).
    init_modules: Vec<Vec<String>>,
    init_eager_edges: Vec<(usize, usize)>,
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
            derived_values: HashMap::new(),
            expr_prov: None,
            proc_sigs: HashMap::new(),
            record_ctors: std::collections::HashSet::new(),
            static_types: HashMap::new(),
            active_static_init_module: None,
            init_modules: Vec::new(),
            init_eager_edges: Vec::new(),
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

    /// What starts again with each module. The binding counter does not: in the reference's
    /// `--emit-ir` path it runs on through every module of the program.
    fn enter_module(&mut self, module_path: &[String]) {
        self.scope = self.scope_for(module_path);
        self.module_path = module_path.to_vec();
        self.scope_stack.clear();
        self.binding_states.clear();
        self.value_types.clear();
        self.temp_depth = 0;
        self.suppress_temp_at_depth = None;
        self.temp_sink = None;
        self.current_proc_symbol = None;
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
    fn register_var(&mut self, name: &str, ty: TypeRef, has_responsibility: bool, prov: ProvenanceKind, prov_region: Option<String>, prov_region_tag: Option<String>) -> String {
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
        self.binding_states.entry(name.to_string()).or_default().push(BindingState { ty, stable_name: stable_name.clone(), binding_id, has_responsibility, is_moved: false, prov, prov_region, prov_region_tag });
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

    pub fn set_init_plan(&mut self, modules: Vec<Vec<String>>, eager_edges: Vec<(usize, usize)>) {
        self.init_modules = modules;
        self.init_eager_edges = eager_edges;
    }

    /// `PoisonSetFor`: the modules that cannot run once the initialisation of one fails: it, and
    /// every module that depends on it, however indirectly.
    fn poison_set_for(&self, module: &str) -> Vec<String> {
        let alone = || vec![module.to_string()];
        let names: Vec<String> = self.init_modules.iter().map(|path| path.join("::")).collect();
        let Some(target) = names.iter().position(|name| name == module) else {
            return alone();
        };
        let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); names.len()];
        for &(dependent, dependency) in &self.init_eager_edges {
            if dependent < names.len() && dependency < names.len() {
                dependents[dependency].push(dependent);
            }
        }
        let mut visited = vec![false; names.len()];
        let mut stack = vec![target];
        visited[target] = true;
        while let Some(current) = stack.pop() {
            for &next in &dependents[current] {
                if !visited[next] {
                    visited[next] = true;
                    stack.push(next);
                }
            }
        }
        let out: Vec<String> = names.into_iter().enumerate().filter(|(index, _)| visited[*index]).map(|(_, name)| name).collect();
        if out.is_empty() { alone() } else { out }
    }

    /// `RegisterDerivedValue`: only opaque and local values carry one.
    fn register_derived_value(&mut self, value: &IrValue, info: DerivedValueInfo) {
        let key = match value.kind {
            IrValueKind::Opaque => format!("o:{}", value.name),
            IrValueKind::Local => format!("l:{}", value.name),
            _ => return,
        };
        self.derived_values.insert(key, info);
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

/// `PanicOutType`: `rawptr[mut, PanicRecord]`.
fn panic_out_type() -> TypeRef {
    make_type_raw_ptr(RawPtrQual::Mut, make_type_path(vec!["PanicRecord".to_string()]))
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

/// `MangleProcInModule`: the symbol of a procedure; procedures of one name in a module get
/// the index of the declaration as a suffix. Procedures with attributes are not ported.
fn mangle_proc_in_module(module: &ASTModule, proc: &ProcedureDecl) -> String {
    let participates = |decl: &ProcedureDecl| decl.name != "main" && decl.attrs.is_empty();
    let base = item_path_proc(&module.path, &proc.name);
    if !participates(proc) {
        return scoped_sym(&base);
    }
    let mut overload_count = 0;
    let mut overload_index = 0;
    for item in &module.items {
        let ASTItem::ProcedureDecl(candidate) = item else {
            continue;
        };
        if candidate.name != proc.name || !participates(candidate) {
            continue;
        }
        if std::ptr::eq(candidate, proc) {
            overload_index = overload_count;
        }
        overload_count += 1;
    }
    if overload_count <= 1 {
        return scoped_sym(&base);
    }
    let mut path = base;
    path.push("$overload".to_string());
    path.push(overload_index.to_string());
    scoped_sym(&path)
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
    if needs_panic_out_for_symbol(&callee.symbol, ctx) {
        sig.params.push(panic_out_param());
    }
    ctx.proc_sigs.insert(callee.symbol.clone(), sig);
}

/// `NeedsPanicOut` of a symbol of a user procedure: every symbol made from a module path
/// differs from the entry symbol, is not a runtime symbol and is not a record constructor.
fn needs_panic_out_for_symbol(symbol: &str, ctx: &LowerCtx) -> bool {
    symbol != "main" && !ctx.record_ctors.contains(symbol)
}

/// `InitPanicHandle`: in the initialisation of a module, a panic poisons the modules that
/// depend on it.
fn init_panic_handle(module: &str, ctx: &mut LowerCtx) -> IrPtr {
    let trace_ir = emit_runtime_trace(ctx);
    let cleanup_ir = emit_cleanup(&[], true, ctx);
    let handle = Arc::new(Ir::InitPanicHandle { module: module.to_string(), poison_modules: ctx.poison_set_for(module), cleanup_ir: Some(cleanup_ir) });
    seq_ir(vec![Some(trace_ir), Some(handle)])
}

/// `PanicFollowup`.
fn panic_check(ctx: &mut LowerCtx) -> IrPtr {
    if let Some(module) = ctx.active_static_init_module.clone() {
        return init_panic_handle(&module, ctx);
    }
    let trace_ir = emit_runtime_trace(ctx);
    let plan = cleanup_plan_to_function_root(ctx);
    let cleanup_ir = emit_cleanup(&plan, true, ctx);
    seq_ir(vec![Some(trace_ir), Some(Arc::new(Ir::CleanupPanicCheck { cleanup_ir: Some(cleanup_ir) }))])
}

/// `LowerAddrOf` of a local name used for the duration of a call (`TransientNoEscape`).
fn lower_addr_of_local(place: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    let ExprNode::IdentifierExpr(ident) = &place.node else {
        ctx.unported(&format!("addresses of {}", variant_name(&place.node)));
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("addr_of") };
    };
    let Some(state) = ctx.binding_state(&ident.name).cloned() else {
        ctx.unported("addresses of names that are not local");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("addr_of") };
    };
    let ptr_value = ctx.fresh_temp_value("addr_of");
    let pointee = state.ty.clone().or_else(|| stored_expr_type(&ctx.scope, &Some(place.clone())).flatten());
    if pointee.is_some() {
        ctx.register_value_type(&ptr_value, make_type_ptr(pointee, Some(PtrState::Valid)));
    }
    let addr = Ir::AddrOf { place: IrPlace { repr: ident.name.clone() }, result: ptr_value.clone(), ref_syms: Vec::new() };
    LowerResult { ir: seq_ir(vec![Some(Arc::new(addr))]), value: ptr_value }
}

/// `LowerRefArgExprWithTemp` and `LowerMoveArgExprWithTemp`: a value that is not a place is
/// bound to a temporary and its address is passed.
fn lower_arg_with_temp(expr: &Arc<Expr>, prefix: &str, expected: &TypeRef, by_move: bool, ctx: &mut LowerCtx) -> LowerResult {
    if !by_move && has_source_provenance(&Some(expr.clone())) {
        return lower_addr_of_local(expr, ctx);
    }
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let value_result = lower_expr(expr, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let temp_name = ctx.fresh_temp_value(prefix).name;
    let mut temp_type = expected.clone();
    if temp_type.is_none() {
        temp_type = ctx.lookup_value_type(&value_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten());
    }
    if temp_type.is_none() {
        temp_type = make_type_prim("()");
    }
    // An argument bound to an Outcome parameter is introduced into it.
    if outcome_sig_of(&temp_type).is_some() {
        ctx.unported("arguments passed to an Outcome parameter");
    }
    let stable_name = ctx.register_var(&temp_name, temp_type.clone(), false, ProvenanceKind::Stack, None, None);
    let bind = Arc::new(Ir::BindVar { name: temp_name.clone(), stable_name, value: value_result.value.clone(), ty: temp_type.clone(), prov: ProvenanceKind::Stack, prov_region: None, prov_region_tag: None });
    let temp_value = IrValue { kind: IrValueKind::Local, name: temp_name.clone(), ..Default::default() };
    if !by_move {
        ctx.register_value_type(&temp_value, temp_type.clone());
        let call_like = matches!(expr.node, ExprNode::CallExpr(_) | ExprNode::MethodCallExpr(_));
        let no_cleanup = matches!(temp_type.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_)));
        let has_responsibility = !(call_like && no_cleanup) && binding_initializer_has_responsibility(expr, ctx);
        ctx.register_temp_value(&temp_value, &temp_type, has_responsibility);
    }
    let temp_ident = Arc::new(Expr { span: expr.span.clone(), node: ExprNode::IdentifierExpr(ast::IdentifierExpr { name: temp_name, from_splice: false }) });
    let addr_result = lower_addr_of_local(&temp_ident, ctx);
    LowerResult { ir: seq_ir(vec![Some(value_result.ir), Some(bind), Some(addr_result.ir)]), value: addr_result.value }
}

/// `LowerArgs`: the arguments left to right, each as the mode of its parameter asks.
fn lower_args(modes: &[Option<ParamMode>], types: &[TypeRef], args: &[ast::Arg], ctx: &mut LowerCtx) -> (IrPtr, Vec<IrValue>) {
    if modes.is_empty() && args.is_empty() {
        return (empty_ir(), Vec::new());
    }
    if modes.len() != args.len() || types.len() != args.len() {
        ctx.unported("calls whose arguments do not match the parameters");
        return (empty_ir(), Vec::new());
    }
    let mut parts = Vec::new();
    let mut values = Vec::new();
    for (index, arg) in args.iter().enumerate() {
        let Some(value) = &arg.value else {
            continue;
        };
        if arg.pass != ast::ArgPassKind::Ref {
            ctx.unported("arguments passed with move or copy");
        }
        if perm_of_type(&types[index]) == Permission::Shared {
            ctx.unported("arguments of shared permission");
        }
        let result = if modes[index].is_some() {
            lower_arg_with_temp(value, "call_move_tmp", &types[index], true, ctx)
        } else {
            lower_arg_with_temp(value, "call_ref_tmp", &types[index], false, ctx)
        };
        parts.push(Some(seq_ir(vec![Some(empty_ir()), Some(result.ir)])));
        values.push(result.value);
    }
    (seq_ir(parts), values)
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
    let sigma = ctx.scope.sigma.clone();
    // The procedure the call names: the one typing selected, or the one the callee names.
    let mut callee_ir = empty_ir();
    let (module, decl) = match selected_call_target(&ctx.scope, call) {
        Some(target) => {
            if target.generic {
                return failed(ctx, "calls of generic procedures");
            }
            let found = sigma.mods.iter().find(|module| module.path == target.module_path).and_then(|module| {
                module.items.iter().find_map(|item| match item {
                    ASTItem::ProcedureDecl(decl) if decl.name == target.proc_name && decl.span == target.proc_span => Some((module, decl)),
                    _ => None,
                })
            });
            match found {
                Some(found) => found,
                None => return failed(ctx, "calls of procedures that are not in the program"),
            }
        }
        None => {
            let Some(callee_expr) = &call.callee else {
                return failed(ctx, "calls without a callee");
            };
            // `ExtractCalleePath`: the path a callee names, and `LowerExpr` of it.
            let (origin, name) = match &callee_expr.node {
                ExprNode::IdentifierExpr(ident) => {
                    if ctx.binding_state(&ident.name).is_some() {
                        return failed(ctx, "calls of local function values");
                    }
                    // `resolve_name`: the origin and the name of a value entity of the module.
                    let (origin, name) = match ctx.scope.module_scope().get(&id_key_of(&ident.name)) {
                        Some(entity) if entity.kind == EntityKind::Value => match &entity.origin_opt {
                            Some(origin) => (origin.clone(), entity.target_opt.clone().unwrap_or_else(|| ident.name.clone())),
                            None => return failed(ctx, "calls of names without an origin"),
                        },
                        _ => (ctx.module_path.clone(), ident.name.clone()),
                    };
                    (origin, name)
                }
                ExprNode::PathExpr(path) => (path.path.clone(), path.name.clone()),
                other => {
                    let kind = variant_name(other);
                    return failed(ctx, &format!("calls through {kind}"));
                }
            };
            let found = sigma.mods.iter().find(|module| module.path == origin).and_then(|module| {
                module.items.iter().find_map(|item| match item {
                    ASTItem::ProcedureDecl(decl) if id_eq(&decl.name, &name) => Some((module, decl)),
                    _ => None,
                })
            });
            let Some((module, decl)) = found else {
                return failed(ctx, "calls of names that are not procedures of the program");
            };
            if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
                return failed(ctx, "calls of generic procedures");
            }
            // `LowerExpr` of the callee: a read of the path of the procedure.
            callee_ir = lower_expr(callee_expr, ctx).ir;
            (module, decl)
        }
    };
    if !decl.attrs.is_empty() {
        return failed(ctx, "calls of procedures with attributes");
    }
    if decl.contract.is_some() {
        return failed(ctx, "calls of procedures with contracts");
    }
    if module.path.len() == 1 && (module.path[0].eq_ignore_ascii_case("string") || module.path[0].eq_ignore_ascii_case("bytes")) {
        return failed(ctx, "calls of built-in procedures");
    }
    let callee = Callee { symbol: mangle_proc_in_module(module, decl), module_path: module.path.clone() };
    ensure_source_signature(&callee, decl, ctx);
    let sig = ctx.proc_sigs.get(&callee.symbol).cloned().unwrap_or_default();
    let source_params: Vec<&IrParam> = sig.params.iter().filter(|param| param.name != PANIC_OUT_NAME).collect();
    let modes: Vec<Option<ParamMode>> = source_params.iter().map(|param| param.mode).collect();
    let types: Vec<TypeRef> = source_params.iter().map(|param| param.ty.clone()).collect();
    let (args_ir, arg_values) = lower_args(&modes, &types, &call.args, ctx);
    let result_value = ctx.fresh_temp_value("call");
    if sig.ret.is_some() {
        ctx.register_value_type(&result_value, sig.ret.clone());
    }
    let mut args = arg_values;
    let needs_panic_out = sig.params.last().is_some_and(|param| param.name == PANIC_OUT_NAME);
    if needs_panic_out {
        args.push(IrValue { kind: IrValueKind::Local, name: PANIC_OUT_NAME.to_string(), ..Default::default() });
    }
    let call_ir = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: callee.symbol, ..Default::default() }, args, result: result_value.clone() });
    let mut parts = vec![Some(callee_ir), Some(args_ir), Some(call_ir)];
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

/// `resolve_name` of the reference's driver: the path of a value entity of the module.
fn resolve_value_path(name: &str, ctx: &LowerCtx) -> Option<Vec<String>> {
    let entity = ctx.scope.module_scope().get(&id_key_of(name))?;
    if entity.kind != EntityKind::Value {
        return None;
    }
    let resolved = entity.target_opt.clone().unwrap_or_else(|| name.to_string());
    let mut full = entity.origin_opt.clone()?;
    full.push(resolved);
    Some(full)
}

/// `LowerStaticIdentifierRead`: a name of the program that is not a local, read through its path.
fn lower_static_identifier_read(expr: &Arc<Expr>, ident: &ast::IdentifierExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(mut full) = resolve_value_path(&ident.name, ctx) else {
        ctx.unported("reads of names that do not resolve to a path");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") };
    };
    let name = full.pop().unwrap_or_default();
    let value = IrValue { kind: IrValueKind::Symbol, name: name.clone(), ..Default::default() };
    let expr_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    ctx.register_value_type(&value, expr_type.clone());
    if perm_of_type(&expr_type) == Permission::Shared {
        ctx.unported("implicit key access");
    }
    LowerResult { ir: seq_ir(vec![Some(empty_ir()), Some(Arc::new(Ir::ReadPath { path: full, name }))]), value }
}

/// `LowerIdentifier` for a name bound in the procedure: `ReadVar` of its stable name.
fn lower_identifier(expr: &Arc<Expr>, ident: &ast::IdentifierExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(state) = ctx.binding_state(&ident.name).cloned() else {
        return lower_static_identifier_read(expr, ident, ctx);
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

fn prim_name(ty: &TypeRef) -> Option<String> {
    match strip_perm(ty).or_else(|| ty.clone()).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Prim(name)) => Some(name.clone()),
        _ => None,
    }
}

fn is_integer_operand(ty: &TypeRef) -> bool {
    prim_name(ty).is_some_and(|name| matches!(name.as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"))
}

fn is_float_operand(ty: &TypeRef) -> bool {
    prim_name(ty).is_some_and(|name| matches!(name.as_str(), "f16" | "f32" | "f64"))
}

fn needs_binary_panic_check(op: &str, lhs: &TypeRef, rhs: &TypeRef) -> bool {
    if op == "<<" || op == ">>" {
        return true;
    }
    if matches!(op, "/" | "%" | "+" | "-" | "*" | "**") {
        if is_float_operand(lhs) || is_float_operand(rhs) {
            return false;
        }
        return is_integer_operand(lhs) || is_integer_operand(rhs) || lhs.is_none() || rhs.is_none();
    }
    false
}

fn bool_immediate(value: bool) -> IrValue {
    IrValue { kind: IrValueKind::Immediate, name: if value { "true" } else { "false" }.to_string(), bytes: vec![u8::from(value)], ..Default::default() }
}

/// The operands of a chain of one short-circuit operator, left to right.
fn collect_short_circuit_operands(expr: &Arc<Expr>, op: &str, out: &mut Vec<Arc<Expr>>) {
    let mut stack = vec![expr.clone()];
    while let Some(current) = stack.pop() {
        if let ExprNode::BinaryExpr(binary) = &current.node {
            if binary.op == op {
                if let (Some(lhs), Some(rhs)) = (&binary.lhs, &binary.rhs) {
                    stack.push(rhs.clone());
                    stack.push(lhs.clone());
                    continue;
                }
            }
        }
        out.push(current);
    }
}

/// What lowering a copy of the context does to the original: the names it bound and the
/// counter of bindings are dropped, what it learned of value types stays.
struct BranchSnapshot {
    scope_stack: Vec<ScopeInfo>,
    binding_states: HashMap<String, Vec<BindingState>>,
    next_binding_id: u64,
    temp_depth: u32,
    suppress_temp_at_depth: Option<u32>,
}

impl LowerCtx<'_, '_> {
    fn begin_branch(&self) -> BranchSnapshot {
        BranchSnapshot { scope_stack: self.scope_stack.clone(), binding_states: self.binding_states.clone(), next_binding_id: self.next_binding_id, temp_depth: self.temp_depth, suppress_temp_at_depth: self.suppress_temp_at_depth }
    }

    fn end_branch(&mut self, snapshot: BranchSnapshot) {
        self.scope_stack = snapshot.scope_stack;
        self.binding_states = snapshot.binding_states;
        self.next_binding_id = snapshot.next_binding_id;
        self.temp_depth = snapshot.temp_depth;
        self.suppress_temp_at_depth = snapshot.suppress_temp_at_depth;
    }
}

/// `LowerShortCircuitChain`: `a && b && c` as nested conditionals; each later operand is
/// lowered in a copy of the context.
fn lower_short_circuit_chain(op: &str, operands: &[Arc<Expr>], ctx: &mut LowerCtx) -> LowerResult {
    let Some(first) = operands.first() else {
        return LowerResult { ir: empty_ir(), value: bool_immediate(op == "&&") };
    };
    let first_result = lower_expr(first, ctx);
    let mut parts = vec![Some(first_result.ir)];
    let mut current = first_result.value;
    for operand in &operands[1..] {
        let snapshot = ctx.begin_branch();
        let rhs = lower_expr(operand, ctx);
        ctx.end_branch(snapshot);
        let result_value = ctx.fresh_temp_value(if op == "&&" { "and" } else { "or" });
        let (then_ir, then_value, else_ir, else_value) = if op == "&&" { (rhs.ir, rhs.value, empty_ir(), bool_immediate(false)) } else { (empty_ir(), bool_immediate(true), rhs.ir, rhs.value) };
        parts.push(Some(Arc::new(Ir::If { cond: current, then_ir: Some(then_ir), then_value, else_ir: Some(else_ir), else_value, result: result_value.clone() })));
        ctx.register_value_type(&result_value, make_type_prim("bool"));
        current = result_value;
    }
    LowerResult { ir: seq_ir(parts), value: current }
}

/// `CleanupTemps`: the drops of the temporaries of a condition or of a branch.
fn cleanup_temps_ir(temps: &[TempValue], ctx: &mut LowerCtx) -> IrPtr {
    if temps.is_empty() {
        return empty_ir();
    }
    let plan: Vec<CleanupAction> = temps.iter().rev().filter(|temp| temp.has_responsibility).map(|temp| CleanupAction { ty: temp.ty.clone() }).collect();
    emit_cleanup(&plan, false, ctx)
}

fn is_noop_ir(ir: &IrPtr) -> bool {
    matches!(ir.as_ref(), Ir::Opaque)
}

/// Lowers an expression into a branch of the context, with temporaries of its own.
fn lower_in_branch(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    let snapshot = ctx.begin_branch();
    let prev_sink = ctx.temp_sink.replace(Vec::new());
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let mut result = lower_expr(expr, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = prev_sink;
    let cleanup = cleanup_temps_ir(&temps, ctx);
    if !is_noop_ir(&cleanup) && ir_flow_may_fall_through(&Some(result.ir.clone())) {
        result.ir = seq_ir(vec![Some(result.ir), Some(cleanup)]);
    }
    ctx.end_branch(snapshot);
    result
}

/// `LowerIfExpr`: the condition, then each branch in a copy of the context.
fn lower_if_expr(expr: &Arc<Expr>, node: &ast::IfExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(cond), Some(then_expr)) = (&node.cond, &node.then_expr) else {
        ctx.unported("conditionals without a condition or a branch");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("if") };
    };
    let prev_sink = ctx.temp_sink.replace(Vec::new());
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let cond_result = lower_expr(cond, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let cond_temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = prev_sink;
    let mut cond_cleanup = cleanup_temps_ir(&cond_temps, ctx);
    if is_noop_ir(&cond_cleanup) {
        cond_cleanup = empty_ir();
    }
    let then_result = lower_in_branch(then_expr, ctx);
    let else_result = match &node.else_expr {
        Some(else_expr) => lower_in_branch(else_expr, ctx),
        None => {
            let value = ctx.fresh_temp_value("unit");
            ctx.register_value_type(&value, make_type_prim("()"));
            LowerResult { ir: empty_ir(), value }
        }
    };
    let result_value = ctx.fresh_temp_value("if");
    let mut result_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if result_type.is_none() && node.else_expr.is_none() {
        result_type = make_type_prim("()");
    }
    if result_type.is_some() {
        ctx.register_value_type(&result_value, result_type);
    }
    let if_ir = Arc::new(Ir::If { cond: cond_result.value, then_ir: Some(then_result.ir), then_value: then_result.value, else_ir: Some(else_result.ir), else_value: else_result.value, result: result_value.clone() });
    LowerResult { ir: seq_ir(vec![Some(cond_result.ir), Some(cond_cleanup), Some(if_ir)]), value: result_value }
}

/// `LowerList`: the expressions left to right, each kept out of the temporaries of the statement.
fn lower_list(exprs: &[Option<Arc<Expr>>], ctx: &mut LowerCtx) -> (IrPtr, Vec<IrValue>) {
    if exprs.is_empty() {
        return (empty_ir(), Vec::new());
    }
    let mut parts = Vec::new();
    let mut values = Vec::new();
    for expr in exprs {
        let Some(expr) = expr else {
            ctx.unported("lists with a missing expression");
            continue;
        };
        let prev_suppress = ctx.suppress_temp_at_depth;
        ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
        let result = lower_expr(expr, ctx);
        ctx.suppress_temp_at_depth = prev_suppress;
        parts.push(Some(result.ir));
        values.push(result.value);
    }
    (seq_ir(parts), values)
}

/// `LowerTuple`: the elements, and a value the emitter builds from them.
fn lower_tuple_expr(expr: &Arc<Expr>, node: &ast::TupleExpr, ctx: &mut LowerCtx) -> LowerResult {
    if node.elements.is_empty() {
        let unit_value = ctx.fresh_temp_value("unit");
        ctx.register_value_type(&unit_value, make_type_prim("()"));
        return LowerResult { ir: empty_ir(), value: unit_value };
    }
    let contextual = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    let (ir, values) = lower_list(&node.elements, ctx);
    let tuple_value = ctx.fresh_temp_value("tuple");
    ctx.register_derived_value(&tuple_value, DerivedValueInfo { elements: values.clone(), ..DerivedValueInfo::new(DerivedKind::TupleLit) });
    let mut tuple_type: TypeRef = None;
    let stripped = strip_perm(&contextual).or(contextual);
    if let Some(TypeNode::Tuple(elements)) = stripped.as_deref().map(|ty| &ty.node) {
        if elements.len() == values.len() {
            tuple_type = stripped.clone();
            for (value, element) in values.iter().zip(elements) {
                ctx.register_value_type(value, element.clone());
            }
        }
    }
    if tuple_type.is_none() {
        let element_types: Vec<TypeRef> = values.iter().map(|value| ctx.lookup_value_type(value)).collect();
        if element_types.iter().all(Option::is_some) {
            tuple_type = make_type_tuple(element_types);
        }
    }
    if tuple_type.is_some() {
        ctx.register_value_type(&tuple_value, tuple_type);
    }
    LowerResult { ir, value: tuple_value }
}

fn is_signed_integer_operand(ty: &TypeRef) -> bool {
    prim_name(ty).is_some_and(|name| matches!(name.as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize"))
}

/// `LowerUnOp`: the operand, the check that negating can overflow, the operation.
fn lower_unary_expr(node: &ast::UnaryExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(operand) = &node.value else {
        ctx.unported("unary expressions without an operand");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unop") };
    };
    let op = node.op.as_str();
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let operand_result = lower_expr(operand, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let operand_type = ctx.lookup_value_type(&operand_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(operand.clone())).flatten());
    if operand_type.is_some() {
        ctx.register_value_type(&operand_result.value, operand_type.clone());
    }
    let result_value = ctx.fresh_temp_value("unop");
    let stripped = strip_perm(&operand_type).or_else(|| operand_type.clone());
    let result_type = match op {
        "widen" => {
            ctx.unported("the operator widen");
            None
        }
        "!" | "-" | "~" => stripped,
        _ => None,
    };
    let mut parts = vec![Some(operand_result.ir)];
    if op == "-" && is_signed_integer_operand(&operand_type) {
        parts.push(Some(Arc::new(Ir::CheckOp { op: op.to_string(), reason: "Overflow".to_string(), lhs: operand_result.value.clone(), rhs: None })));
        parts.push(Some(panic_check(ctx)));
    }
    parts.push(Some(Arc::new(Ir::UnaryOp { op: op.to_string(), operand: operand_result.value.clone(), result: result_value.clone(), operand_type, result_type: result_type.clone() })));
    if result_type.is_some() {
        ctx.register_value_type(&result_value, result_type);
    }
    LowerResult { ir: seq_ir(parts), value: result_value }
}

/// `Lower-Expr-Sizeof` and `Lower-Expr-Alignof`: the size or the alignment of a type, as a constant.
fn lower_layout_constant(written: &Option<Arc<ast::Type>>, align: bool, ctx: &mut LowerCtx) -> LowerResult {
    let lowered = lower_type_for_layout(&ctx.scope, written).flatten();
    let Some(layout) = layout_of(&ctx.scope, &lowered) else {
        ctx.unported("sizes of types whose layout is not known");
        return LowerResult { ir: empty_ir(), value: IrValue::default() };
    };
    let number = if align { layout.align } else { layout.size };
    let value = IrValue { kind: IrValueKind::Immediate, name: number.to_string(), bytes: number.to_le_bytes().to_vec(), ..Default::default() };
    LowerResult { ir: empty_ir(), value }
}

/// `LowerArrayLiteral`: the segments left to right, and a value the emitter builds from them.
fn lower_array_expr(node: &ast::ArrayExpr, ctx: &mut LowerCtx) -> LowerResult {
    let mut parts = Vec::new();
    let mut segments = Vec::new();
    for segment in &node.elements {
        let prev_suppress = ctx.suppress_temp_at_depth;
        ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
        match segment {
            ast::ArraySegment::ArrayElemSegment(element) => {
                let Some(value) = &element.value else {
                    ctx.unported("array elements without a value");
                    continue;
                };
                let result = lower_expr(value, ctx);
                parts.push(Some(result.ir));
                segments.push(DerivedArraySegment { repeat: false, value: result.value, count: IrValue::default() });
            }
            ast::ArraySegment::ArrayRepeatSegment(repeat) => {
                let (Some(value), Some(count)) = (&repeat.value, &repeat.count) else {
                    ctx.unported("repeated array elements without a value");
                    continue;
                };
                let value_result = lower_expr(value, ctx);
                let count_result = lower_expr(count, ctx);
                parts.push(Some(value_result.ir));
                parts.push(Some(count_result.ir));
                segments.push(DerivedArraySegment { repeat: true, value: value_result.value, count: count_result.value });
            }
        }
        ctx.suppress_temp_at_depth = prev_suppress;
    }
    let ir = if segments.is_empty() { empty_ir() } else { seq_ir(parts) };
    let array_value = ctx.fresh_temp_value("array");
    // The concrete array type, when every element has one and they agree.
    let mut element_type: TypeRef = None;
    let mut homogeneous = true;
    let mut element_count = 0u64;
    for segment in &segments {
        let Some(current) = ctx.lookup_value_type(&segment.value) else {
            homogeneous = false;
            break;
        };
        match &element_type {
            None => element_type = Some(current),
            Some(first) => {
                if !type_equiv(&Some(first.clone()), &Some(current)) {
                    homogeneous = false;
                    break;
                }
            }
        }
        if segment.repeat {
            homogeneous = false;
            break;
        }
        element_count += 1;
    }
    ctx.register_derived_value(&array_value, DerivedValueInfo { array_segments: segments, ..DerivedValueInfo::new(DerivedKind::ArraySegments) });
    if homogeneous && element_type.is_some() {
        ctx.register_value_type(&array_value, make_type_array(element_type, element_count, None));
    }
    LowerResult { ir, value: array_value }
}

/// `LowerArrayRepeat`: `[value; count]` as one repeated segment.
fn lower_array_repeat_expr(node: &ast::ArrayRepeatExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(value), Some(count)) = (&node.value, &node.count) else {
        ctx.unported("repeated arrays without a value");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("array_repeat") };
    };
    let value_result = lower_expr(value, ctx);
    let count_result = lower_expr(count, ctx);
    let array_value = ctx.fresh_temp_value("array_repeat");
    let segment = DerivedArraySegment { repeat: true, value: value_result.value, count: count_result.value };
    ctx.register_derived_value(&array_value, DerivedValueInfo { array_segments: vec![segment], ..DerivedValueInfo::new(DerivedKind::ArraySegments) });
    LowerResult { ir: seq_ir(vec![Some(value_result.ir), Some(count_result.ir)]), value: array_value }
}

/// `LowerReadPlaceFieldAccess`: the base is lowered as an expression and the field is read from it.
fn lower_field_access_expr(expr: &Arc<Expr>, node: &ast::FieldAccessExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(base) = &node.base else {
        ctx.unported("field accesses without a base");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("place_field") };
    };
    let base_result = lower_expr(base, ctx);
    let field_value = ctx.fresh_temp_value("place_field");
    let mut type_of_field = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if type_of_field.is_none() {
        // `InferRecordFieldTypeFromBase`.
        let base_type = ctx.lookup_value_type(&base_result.value);
        let mut stripped = base_type;
        while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = stripped.as_deref().map(|ty| &ty.node) {
            stripped = base.clone();
        }
        if let Some(ty) = &stripped {
            if let Some(path) = applied_type_path(ty) {
                if let Some(record) = lookup_record_decl(&ctx.scope, path) {
                    let args: Vec<TypeRef> = applied_type_args(ty).map(<[TypeRef]>::to_vec).unwrap_or_default();
                    type_of_field = field_type(record, &node.name, &ctx.scope, &args).flatten();
                }
            }
        }
    }
    if type_of_field.is_some() {
        ctx.register_value_type(&field_value, type_of_field.clone());
    }
    let mut info = DerivedValueInfo::new(DerivedKind::Field);
    info.base = base_result.value.clone();
    info.field = node.name.clone();
    ctx.register_derived_value(&field_value, info);
    if perm_of_type(&stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten()) == Permission::Shared {
        ctx.unported("implicit key access");
    }
    LowerResult { ir: seq_ir(vec![Some(base_result.ir), Some(empty_ir())]), value: field_value }
}

/// `LowerReadPlaceTupleAccess` of a local: the base is read by its name in the source.
fn lower_tuple_access_expr(expr: &Arc<Expr>, node: &ast::TupleAccessExpr, ctx: &mut LowerCtx) -> LowerResult {
    let base_ident = node.base.as_ref().and_then(|base| match &base.node {
        ExprNode::IdentifierExpr(ident) => Some((base.clone(), ident.clone())),
        _ => None,
    });
    let Some((base, ident)) = base_ident else {
        ctx.unported("tuple accesses on bases that are not local names");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("place_tuple_elem") };
    };
    let Some(state) = ctx.binding_state(&ident.name).cloned() else {
        ctx.unported("tuple accesses on names that are not local");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("place_tuple_elem") };
    };
    let base_value = IrValue { kind: IrValueKind::Local, name: ident.name.clone(), ..Default::default() };
    let base_type = stored_expr_type(&ctx.scope, &Some(base)).flatten();
    ctx.register_value_type(&base_value, if state.ty.is_some() { state.ty.clone() } else { base_type.clone() });
    if perm_of_type(&base_type) == Permission::Shared {
        ctx.unported("implicit key access");
    }
    let base_ir = seq_ir(vec![Some(empty_ir()), Some(Arc::new(Ir::ReadVar { name: ident.name.clone() }))]);
    let elem_value = ctx.fresh_temp_value("place_tuple_elem");
    ctx.register_value_type(&elem_value, stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten());
    let mut info = DerivedValueInfo::new(DerivedKind::Tuple);
    info.base = base_value;
    info.tuple_index = usize::try_from(node.index).unwrap_or(usize::MAX);
    ctx.register_derived_value(&elem_value, info);
    LowerResult { ir: base_ir, value: elem_value }
}

fn is_bool_binop(op: &str) -> bool {
    matches!(op, "==" | "===" | "!=" | "<" | "<=" | ">" | ">=" | "&&" | "||")
}

/// `LowerBinOp`: both operands left to right, the check that can panic, the operation.
fn lower_binary_expr(node: &ast::BinaryExpr, ctx: &mut LowerCtx) -> LowerResult {
    let op = node.op.as_str();
    let (Some(lhs), Some(rhs)) = (&node.lhs, &node.rhs) else {
        ctx.unported("binary expressions without operands");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("binop") };
    };
    if matches!(op, "&&" | "||") {
        let mut operands = Vec::new();
        collect_short_circuit_operands(lhs, op, &mut operands);
        collect_short_circuit_operands(rhs, op, &mut operands);
        return lower_short_circuit_chain(op, &operands, ctx);
    }
    if op == "<:" {
        ctx.unported("the operator <:");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("binop") };
    }
    let lhs_result = lower_expr(lhs, ctx);
    let rhs_result = lower_expr(rhs, ctx);
    let result_value = ctx.fresh_temp_value("binop");
    let mut parts = vec![Some(lhs_result.ir), Some(rhs_result.ir)];
    let lhs_type = ctx.lookup_value_type(&lhs_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(lhs.clone())).flatten());
    let rhs_type = ctx.lookup_value_type(&rhs_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(rhs.clone())).flatten());
    if needs_binary_panic_check(op, &lhs_type, &rhs_type) {
        let reason = match op {
            "/" | "%" => "DivZero",
            "<<" | ">>" => "Shift",
            _ => "Overflow",
        };
        parts.push(Some(Arc::new(Ir::CheckOp { op: op.to_string(), reason: reason.to_string(), lhs: lhs_result.value.clone(), rhs: Some(rhs_result.value.clone()) })));
        parts.push(Some(panic_check(ctx)));
    }
    parts.push(Some(Arc::new(Ir::BinaryOp { op: op.to_string(), lhs: lhs_result.value.clone(), rhs: rhs_result.value.clone(), result: result_value.clone() })));
    if is_bool_binop(op) {
        ctx.register_value_type(&result_value, make_type_prim("bool"));
    } else if let Some(ty) = ctx.lookup_value_type(&lhs_result.value).or_else(|| ctx.lookup_value_type(&rhs_result.value)) {
        ctx.register_value_type(&result_value, Some(ty));
    }
    LowerResult { ir: seq_ir(parts), value: result_value }
}

/// `LowerFieldInits`: the initializers left to right, each kept out of the temporaries of the statement.
fn lower_field_inits(fields: &[ast::FieldInit], ctx: &mut LowerCtx) -> (IrPtr, Vec<(String, IrValue)>) {
    if fields.is_empty() {
        return (empty_ir(), Vec::new());
    }
    let mut parts = Vec::new();
    let mut values = Vec::new();
    for field in fields {
        let Some(value) = &field.value else {
            ctx.unported("field initializers without a value");
            continue;
        };
        let prev_suppress = ctx.suppress_temp_at_depth;
        ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
        let result = lower_expr(value, ctx);
        ctx.suppress_temp_at_depth = prev_suppress;
        parts.push(Some(result.ir));
        values.push((field.name.clone(), result.value));
    }
    (seq_ir(parts), values)
}

/// `LowerRecord`: the value of a record expression is built by the emitter from its fields.
fn lower_record_expr(expr: &Arc<Expr>, node: &ast::RecordExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (ir, field_values) = lower_field_inits(&node.fields, ctx);
    // The type typing gave the expression, when it names the target; else the target written.
    let typed = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    let typed = strip_perm(&typed).or(typed);
    let checked = typed.as_ref().and_then(|typed| match (&node.target, &typed.node) {
        (ast::RecordExprTarget::Path(target), _) => applied_type_path(typed).filter(|path| *path == target).map(|_| Some(typed.clone())),
        (ast::RecordExprTarget::ModalStateRef(target), TypeNode::ModalState(modal)) => (modal.path == target.path && modal.state == target.state).then(|| Some(typed.clone())),
        _ => None,
    });
    let record_type = match checked.flatten() {
        Some(ty) => Some(ty),
        None => match &node.target {
            ast::RecordExprTarget::Path(path) => make_type_path(path.clone()),
            ast::RecordExprTarget::ModalStateRef(target) => {
                let mut args = Vec::new();
                for arg in &target.generic_args {
                    match lower_type(&ctx.scope, arg) {
                        Ok(ty) if ty.is_some() => args.push(ty),
                        _ => {
                            ctx.unported("record targets whose type arguments do not lower");
                            break;
                        }
                    }
                }
                make_type_modal_state(target.path.clone(), &target.state, args)
            }
        },
    };
    // A field of an Outcome type takes a bare initializer as a value of it.
    let mut outcome_field = false;
    if let Some(ty) = &record_type {
        let stripped = strip_perm(&record_type).unwrap_or_else(|| ty.clone());
        if let Some(path) = applied_type_path(&stripped) {
            if let Some(record) = lookup_record_decl(&ctx.scope, path) {
                let args: Vec<TypeRef> = applied_type_args(&stripped).map(<[TypeRef]>::to_vec).unwrap_or_default();
                outcome_field = field_values.iter().any(|(name, _)| field_type(record, name, &ctx.scope, &args).is_some_and(|field| outcome_sig_of(&field).is_some()));
            }
        }
    }
    if outcome_field {
        ctx.unported("record fields of an Outcome type");
    }
    let record_value = ctx.fresh_temp_value("record");
    ctx.register_derived_value(&record_value, DerivedValueInfo { fields: field_values, ..DerivedValueInfo::new(DerivedKind::RecordLit) });
    if record_type.is_some() {
        ctx.register_value_type(&record_value, record_type);
    }
    LowerResult { ir, value: record_value }
}

/// `LowerPath`: a read of the path of an item.
fn lower_path_expr(node: &ast::PathExpr) -> LowerResult {
    let value = IrValue { kind: IrValueKind::Symbol, name: node.name.clone(), ..Default::default() };
    LowerResult { ir: Arc::new(Ir::ReadPath { path: node.path.clone(), name: node.name.clone() }), value }
}

fn lower_expr_impl(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    match &expr.node {
        ExprNode::LiteralExpr(lit) => lower_literal(expr, lit, ctx),
        ExprNode::IdentifierExpr(ident) => lower_identifier(expr, ident, ctx),
        ExprNode::PathExpr(node) => lower_path_expr(node),
        ExprNode::BinaryExpr(node) => lower_binary_expr(node, ctx),
        ExprNode::RecordExpr(node) => lower_record_expr(expr, node, ctx),
        ExprNode::IfExpr(node) => lower_if_expr(expr, node, ctx),
        ExprNode::TupleExpr(node) => lower_tuple_expr(expr, node, ctx),
        ExprNode::FieldAccessExpr(node) => lower_field_access_expr(expr, node, ctx),
        ExprNode::TupleAccessExpr(node) => lower_tuple_access_expr(expr, node, ctx),
        ExprNode::ArrayExpr(node) => lower_array_expr(node, ctx),
        ExprNode::ArrayRepeatExpr(node) => lower_array_repeat_expr(node, ctx),
        ExprNode::SizeofExpr(node) => lower_layout_constant(&node.r#type, false, ctx),
        ExprNode::AlignofExpr(node) => lower_layout_constant(&node.r#type, true, ctx),
        ExprNode::UnaryExpr(node) => lower_unary_expr(node, ctx),
        ExprNode::BlockExpr(node) => match &node.block {
            Some(block) => lower_block(block, ctx),
            None => {
                ctx.unported("block expressions without a block");
                LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
            }
        },
        ExprNode::UnsafeBlockExpr(node) => match &node.block {
            Some(block) => lower_block(block, ctx),
            None => {
                ctx.unported("unsafe blocks without a block");
                LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
            }
        },
        ExprNode::CallExpr(call) => lower_call(expr, call, ctx),
        ExprNode::MethodCallExpr(call) => {
            let receiver = stored_expr_type(&ctx.scope, &Some(call.receiver.clone().unwrap_or_default())).flatten();
            let kind = strip_perm(&receiver).map(|ty| variant_name(&ty.node)).unwrap_or_default();
            ctx.unported(&format!("method call {} on {kind}", call.name));
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
        }
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

/// `BuildPanicAccess`: the addresses of the fields of the panic record behind `__panic`.
fn build_panic_access(ctx: &mut LowerCtx) -> (IrValue, IrValue) {
    let panic_ptr = ctx.fresh_temp_value("panic_ptr");
    let mut deref = DerivedValueInfo::new(DerivedKind::AddrDeref);
    deref.base = IrValue { kind: IrValueKind::Local, name: PANIC_OUT_NAME.to_string(), ..Default::default() };
    ctx.register_derived_value(&panic_ptr, deref);
    ctx.register_value_type(&panic_ptr, panic_out_type());
    let field_ptr = |ctx: &mut LowerCtx, prefix: &str, field: &str, element: &str| {
        let ptr = ctx.fresh_temp_value(prefix);
        let mut info = DerivedValueInfo::new(DerivedKind::AddrField);
        info.base = panic_ptr.clone();
        info.field = field.to_string();
        ctx.register_derived_value(&ptr, info);
        ctx.register_value_type(&ptr, make_type_raw_ptr(RawPtrQual::Mut, make_type_prim(element)));
        ptr
    };
    let flag_ptr = field_ptr(ctx, "panic_flag_ptr", "panic", "bool");
    let code_ptr = field_ptr(ctx, "panic_code_ptr", "code", "u32");
    (flag_ptr, code_ptr)
}

/// `EmitCleanup`, and `EmitCleanupOnPanic` when `on_panic`. An empty plan is a trace. Of a
/// plan, only drops of primitive values are ported, which emit nothing, so what remains
/// is the traces that open and close the cleanup, and on a panic the read of the record.
fn emit_cleanup(plan: &[CleanupAction], on_panic: bool, ctx: &mut LowerCtx) -> IrPtr {
    if plan.is_empty() {
        return emit_runtime_trace(ctx);
    }
    let (flag_ptr, code_ptr) = build_panic_access(ctx);
    let mut parts = Vec::new();
    if on_panic {
        let flag = ctx.fresh_temp_value("panic_flag");
        ctx.register_value_type(&flag, make_type_prim("bool"));
        let code = ctx.fresh_temp_value("panic_code");
        ctx.register_value_type(&code, make_type_prim("u32"));
        let snapshot = seq_ir(vec![Some(Arc::new(Ir::ReadPtr { ptr: flag_ptr, result: flag })), Some(Arc::new(Ir::ReadPtr { ptr: code_ptr, result: code }))]);
        parts.push(Some(snapshot));
    }
    parts.push(Some(emit_runtime_trace(ctx)));
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
                // `ReturnDestExpr`: a place is returned as a move unless its binding holds no responsibility.
                ExprNode::IdentifierExpr(ident) => {
                    if ctx.binding_state(&ident.name).is_none_or(|state| state.has_responsibility) {
                        ctx.unported("returning a binding that holds responsibility");
                    }
                }
                ExprNode::FieldAccessExpr(_) | ExprNode::TupleAccessExpr(_) | ExprNode::IndexAccessExpr(_) | ExprNode::DerefExpr(_) => {
                    let root = place_root(expr);
                    if root.is_none_or(|name| ctx.binding_state(&name).is_none_or(|state| state.has_responsibility)) {
                        ctx.unported("returning a place of a binding that holds responsibility");
                    }
                }
                _ => {}
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
    // `SnapshotReturnValueForPostcondition`: a value that is not copied bit by bit is bound to
    // a local before the cleanup that may drop what it was made from. There is no
    // postcondition to read it.
    let mut value = value;
    let needs_snapshot = value_type.is_some() && !is_unit_type(&value_type) && !bitcopy_type(&ctx.scope, &value_type);
    if needs_snapshot && value.kind != IrValueKind::Immediate {
        let name = ctx.fresh_temp_value("return_snapshot").name;
        let stable_name = ctx.register_var(&name, value_type.clone(), false, ProvenanceKind::Bottom, None, None);
        parts.push(Some(Arc::new(Ir::BindVar { name: name.clone(), stable_name, value: value.clone(), ty: value_type.clone(), prov: ProvenanceKind::Stack, prov_region: None, prov_region_tag: None })));
        value = IrValue { kind: IrValueKind::Local, name, ..Default::default() };
        ctx.register_value_type(&value, value_type.clone());
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

/// `PlaceRoot`: the name a place starts from.
fn place_root(expr: &Arc<Expr>) -> Option<String> {
    match &expr.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().and_then(place_root),
        ExprNode::IdentifierExpr(node) => Some(node.name.clone()),
        ExprNode::FieldAccessExpr(node) => node.base.as_ref().and_then(place_root),
        ExprNode::TupleAccessExpr(node) => node.base.as_ref().and_then(place_root),
        ExprNode::IndexAccessExpr(node) => node.base.as_ref().and_then(place_root),
        ExprNode::DerefExpr(node) => node.value.as_ref().and_then(place_root),
        _ => None,
    }
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
    // `BindProvInfo`: a binding takes the provenance of its initializer, and a value whose
    // provenance is bottom lives on the stack.
    let init_key = Arc::as_ptr(init) as usize;
    let init_kind = ctx.expr_prov.as_ref().and_then(|maps| maps.prov.get(&init_key).copied()).unwrap_or(ProvenanceKind::Bottom);
    let (prov, prov_region, prov_region_tag) = if init_kind == ProvenanceKind::Bottom {
        (ProvenanceKind::Stack, None, None)
    } else if init_kind == ProvenanceKind::Region {
        let maps = ctx.expr_prov.as_ref();
        let region = maps.and_then(|maps| maps.region_targets.get(&init_key)).cloned();
        let tag = maps.and_then(|maps| maps.region_tags.get(&init_key)).cloned();
        // `StableRegionProv`: the region is named by the stable name of its binding.
        let region = region.map(|name| ctx.binding_state(&name).map_or(name, |state| state.stable_name.clone()));
        (init_kind, region, tag)
    } else {
        (init_kind, None, None)
    };
    let has_responsibility = binding_initializer_has_responsibility(init, ctx);
    let mut bind_ir = empty_ir();
    let mut checked_value = init_result.value.clone();
    match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::IdentifierPattern(pat)) => {
            let stable_name = ctx.register_var(&pat.name, var_type.clone(), has_responsibility, prov, prov_region.clone(), prov_region_tag.clone());
            bind_ir = Arc::new(Ir::BindVar { name: pat.name.clone(), stable_name, value: init_result.value.clone(), ty: var_type.clone(), prov, prov_region: prov_region.clone(), prov_region_tag: prov_region_tag.clone() });
            checked_value = IrValue { kind: IrValueKind::Local, name: pat.name.clone(), ..Default::default() };
            ctx.register_value_type(&checked_value, var_type.clone());
        }
        Some(ast::PatternNode::TypedPattern(pat)) if pat.name != "_" => {
            if matches!(strip_perm(&ctx.lookup_value_type(&init_result.value)).as_deref().map(|ty| &ty.node), Some(TypeNode::Union(_))) {
                ctx.unported("binding a union value to a typed pattern");
            }
            let stable_name = ctx.register_var(&pat.name, var_type.clone(), has_responsibility, prov, prov_region.clone(), prov_region_tag.clone());
            bind_ir = Arc::new(Ir::BindVar { name: pat.name.clone(), stable_name, value: init_result.value.clone(), ty: var_type.clone(), prov, prov_region: prov_region.clone(), prov_region_tag: prov_region_tag.clone() });
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
    ctx.expr_prov = compute_expr_provenance_map(&ctx.scope, module_path, &decl.params, &decl.body, None);
    ctx.push_scope();
    for param in &decl.params {
        let mut p = IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), ..Default::default() };
        if let Some(written) = &param.r#type {
            if let Ok(ty) = lower_type(&ctx.scope, &Some(written.clone())) {
                p.ty = ty;
            }
        }
        p.stable_name = ctx.register_var(&param.name, p.ty.clone(), param.mode.is_some(), ProvenanceKind::Param, None, None);
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

// ------------------------------------------------------------- record methods

/// `SubstSelfType` of the lowering: `Self` replaced by the type of the record, through the
/// forms the reference walks.
fn subst_self(self_type: &TypeRef, ty: &TypeRef) -> TypeRef {
    let (Some(_), Some(node)) = (self_type, ty.as_deref()) else {
        return ty.clone();
    };
    let sub = |inner: &TypeRef| subst_self(self_type, inner);
    match &node.node {
        TypeNode::Path { path, .. } if is_self_var_path(path) => self_type.clone(),
        TypeNode::Perm { perm, base } => make_type_perm(*perm, sub(base)),
        TypeNode::Tuple(elements) => make_type_tuple(elements.iter().map(sub).collect()),
        TypeNode::Array { element, length, .. } => make_type_array(sub(element), *length, None),
        TypeNode::Slice(element) => make_type_slice(sub(element)),
        TypeNode::Union(members) => make_type_union(members.iter().map(sub).collect()),
        TypeNode::Func { params, ret } => make_type_func(params.iter().map(|param| uv_analysis::typing::types::TypeFuncParam { mode: param.mode, r#type: sub(&param.r#type) }).collect(), sub(ret)),
        TypeNode::Closure { params, ret, deps_opt } => make_type_closure(
            params.iter().map(|(is_move, ty)| (*is_move, sub(ty))).collect(),
            sub(ret),
            deps_opt.as_ref().map(|deps| deps.iter().map(|dep| uv_analysis::typing::types::SharedDep { name: dep.name.clone(), r#type: sub(&dep.r#type) }).collect()),
        ),
        TypeNode::Ptr { element, state } => make_type_ptr(sub(element), *state),
        TypeNode::RawPtr { qual, element } => make_type_raw_ptr(*qual, sub(element)),
        TypeNode::Refine { base, predicate } => make_type_refine(sub(base), predicate.clone()),
        _ => ty.clone(),
    }
}

/// `LowerParam`: the parameter as the IR declares it.
fn lower_param(param: &ast::Param, self_type: &TypeRef, ctx: &LowerCtx) -> IrParam {
    let mut out = IrParam { mode: param.mode.map(|_| ParamMode::Move), name: param.name.clone(), stable_name: param.name.clone(), ty: None };
    if param.r#type.is_some() {
        if let Ok(ty) = lower_type(&ctx.scope, &param.r#type) {
            out.ty = subst_self(self_type, &ty);
        }
    }
    out
}

/// `LowerProcLike`: a procedure made of parts: a method or a transition. It reads no
/// attributes and has no contract, and the expressions have no provenance map.
fn lower_proc_like(symbol: &str, params: &[IrParam], ret_type: &TypeRef, body: &ast::Block, module_path: &[String], ctx: &mut LowerCtx) -> ProcIr {
    let mut ir = ProcIr { symbol: symbol.to_string(), ..Default::default() };
    ctx.module_path = module_path.to_vec();
    ctx.current_proc_symbol = Some(symbol.to_string());
    ctx.expr_prov = None;
    ctx.push_scope();
    for param in params {
        let mut lowered = param.clone();
        lowered.stable_name = ctx.register_var(&param.name, param.ty.clone(), param.mode.is_some(), ProvenanceKind::Bottom, None, None);
        ir.params.push(lowered);
    }
    ir.ret = if ret_type.is_some() { ret_type.clone() } else { make_type_prim("()") };
    ctx.proc_ret_type = ir.ret.clone();
    ir.params.push(panic_out_param());
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
    ctx.current_proc_symbol = None;
    ir
}

/// `LowerRecordMethod`.
fn lower_record_method(record: &ast::RecordDecl, method: &ast::MethodDecl, module_path: &[String], ctx: &mut LowerCtx) -> Option<ProcIr> {
    let Some(body) = &method.body else {
        ctx.unported("methods without a body");
        return None;
    };
    if method.attrs.iter().chain(&record.attrs).any(|attr| attr.name.full_name == "dynamic") {
        ctx.unported("dynamic checks in methods");
    }
    let mut record_path = module_path.to_vec();
    record_path.push(record.name.clone());
    let self_type = make_type_path(record_path.clone());
    let recv_type = recv_type_for_receiver(&self_type, &method.receiver, |written| match lower_type_for_layout(&ctx.scope, written).flatten() {
        Some(ty) => Ok(Some(ty)),
        None => Err(None),
    });
    let recv_mode = recv_mode_of(&method.receiver);
    let mut params = vec![IrParam { mode: recv_mode, name: "self".to_string(), stable_name: "self".to_string(), ty: recv_type.ok().filter(Option::is_some).unwrap_or_else(|| self_type.clone()) }];
    for param in &method.params {
        params.push(lower_param(param, &self_type, ctx));
    }
    let ret_type = match &method.return_type_opt {
        None => make_type_prim("()"),
        Some(written) => match lower_type(&ctx.scope, &Some(written.clone())) {
            Ok(ty) if ty.is_some() => subst_self(&self_type, &ty),
            _ => None,
        },
    };
    let symbol = scoped_sym(&item_path_proc(&record_path, &method.name));
    let mut proc = lower_proc_like(&symbol, &params, &ret_type, body, module_path, ctx);
    // `ApplyProcAttrs`.
    for attr in &method.attrs {
        match attr.name.full_name.as_str() {
            "inline" => proc.inline_mode = inline_mode_for(attr),
            "cold" => proc.cold = true,
            other => ctx.unported(&format!("attribute {other} on methods")),
        }
    }
    Some(proc)
}

// --------------------------------------------------------------------- statics

/// `StaticName`: the one name a static binds, when it binds one.
fn static_name(binding: &ast::Binding) -> Option<String> {
    match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::IdentifierPattern(pat)) => Some(pat.name.clone()),
        Some(ast::PatternNode::TypedPattern(pat)) if pat.name != "_" => Some(pat.name.clone()),
        _ => None,
    }
}

/// The scope the reference reads layouts in: the program and the module, no names.
fn layout_scope<'b>(ctx: &LowerCtx<'_, 'b>, module_path: &[String]) -> ScopeContext<'b> {
    let mut scope = ctx.base.clone();
    scope.current_module = module_path.to_vec();
    scope.scopes.clear();
    scope.name_resolution_tables = None;
    scope
}

/// `StaticInitTypeForGlobal`: the written type of a static, or the type of its initializer.
fn static_init_type(item: &ast::StaticDecl, module_path: &[String], ctx: &LowerCtx) -> TypeRef {
    let scope = layout_scope(ctx, module_path);
    let annotation = ast::binding_annotation_type_opt(&item.binding);
    if annotation.is_some() {
        return lower_type_for_layout(&scope, &annotation).flatten();
    }
    item.binding.init.as_ref().and_then(|init| stored_expr_type(&ctx.scope, &Some(init.clone())).flatten())
}

fn is_unit_type_ref(ty: &TypeRef) -> bool {
    let mut current = ty.clone();
    loop {
        match current.as_deref().map(|ty| &ty.node) {
            Some(TypeNode::Prim(name)) => return name == "()",
            Some(TypeNode::Perm { base, .. }) | Some(TypeNode::Refine { base, .. }) => current = base.clone(),
            _ => return false,
        }
    }
}

/// `EmitGlobal`: the storage of a static; a constant initializer of an immutable static
/// is stored in the declaration, any other is set when the module starts.
fn emit_global(item: &ast::StaticDecl, module_path: &[String], ctx: &mut LowerCtx) -> Vec<IrDecl> {
    let binding = &item.binding;
    let Some(name) = static_name(binding) else {
        ctx.unported("statics that bind several names");
        return Vec::new();
    };
    let symbol = scoped_sym(&item_path_proc(module_path, &name));
    let externally_visible = matches!(item.vis, ast::Visibility::Public | ast::Visibility::Internal);
    let export_from_shared_library = item.vis == ast::Visibility::Public;
    let init_type = static_init_type(item, module_path, ctx);
    if init_type.is_some() {
        ctx.static_types.insert(symbol.clone(), init_type.clone());
    }
    let scope = layout_scope(ctx, module_path);
    let (Some(size), Some(align)) = (size_of(&scope, &init_type), align_of(&scope, &init_type)) else {
        ctx.unported("statics whose layout is not known");
        return Vec::new();
    };
    if let (Some(init), ast::Mutability::Let) = (&binding.init, item.r#mut) {
        let bytes = match &init.node {
            ExprNode::LiteralExpr(lit) => encode_const(&init_type, &lit.literal),
            ExprNode::TupleExpr(tuple) if tuple.elements.is_empty() && is_unit_type_ref(&init_type) => Some(Vec::new()),
            _ => None,
        };
        if let Some(bytes) = bytes {
            return vec![IrDecl::GlobalConst(GlobalConst { symbol, bytes, align, externally_visible, export_from_shared_library })];
        }
    }
    vec![IrDecl::GlobalZero(GlobalZero { symbol, size, align, externally_visible, export_from_shared_library })]
}

/// `StaticHasResponsibility`.
fn static_has_responsibility(item: &ast::StaticDecl) -> bool {
    match &item.binding.init {
        None => true,
        Some(init) => !is_place_expr(&Some(init.clone())) || matches!(init.node, ExprNode::MoveExpr(_)),
    }
}

/// `LowerStaticInitItem`: the initializer, the stores into the statics, the panic handling.
fn lower_static_init_item(module_path: &[String], item: &ast::StaticDecl, ctx: &mut LowerCtx) -> IrPtr {
    let Some(init) = &item.binding.init else {
        return empty_ir();
    };
    let mut parts = Vec::new();
    let init_result = lower_expr(init, ctx);
    parts.push(Some(init_result.ir));
    // `PatternBindingValuesInOrder` and `StaticStoreIR`.
    match static_name(&item.binding) {
        Some(name) => {
            let symbol = scoped_sym(&item_path_proc(module_path, &name));
            parts.push(Some(Arc::new(Ir::StoreGlobal { symbol, value: init_result.value.clone() })));
            if static_has_responsibility(item) && !matches!(static_init_type(item, module_path, ctx).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
                ctx.unported("statics whose values need a drop");
            }
        }
        None => ctx.unported("statics that bind several names"),
    }
    let module = module_path.join("::");
    parts.push(Some(init_panic_handle(&module, ctx)));
    seq_ir(parts)
}

/// `LowerStaticInit`.
fn lower_static_init(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> IrPtr {
    let items: Vec<&ast::StaticDecl> = module.items.iter().filter_map(|item| if let ASTItem::StaticDecl(decl) = item { Some(decl) } else { None }).collect();
    if items.is_empty() {
        return empty_ir();
    }
    let parts = items.into_iter().map(|item| Some(lower_static_init_item(module_path, item, ctx))).collect();
    seq_ir(parts)
}

/// `LowerStaticDeinit`: the statics are dropped in the reverse order of their declaration.
fn lower_static_deinit(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> IrPtr {
    let items: Vec<&ast::StaticDecl> = module.items.iter().filter_map(|item| if let ASTItem::StaticDecl(decl) = item { Some(decl) } else { None }).collect();
    if items.is_empty() {
        return empty_ir();
    }
    let mut item_parts = Vec::new();
    for item in items.into_iter().rev() {
        let mut drops = Vec::new();
        if let Some(name) = static_name(&item.binding) {
            if static_has_responsibility(item) {
                let ty = static_init_type(item, module_path, ctx);
                if !matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(_))) {
                    ctx.unported("drops of statics of this type");
                }
                let _ = name;
                drops.push(Some(empty_ir()));
            }
        } else {
            ctx.unported("statics that bind several names");
        }
        item_parts.push(Some(if drops.is_empty() { empty_ir() } else { seq_ir(drops) }));
    }
    seq_ir(item_parts)
}

// --------------------------------------------------------------------- modules

fn module_init_fn(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> Option<ProcIr> {
    ctx.pending = None;
    let saved = ctx.active_static_init_module.replace(module_path.join("::"));
    let body = lower_static_init(module_path, module, ctx);
    ctx.active_static_init_module = saved;
    ctx.pending.is_none().then(|| ProcIr { symbol: init_sym(module_path), params: vec![panic_out_param()], ret: make_type_prim("()"), body: Some(body), ..Default::default() })
}

fn module_deinit_fn(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> Option<ProcIr> {
    ctx.pending = None;
    let body = lower_static_deinit(module_path, module, ctx);
    ctx.pending.is_none().then(|| ProcIr { symbol: deinit_sym(module_path), params: vec![panic_out_param()], ret: make_type_prim("()"), body: Some(body), ..Default::default() })
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
                let symbol = mangle_proc_in_module(module, decl);
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
            ASTItem::StaticDecl(decl) => {
                ctx.pending = None;
                let decls = emit_global(decl, &module.path, ctx);
                match ctx.pending.take() {
                    None => out.decls.extend(decls),
                    Some(what) => out.pending.push((scoped_sym(&module.path), what)),
                }
            }
            // A record, a modal or a class gives declarations only through its methods.
            ASTItem::RecordDecl(decl) => {
                let record_symbol = scoped_sym(&item_path_proc(&module.path, &decl.name));
                ctx.record_ctors.insert(record_symbol);
                for member in &decl.members {
                    let ast::RecordMember::MethodDecl(method) = member else {
                        continue;
                    };
                    ctx.pending = None;
                    let proc = lower_record_method(decl, method, &module.path, ctx);
                    let symbol = scoped_sym(&item_path_proc(&item_path_proc(&module.path, &decl.name), &method.name));
                    if let Some(proc) = &proc {
                        ctx.proc_sigs.insert(symbol.clone(), ProcSig { params: proc.params.clone(), ret: proc.ret.clone() });
                    }
                    match (proc, ctx.pending.take()) {
                        (Some(proc), None) => out.decls.push(IrDecl::Proc(proc)),
                        (_, what) => out.pending.push((symbol, what.unwrap_or_else(|| "record methods".to_string()))),
                    }
                }
            }
            ASTItem::ModalDecl(decl) => {
                let generic = decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty());
                let has_bodies = decl.states.iter().flat_map(|state| &state.members).any(|member| matches!(member, ast::StateMember::StateMethodDecl(_) | ast::StateMember::TransitionDecl(_)));
                if has_bodies && !generic {
                    out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "modal methods and transitions".to_string()));
                }
            }
            ASTItem::ClassDecl(decl) => {
                let has_default_bodies = decl.items.iter().any(|item| matches!(item, ast::ClassItem::ClassMethodDecl(method) if method.body_opt.is_some()));
                if has_default_bodies {
                    out.pending.push((scoped_sym(&item_path_proc(&module.path, &decl.name)), "default methods of classes".to_string()));
                }
            }
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
