# Ultraviolet in Rust: compiler, LSP and editor support

Baseline: `ultraviolet` at `2880fdd` (v0.4.0-alpha), `ultraviolet-lsp` at `1b8a38a`.
Both are cloned next to this file and are treated as read-only references.

## What is being ported

| Part | Today | Size | Notes |
| --- | --- | --- | --- |
| `00_core` | C++20 | 14k lines | spans, diagnostics (1,280-line generated registry), Unicode via ICU 72, host services |
| `01_project` | C++20 | 9k | `Ultraviolet.toml` (toml++), module discovery, target profiles, linking |
| `02_source` | C++20 | 35k | lexer 4k, parser 28k, AST, attributes |
| `03_comptime` | C++20 | 10k | eval, quote, reflect, hygiene, derive |
| `04_analysis` | C++20 | 128k | typing 67k, resolve 12k, memory 12k, caps 8k, contracts 7k, layout 5k, keys 3k |
| `05_codegen` | C++20 | 103k | own IR, lowering 36k, LLVM 21.1.8 emission 47k |
| `06_driver` | C++20 | 14k | CLI, pipeline, incremental, tooling API (1k) |
| Runtime | C | 27k | linked into compiled programs; ABI fixed by spec Appendix D |
| `uv-lsp` | C++20 | 7k | thin server over `uv_tooling`/`uv_analysis`; JSON via `llvm/Support/JSON.h` |
| Adapters | TS, Lua, Rust, Python, Kotlin | 2k | VS Code, Neovim, Zed, Sublime Text, JetBrains |

Total compiler: about 313k lines of C++. Analysis and codegen are 74% of it.

Test assets that carry over unchanged:

- `HelloUltraviolet`: 1,583 `.uv` files, including 1,303 rejected-source fixtures, 84 diagnostic fixtures, 82 output-diagnostic fixtures, 98 artifact projects, 27 accepted projects.
- `ultraviolet-lsp/tests/lsp_protocol`: 12 Python files (3k lines) that drive the server over stdio. They are language-agnostic, so they run against a Rust binary as they are.
- `tests/validate_adapters.py` and `examples/shapes/validate_lsp.py`.

## Approach

A faithful port, phase by phase, gated on parity with the v0.4.0-alpha reference
compiler. Same phase boundaries, same diagnostic codes, same CLI, same
`--diag-json` and `--dump-ast` output. No redesign of semantics during the port.

The LSP links only `core`, `project`, `source`, `comptime`, `analysis` and
`tooling`, not codegen. So the front end is ported first, which gives a working
Rust LSP and all editors before the 103k-line backend is touched.

## Workspace

```
ultraviolet-lang/
  Cargo.toml                  workspace
  crates/
    uv-core                   00_core
    uv-project                01_project
    uv-source                 02_source
    uv-comptime               03_comptime
    uv-analysis               04_analysis
    uv-codegen                05_codegen
    uv-tooling                06_driver/tooling + language_service facts
    uvc                       06_driver, the compiler binary
    uv-lsp                    the language server binary
  runtime/                    C runtime, built with the `cc` crate
  editors/
    tree-sitter-ultraviolet   real grammar from spec Appendix B
    vscode  neovim  zed  sublime-text  jetbrains
  tests/                      parity harness, LSP protocol suite, corpus
```

One workspace replaces the current arrangement where the LSP's CMake imports
static libraries out of a sibling compiler build directory.

## Dependency replacements

| C++ | Rust | Risk |
| --- | --- | --- |
| LLVM 21.1.8 C++ API | `inkwell` / `llvm-sys` pinned to LLVM 21.1 | C API covers IRBuilder use; pass pipeline goes through `LLVMRunPasses` |
| ICU 72 (NFC, XID, scripts, confusable skeletons) | `unicode-normalization`, `unicode-ident`, `unicode-script`, `unicode-security` | Unicode data version must match ICU 72 (Unicode 15) or identifier diagnostics drift; verify on the lexical fixtures first |
| toml++ | `toml` | low |
| `llvm/Support/JSON.h` | `serde_json` | low |
| hand-rolled JSON-RPC | `lsp-server` + `lsp-types` | low |

`lsp-server` (synchronous, explicit message loop) is preferred over `tower-lsp`
because the protocol tests assert exact lifecycle behaviour: pre-initialize
rejection, duplicate initialize, parse-error replies, post-shutdown errors.

Dropping ICU removes the shared-library sidecars, so `uvc` and `uv-lsp` become
single static binaries and the `LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` code in
every adapter goes away.

## Milestones

Each gate is a command that passes or fails, compared against the reference.

| # | Work | C++ ported | Gate |
| --- | --- | --- | --- |
| M0 | Workspace, parity harness, golden capture from the reference `uvc` | 0 | goldens recorded for the whole corpus |
| M1 | `uv-core`, `uv-project`, lexer | 27k | project loading and lexical diagnostics match on fixtures |
| M2 | AST, parser, attributes | 31k | full AST dump identical on all 1,585 files (oracle tool; the reference `--dump-ast` only lists items) |
| M3 | Resolve, typing, memory, caps, keys, contracts, layout, comptime | 138k | `--check --diag-json` identical on rejected, diagnostic and accepted fixtures |
| M4 | `uv-tooling`, `uv-lsp` | 9k | the existing Python protocol suite passes unchanged; `validate_lsp.py` passes |
| M5 | Editors connected to the Rust server | 2k | `validate_adapters.py` passes; manual check in Zed, VS Code, Neovim |
| M6 | IR, lowering, LLVM emission, driver, linking | 116k | `Tools/RunHelloVerification.py` passes with the Rust `uvc` |
| M7 | Release packaging, installers, CI for three targets | 0 | installed archive builds and runs HelloUltraviolet |

Editor grammar track (independent of M1 to M4, can start at once):

- Replace the Zed tree-sitter grammar. The current one is a flat token list with
  no structure, and its keyword set lacks `region`, `frame`, `spawn`, `parallel`,
  `dispatch`, `wait`, `transition`, `~>`, octal literals and literal suffixes.
  Write a structural grammar from Appendix B with `highlights`, `indents`,
  `folds`, `outline`, `brackets` and `injections` queries.
- Regenerate the TextMate grammar (VS Code, JetBrains), the Vim syntax file and
  the Sublime syntax from the same keyword and literal tables as the Rust lexer,
  so they cannot drift again.
- Neovim: ship the tree-sitter parser and queries as well as the regex syntax.

## Editors and "in Rust"

Only Zed hosts extensions in Rust (compiled to Wasm). VS Code requires
JavaScript/TypeScript, Neovim Lua, Sublime Text Python and JetBrains a JVM
language. Those four adapters stay thin clients in their host language and all
logic moves into the Rust server. The Zed extension stays Rust and gains
automatic download of the server from GitHub releases.

## Defaults chosen (change if you disagree)

1. The C runtime is kept as C. It is not part of the compiler or the LSP, its ABI
   is fixed by the spec, and porting it adds 27k lines with no user-visible gain.
2. Faithful port, parity-gated, pinned to v0.4.0-alpha. Upstream has cut seven
   alpha releases since the first; later spec changes are merged after parity.
3. Code lives in a new workspace at this directory's root.

## Status

M0, M1 and M2 are done. Run `tools/parity.sh` to re-check every gate (about a minute);
`tools/capture_goldens.sh` regenerates the goldens (needs Docker, and takes a while because
of the inputs the reference does not terminate on).

How the reference is used:

- `reference/ultraviolet`: the v0.4.0-alpha release archive, run in an `ubuntu:24.04`
  container (it needs glibc 2.38).
- `reference/oracle/uv-oracle`: a small tool compiled from the upstream C++ core, lexer and
  parser sources against ICU 72 (`tools/oracle`). It dumps per-scalar Unicode properties,
  tokens, and complete syntax trees with diagnostics, none of which the release binary can
  do. Each input runs in a child process with a 3 second limit; `CRASH` and `HANG` are
  recorded and such inputs are excluded from comparison.

Gates, all passing:

| Gate | Result |
| --- | --- |
| Unicode properties for every scalar value (XID, NFC, case fold, confusable skeleton, script mixing) vs ICU 72 | identical, 1,105,677 lines |
| Tokens, doc comments and lexical diagnostics, corpus | identical, 1,585 files |
| Same, 4,908 generated lexical stress cases | identical on 4,906; the reference crashes on the other 2 |
| Complete syntax tree (every node, field and span), doc attachment, `unsafe` spans and parse diagnostics, corpus | identical, 1,585 files (42 MB of dump, byte for byte) |
| Same, the 4,908 lexical stress cases | identical on 4,446; the reference crashes or hangs on the other 462 |
| Same, 6,000 corpus files with token-level edits (parser error recovery) | identical on 5,977; the reference hangs on the other 23 |
| `--phase1-only`: JSON diagnostics, exit status and `--dump-ast` listing | identical, 549 fixtures, 58 manifest cases, 78 module-level cases |
| `--check`: JSON diagnostics and exit status for projects rejected in phase 1 | identical, 32 fixtures and 41 manifest cases |
| `--check`: phase-1 diagnostics for every other project | identical, 534 |
| `--dump` (project model, output paths, file order) | identical, 496 of 496 the reference prints |
| Human-readable output for phase 1 | identical, 151 projects |
| Command line: help, version, usage errors | identical, 52 command lines |

What M2 added:

- `uv-source::ast`: the node types, generated from the upstream headers by
  `tools/gen_ast.py` together with the dump code used by both sides of the gate.
- `uv-source::parser`: the recursive-descent parser (items, statements, expressions,
  patterns, types, attributes, generics, contracts, key blocks, quote/splice, doc
  attachment), with the reference's error recovery and diagnostics.
- `uv-source::parse_modules`: module aggregation, the reserved-binder check (`E-CNF-0401`)
  and `self` outside a type (`E-SEM-3011`).
- `uv-source::attributes`: the attribute registry and `validate_attributes`.
- `uv-analysis`: created, holding only the FFI surface collection the driver logs.
- `uvc`: phase 1 end to end: import-driven assembly order, `#derive` attribute-list
  validation, the error cap, `--phase1-only` and `--dump-ast`. Any other run that passes
  phase 1 prints its diagnostics, says later phases are not implemented and exits with 3.

Upstream bugs found (the Rust port deliberately differs on these inputs):

- Lexer: malformed UTF-8 that encodes a surrogate (`ED A0 80`) or a value above U+10FFFF
  (`F4 90 80 80`) aborts the reference with an uncaught exception. The Rust port reports
  `E-SRC-0101`, which is what the specification asks for.
- Parser: a block, record, enum, modal, class or extern body left open at end of file makes
  the reference loop forever (for example `let a = unsafe {`). The Rust port stops at end
  of file and reports the missing brace with `E-SRC-0520`.
- Parser: `ForeignContractClause.kind` is read uninitialised when a foreign contract clause
  is malformed. The Rust port uses `Assumes`; the comparison ignores that one field.

Known differences that remain until later milestones:

- `--dump` prints for every project that passes phase 1; the reference prints only after
  type checking succeeds, so 38 fixtures it rejects later still print here.
- Modules of an assembly are parsed sequentially; the reference uses worker threads and
  merges in the same order, so results are the same and only wall time differs.
- `#test(covers(...))` looks for the obligation ledger under the working directory and its
  ancestors only (the reference also checks its support bundle and a build-time path).
- `--profile-compiler` events for parsing are not emitted.

Not ported yet, although it lives in the source directories of finished milestones:

- `00_core`: static rule registry (`behavior_model.cpp`), host primitive classification,
  crash reports, compiler sidecar support, runtime ABI table.
- `01_project`: linking, tool resolution, target platform and IR assembly (needed in M6).
- `02_source`: AST utilities that only later phases call (visitors, cloning helpers,
  `ast_dump` beyond the item summary) are ported when their first caller is.
- Driver: `test`, `init`, `clean`, `--conformance`, `--profile-compiler`.
