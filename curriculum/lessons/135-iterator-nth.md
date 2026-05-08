---
id: 135-iterator-nth
status: accepted
evidence: ../evidence/135-iterator-nth.md
---

# Index into a slice iterator with `iter.nth(n)`

## The Move

Lessons 133 and 134 called `.count()` and `.last()` — provided methods
that *consume* the iterator (receiver `self`). Today calls one that
does *not* consume the binding: `nth`. Bind the iterator with
`let mut`, then call `iter.nth(n)` to fetch the element at zero-based
index `n` from the *current cursor*:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    println!("{:?}", iter.nth(1));
    println!("{:?}", iter.nth(0));
    println!("{:?}", iter.nth(0));
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
Some(20)
Some(30)
Some(40)
```

The trait declaration spells the signature
`fn nth(&mut self, n: usize) -> Option<Self::Item>`
(`output/docs/rust/std/iter/trait.Iterator.md:507`). Three coupled
facts in that signature, each new today:

1. **`&mut self`, not `self`.** This is the FIRST *provided* Iterator
   method the run installs that takes `&mut self`. Lesson 131 caught
   the same receiver shape on the *required* method `next`. Calling
   `iter.nth(1)` does not move `iter` — the binding stays usable for
   the next `nth` call. (Compare lesson 133's `.count()`: one call
   moves the iterator and the binding is gone.)
2. **A second parameter, `n: usize`.** Iterator methods *can* take
   non-`self` arguments. This is the first place a learner sees that
   on `Iterator`. The type is `usize` (lesson 080) — the same indexing
   integer that lesson 077 used for arrays.
3. **Return type `Option<Self::Item>`** — same wrapper as lesson 134's
   `.last()`. For the slice iterator over `Vec<u64>` (lessons 123,
   131), `Self::Item = &u64`, so `iter.nth(1)` is `Option<&u64>`.

Walk the working probe:

- `iter` starts at `[10, 20, 30, 40, 50]`.
- `iter.nth(1)` consumes `&10` *and* `&20`, returns `Some(&20)`. Cursor
  is now past `&20`.
- `iter.nth(0)` returns the next element, `&30`. Cursor past `&30`.
- `iter.nth(0)` returns `&40`.

That matches the corpus prose at `:514-517`: "all preceding elements,
as well as the returned element, will be consumed from the iterator
... calling `nth(0)` multiple times on the same iterator will return
different elements." `nth` is essentially "advance through `n+1`
`next` calls and return the last one."

If `n` is past the end, `nth` returns `None` — no panic (`:519-520`).
`Self: Sized` does not appear on `nth`'s signature; named-deferred
either way.

## Mental Model Delta

- *Before:* "Lessons 133 and 134's consumers took `self` and the
  binding was gone after one call. I assume every Iterator method
  with arguments works that way."
- *After:* "`nth` is a different shape. Receiver is `&mut self` — same
  as the required `next` from lesson 131 — so the binding *survives*
  one call and is reusable for another. But each call still advances
  the cursor: `nth(0)` twice in a row returns *different* elements.
  Methods on `Iterator` can also take non-`self` arguments — `n: usize`
  here. The return wrapper is the same `Option<Self::Item>` lesson 134
  installed; for slice iterators that is `Option<&u64>`. Past the end,
  `None`, no panic."

## Prerequisites

- Installed concepts:
  - **Lesson 134** (load-bearing): installs the return-slot
    `Option<Self::Item>` on a provided Iterator method, with
    `Self::Item = &u64` for slice iterators over `Vec<u64>`. Today
    reuses that exact slot; the new fact is the receiver and the
    second parameter.
  - **Lesson 132** (load-bearing): the `Iterator` trait with 75
    provided methods. `nth` is one of them; today is the first
    provided method that takes `&mut self`.
  - **Lesson 131** (load-bearing): `next` takes `&mut self` on a
    slice iterator and advances the cursor. `nth` is structurally a
    walk of `n+1` `next` calls; the cursor-advance semantic transfers.
    The required-`mut`-binding rule and the E0596 contrast also
    transfer.
  - **Lesson 119** (load-bearing): `Option<T>` / `Some` / `None`.
    Today's `T = Self::Item`.
  - **Lesson 115** (load-bearing): `Self::Item` is the associated-type
    slot.
  - **Lesson 101** (load-bearing): `&mut self` is the third receiver
    shape — read+write through a mutable borrow without consuming.
    Today is the first place this shape appears on a *provided*
    Iterator method.
  - **Lesson 006** (load-bearing): `let mut name = value;`. Without
    `mut` on the binding, the dot call on `&mut self` fires E0596.
  - **Lesson 080** (cited): `usize` — the indexing integer.
  - **Lessons 123, 116, 093, 040, 011, 005, 003, 002, 001** (cited):
    `v.iter()`; default-body trait methods; `{:?}`; dot-call;
    `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the three
lines above. Try `iter.nth(100)` after binding `let mut iter = ...`
to witness `None` (past-the-end is not a panic).

*Now the contrast.* `nth` takes `&mut self`. Drop `mut` from the
binding — change `let mut iter = v.iter();` to `let iter = v.iter();`
— and compile:

```text
error[E0596]: cannot borrow `iter` as mutable, as it is not declared as mutable
 --> no_mut.rs:4:22
  |
4 |     println!("{:?}", iter.nth(1));
  |                      ^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut iter = v.iter();
  |         +++
```

Read with the lesson 003 map. *Headline* `E0596`. *Location*
`no_mut.rs:4:22`, on `iter`. *Source excerpt* carets `iter` and labels
it `cannot borrow as mutable`. *Help* writes the fix
`let mut iter = v.iter();` with `+++` marking the insert. Same E-code
and same shape as lesson 131's contrast for `.next()` — because both
methods take `&mut self`.

## What Changed

- Signature is `fn nth(&mut self, n: usize) -> Option<Self::Item>`
  (`trait.Iterator.md:507`). No `Self: Sized` bound.
- `&mut self`: the call does not move the iterator. The binding
  survives. `iter.nth(1); iter.nth(0)` on the same `iter` compiles
  and yields different elements per call.
- Each call advances the cursor by `n+1` positions. Multiple `nth(0)`
  calls return different elements (`trait.Iterator.md:514-517`).
- Return type is `Option<Self::Item>` — same wrapper as `.last()`
  (lesson 134). For slice iters over `Vec<u64>`, that is `Option<&u64>`
  (Probes 5 and 6 sandwich this with E0308).
- Past the end, `nth` returns `None`, no panic (Probe 3: `nth(100)` on
  five-element iter prints `None`).
- Second parameter slot `n: usize` — first provided Iterator method
  with a non-`self` argument visible to the run.

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    println!("{:?}", iter.nth(1));
    println!("{:?}", iter.nth(1));
}
```

(a) Does it compile silently? What two lines does it print?

(b) Drop `mut` from `let mut iter`. What E-code fires, and what does
the `help:` propose?

(c) Replace the second `iter.nth(1)` with `iter.nth(0)`. What does
that single line print? Why?

*(Answers: (a) Yes; `Some(20)` then `None`. After the first `nth(1)`
the cursor sits past `&20`, leaving only `&30`; the second `nth(1)`
needs two more positions, finds only one, returns `None`. (b) E0596
with `help:` writing `let mut iter = v.iter();` and `+++` for the
insert. (c) `Some(30)`: `nth(0)` returns the next element from the
cursor, `&30`.)*

## What To Ignore For Now

Deferred: the `Self: Sized` bound (still — and notably absent from
`nth`'s signature); the
default body of `nth` in core (uses an internal `for`-loop with
`self.next()`, readable once the `for` desugar lands); `.copied()`
on `Option<&u64>` (named in Probe 6's `help:`); short-circuit
behavior; `usize::MAX` overflow corner cases; `nth_back` (a sibling
on `DoubleEndedIterator`); the other 72 provided methods.

## Evidence

See `../evidence/135-iterator-nth.md`.
