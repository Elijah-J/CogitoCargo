# Evidence — Lesson 136: limit a slice iterator with `iter.take(n)`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/136-iterator-take.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/136-iterator-take.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/136-iterator-take.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet136/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
135.

## Direct prerequisite — lesson 135 (`Iterator::nth`)

Lesson 135 installed:

- A `usize` second-parameter slot on a *provided* Iterator method.
  Today reuses that slot in `take(self, n: usize)`.
- The receiver-shape contrast: `nth`'s `&mut self` (binding survives)
  vs `count` and `last`'s `self` (binding consumed). Today returns to
  the consuming-`self` shape; Probe 5's E0382 is the witness.

The new fact today is the *return* slot. `nth` returned
`Option<Self::Item>` — a wrapper around an element. `take` returns
`Take<Self>` — *another iterator value*. That difference is the
adapter-vs-consumer split lesson 132's "What To Ignore" listed as
deferred.

## Direct prerequisite — lesson 134 (`Iterator::last`)

`.last()` is the consumer chained on `take(3).last()` in the working
probe (output line 2: `Some(30)`). Lesson 134 installed `last`'s
signature `(self) -> Option<Self::Item>` and the `Self::Item = &u64`
substitution for slice iterators. Today's `take(3).last()` returns
`Option<&u64>` (Probe 1 output `Some(30)` — Debug format hides the
`&` for primitive targets, exactly as 134 captured).

## Direct prerequisite — lesson 133 (`Iterator::count`)

`.count()` is the consumer chained on `take(2).count()` in the working
probe (output line 1: `2`). Lesson 133 installed `count`'s
`(self) -> usize` signature and the centered E0382 + `note:` template
for "this stdlib method consumes its receiver." Today's Probe 5 reuses
that template verbatim, only `take` substitutes into the method-name
slot.

## Direct prerequisite — lesson 132 (the `Iterator` trait declaration)

Lesson 132 installed:

- `std::iter::Iterator` declares `type Item;` and `fn next(&mut self)
  -> Option<Self::Item>;` as required, plus 75 provided methods.
- An impl supplying only the required surface inherits all 75 provided
  methods through default bodies (lesson 116).

`take` appears on the synopsis box at `trait.Iterator.md:66-67` as
`fn take(self, n: usize) -> Take<Self>` with `where Self: Sized` —
the `{ ... }` body marker is lesson 116's default-body shape. The
per-method declaration at `:1376` confirms the signature.

Lesson 132's "What To Ignore" listed *"the formal 'lazy adapter' /
'consumer' definitions"* as deferred. Today installs the lazy/adapter
framing for `take`.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed `.next()` on a slice iterator returning
`Option<&T>` plus the `let mut iter` rule. Today's Probe 4 calls
`.next()` on a `Take<Self>` value — directly witnessing that the
binding *is* an iterator. The Trace iterator in Probe 2 also reuses
lesson 131's `&mut self` mechanic on `next`.

## Direct prerequisite — lesson 102 (`self`-by-value receiver)

Lesson 102 installed `self` (no `&`, no `mut`) as the consuming
receiver shape: calling such a method moves the receiver, and using
the binding afterward fires E0382 with a `note:` block at the method-
definition site. Today's Probe 5 fires E0382 on `Iterator::take` with
the same `note:` template lesson 133 captured for `count` and 134
captured for `last`.

## Direct prerequisite — lesson 049 (method chaining)

Lesson 049 installed: a method-call's receiver can be any expression,
including another method call. The chain parses left-associatively.
Today's `v.iter().take(2).count()` is the first chain in the run that
puts a *consumer* (`count`) after a *non-consumer* iterator method
(`take`). The chain is parsed `((v.iter()).take(2)).count()` — `take`
runs on the slice iterator, then `count` runs on the resulting
`Take<Iter<'_, u64>>`.

## Older supporting lessons

- **Lesson 080** (cited) — `usize` is one specific row of the twelve
  integer types. `take`'s second parameter slot is `usize`.
- **Lesson 123** (cited) — `v.iter()` returns the slice iterator; the
  receiver in today's working probe.
- **Lesson 022 / 079** (cited) — `for x in iter` works on any
  Iterator; today reuses that on `for x in v.iter().take(2)`.
- **Lesson 116** (cited) — default-body trait methods. The synopsis-
  box line `fn take(self, n: usize) -> Take<Self> where Self: Sized
  { ... }` ends in `{ ... }`, the lesson-116 default-body marker.
- **Lessons 040, 011, 005, 003, 002, 001** (cited) — dot-call;
  `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.

## Probe 1 — working probe (chain: take + count, take + last, for + take)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/136-iterator-take.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().take(2).count();
    println!("{}", n);
    let last = v.iter().take(3).last();
    println!("{:?}", last);
    for x in v.iter().take(2) {
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
2
Some(30)
10
20
$ echo "run-exit=$?"
run-exit=0
```

Three claims simultaneously witnessed:

1. **`.count()` chains onto `.take(2)`.** Output line 1 is `2` —
   `take(2).count()` counts elements yielded by the wrapper (which is
   the first two elements of the inner iter), not all elements of the
   inner iter. If `take` did nothing, the count would be `5`.
2. **`.last()` chains onto `.take(3)`.** Output line 2 is `Some(30)`
   — `take(3).last()` walks the wrapper and returns the most-recent
   `Some(_)`. If `take` did nothing, `last` would return `Some(50)`.
3. **A `for` loop drives `.take(2)` directly.** Output lines 3-4 are
   `10` and `20` — the `for` walks the `Take<Self>` value (which
   yields `&10, &20, None`); the body prints each element with `{}`.
   If `take` did nothing, lines 3-4 would extend to all five elements.

This is the structural witness that `Take<Self>` is itself an iterator
that participates in every iterator-driving form lessons 131, 133,
134 installed.

## Probe 2 — laziness witness (Trace + take, no eager walk)

```rust
struct Trace { n: u32 }
impl Iterator for Trace {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        println!("next:{}", self.n);
        let v = self.n;
        self.n += 1;
        Some(v)
    }
}

fn main() {
    println!("--- step A: build .take(2), do not iterate");
    let _wrapped = Trace { n: 0 }.take(2);
    println!("--- end step A");

    println!("--- step B: one .next() on a fresh take(2)");
    let mut wrapped_b = Trace { n: 0 }.take(2);
    let v = wrapped_b.next();
    println!("got: {:?}", v);
    println!("--- end step B");

    println!("--- step C: .count() on a fresh take(2)");
    let c = Trace { n: 0 }.take(2).count();
    println!("count = {}", c);
    println!("--- end step C");
}
```

Transcript (verbatim):

```text
$ rustc lazy.rs
compile-exit=0
$ ./lazy
--- step A: build .take(2), do not iterate
--- end step A
--- step B: one .next() on a fresh take(2)
next:0
got: Some(0)
--- end step B
--- step C: .count() on a fresh take(2)
next:0
next:1
count = 2
--- end step C
run-exit=0
```

Empirical findings, line-by-line:

- **Step A: zero `next:` lines** between markers. Building the
  `Take<Self>` value did *not* call `next` on the inner iterator
  even once. This is the laziness witness.
- **Step B: exactly one `next:0` line**, then `got: Some(0)`. Calling
  `.next()` on the wrapper pulled exactly once on the inner iterator.
- **Step C: exactly two `next:` lines** (`next:0`, `next:1`), then
  `count = 2`. `.count()` pulled `n = 2` times on the inner iterator.

This carries the "lazy adapter" claim empirically: `take(n)` does not
do any work itself; it builds a wrapper that, when driven, drives the
inner iterator at most `n` times.

The Trace iterator infinitely yields `Some(K)` for K = 0, 1, 2, ...
That is safe here because Step C's `.count()` only pulls until the
take limit (`n = 2`); without `take`, `Trace { n: 0 }.count()` would
loop forever (or panic on overflow).

## Probe 3 — take(huge) corroboration

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().take(100).count();
    println!("{}", n);
}
```

Compile silent; run prints `5`; exit 0. Witnesses: `take(n)` for
`n >= remaining length` does *not* panic and does *not* yield 100
phantom elements. The wrapper exits early (its inner iter returns
`None` after five elements) and the count is `5`.

Matches the corpus prose at `trait.Iterator.md:1383-1385`: "The
returned iterator is a prefix of length `n` if the original iterator
contains at least `n` elements, otherwise it contains all of the
(fewer than `n`) elements of the original iterator."

## Probe 4 — `Take<Self>` is itself iterable (.next() works on it)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut t = v.iter().take(2);
    println!("{:?}", t.next());
    println!("{:?}", t.next());
    println!("{:?}", t.next());
}
```

Compile silent; run prints `Some(10)`, `Some(20)`, `None`; exit 0.
Witnesses: the binding `t` carries an Iterator value (a `Take<Iter<'_,
u64>>`), so `.next()` is callable through the `Iterator` trait. After
`n = 2` calls return `Some(_)`, subsequent calls return `None`.

The `let mut t` is required because `.next()` takes `&mut self`
(lesson 131). This is structurally the same pattern as lesson 131's
working probe, except the receiver type is the `Take` wrapper rather
than the bare slice iterator.

## Probe 5 — centered contrast (E0382 on consumed receiver)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let _t1 = iter.take(2);
    let _t2 = iter.take(2);
}
```

Verbatim diagnostic:

```text
error[E0382]: use of moved value: `iter`
 --> consume.rs:7:15
  |
5 |     let iter = v.iter();
  |         ---- move occurs because `iter` has type `std::slice::Iter<'_, u64>`, which does not implement the `Copy` trait
6 |     let _t1 = iter.take(2);
  |                    ------- `iter` moved due to this method call
7 |     let _t2 = iter.take(2);
  |               ^^^^ value used here after move
  |
note: `std::iter::Iterator::take` takes ownership of the receiver `self`, which moves `iter`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:1414:12
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
6 |     let _t1 = iter.clone().take(2);
  |                   ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
exit=1
```

The centered contrast for "`take` consumes its receiver." Three
structural alignments to lesson 133's E0382 contrast (and 134's):

1. **Same E-code** `E0382: use of moved value`.
2. **Same `note:` shape**: `\`std::iter::Iterator::take\` takes
   ownership of the receiver \`self\`, which moves \`iter\``. Method-
   name slot has `take` substituted; everything else identical.
3. **Same `help:` proposing `.clone()`** with `++++++++` insert
   marker.

The diagnostic literally proves `trait.Iterator.md:1376` reads `fn
take(self, ...)` (no `&`, no `mut`): if the receiver were `&mut self`
or `&self`, the second call would not move-fail.

The `-->` of the note points at core's
`library/core/src/iter/traits/iterator.rs:1414:12` — different line
than `count`'s 225:13 and `last`'s 258:12 because each provided method
has its own definition site in core.

## Probe 6 — type-pin via E0308 (rustc names the actual type)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: u32 = v.iter().take(2);
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typeprobe.rs:3:19
  |
3 |     let _x: u32 = v.iter().take(2);
  |             ---   ^^^^^^^^^^^^^^^^ expected `u32`, found `Take<Iter<'_, u64>>`
  |             |
  |             expected due to this
  |
  = note: expected type `u32`
           found struct `std::iter::Take<std::slice::Iter<'_, u64>>`
```

rustc spells the actual return type out: `Take<Iter<'_, u64>>` inline,
`std::iter::Take<std::slice::Iter<'_, u64>>` in the secondary note.
The `Take<...>` struct is referenced opaquely; today's lesson does
not unpack its private fields.

This is a forced-error type-pin probe: bind the chain to a wildly
wrong type so rustc prints the real one in the `expected/found`
labels. The technique is identical to lessons 134 Probe 5 and 135
Probe 6, only the bound type is `u32` (instead of `Option<u64>`)
because today's return is *not* an `Option<...>`.

## Probe 7 — std doc replay (adapted from `:1391-1399`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter().take(2);
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
```

Compile silent; run prints `Some(10)`, `Some(20)`, `None`; exit 0.
Mirrors the std doc example at `trait.Iterator.md:1391-1399`:

```text
let a = [1, 2, 3];
let mut iter = a.into_iter().take(2);
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);
```

modulo `[i32; 3].into_iter()` (yields `i32` directly) vs
`Vec<u64>.iter()` (yields `&u64`, Debug-printed without the `&`).
The behavioral pattern is identical: two `Some(_)` payloads then
`None` after the take limit.

## Probe 8 — empty input (count = 0, last = None)

```rust
fn main() {
    let v: Vec<u64> = vec![];
    let n = v.iter().take(2).count();
    println!("count = {}", n);
    let l = v.iter().take(2).last();
    println!("last  = {:?}", l);
}
```

Compile silent; run prints `count = 0` and `last  = None`; exit 0.
Witnesses: `take(2)` on a zero-element source yields zero elements.
The wrapper's inner iter exhausts before two pulls happen.

## Probe 9 — `take(0)` is empty (no pull at all)

```rust
struct Trace { n: u32 }
impl Iterator for Trace {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        println!("next:{}", self.n);
        let v = self.n;
        self.n += 1;
        Some(v)
    }
}
fn main() {
    let c = Trace { n: 0 }.take(0).count();
    println!("count = {}", c);
}
```

Compile silent; run prints `count = 0`; **zero `next:` lines** to
stdout; exit 0. Witnesses: `take(0).count()` on an *infinite* Trace
iterator terminates with `0` and prints zero `next:` lines. The
algorithm exits before pulling once.

This corroborates Probe 2's laziness witness from a different angle:
not only does building the `Take` value not pull, even *driving*
`take(0)` does not pull (because the count is exhausted on the first
loop iteration).

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 66-67 (synopsis-box line)

Verbatim:

```
    fn take(self, n: usize) -> Take<Self> ⓘ
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape — what
licenses every iterator inheriting `take` for free.

### `output/docs/rust/std/iter/trait.Iterator.md` line 1376 (per-method declaration)

Verbatim:

```
#### fn [take](#method.take)(self, n: [usize](../primitive.usize.md)) -> [Take](struct.Take.md "struct std::iter::Take")<Self> [ⓘ](#) where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

Authoritative source for:

- **Method name** `take` and **receiver shape** `(self, n: usize)` —
  bare `self` (lesson 102 consuming) followed by `n: usize` (lesson
  080).
- **Return type** `Take<Self>` — the wrapper struct documented at
  `output/docs/rust/std/iter/struct.Take.md`.
- **`where Self: Sized`** — same bound `count` and `last` carry;
  named-deferred today.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1378-1385 (prose summary)

Verbatim:

```
Creates an iterator that yields the first `n` elements, or fewer
if the underlying iterator ends sooner.

`take(n)` yields elements until `n` elements are yielded or the end of
the iterator is reached (whichever happens first).
The returned iterator is a prefix of length `n` if the original iterator
contains at least `n` elements, otherwise it contains all of the
(fewer than `n`) elements of the original iterator.
```

This grounds:

- "**Yields the first `n` elements**" — Probe 1 line 1 (count = 2 for
  `take(2)`); Probe 4 (two `Some(_)` then `None`); Probe 7 (std doc
  replay).
- "**Or fewer if the underlying iterator ends sooner**" — Probe 3
  (`take(100)` on five-element iter prints `5`); Probe 8 (empty input
  prints `count = 0`).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1391-1399 (basic example)

Verbatim:

```
##### Examples

Basic usage:

```
let a = [1, 2, 3];

let mut iter = a.into_iter().take(2);

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);
```
```

Probe 7 replays this verbatim modulo source-collection shape.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1412-1421 (less-than-`n` example)

Verbatim:

```
If less than `n` elements are available,
`take` will limit itself to the size of the underlying iterator:

```
let v = [1, 2];
let mut iter = v.into_iter().take(5);
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);
```
```

Probe 3 corroborates this with `take(100)` on a five-element vec.

### `output/docs/rust/std/iter/struct.Take.md` lines 1-15

Verbatim:

```
# Struct Take

1.0.0 ·

```
pub struct Take<I> { /* private fields */ }
```

Expand description

An iterator that only iterates over the first `n` iterations of `iter`.

This `struct` is created by the [`take`](trait.Iterator.md#method.take "method std::iter::Iterator::take") method on [`Iterator`](trait.Iterator.md). See its
documentation for more.
```

Today's claim "`Take<Self>` is itself an iterator" rests on the
`struct.Take.md` page, which lists `impl<I: Iterator> Iterator for
Take<I>` further down. Today does not enumerate the impl list; the
opaque "this struct implements Iterator" claim is grounded by the
authoritative struct doc page.

### `output/docs/rust/std/iter/trait.Iterator.md` line 1443 (adapter terminology)

Verbatim (from `scan`'s prose):

```
An iterator adapter which, like `fold`, holds internal state, but
unlike `fold`, produces a new iterator.
```

Grounds the term "iterator adapter" used in the lesson body and in
the audit's terminology.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 866-867, 892 (lazy framing)

Verbatim (from `map`'s prose):

```
`map()` is conceptually similar to a `for` loop. However, as `map()` is
lazy, it is best used when you're already working with other iterators.
```

```
// it won't even execute, as it is lazy. Rust will warn you about this.
```

Grounds the term "lazy" as applied to iterator adapters generally.
The framing transfers to `take` because both are listed in the
synopsis box's *Provided methods* block returning a wrapper struct;
the page treats them as one family.

### `output/docs/rust/error_codes/E0382.md`

Probe 5's diagnostic. Lessons 133 and 134 already installed the
diagnostic shape on `count` and `last`; today's contrast reuses it
on `take`.

### `output/docs/rust/error_codes/E0308.md`

Probe 6's diagnostic. Type-pin technique installed at lessons 134
Probe 5 and 135 Probe 6.

## Claim-to-evidence map

- "`v.iter().take(2).count()` on `vec![10, 20, 30, 40, 50]` prints
  `2`" — Probe 1 line 1.
- "`v.iter().take(3).last()` on the same vec prints `Some(30)`" —
  Probe 1 line 2.
- "`for x in v.iter().take(2)` on the same vec prints `10` then `20`"
  — Probe 1 lines 3-4.
- "`take` takes the receiver by value (`self`); calling it moves the
  iterator" — `trait.Iterator.md:1376` (signature); Probe 5 (E0382 +
  `note:` template).
- "`n: usize` is the second parameter" — `trait.Iterator.md:1376`.
- "Return type is `Take<Self>` — itself an iterator" —
  `trait.Iterator.md:1376` (signature names `Take<Self>`);
  `struct.Take.md:1-15` (`pub struct Take<I> { ... }`); Probe 4
  (`.next()` works on the binding); Probe 6 (rustc names the type
  `Take<Iter<'_, u64>>` in the E0308 expected/found labels).
- "Building the `Take<Self>` value does not call `next` on the inner
  iterator — it is lazy" — `trait.Iterator.md:866-867,892` (corpus
  lazy framing on `map`, applies to adapters generally); Probe 2 step
  A (zero `next:` lines after building); Probe 9 (`take(0).count()`
  on infinite Trace, zero `next:` lines).
- "Driving the wrapper drives the inner iterator at most `n` times"
  — Probe 2 step B (one `next:0` after `.next()` on `take(2)`); step
  C (two `next:` lines after `.count()` on `take(2)`).
- "`take(n)` past the end yields the full iterator's elements, no
  panic" — `trait.Iterator.md:1383-1385,1412-1421`; Probe 3
  (`take(100)` on five-element vec prints `5`); Probe 8 (empty input
  prints `count = 0`).
- "`take` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:13` ("// Provided methods" comment precedes
  `take` at `:66-67`); lesson 132 evidence appendix.
- "Adapters compose with consumers via lesson-049 method chaining" —
  Probe 1 (three different consumer-after-adapter chains all compile
  and produce the expected output).

## Negative / contrast probe coverage

Three contrasts captured:

- **Probe 5 (E0382 on consumed receiver)** — centered contrast for
  `self`-by-value receiver. Without it, the claim "`take` consumes
  its receiver" rests only on corpus prose. The E0382 + matching
  `note:` template empirically aligns with lessons 133/134.
- **Probe 2 step A (zero `next:` lines after building)** — centered
  contrast for the laziness claim. The corpus prose says `map` is
  lazy at `:866-867`; today's Trace probe is the empirical witness
  *for `take` specifically*. Without this probe, the laziness claim
  would rest on transferring `map`'s prose to `take` by structural
  analogy. Probe 9's `take(0).count()` on an infinite Trace adds a
  second-angle witness: an infinite source, zero pulls.
- **Probe 6 (E0308 on `u32`)** — type-pin contrast for the
  `Take<Self>` return. Without it, the claim "the result is itself
  an iterator" rests on Probe 4's silent compile (which is consistent
  with several inferred annotations). E0308's labels naming
  `Take<Iter<'_, u64>>` and `std::iter::Take<std::slice::Iter<'_,
  u64>>` pin the actual type from rustc's mouth.

## Iterator API audit alignment

This lesson is step 6 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 6. **`take`** — small lazy adapter, returns `Take<Self>`.

Audit §4.3 lists `take` as ready-now, composing "self-by-value 102 +
080". Today executes that move per audit §5 step 6. Lessons 133, 134,
and 135 each named today's move in their unlock lists:

- 133's unlock list: *"future `Iterator::take` — `(self, n) ->
  Take<Self>`, lazy adapter" moves (audit §5 step 6 — same self-by-
  value rule, but returns a new iterator instead of consuming end-to-
  end)*.
- 134's unlock list: same item.
- 135's unlock list: same item.

The new graph fact today extends 133-135: where those installed
*consumer* return shapes (`usize`, `Option<Self::Item>`), today's
`take` is the first *adapter* — the first Iterator method whose
return is itself an iterator value, and the first place laziness is
empirically witnessed. The split established today between consumers
(`count`, `last`, `nth`) and adapters (`take`) is the operational
frame for every later Iterator method choice in audit §2's table —
opening the way for `skip`, `enumerate`, `fuse`, `step_by`, and (once
the closure arc lands) `map`, `filter`, `inspect`.
