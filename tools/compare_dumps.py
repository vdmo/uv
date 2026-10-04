#!/usr/bin/env python3
"""Compare a reference oracle dump with a Rust dump, file by file.

Files on which the reference itself crashed or did not terminate (a `CRASH` or `HANG`
record) are counted separately and excluded from the comparison; every other file must
match exactly. `--quiet` suppresses the per-file listing of those excluded files.
"""
from __future__ import annotations

import re
import sys

# The reference leaves `ForeignContractClause.kind` uninitialised when a foreign contract
# clause is malformed, and the oracle prints such an out-of-range value as `?`. The value
# is indeterminate, so it is not compared.
UNINIT_KIND = re.compile(r"\(ForeignContractClause kind=\S+ predicates=\[\]")


def same(ref: list[str], cand: list[str] | None) -> bool:
    if cand == ref:
        return True
    if cand is None or len(cand) != len(ref):
        return False
    for a, b in zip(ref, cand):
        if a != b:
            if "(ForeignContractClause kind=? " not in a:
                return False
            mask = "(ForeignContractClause kind=? predicates=[]"
            if UNINIT_KIND.sub(mask, a) != UNINIT_KIND.sub(mask, b):
                return False
    return True


def split(path: str) -> dict[str, list[str]]:
    files: dict[str, list[str]] = {}
    current: list[str] | None = None
    with open(path, encoding="utf-8", errors="surrogateescape", newline="\n") as handle:
        for line in handle:
            if line.startswith("F\t"):
                current = files.setdefault(line[2:].rstrip("\n"), [])
            elif current is not None:
                current.append(line)
    return files


def main() -> int:
    reference, candidate = split(sys.argv[1]), split(sys.argv[2])
    crashed = [name for name, lines in reference.items() if any(l.startswith(("CRASH\t", "HANG")) for l in lines)]
    mismatched = []
    for name, lines in reference.items():
        if name in crashed:
            continue
        if not same(lines, candidate.get(name)):
            mismatched.append(name)
    missing = [name for name in candidate if name not in reference]
    print(f"files={len(reference)} identical={len(reference) - len(crashed) - len(mismatched)} "
          f"mismatched={len(mismatched)} reference_crashed={len(crashed)} extra={len(missing)}")
    quiet = "--quiet" in sys.argv[3:]
    for name in [] if quiet else crashed:
        print(f"  reference crashed: {name} -> rust: {''.join(candidate.get(name, ['<missing>'])).strip()[:160]}")
    for name in mismatched[:20]:
        print(f"  MISMATCH {name}")
        ref, cand = reference[name], candidate.get(name, [])
        for index in range(max(len(ref), len(cand))):
            a = ref[index] if index < len(ref) else "<end>\n"
            b = cand[index] if index < len(cand) else "<end>\n"
            if a != b:
                print(f"    line {index}: ref={a.rstrip()!r}\n             rust={b.rstrip()!r}")
                break
    return 1 if mismatched or missing else 0


if __name__ == "__main__":
    sys.exit(main())
