#!/bin/sh
# Runs inside the container: run_reference_phase1.sh <golden-subdir> <list-file>.
# For every manifest in the list, records the reference compiler's phase-1 (parse) result:
# exit code, JSON diagnostics and the `--dump-ast` listing.
set -u
U=/w/reference/ultraviolet/uvc
OUT=/w/tests/golden/$1
mkdir -p "$OUT"
: > "$OUT/phase1.tsv"
n=0
while read -r manifest; do
  n=$((n + 1))
  dir=$(dirname "/w/$manifest")
  id=$(printf '%05d' "$n")
  json=$(cd "$dir" && timeout 120 "$U" build Ultraviolet.toml --phase1-only --diag-json --build-progress off --target-profile x86_64-sysv --no-crash-report 2>/dev/null)
  rc=$?
  printf '%s\n' "$json" > "$OUT/$id.phase1.json"
  (cd "$dir" && timeout 120 "$U" build Ultraviolet.toml --phase1-only --dump-ast --build-progress off --target-profile x86_64-sysv --no-crash-report 2>/dev/null > "$OUT/$id.ast")
  printf '%s\t%s\t%s\n' "$id" "$rc" "$manifest" >> "$OUT/phase1.tsv"
  rm -rf "$dir/Build"
done < "/w/tests/golden/$2"
echo "done $n"
