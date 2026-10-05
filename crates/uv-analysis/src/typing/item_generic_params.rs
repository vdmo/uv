//! The generic parameters of a declaration as declaration typing sees them, and the
//! check of type arguments against them.

use std::collections::HashSet;

use uv_source::ast;

use super::type_lower::lower_type;
use super::types::TypeRef;
use crate::caps::builtin_paths::is_capability_class_path;
use crate::composite::classes::type_implements_class;
use crate::context::ScopeContext;
use crate::resolve::scopes::path_key_of;

#[derive(Debug, Clone)]
pub struct GenericParamInfo {
    pub name: String,
    pub class_bounds: Vec<ast::TypeBound>,
    pub default_type: TypeRef,
}

/// Names are unique (`E-TYP-2304`), bounds name classes that exist (`E-TYP-2305`), bound
/// arguments and defaults lower, and defaults come last. The last is reported with the
/// rule for duplicate names, as the reference does.
pub fn process_generic_params(
    ctx: &ScopeContext<'_>,
    params: &[ast::TypeParam],
) -> Result<Vec<GenericParamInfo>, Option<&'static str>> {
    let mut seen_names = HashSet::new();
    let mut infos = Vec::with_capacity(params.len());
    for param in params {
        if !seen_names.insert(&param.name) {
            return Err(Some("E-TYP-2304"));
        }
        for bound in &param.bounds {
            let exists = is_capability_class_path(&bound.class_path)
                || ctx.sigma.classes.contains_key(&path_key_of(&bound.class_path));
            if !exists {
                return Err(Some("E-TYP-2305"));
            }
            for arg in &bound.generic_args {
                lower_type(ctx, arg)?;
            }
        }
        let default_type = if param.default_type.is_some() { lower_type(ctx, &param.default_type)? } else { None };
        infos.push(GenericParamInfo { name: param.name.clone(), class_bounds: param.bounds.clone(), default_type });
    }
    let mut seen_default = false;
    for info in &infos {
        if info.default_type.is_some() {
            seen_default = true;
        } else if seen_default {
            return Err(Some("E-TYP-2304"));
        }
    }
    Ok(infos)
}

/// The number of arguments fits (`E-TYP-2303`) and each implements the classes its
/// parameter is bounded by (`E-TYP-2302`).
pub fn check_generic_args(
    ctx: &ScopeContext<'_>,
    params: &[GenericParamInfo],
    args: &[TypeRef],
) -> Result<(), &'static str> {
    let required = params.iter().filter(|param| param.default_type.is_none()).count();
    if args.len() < required || args.len() > params.len() {
        return Err("E-TYP-2303");
    }
    for (param, arg) in params.iter().zip(args) {
        if param.class_bounds.iter().any(|bound| !type_implements_class(ctx, arg, &bound.class_path)) {
            return Err("E-TYP-2302");
        }
    }
    Ok(())
}
