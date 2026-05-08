# Evidence — Lesson 124: `.rev()` on the iterator from `Vec<T>::iter()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/124-iter-rev.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/124-iter-rev.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/124-iter-rev.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet124/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons 122
and 123.

## Direct prerequisite — lesson 123 (`v.iter()` returns an iterator)

Lesson 123 installed:

- `v.iter()` is a method on `Vec<T>` returning a value of type
  `std::slice::Iter<'_, T>` — an iterator (not a `T`).
- The iterator yields each element of the vec in order, as `&T`.
- `for x in v.iter() { ... }` runs the body once per element with
  `x: &T`.

Today's lesson chains `.rev()` on the result of that call. The
receiver of `.rev()` is exactly the value lesson 123 named
`Iter<'_, u64>`. The element type and yielding semantics are
unchanged; only the order is reversed.

## Direct prerequisite — lesson 091 (`.rev()` on a range)

Lesson 091 installed:

- `.rev()` is a method, attached via the lesson-040 dot-call shape,
  with empty argument list `()`.
- On a parenthesized range, `.rev()` produces an iteration in
  *reversed* order: `(1..4).rev()` yields `3, 2, 1`.
- The trait machinery (`Iterator`, `DoubleEndedIterator`) is named in
  91's "What To Ignore For Now" and explicitly deferred — `.rev()` is
  treated as a name-and-use atom.

Today extends the *same method name* `.rev()` to a different receiver
type — the iterator from `.iter()` instead of a range. Probe 1's
output `30/20/10` from `vec![10, 20, 30]` is the empirical witness
that the same reversal semantics apply.

## Direct prerequisite — lesson 049 (chained dot-calls)

Lesson 049 installed `expr.method1().method2()`: a left-to-right chain
where each call's return value becomes the receiver of the next call.
`v.iter().rev()` fills the shape exactly:

- `v.iter()` evaluates first. Receiver `v: Vec<u64>`, method `iter`,
  empty arg list. Returns `Iter<'_, u64>`.
- `.rev()` then evaluates on that returned iterator value. Same
  shape, empty arg list. Returns `Rev<Iter<'_, u64>>` (per
  `std/iter/trait.Iterator.md:2990` signature
  `fn rev(self) -> Rev<Self>`).

Today does not surface the `Rev<I>` type name in the lesson body; the
empirical handle the learner uses is "the chained iterator that walks
backwards."

## Older supporting lessons

- **Lessons 040, 011, 001, 002, 003, 005, 080, 019** — same roles as
  in lessons 123 and 122: dot-call grammar; `println!`; rustc compile
  and run; `fn main`; the diagnostic four-part map; `let`; `u64`; the
  `: TYPE` annotation slot.
- **Lesson 079** — `for X in COLLECTION { ... }` over a runtime
  collection. Today's COLLECTION is `v.iter().rev()`.
- **Lesson 100** — installed E0599 "no method named X found for type
  Y" diagnostic shape. Today's contrast probe (Probe 3) reuses that
  diagnostic shape on a `Vec<u64>` receiver.
- **Lesson 107** — `Vec<T>` construction with `vec![]`.

## Probe 1 — working probe (`v.iter().rev()`)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/124-iter-rev.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter().rev() {
        println!("{}", x);
    }
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
30
20
10
$ echo "run-exit=$?"
run-exit=0
```

The centered claim — "`.rev()` chained on the iterator from `.iter()`
yields the vec's elements in reversed order" — is carried by the
output. The `vec![10, 20, 30]` literal defines vec order; the printed
sequence `30/20/10` is the same elements in reverse vec order.

## Probe 2 — order-swap contrast (no `.rev()`)

`forward.rs` is the same source minus `.rev()`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter() {
        println!("{}", x);
    }
}
```

Transcript:

```text
$ rustc forward.rs
$ ./forward
10
20
30
```

The output is exactly lesson 123's working probe — `10/20/30` in vec
order. `diff` between Probe 1 and Probe 2 outputs:

```text
$ diff <(./demo) <(./forward)
1,2d0
< 30
< 20
3a2,3
> 20
> 30
```

Same three elements; the only difference is order. This is the
positive empirical witness for "`.rev()` reverses the iteration order
on the iterator from `.iter()`."

## Probe 3 — diagnostic contrast (`v.rev()` directly)

`broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let r = v.rev();
    println!("{:?}", r);
}
```

Transcript verbatim:

```text
$ rustc broken.rs
error[E0599]: no method named `rev` found for struct `Vec<u64>` in the current scope
 --> broken.rs:3:15
  |
3 |     let r = v.rev();
  |               ^^^ `Vec<u64>` is not an iterator
  |
help: call `.into_iter()` first
  |
3 |     let r = v.into_iter().rev();
  |               ++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
exit=1
```

Lesson 100's E0599 shape applied to `Vec<u64>` as receiver. The
inline label "`Vec<u64>` is not an iterator" states today's structural
fact directly: `.rev()` is a method on iterators, not on `Vec<T>`.

The `help:` line surfaces `v.into_iter().rev()` as a fix. That
variant is the consuming form of producing an iterator; lesson 123's
"What To Ignore For Now" named `.into_iter()` as deferred. Today's
lesson body does not center the help line; it points at it as a
deferred sibling.

This is the lesson's *centered* contrast: it has a verbatim
diagnostic that names the boundary ("Vec<T> is not an iterator"),
which Probe 2's silent order-swap does not. Both probes are captured
because they witness *different* facets — Probe 2 the positive
order-reversal, Probe 3 the structural type-vs-method boundary.

## Probe 4 — corroborating (different vec, same mechanic)

`corrob.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![1, 2, 3, 4, 5];
    for x in v.iter().rev() {
        println!("{}", x);
    }
}
```

Transcript:

```text
$ rustc corrob.rs
$ ./corrob
5
4
3
2
1
```

A different vec (length 5, values `[1, 2, 3, 4, 5]`) produces the
elements in reverse vec order: `5, 4, 3, 2, 1`. Corroborates that the
mechanic is general — not coupled to Probe 1's specific values.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 2990-2998

Verbatim:

> #### fn rev(self) -> Rev<Self> where Self: Sized + DoubleEndedIterator,
>
> Reverses an iterator's direction.
>
> Usually, iterators iterate from left to right. After using `rev()`,
> an iterator will instead iterate from right to left.
>
> This is only possible if the iterator has an end, so `rev()` only
> works on `DoubleEndedIterator`s.

This is the authoritative description of the method today centers.
Two facts:

- *signature*: `fn rev(self) -> Rev<Self>` — `.rev()` consumes the
  receiver iterator and returns a new iterator of type `Rev<Self>`.
  Today does not surface the return type in the lesson body.
- *bound*: `where Self: Sized + DoubleEndedIterator` — `.rev()` is
  callable only on iterators that implement `DoubleEndedIterator`.
  `Iter<'_, T>` does (see next section). Today names the trait as
  deferred.

The semantics — "iterate from right to left" — is the load-bearing
claim. Probe 1's output `30/20/10` from `vec![10, 20, 30]` is the
empirical witness on the `Iter<'_, T>` receiver; lesson 091 carried
the same claim on a `Range` receiver.

### `output/docs/rust/std/slice/struct.Iter.md` lines 131-137

Verbatim header line (131-133):

> ### impl<'a, T> DoubleEndedIterator for Iter<'a, T>

And the first method (line 137):

> #### fn next_back(&mut self) -> Option<&'a T>

This is the trait impl that makes `.rev()` callable on `v.iter()`'s
return value. `Iter<'a, T>` (the type lesson 123 surfaced from
rustc's diagnostic) implements `DoubleEndedIterator`, satisfying the
`where`-bound on `Iterator::rev` quoted above. Today does not center
the trait machinery; this section is the structural grounding for
"the same `.rev()` works because `Iter<'_, T>` is double-ended, just
like a `Range<i32>` is."

### `output/docs/rust/std/iter/struct.Rev.md` lines 1-15

Verbatim:

> # Struct Rev
>
> ```
> pub struct Rev<T> { /* private fields */ }
> ```
>
> A double-ended iterator with the direction inverted.
>
> This `struct` is created by the `rev` method on `Iterator`.

The page that exists for the return type `Rev<I>`. Today's lesson
body does not name the type — the learner-facing handle is "a new
iterator that walks backwards." This page grounds the "name exists"
fact only.

### `output/docs/rust/error_codes/E0599.md` line 4

Verbatim:

> This error occurs when a method is used on a type which doesn't
> implement it

Probe 3's diagnostic is exactly an instance: `Vec<u64>` is the type;
`rev` is the method; `Vec<u64>` does not implement an `.rev()` method
because `Vec<u64>` is not an iterator (the inline label states this
directly). Today reuses lesson 100's E0599 reading discipline; the
appendix points at the error-code doc for completeness.

## rmp unlock — `cmp.rs:22` `self.limbs.iter().rev().zip(...)`

Source `output/repos/rmp/src/biguint/cmp.rs` line 22 verbatim:

```rust
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
```

Lesson 123 made the *first link* — `self.limbs.iter()` — readable.
Today makes the *second link* — `.rev()` chained on that iterator —
readable. The chain so far parses as: `self.limbs` (field access on a
`Vec<u64>` per lesson 095), `.iter()` (lesson 123), `.rev()` (today).
The remainder — `.zip(other.limbs.iter().rev())` and the
`for (left, right) in ...` destructuring — composes future moves.

## Claim-to-evidence map

- "`.rev()` is callable on the iterator returned by `Vec<T>::iter()`"
  — `std/iter/trait.Iterator.md:2990` signature
  `fn rev(self) -> Rev<Self> where Self: Sized + DoubleEndedIterator`;
  `std/slice/struct.Iter.md:131-133`
  `impl<'a, T> DoubleEndedIterator for Iter<'a, T>` (witnesses the
  bound is satisfied); Probe 1 silent compile is the empirical
  witness.
- "The chained iterator yields the same elements as `v.iter()`, in
  reversed order" — `trait.Iterator.md:2992-2995` "Reverses an
  iterator's direction. ... an iterator will instead iterate from
  right to left"; Probes 1 + 2 transcripts (same elements, order
  swapped per `diff`).
- "`v.iter().rev()` is the chained-dot-call shape" — lesson 049's
  rule, applied here with empty argument lists; Probe 1 silent
  compile.
- "`.rev()` is not a method on `Vec<T>` itself" — Probe 3 transcript
  verbatim ("`Vec<u64>` is not an iterator"); `error_codes/E0599.md:4`.
- "`for x in v.iter().rev() { ... }` binds `x: &T` and runs the body
  once per element" — lesson 123 grounded the binding type and the
  per-element semantics; today's reversal does not change them.
  Probe 1 transcript (three lines, three elements) is the empirical
  witness.
- "rmp `cmp.rs:22` `.iter().rev()` first two links readable" —
  `output/repos/rmp/src/biguint/cmp.rs:22` verbatim; the chain
  matches Probe 1's `v.iter().rev()` shape on a `Vec<u64>` receiver.

## Negative / contrast probe coverage

Two contrasts captured. Both are needed:

- **Probe 2 (order-swap)** is the positive empirical witness for the
  reversal claim. Same source minus `.rev()` produces vec order; with
  `.rev()` produces reversed vec order. No diagnostic — the contrast
  is in the runtime output. The `diff` transcript pins the *only*
  difference to ordering.
- **Probe 3 (E0599 on `v.rev()`)** is the structural-boundary
  contrast. `.rev()` is not a method on `Vec<T>`; you must produce an
  iterator first. The diagnostic states this verbatim ("`Vec<u64>` is
  not an iterator"). The lesson body centers this contrast because
  it has a clean diagnostic; Probe 2 is referenced via the toolchain
  output description in The Move.

Probe 4 is corroborative, not contrastive — it witnesses the mechanic
on a different vec.
