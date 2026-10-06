//! Collecting every expression node of a piece of syntax, by address. The oracle has
//! the same walk, generated from the same description of the tree.

use std::sync::Arc;

use uv_core::span::Span;

use super::Expr;
use crate::lexer::{DocComment, Token};

pub trait ExprWalk {
    fn walk_exprs(&self, out: &mut Vec<*const Expr>);
}

macro_rules! leaf {
    ($($ty:ty),*) => {
        $(impl ExprWalk for $ty {
            fn walk_exprs(&self, _out: &mut Vec<*const Expr>) {}
        })*
    };
}

leaf!(String, bool, usize, u128, Span, Token, DocComment);

impl<T: ExprWalk> ExprWalk for Option<T> {
    fn walk_exprs(&self, out: &mut Vec<*const Expr>) {
        if let Some(value) = self {
            value.walk_exprs(out);
        }
    }
}

impl<T: ExprWalk> ExprWalk for Vec<T> {
    fn walk_exprs(&self, out: &mut Vec<*const Expr>) {
        for value in self {
            value.walk_exprs(out);
        }
    }
}

impl<T: ExprWalk> ExprWalk for Arc<T> {
    fn walk_exprs(&self, out: &mut Vec<*const Expr>) {
        (**self).walk_exprs(out);
    }
}
