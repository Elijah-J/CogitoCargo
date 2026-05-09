---
id: 153-iterator-any
status: accepted
evidence: ../evidence/153-iterator-any.md
---

# Test a range with a predicate using `iter.any(|x| ...)`

## The Move

Lessons 149-152 installed four closure-driven Iterator methods, and
*all* of them consumed `self` — once you called them, the iterator
binding was gone. Today's method, `any`, is the first one whose
receiver is `&mut self`, and the first one with **short-circuit**
semantics: it stops calling the closure as soon as the closure
returns `true`.

```rust
fn main() {
    let r = (1..10_u32).any(|x| x == 5);
    println!("{}", r);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
true
```

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:2599`:

```text
fn any<F>(&mut self, f: F) -> bool
   where Self: Sized,
         F: FnMut(Self::Item) -> bool,
```

Read it segment by segment with the lesson 147 grammar:

- `<F>` — one type parameter for the closure (lesson 145).
- `(&mut self, f: F)` — receiver `&mut self`, *not* `self`. The
  iterator binding is **still usable after the call returns**. The
  borrow rule is the same as lesson 131's `next()`: the binding must
  be `let mut it = ...`. Probe 7 in the appendix witnesses the E0596
  that fires without `mut`.
- `-> bool` — return type is the primitive `bool` (lesson 013). Not
  `()` like `for_each`, not `Option<_>` like `reduce`. Just `true`
  or `false`.
- `F: FnMut(Self::Item) -> bool` — parens-bound (147), `FnMut` (148),
  `Self::Item` slot (132). Single parameter slot — no comma, no
  second slot. For `Range<u32>`, `Self::Item = u32`, so `x` is a
  `u32` and `x == 5` is `==` on two `u32` values (lesson 013) which
  produces `bool`.

Stabilized at 1.0.0; local toolchain is 1.95.0.

## The iterator survives the call

The most informative probe in this lesson watches the iterator after
`.any(...)` returns:

```rust
fn main() {
    let mut it = 1..10_u32;
    let found = it.any(|x| x == 5);
    let next = it.next();
    println!("{} {:?}", found, next);
}
```

Output: `true Some(6)`. Two structural facts at once:

1. `it` is **still bound**. We just called `it.next()` (lesson 131)
   on the same binding. With `for_each`, `map`, `fold`, or `reduce`
   that line would have fired E0382 "use of moved value: `it`".
2. `it.next()` returned `Some(6)`, **not** `Some(1)` or `None`. `any`
   walked `1, 2, 3, 4, 5`, matched at `5`, and **stopped advancing**.
   That is what *short-circuit* means concretely: the iterator is
   consumed up to and including the first match.

Corpus prose at `trait.Iterator.md:2608-2610`: "`any()` is
short-circuiting; in other words, it will stop processing as soon as
it finds a `true` ... ."

## What the closure returns drives the answer

`any` calls the closure on each element in source order and asks: did
*any* call return `true`?

| Probe | Source | Closure | Closure calls | Returns |
| ----- | ------ | ------- | ------------- | ------- |
| 1     | `1..10_u32` | `\|x\| x == 5`   | 5 (stops at `x = 5`) | `true`  |
| 3     | `1..1_u32`  | `\|x\| x == 5`   | 0 (no elements)      | `false` |
| 4     | `1..5_u32`  | `\|x\| x == 100` | 4 (every element)    | `false` |

The empty case (Probe 3) is the dual of `reduce`'s empty case from
lesson 152: there, no first element forced an `Option<_>` wrapper;
here the empty iterator is just the unambiguous `false` answer, no
wrapper needed. Corpus prose at `trait.Iterator.md:2612`: "An empty
iterator returns `false`."

To watch short-circuit directly, count the calls (Probe 5):

```rust
fn main() {
    let mut count = 0_u32;
    let r = (1..10_u32).any(|x| { count += 1; x == 3 });
    println!("{} {}", r, count);
}
```

Output: `true 3`. The closure ran for `x = 1`, `x = 2`, `x = 3` and
stopped. The braced body `{ count += 1; x == 3 }` is a regular Rust
block: `count += 1;` is a statement, `x == 3` is the trailing
expression that becomes the return value. The closure captures
`count` (lesson 144) and mutates it (lesson 023), which is why the
bound is `FnMut` and not `Fn` (lesson 148).

## Mental Model Delta

- *Before:* "All four closure-driven Iterator methods I know consume
  `self`. Once I call them, the iterator is gone."
- *After:* "Some closure-driven Iterator methods take `&mut self`
  instead. They borrow the iterator mutably, advance it as far as
  needed to answer their question, and return — leaving the iterator
  binding still usable. `any` is the simplest one: it returns `bool`,
  short-circuits on the first `true` from the closure, and on the
  empty iterator returns `false`. The iterator's position after
  `any` is just past the first match (or fully exhausted, if no
  match)."

## Prerequisites

- Installed concepts (load-bearing):
  - **152** (`reduce`): sibling closure-driven Iterator consumer.
    Today contrasts the receiver (`&mut self` vs `self`) and the
    return (`bool` vs `Option<Self::Item>`).
  - **151**, **150**, **149**: closure-driven Iterator family.
    Today's bound drops to single-parameter form `FnMut(_) -> _`.
  - **148** (`Fn`/`FnMut`/`FnOnce`): bound is `FnMut`; Probe 5's
    closure mutates a captured binding.
  - **147** (parens-bound): `<F: FnMut(T) -> R>` segment grammar.
  - **144** (capture): Probe 5's closure captures `count`.
  - **142** (closure literal): `|x| body`.
  - **132** (`Self::Item`): for `Range<u32>`, `Self::Item = u32`.
  - **131** (`&mut self` on `next()`): the borrow rule reused for
    `any`. Probe 7's E0596 fires the same way as 131's contrast;
    Probe 2 calls `it.next()` directly after `.any(...)`.
  - **091**, **081**, **080**: `Range<u32>`; `_u32` suffix; `u32`.
  - **013** (`==` on integers produces `bool`).
  - **023** (`+=`): Probe 5's `count += 1`.
  - **011** (`println!`): formatted output.
  - **003** (rustc diagnostic map): E0277 (Probe 6), E0596 (Probe 7).
- Cited: 145 (`<F>`), 143 (`|x|` no-annotation), 005 (`let`),
  002 (`fn main`), 001 (`rustc + ./name`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is `true`.
Save Probe 2 as `reusable.rs`, compile, run; output is `true Some(6)`.
Then change Probe 2's `5` to `100` and predict the output before you
re-run.

(Answer: `false None`. The closure never returns `true`, so `any`
walks the whole iterator returning `false`; the iterator is now
exhausted, so `it.next()` returns `None`.)

## What Changed

- `any` is the fifth closure-driven Iterator method. Signature
  `fn any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool`.
- Receiver is `&mut self`, not `self` — the iterator binding is
  *still usable* after `.any(...)` returns. First place this is true
  in the closure-driven family.
- Closure bound is single-parameter (no comma) and returns `bool`.
- **Short-circuit**: `any` stops calling the closure as soon as the
  closure returns `true`. The iterator lands just past the matching
  element (Probe 2: `Some(6)` after matching on `5`).
- Empty iterator returns `false`; closure is not called.
- Without `let mut`, `it.any(...)` fires E0596 — same as 131's
  `next()` contrast.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let mut it = 1..6_u32;
    let r = it.any(|x| x == 3);
    let n1 = it.next();
    let n2 = it.next();
    let n3 = it.next();
    println!("{} {:?} {:?} {:?}", r, n1, n2, n3);
}
```

(a) What does `./q` print, and why?

(b) Change `|x| x == 3` to `|x| x == 100`. What does `./q` print?

(Answers: (a) `true Some(4) Some(5) None`. `any` short-circuits on
`x = 3`, leaving `4` next. The iterator yields `Some(4)`, `Some(5)`,
then exhausts to `None`. (b) `false None None None`. The closure
never returns `true`; `any` walks the whole iterator returning
`false` and leaving it exhausted.)

## What To Ignore For Now

- **`all`** — sibling with the same signature shape but inverted
  semantics ("do *all* elements match?"). Its own move.
- **`position`** — sibling with `FnMut(Self::Item) -> bool` bound
  but `Option<usize>` return. Its own move.
- **`find`** — predicate consumer whose closure parameter is
  `&Self::Item`. Pulls in deref-read; later move.
- **`find_map`** — `Option<B>`-returning closure; later move.
- **The design question "why `&mut self` and not `self`?"** —
  briefly: because predicate consumers often want to keep using the
  iterator (resume the search, ask another question). Full
  treatment fits with `position` or a later capstone.
- **`try_for_each`, `try_fold`** — short-circuit-with-`?` variants;
  gated on the `Try` trait sub-arc.
- **`v.iter()` / `v.into_iter()`** — different `Self::Item`.

## Evidence

See `../evidence/153-iterator-any.md`.
