//! Well-formedness of types: every name is known, argument counts fit, unions have at
//! least two distinct members.

use uv_source::ast;

use super::stmt_context::StmtTypeContext;
use super::type_env::{TypeBinding, TypeEnv};
use super::type_equiv::type_equiv;
use super::type_expr::type_expr;
use crate::contracts::purity::{check_purity, ContractContext};
use crate::resolve::scopes::id_key_of;
use super::type_lookup::type_params_of;
use super::type_lower::lower_type;
use super::type_predicates::{perm_of_type, strip_perm};
use super::types::*;
use crate::caps::builtin_paths::is_capability_class_path;
use crate::composite::classes::{class_method_table, vtable_eligible};
use crate::context::ScopeContext;
use crate::generics::generic_params::{required_param_count, total_param_count};
use crate::modal::lookup::{has_state, lookup_modal_decl};
use crate::resolve::scopes::{id_eq, path_key_of, PRIM_TYPE_NAMES};
use crate::resolve::scopes_lookup::resolve_type_name;

/// Why a type is not well formed; some failures name no rule.
pub type WfError = Option<&'static str>;

/// Reported for a refinement type until expression typing exists: its predicate must be
/// typed as `bool` and checked for purity, which this crate cannot do yet.
pub const REFINEMENT_WF_PENDING: &str = "<refinement well-formedness needs expression typing>";

pub fn is_prim_type_name(name: &str) -> bool {
    PRIM_TYPE_NAMES.contains(&name) || name == "!" || name == "()"
}

fn is_gpu_ptr_address_space_type(ty: &TypeRef) -> bool {
    matches!(strip_perm(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::Path { path, generic_args })
        if generic_args.is_empty()
            && matches!(&path[..], [only] if ["Global", "Shared", "Private"].iter().any(|name| id_eq(only, name))))
}

fn is_known_type_path(ctx: &ScopeContext<'_>, path: &[String]) -> bool {
    if ctx.sigma.types.contains_key(&path_key_of(path)) {
        return true;
    }
    let [name] = path else {
        return false;
    };
    let Some(entity) = resolve_type_name(ctx, name) else {
        return false;
    };
    match entity.origin_opt {
        // A type without a declaring module is a type parameter, which names itself.
        None => entity.target_opt.is_some_and(|target| id_eq(&target, name)),
        Some(mut resolved) => {
            resolved.push(entity.target_opt.unwrap_or_else(|| name.clone()));
            ctx.sigma.types.contains_key(&path_key_of(&resolved))
        }
    }
}

fn receiver_is_const(ctx: &ScopeContext<'_>, method: &ast::ClassMethodDecl) -> bool {
    match &method.receiver {
        ast::Receiver::ReceiverShorthand(recv) => recv.perm == ast::ReceiverPerm::Const && recv.mode_opt.is_none(),
        ast::Receiver::ReceiverExplicit(recv) => {
            lower_type(ctx, &recv.r#type).map_or(true, |lowered| perm_of_type(&lowered) == Permission::Const)
        }
    }
}

/// `shared $C` requires every dispatchable method of `C` to take a `const` receiver.
fn shared_dynamic_receivers_are_const(ctx: &ScopeContext<'_>, class_path: &[String]) -> bool {
    class_method_table(ctx, class_path).map_or(true, |table| {
        table.iter().all(|entry| !vtable_eligible(entry.method) || receiver_is_const(ctx, entry.method))
    })
}

fn all_wf(ctx: &ScopeContext<'_>, types: &[TypeRef]) -> Result<(), WfError> {
    types.iter().try_for_each(|ty| type_wf(ctx, ty))
}

/// `Async` takes one to four arguments, each well formed.
fn async_args_wf(ctx: &ScopeContext<'_>, args: &[TypeRef]) -> Result<(), WfError> {
    if !(1..=4).contains(&args.len()) {
        return Err(Some("WF-Async-ArgCount-Err"));
    }
    all_wf(ctx, args).map_err(|_| Some("WF-Async-Arg-WF-Err"))
}

pub fn type_wf(ctx: &ScopeContext<'_>, type_ref: &TypeRef) -> Result<(), WfError> {
    let Some(ty) = type_ref.as_deref() else {
        return Err(None);
    };
    match &ty.node {
        TypeNode::Prim(name) => is_prim_type_name(name).then_some(()).ok_or(None),
        TypeNode::Perm { perm, base } => {
            type_wf(ctx, base)?;
            match (perm, base.as_deref().map(|ty| &ty.node)) {
                (Permission::Shared, Some(TypeNode::Dynamic(path))) if !shared_dynamic_receivers_are_const(ctx, path) => {
                    Err(Some("E-CON-0083"))
                }
                _ => Ok(()),
            }
        }
        TypeNode::Tuple(elements) => all_wf(ctx, elements),
        TypeNode::Array { element, .. }
        | TypeNode::Slice(element)
        | TypeNode::Ptr { element, .. }
        | TypeNode::RawPtr { element, .. } => type_wf(ctx, element),
        TypeNode::Union(members) => {
            if members.len() < 2 {
                return Err(Some("WF-Union-TooFew"));
            }
            let mut distinct: Vec<&TypeRef> = Vec::new();
            for member in members {
                type_wf(ctx, member)?;
                if !distinct.iter().any(|seen| type_equiv(seen, member)) {
                    distinct.push(member);
                }
            }
            if distinct.len() < 2 {
                return Err(Some("WF-Union-TooFew"));
            }
            Ok(())
        }
        TypeNode::Func { params, ret } => {
            params.iter().try_for_each(|param| type_wf(ctx, &param.r#type))?;
            type_wf(ctx, ret)
        }
        TypeNode::Closure { params, ret, deps_opt } => {
            params.iter().try_for_each(|(_, param)| type_wf(ctx, param))?;
            type_wf(ctx, ret)?;
            deps_opt.iter().flatten().try_for_each(|dep| type_wf(ctx, &dep.r#type))
        }
        TypeNode::Path { path, generic_args } => {
            if is_async_modal_path(path) {
                if generic_args.is_empty() {
                    return Err(Some("WF-Async-Path-Err"));
                }
                return async_args_wf(ctx, generic_args);
            }
            let is_self_associated = path.len() == 2 && id_eq(&path[0], "Self");
            let is_test_authority = matches!(&path[..], [only] if id_eq(only, "TestAuthority"));
            if is_self_var_path(path) || is_self_associated || is_test_authority {
                return Ok(());
            }
            // A generic type named without arguments.
            if type_params_of(ctx, path).is_some_and(|params| total_param_count(params) > 0) {
                return Err(Some("E-TYP-2303"));
            }
            is_known_type_path(ctx, path).then_some(()).ok_or(None)
        }
        TypeNode::Apply { path, args } => {
            if matches!(&path[..], [only] if id_eq(only, "GpuPtr")) {
                return match &args[..] {
                    [pointee, space] if is_gpu_ptr_address_space_type(space) => type_wf(ctx, pointee),
                    _ => Err(None),
                };
            }
            if is_async_modal_path(path) {
                return async_args_wf(ctx, args);
            }
            if let Some(params) = type_params_of(ctx, path) {
                if args.len() < required_param_count(params) || args.len() > total_param_count(params) {
                    return Err(Some("E-TYP-2303"));
                }
            }
            all_wf(ctx, args)?;
            is_known_type_path(ctx, path).then_some(()).ok_or(None)
        }
        TypeNode::Dynamic(path) => {
            if is_capability_class_path(path) || ctx.sigma.classes.contains_key(&path_key_of(path)) {
                Ok(())
            } else {
                Err(Some("Superclass-Undefined"))
            }
        }
        TypeNode::Opaque { class_path, .. } => {
            ctx.sigma.classes.contains_key(&path_key_of(class_path)).then_some(()).ok_or(Some("Superclass-Undefined"))
        }
        TypeNode::Refine { base, predicate } => {
            type_wf(ctx, base)?;
            if predicate.is_none() {
                return Err(None);
            }
            // The predicate is a pure `bool` expression that may speak of `self`.
            let mut env = TypeEnv::default();
            env.scopes.push(Default::default());
            env.scopes[0].insert(id_key_of("self"), TypeBinding { r#type: base.clone(), ..Default::default() });
            let type_ctx = StmtTypeContext { return_type: make_type_prim("bool"), ..Default::default() };
            let pred_type = type_expr(ctx, &type_ctx, predicate, &env);
            if !pred_type.ok {
                return Err(pred_type.diag_id);
            }
            if !matches!(strip_perm(&pred_type.r#type).as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "bool") {
                return Err(Some("E-TYP-1955"));
            }
            if !check_purity(&ContractContext::default(), predicate).ok {
                return Err(Some("E-TYP-1954"));
            }
            Ok(())
        }
        TypeNode::String(_) | TypeNode::Bytes(_) | TypeNode::RangeFull => Ok(()),
        TypeNode::ModalState(node) => {
            let decl = lookup_modal_decl(ctx, node.modal_ref.path()).filter(|decl| has_state(decl, &node.state)).ok_or(None)?;
            let provided = node.generic_args.len();
            if provided < required_param_count(&decl.generic_params) || provided > total_param_count(&decl.generic_params) {
                return Err(Some("WF-ModalState-ArgCount-Err"));
            }
            all_wf(ctx, &node.generic_args)
        }
        TypeNode::Range(base)
        | TypeNode::RangeInclusive(base)
        | TypeNode::RangeFrom(base)
        | TypeNode::RangeTo(base)
        | TypeNode::RangeToInclusive(base) => type_wf(ctx, base),
        TypeNode::Var(_) => Err(None),
    }
}
