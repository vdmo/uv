//! `#derive(Target, ..)`: finds the derive targets in scope, orders them by what they
//! require and emit, and runs each target's body against the annotated declaration.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream, Severity};
use uv_core::spec_rule_at;
use uv_core::symbols::string_of_path;
use uv_source::ast::*;
use uv_source::lexer::TokenKind;
use uv_source::module_paths::resolve_import_module_path;

use crate::eval::eval_block;
use crate::rewrite::CtProhibitedConstructFinder;
use crate::value::*;

struct DeriveRequest {
    name: String,
    target: DeriveTargetDecl,
}

/// The identifiers listed in `#derive(..)`; `None` when the argument list is malformed.
fn derive_targets_of(attr_list: &[AttributeItem]) -> Option<Vec<String>> {
    let Some(derive) = find_attribute(attr_list, "derive") else {
        return Some(Vec::new());
    };
    derive
        .args
        .iter()
        .map(|arg| match (&arg.key, &arg.value) {
            (None, AttributeArgValue::Token(token)) if token.kind == TokenKind::Identifier => {
                Some(token.lexeme.clone())
            }
            _ => None,
        })
        .collect()
}

/// `Module::Path::Name` as a type, for a record, enum or modal declaration.
fn target_type_of(item: &ASTItem, module_path: &[String]) -> Option<TypePtr> {
    let (name, span) = match item {
        ASTItem::RecordDecl(d) => (&d.name, &d.span),
        ASTItem::EnumDecl(d) => (&d.name, &d.span),
        ASTItem::ModalDecl(d) => (&d.name, &d.span),
        _ => return None,
    };
    let mut path = module_path.to_vec();
    path.push(name.clone());
    let node = TypePathType { path, generic_args: Vec::new() }.into();
    Some(Some(Arc::new(Type { span: span.clone(), node })))
}

fn path_prefix(prefix: &[String], path: &[String]) -> bool {
    path.len() >= prefix.len() && path[..prefix.len()] == *prefix
}

fn same_assembly(lhs: &[String], rhs: &[String]) -> bool {
    matches!((lhs.first(), rhs.first()), (Some(a), Some(b)) if a == b)
}

fn find_available_module<'a>(env: &'a CtEnv, path: &[String]) -> Option<&'a Rc<ASTModule>> {
    env.available_modules.iter().find(|module| module.path == path)
}

/// The items of a module a derive lookup may see: for the current module, only those
/// before the item being expanded.
fn visible_items<'a>(env: &CtEnv, module: &'a ASTModule) -> &'a [ASTItem] {
    let limit = if module.path == env.current_module {
        env.current_item_index.min(module.items.len())
    } else {
        module.items.len()
    };
    &module.items[..limit]
}

/// Derive targets are visible within their assembly.
fn can_access_internal(accessor_module: &[String], decl_module: &[String]) -> bool {
    same_assembly(accessor_module, decl_module)
}

fn visible_derive_target_decls_in_module(env: &CtEnv, module_path: &[String]) -> Vec<DeriveTargetDecl> {
    let Some(module) = find_available_module(env, module_path) else {
        return Vec::new();
    };
    visible_items(env, module)
        .iter()
        .filter_map(|item| match item {
            ASTItem::DeriveTargetDecl(derive) => Some(derive.clone()),
            _ => None,
        })
        .collect()
}

fn find_visible_derive_target_decl_in_module(env: &CtEnv, module_path: &[String], name: &str) -> Option<DeriveTargetDecl> {
    visible_derive_target_decls_in_module(env, module_path).into_iter().find(|derive| derive.name == name)
}

fn visible_imports(env: &CtEnv) -> Vec<&ImportDecl> {
    let Some(current) = find_available_module(env, &env.current_module) else {
        return Vec::new();
    };
    visible_items(env, current)
        .iter()
        .filter_map(|item| match item {
            ASTItem::ImportDecl(import) => Some(import),
            _ => None,
        })
        .collect()
}

fn expand_import_alias_prefix(env: &CtEnv, path: &[String]) -> Option<Vec<String>> {
    let head = path.first()?;
    for import in visible_imports(env) {
        let Some(resolved) = resolve_import_module_path(&env.current_module, &env.available_module_names, &import.path)
            .filter(|resolved| find_available_module(env, resolved).is_some())
        else {
            continue;
        };
        let alias = import.alias_opt.as_ref().or(resolved.last());
        if !alias.is_some_and(|alias| !alias.is_empty() && alias == head) {
            continue;
        }
        let mut expanded = resolved;
        expanded.extend(path[1..].iter().cloned());
        return Some(expanded);
    }
    None
}

fn resolve_visible_module_path(env: &CtEnv, path: &[String]) -> Option<Vec<String>> {
    if path.is_empty() {
        return None;
    }
    let path = expand_import_alias_prefix(env, path).unwrap_or_else(|| path.to_vec());
    resolve_import_module_path(&env.current_module, &env.available_module_names, &path)
        .filter(|resolved| find_available_module(env, resolved).is_some())
}

fn import_ok(env: &CtEnv, path: &[String]) -> bool {
    let import_required = !env.current_module.is_empty() && !path.is_empty() && !same_assembly(&env.current_module, path);
    if !import_required {
        return true;
    }
    visible_imports(env).into_iter().any(|import| {
        resolve_import_module_path(&env.current_module, &env.available_module_names, &import.path)
            .is_some_and(|resolved| path_prefix(&resolved, path))
    })
}

/// Derive targets found so far, without duplicates of the same declaration.
#[derive(Default)]
struct DeriveTargetSet {
    found: Vec<DeriveTargetDecl>,
    seen: HashSet<String>,
}

impl DeriveTargetSet {
    fn add(&mut self, module_path: &[String], decl: Option<DeriveTargetDecl>) {
        let Some(decl) = decl else {
            return;
        };
        if self.seen.insert(format!("{}::{}", string_of_path(module_path), decl.name)) {
            self.found.push(decl);
        }
    }
}

fn append_using_bound_derive_targets(out: &mut DeriveTargetSet, lookup_name: &str, env: &CtEnv, decl: &UsingDecl) {
    let module_path = match &decl.clause {
        UsingClause::UsingItem(clause) => &clause.module_path,
        UsingClause::UsingList(clause) => &clause.module_path,
        UsingClause::UsingWildcard(clause) => &clause.module_path,
    };
    if matches!(decl.clause, UsingClause::UsingItem(_)) && module_path.is_empty() {
        return;
    }
    let Some(resolved_module) = resolve_visible_module_path(env, module_path).filter(|resolved| import_ok(env, resolved))
    else {
        return;
    };
    let accessible = can_access_internal(&env.current_module, &resolved_module);
    let mut add_named = |target_name: &str, bind_name: &str| {
        let target = find_visible_derive_target_decl_in_module(env, &resolved_module, target_name);
        if target.is_some() && accessible && bind_name == lookup_name {
            out.add(&resolved_module, target);
        }
    };
    match &decl.clause {
        UsingClause::UsingItem(clause) => add_named(&clause.name, clause.alias_opt.as_ref().unwrap_or(&clause.name)),
        UsingClause::UsingList(clause) => {
            for spec in &clause.specs {
                add_named(&spec.name, spec.alias_opt.as_ref().unwrap_or(&spec.name));
            }
        }
        UsingClause::UsingWildcard(_) => {
            if accessible {
                for target in visible_derive_target_decls_in_module(env, &resolved_module) {
                    if target.name == lookup_name {
                        out.add(&resolved_module, Some(target));
                    }
                }
            }
        }
    }
}

/// The derive target a name refers to: declared earlier in this module or brought in by
/// `using`. Ambiguous names resolve to nothing.
fn visible_derive_target(name: &str, env: &CtEnv) -> Option<DeriveTargetDecl> {
    let mut visible = DeriveTargetSet::default();
    visible.add(&env.current_module, find_visible_derive_target_decl_in_module(env, &env.current_module, name));
    if let Some(current) = find_available_module(env, &env.current_module) {
        for item in visible_items(env, current) {
            if let ASTItem::UsingDecl(using_decl) = item {
                append_using_bound_derive_targets(&mut visible, name, env, using_decl);
            }
        }
    }
    if visible.found.len() == 1 {
        visible.found.pop()
    } else {
        None
    }
}

fn bind_derive_target_inputs(env: &CtEnv, item: &ASTItem) -> Option<CtEnv> {
    let target_type = target_type_of(item, &env.current_module)?;
    let mut derive_env = with_ct_caps(env.clone(), &[], true);
    derive_env.values.insert("target".to_string(), CtValue::Type(target_type));
    spec_rule_at!("requirement.22.DeriveTargetBodyBindings", &span_of_item(item));
    Some(derive_env)
}

fn declared_impl_names(item: &ASTItem) -> HashSet<String> {
    let implements = match item {
        ASTItem::RecordDecl(d) => &d.implements,
        ASTItem::EnumDecl(d) => &d.implements,
        ASTItem::ModalDecl(d) => &d.implements,
        _ => return HashSet::new(),
    };
    implements.iter().filter_map(|class_path| class_path.last().cloned()).collect()
}

fn derive_clause_names(target: &DeriveTargetDecl, kind: DeriveClauseKind) -> Vec<&str> {
    target.contract_opt.iter().filter(|clause| clause.kind == kind).map(|clause| clause.name.as_str()).collect()
}

fn derive_reqs(target: &DeriveTargetDecl) -> Vec<&str> {
    derive_clause_names(target, DeriveClauseKind::Requires)
}

fn derive_emits(target: &DeriveTargetDecl) -> Vec<&str> {
    derive_clause_names(target, DeriveClauseKind::Emits)
}

/// Every class a target requires or emits must be one the declaration says it implements.
fn validate_derive_contracts(item: &ASTItem, requests: &[DeriveRequest], diags: &mut DiagnosticStream) -> bool {
    let declared = declared_impl_names(item);
    let span = span_of_item(item);
    let mut report = |diag_id: &str| {
        if let Some(diag) = make_diagnostic_by_id(diag_id, Some(span.clone())) {
            emit(diags, diag);
        }
        false
    };
    for request in requests {
        if derive_reqs(&request.target).iter().any(|req| !declared.contains(*req)) {
            spec_rule_at!("requirement.22.DeriveRequiresValidation", &span);
            return report("E-CTE-0330");
        }
        if derive_emits(&request.target).iter().any(|emitted| !declared.contains(*emitted)) {
            spec_rule_at!("requirement.22.DeriveEmitsValidation", &span);
            return report("E-CTE-0331");
        }
    }
    if !requests.is_empty() {
        spec_rule_at!("requirement.22.DeriveRequiresEmitsScope", &span);
    }
    true
}

/// `lhs` must run after `rhs` when it requires something `rhs` emits.
fn derive_edge(lhs: &DeriveRequest, rhs: &DeriveRequest) -> bool {
    let rhs_emits = derive_emits(&rhs.target);
    derive_reqs(&lhs.target).iter().any(|req| rhs_emits.contains(req))
}

fn has_errors_since(diags: &Option<Rc<RefCell<DiagnosticStream>>>, baseline: usize) -> bool {
    diags
        .as_ref()
        .is_some_and(|diags| diags.borrow().iter().skip(baseline).any(|diag| diag.severity == Severity::Error))
}

fn emit_at_item(env: &CtEnv, diag_id: &str, item: &ASTItem) {
    emit_comptime_diag(env, diag_id, &span_of_item(item));
}

fn report_unresolved_target(env: &CtEnv, item: &ASTItem) {
    spec_rule_at!("requirement.22.DeriveTargetNameResolution", &span_of_item(item));
    emit_at_item(env, "E-CTE-0310", item);
}

fn resolve_derive_requests(item: &ASTItem, env: &CtEnv) -> Option<Vec<DeriveRequest>> {
    let names = match item {
        ASTItem::RecordDecl(d) => derive_targets_of(&d.attrs)?,
        ASTItem::EnumDecl(d) => derive_targets_of(&d.attrs)?,
        ASTItem::ModalDecl(d) => derive_targets_of(&d.attrs)?,
        _ => Vec::new(),
    };
    let mut requests = Vec::with_capacity(names.len());
    for name in names {
        let Some(target) = visible_derive_target(&name, env) else {
            report_unresolved_target(env, item);
            return None;
        };
        requests.push(DeriveRequest { name, target });
    }
    let diags = env.diags.as_ref().expect("compile-time pass runs with a diagnostic stream");
    validate_derive_contracts(item, &requests, &mut diags.borrow_mut()).then_some(requests)
}

/// Runs one derive target against the annotated item and returns what it emitted.
fn run_derive_target(item: &ASTItem, target: &DeriveTargetDecl, env: &CtEnv) -> Option<Vec<ASTItem>> {
    spec_rule_at!("RunDeriveTarget", &target.span);
    spec_rule_at!("requirement.22.DeriveTargetExecutionTiming", &target.span);
    let diag_count_before = env.diags.as_ref().map_or(0, |diags| diags.borrow().len());
    let mut derive_env = bind_derive_target_inputs(env, item)?;
    let target_emits = Rc::new(RefCell::new(Vec::new()));
    derive_env.pending_emits = Some(target_emits.clone());
    let body = target.body.as_deref().expect("derive target without a body");
    if let Some(span) = CtProhibitedConstructFinder::find(body) {
        spec_rule_at!("requirement.22.DeriveTargetBodyRestrictions", &span);
        emit_comptime_diag(env, "E-CTE-0320", &span);
        return None;
    }
    let exec = eval_block(body, &mut derive_env);
    if !exec.ok || has_errors_since(&env.diags, diag_count_before) {
        spec_rule_at!("requirement.22.DeriveTargetFailureSemantics", &target.span);
        return None;
    }
    let emitted = target_emits.borrow().clone();
    Some(emitted)
}

fn run_derive_set(item: &ASTItem, order: &[String], env: &CtEnv) -> Option<Vec<ASTItem>> {
    let span = span_of_item(item);
    let mut items = Vec::new();
    if order.is_empty() {
        spec_rule_at!("RunDeriveSet-Empty", &span);
    }
    for name in order {
        spec_rule_at!("RunDeriveSet-Cons", &span);
        let Some(target) = visible_derive_target(name, env) else {
            report_unresolved_target(env, item);
            return None;
        };
        items.extend(run_derive_target(item, &target, env)?);
    }
    Some(items)
}

/// Orders the requested targets so that a target runs before the targets it requires
/// output from... following the reference: an edge `i -> j` exists when `i` requires what
/// `j` emits, and targets without incoming edges run first, in written order.
fn derive_order_for(item: &ASTItem, env: &CtEnv) -> Option<Vec<String>> {
    let requests = resolve_derive_requests(item, env)?;
    let n = requests.len();
    let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indegree = vec![0usize; n];
    let mut has_dependency_edge = false;
    for i in 0..n {
        for j in 0..n {
            if i != j && derive_edge(&requests[i], &requests[j]) {
                has_dependency_edge = true;
                outgoing[i].push(j);
                indegree[j] += 1;
            }
        }
    }
    let mut emitted = vec![false; n];
    let mut order = Vec::with_capacity(n);
    let mut tie_breaker_observed = false;
    while order.len() < n {
        let mut eligible = (0..n).filter(|&i| !emitted[i] && indegree[i] == 0);
        let Some(next) = eligible.next() else {
            emit_at_item(env, "E-CTE-0340", item);
            return None;
        };
        if eligible.next().is_some() {
            tie_breaker_observed = true;
        }
        emitted[next] = true;
        order.push(requests[next].name.clone());
        for &successor in &outgoing[next] {
            indegree[successor] = indegree[successor].saturating_sub(1);
        }
    }
    let payload = format!("order:{}", order.join(","));
    let span = span_of_item(item);
    if has_dependency_edge {
        uv_core::spec_trace::Conformance::record_at("requirement.22.DeriveExecutionOrder", Some(&span), &payload);
    }
    if tie_breaker_observed {
        uv_core::spec_trace::Conformance::record_at("requirement.22.DeriveOrderTieBreaker", Some(&span), &payload);
    }
    Some(order)
}

pub fn is_derive_annotated_item(item: &ASTItem) -> bool {
    match item {
        ASTItem::RecordDecl(d) => has_attribute(&d.attrs, "derive"),
        ASTItem::EnumDecl(d) => has_attribute(&d.attrs, "derive"),
        ASTItem::ModalDecl(d) => has_attribute(&d.attrs, "derive"),
        _ => false,
    }
}

/// The items produced by the derive targets of an annotated declaration.
pub fn expand_derives(item: &ASTItem, env: &mut CtEnv) -> Option<Vec<ASTItem>> {
    let order = derive_order_for(item, env)?;
    run_derive_set(item, &order, env)
}
