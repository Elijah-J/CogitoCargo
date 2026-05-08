---
id: 030-match-on-bool
move: "write `match value { pattern => arm_value, pattern => arm_value }` to compare a value against a list of patterns and produce the value of the matching arm; the smallest useful surface is `match` on a `bool`, with `true` and `false` as the two arms"
main_concept: "`match` is a Rust expression that takes a *scrutinee* (the value being matched against) and a list of *arms*, each of the form `pattern => arm_expression`, separated by `,`; whichever arm's pattern matches the scrutinee, that arm's expression is evaluated and becomes the value of the whole `match`; all arm expressions must have the same type; **exhaustiveness is mandatory** — rustc requires every possible value of the scrutinee's type to be covered by some arm, and emits a new E-code, **E0004 non-exhaustive patterns**, naming exactly which pattern is missing if one is not covered; on a `bool` scrutinee, that means both `true` and `false` must appear as arm patterns; this is the *fifth surface* on which lesson 024's tail-expression rule applies (after let-block, function body, if/else arms, loop+break) — but with new syntax (`=>` followed by a bare expression with no trailing `;`) and exhaustiveness enforced"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 012-bool-literals
  - 014-if-else
  - 019-type-annotation-i32
  - 024-statement-vs-expression
  - 026-if-as-expression
  - 029-unit-type
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "match with integer-literal patterns and `_` wildcard" moves
  - future "the `_` wildcard pattern in `let` and `match`" moves
  - future "match arms with multi-statement blocks `pattern => { ...; tail }`" moves
  - future "match guards `pattern if condition => arm`" moves
  - future "or-patterns `p1 | p2 => arm`" moves
  - future "range patterns `1..=5 => arm`" moves
  - future "if let / while let" moves
  - future "match against enum variants" moves
sources:
  - output/docs/rust/reference/expressions/match-expr.md
  - output/docs/rust/book/ch06-02-match.md
  - output/docs/rust/error_codes/E0004.md
  - output/docs/rust/reference/patterns.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/030-match-on-bool.rs
  - broken-contrast probe transcript inline in lesson `## Evidence` (not committed)
status: accepted
---

# `match` on a `bool` — the smallest useful pattern match

## The Move

On the right of a `let`, write a whole `match` form. The form takes a
value (the *scrutinee*), then a list of *arms* in `{ ... }`. Each arm
has the shape `pattern => arm_expression,`. Whichever arm's pattern
matches the scrutinee, that arm's expression becomes the value of the
whole `match`.

The smallest useful surface is `match` on a `bool`. The two boolean
values are `true` and `false` (lesson 012); both must appear as
patterns. So:

```rust
let result: i32 = match cond {
    true => 100,
    false => -100,
};
```

binds `result` to `100` when `cond` is `true`, and to `-100` when
`cond` is `false`. The whole `match` is one expression, like `5` or
`a + b` — it sits where any value of type `i32` would.

## Mental Model Delta

- Before: "`if`/`else` (lessons 014, 026) is how I pick between two
  values based on a boolean. It is the only branching shape I know."
- After: "`match value { pattern => v, pattern => v }` does the same
  job — compare the scrutinee against a list of patterns in order,
  produce the matching arm's value. For a `bool` it needs exactly two
  arms (`true =>` and `false =>`); forgetting one fires the new
  E-code **E0004 non-exhaustive patterns**, naming exactly which
  pattern is missing. The arms-must-share-type rule from lesson 026
  also applies. This is a fifth surface for lesson 024's
  tail-expression rule, with new syntax (`=>` plus a bare expression,
  no `;`) and a new constraint (every scrutinee value must be
  covered)."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; `fn main`'s body
    runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt + optional `note:` / `help:`
    sub-lines. The broken-contrast walk decodes a brand-new E-code,
    **E0004**, with exactly this skill.
  - Lesson 005 (load-bearing): `let name: TYPE = value;`. Reused as
    the slot the `match`'s value lands in.
  - Lesson 012 (load-bearing): `true` and `false` are Rust's two
    boolean values. Reused here as arm *patterns*: a literal value
    works directly as a pattern that matches that exact value.
  - Lesson 014 (precursor): `if`/`else` runs one of two blocks on a
    boolean. This lesson is `match` doing the same job for `bool`
    with different syntax plus exhaustiveness.
  - Lesson 019: `name: TYPE` attaches a type; used as
    `let result: i32 = match ...;`.
  - Lesson 024 (load-bearing): block-as-expression rule (a tail
    expression with no `;` is the block's value). Match arms reuse
    that rule with new syntax: `=>` plus a bare expression.
  - Lesson 026 (precursor): `if`/`else` as a value-producing
    expression with the arms-must-share-type rule. `match` installs
    the same rule on a new surface.
  - Lesson 029: named `()`. Match arms can produce `()`, but the
    probe's arms produce `i32` because the binding is `let result:
    i32 = ...;`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let cond: bool = true;
    let result: i32 = match cond {
        true => 100,
        false => -100,
    };
    println!("result = {result}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
result = 100
```

Walk:

- `match cond { ... }` is a `match` expression. The *scrutinee* is
  `cond`. Inside `{ ... }` are two arms of the form `pattern =>
  expression,`. Patterns: `true` and `false`. Arm expressions: `100`
  and `-100`.
- rustc compares `cond` (which is `true`) to the patterns
  top-to-bottom. The first arm's pattern matches; its expression
  `100` becomes the value of the whole `match`.
- `let result: i32 = match ...;` binds `result` to `100`. `println!`
  prints `result = 100`.

Now the contrast. Save `broken.rs` — same as `demo.rs` with the
`false => -100,` arm removed:

```rust
fn main() {
    let cond: bool = true;
    let result: i32 = match cond {
        true => 100,
    };
    println!("result = {result}");
}
```

Compile it. The full transcript lives in `## Evidence`; reading it
with lesson 003's order:

- *Headline*: `error[E0004]: non-exhaustive patterns: \`false\` not
  covered`. **A brand-new E-code** — not E0308. The headline names
  the missing pattern.
- *`-->` location*: `broken.rs:3:29` — the scrutinee `cond`. rustc
  enforces exhaustiveness against the scrutinee's type, so the
  location is the scrutinee, not the missing arm.
- *Source excerpt*: `^^^^` underlines `cond`, sub-line `pattern
  \`false\` not covered`.
- *`note:` line*: `the matched value is of type \`bool\``. rustc names
  the type whose patterns must be covered.
- *`help:` block*: a source-diff with `~` markers suggesting `false
  => todo!(),`. `todo!()` is a rustc placeholder macro meaning "fill
  this in", not literal advice. The real fix is `false => -100,`,
  which `demo.rs` already has.

Contrast with lessons 014/026: `if`/`else` on a `bool` always covers
both branches by construction (the `else` catches `false`). `match`
does not — you list every pattern explicitly. rustc refuses to compile
if any pattern is missing.

## What Changed

- You can write `match value { pattern => v, pattern => v }` on the
  right of `let` and bind the result. For a `bool` scrutinee, the two
  arms are `true => ...` and `false => ...`.
- You have a name for the value being matched: the *scrutinee*. And a
  name for `pattern => expression`: an *arm*. Arms separated by `,`.
  Each arm's expression is a bare value with no trailing `;` — the
  same shape as a block's tail expression in lesson 024. `match` is
  the *fifth surface* of that rule.
- rustc enforces *exhaustiveness*. Missing a case fires the new
  E-code **E0004 non-exhaustive patterns**, with a headline naming
  the missing pattern, a `-->` pointing at the scrutinee, a `note:`
  naming the scrutinee's type, and a `help:` source-diff suggestion.
- The arms-must-share-type rule from lesson 026 still applies. Both
  arms here produce integers and the binding is annotated `: i32`.
- `if`/`else` and `match` overlap on `bool`; `match` shines when the
  scrutinee has more than two cases (deferred) or is something other
  than `bool` (deferred).

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let flag: bool = false;
    let n: i32 = match flag {
        false => 0,
        true => 1,
    };
    println!("n = {n}");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) The arms are in opposite order from the probe (`false` first).
Does the order matter for the result?

(c) If you delete the `true => 1,` arm and recompile, what E-code does
rustc emit, and which pattern does it name as not covered?

(Answers: (a) Yes; prints `n = 0`. (b) No: `match` checks patterns in
source order, but each `bool` value matches exactly one of `true` /
`false`, so order does not change the result here. Order *can* matter
for overlapping patterns in general — the Reference says "the first
arm with a matching pattern is chosen" — but two non-overlapping
literal patterns like these are order-insensitive. (c) `error[E0004]:
non-exhaustive patterns: \`true\` not covered`, with `note: the
matched value is of type \`bool\``.)

## What To Ignore For Now

- *Integer-literal patterns* like `match n { 1 => 10, 2 => 20, _ => 99
  }`. Integer types are unbounded, so exhaustiveness needs a `_`
  wildcard. Future move.
- *The `_` wildcard pattern* — appears in this lesson's E0004
  `help:` line and in the Reference example as a "match anything"
  catch-all. Glossed only; future move.
- *`todo!()`* — the macro rustc's `help:` uses as a placeholder. Not
  a real fix; not installed.
- *Match arms with blocks* like `pattern => { ...; tail_expr }`.
  Allowed by the grammar; the cited Book section shows this form. The
  probe uses bare arm expressions; multi-statement arm blocks are
  deferred.
- *Match against enum variants* — the canonical idiomatic use of
  `match` (the Book introduces it against `Coin::Penny` etc.). Defers
  until enums are installed.
- *Match guards* (`pattern if condition => arm`), *`if let` /
  `while let`*, *or-patterns* (`p1 | p2 => arm`), *range patterns*
  (`1..=5 => arm`), *bindings inside patterns* (`Some(n) => n + 1`),
  *reference patterns* (`&n => ...`, `ref n => ...`). All future
  moves.
- All previously deferred items.

## Evidence

### Sources

Four corpus sources support the lesson's substantive claims.

- `output/docs/rust/reference/expressions/match-expr.md`. The grammar
  block (lines 10-22) names the pieces:

  > MatchExpression → match Scrutinee { ... MatchArms? }
  >
  > MatchArms → ( MatchArm => ( ExpressionWithoutBlock , |
  > ExpressionWithBlock ,? ) )* MatchArm => Expression ,?
  >
  > MatchArm → OuterAttribute* Pattern MatchArmGuard?

  Plain-English translation, used in the lesson: `match` keyword, then
  a *scrutinee*, then a `{ ... }` containing a list of *arms*; each
  arm is `pattern => expression`; arms are separated by `,`; the final
  arm's `,` is optional. Lines 50, 54, 58 add prose:

  > A `match` expression branches on a pattern. ... A `match` expression
  > has a *scrutinee expression*, which is the value to compare to the
  > patterns. ... The scrutinee expression and the patterns must have
  > the same type.

  Line 66 adds:

  > the resulting value is sequentially compared to the patterns in
  > the arms until a match is found. The first arm with a matching
  > pattern is chosen as the branch target of the `match`...

  Line 138 (the type rule):

  > The type of the overall `match` expression is the least upper
  > bound of the individual match arms.

  This last sentence is the Reference's version of the
  arms-must-share-type rule that lesson 026 installed for `if`/`else`.
  In `bool`/`i32` arm cases like the probe, "least upper bound" is
  just "they are both `i32`, so the `match` is `i32`."

- `output/docs/rust/book/ch06-02-match.md`. The Book's chapter-opening
  paragraph (lines 6-13) is the canonical pedagogical statement:

  > Rust has an extremely powerful control flow construct called
  > `match` that allows you to compare a value against a series of
  > patterns and then execute code based on which pattern matches.
  > ... The power of `match` comes from the expressiveness of the
  > patterns and the fact that the compiler confirms that all
  > possible cases are handled.

  Lines 55-58 name the parts:

  > Next are the `match` arms. An arm has two parts: a pattern and
  > some code. The first arm here has a pattern that is the value
  > `Coin::Penny` and then the `=>` operator that separates the
  > pattern and the code to run. ... Each arm is separated from the
  > next with a comma.

  Lines 60-64 describe the matching semantics:

  > When the `match` expression executes, it compares the resultant
  > value against the pattern of each arm, in order. If a pattern
  > matches the value, the code associated with that pattern is
  > executed. If that pattern doesn't match the value, execution
  > continues to the next arm...

  Lines 66-68 give the value-producing rule:

  > The code associated with each arm is an expression, and the
  > resultant value of the expression in the matching arm is the
  > value that gets returned for the entire `match` expression.

  Calibration: the Book introduces `match` against an enum type
  (`Coin::Penny`, `Coin::Nickel`, `Coin::Dime`, `Coin::Quarter`),
  whose four variants exhaustively cover the type. This lesson
  installs `match` against `bool`, whose two variants (`true`,
  `false`) exhaustively cover *that* type. The principle is identical
  — "compiler confirms that all possible cases are handled" — but the
  scrutinee surface is restricted. Enum scrutinees are deferred.

- `output/docs/rust/error_codes/E0004.md`, lines 4-7:

  > This error indicates that the compiler cannot guarantee a matching
  > pattern for one or more possible inputs to a match expression.
  > Guaranteed matches are required in order to assign values to match
  > expressions, or alternatively, determine the flow of execution.

  And lines 28-31:

  > If you encounter this error you must alter your patterns so that
  > every possible value of the input type is matched. ... the
  > underscore `_` wildcard pattern can be added after all other
  > patterns to match "anything else".

  This is the canonical Rust statement of *why* exhaustiveness is
  enforced. The `_` wildcard suggestion is glossed in `## What To
  Ignore For Now`; this lesson installs the explicit `true`/`false`
  pair, not the wildcard.

- `output/docs/rust/reference/patterns.md`, line 168 (the Literal
  Patterns subsection's licensing sentence):

  > *Literal patterns* match exactly the same value as what is created
  > by the literal.

  This is the canonical Reference statement that grounds the
  `## Prerequisites` claim about lesson 012: a literal value (here
  `true` or `false`) works directly as a pattern that matches that
  exact value. The Reference is the authoritative source for pattern
  semantics; the Book chapter ch06-02 says the same thing in passing
  but the Reference's Literal Patterns subsection is the licensing
  text.

Calibration:
- The Reference grammar uses metasyntactic terms (Scrutinee,
  MatchArms, Pattern, etc.) that are dense for beginners. The lesson
  uses plain English ("the value being matched", "arms", "patterns")
  and cites the Reference as the authoritative grammar without
  quoting the dense parts.
- The Book's example uses `cargo` and an `enum`. This lesson uses
  `rustc demo.rs` per lesson 001 and a `bool` scrutinee per lesson
  012. The mechanics are identical.
- "Least upper bound" appears in the Reference but is not installed
  here. In the probe both arms produce `i32`, so the join is just
  `i32`. The lesson restates the rule informally as "all arm
  expressions must have the same type" — which is the same load-bearing
  fact lesson 026 installed for `if`/`else`.

### Probes

Two probes captured on rustc 1.95.0 (59807616e 2026-04-14) on Darwin
x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/030-match-on-bool.rs`.
The broken-contrast probe is *not* committed under `observations/`;
its transcript is reproduced verbatim below.

Both probes were run in temp directories created with `mktemp -d`
and removed at the end.

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
    let cond: bool = true;
    let result: i32 = match cond {
        true => 100,
        false => -100,
    };
    println!("result = {result}");
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
result = 100
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 silently. No warnings.
- The output `result = 100` is the load-bearing observation: the
  scrutinee `cond` was `true`, so the first arm's pattern (`true`)
  matched, the first arm's expression (`100`) was evaluated, and the
  whole `match` produced `100`, which `let result: i32 = ...;` bound
  to `result`. `println!` then printed `100` for the `{result}` slot.
- The annotation `: i32` on the binding is what pins the `match`'s
  type. rustc would also infer `i32` here from the integer literal
  arms (default integer type), but the annotation makes the type-check
  verifiable.
- The trailing `,` after `false => -100,` is allowed (the grammar
  says the final arm's `,` is optional). The probe writes it for
  consistency.

#### Broken-contrast probe

The probe `broken.rs` exists for the broken-contrast walk; the
transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let cond: bool = true;
    let result: i32 = match cond {
        true => 100,
    };
    println!("result = {result}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0004]: non-exhaustive patterns: `false` not covered
 --> broken.rs:3:29
  |
3 |     let result: i32 = match cond {
  |                             ^^^^ pattern `false` not covered
  |
  = note: the matched value is of type `bool`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~         true => 100,
5 ~         false => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0004]: non-exhaustive patterns: \`false\` not
  covered` is a **new E-code for this run** — different from E0308
  (lessons 024, 025, 026, 028, 029), E0425 (lessons 005, 008), E0384
  (lessons 006, 007), E0601 (lesson 002). The trailer `For more
  information about this error, try \`rustc --explain E0004\`.` is
  the standard lesson-003 form.
- The `--> broken.rs:3:29` location points at column 29 of line 3,
  which is the scrutinee `cond`. rustc enforces exhaustiveness
  against the *scrutinee's type*, so the location is the scrutinee,
  not the missing arm.
- The source excerpt underlines `cond` with `^^^^` and adds the
  sub-line `pattern \`false\` not covered` — naming exactly which
  pattern is missing.
- The `= note: the matched value is of type \`bool\`` line names the
  scrutinee's type. rustc tells the learner *what type* must be
  covered, not just that something is missing. For `bool`, the two
  patterns to cover are `true` and `false`; the working probe lists
  both.
- The `help:` block shows a literal source-diff with `~` markers
  indicating modified lines (different from the `+` markers some
  rustc suggestions use). The diff suggests `false => todo!(),` as
  the new arm. `todo!()` is a rustc placeholder macro — it is what
  rustc puts in source-diff suggestions when it does not know what
  the user wants there. It is *not* installed by this lesson; the
  real fix is the working probe's `false => -100,`.
- Exit code: 1. No executable produced.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline + `-->` location + source excerpt with caret +
  optional `note:` / `help:` sub-lines. The broken-contrast walk
  decodes E0004 with exactly this skill.
- `005-let-binding` (accepted, load-bearing) — `let name: TYPE =
  value;` binds a name; reused as the slot the `match`'s value lands
  in.
- `012-bool-literals` (accepted, load-bearing) — `true`/`false` are
  Rust's two boolean values. Reused here as arm *patterns* (a literal
  value works directly as a pattern that matches that exact value).
- `014-if-else` (accepted, precursor) — `if`/`else` runs one of two
  blocks based on a boolean. This lesson's `match` does the same job
  for `bool` with different syntax, plus exhaustiveness enforcement.
- `019-type-annotation-i32` (accepted) — `name: TYPE` attaches a
  type; used here as `let result: i32 = match ...;`.
- `024-statement-vs-expression` (accepted, load-bearing) —
  block-as-expression rule. **`match` is the fifth surface this rule
  applies to** (let-block, function body, if/else arms, loop+break,
  `match`).
- `026-if-as-expression` (accepted, precursor) — installed `if`/`else`
  as a value-producing expression with the arms-must-share-type rule.
  This lesson installs `match` with the same arms-must-share-type
  rule, on a fifth surface.
- `029-unit-type` (accepted) — named `()`. Match arms can produce
  `()` (a `match` used as a statement with side-effecting arms), but
  this lesson's probe binds with `let`, so the arms produce `i32`.
