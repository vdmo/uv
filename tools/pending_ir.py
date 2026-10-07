#!/usr/bin/env python3
"""Counts, over the golden projects, what stops each declaration from lowering.

Runs `uvc build --emit-ir` on every project the reference accepts and counts the
`pending:` lines it prints on stderr. The numbering of bindings is left out: it only
follows from an earlier declaration that is pending.
"""
import collections
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UVC = ROOT / "target" / "release" / "uvc"
ARGS = ["--emit-ir", "--build-progress", "off", "--target-profile", "x86_64-sysv", "--no-crash-report"]

counts: collections.Counter[str] = collections.Counter()
for name in ["projects", "project_cases", "phase1_cases"]:
    golden = ROOT / "tests" / "golden" / name
    for line in (golden / "results.tsv").read_text().splitlines():
        ident, rc, manifest = line.split("\t")
        if rc != "0" or not (golden / f"{ident}.ir").exists():
            continue
        proc = subprocess.run([str(UVC), "build", "Ultraviolet.toml", *ARGS], cwd=(ROOT / manifest).parent, capture_output=True, text=True)
        for text in proc.stderr.splitlines():
            if text.startswith("pending: ") and "numbering of bindings" not in text:
                counts[text.split(": ", 2)[2]] += 1
for reason, n in counts.most_common(int(sys.argv[1]) if len(sys.argv) > 1 else 25):
    print(n, reason)
