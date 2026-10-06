//! A readable form of a written type, as `ast::to_string(const Type&)` prints it: what
//! hover and signature help show.

use super::generated::*;

fn perm(perm: TypePerm) -> &'static str {
    match perm {
        TypePerm::Const => "const",
        TypePerm::Unique => "unique",
        TypePerm::Shared => "shared",
    }
}

fn path(path: &[String], out: &mut String) {
    out.push_str(&path.join("::"));
}

fn list(types: &[Option<std::sync::Arc<Type>>], out: &mut String) {
    for (index, ty) in types.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        if let Some(ty) = ty {
            write_type(ty, out);
        }
    }
}

fn params(params: &[TypeFuncParam], out: &mut String) {
    for (index, param) in params.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        if param.mode.is_some() {
            out.push_str("move ");
        }
        if let Some(ty) = &param.r#type {
            write_type(ty, out);
        }
    }
}

fn opt(ty: &Option<std::sync::Arc<Type>>, out: &mut String) {
    if let Some(ty) = ty {
        write_type(ty, out);
    }
}

fn generic_args(args: &[Option<std::sync::Arc<Type>>], out: &mut String) {
    if !args.is_empty() {
        out.push('<');
        list(args, out);
        out.push('>');
    }
}

fn write_type(ty: &Type, out: &mut String) {
    match &ty.node {
        TypeNode::TypePrim(node) => out.push_str(&node.name),
        TypeNode::TypePermType(node) => {
            out.push_str(perm(node.perm));
            out.push(' ');
            opt(&node.base, out);
        }
        TypeNode::TypeUnion(node) => {
            for (index, member) in node.types.iter().enumerate() {
                if index > 0 {
                    out.push_str(" | ");
                }
                opt(member, out);
            }
        }
        TypeNode::TypeFunc(node) => {
            out.push('(');
            params(&node.params, out);
            out.push_str(") -> ");
            opt(&node.ret, out);
        }
        TypeNode::TypeClosure(node) => {
            out.push('|');
            params(&node.params, out);
            out.push_str("| -> ");
            opt(&node.ret, out);
            if let Some(deps) = &node.deps_opt {
                out.push_str(" [shared: {");
                for (index, dep) in deps.iter().enumerate() {
                    if index > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&dep.name);
                    out.push_str(": ");
                    opt(&dep.r#type, out);
                }
                out.push_str("}]");
            }
        }
        TypeNode::TypeTuple(node) => {
            out.push('(');
            list(&node.elements, out);
            if node.elements.len() == 1 {
                out.push(';');
            }
            out.push(')');
        }
        TypeNode::TypeArray(node) => {
            out.push('[');
            opt(&node.element, out);
            out.push_str("; <length>]");
        }
        TypeNode::TypeSlice(node) => {
            out.push('[');
            opt(&node.element, out);
            out.push(']');
        }
        TypeNode::TypeSafePtr(node) => {
            out.push_str("Ptr<");
            opt(&node.element, out);
            out.push('>');
            if let Some(state) = node.state {
                out.push('@');
                out.push_str(match state {
                    PtrState::Valid => "Valid",
                    PtrState::Null => "Null",
                    PtrState::Expired => "Expired",
                });
            }
        }
        TypeNode::TypeRawPtr(node) => {
            out.push('*');
            out.push_str(match node.qual {
                RawPtrQual::Imm => "imm",
                RawPtrQual::Mut => "mut",
            });
            out.push(' ');
            opt(&node.element, out);
        }
        TypeNode::TypeString(node) => {
            out.push_str("string");
            if let Some(state) = node.state {
                out.push('@');
                out.push_str(if state == StringState::Managed { "Managed" } else { "View" });
            }
        }
        TypeNode::TypeBytes(node) => {
            out.push_str("bytes");
            if let Some(state) = node.state {
                out.push('@');
                out.push_str(if state == BytesState::Managed { "Managed" } else { "View" });
            }
        }
        TypeNode::TypeDynamic(node) => {
            out.push('$');
            path(&node.path, out);
        }
        TypeNode::TypeModalState(node) => {
            path(&node.path, out);
            generic_args(&node.generic_args, out);
            out.push('@');
            out.push_str(&node.state);
        }
        TypeNode::TypePathType(node) => {
            path(&node.path, out);
            generic_args(&node.generic_args, out);
        }
        TypeNode::TypeApply(node) => {
            path(&node.path, out);
            out.push('<');
            list(&node.args, out);
            out.push('>');
        }
        TypeNode::TypeOpaque(node) => {
            out.push_str("opaque ");
            path(&node.path, out);
        }
        TypeNode::TypeRefine(node) => {
            opt(&node.base, out);
            out.push_str(" |: { ... }");
        }
        TypeNode::TypeRange(node) => wrapped("Range", &node.base, out),
        TypeNode::TypeRangeInclusive(node) => wrapped("RangeInclusive", &node.base, out),
        TypeNode::TypeRangeFrom(node) => wrapped("RangeFrom", &node.base, out),
        TypeNode::TypeRangeTo(node) => wrapped("RangeTo", &node.base, out),
        TypeNode::TypeRangeToInclusive(node) => wrapped("RangeToInclusive", &node.base, out),
        TypeNode::TypeRangeFull(_) => out.push_str("RangeFull"),
        _ => out.push_str("<unknown type>"),
    }
}

fn wrapped(name: &str, base: &Option<std::sync::Arc<Type>>, out: &mut String) {
    out.push_str(name);
    out.push('<');
    opt(base, out);
    out.push('>');
}

/// `ast::to_string(type)` without spans.
pub fn type_to_string(ty: &Type) -> String {
    let mut out = String::new();
    write_type(ty, &mut out);
    out
}
