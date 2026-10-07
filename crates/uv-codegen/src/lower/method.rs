//! Lowering: method calls (`LowerMethodCall`).
//!
//! Ported for receivers that are records, class defaults and states of modals declared in
//! the program. The built-in receivers (capabilities, regions, async values, strings, the
//! context, dynamic receivers) are reported as not ported.

use super::*;
use uv_analysis::composite::record_methods::lookup_method_static;
use uv_analysis::modal::lookup::{lookup_state_method_decl, lookup_transition_decl};
use uv_analysis::context::TypeDecl;

/// `UnwrapReceiverExpr`: the receiver without attributes and moves around it.
fn unwrap_receiver(expr: &Arc<Expr>) -> &Arc<Expr> {
    match &expr.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().map_or(expr, unwrap_receiver),
        ExprNode::MoveExpr(node) => node.place.as_ref().map_or(expr, unwrap_receiver),
        _ => expr,
    }
}

/// `ReceiverDispatchType`: the type of the receiver a method is looked up on. A modal state
/// the typing gave wins, then the type its root binding was declared with, then the typing.
fn receiver_dispatch_type(receiver: &Arc<Expr>, ctx: &LowerCtx) -> TypeRef {
    let binding_type = match &unwrap_receiver(receiver).node {
        ExprNode::IdentifierExpr(ident) => ctx.binding_state(&ident.name).and_then(|state| state.ty.clone()),
        _ => None,
    };
    let semantic = stored_expr_type(&ctx.scope, &Some(receiver.clone())).flatten();
    let is_modal_state = |ty: &TypeRef| matches!(strip_perm(ty).as_deref().map(|ty| &ty.node), Some(TypeNode::ModalState(_)));
    if is_modal_state(&semantic) {
        return semantic;
    }
    binding_type.or(semantic)
}

fn param_modes(params: &[ast::Param]) -> Vec<Option<ParamMode>> {
    params.iter().map(|param| param.mode.map(|_| ParamMode::Move)).collect()
}

fn param_types(params: &[ast::Param], ctx: &LowerCtx) -> Vec<TypeRef> {
    params
        .iter()
        .map(|param| match &param.r#type {
            Some(_) => lower_type(&ctx.scope, &param.r#type).ok().flatten_ref(),
            None => None,
        })
        .collect()
}

trait FlattenRef {
    fn flatten_ref(self) -> TypeRef;
}

impl FlattenRef for Option<TypeRef> {
    fn flatten_ref(self) -> TypeRef {
        self.flatten()
    }
}

/// `LowerMethodReturnType`: the declared return type, `()` when there is none.
fn method_return_type(written: &Option<Arc<ast::Type>>, ctx: &LowerCtx) -> TypeRef {
    match written {
        None => make_type_prim("()"),
        Some(written) => lower_type_for_layout(&ctx.scope, &Some(written.clone())).flatten().or_else(|| lower_type(&ctx.scope, &Some(written.clone())).ok().flatten()).or_else(|| make_type_prim("()")),
    }
}

/// `KeyModeForReceiverPerm`.
fn key_mode_for_receiver_perm(perm: Permission) -> ast::KeyMode {
    if perm == Permission::Const {
        ast::KeyMode::Read
    } else {
        ast::KeyMode::Write
    }
}

/// `LowerRefReceiverWithTemp`: a receiver that is a place is passed by its address, any other
/// value is bound to a temporary first.
fn lower_ref_receiver_with_temp(expr: &Arc<Expr>, expected: &TypeRef, ctx: &mut LowerCtx) -> LowerResult {
    if has_source_provenance(&Some(expr.clone())) {
        return lower_addr_of(expr, ctx);
    }
    let value_result = lower_expr(expr, ctx);
    let temp_name = ctx.fresh_temp_value("method_recv_tmp").name;
    let temp_type = expected
        .clone()
        .or_else(|| ctx.lookup_value_type(&value_result.value))
        .or_else(|| stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten())
        .or_else(|| make_type_prim("()"));
    let stable_name = ctx.register_var(&temp_name, temp_type.clone(), false, ProvenanceKind::Stack, None, None);
    let bind = Arc::new(Ir::BindVar { name: temp_name.clone(), stable_name, value: value_result.value.clone(), ty: temp_type, prov: ProvenanceKind::Stack, prov_region: None, prov_region_tag: None });
    let temp_ident = Arc::new(Expr { span: expr.span.clone(), node: ExprNode::IdentifierExpr(ast::IdentifierExpr { name: temp_name, from_splice: false }) });
    let addr_result = lower_addr_of(&temp_ident, ctx);
    LowerResult { ir: seq_ir(vec![Some(value_result.ir), Some(bind), Some(addr_result.ir)]), value: addr_result.value }
}

/// `LowerMethodCall`.
pub(super) fn lower_method_call(expr: &Arc<Expr>, call: &ast::MethodCallExpr, ctx: &mut LowerCtx) -> LowerResult {
    let failed = |ctx: &mut LowerCtx, what: &str| {
        ctx.unported(what);
        LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("method_call") }
    };
    let Some(receiver) = &call.receiver else {
        return failed(ctx, "method calls without a receiver");
    };
    let recv_type = receiver_dispatch_type(receiver, ctx);
    let stripped = strip_perm(&recv_type);
    let kind = stripped.as_ref().map(|ty| variant_name(&ty.node)).unwrap_or_default();
    let unsupported = |ctx: &mut LowerCtx| failed(ctx, &format!("method call {} on {kind}", call.name));
    let sigma = ctx.scope.sigma.clone();
    if id_eq(&call.name, "until") {
        return unsupported(ctx);
    }
    // What a method call on a built-in type is lowered to is not ported: only the types of
    // the program are.
    let method_expr_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if stripped.as_ref().is_some_and(|ty| async_sig_of(&ctx.scope, &Some(ty.clone())).is_some()) || method_expr_type.as_ref().is_some_and(|ty| async_sig_of(&ctx.scope, &Some(ty.clone())).is_some()) {
        return unsupported(ctx);
    }
    let Some(stripped_ty) = stripped.clone() else {
        return unsupported(ctx);
    };
    if uv_analysis::typing::type_predicates::lookup_foundational_builtin_method_sig(Some(&ctx.scope), &Some(stripped_ty.clone()), &call.name).is_some() {
        return unsupported(ctx);
    }

    let modes: Vec<Option<ParamMode>>;
    let types: Vec<TypeRef>;
    let mut move_receiver = false;
    let mut key_mode = ast::KeyMode::Read;
    let mut result_type: TypeRef = None;
    let callee_sym: String;
    match &stripped_ty.node {
        TypeNode::ModalState(modal) => {
            let Some(TypeDecl::Modal(decl)) = sigma.types.get(&path_key_of(&modal.path)) else {
                return unsupported(ctx);
            };
            if decl.generic_params.as_ref().is_some_and(|params| !params.params.is_empty()) {
                return failed(ctx, "method calls on states of generic modals");
            }
            let mut symbol_path = modal.path.clone();
            symbol_path.push(modal.state.clone());
            if let Some(method) = lookup_state_method_decl(decl, &modal.state, &call.name) {
                modes = param_modes(&method.params);
                types = param_types(&method.params, ctx);
                result_type = method_return_type(&method.return_type_opt, ctx);
                symbol_path.push(method.name.clone());
            } else if let Some(transition) = lookup_transition_decl(decl, &modal.state, &call.name) {
                modes = param_modes(&transition.params);
                types = param_types(&transition.params, ctx);
                move_receiver = true;
                key_mode = ast::KeyMode::Write;
                symbol_path.push(transition.name.clone());
            } else {
                return unsupported(ctx);
            }
            callee_sym = scoped_sym(&symbol_path);
        }
        TypeNode::Path { path, .. } => {
            if uv_analysis::caps::builtin_paths::is_context_type_path(path) {
                return unsupported(ctx);
            }
            let Ok(lookup) = lookup_method_static(&ctx.scope, &Some(stripped_ty.clone()), &call.name) else {
                return unsupported(ctx);
            };
            let receiver_key_mode = |receiver: &ast::Receiver, ctx: &LowerCtx| {
                recv_type_for_receiver(&Some(stripped_ty.clone()), receiver, |written| match lower_type_for_layout(&ctx.scope, written).flatten() {
                    Some(ty) => Ok(Some(ty)),
                    None => lower_type(&ctx.scope, written).map_err(|_| None),
                })
                .ok()
                .filter(Option::is_some)
                .map(|ty| key_mode_for_receiver_perm(perm_of_type(&ty)))
            };
            if let Some(method) = lookup.record_method {
                if lookup.record_path.is_empty() {
                    return unsupported(ctx);
                }
                modes = param_modes(&method.params);
                types = param_types(&method.params, ctx);
                result_type = method_return_type(&method.return_type_opt, ctx);
                if let Some(mode) = receiver_key_mode(&method.receiver, ctx) {
                    key_mode = mode;
                }
                callee_sym = scoped_sym(&item_path_proc(&lookup.record_path, &method.name));
            } else if let Some(method) = lookup.class_method.filter(|method| method.body_opt.is_some()) {
                modes = param_modes(&method.params);
                types = param_types(&method.params, ctx);
                result_type = method_return_type(&method.return_type_opt, ctx);
                if let Some(mode) = receiver_key_mode(&method.receiver, ctx) {
                    key_mode = mode;
                }
                callee_sym = proc::mangle_default_impl(&Some(stripped_ty.clone()), &lookup.owner_class, &method.name);
            } else {
                return unsupported(ctx);
            }
        }
        _ => return unsupported(ctx),
    }

    let recv_key_ir = lower_implicit_key_access(receiver, key_mode, ctx);
    let recv_result = if move_receiver {
        if matches!(receiver.node, ExprNode::MoveExpr(_)) {
            lower_expr(receiver, ctx)
        } else {
            lower_move_place(receiver, ctx)
        }
    } else if let ExprNode::MoveExpr(node) = &receiver.node {
        if node.place.as_ref().is_some_and(|place| is_place_expr(&Some(place.clone()))) {
            return failed(ctx, "method calls on a receiver that is moved by reference");
        }
        lower_ref_receiver_with_temp(receiver, &recv_type, ctx)
    } else {
        lower_ref_receiver_with_temp(receiver, &recv_type, ctx)
    };
    let (args_ir, arg_values) = lower_args(&modes, &types, &call.args, ctx);
    let mut args = vec![recv_result.value];
    args.extend(arg_values);

    let result_value = ctx.fresh_temp_value("method_call");
    let method_result_type = method_expr_type.or(result_type);
    if method_result_type.is_some() {
        ctx.register_value_type(&result_value, method_result_type);
    }
    let needs_panic_out = needs_panic_out_for_symbol(&callee_sym, ctx);
    if needs_panic_out {
        args.push(IrValue { kind: IrValueKind::Local, name: PANIC_OUT_NAME.to_string(), ..Default::default() });
    }
    let call_ir = Arc::new(Ir::Call { callee: IrValue { kind: IrValueKind::Symbol, name: callee_sym, ..Default::default() }, args, result: result_value.clone() });
    let mut parts = vec![Some(recv_key_ir), Some(recv_result.ir), Some(args_ir), Some(call_ir)];
    if needs_panic_out {
        parts.push(Some(panic_check(ctx)));
    }
    LowerResult { ir: seq_ir(parts), value: result_value }
}
