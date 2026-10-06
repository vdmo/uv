#!/usr/bin/env python3
"""Compare the Rust `uvc build --check --diag-json` with the reference's, per project.

The goldens are the ones tools/oracle/run_reference_projects.sh recorded: for each
project the exit status and the diagnostics JSON. A project for which the Rust `uvc`
stops with status 3 (a phase that is not ported yet) is reported as pending and not
compared; any other difference is a mismatch and fails the run.
"""
from __future__ import annotations

import subprocess
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UVC = ROOT / "target" / "release" / "uvc"
REF_PREFIX = "/w/"
HOST_PREFIX = str(ROOT) + "/"
ARGS = ["--check", "--diag-json", "--build-progress", "off", "--target-profile", "x86_64-sysv", "--no-crash-report"]
NOT_IMPLEMENTED = 3


def main() -> int:
    dirs = sys.argv[1:] or ["projects", "project_cases", "phase1_cases"]
    stats: Counter[str] = Counter()
    waiting: Counter[str] = Counter()
    problems: list[str] = []
    for name in dirs:
        golden = ROOT / "tests" / "golden" / name
        for line in (golden / "results.tsv").read_text().splitlines():
            ident, ref_rc, manifest = line.split("\t")
            manifest_dir = (ROOT / manifest).parent
            proc = subprocess.run([str(UVC), "build", "Ultraviolet.toml", *ARGS], cwd=manifest_dir,
                                  capture_output=True, text=True, timeout=300)
            stats["projects"] += 1
            if proc.returncode == NOT_IMPLEMENTED:
                stats["pending"] += 1
                what = proc.stderr.strip().splitlines()[-1] if proc.stderr.strip() else "?"
                waiting[what.removeprefix("error: ").split(" is not implemented")[0]] += 1
                continue
            out = proc.stdout.replace(HOST_PREFIX, REF_PREFIX).strip()
            ref = (golden / f"{ident}.json").read_text().strip()
            if proc.returncode != int(ref_rc) or out != ref:
                stats["mismatched"] += 1
                problems.append(f"{name}/{ident} {manifest}: rc rust {proc.returncode} vs {ref_rc}\n  ref ={ref[:300]}\n  rust={out[:300]}")
            else:
                stats["identical"] += 1
    print(" ".join(f"{k}={v}" for k, v in stats.items()))
    for what, count in waiting.most_common():
        print(f"  waiting for {what}: {count}")
    for problem in problems[:20]:
        print(problem)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
