//! The compile-time pass over a whole project: modules are expanded in the order given,
//! each seeing the modules expanded before it.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uv_core::diagnostics::{has_error, DiagnosticStream};
use uv_core::process_config::is_debug_enabled;
use uv_core::spec_rule;
use uv_core::symbols::string_of_path;
use uv_source::ast::*;
use uv_source::module_paths::ModuleNames;

use crate::files::capture_project_file_snapshot;
use crate::rewrite::expand_module_items;
use crate::value::{ct_empty_env, has_attribute};

#[derive(Default)]
pub struct ComptimeResult {
    pub modules: Option<Vec<ASTModule>>,
    pub diags: DiagnosticStream,
}

#[derive(Default, Clone)]
pub struct ComptimePassOptions {
    pub project_root: String,
    pub source_roots_by_assembly: HashMap<String, String>,
    pub fallback_source_root: Option<String>,
}

fn log_comptime_progress(message: impl FnOnce() -> String) {
    if is_debug_enabled("pipeline") || is_debug_enabled("phases") {
        eprintln!("[uv] comptime {}", message());
    }
}

// ---- does anything in the project ask for project files? ---------------------
//
// The file snapshot is captured only when some compile-time code could use it. The walk
// below mirrors the reference exactly, including the constructs it does not descend into.

fn has_files(attr_list: &[AttributeItem]) -> bool {
    has_attribute(attr_list, "files")
}

fn has_emit(attr_list: &[AttributeItem]) -> bool {
    has_attribute(attr_list, "emit")
}

fn has_derive(attr_list: &[AttributeItem]) -> bool {
    has_attribute(attr_list, "derive")
}

fn types_need(types: &[TypePtr]) -> bool {
    types.iter().any(type_needs)
}

fn exprs_need(exprs: &[ExprPtr]) -> bool {
    exprs.iter().any(expr_needs)
}

fn args_need(args: &[Arg]) -> bool {
    args.iter().any(|arg| expr_needs(&arg.value))
}

fn field_inits_need(fields: &[FieldInit]) -> bool {
    fields.iter().any(|field| expr_needs(&field.value))
}

fn field_patterns_need(fields: &[FieldPattern]) -> bool {
    fields.iter().any(|field| pattern_needs(&field.pattern_opt))
}

fn key_path_needs(path: &KeyPathExpr) -> bool {
    path.segs.iter().any(|seg| matches!(seg, KeySeg::KeySegIndex(index) if expr_needs(&index.expr)))
}

fn invariant_needs(invariant: &Option<LoopInvariant>) -> bool {
    invariant.as_ref().is_some_and(|invariant| expr_needs(&invariant.predicate))
}

fn type_invariant_needs(invariant: &Option<TypeInvariant>) -> bool {
    invariant.as_ref().is_some_and(|invariant| expr_needs(&invariant.predicate))
}

fn contract_needs(contract: &Option<ContractClause>) -> bool {
    contract
        .as_ref()
        .is_some_and(|contract| expr_needs(&contract.precondition) || expr_needs(&contract.postcondition))
}

fn generics_need(params: &Option<GenericParams>) -> bool {
    params.as_ref().is_some_and(|params| params.params.iter().any(|param| type_needs(&param.default_type)))
}

fn params_need(params: &[Param]) -> bool {
    params.iter().any(|param| type_needs(&param.r#type))
}

fn receiver_needs(receiver: &Receiver) -> bool {
    matches!(receiver, Receiver::ReceiverExplicit(explicit) if type_needs(&explicit.r#type))
}

fn type_needs(ty: &TypePtr) -> bool {
    let Some(ty) = ty else {
        return false;
    };
    match &ty.node {
        TypeNode::TypePermType(n) => type_needs(&n.base),
        TypeNode::TypeUnion(n) => types_need(&n.types),
        TypeNode::TypeFunc(n) => n.params.iter().any(|p| type_needs(&p.r#type)) || type_needs(&n.ret),
        TypeNode::TypeClosure(n) => {
            n.params.iter().any(|p| type_needs(&p.r#type))
                || type_needs(&n.ret)
                || n.deps_opt.as_ref().is_some_and(|deps| deps.iter().any(|dep| type_needs(&dep.r#type)))
        }
        TypeNode::TypeTuple(n) => types_need(&n.elements),
        TypeNode::TypeArray(n) => type_needs(&n.element) || expr_needs(&n.length),
        TypeNode::TypeSlice(n) => type_needs(&n.element),
        TypeNode::TypeSafePtr(n) => type_needs(&n.element),
        TypeNode::TypeRawPtr(n) => type_needs(&n.element),
        TypeNode::TypeRange(n) => type_needs(&n.base),
        TypeNode::TypeRangeInclusive(n) => type_needs(&n.base),
        TypeNode::TypeRangeFrom(n) => type_needs(&n.base),
        TypeNode::TypeRangeTo(n) => type_needs(&n.base),
        TypeNode::TypeRangeToInclusive(n) => type_needs(&n.base),
        TypeNode::TypePathType(n) => types_need(&n.generic_args),
        TypeNode::TypeApply(n) => types_need(&n.args),
        TypeNode::TypeModalState(n) => {
            let modal_ref_needs = match &n.modal_ref {
                TypeModalRef::TypePathType(path) => types_need(&path.generic_args),
                TypeModalRef::TypeApply(apply) => types_need(&apply.args),
            };
            modal_ref_needs || types_need(&n.generic_args)
        }
        TypeNode::TypeRefine(n) => type_needs(&n.base) || expr_needs(&n.predicate),
        TypeNode::SpliceExprNode(n) => expr_needs(&n.expr),
        _ => false,
    }
}

fn pattern_needs(pattern: &PatternPtr) -> bool {
    let Some(pattern) = pattern else {
        return false;
    };
    match &pattern.node {
        PatternNode::TypedPattern(n) => type_needs(&n.r#type),
        PatternNode::TuplePattern(n) => n.elements.iter().any(pattern_needs),
        PatternNode::RecordPattern(n) => field_patterns_need(&n.fields),
        PatternNode::EnumPattern(n) => match &n.payload_opt {
            Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => payload.elements.iter().any(pattern_needs),
            Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => field_patterns_need(&payload.fields),
            None => false,
        },
        PatternNode::ModalPattern(n) => {
            n.fields_opt.as_ref().is_some_and(|payload| field_patterns_need(&payload.fields))
        }
        PatternNode::RangePattern(n) => pattern_needs(&n.lo) || pattern_needs(&n.hi),
        PatternNode::SpliceExprNode(n) => expr_needs(&n.expr),
        _ => false,
    }
}

fn expr_needs(expr: &ExprPtr) -> bool {
    let Some(expr) = expr else {
        return false;
    };
    if has_files(expr_attr_list(expr)) {
        return true;
    }
    match &expr.node {
        ExprNode::QualifiedApplyExpr(n) => match &n.args {
            ApplyArgs::ParenArgs(paren) => args_need(&paren.args),
            ApplyArgs::BraceArgs(brace) => field_inits_need(&brace.fields),
        },
        ExprNode::EnumLiteralExpr(n) => match &n.payload_opt {
            Some(EnumPayload::EnumPayloadParen(paren)) => exprs_need(&paren.elements),
            Some(EnumPayload::EnumPayloadBrace(brace)) => field_inits_need(&brace.fields),
            None => false,
        },
        ExprNode::TypeLiteralExpr(n) => type_needs(&n.r#type),
        ExprNode::QuoteExpr(n) => n.tokens.iter().any(|token| token.lexeme == "files"),
        ExprNode::RangeExpr(n) => expr_needs(&n.lhs) || expr_needs(&n.rhs),
        ExprNode::BinaryExpr(n) => expr_needs(&n.lhs) || expr_needs(&n.rhs),
        ExprNode::CastExpr(n) => expr_needs(&n.value) || type_needs(&n.r#type),
        ExprNode::UnaryExpr(n) => expr_needs(&n.value),
        ExprNode::DerefExpr(n) => expr_needs(&n.value),
        ExprNode::AllocExpr(n) => expr_needs(&n.value),
        ExprNode::PropagateExpr(n) => expr_needs(&n.value),
        ExprNode::YieldExpr(n) => expr_needs(&n.value),
        ExprNode::YieldFromExpr(n) => expr_needs(&n.value),
        ExprNode::SyncExpr(n) => expr_needs(&n.value),
        ExprNode::AddressOfExpr(n) => expr_needs(&n.place),
        ExprNode::MoveExpr(n) => expr_needs(&n.place),
        ExprNode::EntryExpr(n) => expr_needs(&n.expr),
        ExprNode::WaitExpr(n) => expr_needs(&n.handle),
        ExprNode::TupleExpr(n) => exprs_need(&n.elements),
        ExprNode::ArrayExpr(n) => n.elements.iter().any(|segment| match segment {
            ArraySegment::ArrayElemSegment(elem) => expr_needs(&elem.value),
            ArraySegment::ArrayRepeatSegment(repeat) => expr_needs(&repeat.value) || expr_needs(&repeat.count),
        }),
        ExprNode::ArrayRepeatExpr(n) => expr_needs(&n.value) || expr_needs(&n.count),
        ExprNode::SizeofExpr(n) => type_needs(&n.r#type),
        ExprNode::AlignofExpr(n) => type_needs(&n.r#type),
        ExprNode::RecordExpr(n) => field_inits_need(&n.fields),
        ExprNode::IfExpr(n) => expr_needs(&n.cond) || expr_needs(&n.then_expr) || expr_needs(&n.else_expr),
        ExprNode::IfIsExpr(n) => {
            expr_needs(&n.scrutinee)
                || pattern_needs(&n.pattern)
                || expr_needs(&n.then_expr)
                || expr_needs(&n.else_expr)
        }
        ExprNode::IfCaseExpr(n) => {
            expr_needs(&n.scrutinee)
                || expr_needs(&n.else_expr)
                || n.cases.iter().any(|clause| pattern_needs(&clause.pattern) || expr_needs(&clause.body))
        }
        ExprNode::LoopInfiniteExpr(n) => invariant_needs(&n.invariant_opt) || block_needs(&n.body),
        ExprNode::LoopConditionalExpr(n) => {
            expr_needs(&n.cond) || invariant_needs(&n.invariant_opt) || block_needs(&n.body)
        }
        ExprNode::LoopIterExpr(n) => {
            pattern_needs(&n.pattern)
                || type_needs(&n.type_opt)
                || expr_needs(&n.iter)
                || invariant_needs(&n.invariant_opt)
                || block_needs(&n.body)
        }
        ExprNode::BlockExpr(n) => block_needs(&n.block),
        ExprNode::UnsafeBlockExpr(n) => block_needs(&n.block),
        ExprNode::ComptimeExpr(n) => {
            let attr_list = n.attrs_opt.as_deref().unwrap_or(&[]);
            has_files(attr_list) || has_emit(attr_list) || expr_needs(&n.body)
        }
        ExprNode::CtIfExpr(n) => {
            expr_needs(&n.cond) || block_needs(&n.then_block) || block_needs(&n.else_block_opt)
        }
        ExprNode::CtLoopIterExpr(n) => {
            pattern_needs(&n.pattern) || type_needs(&n.type_opt) || expr_needs(&n.iter) || block_needs(&n.body)
        }
        ExprNode::AttributedExpr(n) => has_files(&n.attrs) || expr_needs(&n.expr),
        ExprNode::TransmuteExpr(n) => type_needs(&n.from) || type_needs(&n.to) || expr_needs(&n.value),
        ExprNode::ClosureExpr(n) => {
            n.params.iter().any(|param| type_needs(&param.type_opt))
                || type_needs(&n.ret_type_opt)
                || expr_needs(&n.body)
        }
        ExprNode::PipelineExpr(n) => expr_needs(&n.lhs) || expr_needs(&n.rhs),
        ExprNode::FieldAccessExpr(n) => expr_needs(&n.base),
        ExprNode::TupleAccessExpr(n) => expr_needs(&n.base),
        ExprNode::IndexAccessExpr(n) => expr_needs(&n.base) || expr_needs(&n.index),
        ExprNode::CallExpr(n) => expr_needs(&n.callee) || args_need(&n.args) || types_need(&n.generic_args),
        ExprNode::CallTypeArgsExpr(n) => {
            expr_needs(&n.callee) || args_need(&n.args) || types_need(&n.type_args)
        }
        ExprNode::MethodCallExpr(n) => expr_needs(&n.receiver) || args_need(&n.args),
        ExprNode::RaceExpr(n) => n.arms.iter().any(|arm| {
            expr_needs(&arm.expr) || pattern_needs(&arm.pattern) || expr_needs(&arm.handler.value)
        }),
        ExprNode::AllExpr(n) => exprs_need(&n.exprs),
        ExprNode::ParallelExpr(n) => {
            expr_needs(&n.domain) || block_needs(&n.body) || n.opts.iter().any(|opt| expr_needs(&opt.value))
        }
        ExprNode::SpawnExpr(n) => block_needs(&n.body) || n.opts.iter().any(|opt| expr_needs(&opt.value)),
        ExprNode::DispatchExpr(n) => {
            pattern_needs(&n.pattern)
                || expr_needs(&n.range)
                || block_needs(&n.body)
                || n.key_clause.as_ref().is_some_and(|clause| key_path_needs(&clause.key_path))
                || n.opts.iter().any(|opt| expr_needs(&opt.chunk_expr) || expr_needs(&opt.workgroup_expr))
        }
        ExprNode::SpliceExprNode(n) => expr_needs(&n.expr),
        ExprNode::SpliceIdentNode(n) => expr_needs(&n.name_expr),
        _ => false,
    }
}

fn binding_needs(binding: &Binding) -> bool {
    has_files(&binding.attrs)
        || pattern_needs(&binding.pat)
        || type_needs(&binding.type_opt)
        || expr_needs(&binding.init)
}

fn block_needs(block: &BlockPtr) -> bool {
    block.as_ref().is_some_and(|block| block.stmts.iter().any(stmt_needs) || expr_needs(&block.tail_opt))
}

fn stmt_needs(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::LetStmt(n) => binding_needs(&n.binding),
        Stmt::VarStmt(n) => binding_needs(&n.binding),
        Stmt::AssignStmt(n) => expr_needs(&n.place) || expr_needs(&n.value),
        Stmt::CompoundAssignStmt(n) => expr_needs(&n.place) || expr_needs(&n.value),
        Stmt::ExprStmt(n) => expr_needs(&n.value),
        Stmt::DeferStmt(n) => block_needs(&n.body),
        Stmt::FrameStmt(n) => block_needs(&n.body),
        Stmt::UnsafeBlockStmt(n) => block_needs(&n.body),
        Stmt::RegionStmt(n) => expr_needs(&n.opts_opt) || block_needs(&n.body),
        Stmt::ReturnStmt(n) => expr_needs(&n.value_opt),
        Stmt::BreakStmt(n) => expr_needs(&n.value_opt),
        Stmt::CtStmt(n) => has_files(&n.attrs) || has_emit(&n.attrs) || block_needs(&n.body),
        Stmt::KeyBlockStmt(n) => {
            has_files(&n.attrs) || block_needs(&n.body) || n.paths.iter().any(key_path_needs)
        }
        _ => false,
    }
}

fn field_decl_needs(field: &FieldDecl) -> bool {
    has_files(&field.attrs) || type_needs(&field.r#type) || expr_needs(&field.init_opt)
}

fn record_member_needs(member: &RecordMember) -> bool {
    match member {
        RecordMember::FieldDecl(n) => field_decl_needs(n),
        RecordMember::MethodDecl(n) => {
            has_files(&n.attrs)
                || generics_need(&n.generic_params)
                || receiver_needs(&n.receiver)
                || params_need(&n.params)
                || type_needs(&n.return_type_opt)
                || contract_needs(&n.contract)
                || block_needs(&n.body)
        }
        RecordMember::AssociatedTypeDecl(n) => has_files(&n.attrs) || type_needs(&n.default_type),
    }
}

fn state_member_needs(member: &StateMember) -> bool {
    match member {
        StateMember::StateFieldDecl(n) => has_files(&n.attrs) || type_needs(&n.r#type),
        StateMember::StateMethodDecl(n) => {
            has_files(&n.attrs)
                || generics_need(&n.generic_params)
                || receiver_needs(&n.receiver)
                || params_need(&n.params)
                || type_needs(&n.return_type_opt)
                || contract_needs(&n.contract)
                || block_needs(&n.body)
        }
        StateMember::TransitionDecl(n) => has_files(&n.attrs) || params_need(&n.params) || block_needs(&n.body),
    }
}

fn class_item_needs(item: &ClassItem) -> bool {
    match item {
        ClassItem::ClassFieldDecl(n) => has_files(&n.attrs) || type_needs(&n.r#type),
        ClassItem::AbstractFieldDecl(n) => has_files(&n.attrs) || type_needs(&n.r#type),
        ClassItem::ClassMethodDecl(n) => {
            has_files(&n.attrs)
                || generics_need(&n.generic_params)
                || receiver_needs(&n.receiver)
                || params_need(&n.params)
                || type_needs(&n.return_type_opt)
                || contract_needs(&n.contract)
                || block_needs(&n.body_opt)
        }
        ClassItem::AssociatedTypeDecl(n) => has_files(&n.attrs) || type_needs(&n.default_type),
        ClassItem::AbstractStateDecl(n) => {
            has_files(&n.attrs)
                || n.fields.iter().any(|field| has_files(&field.attrs) || type_needs(&field.r#type))
        }
    }
}

fn extern_proc_needs(proc: &ExternProcDecl) -> bool {
    has_files(&proc.attrs)
        || generics_need(&proc.generic_params)
        || params_need(&proc.params)
        || type_needs(&proc.return_type_opt)
        || contract_needs(&proc.contract)
        || proc
            .foreign_contracts_opt
            .as_ref()
            .is_some_and(|clauses| clauses.iter().any(|clause| exprs_need(&clause.predicates)))
}

fn item_needs(item: &ASTItem) -> bool {
    if has_files(attr_list_of(item)) {
        return true;
    }
    match item {
        ASTItem::StaticDecl(n) => binding_needs(&n.binding),
        ASTItem::ProcedureDecl(n) => {
            generics_need(&n.generic_params)
                || params_need(&n.params)
                || type_needs(&n.return_type_opt)
                || contract_needs(&n.contract)
                || block_needs(&n.body)
        }
        ASTItem::ComptimeProcedureDecl(n) => {
            generics_need(&n.generic_params)
                || params_need(&n.params)
                || type_needs(&n.return_type_opt)
                || contract_needs(&n.contract)
                || block_needs(&n.body)
        }
        ASTItem::ExternBlock(n) => n.items.iter().any(|ExternItem::ExternProcDecl(proc)| extern_proc_needs(proc)),
        ASTItem::RecordDecl(n) => {
            has_derive(&n.attrs)
                || generics_need(&n.generic_params)
                || type_invariant_needs(&n.invariant_opt)
                || n.members.iter().any(record_member_needs)
        }
        ASTItem::EnumDecl(n) => {
            has_derive(&n.attrs)
                || generics_need(&n.generic_params)
                || type_invariant_needs(&n.invariant_opt)
                || n.variants.iter().any(|variant| match &variant.payload_opt {
                    Some(VariantPayload::VariantPayloadTuple(payload)) => types_need(&payload.elements),
                    Some(VariantPayload::VariantPayloadRecord(payload)) => {
                        payload.fields.iter().any(field_decl_needs)
                    }
                    None => false,
                })
        }
        ASTItem::ModalDecl(n) => {
            has_derive(&n.attrs)
                || generics_need(&n.generic_params)
                || type_invariant_needs(&n.invariant_opt)
                || n.states.iter().any(|state| state.members.iter().any(state_member_needs))
        }
        ASTItem::ClassDecl(n) => generics_need(&n.generic_params) || n.items.iter().any(class_item_needs),
        ASTItem::TypeAliasDecl(n) => generics_need(&n.generic_params) || type_needs(&n.r#type),
        ASTItem::DeriveTargetDecl(_) => true,
        _ => false,
    }
}

fn comptime_may_need_project_files(modules: &[ASTModule]) -> bool {
    modules.iter().any(|module| module.items.iter().any(item_needs))
}

fn source_root_for_module(module: &ASTModule, options: &ComptimePassOptions) -> String {
    module
        .path
        .first()
        .and_then(|assembly| options.source_roots_by_assembly.get(assembly))
        .or(options.fallback_source_root.as_ref())
        .unwrap_or(&options.project_root)
        .clone()
}

pub fn comptime_pass(modules: &[ASTModule], options: &ComptimePassOptions) -> ComptimeResult {
    spec_rule!("ComptimePass");
    spec_rule!("requirement.22.ComptimePassExecutionRequirements");
    let snapshot_start = std::time::Instant::now();
    let project_files = if comptime_may_need_project_files(modules) {
        let snapshot = Rc::new(capture_project_file_snapshot(&options.project_root));
        log_comptime_progress(|| {
            format!(
                "snapshot=captured entries={} files={} dirs={} bytes={} elapsed_ms={}",
                snapshot.entries.len(),
                snapshot.captured_file_count,
                snapshot.captured_directory_count,
                snapshot.captured_byte_count,
                snapshot_start.elapsed().as_millis()
            )
        });
        Some(snapshot)
    } else {
        log_comptime_progress(|| "snapshot=skipped reason=no-files-attribute".to_string());
        None
    };
    let diags = Rc::new(RefCell::new(DiagnosticStream::new()));
    let finish = |modules: Option<Vec<ASTModule>>, diags: &Rc<RefCell<DiagnosticStream>>| ComptimeResult {
        modules,
        diags: diags.borrow().clone(),
    };
    let originals: Vec<Rc<ASTModule>> = modules.iter().cloned().map(Rc::new).collect();
    let mut expanded: Vec<Rc<ASTModule>> = originals.clone();
    let mut next_hygiene = 0usize;
    if modules.is_empty() {
        spec_rule!("ComptimePass-Empty");
    }
    for (module_index, module) in originals.iter().enumerate() {
        spec_rule!("ComptimePass-Cons");
        spec_rule!("Phase2-Deterministic-Dependency-Order");
        spec_rule!("requirement.22.Phase2ExecutionPosition");
        // Every earlier module has been expanded: its source form is visible for lookup
        // and its expanded form for phase-2 queries. The current module is visible as
        // written.
        let mut available_modules: Vec<Rc<ASTModule>> = originals[..module_index].to_vec();
        available_modules.push(module.clone());
        let available_module_names: ModuleNames =
            available_modules.iter().map(|available| string_of_path(&available.path)).collect();
        let mut env = ct_empty_env(module);
        env.diags = Some(diags.clone());
        env.project_root = options.project_root.clone();
        env.source_root = source_root_for_module(module, options);
        env.next_hygiene = next_hygiene;
        env.available_modules = available_modules;
        env.phase2_expanded_modules = expanded[..module_index].to_vec();
        env.available_module_names = available_module_names;
        env.files = project_files.clone();
        spec_rule!("CtExecModule");
        log_comptime_progress(|| {
            format!(
                "module-start index={module_index} path={} items={}",
                string_of_path(&module.path),
                module.items.len()
            )
        });
        let expanded_items = expand_module_items(&module.items, &mut env);
        log_comptime_progress(|| {
            format!(
                "module-finish index={module_index} path={} diags={}",
                string_of_path(&module.path),
                diags.borrow().len()
            )
        });
        let Some(items) = expanded_items.filter(|_| !has_error(&diags.borrow())) else {
            return finish(None, &diags);
        };
        let comptime_procedures = module
            .items
            .iter()
            .filter_map(|item| match item {
                ASTItem::ComptimeProcedureDecl(proc) => Some(proc.clone()),
                _ => None,
            })
            .collect();
        next_hygiene = env.next_hygiene;
        expanded[module_index] = Rc::new(ASTModule {
            path: module.path.clone(),
            items,
            comptime_procedures,
            module_doc: module.module_doc.clone(),
        });
    }
    let modules = expanded.into_iter().map(|module| Rc::try_unwrap(module).unwrap_or_else(|rc| (*rc).clone())).collect();
    finish(Some(modules), &diags)
}

/// Phase 2 of translation.
pub fn execute_comptime(modules: &[ASTModule], options: &ComptimePassOptions) -> ComptimeResult {
    spec_rule!("ExecuteComptime");
    spec_rule!("ExecuteComptime-By-ComptimePass");
    spec_rule!("requirement.22.Phase2ExecutionPosition");
    comptime_pass(modules, options)
}
