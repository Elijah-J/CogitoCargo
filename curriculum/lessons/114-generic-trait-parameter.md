---
id: 114-generic-trait-parameter
status: accepted
evidence: ../evidence/114-generic-trait-parameter.md
---

# Declare a trait with a type parameter `<RHS>`; substitute a concrete type in the impl header

## The Move

Lessons 111-113 wrote concrete trait declarations: every type
inside the trait body was a specific type the learner spelled out
(`u32`, `&Counter`). Today the trait declaration grows a *type
parameter* — a placeholder for a type that an `impl` will choose
later. Three coupled new pieces in one program:

```rust
struct Counter {
    count: u32,
}

trait AddRhs<RHS> {
    fn add(&self, rhs: RHS) -> u32;
}

impl AddRhs<u32> for Counter {
    fn add(&self, rhs: u32) -> u32 {
        self.count + rhs
    }
}

fn main() {
    let c = Counter { count: 7 };
    let total = c.add(35);
    println!("total = {}", total);
}
```

`./demo` prints `total = 42`. Walk the three new tokens:

1. **`<RHS>` after the trait name in the declaration** —
   `trait AddRhs<RHS> { ... }`. The Reference: "Type parameters
   can be specified for a trait to make it generic. These appear
   after the trait name." `RHS` is an ordinary uppercase
   identifier (convention is short uppercase: `T`, `U`, `RHS`).
   Inside the trait body, `RHS` is now a name in scope, usable in
   any *type position*.

2. **`RHS` used as a type inside the trait method's signature** —
   `fn add(&self, rhs: RHS) -> u32;`. Where lessons 112-113 wrote
   `u32` or `&Counter` after the colon, the declaration now writes
   the placeholder. The trait body does not yet know what concrete
   type `RHS` will be; that decision is the impl's.

3. **`<u32>` after the trait name in the impl header** —
   `impl AddRhs<u32> for Counter { ... }`. The angle brackets here
   *substitute* the placeholder. Inside this impl block, every
   `RHS` from the trait declaration is replaced by `u32`. The impl
   method must therefore write `fn add(&self, rhs: u32) -> u32`,
   by lesson 112's contract-matching rule applied after
   substitution.

The three pieces are coupled. Without `<RHS>` in the trait
header, `RHS` in the body is a name out of scope. Without `<u32>`
in the impl header, the substitution never happens — the centered
contrast below.

The same `trait AddRhs<RHS>` could be impl'd for `Counter` more
than once with different substitutions
(`impl AddRhs<u32> for Counter`, `impl AddRhs<u64> for Counter`,
...), each attaching its own `add`. That dispatch power is *the*
reason traits are generic; today's probe shows only one
substitution. You have already seen the angle-bracket shape in
*uses* — `Vec<u64>` (lesson 107), `Option<T>`. Today is the
*declaration* side of the same syntax.

## Mental Model Delta

- *Before*: "Every type inside a trait body is a specific named
  type the impl reproduces verbatim. The angle-bracket shape on
  `Vec<u64>` and `Option<T>` came from the standard library."
- *After*: "A trait declaration may carry a type parameter
  between angle brackets after the trait name. Inside the body,
  that name stands in any type position. The impl header
  substitutes a concrete type for it; the impl method body uses
  the concrete type. The same trait can be impl'd for the same
  target type with different substitutions — that is the dispatch
  power generics give traits."

## Prerequisites

- Installed concepts:
  - **Lesson 113** (load-bearing): trait method with a non-receiver
    parameter slot. Today the parameter type slot holds a type
    variable `RHS` instead of a concrete type.
  - **Lesson 112** (load-bearing): the contract-matching rule. The
    impl method's signature must match the trait method's signature
    *after* the impl header's substitution.
  - **Lesson 111** (load-bearing): `trait Name { ... }` and
    `impl Trait for Type { ... }`. Today extends both headers with
    angle brackets.
  - **Lesson 095** (load-bearing): `struct Counter { count: u32 }`
    and `self.count`.
  - **Lesson 100** (load-bearing): `&self`.
  - **Lesson 040** (load-bearing): the dot-call shape `c.add(35)`.
  - **Lesson 008** (load-bearing): the `(p1: T1, p2: T2)`
    parameter-list grammar; today fills one type slot with `RHS`.
  - **Lesson 107** (cited): `Vec<u64>` introduced the angle-bracket
    shape on a *type* used by the learner; today is the
    declaration side.
  - **Lessons 001, 002, 003, 005, 009 (`+`), 011, 019, 080
    (`u32`)** (cited): `rustc demo.rs && ./demo`, `fn main`,
    diagnostic map, `let`, addition, `println!` `{}`,
    type-annotation slot, `u32`.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
total = 42
```

Three new tokens vs. lesson 113: `<RHS>` after the trait name in
the declaration; `RHS` where a concrete type used to sit in the
parameter list; `<u32>` after the trait name in the impl header.

*Now the contrast.* Save `no_arg.rs` — same source, but drop the
`<u32>` from the impl header (`impl AddRhs for Counter`). The
trait still declares one type parameter; the impl supplies zero:

```
error[E0107]: missing generics for trait `AddRhs`
 --> no_arg.rs:9:6
  |
9 | impl AddRhs for Counter {
  |      ^^^^^^ expected 1 generic argument
  |
note: trait defined here, with 1 generic parameter: `RHS`
 --> no_arg.rs:5:7
  |
5 | trait AddRhs<RHS> {
  |       ^^^^^^ ---
help: add missing generic argument
  |
9 | impl AddRhs<RHS> for Counter {
  |            +++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0107`.
```

Read with the lesson 003 map. Headline: E0107 —
`missing generics for trait`, a new E-code. Caret under `AddRhs`
at the impl header, label `expected 1 generic argument`. The
`note: trait defined here, with 1 generic parameter: `RHS``
points back at the trait declaration — the contract telling the
impl how many type arguments it owes. The `help: add missing
generic argument` block writes `RHS` (the placeholder); a learner
would substitute a concrete type like `u32` instead.

## What Changed

- `trait Name<T> { ... }` declares a trait with one type
  parameter. The angle-bracket slot is the same one `Vec<T>` and
  `Option<T>` use; today is the *declaration* side.
- Inside the trait body, the type parameter (`RHS`, `T`, ...) may
  sit in any type position.
- `impl Trait<ConcreteType> for Type { ... }` substitutes the
  placeholder. The impl method body uses the concrete type by
  lesson 112's contract-matching rule.
- One trait declaration may be impl'd for one target type with
  several different substitutions; today's probe shows only one.
- Without `<ConcreteType>` in the impl header, rustc fires E0107
  with `expected 1 generic argument` and a `note: trait defined
  here, with 1 generic parameter` block pointing at the trait.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait Combine<X> {
    fn combine(&self, x: X) -> u32;
}

impl Combine<u32> for Tally {
    fn combine(&self, x: u32) -> u32 {
        self.n + x
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("combined = {}", t.combine(5));
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you change *only* the impl header from `impl Combine<u32> for Tally`
to `impl Combine for Tally` — leaving everything else unchanged —
what E-code appears, what does the inline label say, and what does
the `note:` block name and point at?

*(Answers: (a) Yes. (b) `combined = 15`. (c) E0107; inline label
`expected 1 generic argument`; the `note: trait defined here, with
1 generic parameter: `X`` block points at the trait declaration
`trait Combine<X>`.)*

## What To Ignore For Now

Today installs one type parameter on one trait, one concrete
substitution at one impl. Deferred:

- **Multiple impls of the same trait for the same target type with
  different substitutions** — `impl AddRhs<u32> for Counter`
  alongside `impl AddRhs<u64> for Counter`. *The* dispatch power
  of generic traits; named, not probed.
- **Multiple type parameters on one trait** — `trait Pair<A, B>`.
- **Generic functions** (`fn f<T>(t: T)`) and **generic struct
  types** (`struct S<T>`) — distinct mechanics that reuse the
  same `<T>` slot, on function/struct headers respectively.
  `Vec<T>` and `Option<T>` are *uses* of std-declared generic
  types of this kind.
- **Trait bounds** — `trait T<U: SomeBound>` and `where`
  clauses. Constraints on substitutable types.
- **Default type parameters** — `trait Add<Rhs = Self>`, what
  `std::ops::Add` actually writes.
- **Associated types** — `type Output = ...;`. The next mechanic
  on the path to reading rmp's `Add` impl.
- **Lifetime parameters** — `trait T<'a>` and `&'a Counter`.
- **The orphan rule** for generic impls.
- **`Self` in the type-parameter slot** (composes with default
  type parameters).
- **Trait method receivers other than `&self`** — compose into
  generic traits identically.

## Evidence

See `../evidence/114-generic-trait-parameter.md`.
