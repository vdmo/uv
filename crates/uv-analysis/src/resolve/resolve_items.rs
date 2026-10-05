//! Resolving top-level items and their members.

use std::sync::Arc;

use uv_core::span::Span;
use uv_source::ast::*;

use super::resolve_contracts::*;
use super::resolve_expr::{resolve_block, resolve_expr};
use super::resolve_generics::resolve_generic_params_opt;
use super::resolve_pattern::resolve_pattern;
use super::resolve_types::{resolve_class_path, resolve_type};
use super::resolver::*;
use super::scopes::id_key_of;
use crate::context::*;

/// What a derive target's body may name.
fn build_derive_target_capability_scope() -> Scope {
    ["target", "emitter", "introspect", "diagnostics"].iter().map(|name| (id_key_of(name), value_entity())).collect()
}

fn add_to_scope(scope: &mut Scope, name: &str, entity: Entity) {
    scope.entry(id_key_of(name)).or_insert(entity);
}

fn param_scope(params: &[Param]) -> Scope {
    let mut scope = Scope::new();
    for param in params {
        add_to_scope(&mut scope, &param.name, local_entity(&param.span));
    }
    scope
}

fn add_type_params(scope: &mut Scope, generic_params: &Option<GenericParams>) {
    for type_param in generic_params.iter().flat_map(|params| &params.params) {
        add_to_scope(scope, &type_param.name, type_entity());
    }
}

/// Runs `body` in the scopes of a procedure-like declaration.
fn in_proc_scopes<'c, 'a, T>(
    ctx: &mut ResolveContext<'c, 'a>,
    proc_scope: Scope,
    body: impl FnOnce(&mut ResolveContext<'c, 'a>) -> T,
) -> T {
    let scopes = make_proc_like_scopes(ctx.ctx, proc_scope);
    with_scopes(ctx, scopes, body)
}

// Declarations report a type that fails to resolve by rule and span only.
fn decl_type(ctx: &mut ResolveContext<'_, '_>, ty: &TypePtr) -> Res<TypePtr> {
    resolve_type(ctx, ty).map_err(ResError::id_span)
}

fn decl_type_opt(ctx: &mut ResolveContext<'_, '_>, ty: &TypePtr) -> Res<TypePtr> {
    match ty {
        Some(_) => decl_type(ctx, ty),
        None => Ok(None),
    }
}

fn resolve_expr_opt(ctx: &mut ResolveContext<'_, '_>, expr_opt: &ExprPtr) -> Res<ExprPtr> {
    match expr_opt {
        Some(_) => resolve_expr(ctx, expr_opt),
        None => Ok(None),
    }
}

fn resolve_body(ctx: &mut ResolveContext<'_, '_>, body: &BlockPtr) -> Res<BlockPtr> {
    match body {
        Some(block) => Ok(Some(Arc::new(resolve_block(ctx, block)?))),
        None => Ok(None),
    }
}

fn resolve_receiver(ctx: &mut ResolveContext<'_, '_>, recv: &Receiver) -> Res<Receiver> {
    match recv {
        Receiver::ReceiverShorthand(_) => Ok(recv.clone()),
        Receiver::ReceiverExplicit(node) => {
            Ok(Receiver::ReceiverExplicit(ReceiverExplicit { mode_opt: node.mode_opt, r#type: decl_type(ctx, &node.r#type)? }))
        }
    }
}

fn resolve_params(ctx: &mut ResolveContext<'_, '_>, params: &[Param]) -> Res<Vec<Param>> {
    params.iter().map(|param| Ok(Param { r#type: decl_type(ctx, &param.r#type)?, ..param.clone() })).collect()
}

fn resolve_class_path_list(ctx: &mut ResolveContext<'_, '_>, paths: &[Vec<String>]) -> Res<Vec<Vec<String>>> {
    paths.iter().map(|path| resolve_class_path(ctx, path).map_err(ResError::id_span)).collect()
}

fn resolve_field_decl(ctx: &mut ResolveContext<'_, '_>, field: &FieldDecl) -> Res<FieldDecl> {
    let r#type = decl_type(ctx, &field.r#type)?;
    let init_opt = resolve_expr_opt(ctx, &field.init_opt)?;
    Ok(FieldDecl { r#type, init_opt, ..field.clone() })
}

fn resolve_variant_list(ctx: &mut ResolveContext<'_, '_>, vars: &[VariantDecl]) -> Res<Vec<VariantDecl>> {
    vars.iter()
        .map(|var| {
            let payload_opt = match &var.payload_opt {
                None => None,
                Some(VariantPayload::VariantPayloadTuple(payload)) => {
                    let elements = payload.elements.iter().map(|ty| decl_type(ctx, ty)).collect::<Res<Vec<_>>>()?;
                    Some(VariantPayload::VariantPayloadTuple(VariantPayloadTuple { elements }))
                }
                Some(VariantPayload::VariantPayloadRecord(payload)) => {
                    let fields = payload
                        .fields
                        .iter()
                        .map(|field| resolve_field_decl(ctx, field).map_err(ResError::id_span))
                        .collect::<Res<Vec<_>>>()?;
                    Some(VariantPayload::VariantPayloadRecord(VariantPayloadRecord { fields }))
                }
            };
            Ok(VariantDecl { payload_opt, ..var.clone() })
        })
        .collect()
}

fn resolve_associated_type(ctx: &mut ResolveContext<'_, '_>, node: &AssociatedTypeDecl) -> Res<AssociatedTypeDecl> {
    Ok(AssociatedTypeDecl { default_type: decl_type_opt(ctx, &node.default_type)?, ..node.clone() })
}

fn self_entity(span: &Span) -> Entity {
    local_entity(span)
}

/// Record methods see `Self` as the record. Their own type parameters are not bound here.
fn resolve_record_member(ctx: &mut ResolveContext<'_, '_>, record: &RecordDecl, member: &RecordMember) -> Res<RecordMember> {
    Ok(match member {
        RecordMember::FieldDecl(node) => RecordMember::FieldDecl(resolve_field_decl(ctx, node)?),
        RecordMember::MethodDecl(node) => {
            let mut proc_scope = param_scope(&node.params);
            add_to_scope(&mut proc_scope, "self", self_entity(&node.span));
            let self_type = Entity::new(
                EntityKind::Type,
                Some(ctx.ctx.current_module.clone()),
                Some(record.name.clone()),
                EntitySource::Decl,
            );
            add_to_scope(&mut proc_scope, "Self", self_type);
            RecordMember::MethodDecl(in_proc_scopes(ctx, proc_scope, |ctx| {
                let receiver = resolve_receiver(ctx, &node.receiver)?;
                let params = resolve_params(ctx, &node.params)?;
                let return_type_opt = decl_type_opt(ctx, &node.return_type_opt)?;
                let contract = resolve_contract_opt(ctx, &node.contract)?;
                let body = resolve_body(ctx, &node.body)?;
                Ok(MethodDecl { receiver, params, return_type_opt, contract, body, ..node.clone() })
            })?)
        }
        RecordMember::AssociatedTypeDecl(node) => RecordMember::AssociatedTypeDecl(resolve_associated_type(ctx, node)?),
    })
}

fn resolve_class_item(ctx: &mut ResolveContext<'_, '_>, item: &ClassItem) -> Res<ClassItem> {
    Ok(match item {
        ClassItem::ClassFieldDecl(node) => {
            ClassItem::ClassFieldDecl(ClassFieldDecl { r#type: decl_type(ctx, &node.r#type)?, ..node.clone() })
        }
        ClassItem::AssociatedTypeDecl(node) => ClassItem::AssociatedTypeDecl(resolve_associated_type(ctx, node)?),
        ClassItem::AbstractFieldDecl(node) => {
            ClassItem::AbstractFieldDecl(AbstractFieldDecl { r#type: decl_type(ctx, &node.r#type)?, ..node.clone() })
        }
        ClassItem::AbstractStateDecl(node) => {
            let fields = node
                .fields
                .iter()
                .map(|field| Ok(AbstractFieldDecl { r#type: decl_type(ctx, &field.r#type)?, ..field.clone() }))
                .collect::<Res<Vec<_>>>()?;
            ClassItem::AbstractStateDecl(AbstractStateDecl { fields, ..node.clone() })
        }
        ClassItem::ClassMethodDecl(node) => {
            let mut proc_scope = Scope::new();
            add_type_params(&mut proc_scope, &node.generic_params);
            for param in &node.params {
                add_to_scope(&mut proc_scope, &param.name, local_entity(&param.span));
            }
            add_to_scope(&mut proc_scope, "self", self_entity(&node.span));
            add_to_scope(&mut proc_scope, "Self", type_entity());
            ClassItem::ClassMethodDecl(in_proc_scopes(ctx, proc_scope, |ctx| {
                let receiver = resolve_receiver(ctx, &node.receiver)?;
                let params = resolve_params(ctx, &node.params)?;
                let return_type_opt = decl_type_opt(ctx, &node.return_type_opt)?;
                let contract = resolve_contract_opt(ctx, &node.contract)?;
                let body_opt = resolve_body(ctx, &node.body_opt)?;
                Ok(ClassMethodDecl { receiver, params, return_type_opt, contract, body_opt, ..node.clone() })
            })?)
        }
    })
}

fn resolve_state_member(ctx: &mut ResolveContext<'_, '_>, member: &StateMember) -> Res<StateMember> {
    Ok(match member {
        StateMember::StateFieldDecl(node) => {
            StateMember::StateFieldDecl(StateFieldDecl { r#type: decl_type(ctx, &node.r#type)?, ..node.clone() })
        }
        StateMember::StateMethodDecl(node) => {
            let mut proc_scope = param_scope(&node.params);
            add_to_scope(&mut proc_scope, "self", self_entity(&node.span));
            StateMember::StateMethodDecl(in_proc_scopes(ctx, proc_scope, |ctx| {
                let params = resolve_params(ctx, &node.params)?;
                let return_type_opt = decl_type_opt(ctx, &node.return_type_opt)?;
                let contract = resolve_contract_opt(ctx, &node.contract)?;
                let body = resolve_body(ctx, &node.body)?;
                Ok(StateMethodDecl { params, return_type_opt, contract, body, ..node.clone() })
            })?)
        }
        StateMember::TransitionDecl(node) => {
            let mut proc_scope = param_scope(&node.params);
            add_to_scope(&mut proc_scope, "self", self_entity(&node.span));
            StateMember::TransitionDecl(in_proc_scopes(ctx, proc_scope, |ctx| {
                let params = resolve_params(ctx, &node.params)?;
                let body = resolve_body(ctx, &node.body)?;
                Ok(TransitionDecl { params, body, ..node.clone() })
            })?)
        }
    })
}

fn resolve_extern_proc(ctx: &mut ResolveContext<'_, '_>, ext: &ExternProcDecl) -> Res<ExternProcDecl> {
    in_proc_scopes(ctx, param_scope(&ext.params), |ctx| {
        let generic_params = resolve_generic_params_opt(ctx, &ext.generic_params)?;
        let params = resolve_params(ctx, &ext.params)?;
        let return_type_opt = decl_type_opt(ctx, &ext.return_type_opt)?;
        let contract = resolve_contract_opt(ctx, &ext.contract)?;
        let foreign_contracts_opt = resolve_foreign_contracts_opt(ctx, &ext.foreign_contracts_opt)?;
        Ok(ExternProcDecl { generic_params, params, return_type_opt, contract, foreign_contracts_opt, ..ext.clone() })
    })
    .map_err(ResError::id_span)
}

pub fn resolve_item(ctx: &mut ResolveContext<'_, '_>, item: &ASTItem) -> Res<ASTItem> {
    Ok(match item {
        ASTItem::StaticDecl(node) => {
            let mut binding = node.binding.clone();
            if node.binding.pat.is_some() {
                binding.pat = resolve_pattern(ctx, &node.binding.pat).map_err(ResError::id_span)?;
            }
            binding.type_opt = decl_type_opt(ctx, &node.binding.type_opt)?;
            binding.init = resolve_expr(ctx, &node.binding.init)?;
            ASTItem::StaticDecl(StaticDecl { binding, ..node.clone() })
        }
        ASTItem::ProcedureDecl(node) => ASTItem::ProcedureDecl(in_proc_scopes(ctx, param_scope(&node.params), |ctx| {
            let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
            let params = resolve_params(ctx, &node.params)?;
            // Unlike other declarations, a procedure keeps the detail of a return type
            // that fails to resolve.
            let return_type_opt = match &node.return_type_opt {
                Some(_) => resolve_type(ctx, &node.return_type_opt)?,
                None => None,
            };
            let contract = resolve_contract_opt(ctx, &node.contract)?;
            let body = resolve_body(ctx, &node.body)?;
            Ok(ProcedureDecl { generic_params, params, return_type_opt, contract, body, ..node.clone() })
        })?),
        ASTItem::ComptimeProcedureDecl(node) => {
            ASTItem::ComptimeProcedureDecl(in_proc_scopes(ctx, param_scope(&node.params), |ctx| {
                let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
                let params = resolve_params(ctx, &node.params)?;
                let return_type_opt = decl_type_opt(ctx, &node.return_type_opt)?;
                let contract = resolve_contract_opt(ctx, &node.contract)?;
                let body = resolve_body(ctx, &node.body)?;
                Ok(ComptimeProcedureDecl { generic_params, params, return_type_opt, contract, body, ..node.clone() })
            })?)
        }
        ASTItem::DeriveTargetDecl(node) => {
            ASTItem::DeriveTargetDecl(in_proc_scopes(ctx, build_derive_target_capability_scope(), |ctx| {
                Ok(DeriveTargetDecl { body: resolve_body(ctx, &node.body)?, ..node.clone() })
            })?)
        }
        ASTItem::TypeAliasDecl(node) => ASTItem::TypeAliasDecl(in_proc_scopes(ctx, Scope::new(), |ctx| {
            let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
            let r#type = decl_type(ctx, &node.r#type)?;
            Ok(TypeAliasDecl { generic_params, r#type, ..node.clone() })
        })?),
        ASTItem::RecordDecl(node) => ASTItem::RecordDecl(in_proc_scopes(ctx, Scope::new(), |ctx| {
            let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
            let implements = resolve_class_path_list(ctx, &node.implements)?;
            let invariant_opt = resolve_type_invariant_opt(ctx, &node.invariant_opt).map_err(ResError::id_span)?;
            let members =
                node.members.iter().map(|member| resolve_record_member(ctx, node, member)).collect::<Res<Vec<_>>>()?;
            Ok(RecordDecl { generic_params, implements, invariant_opt, members, ..node.clone() })
        })?),
        ASTItem::EnumDecl(node) => ASTItem::EnumDecl(in_proc_scopes(ctx, Scope::new(), |ctx| {
            let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
            let implements = resolve_class_path_list(ctx, &node.implements)?;
            let invariant_opt = resolve_type_invariant_opt(ctx, &node.invariant_opt).map_err(ResError::id_span)?;
            let variants = resolve_variant_list(ctx, &node.variants).map_err(ResError::id_span)?;
            Ok(EnumDecl { generic_params, implements, invariant_opt, variants, ..node.clone() })
        })?),
        ASTItem::ModalDecl(node) => ASTItem::ModalDecl(in_proc_scopes(ctx, Scope::new(), |ctx| {
            let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
            let implements = resolve_class_path_list(ctx, &node.implements)?;
            let invariant_opt = resolve_type_invariant_opt(ctx, &node.invariant_opt).map_err(ResError::id_span)?;
            let mut states = Vec::with_capacity(node.states.len());
            for state in &node.states {
                let members =
                    state.members.iter().map(|member| resolve_state_member(ctx, member)).collect::<Res<Vec<_>>>()?;
                states.push(StateBlock { members, ..state.clone() });
            }
            Ok(ModalDecl { generic_params, implements, invariant_opt, states, ..node.clone() })
        })?),
        ASTItem::ClassDecl(node) => ASTItem::ClassDecl(in_proc_scopes(ctx, Scope::new(), |ctx| {
            let generic_params = resolve_generic_params_opt(ctx, &node.generic_params)?;
            let supers = resolve_class_path_list(ctx, &node.supers)?;
            let items = node.items.iter().map(|item| resolve_class_item(ctx, item)).collect::<Res<Vec<_>>>()?;
            Ok(ClassDecl { generic_params, supers, items, ..node.clone() })
        })?),
        ASTItem::ExternBlock(node) => {
            let items = node
                .items
                .iter()
                .map(|ExternItem::ExternProcDecl(ext)| Ok(ExternItem::ExternProcDecl(resolve_extern_proc(ctx, ext)?)))
                .collect::<Res<Vec<_>>>()?;
            ASTItem::ExternBlock(ExternBlock { items, ..node.clone() })
        }
        ASTItem::UsingDecl(_) | ASTItem::ImportDecl(_) | ASTItem::ErrorItem(_) => item.clone(),
    })
}
