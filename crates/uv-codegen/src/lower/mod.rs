//! Lowering of procedures to IR: the context, literals, blocks, `return`, procedures and
//! modules. A construct that is not ported yet is recorded in `LowerCtx::pending` and
//! the declaration that contains it is not produced. See `05_codegen/lower`.

use std::collections::HashMap;
use std::sync::Arc;

use uv_analysis::keys::key_paths::KeyPath;
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
use uv_core::symbols::{mangle, string_of_path};


mod call;
mod cleanup;
mod expr;
mod keys;
mod module;
mod place;
mod proc;
mod statics;
mod stmt;

use self::{call::*, cleanup::*, expr::*, keys::*, place::*, proc::*, statics::*, stmt::*};

pub use self::module::{lower_module, LoweredModule};

pub const PANIC_OUT_NAME: &str = "__panic";

/// What lowering a block or an expression gives: the IR and the value it produced.
pub struct LowerResult {
    pub ir: IrPtr,
    pub value: IrValue,
}

#[derive(Default, Clone)]
struct ScopeInfo {
    /// The names bound in the scope, in order.
    variables: Vec<String>,
    /// What leaving the scope must release.
    cleanup_items: Vec<CleanupItem>,
    /// The number the runtime knows the scope by.
    runtime_scope_id: u64,
}

/// What leaving a scope owes: a binding that holds responsibility is dropped, a held key
/// scope is released, a key released for a block is taken again.
#[derive(Clone)]
enum CleanupItem {
    DropBinding { name: String, binding_id: u64 },
    ReleaseKeyScope(String),
    ReacquireReleasedKey(String),
}

/// A key held in a key scope: where it is taken, how it is written, and for what (1 is write).
#[derive(Clone)]
struct AcquiredKey {
    path: KeyPath,
    encoded: String,
    mode: u8,
}

#[derive(Clone)]
struct ActiveKeyScope {
    scope_runtime_id: u64,
    scope_name: String,
    acquired: Vec<AcquiredKey>,
}

/// The ordering memory accesses are given inside a key block.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AccessOrdering {
    Relaxed,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
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
    /// Fields moved out of the binding, which are not dropped with it.
    moved_fields: Vec<String>,
    prov: ProvenanceKind,
    prov_region: Option<String>,
    prov_region_tag: Option<String>,
}

/// One step of a cleanup plan.
#[allow(dead_code)] // the fields skipped matter to drops of records, which are not ported
enum CleanupAction {
    DropVar { ty: TypeRef, skip_fields: Vec<String> },
    DropTemp { ty: TypeRef },
    ReleaseKeyScope(String),
    ReacquireReleasedKey(String),
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
    Index,
    AddrTuple,
    AddrIndex,
    LoadFromAddr,
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
    pub index: IrValue,
    pub fields: Vec<(String, IrValue)>,
    pub elements: Vec<IrValue>,
    pub array_segments: Vec<DerivedArraySegment>,
}

impl DerivedValueInfo {
    fn new(kind: DerivedKind) -> Self {
        DerivedValueInfo { kind, base: IrValue::default(), field: String::new(), tuple_index: 0, index: IrValue::default(), fields: Vec::new(), elements: Vec::new(), array_segments: Vec::new() }
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
    next_runtime_scope_id: u64,
    active_key_scopes: Vec<ActiveKeyScope>,
    implicit_key_scope_names: HashMap<u64, String>,
    current_access_order: Option<AccessOrdering>,
    /// Whether the procedure being lowered checks its contracts and refinements as it runs.
    dynamic_checks: bool,
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
    /// The procedures exported with a C-callable signature, and whether each catches unwinding.
    export_unwind_modes: HashMap<String, bool>,
    /// The foreign procedures declared so far.
    ffi_imports: std::collections::HashSet<String>,
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
            next_runtime_scope_id: 1,
            active_key_scopes: Vec::new(),
            implicit_key_scope_names: HashMap::new(),
            current_access_order: None,
            dynamic_checks: false,
            binding_states: HashMap::new(),
            temp_depth: 0,
            suppress_temp_at_depth: None,
            temp_sink: None,
            value_types: HashMap::new(),
            derived_values: HashMap::new(),
            expr_prov: None,
            proc_sigs: HashMap::new(),
            record_ctors: std::collections::HashSet::new(),
            export_unwind_modes: HashMap::new(),
            ffi_imports: std::collections::HashSet::new(),
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
        let runtime_scope_id = self.next_runtime_scope_id;
        self.next_runtime_scope_id += 1;
        self.scope_stack.push(ScopeInfo { runtime_scope_id, ..Default::default() });
    }

    /// `CurrentRuntimeScopeId`.
    fn current_runtime_scope_id(&self) -> Option<u64> {
        self.scope_stack.last().map(|scope| scope.runtime_scope_id)
    }

    /// `RegisterKeyScopeExit`.
    fn register_key_scope_exit(&mut self, scope_name: &str) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.cleanup_items.push(CleanupItem::ReleaseKeyScope(scope_name.to_string()));
        }
    }

    /// `RegisterReleasedKeyReacquire`.
    fn register_released_key_reacquire(&mut self, handle_name: &str) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.cleanup_items.push(CleanupItem::ReacquireReleasedKey(handle_name.to_string()));
        }
    }

    fn pop_scope(&mut self) {
        let Some(scope) = self.scope_stack.pop() else {
            return;
        };
        self.active_key_scopes.retain(|active| active.scope_runtime_id != scope.runtime_scope_id);
        self.implicit_key_scope_names.remove(&scope.runtime_scope_id);
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
                scope.cleanup_items.push(CleanupItem::DropBinding { name: name.to_string(), binding_id });
            }
        }
        self.binding_states.entry(name.to_string()).or_default().push(BindingState { ty, stable_name: stable_name.clone(), binding_id, has_responsibility, is_moved: false, moved_fields: Vec::new(), prov, prov_region, prov_region_tag });
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

/// `NormalizeAttrLiteral`: a written literal without its quotes.
fn normalize_attr_literal(text: &str) -> String {
    let quoted = text.len() >= 2 && ((text.starts_with('"') && text.ends_with('"')) || (text.starts_with('\'') && text.ends_with('\'')));
    if quoted { text[1..text.len() - 1].to_string() } else { text.to_string() }
}

fn has_attr(attrs: &[ast::AttributeItem], name: &str) -> bool {
    attrs.iter().any(|attr| attr.name.full_name == name)
}

/// `GetAttributeValue`: the token of the first argument of an attribute, as written.
fn attr_value(attrs: &[ast::AttributeItem], name: &str) -> Option<uv_source::lexer::token::Token> {
    attrs.iter().filter(|attr| attr.name.full_name == name).find_map(|attr| {
        attr.args.iter().find_map(|arg| match (&arg.key, &arg.value) {
            (None, ast::AttributeArgValue::Token(token)) => Some(token.clone()),
            _ => None,
        })
    })
}

/// `LinkName`: the symbol a `mangle` attribute asks for.
fn link_name(attrs: &[ast::AttributeItem], raw_name: &str) -> Option<String> {
    let attr = attrs.iter().find(|attr| attr.name.full_name == "mangle")?;
    let mode_arg = attr.args.iter().find(|arg| arg.key.as_deref().is_none_or(|key| key == "mode"))?;
    let ast::AttributeArgValue::Token(token) = &mode_arg.value else {
        return None;
    };
    let mode = normalize_attr_literal(&token.lexeme);
    if mode.is_empty() {
        return None;
    }
    if mode == "none" && token.kind != TokenKind::StringLiteral {
        return Some(raw_name.to_string());
    }
    (token.kind == TokenKind::StringLiteral).then_some(mode)
}

/// `ExportUnwindModeFor`: whether an exported procedure catches what unwinds out of it.
fn export_unwind_catches(attrs: &[ast::AttributeItem]) -> bool {
    let Some(attr) = attrs.iter().find(|attr| attr.name.full_name == "unwind") else {
        return false;
    };
    let token = attr.args.first().and_then(|arg| match &arg.value {
        ast::AttributeArgValue::Token(token) => Some(token),
        ast::AttributeArgValue::AttributeArgList(_) => None,
    });
    token.is_some_and(|token| normalize_attr_literal(&token.lexeme) == "catch")
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

/// What lowering a copy of the context does to the original: the names it bound and the
/// counter of bindings are dropped, what it learned of value types stays.
struct BranchSnapshot {
    scope_stack: Vec<ScopeInfo>,
    active_key_scopes: Vec<ActiveKeyScope>,
    implicit_key_scope_names: HashMap<u64, String>,
    current_access_order: Option<AccessOrdering>,
    binding_states: HashMap<String, Vec<BindingState>>,
    next_binding_id: u64,
    temp_depth: u32,
    suppress_temp_at_depth: Option<u32>,
}

impl LowerCtx<'_, '_> {
    fn begin_branch(&self) -> BranchSnapshot {
        BranchSnapshot { scope_stack: self.scope_stack.clone(), active_key_scopes: self.active_key_scopes.clone(), implicit_key_scope_names: self.implicit_key_scope_names.clone(), current_access_order: self.current_access_order, binding_states: self.binding_states.clone(), next_binding_id: self.next_binding_id, temp_depth: self.temp_depth, suppress_temp_at_depth: self.suppress_temp_at_depth }
    }

    fn end_branch(&mut self, snapshot: BranchSnapshot) {
        self.scope_stack = snapshot.scope_stack;
        self.active_key_scopes = snapshot.active_key_scopes;
        self.implicit_key_scope_names = snapshot.implicit_key_scope_names;
        self.current_access_order = snapshot.current_access_order;
        self.binding_states = snapshot.binding_states;
        self.next_binding_id = snapshot.next_binding_id;
        self.temp_depth = snapshot.temp_depth;
        self.suppress_temp_at_depth = snapshot.suppress_temp_at_depth;
    }
}
