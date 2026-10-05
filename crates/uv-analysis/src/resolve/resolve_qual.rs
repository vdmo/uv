//! Qualified names in expressions: `a::b`, `a::b(..)` and `a::b { .. }` can each be a
//! value, a record or an enum variant, decided by what the path resolves to.

use uv_project::language_profile::active_language_profile;
use uv_source::ast::*;

use super::resolve_expr::resolve_expr;
use super::resolve_types::resolve_type_path;
use super::resolver::*;
use super::scopes::id_eq;
use super::scopes_lookup::resolve_qualified;
use super::visibility::can_access;
use crate::caps::builtin_paths::*;
use crate::context::*;
use crate::modal::builtin_modal_intrinsics::{is_builtin_modal_member_name, is_builtin_modal_type_path};

/// A failure here carries at most a rule; the caller supplies the span and the detail.
pub type QualRes<T> = Result<T, Option<&'static str>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantKind {
    Unit,
    Tuple,
    Record,
}

fn variant_payload_kind(variant: &VariantDecl) -> VariantKind {
    match &variant.payload_opt {
        None => VariantKind::Unit,
        Some(VariantPayload::VariantPayloadTuple(_)) => VariantKind::Tuple,
        Some(VariantPayload::VariantPayloadRecord(_)) => VariantKind::Record,
    }
}

fn find_variant<'d>(decl: &'d EnumDecl, name: &str) -> Option<&'d VariantDecl> {
    decl.variants.iter().find(|variant| id_eq(&variant.name, name))
}

fn is_runtime_panic_path(path: &[String], name: &str) -> bool {
    matches!(path, [root, module] if id_eq(root, active_language_profile().runtime_root) && id_eq(module, "runtime"))
        && name == "panic"
}

/// Nested lookups made on behalf of a qualified form always use the standard access check.
fn with_standard_access<'c, 'a, T>(
    ctx: &mut ResolveContext<'c, 'a>,
    body: impl FnOnce(&mut ResolveContext<'c, 'a>) -> T,
) -> T {
    let saved = ctx.can_access.replace(can_access);
    let out = body(ctx);
    ctx.can_access = saved;
    out
}

fn qual_type_path(ctx: &mut ResolveContext<'_, '_>, path: &[String]) -> Option<Vec<String>> {
    with_standard_access(ctx, |ctx| resolve_type_path(ctx, path)).ok()
}

fn qual_args(ctx: &mut ResolveContext<'_, '_>, args: &[Arg]) -> QualRes<Vec<Arg>> {
    args.iter()
        .map(|arg| {
            let value = with_standard_access(ctx, |ctx| resolve_expr(ctx, &arg.value)).map_err(|err| err.diag_id)?;
            Ok(Arg { pass: arg.pass, value, span: arg.span.clone() })
        })
        .collect()
}

fn qual_field_inits(ctx: &mut ResolveContext<'_, '_>, fields: &[FieldInit]) -> QualRes<Vec<FieldInit>> {
    fields
        .iter()
        .map(|field| {
            let value = with_standard_access(ctx, |ctx| resolve_expr(ctx, &field.value)).map_err(|err| err.diag_id)?;
            Ok(FieldInit { name: field.name.clone(), value, span: field.span.clone() })
        })
        .collect()
}

/// `path::name` as a record type.
pub fn resolve_record_path(ctx: &mut ResolveContext<'_, '_>, path: &[String], name: &str) -> Option<Vec<String>> {
    let resolved = qual_type_path(ctx, &full_path(path, name))?;
    find_record_decl(ctx.ctx, &resolved).map(|_| resolved)
}

/// `path` as an enum that has a variant `name` of the wanted payload kind.
pub fn resolve_enum_variant(
    ctx: &mut ResolveContext<'_, '_>,
    path: &[String],
    name: &str,
    kind: VariantKind,
) -> Option<Vec<String>> {
    let resolved = qual_type_path(ctx, path)?;
    let variant = find_variant(find_enum_decl(ctx.ctx, &resolved)?, name)?;
    (variant_payload_kind(variant) == kind).then_some(resolved)
}

fn enum_variant_missing(ctx: &mut ResolveContext<'_, '_>, path: &[String], name: &str) -> bool {
    let Some(resolved) = qual_type_path(ctx, path) else {
        return false;
    };
    find_enum_decl(ctx.ctx, &resolved).is_some_and(|decl| find_variant(decl, name).is_none())
}

fn path_expr(path: &[String], name: &str) -> ExprNode {
    ExprNode::PathExpr(PathExpr { path: path.to_vec(), name: name.to_string() })
}

/// Paths that are spelled qualified but name built-in operations, kept as written.
/// `Err` means the path is a built-in namespace without that member.
fn builtin_qualified(path: &[String], name: &str) -> QualRes<bool> {
    if is_string_bytes_builtin_path(path) {
        let is_string = id_eq(&path[0], "string");
        let is_bytes = id_eq(&path[0], "bytes");
        if (is_string_builtin_name(name) && is_string) || (is_bytes_builtin_name(name) && is_bytes) {
            return Ok(true);
        }
        return Err(Some("ResolveExpr-Ident-Err"));
    }
    Ok(is_runtime_panic_path(path, name)
        || (is_builtin_modal_type_path(path) && is_builtin_modal_member_name(path, name)))
}

/// `path::name` as a value; the result names the declaring module and declared name.
fn qualified_value(ctx: &mut ResolveContext<'_, '_>, path: &[String], name: &str) -> Option<ExprNode> {
    let value =
        resolve_qualified(ctx.ctx, ctx.name_maps, ctx.module_names, path, name, EntityKind::Value, ctx.can_access);
    let ent = value.entity.filter(|_| value.ok)?;
    let origin = ent.origin_opt?;
    Some(path_expr(&origin, ent.target_opt.as_deref().unwrap_or(name)))
}

fn record_path_expr(record: &[String]) -> QualRes<ExprNode> {
    match record.split_last() {
        Some((name, prefix)) if !prefix.is_empty() => Ok(path_expr(prefix, name)),
        _ => Err(None),
    }
}

fn unresolved(ctx: &mut ResolveContext<'_, '_>, path: &[String], name: &str) -> Option<&'static str> {
    if enum_variant_missing(ctx, path, name) {
        Some("E-TYP-2007")
    } else {
        Some("ResolveExpr-Ident-Err")
    }
}

pub fn resolve_qualified_form(ctx: &mut ResolveContext<'_, '_>, expr: &Expr) -> QualRes<ExprPtr> {
    let span = &expr.span;
    let call = |callee: ExprNode, args: Vec<Arg>| {
        ExprNode::CallExpr(CallExpr { callee: make_expr(span, callee), generic_args: Vec::new(), args })
    };
    match &expr.node {
        ExprNode::QualifiedNameExpr(node) => {
            let (path, name) = (&node.path[..], node.name.as_str());
            if builtin_qualified(path, name)? {
                return Ok(make_expr(span, path_expr(path, name)));
            }
            if let Some(value) = qualified_value(ctx, path, name) {
                return Ok(make_expr(span, value));
            }
            if let Some(record) = resolve_record_path(ctx, path, name) {
                return Ok(make_expr(span, record_path_expr(&record)?));
            }
            if let Some(unit) = resolve_enum_variant(ctx, path, name, VariantKind::Unit) {
                let literal = EnumLiteralExpr { path: full_path(&unit, name), payload_opt: None };
                return Ok(make_expr(span, ExprNode::EnumLiteralExpr(literal)));
            }
            Err(unresolved(ctx, path, name))
        }
        ExprNode::QualifiedApplyExpr(node) => {
            let (path, name) = (&node.path[..], node.name.as_str());
            match &node.args {
                ApplyArgs::ParenArgs(paren) => {
                    let args = qual_args(ctx, &paren.args)?;
                    if builtin_qualified(path, name)? {
                        return Ok(make_expr(span, call(path_expr(path, name), args)));
                    }
                    if let Some(value) = qualified_value(ctx, path, name) {
                        return Ok(make_expr(span, call(value, args)));
                    }
                    if let Some(record) = resolve_record_path(ctx, path, name) {
                        return Ok(make_expr(span, call(record_path_expr(&record)?, args)));
                    }
                    if let Some(tuple) = resolve_enum_variant(ctx, path, name, VariantKind::Tuple) {
                        let payload = EnumPayloadParen { elements: args.into_iter().map(|arg| arg.value).collect() };
                        let literal = EnumLiteralExpr {
                            path: full_path(&tuple, name),
                            payload_opt: Some(EnumPayload::EnumPayloadParen(payload)),
                        };
                        return Ok(make_expr(span, ExprNode::EnumLiteralExpr(literal)));
                    }
                    Err(unresolved(ctx, path, name))
                }
                ApplyArgs::BraceArgs(brace) => {
                    let fields = qual_field_inits(ctx, &brace.fields)?;
                    if let Some(record) = resolve_record_path(ctx, path, name) {
                        let rec = RecordExpr { target: RecordExprTarget::Path(record), fields };
                        return Ok(make_expr(span, ExprNode::RecordExpr(rec)));
                    }
                    if let Some(record_enum) = resolve_enum_variant(ctx, path, name, VariantKind::Record) {
                        let literal = EnumLiteralExpr {
                            path: full_path(&record_enum, name),
                            payload_opt: Some(EnumPayload::EnumPayloadBrace(EnumPayloadBrace { fields })),
                        };
                        return Ok(make_expr(span, ExprNode::EnumLiteralExpr(literal)));
                    }
                    Err(unresolved(ctx, path, name))
                }
            }
        }
        _ => Err(None),
    }
}
