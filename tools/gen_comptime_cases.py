#!/usr/bin/env python3
"""Generate compile-time (phase 2) edge-case projects under target/ctcases and their list.

Each case is a small project whose compile-time code exercises one area of the pass:
the evaluator, quoting and splicing, hygiene, emission, reflection, project files, derive
targets and the diagnostics each of them reports.
"""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "ctcases"

APP = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'
LIB = '[[assembly]]\nname = "Lib"\nkind = "library"\nroot = "Lib"\n'
MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"

TYPES = """#reflect
internal record Pair<TFirst; TSecond = i32> {
    public first: TFirst
    internal second: TSecond
    private third: [u8; 4]
    internal procedure sum(~) -> i32 {
        return 0
    }
}

#reflect
internal enum Shape {
    Empty
    Circle(f32)
    Rect { width: f32, height: f32 }
}

#reflect
internal modal Door {
    @Open {
        internal width: i32
        internal procedure peek(~) -> i32 {
            return self.width
        }
        internal transition close() -> @Closed {
            return Door@Closed {}
        }
    }
    @Closed {}
}

internal record Hidden {
    internal value: i32
}

internal class Base {}
internal class Middle <: Base {}
internal class Leaf <: Middle {}

internal record Impl <: Leaf {
    internal value: i32
}

internal type PairAlias = Pair<bool>
internal type Chain = PairAlias
internal type Loop1 = Loop2
internal type Loop2 = Loop1
"""


def body(code: str, attrs: str = "") -> str:
    indented = "\n".join("        " + line if line else "" for line in code.strip("\n").split("\n"))
    return f"internal procedure run() -> bool {{\n    {attrs}comptime {{\n{indented}\n    }}\n    return true\n}}\n" + MAIN


def emit(code: str) -> str:
    return body(code, "#emit ")


def expr(code: str, prefix: str = "") -> str:
    return prefix + f"internal procedure run() {{\n    let value = {code}\n}}\n" + MAIN


CASES = {
    # ---- evaluator -----------------------------------------------------------
    "eval_arith": expr("comptime { 2usize + 3usize * 4usize }"),
    "eval_wrap": expr("comptime { 18446744073709551615 + 2 }"),
    "eval_suffix": expr("comptime { 1_000u8 + 0x10 }"),
    "eval_compare": expr("comptime { (1 < 2, 2 <= 2, 3 > 4, 4 >= 4, 5 == 5, 6 != 6) }"),
    "eval_bool": expr("comptime { (true && false, true || false, true == true, true != false) }"),
    "eval_string": expr('comptime { ("a" == "a", "a" != "b") }'),
    "eval_string_value": expr('comptime { "text" }'),
    "eval_char_float": expr("comptime { ('c', 1.5f32) }"),
    "eval_unit": expr("comptime { () }"),
    "eval_array": expr("comptime { [1, 2, 3] }"),
    "eval_array_repeat": expr("comptime { [7u8; 3] }"),
    "eval_array_repeat_bad": expr("comptime { [7u8; true] }"),
    "eval_record": expr("comptime { Hidden { value: 3 } }", TYPES),
    "eval_enum": expr("comptime { (Shape::Empty, Shape::Circle(1.0f32), Shape::Rect { width: 1.0f32, height: 2.0f32 }) }", TYPES),
    "eval_enum_eq": expr("comptime { (Shape::Empty == Shape::Empty, Shape::Empty != Shape::Circle(1.0f32)) }", TYPES),
    "eval_modal": expr("comptime { Door@Open { width: 3 } }", TYPES),
    "eval_field": expr("comptime { Hidden { value: 3 }.value }", TYPES),
    "eval_field_missing": expr("comptime { Hidden { value: 3 }.other }", TYPES),
    "eval_if": expr("comptime { if 1 < 2 { 10 } else { 20 } }"),
    "eval_if_no_else": expr("comptime { if false { 10 } }"),
    "eval_if_is": expr("comptime { if (1, true) is (n, true) { n } else { 0 } }"),
    "eval_if_is_enum": expr("comptime { if Shape::Circle(2.0f32) is Shape::Circle(r) { r } else { 0.0f32 } }", TYPES),
    "eval_if_is_record": expr("comptime { if (Hidden { value: 3 }) is Hidden { value } { value } else { 0 } }", TYPES),
    "eval_if_is_modal": expr("comptime { if (Door@Open { width: 3 }) is @Open { width: w } { w } else { 0 } }", TYPES),
    "eval_if_case": expr("comptime { if 5 is {\n        0..3 { 1 }\n        3..=5 { 2 }\n        _ { 3 }\n    } }"),
    "eval_if_case_literal": expr('comptime { if "b" is {\n        "a" { 1 }\n        "b" { 2 }\n        else { 3 }\n    } }'),
    "eval_block_let": expr("comptime { { let a = 1\n        var b: usize = a + 1\n        let (c, d) = (b, a)\n        c + d } }"),
    "eval_return": "comptime internal procedure pick(flag: bool) -> usize {\n    if flag {\n        return 1usize\n    }\n    return 2usize\n}\n" + expr("comptime { pick(true) + pick(false) }"),
    "eval_unknown_name": expr("comptime { missing + 1 }"),
    "eval_unknown_call": expr("comptime { missing(1) }"),
    "eval_arity": "comptime internal procedure one(a: usize) -> usize {\n    return a\n}\n" + expr("comptime { one(1, 2) }"),
    "eval_contract_pre_fail": "comptime internal procedure pos(a: usize) -> usize\n|: a > 0usize\n{\n    return a\n}\n" + expr("comptime { pos(0usize) }"),
    "eval_contract_post_fail": "comptime internal procedure grow(a: usize) -> usize\n|: a > 0usize |= @result > @entry(a)\n{\n    return a\n}\n" + expr("comptime { grow(1usize) }"),
    "eval_contract_ok": "comptime internal procedure grow(a: usize) -> usize\n|: a > 0usize |= @result > @entry(a)\n{\n    return a + 1usize\n}\n" + expr("comptime { grow(1usize) }"),
    "eval_runtime_call_of_ct_proc": "comptime internal procedure one() -> usize {\n    return 1usize\n}\n" + expr("one()"),
    "eval_prohibited_unsafe": body("unsafe {\n}\n1"),
    "eval_prohibited_deref": body("let p = 1\nlet q = *p"),
    "eval_prohibited_region": body("region {\n}"),
    "eval_prohibited_spawn": body("let h = spawn {\n}"),
    "ct_if_true": expr("comptime if true { 1 } else { missing() }"),
    "ct_if_false_no_else": "internal procedure run() {\n    var total = 0\n    comptime if false {\n        total = total + missing()\n    }\n}\n" + MAIN,
    "ct_if_not_bool": expr("comptime if 1 { 1 } else { 2 }"),
    "ct_if_fail": expr("comptime if missing { 1 } else { 2 }"),
    "ct_if_chain": expr("comptime if false { 1 } else comptime if false { 2 } else { 3 }"),
    "ct_loop": "internal procedure run() {\n    var total = 0\n    comptime loop v in [1, 2, 3] {\n        total = total + comptime { v }\n        total\n    }\n}\n" + MAIN,
    "ct_loop_tuple": "internal procedure run() {\n    var total = 0\n    comptime loop (a, b) in [(1, 2), (3, 4)] {\n        total = total + comptime { a + b }\n    }\n}\n" + MAIN,
    "ct_loop_empty": "internal procedure run() {\n    var total = 0\n    comptime loop v: usize in [0usize; 0] {\n        total = total + 1\n    }\n}\n" + MAIN,
    "ct_loop_not_iterable": "internal procedure run() {\n    comptime loop v in 3 {\n    }\n}\n" + MAIN,
    "ct_loop_fail": "internal procedure run() {\n    comptime loop v in missing {\n    }\n}\n" + MAIN,
    "ct_loop_pattern_mismatch": "internal procedure run() {\n    comptime loop (a, b) in [1, 2] {\n    }\n}\n" + MAIN,
    "ct_nested_static": "internal let LIMIT: usize = comptime { 4usize + 4usize }\n" + MAIN,
    "ct_literalize_values": expr("comptime { (Hidden { value: 1 }, Shape::Rect { width: 1.0f32, height: 2.0f32 }, Door@Open { width: 2 }, [1, 2]) }", TYPES),
    # ---- diagnostics ---------------------------------------------------------
    "diag_error": body('diagnostics~>error("stop here")'),
    "diag_warning": body('diagnostics~>warning("careful")'),
    "diag_note": body('diagnostics~>note("fyi")\ndiagnostics~>warning("careful")'),
    "diag_module_and_span": expr("comptime { (diagnostics~>current_module(), diagnostics~>current_span()) }"),
    "diag_bad_arg": body("diagnostics~>error(3)"),
    # ---- quoting, splicing, emission --------------------------------------------
    "quote_expr_value": expr("comptime { quote { 1 + 2 } }"),
    "quote_expr_hygiene": expr("comptime { quote { { let a = 1\n        let b = a + 1\n        b } } }"),
    "quote_type_kind": emit("let t: Ast::Type = quote type { (i32, bool) }\nlet item: Ast::Item = quote {\n    internal type Made = $(t)\n}\nemitter~>emit(item)"),
    "quote_pattern_kind": emit("let p: Ast::Pattern = quote pattern { (left, right) }\nlet item: Ast::Item = quote {\n    internal procedure made() -> i32 {\n        let $(p) = (1, 2)\n        return left + right\n    }\n}\nemitter~>emit(item)"),
    "quote_stmt_kind": emit("let s: Ast::Stmt = quote { let made = 1 }\nlet item: Ast::Item = quote {\n    internal procedure made_proc() -> i32 {\n        $(s);\n        return 0\n    }\n}\nemitter~>emit(item)"),
    "quote_invalid": emit("let item: Ast::Item = quote { let }\nemitter~>emit(item)"),
    "quote_ambiguous": body("let q = quote { value }"),
    "quote_not_matching_kind": emit("let item: Ast::Item = quote { 1 + 2 }\nemitter~>emit(item)"),
    "emit_twice_same_value": emit("let item: Ast::Item = quote {\n    internal procedure twice() -> i32 {\n        let local: i32 = 1\n        return local\n    }\n}\nemitter~>emit(item)\nemitter~>emit(item)"),
    "emit_shared_splice": emit("let common: Ast::Expr = quote { { let inner = 2\n    inner } }\nlet a: Ast::Item = quote {\n    internal procedure first() -> i32 {\n        return $(common)\n    }\n}\nlet b: Ast::Item = quote {\n    internal procedure second() -> i32 {\n        return $(common) + $(common)\n    }\n}\nemitter~>emit(a)\nemitter~>emit(b)\nemitter~>emit(a)"),
    "emit_non_item": emit("let e: Ast::Expr = quote { 1 + 2 }\nemitter~>emit(e)"),
    "emit_without_capability": body("let item: Ast::Item = quote {\n    internal procedure nope() {\n    }\n}\nemitter~>emit(item)"),
    "emit_non_ast": emit("emitter~>emit(3)"),
    "emit_many_forms": emit("""let item: Ast::Item = quote {
    internal procedure many<TValue>(input: TValue, count: usize) -> usize
    |: count > 0usize |= @result >= count
    {
        var total: usize = count
        let (left, right) = (1usize, 2usize)
        let closure = |step: usize| step + total
        loop index in 0usize..count {
            total = total + closure(index)
        }
        loop total < 10usize {
            total = total + left
        }
        if total is 3usize {
            total = right
        }
        let picked: usize = if total is {
            0usize..5usize { 1usize }
            other { other }
        }
        region as scratch {
            let scoped = scratch ^ total
        }
        using picked as renamed
        return total + picked
    }
}
emitter~>emit(item)"""),
    "emit_type_items": emit("""let rec: Ast::Item = quote {
    internal record Made<TItem> <: Base {
        internal value: TItem = 0
        internal procedure get(~, extra: i32) -> i32 {
            let local = extra
            return local
        }
        type Out = TItem
    }
}
let en: Ast::Item = quote {
    internal enum MadeEnum<TItem> {
        Empty
        One(TItem)
        Named { value: TItem }
    }
}
let mo: Ast::Item = quote {
    internal modal MadeModal<TItem> {
        @A {
            internal value: TItem
            internal procedure get(~, extra: i32) -> i32 {
                return extra
            }
            internal transition go(amount: i32) -> @B {
                let local = amount
                return MadeModal@B {}
            }
        }
        @B {}
    }
}
let cl: Ast::Item = quote {
    internal class MadeClass<TItem> {
        procedure need(~, input: TItem) -> i32
        procedure have(~, input: i32) -> i32 {
            let local = input
            return local
        }
        type Assoc = TItem
        field: TItem
        @State {
            inner: TItem
        }
    }
}
let al: Ast::Item = quote {
    internal type MadeAlias<TItem> = (TItem, MadeEnum<TItem>)
}
let st: Ast::Item = quote {
    internal let made_static: i32 = { let inner = 4
        inner }
}
let ex: Ast::Item = quote {
    extern "C" {
        procedure made_extern<TItem>(input: i32) -> i32
    }
}
let us: Ast::Item = quote {
    using App::Shape as MadeShape
}
let im: Ast::Item = quote {
    import App as made_import
}
emitter~>emit(rec)
emitter~>emit(en)
emitter~>emit(mo)
emitter~>emit(cl)
emitter~>emit(al)
emitter~>emit(st)
emitter~>emit(ex)
emitter~>emit(us)
emitter~>emit(im)""").replace("internal procedure run()", TYPES + "internal procedure run()"),
    "emit_nested_comptime": emit("let item: Ast::Item = quote {\n    internal procedure nested() -> usize {\n        return comptime { 1usize + 1usize }\n    }\n}\nemitter~>emit(item)"),
    "emit_emits_emitter": emit("let item: Ast::Item = quote {\n    internal procedure outer() -> bool {\n        #emit comptime {\n            let inner: Ast::Item = quote {\n                internal procedure innermost() -> i32 {\n                    return 5\n                }\n            }\n            emitter~>emit(inner)\n        }\n        return true\n    }\n}\nemitter~>emit(item)"),
    "splice_literal_values": emit('let n = 5usize\nlet s = "text"\nlet b = true\nlet item: Ast::Item = quote {\n    internal procedure lit() -> usize {\n        let a = $(s)\n        let c = $(b)\n        return $(n)\n    }\n}\nemitter~>emit(item)'),
    "splice_ident": emit('let name = "chosen"\nlet item: Ast::Item = quote {\n    internal procedure ident_proc($name: i32) -> i32 {\n        let other: i32 = $name + 1\n        return $name + other\n    }\n}\nemitter~>emit(item)'),
    "splice_ident_invalid": emit('let name = "not valid"\nlet item: Ast::Item = quote {\n    internal procedure bad() -> i32 {\n        return $name\n    }\n}\nemitter~>emit(item)'),
    "splice_ident_not_string": emit("let name = 3\nlet item: Ast::Item = quote {\n    internal procedure bad() -> i32 {\n        return $name\n    }\n}\nemitter~>emit(item)"),
    "splice_eval_fail": emit("let item: Ast::Item = quote {\n    internal procedure bad() -> i32 {\n        return $(missing)\n    }\n}\nemitter~>emit(item)"),
    "splice_context_mismatch": emit("let t: Ast::Type = quote type { i32 }\nlet item: Ast::Item = quote {\n    internal procedure bad() -> i32 {\n        return $(t)\n    }\n}\nemitter~>emit(item)"),
    "splice_type_from_type_literal": emit("let t = (Type::<(i32, bool)>)\nlet item: Ast::Item = quote {\n    internal type FromLiteral = $(t)\n}\nemitter~>emit(item)"),
    "splice_func_type": emit("let t: Ast::Type = quote type { i32 }\nlet item: Ast::Item = quote {\n    internal type Fn0 = () -> $(t)\n}\nlet item2: Ast::Item = quote {\n    internal type Fn2 = ($(t), $(t)) -> $(t)\n}\nemitter~>emit(item)\nemitter~>emit(item2)"),
    "splice_whole_body": emit("let inner: Ast::Item = quote {\n    internal procedure whole() -> i32 {\n        return 1\n    }\n}\nlet outer: Ast::Item = quote { $(inner) }\nemitter~>emit(outer)"),
    "splice_stmt_tail": emit("let s: Ast::Stmt = quote { return 9 }\nlet item: Ast::Item = quote {\n    internal procedure tail() -> i32 {\n        $(s)\n    }\n}\nemitter~>emit(item)"),
    "comptime_expr_non_expr_ast": expr("comptime { quote type { i32 } }"),
    "comptime_proc_returns_quote": "comptime internal procedure make() -> Ast::Expr {\n    return quote { 40 + 2 }\n}\n" + expr("comptime { make() }"),
    "comptime_proc_returns_item": "comptime internal procedure make() -> Ast::Item {\n    return quote {\n        internal procedure from_proc() -> i32 {\n            let local = 3\n            return local\n        }\n    }\n}\n" + emit("emitter~>emit(make())\nemitter~>emit(make())"),
    # ---- reflection ----------------------------------------------------------
    "reflect_category": expr("""comptime { (
        introspect~>category(Type::<i32>),
        introspect~>category(Type::<(i32, bool)>),
        introspect~>category(Type::<[u8; 4]>),
        introspect~>category(Type::<[u8]>),
        introspect~>category(Type::<i32 | bool>),
        introspect~>category(Type::<(i32) -> bool>),
        introspect~>category(Type::<|i32| -> bool>),
        introspect~>category(Type::<Ptr<i32>>),
        introspect~>category(Type::<*mut i32>),
        introspect~>category(Type::<$Base>),
        introspect~>category(Type::<string>),
        introspect~>category(Type::<bytes@View>),
        introspect~>category(Type::<Door@Open>),
        introspect~>category(Type::<Pair<bool>>),
        introspect~>category(Type::<Shape>),
        introspect~>category(Type::<Door>),
        introspect~>category(Type::<PairAlias>),
        introspect~>category(Type::<Chain>),
        introspect~>category(Type::<Unknown>),
        introspect~>category(Type::<const Hidden>)
    ) }""", TYPES),
    "reflect_category_unknown_path": expr("comptime { introspect~>category(Type::<A::B>) }", TYPES),
    "reflect_category_alias_cycle": expr("comptime { introspect~>category(Type::<Loop1>) }", TYPES),
    "reflect_type_name": expr("""comptime { (
        introspect~>type_name(Type::<const i32>),
        introspect~>type_name(Type::<unique (i32, bool)>),
        introspect~>type_name(Type::<(i32;)>),
        introspect~>type_name(Type::<[u8; 4]>),
        introspect~>type_name(Type::<[u8]>),
        introspect~>type_name(Type::<i32 | bool | ()>),
        introspect~>type_name(Type::<(i32, move bool) -> !>),
        introspect~>type_name(Type::<|i32, move u8| -> bool [shared: { limit: i32 }]>),
        introspect~>type_name(Type::<Ptr<i32>@Valid>),
        introspect~>type_name(Type::<*imm i32>),
        introspect~>type_name(Type::<$Base>),
        introspect~>type_name(Type::<string@Managed>),
        introspect~>type_name(Type::<bytes>),
        introspect~>type_name(Type::<Door@Open>),
        introspect~>type_name(Type::<Pair<bool, u8>@State>),
        introspect~>type_name(Type::<App::Pair<bool>>),
        introspect~>type_name(Type::<u32 |: { self < 4u32 }>),
        introspect~>type_name(Type::<shared Hidden>)
    ) }""", TYPES),
    "reflect_module_path": expr("comptime { (introspect~>module_path(Type::<App::Pair>), introspect~>module_path(Type::<Pair>), introspect~>module_path(Type::<i32>)) }", TYPES),
    "reflect_fields": expr("comptime { introspect~>fields(Type::<Pair<bool>>) }", TYPES),
    "reflect_fields_generic_default": expr("comptime { introspect~>fields(Type::<Pair<Shape, [Hidden]>>) }", TYPES),
    "reflect_fields_alias": expr("comptime { introspect~>fields(Type::<Chain>) }", TYPES),
    "reflect_fields_too_many_args": expr("comptime { introspect~>fields(Type::<Pair<bool, u8, u8>>) }", TYPES),
    "reflect_fields_not_reflect": expr("comptime { introspect~>fields(Type::<Hidden>) }", TYPES),
    "reflect_fields_wrong_kind": expr("comptime { introspect~>fields(Type::<Shape>) }", TYPES),
    "reflect_fields_structural": expr("comptime { introspect~>fields(Type::<(i32, bool)>) }", TYPES),
    "reflect_variants": expr("comptime { introspect~>variants(Type::<Shape>) }", TYPES),
    "reflect_variants_wrong_kind": expr("comptime { introspect~>variants(Type::<Door>) }", TYPES),
    "reflect_states": expr("comptime { introspect~>states(Type::<Door>) }", TYPES),
    "reflect_states_wrong_kind": expr("comptime { introspect~>states(Type::<Pair<bool>>) }", TYPES),
    "reflect_implements": expr("comptime { (introspect~>implements_form(Type::<Impl>, Type::<Base>), introspect~>implements_form(Type::<Impl>, Type::<Leaf>), introspect~>implements_form(Type::<Hidden>, Type::<Base>), introspect~>implements_form(Type::<Impl>, Type::<i32>), introspect~>implements_form(Type::<i32>, Type::<Base>)) }", TYPES),
    "reflect_bad_args": expr("comptime { introspect~>fields(3) }", TYPES),
    "reflect_unknown_method": expr("comptime { introspect~>nothing(Type::<i32>) }", TYPES),
    "reflect_field_loop": TYPES + "internal procedure run() -> usize {\n    var total: usize = 0usize\n    comptime loop field in introspect~>fields(Type::<Pair<bool>>) {\n        total = total + comptime { field.index }\n        let name = comptime { field.name }\n        let vis = comptime { field.visibility }\n    }\n    return total\n}\n" + MAIN,
    "reflect_sees_emitted_type": emit("let rec: Ast::Item = quote {\n    #reflect\n    internal record Late {\n        internal value: i32\n    }\n}\nemitter~>emit(rec)").replace(MAIN, "internal procedure later() {\n    let fields = comptime { introspect~>fields(Type::<Late>) }\n}\n" + MAIN),
    # ---- project files -------------------------------------------------------
    "files_all": body('let root = files~>project_root()\nlet manifest = files~>read("Ultraviolet.toml")\nlet bytes = files~>read_bytes("Ultraviolet.toml")\nlet yes = files~>exists("Source/Main.uv")\nlet no = files~>exists("Source/Nope.uv")\nlet listing = files~>list_dir("Data")\nlet nested = files~>list_dir("Data/Sub")\nlet missing_dir = files~>list_dir("Nowhere")\nlet not_dir = files~>list_dir("Ultraviolet.toml")\nlet not_file = files~>read("Data")\nlet binary = files~>read("Data/binary.bin")\nlet binary_bytes = files~>read_bytes("Data/binary.bin")\nlet outside = files~>read("../escape.txt")\nlet absolute = files~>exists("/etc/passwd")\nlet outside_list = files~>list_dir("..")', "#files "),
    "files_values": expr('#files comptime { (files~>exists("Data/a.txt"), files~>exists("Data/zz"), files~>read("Data/a.txt"), files~>list_dir("Data"), files~>read("Data/none.txt"), files~>read("../x")) }'),
    "files_without_attribute": body('let manifest = files~>read("Ultraviolet.toml")'),
    "files_bad_arg": body("let manifest = files~>read(3)", "#files "),
    "files_unknown_method": body('let manifest = files~>write("a")', "#files "),
    # ---- attributes on compile-time forms --------------------------------------
    "attr_unknown_on_comptime_stmt": body("let a = 1", "#bogus "),
    "attr_bad_target_on_comptime_stmt": body("let a = 1", "#inline "),
    "attr_args_on_emit": body("let a = 1", "#emit(x) "),
    "attr_unknown_on_comptime_expr": expr("#bogus comptime { 1 }"),
    "attr_on_ct_procedure": "#bogus\ncomptime internal procedure one() -> usize {\n    return 1usize\n}\n" + expr("comptime { one() }"),
}

DERIVE_TARGETS = """internal class Emitted {}
internal class Required {}

derive target First(target: Type) |: emits Emitted {
    let ast: Ast::Item = quote {
        internal procedure first_made() -> i32 {
            let local = 1
            return local
        }
    }
    emitter~>emit(ast)
}

derive target Second(target: Type) |: requires Emitted, emits Required {
    let ast: Ast::Item = quote {
        internal procedure second_made() -> i32 {
            return 2
        }
    }
    emitter~>emit(ast)
}

derive target Plain(target: Type) {
    let name = introspect~>type_name(target)
    let ast: Ast::Item = quote {
        internal procedure plain_made() -> string {
            return $(name)
        }
    }
    emitter~>emit(ast)
}
"""


def derive(decl: str, extra: str = "") -> str:
    return DERIVE_TARGETS + extra + decl + "\n" + MAIN


CASES.update({
    "derive_order": derive("#derive(Second, Plain, First)\ninternal record R <: Emitted, Required {\n    internal value: i32\n}"),
    "derive_enum_modal": derive("#derive(Plain)\ninternal enum E {\n    A\n}\n#derive(First)\ninternal modal M <: Emitted {\n    @S {}\n}"),
    "derive_unknown_target": derive("#derive(Nope)\ninternal record R {\n    internal value: i32\n}"),
    "derive_target_declared_later": "#derive(Plain)\ninternal record R {\n    internal value: i32\n}\n" + DERIVE_TARGETS + MAIN,
    "derive_missing_emits_class": derive("#derive(First)\ninternal record R {\n    internal value: i32\n}"),
    "derive_missing_requires_class": derive("#derive(Second)\ninternal record R <: Required {\n    internal value: i32\n}"),
    "derive_cycle": derive("#derive(CycleA, CycleB)\ninternal record R <: Emitted, Required {\n    internal value: i32\n}", "derive target CycleA(target: Type) |: requires Required, emits Emitted {\n}\nderive target CycleB(target: Type) |: requires Emitted, emits Required {\n}\n"),
    "derive_on_procedure": derive("#derive(Plain)\ninternal procedure not_a_type() {\n}"),
    "derive_on_class": derive("#derive(Plain)\ninternal class NotAllowed {}"),
    "derive_on_alias": derive("#derive(Plain)\ninternal type Alias = i32"),
    "derive_body_restricted": derive("#derive(Bad)\ninternal record R {\n    internal value: i32\n}", "derive target Bad(target: Type) {\n    unsafe {\n    }\n}\n"),
    "derive_body_fails": derive("#derive(Bad)\ninternal record R {\n    internal value: i32\n}", "derive target Bad(target: Type) {\n    let x = missing\n}\n"),
    "derive_body_reports_error": derive("#derive(Bad)\ninternal record R {\n    internal value: i32\n}", 'derive target Bad(target: Type) {\n    diagnostics~>error("no derive for you")\n}\n'),
    "derive_body_warns": derive("#derive(Warns)\ninternal record R {\n    internal value: i32\n}", 'derive target Warns(target: Type) {\n    diagnostics~>warning("derive warning")\n}\n'),
    "derive_reflects_target": derive("#derive(Counts)\n#reflect\ninternal record R {\n    internal a: i32\n    internal b: bool\n}", "derive target Counts(target: Type) {\n    let fields = introspect~>fields(target)\n    let category = introspect~>category(target)\n    let ast: Ast::Item = quote {\n        internal procedure counted() -> bool {\n            return true\n        }\n    }\n    emitter~>emit(ast)\n}\n"),
    "derive_twice_same_target": derive("#derive(Plain)\ninternal record R1 {\n    internal value: i32\n}\n#derive(Plain)\ninternal record R2 {\n    internal value: i32\n}"),
    "derive_emitted_item_derives": derive("#derive(Maker)\ninternal record R {\n    internal value: i32\n}", "derive target Maker(target: Type) {\n    let ast: Ast::Item = quote {\n        #derive(Plain)\n        internal record Made {\n            internal value: i32\n        }\n    }\n    emitter~>emit(ast)\n}\n"),
})

MULTI = {
    "cross_module_reflect": (APP, {
        "Source/Main.uv": "using App::Types::Remote\nusing App::Types::{ RemoteEnum as Renamed }\nusing App::Types::*\ninternal procedure run() {\n    let a = comptime { introspect~>fields(Type::<Remote>) }\n    let b = comptime { introspect~>variants(Type::<Renamed>) }\n    let c = comptime { introspect~>states(Type::<RemoteModal>) }\n    let d = comptime { introspect~>fields(Type::<App::Types::Remote>) }\n    let e = comptime { introspect~>category(Type::<Types::Remote>) }\n    let f = comptime { introspect~>type_name(Type::<Remote>) }\n}\n" + MAIN,
        "Source/Types/t.uv": "#reflect\npublic record Remote {\n    public value: i32\n}\n#reflect\npublic enum RemoteEnum {\n    A\n    B(i32)\n}\n#reflect\npublic modal RemoteModal {\n    @S {}\n}\n#reflect\nprivate record Secret {\n    internal value: i32\n}\n",
    }),
    "cross_module_private": (APP, {
        "Source/Main.uv": "using App::Types::Secret\ninternal procedure run() {\n    let a = comptime { introspect~>fields(Type::<Secret>) }\n}\n" + MAIN,
        "Source/Types/t.uv": "#reflect\nprivate record Secret {\n    internal value: i32\n}\n",
    }),
    "cross_module_emitted_decl": (APP, {
        "Source/Main.uv": "import App::Types\ninternal procedure run() {\n    let a = comptime { introspect~>fields(Type::<Types::Late>) }\n}\n" + MAIN,
        "Source/Types/t.uv": "internal procedure make() -> bool {\n    #emit comptime {\n        let rec: Ast::Item = quote {\n            #reflect\n            public record Late {\n                public value: i32\n            }\n        }\n        emitter~>emit(rec)\n    }\n    return true\n}\n",
    }),
    "cross_assembly_import_required": (APP + LIB, {
        "Source/Main.uv": "internal procedure run() {\n    let a = comptime { introspect~>fields(Type::<Lib::Thing>) }\n}\n" + MAIN,
        "Lib/l.uv": "#reflect\npublic record Thing {\n    public value: i32\n}\n",
    }),
    "cross_assembly_with_import": (APP + LIB, {
        "Source/Main.uv": "import Lib as library\ninternal procedure run() {\n    let a = comptime { introspect~>fields(Type::<Lib::Thing>) }\n    let b = comptime { introspect~>fields(Type::<library::Thing>) }\n    let c = comptime { introspect~>fields(Type::<Lib::Inner>) }\n}\n" + MAIN,
        "Lib/l.uv": "#reflect\npublic record Thing {\n    public value: i32\n}\n#reflect\ninternal record Inner {\n    internal value: i32\n}\n",
    }),
    "derive_target_via_using": (APP, {
        "Source/Main.uv": "using App::Targets::Shared\nusing App::Targets::{ Shared as Again }\nusing App::Targets::*\n#derive(Shared)\ninternal record R {\n    internal value: i32\n}\n#derive(Again)\ninternal record R2 {\n    internal value: i32\n}\n#derive(Other)\ninternal record R3 {\n    internal value: i32\n}\n" + MAIN,
        "Source/Targets/t.uv": "derive target Shared(target: Type) {\n    let ast: Ast::Item = quote {\n        internal procedure shared_made() -> i32 {\n            return 1\n        }\n    }\n    emitter~>emit(ast)\n}\nderive target Other(target: Type) {\n}\n",
    }),
    "derive_target_ambiguous": (APP, {
        "Source/Main.uv": "using App::A::*\nusing App::B::*\n#derive(Same)\ninternal record R {\n    internal value: i32\n}\n" + MAIN,
        "Source/A/a.uv": "derive target Same(target: Type) {\n}\n",
        "Source/B/b.uv": "derive target Same(target: Type) {\n}\n",
    }),
    "ct_proc_not_shared_across_modules": (APP, {
        "Source/Main.uv": "internal procedure run() {\n    let a = comptime { helper() }\n}\n" + MAIN,
        "Source/Other/o.uv": "comptime internal procedure helper() -> usize {\n    return 1usize\n}\n",
    }),
    "hygiene_seed_across_modules": (APP, {
        "Source/Main.uv": "internal procedure run() -> bool {\n    #emit comptime {\n        let item: Ast::Item = quote {\n            internal procedure main_made() -> i32 {\n                let local = 1\n                return local\n            }\n        }\n        emitter~>emit(item)\n    }\n    return true\n}\n" + MAIN,
        "Source/Other/o.uv": "internal procedure run2() -> bool {\n    #emit comptime {\n        let item: Ast::Item = quote {\n            internal procedure other_made() -> i32 {\n                let local = 1\n                let second = local\n                return second\n            }\n        }\n        emitter~>emit(item)\n    }\n    return true\n}\n",
    }),
}

FILE_DATA = {
    "Data/a.txt": "alpha\n",
    "Data/b.txt": "beta\n",
    "Data/Sub/c.txt": "gamma\n",
    "Data/binary.bin": b"\xff\xfe\x00\x01",
}


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    cases = {name: (APP, {"Source/Main.uv": text}) for name, text in CASES.items()}
    cases.update(MULTI)
    entries = []
    for name, (manifest, files) in sorted(cases.items()):
        case_dir = OUT / name
        case_dir.mkdir(parents=True)
        (case_dir / "Ultraviolet.toml").write_text(manifest, encoding="utf-8")
        all_files = dict(files)
        if name.startswith("files_"):
            all_files.update(FILE_DATA)
        for rel, content in all_files.items():
            path = case_dir / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            if isinstance(content, bytes):
                path.write_bytes(content)
            else:
                path.write_text(content, encoding="utf-8", newline="")
        entries.append(f"target/ctcases/{name}/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "comptime_cases.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} compile-time cases")


if __name__ == "__main__":
    main()
