#!/usr/bin/env python3
"""Compare the Rust `uvc` with reference results captured by tools/oracle/run_reference_projects.sh.

Checks, per fixture project:
  * the `--dump` report (project model, output paths, source file order);
  * for projects the reference rejects in phase 1 (project load, lexing, parsing and the
    syntactic checks): diagnostics JSON and exit status are identical;
  * for every other project: the Rust driver stops after phase 1 with exactly the
    diagnostics the reference reports for phase 1, and exit status 3 (later phases are
    not ported yet).
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
NOT_IMPLEMENTED_RC = 3
ARGS = ["--build-progress", "off", "--target-profile", "x86_64-sysv", "--no-crash-report"]


def run(manifest_dir: Path, extra: list[str]) -> tuple[int, str]:
    proc = subprocess.run(
        [str(UVC), "build", "Ultraviolet.toml", "--check", *extra, *ARGS],
        cwd=manifest_dir, capture_output=True, text=True, timeout=120,
    )
    return proc.returncode, proc.stdout.replace(HOST_PREFIX, REF_PREFIX)


def main() -> int:
    rows = [line.split("\t") for line in (GOLDEN / "results.tsv").read_text().splitlines()]
    phase1_rc = {ident: int(rc) for ident, rc, _ in
                 (line.split("\t") for line in (GOLDEN / "phase1.tsv").read_text().splitlines())}
    stats = {"dump_same": 0, "dump_diff": 0, "dump_missing": 0, "rejected_same": 0, "rejected_diff": 0,
             "phase1_same": 0, "phase1_diff": 0}
    problems: list[str] = []
    for ident, ref_rc, manifest in rows:
        manifest_dir = (ROOT / manifest).parent
        ref_json_text = (GOLDEN / f"{ident}.json").read_text().strip()
        ref_phase1_text = (GOLDEN / f"{ident}.phase1.json").read_text().strip()
        ref_dump = (GOLDEN / f"{ident}.dump").read_text()
        rc, out = run(manifest_dir, ["--diag-json"])
        out = out.strip()
        if phase1_rc[ident] != 0:
            if out == ref_json_text and rc == int(ref_rc):
                stats["rejected_same"] += 1
            else:
                stats["rejected_diff"] += 1
                problems.append(f"REJECTED {manifest}: rc {rc} vs {ref_rc}\n  ref ={ref_json_text[:300]}\n  rust={out[:300]}")
        else:
            if out == ref_phase1_text and rc == NOT_IMPLEMENTED_RC:
                stats["phase1_same"] += 1
            else:
                stats["phase1_diff"] += 1
                problems.append(f"PHASE1 {manifest}: rc {rc}\n  ref ={ref_phase1_text[:300]}\n  rust={out[:300]}")
        if ref_dump:
            _, dump_out = run(manifest_dir, ["--dump"])
            dump = "".join(l + "\n" for l in dump_out.splitlines() if l.startswith(("<", "file:")))
            if dump == ref_dump:
                stats["dump_same"] += 1
            elif not dump:
                stats["dump_missing"] += 1
                problems.append(f"DUMP-MISSING {manifest}")
            else:
                stats["dump_diff"] += 1
                ref_lines, rust_lines = ref_dump.splitlines(), dump.splitlines()
                first = next((i for i, (a, b) in enumerate(zip(ref_lines, rust_lines)) if a != b), min(len(ref_lines), len(rust_lines)))
                problems.append(f"DUMP {manifest}: line {first}\n  ref ={ref_lines[first] if first < len(ref_lines) else '<end>'}\n  rust={rust_lines[first] if first < len(rust_lines) else '<end>'}")
    print(f"projects={len(rows)} " + " ".join(f"{k}={v}" for k, v in stats.items()))
    for problem in problems[:25]:
        print(problem)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
