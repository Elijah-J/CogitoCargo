---
id: 124-iter-rev
status: accepted
evidence: ../evidence/124-iter-rev.md
---

# Walk a `Vec<T>` back-to-front with `v.iter().rev()`

## The Move

Lesson 091 installed `.rev()` on a parenthesized range, so
`(1..4).rev()` yielded `3, 2, 1`. Lesson 123 installed `v.iter()` to
produce an iterator over a `Vec<T>`. Today combines them: chain
`.rev()` onto the iterator returned by `.iter()` to walk the vec's
elements *back-to-front*.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter().rev() {
        println!("{}", x);
    }
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints three lines:

```text
30
20
10
```

Read the chain left-to-right (lesson 049). `v.iter()` is the
lesson-040 dot-call shape and returns an iterator (type
`Iter<'_, u64>` per lesson 123). `.rev()` is *that same dot-call shape
applied to the iterator*: receiver, dot, `rev`, empty argument list.
It returns a new iterator that yields the same elements, in reversed
order. The `for`-loop from lesson 079 takes the final iterator in its
COLLECTION slot. Three elements, three printed lines — vec order
reversed.

The std `Iterator::rev` page gives the framing verbatim: "Reverses an
iterator's direction. ... After using `rev()`, an iterator will
instead iterate from right to left."

## Mental Model Delta

- *Before:* "I know `(1..4).rev()` reverses a *range* (lesson 091),
  and I know `v.iter()` produces an iterator over a `Vec<T>`
  (lesson 123). I have not seen `.rev()` applied to anything else.
  To walk a vec backwards I would need an index loop with arithmetic."
- *After:* "`.rev()` is the same method name I learned on ranges, and
  it works on the iterator that `.iter()` produces too. I write
  `v.iter().rev()` — chained dot-calls (lesson 049) — and the for-loop
  walks the elements back-to-front. The receiver changed (range to
  slice-iterator); the semantics did not. The trait machinery that
  makes both work is named-deferred."

## Prerequisites

- Installed concepts:
  - **Lesson 123** (load-bearing): `v.iter()` returns an iterator.
    Today chains `.rev()` on that result.
  - **Lesson 091** (load-bearing): `.rev()` reverses iteration order
    (witnessed there on a range). Today extends the same method-name
    to a different receiver type — the iterator from `.iter()`.
  - **Lesson 049** (load-bearing): chained dot-calls
    `expr.method1().method2()`. `v.iter().rev()` is exactly that
    shape, with empty argument lists at each step.
  - **Lesson 040** (cited): the dot-call grammar; reused at both
    calls.
  - **Lesson 107** (cited): `Vec<T>` construction and the
    `Vec<u64>` annotation slot.
  - **Lesson 079** (cited): `for X in COLLECTION { ... }`; today's
    COLLECTION is a chained iterator.
  - **Lessons 011, 001, 002, 003, 005** (cited): `println!("{}", x)`;
    `rustc demo.rs && ./demo` silent on success; `fn main` entry; the
    diagnostic four-part map; `let`.
  - **Lessons 080, 019** (cited): `u64`; the `: TYPE` annotation slot.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs` with
the source above. Compile and run:

```console
$ rustc demo.rs
$ ./demo
30
20
10
```

Now the contrast. `.rev()` is callable on the *iterator*, not on the
vec. Save `broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let r = v.rev();
    println!("{:?}", r);
}
```

Compile:

```text
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
```

Read with the lesson 003 map. Headline `error[E0599]: no method named
\`rev\` found for struct \`Vec<u64>\`` — lesson-100's missing-method
diagnostic. The inline label states today's structural fact directly:
"`Vec<u64>` is not an iterator." `.rev()` is a method on iterators, so
you must produce one first — `v.iter()` today. (The `help:` line's
`v.into_iter()` is the consuming variant; deferred.)

## What Changed

- `.rev()` works on the iterator returned by `.iter()`, not just on a
  range. `v.iter().rev()` is a new iterator that yields the same
  elements as `v.iter()` but in reversed order.
- `v.iter().rev()` is the chained-dot-call shape from lesson 049, with
  empty argument lists at each step.
- `for x in v.iter().rev() { ... }` runs the body once per element,
  back-to-front. The binding `x` still has type `&T` (lesson 123).
- `.rev()` is *not* a method on `Vec<T>` itself. Calling `v.rev()`
  fires E0599 with the inline label "`Vec<u64>` is not an iterator."
- This makes the *second link* of rmp's `src/biguint/cmp.rs:22` chain
  `self.limbs.iter().rev().zip(other.limbs.iter().rev())` readable:
  `.rev()` on `self.limbs.iter()` is exactly today's mechanic. The
  `.zip(...)` adapter still composes later.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![7, 8, 9];
    for n in xs.iter().rev() {
        println!("n = {}", n);
    }
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile silently? What does it print, in order?

(b) You replace line 3 with `for n in xs.rev() {`. What error code
fires, and what does the inline label say?

*(Answers: (a) Yes; three lines: `n = 9`, `n = 8`, `n = 7` — vec
order reversed. (b) E0599; the inline label is "`Vec<u64>` is not an
iterator." `.rev()` is a method on iterators, not on `Vec<T>`.)*

## What To Ignore For Now

Today installs only `.rev()` chained on `v.iter()`. Deferred:

- *The `DoubleEndedIterator` trait* — what makes `.rev()` callable on
  a particular iterator. Both ranges and slice iterators implement
  it; today only states the empirical fact. Named-deferred.
- *The `Rev<I>` adapter struct* — the type that `.rev()` returns.
  Surfaced in the std signature only.
- *`.rev()` on iterators that are not double-ended* — some iterator
  adapters lose the property. Today does not exercise the boundary.
- *The `help:` line's `.into_iter().rev()` form* — the consuming
  variant. Composes lesson 123's named-deferred `.into_iter()`.
- *Other iterator adapters* — `.zip()`, `.map()`, `.filter()`,
  `.collect()`, etc. Each is its own move.
- *Materializing a reversed `Vec<T>`* — `v.iter().rev().collect()` or
  similar. Composes today with `.collect()`, deferred.

## Evidence

See `../evidence/124-iter-rev.md`.
