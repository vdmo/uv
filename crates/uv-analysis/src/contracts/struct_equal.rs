//! Structural equality of syntax, ignoring source locations. Two refinement types are
//! the same type when their predicates are equal in this sense.

use uv_source::ast::*;

use crate::resolve::scopes::id_eq;

/// Paths are equal segment by segment; a bare name also equals a qualified path that
/// ends in it.
fn type_path_eq(a: &[String], b: &[String]) -> bool {
    if a.len() == b.len() {
        return a.iter().zip(b).all(|(x, y)| id_eq(x, y));
    }
    match (a, b) {
        ([only], [.., last]) | ([.., last], [only]) => id_eq(only, last),
        _ => false,
    }
}

fn all_eq<T>(a: &[T], b: &[T], eq: impl Fn(&T, &T) -> bool) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| eq(x, y))
}

fn exprs_eq(a: &[ExprPtr], b: &[ExprPtr]) -> bool {
    all_eq(a, b, expr_struct_equal)
}

fn types_eq(a: &[TypePtr], b: &[TypePtr]) -> bool {
    all_eq(a, b, type_struct_equal)
}

fn args_eq(a: &[Arg], b: &[Arg]) -> bool {
    all_eq(a, b, |x, y| x.pass == y.pass && expr_struct_equal(&x.value, &y.value))
}

fn field_inits_eq(a: &[FieldInit], b: &[FieldInit]) -> bool {
    all_eq(a, b, |x, y| x.name == y.name && expr_struct_equal(&x.value, &y.value))
}

fn token_eq(a: &uv_source::lexer::token::Token, b: &uv_source::lexer::token::Token) -> bool {
    a.kind == b.kind && a.lexeme == b.lexeme
}

fn modal_ref_eq(a: &ModalRef, b: &ModalRef) -> bool {
    match (a, b) {
        (ModalRef::Path(x), ModalRef::Path(y)) => type_path_eq(x, y),
        (ModalRef::GenericTypeRef(x), ModalRef::GenericTypeRef(y)) => {
            type_path_eq(&x.path, &y.path) && types_eq(&x.generic_args, &y.generic_args)
        }
        _ => false,
    }
}

fn record_target_eq(a: &RecordExprTarget, b: &RecordExprTarget) -> bool {
    match (a, b) {
        (RecordExprTarget::Path(x), RecordExprTarget::Path(y)) => type_path_eq(x, y),
        (RecordExprTarget::ModalStateRef(x), RecordExprTarget::ModalStateRef(y)) => {
            x.state == y.state && modal_ref_eq(&x.modal_ref, &y.modal_ref)
        }
        _ => false,
    }
}

fn enum_payload_eq(a: &Option<EnumPayload>, b: &Option<EnumPayload>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(EnumPayload::EnumPayloadParen(x)), Some(EnumPayload::EnumPayloadParen(y))) => {
            exprs_eq(&x.elements, &y.elements)
        }
        (Some(EnumPayload::EnumPayloadBrace(x)), Some(EnumPayload::EnumPayloadBrace(y))) => {
            field_inits_eq(&x.fields, &y.fields)
        }
        _ => false,
    }
}

fn array_segment_eq(a: &ArraySegment, b: &ArraySegment) -> bool {
    match (a, b) {
        (ArraySegment::ArrayElemSegment(x), ArraySegment::ArrayElemSegment(y)) => expr_struct_equal(&x.value, &y.value),
        (ArraySegment::ArrayRepeatSegment(x), ArraySegment::ArrayRepeatSegment(y)) => {
            expr_struct_equal(&x.value, &y.value) && expr_struct_equal(&x.count, &y.count)
        }
        _ => false,
    }
}

fn apply_args_eq(a: &ApplyArgs, b: &ApplyArgs) -> bool {
    match (a, b) {
        (ApplyArgs::ParenArgs(x), ApplyArgs::ParenArgs(y)) => args_eq(&x.args, &y.args),
        (ApplyArgs::BraceArgs(x), ApplyArgs::BraceArgs(y)) => field_inits_eq(&x.fields, &y.fields),
        _ => false,
    }
}

/// Expression forms outside this list (loops, closures, concurrency forms and the like)
/// are never equal, not even to themselves.
pub fn expr_struct_equal(a: &ExprPtr, b: &ExprPtr) -> bool {
    let (a, b) = match (a, b) {
        (None, None) => return true,
        (Some(a), Some(b)) => (a, b),
        _ => return false,
    };
    use ExprNode::*;
    let eq = expr_struct_equal;
    match (&a.node, &b.node) {
        (LiteralExpr(x), LiteralExpr(y)) => token_eq(&x.literal, &y.literal),
        (IdentifierExpr(x), IdentifierExpr(y)) => x.name == y.name,
        (QualifiedNameExpr(x), QualifiedNameExpr(y)) => x.path == y.path && x.name == y.name,
        (PathExpr(x), PathExpr(y)) => x.path == y.path && x.name == y.name,
        (EnumLiteralExpr(x), EnumLiteralExpr(y)) => {
            type_path_eq(&x.path, &y.path) && enum_payload_eq(&x.payload_opt, &y.payload_opt)
        }
        (TypeLiteralExpr(x), TypeLiteralExpr(y)) => type_struct_equal(&x.r#type, &y.r#type),
        (RangeExpr(x), RangeExpr(y)) => x.kind == y.kind && eq(&x.lhs, &y.lhs) && eq(&x.rhs, &y.rhs),
        (BinaryExpr(x), BinaryExpr(y)) => x.op == y.op && eq(&x.lhs, &y.lhs) && eq(&x.rhs, &y.rhs),
        (CastExpr(x), CastExpr(y)) => eq(&x.value, &y.value) && type_struct_equal(&x.r#type, &y.r#type),
        (UnaryExpr(x), UnaryExpr(y)) => x.op == y.op && eq(&x.value, &y.value),
        (DerefExpr(x), DerefExpr(y)) => eq(&x.value, &y.value),
        (AddressOfExpr(x), AddressOfExpr(y)) => eq(&x.place, &y.place),
        (MoveExpr(x), MoveExpr(y)) => eq(&x.place, &y.place),
        (AllocExpr(x), AllocExpr(y)) => x.region_opt == y.region_opt && eq(&x.value, &y.value),
        (PtrNullExpr(_), PtrNullExpr(_)) | (ResultExpr(_), ResultExpr(_)) => true,
        (TupleExpr(x), TupleExpr(y)) => exprs_eq(&x.elements, &y.elements),
        (ArrayExpr(x), ArrayExpr(y)) => all_eq(&x.elements, &y.elements, array_segment_eq),
        (ArrayRepeatExpr(x), ArrayRepeatExpr(y)) => eq(&x.value, &y.value) && eq(&x.count, &y.count),
        (SizeofExpr(x), SizeofExpr(y)) => type_struct_equal(&x.r#type, &y.r#type),
        (AlignofExpr(x), AlignofExpr(y)) => type_struct_equal(&x.r#type, &y.r#type),
        (RecordExpr(x), RecordExpr(y)) => record_target_eq(&x.target, &y.target) && field_inits_eq(&x.fields, &y.fields),
        (IfExpr(x), IfExpr(y)) => {
            eq(&x.cond, &y.cond) && eq(&x.then_expr, &y.then_expr) && eq(&x.else_expr, &y.else_expr)
        }
        (IfIsExpr(x), IfIsExpr(y)) => {
            eq(&x.scrutinee, &y.scrutinee)
                && pattern_struct_equal(&x.pattern, &y.pattern)
                && eq(&x.then_expr, &y.then_expr)
                && eq(&x.else_expr, &y.else_expr)
        }
        (IfCaseExpr(x), IfCaseExpr(y)) => {
            eq(&x.scrutinee, &y.scrutinee)
                && all_eq(&x.cases, &y.cases, |p, q| pattern_struct_equal(&p.pattern, &q.pattern) && eq(&p.body, &q.body))
                && eq(&x.else_expr, &y.else_expr)
        }
        (BlockExpr(x), BlockExpr(y)) => block_struct_equal(&x.block, &y.block),
        (ComptimeExpr(x), ComptimeExpr(y)) => eq(&x.body, &y.body),
        (TransmuteExpr(x), TransmuteExpr(y)) => {
            type_struct_equal(&x.from, &y.from) && type_struct_equal(&x.to, &y.to) && eq(&x.value, &y.value)
        }
        (PipelineExpr(x), PipelineExpr(y)) => eq(&x.lhs, &y.lhs) && eq(&x.rhs, &y.rhs),
        (FieldAccessExpr(x), FieldAccessExpr(y)) => x.name == y.name && eq(&x.base, &y.base),
        (TupleAccessExpr(x), TupleAccessExpr(y)) => x.index == y.index && eq(&x.base, &y.base),
        (IndexAccessExpr(x), IndexAccessExpr(y)) => eq(&x.base, &y.base) && eq(&x.index, &y.index),
        (CallExpr(x), CallExpr(y)) => {
            eq(&x.callee, &y.callee) && types_eq(&x.generic_args, &y.generic_args) && args_eq(&x.args, &y.args)
        }
        (CallTypeArgsExpr(x), CallTypeArgsExpr(y)) => {
            eq(&x.callee, &y.callee) && types_eq(&x.type_args, &y.type_args) && args_eq(&x.args, &y.args)
        }
        (QualifiedApplyExpr(x), QualifiedApplyExpr(y)) => {
            x.path == y.path && x.name == y.name && apply_args_eq(&x.args, &y.args)
        }
        (MethodCallExpr(x), MethodCallExpr(y)) => {
            x.name == y.name && eq(&x.receiver, &y.receiver) && args_eq(&x.args, &y.args)
        }
        (PropagateExpr(x), PropagateExpr(y)) => eq(&x.value, &y.value),
        (EntryExpr(x), EntryExpr(y)) => eq(&x.expr, &y.expr),
        _ => false,
    }
}

fn func_params_eq(a: &[TypeFuncParam], b: &[TypeFuncParam]) -> bool {
    all_eq(a, b, |x, y| x.mode == y.mode && type_struct_equal(&x.r#type, &y.r#type))
}

fn type_modal_ref_eq(a: &TypeModalRef, b: &TypeModalRef) -> bool {
    match (a, b) {
        (TypeModalRef::TypePathType(x), TypeModalRef::TypePathType(y)) => {
            type_path_eq(&x.path, &y.path) && types_eq(&x.generic_args, &y.generic_args)
        }
        (TypeModalRef::TypeApply(x), TypeModalRef::TypeApply(y)) => {
            type_path_eq(&x.path, &y.path) && types_eq(&x.args, &y.args)
        }
        _ => false,
    }
}

pub fn type_struct_equal(a: &TypePtr, b: &TypePtr) -> bool {
    let (a, b) = match (a, b) {
        (None, None) => return true,
        (Some(a), Some(b)) => (a, b),
        _ => return false,
    };
    use TypeNode::*;
    let eq = type_struct_equal;
    match (&a.node, &b.node) {
        (TypePrim(x), TypePrim(y)) => x.name == y.name,
        (TypePermType(x), TypePermType(y)) => x.perm == y.perm && eq(&x.base, &y.base),
        (TypeUnion(x), TypeUnion(y)) => types_eq(&x.types, &y.types),
        (TypeFunc(x), TypeFunc(y)) => func_params_eq(&x.params, &y.params) && eq(&x.ret, &y.ret),
        (TypeClosure(x), TypeClosure(y)) => {
            func_params_eq(&x.params, &y.params)
                && eq(&x.ret, &y.ret)
                && match (&x.deps_opt, &y.deps_opt) {
                    (None, None) => true,
                    (Some(p), Some(q)) => all_eq(p, q, |d, e| d.name == e.name && eq(&d.r#type, &e.r#type)),
                    _ => false,
                }
        }
        (TypeTuple(x), TypeTuple(y)) => types_eq(&x.elements, &y.elements),
        (TypeArray(x), TypeArray(y)) => eq(&x.element, &y.element) && expr_struct_equal(&x.length, &y.length),
        (TypeSlice(x), TypeSlice(y)) => eq(&x.element, &y.element),
        (TypeSafePtr(x), TypeSafePtr(y)) => x.state == y.state && eq(&x.element, &y.element),
        (TypeRawPtr(x), TypeRawPtr(y)) => x.qual == y.qual && eq(&x.element, &y.element),
        (TypeString(x), TypeString(y)) => x.state == y.state,
        (TypeBytes(x), TypeBytes(y)) => x.state == y.state,
        (TypeDynamic(x), TypeDynamic(y)) => type_path_eq(&x.path, &y.path),
        (TypeModalState(x), TypeModalState(y)) => x.state == y.state && type_modal_ref_eq(&x.modal_ref, &y.modal_ref),
        (TypePathType(x), TypePathType(y)) => {
            type_path_eq(&x.path, &y.path) && types_eq(&x.generic_args, &y.generic_args)
        }
        (TypeApply(x), TypeApply(y)) => type_path_eq(&x.path, &y.path) && types_eq(&x.args, &y.args),
        (SpliceExprNode(x), SpliceExprNode(y)) => expr_struct_equal(&x.expr, &y.expr),
        (TypeOpaque(x), TypeOpaque(y)) => type_path_eq(&x.path, &y.path),
        (TypeRefine(x), TypeRefine(y)) => eq(&x.base, &y.base) && expr_struct_equal(&x.predicate, &y.predicate),
        (TypeRange(x), TypeRange(y)) => eq(&x.base, &y.base),
        (TypeRangeInclusive(x), TypeRangeInclusive(y)) => eq(&x.base, &y.base),
        (TypeRangeFrom(x), TypeRangeFrom(y)) => eq(&x.base, &y.base),
        (TypeRangeTo(x), TypeRangeTo(y)) => eq(&x.base, &y.base),
        (TypeRangeToInclusive(x), TypeRangeToInclusive(y)) => eq(&x.base, &y.base),
        (TypeRangeFull(_), TypeRangeFull(_)) => true,
        _ => false,
    }
}

fn splice_ident_eq(a: &Option<SpliceIdentNode>, b: &Option<SpliceIdentNode>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => expr_struct_equal(&x.name_expr, &y.name_expr),
        _ => false,
    }
}

fn field_patterns_eq(a: &[FieldPattern], b: &[FieldPattern]) -> bool {
    all_eq(a, b, |x, y| x.name == y.name && pattern_struct_equal(&x.pattern_opt, &y.pattern_opt))
}

fn patterns_eq(a: &[PatternPtr], b: &[PatternPtr]) -> bool {
    all_eq(a, b, pattern_struct_equal)
}

pub fn pattern_struct_equal(a: &PatternPtr, b: &PatternPtr) -> bool {
    let (a, b) = match (a, b) {
        (None, None) => return true,
        (Some(a), Some(b)) => (a, b),
        _ => return false,
    };
    use PatternNode::*;
    match (&a.node, &b.node) {
        (LiteralPattern(x), LiteralPattern(y)) => token_eq(&x.literal, &y.literal),
        (WildcardPattern(_), WildcardPattern(_)) => true,
        (IdentifierPattern(x), IdentifierPattern(y)) => {
            x.name == y.name && splice_ident_eq(&x.name_splice_opt, &y.name_splice_opt)
        }
        (TypedPattern(x), TypedPattern(y)) => {
            x.name == y.name
                && type_struct_equal(&x.r#type, &y.r#type)
                && splice_ident_eq(&x.name_splice_opt, &y.name_splice_opt)
        }
        (SpliceExprNode(x), SpliceExprNode(y)) => expr_struct_equal(&x.expr, &y.expr),
        (TuplePattern(x), TuplePattern(y)) => patterns_eq(&x.elements, &y.elements),
        (RecordPattern(x), RecordPattern(y)) => type_path_eq(&x.path, &y.path) && field_patterns_eq(&x.fields, &y.fields),
        (EnumPattern(x), EnumPattern(y)) => {
            type_path_eq(&x.path, &y.path)
                && x.name == y.name
                && match (&x.payload_opt, &y.payload_opt) {
                    (None, None) => true,
                    (
                        Some(EnumPayloadPattern::TuplePayloadPattern(p)),
                        Some(EnumPayloadPattern::TuplePayloadPattern(q)),
                    ) => patterns_eq(&p.elements, &q.elements),
                    (
                        Some(EnumPayloadPattern::RecordPayloadPattern(p)),
                        Some(EnumPayloadPattern::RecordPayloadPattern(q)),
                    ) => field_patterns_eq(&p.fields, &q.fields),
                    _ => false,
                }
        }
        (ModalPattern(x), ModalPattern(y)) => {
            x.state == y.state
                && match (&x.fields_opt, &y.fields_opt) {
                    (None, None) => true,
                    (Some(p), Some(q)) => field_patterns_eq(&p.fields, &q.fields),
                    _ => false,
                }
        }
        (RangePattern(x), RangePattern(y)) => {
            x.kind == y.kind && pattern_struct_equal(&x.lo, &y.lo) && pattern_struct_equal(&x.hi, &y.hi)
        }
        _ => false,
    }
}

fn binding_eq(a: &Binding, b: &Binding) -> bool {
    pattern_struct_equal(&a.pat, &b.pat)
        && type_struct_equal(&a.type_opt, &b.type_opt)
        && token_eq(&a.op, &b.op)
        && expr_struct_equal(&a.init, &b.init)
}

fn stmt_struct_equal(a: &Stmt, b: &Stmt) -> bool {
    match (a, b) {
        (Stmt::LetStmt(x), Stmt::LetStmt(y)) => binding_eq(&x.binding, &y.binding),
        (Stmt::VarStmt(x), Stmt::VarStmt(y)) => binding_eq(&x.binding, &y.binding),
        (Stmt::ExprStmt(x), Stmt::ExprStmt(y)) => expr_struct_equal(&x.value, &y.value),
        (Stmt::ReturnStmt(x), Stmt::ReturnStmt(y)) => expr_struct_equal(&x.value_opt, &y.value_opt),
        (Stmt::BreakStmt(x), Stmt::BreakStmt(y)) => expr_struct_equal(&x.value_opt, &y.value_opt),
        (Stmt::ContinueStmt(_), Stmt::ContinueStmt(_)) => true,
        (Stmt::UnsafeBlockStmt(x), Stmt::UnsafeBlockStmt(y)) => block_struct_equal(&x.body, &y.body),
        _ => false,
    }
}

pub fn block_struct_equal(a: &BlockPtr, b: &BlockPtr) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            all_eq(&a.stmts, &b.stmts, stmt_struct_equal) && expr_struct_equal(&a.tail_opt, &b.tail_opt)
        }
        _ => false,
    }
}
