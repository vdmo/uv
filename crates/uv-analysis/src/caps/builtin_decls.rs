//! Declarations of the built-in types and classes, written as the syntax trees a
//! prelude module would parse to. None of them has a source location.

use std::sync::Arc;

use uv_source::ast::*;
use uv_source::lexer::token::{Token, TokenKind};
use uv_source::parser::make_type_modal_ref;

fn ty(node: TypeNode) -> TypePtr {
    Some(Arc::new(Type { span: Default::default(), node }))
}

fn names(comps: &[&str]) -> Vec<String> {
    comps.iter().map(|comp| comp.to_string()).collect()
}

pub(crate) fn prim(name: &str) -> TypePtr {
    ty(TypeNode::TypePrim(TypePrim { name: name.to_string() }))
}

pub(crate) fn path(comps: &[&str]) -> TypePtr {
    apply(comps, Vec::new())
}

fn apply(comps: &[&str], generic_args: Vec<TypePtr>) -> TypePtr {
    ty(TypeNode::TypePathType(TypePathType { path: names(comps), generic_args }))
}

pub(crate) fn dynamic(comps: &[&str]) -> TypePtr {
    ty(TypeNode::TypeDynamic(TypeDynamic { path: names(comps) }))
}

fn string(state: Option<StringState>) -> TypePtr {
    ty(TypeNode::TypeString(TypeString { state }))
}

fn string_view() -> TypePtr {
    string(Some(StringState::View))
}

fn string_managed() -> TypePtr {
    string(Some(StringState::Managed))
}

pub(crate) fn bytes(state: BytesState) -> TypePtr {
    ty(TypeNode::TypeBytes(TypeBytes { state: Some(state) }))
}

pub(crate) fn perm(perm: TypePerm, base: TypePtr) -> TypePtr {
    ty(TypeNode::TypePermType(TypePermType { perm, base }))
}

pub(crate) fn union(types: Vec<TypePtr>) -> TypePtr {
    ty(TypeNode::TypeUnion(TypeUnion { types }))
}

fn slice(element: TypePtr) -> TypePtr {
    ty(TypeNode::TypeSlice(TypeSlice { element }))
}

fn modal_state(comps: &[&str], state: &str, generic_args: Vec<TypePtr>) -> TypePtr {
    let path = names(comps);
    let modal_ref = make_type_modal_ref(path.clone(), generic_args.clone());
    ty(TypeNode::TypeModalState(TypeModalState { modal_ref, path, generic_args, state: state.to_string() }))
}

fn outcome(value: TypePtr, error: TypePtr) -> TypePtr {
    apply(&["Outcome"], vec![value, error])
}

pub(crate) fn param(name: &str, r#type: TypePtr) -> Param {
    Param { name: name.to_string(), r#type, ..Default::default() }
}

fn field_with(name: &str, r#type: TypePtr, vis: Visibility, init_opt: ExprPtr) -> RecordMember {
    RecordMember::FieldDecl(FieldDecl { vis, name: name.to_string(), r#type, init_opt, ..Default::default() })
}

fn field(name: &str, r#type: TypePtr) -> RecordMember {
    field_with(name, r#type, Visibility::Public, None)
}

fn literal(kind: TokenKind, lexeme: &str) -> ExprPtr {
    let literal = Token { kind, lexeme: lexeme.to_string(), span: Default::default() };
    Some(Arc::new(Expr { span: Default::default(), node: ExprNode::LiteralExpr(LiteralExpr { literal }) }))
}

fn empty_block() -> BlockPtr {
    Some(Arc::new(Block::default()))
}

fn type_param(name: &str, default_type: TypePtr) -> TypeParam {
    TypeParam { name: name.to_string(), default_type, ..Default::default() }
}

fn generic_params(params: Vec<TypeParam>) -> Option<GenericParams> {
    Some(GenericParams { params, span: Default::default() })
}

fn shorthand(perm: ReceiverPerm) -> Receiver {
    Receiver::ReceiverShorthand(ReceiverShorthand { perm, mode_opt: None })
}

pub(crate) fn class_method(
    name: &str,
    generic_params: Option<GenericParams>,
    perm: ReceiverPerm,
    params: Vec<Param>,
    ret: TypePtr,
) -> ClassItem {
    ClassItem::ClassMethodDecl(ClassMethodDecl {
        vis: Visibility::Public,
        name: name.to_string(),
        generic_params,
        receiver: shorthand(perm),
        params,
        return_type_opt: ret,
        ..Default::default()
    })
}

pub(crate) fn class(name: &str, generic_params: Option<GenericParams>, items: Vec<ClassItem>) -> ClassDecl {
    ClassDecl { vis: Visibility::Public, name: name.to_string(), generic_params, items, ..Default::default() }
}

fn record(name: &str, implements: &[&str], members: Vec<RecordMember>) -> RecordDecl {
    RecordDecl {
        vis: Visibility::Public,
        name: name.to_string(),
        implements: implements.iter().map(|class| names(&[class])).collect(),
        members,
        ..Default::default()
    }
}

fn variant(name: &str, payload: Option<TypePtr>) -> VariantDecl {
    let payload_opt =
        payload.map(|element| VariantPayload::VariantPayloadTuple(VariantPayloadTuple { elements: vec![element] }));
    VariantDecl { name: name.to_string(), payload_opt, ..Default::default() }
}

fn unit_enum(name: &str, implements: &[&str], variants: &[&str]) -> EnumDecl {
    EnumDecl {
        vis: Visibility::Public,
        name: name.to_string(),
        implements: implements.iter().map(|class| names(&[class])).collect(),
        variants: variants.iter().map(|variant_name| variant(variant_name, None)).collect(),
        ..Default::default()
    }
}

fn alias(name: &str, generic_params: Option<GenericParams>, r#type: TypePtr) -> TypeAliasDecl {
    TypeAliasDecl { vis: Visibility::Public, name: name.to_string(), generic_params, r#type, ..Default::default() }
}

fn state_field(name: &str, r#type: TypePtr, vis: Visibility) -> StateMember {
    StateMember::StateFieldDecl(StateFieldDecl { vis, name: name.to_string(), r#type, ..Default::default() })
}

fn state_method(name: &str, perm: ReceiverPerm, params: Vec<Param>, ret: TypePtr) -> StateMember {
    StateMember::StateMethodDecl(StateMethodDecl {
        vis: Visibility::Public,
        name: name.to_string(),
        receiver: shorthand(perm),
        params,
        return_type_opt: ret,
        body: empty_block(),
        ..Default::default()
    })
}

fn transition(name: &str, target_state: &str) -> StateMember {
    StateMember::TransitionDecl(TransitionDecl {
        vis: Visibility::Public,
        name: name.to_string(),
        target_state: target_state.to_string(),
        body: empty_block(),
        ..Default::default()
    })
}

fn state(name: &str, members: Vec<StateMember>) -> StateBlock {
    StateBlock { name: name.to_string(), members, ..Default::default() }
}

fn modal(name: &str, generic_params: Option<GenericParams>, states: Vec<StateBlock>) -> ModalDecl {
    ModalDecl { vis: Visibility::Public, name: name.to_string(), generic_params, states, ..Default::default() }
}

// --- system ---

pub fn build_context_record_decl() -> RecordDecl {
    record(
        "Context",
        &["Bitcopy"],
        vec![
            field("io", dynamic(&["IO"])),
            field("net", dynamic(&["Network"])),
            field("heap", dynamic(&["HeapAllocator"])),
            field("sys", dynamic(&["System"])),
            field("reactor", dynamic(&["Reactor"])),
            field("time", dynamic(&["Time"])),
        ],
    )
}

pub fn build_test_authority_record_decl() -> RecordDecl {
    record(
        "TestAuthority",
        &[],
        vec![
            field("io", dynamic(&["IO"])),
            field("sys", dynamic(&["System"])),
            field("heap", dynamic(&["HeapAllocator"])),
            field("time", dynamic(&["Time"])),
            field("temporary_directory", string_view()),
            field("target_profile", string_view()),
            field("compiler_executable_path", string_view()),
            field("compiler_current_directory", string_view()),
        ],
    )
}

pub fn build_panic_record_decl() -> RecordDecl {
    record("PanicRecord", &["Bitcopy"], vec![field("panic", prim("bool")), field("code", prim("u32"))])
}

pub fn build_region_options_record_decl() -> RecordDecl {
    record(
        "RegionOptions",
        &[],
        vec![
            field_with("stack_size", prim("usize"), Visibility::Public, literal(TokenKind::IntLiteral, "0usize")),
            field_with("name", string(None), Visibility::Public, literal(TokenKind::StringLiteral, "\"\"")),
        ],
    )
}

pub fn build_system_class_decl() -> ClassDecl {
    let method = |name: &str, params: Vec<Param>, ret: TypePtr| class_method(name, None, ReceiverPerm::Const, params, ret);
    class(
        "System",
        None,
        vec![
            method("exit", vec![param("code", prim("i32"))], prim("!")),
            method("get_env", vec![param("key", string_view())], string_view()),
            method("executable_path", vec![], string_view()),
            method("argument_count", vec![], prim("usize")),
            method("argument", vec![param("index", prim("usize"))], string_view()),
            method("current_directory", vec![], string_view()),
            method("run", vec![param("command", string_view())], prim("i32")),
        ],
    )
}

pub fn build_cpu_set_alias_decl() -> TypeAliasDecl {
    alias("CpuSet", None, prim("u64"))
}

pub fn build_priority_enum_decl() -> EnumDecl {
    unit_enum("Priority", &["Bitcopy"], &["Low", "Normal", "High"])
}

// --- time ---

pub fn build_time_error_enum_decl() -> EnumDecl {
    unit_enum(
        "TimeError",
        &[],
        &["Unsupported", "ClockUnavailable", "OutOfRange", "InvalidResolution", "ClockMismatch"],
    )
}

pub fn build_duration_record_decl() -> RecordDecl {
    record("Duration", &[], vec![field("nanoseconds", prim("u128"))])
}

pub fn build_monotonic_instant_record_decl() -> RecordDecl {
    record(
        "MonotonicInstant",
        &[],
        vec![
            field_with("domain", prim("usize"), Visibility::Private, None),
            field_with("ticks", prim("u128"), Visibility::Private, None),
        ],
    )
}

pub fn build_utc_instant_record_decl() -> RecordDecl {
    record("UtcInstant", &[], vec![field("unix_nanoseconds", prim("i128"))])
}

// --- heap ---

pub fn build_allocation_error_enum_decl() -> EnumDecl {
    EnumDecl {
        vis: Visibility::Public,
        name: "AllocationError".to_string(),
        variants: vec![variant("OutOfMemory", Some(prim("usize"))), variant("QuotaExceeded", Some(prim("usize")))],
        ..Default::default()
    }
}

// --- compile-time reflection ---

pub fn build_source_span_record_decl() -> RecordDecl {
    record(
        "SourceSpan",
        &[],
        vec![
            field("file", string_managed()),
            field("start_line", prim("usize")),
            field("start_col", prim("usize")),
            field("end_line", prim("usize")),
            field("end_col", prim("usize")),
        ],
    )
}

pub fn build_type_category_enum_decl() -> EnumDecl {
    unit_enum(
        "TypeCategory",
        &["Bitcopy"],
        &[
            "Record", "Enum", "Modal", "Primitive", "Tuple", "Array", "Slice", "Union", "Procedure", "Reference",
            "Dynamic", "Opaque", "Generic", "String", "Bytes", "Range",
        ],
    )
}

pub fn build_field_info_record_decl() -> RecordDecl {
    record(
        "FieldInfo",
        &[],
        vec![
            field("name", string_managed()),
            field("type", path(&["Type"])),
            field("visibility", string_managed()),
            field("index", prim("usize")),
            field("span", path(&["SourceSpan"])),
        ],
    )
}

pub fn build_variant_info_record_decl() -> RecordDecl {
    record(
        "VariantInfo",
        &[],
        vec![
            field("name", string_managed()),
            field("payload_kind", string_managed()),
            field("payload_types", slice(path(&["Type"]))),
            field("field_names", slice(string_managed())),
            field("span", path(&["SourceSpan"])),
        ],
    )
}

pub fn build_state_info_record_decl() -> RecordDecl {
    record(
        "StateInfo",
        &[],
        vec![
            field("name", string_managed()),
            field("field_names", slice(string_managed())),
            field("method_names", slice(string_managed())),
            field("transition_names", slice(string_managed())),
            field("span", path(&["SourceSpan"])),
        ],
    )
}

// --- regions ---

/// `alloc<T>(value: T) -> T`, whose body returns its argument.
fn region_alloc_method() -> StateMember {
    let value = Expr {
        span: Default::default(),
        node: ExprNode::IdentifierExpr(IdentifierExpr { name: "value".to_string(), from_splice: false }),
    };
    let ret = Stmt::ReturnStmt(ReturnStmt { value_opt: Some(Arc::new(value)), span: Default::default() });
    StateMember::StateMethodDecl(StateMethodDecl {
        vis: Visibility::Public,
        name: "alloc".to_string(),
        generic_params: generic_params(vec![type_param("T", None)]),
        receiver: shorthand(ReceiverPerm::Unique),
        params: vec![param("value", path(&["T"]))],
        return_type_opt: path(&["T"]),
        body: Some(Arc::new(Block { stmts: vec![ret], ..Default::default() })),
        ..Default::default()
    })
}

pub fn build_region_modal_decl() -> ModalDecl {
    let handle = || state_field("handle", prim("usize"), Visibility::Public);
    modal(
        "Region",
        None,
        vec![
            state(
                "Active",
                vec![
                    handle(),
                    region_alloc_method(),
                    transition("reset_unchecked", "Active"),
                    transition("freeze", "Frozen"),
                    transition("free_unchecked", "Freed"),
                ],
            ),
            state("Frozen", vec![handle(), transition("thaw", "Active"), transition("free_unchecked", "Freed")]),
            state("Freed", vec![handle()]),
        ],
    )
}

// --- outcome ---

pub fn build_outcome_enum_decl() -> EnumDecl {
    EnumDecl {
        vis: Visibility::Public,
        name: "Outcome".to_string(),
        generic_params: generic_params(vec![type_param("TValue", None), type_param("TError", None)]),
        variants: vec![variant("Value", Some(path(&["TValue"]))), variant("Error", Some(path(&["TError"])))],
        ..Default::default()
    }
}

// --- I/O ---

fn io_outcome(value: TypePtr) -> TypePtr {
    outcome(value, path(&["IoError"]))
}

pub fn build_file_modal_decl() -> ModalDecl {
    let handle = || state_field("handle", prim("usize"), Visibility::Public);
    let method = |name: &str, params: Vec<Param>, ret: TypePtr| state_method(name, ReceiverPerm::Const, params, ret);
    let writer = || {
        vec![
            handle(),
            method("write", vec![param("data", bytes(BytesState::View))], io_outcome(prim("()"))),
            method("flush", vec![], io_outcome(prim("()"))),
            transition("close", "Closed"),
        ]
    };
    modal(
        "File",
        None,
        vec![
            state(
                "Read",
                vec![
                    handle(),
                    method("read_all", vec![], io_outcome(perm(TypePerm::Unique, string_managed()))),
                    method("read_all_bytes", vec![], io_outcome(perm(TypePerm::Unique, bytes(BytesState::Managed)))),
                    transition("close", "Closed"),
                ],
            ),
            state("Write", writer()),
            state("Append", writer()),
            state("Closed", vec![]),
        ],
    )
}

pub fn build_dir_iter_modal_decl() -> ModalDecl {
    modal(
        "DirIter",
        None,
        vec![
            state(
                "Open",
                vec![
                    state_field("handle", prim("usize"), Visibility::Public),
                    state_method(
                        "next",
                        ReceiverPerm::Const,
                        vec![],
                        io_outcome(union(vec![path(&["DirEntry"]), prim("()")])),
                    ),
                    transition("close", "Closed"),
                ],
            ),
            state("Closed", vec![]),
        ],
    )
}

pub fn build_dir_entry_record_decl() -> RecordDecl {
    record(
        "DirEntry",
        &[],
        vec![field("name", string_managed()), field("path", string_managed()), field("kind", path(&["FileKind"]))],
    )
}

pub fn build_file_kind_enum_decl() -> EnumDecl {
    unit_enum("FileKind", &["Bitcopy"], &["File", "Dir", "Other"])
}

pub fn build_io_error_enum_decl() -> EnumDecl {
    unit_enum(
        "IoError",
        &["Bitcopy"],
        &["NotFound", "PermissionDenied", "AlreadyExists", "InvalidPath", "Busy", "IoFailure", "DirectoryNotEmpty"],
    )
}

// --- concurrency ---

pub fn build_spawned_modal_decl() -> ModalDecl {
    modal(
        "Spawned",
        generic_params(vec![type_param("T", None)]),
        vec![state("Pending", vec![]), state("Ready", vec![state_field("value", path(&["T"]), Visibility::Public)])],
    )
}

pub fn build_cancel_token_modal_decl() -> ModalDecl {
    let method = |name: &str, ret: TypePtr| state_method(name, ReceiverPerm::Const, vec![], ret);
    modal(
        "CancelToken",
        None,
        vec![state(
            "Active",
            vec![
                state_field("id", prim("usize"), Visibility::Private),
                method("cancel", prim("()")),
                method("is_cancelled", prim("bool")),
                method("wait_cancelled", apply(&["Async"], vec![prim("()")])),
                method("child", modal_state(&["CancelToken"], "Active", vec![])),
            ],
        )],
    )
}

pub fn build_tracked_modal_decl() -> ModalDecl {
    modal(
        "Tracked",
        generic_params(vec![type_param("T", None), type_param("E", None)]),
        vec![
            state("Pending", vec![]),
            state("Ready", vec![state_field("value", union(vec![path(&["T"]), path(&["E"])]), Visibility::Public)]),
        ],
    )
}

pub fn build_async_modal_decl() -> ModalDecl {
    let unique_async_state = |state_name: &str| {
        let generic_args = vec![path(&["TOut"]), path(&["TIn"]), path(&["TResult"]), path(&["TError"])];
        perm(TypePerm::Unique, modal_state(&["Async"], state_name, generic_args))
    };
    let resume = state_method(
        "resume",
        ReceiverPerm::Unique,
        vec![param("input", path(&["TIn"]))],
        union(vec![unique_async_state("Suspended"), unique_async_state("Completed"), unique_async_state("Failed")]),
    );
    modal(
        "Async",
        generic_params(vec![
            type_param("TOut", None),
            type_param("TIn", prim("()")),
            type_param("TResult", prim("()")),
            type_param("TError", prim("!")),
        ]),
        vec![
            state("Suspended", vec![state_field("output", path(&["TOut"]), Visibility::Public), resume]),
            state("Completed", vec![state_field("value", path(&["TResult"]), Visibility::Public)]),
            state("Failed", vec![state_field("error", path(&["TError"]), Visibility::Public)]),
        ],
    )
}

pub fn build_async_class_decl() -> ClassDecl {
    let params = ["TOut", "TIn", "TResult", "TError"].iter().map(|name| type_param(name, None)).collect();
    class("Async", generic_params(params), vec![])
}

fn async_alias(name: &str, params: Vec<TypeParam>, args: [TypePtr; 4]) -> TypeAliasDecl {
    alias(name, generic_params(params), apply(&["Async"], args.to_vec()))
}

pub fn build_sequence_alias_decl() -> TypeAliasDecl {
    async_alias("Sequence", vec![type_param("T", None)], [path(&["T"]), prim("()"), prim("()"), prim("!")])
}

pub fn build_future_alias_decl() -> TypeAliasDecl {
    async_alias(
        "Future",
        vec![type_param("TResult", None), type_param("TError", prim("!"))],
        [prim("()"), prim("()"), path(&["TResult"]), path(&["TError"])],
    )
}

pub fn build_stream_alias_decl() -> TypeAliasDecl {
    async_alias(
        "Stream",
        vec![type_param("TValue", None), type_param("TError", None)],
        [path(&["TValue"]), prim("()"), prim("()"), path(&["TError"])],
    )
}

pub fn build_pipe_alias_decl() -> TypeAliasDecl {
    async_alias(
        "Pipe",
        vec![type_param("TIn", None), type_param("TOut", None)],
        [path(&["TOut"]), path(&["TIn"]), prim("()"), prim("!")],
    )
}

pub fn build_exchange_alias_decl() -> TypeAliasDecl {
    async_alias(
        "Exchange",
        vec![type_param("TValue", None)],
        [path(&["TValue"]), path(&["TValue"]), path(&["TValue"]), prim("!")],
    )
}

pub fn build_execution_domain_class_decl() -> ClassDecl {
    class(
        "ExecutionDomain",
        None,
        vec![
            class_method("name", None, ReceiverPerm::Const, vec![], string_view()),
            class_method("max_concurrency", None, ReceiverPerm::Const, vec![], prim("usize")),
        ],
    )
}

pub fn build_reactor_class_decl() -> ClassDecl {
    let generics = || generic_params(vec![type_param("T", None), type_param("E", None)]);
    let args = || vec![path(&["T"]), path(&["E"])];
    let future = || vec![param("future", apply(&["Future"], args()))];
    class(
        "Reactor",
        None,
        vec![
            class_method("run", generics(), ReceiverPerm::Const, future(), apply(&["Outcome"], args())),
            class_method("register", generics(), ReceiverPerm::Const, future(), apply(&["Tracked"], args())),
        ],
    )
}
