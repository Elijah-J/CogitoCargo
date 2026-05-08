# Evidence — Lesson 117: `==` on `Vec<T>`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/117-vec-equality.md`
Observation: `experimental/eduratchet2/runs/rust-moves/observations/117-vec-equality.rs`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

The probes below were typed into a fresh scratch directory
(`/tmp/eduratchet2-117/`) and compiled with `rustc <file>`; the
resulting executables were run from the same directory.

## Direct prerequisite — lesson 013

Lesson 013 installed:

- The six comparison operators `==`, `!=`, `<`, `<=`, `>`, `>=` as
  shapes that fit on the right of `let` — including `==` — and the
  fact that each produces a `bool` value. The probe and lesson body
  used `==` only on primitive *integer* operands (`a` and `b` bound
  to literals like `5` and `3`). Today extends `==` to a non-primitive
  operand type, `Vec<u64>`. The mechanism that made `==` work on
  integers (a `PartialEq` impl in std) is the same mechanism that
  makes it work on `Vec<u64>` (a different `PartialEq` impl in std);
  lesson 013 named the `PartialEq` trait under "What To Ignore For
  Now" as a deferred future move. Today is the first lesson that
  exercises `==` on a non-primitive operand.

## Direct prerequisite — lesson 107

Lesson 107 installed:

- The `Vec<T>` shape: `vec![]`, `vec![v1, v2, ...]` for construction,
  `v.len()` for length, `v[i]` for element read. The lesson's "What
  To Ignore For Now" block explicitly named *Equality* `v1 == v2` as
  deferred. Today is exactly that deferred move.

## Direct prerequisite — lesson 080

Lesson 080 installed:

- The twelve integer types `u8/u16/u32/u64/u128/usize` and
  `i8/i16/i32/i64/i128/isize` as one *integer family*. Today's probe
  uses `Vec<u64>`. The fact that `u64` itself has a `PartialEq` impl
  is what satisfies the `T: PartialEq` clause in std's
  `impl<T, U, A1, A2> PartialEq<Vec<U, A2>> for Vec<T, A1>` (where
  `T: PartialEq<U>`). Lesson 080 implicitly grounds this — `u64`
  supports `==` by lesson 013's installation.

## Older supporting lessons

- **Lesson 002, 001** — `fn main` entry point; `rustc demo.rs` then
  `./demo`, silent on success.
- **Lesson 005** — `let a: Vec<u64> = vec![10, 20, 30];`.
- **Lesson 011** — `println!("a == b is {}", a == b)` with one
  positional `{}` slot. The expression substituted into the slot is
  the comparison expression itself.
- **Lesson 019** — the `: TYPE` annotation slot, here `: Vec<u64>`.
- **Lesson 003** — the diagnostic-reading map. Not exercised today
  (no failing probe is required); named for completeness in case the
  learner experiments with mismatched element types.
- **Lesson 114** — installed the trait *generic parameter* shape
  `<RHS>` in a trait declaration. Std's `PartialEq` declaration
  (verbatim below) uses the same shape: `pub trait
  PartialEq<Rhs = Self>`. Today reads the shape *structurally* but
  does not exercise its full machinery (the `= Self` *default type
  parameter* clause is named-deferred).
- **Lesson 116** — installed default method bodies in traits. Std's
  `PartialEq` has one required method (`eq`) and one provided
  (default-bodied) method (`ne`). Today uses only `==`; `!=` is
  named-deferred. The `ne` method's default body is the structural
  reason `!=` exists alongside any `PartialEq` impl, but the formal
  exercise of that mechanic is a future move.

## Probe 1 — working probe (`==` on `Vec<u64>`)

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/117-vec-equality.rs`
is the probe.

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

Compile and run on host:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
a == b is true
a == c is false
a == d is false
$ echo "run-exit=$?"
run-exit=0
```

The single working probe carries both halves of the centered claim
in one program:

- `a == b is true` — same length (3), same elements at each index
  (`10`, `20`, `30`). Pairwise equality holds.
- `a == c is false` — same length (3), but the element at index 2
  differs (`30` vs. `99`). Pairwise equality fails on a single
  mismatched element.
- `a == d is false` — different lengths (3 vs. 2). Pairwise equality
  fails on length alone, before any element comparison is even
  logically required.

No separate failing probe is needed. The lesson's contrastive claim
("same content → true; different content or different length →
false") is fully witnessed by the three `println!` lines above.

## Why this works — std's `PartialEq` impl for `Vec<T>`

Std at `output/docs/rust/std/cmp/trait.PartialEq.md` line 1460 lists
the `Implementors` row that grounds today's lesson verbatim:

> `impl<T, U, A1, A2> PartialEq<Vec<U, A2>> for Vec<T, A1>
>   where A1: Allocator, A2: Allocator, T: PartialEq<U>,`

Read structurally:

- `impl PartialEq for Vec<T>` — std implements the `PartialEq` trait
  for `Vec`. This is exactly the `impl Trait for Type` shape lesson
  111 installed.
- `<T, U, A1, A2>` — generic parameters. `T` is the element type of
  the left-hand `Vec`, `U` of the right-hand. The probe uses
  `T = U = u64`. The `A1, A2` allocator parameters are explicitly
  deferred today.
- `where T: PartialEq<U>` — a *trait bound* clause: the
  implementation only applies when `T` itself supports `==` against
  `U`. `u64` supports `==` against `u64` from lesson 013, so the
  probe satisfies the bound. The trait-bounds *syntax* (`where`
  clauses, the colon-after-type-parameter shape `T: Trait`) is a
  separate machine deferred today (the trait-bounds arc, blocked
  since 114 named it).

The Reference at
`output/docs/rust/reference/expressions/operator-expr.md` lines
508-516 grounds the desugar verbatim:

```rust
let a = 1;
let b = 1;
a == b;
// is equivalent to
::std::cmp::PartialEq::eq(&a, &b);
```

That is, the `==` operator on user code is sugar for a call to the
`PartialEq::eq` method. On `Vec<u64>` operands the method dispatched
to is std's `Vec` impl above; its body (defined in std's source) is
what implements the pairwise-equality semantics.

## rmp unlock — `self.limbs == other.limbs`

Source `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`
line 6 verbatim:

```rust
fn eq(&self, other: &BigUInt) -> bool {
    self.limbs == other.limbs
}
```

`BigUInt` is a struct whose `limbs` field has type `Vec<u64>`
(installed by lesson 107's reading of `src/biguint/basic.rs`). The
expression `self.limbs == other.limbs` is exactly today's mechanic:
the `==` operator applied to two `Vec<u64>` values, returning a
`bool`. Lessons 113 and 114 made the surrounding `fn eq(&self,
other: &BigUInt) -> bool` signature readable; today makes the
body's single expression readable.

After today, the only deferred piece of `cmp.rs:4-6` is the surface
`impl PartialEq<BigUInt> for BigUInt` line itself — specifically the
`<BigUInt>` generic argument substituting for std's `<Rhs = Self>`
default type parameter. That is a thin remaining mechanic; lesson
118's PartialEq + Eq capstone composes it with today's machinery.

## Verbatim corpus quotes

### std `output/docs/rust/std/cmp/trait.PartialEq.md`

Lines 7-19 — the trait declaration:

> ```
> pub trait PartialEq<Rhs = Self>
>
> where
>     Rhs: ?Sized,
>
> {
>     // Required method
>     fn eq(&self, other: &Rhs) -> bool;
>
>     // Provided method
>     fn ne(&self, other: &Rhs) -> bool { ... }
> }
> ```

Lines 23-29:

> Trait for comparisons using the equality operator.
>
> Implementing this trait for types provides the `==` and `!=`
> operators for those types.
>
> `x.eq(y)` can also be written `x == y`, and `x.ne(y)` can be
> written `x != y`.

Line 1460 — the impl that today's probe exercises:

> `impl<T, U, A1, A2> PartialEq<Vec<U, A2>> for Vec<T, A1>
>   where A1: Allocator, A2: Allocator, T: PartialEq<U>,`

The `T = U` case (both element types match) is what the probe uses
with `T = U = u64`.

### Reference `output/docs/rust/reference/expressions/operator-expr.md`

Lines 494-516 — the desugar:

> Comparison operators are also defined both for primitive types
> and many types in the standard library.

> Unlike the arithmetic and logical operators above, these operators
> implicitly take shared borrows of their operands, evaluating them
> in [place expression context](...):
>
> ```rust
> let a = 1;
> let b = 1;
> a == b;
> // is equivalent to
> ::std::cmp::PartialEq::eq(&a, &b);
> ```

Lines 523-526 — the table row that names the desugaring target for
`==`:

> | `==` | Equal | `std::cmp::PartialEq::eq` |
> | `!=` | Not equal | `std::cmp::PartialEq::ne` |

### Vec doc `output/docs/rust/std/vec/struct.Vec.md`

Line 2523 — confirms `Vec` carries methods that depend on the
`PartialEq` impl by listing it as a where-bound:

> `impl<T, A> Vec<T, A>
>   where T: PartialEq, A: Allocator,`

(That impl block contains `dedup` and friends. Standard listing
form; the load-bearing fact is that std treats `Vec<T> where T:
PartialEq` as an established shape.)

### rmp `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

Line 6 — the unlock target:

> `self.limbs == other.limbs`

## Claim-to-evidence map

- "`==` works on two `Vec<T>` values where `T: PartialEq`" — std
  `trait.PartialEq.md` line 1460 (the impl); Probe 1 transcript
  (compiles and prints).
- "The result is a `bool`" — std `trait.PartialEq.md` lines 13-14
  (`fn eq(&self, other: &Rhs) -> bool;`); Probe 1's `println!`
  formats the result with `{}`, which renders the bare word
  `true`/`false` (lesson 012's default rendering).
- "Pairwise — same length AND same element values at each index;
  different length returns `false`" — Probe 1 transcript (`a == c`
  false on element mismatch with same length; `a == d` false on
  length mismatch).
- "`==` desugars to `PartialEq::eq`" — Reference lines 508-516
  verbatim quote; Reference lines 525 (table row).
- "Std implements `PartialEq` for `Vec<T>` whenever `T: PartialEq`"
  — std `trait.PartialEq.md` line 1460 verbatim impl line.
- "`u64: PartialEq` is satisfied via lessons 013 + 080" — lesson
  013 (load-bearing) installed `==` on integer values; lesson 080
  (cited) named `u64` as a member of the integer family.
- "`self.limbs == other.limbs` in rmp `cmp.rs:6` is exactly today's
  mechanic" — rmp `cmp.rs` line 6 verbatim; lesson 107 (cited)
  established `BigUInt::limbs` as a `Vec<u64>` field.

## Negative / contrast probe coverage

The lesson's centered contrastive claim is "with equal contents `==`
returns `true`; with different element values OR different length it
returns `false`." Both halves are witnessed inside Probe 1:

- `a == b is true` — equal contents, length 3 = 3.
- `a == c is false` — different element at index 2, length 3 = 3.
- `a == d is false` — different length (3 vs. 2).

No separate failing probe is needed. A *type-mismatch* probe
(`Vec<u64> == Vec<u32>`) was tried during preparation and fires
`error[E0277]: can't compare u64 with u32` plus `the trait
PartialEq<u32> is not implemented for u64`. The diagnostic is
sharper than the lesson's audience needs and exposes the trait-
bounds machine the lesson defers; it is documented here for the
record but is not committed as a probe and is named-deferred under
"What To Ignore For Now."
