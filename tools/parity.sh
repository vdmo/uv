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
[ -d target/ctcases ] || python3 tools/gen_comptime_cases.py > /dev/null
[ -d target/rescases ] || python3 tools/gen_resolve_cases.py > /dev/null
[ -d target/typecases ] || python3 tools/gen_type_cases.py > /dev/null
[ -d target/relcases ] || python3 tools/gen_relation_cases.py > /dev/null
[ -d target/constcases ] || python3 tools/gen_const_cases.py > /dev/null

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

echo "== compile-time pass (expanded modules and diagnostics): every project that passes phase 1"
./target/release/uv-parity comptime-list tests/golden/comptime_projects.list > target/parity/comptime.list
./target/release/uv-parity comptime target/parity/comptime.list | sed "s|$ROOT/|/w/|g" > target/parity/comptime.tsv
python3 tools/compare_dumps.py tests/golden/comptime.tsv target/parity/comptime.tsv --quiet || fail=1

echo "== compile-time pass: targeted cases"
./target/release/uv-parity comptime-list tests/golden/comptime_cases.list > target/parity/comptime_cases.list
./target/release/uv-parity comptime target/parity/comptime_cases.list | sed "s|$ROOT/|/w/|g" > target/parity/comptime_cases.tsv
python3 tools/compare_dumps.py tests/golden/comptime_cases.tsv target/parity/comptime_cases.tsv --quiet || fail=1

echo "== hash-table ordering the reference's output depends on"
cargo test --release --quiet -p uv-core std_unordered > target/parity/unordered.log 2>&1 \
  && echo "identical (probe of the reference's standard library)" || { cat target/parity/unordered.log; fail=1; }

echo "== built-in declaration table"
./target/release/uv-parity sigma > target/parity/sigma.tsv
cmp tests/golden/sigma.tsv target/parity/sigma.tsv && echo "identical ($(grep -c . tests/golden/sigma.tsv) declarations)" || fail=1

echo "== name resolution (name maps, resolved modules, diagnostics): every project that passes phase 1"
./target/release/uv-parity resolve target/parity/comptime.list | sed "s|$ROOT/|/w/|g" > target/parity/resolve.tsv
python3 tools/compare_dumps.py tests/golden/resolve.tsv target/parity/resolve.tsv --quiet || fail=1

echo "== name resolution: compile-time cases"
./target/release/uv-parity resolve target/parity/comptime_cases.list | sed "s|$ROOT/|/w/|g" > target/parity/resolve_cases.tsv
python3 tools/compare_dumps.py tests/golden/resolve_cases.tsv target/parity/resolve_cases.tsv --quiet || fail=1

echo "== name resolution: targeted cases"
./target/release/uv-parity comptime-list tests/golden/resolve_targeted.list > target/parity/resolve_targeted.list
./target/release/uv-parity resolve target/parity/resolve_targeted.list | sed "s|$ROOT/|/w/|g" > target/parity/resolve_targeted.tsv
python3 tools/compare_dumps.py tests/golden/resolve_targeted.tsv target/parity/resolve_targeted.tsv --quiet || fail=1

echo "== type core and layout (lowering, ordering, equivalence, lookup, substitution, variance; sizes, alignments, offsets, discriminants, niches): every project that resolves"
./target/release/uv-parity types target/parity/comptime.list | sed "s|$ROOT/|/w/|g" > target/parity/types.tsv
python3 tools/compare_dumps.py tests/golden/types.tsv target/parity/types.tsv --quiet || fail=1

echo "== type core and layout: compile-time cases"
./target/release/uv-parity types target/parity/comptime_cases.list | sed "s|$ROOT/|/w/|g" > target/parity/types_cases.tsv
python3 tools/compare_dumps.py tests/golden/types_cases.tsv target/parity/types_cases.tsv --quiet || fail=1

echo "== type core and layout: name-resolution cases"
./target/release/uv-parity types target/parity/resolve_targeted.list | sed "s|$ROOT/|/w/|g" > target/parity/types_targeted.tsv
python3 tools/compare_dumps.py tests/golden/types_targeted.tsv target/parity/types_targeted.tsv --quiet || fail=1

echo "== type core and layout: targeted cases"
./target/release/uv-parity comptime-list tests/golden/types_extra.list > target/parity/types_extra.list
./target/release/uv-parity types target/parity/types_extra.list | sed "s|$ROOT/|/w/|g" > target/parity/types_extra.tsv
python3 tools/compare_dumps.py tests/golden/types_extra.tsv target/parity/types_extra.tsv --quiet || fail=1

echo "== relations between types (well-formedness, intrinsic classes, subtyping, class tables, signatures, static proofs): every project that resolves"
./target/release/uv-parity relations target/parity/comptime.list | sed "s|$ROOT/|/w/|g" > target/parity/relations.tsv
python3 tools/compare_dumps.py tests/golden/relations.tsv target/parity/relations.tsv --quiet || fail=1

echo "== relations between types: compile-time cases"
./target/release/uv-parity relations target/parity/comptime_cases.list | sed "s|$ROOT/|/w/|g" > target/parity/relations_cases.tsv
python3 tools/compare_dumps.py tests/golden/relations_cases.tsv target/parity/relations_cases.tsv --quiet || fail=1

echo "== relations between types: name-resolution cases"
./target/release/uv-parity relations target/parity/resolve_targeted.list | sed "s|$ROOT/|/w/|g" > target/parity/relations_targeted.tsv
python3 tools/compare_dumps.py tests/golden/relations_targeted.tsv target/parity/relations_targeted.tsv --quiet || fail=1

echo "== relations between types: type-core cases"
./target/release/uv-parity relations target/parity/types_extra.list | sed "s|$ROOT/|/w/|g" > target/parity/relations_types.tsv
python3 tools/compare_dumps.py tests/golden/relations_types.tsv target/parity/relations_types.tsv --quiet || fail=1

echo "== relations between types: targeted cases"
./target/release/uv-parity comptime-list tests/golden/relations_extra.list > target/parity/relations_extra.list
./target/release/uv-parity relations target/parity/relations_extra.list | sed "s|$ROOT/|/w/|g" > target/parity/relations_extra.tsv
python3 tools/compare_dumps.py tests/golden/relations_extra.tsv target/parity/relations_extra.tsv --quiet || fail=1

echo "== constant encoding (every literal as every primitive type, string literal bytes): corpus"
./target/release/uv-parity consts tests/golden/uv_files.list --root "$ROOT" > target/parity/consts.tsv
python3 tools/compare_dumps.py tests/golden/consts.tsv target/parity/consts.tsv --quiet || fail=1

echo "== constant encoding: lexical stress cases"
./target/release/uv-parity consts tests/golden/lex_cases.list --root "$ROOT" > target/parity/consts_lex_cases.tsv
python3 tools/compare_dumps.py tests/golden/consts_lex_cases.tsv target/parity/consts_lex_cases.tsv --quiet || fail=1

echo "== constant encoding: targeted literals"
./target/release/uv-parity consts tests/golden/const_cases.list --root "$ROOT" > target/parity/const_cases.tsv
python3 tools/compare_dumps.py tests/golden/const_cases.tsv target/parity/const_cases.tsv --quiet || fail=1

for pair in "comptime:values" "comptime_cases:values_cases" "resolve_targeted:values_targeted" "types_extra:values_types" "relations_extra:values_relations"; do
  echo "== value bytes and validity (values built from each module's types): ${pair##*:}"
  ./target/release/uv-parity values "target/parity/${pair%%:*}.list" | sed "s|$ROOT/|/w/|g" > "target/parity/${pair##*:}.tsv"
  python3 tools/compare_dumps.py "tests/golden/${pair##*:}.tsv" "target/parity/${pair##*:}.tsv" --quiet || fail=1
done

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
