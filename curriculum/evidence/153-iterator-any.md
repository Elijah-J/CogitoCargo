# Evidence — Lesson 153: `Iterator::any` (first closure-driven method with `&mut self` receiver; first short-circuit semantics)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/153-iterator-any.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/153-iterator-any.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/153-iterator-any.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet153/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-152.

## Run context — fifth closure-driven Iterator method (audit §4.4.1, §8)

Lessons 149-152 installed the four-method consuming-`self`
closure-driven family (`for_each`, `map`, `fold`, `reduce`). The
audit's predicate-consumer family (`any`, `all`, `position`, `find`,
`find_map`) is the next natural arc and was named in lesson 152's
run-context handoff. `any` is opened first in this arc because:

- **Simplest return type.** `bool` (lesson 013), no `Option<_>` or
  `Self::Item` payload. Lesson 152 already installed `Option<_>` as
  a return shape; today's lesson keeps the by-value closure parameter
  shape unchanged from 149-152 and rotates only the receiver shape
  and the return shape.
- **Empty case unambiguous.** Lesson 152's `reduce` returns `None` on
  empty so the wrapper had a value for the no-input case; today's
  `any` returns `false` on empty (corpus, `trait.Iterator.md:2612`).
  No new wrapper is introduced.
- **Two structurally novel facts under one move.** (1) Receiver
  `&mut self` — first closure-driven method whose iterator is *not*
  consumed; (2) short-circuit — first place a closure-driven method
  stops calling the closure mid-iteration. Both facts are directly
  witnessed by Probe 2 (`true Some(6)`) in a single execution.

Audit `iterator-api-coverage.md:103` rows `any` as "consumer,
predicate; short-circuits", with `&mut self` receiver and 1.0.0
stabilization.

## Direct prerequisite — lesson 152 (`reduce`; sibling closure-driven consumer)

Lesson 152 installed `Iterator::reduce` with signature
`fn reduce<F>(self, f: F) -> Option<Self::Item> where F: FnMut(Self::Item, Self::Item) -> Self::Item`.
Three sub-claims load-bearing today:

- The closure-driven Iterator method *family* shape: a single
  non-receiver argument (the closure), bound is `FnMut(...)`. Today
  reuses both unchanged.
- The receiver-rotation contrast: 152 consumes `self`; today takes
  `&mut self`. Probes 2 and 7 today empirically witness this rotation.
- The return-rotation contrast: 152 returns `Option<Self::Item>`;
  today returns `bool`. Probe 8 type-pins.

## Direct prerequisite — lesson 151 (`fold`; multi-parameter parens-bound)

Lesson 151 installed the two-parameter parens-bound segment
`FnMut(_, _) -> _`. Today's bound `FnMut(Self::Item) -> bool` is the
*single-parameter* form — no comma inside the parens. Probe 6
empirical: rustc spells `FnMut(u32)` (no comma), confirming the
single-slot resolution.

## Direct prerequisite — lessons 150 + 149 (closure-driven Iterator family)

Lesson 150 installed `map` with `<B, F>` and `-> B` arrow segment;
lesson 149 installed `for_each` with `()` return. Today is the fifth
closure-driven member; cited but not re-quoted.

## Direct prerequisite — lesson 148 (Fn / FnMut / FnOnce)

Today's bound is `FnMut(Self::Item) -> bool`. Lesson 148 installed
the three-trait family with the auto-implementation rule. Today's
working closures `|x| x == 5`, `|x| x == 100` capture nothing; under
148's rule they implement `Fn` (and so all three traits). Probe 5's
closure `|x| { count += 1; x == 3 }` mutates a captured outer `mut`
binding; under 148's rule it implements `FnMut` and `FnOnce` but
*not* `Fn`. The bound `FnMut` accepts both shapes — direct empirical
witness that today's closures fit the bound.

## Direct prerequisite — lesson 147 (parenthesized bound)

Lesson 147 installed `<F: FnMut(T) -> R>` with the parens segment
and the `-> R` arrow segment. Today's `FnMut(Self::Item) -> bool` is
the same shape. The single-slot inside the parens is structurally
shown today.

## Direct prerequisite — lesson 144 (closure captures outer `let`)

Lesson 144 installed the capture mechanic: a closure can reference a
name from the enclosing scope. Today's Probe 5 captures `count` and
mutates it via `count += 1`. Lesson 144's E0434 contrast — `fn` items
*cannot* capture — does not fire today; we are only using the
positive direction.

## Direct prerequisite — lesson 142 (closure literal)

Lesson 142 installed `|param| body`. Today's `|x|` reuses the
single-parameter form unchanged.

## Direct prerequisite — lesson 132 (`Self::Item` slot)

Lesson 132 installed `type Item;` as the trait's associated-type
slot. Today's bound has `Self::Item` in the (single) parameter slot.
For `Range<u32>`, `Self::Item = u32`. Probe 6 empirical: rustc
spells `FnMut(u32)`.

## Direct prerequisite — lesson 131 (`&mut self` on `next()`; structural rule reused for `any`)

Lesson 131 installed `fn next(&mut self) -> Option<Self::Item>` and
the borrow rule "binding must be `let mut`" via an E0596 contrast
probe. Today reuses both:

- Probe 2 calls `it.next()` *after* `.any(...)` returns — direct
  empirical reuse of lesson 131's mechanic (call `.next()` on a
  `let mut` binding to advance the iterator one step).
- Probe 7 fires E0596 with rustc's `cannot borrow `it` as mutable`
  payload — same code shape as lesson 131's contrast, with the
  method name rotated from `next` to `any`.

The `&mut self` shape on `next()` is the structural anchor for
today's lesson; the only thing that changes is that a *closure-driven*
method now also exhibits this shape.

## Direct prerequisite — lessons 091 + 081 + 080 (`Range<u32>` source)

Lesson 091 grounds `Range<A>: Iterator` for `A: Step` and the
parens-rule for method calls on a range value. Lesson 081 installs
`_u32` suffix. Lesson 080 installs `u32`. Today's `(1..10_u32)`,
`(1..1_u32)`, `(1..5_u32)`, `(1..4_u32)`, `(1..6_u32)` and the bare
range `1..10_u32` (Probes 2, 7) all rely on these lessons. The
half-open semantics (`1..1` yields nothing) drives Probe 3's `false`.

## Direct prerequisite — lesson 013 (`==` on integers produces `bool`)

Lesson 013 installed `==` as a binary operator on the integer family
producing a `bool`. Today's closure body `x == 5` (and `x == 100`,
`x == 3`) is `u32 == u32 -> bool` per lesson 013. The result type
matches the bound's return slot `-> bool`.

## Direct prerequisite — lesson 023 (`+=` on a mut binding)

Lesson 023 installed `n += value;` on a `mut` integer binding. Probe
5's `count += 1` is `u32 += u32` per lesson 023, with `count`
declared `let mut count = 0_u32;`. Same shape as lesson 149's Probe 1
which used `sum += x` inside a `for_each` closure body.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's
diagnostics:

- **E0277** (Probe 6): "expected a `FnMut(u32)` closure, found
  `{integer}`". Same code as lessons 146-152. Today's load-bearing
  payload is the inline `FnMut(u32)` spelling — direct empirical
  witness for the *single-parameter* parens-bound shape (no comma,
  no second slot, contrast with lesson 151's `FnMut(u32, u32)`).
- **E0596** (Probe 7): "cannot borrow `it` as mutable, as it is not
  declared as mutable". Same code as lessons 131, 047/048's mut
  contrast probes. Today's load-bearing payload is structural: rustc
  reports the borrow error at the `it` token at the start of
  `it.any(|x| x == 5)`, confirming `any` mutably borrows its
  receiver.

No new error codes today. The Move-section structural extension is
*shape-only*; rustc's existing diagnostics suffice.

## Cited prereqs

- **Lesson 145**: `<F>` generic-function type-parameter slot.
- **Lesson 143**: unannotated closure parameter `|x|` form.
- **Lesson 011**: `println!("{} {:?}", ..., ...)`.
- **Lesson 005**: `let r = ...` binding for `any`'s return.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc + ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics, examples)

Verified by reading lines 133-138 (synopsis-box) and 2597-2637 (full
method entry).

### Lines 136-138 (synopsis-box version)

```text
fn any<F>(&mut self, f: F) -> bool
   where Self: Sized,
         F: FnMut(Self::Item) -> bool { ... }
```

`{ ... }` confirms `any` is a *provided* method.

### Line 2599 (full signature, main entry)

```text
fn any<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool,
```

Direct corpus source for the lesson body's signature. Five
load-bearing facts:

- `<F>` — *one* type parameter.
- `(&mut self, f: F)` — receiver `&mut self` (mutable borrow), then
  *one* non-receiver parameter `f: F`.
- `-> bool` — return type is the primitive `bool`.
- `where Self: Sized` — same bound as 149-152. Not centered.
- `F: FnMut(Self::Item) -> bool` — bound; today centers the
  single-parameter parens-bound shape with `bool` return.

### Line 2597 (stabilization)

```text
1.0.0 ·
```

Stabilization at 1.0.0; well below the local toolchain 1.95.0.

### Lines 2601-2612 (semantics)

```text
Tests if any element of the iterator matches a predicate.

`any()` takes a closure that returns `true` or `false`. It applies
this closure to each element of the iterator, and if any of them return
`true`, then so does `any()`. If they all return `false`, it
returns `false`.

`any()` is short-circuiting; in other words, it will stop processing
as soon as it finds a `true`, given that no matter what else happens,
the result will also be `true`.

An empty iterator returns `false`.
```

Direct corpus source for four load-bearing claims:

- "It applies this closure to each element of the iterator" — closure
  is called on elements in source order until it returns `true` or
  the iterator is exhausted. Probes 1, 4, 5 empirical.
- "If any of them return `true`, then so does `any()`" — non-empty
  match returns `true`. Probe 1 (`true`), Probe 5 (`true`).
- "If they all return `false`, it returns `false`" — non-empty no-match
  returns `false`. Probe 4 empirical.
- "`any()` is short-circuiting ... it will stop processing as soon
  as it finds a `true`" — direct corpus source for the lesson's
  centered short-circuit fact. Probe 5 empirical (closure called
  exactly 3 times for matching `x == 3`, not 9 times).
- "An empty iterator returns `false`" — direct corpus source for
  Probe 3's empty-case output.

### Lines 2618-2624 (corpus example, basic)

```text
let a = [1, 2, 3];

assert!(a.into_iter().any(|x| x > 0));

assert!(!a.into_iter().any(|x| x > 5));
```

The corpus example uses `[1, 2, 3].into_iter()` (array source via
`IntoIterator`); today's lesson keeps `Range<u32>` (lesson 091 + 081)
to avoid pulling in `IntoIterator` and arrays. The structural witness
is the same: predicate-with-match returns `true`, predicate-without-match
returns `false`.

### Lines 2626-2637 (corpus example, short-circuit + reuse)

```text
Stopping at the first `true`:

let a = [1, 2, 3];

let mut iter = a.into_iter();

assert!(iter.any(|x| x != 2));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(2));
```

Direct corpus source for the lesson body's "iterator survives the
call" probe. The corpus uses `iter.any(|x| x != 2)`: `1 != 2` returns
`true` on the *first* call, so the iterator stops just after `1`,
leaving `2` as the next yielded value. Today's Probe 2 uses a
similar shape with `Range<u32>` source: `it.any(|x| x == 5)` matches
on `5`, leaving `6` as the next yielded value. The corpus comment
"we can still use `iter`, as there are more elements" is the prose
form of Probe 2's `Some(6)` output.

## Source — `output/docs/rust/error_codes/E0596.md` (mutable borrow on non-mut binding)

Lessons 131, 047/048 already cited. Today reuses unchanged: rustc
cannot mutably borrow a binding declared without `mut`. Probe 7's
diagnostic is the application of E0596 to the receiver of an
`&mut self` method call, with the `help:` line proposing
`let mut it = 1..10_u32;` at the binding site with `+++` markers.

## Source — `output/docs/rust/error_codes/E0277.md` (trait bound not satisfied)

Lessons 146-152 already cited. Today reuses unchanged: when an
argument fails to meet a trait bound, rustc reports E0277 with the
expected-trait spelling. Probe 6's diagnostic spells `FnMut(u32)` —
direct empirical witness for the single-parameter bound shape.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Reused today: `Range<A>:
Iterator where A: Step`. `1..10_u32`, `1..5_u32`, `1..1_u32`,
`1..4_u32`, `1..6_u32` are all `Range<u32>` values; `Iterator::Item =
u32`. The half-open semantics drives Probe 3's empty-case `false`.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lessons 148-152 already cited. Reused today: capture-nothing closures
implement `Fn` (and so all three traits); capture-and-mutate closures
implement `FnMut` and `FnOnce` but not `Fn`. Today's working closures
fit the `FnMut` bound under both halves of the rule.

## Probe 1 — working `any` on `Range<u32>`

Source: `observations/153-iterator-any.rs` (canonical shape; local
probe at `/tmp/eduratchet153/demo.rs`). Transcript: `PROBE 1` block.

```rust
fn main() {
    let r = (1..10_u32).any(|x| x == 5);
    println!("{}", r);
}
```

Output: `true`. Compile-exit=0, run-exit=0. Five load-bearing facts:

- The bound `F: FnMut(Self::Item) -> bool` accepts a closure literal
  `|x| x == 5` as the only non-receiver argument to `.any(...)` on
  `(1..10_u32)`.
- `Self::Item = u32` for `Range<u32>`; the closure parameter `x` is a
  `u32`. The closure body `x == 5` is `==` on two `u32` values
  producing `bool` per lesson 013.
- `5` is one of the elements yielded by `(1..10_u32)` (which yields
  `1, 2, 3, 4, 5, 6, 7, 8, 9`). The closure returns `true` on the
  fifth call.
- `any` returns `bool`; the inferred type for `let r = ...` is `bool`.
- `println!("{}", r)` produces the Display form `true`.

## Probe 2 — `&mut self` reusability witness (load-bearing)

Source: `reusable.rs`. Transcript: `PROBE 2` block.

```rust
fn main() {
    let mut it = 1..10_u32;
    let found = it.any(|x| x == 5);
    let next = it.next();
    println!("{} {:?}", found, next);
}
```

Output: `true Some(6)`. Compile-exit=0, run-exit=0. Four load-bearing
facts witnessed in a single execution:

- After `it.any(...)` returns, the binding `it` is *still usable* —
  the next line successfully calls `it.next()`. Direct empirical
  witness that `any` does NOT consume `self`. With any of lessons
  149-152's methods that line would have fired E0382.
- The iterator's next yielded value is `Some(6)`, *not* `Some(1)`.
  `any` advanced the iterator past elements `1, 2, 3, 4, 5` while
  searching. Direct empirical witness that `any` *does* mutably
  modify the iterator (consistent with `&mut self`, the receiver's
  shape).
- The iterator's next yielded value is `Some(6)`, *not* `None`. `any`
  stopped advancing as soon as the closure returned `true` — short-
  circuit. Direct empirical witness that the iterator is consumed
  *up to and including* the matching element, but no further.
- Both facts (still-bound, short-circuit-position) emerge from the
  same probe in the same execution. This is the structurally most
  informative probe in the lesson and is featured in the lesson body
  rather than only in the appendix.

## Probe 3 — empty iterator returns `false`

Source: `empty.rs`. Transcript: `PROBE 3` block.

```rust
fn main() {
    let r = (1..1_u32).any(|x| x == 5);
    println!("{}", r);
}
```

Output: `false`. Compile-exit=0, run-exit=0. Three load-bearing facts:

- `(1..1_u32)` is a half-open `Range<u32>` whose lower bound equals
  upper bound — yields no elements. Lesson 091 + the half-open rule.
- With no elements to apply the closure to, `any` returns `false`.
  Direct corpus correspondence: `trait.Iterator.md:2612` "An empty
  iterator returns `false`."
- The closure was *never called*. Empirically obvious from the lack
  of side effects (no `println` inside the closure body); supported
  by the corpus prose at `trait.Iterator.md:2603-2604` ("It applies
  this closure to each element").

This probe is the dual of lesson 152 Probe 2's empty-`reduce` case.
There the empty-case output was `None` (with the closure also never
called); here the empty-case output is `false`. The two siblings
together cover the empty-case design space for predicate consumers
*without* introducing the wrapper-shape variation that `reduce` and
`any` represent.

## Probe 4 — no match: closure called every element, returns `false`

Source: `no_match.rs`. Transcript: `PROBE 4` block.

```rust
fn main() {
    let r = (1..5_u32).any(|x| x == 100);
    println!("{}", r);
}
```

Output: `false`. Compile-exit=0, run-exit=0. Two load-bearing facts:

- `(1..5_u32)` yields `1, 2, 3, 4` — four elements. The closure
  returns `false` for each. `any` walks all four, finds no match,
  returns `false`. Direct corpus correspondence:
  `trait.Iterator.md:2604-2606` "If they all return `false`, it
  returns `false`."
- This probe makes short-circuit visible by *contrast*: in Probe 5
  with a match, the closure runs only 3 times before stopping; here
  with no match, the closure runs the full length. The contrast pair
  Probe 4 + Probe 5 is the empirical witness for the short-circuit
  rule.

## Probe 5 — closure-call count (short-circuit witness, captures `count`)

Source: `count_calls.rs`. Transcript: `PROBE 5` block.

```rust
fn main() {
    let mut count = 0_u32;
    let r = (1..10_u32).any(|x| { count += 1; x == 3 });
    println!("{} {}", r, count);
}
```

Output: `true 3`. Compile-exit=0, run-exit=0. Four load-bearing facts:

- The closure ran exactly *three* times — for `x = 1`, `x = 2`,
  `x = 3` — and stopped. The closure body `count += 1` ran on each
  call; final `count = 3`. Direct empirical witness for the
  short-circuit rule: `any` stops calling the closure after the
  first `true`.
- The braced closure body `{ count += 1; x == 3 }` is a regular Rust
  block: `count += 1;` is a statement (note the trailing `;`),
  `x == 3` is the trailing expression with no `;` and becomes the
  closure's return value (a `bool`). The block's final value is
  what the closure returns, per Rust's expression-block rule.
- The closure captures `count` from the enclosing scope (lesson 144)
  and *mutates* it via `count += 1` (lesson 023). Under lesson 148's
  auto-impl rule a closure that mutates a captured binding implements
  `FnMut` and `FnOnce` but not `Fn`. The bound on `any` is `FnMut`,
  which accepts this closure shape — direct empirical witness for
  the bound choice.
- The output `3` (not `9` for the full `1..10_u32` range, not `5`
  for the position of element `5`) is the direct numeric witness
  for short-circuit: the closure was called for each element starting
  from the first, in order, and stopped on the first `true` return.

## Probe 6 — non-closure argument fires E0277 with `FnMut(u32)` spelling

Source: `non_closure_arg.rs`. Transcript: `PROBE 6` block.

```rust
fn main() {
    let r = (1..4_u32).any(7);
    println!("{}", r);
}
```

Output (compile-exit=1):

```text
error[E0277]: expected a `FnMut(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:2:28
  |
2 |     let r = (1..4_u32).any(7);
  |                        --- ^ expected an `FnMut(u32)` closure, found `{integer}`
  |                        |
  |                        required by a bound introduced by this call
  |
  = help: the trait `FnMut(u32)` is not implemented for `{integer}`
note: required by a bound in `any`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:2863:4
```

Three load-bearing facts:

- rustc spells the expected closure type as `FnMut(u32)` — *single*
  parameter slot, no comma. Direct empirical witness for the
  single-parameter parens-bound shape (contrast with lesson 151's
  `FnMut(u32, u32)` and lesson 152's `FnMut(u32, u32)`).
- `Self::Item` resolved to `u32` from the `Range<u32>` source — same
  resolution path as lessons 149-152.
- The `note: required by a bound in `any`` line points at
  `library/core/src/iter/traits/iterator.rs:2863:4` — empirical
  witness that the bound is part of `any`'s declaration. The
  rustc-internal source line shifts from lesson 152's `:2678:4` to
  `:2863:4` because `any` is declared further down in the same trait
  file.
- The `-> bool` return-type segment of the bound is *not* echoed in
  the diagnostic surface — same surface choice as lessons 149-152
  (rustc truncates the return-type segment in this E0277 surface).

## Probe 7 — `&mut self` receiver requires `let mut` (E0596)

Source: `needs_mut.rs`. Transcript: `PROBE 7` block.

```rust
fn main() {
    let it = 1..10_u32;
    let _r = it.any(|x| x == 5);
}
```

Output (compile-exit=1):

```text
error[E0596]: cannot borrow `it` as mutable, as it is not declared as mutable
 --> needs_mut.rs:3:14
  |
3 |     let _r = it.any(|x| x == 5);
  |              ^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut it = 1..10_u32;
  |         +++
```

Three load-bearing facts:

- E0596. Same code as lesson 131's `next()` E0596 contrast probe.
  Today's payload spells `cannot borrow `it` as mutable` with the
  caret on the receiver token (`it`), not the method name (`any`) —
  rustc points at the binding that needs `mut`, not the call site.
- Direct empirical witness that `any`'s receiver is `&mut self`, not
  `self` — only an `&mut self` method requires the binding to be
  mutable; an `&self` method or a consuming `self` method on a
  `Copy` type would not fire E0596 here. (`Range<u32>` is not `Copy`,
  but a consuming-`self` method would fire E0382 on a *second* use,
  not E0596 on the *first*; the E0596 fires *before* any second use,
  on the borrow itself.)
- The `help:` line proposes `let mut it = 1..10_u32;` at the binding
  site with `+++` markers — identical shape to lessons 006/047/131's
  E0596 `help:` lines. The fix is the same as lesson 131's: add
  `mut` to the binding, recompile.

## Probe 8 — type-pin (return type is exactly `bool`)

Source: `type_pin.rs`. Transcript: `PROBE 8` block.

```rust
fn main() {
    let r: bool = (1..10_u32).any(|x| x == 5);
    println!("{}", r);
}
```

Output: `true`. Compile-exit=0, run-exit=0. Two load-bearing facts:

- The annotation `bool` matches the inferred type from
  `(1..10_u32).any(...)`. No diagnostic; same output as Probe 1.
- Empirical confirmation that the return type is exactly `bool`
  (the primitive type from lesson 013), not `Option<bool>`, not a
  wrapper. The type is fully determined by the method's signature.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn any<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool` | `output/docs/rust/std/iter/trait.Iterator.md:2599` verbatim; synopsis at `:136-138` |
| `any` borrows the iterator mutably (`&mut self`); does NOT consume | Same line; Probe 2 empirical (iterator still usable after); Probe 7 empirical (E0596 requires `let mut`) |
| One non-receiver argument (`f: F`) | Same line |
| Returns `bool` | Same line; Probes 1, 2, 3, 4, 5, 8 empirical |
| Closure bound is `FnMut(Self::Item) -> bool` (single parameter slot) | Same line; Probe 6 empirical (rustc spells `FnMut(u32)`) |
| `Self::Item = u32` for `Range<u32>` | Lesson 091 (`Range<A>: Iterator` for `A: Step`); lesson 080 (u32); lesson 132 (`Self::Item` slot); Probe 6 empirical (`FnMut(u32)`) |
| Short-circuit: closure stops being called on first `true` | `trait.Iterator.md:2608-2610` verbatim; Probe 5 empirical (closure called exactly 3 times for `x == 3` match in `1..10_u32`); Probe 2 empirical (iterator position is `Some(6)`, not exhausted) |
| Iterator position lands just past the matching element | Probe 2 empirical (`Some(6)` after matching on `5`); corpus example `trait.Iterator.md:2629-2637` (matching on `1 != 2` leaves `2` next) |
| Closure runs once per element when no match | `trait.Iterator.md:2604-2606` verbatim; Probe 4 empirical (output `false`, full walk implied) |
| Empty iterator returns `false` | `trait.Iterator.md:2612` verbatim; Probe 3 empirical |
| Non-empty match returns `true` | `trait.Iterator.md:2603-2605` verbatim; Probes 1, 2, 5 empirical |
| Non-empty no-match returns `false` | `trait.Iterator.md:2604-2606` verbatim; Probe 4 empirical |
| Closure body `x == 5` is `u32 == u32 -> bool` | Lesson 013; Probe 1 empirical |
| Probe 5 closure body `count += 1; x == 3` uses block + capture + mutation | Lesson 144 (capture); lesson 023 (`+=`); lesson 148 (FnMut auto-impl); Probe 5 empirical |
| Without `let mut`, `it.any(...)` fires E0596 | `output/docs/rust/error_codes/E0596.md`; Probe 7 empirical |
| Non-closure argument fires E0277 with `FnMut(u32)` spelling | Probe 6 empirical |
| `(1..10_u32)`, `(1..1_u32)`, `(1..5_u32)`, `(1..4_u32)`, `(1..6_u32)` are `Range<u32>` values | Lesson 091; lesson 081; Probe 6 empirical (rustc reads `u32` from `Self::Item` resolution) |
| Stabilized at 1.0.0 | `trait.Iterator.md:2597` verbatim; toolchain is 1.95.0 |
| Empty-case-on-`any` is dual to lesson 152's empty-case-on-`reduce` | Lesson 152 evidence appendix; corpus `trait.Iterator.md:2474-2475` (reduce) and `:2612` (any) |

## Older supporting lessons (named only)

- 152-iterator-reduce — sibling closure-driven Iterator consumer; first
  Option-returning closure-driven member; consuming `self`.
- 151-iterator-fold — multi-parameter parens-bound; threading; consuming
  `self`.
- 150-iterator-map — closure-driven family member; lazy adapter.
- 149-iterator-for-each — first closure-driven Iterator method.
- 148-fn-fnmut-fnonce-distinction — `FnMut` choice + auto-impl rule.
- 147-fn-trait-parenthesized-bound — parens-segment + arrow-segment
  bound grammar.
- 145-generic-function-type-parameter — `<F>` slot.
- 144-closure-captures-outer-let — capture mechanic; load-bearing for
  Probe 5 only.
- 143-unannotated-closure-first-use — `|x|` no-annotation form.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 132-iterator-trait-declaration — `Self::Item` slot.
- 131-iterator-next-call — `&mut self` on `next()`; binding must be
  `let mut` (E0596 contrast). Today's structural anchor.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens-rule.
- 081-integer-literal-forms — `_u32` suffix forms.
- 080-integer-type-family — twelve integer types.
- 023-compound-add-assign — `+=` on a `mut` integer binding.
- 013-comparison-operators — `==` on `u32` produces `bool`.
- 011-println-positional-args — `println!`.
- 005-let-binding — `let x = ...`.
- 003-read-rustc-diagnostic — four-part diagnostic map.
- 002-fn-main-entry-point — `fn main()`.
- 001-rustc-compile-and-run — `rustc + ./name`.

## Deliberate scope discipline

The orchestrator's brief named scope items to NOT install. The lesson
body's *What To Ignore For Now* section names each:

1. `all` — sibling consumer; lesson 154.
2. `position` — sibling consumer with `Option<usize>` return; lesson
   155 candidate.
3. `find` — predicate consumer with `&Self::Item` parameter; gates
   on deref-read.
4. `find_map` — predicate consumer with `Option<B>`-returning closure;
   later move.
5. The "why `&mut self` and not `self`?" design rule — named and
   deferred.
6. `try_for_each`, `try_fold` — short-circuit-with-`?` variants;
   gated on the `Try` trait sub-arc.
7. `v.iter()` / `v.into_iter()` source shapes — different `Self::Item`.

## Mechanics deliberately *not* smuggled

The orchestrator's reminder list called out specific smuggling risks.
Today's discipline check:

- **No `Vec`** — source is `Range<u32>` for every probe.
- **No `.iter()` or `.into_iter()`** — bare range only.
- **No `IntoIterator`** — gated on its own sub-arc.
- **No `&Self::Item`** — `any`'s closure parameter is `Self::Item` by
  *value*. Probe 6 empirically: rustc spells `FnMut(u32)`, not
  `FnMut(&u32)`. The deref-read mechanic is reserved for `find`.
- **No deref-read `*x`** — closure bodies are `x == 5`, `x == 100`,
  `x == 3`, `x != 2` shapes; plain owned `==` per lesson 013.
- **No `as` casting** — `==` on `u32 == u32` produces `bool` directly.
- **No `Box<dyn Fn>`, no `impl Fn`** — bound is named
  `FnMut(Self::Item) -> bool`, consumed by the generic `<F>` slot.
- **No `move` keyword** — Probes 1, 2, 3, 4, 8 capture nothing;
  Probe 5 captures by mutable reference (the default mode for
  `count += 1` since `u32` is `Copy` but the *binding* `count` is
  what's referenced — see lesson 144 evidence for capture-mode
  details).
- **No `match` on `bool`** — lesson uses `println!("{}", r)` only.
- **No introduction of `&mut self` as a centered concept on its own**
  — lesson 131 already installed it; today reuses it on a new method.

## Run-context handoff

After this lesson lands, the orchestrator's options for lesson 154
include:

- `all` — direct sibling. Same signature shape (`&mut self`,
  `FnMut(Self::Item) -> bool`, returns `bool`) but inverted semantics
  ("do *all* elements match?"); short-circuits on the first `false`;
  empty iterator returns `true` (the existential-versus-universal
  duality). Smallest possible next step from today.
- `position` — sibling with same closure shape but `Option<usize>`
  return. Combines today's short-circuit fact with a different
  return-type wrapper.
- `find` — predicate consumer with `&Self::Item` parameter. First
  place the audience meets the *reference closure parameter* shape;
  introduces the deref-read sub-arc.
- `find_map` — `Option<B>`-returning closure; composes today's
  short-circuit with `Option`-returning closure bodies.

The audit's predicate-consumer arc continues for at least 4 more
moves; today is the first.
