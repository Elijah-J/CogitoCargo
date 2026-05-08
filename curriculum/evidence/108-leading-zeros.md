# Evidence — 108-leading-zeros

This appendix grounds lesson 108's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run from a `mktemp -d` directory on this host. Same
  toolchain as recent accepted lessons (107 etc.).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/108-leading-zeros.rs`
is the working probe verbatim.

## Sources

### `output/docs/rust/std/primitive.u64.md`

The std doc page for the `u64` primitive type.

#### Lines 113-131 — the `leading_zeros` signature, gloss, examples

> 1.0.0 (const: 1.32.0) ·
>
> #### pub const fn [leading_zeros](#method.leading_zeros)(self) -> [u32](primitive.u32.md)
>
> Returns the number of leading zeros in the binary representation of `self`.
>
> ...
>
> ##### Examples
>
> ```
> let n = u64::MAX >> 2;
> assert_eq!(n.leading_zeros(), 2);
>
> let zero = 0u64;
> assert_eq!(zero.leading_zeros(), 64);
>
> let max = u64::MAX;
> assert_eq!(max.leading_zeros(), 0);
> ```

Direct corpus warrant for:

- the signature `pub const fn leading_zeros(self) -> u32` quoted in
  the lesson body verbatim;
- the gloss "Returns the number of leading zeros in the binary
  representation of `self`" quoted in the lesson body verbatim;
- the two edge cases `0u64.leading_zeros() == 64` and
  `u64::MAX.leading_zeros() == 0` cited in the lesson and
  reproduced by Probe 1.

### `output/docs/rust/std/primitive.u8.md`, `primitive.u32.md`, `primitive.i64.md`

Each integer-primitive doc page carries an analogous `leading_zeros`
method with the same signature shape `pub const fn leading_zeros(self)
-> u32`. Verified by:

- `primitive.u8.md` lines 113-130 — `leading_zeros(self) -> u32`,
  examples `n.leading_zeros() == 2`, `zero.leading_zeros() == 8`,
  `max.leading_zeros() == 0`.
- `primitive.u32.md` lines 113-130 — `leading_zeros(self) -> u32`,
  examples `n.leading_zeros() == 2`, `zero.leading_zeros() == 32`,
  `max.leading_zeros() == 0`.
- `primitive.i64.md` lines 82-94 — `leading_zeros(self) -> u32`,
  example `n.leading_zeros() == 0`.

Corpus warrant for the lesson's claim "the same method name lives on
every integer width with the same `u32` return."

#### Negative coverage

`grep -n "leading_zeros" output/docs/rust/std/primitive.f32.md
primitive.f64.md primitive.bool.md` returns no matches. Corpus
warrant for "Not on `f32`/`f64` or `bool`." Operationally
confirmed by Probe 2 (`f64`) and Probe 4 (`bool`).

### `output/docs/rust/error_codes/E0599.md`

The error-code reference page.

> This error occurs when a method is used on a type which doesn't
> implement it:

Direct corpus warrant for E0599's meaning, which today's contrast
probe reports verbatim.

## Direct prerequisite summaries

Each direct prerequisite's specific claim today reuses, in 1-3
bullets per Audit Trail Depth.

### Lesson 040 (load-bearing — `value.method()` dot-call shape)

- Lesson 040 installed the receiver-then-dot-then-method-name form
  for calling methods on values, e.g. `n.abs()` for an `i32`.
- Today applies the exact same shape to `n.leading_zeros()` for a
  `u64` receiver. No new syntactic rule. The contrast probe writes
  `1.0f64.leading_zeros()` — same shape, different receiver type —
  to witness that the syntactic shape is independent of receiver
  type but the method *resolution* depends on receiver type.

### Lesson 080 (load-bearing — integer type family)

- Lesson 080 installed twelve typed integer names organized by
  sign and width, plus the rule that bit width fixes the range.
  `u64` is one of those twelve names.
- Today uses `u64` as the worked receiver type. The lesson body's
  claim "the same method exists on every integer width" extends
  lesson 080's family-naming with a per-width method. Warranted by
  the per-primitive doc cross-check above.
- Lesson 080 named the `MIN`/`MAX` associated constants on each
  primitive integer type as deferred. Today uses `u64::MAX` as an
  opaque constant — its value is not load-bearing, only its
  membership in `u64` and the std-doc edge case
  `u64::MAX.leading_zeros() == 0`.

### Lesson 102 (cited — `self` as the consuming receiver shape)

- Lesson 102 installed `fn method(self)` (no `&`, no `mut`) as the
  third receiver shape. For non-`Copy` types the receiver gets
  moved. Lesson 102's *What To Ignore* named "`Copy` types vs move
  types" as deferred and noted that primitive integers like `u32`
  are `Copy`.
- Today's method has signature `(self)`. `u64` is `Copy` (one of
  lesson 080's family — the canonical Copy set), so the move rule
  is invisible at the call site. The lesson body cites lesson 102
  to acknowledge the receiver-shape line and notes the `Copy`
  surface effect. No new claim about `Copy` is made today.

### Lesson 081 (cited — integer literal forms)

- Lesson 081 installed `0xff`, `0o77`, `0b1111_0000` as alternative
  notations for integer values, plus the type-suffix form like
  `57u8` and the `_` separator.
- Today uses `0u64`, `1u64` (typed-decimal forms), `0x100000000u64`
  (typed-hex form). All three are admitted by lesson 081's table.

### Lesson 062 (cited — `u32` as a type name)

- Lesson 062 installed `u32` as the unsigned 32-bit integer type
  with range `0..=4_294_967_295`.
- Today's method returns `u32`. The lesson notes the return type
  to install the rule "`n.leading_zeros()` is *not* a `u64`, even
  when `n: u64`." Probe 3 below witnesses this via E0308 if needed
  (auxiliary, not centered).

## Probes

### Probe 1 — working: four `u64` values

Source (committed at `observations/108-leading-zeros.rs`, comments
elided):

```rust
fn main() {
    println!("0u64 leading zeros = {}", 0u64.leading_zeros());
    println!("1u64 leading zeros = {}", 1u64.leading_zeros());
    println!("0x100000000u64 leading zeros = {}", 0x100000000u64.leading_zeros());
    println!("u64::MAX leading zeros = {}", u64::MAX.leading_zeros());
}
```

Transcript:

```text
$ rustc demo.rs
$ ./demo
0u64 leading zeros = 64
1u64 leading zeros = 63
0x100000000u64 leading zeros = 31
u64::MAX leading zeros = 0
$ echo $?
0
```

`rustc demo.rs` exits `0` and is silent — no warnings, no errors.
`./demo` prints four lines and exits `0`. Each line witnesses one
edge or interior case:

- `0u64`: all bits zero, the std doc edge case `64`.
- `1u64`: only bit `0` set, so `63` zeros above.
- `0x100000000u64` = `2^32`: bit `32` is the only set bit, so `31`
  zeros above (bits `33..=63`).
- `u64::MAX`: all bits one, the std doc edge case `0`.

The probe directly grounds the lesson body's "Returns" output table.

### Probe 2 — contrast: `leading_zeros` on `f64` fires E0599

Source (`broken.rs`):

```rust
fn main() {
    let x = 1.0f64.leading_zeros();
    println!("x = {}", x);
}
```

Compile transcript:

```text
$ rustc broken.rs
error[E0599]: no method named `leading_zeros` found for type `f64` in the current scope
 --> broken.rs:2:20
  |
2 |     let x = 1.0f64.leading_zeros();
  |                    ^^^^^^^^^^^^^ method not found in `f64`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
$ echo $?
1
```

This is the centered contrast: same dot shape, same method name,
the only change is the receiver type from `u64` to `f64`. The
diagnostic's headline names today's rule directly — the method is
on integer types, not floats. Reproduced verbatim in the lesson
body. Wrapper trailer `For more information about this error, try
\`rustc --explain E0599\`.` is lesson 003's `--explain` shape and
matches lesson 100's E0599 contrast (where the same E-code fired
for a different cause: associated function vs method). Same E-code,
different inline reason.

### Probe 3 — auxiliary: return type is `u32`, not `u64`

Source (`aux.rs`):

```rust
fn main() {
    let n: u64 = 1u64.leading_zeros();
    println!("n = {}", n);
}
```

Compile transcript:

```text
$ rustc aux.rs
error[E0308]: mismatched types
 --> aux.rs:2:18
  |
2 |     let n: u64 = 1u64.leading_zeros();
  |            ---   ^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `u32`
  |            |
  |            expected due to this
  |
help: you can convert a `u32` to a `u64`
  |
2 |     let n: u64 = 1u64.leading_zeros().into();
  |                                      +++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
$ echo $?
1
```

Witnesses the return-type claim: `1u64.leading_zeros()` produces a
`u32`, not a `u64`. The lesson body does not include this transcript
(the centered contrast is the f64 case), but cites the typing rule
in the *Two typing details* paragraph. The `help:` block also names
the natural fix — `.into()` — which is its own future move and not
load-bearing today.

The auxiliary E0308 also lands the lesson body's `as u64` reference
at the end. The rmp source uses `self.limbs[n - 1].leading_zeros()
as u64` because the surrounding arithmetic is on `u64`; without the
cast the same E0308 would fire.

### Probe 4 — Check Yourself (b) and (c)

Source for Check Yourself (b) (`tiny.rs`):

```rust
fn main() {
    println!("a = {}", 2u64.leading_zeros());
    println!("b = {}", 4u64.leading_zeros());
    println!("c = {}", 0xFu64.leading_zeros());
}
```

Transcript:

```text
$ rustc tiny.rs
$ ./tiny
a = 62
b = 61
c = 60
$ echo $?
0
```

Witnesses the answers in Check Yourself: `2u64` is `0b10` (bit 1
set, 62 leading zeros); `4u64` is `0b100` (bit 2 set, 61 leading
zeros); `0xFu64` is `0b1111` (bits 0-3 set, highest is bit 3, 60
leading zeros).

Source for Check Yourself (c) — `tiny.rs` plus a fourth line
`println!("d = {}", true.leading_zeros());`:

```text
$ rustc tiny_with_bool.rs
error[E0599]: no method named `leading_zeros` found for type `bool` in the current scope
 --> tiny_with_bool.rs:5:29
  |
5 |     println!("d = {}", true.leading_zeros());
  |                             ^^^^^^^^^^^^^ method not found in `bool`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
$ echo $?
1
```

Witnesses Check Yourself (c)'s answer: same E0599 family as Probe 2,
inline label `method not found in \`bool\`` — same shape as
`f64`'s, with `bool` substituted. Confirms the negative-coverage
claim that `leading_zeros` is on integers only.

## Claim-to-evidence map

| Lesson claim | Source |
|---|---|
| `pub const fn leading_zeros(self) -> u32` and the gloss | std `primitive.u64.md` lines 113-115 (verbatim) |
| `0u64.leading_zeros() == 64`, `u64::MAX.leading_zeros() == 0` | std `primitive.u64.md` lines 122-131; Probe 1 |
| `1u64.leading_zeros() == 63` | Probe 1 (interior case not in std edge examples) |
| `0x100000000u64.leading_zeros() == 31` | Probe 1 (interior case) |
| The receiver shape is `self` (no `&`) | std signature above; lesson 102 (load-bearing for `self` shape) |
| `u64` is `Copy`, so the call does not consume the value | Lesson 080's family is the canonical Copy set; lesson 102's *What To Ignore* names this; surface confirmed by Probe 1 reusing `0u64` (a literal, but the rule applies symmetrically to bound `u64` values) |
| Return type is `u32`, not `u64` | std signature; Probe 3 (E0308) |
| The same method exists on every integer width | std `primitive.u8.md`/`u32.md`/`i64.md` cross-check above |
| The method is *not* on `f32`/`f64` or `bool` | grep -n "leading_zeros" returns no matches in `primitive.f32.md`, `primitive.f64.md`, `primitive.bool.md`; Probes 2 and 4 |
| E0599 fires with inline label `method not found in <type>` | Probes 2 and 4; std `error_codes/E0599.md` |
| `0u64`, `1u64`, `0x100000000u64` are admitted literal forms | Lesson 081 (load-bearing for integer literal forms) |
| `u64::MAX` is the largest `u64` value | std `u64/constant.MAX.md` lines 1-14 |
| `as u64` cast in rmp converts `u32` -> `u64` | Lesson 034 (load-bearing for `as` cast); the rmp target audit at `runs/rust-moves/rmp-target-audit.md` line 68 names `num_bits` reading `limbs[n-1].leading_zeros()` |

## Notes

- The probe directories `/tmp/lesson108.*/` etc. are ephemeral and
  not committed; only `observations/108-leading-zeros.rs` is.
- Why `u64` and not `u8`/`u32`? `u64` matches rmp's `BigUInt::limbs:
  Vec<u64>` (lesson 107's element type) and lesson 080's family. Any
  unsigned integer type would witness the same rule — Probe 4's
  `0xFu64` is one example; the per-primitive doc cross-check above
  confirms.
- The hex literal `0x100000000u64` was chosen deliberately. The
  orchestrator's draft used `(1u64 << 32)`, which depends on the
  shift operator `<<` — lesson 013 named `<<` as deferred and lesson
  023 confirmed it has not been installed. The hex literal
  `0x100000000` (lesson 081's hex form, value `2^32`) is the same
  value with no new operator.
- The lesson body cites lesson 100's E0599 in *Try It* because both
  lessons hit the same E-code from different angles. Lesson 100's
  case was "associated function called as method" (signature has no
  `self`); today's case is "method does not exist on this type at
  all." Both are admitted under E0599's umbrella per the std error
  code page.
