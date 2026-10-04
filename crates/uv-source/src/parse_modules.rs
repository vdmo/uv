//! Phase 1 for whole modules: read, load and parse every file of a module, run the
//! file-level syntactic checks, and aggregate the files into one `ASTModule`.

use std::collections::HashMap;
use std::sync::Arc;

use uv_core::diagnostic_messages::{emit_external_diagnostic, make_diagnostic_by_id};
use uv_core::diagnostics::{emit, has_error, DiagnosticStream};
use uv_core::host::host_get_env_utf8;
use uv_core::process_config::is_debug_enabled;
use uv_core::source_load::load_source;
use uv_core::source_text::SourceFile;
use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;
use uv_project::fs_path::fs_join;
use uv_project::module_discovery::{compilation_unit, ModuleInfo};

use crate::ast::*;
use crate::lexer::keyword_policy::is_keyword;
use crate::parser::parse_file;

pub type UnsafeSpanMap = HashMap<String, Vec<Span>>;

#[derive(Default)]
pub struct ReadBytesResult {
    pub bytes: Option<Vec<u8>>,
    pub diags: DiagnosticStream,
}

#[derive(Default)]
pub struct ParseModuleResult {
    pub module: Option<ASTModule>,
    pub diags: DiagnosticStream,
    pub unsafe_spans_by_file: UnsafeSpanMap,
}

#[derive(Default)]
pub struct ParseModulesResult {
    pub modules: Option<Vec<ASTModule>>,
    pub diags: DiagnosticStream,
    pub unsafe_spans_by_file: UnsafeSpanMap,
}

type ReadBytesFn<'a> = &'a dyn Fn(&str) -> ReadBytesResult;
type InspectSourceFn<'a> = &'a dyn Fn(&SourceFile) -> DiagnosticStream;

/// Hooks that let tooling substitute file contents and add per-source checks.
#[derive(Clone, Copy)]
pub struct ParseModuleDeps<'a> {
    pub read_bytes: ReadBytesFn<'a>,
    pub inspect_source: Option<InspectSourceFn<'a>>,
}

impl Default for ParseModuleDeps<'_> {
    fn default() -> Self {
        ParseModuleDeps { read_bytes: &read_bytes_default, inspect_source: None }
    }
}

pub fn read_bytes_default(path: &str) -> ReadBytesResult {
    let mut result = ReadBytesResult::default();
    let forced_failure =
        host_get_env_utf8("UV_TEST_READ_BYTES_FAIL").is_some_and(|force| !force.is_empty());
    match (forced_failure, std::fs::read(path)) {
        (false, Ok(bytes)) => {
            spec_rule!("ReadBytes-Ok");
            result.bytes = Some(bytes);
        }
        _ => {
            spec_rule!("ReadBytes-Err");
            emit_external_diagnostic(&mut result.diags, "E-SRC-0102");
        }
    }
    result
}

fn append_diags(out: &mut DiagnosticStream, add: DiagnosticStream) {
    for diag in add {
        emit(out, diag);
    }
}

// ---- reserved binders -------------------------------------------------------

/// Reports every binder whose name is a reserved keyword, once per source span.
struct ReservedBinders<'a> {
    diags: &'a mut DiagnosticStream,
}

trait Visit<T: ?Sized> {
    fn visit(&mut self, node: &T);
}

impl ReservedBinders<'_> {
    fn binder(&mut self, name: &str, span: &Span) {
        if !is_keyword(name) {
            return;
        }
        let already_reported = self
            .diags
            .iter()
            .any(|diag| diag.code == "E-CNF-0401" && diag.span.as_ref() == Some(span));
        if already_reported {
            return;
        }
        if let Some(diag) = make_diagnostic_by_id("E-CNF-0401", Some(span.clone())) {
            emit(self.diags, diag);
        }
    }

    fn generics(&mut self, params: &Option<GenericParams>) {
        for param in params.iter().flat_map(|params| &params.params) {
            self.binder(&param.name, &param.span);
        }
    }

    fn params(&mut self, params: &[Param]) {
        for param in params {
            self.binder(&param.name, &param.span);
        }
    }

    fn args(&mut self, args: &[Arg]) {
        for arg in args {
            self.visit(&arg.value);
        }
    }

    fn field_inits(&mut self, fields: &[FieldInit]) {
        for field in fields {
            self.visit(&field.value);
        }
    }

    fn field_patterns(&mut self, fields: &[FieldPattern]) {
        for field in fields {
            if field.pattern_opt.is_some() {
                self.visit(&field.pattern_opt);
            } else {
                self.binder(&field.name, &field.span);
            }
        }
    }

    fn field_decl(&mut self, field: &FieldDecl) {
        self.binder(&field.name, &field.span);
        self.visit(&field.init_opt);
    }

    fn class_item(&mut self, item: &ClassItem) {
        match item {
            ClassItem::ClassFieldDecl(d) => self.binder(&d.name, &d.span),
            ClassItem::AssociatedTypeDecl(d) => self.binder(&d.name, &d.span),
            ClassItem::AbstractFieldDecl(d) => self.binder(&d.name, &d.span),
            ClassItem::ClassMethodDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                self.params(&d.params);
                self.visit(&d.body_opt);
            }
            ClassItem::AbstractStateDecl(d) => {
                self.binder(&d.name, &d.span);
                for field in &d.fields {
                    self.binder(&field.name, &field.span);
                }
            }
        }
    }

    fn item(&mut self, item: &ASTItem) {
        match item {
            ASTItem::UsingDecl(d) => match &d.clause {
                UsingClause::UsingItem(clause) => {
                    self.binder(clause.alias_opt.as_ref().unwrap_or(&clause.name), &d.span);
                }
                UsingClause::UsingList(clause) => {
                    for spec in &clause.specs {
                        self.binder(spec.alias_opt.as_ref().unwrap_or(&spec.name), &d.span);
                    }
                }
                UsingClause::UsingWildcard(_) => {}
            },
            ASTItem::ImportDecl(d) => {
                if let Some(alias) = &d.alias_opt {
                    self.binder(alias, &d.span);
                }
            }
            ASTItem::ExternBlock(d) => {
                for ExternItem::ExternProcDecl(ext) in &d.items {
                    self.binder(&ext.name, &ext.span);
                    self.generics(&ext.generic_params);
                    self.params(&ext.params);
                }
            }
            ASTItem::StaticDecl(d) => {
                self.visit(&d.binding.pat);
                self.visit(&d.binding.init);
            }
            ASTItem::ProcedureDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                self.params(&d.params);
                self.visit(&d.body);
            }
            ASTItem::ComptimeProcedureDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                self.params(&d.params);
                self.visit(&d.body);
            }
            ASTItem::RecordDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                for member in &d.members {
                    match member {
                        RecordMember::FieldDecl(field) => self.field_decl(field),
                        RecordMember::MethodDecl(method) => {
                            self.binder(&method.name, &method.span);
                            self.generics(&method.generic_params);
                            self.params(&method.params);
                            self.visit(&method.body);
                        }
                        RecordMember::AssociatedTypeDecl(assoc) => self.binder(&assoc.name, &assoc.span),
                    }
                }
            }
            ASTItem::EnumDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                for variant in &d.variants {
                    self.binder(&variant.name, &variant.span);
                    if let Some(VariantPayload::VariantPayloadRecord(payload)) = &variant.payload_opt {
                        for field in &payload.fields {
                            self.field_decl(field);
                        }
                    }
                }
            }
            ASTItem::ModalDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                for state in &d.states {
                    self.binder(&state.name, &state.span);
                    for member in &state.members {
                        match member {
                            StateMember::StateFieldDecl(field) => self.binder(&field.name, &field.span),
                            StateMember::StateMethodDecl(method) => {
                                self.binder(&method.name, &method.span);
                                self.generics(&method.generic_params);
                                self.params(&method.params);
                                self.visit(&method.body);
                            }
                            StateMember::TransitionDecl(transition) => {
                                self.binder(&transition.name, &transition.span);
                                self.params(&transition.params);
                                self.visit(&transition.body);
                            }
                        }
                    }
                }
            }
            ASTItem::ClassDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
                for class_item in &d.items {
                    self.class_item(class_item);
                }
            }
            ASTItem::TypeAliasDecl(d) => {
                self.binder(&d.name, &d.span);
                self.generics(&d.generic_params);
            }
            ASTItem::DeriveTargetDecl(d) => {
                self.binder(&d.name, &d.span);
                self.visit(&d.body);
            }
            ASTItem::ErrorItem(_) => {}
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(LetStmt { binding, .. }) | Stmt::VarStmt(VarStmt { binding, .. }) => {
                self.visit(&binding.pat);
                self.visit(&binding.init);
            }
            Stmt::UsingLocalStmt(s) => self.binder(&s.alias, &s.span),
            Stmt::AssignStmt(s) => {
                self.visit(&s.place);
                self.visit(&s.value);
            }
            Stmt::CompoundAssignStmt(s) => {
                self.visit(&s.place);
                self.visit(&s.value);
            }
            Stmt::ExprStmt(s) => self.visit(&s.value),
            Stmt::DeferStmt(s) => self.visit(&s.body),
            Stmt::FrameStmt(s) => self.visit(&s.body),
            Stmt::UnsafeBlockStmt(s) => self.visit(&s.body),
            Stmt::CtStmt(s) => self.visit(&s.body),
            Stmt::KeyBlockStmt(s) => self.visit(&s.body),
            Stmt::RegionStmt(s) => {
                if let Some(alias) = &s.alias_opt {
                    self.binder(alias, &s.span);
                }
                self.visit(&s.opts_opt);
                self.visit(&s.body);
            }
            Stmt::ReturnStmt(s) => self.visit(&s.value_opt),
            Stmt::BreakStmt(s) => self.visit(&s.value_opt),
            Stmt::ContinueStmt(_) | Stmt::ErrorStmt(_) => {}
        }
    }

    fn expr(&mut self, expr: &Expr) {
        match &expr.node {
            ExprNode::QualifiedApplyExpr(n) => match &n.args {
                ApplyArgs::ParenArgs(paren) => self.args(&paren.args),
                ApplyArgs::BraceArgs(brace) => self.field_inits(&brace.fields),
            },
            ExprNode::RangeExpr(n) => {
                self.visit(&n.lhs);
                self.visit(&n.rhs);
            }
            ExprNode::BinaryExpr(n) => {
                self.visit(&n.lhs);
                self.visit(&n.rhs);
            }
            ExprNode::ArrayRepeatExpr(n) => {
                self.visit(&n.value);
                self.visit(&n.count);
            }
            ExprNode::UnaryExpr(n) => self.visit(&n.value),
            ExprNode::DerefExpr(n) => self.visit(&n.value),
            ExprNode::AllocExpr(n) => self.visit(&n.value),
            ExprNode::PropagateExpr(n) => self.visit(&n.value),
            ExprNode::YieldExpr(n) => self.visit(&n.value),
            ExprNode::YieldFromExpr(n) => self.visit(&n.value),
            ExprNode::SyncExpr(n) => self.visit(&n.value),
            ExprNode::CastExpr(n) => self.visit(&n.value),
            ExprNode::TransmuteExpr(n) => self.visit(&n.value),
            ExprNode::TupleExpr(n) => {
                for elem in &n.elements {
                    self.visit(elem);
                }
            }
            ExprNode::ArrayExpr(n) => {
                for segment in &n.elements {
                    match segment {
                        ArraySegment::ArrayElemSegment(elem) => self.visit(&elem.value),
                        ArraySegment::ArrayRepeatSegment(repeat) => {
                            self.visit(&repeat.value);
                            self.visit(&repeat.count);
                        }
                    }
                }
            }
            ExprNode::RecordExpr(n) => self.field_inits(&n.fields),
            ExprNode::EnumLiteralExpr(n) => match &n.payload_opt {
                Some(EnumPayload::EnumPayloadParen(paren)) => {
                    for elem in &paren.elements {
                        self.visit(elem);
                    }
                }
                Some(EnumPayload::EnumPayloadBrace(brace)) => self.field_inits(&brace.fields),
                None => {}
            },
            ExprNode::IfExpr(n) => {
                self.visit(&n.cond);
                self.visit(&n.then_expr);
                self.visit(&n.else_expr);
            }
            ExprNode::IfIsExpr(n) => {
                self.visit(&n.scrutinee);
                self.visit(&n.pattern);
                self.visit(&n.then_expr);
                self.visit(&n.else_expr);
            }
            ExprNode::IfCaseExpr(n) => {
                self.visit(&n.scrutinee);
                for clause in &n.cases {
                    self.visit(&clause.pattern);
                    self.visit(&clause.body);
                }
                self.visit(&n.else_expr);
            }
            ExprNode::LoopInfiniteExpr(n) => self.visit(&n.body),
            ExprNode::LoopConditionalExpr(n) => {
                self.visit(&n.cond);
                self.visit(&n.body);
            }
            ExprNode::LoopIterExpr(n) => {
                self.visit(&n.pattern);
                self.visit(&n.iter);
                self.visit(&n.body);
            }
            ExprNode::BlockExpr(n) => self.visit(&n.block),
            ExprNode::UnsafeBlockExpr(n) => self.visit(&n.block),
            ExprNode::ComptimeExpr(n) => self.visit(&n.body),
            ExprNode::AttributedExpr(n) => self.visit(&n.expr),
            ExprNode::PipelineExpr(n) => {
                self.visit(&n.lhs);
                self.visit(&n.rhs);
            }
            ExprNode::CtIfExpr(n) => {
                self.visit(&n.cond);
                self.visit(&n.then_block);
                self.visit(&n.else_block_opt);
            }
            ExprNode::CtLoopIterExpr(n) => {
                self.visit(&n.pattern);
                self.visit(&n.iter);
                self.visit(&n.body);
            }
            ExprNode::ClosureExpr(n) => {
                for param in &n.params {
                    self.binder(&param.name, &expr.span);
                }
                self.visit(&n.body);
            }
            ExprNode::FieldAccessExpr(n) => self.visit(&n.base),
            ExprNode::TupleAccessExpr(n) => self.visit(&n.base),
            ExprNode::IndexAccessExpr(n) => {
                self.visit(&n.base);
                self.visit(&n.index);
            }
            ExprNode::CallExpr(n) => {
                self.visit(&n.callee);
                self.args(&n.args);
            }
            ExprNode::CallTypeArgsExpr(n) => {
                self.visit(&n.callee);
                self.args(&n.args);
            }
            ExprNode::MethodCallExpr(n) => {
                self.visit(&n.receiver);
                self.args(&n.args);
            }
            ExprNode::RaceExpr(n) => {
                for arm in &n.arms {
                    self.visit(&arm.expr);
                    self.visit(&arm.pattern);
                    self.visit(&arm.handler.value);
                }
            }
            ExprNode::AllExpr(n) => {
                for elem in &n.exprs {
                    self.visit(elem);
                }
            }
            ExprNode::ParallelExpr(n) => {
                self.visit(&n.domain);
                for opt in &n.opts {
                    self.visit(&opt.value);
                }
                self.visit(&n.body);
            }
            ExprNode::SpawnExpr(n) => {
                for opt in &n.opts {
                    self.visit(&opt.value);
                }
                self.visit(&n.body);
            }
            ExprNode::DispatchExpr(n) => {
                self.visit(&n.pattern);
                self.visit(&n.range);
                for opt in &n.opts {
                    match opt.kind {
                        DispatchOptionKind::Chunk => self.visit(&opt.chunk_expr),
                        DispatchOptionKind::Workgroup => self.visit(&opt.workgroup_expr),
                        _ => {}
                    }
                }
                self.visit(&n.body);
            }
            // The reference does not descend into the remaining forms.
            _ => {}
        }
    }

    fn pattern(&mut self, pattern: &Pattern) {
        match &pattern.node {
            PatternNode::IdentifierPattern(n) => self.binder(&n.name, &pattern.span),
            PatternNode::TypedPattern(n) => self.binder(&n.name, &pattern.span),
            PatternNode::TuplePattern(n) => {
                for elem in &n.elements {
                    self.visit(elem);
                }
            }
            PatternNode::RecordPattern(n) => self.field_patterns(&n.fields),
            PatternNode::EnumPattern(n) => match &n.payload_opt {
                Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                    for elem in &payload.elements {
                        self.visit(elem);
                    }
                }
                Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                    self.field_patterns(&payload.fields);
                }
                None => {}
            },
            PatternNode::ModalPattern(n) => {
                if let Some(payload) = &n.fields_opt {
                    self.field_patterns(&payload.fields);
                }
            }
            PatternNode::RangePattern(n) => {
                self.visit(&n.lo);
                self.visit(&n.hi);
            }
            PatternNode::LiteralPattern(_)
            | PatternNode::WildcardPattern(_)
            | PatternNode::SpliceExprNode(_) => {}
        }
    }
}

impl Visit<Option<Arc<Expr>>> for ReservedBinders<'_> {
    fn visit(&mut self, node: &Option<Arc<Expr>>) {
        if let Some(expr) = node {
            self.expr(expr);
        }
    }
}

impl Visit<Option<Arc<Pattern>>> for ReservedBinders<'_> {
    fn visit(&mut self, node: &Option<Arc<Pattern>>) {
        if let Some(pattern) = node {
            self.pattern(pattern);
        }
    }
}

impl Visit<Option<Arc<Block>>> for ReservedBinders<'_> {
    fn visit(&mut self, node: &Option<Arc<Block>>) {
        if let Some(block) = node {
            for stmt in &block.stmts {
                self.stmt(stmt);
            }
            self.visit(&block.tail_opt);
        }
    }
}

pub fn validate_reserved_binders(file: &ASTFile, diags: &mut DiagnosticStream) {
    let mut visitor = ReservedBinders { diags };
    for item in &file.items {
        visitor.item(item);
    }
}

/// A free procedure may not declare a `self` parameter.
pub fn check_method_context(file: &ASTFile, diags: &mut DiagnosticStream) {
    for item in &file.items {
        let ASTItem::ProcedureDecl(proc) = item else {
            continue;
        };
        if let Some(param) = proc.params.iter().find(|param| param.name == "self") {
            if let Some(diag) = make_diagnostic_by_id("E-SEM-3011", Some(param.span.clone())) {
                emit(diags, diag);
            }
        }
    }
}

// ---- modules ----------------------------------------------------------------

fn split_module_path(path: &str) -> Vec<String> {
    path.split("::").map(str::to_string).collect()
}

/// Directory of a module: the source root for the assembly's root module, otherwise the
/// module path below it (without the leading assembly name).
pub fn dir_of(module_path: &str, source_root: &str, assembly_name: &str) -> String {
    if module_path == assembly_name {
        spec_rule!("DirOf-Root");
        return source_root.to_string();
    }
    spec_rule!("DirOf-Rel");
    let comps = split_module_path(module_path);
    let skip = usize::from(comps.first().is_some_and(|first| first == assembly_name));
    comps.iter().skip(skip).fold(source_root.to_string(), |dir, comp| fs_join(&dir, comp))
}

pub fn parse_module_from_dir(
    module_path: &str,
    module_dir: &str,
    deps: &ParseModuleDeps<'_>,
) -> ParseModuleResult {
    let mut result = ParseModuleResult::default();
    let debug_phases = is_debug_enabled("phases");
    let log_phase = |label: &str, path: &str| {
        if debug_phases {
            eprintln!("[uv] parse: {label} {path}");
        }
    };
    spec_rule!("Mod-Start");
    let unit = compilation_unit(module_dir);
    let unit_failed = has_error(&unit.diags);
    append_diags(&mut result.diags, unit.diags);
    if unit_failed {
        spec_rule!("Mod-Start-Err-Unit");
        spec_rule!("ParseModule-Err-Unit");
        return result;
    }
    let mut items = Vec::new();
    let mut docs = Vec::new();
    for file in &unit.files {
        log_phase("read", file);
        let bytes = (deps.read_bytes)(file);
        append_diags(&mut result.diags, bytes.diags);
        let Some(bytes) = bytes.bytes else {
            spec_rule!("Mod-Scan-Err-Read");
            spec_rule!("ParseModule-Err-Read");
            return result;
        };
        let load = load_source(file, &bytes);
        append_diags(&mut result.diags, load.diags);
        let Some(source) = load.source else {
            spec_rule!("Mod-Scan-Err-Load");
            spec_rule!("ParseModule-Err-Load");
            Conformance::record_at(
                "req.LoadSourceShortCircuit",
                None,
                &format!(
                    "source=ParseModuleWithDeps;file={file};load_source_ok=false;tokenize_invoked=false;parse_file_invoked=false;syntactic_checks_invoked=false"
                ),
            );
            return result;
        };
        let inspect_diags = match deps.inspect_source {
            Some(inspect_source) => {
                log_phase("inspect", file);
                inspect_source(&source)
            }
            None => DiagnosticStream::new(),
        };
        log_phase("parse", file);
        let parsed = parse_file(&source);
        if is_debug_enabled("parse") {
            eprintln!(
                "[uv] parse: file={file} diags={} ok={}",
                parsed.diags.len(),
                if parsed.file.is_some() { "yes" } else { "no" }
            );
        }
        append_diags(&mut result.diags, parsed.diags);
        append_diags(&mut result.diags, inspect_diags);
        let Some(parsed_file) = parsed.file else {
            spec_rule!("Mod-Scan-Err-Parse");
            spec_rule!("ParseModule-Err-Parse");
            return result;
        };
        check_method_context(&parsed_file, &mut result.diags);
        validate_reserved_binders(&parsed_file, &mut result.diags);
        spec_rule!("Mod-Scan");
        items.extend(parsed_file.items);
        docs.extend(parsed_file.module_doc);
        result.unsafe_spans_by_file.insert(source.path.to_string(), parsed.unsafe_spans);
    }
    spec_rule!("Mod-Done");
    result.module = Some(ASTModule {
        path: split_module_path(module_path),
        items,
        comptime_procedures: Vec::new(),
        module_doc: docs,
    });
    spec_rule!("ParseModule-Ok");
    result
}

/// Parses the modules of one assembly in order, stopping at the first module that cannot
/// be formed. The reference parses modules on worker threads and merges the results in
/// this same order, so the observable result is identical.
pub fn parse_modules(
    modules: &[ModuleInfo],
    source_root: &str,
    assembly_name: &str,
    deps: &ParseModuleDeps<'_>,
) -> ParseModulesResult {
    let mut result = ParseModulesResult::default();
    let mut parsed_modules = Vec::with_capacity(modules.len());
    for module in modules {
        let computed_dir;
        let module_dir = if module.dir.is_empty() {
            computed_dir = dir_of(&module.path, source_root, assembly_name);
            &computed_dir
        } else {
            &module.dir
        };
        let parsed = parse_module_from_dir(&module.path, module_dir, deps);
        append_diags(&mut result.diags, parsed.diags);
        result.unsafe_spans_by_file.extend(parsed.unsafe_spans_by_file);
        let Some(module) = parsed.module else {
            spec_rule!("ParseModules-Err");
            return result;
        };
        parsed_modules.push(module);
    }
    spec_rule!("ParseModules-Ok");
    spec_rule!("Phase1-Complete");
    spec_rule!("Phase1-Declarations");
    spec_rule!("Phase1-Forward-Refs");
    result.modules = Some(parsed_modules);
    result
}
