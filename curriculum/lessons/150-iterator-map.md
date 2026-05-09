---
id: 150-iterator-map
status: accepted
evidence: ../evidence/150-iterator-map.md
---

# Rewrite each element with `iter.map(closure)`

## The Move

Lesson 149 installed `for_each` — the simplest closure-driven Iterator
method. It consumes `self`, takes one `FnMut(Self::Item)` closure, and
returns `()`. Today's `map` is the next step. It also takes one
closure, but two structural facts change at once: the closure has a
*return type*, and `map` itself returns *another iterator* — a wrapper
struct — instead of doing work right away.

```rust
fn main() {
    (1..4_u32).map(|x| x * 10).for_each(|y| println!("{}", y));
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
10
20
30
```

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:852`:

```text
fn map<B, F>(self, f: F) -> Map<Self, F>
   where Self: Sized, F: FnMut(Self::Item) -> B,
```

Read it segment by segment with the lesson 147 grammar:

- `<B, F>` — *two* type parameters. `F` is for the closure (same as
  `for_each`). `B` is new: the closure's return type.
- `(self, f: F)` — receiver `self`, consuming (Probe 7 witnesses E0382).
- `-> Map<Self, F>` — `map` returns a *struct* called `Map`,
  parameterized by the source iterator's type and the closure's
  anonymous type. Same wrapper-struct family lessons 136-140 installed
  for `Take<Self>` etc.; new today is the second type parameter.
- `F: FnMut(Self::Item) -> B` — the parenthesized bound (lesson 147)
  with `FnMut` (148) and `Self::Item` (132). The `-> B` segment is new:
  lesson 149's `for_each` had `FnMut(Self::Item)` with no arrow.
  Today's closure has both a parameter type (`Self::Item`) and a return
  type (whatever you write); rustc resolves `B` to the latter.

For `(1..4_u32).map(|x| x * 10)`: the source is a `Range<u32>` (lesson
091 + 081), so `Self::Item = u32`. The closure body `x * 10` is plain
`u32 * u32` (lesson 009 `*` on integers), producing a `u32`. So
`B = u32`. Probe 5 in the appendix verifies this verbatim — it tries
`let r: u32 = (1..4_u32).map(|x| x * 10);` and the resulting E0308
diagnostic says `found struct \`Map<std::ops::Range<u32>,
{closure@...}>\``.

The wrapper itself implements `Iterator`. From `struct.Map.md:150-154`
verbatim: `impl<B, I, F> Iterator for Map<I, F> where I: Iterator,
F: FnMut(<I as Iterator>::Item) -> B,` followed by `type Item = B`.
Read in the lesson 132 grammar: `Map<I, F>` *is* an `Iterator`, and
its `Item` is `B`. That is why chaining `.for_each(|y| ...)` onto
`.map(...)` works — the wrapper *is* an iterator, and `y` is
`B = u32`.

## `map` is lazy

Calling `.map(f)` does *not* call `f` on any element. It builds a
`Map` value that *will* call `f` once per element when something
iterates the wrapper. Probe 2 witnesses this: a closure body
containing `println!("called: {}", x)` is bound but never consumed,
and prints nothing — no `called: ...` lines, just `end`. Probe 3
puts the same closure body inside a `.for_each(...)` chain; now the
closure runs three times, interleaved with `for_each`'s body, because
the consumer pulls elements one at a time.

If you write `(1..4_u32).map(|x| x * 10);` as a bare statement (no
`let`, no consumer), rustc itself surfaces the lazy framing — Probe 4
captures the warning verbatim:

```text
warning: unused `Map` that must be used
  = note: iterators are lazy and do nothing unless consumed
```

This is the first `must_use` warning the run has surfaced.

## Mental Model Delta

- *Before:* "`for_each` takes a closure and runs it once per element.
  Iterator methods that take closures look like `FnMut(Self::Item)`."
- *After:* "`map` also takes one closure, but two things differ: the
  closure has a *return type* (`-> B`), and `map` itself returns
  *another iterator* — a `Map<Self, F>` wrapper struct that yields the
  closure's return value once per source element. The wrapper itself
  implements `Iterator` (with `Item = B`), so I can chain another
  iterator method onto it. And `map` is *lazy*: the closure is not
  called when I write `.map(...)` — only when something iterates the
  wrapper."

## Prerequisites

- Installed concepts (load-bearing):
  - **149**: `for_each` as a closure-driven Iterator method, used today
    as the consumer that iterates the `Map` wrapper.
  - **148**: `FnMut` and the auto-impl rule. Today's bound is
    `FnMut(Self::Item) -> B`.
  - **147**: parenthesized `<F: FnMut(T) -> R>` bound. Today is the
    *with-return-segment* form (lesson 149 had no `-> R`).
  - **138**: enumerate frame — yielded element shape can be rewritten
    by an adapter (then: wrap in a tuple; today: pick any `B`).
  - **136**: `Take<Self>` wrapper-struct frame. Today's wrapper is the
    same family with two type parameters instead of one.
  - **132**: `Self::Item` slot.
  - **091**: `Range<A>: Iterator` for `A: Step`; parens-rule.
  - **081 + 080**: `4_u32` suffix pins `Range<u32>`.
  - **009**: `*` on integers. Today's `x * 10` is `u32 * u32`.
  - **003**: rustc diagnostic map.
- Cited: 145 (`<F>` slot, today extended to `<B, F>`), 142 (closure
  literal), 143 (`|x|` no annotation), 144 (capture mechanic — today's
  closures capture nothing, named for completeness), 011 (`println!`),
  005 (`let _m = ...`), 002 (`fn main`), 001 (`rustc + ./name`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; the output is `10`,
`20`, `30`. Then run Probes 2 and 3 in the appendix back-to-back —
same closure body, but Probe 2 binds the wrapper to `_m` and never
consumes it (prints just `end`), while Probe 3 chains `.for_each(...)`
(prints six interleaved lines plus `end`). Same closure, two different
programs.

## What Changed

- `map` is the second closure-driven Iterator method. Signature
  `fn map<B, F>(self, f: F) -> Map<Self, F> where Self: Sized,
  F: FnMut(Self::Item) -> B`. Two type parameters (`B`, `F`); the bound
  has a `-> B` return-type segment.
- `map` returns a `Map<Self, F>` wrapper struct, not a number or
  `()`. The wrapper itself implements `Iterator` with `Item = B`, so
  iterator methods can be chained onto it.
- `map` is *lazy*. Building the wrapper does not call the closure on
  any element. The closure runs once per element only when something
  iterates the wrapper.
- Bare expression-statement form `(1..4_u32).map(|x| x * 10);`
  triggers `warning: unused \`Map\` that must be used` with
  `note: iterators are lazy and do nothing unless consumed`.
- The yielded element type is whatever the closure returns. Today's
  closure returns `u32`, so the wrapper's `Item` is still `u32` — a
  coincidence of body choice, not a property of `map`.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    (1..5_u32).map(|n| n + 100).for_each(|y| println!("{}", y));
}
```

(a) What four lines does `./q` print?

(b) You change the bare expression-statement form
`(1..5_u32).map(|n| n + 100);` (no `for_each`). Compile it. What
warning's `note:` text appears, and what does the program print at
runtime?

(Answers: (a) `101`, `102`, `103`, `104` — `1..5_u32` yields
`1, 2, 3, 4` and the closure adds `100` to each. (b) The warning is
`warning: unused \`Map\` that must be used` with
`= note: iterators are lazy and do nothing unless consumed`. The
program runs and prints nothing — the closure is never called because
the wrapper is never iterated.)

## What To Ignore For Now

- **`Map<I, F>` internal fields and other trait impls** —
  `DoubleEndedIterator`, `ExactSizeIterator`, etc. Implementor-side.
- **`map` with a closure that returns a different type** (e.g. `u32` to
  `String`). The mechanic is the same; today's body keeps `B = u32`
  for surface minimality.
- **`map_while`, `filter_map`, `flat_map`** — separate methods. Each
  is its own move.
- **Real iterator pipelines** — `(1..N).filter(...).map(...).collect::<Vec<_>>()`.
  Today is *one* lazy adapter chained into *one* consumer; pipelines
  as a centered concept come later.
- **`v.iter()` / `v.into_iter()` as the source** — each composes a
  different `Self::Item` resolution. `Range<u32>` is the leanest.

## Evidence

See `../evidence/150-iterator-map.md`.
