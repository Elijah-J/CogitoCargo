---
id: 141-iterator-size-hint
status: accepted
evidence: ../evidence/141-iterator-size-hint.md
---

# Read remaining-length bounds with `iter.size_hint()`

## The Move

Lessons 133-140 walked through the consuming-`self` and `&mut self`
methods on `Iterator`: `count`, `last`, `nth`, `take`, `skip`,
`enumerate`, `fuse`, `step_by`. Today's `size_hint()` is the next
sibling, but its receiver is new. The trait declaration at
`output/docs/rust/std/iter/trait.Iterator.md:345` spells:

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

`&self` (lesson 100) — *not* `self`, *not* `&mut self`. This is the
first stable Iterator provided method whose receiver is `&self`. The
return type is a bare 2-tuple: `(lower_bound, upper_bound)`. The
upper bound is wrapped in `Option<usize>` because some iterators
cannot give an exact upper bound; in that case the slot is `None`.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];

    let iter = v.iter();
    println!("{:?}", iter.size_hint());

    let mut iter = v.iter();
    let _ = iter.next();
    println!("{:?}", iter.size_hint());

    let empty: Vec<u64> = vec![];
    println!("{:?}", empty.iter().size_hint());
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
(3, Some(3))
(2, Some(2))
(0, Some(0))
```

Walk: a fresh `v.iter()` over a 3-element vec reports lower=3,
upper=`Some(3)`. After one `.next()` advances the cursor, the
remaining length is 2 — and the slice iterator still knows it
exactly, so the upper slot stays `Some(2)`. The empty case reports
`(0, Some(0))`. Slice iterators can always give an *exact* count, so
their lower and upper bounds match.

## The new fact: `&self` on a provided Iterator method

Lesson 131 installed `&mut self` on `next` — the binding had to be
`let mut iter`. Today's `size_hint` takes `&self`, so the binding
does *not* need `mut`, the cursor is not advanced, and the call can
be repeated. Drop `let mut` from lesson 131's `.next()` example and
you get E0596; here you don't (Probe 2 of the appendix witnesses
three sequential calls on `let iter`).

A type-pin probe (`let _x: u32 = v.iter().size_hint();`) fires E0308
with `expected u32, found (usize, Option<usize>)`. The `note:` line
names the *kind*: `tuple`. The wrapper is *not* `Option` around the
whole thing — the tuple comes back directly; only the *upper* slot
is an `Option`.

## Three corpus claims

- `trait.Iterator.md:347-354`: the first slot is the *lower bound*,
  always known; the second slot is the *upper bound*, which is
  `Some(n)` if known and `None` if either there is no known upper
  bound or it exceeds `usize::MAX`.
- `:371-372`: "The default implementation returns `(0, None)` which
  is correct for any iterator." If an `Iterator` impl does not write
  a custom `size_hint`, calls return `(0, None)`. The slice iterator
  overrides the default and returns the exact count.
- `:358-369`: `size_hint()` is a *hint*, not a guarantee. "A buggy
  iterator may yield less than the lower bound or more than the
  upper bound." Intended for optimizations such as reserving space;
  do not rely on it for memory safety.

## Mental Model Delta

- *Before:* "Iterator's `next` takes `&mut self`; provided methods
  seen so far take `self` (consuming) or `&mut self` (`nth`). `&self`
  from lesson 100 has not yet appeared on a stdlib iterator method."
- *After:* "`size_hint()` takes `&self`. Three sequential calls on
  `let iter` (no `mut`) succeed and return the same tuple — the
  cursor is not advanced. The return is `(usize, Option<usize>)`:
  lower bound first, upper bound second. The upper slot is `None`
  when the iterator cannot give an exact answer. The default impl
  returns `(0, None)`. It is a *hint*, not a guarantee."

## Prerequisites

- Installed concepts:
  - **Lesson 140** (load-bearing): the closure-free Iterator arc
    pre-`size_hint`. Today closes that arc per audit §5 step 11.
  - **Lesson 132** (load-bearing): `Iterator` trait with 75 provided
    methods. `size_hint` is one. Synopsis line at `:18` ends in
    `{ ... }` — lesson 116's default-body marker.
  - **Lesson 131** (load-bearing): `.next()` takes `&mut self`,
    needs `let mut iter`. Today's `size_hint` takes `&self`, no `mut`
    — the centered contrast.
  - **Lesson 119** (load-bearing): `Option<T>` / `Some` / `None`.
    Today's `T = usize`, so the upper slot is `Option<usize>`.
  - **Lesson 100** (load-bearing): `&self` receiver shape on
    user-defined inherent impls. Today is its first appearance on a
    stdlib provided Iterator method.
  - **Lessons 072, 073, 080, 116, 040, 011, 005, 003, 002, 001**
    (cited): tuple type `(A, B)`; `let (a, b) = pair;`; `usize`;
    default-body trait methods; dot-call; `println!`; `let`;
    diagnostic map; `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the
three lines above. Now witness `&self` directly — save `twice.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    println!("{:?}", iter.size_hint());
    println!("{:?}", iter.size_hint());
    println!("{:?}", iter.size_hint());
}
```

`rustc twice.rs` compiles silently; `./twice` prints three identical
lines `(3, Some(3))`. Note the `let iter` — no `mut`. Lesson 131's
`.next()` on `let iter` (without `mut`) fired E0596. Today's
`.size_hint()` does not, because `&self` is the immutable borrow.

## What Changed

- Signature `fn size_hint(&self) -> (usize, Option<usize>)`
  (`trait.Iterator.md:345`). Receiver is `&self`.
- The binding does not need `mut`. Three sequential calls succeed
  on a `let iter` (Probe 2 of the appendix).
- The return is a bare 2-tuple. First slot: lower bound (`usize`).
  Second slot: upper bound (`Option<usize>` — `Some(n)` or `None`).
- The default impl returns `(0, None)`. Slice iterators override
  and return an exact count.
- It is a *hint*, not a guarantee. Used for optimizations; do not
  rely on it for memory safety (`:358-369`).

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![1, 2, 3, 4];
    let iter = v.iter();
    println!("{:?}", iter.size_hint());
    println!("{:?}", iter.size_hint());

    let mut iter = v.iter();
    let _ = iter.next();
    let _ = iter.next();
    println!("{:?}", iter.size_hint());
}
```

(a) Does it compile silently? What three lines does it print?

(b) Why does the *first* binding `let iter` not need `mut`, while
the second binding `let mut iter` does?

*(Answers: (a) Yes. `(4, Some(4))`, `(4, Some(4))`, `(2, Some(2))`.
(b) The first binding only calls `size_hint` (`&self`) — no
mutation. The second binding calls `.next()` (`&mut self`), which
requires the binding to be declared `mut`.)*

## What To Ignore For Now

Deferred: a custom `Iterator` impl that overrides `size_hint` (out
of scope today; centered fact is the `&self` receiver, not custom
impls). Iterators whose `size_hint` returns `None` upper — the doc
gives `0..` at `:407-411` as an example, but infinite ranges via
`Range` are still gated (deferred since 022). The `(0, None)` /
`extend` / `collect` reservation interaction. The `ExactSizeIterator`
trait, which turns the upper-slot `Option<usize>` guarantee into a
stronger contract. The closure sub-arc (audit §4.4.1).

## Evidence

See `../evidence/141-iterator-size-hint.md`.
