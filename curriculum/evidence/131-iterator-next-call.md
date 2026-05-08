# Evidence — Lesson 131: `iter.next()` on a slice iterator

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/131-iterator-next-call.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/131-iterator-next-call.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/131-iterator-next-call.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet131/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
130.

## Direct prerequisite — lesson 123 (`v.iter()` returns a slice iterator)

Lesson 123 installed:

- `v.iter()` is a method on `Vec<T>` returning a value of type
  `std::slice::Iter<'_, T>` — an iterator (not a `T`).
- The iterator yields each element of the vec in order, as `&T`.
- 123's *What To Ignore For Now* explicitly lists `.next()` as the
  "explicit pull form. Each call returns `Option<&T>`. Composes lesson
  119 with the iterator machinery." — today executes that named-deferred
  composition.

Today binds the iterator with `let mut iter = v.iter();` and pulls
elements with `iter.next()` instead of routing it into a `for`-loop.
The element type and yielding semantics are unchanged from 123;
"yielded as `&T`" carries through, so the wrapper today is `Option<&T>`.

## Direct prerequisite — lesson 119 (`Option<T>` / `Some` / `None`)

Lesson 119 installed:

- `pub enum Option<T> { None, Some(T) }`. `Option` and both variants
  are in the prelude.
- `Some(value)` wraps a value into `Option<T>`; `None` is the
  payload-free variant.
- A `match` on `Option<T>` handles both arms; the
  exhaustiveness/all-arms-share-a-type rules carry through.

Today's `T` is `&u64`. `iter.next()` returns either `Some(&value)`
(payload-bearing) or `None` (exhausted). The `Some(&u64)` slot is just
119's `Some(T)` with `T = &u64` — composition only, no new constructor
shape.

## Direct prerequisite — lesson 006 (`let mut name = value;`)

Lesson 006 installed:

- `let name = value;` is immutable by default.
- Adding `mut` between `let` and the name (`let mut name = value;`)
  makes the binding reassignable / mutable-borrowable.
- Without `mut`, attempting an operation that mutates the binding
  fires the relevant E-code (006's centered code was E0384 for
  reassignment; today's contrast surfaces E0596 for mutable borrow).

Today's `.next()` has receiver `&mut self`; calling it requires a
mutable borrow of the iterator. Lesson 006's rule "without `mut`,
mutating use is rejected" applies, with E0596 in place of E0384.

## Direct prerequisite — lesson 040 (dot-call shape)

`iter.next()` is exactly the dot-call shape: receiver `iter`, dot,
method name `next`, empty argument list `()`. No new call form.

## Older supporting lessons

- **Lesson 093** (cited) — `{:?}` Debug placeholder; example file
  shows `println!("opt = {:?}", opt);` printing `Some(7)` for
  `Option<i32>`. Today applies the same shape to `Option<&u64>` and
  observes `Some(10)` (the `&` is invisible in Debug for primitive
  types).
- **Lessons 011, 001, 002, 003, 005, 080, 019, 107** — same roles as
  in lessons 123, 124, 125, 126: `println!`; rustc compile + run; `fn
  main`; the diagnostic four-part map; `let`; `u64`; the `: TYPE`
  annotation slot; `Vec<T>` / `vec![...]`.

## Probe 1 — working probe (`iter.next()` four times)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/131-iterator-next-call.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
Some(10)
Some(20)
Some(30)
None
$ echo "run-exit=$?"
run-exit=0
```

The centered claim — "successive `iter.next()` calls hand out the
elements in vec order, then `None` once exhausted" — is carried by
the four output lines.

Note the Debug shape: `Option<&u64>` Debug-prints as `Some(10)`, not
`Some(&10)`. This matches the std `Iterator::next` example at
`output/docs/rust/std/iter/trait.Iterator.md:289-303` which prints
`Some(1)` for an `Option<i32>` from `[i32; 3].into_iter()`. Probes 4
and 5 below pin the actual return type as `Option<&u64>` despite the
`&`-less Debug output.

## Probe 2 — centered contrast (drop `mut`)

`broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    println!("{:?}", iter.next());
}
```

Transcript verbatim:

```text
$ rustc broken.rs
error[E0596]: cannot borrow `iter` as mutable, as it is not declared as mutable
 --> broken.rs:4:22
  |
4 |     println!("{:?}", iter.next());
  |                      ^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut iter = v.iter();
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
exit=1
```

The diagnostic states the structural fact directly: `iter.next()`
needs to *borrow `iter` as mutable* — that is the `&mut self` receiver
at work. The `help:` line literally writes today's fix.

This is the lesson's centered contrast: it has a verbatim diagnostic
that names the boundary ("cannot borrow `iter` as mutable"), and it
empirically witnesses the otherwise-invisible `&mut self` receiver in
the std `Iterator::next` declaration.

## Probe 3 — corroborating (different vec, same mechanic)

`corrob.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![100, 200];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
```

Transcript:

```text
$ rustc corrob.rs
$ ./corrob
Some(100)
Some(200)
None
```

A vec of length 2 yields two `Some(_)` calls then `None` on the third
call. Confirms the rule "`None` after every element has been yielded"
is general — not coupled to Probe 1's specific length 3.

## Probe 4 — type witness (pin the return type)

`typetest.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    let first: Option<&u64> = iter.next();
    println!("{:?}", first);
}
```

Transcript:

```text
$ rustc typetest.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./typetest
Some(10)
$ echo "run-exit=$?"
run-exit=0
```

The `let first: Option<&u64> = iter.next();` annotation pins the
return type. rustc accepts it without diagnostic, so `iter.next()`
empirically has type `Option<&u64>` on this iterator.

## Probe 5 — type-witness contrast (wrong inner type)

`typetest2.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    let first: Option<u64> = iter.next();
    println!("{:?}", first);
}
```

Transcript verbatim:

```text
$ rustc typetest2.rs
error[E0308]: mismatched types
 --> typetest2.rs:4:30
  |
4 |     let first: Option<u64> = iter.next();
  |                -----------   ^^^^^^^^^^^ expected `Option<u64>`, found `Option<&u64>`
  |                |
  |                expected due to this
  |
  = note: expected enum `Option<_>`
             found enum `Option<&_>`
help: use `Option::copied` to copy the value inside the `Option`
  |
4 |     let first: Option<u64> = iter.next().copied();
  |                                         +++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Negative type-pin: trying to bind the result to `Option<u64>` (with no
reference) fires E0308 with inline label `expected Option<u64>, found
Option<&u64>`. This is the cleanest empirical witness for the claim
"the actual return type is `Option<&u64>`, with the `&`." Together
with Probe 4 (positive type-pin compiles), Probes 4-5 sandwich the
return type.

## Probe 6 — Check Yourself ground

`checkyourself.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![5, 6];
    let mut it = xs.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

Transcript:

```text
$ rustc checkyourself.rs
$ ./checkyourself
Some(5)
Some(6)
None
```

Confirms Check Yourself (a) + (c): three `println!` lines, output
`Some(5)`, `Some(6)`, `None` (the third call returns `None` because the
two-element vec is exhausted).

## Probe 7 — independent iterators (cursor lives on the iterator)

`rebound.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    let mut iter2 = v.iter();
    println!("{:?} {:?}", iter.next(), iter2.next());
    println!("{:?} {:?}", iter.next(), iter2.next());
}
```

Transcript:

```text
$ rustc rebound.rs
$ ./rebound
Some(10) Some(10)
Some(20) Some(20)
```

Two iterators built from the same vec each track their own position:
both return `Some(10)` then `Some(20)`. Confirms the cursor is local
state on the iterator object, not on the vec — the `mut` is needed
because the iterator is the thing being mutated, not the vec. Used
internally in this appendix to ground the *Mental Model Delta*'s "the
iterator carries a cursor" framing; not surfaced in the lesson body.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 273-304

Lines 273-284 verbatim:

> ## Required Methods[§](#required-methods)
>
> 1.0.0 ·
>
> #### fn next(&mut self) -> Option<Self::Item>
>
> Advances the iterator and returns the next value.
>
> Returns `None` when iteration is finished. Individual iterator
> implementations may choose to resume iteration, and so calling
> `next()` again may or may not eventually start returning `Some(Item)`
> again at some point.

Lines 286-304 (the `## Examples` section) verbatim:

> ##### Examples
>
> ```
> let a = [1, 2, 3];
>
> let mut iter = a.into_iter();
>
> // A call to next() returns the next value...
> assert_eq!(Some(1), iter.next());
> assert_eq!(Some(2), iter.next());
> assert_eq!(Some(3), iter.next());
>
> // ... and then None once it's over.
> assert_eq!(None, iter.next());
>
> // More calls may or may not return `None`. Here, they always will.
> assert_eq!(None, iter.next());
> assert_eq!(None, iter.next());
> ```

This is the authoritative source of every load-bearing claim in the
lesson:

- *signature*: `fn next(&mut self) -> Option<Self::Item>`. The `&mut
  self` receiver is what Probe 2's E0596 surfaces empirically.
- *semantics*: "Advances the iterator and returns the next value.
  Returns `None` when iteration is finished." Probe 1's four lines
  witness this on a slice iterator.
- *let mut iter = ...; iter.next(); iter.next(); ...; None* — the
  std example uses exactly this driver shape (with `into_iter()` over
  an array; today uses `iter()` over a `Vec`). Same `let mut iter =`,
  same `iter.next()` repeated, same `Some(_)` then `None` arc.
- *resume-after-None caveat*: "Individual iterator implementations may
  choose to resume iteration." This is the *abstract* statement about
  the `Iterator` trait in general — it does **not** license a universal
  claim on slice iterators. The universal claim "for slice iterators,
  every call after the first `None` is also `None`" is licensed by the
  `FusedIterator` impl at `struct.Iter.md:731-733` plus the trait body
  at `trait.FusedIterator.md:6-15`; see those two blocks below. The
  sticky-`None` rule and the `Fuse<I>` adapter are named-deferred.

The std example uses `assert_eq!` macros instead of `println!("{:?}",
…)`. Today uses `println!("{:?}", iter.next())` — `assert_eq!` is not
yet installed in the lesson stream (lesson 071 named-deferred it, no
later lesson centered it), so lesson 093's `{:?}` is the available
shape for showing the values.

### `output/docs/rust/std/slice/struct.Iter.md` lines 195-207

Verbatim:

> ### impl<'a, T> Iterator for Iter<'a, T>
>
> #### type Item = &'a T
>
> The type of the elements being iterated over.
>
> #### fn next(&mut self) -> Option<&'a T>
>
> Advances the iterator and returns the next value. [Read more](../iter/trait.Iterator.md#tymethod.next)

This is the trait impl for the *specific* iterator type `v.iter()`
returns. Three load-bearing facts:

- `impl<'a, T> Iterator for Iter<'a, T>` — `Iter<'_, T>` (the type
  lesson 123 surfaced from rustc's diagnostic) implements `Iterator`,
  so `iter.next()` is callable on it.
- `type Item = &'a T` — the slice iterator's `Item` associated type is
  `&'a T`. This is what makes `next` on this specific iterator return
  `Option<&T>` rather than `Option<T>` or anything else.
- `fn next(&mut self) -> Option<&'a T>` — the *specialized* signature
  for `Iter<'a, T>`, with `Self::Item` resolved to `&'a T`. This is
  what Probe 4's `let first: Option<&u64> = iter.next();` empirically
  pins.

Today's lesson body does not surface the `'a` lifetime — wholesale
deferred since 123. The lesson body says only "for the slice iterator,
`Item = &u64`" (the `'a` is implicit).

Lines 731-733 also matter today. Verbatim:

> 1.26.0 · [§](#impl-FusedIterator-for-Iter%3C'_,+T%3E)
>
> ### impl<T> [FusedIterator](../iter/trait.FusedIterator.md "trait std::iter::FusedIterator") for [Iter](struct.Iter.md "struct std::slice::Iter")<'_, T>

Slice iterators implement `FusedIterator`. This is the type-level
license for the universal claim "once `iter.next()` returns `None`,
every subsequent call also returns `None`" on slice iterators
specifically — see the `FusedIterator` block immediately below.

### `output/docs/rust/std/iter/trait.FusedIterator.md` lines 6-15

Verbatim:

> ```
> pub trait FusedIterator: Iterator { }
> ```
>
> Expand description
>
> An iterator that always continues to yield `None` when exhausted.
>
> Calling next on a fused iterator that has returned `None` once is guaranteed
> to return [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None") again.

Combined with `struct.Iter.md:731-733`'s `impl<T> FusedIterator for
Iter<'_, T>`, this is the licensor for today's universal claim on the
slice iterator: every call after the first `None` is *guaranteed* to
return `None`. The abstract `Iterator::next` caveat at
`trait.Iterator.md:281-284` ("may or may not eventually start
returning `Some(Item)` again") is the general statement and *cannot*
license a universal claim on its own; the universal claim is licensed
by the `FusedIterator` impl + trait body.

The lesson body's named-deferred phrase "exhausted means `None`
forever" is the audience-level rendering of this `FusedIterator`
guarantee. The trait name `FusedIterator` and the `Fuse<I>` adapter
are wholesale deferred (named in *What To Ignore For Now*).

### `output/docs/rust/error_codes/E0596.md`

Probe 2 produces an `E0596` block. The error code documents the
"cannot borrow X as mutable, as it is not declared as mutable" rule.
Lesson 003's diagnostic four-part map covers the block; today reuses
it without re-teaching the diagnostic format.

## Claim-to-evidence map

- "A slice iterator has a method `.next()` that you call with the
  dot-call shape `iter.next()`" — `std/iter/trait.Iterator.md:277`
  declaration `fn next(&mut self) -> Option<Self::Item>`;
  `std/slice/struct.Iter.md:195-207` `impl Iterator for Iter`; lesson
  040 dot-call shape; Probe 1 silent compile.
- "Each call hands back the next element wrapped in `Some(...)`, in vec
  order" — `std/iter/trait.Iterator.md:279` "Advances the iterator and
  returns the next value"; `std/iter/trait.Iterator.md:288-296` example
  showing `Some(1)`, `Some(2)`, `Some(3)`; Probe 1 transcript
  `Some(10)`, `Some(20)`, `Some(30)`.
- "Once exhausted, calls return `None`" —
  `std/iter/trait.Iterator.md:281` "Returns `None` when iteration is
  finished"; Probe 1's fourth call transcript line `None`.
- "For slice iterators, every subsequent call after `None` is also
  `None`" — `std/slice/struct.Iter.md:731-733` `impl<T>
  FusedIterator for Iter<'_, T>` (slice iterators implement
  `FusedIterator`); `std/iter/trait.FusedIterator.md:6-15` declares
  `pub trait FusedIterator: Iterator { }` with body sentence "An
  iterator that always continues to yield `None` when exhausted.
  Calling next on a fused iterator that has returned `None` once is
  guaranteed to return `None` again." The general resume-after-`None`
  caveat at `trait.Iterator.md:281-284` is the *abstract-Iterator*
  may-or-may-not-resume statement — the universal claim for slice
  iterators is licensed by the `FusedIterator` impl + trait body, not
  by that abstract caveat.
- "Return type is `Option<&T>`" — `std/slice/struct.Iter.md:199`
  `type Item = &'a T`; `std/slice/struct.Iter.md:205` `fn next(&mut
  self) -> Option<&'a T>`; Probe 4 silent compile (positive type-pin);
  Probe 5 E0308 with inline label `expected Option<u64>, found
  Option<&u64>` (negative type-pin).
- "`{:?}` prints `Some(10)` not `Some(&10)` for `Option<&u64>`" —
  Probe 1 transcript empirical; lesson 093 installed `{:?}`. Formatter
  rule (`impl<T: Debug + ?Sized> Debug for &T`) named-deferred in
  *What To Ignore*.
- "`.next()` takes `&mut self`; binding must be `let mut iter`" —
  `std/iter/trait.Iterator.md:277` declaration; Probe 2 E0596 verbatim
  diagnostic with `help:` proposing `let mut iter`; lesson 006 rule.
- "The iterator carries a cursor; the loop walks each element exactly
  once" — Probe 7 transcript (two iterators, independent positions);
  internal grounding for *Mental Model Delta*.

## Negative / contrast probe coverage

Three contrasts captured. All three needed:

- **Probe 2 (E0596 on missing `mut`)** is the centered contrast. It
  has a verbatim diagnostic naming the boundary ("cannot borrow
  `iter` as mutable") and a `help:` line that writes the fix. It is
  the empirical witness for the otherwise-invisible `&mut self`
  receiver in `Iterator::next`'s declaration.
- **Probe 5 (E0308 on `Option<u64>`)** is the type-witness contrast.
  It pins the actual return type as `Option<&u64>` despite the
  `&`-less Debug output. The lesson body cites this without showing
  the full transcript; the appendix has the verbatim diagnostic.
- **Probe 7 (two iterators, independent state)** is a corroborating
  contrast. It witnesses that the cursor lives on the iterator
  object — vacating the worry "did the first `.next()` modify the
  vec?" Used in the appendix to ground *Mental Model Delta*'s
  "iterator carries a cursor" framing; not surfaced in the lesson
  body.

Probe 3 (different vec) and Probe 6 (Check Yourself ground) are
corroborative, not contrastive — they witness the mechanic on
different vec lengths.
