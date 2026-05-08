---
id: 117-vec-equality
status: accepted
evidence: ../evidence/117-vec-equality.md
---

# Compare two `Vec<T>` values with `==`

## The Move

Lesson 013 installed `==` only on primitive integers. Today's working
probe shows `==` working on a non-primitive: two `Vec<u64>` values,
returning a `bool`.

```rust
fn main() {
    let a: Vec<u64> = vec![10, 20, 30];
    let b: Vec<u64> = vec![10, 20, 30];
    let c: Vec<u64> = vec![10, 20, 99];
    let d: Vec<u64> = vec![10, 20];
    println!("a == b is {}", a == b);
    println!("a == c is {}", a == c);
    println!("a == d is {}", a == d);
}
```

`rustc demo.rs` compiles silently. `./demo` prints:

```text
a == b is true
a == c is false
a == d is false
```

The semantics are *pairwise* — `==` returns `true` exactly when the
two vectors have **the same length AND the same element value at
every index**. `a == b` is `true`: same length, agreeing elements.
`a == c` is `false` on one mismatched element (index 2). `a == d`
is `false` on length alone.

The mechanism: std implements the `PartialEq` trait for `Vec<T>`
whenever the element type implements `PartialEq` (std lists the
impl as `impl<T, U, A1, A2> PartialEq<Vec<U, A2>> for Vec<T, A1>
where ... T: PartialEq<U>`). The probe substitutes `T = U = u64`,
and lesson 013 already installed `u64 == u64`, so the bound is
satisfied. The Reference at `reference/expressions/operator-expr.md`
states that `a == b` is equivalent to `::std::cmp::PartialEq::eq(&a,
&b)` — `==` is sugar for a `PartialEq::eq` call. On `Vec<u64>`
operands the dispatched `eq` is std's `Vec` impl; on integer
operands (lesson 013) it was std's integer impl. Same surface
operator, different dispatched body.

## Mental Model Delta

- *Before:* "`==` works on integers and produces a `bool`. I have
  not seen it on a non-primitive value."
- *After:* "`==` is sugar for a `PartialEq::eq` call. Whoever
  implemented `PartialEq` for a type makes `==` work on that type.
  Std implemented `PartialEq` for `Vec<T>` (when the element type
  itself supports `==`), so `vec_a == vec_b` returns `true` exactly
  when the two have the same length and the same element values
  pairwise."

## Prerequisites

- Installed concepts:
  - **Lesson 013** (load-bearing): `==` between two values produces
    a `bool`. Today extends 013 from integer operands to `Vec<T>`
    operands; the operator and the result kind are unchanged.
  - **Lesson 107** (load-bearing): `Vec<T>` construction with
    `vec![v1, v2, ...]` and the `: Vec<u64>` annotation slot.
  - **Lesson 080** (load-bearing): `u64`. Combined with lesson 013,
    `u64 == u64` is the impl that satisfies the *element-type*
    clause in std's `Vec` impl.
  - **Lesson 114** (cited): the `<RHS>` generic-trait-parameter
    shape used to *read* std's `PartialEq<Rhs = Self>` declaration.
  - **Lesson 116** (cited): default method bodies. Std's `PartialEq`
    has one provided method `ne` alongside the required `eq`; the
    structural reason `!=` is available wherever `==` is.
  - **Lessons 002, 005, 011, 019, 003, 001** (cited): unchanged from
    prior usage. The evidence appendix carries the topical detail.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`.

## Try It

Save the program above as `demo.rs`, then `rustc demo.rs && ./demo`.
Read each output line as a yes/no question — "do these two vectors
have the same length and the same element values at every index?"
For `a == b` the answer is yes; for `a == c` it fails on one
element; for `a == d` it fails on length.

## What Changed

- `==` works on `Vec<T>`, not just primitive integers. The result
  is still a `bool`.
- Semantics are *pairwise*: `true` iff same length AND same element
  value at every index. Different length → `false`.
- The mechanism is std's `impl PartialEq for Vec<T>`. `==` desugars
  to a `PartialEq::eq` call; on `Vec<u64>` operands, the dispatched
  `eq` is std's `Vec` impl.
- This makes rmp's `cmp.rs:6` `self.limbs == other.limbs` readable:
  `BigUInt::limbs` is a `Vec<u64>` (lesson 107), so the expression
  is exactly today's mechanic.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![1, 2, 3];
    let ys: Vec<u64> = vec![1, 2, 3];
    let zs: Vec<u64> = vec![1, 2, 4];
    println!("xs == ys is {}", xs == ys);
    println!("xs == zs is {}", xs == zs);
}
```

What two lines does `./tiny` print?

*(Answer: `xs == ys is true` then `xs == zs is false` — same length
in both cases, but `zs` differs from `xs` at index 2 (`3` vs. `4`),
and pairwise equality needs every index to agree.)*

## What To Ignore For Now

Today installs only `vec_a == vec_b` on `Vec<T>` where `T` is a
primitive integer type. Deferred:

- **`!=` on `Vec<T>`** — works alongside `==` (std: "Implementing
  this trait provides the `==` and `!=` operators"); separate move.
- **The full `PartialEq` declaration** — `pub trait PartialEq<Rhs:
  ?Sized = Self> { fn eq(...) -> bool; fn ne(...) -> bool { ... } }`.
  The deeper machinery (`?Sized`, the *default type parameter*
  `= Self`, `ne`'s default body) is not exercised.
- **Trait bounds `T: PartialEq` and `where` clauses** — the
  trait-bounds arc, blocked since lesson 114 named it.
- **Mismatched element types** (`Vec<u64> == Vec<u32>`) — fires
  `E0277` exposing the deferred trait-bounds machine.
- **The `Allocator` parameter, `==` on other std collections,
  manual `impl PartialEq for MyType`, `#[derive(PartialEq)]`,
  E0277** — each a separate later move. rmp's `cmp.rs:4-8` is the
  manual-impl shape today's lesson does *not* cover; today reads
  only the *operator*.

## Evidence

See `../evidence/117-vec-equality.md`.
