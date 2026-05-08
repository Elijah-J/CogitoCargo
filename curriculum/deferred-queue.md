# Deferred Queue

This file is the running queue of deferred items surfaced during EduRatchet-2
backpropagation passes. Items here are NOT in `graph.md`'s draft section —
they are concept candidates not yet promoted to a draft node.

Each entry names:
- the concept;
- the source lesson(s) where the deferral was registered;
- a classification: `ready now`, `requires prerequisites`, or `out of scope`;
- for `requires prerequisites`, the missing prerequisites by name;
- for `out of scope`, a one-line reason.

`ready now` items are candidates for the orchestrator to dispatch as new
normal cycles. `requires prerequisites` items wait for their prerequisites
to be installed first. `out of scope` items are recorded so the run does
not re-surface them.

## Pass 2026-05-07 — Nodes 001-005 backprop pass

This pass surfaced seven queue items (Q01-Q07) and a list of out-of-scope deliberations. Items have moved as the run has accepted further nodes. Current state:

### Ready now

None.

### Requires prerequisites

#### Q07: `main() -> Result<...>` and the `Termination` trait
- Source bullets: lesson 002 (`main() -> Result<...>, and the Termination trait that the Rust Reference mentions next to main`).
- Description: `main` is allowed to return any type that implements the `Termination` trait, including `Result<(), E>`; non-zero exit codes can be produced by returning `Err(...)`.
- Missing prerequisites: trait machinery as a concept (deferred since lesson 013); fluency with `Result` (lessons 052/053/058 cover the operational shape but not enough to bind to a trait). Trait machinery here IS load-bearing — `Termination` is literally a trait, so a teach-able lesson must name traits.
- Status: blocked on traits-as-concept install.

### Out of scope

- Compiler internals (parsing, code generation, linking, etc.) — lesson 001. Not Rust-specific; out of scope for this run.
- Windows shells — lesson 001. Run targets Linux/macOS shells.
- `#![no_main]` attribute and "main can be imported" exotica — lesson 002. Advanced feature.
- OS / Rust runtime before `main` — lesson 002. Not Rust-specific.
- Type-error diagnostics as a separate walkthrough (E0277, trait bounds, lifetime errors, etc.) — lesson 003. No exhaustive walkthrough planned; specific type errors fire operationally in the lessons that surface them.
- Multi-error diagnostics — lesson 003. Out of scope.
- ANSI color codes / terminal coloring of diagnostics — lesson 003. Plain-text only.
- JSON diagnostic format (`--error-format=json`) — lesson 003. Not for human reading.
- Concurrency / parallel execution — lesson 004. Out of scope.

### Closed since the original pass

- Q01 (`scope` as a general concept) — installed by lesson 068 (`068-let-binding-scope`); annotated in lessons 003 and 005's `## What To Ignore For Now` sections.
- Q02 (`warning:` rustc warnings distinct from `error:`) — installed by lesson 069 (`069-rustc-warnings`); diagnostic-shape vocabulary from lesson 003 reused without extension.
- Q03 (`rustc --explain ECODE` as an interactive aid) — installed by lesson 070 (`070-rustc-explain`); closes lesson 003's named-but-deferred `--explain` trailer.
- Q04 (macro invocation syntax `name!(...)` vs `name(...)`) — installed by lesson 071 (`071-macro-invocation-syntax`) as a shallow operational install; depends_on cites no trait-related lesson per the queue normalization that removed that prerequisite.
- Q05 (string-literal type `&str`) — covered incidentally by lesson 055 (`055-string-trim`); annotated in lessons 001, 002, 003, and 004's `## What To Ignore For Now` sections.
- Q06 (`let` pattern destructuring, e.g. `let (a, b) = pair;`) — installed by lesson 073 (`073-let-tuple-destructure`) after lesson 072 (`072-tuple-type-and-index`) installed tuple types and values as the missing prerequisite; closes lesson 005's `Pattern destructuring on the left side of let` deferral.

## Status legend

- `ready now`: prerequisites are installed; orchestrator may dispatch a normal cycle.
- `requires prerequisites`: blocked on a named earlier install; do not re-surface as ready until prereq lands.
- `out of scope`: deliberately excluded from this run; recorded to prevent re-evaluation.
