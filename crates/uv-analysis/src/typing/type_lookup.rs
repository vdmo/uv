//! Finding nominal declarations and record fields.

use uv_source::ast;

use super::type_lower::lower_type;
use super::types::*;
use crate::context::*;
use crate::generics::monomorphize::{build_substitution, instantiate_type};
use crate::resolve::scopes::{id_key_of, path_key_of};
use crate::resolve::scopes_lookup::{resolve_module_name, resolve_type_name};

fn lookup_at<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<(&'c TypeDecl, TypePath)> {
    ctx.sigma.types.get(&path_key_of(path)).map(|decl| (decl, path.to_vec()))
}

/// A bare name through the scopes, or a path whose head is a module alias.
fn lookup_scoped<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<(&'c TypeDecl, TypePath)> {
    let (head, suffix) = path.split_first()?;
    if suffix.is_empty() {
        let entity = resolve_type_name(ctx, head)?;
        let mut resolved = entity.origin_opt.unwrap_or_default();
        resolved.push(entity.target_opt.unwrap_or_else(|| head.clone()));
        return lookup_at(ctx, &resolved);
    }
    let mut resolved = resolve_module_name(ctx, head)?.origin_opt?;
    resolved.extend(suffix.iter().cloned());
    lookup_at(ctx, &resolved)
}

/// Relative to the current module, then (for a qualified path) to its assembly.
fn lookup_module_relative<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<(&'c TypeDecl, TypePath)> {
    let assembly = ctx.current_module.first()?;
    if path.is_empty() {
        return None;
    }
    let qualify = |prefix: &[String]| [prefix, path].concat();
    lookup_at(ctx, &qualify(&ctx.current_module))
        .or_else(|| if path.len() >= 2 { lookup_at(ctx, &qualify(std::slice::from_ref(assembly))) } else { None })
}

/// The declaration a type path names, with the path it was found under.
pub fn lookup_type_decl_resolved<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<(&'c TypeDecl, TypePath)> {
    lookup_at(ctx, path).or_else(|| lookup_scoped(ctx, path)).or_else(|| lookup_module_relative(ctx, path))
}

pub fn lookup_type_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c TypeDecl> {
    lookup_type_decl_resolved(ctx, path).map(|(decl, _)| decl)
}

pub fn lookup_record_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::RecordDecl> {
    match lookup_type_decl(ctx, path)? {
        TypeDecl::Record(decl) => Some(decl),
        _ => None,
    }
}

pub fn lookup_enum_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::EnumDecl> {
    match lookup_type_decl(ctx, path)? {
        TypeDecl::Enum(decl) => Some(decl),
        _ => None,
    }
}

pub fn type_params_of<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c Option<ast::GenericParams>> {
    Some(match lookup_type_decl(ctx, path)? {
        TypeDecl::Record(decl) => &decl.generic_params,
        TypeDecl::Enum(decl) => &decl.generic_params,
        TypeDecl::Modal(decl) => &decl.generic_params,
        TypeDecl::TypeAlias(decl) => &decl.generic_params,
    })
}

pub fn record_fields(record: &ast::RecordDecl) -> Vec<&ast::FieldDecl> {
    record
        .members
        .iter()
        .filter_map(|member| match member {
            ast::RecordMember::FieldDecl(field) => Some(field),
            _ => None,
        })
        .collect()
}

pub fn lookup_field_decl<'r>(record: &'r ast::RecordDecl, field_name: &str) -> Option<&'r ast::FieldDecl> {
    let key = id_key_of(field_name);
    record_fields(record).into_iter().find(|field| id_key_of(&field.name) == key)
}

pub fn field_exists(record: &ast::RecordDecl, field_name: &str) -> bool {
    lookup_field_decl(record, field_name).is_some()
}

/// A field that does not exist counts as private.
pub fn field_vis(record: &ast::RecordDecl, field_name: &str) -> ast::Visibility {
    lookup_field_decl(record, field_name).map_or(ast::Visibility::Private, |field| field.vis)
}

/// Private fields are visible in the module that declares the record.
pub fn field_visible(ctx: &ScopeContext<'_>, record: &ast::RecordDecl, field_name: &str, record_path: &[String]) -> bool {
    match field_vis(record, field_name) {
        ast::Visibility::Public | ast::Visibility::Internal => true,
        ast::Visibility::Private => {
            record_path.split_last().is_some_and(|(_, declaring_module)| declaring_module == ctx.current_module)
        }
    }
}

/// The type of a field for the given arguments of its record.
pub fn field_type(
    record: &ast::RecordDecl,
    field_name: &str,
    ctx: &ScopeContext<'_>,
    generic_args: &[TypeRef],
) -> Option<TypeRef> {
    let field = lookup_field_decl(record, field_name)?;
    let lowered = lower_type(ctx, &field.r#type).ok().filter(|ty| ty.is_some())?;
    match &record.generic_params {
        Some(params) if generic_args.len() > params.params.len() => None,
        Some(params) => Some(instantiate_type(&lowered, &build_substitution(&params.params, generic_args))),
        None if !generic_args.is_empty() => None,
        None => Some(lowered),
    }
}

fn lookup_type_alias_decl<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::TypeAliasDecl> {
    let as_alias = |decl: &'c TypeDecl| match decl {
        TypeDecl::TypeAlias(alias) => Some(alias),
        _ => None,
    };
    if let Some(decl) = ctx.sigma.types.get(&path_key_of(path)) {
        return as_alias(decl);
    }
    let [name] = path else {
        return None;
    };
    let entity = resolve_type_name(ctx, name)?;
    let mut resolved = entity.origin_opt?;
    resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
    as_alias(ctx.sigma.types.get(&path_key_of(&resolved))?)
}

/// The asynchronous signature of a type, looking through user type aliases. An alias
/// cycle has none (the reference recurses until it crashes there).
pub fn async_sig_of(ctx: &ScopeContext<'_>, type_ref: &TypeRef) -> Option<AsyncSig> {
    async_sig_through_aliases(ctx, type_ref, &mut Vec::new())
}

fn async_sig_through_aliases<'c>(
    ctx: &'c ScopeContext<'_>,
    type_ref: &TypeRef,
    seen: &mut Vec<&'c ast::TypeAliasDecl>,
) -> Option<AsyncSig> {
    if let Some(sig) = get_async_sig(type_ref) {
        return Some(sig);
    }
    let ty = type_ref.as_deref()?;
    let path = applied_type_path(ty).filter(|path| !path.is_empty())?;
    let args = applied_type_args(ty)?;
    let alias = lookup_type_alias_decl(ctx, path)?;
    if seen.iter().any(|visited| std::ptr::eq(*visited, alias)) {
        return None;
    }
    seen.push(alias);
    let lowered = lower_type(ctx, &alias.r#type).ok()?;
    let instantiated = match &alias.generic_params {
        Some(params) if args.len() > params.params.len() => return None,
        Some(params) => instantiate_type(&lowered, &build_substitution(&params.params, args)),
        None if !args.is_empty() => return None,
        None => lowered,
    };
    instantiated.as_ref()?;
    async_sig_through_aliases(ctx, &instantiated, seen)
}
