---
id: 009-arithmetic-on-integers
move: "combine integer values with `+ - * /` on the right of a `let`, then print each bound result"
main_concept: "Rust's binary operators `+` `-` `*` `/` combine two integer values into a new integer value; the resulting expression fits on the right of `let`; integer division truncates toward zero so `5 / 3` is `1`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "remainder operator %" moves
  - future "operator precedence" moves
  - future "types / i32 / floats" moves
  - future "shadowing with arithmetic" moves
sources:
  - output/docs/rust/book/ch03-02-data-types.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/009-arithmetic-on-integers.rs
status: accepted
---

# Arithmetic on integers with `+ - * /`

## The Move

Inside `fn main`, write a `let` whose right-hand side is *not* a bare
literal like `5`, but two integer values joined by one of `+`, `-`,
`*`, `/`. The whole right side evaluates to a single integer value
that gets bound to the name on the left. Print it with
`println!("... {name}")`. The two values on either side of the
operator can be bare literals (`5 + 10`), already-bound names
(`a + b`), or one of each.

## Mental Model Delta

- Before: "the right of `let` is a value like `5`. I have not seen
  anything else go there."
- After: "the right of `let` can also be two values joined by an
  operator. The Book calls `+`, `-`, `*`, `/` *operators* and uses
  the words *add*, *subtract*, *multiply*, *divide*. Each combines
  two integer values into one new integer value. One surprise: when
  both operands are integers, `/` *truncates toward zero* — `5 / 3`
  is `1`, not `1.66…`."

## Prerequisites

- Installed concepts:
  - Lesson 001 (`001-rustc-compile-and-run`): `rustc file.rs`
    produces an executable next to the source; `./name` runs it;
    `rustc` is silent on success.
  - Lesson 002 (`002-fn-main-entry-point`): the body of
    `fn main() { ... }` runs when the executable launches.
  - Lesson 004 (`004-statements-in-order`, load-bearing): the body
    of `fn main` is a sequence of `;`-terminated statements that run
    top to bottom; a later statement can use a name introduced by an
    earlier one.
  - Lesson 005 (`005-let-binding`, load-bearing): `let name = value;`
    binds a name to a value; later statements use the name as that
    value; `println!("... {name} ...");` substitutes the bound value
    at print time.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `main.rs`
containing exactly:

```rust
fn main() {
    let a = 5;
    let b = 3;
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;
    println!("sum  = {sum}");
    println!("diff = {diff}");
    println!("prod = {prod}");
    println!("quot = {quot}");
}
```

Read the file before compiling. The first two `let` statements bind
`a` and `b` (lesson 005 shape). The next four `let` statements each
have an *operator expression* on the right (`a + b`, `a - b`,
`a * b`, `a / b`) — the new thing this lesson teaches. The four
`println!` lines print each bound result using the `{name}` form
from lesson 005.

Now *predict* — write your guess down before you compile — what the
four output lines will say:

```
sum  = ?
diff = ?
prod = ?
quot = ?
```

Most beginners predict `sum = 8`, `diff = 2`, `prod = 15`, and
something like `quot = 1.66…` or `quot = 1.6`. Compile and run:

```console
$ rustc main.rs
$ ./main
sum  = 8
diff = 2
prod = 15
quot = 1
```

The first three are unsurprising. `quot` is the surprise: it is `1`,
not `1` point anything. Both operands are integers, so the `/`
operator does *integer division*, which throws away the fractional
part. The Book states this directly:

> Rust supports the basic mathematical operations you'd expect for
> all the number types: addition, subtraction, multiplication,
> division, and remainder. Integer division truncates toward zero to
> the nearest integer.

`5 / 3` would be `1.66…` in arithmetic, so truncating gives `1`.

The Book mentions two nearby things in the same section that this
lesson does *not* teach: a `%` *remainder* operator, and a separate
kind of number called *floating-point* (with operators that do
return things like `1.66…`). See What To Ignore For Now.

## What Changed

- You can write two integer values joined by `+`, `-`, `*`, or `/` on
  the right of a `let`; the whole expression evaluates to one new
  integer value, which gets bound to the name on the left.
- The corpus calls `+`, `-`, `*`, `/` *operators* and the actions
  *addition*, *subtraction*, *multiplication*, *division*.
- The operands can be bare literals, already-bound names, or a mix.
- Integer-division surprise: `5 / 3` is `1`, not `1.66…`. Integer
  division truncates toward zero.

## Check Yourself

You write `arith.rs` containing:

```rust
fn main() {
    let p = 10;
    let q = 4;
    let r = p / q;
    let s = p - q;
    println!("r = {r}");
    println!("s = {s}");
}
```

You run `rustc arith.rs` and then `./arith`.

- What two lines does the executable print, in order?
- Why is the first line not `r = 2.5`?

(Answers: `r = 2` then `s = 6`. The first line is not `r = 2.5`
because `p` and `q` are integers, so `p / q` is integer division and
truncates toward zero; `10 / 4` would be `2.5`, so truncating gives
`2`.)

## What To Ignore For Now

This lesson installs only one idea: `+`, `-`, `*`, `/` between two
integer values produce a new integer value, fit on the right of
`let`, and `/` truncates toward zero. Each of the following is real
and will be taught later, but is *not* part of this move:

- *Floating-point numbers* and float arithmetic. The Book's same
  section also writes `let quotient = 56.7 / 32.2;`, where `/` does
  *not* truncate. Floats and the names `f32` / `f64` are deferred.
- The *remainder* operator `%`. The Book lists it alongside
  `+ - * /`; this lesson does not teach it.
- *Operator precedence*. Mixed expressions like `2 + 3 * 4` evaluate
  to `14`, not `20`. The probe uses only single-operator expressions
  (`a + b`, not `a + b * 2`) so precedence never comes up. Deferred.
- *Integer overflow* when a result exceeds the largest representable
  integer. Out of scope; we stay with small numbers.
- *Types* and *type annotations* (`i32`, `i64`, `u32`, `usize`, …).
  Still deferred from lesson 005.
- *Methods on numbers* like `n.pow(2)` or `n.abs()`. Deferred.
- *Comparison operators* (`==`, `<`, `>`) and *logical operators*
  (`&&`, `||`). Different operator families; deferred.
- The detail that `/` truncates *toward zero* rather than toward
  negative infinity. The Book shows `let truncated = -5 / 3; //
  Results in -1`, which only matters with negative operands. This
  lesson uses only positive operands, so the simpler "throw away
  the fractional part" reading is enough.
- All previously deferred items: `mut`, shadowing, `&mut`, the broader
  format-string DSL, comments, `cargo`, defining your own functions,
  function parameters, return values, and statements vs expressions
  beyond the informal "right-of-`let` is one expression" used here.

## Evidence

### Sources

- `output/docs/rust/book/ch03-02-data-types.md`, the "Numeric
  Operations" section (lines 167-199). Two load-bearing direct
  quotes:
  - Lines 169-171: "Rust supports the basic mathematical operations
    you'd expect for all the number types: addition, subtraction,
    multiplication, division, and remainder. Integer division
    truncates toward zero to the nearest integer." This is the
    corpus source for the lesson's vocabulary (*addition*,
    *subtraction*, *multiplication*, *division*) and for the
    integer-division truncation rule.
  - Lines 196-198: "Each expression in these statements uses a
    mathematical operator and evaluates to a single value, which is
    then bound to a variable." This is the corpus source for the
    framing that an operator expression on the right of `let`
    evaluates to one value and gets bound to the name on the left.

  Calibration:

  - The Book's example program (lines 176-193) mixes integer and
    float literals — `let sum = 5 + 10;` and `let product = 4 * 30;`
    use integers, but `let difference = 95.5 - 4.3;` and
    `let quotient = 56.7 / 32.2;` use floats. This lesson uses *only*
    integer literals so it does not have to introduce floating-point
    numbers, the `f32` / `f64` types, or the fact that `/` between
    floats does *not* truncate. Floats are listed under What To
    Ignore For Now.
  - The Book's example also includes `let truncated = -5 / 3; //
    Results in -1` to demonstrate that truncation is *toward zero*,
    not floor (a floor-division language would round toward negative
    infinity and give `-2`). This lesson uses only positive operands,
    so the simpler "throw away the fractional part" intuition is
    sufficient. The toward-zero precision is listed under What To
    Ignore For Now.
  - The Book's example also includes `let remainder = 43 % 5;`. This
    lesson does not introduce `%`; it is listed under What To Ignore
    For Now.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/009-arithmetic-on-integers.rs`.
The committed file is the working program. There is no separate
broken-contrast file; the load-bearing observation is the
prediction-versus-result on `quot`, recorded inside this Evidence
section.

Probe transcript, run in a temp directory created with `mktemp -d`
and removed at the end:

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
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;
    println!("sum  = {sum}");
    println!("diff = {diff}");
    println!("prod = {prod}");
    println!("quot = {quot}");
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
sum  = 8
diff = 2
prod = 15
quot = 1
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The four output lines match the predictions for `+`, `-`, `*` and
  contradict the natural-arithmetic prediction for `/`. `5 / 3` would
  be `1.66…` in ordinary arithmetic; the printed value is `1`. That
  is the integer-division-truncates-toward-zero observation the Book
  describes.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success. Used for the compile-and-run cycle in
  this probe.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted, load-bearing) — the body of
  `fn main` is a sequence of `;`-terminated statements that run top
  to bottom; a later `let` can refer to names from earlier `let`s
  (this is what makes `let sum = a + b;` work, two lines after the
  `let a = 5;` and `let b = 3;`).
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name to a value, and `println!("... {name} ...");`
  substitutes the bound value at print time. The new thing in this
  lesson is what can sit on the right of `=`: not just a literal
  like `5`, but an operator expression like `a + b`.
