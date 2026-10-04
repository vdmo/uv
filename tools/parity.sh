#!/bin/sh
# Runs every parity gate of the Rust port against goldens captured from the reference
# compiler (v0.4.0-alpha). Regenerate goldens with tools/capture_goldens.sh.
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
cargo build --release --quiet
mkdir -p target/parity
fail=0

# Generated inputs live under target/ and are deterministic; recreate them after a clean.
[ -d target/lexcases ] || python3 tools/gen_lex_cases.py > /dev/null
[ -d target/parsecases ] || python3 tools/gen_parse_cases.py > /dev/null
[ -d target/prjcases ] || python3 tools/gen_project_cases.py > /dev/null
[ -d target/p1cases ] || python3 tools/gen_phase1_cases.py > /dev/null

echo "== unicode properties (all scalar values vs ICU 72)"
./target/release/uv-parity unicode > target/parity/unicode.tsv
cmp tests/golden/unicode.tsv target/parity/unicode.tsv && echo "identical" || fail=1

echo "== tokens: corpus"
./target/release/uv-parity tokens tests/golden/uv_files.list --root "$ROOT" > target/parity/tokens.tsv
python3 tools/compare_dumps.py tests/golden/tokens.tsv target/parity/tokens.tsv || fail=1

echo "== tokens: lexical stress cases"
./target/release/uv-parity tokens tests/golden/lex_cases.list --root "$ROOT" > target/parity/lex_cases.tsv
python3 tools/compare_dumps.py tests/golden/lex_cases.tsv target/parity/lex_cases.tsv || fail=1

echo "== syntax trees: corpus"
./target/release/uv-parity ast tests/golden/uv_files.list --root "$ROOT" > target/parity/ast.tsv
python3 tools/compare_dumps.py tests/golden/ast.tsv target/parity/ast.tsv || fail=1

echo "== syntax trees: lexical stress cases"
./target/release/uv-parity ast tests/golden/lex_cases.list --root "$ROOT" > target/parity/ast_lex_cases.tsv
python3 tools/compare_dumps.py tests/golden/ast_lex_cases.tsv target/parity/ast_lex_cases.tsv --quiet || fail=1

echo "== syntax trees: parser stress cases (mutated corpus files)"
./target/release/uv-parity ast tests/golden/parse_cases.list --root "$ROOT" > target/parity/ast_parse_cases.tsv
python3 tools/compare_dumps.py tests/golden/ast_parse_cases.tsv target/parity/ast_parse_cases.tsv --quiet || fail=1

echo "== phase 1 (parse, syntactic checks, --dump-ast): conformance fixtures"
python3 tools/parity_phase1.py projects || fail=1

echo "== phase 1: manifest and layout cases"
python3 tools/parity_phase1.py project_cases || fail=1

echo "== phase 1: module-level cases"
python3 tools/parity_phase1.py phase1_cases || fail=1

echo "== projects: conformance fixtures"
python3 tools/parity_projects.py projects || fail=1

echo "== projects: manifest and layout cases"
python3 tools/parity_projects.py project_cases || fail=1

echo "== command line"
(cd /tmp && n=0 && while IFS= read -r line; do
  n=$((n + 1))
  out=$("$ROOT/target/release/uvc" $line 2>"$ROOT/target/parity/cli.err") && rc=0 || rc=$?
  printf '### %s | %s\nrc=%s\n--stdout--\n%s\n--stderr--\n%s\n' "$n" "$line" "$rc" "$out" "$(cat "$ROOT/target/parity/cli.err")"
done < "$ROOT/tests/golden/cli_cases.txt") > target/parity/cli_cases.out
cmp tests/golden/cli_cases.out target/parity/cli_cases.out && echo "identical ($(grep -c '^###' tests/golden/cli_cases.out) command lines)" || fail=1

echo "== human-readable output for phase 1 (rejected projects and module-level cases)"
grep '^### ' tests/golden/text_render.out | sed 's/^### //' | while read -r manifest; do
  printf '### %s\n' "$manifest"
  (cd "$(dirname "$manifest")" && "$ROOT/target/release/uvc" build Ultraviolet.toml --phase1-only --color never \
    --target-profile x86_64-sysv --no-crash-report 2>&1 | sed 's/ in [0-9.]*m\?s$/ in <t>/' | sed "s|$ROOT/|/w/|g") || true
done > target/parity/text_render.out
cmp tests/golden/text_render.out target/parity/text_render.out && echo "identical ($(grep -c '^###' tests/golden/text_render.out) projects)" || fail=1

[ "$fail" -eq 0 ] && echo "ALL PARITY GATES PASS" || { echo "PARITY FAILURES"; exit 1; }
