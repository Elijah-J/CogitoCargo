---
id: 118-capstone-rmp-cmp-equality
status: accepted
evidence: ../evidence/118-capstone-rmp-cmp-equality.md
---

# Capstone: read rmp's `cmp.rs:4-10` PartialEq + Eq pair end-to-end

## The Slice

This is *Capstone Mode* — the third capstone in this run after lesson
110. **No new Rust mechanic.** The trait arc — lessons 111 through 117
— was assembled to make one specific real-world rmp slice readable
end-to-end: the `PartialEq` and `Eq` impls on `BigUInt` in
`src/biguint/cmp.rs`, lines 4-10. Today reads them.

The literal source from `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`:

```rust
impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}

impl Eq for BigUInt {}
```

Seven lines, two `impl` blocks. Three understanding axes — impl
headers, the `eq` method body, the dispatch chain from `a == b` to a
`bool`. Three named deferrals are clearly marked: the `?Sized` bound
on `Rhs`, the `= Self` default-type-parameter on `Rhs`, and the
supertrait `Eq: PartialEq`.

## The Two Impl Headers, Token by Token

**`impl PartialEq<BigUInt> for BigUInt`** (line 4):

- `impl ... for ...` (111) — the *trait impl* form, distinct from
  inherent `impl Type { ... }` (100). The header reads "implement this
  trait for this type."
- `PartialEq<BigUInt>` is the trait + a generic argument. The trait
  declaration in std is `pub trait PartialEq<Rhs: ?Sized = Self> { fn
  eq(&self, other: &Rhs) -> bool; fn ne(&self, other: &Rhs) -> bool {
  ... } }` (file `output/docs/rust/std/cmp/trait.PartialEq.md` lines
  7-19). Two pieces are named deferrals: `?Sized` (a trait bound that
  loosens the default Sized requirement on `Rhs`) and `= Self` (a
  *default* type argument — if the impl writes nothing in the angle
  brackets, `Rhs` becomes `Self`). rmp wrote `<BigUInt>` explicitly,
  bypassing the default; the bare `impl PartialEq for BigUInt` form
  would also have worked because `Self` here is `BigUInt`.
- `for BigUInt` (111) — the target type.

**`impl Eq for BigUInt`** (line 10):

- Same `impl Trait for Type` shape with no generic arguments. std's
  declaration is `pub trait Eq: PartialEq { }` (`trait.Eq.md` line
  7). The `: PartialEq` after the trait name is a *supertrait*. Read
  structurally as "Eq requires PartialEq is also impl'd for the
  type." rmp's line 4 satisfies that requirement. The formal
  supertrait-machinery move is deferred.
- The body `{ }` is empty. By lesson 116, an empty `{}` impl is
  legal exactly when every method the trait declared has a default
  body. `Eq`'s body in std is empty — its doc says "Eq is a trait
  without methods" — so the empty impl is complete.

## The `eq` Method, Token by Token

```rust
fn eq(&self, other: &BigUInt) -> bool {
    self.limbs == other.limbs
}
```

- `fn` (008) introduces a function item; `eq` is the method name and
  must match the trait's `eq` declaration (lesson 112's
  contract-matching rule, applied after the impl header substitutes
  `Rhs = BigUInt`).
- `&self` (100): the borrowing receiver — Reference shorthand for
  `self: &Self`.
- `other: &BigUInt` (113): a non-receiver reference parameter, same
  shape as 113's `other: &Counter`. After substituting
  `Rhs = BigUInt`, the trait's `other: &Rhs` slot becomes
  `other: &BigUInt` — exactly what rmp wrote.
- `-> bool` (012): the return type — yes/no for "are these equal?"
- Body `self.limbs == other.limbs`:
  - `self.limbs` (095 + 100) reads the `Vec<u64>` field through
    `&self`; `other.limbs` does the same through `other: &BigUInt`.
  - `==` (013 + 117) on two `Vec<u64>` values dispatches into std's
    `Vec` `PartialEq` impl: `true` iff same length and same element
    values pairwise.
- The expression is the function's only line and has no trailing
  `;`, so it is the implicit return value (025).

## The Dispatch Chain for `a == b`

Reference `expressions/operator-expr.md` lines 512-516: `a == b` is
equivalent to `::std::cmp::PartialEq::eq(&a, &b)`. The chain:

1. `a == b` desugars to `<BigUInt as PartialEq<BigUInt>>::eq(&a, &b)`.
2. Trait resolution finds rmp's impl on `cmp.rs:4-7`.
3. Inside the body, `self.limbs == other.limbs` becomes a
   `Vec<u64> == Vec<u64>` comparison (lesson 117).
4. std's `Vec` `PartialEq` impl returns `true` iff same length and
   same element values pairwise.
5. The resulting `bool` is the implicit return.
6. `a != b` rides std's *default* `ne` body — lesson 116's
   default-method-body machinery in real-world use.

## Empirical Witness

Two probes; transcripts in the appendix.

The self-contained probe at
`observations/118-capstone-rmp-cmp-equality.rs` mirrors the rmp slice
with a small `BigUInt`-shaped struct (the field is plain `pub` because
the single-file probe has no parent module). `rustc probe.rs && ./probe`
prints `a == b -> true`, `a == c -> false`, `a != b -> false`,
`a != c -> true`.

The cross-crate driver depends on rmp via path dependency
(`bignum = { path = "/Users/eli/InfoScraper/output/repos/rmp" }`),
imports `BigUInt` through the path lesson 110 walked, and builds
values with `BigUInt::from(42u64)`. `cargo run` confirms the same
four lines, then adds three more: `z == from_zero -> false`,
`z == z -> true`, `from_zero == from_zero -> true`. Lesson 110's
honest defect — `BigUInt::zero()` builds `limbs: vec![]` while
`BigUInt::from(0u64)` builds `limbs: vec![0]` — shows up identically
on the `==` axis: same mathematical zero, unequal limbs, unequal
under rmp's PartialEq. `cargo test --lib` against rmp itself reports
`17 passed; 0 failed`; the impls compile and rmp's existing tests
still pass.

A contrast probe drops the `impl PartialEq` block entirely. `a == b`
fires `error[E0369]: binary operation '==' cannot be applied to type
'BigUInt'` with `note: an implementation of 'PartialEq' might be
missing for 'BigUInt'`. Witness that the `impl PartialEq` block is
*what makes* `==` work. A second contrast drops only `impl Eq for
BigUInt {}` (PartialEq kept); the probe still compiles and runs
identically — Eq is a marker for stronger guarantees needed by
HashMap/BTreeMap key positions and certain generic bounds, none of
which today's probes exercise.

## Mental Model Delta

- *After lesson 110:* "I can read rmp's `BigUInt` plus the trio
  `zero` / `is_zero` / `num_bits` end-to-end. Anything starting with
  `impl Trait for ...` is opaque."
- *After lesson 117:* "I have installed every named ingredient for
  rmp's `cmp.rs:4-10`, but I have not read it end-to-end."
- *After today:* "I can read `cmp.rs:4-10` token-by-token. The
  PartialEq impl header composes 111 and 114; its `eq` body composes
  100, 112, 113, 012, 095, and 117. The Eq impl is 111 + 116. The
  dispatch from `a == b` to `bool` is fully traced through the std
  desugar rule and the `Vec` `PartialEq` impl. Three named deferrals
  remain: `?Sized`, `= Self`, and the supertrait `Eq: PartialEq`.
  Two halves of rmp's `cmp.rs` close; lines 12-33 (PartialOrd/Ord)
  remain for a later arc."

## Prerequisites

- Installed concepts (the trait arc, eight lessons including today):
  - **Lesson 117** (load-bearing): `vec_a == vec_b` on `Vec<T>`. The
    body `self.limbs == other.limbs` is exactly this mechanic.
  - **Lesson 116** (load-bearing): empty-`{}` impl accepts every
    default the trait provides. `impl Eq for BigUInt {}` rides this.
  - **Lesson 114** (load-bearing): the `<RHS>` generic-trait-parameter
    shape. `<BigUInt>` after `PartialEq` is a substitution.
  - **Lesson 113** (load-bearing): `other: &BigUInt` as a non-receiver
    reference parameter on a trait method.
  - **Lesson 112** (load-bearing): the contract-matching rule.
  - **Lesson 111** (load-bearing): `impl Trait for Type { ... }`.
  - **Lesson 110** (load-bearing): the rmp module-path chain
    `bignum::biguint::BigUInt`, the `pub struct BigUInt { pub(super)
    limbs: Vec<u64> }` reading, and the `From<u64>`
    non-canonicalization observation reused today.
  - **Lesson 100** (load-bearing): `&self`; `Self` as alias inside an
    impl block.
  - **Lesson 095** (load-bearing): struct field access `self.limbs`.
  - **Earlier lessons** (cited): 002, 005, 008, 011, 012, 013, 019,
    025, 040, 044, 080, 003, 001, 032, 065. Each named in the
    appendix with the carry-through claim.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` and `cargo` on `PATH`, ability to `cd` into a directory.

## Try It

Save the slice above as `probe.rs` (with the field made plain `pub`
so a single-file probe runs):

```console
$ rustc probe.rs -o probe
$ ./probe
a == b -> true
a == c -> false
a != b -> false
a != c -> true
```

Predict each output line by walking the dispatch chain in your head:
`a == b` → `eq(&a, &b)` → body returns `a.limbs == b.limbs` → both
are `vec![42]` → `true`. Then verify with the run.

The cross-crate driver (full `Cargo.toml` + `src/main.rs` in the
appendix) requires only a `bignum = { path = "..." }` dependency and
`cargo run`.

## Check Yourself

(a) The line `impl PartialEq<BigUInt> for BigUInt` could equivalently
be written as `impl PartialEq for BigUInt`. Why? Which named-deferred
mechanic in std's `PartialEq` declaration makes the bare form legal?

(b) Two `BigUInt` values built as `BigUInt::from(0u64)` and
`BigUInt::zero()` represent the same mathematical value but compare
*unequal* under rmp's `==`. Which constructor's discipline fails,
and which lesson surfaced this defect first?

(c) If the line `impl Eq for BigUInt {}` were deleted, what behavior
would change in today's probes? What deferred behavior does the line
guard?

*(Answers: (a) std's `pub trait PartialEq<Rhs: ?Sized = Self>` carries
a *default type parameter* `= Self`. When the impl header's angle
bracket slot is empty, `Rhs` becomes `Self`, which inside `impl ... for
BigUInt` is `BigUInt`. Both forms expand to the same impl. (b)
`From<u64>` builds `BigUInt { limbs: vec![n] }` without trimming when
`n == 0`, producing a non-canonical `vec![0]` limb. The `Vec<u64>`
equality body in `cmp.rs:6` then sees `vec![] != vec![0]`. Lesson 110
surfaced the same defect first via `is_zero`'s length-only check. (c)
Today's probes' `==` and `!=` behavior is unchanged — they need only
PartialEq. The Eq impl guards the deferred reflexivity-marker
requirement that `HashMap`/`BTreeMap` keys, `#[derive(Eq)]`-using
callers, and generic functions with `T: Eq` bounds depend on; without
it those sites would refuse `BigUInt`.)*

## What To Ignore For Now

Today reads only `cmp.rs:4-10`. Same `cmp.rs` file holds:

- Lines 12-16 — `impl PartialOrd for BigUInt` with `partial_cmp`
  delegating to `cmp`. Composes today's machinery with `Option<T>`,
  `Ordering`, and PartialOrd's default-bodied comparison methods.
- Lines 18-33 — `impl Ord for BigUInt`, an algorithmic body using
  `self.limbs.iter().rev().zip(...)`, `match` on `Ordering` variants,
  and a `for` loop with `return` from inside an arm. Each ingredient
  is its own future move.

Mechanic-level deferrals named today:

- The `<Rhs: ?Sized = Self>` shape on `PartialEq` — the `?Sized`
  bound and the *default type parameter* `= Self`. rmp's explicit
  `<BigUInt>` bypasses the default; the formal default-type-parameter
  machinery is its own move (and is exactly what `std::ops::Add<Rhs
  = Self>` also uses).
- The supertrait `Eq: PartialEq` in std's `pub trait Eq: PartialEq
  { }` declaration. Read structurally today; the colon-after-trait-
  name shape and its compile-time enforcement are a separate move.
- `Eq`'s implicit `assert_receiver_is_total_eq` method —
  feature-gated lint internal in std. Deferred wholesale.
- The `ne` method's *default body* in `PartialEq` is
  `{ !self.eq(other) }` — lesson 116's mechanic at work in a real
  std trait. Today rides it without exercising the body.
- `#[derive(PartialEq)]` and `#[derive(Eq)]` macros — deferred since
  095/110.
- `impl PartialEq` for *foreign* types and the std "never implement
  PartialEq for a foreign type" recommendation
  (`trait.PartialEq.md` lines 67-83). rmp's impl is on its own type,
  so the rule is satisfied; the rule itself is its own topic.
- The full `Allocator` parameter on `Vec` — invisible while the
  default global allocator is used; deferred from 117.
- E0369 (binary op cannot be applied) and E0277 (trait bound not
  satisfied) — diagnostics surfaced by the contrast probes; the
  E-code catalogue itself is its own arc.
- Auto-deref through `&self` on field access — `self.limbs` works
  regardless of `self` being `&BigUInt` rather than `BigUInt`. The
  formal auto-deref rule remains deferred.

The whole rest of rmp:

- `biguint/add.rs`, `mul.rs`, `div.rs`, `shift.rs`, `format.rs`, the
  rest of `convert.rs`, `bigint.rs`. Each composes the trait
  machinery installed through 117 with arithmetic, iteration, and
  formatting machinery yet to be installed.

## Evidence

See `../evidence/118-capstone-rmp-cmp-equality.md`.
