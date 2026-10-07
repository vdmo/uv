#!/usr/bin/env python3
"""Compare the LLVM IR the Rust `uvc build` writes with the reference's, per function.

The goldens are tests/golden/<dir>/<id>.ll, recorded by tools/oracle/run_reference_ll.sh: the
`.ll` file of every module of the project, one after the other under `; ==== <path>` headers.
Each project is built in a scratch copy with `emit_ir = "ll"` in its manifest, the files the
build writes under Build/Intermediate/IR are joined the same way, and the two texts are
compared entity by entity: `define` bodies, `declare` lines, global variables and attribute
groups. Names of registers are not compared (they are numbered by definition order), nor
comments. What the Rust `uvc` leaves out because it is not ported is counted as pending.

  tools/parity_ll.py [-v] [id ...]       ids of tests/golden/projects (default: all)
"""
from __future__ import annotations

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


def split_files(text: str) -> dict[str, str]:
    files: dict[str, str] = {}
    name = None
    lines: list[str] = []
    for line in text.splitlines():
        if line.startswith("; ==== "):
            if name is not None:
                files[name] = "\n".join(lines)
            name, lines = line[7:].strip(), []
        elif name is not None:
            lines.append(line)
    if name is not None:
        files[name] = "\n".join(lines)
    return files


DEF = re.compile(r"^(define|declare)\b.*?@(\"[^\"]+\"|[^\s(]+)\(")
GLOBAL = re.compile(r"^@(\"[^\"]+\"|[^\s=]+) = ")
ATTRS = re.compile(r"^attributes #(\d+) = (.*)$")
REG_DEF = re.compile(r"^\s*(%[^\s=]+) = ")
TOKEN = re.compile(r"%(\"[^\"]+\"|[A-Za-z0-9_.$-]+)")


def entities(text: str) -> dict[str, str]:
    """The comparable pieces of one module's text, by key."""
    out: dict[str, str] = {}
    lines = text.splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        match = DEF.match(line)
        if match:
            kind, name = match.group(1), match.group(2)
            if kind == "define":
                body = [line]
                i += 1
                while i < len(lines) and lines[i] != "}":
                    body.append(lines[i])
                    i += 1
                out[f"define {name}"] = normalize_function(body)
            else:
                out[f"declare {name}"] = re.sub(r"\s+#\d+$", " #", line.strip())
            i += 1
            continue
        match = GLOBAL.match(line)
        if match:
            out[f"global {match.group(1)}"] = line.strip()
        elif line.startswith("attributes #"):
            attrs = ATTRS.match(line)
            if attrs:
                out[f"attributes {attrs.group(2)}"] = attrs.group(2)
        i += 1
    return out


def normalize_function(lines: list[str]) -> str:
    cleaned: list[str] = []
    for line in lines:
        line = re.sub(r"\s*;.*$", "", line).rstrip()
        if line.strip():
            cleaned.append(line)
    header = re.sub(r"\s+#\d+\s*\{$", " # {", cleaned[0])
    defined: dict[str, str] = {}
    for line in cleaned[1:]:
        match = REG_DEF.match(line)
        if match and match.group(1) not in defined:
            defined[match.group(1)] = f"%r{len(defined)}"
    body = []
    for line in cleaned[1:]:
        body.append(TOKEN.sub(lambda m: defined.get(m.group(0), m.group(0)), line.strip()))
    return "\n".join([header, *body])


def build(project: Path) -> tuple[int, str]:
    """The text the Rust uvc writes for a project directory, and its exit status."""
    with tempfile.TemporaryDirectory(prefix="llparity") as scratch:
        work = Path(scratch) / "p"
        shutil.copytree(project, work)
        shutil.rmtree(work / "Build", ignore_errors=True)
        manifest = work / "Ultraviolet.toml"
        text = manifest.read_text()
        if not re.search(r"^emit_ir", text, re.M):
            text = re.sub(r"^(root = .*)$", 'emit_ir = "ll"\n\\1', text, count=1, flags=re.M)
            manifest.write_text(text)
        proc = subprocess.run([str(UVC), *ARGS], cwd=work, capture_output=True, text=True, timeout=600)
        parts = []
        ir = work / "Build" / "Intermediate" / "IR"
        for path in sorted(ir.rglob("*.ll")) if ir.exists() else []:
            parts.append(f"; ==== {path.relative_to(ir)}\n{path.read_text()}")
        return proc.returncode, "".join(parts)


def main() -> int:
    verbose = "-v" in sys.argv
    ids = [a for a in sys.argv[1:] if not a.startswith("-")]
    golden = ROOT / "tests" / "golden" / "projects"
    stats: Counter[str] = Counter()
    problems: list[str] = []
    for line in (golden / "results.tsv").read_text().splitlines():
        ident, rc, manifest = line.split("\t")
        reference = golden / f"{ident}.ll"
        if rc != "0" or not reference.exists() or reference.stat().st_size == 0:
            continue
        if ids and ident not in ids:
            continue
        stats["projects"] += 1
        code, got_text = build(ROOT / manifest.rsplit("/", 1)[0])
        want_files, got_files = split_files(reference.read_text()), split_files(got_text)
        if code == 3:
            stats["pending_projects"] += 1
        for fname, want_text in want_files.items():
            want = entities(want_text)
            got = entities(got_files.get(fname, ""))
            for key, text in want.items():
                if key.startswith("attributes"):
                    continue
                if key not in got:
                    stats["pending_entities"] += 1
                elif got[key] == text:
                    stats["identical"] += 1
                else:
                    stats["different"] += 1
                    problems.append(f"{ident}: {key} differs")
            for key in got:
                if key not in want and not key.startswith("attributes"):
                    stats["extra"] += 1
                    problems.append(f"{ident}: {key} is extra")
    for p in problems[: (1000 if verbose else 25)]:
        print(p)
    print(" ".join(f"{k}={v}" for k, v in sorted(stats.items())))
    return 1 if stats["different"] or stats["extra"] else 0


if __name__ == "__main__":
    sys.exit(main())
