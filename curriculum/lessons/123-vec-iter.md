---
id: 123-vec-iter
status: accepted
evidence: ../evidence/123-vec-iter.md
---

# Walk a `Vec<T>`'s elements with `v.iter()` in a `for` loop

## The Move

Lesson 079 installed `for x in array { ... }` for fixed-size arrays.
Lesson 107 built `Vec<T>` values with `vec![]` but stopped at `.len()`
and `v[i]` — the deferred sibling note pointed at iteration. Today
installs the standard way to walk a `Vec<T>` element-by-element: the
`.iter()` method, fed into a `for`-loop.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter() {
        println!("{}", x);
    }
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints three lines:

```text
10
20
30
```

Read it left-to-right. `v.iter()` is the lesson-040 dot-call shape:
receiver `v`, dot, method `iter`, empty argument list. The return
value is *not* a number — it is an *iterator*: an object that hands
out the vec's elements one at a time, in order, until they run out.
The `for` from lesson 079 takes that iterator in its collection slot
and runs the body once per yielded element. Three elements, three
passes, three printed lines.

The std `primitive.slice` page gives `pub fn iter(&self) -> Iter<'_, T>`
and one-liner "Returns an iterator over the slice. The iterator yields
all items from start to end." `Vec<T>` inherits this method (the
appendix points at the exact line); for today, treat `.iter()` as a
method on `Vec<T>`.

## Mental Model Delta

- *Before:* "`for X in C { ... }` works on ranges (lesson 022) and on
  arrays (lesson 079). For a `Vec<T>` I have `vec![]`, `.len()`,
  `v[i]` (lesson 107) — but no installed shape that walks every
  element."
- *After:* "I write `v.iter()` to get an *iterator* over the vec — an
  object that hands out the elements one at a time. I drop that into
  the same `for X in COLLECTION { ... }` shape and the loop runs once
  per element. The yielded value is a reference to the element, not
  the element itself; for `println!("{}", x)` the distinction is
  invisible. The return type of `.iter()` is named `Iter<'_, T>`, but
  I do not need to type that name to use the loop. `.iter()` is *one
  specific* iterator-producing method; the bare `for x in v` shape
  exists too and behaves differently (deferred)."

## Prerequisites

- Installed concepts:
  - **Lesson 107** (load-bearing): `Vec<T>` construction with `vec![]`
    and the `Vec<u64>` annotation. Today's `.iter()` is the explicitly
    deferred sibling from 107's "What To Ignore."
  - **Lesson 079** (load-bearing): `for X in COLLECTION { ... }` over
    a runtime collection. Today extends the COLLECTION slot from
    arrays to `v.iter()` on `Vec<T>`.
  - **Lesson 040** (load-bearing): `value.method(args)`. `v.iter()` is
    that shape with empty argument list.
  - **Lesson 011** (load-bearing): `println!("{}", x)` with one
    positional `{}`. Today's `x` has type `&u64`; std prints behind
    the reference — named-deferred, the appendix points at the rule.
  - **Lessons 080, 019** (cited): `u64` element type; the `: TYPE`
    annotation slot, used as `Vec<u64>`.
  - **Lessons 001, 002, 003, 005** (cited): `rustc demo.rs && ./demo`
    silent on success; `fn main` entry; the diagnostic four-part map;
    `let v = ...;`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs` with
the source above. Compile and run:

```console
$ rustc demo.rs
$ ./demo
10
20
30
```

Three elements, three lines, in vec order.

Now the contrast. `v.iter()` returns an *iterator*, not a `u64`. To
witness this directly, save `broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let bad: u64 = v.iter();
}
```

Compile:

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
```

Read with the lesson 003 map. Headline `error[E0308]: mismatched
types` — the actual type of an expression did not match the type the
surrounding context expected. The inline label `expected u64, found
Iter<'_, u64>` names both types directly.

That is today's centered claim made empirical: `v.iter()` produces a
value of type `Iter<'_, u64>`, *not* a `u64`. The `for`-loop unpacks
the iterator into elements; treating the iterator *as* an element
fails at compile time.

## What Changed

- `Vec<T>` has an `.iter()` method. Calling it produces an *iterator*
  over the vec — an object that yields each element once, in order.
- `for x in v.iter() { ... }` runs the body once per element. `x` is
  bound to a reference to that pass's element (`&T`); for `println!`
  this is invisible.
- The return value of `v.iter()` is *not* a `T` value. Its actual type
  is `Iter<'_, T>`. Binding it where a `T` is expected fires E0308
  with a clear `expected T, found Iter<'_, T>` label.
- `.iter()` is one specific iterator-producing method on `Vec<T>`, not
  the only way to walk a vec. The bare `for x in v` shape also walks
  the elements but composes differently with ownership — deferred.
- This makes the *first link* of rmp's `src/biguint/cmp.rs:22`
  `self.limbs.iter().rev().zip(other.limbs.iter().rev())` readable:
  `self.limbs` is field access on a `Vec<u64>`; `.iter()` is exactly
  today's method call. The chained `.rev()` and `.zip()` compose later.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![7, 8, 9, 10];
    for n in xs.iter() {
        println!("n = {}", n);
    }
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile silently? How many lines does it print?

(b) On the second pass, what value is bound to `n`?

(c) You replace line 3 with `let total: u64 = xs.iter();`. What E-code
does rustc emit, and which two type names does the inline label name?

*(Answers: (a) Yes; four lines: `n = 7`, `n = 8`, `n = 9`, `n = 10`.
(b) `8` — the loop yields elements in vec order. (c) E0308; inline
label `expected u64, found Iter<'_, u64>`.)*

## What To Ignore For Now

Today installs only `v.iter()` on `Vec<T>` consumed by a `for`-loop.
Real and deferred:

- *`.iter_mut()`* — mutable-reference sibling. Yields `&mut T`.
- *`.into_iter()`* — consuming sibling. Yields `T` (taking ownership
  of the vec). Probe 6 in the appendix shows the bare `for x in v`
  shape calling this implicitly.
- *`for x in &v` and `for x in v`* — the shorthand shapes that desugar
  via `IntoIterator` impls. Probes in the appendix witness one of
  them; today centers `.iter()`.
- *`.next()`* — the explicit pull form. Each call returns `Option<&T>`.
  Composes lesson 119 with the iterator machinery.
- *Iterator adapters* — `.rev()`, `.zip()`, `.map()`, `.filter()`,
  `.fold()`, `.sum()`, `.collect()`, `.enumerate()`. Each composes on
  the output of `.iter()` but is its own move.
- *The `Iterator` trait* (`type Item; fn next(&mut self) -> Option<...>`).
  Named-deferred.
- *The `IntoIterator` trait* — the trait the `for`-loop desugar uses
  to accept ranges, arrays, vecs, and references uniformly. Named-deferred.
- *Iterator lifetimes* — the `'_` in `Iter<'_, T>`. Wholesale deferred.
- *The named `Iter<'_, T>` type* — surfaced only in today's diagnostic;
  you never write it.
- *`.iter()` on slices and other collections* — same method shape,
  different receivers; deferred.

## Evidence

See `../evidence/123-vec-iter.md`.
