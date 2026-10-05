//! Compile-time values and the environment a compile-time evaluation runs in.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::span::Span;
use uv_core::spec_rule_at;
use uv_source::ast::*;
use uv_source::lexer::{Token, TokenKind};
use uv_source::module_paths::ModuleNames;

use crate::files::ProjectFileSnapshot;

#[derive(Debug, Clone, PartialEq)]
pub struct CtPrimInt {
    pub value: u64,
    pub suffix: String,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CtPrim {
    Unit,
    Bool(bool),
    Int(CtPrimInt),
    /// The literal's lexeme.
    Float(String),
    /// The literal's lexeme.
    Char(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtAstKind {
    Expr,
    Stmt,
    Item,
    Type,
    Pattern,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CtSite {
    pub module_path: Vec<String>,
    pub ordinal: usize,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CtHygiene {
    pub quote_site: CtSite,
    pub emit_site: CtSite,
    pub mark: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CtAstPayload {
    Expr(ExprPtr),
    Stmt(Box<Stmt>),
    Item(Box<ASTItem>),
    Type(TypePtr),
    Pattern(PatternPtr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CtAst {
    pub kind: CtAstKind,
    pub payload: CtAstPayload,
    pub span: Option<Span>,
    pub hygiene: Option<Box<CtHygiene>>,
}

pub type CtFields = Vec<(Identifier, CtValue)>;

#[derive(Debug, Clone, PartialEq)]
pub enum CtPayload {
    None,
    Tuple(Vec<CtValue>),
    Record(CtFields),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CtRecord {
    pub path: Path,
    pub fields: CtFields,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CtModalState {
    pub target: ModalStateRef,
    pub fields: CtFields,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CtEnum {
    pub path: Path,
    pub variant: Identifier,
    pub payload: CtPayload,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CtValue {
    Prim(CtPrim),
    String(String),
    Bytes(Vec<u8>),
    Type(TypePtr),
    Ast(CtAst),
    Tuple(Rc<Vec<CtValue>>),
    Array(Rc<Vec<CtValue>>),
    Slice(Rc<Vec<CtValue>>),
    Record(Rc<CtRecord>),
    ModalState(Rc<CtModalState>),
    Enum(Rc<CtEnum>),
}

impl Default for CtValue {
    fn default() -> Self {
        make_ct_unit()
    }
}

#[derive(Debug, Clone, Default)]
pub struct EvalResult {
    pub ok: bool,
    pub value: CtValue,
    pub returned: bool,
}

/// The quote being built. The pass records it for the duration of a quote; nothing in
/// phase 2 reads it back.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CtQuoteCtx {
    pub kind: QuoteKind,
    pub quote_site: CtSite,
}

#[derive(Clone, Default)]
pub struct CtEnv {
    pub values: HashMap<String, CtValue>,
    pub procs: HashMap<String, Rc<ComptimeProcedureDecl>>,
    pub caps: Vec<String>,
    pub site: CtSite,
    pub quote_ctx: Option<CtQuoteCtx>,
    pub available_modules: Vec<Rc<ASTModule>>,
    pub phase2_expanded_modules: Vec<Rc<ASTModule>>,
    pub available_module_names: ModuleNames,
    pub current_module_items: Option<Rc<RefCell<Vec<ASTItem>>>>,
    pub pending_emits: Option<Rc<RefCell<Vec<ASTItem>>>>,
    pub diags: Option<Rc<RefCell<DiagnosticStream>>>,
    pub project_root: String,
    pub source_root: String,
    pub next_hygiene: usize,
    pub current_module: Vec<String>,
    pub current_item_index: usize,
    pub current_span: Span,
    pub contract_entry_values: Option<Rc<HashMap<String, CtValue>>>,
    pub contract_result_value: Option<CtValue>,
    pub files: Option<Rc<ProjectFileSnapshot>>,
    pub return_quote_kind: Option<QuoteKind>,
    pub quote_splice_trace: Vec<String>,
}

pub fn append_diags(out: &mut DiagnosticStream, add: &[uv_core::diagnostics::Diagnostic]) {
    for diag in add {
        emit(out, diag.clone());
    }
}

pub fn emit_comptime_diag(env: &CtEnv, diag_id: &str, span: &Span) {
    let Some(diags) = &env.diags else {
        return;
    };
    if let Some(diag) = make_diagnostic_by_id(diag_id, Some(span.clone())) {
        emit(&mut diags.borrow_mut(), diag);
    }
}

pub fn has_attribute(attrs: &[AttributeItem], name: &str) -> bool {
    attrs.iter().any(|attr| attr.name.full_name == name)
}

pub fn find_attribute<'a>(attrs: &'a [AttributeItem], name: &str) -> Option<&'a AttributeItem> {
    attrs.iter().find(|attr| attr.name.full_name == name)
}

pub fn strip_attribute(attrs: &[AttributeItem], name: &str) -> AttributeList {
    attrs.iter().filter(|attr| attr.name.full_name != name).cloned().collect()
}

pub fn ct_empty_env(module: &ASTModule) -> CtEnv {
    let mut env = CtEnv { current_module: module.path.clone(), ..CtEnv::default() };
    env.site.module_path = module.path.clone();
    env
}

/// Installs the capabilities a compile-time block has: introspection and diagnostics
/// always, the emitter under `#emit` (or in a derive body) and project files under `#files`.
pub fn with_ct_caps(mut env: CtEnv, attrs: &[AttributeItem], derive_body: bool) -> CtEnv {
    spec_rule_at!("def.22.CompileTimeTypingEnvironment", &env.current_span);
    spec_rule_at!("def.22.CtCapabilitiesAndBuiltinTypes", &env.current_span);
    spec_rule_at!("requirement.22.IntrospectAndDiagnosticsAvailability", &env.current_span);
    env.caps.clear();
    env.values.insert("introspect".to_string(), make_ct_unit());
    env.values.insert("diagnostics".to_string(), make_ct_unit());
    env.caps.push("Introspect".to_string());
    env.caps.push("ComptimeDiagnostics".to_string());
    if derive_body || has_attribute(attrs, "emit") {
        spec_rule_at!("requirement.22.TypeEmitterAvailability", &env.current_span);
        env.values.insert("emitter".to_string(), make_ct_unit());
        env.caps.push("TypeEmitter".to_string());
    }
    if has_attribute(attrs, "files") {
        if env.files.is_none() {
            emit_comptime_diag(&env, "E-CTE-0060", &env.current_span);
            return env;
        }
        spec_rule_at!("requirement.22.ProjectFilesAvailability", &env.current_span);
        env.values.insert("files".to_string(), make_ct_unit());
        env.caps.push("ProjectFiles".to_string());
    }
    env
}

pub fn with_ct_site(mut env: CtEnv, ordinal: usize, span: &Span) -> CtEnv {
    env.site.ordinal = ordinal;
    env.site.span = span.clone();
    env.current_item_index = ordinal;
    env.current_span = span.clone();
    env
}

pub fn bind_ct_proc(mut env: CtEnv, proc: &ComptimeProcedureDecl) -> CtEnv {
    env.procs.insert(proc.name.clone(), Rc::new(proc.clone()));
    env
}

pub fn ast_of(kind: CtAstKind, payload: CtAstPayload) -> CtAst {
    CtAst { kind, payload, span: None, hygiene: None }
}

pub fn make_ct_unit() -> CtValue {
    CtValue::Prim(CtPrim::Unit)
}

pub fn make_ct_bool(value: bool) -> CtValue {
    CtValue::Prim(CtPrim::Bool(value))
}

/// An integer value; without an explicit lexeme it is spelled `<value><suffix>`.
pub fn make_ct_int(value: u64, suffix: &str, lexeme: &str) -> CtValue {
    let lexeme = if lexeme.is_empty() { format!("{value}{suffix}") } else { lexeme.to_string() };
    CtValue::Prim(CtPrim::Int(CtPrimInt { value, suffix: suffix.to_string(), lexeme }))
}

pub fn make_ct_float(lexeme: &str) -> CtValue {
    CtValue::Prim(CtPrim::Float(lexeme.to_string()))
}

pub fn make_ct_char(lexeme: &str) -> CtValue {
    CtValue::Prim(CtPrim::Char(lexeme.to_string()))
}

pub fn try_get_ct_bool(value: &CtValue) -> Option<bool> {
    match value {
        CtValue::Prim(CtPrim::Bool(value)) => Some(*value),
        _ => None,
    }
}

pub fn try_get_ct_int(value: &CtValue) -> Option<&CtPrimInt> {
    match value {
        CtValue::Prim(CtPrim::Int(int)) => Some(int),
        _ => None,
    }
}

pub fn ct_elems(value: &CtValue) -> Option<&[CtValue]> {
    match value {
        CtValue::Array(elements) | CtValue::Slice(elements) => Some(elements),
        _ => None,
    }
}

fn make_expr(span: &Span, node: ExprNode) -> ExprPtr {
    Some(std::sync::Arc::new(Expr { span: span.clone(), node }))
}

fn literal_expr(span: &Span, kind: TokenKind, lexeme: String) -> ExprPtr {
    let literal = Token { kind, lexeme, span: span.clone() };
    make_expr(span, LiteralExpr { literal }.into())
}

fn literalize_fields(fields: &CtFields, span: &Span) -> Option<Vec<FieldInit>> {
    fields
        .iter()
        .map(|(name, value)| {
            let value = literalize_value(value, span)?;
            Some(FieldInit { name: name.clone(), value: Some(value), span: span.clone() })
        })
        .collect()
}

fn literalize_all(values: &[CtValue], span: &Span) -> Option<Vec<ExprPtr>> {
    values.iter().map(|value| literalize_value(value, span).map(Some)).collect()
}

pub fn make_span_value(span: &Span) -> CtValue {
    let usize_field = |name: &str, value: usize| (name.to_string(), make_ct_int(value as u64, "usize", ""));
    CtValue::Record(Rc::new(CtRecord {
        path: vec!["SourceSpan".to_string()],
        fields: vec![
            ("file".to_string(), CtValue::String(span.file.to_string())),
            usize_field("start_line", span.start_line),
            usize_field("start_col", span.start_col),
            usize_field("end_line", span.end_line),
            usize_field("end_col", span.end_col),
        ],
    }))
}

/// The expression that denotes a compile-time value, or `None` when the value has no
/// literal form (bytes, types, non-expression syntax).
pub fn literalize_value(value: &CtValue, span: &Span) -> Option<std::sync::Arc<Expr>> {
    match value {
        CtValue::Prim(CtPrim::Unit) => make_expr(span, TupleExpr::default().into()),
        CtValue::Prim(CtPrim::Bool(value)) => {
            literal_expr(span, TokenKind::BoolLiteral, value.to_string())
        }
        CtValue::Prim(CtPrim::Int(int)) => literal_expr(span, TokenKind::IntLiteral, int.lexeme.clone()),
        CtValue::Prim(CtPrim::Float(lexeme)) => {
            literal_expr(span, TokenKind::FloatLiteral, lexeme.clone())
        }
        CtValue::Prim(CtPrim::Char(lexeme)) => literal_expr(span, TokenKind::CharLiteral, lexeme.clone()),
        CtValue::String(text) => literal_expr(span, TokenKind::StringLiteral, format!("\"{text}\"")),
        CtValue::Bytes(_) | CtValue::Type(_) | CtValue::Slice(_) => None,
        CtValue::Tuple(elements) => {
            let elements = literalize_all(elements, span)?;
            make_expr(span, TupleExpr { elements }.into())
        }
        CtValue::Array(elements) => {
            let elements = literalize_all(elements, span)?
                .into_iter()
                .map(|value| ArrayElemSegment { value }.into())
                .collect();
            make_expr(span, ArrayExpr { elements }.into())
        }
        CtValue::Record(record) => {
            let fields = literalize_fields(&record.fields, span)?;
            make_expr(span, RecordExpr { target: record.path.clone().into(), fields }.into())
        }
        CtValue::ModalState(state) => {
            let fields = literalize_fields(&state.fields, span)?;
            make_expr(span, RecordExpr { target: state.target.clone().into(), fields }.into())
        }
        CtValue::Enum(value) => {
            let mut path = value.path.clone();
            path.push(value.variant.clone());
            let payload_opt = match &value.payload {
                CtPayload::None => None,
                CtPayload::Tuple(elements) => {
                    Some(EnumPayloadParen { elements: literalize_all(elements, span)? }.into())
                }
                CtPayload::Record(fields) => {
                    Some(EnumPayloadBrace { fields: literalize_fields(fields, span)? }.into())
                }
            };
            make_expr(span, EnumLiteralExpr { path, payload_opt }.into())
        }
        CtValue::Ast(ast) => match (&ast.kind, &ast.payload) {
            (CtAstKind::Expr, CtAstPayload::Expr(expr)) => expr.clone(),
            _ => None,
        },
    }
}

pub fn span_of_item(item: &ASTItem) -> Span {
    item_span(item).clone()
}
