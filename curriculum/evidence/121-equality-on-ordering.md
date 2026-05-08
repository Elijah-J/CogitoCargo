# Evidence — Lesson 121: `==` and `!=` on `Ordering`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/121-equality-on-ordering.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/121-equality-on-ordering.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/121-equality-on-ordering.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

The probes below were typed into a fresh scratch directory
(`/tmp/eduratchet121/`) and compiled with `rustc <file>`; the
resulting executables were run from the same directory.

## Direct prerequisite — lesson 117

Lesson 117 installed:

- `==`/`!=` on `Vec<u64>` via std's `impl PartialEq for Vec<T>`. The
  desugar `a == b ≡ ::std::cmp::PartialEq::eq(&a, &b)` (Reference
  `expressions/operator-expr.md` lines 508-516). The result is `bool`.
- The framing "whoever implemented `PartialEq` for a type makes `==`
  work on that type." Today extends from `Vec<u64>` to `Ordering`;
  the operator, the desugar, the `bool` result, and the framing are
  unchanged. Only the *dispatched impl body* differs — std's
  `Ordering` impl rather than std's `Vec` impl.

## Direct prerequisite — lesson 051

Lesson 051 installed:

- `Ordering` as the standard library's three-variant enum at path
  `std::cmp::Ordering`, with unit variants `Less`, `Greater`, `Equal`.
  The `use std::cmp::Ordering;` line, the variant-construction shape
  `Ordering::Less`, and the `: Ordering` annotation slot all reused
  in today's working probe. Lesson 051's "What To Ignore For Now"
  explicitly named "*The `Ord` and `PartialOrd` traits*" as deferred;
  it did not name `PartialEq` directly, but `Ordering`'s `PartialEq`
  impl is what enables today's move.

## Direct prerequisite — lesson 013

Lesson 013 installed:

- `==` and `!=` between two values of the same kind, producing a
  `bool`. Today extends the operand types to `Ordering`. The result
  type and the operator surface are unchanged. The lesson 013 body
  exercised `==` on integer operands; lesson 117 extended to
  `Vec<u64>`; today extends to `Ordering`.

## Direct prerequisite — lesson 098

Lesson 098 installed:

- The user-side declaration `enum Name { V1, V2 }` for unit-variant
  enums. Today's contrast probe uses exactly this shape:
  `enum Color { Red, Blue }`. Today's lesson does not author a
  `PartialEq` impl on `Color` (deferred); the contrast probe relies
  on the *absence* of any such impl to provoke E0369.

## Older supporting lessons

- **Lessons 002, 001** — `fn main` entry point; `rustc demo.rs` then
  `./demo`, silent on success.
- **Lesson 005** — `let a: Ordering = Ordering::Less;` (three uses).
- **Lesson 011** — `println!("a == b is {}", a == b)` with one
  positional `{}` slot per call. The expression substituted into the
  slot is the comparison expression itself, of type `bool`.
- **Lesson 019** — the `: TYPE` annotation slot, here `: Ordering`.
- **Lesson 044** — the `use std::cmp::Ordering;` line that lets the
  rest of the file write `Ordering` rather than the full path.
- **Lesson 003** — the diagnostic-reading map. Used to walk the
  contrast probe's E0369 transcript (headline, location, `note:`,
  `help:`).
- **Lessons 022, 076, 077** — used only by the corroborating matrix
  probe in this appendix (range-form `for i in 0..3`, array literal
  `[Ordering::Less, ...]`, indexing `xs[i]`). The lesson body does
  not reproduce the matrix; it cites the smaller working probe.
- **Lesson 116** — installed default method bodies. Std's `PartialEq`
  has one required method (`eq`) and one provided (default-bodied)
  method (`ne`). Today exercises both `==` and `!=`; the structural
  reason `!=` works alongside `==` is the `ne` default body, named
  here for the record.

## Probe 1 — working probe (`==`/`!=` on `Ordering`)

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/121-equality-on-ordering.rs`
is the probe.

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

Compile and run on host (full transcript at
`observations/121-equality-on-ordering.transcript.txt`):

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
a == b is true
a == c is false
a != c is true
$ echo "run-exit=$?"
run-exit=0
```

The single working probe carries both halves of the centered claim
in one program:

- `a == b is true` — same variant on both sides (`Less`). Equal.
- `a == c is false` — different variants (`Less` vs `Equal`).
  Unequal.
- `a != c is true` — `!=` returns the negation of `==` on the same
  pair.

## Probe 2 — contrast (E0369 on a fresh enum without `PartialEq`)

```rust
enum Color { Red, Blue }

fn main() {
    let a = Color::Red;
    let b = Color::Red;
    println!("{}", a == b);
}
```

Compile result, captured verbatim:

```text
error[E0369]: binary operation `==` cannot be applied to type `Color`
 --> broken.rs:6:22
  |
6 |     println!("{}", a == b);
  |                    - ^^ - Color
  |                    |
  |                    Color
  |
note: an implementation of `PartialEq` might be missing for `Color`
 --> broken.rs:1:1
  |
1 | enum Color { Red, Blue }
  | ^^^^^^^^^^ must implement `PartialEq`
help: consider annotating `Color` with `#[derive(PartialEq)]`
  |
1 + #[derive(PartialEq)]
2 | enum Color { Red, Blue }
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
exit=1
```

The diagnostic states the centered fact today installs verbatim:
`==` cannot be applied to a type that has no `PartialEq` impl.
`error_codes/E0369.md` line 4: "A binary operation was attempted on
a type which doesn't support it." The `note:` and `help:` blocks
name `PartialEq` and `#[derive(PartialEq)]` explicitly — the lesson
body cites the headline and the `note:` only; the `help:`'s
`#[derive(...)]` mechanic is named-deferred.

## Probe 3 — corroborating matrix (full 3x3 over the variant space)

```rust
use std::cmp::Ordering;

fn main() {
    let xs = [Ordering::Less, Ordering::Equal, Ordering::Greater];
    let names = ["Less", "Equal", "Greater"];
    for i in 0..3 {
        for j in 0..3 {
            println!("{} == {} is {}", names[i], names[j], xs[i] == xs[j]);
        }
    }
}
```

Compile-exit 0, run-exit 0. Output:

```text
Less == Less is true
Less == Equal is false
Less == Greater is false
Equal == Less is false
Equal == Equal is true
Equal == Greater is false
Greater == Less is false
Greater == Equal is false
Greater == Greater is true
```

The diagonal is uniformly `true` (same variant); every off-diagonal
cell is `false` (different variants). This corroborates the working
probe's three-line witness across the entire 3-variant space — nine
pairs total, three same-variant and six different-variant.

The probe additionally exercises lesson 022's `for i in 0..3`
range-form, lesson 076's array literal `[Ordering::Less,
Ordering::Equal, Ordering::Greater]`, and lesson 077's indexing
`xs[i]`. The `xs[i] == xs[j]` expression reads two `Ordering` values
out of the array and compares them; this compiles cleanly because
std implements `Copy for Ordering` (Ordering page line 381), so
indexing-read does not move out of the array. `Copy` on `Ordering`
is *not* centered today — it is named-deferred — but the
corroborating probe surfaces it implicitly. Today's centered move
on the working probe (Probe 1) does *not* depend on Copy: the three
bindings `a`, `b`, `c` are each compared once and never moved, so
the probe compiles even before the audience knows what Copy is.

## Why this works — std's `PartialEq` impl for `Ordering`

Std at `output/docs/rust/std/cmp/enum.Ordering.md` lines 325-340
verbatim:

```text
1.0.0 (const: unstable) · §

### impl PartialEq for Ordering

§

#### fn eq(&self, other: &Ordering) -> bool

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · §

#### fn ne(&self, other: &Rhs) -> bool

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
```

Read structurally:

- `impl PartialEq for Ordering` (line 327) — std implements the
  `PartialEq` trait for `Ordering`. Same `impl Trait for Type` shape
  lesson 111 installed and lesson 117 cited for `Vec<T>`.
- `fn eq(&self, other: &Ordering) -> bool` (line 331) — the body
  that `a == b` desugars to (when `a, b: Ordering`).
- `fn ne(&self, other: &Rhs) -> bool` (line 337) — the inherited
  default-body method that `a != b` desugars to. The default body's
  source is at `trait.PartialEq.md` line 17 (declared as
  `// Provided method`); the line "The default implementation is
  almost always sufficient" (line 339) confirms `Ordering` does not
  override it.

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
`PartialEq::eq` method. On `Ordering` operands the method dispatched
to is std's `Ordering` impl above; its body (defined in std's source)
is what implements the same-variant-equality semantics. The same
section's table at lines 525-526 maps:

```text
| `==` | Equal     | `std::cmp::PartialEq::eq` |
| `!=` | Not equal | `std::cmp::PartialEq::ne` |
```

`trait.PartialEq.md` lines 25-28 verbatim restate the contract:

> Implementing this trait for types provides the `==` and `!=`
> operators for those types.
>
> `x.eq(y)` can also be written `x == y`, and `x.ne(y)` can be
> written `x != y`.

Same span lesson 117 cited. Today reuses it for `Ordering`.

## rmp unlock — `if ord == cmp::Ordering::Equal`

Source `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`
line 21 verbatim (within the body of an `Ord::cmp` impl that today
does not yet read):

```text
if ord == cmp::Ordering::Equal { ... }
```

`ord` has type `Ordering`; the right-hand side is the variant
constructor `cmp::Ordering::Equal`. The `==` operator applies to two
`Ordering` values and returns a `bool` — exactly today's mechanic.
The surrounding iterator-chain that produces `ord` and the `if`-cell
of an `else` branch are deferred; today reads only the equality
comparison.

## Verbatim corpus quotes

### std `output/docs/rust/std/cmp/enum.Ordering.md`

Lines 325-340 — the `impl PartialEq for Ordering` block (full quote
in the "Why this works" section above).

Line 381 — confirms std also implements Copy:

> `### impl Copy for Ordering`

Line 385 — confirms std also implements Eq:

> `### impl Eq for Ordering`

Line 344 — confirms std also implements PartialOrd (deferred):

> `### impl PartialOrd for Ordering`

Lines 354, 358, 365, 372 — `lt`, `le`, `gt`, `ge` are listed as
methods of `PartialOrd for Ordering`; the operators `<`, `<=`, `>`,
`>=` are wired through PartialOrd, deferred today.

### std `output/docs/rust/std/cmp/trait.PartialEq.md`

Lines 6-19 — the trait declaration (same span lesson 117 cited):

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

Lines 23-28 — the contract that "implementing this trait provides
the `==` and `!=` operators":

> Trait for comparisons using the equality operator.
>
> Implementing this trait for types provides the `==` and `!=`
> operators for those types.
>
> `x.eq(y)` can also be written `x == y`, and `x.ne(y)` can be
> written `x != y`.

### Reference `output/docs/rust/reference/expressions/operator-expr.md`

Lines 508-516 — the desugar (same span lesson 117 cited):

> ```rust
> let a = 1;
> let b = 1;
> a == b;
> // is equivalent to
> ::std::cmp::PartialEq::eq(&a, &b);
> ```

Lines 523-526 — the operator-to-method table:

> | `==` | Equal     | `std::cmp::PartialEq::eq` |
> | `!=` | Not equal | `std::cmp::PartialEq::ne` |

### Error codes `output/docs/rust/error_codes/E0369.md`

Line 4:

> A binary operation was attempted on a type which doesn't support it.

The fix instruction at line 18 — "please check that this type
implements this binary operation" — corresponds to the contrast
probe's `note:` ("an implementation of `PartialEq` might be
missing").

### rmp `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

Line 21 — the unlock target:

> `if ord == cmp::Ordering::Equal { ... }`

## Claim-to-evidence map

- "`==` and `!=` work on two `Ordering` values" — Ordering page
  line 327 (the `impl PartialEq for Ordering` block); Probe 1
  transcript (compiles and prints expected three lines).
- "The result is a `bool`" — Ordering page line 331
  (`fn eq(&self, other: &Ordering) -> bool`); Ordering page line 337
  (`fn ne(...) -> bool`); Probe 1's `println!` formats the result
  with `{}`, which renders the bare word `true`/`false`.
- "Same variant → equal; different variant → unequal" — Probe 1
  transcript (`a == b is true` for `Less`/`Less`; `a == c is false`
  for `Less`/`Equal`; `a != c is true`); Probe 3 corroborating
  matrix (full 3x3, diagonal true off-diagonal false).
- "`==` desugars to `PartialEq::eq`" — Reference lines 508-516
  verbatim; Reference line 525 (table row).
- "`!=` desugars to `PartialEq::ne`" — Reference line 526 (table row).
- "Std implements `PartialEq` for `Ordering`" — Ordering page
  line 327 verbatim impl block.
- "Whether an enum supports `==` is not automatic — depends on a
  `PartialEq` impl" — Probe 2 verbatim transcript;
  `error_codes/E0369.md` line 4 verbatim; Probe 2's `note:` block
  ("an implementation of `PartialEq` might be missing for `Color`").
- "rmp `cmp.rs:21` `if ord == cmp::Ordering::Equal` is exactly
  today's mechanic" — rmp `cmp.rs` line 21 verbatim; Ordering page
  line 327 (the impl that makes `==` available on `Ordering`).

## Negative / contrast probe coverage

The lesson's centered contrastive claim is "`==` works on `Ordering`
*because* std implements `PartialEq` for it; on a user-declared enum
without that impl, the same operator fails to dispatch." Probe 2
(E0369 on `Color`) is the centered contrast and is captured verbatim.

A second contrast was *not* attempted: trying `<` on `Ordering` to
witness that `PartialOrd` is needed for ordering operators. That
contrast is structurally for the *next* move (PartialOrd-driven
operators on Ordering), not today's, and would only confuse the
lesson's centered focus on PartialEq.

The Copy implication of Probe 3 (the corroborating matrix) is
named-deferred; the working probe (Probe 1) is structured so that
each `Ordering` value is read at most once at the comparison site,
so Copy is not load-bearing for the centered claim. Probe 1
compiles and runs as predicted; the centered claim does not depend
on whether the audience knows about Copy yet.
