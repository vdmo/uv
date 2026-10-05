//! Array lengths: an integer literal, a module-level binding initialised with one, or an
//! expression the compile-time evaluator reduces to an integer.

use std::cell::RefCell;
use std::rc::Rc;

use uv_comptime::eval::eval_expr;
use uv_comptime::value::{bind_ct_proc, ct_empty_env, CtPrim, CtValue};
use uv_core::numeric_literals::{parse_int_core, strip_int_suffix};
use uv_core::symbols::string_of_path;
use uv_source::ast;
use uv_source::lexer::token::TokenKind;
use uv_source::module_paths::ModuleNames;

use crate::context::*;
use crate::resolve::collect_toplevel::{collect_name_maps, pat_names_ptr};
use crate::resolve::scopes::{id_eq, path_eq};
use crate::resolve::scopes_lookup::{module_names_of, resolve_qualified, resolve_value_name};
use crate::resolve::visibility::can_access;

const ERR: Option<&str> = Some("ConstLen-Err");

/// A literal that fits `usize` on the 64-bit targets.
fn parse_int_literal_usize(lexeme: &str) -> Option<u64> {
    u64::try_from(parse_int_core(strip_int_suffix(lexeme))?).ok()
}

fn find_module<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::ASTModule> {
    ctx.sigma.mods.iter().find(|module| path_eq(&module.path, path))
}

/// The initialiser of the first `name = ..` binding at module level.
fn find_static_init<'m>(module: &'m ast::ASTModule, name: &str) -> Option<&'m ast::ExprPtr> {
    module.items.iter().find_map(|item| match item {
        ast::ASTItem::StaticDecl(decl)
            if decl.binding.pat.is_some()
                && pat_names_ptr(&decl.binding.pat).iter().any(|bound| id_eq(bound, name))
                && decl.binding.op.kind == TokenKind::Operator
                && decl.binding.op.lexeme == "=" =>
        {
            Some(&decl.binding.init)
        }
        _ => None,
    })
}

fn current_module_static_init<'c>(ctx: &'c ScopeContext<'_>, name: &str) -> Option<&'c ast::ExprPtr> {
    if ctx.current_module.is_empty() {
        return None;
    }
    find_static_init(find_module(ctx, &ctx.current_module)?, name)
}

fn module_names_for_const_len(ctx: &ScopeContext<'_>) -> ModuleNames {
    match ctx.project {
        Some(project) => module_names_of(project),
        None => ctx.sigma.mods.iter().map(|module| string_of_path(&module.path)).collect(),
    }
}

/// Identifies the program a set of name maps was computed for.
#[derive(PartialEq, Eq, Clone, Copy)]
struct ProgramKey {
    mods: usize,
    mods_len: usize,
    types_len: usize,
    classes_len: usize,
    project: usize,
}

thread_local! {
    static NAME_MAPS: RefCell<Option<(ProgramKey, Rc<NameMapTable>)>> = const { RefCell::new(None) };
}

/// Name maps for a qualified length. Lowering has no name maps at hand, so they are
/// collected once per program and kept.
fn cached_name_maps(ctx: &ScopeContext<'_>) -> Rc<NameMapTable> {
    let key = ProgramKey {
        mods: ctx.sigma.mods.as_ptr() as usize,
        mods_len: ctx.sigma.mods.len(),
        types_len: ctx.sigma.types.len(),
        classes_len: ctx.sigma.classes.len(),
        project: ctx.project.map_or(0, |project| project as *const _ as usize),
    };
    NAME_MAPS.with(|cache| {
        if let Some((cached_key, maps)) = &*cache.borrow() {
            if *cached_key == key {
                return maps.clone();
            }
        }
        let maps = Rc::new(collect_name_maps(&mut ctx.clone()).name_maps);
        *cache.borrow_mut() = Some((key, maps.clone()));
        maps
    })
}

fn resolve_value_path(ctx: &ScopeContext<'_>, path: &[String], name: &str) -> Option<(Vec<String>, String)> {
    let name_maps = cached_name_maps(ctx);
    let module_names = module_names_for_const_len(ctx);
    let value = resolve_qualified(ctx, &name_maps, &module_names, path, name, EntityKind::Value, Some(can_access));
    let entity = value.entity.filter(|_| value.ok)?;
    Some((entity.origin_opt?, entity.target_opt.unwrap_or_else(|| name.to_string())))
}

/// Any other expression goes to the compile-time evaluator, with the current module's
/// compile-time procedures available.
fn evaluate(ctx: &ScopeContext<'_>, expr: &ast::ExprPtr) -> Option<u64> {
    let current = find_module(ctx, &ctx.current_module)?;
    let mut env = ct_empty_env(current);
    env.diags = Some(Rc::new(RefCell::new(Vec::new())));
    for module in &ctx.sigma.mods {
        env.available_modules.push(Rc::new(module.clone()));
        env.available_module_names.insert(string_of_path(&module.path));
    }
    for proc in &current.comptime_procedures {
        env = bind_ct_proc(env, proc);
    }
    let eval = eval_expr(expr, &mut env);
    match eval.value {
        CtValue::Prim(CtPrim::Int(int)) if eval.ok => Some(int.value),
        _ => None,
    }
}

pub fn const_len(ctx: &ScopeContext<'_>, expr_ptr: &ast::ExprPtr) -> Result<u64, Option<&'static str>> {
    const_len_following(ctx, expr_ptr, &mut Vec::new())
}

/// `following` holds the initialisers being evaluated: a binding defined through itself
/// has no length (the reference recurses until it crashes there).
fn const_len_following(
    ctx: &ScopeContext<'_>,
    expr_ptr: &ast::ExprPtr,
    following: &mut Vec<*const ast::Expr>,
) -> Result<u64, Option<&'static str>> {
    let Some(expr) = expr_ptr else {
        return Err(ERR);
    };
    let mut follow = |init: &ast::ExprPtr| {
        let Some(target) = init.as_deref().map(|target| target as *const ast::Expr) else {
            return Err(ERR);
        };
        if following.contains(&target) {
            return Err(ERR);
        }
        following.push(target);
        let result = const_len_following(ctx, init, following);
        following.pop();
        result
    };
    // A name that does not lead to a module-level binding elsewhere is tried in the
    // current module.
    let in_current_module = |name: &str, follow: &mut dyn FnMut(&ast::ExprPtr) -> Result<u64, Option<&'static str>>| {
        match current_module_static_init(ctx, name) {
            Some(init) => follow(init),
            None => Err(ERR),
        }
    };
    match &expr.node {
        ast::ExprNode::LiteralExpr(node) => {
            if node.literal.kind != TokenKind::IntLiteral {
                return Err(ERR);
            }
            parse_int_literal_usize(&node.literal.lexeme).ok_or(ERR)
        }
        ast::ExprNode::IdentifierExpr(node) => {
            let Some(Entity { origin_opt: Some(origin), target_opt, .. }) = resolve_value_name(ctx, &node.name) else {
                return in_current_module(&node.name, &mut follow);
            };
            let resolved_name = target_opt.as_ref().unwrap_or(&node.name);
            match find_module(ctx, &origin).and_then(|module| find_static_init(module, resolved_name)) {
                Some(init) => follow(init),
                None => in_current_module(&node.name, &mut follow),
            }
        }
        ast::ExprNode::PathExpr(node) => {
            let init = resolve_value_path(ctx, &node.path, &node.name)
                .and_then(|(module_path, name)| find_static_init(find_module(ctx, &module_path)?, &name).cloned());
            match init {
                Some(init) => follow(&init),
                None if node.path.is_empty() => in_current_module(&node.name, &mut follow),
                None => Err(ERR),
            }
        }
        _ => evaluate(ctx, expr_ptr).ok_or(ERR),
    }
}
