---
id: 013-comparison-operators
move: "combine two integer values with `==`, `!=`, `<`, `<=`, `>`, or `>=` to produce a boolean, then bind and print it"
main_concept: "Rust's six comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) take two values of the same kind and produce a `true` or `false` (a value of the boolean kind from lesson 012); the result fits on the right of `let` and prints as the word `true` or `false`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 009-arithmetic-on-integers
  - 012-bool-literals
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "if/else expressions" moves
  - future "logical operators && || !" moves
  - future "comparing other kinds of values" moves
  - future "PartialEq / PartialOrd traits" moves
sources:
  - output/docs/rust/book/appendix-02-operators.md
  - output/docs/rust/reference/expressions/operator-expr.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/013-comparison-operators.rs
status: accepted
---

# Compare two integers with `==`, `!=`, `<`, `<=`, `>`, `>=`

## The Move

Inside `fn main`, join two integer values with one of the six
*comparison operators* — `==`, `!=`, `<`, `<=`, `>`, `>=` — on the
right of a `let`. The expression evaluates to a single boolean value
(`true` or `false`, lesson 012). Bind it to a name and print it with
`{name}`. The output is the bare word `true` or `false`.

## Mental Model Delta

- Before: "I can write `true`/`false` directly with `let` and combine
  integers with `+ - * /`. I have no way to write a yes-or-no value
  that *answers a question* about other values."
- After: "Rust has six *comparison operators* — `==`, `!=`, `<`, `<=`,
  `>`, `>=` — that take two values of the same kind and produce a
  boolean. They fit on the right of `let` the same way arithmetic
  operators do (lesson 009); only the kind of value handed back is
  different. They are the normal way to *make* a boolean."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 004 — `rustc file.rs` makes an executable next
    to the source; the body of `fn main` runs in source order.
  - Lesson 005 (load-bearing): `let name = value;` binds a name;
    `println!("... {name}")` substitutes the bound value.
  - Lesson 009 (cited): an *operator expression* (two values joined
    by an operator) fits on the right of `let`. This lesson reuses
    that shape; only the operators and the kind of value produced
    are new.
  - Lesson 012 (load-bearing): `true` and `false` are Rust's two
    boolean literal values; printed with `{name}`, they render as the
    bare word. Each comparison produces one of these two.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `main.rs`
containing exactly:

```rust
fn main() {
    let a = 5;
    let b = 3;
    let bigger = a > b;
    let equal = a == b;
    let no_more = a <= b;
    println!("a > b is {bigger}");
    println!("a == b is {equal}");
    println!("a <= b is {no_more}");
}
```

Lines 2-3 bind `a` and `b` to integers (lesson 005). Lines 4-6 each
have an operator expression on the right of `let` (lesson 009 shape) —
but the operator is now a *comparison*, not arithmetic. The answer is
a boolean (lesson 012): `a > b` asks "is `a` greater than `b`?",
`a == b` asks "is `a` equal to `b`?", `a <= b` asks "is `a` less than
or equal to `b`?".

The Book's operator appendix is the canonical glossary. One row reads:

> `` `!=` `expr != expr` Nonequality comparison ``

Each row gives the operator, the form `expr OP expr`, and a short
explanation. The full set of six:

| Operator | Means              |
| -------- | ------------------ |
| `==`     | Equality           |
| `!=`     | Nonequality        |
| `<`      | Less than          |
| `<=`     | Less than or equal |
| `>`      | Greater than       |
| `>=`     | Greater than or equal |

Compile and run:

```console
$ rustc main.rs
$ ./main
a > b is true
a == b is false
a <= b is false
```

Walk through the three lines:

- `a > b` is `true`: `5` is greater than `3`.
- `a == b` is `false`: `5` and `3` are not the same number.
- `a <= b` is `false`: `5` is greater than `3`, not less than or
  equal to it. `<=` does what its two pieces literally say ("less
  than" *or* "equal"), and neither holds.

One note: the two sides of a comparison must be values of the same
kind. Both sides are integers here, so this works. Comparing an
integer against, say, a piece of text is a separate deferred topic.

## What Changed

- You can write a comparison between two integers on the right of a
  `let` and bind the resulting boolean to a name.
- You know the names: the corpus calls these *comparison* operators;
  the six are `==`, `!=`, `<`, `<=`, `>`, `>=`.
- Each comparison produces a `true` or `false` (lesson 012) that
  prints as the bare word.
- You have the normal way to *produce* booleans, not just type them
  literally.

## Check Yourself

You write `cmp.rs` containing:

```rust
fn main() {
    let x = 4;
    let y = 7;
    let lt = x < y;
    let ge = x >= y;
    println!("x < y is {lt}");
    println!("x >= y is {ge}");
}
```

You run `rustc cmp.rs` and then `./cmp`. How many lines does it print,
and what does each one say?

(Answers: two lines. `x < y is true` because `4` is less than `7`.
`x >= y is false` because `4` is not greater than or equal to `7`.)

## What To Ignore For Now

This lesson installs one idea: comparison operators take two values
of the same kind and produce a `true` or `false`. Deferred:

- `if` / `else` expressions that *consume* a boolean. Lesson 012
  cited the Book's forward pointer that `if` is "the main way to use
  Boolean values"; still a future cycle.
- *Logical operators* `&&`, `||`, `!` that combine or negate booleans.
  Different family.
- Comparing values that are *not* the same kind, e.g. `5 == "five"`.
  We do not exercise this here; cross-kind comparison is its own later
  move with its own grounding.
- The `PartialEq` and `PartialOrd` *traits* in the appendix's
  "Overloadable?" column. Deeper machinery for opting other kinds of
  values into comparison; traits are a much later move.
- *Comparison chaining* like `a < b < c`. The Reference says
  "Parentheses are required when chaining comparison operators. For
  example, the expression `a == b == c` is invalid and may be written
  as `(a == b) == c`." Bare chaining is not what we are doing here.
- *Floating-point comparison subtleties* (NaN etc.). Floats are still
  deferred from lesson 009.
- Comparing strings, ranges, custom types, or anything else.
- Bit-shift operators `<<`, `<<=`, `>>`, `>>=` on adjacent table rows
  — visually similar to `<` / `>`, unrelated.
- All previously-deferred items: `mut`, shadowing, type annotations,
  the broader format-string DSL, comments, defining functions,
  function parameters and return values, `cargo`.

## Evidence

### Sources

- `output/docs/rust/book/appendix-02-operators.md`, Table B-1
  (Operators). The six rows for the comparison operators are the
  load-bearing source:
  - Line 21: `` `!=` `expr != expr` Nonequality comparison ``
    (`PartialEq`).
  - Line 58: `` `<` `expr < expr` Less than comparison `` (`PartialOrd`).
  - Line 59: `` `<=` `expr <= expr` Less than or equal to comparison ``
    (`PartialOrd`).
  - Line 61: `` `==` `expr == expr` Equality comparison `` (`PartialEq`).
  - Line 63: `` `>` `expr > expr` Greater than comparison `` (`PartialOrd`).
  - Line 64: `` `>=` `expr >= expr` Greater than or equal to comparison ``
    (`PartialOrd`).

  Each row gives the operator, the form `expr OP expr`, and the short
  word for the question. The lesson's vocabulary ("equality",
  "nonequality", "less than", "less than or equal", "greater than",
  "greater than or equal") is taken directly from these rows.

  Calibration:
  - Lines 56-57 and 65-66 of the same table list `<<`, `<<=`, `>>`,
    `>>=` as bit-shift operators — visually similar, different
    family; deferred under What To Ignore For Now.
  - The "Overloadable?" column references `PartialEq` / `PartialOrd`
    traits. The lesson does *not* introduce traits; flagged deferred.
  - The form `expr OP expr` licenses "two values of the same kind"
    without yet introducing the word *type*.

- `output/docs/rust/reference/expressions/operator-expr.md`, the
  comparison-expression section (`expr.cmp.paren-chaining`, around
  line 498). Grounds the chaining deferral in What To Ignore For Now.
  Verbatim:

  > Parentheses are required when chaining comparison operators. For
  > example, the expression `a == b == c` is invalid and may be written
  > as `(a == b) == c`.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/013-comparison-operators.rs`.
The committed file is the exact working program below.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
main.rs
--- cat main.rs ---
fn main() {
    let a = 5;
    let b = 3;
    let bigger = a > b;
    let equal = a == b;
    let no_more = a <= b;
    println!("a > b is {bigger}");
    println!("a == b is {equal}");
    println!("a <= b is {no_more}");
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
a > b is true
a == b is false
a <= b is false
exit=0
```

Notes:

- `rustc main.rs` exits 0 and is silent (lesson 001).
- `./main` prints three lines in source order (lesson 004).
- `a > b is true`: `5 > 3` holds.
- `a == b is false`: `5` is not equal to `3`.
- `a <= b is false`: `5` is not less than `3`, and not equal to `3`,
  so neither half of `<=` holds. This is the load-bearing observation
  that the compound operator name describes its meaning literally.
- Each printed value is the bare word `true` or `false` (lesson 012's
  default rendering).
- Only the working source is committed under `observations/`; no
  binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run`, `002-fn-main-entry-point`,
  `004-statements-in-order` (accepted) — toolchain and source-order
  scaffolding used by the probe.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name; `println!("... {name}")` substitutes the bound value
  at print time. The five `let`s and three `println!`s in the probe
  reuse this shape.
- `009-arithmetic-on-integers` (accepted, cited) — established that
  an "operator expression" fits on the right of `let`. Comparisons
  reuse that pattern; only the kind of value produced is new.
- `012-bool-literals` (accepted, load-bearing) — `true` and `false`
  are Rust's two boolean literal values; printed with `{name}`, they
  render as the bare word. Each comparison in this lesson produces
  one of those two values.
