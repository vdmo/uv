//! The context semantic analysis runs in: entities, scopes and the declaration tables.

use std::collections::{BTreeMap, HashMap};

use uv_core::diagnostics::DiagnosticStream;
use uv_core::span::Span;
use uv_core::std_unordered::UnorderedMap;
use uv_project::project::Project;
use uv_project::target_profile::TargetProfile;
use uv_source::ast::*;
use uv_source::module_paths::ModuleNames;

/// An identifier in NFC, the form names are compared in.
pub type IdKey = String;
pub type PathKey = Vec<IdKey>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Value,
    Type,
    Class,
    ModuleAlias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitySource {
    Decl,
    Using,
    RegionAlias,
    Import,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub kind: EntityKind,
    pub origin_opt: Option<Vec<String>>,
    pub target_opt: Option<Identifier>,
    pub source: EntitySource,
    pub declaration_span: Option<Span>,
    pub language_symbol_id: String,
    pub type_param_class_bounds: Vec<TypeBound>,
    pub visibility: Option<Visibility>,
}

impl Entity {
    pub fn new(kind: EntityKind, origin_opt: Option<Vec<String>>, target_opt: Option<Identifier>, source: EntitySource) -> Entity {
        Entity {
            kind,
            origin_opt,
            target_opt,
            source,
            declaration_span: None,
            language_symbol_id: String::new(),
            type_param_class_bounds: Vec::new(),
            visibility: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDecl {
    Record(RecordDecl),
    Enum(EnumDecl),
    Modal(ModalDecl),
    TypeAlias(TypeAliasDecl),
}

/// The program as analysis sees it: the modules and the nominal declarations by path.
#[derive(Debug, Clone, Default)]
pub struct Sigma {
    pub mods: Vec<ASTModule>,
    pub unsafe_spans_by_file: HashMap<String, Vec<Span>>,
    pub types: BTreeMap<PathKey, TypeDecl>,
    pub classes: BTreeMap<PathKey, ClassDecl>,
    /// What each `opaque` return type stands for, once the type checker has found out.
    pub opaque_underlying_by_class_path: BTreeMap<PathKey, crate::typing::types::TypeRef>,
}

/// Names to entities. Iteration follows the reference's hash table, which decides which
/// of several equally close names a suggestion offers.
pub type Scope = UnorderedMap<Entity>;
pub type ScopeList = Vec<Scope>;
pub type NameMap = Scope;
pub type NameMapTable = BTreeMap<PathKey, NameMap>;
pub type VisibleModuleNameTable = BTreeMap<PathKey, ModuleNames>;

/// Tables computed once per project so that lookups need not recompute them.
#[derive(Clone, Copy)]
pub struct NameResolutionTables<'a> {
    pub name_maps: Option<&'a NameMapTable>,
    pub module_names: Option<&'a ModuleNames>,
    pub visible_module_names: Option<&'a VisibleModuleNameTable>,
}

#[derive(Clone, Default)]
pub struct ScopeContext<'a> {
    pub project: Option<&'a Project>,
    pub target_profile: Option<TargetProfile>,
    pub sigma: Sigma,
    pub diagnostics: Option<std::rc::Rc<std::cell::RefCell<DiagnosticStream>>>,
    pub name_resolution_tables: Option<NameResolutionTables<'a>>,
    pub current_module: Vec<String>,
    /// Innermost scope first; the last three are the procedure, module and universe scopes.
    pub scopes: ScopeList,
}

impl ScopeContext<'_> {
    pub fn local_scopes(&self) -> &[Scope] {
        match self.scopes.len() {
            len if len < 3 => &[],
            len => &self.scopes[..len - 3],
        }
    }

    pub fn proc_scope(&self) -> &Scope {
        &self.scopes[self.scopes.len() - 3]
    }

    pub fn module_scope(&self) -> &Scope {
        &self.scopes[self.scopes.len() - 2]
    }

    pub fn universe_scope(&self) -> &Scope {
        &self.scopes[self.scopes.len() - 1]
    }
}
