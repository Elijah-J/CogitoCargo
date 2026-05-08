# Evidence — Lesson 137: drop the first `n` elements with `iter.skip(n)`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/137-iterator-skip.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/137-iterator-skip.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/137-iterator-skip.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet137/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
136.

## Direct prerequisite — lesson 136 (`Iterator::take`)

Lesson 136 installed three structural facts that today reuses
verbatim with `skip` substituted into the method-name slot:

- **The adapter shape** `(self, n: usize) -> WrapperStruct<Self>`.
  Today's `skip` has signature
  `fn skip(self, n: usize) -> Skip<Self> where Self: Sized,` at
  `output/docs/rust/std/iter/trait.Iterator.md:1352` — same three
  slots, with `Skip<Self>` substituted for `Take<Self>`.
- **Lazy framing**: building the wrapper does no work; the work
  happens only when something pulls. Today does *not* re-witness this
  with a Trace probe — lesson 136's Probe 2 already captured the
  laziness shape, and skip's default body in core delegates to `nth`
  per the std doc note at `:1361`. Re-probing would add appendix
  volume for negligible new fact value.
- **Chain-with-consumer**: `v.iter().take(2).count()` works and reads
  "take the slice iter, limit to two, count." Today's Probe 1 chains
  `.skip(2).count()`, `.skip(2).next()`, and `for x in v.iter()
  .skip(3)` — same three consumer-after-adapter chain shapes 136
  exercised, with `skip` substituted for `take`.

The new fact today is the **inverse semantic**: `take(n)` keeps the
first `n`; `skip(n)` discards the first `n` and yields the rest.
Probe 2 witnesses the inverse-pair identity:
`v.iter().take(2).count() + v.iter().skip(2).count() == v.iter()
.count()` (`2 + 3 == 5`).

## Direct prerequisite — lesson 134 (`Iterator::last`)

Lesson 134 installed `last`'s `(self) -> Option<Self::Item>` signature
and the `Self::Item = &u64` substitution for slice iterators. Today
does not chain `.last()` directly in the working probe (Probe 1 uses
`.next()` instead, which is also iterable on the wrapper), but the
`Self::Item = &u64` substitution carries through Probe 1 line 2
(`Some(30)` — Debug format hides `&` for primitive targets).

## Direct prerequisite — lesson 133 (`Iterator::count`)

`.count()` is the consumer chained on `.skip(2).count()` in Probe 1
(output line 1: `3`) and on the inverse-sum probe in Probe 2 (output
`take(2)=2`, `skip(2)=3`, sum `5`). Lesson 133 installed `count`'s
`(self) -> usize` signature. Probe 5's E0382 + `note:` template
reuses lesson 133's verbatim with `skip` substituted into the method-
name slot.

## Direct prerequisite — lesson 132 (the `Iterator` trait declaration)

Lesson 132 installed `std::iter::Iterator` with 75 provided methods
inheriting via default bodies. `skip` appears on the synopsis box at
`trait.Iterator.md:64-65` as `fn skip(self, n: usize) -> Skip<Self>`
with `where Self: Sized` and the `{ ... }` body marker (lesson 116's
default-body shape). The per-method declaration at `:1352` confirms
the signature.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed `.next()` on a slice iterator returning
`Option<&T>`. Today's Probe 1 calls `.next()` on a `Skip<Self>` value
and reads `Some(30)` — the binding is itself an iterator. The
mechanic transfers from the bare slice iterator to the wrapper.

## Direct prerequisite — lesson 102 (`self`-by-value receiver)

Lesson 102 installed `self` (no `&`, no `mut`) as the consuming
receiver shape. Probe 5 (appendix-only continuity) fires E0382 on
`Iterator::skip` with the same `note:` shape lessons 133/134/136
captured, only `skip` substitutes into the method-name slot.

## Direct prerequisite — lesson 049 (method chaining)

Lesson 049 installed left-associative method-chain parsing. Today's
`v.iter().skip(2).count()` parses as `((v.iter()).skip(2)).count()` —
`skip` runs on the slice iter, then `count` runs on the resulting
`Skip<Iter<'_, u64>>`.

## Older supporting lessons

- **Lesson 080** (cited) — `usize` is one specific row of the integer
  type family. `skip`'s second parameter slot is `usize`.
- **Lesson 123** (cited) — `v.iter()` returns the slice iterator.
- **Lesson 022** (cited) — `for x in iter` works on any Iterator;
  today reuses on `for x in v.iter().skip(3)`.
- **Lesson 116** (cited) — default-body trait methods. The synopsis-
  box line `fn skip(self, n: usize) -> Skip<Self> where Self: Sized
  { ... }` ends in `{ ... }`.
- **Lessons 040, 011, 005, 003, 002, 001** (cited) — dot-call;
  `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.

## Probe 1 — working probe (chain: skip + count, skip + next, for + skip)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/137-iterator-skip.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().skip(2).count();
    println!("{}", n);
    let first_remaining = v.iter().skip(2).next();
    println!("{:?}", first_remaining);
    for x in v.iter().skip(3) {
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
3
Some(30)
40
50
$ echo "run-exit=$?"
run-exit=0
```

Three claims simultaneously witnessed:

1. **`.count()` chains onto `.skip(2)`.** Output line 1 is `3` —
   `skip(2).count()` counts elements remaining after dropping the
   first two. If `skip` did nothing, the count would be `5`.
2. **`.next()` chains onto `.skip(2)`.** Output line 2 is `Some(30)`
   — after dropping `&10` and `&20`, the first remaining element is
   `&30`. If `skip` did nothing, `next` would return `Some(10)`.
3. **A `for` loop drives `.skip(3)` directly.** Output lines 3-4 are
   `40` and `50` — the `for` walks the `Skip<Self>` value (which
   yields `&40, &50, None`); the body prints each element with `{}`.
   If `skip` did nothing, lines 3-4 would extend to all five elements.

This is the structural witness that `Skip<Self>` is itself an iterator
that participates in every iterator-driving form lessons 131, 133,
134 installed, and that its semantic is the inverse of `take`.

## Probe 2 — take + skip inverse-sum on the same vec

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let total = v.iter().count();
    let kept = v.iter().take(2).count();
    let dropped_then_kept = v.iter().skip(2).count();
    println!("total={}", total);
    println!("take(2)={}", kept);
    println!("skip(2)={}", dropped_then_kept);
    println!("take+skip={}", kept + dropped_then_kept);
}
```

Transcript:

```text
$ rustc inverse.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./inverse
total=5
take(2)=2
skip(2)=3
take+skip=5
$ echo "run-exit=$?"
run-exit=0
```

Witness for the **new fact today** — the inverse-pair identity:

- `v.iter().take(2).count() == 2` (lesson 136's installed claim).
- `v.iter().skip(2).count() == 3` (today's working probe).
- `2 + 3 == 5 == v.iter().count()`.

This empirically anchors the "inverse semantic" framing: `take(n)` and
`skip(n)` partition the source iterator at position `n`. For any
in-range `n`, the counts sum to the full count. (For `n` larger than
the source length, both adapters truncate to the source bounds —
`take(huge).count() == 5` per lesson 136 Probe 3, `skip(huge).count()
== 0` per Probe 3 below.)

## Probe 3 — skip(huge) on a short iter returns empty

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().skip(100).count();
    println!("count = {}", n);
    let f = v.iter().skip(100).next();
    println!("next  = {:?}", f);
}
```

Transcript:

```text
$ rustc huge.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./huge
count = 0
next  = None
$ echo "run-exit=$?"
run-exit=0
```

Witnesses corpus claim at `trait.Iterator.md:1356-1359`:

> `skip(n)` skips elements until `n` elements are skipped or the end of
> the iterator is reached (whichever happens first). After that, all
> the remaining elements are yielded. In particular, if the original
> iterator is too short, then the returned iterator is empty.

`skip(100)` on a five-element vec produces an empty iterator: `.count()
== 0` and `.next() == None`. No panic.

## Probe 4 — type-pin via E0308 (rustc names the actual type)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: u32 = v.iter().skip(2);
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typeprobe.rs:3:19
  |
3 |     let _x: u32 = v.iter().skip(2);
  |             ---   ^^^^^^^^^^^^^^^^ expected `u32`, found `Skip<Iter<'_, u64>>`
  |             |
  |             expected due to this
  |
  = note: expected type `u32`
           found struct `Skip<std::slice::Iter<'_, u64>>`
```

rustc spells the actual return type out: `Skip<Iter<'_, u64>>` inline,
`Skip<std::slice::Iter<'_, u64>>` in the secondary note. Same forced-
error type-pin technique lessons 134 / 135 / 136 used; today
substitutes `Skip` for `Take`. The `Skip<...>` struct is referenced
opaquely; today's lesson does not unpack its private fields.

## Probe 5 — appendix-only continuity check (E0382 on consumed receiver)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let _t1 = iter.skip(2);
    let _t2 = iter.skip(2);
}
```

Verbatim diagnostic:

```text
error[E0382]: use of moved value: `iter`
 --> consume.rs:5:15
  |
3 |     let iter = v.iter();
  |         ---- move occurs because `iter` has type `std::slice::Iter<'_, u64>`, which does not implement the `Copy` trait
4 |     let _t1 = iter.skip(2);
  |                    ------- `iter` moved due to this method call
5 |     let _t2 = iter.skip(2);
  |               ^^^^ value used here after move
  |
note: `skip` takes ownership of the receiver `self`, which moves `iter`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:1341:12
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
4 |     let _t1 = iter.clone().skip(2);
  |                   ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
exit=1
```

Continuity check, not a centered new claim. Same E-code, same `note:`
shape, same `help:` proposing `.clone()` lessons 133/134/136 captured
for `count`/`last`/`take`. This appendix-only probe confirms `skip`'s
receiver is `self` (no `&`, no `mut`) — if it were `&mut self` or
`&self`, the second call would not move-fail.

**Diagnostic-text variation (worth flagging for audit accuracy):**
`skip`'s `note:` line names the method shortened (`` `skip` takes
ownership ``) rather than fully qualified (lesson 136's `take`
produced `` `std::iter::Iterator::take` takes ownership ``). The
note's source-location pointer reads
`library/core/src/iter/traits/iterator.rs:1341:12` — different from
`take`'s 1414:12 because each provided method has its own definition
site in core. Both forms ("shortened" and "fully qualified") appear
in stable rustc diagnostics depending on rustc internals; the semantic
content (the note exists; it points to the receiver-shape decision in
the trait declaration) is identical.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 64-65 (synopsis-box line)

Verbatim:

```
    fn skip(self, n: usize) -> Skip<Self> ⓘ
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape — what
licenses every iterator inheriting `skip` for free.

### `output/docs/rust/std/iter/trait.Iterator.md` line 1352 (per-method declaration)

Verbatim:

```
#### fn [skip](#method.skip)(self, n: [usize](../primitive.usize.md)) -> [Skip](struct.Skip.md "struct std::iter::Skip")<Self> [ⓘ](#) where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

Authoritative source for:

- **Method name** `skip` and **receiver shape** `(self, n: usize)` —
  bare `self` (lesson 102 consuming) followed by `n: usize` (lesson
  080).
- **Return type** `Skip<Self>` — the wrapper struct documented at
  `output/docs/rust/std/iter/struct.Skip.md`.
- **`where Self: Sized`** — same bound `take`, `count`, and `last`
  carry; named-deferred today.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1354-1359 (prose summary)

Verbatim:

```
Creates an iterator that skips the first `n` elements.

`skip(n)` skips elements until `n` elements are skipped or the end of the
iterator is reached (whichever happens first). After that, all the remaining
elements are yielded. In particular, if the original iterator is too short,
then the returned iterator is empty.
```

This grounds:

- **"Skips the first `n` elements"** — Probe 1 line 1 (count = 3 for
  `skip(2)` on five elements); Probe 1 line 2 (`Some(30)` after
  dropping `&10, &20`); Probe 2 (inverse-sum identity).
- **"If the original iterator is too short, then the returned iterator
  is empty"** — Probe 3 (`skip(100)` on five-element vec → `count =
  0`, `next = None`).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1363-1372 (basic example)

Verbatim:

```
##### Examples

```
let a = [1, 2, 3];

let mut iter = a.into_iter().skip(2);

assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), None);
```
```

The std doc example matches Probe 1's structural pattern modulo source-
collection shape (`[i32; 3].into_iter()` yields `i32`; `Vec<u64>.iter()`
yields `&u64`). Both shapes confirm: `skip(n).next()` on a source of
length `>= n` yields `Some(_)` for the `(n+1)`-th element, then `None`
once the source is exhausted.

### `output/docs/rust/std/iter/struct.Skip.md` lines 1-15

Verbatim:

```
# Struct Skip

1.0.0 ·

```
pub struct Skip<I> { /* private fields */ }
```

Expand description

An iterator that skips over `n` elements of `iter`.

This `struct` is created by the [`skip`](trait.Iterator.md#method.skip "method std::iter::Iterator::skip") method on [`Iterator`](trait.Iterator.md). See its
documentation for more.
```

Today's claim "`Skip<Self>` is itself an iterator" rests on the
`struct.Skip.md` page, which lists `impl<I> Iterator for Skip<I>
where I: Iterator,` (line 60+). Today does not enumerate the impl
list; the opaque "this struct implements Iterator" claim is grounded
by the authoritative struct doc page.

### `output/docs/rust/error_codes/E0382.md`

Probe 5's diagnostic. Lessons 133, 134, and 136 already installed the
diagnostic shape on `count`, `last`, and `take`; today's continuity
check reuses it on `skip`.

### `output/docs/rust/error_codes/E0308.md`

Probe 4's diagnostic. Type-pin technique installed at lessons 134
Probe 5, 135 Probe 6, 136 Probe 6.

## Claim-to-evidence map

- "`v.iter().skip(2).count()` on `vec![10, 20, 30, 40, 50]` prints
  `3`" — Probe 1 line 1.
- "`v.iter().skip(2).next()` on the same vec prints `Some(30)`" —
  Probe 1 line 2.
- "`for x in v.iter().skip(3)` on the same vec prints `40` then `50`"
  — Probe 1 lines 3-4.
- "`take(n)` and `skip(n)` are inverse: `take(n).count() +
  skip(n).count() == count()` for in-range `n`" — Probe 2
  (`2 + 3 == 5`); corpus prose at `:1354-1359` ("skips the first `n`
  elements") + 136's installed claim "yields the first `n` elements".
- "If the original iterator is too short, the returned iterator is
  empty" — `trait.Iterator.md:1356-1359`; Probe 3 (`skip(100)` on
  five-element vec → `count = 0`, `next = None`).
- "`skip` takes the receiver by value (`self`); calling it moves the
  iterator" — `trait.Iterator.md:1352` (signature); Probe 5 (E0382 +
  `note:` block with `skip` in the method-name slot).
- "`n: usize` is the second parameter" — `trait.Iterator.md:1352`.
- "Return type is `Skip<Self>` — itself an iterator" —
  `trait.Iterator.md:1352` (signature names `Skip<Self>`);
  `struct.Skip.md:1-15` (`pub struct Skip<I> { ... }`); Probe 1 (`.next()`
  and `.count()` and `for` all work on the binding); Probe 4 (rustc
  names the type `Skip<Iter<'_, u64>>` in the E0308 expected/found
  labels).
- "Building the `Skip<Self>` value does not call `next` on the inner
  iterator — it is lazy" — *not re-witnessed today*. Inherited from
  lesson 136 by structural analogy: both `take` and `skip` are listed
  in the synopsis box's *Provided methods* block returning a wrapper
  struct, the page treats them as one family, and lesson 136's Trace
  probe established the laziness shape for the `take` adapter. Skip's
  std doc note at `:1361` ("Rather than overriding this method
  directly, instead override the `nth` method") implies its default
  body delegates to `nth`-based traversal — `nth` itself is lazy in
  the sense that it does not eagerly walk on construction (lesson 135
  installed `nth`'s `(&mut self, n: usize) -> Option<Item>` shape).
- "`skip` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:13` ("// Provided methods" comment precedes
  `skip` at `:64-65`); lesson 132 evidence appendix.
- "Adapters compose with consumers via lesson-049 method chaining" —
  Probe 1 (three different consumer-after-adapter chains all compile
  and produce the expected output); inherited from lesson 136.

## Negative / contrast probe coverage

Three contrasts captured:

- **Probe 2 (take + skip inverse-sum)** — *the centered new contrast
  today*. Without this probe, the inverse-semantic claim would rest
  only on corpus prose and structural analogy. The probe gives the
  empirical witness that the two adapters are complementary on the
  same source: `2 + 3 == 5`.
- **Probe 3 (skip(huge) on short iter)** — corroborates corpus claim
  at `:1356-1359`. Without this probe, the "if the original iterator
  is too short, the returned iterator is empty" claim would rest only
  on corpus prose. The probe witnesses both `.count() == 0` and
  `.next() == None` on a `skip(100)` of a five-element vec.
- **Probe 4 (E0308 on `u32`)** — type-pin contrast for the
  `Skip<Self>` return. Without it, the claim "the result is itself an
  iterator" rests on Probe 1's silent compile (which is consistent
  with several inferred annotations). E0308's labels naming
  `Skip<Iter<'_, u64>>` and `Skip<std::slice::Iter<'_, u64>>` pin the
  actual type from rustc's mouth.

Probe 5 (E0382) is appendix-only continuity, not a centered contrast.
The receiver-shape claim is well-installed by lessons 133/134/136;
today's E0382 capture confirms `skip`'s diagnostic uses the same
template (modulo the noted method-name format variation).

**Why no new lazy-witness probe today:** lesson 136's Probe 2 (Trace
+ take, three steps) installed the laziness shape for the adapter
family. Skip is structurally identical (consuming `self`, `usize`
arg, returns wrapper struct from the same synopsis-box block), and
the std doc treats `take` and `skip` as a pair (`:1352, :1376`
adjacent). Re-running a Trace probe with `skip` in place of `take`
would add appendix volume for negligible new fact value. The
laziness claim today is named as inherited, not re-witnessed.

## Iterator API audit alignment

This lesson is step 7 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 7. **`skip`** — small lazy adapter, returns `Skip<Self>`; sibling of
>    `take`.

Audit §4.3 lists `skip` as ready-now, composing "self-by-value 102 +
080". Today executes that move per audit §5 step 7. Lesson 136's
unlock list named today's move:

> future "`Iterator::skip` — `(self, n) -> Skip<Self>`, lazy adapter
> (sibling of `take` with the inverse semantic)" moves (audit §5 step
> 7 — reuses today's `n: usize` slot, self-by-value rule, and lazy-
> adapter frame; the Skip wrapper is the structural sibling of Take).

The new graph fact today: where lesson 136 installed the *adapter*
return shape and laziness in general, today installs the **inverse
semantic** specifically — the first place in the run where two
Iterator methods are framed as a complementary pair on the same
source. This unlocks `enumerate` (audit §5 step 8), `fuse` (step 9),
`step_by` (step 10), and the eventual closure-driven adapters
(`map`, `filter`, `take_while`, `skip_while`) — the last two
specifically reuse today's "the take/skip family also has closure-
driven variants" framing.
