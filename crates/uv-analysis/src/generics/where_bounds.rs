//! Class bounds on type arguments.

use crate::composite::classes::type_implements_class;
use crate::context::ScopeContext;
use crate::typing::types::TypeRef;

pub fn check_class_bound(ctx: &ScopeContext<'_>, ty: &TypeRef, class_path: &[String]) -> bool {
    ty.is_some() && !class_path.is_empty() && type_implements_class(ctx, ty, class_path)
}
