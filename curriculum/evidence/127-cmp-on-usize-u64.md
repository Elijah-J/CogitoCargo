# Evidence — 127-cmp-on-usize-u64

Audit appendix for `lessons/127-cmp-on-usize-u64.md`. Holds the
corpus-quote map, the toolchain string, the working-probe
transcript, the cross-type contrast E0308 transcript, the
corroborating `Vec::len().cmp(&...)` transcript, and the
prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srm` → `Darwin 24.5.0 x86_64`
- Probes captured 2026-05-08 in `/tmp/lesson127/` on this host.
- The working `.rs` source plus a single `.transcript.txt` are
  committed under `observations/127-cmp-on-usize-u64.{rs,transcript.txt}`.
  The contrast and corroborating sources are not committed; their
  transcripts in the `observations/...transcript.txt` artifact are
  the evidence.

## Sources

### `output/docs/rust/std/cmp/trait.Ord.md`

The std-library page for the `Ord` trait. Already cited in lesson
061. Reused here for two load-bearing spans.

Lines 6-19 (the trait declaration with `cmp`'s signature):

> ```
> pub trait Ord: Eq + PartialOrd {
>     // Required method
>     fn cmp(&self, other: &Self) -> Ordering;
>
>     // Provided methods
>     fn max(self, other: Self) -> Self
>        where Self: Sized { ... }
>     fn min(self, other: Self) -> Self
>        where Self: Sized { ... }
>     fn clamp(self, min: Self, max: Self) -> Self
>        where Self: Sized { ... }
> }
> ```

The required method's signature `fn cmp(&self, other: &Self) ->
Ordering` is *the* corpus statement for the lesson body's "same
signature, every implementing type reuses it with `Self` replaced
by itself" claim. `Self` in the signature literally means the
implementing type; the per-primitive pages below show the
specialization for `u64` and `usize`.

The supertrait bound (`: Eq + PartialOrd`) and the three provided
methods (`max` / `min` / `clamp`) are explicitly deferred — the
lesson's *What To Ignore For Now* names all four as deferred trait
machinery, identical to lesson 061's deferral.

Lines 467 and 479 (the `Ord` implementor list, `u64` and `usize`):

> ### impl [Ord](trait.Ord.md "trait std::cmp::Ord") for [u64](../primitive.u64.md)

> ### impl [Ord](trait.Ord.md "trait std::cmp::Ord") for [usize](https://doc.rust-lang.org/stable/std/primitive.usize.html)

These two list entries are the corpus statement that `u64` and
`usize` implement `Ord`, so `.cmp` is reachable on them by the
same trait that made it reachable on `i32` (lesson 061, line 423
of the same page). The lesson body's "the same page lists `impl
Ord for u64` and `impl Ord for usize` alongside `impl Ord for i32`"
is grounded here.

Calibration: the same page lists ~50 `impl Ord for ...` entries.
The lesson defers all integer types other than `usize` / `u64`
(and `i32` from lesson 061) explicitly under *What To Ignore For
Now*. The lesson does *not* claim "implemented for every primitive
integer type" outside the appendix, even though the trait page
makes that claim available.

### `output/docs/rust/std/primitive.u64.md`

The std-library page for the `u64` primitive type. Already cited
across multiple lessons (108 in particular). Today's citation is
the specific `impl Ord for u64` block.

Lines 4231-4237 (the `Ord` implementation for `u64`):

> ### impl [Ord](cmp/trait.Ord.md "trait std::cmp::Ord") for [u64](primitive.u64.md)
>
> #### fn [cmp](cmp/trait.Ord.md#tymethod.cmp)(&self, other: &[u64](primitive.u64.md)) -> [Ordering](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html "enum std::cmp::Ordering")
>
> This method returns an [`Ordering`] between `self` and `other`. [Read more](cmp/trait.Ord.md#tymethod.cmp)

Two load-bearing facts: (a) `u64` implements `Ord`, so `.cmp` is
reachable on `u64`-typed values; (b) the specialized signature for
`u64` reads `fn cmp(&self, other: &u64) -> Ordering` — the trait's
`&Self` is concretely `&u64`. The contrast probe (mixing `u64` and
`i32`) fires E0308 with `expected `&u64`, found `&i32`` because
the `&Self` parameter on `u64`'s `cmp` is concretely `&u64`.

### `output/docs/rust/std/primitive.usize.md`

The std-library page for the `usize` primitive type. Already
cited in lesson 077 (`usize` indexing).

Lines 4538-4544 (the `Ord` implementation for `usize`):

> ### impl [Ord](cmp/trait.Ord.md "trait std::cmp::Ord") for [usize](primitive.usize.md)
>
> #### fn [cmp](cmp/trait.Ord.md#tymethod.cmp)(&self, other: &[usize](primitive.usize.md)) -> [Ordering](cmp/enum.Ordering.md "enum std::cmp::Ordering")
>
> This method returns an [`Ordering`] between `self` and `other`. [Read more](cmp/trait.Ord.md#tymethod.cmp)

Same two facts as the `u64` page, with `usize` substituted: (a)
`usize` implements `Ord`; (b) the specialized signature reads
`fn cmp(&self, other: &usize) -> Ordering`. This is the corpus
license for the working probe's `c.cmp(&d)` line where both `c`
and `d` are typed `usize`, and for the corroborating probe's
`v.len().cmp(&w.len())` (`Vec::len()` returns `usize` per lesson
107, so the chain is `usize.cmp(&usize)` which matches the
specialized signature).

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308 *mismatched types*. Already
cited in many lessons, including 061's missing-`&` contrast on
`i32`. Today's contrast triggers the same E-code with the
type pair `&u64` vs `&i32` instead of `&i32` vs `i32`. The
explainer's first example (`plus_one("Not a number")` with
`fn plus_one(x: i32) -> i32`) is structurally the same call-site
argument-type mismatch sub-case; the only difference is which
type pair is mismatched. The lesson body cites the E-code by
family ("Same E-code as lesson 061's missing-`&` contrast") and
does not re-explain it.

## Probes

### Working probe

Source: `experimental/eduratchet2/runs/rust-moves/observations/127-cmp-on-usize-u64.rs`.
Identical source to *The Move* code block in the lesson.

Transcript captured at
`experimental/eduratchet2/runs/rust-moves/observations/127-cmp-on-usize-u64.transcript.txt`.
Headline:

```text
--- rustc demo.rs ---
exit=0
(no output)

--- ./demo ---
u64: a < b
usize: c == d
exit=0
```

Notes:

- `rustc demo.rs` is silent, exits 0 — same compile-success shape
  as lesson 001.
- `./demo` prints exactly two lines. The first line is the body of
  the `Ordering::Less` arm of the first `match`, so reaching it
  empirically witnesses `100u64.cmp(&200u64) == Ordering::Less`.
  The second line is the body of the `Ordering::Equal` arm of the
  second `match`, witnessing `5usize.cmp(&5usize) ==
  Ordering::Equal`.
- The probe deliberately uses two different `Ordering` arms across
  its two `match` blocks: `Less` (u64 case) and `Equal` (usize
  case). This is enough to corroborate that the call expression
  has type `Ordering` for both receiver types — not just for one.
  The third arm (`Greater`) is exercised by the corroborating
  probe below (`v longer` reaches `Ordering::Greater`).

### Cross-type contrast probe

Not committed; transcript embedded in `127-cmp-on-usize-u64.transcript.txt`.

Source:

```rust
fn main() {
    let a: u64 = 100;
    let b: i32 = 5;
    let _ = a.cmp(&b);
}
```

Receiver `a` is `u64`. Argument `&b` has type `&i32`. The trait's
`Self` parameter is concretely `u64` for this call (because the
receiver's type drives method resolution), so the argument slot
expects `&u64`. `&i32` does not match.

```text
error[E0308]: mismatched types
 --> broken.rs:4:19
  |
4 |     let _ = a.cmp(&b);
  |               --- ^^ expected `&u64`, found `&i32`
  |               |
  |               arguments to this method are incorrect
  |
  = note: expected reference `&u64`
             found reference `&i32`
note: method defined here
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/cmp.rs:999:7

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Notes (probe evidence — not corpus quotation):

- E-code is E0308. Same E-code as lesson 061's missing-`&` contrast
  and the wider E0308 family used in lessons 024, 025, 026, 028,
  033, 045, 046, 047, 048, 061.
- The inline label is `expected `&u64`, found `&i32``. This is the
  load-bearing piece of probe evidence for the lesson's "Self in
  the signature is the receiver's type" claim — rustc explicitly
  reports the expected reference type as `&u64` (the receiver's
  type prefixed with `&`), not `&i32`.
- The `note:` block's `expected reference `&u64` / found reference
  `&i32`` separates the two reference types into one line each, a
  clearer sub-form of the inline label.
- The `arguments to this method are incorrect` underline points at
  the method-name position (`cmp`), the same shape lesson 061
  captured for E0308 in a method-call context.
- The dual-`-->` pattern (lesson 036; reused in 061): `note:
  method defined here` followed by a second `-->` pointing into
  the standard-library source
  (`/rustc/59807616e.../library/core/src/cmp.rs:999:7`). Same
  source location as lesson 061's broken-contrast probe — the
  `core::cmp::Ord::cmp` declaration the call resolves through.
- Exit code 1; no executable produced.

This contrast probe is necessary because the lesson makes the
contrastive claim "operands must be the same type." rustc itself
states the rule via the `expected `&u64`, found `&i32`` label.
A contrast probe with a missing `&` (lesson 061's shape, in
effect, would still hold for `u64`) is not separately re-captured
here because lesson 061's transcript already documents that
sub-case verbatim and the `i32`-only-vs-other type pair is
orthogonal to today's centered claim.

### Corroborating probe — rmp `cmp.rs:20` shape

Not committed; transcript embedded in `127-cmp-on-usize-u64.transcript.txt`.

Source:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![10, 20];
    let result = v.len().cmp(&w.len());
    match result {
        std::cmp::Ordering::Less => println!("v shorter"),
        std::cmp::Ordering::Greater => println!("v longer"),
        std::cmp::Ordering::Equal => println!("same length"),
    }
}
```

This is exactly the rmp `src/biguint/cmp.rs:20` shape
`self.limbs.len().cmp(&other.limbs.len())` transcribed onto a
probe-level program, with `v` / `w` standing in for the two
`BigUInt` operands' `limbs` fields. `Vec::len()` returns `usize`
(lesson 107), so both operands of the chained `.cmp(&...)` are
`usize` values — exactly today's centered case.

```text
--- rustc corroborator.rs ---
exit=0
(no output)

--- ./corroborator ---
v longer
exit=0
```

Notes:

- Compiles silently, exits 0. The chained call `v.len().cmp(&w.len())`
  is the lesson-049 chained-dot-call shape with `.len()` returning
  `usize` and `.cmp(&usize)` accepting it.
- Output is `v longer` because `v.len() == 3` and `w.len() == 2`,
  so `3usize.cmp(&2usize) == Ordering::Greater`.
- Witnesses: (a) `.cmp` works on `usize` outside the lesson's
  explicit `let c: usize = ...; let d: usize = ...; c.cmp(&d)`
  shape (which could in principle be a special case of typed
  literals), and (b) the rmp use site `.len().cmp(&...len())` is
  immediately readable end-to-end, fulfilling the lesson's stated
  unlock for rmp `cmp.rs:20`.
- Reaches the third `Ordering` arm (`Greater`), complementing the
  working probe's `Less` and `Equal` arms.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 127.

- **Lesson 061 (load-bearing for `.cmp(&other)` returning
  `Ordering`)** — installed `a.cmp(&b)` on `i32` with the
  signature `fn cmp(&self, other: &Self) -> Ordering`, the
  `use std::cmp::Ordering;` import shape, and the three-arm match
  on the call expression. Today's working probe reuses lesson 061's
  exact shape (use line, two `let` bindings of the same type, a
  `match call { ... }` with three `Ordering::*` arms) twice in one
  program — once for `u64` and once for `usize`. The new claim is
  *only* that `.cmp` is reachable with the same shape on those two
  additional receiver types. Lesson 061 already captured the
  E0308 sub-case for the missing-`&` mistake; today's E0308
  contrast captures the cross-type sub-case, which lesson 061
  did not exercise.
- **Lesson 080 (load-bearing for `usize` and `u64` as integer
  type names)** — installed the twelve-name integer family by sign
  and width, including `u64` and `usize`, and the `: TYPE`
  annotation slot reused unchanged. Today's working probe
  annotates each binding (`let a: u64 = 100;`, `let c: usize =
  5;`) so the centered receiver types are visible at the source
  level without inference.
- **Lesson 051 (cited)** — installed the `Ordering` enum and the
  three-variant match. Reused unchanged via lesson 061 as in
  lesson 061's `depends_on`.
- **Lessons 040, 044, 045 (cited)** — dot-call grammar, `use
  std::cmp::Ordering;`, prefix-`&` operator. Reused via lesson
  061. The lesson body does not re-teach any of these.
- **Lessons 001, 002, 003, 005, 011, 019** — compile/run shape,
  `fn main`, the four-part diagnostic map, `let name = value;`,
  `println!("{}", x)`, `let name: TYPE = value;`. Used unchanged.

## Older supporting lessons

- Lesson 107 (cited only — appendix corroborator): `Vec::new()`
  empty / prefilled construction with `vec![]`, `.len()` returning
  `usize`. The corroborator probe uses both. The chain
  `v.len().cmp(&w.len())` is exactly the rmp `cmp.rs:20` shape
  and the immediate unlock for the next move in the rmp target
  reading.
- Lesson 049 (cited only — appendix corroborator): chained-dot-call
  shape `expr.method1().method2(arg)`. Used in the corroborator's
  `v.len().cmp(&w.len())` chain.
- Lesson 036 (dual-`-->` diagnostic shape): the `note: method
  defined here` second `-->` in the contrast probe's transcript
  is the same dual-`-->` form lesson 036 first observed. Not
  re-cited in the lesson body.
- Lesson 077 (cited via 080): `usize` as the architecture-dependent
  indexing type. Reused via lesson 080's family-naming.
- Wider E0308 family (lessons 024, 025, 026, 028, 033, 045, 046,
  047, 048, 061): different sub-cases of "expected type X, found
  type Y." Today's contrast probe is the cross-type method-argument
  sub-case; cited only by family.

## Why no separate "missing-&" contrast probe today

Lesson 061 already captured the missing-`&` contrast on `i32`
verbatim, with `expected `&i32`, found `i32`` and the `help:
consider borrowing here` source-diff. The same diagnostic shape
holds for `u64` and `usize`; rustc's E0308 sub-case is uniform
across `Ord` implementors. Re-capturing it here would duplicate
lesson 061's transcript with one type-name change. The
cross-type contrast (mixing `u64` and `i32`) is the *new*
sub-case today's lesson centers — the rule "both operands must be
the same type" is what's load-bearing for extending from "I know
`.cmp` on `i32`" to "I know `.cmp` on `usize` and `u64`." Today's
contrast probe captures that rule directly.
