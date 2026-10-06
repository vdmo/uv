//! The IR the lowering produces. See `ir_model.h`.

use std::sync::Arc;

use uv_analysis::memory::regions::ProvenanceKind;
use uv_analysis::typing::types::{ParamMode, TypePath, TypeRef};

pub type IrPtr = Arc<Ir>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrImmediateLiteralKind {
    String,
    Bytes,
    Char,
    Int,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrValueKind {
    #[default]
    Opaque,
    Local,
    Symbol,
    Immediate,
}

#[derive(Debug, Clone, Default)]
pub struct IrValue {
    pub kind: IrValueKind,
    pub name: String,
    pub bytes: Vec<u8>,
    pub literal_id: u64,
    pub literal_kind: Option<IrImmediateLiteralKind>,
    pub vtable_sym: String,
}

#[derive(Debug, Clone, Default)]
pub struct IrPlace {
    pub repr: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrRangeKind {
    To,
    ToInclusive,
    #[default]
    Full,
    From,
    Exclusive,
    Inclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrFenceOrder {
    Acquire,
    Release,
    #[default]
    SeqCst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrLiteralKind {
    Int,
    Float,
    String,
    Char,
    Bool,
    Null,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct IrLiteral {
    pub kind: IrLiteralKind,
    pub lexeme: String,
}

pub type IrPatternPtr = Option<Arc<IrPattern>>;

#[derive(Debug, Clone)]
pub struct IrFieldPattern {
    pub name: String,
    pub pattern: IrPatternPtr,
}

#[derive(Debug, Clone)]
pub enum IrEnumPayloadPattern {
    Tuple(Vec<IrPatternPtr>),
    Record(Vec<IrFieldPattern>),
}

#[derive(Debug, Clone)]
pub enum IrPatternNode {
    Literal(IrLiteral),
    Wildcard,
    Identifier { name: String },
    Typed { name: String, ty: TypeRef },
    Tuple { elements: Vec<IrPatternPtr> },
    Record { path: TypePath, fields: Vec<IrFieldPattern> },
    Enum { path: TypePath, name: String, payload: Option<IrEnumPayloadPattern> },
    Modal { state: String, fields: Option<Vec<IrFieldPattern>> },
    Range { kind: IrRangeKind, lo: IrPatternPtr, hi: IrPatternPtr },
    Opaque,
}

#[derive(Debug, Clone)]
pub struct IrPattern {
    pub node: IrPatternNode,
}

#[derive(Debug, Clone, Default)]
pub struct IrRange {
    pub kind: IrRangeKind,
    pub lo: Option<IrValue>,
    pub hi: Option<IrValue>,
}

#[derive(Debug, Clone, Default)]
pub struct IrParam {
    pub mode: Option<ParamMode>,
    pub name: String,
    pub stable_name: String,
    pub ty: TypeRef,
}

#[derive(Debug, Clone, Default)]
pub struct IrIncoming {
    pub label: String,
    pub value: IrValue,
}

#[derive(Debug, Clone, Default)]
pub struct IrIfCaseClause {
    pub pattern: IrPatternPtr,
    pub body: Option<IrPtr>,
    pub cleanup_ir: Option<IrPtr>,
    pub value: IrValue,
    pub value_type: TypeRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrLoopKind {
    #[default]
    Infinite,
    Conditional,
    Iter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrWaitKind {
    #[default]
    Unknown,
    Spawned,
    Tracked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrGpuBarrierKind {
    #[default]
    Full,
    Memory,
    Workgroup,
}

#[derive(Debug, Clone, Default)]
pub struct IrRaceArm {
    pub async_ir: Option<IrPtr>,
    pub async_value: IrValue,
    pub match_value: IrValue,
    pub handler_ir: Option<IrPtr>,
    pub handler_result: IrValue,
}

/// One node of the IR; the kinds and their fields are those of `ir_model.h`.
#[derive(Debug, Clone)]
pub enum Ir {
    Opaque,
    Seq { items: Vec<IrPtr> },
    Call { callee: IrValue, args: Vec<IrValue>, result: IrValue },
    CallVTable { base: IrValue, slot: usize, args: Vec<IrValue>, params: Vec<IrParam>, result: IrValue, ret_type: TypeRef, check_dynamic_receiver_addr_active: bool },
    StoreGlobal { symbol: String, value: IrValue },
    ReadVar { name: String },
    ReadPath { path: Vec<String>, name: String },
    BindVar { name: String, stable_name: String, value: IrValue, ty: TypeRef, prov: ProvenanceKind, prov_region: Option<String>, prov_region_tag: Option<String> },
    StoreVar { name: String, value: IrValue },
    StoreVarNoDrop { name: String, value: IrValue },
    ReadPlace { place: IrPlace },
    WritePlace { place: IrPlace, value: IrValue },
    AddrOf { place: IrPlace, result: IrValue, ref_syms: Vec<String> },
    ReadPtr { ptr: IrValue, result: IrValue },
    WritePtr { ptr: IrValue, value: IrValue },
    UnaryOp { op: String, operand: IrValue, result: IrValue, operand_type: TypeRef, result_type: TypeRef },
    Fence { order: IrFenceOrder, result: IrValue },
    BinaryOp { op: String, lhs: IrValue, rhs: IrValue, result: IrValue },
    Cast { target: TypeRef, value: IrValue, result: IrValue },
    Transmute { from: TypeRef, to: TypeRef, value: IrValue, result: IrValue },
    CheckIndex { base: IrValue, index: IrValue },
    CheckRange { base: IrValue, range: IrRange, range_value: Option<IrValue> },
    CheckSliceLen { base: IrValue, range: IrRange, range_value: Option<IrValue>, value: IrValue },
    CheckOp { op: String, reason: String, lhs: IrValue, rhs: Option<IrValue> },
    CheckCast { target: TypeRef, value: IrValue },
    Alloc { region: Option<IrValue>, value: IrValue, result: IrValue, ty: TypeRef },
    ContextBundleBuild { target_type: TypeRef, root_ctx: IrValue, result: IrValue },
    Return { value: IrValue },
    Result { value: IrValue },
    Break { value: Option<IrValue> },
    Continue,
    Defer,
    MoveState { place: IrPlace },
    If { cond: IrValue, then_ir: Option<IrPtr>, then_value: IrValue, else_ir: Option<IrPtr>, else_value: IrValue, result: IrValue },
    Block { setup: Option<IrPtr>, body: Option<IrPtr>, value: IrValue },
    Loop {
        kind: IrLoopKind,
        pattern: IrPatternPtr,
        iter_ir: Option<IrPtr>,
        iter_value: Option<IrValue>,
        cond_ir: Option<IrPtr>,
        cond_value: Option<IrValue>,
        body_ir: Option<IrPtr>,
        invariant_entry_ir: Option<IrPtr>,
        invariant_backedge_ir: Option<IrPtr>,
        body_value: IrValue,
        result: IrValue,
    },
    IfCase { scrutinee: IrValue, scrutinee_type: TypeRef, arms: Vec<IrIfCaseClause>, result: IrValue },
    Region { owner: IrValue, alias: Option<String>, body: Option<IrPtr>, value: IrValue },
    Frame { region: Option<IrValue>, body: Option<IrPtr>, value: IrValue },
    Branch { cond: Option<IrValue>, true_label: String, false_label: String },
    Phi { ty: TypeRef, incoming: Vec<IrIncoming>, value: IrValue },
    ClearPanic,
    PanicCheck,
    CleanupPanicCheck { cleanup_ir: Option<IrPtr> },
    InitPanicHandle { module: String, poison_modules: Vec<String>, cleanup_ir: Option<IrPtr> },
    InitPanicRaise { module: String, poison_modules: Vec<String>, cleanup_ir: Option<IrPtr> },
    CheckPoison { module: String },
    LowerPanic { reason: String, cleanup_ir: Option<IrPtr> },
    Parallel { domain: IrValue, body: Option<IrPtr>, result: IrValue, cancel_token: Option<IrValue>, name: String },
    Spawn {
        captured_env: Option<IrPtr>,
        body: Option<IrPtr>,
        body_result: IrValue,
        result: IrValue,
        env_ptr: IrValue,
        env_size: IrValue,
        body_fn: IrValue,
        result_size: IrValue,
        runtime_symbol: Option<String>,
        runtime_receiver: Option<IrValue>,
        affinity_mask: Option<IrValue>,
        priority: Option<IrValue>,
        name: String,
    },
    Wait { handle: IrValue, result: IrValue, kind: IrWaitKind },
    CancelCreate { result: IrValue },
    CancelRequest { token: IrValue, result: IrValue },
    CancelWait { token: IrValue, result: IrValue },
    CancelSuppress,
    CancelCheck { token: IrValue, result: IrValue },
    GpuBarrier { kind: IrGpuBarrierKind, result: IrValue },
    Dispatch {
        range: IrValue,
        body: Option<IrPtr>,
        body_result: IrValue,
        captured_env: Option<IrPtr>,
        env_ptr: IrValue,
        body_fn: IrValue,
        elem_size: IrValue,
        result_size: IrValue,
        result_ptr: IrValue,
        reduce_op: Option<String>,
        reduce_fn: Option<IrValue>,
        result: IrValue,
        ordered: bool,
        chunk_size: Option<IrValue>,
        workgroup_size: IrValue,
    },
    Yield { release: bool, value: IrValue, result: IrValue, keys_record: IrValue, state_index: usize },
    YieldFrom { release: bool, source: IrValue, result: IrValue, source_type: TypeRef, state_index: usize },
    SpecSnapshot { paths: Vec<String>, result: IrValue },
    SpecValidate { paths: Vec<String>, result: IrValue },
    SpecCommit { paths: Vec<String>, value: IrValue, result: IrValue },
    SpecRetry { result: IrValue },
    SpecFallback { body: Option<IrPtr>, result: IrValue },
    SpecLoop {
        snapshot_ir: Option<IrPtr>,
        body_ir: Option<IrPtr>,
        validate_ir: Option<IrPtr>,
        commit_ir: Option<IrPtr>,
        retry_ir: Option<IrPtr>,
        fallback_ir: Option<IrPtr>,
        result: IrValue,
    },
    Sync { async_value: IrValue, result: IrValue, async_type: TypeRef, result_type: TypeRef, error_type: TypeRef, runtime_symbol: Option<String>, runtime_receiver: Option<IrValue> },
    RaceReturn { arms: Vec<IrRaceArm>, result: IrValue, result_type: TypeRef },
    RaceYield { arms: Vec<IrRaceArm>, result: IrValue, stream_type: TypeRef },
    All { async_irs: Vec<IrPtr>, async_values: Vec<IrValue>, result: IrValue, tuple_type: TypeRef, error_types: Vec<TypeRef> },
    AsyncComplete { value: IrValue, result: IrValue, async_type: TypeRef, result_type: TypeRef },
    AsyncFail { value: IrValue, result: IrValue, async_type: TypeRef, error_type: TypeRef },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IrInlineMode {
    #[default]
    Default,
    Always,
    Never,
}

#[derive(Debug, Clone, Default)]
pub struct IrAggregateCopyElision {
    pub return_local_uses_sret: bool,
    pub return_local: String,
    pub return_local_stable_name: String,
    pub source_param: String,
    pub source_param_stable_name: String,
    pub source_param_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ProcIr {
    pub symbol: String,
    pub defining_module_path: Vec<String>,
    pub params: Vec<IrParam>,
    pub ret: TypeRef,
    pub body: Option<IrPtr>,
    pub abi: Option<String>,
    pub inline_mode: IrInlineMode,
    pub cold: bool,
    pub aggregate_copy_elision: Option<IrAggregateCopyElision>,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalConst {
    pub symbol: String,
    pub bytes: Vec<u8>,
    pub align: u64,
    pub externally_visible: bool,
    pub export_from_shared_library: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalZero {
    pub symbol: String,
    pub size: u64,
    pub align: u64,
    pub externally_visible: bool,
    pub export_from_shared_library: bool,
}

#[derive(Debug, Clone, Default)]
pub struct VTableHeader {
    pub size: u64,
    pub align: u64,
    pub drop_sym: String,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalVTable {
    pub symbol: String,
    pub header: VTableHeader,
    pub slots: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExternProcIr {
    pub symbol: String,
    pub params: Vec<IrParam>,
    pub ret: TypeRef,
    pub abi: Option<String>,
    pub raw_dylib_library_name: Option<String>,
    pub raw_dylib_foreign_symbol: Option<String>,
    pub raw_dylib_catch_unwind: bool,
}

#[derive(Debug, Clone)]
pub enum IrDecl {
    Proc(ProcIr),
    GlobalConst(GlobalConst),
    GlobalZero(GlobalZero),
    GlobalVTable(GlobalVTable),
    ExternProc(ExternProcIr),
}

pub type IrDecls = Vec<IrDecl>;

/// A sequence node; flattening rules of `SeqIR` are in `seq_ir*`.
pub fn seq_ir(items: Vec<IrPtr>) -> IrPtr {
    Arc::new(Ir::Seq { items })
}
