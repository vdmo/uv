//! What the resolver passes around: its context, and failures that carry the rule that
//! rejected the program.

use uv_core::diagnostics::{DiagnosticStream, SubDiagnostic};
use uv_core::span::Span;
use uv_source::ast::*;
use uv_source::module_paths::ModuleNames;

use crate::language_service::{
    make_language_service_local_entity, record_language_service_member_reference, record_language_service_reference, record_language_service_type_path_reference,
    LanguageServiceIndex, LanguageSymbolKind,
};
use super::scopes_lookup::CanAccessFn;
use crate::context::*;

pub struct ResolveContext<'c, 'a> {
    pub ctx: &'c mut ScopeContext<'a>,
    pub name_maps: &'c NameMapTable,
    pub module_names: &'c ModuleNames,
    pub can_access: Option<CanAccessFn>,
    pub parse_ok: bool,
    pub parse_diags: Option<&'c DiagnosticStream>,
    /// Where the language server's facts are collected, when something asks for them.
    pub language_service: Option<&'c std::cell::RefCell<LanguageServiceIndex>>,
}

impl ResolveContext<'_, '_> {
    /// A local name, with its declaration recorded for the language service.
    pub fn local_language_entity(&self, name: &str, span: &Span, kind: LanguageSymbolKind, detail: &str) -> Entity {
        match self.language_service {
            Some(index) => make_language_service_local_entity(Some(&mut index.borrow_mut()), self.ctx, name, span, kind, detail),
            None => make_language_service_local_entity(None, self.ctx, name, span, kind, detail),
        }
    }

    pub fn record_reference(&self, fallback_name: &str, span: &Span, entity: &Entity) {
        if let Some(index) = self.language_service {
            record_language_service_reference(Some(&mut index.borrow_mut()), fallback_name, span, entity);
        }
    }

    pub fn record_type_path_reference(&self, path: &[String], span: &Span) {
        if let Some(index) = self.language_service {
            record_language_service_type_path_reference(Some(&mut index.borrow_mut()), path, span);
        }
    }

    pub fn record_member_reference(&self, owner_path: &[String], member: &str, span: &Span) {
        if let Some(index) = self.language_service {
            record_language_service_member_reference(Some(&mut index.borrow_mut()), owner_path, member, span);
        }
    }
}

/// Why resolution failed. `diag_id` is a rule name or a diagnostic code; it is absent
/// for failures the reference treats as internal.
#[derive(Debug, Clone, Default)]
pub struct Failure {
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub detail: String,
    pub children: Vec<SubDiagnostic>,
}

/// A failure, boxed so that results stay small on the path where nothing fails.
#[derive(Debug, Clone, Default)]
pub struct ResError(Box<Failure>);

pub type Res<T> = Result<T, ResError>;

impl std::ops::Deref for ResError {
    type Target = Failure;

    fn deref(&self) -> &Failure {
        &self.0
    }
}

impl std::ops::DerefMut for ResError {
    fn deref_mut(&mut self) -> &mut Failure {
        &mut self.0
    }
}

impl ResError {
    pub fn new(diag_id: &'static str, span: Option<Span>) -> ResError {
        ResError::from_id(Some(diag_id), span)
    }

    pub fn at(diag_id: &'static str, span: &Span) -> ResError {
        ResError::new(diag_id, Some(span.clone()))
    }

    pub fn with_detail(diag_id: &'static str, span: &Span, detail: impl Into<String>) -> ResError {
        ResError::detailed(Some(diag_id), Some(span.clone()), detail)
    }

    pub fn detailed(diag_id: Option<&'static str>, span: Option<Span>, detail: impl Into<String>) -> ResError {
        ResError(Box::new(Failure { diag_id, span, detail: detail.into(), children: Vec::new() }))
    }

    pub fn from_id(diag_id: Option<&'static str>, span: Option<Span>) -> ResError {
        ResError(Box::new(Failure { diag_id, span, ..Default::default() }))
    }

    // The reference forwards a failure field by field, and many call sites drop some of
    // the fields. Which ones are dropped decides what the diagnostic looks like, so each
    // of these mirrors one forwarding shape.

    /// Keeps the rule only.
    pub fn id_only(self) -> ResError {
        ResError::from_id(self.diag_id, None)
    }

    /// Keeps the rule and the span.
    pub fn id_span(mut self) -> ResError {
        self.detail.clear();
        self.children.clear();
        self
    }

    /// Keeps everything but the suggestions.
    pub fn no_children(mut self) -> ResError {
        self.children.clear();
        self
    }

    pub fn span_or(mut self, span: &Span) -> ResError {
        if self.span.is_none() {
            self.span = Some(span.clone());
        }
        self
    }

    pub fn with_new_detail(mut self, detail: &str) -> ResError {
        self.detail = detail.to_string();
        self
    }
}

pub fn value_entity() -> Entity {
    Entity::new(EntityKind::Value, None, None, EntitySource::Decl)
}

pub fn type_entity() -> Entity {
    Entity::new(EntityKind::Type, None, None, EntitySource::Decl)
}

/// A local binding. The declaration span is kept for the language service.
pub fn local_entity(span: &Span) -> Entity {
    let mut entity = value_entity();
    entity.declaration_span = Some(span.clone());
    entity
}

/// Runs `body` with a fresh innermost scope, which is dropped afterwards.
pub fn with_scope<'c, 'a, T>(
    ctx: &mut ResolveContext<'c, 'a>,
    scope: Scope,
    body: impl FnOnce(&mut ResolveContext<'c, 'a>) -> T,
) -> T {
    ctx.ctx.scopes.insert(0, scope);
    let out = body(ctx);
    if !ctx.ctx.scopes.is_empty() {
        ctx.ctx.scopes.remove(0);
    }
    out
}

/// Runs `body` with the scope list replaced, and puts the old list back afterwards.
pub fn with_scopes<'c, 'a, T>(
    ctx: &mut ResolveContext<'c, 'a>,
    replacement: ScopeList,
    body: impl FnOnce(&mut ResolveContext<'c, 'a>) -> T,
) -> T {
    let saved = std::mem::replace(&mut ctx.ctx.scopes, replacement);
    let out = body(ctx);
    ctx.ctx.scopes = saved;
    out
}

/// The scopes of a procedure-like body: the enclosing locals, the procedure's own scope,
/// then the module and universe scopes.
pub fn make_proc_like_scopes(base: &ScopeContext<'_>, proc_scope: Scope) -> ScopeList {
    let mut scopes: ScopeList = base.local_scopes().to_vec();
    scopes.push(proc_scope);
    scopes.push(base.module_scope().clone());
    scopes.push(base.universe_scope().clone());
    scopes
}

pub fn full_path(path: &[String], name: &str) -> Vec<String> {
    let mut out = path.to_vec();
    out.push(name.to_string());
    out
}

pub fn make_expr(span: &Span, node: ExprNode) -> ExprPtr {
    Some(std::sync::Arc::new(Expr { span: span.clone(), node }))
}

pub fn find_record_decl<'s>(ctx: &'s ScopeContext<'_>, path: &[String]) -> Option<&'s RecordDecl> {
    match ctx.sigma.types.get(&super::scopes::path_key_of(path)) {
        Some(TypeDecl::Record(decl)) => Some(decl),
        _ => None,
    }
}

pub fn find_enum_decl<'s>(ctx: &'s ScopeContext<'_>, path: &[String]) -> Option<&'s EnumDecl> {
    match ctx.sigma.types.get(&super::scopes::path_key_of(path)) {
        Some(TypeDecl::Enum(decl)) => Some(decl),
        _ => None,
    }
}
