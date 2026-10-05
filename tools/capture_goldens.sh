#!/bin/sh
# Regenerates every golden under tests/golden from the reference compiler. Needs Docker.
# Inputs: ultraviolet/ and ultraviolet-lsp/ checkouts, reference/ultraviolet (release
# archive v0.4.0-alpha), and the extern payload (Tools/FetchTargetExterns.py).
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
ICU=/w/ultraviolet/Bootstrap/extern/icu/linux/lib
run() { docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -e LD_LIBRARY_PATH="$ICU" "$@"; }

docker build -q -t uv-oracle tools/oracle
run uv-oracle sh /w/tools/oracle/build.sh
mkdir -p tests/golden target/fixtures

find ultraviolet ultraviolet-lsp -name '*.uv' -not -path '*/Build/*' | LC_ALL=C sort \
  | awk '{print $0 "\t/w/" $0}' > tests/golden/uv_files.list
python3 tools/gen_lex_cases.py
python3 tools/gen_parse_cases.py
# The oracle runs every file in a child process with a 3 second limit and records CRASH or
# HANG for inputs the reference cannot process, so the stress captures take a while.
run uv-oracle sh -c '/w/reference/oracle/uv-oracle unicode > /w/tests/golden/unicode.tsv
  /w/reference/oracle/uv-oracle tokens /w/tests/golden/uv_files.list > /w/tests/golden/tokens.tsv
  /w/reference/oracle/uv-oracle tokens /w/tests/golden/lex_cases.list > /w/tests/golden/lex_cases.tsv
  /w/reference/oracle/uv-oracle ast /w/tests/golden/uv_files.list > /w/tests/golden/ast.tsv
  /w/reference/oracle/uv-oracle ast /w/tests/golden/lex_cases.list > /w/tests/golden/ast_lex_cases.tsv
  /w/reference/oracle/uv-oracle ast /w/tests/golden/parse_cases.list > /w/tests/golden/ast_parse_cases.tsv'

rm -rf target/fixtures/Fixtures target/fixtures/shapes
cp -r ultraviolet/HelloUltraviolet/Fixtures target/fixtures/Fixtures
cp -r ultraviolet-lsp/examples/shapes target/fixtures/shapes
find target/fixtures -name Ultraviolet.toml | LC_ALL=C sort > tests/golden/fixture_projects.list
python3 tools/gen_project_cases.py
run ubuntu:24.04 sh /w/tools/oracle/run_reference_projects.sh projects fixture_projects.list
run ubuntu:24.04 sh /w/tools/oracle/run_reference_projects.sh project_cases project_cases.list
python3 tools/gen_phase1_cases.py
run ubuntu:24.04 sh /w/tools/oracle/run_reference_phase1.sh projects fixture_projects.list
run ubuntu:24.04 sh /w/tools/oracle/run_reference_phase1.sh project_cases project_cases.list
run ubuntu:24.04 sh /w/tools/oracle/run_reference_phase1.sh phase1_cases phase1_cases.list

# Compile-time pass. The project lists are produced by the Rust phase 1 (which is gated
# above); the oracle runs with the workspace mounted at its host path so that the project
# files capability sees the same paths on both sides, and the paths are normalised after.
python3 tools/gen_comptime_cases.py
cargo build --release --quiet -p uv-parity
(cat tests/golden/fixture_projects.list tests/golden/project_cases.list tests/golden/phase1_cases.list
 echo ultraviolet/HelloUltraviolet/Ultraviolet.toml) > tests/golden/comptime_projects.list
for name in comptime_projects comptime_cases; do
  ./target/release/uv-parity comptime-list "tests/golden/$name.list" > "target/parity/$name.oracle.list"
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle comptime "$ROOT/target/parity/$name.oracle.list" \
    | sed "s|$ROOT/|/w/|g" > "tests/golden/$(echo "$name" | sed 's/_projects//').tsv"
done

# Name resolution: the same project lists, plus cases aimed at the resolver, run through
# the reference's name collection and module resolution. Then the built-in declaration
# table, and the iteration order of the standard library's hash table, which decides the
# reference's spelling suggestions.
python3 tools/gen_resolve_cases.py
for name in comptime_projects comptime_cases resolve_targeted; do
  ./target/release/uv-parity comptime-list "tests/golden/$name.list" > "target/parity/$name.oracle.list"
  case "$name" in
    comptime_projects) out=resolve ;;
    comptime_cases) out=resolve_cases ;;
    *) out=$name ;;
  esac
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle resolve "$ROOT/target/parity/$name.oracle.list" \
    | sed "s|$ROOT/|/w/|g" > "tests/golden/$out.tsv"
done
# The type core and layout, on the same lists and on cases of their own.
python3 tools/gen_type_cases.py
for name in comptime_projects comptime_cases resolve_targeted types_extra; do
  ./target/release/uv-parity comptime-list "tests/golden/$name.list" > "target/parity/$name.oracle.list"
  case "$name" in
    comptime_projects) out=types ;;
    comptime_cases) out=types_cases ;;
    resolve_targeted) out=types_targeted ;;
    *) out=$name ;;
  esac
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle types "$ROOT/target/parity/$name.oracle.list" \
    | sed "s|$ROOT/|/w/|g" > "tests/golden/$out.tsv"
done
# The relations between types, on the same lists and on cases of their own.
python3 tools/gen_relation_cases.py
for name in comptime_projects comptime_cases resolve_targeted types_extra relations_extra; do
  ./target/release/uv-parity comptime-list "tests/golden/$name.list" > "target/parity/$name.oracle.list"
  case "$name" in
    comptime_projects) out=relations ;;
    comptime_cases) out=relations_cases ;;
    resolve_targeted) out=relations_targeted ;;
    types_extra) out=relations_types ;;
    *) out=$name ;;
  esac
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle relations "$ROOT/target/parity/$name.oracle.list" \
    | sed "s|$ROOT/|/w/|g" > "tests/golden/$out.tsv"
done
# Constant encoding: literals of the corpus, the lexical cases and cases of their own;
# then values built from the types of the same project lists.
python3 tools/gen_const_cases.py
for pair in uv_files:consts lex_cases:consts_lex_cases const_cases:const_cases; do
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle consts "/w/tests/golden/${pair%%:*}.list" > "tests/golden/${pair##*:}.tsv"
done
for pair in comptime_projects:values comptime_cases:values_cases resolve_targeted:values_targeted types_extra:values_types relations_extra:values_relations; do
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle values "$ROOT/target/parity/${pair%%:*}.oracle.list" \
    | sed "s|$ROOT/|/w/|g" > "tests/golden/${pair##*:}.tsv"
done
# Pattern typing, on the project lists and on cases of its own.
python3 tools/gen_pattern_cases.py
./target/release/uv-parity comptime-list tests/golden/patterns_extra.list > target/parity/patterns_extra.oracle.list
for pair in comptime_projects:patterns comptime_cases:patterns_cases resolve_targeted:patterns_targeted patterns_extra:patterns_extra; do
  docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
    /w/reference/oracle/uv-oracle patterns "$ROOT/target/parity/${pair%%:*}.oracle.list" \
    | sed "s|$ROOT/|/w/|g" > "tests/golden/${pair##*:}.tsv"
done
# Body typing: every procedure body of every project that resolves.
docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e LD_LIBRARY_PATH="$ICU" uv-oracle \
  /w/reference/oracle/uv-oracle bodies "$ROOT/target/parity/comptime_projects.oracle.list" \
  | sed "s|$ROOT/|/w/|g" > tests/golden/bodies.tsv
run uv-oracle /w/reference/oracle/uv-oracle sigma > tests/golden/sigma.tsv
run uv-oracle sh -c 'g++ -std=c++20 -O1 -o /tmp/probe /w/tools/oracle/unordered_probe.cpp && /tmp/probe' \
  > tests/golden/unordered_order.txt

run ubuntu:24.04 sh /w/tools/oracle/run_reference_cli.sh > tests/golden/cli_cases.out
run ubuntu:24.04 sh /w/tools/oracle/run_reference_text.sh > tests/golden/text_render.out
