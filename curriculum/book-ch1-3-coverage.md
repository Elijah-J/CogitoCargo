# Book Chapters 1-3 Coverage Queue

This file tracks the audit-grade coverage gap between the local Rust Book
chapters 1-3 (`output/docs/rust/book/ch01-*` and `ch03-*`, plus the Ch2
guessing-game tutorial that the rand-capstone already closes) and the
graph's accepted nodes. The goal: a learner taking a quiz on anything
even slightly load-bearing in Chapters 1-3 should not miss because the
graph skipped it.

Classifications used:

- `already covered`: a prior accepted lesson installs the concept. A
  parenthetical names the lesson(s).
- `ready now`: prerequisites are installed; orchestrator may dispatch a
  cycle.
- `requires prerequisites`: blocked on a named earlier install in this
  queue or elsewhere.
- `out of scope`: deliberately excluded from Ch1-3 closure (e.g. host-OS
  specifics, internals).

## Pass 2026-05-07 — Ch1-3 closure pass

### Already covered

- Hello-world program shape (`fn main`, `{}` body, `;` line ends), `rustc
  file.rs` + `./file` two-step workflow, `.rs` extension. (Lessons 001,
  002, 003, 011.)
- `cargo new` + `cargo run`, `Cargo.toml` `[package]` block (name,
  version, edition) and `[dependencies]` section header, `src/main.rs`
  layout. (Lesson 032.)
- `cargo build` + `target/debug/<name>` executable. (Lesson 064.)
- `[dependencies]` SemVer-pinned entry; resolver fetching from
  crates.io. (Lesson 065.)
- `println!` macro vs function syntax (`name!` vs `name`). (Lesson 071.)
- `let` immutability default + `let mut` reassign + E0384. (Lessons 005,
  006.)
- Shadowing with second `let`; type-changing shadowing. (Lessons 007,
  057.)
- `if`/`else`, `else if`, `if`-as-expression with `let`, same-type-arms
  rule. (Lessons 026, 038.)
- `loop` + `break`, `break value;` returning out of `loop`,
  `continue`, `while`, `for var in 0..N`, inclusive range `0..=N`.
  (Lessons 017, 022, 027, 028, 035, 039.)
- `match` expression with bool, integer, and enum scrutinees, all in
  *match-arm pattern* position only. Specifically: `match` itself with
  exhaustiveness on a `bool` scrutinee (lesson 030); integer literal
  patterns plus inclusive-range patterns like `i32::MIN..=0_i32` and
  the wildcard `_` (lesson 031); unit-variant `Ordering` enum-variant
  patterns (lesson 051); payload-bearing `Result` variant patterns
  `Ok(num)` / `Err(_)` (lesson 058); `continue` in match arms (lesson
  059). Tuple patterns on the LEFT of `let` are lesson 073. Range
  patterns on the LEFT of `let` are NOT installed (only in match
  arms).
- `bool` literals + comparison + logical operators; `if` condition must
  be `bool`. (Lessons 012, 013, 015, 026.)
- `i32` (typed name + integer-literal default + `: i32` annotation
  slot); `u32` (the unsigned counterpart, used in the rand capstone);
  `f64` (floats, default float type); `as` cast `i32 as f64`. (Lessons
  019, 062, 033, 034.)
- Numeric operators + remainder `%`. (Lessons covering arithmetic;
  remainder is 037.)
- Functions: `fn name() { ... }` definition + `name();` call;
  parameters with `: TYPE`; multiple parameters; `-> RTYPE` return
  type; `return value;` and implicit-return tail expression; statement
  vs expression rule. (Lessons 008, 020, 021, 024, 025, 036.)
- Tuples: `()` unit; `(T1, T2, ...)` non-zero arity + `pair.0`/`pair.1`
  field access; `let (a, b) = pair;` destructure. (Lessons 029, 072,
  073.)
- Comments: `//` line comments. (Lesson 010.)
- `use` declarations and qualified `Type::method` paths; `String::new`;
  method-call syntax with `&self` receiver. (Lessons 040, 041, 042, 043,
  044, 049.)
- Result / Ordering enums and `match` on payload variants. (Lessons 051,
  052, 053, 058, 061.)
- I/O: `std::io::stdin().read_line(&mut buf)`. (Lessons 050, 054.)
- `String::trim`, `str::parse::<i32>` chained with `.expect`. (Lessons
  055, 056, 053.)
- Diagnostic anatomy + `--explain` lookup + `warning:` vs `error:`
  category. (Lessons 003, 069, 070.)

### Ready now

(All ready-now items closed. See *Closed since the original pass* below.)

### Requires prerequisites

(Empty after the audit. Items I, L, P depend on other queue items but
each prereq is itself ready-now in this queue, so all dependents become
ready-now after the prereq lands. They appear under `Ready now` with
the prereq named in their entry.)

### Out of scope

- Ch1-1 install steps: `curl ... | sh`, `xcode-select --install`,
  Visual Studio install — host-OS sprawl. Run targets a learner who
  already has `rustc` on PATH.
- Linker / `%PATH%` troubleshooting beyond `rustc --version` —
  os-specific sprawl.
- `rustup self uninstall` — administrative; out of scope.
- Cargo's git init / `.gitignore` / `--vcs=git` flag — not load-bearing
  for compilation; minor.
- Ch3-2 IEEE-754 / two's complement representation as a typed concept —
  named in the Book but not load-bearing for any quiz-grade
  compile-and-run question.
- Ch3-2 *Numeric Operations* code block (`5 + 10`, `95.5 - 4.3`, `4 *
  30`, `56.7 / 32.2`, `-5 / 3`) — composes already-installed lessons;
  no new mechanism. Operationally covered piecewise.
- Ch3-3 *Statements and Expressions* `let x = (let y = 6);` E0658-style
  fail demo — covered by lesson 024's `;` rule plus 005's `let` shape;
  the specific demo is not load-bearing.
- Ch3-5 *Returning Values from Loops* `loop { break value; }` — already
  lesson 028.
- Ch3-5 *Streamlining Conditional Loops with while* — already lesson
  017.
- Ch1-2 *Project Directory Setup* `mkdir`, `cd` — ordinary computer-use
  per lesson 001.
- Ch1-2 PowerShell / cmd.exe shell distinctions — Linux/macOS only per
  lesson 001's assumption.
- Ch1-3 `cargo init` (in-place project init) — minor Cargo subcommand;
  out of scope unless a later move surfaces a need.
- Reserved keywords list (Ch3-0 *Keywords*) — minor; out of scope as a
  centered move. Book defers to Appendix A.
- *Statically typed* as a meta-claim about Rust — implicit since
  lesson 019; not a centered move.
(`Prelude`, `must-use` Result warnings, and `rustup update` were
initially placed here but reclassified to ready-now during the
2026-05-07 queue-correction pass after re-auditing against the Book's
explicit Ch1-2 coverage; see items U, V, and M above.)

## Status legend

- `already covered`: installed by a named prior lesson.
- `ready now`: orchestrator may dispatch a normal cycle.
- `requires prerequisites`: blocked on a named earlier install in this
  queue.
- `out of scope`: not closed in this run.

## Closed since the original pass

- A (`char` type with single-quoted literal and `: char` annotation
  slot, contrast against double-quoted string literals) — installed by
  lesson 074 (`074-char-type`).
- B (`const NAME: TYPE = value;` and the five-fact difference list from
  `let`: keyword change, always immutable, type annotation required,
  constant expression required, scope-flexible incl. global; plus the
  SCREAMING_SNAKE_CASE convention) — installed by lesson 075
  (`075-const-declaration`).
- C (array literal `[v1, v2, ...]`, type `[T; N]`, repeat-init `[v; N]`,
  fixed-length, homogeneous; `.len()` as operational witness with `usize`
  glossed-not-centered) — installed by lesson 076
  (`076-array-literal-and-type`).
- D (array element access `a[i]` co-installed with `usize` as a centered
  typed name; zero-based indices; type-load-bearing for variable indices
  via Probe 2's E0277) — installed by lesson 077
  (`077-array-indexing-and-usize`).
- E (array out-of-bounds runtime panic; `a[i]` with `i >= a.len()` is
  bounds-checked at runtime, panicking with the shape `index out of
  bounds: the len is N but the index is M`; runtime-vs-compile-time
  split named) — installed by lesson 078
  (`078-array-out-of-bounds-panic`).
- F (`for element in array { ... }` iteration as the safer alternative
  to manual `while index < a.len()` indexing; Book Ch3-5 Listings 3-4
  vs 3-5 contrast captured side-by-side) — installed by lesson 079
  (`079-for-over-array`); closes the array arc (C, D, E, F).
- G (twelve-name integer type family `i8/u8/i16/u16/i32/u32/i64/u64/
  i128/u128/isize/usize` with sign × width axes; range-from-bit-width
  rule witnessed by `let too_big: u8 = 256;` firing
  `overflowing_literals` with `0..=255` note) — installed by lesson
  080 (`080-integer-type-family`).
- H (five non-decimal integer-literal forms — hex `0xff`, octal `0o77`,
  binary `0b1111_0000`, type-suffix `57u8`, byte literal `b'A'` — plus
  the `_` numeric separator and the integer-literal default `i32` still
  applying; Book Ch3-2 Table 3-2 covered as one notational set;
  ASCII-only rule witnessed by `b'ℤ'` firing
  `error: non-ASCII character in byte literal`) — installed by lesson
  081 (`081-integer-literal-forms`) after one redteam-revision round.
- J (`cargo build --release` plus the `target/release/<name>` path; the
  `Finished` line shape ``Finished `release` profile [optimized] target(s)
  in <time>s`` contrasted against cycle 064's
  ``Finished `dev` profile [unoptimized + debuginfo]``; two parallel
  binaries from one source witnessed by editing `src/main.rs` and
  rebuilding only `--release` — release rebuilt, debug untouched; the
  Book's iteration-vs-shipping framing for the two profiles, plus the
  benchmarking rule that timed measurements should run from
  `target/release/`) — installed by lesson 082
  (`082-cargo-build-release`).
- I (integer overflow at runtime — debug-mode panic with `attempt to
  <op> with overflow` headline reusing the lesson 053/078 panic trailer
  shape vs release-mode two's-complement wrap, witnessed empirically
  by `let mut x: u8 = 255; x = x + 1;` in both profiles — debug exits
  101, release prints `x = 0` exit 0; the four standard-library method
  families `wrapping_*`/`checked_*`/`overflowing_*`/`saturating_*`
  named at Book level only, no method-call mechanics; the Book's rule
  that relying on the wrap is "considered an error" stated as Book
  policy) — installed by lesson 083 (`083-integer-overflow`).
- K (`cargo check` typecheck-only Cargo verb; the leading work line is
  ``Checking <name> v0.1.0 (...)`` instead of cycle 064's
  ``Compiling ...``, the `Finished` line shape is unchanged, no
  executable lands at `target/debug/<name>`; the Book's
  iteration-then-`cargo build` workflow framing) — installed by lesson
  084 (`084-cargo-check`).
- M (toolchain housekeeping — three commands as one operational
  concept: `rustc --version` printing `rustc x.y.z (abcabcabc
  yyyy-mm-dd)` per Book Ch1-1 line 83, `cargo --version` matching the
  shape, `rustup update` re-running rustup against the installed
  stable channel; "from any directory in any shell, no project, no
  source file"; rustup as the program that installed Rust on this
  host; `rustup self uninstall` named only as out-of-scope) —
  installed by lesson 085 (`085-toolchain-housekeeping`). No
  `observations/` package; verbatim transcripts live in the evidence
  appendix because no source file is consulted.
- N (`rustup doc` opens the local Rust documentation in the default
  browser; the local doc set was installed alongside `rustc` and
  `cargo` by rustup, includes the Book + Reference + std API docs,
  and is readable offline; the browser tab is the actual witness so
  the lesson uses three substitutes — the one-line `Opening docs in
  your browser` stdout, `rustup doc --path` resolving the on-disk
  index file, and `ls` + `<title>` extraction on the doc-set HTML —
  to ground the centered claim) — installed by lesson 086
  (`086-rustup-doc`). `cargo doc --open` — originally listed alongside
  `rustup doc` in the queue note — was deferred to a future move
  (Ch14-2 territory; project-scoped docs are a different command and
  out of Ch1-3 scope; the queue's only Ch1 source citation was
  Ch1-1 lines 130-138 which only names `rustup doc`).
- T (`cargo fmt` rewrites the package's Rust source files in place to
  the community standard style; the rewrite is *style-only* — no
  tokens added or removed, behavior unchanged, witnessed by
  bit-identical `x = 42` exit-0 stdout before and after on a probe
  whose `src/main.rs` was deliberately written with no spaces around
  `=` or `,` and everything on two lines; `rustfmt` and `cargo-fmt`
  are bundled with rustup-installed Rust, same toolchain framing as
  lesson 085; the standard style's *rule list* is deferred) —
  installed by lesson 087 (`087-rustfmt`).
- O (`f32` as the second floating-point primitive — 32 bits to `f64`'s
  64; both signed; unsuffixed-float-literal default still `f64` per
  lesson 033; the `: f32` annotation slot accepts a float literal the
  same way `: f64` does, witnessed by `let small: f32 = 3.0;` plus the
  contrast `let small: f32 = 3;` firing E0308 with `help: use a float
  literal` source-diff bit-for-bit identical to lesson 033's `f64 = 3`
  shape; IEEE-754 named, mechanics deferred) — installed by lesson
  088 (`088-f32-floating-point`).
- R + S (Book Ch3-3 *Functions* lines 9-36 conventions, folded into one
  move per the queue note's worker-discretion authorization: (a)
  snake_case is the conventional naming style for function AND variable
  names — `another_function`, `my_count` are snake_case; CamelCase
  fires the `non_snake_case` warn-by-default lint with `help: convert
  the identifier to snake case` source-diff; convention vs requirement;
  (b) function definition *order* is free — `another_function` defined
  AFTER `main` is callable from inside `main`, witnessed by the Book's
  verbatim two-function example; closes lesson 008's explicit
  definition-order deferral) — installed by lesson 089
  (`089-ch3-3-function-conventions`).
- P (loop labels — `'name:` prefix on any of the three loop forms
  (`loop`/`while`/`for`) plus `break 'name;` / `continue 'name;` to
  target the labeled loop instead of the innermost; bare `break;` and
  `continue;` from lessons 027/035 retain their innermost-only meaning;
  single-quote prefix is the syntactic discriminator, witnessed by
  `outer:` (no `'`) firing `error: malformed loop label`; auxiliary
  `warning: unused label` from a bare-`break;` form on a labeled loop
  read with lesson 069's category map) — installed by lesson 090
  (`090-loop-labels`) after one redteam-revision round (the cleanest-fix
  trim of a non-load-bearing scoping-analogy clause).
- Q (range reversal — `(start..end).rev()` walks the range in
  descending order, witnessed by the Book's verbatim countdown
  `for number in (1..4).rev() { println!("{}!", number); } println!
  ("LIFTOFF!!!");` printing `3!`, `2!`, `1!`, `LIFTOFF!!!`; parens
  syntactically required, witnessed by the no-parens contrast
  `for number in 1..4.rev()` firing `error[E0689]: can't call method
  \`rev\` on type \`{integer}\`` with the on-the-nose `help: you must
  surround the range in parentheses to call its \`rev\` function`
  source-diff; trait machinery `Iterator`/`DoubleEndedIterator`
  deferred per the Book's "another method we've not yet talked
  about" framing) — installed by lesson 091
  (`091-range-reversal-rev`).
- L (`Cargo.lock` as Cargo's reproducible-build record — the file
  Cargo creates at the package root after the first `cargo build`/
  `cargo run`/`cargo check` to record exact resolved versions of
  every dependency; future builds reuse those versions; even
  no-dependency packages get a sparse 7-line lockfile with a single
  `[[package]]` block; Cargo-managed via the auto-generated header
  banner — do not hand-edit; for binary applications the Book
  recommends checking it into source control; reproducibility claim
  carries through SemVer caret resolution per lesson 065 — host probe
  locked `rand v0.8.6` from `"0.8.5"` and the rule is unchanged) —
  installed by lesson 092 (`092-cargo-lock-reproducibility`) after
  one redteam-revision round (added the inline rand-version-mismatch
  acknowledgment matching lesson 065's pattern).
- U (the standard library *prelude* as a named language feature —
  "a collection of names that are automatically brought into scope
  of every module in a crate" per Reference *Preludes* line 8; the
  binary rule for standard-library names: in the prelude (write
  bare — `String`, `Vec`, `Option`/`Some`/`None`, `Result`/`Ok`/
  `Err`) or not (use full path per lesson 050, or `use` per lesson
  044); `cargo new`'s 2024-edition default selects `std::prelude::
  rust_2024` (empirically witnessed on this host); lesson 052's
  in-passing prelude framing now centered as its own concept;
  contrast probe `let m = HashMap::new();` fires E0433 with the
  on-the-nose `help: consider importing this struct: use std::
  collections::HashMap;` literally suggesting the lesson 044
  mechanism) — installed by lesson 093 (`093-standard-library-
  prelude`) after one redteam-revision round (added empirical
  edition-default probe transcript and fixed `Box`-at-`std::alloc`
  path to `std::boxed`).
- V (`unused_must_use` warn-by-default lint fires when a function
  returning `Result<T, E>` is called as an expression statement and
  the value is discarded; warning category per lesson 069 — program
  compiles, exit 0, executable produced; the diagnostic includes the
  on-the-nose `help: use \`let _ = ...\` to ignore the resulting
  value` source-diff suggesting the documented escape hatch; the
  silenced contrast probe `let _ = io::stdin().read_line(&mut buf);`
  compiles silently; auxiliary negative probe `fn maybe() ->
  Option<i32> { Some(7) } fn main() { maybe(); }` compiles silently
  too — `Option<T>` is NOT `#[must_use]`, so the lint is bound to
  `#[must_use]` types specifically; today installs only the exact
  shape `let _ = expr;` for the discard case, NOT `_` as a centered
  pattern token; third specific warn-by-default lint named in this
  run after lesson 069's `unused_variables` and lesson 089's
  `non_snake_case`) — installed by lesson 094
  (`094-unused-must-use-result`) after one orchestrator-trim round
  (1208 → 1176 words, surgical removal of redundant phrasing in
  Mental Model Delta + Check Yourself + What To Ignore).

## Pass complete

The Ch1-3 closure pass is complete as of 2026-05-07. Total accepted
nodes: 94. Items closed during this pass (in order): A (074), B (075),
C (076), D (077), E (078), F (079), G (080), H (081), J (082), I
(083), K (084), M (085), N (086), T (087), O (088), R+S (089), P
(090), Q (091), L (092), U (093), V (094) — 21 items, 21 lessons (R
and S folded into one lesson per the queue's worker-discretion
authorization). The Q07/trait frontier may now open per the
orchestrator's standing instruction.
