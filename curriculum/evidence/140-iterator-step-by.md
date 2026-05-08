# Evidence — Lesson 140: yield every `step`th element with `iter.step_by(step)`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/140-iterator-step-by.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/140-iterator-step-by.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/140-iterator-step-by.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet140/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons through 139.

## Direct prerequisites — lessons 136 (`take`) and 137 (`skip`)

Lessons 136 and 137 installed the adapter shape:

- Receiver: bare consuming `self`.
- Second parameter: `n: usize`.
- Return type: opaque wrapper struct (`Take<Self>` or `Skip<Self>`)
  that itself implements `Iterator`.
- Lazy: building the wrapper performs no work.

Today's signature
`fn step_by(self, step: usize) -> StepBy<Self> where Self: Sized,`
(`output/docs/rust/std/iter/trait.Iterator.md:551`) reuses every slot
modulo the parameter name (`step` vs `n`) and wrapper type (`StepBy`
vs `Take`/`Skip`). The bare `self` rule is *not* re-witnessed via
E0382 — six prior captures on lessons 102/133/134/136/137/138 suffice.

The new fact relative to 136/137:

- 136/137 had no documented runtime precondition. Calling `take(0)`
  produces an empty iterator silently; calling `skip(N)` past length
  produces an empty iterator silently (lesson 137 evidence). Today's
  `step_by(0)` *panics*, and the panic fires at construction — see
  Probe 2 below.

## Direct prerequisite — lesson 132 (`Iterator` trait declaration)

Lesson 132 installed
`pub trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; /* + 75 provided */ }`
and the default-body inheritance mechanic from lesson 116. The
synopsis-box line for `step_by` at
`output/docs/rust/std/iter/trait.Iterator.md:25-26`:

```text
    fn step_by(self, step: usize) -> StepBy<Self> ⓘ
       where Self: Sized { ... }
```

ends in `{ ... }` — lesson 116's default-body marker. Every iterator
inherits `step_by` automatically; no implementor work required.

## Direct prerequisite — lesson 131 (`.next()` on a slice iterator)

Lesson 131 installed `.next()` on a slice iterator returning
`Option<&T>` and stopping at `None`. Probe 1's `for` loop and chained
`.count()` both rely on the slice iterator stopping at `None` once
indices `0, 2, 4` have been yielded and index 6 is past the end.

## Direct prerequisite — lesson 053 (`Result::expect` panic shape)

Lesson 053 installed:

- Runtime panic format `thread 'main' (...) panicked at <file>:<line>:
  <message>`.
- Output goes to stderr; stdout is silent for the line that would
  have followed.
- Exit status 101 for a panic in `main`.
- `note: run with RUST_BACKTRACE=1` trailer.

Today's `step != 0` panic (Probe 2a/2b/2c) reproduces this exact
template — same prefix, same trailer, same exit status 101. The only
substantive difference: lesson 053's message had a human-written
prefix (`expected even: 7`) because `expect` takes a `msg: &str`
parameter; today's message is the bare `assert!` body
`assertion failed: step != 0` because the std implementation uses
`assert!(step != 0)` with no human prefix. No new panic mechanic is
introduced.

## Older supporting lessons

- **Lesson 133** (cited) — `.count()` is the chained consumer on
  Probe 1's second half.
- **Lesson 102** (cited) — bare-`self` consuming receiver shape.
- **Lesson 080** (cited) — `usize` is one row of the integer family.
- **Lesson 123** (cited) — `v.iter()` for slice iteration.
- **Lessons 049, 040, 022, 011, 005, 003, 002, 001** (cited) — method
  chaining; dot-call; `for x in iter`; `println!`; `let`; diagnostic
  map; `fn main`; rustc compile + run.

## Probe 1 — working probe (`step_by(2)` for-loop + chained `.count()`)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/140-iterator-step-by.rs`.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    for x in v.iter().step_by(2) {
        println!("{}", x);
    }

    println!("---");

    let n = v.iter().step_by(2).count();
    println!("{}", n);
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
10
30
50
---
3
$ echo "run-exit=$?"
run-exit=0
```

Witnesses three claims:

1. **`step_by(step)` yields the element at indices `0, step, 2*step,
   …` and stops past the end.** Output lines 1-3 (`10`, `30`, `50`)
   correspond to indices `0`, `2`, `4` of the source vec; index `6`
   is past the end so the for-loop stops. Per
   `trait.Iterator.md:556-557`: "The first element of the iterator
   will always be returned, regardless of the step given."
2. **`StepBy<Self>` is itself an `Iterator`.** Two pieces of
   evidence: (a) the `for x in v.iter().step_by(2)` form works —
   `for` desugars through `IntoIterator`/`Iterator`; (b) the chained
   `.count()` works — `count` is one of the 75 provided methods on
   `Iterator`.
3. **Two consumer disciplines agree.** Iterating with `for` produces
   3 lines; chaining `.count()` independently reports `3`. The two
   disciplines walk different `StepBy<Self>` instances (each call to
   `v.iter().step_by(2)` builds a fresh wrapper), so the agreement
   is a sanity check.

## Probe 2 — `step_by(0)` panic at construction (the centered new-fact probe)

The doc at `trait.Iterator.md:582-584`:

```text
##### Panics

The method will panic if the given step is `0`.
```

does not specify *when* the panic fires. The implementation could
plausibly check at construction (`step_by` body) or lazily at first
`.next()` (`StepBy::next` body). Three sub-probes determine which.

### Probe 2a — `let _ = v.iter().step_by(0);` (no `.next()` call)

Source `zero_construct.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _ = v.iter().step_by(0);
    println!("constructed, no iteration");
}
```

Transcript:

```text
$ rustc zero_construct.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./zero_construct
thread 'main' (146355320) panicked at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/adapters/step_by.rs:35:9:
assertion failed: step != 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo "run-exit=$?"
run-exit=101
```

`println!("constructed, ...")` never runs (stdout is empty). The
panic fires *before* the line after the `step_by(0)` call. Therefore
the panic is not in `StepBy::next` — it must be in the `step_by`
constructor body itself.

### Probe 2b — `let mut it = ...; it.next();` (forced first iteration)

Source `zero_iter.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut it = v.iter().step_by(0);
    let _ = it.next();
    println!("after first next");
}
```

Transcript: identical panic message, identical line number
(`step_by.rs:35:9`), exit `101`. Same call site `step_by(0)` is
where the panic fires; the `it.next()` call never runs.

### Probe 2c — bare statement form (no binding)

Source `zero_stmt.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    v.iter().step_by(0);
    println!("after stmt");
}
```

`rustc zero_stmt.rs` produces a `unused_must_use` warning ("iterators
are lazy and do nothing unless consumed") — useful corroboration of
the family-level laziness claim. The compile still succeeds. At
runtime: same panic, same line, exit `101`. The binding form does
not affect when the panic fires.

### Conclusion from Probe 2

The std implementation calls `assert!(step != 0)` (or equivalent)
inside the `step_by` constructor body — visible in the panic location
`library/core/src/iter/adapters/step_by.rs:35:9` (rustc's commit
hash `59807616e1fa2540724bfbac14d7976d7e4a3860` matches the toolchain's
build hash). This precondition is checked unconditionally at
construction; *no* iteration is required to trigger the panic.

The panic shape exactly reproduces lesson 053's template:
`thread 'main' (...) panicked at <file>:<line>: <message>` plus the
`note: run with RUST_BACKTRACE=1` trailer plus exit `101`. The only
new wrinkle is that the message comes from a bare `assert!` (no
human-written prefix), so it reads `assertion failed: step != 0`.

## Probe 3 — `step_by(1)` is the identity adapter

Source `step_one.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter().step_by(1) {
        println!("{}", x);
    }
}
```

Transcript:

```text
$ rustc step_one.rs
$ ./step_one
10
20
30
$ echo "run-exit=$?"
run-exit=0
```

`step_by(1)` reaches every index `0, 1, 2, …` so every element is
yielded. The corpus does not state this directly, but it follows
mechanically from "stepping by the given amount" and the `:556-557`
note "The first element of the iterator will always be returned".
Probe 3 corroborates.

## Probe 4 — `step_by(100)` past length yields just the first element

Source `step_past.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter().step_by(100) {
        println!("{}", x);
    }
    println!("---");
    let n = v.iter().step_by(100).count();
    println!("count={}", n);
}
```

Transcript:

```text
$ rustc step_past.rs
$ ./step_past
10
---
count=1
$ echo "run-exit=$?"
run-exit=0
```

Two witnesses for the same claim — per `:556-557` "The first element
of the iterator will always be returned, regardless of the step
given." A 3-element source with `step_by(100)` yields just `&10`; the
for-loop runs once, the chained `.count()` reports `1`. No panic
(distinct from `step_by(0)`).

## Probe 5 — type-pin via E0308 names `StepBy<Iter<'_, u64>>`

Source `typeprobe.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: u32 = v.iter().step_by(2);
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typeprobe.rs:3:19
  |
3 |     let _x: u32 = v.iter().step_by(2);
  |             ---   ^^^^^^^^^^^^^^^^^^^ expected `u32`, found `StepBy<Iter<'_, u64>>`
  |             |
  |             expected due to this
  |
  = note: expected type `u32`
           found struct `StepBy<std::slice::Iter<'_, u64>>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

rustc names the wrapper-struct type: `StepBy<Iter<'_, u64>>` inline,
`StepBy<std::slice::Iter<'_, u64>>` in the secondary note. Same
forced-error type-pin technique 134/135/136/137/138/139 used; today
substitutes `StepBy` for `Take`/`Skip`/`Enumerate`/`Fuse`. The struct
is referred to opaquely; today's lesson does not unpack its private
fields.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md:25-26` (synopsis-box line)

Verbatim:

```text
    fn step_by(self, step: usize) -> StepBy<Self> ⓘ
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape — every
iterator inherits `step_by` automatically.

### `output/docs/rust/std/iter/trait.Iterator.md:551` (per-method declaration)

Verbatim:

```text
#### fn [step_by](#method.step_by)(self, step: [usize](../primitive.usize.md)) -> [StepBy](struct.StepBy.md "struct std::iter::StepBy")<Self> [ⓘ](#) where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

Authoritative source for:

- **Method name** `step_by` and **receiver shape** `(self, step:
  usize)` — bare `self` (lesson 102 consuming) plus a `usize` second
  parameter (same slot as 136's `n`/137's `n`).
- **Return type** `StepBy<Self>` — wrapper struct documented at
  `struct.StepBy.md`.
- **`where Self: Sized`** — same bound `take` / `skip` / `enumerate`
  / `fuse` carry; named-deferred today.

### `output/docs/rust/std/iter/trait.Iterator.md:553-565` (prose summary)

Verbatim:

```text
Creates an iterator starting at the same point, but stepping by
the given amount at each iteration.

Note 1: The first element of the iterator will always be returned,
regardless of the step given.

Note 2: The time at which ignored elements are pulled is not fixed.
`StepBy` behaves like the sequence `self.next()`, `self.nth(step-1)`,
`self.nth(step-1)`, …, but is also free to behave like the sequence
`advance_n_and_return_first(&mut self, step)`,
`advance_n_and_return_first(&mut self, step)`, …
Which way is used may change for some iterators for performance reasons.
The second way will advance the iterator earlier and may consume more items.
```

Grounds:

- **"Creates an iterator starting at the same point, but stepping by
  the given amount at each iteration"** — the centered structural
  semantic. Probe 1 left half (indices `0, 2, 4`).
- **"The first element of the iterator will always be returned,
  regardless of the step given"** — Probe 4 (`step_by(100)` on a
  3-element vec yields just the first).
- **"The time at which ignored elements are pulled is not fixed"** —
  named-deferred. The two pull strategies are unobservable to the
  caller; today does not depend on either.

### `output/docs/rust/std/iter/trait.Iterator.md:582-584` (Panics note)

Verbatim:

```text
##### Panics

The method will panic if the given step is `0`.
```

The load-bearing precondition. Probe 2 (three sub-probes) is the
empirical witness, and additionally pins down that the panic fires
at construction (not at first `.next()`).

### `output/docs/rust/std/iter/trait.Iterator.md:586-596` (Examples)

Verbatim:

```text
##### Examples

let a = [0, 1, 2, 3, 4, 5];
let mut iter = a.into_iter().step_by(2);

assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), None);
```

The std example uses `into_iter()` (today named-deferred since 022)
on an array literal; today's Probe 1 substitutes `Vec::iter()` (123)
on a `Vec` literal — the choice keeps the lesson within already-
installed prereqs and produces `&u64` references rather than `u64`
values, but the structural witness (indices `0, 2, 4` reached, then
`None`) is identical.

### `output/docs/rust/std/iter/struct.StepBy.md:7-12`

Verbatim:

```text
pub struct StepBy<I> { /* private fields */ }

Expand description

An iterator for stepping iterators by a custom amount.

This `struct` is created by the [`step_by`](trait.Iterator.md#method.step_by "method std::iter::Iterator::step_by") method on [`Iterator`](trait.Iterator.md). See
its documentation for more.
```

Grounds the wrapper-struct claim. Same opaque-struct treatment as
`Take<I>` (136), `Skip<I>` (137), `Enumerate<I>` (138), `Fuse<I>`
(139).

### `output/docs/rust/error_codes/E0308.md`

Probe 5's diagnostic. Type-pin technique installed at lessons
134-139.

## Claim-to-evidence map

- "`step_by` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:25-26` synopsis-box line ends in `{ ... }`
  (lesson 116's default-body marker); lesson 132 evidence appendix.
- "Signature `fn step_by(self, step: usize) -> StepBy<Self> where
  Self: Sized,`" — `trait.Iterator.md:551` (per-method declaration).
- "`step_by(step)` yields the element at indices `0, step, 2*step,
  …`" — `trait.Iterator.md:553-557`; Probe 1 left half (indices
  `0, 2, 4` of a 5-element vec yield three lines `10, 30, 50`); std
  example at `:589-595`.
- "The first element is always returned regardless of step" —
  `trait.Iterator.md:556-557`; Probe 4 (`step_by(100)` on a 3-
  element vec yields just the first).
- "`step_by(0)` panics" — `trait.Iterator.md:582-584`; Probe 2 (three
  sub-probes capture identical panic).
- "The panic fires at construction, not at first `.next()`" — Probe
  2a (`let _ = v.iter().step_by(0);` with no `.next()` call still
  panics); also confirmed by panic location
  `library/core/src/iter/adapters/step_by.rs:35:9` (the std-library
  source path is the `step_by` body, not `StepBy::next`).
- "The panic shape is lesson 053's template" — `evidence/053-result-
  expect-and-panic.md` Probe 2 transcript; today's Probe 2a
  transcript reproduces every line of that template (panic header,
  message, RUST_BACKTRACE note, exit 101).
- "Return type `StepBy<Self>` is itself an iterator" —
  `trait.Iterator.md:551` (signature names `StepBy<Self>`);
  `struct.StepBy.md:7,12-13` (struct + prose); Probe 1 (the `for`
  loop and the chained `.count()` both work on the wrapper); Probe 5
  (rustc names the type `StepBy<Iter<'_, u64>>`).
- "`step_by(1)` is the identity adapter" — Probe 3 (every element of
  the vec is yielded); follows mechanically from "stepping by the
  given amount" + `:556-557`.
- "rustc spells the wrapper type `StepBy<Iter<'_, u64>>` /
  `StepBy<std::slice::Iter<'_, u64>>`" — Probe 5 (forced E0308
  inline + secondary note labels).

## Negative / contrast probe coverage

Five probes captured (one centered working, three contrast/
corroboration, one type-pin):

- **Probe 1 (working, for + count)** — centered demonstration of
  `step_by(2)` semantics on a familiar `Vec<u64>` source.
- **Probe 2 (three sub-probes for the panic precondition)** — *the
  centered new-fact probe.* The panic fires at construction; the
  three sub-probes (no-`.next()`, with-`.next()`, bare-statement)
  triangulate that the precondition check is unconditional and lives
  in the `step_by` constructor body, not in `StepBy::next`.
- **Probe 3 (`step_by(1)` identity)** — corroboration that the
  structural claim "yields every step-th element starting at 0"
  reduces correctly when `step == 1`.
- **Probe 4 (`step_by(100)` past length)** — corroboration of the
  `:556-557` "first element always returned" rule, and a contrast
  with Probe 2's panic (a different "edge" — large step does not
  panic).
- **Probe 5 (type-pin via E0308)** — names the wrapper-struct type
  from rustc's mouth: `StepBy<Iter<'_, u64>>` /
  `StepBy<std::slice::Iter<'_, u64>>`.

**No centered E0382 today.** The consuming-`self` rule is well-
installed by lessons 102/133/134/136/137/138/139 (seven prior
captures). Today's signature reads `self`; the rule applies.

**Not re-witnessing laziness today.** Lesson 136's Probe 2
(Trace + take) installed the laziness shape for the adapter family.
Today's centering is on the panic precondition, not laziness — though
Probe 2c's `unused_must_use` warning *does* happen to corroborate the
laziness framing ("iterators are lazy and do nothing unless
consumed"). That corroboration is incidental.

## Iterator API audit alignment

This lesson is step 10 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 10. **`step_by`** — `step != 0` panic.

Audit §4.3 lists `step_by` as ready-now, composing "self-by-value 102
+ 080". Today executes that move per audit §5 step 10. Lesson 139's
unlock list named today's move:

> future "`Iterator::step_by` — `(self, step) -> StepBy<Self>`,
> panics on `step == 0`" moves (audit §5 step 10 — same `n: usize`-
> shaped second parameter from 136/137 returns, but with a panic
> precondition; reuses today's lazy-adapter frame)

The new graph fact today: where lessons 136-139 installed adapters
that change *what or after-what the wrapper yields* (count, count,
element shape, post-`None` semantics), today installs **the first
adapter with a runtime panic precondition**. The panic check is
*not* lazy — it fires at construction, not at first `.next()`. This
is the run's first appearance of an "argument validation happens
eagerly even though iteration is lazy" pattern, which generalizes to
many other std assertions on adapter constructors. Unlocks
`size_hint` (audit §5 step 11), the last closure-free non-consumer
Iterator surface move.
