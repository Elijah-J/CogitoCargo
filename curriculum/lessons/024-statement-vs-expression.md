---
id: 024-statement-vs-expression
move: "write `let x: i32 = { let inner: i32 = 2; inner + 3 };` to bind `x` to the value of a block; the block's value is the value of its final inner expression, written without a trailing `;`"
main_concept: "Rust distinguishes *statements* (instructions that perform an action and do not return a value) from *expressions* (evaluate to a value); a `;` after an expression turns it into a statement and discards the value; a block `{ ... }` is itself an expression whose value is the value of its final inner expression, written without a trailing `;`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 004-statements-in-order
  - 005-let-binding
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "implicit final-expression return" moves
  - future "if/else as expression in let" moves
  - future "match as expression" moves
  - future "loop with break value" moves
  - future "unit type ()" moves
  - future "block scope" moves
sources:
  - output/docs/rust/book/ch03-03-how-functions-work.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/024-statement-vs-expression.rs
status: accepted
---

# A block can sit on the right of `let`

## The Move

On the right of a `let`, instead of a literal or a single expression
like `x + y`, write a *block* — a pair of curly braces with statements
inside, ending in one line that has no `;`. That last line is the value
the whole block produces, and that value is what `let` binds to the
name. So `let x: i32 = { let inner: i32 = 2; inner + 3 };` binds `x`
to `5`.

## Mental Model Delta

- Before: "Each line in `fn main` ends in `;` and runs in order
  (lesson 004). The right side of `let` is a value — a literal or
  arithmetic (lessons 005, 009). Curly braces are just where `fn
  main`'s body lives."
- After: "Rust splits what I write into two kinds. *Statements*
  perform an action and do not return a value. *Expressions* evaluate
  to a value. A `;` at the end of an expression turns it into a
  statement and throws the value away. A block `{ ... }` is itself
  an expression: its value = the value of the final inner line if
  that line has no `;`. So a block can sit anywhere a value can,
  including on the right of `let`."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`; silent on success.
  - Lesson 002: the body of `fn main` runs when the executable
    launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline, a
    `-->` location, a source excerpt with caret, and optional `help:`
    or `note:` lines. The broken-contrast walkthrough below parses
    this lesson's `help: remove this semicolon to return this value`
    using exactly that skill.
  - Lesson 004: the body of `fn main` is a sequence of `;`-terminated
    lines that run top to bottom. This lesson generalizes those lines
    by giving them a name (*statement*) and contrasting them with a
    second kind of thing (*expression*).
  - Lesson 005 (load-bearing): `let name = value;` binds a name to a
    value. This lesson reuses `let` as the slot the block-expression's
    value lands in.
  - Lesson 009 (load-bearing): `+` between two integers produces a new
    integer. Used inside the block as `inner + 3`.
  - Lesson 019 (load-bearing): `name: TYPE` attaches a type. The
    annotations `let x: i32` and `let inner: i32` make the
    broken-contrast type mismatch (`expected i32, found ()`) precise
    when rustc reports it.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let x: i32 = {
        let inner: i32 = 2;
        inner + 3
    };
    println!("x = {x}");
}
```

The right side of the outer `let` is not a literal and not a single
arithmetic expression — it is a pair of curly braces with two lines
inside. The first, `let inner: i32 = 2;`, is the familiar lesson-005
+ lesson-019 binding, ending in `;`. The second is `inner + 3` — the
lesson-009 sum, with **no `;`**. That bare final line is the *value*
the block produces.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
x = 5
```

Inside the block, the first line binds `inner` to `2`. The second,
`inner + 3`, evaluates to `5`. Because it has no `;`, it is the
block's *tail expression* and the block as a whole evaluates to `5`.
The outer `let` binds `x` to that `5`, and `println!` prints it.

The Book states the rule directly:

> If you add a semicolon to the end of an expression, you turn it
> into a statement, and it will then not return a value.

And the two definitions behind it:

> *Statements* are instructions that perform some action and do not
> return a value.
>
> *Expressions* evaluate to a resultant value.

So `inner + 3` (no `;`) is an expression — value `5` — and it is the
block's tail. Add a `;` and it becomes a statement: it still computes
`5`, but the value is thrown away, and the block has no tail
expression. (The Book's own example, `let y = { let x = 3; x + 1 };`
on lines 225-234 of the source, is exactly this lesson's shape with
different numbers.)

Now do the contrast. In the same directory, save a second file
`broken.rs` that is identical except for one extra `;` after
`inner + 3`:

```rust
fn main() {
    let x: i32 = {
        let inner: i32 = 2;
        inner + 3;
    };
    println!("x = {x}");
}
```

Compile it. rustc rejects the program with `error[E0308]: mismatched
types`. The full transcript and a part-by-part walk live in `##
Evidence` below; the load-bearing bits to read using lesson 003's
order are:

- *Source excerpt* — a multi-line span covering the whole block,
  trailing `expected i32, found ()`. The block as a whole is the
  type-mismatched expression: the outer binding expects `i32`, the
  block produced `()`.
- *Help line* — `help: remove this semicolon to return this value`,
  with a single `-` directly under the stray `;` on line 4. That is
  the rule applied: the `;` turned `inner + 3` from the block's tail
  expression into a discarded statement, so the block has no tail and
  evaluates to `()`.

`()` is the "nothing useful" type that lesson 021 named in passing as
the return type of a function with no `->`. Read it here as "the
block produced no value", and defer it to a later move.

## What Changed

- You can put a block `{ ... }` on the right of `let` and bind to its
  value, not just a literal or arithmetic expression.
- Two new working nouns: a **statement** performs an action and does
  not return a value; an **expression** evaluates to a value.
- Load-bearing rule: a `;` at the end of an expression turns it into
  a statement and throws the value away.
- A block's value is the value of its final inner line *without* a
  trailing `;`. With a trailing `;`, the block has no tail and
  evaluates to `()`; rustc reports this as E0308 `expected T, found
  ()` when the binding expects a real type.
- Lesson 005's `let name = value;` is itself a *statement*; the
  `value` on its right is an *expression*. The two kinds were there
  all along — this lesson just named them.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    let n: i32 = {
        let a: i32 = 4;
        let b: i32 = 6;
        a + b
    };
    println!("n = {n}");
}
```

You run `rustc tiny.rs && ./tiny`.

- Does rustc accept the program, and what does the executable print?
- Inside the block, which lines are statements and which is the tail
  expression?
- If you added a `;` after `a + b`, what would rustc say, and at
  which part of the diagnostic would the help line point?

(Answers: yes; prints `n = 10`. The two `let` lines are statements;
`a + b` with no `;` is the tail expression, value `10`. A `;` after
`a + b` would turn it into a statement and leave the block tailless —
rustc would emit `error[E0308]: mismatched types ... expected i32,
found ()` with a `help: remove this semicolon to return this value`
line under the offending `;`.)

## What To Ignore For Now

Real and deferred:

- *Implicit final-expression return* — `fn five() -> i32 { 5 }` (no
  `return`, no `;`). Same rule applied to a function body instead of
  an arbitrary `let`-right block. Lesson 021 explicitly deferred this
  pending statement-vs-expression; this lesson installs the
  distinction, so it is the immediate downstream unlock.
- *`if`/`else` as an expression*, e.g. `let x = if cond { a } else
  { b };`. Same principle, different surface. Deferred.
- *`match` as an expression*. Deferred.
- *`loop` with `break value;`*, where the loop evaluates to the
  `value` passed to `break`. Deferred.
- *The unit type `()`.* Named by rustc in the broken-contrast
  diagnostic; calibrated in lesson 021. Future move.
- *Block scope.* The inner `let inner` does not leak outside the
  block. Orthogonal scoping move; deferred.
- *Expression statements with side effects* (e.g. a bare `5;`).
  Implicit in the rule, not its own move.
- All previously deferred items: `mut` scope, `&` references,
  generics, `cargo`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-03-how-functions-work.md`, the section
  "Statements and Expressions" (approximately lines 134-250). Three
  load-bearing direct quotes:

  Lines 144-146 (the two definitions):

  > *Statements* are instructions that perform some action and do not
  > return a value.
  >
  > *Expressions* evaluate to a resultant value.

  Lines 220-221 (block-as-expression, justifies putting `{ ... }` on
  the right of `let`):

  > A new scope block created with curly brackets is an expression,
  > for example:

  followed by the Book's `let y = { let x = 3; x + 1 };` example on
  lines 225-234, which is *exactly* the shape used in this lesson's
  probe.

  Lines 247-249 (the load-bearing semicolon rule):

  > Expressions do not include ending semicolons. If you add a
  > semicolon to the end of an expression, you turn it into a
  > statement, and it will then not return a value.

  Calibration: the Book's "Statements and Expressions" section is
  inside chapter 3.3 (Functions) and frames the distinction partly to
  motivate *implicit returns* from function bodies. This lesson takes
  the same definitions and the same semicolon rule, but applies them
  to a *block on the right of `let`* rather than a function body. The
  underlying rule is identical (the Book's own example puts the block
  on the right of `let y`, not in a function body), and the implicit
  function-body return is intentionally deferred to the next cycle so
  that this move installs exactly one concept.

### Probes

Two probes were captured. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/024-statement-vs-expression.rs`.
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
    let x: i32 = {
        let inner: i32 = 2;
        inner + 3
    };
    println!("x = {x}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
x = 5
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The single output line is `x = 5`. The `5` is the value of the
  block's tail expression `inner + 3`, with `inner` holding `2`. That
  value reached the right-hand side of the outer `let` *because the
  tail line has no `;`*; the outer `let` then bound it to `x`, and
  `println!` printed it.
- That a block-with-tail-expression compiles where a value is
  expected (right of `let`) and produces that value is the
  load-bearing observation for this lesson.

#### Broken-contrast probe

Same `main`, with one extra `;` after `inner + 3`. Not committed; the
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
    let x: i32 = {
        let inner: i32 = 2;
        inner + 3;
    };
    println!("x = {x}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:18
  |
2 |       let x: i32 = {
  |  __________________^
3 | |         let inner: i32 = 2;
4 | |         inner + 3;
  | |                  - help: remove this semicolon to return this value
5 | |     };
  | |_____^ expected `i32`, found `()`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0308]: mismatched types` carries the code
  `E0308` (so the optional `--explain E0308` trailer is also
  present, consistent with lesson 003's "code + trailer go together"
  pattern).
- The `--> broken.rs:2:18` location points at column 18 of line 2 —
  the `{` that opens the block-expression on the right of `let`.
- The source excerpt is a *multi-line span*: rustc draws a `^` at
  the opening `{`, vertical bars `|` continuing through lines 3-5,
  and a `^` at the closing `}` on line 5, with the trailing
  annotation `expected i32, found ()`. The whole block is the
  type-mismatched expression.
- The `help:` line `remove this semicolon to return this value`
  points at the dash `-` directly under the stray `;` on line 4.
  This is the load-bearing pedagogical observation: rustc itself
  encodes the semicolon rule, and the help line is exact. Reading it
  uses lesson 003's headline + `-->` + source-excerpt + `help:`
  structure unchanged.
- The type `()` named in `found ()` is the unit type. Calibrated as
  "nothing useful" return in lesson 021; deferred for a dedicated
  move.
- Exit code: 1. No executable was produced.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success. Used by both probes.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline, a `-->` location, a source excerpt with caret, and
  optional `help:`/`note:` lines. The broken-contrast walkthrough
  parses E0308 using exactly this structure, including the
  `help: remove this semicolon to return this value` line.
- `004-statements-in-order` (accepted) — body of `fn main` is a
  sequence of `;`-terminated lines that run top to bottom. This
  lesson generalizes those lines by naming the kind they belong to
  (statement) and contrasting it with expressions.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name to a value. Reused as the slot the block-expression's
  value lands in.
- `009-arithmetic-on-integers` (accepted, load-bearing) — `+` between
  two integers produces a new integer. Used inside the block as
  `inner + 3` to make the tail expression a non-trivial value.
- `019-type-annotation-i32` (accepted, load-bearing) — `name: TYPE`
  attaches a type. The annotations `let x: i32` and `let inner: i32`
  make the broken-contrast diagnostic precise: rustc reports the
  mismatch as `expected i32, found ()` because `x` is annotated `i32`
  and the tailless block has type `()`.
