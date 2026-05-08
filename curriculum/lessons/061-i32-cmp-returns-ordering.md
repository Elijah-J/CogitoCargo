---
id: 061-i32-cmp-returns-ordering
status: accepted
evidence: ../evidence/061-i32-cmp-returns-ordering.md
---

# Compare two `i32` values with `a.cmp(&b)` and match on the `Ordering`

## The Move

A pure composition cycle. Three previously-installed cycles snap
together to give a single new sentence: *"to ask which of two
integers is smaller, call `a.cmp(&b)` and `match` on the answer."*

```rust
use std::cmp::Ordering;

fn main() {
    let a: i32 = 3;
    let b: i32 = 5;
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
```

`rustc demo.rs && ./demo` prints `a < b` and exits 0. Re-run with
`a = 3, b = 3` to get `a == b`; with `a = 7, b = 5` to get
`a > b`. Three different inputs reach all three arms.

Three pieces, all already installed:

- `a.cmp(&b)` is cycle 040's *method-call form* — receiver,
  dot, method name, parenthesized argument list.
- `&b` is cycle 045's *prefix-`&` operator* applied to the
  binding `b`, producing a value of type `&i32`.
- The whole call expression has type `Ordering` (cycle 051),
  and the `match` arms use the three variant patterns
  `Ordering::Less`, `Ordering::Greater`, `Ordering::Equal`.

The one new fact: there is a method named `cmp` on `i32` with
signature `fn cmp(&self, other: &Self) -> Ordering` (per the
std page for the `Ord` trait, which `i32` implements). It is
a method (cycle 040), takes its second argument by shared
reference (cycle 045), and returns an `Ordering` value
(cycle 051).

## Mental Model Delta

- *Before:* "I know three things separately. (a) Methods on a
  value are called `value.method(args)`. (b) `&value` produces a
  shared reference. (c) `Ordering` has three variants and `match`
  picks one. I have not seen them snap together."
- *After:* "Two `i32` values can be compared three-way with
  `a.cmp(&b)`. The result is an `Ordering`, the same enum from
  cycle 051. Plug it into a `match` and you choose between
  *less*, *greater*, and *equal* paths in one expression. This is
  the alternative to cycle 013's two-way comparison operators
  (`<`, `==`, `>`, ...): when you want three branches in one
  go, reach for `cmp`. The Book's guessing game uses this exact
  shape."

## Prerequisites

- Installed concepts:
  - Cycles 001, 002, 005, 019: compile/run shape, `fn main`,
    `let name: i32 = value;`.
  - Cycle 040 (load-bearing): the dot-form method call
    `receiver.method(args)`.
  - Cycle 044 (load-bearing): `use std::cmp::Ordering;` brings
    the short variant name into scope (same shape as cycle 051).
  - Cycle 045 (load-bearing): `&value` produces a `&T` value;
    `T` and `&T` are distinct types; passing `T` where `&T` is
    expected fires E0308. Today's broken contrast reuses this.
  - Cycle 051 (load-bearing): the `Ordering` enum, its three
    variants, and `match scrutinee { Variant => ... }` with
    exhaustiveness via E0004.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell (same as cycle 001).

## Try It

Save the snippet above as `demo.rs`.

```console
$ rustc demo.rs
$ ./demo
a < b
```

Edit two lines and re-run:

- `a = 3, b = 3` prints `a == b`.
- `a = 7, b = 5` prints `a > b`.

The three runs reach all three arms, confirming that `cmp`
distinguishes the cases empirically. The std `Ord` page's
convention sentence makes it verbal: *"`self.cmp(&other)`
returns the ordering matching the expression
`self <operator> other` if true."* So `3.cmp(&5)` is
`Ordering::Less` exactly when `3 < 5`.

Now the broken contrast. *Predict*: drop the `&` and write
`a.cmp(b)` instead. Recompile:

```
error[E0308]: mismatched types
 --> broken.rs:6:17
  |
6 |     match a.cmp(b) {
  |             --- ^ expected `&i32`, found `i32`
  |             |
  |             arguments to this method are incorrect
  |
note: method defined here
 --> /rustc/.../library/core/src/cmp.rs:999:7
help: consider borrowing here
  |
6 |     match a.cmp(&b) {
  |                 +
```

Same E-code as cycles 045 and 046's broken contrasts —
*mismatched types*, this time with `expected `&i32`, found
`i32`` on the argument. `help: consider borrowing here`
source-diffs the missing `&`. The `note: method defined here`
sub-diagnostic is the dual-`-->` shape cycle 036 first
captured.

(Full transcripts are in `../evidence/061-i32-cmp-returns-ordering.md`.)

## What Changed

- You can compare two `i32` values three-way: `a.cmp(&b)`
  returns an `Ordering` value.
- You know one new method, `i32::cmp`, with signature
  `fn cmp(&self, other: &Self) -> Ordering`. The second
  argument is taken by shared reference, so the call site
  writes `&b`, not bare `b`.
- The natural use is feeding the result into a `match` against
  the three variants — exactly cycle 051's shape, with the
  scrutinee now produced by a method call.
- You have a second tool for comparing integers alongside
  cycle 013's `<`, `==`, `>` operators. The operators each
  return a `bool` (two-way branch); `cmp` returns an
  `Ordering` (three-way branch in one `match`). Pick the
  shape that matches the decision.
- Failure mode: dropping the `&` (writing `a.cmp(b)`) fires
  E0308 *mismatched types* — same E-code as cycles 045/046,
  with `help: consider borrowing here` source-diffing the `&`.

## Check Yourself

You write `pred.rs`:

```rust
use std::cmp::Ordering;

fn main() {
    let x: i32 = 10;
    let y: i32 = 4;
    match x.cmp(&y) {
        Ordering::Less => println!("smaller"),
        Ordering::Greater => println!("bigger"),
        Ordering::Equal => println!("same"),
    }
}
```

(a) Does `rustc pred.rs` accept the program? What does
`./pred` print?

(b) If you replaced line 6 with `match x.cmp(y) {`, which
E-code would the headline carry, and what would `help:`
suggest?

(c) Name a pair of `i32` values for `x` and `y` that would
print `same`.

*(Answers: (a) Yes — same shape, different binding names.
Prints `bigger` because `10 > 4`. (b) E0308 *mismatched
types* with `expected `&i32`, found `i32``; `help:` reads
"consider borrowing here" and source-diffs `&` in front of
`y`. (c) Any pair where the two values are equal, e.g.
`x = 5, y = 5`. By the std `Ord` page's convention,
`x.cmp(&y) == Ordering::Equal` exactly when `x == y`.)*

## What To Ignore For Now

- *The `Ord` and `PartialOrd` traits.* `cmp` is declared by
  the `Ord` trait, which `i32` implements — that is how the
  method is reachable on `i32`. Trait machinery has been
  deferred since cycle 040; the lesson uses `cmp` operationally
  without surfacing the trait noun.
- *`PartialOrd::partial_cmp` returning `Option<Ordering>`.*
  For types that include values like `f64::NAN`, comparison
  can fail — `partial_cmp` returns `None`. `i32` has no such
  values, so today's `cmp` always returns a real `Ordering`.
- *`String::cmp` and `&str::cmp`.* Same shape as `i32::cmp`
  but on strings, in lexicographic order. Defer.
- *`Ordering`'s methods* — `is_eq`, `is_lt`, `reverse`,
  `then`, etc. Today's lesson reaches the variants only via
  `match`.
- *Method dispatch via traits.* When `a.cmp(&b)` is compiled,
  Rust resolves `cmp` through `Ord::cmp`. Defer.
- *Generic functions taking `T: Ord`.* Defer.
- *`cmp` on tuples, structs, `Vec<T>`.* Comparison
  lexicographically by element/field; defer.
- *`std::cmp::min` and `std::cmp::max`* (cycle 043 already
  installed `min`). Sibling free functions; not relevant
  today.
- All previously deferred items.

## Evidence

See `../evidence/061-i32-cmp-returns-ordering.md` for the
corpus-quote map, the toolchain string, the working probe
transcripts (Less / Equal / Greater), the broken-contrast
E0308 transcript, and the prerequisite-claim summary.
