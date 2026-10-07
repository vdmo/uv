#!/usr/bin/env python3
"""Counts why procedures are left out of the LLVM IR the Rust `uvc` writes, over the goldens.

The compiler is run with UV_EMIT_REPORT set; each `emit-pending:` line names a procedure and
what stopped it. `tools/pending_ll.py [N]` prints the N most common reasons.
"""
import os
import re
import shutil
import subprocess
import sys
import tempfile
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UVC = ROOT / "target" / "release" / "uvc"
ARGS = ["build", "Ultraviolet.toml", "--target-profile", "x86_64-sysv", "--build-progress", "off", "--no-crash-report"]


def main() -> None:
    top = int(sys.argv[1]) if len(sys.argv) > 1 else 25
    golden = ROOT / "tests" / "golden" / "projects"
    counts: Counter[str] = Counter()
    env = dict(os.environ, UV_EMIT_REPORT="1")
    for line in (golden / "results.tsv").read_text().splitlines():
        ident, rc, manifest = line.split("\t")
        reference = golden / f"{ident}.ll"
        if rc != "0" or not reference.exists() or reference.stat().st_size == 0:
            continue
        with tempfile.TemporaryDirectory(prefix="llpending") as scratch:
            work = Path(scratch) / "p"
            shutil.copytree(ROOT / manifest.rsplit("/", 1)[0], work)
            shutil.rmtree(work / "Build", ignore_errors=True)
            proc = subprocess.run([str(UVC), *ARGS], cwd=work, capture_output=True, text=True, env=env, timeout=600)
            for text in proc.stderr.splitlines():
                match = re.match(r"emit-pending: \S+: (.*)", text)
                if match:
                    counts[match.group(1)] += 1
                elif text.startswith("error: code generation is not implemented") and "the lowerability check" in text:
                    counts["lowering: " + text.split("(", 1)[1].rsplit(")", 1)[0]] += 1
    for reason, count in counts.most_common(top):
        print(f"{count:6} {reason}")


if __name__ == "__main__":
    main()
