# Evidence — Lesson 155: `Iterator::position` (third predicate consumer; `Option<usize>` return)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/155-iterator-position.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/155-iterator-position.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/155-iterator-position.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet155/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-154.

## Run context — `position` as third predicate consumer

Lessons 153 and 154 installed `any` and `all` — the first two
predicate consumers. Today's `position` is the third and final
"signature-shape" sibling because:

1. Same closure shape `FnMut(Self::Item) -> bool` as lessons 153/154.
   No new closure-bound mechanic.
2. Same `&mut self` receiver — iterator still usable after.
3. Same short-circuit-on-`true` rule as `any` (whereas `all` short-
   circuits on `false`).
4. **One new structural fact**: the return type rotates from `bool`
   to `Option<usize>`. This is the smallest possible move because it
   composes lesson 119's `Option<T>` with `T = usize` (lesson 077)
   plus lesson 138's enumerate-style index counting. No new type
   primitive enters the picture.
5. The index-vs-value distinction is the centered concept today;
   Probe 1 (`Some(5)` for matching `15`) and Probe 4 (`Some(5) Some(16)`
   side by side) make it empirically concrete.

`position` is opened *after* `any` and `all` because (a) the empty-
case behavior on `Option<_>`-returning consumers was already installed
by lesson 152 (`reduce`) and the audience can map it onto `position`
without re-derivation; (b) the existential-versus-universal duality
of `any`/`all` reads more naturally with both bool-returning siblings
adjacent before introducing the wrapper-rotation.

## Direct prerequisite — lesson 154 (`all`; load-bearing)

Lesson 154 just installed the second predicate consumer and is the
direct sibling. Five sub-claims load-bearing today:

- **Signature shape carries unchanged.** Lesson 154 (and 153 before
  it) installed the segment grammar `<F>(&mut self, f: F) -> bool
  where F: FnMut(Self::Item) -> bool`. Today's `position` rotates
  only the return slot to `Option<usize>` and (cosmetically) the
  type-parameter name from `<F>` to `<P>`. The lesson body says
  explicitly "this is the same shape" and cross-references 154 (and
  through it 153) rather than re-deriving each segment.
- **Receiver is `&mut self`.** Lessons 131/153/154 installed this.
  Today's Probes 4 and 9 rotate the same witnesses with method name
  `position`.
- **Closure bound is `FnMut(Self::Item) -> bool` (single parameter).**
  Lessons 153/154 installed this single-parameter parens-bound shape.
  Today's Probe 8 rotates the same witness with method name
  `position`.
- **Short-circuit on first `true`.** Lesson 153 installed this for
  `any`; lesson 154 inverted it for `all` (first `false`); today's
  `position` reuses the `any` polarity — first `true` from the
  closure, then stop. Probes 1, 4, 6 empirical.
- **Iterator position lands just past the matching element.** Lesson
  153 Probe 2 showed `any` matching on `5` left `6` next. Today's
  Probe 4 shows `position` matching on `15` left `16` next — same
  position-after-short-circuit shape with the matched value rotated.

## Direct prerequisite — lesson 153 (`any`; load-bearing)

Lesson 153 installed `Iterator::any`. Today's `position` reuses 153's
short-circuit polarity (first `true`) and the iterator-still-usable
mechanic. Probe 4 today is direct rotation of lesson 153 Probe 2 and
lesson 154 Probe 5 with method name `position` and matched value
shifted from `5` to `15` (specifically chosen so the index `5` and
the matched value `15` are *different* numbers — making the index-vs-
value distinction visible from a single output line).

## Direct prerequisite — lesson 152 (`reduce`; load-bearing)

Lesson 152 installed the first `Option<_>`-returning closure-driven
consumer with `None` for the empty case. Today's empty-case behavior
(Probe 5: `(1..1_u32).position(...) == None`) has the same shape as
lesson 152 Probe 2's empty `reduce` returning `None`. The rule "once
the return type is `Option<_>`, the empty case has its own slot" is
reused directly. Today's lesson body cites 152 once for this dual.

## Direct prerequisite — lesson 138 (`enumerate`; load-bearing)

Lesson 138 installed the `usize` iteration index starting at `0` and
incrementing by `1` per element. Today's index has the same identity:
it counts positions in the iterator's yielded sequence from `0`.

The numeric witness is Probe 6: closure called 6 times, returns
`Some(5)`. The index `5` is one less than the call count `6` because
indexing starts at `0` — exactly the same identity that `enumerate`
produces (the first yielded pair has index `0`). Probe 3 reinforces
the zero-base directly: first-element match returns `Some(0)`.

## Direct prerequisite — lesson 119 (`Option<T>` with `Some(T)` / `None`)

Lesson 119 installed `Option<T>` as a generic enum with two variants.
Today's return type `Option<usize>` is `Option<T>` with `T = usize`.

- `Some(5)` (Probes 1, 4, 6, 7): the `Some(_)` constructor wrapping a
  `usize` value `5`.
- `None` (Probes 2, 5): the bare `None` variant; today's annotation
  context (a method whose return type is fixed to `Option<usize>`)
  pins `T = usize` without needing the explicit annotation that
  lesson 119 Probe 2 required.
- `println!("{:?}", r)`: `Option<T>` derives `Debug`; lesson 119
  already used `{:?}` for a Debug-printed `Option`.

## Direct prerequisite — lesson 148 (`FnMut`; auto-impl rule)

Lesson 148 installed the three-trait family. Today's Probe 6 closure
captures `count` and mutates it; under 148's auto-impl rule, that
closure implements `FnMut` and `FnOnce` but not `Fn`. The bound
`FnMut` accepts it. Same direct empirical witness as lesson 153
Probe 5 / lesson 154 Probe 4. Today's Probes 1, 2, 3, 4, 5, 7 use
closures that capture nothing — they implement all three traits and
fit the bound trivially.

## Direct prerequisite — lesson 147 (parenthesized bound)

Lesson 147 installed `<F: FnMut(T) -> R>` segment grammar. Today's
bound has the same single-slot form `FnMut(Self::Item) -> bool` as
`any` and `all`. Cited but not re-derived.

## Direct prerequisite — lesson 144 (closure captures outer `let`)

Lesson 144 installed the capture mechanic. Today's Probe 6 captures
`count` and mutates it — same usage shape as lessons 153/154's count
probes.

## Direct prerequisite — lesson 142 (closure literal)

Lesson 142 installed `|param| body`. Today's `|x|` reuses the
single-parameter form unchanged.

## Direct prerequisite — lesson 132 (`Self::Item` slot)

Lesson 132 installed `type Item;` as the trait's associated-type
slot. For `Range<u32>`, `Self::Item = u32`. Probe 8 empirical: rustc
spells `FnMut(u32)`.

## Direct prerequisite — lesson 131 (`&mut self` + `let mut` rule)

Lesson 131 installed `fn next(&mut self) -> Option<Self::Item>` and
the borrow rule "binding must be `let mut`" via E0596 contrast.
Lessons 153/154 reused both. Today reuses both again on `position`:
- Probe 4 calls `it.next()` after `.position(...)` returns — direct
  empirical reuse.
- Probe 9 fires E0596 — same code shape as lessons 131/153/154 with
  method name rotated.

## Direct prerequisite — lessons 091 + 081 + 080 (`Range<u32>` source)

Lesson 091 grounds `Range<A>: Iterator` for `A: Step`. Lesson 081
installs `_u32` suffix. Lesson 080 installs `u32`. Today's
`(10..20_u32)`, `(1..10_u32)`, `(5..10_u32)`, `(1..1_u32)`, `(1..4_u32)`,
and the bare range `10..20_u32` (Probes 4, 9) all rely on these
lessons. The half-open semantics drives Probe 5's empty case.

## Direct prerequisite — lesson 077 (`usize` integer type)

Lesson 077 installed `usize` as the third typed integer (after `i32`
and `u32`), unsigned and architecture-dependent, named by the Book
specifically as the indexing type. Today's `Option<usize>` payload
type is exactly `usize`. Probe 7 type-pins the wrapper as
`Option<usize>` empirically.

## Direct prerequisite — lesson 023 (`+=` on a mut binding)

Probe 6's `count += 1` is `u32 += u32` per lesson 023, with `count`
declared `let mut count = 0_u32;`. Same shape as lessons 153/154
count probes.

## Direct prerequisite — lesson 013 (`==` on integers produces `bool`)

Lesson 013 installed comparison operators on integers. Today's
closure bodies `x == 15`, `x == 100`, `x == 5` are
`u32 == u32 -> bool` per lesson 013. The result type matches the
bound's return slot `-> bool`.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's
diagnostics:

- **E0277** (Probe 8): `expected a FnMut(u32) closure, found {integer}`.
  Same code as lessons 153/154 with method name rotated. Direct
  empirical witness for the single-parameter parens-bound shape on
  `position`. The `note: required by a bound in `position`` points
  at `library/core/src/iter/traits/iterator.rs:3112:4` — *after*
  lesson 154's `:2809:4` for `all` and lesson 153's `:2863:4` for
  `any`, matching the corpus order at `trait.Iterator.md:2557`
  (`all`), `:2599` (`any`), `:2763` (`position`).
- **E0596** (Probe 9): `cannot borrow `it` as mutable`. Same code as
  lessons 131/153/154 with method name rotated. Direct empirical
  witness that `position`'s receiver is `&mut self`.

No new error code today. The move is *signature-shape-only* relative
to lessons 153/154 except for the return-type rotation; rustc's
existing diagnostics suffice with method name rotated.

## Cited prereqs

- **Lesson 145**: `<F>` / `<P>` generic-function type-parameter slot.
- **Lesson 011**: `println!("{:?}", r)` and `println!("{:?} {:?}", ..., ...)`.
- **Lesson 005**: `let r = ...` binding for `position`'s return.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc + ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics, examples)

Verified by reading lines 152-154 (synopsis-box version) and 2761-2813
(full method entry).

### Lines 152-154 (synopsis-box version)

```text
fn position<P>(&mut self, predicate: P) -> Option<usize>
   where Self: Sized,
         P: FnMut(Self::Item) -> bool { ... }
```

`{ ... }` confirms `position` is a *provided* method. Note: the
synopsis box uses `<P>` as the type-parameter name, not `<F>` —
documentation convention ("`P` for predicate"); functionally
equivalent to `any`'s `<F>`.

### Line 2763 (full signature, main entry)

```text
fn position<P>(&mut self, predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool,
```

Direct corpus source for the lesson body's signature. Five
load-bearing facts:

- `<P>` — *one* type parameter (named `P` not `F`; cosmetic).
- `(&mut self, predicate: P)` — receiver `&mut self`, then *one*
  non-receiver parameter `predicate: P`.
- `-> Option<usize>` — return type is `Option<T>` (lesson 119) with
  `T = usize` (lesson 077). This is the only structural rotation
  relative to `any`/`all`.
- `where Self: Sized` — same bound as 149-154. Not centered.
- `P: FnMut(Self::Item) -> bool` — bound; identical shape to `any`'s
  and `all`'s. Single-parameter parens-bound, `bool` return.

### Line 2761 (stabilization)

```text
1.0.0 ·
```

Stabilization at 1.0.0; well below the local toolchain 1.95.0.

### Lines 2765-2773 (semantics)

```text
Searches for an element in an iterator, returning its index.

`position()` takes a closure that returns `true` or `false`. It applies
this closure to each element of the iterator, and if one of them
returns `true`, then `position()` returns `Some(index)`. If all of
them return `false`, it returns `None`.

`position()` is short-circuiting; in other words, it will stop
processing as soon as it finds a `true`.
```

Direct corpus source for four load-bearing claims:

- "Searches for an element in an iterator, returning its **index**" —
  the headline emphasis on *index* is the corpus's own wording for
  the index-vs-value distinction.
- "It applies this closure to each element of the iterator" — same
  per-element pattern as `any`/`all`. Probes 1, 6 empirical.
- "If one of them returns `true`, then `position()` returns
  `Some(index)`" — direct corpus source for the `Some(idx)` return.
  Probes 1, 3, 4, 6, 7 empirical.
- "If all of them return `false`, it returns `None`" — direct corpus
  source for the no-match case. Probe 2 empirical (closure called
  for every element of `1..10`, never returns `true`).
- "`position()` is short-circuiting ... it will stop processing as
  soon as it finds a `true`" — direct corpus source for the lesson's
  short-circuit-on-`true` rule. Probe 6 empirical (closure called
  exactly 6 times for `15` match in `10..20_u32`, not 10 times).

### Lines 2775-2785 (overflow + panics)

```text
##### Overflow Behavior

The method does no guarding against overflows, so if there are more
than `usize::MAX` non-matching elements, it either produces the wrong
result or panics. If overflow checks are enabled, a panic is
guaranteed.

##### Panics

This function might panic if the iterator has more than `usize::MAX`
non-matching elements.
```

Named in the lesson's *What To Ignore For Now* as a corpus-named
corner case; impractical to probe.

### Lines 2789-2797 (corpus example, basic)

```text
let a = [1, 2, 3];

assert_eq!(a.into_iter().position(|x| x == 2), Some(1));

assert_eq!(a.into_iter().position(|x| x == 5), None);
```

The corpus example uses `[1, 2, 3].into_iter()`; today's lesson
keeps `Range<u32>` (lessons 091 + 081) to avoid pulling in
`IntoIterator` and arrays — same source-choice discipline as
lessons 149-154. The corpus's `Some(1)` for matching `x == 2` is
the same shape as today's `Some(5)` for matching `x == 15`: index
counted from 0, matched value different from index.

The corpus's first assert (`Some(1)` for `x == 2` in `[1, 2, 3]`)
also makes the index-vs-value distinction concrete: index `1` is
the second element, whose value is `2`. They happen to be similar
because the array starts at `1`. Today's probe uses `(10..20_u32)`
specifically so the index (`5`) and the matched value (`15`) are
*clearly different numbers* — sharper version of the same point.

### Lines 2799-2813 (corpus example, short-circuit + reuse)

```text
Stopping at the first `true`:

let a = [1, 2, 3, 4];

let mut iter = a.into_iter();

assert_eq!(iter.position(|x| x >= 2), Some(1));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(3));

// The returned index depends on iterator state
assert_eq!(iter.position(|x| x == 4), Some(0));
```

Direct corpus source for the lesson body's "iterator survives the
call" probe. The corpus uses `iter.position(|x| x >= 2)` matching on
`2`; today's Probe 4 uses `it.position(|x| x == 15)` matching on
`15` — same shape, different value range. The corpus comment "we
can still use `iter`, as there are more elements" is the prose form
of Probe 4's `Some(16)` for `it.next()`.

The corpus's third assert (`iter.position(|x| x == 4) == Some(0)`)
demonstrates that the index is *relative to the iterator's current
state*: after the first `position` call left the iterator pointing
at `3`, the next `position` call walks `3, 4`, matches at `4`, and
the index is `0` because `4` was the first element the second call
saw. This is a stronger statement than today's lesson centers (the
lesson says "the index is the position in the iterator's yielded
sequence"). Today's Probe 4 stops short of this second-call
demonstration to keep the move atomic; the lesson's *What Changed*
fourth bullet stays one beat ahead but does not assert this.

## Source — `output/docs/rust/error_codes/E0596.md` (mutable borrow on non-mut binding)

Lessons 131, 047/048, 153, 154 already cited. Today reuses unchanged:
rustc cannot mutably borrow a binding declared without `mut`. Probe
9's diagnostic is the application of E0596 to the receiver of an
`&mut self` method call.

## Source — `output/docs/rust/error_codes/E0277.md` (trait bound not satisfied)

Lessons 146-154 already cited. Today reuses unchanged. Probe 8's
diagnostic spells `FnMut(u32)` — same single-parameter shape as
lessons 153/154 Probes.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Reused today: `Range<A>:
Iterator where A: Step`. The half-open semantics drives Probe 5's
empty case.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lessons 148-154 already cited. Reused today.

## Source — `output/docs/rust/std/option/enum.Option.md` (return-type wrapper)

Lesson 119 already established this. Reused today: `Option<T>` with
`T = usize` for today's return slot.

## Probe 1 — working `position` on `Range<u32>` (load-bearing)

Source: `observations/155-iterator-position.rs` (canonical shape;
local probe at `/tmp/eduratchet155/demo.rs`). Transcript: `PROBE 1`
block.

```rust
fn main() {
    let r = (10..20_u32).position(|x| x == 15);
    println!("{:?}", r);
}
```

Output: `Some(5)`. Compile-exit=0, run-exit=0. Five load-bearing
facts:

- The bound `P: FnMut(Self::Item) -> bool` accepts a closure literal
  `|x| x == 15` as the only non-receiver argument to `.position(...)`
  on `(10..20_u32)`.
- `Self::Item = u32` for `Range<u32>`; the closure parameter `x` is
  a `u32`. The closure body `x == 15` is `==` on two `u32` values
  producing `bool` per lesson 013.
- The element `15` is at index `5` in the sequence `10, 11, 12, 13,
  14, 15, 16, 17, 18, 19` (zero-based). The closure returns `false`
  for indices `0..=4` (values `10..=14`), `true` at index `5` (value
  `15`); `position` short-circuits and returns `Some(5)`.
- The return type is `Option<usize>` (lesson 119 with `T = usize`).
  `Some(5)` is the `Some(_)` constructor wrapping a `usize` value.
  `println!("{:?}", r)` produces the Debug form `Some(5)`.
- **Index-vs-value disambiguation**: the matched element value is
  `15`, but the index returned is `5`. Different numbers in the
  same output. This is the centered new fact for the lesson.

## Probe 2 — no match returns `None`

Source: `no_match.rs`. Transcript: `PROBE 2` block.

```rust
fn main() {
    let r = (1..10_u32).position(|x| x == 100);
    println!("{:?}", r);
}
```

Output: `None`. Compile-exit=0, run-exit=0. The closure runs once
for every element `1..=9` (nine times), `false` every time;
`position` returns `None`. Direct corpus correspondence:
`trait.Iterator.md:2769-2770` "If all of them return `false`, it
returns `None`."

## Probe 3 — first-element match returns `Some(0)`

Source: `first_element.rs`. Transcript: `PROBE 3` block.

```rust
fn main() {
    let r = (5..10_u32).position(|x| x == 5);
    println!("{:?}", r);
}
```

Output: `Some(0)`. Compile-exit=0, run-exit=0. Direct empirical
witness for the zero-based-index rule: when the first element
matches, the returned index is `0`. The element value (`5`) and the
returned index (`0`) are again different numbers — sharpens the
centered point.

## Probe 4 — `&mut self` reusability + index/value disambiguation (load-bearing)

Source: `reusable.rs`. Transcript: `PROBE 4` block.

```rust
fn main() {
    let mut it = 10..20_u32;
    let r = it.position(|x| x == 15);
    let n = it.next();
    println!("{:?} {:?}", r, n);
}
```

Output: `Some(5) Some(16)`. Compile-exit=0, run-exit=0. Three load-
bearing facts witnessed in a single execution (same shape as lessons
153 Probe 2 / 154 Probe 5, with method name rotated and matched
value shifted to `15` so the index `5` and the value `15` are
distinguishable):

- After `it.position(...)` returns, the binding `it` is *still
  usable* — the next line successfully calls `it.next()`. Direct
  empirical reuse of the "iterator survives the call" mechanic.
- The iterator's next yielded value is `Some(16)`, *not* `Some(10)`
  or `None`. `position` advanced through `10, 11, 12, 13, 14, 15`
  while testing, matched on `15`, and stopped — short-circuit. The
  iterator is consumed up to and including the matching element,
  but no further. Same position-after-short-circuit shape as lesson
  153 Probe 2.
- **Index/value disambiguation in a single output line.** Three
  distinct numbers visible at once: `5` (returned index), `15`
  (matched element value, implicit), `16` (next element value).
  `5` is a position in the yielded sequence; `15` is the value at
  that position; `16` is the value at the next position. They live
  on different axes. This is the load-bearing semantic for the
  lesson — chosen specifically because the orchestrator's brief
  identified the same point as load-bearing.

## Probe 5 — empty iterator returns `None`

Source: `empty.rs`. Transcript: `PROBE 5` block.

```rust
fn main() {
    let r = (1..1_u32).position(|x| x == 5);
    println!("{:?}", r);
}
```

Output: `None`. Compile-exit=0, run-exit=0. Three load-bearing
facts:

- `(1..1_u32)` is a half-open `Range<u32>` whose lower bound equals
  upper bound — yields no elements (lesson 091 + 081).
- With no elements to apply the closure to, `position` returns
  `None`. Closure was never called.
- Empty-`position` returns `None`, contrasting with empty-`any`
  returning plain `false` (lesson 153 Probe 3) and empty-`all`
  returning plain `true` (lesson 154 Probe 3). Once the return type
  is `Option<_>`, the empty case has its own slot — same dual as
  lesson 152's empty `reduce` returning `None`.

## Probe 6 — closure-call count witnesses zero-based index (load-bearing)

Source: `count_calls.rs`. Transcript: `PROBE 6` block.

```rust
fn main() {
    let mut count = 0_u32;
    let r = (10..20_u32).position(|x| { count += 1; x == 15 });
    println!("{:?} {}", r, count);
}
```

Output: `Some(5) 6`. Compile-exit=0, run-exit=0. Five load-bearing
facts:

- The closure ran exactly *six* times — for `x = 10, 11, 12, 13, 14,
  15` — and stopped. The closure body `count += 1` ran on each call;
  final `count = 6`. Direct empirical witness for short-circuit on
  `true`: `position` stops calling the closure after the first
  `true`.
- For `x = 10, 11, 12, 13, 14` the body `x == 15` returned `false`.
  On the sixth call (`x = 15`), `15 == 15` returned `true`, and
  `position` stopped.
- The returned index `Some(5)` is **one less than the call count
  `6`**. Direct numeric witness for the zero-based-index identity:
  the first call sat at index `0`, the second at index `1`, ..., the
  sixth at index `5`. Same `usize` counter shape as lesson 138's
  `enumerate`.
- The braced closure body `{ count += 1; x == 15 }` is a regular
  Rust block: `count += 1;` is a statement, `x == 15` is the
  trailing expression that becomes the closure's return value.
  Same shape as lessons 153 Probe 5 / 154 Probe 4 with the closure
  body's predicate rotated.
- The closure captures `count` from the enclosing scope (lesson 144)
  and *mutates* it via `count += 1` (lesson 023). Under lesson 148's
  auto-impl rule, this closure implements `FnMut` and `FnOnce` but
  not `Fn`. The bound `FnMut` accepts this shape.

The output `6` for `count` (not `10` for the full range, not `5` for
the count of `false` returns) is the direct numeric witness: the
closure was called for each element starting from the first, in
order, and stopped on the first `true` return — which happened on
the sixth call.

## Probe 7 — type-pin (return type is exactly `Option<usize>`)

Source: `type_pin.rs`. Transcript: `PROBE 7` block.

```rust
fn main() {
    let r: Option<usize> = (10..20_u32).position(|x| x == 15);
    println!("{:?}", r);
}
```

Output: `Some(5)`. Compile-exit=0, run-exit=0. The annotation
`Option<usize>` matches the inferred type from
`(10..20_u32).position(...)`. Empirical confirmation that the return
type is exactly `Option<usize>`. Composes lesson 119's `Option<T>`
with `T = usize` (lesson 077). First time `Option<usize>` appears
as the return type of a closure-driven consumer in the run.

## Probe 8 — non-closure argument fires E0277 (rotation only)

Source: `non_closure_arg.rs`. Transcript: `PROBE 8` block.

```rust
fn main() {
    let r = (1..4_u32).position(7);
    println!("{:?}", r);
}
```

Output (compile-exit=1):

```text
error[E0277]: expected a `FnMut(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:2:33
  |
2 |     let r = (1..4_u32).position(7);
  |                        -------- ^ expected an `FnMut(u32)` closure, found `{integer}`
  |                        |
  |                        required by a bound introduced by this call
  |
  = help: the trait `FnMut(u32)` is not implemented for `{integer}`
note: required by a bound in `position`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:3112:4
```

Three load-bearing facts (same as lessons 153 Probe 6 / 154 Probe 7
with method name `position`):

- rustc spells the expected closure type as `FnMut(u32)` — *single*
  parameter slot, no comma. Identical bound shape to `any` and
  `all`. Despite the corpus naming the type parameter `<P>` instead
  of `<F>`, rustc still spells the trait as `FnMut(u32)`,
  confirming the type-parameter name is purely cosmetic.
- `Self::Item` resolved to `u32` from the `Range<u32>` source.
- The `note: required by a bound in `position`` line points at
  `library/core/src/iter/traits/iterator.rs:3112:4` — *after*
  lesson 153's `:2863:4` for `any` and lesson 154's `:2809:4` for
  `all`, matching the corpus order at `trait.Iterator.md:2557`
  (`all`), `:2599` (`any`), `:2763` (`position`).

This probe is included only to confirm the bound shape matches
`any`/`all` exactly. Lessons 153/154 already installed the
diagnostic; today's witness is rotation-only.

## Probe 9 — `&mut self` receiver requires `let mut` (E0596; rotation only)

Source: `needs_mut.rs`. Transcript: `PROBE 9` block.

```rust
fn main() {
    let it = 10..20_u32;
    let _r = it.position(|x| x == 15);
}
```

Output (compile-exit=1):

```text
error[E0596]: cannot borrow `it` as mutable, as it is not declared as mutable
 --> needs_mut.rs:3:14
  |
3 |     let _r = it.position(|x| x == 15);
  |              ^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut it = 10..20_u32;
  |         +++
```

Same shape as lessons 131/153/154 with method name rotated. Direct
empirical witness that `position`'s receiver is `&mut self`.
Included only for rotation-confirmation.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn position<P>(&mut self, predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool` | `output/docs/rust/std/iter/trait.Iterator.md:2763` verbatim; synopsis at `:152-154` |
| Type-parameter named `<P>` not `<F>` (cosmetic, "P for predicate") | Same line; Probe 8 empirical (rustc still spells trait `FnMut(u32)`, confirming the slot is purely cosmetic) |
| Receiver `&mut self`; iterator NOT consumed | Same line; Probe 4 empirical (iterator still usable); Probe 9 empirical (E0596 requires `let mut`) |
| One non-receiver argument (`predicate: P`) | Same line |
| Returns `Option<usize>` | Same line; Probes 1, 4, 6, 7 empirical (`Some(_)`); Probes 2, 5 empirical (`None`); Probe 7 type-pin |
| Closure bound is `FnMut(Self::Item) -> bool` (single parameter slot) | Same line; Probe 8 empirical (rustc spells `FnMut(u32)`) |
| `Self::Item = u32` for `Range<u32>` | Lesson 091, 080, 132; Probe 8 empirical |
| Short-circuit: closure stops on first `true` | `trait.Iterator.md:2772-2773` verbatim; Probe 6 empirical (closure called exactly 6 times for `x == 15` match in `10..20_u32`); Probe 4 empirical (iterator position is `Some(16)`, not exhausted) |
| `Some(idx)` carries zero-based index of first match | `trait.Iterator.md:2767-2769` verbatim; Probes 1, 3, 4, 6 empirical |
| Index is position in yielded sequence, NOT element value | Probe 1 empirical (`Some(5)` for matching value `15`); Probe 4 empirical (three distinct numbers `5`, `15`, `16` in same output line); Probe 6 empirical (call count `6` − 1 = index `5`) |
| Index counted from `0` (zero-based) | Probe 3 empirical (first-element match returns `Some(0)`); Probe 6 empirical (call count `6` − 1 = index `5`); lesson 138 enumerate identity |
| `None` when closure never returns `true` (no match) | `trait.Iterator.md:2769-2770` verbatim; Probe 2 empirical |
| `None` when iterator empty (closure never called) | Probe 5 empirical; lesson 152 dual on empty `reduce` |
| Iterator position lands just past matching element | Probe 4 empirical (`Some(16)` after matching on `15`); corpus example `trait.Iterator.md:2799-2813` (matching on `2 >= 2` leaves `3` next) |
| Closure body `x == 15` is `u32 == u32 -> bool` | Lesson 013; Probe 1 empirical |
| Probe 6 closure body uses block + capture + mutation | Lesson 144 (capture); lesson 023 (`+=`); lesson 148 (FnMut auto-impl); Probe 6 empirical |
| Without `let mut`, `it.position(...)` fires E0596 | Lesson 131 + 153 + 154; Probe 9 empirical |
| Non-closure argument fires E0277 with `FnMut(u32)` spelling | Probe 8 empirical |
| Stabilized at 1.0.0 | `trait.Iterator.md:2761` verbatim; toolchain is 1.95.0 |
| `position` is sibling of `any`/`all` with same closure shape, `Option<usize>` return | Lessons 153, 154 (`any`/`all` evidence); `trait.Iterator.md:2763` (`position`) — identical closure-bound shape, return slot rotated |
| Empty-`position` is dual to empty-`reduce`'s `None` (both `Option<_>`-returning) | Lesson 152 evidence; `trait.Iterator.md:2769-2770` |

## Older supporting lessons (named only)

- 154-iterator-all — direct sibling. Same signature shape; today
  rotates return only. Today's structural anchor (with 153).
- 153-iterator-any — direct sibling. Same short-circuit polarity
  (first `true`) as today.
- 152-iterator-reduce — sibling closure-driven consumer; today's
  empty-case `None` has the same shape as 152's.
- 151-iterator-fold, 150-iterator-map, 149-iterator-for-each —
  closure-driven Iterator family.
- 148-fn-fnmut-fnonce-distinction — `FnMut` choice + auto-impl rule.
- 147-fn-trait-parenthesized-bound — parens-segment + arrow-segment.
- 145-generic-function-type-parameter — `<F>`/`<P>` slot.
- 144-closure-captures-outer-let — capture mechanic.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 138-iterator-enumerate — `usize` index identity (zero-based,
  increments per element).
- 132-iterator-trait-declaration — `Self::Item` slot.
- 131-iterator-next-call — `&mut self` on `next()`; E0596 contrast.
- 119-option-some-none — `Option<T>` with `Some(T)` / `None`.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens-rule;
  half-open semantics.
- 081-integer-literal-forms — `_u32` suffix.
- 080-integer-type-family — `u32`.
- 077-array-indexing-and-usize — `usize` integer type.
- 023-compound-add-assign — `+=` on a `mut` integer binding.
- 013-comparison-operators — `==` on `u32` produces `bool`.
- 011-println-positional-args — `println!`.
- 005-let-binding — `let x = ...`.
- 003-read-rustc-diagnostic — four-part diagnostic map.
- 002-fn-main-entry-point — `fn main()`.
- 001-rustc-compile-and-run — `rustc + ./name`.

## Deliberate scope discipline

The orchestrator's brief named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. `rposition` — sibling with same shape but starts from the right;
   gates on `DoubleEndedIterator` trait sub-arc.
2. `find` — predicate consumer with `&Self::Item` parameter and
   `Option<Self::Item>` return; gates on deref-read.
3. `find_map` — `FnMut(Self::Item) -> Option<B>` bound and
   `Option<B>` return; later move.
4. The "index" vs. "position" naming — both names refer to the same
   `usize` counter; detailed treatment deferred.
5. Methods on `Option<usize>` — `.unwrap()`, `.map()`, `if let`,
   etc.; lesson 119 already deferred these.
6. `usize::MAX` overflow — corpus-named at `:2775-2785`; impractical
   to probe.
7. `try_for_each`, `try_fold` — short-circuit-with-`?` variants;
   gated on the `Try` trait sub-arc.

## Mechanics deliberately *not* smuggled

The orchestrator's reminder list called out specific smuggling risks.
Today's discipline check (carries from lessons 149-154 unchanged):

- **No `Vec`** — source is `Range<u32>` for every probe.
- **No `.iter()` or `.into_iter()`** — bare range only.
- **No `IntoIterator`** — gated on its own sub-arc.
- **No `&Self::Item`** — `position`'s closure parameter is
  `Self::Item` by *value*. Probe 8 empirical: rustc spells
  `FnMut(u32)`.
- **No deref-read `*x`** — closure bodies are `x == 15`, `x == 100`,
  `x == 5`; plain owned `==` per lesson 013.
- **No `as` casting** — `==` on `u32 == u32` produces `bool`
  directly.
- **No `Box<dyn Fn>`, no `impl Fn`** — bound is named
  `FnMut(Self::Item) -> bool`, consumed by the generic `<P>` slot.
- **No `move` keyword** — Probes 1, 2, 3, 4, 5, 7 capture nothing;
  Probe 6 captures by mutable reference (the default mode for
  `count += 1`).
- **No `match` on `Option<usize>`** — lesson uses
  `println!("{:?}", r)` only. Lesson 119 introduced `match` on
  `Option`, but today does not center it; the audience can read
  `Some(5)` / `None` from Debug output.
- **No `.unwrap()` / `.map()` / `if let` on the returned
  `Option<usize>`** — lesson 119 already deferred these and they
  remain deferred.

## Run-context handoff

After this lesson lands, the orchestrator's options for lesson 156
include:

- `find` — predicate consumer with `&Self::Item` parameter. First
  place the audience meets the *reference closure parameter* shape;
  introduces the deref-read sub-arc.
- `find_map` — `Option<B>`-returning closure body composed with
  short-circuit; new closure-bound shape `FnMut(Self::Item) ->
  Option<B>`.
- `rposition` — same as today, starts from the right; gates on
  `DoubleEndedIterator`.
- `filter` — first lazy adapter with predicate closure;
  `FnMut(&Self::Item) -> bool`; surfaces double-reference issue.
- `inspect` — lazy adapter with `FnMut(&Self::Item)`, no return;
  lighter intro to the reference-parameter shape.

The audit's predicate-consumer arc continues for at least 2 more
moves; today closes the by-value-predicate-consumer triplet (`any`,
`all`, `position`) and opens the door to the by-reference-predicate
arc (`find`, `find_map`, `filter`, `inspect`).
