#!/usr/bin/env python3
"""Generate manifest/layout edge-case projects under target/prjcases plus a CLI case list."""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "prjcases"

MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"
BASE = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'

MANIFESTS = {
    "ok_minimal": BASE,
    "ok_single_table": '[assembly]\nname = "App"\nkind = "executable"\nroot = "Source"\n',
    "ok_toolchain": BASE + '[toolchain]\ntarget_profile = "aarch64-darwin"\nruntime_lib = "x.a"\n',
    "ok_library_default": '[[assembly]]\nname = "Lib"\nkind = "library"\nroot = "Source"\n',
    "ok_library_static": '[[assembly]]\nname = "Lib"\nkind = "library"\nlink_kind = "static"\nroot = "Source"\nemit_ir = "bc"\n',
    "ok_dependency": '[[assembly]]\nname = "Dep"\nkind = "dependency"\nroot = "Source"\n',
    "ok_out_dir": BASE + 'out_dir = "Out/Dir"\nemit_ir = "ll"\n',
    "ok_build": BASE + "[build]\nincremental = true\nprogress = false\n",
    "ok_root_dot": '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "."\n',
    "ok_unicode_name": '[[assembly]]\nname = "Café"\nkind = "executable"\nroot = "Source"\n',
    "bad_toml": "[[assembly]\nname = ",
    "bad_empty": "",
    "bad_top_key": BASE + "[extra]\nx = 1\n",
    "bad_assembly_string": 'assembly = "nope"\n',
    "bad_assembly_empty_array": "assembly = []\n",
    "bad_assembly_mixed_array": 'assembly = [1, 2]\n',
    "bad_assembly_key": BASE + 'extra = "x"\n',
    "bad_missing_root": '[[assembly]]\nname = "App"\nkind = "executable"\n',
    "bad_name_type": '[[assembly]]\nname = 1\nkind = "executable"\nroot = "Source"\n',
    "bad_name_keyword": '[[assembly]]\nname = "loop"\nkind = "executable"\nroot = "Source"\n',
    "bad_name_ident": '[[assembly]]\nname = "1App"\nkind = "executable"\nroot = "Source"\n',
    "bad_name_empty": '[[assembly]]\nname = ""\nkind = "executable"\nroot = "Source"\n',
    "bad_kind": '[[assembly]]\nname = "App"\nkind = "program"\nroot = "Source"\n',
    "bad_dup_name": BASE + BASE,
    "bad_link_kind_use": BASE + 'link_kind = "static"\n',
    "bad_link_kind": '[[assembly]]\nname = "Lib"\nkind = "library"\nlink_kind = "dynamic"\nroot = "Source"\n',
    "bad_link_kind_type": '[[assembly]]\nname = "Lib"\nkind = "library"\nlink_kind = 3\nroot = "Source"\n',
    "bad_emit_ir": BASE + 'emit_ir = "asm"\n',
    "bad_emit_ir_type": BASE + "emit_ir = true\n",
    "bad_out_dir_type": BASE + "out_dir = 1\n",
    "bad_out_dir_abs": BASE + 'out_dir = "/abs"\n',
    "bad_out_dir_parent": BASE + 'out_dir = "../up"\n',
    "bad_root_abs": '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "/abs"\n',
    "bad_root_parent": '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "../x"\n',
    "bad_root_drive": '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "C:/x"\n',
    "bad_root_missing_dir": '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Nope"\n',
    "bad_toolchain_type": BASE + 'toolchain = "x"\n',
    "bad_toolchain_key": BASE + '[toolchain]\nfoo = "x"\n',
    "bad_toolchain_profile": BASE + '[toolchain]\ntarget_profile = "riscv"\n',
    "bad_toolchain_profile_type": BASE + "[toolchain]\ntarget_profile = 1\n",
    "bad_toolchain_llvm_type": BASE + "[toolchain]\nllvm_bin = 1\n",
    "bad_build_type": BASE + "build = 3\n",
    "bad_build_key": BASE + "[build]\nfast = true\n",
    "bad_build_value": BASE + '[build]\nincremental = "yes"\n',
    "two_executables": BASE + '[[assembly]]\nname = "Other"\nkind = "executable"\nroot = "Other"\n',
    "exe_and_lib": BASE + '[[assembly]]\nname = "Lib"\nkind = "library"\nroot = "Lib"\n',
    "nested_roots": BASE + '[[assembly]]\nname = "Inner"\nkind = "library"\nroot = "Source/Inner"\n',
    "same_roots": BASE + '[[assembly]]\nname = "Twin"\nkind = "library"\nroot = "Source"\n',
}

LAYOUTS = {
    "layout_nested": ["Source/Main.uv", "Source/B/Main.uv", "Source/a/Main.uv", "Source/A2/Deep/x.uv", "Source/Empty/readme.txt"],
    "layout_order": ["Source/b.uv", "Source/A.uv", "Source/a2.uv", "Source/Z.uv", "Source/_x.uv"],
    "layout_case_collision": ["Source/Main.uv", "Source/Mod/a.uv", "Source/mod/b.uv"],
    "layout_keyword_dir": ["Source/Main.uv", "Source/loop/a.uv"],
    "layout_bad_ident_dir": ["Source/Main.uv", "Source/9lives/a.uv"],
    "layout_dash_dir": ["Source/Main.uv", "Source/my-mod/a.uv"],
    "layout_unicode_dir": ["Source/Main.uv", "Source/Größe/a.uv", "Source/grosse/a.uv"],
    "layout_no_sources": ["Source/readme.txt"],
    "layout_dot_file": ["Source/Main.uv", "Source/.uv", "Source/Sub/.hidden.uv"],
}


def write_project(name: str, manifest: str, files: list[str]) -> None:
    root = OUT / name
    root.mkdir(parents=True)
    (root / "Ultraviolet.toml").write_text(manifest, encoding="utf-8")
    for rel in files:
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(MAIN if rel.endswith("Main.uv") else "// module file\n", encoding="utf-8")


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    for name, manifest in MANIFESTS.items():
        files = ["Source/Main.uv"]
        if name in ("two_executables",):
            files.append("Other/Main.uv")
        if name in ("exe_and_lib",):
            files.append("Lib/Main.uv")
        if name in ("nested_roots",):
            files.append("Source/Inner/Main.uv")
        if name == "ok_root_dot":
            files = ["Main.uv"]
        write_project(name, manifest, files)
    for name, files in LAYOUTS.items():
        write_project(name, BASE, files)
    (OUT / "no_manifest" / "Source").mkdir(parents=True)
    (OUT / "no_manifest" / "Source" / "Main.uv").write_text(MAIN)
    manifests = sorted(p.relative_to(ROOT).as_posix() for p in OUT.glob("*/Ultraviolet.toml"))
    manifests.append("target/prjcases/no_manifest/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "project_cases.list").write_text("\n".join(manifests) + "\n")
    print(f"project cases={len(manifests)}")


if __name__ == "__main__":
    main()
