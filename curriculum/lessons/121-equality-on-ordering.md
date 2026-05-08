---
id: 121-equality-on-ordering
status: accepted
evidence: ../evidence/121-equality-on-ordering.md
---

# Compare two `Ordering` values with `==` and `!=`

## The Move

Lesson 117 extended `==` from primitive integers (lesson 013) to
`Vec<u64>` — the first non-primitive operand. Today extends `==` and
`!=` again, this time to `Ordering` — the standard library's
three-variant unit-only enum from lesson 051. The mechanism is the
same as lesson 117; the dispatched body is std's `impl PartialEq for
Ordering` instead of std's `impl PartialEq for Vec`.

```rust
use std::cmp::Ordering;

fn main() {
    let a: Ordering = Ordering::Less;
    let b: Ordering = Ordering::Less;
    let c: Ordering = Ordering::Equal;

    println!("a == b is {}", a == b);
    println!("a == c is {}", a == c);
    println!("a != c is {}", a != c);
}
```

`rustc demo.rs` compiles silently. `./demo` prints:

```text
a == b is true
a == c is false
a != c is true
```

Read the semantics directly: same variant on both sides → `==` is
`true`. Different variants on the two sides → `==` is `false`. `!=`
returns the opposite of `==`. `a` and `b` are both `Ordering::Less`,
so `a == b` is `true`. `a` is `Ordering::Less` and `c` is
`Ordering::Equal`, so `a == c` is `false` and `a != c` is `true`.

The mechanism: std's `Ordering` page (`std/cmp/enum.Ordering.md` line
327) lists `impl PartialEq for Ordering` — std implements the
`PartialEq` trait on the `Ordering` type, providing
`fn eq(&self, other: &Ordering) -> bool` (line 331) and the inherited
`fn ne(&self, other: &Rhs) -> bool` (line 337). The Reference at
`reference/expressions/operator-expr.md:508-516` states that `a == b`
is equivalent to `::std::cmp::PartialEq::eq(&a, &b)` — the same
desugar lesson 117 cited. On `Ordering` operands the dispatched `eq`
is std's `Ordering` impl, not the integer impl (013) and not the
`Vec` impl (117). Same surface operator, same desugar, third
dispatched body.

## Mental Model Delta

- *Before:* "`==` works on integers (013) and on `Vec<u64>` (117)
  because std implements `PartialEq` for both. I have not yet seen
  it on a standard-library *enum*."
- *After:* "`==` and `!=` work on `Ordering` for the same reason —
  std implements `PartialEq` for `Ordering`. The desugar is
  unchanged; the dispatched `eq` body is std's `Ordering` impl. Two
  `Ordering` values compare equal exactly when they pick the same
  variant. Whether a given enum supports `==` is not automatic — it
  depends on whether *someone* (std for `Ordering`, the user via
  `#[derive(PartialEq)]` for their own enum) wrote the impl."

## Prerequisites

- Installed concepts:
  - **Lesson 117** (load-bearing): `==`/`!=` on a non-primitive type
    via std's `PartialEq` impl, and the desugar `a == b ≡
    ::std::cmp::PartialEq::eq(&a, &b)`. Today reuses the framing
    verbatim, with `Ordering` substituted for `Vec<u64>`.
  - **Lesson 051** (load-bearing): `Ordering` as the std three-variant
    unit-only enum at path `std::cmp::Ordering`; `Ordering::Less`
    etc. as path-expression construction; `use std::cmp::Ordering;`
    to bring the type into scope.
  - **Lesson 013** (load-bearing): `==`, `!=` between two values of
    the same kind produces a `bool`. Today extends the operand types
    again; the operator and result kind are unchanged.
  - **Lesson 098** (load-bearing for the contrast): `enum Name { V1,
    V2 }` declares a user-side unit-variant enum. The contrast probe
    declares `enum Color { Red, Blue }` exactly this way.
  - **Lessons 019, 011, 044, 005, 002, 001, 003** (cited): `: TYPE`
    annotation slot filled with `Ordering`; `{}` formats `bool`;
    `use` declaration; `let` binding; `fn main`; `rustc`/`./demo`
    shape; rustc diagnostic four-part map for the contrast probe.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`.

## Try It

Save the program above as `demo.rs` in a fresh empty directory.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
a == b is true
a == c is false
a != c is true
```

Walk it. The `use` line pulls `Ordering` into scope. Three `let`
bindings each name an `Ordering` value — two `Less`, one `Equal`.
The three `println!` calls each format the result of one comparison
expression. `a == b` asks "do these two values pick the same
variant?" — yes, both `Less`. `a == c` asks the same — no, `Less`
vs `Equal`. `a != c` is the negation.

*Now the contrast.* Save `broken.rs` with a fresh user-side enum
that has *no* `PartialEq` impl, then try `==` on its values:

```rust
enum Color { Red, Blue }

fn main() {
    let a = Color::Red;
    let b = Color::Red;
    println!("{}", a == b);
}
```

Compile:

```text
error[E0369]: binary operation `==` cannot be applied to type `Color`
 --> broken.rs:6:22
note: an implementation of `PartialEq` might be missing for `Color`
help: consider annotating `Color` with `#[derive(PartialEq)]`
```

Read with the lesson 003 map: headline E-code `E0369`, location
`broken.rs:6:22`, and a `note:` plus `help:` block that names the
exact missing piece — `PartialEq` is not implemented for `Color`.
This is the contrastive fact today installs: `==` works on
`Ordering` *because* std implements `PartialEq` for it (page line
327); on a user-declared enum without that impl, the same operator
fails to dispatch and rustc fires E0369. The error code's own
description (`error_codes/E0369.md` line 4) reads "A binary operation
was attempted on a type which doesn't support it." `#[derive(...)]`
in the `help:` block is named-deferred — today only reads the
diagnostic.

## What Changed

- `==` and `!=` work on two `Ordering` values, returning a `bool`.
  Same-variant pairs are equal; different-variant pairs are unequal.
- The mechanism is std's `impl PartialEq for Ordering` (page line
  327). `==` desugars to `PartialEq::eq` — same desugar as lessons
  013 and 117; the dispatched body is `Ordering`'s own.
- This makes rmp's `cmp.rs:21` `if ord == cmp::Ordering::Equal` line
  readable: `ord` is an `Ordering` value, the right-hand side is one
  of `Ordering`'s variant constructors, and `==` is exactly today's
  mechanic.
- Whether an enum supports `==` is *not* automatic. A user-declared
  enum with no `PartialEq` impl fires `error[E0369]: binary
  operation \`==\` cannot be applied to type \`Color\``.

## Check Yourself

You write `tiny.rs`:

```rust
use std::cmp::Ordering;

fn main() {
    let g = Ordering::Greater;
    let l = Ordering::Less;
    let g2 = Ordering::Greater;
    println!("g == l  is {}", g == l);
    println!("g == g2 is {}", g == g2);
    println!("g != l  is {}", g != l);
}
```

What three lines does `./tiny` print?

*(Answer: `g == l is false` (different variants); `g == g2 is true`
(same variant `Greater`); `g != l is true` (negation of the first).)*

## What To Ignore For Now

Today installs only `==` and `!=` on `Ordering`. Deferred:

- **`<`, `<=`, `>`, `>=` on `Ordering`** — these dispatch via
  `PartialOrd`, a separate trait. Std implements `PartialOrd for
  Ordering` (page line 344) but the operator-to-trait wiring is its
  own move.
- **The `Eq` marker trait on `Ordering`** — std also implements `Eq`
  for `Ordering` (page line 385); it is a marker that promises
  reflexivity that `PartialEq` alone does not. Today reads `==`,
  not `Eq`.
- **`#[derive(PartialEq)]`** as a user-side mechanic — named only in
  the contrast probe's `help:` block. Authoring the impl ourselves
  is a separate move.
- **Manual `impl PartialEq for MyType`** — wholesale deferred.
- **`Copy` on `Ordering`** — implicit in the corroborating matrix
  probe in the appendix; not centered today.
- **`==` on `Result`, `Option`, generic enums beyond `Ordering`** —
  each a separate sibling move.
- **The `PartialEq<Rhs = Self>` declaration's `?Sized` and default
  type parameter** — same machinery 117 already named; still
  deferred.
- **Trait bounds and `where` clauses** — the trait-bounds arc,
  blocked since lesson 114.

## Evidence

See `../evidence/121-equality-on-ordering.md`.
