#!/usr/bin/env python3
"""Generate name-resolution edge-case projects under target/rescases and their list.

Each case is a small project aimed at one resolver rule: unresolved names and the
suggestions that go with them, scopes and shadowing, qualified forms, patterns, `using`
and `import`, module-level name collection and the reserved names.
"""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "rescases"

APP = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'
LIB = '[[assembly]]\nname = "Lib"\nkind = "library"\nroot = "Lib"\n'
MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"

DECLS = """internal record Point {
    public x: i32
    public y: i32
}

internal enum Shape {
    Empty
    Circle(i32)
    Rect { width: i32, height: i32 }
}

internal modal Door {
    @Open {
        public width: i32
    }
    @Closed {}
}

internal class Named {
    procedure name(~) -> i32
}

internal type Count = i32

internal let limit: i32 = 10

internal procedure helper(value: i32) -> i32 {
    return value
}

"""


def body(code: str, prefix: str = "") -> str:
    """A procedure `run` whose body is `code`, after the offered declarations."""
    lines = "".join(f"    {line}\n" if line else "\n" for line in code.split("\n"))
    return DECLS + prefix + "internal procedure run(input: i32) -> i32 {\n" + lines + "    return 0\n}\n" + MAIN


def top(code: str) -> str:
    return code + MAIN


CASES = {
    # --- unresolved names and suggestions ---
    "name_missing": body("let a = missing"),
    "name_missing_in_tail": body("let a = 1\nlet b = a + nothing_here"),
    "suggest_transposed": body("let count = 1\nlet b = coutn"),
    "suggest_one_off": body("let total = 1\nlet b = totel"),
    "suggest_short_name_none": body("let ab = 1\nlet b = xy"),
    "suggest_tie_same_scope": body("let x1 = 1\nlet x2 = 2\nlet x3 = 3\nlet b = x4"),
    "suggest_tie_many": body("let aa = 1\nlet ab = 2\nlet ac = 3\nlet ad = 4\nlet ae = 5\nlet af = 6\nlet ag = 7\nlet b = az"),
    "suggest_tie_long": body("let value_one = 1\nlet value_two = 2\nlet value_six = 3\nlet b = value_ten"),
    "suggest_inner_scope_wins": body("let name_a = 1\n{\n    let name_b = 2\n    let c = name_c\n}"),
    "suggest_param": body("let b = inptu"),
    "suggest_module_proc": body("let b = helpr(1)"),
    "suggest_module_tie": body("let b = limiz", "internal let limix: i32 = 1\ninternal let limiy: i32 = 2\n"),
    "suggest_type_name_for_value": body("let b = Poimt"),
    "suggest_universe": body("let b = boool"),
    "suggest_far_none": body("let alpha = 1\nlet b = omega_value"),
    "hint_option": body("let b = Option"),
    "hint_some": body("let b = Some(1)"),
    "hint_none": body("let b = None"),
    "hint_result": body("let b = Result"),
    "hint_ok": body("let b = Ok(1)"),
    "hint_err": body("let b = Err(1)"),
    "hint_vec": body("let b = Vec"),
    "hint_box": body("let b = Box"),
    "hint_rc": body("let b = Rc"),
    "hint_arc": body("let b = Arc"),
    "hint_string": body("let b = String"),
    "hint_println": body("println(1)"),
    "hint_print": body("print(1)"),
    "hint_eprintln": body("eprintln(1)"),
    "hint_eprint": body("eprint(1)"),
    "hint_with_suggestion": body("let Optio = 1\nlet b = Option"),
    "name_in_call_arg": body("let b = helper(missing)"),
    "name_in_method_receiver": body("let b = missing~>go()"),
    "name_in_method_arg": body("let p = Point { x: 1, y: 2 }\nlet b = p~>go(missing)"),
    "name_in_field_base": body("let b = missing.x"),
    "name_in_index": body("let xs = [1, 2]\nlet b = xs[missing]"),
    "name_in_tuple": body("let b = (1, missing)"),
    "name_in_array": body("let b = [1, missing]"),
    "name_in_array_repeat": body("let b = [0; missing]"),
    "name_in_unary": body("let b = -missing"),
    "name_in_cast": body("let b = missing as i64"),
    "name_in_if_cond": body("let b = if missing { 1 } else { 2 }"),
    "name_in_if_then": body("let b = if true { missing } else { 2 }"),
    "name_in_if_else": body("let b = if true { 1 } else { missing }"),
    "name_in_return": body("return missing"),
    "name_in_assign_place": body("missing = 1"),
    "name_in_assign_value": body("var a = 1\na = missing"),
    "name_in_compound_assign": body("var a = 1\na += missing"),
    "name_in_range": body("let b = 0..missing"),
    "name_in_block_expr": body("let b = { missing }"),
    "name_in_loop_cond": body("loop missing {\n}"),
    "name_in_loop_iter": body("loop item in missing {\n}"),
    "name_in_loop_body": body("loop {\n    let a = missing\n    break\n}"),
    "name_in_defer": body("defer {\n    let a = missing\n}"),
    "name_in_unsafe": body("unsafe {\n    let a = missing\n}"),
    "name_in_record_field": body("let b = Point { x: missing, y: 2 }"),
    "name_in_move": body("let b = move missing"),
    "name_in_static_init": DECLS + "internal let derived: i32 = missing\n" + MAIN,
    "binary_chain_first": body("let a = 1\nlet b = missing + a + a + a"),
    "binary_chain_last": body("let a = 1\nlet b = a + a + a + missing"),
    "binary_chain_mixed": body("let a = 1\nlet b = a + a * missing - a"),
    "binary_chain_nested": body("let a = 1\nlet b = (a + missing) + (a + other)"),
    "binary_chain_ok": body("let a = 1\nlet b = a + a + a + a - a * a / a"),
    "tilde_receiver_outside_method": body("let b = ~"),
    # --- scopes ---
    "dup_let_same_scope": body("let a = 1\nlet a = 2"),
    "dup_var_same_scope": body("var a = 1\nvar a = 2"),
    "shadow_outer_block": body("let a = 1\n{\n    let a = 2\n}"),
    "shadow_param": body("let input = 2"),
    "shadow_module_name": body("let helper = 2"),
    "shadow_module_type": body("let Point = 2"),
    "shadow_universe_type": body("let bool = 2"),
    "sibling_blocks_ok": body("{\n    let a = 1\n}\n{\n    let a = 2\n}"),
    "block_name_out_of_scope": body("{\n    let a = 1\n}\nlet b = a"),
    "let_uses_itself": body("let a = a"),
    "pattern_dup_tuple": body("let (a, a) = (1, 2)"),
    "pattern_dup_nested": body("let (a, (b, a)) = (1, (2, 3))"),
    "pattern_tuple_ok": body("let (a, b) = (1, 2)\nlet c = a + b"),
    "pattern_typed": body("let a: i32 = 1\nlet b: Missing = 2"),
    "pattern_wildcard": body("let _ = 1\nlet _ = 2"),
    "reserved_local_gen": body("let gen_value = 1"),
    "reserved_local_root": body("let ultraviolet = 1"),
    "using_local_ok": body("let source_value = 1\nusing source_value as alias_value\nlet b = alias_value"),
    "using_local_missing": body("using nothing as alias_value"),
    "using_local_dup_alias": body("let source_value = 1\nlet alias_value = 2\nusing source_value as alias_value"),
    "using_local_shadow_outer": body("let source_value = 1\n{\n    using source_value as input\n}"),
    "region_alias_ok": body("region as scratch {\n    let a = 1\n}"),
    "region_alias_out_of_scope": body("region as scratch {\n}\nlet b = scratch"),
    "region_alias_dup": body("let scratch = 1\nregion as scratch {\n}"),
    "region_alias_reused": body("region as scratch {\n}\nregion as scratch {\n}"),
    "region_body_missing": body("region as scratch {\n    let a = missing\n}"),
    "region_frame_ok": body("region as scratch {\n    scratch.frame {\n        let a = 1\n    }\n}"),
    "region_frame_missing": body("nothing.frame {\n    let a = 1\n}"),
    "frame_plain": body("region {\n    frame {\n        let a = missing\n    }\n}"),
    "loop_binding_ok": body("let xs = [1, 2]\nloop item in xs {\n    let a = item\n}"),
    "loop_binding_out_of_scope": body("let xs = [1, 2]\nloop item in xs {\n}\nlet b = item"),
    "loop_binding_shadow": body("let xs = [1, 2]\nloop input in xs {\n}"),
    "loop_binding_dup": body("let xs = [(1, 2)]\nloop (a, a) in xs {\n}"),
    "loop_typed_binding_missing_type": body("let xs = [1, 2]\nloop item: Missing in xs {\n}"),
    # --- if .. is ---
    "if_is_bare_type": body("let b = if input is Point {\n    1\n} else {\n    2\n}"),
    "if_is_bare_universe_type": body("let b = if input is i32 {\n    1\n} else {\n    2\n}"),
    "if_is_bare_alias": body("let b = if input is Count {\n    1\n} else {\n    2\n}"),
    "if_is_typed": body("let b = if input is value: i32 {\n    value\n} else {\n    2\n}"),
    "if_is_typed_missing": body("let b = if input is value: Missing {\n    1\n} else {\n    2\n}"),
    "if_is_binding_out_of_scope": body("let b = if input is value: i32 {\n    1\n} else {\n    value\n}"),
    "if_is_binding_shadow": body("let b = if input is input: i32 {\n    1\n} else {\n    2\n}"),
    "if_case_ok": body("let b = if input is {\n    1 {\n        0\n    }\n    value: i32 {\n        value\n    }\n}"),
    "if_case_arm_missing": body("let b = if input is {\n    1 {\n        missing\n    }\n    else {\n        0\n    }\n}"),
    "if_case_else_missing": body("let b = if input is {\n    1 {\n        0\n    }\n    else {\n        missing\n    }\n}"),
    "if_case_bare_type": body("let b = if input is {\n    Point {\n        0\n    }\n    else {\n        1\n    }\n}"),
    "if_case_dup": body("let pair = (1, 2)\nlet b = if pair is {\n    (a, a) {\n        0\n    }\n}"),
    "if_case_arm_scopes": body("let pair = (1, 2)\nlet b = if pair is {\n    (a, 1) {\n        a\n    }\n    (a, 2) {\n        a\n    }\n    else {\n        0\n    }\n}"),
    "if_case_typed_missing": body("let b = if input is {\n    value: Missing {\n        0\n    }\n}"),
    # --- patterns over nominal types ---
    "pat_record_ok": body("let p = Point { x: 1, y: 2 }\nlet b = if p is {\n    Point { x: px, y: py } {\n        px + py\n    }\n}"),
    "pat_record_shorthand": body("let p = Point { x: 1, y: 2 }\nlet b = if p is {\n    Point { x, y } {\n        x + y\n    }\n}"),
    "pat_record_missing_type": body("let b = if input is {\n    Missing { x } {\n        x\n    }\n}"),
    "pat_record_qualified": body("let p = Point { x: 1, y: 2 }\nlet b = if p is {\n    App::Point { x, y } {\n        x + y\n    }\n}"),
    "pat_record_qualified_missing": body("let b = if input is {\n    App::Missing { x } {\n        x\n    }\n}"),
    "pat_record_qualified_not_record": body("let b = if input is {\n    App::Shape { x } {\n        x\n    }\n}"),
    "pat_enum_ok": body("let s = Shape::Circle(1)\nlet b = if s is {\n    Shape::Empty {\n        0\n    }\n    Shape::Circle(r) {\n        r\n    }\n    Shape::Rect { width, height } {\n        width + height\n    }\n}"),
    "pat_enum_missing_type": body("let b = if input is {\n    Missing::Empty {\n        0\n    }\n}"),
    "pat_enum_missing_type_tuple": body("let b = if input is {\n    Missing::Circle(r) {\n        r\n    }\n}"),
    "pat_enum_missing_variant": body("let s = Shape::Empty\nlet b = if s is {\n    Shape::Nope {\n        0\n    }\n}"),
    "pat_enum_qualified": body("let s = Shape::Empty\nlet b = if s is {\n    App::Shape::Empty {\n        0\n    }\n}"),
    "pat_enum_record_named_like_variant": body(
        "let s = Both::Inner { value: 1 }\nlet b = if s is {\n    Both::Inner { value } {\n        value\n    }\n}",
        "internal enum Both {\n    Inner {\n        value: i32\n    }\n}\n",
    ),
    "pat_enum_dup": body("let s = Shape::Empty\nlet b = if s is {\n    Shape::Rect { width: a, height: a } {\n        a\n    }\n}"),
    "pat_modal_ok": body("let d: Door@Open = Door@Open { width: 1 }\nlet w: Door = widen d\nlet b = if w is {\n    @Open { width } {\n        width\n    }\n    @Closed {\n        0\n    }\n}"),
    "pat_range": body("let b = if input is {\n    1..5 {\n        1\n    }\n    else {\n        0\n    }\n}"),
    # --- qualified forms in expressions ---
    "enum_unit": body("let s = Shape::Empty"),
    "enum_tuple": body("let s = Shape::Circle(input)"),
    "enum_record": body("let s = Shape::Rect { width: input, height: 2 }"),
    "enum_qualified": body("let s = App::Shape::Circle(1)\nlet t = App::Shape::Empty\nlet u = App::Shape::Rect { width: 1, height: 2 }"),
    "enum_missing_variant": body("let s = Shape::Nope"),
    "enum_missing_variant_tuple": body("let s = Shape::Nope(1)"),
    "enum_missing_variant_record": body("let s = Shape::Nope { width: 1 }"),
    "enum_unit_as_tuple": body("let s = Shape::Empty(1)"),
    "enum_tuple_as_unit": body("let s = Shape::Circle"),
    "enum_tuple_as_record": body("let s = Shape::Circle { r: 1 }"),
    "enum_record_as_tuple": body("let s = Shape::Rect(1, 2)"),
    "enum_arg_missing": body("let s = Shape::Circle(missing)"),
    "enum_field_missing": body("let s = Shape::Rect { width: missing, height: 2 }"),
    "enum_missing_type": body("let s = Nope::Empty"),
    "enum_missing_type_call": body("let s = Nope::Circle(1)"),
    "enum_missing_type_record": body("let s = Nope::Rect { width: 1 }"),
    "enum_builtin_priority": body("let p = Priority::High\nlet q = Priority::Nope"),
    "enum_builtin_io_error": body("let e = IoError::NotFound\nlet k = FileKind::Dir"),
    "enum_builtin_alloc_error": body("let e = AllocationError::OutOfMemory(1usize)"),
    "enum_builtin_outcome": body("let o = Outcome::Value(1)\nlet p = Outcome::Error(2)\nlet q = Outcome::Nope(3)"),
    "record_literal": body("let p = Point { x: 1, y: 2 }"),
    "record_literal_missing": body("let p = Missing { x: 1 }"),
    "record_literal_qualified": body("let p = App::Point { x: 1, y: 2 }"),
    "record_literal_qualified_missing": body("let p = App::Missing { x: 1 }"),
    "record_literal_field_missing": body("let p = App::Point { x: missing, y: 2 }"),
    "record_literal_builtin": body("let o = RegionOptions { stack_size: 0usize, name: \"\" }"),
    "record_call_form": body("let p = Point()"),
    "record_call_form_qualified": body("let p = App::Point()"),
    "record_call_form_builtin": body("let o = RegionOptions()\nlet d = Duration()"),
    "record_as_value": body("let p = App::Point"),
    "modal_state_literal": body("let d = Door@Open { width: 1 }"),
    "modal_state_literal_missing": body("let d = Missing@Open { width: 1 }"),
    "modal_state_literal_field_missing": body("let d = Door@Open { width: missing }"),
    "qualified_proc": body("let a = App::helper(1)"),
    "qualified_proc_missing": body("let a = App::nothing(1)"),
    "qualified_proc_arg_missing": body("let a = App::helper(missing)"),
    "qualified_static": body("let a = App::limit"),
    "qualified_static_missing": body("let a = App::nothing"),
    "qualified_module_missing": body("let a = Nope::helper(1)"),
    "qualified_module_missing_name": body("let a = Nope::value"),
    "qualified_deep_missing": body("let a = App::Inner::Deeper::value"),
    "qualified_type_as_value": body("let a = App::Count"),
    "string_builtin_ok": body("let a = string::from(\"a\")\nlet b = string::length"),
    "string_builtin_missing": body("let a = string::nope(\"a\")"),
    "string_builtin_missing_name": body("let a = string::nope"),
    "bytes_builtin_ok": body("let a = bytes::with_capacity(1usize)\nlet b = bytes::view"),
    "bytes_builtin_wrong_namespace": body("let a = bytes::clone_with(\"a\")"),
    "string_builtin_wrong_namespace": body("let a = string::with_capacity(1usize)"),
    "string_builtin_arg_missing": body("let a = string::from(missing)"),
    "generic_call": body("let a = pick<i32>(1)\nlet b = pick<Missing>(1)", "internal procedure pick<TValue>(value: TValue) -> TValue {\n    return value\n}\n"),
    "generic_call_missing_callee": body("let a = nothing<i32>(1)"),
    # --- types ---
    "type_missing_param": DECLS + "internal procedure run(value: Missing) -> i32 {\n    return 0\n}\n" + MAIN,
    "type_missing_return": DECLS + "internal procedure run() -> Missing {\n    return 0\n}\n" + MAIN,
    "type_missing_field": top("internal record R {\n    public value: Missing\n}\n"),
    "type_missing_variant_tuple": top("internal enum E {\n    A(Missing)\n}\n"),
    "type_missing_variant_record": top("internal enum E {\n    A {\n        value: Missing\n    }\n}\n"),
    "type_missing_state_field": top("internal modal M {\n    @S {\n        public value: Missing\n    }\n}\n"),
    "type_missing_alias": top("internal type T = Missing\n"),
    "type_missing_static": top("internal let value: Missing = 1\n"),
    "type_missing_generic_arg": body("let a: Wrapper<Missing> = 1", "internal record Wrapper<TValue> {\n    public value: TValue\n}\n"),
    "type_missing_in_tuple": body("let a: (i32, Missing) = 1"),
    "type_missing_in_array": body("let a: [Missing; 2] = 1"),
    "type_missing_array_len": body("let a: [i32; missing] = 1"),
    "type_missing_in_slice": body("let a: [Missing] = 1"),
    "type_missing_in_union": body("let a: i32 | Missing = 1"),
    "type_missing_in_perm": body("let a: unique Missing = 1"),
    "type_missing_in_ptr": body("let a: Ptr<Missing> = 1"),
    "type_missing_in_func": body("let a: (Missing) -> i32 = 1"),
    "type_missing_func_ret": body("let a: (i32) -> Missing = 1"),
    "type_missing_modal_state": body("let a: Missing@Open = 1"),
    "type_modal_state_ok": body("let a: Door@Open = Door@Open { width: 1 }"),
    "type_qualified_ok": body("let a: App::Point = Point { x: 1, y: 2 }"),
    "type_qualified_missing": body("let a: App::Missing = 1"),
    "type_qualified_module_missing": body("let a: Nope::Point = 1"),
    "type_value_as_type": body("let a: helper = 1"),
    "type_class_as_type": body("let a: Named = 1"),
    "type_builtin_names": body("let a: Context = 1\nlet b: File = 2\nlet c: Outcome<i32, IoError> = 3\nlet d: Duration = 4\nlet e: AllocationError = 5"),
    "type_cast_missing": body("let a = input as Missing"),
    "type_sizeof_missing": body("let a = sizeof(Missing)"),
    "type_alignof_missing": body("let a = alignof(Missing)"),
    "type_transmute_missing": body("let a = transmute<i32, Missing>(input)"),
    "type_refine_self": body("let a: i32 |: { self > 0 } = 1"),
    "type_refine_missing": body("let a: i32 |: { missing > 0 } = 1"),
    "type_refine_shadow_self": DECLS + "internal record R {\n    public value: i32\n    internal procedure check(~) -> i32 {\n        let a: i32 |: { self > 0 } = 1\n        return a\n    }\n}\n" + MAIN,
    "type_dynamic_missing": body("let a: $Missing = 1"),
    "type_dynamic_ok": body("let a: $Named = 1\nlet b: $App::Named = 2\nlet c: $IO = 3"),
    "type_dynamic_qualified_missing": body("let a: $App::Missing = 1"),
    "type_self_outside": body("let a: Self = 1"),
    # --- generics ---
    "generic_proc_ok": top("internal procedure pick<TValue>(value: TValue) -> TValue {\n    let held: TValue = value\n    return held\n}\n"),
    "generic_proc_bound_missing": top("internal procedure pick<TValue <: Missing>(value: TValue) -> TValue {\n    return value\n}\n"),
    "generic_proc_bound_qualified_missing": top("internal procedure pick<TValue <: App::Missing>(value: TValue) -> TValue {\n    return value\n}\n"),
    "generic_proc_default_missing": top("internal procedure pick<TValue = Missing>(value: TValue) -> TValue {\n    return value\n}\n"),
    "generic_proc_param_shadows_param": top("internal procedure pick<value>(value: value) -> i32 {\n    return 0\n}\n"),
    "generic_record_ok": top("internal record Wrapper<TValue> {\n    public value: TValue\n    internal procedure get(~) -> TValue {\n        return self.value\n    }\n}\n"),
    "generic_record_method_own_param": top("internal record Wrapper<TValue> {\n    public value: TValue\n    internal procedure map<TOther>(~, other: TOther) -> TOther {\n        return other\n    }\n}\n"),
    "generic_enum_ok": top("internal enum Maybe<TValue> {\n    Nothing\n    Just(TValue)\n}\n"),
    "generic_modal_ok": top("internal modal Cell<TValue> {\n    @Full {\n        public value: TValue\n    }\n    @Empty {}\n}\n"),
    "generic_alias_ok": top("internal type Pair<TValue> = (TValue, TValue)\n"),
    "generic_class_method": top("internal class Maker<TValue> {\n    procedure make<TOther>(~, other: TOther) -> TValue\n}\n"),
    "generic_dup_param": top("internal procedure pick<TValue; TValue>(value: TValue) -> i32 {\n    return 0\n}\n"),
    "generic_param_not_in_sibling": top("internal procedure a<TValue>(value: TValue) -> i32 {\n    return 0\n}\ninternal procedure b(value: TValue) -> i32 {\n    return 0\n}\n"),
    # --- members ---
    "method_self_ok": top("internal record R {\n    public value: i32\n    internal procedure get(~) -> i32 {\n        return self.value\n    }\n    internal procedure make(~) -> Self {\n        return Self { value: 1 }\n    }\n}\n"),
    "method_body_missing": top("internal record R {\n    public value: i32\n    internal procedure get(~) -> i32 {\n        return missing\n    }\n}\n"),
    "method_param_dup_self": top("internal record R {\n    public value: i32\n    internal procedure get(~, self: i32) -> i32 {\n        return 0\n    }\n}\n"),
    "method_local_shadows_self": top("internal record R {\n    public value: i32\n    internal procedure get(~) -> i32 {\n        let self = 1\n        return 0\n    }\n}\n"),
    "method_explicit_receiver_missing": top("internal record R {\n    public value: i32\n    internal procedure get(self: Missing) -> i32 {\n        return 0\n    }\n}\n"),
    "method_tilde_value": top("internal record R {\n    public value: i32\n    internal procedure get(~) -> i32 {\n        return ~.value\n    }\n}\n"),
    "field_init_missing": top("internal record R {\n    public value: i32 = missing\n}\n"),
    "field_init_ok": top("internal let base: i32 = 1\ninternal record R {\n    public value: i32 = base\n}\n"),
    "record_implements_missing": top("internal record R <: Missing {\n    public value: i32\n}\n"),
    "record_implements_qualified_missing": top("internal record R <: App::Missing {\n    public value: i32\n}\n"),
    "record_implements_ok": top("internal class C {}\ninternal record R <: C {\n    public value: i32\n}\ninternal record S <: App::C {\n    public value: i32\n}\ninternal record T <: Bitcopy {\n    public value: i32\n}\n"),
    "class_super_qualified_missing": top("internal class C <: App::Missing {}\n"),
    "class_method_body": top("internal class C {\n    procedure get(~) -> i32 {\n        return missing\n    }\n}\n"),
    "class_method_self_type": top("internal class C {\n    procedure get(~, other: Self) -> Self\n}\n"),
    "class_field_missing": top("internal class C {\n    value: Missing\n}\n"),
    "class_assoc_default_missing": top("internal class C {\n    type Item = Missing\n}\n"),
    "state_method_ok": top("internal modal M {\n    @S {\n        public value: i32\n        internal procedure get(~) -> i32 {\n            return self.value\n        }\n        internal transition next(step: i32) -> @T {\n            return M@T {}\n        }\n    }\n    @T {}\n}\n"),
    "state_method_missing": top("internal modal M {\n    @S {\n        internal procedure get(~) -> i32 {\n            return missing\n        }\n    }\n}\n"),
    "state_method_self_type": top("internal modal M {\n    @S {\n        internal procedure get(~) -> Self {\n            return self\n        }\n    }\n}\n"),
    "transition_missing": top("internal modal M {\n    @S {\n        internal transition next() -> @T {\n            return missing\n        }\n    }\n    @T {}\n}\n"),
    "transition_param_missing_type": top("internal modal M {\n    @S {\n        internal transition next(step: Missing) -> @T {\n            return M@T {}\n        }\n    }\n    @T {}\n}\n"),
    # --- contracts ---
    "contract_ok": top("internal procedure f(value: i32) -> i32\n    |: value > 0 |= @result > 0\n{\n    return value\n}\n"),
    "contract_pre_missing": top("internal procedure f(value: i32) -> i32\n    |: missing > 0\n{\n    return value\n}\n"),
    "contract_post_missing": top("internal procedure f(value: i32) -> i32\n    |: value > 0 |= @result > missing\n{\n    return value\n}\n"),
    "contract_entry": top("internal procedure f(value: i32) -> i32\n    |: |= @entry(value) == @entry(missing)\n{\n    return value\n}\n"),
    "contract_result_in_pre": top("internal procedure f(value: i32) -> i32\n    |: @result > 0\n{\n    return value\n}\n"),
    "invariant_ok": top("internal record R {\n    public value: i32\n} |: { self.value > 0 }\n"),
    "invariant_missing": top("internal record R {\n    public value: i32\n} |: { missing > 0 }\n"),
    "loop_invariant_ok": body("var a = 0\nloop a < 3 |: { a >= 0 } {\n    a += 1\n}"),
    "loop_iter_invariant_sees_binding": body("let xs = [1, 2]\nloop item in xs |: { item > 0 } {\n}"),
    "transmute_ok": body("let a = transmute<i32, u32>(input)\nlet b = transmute<Missing, u32>(input)"),
    "enum_pattern_tuple_two": body("let b = if input is {\n    Missing::Pair(a, b) {\n        a\n    }\n}"),
    "loop_invariant_missing": body("var a = 0\nloop a < 3 |: { missing > 0 } {\n    a += 1\n}"),
    "extern_ok": top("extern \"C\" {\n    public procedure imported(value: i32) -> i32\n}\n"),
    "extern_missing_type": top("extern \"C\" {\n    public procedure imported(value: Missing) -> i32\n}\n"),
    "extern_missing_return": top("extern \"C\" {\n    public procedure imported(value: i32) -> Missing\n}\n"),
    "extern_foreign_contract_ok": top("extern \"C\" {\n    public procedure imported(value: i32) -> i32\n        |: @foreign_assumes(value > 0)\n}\n"),
    "extern_foreign_contract_missing": top("extern \"C\" {\n    public procedure imported(value: i32) -> i32\n        |: @foreign_assumes(missing > 0)\n}\n"),
    "extern_foreign_ensures_missing": top("extern \"C\" {\n    public procedure imported(value: i32) -> i32\n        |: @foreign_ensures(missing > 0)\n}\n"),
    "extern_unknown_abi": top("extern \"weird\" {\n    public procedure imported(value: i32) -> i32\n}\n"),
    # --- module-level names ---
    "dup_record": top("internal record R {\n    public value: i32\n}\ninternal record R {\n    public other: i32\n}\n"),
    "dup_record_enum": top("internal record R {\n    public value: i32\n}\ninternal enum R {\n    A\n}\n"),
    "dup_proc_record": top("internal procedure R() -> i32 {\n    return 0\n}\ninternal record R {\n    public value: i32\n}\n"),
    "dup_record_proc": top("internal record R {\n    public value: i32\n}\ninternal procedure R() -> i32 {\n    return 0\n}\n"),
    "proc_overloads": top("internal procedure f(value: i32) -> i32 {\n    return 0\n}\ninternal procedure f(value: bool) -> i32 {\n    return 0\n}\ninternal procedure f() -> i32 {\n    return 0\n}\n"),
    "proc_overload_then_record": top("internal procedure f(value: i32) -> i32 {\n    return 0\n}\ninternal procedure f(value: bool) -> i32 {\n    return 0\n}\ninternal record f {\n    public value: i32\n}\n"),
    "dup_static": top("internal let value: i32 = 1\ninternal let value: i32 = 2\n"),
    "dup_static_proc": top("internal let value: i32 = 1\ninternal procedure value() -> i32 {\n    return 0\n}\n"),
    "dup_proc_static": top("internal procedure value() -> i32 {\n    return 0\n}\ninternal let value: i32 = 1\n"),
    "dup_static_tuple": top("internal let (a, a): (i32, i32) = (1, 2)\n"),
    "dup_main": "public procedure main() -> i32 {\n    return 0\n}\npublic procedure main() -> i32 {\n    return 1\n}\n",
    "dup_main_static": "internal let main: i32 = 1\npublic procedure main() -> i32 {\n    return 0\n}\n",
    "dup_extern": top("extern \"C\" {\n    public procedure imported() -> i32\n    public procedure imported() -> i32\n}\n"),
    "dup_extern_proc": top("extern \"C\" {\n    public procedure imported() -> i32\n}\ninternal procedure imported() -> i32 {\n    return 0\n}\n"),
    "dup_class_record": top("internal class C {}\ninternal record C {\n    public value: i32\n}\n"),
    "dup_alias_record": top("internal type T = i32\ninternal record T {\n    public value: i32\n}\n"),
    "reserved_module_gen": top("internal procedure gen_helper() -> i32 {\n    return 0\n}\n"),
    "reserved_module_root": top("internal procedure ultraviolet() -> i32 {\n    return 0\n}\n"),
    "reserved_module_async": top("internal record Future {\n    public value: i32\n}\n"),
    "reserved_module_special": top("internal record Region {\n    public value: i32\n}\n"),
    "reserved_module_capability": top("internal class IO {}\n"),
    "reserved_module_foundational": top("internal class Clone {}\n"),
    "reserved_module_context": top("internal record Context {\n    public value: i32\n}\n"),
    "reserved_module_static": top("internal let Outcome: i32 = 1\n"),
    "reserved_several_first_wins": top("internal record Stream {\n    public value: i32\n}\ninternal record Region {\n    public value: i32\n}\ninternal procedure gen_x() -> i32 {\n    return 0\n}\n"),
    "forward_reference_ok": top("internal procedure first() -> i32 {\n    return second()\n}\ninternal procedure second() -> i32 {\n    let p: Late = Late { value: 1 }\n    return p.value\n}\ninternal record Late {\n    public value: i32\n}\n"),
    "two_errors_one_module": top("internal procedure first() -> i32 {\n    return missing_one\n}\ninternal procedure second() -> i32 {\n    return missing_two\n}\n"),
    "error_in_record_then_proc": top("internal record R {\n    public value: Missing\n}\ninternal procedure second() -> i32 {\n    return missing_two\n}\n"),
    "error_item_detail_enum": top("internal enum E {\n    A(Missing)\n}\n"),
    "error_item_detail_modal": top("internal modal M {\n    @S {\n        public value: Missing\n    }\n}\n"),
    "error_item_detail_class": top("internal class C {\n    procedure get(~) -> Missing\n}\n"),
    "error_item_detail_alias": top("internal type T = (i32, Missing)\n"),
    "error_item_detail_static": top("internal let value: i32 = missing\n"),
    # --- compile-time code ---
    "comptime_capabilities": body("let a = comptime { introspect~>type_name(Type::<Point>) }"),
    "comptime_proc_ok": top("comptime internal procedure twice(value: usize) -> usize {\n    return value + value\n}\ninternal procedure run() -> usize {\n    return comptime { twice(2usize) }\n}\n"),
    "comptime_proc_body_missing": top("comptime internal procedure twice(value: usize) -> usize {\n    return missing\n}\n"),
    "comptime_proc_missing_type": top("comptime internal procedure twice(value: Missing) -> usize {\n    return 1usize\n}\n"),
    "comptime_proc_dup": top("comptime internal procedure twice(value: usize) -> usize {\n    return value\n}\ncomptime internal procedure twice(value: usize) -> usize {\n    return value\n}\n"),
    "comptime_proc_vs_record": top("comptime internal procedure twice(value: usize) -> usize {\n    return value\n}\ninternal record twice {\n    public value: i32\n}\n"),
    "derive_target_ok": top("derive target Marker(target: Type) {\n    let name = introspect~>type_name(target)\n}\n"),
    "derive_target_missing": top("derive target Marker(target: Type) {\n    let name = missing\n}\n"),
    "derive_target_dup": top("derive target Marker(target: Type) {\n}\nderive target Marker(target: Type) {\n}\n"),
}

UTIL = """public procedure offered(value: i32) -> i32 {
    return value
}

internal procedure inner(value: i32) -> i32 {
    return value
}

private procedure hidden(value: i32) -> i32 {
    return value
}

public record Remote {
    public value: i32
}

internal record Local {
    public value: i32
}

private record Secret {
    public value: i32
}

public enum Kind {
    One
    Two(i32)
    Three {
        value: i32
    }
}

public class Trait {}

public let level: i32 = 3
"""

LIBRARY = """public procedure exported(value: i32) -> i32 {
    return value
}

internal procedure internal_only(value: i32) -> i32 {
    return value
}

public record Thing {
    public value: i32
}

public enum Flag {
    On
    Off
}
"""


def app(main_text: str, extra: dict | None = None) -> tuple:
    files = {"Source/Main.uv": main_text + MAIN, "Source/Util/u.uv": UTIL}
    files.update(extra or {})
    return (APP, files)


def with_lib(main_text: str, extra: dict | None = None) -> tuple:
    files = {"Source/Main.uv": main_text + MAIN, "Source/Util/u.uv": UTIL, "Lib/l.uv": LIBRARY}
    files.update(extra or {})
    return (APP + LIB, files)


def use(code: str) -> str:
    lines = "".join(f"    {line}\n" for line in code.split("\n"))
    return "internal procedure run() -> i32 {\n" + lines + "    return 0\n}\n"


MULTI = {
    # --- using ---
    "using_item": app("using App::Util::offered\n" + use("let a = offered(1)")),
    "using_item_alias": app("using App::Util::offered as renamed\n" + use("let a = renamed(1)\nlet b = offered(1)")),
    "using_item_relative": app("using Util::offered\n" + use("let a = offered(1)")),
    "using_item_type": app("using App::Util::Remote\n" + use("let a: Remote = Remote { value: 1 }")),
    "using_item_enum": app("using App::Util::Kind\n" + use("let a = Kind::One\nlet b = Kind::Two(1)\nlet c = Kind::Three { value: 1 }\nlet d = Kind::Four")),
    "using_item_class": app("using App::Util::Trait\ninternal record R <: Trait {\n    public value: i32\n}\n"),
    "using_item_static": app("using App::Util::level\n" + use("let a = level")),
    "using_item_missing": app("using App::Util::nothing\n"),
    "using_item_module_missing": app("using App::Nope::offered\n"),
    "using_item_internal": app("using App::Util::inner\n" + use("let a = inner(1)")),
    "using_item_private": app("using App::Util::hidden\n"),
    "using_item_private_type": app("using App::Util::Secret\n"),
    "using_item_module_itself": app("using App::Util\n"),
    "using_list": app("using App::Util::{ offered, Remote, Kind }\n" + use("let a = offered(1)\nlet b: Remote = Remote { value: 1 }\nlet c = Kind::One")),
    "using_list_alias": app("using App::Util::{ offered as renamed, Remote as Far }\n" + use("let a = renamed(1)\nlet b: Far = Far { value: 1 }")),
    "using_list_dup": app("using App::Util::{ offered, offered }\n"),
    "using_list_dup_alias": app("using App::Util::{ offered as one, inner as one }\n"),
    "using_list_same_item_two_names": app("using App::Util::{ offered as one, offered as two }\n" + use("let a = one(1) + two(2)")),
    "using_list_missing": app("using App::Util::{ offered, nothing }\n"),
    "using_list_private": app("using App::Util::{ offered, hidden }\n"),
    "using_list_module_missing": app("using App::Nope::{ offered }\n"),
    "using_wildcard": app("using App::Util::*\n" + use("let a = offered(1) + inner(2)\nlet b: Remote = Remote { value: level }\nlet c = hidden(1)")),
    "using_wildcard_module_missing": app("using App::Nope::*\n"),
    "using_wildcard_private_api": app("using App::Util::*\ninternal procedure run() -> i32 {\n    return offered(1)\n}\n"),
    "using_wildcard_public_api": app("using App::Util::*\npublic procedure run() -> i32 {\n    return offered(1)\n}\n"),
    "using_wildcard_conflict_decl": app("using App::Util::*\ninternal procedure offered(value: i32) -> i32 {\n    return value\n}\n"),
    "using_decl_then_wildcard": app("internal procedure offered(value: i32) -> i32 {\n    return value\n}\nusing App::Util::*\n"),
    "using_conflict_decl": app("using App::Util::offered\ninternal procedure offered(value: i32) -> i32 {\n    return value\n}\n"),
    "using_conflict_decl_before": app("internal record Remote {\n    public value: i32\n}\nusing App::Util::Remote\n"),
    "using_twice_same": app("using App::Util::offered\nusing App::Util::offered\n"),
    "using_two_modules_same_name": app(
        "using App::Util::offered\nusing App::Other::offered\n",
        {"Source/Other/o.uv": "public procedure offered(value: i32) -> i32 {\n    return value\n}\n"},
    ),
    "using_wildcards_same_name": app(
        "using App::Util::*\nusing App::Other::*\n" + use("let a = offered(1)"),
        {"Source/Other/o.uv": "public procedure offered(value: i32) -> i32 {\n    return value\n}\n"},
    ),
    "using_public_of_public": app("public using App::Util::offered\n"),
    "using_public_of_internal": app("public using App::Util::inner\n"),
    "using_public_list_of_internal": app("public using App::Util::{ offered, inner }\n"),
    "using_public_wildcard": app("public using App::Util::*\n"),
    "using_public_wildcard_all_public": app(
        "public using App::Open::*\n",
        {"Source/Open/o.uv": "public procedure opened(value: i32) -> i32 {\n    return value\n}\npublic record Wide {\n    public value: i32\n}\n"},
    ),
    "using_reexport_chain": app(
        "using App::Mid::offered\n" + use("let a = offered(1)"),
        {"Source/Mid/m.uv": "public using App::Util::offered\n"},
    ),
    "using_reexport_wildcard_chain": app(
        "using App::Mid::*\n" + use("let a = offered(1)\nlet b: Remote = Remote { value: 1 }"),
        {"Source/Mid/m.uv": "public using App::Util::{ offered, Remote }\n"},
    ),
    "using_reexport_internal_chain": app(
        "using App::Mid::inner\n" + use("let a = inner(1)"),
        {"Source/Mid/m.uv": "internal using App::Util::inner\n"},
    ),
    "using_reexport_private_chain": app(
        "using App::Mid::offered\n",
        {"Source/Mid/m.uv": "private using App::Util::offered\n"},
    ),
    "using_cycle": app(
        "using App::A::*\n" + use("let a = from_a(1) + from_b(2)"),
        {
            "Source/A/a.uv": "public using App::B::*\npublic procedure from_a(value: i32) -> i32 {\n    return value\n}\n",
            "Source/B/b.uv": "public using App::A::*\npublic procedure from_b(value: i32) -> i32 {\n    return value\n}\n",
        },
    ),
    "using_enum_variant_path": app("using App::Util::Kind::One\n"),
    "using_reserved_alias": app("using App::Util::offered as gen_shared\n"),
    "using_alias_universe": app("using App::Util::Remote as Region\n"),
    "using_alias_keyword_like": app("using App::Util::offered as helper\ninternal procedure helper() -> i32 {\n    return 0\n}\n"),
    # A type and a module with the same name: `Util::X { .. }` in a pattern can then mean
    # a variant of the type or a record of the module.
    "pattern_type_vs_module_enum": app("internal record Util {\n    public value: i32\n}\n" + use("let b = if 1 is {\n    Util::Kind { value } {\n        value\n    }\n}")),
    "pattern_type_vs_module_record": app("internal record Util {\n    public value: i32\n}\n" + use("let b = if 1 is {\n    Util::Remote { value } {\n        value\n    }\n}")),
    "pattern_type_vs_module_missing": app("internal record Util {\n    public value: i32\n}\n" + use("let b = if 1 is {\n    Util::Nope { value } {\n        value\n    }\n}")),
    "pattern_variant_vs_module_record": app("internal enum Util {\n    Remote {\n        value: i32\n    }\n    Other\n}\n" + use("let b = if 1 is {\n    Util::Remote { value } {\n        value\n    }\n}")),
    "pattern_variant_vs_module_enum": app("internal enum Util {\n    Kind {\n        value: i32\n    }\n}\n" + use("let b = if 1 is {\n    Util::Kind { value } {\n        value\n    }\n}")),
    "pattern_tuple_variant_vs_module_record": app("internal enum Util {\n    Remote(i32)\n}\n" + use("let b = if 1 is {\n    Util::Remote { value } {\n        value\n    }\n}")),
    "expr_type_vs_module": app("internal enum Util {\n    Remote {\n        value: i32\n    }\n    offered(i32)\n    level\n}\n" + use("let a = Util::Remote { value: 1 }\nlet b = Util::offered(1)\nlet c = Util::level\nlet d = Util::Kind::One")),
    "emit_in_comptime_procedure": app("comptime internal procedure f() -> usize {\n    emitter~>emit(1)\n    return 1usize\n}\n"),
    "emit_in_nested_comptime": app("comptime internal procedure f() -> usize {\n    let a = comptime { emitter~>emit(1) }\n    return 1usize\n}\n"),
    "emit_in_ct_statement": app("comptime internal procedure f() -> usize {\n    comptime {\n        emitter~>emit(1)\n    }\n    return 1usize\n}\n"),
    "emit_with_files_only": app("comptime internal procedure f() -> usize {\n    #files comptime {\n        emitter~>emit(1)\n        let a = files\n    }\n    return 1usize\n}\n"),
    "emit_granted": app("comptime internal procedure f() -> usize {\n    #emit comptime {\n        emitter~>emit(1)\n        let a = files\n    }\n    return 1usize\n}\n"),
    "emit_other_method": app("comptime internal procedure f() -> usize {\n    comptime {\n        emitter~>other(1)\n    }\n    return 1usize\n}\n"),
    # --- import ---
    "import_module": app("import App::Util\n" + use("let a = Util::offered(1)\nlet b: Util::Remote = Util::Remote { value: 1 }\nlet c = Util::Kind::Two(1)")),
    "import_alias": app("import App::Util as tools\n" + use("let a = tools::offered(1)\nlet b = Util::offered(1)")),
    "import_relative": app("import Util\n" + use("let a = Util::offered(1)")),
    "import_missing": app("import App::Nope\n"),
    "import_missing_alias": app("import App::Nope as nope\n"),
    "import_twice": app("import App::Util\nimport App::Util\n"),
    "import_two_aliases": app("import App::Util as one\nimport App::Util as two\n" + use("let a = one::offered(1) + two::offered(2)")),
    "import_alias_conflict_decl": app("import App::Util as tools\ninternal procedure tools() -> i32 {\n    return 0\n}\n"),
    "import_conflict_using": app("import App::Util\nusing App::Other::Util\n", {"Source/Other/o.uv": "public record Util {\n    public value: i32\n}\n"}),
    "import_self": app("import App\n"),
    "import_alias_reserved": app("import App::Util as gen_tools\n"),
    "import_alias_as_value": app("import App::Util as tools\n" + use("let a = tools")),
    "import_alias_as_type": app("import App::Util as tools\n" + use("let a: tools = 1")),
    "module_path_without_import": app(use("let a = App::Util::offered(1)\nlet b = Util::offered(2)")),
    "module_item_private": app(use("let a = App::Util::hidden(1)")),
    "module_item_private_type": app(use("let a: App::Util::Secret = 1")),
    "module_item_internal": app(use("let a = App::Util::inner(1)\nlet b: App::Util::Local = App::Util::Local { value: 1 }")),
    "module_enum_paths": app(use("let a = App::Util::Kind::One\nlet b = Util::Kind::Two(1)\nlet c = App::Util::Kind::Three { value: 1 }\nlet d = App::Util::Kind::Four")),
    "module_record_pattern": app(use("let r = App::Util::Remote { value: 1 }\nlet b = if r is {\n    App::Util::Remote { value } {\n        value\n    }\n}")),
    "module_enum_pattern": app(use("let k = App::Util::Kind::One\nlet b = if k is {\n    App::Util::Kind::One {\n        0\n    }\n    Util::Kind::Two(n) {\n        n\n    }\n    App::Util::Kind::Three { value } {\n        value\n    }\n}")),
    "module_pattern_not_record": app(use("let b = if 1 is {\n    App::Util::offered { value } {\n        value\n    }\n}")),
    "module_pattern_enum_not_variant": app(use("let k = App::Util::Kind::One\nlet b = if k is {\n    App::Util::Kind::Nope { value } {\n        value\n    }\n}")),
    "module_class_paths": app("internal record R <: App::Util::Trait {\n    public value: i32\n}\ninternal record S <: Util::Trait {\n    public value: i32\n}\ninternal record T <: App::Util::Nope {\n    public value: i32\n}\n"),
    "module_shadowed_by_local": app("import App::Util\n" + use("let Util = 1\nlet a = Util::offered(1)")),
    "nested_modules": app(
        use("let a = App::Util::Deep::deepest(1)\nlet b = Util::Deep::deepest(2)"),
        {"Source/Util/Deep/d.uv": "public procedure deepest(value: i32) -> i32 {\n    return value\n}\n"},
    ),
    "sibling_module_names": app(
        use("let a = App::Other::twin(1)"),
        {"Source/Other/o.uv": "using App::Util::offered\npublic procedure twin(value: i32) -> i32 {\n    return offered(value) + missing_here\n}\n"},
    ),
    "errors_in_two_modules": app(
        use("let a = missing_in_main"),
        {"Source/Other/o.uv": "public procedure twin(value: i32) -> i32 {\n    return missing_in_other\n}\n"},
    ),
    # --- other assemblies ---
    "lib_without_import": with_lib(use("let a = Lib::exported(1)")),
    "lib_with_import": with_lib("import Lib\n" + use("let a = Lib::exported(1)\nlet b: Lib::Thing = Lib::Thing { value: 1 }\nlet c = Lib::Flag::On")),
    "lib_import_alias": with_lib("import Lib as library\n" + use("let a = library::exported(1)\nlet b = Lib::exported(2)")),
    "lib_internal_item": with_lib("import Lib\n" + use("let a = Lib::internal_only(1)")),
    "lib_using_without_import": with_lib("using Lib::exported\n"),
    "lib_using_with_import": with_lib("import Lib\nusing Lib::exported\n" + use("let a = exported(1)")),
    "lib_using_import_after": with_lib("using Lib::exported\nimport Lib\n" + use("let a = exported(1)")),
    "lib_using_list_without_import": with_lib("using Lib::{ exported, Thing }\n"),
    "lib_using_wildcard_without_import": with_lib("using Lib::*\n"),
    "lib_using_wildcard_with_import": with_lib("import Lib\nusing Lib::*\n" + use("let a = exported(1)\nlet b = internal_only(2)")),
    "lib_using_internal": with_lib("import Lib\nusing Lib::internal_only\n"),
    "lib_public_using": with_lib("import Lib\npublic using Lib::exported\n"),
    "lib_type_without_import": with_lib(use("let a: Lib::Thing = 1")),
    "lib_pattern_without_import": with_lib(use("let b = if 1 is {\n    Lib::Flag::On {\n        0\n    }\n}")),
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
        for rel, content in files.items():
            path = case_dir / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8", newline="")
        entries.append(f"target/rescases/{name}/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "resolve_targeted.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} name-resolution cases")


if __name__ == "__main__":
    main()
