//! Lowering: statics.

use super::*;

/// `StaticName`: the one name a static binds, when it binds one.
pub(super) fn static_name(binding: &ast::Binding) -> Option<String> {
    match binding.pat.as_deref().map(|pat| &pat.node) {
        Some(ast::PatternNode::IdentifierPattern(pat)) => Some(pat.name.clone()),
        Some(ast::PatternNode::TypedPattern(pat)) if pat.name != "_" => Some(pat.name.clone()),
        _ => None,
    }
}

/// The scope the reference reads layouts in: the program and the module, no names.
pub(super) fn layout_scope<'b>(ctx: &LowerCtx<'_, 'b>, module_path: &[String]) -> ScopeContext<'b> {
    let mut scope = ctx.base.clone();
    scope.current_module = module_path.to_vec();
    scope.scopes.clear();
    scope.name_resolution_tables = None;
    scope
}

/// `StaticInitTypeForGlobal`: the written type of a static, or the type of its initializer.
pub(super) fn static_init_type(item: &ast::StaticDecl, module_path: &[String], ctx: &LowerCtx) -> TypeRef {
    let scope = layout_scope(ctx, module_path);
    let annotation = ast::binding_annotation_type_opt(&item.binding);
    if annotation.is_some() {
        return lower_type_for_layout(&scope, &annotation).flatten();
    }
    item.binding.init.as_ref().and_then(|init| stored_expr_type(&ctx.scope, &Some(init.clone())).flatten())
}

pub(super) fn is_unit_type_ref(ty: &TypeRef) -> bool {
    let mut current = ty.clone();
    loop {
        match current.as_deref().map(|ty| &ty.node) {
            Some(TypeNode::Prim(name)) => return name == "()",
            Some(TypeNode::Perm { base, .. }) | Some(TypeNode::Refine { base, .. }) => current = base.clone(),
            _ => return false,
        }
    }
}

/// `EmitGlobal`: the storage of a static; a constant initializer of an immutable static
/// is stored in the declaration, any other is set when the module starts.
pub(super) fn emit_global(item: &ast::StaticDecl, module_path: &[String], ctx: &mut LowerCtx) -> Vec<IrDecl> {
    let binding = &item.binding;
    let Some(name) = static_name(binding) else {
        ctx.unported("statics that bind several names");
        return Vec::new();
    };
    let symbol = scoped_sym(&item_path_proc(module_path, &name));
    let externally_visible = matches!(item.vis, ast::Visibility::Public | ast::Visibility::Internal);
    let export_from_shared_library = item.vis == ast::Visibility::Public;
    let init_type = static_init_type(item, module_path, ctx);
    if init_type.is_some() {
        ctx.static_types.insert(symbol.clone(), init_type.clone());
    }
    let scope = layout_scope(ctx, module_path);
    let (Some(size), Some(align)) = (size_of(&scope, &init_type), align_of(&scope, &init_type)) else {
        ctx.unported("statics whose layout is not known");
        return Vec::new();
    };
    if let (Some(init), ast::Mutability::Let) = (&binding.init, item.r#mut) {
        let bytes = match &init.node {
            ExprNode::LiteralExpr(lit) => encode_const(&init_type, &lit.literal),
            ExprNode::TupleExpr(tuple) if tuple.elements.is_empty() && is_unit_type_ref(&init_type) => Some(Vec::new()),
            _ => None,
        };
        if let Some(bytes) = bytes {
            return vec![IrDecl::GlobalConst(GlobalConst { symbol, bytes, align, externally_visible, export_from_shared_library })];
        }
    }
    vec![IrDecl::GlobalZero(GlobalZero { symbol, size, align, externally_visible, export_from_shared_library })]
}

/// `StaticHasResponsibility`.
pub(super) fn static_has_responsibility(item: &ast::StaticDecl) -> bool {
    match &item.binding.init {
        None => true,
        Some(init) => !is_place_expr(&Some(init.clone())) || matches!(init.node, ExprNode::MoveExpr(_)),
    }
}

/// `LowerStaticInitItem`: the initializer, the stores into the statics, the panic handling.
pub(super) fn lower_static_init_item(module_path: &[String], item: &ast::StaticDecl, ctx: &mut LowerCtx) -> IrPtr {
    let Some(init) = &item.binding.init else {
        return empty_ir();
    };
    let mut parts = Vec::new();
    let init_result = lower_expr(init, ctx);
    parts.push(Some(init_result.ir));
    // `PatternBindingValuesInOrder` and `StaticStoreIR`.
    match static_name(&item.binding) {
        Some(name) => {
            let symbol = scoped_sym(&item_path_proc(module_path, &name));
            parts.push(Some(Arc::new(Ir::StoreGlobal { symbol, value: init_result.value.clone() })));
            if static_has_responsibility(item) && type_needs_drop(&static_init_type(item, module_path, ctx), ctx) {
                ctx.unported("statics whose values need a drop");
            }
        }
        None => ctx.unported("statics that bind several names"),
    }
    let module = module_path.join("::");
    parts.push(Some(init_panic_handle(&module, ctx)));
    seq_ir(parts)
}

/// `LowerStaticInit`.
pub(super) fn lower_static_init(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> IrPtr {
    let items: Vec<&ast::StaticDecl> = module.items.iter().filter_map(|item| if let ASTItem::StaticDecl(decl) = item { Some(decl) } else { None }).collect();
    if items.is_empty() {
        return empty_ir();
    }
    let parts = items.into_iter().map(|item| Some(lower_static_init_item(module_path, item, ctx))).collect();
    seq_ir(parts)
}

/// `LowerStaticDeinit`: the statics are dropped in the reverse order of their declaration.
pub(super) fn lower_static_deinit(module_path: &[String], module: &ASTModule, ctx: &mut LowerCtx) -> IrPtr {
    let items: Vec<&ast::StaticDecl> = module.items.iter().filter_map(|item| if let ASTItem::StaticDecl(decl) = item { Some(decl) } else { None }).collect();
    if items.is_empty() {
        return empty_ir();
    }
    let mut item_parts = Vec::new();
    for item in items.into_iter().rev() {
        let mut drops = Vec::new();
        if let Some(name) = static_name(&item.binding) {
            if static_has_responsibility(item) {
                let ty = static_init_type(item, module_path, ctx);
                if type_needs_drop(&ty, ctx) {
                    ctx.unported("drops of statics of this type");
                }
                let _ = name;
                drops.push(Some(empty_ir()));
            }
        } else {
            ctx.unported("statics that bind several names");
        }
        item_parts.push(Some(if drops.is_empty() { empty_ir() } else { seq_ir(drops) }));
    }
    seq_ir(item_parts)
}
