//! The types of module-level values: statics, and procedures used as values.

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use uv_core::symbols::string_of_path;
use uv_source::ast::{self, ASTItem, ExternItem};
use uv_source::module_paths::ModuleNames;

use crate::caps::cap_concurrency::lookup_gpu_intrinsic_type;
use crate::context::{EntityKind, NameMapTable, ScopeContext};
use crate::memory::string_bytes::lookup_string_bytes_builtin_type;
use crate::modal::builtin_modal_intrinsics::lookup_builtin_modal_static_func_sig;
use crate::resolve::collect_toplevel::{collect_name_maps, pat_names};
use crate::resolve::scopes::{id_eq, path_eq};
use crate::resolve::scopes_lookup::{
    find_context_module_by_path, module_names_of, resolve_qualified,
};
use crate::resolve::visibility::can_access;
use crate::typing::type_env::type_pattern;
use crate::typing::type_lower::{lower_param_mode, lower_type};
use crate::typing::types::{make_type_func, make_type_prim, TypeFuncParam, TypeRef};

/// The type of a value path. `ok` with no type means the path names no value.
#[derive(Debug, Clone, Default)]
pub struct ValuePathTypeResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub r#type: TypeRef,
}

impl ValuePathTypeResult {
    fn typed(ty: TypeRef) -> Self {
        ValuePathTypeResult {
            ok: true,
            diag_id: None,
            r#type: ty,
        }
    }

    fn none() -> Self {
        ValuePathTypeResult {
            ok: true,
            diag_id: None,
            r#type: None,
        }
    }

    fn failed(diag_id: Option<&'static str>) -> Self {
        ValuePathTypeResult {
            ok: false,
            diag_id,
            r#type: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModuleStaticLookupResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub r#type: TypeRef,
    pub is_var: bool,
}

/// A procedure declaration of any of the three kinds, as far as its type goes.
struct ProcedureView<'d> {
    params: &'d [ast::Param],
    return_type_opt: &'d ast::TypePtr,
}

fn find_module<'c>(ctx: &'c ScopeContext<'_>, path: &[String]) -> Option<&'c ast::ASTModule> {
    find_context_module_by_path(ctx, path)
}

/// The first procedure of the name, in the order the reference indexes them: the
/// module's compile-time procedures, then its items with extern blocks inline.
fn find_procedure<'m>(module: &'m ast::ASTModule, name: &str) -> Option<ProcedureView<'m>> {
    if let Some(decl) = module
        .comptime_procedures
        .iter()
        .find(|decl| id_eq(&decl.name, name))
    {
        return Some(ProcedureView {
            params: &decl.params,
            return_type_opt: &decl.return_type_opt,
        });
    }
    for item in &module.items {
        match item {
            ASTItem::ProcedureDecl(decl) if id_eq(&decl.name, name) => {
                return Some(ProcedureView {
                    params: &decl.params,
                    return_type_opt: &decl.return_type_opt,
                });
            }
            ASTItem::ComptimeProcedureDecl(decl) if id_eq(&decl.name, name) => {
                return Some(ProcedureView {
                    params: &decl.params,
                    return_type_opt: &decl.return_type_opt,
                });
            }
            ASTItem::ExternBlock(block) => {
                for ExternItem::ExternProcDecl(decl) in &block.items {
                    if id_eq(&decl.name, name) {
                        return Some(ProcedureView {
                            params: &decl.params,
                            return_type_opt: &decl.return_type_opt,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn build_proc_type(
    ctx: &ScopeContext<'_>,
    params_src: &[ast::Param],
    return_type_opt: &ast::TypePtr,
) -> ValuePathTypeResult {
    let mut params = Vec::with_capacity(params_src.len());
    for param in params_src {
        match lower_type(ctx, &param.r#type) {
            Ok(lowered) => params.push(TypeFuncParam {
                mode: lower_param_mode(param.mode),
                r#type: lowered,
            }),
            Err(diag_id) => return ValuePathTypeResult::failed(diag_id),
        }
    }
    let ret = if return_type_opt.is_none() {
        make_type_prim("()")
    } else {
        match lower_type(ctx, return_type_opt) {
            Ok(lowered) => lowered,
            Err(diag_id) => return ValuePathTypeResult::failed(diag_id),
        }
    };
    ValuePathTypeResult::typed(make_type_func(params, ret))
}

fn proc_type(ctx: &ScopeContext<'_>, view: &ProcedureView<'_>) -> ValuePathTypeResult {
    build_proc_type(ctx, view.params, view.return_type_opt)
}

/// The type of a procedure declaration used as a value.
pub fn procedure_decl_type(
    ctx: &ScopeContext<'_>,
    decl: &ast::ProcedureDecl,
) -> ValuePathTypeResult {
    build_proc_type(ctx, &decl.params, &decl.return_type_opt)
}

fn lookup_module_static_in_module(
    ctx: &ScopeContext<'_>,
    module: &ast::ASTModule,
    name: &str,
) -> ModuleStaticLookupResult {
    for item in &module.items {
        let ASTItem::StaticDecl(decl) = item else {
            continue;
        };
        let Some(pat) = decl.binding.pat.as_deref() else {
            continue;
        };
        if !pat_names(pat).iter().any(|bound| id_eq(bound, name)) {
            continue;
        }
        let ann_type = ast::binding_annotation_type_opt(&decl.binding);
        if ann_type.is_none() {
            continue;
        }
        let lowered = match lower_type(ctx, &ann_type) {
            Ok(lowered) => lowered,
            Err(diag_id) => {
                return ModuleStaticLookupResult {
                    ok: false,
                    diag_id,
                    r#type: None,
                    is_var: false,
                }
            }
        };
        let bindings = match type_pattern(ctx, &decl.binding.pat, &lowered) {
            Ok(bindings) => bindings,
            Err(diag_id) => {
                return ModuleStaticLookupResult {
                    ok: false,
                    diag_id,
                    r#type: None,
                    is_var: false,
                }
            }
        };
        if let Some((_, ty)) = bindings.into_iter().find(|(bound, _)| id_eq(bound, name)) {
            return ModuleStaticLookupResult {
                ok: true,
                diag_id: None,
                r#type: ty,
                is_var: decl.r#mut == ast::Mutability::Var,
            };
        }
    }
    ModuleStaticLookupResult {
        ok: true,
        ..Default::default()
    }
}

/// The static `name` declared in the module at `path`, if any.
pub fn lookup_module_static(
    ctx: &ScopeContext<'_>,
    path: &[String],
    name: &str,
) -> ModuleStaticLookupResult {
    match find_module(ctx, path) {
        Some(module) => lookup_module_static_in_module(ctx, module, name),
        None => ModuleStaticLookupResult {
            ok: true,
            ..Default::default()
        },
    }
}

type NameTables = Rc<(NameMapTable, ModuleNames)>;

thread_local! {
    static NAME_TABLES: RefCell<Option<(u64, NameTables)>> = const { RefCell::new(None) };
}

/// Identifies the program by what the name maps depend on, so that the copies of the
/// context made per body share one computation.
fn sigma_fingerprint(ctx: &ScopeContext<'_>) -> u64 {
    let mut hasher = DefaultHasher::new();
    ctx.project
        .map(|project| project as *const _ as usize)
        .hash(&mut hasher);
    ctx.sigma.mods.len().hash(&mut hasher);
    for module in &ctx.sigma.mods {
        module.path.hash(&mut hasher);
        module.items.len().hash(&mut hasher);
    }
    hasher.finish()
}

/// The name maps and module names, computed from the context when the driver did not
/// hand them over, as the reference does.
fn name_tables_for_value_path(ctx: &ScopeContext<'_>) -> NameTables {
    if let Some(tables) = ctx.name_resolution_tables {
        if let (Some(name_maps), Some(module_names)) = (tables.name_maps, tables.module_names) {
            return Rc::new((name_maps.clone(), module_names.clone()));
        }
    }
    let fingerprint = sigma_fingerprint(ctx);
    if let Some(tables) = NAME_TABLES.with(|cell| {
        cell.borrow()
            .as_ref()
            .filter(|(key, _)| *key == fingerprint)
            .map(|(_, tables)| tables.clone())
    }) {
        return tables;
    }
    let mut scratch = ctx.clone();
    let name_maps = collect_name_maps(&mut scratch).name_maps;
    let module_names = match ctx.project {
        Some(project) => module_names_of(project),
        None => ctx
            .sigma
            .mods
            .iter()
            .map(|module| string_of_path(&module.path))
            .collect(),
    };
    let tables = Rc::new((name_maps, module_names));
    NAME_TABLES.with(|cell| *cell.borrow_mut() = Some((fingerprint, tables.clone())));
    tables
}

fn direct_module_lookup(
    ctx: &ScopeContext<'_>,
    path: &[String],
    name: &str,
) -> Option<ValuePathTypeResult> {
    let module_path: &[String] = if path.is_empty() {
        &ctx.current_module
    } else {
        path
    };
    let module = find_module(ctx, module_path)?;
    let static_lookup = lookup_module_static_in_module(ctx, module, name);
    if !static_lookup.ok {
        return Some(ValuePathTypeResult::failed(static_lookup.diag_id));
    }
    if static_lookup.r#type.is_some() {
        return Some(ValuePathTypeResult::typed(static_lookup.r#type));
    }
    find_procedure(module, name).map(|view| proc_type(ctx, &view))
}

/// The type of the value `path::name`: a static, a procedure, or a built-in procedure.
pub fn value_path_type(ctx: &ScopeContext<'_>, path: &[String], name: &str) -> ValuePathTypeResult {
    if let Some(builtin) = lookup_string_bytes_builtin_type(path, name) {
        return ValuePathTypeResult::typed(builtin);
    }
    if let Some((params, ret)) = lookup_builtin_modal_static_func_sig(path, name) {
        return ValuePathTypeResult::typed(make_type_func(params, ret));
    }
    let tables = name_tables_for_value_path(ctx);
    let (name_maps, module_names) = (&tables.0, &tables.1);
    let resolved = resolve_qualified(
        ctx,
        name_maps,
        module_names,
        path,
        name,
        EntityKind::Value,
        Some(can_access),
    );
    if !resolved.ok {
        if resolved.diag_id.is_none() {
            if let Some(direct) = direct_module_lookup(ctx, path, name) {
                return direct;
            }
            if path_eq(path, &ctx.current_module) {
                if let Some(gpu_intrinsic) = lookup_gpu_intrinsic_type(name) {
                    return ValuePathTypeResult::typed(gpu_intrinsic);
                }
            }
        }
        return ValuePathTypeResult::failed(resolved.diag_id);
    }
    let Some((origin, target)) = resolved.entity.as_ref().and_then(|entity| {
        entity
            .origin_opt
            .clone()
            .map(|origin| (origin, entity.target_opt.clone()))
    }) else {
        return direct_module_lookup(ctx, path, name).unwrap_or_else(ValuePathTypeResult::none);
    };
    let resolved_name = target.unwrap_or_else(|| name.to_string());
    let Some(module) = find_module(ctx, &origin) else {
        return ValuePathTypeResult::none();
    };
    let static_lookup = lookup_module_static_in_module(ctx, module, &resolved_name);
    if !static_lookup.ok {
        return ValuePathTypeResult::failed(static_lookup.diag_id);
    }
    if static_lookup.r#type.is_some() {
        return ValuePathTypeResult::typed(static_lookup.r#type);
    }
    match find_procedure(module, &resolved_name) {
        Some(view) => proc_type(ctx, &view),
        None => {
            if path_eq(path, &ctx.current_module) {
                if let Some(gpu_intrinsic) = lookup_gpu_intrinsic_type(name) {
                    return ValuePathTypeResult::typed(gpu_intrinsic);
                }
            }
            ValuePathTypeResult::none()
        }
    }
}
