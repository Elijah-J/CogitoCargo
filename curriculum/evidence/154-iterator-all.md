# Evidence — Lesson 154: `Iterator::all` (sibling of `any`; inverted polarity, dual empty-case)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/154-iterator-all.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/154-iterator-all.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/154-iterator-all.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet154/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-153.

## Run context — `all` as sibling of `any`

Lesson 153 just installed `any`. Today's `all` is the **smallest**
next move because:

1. Same signature shape as `any` (`<F>(&mut self, f: F) -> bool where
   F: FnMut(Self::Item) -> bool`); only the semantics differ.
2. No new mechanics — same `&mut self`, same closure bound, same
   `bool` return, same short-circuit pattern.
3. Sharpens the empty-case rule installed by `any` (lesson 153
   Probe 3 returned `false` for empty `any`; today's Probe 3 returns
   `true` for empty `all`) — the existential-versus-universal duality
   becomes empirically concrete.

Audit `iterator-api-coverage.md` (already cited by lesson 153) lists
`all` immediately adjacent to `any` in the predicate-consumer arc.

## Direct prerequisite — lesson 153 (`any`; load-bearing)

Lesson 153 installed `Iterator::any` with signature
`fn any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool`.
Today's lesson explicitly leans on lesson 153 throughout. Five
sub-claims load-bearing today:

- **Signature shape carries unchanged.** Lesson 153's signature
  reading (`<F>` slot, `&mut self` receiver, single non-receiver
  argument `f: F`, `-> bool` return, parens-bound
  `FnMut(Self::Item) -> bool`) applies verbatim to today's `all`.
  Today's lesson body says explicitly "this is the same shape" and
  cross-references lesson 153 rather than re-deriving each segment.
- **Receiver is `&mut self`.** Lesson 153 installed this rotation
  (consuming `self` in 149-152 → mutable borrow `&mut self` in 153)
  and witnessed it via Probe 2 (iterator still usable after) and
  Probe 7 (E0596 without `let mut`). Today's Probes 5 and 8 rotate
  the same witnesses with `any` → `all`.
- **Closure bound is `FnMut(Self::Item) -> bool` (single parameter).**
  Lesson 153 installed this single-parameter parens-bound shape and
  witnessed it via Probe 6 (`expected a FnMut(u32) closure`). Today's
  Probe 7 rotates the same witness with `any` → `all`.
- **Iterator position after short-circuit lands just past the
  decisive element.** Lesson 153 Probe 2 showed `any` matching on
  `5` and leaving `6` next. Today's Probe 5 shows `all` failing on
  `5` and leaving `6` next — same position-after-short-circuit shape.
- **Empty-case identity is part of the design.** Lesson 153 Probe 3
  showed empty-`any` returns `false` (corpus `:2612`). Today's
  Probe 3 shows empty-`all` returns `true` (corpus `:2570`) — direct
  dual.

## Direct prerequisite — lesson 152 (`reduce`; named only)

Lesson 152 installed the empty-case-as-design-choice contrast for
predicate consumers. Today cites 152 only via lesson 153's existing
contrast chain; not load-bearing in its own right.

## Direct prerequisite — lesson 148 (`FnMut`; auto-impl rule)

Lesson 148 installed the three-trait family. Today's Probe 4 closure
captures `count` and mutates it; under 148's auto-impl rule, that
closure implements `FnMut` and `FnOnce` but not `Fn`. The bound
`FnMut` accepts it. Same direct empirical witness as lesson 153
Probe 5. Today's Probes 1, 2, 3, 5, 6 use closures that capture
nothing — they implement all three traits and fit the bound trivially.

## Direct prerequisite — lesson 147 (parenthesized bound)

Lesson 147 installed `<F: FnMut(T) -> R>` segment grammar. Today's
bound has the same single-slot form `FnMut(Self::Item) -> bool` as
`any`. Cited but not re-derived.

## Direct prerequisite — lesson 144 (closure captures outer `let`)

Lesson 144 installed the capture mechanic. Today's Probe 4 captures
`count` and mutates it — same usage shape as lesson 153 Probe 5.

## Direct prerequisite — lesson 142 (closure literal)

Lesson 142 installed `|param| body`. Today's `|x|` reuses the
single-parameter form unchanged.

## Direct prerequisite — lesson 132 (`Self::Item` slot)

Lesson 132 installed `type Item;` as the trait's associated-type
slot. For `Range<u32>`, `Self::Item = u32`. Probe 7 empirical: rustc
spells `FnMut(u32)`.

## Direct prerequisite — lesson 131 (`&mut self` + `let mut` rule)

Lesson 131 installed `fn next(&mut self) -> Option<Self::Item>` and
the borrow rule "binding must be `let mut`" via E0596 contrast.
Lesson 153 reused both on `any`; today reuses both again on `all`:
- Probe 5 calls `it.next()` after `.all(...)` returns — direct
  empirical reuse, same shape as lesson 153 Probe 2.
- Probe 8 fires E0596 with `cannot borrow `it` as mutable` —
  same shape as lesson 153 Probe 7, method name rotated.

## Direct prerequisite — lessons 091 + 081 + 080 (`Range<u32>` source)

Lesson 091 grounds `Range<A>: Iterator` for `A: Step`. Lesson 081
installs `_u32` suffix. Lesson 080 installs `u32`. Today's
`(1..10_u32)`, `(1..1_u32)`, `(1..4_u32)`, and the bare range
`1..10_u32` (Probes 5, 8) all rely on these lessons. The half-open
semantics drives Probe 3's empty case.

## Direct prerequisite — lesson 023 (`+=` on a mut binding)

Probe 4's `count += 1` is `u32 += u32` per lesson 023, with `count`
declared `let mut count = 0_u32;`. Same shape as lesson 153 Probe 5.

## Direct prerequisite — lesson 013 (`<` and `==` on integers produce `bool`)

Lesson 013 installed comparison operators on integers. Today's
closure bodies `x < 100`, `x < 5`, `x < 0`, `x > 100` are
`u32 < u32 -> bool` (or `>`) per lesson 013. The result type
matches the bound's return slot `-> bool`.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's
diagnostics:

- **`unused_comparisons` warning** (Probe 3): rustc warns when a
  comparison is impossible due to type limits. Today's `x < 0` for
  `u32` triggers this. Not an error code; just a warning. The
  warning is *load-bearing* for the lesson's vacuous-truth point —
  rustc agrees the predicate is impossible, yet the program prints
  `true` because the closure is never called.
- **E0277** (Probe 7): `expected a FnMut(u32) closure, found {integer}`.
  Same code as lesson 153 Probe 6, with method name rotated. Direct
  empirical witness for the single-parameter parens-bound shape on
  `all`. Lesson 003 covered the four-part map. Today's load-bearing
  payload is the inline `FnMut(u32)` spelling and the
  `library/core/src/iter/traits/iterator.rs:2809:4` location (note:
  `:2809:4` for `all` is *before* `:2863:4` for `any`, matching the
  declaration order in the corpus where `all` precedes `any` —
  `trait.Iterator.md:2557` for `all`, `:2599` for `any`).
- **E0596** (Probe 8): `cannot borrow `it` as mutable`. Same code
  as lesson 153 Probe 7, method name rotated. Direct empirical
  witness that `all`'s receiver is `&mut self`.

No new error code today. The move is *shape-only* relative to lesson
153; rustc's existing diagnostics suffice with method name rotated.

## Cited prereqs

- **Lesson 145**: `<F>` generic-function type-parameter slot.
- **Lesson 011**: `println!("{} {:?}", ..., ...)`.
- **Lesson 005**: `let r = ...` binding for `all`'s return.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc + ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics, examples)

Verified by reading lines 133-135 (synopsis-box) and 2555-2595 (full
method entry).

### Lines 133-135 (synopsis-box version)

```text
fn all<F>(&mut self, f: F) -> bool
   where Self: Sized,
         F: FnMut(Self::Item) -> bool { ... }
```

`{ ... }` confirms `all` is a *provided* method. Note: `all` appears
in the synopsis box *before* `any` (lines 136-138), matching the
alphabetical order in which they are declared in the corpus.

### Line 2557 (full signature, main entry)

```text
fn all<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool,
```

Direct corpus source for the lesson body's signature. Five
load-bearing facts (same shape as lesson 153's `any`):

- `<F>` — *one* type parameter.
- `(&mut self, f: F)` — receiver `&mut self` (mutable borrow), then
  *one* non-receiver parameter `f: F`.
- `-> bool` — return type is the primitive `bool`.
- `where Self: Sized` — same bound as 149-153. Not centered.
- `F: FnMut(Self::Item) -> bool` — bound; identical to `any`'s.

### Line 2555 (stabilization)

```text
1.0.0 ·
```

Stabilization at 1.0.0; well below the local toolchain 1.95.0.

### Lines 2559-2570 (semantics)

```text
Tests if every element of the iterator matches a predicate.

`all()` takes a closure that returns `true` or `false`. It applies
this closure to each element of the iterator, and if they all return
`true`, then so does `all()`. If any of them return `false`, it
returns `false`.

`all()` is short-circuiting; in other words, it will stop processing
as soon as it finds a `false`, given that no matter what else happens,
the result will also be `false`.

An empty iterator returns `true`.
```

Direct corpus source for four load-bearing claims:

- "It applies this closure to each element of the iterator" — same
  per-element pattern as `any`. Probes 1, 4 empirical.
- "If they all return `true`, then so does `all()`" — universal
  match returns `true`. Probe 1 (`true`) empirical.
- "If any of them return `false`, it returns `false`" — first
  failing element returns `false`. Probes 2, 4, 5 empirical.
- "`all()` is short-circuiting ... it will stop processing as soon
  as it finds a `false`" — direct corpus source for the lesson's
  centered short-circuit-on-`false` fact. Probe 4 empirical (closure
  called exactly 5 times for failing `5 < 5` in `1..10_u32`, not 9
  times).
- "An empty iterator returns `true`" — direct corpus source for
  Probe 3's empty-case output. Dual to lesson 153 corpus `:2612`.

### Lines 2576-2582 (corpus example, basic)

```text
let a = [1, 2, 3];

assert!(a.into_iter().all(|x| x > 0));

assert!(!a.into_iter().all(|x| x > 2));
```

The corpus example uses `[1, 2, 3].into_iter()`; today's lesson
keeps `Range<u32>` (lessons 091 + 081) to avoid pulling in
`IntoIterator` and arrays — same source-choice discipline as
lesson 153. The structural witness is the same: predicate-true-for-
all returns `true`, predicate-false-for-some returns `false`.

### Lines 2584-2594 (corpus example, short-circuit + reuse)

```text
Stopping at the first `false`:

let a = [1, 2, 3];

let mut iter = a.into_iter();

assert!(!iter.all(|x| x != 2));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(3));
```

Direct corpus source for the lesson body's "iterator survives the
call" probe. The corpus uses `iter.all(|x| x != 2)`: `1 != 2` returns
`true`, `2 != 2` returns `false` on the second call, so the iterator
stops just after `2`, leaving `3` as the next yielded value. Today's
Probe 5 uses a similar shape with `Range<u32>` source: `it.all(|x|
x < 5)` fails on `5`, leaving `6` as the next yielded value. The
corpus comment "we can still use `iter`, as there are more elements"
is the prose form of Probe 5's `Some(6)` output.

## Source — `output/docs/rust/error_codes/E0596.md` (mutable borrow on non-mut binding)

Lessons 131, 047/048, 153 already cited. Today reuses unchanged:
rustc cannot mutably borrow a binding declared without `mut`. Probe
8's diagnostic is the application of E0596 to the receiver of an
`&mut self` method call, with the `help:` line proposing
`let mut it = 1..10_u32;` at the binding site with `+++` markers.

## Source — `output/docs/rust/error_codes/E0277.md` (trait bound not satisfied)

Lessons 146-153 already cited. Today reuses unchanged. Probe 7's
diagnostic spells `FnMut(u32)` — same single-parameter shape as
lesson 153 Probe 6.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Reused today: `Range<A>:
Iterator where A: Step`. The half-open semantics drives Probe 3's
empty case.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lessons 148-153 already cited. Reused today.

## Probe 1 — working `all` on `Range<u32>` (universally-quantified positive)

Source: `observations/154-iterator-all.rs` (canonical shape; local
probe at `/tmp/eduratchet154/demo.rs`). Transcript: `PROBE 1` block.

```rust
fn main() {
    let r = (1..10_u32).all(|x| x < 100);
    println!("{}", r);
}
```

Output: `true`. Compile-exit=0, run-exit=0. Four load-bearing facts:

- The bound `F: FnMut(Self::Item) -> bool` accepts a closure literal
  `|x| x < 100` as the only non-receiver argument to `.all(...)` on
  `(1..10_u32)`.
- `Self::Item = u32` for `Range<u32>`; the closure parameter `x` is
  a `u32`. The closure body `x < 100` is `<` on two `u32` values
  producing `bool` per lesson 013.
- Every element `1..9` is less than `100`. The closure returns
  `true` for every call; `all` walks the full iterator and returns
  `true`.
- `all` returns `bool`; the inferred type for `let r = ...` is
  `bool`. `println!("{}", r)` produces the Display form `true`.

## Probe 2 — first-element-fails short-circuit

Source: `short_circuit.rs`. Transcript: `PROBE 2` block.

```rust
fn main() {
    let r = (1..10_u32).all(|x| x < 5);
    println!("{}", r);
}
```

Output: `false`. Compile-exit=0, run-exit=0. The closure returns
`true` for `x = 1, 2, 3, 4` and `false` for `x = 5`. `all` short-
circuits on `false`. Probe 4 makes the call count explicit; here
just witnesses the return value.

## Probe 3 — empty iterator returns `true` (load-bearing — vacuous truth)

Source: `empty.rs`. Transcript: `PROBE 3` block.

```rust
fn main() {
    let r = (1..1_u32).all(|x| x < 0);
    println!("{}", r);
}
```

Compile output (rustc 1.95.0):

```text
warning: comparison is useless due to type limits
 --> empty.rs:2:32
  |
2 |     let r = (1..1_u32).all(|x| x < 0);
  |                                ^^^^^
  |
  = note: `#[warn(unused_comparisons)]` on by default

warning: 1 warning emitted
```

Run output: `true`. Compile-exit=0, run-exit=0.

This is the **load-bearing empty-case probe** for the lesson. Five
load-bearing facts:

- `(1..1_u32)` is a half-open `Range<u32>` whose lower bound equals
  upper bound — yields no elements (lesson 091 + 081).
- With no elements to apply the closure to, `all` returns `true`.
  Direct corpus correspondence: `trait.Iterator.md:2570` "An empty
  iterator returns `true`."
- The closure was *never called*. No side effects observable.
- The `unused_comparisons` warning is rustc detecting at compile
  time that `x < 0` is impossible for any `u32` value (the lower
  bound of `u32` is `0`, so `x < 0` is always `false`). Yet the
  program still prints `true`. Direct empirical witness for vacuous
  truth: the closure's logic is *irrelevant* because the closure is
  never called. The compile-time warning and the runtime answer are
  decoupled.
- This is the **dual** of lesson 153 Probe 3: there
  `(1..1_u32).any(|x| x == 5)` returned `false`; here
  `(1..1_u32).all(|x| x < 0)` returns `true`. Empty-`any` is `false`;
  empty-`all` is `true`.

The orchestrator's brief specifically chose `x < 0` over a
non-warning predicate (e.g. `x == 0`) because the warning sharpens
the vacuous-truth point: rustc agrees the predicate is impossible,
yet the answer is still `true`. The warning is informative, not
distracting.

## Probe 4 — closure-call count (short-circuit witness, captures `count`)

Source: `count_calls.rs`. Transcript: `PROBE 4` block.

```rust
fn main() {
    let mut count = 0_u32;
    let r = (1..10_u32).all(|x| { count += 1; x < 5 });
    println!("{} {}", r, count);
}
```

Output: `false 5`. Compile-exit=0, run-exit=0. Four load-bearing
facts:

- The closure ran exactly *five* times — for `x = 1, 2, 3, 4, 5` —
  and stopped. The closure body `count += 1` ran on each call; final
  `count = 5`. Direct empirical witness for short-circuit on `false`:
  `all` stops calling the closure after the first `false`.
- For `x = 1, 2, 3, 4` the body `x < 5` returned `true`. On the
  fifth call (`x = 5`), `5 < 5` returned `false`, and `all` stopped.
- The braced closure body `{ count += 1; x < 5 }` is a regular Rust
  block: `count += 1;` is a statement, `x < 5` is the trailing
  expression that becomes the closure's return value. Same shape as
  lesson 153 Probe 5 with `==` swapped to `<`.
- The closure captures `count` from the enclosing scope (lesson 144)
  and *mutates* it via `count += 1` (lesson 023). Under lesson 148's
  auto-impl rule, this closure implements `FnMut` and `FnOnce` but
  not `Fn`. The bound `FnMut` accepts this shape. Direct empirical
  witness — same as lesson 153 Probe 5.

The output `5` (not `9` for the full range, not `4` for the count
of `true` returns) is the direct numeric witness: the closure was
called for each element starting from the first, in order, and
stopped on the first `false` return — which happened on the fifth
call, *one call after* the run of four `true`s. Contrast with lesson
153 Probe 5 (`true 3` for `any` matching on `x == 3`).

## Probe 5 — `&mut self` reusability witness (rotated from lesson 153 Probe 2)

Source: `reusable.rs`. Transcript: `PROBE 5` block.

```rust
fn main() {
    let mut it = 1..10_u32;
    let r = it.all(|x| x < 5);
    let n = it.next();
    println!("{} {:?}", r, n);
}
```

Output: `false Some(6)`. Compile-exit=0, run-exit=0. Three load-bearing
facts witnessed in a single execution (same shape as lesson 153
Probe 2, with `any` rotated to `all`):

- After `it.all(...)` returns, the binding `it` is *still usable* —
  the next line successfully calls `it.next()`. Direct empirical
  reuse of lesson 153's "iterator survives the call" mechanic.
- The iterator's next yielded value is `Some(6)`, *not* `Some(1)`.
  `all` advanced the iterator past elements `1, 2, 3, 4, 5` while
  testing.
- The iterator's next yielded value is `Some(6)`, *not* `None`.
  `all` stopped advancing as soon as the closure returned `false` —
  short-circuit. The iterator is consumed *up to and including* the
  first failing element, but no further. Same position-after-short-
  circuit shape as lesson 153 Probe 2 (`any` matching on `5` left
  `6` next; here `all` failing on `5` leaves `6` next).

## Probe 6 — type-pin (return type is exactly `bool`)

Source: `type_pin.rs`. Transcript: `PROBE 6` block.

```rust
fn main() {
    let r: bool = (1..10_u32).all(|x| x < 100);
    println!("{}", r);
}
```

Output: `true`. Compile-exit=0, run-exit=0. The annotation `bool`
matches the inferred type from `(1..10_u32).all(...)`. Empirical
confirmation that the return type is exactly `bool` — same as `any`,
same as Probe 1.

## Probe 7 — non-closure argument fires E0277 (rehearses lesson 153 Probe 6)

Source: `non_closure_arg.rs`. Transcript: `PROBE 7` block.

```rust
fn main() {
    let r = (1..4_u32).all(7);
    println!("{}", r);
}
```

Output (compile-exit=1):

```text
error[E0277]: expected a `FnMut(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:2:28
  |
2 |     let r = (1..4_u32).all(7);
  |                        --- ^ expected an `FnMut(u32)` closure, found `{integer}`
  |                        |
  |                        required by a bound introduced by this call
  |
  = help: the trait `FnMut(u32)` is not implemented for `{integer}`
note: required by a bound in `all`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:2809:4
```

Three load-bearing facts (same as lesson 153 Probe 6 with `any`
rotated to `all`):

- rustc spells the expected closure type as `FnMut(u32)` — *single*
  parameter slot, no comma. Identical bound shape to `any`.
- `Self::Item` resolved to `u32` from the `Range<u32>` source.
- The `note: required by a bound in `all`` line points at
  `library/core/src/iter/traits/iterator.rs:2809:4` — *before*
  lesson 153's `:2863:4` for `any`. This matches the corpus order:
  `all` is declared at `trait.Iterator.md:2557`, `any` at `:2599`.
- The `-> bool` return-type segment is truncated in the diagnostic
  surface — same convention as lessons 149-153.

This probe is included only to confirm the bound shape matches
`any` exactly. Lesson 153 already installed the diagnostic; today's
witness is rotation-only.

## Probe 8 — `&mut self` receiver requires `let mut` (E0596)

Source: `needs_mut.rs`. Transcript: `PROBE 8` block.

```rust
fn main() {
    let it = 1..10_u32;
    let _r = it.all(|x| x < 100);
}
```

Output (compile-exit=1):

```text
error[E0596]: cannot borrow `it` as mutable, as it is not declared as mutable
 --> needs_mut.rs:3:14
  |
3 |     let _r = it.all(|x| x < 100);
  |              ^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut it = 1..10_u32;
  |         +++
```

Same shape as lesson 153 Probe 7, method name rotated. Direct
empirical witness that `all`'s receiver is `&mut self`. Included
only for rotation-confirmation.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn all<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool` | `output/docs/rust/std/iter/trait.Iterator.md:2557` verbatim; synopsis at `:133-135` |
| `all` borrows the iterator mutably (`&mut self`); does NOT consume | Same line; Probe 5 empirical (iterator still usable); Probe 8 empirical (E0596 requires `let mut`) |
| One non-receiver argument (`f: F`) | Same line |
| Returns `bool` | Same line; Probes 1, 2, 3, 4, 5, 6 empirical |
| Closure bound is `FnMut(Self::Item) -> bool` (single parameter slot) | Same line; Probe 7 empirical (rustc spells `FnMut(u32)`) |
| `Self::Item = u32` for `Range<u32>` | Lesson 091, 080, 132; Probe 7 empirical |
| Short-circuit: closure stops on first `false` (inverted from `any`) | `trait.Iterator.md:2566-2568` verbatim; Probe 4 empirical (closure called exactly 5 times for `x < 5` failing in `1..10_u32`); Probe 5 empirical (iterator position is `Some(6)`, not exhausted) |
| Iterator position lands just past the failing element | Probe 5 empirical (`Some(6)` after failing on `5`); corpus example `trait.Iterator.md:2584-2594` (failing on `2 != 2` leaves `3` next) |
| Closure runs once per element when all return `true` | `trait.Iterator.md:2561-2563` verbatim; Probe 1 empirical (output `true`, full walk implied) |
| Empty iterator returns `true` (vacuous truth, dual to `any`'s `false`) | `trait.Iterator.md:2570` verbatim; Probe 3 empirical |
| Non-empty all-true returns `true` | `trait.Iterator.md:2561-2563` verbatim; Probe 1 empirical |
| Non-empty first-false returns `false` | `trait.Iterator.md:2563-2564` verbatim; Probes 2, 4, 5 empirical |
| Closure body `x < 5` is `u32 < u32 -> bool` | Lesson 013; Probe 4 empirical |
| Probe 4 closure body uses block + capture + mutation | Lesson 144 (capture); lesson 023 (`+=`); lesson 148 (FnMut auto-impl); Probe 4 empirical |
| Without `let mut`, `it.all(...)` fires E0596 | Lesson 131 + 153; Probe 8 empirical |
| Non-closure argument fires E0277 with `FnMut(u32)` spelling | Probe 7 empirical |
| `unused_comparisons` warning fires for `x < 0` on `u32` | Probe 3 transcript (compile-time warning); not a separate corpus citation — built-in lint |
| Stabilized at 1.0.0 | `trait.Iterator.md:2555` verbatim; toolchain is 1.95.0 |
| `all` is a sibling of `any` with same signature shape, inverted polarity | Lesson 153 (`any` evidence); `trait.Iterator.md:2557` (`all`) and `:2599` (`any`) — identical signature shapes |
| Empty-`all` is dual to empty-`any` | Lesson 153 evidence; `trait.Iterator.md:2570` (`all` true) vs `:2612` (`any` false) |

## Older supporting lessons (named only)

- 153-iterator-any — direct sibling. Same signature; today inverts
  polarity. Today's structural anchor.
- 152-iterator-reduce — sibling closure-driven consumer; cited via
  153 only.
- 151-iterator-fold, 150-iterator-map, 149-iterator-for-each —
  closure-driven Iterator family.
- 148-fn-fnmut-fnonce-distinction — `FnMut` choice + auto-impl rule.
- 147-fn-trait-parenthesized-bound — parens-segment + arrow-segment.
- 145-generic-function-type-parameter — `<F>` slot.
- 144-closure-captures-outer-let — capture mechanic.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 132-iterator-trait-declaration — `Self::Item` slot.
- 131-iterator-next-call — `&mut self` on `next()`; E0596 contrast.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens-rule;
  half-open semantics.
- 081-integer-literal-forms — `_u32` suffix.
- 080-integer-type-family — `u32`.
- 023-compound-add-assign — `+=` on a `mut` integer binding.
- 013-comparison-operators — `<`, `==` on `u32` produce `bool`.
- 011-println-positional-args — `println!`.
- 005-let-binding — `let x = ...`.
- 003-read-rustc-diagnostic — four-part diagnostic map.
- 002-fn-main-entry-point — `fn main()`.
- 001-rustc-compile-and-run — `rustc + ./name`.

## Deliberate scope discipline

The orchestrator's brief named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. `position` — sibling consumer with `Option<usize>` return; lesson
   155 candidate.
2. `find` / `find_map` — predicate consumers with `&Self::Item`
   parameter; gates on deref-read.
3. The `any` ↔ `all` De Morgan duality at the closure-body level
   (`!cond`) — named, formal proof deferred.
4. The "why have both `any` and `all`?" design rule — named,
   deferred.
5. `try_for_each`, `try_fold` — short-circuit-with-`?` variants;
   gated on the `Try` trait sub-arc.

## Mechanics deliberately *not* smuggled

The orchestrator's reminder list called out specific smuggling risks.
Today's discipline check (carries from lessons 149-153 unchanged):

- **No `Vec`** — source is `Range<u32>` for every probe.
- **No `.iter()` or `.into_iter()`** — bare range only.
- **No `IntoIterator`** — gated on its own sub-arc.
- **No `&Self::Item`** — `all`'s closure parameter is `Self::Item`
  by *value*. Probe 7 empirical: rustc spells `FnMut(u32)`.
- **No deref-read `*x`** — closure bodies are `x < 100`, `x < 5`,
  `x < 0`, `x > 100`; plain owned `<`, `>` per lesson 013.
- **No `as` casting** — `<` on `u32 < u32` produces `bool` directly.
- **No `Box<dyn Fn>`, no `impl Fn`** — bound is named
  `FnMut(Self::Item) -> bool`, consumed by the generic `<F>` slot.
- **No `move` keyword** — Probes 1, 2, 3, 5, 6 capture nothing;
  Probe 4 captures by mutable reference (the default mode for
  `count += 1`).
- **No `match` on `bool`** — lesson uses `println!("{}", r)` only.
- **No re-installation of `&mut self` as a centered concept** —
  lesson 131 installed it; lesson 153 reused it on a closure-driven
  method; today reuses it on `all` without re-deriving.

## Run-context handoff

After this lesson lands, the orchestrator's options for lesson 155
include:

- `position` — sibling with same closure shape but `Option<usize>`
  return. Combines today's short-circuit-on-`false` (well, `true` —
  `position` short-circuits on first `true` like `any`) with a
  different return-type wrapper. First time `Option<usize>` appears
  as a payload type.
- `find` — predicate consumer with `&Self::Item` parameter. First
  place the audience meets the *reference closure parameter* shape;
  introduces the deref-read sub-arc.
- `find_map` — `Option<B>`-returning closure; composes today's
  short-circuit pattern with `Option`-returning closure bodies.

The audit's predicate-consumer arc continues for at least 3 more
moves; today is the second.
