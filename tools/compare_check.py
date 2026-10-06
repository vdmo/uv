#!/usr/bin/env python3
"""Compare type-check dumps while the port of declaration typing is incomplete.

Each dump lists, per project, one `I` line per declaration with the diagnostics reported
inside it, and a `Z` line with the outcome and the remaining diagnostics. The port prints
`PENDING` and what it waits for in place of a declaration it cannot type yet, and in
place of the `Z` line while anything before it is pending; those are counted and not
compared. Every other line must match. Exits non-zero on any mismatch.
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
    items = collections.Counter()
    projects = collections.Counter()
    waiting: collections.Counter[str] = collections.Counter()
    shown = 0

    def mismatch(name: str, ref: str, cand: str) -> None:
        nonlocal shown
        if shown < 12:
            shown += 1
            print(f"  MISMATCH {name}\n    ref ={ref[:400]!r}\n    rust={cand[:400]!r}")

    for name, ref_lines in reference.items():
        if any(line.startswith(("CRASH\t", "HANG")) for line in ref_lines):
            projects["reference crashed"] += 1
            continue
        cand_lines = candidate.get(name)
        if cand_lines is None or len(cand_lines) != len(ref_lines):
            projects["mismatched"] += 1
            mismatch(name, f"{len(ref_lines)} lines", "none" if cand_lines is None else f"{len(cand_lines)} lines")
            continue
        whole = True
        bad = False
        for ref, cand in zip(ref_lines, cand_lines):
            fields = cand.split("\t")
            if ref.startswith("I\t"):
                if len(fields) >= 5 and fields[3] == "PENDING":
                    items["pending"] += 1
                    waiting[fields[4]] += 1
                    whole = False
                elif ref == cand:
                    items["compared"] += 1
                else:
                    items["compared"] += 1
                    items["mismatched"] += 1
                    bad = True
                    mismatch(name, ref, cand)
            elif ref.startswith("Z\t"):
                if len(fields) >= 3 and fields[1] == "PENDING":
                    waiting["project: " + fields[2]] += 1
                    whole = False
                elif ref != cand:
                    bad = True
                    mismatch(name, ref, cand)
            elif ref != cand:
                bad = True
                mismatch(name, ref, cand)
        projects["mismatched" if bad else "identical" if whole else "partly pending"] += 1
    total = items["compared"] + items["pending"]
    share = 100.0 * items["compared"] / total if total else 0.0
    print(f"declarations={total} compared={items['compared']} ({share:.1f}%) pending={items['pending']} "
          f"mismatched={items['mismatched']}; projects: identical={projects['identical']} "
          f"partly_pending={projects['partly pending']} mismatched={projects['mismatched']} "
          f"reference_crashed={projects['reference crashed']}")
    if "--waiting" in sys.argv[3:]:
        for what, count in waiting.most_common(25):
            print(f"  waiting for {what}: {count}")
    return 1 if projects["mismatched"] else 0


if __name__ == "__main__":
    sys.exit(main())
