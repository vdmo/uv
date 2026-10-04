#!/usr/bin/env python3
"""Compare the Rust `uvc --phase1-only` with reference results captured by
tools/oracle/run_reference_phase1.sh.

Per fixture project, the exit status, the diagnostics JSON and the `--dump-ast` listing
must all be identical to the reference.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GOLDEN = ROOT / "tests" / "golden" / (sys.argv[1] if len(sys.argv) > 1 else "projects")
UVC = ROOT / "target" / "release" / "uvc"
REF_PREFIX = "/w/"
HOST_PREFIX = str(ROOT) + "/"
ARGS = ["--build-progress", "off", "--target-profile", "x86_64-sysv", "--no-crash-report"]


def run(manifest_dir: Path, extra: list[str]) -> tuple[int, str]:
    proc = subprocess.run(
        [str(UVC), "build", "Ultraviolet.toml", "--phase1-only", *extra, *ARGS],
        cwd=manifest_dir, capture_output=True, text=True, timeout=120,
    )
    return proc.returncode, proc.stdout.replace(HOST_PREFIX, REF_PREFIX)


def main() -> int:
    rows = [line.split("\t") for line in (GOLDEN / "phase1.tsv").read_text().splitlines()]
    stats = {"same": 0, "diag_diff": 0, "rc_diff": 0, "ast_diff": 0}
    problems: list[str] = []
    for ident, ref_rc, manifest in rows:
        manifest_dir = (ROOT / manifest).parent
        ref_json = (GOLDEN / f"{ident}.phase1.json").read_text().strip()
        ref_ast = (GOLDEN / f"{ident}.ast").read_text()
        rc, out = run(manifest_dir, ["--diag-json"])
        _, ast = run(manifest_dir, ["--dump-ast"])
        ok = True
        if out.strip() != ref_json:
            stats["diag_diff"] += 1
            ok = False
            problems.append(f"DIAG {manifest}:\n  ref ={ref_json[:400]}\n  rust={out.strip()[:400]}")
        if rc != int(ref_rc):
            stats["rc_diff"] += 1
            ok = False
            problems.append(f"RC {manifest}: rust {rc} vs reference {ref_rc}")
        if ast != ref_ast:
            stats["ast_diff"] += 1
            ok = False
            ref_lines, rust_lines = ref_ast.splitlines(), ast.splitlines()
            first = next((i for i, (a, b) in enumerate(zip(ref_lines, rust_lines)) if a != b),
                         min(len(ref_lines), len(rust_lines)))
            problems.append(
                f"AST {manifest}: line {first}\n"
                f"  ref ={ref_lines[first] if first < len(ref_lines) else '<end>'}\n"
                f"  rust={rust_lines[first] if first < len(rust_lines) else '<end>'}")
        stats["same"] += ok
    print(f"projects={len(rows)} " + " ".join(f"{k}={v}" for k, v in stats.items()))
    for problem in problems[:25]:
        print(problem)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
