---
id: 152-iterator-reduce
status: accepted
evidence: ../evidence/152-iterator-reduce.md
---

# Reduce a range to one value with `iter.reduce(|acc, x| ...)`

## The Move

Lesson 151 installed `fold` — explicit `init` plus a two-parameter
closure. `reduce` is the close sibling: same threading, but with **no
`init` argument**. Instead, `reduce` takes the iterator's *first
element* as the initial accumulator. Two structural facts change at
once: the argument list shrinks from two to one (just the closure),
and the return type wraps in `Option` — when the iterator is empty
there is no first element to start from.

```rust
fn main() {
    let s = (1..4_u32).reduce(|acc, x| acc + x);
    println!("{:?}", s);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
Some(6)
```

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:2469`:

```text
fn reduce<F>(self, f: F) -> Option<Self::Item>
   where Self: Sized,
         F: FnMut(Self::Item, Self::Item) -> Self::Item,
```

Compare with `fold`'s signature segment by segment:

- `<F>` — *one* type parameter, where `fold` had `<B, F>`. No
  accumulator-type slot today; the accumulator's type *is* `Self::Item`.
- `(self, f: F)` — consuming `self`, then *one* non-receiver argument:
  the closure. No `init: B`. Probe 6 in the appendix witnesses this
  directly: passing an extra argument fires `error[E0061]: this method
  takes 1 argument but 2 arguments were supplied`.
- `-> Option<Self::Item>` — the return type is *not* a bare value
  (`fold`'s `B`) and *not* `()` (`for_each`'s). It is `Option<T>`
  (lesson 119) where `T = Self::Item`.
- `F: FnMut(Self::Item, Self::Item) -> Self::Item` — same parens-bound
  shape (147), same `FnMut` (148), same two-parameter list inside the
  parens (151). What differs: every position is the *same* type,
  `Self::Item`. No separate `B`. The closure takes two elements and
  returns one element of the same type.

For `(1..4_u32)`, `Self::Item = u32`. Probe 5 empirically witnesses by
passing `7` instead of a closure: rustc spells the expected closure
type as `FnMut(u32, u32)` — both slots are `u32`, not `(B, u32)` like
fold's diagnostic.

## How `reduce` threads — and why `Option`

Walk Probe 1: three elements, but the closure runs only *twice*.

| step | acc (in)  | x  | acc (out) |
| ---- | --------- | -- | --------- |
| 0    | (none)    | 1  | 1 (becomes initial acc — *no closure call*) |
| 1    | 1         | 2  | 1 + 2 = 3 |
| 2    | 3         | 3  | 3 + 3 = 6 |

The first element is *taken as* the initial accumulator, not passed
through the closure. After the last element the final accumulator is
`6`, wrapped in `Some(6)`. Corpus prose at `trait.Iterator.md:2477-2480`:
"For iterators with at least one element, this is the same as `fold()`
with the first element of the iterator as the initial accumulator
value, folding every subsequent element into it."

The empty case is the load-bearing reason for the `Option`. Probe 2
runs the same closure on `(1..1_u32)` — a half-open range whose lower
bound equals its upper bound, yielding nothing. Output: `None`.
Corpus at `trait.Iterator.md:2474-2475`: "If the iterator is empty,
returns `None`; otherwise, returns the result of the reduction."
`fold`'s `init` always exists; `reduce` cannot make a value out of
nothing, so it returns `Option<Self::Item>` to flag the empty case.

Probe 3 sharpens the other boundary: `(5..6_u32)` yields exactly one
element. `reduce` takes it as the initial accumulator, has nothing
remaining, and returns `Some(5)` — the closure runs *zero* times.

## Mental Model Delta

- *Before:* "Closure-driven Iterator consumers I know: `for_each`
  (returns `()`) and `fold` (takes `init` plus a closure, returns
  whatever the closure returns)."
- *After:* "`reduce` is `fold` without an `init`. It uses the first
  element as the initial accumulator and threads the rest the same
  way. Because an empty iterator has no first element, `reduce`
  returns `Option<Self::Item>` — `Some(final_acc)` for non-empty,
  `None` for empty. The bound has no separate accumulator-type slot:
  both parameters and the return are the same type, `Self::Item`."

## Prerequisites

- Installed concepts (load-bearing):
  - **151** (`fold`): threading semantics — each closure call's return
    becomes the next call's first argument. Today reuses that rule
    unchanged; what differs is *only* where the first accumulator
    value comes from.
  - **148**, **147**: `FnMut` choice and the parens bound. Today drops
    the `-> B` arrow and the `<B, ...>` type parameter.
  - **142** + **036**: closure literal and the comma-separated
    parameter list. Today's `|acc, x|` is the same shape as 151's.
  - **132**: `Self::Item` slot, today filling *all three* positions
    inside the bound.
  - **119**: `Option<T>` with `Some(T)` / `None`. Today is the first
    time `Option` appears as the *return type* of a method that
    aggregates rather than steps.
  - **131** + **093**: `{:?}` Debug-printing of `Option<u32>`. Today
    prints `Some(6)` and `None`.
  - **091** + **081** + **080**: `Range<u32>` source; `Self::Item = u32`.
  - **009**: `+` on integers. **003**: rustc diagnostic map.
- Cited: 145 (`<F>` — today's only generic, no `B`), 143 (`|x|` no
  annotation), 144 (capture — today's closure captures nothing),
  150 / 149 (closure-driven Iterator method family), 011, 005, 002,
  001.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`; output is `Some(6)`. Save the
empty-iterator probe (`1..1_u32` instead of `1..4_u32`); output is
`None`. Same closure, same method, same shape — only the source
iterator changed, and the return value crossed the `Some` / `None`
boundary.

## What Changed

- `reduce` is the fourth closure-driven Iterator method. Signature
  `fn reduce<F>(self, f: F) -> Option<Self::Item> where Self: Sized,
  F: FnMut(Self::Item, Self::Item) -> Self::Item`. One non-receiver
  argument; no `init`.
- The bound has the same type, `Self::Item`, in both parameter slots
  and the return slot — no separate accumulator type.
- The first element becomes the initial accumulator; the closure runs
  once per *remaining* element. Three elements → two closure calls.
- Return is `Option<Self::Item>`: `Some(final_acc)` for non-empty,
  `None` for empty.
- Passing two arguments (as if `reduce` were `fold`) fires E0061 "this
  method takes 1 argument but 2 arguments were supplied" (Probe 6) —
  direct structural witness that the argument list has length one.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let p = (1..4_u32).reduce(|acc, x| acc * x);
    println!("{:?}", p);
}
```

(a) Does it compile silently, and what does `./q` print?

(b) Change the source to `(1..1_u32)`. What does `./q` print, and why?

(c) Change the source to `(7..8_u32)` (one element). What does it
print, and how many times is the closure called?

(Answers: (a) Yes; `Some(6)`. First element `1` becomes acc; closure
walks `(1, 2) → 2`, `(2, 3) → 6`. (b) `None`. Empty range; no first
element. (c) `Some(7)`. The single element becomes acc; closure runs
zero times.)

## What To Ignore For Now

- **`fold` vs `reduce` choice rule** — when to pick one over the
  other in real code. Today contrasts the *signature*; the design
  question ("does the operation have a natural identity?") is its own.
- **`try_reduce`** — nightly-only (`trait.Iterator.md:2495`).
- **`Option::unwrap`, `Option::unwrap_or`, `Option::map`, `?`** — ways
  to get the inner value out. Each is a separate `Option`-API move.
  The corpus example (`trait.Iterator.md:2485`) uses `.unwrap_or(0)`;
  today keeps the wrapper visible with `{:?}` instead.
- **Numerical edge cases** for `reduce` on signed integers — beyond
  scope.
- **`v.iter()` / `v.into_iter()` as the source** — composes a
  different `Self::Item` resolution.

## Evidence

See `../evidence/152-iterator-reduce.md`.
