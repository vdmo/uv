#!/usr/bin/env python3
"""Generate literal-heavy files under target/constcases and their list.

Each file is a run of literals, one per line, whose encoding as every primitive type is
compared with the reference: integers at the boundaries of each width and in each base,
floats at the rounding and range boundaries of each precision, character and string
literals with every escape, and malformed forms of each.
"""
from pathlib import Path
import random
import shutil
import struct

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "constcases"

INT_SUFFIXES = ["", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize"]
FLOAT_SUFFIXES = ["", "f", "f16", "f32", "f64"]


def ints() -> list[str]:
    out = []
    edges = [0, 1, 2, 7, 9, 10]
    for bits in (7, 8, 15, 16, 31, 32, 63, 64, 127, 128):
        edges += [2**bits - 1, 2**bits, 2**bits + 1]
    edges += [2**128 * 10, 10**38, 10**39, 340282366920938463463374607431768211455, 340282366920938463463374607431768211456]
    for value in edges:
        out += [str(value), hex(value), oct(value), bin(value), hex(value).upper().replace("0X", "0X")]
    for value in (0, 255, 256, 65535, 2**64 - 1):
        for suffix in INT_SUFFIXES:
            out.append(f"{value}{suffix}")
            out.append(f"{value:#x}{suffix}")
    out += ["1_000", "1__0", "0x_ff", "0xff_", "0b1010_1010", "0o7_7", "0x", "0b", "0o", "0b2", "0o8", "0xg",
            "00", "007", "0X1F", "0B11", "0O17", "1u8u8", "0xf64", "0x1f32", "1e", "0b1e1", "0xe1", "0x1e1",
            "1i", "1u", "0xabcdef", "0xABCDEF", "0xAbCdEf", "9" * 40, "0x" + "f" * 32, "0x" + "f" * 33,
            "0b" + "1" * 128, "0b" + "1" * 129, "0o" + "7" * 43, "0o3" + "7" * 42, "0o4" + "0" * 42]
    return out


def floats() -> list[str]:
    out = ["0.0", "1.0", "1.5", "0.1", "0.2", "0.3", "3.14159", "2.718281828459045", "1e0", "1e1", "1E1", "1e+1", "1e-1",
           "1.0e10", "1.0e-10", "1e308", "1e309", "1.7976931348623157e308", "1.7976931348623159e308", "1e-323",
           "1e-324", "4.9e-324", "2.4703282292062327e-324", "2.4703282292062328e-324", "2.2250738585072014e-308",
           "2.2250738585072011e-308", "0.000001", "123456789.123456789", "1_000.5", "1_0.0_1", "1.0_", "1._0",
           "1.", ".5", "1.e5", "1e", "1e+", "1.0e", "0x1.8p3", "1.0p3", "1e5f", "1e5f32", "00.5", "0.5e-0",
           "9007199254740993.0", "9007199254740992.0", "0.1e1", "1" + "0" * 400 + ".0", "0." + "0" * 400 + "1",
           "3.4028234e38", "3.4028235e38", "3.4028236e38", "3.4028235677973366e38", "3.40282357e38", "3.5e38",
           "1.17549435e-38", "1.4e-45", "7e-46", "7.1e-46", "1e-46", "16777217.0", "16777216.0", "16777219.0",
           "1.0000001", "1.00000001", "1.0000000596046448", "1.00000005960464477539", "1.00000005960464477540"]
    # Half precision: the largest finite value, the overflow boundary, the smallest
    # normal and subnormal values, and rounding ties around them.
    out += ["65504.0", "65519.0", "65519.99", "65520.0", "65520.01", "65536.0", "1e5", "6.1035156e-5", "6.1e-5",
            "6.0975552e-5", "5.9604645e-8", "2.9802322e-8", "2.98e-8", "3e-8", "8.9e-8", "1.4901161e-7", "1e-8",
            "1e-10", "1.0009765625", "1.00048828125", "1.000732421875", "1.0004882812", "1.0004882813",
            "1.00146484375", "2047.5", "2047.75", "2048.5", "0.99997", "0.999969482421875", "0.99998474121",
            "0.33325195", "0.333251953125", "0.3332824707", "1023.75", "1023.5", "1023.25", "32.015625", "32.0078125"]
    rng = random.Random(7)
    for _ in range(600):
        bits = rng.getrandbits(32)
        value = struct.unpack("<f", struct.pack("<I", bits & 0x7FFFFFFF))[0]
        if value == value and value != float("inf"):
            out.append(repr(value))
    for _ in range(400):
        mantissa = rng.randrange(1, 10**rng.randint(1, 20))
        out.append(f"{mantissa}.0e{rng.randint(-50, 45)}")
    for _ in range(300):
        # Near half-precision values, between two representable neighbours.
        half = rng.randrange(1, 0x7C00)
        exp, frac = half >> 10, half & 0x3FF
        value = (frac / 1024) * 2.0**-14 if exp == 0 else (1 + frac / 1024) * 2.0 ** (exp - 15)
        step = 2.0**-24 if exp == 0 else 2.0 ** (exp - 25)
        out.append(repr(value + step * rng.choice([-1.0, -0.5, -0.499, 0.0, 0.499, 0.5, 0.501, 1.0])))
    return [f"{value}{suffix}" for value in out for suffix in (FLOAT_SUFFIXES if len(value) < 12 else [""])]


def chars() -> list[str]:
    bodies = ["a", "Z", "0", " ", "é", "变", "😀", "\\\\", "\\'", '\\"', "\\n", "\\r", "\\t", "\\0", "\\x00", "\\x41",
              "\\x7f", "\\x80", "\\xff", "\\xFF", "\\xc3\\xa9", "\\xe5\\x8f\\x98", "\\xf0\\x9f\\x98\\x80",
              "\\xed\\xa0\\x80", "\\xc0\\x80", "\\xf4\\x90\\x80\\x80", "\\x4", "\\x", "\\xg1", "\\u{41}", "\\u{0}",
              "\\u{7F}", "\\u{80}", "\\u{7FF}", "\\u{800}", "\\u{FFFF}", "\\u{10000}", "\\u{10FFFF}", "\\u{110000}",
              "\\u{D800}", "\\u{DFFF}", "\\u{d7ff}", "\\u{E000}", "\\u{}", "\\u{41", "\\u41", "\\u{0000041}",
              "\\u{FFFFFFFFF}", "\\u{g}", "\\q", "\\a", "\\", "ab", "", "a\\n", "\\n\\n", "é́", "'", '"', "\\u{1F600}"]
    return [f"'{body}'" for body in bodies]


def strings() -> list[str]:
    bodies = ["", "plain", "with space", "é变😀", "\\\\", "\\'", '\\"', "\\n\\r\\t\\0", "\\x00\\x41\\xff", "\\xc3\\xa9",
              "\\xed\\xa0\\x80", "\\x4", "\\xg1", "tail\\x", "\\u{41}\\u{10FFFF}", "\\u{D800}", "\\u{110000}", "\\u{}",
              "\\u{41", "\\u41", "\\q", "a\\", "mixed \\u{1F600} é \\x7f end", "'single'", "a}b{c", "\\u{0}}",
              "line\\nbreak", "\\\\n", "\\\\\\\\", "tab\\there"]
    return [f'"{body}"' for body in bodies]


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True)
    groups = {"ints": ints(), "floats": floats(), "chars": chars(), "strings": strings(),
              "others": ["true", "false", "null", "-1", "-1.5", "- 1", "1 .5", "1..2", "1...2", "1.method", "1.0.0"]}
    entries = []
    total = 0
    for name, literals in groups.items():
        # A literal the lexer rejects must not hide its neighbours: each gets a file line
        # of its own, and the long groups are split so one failure costs little.
        for part, start in enumerate(range(0, len(literals), 40)):
            chunk = literals[start:start + 40]
            total += len(chunk)
            path = OUT / f"{name}{part:03}.uv"
            path.write_text("".join(f"{literal}\n" for literal in chunk), encoding="utf-8", newline="")
            entries.append(f"constcases/{path.name}\t/w/target/constcases/{path.name}")
    # And each literal the lexer may reject alone, so its rejection is compared too.
    for index, literal in enumerate(groups["chars"] + groups["strings"] + groups["ints"][-60:]):
        path = OUT / f"single{index:03}.uv"
        path.write_text(literal + "\n", encoding="utf-8", newline="")
        entries.append(f"constcases/{path.name}\t/w/target/constcases/{path.name}")
    (ROOT / "tests" / "golden" / "const_cases.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"files={len(entries)} literals={total}")


if __name__ == "__main__":
    main()
