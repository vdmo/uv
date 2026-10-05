//! Type equivalence: structural, with paths compared by identifier key and unions
//! compared member by member in canonical order.

use uv_source::ast;

use super::types::*;
use crate::contracts::struct_equal::expr_struct_equal;
use crate::resolve::scopes::id_key_of;

fn type_path_eq(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| a == b || id_key_of(a) == id_key_of(b))
}

fn all_equiv(lhs: &[TypeRef], rhs: &[TypeRef]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| type_equiv(a, b))
}

fn collect_conjuncts(expr: &ast::ExprPtr, out: &mut Vec<ast::ExprPtr>) {
    match expr.as_deref().map(|expr| &expr.node) {
        None => {}
        Some(ast::ExprNode::BinaryExpr(binary)) if binary.op == "&&" => {
            collect_conjuncts(&binary.lhs, out);
            collect_conjuncts(&binary.rhs, out);
        }
        Some(_) => out.push(expr.clone()),
    }
}

/// A chain of refinements as its innermost base and all predicates, outermost first,
/// each split at `&&`.
fn unpack_refine(ty: &TypeRef) -> (TypeRef, Vec<ast::ExprPtr>) {
    let mut predicates = Vec::new();
    let mut current = ty.clone();
    while let Some(TypeNode::Refine { base, predicate }) = current.as_deref().map(|ty| &ty.node) {
        collect_conjuncts(predicate, &mut predicates);
        let base = base.clone();
        current = base;
    }
    (current, predicates)
}

/// Whether two types are the same type. A missing type is equivalent to nothing.
pub fn type_equiv(lhs: &TypeRef, rhs: &TypeRef) -> bool {
    let (Some(left), Some(right)) = (lhs, rhs) else {
        return false;
    };
    if std::sync::Arc::ptr_eq(left, right) {
        return true;
    }
    use TypeNode::*;
    match (&left.node, &right.node) {
        (Prim(a), Prim(b)) => a == b,
        (Var(a), Var(b)) => a == b,
        (Perm { perm: p, base: a }, Perm { perm: q, base: b }) => p == q && type_equiv(a, b),
        (Tuple(a), Tuple(b)) => all_equiv(a, b),
        (Array { element: a, length: m, .. }, Array { element: b, length: n, .. }) => m == n && type_equiv(a, b),
        (Slice(a), Slice(b)) => type_equiv(a, b),
        (Func { params: p, ret: a }, Func { params: q, ret: b }) => {
            p.len() == q.len()
                && p.iter().zip(q).all(|(x, y)| x.mode == y.mode && type_equiv(&x.r#type, &y.r#type))
                && type_equiv(a, b)
        }
        (Closure { params: p, ret: a, deps_opt: d }, Closure { params: q, ret: b, deps_opt: e }) => {
            p.len() == q.len()
                && p.iter().zip(q).all(|(x, y)| x.0 == y.0 && type_equiv(&x.1, &y.1))
                && match (d, e) {
                    (None, None) => true,
                    (Some(d), Some(e)) => {
                        d.len() == e.len()
                            && d.iter().zip(e).all(|(x, y)| x.name == y.name && type_equiv(&x.r#type, &y.r#type))
                    }
                    _ => false,
                }
                && type_equiv(a, b)
        }
        (Union(a), Union(b)) => all_equiv(&sort_union_members(a), &sort_union_members(b)),
        // A path with arguments and an application of the same path are one type.
        (Path { path: p, generic_args: a } | Apply { path: p, args: a }, Path { path: q, generic_args: b } | Apply { path: q, args: b }) => {
            type_path_eq(p, q) && all_equiv(a, b)
        }
        (ModalState(a), ModalState(b)) => {
            type_path_eq(&a.path, &b.path) && a.state == b.state && all_equiv(&a.generic_args, &b.generic_args)
        }
        (String(a), String(b)) => a == b,
        (Bytes(a), Bytes(b)) => a == b,
        (Range(a), Range(b))
        | (RangeInclusive(a), RangeInclusive(b))
        | (RangeFrom(a), RangeFrom(b))
        | (RangeTo(a), RangeTo(b))
        | (RangeToInclusive(a), RangeToInclusive(b)) => type_equiv(a, b),
        (RangeFull, RangeFull) => true,
        (Ptr { element: a, state: s }, Ptr { element: b, state: t }) => s == t && type_equiv(a, b),
        (RawPtr { qual: s, element: a }, RawPtr { qual: t, element: b }) => s == t && type_equiv(a, b),
        (Dynamic(a), Dynamic(b)) => type_path_eq(a, b),
        (Opaque { class_path: a, .. }, Opaque { class_path: b, .. }) => type_path_eq(a, b),
        (Refine { .. }, Refine { .. }) => {
            let (left_base, left_preds) = unpack_refine(lhs);
            let (right_base, right_preds) = unpack_refine(rhs);
            type_equiv(&left_base, &right_base)
                && left_preds.len() == right_preds.len()
                && left_preds.iter().zip(&right_preds).all(|(a, b)| expr_struct_equal(a, b))
        }
        _ => false,
    }
}
