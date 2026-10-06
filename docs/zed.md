# Ultraviolet in Zed

## What exists today

| Piece | State |
| --- | --- |
| Zed extension | upstream's, in `ultraviolet-lsp/adapters/zed` |
| Highlighting in that extension | a flat token grammar: keywords, literals and comments are coloured, with no structure. It lacks `region`, `frame`, `spawn`, `parallel`, `dispatch`, `wait`, `transition`, `~>`, octal literals and literal suffixes |
| Language server (`uv-lsp`) | upstream's, in C++. The v0.4.0-alpha release archive does not contain a built one, so it has to be built from source |
| Structural grammar | written, in this repository at `editors/tree-sitter-ultraviolet`, with highlight, indent, fold, injection, locals and tags queries. Not yet packaged as a Zed extension |
| Language server in Rust | not started (milestone M4) |

So what can be installed now is upstream's extension. It gives basic colouring by
itself, and diagnostics, hover, completion and the rest only if a `uv-lsp` binary is
available.

The steps below follow upstream's instructions and Zed's rules for dev extensions.
They have not been run on this machine.

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

- M4 replaces the C++ server with a Rust `uv-lsp` built from this workspace by
  `cargo build`, checked against upstream's protocol test suite.
- M5 ships a Zed extension from this repository that uses the structural grammar,
  adds outline and bracket queries, and downloads the server from the releases
  instead of needing it on `PATH`.

Until then the structural grammar can be exercised by itself with the tree-sitter CLI,
which runs the cases under `test/corpus`:

```bash
cd editors/tree-sitter-ultraviolet && npx tree-sitter-cli test
```
