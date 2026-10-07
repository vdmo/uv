#!/bin/sh
# ir_of.sh <project directory under the workspace>: the IR of the reference and of the
# Rust compiler for one project, side by side in /tmp/ir_ref.txt and /tmp/ir_rust.txt.
set -eu
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
DIR=$(cd "$1" && pwd)
REL=${DIR#"$ROOT"/}
docker run --rm -v "$ROOT":/w -w "/w/$REL" uv-oracle /w/reference/ultraviolet/uvc build Ultraviolet.toml --emit-ir --build-progress off --target-profile x86_64-sysv --no-crash-report > /tmp/ir_ref.txt 2>/tmp/ir_ref.err || true
rm -rf "$DIR/Build"
(cd "$DIR" && "$ROOT/target/release/uvc" build Ultraviolet.toml --emit-ir --build-progress off --target-profile x86_64-sysv --no-crash-report > /tmp/ir_rust.txt 2>/tmp/ir_rust.err) || true
diff /tmp/ir_ref.txt /tmp/ir_rust.txt && echo identical
