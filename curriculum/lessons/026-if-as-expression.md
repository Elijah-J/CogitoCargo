---
id: 026-if-as-expression
move: "put `if condition { value_a } else { value_b }` on the right of `let` — `let category: i32 = if n > 5 { 100 } else { -100 };` — and bind `category` to whichever arm's value the condition selects"
main_concept: "lesson 014's `if`/`else` is itself an *expression*, not just a control-flow statement; each arm `{ ... }` is a block (lesson 024) whose value is its tail expression's value (no `;`); the whole `if`/`else` evaluates to the value of whichever arm ran, so it sits anywhere a value can — most directly on the right of `let`; the two arms must produce the same type, otherwise rustc emits the specialized headline `error[E0308]: ` plus the words `if` and `else` have incompatible types"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 009-arithmetic-on-integers
  - 012-bool-literals
  - 013-comparison-operators
  - 014-if-else
  - 019-type-annotation-i32
  - 024-statement-vs-expression
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "else if chains as expressions" moves
  - future "if/else as expression in function body" moves
  - future "if without else evaluating to ()" moves
  - future "match as expression" moves
  - future "loop with break value" moves
  - future "if let / while let" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/026-if-as-expression.rs
status: accepted
---

# An `if`/`else` produces a value; put it on the right of `let`

## The Move

On the right of a `let`, write a whole `if`/`else` form whose two arms
each end with one bare value — no `;` after that value. The condition
picks an arm; the arm's tail value becomes the value the `let` binds.
So

```rust
let category: i32 = if n > 5 { 100 } else { -100 };
```

binds `category` to `100` if `n > 5` is `true`, and to `-100` if it is
`false`. The whole `if`/`else` is one expression, like a literal or
`a + b` — it sits where any value of type `i32` would.

## Mental Model Delta

- Before: "`if condition { ... } else { ... }` runs one of two blocks
  (lesson 014). It is something the executable *does*. Separately, a
  block on the right of `let` produces a value via its tail expression
  (lesson 024)."
- After: "`if`/`else` is itself an expression. Each arm `{ ... }` is a
  block whose value is its tail expression (lesson 024). Whichever arm
  runs, its value becomes the value of the whole `if`/`else`. So the
  whole form fits on the right of `let`, exactly like `5` or `a + b`.
  Lesson 014 was the special case where I discarded that value; now I
  am keeping it. There is one new constraint: both arms must produce
  the same type, otherwise rustc emits a specialized E0308 headline."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs`, `./name`, silent on success;
    `fn main`'s body runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline + `-->`
    location + source excerpt + optional sub-annotations. The
    broken-contrast walkthrough below uses exactly this skill on a
    *specialized* E0308 headline.
  - Lesson 005 (load-bearing): `let name: TYPE = value;` binds a name;
    reused as the slot the `if`/`else`'s value lands in.
  - Lesson 009: integer values and the `+ - * /` operators; the literal
    arm values `100`, `-100`, and the binding `n`.
  - Lesson 012: `true`/`false` are Rust's two boolean values; the
    broken-contrast probe uses `true` directly, and rustc's diagnostic
    names the type `bool`.
  - Lesson 013 (load-bearing): `n > 5` produces a boolean; used as the
    `if`'s condition.
  - Lesson 014 (load-bearing): `if condition { ... } else { ... }`
    runs the first block on `true` and the second on `false`. **This
    lesson recasts that same shape as a value-producing expression.**
  - Lesson 019: `name: TYPE` attaches a type; used here as
    `let category: i32 = ...;`.
  - Lesson 024 (load-bearing): a block `{ ... }` is itself an
    expression; its value is the value of its tail expression (no `;`).
    **Each `if`/`else` arm is one such block.**
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let n: i32 = 7;
    let category: i32 = if n > 5 { 100 } else { -100 };
    println!("category = {category}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
category = 100
```

Walk it. `n` is `7`. `n > 5` is `true` (lesson 013). Per lesson 014,
the *then* arm runs and the `else` arm is skipped. The *then* arm is
the block `{ 100 }`; per lesson 024, its value is its tail expression
`100` (no `;`). The whole `if`/`else` evaluates to `100`. The outer
`let category: i32` binds `category` to `100`. `println!` prints
`category = 100`.

The Book licenses this directly, in the section "Using `if` in a `let`
Statement":

> Because `if` is an expression, we can use it on the right side of a
> `let` statement to assign the outcome to a variable

So the *whole* `if condition { value_a } else { value_b }` form is one
expression — like `5` or `a + b` — whose value depends on which arm
ran. The Book's own example (Listing 3-2) is `let number = if condition { 5 } else { 6 };`, the same shape with different values.

The Book also names the rule about arm types directly:

> the values that have the potential to be results from each arm of
> the `if` must be the same type

Our two arm values are `100` and `-100`. Both are integer literals; both
fit `i32`; the constraint holds; rustc accepts the program.

Now do the contrast. In the same directory, save a second file
`broken.rs` whose `else` arm produces a *different* type from the
`then` arm:

```rust
fn main() {
    let n: i32 = 7;
    let x = if n > 5 { 100 } else { true };
    println!("x = {x}");
}
```

The *then* arm is the block `{ 100 }` — value `100`, an integer. The
`else` arm is the block `{ true }` — value `true`, a `bool` (lesson
012). Two different types. Compile it. The full transcript lives in
`## Evidence`; the load-bearing observation is the headline:

```text
error[E0308]: `if` and `else` have incompatible types
```

This is **not** the generic `mismatched types` text we saw in lessons
024 and 025. Same E-code (`E0308`) — and the optional `--explain
E0308` trailer is also present, consistent with lesson 003 — but the
headline phrasing is *specialized to this situation*: rustc recognizes
the two-arm-type-mismatch shape and tailors the headline. Reading the
rest with lesson 003's order:

- The `--> broken.rs:3:37` location points at column 37 of line 3 —
  the start of the `else` arm's tail value `true` (the same column
  where the caret `^^^^` underlines `true` in the source excerpt
  below). rustc anchors `-->` at the offending arm's tail value, not
  at the arm's opening `{` (which is column 35).
- The source excerpt's structure has *two* annotations on one line of
  source. Dashes `---` underline the *then* arm's tail value `100`
  with the sub-annotation `expected because of this`. The caret `^^^^`
  underlines the `else` arm's tail value `true` with the trailing
  message `expected integer, found \`bool\``. Read it as: rustc
  decided the expected type from the first arm it saw, then complained
  the second arm did not match.

Removing the `else { true }` arm and putting back `else { -100 }` makes
both arms `i32`, the constraint holds, and rustc accepts the program
again. That is the working `demo.rs`.

## What Changed

- You can put a whole `if condition { value_a } else { value_b }` on
  the right of a `let` and bind to whichever arm's value the condition
  selects.
- New mental-model upgrade for lesson 014: `if`/`else` is itself an
  *expression* — it produces a value. Lesson 014 used it as a step
  whose value was discarded; this lesson keeps the value.
- Why it works at all: each arm is a block (lesson 024); each block's
  value is its tail expression (no `;`); the whole `if`/`else`
  evaluates to the value of whichever arm ran. Lesson 024's rule is
  reused unchanged on a third surface (after lesson 024's
  `let`-right-block and lesson 025's function body).
- New constraint: the two arms must produce the same type. If they do
  not, rustc emits the *specialized* diagnostic
  `error[E0308]: \`if\` and \`else\` have incompatible types` — same
  E-code as lessons 024 and 025, different headline tailored to the
  two-arms situation. The source excerpt marks the first arm with
  `expected because of this` and the second arm with the actual type
  mismatch.

## Check Yourself

You write `pick.rs` containing:

```rust
fn main() {
    let score: i32 = 80;
    let bonus: i32 = if score >= 60 { 10 } else { 0 };
    println!("bonus = {bonus}");
}
```

You run `rustc pick.rs && ./pick`.

(a) Does rustc accept the program, and what does it print?

(b) You change `let score: i32 = 80;` to `let score: i32 = 50;`,
leaving the rest alone. After recompiling, what does `./pick` print?

(c) You instead change the `else { 0 }` arm to `else { false }`,
leaving `score = 80`. What headline does rustc emit, and which arm
does the caret `^^^^^` underline?

(Answers: (a) yes; prints `bonus = 10`. `score >= 60` is `true`, the
*then* arm's tail value `10` becomes the `if`/`else`'s value, `bonus`
is bound to `10`. (b) `bonus = 0`. `50 >= 60` is `false`, the `else`
arm runs, its tail value `0` becomes the `if`/`else`'s value. (c)
`error[E0308]: \`if\` and \`else\` have incompatible types`. The
caret underlines `false` in the `else` arm — that is the second arm
and the one whose type does not match the first arm's `i32`.)

## What To Ignore For Now

Real and deferred:

- *`else if` chains as expressions*, e.g. `let x = if a { 1 } else if b { 2 } else { 3 };`. Same arms-must-share-type rule applies, but
  the no-final-`else` exhaustiveness story is its own move. Deferred.
- *`if`-as-expression as a function body's tail expression*, e.g.
  `fn classify(n: i32) -> i32 { if n > 5 { 100 } else { -100 } }`.
  Combines this lesson with lesson 025's implicit return; not a new
  concept. Deferred.
- *`if`-without-`else`*. Rust *allows* `let x = if cond { value };`,
  but the missing `else` arm implicitly produces the unit type `()`,
  so `x` ends up bound to `()` rather than `value`. The `else` arm is
  effectively mandatory whenever you want a useful value out — gloss
  this and defer the unit-type explanation.
- *`match` as an expression*. Same principle on a different surface;
  deferred.
- *`if let` and `while let`*. Pattern matching; deferred.
- *Nested `if`-as-expression inside an arm*. Allowed; not a new
  concept; deferred.
- *Rustc's "expected integer" wording* in the broken-contrast
  diagnostic. The unannotated literal `100` has not yet been pinned to
  a concrete integer type, which is why rustc says "expected integer"
  rather than "expected i32" — a defaulting subtlety. Mention only;
  deferred.
- *The unit type `()`*. Still deferred (calibrated in lessons 021, 024,
  025).
- All previously deferred items: `cargo`, modules and `pub`, `&`
  references, generics.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the section "Using
  `if` in a `let` Statement" (lines 180-256). Three load-bearing direct
  quotes:

  Lines 182-183 (the licence to put `if` on the right of `let`):

  > Because `if` is an expression, we can use it on the right side of a
  > `let` statement to assign the outcome to a variable

  followed by Listing 3-2 (lines 188-194), which is
  `let number = if condition { 5 } else { 6 };` — exactly this lesson's
  shape with different values.

  Lines 212-214 (the same-type rule for arms):

  > the values that have the potential to be results from each arm of
  > the `if` must be the same type

  Lines 219-230 (the Book's broken-contrast example and its prose
  framing):

  > ```rust
  > let number = if condition { 5 } else { "six" };
  > ```
  >
  > When we try to compile this code, we'll get an error. The `if` and
  > `else` arms have value types that are incompatible

  The Book then prints rustc's diagnostic (lines 233-246) headlined
  `error[E0308]: \`if\` and \`else\` have incompatible types` with
  `expected integer, found \`&str\`` — *the same specialized headline
  this lesson's broken-contrast probe captures*, only with a different
  named type.

  Calibration:
  - The Book builds with `cargo run`; this lesson uses `rustc demo.rs`
    per lesson 001. Behavior is identical; only the build command
    differs.
  - The Book's broken-contrast uses `"six"` (a string literal), which
    generates `expected integer, found \`&str\``. This lesson uses
    `true` (a `bool` literal, lesson 012), which generates
    `expected integer, found \`bool\``. The diagnostic *structure* is
    identical (specialized E0308 headline, two-annotation source
    excerpt with `expected because of this` on the first arm and the
    actual mismatch on the second arm); only the named type differs.
    The substitution is deliberate — `&str` is not installed in this
    run, but `bool` is (lesson 012) — and the load-bearing
    pedagogical observation (the *specialized headline*) is unchanged.
  - Lesson 014 framed `if`/`else` as a control-flow statement that
    runs one of two blocks. This lesson recasts it as a *value-producing
    expression*. Both framings are correct: lesson 014 is the case
    where the value is discarded (the `if`/`else` is *used as* a
    statement); this lesson is the case where the value is kept.

### Probes

Two probes were captured on rustc 1.95.0 (59807616e 2026-04-14) on
Darwin x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/026-if-as-expression.rs`.
The broken-contrast probe is *not* committed under `observations/`;
its transcript is reproduced verbatim below.

Both probes were run in temp directories created with `mktemp -d` and
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
    let n: i32 = 7;
    let category: i32 = if n > 5 { 100 } else { -100 };
    println!("category = {category}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
category = 100
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The single output line is `category = 100`. The `100` is the value
  of the *then* arm's block `{ 100 }`. That value reached the right
  of the outer `let` because the `if`/`else` is itself an expression
  (the load-bearing observation for this lesson) and because each arm
  is a block whose value is its tail expression (lesson 024).
- The two arms produce the same type (`i32`); the constraint named in
  the Book holds; rustc accepts the program.

#### Broken-contrast probe

Same `main`, with `else { true }` instead of `else { -100 }`. Not
committed; the transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let n: i32 = 7;
    let x = if n > 5 { 100 } else { true };
    println!("x = {x}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: `if` and `else` have incompatible types
 --> broken.rs:3:37
  |
3 |     let x = if n > 5 { 100 } else { true };
  |                        ---          ^^^^ expected integer, found `bool`
  |                        |
  |                        expected because of this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0308]: \`if\` and \`else\` have incompatible types`
  is **specialized**: same E-code (`E0308`) as lessons 024 and 025, but
  the headline phrasing is tailored to the two-arm-type-mismatch shape
  rather than the generic `mismatched types`. The optional `--explain
  E0308` trailer is also present, consistent with lesson 003's "code +
  trailer go together" pattern.
- The `--> broken.rs:3:37` location points at column 37 of line 3 —
  the start of the `else` arm's tail value `true` (the same column
  where the caret `^^^^` underlines `true` in the source excerpt
  below). rustc anchors `-->` at the offending arm's tail value, not
  at the arm's opening `{` (which is column 35).
- The source excerpt has *two* annotations on the same source line.
  Dashes `---` underline the *then* arm's tail value `100`; under
  those dashes a sub-annotation reads `expected because of this`. The
  caret `^^^^` underlines the `else` arm's tail value `true`; trailing
  it is `expected integer, found \`bool\``. Read it as: rustc decided
  the expected type from the first arm it saw, then complained the
  second arm did not match.
- The "expected integer" wording (rather than "expected i32") is
  rustc's way of saying the first arm's literal `100` has not yet been
  pinned to a specific integer type. A defaulting subtlety; deferred.
- The named type `bool` in `found \`bool\`` is lesson 012's type.
- Exit code: 1. No executable was produced.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline, `-->` location, source excerpt, optional
  sub-annotations; this lesson exercises that skill on a *specialized*
  E0308 headline.
- `005-let-binding` (accepted, load-bearing) — `let name: TYPE = value;`
  binds a name; reused as the slot the `if`/`else`'s value lands in.
- `009-arithmetic-on-integers` (accepted) — integer values; supplies
  the literal arm values `100`/`-100` and the binding `n`.
- `012-bool-literals` (accepted) — `true`/`false`; the broken-contrast
  probe uses `true`, and rustc's diagnostic names the type `bool`.
- `013-comparison-operators` (accepted, load-bearing) — `n > 5`
  produces a boolean; used as the `if`'s condition.
- `014-if-else` (accepted, load-bearing) — `if condition { ... } else { ... }`
  runs one of two blocks based on the boolean condition. **This lesson
  recasts that same shape as a value-producing expression on the right
  of `let`.**
- `019-type-annotation-i32` (accepted) — `name: TYPE` attaches a type;
  used as `let category: i32 = ...;`.
- `024-statement-vs-expression` (accepted, load-bearing) — a block
  `{ ... }` is itself an expression; its value is its tail expression
  (no `;`). **Each `if`/`else` arm is one such block; this is the
  third surface lesson 024's rule applies to (after lesson 024's
  `let`-right block and lesson 025's function body).**
