//! The `introspect` capability: reflection over the declarations visible to compile-time
//! code, with the same visibility and import rules name resolution applies.

use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::emit;
use uv_core::spec_rule;
use uv_core::unicode::nfc;
use uv_source::ast::*;
use uv_source::module_paths::resolve_import_module_path;

use crate::value::*;

fn id_eq(lhs: &str, rhs: &str) -> bool {
    lhs == rhs || nfc(lhs) == nfc(rhs)
}

fn path_eq(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| id_eq(a, b))
}

fn path_prefix(prefix: &[String], path: &[String]) -> bool {
    prefix.len() <= path.len() && prefix.iter().zip(path).all(|(a, b)| id_eq(a, b))
}

fn same_assembly(lhs: &[String], rhs: &[String]) -> bool {
    matches!((lhs.first(), rhs.first()), (Some(a), Some(b)) if id_eq(a, b))
}

fn visibility_text(vis: Visibility) -> &'static str {
    match vis {
        Visibility::Public => "public",
        Visibility::Internal => "internal",
        Visibility::Private => "private",
    }
}

fn find_available_module<'a>(env: &'a CtEnv, path: &[String]) -> Option<&'a Rc<ASTModule>> {
    env.available_modules.iter().find(|module| path_eq(&module.path, path))
}

/// The items of a module as compile-time code sees them. `current_module` is the module
/// the lookup is made from; for that module the items include what has been emitted so far.
fn find_visible_module_items(env: &CtEnv, current_module: &[String], path: &[String]) -> Option<Vec<ASTItem>> {
    if path_eq(path, current_module) {
        if let Some(items) = &env.current_module_items {
            return Some(items.borrow().clone());
        }
    }
    find_available_module(env, path).map(|module| module.items.clone())
}

fn can_access_vis(accessor_module: &[String], decl_module: &[String], vis: Visibility) -> bool {
    match vis {
        Visibility::Public => true,
        Visibility::Internal => same_assembly(accessor_module, decl_module),
        Visibility::Private => path_eq(accessor_module, decl_module),
    }
}

fn item_name_of(item: &ASTItem) -> Option<&str> {
    match item {
        ASTItem::RecordDecl(d) => Some(&d.name),
        ASTItem::EnumDecl(d) => Some(&d.name),
        ASTItem::ModalDecl(d) => Some(&d.name),
        ASTItem::ClassDecl(d) => Some(&d.name),
        ASTItem::TypeAliasDecl(d) => Some(&d.name),
        _ => None,
    }
}

fn item_visibility_opt(item: &ASTItem) -> Option<Visibility> {
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
        ASTItem::DeriveTargetDecl(_) | ASTItem::ErrorItem(_) => None,
    }
}

/// The module an import of `current_module` resolves to, when it is available.
fn resolve_available_import(env: &CtEnv, current_module: &[String], import: &ImportDecl) -> Option<Vec<String>> {
    resolve_import_module_path(current_module, &env.available_module_names, &import.path)
        .filter(|resolved| find_available_module(env, resolved).is_some())
}

/// Rewrites a path whose head is an import alias of the current module.
fn expand_import_alias_prefix(env: &CtEnv, current_module: &[String], path: &[String]) -> Option<Vec<String>> {
    let head = path.first()?;
    let current_items = find_visible_module_items(env, current_module, current_module)?;
    for item in &current_items {
        let ASTItem::ImportDecl(import) = item else {
            continue;
        };
        let Some(resolved) = resolve_available_import(env, current_module, import) else {
            continue;
        };
        let alias = import.alias_opt.as_ref().or(resolved.last());
        if !alias.is_some_and(|alias| !alias.is_empty() && id_eq(alias, head)) {
            continue;
        }
        let mut expanded = resolved;
        expanded.extend(path[1..].iter().cloned());
        return Some(expanded);
    }
    None
}

fn resolve_visible_module_path(env: &CtEnv, current_module: &[String], path: &[String]) -> Option<Vec<String>> {
    if path.is_empty() {
        return None;
    }
    let path = expand_import_alias_prefix(env, current_module, path).unwrap_or_else(|| path.to_vec());
    resolve_import_module_path(current_module, &env.available_module_names, &path)
        .filter(|resolved| find_available_module(env, resolved).is_some())
}

/// A module of another assembly is reachable only through an import that covers it.
fn import_ok(env: &CtEnv, current_module: &[String], path: &[String]) -> bool {
    let import_required = !current_module.is_empty() && !path.is_empty() && !same_assembly(current_module, path);
    if !import_required {
        return true;
    }
    let Some(current_items) = find_visible_module_items(env, current_module, current_module) else {
        return false;
    };
    current_items.iter().any(|item| match item {
        ASTItem::ImportDecl(import) => {
            resolve_import_module_path(current_module, &env.available_module_names, &import.path)
                .is_some_and(|resolved| path_prefix(&resolved, path))
        }
        _ => false,
    })
}

#[derive(Clone)]
enum NamedDecl {
    Record(RecordDecl),
    Enum(EnumDecl),
    Modal(ModalDecl),
    TypeAlias(TypeAliasDecl),
    Class(ClassDecl),
}

#[derive(Clone)]
struct NamedDeclRef {
    module_path: Vec<String>,
    decl: NamedDecl,
}

fn maybe_add_named_decl(
    out: &mut Vec<NamedDeclRef>,
    item: &ASTItem,
    module_path: &[String],
    accessor_module: &[String],
    lookup_name: &str,
) {
    let (Some(name), Some(vis)) = (item_name_of(item), item_visibility_opt(item)) else {
        return;
    };
    if !id_eq(name, lookup_name) || !can_access_vis(accessor_module, module_path, vis) {
        return;
    }
    let decl = match item {
        ASTItem::RecordDecl(d) => NamedDecl::Record(d.clone()),
        ASTItem::EnumDecl(d) => NamedDecl::Enum(d.clone()),
        ASTItem::ModalDecl(d) => NamedDecl::Modal(d.clone()),
        ASTItem::TypeAliasDecl(d) => NamedDecl::TypeAlias(d.clone()),
        ASTItem::ClassDecl(d) => NamedDecl::Class(d.clone()),
        _ => return,
    };
    out.push(NamedDeclRef { module_path: module_path.to_vec(), decl });
}

/// Declarations named `name` in a module, as seen from `accessor_module`. A declaration
/// that exists only in the expanded form of an earlier module was emitted there and
/// cannot be referred to across modules.
fn find_named_decls_in_module(
    env: &CtEnv,
    module_path: &[String],
    accessor_module: &[String],
    name: &str,
) -> Vec<NamedDeclRef> {
    let mut out = Vec::new();
    if let Some(items) = find_visible_module_items(env, &env.current_module, module_path) {
        for item in &items {
            maybe_add_named_decl(&mut out, item, module_path, accessor_module, name);
        }
    }
    if out.is_empty() && !path_eq(module_path, &env.current_module) {
        spec_rule!("CtExpand-CrossModule-Emit-Err");
        if let Some(expanded) = env.phase2_expanded_modules.iter().find(|module| path_eq(&module.path, module_path)) {
            let mut emitted = Vec::new();
            for item in &expanded.items {
                maybe_add_named_decl(&mut emitted, item, module_path, accessor_module, name);
            }
            if let (false, Some(diags)) = (emitted.is_empty(), &env.diags) {
                if let Some(diag) = make_diagnostic_by_id("E-CTE-0090", Some(env.current_span.clone())) {
                    emit(&mut diags.borrow_mut(), diag);
                }
            }
        }
    }
    out
}

fn append_using_bound_named_decls(
    out: &mut Vec<NamedDeclRef>,
    env: &CtEnv,
    accessor_module: &[String],
    decl: &UsingDecl,
    lookup_name: &str,
) {
    let module_path = match &decl.clause {
        UsingClause::UsingItem(clause) => &clause.module_path,
        UsingClause::UsingList(clause) => &clause.module_path,
        UsingClause::UsingWildcard(clause) => &clause.module_path,
    };
    let Some(resolved_module) = resolve_visible_module_path(env, accessor_module, module_path)
        .filter(|resolved| import_ok(env, accessor_module, resolved))
    else {
        return;
    };
    let mut add = |bind_name: &str, target_name: &str| {
        if id_eq(bind_name, lookup_name) {
            out.extend(find_named_decls_in_module(env, &resolved_module, accessor_module, target_name));
        }
    };
    match &decl.clause {
        UsingClause::UsingItem(clause) => add(clause.alias_opt.as_ref().unwrap_or(&clause.name), &clause.name),
        UsingClause::UsingList(clause) => {
            for spec in &clause.specs {
                add(spec.alias_opt.as_ref().unwrap_or(&spec.name), &spec.name);
            }
        }
        UsingClause::UsingWildcard(_) => add(lookup_name, lookup_name),
    }
}

/// The declarations a type path names when written in `accessor_module`.
fn resolve_named_decls_in_context(env: &CtEnv, accessor_module: &[String], path: &[String]) -> Vec<NamedDeclRef> {
    let Some((name, module_path)) = path.split_last() else {
        return Vec::new();
    };
    if module_path.is_empty() {
        let mut out = find_named_decls_in_module(env, accessor_module, accessor_module, name);
        if let Some(items) = find_visible_module_items(env, &env.current_module, accessor_module) {
            for item in &items {
                if let ASTItem::UsingDecl(using_decl) = item {
                    append_using_bound_named_decls(&mut out, env, accessor_module, using_decl, name);
                }
            }
        }
        return out;
    }
    match resolve_visible_module_path(env, accessor_module, module_path)
        .filter(|resolved| import_ok(env, accessor_module, resolved))
    {
        Some(resolved_module) => find_named_decls_in_module(env, &resolved_module, accessor_module, name),
        None => Vec::new(),
    }
}

fn resolve_unique_named_decl_in_context(env: &CtEnv, accessor_module: &[String], path: &[String]) -> Option<NamedDeclRef> {
    let mut decls = resolve_named_decls_in_context(env, accessor_module, path);
    if decls.len() == 1 {
        decls.pop()
    } else {
        None
    }
}

impl NamedDeclRef {
    fn has_reflect_attribute(&self) -> bool {
        match &self.decl {
            NamedDecl::Record(d) => has_attribute(&d.attrs, "reflect"),
            NamedDecl::Enum(d) => has_attribute(&d.attrs, "reflect"),
            NamedDecl::Modal(d) => has_attribute(&d.attrs, "reflect"),
            _ => false,
        }
    }

    fn name(&self) -> &str {
        match &self.decl {
            NamedDecl::Record(d) => &d.name,
            NamedDecl::Enum(d) => &d.name,
            NamedDecl::Modal(d) => &d.name,
            NamedDecl::TypeAlias(d) => &d.name,
            NamedDecl::Class(d) => &d.name,
        }
    }

    fn full_path(&self) -> Vec<String> {
        let mut path = self.module_path.clone();
        path.push(self.name().to_string());
        path
    }

    /// Generic parameters of the declaration; classes are not instantiated here.
    fn generic_params(&self) -> Option<&Option<GenericParams>> {
        match &self.decl {
            NamedDecl::Record(d) => Some(&d.generic_params),
            NamedDecl::Enum(d) => Some(&d.generic_params),
            NamedDecl::Modal(d) => Some(&d.generic_params),
            NamedDecl::TypeAlias(d) => Some(&d.generic_params),
            NamedDecl::Class(_) => None,
        }
    }
}

type AstTypeSubst = std::collections::HashMap<String, TypePtr>;
type AliasVisiting = std::collections::HashSet<String>;

fn make_type(span: &uv_core::span::Span, node: TypeNode) -> TypePtr {
    Some(std::sync::Arc::new(Type { span: span.clone(), node }))
}

fn substitute_types(types: &[TypePtr], subst: &AstTypeSubst) -> Vec<TypePtr> {
    types.iter().map(|ty| substitute_ast_type(ty, subst)).collect()
}

fn substitute_func_params(params: &[TypeFuncParam], subst: &AstTypeSubst) -> Vec<TypeFuncParam> {
    params
        .iter()
        .map(|param| TypeFuncParam { mode: param.mode, r#type: substitute_ast_type(&param.r#type, subst) })
        .collect()
}

/// Replaces every use of a generic parameter (a bare single-segment path) by its argument.
fn substitute_ast_type(ty_ptr: &TypePtr, subst: &AstTypeSubst) -> TypePtr {
    let ty = ty_ptr.as_ref()?;
    if let TypeNode::TypePathType(path) = &ty.node {
        if path.path.len() == 1 && path.generic_args.is_empty() {
            if let Some(replacement) = subst.get(&path.path[0]) {
                return replacement.clone();
            }
        }
    }
    let base = |base: &TypePtr| substitute_ast_type(base, subst);
    let node: TypeNode = match &ty.node {
        TypeNode::TypePermType(n) => TypePermType { perm: n.perm, base: base(&n.base) }.into(),
        TypeNode::TypeUnion(n) => TypeUnion { types: substitute_types(&n.types, subst) }.into(),
        TypeNode::TypeFunc(n) => {
            TypeFunc { params: substitute_func_params(&n.params, subst), ret: base(&n.ret) }.into()
        }
        TypeNode::TypeClosure(n) => TypeClosure {
            params: substitute_func_params(&n.params, subst),
            ret: base(&n.ret),
            deps_opt: n.deps_opt.as_ref().map(|deps| {
                deps.iter().map(|dep| SharedDep { name: dep.name.clone(), r#type: base(&dep.r#type) }).collect()
            }),
        }
        .into(),
        TypeNode::TypeTuple(n) => TypeTuple { elements: substitute_types(&n.elements, subst) }.into(),
        TypeNode::TypeArray(n) => TypeArray { element: base(&n.element), length: n.length.clone() }.into(),
        TypeNode::TypeSlice(n) => TypeSlice { element: base(&n.element) }.into(),
        TypeNode::TypeSafePtr(n) => TypeSafePtr { element: base(&n.element), state: n.state }.into(),
        TypeNode::TypeRawPtr(n) => TypeRawPtr { qual: n.qual, element: base(&n.element) }.into(),
        TypeNode::TypeModalState(n) => modal_state_type(n.path.clone(), substitute_types(&n.generic_args, subst), &n.state),
        TypeNode::TypePathType(n) => {
            TypePathType { path: n.path.clone(), generic_args: substitute_types(&n.generic_args, subst) }.into()
        }
        TypeNode::TypeApply(n) => TypeApply { path: n.path.clone(), args: substitute_types(&n.args, subst) }.into(),
        TypeNode::TypeRefine(n) => TypeRefine { base: base(&n.base), predicate: n.predicate.clone() }.into(),
        TypeNode::TypeRange(n) => TypeRange { base: base(&n.base) }.into(),
        TypeNode::TypeRangeInclusive(n) => TypeRangeInclusive { base: base(&n.base) }.into(),
        TypeNode::TypeRangeFrom(n) => TypeRangeFrom { base: base(&n.base) }.into(),
        TypeNode::TypeRangeTo(n) => TypeRangeTo { base: base(&n.base) }.into(),
        TypeNode::TypeRangeToInclusive(n) => TypeRangeToInclusive { base: base(&n.base) }.into(),
        other => other.clone(),
    };
    make_type(&ty.span, node)
}

fn modal_state_type(path: Vec<String>, generic_args: Vec<TypePtr>, state: &str) -> TypeNode {
    TypeModalState {
        modal_ref: uv_source::parser::make_type_modal_ref(path.clone(), generic_args.clone()),
        path,
        generic_args,
        state: state.to_string(),
    }
    .into()
}

/// Completes the generic arguments of a use from the declaration's defaults.
fn resolve_ast_generic_args(generic_params: Option<&Option<GenericParams>>, provided_args: &[TypePtr]) -> Option<Vec<TypePtr>> {
    let params = match generic_params.and_then(|params| params.as_ref()) {
        Some(params) if !params.params.is_empty() => &params.params,
        _ => return provided_args.is_empty().then(Vec::new),
    };
    if provided_args.len() > params.len() {
        return None;
    }
    let mut out = Vec::with_capacity(params.len());
    let mut subst = AstTypeSubst::new();
    for (index, param) in params.iter().enumerate() {
        let source = match provided_args.get(index) {
            Some(arg) => arg,
            None if param.default_type.is_some() => &param.default_type,
            None => return None,
        };
        let arg = substitute_ast_type(source, &subst);
        subst.insert(param.name.clone(), arg.clone());
        out.push(arg);
    }
    Some(out)
}

fn instantiate_ast_decl_type(ty: &TypePtr, params: Option<&Option<GenericParams>>, args: &[TypePtr]) -> TypePtr {
    ty.as_ref()?;
    let type_params = match params.and_then(|params| params.as_ref()) {
        Some(params) if !params.params.is_empty() => &params.params,
        _ => return ty.clone(),
    };
    let subst: AstTypeSubst =
        type_params.iter().zip(args).map(|(param, arg)| (param.name.clone(), arg.clone())).collect();
    substitute_ast_type(ty, &subst)
}

fn expand_reflect_alias_type_in_context(
    env: &CtEnv,
    accessor_module: &[String],
    path: &[String],
    provided_args: &[TypePtr],
    alias_visiting: &mut AliasVisiting,
) -> TypePtr {
    let decl = resolve_unique_named_decl_in_context(env, accessor_module, path)?;
    let NamedDecl::TypeAlias(alias) = &decl.decl else {
        return None;
    };
    alias.r#type.as_ref()?;
    let alias_key = uv_core::symbols::string_of_path(&decl.full_path());
    if !alias_visiting.insert(alias_key.clone()) {
        return None;
    }
    let expanded = resolve_ast_generic_args(decl.generic_params(), provided_args).map(|args| {
        let instantiated = instantiate_ast_decl_type(&alias.r#type, decl.generic_params(), &args);
        canonicalize_reflect_type_in_context(env, &decl.module_path, &instantiated, alias_visiting)
    });
    alias_visiting.remove(&alias_key);
    expanded.flatten()
}

struct ReflectNominalTarget {
    decl: NamedDeclRef,
    generic_args: Vec<TypePtr>,
}

fn resolve_path_reflect_target(
    env: &CtEnv,
    accessor_module: &[String],
    path: &[String],
    provided_args: &[TypePtr],
    alias_visiting: &mut AliasVisiting,
) -> Option<ReflectNominalTarget> {
    let decl = resolve_unique_named_decl_in_context(env, accessor_module, path)?;
    match &decl.decl {
        NamedDecl::TypeAlias(_) => {
            let expanded =
                expand_reflect_alias_type_in_context(env, accessor_module, path, provided_args, alias_visiting);
            expanded.as_ref()?;
            resolve_reflect_nominal_target_in_context(env, &decl.module_path, &expanded, alias_visiting)
        }
        NamedDecl::Class(_) => None,
        _ => {
            let generic_args = resolve_ast_generic_args(decl.generic_params(), provided_args)?;
            Some(ReflectNominalTarget { decl, generic_args })
        }
    }
}

fn resolve_reflect_nominal_target_in_context(
    env: &CtEnv,
    accessor_module: &[String],
    ty: &TypePtr,
    alias_visiting: &mut AliasVisiting,
) -> Option<ReflectNominalTarget> {
    match &ty.as_ref()?.node {
        TypeNode::TypePermType(n) => resolve_reflect_nominal_target_in_context(env, accessor_module, &n.base, alias_visiting),
        TypeNode::TypeRefine(n) => resolve_reflect_nominal_target_in_context(env, accessor_module, &n.base, alias_visiting),
        TypeNode::TypePathType(n) => resolve_path_reflect_target(env, accessor_module, &n.path, &n.generic_args, alias_visiting),
        TypeNode::TypeApply(n) => resolve_path_reflect_target(env, accessor_module, &n.path, &n.args, alias_visiting),
        TypeNode::TypeModalState(n) => {
            resolve_path_reflect_target(env, accessor_module, &n.path, &n.generic_args, alias_visiting)
        }
        _ => None,
    }
}

fn resolve_reflect_nominal_target(env: &CtEnv, ty: &TypePtr) -> Option<ReflectNominalTarget> {
    resolve_reflect_nominal_target_in_context(env, &env.current_module, ty, &mut AliasVisiting::new())
}

/// The type with permissions and refinements removed, aliases expanded and nominal paths
/// written in full.
fn canonicalize_reflect_type_in_context(
    env: &CtEnv,
    accessor_module: &[String],
    ty_ptr: &TypePtr,
    alias_visiting: &mut AliasVisiting,
) -> TypePtr {
    let ty = ty_ptr.as_ref()?;
    let (path, args, state) = match &ty.node {
        TypeNode::TypePermType(n) => {
            return canonicalize_reflect_type_in_context(env, accessor_module, &n.base, alias_visiting)
        }
        TypeNode::TypeRefine(n) => {
            return canonicalize_reflect_type_in_context(env, accessor_module, &n.base, alias_visiting)
        }
        TypeNode::TypePathType(n) => (&n.path, &n.generic_args, None),
        TypeNode::TypeApply(n) => (&n.path, &n.args, None),
        TypeNode::TypeModalState(n) => (&n.path, &n.generic_args, Some(&n.state)),
        _ => return ty_ptr.clone(),
    };
    let expanded = expand_reflect_alias_type_in_context(env, accessor_module, path, args, alias_visiting);
    if expanded.is_some() {
        return expanded;
    }
    let Some(target) = resolve_path_reflect_target(env, accessor_module, path, args, alias_visiting) else {
        return ty_ptr.clone();
    };
    let full_path = target.decl.full_path();
    let node = match state {
        Some(state) => modal_state_type(full_path, target.generic_args, state),
        None => TypePathType { path: full_path, generic_args: target.generic_args }.into(),
    };
    make_type(&ty.span, node)
}

fn resolve_reflect_class_path_in_context(env: &CtEnv, accessor_module: &[String], path: &[String]) -> Vec<String> {
    match resolve_unique_named_decl_in_context(env, accessor_module, path) {
        Some(decl) if matches!(decl.decl, NamedDecl::Class(_)) => decl.full_path(),
        _ => path.to_vec(),
    }
}

fn resolve_reflect_class_decl_in_context(env: &CtEnv, accessor_module: &[String], path: &[String]) -> Option<(NamedDeclRef, ClassDecl)> {
    let resolved = resolve_reflect_class_path_in_context(env, accessor_module, path);
    let decl = resolve_unique_named_decl_in_context(env, accessor_module, &resolved)?;
    match &decl.decl {
        NamedDecl::Class(class_decl) => {
            let class_decl = class_decl.clone();
            Some((decl, class_decl))
        }
        _ => None,
    }
}

fn reflect_class_subtypes_in_context(
    env: &CtEnv,
    accessor_module: &[String],
    sub: &[String],
    sup: &[String],
    visiting: &mut AliasVisiting,
) -> bool {
    let resolved_sub = resolve_reflect_class_path_in_context(env, accessor_module, sub);
    let resolved_sup = resolve_reflect_class_path_in_context(env, accessor_module, sup);
    if path_eq(&resolved_sub, &resolved_sup) {
        return true;
    }
    let visit_key = uv_core::symbols::string_of_path(&resolved_sub);
    if !visiting.insert(visit_key.clone()) {
        return false;
    }
    let found = resolve_reflect_class_decl_in_context(env, accessor_module, &resolved_sub).is_some_and(
        |(decl, class_decl)| {
            class_decl.supers.iter().any(|super_path| {
                reflect_class_subtypes_in_context(env, &decl.module_path, super_path, &resolved_sup, visiting)
            })
        },
    );
    visiting.remove(&visit_key);
    found
}

impl ReflectNominalTarget {
    fn implements(&self) -> Option<&Vec<Vec<String>>> {
        match &self.decl.decl {
            NamedDecl::Record(d) => Some(&d.implements),
            NamedDecl::Enum(d) => Some(&d.implements),
            NamedDecl::Modal(d) => Some(&d.implements),
            _ => None,
        }
    }
}

fn reflect_nominal_implements_form(env: &CtEnv, target: &ReflectNominalTarget, form_path: &[String]) -> bool {
    let Some(impls) = target.implements() else {
        return false;
    };
    let module = &target.decl.module_path;
    let resolved_form = resolve_reflect_class_path_in_context(env, module, form_path);
    impls
        .iter()
        .any(|implemented| reflect_class_subtypes_in_context(env, module, implemented, &resolved_form, &mut AliasVisiting::new()))
}

fn make_type_category_value(variant: &str) -> CtValue {
    CtValue::Enum(Rc::new(CtEnum {
        path: vec!["TypeCategory".to_string()],
        variant: variant.to_string(),
        payload: CtPayload::None,
    }))
}

fn make_string_array_value(values: impl IntoIterator<Item = String>) -> CtValue {
    CtValue::Slice(Rc::new(values.into_iter().map(CtValue::String).collect()))
}

fn make_type_array_value(values: impl IntoIterator<Item = TypePtr>) -> CtValue {
    CtValue::Slice(Rc::new(values.into_iter().map(CtValue::Type).collect()))
}

fn make_record_value(path: &str, fields: Vec<(&str, CtValue)>) -> CtValue {
    CtValue::Record(Rc::new(CtRecord {
        path: vec![path.to_string()],
        fields: fields.into_iter().map(|(name, value)| (name.to_string(), value)).collect(),
    }))
}

fn make_field_info_value(name: &str, ty: &TypePtr, vis: Visibility, index: usize, span: &uv_core::span::Span) -> CtValue {
    make_record_value(
        "FieldInfo",
        vec![
            ("name", CtValue::String(name.to_string())),
            ("type", CtValue::Type(ty.clone())),
            ("visibility", CtValue::String(visibility_text(vis).to_string())),
            ("index", make_ct_int(index as u64, "usize", "")),
            ("span", make_span_value(span)),
        ],
    )
}

fn payload_kind_text(payload_opt: &Option<VariantPayload>) -> &'static str {
    match payload_opt {
        None => "unit",
        Some(VariantPayload::VariantPayloadTuple(_)) => "tuple",
        Some(VariantPayload::VariantPayloadRecord(_)) => "record",
    }
}

fn payload_types(payload_opt: &Option<VariantPayload>) -> Vec<TypePtr> {
    match payload_opt {
        None => Vec::new(),
        Some(VariantPayload::VariantPayloadTuple(tuple)) => tuple.elements.clone(),
        Some(VariantPayload::VariantPayloadRecord(record)) => {
            record.fields.iter().map(|field| field.r#type.clone()).collect()
        }
    }
}

fn payload_field_names(payload_opt: &Option<VariantPayload>) -> Vec<String> {
    match payload_opt {
        Some(VariantPayload::VariantPayloadRecord(record)) => {
            record.fields.iter().map(|field| field.name.clone()).collect()
        }
        _ => Vec::new(),
    }
}

fn make_variant_info_value(
    name: &str,
    payload_kind: &str,
    payload_types: Vec<TypePtr>,
    field_names: Vec<String>,
    span: &uv_core::span::Span,
) -> CtValue {
    make_record_value(
        "VariantInfo",
        vec![
            ("name", CtValue::String(name.to_string())),
            ("payload_kind", CtValue::String(payload_kind.to_string())),
            ("payload_types", make_type_array_value(payload_types)),
            ("field_names", make_string_array_value(field_names)),
            ("span", make_span_value(span)),
        ],
    )
}

fn make_state_info_value(state: &StateBlock) -> CtValue {
    let (mut field_names, mut method_names, mut transition_names) = (Vec::new(), Vec::new(), Vec::new());
    for member in &state.members {
        match member {
            StateMember::StateFieldDecl(node) => field_names.push(node.name.clone()),
            StateMember::StateMethodDecl(node) => method_names.push(node.name.clone()),
            StateMember::TransitionDecl(node) => transition_names.push(node.name.clone()),
        }
    }
    make_record_value(
        "StateInfo",
        vec![
            ("name", CtValue::String(state.name.clone())),
            ("field_names", make_string_array_value(field_names)),
            ("method_names", make_string_array_value(method_names)),
            ("transition_names", make_string_array_value(transition_names)),
            ("span", make_span_value(&state.span)),
        ],
    )
}

fn category_of_named_decl(decl: &NamedDeclRef) -> Option<CtValue> {
    match &decl.decl {
        NamedDecl::Record(_) => Some(make_type_category_value("Record")),
        NamedDecl::Enum(_) => Some(make_type_category_value("Enum")),
        NamedDecl::Modal(_) => Some(make_type_category_value("Modal")),
        _ => None,
    }
}

type TypeVisiting = std::collections::HashSet<*const Type>;

fn category_of_path(env: &CtEnv, path: &[String], args: &[TypePtr], visiting: &mut TypeVisiting) -> Option<CtValue> {
    match resolve_unique_named_decl_in_context(env, &env.current_module, path) {
        Some(decl) if matches!(decl.decl, NamedDecl::TypeAlias(_)) => {
            let expanded = expand_reflect_alias_type_in_context(
                env,
                &env.current_module,
                path,
                args,
                &mut AliasVisiting::new(),
            );
            expanded.as_ref()?;
            category_of_type(env, &expanded, visiting)
        }
        Some(decl) => category_of_named_decl(&decl),
        None if path.len() == 1 => Some(make_type_category_value("Generic")),
        None => None,
    }
}

fn category_of_type(env: &CtEnv, ty_ptr: &TypePtr, visiting: &mut TypeVisiting) -> Option<CtValue> {
    let ty = ty_ptr.as_ref()?;
    let key = std::sync::Arc::as_ptr(ty);
    if !visiting.insert(key) {
        return None;
    }
    let category = |name: &str| Some(make_type_category_value(name));
    let result = match &ty.node {
        TypeNode::TypePrim(_) => category("Primitive"),
        TypeNode::TypePermType(n) => category_of_type(env, &n.base, visiting),
        TypeNode::TypeRefine(n) => category_of_type(env, &n.base, visiting),
        TypeNode::TypeTuple(_) => category("Tuple"),
        TypeNode::TypeArray(_) => category("Array"),
        TypeNode::TypeSlice(_) => category("Slice"),
        TypeNode::TypeUnion(_) => category("Union"),
        TypeNode::TypeFunc(_) | TypeNode::TypeClosure(_) => category("Procedure"),
        TypeNode::TypeSafePtr(_) | TypeNode::TypeRawPtr(_) => category("Reference"),
        TypeNode::TypeDynamic(_) => category("Dynamic"),
        TypeNode::TypeOpaque(_) => category("Opaque"),
        TypeNode::TypeString(_) => category("String"),
        TypeNode::TypeBytes(_) => category("Bytes"),
        TypeNode::TypeModalState(_) => category("Modal"),
        TypeNode::TypePathType(n) => category_of_path(env, &n.path, &n.generic_args, visiting),
        TypeNode::TypeApply(n) => category_of_path(env, &n.path, &n.args, visiting),
        TypeNode::TypeRange(_)
        | TypeNode::TypeRangeInclusive(_)
        | TypeNode::TypeRangeFrom(_)
        | TypeNode::TypeRangeTo(_)
        | TypeNode::TypeRangeToInclusive(_)
        | TypeNode::TypeRangeFull(_) => category("Range"),
        _ => None,
    };
    visiting.remove(&key);
    result
}

/// A type compile-time code may inspect: structural types always, nominal types only
/// when declared with `#reflect`.
fn reflectable_type(env: &CtEnv, ty_ptr: &TypePtr) -> bool {
    let Some(ty) = ty_ptr else {
        return false;
    };
    let (path, args) = match &ty.node {
        TypeNode::TypePrim(_)
        | TypeNode::TypeTuple(_)
        | TypeNode::TypeArray(_)
        | TypeNode::TypeSlice(_)
        | TypeNode::TypeUnion(_) => return true,
        TypeNode::TypePermType(n) => return reflectable_type(env, &n.base),
        TypeNode::TypeRefine(n) => return reflectable_type(env, &n.base),
        TypeNode::TypePathType(n) => (&n.path, &n.generic_args),
        TypeNode::TypeApply(n) => (&n.path, &n.args),
        TypeNode::TypeModalState(n) => (&n.path, &n.generic_args),
        _ => return false,
    };
    if let Some(target) = resolve_reflect_nominal_target(env, ty_ptr) {
        return target.decl.has_reflect_attribute();
    }
    let expanded =
        expand_reflect_alias_type_in_context(env, &env.current_module, path, args, &mut AliasVisiting::new());
    expanded.is_some() && reflectable_type(env, &expanded)
}

fn make_array_result(values: Vec<CtValue>) -> EvalResult {
    EvalResult { ok: true, value: CtValue::Slice(Rc::new(values)), returned: false }
}

// ---- type names ---------------------------------------------------------------

fn render_types(types: &[TypePtr], out: &mut String) {
    for (index, ty) in types.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        render_type(ty, out);
    }
}

fn render_func_params(params: &[TypeFuncParam], out: &mut String) {
    for (index, param) in params.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        if param.mode.is_some() {
            out.push_str("move ");
        }
        render_type(&param.r#type, out);
    }
}

fn render_path_with_args(path: &[String], args: &[TypePtr], out: &mut String) {
    out.push_str(&path.join("::"));
    if !args.is_empty() {
        out.push('<');
        render_types(args, out);
        out.push('>');
    }
}

fn render_wrapped(name: &str, base: &TypePtr, out: &mut String) {
    out.push_str(name);
    out.push('<');
    render_type(base, out);
    out.push('>');
}

/// The spelling `introspect~>type_name` reports for a type.
fn render_type(ty: &TypePtr, out: &mut String) {
    let Some(ty) = ty else {
        return;
    };
    match &ty.node {
        TypeNode::TypePrim(n) => out.push_str(&n.name),
        TypeNode::TypePermType(n) => {
            out.push_str(match n.perm {
                TypePerm::Const => "const ",
                TypePerm::Unique => "unique ",
                TypePerm::Shared => "shared ",
            });
            render_type(&n.base, out);
        }
        TypeNode::TypeUnion(n) => {
            for (index, member) in n.types.iter().enumerate() {
                if index > 0 {
                    out.push_str(" | ");
                }
                render_type(member, out);
            }
        }
        TypeNode::TypeFunc(n) => {
            out.push('(');
            render_func_params(&n.params, out);
            out.push_str(") -> ");
            render_type(&n.ret, out);
        }
        TypeNode::TypeClosure(n) => {
            out.push('|');
            render_func_params(&n.params, out);
            out.push_str("| -> ");
            render_type(&n.ret, out);
            if let Some(deps) = &n.deps_opt {
                out.push_str(" [shared: {");
                for (index, dep) in deps.iter().enumerate() {
                    if index > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&dep.name);
                    out.push_str(": ");
                    render_type(&dep.r#type, out);
                }
                out.push_str("}]");
            }
        }
        TypeNode::TypeTuple(n) => {
            out.push('(');
            render_types(&n.elements, out);
            if n.elements.len() == 1 {
                out.push(';');
            }
            out.push(')');
        }
        TypeNode::TypeArray(n) => {
            out.push('[');
            render_type(&n.element, out);
            out.push_str("; <length>]");
        }
        TypeNode::TypeSlice(n) => {
            out.push('[');
            render_type(&n.element, out);
            out.push(']');
        }
        TypeNode::TypeSafePtr(n) => {
            render_wrapped("Ptr", &n.element, out);
            if let Some(state) = n.state {
                out.push('@');
                out.push_str(match state {
                    PtrState::Valid => "Valid",
                    PtrState::Null => "Null",
                    PtrState::Expired => "Expired",
                });
            }
        }
        TypeNode::TypeRawPtr(n) => {
            out.push_str(match n.qual {
                RawPtrQual::Imm => "*imm ",
                RawPtrQual::Mut => "*mut ",
            });
            render_type(&n.element, out);
        }
        TypeNode::TypeString(n) => {
            out.push_str("string");
            if let Some(state) = n.state {
                out.push_str(match state {
                    StringState::Managed => "@Managed",
                    StringState::View => "@View",
                });
            }
        }
        TypeNode::TypeBytes(n) => {
            out.push_str("bytes");
            if let Some(state) = n.state {
                out.push_str(match state {
                    BytesState::Managed => "@Managed",
                    BytesState::View => "@View",
                });
            }
        }
        TypeNode::TypeDynamic(n) => {
            out.push('$');
            out.push_str(&n.path.join("::"));
        }
        TypeNode::TypeModalState(n) => {
            match &n.modal_ref {
                TypeModalRef::TypePathType(path) => render_path_with_args(&path.path, &path.generic_args, out),
                TypeModalRef::TypeApply(apply) => render_path_with_args(&apply.path, &apply.args, out),
            }
            out.push('@');
            out.push_str(&n.state);
        }
        TypeNode::TypePathType(n) => render_path_with_args(&n.path, &n.generic_args, out),
        TypeNode::TypeApply(n) => {
            out.push_str(&n.path.join("::"));
            out.push('<');
            render_types(&n.args, out);
            out.push('>');
        }
        TypeNode::TypeOpaque(n) => {
            out.push_str("opaque ");
            out.push_str(&n.path.join("::"));
        }
        TypeNode::TypeRefine(n) => {
            render_type(&n.base, out);
            out.push_str(" |: { ... }");
        }
        TypeNode::TypeRange(n) => render_wrapped("Range", &n.base, out),
        TypeNode::TypeRangeInclusive(n) => render_wrapped("RangeInclusive", &n.base, out),
        TypeNode::TypeRangeFrom(n) => render_wrapped("RangeFrom", &n.base, out),
        TypeNode::TypeRangeTo(n) => render_wrapped("RangeTo", &n.base, out),
        TypeNode::TypeRangeToInclusive(n) => render_wrapped("RangeToInclusive", &n.base, out),
        TypeNode::TypeRangeFull(_) => out.push_str("RangeFull"),
        _ => out.push_str("<unknown type>"),
    }
}

// ---- the methods -----------------------------------------------------------------

fn reflection_diag_span(call: &MethodCallExpr) -> uv_core::span::Span {
    call.args
        .first()
        .and_then(|arg| arg.value.as_ref())
        .or(call.receiver.as_ref())
        .map(|expr| expr.span.clone())
        .unwrap_or_default()
}

fn emit_member_diag(env: &CtEnv, call: &MethodCallExpr, diag_id: &str) -> Option<EvalResult> {
    let span = reflection_diag_span(call);
    uv_core::spec_rule_at!("requirement.22.IntrospectMemberValidity", &span);
    emit_comptime_diag(env, diag_id, &span);
    Some(EvalResult::default())
}

/// Evaluates `introspect~>method(..)`. `None` means the call is not an introspection method.
pub fn eval_introspect_method(call: &MethodCallExpr, env: &mut CtEnv) -> Option<EvalResult> {
    let receiver = call.receiver.as_ref()?;
    let ExprNode::IdentifierExpr(ident) = &receiver.node else {
        return None;
    };
    if !id_eq(&ident.name, "introspect") {
        return None;
    }
    let call_span = receiver.span.clone();
    uv_core::spec_rule_at!("requirement.22.CtCapMethodCallParsing", &call_span);
    uv_core::spec_rule_at!("def.22.CtCapabilityDynamicHelpers", &call_span);
    uv_core::spec_rule_at!("requirement.22.IntrospectAndDiagnosticsAvailability", &call_span);
    let failed = || Some(EvalResult::default());
    let ok = |value: CtValue| Some(EvalResult { ok: true, value, returned: false });
    let pure = || uv_core::spec_rule_at!("requirement.22.ReflectionPurityAndImmutability", &call_span);
    let method = ["category", "type_name", "module_path", "fields", "variants", "states", "implements_form"]
        .into_iter()
        .find(|name| id_eq(&call.name, name))?;
    let eval_type_arg = |index: usize, env: &mut CtEnv| -> Option<TypePtr> {
        let value = crate::eval::eval_expr(&call.args.get(index)?.value, env);
        match (value.ok, value.value) {
            (true, CtValue::Type(ty)) => Some(ty),
            _ => None,
        }
    };
    let Some(ty) = eval_type_arg(0, env) else {
        return failed();
    };
    match method {
        "category" => {
            let Some(category) = category_of_type(env, &ty, &mut TypeVisiting::new()) else {
                return failed();
            };
            spec_rule!("CtBuiltin-Reflect-Category");
            uv_core::spec_rule_at!("requirement.22.IntrospectCategoryValidity", &call_span);
            pure();
            ok(category)
        }
        "type_name" => {
            spec_rule!("CtBuiltin-Reflect-TypeName");
            pure();
            let mut name = String::new();
            render_type(&ty, &mut name);
            ok(CtValue::String(name))
        }
        "module_path" => {
            spec_rule!("CtBuiltin-Reflect-ModulePath");
            pure();
            let text = match ty.as_deref().map(|ty| &ty.node) {
                Some(TypeNode::TypePathType(node)) if node.path.len() >= 2 => {
                    uv_core::symbols::string_of_path(&node.path[..node.path.len() - 1])
                }
                _ => String::new(),
            };
            ok(CtValue::String(text))
        }
        "fields" | "variants" | "states" => {
            if !reflectable_type(env, &ty) {
                return emit_member_diag(env, call, "E-CTE-0053");
            }
            let target = resolve_reflect_nominal_target(env, &ty);
            let decl = target.as_ref().map(|target| &target.decl.decl);
            let instantiate = |member_type: &TypePtr| {
                let target = target.as_ref().expect("target checked before instantiation");
                instantiate_ast_decl_type(member_type, target.decl.generic_params(), &target.generic_args)
            };
            let infos: Vec<CtValue> = match (method, decl) {
                ("fields", Some(NamedDecl::Record(record))) => {
                    let fields = record.members.iter().filter_map(|member| match member {
                        RecordMember::FieldDecl(field) => Some(field),
                        _ => None,
                    });
                    let infos = fields
                        .enumerate()
                        .map(|(index, field)| {
                            make_field_info_value(&field.name, &instantiate(&field.r#type), field.vis, index, &field.span)
                        })
                        .collect();
                    spec_rule!("CtBuiltin-Reflect-Fields");
                    infos
                }
                ("fields", _) => return emit_member_diag(env, call, "E-CTE-0050"),
                ("variants", Some(NamedDecl::Enum(enum_decl))) => {
                    let infos = enum_decl
                        .variants
                        .iter()
                        .map(|variant| {
                            let types = payload_types(&variant.payload_opt).iter().map(&instantiate).collect();
                            make_variant_info_value(
                                &variant.name,
                                payload_kind_text(&variant.payload_opt),
                                types,
                                payload_field_names(&variant.payload_opt),
                                &variant.span,
                            )
                        })
                        .collect();
                    spec_rule!("CtBuiltin-Reflect-Variants");
                    infos
                }
                ("variants", _) => return emit_member_diag(env, call, "E-CTE-0051"),
                (_, Some(NamedDecl::Modal(modal))) => {
                    let infos = modal.states.iter().map(make_state_info_value).collect();
                    spec_rule!("CtBuiltin-Reflect-States");
                    infos
                }
                _ => return emit_member_diag(env, call, "E-CTE-0052"),
            };
            uv_core::spec_rule_at!("requirement.22.ReflectionCanonicalOrder", &call_span);
            pure();
            Some(make_array_result(infos))
        }
        _ => {
            let Some(form) = eval_type_arg(1, env) else {
                return failed();
            };
            let form_path = match form.as_deref().map(|form| &form.node) {
                Some(TypeNode::TypePathType(node)) => &node.path,
                Some(TypeNode::TypeApply(node)) => &node.path,
                _ => return ok(make_ct_bool(false)),
            };
            let form_path = resolve_reflect_class_path_in_context(env, &env.current_module, form_path);
            let Some(target) = resolve_reflect_nominal_target(env, &ty) else {
                return ok(make_ct_bool(false));
            };
            spec_rule!("CtBuiltin-Reflect-Form");
            uv_core::spec_rule_at!("requirement.22.IntrospectImplementsFormSemantics", &call_span);
            pure();
            ok(make_ct_bool(reflect_nominal_implements_form(env, &target, &form_path)))
        }
    }
}
