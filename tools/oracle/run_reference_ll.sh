#!/bin/sh
# Runs inside the container (image uv-oracle-link): run_reference_ll.sh <golden-subdir>.
# For every project the reference accepted (exit 0 in results.tsv), builds it with
# `emit_ir = "ll"` set in the manifest and records the LLVM IR it wrote, one file per module,
# joined under `; ==== <path>` headers, into <id>.ll. The build's exit status goes to <id>.ll.rc.
set -u
U=/w/reference/ultraviolet/uvc
RT=/w/reference/ultraviolet/UltravioletRT.a
OUT=/w/tests/golden/$1
WORK=/tmp/llwork
n=0
while IFS="$(printf '\t')" read -r id rc manifest; do
  [ "$rc" = "0" ] || continue
  src=$(dirname "/w/$manifest")
  rm -rf "$WORK" && mkdir -p "$WORK" && cp -r "$src/." "$WORK/"
  rm -rf "$WORK/Build"
  # Libraries emit the IR by default; executables are asked to.
  if ! grep -q '^emit_ir' "$WORK/Ultraviolet.toml"; then
    sed -i '0,/^root = /s//emit_ir = "ll"\nroot = /' "$WORK/Ultraviolet.toml"
  fi
  (cd "$WORK" && timeout 600 "$U" build Ultraviolet.toml --target-profile x86_64-sysv --build-progress off --no-crash-report --runtime-lib "$RT" >"$OUT/$id.ll.stdout" 2>"$OUT/$id.ll.stderr"; echo $? > "$OUT/$id.ll.rc")
  : > "$OUT/$id.ll"
  find "$WORK/Build/Intermediate/IR" -name '*.ll' 2>/dev/null | sort | while read -r f; do
    echo "; ==== ${f#$WORK/Build/Intermediate/IR/}" >> "$OUT/$id.ll"
    cat "$f" >> "$OUT/$id.ll"
  done
  n=$((n + 1))
done < "$OUT/results.tsv"
echo "done $n"
