---
id: 133-iterator-count
status: accepted
evidence: ../evidence/133-iterator-count.md
---

# Count a slice iterator's elements with `.count()`

## The Move

Lesson 132 said the `Iterator` trait declares 75 *provided* methods.
Today calls one. `.count()` asks `next()` until `None` comes back and
returns the number of `Some(_)` results as a `usize`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().count();
    println!("{}", n);
}
```

`rustc demo.rs` is silent; `./demo` prints `5`. Empty vec prints `0`.
The trait declaration spells the signature `fn count(self) -> usize
where Self: Sized,` (`output/docs/rust/std/iter/trait.Iterator.md:416`).
Two coupled facts in that signature:

1. **`self`, no `&`.** `count` takes the receiver *by value* — the
   lesson-102 *consuming* shape. Calling `iter.count()` moves the
   iterator. The binding cannot be used again. Lesson 131's
   `iter.next()` borrowed `&mut self`; today's `.count()` consumes.
2. **Returns `usize`.** The architecture-width unsigned integer from
   lesson 080 — same type `Vec::len` returned at lesson 107.

`Self: Sized` is named-deferred; it appears on most provided methods
and is its own future move.

## Mental Model Delta

- *Before:* "Lesson 132 told me there are 75 provided Iterator methods.
  I have not called one yet. Lesson 131's `.next()` left the iterator
  binding usable for another call. I assume every iterator method
  works that way."
- *After:* "`.count()` is one of the 75. It is *consuming*: it takes
  `self` by value (the lesson-102 shape), so after `iter.count()` the
  binding `iter` is gone. It returns a `usize`, the standard
  collection-size type. Internally it is just a loop calling `next`
  until `None` and tallying — empirically visible in Probe 6."

## Prerequisites

- Installed concepts:
  - **Lesson 132** (load-bearing): the `Iterator` trait's 75 provided
    methods, inherited via default bodies. `count` is one of them.
  - **Lesson 131** (load-bearing): `.next()` on a slice iterator
    returns `Option<&T>`. `count`'s definition is "call `next` until
    `None`, return how many `Some(_)`s came back."
  - **Lesson 102** (load-bearing): `self` (no `&`) is the consuming
    receiver. Using the binding after such a call fires E0382 with a
    `note:` block naming the method as taking ownership. The contrast
    witnesses this on `Iterator::count`.
  - **Lesson 080** (load-bearing): `usize` is one specific integer
    type. `count` returns *that one*.
  - **Lessons 123, 119, 116, 107, 040, 011, 005, 003, 002, 001**
    (cited): `v.iter()`; `Option`/`Some`/`None`; default trait-method
    bodies; `Vec::len -> usize`; dot-call; `println!`; `let`;
    diagnostic map; `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe above as `demo.rs`, compile, run; output is
`5`. Try `let v: Vec<u64> = vec![];` to witness `0`.

*Now the contrast.* `.count()` consumes the iterator. Bind it once
and try to call `.count()` twice. Save `use_after.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let n = iter.count();
    let _ = iter.count();
    println!("{}", n);
}
```

Compile:

```text
error[E0382]: use of moved value: `iter`
 --> use_after.rs:5:13
  |
3 |     let iter = v.iter();
  |         ---- move occurs because `iter` has type `std::slice::Iter<'_, u64>`, which does not implement the `Copy` trait
4 |     let n = iter.count();
  |                  ------- `iter` moved due to this method call
5 |     let _ = iter.count();
  |             ^^^^ value used here after move
  |
note: `count` takes ownership of the receiver `self`, which moves `iter`
 --> /rustc/.../library/core/src/iter/traits/iterator.rs:225:13
help: you can `clone` the value and consume it, but this might not be your desired behavior
```

Read with the lesson 003 map. Same E-code (`E0382`) and same `note:`
shape as lesson 102's user-defined `into_inner`, but pointed at
`Iterator::count` in core. The `-->` location names the std source
file. The trait declaration's `fn count(self) -> usize` is what
produced that note.

## What Changed

- `.count()` is callable on any iterator that supplies the lesson-132
  required surface; today calls it on the slice iterator from 123.
- The signature is `fn count(self) -> usize where Self: Sized,`
  (`Self: Sized` deferred). `self` is the lesson-102 consuming
  receiver: the call moves the iterator.
- Return type is `usize` (lesson 080). `let n: usize = v.iter().count();`
  compiles; `let n: u64 = ...` fires E0308 with `expected u64, found
  usize` (appendix Probes 4 + 5).
- Internally `count` calls `next` until `None` and tallies `Some(_)`
  returns (`output/docs/rust/std/iter/trait.Iterator.md:420-421`,
  empirically visible in Probe 6's `Trace` iterator). For an empty
  iterator, `next` is still called once and returns `None`; `count`
  returns `0`.
- Counting elements of an iterator with more than `usize::MAX` items
  "either produces the wrong result or panics" — corpus fact at
  `trait.Iterator.md:424-434`. Not probed.

## Check Yourself

```rust
fn main() {
    let xs: Vec<u64> = vec![7, 8, 9, 10];
    let it = xs.iter();
    let n = it.count();
    let _ = it.count();
    println!("{}", n);
}
```

(a) What E-code fires, and what does the `note:` block at the method
site say?

(b) Now delete the `let _ = it.count();` line. Does it compile? What
prints?

(c) Without binding to `it` at all — write `let n = xs.iter().count();`
twice on consecutive lines, then `println!("{}", n);`. Does *that*
compile? Why does it differ from (a)?

*(Answers: (a) E0382. The `note:` says `count` takes ownership of the
receiver `self`, which moves `it`. (b) Yes; `4`. (c) Yes; each
`xs.iter()` builds a fresh iterator, so each `.count()` consumes its
own brand-new walker — there is no second use of one moved binding.)*

## What To Ignore For Now

Deferred: `Self: Sized` and the `Sized` / `?Sized` bound (named on
`count`'s signature, future move); the explicit `Self::Item` link in
provided-method signatures (composes 132 + 115); `usize::MAX` overflow
and panic behavior of `count` (named only); `.clone()` on iterators
(named in the contrast's `help:` line); `for` desugaring through
`IntoIterator`; closures (gate 27 other Iterator methods); the other
74 provided methods.

## Evidence

See `../evidence/133-iterator-count.md`.
