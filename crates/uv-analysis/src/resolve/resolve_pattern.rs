//! Resolving the type paths patterns mention.

use std::sync::Arc;

use uv_source::ast::*;

use super::resolve_types::{resolve_type, resolve_type_path};
use super::resolver::*;

fn has_record_payload_variant(decl: &EnumDecl, variant_name: &str) -> bool {
    decl.variants
        .iter()
        .find(|variant| variant.name == variant_name && variant.payload_opt.is_some())
        .is_some_and(|variant| matches!(variant.payload_opt, Some(VariantPayload::VariantPayloadRecord(_))))
}

fn pattern_list(ctx: &mut ResolveContext<'_, '_>, patterns: &[PatternPtr]) -> Res<Vec<PatternPtr>> {
    patterns.iter().map(|pattern| resolve_pattern(ctx, pattern).map_err(ResError::id_span)).collect()
}

fn field_pattern_list(ctx: &mut ResolveContext<'_, '_>, fields: &[FieldPattern]) -> Res<Vec<FieldPattern>> {
    fields
        .iter()
        .map(|field| {
            let pattern_opt = match &field.pattern_opt {
                Some(_) => resolve_pattern(ctx, &field.pattern_opt).map_err(ResError::id_span)?,
                None => None,
            };
            Ok(FieldPattern { name: field.name.clone(), pattern_opt, span: field.span.clone() })
        })
        .collect()
}

fn rebuilt(pattern: &Pattern, node: PatternNode) -> Res<PatternPtr> {
    Ok(Some(Arc::new(Pattern { span: pattern.span.clone(), node })))
}

fn record_payload_fields(node: &EnumPattern) -> &[FieldPattern] {
    match &node.payload_opt {
        Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => &payload.fields,
        _ => &[],
    }
}

/// `a::B { .. }` parses as an enum pattern with a record payload; when `a::B` turns out
/// to be a record it becomes a record pattern.
fn resolve_enum_pattern(ctx: &mut ResolveContext<'_, '_>, pattern: &Pattern, node: &EnumPattern) -> Res<PatternPtr> {
    let record_payload = matches!(node.payload_opt, Some(EnumPayloadPattern::RecordPayloadPattern(_)));
    let joined = full_path(&node.path, &node.name);
    let as_record = |ctx: &mut ResolveContext<'_, '_>, path: Vec<String>| {
        let fields = field_pattern_list(ctx, record_payload_fields(node))?;
        rebuilt(pattern, PatternNode::RecordPattern(RecordPattern { path, fields }))
    };
    let resolved_path = match resolve_type_path(ctx, &node.path) {
        Ok(path) => path,
        Err(err) => {
            if record_payload {
                if let Ok(record_path) = resolve_type_path(ctx, &joined) {
                    if find_record_decl(ctx.ctx, &record_path).is_some() {
                        return as_record(ctx, record_path);
                    }
                }
            }
            return Err(err);
        }
    };
    let enum_decl = find_enum_decl(ctx.ctx, &resolved_path);
    let is_enum = enum_decl.is_some();
    if record_payload && enum_decl.is_some_and(|decl| has_record_payload_variant(decl, &node.name)) {
        if let Ok(record_path) = resolve_type_path(ctx, &joined) {
            if find_record_decl(ctx.ctx, &record_path).is_some() {
                return Err(ResError::at("E-MOD-1307", &pattern.span));
            }
        }
    }
    if !is_enum && record_payload {
        let record_path = resolve_type_path(ctx, &joined)?;
        if find_record_decl(ctx.ctx, &record_path).is_none() {
            return Err(ResError::at("E-TYP-1501", &pattern.span));
        }
        return as_record(ctx, record_path);
    }
    let payload_opt = match &node.payload_opt {
        None => None,
        Some(EnumPayloadPattern::TuplePayloadPattern(payload)) => {
            let elements = pattern_list(ctx, &payload.elements)?;
            Some(EnumPayloadPattern::TuplePayloadPattern(TuplePayloadPattern { elements }))
        }
        Some(EnumPayloadPattern::RecordPayloadPattern(payload)) => {
            let fields = field_pattern_list(ctx, &payload.fields)?;
            Some(EnumPayloadPattern::RecordPayloadPattern(RecordPayloadPattern { fields }))
        }
    };
    rebuilt(pattern, PatternNode::EnumPattern(EnumPattern { path: resolved_path, name: node.name.clone(), payload_opt }))
}

pub fn resolve_pattern(ctx: &mut ResolveContext<'_, '_>, pattern_ptr: &PatternPtr) -> Res<PatternPtr> {
    let Some(pattern) = pattern_ptr.as_deref() else {
        return Err(ResError::default());
    };
    match &pattern.node {
        PatternNode::TypedPattern(node) => {
            let r#type = resolve_type(ctx, &node.r#type)?;
            rebuilt(pattern, PatternNode::TypedPattern(TypedPattern { r#type, ..node.clone() }))
        }
        PatternNode::TuplePattern(node) => {
            let elements = pattern_list(ctx, &node.elements)?;
            rebuilt(pattern, PatternNode::TuplePattern(TuplePattern { elements }))
        }
        PatternNode::RecordPattern(node) => {
            let path = resolve_type_path(ctx, &node.path).map_err(|err| err.span_or(&pattern.span))?;
            let fields = field_pattern_list(ctx, &node.fields)?;
            rebuilt(pattern, PatternNode::RecordPattern(RecordPattern { path, fields }))
        }
        PatternNode::EnumPattern(node) => resolve_enum_pattern(ctx, pattern, node),
        PatternNode::ModalPattern(node) => {
            let fields_opt = match &node.fields_opt {
                Some(payload) => Some(ModalRecordPayload { fields: field_pattern_list(ctx, &payload.fields)? }),
                None => None,
            };
            rebuilt(pattern, PatternNode::ModalPattern(ModalPattern { state: node.state.clone(), fields_opt }))
        }
        PatternNode::RangePattern(node) => {
            let lo = resolve_pattern(ctx, &node.lo).map_err(ResError::id_span)?;
            let hi = resolve_pattern(ctx, &node.hi).map_err(ResError::id_span)?;
            rebuilt(pattern, PatternNode::RangePattern(RangePattern { kind: node.kind, lo, hi }))
        }
        PatternNode::WildcardPattern(_)
        | PatternNode::IdentifierPattern(_)
        | PatternNode::LiteralPattern(_)
        | PatternNode::SpliceExprNode(_) => Ok(pattern_ptr.clone()),
    }
}
