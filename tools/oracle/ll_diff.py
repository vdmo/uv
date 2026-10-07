#!/usr/bin/env python3
"""ll_diff.py <project dir> [-s]: the LLVM IR of the reference and of the Rust compiler for one
project, compared entity by entity (see tools/parity_ll.py). Prints the differing entities as
diffs of their normalized text; -s keeps the two texts in /tmp/ll_ref.ll and /tmp/ll_rust.ll.
"""
import difflib
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
import parity_ll  # noqa: E402


def reference_text(project: Path) -> str:
    with tempfile.TemporaryDirectory(prefix="llref", dir=ROOT / "target") as scratch:
        work = Path(scratch) / "p"
        shutil.copytree(project, work)
        shutil.rmtree(work / "Build", ignore_errors=True)
        manifest = work / "Ultraviolet.toml"
        text = manifest.read_text()
        if not re.search(r"^emit_ir", text, re.M):
            text = re.sub(r"^(root = .*)$", 'emit_ir = "ll"\n\\1', text, count=1, flags=re.M)
            manifest.write_text(text)
        rel = work.relative_to(ROOT)
        subprocess.run(
            ["docker", "run", "--rm", "-v", f"{ROOT}:/w", "-w", f"/w/{rel}", "uv-oracle-link", "sh", "-c",
             "/w/reference/ultraviolet/uvc build Ultraviolet.toml --target-profile x86_64-sysv --build-progress off --no-crash-report --runtime-lib /w/reference/ultraviolet/UltravioletRT.a > build.out 2> build.err; chmod -R a+rwX Build 2>/dev/null"],
            capture_output=True, text=True, timeout=600)
        err = (work / "build.err").read_text() if (work / "build.err").exists() else ""
        ir = work / "Build" / "Intermediate" / "IR"
        parts = [f"; ==== {p.relative_to(ir)}\n{p.read_text()}" for p in sorted(ir.rglob("*.ll"))] if ir.exists() else []
        if not parts:
            print("reference produced no IR:\n" + err[-2000:])
        text = "".join(parts)
        shutil.rmtree(work, ignore_errors=True)
        return text


def main() -> int:
    project = Path(sys.argv[1]).resolve()
    keep = "-s" in sys.argv
    want_text = reference_text(project)
    code, got_text = parity_ll.build(project)
    if keep:
        Path("/tmp/ll_ref.ll").write_text(want_text)
        Path("/tmp/ll_rust.ll").write_text(got_text)
    want_files, got_files = parity_ll.split_files(want_text), parity_ll.split_files(got_text)
    different = 0
    for name, text in want_files.items():
        want = parity_ll.entities(text)
        got = parity_ll.entities(got_files.get(name, ""))
        for key, value in want.items():
            if key.startswith("attributes"):
                continue
            if key not in got:
                print(f"missing {key}")
            elif got[key] != value:
                different += 1
                print(f"--- {key}")
                for line in difflib.unified_diff(value.split("\n"), got[key].split("\n"), "reference", "rust", lineterm="", n=2):
                    print(line)
        for key in got:
            if key not in want and not key.startswith("attributes"):
                print(f"extra {key}")
    print(f"rust exit {code}; {different} entities differ")
    return 1 if different else 0


if __name__ == "__main__":
    sys.exit(main())
