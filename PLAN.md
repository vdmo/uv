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

## Direction beyond parity

`learn-review.md` (kept locally, not in the repository) sketches where the compiler should
go once it is in Rust. It was written without the language specification, so it is read as
a list of intentions and checked against what Ultraviolet actually defines.

Adopted, with where each lands:

| Idea | Decision |
| --- | --- |
| Explicit pure phases: tokens, AST, typed module, IR, backend | Already the shape of the port; kept |
| Golden tests comparing diagnostics and IR against the old compiler | Already the method (`tools/parity.sh`); kept through M6 |
| Single diagnostic type with human and JSON renderers | Done in M1 |
| Invariants enforced by Rust types (private fields, transitions only through one API) | Applied when M3 ports permissions, binding state and provenance: one module owns each state machine, no pass mutates it directly |
| IR verifier run before any backend | New work in M6: a pass over the ported IR that checks the lowering did not introduce moves, allocations or key acquisitions the typed program does not show |
| Backend trait so LLVM is one implementation | M6: introduce the trait while porting LLVM emission; Cranelift is a later backend, not part of parity |
| Parallel parsing and per-function work | After M3 parity; the reference already parses modules on threads |
| Incremental query engine shared by compiler and LSP | After M4 parity. The reference LSP re-analyses the workspace on change; a query database replaces that once behaviour is locked by the protocol suite |
| Semantic tokens, hover and signature help that show permissions, regions, keys and contracts | M4 ports what the reference server has; the richer payloads are extensions after it |
| Fuzzing | Started: the 10,908 generated lexer and parser cases already found three reference bugs; `cargo-fuzz` targets follow |
| Formatter, linter, REPL, flow visualisation | After M7 |

Not adopted, because the language is different from what the note assumes:

- **Ownership model.** The note uses `Unique / Shared / Borrowed / Phantom` with borrows
  and lifetimes. Ultraviolet has no borrows: it has the permissions `const`, `unique` and
  `shared`, binding state (moved, partially moved), and the key system (`%read`,
  `%write`) for access to `shared` data. The Rust types follow the specification, not
  Rust's own borrow checker.
- **Regions.** The note's `Stack / Heap / Async / Foreign` kinds are not the language's.
  Regions are lexical arenas (`region`, `frame`, `^` allocation) with provenance tracked
  on pointers (`Ptr<T>@Valid`, `@Null`, `@Expired`).
- **Effect system.** There is no `EffectSet` or notion of a pure procedure. The language
  controls effects through capabilities passed as values (no ambient authority) and
  through contracts. "Effect" highlights and completion filters are built on
  capabilities and contracts, or not at all.
- **Syntax in the examples** (`fn`, `Int`, `alloc T`) is not Ultraviolet.
- **Wrapping the old compiler in a Rust shell first** (the note's phase 1) is skipped: the
  reference is used as an oracle from outside instead.
- **Parser combinator crates and a new `SourceMap`.** The hand-written parser and the
  existing `SourceFile` are byte-for-byte equal to the reference; they stay.

Order of work is unchanged: parity first (M3 to M7), then the items above. An improvement
that changes observable behaviour before its milestone's gate passes would make the gate
meaningless.

## Status

M0, M1 and M2 are done, and M3 has started (its first two slices, the compile-time pass
and name resolution, are done). Run `tools/parity.sh` to re-check every gate (about a
minute);
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
| Compile-time pass: expanded modules (full syntax trees) and diagnostics, every project that passes phase 1, including the 62-module HelloUltraviolet project | identical, 546 projects (42 MB of dump, byte for byte) |
| Same, 144 targeted cases (evaluator, quote and splice, hygiene, emission, reflection, project files, derive) | identical on 143; the reference crashes on the other 1 |
| Name resolution: per-module name maps, resolved modules (full syntax trees) and diagnostics with notes and suggestions, every project that passes phase 1 | identical, 546 projects (43 MB of dump, byte for byte) |
| Same, the 144 compile-time cases | identical on 143; the reference crashes on the other 1 |
| Same, 410 targeted cases (unresolved names and suggestions, scopes, qualified forms, patterns, `using`, `import`, other assemblies, reserved names) | identical, 410 |
| Built-in declaration table (the records, enums, modals, aliases and classes analysis registers) | identical, 52 declarations |
| Iteration order of the reference's hash table (hash values, growth, element order after inserts, copies and assignments) | identical to a probe of its standard library |
| Type core and layout: every type written in a declaration, lowered and printed, with its ordering key, the paths it names and what they resolve to, equivalence against the module's other types, field types, instantiations and variance, and its layout, size and alignment; every record, enum and modal declaration's layout (offsets, discriminants, payloads, niches); every project that resolves | identical on 545 of 546; the reference crashes on the other 1 |
| Same, the compile-time and name-resolution cases | identical on 121 of 144 (the reference crashes on 23) and 410 of 410 |
| Same, 54 targeted cases (union order, array lengths that need evaluation, refinements, generic defaults, asynchronous aliases, lookup across modules and assemblies; packing and alignment attributes, explicit and invalid discriminants, niches, recursive and generic types) | identical on 51; the reference crashes on the other 3 |
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

M3 is split into slices, each gated on its own against the oracle before the next starts:

| Slice | Reference source | Lines | State |
| --- | --- | --- | --- |
| M3.1 compile-time pass (`uv-comptime`) | `03_comptime` | 10.6k | done |
| M3.2 name resolution (`uv-analysis::resolve`) | `04_analysis/resolve`, built-in declarations from `caps`, `memory`, `typing` | 13k | done |
| M3.3 typing context, generics, modal, composite, layout | `typing` (core), `generics`, `modal`, `composite`, `layout` | about 45k | in progress, see below |
| M3.4 expression, statement and declaration typing | `typing` (rest) | about 40k | |
| M3.5 memory, provenance, capabilities, keys, contracts | `memory`, `provenance`, `caps`, `keys`, `contracts` | 36k | |
| M3.6 driver phases 2 and 3 end to end | `06_driver` (sema section), `conformance` | | gate: `--check --diag-json` identical on every fixture |

What M3.1 added:

- `uv-comptime`: the evaluator, quote parsing and splice substitution, hygiene, emission,
  reflection (`introspect`), the project-files snapshot (`files`), derive targets and
  their ordering, and the per-module expansion driver.
- The oracle gained a `comptime` mode that runs the reference pass on a project and dumps
  the expanded modules; `uv-parity comptime-list` produces the project descriptions from
  the Rust phase 1.
- Phase-1 orchestration moved from the driver into `uv-source::phase1` so tools can run it.
- `uvc` does not run phase 2 yet: the reference validates compile-time procedure
  signatures with the type checker first, which arrives in M3.3.

What M3.2 added:

- `uv-analysis::resolve`: scopes and lookup, visibility checks, top-level name collection
  to a fixed point across modules, `using` and `import`, and the resolver proper for
  types, patterns, expressions, statements, items and modules, with the reference's
  diagnostics (including which parts of a failure each call site keeps).
- `uv-analysis::caps::builtin_decls` and `resolve::populate_sigma`: the declarations of
  the built-in types and classes.
- `uv-core::std_unordered`: a map that iterates in the order of the reference's
  `std::unordered_map` (see below).
- The oracle gained `resolve` and `sigma` modes, and `tools/oracle/unordered_probe.cpp`;
  `tools/gen_resolve_cases.py` writes the targeted cases.
- `uvc` still stops after phase 1, for the reason given under M3.1.

M3.3 is itself in parts:

| Part | Reference source | State |
| --- | --- | --- |
| a. Type core: semantic types, lowering, array lengths, equivalence, ordering, lookup, substitution, variance | `typing/type_refs`, `type_lower`, `const_len`, `type_equiv`, `type_lookup`, `variance`; `generics/monomorphize` (substitution); `contracts/verification` (structural equality) | done |
| b. Layout: sizes, alignments, field offsets, discriminants, niches | `layout` (all but constant encoding), `composite/enums`, `modal/modal_widen` (payload state) | done |
| c. Subtyping, well-formedness, predicates, modal, composite types, the rest of generics | `typing/subtyping`, `type_wf`, `type_predicates`, `modal`, `composite`, `generics` | next |
| d. Constant encoding (the bytes of a value of each type) | `layout/layout_value_bits` | |

What M3.3a added:

- `uv-analysis::typing`: `types` (representation, constructors, display, ordering key,
  canonical unions, asynchronous signatures), `type_lower`, `const_len`, `type_equiv`,
  `type_lookup`, `variance`.
- `uv-analysis::generics::monomorphize`: instantiation and building substitutions with
  defaults.
- `uv-analysis::contracts::struct_equal`: equality of syntax up to source locations,
  which refinement types are compared with.
- The oracle gained a `types` mode; `tools/gen_type_cases.py` writes the targeted cases.

What M3.3b added:

- `uv-analysis::layout`: `layout_of`, `size_of` and `align_of` for every type; record,
  tuple, range, enum (with payload member offsets), union and modal layouts, including
  the niche representations; the `#layout` and `#align` attributes; the lowering of
  asynchronous types to their state machine.
- `uv-analysis::composite::enums`: enum discriminants and their diagnostics.
- `uv-analysis::modal::modal_widen`: which state of a modal carries the payload.
- Type lowering now has the three flavours the reference has (ordinary, for layout, for
  modal representation), as one function with a flavour argument.
- The oracle's `types` mode prints layouts too.

Two pieces of reference behaviour are reproduced on purpose.

The first is hash-table order. The reference iterates `std::unordered_map` where the order
shows: a spelling suggestion takes the first of several equally close names, and a
wildcard `using` binds names in the order of the map it reads. That order comes from the
standard library the reference is built with (libstdc++ here), so scopes use a map that
reproduces its hash function, bucket growth and element linking, tested against a probe of
the real library. A reference built with another standard library would order ties
differently; the port matches the Linux build.

The second: hygiene renames quoted syntax in
place through shared nodes, so a quoted value that is emitted twice or spliced into two
quotes is renamed again on each insertion and earlier insertions change with it. Matching
that needs one documented `unsafe` function (`shared_node_mut` in `hygiene.rs`); every
other crate is free of `unsafe`.

Upstream bugs found (the Rust port deliberately differs on these inputs):

- Lexer: malformed UTF-8 that encodes a surrogate (`ED A0 80`) or a value above U+10FFFF
  (`F4 90 80 80`) aborts the reference with an uncaught exception. The Rust port reports
  `E-SRC-0101`, which is what the specification asks for.
- Parser: a block, record, enum, modal, class or extern body left open at end of file makes
  the reference loop forever (for example `let a = unsafe {`). The Rust port stops at end
  of file and reports the missing brace with `E-SRC-0520`.
- Parser: `ForeignContractClause.kind` is read uninitialised when a foreign contract clause
  is malformed. The Rust port uses `Assumes`; the comparison ignores that one field.
- Name resolution: the rules `E-MOD-1307` (a pattern `E::V { .. }` where `V` is both a
  record variant of enum `E` and a record of module `E`) and `E-TYP-1501` (the same shape
  where neither reading is a record) are missing from the reference's table of resolver
  rules, so it reports "Internal error: resolver failed with unmapped diagnostic id". A
  qualified class path whose last segment does not exist (`record R <: App::Missing`)
  fails without any rule and is reported as "Internal error: module resolution failed
  without diagnostic". The Rust port reproduces both for now; they are flagged here to be
  fixed once the type checker's own diagnostics for these inputs are ported.
- Name resolution: the type parameters of a record's method are not in scope while the
  method is resolved (`procedure map<TOther>(~, other: TOther)` in a record fails with
  `E-MOD-1301`), although those of class methods and free procedures are. Reproduced.
- Type core and layout: asking for the asynchronous signature, the size or the alignment
  of a type alias that is defined through itself (`type A = B`, `type B = A`;
  `type A = [A; 2]`), and taking an array length from a module-level binding that is
  defined through itself (`let N: usize = N`), all recurse until the reference crashes.
  The Rust port stops: such an alias has no signature and no size, and the length is not
  a constant.
- Compile-time pass: `introspect~>category` on a cyclic type alias (`type A = B`, `type B = A`)
  recurses until the reference crashes. The Rust port stops and the evaluation fails.

Known differences that remain until later milestones:

- `--dump` prints for every project that passes phase 1; the reference prints only after
  type checking succeeds, so 38 fixtures it rejects later still print here.
- Modules of an assembly are parsed sequentially; the reference uses worker threads and
  merges in the same order, so results are the same and only wall time differs.
- `#test(covers(...))` looks for the obligation ledger under the working directory and its
  ancestors only (the reference also checks its support bundle and a build-time path).
- `--profile-compiler` events for parsing are not emitted.
- The resolver does not record language-service facts (declarations of locals and
  references to symbols); the reference records them only when the LSP drives it, so
  they are part of M4. Entities carry the declaration span but no symbol id yet.
- Conformance-trace records written by name collection and module resolution are not
  emitted (the trace itself is part of M3.6).

Not ported yet, although it lives in the source directories of finished milestones:

- `00_core`: static rule registry (`behavior_model.cpp`), host primitive classification,
  crash reports, compiler sidecar support, runtime ABI table.
- `01_project`: linking, tool resolution, target platform and IR assembly (needed in M6).
- `02_source`: AST utilities that only later phases call (visitors, cloning helpers,
  `ast_dump` beyond the item summary) are ported when their first caller is.
- `04_analysis/resolve`: the assembly import graph (`assembly_import_graph.cpp`), which
  only the driver calls, comes with M3.6. Five files there have no caller in the
  reference build and are not ported: `resolve_callee.cpp`, `resolve_enum_payload.cpp`,
  `resolve_expr_list.cpp`, `resolve_extern.cpp`, `resolve_attributes.cpp` (the resolver
  uses its own copies of what they define), as are the step-wise name collection
  (`NamesStep`) and `DeclNames`.
- Driver: `test`, `init`, `clean`, `--conformance`, `--profile-compiler`.
