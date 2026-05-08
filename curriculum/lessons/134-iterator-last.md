---
id: 134-iterator-last
status: accepted
evidence: ../evidence/134-iterator-last.md
---

# Pull the final element of a slice iterator with `.last()`

## The Move

Lesson 133 called `.count()` — the smallest stable consumer in the 75
provided methods. Today calls the next one. `.last()` walks the
iterator to exhaustion and hands back the most recent `Some(_)` it saw,
wrapped in another `Option`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let result = v.iter().last();
    println!("{:?}", result);
}
```

`rustc demo.rs` is silent; `./demo` prints `Some(30)`. An empty vec
prints `None` instead. The trait declaration spells the signature
`fn last(self) -> Option<Self::Item> where Self: Sized,`
(`output/docs/rust/std/iter/trait.Iterator.md:448`). Three coupled
facts in that signature:

1. **`self`, no `&`.** Same shape as 133's `count` — lesson-102's
   *consuming* receiver. Calling `iter.last()` moves the iterator; the
   binding cannot be used again. Probe 3 captures E0382 with the same
   `note:` template 133 captured, only `last` substituted into the
   method-name slot.
2. **Return type `Option<Self::Item>`.** *This is the new fact.* Where
   133's `count` returned a primitive `usize`, `last` returns
   `Option<Self::Item>` — the lesson-115 associated-type slot, resolved
   per impl. For the slice iterator over `Vec<u64>` (lesson 123),
   `Self::Item = &u64` (lesson 131), so `v.iter().last()` returns
   `Option<&u64>`. Probes 4 and 5 sandwich this empirically.
3. **Walk `next` to `None`, remember the most recent `Some(_)`.** The
   corpus prose at `:450-454` reads "evaluate the iterator until it
   returns `None`. While doing so, it keeps track of the current
   element. After `None` is returned, `last()` will then return the
   last element it saw." Same template as `count`'s walker, different
   bookkeeping.

The doc page also notes at `:456-458`: "This function might panic if
the iterator is infinite." Named as a corpus fact today; not probed
(constructing an infinite iterator is impractical to run).

`Self: Sized` is named-deferred (same disposition as lesson 133).

## Mental Model Delta

- *Before:* "Lesson 133's `count` returned a `usize` — a primitive.
  Every consumer probably returns a primitive."
- *After:* "`last` is the second consumer. Same self-by-value receiver
  as `count`, but a new return shape: `Option<Self::Item>`. The
  `Self::Item` slot from 132/115 finally appears in a *return* type.
  For the slice iterator over `Vec<u64>`, that slot is `&u64`, so
  `last` returns `Option<&u64>`. The defining walker is `next`-until-
  `None`, same template as `count`; only the bookkeeping differs."

## Prerequisites

- Installed concepts:
  - **Lesson 133** (load-bearing): `(self) -> usize` consumer with the
    walk-`next`-until-`None` template. Today extends to
    `(self) -> Option<Self::Item>` — same self-by-value, same walker;
    the new fact is the return-type slot.
  - **Lesson 132** (load-bearing): the `Iterator` trait declaration
    with 75 provided methods inherited via default bodies. `last` is
    one of them.
  - **Lesson 131** (load-bearing): for the slice iterator,
    `Self::Item = &u64`. Today's `Option<&u64>` return wrapper rests
    on this directly.
  - **Lesson 119** (load-bearing): `Option<T>` / `Some` / `None`.
    Today `T = Self::Item`.
  - **Lesson 102** (load-bearing): `self` is the consuming receiver.
    Probe 3 reuses the E0382 + `note:` template.
  - **Lesson 115** (load-bearing): `Self::Item` is the associated-type
    slot. Today is the first *return* type built from that slot.
  - **Lessons 123, 116, 093, 040, 011, 005, 003, 002, 001** (cited):
    `v.iter()`; default-body trait methods; `{:?}`; dot-call;
    `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe above as `demo.rs`, compile, run; output is
`Some(30)`. Try `let v: Vec<u64> = vec![];` to witness `None`.

*Now the contrast.* `.last()` consumes the iterator. Bind it once and
try to call `.last()` twice. Save `use_after.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let _ = iter.last();
    let _ = iter.last();
}
```

Compile. The diagnostic is `error[E0382]: use of moved value: \`iter\``
with a `note:` block reading verbatim *`last` takes ownership of the
receiver `self`, which moves `iter`* — same template lesson 133
captured for `count`, only the method name substitutes. The `-->` of
the note points at core's `library/core/src/iter/traits/iterator.rs`
at the std source for `Iterator::last`. Read with the lesson 003 map.

## What Changed

- `.last()` is callable on any iterator supplying the lesson-132
  required surface; today calls it on the slice iterator from 123.
- Signature is `fn last(self) -> Option<Self::Item> where Self: Sized,`.
  `self` is the lesson-102 consuming receiver: the call moves the
  iterator. (`Self: Sized` deferred.)
- Return type is `Option<Self::Item>` — the first *provided* Iterator
  method whose return is anchored to the 115 / 132 associated-type
  slot. For the slice iterator over `Vec<u64>`, `Self::Item = &u64`,
  so `v.iter().last()` is `Option<&u64>`. `let x: Option<&u64> =
  v.iter().last();` compiles silent (Probe 4); `Option<u64>` fires
  E0308 with `expected Option<u64>, found Option<&u64>` (Probe 5).
- Internally `last` walks `next` until `None`, remembering the most
  recent `Some(_)` (`trait.Iterator.md:450-454`).
- The doc page warns "This function might panic if the iterator is
  infinite" (`:458`). Named as a corpus fact, not probed.

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let r = iter.last();
    let _ = iter.last();
    println!("{:?}", r);
}
```

(a) What E-code fires, and what does the `note:` block at the method
site say?

(b) Delete the `let _ = iter.last();` line. What prints?

(c) Replace `vec![10, 20, 30]` with `vec![]` (still `Vec<u64>`); keep
(b)'s shape. What prints? What is the type of `r`?

*(Answers: (a) E0382; the `note:` says `last` takes ownership of the
receiver `self`, which moves `iter`. (b) `Some(30)`. (c) `None`; the
type is `Option<&u64>` either way — empty vs non-empty changes the
variant, not the static return type.)*

## What To Ignore For Now

Deferred: `Self: Sized` (still); the panic-if-infinite case (named
only); `.copied()` on `Option<&u64>` to obtain `Option<u64>` (named in
Probe 5's `help:`); `Iterator::copied` / `cloned`; the default body of
`last` in core (a `fold`-based one-liner, readable once closures +
`fold` land); the other 73 provided methods.

## Evidence

See `../evidence/134-iterator-last.md`.
