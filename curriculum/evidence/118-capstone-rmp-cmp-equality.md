# Evidence: 118-capstone-rmp-cmp-equality

Capstone for the trait arc (lessons 111-117). The slice is rmp's
`PartialEq` and `Eq` impls on `BigUInt`, lines 4-10 of
`/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`.

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

`src/biguint/cmp.rs` (full file, 33 lines; today reads lines 4-10):

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

`src/biguint/basic.rs` lines 1-4 (the `BigUInt` declaration the
capstone references; lesson 110 read this end-to-end):

```rust
#[derive(Clone)]
pub struct BigUInt {
    pub(super) limbs: Vec<u64>,
}
```

`src/biguint/convert.rs` lines 8-20 (the `From<u64>` impl reused by
the cross-crate driver; lesson 110 named the non-canonicalization
defect):

```rust
impl From<u32> for BigUInt {
    fn from(n: u32) -> BigUInt {
        BigUInt {
            limbs: vec![n as u64],
        }
    }
}

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

## std Trait Declarations — Verbatim

From `output/docs/rust/std/cmp/trait.PartialEq.md` lines 7-19:

```
pub trait PartialEq<Rhs = Self>

where
    Rhs: ?Sized,

{
    // Required method
    fn eq(&self, other: &Rhs) -> bool;

    // Provided method
    fn ne(&self, other: &Rhs) -> bool { ... }
}
```

Surrounding prose (lines 25-29):

> Implementing this trait for types provides the `==` and `!=`
> operators for those types.
>
> `x.eq(y)` can also be written `x == y`, and `x.ne(y)` can be
> written `x != y`. We use the easier-to-read infix notation in the
> remainder of this documentation.

From `output/docs/rust/std/cmp/trait.Eq.md` lines 7-8:

```
pub trait Eq: PartialEq { }
```

Surrounding prose (lines 23-24):

> This property cannot be checked by the compiler, and therefore
> `Eq` is a trait without methods.

The `==` desugar rule from
`output/docs/rust/reference/expressions/operator-expr.md` lines
512-516:

```rust
let a = 1;
let b = 1;
a == b;
// is equivalent to
::std::cmp::PartialEq::eq(&a, &b);
```

And the operator-overload-method table at lines 525-526:

| Symbol | Meaning | Overloading method |
| --- | --- | --- |
| `==` | Equal | `std::cmp::PartialEq::eq` |
| `!=` | Not equal | `std::cmp::PartialEq::ne` |

## Probes

### Probe 1 — self-contained (committed)

`observations/118-capstone-rmp-cmp-equality.rs` mirrors `cmp.rs:4-10`
with a small `BigUInt`-shaped struct (the field is plain `pub` rather
than `pub(super)` because the probe has no parent module).

```
$ rustc 118-capstone-rmp-cmp-equality.rs -o probe
(silent, exit 0)
$ ./probe
a == b -> true
a == c -> false
a != b -> false
a != c -> true
$ echo $?
0
```

### Probe 2 — cross-crate driver

Layout under `/tmp/eduratchet2-118/rmp_driver/`:

`Cargo.toml`:

```toml
[package]
name = "rmp_driver"
version = "0.1.0"
edition = "2024"

[dependencies]
bignum = { path = "/Users/eli/InfoScraper/output/repos/rmp" }
```

`src/main.rs`:

```rust
use bignum::biguint::BigUInt;

fn main() {
    let a: BigUInt = BigUInt::from(42u64);
    let b: BigUInt = BigUInt::from(42u64);
    let c: BigUInt = BigUInt::from(7u64);
    let z: BigUInt = BigUInt::zero();

    println!("a == b -> {}", a == b);
    println!("a == c -> {}", a == c);
    println!("a != b -> {}", a != b);
    println!("a != c -> {}", a != c);

    let from_zero: BigUInt = BigUInt::from(0u64);
    println!("z == from_zero -> {}", z == from_zero);
    println!("z == z         -> {}", z == z);
    println!("from_zero == from_zero -> {}", from_zero == from_zero);
}
```

Run:

```
$ cargo run
   Compiling bignum v0.1.0 (/Users/eli/InfoScraper/output/repos/rmp)
[2 unrelated rmp warnings: unused `index` in mul.rs:25 and dead `is_nonnegative` in bigint.rs:73]
   Compiling rmp_driver v0.1.0 (/private/tmp/eduratchet2-118/rmp_driver)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.12s
     Running `target/debug/rmp_driver`
a == b -> true
a == c -> false
a != b -> false
a != c -> true
z == from_zero -> false
z == z         -> true
from_zero == from_zero -> true
```

The first four lines confirm `==` and `!=` dispatch through rmp's
`cmp.rs:4-7` `eq` impl with the default `ne` body. The last three:

- `z == from_zero` is `false` because `z.limbs == vec![]` while
  `from_zero.limbs == vec![0]` — different lengths, `Vec<u64>`
  PartialEq returns `false`.
- `z == z` is `true` because `vec![] == vec![]`.
- `from_zero == from_zero` is `true` because `vec![0] == vec![0]`.

This is lesson 110's `BigUInt::from(0u64).is_zero() == false`
observation, replayed on the `==` axis: the same constructor-side
discipline defect.

### Probe 3 — rmp's own tests

```
$ cd /Users/eli/InfoScraper/output/repos/rmp && cargo test --lib
[2 warnings as above]
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/bignum-...)

running 17 tests
test biguint::basic::tests::display_factorial_correct ... ok
test bigint::tests::from_i64_compares_correctly_to_zero ... ok
test biguint::mul::tests::mul_assign_factorial_matches ... ok
test bigint::tests::from_i32_compares_correctly_to_zero ... ok
test biguint::convert::test::from_str_small ... ok
test biguint::mul::tests::mul_factorial_matches ... ok
test biguint::basic::tests::num_bits_correct ... ok
test bigint::tests::mul_factorial_square ... ok
test biguint::mul::tests::mul_factorial_square ... ok
test biguint::add::tests::sub_assign_reverses_fib_step ... ok
test biguint::add::tests::add_eq_corresponds_to_add_fib ... ok
test bigint::tests::add_eq_corresponds_to_add_fib ... ok
test biguint::add::tests::sub_reverses_fib_step ... ok
test bigint::tests::sub_assign_reverses_neg_fib_step ... ok
test bigint::tests::sub_assign_reverses_fib_step ... ok
test bigint::tests::sub_reverses_fib_step ... ok
test bigint::tests::sub_reverses_neg_fib_step ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

The capstone reads code that already passes its own author's tests.

### Probe 4 — contrast: drop the `impl PartialEq` block

`/tmp/eduratchet2-118/contrast_no_eq.rs`:

```rust
struct BigUInt {
    limbs: Vec<u64>,
}

fn main() {
    let a = BigUInt { limbs: vec![42] };
    let b = BigUInt { limbs: vec![42] };
    println!("{}", a == b);
}
```

`rustc contrast_no_eq.rs`:

```
error[E0369]: binary operation `==` cannot be applied to type `BigUInt`
  --> contrast_no_eq.rs:13:22
   |
13 |     println!("{}", a == b);
   |                    - ^^ - BigUInt
   |                    |
   |                    BigUInt
   |
note: an implementation of `PartialEq` might be missing for `BigUInt`
  --> contrast_no_eq.rs:6:1
   |
 6 | struct BigUInt {
   | ^^^^^^^^^^^^^^ must implement `PartialEq`
help: consider annotating `BigUInt` with `#[derive(PartialEq)]`
   |
 6 + #[derive(PartialEq)]
 7 | struct BigUInt {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
```

E0369 confirms the dispatch path: without an `impl PartialEq for
BigUInt`, the `==` operator has nothing to dispatch to. The same
`impl PartialEq<BigUInt> for BigUInt` block is *what makes* `==`
work for `BigUInt` — empirical witness for the claim that `==` is
dispatched through the trait, not hardwired into the language.

### Probe 5 — contrast: drop only the Eq impl

`/tmp/eduratchet2-118/contrast_no_eq_marker.rs` keeps `impl PartialEq`
and removes only `impl Eq for BigUInt {}`. Output identical to
Probe 1:

```
a == b -> true
a == c -> false
a != b -> false
a != c -> true
```

Witness that `==` and `!=` dispatch only requires `PartialEq`, not
`Eq`. The `Eq` impl is a *marker* for stronger guarantees the
probe does not exercise. This bounds today's structural reading of
`impl Eq for BigUInt {}` — the impl is necessary in the context of
broader rmp use (HashMap keys, derive-generated impls, generic
bounds), not in the context of this probe's `==`/`!=` calls.

### Probe 6 — vec equality on empty vs. zero-singleton

`/tmp/eduratchet2-118/vec_zero.rs`:

```rust
fn main() {
    let empty: Vec<u64> = vec![];
    let zero: Vec<u64> = vec![0];
    println!("vec![] == vec![0] -> {}", empty == zero);
    println!("vec![] == vec![]   -> {}", empty == Vec::<u64>::new());
    println!("vec![0] == vec![0] -> {}", zero == vec![0u64]);
}
```

Output:

```
vec![] == vec![0] -> false
vec![] == vec![]   -> true
vec![0] == vec![0] -> true
```

Direct witness for the `Vec<u64>` mechanic underlying Probe 2's last
three lines. `vec![]` and `vec![0]` differ in length, so
`Vec<u64>` PartialEq returns `false` before any element comparison.

## Claim-to-Evidence Mapping

| Claim | Source |
|---|---|
| `cmp.rs:4-10` is the slice. | rmp source quoted above. |
| `impl Trait for Type { ... }` is the trait-impl form. | Lesson 111 (load-bearing). |
| `<BigUInt>` after `PartialEq` is a generic argument substitution. | Lesson 114 (load-bearing). |
| std's `PartialEq` declaration is `pub trait PartialEq<Rhs: ?Sized = Self> { ... }`. | `trait.PartialEq.md` lines 7-19, quoted above. |
| `?Sized` and `= Self` are named deferrals. | Lesson body's *What To Ignore* + this appendix's *Trait Declarations* section. |
| std's `Eq` declaration is `pub trait Eq: PartialEq { }`. | `trait.Eq.md` line 7, quoted above. |
| `Eq` has no methods. | `trait.Eq.md` line 23-24, quoted above. |
| `impl Eq for BigUInt {}` is the empty-body shape. | Lesson 116 (load-bearing); Probe 1 transcript. |
| The supertrait `Eq: PartialEq` is a named deferral. | Lesson 116's *What To Ignore* (already named); reaffirmed today. |
| `&self` is the borrowing receiver. | Lesson 100 (load-bearing). |
| `other: &BigUInt` is a non-receiver reference parameter. | Lesson 113 (load-bearing). |
| The impl method must match the trait method post-substitution. | Lesson 112 (load-bearing). |
| `bool` is the return type; field access reads `Vec<u64>`. | Lesson 012, 095, 100 (cited/load-bearing). |
| `self.limbs == other.limbs` is `Vec<u64> == Vec<u64>`. | Lesson 117 (load-bearing); Probe 6 confirms. |
| `==` desugars to a `PartialEq::eq` call. | `expressions/operator-expr.md` lines 512-516, quoted above. |
| `!=` rides std's default `ne` body. | Lesson 116; `trait.PartialEq.md` line 17 `fn ne(...) { ... }`. |
| Without the `impl PartialEq` block, `a == b` fires E0369. | Probe 4 transcript. |
| Without the `impl Eq` block, today's probe still works. | Probe 5 transcript. |
| `BigUInt::from(0u64) != BigUInt::zero()`. | Probe 2 transcript line `z == from_zero -> false`; Probe 6 confirms via the underlying `vec![] != vec![0]`. |
| Lesson 110 surfaced this defect first. | Lesson 110 *Honest observation*, *Try It* output `bad.is_zero() = false (length-only check)`. |
| rmp's existing tests pass with these impls. | Probe 3 transcript. |

## Direct Prerequisites — Specific Claims Used

- **Lesson 117** installs `vec_a == vec_b` on `Vec<T>`: `true` iff
  same length and same element values pairwise. The body
  `self.limbs == other.limbs` is exactly this on `Vec<u64>`.
- **Lesson 116** installs the empty-`{}` impl as legal exactly when
  every method the trait declared has a default body. `impl Eq for
  BigUInt {}` rides this rule.
- **Lesson 114** installs `trait T<RHS> { ... }` with the `<RHS>`
  generic-trait-parameter shape, substituted at the impl header as
  `impl T<ConcreteType> for Target`. `<BigUInt>` after `PartialEq`
  is exactly this substitution.
- **Lesson 113** installs `other: &Type` as a non-receiver reference
  parameter; the call site passes `&value`. The trait method's
  `other: &BigUInt` slot uses this shape.
- **Lesson 112** installs the contract-matching rule: the impl
  signature must match the trait's. After the impl header
  substitutes `Rhs = BigUInt`, the trait's `other: &Rhs` becomes
  `other: &BigUInt`, which is what rmp wrote.
- **Lesson 111** installs `impl Trait for Type { ... }` as a
  *trait* impl, distinct from inherent `impl Type { ... }`. Both
  blocks today use this shape.
- **Lesson 110** read rmp's `BigUInt` plus the trio end-to-end and
  surfaced the `From<u64>` non-canonicalization defect; today's
  appendix replays the defect on the `==` axis.
- **Lesson 100** installs `&self` as the borrowing receiver and
  `Self` as a type alias inside an impl. `&self` in `eq` and the
  potential `<Rhs = Self>` default both ride this.
- **Lesson 095** installs struct field access; `self.limbs` and
  `other.limbs` reuse the syntax through references.

## Older Supporting Lessons (cited only)

- 002 (`fn main`), 005 (`let`), 008 (`fn name(...) -> R { ... }`),
  011 (`println!` with `{}`), 012 (`bool`), 013 (`==`), 019 (type
  annotation slot), 025 (implicit return), 040 (dot-call shape),
  044 (`use Path::Item;` import), 080 (integer family `u64`), 003
  (diagnostic shape — cited because Probe 4's E0369 fits the same
  four-part map), 001 (`rustc demo.rs && ./demo`), 032 (`cargo new
  --vcs none rmp_driver`, `cargo run`, `cargo test`), 065
  (`[dependencies]` and the path-dependency form `bignum = { path =
  "..." }`).

## Capstone Status

This is the third capstone in the run after:

- **Lesson 063** — closed the Book's guessing-game arc (cited but
  not load-bearing today).
- **Lesson 067** — closed the second guessing-game capstone (cited
  but not load-bearing today).
- **Lesson 110** — closed the rmp BigUInt trio (load-bearing today).

Today closes the equality halves of rmp's `cmp.rs`. The remaining
21 lines of `cmp.rs` (the PartialOrd/Ord pair) are unlocked but
deferred to a later arc.
