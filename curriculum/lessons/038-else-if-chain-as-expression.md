---
id: 038-else-if-chain-as-expression
move: "put a whole `if cond_a { ... } else if cond_b { ... } else if cond_c { ... } else { ... }` chain on the right of `let` and bind to the value of whichever arm's tail expression runs first"
main_concept: "lesson 016's multi-arm `if`/`else if`/`else` chain is *also* a value-producing expression: combining lesson 016 (first matching condition wins) with lesson 026 (`if`/`else` is itself an expression on the right of `let`), the whole chain evaluates to the value of whichever arm's tail expression ran first; lesson 026's same-type-arms rule generalizes — *every* arm's tail expression must produce the same type — and the trailing `else` is mandatory for a value-producing chain because the no-block fallback is `()` (lesson 029)"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 005-let-binding
  - 013-comparison-operators
  - 014-if-else
  - 016-else-if-chain
  - 019-type-annotation-i32
  - 024-statement-vs-expression
  - 026-if-as-expression
  - 029-unit-type
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "if/else as expression in function body" moves
  - future "if without else evaluating to ()" moves
  - future "match as a cleaner alternative to long else-if chains" moves
  - future "match guards `pattern if condition => arm`" moves
  - future "if let / while let chains" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/expressions/if-expr.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/038-else-if-chain-as-expression.rs
status: accepted
---

# A multi-arm `if`/`else if`/`else` chain produces a value too

## The Move

Lesson 026 put a two-arm `if condition { value_a } else { value_b }` on
the right of `let`. Lesson 016 extended a two-arm `if`/`else` to a
multi-arm chain `if cond_a { ... } else if cond_b { ... } else { ... }`
as control flow. Combine the two: write a whole multi-arm chain on the
right of `let` and bind to the value of whichever arm's tail expression
ran first. So

```rust
let grade: i32 = if n >= 90 { 4 }
                 else if n >= 80 { 3 }
                 else if n >= 70 { 2 }
                 else { 1 };
```

binds `grade` to `4`, `3`, `2`, or `1` depending on `n`. No new keyword,
no new operator — just the combination of lessons 016 + 024 + 026 + 029.

## Mental Model Delta

- Before: "Lesson 016 gave me multi-arm `if`/`else if`/`else` chains as
  control flow. Lesson 026 gave me two-arm `if`/`else` as a value on
  the right of `let`. I can do *one* of the two — extend to multiple
  arms, or use the value — but I have not put them together."
- After: "A multi-arm chain is itself a single expression. The same
  rule from lesson 026 applies, just to more arms: whichever arm wins
  per lesson 016's first-match-wins rule, *its* tail expression's
  value becomes the chain's value. Every arm must produce the same
  type. The trailing `else` is no longer optional when I want a value
  out — without it, the chain's no-block fallback type is `()`
  (lesson 029) and that collides with any non-`()` annotation on the
  `let`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs`, `./name`, silent on success;
    `fn main`'s body runs when the executable launches.
  - Lesson 005 (load-bearing): `let name: TYPE = value;` binds a name;
    reused as the slot the chain's value lands in.
  - Lesson 013: comparisons like `n >= 70` produce booleans. The probe
    uses four such conditions.
  - Lesson 014: two-arm `if condition { ... } else { ... }`, the seed
    shape that lesson 016 extends and lesson 026 promotes to an
    expression.
  - Lesson 016 (load-bearing): `else if condition { ... }` arms;
    *first matching condition wins*; later arms are skipped, even
    their conditions are not evaluated. **This lesson takes 016's
    chain and uses it on the right of `let`.**
  - Lesson 019: `name: TYPE` attaches a type. Used as
    `let n: i32` and `let grade: i32`.
  - Lesson 024 (load-bearing): a block `{ ... }` is itself an
    expression; its value is its tail expression's value (no `;`).
    Each chain arm is one such block.
  - Lesson 026 (load-bearing): a two-arm `if`/`else` is itself an
    expression on the right of `let`; both arms must produce the same
    type, otherwise rustc emits `error[E0308]: \`if\` and \`else\`
    have incompatible types`. **This lesson generalizes 026's two-arm
    form to N arms.**
  - Lesson 029: the unit type `()` — the type Rust falls back to when
    a block has no value-producing tail. Used here only to explain
    why the trailing `else` is mandatory for a value-producing chain.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let n: i32 = 75;
    let grade: i32 = if n >= 90 {
        4
    } else if n >= 80 {
        3
    } else if n >= 70 {
        2
    } else {
        1
    };
    println!("n = {n}, grade = {grade}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 75, grade = 2
```

Walk it. `n` is `75`. The right-hand side of `let grade: i32 = ...;` is
a four-arm chain. Per lesson 016, conditions are checked in source
order:

- `75 >= 90` is `false`. Arm 1 is skipped.
- `75 >= 80` is `false`. Arm 2 is skipped.
- `75 >= 70` is `true`. Arm 3 runs. Its tail expression is `2` (no
  `;` — per lesson 024, the block's value is its tail expression).
- Arm 4 (the trailing `else`) is *not even checked* — lesson 016's
  first-match-wins rule.

The whole chain evaluates to `2`. The outer `let grade: i32` binds
`grade` to `2`. `println!` prints `n = 75, grade = 2`.

The Reference licenses both halves of this in one place. The formal
grammar of an `if` expression is recursive: an `else` arm can be
*either* a block *or* another `IfExpression`, so a chain
`if … else if … else if … else …` is one grammatical entity, not a
sequence of nested ifs. The Reference also states the value rule
directly: "An `if` expression evaluates to the same value as the
executed block, or `()` if no block is evaluated" — which is what
lesson 026 captured for two arms and what extends unchanged to N. And
the type rule: "An `if` expression must have the same type in all
situations." That covers all arms, not just the first two.

The Book frames the same value rule in chapter 3.5 ("Using `if` in a
`let` Statement") and the chain shape in chapter 3.5 ("Handling
Multiple Conditions with `else if`") as separate sections — it does
not give a worked multi-arm-chain-on-right-of-`let` example. This
lesson is the composition of the two.

Two things are worth checking at a glance in the probe:

1. Every arm's tail expression is an integer literal — `4`, `3`, `2`,
   `1`. All are the same type. Lesson 026's same-type-arms rule
   generalizes to "all four arms must agree." If you replaced `1`
   with `true`, rustc would emit the same specialized
   `error[E0308]: \`if\` and \`else\` have incompatible types`
   diagnostic captured in lesson 026, just naming a later arm.

2. The trailing `else { 1 }` is mandatory for a value-producing
   chain. Drop it, and the chain has *no* arm to run when every
   condition is `false`. Per the Reference rule above (and lesson
   029), the chain's value in that case is `()`. The `let grade:
   i32 = ...;` annotation expects `i32`, so rustc would reject the
   program with a generic E0308 `expected i32, found ()` — the same
   shape lesson 029 already calibrated. Defer that direct probe; the
   point is just to know why the final `else` is here.

## What Changed

- You can put a *multi-arm* `if`/`else if`/.../`else` chain on the
  right of `let` and bind to the value of whichever arm wins per
  lesson 016's first-match-wins rule.
- Lesson 026's "two arms must produce the same type" generalizes to
  "every arm in the chain must produce the same type." The same
  specialized E0308 headline applies if any arm disagrees.
- The trailing `else` is mandatory for a value-producing chain.
  Without it, the no-block fallback type is `()` (lesson 029),
  colliding with any non-`()` `let` annotation.
- No new mechanism: this lesson is purely the combination of lessons
  016 + 024 + 026 + 029. The Reference's recursive `if`-expression
  grammar (`else` may be a block *or* another `IfExpression`) is
  what makes the combination automatic — once 016 and 026 are
  installed, the chain-on-right-of-`let` form is a corollary.

## Check Yourself

You write `pick.rs` containing:

```rust
fn main() {
    let n: i32 = 42;
    let label: i32 = if n >= 100 {
        3
    } else if n >= 50 {
        2
    } else if n >= 0 {
        1
    } else {
        0
    };
    println!("label = {label}");
}
```

You run `rustc pick.rs && ./pick`.

(a) What does it print, and which arm produced the value?

(b) You change `let n: i32 = 42;` to `let n: i32 = -7;`. After
recompiling, what does `./pick` print?

(c) You drop the trailing `else { 0 }` arm entirely (delete those
three lines). What kind of error does rustc emit, and why? (Hint:
think about the chain's value when every condition is `false`, and
about the `: i32` annotation on the `let`.)

(Answers: (a) `label = 1`. `42 >= 100` is `false`; `42 >= 50` is
`false`; `42 >= 0` is `true`, so arm 3 runs, tail value `1`. Arm 4 is
not evaluated. (b) `label = 0`. `-7 >= 100` and `-7 >= 50` and
`-7 >= 0` are all `false`, so the trailing `else` runs; tail value
`0`. (c) `error[E0308]: mismatched types` with `expected i32, found
()`. Without the trailing `else`, when every condition is `false` the
chain has no block to run; per the Reference, it produces `()` (lesson
029). The `let label: i32` annotation expects `i32`. The two collide.)

## What To Ignore For Now

- *`if let` chains* — `if let Some(x) = opt { ... } else if let Some(y) = other { ... }`.
  Pattern matching on enums; the Reference's `LetChain` grammar in
  the same file. Future move (needs `Option`/enums).
- *`match` as a cleaner alternative* to long `else if` chains. Lesson
  030's note already cited the Book's own pointer: "Using too many
  `else if` expressions can clutter your code, ... Chapter 6
  describes a powerful Rust branching construct called `match` for
  these cases." For two-or-three-arm decisions a chain is fine; for
  many-arm value lookup the patterns of lesson 030/031 read more
  cleanly. Style choice; no new mechanism.
- *Match guards* — `pattern if condition => arm` inside `match`.
  Combines patterns with conditional logic; future move.
- *`if`-without-`else` evaluating to `()`* explicitly — only the
  diagnostic shape is named here. Future move.
- *Operator precedence* in conditions — comparison and arithmetic
  precedence rules. Future move.
- *Short-circuit evaluation* between conditions. Already implicit in
  lesson 016's first-match-wins rule (later conditions are not
  evaluated once an earlier one matches); not a new install.
- All previously deferred items.

## Evidence

### Sources

- `output/docs/rust/reference/expressions/if-expr.md`. Three
  load-bearing items:

  Lines 10-12, the formal grammar:

  > IfExpression →
  >     if Conditions BlockExpression
  >     ( else ( BlockExpression | IfExpression ) )?

  The recursive `IfExpression` on the right of `else` is what makes
  a chain `if A { ... } else if B { ... } else { ... }` one
  grammatical entity, not a sequence of separately-nested ifs.

  Line 58, `[[expr.if.result]]`:

  > An `if` expression evaluates to the same value as the executed
  > block, or `()` if no block is evaluated.

  This is the rule that grounds the value-producing behavior of any
  `if` expression — two arms or N — *and* grounds the "no block ran
  ⇒ value is `()`" claim that justifies the mandatory trailing
  `else` for a value-producing chain.

  Line 62, `[[expr.if.type]]`:

  > An `if` expression must have the same type in all situations.

  This is the same-type rule generalized: not "the two arms" but
  "all situations" — every arm.

- `output/docs/rust/book/ch03-05-control-flow.md`. Two relevant
  sections, both already cited in lessons 016 and 026:

  - "Handling Multiple Conditions with `else if`" (lines 135-178)
    introduces the chain shape (cited in lesson 016).

  - "Using `if` in a `let` Statement" (lines 180-256) puts a two-arm
    `if`/`else` on the right of `let` and states the same-type rule
    (cited in lesson 026).

  The Book treats these as two separate subsections and does not
  give a worked multi-arm-chain-on-right-of-`let` example. The
  Reference quotes above are what tie them together.

  Calibration:
  - The Book's "Using `if` in a `let` Statement" example uses a
    two-arm `if`/`else`, the same shape lesson 026 already
    captured. This lesson is the composition step beyond it.
  - The Book's "Handling Multiple Conditions with `else if`" example
    uses `%` (deferred until lesson 037) and is purely control-flow
    — its arms are `println!` calls, not value-producing tail
    expressions. The probe in this lesson uses simpler comparisons
    (`>=`) and integer-literal tail expressions.
  - Both the Book and the Reference build with `cargo run`; this
    lesson uses `rustc demo.rs` per lesson 001. Behavior is
    identical.
  - No broken-contrast probe is captured for this lesson. Two
    natural broken contrasts exist — (a) one arm with a different
    tail-expression type, (b) dropping the trailing `else` — but
    both are already exercised in prior lessons (a) in lesson 026's
    specialized E0308 walk and (b) in lesson 029's `expected (),
    found integer` calibration. The Check Yourself (c) prediction
    relies on those prior probes, not on a fresh capture.

### Probes

One probe was captured on rustc 1.95.0 (59807616e 2026-04-14) on
Darwin x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/038-else-if-chain-as-expression.rs`.
The probe was run in a temp directory created with `mktemp -d` and
removed at the end.

#### Working probe

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let n: i32 = 75;
    let grade: i32 = if n >= 90 {
        4
    } else if n >= 80 {
        3
    } else if n >= 70 {
        2
    } else {
        1
    };
    println!("n = {n}, grade = {grade}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
n = 75, grade = 2
exit=0
```

Notes:

- `rustc` exits 0 silently (consistent with lesson 001).
- The single output line is `n = 75, grade = 2`. The `2` is the value
  of arm 3's block `{ 2 }` — its tail expression is the literal `2`
  (no `;`), per lesson 024. That value reached the right of the outer
  `let grade: i32 = ...;` because the whole multi-arm chain is one
  expression — the load-bearing observation for this lesson, and the
  combination of lesson 016 (first-match-wins) with lesson 026 (`if`
  is an expression).
- All four arm tail expressions are `i32` integer literals; lesson
  026's same-type-arms rule (per the Reference's `[[expr.if.type]]`
  rule) is satisfied for all four.
- The trailing `else { 1 }` is present so the chain has a
  value-producing arm even when every earlier condition is `false`.
  Without it, per the Reference's `[[expr.if.result]]` rule, the
  chain's value when no block is evaluated is `()` (lesson 029),
  colliding with the `: i32` annotation. No separate broken-contrast
  probe was captured for this case — lesson 029's already-captured
  `expected (), found integer` diagnostic is the same diagnostic
  shape.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `005-let-binding` (accepted, load-bearing) — `let name: TYPE =
  value;` binds a name; reused as the slot the chain's value lands
  in.
- `013-comparison-operators` (accepted) — `n >= 70` produces a
  boolean; used as four conditions in the chain.
- `014-if-else` (accepted) — two-arm `if condition { ... } else { ... }`,
  the seed shape extended in lesson 016 and promoted to an
  expression in lesson 026.
- `016-else-if-chain` (accepted, load-bearing) — multi-arm chain
  with `else if` arms; first matching condition wins; later
  conditions are not evaluated. **This lesson takes 016's chain
  shape and uses it on the right of `let`.**
- `019-type-annotation-i32` (accepted) — `name: TYPE`. Used as
  `let n: i32` and `let grade: i32`.
- `024-statement-vs-expression` (accepted, load-bearing) — a block
  `{ ... }` is itself an expression; its value is its tail
  expression (no `;`). Each chain arm is one such block.
- `026-if-as-expression` (accepted, load-bearing) — a two-arm
  `if`/`else` on the right of `let`, with arms-must-share-type
  enforced by the specialized E0308 headline. **This lesson
  generalizes 026's two-arm form to N arms.**
- `029-unit-type` (accepted) — the unit type `()`; the type a block
  with no tail expression evaluates to. Used here only to explain
  why the trailing `else` is mandatory for a value-producing chain.
