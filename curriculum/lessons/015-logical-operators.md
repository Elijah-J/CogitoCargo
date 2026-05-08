---
id: 015-logical-operators
move: "combine boolean values with `&&`, `||`, or `!` to produce a new boolean, then bind and print"
main_concept: "Rust's logical operators act on booleans: `&&` is true when both sides are true, `||` is true when either side is true, `!` flips a boolean; the result is a boolean (lesson 012) that fits on the right of `let` and prints as the word `true` or `false`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 012-bool-literals
  - 013-comparison-operators
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "short-circuit evaluation semantics" moves
  - future "bitwise operators & | ^" moves
  - future "operator precedence" moves
  - future "if conditions with logical operators" moves
sources:
  - output/docs/rust/book/appendix-02-operators.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/015-logical-operators.rs
status: accepted
---

# Combine booleans with `&&`, `||`, and `!`

## The Move

Inside `fn main`, take two boolean values (or one, for `!`) and join
them with one of the three *logical operators* — `&&`, `||`, `!` — on
the right of a `let`. The expression evaluates to a single boolean. Bind
it to a name and print it with `{name}`. The output is the bare word
`true` or `false`. The operands can be boolean literals (`true` /
`false` from lesson 012) or comparison expressions (lesson 013) like
`n > 0`.

## Mental Model Delta

- Before: "I can produce one boolean from one comparison like `n > 0`.
  I have no way to combine two answers — 'is `n` positive *and* less
  than 10?' — into a single boolean."
- After: "Rust has three *logical operators* that act on booleans:
  `a && b` is `true` only when *both* sides are true; `a || b` is
  `true` when *either* side is true; `!a` flips a boolean (`true`
  becomes `false`, `false` becomes `true`). Each one produces a new
  boolean, so the result fits on the right of `let` and prints as the
  bare word `true` or `false` just like lessons 012 and 013."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 004 — `rustc file.rs` makes an executable next
    to the source; the body of `fn main` runs in source order.
  - Lesson 005 (load-bearing): `let name = value;` binds a name;
    `println!("... {name}")` substitutes the bound value.
  - Lesson 012 (load-bearing): `true` and `false` are Rust's two
    boolean literal values; printed with `{name}` they render as the
    bare word.
  - Lesson 013 (load-bearing): comparisons like `n > 0`, `n < 10`,
    `n == 0` produce booleans. This lesson feeds those into `&&`,
    `||`, `!` as operand sub-expressions.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `logical.rs`
containing exactly:

```rust
fn main() {
    let n = 7;
    let in_range = n > 0 && n < 10;
    let outside = n < 0 || n > 100;
    let not_zero = !(n == 0);
    println!("in_range = {in_range}");
    println!("outside = {outside}");
    println!("not_zero = {not_zero}");
}
```

Each `let` on lines 3-5 has an operator expression on the right; each
operand of `&&` and `||` is itself a comparison from lesson 013. The
parentheses in `!(n == 0)` make it unambiguous which sub-expression
`!` applies to.

The Book's operator appendix is the canonical glossary. Three rows are
load-bearing here, quoted verbatim:

> `` `&&` `expr && expr` Short-circuiting logical AND ``

> `` `||` `expr || expr` Short-circuiting logical OR ``

> `` `!` `!expr` Bitwise or logical complement ``

In plain English, for booleans:

- `a && b`: `true` only if both `a` and `b` are `true`.
- `a || b`: `true` if either `a` or `b` (or both) is `true`.
- `!a`: flips `a` — `true` becomes `false`, `false` becomes `true`.

The appendix labels `&&` and `||` as "short-circuiting"; that nuance is
flagged below under What To Ignore For Now. The probe does not depend
on it.

Compile and run:

```console
$ rustc logical.rs
$ ./logical
in_range = true
outside = false
not_zero = true
```

Walk through each line with `n = 7`:

- `in_range`: `n > 0` is `true`; `n < 10` is `true`. So `n > 0 && n < 10`
  is `true && true` = `true`.
- `outside`: `n < 0` is `false`; `n > 100` is `false`. So
  `n < 0 || n > 100` is `false || false` = `false`.
- `not_zero`: `n == 0` is `false`. So `!(n == 0)` is `!false` = `true`.

The three printed values match.

## What Changed

- You can write `a && b`, `a || b`, or `!a` on the right of a `let`
  whenever `a` and `b` are booleans, and bind the result to a name.
- You have names from the Book: *logical AND*, *logical OR*, *logical
  complement*. Two binary operators on two booleans, one unary on one.
- You can feed comparisons (lesson 013) in as operands, e.g.
  `n > 0 && n < 10` for "in the open range 0 to 10".
- You wrap the operand of `!` in parentheses when it is anything more
  than a single name, so `!(n == 0)` is unambiguous.

## Check Yourself

You write `check.rs` containing:

```rust
fn main() {
    let m = 0;
    let zero_or_neg = m == 0 || m < 0;
    let positive = !(m <= 0);
    println!("zero_or_neg = {zero_or_neg}");
    println!("positive = {positive}");
}
```

You run `rustc check.rs` and then `./check`. How many lines does it
print, and what does each say?

(Answers: two lines. `m == 0` is `true`, so the `||` is
`true || (anything)` = `true`, so `zero_or_neg = true`. `m <= 0` is
`true` (0 is equal to 0), so `!(m <= 0)` is `!true` = `false`, so
`positive = false`.)

## What To Ignore For Now

This lesson installs one idea: three logical operators combine
booleans into a new boolean. Deferred:

- *Short-circuit evaluation*. The appendix calls `&&` and `||`
  "short-circuiting": when the left side determines the answer, the
  right side may be skipped. The probe's operands all evaluate cleanly
  either way, so the rule does not bite. Its own future move.
- *Bitwise operators* `&`, `|`, `^`. Separate rows in the appendix
  ("Bitwise AND", "Bitwise OR", "Bitwise exclusive OR"). Different
  family. The double-character `&&` / `||` are the boolean ones.
- *Operator precedence between `&&` and `||`*. In `a || b && c`, Rust
  evaluates `a || (b && c)` because `&&` binds tighter than `||`. The
  probe avoids mixing them; parentheses around `(n == 0)` keep `!`
  unambiguous.
- *Truth tables and De Morgan's laws*. Out of scope.
- *Negation of complex expressions*. `!(n == 0)` is the same boolean
  as `n != 0` from lesson 013; this lesson does not analyze that
  equivalence.
- All previously-deferred items: `mut`, shadowing, type annotations,
  defining functions, the broader format-string DSL, comments,
  `cargo`, traits, `if`/`else` consuming a logical expression.

## Evidence

### Sources

- `output/docs/rust/book/appendix-02-operators.md`, Table B-1
  (Operators). Three load-bearing rows, quoted verbatim:
  - Line 20: `` `!` `!expr` Bitwise or logical complement `` (`Not`).
    Licenses the unary form `!a` and the word *complement*. The row
    covers two meanings (bitwise on integers, logical on booleans);
    the lesson uses only the boolean reading.
  - Line 28: `` `&&` `expr && expr` Short-circuiting logical AND ``.
    Licenses `a && b` and the name *logical AND*. "Short-circuiting"
    is corpus vocabulary, deferred under What To Ignore.
  - Line 73: `` `||` `expr || expr` Short-circuiting logical OR ``.
    Licenses `a || b` and the name *logical OR*. Same deferral.

  Calibration: lines 26 and 71 list single-character `&` and `|` as
  bitwise operators — different family, deferred. Line 19's
  `` `!` `ident!(...)` Macro expansion `` row covers the `!` in
  `println!`, a non-operator use from lesson 001.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/015-logical-operators.rs`.
The committed file is the exact working program below.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
logical.rs
--- cat logical.rs ---
fn main() {
    let n = 7;
    let in_range = n > 0 && n < 10;
    let outside = n < 0 || n > 100;
    let not_zero = !(n == 0);
    println!("in_range = {in_range}");
    println!("outside = {outside}");
    println!("not_zero = {not_zero}");
}
--- rustc logical.rs ---
exit=0
--- ls after compile ---
logical
logical.rs
--- ./logical ---
in_range = true
outside = false
not_zero = true
exit=0
```

Notes:

- `rustc logical.rs` exits 0 and is silent (lesson 001).
- `./logical` prints three lines in source order (lesson 004).
- `in_range = true`: with `n = 7`, `n > 0` and `n < 10` are both
  `true`, so `&&` yields `true`.
- `outside = false`: with `n = 7`, `n < 0` and `n > 100` are both
  `false`, so `||` yields `false`.
- `not_zero = true`: `n == 0` is `false`, so `!(n == 0)` flips to
  `true`.
- Each printed value is the bare word `true` or `false` (lesson 012's
  default rendering).
- Only the working source is committed under `observations/`; no
  binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run`, `002-fn-main-entry-point`,
  `004-statements-in-order` (accepted) — toolchain and source-order
  scaffolding used by the probe.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name; `println!("... {name}")` substitutes the bound value.
  The four `let`s and three `println!`s in the probe reuse this shape.
- `012-bool-literals` (accepted, load-bearing) — `true` and `false`
  printed as the bare word. Each logical expression here produces one
  of those two values.
- `013-comparison-operators` (accepted, load-bearing) — `n > 0`,
  `n < 10`, `n == 0` produce booleans, supplying the operand
  sub-expressions for `&&`, `||`, and `!`.
