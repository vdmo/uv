//! Which classes are capabilities.

use super::builtin_paths::is_capability_class_path;
use crate::context::{PathKey, ScopeContext};
use crate::resolve::scopes::path_key_of;

fn is_capability_class_from(ctx: &ScopeContext<'_>, path: &[String], visiting: &mut Vec<PathKey>) -> bool {
    if is_capability_class_path(path) {
        return true;
    }
    let key = path_key_of(path);
    // A superclass cycle is reported elsewhere; here it is simply not a capability.
    if visiting.contains(&key) {
        return false;
    }
    let Some(decl) = ctx.sigma.classes.get(&key) else {
        return false;
    };
    visiting.push(key);
    let found = decl.supers.iter().any(|sup| is_capability_class_from(ctx, sup, visiting));
    visiting.pop();
    found
}

/// A built-in capability class, or a class whose superclass chain reaches one.
pub fn is_capability_class(ctx: &ScopeContext<'_>, path: &[String]) -> bool {
    is_capability_class_from(ctx, path, &mut Vec::new())
}
