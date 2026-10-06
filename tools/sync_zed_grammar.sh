#!/bin/sh
# Copies the grammar into the Zed extension, which needs it inside its own directory.
# `sh tools/sync_zed_grammar.sh --check` fails when the copy is out of date.
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
SRC=$ROOT/editors/tree-sitter-ultraviolet
DST=$ROOT/editors/zed/tree-sitter-ultraviolet
if [ "${1:-}" = "--check" ]; then
  for item in grammar.js package.json tree-sitter.json src; do
    diff -r "$SRC/$item" "$DST/$item" > /dev/null || { echo "editors/zed/tree-sitter-ultraviolet/$item is out of date; run tools/sync_zed_grammar.sh"; exit 1; }
  done
  cmp "$SRC/queries/highlights.scm" "$ROOT/editors/zed/languages/ultraviolet/highlights.scm" || { echo "Zed highlights.scm is out of date"; exit 1; }
  exit 0
fi
rm -rf "$DST"
mkdir -p "$DST"
for item in grammar.js package.json tree-sitter.json src; do cp -r "$SRC/$item" "$DST/$item"; done
cp "$SRC/queries/highlights.scm" "$ROOT/editors/zed/languages/ultraviolet/highlights.scm"
