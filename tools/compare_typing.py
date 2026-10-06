#!/usr/bin/env python3
"""Compare body-typing dumps while the port of the typer is incomplete.

Each dump has one `B` line per procedure, method or transition body. The port prints `PENDING` in place of a
result when typing the body reached a construct that is not ported yet; those bodies are
counted, with what they are waiting for, and not compared. Every other line must match.
Exits non-zero on any mismatch; pending bodies do not fail the gate.
"""
from __future__ import annotations

import collections
import sys


def split(path: str) -> dict[str, list[str]]:
    files: dict[str, list[str]] = {}
    current: list[str] | None = None
    with open(path, encoding="utf-8", errors="surrogateescape", newline="\n") as handle:
        for line in handle:
            if line.startswith("F\t"):
                current = files.setdefault(line[2:].rstrip("\n"), [])
            elif current is not None:
                current.append(line.rstrip("\n"))
    return files


def main() -> int:
    reference, candidate = split(sys.argv[1]), split(sys.argv[2])
    verbose = "--verbose" in sys.argv[3:]
    compared = pending = mismatched = crashed = 0
    waiting: collections.Counter[str] = collections.Counter()
    shown = 0
    for name, ref_lines in reference.items():
        if any(line.startswith(("CRASH\t", "HANG")) for line in ref_lines):
            crashed += 1
            continue
        cand_lines = candidate.get(name)
        if cand_lines is None or len(cand_lines) != len(ref_lines):
            mismatched += 1
            if shown < 12:
                shown += 1
                print(f"  MISMATCH {name}: {len(ref_lines)} lines in the reference, "
                      f"{'none' if cand_lines is None else len(cand_lines)} in the port")
            continue
        for ref, cand in zip(ref_lines, cand_lines):
            if not ref.startswith("B\t"):
                if ref != cand:
                    mismatched += 1
                continue
            fields = cand.split("\t")
            if len(fields) >= 4 and fields[2] == "PENDING":
                pending += 1
                waiting[fields[3]] += 1
                continue
            compared += 1
            if ref != cand:
                mismatched += 1
                if shown < 12:
                    shown += 1
                    print(f"  MISMATCH {name}\n    ref ={ref[:400]!r}\n    rust={cand[:400]!r}")
    total = compared + pending
    share = 100.0 * compared / total if total else 0.0
    print(f"bodies={total} compared={compared} ({share:.1f}%) pending={pending} mismatched={mismatched} "
          f"reference_crashed={crashed}")
    if verbose or "--waiting" in sys.argv[3:]:
        for what, count in waiting.most_common(25):
            print(f"  waiting for {what}: {count}")
    return 1 if mismatched else 0


if __name__ == "__main__":
    sys.exit(main())
