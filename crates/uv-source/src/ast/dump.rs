//! Canonical text dump of AST values, matching the reference oracle's dump byte for byte.

use std::sync::Arc;

use uv_core::span::Span;

use crate::lexer::{DocComment, Token};

pub trait AstDump {
    fn dump(&self, out: &mut String);
}

pub fn dump_to_string<T: AstDump + ?Sized>(value: &T) -> String {
    let mut out = String::new();
    value.dump(&mut out);
    out
}

fn dump_text(text: &str, out: &mut String) {
    out.push('"');
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out.push('"');
}

impl AstDump for String {
    fn dump(&self, out: &mut String) {
        dump_text(self, out);
    }
}

impl AstDump for bool {
    fn dump(&self, out: &mut String) {
        out.push_str(if *self { "true" } else { "false" });
    }
}

impl AstDump for usize {
    fn dump(&self, out: &mut String) {
        out.push_str(&self.to_string());
    }
}

impl AstDump for u128 {
    fn dump(&self, out: &mut String) {
        out.push_str(&self.to_string());
    }
}

impl AstDump for Span {
    fn dump(&self, out: &mut String) {
        out.push_str(&format!(
            "@{}:{}:{}:{}:{}:{}",
            self.start_offset,
            self.end_offset,
            self.start_line,
            self.start_col,
            self.end_line,
            self.end_col
        ));
    }
}

impl AstDump for Token {
    fn dump(&self, out: &mut String) {
        out.push_str("(Token ");
        out.push_str(self.kind.name());
        out.push(' ');
        dump_text(&self.lexeme, out);
        out.push(' ');
        self.span.dump(out);
        out.push(')');
    }
}

impl AstDump for DocComment {
    fn dump(&self, out: &mut String) {
        out.push_str("(Doc ");
        out.push_str(self.kind.name());
        out.push(' ');
        dump_text(&self.text, out);
        out.push(' ');
        self.span.dump(out);
        out.push(')');
    }
}

impl<T: AstDump> AstDump for Option<T> {
    fn dump(&self, out: &mut String) {
        match self {
            None => out.push_str("none"),
            Some(value) => value.dump(out),
        }
    }
}

impl<T: AstDump> AstDump for Arc<T> {
    fn dump(&self, out: &mut String) {
        (**self).dump(out);
    }
}

impl<T: AstDump> AstDump for Vec<T> {
    fn dump(&self, out: &mut String) {
        out.push('[');
        for (index, value) in self.iter().enumerate() {
            if index != 0 {
                out.push(' ');
            }
            value.dump(out);
        }
        out.push(']');
    }
}
