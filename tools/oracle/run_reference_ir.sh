#!/bin/sh
# Runs inside the container: run_reference_ir.sh <golden-subdir>.
# For every project the reference accepted (exit 0 in results.tsv), records the textual
# IR of `uvc build --emit-ir`.
set -u
U=/w/reference/ultraviolet/uvc
OUT=/w/tests/golden/$1
n=0
while IFS="$(printf '\t')" read -r id rc manifest; do
  [ "$rc" = "0" ] || continue
  dir=$(dirname "/w/$manifest")
  (cd "$dir" && timeout 300 "$U" build Ultraviolet.toml --emit-ir --build-progress off --target-profile x86_64-sysv --no-crash-report > "$OUT/$id.ir" 2>"$OUT/$id.ir.stderr"; echo $? > "$OUT/$id.ir.rc")
  rm -rf "$dir/Build"
  n=$((n + 1))
done < "$OUT/results.tsv"
echo "done $n"
