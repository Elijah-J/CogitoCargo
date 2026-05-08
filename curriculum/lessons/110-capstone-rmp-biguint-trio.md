---
id: 110-capstone-rmp-biguint-trio
status: accepted
evidence: ../evidence/110-capstone-rmp-biguint-trio.md
---

# Capstone: read rmp's `BigUInt` plus the trio `zero` / `is_zero` / `num_bits`

## The Slice

This is a *Capstone Mode* cycle, and the third capstone in this run
(after 063 and 067 closed the Book's guessing-game path). **No new
Rust mechanic.** The rmp arc — lessons 095 through 109, fifteen
moves — was assembled to make one specific real-world slice readable
end-to-end: `BigUInt` plus the foundational trio `zero`, `is_zero`,
`num_bits` in rmp's `src/biguint/basic.rs`. Today reads it.

The slice is small enough to fully explain at lesson length, but it
exercises *every* axis the rmp arc installed: the module-path chain
from `lib.rs` to `BigUInt`, a `pub` struct with a `pub(super)` field
and a `Vec<u64>` field type, an `impl` block with three methods
covering the shapes lessons 100–102 installed (associated function,
`&self` method), the `pub(crate)` visibility shape, and an
algorithmic method `num_bits` that composes `Vec`'s `.len()` plus
indexing, the `leading_zeros` integer method, an `as` cast, and
arithmetic — every named ingredient already installed.

The literal source from `src/biguint/basic.rs`:

```rust
pub(crate) const LIMB_SIZE_BITS: u64 = 8 * (std::mem::size_of::<u64>() as u64);

pub struct BigUInt {
    pub(super) limbs: Vec<u64>,
}

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

(There is also a `Bones` struct, a `BonesU32` struct, a
`limb_to_bones` function, and a `tests` module in the same file —
all out of today's slice and named in *What To Ignore*.)

## The Module-Path Chain

`src/main.rs` reaches `BigUInt` as `bignum::biguint::BigUInt`. Walk
the chain:

- `bignum` is the *crate name*. The `Cargo.toml` line `name =
  "bignum"` (lesson 032's manifest shape) sets it. The crate root is
  `src/lib.rs` (because the package is a library).
- `src/lib.rs` declares `pub mod biguint;`. By lesson 097's
  file-based-module rule, `mod biguint;` resolves to a sibling
  *directory* `biguint/` containing `mod.rs` (lesson 106's
  subdirectory-module form, since both `biguint.rs` and `biguint/mod.rs`
  cannot coexist and the directory shape is what rmp uses).
- `src/biguint/mod.rs` declares `mod basic;` (no `pub`). By lesson
  097, this resolves to `biguint/basic.rs`, the file holding the
  type. Plain `mod basic;` makes `basic` *private* to its parent
  `biguint` — so the path `bignum::biguint::basic::BigUInt` is
  unreachable from outside `biguint`.
- The next line, `pub use basic::BigUInt;`, is lesson 105's
  re-export. It lifts `BigUInt` *out of* the private `basic` and
  *into* `biguint` itself, with `pub` visibility. That is what makes
  the path `bignum::biguint::BigUInt` resolve.
- Inside `biguint/basic.rs`, `pub struct BigUInt { ... }` is the
  declaration the re-export points at.

The cross-crate evidence probe (in the appendix) confirms both
halves: `use bignum::biguint::BigUInt;` works, and the contrast
`use bignum::biguint::basic::BigUInt;` fails with E0603 *"module
`basic` is private"* with the `note:` block pointing at line 2 of
`mod.rs` underlining the exact `mod basic;` line.

## The Type Declaration, Token by Token

```rust
pub struct BigUInt {
    pub(super) limbs: Vec<u64>,
}
```

Every token has a lesson:

- `pub` (096): visibility modifier in front of the `struct` item.
  Cross-crate-visible.
- `struct` (095): the keyword that introduces a named-fields type.
- `BigUInt`: the type's name, identifier-rules unchanged.
- `{ ... }`: braces enclose the field list.
- `pub(super)` (103): visibility modifier — but on a *field*, a
  position lesson 103 named in *What To Ignore* and grounded in the
  Reference grammar `StructField → OuterAttribute* Visibility?
  IDENTIFIER : Type` (`reference/items/structs.md` line 22). Today
  composes lesson 103's modifier into the Visibility slot of the
  field grammar. The reach is the *parent* of the declaring module.
  The declaring module is `basic` (the file `biguint/basic.rs`), so
  the parent is `biguint` — meaning `biguint/mod.rs` and the sibling
  files inside `biguint/` (`add.rs`, `cmp.rs`, `convert.rs`, `mul.rs`,
  `div.rs`, `format.rs`, `shift.rs`) can read `limbs`. It does *not*
  reach `lib.rs` (which is the parent of `biguint`, not of `basic`)
  or `bigint.rs` (a sibling of `biguint`, not of `basic`). External
  crates cannot reach it either. Witness: rmp's own
  `biguint/convert.rs:1` writes `use super::basic::BigUInt;` —
  exactly the same parent-of-`basic` relationship that `pub(super)`
  on `limbs` rides.
- `limbs`: the field's name.
- `Vec<u64>` (107 + 080): a heap-backed growable list of `u64`
  elements. The annotation slot from lesson 019 with a `Vec`
  parameterized by `u64`.

The contrast probe `use bignum::biguint::BigUInt; let z =
BigUInt::zero(); let _ = z.limbs.len();` fires `error[E0616]: field
`limbs` of struct `BigUInt` is private` from outside the crate —
witness that `pub(super)` does not reach external callers.

## Representation Invariants

`BigUInt` represents an arbitrary-precision unsigned integer. The
invariants that the trio depends on (and that other rmp methods
maintain on output):

1. **Limb order is little-endian.** `limbs[0]` is the
   *least-significant* 64-bit limb; `limbs[n-1]` is the
   *most-significant*. Witnessed by `num_bits`'s use of
   `self.limbs[n - 1].leading_zeros()` — counting leading zeros only
   makes sense if `limbs[n-1]` is the most-significant limb. The
   value of a `BigUInt` is therefore `sum_{i=0..n} limbs[i] *
   2^(64*i)`.
2. **No trailing-zero limbs.** A canonical `BigUInt` has either an
   empty `Vec` or a non-zero most-significant limb. Every method
   that *produces* a `BigUInt` is responsible for trimming.
3. **Empty Vec means zero.** `BigUInt { limbs: vec![] }` is the
   canonical representation of the value 0.
4. **`pub(super)` on `limbs`** keeps external callers from
   constructing a `BigUInt` with arbitrary contents — they go
   through associated functions like `zero` or `From<u64>` instead,
   which protect the invariants.

Honest observation. The `From<u64>` impl in `convert.rs` builds
`BigUInt { limbs: vec![n] }` *without* trimming when `n == 0`. The
cross-crate driver in the appendix witnesses
`BigUInt::from(0u64).is_zero() == false` — the invariant is a
*maintainer-side discipline*, not enforced at runtime by `is_zero`.

## The Trio, Method by Method

### `LIMB_SIZE_BITS`

```rust
pub(crate) const LIMB_SIZE_BITS: u64 = 8 * (std::mem::size_of::<u64>() as u64);
```

- `pub(crate)` (103) + `const NAME: TYPE = value;` at module scope
  (109): exactly the shape lesson 109 installed.
- The right-hand side: `std::mem::size_of::<u64>()` is a generic
  associated function (deferred — the `::<T>` turbofish syntax is
  not yet installed). The std `mem/fn.size_of.md` page tabulates
  `size_of::<u64>() == 8` (size in bytes). The `as u64` cast (034)
  turns the `usize` return into `u64`, then `8 *` multiplies bytes
  by 8 bits/byte. Compile-time constant: 64. The probe prints
  `LIMB_SIZE_BITS = 64`.

### `zero`

```rust
pub fn zero() -> Self {
    BigUInt { limbs: vec![] }
}
```

- `pub fn` (096), `Self` return type (100): an associated function
  on `BigUInt`. No receiver — called as `BigUInt::zero()`.
- `BigUInt { limbs: vec![] }`: the named-fields struct literal
  (095) with the empty `vec![]` form (107) for the field. This is
  the canonical zero — empty Vec, length 0.

### `is_zero`

```rust
pub fn is_zero(&self) -> bool {
    return self.limbs.len() == 0;
}
```

- `&self` (100): a read-only method called as `value.is_zero()`.
- `self.limbs` reads the `limbs` field through the receiver —
  field-access on a `&self` receiver, the same `value.field`
  syntax lesson 095 installed. Returns `&Vec<u64>` here (auto-deref;
  the body is reading `self.limbs.len()`, not consuming).
- `.len()` (107) returns the element count as `usize`.
- `== 0` is integer equality (013), comparing `usize` to the integer
  literal `0` which infers to `usize`.
- `return` (deferred since lesson 025 in favour of implicit return)
  is operationally equivalent to the implicit form
  `self.limbs.len() == 0` and is what rmp wrote.

The function returns `true` exactly when `limbs` is empty — i.e.,
the *canonical* zero. Non-canonical zeros like `BigUInt { limbs:
vec![0] }` return `false`, which is why constructor-side trimming
matters.

### `num_bits` — the algorithmic method

```rust
pub(crate) fn num_bits(&self) -> u64 {
    match self.limbs.len() {
        0 => 0,
        n => {
            (n as u64 - 1) * LIMB_SIZE_BITS
                + (LIMB_SIZE_BITS - (self.limbs[n - 1].leading_zeros() as u64))
        }
    }
}
```

- `pub(crate)` (103): visible across the crate, but not from outside.
  The cross-crate driver's contrast `z.num_bits()` fires E0624
  *"method `num_bits` is private"*.
- `match self.limbs.len() { ... }` (031): match on the length, a
  `usize`. The patterns are an integer literal `0` and an
  identifier-pattern `n` — 058's identifier-binding pattern reused
  at the top-level scrutinee (the final-arm slot where 031 used `_`).
  - Arm `0 => 0`: empty Vec means canonical zero. Zero bits.
  - Arm `n => { ... }`: `n` binds the length (≥ 1).
- The non-empty arm: `(n as u64 - 1) * LIMB_SIZE_BITS + (LIMB_SIZE_BITS
  - (self.limbs[n - 1].leading_zeros() as u64))`. Two pieces, both
  named.
  - **Lower-limbs term** `(n as u64 - 1) * LIMB_SIZE_BITS`: there are
    `n - 1` limbs *below* the most-significant one (`limbs[0..n-1]`),
    each holding a full 64 bits. `n as u64` casts the `usize` count
    to `u64` (034) so the arithmetic stays in `u64` and matches the
    return type. Subtract 1 for the most-significant limb counted
    separately. Multiply by `LIMB_SIZE_BITS`.
  - **MSL term** `LIMB_SIZE_BITS - (self.limbs[n - 1].leading_zeros()
    as u64)`: `self.limbs[n - 1]` reads the most-significant limb by
    index (107). `.leading_zeros()` (108) returns the count of
    leading zero bits as `u32`. `as u64` casts to `u64`. `64 -
    leading_zeros` is the count of bits *actually used* in the MSL.

Worked example. `BigUInt { limbs: vec![0x100] }` — value 256, one
limb `0x100`. `n = 1`. Lower-limbs term: `(1 - 1) * 64 = 0`. MSL term:
`leading_zeros(0x100) = 55` (since `0x100 = 0b1_0000_0000` has 8
trailing data plus 1 leading 1, total 9 used bits, 64-9 = 55 leading
zeros). MSL term: `64 - 55 = 9`. Total: 9 bits. The probe prints
`v256.num_bits() = 9`.

Edge cases the probe witnesses. `vec![1]` → 1 bit. `vec![u64::MAX]`
→ 64. `vec![0, 1]` (value 2^64) → 65. `vec![u64::MAX, u64::MAX]`
(value 2^128 - 1) → 128. `BigUInt { limbs: vec![0] }` (non-canonical
zero) → 0, because `leading_zeros(0) == 64` makes the MSL term zero.

## Mental Model Delta

- *Before lesson 095:* "I have read tiny `fn main` programs and
  toy Cargo packages, plus the Book's guessing-game capstone. Real
  Rust libraries are still opaque."
- *After lesson 109:* "I have installed every named ingredient the
  trio uses, but I have not read a real-world slice end-to-end."
- *After today:* "I can read `bignum::biguint::BigUInt` plus the
  trio `zero`, `is_zero`, `num_bits` in rmp's `biguint/basic.rs` —
  every token, every line, every visibility modifier, every
  arithmetic step — using only the 109 installed concepts plus
  `std::mem::size_of::<u64>() == 8` looked up in the std doc table.
  I can also predict each method's behavior on hand-built inputs and
  confirm the predictions empirically."

## Prerequisites

- Installed concepts (the rmp arc, sixteen lessons including today):
  - **Lesson 095** (load-bearing): `struct Name { field: Type, }`,
    field access `value.field`, and field-private-by-default.
  - **Lesson 096** (load-bearing): `pub` and `mod foo { ... }`, the
    E0603 diagnostic for visibility failures.
  - **Lesson 097** (load-bearing): `mod foo;` resolves to a sibling
    file `foo.rs`. Used to read `mod basic;` in `biguint/mod.rs`.
  - **Lessons 098, 099, 101, 102, 104** (cited only):
    098 (enum unit variants); 099 (enum tuple variants); 101
    (`&mut self`); 102 (`self` by value); 104 (`super::` / `crate::`
    paths). Per-claim summaries live in the evidence appendix.
  - **Lesson 100** (load-bearing): `impl Type { ... }`, associated
    functions (`fn zero() -> Self`), `&self` methods, `Self` as a
    return type.
  - **Lesson 103** (load-bearing): `pub(super)` and `pub(crate)`.
    Used both as a *field* visibility on `limbs` (composing into the
    Reference's `StructField → ... Visibility? IDENTIFIER : Type`
    grammar slot named-but-deferred in lesson 103) and as an *item*
    visibility on `num_bits` and `LIMB_SIZE_BITS`.
  - **Lesson 105** (load-bearing): `pub use Path::Item;` re-export.
    The `pub use basic::BigUInt;` line in `biguint/mod.rs` is exactly
    this shape.
  - **Lesson 106** (load-bearing): subdirectory module —
    `biguint/mod.rs`. The `mod biguint;` line in `lib.rs` resolves
    to this file.
  - **Lesson 107** (load-bearing): `vec![]` (empty), `.len()`,
    indexing `v[i]`. All three operations appear in the trio.
  - **Lesson 108** (load-bearing): `n.leading_zeros()` on a `u64`,
    returning `u32`.
  - **Lesson 109** (load-bearing): `pub(crate) const NAME: TYPE =
    value;` at module scope. Exactly `LIMB_SIZE_BITS`'s shape.
  - **Earlier lessons** (cited only — none load-bearing today):
    002, 005, 011, 013, 019, 025, 030, 031, 034, 040, 044, 058, 071,
    080. Each named in the corpus appendix with the specific carry-
    through claim.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` and `cargo` on `PATH`, ability to `cd` into a directory.
  All four already used since lesson 001 / 032.

## Try It

Two probes, in order. The first is fully self-contained and lives at
`observations/110-capstone-rmp-biguint-trio.rs`.

```console
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
```

The probe replicates the slice (with `pub(super)` on `limbs`
replaced by `pub` because the standalone file has no parent module
to reach) and calls every method on hand-built inputs. Predict each
line before running.

The second probe — optional but informative — is a *cross-crate
driver* that depends on the actual rmp library at
`/Users/eli/InfoScraper/output/repos/rmp` via a path dependency:

```toml
# rmp_driver/Cargo.toml
[dependencies]
bignum = { path = "/Users/eli/InfoScraper/output/repos/rmp" }
```

```rust
// rmp_driver/src/main.rs
use bignum::biguint::BigUInt;

fn main() {
    let z = BigUInt::zero();
    println!("BigUInt::zero().is_zero() = {}", z.is_zero());
}
```

`cargo run` builds rmp as a transitive dep, then runs the driver.
Output: `BigUInt::zero().is_zero() = true`. The driver also runs
three contrast probes, each documented in the evidence appendix:

- `let n = z.num_bits();` → `error[E0624]: method 'num_bits' is
  private`. Witnesses that `pub(crate)` does not reach external
  callers.
- `use bignum::biguint::basic::BigUInt;` → `error[E0603]: module
  'basic' is private`. Witnesses that the `pub use basic::BigUInt;`
  re-export in `mod.rs` is load-bearing — without it, `BigUInt`
  would be unreachable.
- `let _ = z.limbs.len();` → `error[E0616]: field 'limbs' of struct
  'BigUInt' is private`. Witnesses that `pub(super)` on the field
  doesn't reach external callers.

## What Changed

- The rmp arc closes. Sixteen lessons (095–110) compose into reading
  one real-world Rust slice end-to-end: `BigUInt` and the trio
  `zero` / `is_zero` / `num_bits` in `biguint/basic.rs`, plus the
  `LIMB_SIZE_BITS` constant.
- The module-path chain `bignum::biguint::BigUInt` is fully walked.
  Every link in the chain — `pub mod biguint;`, `mod basic;`, `pub
  use basic::BigUInt;` — is composed of installed lessons.
- Every visibility modifier in the slice is explained: plain `pub`
  on the struct and on `zero`/`is_zero`; `pub(super)` on the
  `limbs` field (composed into lesson 103's modifier-on-a-field slot,
  grounded by the Reference grammar); `pub(crate)` on `num_bits` and
  `LIMB_SIZE_BITS`.
- The `num_bits` algorithm is explained step by step, predicted on
  six hand-built inputs, and confirmed by the probe transcript.
- Two follow-on arcs are unlocked: the *trait arc* (so the rest of
  rmp's machinery — `From`, `TryFrom`, `Add`, `Sub`, `Mul`, `Div`,
  `PartialOrd`, `Display`, etc. — becomes readable) and *deeper rmp
  algorithmic methods* (the carry-propagation in `add.rs`, the
  schoolbook multiplication in `mul.rs`, the long division in
  `div.rs`).

## Check Yourself

(a) `BigUInt { limbs: vec![0xFFFF_FFFF_FFFF_FFFF, 0x1] }` represents
what value (in terms of 2^64), and what does `num_bits` return?

(b) Why is `BigUInt::from(0u64).is_zero()` `false`, and which
invariant does the discrepancy point at?

(c) The line `pub use basic::BigUInt;` is dropped from
`biguint/mod.rs`. Which path now compiles in `main.rs`, and which
fails?

*(Answers: (a) Value `(2^64 - 1) + 1 * 2^64 = 2 * 2^64 - 1`. The MSL
is `0x1`, with 63 leading zeros, so MSL term = 64 - 63 = 1. Lower-
limbs term = (2 - 1) * 64 = 64. Total: 65 bits. (b) `From<u64>` in
`convert.rs` builds `BigUInt { limbs: vec![n] }` without trimming
trailing zeros, so `from(0u64)` produces a one-limb-zero `BigUInt`.
`is_zero` is a length-only check (`limbs.len() == 0`), so it returns
`false`. The discrepancy points at the **no-trailing-zero-limbs**
invariant — `is_zero` assumes constructors maintained it; `From<u64>`
on the literal `0` does not. (c) Without the re-export,
`bignum::biguint::BigUInt` fails (no `BigUInt` directly in
`biguint`'s value namespace) and `bignum::biguint::basic::BigUInt`
also fails (E0603, `basic` is private). The slice becomes
unreachable from `lib.rs`'s consumers.)*

## What To Ignore For Now

Today reads only the trio. Same `basic.rs` file holds:

- The `Bones` and `BonesU32` structs and the `limb_to_bones`
  function — internal helpers used by `mul.rs`. Out of scope.
- The `#[cfg(test)] mod tests { ... }` block, the `#[test]`
  attribute, and the `assert_eq!` macro — testing infrastructure.
  The block uses `BigUInt::from(...)` and `<<=`, both deferred.
- `#[derive(Clone)]` on `BigUInt` and `#[derive(Debug)]` on `Bones`
  — derive macros and trait machinery. The trait arc unlocks these.

The whole rest of rmp:

- `bigint.rs` — `BigInt` (signed), `Sign` enum, conversion impls.
- `biguint/cmp.rs` — `PartialOrd`, `Ord`, `PartialEq`, `Eq` impls.
- `biguint/add.rs`, `mul.rs`, `div.rs`, `shift.rs` — the
  algorithmic methods. Each composes deferred trait machinery
  (`Add`, `Sub`, `AddAssign`, `Mul`, `MulAssign`, `Div`, `Rem`,
  `Shl`, `Shr`, `ShlAssign`, `ShrAssign`) with arithmetic-method
  families lesson 108 named (`overflowing_add`, `overflowing_sub`,
  `overflowing_mul`, `wrapping_add`, etc.).
- `biguint/format.rs` — `Display` and `Debug` impls.
- `biguint/convert.rs` — the `From`/`TryFrom` impls including the
  honest `From<u64>` non-canonicalization observed today.
- `unsafe fn`, `MaybeUninit`, manual `set_len` — the heavy
  machinery used elsewhere in rmp but not by the trio.

Mechanic-level deferrals named today:

- `std::mem::size_of::<T>()` — generic associated function with
  turbofish syntax. The `size_of::<u64>() == 8` value is grounded
  by the std doc table; the call shape is its own future move.
- The `Bones` types and `limb_to_bones` function in the same file.
- The `#[cfg(test)]` and `#[test]` attribute machinery.
- `#[derive(Clone)]` (lesson 095 named derives as deferred; today
  honors that).
- Match arm with an identifier pattern `n => { ... }` is *operationally*
  used today; lesson 058's binding-pattern installed the shape on a
  `Result` payload but the same pattern slot accepts an arbitrary
  identifier. Already in 058's deferral. The pattern matcher's full
  syntax (literal patterns, range patterns, struct patterns, etc.)
  remains deferred.
- Auto-deref and method receiver coercions — when `is_zero` writes
  `self.limbs.len()`, `self` is `&BigUInt` and `self.limbs` reads
  the field; the precise ownership and reborrow rules at this site
  are deferred.

## Evidence

See `../evidence/110-capstone-rmp-biguint-trio.md`.
