#!/bin/sh
# Runs inside the container: run_reference_projects.sh <golden-subdir> <list-file>.
# For every manifest in the list, records the reference compiler's exit code, JSON
# diagnostics and project dump.
set -u
U=/w/reference/ultraviolet/uvc
OUT=/w/tests/golden/$1
mkdir -p "$OUT"
: > "$OUT/results.tsv"
n=0
while read -r manifest; do
  n=$((n + 1))
  dir=$(dirname "/w/$manifest")
  id=$(printf '%05d' "$n")
  json=$(cd "$dir" && timeout 120 "$U" build Ultraviolet.toml --check --diag-json --build-progress off --target-profile x86_64-sysv --no-crash-report 2>"$OUT/$id.stderr")
  rc=$?
  printf '%s\n' "$json" > "$OUT/$id.json"
  (cd "$dir" && timeout 120 "$U" build Ultraviolet.toml --check --dump --build-progress off --target-profile x86_64-sysv --no-crash-report 2>/dev/null | grep -E '^(<|file:)' > "$OUT/$id.dump")
  printf '%s\t%s\t%s\n' "$id" "$rc" "$manifest" >> "$OUT/results.tsv"
  rm -rf "$dir/Build"
done < "/w/tests/golden/$2"
echo "done $n"
