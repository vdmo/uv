//! Lowering: expr.

use super::*;
use uv_analysis::layout::value_bits::decode_string_literal_bytes;

const INT_SUFFIXES: [&str; 12] = ["i128", "u128", "isize", "usize", "i64", "u64", "i32", "u32", "i16", "u16", "i8", "u8"];

fn strip_int_suffix(text: &str) -> &str {
    INT_SUFFIXES.iter().find_map(|suffix| text.strip_suffix(suffix)).unwrap_or(text)
}

fn match_int_suffix(text: &str) -> Option<&'static str> {
    INT_SUFFIXES.iter().find(|suffix| text.len() > suffix.len() && text.ends_with(*suffix)).copied()
}

/// `UniquePrimitiveUnionMember`: the one member of a union that is the primitive named.
fn unique_primitive_union_member(contextual: &TypeRef, primitive: &str) -> TypeRef {
    let stripped = strip_perm(contextual);
    let Some(TypeNode::Union(members)) = stripped.as_deref().map(|ty| &ty.node) else {
        return None;
    };
    let mut found: TypeRef = None;
    for member in members {
        let member_stripped = strip_perm(member);
        if !matches!(member_stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == primitive) {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = member.clone();
    }
    found
}

/// `ParseIntLiteralLexeme`: the value of an integer literal, with its prefix and underscores.
fn parse_int_literal_lexeme(lexeme: &str) -> Option<u64> {
    let text = strip_int_suffix(lexeme);
    let digits = |body: &str, radix: u32| -> Option<u64> {
        if body.is_empty() {
            return None;
        }
        let mut out = 0u64;
        for c in body.chars() {
            if c == '_' {
                continue;
            }
            let digit = c.to_digit(radix)?;
            out = out.wrapping_mul(u64::from(radix)).wrapping_add(u64::from(digit));
        }
        Some(out)
    };
    let lower = text.as_bytes();
    if lower.len() >= 2 && lower[0] == b'0' {
        match lower[1] {
            b'b' | b'B' => return digits(&text[2..], 2),
            b'o' | b'O' => return digits(&text[2..], 8),
            b'x' | b'X' => return digits(&text[2..], 16),
            _ => {}
        }
    }
    let cleaned: String = text.chars().filter(|c| *c != '_').collect();
    if cleaned.is_empty() {
        return None;
    }
    cleaned.parse::<u64>().ok()
}

/// `EncodeU64LE`: little-endian bytes without the zeros at the top, and one byte for zero.
pub(super) fn encode_u64_le(mut value: u64) -> Vec<u8> {
    if value == 0 {
        return vec![0];
    }
    let mut bytes = Vec::new();
    while value > 0 {
        bytes.push((value & 0xFF) as u8);
        value >>= 8;
    }
    bytes
}

/// `LowerLiteral`: an immediate value; a literal has no IR of its own.
pub(super) fn lower_literal(expr: &Arc<Expr>, lit: &ast::LiteralExpr, ctx: &mut LowerCtx) -> LowerResult {
    let literal_kind = match lit.literal.kind {
        TokenKind::StringLiteral => Some(IrImmediateLiteralKind::String),
        TokenKind::CharLiteral => Some(IrImmediateLiteralKind::Char),
        TokenKind::IntLiteral => Some(IrImmediateLiteralKind::Int),
        TokenKind::FloatLiteral => Some(IrImmediateLiteralKind::Float),
        _ => None,
    };
    let mut value = IrValue { kind: IrValueKind::Immediate, name: lit.literal.lexeme.clone(), literal_id: ctx.next_literal_id(), literal_kind, ..Default::default() };
    // A string literal is its text with the escapes decoded.
    if lit.literal.kind == TokenKind::StringLiteral {
        match decode_string_literal_bytes(&lit.literal.lexeme) {
            Some(bytes) => value.bytes = bytes,
            None => ctx.unported("string literals that do not decode"),
        }
    }
    // With the type typing gave it, the literal is encoded as a value of that type; inside a
    // union, an integer with a suffix is encoded as the member that suffix names.
    let mut lit_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if lit.literal.kind == TokenKind::IntLiteral {
        if let Some(suffix) = match_int_suffix(&lit.literal.lexeme) {
            if let Some(source) = unique_primitive_union_member(&lit_type, suffix) {
                lit_type = Some(source);
            }
        }
    }
    if lit_type.is_some() {
        if let Some(bytes) = encode_const(&lit_type, &lit.literal) {
            value.bytes = bytes;
        }
        ctx.register_value_type(&value, lit_type);
    }
    // Without bytes from the type: integers by their digits, floats as `f32` unless the suffix
    // says otherwise, booleans and null by their words.
    if value.bytes.is_empty() && lit.literal.kind == TokenKind::IntLiteral {
        if let Some(parsed) = parse_int_literal_lexeme(&lit.literal.lexeme) {
            value.bytes = encode_u64_le(parsed);
        }
    } else if value.bytes.is_empty() && lit.literal.kind == TokenKind::FloatLiteral {
        let lexeme = lit.literal.lexeme.as_str();
        let fallback = if lexeme.ends_with("f16") {
            "f16"
        } else if lexeme.ends_with("f64") {
            "f64"
        } else {
            "f32"
        };
        let fallback_type = make_type_prim(fallback);
        if let Some(bytes) = encode_const(&fallback_type, &lit.literal) {
            value.bytes = bytes;
            ctx.register_value_type(&value, fallback_type);
        }
    } else if value.bytes.is_empty() && lit.literal.kind == TokenKind::BoolLiteral {
        value.bytes = vec![u8::from(lit.literal.lexeme == "true")];
    } else if value.bytes.is_empty() && lit.literal.kind == TokenKind::NullLiteral {
        value.bytes = vec![0];
    }
    LowerResult { ir: empty_ir(), value }
}

pub(super) fn needs_refinement_check(expr: &Arc<Expr>, ctx: &LowerCtx) -> bool {
    let key = Arc::as_ptr(expr) as usize;
    ctx.scope.stores.as_ref().is_some_and(|stores| stores.dynamic_refine_checks.borrow().contains_key(&key))
}

/// `resolve_name` of the reference's driver: the path of a value entity of the module.
pub(super) fn resolve_value_path(name: &str, ctx: &LowerCtx) -> Option<Vec<String>> {
    let entity = ctx.scope.module_scope().get(&id_key_of(name))?;
    if entity.kind != EntityKind::Value {
        return None;
    }
    let resolved = entity.target_opt.clone().unwrap_or_else(|| name.to_string());
    let mut full = entity.origin_opt.clone()?;
    full.push(resolved);
    Some(full)
}

/// `LowerStaticIdentifierRead`: a name of the program that is not a local, read through its path.
pub(super) fn lower_static_identifier_read(expr: &Arc<Expr>, ident: &ast::IdentifierExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(mut full) = resolve_value_path(&ident.name, ctx) else {
        ctx.unported("reads of names that do not resolve to a path");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") };
    };
    let name = full.pop().unwrap_or_default();
    let value = IrValue { kind: IrValueKind::Symbol, name: name.clone(), ..Default::default() };
    let expr_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    ctx.register_value_type(&value, expr_type);
    let key_ir = lower_implicit_key_access(expr, ast::KeyMode::Read, ctx);
    LowerResult { ir: seq_ir(vec![Some(key_ir), Some(Arc::new(Ir::ReadPath { path: full, name }))]), value }
}

/// `LowerIdentifier` for a name bound in the procedure: `ReadVar` of its stable name.
pub(super) fn lower_identifier(expr: &Arc<Expr>, ident: &ast::IdentifierExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(state) = ctx.binding_state(&ident.name).cloned() else {
        return lower_static_identifier_read(expr, ident, ctx);
    };
    let ir_name = if state.stable_name.is_empty() { ident.name.clone() } else { state.stable_name.clone() };
    let value = IrValue { kind: IrValueKind::Local, name: ir_name.clone(), ..Default::default() };
    let expr_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if state.ty.is_some() {
        ctx.register_value_type(&value, state.ty.clone());
    } else {
        ctx.register_value_type(&value, expr_type.clone());
    }
    let key_ir = lower_implicit_key_access(expr, ast::KeyMode::Read, ctx);
    LowerResult { ir: seq_ir(vec![Some(key_ir), Some(Arc::new(Ir::ReadVar { name: ir_name }))]), value }
}

pub(super) fn prim_name(ty: &TypeRef) -> Option<String> {
    match strip_perm(ty).or_else(|| ty.clone()).as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Prim(name)) => Some(name.clone()),
        _ => None,
    }
}

pub(super) fn is_integer_operand(ty: &TypeRef) -> bool {
    prim_name(ty).is_some_and(|name| matches!(name.as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"))
}

pub(super) fn is_float_operand(ty: &TypeRef) -> bool {
    prim_name(ty).is_some_and(|name| matches!(name.as_str(), "f16" | "f32" | "f64"))
}

pub(super) fn needs_binary_panic_check(op: &str, lhs: &TypeRef, rhs: &TypeRef) -> bool {
    if op == "<<" || op == ">>" {
        return true;
    }
    if matches!(op, "/" | "%" | "+" | "-" | "*" | "**") {
        if is_float_operand(lhs) || is_float_operand(rhs) {
            return false;
        }
        return is_integer_operand(lhs) || is_integer_operand(rhs) || lhs.is_none() || rhs.is_none();
    }
    false
}

pub(super) fn bool_immediate(value: bool) -> IrValue {
    IrValue { kind: IrValueKind::Immediate, name: if value { "true" } else { "false" }.to_string(), bytes: vec![u8::from(value)], ..Default::default() }
}

/// The operands of a chain of one short-circuit operator, left to right.
pub(super) fn collect_short_circuit_operands(expr: &Arc<Expr>, op: &str, out: &mut Vec<Arc<Expr>>) {
    let mut stack = vec![expr.clone()];
    while let Some(current) = stack.pop() {
        if let ExprNode::BinaryExpr(binary) = &current.node {
            if binary.op == op {
                if let (Some(lhs), Some(rhs)) = (&binary.lhs, &binary.rhs) {
                    stack.push(rhs.clone());
                    stack.push(lhs.clone());
                    continue;
                }
            }
        }
        out.push(current);
    }
}

/// `LowerShortCircuitChain`: `a && b && c` as nested conditionals; each later operand is
/// lowered in a copy of the context.
pub(super) fn lower_short_circuit_chain(op: &str, operands: &[Arc<Expr>], ctx: &mut LowerCtx) -> LowerResult {
    let Some(first) = operands.first() else {
        return LowerResult { ir: empty_ir(), value: bool_immediate(op == "&&") };
    };
    let first_result = lower_expr(first, ctx);
    let mut parts = vec![Some(first_result.ir)];
    let mut current = first_result.value;
    for operand in &operands[1..] {
        let snapshot = ctx.begin_branch();
        let rhs = lower_expr(operand, ctx);
        ctx.end_branch(snapshot);
        let result_value = ctx.fresh_temp_value(if op == "&&" { "and" } else { "or" });
        let (then_ir, then_value, else_ir, else_value) = if op == "&&" { (rhs.ir, rhs.value, empty_ir(), bool_immediate(false)) } else { (empty_ir(), bool_immediate(true), rhs.ir, rhs.value) };
        parts.push(Some(Arc::new(Ir::If { cond: current, then_ir: Some(then_ir), then_value, else_ir: Some(else_ir), else_value, result: result_value.clone() })));
        ctx.register_value_type(&result_value, make_type_prim("bool"));
        current = result_value;
    }
    LowerResult { ir: seq_ir(parts), value: current }
}

/// Lowers an expression into a branch of the context, with temporaries of its own.
pub(super) fn lower_in_branch(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    let snapshot = ctx.begin_branch();
    let prev_sink = ctx.temp_sink.replace(Vec::new());
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let mut result = lower_expr(expr, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = prev_sink;
    let cleanup = cleanup_temps_ir(&temps, ctx);
    if !is_noop_ir(&cleanup) && ir_flow_may_fall_through(&Some(result.ir.clone())) {
        result.ir = seq_ir(vec![Some(result.ir), Some(cleanup)]);
    }
    ctx.end_branch(snapshot);
    result
}

/// `LowerIfExpr`: the condition, then each branch in a copy of the context.
pub(super) fn lower_if_expr(expr: &Arc<Expr>, node: &ast::IfExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(cond), Some(then_expr)) = (&node.cond, &node.then_expr) else {
        ctx.unported("conditionals without a condition or a branch");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("if") };
    };
    let prev_sink = ctx.temp_sink.replace(Vec::new());
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let cond_result = lower_expr(cond, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let cond_temps = ctx.temp_sink.take().unwrap_or_default();
    ctx.temp_sink = prev_sink;
    let mut cond_cleanup = cleanup_temps_ir(&cond_temps, ctx);
    if is_noop_ir(&cond_cleanup) {
        cond_cleanup = empty_ir();
    }
    let then_result = lower_in_branch(then_expr, ctx);
    let else_result = match &node.else_expr {
        Some(else_expr) => lower_in_branch(else_expr, ctx),
        None => {
            let value = ctx.fresh_temp_value("unit");
            ctx.register_value_type(&value, make_type_prim("()"));
            LowerResult { ir: empty_ir(), value }
        }
    };
    let result_value = ctx.fresh_temp_value("if");
    let mut result_type = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if result_type.is_none() && node.else_expr.is_none() {
        result_type = make_type_prim("()");
    }
    if result_type.is_some() {
        ctx.register_value_type(&result_value, result_type);
    }
    let if_ir = Arc::new(Ir::If { cond: cond_result.value, then_ir: Some(then_result.ir), then_value: then_result.value, else_ir: Some(else_result.ir), else_value: else_result.value, result: result_value.clone() });
    LowerResult { ir: seq_ir(vec![Some(cond_result.ir), Some(cond_cleanup), Some(if_ir)]), value: result_value }
}

/// `LowerList`: the expressions left to right, each kept out of the temporaries of the statement.
pub(super) fn lower_list(exprs: &[Option<Arc<Expr>>], ctx: &mut LowerCtx) -> (IrPtr, Vec<IrValue>) {
    if exprs.is_empty() {
        return (empty_ir(), Vec::new());
    }
    let mut parts = Vec::new();
    let mut values = Vec::new();
    for expr in exprs {
        let Some(expr) = expr else {
            ctx.unported("lists with a missing expression");
            continue;
        };
        let prev_suppress = ctx.suppress_temp_at_depth;
        ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
        let result = lower_expr(expr, ctx);
        ctx.suppress_temp_at_depth = prev_suppress;
        parts.push(Some(result.ir));
        values.push(result.value);
    }
    (seq_ir(parts), values)
}

/// `LowerTuple`: the elements, and a value the emitter builds from them.
pub(super) fn lower_tuple_expr(expr: &Arc<Expr>, node: &ast::TupleExpr, ctx: &mut LowerCtx) -> LowerResult {
    if node.elements.is_empty() {
        let unit_value = ctx.fresh_temp_value("unit");
        ctx.register_value_type(&unit_value, make_type_prim("()"));
        return LowerResult { ir: empty_ir(), value: unit_value };
    }
    let contextual = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    let (ir, values) = lower_list(&node.elements, ctx);
    let tuple_value = ctx.fresh_temp_value("tuple");
    ctx.register_derived_value(&tuple_value, DerivedValueInfo { elements: values.clone(), ..DerivedValueInfo::new(DerivedKind::TupleLit) });
    let mut tuple_type: TypeRef = None;
    let stripped = strip_perm(&contextual).or(contextual);
    if let Some(TypeNode::Tuple(elements)) = stripped.as_deref().map(|ty| &ty.node) {
        if elements.len() == values.len() {
            tuple_type = stripped.clone();
            for (value, element) in values.iter().zip(elements) {
                ctx.register_value_type(value, element.clone());
            }
        }
    }
    if tuple_type.is_none() {
        let element_types: Vec<TypeRef> = values.iter().map(|value| ctx.lookup_value_type(value)).collect();
        if element_types.iter().all(Option::is_some) {
            tuple_type = make_type_tuple(element_types);
        }
    }
    if tuple_type.is_some() {
        ctx.register_value_type(&tuple_value, tuple_type);
    }
    LowerResult { ir, value: tuple_value }
}

pub(super) fn is_signed_integer_operand(ty: &TypeRef) -> bool {
    prim_name(ty).is_some_and(|name| matches!(name.as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize"))
}

/// `LowerUnOp`: the operand, the check that negating can overflow, the operation.
pub(super) fn lower_unary_expr(node: &ast::UnaryExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(operand) = &node.value else {
        ctx.unported("unary expressions without an operand");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unop") };
    };
    let op = node.op.as_str();
    let prev_suppress = ctx.suppress_temp_at_depth;
    ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
    let operand_result = lower_expr(operand, ctx);
    ctx.suppress_temp_at_depth = prev_suppress;
    let operand_type = ctx.lookup_value_type(&operand_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(operand.clone())).flatten());
    if operand_type.is_some() {
        ctx.register_value_type(&operand_result.value, operand_type.clone());
    }
    let result_value = ctx.fresh_temp_value("unop");
    let stripped = strip_perm(&operand_type).or_else(|| operand_type.clone());
    let result_type = match op {
        "widen" => {
            ctx.unported("the operator widen");
            None
        }
        "!" | "-" | "~" => stripped,
        _ => None,
    };
    let mut parts = vec![Some(operand_result.ir)];
    if op == "-" && is_signed_integer_operand(&operand_type) {
        parts.push(Some(Arc::new(Ir::CheckOp { op: op.to_string(), reason: "Overflow".to_string(), lhs: operand_result.value.clone(), rhs: None })));
        parts.push(Some(panic_check(ctx)));
    }
    parts.push(Some(Arc::new(Ir::UnaryOp { op: op.to_string(), operand: operand_result.value.clone(), result: result_value.clone(), operand_type, result_type: result_type.clone() })));
    if result_type.is_some() {
        ctx.register_value_type(&result_value, result_type);
    }
    LowerResult { ir: seq_ir(parts), value: result_value }
}

/// `Lower-Expr-Sizeof` and `Lower-Expr-Alignof`: the size or the alignment of a type, as a constant.
pub(super) fn lower_layout_constant(written: &Option<Arc<ast::Type>>, align: bool, ctx: &mut LowerCtx) -> LowerResult {
    let lowered = lower_type_for_layout(&ctx.scope, written).flatten();
    let Some(layout) = layout_of(&ctx.scope, &lowered) else {
        ctx.unported("sizes of types whose layout is not known");
        return LowerResult { ir: empty_ir(), value: IrValue::default() };
    };
    let number = if align { layout.align } else { layout.size };
    let value = IrValue { kind: IrValueKind::Immediate, name: number.to_string(), bytes: encode_u64_le(number), ..Default::default() };
    LowerResult { ir: empty_ir(), value }
}

/// `LowerArrayLiteral`: the segments left to right, and a value the emitter builds from them.
pub(super) fn lower_array_expr(node: &ast::ArrayExpr, ctx: &mut LowerCtx) -> LowerResult {
    let mut parts = Vec::new();
    let mut segments = Vec::new();
    for segment in &node.elements {
        let prev_suppress = ctx.suppress_temp_at_depth;
        ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
        match segment {
            ast::ArraySegment::ArrayElemSegment(element) => {
                let Some(value) = &element.value else {
                    ctx.unported("array elements without a value");
                    continue;
                };
                let result = lower_expr(value, ctx);
                parts.push(Some(result.ir));
                segments.push(DerivedArraySegment { repeat: false, value: result.value, count: IrValue::default() });
            }
            ast::ArraySegment::ArrayRepeatSegment(repeat) => {
                let (Some(value), Some(count)) = (&repeat.value, &repeat.count) else {
                    ctx.unported("repeated array elements without a value");
                    continue;
                };
                let value_result = lower_expr(value, ctx);
                let count_result = lower_expr(count, ctx);
                parts.push(Some(value_result.ir));
                parts.push(Some(count_result.ir));
                segments.push(DerivedArraySegment { repeat: true, value: value_result.value, count: count_result.value });
            }
        }
        ctx.suppress_temp_at_depth = prev_suppress;
    }
    let ir = if segments.is_empty() { empty_ir() } else { seq_ir(parts) };
    let array_value = ctx.fresh_temp_value("array");
    // The concrete array type, when every element has one and they agree.
    let mut element_type: TypeRef = None;
    let mut homogeneous = true;
    let mut element_count = 0u64;
    for segment in &segments {
        let Some(current) = ctx.lookup_value_type(&segment.value) else {
            homogeneous = false;
            break;
        };
        match &element_type {
            None => element_type = Some(current),
            Some(first) => {
                if !type_equiv(&Some(first.clone()), &Some(current)) {
                    homogeneous = false;
                    break;
                }
            }
        }
        if segment.repeat {
            homogeneous = false;
            break;
        }
        element_count += 1;
    }
    ctx.register_derived_value(&array_value, DerivedValueInfo { array_segments: segments, ..DerivedValueInfo::new(DerivedKind::ArraySegments) });
    if homogeneous && element_type.is_some() {
        ctx.register_value_type(&array_value, make_type_array(element_type, element_count, None));
    }
    LowerResult { ir, value: array_value }
}

/// `LowerArrayRepeat`: `[value; count]` as one repeated segment.
pub(super) fn lower_array_repeat_expr(node: &ast::ArrayRepeatExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (Some(value), Some(count)) = (&node.value, &node.count) else {
        ctx.unported("repeated arrays without a value");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("array_repeat") };
    };
    let value_result = lower_expr(value, ctx);
    let count_result = lower_expr(count, ctx);
    let array_value = ctx.fresh_temp_value("array_repeat");
    let segment = DerivedArraySegment { repeat: true, value: value_result.value, count: count_result.value };
    ctx.register_derived_value(&array_value, DerivedValueInfo { array_segments: vec![segment], ..DerivedValueInfo::new(DerivedKind::ArraySegments) });
    LowerResult { ir: seq_ir(vec![Some(value_result.ir), Some(count_result.ir)]), value: array_value }
}

/// `LowerReadPlaceFieldAccess`: the base is lowered as an expression and the field is read from it.
pub(super) fn lower_field_access_expr(expr: &Arc<Expr>, node: &ast::FieldAccessExpr, ctx: &mut LowerCtx) -> LowerResult {
    let Some(base) = &node.base else {
        ctx.unported("field accesses without a base");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("place_field") };
    };
    let base_result = lower_expr(base, ctx);
    let field_value = ctx.fresh_temp_value("place_field");
    let mut type_of_field = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    if type_of_field.is_none() {
        // `InferRecordFieldTypeFromBase`.
        let base_type = ctx.lookup_value_type(&base_result.value);
        let mut stripped = base_type;
        while let Some(TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. }) = stripped.as_deref().map(|ty| &ty.node) {
            stripped = base.clone();
        }
        if let Some(ty) = &stripped {
            if let Some(path) = applied_type_path(ty) {
                if let Some(record) = lookup_record_decl(&ctx.scope, path) {
                    let args: Vec<TypeRef> = applied_type_args(ty).map(<[TypeRef]>::to_vec).unwrap_or_default();
                    type_of_field = field_type(record, &node.name, &ctx.scope, &args).flatten();
                }
            }
        }
    }
    if type_of_field.is_some() {
        ctx.register_value_type(&field_value, type_of_field.clone());
    }
    let mut info = DerivedValueInfo::new(DerivedKind::Field);
    info.base = base_result.value.clone();
    info.field = node.name.clone();
    ctx.register_derived_value(&field_value, info);
    let key_ir = lower_implicit_key_access(expr, ast::KeyMode::Read, ctx);
    LowerResult { ir: seq_ir(vec![Some(base_result.ir), Some(key_ir)]), value: field_value }
}

/// `LowerReadPlaceTupleAccess` of a local: the base is read by its name in the source.
pub(super) fn lower_tuple_access_expr(expr: &Arc<Expr>, node: &ast::TupleAccessExpr, ctx: &mut LowerCtx) -> LowerResult {
    let base_ident = node.base.as_ref().and_then(|base| match &base.node {
        ExprNode::IdentifierExpr(ident) => Some((base.clone(), ident.clone())),
        _ => None,
    });
    let Some((base, ident)) = base_ident else {
        ctx.unported("tuple accesses on bases that are not local names");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("place_tuple_elem") };
    };
    let Some(state) = ctx.binding_state(&ident.name).cloned() else {
        ctx.unported("tuple accesses on names that are not local");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("place_tuple_elem") };
    };
    let base_value = IrValue { kind: IrValueKind::Local, name: ident.name.clone(), ..Default::default() };
    let base_type = stored_expr_type(&ctx.scope, &Some(base.clone())).flatten();
    ctx.register_value_type(&base_value, if state.ty.is_some() { state.ty.clone() } else { base_type });
    let key_ir = lower_implicit_key_access(&base, ast::KeyMode::Read, ctx);
    let base_ir = seq_ir(vec![Some(key_ir), Some(Arc::new(Ir::ReadVar { name: ident.name.clone() }))]);
    let elem_value = ctx.fresh_temp_value("place_tuple_elem");
    ctx.register_value_type(&elem_value, stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten());
    let mut info = DerivedValueInfo::new(DerivedKind::Tuple);
    info.base = base_value;
    info.tuple_index = usize::try_from(node.index).unwrap_or(usize::MAX);
    ctx.register_derived_value(&elem_value, info);
    LowerResult { ir: base_ir, value: elem_value }
}

pub(super) fn is_bool_binop(op: &str) -> bool {
    matches!(op, "==" | "===" | "!=" | "<" | "<=" | ">" | ">=" | "&&" | "||")
}

/// `LowerBinOp`: both operands left to right, the check that can panic, the operation.
pub(super) fn lower_binary_expr(node: &ast::BinaryExpr, ctx: &mut LowerCtx) -> LowerResult {
    let op = node.op.as_str();
    let (Some(lhs), Some(rhs)) = (&node.lhs, &node.rhs) else {
        ctx.unported("binary expressions without operands");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("binop") };
    };
    if matches!(op, "&&" | "||") {
        let mut operands = Vec::new();
        collect_short_circuit_operands(lhs, op, &mut operands);
        collect_short_circuit_operands(rhs, op, &mut operands);
        return lower_short_circuit_chain(op, &operands, ctx);
    }
    if op == "<:" {
        ctx.unported("the operator <:");
        return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("binop") };
    }
    let lhs_result = lower_expr(lhs, ctx);
    let rhs_result = lower_expr(rhs, ctx);
    let result_value = ctx.fresh_temp_value("binop");
    let mut parts = vec![Some(lhs_result.ir), Some(rhs_result.ir)];
    let lhs_type = ctx.lookup_value_type(&lhs_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(lhs.clone())).flatten());
    let rhs_type = ctx.lookup_value_type(&rhs_result.value).or_else(|| stored_expr_type(&ctx.scope, &Some(rhs.clone())).flatten());
    if needs_binary_panic_check(op, &lhs_type, &rhs_type) {
        let reason = match op {
            "/" | "%" => "DivZero",
            "<<" | ">>" => "Shift",
            _ => "Overflow",
        };
        parts.push(Some(Arc::new(Ir::CheckOp { op: op.to_string(), reason: reason.to_string(), lhs: lhs_result.value.clone(), rhs: Some(rhs_result.value.clone()) })));
        parts.push(Some(panic_check(ctx)));
    }
    parts.push(Some(Arc::new(Ir::BinaryOp { op: op.to_string(), lhs: lhs_result.value.clone(), rhs: rhs_result.value.clone(), result: result_value.clone() })));
    if is_bool_binop(op) {
        ctx.register_value_type(&result_value, make_type_prim("bool"));
    } else if let Some(ty) = ctx.lookup_value_type(&lhs_result.value).or_else(|| ctx.lookup_value_type(&rhs_result.value)) {
        ctx.register_value_type(&result_value, Some(ty));
    }
    LowerResult { ir: seq_ir(parts), value: result_value }
}

/// `LowerFieldInits`: the initializers left to right, each kept out of the temporaries of the statement.
pub(super) fn lower_field_inits(fields: &[ast::FieldInit], ctx: &mut LowerCtx) -> (IrPtr, Vec<(String, IrValue)>) {
    if fields.is_empty() {
        return (empty_ir(), Vec::new());
    }
    let mut parts = Vec::new();
    let mut values = Vec::new();
    for field in fields {
        let Some(value) = &field.value else {
            ctx.unported("field initializers without a value");
            continue;
        };
        let prev_suppress = ctx.suppress_temp_at_depth;
        ctx.suppress_temp_at_depth = Some(ctx.temp_depth + 1);
        let result = lower_expr(value, ctx);
        ctx.suppress_temp_at_depth = prev_suppress;
        parts.push(Some(result.ir));
        values.push((field.name.clone(), result.value));
    }
    (seq_ir(parts), values)
}

/// `LowerRecord`: the value of a record expression is built by the emitter from its fields.
pub(super) fn lower_record_expr(expr: &Arc<Expr>, node: &ast::RecordExpr, ctx: &mut LowerCtx) -> LowerResult {
    let (ir, field_values) = lower_field_inits(&node.fields, ctx);
    // The type typing gave the expression, when it names the target; else the target written.
    let typed = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten();
    let typed = strip_perm(&typed).or(typed);
    let checked = typed.as_ref().and_then(|typed| match (&node.target, &typed.node) {
        (ast::RecordExprTarget::Path(target), _) => applied_type_path(typed).filter(|path| *path == target).map(|_| Some(typed.clone())),
        (ast::RecordExprTarget::ModalStateRef(target), TypeNode::ModalState(modal)) => (modal.path == target.path && modal.state == target.state).then(|| Some(typed.clone())),
        _ => None,
    });
    let record_type = match checked.flatten() {
        Some(ty) => Some(ty),
        None => match &node.target {
            ast::RecordExprTarget::Path(path) => make_type_path(path.clone()),
            ast::RecordExprTarget::ModalStateRef(target) => {
                let mut args = Vec::new();
                for arg in &target.generic_args {
                    match lower_type(&ctx.scope, arg) {
                        Ok(ty) if ty.is_some() => args.push(ty),
                        _ => {
                            ctx.unported("record targets whose type arguments do not lower");
                            break;
                        }
                    }
                }
                make_type_modal_state(target.path.clone(), &target.state, args)
            }
        },
    };
    // A field of an Outcome type takes a bare initializer as a value of it.
    let mut outcome_field = false;
    if let Some(ty) = &record_type {
        let stripped = strip_perm(&record_type).unwrap_or_else(|| ty.clone());
        if let Some(path) = applied_type_path(&stripped) {
            if let Some(record) = lookup_record_decl(&ctx.scope, path) {
                let args: Vec<TypeRef> = applied_type_args(&stripped).map(<[TypeRef]>::to_vec).unwrap_or_default();
                outcome_field = field_values.iter().any(|(name, _)| field_type(record, name, &ctx.scope, &args).is_some_and(|field| outcome_sig_of(&field).is_some()));
            }
        }
    }
    if outcome_field {
        ctx.unported("record fields of an Outcome type");
    }
    let record_value = ctx.fresh_temp_value("record");
    ctx.register_derived_value(&record_value, DerivedValueInfo { fields: field_values, ..DerivedValueInfo::new(DerivedKind::RecordLit) });
    if record_type.is_some() {
        ctx.register_value_type(&record_value, record_type);
    }
    LowerResult { ir, value: record_value }
}

/// `FieldHead`: the first field a place goes through.
pub(super) fn field_head(expr: &Arc<Expr>) -> Option<String> {
    match &expr.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().and_then(field_head),
        ExprNode::IdentifierExpr(_) | ExprNode::DerefExpr(_) => None,
        ExprNode::FieldAccessExpr(node) => node.base.as_ref().and_then(field_head).or_else(|| Some(node.name.clone())),
        ExprNode::TupleAccessExpr(node) => node.base.as_ref().and_then(field_head),
        ExprNode::IndexAccessExpr(node) => node.base.as_ref().and_then(field_head),
        _ => None,
    }
}

/// `BuildPlaceRepr`: the text of a place.
pub(super) fn build_place_repr(expr: &Arc<Expr>) -> String {
    match &expr.node {
        ExprNode::AttributedExpr(node) => node.expr.as_ref().map(build_place_repr).unwrap_or_default(),
        ExprNode::IdentifierExpr(node) => node.name.clone(),
        ExprNode::FieldAccessExpr(node) => {
            let base = node.base.as_ref().map(build_place_repr).unwrap_or_default();
            if base.is_empty() { node.name.clone() } else { format!("{base}.{}", node.name) }
        }
        ExprNode::TupleAccessExpr(node) => {
            let base = node.base.as_ref().map(build_place_repr).unwrap_or_default();
            let index = uv_analysis::keys::key_paths::format_tuple_index(node.index);
            if base.is_empty() { index } else { format!("{base}.{index}") }
        }
        ExprNode::DerefExpr(node) => format!("*{}", node.value.as_ref().map(build_place_repr).unwrap_or_default()),
        _ => String::new(),
    }
}

/// `LowerReadPlace`: a place read as a value. A local is read by the name it has in the source.
pub(super) fn lower_read_place(place: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    match &place.node {
        ExprNode::IdentifierExpr(ident) => {
            let Some(state) = ctx.binding_state(&ident.name).cloned() else {
                ctx.unported("reads of places that are not local");
                return LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") };
            };
            let value = IrValue { kind: IrValueKind::Local, name: ident.name.clone(), ..Default::default() };
            if state.ty.is_some() {
                ctx.register_value_type(&value, state.ty.clone());
            } else {
                ctx.register_value_type(&value, stored_expr_type(&ctx.scope, &Some(place.clone())).flatten());
            }
            let key_ir = lower_implicit_key_access(place, ast::KeyMode::Read, ctx);
            LowerResult { ir: seq_ir(vec![Some(key_ir), Some(Arc::new(Ir::ReadVar { name: ident.name.clone() }))]), value }
        }
        ExprNode::FieldAccessExpr(node) => lower_field_access_expr(place, node, ctx),
        ExprNode::TupleAccessExpr(node) => lower_tuple_access_expr(place, node, ctx),
        other => {
            ctx.unported(&format!("reads of the place {}", variant_name(other)));
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
        }
    }
}

/// `MarkMovedPlace`: the binding a place starts from is moved, or one of its fields is.
fn mark_moved_place(place: &Arc<Expr>, ctx: &mut LowerCtx) -> IrPtr {
    if let Some(root) = place_root(place) {
        if ctx.binding_state(&root).is_some() {
            let head = field_head(place);
            if let Some(state) = ctx.binding_states.get_mut(&root).and_then(|states| states.last_mut()) {
                match head {
                    Some(field) => state.moved_fields.push(field),
                    None => state.is_moved = true,
                }
            }
        } else {
            ctx.unported("moves out of places that are not local");
        }
    }
    Arc::new(Ir::MoveState { place: IrPlace { repr: build_place_repr(place) } })
}

/// `LowerMovePlace`.
fn lower_move_place(place: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    let read_result = lower_read_place(place, ctx);
    let move_state = mark_moved_place(place, ctx);
    LowerResult { ir: seq_ir(vec![Some(read_result.ir), Some(move_state)]), value: read_result.value }
}

/// `LowerPath`: a read of the path of an item.
pub(super) fn lower_path_expr(node: &ast::PathExpr) -> LowerResult {
    let value = IrValue { kind: IrValueKind::Symbol, name: node.name.clone(), ..Default::default() };
    LowerResult { ir: Arc::new(Ir::ReadPath { path: node.path.clone(), name: node.name.clone() }), value }
}

pub(super) fn lower_expr_impl(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    match &expr.node {
        ExprNode::LiteralExpr(lit) => lower_literal(expr, lit, ctx),
        ExprNode::IdentifierExpr(ident) => lower_identifier(expr, ident, ctx),
        ExprNode::PathExpr(node) => lower_path_expr(node),
        ExprNode::MoveExpr(node) => match &node.place {
            Some(place) => lower_move_place(place, ctx),
            None => {
                ctx.unported("moves without a place");
                LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
            }
        },
        ExprNode::BinaryExpr(node) => lower_binary_expr(node, ctx),
        ExprNode::RecordExpr(node) => lower_record_expr(expr, node, ctx),
        ExprNode::IfExpr(node) => lower_if_expr(expr, node, ctx),
        ExprNode::TupleExpr(node) => lower_tuple_expr(expr, node, ctx),
        ExprNode::FieldAccessExpr(node) => lower_field_access_expr(expr, node, ctx),
        ExprNode::TupleAccessExpr(node) => lower_tuple_access_expr(expr, node, ctx),
        ExprNode::IndexAccessExpr(node) => lower_index_access(expr, node, ctx),
        ExprNode::ArrayExpr(node) => lower_array_expr(node, ctx),
        ExprNode::ArrayRepeatExpr(node) => lower_array_repeat_expr(node, ctx),
        ExprNode::SizeofExpr(node) => lower_layout_constant(&node.r#type, false, ctx),
        ExprNode::AlignofExpr(node) => lower_layout_constant(&node.r#type, true, ctx),
        ExprNode::UnaryExpr(node) => lower_unary_expr(node, ctx),
        ExprNode::BlockExpr(node) => match &node.block {
            Some(block) => lower_block(block, ctx),
            None => {
                ctx.unported("block expressions without a block");
                LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
            }
        },
        ExprNode::UnsafeBlockExpr(node) => match &node.block {
            Some(block) => lower_block(block, ctx),
            None => {
                ctx.unported("unsafe blocks without a block");
                LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
            }
        },
        ExprNode::CallExpr(call) => lower_call(call, ctx),
        ExprNode::EnumLiteralExpr(node) => lower_enum_literal(expr, node, ctx),
        // `LowerPtrNull`: the null pointer is the immediate of eight zero bytes.
        ExprNode::PtrNullExpr(_) => LowerResult {
            ir: empty_ir(),
            value: IrValue { kind: IrValueKind::Immediate, name: "null".to_string(), bytes: vec![0; 8], ..Default::default() },
        },
        ExprNode::MethodCallExpr(call) => {
            let receiver = stored_expr_type(&ctx.scope, &Some(call.receiver.clone().unwrap_or_default())).flatten();
            let kind = strip_perm(&receiver).map(|ty| variant_name(&ty.node)).unwrap_or_default();
            ctx.unported(&format!("method call {} on {kind}", call.name));
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
        }
        _ => {
            ctx.unported(&format!("expression {}", variant_name(&expr.node)));
            LowerResult { ir: empty_ir(), value: ctx.fresh_temp_value("unported") }
        }
    }
}

/// `LowerExpr`: the expression, then the bookkeeping every expression shares.
pub(super) fn lower_expr(expr: &Arc<Expr>, ctx: &mut LowerCtx) -> LowerResult {
    ctx.temp_depth += 1;
    let result = lower_expr_impl(expr, ctx);
    let depth = ctx.temp_depth;
    let mut value_type = ctx.lookup_value_type(&result.value);
    if value_type.is_none() {
        if let Some(inferred) = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten() {
            ctx.register_value_type(&result.value, Some(inferred.clone()));
            value_type = Some(inferred);
        }
    }
    ctx.temp_depth -= 1;
    let suppress = ctx.suppress_temp_at_depth == Some(depth);
    if suppress {
        ctx.suppress_temp_at_depth = None;
    }
    if !suppress && ctx.temp_sink.is_some() && !is_place_expr(&Some(expr.clone())) {
        ctx.register_temp_value(&result.value, &value_type, true);
    }
    if needs_refinement_check(expr, ctx) {
        ctx.unported("checks of refinement types");
    }
    apply_effective_ordering(expr, result, ctx)
}

/// `LowerEnumLiteral`: the payload, and a value the emitter builds from the variant.
pub(super) fn lower_enum_literal(expr: &Arc<Expr>, node: &ast::EnumLiteralExpr, ctx: &mut LowerCtx) -> LowerResult {
    let variant = node.path.last().cloned().unwrap_or_default();
    let static_path = if node.path.len() >= 2 { node.path[..node.path.len() - 1].to_vec() } else { Vec::new() };
    let mut info = DerivedValueInfo::new(DerivedKind::EnumLit);
    info.variant = variant;
    info.static_path = static_path;
    let (ir, prefix) = match &node.payload_opt {
        None => (empty_ir(), "enum_unit"),
        Some(ast::EnumPayload::EnumPayloadParen(payload)) => {
            let (ir, values) = lower_list(&payload.elements, ctx);
            info.payload_elems = values;
            (ir, "enum_tuple")
        }
        Some(ast::EnumPayload::EnumPayloadBrace(payload)) => {
            let (ir, values) = lower_field_inits(&payload.fields, ctx);
            info.payload_fields = values;
            (ir, "enum_record")
        }
    };
    let value = ctx.fresh_temp_value(prefix);
    ctx.register_derived_value(&value, info);
    if let Some(ty) = stored_expr_type(&ctx.scope, &Some(expr.clone())).flatten() {
        ctx.register_value_type(&value, Some(ty));
    }
    LowerResult { ir, value }
}
