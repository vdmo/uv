#!/usr/bin/env python3
"""Generate phase-1 edge-case projects under target/p1cases and their list file.

The cases target the checks that run on whole modules after parsing: reserved-keyword
binders, `self` outside a type, `#derive` attribute lists, the order in which assemblies
are parsed (import-driven), multi-file modules and the error cap.
"""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "p1cases"

MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"
APP = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'


def lib(name: str, root: str | None = None) -> str:
    return f'[[assembly]]\nname = "{name}"\nkind = "library"\nroot = "{root or name}"\n'


# name -> {relative path: contents}; the manifest defaults to a single executable.
SINGLE = {
    "ok_plain": MAIN,
    "self_free_procedure": "procedure f(self: i32) -> i32 {\n    return 0\n}\n" + MAIN,
    "self_second_param": "procedure f(a: i32, self: i32, self: i32) {\n}\n" + MAIN,
    "self_in_record_method": "record R {\n    x: i32\n    procedure get(~) -> i32 {\n        return self.x\n    }\n}\n" + MAIN,
    "kw_procedure_name": "procedure loop() {\n}\n" + MAIN,
    "kw_param_name": "procedure f(if: i32, record: i32) {\n}\n" + MAIN,
    "kw_generic_param": "procedure f<enum>(x: enum) {\n}\n" + MAIN,
    "kw_let_binding": "procedure f() {\n    let record = 1\n    var enum: i32 = 2\n    let (modal, class) = (1, 2)\n}\n" + MAIN,
    "kw_static_binding": "let record: i32 = 1\nvar loop = 2\n" + MAIN,
    "kw_record_name": "record if {\n    x: i32\n}\n" + MAIN,
    "kw_record_field": "record R {\n    loop: i32\n    return: i32 = 3\n    procedure break(~) {\n    }\n    type enum = i32\n}\n" + MAIN,
    "kw_enum": "enum let {\n    var\n    record(i32)\n    Named { loop: i32, y: i32 }\n}\n" + MAIN,
    "kw_modal": "modal M {\n    @loop {\n        if: i32\n        procedure record(~) {\n        }\n        transition enum() -> @Done {\n        }\n    }\n    @Done {\n    }\n}\n" + MAIN,
    "kw_class": "class C {\n    procedure loop(~) -> i32\n    type record\n    if: i32\n    @enum {\n        let: i32\n    }\n}\n" + MAIN,
    "kw_type_alias": "type loop = i32\ntype Pair<record> = (record, record)\n" + MAIN,
    "kw_using": "using App::helper as loop\nusing App::{helper as record, other}\n" + MAIN,
    "kw_import_alias": "import App as loop\n" + MAIN,
    "kw_extern": 'extern "C" {\n    procedure loop(if: i32) -> i32\n}\n' + MAIN,
    "kw_closure": "procedure f() {\n    let g = |loop, record: i32| loop\n    let h = |x| { let enum = x\n        enum }\n}\n" + MAIN,
    "kw_patterns": "procedure f(v: (i32, i32)) {\n    if v is (loop, record) {\n    }\n    loop enum in 0..3 {\n    }\n    if v is {\n        (modal, 1) { }\n        _ { }\n    }\n}\n" + MAIN,
    "kw_region_using": "procedure f() {\n    region as loop {\n    }\n    using x as record\n}\n" + MAIN,
    "kw_nested_exprs": "procedure f() -> i32 {\n    let a = [ { let loop = 1\n        loop }, 2 ]\n    let b = g(|record| record, (1, { let enum = 2\n        enum }))\n    return if true { let modal = 1\n        modal } else { 0 }\n}\n" + MAIN,
    "kw_duplicate_span": "procedure f() {\n    let loop = 1\n    let loop = 2\n}\n" + MAIN,
    "kw_derive_target": "derive target loop(target: Type) {\n    let record = 1\n}\n" + MAIN,
    "kw_comptime_procedure": "comptime procedure loop(if: i32) -> i32 {\n    let enum = 1\n    return enum\n}\n" + MAIN,
    "derive_ok": "#derive(Eq, Hash)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_empty": "#derive()\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_no_args": "#derive\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_duplicate": "#derive(Eq, Eq)\nenum E {\n    A\n}\n" + MAIN,
    "derive_string_arg": '#derive("Eq")\nrecord R {\n    x: i32\n}\n' + MAIN,
    "derive_keyed_arg": "#derive(target: Eq)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_with_unknown_attr": "#derive(Eq)\n#bogus\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_with_bad_target_attr": "#derive(Eq)\n#inline\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_with_test_attr": "#test\n#derive(Eq)\nmodal M {\n    @A {\n    }\n}\n" + MAIN,
    "derive_with_library_attr": '#derive(Eq)\n#library(name: "c")\nenum E {\n    A\n}\n' + MAIN,
    "derive_layout_ok": "#derive(Eq)\n#layout(C)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_packed_enum": "#derive(Eq)\n#layout(packed)\nenum E {\n    A\n}\n" + MAIN,
    "derive_layout_enum_disc": "#derive(Eq)\n#layout(u8)\nenum E {\n    A\n}\n" + MAIN,
    "derive_layout_record_disc": "#derive(Eq)\n#layout(u8)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_align": "#derive(Eq)\n#layout(C, align(8))\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_align_bad": "#derive(Eq)\n#layout(align(3))\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_align_twice": "#derive(Eq)\n#layout(align(4), align(8))\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_packed_align": "#derive(Eq)\n#layout(packed, align(4))\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_empty": "#derive(Eq)\n#layout()\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_unknown": "#derive(Eq)\n#layout(rust)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_dup_c": "#derive(Eq)\n#layout(C, C)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_layout_keyed": "#derive(Eq)\n#layout(kind: C)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_deprecated_ok": '#derive(Eq)\n#deprecated("old")\nrecord R {\n    x: i32\n}\n' + MAIN,
    "derive_deprecated_bad": "#derive(Eq)\n#deprecated(1, 2)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_reflect_args": "#derive(Eq)\n#reflect(x)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_dynamic_ok": "#derive(Eq)\n#dynamic\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_ffi_by_value_modal": "#derive(Eq)\n#ffi_pass_by_value\nmodal M {\n    @A {\n    }\n}\n" + MAIN,
    "derive_vendor_attr": "#derive(Eq)\n#ultraviolet::thing\nrecord R {\n    x: i32\n}\n" + MAIN,
    "derive_second_bad": "#derive(Eq)\nrecord A {\n    x: i32\n}\n#derive()\nrecord B {\n    x: i32\n}\n#derive(Eq, Eq)\nrecord C {\n    x: i32\n}\n" + MAIN,
    "no_derive_bad_attr": "#bogus\n#layout(nope)\nrecord R {\n    x: i32\n}\n" + MAIN,
    "syntax_error_then_kw": "let f = = 1\nprocedure loop() {\n}\n" + MAIN,
    "lex_error_only": "let a = 0x\n" + MAIN,
    "warning_only": "﻿" + MAIN,
    "error_item_only": "@@@\n" + MAIN,
    "many_errors": "".join(f"let v{i} = 0x\n" for i in range(130)) + MAIN,
    "many_kw_binders": "".join(f"procedure f{i}(loop: i32) {{\n}}\n" for i in range(130)) + MAIN,
    "exactly_cap_errors": "".join(f"procedure f{i}(loop: i32) {{\n}}\n" for i in range(100)) + MAIN,
    "below_cap_errors": "".join(f"procedure f{i}(loop: i32) {{\n}}\n" for i in range(99)) + MAIN,
    "empty_file": "",
    "docs_only": "//! Module doc\n/// Item doc\n" + MAIN + "/// trailing doc\n",
}

MULTI = {
    "multi_file_module": (APP, {
        "Source/Main.uv": MAIN,
        "Source/b.uv": "procedure loop() {\n}\n",
        "Source/A.uv": "procedure f(self: i32) {\n}\n",
        "Source/Sub/x.uv": "let record = 1\n",
        "Source/Sub/Deep/y.uv": "#derive()\nrecord R {\n    x: i32\n}\n",
    }),
    "multi_file_stop_at_first_bad_module": (APP, {
        "Source/Main.uv": MAIN,
        "Source/A/a.uv": "\xff\xfe".encode("latin-1"),
        "Source/B/b.uv": "procedure loop() {\n}\n",
    }),
    "multi_file_syntax_errors": (APP, {
        "Source/Main.uv": MAIN + "procedure 1() {\n}\n",
        "Source/A/a.uv": "record 1 {\n}\n",
        "Source/B/b.uv": "procedure loop() {\n}\n",
    }),
    "import_chain": (APP + lib("Zed") + lib("Mid") + lib("Alone"), {
        "Source/Main.uv": "import Mid\n" + MAIN,
        "Mid/m.uv": "import Zed\npublic procedure mid() {\n}\n",
        "Zed/z.uv": "public procedure zed(loop: i32) {\n}\n",
        "Alone/a.uv": "public procedure alone(record: i32) {\n}\n",
    }),
    "import_error_in_imported": (APP + lib("First") + lib("Second"), {
        "Source/Main.uv": "import Second\n" + MAIN,
        "First/f.uv": "let broken = = 1\n",
        "Second/s.uv": "let also_broken = = 1\n",
    }),
    "import_error_in_unreachable": (APP + lib("First") + lib("Second"), {
        "Source/Main.uv": MAIN,
        "First/f.uv": "let broken = = 1\n",
        "Second/s.uv": "let also_broken = = 1\n",
    }),
    "import_error_in_root": (APP + lib("First"), {
        "Source/Main.uv": "import First\nlet broken = = 1\n",
        "First/f.uv": "let also_broken = = 1\n",
    }),
    "using_reaches_assembly": (APP + lib("First") + lib("Second"), {
        "Source/Main.uv": "using Second::helper\n" + MAIN,
        "First/f.uv": "let broken = = 1\n",
        "Second/s.uv": "public procedure helper() {\n}\nlet broken = = 1\n",
    }),
    "using_list_and_wildcard": (APP + lib("First") + lib("Second") + lib("Third"), {
        "Source/Main.uv": "using Third::{a, b}\nusing Second::*\n" + MAIN,
        "First/f.uv": "#derive()\nrecord R {\n    x: i32\n}\n",
        "Second/s.uv": "#derive(Eq, Eq)\nrecord R {\n    x: i32\n}\n",
        "Third/t.uv": "public procedure a() {\n}\npublic procedure b() {\n}\n",
    }),
    "import_submodule": (APP + lib("Lib"), {
        "Source/Main.uv": "import Lib::Inner\nimport Helper\n" + MAIN,
        "Source/Helper/h.uv": "public procedure h() {\n}\n",
        "Lib/l.uv": "public procedure l() {\n}\n",
        "Lib/Inner/i.uv": "public procedure i(loop: i32) {\n}\n",
    }),
    "import_unknown_and_cycle": (APP + lib("A") + lib("B"), {
        "Source/Main.uv": "import Nowhere\nimport A\n" + MAIN,
        "A/a.uv": "import B\nimport App\npublic procedure a() {\n}\n",
        "B/b.uv": "import A\npublic procedure b(self: i32) {\n}\n",
    }),
    "derive_stops_before_later_assembly": (APP + lib("Later"), {
        "Source/Main.uv": "#derive()\nrecord R {\n    x: i32\n}\n" + MAIN,
        "Later/l.uv": "let broken = = 1\n",
    }),
    "library_selected_first": (lib("Lib") + lib("Other"), {
        "Lib/l.uv": "public procedure l(loop: i32) {\n}\n",
        "Other/o.uv": "public procedure o(record: i32) {\n}\n",
    }),
}


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    cases = {name: (APP, {"Source/Main.uv": text}) for name, text in SINGLE.items()}
    cases.update(MULTI)
    entries = []
    for name, (manifest, files) in sorted(cases.items()):
        case_dir = OUT / name
        case_dir.mkdir(parents=True)
        (case_dir / "Ultraviolet.toml").write_text(manifest, encoding="utf-8")
        for rel, content in files.items():
            path = case_dir / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            if isinstance(content, bytes):
                path.write_bytes(content)
            else:
                path.write_text(content, encoding="utf-8", newline="")
        entries.append(f"target/p1cases/{name}/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "phase1_cases.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} phase-1 cases")


if __name__ == "__main__":
    main()
