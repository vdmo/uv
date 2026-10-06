# Building and running

## What you need

- Rust (stable) with Cargo.
- Python 3, for the gates.
- Docker, for the reference compiler and for regenerating goldens. Not needed to build
  the port or to run the gates.

Three directories are not in this repository and sit at its root. The gates read them;
building the port does not need them.

| Directory | What | From |
| --- | --- | --- |
| `ultraviolet/` | compiler sources and the `HelloUltraviolet` corpus, at tag `v0.4.0-alpha` | `git clone https://github.com/blacklight-foundation/ultraviolet` |
| `ultraviolet-lsp/` | language server sources, editor adapters, protocol tests | `git clone https://github.com/blacklight-foundation/ultraviolet-lsp` |
| `reference/ultraviolet/` | the v0.4.0-alpha release archive, unpacked | the release page of `blacklight-foundation/ultraviolet` |

## Build

```bash
cargo build --release
```

The binaries are `target/release/uvc` and `target/release/uv-parity`.

## The Rust compiler

`uvc` takes the same command line as the reference. Today it implements the first
phase only: loading the project, lexing and parsing.

```bash
./target/release/uvc --version
```

```bash
./target/release/uvc build path/to/project --phase1-only --diag-json
```

```bash
./target/release/uvc build path/to/project --dump
```

```bash
./target/release/uvc build path/to/project --dump-ast --phase1-only
```

`path/to/project` is a directory with an `Ultraviolet.toml`, or a file inside one.
`ultraviolet/HelloUltraviolet` and `ultraviolet-lsp/examples/shapes` are two to try.

`--check` also runs the compile-time pass, name resolution, type checking and the
capability and authority checks, and prints their diagnostics. A program that passes
them all reaches the lowering to IR, which is not ported, so the run (and a plain
`build`) prints the diagnostics so far, then this, and exits with status 3:

```
error: the lowerability check is not implemented in this build of uvc (Rust port in progress)
```

Exit statuses otherwise follow the reference: 0 success, 1 errors reported, 2 bad
command line. `uvc --help` lists every option; the options for later phases are parsed
and not acted on.

## The reference compiler

To check or build Ultraviolet code today, use the release binary. It needs glibc 2.38,
so on an older system run it in a container:

```bash
docker run --rm -v "$PWD":/w ubuntu:24.04 /w/reference/ultraviolet/uvc build /w/ultraviolet-lsp/examples/shapes --check
```

Drop `--check` to build; output goes to the project's `Build/` directory. On a system
with glibc 2.38 or newer, `reference/ultraviolet/uvc` runs directly.

## The gates

Every ported part is compared with the reference:

```bash
sh tools/parity.sh
```

It builds the workspace, runs each gate and ends with `ALL PARITY GATES PASS` or
`PARITY FAILURES`. It takes ten minutes or more. It needs the three directories above
and the goldens in `tests/golden`, and does not need Docker.

The body-typing gate alone, which is the one that moves while typing is ported:

```bash
./target/release/uv-parity bodies target/parity/comptime.list | sed "s|$PWD/|/w/|g" > target/parity/bodies.tsv
```

```bash
python3 tools/compare_typing.py tests/golden/bodies.tsv target/parity/bodies.tsv --waiting
```

`target/parity/comptime.list` is written by `tools/parity.sh`, so run that once first.

Lints:

```bash
cargo clippy --workspace --all-targets
```

## Regenerating goldens

Goldens come from two things run in Docker: the release binary, and an oracle built
from the reference's own sources (`tools/oracle`) that dumps what the release binary
cannot, such as syntax trees, resolved modules and typed bodies.

```bash
sh tools/capture_goldens.sh
```

This takes a long while, because some inputs make the reference hang until a time
limit. It also needs the external payload the upstream build fetches
(`ultraviolet/Tools/FetchTargetExterns.py`), for ICU.

To rebuild the oracle after changing `tools/oracle/oracle_main.cpp`:

```bash
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD":/w -e LD_LIBRARY_PATH=/w/ultraviolet/Bootstrap/extern/icu/linux/lib uv-oracle sh /w/tools/oracle/build.sh
```

`tools/store_diff.sh <part of a manifest path>` shows, for the projects that match,
which entries of the typing stores differ between the reference and the port.

The syntax tree types, their dump and their walker are generated from the reference
headers for both sides:

```bash
python3 tools/gen_ast.py
```
