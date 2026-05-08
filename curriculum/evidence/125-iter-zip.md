# Evidence — Lesson 125: `.zip()` on the iterator from `Vec<T>::iter()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/125-iter-zip.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/125-iter-zip.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/125-iter-zip.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet125/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons 123
and 124.

## Direct prerequisite — lesson 123 (`v.iter()` returns an iterator)

Lesson 123 installed:

- `v.iter()` is a method on `Vec<T>` returning a value of type
  `std::slice::Iter<'_, T>` — an iterator yielding each element of
  the vec in order, as `&T`.
- The iterator is consumable by `for x in v.iter() { ... }` with
  `x: &T`.

Today's lesson calls `.iter()` *twice* — once on each of two vecs —
and chains `.zip(...)` on the first call's result, passing the second
call's result as the argument. The receiver and argument types are
both `Iter<'_, u64>` for `Vec<u64>` inputs.

## Direct prerequisite — lesson 072 (tuple type and `.0` / `.1` indexing)

Lesson 072 installed:

- A 2-tuple value `(v1, v2)` and the type `(T1, T2)`.
- Field access by `expr.0` (first field) and `expr.1` (second field)
  — plain decimal numbers, no leading zero, no type suffix.
- Tuples have fixed length known to rustc at compile time.

Today's `for pair in v.iter().zip(w.iter())` binds `pair` to a
2-tuple yielded by the zipped iterator. `pair.0` and `pair.1` are
exactly lesson 072's indexing on that bound tuple. Probe 5 in this
appendix witnesses the field types empirically: `let _: &u64 =
pair.0;` and `let _: &u64 = pair.1;` are accepted by rustc.

## Direct prerequisite — lesson 049 (chained dot-calls)

Lesson 049 installed `expr.method1().method2()`: a left-to-right
chain where each call's return value becomes the receiver of the next
call. `v.iter().zip(w.iter())` fills the shape exactly:

- `v.iter()` evaluates first. Receiver `v: Vec<u64>`, method `iter`,
  empty arg list. Returns `Iter<'_, u64>` (lesson 123).
- `.zip(w.iter())` then evaluates on that returned iterator. Same
  dot-call shape, but with one argument inside the parentheses —
  `w.iter()` is itself a call expression returning the second
  `Iter<'_, u64>`.

The signature `fn zip<U>(self, other: U) -> Zip<Self, ...> where
Self: Sized, U: IntoIterator` (per `std/iter/trait.Iterator.md:664`)
shows that `.zip()` consumes its receiver iterator and returns a new
iterator of type `Zip<Self, U::IntoIter>`. The lesson body does not
surface that return type name.

## Older supporting lessons

- **Lesson 124** — installed `.rev()` chained on the iterator from
  `.iter()`, with the same E0599 contrast on `Vec<T>` as receiver
  ("`Vec<u64>` is not an iterator"). Today's contrast (Probe 2)
  reuses that exact diagnostic shape with `zip` swapped in for `rev`.
- **Lessons 040, 011, 001, 002, 003, 005, 080, 019** — same roles as
  in lessons 123 and 124: dot-call grammar; `println!`; rustc compile
  and run; `fn main`; the diagnostic four-part map; `let`; `u64`;
  the `: TYPE` annotation slot.
- **Lesson 079** — `for X in COLLECTION { ... }` over a runtime
  collection. Today's COLLECTION is the chained iterator returned by
  `.zip(...)`.
- **Lesson 100** — installed E0599 "no method named X found for type
  Y" diagnostic shape. Today's contrast probe (Probe 2) reuses that
  diagnostic shape on a `Vec<u64>` receiver with `zip` as the method.
- **Lesson 107** — `Vec<T>` construction with `vec![]`.

## Probe 1 — working probe (`v.iter().zip(w.iter())`)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/125-iter-zip.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for pair in v.iter().zip(w.iter()) {
        println!("{} / {}", pair.0, pair.1);
    }
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
10 / 100
20 / 200
30 / 300
$ echo "run-exit=$?"
run-exit=0
```

The centered claims — "`.zip(...)` chained on the iterator from
`.iter()` produces an iterator yielding tuples paired position-wise
from the two sources" — is carried by the output. The `vec![10, 20,
30]` and `vec![100, 200, 300]` literals define source order; the
printed sequence (`10 / 100`, `20 / 200`, `30 / 300`) is exactly
positional pairing.

## Probe 2 — diagnostic contrast (`v.zip(w)` directly on `Vec<u64>`)

`broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    let z = v.zip(w);
    println!("{:?}", z);
}
```

Transcript verbatim:

```text
$ rustc broken.rs
error[E0599]: no method named `zip` found for struct `Vec<u64>` in the current scope
 --> broken.rs:4:15
  |
4 |     let z = v.zip(w);
  |               ^^^ `Vec<u64>` is not an iterator
  |
help: call `.into_iter()` first
  |
4 |     let z = v.into_iter().zip(w);
  |               ++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
exit=1
```

Lesson 100's E0599 shape applied to `Vec<u64>` as receiver. The
inline label "`Vec<u64>` is not an iterator" states today's
structural fact directly: `.zip()` is a method on iterators, not on
`Vec<T>`. The diagnostic is structurally identical to lesson 124's
`v.rev()` contrast — same E-code, same inline label, same `help:`
line — only the method name changes.

This is the lesson's *centered* contrast because it has a verbatim
diagnostic that names the boundary. Probe 3 (shortest-source) is a
*semantic* contrast captured separately.

## Probe 3 — shortest-source rule (mismatched lengths)

`short.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100];
    for pair in v.iter().zip(w.iter()) {
        println!("{} / {}", pair.0, pair.1);
    }
}
```

Transcript:

```text
$ rustc short.rs
$ ./short
10 / 100
```

`v` has three elements, `w` has one; the zipped iterator yields
exactly one tuple and stops. No panic, no error — the loop body
runs once and the for-loop exits cleanly. The remaining elements of
`v` (`20`, `30`) are not visited. Empirical witness for the
shortest-source rule named in the lesson body.

## Probe 4 — empty-source corroborator

`empty.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![];
    let mut count = 0u64;
    for pair in v.iter().zip(w.iter()) {
        println!("{} / {}", pair.0, pair.1);
        count += 1;
    }
    println!("count = {}", count);
}
```

Transcript:

```text
$ rustc empty.rs
$ ./empty
count = 0
```

The for-loop body never runs — when one source is empty, the
zipped iterator yields zero tuples. Confirms the shortest-source
rule at the zero boundary; the loop body's `println!` and `count
+= 1` never execute, so only the trailing `count = 0` line is
printed.

## Probe 5 — type-witness (`pair.0` and `pair.1` are `&u64`)

`typecheck.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for pair in v.iter().zip(w.iter()) {
        let _: &u64 = pair.0;
        let _: &u64 = pair.1;
        println!("{} / {}", pair.0, pair.1);
    }
}
```

Transcript:

```text
$ rustc typecheck.rs
$ ./typecheck
10 / 100
20 / 200
30 / 300
```

The annotations `let _: &u64 = pair.0;` and `let _: &u64 = pair.1;`
are accepted by rustc. Empirical witness that `pair: (&u64, &u64)`
and the two indexing expressions return `&u64`. Lesson 123's
"yields `&T`" rule is preserved per-element under zip.

## Probe 6 — corroborating (different lengths, arithmetic on `&u64`)

`corrob.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![1, 2];
    let ys: Vec<u64> = vec![100, 200];
    for pair in xs.iter().zip(ys.iter()) {
        println!("{} + {} = {}", pair.0, pair.1, pair.0 + pair.1);
    }
}
```

Transcript:

```text
$ rustc corrob.rs
$ ./corrob
1 + 100 = 101
2 + 200 = 202
```

A different vec length (2 instead of 3) produces two paired tuples
in vec order, witnessing that the mechanic generalizes across vec
lengths. The `pair.0 + pair.1` expression — adding two `&u64` values
— compiles and runs; std's auto-deref / `Add for &u64` machinery
handles this and is named-deferred in the lesson body. The
load-bearing fact for today is just that the mechanic is general,
not the arithmetic on references.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 664-680

Verbatim signature (line 664):

> #### fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> where Self: Sized, U: IntoIterator,

Verbatim prose (lines 666-678):

> 'Zips up' two iterators into a single iterator of pairs.
>
> `zip()` returns a new iterator that will iterate over two other
> iterators, returning a tuple where the first element comes from
> the first iterator, and the second element comes from the second
> iterator.
>
> In other words, it zips two iterators together, into a single one.
>
> If either iterator returns `None`, `next` from the zipped iterator
> will return `None`.

This is the authoritative description of the method today centers.
Three load-bearing facts, mapped to the lesson body:

- *signature* `fn zip<U>(self, other: U) -> Zip<Self, ...>`:
  `.zip()` consumes the receiver iterator and takes one argument.
  Today does not surface the return type `Zip<Self, ...>` in the
  lesson body; it is named-deferred.
- *pairing rule* "returning a tuple where the first element comes
  from the first iterator, and the second element comes from the
  second iterator": Probe 1's three printed pairs (`10 / 100`,
  `20 / 200`, `30 / 300`) are the empirical witness on `Vec<u64>`
  inputs.
- *shortest-source rule* "If either iterator returns `None`, `next`
  from the zipped iterator will return `None`": Probes 3 and 4
  witness this — at length 1 only one pair is yielded; at length 0
  none are.

The `where U: IntoIterator` bound — why the argument can be an
iterator-like value and not just an `Iterator` — is named-deferred
in the lesson body's "What To Ignore For Now."

### `output/docs/rust/std/iter/struct.Zip.md` lines 1-15

Verbatim:

> # Struct Zip
>
> ```
> pub struct Zip<A, B> { /* private fields */ }
> ```
>
> An iterator that iterates two other iterators simultaneously.
>
> This `struct` is created by `zip` or `Iterator::zip`. See their
> documentation for more.

The page that exists for the return type `Zip<A, B>`. The lesson
body does not surface the type name; this page grounds the "name
exists" fact only.

### `output/docs/rust/error_codes/E0599.md` line 4

Verbatim:

> This error occurs when a method is used on a type which doesn't
> implement it

Probe 2's diagnostic is exactly an instance: `Vec<u64>` is the type;
`zip` is the method; `Vec<u64>` does not implement an inherent or
trait `.zip()` method because `Vec<u64>` is not an iterator (the
inline label states this directly). Today reuses lesson 100's E0599
reading discipline; this appendix points at the error-code doc for
completeness.

## rmp unlock — `cmp.rs:22` `self.limbs.iter().rev().zip(...)`

Source `output/repos/rmp/src/biguint/cmp.rs` line 22 verbatim:

```rust
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
```

Lesson 123 made `self.limbs.iter()` readable. Lesson 124 made
`.rev()` on that iterator readable. Today makes `.zip(...)`
readable as the method that consumes one iterator and takes a
second one as argument. The chain so far parses as: `self.limbs`
(field access on a `Vec<u64>` per lesson 095), `.iter()` (123),
`.rev()` (124), `.zip(other.limbs.iter().rev())` (today, with the
argument's structure mirroring the receiver's). The remainder —
the `for (left, right) in ...` destructuring binding the yielded
tuple's parts in the for-loop slot — is a separate future move.

## Claim-to-evidence map

- "`.zip()` is callable on the iterator returned by
  `Vec<T>::iter()`" — `std/iter/trait.Iterator.md:664` signature
  `fn zip<U>(self, other: U) -> Zip<Self, ...> where Self: Sized,
  U: IntoIterator`; Probe 1 silent compile is the empirical
  witness on `Iter<'_, u64>` as the receiver type.
- "`.zip(...)` takes another iterator as argument" — same signature
  line; Probe 1 has `w.iter()` in the argument slot, also of type
  `Iter<'_, u64>`. The `where U: IntoIterator` bound permits any
  iterator-like, but today's probe uses an `Iterator` directly,
  which satisfies the bound trivially.
- "Each yielded value is a 2-tuple, paired position-wise" —
  `trait.Iterator.md:668-670` "returning a tuple where the first
  element comes from the first iterator, and the second element
  comes from the second iterator"; Probe 1's three printed lines
  match position-by-position; Probe 5's type-witness confirms the
  tuple is `(&u64, &u64)`.
- "`pair.0` and `pair.1` are the lesson-072 indexing on the
  yielded tuple" — lesson 072's rule applied to a tuple value
  in the `for`-loop binding slot; Probe 1 silent compile and
  expected output; Probe 5 explicit type-witness `let _: &u64 =
  pair.0;` accepted.
- "When sources differ in length, iteration stops at the shorter
  one" — `trait.Iterator.md:674-675` "If either iterator returns
  `None`, `next` from the zipped iterator will return `None`";
  Probe 3 transcript (length 3 zipped against length 1 yields one
  pair); Probe 4 transcript (length 3 zipped against length 0
  yields zero pairs).
- "`.zip()` is not a method on `Vec<T>` itself" — Probe 2
  transcript verbatim ("`Vec<u64>` is not an iterator");
  `error_codes/E0599.md:4`.
- "`for pair in v.iter().zip(w.iter()) { ... }` runs the body
  once per yielded tuple" — lesson 079's per-element rule applied
  to the new COLLECTION; Probe 1 transcript (three lines, three
  yielded tuples, each binding `pair` once).
- "rmp `cmp.rs:22` `.zip(...)` link readable" —
  `output/repos/rmp/src/biguint/cmp.rs:22` verbatim; the chain's
  third link matches Probe 1's `.zip(...)` shape with iterator
  receiver and iterator argument.

## Negative / contrast probe coverage

Two contrasts captured. Both are needed:

- **Probe 2 (E0599 on `v.zip(w)`)** is the structural-boundary
  contrast. `.zip()` is not a method on `Vec<T>`; you must produce
  an iterator first. The diagnostic states this verbatim
  ("`Vec<u64>` is not an iterator"). The lesson body centers this
  contrast because it has a clean verbatim diagnostic that mirrors
  lesson 124's structure; only the method name swaps.
- **Probes 3 + 4 (shortest-source)** are the *semantic* contrast
  for the shortest-source claim named in the lesson body. The
  std prose says "If either iterator returns `None`, `next` from
  the zipped iterator will return `None`"; Probe 3 witnesses this
  at length 1, Probe 4 at length 0. No diagnostic — the contrast
  is in the runtime behavior. The lesson body references Probe 3
  in The Move ("`vec![10, 20, 30]` zipped against `vec![100]`
  yields one pair and the loop ends").

Probes 5 and 6 are corroborative, not contrastive. Probe 5
witnesses the field types `(&u64, &u64)` directly; Probe 6
witnesses that the mechanic generalizes across vec lengths.
