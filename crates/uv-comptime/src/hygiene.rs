//! Hygiene for quoted syntax: bindings introduced by a quote are renamed to names that
//! cannot collide with anything at the place the syntax is inserted. Names produced by an
//! identifier splice are the user's choice and are kept.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::span::Span;
use uv_source::ast::*;

use crate::value::*;

/// Mutable access to a syntax node that may be shared.
///
/// The reference compiler renames quoted syntax in place through shared pointers, and the
/// result is observable: a quoted value that is emitted twice, or spliced into two quotes,
/// is renamed again on every insertion, and the earlier insertions see the new names
/// because they point to the same nodes. Copy-on-write (`Arc::make_mut`) would rename only
/// the latest copy and produce different output, so the node is written through its
/// shared pointer here.
///
/// # Safety
///
/// This is `Arc::get_mut_unchecked`: the caller must not dereference any other pointer to
/// the same node while the returned borrow is live. Renaming upholds that because it is
/// single-threaded, walks the tree strictly top-down and never holds a reference to a node
/// while visiting it again (syntax trees are acyclic), and the only other holders are
/// compile-time values, pending emits and already-built quotes, none of which are read
/// until renaming returns.
#[allow(clippy::mut_from_ref)]
fn shared_node_mut<T>(node: &Arc<T>) -> &mut T {
    // SAFETY: see the function documentation.
    unsafe { &mut *Arc::as_ptr(node).cast_mut() }
}

struct HygieneContext {
    diags: Option<Rc<RefCell<DiagnosticStream>>>,
    quote_site: CtSite,
    emit_site: CtSite,
    next_seed: usize,
    ok: bool,
    all_names: HashSet<String>,
    unhygienic_names: HashSet<String>,
    scopes: Vec<HashMap<String, String>>,
    /// Where diagnostics about the syntax being renamed are reported.
    diag_span: Span,
}

fn fnv1a_append(mut hash: u64, text: &str) -> u64 {
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    hash
}

fn fnv1a_append_site(mut hash: u64, site: &CtSite) -> u64 {
    hash = fnv1a_append(hash, &site.module_path.join("::"));
    hash = fnv1a_append(hash, &site.ordinal.to_string());
    hash = fnv1a_append(hash, &site.span.file);
    for number in [site.span.start_line, site.span.start_col, site.span.end_line, site.span.end_col] {
        hash = fnv1a_append(hash, &number.to_string());
    }
    hash
}

fn site_hash(quote_site: &CtSite, emit_site: &CtSite) -> u64 {
    fnv1a_append_site(fnv1a_append_site(1_469_598_103_934_665_603, quote_site), emit_site)
}

fn sanitize_name(name: &str) -> String {
    let out: String =
        name.bytes().map(|byte| if byte.is_ascii_alphanumeric() { byte as char } else { '_' }).collect();
    if out.is_empty() {
        "id".to_string()
    } else {
        out
    }
}

impl HygieneContext {
    fn emit_diag(&self, diag_id: &str) {
        if let (Some(diags), Some(diag)) = (&self.diags, make_diagnostic_by_id(diag_id, Some(self.diag_span.clone()))) {
            emit(&mut diags.borrow_mut(), diag);
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn lookup_name(&self, name: &str) -> Option<String> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name).cloned())
    }

    fn innermost_scope(&mut self) -> &mut HashMap<String, String> {
        if self.scopes.is_empty() {
            self.push_scope();
        }
        self.scopes.last_mut().expect("scope stack is not empty")
    }

    fn bind_preserved_name(&mut self, name: &str) {
        self.innermost_scope().insert(name.to_string(), name.to_string());
    }

    /// `hyg_<site hash in hex>_<seed>_<name>`, skipping names already in use. A clash
    /// with a spliced (user-chosen) name cannot be resolved and is an error.
    fn fresh_hygienic_name(&mut self, base: &str) -> Option<String> {
        let hash = site_hash(&self.quote_site, &self.emit_site);
        let sanitized = sanitize_name(base);
        loop {
            let candidate = format!("hyg_{hash:x}_{}_{sanitized}", self.next_seed);
            self.next_seed += 1;
            if self.unhygienic_names.contains(&candidate) {
                self.emit_diag("E-CTE-0241");
                self.ok = false;
                return None;
            }
            if self.all_names.insert(candidate.clone()) {
                return Some(candidate);
            }
        }
    }

    fn reserve_name(&mut self, name: &str, unhygienic: bool) {
        if name.is_empty() {
            return;
        }
        self.all_names.insert(name.to_string());
        if unhygienic {
            self.unhygienic_names.insert(name.to_string());
        }
    }

    // ---- collecting every name the syntax mentions ---------------------------

    fn collect_generic_params(&mut self, params_opt: &Option<GenericParams>) {
        for param in params_opt.iter().flat_map(|params| &params.params) {
            self.reserve_name(&param.name, false);
            self.collect_type(&param.default_type);
        }
    }

    fn collect_type_invariant(&mut self, invariant_opt: &Option<TypeInvariant>) {
        if let Some(invariant) = invariant_opt {
            self.collect_expr(&invariant.predicate);
        }
    }

    fn collect_loop_invariant(&mut self, invariant_opt: &Option<LoopInvariant>) {
        if let Some(invariant) = invariant_opt {
            self.collect_expr(&invariant.predicate);
        }
    }

    fn collect_args(&mut self, args: &[Arg]) {
        for arg in args {
            self.collect_expr(&arg.value);
        }
    }

    fn collect_field_inits(&mut self, fields: &[FieldInit]) {
        for field in fields {
            self.collect_expr(&field.value);
        }
    }

    fn collect_types(&mut self, types: &[TypePtr]) {
        for ty in types {
            self.collect_type(ty);
        }
    }

    fn collect_exprs(&mut self, exprs: &[ExprPtr]) {
        for expr in exprs {
            self.collect_expr(expr);
        }
    }

    fn collect_block_ptr(&mut self, block: &BlockPtr) {
        self.collect_block(block.as_deref().expect("block expected in quoted syntax"));
    }

    fn collect_expr(&mut self, expr: &ExprPtr) {
        let Some(expr) = expr else {
            return;
        };
        match &expr.node {
            ExprNode::IdentifierExpr(n) => self.reserve_name(&n.name, n.from_splice),
            ExprNode::BinaryExpr(n) => {
                self.collect_expr(&n.lhs);
                self.collect_expr(&n.rhs);
            }
            ExprNode::QualifiedApplyExpr(n) => match &n.args {
                ApplyArgs::ParenArgs(paren) => self.collect_args(&paren.args),
                ApplyArgs::BraceArgs(brace) => self.collect_field_inits(&brace.fields),
            },
            ExprNode::CallExpr(n) => {
                self.collect_expr(&n.callee);
                self.collect_args(&n.args);
                self.collect_types(&n.generic_args);
            }
            ExprNode::MethodCallExpr(n) => {
                self.collect_expr(&n.receiver);
                self.collect_args(&n.args);
            }
            ExprNode::RangeExpr(n) => {
                self.collect_expr(&n.lhs);
                self.collect_expr(&n.rhs);
            }
            ExprNode::CastExpr(n) => {
                self.collect_expr(&n.value);
                self.collect_type(&n.r#type);
            }
            ExprNode::DerefExpr(n) => self.collect_expr(&n.value),
            ExprNode::AddressOfExpr(n) => self.collect_expr(&n.place),
            ExprNode::MoveExpr(n) => self.collect_expr(&n.place),
            ExprNode::AllocExpr(n) => {
                if let Some(region) = &n.region_opt {
                    self.reserve_name(region, false);
                }
                self.collect_expr(&n.value);
            }
            ExprNode::TupleExpr(n) => self.collect_exprs(&n.elements),
            ExprNode::ArrayExpr(n) => {
                for segment in &n.elements {
                    match segment {
                        ArraySegment::ArrayElemSegment(elem) => self.collect_expr(&elem.value),
                        ArraySegment::ArrayRepeatSegment(repeat) => {
                            self.collect_expr(&repeat.value);
                            self.collect_expr(&repeat.count);
                        }
                    }
                }
            }
            ExprNode::ArrayRepeatExpr(n) => {
                self.collect_expr(&n.value);
                self.collect_expr(&n.count);
            }
            ExprNode::SizeofExpr(n) => self.collect_type(&n.r#type),
            ExprNode::AlignofExpr(n) => self.collect_type(&n.r#type),
            ExprNode::RecordExpr(n) => self.collect_field_inits(&n.fields),
            ExprNode::EnumLiteralExpr(n) => match &n.payload_opt {
                Some(EnumPayload::EnumPayloadParen(paren)) => self.collect_exprs(&paren.elements),
                Some(EnumPayload::EnumPayloadBrace(brace)) => self.collect_field_inits(&brace.fields),
                None => {}
            },
            ExprNode::IfExpr(n) => {
                self.collect_expr(&n.cond);
                self.collect_expr(&n.then_expr);
                self.collect_expr(&n.else_expr);
            }
            ExprNode::IfIsExpr(n) => {
                self.collect_expr(&n.scrutinee);
                self.collect_pattern(&n.pattern);
                self.collect_expr(&n.then_expr);
                self.collect_expr(&n.else_expr);
            }
            ExprNode::IfCaseExpr(n) => {
                self.collect_expr(&n.scrutinee);
                for clause in &n.cases {
                    self.collect_pattern(&clause.pattern);
                    self.collect_expr(&clause.body);
                }
                self.collect_expr(&n.else_expr);
            }
            ExprNode::LoopInfiniteExpr(n) => {
                self.collect_loop_invariant(&n.invariant_opt);
                self.collect_block_ptr(&n.body);
            }
            ExprNode::LoopConditionalExpr(n) => {
                self.collect_expr(&n.cond);
                self.collect_loop_invariant(&n.invariant_opt);
                self.collect_block_ptr(&n.body);
            }
            ExprNode::LoopIterExpr(n) => {
                self.collect_pattern(&n.pattern);
                self.collect_type(&n.type_opt);
                self.collect_expr(&n.iter);
                self.collect_loop_invariant(&n.invariant_opt);
                self.collect_block_ptr(&n.body);
            }
            ExprNode::BlockExpr(n) => self.collect_block_ptr(&n.block),
            ExprNode::UnsafeBlockExpr(n) => self.collect_block_ptr(&n.block),
            ExprNode::ComptimeExpr(n) => self.collect_expr(&n.body),
            ExprNode::CtIfExpr(n) => {
                self.collect_expr(&n.cond);
                self.collect_block_ptr(&n.then_block);
                if n.else_block_opt.is_some() {
                    self.collect_block_ptr(&n.else_block_opt);
                }
            }
            ExprNode::CtLoopIterExpr(n) => {
                self.collect_pattern(&n.pattern);
                self.collect_type(&n.type_opt);
                self.collect_expr(&n.iter);
                self.collect_block_ptr(&n.body);
            }
            ExprNode::AttributedExpr(n) => self.collect_expr(&n.expr),
            ExprNode::TransmuteExpr(n) => {
                self.collect_type(&n.from);
                self.collect_type(&n.to);
                self.collect_expr(&n.value);
            }
            ExprNode::ClosureExpr(n) => {
                for param in &n.params {
                    self.reserve_name(&param.name, false);
                    self.collect_type(&param.type_opt);
                }
                self.collect_type(&n.ret_type_opt);
                self.collect_expr(&n.body);
            }
            ExprNode::PipelineExpr(n) => {
                self.collect_expr(&n.lhs);
                self.collect_expr(&n.rhs);
            }
            ExprNode::FieldAccessExpr(n) => self.collect_expr(&n.base),
            ExprNode::TupleAccessExpr(n) => self.collect_expr(&n.base),
            ExprNode::IndexAccessExpr(n) => {
                self.collect_expr(&n.base);
                self.collect_expr(&n.index);
            }
            ExprNode::CallTypeArgsExpr(n) => {
                self.collect_expr(&n.callee);
                self.collect_types(&n.type_args);
                self.collect_args(&n.args);
            }
            ExprNode::UnaryExpr(n) => self.collect_expr(&n.value),
            ExprNode::PropagateExpr(n) => self.collect_expr(&n.value),
            ExprNode::YieldExpr(n) => self.collect_expr(&n.value),
            ExprNode::YieldFromExpr(n) => self.collect_expr(&n.value),
            ExprNode::SyncExpr(n) => self.collect_expr(&n.value),
            ExprNode::RaceExpr(n) => {
                for arm in &n.arms {
                    self.collect_pattern(&arm.pattern);
                    self.collect_expr(&arm.expr);
                    self.collect_expr(&arm.handler.value);
                }
            }
            ExprNode::AllExpr(n) => self.collect_exprs(&n.exprs),
            ExprNode::ParallelExpr(n) => {
                self.collect_expr(&n.domain);
                for opt in &n.opts {
                    self.collect_expr(&opt.value);
                }
                self.collect_block_ptr(&n.body);
            }
            ExprNode::SpawnExpr(n) => {
                for opt in &n.opts {
                    self.collect_expr(&opt.value);
                }
                self.collect_block_ptr(&n.body);
            }
            ExprNode::WaitExpr(n) => self.collect_expr(&n.handle),
            ExprNode::DispatchExpr(n) => {
                self.collect_pattern(&n.pattern);
                self.collect_expr(&n.range);
                self.collect_block_ptr(&n.body);
            }
            ExprNode::TypeLiteralExpr(n) => self.collect_type(&n.r#type),
            ExprNode::EntryExpr(n) => self.collect_expr(&n.expr),
            _ => {}
        }
    }

    fn collect_type(&mut self, ty: &TypePtr) {
        let Some(ty) = ty else {
            return;
        };
        match &ty.node {
            TypeNode::TypePermType(n) => self.collect_type(&n.base),
            TypeNode::TypeUnion(n) => self.collect_types(&n.types),
            TypeNode::TypeFunc(n) => {
                for param in &n.params {
                    self.collect_type(&param.r#type);
                }
                self.collect_type(&n.ret);
            }
            TypeNode::TypeClosure(n) => {
                for param in &n.params {
                    self.collect_type(&param.r#type);
                }
                self.collect_type(&n.ret);
                for dep in n.deps_opt.iter().flatten() {
                    self.collect_type(&dep.r#type);
                }
            }
            TypeNode::TypeTuple(n) => self.collect_types(&n.elements),
            TypeNode::TypeArray(n) => {
                self.collect_type(&n.element);
                self.collect_expr(&n.length);
            }
            TypeNode::TypeSlice(n) => self.collect_type(&n.element),
            TypeNode::TypeSafePtr(n) => self.collect_type(&n.element),
            TypeNode::TypeRawPtr(n) => self.collect_type(&n.element),
            TypeNode::TypeModalState(n) => self.collect_types(&n.generic_args),
            TypeNode::TypePathType(n) => self.collect_types(&n.generic_args),
            TypeNode::TypeApply(n) => self.collect_types(&n.args),
            TypeNode::TypeRefine(n) => {
                self.collect_type(&n.base);
                self.collect_expr(&n.predicate);
            }
            TypeNode::TypeRange(n) => self.collect_type(&n.base),
            TypeNode::TypeRangeInclusive(n) => self.collect_type(&n.base),
            TypeNode::TypeRangeFrom(n) => self.collect_type(&n.base),
            TypeNode::TypeRangeTo(n) => self.collect_type(&n.base),
            TypeNode::TypeRangeToInclusive(n) => self.collect_type(&n.base),
            _ => {}
        }
    }

    fn collect_field_patterns(&mut self, fields: &[FieldPattern]) {
        for field in fields {
            self.collect_pattern(&field.pattern_opt);
        }
    }

    fn collect_pattern(&mut self, pattern: &PatternPtr) {
        let Some(pattern) = pattern else {
            return;
        };
        match &pattern.node {
            PatternNode::IdentifierPattern(n) => self.reserve_name(&n.name, n.name_splice_opt.is_some()),
            PatternNode::TypedPattern(n) => {
                self.reserve_name(&n.name, n.name_splice_opt.is_some());
                self.collect_type(&n.r#type);
            }
            PatternNode::TuplePattern(n) => {
                for elem in &n.elements {
                    self.collect_pattern(elem);
                }
            }
            PatternNode::RecordPattern(n) => self.collect_field_patterns(&n.fields),
            PatternNode::EnumPattern(n) => match &n.payload_opt {
                Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                    for elem in &payload.elements {
                        self.collect_pattern(elem);
                    }
                }
                Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                    self.collect_field_patterns(&payload.fields);
                }
                None => {}
            },
            PatternNode::ModalPattern(n) => {
                if let Some(payload) = &n.fields_opt {
                    self.collect_field_patterns(&payload.fields);
                }
            }
            PatternNode::RangePattern(n) => {
                self.collect_pattern(&n.lo);
                self.collect_pattern(&n.hi);
            }
            _ => {}
        }
    }

    fn collect_binding(&mut self, binding: &Binding) {
        self.collect_pattern(&binding.pat);
        self.collect_type(&binding.type_opt);
        self.collect_expr(&binding.init);
    }

    fn collect_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(n) => self.collect_binding(&n.binding),
            Stmt::VarStmt(n) => self.collect_binding(&n.binding),
            Stmt::UsingLocalStmt(n) => self.reserve_name(&n.alias, n.alias_splice_opt.is_some()),
            Stmt::AssignStmt(n) => {
                self.collect_expr(&n.place);
                self.collect_expr(&n.value);
            }
            Stmt::CompoundAssignStmt(n) => {
                self.collect_expr(&n.place);
                self.collect_expr(&n.value);
            }
            Stmt::ExprStmt(n) => self.collect_expr(&n.value),
            Stmt::DeferStmt(n) => self.collect_block_ptr(&n.body),
            Stmt::UnsafeBlockStmt(n) => self.collect_block_ptr(&n.body),
            Stmt::CtStmt(n) => self.collect_block_ptr(&n.body),
            Stmt::KeyBlockStmt(n) => self.collect_block_ptr(&n.body),
            Stmt::RegionStmt(n) => {
                if let Some(alias) = &n.alias_opt {
                    self.reserve_name(alias, n.alias_splice_opt.is_some());
                }
                self.collect_expr(&n.opts_opt);
                self.collect_block_ptr(&n.body);
            }
            Stmt::FrameStmt(n) => {
                if let Some(target) = &n.target_opt {
                    self.reserve_name(target, false);
                }
                self.collect_block_ptr(&n.body);
            }
            Stmt::ReturnStmt(n) => self.collect_expr(&n.value_opt),
            Stmt::BreakStmt(n) => self.collect_expr(&n.value_opt),
            _ => {}
        }
    }

    fn collect_contract(&mut self, contract: &Option<ContractClause>) {
        if let Some(contract) = contract {
            self.collect_expr(&contract.precondition);
            self.collect_expr(&contract.postcondition);
        }
    }

    fn collect_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.collect_stmt(stmt);
        }
        self.collect_expr(&block.tail_opt);
    }

    fn collect_params(&mut self, params: &[Param]) {
        for param in params {
            self.reserve_name(&param.name, param.name_splice_opt.is_some());
            self.collect_type(&param.r#type);
        }
    }

    fn collect_method_like(
        &mut self,
        generic_params: &Option<GenericParams>,
        params: &[Param],
        return_type_opt: &TypePtr,
        contract: &Option<ContractClause>,
        body_opt: &BlockPtr,
    ) {
        self.collect_generic_params(generic_params);
        self.collect_params(params);
        self.collect_type(return_type_opt);
        self.collect_contract(contract);
        if let Some(body) = body_opt {
            self.collect_block(body);
        }
    }
}

impl HygieneContext {
    fn collect_using_aliases(&mut self, clause: &UsingClause) {
        match clause {
            UsingClause::UsingItem(entry) => {
                if let Some(alias) = &entry.alias_opt {
                    self.reserve_name(alias, false);
                }
            }
            UsingClause::UsingList(entry) => {
                for alias in entry.specs.iter().filter_map(|spec| spec.alias_opt.as_ref()) {
                    self.reserve_name(alias, false);
                }
            }
            UsingClause::UsingWildcard(_) => {}
        }
    }

    fn collect_item(&mut self, item: &ASTItem) {
        match item {
            ASTItem::UsingDecl(n) => self.collect_using_aliases(&n.clause),
            ASTItem::ImportDecl(n) => {
                if let Some(alias) = &n.alias_opt {
                    self.reserve_name(alias, false);
                }
            }
            ASTItem::ProcedureDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_method_like(&n.generic_params, &n.params, &n.return_type_opt, &n.contract, &n.body);
            }
            ASTItem::ComptimeProcedureDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_method_like(&n.generic_params, &n.params, &n.return_type_opt, &n.contract, &n.body);
            }
            ASTItem::ExternBlock(n) => {
                for ExternItem::ExternProcDecl(ext) in &n.items {
                    self.reserve_name(&ext.name, false);
                    self.collect_method_like(&ext.generic_params, &ext.params, &ext.return_type_opt, &ext.contract, &None);
                }
            }
            ASTItem::RecordDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_generic_params(&n.generic_params);
                self.collect_type_invariant(&n.invariant_opt);
                for member in &n.members {
                    match member {
                        RecordMember::FieldDecl(field) => {
                            self.collect_type(&field.r#type);
                            self.collect_expr(&field.init_opt);
                        }
                        RecordMember::MethodDecl(method) => self.collect_method_like(
                            &None,
                            &method.params,
                            &method.return_type_opt,
                            &method.contract,
                            &method.body,
                        ),
                        RecordMember::AssociatedTypeDecl(assoc) => self.collect_type(&assoc.default_type),
                    }
                }
            }
            ASTItem::EnumDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_generic_params(&n.generic_params);
                self.collect_type_invariant(&n.invariant_opt);
                for variant in &n.variants {
                    match &variant.payload_opt {
                        Some(VariantPayload::VariantPayloadTuple(payload)) => self.collect_types(&payload.elements),
                        Some(VariantPayload::VariantPayloadRecord(payload)) => {
                            for field in &payload.fields {
                                self.collect_type(&field.r#type);
                            }
                        }
                        None => {}
                    }
                }
            }
            ASTItem::ModalDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_generic_params(&n.generic_params);
                self.collect_type_invariant(&n.invariant_opt);
                for member in n.states.iter().flat_map(|state| &state.members) {
                    match member {
                        StateMember::StateFieldDecl(field) => self.collect_type(&field.r#type),
                        StateMember::StateMethodDecl(method) => self.collect_method_like(
                            &method.generic_params,
                            &method.params,
                            &method.return_type_opt,
                            &method.contract,
                            &method.body,
                        ),
                        StateMember::TransitionDecl(transition) => {
                            self.collect_params(&transition.params);
                            self.collect_block_ptr(&transition.body);
                        }
                    }
                }
            }
            ASTItem::ClassDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_generic_params(&n.generic_params);
                for class_item in &n.items {
                    match class_item {
                        ClassItem::ClassFieldDecl(field) => self.collect_type(&field.r#type),
                        ClassItem::AbstractFieldDecl(field) => self.collect_type(&field.r#type),
                        ClassItem::ClassMethodDecl(method) => self.collect_method_like(
                            &method.generic_params,
                            &method.params,
                            &method.return_type_opt,
                            &method.contract,
                            &method.body_opt,
                        ),
                        ClassItem::AssociatedTypeDecl(assoc) => self.collect_type(&assoc.default_type),
                        ClassItem::AbstractStateDecl(state) => {
                            for field in &state.fields {
                                self.collect_type(&field.r#type);
                            }
                        }
                    }
                }
            }
            ASTItem::StaticDecl(n) => self.collect_binding(&n.binding),
            ASTItem::TypeAliasDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_generic_params(&n.generic_params);
                self.collect_type(&n.r#type);
            }
            ASTItem::DeriveTargetDecl(n) => {
                self.reserve_name(&n.name, false);
                self.collect_block_ptr(&n.body);
            }
            ASTItem::ErrorItem(_) => {}
        }
    }

    // ---- renaming --------------------------------------------------------------
    //
    // Renaming rewrites nodes where they are, including nodes that other syntax also
    // points to (see `shared_node_mut`).

    fn rename_local_ref(&self, name_opt: &mut Option<Identifier>) {
        if let Some(mapped) = name_opt.as_ref().and_then(|name| self.lookup_name(name)) {
            *name_opt = Some(mapped);
        }
    }

    fn rename_path_head(&self, path: &mut [String]) {
        if let Some(mapped) = path.first().and_then(|head| self.lookup_name(head)) {
            path[0] = mapped;
        }
    }

    /// Introduces a binding: spliced names are kept, written names get a fresh name.
    fn bind_pattern_name(&mut self, name: &mut Identifier, unhygienic: bool) {
        if unhygienic {
            self.bind_preserved_name(name);
            return;
        }
        let Some(fresh) = self.fresh_hygienic_name(name) else {
            return;
        };
        self.innermost_scope().insert(name.clone(), fresh.clone());
        *name = fresh;
    }

    fn rename_using_aliases(&mut self, clause: &mut UsingClause) {
        let span = self.emit_site.span.clone();
        match clause {
            UsingClause::UsingItem(entry) => {
                if let Some(alias) = &mut entry.alias_opt {
                    uv_core::spec_rule_at!("requirement.22.ImportUsingHygiene", &span);
                    self.bind_pattern_name(alias, false);
                }
            }
            UsingClause::UsingList(entry) => {
                for alias in entry.specs.iter_mut().filter_map(|spec| spec.alias_opt.as_mut()) {
                    uv_core::spec_rule_at!("requirement.22.ImportUsingHygiene", &span);
                    self.bind_pattern_name(alias, false);
                }
            }
            UsingClause::UsingWildcard(_) => {}
        }
    }

    fn rename_field_patterns(&mut self, fields: &mut [FieldPattern]) {
        for field in fields {
            self.rename_pattern(&mut field.pattern_opt);
        }
    }

    fn rename_pattern(&mut self, pattern: &mut PatternPtr) {
        let Some(pattern) = pattern else {
            return;
        };
        if !self.ok {
            return;
        }
        match &mut shared_node_mut(pattern).node {
            PatternNode::IdentifierPattern(n) => {
                let unhygienic = n.name_splice_opt.is_some();
                self.bind_pattern_name(&mut n.name, unhygienic);
            }
            PatternNode::TypedPattern(n) => {
                self.rename_type(&mut n.r#type);
                let unhygienic = n.name_splice_opt.is_some();
                self.bind_pattern_name(&mut n.name, unhygienic);
            }
            PatternNode::TuplePattern(n) => {
                for elem in &mut n.elements {
                    self.rename_pattern(elem);
                }
            }
            PatternNode::RecordPattern(n) => {
                self.rename_path_head(&mut n.path);
                self.rename_field_patterns(&mut n.fields);
            }
            PatternNode::EnumPattern(n) => {
                self.rename_path_head(&mut n.path);
                match &mut n.payload_opt {
                    Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                        for elem in &mut payload.elements {
                            self.rename_pattern(elem);
                        }
                    }
                    Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                        self.rename_field_patterns(&mut payload.fields);
                    }
                    None => {}
                }
            }
            PatternNode::ModalPattern(n) => {
                if let Some(payload) = &mut n.fields_opt {
                    self.rename_field_patterns(&mut payload.fields);
                }
            }
            PatternNode::RangePattern(n) => {
                self.rename_pattern(&mut n.lo);
                self.rename_pattern(&mut n.hi);
            }
            _ => {}
        }
    }

    fn rename_types(&mut self, types: &mut [TypePtr]) {
        for ty in types {
            self.rename_type(ty);
        }
    }

    fn rename_type(&mut self, ty: &mut TypePtr) {
        let Some(ty) = ty else {
            return;
        };
        if !self.ok {
            return;
        }
        match &mut shared_node_mut(ty).node {
            TypeNode::TypePermType(n) => self.rename_type(&mut n.base),
            TypeNode::TypeUnion(n) => self.rename_types(&mut n.types),
            TypeNode::TypeFunc(n) => {
                for param in &mut n.params {
                    self.rename_type(&mut param.r#type);
                }
                self.rename_type(&mut n.ret);
            }
            TypeNode::TypeClosure(n) => {
                for param in &mut n.params {
                    self.rename_type(&mut param.r#type);
                }
                self.rename_type(&mut n.ret);
                for dep in n.deps_opt.iter_mut().flatten() {
                    self.rename_type(&mut dep.r#type);
                }
            }
            TypeNode::TypeTuple(n) => self.rename_types(&mut n.elements),
            TypeNode::TypeArray(n) => {
                self.rename_type(&mut n.element);
                self.rename_expr(&mut n.length);
            }
            TypeNode::TypeSlice(n) => self.rename_type(&mut n.element),
            TypeNode::TypeSafePtr(n) => self.rename_type(&mut n.element),
            TypeNode::TypeRawPtr(n) => self.rename_type(&mut n.element),
            TypeNode::TypeDynamic(n) => self.rename_path_head(&mut n.path),
            TypeNode::TypeModalState(n) => {
                self.rename_path_head(&mut n.path);
                self.rename_types(&mut n.generic_args);
            }
            TypeNode::TypePathType(n) => {
                self.rename_path_head(&mut n.path);
                self.rename_types(&mut n.generic_args);
            }
            TypeNode::TypeApply(n) => {
                self.rename_path_head(&mut n.path);
                self.rename_types(&mut n.args);
            }
            TypeNode::TypeOpaque(n) => self.rename_path_head(&mut n.path),
            TypeNode::TypeRefine(n) => {
                self.rename_type(&mut n.base);
                self.rename_expr(&mut n.predicate);
            }
            TypeNode::TypeRange(n) => self.rename_type(&mut n.base),
            TypeNode::TypeRangeInclusive(n) => self.rename_type(&mut n.base),
            TypeNode::TypeRangeFrom(n) => self.rename_type(&mut n.base),
            TypeNode::TypeRangeTo(n) => self.rename_type(&mut n.base),
            TypeNode::TypeRangeToInclusive(n) => self.rename_type(&mut n.base),
            _ => {}
        }
    }

    fn rename_args(&mut self, args: &mut [Arg]) {
        for arg in args {
            self.rename_expr(&mut arg.value);
        }
    }

    fn rename_field_inits(&mut self, fields: &mut [FieldInit]) {
        for field in fields {
            self.rename_expr(&mut field.value);
        }
    }

    fn rename_exprs(&mut self, exprs: &mut [ExprPtr]) {
        for expr in exprs {
            self.rename_expr(expr);
        }
    }

    fn rename_loop_invariant(&mut self, invariant_opt: &mut Option<LoopInvariant>) {
        if let Some(invariant) = invariant_opt {
            self.rename_expr(&mut invariant.predicate);
        }
    }

    fn rename_block_ptr(&mut self, block: &mut BlockPtr) {
        let block = block.as_mut().expect("block expected in quoted syntax");
        self.rename_block(shared_node_mut(block));
    }

    /// Diagnostics raised while renaming inside an expression are reported at the
    /// insertion site, not at the quoted syntax.
    fn rename_expr(&mut self, expr: &mut ExprPtr) {
        let Some(expr) = expr else {
            return;
        };
        if !self.ok {
            return;
        }
        let saved_span = std::mem::replace(&mut self.diag_span, self.emit_site.span.clone());
        self.rename_expr_node(&mut shared_node_mut(expr).node);
        self.diag_span = saved_span;
    }

    fn rename_expr_node(&mut self, node: &mut ExprNode) {
        match node {
            ExprNode::IdentifierExpr(n) => {
                if !n.from_splice {
                    if let Some(mapped) = self.lookup_name(&n.name) {
                        n.name = mapped;
                    }
                }
            }
            ExprNode::QualifiedNameExpr(n) => self.rename_path_head(&mut n.path),
            ExprNode::PathExpr(n) => self.rename_path_head(&mut n.path),
            ExprNode::BinaryExpr(n) => {
                self.rename_expr(&mut n.lhs);
                self.rename_expr(&mut n.rhs);
            }
            ExprNode::QualifiedApplyExpr(n) => {
                self.rename_path_head(&mut n.path);
                match &mut n.args {
                    ApplyArgs::ParenArgs(paren) => self.rename_args(&mut paren.args),
                    ApplyArgs::BraceArgs(brace) => self.rename_field_inits(&mut brace.fields),
                }
            }
            ExprNode::CallExpr(n) => {
                self.rename_expr(&mut n.callee);
                self.rename_types(&mut n.generic_args);
                self.rename_args(&mut n.args);
            }
            ExprNode::MethodCallExpr(n) => {
                self.rename_expr(&mut n.receiver);
                self.rename_args(&mut n.args);
            }
            ExprNode::RangeExpr(n) => {
                self.rename_expr(&mut n.lhs);
                self.rename_expr(&mut n.rhs);
            }
            ExprNode::CastExpr(n) => {
                self.rename_expr(&mut n.value);
                self.rename_type(&mut n.r#type);
            }
            ExprNode::DerefExpr(n) => self.rename_expr(&mut n.value),
            ExprNode::AddressOfExpr(n) => self.rename_expr(&mut n.place),
            ExprNode::MoveExpr(n) => self.rename_expr(&mut n.place),
            ExprNode::AllocExpr(n) => {
                self.rename_local_ref(&mut n.region_opt);
                self.rename_expr(&mut n.value);
            }
            ExprNode::TupleExpr(n) => self.rename_exprs(&mut n.elements),
            ExprNode::ArrayExpr(n) => {
                for segment in &mut n.elements {
                    match segment {
                        ArraySegment::ArrayElemSegment(elem) => self.rename_expr(&mut elem.value),
                        ArraySegment::ArrayRepeatSegment(repeat) => {
                            self.rename_expr(&mut repeat.value);
                            self.rename_expr(&mut repeat.count);
                        }
                    }
                }
            }
            ExprNode::ArrayRepeatExpr(n) => {
                self.rename_expr(&mut n.value);
                self.rename_expr(&mut n.count);
            }
            ExprNode::SizeofExpr(n) => self.rename_type(&mut n.r#type),
            ExprNode::AlignofExpr(n) => self.rename_type(&mut n.r#type),
            ExprNode::RecordExpr(n) => {
                match &mut n.target {
                    RecordExprTarget::Path(path) => self.rename_path_head(path),
                    RecordExprTarget::ModalStateRef(target) => {
                        self.rename_path_head(&mut target.path);
                        self.rename_types(&mut target.generic_args);
                    }
                }
                self.rename_field_inits(&mut n.fields);
            }
            ExprNode::EnumLiteralExpr(n) => {
                self.rename_path_head(&mut n.path);
                match &mut n.payload_opt {
                    Some(EnumPayload::EnumPayloadParen(paren)) => self.rename_exprs(&mut paren.elements),
                    Some(EnumPayload::EnumPayloadBrace(brace)) => self.rename_field_inits(&mut brace.fields),
                    None => {}
                }
            }
            ExprNode::IfExpr(n) => {
                self.rename_expr(&mut n.cond);
                self.rename_expr(&mut n.then_expr);
                self.rename_expr(&mut n.else_expr);
            }
            ExprNode::IfIsExpr(n) => {
                self.rename_expr(&mut n.scrutinee);
                self.push_scope();
                self.rename_pattern(&mut n.pattern);
                self.rename_expr(&mut n.then_expr);
                self.pop_scope();
                self.rename_expr(&mut n.else_expr);
            }
            ExprNode::IfCaseExpr(n) => {
                self.rename_expr(&mut n.scrutinee);
                for clause in &mut n.cases {
                    self.push_scope();
                    self.rename_pattern(&mut clause.pattern);
                    self.rename_expr(&mut clause.body);
                    self.pop_scope();
                }
                self.rename_expr(&mut n.else_expr);
            }
            ExprNode::LoopInfiniteExpr(n) => {
                self.rename_loop_invariant(&mut n.invariant_opt);
                self.rename_block_ptr(&mut n.body);
            }
            ExprNode::LoopConditionalExpr(n) => {
                self.rename_expr(&mut n.cond);
                self.rename_loop_invariant(&mut n.invariant_opt);
                self.rename_block_ptr(&mut n.body);
            }
            ExprNode::LoopIterExpr(n) => {
                self.rename_type(&mut n.type_opt);
                self.rename_expr(&mut n.iter);
                self.rename_loop_invariant(&mut n.invariant_opt);
                self.push_scope();
                self.rename_pattern(&mut n.pattern);
                self.rename_block_ptr(&mut n.body);
                self.pop_scope();
            }
            ExprNode::BlockExpr(n) => self.rename_block_ptr(&mut n.block),
            ExprNode::UnsafeBlockExpr(n) => self.rename_block_ptr(&mut n.block),
            ExprNode::ComptimeExpr(n) => self.rename_expr(&mut n.body),
            ExprNode::CtIfExpr(n) => {
                self.rename_expr(&mut n.cond);
                self.rename_block_ptr(&mut n.then_block);
                if n.else_block_opt.is_some() {
                    self.rename_block_ptr(&mut n.else_block_opt);
                }
            }
            ExprNode::CtLoopIterExpr(n) => {
                self.rename_type(&mut n.type_opt);
                self.rename_expr(&mut n.iter);
                self.push_scope();
                self.rename_pattern(&mut n.pattern);
                self.rename_block_ptr(&mut n.body);
                self.pop_scope();
            }
            ExprNode::AttributedExpr(n) => self.rename_expr(&mut n.expr),
            ExprNode::EntryExpr(n) => self.rename_expr(&mut n.expr),
            ExprNode::TypeLiteralExpr(n) => self.rename_type(&mut n.r#type),
            ExprNode::TransmuteExpr(n) => {
                self.rename_type(&mut n.from);
                self.rename_type(&mut n.to);
                self.rename_expr(&mut n.value);
            }
            ExprNode::ClosureExpr(n) => {
                for param in &mut n.params {
                    self.rename_type(&mut param.type_opt);
                }
                self.rename_type(&mut n.ret_type_opt);
                self.push_scope();
                for param in &mut n.params {
                    self.bind_pattern_name(&mut param.name, false);
                }
                self.rename_expr(&mut n.body);
                self.pop_scope();
            }
            ExprNode::PipelineExpr(n) => {
                self.rename_expr(&mut n.lhs);
                self.rename_expr(&mut n.rhs);
            }
            ExprNode::FieldAccessExpr(n) => self.rename_expr(&mut n.base),
            ExprNode::TupleAccessExpr(n) => self.rename_expr(&mut n.base),
            ExprNode::IndexAccessExpr(n) => {
                self.rename_expr(&mut n.base);
                self.rename_expr(&mut n.index);
            }
            ExprNode::CallTypeArgsExpr(n) => {
                self.rename_expr(&mut n.callee);
                self.rename_types(&mut n.type_args);
                self.rename_args(&mut n.args);
            }
            ExprNode::UnaryExpr(n) => self.rename_expr(&mut n.value),
            ExprNode::PropagateExpr(n) => self.rename_expr(&mut n.value),
            ExprNode::YieldExpr(n) => self.rename_expr(&mut n.value),
            ExprNode::YieldFromExpr(n) => self.rename_expr(&mut n.value),
            ExprNode::SyncExpr(n) => self.rename_expr(&mut n.value),
            ExprNode::RaceExpr(n) => {
                for arm in &mut n.arms {
                    self.rename_expr(&mut arm.expr);
                    self.push_scope();
                    self.rename_pattern(&mut arm.pattern);
                    self.rename_expr(&mut arm.handler.value);
                    self.pop_scope();
                }
            }
            ExprNode::AllExpr(n) => self.rename_exprs(&mut n.exprs),
            ExprNode::ParallelExpr(n) => {
                self.rename_expr(&mut n.domain);
                for opt in &mut n.opts {
                    self.rename_expr(&mut opt.value);
                }
                self.rename_block_ptr(&mut n.body);
            }
            ExprNode::SpawnExpr(n) => {
                for opt in &mut n.opts {
                    self.rename_expr(&mut opt.value);
                }
                self.rename_block_ptr(&mut n.body);
            }
            ExprNode::WaitExpr(n) => self.rename_expr(&mut n.handle),
            ExprNode::DispatchExpr(n) => {
                self.rename_pattern(&mut n.pattern);
                self.rename_expr(&mut n.range);
                self.rename_block_ptr(&mut n.body);
            }
            _ => {}
        }
    }

    fn rename_binding(&mut self, binding: &mut Binding) {
        self.rename_type(&mut binding.type_opt);
        self.rename_expr(&mut binding.init);
        self.rename_pattern(&mut binding.pat);
    }

    fn rename_stmt(&mut self, stmt: &mut Stmt) {
        if !self.ok {
            return;
        }
        match stmt {
            Stmt::LetStmt(n) => self.rename_binding(&mut n.binding),
            Stmt::VarStmt(n) => self.rename_binding(&mut n.binding),
            Stmt::UsingLocalStmt(n) => {
                let unhygienic = n.alias_splice_opt.is_some();
                self.bind_pattern_name(&mut n.alias, unhygienic);
            }
            Stmt::AssignStmt(n) => {
                self.rename_expr(&mut n.place);
                self.rename_expr(&mut n.value);
            }
            Stmt::CompoundAssignStmt(n) => {
                self.rename_expr(&mut n.place);
                self.rename_expr(&mut n.value);
            }
            Stmt::ExprStmt(n) => self.rename_expr(&mut n.value),
            Stmt::DeferStmt(n) => self.rename_block_ptr(&mut n.body),
            Stmt::UnsafeBlockStmt(n) => self.rename_block_ptr(&mut n.body),
            Stmt::CtStmt(n) => self.rename_block_ptr(&mut n.body),
            Stmt::KeyBlockStmt(n) => self.rename_block_ptr(&mut n.body),
            Stmt::RegionStmt(n) => {
                self.rename_expr(&mut n.opts_opt);
                self.push_scope();
                let unhygienic = n.alias_splice_opt.is_some();
                if let Some(alias) = &mut n.alias_opt {
                    self.bind_pattern_name(alias, unhygienic);
                }
                self.rename_block_ptr(&mut n.body);
                self.pop_scope();
            }
            Stmt::FrameStmt(n) => {
                self.rename_local_ref(&mut n.target_opt);
                self.rename_block_ptr(&mut n.body);
            }
            Stmt::ReturnStmt(n) => self.rename_expr(&mut n.value_opt),
            Stmt::BreakStmt(n) => self.rename_expr(&mut n.value_opt),
            _ => {}
        }
    }

    fn rename_block(&mut self, block: &mut Block) {
        if !self.ok {
            return;
        }
        self.push_scope();
        for stmt in &mut block.stmts {
            self.rename_stmt(stmt);
        }
        self.rename_expr(&mut block.tail_opt);
        self.pop_scope();
    }

    fn rename_contract(&mut self, contract: &mut Option<ContractClause>) {
        let Some(contract) = contract else {
            return;
        };
        if !self.ok {
            return;
        }
        self.rename_expr(&mut contract.precondition);
        self.rename_expr(&mut contract.postcondition);
    }

    fn rename_generic_params(&mut self, params_opt: &mut Option<GenericParams>) {
        let Some(params) = params_opt else {
            return;
        };
        if !self.ok {
            return;
        }
        for param in &mut params.params {
            self.rename_type(&mut param.default_type);
            self.bind_pattern_name(&mut param.name, false);
        }
    }

    fn rename_type_invariant(&mut self, invariant_opt: &mut Option<TypeInvariant>) {
        let Some(invariant) = invariant_opt else {
            return;
        };
        if !self.ok {
            return;
        }
        self.rename_expr(&mut invariant.predicate);
    }

    fn bind_params(&mut self, params: &mut [Param]) {
        for param in params.iter_mut() {
            self.rename_type(&mut param.r#type);
        }
        for param in params.iter_mut() {
            let unhygienic = param.name_splice_opt.is_some();
            self.bind_pattern_name(&mut param.name, unhygienic);
        }
    }

    fn rename_method_like(
        &mut self,
        generic_params: Option<&mut Option<GenericParams>>,
        params: &mut [Param],
        return_type_opt: &mut TypePtr,
        contract: &mut Option<ContractClause>,
        body_opt: Option<&mut BlockPtr>,
    ) {
        self.push_scope();
        if let Some(generic_params) = generic_params {
            self.rename_generic_params(generic_params);
        }
        self.rename_type(return_type_opt);
        self.bind_params(params);
        self.rename_contract(contract);
        if let Some(body) = body_opt.filter(|body| body.is_some()) {
            self.rename_block_ptr(body);
        }
        self.pop_scope();
    }

    fn rename_item(&mut self, item: &mut ASTItem) {
        if !self.ok {
            return;
        }
        match item {
            ASTItem::UsingDecl(n) => self.rename_using_aliases(&mut n.clause),
            ASTItem::ImportDecl(n) => {
                if let Some(alias) = &mut n.alias_opt {
                    uv_core::spec_rule_at!("requirement.22.ImportUsingHygiene", &self.emit_site.span);
                    self.bind_pattern_name(alias, false);
                }
            }
            ASTItem::ProcedureDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.rename_method_like(
                    Some(&mut n.generic_params),
                    &mut n.params,
                    &mut n.return_type_opt,
                    &mut n.contract,
                    Some(&mut n.body),
                );
            }
            ASTItem::ComptimeProcedureDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.rename_method_like(
                    Some(&mut n.generic_params),
                    &mut n.params,
                    &mut n.return_type_opt,
                    &mut n.contract,
                    Some(&mut n.body),
                );
            }
            ASTItem::ExternBlock(n) => {
                for ExternItem::ExternProcDecl(ext) in &mut n.items {
                    self.bind_preserved_name(&ext.name);
                    self.rename_method_like(
                        Some(&mut ext.generic_params),
                        &mut ext.params,
                        &mut ext.return_type_opt,
                        &mut ext.contract,
                        None,
                    );
                }
            }
            ASTItem::RecordDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.push_scope();
                self.rename_generic_params(&mut n.generic_params);
                self.rename_type_invariant(&mut n.invariant_opt);
                for member in &mut n.members {
                    match member {
                        RecordMember::FieldDecl(field) => {
                            self.rename_type(&mut field.r#type);
                            self.rename_expr(&mut field.init_opt);
                        }
                        RecordMember::MethodDecl(method) => self.rename_method_like(
                            None,
                            &mut method.params,
                            &mut method.return_type_opt,
                            &mut method.contract,
                            Some(&mut method.body),
                        ),
                        RecordMember::AssociatedTypeDecl(assoc) => self.rename_type(&mut assoc.default_type),
                    }
                }
                self.pop_scope();
            }
            ASTItem::EnumDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.push_scope();
                self.rename_generic_params(&mut n.generic_params);
                self.rename_type_invariant(&mut n.invariant_opt);
                for variant in &mut n.variants {
                    match &mut variant.payload_opt {
                        Some(VariantPayload::VariantPayloadTuple(payload)) => self.rename_types(&mut payload.elements),
                        Some(VariantPayload::VariantPayloadRecord(payload)) => {
                            for field in &mut payload.fields {
                                self.rename_type(&mut field.r#type);
                            }
                        }
                        None => {}
                    }
                }
                self.pop_scope();
            }
            ASTItem::ModalDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.push_scope();
                self.rename_generic_params(&mut n.generic_params);
                self.rename_type_invariant(&mut n.invariant_opt);
                for member in n.states.iter_mut().flat_map(|state| &mut state.members) {
                    match member {
                        StateMember::StateFieldDecl(field) => self.rename_type(&mut field.r#type),
                        StateMember::StateMethodDecl(method) => self.rename_method_like(
                            Some(&mut method.generic_params),
                            &mut method.params,
                            &mut method.return_type_opt,
                            &mut method.contract,
                            Some(&mut method.body),
                        ),
                        StateMember::TransitionDecl(transition) => {
                            self.push_scope();
                            self.bind_params(&mut transition.params);
                            self.rename_block_ptr(&mut transition.body);
                            self.pop_scope();
                        }
                    }
                }
                self.pop_scope();
            }
            ASTItem::ClassDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.push_scope();
                self.rename_generic_params(&mut n.generic_params);
                for class_item in &mut n.items {
                    match class_item {
                        ClassItem::ClassFieldDecl(field) => self.rename_type(&mut field.r#type),
                        ClassItem::AbstractFieldDecl(field) => self.rename_type(&mut field.r#type),
                        ClassItem::ClassMethodDecl(method) => self.rename_method_like(
                            Some(&mut method.generic_params),
                            &mut method.params,
                            &mut method.return_type_opt,
                            &mut method.contract,
                            Some(&mut method.body_opt),
                        ),
                        ClassItem::AssociatedTypeDecl(assoc) => self.rename_type(&mut assoc.default_type),
                        ClassItem::AbstractStateDecl(state) => {
                            for field in &mut state.fields {
                                self.rename_type(&mut field.r#type);
                            }
                        }
                    }
                }
                self.pop_scope();
            }
            ASTItem::StaticDecl(n) => {
                self.rename_type(&mut n.binding.type_opt);
                self.rename_expr(&mut n.binding.init);
                self.push_scope();
                self.rename_pattern(&mut n.binding.pat);
                self.pop_scope();
            }
            ASTItem::TypeAliasDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.push_scope();
                self.rename_generic_params(&mut n.generic_params);
                self.rename_type(&mut n.r#type);
                self.pop_scope();
            }
            ASTItem::DeriveTargetDecl(n) => {
                self.bind_preserved_name(&n.name);
                self.rename_block_ptr(&mut n.body);
            }
            ASTItem::ErrorItem(_) => {}
        }
    }
}

/// Renames the bindings of quoted syntax for insertion at `emit_site`. Returns the renamed
/// syntax and the next unused seed; `None` when a fresh name collides with a spliced one.
pub fn hygienize_ast(
    ast: &CtAst,
    quote_site: &CtSite,
    emit_site: &CtSite,
    seed: usize,
    env: &CtEnv,
) -> (Option<CtAst>, usize) {
    let mut ctx = HygieneContext {
        diags: env.diags.clone(),
        quote_site: quote_site.clone(),
        emit_site: emit_site.clone(),
        next_seed: seed,
        ok: true,
        all_names: HashSet::new(),
        unhygienic_names: HashSet::new(),
        scopes: Vec::new(),
        diag_span: ast.span.clone().unwrap_or_else(|| emit_site.span.clone()),
    };
    match &ast.payload {
        CtAstPayload::Expr(expr) => ctx.collect_expr(expr),
        CtAstPayload::Stmt(stmt) => ctx.collect_stmt(stmt),
        CtAstPayload::Item(item) => ctx.collect_item(item),
        CtAstPayload::Type(ty) => ctx.collect_type(ty),
        CtAstPayload::Pattern(pattern) => ctx.collect_pattern(pattern),
    }
    let mut out = ast.clone();
    out.hygiene =
        Some(Box::new(CtHygiene { quote_site: quote_site.clone(), emit_site: emit_site.clone(), mark: seed }));
    ctx.push_scope();
    match &mut out.payload {
        CtAstPayload::Expr(expr) => ctx.rename_expr(expr),
        CtAstPayload::Stmt(stmt) => ctx.rename_stmt(stmt),
        CtAstPayload::Item(item) => ctx.rename_item(item),
        CtAstPayload::Type(ty) => ctx.rename_type(ty),
        CtAstPayload::Pattern(pattern) => ctx.rename_pattern(pattern),
    }
    ctx.pop_scope();
    if !ctx.ok {
        return (None, ctx.next_seed);
    }
    uv_core::spec_rule_at!("requirement.22.HygienizeAstProperties", &emit_site.span);
    uv_core::spec_rule_at!("requirement.22.HygienicInternalReferences", &emit_site.span);
    (Some(out), ctx.next_seed)
}

pub fn prepare_ast_for_insertion(ast: &CtAst, emit_site: &CtSite, env: &mut CtEnv) -> Option<CtAst> {
    let quote_site = ast.hygiene.as_ref().map_or_else(|| env.site.clone(), |hygiene| hygiene.quote_site.clone());
    let (hygienized, next_seed) = hygienize_ast(ast, &quote_site, emit_site, env.next_hygiene, env);
    env.next_hygiene = next_seed;
    hygienized
}
