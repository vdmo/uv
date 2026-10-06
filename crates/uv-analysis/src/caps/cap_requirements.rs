//! Which capabilities a type carries: a capability class behind `$`, the context
//! bundle, or a nominal type with such a thing in its fields.
//!
//! The reference computes the set of capability kinds; so far only whether the set is
//! empty is asked, so that is what is computed.

use std::collections::HashSet;

use uv_source::ast;

use super::builtin_paths::{is_capability_class_path, is_context_type_path};
use crate::context::{ScopeContext, TypeDecl};
use crate::resolve::scopes::{id_eq, path_key_of};
use crate::typing::types::{TypeNode, TypeRef};

/// The declaration a type path names: as written, under the current module, under the
/// current assembly, or else the only declaration the path is a suffix of.
fn lookup_nominal_type_decl<'c>(ctx: &'c ScopeContext<'_>, current_module: &[String], path: &[String]) -> Option<(Vec<String>, &'c TypeDecl)> {
    if path.is_empty() {
        return None;
    }
    let lookup = |candidate: Vec<String>| ctx.sigma.types.get(&path_key_of(&candidate)).map(|decl| (candidate, decl));
    if let Some(found) = lookup(path.to_vec()) {
        return Some(found);
    }
    if let Some(found) = lookup(current_module.iter().chain(path).cloned().collect()) {
        return Some(found);
    }
    if let ([only], Some(root)) = (path, current_module.first()) {
        if let Some(found) = lookup(vec![root.clone(), only.clone()]) {
            return Some(found);
        }
    }
    let mut unique_match = None;
    for (key, decl) in &ctx.sigma.types {
        if key.len() < path.len() || !key[key.len() - path.len()..].iter().zip(path).all(|(lhs, rhs)| id_eq(lhs, rhs)) {
            continue;
        }
        if unique_match.is_some() {
            return None;
        }
        unique_match = Some((key.to_vec(), decl));
    }
    unique_match
}

struct Walk<'c, 'x> {
    ctx: &'c ScopeContext<'x>,
    visiting: HashSet<String>,
}

impl Walk<'_, '_> {
    fn nominal(&mut self, current_module: &[String], path: &[String]) -> bool {
        let Some((resolved, decl)) = lookup_nominal_type_decl(self.ctx, current_module, path) else {
            return false;
        };
        let visit_key = resolved.join("::");
        if !self.visiting.insert(visit_key.clone()) {
            return false;
        }
        let decl_module = &resolved[..resolved.len() - 1];
        let found = match decl {
            TypeDecl::Record(node) => node.members.iter().any(|member| match member {
                ast::RecordMember::FieldDecl(field) => self.ast_type(decl_module, &field.r#type),
                _ => false,
            }),
            TypeDecl::Enum(node) => node.variants.iter().any(|variant| match &variant.payload_opt {
                Some(ast::VariantPayload::VariantPayloadTuple(payload)) => payload.elements.iter().any(|elem| self.ast_type(decl_module, elem)),
                Some(ast::VariantPayload::VariantPayloadRecord(payload)) => payload.fields.iter().any(|field| self.ast_type(decl_module, &field.r#type)),
                None => false,
            }),
            TypeDecl::Modal(node) => node.states.iter().flat_map(|state| &state.members).any(|member| match member {
                ast::StateMember::StateFieldDecl(field) => self.ast_type(decl_module, &field.r#type),
                _ => false,
            }),
            TypeDecl::TypeAlias(node) => self.ast_type(decl_module, &node.r#type),
        };
        self.visiting.remove(&visit_key);
        found
    }

    fn ast_type(&mut self, current_module: &[String], ty: &ast::TypePtr) -> bool {
        use ast::TypeNode as T;
        let Some(t) = ty.as_deref() else {
            return false;
        };
        match &t.node {
            T::TypeDynamic(node) => is_capability_class_path(&node.path),
            T::TypePathType(node) => {
                let args = node.generic_args.iter().any(|arg| self.ast_type(current_module, arg));
                is_context_type_path(&node.path) || args || self.nominal(current_module, &node.path)
            }
            T::TypeModalState(node) => {
                let args = node.generic_args.iter().any(|arg| self.ast_type(current_module, arg));
                args || self.nominal(current_module, &node.path)
            }
            T::TypePermType(node) => self.ast_type(current_module, &node.base),
            T::TypeRefine(node) => self.ast_type(current_module, &node.base),
            T::TypeUnion(node) => node.types.iter().any(|member| self.ast_type(current_module, member)),
            T::TypeTuple(node) => node.elements.iter().any(|elem| self.ast_type(current_module, elem)),
            T::TypeArray(node) => self.ast_type(current_module, &node.element),
            T::TypeSlice(node) => self.ast_type(current_module, &node.element),
            T::TypeSafePtr(node) => self.ast_type(current_module, &node.element),
            T::TypeRawPtr(node) => self.ast_type(current_module, &node.element),
            T::TypeFunc(node) => node.params.iter().any(|param| self.ast_type(current_module, &param.r#type)) || self.ast_type(current_module, &node.ret),
            T::TypeClosure(node) => {
                node.params.iter().any(|param| self.ast_type(current_module, &param.r#type))
                    || self.ast_type(current_module, &node.ret)
                    || node.deps_opt.as_ref().is_some_and(|deps| deps.iter().any(|dep| self.ast_type(current_module, &dep.r#type)))
            }
            _ => false,
        }
    }

    fn lowered(&mut self, current_module: &[String], ty: &TypeRef) -> bool {
        let Some(t) = ty.as_deref() else {
            return false;
        };
        match &t.node {
            TypeNode::Dynamic(path) => is_capability_class_path(path),
            TypeNode::Path { path, generic_args } => {
                is_context_type_path(path) || generic_args.iter().any(|arg| self.lowered(current_module, arg)) || self.nominal(current_module, path)
            }
            TypeNode::ModalState(modal) => modal.generic_args.iter().any(|arg| self.lowered(current_module, arg)) || self.nominal(current_module, &modal.path),
            TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => self.lowered(current_module, base),
            TypeNode::Union(members) => members.iter().any(|member| self.lowered(current_module, member)),
            TypeNode::Tuple(elements) => elements.iter().any(|elem| self.lowered(current_module, elem)),
            TypeNode::Array { element, .. } | TypeNode::Ptr { element, .. } | TypeNode::RawPtr { element, .. } => self.lowered(current_module, element),
            TypeNode::Slice(element) => self.lowered(current_module, element),
            TypeNode::Func { params, ret } => params.iter().any(|param| self.lowered(current_module, &param.r#type)) || self.lowered(current_module, ret),
            TypeNode::Closure { params, ret, deps_opt } => {
                params.iter().any(|(_, ty)| self.lowered(current_module, ty))
                    || self.lowered(current_module, ret)
                    || deps_opt.as_ref().is_some_and(|deps| deps.iter().any(|dep| self.lowered(current_module, &dep.r#type)))
            }
            TypeNode::Range(base)
            | TypeNode::RangeInclusive(base)
            | TypeNode::RangeFrom(base)
            | TypeNode::RangeTo(base)
            | TypeNode::RangeToInclusive(base) => self.lowered(current_module, base),
            _ => false,
        }
    }
}

/// Whether `InferCapabilitiesFromType` would find any capability in the type.
pub fn type_has_capabilities(ctx: &ScopeContext<'_>, current_module: &[String], ty: &TypeRef) -> bool {
    Walk { ctx, visiting: HashSet::new() }.lowered(current_module, ty)
}
