#!/usr/bin/env python3
"""Derive the AST schema from the reference C++ headers and generate, from that one schema:

  * crates/uv-source/src/ast/generated.rs   Rust AST types, defaults and dump impls
  * tools/oracle/ast_dump_generated.inc     C++ dump functions for the oracle tool
  * tools/oracle/ast_walk_generated.inc     C++ functions collecting every expression node

Both dumpers print the same text for the same tree, so parser output can be compared
node for node.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NODES = ROOT / "ultraviolet" / "Bootstrap" / "Ultraviolet" / "include" / "02_source" / "ast"
HEADERS = ["nodes/ast_fwd.h", "nodes/ast_enums.h", "nodes/ast_attributes.h", "ast_common.h",
           "nodes/ast_types.h", "nodes/ast_patterns.h", "nodes/ast_exprs.h", "nodes/ast_stmts.h",
           "nodes/ast_items.h", "nodes/ast_module.h"]
RUST_OUT = ROOT / "crates" / "uv-source" / "src" / "ast" / "generated.rs"
CPP_OUT = ROOT / "tools" / "oracle" / "ast_dump_generated.inc"
CPP_WALK_OUT = ROOT / "tools" / "oracle" / "ast_walk_generated.inc"

STRING_ALIASES = {"Identifier", "std::string"}
PATH_ALIASES = {"Path", "ModulePath", "TypePath", "ClassPath", "VendorPrefix"}
SPAN_ALIASES = {"Span", "core::Span", "ultraviolet::core::Span"}
PTR_ALIASES = {"ExprPtr": "Expr", "TypePtr": "Type", "PatternPtr": "Pattern", "BlockPtr": "Block"}
RUST_KEYWORDS = {"type", "mut", "ref", "move", "loop", "override", "abstract", "final", "box"}


def load() -> str:
    text = ""
    for header in HEADERS:
        for line in (NODES / header).read_text(encoding="utf-8").splitlines():
            line = re.sub(r"//.*$", "", line)
            if line.lstrip().startswith("#"):
                continue
            text += line + "\n"
    return text


def match_brace(text: str, open_index: int) -> int:
    depth = 0
    for index in range(open_index, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index
    raise SystemExit("unbalanced braces")


def depth_at(text: str, index: int) -> int:
    return text.count("{", 0, index) - text.count("}", 0, index)


def split_args(text: str) -> list[str]:
    parts, depth, current = [], 0, ""
    for ch in text:
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(current.strip())
            current = ""
        else:
            current += ch
    if current.strip():
        parts.append(current.strip())
    return parts


class Schema:
    def __init__(self) -> None:
        self.enums: dict[str, list[str]] = {}
        self.structs: dict[str, list[tuple[str, object, str | None]]] = {}
        self.variants: dict[str, list[object]] = {}
        self.aliases: dict[str, object] = {}
        self.order: list[tuple[str, str]] = []

    def parse_type(self, text: str, owner: str = "", field: str = ""):
        text = " ".join(text.split())
        if text in STRING_ALIASES:
            return ("string",)
        if text in PATH_ALIASES:
            return ("vec", ("string",))
        if text in SPAN_ALIASES:
            return ("span",)
        if text in PTR_ALIASES:
            return ("ptr", PTR_ALIASES[text])
        if text == "bool":
            return ("bool",)
        if text in ("std::size_t", "size_t"):
            return ("usize",)
        if text == "TupleIndex":
            return ("u128",)
        if text == "Token":
            return ("token",)
        if text == "DocList":
            return ("vec", ("doc",))
        if text == "AttributeList":
            return ("vec", ("named", "AttributeItem"))
        if text == "AttrOpt":
            return ("opt", ("vec", ("named", "AttributeItem")))
        if text == "ComptimeStmt":
            return ("named", "CtStmt")
        match = re.fullmatch(r"std::(\w+)<(.*)>", text)
        if match:
            kind, inner = match.group(1), match.group(2)
            if kind == "shared_ptr":
                return ("ptr", inner.strip())
            if kind == "optional":
                return ("opt", self.parse_type(inner, owner, field))
            if kind == "vector":
                return ("vec", self.parse_type(inner, owner, field))
            if kind == "variant":
                name = owner + "".join(part.capitalize() for part in field.split("_"))
                self.add_variant(name, inner)
                return ("named", name)
        if re.fullmatch(r"\w+", text):
            return ("named", text)
        raise SystemExit(f"unsupported type {text!r} in {owner}.{field}")

    def add_variant(self, name: str, inner: str) -> None:
        if name in self.variants:
            return
        self.variants[name] = [self.parse_type(alt, name, "alt") for alt in split_args(inner)]
        self.order.append(("variant", name))

    def parse(self, text: str) -> None:
        events = []
        for match in re.finditer(r"\benum class (\w+)\s*\{([^}]*)\}", text):
            events.append((match.start(), "enum", match))
        for match in re.finditer(r"\bstruct (\w+)\s*\{", text):
            events.append((match.start(), "struct", match))
        for match in re.finditer(r"\busing (\w+) = ([^;]+);", text):
            events.append((match.start(), "using", match))
        for start, kind, match in sorted(events, key=lambda e: e[0]):
            if depth_at(text, start) != 1:
                continue
            name = match.group(1)
            if kind == "enum":
                if name not in self.enums:
                    self.enums[name] = [v.strip() for v in match.group(2).split(",") if v.strip()]
                    self.order.append(("enum", name))
            elif kind == "struct":
                open_index = match.end() - 1
                body = text[open_index + 1:match_brace(text, open_index)]
                if name not in self.structs:
                    self.structs[name] = []
                    self.structs[name] = self.parse_fields(name, body)
                    self.order.append(("struct", name))
            else:
                target = " ".join(match.group(2).split())
                variant = re.fullmatch(r"std::variant<(.*)>", target)
                if variant:
                    self.add_variant(name, variant.group(1))
                elif name not in STRING_ALIASES | PATH_ALIASES | SPAN_ALIASES | set(PTR_ALIASES) and name not in (
                        "DocKind", "DocComment", "DocList", "Token", "TokenKind", "TupleIndex",
                        "AttributeList", "AttrOpt", "ComptimeStmt"):
                    raise SystemExit(f"unhandled alias {name} = {target}")

    def parse_fields(self, owner: str, body: str):
        fields, chunk, index = [], "", 0
        while index < len(body):
            ch = body[index]
            if ch == "{":
                index = match_brace(body, index) + 1
                chunk = ""
                continue
            if ch == ";":
                statement = " ".join(chunk.split())
                chunk = ""
                index += 1
                if not statement or "(" in statement.split("=")[0] or statement.startswith("friend"):
                    continue
                match = re.fullmatch(r"(.+?)\s+(\w+)(?:\s*=\s*(.+))?", statement)
                if not match:
                    raise SystemExit(f"unparsed field in {owner}: {statement!r}")
                fields.append((match.group(2), self.parse_type(match.group(1), owner, match.group(2)), match.group(3)))
                continue
            chunk += ch
            index += 1
        return fields


def rust_field(name: str) -> str:
    return "r#" + name if name in RUST_KEYWORDS else name


def alt_name(ty) -> str:
    if ty[0] == "named":
        return ty[1]
    if ty[0] == "token":
        return "Token"
    if ty == ("vec", ("string",)):
        return "Path"
    if ty[0] == "vec" and ty[1][0] == "named":
        return ty[1][1] + "List"
    raise SystemExit(f"no variant name for {ty}")


def rust_type(ty) -> str:
    kind = ty[0]
    if kind == "string":
        return "String"
    if kind == "span":
        return "Span"
    if kind == "bool":
        return "bool"
    if kind == "usize":
        return "usize"
    if kind == "u128":
        return "u128"
    if kind == "token":
        return "Token"
    if kind == "doc":
        return "DocComment"
    if kind == "ptr":
        return f"Option<Arc<{ty[1]}>>"
    if kind == "opt":
        return f"Option<{rust_type(ty[1])}>"
    if kind == "vec":
        return f"Vec<{rust_type(ty[1])}>"
    return ty[1]


def rust_default(ty, init: str | None) -> str:
    if init is None or init == "std::nullopt":
        return "Default::default()"
    if init in ("false", "true"):
        return init
    match = re.fullmatch(r"(\w+)::(\w+)", init)
    if match:
        return f"{match.group(1)}::{match.group(2)}"
    raise SystemExit(f"unsupported initializer {init!r}")


def generate_rust(schema: Schema) -> str:
    out = [
        "// Generated by tools/gen_ast.py from the reference AST headers. Do not edit.",
        "",
        "#![allow(clippy::large_enum_variant)]",
        "",
        "use std::sync::Arc;",
        "",
        "use uv_core::span::Span;",
        "",
        "use super::dump::AstDump;",
        "use super::walk::ExprWalk;",
        "use crate::lexer::{DocComment, Token};",
        "",
    ]
    for kind, name in schema.order:
        if kind == "enum":
            values = schema.enums[name]
            out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]")
            out.append(f"pub enum {name} {{")
            for index, value in enumerate(values):
                if index == 0:
                    out.append("    #[default]")
                out.append(f"    {value},")
            out.append("}")
            out.append("")
            out.append(f"impl ExprWalk for {name} {{")
            out.append("    fn walk_exprs(&self, _out: &mut Vec<*const Expr>) {}")
            out.append("}")
            out.append("")
            out.append(f"impl AstDump for {name} {{")
            out.append("    fn dump(&self, out: &mut String) {")
            out.append("        out.push_str(match self {")
            for value in values:
                out.append(f"            {name}::{value} => \"{value}\",")
            out.append("        });")
            out.append("    }")
            out.append("}")
            out.append("")
        elif kind == "struct":
            fields = schema.structs[name]
            out.append("#[derive(Debug, Clone, PartialEq)]")
            if fields:
                out.append(f"pub struct {name} {{")
                for field, ty, _ in fields:
                    out.append(f"    pub {rust_field(field)}: {rust_type(ty)},")
                out.append("}")
            else:
                out.append(f"pub struct {name} {{}}")
            out.append("")
            out.append(f"impl Default for {name} {{")
            out.append("    fn default() -> Self {")
            if fields:
                out.append(f"        {name} {{")
                for field, ty, init in fields:
                    out.append(f"            {rust_field(field)}: {rust_default(ty, init)},")
                out.append("        }")
            else:
                out.append(f"        {name} {{}}")
            out.append("    }")
            out.append("}")
            out.append("")
            out.append(f"impl ExprWalk for {name} {{")
            out.append(f"    fn walk_exprs(&self, {'out' if fields or name == 'Expr' else '_out'}: &mut Vec<*const Expr>) {{")
            if name == "Expr":
                out.append("        out.push(std::ptr::from_ref(self));")
            for field, _, _ in fields:
                out.append(f"        self.{rust_field(field)}.walk_exprs(out);")
            out.append("    }")
            out.append("}")
            out.append("")
            out.append(f"impl AstDump for {name} {{")
            out.append("    fn dump(&self, out: &mut String) {")
            out.append(f"        out.push_str(\"({name}\");")
            for field, _, _ in fields:
                out.append(f"        out.push_str(\" {field}=\");")
                out.append(f"        self.{rust_field(field)}.dump(out);")
            out.append("        out.push(')');")
            out.append("    }")
            out.append("}")
            out.append("")
        else:
            alts = schema.variants[name]
            out.append("#[derive(Debug, Clone, PartialEq)]")
            out.append(f"pub enum {name} {{")
            for alt in alts:
                out.append(f"    {alt_name(alt)}({rust_type(alt)}),")
            out.append("}")
            out.append("")
            out.append(f"impl Default for {name} {{")
            out.append("    fn default() -> Self {")
            out.append(f"        {name}::{alt_name(alts[0])}(Default::default())")
            out.append("    }")
            out.append("}")
            out.append("")
            out.append(f"impl ExprWalk for {name} {{")
            out.append("    fn walk_exprs(&self, out: &mut Vec<*const Expr>) {")
            out.append("        match self {")
            for alt in alts:
                out.append(f"            {name}::{alt_name(alt)}(value) => value.walk_exprs(out),")
            out.append("        }")
            out.append("    }")
            out.append("}")
            out.append("")
            out.append(f"impl AstDump for {name} {{")
            out.append("    fn dump(&self, out: &mut String) {")
            out.append("        match self {")
            for alt in alts:
                out.append(f"            {name}::{alt_name(alt)}(value) => value.dump(out),")
            out.append("        }")
            out.append("    }")
            out.append("}")
            out.append("")
            for alt in alts:
                out.append(f"impl From<{rust_type(alt)}> for {name} {{")
                out.append(f"    fn from(value: {rust_type(alt)}) -> Self {{")
                out.append(f"        {name}::{alt_name(alt)}(value)")
                out.append("    }")
                out.append("}")
                out.append("")
    return "\n".join(out)


def generate_cpp(schema: Schema) -> str:
    out = ["// Generated by tools/gen_ast.py from the reference AST headers. Do not edit.", ""]
    for kind, name in schema.order:
        if kind == "enum":
            out.append(f"void D(std::ostream& o, ast::{name} v) {{")
            out.append("  switch (v) {")
            for value in schema.enums[name]:
                out.append(f"    case ast::{name}::{value}: o << \"{value}\"; return;")
            out.append("  }")
            out.append("  o << \"?\";")
            out.append("}")
        elif kind == "struct":
            out.append(f"void D(std::ostream& o, const ast::{name}& v);")
    out.append("")
    for kind, name in schema.order:
        if kind != "struct":
            continue
        out.append(f"void D(std::ostream& o, const ast::{name}& v) {{")
        out.append("  (void)v;")
        out.append(f"  o << \"({name}\";")
        for field, _, _ in schema.structs[name]:
            out.append(f"  o << \" {field}=\";")
            out.append(f"  D(o, v.{field});")
        out.append("  o << ')';")
        out.append("}")
    out.append("")
    return "\n".join(out)


def generate_cpp_walk(schema: Schema) -> str:
    out = ["// Generated by tools/gen_ast.py from the reference AST headers. Do not edit.", ""]
    structs = [name for kind, name in schema.order if kind == "struct"]
    for name in structs:
        out.append(f"void W(ExprSet& s, const ast::{name}& v);")
    out.append("")
    for name in structs:
        out.append(f"void W(ExprSet& s, const ast::{name}& v) {{")
        out.append("  (void)s;")
        out.append("  (void)v;")
        if name == "Expr":
            out.append("  s.insert(&v);")
        for field, _, _ in schema.structs[name]:
            out.append(f"  W(s, v.{field});")
        out.append("}")
    out.append("")
    return "\n".join(out)


def main() -> int:
    schema = Schema()
    schema.parse(load())
    RUST_OUT.parent.mkdir(parents=True, exist_ok=True)
    RUST_OUT.write_text(generate_rust(schema), encoding="utf-8")
    CPP_OUT.write_text(generate_cpp(schema), encoding="utf-8")
    CPP_WALK_OUT.write_text(generate_cpp_walk(schema), encoding="utf-8")
    print(f"enums={len(schema.enums)} structs={len(schema.structs)} variants={len(schema.variants)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
