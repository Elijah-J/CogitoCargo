---
id: 113-reference-parameter-in-trait-method
status: accepted
evidence: ../evidence/113-reference-parameter-in-trait-method.md
---

# Take `&Counter` as a non-receiver parameter; pass `&b` at the call site

## The Move

Lessons 100 and 102 installed two *receiver* shapes — `&self` (the
borrowing receiver) and `self` by value (the consuming receiver).
Lesson 112 installed a trait method with one ordinary primitive
parameter beyond `&self`. Today extends that parameter slot to a
*reference* type, and shows the matching call-site shape:

```rust
struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: &Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: &Counter) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let first  = a.combine(&b);
    let second = a.combine(&b);
    println!("first  = {}", first);
    println!("second = {}", second);
    println!("b.count still = {}", b.count);
}
```

`./demo` prints `first  = 42`, `second = 42`, `b.count still = 35`.
Two coupled new pieces:

1. **`other: &Counter`** in the trait method's parameter list —
   and the same `&Counter` in the impl's matching method body. The
   parameter type starts with `&`; that ampersand makes the
   parameter a *reference*. Inside the body, `other.count` reads
   the field through the reference, the same way `self.count`
   reads through `&self` (lesson 100 installed that mechanic).
2. **`&b`** at the call site. The argument expression starts with
   `&`. The Book: "the `&s1` syntax lets us create a reference that
   *refers* to the value of `s1` but does not own it. Because the
   reference does not own it, the value it points to will not be
   dropped when the reference stops being used." Translation: `&b`
   makes a reference to `b`, and `b` is still usable after the call.

The two-call pattern is the witness for "still usable." The first
`a.combine(&b)` makes a reference, runs the method, and returns.
`b` is still owned by `main`. The second `a.combine(&b)` makes
another reference and runs again. The final `println!` reads
`b.count` directly — `b` is alive and well.

This is the same `&` you already saw in `&self`. The Reference
calls `&self` shorthand for `self: &Self`. Today's `other: &Counter`
is the same `&Type` shape, just in a non-receiver slot and with the
type spelled explicitly. (Inside this impl `Self` is an alias for
`Counter`, so `other: &Self` would mean the same thing — a stylistic
choice the rmp source uses both ways.)

## Mental Model Delta

- *Before*: "`&self` is what makes the dot call resolve; trait
  methods can take ordinary parameters like `factor: u32`. References
  are something I've seen in `&self` but only as a receiver."
- *After*: "`&Type` is a *reference type* — and it can sit in *any*
  parameter slot of any function or method, not just the receiver.
  `&self` was the special-cased receiver case all along. The matching
  call-site argument is `&value`. A `&value` argument *borrows*: the
  caller still owns `value` after the call, so the same `&value` can
  be passed again. Without `&`, the caller passes the value itself —
  which is a different type and rustc fires E0308."

## Prerequisites

- Installed concepts:
  - **Lesson 112** (load-bearing): the trait method shape
    `fn name(&self, p: T) -> R;` and the rule that the impl's
    signature reproduces the trait's exactly. Today the parameter
    type is a reference rather than a primitive; the contract-matching
    rule is unchanged.
  - **Lesson 100** (load-bearing): `&self` as the borrowing
    receiver, `self.field` as field access through the reference,
    and `Self` as the type alias inside an impl. Today extends the
    `&Type` shape from the receiver to a non-receiver parameter,
    and `other.field` reuses the same auto-deref-on-field-access.
  - **Lesson 095** (load-bearing): `struct Name { field: Type }`,
    struct expression, field access. The probe builds two `Counter`
    values and reads `.count` after the calls.
  - **Lesson 008** (load-bearing): the `(p1: T1, p2: T2)`
    parameter-list grammar. Today's `(other: &Counter)` slot is the
    same shape with `&Counter` filling the type position.
  - **Lesson 040** (load-bearing): the dot-call shape
    `value.method(arg)`. The argument expression is now `&b` rather
    than a bare value.
  - **Lessons 002, 005, 009 (`+`), 011, 019, 080** (cited):
    `fn main`, `let`, addition, `println!` `{}`, type-annotation
    slot, `u32`.
  - **Lesson 003** (cited): the four-part diagnostic map.
  - **Lesson 001** (cited): `rustc demo.rs` then `./demo`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
first  = 42
second = 42
b.count still = 35
```

Three new tokens vs. lesson 112: the `&` in front of `Counter` in the
trait signature, the `&` in front of `Counter` in the impl
signature, and the `&` in front of `b` at the call site. The first
two are the *type* `&Counter`; the third is the *expression* `&b`
producing a reference to `b`.

*Now the contrast.* Save `no_amp.rs` — same source, but drop the `&`
from the call-site argument: `a.combine(b)` instead of `a.combine(&b)`.
Trait and impl signatures unchanged. Compile:

```
error[E0308]: mismatched types
  --> no_amp.rs:18:27
   |
18 |     let first = a.combine(b);
   |                   ------- ^ expected `&Counter`, found `Counter`
   |                   |
   |                   arguments to this method are incorrect
   |
note: method defined here
  --> no_amp.rs:6:8
   |
 6 |     fn combine(&self, other: &Counter) -> u32;
   |        ^^^^^^^        -----
help: consider borrowing here
   |
18 |     let first = a.combine(&b);
   |                           +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Read with the lesson 003 map. Headline: E0308 — *mismatched types*.
Caret under `b` at the call site. The inline label says
`expected `&Counter`, found `Counter`` — verbatim, the rule today
installs. The `note: method defined here` block points back at the
trait declaration. The `help: consider borrowing here` proposes
inserting a single `+`: the `&` that converts the bare `b` into a
`&b` reference. Without that `&`, the value `b` has type `Counter`,
not `&Counter`, and the call doesn't typecheck.

## What Changed

- `&Type` is a *reference type*; it can appear in any parameter
  slot, not just the receiver.
- The call-site argument for a `&Type` parameter is `&value`. The
  caller still owns `value` after the call.
- `other.count` reads a field through a reference, the same way
  `self.count` reads through `&self`.
- Without the `&` at the call site, rustc fires E0308 with the
  inline label `expected `&Counter`, found `Counter`` and a
  `help: consider borrowing here` proposing the `&` insertion.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait Sum { fn sum(&self, other: &Tally) -> u32; }

impl Sum for Tally {
    fn sum(&self, other: &Tally) -> u32 {
        self.n + other.n
    }
}

fn main() {
    let x = Tally { n: 10 };
    let y = Tally { n: 5 };
    let s1 = x.sum(&y);
    let s2 = x.sum(&y);
    println!("s1 = {}, s2 = {}, y.n = {}", s1, s2, y.n);
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you change *only* the second call site from `x.sum(&y)` to
`x.sum(y)` — leaving the trait, impl, and first call site alone —
what E-code appears, what does the inline label say, and what does
the `help:` block propose?

*(Answers: (a) Yes. (b) `s1 = 15, s2 = 15, y.n = 5`. (c) E0308;
inline label `expected `&Tally`, found `Tally``; the `help:` block
proposes inserting `&` before the argument — `x.sum(&y)`.)*

## What To Ignore For Now

Today installs one `&Type` parameter on a trait method, one
`&value` call site, the witness that the caller still owns the
value. Deferred:

- **`&Self` vs `&Counter` inside the impl** — same meaning here
  (`Self` is the type alias from lesson 100). The rmp source uses
  both; today centers the named-type form because it is directly
  readable.
- **`&mut Type` in non-receiver slots** — composes lesson 101's
  `&mut self` into a parameter slot. Distinct mechanic.
- **The full borrowing rules** — the borrow checker, shared-vs-mut
  exclusivity, lifetime parameters like `&'a Counter`, `'static`.
  All deferred wholesale.
- **Multiple reference parameters in one signature**, and
  **reference-of-reference** `&&Counter`. Deferred.
- **`&Type` as a return type** — `fn first(&self) -> &Counter`
  triggers lifetime questions. Deferred.
- **Other receivers in trait methods** (`&mut self`, `self` by
  value) — compose identically; not exercised today.
- **Multi-type dispatch, default method bodies, generic trait
  parameters, associated types, operator traits** — all named in
  111/112 and unchanged today.

## Evidence

See `../evidence/113-reference-parameter-in-trait-method.md`.
