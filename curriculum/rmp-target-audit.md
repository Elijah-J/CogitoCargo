# rmp (`bignum`) Backward Target Audit — v1

## 0. Scope and dating

- Audit captured 2026-05-07 against
  `/Users/eli/InfoScraper/output/repos/rmp/` (origin
  `https://github.com/stormalinblue/rmp`).
- Toolchain on host: `rustc 1.95.0 (59807616e 2026-04-14)`,
  `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`. Same toolchain the run has
  been probing on since lesson 001.
- Goal of this document: drive curriculum planning. After this audit,
  the graph (currently 94 accepted nodes through lesson 094) is
  extended forward toward the smallest set of lessons that lets a
  learner read every line of rmp source, state every representation
  invariant, audit every `unsafe` block, run and interpret the test
  suite and warnings, and identify safe first contribution seams. The
  capstone for this target is an *understanding* capstone, not a
  coding capstone.

## 1. Repo facts

- Package: `bignum`, edition 2024, version 0.1.0, no external
  dependencies (`Cargo.toml` lines 1–7).
- Source: 1410 LOC across 12 `.rs` files (`wc -l` 2026-05-07).
- Layout: a binary crate (`src/main.rs`) and a library crate
  (`src/lib.rs`) coexist in one package. `main.rs` consumes the
  library via `use bignum::bigint::BigInt;` and
  `use bignum::biguint::BigUInt;`.
- `.gitignore`: `/target /scratch /bench`.

## 2. Module / file map

| Path | Role | LOC |
|---|---|---|
| `src/lib.rs` | Crate root: `pub mod bigint; pub mod biguint;` | 2 |
| `src/main.rs` | Demo binary: factorial loop in `main`; unused `fibonacci`/`do_fibonacci` | 41 |
| `src/bigint.rs` | The signed bignum: `Sign`, `Nonzero`, `BigInt` enum, all arithmetic + format impls | 488 |
| `src/biguint/mod.rs` | Module-of-files: declares `add`, `basic`, `cmp`, `convert`, `div`, `format`, `mul`, `shift`; `pub use basic::BigUInt;` | 10 |
| `src/biguint/basic.rs` | `BigUInt` struct, `Bones`/`BonesU32` half-limb structs, `LIMB_SIZE_BITS`, `zero/is_zero/num_bits` | 78 |
| `src/biguint/cmp.rs` | `PartialEq`/`Eq`/`PartialOrd`/`Ord` for `BigUInt` | 33 |
| `src/biguint/convert.rs` | `From<u32>`, `From<u64>`, `TryFrom<&BigUInt> for u64`, `FromStr` (parse from decimal string), `ConvertOutNumError`, `ParseBigUIntError` | 65 |
| `src/biguint/add.rs` | `limb_lshift_add_assign`, `unsafe sub_unchecked`, `unsafe sub_assign_unchecked`, `Add<&BigUInt>`, `AddAssign<u64>`, `AddAssign<&BigUInt>`, `Sub<&BigUInt>` | 270 |
| `src/biguint/mul.rs` | `Mul<u32>`, `MulAssign<u32>`, `Mul<&BigUInt>` | 218 |
| `src/biguint/div.rs` | `lt_u32`, `div_mod` returning `Option<(Self, u32)>`, `Div<u32>` | 70 |
| `src/biguint/shift.rs` | `ShlAssign<u64>` only (no `Shl`, no right-shift) | 51 |
| `src/biguint/format.rs` | `LowerHex`/`Display`/`Debug` for `BigUInt` | 84 |

Module structure observation: `biguint/` is a single conceptual module
spread across eight files, each adding `impl BigUInt` blocks or trait
impls into the same namespace. `pub use basic::BigUInt;` re-exports
the type at `bignum::biguint::BigUInt` so the binary does not write
`bignum::biguint::basic::BigUInt`.

## 3. Type surface and representation invariants

### 3.1 `BigUInt` (`biguint/basic.rs:2`)

```rust
pub struct BigUInt {
    pub(super) limbs: Vec<u64>,
}
```

Invariants (gathered from constructors and `is_zero` / `num_bits` /
`cmp::Ord` consumers):

- **Limb order**: little-endian. `limbs[0]` is the least-significant
  64-bit limb. (`num_bits` reads `limbs[n-1].leading_zeros()` as the
  most-significant limb; `cmp::Ord` zips the *reversed* iterators.)
- **Canonicalization**: trailing-zero limbs are forbidden in the
  canonical form. The empty `Vec` represents `0` exactly. Every
  routine that produces a `BigUInt` is responsible for trimming
  trailing zeros (most do so by tracking `nonzero_size` and finishing
  with `set_len(nonzero_size)` or `result_num_limbs`).
- **Equality**: `PartialEq` is `self.limbs == other.limbs`, which
  implicitly relies on the no-trailing-zero invariant — otherwise
  `vec![0]` and `vec![]` would compare unequal but represent the
  same number.

### 3.2 Half-limb helpers (`biguint/basic.rs:7,22`)

```rust
pub(super) struct Bones { pub(super) upper: u64, pub(super) lower: u64 }
pub(super) struct BonesU32 { pub(super) upper: u32, pub(super) lower: u32 }
```

Invariants:

- For `Bones`: `upper` and `lower` each hold values in `0..(1u64 << 32)`
  — the high and low 32-bit halves of a u64 limb. This is the avoid-u128
  trick used by the multiplication algorithms.
- `BonesU32` is the same split but typed as `u32` halves.
- Constructed only via `limb_to_bones` / `limb_to_bones_u32` so the
  invariant holds by construction.

### 3.3 `Sign`, `Nonzero`, `BigInt` (`bigint.rs:6,26,49`)

```rust
#[derive(Clone, Copy, PartialEq, Eq)] enum Sign { Positive, Negative }
#[derive(Clone, PartialEq, Eq)] pub struct Nonzero { sign: Sign, mantissa: BigUInt }
#[derive(Clone, PartialEq, Eq)] pub enum BigInt { Zero, Nonzero(Nonzero) }
```

Invariants:

- `Sign` is sign-magnitude with no zero variant. Zero is encoded
  exclusively at the `BigInt::Zero` level.
- `Nonzero.mantissa` MUST be a non-zero `BigUInt` (its `limbs` MUST be
  non-empty and canonical). All public BigInt constructors enforce
  this: `BigInt::nonzero_unchecked` / `positive_unchecked` /
  `negative_unchecked` are private; `From<u64>` / `From<i32>` /
  `From<i64>` / `From<BigUInt>` route the `0` case to `BigInt::Zero`.
  Every arithmetic impl that *might* produce zero collapses to
  `BigInt::Zero` explicitly (e.g. `Add` line 170 `Ordering::Equal =>
  BigInt::Zero`).
- Therefore: `BigInt` has exactly one zero representation. Equality
  via the auto-derived `PartialEq` is well-defined.

### 3.4 Error types

- `ConvertOutNumError::WouldOverflow` — single-variant enum returned
  from `TryFrom<&BigUInt> for u64` when more than one limb is set.
- `ParseBigUIntError {}` — empty struct returned from
  `FromStr for BigUInt` on any non-decimal-digit char.

## 4. Public API surface

### 4.1 Re-exports

- `bignum::biguint::BigUInt` (via `pub use basic::BigUInt;`).
- `bignum::biguint::ConvertOutNumError`, `bignum::biguint::ParseBigUIntError`,
  `bignum::biguint::limb_to_bones_u32` (exposed indirectly because
  their parent module is `pub`).
- `bignum::bigint::BigInt`, `bignum::bigint::Nonzero`.

### 4.2 Inherent public methods

- `BigUInt::zero() -> Self` (basic.rs:37)
- `BigUInt::is_zero(&self) -> bool` (basic.rs:41)
- `BigUInt::num_bits(&self) -> u64` (`pub(crate)`, basic.rs:45)
- `BigUInt::div_mod(&self, rhs: u32) -> Option<(Self, u32)>` (div.rs:15)
- `BigUInt::limb_lshift_add_assign(&mut self, lshift_limbs: usize, rhs: &Self)` (`pub(super)`, add.rs:6) — used internally by `Mul<&BigUInt>`
- `unsafe BigUInt::sub_unchecked(&self, rhs: &Self) -> Self` (`pub(crate)`, add.rs:37)
- `unsafe BigUInt::sub_assign_unchecked(&mut self, rhs: &Self)` (`pub(crate)`, add.rs:78)
- `BigInt` has no public inherent methods. Its three private
  constructors `nonzero_unchecked` / `positive_unchecked` /
  `negative_unchecked` are crate-internal; `is_nonnegative` is a
  dead-code-warning private helper.

### 4.3 Free public function

- `biguint::basic::limb_to_bones_u32(a: u64) -> BonesU32` is `pub`
  (basic.rs:28). Likely an authoring oversight given that
  `limb_to_bones`/`Bones` are `pub(super)`; the BonesU32 return type
  itself is `pub(super)`. This is a candidate "first contribution
  seam" — narrow visibility once understood.

## 5. Trait-impl surface

Reading order: same-type families grouped.

### 5.1 BigUInt trait impls

- `Clone` via `#[derive]` (basic.rs:1)
- `PartialEq<BigUInt>` (cmp.rs:4) and `Eq` (cmp.rs:10) — both
  hand-written; PartialEq is a `self.limbs == other.limbs` shortcut.
- `PartialOrd` (cmp.rs:12) delegating to `Ord::cmp`.
- `Ord` (cmp.rs:18) — limb-count-then-most-significant-first lexical.
- `From<u32>` (convert.rs:8) — single-limb construction.
- `From<u64>` (convert.rs:16) — single-limb construction. *Note*:
  `From<0u64>` produces `BigUInt { limbs: vec![0] }` which violates
  the no-trailing-zero canonical form. Audit-grade defect candidate —
  the codebase's invariant is "limbs has no trailing-zero limb"; this
  constructor breaches it for the `0u64` case. Worth flagging in
  capstone; not a live test case in the suite.
- `TryFrom<&BigUInt> for u64` (convert.rs:22) — succeeds iff `limbs.len() <= 1`.
- `FromStr` (convert.rs:37) — parses decimal digits character-by-character.
- `Add<&BigUInt> for &BigUInt` (add.rs:112) — long add with carry; `Output = BigUInt`.
- `AddAssign<u64> for BigUInt` (add.rs:150) — fast path for single-limb add.
- `AddAssign<&BigUInt> for BigUInt` (add.rs:172) — long add into mutable `self`.
- `Sub<&'b BigUInt> for &'a BigUInt` (add.rs:201) — *checked* subtraction returning `Option<BigUInt>`. Explicit lifetime parameters on the impl block; `Output = Option<BigUInt>`.
- `Mul<u32> for &BigUInt` (mul.rs:4) — bones-trick u32 multiplication.
- `MulAssign<u32> for BigUInt` (mul.rs:59) — same algorithm in-place.
- `Mul<&BigUInt> for &BigUInt` (mul.rs:100) — long multiplication via `Mul<u32>` on each half-limb of `rhs`.
- `Div<u32> for &BigUInt` (div.rs:62) — wrapper over `div_mod` keeping only the quotient.
- `ShlAssign<u64> for BigUInt` (shift.rs:5) — left-shift by arbitrary bit count.
- `LowerHex`, `Display`, `Debug` for `BigUInt` (format.rs:4,27,76).

### 5.2 BigInt trait impls

- `Clone, PartialEq, Eq` derived.
- `PartialOrd` (bigint.rs:85) delegating to `Ord::cmp`.
- `Ord` (bigint.rs:91) — Zero/Nonzero × Zero/Nonzero match table.
- `From<BigUInt>` (bigint.rs:108), `From<u64>` (118), `From<i32>` (128), `From<i64>` (138).
- `Add<&BigInt> for &BigInt` (148), `AddAssign<&BigInt>` (184),
  `Sub<&BigInt> for &BigInt` (211), `SubAssign<&BigInt>` (247),
  `Neg for BigInt` (274) — `Neg` consumes `self` by value, all others borrow.
- `Mul<u32> for &BigInt` (286), `Mul<&BigInt> for &BigInt` (301).
- `LowerHex`, `Display`, `Debug` for `BigInt` (319, 331, 343).

### 5.3 Sign / Nonzero trait impls

- `Sign`: `#[derive(Clone, Copy, PartialEq, Eq)]`. Inherent `flipped`
  (returns) and `flip` (mutates).
- `Nonzero`: `#[derive(Clone, PartialEq, Eq)]`. Hand-written
  `PartialOrd`/`Ord` (bigint.rs:31,37) using sign-then-magnitude
  comparison (note: for two negatives the magnitude comparison is
  *reversed*).

### 5.4 Notable absences

- No `Sub` returning `Option<BigInt>` or panicking — `Sub` for `BigInt`
  returns `BigInt` directly because signed arithmetic always closes.
- No `Shr` / `ShrAssign` — only left shift is implemented.
- No `Div<&BigUInt>` — division is u32-only.
- No `Add<u32>` for either type beyond `AddAssign<u64> for BigUInt`.

## 6. Unsafe sites

17 `unsafe` mentions across 5 files. Two distinct groups: `unsafe fn`
declarations and `unsafe` blocks (callsites + self-contained risky
ops).

### 6.1 Unsafe function declarations (caller-obligation contracts)

| Site | Signature | Caller must guarantee |
|---|---|---|
| `add.rs:37` | `pub(crate) unsafe fn sub_unchecked(&self, rhs: &Self) -> Self` | `self >= rhs`. The routine adds `!rhs[i]` plus an initial carry of 1 (two's complement subtraction). If `self < rhs` the most-significant carry chain wraps and the algorithm's `nonzero_size`-driven trim produces a wrong result. |
| `add.rs:78` | `pub(crate) unsafe fn sub_assign_unchecked(&mut self, rhs: &Self)` | Same `self >= rhs` precondition; in-place version. |

Each callsite of these two functions must wrap the call in `unsafe { ... }`.

### 6.2 Unsafe-block callsites (precondition-discharging)

| Site | Discharges precondition via |
|---|---|
| `add.rs:208` | `match self.cmp(other) { Ordering::Greater => Some(unsafe { self.sub_unchecked(other) }), ... }` — only the `Greater` arm calls. |
| `add.rs:263` (test) | The test asserts `b > prev_b` from the Fibonacci recurrence. |
| `bigint.rs:171,174,232,235` | Inside arms `match man_a.cmp(man_b)` for the `Greater` / `Less` branches; operands are swapped on `Less`. |
| `bigint.rs:196,201,259,264` | AddAssign / SubAssign cases — same `cmp` discharge pattern, with operand swap on `Less`. |

### 6.3 Self-contained unsafe ops (require local proof)

| Site | Operation | Local safety argument |
|---|---|---|
| `add.rs:71` | `new_limbs.set_len(nonzero_size)` after pushing one element per index `0..left_limbs.len()` | `nonzero_size <= left_limbs.len() == new_limbs.len()`, all those positions were initialized via `push`. Sound. |
| `add.rs:106` | `self.limbs.set_len(nonzero_size)` after writing `self.limbs[index] = final_val` for `0..self.limbs.len()` | Same shape. Sound. |
| `div.rs:49` | `result_limbs.set_len(result_num_limbs)` after writing into `spare_capacity_mut()[lshift_limbs]` for every `lshift_limbs in 0..self.limbs.len()` where `result_num_limbs > lshift_limbs` | The loop writes positions `0..result_num_limbs` (inclusive boundary); `set_len` to that length is sound. |
| `format.rs:67` | `String::from_utf8_unchecked(characters[chars_required - character_index..].to_vec())` | Each byte was set as `'0' as u8 + (rem as u8)` for `rem in 0..10`, i.e., ASCII `'0'..='9'`. ASCII is valid UTF-8. Sound. |
| `shift.rs:19` | `self.limbs.set_len(new_num_limbs)` *before* writes are complete | Comment marks this as "Perhaps we are stepping into UB / All of these bytes should be written to before we read them." Subsequent `copy_within(0..old_num_limbs, shift_limbs)` and `[0..min(old_num_limbs, shift_limbs)].fill(0)` initialize most positions; in the `old_num_limbs < shift_limbs` case the gap `[old_num_limbs..shift_limbs]` may be left uninitialized before the trailing for-loop reads `self.limbs[index + 1]` at `(shift_limbs + old_num_limbs - 1) + 1`. **This site warrants close audit in the capstone.** |

### 6.4 Capstone obligations

The understanding capstone for rmp must, for each site above, name the
invariant and judge whether the code establishes it. The graph must
install enough `unsafe`/`Vec`/`MaybeUninit`/two's-complement-on-unsigned
vocabulary that this judgement is teachable.

## 7. Test surface

17 tests, all `#[test]` in `mod tests` submodules (`#[cfg(test)]`).

| File | Tests | Notes |
|---|---|---|
| `bigint.rs` | `from_i32_compares_correctly_to_zero`, `from_i64_compares_correctly_to_zero`, `add_eq_corresponds_to_add_fib`, `sub_reverses_fib_step`, `sub_reverses_neg_fib_step`, `sub_assign_reverses_fib_step`, `sub_assign_reverses_neg_fib_step`, `mul_factorial_square` | 8 tests; the last has a doc-comment-style block comment explaining the cross-check pattern. |
| `biguint/basic.rs` | `num_bits_correct`, `display_factorial_correct` | 2 tests; the second is an empty `// TODO` body and counts as ignored-by-omission, not an assertion. |
| `biguint/convert.rs` | `from_str_small` | 1 test. |
| `biguint/add.rs` | `add_eq_corresponds_to_add_fib`, `sub_reverses_fib_step`, `sub_assign_reverses_fib_step` | 3 tests. |
| `biguint/mul.rs` | `mul_factorial_matches`, `mul_assign_factorial_matches`, `mul_factorial_square` | 3 tests; first two compare 100! against a Python-generated 9-limb golden array. |

Patterns reused across tests:

- Fibonacci 0..3000 as a stress generator for arithmetic identities.
- Factorial 100! as a stress generator for multiplication.
- Cross-check between two independent computation paths (e.g.,
  `mul(&BigInt) vs. mul(u32)`).

## 8. Build health (2026-05-07)

- `cargo test --quiet` → 17 passed, 0 failed, 0 ignored. ~0.01s on host.
- 4 warnings:
  1. `unused variable: index` in `biguint/mul.rs:25` (the loop `for (index, limb) in self.limbs.iter().enumerate()` does not use `index`; should be `_index` or replaced with `for limb in self.limbs.iter()`).
  2. `method is_nonnegative is never used` in `bigint.rs:73`.
  3. `function fibonacci is never used` in `main.rs:4`.
  4. `function do_fibonacci is never used` in `main.rs:21`.
- All four are `#[warn(unused)]` (parent group of `unused_variables`
  + `dead_code`); the run already installed the category in lessons
  069 and 094.

## 9. Concepts not yet installed by the graph

The 94-node graph through lesson 094 installs Rust at the level of
Book Chapters 1–3 plus targeted Ch4–Ch5 vocabulary (references,
arrays, tuples, char, const, integer family). Below are the concepts
rmp uses that the graph has *not* yet installed.

### 9a. Repo-specific moves

These bind directly to constructs in rmp source:

- **Struct definition** with named fields (`struct Foo { x: T }`).
  Required to read `BigUInt`, `Bones`, `BonesU32`, `Nonzero`,
  `ParseBigUIntError`.
- **Struct construction expression** (`Foo { x: 1 }`).
- **Field access on a struct** (`foo.x`) — the run has used `s.method()`
  but not `s.field`.
- **Empty struct / unit-like struct** (`pub struct ParseBigUIntError {}`).
- **Tuple-like struct field count = 0**: not used; out of scope.
- **`enum` definition** with both unit variants (`Sign::Positive`)
  and tuple-like payload (`BigInt::Nonzero(Nonzero)`). Required to
  read `Sign`, `BigInt`, `ConvertOutNumError`.
- **`pub` keyword** on items.
- **Visibility paths**: `pub(super)`, `pub(crate)`. (Not strictly
  required to *read* the code, but required to explain it.)
- **`mod foo;` file-based module declaration**.
- **`mod foo { ... }` inline module** (used for the `tests` submodule).
- **`use Path::Item;` with nested braces** — installed in lesson 044
  (`use std::io;`), but rmp uses heavier forms like
  `use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};`. Likely
  reusable without a new lesson.
- **`super::` and `crate::` path prefixes** — heavily used; only
  `std::` and bare crate-prelude paths are in the graph today.
- **`impl Type { ... }` inherent-method block authoring**.
- **`impl Trait for Type { ... }` trait-impl block authoring** —
  blocks the entire impl surface and is downstream of the trait-machinery
  install (Q07).
- **`#[derive(Trait, ...)]` attribute** — downstream of trait
  machinery.
- **Pattern destructure in `match` arms**:
  `Nonzero { sign, mantissa }`, `Bones { upper: u_n, lower: u_c }`.
  Pattern destructure for tuple-bindings is in lesson 073; struct
  patterns and the `..` rest token are not.
- **Receiver forms `self`, `&self`, `&mut self`** — the `&self`
  receiver is implicit in lessons 040/049 but the explicit definition
  form has not been authored.

### 9b. Rust language

Beyond the repo-specific moves above:

- **Trait machinery** (define a trait, name an associated type, name a
  trait method): blocked by Q07 in `deferred-queue.md`. Required for
  every `impl Trait for ...` line.
- **Operator-overload traits** (`Add`, `Sub`, `Mul`, `Div`, `Neg`,
  `Shl`, `ShlAssign`, `AddAssign`, `SubAssign`, `MulAssign`, etc.).
  Downstream of trait machinery + the meaning of `Output =`.
- **Conversion traits**: `From<T>`, `Into<T>`, `TryFrom<T>`,
  `TryInto<T>`, `FromStr`. `parse::<T>()` was installed (lesson 056)
  but the `FromStr` trait it dispatches to was not.
- **Formatting traits**: `Display`, `Debug`, `LowerHex`, and the
  `fmt::Result` / `fmt::Formatter` types. `format!` and `write!`
  macros and the `?` operator on `fmt::Result`.
- **Comparison traits**: `PartialEq`, `Eq`, `PartialOrd`, `Ord`.
  `i32::cmp` was installed (lesson 061) but the `Ord` trait it lives
  on was not.
- **`Clone` and `Copy` traits** and the difference between them.
  `clone()` calls have been incidental.
- **The `?` operator** on `Result<T, E>` and on `fmt::Result`.
- **`Vec<T>` as a parameterized type**: lesson 093 named `Vec<T>` only
  as a prelude member. The graph has not authored `Vec::new`, `push`,
  `len`, indexing, slicing, `iter` / `iter_mut` / `enumerate` / `rev`
  / `zip`, `with_capacity`, `reserve`, `resize`, `set_len`,
  `copy_within`, `fill`, `spare_capacity_mut`, `as_slice`, or the
  `vec![]` macro. Each is an atomic move.
- **Slices `&[T]` / `&mut [T]`** as a type and its relationship to
  `Vec<T>` — implied by `slice.fill`, `&v[a..b]`.
- **`String` advanced**: `with_capacity`, `+=`, `from_utf8_unchecked`
  (unsafe). The `String::new` install was lesson 042; deeper API is
  uncovered.
- **`Vec<u8>` and the bytes ↔ string boundary**.
- **`Option<T>` defining-shape** beyond `Some`/`None` as prelude
  names: lesson 093 named `Option<T>` as a prelude member but the
  graph has not authored `Option::Some`/`None` matching, `unwrap`,
  `?`, or the deliberate distinction from `Result<T, E>`. Several
  rmp public APIs return `Option<...>` (`div_mod`, `Sub<&BigUInt>`,
  `Div<u32>`).
- **`Result::unwrap`** — used in the FromStr test (`"1234".parse::<BigUInt>().unwrap()`).
- **Lifetimes**: explicit annotation `<'a, 'b>` and the
  `&'b BigUInt` / `&'a BigUInt` form on the `Sub` impl. The graph
  installs references but not lifetime syntax.
- **`unsafe { ... }` blocks** and **`unsafe fn`** declaration form.
- **`MaybeUninit<T>`** and `Vec::spare_capacity_mut()` returning
  `&mut [MaybeUninit<T>]`.
- **Two's-complement representation** of unsigned integers (used in
  `sub_unchecked`'s `!right_limbs[i] + 1` carry-trick).
- **Bit operators `<<`, `>>`, `&`, `|`, `^`, `!`** on integers, and
  the difference between `<<` (shift) and `unbounded_shl` (panic-free).
- **u64 inherent methods**: `leading_zeros()`, `overflowing_add()`
  (named in lesson 083 but not authored as a move),
  `unbounded_shl()`, `unbounded_shr()`.
- **Boolean `as` integer cast**: `(carry as u64)` / `(overflow as u64)`.
  The graph has `as` only between numeric primitives (lesson 034) and
  between char and integer (074 implicitly).
- **`std::mem::size_of::<T>()`** — used to compute `LIMB_SIZE_BITS`.
- **`const` items at module scope** with `pub(crate)` visibility —
  lesson 075 covered `const` at function scope only.
- **Turbofish `Vec::<T>::new()`** and `.parse::<T>()` (the latter is
  in lesson 056 informally).
- **`unreachable!()` macro**.
- **Block expression with curly-braces returning a value**
  (`let x = { /* statements */; expr };`). Used in the
  `mul_factorial_square` tests.

### 9c. Rust tooling / testing surface

- **`#[cfg(test)]`** attribute on a module.
- **`#[test]`** attribute on a function.
- **`assert!`, `assert_eq!`, `assert_ne!`** macros and their
  `panic` semantics.
- **`mod tests { use super::*; ... }` test-co-located pattern**.
- **`cargo test`** subcommand — not yet authored; contrast against
  `cargo build` / `cargo run` / `cargo check` already in the graph.

### 9d. Domain / math

- **Positional number representation** in a chosen radix; what a
  "limb" is.
- **Long addition with carry** at the textbook level.
- **Long subtraction via two's complement on unsigned limbs**
  (`a - b ≡ a + (!b) + 1` mod `2^N`).
- **Long multiplication via half-limb decomposition** (Bones).
- **Long division by a small (single-limb) divisor**.
- **Sign-magnitude representation** of signed integers (vs. two's
  complement at the BigInt layer).
- **Decimal-digit count from bit count** via fixed-point log10(2)
  (the `1292913987 / 2^32` constant in `format.rs`).

## 10. Classification

### Ready now

These have all installed prerequisites in the current graph. Pick the
smallest items first.

- `struct` definition with named fields, construction, and field access
  (one tightly-composed move; mental model: "I have used `String` and
  `Vec<i32>` instances; today I write the type myself.")
- `enum` definition with unit variants and with payload variants
  (parallel move to `struct`; downstream of `struct`).
- `pub` keyword as the basic visibility opt-in (smallest atomic).
- `mod foo;` file-based module declaration (smallest atomic; one new
  file is the only setup).
- `mod foo { ... }` inline module (cheap follow-on used for tests).
- `super::` and `crate::` path prefixes.
- `impl Type { fn ... }` inherent-method authoring (downstream of `struct`).
- Receiver forms `self`/`&self`/`&mut self` (downstream of inherent impl).
- Pattern destructure `Foo { x, y }` and the `..` rest token in
  patterns (downstream of `struct`).
- `vec![]` macro (uses lesson 071's macro-invocation syntax).
- `Vec::new()`, `Vec::push`, `Vec::len`, indexing `v[i]`, iteration
  `for x in &v` (each an atomic move; lesson 093 already named the type).
- `&[T]` / `&mut [T]` slice as a type and the `&v[a..b]` borrow form.
- Bit operators `<<`, `>>`, `&`, `|`, `!`, `^` on integers
  (each atomic; could ship as one composed family lesson similar to 080).
- `as u64` from `bool` (atomic, very narrow).
- `u64::leading_zeros()` (atomic; pairs naturally with the integer
  family already installed).
- `u64::overflowing_add()` as a centered move (lesson 083 named the
  family but did not center any single method).
- Block expression `{ stmts; expr }` returning a value (atomic; the
  expression-vs-statement install (024–026) made this trivially
  reachable).
- `#[cfg(test)]` + `#[test]` + `assert_eq!` + `cargo test` as a
  bundled "first test" lesson, OR each as separate atomic moves
  (orchestrator discretion at dispatch).
- `Option<T>` defining-shape (lessons 052/058 covered Result; Option
  is the natural parallel). Several Option-returning rmp APIs are
  blocked on this.
- `Result::unwrap` and `Option::unwrap` (atomic each; cheap follow-on).
- Domain: limb / positional-radix / long-addition-with-carry as a
  conceptual install grounded against rmp source itself (not Rust
  doc). Useful even before authoring the algorithm-level lessons.

### Requires prerequisites

Listed with the missing prerequisite by name.

- **Trait definition** — Q07 in `deferred-queue.md`. Still blocking.
  Until the run installs traits, every `impl Trait for ...` line in
  rmp is unreadable beyond pattern recognition.
- **Operator-overload traits** (`Add`, `Sub`, ..., `ShlAssign`) —
  blocked on trait machinery.
- **Conversion traits** (`From`, `TryFrom`, `FromStr`) — blocked on
  trait machinery.
- **Formatting traits** (`Display`, `Debug`, `LowerHex`, `Formatter`,
  `fmt::Result`) — blocked on trait machinery; also blocked on `?`
  operator install.
- **Comparison traits** (`PartialEq`, `Eq`, `PartialOrd`, `Ord`) —
  blocked on trait machinery.
- **`Clone`, `Copy`** — blocked on trait machinery.
- **`#[derive(...)]`** — blocked on trait machinery.
- **`?` operator** — blocked on a deeper Result-fluency install
  (matches Q07's note that lessons 052/053/058 cover Result
  operationally but not enough to anchor the trait-side machinery).
- **`unsafe { ... }` blocks** — needs the `Vec`/`set_len`/raw-pointer
  vocabulary first; specifically requires `Vec` API depth
  (`with_capacity`, `set_len`, `spare_capacity_mut`) to be ready in
  the graph before an `unsafe` lesson can give a runnable example
  with a meaningful safety argument.
- **`unsafe fn` declaration form** — downstream of `unsafe { ... }`.
- **`MaybeUninit<T>` + `spare_capacity_mut`** — downstream of
  `unsafe`.
- **Lifetimes (`<'a>` syntax, lifetime params on impls)** — needs a
  deeper references / borrow-checker lesson family than the current
  references install (045–048).
- **Two's-complement representation as a Rust-grounded concept** —
  needs `!` (bitwise NOT) on unsigned + `overflowing_add` first; both
  of those are ready-now atomic moves.
- **Half-limb (Bones) multiplication** — needs `<<`, `>>`, `&` on
  integers + the `as u32`/`as u64` casts; all ready-now after the
  bit-ops lesson lands.
- **Long multiplication and long division algorithms** — domain
  installs that need limb / two's-complement / bit-ops to be in
  place first.

### Out of scope

- The four other Rust preludes (`extern`, language, `macro_use`,
  tool) — already classified out of scope in `deferred-queue.md` for
  the broader run. Not load-bearing for rmp.
- SIMD / `#![no_std]` / `#![no_implicit_prelude]` — referenced in
  rmp `// Optimization:` comments but not used.
- Workspaces / `[patch]` / nested packages — not used by rmp.
- `cargo doc --open`, `cargo bench` — not used by rmp; deferred
  earlier in the run.
- Compiler internals (codegen, MIR, borrow-checker mechanics beyond
  user-visible rules) — out of scope for the run as a whole.
- Library-vs-binary lockfile-checkin policy debates — already in the
  out-of-scope list at lesson 092.
- u128 / u256 types — the rmp comments reference `// Optimization: u128`
  but the code does not use it.

## 11. First-move recommendation

Smallest item that unlocks the most subsequent rmp reading: **`struct`
definition with named fields, construction, and field access**.

Why this first:

- Every rmp data type except the two enums is a struct. Until the
  learner can read a `struct` declaration they cannot read `BigUInt`,
  `Bones`, `Nonzero`, `BonesU32`, `ParseBigUIntError`.
- Strict prerequisites already installed: `let` (005), type
  annotation (019), reference forms (045–048 — useful but not
  required for the bare struct lesson), and the audience-known fact
  that `String` and `Vec<i32>` are types-with-data (lessons 042, 093).
- It is genuinely one move: define a struct, build one, read a field.
  The natural deferrals (`pub`, tuple struct, unit struct, struct
  update syntax, `#[derive]`) are clean and named.
- Mental-model delta is sharp: the audience moves from "type names
  refer to library-built things" to "I declare a type."

Natural follow-on chain after struct definition:

1. `struct` definition + construction + field access (recommended first).
2. `pub` keyword on items (atomic, very narrow).
3. `mod foo;` file-based module (atomic; one new file).
4. `enum` definition (parallel to struct; the two together give the
   audience full type-declaration vocabulary).
5. `impl Type { fn ... }` inherent-method authoring (uses lessons
   008/020/021/036's function machinery + struct).
6. Receiver forms `self` / `&self` / `&mut self` (small follow-on;
   may bundle with 5).
7. Struct pattern destructure in `match`/`let` (uses 058/073).
8. `Vec<T>` API depth (atomic moves: `Vec::new`, `push`, `len`, `[i]`,
   `for x in &v`).
9. Bit operators as a family lesson (`<<`, `>>`, `&`, `|`, `!`).
10. `bool as integer` and `u64::leading_zeros` / `u64::overflowing_add`.
11. `Option<T>` defining shape + `unwrap`.
12. `cargo test` + `#[test]` + `#[cfg(test)]` + `assert_eq!`.

Items 1–12 are all classified ready-now and unblock the trait-machinery
arc that begins with Q07. The orchestrator may reorder within the chain
when a stronger dependency argument appears.

The `unsafe`/`MaybeUninit` arc and the trait arc are *not* started
until ready-now items they depend on land. The capstone for rmp is
not attempted until both arcs have closed enough that the learner can
read every line of `add.rs` and `bigint.rs` from source.
