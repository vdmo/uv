//! The order modules are initialised in. A module depends on the modules its types
//! name, on those its `static` initialisers use (eagerly: they run at start-up), and on
//! those its procedure bodies and field defaults use (lazily). Eager dependencies must
//! be acyclic, and their topological order is the initialisation order.
//! See `BuildInitPlan`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use uv_core::diagnostics::DiagnosticStream;
use uv_core::symbols::string_of_path;
use uv_source::ast::{self, ASTItem, ExprNode, ExprPtr, PatternNode, PatternPtr, Stmt, TypePtr};

use crate::context::{EntityKind, EntitySource, NameMapTable, ScopeContext};
use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};

type ModuleSet = BTreeSet<usize>;
type Edges = Vec<BTreeSet<usize>>;

#[derive(Default)]
pub struct InitGraph {
    pub modules: Vec<Vec<String>>,
    pub type_edges: Vec<(usize, usize)>,
    pub eager_edges: Vec<(usize, usize)>,
    pub lazy_edges: Vec<(usize, usize)>,
}

#[derive(Default)]
pub struct InitPlan {
    pub graph: InitGraph,
    pub init_order: Vec<Vec<String>>,
    pub topo_ok: bool,
}

pub struct InitPlanResult {
    pub ok: bool,
    pub diags: DiagnosticStream,
    pub plan: InitPlan,
}

struct InitEnv<'m> {
    own: Vec<String>,
    modules: &'m [Vec<String>],
    module_index: &'m BTreeMap<String, usize>,
    alias: HashMap<String, Vec<String>>,
    using_value: HashMap<String, Vec<String>>,
    using_type: HashMap<String, Vec<String>>,
}

fn split_module_path(path: &str) -> Vec<String> {
    path.split("::").map(str::to_string).collect()
}

fn path_prefix(path: &[String], prefix: &[String]) -> bool {
    prefix.len() <= path.len() && prefix.iter().zip(path).all(|(lhs, rhs)| id_eq(rhs, lhs))
}

/// The longest module whose path starts the given path.
fn longest_module_prefix(path: &[String], modules: &[Vec<String>]) -> Option<Vec<String>> {
    let mut best: Option<&Vec<String>> = None;
    for module in modules {
        if path_prefix(path, module) && best.is_none_or(|found| module.len() > found.len()) {
            best = Some(module);
        }
    }
    best.cloned()
}

impl InitEnv<'_> {
    fn module_index_of(&self, path: &[String]) -> Option<usize> {
        self.module_index.get(&string_of_path(path)).copied()
    }

    fn alias_expand(&self, path: &[String]) -> Vec<String> {
        let Some(first) = path.first() else {
            return Vec::new();
        };
        match self.alias.get(&id_key_of(first)) {
            Some(target) => target.iter().chain(&path[1..]).cloned().collect(),
            None => path.to_vec(),
        }
    }

    /// The module a path starts in: as written, or under this module's assembly.
    fn module_prefix(&self, path: &[String]) -> Option<Vec<String>> {
        let expanded = self.alias_expand(path);
        if let Some(direct) = longest_module_prefix(&expanded, self.modules) {
            return Some(direct);
        }
        let assembly = self.own.first()?;
        let under_assembly: Vec<String> = std::iter::once(assembly.clone()).chain(expanded).collect();
        longest_module_prefix(&under_assembly, self.modules)
    }

    /// The module as a set of one, when it is not this one.
    fn other_module(&self, path: Option<&[String]>) -> ModuleSet {
        let found = path.filter(|path| *path != self.own.as_slice()).and_then(|path| self.module_index_of(path));
        found.into_iter().collect()
    }

    fn type_refs_type_path(&self, path: &[String]) -> ModuleSet {
        match path {
            [] => ModuleSet::new(),
            [only] => self.other_module(self.using_type.get(&id_key_of(only)).map(Vec::as_slice)),
            _ => self.other_module(self.module_prefix(path).as_deref()),
        }
    }

    fn type_refs_types<'t>(&self, types: impl IntoIterator<Item = &'t TypePtr>) -> ModuleSet {
        types.into_iter().flat_map(|ty| self.type_refs_ty(ty)).collect()
    }

    fn type_refs_ty(&self, ty: &TypePtr) -> ModuleSet {
        use ast::TypeNode as T;
        let Some(t) = ty.as_deref() else {
            return ModuleSet::new();
        };
        match &t.node {
            T::TypePathType(node) => self.type_refs_type_path(&node.path).into_iter().chain(self.type_refs_types(&node.generic_args)).collect(),
            T::TypeApply(node) => self.type_refs_type_path(&node.path).into_iter().chain(self.type_refs_types(&node.args)).collect(),
            T::TypeDynamic(node) => self.type_refs_type_path(&node.path),
            T::TypeModalState(node) => self.type_refs_type_path(&node.path).into_iter().chain(self.type_refs_types(&node.generic_args)).collect(),
            T::TypePermType(node) => self.type_refs_ty(&node.base),
            T::TypeRefine(node) => self.type_refs_ty(&node.base).into_iter().chain(self.type_refs_expr(&node.predicate)).collect(),
            T::TypeTuple(node) => self.type_refs_types(&node.elements),
            T::TypeArray(node) => self.type_refs_ty(&node.element).into_iter().chain(self.type_refs_expr(&node.length)).collect(),
            T::TypeSlice(node) => self.type_refs_ty(&node.element),
            T::TypeUnion(node) => self.type_refs_types(&node.types),
            T::TypeFunc(node) => self.type_refs_types(node.params.iter().map(|param| &param.r#type).chain([&node.ret])),
            T::TypeClosure(node) => {
                let mut out = self.type_refs_types(node.params.iter().map(|param| &param.r#type).chain([&node.ret]));
                for dep in node.deps_opt.iter().flatten() {
                    out.extend(self.type_refs_ty(&dep.r#type));
                }
                out
            }
            T::TypeSafePtr(node) => self.type_refs_ty(&node.element),
            T::TypeRawPtr(node) => self.type_refs_ty(&node.element),
            T::TypeRange(node) => self.type_refs_ty(&node.base),
            T::TypeRangeInclusive(node) => self.type_refs_ty(&node.base),
            T::TypeRangeFrom(node) => self.type_refs_ty(&node.base),
            T::TypeRangeTo(node) => self.type_refs_ty(&node.base),
            T::TypeRangeToInclusive(node) => self.type_refs_ty(&node.base),
            _ => ModuleSet::new(),
        }
    }

    fn type_refs_field_patterns(&self, fields: &[ast::FieldPattern]) -> ModuleSet {
        fields.iter().flat_map(|field| self.type_refs_pat(&field.pattern_opt)).collect()
    }

    fn type_refs_pat(&self, pat: &PatternPtr) -> ModuleSet {
        let Some(p) = pat.as_deref() else {
            return ModuleSet::new();
        };
        match &p.node {
            PatternNode::RecordPattern(node) => self.type_refs_type_path(&node.path).into_iter().chain(self.type_refs_field_patterns(&node.fields)).collect(),
            PatternNode::EnumPattern(node) => {
                let payload = match &node.payload_opt {
                    Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => payload.elements.iter().flat_map(|elem| self.type_refs_pat(elem)).collect(),
                    Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => self.type_refs_field_patterns(&payload.fields),
                    None => ModuleSet::new(),
                };
                self.type_refs_type_path(&node.path).into_iter().chain(payload).collect()
            }
            PatternNode::TuplePattern(node) => node.elements.iter().flat_map(|elem| self.type_refs_pat(elem)).collect(),
            PatternNode::ModalPattern(node) => node.fields_opt.as_ref().map_or_else(ModuleSet::new, |payload| self.type_refs_field_patterns(&payload.fields)),
            PatternNode::RangePattern(node) => self.type_refs_pat(&node.lo).into_iter().chain(self.type_refs_pat(&node.hi)).collect(),
            _ => ModuleSet::new(),
        }
    }

    fn type_refs_exprs<'e>(&self, exprs: impl IntoIterator<Item = &'e ExprPtr>) -> ModuleSet {
        exprs.into_iter().flat_map(|expr| self.type_refs_expr(expr)).collect()
    }

    /// The modules the types written inside an expression name.
    fn type_refs_expr(&self, expr: &ExprPtr) -> ModuleSet {
        let Some(e) = expr.as_deref() else {
            return ModuleSet::new();
        };
        match &e.node {
            ExprNode::RecordExpr(node) => {
                let target = match &node.target {
                    ast::RecordExprTarget::Path(path) => self.type_refs_type_path(path),
                    ast::RecordExprTarget::ModalStateRef(state) => {
                        self.type_refs_type_path(&state.path).into_iter().chain(self.type_refs_types(&state.generic_args)).collect()
                    }
                };
                target.into_iter().chain(self.type_refs_exprs(node.fields.iter().map(|field| &field.value))).collect()
            }
            ExprNode::EnumLiteralExpr(node) => {
                let enum_path = if node.path.len() < 2 { &[][..] } else { &node.path[..node.path.len() - 1] };
                let payload = match &node.payload_opt {
                    Some(ast::EnumPayload::EnumPayloadParen(payload)) => self.type_refs_exprs(&payload.elements),
                    Some(ast::EnumPayload::EnumPayloadBrace(payload)) => self.type_refs_exprs(payload.fields.iter().map(|field| &field.value)),
                    None => ModuleSet::new(),
                };
                self.type_refs_type_path(enum_path).into_iter().chain(payload).collect()
            }
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::BraceArgs(args) => {
                    let full: Vec<String> = node.path.iter().cloned().chain([node.name.clone()]).collect();
                    self.type_refs_type_path(&full).into_iter().chain(self.type_refs_exprs(args.fields.iter().map(|field| &field.value))).collect()
                }
                ast::ApplyArgs::ParenArgs(_) => self.type_refs_children(e),
            },
            ExprNode::CallExpr(node) if !node.generic_args.is_empty() => {
                let mut out = self.type_refs_expr(&node.callee);
                out.extend(self.type_refs_exprs(node.args.iter().map(|arg| &arg.value)));
                out.extend(self.type_refs_types(&node.generic_args));
                out
            }
            ExprNode::CallTypeArgsExpr(node) => {
                let mut out = self.type_refs_expr(&node.callee);
                out.extend(self.type_refs_exprs(node.args.iter().map(|arg| &arg.value)));
                out.extend(self.type_refs_types(&node.type_args));
                out
            }
            ExprNode::CastExpr(node) => self.type_refs_expr(&node.value).into_iter().chain(self.type_refs_ty(&node.r#type)).collect(),
            ExprNode::TransmuteExpr(node) => {
                self.type_refs_expr(&node.value).into_iter().chain(self.type_refs_ty(&node.from)).chain(self.type_refs_ty(&node.to)).collect()
            }
            _ => self.type_refs_children(e),
        }
    }

    fn type_refs_children(&self, e: &ast::Expr) -> ModuleSet {
        children_ltr(e).into_iter().flat_map(|child| self.type_refs_expr(child)).collect()
    }

    fn value_refs_qualified(&self, path: &[String]) -> ModuleSet {
        self.other_module(self.module_prefix(path).as_deref())
    }

    /// The modules the values an expression uses live in.
    fn value_refs(&self, expr: &ExprPtr) -> ModuleSet {
        let Some(e) = expr.as_deref() else {
            return ModuleSet::new();
        };
        match &e.node {
            ExprNode::IdentifierExpr(node) => self.other_module(self.using_value.get(&id_key_of(&node.name)).map(Vec::as_slice)),
            ExprNode::QualifiedNameExpr(node) => self.value_refs_qualified(&node.path),
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::ParenArgs(args) => {
                    let mut out: ModuleSet = args.args.iter().flat_map(|arg| self.value_refs(&arg.value)).collect();
                    out.extend(self.other_module(self.module_prefix(&node.path).as_deref()));
                    out
                }
                ast::ApplyArgs::BraceArgs(args) => args.fields.iter().flat_map(|field| self.value_refs(&field.value)).collect(),
            },
            ExprNode::PathExpr(node) => self.value_refs_qualified(&node.path),
            _ => children_ltr(e).into_iter().flat_map(|child| self.value_refs(child)).collect(),
        }
    }
}

/// The direct sub-expressions the planner looks into. A block, an `if`'s branches and
/// a case's bodies are not among them.
fn children_ltr(expr: &ast::Expr) -> Vec<&ExprPtr> {
    match &expr.node {
        ExprNode::TupleExpr(node) => node.elements.iter().collect(),
        ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node),
        ExprNode::ArrayRepeatExpr(node) => vec![&node.value, &node.count],
        ExprNode::RecordExpr(node) => node.fields.iter().map(|field| &field.value).collect(),
        ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
            Some(ast::EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().collect(),
            Some(ast::EnumPayload::EnumPayloadBrace(payload)) => payload.fields.iter().map(|field| &field.value).collect(),
            None => Vec::new(),
        },
        ExprNode::FieldAccessExpr(node) => vec![&node.base],
        ExprNode::TupleAccessExpr(node) => vec![&node.base],
        ExprNode::IndexAccessExpr(node) => vec![&node.base, &node.index],
        ExprNode::CallExpr(node) => std::iter::once(&node.callee).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::MethodCallExpr(node) => std::iter::once(&node.receiver).chain(node.args.iter().map(|arg| &arg.value)).collect(),
        ExprNode::UnaryExpr(node) => vec![&node.value],
        ExprNode::BinaryExpr(node) => vec![&node.lhs, &node.rhs],
        ExprNode::CastExpr(node) => vec![&node.value],
        ExprNode::TransmuteExpr(node) => vec![&node.value],
        ExprNode::PropagateExpr(node) => vec![&node.value],
        ExprNode::RangeExpr(node) => [&node.lhs, &node.rhs].into_iter().filter(|side| side.is_some()).collect(),
        ExprNode::IfExpr(node) => vec![&node.cond],
        ExprNode::IfCaseExpr(node) => vec![&node.scrutinee],
        ExprNode::IfIsExpr(node) => vec![&node.scrutinee],
        ExprNode::LoopConditionalExpr(node) => vec![&node.cond],
        ExprNode::LoopIterExpr(node) => vec![&node.iter],
        ExprNode::DerefExpr(node) => vec![&node.value],
        ExprNode::AllocExpr(node) => vec![&node.value],
        ExprNode::QualifiedApplyExpr(node) => match &node.args {
            ast::ApplyArgs::ParenArgs(args) => args.args.iter().map(|arg| &arg.value).collect(),
            ast::ApplyArgs::BraceArgs(args) => args.fields.iter().map(|field| &field.value).collect(),
        },
        _ => Vec::new(),
    }
}

/// Every expression a piece of syntax holds, as the planner collects them.
#[derive(Default)]
struct Collected {
    exprs: Vec<ExprPtr>,
    pats: Vec<PatternPtr>,
}

impl Collected {
    fn expr_from_type(&mut self, ty: &TypePtr) {
        use ast::TypeNode as T;
        let Some(t) = ty.as_deref() else {
            return;
        };
        match &t.node {
            T::TypePermType(node) => self.expr_from_type(&node.base),
            T::TypeUnion(node) => node.types.iter().for_each(|elem| self.expr_from_type(elem)),
            T::TypeFunc(node) => {
                node.params.iter().for_each(|param| self.expr_from_type(&param.r#type));
                self.expr_from_type(&node.ret);
            }
            T::TypeClosure(node) => {
                node.params.iter().for_each(|param| self.expr_from_type(&param.r#type));
                self.expr_from_type(&node.ret);
                node.deps_opt.iter().flatten().for_each(|dep| self.expr_from_type(&dep.r#type));
            }
            T::TypeTuple(node) => node.elements.iter().for_each(|elem| self.expr_from_type(elem)),
            T::TypeArray(node) => {
                self.expr_from_type(&node.element);
                self.expr(&node.length);
            }
            T::TypeSlice(node) => self.expr_from_type(&node.element),
            T::TypeSafePtr(node) => self.expr_from_type(&node.element),
            T::TypeRawPtr(node) => self.expr_from_type(&node.element),
            T::TypeRefine(node) => {
                self.expr_from_type(&node.base);
                self.expr(&node.predicate);
            }
            _ => {}
        }
    }

    fn block(&mut self, block: &ast::BlockPtr) {
        let Some(block) = block.as_deref() else {
            return;
        };
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
        self.expr(&block.tail_opt);
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LetStmt(node) => {
                self.expr(&node.binding.init);
                self.expr_from_type(&node.binding.type_opt);
            }
            Stmt::VarStmt(node) => {
                self.expr(&node.binding.init);
                self.expr_from_type(&node.binding.type_opt);
            }
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
        let Some(e) = expr.as_deref() else {
            return;
        };
        self.exprs.push(expr.clone());
        match &e.node {
            ExprNode::UnaryExpr(node) => self.expr(&node.value),
            ExprNode::BinaryExpr(node) => {
                self.expr(&node.lhs);
                self.expr(&node.rhs);
            }
            ExprNode::CastExpr(node) => {
                self.expr(&node.value);
                self.expr_from_type(&node.r#type);
            }
            ExprNode::TransmuteExpr(node) => {
                self.expr(&node.value);
                self.expr_from_type(&node.from);
                self.expr_from_type(&node.to);
            }
            ExprNode::DerefExpr(node) => self.expr(&node.value),
            ExprNode::AddressOfExpr(node) => self.expr(&node.place),
            ExprNode::MoveExpr(node) => self.expr(&node.place),
            ExprNode::AllocExpr(node) => self.expr(&node.value),
            ExprNode::TupleExpr(node) => node.elements.iter().for_each(|elem| self.expr(elem)),
            ExprNode::ArrayExpr(node) => ast::array_expr_subexprs(node).into_iter().for_each(|elem| self.expr(elem)),
            ExprNode::ArrayRepeatExpr(node) => {
                self.expr(&node.value);
                self.expr(&node.count);
            }
            ExprNode::RecordExpr(node) => node.fields.iter().for_each(|field| self.expr(&field.value)),
            ExprNode::EnumLiteralExpr(node) => match &node.payload_opt {
                Some(ast::EnumPayload::EnumPayloadParen(payload)) => payload.elements.iter().for_each(|elem| self.expr(elem)),
                Some(ast::EnumPayload::EnumPayloadBrace(payload)) => payload.fields.iter().for_each(|field| self.expr(&field.value)),
                None => {}
            },
            ExprNode::IfExpr(node) => {
                self.expr(&node.cond);
                self.expr(&node.then_expr);
                self.expr(&node.else_expr);
            }
            ExprNode::IfCaseExpr(node) => {
                self.expr(&node.scrutinee);
                node.cases.iter().for_each(|clause| self.expr(&clause.body));
                self.expr(&node.else_expr);
            }
            ExprNode::IfIsExpr(node) => {
                self.expr(&node.scrutinee);
                self.expr(&node.then_expr);
                self.expr(&node.else_expr);
            }
            ExprNode::LoopInfiniteExpr(node) => self.block(&node.body),
            ExprNode::LoopConditionalExpr(node) => {
                self.expr(&node.cond);
                self.block(&node.body);
            }
            ExprNode::LoopIterExpr(node) => {
                self.expr(&node.iter);
                self.expr_from_type(&node.type_opt);
                self.block(&node.body);
            }
            ExprNode::BlockExpr(node) => self.block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.block(&node.block),
            ExprNode::RangeExpr(node) => {
                self.expr(&node.lhs);
                self.expr(&node.rhs);
            }
            ExprNode::FieldAccessExpr(node) => self.expr(&node.base),
            ExprNode::TupleAccessExpr(node) => self.expr(&node.base),
            ExprNode::IndexAccessExpr(node) => {
                self.expr(&node.base);
                self.expr(&node.index);
            }
            ExprNode::CallExpr(node) => {
                self.expr(&node.callee);
                node.args.iter().for_each(|arg| self.expr(&arg.value));
            }
            ExprNode::MethodCallExpr(node) => {
                self.expr(&node.receiver);
                node.args.iter().for_each(|arg| self.expr(&arg.value));
            }
            ExprNode::PropagateExpr(node) => self.expr(&node.value),
            ExprNode::QualifiedApplyExpr(node) => match &node.args {
                ast::ApplyArgs::ParenArgs(args) => args.args.iter().for_each(|arg| self.expr(&arg.value)),
                ast::ApplyArgs::BraceArgs(args) => args.fields.iter().for_each(|field| self.expr(&field.value)),
            },
            _ => {}
        }
    }

    fn pattern(&mut self, pattern: &PatternPtr) {
        let Some(p) = pattern.as_deref() else {
            return;
        };
        self.pats.push(pattern.clone());
        match &p.node {
            PatternNode::TuplePattern(node) => node.elements.iter().for_each(|elem| self.pattern(elem)),
            PatternNode::RecordPattern(node) => node.fields.iter().for_each(|field| self.pattern(&field.pattern_opt)),
            PatternNode::EnumPattern(node) => match &node.payload_opt {
                Some(ast::EnumPayloadPattern::TuplePayloadPattern(payload)) => payload.elements.iter().for_each(|elem| self.pattern(elem)),
                Some(ast::EnumPayloadPattern::RecordPayloadPattern(payload)) => payload.fields.iter().for_each(|field| self.pattern(&field.pattern_opt)),
                None => {}
            },
            PatternNode::ModalPattern(node) => node.fields_opt.iter().flat_map(|payload| &payload.fields).for_each(|field| self.pattern(&field.pattern_opt)),
            PatternNode::RangePattern(node) => {
                self.pattern(&node.lo);
                self.pattern(&node.hi);
            }
            PatternNode::TypedPattern(node) => self.expr_from_type(&node.r#type),
            _ => {}
        }
    }

    fn pattern_from_expr(&mut self, expr: &ExprPtr) {
        let Some(e) = expr.as_deref() else {
            return;
        };
        match &e.node {
            ExprNode::IfCaseExpr(node) => {
                self.pattern_from_expr(&node.scrutinee);
                for clause in &node.cases {
                    self.pattern(&clause.pattern);
                    self.pattern_from_expr(&clause.body);
                }
                self.pattern_from_expr(&node.else_expr);
            }
            ExprNode::IfIsExpr(node) => {
                self.pattern_from_expr(&node.scrutinee);
                self.pattern(&node.pattern);
                self.pattern_from_expr(&node.then_expr);
                self.pattern_from_expr(&node.else_expr);
            }
            ExprNode::LoopIterExpr(node) => {
                self.pattern(&node.pattern);
                self.expr_from_type(&node.type_opt);
                self.pattern_from_block(&node.body);
            }
            ExprNode::IfExpr(node) => {
                self.pattern_from_expr(&node.cond);
                self.pattern_from_expr(&node.then_expr);
                self.pattern_from_expr(&node.else_expr);
            }
            ExprNode::BlockExpr(node) => self.pattern_from_block(&node.block),
            ExprNode::UnsafeBlockExpr(node) => self.pattern_from_block(&node.block),
            ExprNode::LoopInfiniteExpr(node) => self.pattern_from_block(&node.body),
            ExprNode::LoopConditionalExpr(node) => self.pattern_from_block(&node.body),
            _ => {}
        }
    }

    fn pattern_from_block(&mut self, block: &ast::BlockPtr) {
        let Some(block) = block.as_deref() else {
            return;
        };
        for stmt in &block.stmts {
            match stmt {
                Stmt::LetStmt(node) => self.pattern(&node.binding.pat),
                Stmt::VarStmt(node) => self.pattern(&node.binding.pat),
                _ => {}
            }
        }
        self.pattern_from_expr(&block.tail_opt);
    }
}

fn explicit_receiver_type(receiver: &ast::Receiver) -> Option<&TypePtr> {
    match receiver {
        ast::Receiver::ReceiverExplicit(explicit) => Some(&explicit.r#type),
        ast::Receiver::ReceiverShorthand(_) => None,
    }
}

/// The types written in a declaration's signature and fields, for each declaration of the module.
fn signature_types(item: &ASTItem) -> Vec<&TypePtr> {
    let mut out: Vec<&TypePtr> = Vec::new();
    match item {
        ASTItem::StaticDecl(node) => out.push(&node.binding.type_opt),
        ASTItem::ProcedureDecl(node) => out.extend(node.params.iter().map(|param| &param.r#type).chain([&node.return_type_opt])),
        ASTItem::RecordDecl(node) => {
            for member in &node.members {
                match member {
                    ast::RecordMember::FieldDecl(field) => out.push(&field.r#type),
                    ast::RecordMember::MethodDecl(method) => {
                        out.extend(explicit_receiver_type(&method.receiver));
                        out.extend(method.params.iter().map(|param| &param.r#type));
                        out.push(&method.return_type_opt);
                    }
                    ast::RecordMember::AssociatedTypeDecl(assoc) => out.push(&assoc.default_type),
                }
            }
        }
        ASTItem::EnumDecl(node) => {
            for variant in &node.variants {
                match &variant.payload_opt {
                    Some(ast::VariantPayload::VariantPayloadTuple(payload)) => out.extend(payload.elements.iter()),
                    Some(ast::VariantPayload::VariantPayloadRecord(payload)) => out.extend(payload.fields.iter().map(|field| &field.r#type)),
                    None => {}
                }
            }
        }
        ASTItem::ClassDecl(node) => {
            for member in &node.items {
                match member {
                    ast::ClassItem::ClassFieldDecl(field) => out.push(&field.r#type),
                    ast::ClassItem::ClassMethodDecl(method) => {
                        out.extend(explicit_receiver_type(&method.receiver));
                        out.extend(method.params.iter().map(|param| &param.r#type));
                        out.push(&method.return_type_opt);
                    }
                    _ => {}
                }
            }
        }
        ASTItem::TypeAliasDecl(node) => out.push(&node.r#type),
        _ => {}
    }
    out
}

fn implemented_paths(item: &ASTItem) -> &[Vec<String>] {
    match item {
        ASTItem::RecordDecl(node) => &node.implements,
        ASTItem::EnumDecl(node) => &node.implements,
        ASTItem::ModalDecl(node) => &node.implements,
        ASTItem::ClassDecl(node) => &node.supers,
        _ => &[],
    }
}

/// The array lengths written in a declaration's signature and fields.
fn array_size_exprs(ty: &TypePtr, out: &mut Vec<ExprPtr>) {
    use ast::TypeNode as T;
    let Some(t) = ty.as_deref() else {
        return;
    };
    match &t.node {
        T::TypeArray(node) => {
            if node.length.is_some() {
                out.push(node.length.clone());
            }
            array_size_exprs(&node.element, out);
        }
        T::TypePermType(node) => array_size_exprs(&node.base, out),
        T::TypeTuple(node) => node.elements.iter().for_each(|elem| array_size_exprs(elem, out)),
        T::TypeUnion(node) => node.types.iter().for_each(|elem| array_size_exprs(elem, out)),
        T::TypeFunc(node) => {
            node.params.iter().for_each(|param| array_size_exprs(&param.r#type, out));
            array_size_exprs(&node.ret, out);
        }
        T::TypeClosure(node) => {
            node.params.iter().for_each(|param| array_size_exprs(&param.r#type, out));
            array_size_exprs(&node.ret, out);
            node.deps_opt.iter().flatten().for_each(|dep| array_size_exprs(&dep.r#type, out));
        }
        T::TypeSlice(node) => array_size_exprs(&node.element, out),
        T::TypeSafePtr(node) => array_size_exprs(&node.element, out),
        T::TypeRawPtr(node) => array_size_exprs(&node.element, out),
        T::TypeRefine(node) => array_size_exprs(&node.base, out),
        _ => {}
    }
}

/// The signature types of a modal's methods and transitions also hold array lengths.
fn modal_signature_types(node: &ast::ModalDecl) -> Vec<&TypePtr> {
    let mut out: Vec<&TypePtr> = Vec::new();
    for member in node.states.iter().flat_map(|state| &state.members) {
        match member {
            ast::StateMember::StateFieldDecl(field) => out.push(&field.r#type),
            ast::StateMember::StateMethodDecl(method) => {
                out.extend(method.params.iter().map(|param| &param.r#type));
                out.push(&method.return_type_opt);
            }
            ast::StateMember::TransitionDecl(transition) => out.extend(transition.params.iter().map(|param| &param.r#type)),
        }
    }
    out
}

fn type_deps_for_module(module: &ast::ASTModule, env: &InitEnv<'_>) -> ModuleSet {
    let mut deps = ModuleSet::new();
    let mut type_pos_exprs: Vec<ExprPtr> = Vec::new();
    for item in &module.items {
        for ty in signature_types(item) {
            deps.extend(env.type_refs_ty(ty));
        }
        for path in implemented_paths(item).iter().filter(|path| !path.is_empty()) {
            deps.extend(env.type_refs_type_path(path));
        }
        if let ASTItem::EnumDecl(node) = item {
            for variant in &node.variants {
                if let Some(token) = &variant.discriminant_opt {
                    let literal = ast::Expr { span: token.span.clone(), node: ExprNode::LiteralExpr(ast::LiteralExpr { literal: token.clone() }) };
                    type_pos_exprs.push(Some(std::sync::Arc::new(literal)));
                }
            }
        }
    }
    let mut collected = Collected::default();
    for item in &module.items {
        match item {
            ASTItem::StaticDecl(node) => {
                collected.expr(&node.binding.init);
                collected.expr_from_type(&node.binding.type_opt);
            }
            ASTItem::ProcedureDecl(node) => {
                collected.block(&node.body);
                node.params.iter().for_each(|param| collected.expr_from_type(&param.r#type));
                collected.expr_from_type(&node.return_type_opt);
            }
            ASTItem::RecordDecl(node) => {
                for member in &node.members {
                    match member {
                        ast::RecordMember::FieldDecl(field) => {
                            collected.expr(&field.init_opt);
                            collected.expr_from_type(&field.r#type);
                        }
                        ast::RecordMember::MethodDecl(method) => {
                            collected.block(&method.body);
                            if let Some(receiver) = explicit_receiver_type(&method.receiver) {
                                collected.expr_from_type(receiver);
                            }
                            method.params.iter().for_each(|param| collected.expr_from_type(&param.r#type));
                            collected.expr_from_type(&method.return_type_opt);
                        }
                        ast::RecordMember::AssociatedTypeDecl(_) => {}
                    }
                }
            }
            ASTItem::EnumDecl(node) => {
                for variant in &node.variants {
                    match &variant.payload_opt {
                        Some(ast::VariantPayload::VariantPayloadTuple(payload)) => payload.elements.iter().for_each(|elem| collected.expr_from_type(elem)),
                        Some(ast::VariantPayload::VariantPayloadRecord(payload)) => {
                            for field in &payload.fields {
                                collected.expr_from_type(&field.r#type);
                                collected.expr(&field.init_opt);
                            }
                        }
                        None => {}
                    }
                }
            }
            ASTItem::ModalDecl(node) => {
                for member in node.states.iter().flat_map(|state| &state.members) {
                    match member {
                        ast::StateMember::StateFieldDecl(field) => collected.expr_from_type(&field.r#type),
                        ast::StateMember::StateMethodDecl(method) => {
                            collected.block(&method.body);
                            method.params.iter().for_each(|param| collected.expr_from_type(&param.r#type));
                            collected.expr_from_type(&method.return_type_opt);
                        }
                        ast::StateMember::TransitionDecl(transition) => {
                            collected.block(&transition.body);
                            transition.params.iter().for_each(|param| collected.expr_from_type(&param.r#type));
                        }
                    }
                }
            }
            ASTItem::ClassDecl(node) => {
                for member in &node.items {
                    match member {
                        ast::ClassItem::ClassFieldDecl(field) => collected.expr_from_type(&field.r#type),
                        ast::ClassItem::ClassMethodDecl(method) => {
                            collected.block(&method.body_opt);
                            if let Some(receiver) = explicit_receiver_type(&method.receiver) {
                                collected.expr_from_type(receiver);
                            }
                            method.params.iter().for_each(|param| collected.expr_from_type(&param.r#type));
                            collected.expr_from_type(&method.return_type_opt);
                        }
                        _ => {}
                    }
                }
            }
            ASTItem::TypeAliasDecl(node) => collected.expr_from_type(&node.r#type),
            _ => {}
        }
    }
    for item in &module.items {
        match item {
            ASTItem::StaticDecl(node) => collected.pattern(&node.binding.pat),
            ASTItem::ProcedureDecl(node) => collected.pattern_from_block(&node.body),
            ASTItem::RecordDecl(node) => {
                for member in &node.members {
                    if let ast::RecordMember::MethodDecl(method) = member {
                        collected.pattern_from_block(&method.body);
                    }
                }
            }
            ASTItem::ClassDecl(node) => {
                for member in &node.items {
                    if let ast::ClassItem::ClassMethodDecl(method) = member {
                        collected.pattern_from_block(&method.body_opt);
                    }
                }
            }
            ASTItem::ModalDecl(node) => {
                for member in node.states.iter().flat_map(|state| &state.members) {
                    match member {
                        ast::StateMember::StateMethodDecl(method) => collected.pattern_from_block(&method.body),
                        ast::StateMember::TransitionDecl(transition) => collected.pattern_from_block(&transition.body),
                        ast::StateMember::StateFieldDecl(_) => {}
                    }
                }
            }
            _ => {}
        }
    }
    for pat in &collected.pats {
        deps.extend(env.type_refs_pat(pat));
    }
    for expr in &collected.exprs {
        deps.extend(env.type_refs_expr(expr));
    }
    for item in &module.items {
        let mut sizes = Vec::new();
        for ty in signature_types(item) {
            array_size_exprs(ty, &mut sizes);
        }
        if let ASTItem::ModalDecl(node) = item {
            for ty in modal_signature_types(node) {
                array_size_exprs(ty, &mut sizes);
            }
        }
        type_pos_exprs.extend(sizes);
    }
    for expr in &type_pos_exprs {
        deps.extend(env.type_refs_expr(expr));
    }
    deps
}

fn value_deps_eager_for_module(module: &ast::ASTModule, env: &InitEnv<'_>) -> ModuleSet {
    module
        .items
        .iter()
        .filter_map(|item| if let ASTItem::StaticDecl(decl) = item { Some(decl) } else { None })
        .flat_map(|decl| env.value_refs(&decl.binding.init))
        .collect()
}

fn value_deps_lazy_for_module(module: &ast::ASTModule, env: &InitEnv<'_>) -> ModuleSet {
    let mut collected = Collected::default();
    for item in &module.items {
        if let ASTItem::RecordDecl(record) = item {
            for member in &record.members {
                if let ast::RecordMember::FieldDecl(field) = member {
                    if field.init_opt.is_some() {
                        collected.exprs.push(field.init_opt.clone());
                    }
                }
            }
        }
    }
    for item in &module.items {
        match item {
            ASTItem::ProcedureDecl(node) => collected.block(&node.body),
            ASTItem::RecordDecl(node) => {
                for member in &node.members {
                    if let ast::RecordMember::MethodDecl(method) = member {
                        collected.block(&method.body);
                    }
                }
            }
            ASTItem::ClassDecl(node) => {
                for member in &node.items {
                    if let ast::ClassItem::ClassMethodDecl(method) = member {
                        collected.block(&method.body_opt);
                    }
                }
            }
            _ => {}
        }
    }
    collected.exprs.iter().flat_map(|expr| env.value_refs(expr)).collect()
}

fn add_edges(edges: &mut Edges, from: usize, deps: &ModuleSet) {
    for dep in deps {
        if let Some(targets) = edges.get_mut(*dep) {
            targets.insert(from);
        }
    }
}

fn reachable(u: usize, v: usize, edges: &Edges, visiting: &mut [bool]) -> bool {
    if edges[u].contains(&v) {
        return true;
    }
    for &w in &edges[u] {
        if visiting[w] {
            continue;
        }
        visiting[w] = true;
        if reachable(w, v, edges, visiting) {
            return true;
        }
    }
    false
}

fn acyclic_eager(edges: &Edges) -> bool {
    (0..edges.len()).all(|v| {
        let mut visiting = vec![false; edges.len()];
        visiting[v] = true;
        !reachable(v, v, edges, &mut visiting)
    })
}

/// The modules in an order where each comes after those it depends on, the lowest
/// index first among those ready.
fn topo_order(edges: &Edges) -> (Vec<usize>, bool) {
    let n = edges.len();
    let mut indegree = vec![0usize; n];
    for targets in edges {
        for &v in targets.iter().filter(|v| **v < n) {
            indegree[v] += 1;
        }
    }
    let mut ready: BTreeSet<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(u) = ready.pop_first() {
        order.push(u);
        for &v in edges[u].iter().filter(|v| **v < n) {
            if indegree[v] == 0 {
                continue;
            }
            indegree[v] -= 1;
            if indegree[v] == 0 {
                ready.insert(v);
            }
        }
    }
    let ok = order.len() == n;
    (order, ok)
}

fn flatten_edges(edges: &Edges) -> Vec<(usize, usize)> {
    edges.iter().enumerate().flat_map(|(u, targets)| targets.iter().map(move |v| (u, *v))).collect()
}

/// See `BuildInitPlan`.
pub fn build_init_plan(ctx: &ScopeContext<'_>, name_maps: &NameMapTable) -> InitPlanResult {
    let mut modules: Vec<Vec<String>> = Vec::new();
    let mut module_index: BTreeMap<String, usize> = BTreeMap::new();
    let mut add_module = |path: Vec<String>| {
        let key = string_of_path(&path);
        if let std::collections::btree_map::Entry::Vacant(entry) = module_index.entry(key) {
            entry.insert(modules.len());
            modules.push(path);
        }
    };
    if let Some(project) = ctx.project {
        for module in &project.modules {
            add_module(split_module_path(&module.path));
        }
    }
    for module in &ctx.sigma.mods {
        add_module(module.path.clone());
    }
    let mut type_edges: Edges = vec![BTreeSet::new(); modules.len()];
    let mut eager_edges = type_edges.clone();
    let mut lazy_edges = type_edges.clone();
    for module in &ctx.sigma.mods {
        let names = name_maps.get(&path_key_of(&module.path));
        let mut env = InitEnv {
            own: module.path.clone(),
            modules: &modules,
            module_index: &module_index,
            alias: HashMap::new(),
            using_value: HashMap::new(),
            using_type: HashMap::new(),
        };
        for (key, entity) in names.into_iter().flat_map(|names| names.iter()) {
            let Some(origin) = &entity.origin_opt else {
                continue;
            };
            if entity.kind == EntityKind::ModuleAlias {
                env.alias.insert(key.clone(), origin.clone());
            }
            if entity.source != EntitySource::Using {
                continue;
            }
            match entity.kind {
                EntityKind::Value => {
                    env.using_value.insert(key.clone(), origin.clone());
                }
                EntityKind::Type | EntityKind::Class => {
                    env.using_type.insert(key.clone(), origin.clone());
                }
                EntityKind::ModuleAlias => {}
            }
        }
        let Some(self_index) = env.module_index_of(&env.own) else {
            continue;
        };
        add_edges(&mut type_edges, self_index, &type_deps_for_module(module, &env));
        add_edges(&mut eager_edges, self_index, &value_deps_eager_for_module(module, &env));
        add_edges(&mut lazy_edges, self_index, &value_deps_lazy_for_module(module, &env));
    }
    let mut plan = InitPlan {
        graph: InitGraph {
            modules: modules.clone(),
            type_edges: flatten_edges(&type_edges),
            eager_edges: flatten_edges(&eager_edges),
            lazy_edges: flatten_edges(&lazy_edges),
        },
        ..Default::default()
    };
    let mut diags = DiagnosticStream::new();
    if acyclic_eager(&eager_edges) {
        let (order, ok) = topo_order(&eager_edges);
        if ok {
            plan.init_order = order.into_iter().filter(|idx| *idx < modules.len()).map(|idx| modules[idx].clone()).collect();
        }
        plan.topo_ok = ok;
    } else {
        uv_core::diagnostic_messages::emit_external_diagnostic(&mut diags, "E-MOD-1401");
    }
    let ok = !uv_core::diagnostics::has_error(&diags);
    InitPlanResult { ok, diags, plan }
}
