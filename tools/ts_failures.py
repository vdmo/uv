#!/usr/bin/env python3
"""Summarise tree-sitter parse failures: for each failing file, the source text at the
first ERROR/MISSING node. Usage: ts_failures.py <tree-sitter --quiet output> [limit]"""
import re
import sys

limit = int(sys.argv[2]) if len(sys.argv) > 2 else 40
seen = {}
total = 0
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    m = re.match(r"^(\S+\.uv)\s.*\((ERROR|MISSING[^\[]*) \[(\d+), (\d+)\] - \[(\d+), (\d+)\]\)", line)
    if not m:
        continue
    total += 1
    path, kind, r1, c1, r2, c2 = m.group(1), m.group(2), int(m.group(3)), int(m.group(4)), int(m.group(5)), int(m.group(6))
    src = open(path, encoding="utf-8", errors="replace").read().split("\n")
    text = src[r1] if r1 < len(src) else ""
    key = re.sub(r"[A-Za-z_][A-Za-z0-9_]*", "I", text[c1:c1 + 30])
    seen.setdefault(key, []).append((path, kind, r1, c1, text))
print(f"failing files: {total}; distinct shapes: {len(seen)}")
for key, items in sorted(seen.items(), key=lambda kv: -len(kv[1]))[:limit]:
    path, kind, r1, c1, text = items[0]
    print(f"{len(items):4d} {kind.strip()} {path.split('ultraviolet-lang/')[-1]}:{r1 + 1}:{c1 + 1}\n       {text.strip()[:150]}\n       {' ' * max(0, c1 - (len(text) - len(text.lstrip())))}^")
