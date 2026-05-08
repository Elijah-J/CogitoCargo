---
id: 037-remainder-operator
move: "use `%` between two integer values to produce the *remainder* after integer division, e.g. `let r: i32 = 10 % 3;` binds `r` to `1`"
main_concept: "`%` is the *remainder* operator, the fifth member of Rust's basic arithmetic family alongside `+ - * /` from lesson 009; for two integer values `a` and `b` (`b` non-zero), `a % b` produces the integer left over after dividing `a` by `b`, satisfying the identity `a == (a / b) * b + (a % b)`; lesson 009's integer `/` truncates toward zero so `10 / 3` is `3` and `10 % 3` is `1`; `%` is a *remainder*, not a mathematical *modulo* — for negative dividends the result has the sign of the dividend, but this lesson uses positive operands only"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 005-let-binding
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "compound remainder-assign `%=`" moves
  - future "operator precedence" moves
  - future "negative-dividend behavior and `rem_euclid`" moves
  - future "`%` on floats" moves
  - future "even/odd checks via `n % 2 == 0`" moves
  - future "runtime panics (e.g. divide-by-zero)" moves
sources:
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/book/appendix-02-operators.md
  - output/docs/rust/reference/expressions/operator-expr.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/037-remainder-operator.rs
status: accepted
---

# The `%` (remainder) operator on integers

## The Move

Inside `fn main`, write `let name: i32 = a % b;` where `a` and `b` are
two integer values. The expression `a % b` produces the *remainder*
after dividing `a` by `b`. The shape on the page is identical to
lesson 009's `+`, `-`, `*`, `/`: two operands separated by a single
infix symbol, the whole thing sits on the right of `let`. For example,
`10 % 3` evaluates to `1`, because `10 = 3 * 3 + 1`.

## Mental Model Delta

- Before: "Rust's basic arithmetic operators are `+`, `-`, `*`, `/`
  (lesson 009). Integer division truncates toward zero — `10 / 3` is
  `3`, throwing away the leftover `1`."
- After: "There is a fifth basic arithmetic operator, `%`, that
  produces exactly the *leftover* the integer `/` threw away. Together
  `/` and `%` partition the dividend: `a == (a / b) * b + (a % b)`.
  The Book lists five basic mathematical operations, not four:
  *addition, subtraction, multiplication, division, and remainder.*
  `%` is also infix, also produces an `i32` from two `i32` operands,
  also fits on the right of `let`. One subtlety to flag for later:
  Rust's `%` is *remainder*, not *mathematical modulo* — they coincide
  for positive operands, which is all this lesson uses."

## Prerequisites

- Installed concepts:
  - Lesson 001 (`001-rustc-compile-and-run`): `rustc file.rs`
    produces an executable next to the source; `./name` runs it;
    `rustc` is silent on success.
  - Lesson 002 (`002-fn-main-entry-point`): the body of
    `fn main() { ... }` runs when the executable launches.
  - Lesson 005 (`005-let-binding`): `let name = value;` binds a name
    to a value, and `println!("... {name} ...");` substitutes the
    bound value at print time.
  - Lesson 009 (`009-arithmetic-on-integers`, load-bearing): the
    binary operators `+ - * /` between two integer values produce a
    new integer value; the expression fits on the right of `let`;
    integer division truncates toward zero, so `5 / 3` is `1`. **This
    lesson promotes lesson 009's deferred `%` mention to the installed
    move and adds it as the fifth member of the same family.**
  - Lesson 019 (`019-type-annotation-i32`): `let name: i32 = value;`
    attaches an explicit `i32` type annotation. The probe writes
    `let q: i32 = ...;` and `let r: i32 = ...;` for symmetry with the
    Book's prose ("the basic mathematical operations ... for all the
    number types") even though the annotation is optional here.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let q: i32 = 10 / 3;
    let r: i32 = 10 % 3;
    println!("10 / 3 = {q}");
    println!("10 % 3 = {r}");
}
```

Two `let` lines. The first uses `/` from lesson 009; the second uses
the new `%`. Predict — write your guess down before you compile —
what each output line says:

```
10 / 3 = ?
10 % 3 = ?
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
10 / 3 = 3
10 % 3 = 1
```

Walk it. `10 / 3` is integer division (lesson 009): `3 * 3 = 9` is the
largest multiple of `3` that is not greater than `10`, so the
truncated quotient is `3`. The natural follow-up question — *what got
thrown away?* — is exactly what `%` answers. `10 - 3 * 3` is `10 - 9`
is `1`, and that is what `10 % 3` produces. Together `/` and `%`
satisfy the identity

```text
a == (a / b) * b + (a % b)
```

for any integers `a` and `b` with `b` non-zero. Plug in: `(10 / 3) * 3
+ (10 % 3) == 3 * 3 + 1 == 10`. The probe checks both halves of that
identity in two `let` lines.

The Book lists `%` directly alongside `+ - * /` in the *Numeric
Operations* section:

> Rust supports the basic mathematical operations you'd expect for
> all the number types: addition, subtraction, multiplication,
> division, and remainder.

— and its same example program includes `let remainder = 43 % 5;`
right after the integer-division line. The Reference's operator table
calls `%` *Remainder* and lists it in the same row family as
`+ - * /`. The Appendix B operator table summarizes it as `expr %
expr`, *Arithmetic remainder*. All three corpus sources agree: `%` is
the fifth basic arithmetic operator, same shape, same kinds of
operands.

One calibration to know about now and defer the details on. Rust's
`%` is a *remainder*, not a mathematical *modulo*. The two coincide
for non-negative operands like the probe's `10 % 3`. They differ when
the dividend is negative: `-7 % 3` evaluates to `-1` in Rust (the sign
of the dividend), where mathematical modulo would give `2` (always
non-negative). The Reference says this directly:

> Rust uses a remainder defined with truncating division. Given
> `remainder = dividend % divisor`, the remainder will have the same
> sign as the dividend.

The probe sticks to positive operands so this distinction does not
surface; the negative-dividend case is deferred.

## What Changed

- You can write `a % b` between two integer values, and the whole
  expression is an integer that fits on the right of `let`. Same
  shape as lesson 009's `+ - * /`.
- You know the leftover-after-integer-division identity
  `a == (a / b) * b + (a % b)`. Lesson 009 said `10 / 3` is `3`; this
  lesson says `10 % 3` is `1`; `3 * 3 + 1` is `10`.
- You know the corpus-canonical name for `%` is *remainder*, not
  *modulo*, with one consequence: the sign of `a % b` matches the
  sign of `a` (the dividend) — but only positive operands appear in
  this lesson's probe.
- You know `%` is the fifth member of Rust's basic arithmetic family.
  The Book and the Reference both list it alongside `+ - * /`.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    let n: i32 = 17;
    let d: i32 = 5;
    let q: i32 = n / d;
    let r: i32 = n % d;
    println!("q = {q}");
    println!("r = {r}");
}
```

You run `rustc tiny.rs && ./tiny`.

- What two output lines does the executable print, in order?
- Verify the identity `n == q * d + r` using your two answers.
- Predict (do not compile): if you replaced `let r: i32 = n % d;`
  with `let r: i32 = n % 0;`, would `rustc` reject the program at
  compile time? (Hint: lesson 009 didn't address this case either.)

(Answers: `q = 3` then `r = 2`. Identity: `3 * 5 + 2 == 17`. Yes. The
`n % 0` version *compiles* — `%` accepts any two `i32` values at
compile time — but the executable panics at runtime with the message
*attempt to calculate the remainder with a divisor of zero*. Runtime
panics are deferred; the point of the prediction is just that
divisor-zero is a runtime concern, not a compile-time one.)

## What To Ignore For Now

This lesson installs only one idea: `%` between two integer values
produces the integer remainder of `/`, fits on the right of `let`,
and is the fifth member of Rust's basic arithmetic family. Each of
the following is real and will be taught later, but is *not* part of
this move:

- *`%` on floats*. `5.5 % 2.0` works and produces `1.5`. The Reference
  table lists `Floating Point: Remainder` in the same row as
  `Integer: Remainder`. This lesson stays integer-only to remain
  parallel with lesson 009.
- *Compound remainder-assign `%=`*. Analogous to lesson 023's `+=`,
  `-=`, `*=`, `/=`. The Appendix B table lists `var %= expr` next to
  `expr % expr`. Future move.
- *Mathematical modulo vs. remainder*. Rust's `%` follows the dividend
  in sign; mathematical modulo always returns non-negative. To get
  true non-negative modulo on signed integers there is a method
  `i32::rem_euclid`. Future move (probably with method-call syntax).
- *Negative-dividend behavior*. `-7 % 3` is `-1`, not `2`. Reference
  line 452 states this directly. The probe uses positive operands so
  the distinction never surfaces here.
- *Runtime panic on `% 0`*. `let r: i32 = 5 % 0;` compiles fine but
  the executable panics at runtime. Runtime panics are a different
  failure category from lesson 003's compile-time diagnostics.
  Deferred.
- *Operator precedence*. `%` shares precedence with `*` and `/` (all
  three bind tighter than `+` and `-`). The probe uses no mixed
  expressions, so precedence never matters here. Deferred.
- *Trait overloading via `std::ops::Rem`*. The Reference table names
  `std::ops::Rem` as the trait that lets non-primitive types
  customize `%`. Deferred along with the rest of trait machinery.
- All previously deferred items.

## Evidence

### Sources

- `output/docs/rust/book/ch03-02-data-types.md`, the *Numeric
  Operations* subsection (lines 167-199). Two load-bearing direct
  quotes:
  - Lines 169-171: "Rust supports the basic mathematical operations
    you'd expect for all the number types: addition, subtraction,
    multiplication, division, and remainder. Integer division
    truncates toward zero to the nearest integer." Lesson 009 used
    the integer-division half of this sentence; this lesson uses the
    *remainder* word at the end of the list, which establishes that
    `%` is one of the five "basic mathematical operations" alongside
    the four lesson 009 already installed.
  - Line 192: `let remainder = 43 % 5;` — the Book's own example use
    of `%`. Confirms the operator syntax is `a % b`. The Book's
    binding is unannotated (relying on integer-literal default
    `i32`); this lesson uses an explicit `let r: i32 = ...;` per
    lesson 019.

- `output/docs/rust/book/appendix-02-operators.md`, the operator
  table. Load-bearing row, line 22:

  > `%` | `expr % expr` | Arithmetic remainder | `Rem`

  Confirms the formal name *Arithmetic remainder* and the syntactic
  shape `expr % expr`. The next row (line 23) is `%=` `var %= expr`
  *Arithmetic remainder and assignment*; mentioned only in *What To
  Ignore For Now* as the deferred compound form analogous to lesson
  023's `+=`/etc.

- `output/docs/rust/reference/expressions/operator-expr.md`,
  *Arithmetic and logical binary operators* section (lines 411-474).
  Three load-bearing pieces:
  - Lines 417-422: the grammar `ArithmeticOrLogicalExpression →
    Expression + Expression | ... | Expression % Expression | ...`,
    listing `%` as one of the binary arithmetic forms in the same
    family as `+`, `-`, `*`, `/`.
  - Lines 437-443: the operator behavior table, row `%`:

    > `%` | Remainder\*\*† |  | Remainder | `std::ops::Rem` | `std::ops::RemAssign`

    Same row family as `+ - * /`; the column under *Integer* is
    *Remainder*, the column under *Floating Point* is also
    *Remainder*. Footnote `†` is "For integer types, division by zero
    panics" — corpus source for the deferred runtime-panic
    calibration.
  - Line 452: the footnote `**`:

    > Rust uses a remainder defined with truncating division. Given
    > `remainder = dividend % divisor`, the remainder will have the
    > same sign as the dividend.

    Corpus source for the *remainder vs. modulo* calibration.
  - Line 467: the operator example `assert_eq!(100 % 7, 2);`,
    confirming the value of `%` on positive operands matches ordinary
    arithmetic intuition. The probe uses `10 % 3` which is the same
    shape on smaller numbers.

  Calibration:
  - The probe uses only positive operands (`10`, `3`). The Reference
    sentence on dividend sign is therefore *cited but not exercised*
    in the probe; the negative-dividend case is mentioned in
    `## What To Ignore For Now` as deferred. The probe never
    contradicts the Reference.
  - The probe does not exercise `% 0`. The Reference footnote `†`
    says "For integer types, division by zero panics" — this is a
    runtime panic, not a compile-time error, and is deferred per the
    *What To Ignore For Now* note.
  - The probe stays integer-only. The Reference's *Floating Point:
    Remainder* column is cited only to note that `%` works on floats
    too; floats are deferred.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/037-remainder-operator.rs`.
The committed file is the working program. There is no separate
broken-contrast file; the natural broken-contrast `% 0` is a runtime
panic rather than a compile-time error, so it does not fit lesson
003's diagnostic-walking framework. The load-bearing observation is
the printed values of `10 / 3` and `10 % 3` together with the
identity `(10 / 3) * 3 + (10 % 3) == 10`.

Probe transcript, run in a temp directory created with `mktemp -d`
and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let q: i32 = 10 / 3;
    let r: i32 = 10 % 3;
    println!("10 / 3 = {q}");
    println!("10 % 3 = {r}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
10 / 3 = 3
10 % 3 = 1
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The first output line `10 / 3 = 3` re-confirms lesson 009's
  integer-division-truncates-toward-zero rule.
- The second output line `10 % 3 = 1` is the new observation: `%`
  produces the leftover. Identity check: `(10 / 3) * 3 + (10 % 3) ==
  3 * 3 + 1 == 10`. Holds.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success. Used for the compile-and-run cycle in
  this probe.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `005-let-binding` (accepted) — `let name = value;` binds a name to
  a value, and `println!("... {name} ...");` substitutes the bound
  value at print time. Reused twice in the probe.
- `009-arithmetic-on-integers` (accepted, load-bearing) — `+ - * /`
  on two integer values produce a new integer value, fit on the
  right of `let`, and `/` truncates toward zero so `5 / 3` is `1`.
  This lesson adds `%` as the fifth member of the same family,
  parallel in shape, with the corresponding "leftover" semantics.
  The integer-division-truncates rule from lesson 009 is what makes
  `10 / 3 == 3` in the probe, which in turn makes `10 % 3 == 1` the
  consistent leftover.
- `019-type-annotation-i32` (accepted) — `let name: i32 = value;`
  attaches the `i32` type annotation. The probe uses two annotated
  bindings (`let q: i32 = ...;`, `let r: i32 = ...;`) for symmetry
  with the Book's "for all the number types" framing; the
  annotations are not strictly required (integer literals default to
  `i32`) but they make the lesson's "two integers in, one integer
  out" framing visually explicit.
