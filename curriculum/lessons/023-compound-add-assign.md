---
id: 023-compound-add-assign
move: "use `n += value;` to add a value to a `mut` binding in place; analogous shorthands `-=`, `*=`, `/=` exist for the other arithmetic operators"
main_concept: "`n += value;` is shorthand for `n = n + value;` (lesson 006 reassignment + lesson 009 arithmetic); it requires `n` to be declared with `let mut`; the same shape works for `-=`, `*=`, `/=` with their corresponding operators"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 006-mut-binding
  - 009-arithmetic-on-integers
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "remainder compound `%=`" moves
  - future "bitwise compound `&=` `|=` `^=` `<<=` `>>=`" moves
  - future "AddAssign and other compound-assignment traits" moves
sources:
  - output/docs/rust/book/appendix-02-operators.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/023-compound-add-assign.rs
status: accepted
---

# Compound add-assign with `+=`

## The Move

Inside `fn main`, after `let mut n = 0;`, write `n += 1;` instead of
`n = n + 1;`. The two forms are shorthand for the same action: add
the right-hand value to `n` and store the result back into `n`. The
same pattern works with `-`, `*`, `/`: `n -= value;`, `n *= value;`,
`n /= value;`.

## Mental Model Delta

- Before: "to bump a `mut` binding I write `n = n + 1;`. There is no
  other spelling."
- After: "Rust has a *compound assignment* shorthand: `n += value;` is
  exactly `n = n + value;`. It still requires `let mut` because it is
  still a reassignment. The same shape exists for `-`, `*`, `/`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 004: compile and run with `rustc`; `fn main`
    runs; statements execute top-to-bottom.
  - Lesson 005: `let name = value;` and the `{name}` placeholder.
  - Lesson 006 (load-bearing): plain `let` is immutable; `let mut`
    makes a binding reassignable, so `name = new_value;` is allowed.
    Compound assignment is a flavor of that reassignment, so the
    `mut` requirement carries over.
  - Lesson 009 (load-bearing): `+`, `-`, `*`, `/` combine two integer
    values into a new integer value. The desugared form
    `n = n + value;` uses lesson 009's `+`; the other compound forms
    reuse the matching operator.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let mut n = 0;
    n += 1;
    n += 2;
    n += 3;
    println!("n = {n}");
}
```

Line 2 is a `let mut` binding starting `n` at `0`. Lines 3-5 are the
new shape: binding name, `+=`, value — no `let`, no second `n`.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 6
```

Walk through by lesson 004's source-order rule:

- After `let mut n = 0;`, `n` is `0`.
- `n += 1;` means `n = n + 1;`. `n` becomes `1`.
- `n += 2;` means `n = n + 2;`. `n` becomes `3`.
- `n += 3;` means `n = n + 3;`. `n` becomes `6`.
- `println!("n = {n}");` prints `n = 6`.

`n += 1;` is *exactly* `n = n + 1;`, just shorter. The pattern
generalizes to the other lesson 009 operators: `m -= 1;` (if
`m` started at `5`, `m` ends at `4`); `p *= 2;` (`3` to `6`);
`q /= 2;` (`10` to `5`). The probe only exercises `+=`.

The corpus's appendix lists these forms in one table. The
load-bearing row, verbatim:

> `+=`  `var += expr`  Arithmetic addition and assignment

Table B-1 has matching rows for `-=`, `*=`, and `/=` with the same
`var OP= expr` shape.

## What Changed

- You can write `n += value;` to add to a `mut` binding in place,
  instead of `n = n + value;`. Both produce the same result.
- The new statement still requires `let mut`. Compound assignment is
  a flavor of reassignment, so lesson 006's `mut` rule applies.
- The same shape exists for `-=`, `*=`, `/=`. One concept,
  four instances.
- You have a name for it: *compound assignment*.

## Check Yourself

You write `bump.rs`:

```rust
fn main() {
    let mut k = 10;
    k -= 4;
    k += 1;
    println!("k = {k}");
}
```

- What does `./bump` print?
- Rewrite lines 3 and 4 without using `-=` or `+=`, so the behavior
  is the same.

(Answers: `k = 7`. `k -= 4;` makes `k` equal `6`; `k += 1;` makes `k`
equal `7`. The longer form: `k = k - 4;` then `k = k + 1;`.)

## What To Ignore For Now

This lesson installs only one idea: `n += value;` is shorthand for
`n = n + value;`, and the same shorthand exists for `-`, `*`, `/`.
Each of the following is real but *not* part of this move:

- `%=` (remainder-assign). Same family, but `%` itself is deferred
  from lesson 009.
- The bitwise compound forms `&=`, `|=`, `^=`, `<<=`, `>>=`. Listed
  in the same appendix table; different operator family.
- *Operator overloading* via the `AddAssign` / `SubAssign` /
  `MulAssign` / `DivAssign` traits in the appendix's "Overloadable?"
  column. Out of scope.
- Compound assignment on non-integer types (floats, strings).
- All previously deferred items.

## Evidence

### Sources

- `output/docs/rust/book/appendix-02-operators.md`, Table B-1
  (the operator/symbol catalog). Four rows are load-bearing for this
  lesson:
  - Line 35, the verbatim row for `+=`:
    `` | `+=` | `var += expr` | Arithmetic addition and assignment | `AddAssign` | ``
  - Line 39, the matching row for `-=`:
    `Arithmetic subtraction and assignment`.
  - Line 30, the matching row for `*=`:
    `Arithmetic multiplication and assignment`.
  - Line 50, the matching row for `/=`:
    `Arithmetic division and assignment`.
  Together these four rows give the lesson its claim that `+=`, `-=`,
  `*=`, `/=` form one shape: `var OP= expr` for each of lesson 009's
  operators.

  Calibration:
  - The same table also includes bitwise compound forms (`&=`, `|=`,
    `^=`, `<<=`, `>>=`) and `%=`. This lesson defers all of them; see
    What To Ignore For Now.
  - The "Overloadable?" column lists trait names (`AddAssign`,
    `SubAssign`, `MulAssign`, `DivAssign`). Traits are out of scope
    for this lesson.

- The local probe (transcript below).

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/023-compound-add-assign.rs`.
The committed file is the working program. There is no broken
contrast file; the load-bearing observation is that the printed value
is `n = 6`, matching the desugared sum `0 + 1 + 2 + 3`.

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
    let mut n = 0;
    n += 1;
    n += 2;
    n += 3;
    println!("n = {n}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
n = 6
exit=0
--- temp dir removed ---
```

Notes:

- `rustc` exits 0 silently (lesson 001).
- `./demo` prints `n = 6`. The three `n += ...;` statements run in
  source order (lesson 004), each one updating the same `mut`
  binding (lesson 006), with `+` semantics from lesson 009. Adding
  `1 + 2 + 3` to a starting value of `0` yields `6`, which is what
  the executable prints.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — `;`-terminated statements
  in `fn main` execute top to bottom; the three `n += ...;` lines
  apply in that order.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name; `println!("... {name} ...")` substitutes the bound
  value at print time.
- `006-mut-binding` (accepted, load-bearing) — `let mut name = value;`
  makes the binding reassignable, and bare `name = new_value;`
  reassigns it. `n += value;` is a flavor of reassignment, so it
  inherits the `mut` requirement.
- `009-arithmetic-on-integers` (accepted, load-bearing) — `+`, `-`,
  `*`, `/` combine two integer values. The desugared form
  `n = n + value;` uses lesson 009's `+`; the other compound forms
  reuse the matching operator.
