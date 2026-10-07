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
and name resolution, are done). Run `tools/parity.sh` to re-check every gate (ten
minutes or more);
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
| Relations between types: for every type written in a declaration its well-formedness, intrinsic classes (`Bitcopy`, `Clone`, `Drop`, `FfiSafe`, `GpuSafe`, zeroable, `Eq`, discrete, ordered) and intrinsic method signatures; subtyping, argument compatibility and cast validity between the module's types; class linearisation, method and field tables, dispatchability, class subtyping and implementation; method and transition signatures; generic parameter validation, bounds and inference; static proofs of contracts and refinements; every project that resolves | identical on 545 of 546; the reference crashes on the other 1 |
| Same, the compile-time, name-resolution and type-core cases | identical on 121 of 144 (the reference crashes on 23), 410 of 410, and 52 of 54 (the reference crashes on 2) |
| Same, 35 targeted cases (each structural subtyping rule, variance, aliases, refinements, class hierarchies and their failures, the predicates, generic parameter lists and inference, 78 contracts for the prover) | identical, 35 |
| Constant encoding: every literal token encoded as each primitive type and as a raw pointer, and the bytes of every string literal; the corpus, the lexical stress cases and 235 targeted files | identical on 1585 of 1585, 4906 of 4908 (the reference crashes on 2), and 235 of 235 |
| Value bytes and validity: for the types of every module, values built from the type are encoded and the result judged, with a value of a neighbouring type and five byte patterns; every project that resolves and the four case sets | identical on 545 of 546, 121 of 144, 410 of 410, 51 of 54 and 35 of 35; the reference crashes on the rest |
| Literal typing: the type of every literal token and its check against 30 expected types; same three sets as constant encoding | identical (part of the constant-encoding dumps) |
| Pattern typing: every pattern written in a module's bodies against each of the module's types, with irrefutability and coverage; every project that resolves, the compile-time and name-resolution cases, and 8 targeted cases | identical on 546 of 546, 143 of 144 (the reference crashes on 1), 410 of 410 and 8 of 8 |
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
| M3.3 typing context, generics, modal, composite, layout | `typing` (core), `generics`, `modal`, `composite`, `layout` | about 45k | done, with the gaps noted below |
| M3.4 expression, statement and declaration typing | `typing` (rest) | about 52k | in progress, see below |
| M3.5 memory, provenance, capabilities, keys, contracts | `memory`, `provenance`, `caps`, `keys`, `contracts` | 36k | |
| M3.6 driver phases 2 and 3 end to end | `06_driver` (sema section), `conformance` | | in progress, see below; gate: `--check --diag-json` identical on every fixture |

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
| c. Relations between types: subtyping, well-formedness, intrinsic classes, class tables, method signatures, generic parameters and arguments, static proofs | `typing/subtyping`, `type_wf`, `type_predicates`, `signature`, `item_generic_params`; `composite/classes`, `class_linearization`, `record_methods` (receivers); `modal` (lookups, widening checks); `generics` (all of it); `contracts/verification` | done, with the two gaps below |
| d. Constant encoding (the bytes of a value of each type) | `layout/layout_value_bits` | done |

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

What M3.3c added:

- `uv-analysis::typing`: `subtyping` (with alias normalisation, variance of nominal
  arguments, refinement entailment, and `argument_type_compatible`), `type_wf`,
  `type_predicates` (`Bitcopy`, `Clone`, `Drop`, `FfiSafe`, `GpuSafe`, zeroable, `Eq`,
  discrete, ordered, casts, the intrinsic `eq`/`successor`/`predecessor` signatures),
  `signature` (method and transition signatures, `Self` substitution).
- `uv-analysis::composite`: `class_linearization` (C3), `classes` (method and field
  tables, dispatchability, class subtyping, whether a type implements a class,
  completeness of an implementation, the orphan rule), `record_methods` (receiver types
  and modes).
- `uv-analysis::contracts::verification`: the static prover (constant folding, known
  facts, conjunction and disjunction, linear integer entailment by simplex with branch
  and bound).
- `uv-analysis::modal`: `lookup` (states, fields, methods, transitions) and the widening
  checks; `caps::context_caps::is_capability_class`.
- `uv-analysis::generics`: `generic_params` (validation of a parameter list, its scope,
  counts, constant parameters), `monomorphize` (bounds on arguments, inference of type
  arguments by matching, the set of demanded instantiations), `where_bounds`; and
  `typing::item_generic_params` (parameters as declaration typing sees them, the check of
  arguments against them).
- The oracle gained a `relations` mode; `tools/gen_relation_cases.py` writes the targeted
  cases (35 projects: each structural subtyping rule, variance, aliases, refinements,
  class hierarchies with every linearisation and table failure, the predicates, generic
  parameter lists and inference, and 78 contracts for the prover).

Two gaps in M3.3c, both waiting for expression typing (M3.4):

- Well-formedness of a refinement type types its predicate as an expression and checks
  that it is pure. `type_wf` returns a marker for such a type (`REFINEMENT_WF_PENDING`)
  instead of an answer, and the parity gate does not compare the well-formedness of types
  that contain a refinement. Everything else about refinements (subtyping by proof,
  predicates, layout) is compared.
- The functions of these files that type expressions or take the expression typer as an
  argument are left for M3.4, where their callers are: `BuildProcedureSignature`
  (`signature`), receiver and argument checks and `LookupMethodStatic`
  (`record_methods`), and all of `composite/records`, `tuples`, `unions`, `arrays_slices`
  and `function_types`.

Two things about the reference worth knowing before M3.4 builds on this part. Its
instantiation worklist (`ProcessToFixedPoint`) only marks entries processed; nothing is
instantiated there. And it validates a default type argument with an unsupported form
nested inside it (`<A = (Range<i32>, i32)>`) into a type with a hole, which its own
printer then cannot print; the Rust port builds the same type and prints the hole.

One numeric difference: the prover's simplex uses `long double` in the reference (80-bit on
x86-64) and `f64` here, with the same tolerance. No input in the gates tells them apart;
a system whose pivots differ only beyond 53 bits of mantissa could.

What M3.3d added:

- `uv-analysis::layout::value_bits`: `encode_const` (a literal as a primitive type, `null`
  as a raw pointer), `decode_string_literal_bytes`, `value_bits` (a structured value in
  the layout of its type) and `valid_value` (whether bytes are a value of a type), with
  the value representation they share.
- The oracle gained `consts` and `values` modes; `tools/gen_const_cases.py` writes 235
  files of literals (integers at the edges of each width and base, floats at the
  rounding and range edges of each precision, every character and string escape and
  malformed forms of each).

Things about the reference's constant encoding that M3.4 and code generation inherit:

- Half precision is converted by the reference's own routine, not IEEE rounding: a
  single-precision subnormal becomes zero, and every NaN gets its lowest payload bit set.
  A literal is read as a double first and narrowed from there, so `f32` and `f16`
  literals are rounded twice. Reproduced.
- `value_bits` has no encoding for records, modals or unions, and `valid_value` accepts
  no nominal type, union, function or refinement. The reference file holds helpers for
  them (`ValueBitsForRecord`, state and niche helpers) that nothing calls; they are not
  ported.
- Text values encode as zeroes of the right size.
- Float digits go through `strtod` in the reference and Rust's parser here. Both round
  decimal forms correctly (2,724 literals in the gate agree); `strtod` would also take
  hexadecimal floats, which the lexer never produces.

M3.4 is the largest slice: about 52k lines of reference in `typing/expr`, `typing/stmt`,
`typing/item`, `typing/pattern` and the files beside them. Expressions, statements and
blocks type each other through callbacks, so most of it can only be compared once it is
all there. It is cut so that everything that can be compared alone is compared first:

| Part | Reference source | Lines | State |
| --- | --- | --- | --- |
| a. Leaves that stand alone: literals, patterns, the result and environment types, constraint solving | `literals`, `pattern/pattern_common`, `type_infer` (`Solve`, `ApplySubstitution`), environment operations of `stmt_common` | 3k | done |
| b. Expression and statement typing | `type_expr`, `type_infer`, `if_case_check`, `expr/*`, `stmt/*`, the expression-typing functions of `composite` and `record_methods` left over from M3.3, refinement well-formedness | 36k | done against the body gate: all 4,704 bodies compared (procedures, methods, transitions) with the stores typing fills and the context declaration typing sets, none mismatched or pending |
| c. Declaration typing and the type-check entry points | `item/*`, `typecheck` | 13k | done against the type-check gate: all 6,822 declarations compared, none mismatched or pending; all 546 projects identical end to end (diagnostics, initialisation plan, main check) |

Part c and M3.5 are ported together against one gate, because declaration typing calls
straight into the code of M3.5 (the borrow check of each body, contract checks, the
initialisation plan). The oracle's `typecheck` mode runs the reference's
`TypecheckModules` on every project as the driver does and prints, per declaration, the
diagnostics reported inside its extent (code, severity, message, span, label,
obligations, children), then the outcome and the diagnostics left over. The port
(`typing::typecheck`) prints the same, or `PENDING` for a declaration of a kind it
cannot type yet and for the tail of the check while anything before it is pending.
`tools/compare_check.py` compares what is not pending. The corpus has 6,822
declarations in 503 projects that resolve: 4,569 procedures, 1,500 `using`, 332 records,
114 aliases, 82 classes, 76 enums, 72 statics, 44 extern blocks, 26 modals, 7 imports.
The reference reports diagnostics with 260 different codes on them.

In so far: how a failed rule becomes a diagnostic (`typing::typecheck_diag`: the
overrides, the diagnostic tables, and the registry of static rules, generated into
`uv-core` by `tools/gen_static_rules.py`), the check that no two procedures are given
one symbol, the error limit, and `using` and `import` declarations
(`typing::item_using`), and the check of overloads that erase to one signature.

Procedure declarations have their frame (`typing::item_procedure`, the reference's
`TypeProcedureDecl`): attribute validation, the shape of a test procedure, parameter
names, the signature (with the rule that a refinement on a parameter may not speak of
`self`), generic parameters, the explicit return a non-unit body needs, the body typed
under the context of part b, and its type against the declared one. Each check that is
not ported makes the declaration pending when it would apply; none is left. The
foreign-interface attributes and export signatures (`typing::item_ffi`: ABI, mangle and
unwind checks, the mixed-mode and host-export rules, the by-value and zeroable checks, and
the two warnings), extern blocks (`item_ffi::type_extern_block`), modal declarations
(`typing::item_modal`: states, methods and transitions with their bodies, the invariant,
and the abstract states of the classes a modal implements), the `#dynamic` warning where
nothing needs a run-time check (`typing::dynamic_runtime`), the `inline(always)` warning
(`typing::inline_always`), refinement predicates (`type_wf`) and opaque return types
(the first `return` fixes the underlying type, kept in `Sigma::opaque_underlying_by_class_path`,
which typing fills through a shared reference as the reference compiler does) are all in.

The borrow check is in (`memory::borrow_bind`, the reference's `BindCheckBody`): for
each binding whether it is valid, moved or partly moved, and for each `unique` place
whether it is inactive because it has been lent. It walks the body with those two
environments, joins them where branches meet, iterates loop bodies to a fixed point,
passes arguments in order (a lent place stays inactive until the call is over), and
checks closures by what they capture and how. The parameter modes of a callee come from
the overload typing selected, the callee's type, or the declaration it names. A
binding initialised from a call owns the result unless the callee only hands back
something it was lent (`memory::return_responsibility`). The check also runs on a body
that failed typing, and what it finds is reported alongside. 336 procedures of the
corpus are decided by it or fail before it, and all match; the other 3,870 pass it.

The type checker's entry point sets up the stores of part b for the whole check, as the
reference does; the borrow check reads expression types back from them.

Not as in the reference: the statics of a module are bound again for each body rather
than cached, and the per-body timing is not kept.

The provenance check is in too (`memory::region_prov`, the reference's `ProvBindCheck`):
where each value's storage comes from (a region, the stack, the heap, a global, a
parameter), that nothing is stored in or captured by something that outlives it, and
the provenance of a call followed through the body of the overload typing selected. It
matched the reference on all of the corpus on its first run. The maps of expression
provenance the reference also fills for later phases are not kept yet.

Type alias and static declarations are in (`typing::item_simple`), with the capability
check that statics need (`caps::cap_requirements`, only whether a type carries any
capability, which is all that has been asked so far). So are enums
(`typing::item_simple`), records (`typing::item_record`: fields, implemented classes,
methods and their bodies), classes (`typing::item_class`), the signature of `main` with
its fix-it (`caps::context_caps::is_context_bundle_type`), and the check for overloads
that erase to one signature, type invariants, and contracts (also those a method inherits).

The tail of the check is in too: the initialisation plan (`memory::init_planner`, the
reference's `BuildInitPlan`: for each module the modules its types, its `static`
initialisers and its bodies depend on, a cycle among the eager ones reported as
`E-MOD-1401`, and the topological order) and the project-wide check that exactly one
`main` exists, is generic-free and has the signature of an entry point. With the rest of
declaration typing in, all 546 projects compare identically from start to finish.

Contracts are in (`contracts::intrinsics`, `contracts::contract_check`, and
`typing::expr::contract_entry` for `@entry`): `@result` only in a postcondition, an
`@entry` expression over bindings that exist at entry, without capabilities, side effects
or moved parameters; purity of predicates; the predicates typed as `bool` under the
contract phases; type invariants; and behavioural subtyping of an implementation's
contract against its class's. With them 471 of the 546 projects compare identically end
to end.

Left: modals (26) and extern blocks (44), the foreign-interface attributes of procedures
(66), the `#dynamic` no-effect warning (23), refinement well-formedness (12), the
`inline(always)` warning (10) and opaque return types (6).

Part b is ported against a gate that measures it. The oracle's `bodies` mode types every
procedure body as declaration typing does (type parameters and parameters in scope, the
declared return type expected) and prints one line per body: the outcome, rule, detail,
type, span and the diagnostics emitted. The port prints the same line, or `PENDING` with
the first construct it reached that is not ported (`typing::pending`, scaffolding that
goes away with the last such construct). `tools/compare_typing.py` compares the bodies
that are not pending, fails on any mismatch, and reports how many bodies are compared and
what the rest wait for. The reference types 4,569 procedure bodies on the corpus, 4,203
of them successfully and the rest with 120 different rules.

The same mode types the bodies of methods and transitions under the bindings of their
signatures, set up as the typing of record, class and modal declarations does: a
record's associated types substituted for `Self::Name` and its invariant assumed under
a plain `~` receiver; `Self` left a variable and the class current in a class method;
the state as the receiver of a state method, with the state's part of the modal's
invariant; a transition typed against its target state without the environment
reference or the diagnostic stream. That is 135 more bodies (111 methods of records and
classes, 15 state methods, 9 transitions), 6 of which fail in their signature. Two
thing differs from the driver and waits for part c: the modal's own parameters are
taken from the declaration without `ProcessGenericParams`.

The proof facts statements leave for later ones are tracked
(`typing::stmt::proof_facts`, the reference's `FallthroughProofContextForStmt`): an
assignment forgets the facts about the name it writes, a binding to a pure expression
is known to equal it, a binding to a call learns the callee's postcondition and a
foreign procedure's `ensures` clauses, and an `if` that leaves when its condition holds
leaves the condition false behind it.

So far part b has:

- the dispatch skeletons (`type_expr`, `type_place`, `type_stmt`) and the statement
  context;
- the block core: statement sequences, block typing with its result and break flow, the
  loop result types;
- checking an expression against an expected type (`check_expr`, `check_expr_against`):
  literals, `null`, closures by arity, array literals and repeats, negated literals,
  union membership, array-to-slice coercion, fresh aggregates under a permission, moved
  unique values, and the final `E-SEM-2526`;
- `return`, with outcome introduction (`outcome`) and array coercion
  (`composite::arrays_slices`);
- `move`, `copy`, and names bound in the environment as values and places.

Added since: `let`/`var`, module-level names (statics and procedures), binary and
unary operators, casts, `sizeof`/`alignof`, tuple and array literals, expression
statements, `if` with the narrowing of bindings by a pure condition, block expressions and
checking a block against an expected type, and the purity
analysis the typer asks for (`contracts::purity`, from the reference's
`contract_check`: it follows calls into procedure and method bodies). Purity is gated on
its own in the `relations` dumps: every contract clause and every expression written
directly in a body, 22,940 expressions on the corpus.

Calls are in (`memory::calls`, `typing::expr::call`): which procedure a callee names,
foreign procedures and the `unsafe` they need, default construction of a record named
without arguments, overload resolution among procedures of one name, explicit type
arguments with defaults and bounds, inference of type arguments from the arguments and
from the expected type, argument passing (`move`, `copy`, places passed by reference),
the write-key requirement for shared arguments to `unique` parameters, and the raw
pointer check at the foreign boundary. The callee's precondition and a foreign
procedure's `assumes` clauses are proved at the call with the arguments substituted
(`typing::expr::call_contracts`). A call under held keys of a procedure whose key
accesses are unknown warns (`typing::expr::callee_key_access`); the reference also
collects the accesses themselves, which nothing reads during typing, so only whether
they are unknown is computed. The selected overload and the inferred substitution are
recorded in the stores described below.

Field access (records, modal states, `Self` in a class) and tuple element access are in
too, as values and as places (`typing::expr::field_access`, `tuple_access`).

Assignment is in (`typing::stmt::assign_stmt`): the place must be a place, not `const`,
rooted in a `var` unless written through a pointer, and the value is checked against the
declared type of the place, with outcome introduction; the root binding then takes the
value's provenance. Writing a `shared` place needs a write key that covers it, and a
value that reads the place it writes is a read followed by a write of one location
(`typing::stmt::shared_write`). Compound assignment is in with it.

Record literals (records and modal states) and enum literals are in
(`typing::expr::record_literal`, `enum_literal`), on their own and against an expected
type, which supplies the type arguments of a generic record, modal or enum.

`if … is` and `if … case` are in (`typing::expr::if_case`, from the reference's
`if_case_check`), typed on their own and checked against an expected type: the arm
environments with the scrutinee narrowed to what the pattern matched and `else` to what
the patterns rejected, unreachable arms, and exhaustiveness over enums, modals (an
asynchronous computation that cannot fail has no `Failed` state to cover) and unions.
One simplification: a typed pattern's type is lowered with the general `lower_type`
where the reference uses a local lowering that does not handle every type form; the
gate shows no difference on the corpus.

Method calls are in for declared methods (`typing::expr::method_call`): methods and
transitions of a modal state, class methods through dynamic and opaque types, a record's
own methods and those of the classes a type implements, inference of a generic method's
type arguments, the receiver's permission and key requirements, the argument checker
(`ArgsOk`), and `eq`/`successor`/`predecessor`. The methods the language builds in are
in too: those of the capability classes and compile-time capabilities
(`caps::cap_methods`), of strings and bytes (`memory::string_bytes`), of `Region`,
`CancelToken` and `Async@Suspended`, the asynchronous combinators, and `until`.

The smaller forms are in: dereference, indexing (elements and range slices, as values
and places), address-of, ranges and `unsafe` blocks as expressions and statements
(`typing::expr::access`), and the three loops with `break` and `continue`
(`typing::expr::loops`). Loop invariants are in
(`typing::expr::loop_invariant`): the invariant must be a pure `bool` without
`@result` that holds on entry, and the body must leave its names alone, or, in a
conditional loop, leave the invariant provable with its assignments applied.

Closures and pipelines are in (`typing::expr::closure_expr`), with the capture analysis
(`typing::closure_capture`): the names a closure uses from outside, whether any is
shared, and whether its body spawns. A closure that captures nothing is a function
type; an expected closure type supplies what the closure leaves out. The checks that
use the capture facts are in with it: a closure expected to declare shared dependencies
must not spawn, and a returned closure must declare the shared data it captures.

The postcondition is proved at `return` (`typing::stmt::postcondition`): the returned
value takes the place of `@result`, `@entry(e)` becomes `e` where it cannot have
changed, and the predicate is simplified where the value decides a branch.

Refinement predicates are proved when a value is checked against a refinement type:
the predicate with the value for `self` must follow from the facts in scope
(`E-TYP-1953` otherwise). In a dynamic context a refinement that cannot be proved is
left to a run-time check and the value only needs the base type; the check is
recorded in the stores.

The scoped statements are in (`typing::stmt::scoped`): `region` and `frame` with the
active region they bind, `defer`, and `using`; so are `?` propagation, `transmute` with
the warning pass that runs after an `unsafe` block, and region allocation
(`typing::expr::transmute`).

The asynchronous forms are in (`typing::expr::async_forms`): `yield` and `yield from`
with the release of keys at a suspension point, `sync`, `race` with returning or
yielding handlers, `all`, and `wait` on spawned and tracked tasks.

`parallel` and `spawn` are in (`typing::expr::parallel`): the execution domain and its
options, the block's value (its tail, or the values of its tasks, which are typed a
second time to collect them as the reference does), what a task may capture (a `unique`
binding only by `move`, and a binding of the block by one child only), and the
restrictions on captures into GPU code. Captures are visited in the order of the
reference's hash set, which decides the error reported first. `dispatch` is in: its
range, index pattern, body, captures and options, and the keys each iteration needs
(`typing::expr::dispatch_keys`). Those come from the key clause, or are inferred from
the uses of shared data in the body: a use that is not a path with iteration-invariant
indices is rejected (`E-CON-0141`), as are two uses that are not provably disjoint
unless both read (`E-CON-0142`); an index that is more than literals and index
variables is warned about (`W-CON-0140`). The key set itself is not recorded for later
phases yet.

Key blocks are in for the ordinary forms (`typing::stmt::key_block`): the paths must be
rooted in shared data and marked at most once at a record field, a key already held may
not be taken again in another mode except to release it, a block that writes under its
own key needs write mode, and the body is typed with the keys held. The variants are in
`typing::stmt::key_block_checks`: `ordered` paths must differ only in their indices; a
path indexed by a value known only at run time is accepted when the body's own indices
cannot conflict (equal or provably different by constants, a proved inequality, offsets
from one name, an index particular to each parallel task, or loop ranges that do not
meet) and the block cannot race with another task; a speculative block must be a write
block that writes only what its keys cover and makes no impure call.

The declaration tables are now shared between contexts instead of copied, as typing a
body under another module's name needs a context of its own; the body dump went from
37 to 5 seconds.

The last small forms: attributed expressions as values, places and against an expected
type (`typing::attributed`: attribute validation, a memory ordering only on an access
to shared data, `#dynamic` as a dynamic context), `alloc_raw` on a heap allocator
where a raw pointer is expected, the GPU barrier divergence check of `if`, and the
provenance of a returned safe pointer or, at an exported boundary, raw pointer.

Known gaps inside what is ported, each of which makes a body pending when reached
rather than answering; no body of the gate reaches one: `comptime` expressions (also under
attributes), a quote inside compile-time code and `@entry` inside a postcondition
(outside those contexts both are rejected as in the reference).

The stores typing fills for the passes after it are in (`typing::expr_store`, on the
context as `stores`): the type of each expression as last typed or checked, the same
for expressions typed as values, the substitution of each generic call, the overload a
call with several candidates selected, and the refinements left to a run-time check.
Typing reads them back too: a check against an expected type is answered from the type
the expression already has as a value when that fits, the purity analysis of contracts
takes the type of a receiver from them, and the postcondition facts of a call use the
selected overload. The gate sets the stores up as the type checker's entry point does
and compares, per body, the size and a hash of each (`UV_STORE_VERBOSE` prints the
entries; `tools/store_diff.sh` shows the ones that differ for a project). With the
stores one body of the corpus types differently, through the purity analysis.

The reference keys the stores by the address of the syntax node, also for nodes it
synthesizes and frees while typing (the `move` it wraps a returned place in, for one).
Two consequences for the gate. Only entries for nodes of the project's modules are
compared, found by a walk generated from the tree's description for both sides
(`tools/gen_ast.py`, `ast::walk`). And a freed node's address may be handed to a later
node, which the reference then answers from the stale entry; 7 bodies of the corpus
depended on that. The oracle now keeps the memory of expression nodes while it types
bodies, so no two share an address, and the port keeps the node of each entry alive
for the same reason. The port does not reproduce the reference's answers under address
reuse. Calls are keyed by the address of the call inside its node; those entries are
compared by value only.

The readers of the stores outside typing (`memory/regions`, `borrow_bind`,
`return_responsibility`, `caps`, layout) come with M3.5, and the provenance stores are
written there.

The gate sets the rest of the typing context as declaration typing does
(`typing::dynamic_context`): a body is in a dynamic context when its declaration, the
type around a method, or the class method a record's method implements is marked
`#dynamic`; an exported procedure is a foreign boundary; a test procedure's
postcondition is left to run time. That changed 128 bodies of the corpus, 124 of them
from a failure to success (unprovable contracts, non-constant array indices and
dynamically indexed key paths become run-time checks), and 2 exported procedures now
fail for returning a raw pointer into a region.

Part b calls into code that belongs to M3.5 (`memory/regions`, `memory/calls`,
`memory/borrow_bind`, `contracts/contract_check`, `keys/key_paths`, `caps`); what it needs
from there is ported with it.

What M3.4a added:

- `uv-analysis::typing::literals`: the type of a literal, the check of a literal against
  an expected type, and where `null` is expected. Compared for every literal token of the
  corpus, the lexical cases and the 235 literal files, against 30 expected types each.
- `uv-analysis::typing::pattern`: typing a pattern against a type (with the merging of
  bindings across union members), irrefutability, and whether an enum or modal pattern
  covers its variant or state. The oracle gained a `patterns` mode that types each
  pattern written in a module's bodies against each of the module's types;
  `tools/gen_pattern_cases.py` writes 8 cases with 134 patterns.
- `uv-analysis::typing::solve`: applying a substitution of type variables, unification
  with the occurs check, and solving a list of equality and subtyping constraints.
  Compared in the `relations` mode on fourteen constraint sets built from each pair of
  neighbouring types of every module, and on the equation of every pair.
- `uv-analysis::typing::type_env`: bindings and scopes, introducing and shadowing
  names, lookups, provenance seeds, staleness, the names of a pattern, and the typing of
  a binding pattern (which, unlike a matching pattern, admits only irrefutable forms).
  Compared in the `patterns` mode: every pattern is also typed as a binding, and the
  bindings of the first type that accepts it are run through a fixed script of
  environment operations. `EmitStaleBindingReferenceWarning` needs the statement
  context and comes with part b.
- `uv-analysis::typing::expr_result`: the result of typing an expression. It keeps the
  reference's shape (a flag, an optional rule, a type, detail) rather than a `Result`,
  because the reference sets these independently and 36k lines of callers read them.

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
- Relations between types: subtyping and the `GpuSafe` check also follow a type alias
  that is defined through itself until the reference crashes. The Rust port stops: such a
  type is a subtype of nothing and is not GPU-safe.
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

## M3.6: the driver, phases 2 and 3

`uvc build --check` runs, in the order of the reference driver (`crates/uvc/src/sema.rs`):
the signature check of compile-time procedures (`typing::comptime_avail`), the
compile-time pass, the visibility check, name collection and resolution, the graph of
imports between assemblies and its two validations (`resolve::assembly_import_graph`),
type checking, the call graph and capability chain (`caps::callgraph_caps`), and the
authority model (`caps::authority_model`: ambient authority, attenuation, extern
isolation). The capability sets and signatures they use are in `caps::cap_requirements`.

Gate: `tools/parity_check.py` runs `uvc build --check --diag-json` on every project that
has a reference result (`projects`, `project_cases`, `phase1_cases`: 685) and compares
the exit status and the JSON. Result: 593 identical, none different, 92 pending. A
project is pending when the run reaches the lowerability check, which in the reference
lowers every module to IR (`driver::ValidateLowerability`), so it waits for the lowering
of M6.

Simplifications, all of which can only change the order of two reports from the same
check: the call graph keeps its nodes in the order they were added where the reference
iterates a hash table, and so does the attenuation check with its bindings. The rest of
the output of the assembly graph (the modules and libraries to emit) waits for M6.

## M5: editors

`editors/zed` is a Zed extension that uses `editors/tree-sitter-ultraviolet` (copied in
by `tools/sync_zed_grammar.sh`, checked by `--check`), has highlight, bracket, indent and
outline queries, and starts `uv-lsp`. Checks: it compiles to `wasm32-wasip1`, every query
runs under the tree-sitter CLI against a sample file, the grammar's corpus passes, and
upstream's `tests/validate_adapters.py` passes for the Neovim, VS Code, JetBrains and
Sublime adapters (they start the server by name, so they use the Rust one unchanged).
Not done: opening the extension in Zed, VS Code and Neovim by hand, and downloading the
server from releases (M7).

## M4: the language server

`crates/uv-tooling` (paths and URIs, line index, open documents, one analysis of the
workspace as the reference's `AnalyzeWorkspace`) and `crates/uv-lsp` (the server) port
`06_driver/tooling`, `04_analysis/language_service/facts.cpp` and the C++ server of
`ultraviolet-lsp`. Name resolution records the facts the server answers from
(`uv-analysis::language_service`, threaded through the resolver as an optional index, so
the resolver's output is unchanged: the resolve gates still pass). A written type is
printed for signatures by `uv-source::ast::pretty`.

Gate: `ultraviolet-lsp/tests/lsp_protocol_driver.py` (the upstream protocol suite,
unchanged) and `examples/shapes/validate_lsp.py` pass against `target/release/uv-lsp`;
both are in `tools/parity.sh`.

Simplifications: the tooling's older `SymbolIndex` is not ported (the server does not
use it); the Windows forms of paths and URIs (drive letters) are not carried; the
analysis worker hands a snapshot to the main thread with the types of expressions as
pairs of span and type instead of the reference's pointer-keyed map.

## M6: IR, lowering, LLVM, linking

Started with the end that can be checked: `crates/uv-codegen` has the IR model
(`ir.rs`, from `ir_model.h`) and the text dump (`ir_dump.rs`, from `ir_dump.cpp`). The
gate is `tools/parity_ir.py`: the reference's `uvc build --emit-ir` was recorded for the
90 projects it accepts (`tests/golden/*/<id>.ir`, by `tools/oracle/run_reference_ir.sh`),
840 procedures, 7 constants, 3 zeroed globals and 9 extern procedures in all, and the
Rust output is compared declaration by declaration. Those 90 projects are exactly the
ones `uvc --check` leaves pending at the lowerability check, so lowering them is also what
finishes M3.6.

Still to port, by size of the C++ it comes from (lines): `lower` 36k (expressions,
statements, patterns, procedures, modules), `globals` 3.5k (initialisation, literals,
entry points), `intrinsics` 3.9k, `cleanup` 3.5k (drops, unwinding), `abi` 2.1k,
`checks` 1.4k, `symbols` 1.3k (mangling, linkage), `dyn_dispatch` 1.2k, and the driver's
`BuildCodegenCache` and `PopulateCodegenModules`. Then `llvm` 46k for code emission and
the linker driver. The lowering is the next step; each slice is judged by how many of the
859 declarations of the gate come out identical, with the constructs not ported yet
reported as pending, as the typing was.


### M6 progress: lowering

`crates/uv-codegen/src/lower/` (`mod`, `expr`, `call`, `place`, `stmt`, `proc`, `module`,
`statics`, `cleanup`, `keys`, `drop`) lowers procedures and modules. A construct that is
not ported yet is recorded and its declaration is left out of the output; `uvc --emit-ir`
prints what lowered and exits 3 when anything did not, with one `pending:` line per
declaration on stderr (`tools/pending_ir.py` counts them over the golden projects).
`tools/parity_ir.py` compares the declarations that were printed and counts the others
as pending. State of the gate: 90 projects, 318 of 859 declarations identical, 0
different, 0 extra; the driver gate has 638 of 685 projects identical.

Ported: literals (full), identifier reads, binary/unary/short-circuit operators, `if`,
tuples, arrays, records, field and tuple access, `sizeof`/`alignof`, moves, calls of
procedures of the program (selected, identifier, path; by-reference and by-move
arguments; panic-out), extern blocks and calls to foreign procedures, `export`,
`host_export`, `mangle`, `unwind`, `inline`, `cold` and `dynamic` attributes, `let`/`var`,
assignments to locals and to fields, tuple elements and scalar indexes, addresses of
places, index access, `return` (with the snapshot of values that are not bitcopy), the
key system (key blocks, implicit key access, ordering fences), statics with their
initialisation and deinitialisation, record methods, class default methods, modal state
methods and transitions, `TypeNeedsDrop`, cleanup plans and the expression provenance
map.

Simplifications, each of which leaves a declaration pending rather than wrong:

- The numbering of bindings (`__bind_<n>_<name>`) is one counter over the whole program.
  Once a construct is skipped, later declarations that bind names are left out too.
- Drops are ported for values that need no drop; any other drop (drop glue) is pending.
- Contracts are only lowered when inert (outside `#dynamic`); dynamic contract checks,
  calls from dynamic code to a callee with a precondition, and the aggregate copy elision
  analysis (`AnalyzeAggregateCopyElision`) are pending.
- A returned value whose type is a permission over a bitcopy type (`unique i32`) is
  pending: the reference snapshots the read of a field of a unique receiver, but not after
  an assignment to it, and the type store behind that is not reproduced.
- `BuiltinSym` and the catalogue of runtime symbols are not ported.
- The derived-value table holds the kinds the ported expressions need.
- Speculative key blocks, raw-dylib externs and foreign contracts are pending.
- Pending constructs, by order of work: ranges and slices, enum literals, casts,
  `transmute`, pointer null, propagate (`?`), loops, `if case`, yield, `#test`, generics
  and monomorphization, vtables and dynamic dispatch, then async (race, sync, parallel,
  all, closures, async returns, stream and sequence combinators) last.

### Backend decision: textual LLVM IR, `llvm-as`, `ld.lld`

The reference emits machine code through the LLVM C++ API (46k lines under
`05_codegen/llvm`). No LLVM development files exist on this machine, and the release the
reference ships (`reference/ultraviolet/linux/tools`) has only `ld.lld`, `llvm-ar` and
`llvm-as`. That is enough: `llvm-as` turns textual IR into bitcode and `ld.lld` links
bitcode with LTO (it needs a `target datalayout`), so the Rust backend can emit textual
LLVM IR from the lowered IR and finish with those two tools, no LLVM library in the
build. Checked in the oracle image (`tools/oracle/Dockerfile.link`, which adds
`libxml2` that `ld.lld` needs): a hand-written `.ll` built and ran, and the reference
`uvc build` of a `main` returning 7 builds and runs there (exit code 7), with
`--runtime-lib reference/ultraviolet/UltravioletRT.a`. Its object holds `main`, the
lifecycle procedures (`__cx_lifecycle_init_<Assembly>`), the module init and deinit,
the poison flag and the literal data; the runtime start object calls `main`.

The gate for emission is behavioural: the same program built by the reference (in the
oracle image) and by the Rust `uvc` must give the same exit status and output.

### M6 progress: LLVM emission and linking

`crates/uv-llvm` is the textual IR builder (types with LLVM's data layout rules, IRBuilder's
defaults and constant folding, the module printer). `crates/uv-codegen/src/llvm` is the port of
`05_codegen/llvm`: the type mapping (`GetLLVMType`), the calling convention (`ComputeCallABI`),
modules, procedures, the entry point and its lifecycle procedures, the entry points of shared
libraries and their visibility. `crates/uv-project/src/link.rs` finds the tools, makes an object
of each module (`llvm-as`, then `ld.lld -r`, which runs the code generator) and links an
executable with the runtime. `uvc build` of an executable now builds a program that runs
(`main` returning 7 exits 7). Libraries are not linked yet.

The gate is `tools/parity_ll.py`: the `.ll` the reference writes (`emit_ir = "ll"`, recorded by
`tools/oracle/run_reference_ll.sh` into `tests/golden/projects/<id>.ll`) against the one the
Rust `uvc` writes, per function, global and declaration, with registers renamed by order of
definition and comments dropped. A procedure with a form of IR that is not emitted yet is
left out and counted as pending, as in the IR gate. State: 227 entities identical, none
different, 2228 pending, in 81 projects.

Emitted so far: `Seq`, `Block`, `Return` of immediates and locals, poison checks, the panic
record, `Opaque`. Everything else of the IR (calls, binary operations, variables, `if`, loops,
aggregates, checks, ...) is next, in the order the goldens need it; async comes last, and
parameter attributes beyond the ones the ABI sets are not ported.

### What is left to use it as a compiler and a language service
The language service (M4) and the editors (M5) already run on the ported front end, so
editing, diagnostics, hover, completion and navigation work today without code
generation. What stops `uvc build` from producing a program is the rest of M6, in order:

1. Finish lowering the pending constructs above until every golden declaration is
   identical (the IR gate at 859 of 859).
2. Emit every form of the IR as LLVM (the `.ll` gate), then link libraries and dependencies;
   goal: `Tools/RunHelloVerification.py` passes.
3. M7: packaging.
