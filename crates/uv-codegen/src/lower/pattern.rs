//! Lowering: patterns. `LowerIRPattern` makes the pattern a node of the IR, and
//! `RegisterBindingsFromPattern` registers the names it binds.

use super::*;
use uv_analysis::context::TypeDecl;
use uv_analysis::generics::monomorphize::{build_substitution, instantiate_type};
use uv_analysis::layout::{enum_record_payload_member_layout, enum_tuple_payload_member_layout, union_layout_of};

fn ir_literal_kind(kind: TokenKind) -> IrLiteralKind {
    match kind {
        TokenKind::IntLiteral => IrLiteralKind::Int,
        TokenKind::FloatLiteral => IrLiteralKind::Float,
        TokenKind::StringLiteral => IrLiteralKind::String,
        TokenKind::CharLiteral => IrLiteralKind::Char,
        TokenKind::BoolLiteral => IrLiteralKind::Bool,
        TokenKind::NullLiteral => IrLiteralKind::Null,
        _ => IrLiteralKind::Unknown,
    }
}

/// `ToIRRangeKind`.
pub(super) fn ir_range_kind(kind: ast::RangeKind) -> IrRangeKind {
    match kind {
        ast::RangeKind::To => IrRangeKind::To,
        ast::RangeKind::ToInclusive => IrRangeKind::ToInclusive,
        ast::RangeKind::Full => IrRangeKind::Full,
        ast::RangeKind::From => IrRangeKind::From,
        ast::RangeKind::Exclusive => IrRangeKind::Exclusive,
        ast::RangeKind::Inclusive => IrRangeKind::Inclusive,
    }
}

fn lower_ir_pattern_ptr(pattern: &Option<Arc<ast::Pattern>>, ctx: &LowerCtx) -> IrPatternPtr {
    pattern.as_ref().map(|pattern| lower_ir_pattern(pattern, ctx))
}

fn lower_ir_field_patterns(fields: &[ast::FieldPattern], ctx: &LowerCtx) -> Vec<IrFieldPattern> {
    fields.iter().map(|field| IrFieldPattern { name: field.name.clone(), pattern: lower_ir_pattern_ptr(&field.pattern_opt, ctx) }).collect()
}

/// `StableBindingName` of a name bound by a pattern; the name itself when it is not bound.
fn stable_or_plain(name: &str, ctx: &LowerCtx) -> String {
    ctx.binding_state(name).map(|state| state.stable_name.clone()).filter(|stable| !stable.is_empty()).unwrap_or_else(|| name.to_string())
}

/// `LowerIRPattern`.
pub(super) fn lower_ir_pattern(pattern: &ast::Pattern, ctx: &LowerCtx) -> Arc<IrPattern> {
    let node = match &pattern.node {
        ast::PatternNode::LiteralPattern(node) => IrPatternNode::Literal(IrLiteral { kind: ir_literal_kind(node.literal.kind), lexeme: node.literal.lexeme.clone() }),
        ast::PatternNode::WildcardPattern(_) => IrPatternNode::Wildcard,
        ast::PatternNode::IdentifierPattern(node) => IrPatternNode::Identifier { name: stable_or_plain(&node.name, ctx) },
        ast::PatternNode::TypedPattern(node) => {
            let ty = node.r#type.as_ref().and_then(|written| lower_type_for_layout(&ctx.scope, &Some(written.clone())).flatten());
            IrPatternNode::Typed { name: stable_or_plain(&node.name, ctx), ty }
        }
        ast::PatternNode::TuplePattern(node) => IrPatternNode::Tuple { elements: node.elements.iter().map(|element| lower_ir_pattern_ptr(element, ctx)).collect() },
        ast::PatternNode::RecordPattern(node) => IrPatternNode::Record { path: node.path.clone(), fields: lower_ir_field_patterns(&node.fields, ctx) },
        ast::PatternNode::EnumPattern(node) => {
            let payload = node.payload_opt.as_ref().map(|payload| match payload {
                ast::EnumPayloadPattern::TuplePayloadPattern(tuple) => IrEnumPayloadPattern::Tuple(tuple.elements.iter().map(|element| lower_ir_pattern_ptr(element, ctx)).collect()),
                ast::EnumPayloadPattern::RecordPayloadPattern(record) => IrEnumPayloadPattern::Record(lower_ir_field_patterns(&record.fields, ctx)),
            });
            IrPatternNode::Enum { path: node.path.clone(), name: node.name.clone(), payload }
        }
        ast::PatternNode::ModalPattern(node) => IrPatternNode::Modal { state: node.state.clone(), fields: node.fields_opt.as_ref().map(|fields| lower_ir_field_patterns(&fields.fields, ctx)) },
        ast::PatternNode::RangePattern(node) => IrPatternNode::Range { kind: ir_range_kind(node.kind), lo: lower_ir_pattern_ptr(&node.lo, ctx), hi: lower_ir_pattern_ptr(&node.hi, ctx) },
        ast::PatternNode::SpliceExprNode(_) => IrPatternNode::Opaque,
    };
    Arc::new(IrPattern { node })
}

fn binding_type(written: &Option<Arc<ast::Type>>, ctx: &LowerCtx) -> TypeRef {
    written.as_ref().and_then(|written| lower_type_for_layout(&ctx.scope, &Some(written.clone())).flatten())
}

fn strip_perm_and_refine(ty: &TypeRef) -> TypeRef {
    let mut stripped = ty.clone();
    while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = stripped.as_deref().map(|ty| &ty.node) {
        stripped = base.clone();
    }
    stripped
}

fn generic_args_from_hint(hint: &TypeRef) -> Vec<TypeRef> {
    strip_perm_and_refine(hint).as_ref().and_then(|ty| applied_type_args(ty).map(<[TypeRef]>::to_vec)).unwrap_or_default()
}

fn modal_field_type(decl: &ast::ModalDecl, modal_args: &[TypeRef], state_name: &str, field_name: &str, ctx: &LowerCtx) -> TypeRef {
    let mut subst = Default::default();
    if let Some(params) = decl.generic_params.as_ref().filter(|params| !params.params.is_empty()) {
        if modal_args.len() > params.params.len() {
            return None;
        }
        subst = build_substitution(&params.params, modal_args);
    }
    let state = decl.states.iter().find(|state| id_eq(&state.name, state_name))?;
    for member in &state.members {
        if let ast::StateMember::StateFieldDecl(field) = member {
            if id_eq(&field.name, field_name) {
                let ty = binding_type(&field.r#type, ctx);
                return if ty.is_some() && !subst.is_empty() { instantiate_type(&ty, &subst) } else { ty };
            }
        }
    }
    None
}

/// What a binding registered by a pattern is registered with.
pub(super) struct BindingKind {
    pub is_immovable: bool,
    pub prov: ProvenanceKind,
    pub prov_region: Option<String>,
    pub prov_region_tag: Option<String>,
    pub has_responsibility: bool,
}

fn register_pattern_var(name: &str, ty: TypeRef, kind: &BindingKind, ctx: &mut LowerCtx) {
    ctx.register_var(name, ty, kind.has_responsibility, kind.prov, kind.prov_region.clone(), kind.prov_region_tag.clone());
    if kind.is_immovable {
        ctx.mark_last_binding_immovable(name);
    }
}

fn walk_pattern(pattern: &ast::Pattern, hint: TypeRef, kind: &BindingKind, ctx: &mut LowerCtx) {
    match &pattern.node {
        ast::PatternNode::WildcardPattern(_) | ast::PatternNode::LiteralPattern(_) | ast::PatternNode::SpliceExprNode(_) => {}
        ast::PatternNode::IdentifierPattern(node) => register_pattern_var(&node.name, hint, kind, ctx),
        ast::PatternNode::TypedPattern(node) => {
            if node.name == "_" {
                return;
            }
            let typed = binding_type(&node.r#type, ctx).or(hint);
            register_pattern_var(&node.name, typed, kind, ctx);
        }
        ast::PatternNode::TuplePattern(node) => {
            let elements = match hint.as_deref().map(|ty| &ty.node) {
                Some(TypeNode::Tuple(elements)) => elements.clone(),
                _ => Vec::new(),
            };
            for (index, element) in node.elements.iter().enumerate() {
                if let Some(element) = element {
                    walk_pattern(element, elements.get(index).cloned().flatten_type(), kind, ctx);
                }
            }
        }
        ast::PatternNode::RecordPattern(node) => {
            let record = match ctx.scope.sigma.clone().types.get(&path_key_of(&node.path)) {
                Some(TypeDecl::Record(record)) => Some(record.clone()),
                _ => None,
            };
            for field in &node.fields {
                let field_ty = record.as_ref().and_then(|record| {
                    record.members.iter().find_map(|member| match member {
                        ast::RecordMember::FieldDecl(decl) if id_eq(&decl.name, &field.name) => Some(binding_type(&decl.r#type, ctx)),
                        _ => None,
                    })
                });
                match &field.pattern_opt {
                    Some(inner) => walk_pattern(inner, field_ty.flatten(), kind, ctx),
                    None => register_pattern_var(&field.name, field_ty.flatten(), kind, ctx),
                }
            }
        }
        ast::PatternNode::EnumPattern(node) => {
            let Some(payload) = &node.payload_opt else {
                return;
            };
            let sigma = ctx.scope.sigma.clone();
            let enum_decl = match sigma.types.get(&path_key_of(&node.path)) {
                Some(TypeDecl::Enum(decl)) => Some(decl),
                _ => None,
            };
            let variant = enum_decl.and_then(|decl| decl.variants.iter().find(|variant| id_eq(&variant.name, &node.name)));
            let generic_args = generic_args_from_hint(&hint);
            match payload {
                ast::EnumPayloadPattern::TuplePayloadPattern(tuple) => {
                    for (index, element) in tuple.elements.iter().enumerate() {
                        let Some(element) = element else { continue };
                        let element_ty = enum_decl.zip(variant).and_then(|(decl, variant)| enum_tuple_payload_member_layout(&ctx.scope, decl, variant, &generic_args, index)).and_then(|member| member.r#type);
                        walk_pattern(element, element_ty, kind, ctx);
                    }
                }
                ast::EnumPayloadPattern::RecordPayloadPattern(record) => {
                    for field in &record.fields {
                        let field_ty = enum_decl.zip(variant).and_then(|(decl, variant)| enum_record_payload_member_layout(&ctx.scope, decl, variant, &generic_args, &field.name)).and_then(|member| member.r#type);
                        match &field.pattern_opt {
                            Some(inner) => walk_pattern(inner, field_ty, kind, ctx),
                            None => register_pattern_var(&field.name, field_ty, kind, ctx),
                        }
                    }
                }
            }
        }
        ast::PatternNode::ModalPattern(node) => {
            // The modal the pattern looks into: named by the hint, or by the one member of a
            // union hint that is that state.
            let modal_of = |ty: &TypeRef, require_state_match: bool| -> Option<(Vec<String>, Vec<TypeRef>)> {
                let stripped = strip_perm_and_refine(ty)?;
                if let TypeNode::ModalState(modal) = &stripped.node {
                    if require_state_match && !id_eq(&modal.state, &node.state) {
                        return None;
                    }
                    return Some((modal.path.clone(), modal.generic_args.clone()));
                }
                if !require_state_match {
                    let path = applied_type_path(&stripped)?;
                    return Some((path.to_vec(), applied_type_args(&stripped).map(<[TypeRef]>::to_vec).unwrap_or_default()));
                }
                None
            };
            let modal_hint = strip_perm_and_refine(&hint);
            if matches!(modal_hint.as_deref().map(|ty| &ty.node), Some(TypeNode::Path { .. })) && applied_type_path(modal_hint.as_ref().unwrap()).is_some_and(|path| matches!(ctx.scope.sigma.types.get(&path_key_of(path)), Some(TypeDecl::TypeAlias(_)))) {
                ctx.unported("modal patterns over aliased types");
                return;
            }
            let mut found = modal_of(&modal_hint, false);
            if found.is_none() {
                if let Some(TypeNode::Union(members)) = modal_hint.as_deref().map(|ty| &ty.node) {
                    let members = union_layout_of(&ctx.scope, members).map(|layout| layout.member_list).unwrap_or_else(|| members.clone());
                    let mut matched: Option<Arc<uv_analysis::typing::types::Type>> = None;
                    let mut ambiguous = false;
                    for member in &members {
                        let stripped = strip_perm_and_refine(member);
                        if let Some(TypeNode::ModalState(modal)) = stripped.as_deref().map(|ty| &ty.node) {
                            if id_eq(&modal.state, &node.state) {
                                if matched.is_some() {
                                    ambiguous = true;
                                    break;
                                }
                                matched = stripped.clone();
                            }
                        }
                    }
                    if let (Some(matched), false) = (matched, ambiguous) {
                        found = modal_of(&Some(matched), true);
                    }
                }
            }
            let (modal_path, modal_args) = found.unwrap_or_default();
            let sigma = ctx.scope.sigma.clone();
            let modal_decl = if modal_path.is_empty() {
                None
            } else {
                match sigma.types.get(&path_key_of(&modal_path)) {
                    Some(TypeDecl::Modal(decl)) => Some(decl),
                    _ => None,
                }
            };
            let Some(fields) = &node.fields_opt else {
                return;
            };
            for field in &fields.fields {
                let field_ty = modal_decl.and_then(|decl| modal_field_type(decl, &modal_args, &node.state, &field.name, ctx));
                match &field.pattern_opt {
                    Some(inner) => walk_pattern(inner, field_ty, kind, ctx),
                    None => register_pattern_var(&field.name, field_ty, kind, ctx),
                }
            }
        }
        ast::PatternNode::RangePattern(node) => {
            if let Some(lo) = &node.lo {
                walk_pattern(lo, hint.clone(), kind, ctx);
            }
            if let Some(hi) = &node.hi {
                walk_pattern(hi, hint, kind, ctx);
            }
        }
    }
}

trait FlattenType {
    fn flatten_type(self) -> TypeRef;
}

impl FlattenType for Option<TypeRef> {
    fn flatten_type(self) -> TypeRef {
        self.flatten()
    }
}

/// `RegisterPatternBindings`: the names a pattern binds, each with the type its place in
/// the hint gives it.
pub(super) fn register_pattern_bindings(pattern: &ast::Pattern, hint: &TypeRef, kind: &BindingKind, ctx: &mut LowerCtx) {
    walk_pattern(pattern, hint.clone(), kind, ctx);
}
