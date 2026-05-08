---
id: 125-iter-zip
status: accepted
evidence: ../evidence/125-iter-zip.md
---

# Walk two `Vec<T>` values in lockstep with `v.iter().zip(w.iter())`

## The Move

Lesson 123 installed `v.iter()` for walking one `Vec<T>`. Today
chains `.zip(...)` on that iterator to walk *two* vecs side by side,
yielding pairs.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for pair in v.iter().zip(w.iter()) {
        println!("{} / {}", pair.0, pair.1);
    }
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints three lines:

```text
10 / 100
20 / 200
30 / 300
```

Read the chain left-to-right (lesson 049). `v.iter()` is the
lesson-040 dot-call shape and returns an iterator (lesson 123).
`.zip(w.iter())` is *that same dot-call shape applied to the
iterator*: receiver, dot, `zip`, and one argument — itself another
iterator, produced by `.iter()` on `w`.

The result is a new iterator. Each value it yields is a *tuple*
(lesson 072) of two elements: one from each source per pass. Here
the three yielded tuples are `(&10, &100)`, `(&20, &200)`,
`(&30, &300)`. The `for`-loop binds each to `pair`; `pair.0` and
`pair.1` read the parts via lesson 072's indexing. Both have type
`&u64` (lesson 123's "yields `&T`" rule, preserved per-element).

The std `Iterator::zip` page frames it verbatim: "Zips up two
iterators into a single iterator of pairs ... If either iterator
returns `None`, `next` from the zipped iterator will return `None`."
That last sentence is the *shortest-source rule*: when the sources
differ in length, iteration stops as soon as either runs out. A
probe in the appendix witnesses this — `vec![10, 20, 30]` zipped
against `vec![100]` yields one pair and the loop ends.

## Mental Model Delta

- *Before:* "I can walk one `Vec<T>` with `v.iter()` (lesson 123) and
  I can chain *empty-argument* iterator methods like `.rev()` on it
  (lesson 124). To walk two vecs together I would write an index
  loop and read `v[i]` and `w[i]` separately."
- *After:* "`.zip(...)` is another iterator method, but it takes one
  argument: a *second* iterator. The chain `v.iter().zip(w.iter())`
  produces an iterator whose yielded value is a 2-tuple — the next
  pair from the two sources. I read the elements with the lesson-072
  indexing `pair.0` and `pair.1`. Iteration stops when *either*
  source runs out. The trait machinery that lets `.zip()` accept any
  iterator-like argument is named-deferred."

## Prerequisites

- Installed concepts:
  - **Lesson 123** (load-bearing): `v.iter()` returns an iterator
    yielding `&T`. Today calls `.iter()` twice — once on each vec —
    and `.zip(...)` consumes both.
  - **Lesson 072** (load-bearing): the tuple type `(A, B)` and field
    access by `.0` / `.1`. Today's `pair` is a tuple value.
  - **Lesson 049** (load-bearing): chained dot-calls
    `expr.method1().method2(arg)`. `v.iter().zip(w.iter())` is that
    shape — empty argument list at the first call, one argument at
    the second.
  - **Lesson 124** (cited): structurally parallel — both `.rev()`
    and `.zip()` are iterator methods chained after `.iter()`. The
    centered contrast (calling on `Vec<T>` directly) reuses 124's
    diagnostic shape with the method name swapped.
  - **Lesson 040** (cited): dot-call grammar; reused at every call.
  - **Lesson 107** (cited): `Vec<T>` construction with `vec![]`.
  - **Lesson 079** (cited): `for X in COLLECTION { ... }`; today's
    COLLECTION is the chained iterator returned by `.zip(...)`.
  - **Lessons 011, 001, 002, 003, 005** (cited): `println!("{}", x)`;
    `rustc demo.rs && ./demo` silent on success; `fn main` entry;
    the diagnostic four-part map; `let`.
  - **Lessons 080, 019** (cited): `u64`; the `: TYPE` annotation slot.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
with the source above. Compile and run:

```console
$ rustc demo.rs
$ ./demo
10 / 100
20 / 200
30 / 300
```

Now the contrast. `.zip()` is a method on the *iterator*, not on the
vec. Save `broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    let z = v.zip(w);
    println!("{:?}", z);
}
```

Compile:

```text
error[E0599]: no method named `zip` found for struct `Vec<u64>` in the current scope
 --> broken.rs:4:15
  |
4 |     let z = v.zip(w);
  |               ^^^ `Vec<u64>` is not an iterator
  |
help: call `.into_iter()` first
  |
4 |     let z = v.into_iter().zip(w);
  |               ++++++++++++
```

Read with the lesson 003 map. Headline `error[E0599]: no method
named \`zip\` found for struct \`Vec<u64>\`` — lesson 100's missing-
method shape. The inline label states today's structural fact
directly: "`Vec<u64>` is not an iterator." `.zip()` lives on
iterators, so you must produce one first — `v.iter()` today. The
same pattern as lesson 124's `v.rev()` contrast, with the method
name changed.

## What Changed

- `.zip(other)` is an iterator method that takes another iterator
  as argument and returns a new iterator yielding 2-tuples — one
  element from each source per pass.
- `v.iter().zip(w.iter())` is the chained-dot-call shape from
  lesson 049: empty argument list at the first call, one argument
  at the second.
- Each yielded value is a tuple — here `(&u64, &u64)`. Read the
  parts with `pair.0` and `pair.1` (lesson 072).
- When the sources differ in length, iteration stops at the
  shorter one. No panic, no error — the loop just ends.
- `.zip()` is *not* a method on `Vec<T>` itself. Calling `v.zip(w)`
  fires E0599 with the inline label "`Vec<u64>` is not an iterator."
- This makes the *third link* of rmp's `src/biguint/cmp.rs:22` chain
  `self.limbs.iter().rev().zip(other.limbs.iter().rev())` readable.
  The `for (left, right) in ...` destructuring on the zipped output
  is a separate future move.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![7, 8, 9];
    let ys: Vec<u64> = vec![1, 2];
    for pair in xs.iter().zip(ys.iter()) {
        println!("{} / {}", pair.0, pair.1);
    }
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile silently? How many lines does it print, and
what are they?

(b) You replace line 4 with `for pair in xs.zip(ys) {`. What error
code fires, and what does the inline label say?

*(Answers: (a) Yes; two lines: `7 / 1`, `8 / 2`. The shorter source
`ys` has length 2, so iteration stops after two pairs and `9` is
never visited. (b) E0599; the inline label is "`Vec<u64>` is not an
iterator." `.zip()` is a method on iterators, not on `Vec<T>`.)*

## What To Ignore For Now

Today installs only `.zip(...)` chained on `v.iter()`, with the
yielded tuple read by `pair.0` / `pair.1`. Deferred:

- *For-pattern destructuring* — `for (left, right) in iter` binds
  the two tuple parts directly in the loop slot. rmp's `cmp.rs:22`
  uses exactly this form. It is its own move.
- *The `Iterator` trait* and *the `IntoIterator` parameter rule* —
  what makes `.zip()` callable on any iterator and what lets it
  accept arrays, ranges, and other iterator-likes as argument.
  Named-deferred from 123.
- *The `Zip<A, B>` adapter struct* — the type returned by
  `.zip(...)`. Surfaced in the std signature only; you never
  write the name today.
- *Other iterator adapters* — `.map()`, `.filter()`, `.fold()`,
  `.collect()`, `.enumerate()`, etc. Each is its own move.
- *`.zip(...)` chained after `.rev()` or other adapters* — the rmp
  source uses both; the longer chain composes 123 + 124 + 125
  step by step.
- *Auto-deref on `&u64`* — Probe 6 in the appendix uses
  `pair.0 + pair.1` (each `&u64`) and it compiles. The rule is
  std machinery; today does not center it.

## Evidence

See `../evidence/125-iter-zip.md`.
