---
id: 115-trait-associated-type
status: accepted
evidence: ../evidence/115-trait-associated-type.md
---

# Declare an associated type `type Output;` in a trait; resolve it `type Output = u32;` in the impl

## The Move

Lesson 114 declared a trait with a *type parameter* `<RHS>` after
the trait name; the impl chose a concrete type at the impl header.
Today installs a dual mechanic: the trait may include its own type
slot — an *associated type* — that the impl fills *inside the impl
block*. Three coupled new pieces:

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    type Output;
    fn doubled(&self) -> Self::Output;
}

impl Doubled for Counter {
    type Output = u32;
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

`./demo` prints `doubled = 42`. Walk the three new tokens:

1. **`type Output;` inside the trait body** — a *required associated
   type*. The syntax mirrors a method signature: `type IDENTIFIER;`
   ends in `;`, declaring a contract the impl must fulfill. The
   Reference: "Associated types must never define the type, the type
   may only be specified in an implementation." The trait does not
   say what `Output` *is*; only that there is one.

2. **`Self::Output` as a type-position path** — used in
   `fn doubled(&self) -> Self::Output;`. Inside the trait body,
   `Self::Output` is the *path* to the associated type and can sit
   anywhere a type is expected. Inside the trait declaration, it is
   the *only* way to refer to the associated type, because the trait
   does not yet know what concrete type it will become.

3. **`type Output = u32;` inside the impl block** — the
   *resolution*. The impl body lists each associated type the trait
   declared and assigns a concrete type with `type IDENTIFIER = Type;`.

After resolution, `Self::Output` *is* `u32` inside this impl block.
The impl method's signature can write either spelling — `-> u32` or
`-> Self::Output`. Both compile. Today's probe uses the concrete-type
form because rmp's `add.rs:115` does
(`fn add(self, rhs: &BigUInt) -> BigUInt`).

The contrast with 114 is the heart of today's move. A generic
parameter `<RHS>` is filled at the impl *header*; one trait + one
target type can carry many different `<RHS>` choices. An associated
type is filled *inside the impl body* and is unique per impl: one
trait + one target type = one resolved `Output`.

## Mental Model Delta

- *Before*: "A trait exposes a type slot via a generic parameter
  `<RHS>` after the trait name; the impl header writes the concrete
  type."
- *After*: "A trait exposes a *second* kind of type slot: declare
  `type Output;` as a required item inside the trait body, reference
  it via `Self::Output`, and resolve it via `type Output = T;` inside
  the impl block. Generic parameters are chosen at the impl *header*
  and may differ across impls; associated types are chosen inside
  the impl *body* and are unique per (trait, target type) pair.
  After resolution, `Self::Output` is just an alias for the concrete
  type."

## Prerequisites

- Installed concepts:
  - **Lesson 114** (load-bearing): trait declarations can carry
    fillable type slots. 114's was a generic parameter `<RHS>`
    filled at the *impl header*; today's is an associated type
    filled *inside the impl body*.
  - **Lesson 112** (load-bearing): the *contract-matching rule*.
    Today extends it post-resolution: after `type Output = u32;`
    resolves, `Self::Output` *is* `u32`, and the impl method must
    match either spelling.
  - **Lesson 111** (load-bearing): `trait Name { ... }` and
    `impl Trait for Type { ... }`. Today places one new associated
    item inside each block.
  - **Lesson 100** (load-bearing): `Self` is a *type alias* inside
    an impl block. Today's `Self::Output` is a path *through* that
    alias to a name the trait declared.
  - **Lesson 095** (load-bearing): `struct Counter { count: u32 }`
    and `self.count` field access.
  - **Lesson 040** (load-bearing): the dot-call shape `c.doubled()`.
  - **Lesson 008** (load-bearing): `fn name(p: T) -> R { ... }` —
    today's signatures slot `Self::Output` or `u32` into the
    return-type position.
  - **Lessons 002, 005, 009 (`*`), 011, 019, 080, 003, 001** (cited):
    unchanged from prior usage. The evidence appendix's *Older
    supporting lessons* section carries the topical detail.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
doubled = 42
```

Three new tokens vs. lesson 114: `type Output;` in the trait body
(semicolon, like a method signature with no body); `Self::Output` in
the trait method's return type; `type Output = u32;` inside the
impl body.

*Now the contrast.* Save `no_type.rs` — same source, but drop the
`type Output = u32;` line from the impl block. The trait still
declares `type Output;` as a required item; the impl supplies zero:

```
error[E0046]: not all trait items implemented, missing: `Output`
  --> no_type.rs:10:1
   |
 6 |     type Output;
   |     ----------- `Output` from trait
...
10 | impl Doubled for Counter {
   | ^^^^^^^^^^^^^^^^^^^^^^^^ missing `Output` in implementation

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
```

Read with the lesson 003 map. Headline: E0046 — new code,
*not all trait items implemented, missing: `Output`*. Two `-->`
blocks: one underlines the trait's `type Output;` line as ``Output`
from trait` (the contract item), the other underlines the impl
header as `missing `Output` in implementation`. The diagnostic
names the missing item by identifier — the same shape lesson 111's
E0599 used for a missing *method*. Without the `type Output = u32;`
line, the impl is incomplete.

## What Changed

- The trait body now has a third item kind alongside functions and
  constants — an *associated type* declared with `type IDENTIFIER;`,
  referenced via `Self::IDENTIFIER`, and resolved with
  `type IDENTIFIER = ConcreteType;` inside the impl.
- The impl method's signature can write either the concrete type or
  `Self::Output`; both compile, and rmp uses the concrete-type style.
- Without the `type IDENTIFIER = ConcreteType;` line, rustc fires
  E0046 with `missing: `IDENTIFIER`` and points at the trait
  declaration as the contract.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait Halved {
    type Output;
    fn halved(&self) -> Self::Output;
}

impl Halved for Tally {
    type Output = u32;
    fn halved(&self) -> u32 {
        self.n / 2
    }
}

fn main() {
    let t = Tally { n: 50 };
    println!("halved = {}", t.halved());
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you delete *only* the line `type Output = u32;` from inside
the impl block — leaving the trait, the impl method, and `fn main`
unchanged — what E-code appears, and what identifier does the
headline name as the missing item?

*(Answers: (a) Yes. (b) `halved = 25`. (c) E0046; the headline reads
`not all trait items implemented, missing: `Output``, and the
diagnostic points at the trait body's `type Output;` line with the
label `Output` from trait.)*

## What To Ignore For Now

Today installs one associated type, no default, no bounds, used in
return-type position only. Deferred:

- **Default associated types** — `type Output = u64;` *inside the
  trait body* declares a default the impl may override.
- **Multiple associated types** — `type A; type B;`.
- **Associated types with trait bounds** — `type Output: Display;`.
  Blocked on trait bounds.
- **Generic associated types (GATs)** — `type Item<'a>;`.
- **Associated types as parameter types** — same path syntax,
  different position.
- **Cross-impl qualified path** — `<Counter as Doubled>::Output`.
- **Default type parameters on the trait** — `trait T<U = Self>` —
  which `std::ops::Add` uses together with an associated type.
- **Multiple impls dispatching different `Output` types** — the
  payoff. Named, not probed today.
- **Operator traits from `std::ops`** — compose today's mechanic
  with 114's generic parameter and a default type parameter. The
  capstone rmp `add.rs:112-115`
  `impl Add<&BigUInt> for &BigUInt { type Output = BigUInt; ... }`
  combines 114's mechanic and today's; the `type Output = BigUInt;`
  line is fully readable after today.
- **The orphan rule, `where` clauses, lifetimes** — wholesale from
  111/114.

## Evidence

See `../evidence/115-trait-associated-type.md`.
