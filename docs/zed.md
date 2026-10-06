# Ultraviolet in Zed

## What exists today

| Piece | State |
| --- | --- |
| Zed extension | upstream's, in `ultraviolet-lsp/adapters/zed` |
| Highlighting in that extension | a flat token grammar: keywords, literals and comments are coloured, with no structure. It lacks `region`, `frame`, `spawn`, `parallel`, `dispatch`, `wait`, `transition`, `~>`, octal literals and literal suffixes |
| Language server (`uv-lsp`) | upstream's, in C++. The v0.4.0-alpha release archive does not contain a built one, so it has to be built from source |
| Structural grammar | written, in this repository at `editors/tree-sitter-ultraviolet`, with highlight, indent, fold, injection, locals and tags queries. Not yet packaged as a Zed extension |
| Language server in Rust | `uv-lsp`, in this workspace (`crates/uv-lsp`): `cargo build --release -p uv-lsp` gives `target/release/uv-lsp`. It passes upstream's protocol suite and the Shapes example |

So what can be installed now is upstream's extension. It gives basic colouring by
itself, and diagnostics, hover, completion and the rest only if a `uv-lsp` binary is
available.

The steps below follow upstream's instructions and Zed's rules for dev extensions.
They have not been run on this machine.

## Installing this repository's extension

`editors/zed` is an extension that uses the structural grammar (highlighting, outline,
brackets, indentation) and starts `uv-lsp`. It looks for the server in
`ULTRAVIOLET_LSP_SERVER`, then `uv-lsp` on the `PATH`, then `target/release/uv-lsp` in
the open worktree, and passes `ULTRAVIOLET_TARGET_PROFILE` to it as the target profile.

1. `cargo build --release -p uv-lsp`, and either put `target/release` on the `PATH` or
   set `ULTRAVIOLET_LSP_SERVER` to the binary.
2. Have Rust installed through `rustup` with the `wasm32-wasip1` target (Zed builds the
   extension itself; `cargo build --release --target wasm32-wasip1` in `editors/zed`
   shows it compiles).
3. In Zed, run `zed: install dev extension` and pick `editors/zed`.

The extension needs its own copy of the grammar; `sh tools/sync_zed_grammar.sh` makes it
and `--check` (part of `tools/parity.sh`) fails when it is out of date. The extension
has been compiled and its queries run against a sample file with the tree-sitter CLI;
it has not been opened in Zed on this machine. Fetching the server from a release
waits for M7, when there are releases.

## Installing upstream's extension

1. Install Rust through `rustup` on the machine Zed runs on. Zed compiles a dev
   extension to WebAssembly itself and needs it for that.
2. Have `ultraviolet-lsp/adapters/zed` on that machine's file system.
3. In Zed, open the command palette and run `zed: install dev extension`, then pick
   the `adapters/zed` directory.
4. Open a `.uv` file. The language shows as Ultraviolet in the status bar.

If Zed runs on Windows and the checkout is inside WSL, clone `ultraviolet-lsp` on the
Windows side as well and install from there.

If the extension fails to build, `zed: open log` shows why. The grammar is declared as
`file://./tree-sitter-ultraviolet` at revision `main`, which is the first thing to
check there.

## The language server

The extension starts `uv-lsp --stdio`. It looks for the server in this order:

1. the path in the `ULTRAVIOLET_LSP_SERVER` environment variable;
2. `uv-lsp` on the `PATH` of the Zed worktree;
3. `uv-lsp.exe` on that `PATH`.

`ULTRAVIOLET_TARGET_PROFILE`, when set, is passed to the server as the target profile.

Colouring comes from the grammar and does not depend on the server.

To get a server today, build upstream's with CMake. It links against the compiler's
libraries, so the compiler has to be built first, with the two checkouts side by side:

```
cmake -S . -B build -DULTRAVIOLET_REPO=../ultraviolet -DULTRAVIOLET_BUILD_DIR=../ultraviolet/Bootstrap/Ultraviolet/build/<platform>
cmake --build build --config Release --target uv-lsp
cmake --install build --config Release --prefix install
```

Run from the `ultraviolet-lsp` directory; the server lands in `install/bin`. Upstream
documents this for Windows, and the compiler build pulls LLVM 21 and ICU 72.

## What changes with the port

- M4 is done: the Rust `uv-lsp` replaces the C++ server and is checked against
  upstream's protocol test suite (`python3 ultraviolet-lsp/tests/lsp_protocol_driver.py
  target/release/uv-lsp`) and the Shapes example (`ultraviolet-lsp/examples/shapes/validate_lsp.py`).
  To use it in Zed today, build it with the command above and point
  `ULTRAVIOLET_LSP_SERVER` at it.
- M5 ships a Zed extension from this repository that uses the structural grammar,
  adds outline and bracket queries, and downloads the server from the releases
  instead of needing it on `PATH`.

Until then the structural grammar can be exercised by itself with the tree-sitter CLI,
which runs the cases under `test/corpus`:

```bash
cd editors/tree-sitter-ultraviolet && npx tree-sitter-cli test
```
