#!/bin/sh
# Shows the entries of the typing stores that differ between the reference and the port
# for the projects whose manifest path contains $1. Needs Docker and a built oracle.
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"
pick() { awk -v want="$1" 'BEGIN{keep=0} /^P\t/{keep=index($0,want)>0} keep{print}' "$2"; }
pick "$1" target/parity/comptime_projects.oracle.list > target/parity/store_one.oracle.list
pick "$1" target/parity/comptime.list > target/parity/store_one.list
docker run --rm -u "$(id -u):$(id -g)" -v "$ROOT":/w -v "$ROOT":"$ROOT" -e UV_STORE_VERBOSE=1 \
  -e LD_LIBRARY_PATH=/w/ultraviolet/Bootstrap/extern/icu/linux/lib uv-oracle \
  /w/reference/oracle/uv-oracle bodies "$ROOT/target/parity/store_one.oracle.list" 2>/dev/null \
  | sed "s|$ROOT/|/w/|g" > target/parity/store_one.ref.tsv
UV_STORE_VERBOSE=1 ./target/release/uv-parity bodies target/parity/store_one.list 2>/dev/null \
  | sed "s|$ROOT/|/w/|g" > target/parity/store_one.rust.tsv
diff target/parity/store_one.ref.tsv target/parity/store_one.rust.tsv || true
