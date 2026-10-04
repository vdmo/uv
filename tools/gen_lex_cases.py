#!/usr/bin/env python3
"""Generate a deterministic lexical stress corpus under target/lexcases and its list file.

Hand-written edge cases cover every lexical diagnostic; the rest are seeded mutations of
real corpus files and random token soup. The oracle and the Rust lexer both consume the
resulting list file.
"""
from __future__ import annotations

import random
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "lexcases"

NUMERIC = """0 00 007 1_000 1__0 1_ 0x 0x_1 0x1_ 0xG 0xfF 0X1F 0b 0b102 0b1_0 0o 0o78 0o17 1e5 1E5 1e 1e+ 1e-3
1e_5 1_e5 1.5 1. 1.e5 1.5e-3 1.5e 1.5e+ 1.5E+10 1..2 1..=2 1.5f32 1.5f 1.5f16 1.5f64 1f 1f32 1_u8 1u8
1u128 1isize 1usize 0x1F_u8 0x1Fu8 1.0_f32 1.2.3 123abc 1.5i32 1e5u8 0b1u8 0o7i64 0x 9999999999999999999999
1_.5 1._5 1.5_ 1.5_e3 0.0 00.5 0_0 0e0 1i 1u 1i7 0xe 0xe+1 0b_ 0o_7 1__u8 12_usize 1_f 1.f32 01u8 1.0e1_0"""

SNIPPETS = [
    'x.0.1', 't.0', 't.0.5', 'a.1e5', 'a . 0.0', 'a.\n0.5', '"plain"', '"a\\"b"', '"a\\\\"', '"\\n\\r\\t\\0\\\\\\\'\\""',
    '"\\q"', '"\\x4"', '"\\x4g"', '"\\x41"', '"\\u{41}"', '"\\u{}"', '"\\u{110000}"', '"\\u{D800}"',
    '"\\u{10FFFF}"', '"\\u{0000041}"', '"\\u{41"', '"\\u41"', '"\\u{11111111111111111}"', '"unterminated',
    '"unterminated\nnext', '"a\\', "'a'", "'ab'", "''", "'\\n'", "'\\''", "'\\\\'", "'\\x41'", "'\\u{1F600}'",
    "'\\q'", "'unterminated", "'\n'", "'\\", '"mixed \' quote"', "'\"'", '"" ""', "'''", '"\\"', "x'y",
    '// line', '// line\nx', '/// doc', '///doc', '///  two', '//! module', '//!', '///', '////x', '//',
    '/* block */', '/* /* nested */ */', '/* /* unterminated */', '/*', '/**/', '/***/', '/*/ x */', 'a /* c */ b',
    '/* \n */ x\n', 'x // c\ny', '"// not comment"', "'/' '*'", '/ / /', '*/', 'a/*b*/c//d\ne',
    'café', 'café', 'café café', 'раураl', 'paypal раураl', 'a а', 'аb', 'x‍', 'x‌y',
    '‮ x', '// ‮ x', '"‮"', 'unsafe { ‮ }', 'unsafe\n{ x⁦ }', 'unsafe { } ⁩',
    'unsafe { { ‪ } }', 'unsafe {', '﻿x', 'x﻿y', '﻿', 'a\r\nb', 'a\rb', 'a\r\r\nb',
    'a\x00b', '"\x01"', "'\x02'", '// \x03', 'a\x7fb', 'a\tb\x0cc', '\x0b', 'a\u0085b', '"\u0085"',
    '+-*/%**==!=<<=>>=&&||!&|^<<>>=+=-=*=/=%=&=|:|=^=<<=>>=:=<:....==>->::~~>~!~%?#@$', '`', '\\', '€',
    'a\\b', '§', '()[]{},:;.', '...', '....', '..=', '.=', '=>=', '<<<', '>>>=', '~~>', '|::', '<::>',
    '_', '__', '_a', 'a_', '_1', 'true false null', 'trueish nullable', 'if else loop', 'gen_x', 'i32 u8',
    'x﷐', '\U0001fffe', 'a￿', '𝒳', '变量 = 1', 'ℌ', 'ᢅ', 'x́', '́x', 'Å Å', 'ǆ Ǆ',
    'K K', 'ﬁ fi', 'ı i', 'scope ѕcope', 'O 0 О', 'rn m', 'l I 1', 'ω ѡ', 'e е', 'Ω Ω',
    '#[attr]\nx', '#attr(a, b)\nprocedure', '#a::b\n#c\nrecord', 'a +\nb', 'a\n+ b', 'a ..\nb', 'a,\nb',
    '(a\nb)', '[a\nb]', '{a\nb}', '}\nelse', 'a\n.b', 'a\n::b', 'a\n~>b', 'a !\nb', 'a ?\nb', 'a ~\nb',
    '', '\n', '\n\n\n', ' ', '\t', 'x', 'x\n', '0', '"', "'", '/', '.', '1.', '.1', '1.x', '1.0.0', '1.0.x',
]

POOL = list(
    "abcxyz_ABC0123456789 \t\n\n\"'\\/+-*%=<>!&|^~?#@$()[]{},:;.eExXoObBuif"
) + [
    "é", "а", "е", "́", "‍", "‮", "﻿", "\r", "\x01", "\x7f", "€", "变", "𝒳", "￿",
    "//", "/*", "*/", "///", "//!", "unsafe", "{", "}", "0x", "0b", "0o", "..", "..=", "~>", "::", "u8",
    "f32", "isize", "1.5", "\\u{", "\\x", "true", "null", "else", "\x0c",
]


def write_case(index: int, data: bytes, entries: list[str]) -> None:
    path = OUT / f"case{index:05d}.uv"
    path.write_bytes(data)
    rel = path.relative_to(ROOT).as_posix()
    entries.append(f"lexcases/case{index:05d}.uv\t/w/{rel}")


def main() -> None:
    rng = random.Random(0x5556)
    OUT.mkdir(parents=True, exist_ok=True)
    for stale in OUT.glob("case*.uv"):
        stale.unlink()
    cases: list[bytes] = []
    for word in NUMERIC.split():
        cases.append(word.encode())
        cases.append(f"let x = {word}\n".encode())
        cases.append(f"t.{word}".encode())
    for snippet in SNIPPETS:
        cases.append(snippet.encode("utf-8"))
        cases.append(("let a = " + snippet + "\nlet b = 1\n").encode("utf-8"))
    cases += [b"\xff", b"a\xc0\x80", b"\xed\xa0\x80", b"\xf4\x90\x80\x80", b"\xe2\x82", b"ok \xf8\x88\x80\x80\x80",
              b"\xef\xbb\xbfx", b"\xc2", b"\xf0\x9f\x98\x80", b"x = \"\xf0\x9f\x98\x80\"", b"\xef\xbb\xbf\xef\xbb\xbf"]
    corpus = sorted((ROOT / "ultraviolet" / "HelloUltraviolet" / "Source" / "Reference").rglob("*.uv"))
    for _ in range(1500):
        text = list(rng.choice(corpus).read_text(encoding="utf-8")[: rng.randint(200, 3000)])
        for _ in range(rng.randint(1, 6)):
            pos = rng.randrange(len(text) + 1)
            action = rng.random()
            if action < 0.4 and text:
                del text[min(pos, len(text) - 1)]
            elif action < 0.8:
                text.insert(pos, rng.choice(POOL))
            else:
                text[pos:pos] = list(rng.choice(SNIPPETS))
        cases.append("".join(text).encode("utf-8", "surrogatepass"))
    for _ in range(2500):
        cases.append("".join(rng.choice(POOL) for _ in range(rng.randint(1, 60))).encode("utf-8"))
    for _ in range(300):
        cases.append(bytes(rng.randrange(256) for _ in range(rng.randint(1, 24))))
    entries: list[str] = []
    for index, data in enumerate(cases):
        write_case(index, data, entries)
    (ROOT / "tests" / "golden" / "lex_cases.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"cases={len(cases)}")


if __name__ == "__main__":
    main()
