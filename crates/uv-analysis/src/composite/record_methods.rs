//! Receivers of methods.

use uv_source::ast;

use crate::typing::signature::subst_self_type;
use crate::typing::type_lower::{lower_param_mode, LowerError};
use crate::typing::types::*;

pub fn lower_receiver_perm(perm: ast::ReceiverPerm) -> Permission {
    match perm {
        ast::ReceiverPerm::Const => Permission::Const,
        ast::ReceiverPerm::Unique => Permission::Unique,
        ast::ReceiverPerm::Shared => Permission::Shared,
    }
}

/// An explicit receiver must be `Self`, possibly under permissions and refinements.
fn is_explicit_self_receiver_type(ty: &TypeRef) -> bool {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Path { path, .. }) => is_self_var_path(path),
        Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) => is_explicit_self_receiver_type(base),
        _ => false,
    }
}

/// The type of `self` in a method of `base`. `lower_type` lowers an explicit receiver's
/// written type.
pub fn recv_type_for_receiver(
    base: &TypeRef,
    receiver: &ast::Receiver,
    lower_type: impl FnOnce(&ast::TypePtr) -> Result<TypeRef, LowerError>,
) -> Result<TypeRef, LowerError> {
    match receiver {
        ast::Receiver::ReceiverShorthand(recv) => Ok(make_type_perm(lower_receiver_perm(recv.perm), base.clone())),
        ast::Receiver::ReceiverExplicit(recv) => {
            let lowered = lower_type(&recv.r#type)?;
            if !is_explicit_self_receiver_type(&lowered) {
                return Err(Some("Record-Method-RecvSelf-Err"));
            }
            Ok(subst_self_type(base, &lowered, None))
        }
    }
}

pub fn recv_mode_of(receiver: &ast::Receiver) -> Option<ParamMode> {
    match receiver {
        ast::Receiver::ReceiverShorthand(recv) => lower_param_mode(recv.mode_opt),
        ast::Receiver::ReceiverExplicit(recv) => lower_param_mode(recv.mode_opt),
    }
}

/// The method a call `value~>name(…)` names, found without looking at arguments.
#[derive(Debug, Clone, Default)]
pub struct StaticMethodLookup<'c> {
    pub record_decl: Option<&'c ast::RecordDecl>,
    pub record_method: Option<&'c ast::MethodDecl>,
    pub class_method: Option<&'c ast::ClassMethodDecl>,
    pub record_path: TypePath,
    pub record_generic_args: Vec<TypeRef>,
    pub owner_class: Vec<String>,
}

/// A record's own method wins; otherwise the method comes from the classes the type
/// implements (for a type parameter, from its bounds), and there must be exactly one.
/// For a declared type only class methods with a body count.
pub fn lookup_method_static<'c>(
    ctx: &'c crate::context::ScopeContext<'_>,
    base: &TypeRef,
    name: &str,
) -> Result<StaticMethodLookup<'c>, Option<&'static str>> {
    use crate::composite::classes::class_method_table;
    use crate::context::{EntityKind, TypeDecl};
    use crate::resolve::scopes::{id_eq, id_key_of, path_key_of};
    let Some(base_ty) = base.as_deref() else {
        return Err(None);
    };
    // An opaque type is looked through once its underlying type is known.
    let lookup_base = match &base_ty.node {
        TypeNode::Opaque { class_path, .. } => ctx
            .sigma
            .opaque_underlying_by_class_path
            .get(&path_key_of(class_path))
            .and_then(|underlying| underlying.as_deref())
            .unwrap_or(base_ty),
        _ => base_ty,
    };
    let method_type_path = applied_type_path(lookup_base);
    let mut record = None;
    let mut implements: Vec<Vec<String>> = Vec::new();
    let mut type_param_bound_lookup = false;
    if let Some(path) = method_type_path {
        match ctx.sigma.types.get(&path_key_of(path)) {
            Some(TypeDecl::Record(decl)) => {
                record = Some(decl);
                implements = decl.implements.clone();
            }
            Some(TypeDecl::Enum(decl)) => implements = decl.implements.clone(),
            Some(TypeDecl::Modal(decl)) => implements = decl.implements.clone(),
            Some(TypeDecl::TypeAlias(_)) => {}
            None => {
                let type_param = path.last().and_then(|last| {
                    let key = id_key_of(last);
                    ctx.scopes.iter().filter_map(|scope| scope.get(&key)).find(|entity| {
                        entity.kind == EntityKind::Type
                            && !entity.type_param_class_bounds.is_empty()
                            && entity.target_opt.as_ref().is_none_or(|target| id_eq(target, last))
                    })
                });
                if let Some(entity) = type_param {
                    implements = entity.type_param_class_bounds.iter().map(|bound| bound.class_path.clone()).collect();
                    type_param_bound_lookup = !implements.is_empty();
                }
            }
        }
    }
    if let Some(record) = record {
        let method = record.members.iter().find_map(|member| match member {
            ast::RecordMember::MethodDecl(method) if id_eq(&method.name, name) => Some(method),
            _ => None,
        });
        if let Some(method) = method {
            return Ok(StaticMethodLookup {
                record_decl: Some(record),
                record_method: Some(method),
                record_path: method_type_path.cloned().unwrap_or_default(),
                record_generic_args: applied_type_args(lookup_base).map(<[TypeRef]>::to_vec).unwrap_or_default(),
                ..Default::default()
            });
        }
    }
    let mut defaults: Vec<(&ast::ClassMethodDecl, Vec<String>)> = Vec::new();
    for class_path in &implements {
        for entry in class_method_table(ctx, class_path).map_err(Some)? {
            if (!type_param_bound_lookup && entry.method.body_opt.is_none()) || !id_eq(&entry.method.name, name) {
                continue;
            }
            if !defaults.iter().any(|(seen, _)| std::ptr::eq(*seen, entry.method)) {
                defaults.push((entry.method, entry.owner));
            }
        }
    }
    match defaults.len() {
        0 => Err(Some("LookupMethod-NotFound")),
        1 => {
            let (method, owner) = defaults.remove(0);
            Ok(StaticMethodLookup { class_method: Some(method), owner_class: owner, ..Default::default() })
        }
        _ => Err(Some("LookupMethod-Ambig")),
    }
}
