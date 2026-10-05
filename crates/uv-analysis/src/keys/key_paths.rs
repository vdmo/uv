//! Key paths: the places keys are held on, built from field, element and tuple accesses
//! rooted at a binding. A dereference is a boundary that paths do not extend across.

use uv_source::ast::{ArgPassKind, ExprNode, ExprPtr, RangeKind};

use crate::contracts::verification::{evaluate_constant, ConstValue};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyPathSeg {
    pub boundary: bool,
    /// A field name, or the canonical text of an index expression.
    pub name: String,
    pub is_index: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyPath {
    pub root: String,
    pub segs: Vec<KeyPathSeg>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoundaryKind {
    #[default]
    None,
    PointerDeref,
    FieldMarker,
    TypeBoundary,
}

#[derive(Debug, Clone, Default)]
pub struct KeyPathResult {
    pub success: bool,
    pub path: KeyPath,
    pub error_code: Option<&'static str>,
    pub hit_boundary: bool,
    pub boundary_kind: BoundaryKind,
}

pub fn format_tuple_index(index: u128) -> String {
    index.to_string()
}

fn join_path(path: &[String]) -> String {
    path.join("::")
}

/// A text that two index expressions share exactly when they denote the same index.
pub fn canonical_expr_identity(expr: &ExprPtr) -> String {
    let Some(e) = expr.as_deref() else {
        return "?".to_string();
    };
    match evaluate_constant(expr) {
        ConstValue::Bool(value) => return value.to_string(),
        ConstValue::Int(value) => return value.to_string(),
        ConstValue::Unknown => {}
    }
    let args_text = |args: &[uv_source::ast::Arg]| {
        args.iter()
            .map(|arg| {
                let prefix = if arg.pass == ArgPassKind::Move {
                    "move "
                } else {
                    ""
                };
                format!("{prefix}{}", canonical_expr_identity(&arg.value))
            })
            .collect::<Vec<_>>()
            .join(",")
    };
    match &e.node {
        ExprNode::IdentifierExpr(ident) => ident.name.clone(),
        ExprNode::LiteralExpr(literal) => literal.literal.lexeme.clone(),
        ExprNode::QualifiedNameExpr(qname) => format!("{}::{}", join_path(&qname.path), qname.name),
        ExprNode::PathExpr(path) => format!("{}::{}", join_path(&path.path), path.name),
        ExprNode::FieldAccessExpr(field) => {
            format!("{}.{}", canonical_expr_identity(&field.base), field.name)
        }
        ExprNode::TupleAccessExpr(tuple) => {
            format!(
                "{}.{}",
                canonical_expr_identity(&tuple.base),
                format_tuple_index(tuple.index)
            )
        }
        ExprNode::IndexAccessExpr(index) => {
            format!(
                "{}[{}]",
                canonical_expr_identity(&index.base),
                canonical_expr_identity(&index.index)
            )
        }
        ExprNode::UnaryExpr(unary) => {
            format!("({}{})", unary.op, canonical_expr_identity(&unary.value))
        }
        ExprNode::BinaryExpr(binary) => {
            format!(
                "({}{}{})",
                canonical_expr_identity(&binary.lhs),
                binary.op,
                canonical_expr_identity(&binary.rhs)
            )
        }
        ExprNode::CastExpr(cast) => format!("({} as _)", canonical_expr_identity(&cast.value)),
        ExprNode::MoveExpr(node) => format!("(move {})", canonical_expr_identity(&node.place)),
        ExprNode::AddressOfExpr(node) => format!("(&{})", canonical_expr_identity(&node.place)),
        ExprNode::DerefExpr(node) => format!("(*{})", canonical_expr_identity(&node.value)),
        ExprNode::CallExpr(call) => format!(
            "{}({})",
            canonical_expr_identity(&call.callee),
            args_text(&call.args)
        ),
        ExprNode::MethodCallExpr(method) => {
            format!(
                "{}~>{}({})",
                canonical_expr_identity(&method.receiver),
                method.name,
                args_text(&method.args)
            )
        }
        ExprNode::PropagateExpr(node) => format!("{}?", canonical_expr_identity(&node.value)),
        ExprNode::AttributedExpr(node) => canonical_expr_identity(&node.expr),
        ExprNode::RangeExpr(range) => {
            let lhs = if range.lhs.is_some() {
                canonical_expr_identity(&range.lhs)
            } else {
                String::new()
            };
            let rhs = if range.rhs.is_some() {
                canonical_expr_identity(&range.rhs)
            } else {
                String::new()
            };
            match range.kind {
                RangeKind::To => format!("..{rhs}"),
                RangeKind::ToInclusive => format!("..={rhs}"),
                RangeKind::Full => "..".to_string(),
                RangeKind::From => format!("{lhs}.."),
                RangeKind::Exclusive => format!("{lhs}..{rhs}"),
                RangeKind::Inclusive => format!("{lhs}..={rhs}"),
            }
        }
        // The reference writes the variant's index; the kind identifies it as well,
        // and the span makes the text unique to the expression either way.
        _ => format!(
            "$expr{}@{}:{}",
            uv_source::ast::expr_kind(e),
            e.span.start_offset,
            e.span.end_offset
        ),
    }
}

fn build_path_segments(
    expr: &ExprPtr,
    segs: &mut Vec<KeyPathSeg>,
    hit_boundary: &mut bool,
    boundary_kind: &mut BoundaryKind,
) {
    let Some(e) = expr.as_deref() else {
        return;
    };
    match &e.node {
        ExprNode::FieldAccessExpr(field) => {
            build_path_segments(&field.base, segs, hit_boundary, boundary_kind);
            if *hit_boundary {
                return;
            }
            segs.push(KeyPathSeg {
                boundary: false,
                name: field.name.clone(),
                is_index: false,
            });
        }
        ExprNode::IndexAccessExpr(index) => {
            build_path_segments(&index.base, segs, hit_boundary, boundary_kind);
            if *hit_boundary {
                return;
            }
            segs.push(KeyPathSeg {
                boundary: false,
                name: canonical_expr_identity(&index.index),
                is_index: true,
            });
        }
        ExprNode::TupleAccessExpr(tuple) => {
            build_path_segments(&tuple.base, segs, hit_boundary, boundary_kind);
            if *hit_boundary {
                return;
            }
            segs.push(KeyPathSeg {
                boundary: false,
                name: format_tuple_index(tuple.index),
                is_index: false,
            });
        }
        ExprNode::DerefExpr(_) => {
            *hit_boundary = true;
            *boundary_kind = BoundaryKind::PointerDeref;
        }
        ExprNode::MethodCallExpr(method) => {
            build_path_segments(&method.receiver, segs, hit_boundary, boundary_kind)
        }
        _ => {}
    }
}

/// The binding a place is rooted at; `$deref` past a dereference.
pub fn extract_path_root(expr: &ExprPtr) -> Option<String> {
    match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => Some(ident.name.clone()),
        ExprNode::FieldAccessExpr(field) => extract_path_root(&field.base),
        ExprNode::IndexAccessExpr(index) => extract_path_root(&index.base),
        ExprNode::MethodCallExpr(method) => extract_path_root(&method.receiver),
        ExprNode::TupleAccessExpr(tuple) => extract_path_root(&tuple.base),
        ExprNode::DerefExpr(_) => Some("$deref".to_string()),
        _ => None,
    }
}

pub fn build_key_path(expr: &ExprPtr) -> KeyPathResult {
    let mut result = KeyPathResult::default();
    if expr.is_none() {
        result.error_code = Some("E-CON-0034");
        return result;
    }
    let Some(root) = extract_path_root(expr) else {
        result.error_code = Some("E-CON-0031");
        return result;
    };
    result.path.root = root;
    let mut segs = Vec::new();
    build_path_segments(
        expr,
        &mut segs,
        &mut result.hit_boundary,
        &mut result.boundary_kind,
    );
    result.path.segs = segs;
    result.success = true;
    result
}

pub fn is_place_expression(expr: &ExprPtr) -> bool {
    match expr.as_deref().map(|e| &e.node) {
        Some(ExprNode::IdentifierExpr(_) | ExprNode::DerefExpr(_)) => true,
        Some(ExprNode::FieldAccessExpr(field)) => is_place_expression(&field.base),
        Some(ExprNode::IndexAccessExpr(index)) => is_place_expression(&index.base),
        Some(ExprNode::TupleAccessExpr(tuple)) => is_place_expression(&tuple.base),
        _ => false,
    }
}

pub fn is_prefix(prefix: &KeyPath, path: &KeyPath) -> bool {
    prefix.root == path.root
        && prefix.segs.len() <= path.segs.len()
        && prefix
            .segs
            .iter()
            .zip(&path.segs)
            .all(|(lhs, rhs)| lhs == rhs)
}
