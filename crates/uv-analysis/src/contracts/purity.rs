//! Whether an expression is pure: free of effects, so that it may stand in a contract
//! and its value may be taken as a fact.
//!
//! Calls are followed: a call is pure when the procedure or method it names has no
//! capability parameters and a pure body; a method must also take its receiver as
//! `const`. A procedure reached again while it is being analysed counts as pure.

use std::collections::{BTreeMap, HashSet};

use uv_source::ast::{self, ApplyArgs, ArraySegment, EnumPayload, ExprNode, ExprPtr, Stmt};

use crate::caps::builtin_paths::{is_capability_class_path, is_context_type_path};
use crate::composite::record_methods::lookup_method_static;
use crate::context::{ScopeContext, TypeDecl};
use crate::generics::monomorphize::{build_modal_ref_substitution, build_substitution, instantiate_type};
use crate::modal::lookup::{lookup_modal_decl, lookup_modal_field_decl, lookup_state_method_decl, lookup_transition_decl};
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::resolve::scopes_lookup::{resolve_type_name, resolve_value_name};
use crate::typing::type_lookup::{field_type, lookup_record_decl};
use crate::typing::type_lower::lower_type;
use crate::typing::type_predicates::{perm_of_type, strip_perm};
use crate::typing::types::*;
use uv_core::span::Span;

/// What a predicate or body is checked in: the bindings whose types are known, and
/// which of them are local, so that assigning to them is no effect.
#[derive(Clone, Default)]
pub struct ContractContext<'c, 'a> {
    pub params: BTreeMap<String, TypeRef>,
    pub local_bindings: HashSet<String>,
    pub receiver_type: TypeRef,
    pub return_type: TypeRef,
    pub scope_ctx: Option<&'c ScopeContext<'a>>,
    pub is_postcondition: bool,
    /// `move` is allowed in a body that is analysed, never in a predicate itself.
    pub allow_responsibility_moves: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ContractCheckResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
}

/// The declarations being analysed, by address.
#[derive(Default)]
struct PurityStack {
    procedures: HashSet<*const ast::ProcedureDecl>,
    record_methods: HashSet<*const ast::MethodDecl>,
    class_methods: HashSet<*const ast::ClassMethodDecl>,
    state_methods: HashSet<*const ast::StateMethodDecl>,
}

/// Whether a written type mentions a capability: `$IO` and the like, or `Context`.
/// Type applications are not looked into.
fn ast_type_has_capability(type_ptr: &ast::TypePtr) -> bool {
    let Some(ty) = type_ptr.as_deref() else {
        return false;
    };
    let any = |types: &[ast::TypePtr]| types.iter().any(ast_type_has_capability);
    use ast::TypeNode as N;
    match &ty.node {
        N::TypeDynamic(node) => is_capability_class_path(&node.path),
        N::TypePathType(node) => is_context_type_path(&node.path) || any(&node.generic_args),
        N::TypeModalState(node) => any(&node.generic_args),
        N::TypePermType(node) => ast_type_has_capability(&node.base),
        N::TypeRefine(node) => ast_type_has_capability(&node.base),
        N::TypeUnion(node) => any(&node.types),
        N::TypeTuple(node) => any(&node.elements),
        N::TypeArray(node) => ast_type_has_capability(&node.element),
        N::TypeSlice(node) => ast_type_has_capability(&node.element),
        N::TypeSafePtr(node) => ast_type_has_capability(&node.element),
        N::TypeRawPtr(node) => ast_type_has_capability(&node.element),
        N::TypeFunc(node) => {
            node.params.iter().any(|param| ast_type_has_capability(&param.r#type)) || ast_type_has_capability(&node.ret)
        }
        N::TypeClosure(node) => {
            node.params.iter().any(|param| ast_type_has_capability(&param.r#type))
                || ast_type_has_capability(&node.ret)
                || node.deps_opt.iter().flatten().any(|dep| ast_type_has_capability(&dep.r#type))
        }
        _ => false,
    }
}

fn has_capability_params(params: &[ast::Param]) -> bool {
    params.iter().any(|param| ast_type_has_capability(&param.r#type))
}

/// `string::length`, `string::as_view`, `string::slice`, `bytes::view_string`,
/// `bytes::as_slice`.
fn is_pure_builtin_qualified_name(path: &[String], name: &str) -> bool {
    let names: &[&str] = match path {
        [only] if id_eq(only, "string") => &["length", "as_view", "slice"],
        [only] if id_eq(only, "bytes") => &["view_string", "as_slice"],
        _ => return false,
    };
    names.iter().any(|candidate| id_eq(name, candidate))
}

enum Callee<'c> {
    Procedure(&'c ast::ProcedureDecl, Vec<String>),
    Comptime,
}

fn lookup_procedure_for_callee<'c>(ctx: &'c ScopeContext<'_>, callee: &ExprPtr) -> Option<Callee<'c>> {
    let (origin, name) = match &callee.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => match resolve_value_name(ctx, &ident.name) {
            Some(entity) if entity.origin_opt.is_some() => {
                (entity.origin_opt.unwrap_or_default(), entity.target_opt.unwrap_or_else(|| ident.name.clone()))
            }
            _ => (ctx.current_module.clone(), ident.name.clone()),
        },
        ExprNode::QualifiedNameExpr(node) => (node.path.clone(), node.name.clone()),
        ExprNode::PathExpr(node) => {
            (if node.path.is_empty() { ctx.current_module.clone() } else { node.path.clone() }, node.name.clone())
        }
        _ => return None,
    };
    let module = ctx.sigma.mods.iter().find(|module| module.path == origin)?;
    let procedure = module.items.iter().find_map(|item| match item {
        ast::ASTItem::ProcedureDecl(proc) if id_eq(&proc.name, &name) => Some(proc),
        _ => None,
    });
    if let Some(proc) = procedure {
        return Some(Callee::Procedure(proc, origin));
    }
    let comptime = module.comptime_procedures.iter().any(|proc| id_eq(&proc.name, &name))
        || module.items.iter().any(|item| matches!(item, ast::ASTItem::ComptimeProcedureDecl(proc) if id_eq(&proc.name, &name)));
    comptime.then_some(Callee::Comptime)
}

fn field_pattern_names(field: &ast::FieldPattern, out: &mut HashSet<String>) {
    if field.pattern_opt.is_some() {
        collect_pattern_binding_names(&field.pattern_opt, out);
    } else if !id_eq(&field.name, "_") {
        out.insert(field.name.clone());
    }
}

fn collect_pattern_binding_names(pattern: &ast::PatternPtr, out: &mut HashSet<String>) {
    let Some(pattern) = pattern.as_deref() else {
        return;
    };
    use ast::PatternNode as P;
    match &pattern.node {
        P::IdentifierPattern(node) if !id_eq(&node.name, "_") => {
            out.insert(node.name.clone());
        }
        P::TypedPattern(node) if !id_eq(&node.name, "_") => {
            out.insert(node.name.clone());
        }
        P::TuplePattern(node) => node.elements.iter().for_each(|elem| collect_pattern_binding_names(elem, out)),
        P::RecordPattern(node) => node.fields.iter().for_each(|field| field_pattern_names(field, out)),
        P::EnumPattern(node) => match &node.payload_opt {
            Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => {
                payload.elements.iter().for_each(|elem| collect_pattern_binding_names(elem, out));
            }
            Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => {
                payload.fields.iter().for_each(|field| field_pattern_names(field, out));
            }
            None => {}
        },
        P::ModalPattern(node) => {
            node.fields_opt.iter().flat_map(|payload| &payload.fields).for_each(|field| field_pattern_names(field, out));
        }
        P::RangePattern(node) => {
            collect_pattern_binding_names(&node.lo, out);
            collect_pattern_binding_names(&node.hi, out);
        }
        _ => {}
    }
}

/// The binding a place is rooted in.
fn assignment_root_name(expr: &ExprPtr) -> Option<&str> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(node) => Some(&node.name),
        ExprNode::PathExpr(node) if node.path.is_empty() => Some(&node.name),
        ExprNode::FieldAccessExpr(node) => assignment_root_name(&node.base),
        ExprNode::TupleAccessExpr(node) => assignment_root_name(&node.base),
        ExprNode::IndexAccessExpr(node) => assignment_root_name(&node.base),
        _ => None,
    }
}

fn strip_perm_or_self(ty: &TypeRef) -> TypeRef {
    // Stripping permissions never empties a type.
    strip_perm(ty)
}

impl<'c, 'a> ContractContext<'c, 'a> {
    fn binding_type(&self, name: &str) -> TypeRef {
        if id_eq(name, "self") {
            return self.receiver_type.clone();
        }
        self.params.get(name).cloned().flatten()
    }

    fn lookup_type_alias_decl(scope_ctx: &'c ScopeContext<'a>, path: &[String]) -> Option<&'c ast::TypeAliasDecl> {
        let decl = match scope_ctx.sigma.types.get(&path_key_of(path)) {
            Some(decl) => decl,
            None => {
                let [name] = path else {
                    return None;
                };
                let entity = resolve_type_name(scope_ctx, name)?;
                let mut resolved = entity.origin_opt?;
                resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
                scope_ctx.sigma.types.get(&path_key_of(&resolved))?
            }
        };
        match decl {
            TypeDecl::TypeAlias(alias) => Some(alias),
            _ => None,
        }
    }

    /// The type with aliases expanded, under permissions and refinements and inside
    /// unions.
    fn normalize_alias_type(&self, type_ref: &TypeRef, depth: usize) -> TypeRef {
        let (Some(scope_ctx), Some(ty)) = (self.scope_ctx, type_ref.as_deref()) else {
            return type_ref.clone();
        };
        if depth > 16 {
            return type_ref.clone();
        }
        match &ty.node {
            TypeNode::Perm { perm, base } => return make_type_perm(*perm, self.normalize_alias_type(base, depth + 1)),
            TypeNode::Refine { base, predicate } => {
                return make_type_refine(self.normalize_alias_type(base, depth + 1), predicate.clone());
            }
            TypeNode::Union(members) => {
                return make_type_union(members.iter().map(|member| self.normalize_alias_type(member, depth + 1)).collect());
            }
            _ => {}
        }
        let Some(path) = applied_type_path(ty).filter(|path| !path.is_empty()) else {
            return type_ref.clone();
        };
        let Some(alias) = Self::lookup_type_alias_decl(scope_ctx, path) else {
            return type_ref.clone();
        };
        let Ok(lowered @ Some(_)) = lower_type(scope_ctx, &alias.r#type) else {
            return type_ref.clone();
        };
        let mut instantiated = lowered;
        if let (Some(params), Some(args)) = (&alias.generic_params, applied_type_args(ty)) {
            if args.len() > params.params.len() {
                return type_ref.clone();
            }
            instantiated = instantiate_type(&instantiated, &build_substitution(&params.params, args));
            if instantiated.is_none() {
                return type_ref.clone();
            }
        }
        self.normalize_alias_type(&instantiated, depth + 1)
    }

    fn normalized(&self, ty: &TypeRef) -> TypeRef {
        self.normalize_alias_type(ty, 0)
    }

    fn record_field_type(&self, record_type: &TypeRef, field_name: &str) -> TypeRef {
        let scope_ctx = self.scope_ctx?;
        let stripped = strip_perm_or_self(&self.normalized(record_type));
        let stripped_ty = stripped.as_deref()?;
        let record = lookup_record_decl(scope_ctx, applied_type_path(stripped_ty)?)?;
        let args = applied_type_args(stripped_ty).unwrap_or(&[]);
        self.normalized(&field_type(record, field_name, scope_ctx, args)?)
    }

    fn modal_state_field_type(&self, state_type: &TypeRef, field_name: &str) -> TypeRef {
        let scope_ctx = self.scope_ctx?;
        let stripped = strip_perm_or_self(&self.normalized(state_type));
        let TypeNode::ModalState(modal) = &stripped.as_deref()?.node else {
            return None;
        };
        let decl = lookup_modal_decl(scope_ctx, &modal.path)?;
        let field = lookup_modal_field_decl(decl, &modal.state, field_name)?;
        field.r#type.as_ref()?;
        let mut field_ty = lower_type(scope_ctx, &field.r#type).ok().filter(|ty| ty.is_some())?;
        if let Some(params) = &decl.generic_params {
            field_ty = instantiate_type(&field_ty, &build_modal_ref_substitution(&params.params, &modal.generic_args));
        }
        self.normalized(&field_ty)
    }

    /// The type of an expression, as far as a contract can tell without the typer: a
    /// known binding, `@result`, a cast, or a field of one of these.
    fn infer_expr_type(&self, expr: &ExprPtr) -> TypeRef {
        let e = expr.as_deref()?;
        match &e.node {
            ExprNode::IdentifierExpr(node) => self.normalized(&self.binding_type(&node.name)),
            ExprNode::ResultExpr(_) => {
                self.normalized(&if self.is_postcondition { self.return_type.clone() } else { None })
            }
            ExprNode::EntryExpr(node) => self.infer_expr_type(&node.expr),
            ExprNode::AttributedExpr(node) => self.infer_expr_type(&node.expr),
            ExprNode::CastExpr(node) => {
                node.r#type.as_ref()?;
                lower_type(self.scope_ctx?, &node.r#type).ok().flatten()
            }
            ExprNode::FieldAccessExpr(node) => {
                self.scope_ctx?;
                let base = strip_perm_or_self(&self.infer_expr_type(&node.base));
                let base = strip_perm_or_self(&self.normalized(&base));
                let base_ty = base.as_deref()?;
                if applied_type_path(base_ty).is_some() {
                    return self.record_field_type(&base, &node.name);
                }
                if matches!(base_ty.node, TypeNode::ModalState(_)) {
                    return self.modal_state_field_type(&base, &node.name);
                }
                None
            }
            _ => None,
        }
    }

    /// The member of a type that is the named state: the type itself, or a member of a
    /// union.
    fn modal_state_member_for_pattern(&self, ty: &TypeRef, state_name: &str) -> TypeRef {
        let stripped = strip_perm_or_self(&self.normalized(ty));
        match &stripped.as_deref()?.node {
            TypeNode::ModalState(modal) => id_eq(&modal.state, state_name).then(|| stripped.clone()).flatten(),
            TypeNode::Union(members) => {
                members.iter().find_map(|member| self.modal_state_member_for_pattern(member, state_name).map(Some)).flatten()
            }
            _ => None,
        }
    }

    fn bind_local(&mut self, name: &str, ty: TypeRef) {
        self.params.insert(name.to_string(), ty);
        self.local_bindings.insert(name.to_string());
    }

    /// Gives the names a pattern binds their types, as far as the matched type shows them.
    fn bind_pattern_types_from_expected(&mut self, pattern: &ast::PatternPtr, expected_type: &TypeRef) {
        let (Some(pattern), Some(_)) = (pattern.as_deref(), expected_type) else {
            return;
        };
        let normalized = self.normalized(expected_type);
        use ast::PatternNode as P;
        let bind_field = |ctx: &mut Self, field: &ast::FieldPattern, field_ty: TypeRef| {
            if field_ty.is_none() {
                return;
            }
            if field.pattern_opt.is_some() {
                ctx.bind_pattern_types_from_expected(&field.pattern_opt, &field_ty);
            } else if !id_eq(&field.name, "_") {
                ctx.bind_local(&field.name, field_ty);
            }
        };
        match &pattern.node {
            P::IdentifierPattern(node) if !id_eq(&node.name, "_") => self.bind_local(&node.name, normalized),
            P::TypedPattern(node) if !id_eq(&node.name, "_") => {
                let declared = self
                    .scope_ctx
                    .filter(|_| node.r#type.is_some())
                    .and_then(|scope_ctx| lower_type(scope_ctx, &node.r#type).ok())
                    .filter(|ty| ty.is_some())
                    .map(|ty| self.normalized(&ty));
                self.bind_local(&node.name, declared.unwrap_or(normalized));
            }
            P::TuplePattern(node) => {
                let stripped = strip_perm_or_self(&normalized);
                if let Some(TypeNode::Tuple(elements)) = stripped.as_deref().map(|ty| &ty.node) {
                    for (pattern, ty) in node.elements.iter().zip(elements) {
                        self.bind_pattern_types_from_expected(pattern, ty);
                    }
                }
            }
            P::RecordPattern(node) => {
                for field in &node.fields {
                    let field_ty = self.record_field_type(&normalized, &field.name);
                    bind_field(self, field, field_ty);
                }
            }
            P::ModalPattern(node) => {
                let state_type = self.modal_state_member_for_pattern(&normalized, &node.state);
                if state_type.is_none() {
                    return;
                }
                for field in node.fields_opt.iter().flat_map(|payload| &payload.fields) {
                    let field_ty = self.modal_state_field_type(&state_type, &field.name);
                    bind_field(self, field, field_ty);
                }
            }
            _ => {}
        }
    }

    /// The context for the branch a state pattern guards: the matched binding is known
    /// to be in that state.
    fn narrowed_for_pattern(&self, scrutinee: &ExprPtr, pattern: &ast::PatternPtr) -> Self {
        let mut narrowed = self.clone();
        let Some(ast::PatternNode::ModalPattern(modal_pattern)) = pattern.as_deref().map(|pattern| &pattern.node) else {
            return narrowed;
        };
        let binding_name = match scrutinee.as_deref().map(|expr| &expr.node) {
            Some(ExprNode::IdentifierExpr(node)) => &node.name,
            Some(ExprNode::PathExpr(node)) if node.path.is_empty() => &node.name,
            _ => return narrowed,
        };
        let mut scrutinee_type = self.binding_type(binding_name);
        if scrutinee_type.is_none() {
            scrutinee_type = self.infer_expr_type(scrutinee);
        }
        let state_type = self.modal_state_member_for_pattern(&scrutinee_type, &modal_pattern.state);
        if scrutinee_type.is_none() || state_type.is_none() {
            return narrowed;
        }
        if id_eq(binding_name, "self") {
            narrowed.receiver_type = state_type.clone();
        } else {
            narrowed.params.insert(binding_name.clone(), state_type.clone());
        }
        narrowed.bind_pattern_types_from_expected(pattern, &state_type);
        narrowed
    }
}

fn receiver_is_const(receiver: &ast::Receiver, scope_ctx: &ScopeContext<'_>) -> bool {
    match receiver {
        ast::Receiver::ReceiverShorthand(recv) => recv.perm == ast::ReceiverPerm::Const && recv.mode_opt.is_none(),
        ast::Receiver::ReceiverExplicit(recv) => {
            recv.r#type.is_some()
                && recv.mode_opt.is_none()
                && lower_type(scope_ctx, &recv.r#type)
                    .is_ok_and(|lowered| lowered.is_some() && perm_of_type(&lowered) == Permission::Const)
        }
    }
}

/// The module a type is declared in.
fn module_path_for_owned_type(type_path: &[String]) -> Vec<String> {
    type_path.split_last().map(|(_, module)| module.to_vec()).unwrap_or_default()
}

/// The callee is analysed as its own module sees it, with the caller's scopes.
fn in_module<'a>(scope_ctx: &ScopeContext<'a>, module: &[String]) -> ScopeContext<'a> {
    let mut callee_ctx = scope_ctx.clone();
    callee_ctx.current_module = module.to_vec();
    callee_ctx
}

fn body_context<'c, 'a>(scope_ctx: &'c ScopeContext<'a>, params: &[ast::Param], receiver_type: TypeRef) -> ContractContext<'c, 'a> {
    let mut ctx = ContractContext {
        scope_ctx: Some(scope_ctx),
        receiver_type,
        allow_responsibility_moves: true,
        ..Default::default()
    };
    for param in params.iter().filter(|param| param.r#type.is_some()) {
        if let Ok(lowered @ Some(_)) = lower_type(scope_ctx, &param.r#type) {
            ctx.params.insert(param.name.clone(), lowered);
        }
    }
    ctx
}

fn is_pure_procedure(scope_ctx: &ScopeContext<'_>, owner_module: &[String], proc: &ast::ProcedureDecl, stack: &mut PurityStack) -> bool {
    let Some(body) = proc.body.as_deref().filter(|_| !has_capability_params(&proc.params)) else {
        return false;
    };
    if !stack.procedures.insert(proc) {
        return true;
    }
    let callee_ctx = in_module(scope_ctx, owner_module);
    let pure = !is_impure_block(&mut body_context(&callee_ctx, &proc.params, None), body, stack);
    stack.procedures.remove(&(proc as *const _));
    pure
}

/// A method is pure when it takes its receiver as `const`, has no capability parameters
/// and a pure body.
fn is_pure_method<M>(
    scope_ctx: &ScopeContext<'_>,
    method: &M,
    parts: (&ast::Receiver, &[ast::Param], &ast::BlockPtr),
    receiver_type: &TypeRef,
    owner_module: &[String],
    active: fn(&mut PurityStack) -> &mut HashSet<*const M>,
    stack: &mut PurityStack,
) -> bool {
    let (receiver, params, body) = parts;
    let Some(body) = body.as_deref() else {
        return false;
    };
    if has_capability_params(params) || !receiver_is_const(receiver, scope_ctx) {
        return false;
    }
    if !active(stack).insert(method) {
        return true;
    }
    let callee_ctx = in_module(scope_ctx, owner_module);
    let pure = !is_impure_block(&mut body_context(&callee_ctx, params, receiver_type.clone()), body, stack);
    active(stack).remove(&(method as *const M));
    pure
}

fn is_impure_stmt(ctx: &mut ContractContext<'_, '_>, stmt: &Stmt, stack: &mut PurityStack) -> bool {
    let mut binding = |ctx: &mut ContractContext<'_, '_>, binding: &ast::Binding| {
        let init_impure = is_impure_expr(ctx, &binding.init, stack);
        let annotation = if binding.type_opt.is_some() {
            binding.type_opt.clone()
        } else {
            match binding.pat.as_deref().map(|pat| &pat.node) {
                Some(ast::PatternNode::TypedPattern(typed)) => typed.r#type.clone(),
                _ => None,
            }
        };
        let binding_type = if annotation.is_some() {
            ctx.scope_ctx.and_then(|scope_ctx| lower_type(scope_ctx, &annotation).ok()).and_then(|ty| ctx.normalized(&ty))
        } else {
            ctx.infer_expr_type(&binding.init)
        };
        collect_pattern_binding_names(&binding.pat, &mut ctx.local_bindings);
        ctx.bind_pattern_types_from_expected(&binding.pat, &binding_type);
        init_impure
    };
    match stmt {
        Stmt::LetStmt(node) => binding(ctx, &node.binding),
        Stmt::VarStmt(node) => binding(ctx, &node.binding),
        Stmt::ExprStmt(node) => is_impure_expr(ctx, &node.value, stack),
        Stmt::ReturnStmt(node) => is_impure_expr(ctx, &node.value_opt, stack),
        // Assigning is an effect unless the target is a local of the analysed body.
        Stmt::AssignStmt(ast::AssignStmt { place, value, .. }) | Stmt::CompoundAssignStmt(ast::CompoundAssignStmt { place, value, .. }) => {
            is_impure_expr(ctx, place, stack)
                || is_impure_expr(ctx, value, stack)
                || !assignment_root_name(place).is_some_and(|root| ctx.local_bindings.contains(root))
        }
        Stmt::DeferStmt(_) | Stmt::RegionStmt(_) | Stmt::FrameStmt(_) | Stmt::KeyBlockStmt(_) | Stmt::UnsafeBlockStmt(_) => true,
        _ => false,
    }
}

fn is_impure_block(ctx: &mut ContractContext<'_, '_>, block: &ast::Block, stack: &mut PurityStack) -> bool {
    block.stmts.iter().any(|stmt| is_impure_stmt(ctx, stmt, stack)) || is_impure_expr(ctx, &block.tail_opt, stack)
}

fn is_impure_expr(ctx: &mut ContractContext<'_, '_>, expr: &ExprPtr, stack: &mut PurityStack) -> bool {
    let Some(e) = expr.as_deref() else {
        return false;
    };
    let mut impure = |ctx: &mut ContractContext<'_, '_>, expr: &ExprPtr| is_impure_expr(ctx, expr, stack);
    match &e.node {
        ExprNode::LiteralExpr(_)
        | ExprNode::PtrNullExpr(_)
        | ExprNode::IdentifierExpr(_)
        | ExprNode::QualifiedNameExpr(_)
        | ExprNode::PathExpr(_)
        | ExprNode::ResultExpr(_)
        | ExprNode::SizeofExpr(_)
        | ExprNode::AlignofExpr(_) => false,
        ExprNode::BinaryExpr(node) => impure(ctx, &node.lhs) || impure(ctx, &node.rhs),
        ExprNode::RangeExpr(node) => impure(ctx, &node.lhs) || impure(ctx, &node.rhs),
        ExprNode::PipelineExpr(node) => impure(ctx, &node.lhs) || impure(ctx, &node.rhs),
        ExprNode::UnaryExpr(node) => impure(ctx, &node.value),
        ExprNode::CastExpr(node) => impure(ctx, &node.value),
        ExprNode::FieldAccessExpr(node) => impure(ctx, &node.base),
        ExprNode::TupleAccessExpr(node) => impure(ctx, &node.base),
        ExprNode::IndexAccessExpr(node) => impure(ctx, &node.base) || impure(ctx, &node.index),
        ExprNode::AddressOfExpr(node) => impure(ctx, &node.place),
        ExprNode::DerefExpr(node) => impure(ctx, &node.value),
        ExprNode::EntryExpr(node) => impure(ctx, &node.expr),
        ExprNode::AttributedExpr(node) => impure(ctx, &node.expr),
        ExprNode::IfExpr(node) => impure(ctx, &node.cond) || impure(ctx, &node.then_expr) || impure(ctx, &node.else_expr),
        ExprNode::IfCaseExpr(node) => {
            if impure(ctx, &node.scrutinee) {
                return true;
            }
            for arm in &node.cases {
                let mut arm_ctx = ctx.narrowed_for_pattern(&node.scrutinee, &arm.pattern);
                if impure(&mut arm_ctx, &arm.body) {
                    return true;
                }
            }
            impure(ctx, &node.else_expr)
        }
        ExprNode::IfIsExpr(node) => {
            if impure(ctx, &node.scrutinee) {
                return true;
            }
            let mut then_ctx = ctx.narrowed_for_pattern(&node.scrutinee, &node.pattern);
            impure(&mut then_ctx, &node.then_expr) || impure(ctx, &node.else_expr)
        }
        ExprNode::TupleExpr(node) => node.elements.iter().any(|elem| impure(ctx, elem)),
        ExprNode::ArrayExpr(node) => node.elements.iter().any(|segment| match segment {
            ArraySegment::ArrayElemSegment(elem) => impure(ctx, &elem.value),
            ArraySegment::ArrayRepeatSegment(repeat) => impure(ctx, &repeat.value) || impure(ctx, &repeat.count),
        }),
        ExprNode::ArrayRepeatExpr(node) => impure(ctx, &node.value) || impure(ctx, &node.count),
        ExprNode::RecordExpr(node) => node.fields.iter().any(|field| impure(ctx, &field.value)),
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            None => false,
            Some(EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().any(|elem| impure(ctx, elem)),
            Some(EnumPayload::EnumPayloadBrace(payload)) => payload.fields.iter().any(|field| impure(ctx, &field.value)),
        },
        ExprNode::QualifiedApplyExpr(node) => match &node.args {
            ApplyArgs::ParenArgs(paren) => {
                paren.args.iter().any(|arg| impure(ctx, &arg.value)) || !is_pure_builtin_qualified_name(&node.path, &node.name)
            }
            ApplyArgs::BraceArgs(brace) => {
                let _ = brace.fields.iter().any(|field| impure(ctx, &field.value));
                true
            }
        },
        ExprNode::MoveExpr(node) => !ctx.allow_responsibility_moves || impure(ctx, &node.place),
        ExprNode::CallExpr(node) => {
            if impure(ctx, &node.callee) || node.args.iter().any(|arg| impure(ctx, &arg.value)) {
                return true;
            }
            let builtin = match node.callee.as_deref().map(|callee| &callee.node) {
                Some(ExprNode::QualifiedNameExpr(callee)) => is_pure_builtin_qualified_name(&callee.path, &callee.name),
                Some(ExprNode::PathExpr(callee)) => is_pure_builtin_qualified_name(&callee.path, &callee.name),
                _ => false,
            };
            if builtin {
                return false;
            }
            let Some(scope_ctx) = ctx.scope_ctx else {
                return true;
            };
            match lookup_procedure_for_callee(scope_ctx, &node.callee) {
                None => true,
                Some(Callee::Comptime) => false,
                Some(Callee::Procedure(proc, origin)) => !is_pure_procedure(scope_ctx, &origin, proc, stack),
            }
        }
        ExprNode::MethodCallExpr(node) => {
            if impure(ctx, &node.receiver) || node.args.iter().any(|arg| impure(ctx, &arg.value)) {
                return true;
            }
            let Some(scope_ctx) = ctx.scope_ctx else {
                return true;
            };
            let receiver_type = strip_perm_or_self(&ctx.infer_expr_type(&node.receiver));
            let Some(receiver_ty) = receiver_type.as_deref() else {
                return true;
            };
            if let TypeNode::ModalState(modal) = &receiver_ty.node {
                // A transition changes the state; a state method is judged by its body.
                let Some(decl) = lookup_modal_decl(scope_ctx, &modal.path) else {
                    return true;
                };
                if lookup_transition_decl(decl, &modal.state, &node.name).is_some() {
                    return true;
                }
                let Some(method) = lookup_state_method_decl(decl, &modal.state, &node.name) else {
                    return true;
                };
                return !is_pure_method(
                    scope_ctx,
                    method,
                    (&method.receiver, &method.params, &method.body),
                    &receiver_type,
                    &module_path_for_owned_type(&modal.path),
                    |stack| &mut stack.state_methods,
                    stack,
                );
            }
            let Ok(lookup) = lookup_method_static(scope_ctx, &receiver_type, &node.name) else {
                return true;
            };
            if let Some(method) = lookup.record_method {
                return !is_pure_method(
                    scope_ctx,
                    method,
                    (&method.receiver, &method.params, &method.body),
                    &receiver_type,
                    &module_path_for_owned_type(&lookup.record_path),
                    |stack| &mut stack.record_methods,
                    stack,
                );
            }
            if let Some(method) = lookup.class_method {
                return !is_pure_method(
                    scope_ctx,
                    method,
                    (&method.receiver, &method.params, &method.body_opt),
                    &receiver_type,
                    &module_path_for_owned_type(&lookup.owner_class),
                    |stack| &mut stack.class_methods,
                    stack,
                );
            }
            true
        }
        ExprNode::BlockExpr(node) => node.block.as_deref().is_some_and(|block| is_impure_block(ctx, block, stack)),
        ExprNode::LoopInfiniteExpr(node) => {
            node.invariant_opt.as_ref().is_some_and(|invariant| impure(ctx, &invariant.predicate))
                || node.body.as_deref().is_some_and(|body| is_impure_block(ctx, body, stack))
        }
        ExprNode::LoopConditionalExpr(node) => {
            impure(ctx, &node.cond)
                || node.invariant_opt.as_ref().is_some_and(|invariant| impure(ctx, &invariant.predicate))
                || node.body.as_deref().is_some_and(|body| is_impure_block(ctx, body, stack))
        }
        ExprNode::LoopIterExpr(node) => {
            if impure(ctx, &node.iter) || node.invariant_opt.as_ref().is_some_and(|invariant| impure(ctx, &invariant.predicate)) {
                return true;
            }
            let mut loop_ctx = ctx.clone();
            collect_pattern_binding_names(&node.pattern, &mut loop_ctx.local_bindings);
            node.body.as_deref().is_some_and(|body| is_impure_block(&mut loop_ctx, body, stack))
        }
        ExprNode::AllocExpr(_)
        | ExprNode::YieldExpr(_)
        | ExprNode::YieldFromExpr(_)
        | ExprNode::WaitExpr(_)
        | ExprNode::SyncExpr(_)
        | ExprNode::SpawnExpr(_)
        | ExprNode::ParallelExpr(_)
        | ExprNode::DispatchExpr(_)
        | ExprNode::TransmuteExpr(_)
        | ExprNode::UnsafeBlockExpr(_)
        | ExprNode::ClosureExpr(_)
        | ExprNode::FenceExpr(_)
        | ExprNode::PropagateExpr(_) => true,
        _ => false,
    }
}

/// Whether the expression is pure in the given context; impurity is `E-SEM-2802` at the
/// expression.
pub fn check_purity(ctx: &ContractContext<'_, '_>, expr: &ExprPtr) -> ContractCheckResult {
    let mut local_ctx = ctx.clone();
    if is_impure_expr(&mut local_ctx, expr, &mut PurityStack::default()) {
        ContractCheckResult { ok: false, diag_id: Some("E-SEM-2802"), span: expr.as_deref().map(|expr| expr.span.clone()) }
    } else {
        ContractCheckResult { ok: true, ..Default::default() }
    }
}

/// Purity as the typer asks for it: in the given scope, with nothing else known.
pub fn is_pure_in_scope(scope_ctx: &ScopeContext<'_>, expr: &ExprPtr) -> bool {
    check_purity(&ContractContext { scope_ctx: Some(scope_ctx), ..Default::default() }, expr).ok
}
