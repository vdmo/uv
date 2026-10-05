#!/usr/bin/env python3
"""Generate projects under target/patcases that pin pattern typing.

Each case holds one procedure whose `if … is` clauses and `let` bindings carry the
patterns, and aliases for the types they are checked against: every pattern of a case is
typed against every type of the case.
"""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "patcases"

APP = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'
MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"

BASE = """internal record Point {
    public x: i32
    public y: i32
}

internal record Hidden {
    public shown: i32
    private secret: bool
}

internal record Box<T> {
    public value: T
}

internal enum Shape {
    Empty
    Circle(i32)
    Pair(i32, bool)
    Rect { w: i32, h: i32 }
}

internal enum Opt<T> {
    None
    Some(T)
    Both { left: T, right: (T, bool) }
}

internal modal Door {
    @Open {
        public width: i32
        public label: (i32, bool)
    }
    @Closed {}
}

internal modal Cell<T> {
    @Full {
        public value: T
    }
    @Empty {}
}

"""


def case(types: list[str], clauses: list[str], lets: list[str] = ()) -> str:
    aliases = "".join(f"internal type T{index} = {ty}\n" for index, ty in enumerate(types))
    arms = "".join(f"        {clause} {{\n            0\n        }}\n" for clause in clauses)
    bindings = "".join(f"    let {binding} = input\n" for binding in lets)
    body = f"    let chosen = if input is {{\n{arms}        else {{\n            0\n        }}\n    }}\n" if clauses else ""
    return BASE + aliases + f"internal procedure run(input: i32) -> i32 {{\n{bindings}{body}    return 0\n}}\n" + MAIN


CASES = {
    "literals": case(
        ["i32", "i8", "u8", "u64", "f32", "f64", "bool", "char", "string@View", "string", "()", "unique i32", "i32 | bool",
         "i32 |: { self > 0 }", "(i32, bool)"],
        ["1", "300", "1u8", "1i64", "1.5", "1.5f64", "true", "'a'", '"text"', "0x1F", "99999999999"]),
    "bindings": case(
        ["i32", "unique i32", "shared Point", "Point", "(i32, bool)", "i32 | bool", "const (i32, bool)", "()", "Shape",
         "unique (i32, Point)"],
        ["_", "x", "x: i32", "x: bool", "_: i32", "p: Point", "t: (i32, bool)", "x: unique i32", "u: i32 | bool"]),
    "tuples": case(
        ["()", "(i32;)", "(i32, bool)", "(i32, bool, char)", "((i32, bool), char)", "unique (i32, bool)", "(Point, Shape)",
         "(i32, bool) | (i32, char)", "(i32, i32)", "(i32, (i32, i32))"],
        ["()", "(a;)", "(a, b)", "(a, b, c)", "((a, b), c)", "(_, _)", "(a, _)", "(1, b)", "(a, true)", "(a: i32, b: bool)",
         "(p, Shape::Empty)", "(a, (b, c))", "(1..5, b)"],
        ["(lp, lq)", "(_, _)", "(first, (second, third))"]),
    "records": case(
        ["Point", "unique Point", "Hidden", "Box<i32>", "Box<Point>", "Box<(i32, bool)>", "Shape", "(i32, i32)",
         "Point | Hidden", "shared Box<i32>"],
        ["Point { x, y }", "Point { x }", "Point { x: 1, y }", "Point { x: a, y: b }", "Point { z }", "Point {}",
         "Hidden { shown }", "Hidden { secret }", "Hidden { shown, secret }", "Box { value }", "Box { value: Point { x, y } }",
         "Box { value: (a, b) }", "Box { value: 1 }", "Point { x: _, y: _ }", "Point { x: n: i32, y }"],
        ["Point { x: lx, y: ly }", "Box { value: lv }"]),
    "enums": case(
        ["Shape", "unique Shape", "Opt<i32>", "Opt<Point>", "Opt<(i32, bool)>", "Opt", "Opt<i32, bool>", "Point",
         "Shape | i32", "Opt<Shape>", "const Opt<i32>"],
        ["Shape::Empty", "Shape::Circle(r)", "Shape::Circle(1)", "Shape::Circle(_)", "Shape::Circle", "Shape::Empty(x)",
         "Shape::Pair(a, b)", "Shape::Pair(a)", "Shape::Pair(a, true)", "Shape::Rect { w, h }", "Shape::Rect { w }",
         "Shape::Rect { w: 1, h }", "Shape::Rect { depth }", "Shape::Rect(a, b)", "Shape::Circle { r }", "Shape::Missing",
         "Opt::None", "Opt::Some(v)", "Opt::Some(Point { x, y })", "Opt::Some((a, b))", "Opt::Some(Shape::Circle(r))",
         "Opt::Both { left, right }", "Opt::Both { left, right: (a, b) }", "Opt::Both { left: 1, right }",
         "Opt::Some(1)", "Opt::Some(_)"]),
    "modals": case(
        ["Door", "Door@Open", "Door@Closed", "unique Door", "Cell<i32>", "Cell<i32>@Full", "Cell<Point>", "Cell<i32>@Empty",
         "Point", "Door | i32", "Cell<(i32, bool)>@Full"],
        ["@Open", "@Closed", "@Open { width }", "@Open { width, label }", "@Open { width: 1 }", "@Open { label: (a, b) }",
         "@Open { missing }", "@Closed { width }", "@Full", "@Full { value }", "@Full { value: Point { x, y } }",
         "@Full { value: (a, b) }", "@Empty", "@Missing", "@Open { width: _ }", "d: Door@Open", "c: Cell<i32>@Full",
         "c: Cell<bool>@Full"]),
    "ranges": case(
        ["i32", "u8", "i64", "usize", "char", "f32", "bool", "unique i32", "i32 | u8", "(i32, i32)"],
        ["1..5", "1..=5", "5..1", "5..=5", "5..5", "0..=255", "0x10..0x20", "1u8..5u8", "1..=1",
         "340282366920938463463374607431768211455..=340282366920938463463374607431768211455", "0..1_000"]),
    "unions": case(
        ["i32 | bool", "Point | Shape", "i32 | (i32, bool)", "(i32, bool) | (i32, char)", "Opt<i32> | Opt<bool>",
         "Point | Hidden | i32", "Door@Open | Door@Closed", "string@View | i32", "Shape | Opt<i32>",
         "(i32, Point) | (bool, Point)"],
        ["x: i32", "x: bool", "x: char", "1", "true", "(a, b)", "(a, 1)", "Point { x, y }", "Shape::Circle(r)", "Opt::Some(v)",
         "@Open { width }", "@Closed", "p: Point", "_", "whole", '"text"', "(a, p)", "Opt::None"]),
}


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    entries = []
    for name, text in sorted(CASES.items()):
        case_dir = OUT / name
        (case_dir / "Source").mkdir(parents=True)
        (case_dir / "Ultraviolet.toml").write_text(APP, encoding="utf-8")
        (case_dir / "Source" / "Main.uv").write_text(text, encoding="utf-8", newline="")
        entries.append(f"target/patcases/{name}/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "patterns_extra.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} pattern cases")


if __name__ == "__main__":
    main()
