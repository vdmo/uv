//! The facts the language server answers from: the symbols the program declares, and
//! the references to them that name resolution finds. See `language_service/facts.cpp`.

use std::collections::{HashMap, HashSet};

use uv_core::span::Span;
use uv_source::ast::{self, pretty::type_to_string, ASTItem, ASTModule};
use uv_source::lexer::DocComment;

use crate::context::{Entity, EntityKind, EntitySource, ScopeContext};
use crate::resolve::collect_toplevel::pat_names_ptr;
use crate::typing::expr_store::TypeStores;
use crate::typing::types::TypeRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageSymbolKind {
    Module,
    Function,
    Method,
    Variable,
    Constant,
    Field,
    Record,
    Enum,
    EnumMember,
    Modal,
    Class,
    TypeAlias,
    State,
    Parameter,
}

pub fn language_symbol_kind_name(kind: LanguageSymbolKind) -> &'static str {
    match kind {
        LanguageSymbolKind::Module => "module",
        LanguageSymbolKind::Function => "procedure",
        LanguageSymbolKind::Method => "method",
        LanguageSymbolKind::Variable => "variable",
        LanguageSymbolKind::Constant => "constant",
        LanguageSymbolKind::Field => "field",
        LanguageSymbolKind::Record => "record",
        LanguageSymbolKind::Enum => "enum",
        LanguageSymbolKind::EnumMember => "enum variant",
        LanguageSymbolKind::Modal => "modal",
        LanguageSymbolKind::Class => "class",
        LanguageSymbolKind::TypeAlias => "type alias",
        LanguageSymbolKind::State => "state",
        LanguageSymbolKind::Parameter => "parameter",
    }
}

#[derive(Debug, Clone)]
pub struct LanguageParameterInfo {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct LanguageSymbolInfo {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    pub module_path: String,
    pub kind: LanguageSymbolKind,
    pub range: Span,
    pub selection_range: Span,
    pub detail: String,
    pub documentation: String,
    pub r#type: TypeRef,
    pub signature_label: String,
    pub parameters: Vec<LanguageParameterInfo>,
    pub is_local: bool,
    pub include_in_outline: bool,
    pub include_in_workspace: bool,
}

#[derive(Debug, Clone)]
pub struct LanguageReference {
    pub symbol_id: String,
    pub range: Span,
    pub is_declaration: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LanguageServiceIndex {
    symbols: Vec<LanguageSymbolInfo>,
    references: Vec<LanguageReference>,
    symbol_offsets: HashMap<String, usize>,
}

fn file_key(file: &str) -> String {
    let path = std::path::Path::new(file);
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if parts.last().is_some_and(|last| last != "/" && last != "..") {
                    parts.pop();
                } else {
                    parts.push("..".to_string());
                }
            }
            other => parts.push(other.as_os_str().to_string_lossy().into_owned()),
        }
    }
    let mut key = parts.join("/");
    if key.starts_with("//") {
        key.remove(0);
    }
    key
}

fn same_file(span: &Span, path: &str) -> bool {
    file_key(&span.file) == file_key(path)
}

fn contains(span: &Span, path: &str, offset: usize) -> bool {
    same_file(span, path) && span.start_offset <= offset && offset <= span.end_offset
}

fn width(span: &Span) -> usize {
    span.end_offset.saturating_sub(span.start_offset)
}

fn doc_text(docs: &[DocComment]) -> String {
    docs.iter().map(|doc| doc.text.as_str()).collect::<Vec<_>>().join("\n")
}

fn doc_text_opt(docs: &Option<Vec<DocComment>>) -> String {
    docs.as_deref().map(doc_text).unwrap_or_default()
}

fn ast_type_text(ty: &ast::TypePtr) -> String {
    match ty {
        Some(ty) => type_to_string(ty),
        None => "()".to_string(),
    }
}

fn param_label(param: &ast::Param) -> String {
    let mut out = String::new();
    if param.mode.is_some() {
        out.push_str("move ");
    }
    out.push_str(&param.name);
    out.push_str(": ");
    out.push_str(&ast_type_text(&param.r#type));
    out
}

fn parameter_info(params: &[ast::Param]) -> Vec<LanguageParameterInfo> {
    params.iter().map(|param| LanguageParameterInfo { name: param.name.clone(), label: param_label(param) }).collect()
}

fn callable_signature_label(name: &str, params: &[ast::Param], return_type_opt: &ast::TypePtr) -> String {
    let mut out = format!("{name}({})", params.iter().map(param_label).collect::<Vec<_>>().join(", "));
    if return_type_opt.is_some() {
        out.push_str(" -> ");
        out.push_str(&ast_type_text(return_type_opt));
    }
    out
}

fn qualified(module_path: &str, name: &str) -> String {
    if module_path.is_empty() {
        name.to_string()
    } else {
        format!("{module_path}::{name}")
    }
}

fn entity_kind_prefix(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Value => "value",
        EntityKind::Type => "type",
        EntityKind::Class => "class",
        EntityKind::ModuleAlias => "module",
    }
}

fn entity_kind_for_symbol_kind(kind: LanguageSymbolKind) -> EntityKind {
    match kind {
        LanguageSymbolKind::Record | LanguageSymbolKind::Enum | LanguageSymbolKind::Modal | LanguageSymbolKind::TypeAlias | LanguageSymbolKind::State => EntityKind::Type,
        LanguageSymbolKind::Class => EntityKind::Class,
        LanguageSymbolKind::Module => EntityKind::ModuleAlias,
        _ => EntityKind::Value,
    }
}

fn top_level_symbol_id(kind: EntityKind, module_path: &[String], name: &str) -> String {
    format!("{}:{}", entity_kind_prefix(kind), qualified(&module_path.join("::"), name))
}

fn member_symbol_id(module_path: &str, owner: &str, name: &str) -> String {
    let mut out = "member:".to_string();
    if !module_path.is_empty() {
        out.push_str(module_path);
        out.push_str("::");
    }
    out.push_str(owner);
    out.push_str("::");
    out.push_str(name);
    out
}

fn local_symbol_id(name: &str, span: &Span) -> String {
    format!("local:{}:{}:{}:{}", file_key(&span.file), span.start_offset, span.end_offset, name)
}

fn symbol_id_for_entity(entity: &Entity, fallback_name: &str) -> Option<String> {
    if !entity.language_symbol_id.is_empty() {
        return Some(entity.language_symbol_id.clone());
    }
    let origin = entity.origin_opt.as_ref()?;
    let name = entity.target_opt.clone().unwrap_or_else(|| fallback_name.to_string());
    if name.is_empty() {
        return None;
    }
    Some(top_level_symbol_id(entity.kind, origin, &name))
}

fn module_path_at(symbols: &[LanguageSymbolInfo], path: &str, offset: usize) -> Option<String> {
    let mut best: Option<&LanguageSymbolInfo> = None;
    for symbol in symbols {
        if symbol.module_path.is_empty() || !contains(&symbol.range, path, offset) {
            continue;
        }
        if best.is_none_or(|best| width(&symbol.range) < width(&best.range)) {
            best = Some(symbol);
        }
    }
    if let Some(best) = best {
        return Some(best.module_path.clone());
    }
    symbols.iter().find(|symbol| !symbol.module_path.is_empty() && same_file(&symbol.range, path)).map(|symbol| symbol.module_path.clone())
}

impl LanguageServiceIndex {
    pub fn symbols(&self) -> &[LanguageSymbolInfo] {
        &self.symbols
    }

    pub fn references(&self) -> &[LanguageReference] {
        &self.references
    }

    pub fn add_symbol(&mut self, symbol: LanguageSymbolInfo) {
        if symbol.id.is_empty() || self.symbol_offsets.contains_key(&symbol.id) {
            return;
        }
        self.symbol_offsets.insert(symbol.id.clone(), self.symbols.len());
        self.symbols.push(symbol);
    }

    pub fn add_reference(&mut self, reference: LanguageReference) {
        if !reference.symbol_id.is_empty() {
            self.references.push(reference);
        }
    }

    pub fn symbol_by_id(&self, id: &str) -> Option<&LanguageSymbolInfo> {
        self.symbol_offsets.get(id).map(|&at| &self.symbols[at])
    }

    pub fn symbols_in_file(&self, path: &str) -> Vec<&LanguageSymbolInfo> {
        let mut out: Vec<&LanguageSymbolInfo> = self.symbols.iter().filter(|symbol| same_file(&symbol.range, path)).collect();
        out.sort_by_key(|symbol| symbol.range.start_offset);
        out
    }

    pub fn symbol_at(&self, path: &str, offset: usize) -> Option<&LanguageSymbolInfo> {
        let mut best: Option<&LanguageSymbolInfo> = None;
        for symbol in &self.symbols {
            if !contains(&symbol.selection_range, path, offset) {
                continue;
            }
            if best.is_none_or(|best| width(&symbol.selection_range) < width(&best.selection_range)) {
                best = Some(symbol);
            }
        }
        best
    }

    pub fn reference_at(&self, path: &str, offset: usize) -> Option<&LanguageReference> {
        let mut best: Option<&LanguageReference> = None;
        for reference in &self.references {
            if !contains(&reference.range, path, offset) {
                continue;
            }
            if best.is_none_or(|best| width(&reference.range) < width(&best.range)) {
                best = Some(reference);
            }
        }
        best
    }

    pub fn resolved_symbol_at(&self, path: &str, offset: usize) -> Option<&LanguageSymbolInfo> {
        if let Some(symbol) = self.reference_at(path, offset).and_then(|reference| self.symbol_by_id(&reference.symbol_id)) {
            return Some(symbol);
        }
        self.symbol_at(path, offset)
    }

    pub fn references_for_symbol(&self, symbol_id: &str, include_declaration: bool) -> Vec<&LanguageReference> {
        let mut out: Vec<&LanguageReference> =
            self.references.iter().filter(|reference| reference.symbol_id == symbol_id && (include_declaration || !reference.is_declaration)).collect();
        out.sort_by(|a, b| file_key(&a.range.file).cmp(&file_key(&b.range.file)).then(a.range.start_offset.cmp(&b.range.start_offset)));
        out
    }

    /// What may be named at a point: locals declared before it (nearest first), then the
    /// symbols of the same module, then the rest of the workspace.
    pub fn completion_symbols(&self, path: &str, offset: usize) -> Vec<&LanguageSymbolInfo> {
        let mut locals = Vec::new();
        let mut module_symbols = Vec::new();
        let mut workspace = Vec::new();
        let module_path = module_path_at(&self.symbols, path, offset);
        for symbol in &self.symbols {
            if symbol.is_local {
                if same_file(&symbol.range, path) && symbol.range.start_offset <= offset {
                    locals.push(symbol);
                }
                continue;
            }
            if module_path.as_deref() == Some(symbol.module_path.as_str()) {
                module_symbols.push(symbol);
                continue;
            }
            if symbol.include_in_workspace {
                workspace.push(symbol);
            }
        }
        locals.sort_by_key(|symbol| std::cmp::Reverse(symbol.range.start_offset));
        let mut seen = HashSet::new();
        locals.into_iter().chain(module_symbols).chain(workspace).filter(|symbol| seen.insert(symbol.id.clone())).collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn add_declaration(
        &mut self,
        id: String,
        module: &[String],
        name: &str,
        kind: LanguageSymbolKind,
        range: &Span,
        detail: &str,
        documentation: String,
        signature_label: String,
        parameters: Vec<LanguageParameterInfo>,
        qualified_name: Option<String>,
    ) {
        let module_path = module.join("::");
        let symbol = LanguageSymbolInfo {
            id: id.clone(),
            name: name.to_string(),
            qualified_name: qualified_name.unwrap_or_else(|| qualified(&module_path, name)),
            module_path,
            kind,
            range: range.clone(),
            selection_range: range.clone(),
            detail: detail.to_string(),
            documentation,
            r#type: None,
            signature_label,
            parameters,
            is_local: false,
            include_in_outline: true,
            include_in_workspace: true,
        };
        self.add_symbol(symbol);
        self.add_reference(LanguageReference { symbol_id: id, range: range.clone(), is_declaration: true });
    }

    #[allow(clippy::too_many_arguments)]
    fn add_top_level(&mut self, module: &[String], name: &str, entity_kind: EntityKind, kind: LanguageSymbolKind, range: &Span, detail: &str, documentation: String) {
        let id = top_level_symbol_id(entity_kind, module, name);
        self.add_declaration(id, module, name, kind, range, detail, documentation, String::new(), Vec::new(), None);
    }

    #[allow(clippy::too_many_arguments)]
    fn add_callable(&mut self, module: &[String], name: &str, kind: LanguageSymbolKind, range: &Span, detail: &str, documentation: String, label: String, params: Vec<LanguageParameterInfo>) {
        let id = top_level_symbol_id(EntityKind::Value, module, name);
        self.add_declaration(id, module, name, kind, range, detail, documentation, label, params, None);
    }

    #[allow(clippy::too_many_arguments)]
    fn add_member(&mut self, module: &[String], owner: &str, name: &str, kind: LanguageSymbolKind, range: &Span, detail: &str, documentation: String, label: String, params: Vec<LanguageParameterInfo>) {
        let module_path = module.join("::");
        let id = member_symbol_id(&module_path, owner, name);
        let qualified_name = qualified(&module_path, &format!("{owner}::{name}"));
        self.add_declaration(id, module, name, kind, range, detail, documentation, label, params, Some(qualified_name));
    }
}

/// The symbols every declaration of the modules introduces, before resolution.
pub fn build_language_service_declarations(modules: &[ASTModule]) -> LanguageServiceIndex {
    use LanguageSymbolKind as K;
    let mut index = LanguageServiceIndex::default();
    for module in modules {
        let m = &module.path;
        for item in &module.items {
            match item {
                ASTItem::ProcedureDecl(node) => index.add_callable(
                    m, &node.name, K::Function, &node.span, "procedure", doc_text(&node.doc),
                    callable_signature_label(&node.name, &node.params, &node.return_type_opt), parameter_info(&node.params),
                ),
                ASTItem::ComptimeProcedureDecl(node) => index.add_callable(
                    m, &node.name, K::Function, &node.span, "comptime procedure", doc_text(&node.doc),
                    callable_signature_label(&node.name, &node.params, &node.return_type_opt), parameter_info(&node.params),
                ),
                ASTItem::StaticDecl(node) => {
                    if let Some(pattern) = &node.binding.pat {
                        for name in pat_names_ptr(&Some(pattern.clone())) {
                            index.add_top_level(m, &name, EntityKind::Value, K::Constant, &pattern.span, "module storage", String::new());
                        }
                    }
                }
                ASTItem::RecordDecl(node) => {
                    index.add_top_level(m, &node.name, EntityKind::Type, K::Record, &node.span, "record", doc_text(&node.doc));
                    for member in &node.members {
                        match member {
                            ast::RecordMember::FieldDecl(f) => index.add_member(m, &node.name, &f.name, K::Field, &f.span, "record field", doc_text_opt(&f.doc_opt), String::new(), Vec::new()),
                            ast::RecordMember::MethodDecl(f) => index.add_member(
                                m, &node.name, &f.name, K::Method, &f.span, "record method", doc_text_opt(&f.doc_opt),
                                callable_signature_label(&f.name, &f.params, &f.return_type_opt), parameter_info(&f.params),
                            ),
                            ast::RecordMember::AssociatedTypeDecl(f) => {
                                index.add_member(m, &node.name, &f.name, K::TypeAlias, &f.span, "associated type", doc_text_opt(&f.doc_opt), String::new(), Vec::new())
                            }
                        }
                    }
                }
                ASTItem::EnumDecl(node) => {
                    index.add_top_level(m, &node.name, EntityKind::Type, K::Enum, &node.span, "enum", doc_text(&node.doc));
                    for variant in &node.variants {
                        index.add_member(m, &node.name, &variant.name, K::EnumMember, &variant.span, "enum variant", doc_text_opt(&variant.doc_opt), String::new(), Vec::new());
                    }
                }
                ASTItem::ModalDecl(node) => {
                    index.add_top_level(m, &node.name, EntityKind::Type, K::Modal, &node.span, "modal", doc_text(&node.doc));
                    for state in &node.states {
                        index.add_member(m, &node.name, &state.name, K::State, &state.span, "modal state", doc_text_opt(&state.doc_opt), String::new(), Vec::new());
                        let owner = format!("{}::{}", node.name, state.name);
                        for member in &state.members {
                            match member {
                                ast::StateMember::StateFieldDecl(f) => {
                                    index.add_member(m, &owner, &f.name, K::Field, &f.span, "modal state field", doc_text_opt(&f.doc_opt), String::new(), Vec::new())
                                }
                                ast::StateMember::StateMethodDecl(f) => index.add_member(
                                    m, &owner, &f.name, K::Method, &f.span, "modal state method", doc_text_opt(&f.doc_opt),
                                    callable_signature_label(&f.name, &f.params, &f.return_type_opt), parameter_info(&f.params),
                                ),
                                ast::StateMember::TransitionDecl(f) => index.add_member(
                                    m, &owner, &f.name, K::Method, &f.span, "modal transition", doc_text_opt(&f.doc_opt),
                                    callable_signature_label(&f.name, &f.params, &None), parameter_info(&f.params),
                                ),
                            }
                        }
                    }
                }
                ASTItem::ClassDecl(node) => {
                    index.add_top_level(m, &node.name, EntityKind::Class, K::Class, &node.span, if node.modal { "modal class" } else { "class" }, doc_text(&node.doc));
                    for class_item in &node.items {
                        match class_item {
                            ast::ClassItem::ClassFieldDecl(f) => index.add_member(m, &node.name, &f.name, K::Field, &f.span, "class field", doc_text_opt(&f.doc_opt), String::new(), Vec::new()),
                            ast::ClassItem::ClassMethodDecl(f) => index.add_member(
                                m, &node.name, &f.name, K::Method, &f.span, "class method", doc_text_opt(&f.doc_opt),
                                callable_signature_label(&f.name, &f.params, &f.return_type_opt), parameter_info(&f.params),
                            ),
                            ast::ClassItem::AssociatedTypeDecl(f) => {
                                index.add_member(m, &node.name, &f.name, K::TypeAlias, &f.span, "associated type", doc_text_opt(&f.doc_opt), String::new(), Vec::new())
                            }
                            ast::ClassItem::AbstractFieldDecl(f) => index.add_member(m, &node.name, &f.name, K::Field, &f.span, "abstract field", doc_text_opt(&f.doc_opt), String::new(), Vec::new()),
                            ast::ClassItem::AbstractStateDecl(f) => {
                                index.add_member(m, &node.name, &f.name, K::State, &f.span, "abstract state", doc_text_opt(&f.doc_opt), String::new(), Vec::new());
                                let owner = format!("{}::{}", node.name, f.name);
                                for field in &f.fields {
                                    index.add_member(m, &owner, &field.name, K::Field, &field.span, "abstract state field", doc_text_opt(&field.doc_opt), String::new(), Vec::new());
                                }
                            }
                        }
                    }
                }
                ASTItem::TypeAliasDecl(node) => index.add_top_level(m, &node.name, EntityKind::Type, K::TypeAlias, &node.span, "type alias", doc_text(&node.doc)),
                _ => {}
            }
        }
    }
    index
}

/// A local name the resolver introduces: its declaration goes into the index, and the
/// entity that stands for it carries the symbol's id.
pub fn make_language_service_local_entity(
    index: Option<&mut LanguageServiceIndex>,
    ctx: &ScopeContext<'_>,
    name: &str,
    declaration_span: &Span,
    kind: LanguageSymbolKind,
    detail: &str,
) -> Entity {
    let id = local_symbol_id(name, declaration_span);
    if let Some(index) = index {
        let module_path = ctx.current_module.join("::");
        index.add_symbol(LanguageSymbolInfo {
            id: id.clone(),
            name: name.to_string(),
            qualified_name: qualified(&module_path, name),
            module_path,
            kind,
            range: declaration_span.clone(),
            selection_range: declaration_span.clone(),
            detail: detail.to_string(),
            documentation: String::new(),
            r#type: None,
            signature_label: String::new(),
            parameters: Vec::new(),
            is_local: true,
            include_in_outline: kind != LanguageSymbolKind::Parameter,
            include_in_workspace: false,
        });
        index.add_reference(LanguageReference { symbol_id: id.clone(), range: declaration_span.clone(), is_declaration: true });
    }
    let mut entity = Entity::new(entity_kind_for_symbol_kind(kind), None, None, EntitySource::Decl);
    entity.declaration_span = Some(declaration_span.clone());
    entity.language_symbol_id = id;
    entity
}

pub fn record_language_service_reference(index: Option<&mut LanguageServiceIndex>, fallback_name: &str, reference_span: &Span, entity: &Entity) {
    let Some(index) = index else {
        return;
    };
    if let Some(symbol_id) = symbol_id_for_entity(entity, fallback_name) {
        index.add_reference(LanguageReference { symbol_id, range: reference_span.clone(), is_declaration: false });
    }
}

pub fn record_language_service_type_path_reference(index: Option<&mut LanguageServiceIndex>, path: &[String], reference_span: &Span) {
    let (Some(index), Some(last)) = (index, path.last()) else {
        return;
    };
    let mut entity = Entity::new(EntityKind::Type, Some(path[..path.len() - 1].to_vec()), Some(last.clone()), EntitySource::Decl);
    entity.language_symbol_id = String::new();
    record_language_service_reference(Some(index), last, reference_span, &entity);
}

pub fn record_language_service_member_reference(index: Option<&mut LanguageServiceIndex>, owner_path: &[String], member_name: &str, reference_span: &Span) {
    let (Some(index), Some(owner)) = (index, owner_path.last()) else {
        return;
    };
    let symbol_id = member_symbol_id(&owner_path[..owner_path.len() - 1].join("::"), owner, member_name);
    index.add_reference(LanguageReference { symbol_id, range: reference_span.clone(), is_declaration: false });
}

/// The type of the smallest expression that covers the point.
pub fn language_service_type_at(stores: &TypeStores, path: &str, offset: usize) -> TypeRef {
    let mut best: Option<(usize, TypeRef)> = None;
    for (expr, ty) in stores.expr_types.borrow().values() {
        if ty.is_none() || !contains(&expr.span, path, offset) {
            continue;
        }
        let w = width(&expr.span);
        if best.as_ref().is_none_or(|(best_width, _)| w < *best_width) {
            best = Some((w, ty.clone()));
        }
    }
    best.and_then(|(_, ty)| ty)
}
