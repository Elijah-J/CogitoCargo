# Evidence: 110-capstone-rmp-biguint-trio

Capstone for the rmp arc (lessons 095–109). The slice is rmp's
`BigUInt` declaration plus the trio `zero` / `is_zero` / `num_bits`,
and the constant `LIMB_SIZE_BITS`, all defined in
`/Users/eli/InfoScraper/output/repos/rmp/src/biguint/basic.rs`.

## Toolchain

```
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)

$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)

$ uname -srm
Darwin 24.5.0 x86_64
```

## rmp Source — Verbatim Lines

`src/lib.rs` (full file, 2 lines):

```rust
pub mod bigint;
pub mod biguint;
```

`src/biguint/mod.rs` (full file, 10 lines):

```rust
mod add;
mod basic;
mod cmp;
mod convert;
mod div;
mod format;
mod mul;
mod shift;

pub use basic::BigUInt;
```

`src/biguint/basic.rs` lines 1–54 (the slice; lines 56–78 are the
`#[cfg(test)] mod tests` block, deferred):

```rust
#[derive(Clone)]
pub struct BigUInt {
    pub(super) limbs: Vec<u64>,
}

#[derive(Debug)]
pub(super) struct Bones {
    pub(super) upper: u64,
    pub(super) lower: u64,
}

pub(crate) const LIMB_SIZE_BITS: u64 = 8 * (std::mem::size_of::<u64>() as u64);

pub(super) fn limb_to_bones(a: u64) -> Bones { ... }

#[derive(Debug)]
pub(super) struct BonesU32 { ... }

pub fn limb_to_bones_u32(a: u64) -> BonesU32 { ... }

impl BigUInt {
    pub fn zero() -> Self {
        BigUInt { limbs: vec![] }
    }

    pub fn is_zero(&self) -> bool {
        return self.limbs.len() == 0;
    }

    pub(crate) fn num_bits(&self) -> u64 {
        match self.limbs.len() {
            0 => 0,
            n => {
                (n as u64 - 1) * LIMB_SIZE_BITS
                    + (LIMB_SIZE_BITS - (self.limbs[n - 1].leading_zeros() as u64))
            }
        }
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

## Probes

Working dir: `/tmp/eduratchet-110-probe`. Five probes total.

### Probe 1 — Self-contained probe (`probe.rs`)

This is the file checked in at
`observations/110-capstone-rmp-biguint-trio.rs`. It replicates the
slice as a single-file program. The `pub(super) limbs` from rmp is
replaced with `pub limbs` because the standalone file has no parent
module to reach (`pub(super)` at the crate root has empty reach).

Compilation and run transcript:

```
$ rustc 110-capstone-rmp-biguint-trio.rs -o probe
$ ./probe
LIMB_SIZE_BITS = 64
zero.limbs.len() = 0
zero.is_zero() = true
zero.num_bits() = 0
one.is_zero() = false
one.num_bits() = 1
v256.num_bits() = 9
max1.num_bits() = 64
two_64.num_bits() = 65
max2.num_bits() = 128
bad.is_zero() = false  (length-only check)
bad.num_bits() = 0
$ echo $?
0
```

Hand-computation cross-check for each non-zero output:

- `LIMB_SIZE_BITS = 64`: `8 * size_of::<u64>() = 8 * 8 = 64`.
  `size_of::<u64>() == 8` per `std/mem/fn.size_of.md` line 30
  (table row "u64 | 8").
- `one.num_bits() = 1`: `n=1`, lower = 0, MSL =
  `64 - leading_zeros(1) = 64 - 63 = 1`.
- `v256.num_bits() = 9`: `0x100 = 256 = 2^8`, so the highest set
  bit is bit 8 (the 9th bit). `leading_zeros(0x100) = 64 - 9 = 55`.
  MSL = `64 - 55 = 9`.
- `max1.num_bits() = 64`: `u64::MAX = 2^64 - 1`,
  `leading_zeros(u64::MAX) = 0`. MSL = `64 - 0 = 64`.
- `two_64.num_bits() = 65`: two limbs, `n=2`, lower = `(2-1)*64 = 64`,
  MSL is `limbs[1] = 1`, `leading_zeros(1) = 63`, MSL term = `64 - 63
  = 1`. Total: 65. (Value: `0 + 1 * 2^64 = 2^64`, which is the
  smallest 65-bit value.)
- `max2.num_bits() = 128`: two limbs both `u64::MAX`. lower =
  `(2-1)*64 = 64`. MSL is `u64::MAX`, MSL term = `64 - 0 = 64`.
  Total: 128. (Value: `2^128 - 1`.)
- `bad.num_bits() = 0`: `vec![0]`, `n=1`, lower = 0, MSL =
  `64 - leading_zeros(0) = 64 - 64 = 0`. The `match` arm `0 => 0`
  doesn't fire because `len == 1`, but the algorithm gives 0
  anyway.
- `bad.is_zero() = false`: `len == 1`, not 0. Length-only check,
  not value-equality.

All outputs match predictions.

### Probe 2 — Cross-crate driver: zero & is_zero work

Working dir: `/tmp/eduratchet-110-probe/rmp_driver`. `Cargo.toml`:

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
    let z = BigUInt::zero();
    println!("BigUInt::zero().is_zero() = {}", z.is_zero());
}
```

Run:

```
$ cargo run
   Compiling bignum v0.1.0 (/Users/eli/InfoScraper/output/repos/rmp)
warning: unused variable: `index`
  --> /Users/eli/InfoScraper/output/repos/rmp/src/biguint/mul.rs:25:14
warning: method `is_nonnegative` is never used
  --> /Users/eli/InfoScraper/output/repos/rmp/src/bigint.rs:73:8
warning: `bignum` (lib) generated 2 warnings
   Compiling rmp_driver v0.1.0 (/private/tmp/eduratchet-110-probe/rmp_driver)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/rmp_driver`
BigUInt::zero().is_zero() = true
$ echo $?
0
```

(The two warnings are in unrelated rmp source — `mul.rs:25` has an
unused param, `bigint.rs:73` has an unused method. Out of today's
slice. They are not errors and the build succeeds.)

Witnesses:
- `use bignum::biguint::BigUInt;` resolves — the path the slice
  claims (`bignum::biguint::BigUInt`) is reachable from outside the
  crate.
- `BigUInt::zero()` is callable — `pub` reaches across the crate
  boundary.
- `z.is_zero()` is callable and returns `true` — same.

### Probe 3 — Cross-crate contrast: `num_bits` is `pub(crate)`

`src/bin/contrast_num_bits.rs`:

```rust
use bignum::biguint::BigUInt;

fn main() {
    let z = BigUInt::zero();
    let n = z.num_bits();
    println!("num_bits = {}", n);
}
```

Build:

```
$ cargo build --bin contrast_num_bits
... (rmp warnings) ...
error[E0624]: method `num_bits` is private
  --> src/bin/contrast_num_bits.rs:10:15
   |
10 |     let n = z.num_bits();
   |               ^^^^^^^^ private method
   |
  ::: /Users/eli/InfoScraper/output/repos/rmp/src/biguint/basic.rs:45:5
   |
45 |     pub(crate) fn num_bits(&self) -> u64 {
   |     ------------------------------------ private method defined here

For more information about this error, try `rustc --explain E0624`.
error: could not compile `rmp_driver` (bin "contrast_num_bits") due to 1 previous error
```

E0624. The `note:` block points back at the exact `pub(crate) fn
num_bits(&self) -> u64 {` line in rmp's source. Witnesses that
`pub(crate)` does not reach external callers — same exhaustion shape
lesson 103 installed for fn items, witnessed today on the real rmp
target.

### Probe 4 — Contrast: re-export drop simulated by reaching basic directly

`src/bin/contrast_basic_path.rs`:

```rust
use bignum::biguint::basic::BigUInt;

fn main() {
    let _z = BigUInt::zero();
}
```

Build:

```
$ cargo build --bin contrast_basic_path
... (rmp warnings) ...
error[E0603]: module `basic` is private
 --> src/bin/contrast_basic_path.rs:7:22
  |
7 | use bignum::biguint::basic::BigUInt;
  |                      ^^^^^ private module
  |
note: the module `basic` is defined here
 --> /Users/eli/InfoScraper/output/repos/rmp/src/biguint/mod.rs:2:1
  |
2 | mod basic;
  | ^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
```

E0603 with caret on the `basic` segment of the path; `note:` block
points at the `mod basic;` line in `biguint/mod.rs`. Witnesses that
`mod basic;` (no `pub`) keeps `basic` private — and therefore the
`pub use basic::BigUInt;` line on `mod.rs:10` is the load-bearing
re-export that lifts `BigUInt` into the `biguint` namespace.

### Probe 5 — Contrast: `pub(super)` field unreachable from outside

`src/bin/contrast_limbs_field.rs`:

```rust
use bignum::biguint::BigUInt;

fn main() {
    let z = BigUInt::zero();
    let _len = z.limbs.len();
}
```

Build:

```
$ cargo build --bin contrast_limbs_field
... (rmp warnings) ...
error[E0616]: field `limbs` of struct `BigUInt` is private
  --> src/bin/contrast_limbs_field.rs:11:18
   |
11 |     let _len = z.limbs.len();
   |                  ^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
```

E0616. `pub(super)` on the field reaches only `biguint`'s parent
module (the crate root, `lib.rs`), not external callers. Witness
that the field-level visibility shape composed today actually fires
the expected error from outside.

### Probe 6 — rmp's own test passes

```
$ cd /Users/eli/InfoScraper/output/repos/rmp
$ cargo test --lib num_bits
... (rmp warnings) ...
running 1 test
test biguint::basic::tests::num_bits_correct ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s
```

The rmp project's own test for `num_bits` passes. The test
(`basic.rs:60-72`) checks that `BigUInt::from(0u64).num_bits() == 0`,
`BigUInt::from(1u64).num_bits() == 1`, `BigUInt::from(1u64 <<
32).num_bits() == 33`, and that `1u64 << shift` for `shift in
0..100` produces `num_bits() == shift + 1`. Independent grounding
that the `num_bits` algorithm walked in today's lesson is correct on
the rmp project's own canonical inputs.

### Probe 7 — Honest observation: `From<u64>` non-canonicalization

`src/bin/check_canonical.rs`:

```rust
use bignum::biguint::BigUInt;

fn main() {
    let z1 = BigUInt::zero();
    let z2 = BigUInt::from(0u64);
    println!("zero().is_zero()      = {}", z1.is_zero());
    println!("from(0u64).is_zero()  = {}", z2.is_zero());
}
```

Run:

```
$ cargo run --bin check_canonical
... (rmp warnings) ...
zero().is_zero()      = true
from(0u64).is_zero()  = false
```

Witnesses that `BigUInt::from(0u64)` produces a one-limb-zero
`BigUInt` (because `convert.rs:18` writes `BigUInt { limbs: vec![n]
}` without trimming when `n == 0`), and `is_zero` returns `false` on
it. The canonical-form invariant *no trailing-zero limbs* is a
maintainer-side discipline, not a runtime check inside `is_zero`.

## Corpus Quote Map (substantive claims → source)

| Claim in the lesson | Source |
|---------------------|--------|
| `bignum` crate name | `/Users/eli/InfoScraper/output/repos/rmp/Cargo.toml` line 2 `name = "bignum"` |
| `lib.rs` declares `pub mod biguint;` | `/Users/eli/InfoScraper/output/repos/rmp/src/lib.rs` line 2 |
| `biguint/mod.rs` declares `mod basic;` (no `pub`) | `src/biguint/mod.rs` line 2 |
| `biguint/mod.rs` declares `pub use basic::BigUInt;` | `src/biguint/mod.rs` line 10 |
| `BigUInt` struct declaration | `src/biguint/basic.rs` lines 1–4 |
| `LIMB_SIZE_BITS` declaration | `src/biguint/basic.rs` line 12 |
| `zero`, `is_zero`, `num_bits` definitions | `src/biguint/basic.rs` lines 36–54 |
| `From<u64>` builds `vec![n]` without trimming | `src/biguint/convert.rs` lines 16–20 |
| `pub use ...` is a re-export form | Lesson 105 (load-bearing prerequisite); Reference `items/use-declarations.md` |
| `mod foo;` resolves to a sibling file `foo.rs` | Lesson 097 (load-bearing) |
| `mod foo;` may also resolve to a sibling directory `foo/mod.rs` | Lesson 106 (load-bearing) |
| `Item → ... ( VisItem ...) ; VisItem → Visibility? ( ... Struct ... ConstantItem ... )` | `output/docs/rust/reference/items.md` lines 10–29 |
| `StructField → OuterAttribute* Visibility? IDENTIFIER : Type` | `output/docs/rust/reference/items/structs.md` line 22 |
| `pub(super)` reaches the parent module | `output/docs/rust/reference/visibility-and-privacy.md` line 141; lesson 103 (load-bearing) |
| `pub(crate)` reaches all of the crate | Lesson 103 (load-bearing); same Reference page |
| `pub` cross-crate reach | Lesson 096 (load-bearing) |
| `Vec<u64>` field type semantics | Lesson 107 (load-bearing) |
| `vec![]` empty-Vec construction | Lesson 107; `output/docs/rust/std/macro.vec.md` lines 7–11 |
| `.len()` returns `usize` | Lesson 107; `output/docs/rust/std/vec/struct.Vec.md` lines 2114–2126 |
| `v[i]` with `i: usize` reads element | Lesson 107; `output/docs/rust/std/vec/struct.Vec.md` lines 84–92 |
| `n.leading_zeros()` returns `u32` count of leading zeros in `u64` | Lesson 108; `output/docs/rust/std/primitive.u64.md` lines 113–115 |
| `leading_zeros(0u64) == 64` | Lesson 108 probe; `std/primitive.u64.md` lines 126–127 |
| `leading_zeros(u64::MAX) == 0` | Lesson 108 probe; `std/primitive.u64.md` lines 129–130 |
| `as u64` cast from `usize`/`u32` | Lesson 034 (load-bearing in earlier graph; cited today) |
| `match self.limbs.len() { 0 => 0, n => ... }` | Lesson 031 (match on integer with wildcard) and lesson 058 (binding pattern); cited |
| `&self` method shape | Lesson 100 (load-bearing) |
| `Self` as a return type | Lesson 100 (load-bearing) |
| `BigUInt { limbs: vec![] }` struct literal | Lesson 095 (load-bearing) |
| `fn name() -> Self { ... }` associated function | Lesson 100 (load-bearing) |
| `pub(crate) const NAME: TYPE = value;` at module scope | Lesson 109 (load-bearing) |
| `size_of::<u64>() == 8` (bytes) | `output/docs/rust/std/mem/fn.size_of.md` line 30 (table row `u64 | 8`) |
| `8 * size_of::<u64>() = 64` | Probe 1 line `LIMB_SIZE_BITS = 64` |
| Canonical `BigUInt` little-endian limb order | Implied-and-witnessed: `num_bits` reads `self.limbs[n-1].leading_zeros()` in `basic.rs:50`. The expression only makes sense if `limbs[n-1]` is the most-significant limb. No corpus statement says "limbs are little-endian" outside this implementation; the framing is grounded in the algorithm itself. |
| Canonical `BigUInt` no-trailing-zero invariant | Same: implied by `num_bits` returning a sensible bit count only if the MSL is non-zero (otherwise `64 - leading_zeros = 0` and the count is undercounted). |
| Empty `Vec` represents 0 | `BigUInt::zero` definition `BigUInt { limbs: vec![] }` (`basic.rs:38`) directly. |
| `pub(super)` field protects invariants | Reasoned: external callers cannot construct `BigUInt { limbs: ... }` because the field is field-private from outside the crate; Probe 5 witnesses E0616. |
| `From<u64>` non-canonicalization observation | Probe 7 transcript. |
| E0624 on `pub(crate)` from outside | Probe 3 transcript. |
| E0603 on `mod basic;` from outside | Probe 4 transcript. |
| E0616 on `pub(super)` field from outside | Probe 5 transcript. |
| `match` integer-pattern-with-binding | Lesson 031 + lesson 058 |
| `return` statement (operationally equivalent to implicit return) | Lesson 025's deferral; lesson 008 used `return` in early bodies |

## Direct Prerequisite Claim Summaries

The rmp arc spans 16 lessons. Each direct prerequisite installs a
specific claim that today reuses verbatim. Older lessons are cited
by number only when their claim is restated by a closer lesson.

### 095 — `struct Name { field: Type, }`

- Field-by-name access `value.field` reads a field; today reuses
  this via `self.limbs` inside `is_zero`.
- Fields without a visibility marker are private; today the
  `pub(super) limbs` modifier is the explicit alternative to
  default-private.
- Struct literal `Name { field1: value1, ... }` constructs an
  instance; today reuses inside `BigUInt { limbs: vec![] }`.

### 096 — `mod foo { ... }` and `pub`

- `pub` in front of an item makes it visible from outside its
  module; today applies to `pub struct BigUInt`, `pub fn zero`,
  `pub fn is_zero`.
- `mod foo;` (with `pub` or without) declares a child module; today
  reads `pub mod biguint;` in `lib.rs`.
- E0603 fires on a visibility failure with `note:` pointing at the
  definition; today's Probe 4 carries this exact transcript shape.

### 097 — `mod foo;` resolves to file `foo.rs`

- Today applies the rule to read `mod basic;` in `biguint/mod.rs`,
  resolving to `biguint/basic.rs`.

### 098 — enum unit variants

- Cited only. The trio doesn't declare an enum, but `Ordering`
  (used in 051) and `Result` (058) follow the same shape that 098
  installed for user-declared enums. Out of today's slice but in
  the rmp arc as a foundational shape.

### 099 — enum tuple variants

- Cited only. Same reasoning as 098.

### 100 — `impl Type { ... }`, `fn name() -> Self`, `&self`, `Self`

- Today reuses all four pieces:
  - `impl BigUInt { ... }` block.
  - `fn zero() -> Self` associated function.
  - `fn is_zero(&self) -> bool` and `fn num_bits(&self) -> u64`
    `&self` methods.
  - `Self` as the return type of `zero`.

### 101 — `&mut self`

- Cited only. The trio is read-only; `&mut self` is used elsewhere
  in rmp (`add.rs`, `mul.rs`, `div.rs`) but not here.

### 102 — `self` (consuming)

- Cited only. The trio doesn't consume `self`; the receiver shape
  is named in lesson 108 (where `n.leading_zeros()` takes `self`
  but `u64` is `Copy`, so the consume rule is invisible).

### 103 — `pub(super)` and `pub(crate)`

- Today reuses both modifiers:
  - `pub(crate) const LIMB_SIZE_BITS` — module-scope const item
    visibility (composed with lesson 109 today).
  - `pub(crate) fn num_bits` — method visibility on an inherent
    impl item.
  - `pub(super) limbs: Vec<u64>` — *field* visibility. Lesson 103's
    *What To Ignore* explicitly named "Restricted visibility on
    struct fields — composes today's modifier with lesson 095's
    field declarations" as the natural next move; today closes that
    composition, grounded by Reference `items/structs.md` line 22's
    grammar `StructField → ... Visibility? IDENTIFIER : Type`.
- E0603 (item) and E0616 (field) and E0624 (method) all fire from
  outside the crate; Probes 3, 4, 5 witness each.

### 104 — `super::` and `crate::` paths

- Cited only. The trio's source uses neither; the *meaning* of
  `pub(super)` (parent-module reach) is named here. The rmp source
  uses `super::` once, in `convert.rs:1` `use super::basic::BigUInt;`,
  but that line is outside today's slice.

### 105 — `pub use Path::Item;`

- Today reads `pub use basic::BigUInt;` in `biguint/mod.rs:10`
  exactly as 105 installed: re-export `Item` from path
  `Path::Item` with `pub` visibility. The contrast Probe 4
  witnesses what fails *without* the re-export (E0603 on
  `basic`).

### 106 — subdirectory module (`foo/mod.rs`)

- The `pub mod biguint;` line in `lib.rs` resolves to
  `src/biguint/mod.rs` rather than `src/biguint.rs`; lesson 106's
  rule covers this exact disambiguation.

### 107 — `Vec<T>` basics: `vec![]`, `.len()`, `v[i]`

- All three operations appear in the trio:
  - `vec![]` (empty form) in `BigUInt::zero` body.
  - `self.limbs.len()` in `is_zero` and `num_bits`.
  - `self.limbs[n - 1]` in `num_bits`.

### 108 — `u64::leading_zeros`

- `self.limbs[n - 1].leading_zeros()` is exactly 108's call shape
  with a `Vec`-indexed `u64` as the receiver. Returns `u32`; cast
  to `u64` via 034's `as` cast.

### 109 — `pub(crate) const NAME: TYPE = value;` at module scope

- `LIMB_SIZE_BITS` is exactly 109's shape. The right-hand side
  `8 * (std::mem::size_of::<u64>() as u64)` is a constant expression
  per lesson 109's deferred details, with `size_of::<u64>()` itself
  named-but-deferred.

## Older Lessons Cited (by number only)

- **002** `fn main` entry point; the host of all probes.
- **005** `let` binding; used in probes 1, 2, 7 to bind `BigUInt`
  values.
- **008** `fn name() -> T` definition shape; used in `is_zero` and
  `num_bits`.
- **011** `println!` with `{}` placeholders; used in all probes.
- **013** comparison operators; `== 0` in `is_zero`.
- **019** type-annotation slot; `: u64`, `: bool`, `: u64`.
- **025** implicit return; `is_zero` uses explicit `return` instead,
  operationally equivalent.
- **030/031** `match` on `bool` / integer with wildcard; today
  matches on a `usize` (the result of `len()`) with arms `0` and
  `n` (binding identifier).
- **034** `as` cast; `n as u64` and `... as u64` in `num_bits`.
- **040** method-call syntax; `value.method()`.
- **044** `use Path::Item;`; the cross-crate driver writes `use
  bignum::biguint::BigUInt;`.
- **058** `match` arm with binding pattern (originally on a
  `Result` payload); today the binding pattern `n => ...` is the
  same slot shape.
- **071** macro invocation; `vec![]` and `println!()` are macros.
- **080** the integer-type family naming `u64`.

## Deferred / Out-of-Scope Notes

Items the lesson explicitly defers and grounds:

- `std::mem::size_of::<T>()`. The value `size_of::<u64>() == 8` is
  in the std doc table at `std/mem/fn.size_of.md:30`. The call
  syntax `::<T>` (turbofish) and the `mem` module are not yet
  installed concepts in the graph; a future move installs them.
- `Bones`, `BonesU32`, `limb_to_bones`, `limb_to_bones_u32` in
  `basic.rs`. Out of slice; named only.
- `#[derive(Clone)]` on `BigUInt`. Lesson 095's *What To Ignore*
  explicitly defers derive macros; today honors that.
- `#[cfg(test)] mod tests { ... }` block; testing infrastructure.
  Probe 6 invokes the `cargo test` form indirectly to confirm rmp's
  own test passes, but the `#[test]` attribute and `assert_eq!`
  macro are not installed.
- The rest of rmp (`bigint.rs`, the `cmp`/`add`/`mul`/`div`/
  `format`/`convert`/`shift` modules). The trait arc unlocks
  `Add`/`Sub`/`Mul`/`Div`/`Display`/`From`/`TryFrom`/`PartialOrd`
  etc.; deeper algorithmic methods compose those traits with the
  arithmetic-method families lesson 108 named.

## Lines This Lesson Does Not Fully Explain

The slice is fully reduced to installed lessons except:

- `std::mem::size_of::<u64>()` — call form deferred; numeric value
  grounded by the std doc table.
- `#[derive(Clone)]` on the `BigUInt` declaration in the rmp source
  — the slice's struct decl as printed in this lesson omits the
  derive line and treats it as "decorate, deferred."
- The `match` arm `n => { ... }` uses an identifier-pattern in a
  position the graph hasn't formally installed independently. Lesson
  058 installed the same syntactic shape for the `Result` payload's
  `Ok(n)`/`Err(_)`; the standalone identifier-pattern in a top-level
  match arm is operationally the same slot. Today reuses without a
  new install.

No other line in the slice resists reduction to installed concepts.

## Risks / Honest Caveats

- **Canonical-form invariant grounding.** The "limbs are little-
  endian" and "no trailing-zero limbs" claims are *inferred from
  algorithm behavior*, not stated in any rmp comment or doc. The
  inference is sound (the algorithm only makes sense under these
  assumptions) but the lesson is honest that no rmp-side documentation
  claims them. Probe 7's `From<u64>` non-canonicalization observation
  is the strongest direct witness that "constructor-side invariant
  maintenance" is a per-method discipline, not a project-wide
  enforced rule.
- **`is_zero` is not value-equality.** A user reading `is_zero` who
  expects "returns true iff value == 0" will be surprised by Probe
  7. The lesson surfaces this honestly rather than papering over it.
- **`pub(super)` on a field.** Lesson 103 deferred this; today
  composes 103's modifier with 095's struct field grammar via the
  Reference `items/structs.md` line 22 grammar fragment, which
  *does* admit `Visibility?` in the `StructField` rule. Probe 5
  empirically witnesses the field-private behavior from outside the
  crate (E0616). The composition is sound; whether it deserves its
  own pre-capstone install lesson is a judgment call. Today's
  capstone framing makes the composition explicit; if a future
  cycle needs the modifier in non-capstone position it should still
  install a dedicated lesson.
- **Two warnings in rmp's lib build** (`mul.rs:25` unused param,
  `bigint.rs:73` unused method) appear in every probe transcript
  but are out of slice; named honestly.
