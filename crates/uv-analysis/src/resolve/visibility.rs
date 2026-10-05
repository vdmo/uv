//! Visibility: who may name a declaration, and the per-module visibility checks.

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::span::Span;
use uv_core::spec_rule;
use uv_source::ast::*;

use super::scopes::{id_key_of, path_eq, path_key_of};
use super::scopes_lookup::{find_context_module_by_path, AccessResult};
use crate::context::{IdKey, ScopeContext};

const ALLOWED: AccessResult = AccessResult { ok: true, diag_id: None };
const DENIED: AccessResult = AccessResult { ok: false, diag_id: Some("Access-Err") };

fn same_assembly(m1: &[String], m2: &[String]) -> bool {
    matches!((m1.first(), m2.first()), (Some(a), Some(b)) if a == b)
}

/// The visibility an item has for access checks; derive targets are assembly-internal.
pub(super) fn vis_opt(item: &ASTItem) -> Option<Visibility> {
    match item {
        ASTItem::UsingDecl(d) => Some(d.vis),
        ASTItem::ImportDecl(d) => Some(d.vis),
        ASTItem::ExternBlock(d) => Some(d.vis),
        ASTItem::StaticDecl(d) => Some(d.vis),
        ASTItem::ProcedureDecl(d) => Some(d.vis),
        ASTItem::ComptimeProcedureDecl(d) => Some(d.vis),
        ASTItem::RecordDecl(d) => Some(d.vis),
        ASTItem::EnumDecl(d) => Some(d.vis),
        ASTItem::ModalDecl(d) => Some(d.vis),
        ASTItem::ClassDecl(d) => Some(d.vis),
        ASTItem::TypeAliasDecl(d) => Some(d.vis),
        ASTItem::DeriveTargetDecl(_) => Some(Visibility::Internal),
        ASTItem::ErrorItem(_) => None,
    }
}

pub(super) fn name_matches(key: &IdKey, candidate: &str) -> bool {
    *key == id_key_of(candidate)
}

fn record_fields_bind_name(fields: &[FieldPattern], key: &IdKey) -> bool {
    fields.iter().any(|field| match &field.pattern_opt {
        Some(pattern) => pattern_binds_name(pattern, key),
        None => name_matches(key, &field.name),
    })
}

pub(super) fn pattern_ptr_binds_name(pattern: &PatternPtr, key: &IdKey) -> bool {
    pattern.as_deref().is_some_and(|pattern| pattern_binds_name(pattern, key))
}

fn pattern_binds_name(pattern: &Pattern, key: &IdKey) -> bool {
    match &pattern.node {
        PatternNode::IdentifierPattern(node) => name_matches(key, &node.name),
        PatternNode::TypedPattern(node) => node.name != "_" && name_matches(key, &node.name),
        PatternNode::TuplePattern(node) => node.elements.iter().any(|elem| pattern_ptr_binds_name(elem, key)),
        PatternNode::RecordPattern(node) => record_fields_bind_name(&node.fields, key),
        PatternNode::EnumPattern(node) => match &node.payload_opt {
            None => false,
            Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                payload.elements.iter().any(|elem| pattern_ptr_binds_name(elem, key))
            }
            Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => record_fields_bind_name(&payload.fields, key),
        },
        PatternNode::ModalPattern(node) => {
            node.fields_opt.as_ref().is_some_and(|payload| record_fields_bind_name(&payload.fields, key))
        }
        PatternNode::RangePattern(node) => {
            pattern_ptr_binds_name(&node.lo, key) || pattern_ptr_binds_name(&node.hi, key)
        }
        _ => false,
    }
}

pub(super) fn using_clause_binds_name(clause: &UsingClause, key: &IdKey) -> bool {
    match clause {
        UsingClause::UsingItem(node) => name_matches(key, node.alias_opt.as_ref().unwrap_or(&node.name)),
        UsingClause::UsingWildcard(_) => true,
        UsingClause::UsingList(node) => {
            node.specs.iter().any(|spec| name_matches(key, spec.alias_opt.as_ref().unwrap_or(&spec.name)))
        }
    }
}

fn item_binds_name(item: &ASTItem, key: &IdKey) -> bool {
    match item {
        ASTItem::UsingDecl(it) => using_clause_binds_name(&it.clause, key),
        ASTItem::ExternBlock(it) => {
            it.items.iter().any(|ExternItem::ExternProcDecl(proc)| name_matches(key, &proc.name))
        }
        ASTItem::StaticDecl(it) => pattern_ptr_binds_name(&it.binding.pat, key),
        ASTItem::ProcedureDecl(it) => name_matches(key, &it.name),
        ASTItem::ComptimeProcedureDecl(it) => name_matches(key, &it.name),
        ASTItem::DeriveTargetDecl(it) => name_matches(key, &it.name),
        ASTItem::RecordDecl(it) => name_matches(key, &it.name),
        ASTItem::EnumDecl(it) => name_matches(key, &it.name),
        ASTItem::ModalDecl(it) => name_matches(key, &it.name),
        ASTItem::ClassDecl(it) => name_matches(key, &it.name),
        ASTItem::TypeAliasDecl(it) => name_matches(key, &it.name),
        ASTItem::ImportDecl(_) | ASTItem::ErrorItem(_) => false,
    }
}

/// The visibility of the first declaration in a module that binds a name. An extern
/// procedure carries its own visibility, not its block's.
fn find_decl_visibility_by_name(ctx: &ScopeContext<'_>, module_path: &[String], name: &str) -> Option<Visibility> {
    let module = find_context_module_by_path(ctx, module_path)?;
    let key = id_key_of(name);
    for item in &module.items {
        if let ASTItem::ExternBlock(block) = item {
            let proc = block.items.iter().map(|ExternItem::ExternProcDecl(proc)| proc);
            if let Some(proc) = proc.into_iter().find(|proc| name_matches(&key, &proc.name)) {
                return Some(proc.vis);
            }
        }
        if item_binds_name(item, &key) {
            return vis_opt(item);
        }
    }
    None
}

fn has_module_path(ctx: &ScopeContext<'_>, path: &[String]) -> bool {
    find_context_module_by_path(ctx, path).is_some()
}

fn resolve_visible_module_path(ctx: &ScopeContext<'_>, module_path: &[String]) -> Option<Vec<String>> {
    if module_path.is_empty() {
        return None;
    }
    if has_module_path(ctx, module_path) {
        return Some(module_path.to_vec());
    }
    let mut candidate = vec![ctx.current_module.first()?.clone()];
    candidate.extend(module_path.iter().cloned());
    has_module_path(ctx, &candidate).then_some(candidate)
}

fn can_access_from_name_map(ctx: &ScopeContext<'_>, module_path: &[String], name: &str) -> Option<AccessResult> {
    let name_maps = ctx.name_resolution_tables.as_ref()?.name_maps?;
    let visibility = name_maps.get(&path_key_of(module_path))?.get(&id_key_of(name))?.visibility?;
    Some(can_access_vis(&ctx.current_module, module_path, visibility))
}

fn emit_diag(diags: &mut DiagnosticStream, code: &str, span: &Span) {
    if let Some(diag) = make_diagnostic_by_id(code, Some(span.clone())) {
        emit(diags, diag);
    }
}

/// Reports every qualified name in a module's code that the module may not access.
struct AccessChecker<'c, 'a> {
    ctx: &'c ScopeContext<'a>,
    diags: DiagnosticStream,
}

impl AccessChecker<'_, '_> {
    fn check_access(&mut self, path: &[String], name: &str, span: &Span) {
        if can_access(self.ctx, path, name) == DENIED {
            emit_diag(&mut self.diags, "E-MOD-1207", span);
        }
    }

    fn args(&mut self, args: &[Arg]) {
        for arg in args {
            self.expr(&arg.value);
        }
    }

    fn field_inits(&mut self, fields: &[FieldInit]) {
        for field in fields {
            self.expr(&field.value);
        }
    }

    fn exprs(&mut self, exprs: &[ExprPtr]) {
        for expr in exprs {
            self.expr(expr);
        }
    }

    fn invariant(&mut self, invariant: &Option<LoopInvariant>) {
        if let Some(invariant) = invariant {
            self.expr(&invariant.predicate);
        }
    }

    fn type_invariant(&mut self, invariant: &Option<TypeInvariant>) {
        if let Some(invariant) = invariant {
            self.expr(&invariant.predicate);
        }
    }

    fn block(&mut self, block: &BlockPtr) {
        let Some(block) = block else {
            return;
        };
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
        self.expr(&block.tail_opt);
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(node) => self.expr(&node.binding.init),
            Stmt::VarStmt(node) => self.expr(&node.binding.init),
            Stmt::AssignStmt(node) => {
                self.expr(&node.place);
                self.expr(&node.value);
            }
            Stmt::CompoundAssignStmt(node) => {
                self.expr(&node.place);
                self.expr(&node.value);
            }
            Stmt::ExprStmt(node) => self.expr(&node.value),
            Stmt::DeferStmt(node) => self.block(&node.body),
            Stmt::RegionStmt(node) => {
                self.expr(&node.opts_opt);
                self.block(&node.body);
            }
            Stmt::FrameStmt(node) => self.block(&node.body),
            Stmt::ReturnStmt(node) => self.expr(&node.value_opt),
            Stmt::BreakStmt(node) => self.expr(&node.value_opt),
            Stmt::UnsafeBlockStmt(node) => self.block(&node.body),
            _ => {}
        }
    }

    fn expr(&mut self, expr: &ExprPtr) {
        let Some(expr) = expr else {
            return;
        };
        match &expr.node {
            ExprNode::QualifiedNameExpr(node) => self.check_access(&node.path, &node.name, &expr.span),
            ExprNode::QualifiedApplyExpr(node) => {
                self.check_access(&node.path, &node.name, &expr.span);
                match &node.args {
                    ApplyArgs::ParenArgs(paren) => self.args(&paren.args),
                    ApplyArgs::BraceArgs(brace) => self.field_inits(&brace.fields),
                }
            }
            ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
                Some(EnumPayload::EnumPayloadParen(paren)) => self.exprs(&paren.elements),
                Some(EnumPayload::EnumPayloadBrace(brace)) => self.field_inits(&brace.fields),
                None => {}
            },
            ExprNode::RangeExpr(node) => {
                self.expr(&node.lhs);
                self.expr(&node.rhs);
            }
            ExprNode::BinaryExpr(node) => {
                self.expr(&node.lhs);
                self.expr(&node.rhs);
            }
            ExprNode::CastExpr(node) => self.expr(&node.value),
            ExprNode::UnaryExpr(node) => self.expr(&node.value),
            ExprNode::DerefExpr(node) => self.expr(&node.value),
            ExprNode::AddressOfExpr(node) => self.expr(&node.place),
            ExprNode::MoveExpr(node) => self.expr(&node.place),
            ExprNode::AllocExpr(node) => self.expr(&node.value),
            ExprNode::TupleExpr(node) => self.exprs(&node.elements),
            ExprNode::ArrayExpr(node) => {
                for segment in &node.elements {
                    match segment {
                        ArraySegment::ArrayElemSegment(elem) => self.expr(&elem.value),
                        ArraySegment::ArrayRepeatSegment(repeat) => {
                            self.expr(&repeat.value);
                            self.expr(&repeat.count);
                        }
                    }
                }
            }
            ExprNode::ArrayRepeatExpr(node) => {
                self.expr(&node.value);
                self.expr(&node.count);
            }
            ExprNode::RecordExpr(node) => self.field_inits(&node.fields),
            ExprNode::IfExpr(node) => {
                self.expr(&node.cond);
                self.expr(&node.then_expr);
                self.expr(&node.else_expr);
            }
            ExprNode::IfCaseExpr(node) => {
                self.expr(&node.scrutinee);
                for case_clause in &node.cases {
                    self.expr(&case_clause.body);
                }
                self.expr(&node.else_expr);
            }
            ExprNode::IfIsExpr(node) => {
                self.expr(&node.scrutinee);
                self.expr(&node.then_expr);
                self.expr(&node.else_expr);
            }
            ExprNode::LoopInfiniteExpr(node) => {
                self.invariant(&node.invariant_opt);
                self.block(&node.body);
            }
            ExprNode::LoopConditionalExpr(node) => {
                self.expr(&node.cond);
                self.invariant(&node.invariant_opt);
                self.block(&node.body);
            }
            ExprNode::LoopIterExpr(node) => {
                self.expr(&node.iter);
                self.invariant(&node.invariant_opt);
                self.block(&node.body);
            }
            ExprNode::BlockExpr(node) => self.block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.block(&node.block),
            ExprNode::TransmuteExpr(node) => self.expr(&node.value),
            ExprNode::FieldAccessExpr(node) => self.expr(&node.base),
            ExprNode::TupleAccessExpr(node) => self.expr(&node.base),
            ExprNode::IndexAccessExpr(node) => {
                self.expr(&node.base);
                self.expr(&node.index);
            }
            ExprNode::CallExpr(node) => {
                self.expr(&node.callee);
                self.args(&node.args);
            }
            ExprNode::CallTypeArgsExpr(node) => {
                self.expr(&node.callee);
                self.args(&node.args);
            }
            ExprNode::MethodCallExpr(node) => {
                self.expr(&node.receiver);
                self.args(&node.args);
            }
            ExprNode::PropagateExpr(node) => self.expr(&node.value),
            ExprNode::EntryExpr(node) => self.expr(&node.expr),
            ExprNode::YieldExpr(node) => self.expr(&node.value),
            ExprNode::YieldFromExpr(node) => self.expr(&node.value),
            ExprNode::SyncExpr(node) => self.expr(&node.value),
            ExprNode::RaceExpr(node) => {
                for arm in &node.arms {
                    self.expr(&arm.expr);
                    self.expr(&arm.handler.value);
                }
            }
            ExprNode::AllExpr(node) => self.exprs(&node.exprs),
            // The reference does not descend into the remaining forms.
            _ => {}
        }
    }

    fn item(&mut self, item: &ASTItem) {
        match item {
            ASTItem::StaticDecl(node) => self.expr(&node.binding.init),
            ASTItem::ProcedureDecl(node) => self.block(&node.body),
            ASTItem::RecordDecl(node) => {
                self.type_invariant(&node.invariant_opt);
                for member in &node.members {
                    match member {
                        RecordMember::FieldDecl(field) => self.expr(&field.init_opt),
                        RecordMember::MethodDecl(method) => self.block(&method.body),
                        RecordMember::AssociatedTypeDecl(_) => {}
                    }
                }
            }
            ASTItem::EnumDecl(node) => self.type_invariant(&node.invariant_opt),
            ASTItem::ModalDecl(node) => {
                self.type_invariant(&node.invariant_opt);
                for member in node.states.iter().flat_map(|state| &state.members) {
                    match member {
                        StateMember::StateMethodDecl(method) => self.block(&method.body),
                        StateMember::TransitionDecl(transition) => self.block(&transition.body),
                        StateMember::StateFieldDecl(_) => {}
                    }
                }
            }
            ASTItem::ClassDecl(node) => {
                for class_item in &node.items {
                    if let ClassItem::ClassMethodDecl(method) = class_item {
                        self.block(&method.body_opt);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn can_access_vis(accessor_module: &[String], decl_module: &[String], vis: Visibility) -> AccessResult {
    match vis {
        Visibility::Public => {
            spec_rule!("Access-Public");
            ALLOWED
        }
        Visibility::Internal if same_assembly(accessor_module, decl_module) => {
            spec_rule!("Access-Internal");
            ALLOWED
        }
        Visibility::Internal => {
            spec_rule!("Access-Internal-Err");
            DENIED
        }
        Visibility::Private if path_eq(accessor_module, decl_module) => {
            spec_rule!("Access-Private");
            ALLOWED
        }
        Visibility::Private => {
            spec_rule!("Access-Err");
            DENIED
        }
    }
}

/// Whether the current module may name `module_path::name`. Names that cannot be found
/// are allowed here; resolution reports them.
pub fn can_access(ctx: &ScopeContext<'_>, module_path: &[String], name: &str) -> AccessResult {
    if !module_path.is_empty() {
        if let Some(access) = can_access_from_name_map(ctx, module_path, name) {
            return access;
        }
    }
    let Some(resolved_module) = resolve_visible_module_path(ctx, module_path) else {
        return ALLOWED;
    };
    if let Some(access) = can_access_from_name_map(ctx, &resolved_module, name) {
        return access;
    }
    match find_decl_visibility_by_name(ctx, &resolved_module, name) {
        Some(vis) => can_access_vis(&ctx.current_module, &resolved_module, vis),
        None => ALLOWED,
    }
}

pub fn top_level_vis(item: &ASTItem) -> AccessResult {
    if vis_opt(item).is_some() {
        spec_rule!("TopLevelVis-Ok");
    }
    ALLOWED
}

/// Access checks for one module, plus the warning for a wildcard `using` in a module that
/// has a public interface.
pub fn check_module_visibility(ctx: &ScopeContext<'_>, module: &ASTModule) -> DiagnosticStream {
    let mut checker = AccessChecker { ctx, diags: DiagnosticStream::new() };
    let public_api = module.items.iter().any(|item| vis_opt(item) == Some(Visibility::Public));
    for item in &module.items {
        if let (true, ASTItem::UsingDecl(using_decl)) = (public_api, item) {
            if matches!(using_decl.clause, UsingClause::UsingWildcard(_)) {
                spec_rule!("Using-Wildcard-Warn");
                emit_diag(&mut checker.diags, "W-MOD-1201", &using_decl.span);
            }
        }
        checker.item(item);
    }
    checker.diags
}
