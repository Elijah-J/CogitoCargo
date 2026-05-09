---
id: 155-iterator-position
status: accepted
evidence: ../evidence/155-iterator-position.md
---

# Find the index of the first matching element with `iter.position(|x| ...)`

## The Move

`position` is the third predicate consumer, after `any` (153) and
`all` (154). Same closure shape (`FnMut(Self::Item) -> bool`), same
`&mut self`, same short-circuit-on-`true` rule as `any`. **The only
thing that rotates is the return type:** where `any` and `all`
returned plain `bool`, `position` returns `Option<usize>` (lesson 119
with `T = usize`).

```rust
fn main() {
    let r = (10..20_u32).position(|x| x == 15);
    println!("{:?}", r);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
Some(5)
```

Read that carefully. The matched value `15` and the answer `Some(5)`
are *different numbers*. `(10..20_u32)` yields `10, 11, 12, 13, 14,
15, 16, ...`; `15` is the sixth element, which — counted from `0` —
sits at index `5`. `position` answers "at what index did the closure
first return `true`?", not "what was the matching element?". This is
the load-bearing fact today.

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:2763`:

```text
fn position<P>(&mut self, predicate: P) -> Option<usize>
   where Self: Sized,
         P: FnMut(Self::Item) -> bool,
```

The type-parameter slot is named `<P>` instead of `<F>` — convention
("P for predicate"); not load-bearing. The return slot, `Option<usize>`
instead of `bool`, *is* load-bearing. Stabilized at 1.0.0; local
toolchain 1.95.0.

## Index, not value

The index is a position in the yielded sequence, counted from `0` —
the same `usize` counter as lesson 138's `enumerate`. To watch:

```rust
fn main() {
    let mut count = 0_u32;
    let r = (10..20_u32).position(|x| { count += 1; x == 15 });
    println!("{:?} {}", r, count);
}
```

Output: `Some(5) 6`. Six calls (`x = 10, 11, 12, 13, 14, 15`), five
`false`s then one `true`, short-circuit. The returned index `5` is
one less than the call count `6`, because the first call sits at
index `0`. The braced body composes capture (144) + mutation (023) +
`FnMut` (148), same shape as lesson 154 Probe 4.

A first-element match returns `Some(0)`:
`(5..10_u32).position(|x| x == 5)` is `Some(0)`.

## No match → `None`. Empty → `None`.

`(1..10_u32).position(|x| x == 100)` returns `None`: closure called
nine times, never `true`. `(1..1_u32).position(|x| x == 5)` also
returns `None`: empty range, closure never called. Same `Option<T>`
slot for both cases — dual to lesson 152's empty `reduce`. Contrast
with 153/154: empty-`any` was plain `false`; empty-`all` was plain
`true`. Once the return type is `Option<_>`, the empty case gets its
own slot.

## The iterator survives the call

Same `&mut self` mechanic as `any`/`all`:

```rust
fn main() {
    let mut it = 10..20_u32;
    let r = it.position(|x| x == 15);
    let n = it.next();
    println!("{:?} {:?}", r, n);
}
```

Output: `Some(5) Some(16)`. `it` is still bound; `&mut self` borrows
but does not consume. The next yielded element is `Some(16)` — the
value just past the match. Three distinct numbers in one line: `5`
(index), `15` (matched value, implicit), `16` (next value). Index
counts positions; values are at those positions. Without `let mut
it`, the call fires E0596 — same diagnostic as lessons 131, 153, 154.

## Mental Model Delta

- *Before:* "`any` and `all` return plain `bool` from a single-
  parameter `FnMut(_) -> bool` closure. They short-circuit and leave
  the iterator usable."
- *After:* "`position` is the same shape with the answer rotated to
  `Option<usize>`. `Some(idx)` carries the zero-based index of the
  iterator element where the closure first returned `true`; `None`
  means no element matched, or the iterator was empty. The index is
  a position in the yielded sequence, not the matched value — those
  are different numbers in general."

## Prerequisites

- Installed concepts (load-bearing):
  - **154** (`all`) and **153** (`any`): direct siblings. Reuse
    signature shape, `&mut self` receiver, `FnMut(Self::Item) ->
    bool` bound, and the short-circuit-on-`true` rule from `any`.
    Only the return rotates.
  - **152** (`reduce`): prior `Option<_>`-returning consumer; today's
    `None`-on-empty has the same shape as 152's.
  - **138** (`enumerate`): installs the `usize` iteration index from
    `0`; today's index has the same identity.
  - **148** `FnMut`, **147** parens-bound, **144** capture, **142**
    closure literal, **132** `Self::Item`, **131** `&mut self` rule
    with E0596, **119** `Option<T>` with `Some`/`None`,
    **091/081/080** `Range<u32>`, **077** `usize`, **023** `+=`,
    **013** `==` on integers, **011** `println!` with `{:?}`,
    **003** diagnostic map.
- Cited: 145 (`<F>`/`<P>`), 005 (`let`), 002 (`fn main`), 001 (`rustc`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is
`Some(5)`. Then save the reusable probe and predict its output before
running. The closure is `|x| x == 15` on `10..20_u32`. What index?
What does `it.next()` yield after?

(Answer: `Some(5) Some(16)`. Match at index `5`; iterator now points
at `16`.)

## What Changed

- Signature `fn position<P>(&mut self, predicate: P) -> Option<usize>
  where P: FnMut(Self::Item) -> bool`. Same shape as `any`/`all`;
  only the return rotates.
- Return: `Some(idx)` carries the zero-based index of the first
  match; `None` covers both no-match and empty-iterator.
- The index is a position in the yielded sequence, not the matched
  value. `(10..20_u32).position(|x| x == 15)` returns `Some(5)`.
- Closure bound, receiver, short-circuit-on-`true`, and iterator-
  still-usable rule are unchanged from `any`.
- The corpus names the type parameter `<P>` ("predicate"); functionally
  equivalent to `any`'s `<F>`.

## Check Yourself

```rust
fn main() {
    let mut it = 100..200_u32;
    let r = it.position(|x| x == 103);
    let n = it.next();
    println!("{:?} {:?}", r, n);
}
```

(a) What does it print, and why?

(b) Change `103` to `999`. What does it print?

(Answers: (a) `Some(3) Some(104)`. `103` is at index `3` in `100,
101, 102, 103, 104, ...`. The numbers `3`, `103`, `104` are index,
matched value, and next value. (b) `None None`. No match; iterator
walks to exhaustion; next call returns `None`.)

## What To Ignore For Now

- **`rposition`** — same shape but right-to-left; gates on
  `DoubleEndedIterator`.
- **`find`** — predicate consumer with `&Self::Item` parameter and
  `Option<Self::Item>` return. Pulls in deref-read.
- **`find_map`** — `FnMut(Self::Item) -> Option<B>` bound; later.
- **"Index" vs. "position" naming** — both refer to the same `usize`
  counter; treatment deferred.
- **Methods on `Option<usize>`** — `.unwrap()`, `.map()`, `if let
  Some(i) = r`; deferred since 119.
- **`usize::MAX` overflow on > 2^64 non-matches** — corpus-named at
  `:2775-2785`; impractical to probe.
- **`try_for_each`, `try_fold`** — short-circuit-with-`?`; Try sub-arc.

## Evidence

See `../evidence/155-iterator-position.md`.
