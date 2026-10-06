# Ultraviolet in Rust

A port of the Ultraviolet compiler ([blacklight-foundation/ultraviolet](https://github.com/blacklight-foundation/ultraviolet),
v0.4.0-alpha, about 313k lines of C++) to Rust. The port is faithful and gated: each
part is compared against the reference compiler on the whole corpus before the next
part starts.

- [docs/running.md](docs/running.md): building, running the compiler, running the gates.
- [docs/zed.md](docs/zed.md): Ultraviolet support in Zed, as it stands.
- [PLAN.md](PLAN.md): the plan, every gate and its result, and the known gaps.

## Where the port is

The Rust `uvc` cannot yet say a program is accepted, or build it. `uvc build --check`
runs project loading, parsing, the compile-time pass, name resolution, type checking
and the capability and authority checks, and reports the diagnostics the reference does
(identical on 593 of the 685 projects of the corpus, none different). A program that
passes all of that stops with exit status 3 where the lowering to IR begins.
To check or build Ultraviolet code today, use the reference compiler
([docs/running.md](docs/running.md#the-reference-compiler)).

Behind the command line, the front end is ported well past that point as libraries,
each piece matching the reference on the corpus:

| Milestone | What | State |
| --- | --- | --- |
| M0 | Workspace, parity harness, goldens from the reference | done |
| M1 | Core, project loading, lexer | done |
| M2 | Syntax tree, parser, attributes | done |
| M3.1 | Compile-time pass | done |
| M3.2 | Name resolution | done |
| M3.3 | Type core, generics, modal types, composite types, layout | done |
| M3.4a | Literals, patterns, constraint solving | done |
| M3.4b | Expression and statement typing | done against its gate: 4,704 bodies |
| M3.4c | Declaration typing and the type-check entry points | done against its gate: all 6,822 declarations; all 546 projects identical end to end |
| M3.5 | Memory, provenance, capabilities, keys, contracts | done with M3.4c, against the same gate |
| M3.6 | Driver phases 2 and 3: `uvc --check` end to end | in progress: everything up to the lowerability check, which is the IR lowering of M6; 593 of 685 projects identical, 92 waiting for it |
| M4 | Language server in Rust | done against its gate: `uv-lsp` passes upstream's protocol suite and the Shapes example |
| M5 | Editors connected to the Rust server | Zed extension in `editors/zed`; the other editors' adapters are upstream's and start `uv-lsp` by name |
| M6 | IR, lowering, LLVM emission, linking | not started |
| M7 | Packaging, installers, CI | not started |

A structural tree-sitter grammar for the language is in `editors/tree-sitter-ultraviolet`.
It is not packaged into an editor extension yet.

`uvc --check` starts working on real projects at the end of M3.6, and `uvc build` at the
end of M6.

## Layout

```
crates/
  uv-core       spans, diagnostics, Unicode, host services
  uv-project    Ultraviolet.toml, module discovery, target profiles
  uv-source     lexer, syntax tree, parser, attributes
  uv-comptime   compile-time evaluation, quote, reflection, derive
  uv-analysis   name resolution, typing, layout, and the rest of analysis
  uvc           the compiler binary
  uv-parity     dumps compared against the reference by the gates
editors/
  tree-sitter-ultraviolet
tools/          the parity harness and the oracle built from the reference sources
tests/golden/   what the reference produced
```
