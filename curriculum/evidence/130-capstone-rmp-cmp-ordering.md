# Evidence: 130-capstone-rmp-cmp-ordering

Capstone for the trait-and-iterator arc (lessons 119-129). The slice
is rmp's `PartialOrd` and `Ord` impls on `BigUInt`, lines 12-33 of
`/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`, plus
the file-level `use std::cmp::{self, Ord, Ordering};` on line 2.

## Toolchain

```
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)

$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)

$ uname -srm
Darwin 24.5.0 x86_64
```

## rmp Source — Verbatim

`src/biguint/cmp.rs` (full file, 33 lines):

```rust
use super::basic::BigUInt;
use std::cmp::{self, Ord, Ordering};

impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}

impl Eq for BigUInt {}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.limbs.len().cmp(&other.limbs.len());
        if ord == cmp::Ordering::Equal {
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
                match left.cmp(right) {
                    Ordering::Equal => {}
                    ord => return ord,
                }
            }
            return Ordering::Equal;
        } else {
            ord
        }
    }
}
```

`src/biguint/basic.rs` lines 1-4:

```rust
#[derive(Clone)]
pub struct BigUInt {
    pub(super) limbs: Vec<u64>,
}
```

`src/biguint/convert.rs` lines 14-20 (the `From<u64>` impl reused by
Probe 2; lessons 110 and 118 named the non-canonicalization defect):

```rust
impl From<u64> for BigUInt {
    fn from(n: u64) -> BigUInt {
        BigUInt { limbs: vec![n] }
    }
}
```

`Cargo.toml` (full file, 6 lines):

```toml
[package]
name = "bignum"
version = "0.1.0"
edition = "2024"

[dependencies]
```

## std and Reference Sources — Verbatim

From `output/docs/rust/std/cmp/trait.PartialOrd.md` lines 7-21:

```text
pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>

where
    Rhs: ?Sized,

{
    // Required method
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    // Provided methods
    fn lt(&self, other: &Rhs) -> bool { ... }
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}
```

Surrounding prose, lines 31-33:

> This trait should **only** contain the comparison logic for a
> type **if one plans on only implementing `PartialOrd` but not
> `Ord`**. Otherwise the comparison logic should be in `Ord` and
> this trait implemented with `Some(self.cmp(other))`.

The std page itself names the rmp pattern verbatim. rmp's
cmp.rs:14 line `Some(self.cmp(other))` is what the std doc tells
implementers to write.

From `output/docs/rust/std/cmp/trait.Ord.md` lines 7-19:

```text
pub trait Ord: Eq + PartialOrd {
    // Required method
    fn cmp(&self, other: &Self) -> Ordering;

    // Provided methods
    fn max(self, other: Self) -> Self
       where Self: Sized { ... }
    fn min(self, other: Self) -> Self
       where Self: Sized { ... }
    fn clamp(self, min: Self, max: Self) -> Self
       where Self: Sized { ... }
}
```

From `output/docs/rust/std/cmp/enum.Ordering.md` lines 6-13:

```text
#[repr(i8)]

pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

From `output/docs/rust/reference/items/use-declarations.md` lines
27-35:

> Use declarations support a number of convenient shortcuts:
>
> - Simultaneously binding a list of paths with a common prefix,
>   using the brace syntax `use a::b::{c, d, e::f, g::h::i};`
>
> - Simultaneously binding a list of paths with a common prefix
>   and their common parent module, using the `self` keyword, such
>   as `use a::b::{self, c, d::e};`

The grouped form's grammar, line 14 of the same file:

```text
UseTree → ... | ( SimplePath? :: )? { ( UseTree ( , UseTree )* ,? )? } | ...
```

The example at lines 47-66 of the same file uses
`use std::collections::hash_map::{self, HashMap};` and reads both
forms — `hash_map::HashMap::new()` (the module-prefix form) and
`HashMap::new()` (the bare form) — verbatim.

From `output/docs/rust/reference/expressions/operator-expr.md`
lines 525-530:

| Symbol | Meaning | Overloading method |
| --- | --- | --- |
| `==` | Equal | `std::cmp::PartialEq::eq` |
| `!=` | Not equal | `std::cmp::PartialEq::ne` |
| `>` | Greater than | `std::cmp::PartialOrd::gt` |
| `<` | Less than | `std::cmp::PartialOrd::lt` |
| `>=` | Greater than or equal to | `std::cmp::PartialOrd::ge` |
| `<=` | Less than or equal to | `std::cmp::PartialOrd::le` |

This table grounds the contrast probe's `error[E0369]: binary
operation \`<\` cannot be applied to type \`BigUInt\`` — `<` is
exactly `PartialOrd::lt`, so without an `impl PartialOrd`, no
dispatch.

From `output/docs/rust/reference/expressions/match-expr.md`
(referenced earlier by 129 for diverging arms):

> if there are no match arms, then the match expression is
> diverging and the type is `!`

Today rides 129's grounding of the diverging-arm exemption.

## Probes

### Probe 1 — self-contained mirror (committed)

`observations/130-capstone-rmp-cmp-ordering.rs` mirrors cmp.rs:1-33
verbatim with the field made plain `pub` (no parent module).

```
$ rustc 130-capstone-rmp-cmp-ordering.rs -o probe
(silent, exit 0)
$ ./probe
a.partial_cmp(&b) is Some(Less)? true
a.cmp(&b) = Less
a.cmp(&c) = Equal
zero.cmp(&from0) = Less
p.cmp(&q) (MSL 1 vs 2)  = Less
$ echo $?
0
```

The first three lines witness equal-length `cmp` on equal-and-
unequal limbs. Line 4 witnesses the length-first short-circuit
(canonical-zero `vec![]` vs non-canonical zero `vec![0]`). Line 5
witnesses the for-loop's big-endian most-significant-first walk:
`p` and `q` both have length 2; `.iter().rev()` walks the MSL first
(`p`'s MSL is `0x1`, `q`'s is `0x2`); `1 < 2` fires the `ord =>
return ord` arm.

### Probe 2 — cross-crate driver

Layout under `/tmp/eduratchet2-130/cmp_capstone/`:

`Cargo.toml`:

```toml
[package]
name = "cmp_capstone"
version = "0.1.0"
edition = "2021"

[dependencies]
bignum = { path = "/Users/eli/InfoScraper/output/repos/rmp" }
```

`src/main.rs`:

```rust
use bignum::biguint::BigUInt;
use std::cmp::Ordering;

fn label(o: Ordering) -> &'static str {
    match o {
        Ordering::Less => "Less",
        Ordering::Equal => "Equal",
        Ordering::Greater => "Greater",
    }
}

fn main() {
    let a = BigUInt::from(100u64);
    let b = BigUInt::from(200u64);
    let c = BigUInt::from(100u64);

    let pc = a.partial_cmp(&b);
    let _: Option<Ordering> = pc;
    println!("a.partial_cmp(&b) == Some(Less): {}", pc == Some(Ordering::Less));

    let o = a.cmp(&b);
    println!("a.cmp(&b) = {}", label(o));

    let o2 = a.cmp(&c);
    println!("a.cmp(&c) = {}", label(o2));

    let z = BigUInt::zero();
    let from_zero = BigUInt::from(0u64);
    let o3 = z.cmp(&from_zero);
    println!("zero().cmp(&from(0u64)) = {}", label(o3));
}
```

Run:

```
$ cargo run
   Compiling bignum v0.1.0 (/Users/eli/InfoScraper/output/repos/rmp)
[2 unrelated rmp warnings: unused `index` in mul.rs:25 and dead `is_nonnegative` in bigint.rs:73]
   Compiling cmp_capstone v0.1.0 (/private/tmp/eduratchet2-130/cmp_capstone)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/cmp_capstone`
a.partial_cmp(&b) == Some(Less): true
a.cmp(&b) = Less
a.cmp(&c) = Equal
zero().cmp(&from(0u64)) = Less
```

### Probe 3 — rmp's own tests

```
$ cd /Users/eli/InfoScraper/output/repos/rmp && cargo test --lib
[2 warnings as above]
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/bignum-...)

running 17 tests
test biguint::convert::test::from_str_small ... ok
test biguint::basic::tests::display_factorial_correct ... ok
test bigint::tests::from_i64_compares_correctly_to_zero ... ok
test biguint::mul::tests::mul_assign_factorial_matches ... ok
test bigint::tests::from_i32_compares_correctly_to_zero ... ok
test biguint::basic::tests::num_bits_correct ... ok
test bigint::tests::mul_factorial_square ... ok
test biguint::mul::tests::mul_factorial_matches ... ok
test biguint::mul::tests::mul_factorial_square ... ok
test biguint::add::tests::add_eq_corresponds_to_add_fib ... ok
test bigint::tests::sub_assign_reverses_neg_fib_step ... ok
test bigint::tests::sub_reverses_fib_step ... ok
test bigint::tests::sub_assign_reverses_fib_step ... ok
test biguint::add::tests::sub_reverses_fib_step ... ok
test bigint::tests::add_eq_corresponds_to_add_fib ... ok
test biguint::add::tests::sub_assign_reverses_fib_step ... ok
test bigint::tests::sub_reverses_neg_fib_step ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

The capstone reads code that already passes its own author's tests.
None of the 17 tests pairs `BigUInt::zero()` against
`BigUInt::from(0u64)` on the order axis, so the canonical-zero
defect surfaces here, not in rmp's suite.

### Probe 4 — contrast: drop the `impl PartialOrd` block

`/tmp/eduratchet2-130/contrast_no_partialord.rs` keeps `impl
PartialEq` and `impl Ord` but removes `impl PartialOrd`.

```
$ rustc contrast_no_partialord.rs
error[E0277]: can't compare `BigUInt` with `BigUInt`
  --> contrast_no_partialord.rs:20:14
   |
20 | impl Ord for BigUInt {
   |              ^^^^^^^ no implementation for `BigUInt < BigUInt` and `BigUInt > BigUInt`
   |
   = help: the trait `PartialOrd` is not implemented for `BigUInt`
note: required by a bound in `Ord`
  --> /rustc/.../library/core/src/cmp.rs:981:0
help: consider annotating `BigUInt` with `#[derive(PartialOrd)]`
   |
 6 + #[derive(PartialOrd)]
 7 | pub struct BigUInt {
   |

error[E0599]: `BigUInt` is not an iterator
  --> contrast_no_partialord.rs:29:15
   |
 6 | pub struct BigUInt {
   | ------------------ method `partial_cmp` not found for this struct because it doesn't satisfy `BigUInt: Iterator`
...
29 |     let _ = a.partial_cmp(&b);
   |               ^^^^^^^^^^^ `BigUInt` is not an iterator
   |
   = note: the following trait bounds were not satisfied:
           `BigUInt: Iterator`
           which is required by `&mut BigUInt: Iterator`
note: the trait `Iterator` must be implemented
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `partial_cmp`, perhaps you need to implement one of them:
           candidate #1: `Iterator`
           candidate #2: `PartialOrd`

error[E0369]: binary operation `<` cannot be applied to type `BigUInt`
  --> contrast_no_partialord.rs:30:15
   |
30 |     let _ = a < b;
   |             - ^ - BigUInt
   |             |
   |             BigUInt
   |
note: an implementation of `PartialOrd` might be missing for `BigUInt`
help: consider annotating `BigUInt` with `#[derive(PartialEq, PartialOrd)]`

error: aborting due to 3 previous errors
```

Three diagnostics in one probe. (1) E0277 on `impl Ord for BigUInt`
with the help line *the trait `PartialOrd` is not implemented for
`BigUInt`* and the note *required by a bound in `Ord`* — empirical
witness that `Ord`'s `: Eq + PartialOrd` supertrait is enforced at
compile time. (2) E0599 on `.partial_cmp` lookup. (3) E0369 on `<`,
naming `PartialOrd` as the missing impl — the operator-to-trait
table from operator-expr.md:528 made structurally visible.

### Probe 5 — contrast: drop the `impl Ord` block

`/tmp/eduratchet2-130/contrast_no_ord.rs` keeps `impl PartialEq`
and `impl Eq` but removes `impl Ord`.

```
$ rustc contrast_no_ord.rs
error[E0599]: `BigUInt` is not an iterator
  --> contrast_no_ord.rs:21:15
   |
 4 | pub struct BigUInt {
   | ------------------ method `cmp` not found for this struct because it doesn't satisfy `BigUInt: Iterator`
...
21 |     let _ = a.cmp(&b);
   |               ^^^ `BigUInt` is not an iterator
   |
   = note: the following traits define an item `cmp`, perhaps you need to implement one of them:
           candidate #1: `Iterator`
           candidate #2: `Ord`

error: aborting due to 1 previous error
```

E0599: without `impl Ord`, `.cmp` does not exist on `BigUInt`. The
candidate list names `Ord` directly. (Note: `Iterator` also has a
`.cmp` method per std, which is why rustc lists both candidates;
the relevant one for the slice is `Ord`.)

### Probe 6 — contrast: drop the `PartialEq` supertrait

`/tmp/eduratchet2-130/contrast_no_partialeq.rs` declares a fresh
`Score` struct, omits `impl PartialEq`, and writes `impl PartialOrd`.

```
$ rustc contrast_no_partialeq.rs
error[E0277]: can't compare `Score` with `Score`
  --> contrast_no_partialeq.rs:12:21
   |
12 | impl PartialOrd for Score {
   |                     ^^^^^ no implementation for `Score == Score`
   |
   = help: the trait `PartialEq` is not implemented for `Score`
note: required by a bound in `PartialOrd`
help: consider annotating `Score` with `#[derive(PartialEq)]`

error: aborting due to 1 previous error
```

E0277: `PartialOrd`'s `: PartialEq<Rhs>` supertrait clause is
enforced at compile time. The diagnostic phrases the requirement as
*no implementation for `Score == Score`*, and the help line names
`PartialEq`. Witness for *fact A* (PartialOrd's supertrait) and the
parallel pattern for *fact B* (Ord's supertrait).

### Probe 7 — canonical-zero defect

`/tmp/eduratchet2-130/canonical_zero_defect.rs` mirrors cmp.rs:1-33
and exercises both `==` and `<` on `vec![]` vs `vec![0]`:

```
$ rustc canonical_zero_defect.rs -o czero && ./czero
canonical_zero.limbs.len() = 0
from_zero.limbs.len()      = 1
zero.cmp(&from(0u64))      = Less
zero  <  from(0u64)        = true
zero  == from(0u64)        = false
```

Three confirmations. The lengths differ. `cmp` returns `Less`
because of the length-first short-circuit. The `<` operator
dispatches through `PartialOrd::lt`, which for total-order types
is `Some(self.cmp(other)) == Some(Less)`, returning `true`. The
`==` operator dispatches through `PartialEq::eq` (lesson 118),
which returns `false` because `vec![] != vec![0]` (lesson 117).
Three axes, one constructor-side failure.

## Claim-to-Evidence Mapping

| Claim | Source |
|---|---|
| `cmp.rs:12-33` plus `cmp.rs:2` is today's slice. | rmp source quoted above. |
| `impl Trait for Type { ... }` is the trait-impl form. | Lesson 111 (load-bearing via 118). |
| `&self` is the borrowing receiver. | Lesson 100 (load-bearing). |
| `other: &Self` inside an impl resolves to `&BigUInt`. | Lesson 120 (load-bearing). |
| `Option<T>` with `Some(t)` constructor and the `Option<std::cmp::Ordering>` annotation slot. | Lesson 119 (load-bearing). |
| `self.method(args)` for a sibling method. | Lesson 122 (load-bearing). |
| `==` between two `Ordering` values. | Lesson 121 (load-bearing). |
| `v.iter()` on `Vec<T>`. | Lesson 123 (load-bearing). |
| `.iter().rev()` reverses the iterator. | Lesson 124 (load-bearing). |
| `.zip(...)` pairs two iterators. | Lesson 125 (load-bearing). |
| `for (left, right) in ...` tuple-pattern destructuring. | Lesson 126 (load-bearing). |
| `.cmp(&...)` on `usize` and `u64`. | Lesson 127 (load-bearing). |
| `Pattern => {}` empty arm body. | Lesson 128 (load-bearing). |
| `name => return name,` diverging arm with bare-name binding. | Lesson 129 (load-bearing). |
| `if cond { ... } else { ... }` as expression with else-tail. | Lessons 014, 026, 025 (cited; carry through). |
| `return Ordering::Equal;` inside if-then block. | Lesson 021 (cited; standard `return value;`). |
| Field access `self.limbs` returns `&Vec<u64>` through `&self`. | Lesson 095 + 100 (cited). |
| `.len()` on `Vec<T>` returns `usize`. | Lesson 107 (cited). |
| `&other.limbs.len()` is the prefix-`&` operator on a `usize`. | Lesson 045 (cited). |
| Chained dot-calls `.iter().rev().zip(...)`. | Lesson 049 (cited). |
| `Ordering::Equal`, `Ordering::Less`, `Ordering::Greater` are unit-variant constructors. | Lesson 051 (cited). |
| `match` with multiple arms. | Lesson 058 (cited). |
| `std::cmp::PartialOrd` declaration `pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>`. | `output/docs/rust/std/cmp/trait.PartialOrd.md` lines 7-21, quoted above. |
| `std::cmp::Ord` declaration `pub trait Ord: Eq + PartialOrd`. | `output/docs/rust/std/cmp/trait.Ord.md` lines 7-19, quoted above. |
| `Some(self.cmp(other))` is the std-recommended `partial_cmp` body for total-order types. | `trait.PartialOrd.md` lines 31-33, quoted above. |
| Grouped `use a::b::{c, d}` and the `self` keyword for the parent module. | Reference `items/use-declarations.md` lines 27-31, quoted above. |
| `<`, `<=`, `>`, `>=` dispatch through `PartialOrd::lt` etc. | Reference `expressions/operator-expr.md` lines 527-530, quoted above. |
| `Ordering` variants `Less = -1`, `Equal = 0`, `Greater = 1`. | `output/docs/rust/std/cmp/enum.Ordering.md` lines 6-13. |
| `BigUInt::zero() < BigUInt::from(0u64)` returns `Less` even though both represent 0. | Probe 2 (line 4); Probe 7. |
| Lesson 110 surfaced the same `From<u64>` defect via `is_zero`. | Lesson 110 *Honest observation*. |
| Lesson 118 surfaced the same defect via `==`. | Lesson 118 *Empirical Witness*. |
| Without `impl PartialOrd`, `impl Ord` fires E0277 with `PartialOrd` named in the help line; `<` fires E0369 with PartialOrd named. | Probe 4. |
| Without `impl Ord`, `.cmp` fires E0599 with `Ord` in the candidate list. | Probe 5. |
| Without `impl PartialEq`, `impl PartialOrd` fires E0277 with `PartialEq` named. | Probe 6. |
| rmp's existing tests pass with these impls. | Probe 3 transcript. |

## Direct Prerequisites — Specific Claims Used

- **Lesson 119** (load-bearing): `Option<T>` with `Some(t)` /
  `None` constructors and `Option<...>` as a return-type
  annotation. The `partial_cmp` signature `Option<std::cmp::Ordering>`
  fills the type slot; `Some(self.cmp(other))` is the constructor.
- **Lesson 120** (load-bearing): `other: &Self` in a non-receiver
  trait-impl parameter slot. cmp.rs lines 13 and 19 use this exact
  spelling; inside `impl ... for BigUInt`, `&Self` resolves to
  `&BigUInt`, matching the trait declarations.
- **Lesson 121** (load-bearing): `==` on two `Ordering` values
  dispatches through std's `impl PartialEq for Ordering`. cmp.rs
  line 21 `if ord == cmp::Ordering::Equal` is exactly this.
- **Lesson 122** (load-bearing): `self.method(args)` for sibling-
  method delegation. cmp.rs line 14 `Some(self.cmp(other))` is
  the canonical example.
- **Lesson 123** (load-bearing): `v.iter()` returns a slice
  iterator yielding `&T`. cmp.rs line 22 calls `.iter()` twice.
- **Lesson 124** (load-bearing): `.iter().rev()` reverses the
  iterator. cmp.rs line 22 chains `.rev()` on each `.iter()`.
- **Lesson 125** (load-bearing): `.zip(other_iter)` pairs two
  iterators. cmp.rs line 22 zips the two reversed iterators.
- **Lesson 126** (load-bearing): `for (left, right) in iter`
  tuple-pattern destructuring at the loop binding slot. cmp.rs
  line 22 uses exactly `for (left, right) in ...`.
- **Lesson 127** (load-bearing): `.cmp(&other)` on `usize` and
  `u64` returns `Ordering`. cmp.rs line 20's `usize.cmp(&usize)`
  and line 23's `u64.cmp(&u64)` (through auto-deref) both rely
  on this.
- **Lesson 128** (load-bearing): `Pattern => {}` empty arm body.
  cmp.rs line 24 `Ordering::Equal => {}` is exactly this.
- **Lesson 129** (load-bearing): `name => return name,` diverging
  arm with bare-name binding. cmp.rs line 25 `ord => return ord,`
  is exactly this.
- **Lesson 118** (load-bearing): the capstone-mode template, the
  named-deferred treatment of supertraits and `<Rhs = Self>`, the
  `Vec<u64>` equality body that the `PartialEq` impl on the same
  file rides, and the `From<u64>` non-canonicalization defect that
  today's `<`-axis surfaces a third time.
- **Lesson 110** (load-bearing): the rmp module-path chain
  `bignum::biguint::BigUInt`, the `pub struct BigUInt {
  pub(super) limbs: Vec<u64> }` declaration, the cross-crate
  driver pattern, and the limb-ordering invariants
  (little-endian, no trailing zeros, empty-vec means zero).
- **Lesson 111** (load-bearing through 118 + 120): `impl Trait
  for Type { ... }`. Both today's impl blocks use this shape.
- **Lesson 100** (load-bearing): `&self` borrowing receiver,
  `Self` as impl-target alias, return-type slot. Auto-deref
  named-deferred.
- **Lesson 044** (load-bearing): `use Path::Item;` import. Today
  extends with the grouped form and `self` keyword.
- **Lesson 051** (load-bearing): `Ordering` enum, `use
  std::cmp::Ordering;`, `Ordering::Less` etc. variant constructors.
- **Lesson 049** (load-bearing): chained dot-calls.
- **Lesson 058** (load-bearing): `match` with multiple arms.
- **Lesson 026** (load-bearing): `if condition { ... }` as an
  expression. **Lesson 014** (load-bearing): `if-else` shape.
  **Lesson 025** (load-bearing): tail expression as implicit
  return.
- **Lesson 021** (load-bearing): `return value;` as a function-
  exit statement.
- **Lesson 095** (load-bearing): struct field access through `&`.
- **Lesson 107** (load-bearing): `Vec<T>` with `.len() -> usize`.
- **Lesson 045** (cited): prefix-`&` operator.

## Older Supporting Lessons (cited only)

- 002 (`fn main`), 005 (`let`), 008 (`fn`), 011 (`println!`),
  012 (`bool`), 013 (`==`), 019 (annotation slot), 040 (dot-call
  shape), 080 (`u64`/`usize`), 003 (diagnostic shape — used by
  contrast probes), 001 (`rustc demo.rs && ./demo`), 032 (`cargo
  new --vcs none`, `cargo run`, `cargo test`), 065
  (`[dependencies]` and `bignum = { path = "..." }`).

## Capstone Status

This is the fourth capstone in the run after:

- **Lesson 063** — closed the Book's first guessing-game capstone
  (cited only).
- **Lesson 067** — closed the Book's second guessing-game capstone
  (cited only).
- **Lesson 110** — closed the rmp BigUInt trio (load-bearing today
  via the limb-ordering invariants and the cross-crate driver
  pattern).
- **Lesson 118** — closed the rmp `cmp.rs:4-10` PartialEq + Eq
  pair (load-bearing today via the capstone-mode template, the
  named-deferred supertrait treatment, and the `From<u64>` defect
  reused on the `<` axis).

Today closes the ordering halves of rmp's `cmp.rs`. The whole file
`cmp.rs` is now read end-to-end across two cycles. The remaining
rmp files (`add.rs`, `mul.rs`, `div.rs`, `shift.rs`, `format.rs`,
the rest of `convert.rs`, `bigint.rs`) are unlocked but deferred to
later arcs.
