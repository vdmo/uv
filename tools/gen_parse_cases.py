#!/usr/bin/env python3
"""Generate a deterministic parser stress corpus under target/parsecases and its list file.

Every case is a small corpus file with a few token-level edits (delete, duplicate, swap,
replace, insert), which drives the parser through its error and recovery paths while most
of the file stays well formed. Braces are left alone in most cases because the reference
parser does not terminate on blocks left open at end of file.
"""
from __future__ import annotations

import random
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "parsecases"
CASES = 6000
MAX_BYTES = 1500

TOKEN = re.compile(
    r"""//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\\n])*"|'(?:\\.|[^'\\\n])*'|[A-Za-z_][A-Za-z0-9_]*"""
    r"""|[0-9][0-9A-Za-z_.]*|\n|[ \t\r]+|::|->|=>|\.\.=|\.\.|<:|\|:|\|=|:=|==|!=|<=|>=|&&|\|\||<<|>>|~>|."""
    , re.S)

EXTRA = """let var procedure record enum modal class type using import extern comptime derive if else
loop match return break continue unsafe defer region frame move transmute widen as is in where
public internal private protected const unique shared override transition static parallel spawn
dispatch race all wait yield quote emit ( ) [ ] { } , : ; . .. ..= :: -> => = == != < > <= >= + - * /
% ** & | ^ ! ~ ? # @ $ <: |: |= := && || << >> ~> ~! ~% 0 1 0x1F 1.5 "s" 'c' true false null _ x T
i32 bool Self self""".split()


def tokens_of(text: str) -> list[str]:
    return TOKEN.findall(text)


def is_blank(tok: str) -> bool:
    return tok.strip(" \t\r") == ""


def main() -> None:
    rng = random.Random(0x7576)
    listing = (ROOT / "tests" / "golden" / "uv_files.list").read_text(encoding="utf-8").splitlines()
    sources = []
    for line in listing:
        path = ROOT / line.split("\t")[1].removeprefix("/w/")
        if not path.is_file() or path.stat().st_size > MAX_BYTES:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        toks = tokens_of(text)
        if sum(1 for tok in toks if not is_blank(tok)) >= 6:
            sources.append(toks)
    pool = sorted({tok for toks in sources for tok in toks if not is_blank(tok) and tok != "\n"
                   and not tok.startswith("//")}) + EXTRA
    OUT.mkdir(parents=True, exist_ok=True)
    entries = []
    for index in range(CASES):
        toks = list(rng.choice(sources))
        allow_braces = rng.random() < 0.15
        for _ in range(rng.choice((1, 1, 1, 2, 2, 3, 5))):
            slots = [i for i, tok in enumerate(toks)
                     if not is_blank(tok) and (allow_braces or tok not in "{}")]
            if not slots:
                break
            at = rng.choice(slots)
            action = rng.random()
            replacement = rng.choice(pool)
            if not allow_braces and replacement in ("{", "}"):
                replacement = ";"
            if action < 0.25:
                del toks[at]
            elif action < 0.40:
                toks.insert(at, toks[at])
            elif action < 0.55 and at + 1 < len(toks):
                toks[at], toks[at + 1] = toks[at + 1], toks[at]
            elif action < 0.80:
                toks[at] = replacement
            elif action < 0.95:
                toks[at:at] = [replacement, " "]
            else:
                toks[at] = "\n"
        name = f"case{index:05d}.uv"
        (OUT / name).write_text("".join(toks), encoding="utf-8", newline="")
        entries.append(f"parsecases/{name}\t/w/target/parsecases/{name}")
    (ROOT / "tests" / "golden" / "parse_cases.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {CASES} cases from {len(sources)} source files")


if __name__ == "__main__":
    main()
