# Evidence — Lesson 123: `v.iter()` on `Vec<T>` consumed by a `for`-loop

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/123-vec-iter.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/123-vec-iter.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/123-vec-iter.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet123/`)
and compiled with `rustc <file>`; the resulting executables were run
from the same directory.

## Direct prerequisite — lesson 107

Lesson 107 installed:

- `Vec<T>` construction via the `vec![]` macro: `let v: Vec<u64> =
  vec![10, 20, 30];` is exactly the shape today reuses to bind the
  receiver.
- The explicit `Vec<u64>` annotation in the type slot.
- 107's "What To Ignore For Now" lists `for x in &v`, `v.iter()`, and
  `v.iter_mut()` as deferred siblings; today installs the `.iter()`
  branch only.

Today does not re-derive vec construction; the working probe declares
`v: Vec<u64>` exactly as in 107's working probe.

## Direct prerequisite — lesson 079

Lesson 079 installed:

- `for X in COLLECTION { ... }` over a runtime collection — Reference
  `expressions/loop-expr.md:208` "A `for` expression is a syntactic
  construct for looping over elements provided by an implementation
  of `std::iter::IntoIterator`."
- The loop runs the body once per yielded element with `X` bound to
  that element. 079's COLLECTION slot held an array; today's holds an
  iterator (the return value of `.iter()`).

Today fills the COLLECTION slot with `v.iter()`. The loop machinery is
unchanged. The structural reason this composes is the `IntoIterator`
impl on `Iter<'_, T>` itself (every `Iterator` is its own
`IntoIterator`) — named-deferred today; the empirical witness is
Probe 1's silent compile.

## Direct prerequisite — lesson 040

Lesson 040 installed `value.method(args)` as the dot-call shape: a
receiver expression, a dot, a method name, and a parenthesized
argument list. `v.iter()` fills the shape with `v` as the receiver,
`iter` as the method name, and an empty argument list. The whole
expression has whatever type the method returns — here, `Iter<'_, u64>`
per `primitive.slice.md:1434`. The call expression slots into the
COLLECTION slot of the for-loop the same way `n.abs()` slotted into
the right of `let` in lesson 040.

## Direct prerequisite — lesson 011

Lesson 011 installed `println!("... {}", expr)` with positional `{}`
slots. Today's working probe has one slot per call.

Today's `expr` (the loop binding `x`) has type `&u64` (Probe 4 verbatim
witnesses this with `let _: &u64 = x;`). `println!`'s `{}` formatting
applies the `Display` trait, and std provides a *blanket* `Display`
impl `impl<T: ?Sized + Display> Display for &T` (per std's
`fmt::Display` page; not exhibited centrally today). The practical
effect: `println!("{}", x)` for `x: &u64` produces the same output as
`println!("{}", *x)` would for `*x: u64`. Today does not re-derive
this; the appendix names the rule and Probe 1's output is the
empirical witness ("10/20/30", not addresses or pointers).

## Older supporting lessons

- **Lessons 001, 002** — `rustc demo.rs && ./demo`, silent on success;
  `fn main` entry. Used by every probe.
- **Lesson 003** — diagnostic four-part map (headline, location,
  source excerpt with caret, optional notes/help). Applied to Probe 2's
  E0308 transcript in the lesson body.
- **Lesson 005** — `let v = ...;` and the equivalents in every probe.
- **Lessons 080, 019** — `u64` and `: Vec<u64>`.
- **Lesson 022** — `for X in 0..N { ... }` was the original for-loop
  shape; lesson 079 extended COLLECTION to arrays; today extends it
  again to iterators returned from `.iter()`.
- **Lesson 095** — `self.field` field access, used in the "What
  Changed" rmp-unlock framing for `self.limbs.iter()`.

## Probe 1 — working probe (`for x in v.iter()`)

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/123-vec-iter.rs`
is the probe.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter() {
        println!("{}", x);
    }
}
```

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
10
20
30
$ echo "run-exit=$?"
run-exit=0
```

The centered claim — "`v.iter()` returns an iterator that, when
consumed by a `for`-loop, yields each element of the vec in order" —
is carried by lines 2-4 of the source: the `vec![10, 20, 30]` literal
defines element order; the loop output `10/20/30` is the empirical
witness that the iteration produces those elements in that order.

## Probe 2 — centered contrast (E0308 type-mismatch)

`broken.rs` treats `v.iter()` as if it were a `u64`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let bad: u64 = v.iter();
}
```

Compile result, captured verbatim:

```text
error[E0308]: mismatched types
 --> broken.rs:3:20
  |
3 |     let bad: u64 = v.iter();
  |              ---   ^^^^^^^^ expected `u64`, found `Iter<'_, u64>`
  |              |
  |              expected due to this
  |
  = note: expected type `u64`
           found struct `std::slice::Iter<'_, u64>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

The diagnostic states today's structural fact verbatim. The headline
E0308 is the type-mismatch error code (`error_codes/E0308.md:4`
"Expected type did not match the received type"). The inline label
`expected u64, found Iter<'_, u64>` and the `note:` block `found
struct \`std::slice::Iter<'_, u64>\`` together name *both* types: the
type the surrounding context expects (`u64`, from the `: u64`
annotation) and the type the call expression actually has
(`std::slice::Iter<'_, u64>`).

This is the centered contrast for the lesson body's claim "the return
value is an iterator, not a `T` value." The loop machinery of lesson
079 is what unpacks the iterator into elements; without that
machinery, the iterator is a value of a *different* type than the
elements.

## Probe 3 — corroborating (different vec, same mechanic)

```rust
fn main() {
    let names: Vec<u64> = vec![100, 200];
    for x in names.iter() {
        println!("{}", x);
    }
}
```

```text
$ rustc corrob.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./corrob
100
200
$ echo "run-exit=$?"
run-exit=0
```

Same mechanic on a different vec (different element values, different
length, different binding name). The yielded elements are `100`, `200`
in vec order. Corroborates that the mechanic is general — not coupled
to the specific values `[10, 20, 30]` in Probe 1.

## Probe 4 — type witness (`let _: &u64 = x;`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter() {
        let _: &u64 = x;
        println!("{}", x);
    }
}
```

```text
$ rustc typetest.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./typetest
10
20
30
$ echo "run-exit=$?"
run-exit=0
```

The line `let _: &u64 = x;` is a type-assertion pattern: rustc accepts
this if and only if the right-hand side has type `&u64` (or coerces
to it). The compile succeeds silently, so the loop binding `x` has
type `&u64` — the yielded item type for `Iter<'_, u64>` per
`std/slice/struct.Iter.md:199` `type Item = &'a T`. The lesson body
states this fact in `Mental Model Delta` and `What Changed`; Probe 4
is the empirical witness.

## Probe 5 — alternative shape (bare `for x in v`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v {
        println!("{}", x);
    }
}
```

```text
$ rustc alt.rs
$ ./alt
10
20
30
```

`diff <(./demo) <(./alt)` exits 0 — byte-identical output. The bare
`for x in v` shape *also* walks the elements; today does not center
it but notes its existence in `## What To Ignore For Now`. They are
*different* shapes — Probe 6 surfaces the structural difference.

## Probe 6 — alternative shape consumes v (E0382)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v {
        println!("{}", x);
    }
    println!("len after = {}", v.len());
}
```

Compile result, captured verbatim:

```text
error[E0382]: borrow of moved value: `v`
 --> use_after.rs:6:32
  |
2 |     let v: Vec<u64> = vec![10, 20, 30];
  |         - move occurs because `v` has type `Vec<u64>`, which does not implement the `Copy` trait
3 |     for x in v {
  |              - `v` moved due to this implicit call to `.into_iter()`
...
6 |     println!("len after = {}", v.len());
  |                                ^ value borrowed here after move
  |
note: `into_iter` takes ownership of the receiver `self`, which moves `v`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/collect.rs:312:17
help: consider iterating over a slice of the `Vec<u64>`'s content to avoid moving into the `for` loop
  |
3 |     for x in &v {
  |              +
```

This is *not* a centered probe for today. It is captured to ground the
appendix's "different shape" framing: the diagnostic says explicitly
that `for x in v` is "an implicit call to `.into_iter()`" that moves
`v`. The dot-form `v.iter()` shape today centers does *not* move `v`
— it borrows. Today does not exercise this distinction in the lesson
body; ownership-and-iterator interactions are wholesale deferred.

## Why this works — Reference, std, and Book grounding

### std `output/docs/rust/std/primitive.slice.md` line 1434

Verbatim:

> `pub fn iter(&self) -> Iter<'_, T>`
> Returns an iterator over the slice.
> The iterator yields all items from start to end.

This is the exact signature today centers. Two facts: the return type
`Iter<'_, T>` (named in Probe 2's diagnostic verbatim) and the
order guarantee "from start to end" (witnessed by Probe 1's output
order matching the `vec![10, 20, 30]` literal order).

The method is declared on the slice primitive `[T]`; `Vec<T>`
inherits it via the `Deref<Target = [T]>` impl on `struct.Vec`
(see "Inherits from `[T]`" below). The lesson body does not exercise
the Deref mechanic; today treats `.iter()` as available on `Vec<T>`
and points at this signature for grounding.

### std `output/docs/rust/std/vec/struct.Vec.md` lines 6970-6982

Verbatim:

> ### impl<T, A> Deref for Vec<T, A> where A: Allocator,
> #### type Target = [T]
> The resulting type after dereferencing.
> #### fn deref(&self) -> &[T]
> Dereferences the value.

And lines 2663-2665 ("Methods from `Deref<Target = [T]>`") header
introduce the inherited-methods section. So the `.iter()` method
documented on `primitive.slice.md:1434` is callable on a `Vec<T>`
receiver via this Deref impl. Today does not center the Deref step;
it is structural grounding for "Vec inherits .iter() from slices".

### std `output/docs/rust/std/slice/struct.Iter.md` line 199

Verbatim:

> #### type Item = &'a T

This is the iterator's associated `Item` type — the type yielded each
pass. Today's loop yields `&u64` (= `&'a T` instantiated at `T = u64`).
Probe 4's `let _: &u64 = x;` is the empirical witness.

The full `Iterator` trait impl is on lines 193-211; today reads only
the `Item` line.

### Book `output/docs/rust/book/ch13-02-iterators.md` lines 9-23, 40-50

Verbatim (lines 9-13):

> In Rust, iterators are *lazy*, meaning they have no effect until you
> call methods that consume the iterator to use it up. For example,
> the code in Listing 13-10 creates an iterator over the items in the
> vector `v1` by calling the `iter` method defined on `Vec<T>`.

And Listing 13-11 (lines 40-50):

> ```
> fn main() {
>     let v1 = vec![1, 2, 3];
>     let v1_iter = v1.iter();
>     for val in v1_iter {
>         println!("Got: {val}");
>     }
> }
> ```

This is the audience-level introduction for today's exact shape.
The Book attributes `iter` "defined on `Vec<T>`" (load-bearing for the
lesson body's framing — even though strictly the method lives on
`[T]` and Vec inherits it, the Book and learner-facing convention
treat it as a method on `Vec<T>`). Listing 13-11 is the canonical
two-step shape: bind the iterator, then `for val in v1_iter`. Today's
working probe collapses the two steps into a single `for x in
v.iter()` — Listing 13-11's iterator is unbound, today's is the same
expression in the loop position.

The Book also notes (lines 127-132) that "the values we get from the
calls to `next` are immutable references to the values in the
vector. The `iter` method produces an iterator over immutable
references." This is the framing today uses ("yields a reference to
the element, not the element itself"). The contrast with `into_iter`
(Book line 130) and `iter_mut` (line 132) is named-deferred in the
lesson body.

### Reference `output/docs/rust/reference/expressions/loop-expr.md` line 208

Verbatim:

> A `for` expression is a syntactic construct for looping over elements
> provided by an implementation of `std::iter::IntoIterator`.

This is the rule lesson 022 and 079 already cited. Today reuses it
without re-deriving: the COLLECTION slot accepts any expression whose
type implements `IntoIterator`. `Iter<'_, T>` does (every `Iterator`
is `IntoIterator` via std's blanket impl, named-deferred). Probe 1's
silent compile is the empirical witness.

### Error code `output/docs/rust/error_codes/E0308.md` line 4

Verbatim:

> Expected type did not match the received type.

Probe 2's headline is exactly this error: the surrounding context
(`let bad: u64 = ...`) expected a `u64`; the actual expression
`v.iter()` produced an `Iter<'_, u64>`.

## rmp unlock — `cmp.rs:22` `self.limbs.iter().rev().zip(...)`

Source `output/repos/rmp/src/biguint/cmp.rs` line 22 verbatim:

```rust
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
```

`self.limbs` is field access (lesson 095) on a `BigUInt` whose
`limbs` field has type `Vec<u64>` (per rmp's `basic.rs`, cited in
lesson 107's evidence). `.iter()` on that field is exactly today's
mechanic — receiver `self.limbs` is a `Vec<u64>`, the call returns
an iterator over `&u64`. The `.rev()` and `.zip(...)` adapters chain
on top; today does not install them. The destructuring loop pattern
`for (left, right) in ...` (lesson 073's tuple destructuring composed
with lesson 079's for-loop) is also still not centrally exercised
end-to-end here.

What today unlocks: the *first link* of the chain — `self.limbs.iter()`
— is now a readable composition of installed mechanics. The full
line composes today + lessons 091 (`.rev()`), the future `.zip()`
move, and the future `for (a, b) in ...` destructuring move on
iterator output.

A second instance: `cmp.rs:20` `self.limbs.len().cmp(&other.limbs.len())`
is *not* today's mechanic — the chained call is `.len().cmp(...)`,
where `.len()` returns a `usize` and `.cmp(...)` is a method on
`usize`. That chain is lesson 049's chaining mechanic on different
methods.

## Claim-to-evidence map

- "`v.iter()` is a method on `Vec<T>` that returns an iterator
  yielding each element in order" — `primitive.slice.md:1434`
  verbatim ("Returns an iterator over the slice. The iterator yields
  all items from start to end."); inherited to `Vec<T>` via
  `struct.Vec:6970-6982` Deref impl; Probe 1 transcript.
- "The yielded item type is `&T`" — `slice/struct.Iter.md:199`
  verbatim (`type Item = &'a T`); Probe 4 transcript.
- "`for x in v.iter() { ... }` runs the body once per element"
  — Reference `loop-expr.md:208` verbatim; Probe 1 transcript;
  Book ch13-02 Listing 13-11 (lines 40-50) verbatim.
- "The return value of `v.iter()` has type `Iter<'_, T>`, not `T`"
  — Probe 2 transcript verbatim ("found `Iter<'_, u64>`");
  `error_codes/E0308.md:4`; `primitive.slice.md:1434` (signature).
- "`println!("{}", x)` works for `x: &T` when `T: Display`" — std's
  `fmt::Display` blanket impl `impl<T: ?Sized + Display> Display for
  &T` (named-deferred); Probe 1's output ("10/20/30") is the
  empirical witness.
- "`.iter()` is one specific iterator-producing method; bare `for x
  in v` is a different shape" — Probe 5 transcript (alt compiles and
  produces identical output); Probe 6 transcript (`for x in v`
  surfaces "implicit call to `.into_iter()` that moves `v`").
- "rmp `cmp.rs:22` `self.limbs.iter()` is exactly today's mechanic"
  — `output/repos/rmp/src/biguint/cmp.rs:22` verbatim; Probe 1
  mirrors the `v.iter()` shape on a `Vec<u64>` receiver.

## Negative / contrast probe coverage

The lesson's centered contrastive claim is "`v.iter()` produces an
iterator (a value of type `Iter<'_, T>`), not a `T` value." Probe 2
is the centered contrast: the type-mismatch fires E0308 with both
type names surfaced verbatim ("expected `u64`, found `Iter<'_, u64>`").

Probe 5 (bare `for x in v` working) and Probe 6 (`for x in v`
consumes `v`) are *not* primary contrasts; they are appendix-only
witnesses for the claim "`.iter()` is *one specific* iterator-producing
method, not the only way to iterate." The lesson body names this
distinction in `What Changed` and `What To Ignore For Now`; the
ownership semantics of the alternative are wholesale deferred.

Probe 4 (`let _: &u64 = x;`) is corroborative, not contrastive — it
is a positive empirical witness that `x: &u64`.
