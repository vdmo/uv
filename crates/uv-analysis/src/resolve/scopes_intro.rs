//! Introducing names into the innermost scope, and validating a module's names.

use std::collections::HashMap;

use uv_core::span::Span;
use uv_core::spec_rule;
use uv_project::language_profile::active_language_profile;

use super::scopes::*;
use crate::context::{Entity, IdKey, Scope, ScopeContext};

/// The outcome of an introduction; `diag_id` names the rule that rejected it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IntroResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
}

fn rejected(diag_id: &'static str) -> IntroResult {
    IntroResult { ok: false, diag_id: Some(diag_id) }
}

fn reserved_language_root(name: &str) -> bool {
    id_eq(name, active_language_profile().runtime_root)
}

fn is_capability_class_reserved_key(key: &str) -> bool {
    ["IO", "Network", "HeapAllocator", "ExecutionDomain", "System", "Reactor", "Time", "MonotonicTime", "WallTime"]
        .contains(&key)
}

fn is_foundational_class_reserved_key(key: &str) -> bool {
    ["Bitcopy", "Clone", "Drop", "FfiSafe", "GpuSafe", "Eq", "Hasher", "Hash", "Iterator", "Discrete"].contains(&key)
}

/// The rule that forbids declaring a universe-protected name at module level.
fn universe_shadow_rule(key: &str, trace_class_rules: bool) -> Option<&'static str> {
    if PRIM_TYPE_NAMES.contains(&key) {
        spec_rule!("Validate-Module-Prim-Shadow-Err");
        return Some("Validate-Module-Prim-Shadow-Err");
    }
    if SPECIAL_TYPE_NAMES.contains(&key) {
        if trace_class_rules && is_capability_class_reserved_key(key) {
            spec_rule!("req.14.CapabilityClassNamesReserved");
        }
        if trace_class_rules && is_foundational_class_reserved_key(key) {
            spec_rule!("req.14.FoundationalClassesSyntaxAndReservedNames");
        }
        spec_rule!("Validate-Module-Special-Shadow-Err");
        return Some("Validate-Module-Special-Shadow-Err");
    }
    if ASYNC_TYPE_NAMES.contains(&key) {
        spec_rule!("Validate-Module-Async-Shadow-Err");
        return Some("Validate-Module-Async-Shadow-Err");
    }
    None
}

pub fn in_scope(scope: &Scope, name: &str) -> bool {
    scope.contains_key(&id_key_of(name))
}

pub fn in_outer(ctx: &ScopeContext<'_>, name: &str) -> bool {
    ctx.scopes.iter().skip(1).any(|scope| in_scope(scope, name))
}

fn intro_impl(ctx: &mut ScopeContext<'_>, name: &str, ent: &Entity, shadow: bool) -> IntroResult {
    if reserved_gen(name) || reserved_language_root(name) {
        let rule = if shadow { "Shadow-Reserved-Id-Err" } else { "Intro-Reserved-Id-Err" };
        uv_core::spec_trace::Conformance::record_if_enabled(rule);
        return rejected(rule);
    }
    if ctx.scopes.is_empty() {
        return rejected("ResolveExpr-Ident-Err");
    }
    if in_scope(&ctx.scopes[0], name) {
        spec_rule!("Intro-Dup");
        return rejected("Intro-Dup");
    }
    let outer = in_outer(ctx, name);
    if shadow && !outer {
        spec_rule!("Shadow-Unnecessary");
        return rejected("Shadow-Unnecessary");
    }
    if !shadow && outer {
        spec_rule!("Intro-Outer-Err");
        return rejected("Intro-Outer-Err");
    }
    let key = id_key_of(name);
    // The module scope is the second-to-last; it is "current" when it is also the innermost.
    let module_scope_current = ctx.scopes.len() == 2;
    let protected = PRIM_TYPE_NAMES.contains(&key.as_str())
        || SPECIAL_TYPE_NAMES.contains(&key.as_str())
        || ASYNC_TYPE_NAMES.contains(&key.as_str());
    if module_scope_current && protected {
        return rejected(universe_shadow_rule(&key, !shadow).unwrap_or("ResolveExpr-Ident-Err"));
    }
    if shadow {
        spec_rule!("Shadow-Ok");
    } else {
        spec_rule!("Intro-Ok");
    }
    ctx.scopes[0].entry(key).or_insert_with(|| ent.clone());
    IntroResult { ok: true, diag_id: None }
}

/// Introduces a name that must not exist in any enclosing scope.
pub fn intro(ctx: &mut ScopeContext<'_>, name: &str, ent: &Entity) -> IntroResult {
    intro_impl(ctx, name, ent, false)
}

/// Introduces a name that deliberately shadows one from an enclosing scope.
pub fn shadow_intro(ctx: &mut ScopeContext<'_>, name: &str, ent: &Entity) -> IntroResult {
    intro_impl(ctx, name, ent, true)
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidateModuleNamesResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
}

/// Checks a module's names against the reserved sets, class of violation by class of
/// violation, each in key order.
pub fn validate_module_names(names: &Scope, name_spans: &HashMap<IdKey, Option<Span>>) -> ValidateModuleNamesResult {
    let mut keys: Vec<&IdKey> = names.keys().collect();
    keys.sort();
    let fail = |diag_id: &'static str, key: &IdKey| ValidateModuleNamesResult {
        ok: false,
        diag_id: Some(diag_id),
        span: name_spans.get(key).cloned().flatten(),
    };
    if let Some(key) = keys.iter().find(|key| reserved_gen(key) || reserved_language_root(key)) {
        spec_rule!("Intro-Reserved-Id-Err");
        return fail("Intro-Reserved-Id-Err", key);
    }
    if let Some(key) = keys.iter().find(|key| keyword_key(key)) {
        spec_rule!("Validate-Module-Keyword-Err");
        return fail("Validate-Module-Keyword-Err", key);
    }
    for class in [&PRIM_TYPE_NAMES[..], &SPECIAL_TYPE_NAMES[..], &ASYNC_TYPE_NAMES[..]] {
        if let Some(key) = keys.iter().find(|key| class.contains(&key.as_str())) {
            let rule = universe_shadow_rule(key, true).expect("key is in a protected class");
            return fail(rule, key);
        }
    }
    spec_rule!("Validate-Module-Ok");
    ValidateModuleNamesResult { ok: true, diag_id: None, span: None }
}
