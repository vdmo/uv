#!/usr/bin/env python3
"""Compare the Rust `uvc build --emit-ir` with the reference's, per declaration.

The goldens are tests/golden/<dir>/<id>.ir, recorded by tools/oracle/run_reference_ir.sh
for every project the reference accepts. A dump is a list of declarations, each starting
at a line that begins with `proc @`, `global_const @`, `global_zero @`, `vtable @` or
`extern_proc @`. A project for which the Rust `uvc` stops with status 3 is pending; for
the others, every reference declaration is compared by name and text.
"""
from __future__ import annotations

import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UVC = ROOT / "target" / "release" / "uvc"
ARGS = ["--emit-ir", "--build-progress", "off", "--target-profile", "x86_64-sysv", "--no-crash-report"]
HEADER = re.compile(r"^(proc|global_const|global_zero|vtable|extern_proc) @(\S+)")


def declarations(text: str) -> dict[str, str]:
    out: dict[str, str] = {}
    key = None
    lines: list[str] = []
    for line in text.splitlines():
        match = HEADER.match(line)
        if match:
            if key is not None:
                out[key] = "\n".join(lines).rstrip()
            key, lines = f"{match.group(1)} {match.group(2)}", [line]
        elif key is not None:
            lines.append(line)
    if key is not None:
        out[key] = "\n".join(lines).rstrip()
    return out


def main() -> int:
    stats: Counter[str] = Counter()
    problems: list[str] = []
    for name in sys.argv[1:] or ["projects", "project_cases", "phase1_cases"]:
        golden = ROOT / "tests" / "golden" / name
        for line in (golden / "results.tsv").read_text().splitlines():
            ident, rc, manifest = line.split("\t")
            reference = golden / f"{ident}.ir"
            if rc != "0" or not reference.exists():
                continue
            stats["projects"] += 1
            proc = subprocess.run([str(UVC), "build", "Ultraviolet.toml", *ARGS], cwd=(ROOT / manifest).parent,
                                  capture_output=True, text=True, timeout=600)
            if proc.returncode == 3:
                stats["pending_projects"] += 1
                continue
            want, got = declarations(reference.read_text()), declarations(proc.stdout.replace(str(ROOT) + "/", "/w/"))
            for key, text in want.items():
                if key not in got:
                    stats["missing"] += 1
                    problems.append(f"{name}/{ident}: missing {key}")
                elif got[key] != text:
                    stats["different"] += 1
                    problems.append(f"{name}/{ident}: {key} differs")
                else:
                    stats["identical"] += 1
            stats["extra"] += sum(1 for key in got if key not in want)
    print(" ".join(f"{k}={v}" for k, v in stats.items()))
    for problem in problems[:20]:
        print(problem)
    return 1 if stats["missing"] or stats["different"] or stats["extra"] else 0


if __name__ == "__main__":
    sys.exit(main())
