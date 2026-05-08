---
id: 108-leading-zeros
status: accepted
evidence: ../evidence/108-leading-zeros.md
---

# Count leading zero bits with `n.leading_zeros()`

## The Move

Lesson 080 named `u64` as an integer type; lesson 040 installed
`value.method(args)`. Today combines the two: integer types carry
*inherent methods*. The first one we install is `leading_zeros`,
which counts how many leading zero bits a value has in its binary
representation.

```rust
fn main() {
    println!("0u64 leading zeros = {}", 0u64.leading_zeros());
    println!("1u64 leading zeros = {}", 1u64.leading_zeros());
    println!("0x100000000u64 leading zeros = {}", 0x100000000u64.leading_zeros());
    println!("u64::MAX leading zeros = {}", u64::MAX.leading_zeros());
}
```

`./demo` prints:

```text
0u64 leading zeros = 64
1u64 leading zeros = 63
0x100000000u64 leading zeros = 31
u64::MAX leading zeros = 0
```

The std `primitive.u64` page gives the signature verbatim: `pub
const fn leading_zeros(self) -> u32`, gloss "Returns the number of
leading zeros in the binary representation of `self`." The docs
spell out two edge cases — `0u64` returns `64`, `u64::MAX` returns
`0`.

Two typing details. The signature uses receiver `self` (no `&`) —
lesson 102's *consuming* shape. But `u64` is `Copy`, so the call
does not actually consume the value. The return type is `u32`
(lesson 062), not `u64`; the std library uses `u32` for every
integer width's bit-counting methods.

The literals: `0u64`, `1u64`, `0x100000000u64` are lesson 081's
typed-decimal and typed-hex forms. `0x100000000` is `2^32` — bit
`32` set, bits `33`-`63` zero — so 31 leading zeros. `u64::MAX` is
sixty-four `1` bits, so zero leading zeros.

## Mental Model Delta

- *Before*: "Lesson 080 told me `u64` is one of twelve integer
  types. I can plug it into `: TYPE` and arithmetic operators, but
  I do not know what dot-form methods exist on a `u64` value."
- *After*: "Integer types carry inherent methods, called via the
  dot shape from lesson 040. `n.leading_zeros()` returns the number
  of leading zero bits in `n`'s binary representation as a `u32`.
  The method is on every `uN` and `iN`, not on `f32`/`f64` or
  `bool`."

## Prerequisites

- Installed concepts:
  - Lesson 040 (*load-bearing*): `value.method()` dot-call shape.
    Today applies it to `.leading_zeros()` on a `u64` receiver.
  - Lesson 080 (*load-bearing*): the integer type family. Today
    uses `u64` as the worked receiver type and names that the same
    method exists on every integer width. Lesson 080 also named
    `u64::MAX` as deferred; today uses it as an opaque constant.
  - Lesson 062 (cited): `u32` as the return type of today's method.
  - Lesson 081 (cited): integer literal forms — `0u64`, `1u64`,
    `0x100000000u64` are all from 081's table.
  - Lesson 102 (cited): `self` as the consuming receiver shape;
    invisible at the call site here because `u64` is `Copy`.
  - Lesson 003 (cited): the four-part diagnostic map applied to
    E0599.
  - Lessons 001, 002, 005, 011 (cited): `rustc demo.rs && ./demo`,
    `fn main`, `let`, `println!` `{}`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
0u64 leading zeros = 64
1u64 leading zeros = 63
0x100000000u64 leading zeros = 31
u64::MAX leading zeros = 0
```

*Now the contrast.* Same dot-call shape, same method name, but on a
type that does not have it. Save as `broken.rs`:

```rust
fn main() {
    let x = 1.0f64.leading_zeros();
    println!("x = {}", x);
}
```

Compile:

```text
error[E0599]: no method named `leading_zeros` found for type `f64` in the current scope
 --> broken.rs:2:20
  |
2 |     let x = 1.0f64.leading_zeros();
  |                    ^^^^^^^^^^^^^ method not found in `f64`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
```

Read with the lesson 003 map. Headline E0599 (lesson 100's E-code
for "no method found"). Caret under the method name. Inline label
`method not found in \`f64\`` names today's rule directly:
`leading_zeros` is on integer types, not floats. Same dot shape,
same method name — only the receiver type differs.

## What Changed

- Integer types carry **inherent methods**. `n.leading_zeros()` is
  the first one we install — dot shape from lesson 040, `u64`
  receiver.
- Returns the count of leading zero bits as a `u32`. For `u64`:
  `0u64` -> `64`, `1u64` -> `63`, `u64::MAX` -> `0`.
- The signature uses receiver `self` (no `&`), but `u64` is `Copy`
  so the call does not consume the value.
- Same method name on every integer width (`u8`, `i32`, `u128`,
  ...), same `u32` return. Not on `f32`/`f64` or `bool` — those
  fire E0599 verbatim.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    println!("a = {}", 2u64.leading_zeros());
    println!("b = {}", 4u64.leading_zeros());
    println!("c = {}", 0xFu64.leading_zeros());
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile silently?

(b) What three lines does `./tiny` print?

(c) You add a fourth line `println!("d = {}", true.leading_zeros());`
and rerun. What E-code fires, and what does the inline label under
the caret say?

*(Answers: (a) Yes — silent compile, exit 0. (b) `a = 62`, `b = 61`,
`c = 60`. (c) E0599; label `method not found in \`bool\``.)*

## What To Ignore For Now

Today installs only `leading_zeros` on `u64`. Real and deferred:

- *The bit-counting family*: `trailing_zeros`, `count_ones`,
  `count_zeros`, `leading_ones`, `trailing_ones`. Same signature
  shape, same `u32` return. Future move.
- *Wrapping/checked/overflowing/saturating arithmetic*: lesson 083
  named these as deferred families.
- *The bit operators* `<<`, `>>`, `&`, `|`, `^`, `!` — operator
  level, separate from method-call shape. Lesson 013 deferred.
- *The `BITS`, `MAX`, `MIN` associated constants*: today uses
  `u64::MAX` as an opaque constant; the `Type::CONST` shape is its
  own move.
- *`std::mem::size_of::<T>()`*; *`unbounded_shl` / `unbounded_shr`*
  — future moves.

(Today's move unlocks reading `self.limbs[n - 1].leading_zeros() as
u64` in rmp's `BigUInt::num_bits`. The `as u64` cast is lesson
034's `as` shape, converting the `u32` return to `u64`.)

## Evidence

See `../evidence/108-leading-zeros.md`.
