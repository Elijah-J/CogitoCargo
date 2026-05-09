# Evidence — Lesson 150: `Iterator::map` (first lazy closure-driven Iterator adapter)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/150-iterator-map.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/150-iterator-map.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/150-iterator-map.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet150/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-149.

## Run context — first lazy closure-driven Iterator adapter (audit §4.4.1, §7)

Lesson 149 installed `for_each` — a *consumer*, `FnMut(Self::Item)`,
returns `()`, no wrapper struct, no element-type rewriting. Audit §7
named the two leading first closure-driven Iterator methods as
`for_each` (chosen for 149) and `map` (chosen for 150). Audit §4.4.1
counts 27 closure-driven Iterator methods in total; today is the
second. The audit's leanness ordering picks `map` next because:

- It is the first *lazy adapter* in the closure-driven family. Lessons
  136-141 installed five non-closure lazy adapters (`take`, `skip`,
  `enumerate`, `fuse`, `step_by`); today is the closure-driven
  analogue.
- The bound `FnMut(Self::Item) -> B` is the smallest extension of
  lesson 149's `FnMut(Self::Item)` — it adds the `-> R` return-type
  segment of lesson 147's parenthesized-bound grammar, exercised on a
  closure-driven Iterator method for the first time.
- The wrapper `Map<Self, F>` is the same struct family as lessons
  136-140's `Take<Self>`/`Skip<Self>`/`Enumerate<Self>` etc., but
  parameterized by two type parameters (`Self` for the source iterator,
  `F` for the closure's anonymous type) instead of one.
- The audit names "intermediate iterator-pipeline capstone" as a
  future move once 2-4 closure-driven methods land. Today is method 2;
  the capstone is *not* this lesson's centered concept.

## Direct prerequisite — lesson 149 (`for_each` as consumer)

Lesson 149 installed `Iterator::for_each(self, f) where F:
FnMut(Self::Item)` — consumes `self`, takes one closure, returns `()`,
non-short-circuiting. Today's working probe chains `.for_each(|y|
println!("{}", y))` onto `.map(...)`, with `for_each` playing the
*consumer* role at the end of the chain. Three sub-claims load-bearing
today:

- The bound `FnMut(Self::Item)` from lesson 149 — today's `y` parameter
  resolves to `u32` because the wrapper's `Item = B = u32`.
- The consuming `self` shape from lesson 149 — `for_each` consumes the
  `Map` wrapper exactly as it consumed the `Range<u32>` in lesson 149.
- The lesson 149 source choice (`Range<u32>`) and discipline (no
  `&u32`, no AddAssign-on-reference, no deref-read) — carried over.

## Direct prerequisite — lesson 148 (Fn / FnMut / FnOnce)

Today's bound is `FnMut(Self::Item) -> B`. Lesson 148 installed the
three-trait family with the auto-implementation rule mapping closure
body actions to traits. Today's closure `|x| x * 10` reads its
parameter and returns a value; under lesson 148's rule it implements
`Fn` (and so all three traits). The bound `FnMut` accepts it because
`Fn: FnMut: FnOnce` (lesson 148's supertrait relation). Probe 6 in the
transcript empirically confirms the bound's shape — rustc spells the
expected closure type as `FnMut(u32)` when a non-closure is passed.

## Direct prerequisite — lesson 147 (parenthesized bound, `-> R` slot)

Lesson 147 installed the parenthesized form `<F: FnMut(T) -> R>` with
both the parameter-types-in-parens segment and the optional `-> R`
return-type segment. Lesson 149's `for_each` bound `FnMut(Self::Item)`
exercised the parens segment but had no `-> R`. Today exercises both
segments at once for the first time on an Iterator method: the bound
`FnMut(Self::Item) -> B` has `Self::Item` inside the parens and `B`
after the arrow. The lesson body's "Read it segment by segment" walk
applies lesson 147's grammar exactly.

## Direct prerequisite — lesson 138 (enumerate frame: yielded element shape rewrite)

Lesson 138 installed the frame "an adapter can rewrite the yielded
element type." `Enumerate<Self>` rewrote `Self::Item = T` to
`Item = (usize, T)` — wrapping in a tuple. Today's rewrite is
*closure-driven* and chooses any type `B`: the wrapper's `Item` is
whatever the closure returns, not a fixed shape baked into the
adapter. The lesson body's "yielded element type is whatever the
closure returns" bullet is exactly this composition.

## Direct prerequisite — lesson 136 (wrapper-struct family)

Lesson 136 installed `Take<Self>` as the first wrapper struct in the
run, with the structural fact that calling `.take(n)` returns a fresh
iterator value (not a number / `Option` / `()`). Lessons 137-140
extended the family: `Skip<Self>`, `Enumerate<Self>`, `Fuse<Self>`,
`StepBy<Self>` — all single-parameter wrappers. Today's `Map<Self, F>`
is the same family with two parameters instead of one. The structural
fact "calling `.map(f)` returns a fresh iterator value" reuses lesson
136's frame; the new fact is the second type parameter for the
closure's anonymous type. The corpus structural source is
`output/docs/rust/std/iter/struct.Map.md:7` verbatim:
`pub struct Map<I, F> { /* private fields */ }` — two type parameters,
private fields (implementor-side, deferred per lesson 136-140
discipline).

## Direct prerequisite — lesson 132 (Iterator trait + `Self::Item`)

Lesson 132 installed `type Item;` as the trait's associated-type slot
and `Self::Item` as the path through it. Today's bound `FnMut(Self::Item)
-> B` reads the closure's parameter type from this slot. For
`Range<u32>`, `Item = u32`. The Map struct page's impl
`impl<B, I, F> Iterator for Map<I, F> where I: Iterator, F:
FnMut(<I as Iterator>::Item) -> B,` followed by `type Item = B;`
(`struct.Map.md:150-154`) reuses the same slot on the *wrapper*'s side
— the wrapper's `Item` is `B`, not `Self::Item`. Probe 5 empirically
confirms the type names by trying `let r: u32 = (1..4_u32).map(|x| x *
10);` and reading rustc's spelling of the wrapper type as
`Map<std::ops::Range<u32>, {closure@...}>`.

## Direct prerequisite — lesson 091 (Range as Iterator + parens-rule)

Lesson 091 grounds two facts load-bearing today:

- `Range<A>: Iterator` where `A: Step`. All twelve integer types from
  lesson 080 implement `Step`; `Range<u32>` is an `Iterator` whose
  `Item = u32`. Probe 7 empirically confirms — rustc spells the
  moved-value type as `std::ops::Range<u32>`.
- Parens-rule for method calls on a range: `(1..4_u32).map(...)` parses
  with the range as the receiver of `.map(...)`. Without parens,
  rustc parses `4_u32.map(...)` first as a method call on the integer
  and fires E0689 (Probe 10 in the transcript witnesses this).

## Direct prerequisite — lesson 081 (integer-literal type-suffix form)

Lesson 081 installs the type-suffix form `4_u32`. Today's `4_u32`
pins the upper bound of the range to `u32`, fixing the range to
`Range<u32>` without a separate `let upper: u32 = 4;` line.

## Direct prerequisite — lesson 080 (integer-type family)

Lesson 080 installs the twelve integer type names. Today's `u32`
appears in the literal suffix `4_u32`, in the resolved `Self::Item`,
and in the closure return type `B = u32`.

## Direct prerequisite — lesson 009 (`*` on integers)

Lesson 009 installed `*` between two integer values. Today's closure
body `x * 10` is `u32 * u32` producing `u32`. No new arithmetic
mechanic; no `as` cast (lesson 034's `i32 as f64` was the only cast
direction installed; the integer-to-integer direction is named in 081
as deferred). Staying inside `u32 * u32` keeps the closure return type
unambiguous and inside lesson 009's installed mechanic.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's reappearing
codes (no new codes today):

- **E0308** (Probe 5): same code as lessons 138/149; today's payload
  spells the actual return type as `Map<std::ops::Range<u32>,
  {closure@...}>` — direct empirical witness for the wrapper-struct
  return shape.
- **E0277** (Probe 6): same code as lessons 146/147/149; today's
  payload spells the trait in the parenthesized form `FnMut(u32)`
  (no `-> _` shown — rustc does not echo the closure's return type
  in this surface).
- **E0382** (Probe 7): same code as lessons 133/134/148/149; today's
  payload's `note:` reads "`map` takes ownership of the receiver
  `self`, which moves `it`." Direct empirical witness for `self`
  consuming, with rustc spelling `map` by name.

The bare-statement warning surfaced by Probe 4 is a *lint*, not an
E-code; the diagnostic shape is similar (headline + `-->` + source
excerpt + `note:` and `help:` lines) but the headline is `warning:
unused \`Map\` that must be used` instead of `error[E####]`. Same
lesson 003 four-part map; new top-level kind.

## Cited prereqs

- **Lesson 145**: `<F>` generic-function type-parameter slot. Today
  uses `<B, F>` — two slots in the same angle brackets (lesson 145's
  `unlocks` named multi-parameter as deferred; today realises it
  trivially because `B` is purely inferred from the closure's body).
- **Lesson 142**: closure literal `|p| body`. Today's two probe
  closures (`|x| x * 10` in `demo.rs` and `|n| n + 100` in `q.rs`)
  are lesson 142 closure literals as call-arguments.
- **Lesson 143**: unannotated closure parameter `|x|` (no `: u32`).
  Today the parameter type comes from the bound `FnMut(Self::Item)
  = FnMut(u32)`. Probe 8 confirms by accepting the explicit
  `|x: u32| -> u32 { x * 10 }` form.
- **Lesson 144**: capture mechanic. Probe 2 in the transcript captures
  no outer binding (the `println!` call is just I/O, not a capture);
  the lazy-witness shape would still work without 144, so 144 is
  cited but not load-bearing.
- **Lesson 005**: `let _m = ...` binding for the lazy-witness probe.
- **Lesson 011**: `println!("{}", x)`.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc + ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics)

The corpus file is the std doc page for `Iterator`. Verified by
reading.

### Line 852 (`map` signature)

```text
#### fn map<B, F>(self, f: F) -> Map<Self, F> ⓘ
   where Self: Sized,
         F: FnMut(Self::Item) -> B,
```

Direct corpus source for the lesson body's signature. Five
load-bearing facts read from this line:

- `<B, F>` — *two* type parameters (vs. `for_each`'s one).
- `(self, f: F)` — receiver `self` (consuming), one closure-typed
  parameter `f: F`.
- `-> Map<Self, F>` — return type is a struct (composes lesson 136's
  wrapper frame with two parameters instead of one).
- `where Self: Sized, F: FnMut(Self::Item) -> B` — the bound; today
  centers the closure-bound segment.
- `FnMut(Self::Item) -> B` — parenthesized bound (lesson 147) with
  `FnMut` (148), `Self::Item` (132), and the new `-> B` segment.

### Lines 39-41 (synopsis-box version)

```text
fn map<B, F>(self, f: F) -> Map<Self, F> ⓘ
   where Self: Sized,
         F: FnMut(Self::Item) -> B { ... }
```

Same signature in the synopsis box at the top of the file. The
`{ ... }` placeholder confirms `map` is a *provided* method (lesson
116) — every Iterator impl gets it for free.

### Lines 850-855 (stabilization, semantics)

```text
1.0.0 ·

#### fn map<B, F>(self, f: F) -> Map<Self, F> ⓘ ...

Takes a closure and creates an iterator which calls that closure on each
element.
```

- "Takes a closure and creates an iterator which calls that closure on
  each element" (lines 854-855) → today's centered semantics. The
  phrase "creates an iterator" is the corpus's own framing for the
  wrapper-struct return.
- Stabilization at 1.0.0 (line 850) → far below the local toolchain
  1.95.0.

### Lines 857-869 (longer semantics + lazy framing)

```text
`map()` transforms one iterator into another, by means of its argument:
something that implements `FnMut`. It produces a new iterator which
calls this closure on each element of the original iterator.

If you are good at thinking in types, you can think of `map()` like this:
If you have an iterator that gives you elements of some type `A`, and
you want an iterator of some other type `B`, you can use `map()`,
passing a closure that takes an `A` and returns a `B`.

`map()` is conceptually similar to a `for` loop. However, as `map()` is
lazy, it is best used when you're already working with other iterators.
If you're doing some sort of looping for a side effect, it's considered
more idiomatic to use `for` than `map()`.
```

- "produces a new iterator which calls this closure on each element"
  → corroborates Probe 3's interleaved output (the consumer's pull
  triggers each closure call).
- "an iterator that gives you elements of some type `A`, and you want
  an iterator of some other type `B`" → corpus framing for the
  closure-return-type-becomes-wrapper-`Item` rule.
- "as `map()` is lazy" → corpus's verbatim word for the lazy
  framing. Probes 2 and 3 empirically witness.

### Lines 887-898 (lazy-warning corpus example)

```text
If you're doing some sort of side effect, prefer `for` to `map()`:

// don't do this:
(0..5).map(|x| println!("{x}"));

// it won't even execute, as it is lazy. Rust will warn you about this.
```

Direct corpus source for the lesson body's bare-statement warning
claim. The corpus's claim "Rust will warn you about this" is exactly
what Probe 4 captures: the `warning: unused \`Map\` that must be used`
diagnostic with `note: iterators are lazy and do nothing unless
consumed`.

## Source — `output/docs/rust/std/iter/struct.Map.md` (wrapper struct, Iterator impl)

Corpus file for the `Map` struct. Verified by reading.

### Line 7 (struct declaration)

```text
pub struct Map<I, F> { /* private fields */ }
```

Direct corpus source for "two type parameters, private fields." The
private fields are implementor-side, deferred per lesson 136-140
discipline.

### Lines 12-15 (one-line semantics)

```text
An iterator that maps the values of `iter` with `f`.

This `struct` is created by the `map` method on `Iterator`.
```

Direct corpus source for "this is the wrapper struct returned by
`Iterator::map`."

### Lines 150-154 (Iterator impl)

```text
### impl<B, I, F> Iterator for Map<I, F>
   where I: Iterator,
         F: FnMut(<I as Iterator>::Item) -> B,

#### type Item = B
```

Direct corpus source for two load-bearing facts:

- `Map<I, F>` *is* an `Iterator` — the wrapper itself implements the
  trait, so any Iterator method (including `for_each` from lesson
  149) can be chained onto it.
- `type Item = B` — the wrapper's yielded element type is `B`, the
  closure's return type. Direct corpus warrant for the lesson body's
  centered claim "the wrapper yields the closure's return value once
  per source element."

The `<I as Iterator>::Item` syntax in the bound is the same
`Self::Item` slot from lesson 132 written in fully-qualified form;
when reading on `Range<u32>`, `<Range<u32> as Iterator>::Item = u32`.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Reused today: `Range<A>:
Iterator where A: Step` (lines 139-141 verbatim). All twelve integer
types from lesson 080 implement `Step`. `1..4_u32` is a `Range<u32>`
and its `Iterator::Item` is `u32`. Probe 7 empirically confirms
("`it` has type `std::ops::Range<u32>`").

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lessons 148 and 149 already cited this. Today reuses without
re-quoting: line 27 verbatim "`FnMut` is implemented automatically
by closures which take mutable references to captured variables."
Today's closures `|x| x * 10` and `|n| n + 100` capture nothing — they
read their parameter `x`/`n` (passed by the bound), do an arithmetic
op, and return. By lesson 148's auto-impl rule, capture-nothing
closures implement `Fn` (and so all three traits). The bound `FnMut`
accepts them because `Fn: FnMut: FnOnce`.

## Probe 1 — working program (map → for_each)

Source: `observations/150-iterator-map.rs`. Transcript: `PROBE 1`
block.

Output: three lines `10` / `20` / `30`. Compile-exit=0, run-exit=0.
Five load-bearing facts:

- The bound `F: FnMut(Self::Item) -> B` accepts a closure literal
  `|x| x * 10` as the argument to `.map(...)` on `(1..4_u32)`.
- The wrapper `Map<Range<u32>, {closure}>` implements `Iterator` (per
  `struct.Map.md:150-154`), so `.for_each(...)` works on it. The
  closure runs once per source element.
- The wrapper's `Item = B = u32` (closure return type), so the
  `.for_each(|y| ...)` closure parameter `y` is `u32`. Probe 8
  empirically confirms by annotating `|x: u32| -> u32` explicitly.
- Output values are `10, 20, 30` (= `1*10, 2*10, 3*10`); the
  exclusive upper bound rule from lesson 091 means `4` is not
  yielded.
- `(1..4_u32)` is a `Range<u32>`. Probe 7 empirically confirms.

## Probe 2 — lazy witness (bind wrapper, never consume)

Source: `lazy.rs`. Transcript: `PROBE 2` block.

Modification from Probe 1: replace the chained `.for_each(...)` with
`let _m = ...;` and add a `println!("end")` at the bottom. The
closure body contains `println!("called: {}", x)` — if it ran on any
element, we would see `called: 1`, `called: 2`, `called: 3`. Output:

```text
end
```

Compile-exit=0, run-exit=0. Three load-bearing facts:

- `.map(...)` builds a `Map` value but does not call the closure on
  any element. The `called: ...` lines do not appear.
- The `_m` binding is held until the end of `main`; the `Map` value
  is alive in scope, but no one iterates it.
- Direct empirical witness for the corpus claim
  `trait.Iterator.md:867`: "as `map()` is lazy, it is best used when
  you're already working with other iterators."

## Probe 3 — lazy contrast (same closure body, IS consumed)

Source: `lazy_consume.rs`. Transcript: `PROBE 3` block.

Modification from Probe 2: replace `let _m = ...;` with the chained
`.for_each(|y| println!("got: {}", y))` consumer. Output:

```text
called: 1
got: 10
called: 2
got: 20
called: 3
got: 30
end
```

Compile-exit=0, run-exit=0. Three load-bearing facts:

- The closure now runs three times. The differences between Probes
  2 and 3 are *only* on the consumer side, not the `.map(...)` call;
  same closure literal, same `Map<Range<u32>, _>` value.
- Output is *interleaved* (`called: 1` immediately followed by
  `got: 10`), not batched (`called: 1, called: 2, called: 3` then
  `got: 10, ...`). Direct empirical witness for the pull-style
  one-element-at-a-time iteration: `for_each` calls `next()` on the
  `Map` wrapper, which calls `next()` on the source `Range<u32>`,
  gets `1`, hands it to the closure, gets `10`, hands `10` to
  `for_each`'s closure, prints `got: 10`, then loops.
- `for_each` on the wrapper consumes the wrapper (lesson 149's
  consuming `self`); the source `Range<u32>` was already consumed by
  `.map(...)` (today's E0382 Probe 7).

## Probe 4 — bare expression-statement form (must_use lint)

Source: `expr.rs`. Transcript: `PROBE 4` block.

Modification from Probe 1: drop the `.for_each(...)` and the `let
_m =`. The expression statement is `(1..4_u32).map(|x| x * 10);`
followed by `println!("end")`. Output:

```text
warning: unused `Map` that must be used
 --> expr.rs:5:5
  |
5 |     (1..4_u32).map(|x| x * 10);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
5 |     let _ = (1..4_u32).map(|x| x * 10);
  |     +++++++

warning: 1 warning emitted
```

Compile-exit=0 (warning, not error), run-exit=0; runtime output is
just `end`. Three load-bearing facts:

- rustc emits a `warning:` (not `error:`) — the program compiles and
  runs. The `Map` type carries a `#[must_use]` attribute (the
  implementor-side detail not centered today).
- The `note:` line "iterators are lazy and do nothing unless
  consumed" is the corpus claim from `trait.Iterator.md:867` and
  the corpus example claim at lines 887-892 ("it won't even execute,
  as it is lazy. Rust will warn you about this") — surfaced verbatim
  by rustc itself.
- The `help:` line proposes `let _ = ...` as the fix to silence the
  warning. Probe 2's `let _m = ...` is structurally similar; the `_`
  / `_m` underscore-prefix form composes with lesson 144's no-warn
  rule for unused bindings.

## Probe 5 — bind map result to non-Map type (E0308)

Source: `non_unit_bind.rs`. Transcript: `PROBE 5` block.

Modification: bind to `let r: u32 = (1..4_u32).map(|x| x * 10);`.
Output:

```text
error[E0308]: mismatched types
 --> non_unit_bind.rs:6:18
  |
6 |     let r: u32 = (1..4_u32).map(|x| x * 10);
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `Map<Range<u32>, {closure@...}>`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
           found struct `Map<std::ops::Range<u32>, {closure@non_unit_bind.rs:6:33: 6:36}>`
```

Compile exit 1. Three load-bearing facts:

- rustc spells the actual return type as `Map<std::ops::Range<u32>,
  {closure@...}>`. Direct empirical witness for two centered
  structural facts: (i) `map` returns a `Map<I, F>` wrapper struct;
  (ii) the source iterator's type is the first parameter, the
  closure's anonymous type is the second.
- The closure's anonymous type is rustc-spelled `{closure@FILE:L:C: L:C}`
  — a unique-per-call-site type, distinct from any nameable named
  type. This is the type system's rendering of "the closure's
  anonymous type"; further detail (e.g. how rustc generates and
  manages this type per call site) is implementor-side, deferred.
- Same E-code as lessons 138, 149 (Probe 3 in 149 also fired E0308);
  today's payload's `found` line is the most informative yet because
  it spells out the full wrapper-struct type.

## Probe 6 — pass a non-closure to map (E0277)

Source: `non_closure_arg.rs`. Transcript: `PROBE 6` block.

Modification: pass `7` instead of a closure to `(1..4_u32).map(7)`.
Output:

```text
error[E0277]: expected a `FnMut(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:4:29
  |
4 |     let _m = (1..4_u32).map(7);
  |                         --- ^ expected an `FnMut(u32)` closure, found `{integer}`
  |
  = help: the trait `FnMut(u32)` is not implemented for `{integer}`
note: required by a bound in `map`
```

Compile exit 1. Three load-bearing facts:

- rustc spells the expected closure type as `FnMut(u32)` — direct
  empirical witness that `Self::Item = u32` for `Range<u32>` and that
  the bound's parens segment resolves to `FnMut(u32)`. Same payload
  shape as lesson 149's Probe 4.
- The diagnostic does NOT echo the `-> B` return-type segment
  (rustc spells the trait as `FnMut(u32)`, not `FnMut(u32) -> _`).
  This is a rustc surface choice — the bound *does* have a return
  segment, but rustc's diagnostic spelling truncates it because `B`
  is unconstrained at this call site (no closure body to read it
  from). The signature-level fact "the bound has a `-> B` segment"
  is grounded in the verbatim corpus signature
  `trait.Iterator.md:852`, not in this E0277 payload.
- The `note: required by a bound in \`map\`` block points at the
  bound itself in the function's declaration (in the std-lib
  source). Same shape as lesson 149's Probe 4.

## Probe 7 — use the iterator after `map` consumes it (E0382)

Source: `map_consumes.rs`. Transcript: `PROBE 7` block.

Modification: bind `let it = 1..4_u32;`, call `it.map(|x| x * 10)`,
then call `it.count()` afterwards. Output:

```text
error[E0382]: use of moved value: `it`
 --> map_consumes.rs:6:18
  |
4 |     let it = 1..4_u32;
  |         -- move occurs because `it` has type `std::ops::Range<u32>`, which does not implement the `Copy` trait
5 |     let _m = it.map(|x| x * 10);
  |                 --------------- `it` moved due to this method call
6 |     let _again = it.count();
  |                  ^^ value used here after move
  |
note: `map` takes ownership of the receiver `self`, which moves `it`
```

Compile exit 1. Three load-bearing facts:

- Error code `E0382`. Same code lessons 133, 134, 148, 149 have
  witnessed for `self`-by-value receivers; today's payload's `note:`
  line reads verbatim "`map` takes ownership of the receiver `self`,
  which moves `it`" — direct empirical witness for `map`'s consuming
  receiver, with rustc spelling `map` by name.
- Rustc spells the moved-value type as `std::ops::Range<u32>` —
  direct empirical witness that `1..4_u32` is a `Range<u32>` value
  (composes lesson 091 with lesson 081).
- Same structural shape as lesson 149's Probe 5 with the method name
  `for_each` rotated to `map`.

## Probe 8 — type-pin (`|x: u32| -> u32 { x * 10 }`)

Source: `type_pin.rs`. Transcript: `PROBE 8` block.

Modification from Probe 1: annotate both the closure parameter type
(`x: u32`) and the closure return type (`-> u32 { ... }`) explicitly.
Output: same three lines `10` / `20` / `30`, compile-exit=0,
run-exit=0. Two load-bearing facts:

- The closure parameter type *is* `u32` (owned, not `&u32`). Probe 1
  leaves this inferred (lesson 143 rule); this probe annotates it
  explicitly and rustc accepts.
- The closure return type *is* `u32`. Probe 1 leaves this inferred;
  this probe annotates `-> u32` explicitly and rustc accepts. Direct
  empirical witness for `B = u32` in this call.

## Probe 9 — Check Yourself

Source: `q.rs`. Transcript: `PROBE 9` block.

`(1..5_u32).map(|n| n + 100).for_each(|y| println!("{}", y))`. Output:
four lines `101` / `102` / `103` / `104`, compile-exit=0, run-exit=0.
Verifies the Check Yourself part (a) answer empirically: `1..5_u32`
yields `1, 2, 3, 4` (exclusive upper bound rule, lesson 091/022); the
closure adds `100`; `for_each` prints each.

## Probe 10 — side: drop parens around range (E0689, parens-rule)

Source: `no_parens.rs`. Transcript: `PROBE 10` block.

Modification: drop the parens around `1..4_u32`. Output:

```text
error[E0689]: can't call method `map` on type `u32`
 --> no_parens.rs:5:14
  |
5 |     1..4_u32.map(|x| x * 10).for_each(|y| println!("{}", y));
  |              ^^^ can't call method `map` on type `u32`
  |
help: you must surround the range in parentheses to call its `map` function
```

Compile exit 1. Side-probe only — rehearses lesson 091's parens-rule
(originally witnessed for `.rev()`), not centered today. Recorded so
the lesson body's "(parens-rule)" parenthetical in Prerequisites has
a direct cite.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn map<B, F>(self, f: F) -> Map<Self, F> where Self: Sized, F: FnMut(Self::Item) -> B` | `output/docs/rust/std/iter/trait.Iterator.md:852` verbatim |
| `map` consumes `self` | Same line; Probe 7 empirical (E0382 with `note:` "takes ownership of the receiver `self`") |
| Returns `Map<Self, F>` (a wrapper struct) | Signature; `struct.Map.md:7` verbatim `pub struct Map<I, F>`; Probe 5 empirical (rustc spells `found struct \`Map<std::ops::Range<u32>, {closure@...}>\``) |
| Closure bound is `FnMut(Self::Item) -> B` | Signature; Probe 6 empirical (rustc spells `FnMut(u32)`) |
| `Self::Item = u32` for `Range<u32>` | Lesson 091 (`Range<A>: Iterator` for `A: Step`); lesson 080 (u32); lesson 132 (`Self::Item` slot); Probe 6 + Probe 7 + Probe 8 empirical |
| `Map<I, F>` itself implements `Iterator` with `Item = B` | `output/docs/rust/std/iter/struct.Map.md:150-154` verbatim; Probe 1 + Probe 3 empirical (chaining `.for_each(...)` works) |
| `B = u32` for today's closure `|x| x * 10` | Lesson 009 (`*` on integers); Probe 1 empirical (output is `10/20/30`); Probe 8 empirical (explicit `-> u32` accepted) |
| `(1..4_u32)` is a `Range<u32>` value | Lesson 091; lesson 081; Probe 7 empirical (rustc spells `std::ops::Range<u32>`) |
| Parens-rule: methods on a range need surrounding parens | Lesson 091; Probe 10 empirical (E0689 without parens) |
| `map` is *lazy*: closure not called when wrapper is built | `trait.Iterator.md:867` verbatim "as `map()` is lazy"; Probe 2 empirical (no `called: ...` lines) |
| `map` calls the closure once per source element when iterated | `trait.Iterator.md:854-855` verbatim "calls that closure on each element"; Probe 3 empirical (three `called: ...` lines, interleaved with `got: ...`) |
| Bare-statement form fires `must_use` warning with lazy-framing note | `trait.Iterator.md:887-892` verbatim corpus example; Probe 4 empirical (full warning transcript) |
| Yielded element type is whatever the closure returns | `struct.Map.md:154` verbatim `type Item = B`; today's `B = u32` so wrapper yields `u32` |
| Stabilized at 1.0.0 | `trait.Iterator.md:850` verbatim; toolchain is 1.95.0 |
| `x * 10` legal on `u32 * u32` | Lesson 009 (`*` on integers, both sides owned `u32`); Probe 1 empirical (compiles, prints `10/20/30`) |
| `n + 100` legal on `u32 + u32` (Check Yourself) | Lesson 009; Probe 9 empirical (compiles silently, prints `101..104`) |
| Exclusive upper bound: `1..N_u32` does not yield `N` | Lesson 091/022; Probe 1 empirical (`1..4_u32` yields `1, 2, 3` not `4`); Probe 9 empirical (`1..5_u32` yields `1..4`) |

## Older supporting lessons (named only)

- 149-iterator-for-each — consumer that iterates the `Map` wrapper today.
- 148-fn-fnmut-fnonce-distinction — `FnMut` choice + auto-impl rule.
- 147-fn-trait-parenthesized-bound — `<F: FnMut(T) -> R>` shape;
  today exercises both segments at once.
- 146-trait-bound-on-type-parameter — inline trait bound shape.
- 145-generic-function-type-parameter — `<F>` slot, today extended
  to `<B, F>`.
- 144-closure-captures-outer-let — capture mechanic (cited only;
  today's probe closures capture nothing).
- 143-unannotated-closure-first-use — `|x|` without annotation.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 138-iterator-enumerate — element-shape rewrite frame.
- 136-iterator-take — wrapper-struct frame.
- 132-iterator-trait-declaration — `Self::Item` slot.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens-rule (Probe 10).
- 081-integer-literal-forms — `4_u32` suffix form.
- 080-integer-type-family — twelve integer types.
- 009-arithmetic-on-integers — `*` on integers.
- 011-println-positional-args — `println!("{}", x)`.
- 005-let-binding — `let _m = ...`.
- 003-read-rustc-diagnostic, 002-fn-main-entry-point,
  001-rustc-compile-and-run.

## Deliberate scope discipline

The orchestrator's prompt named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. `Map<I, F>` internal fields and other trait impls
   (`DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`,
   `TrustedLen` from `struct.Map.md` lines 88, 132, 688, 692) —
   implementor-side, not centered.
2. `flat_map` — gated on `IntoIterator` (audit §4.4.2).
3. `map_while` — separate method.
4. `filter_map` — separate method.
5. The "iterator chain" / pipeline as a centered concept — defer to
   future capstone after 2-4 closure-driven methods land (audit §7).
6. The `Notes about side effects` section of `struct.Map.md`
   (lines 17-56) — implementor-side; not centered.
7. The element-type-changing nature of `map` realised on a different
   `B` type (e.g. `u32` to `String`). Today's body keeps `B = u32`
   for surface minimality; the mechanic is the same.
8. `v.iter()` / `v.into_iter()` source shapes — each composes a
   different `Self::Item` resolution.

## Mechanics deliberately *not* smuggled

The orchestrator's reminder list called out specific smuggling risks
from the prior `for_each` round. Today's discipline check:

- **No `&u32` element types**: source is `Range<u32>`, so
  `Self::Item = u32` (owned). Probe 8 explicitly annotates
  `|x: u32|` and rustc accepts.
- **No deref-read `*x`**: closure body is `x * 10`, plain `u32 *
  u32` per lesson 009. Check Yourself body is `n + 100`, plain
  `u32 + u32` per lesson 009.
- **No AddAssign-on-reference**: today's bodies use only `*` and
  `+` on owned `u32` values, both installed by lesson 009. No
  compound assignment, no `+=`, no `AddAssign`-machinery reach.
- **No `as` casting**: lesson 034 installed only `i32 as f64`;
  lesson 081 named integer-to-integer `as` as deferred. Today's
  closure body produces a `u32` directly via `* 10`, no cast
  needed. (Probe 8 annotates `-> u32` explicitly, not via cast.)
- **No `Box<dyn Fn>`, no `impl Fn`, no `move` keyword, no Try
  machinery, no `IntoIterator`**: none appear in any probe. Sources
  are bare ranges; consumers are `for_each` (lesson 149).
- **No `.iter()` or `.into_iter()` source**: all probes use
  `(1..N_u32)` directly, per lesson 149's discipline.

## Run-context handoff

After this lesson lands, the orchestrator's options for lesson 151
include:

- `filter` — second lazy adapter; bound is `FnMut(&Self::Item) ->
  bool`, surfaces the *double-reference* situation
  (`trait.Iterator.md:962-967`); returns `Filter<Self, P>`. Composes
  today's mechanic with a *predicate-shape* bound.
- `inspect` — like `for_each` but lazy; bound is
  `FnMut(&Self::Item)` (no return); returns `Inspect<Self, F>`.
  Lighter than `filter` because no element-type rewrite, no
  predicate decision.
- `find` / `position` / `any` / `all` — short-circuiting consumers;
  bound is `FnMut(&Self::Item) -> bool` or similar.
- `fold` — fold-with-closure; bound is `FnMut(B, Self::Item) -> B`
  (two-parameter parenthesized bound, the next sub-step beyond
  today's one-parameter form).

The audit's intermediate-pipeline capstone (§7) becomes more
plausible once `filter` lands — then a chain
`(1..N_u32).filter(...).map(...).for_each(...)` reads end-to-end
in the audience's installed mechanics.
