---
id: 112-trait-method-extra-parameter
status: accepted
evidence: ../evidence/112-trait-method-extra-parameter.md
---

# Add a parameter beyond `&self` to a trait method, and match it exactly in the impl

## The Move

Lesson 111's trait method took only `&self`. Today extends the
signature with one extra parameter, and centers a new rule: the
impl's signature must match the trait's *exactly*.

```rust
struct Counter {
    count: u32,
}

trait Scale {
    fn scaled(&self, factor: u32) -> u32;
}

impl Scale for Counter {
    fn scaled(&self, factor: u32) -> u32 {
        self.count * factor
    }
}

fn main() {
    let c = Counter { count: 7 };
    println!("scaled = {}", c.scaled(6));
}
```

`./demo` prints `scaled = 42`. Only one piece is new — the
parameter list `(&self, factor: u32)`. After the comma sits the
ordinary `name: Type` parameter shape lesson 008 installed for free
functions; the trait body and impl body each use it. The body
multiplies `self.count` by `factor` (field access from 095, `*`
from 009, parameter binding from 008). The call site `c.scaled(6)`
is the dot-call shape from lesson 040 with the argument case
exercised: `6` lands on `factor`, `c` fills `&self`.

The new teaching is the *contract-matching rule*. The trait
declaration is binding: the impl must reproduce the parameter list
and types verbatim. The E0053 page: "The parameters of any trait
method must match between a trait implementation and the trait
definition."

## Mental Model Delta

- *Before*: "A trait method has `&self` and returns a type. The
  impl writes the same signature with a body."
- *After*: "A trait method's parameter list is
  `(&self, p1: T1, ...)` — the same `name: Type` parameter shape
  free functions use, slotted after `&self`. The impl must
  reproduce that parameter list *exactly*. Names may differ; types
  may not. On mismatch, rustc fires E0053 with a `note: type in
  trait` block pointing at the trait as the contract."

## Prerequisites

- Installed concepts:
  - **Lesson 111** (load-bearing): `trait Name { fn method(&self) -> T; }`,
    `impl Trait for Type { ... }`, and the rule that impl signatures
    match trait signatures. Today's only diff is the extra
    parameter slot.
  - **Lesson 008** (load-bearing): the `(p1: T1, p2: T2)` parameter
    list. Today reuses it after `&self,`.
  - **Lesson 040** (load-bearing): the dot call `value.method(arg)`,
    now exercised on a trait method.
  - **Lesson 095** (load-bearing): `struct Name { field: Type }` and
    `self.field`; reused in the impl body.
  - **Lesson 100** (cited): `&self` receiver and the
    inherent-vs-trait impl distinction.
  - **Lessons 002, 005, 009 (`*`), 011, 019, 080 (`u32`)** (cited):
    `fn main`, `let`, multiplication, `println!` `{}`,
    type-annotation slot, integer family.
  - **Lesson 003** (cited): the four-part diagnostic map.
  - **Lesson 001** (cited): `rustc demo.rs` then `./demo`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
scaled = 42
```

*Now the contrast.* Save `mismatch.rs` — same source, but change
the trait's `factor` type from `u32` to `u64` (one character). The
impl still says `factor: u32`; the trait now says `factor: u64`.
The signatures don't match. E0053 fires:

```rust
struct Counter {
    count: u32,
}

trait Scale {
    fn scaled(&self, factor: u64) -> u32;
}

impl Scale for Counter {
    fn scaled(&self, factor: u32) -> u32 {
        self.count * factor
    }
}

fn main() {
    let c = Counter { count: 7 };
    println!("scaled = {}", c.scaled(6));
}
```

Compile:

```
error[E0053]: method `scaled` has an incompatible type for trait
  --> mismatch.rs:10:30
   |
10 |     fn scaled(&self, factor: u32) -> u32 {
   |                              ^^^ expected `u64`, found `u32`
   |
note: type in trait
  --> mismatch.rs:6:30
   |
 6 |     fn scaled(&self, factor: u64) -> u32;
   |                              ^^^
   = note: expected signature `fn(&Counter, u64) -> _`
              found signature `fn(&Counter, u32) -> _`
help: change the parameter type to match the trait
   |
10 -     fn scaled(&self, factor: u32) -> u32 {
10 +     fn scaled(&self, factor: u64) -> u32 {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0053`.
```

Read with the lesson 003 map. Headline: E0053 — new code, name
`method has an incompatible type for trait`. Caret at the impl's
`u32` on line 10. The `note: type in trait` block points back at
line 6 with caret on the trait's `u64` — the contract's source of
truth. The `= note:` line summarizes both signatures side by side.
The `help:` block proposes the exact edit. The impl is the
variable; the trait is fixed.

## What Changed

- A trait method's parameter list extends past `&self` using the
  lesson 008 `name: Type` shape: `fn scaled(&self, factor: u32) -> u32;`.
- The impl signature must match the trait's exactly. Names may
  differ; types may not.
- E0053 is the diagnostic for a trait-method signature mismatch.
  The `note: type in trait` block points at the trait — the
  contract — and the impl must conform.
- The dot call `c.scaled(6)` is unchanged from lesson 040: `6`
  fills `factor`, `c` fills `&self`.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait Plus { fn plus(&self, addend: u32) -> u32; }

impl Plus for Tally {
    fn plus(&self, addend: u32) -> u32 {
        self.n + addend
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("plus = {}", t.plus(5));
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you change the trait declaration's parameter type from
`addend: u32` to `addend: i32` (the impl is unchanged, so its body
`self.n + addend` still typechecks as `u32 + u32`), what E-code
appears, and what does the `note:` block name and point at?

*(Answers: (a) Yes. (b) `plus = 15`. (c) E0053; the caret sits on
the impl's `addend: u32` and a `note: type in trait` block points
at the trait declaration's new `addend: i32`.)*

## What To Ignore For Now

Today extends 111 with one ordinary parameter beyond `&self`,
primitive type. Deferred:

- **Reference parameters in non-receiver slots** —
  `fn eq(&self, other: &Counter) -> bool` is the rmp pattern in
  `cmp.rs:5`. Passing `&Type` as a non-receiver argument is its
  own mechanic; today's parameter is `u32` only.
- **Multiple extra parameters** — `fn fma(&self, a: u32, b: u32) -> u32`.
  The rule extends naturally; not centered today.
- **Other receivers in trait methods** — `&mut self`, `self` by
  value (lessons 101/102 compose in identically).
- **Default method bodies** — `fn method(&self, p: T) -> T { ... }`
  inside the trait acts as a default the impl may override.
- **Multiple types implementing one trait** — still deferred from
  111.
- **Generic trait parameters** `trait Add<Rhs> { ... }`,
  **associated types** `type Output = ...;`, and **trait bounds**
  `fn f<T: Scale>(t: T)` — all blocked on the generics arc.
- **Operator traits** (`std::ops::Add`, `Mul`, ...),
  **`#[derive(...)]` macros**, **lifetime parameters**, the
  **orphan rule** — all named in 111 and unchanged today.

## Evidence

See `../evidence/112-trait-method-extra-parameter.md`.
