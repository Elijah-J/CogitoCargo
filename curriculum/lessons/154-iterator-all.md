---
id: 154-iterator-all
status: accepted
evidence: ../evidence/154-iterator-all.md
---

# Test a range with a predicate using `iter.all(|x| ...)`

## The Move

`all` is the sibling of `any` from lesson 153: same signature, same
receiver, same closure bound, same return type. The only thing that
changes is the *polarity*. `any` asks "did at least one element
match?"; `all` asks "did every element match?"

```rust
fn main() {
    let r = (1..10_u32).all(|x| x < 100);
    println!("{}", r);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
true
```

Every element of `(1..10_u32)` is less than `100`, so every closure
call returns `true`, and `all` returns `true`.

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:2557`:

```text
fn all<F>(&mut self, f: F) -> bool
   where Self: Sized,
         F: FnMut(Self::Item) -> bool,
```

Same shape as `any`'s. Read each segment the same way (`<F>`,
`&mut self`, single-parameter `FnMut(Self::Item) -> bool`,
`-> bool`). Stabilized at 1.0.0; local toolchain 1.95.0.

## Inverted polarity

Two facts flip relative to `any`. Everything else is unchanged.

**Short-circuit on `false`, not `true`.** Watch the count directly:

```rust
fn main() {
    let mut count = 0_u32;
    let r = (1..10_u32).all(|x| { count += 1; x < 5 });
    println!("{} {}", r, count);
}
```

Output: `false 5`. The closure ran for `x = 1, 2, 3, 4, 5`. For the
first four, `x < 5` returned `true`; on the fifth call, `5 < 5`
returned `false`, and `all` stopped. Same closure shape as lesson
153 Probe 5; same composition (capture + `+=` + `FnMut`).

Corpus prose at `trait.Iterator.md:2566-2568`: "`all()` is
short-circuiting; in other words, it will stop processing as soon as
it finds a `false` ... ."

**Empty iterator returns `true`, not `false`.** This is the load-bearing
empty-case probe:

```rust
fn main() {
    let r = (1..1_u32).all(|x| x < 0);
    println!("{}", r);
}
```

`(1..1_u32)` yields no elements. The predicate `x < 0` is impossible
for any `u32` (rustc even warns: `comparison is useless due to type
limits`). Yet `./empty` prints `true`. The closure is *never called*;
the runtime answer comes from the empty-iterator rule. Corpus prose
at `trait.Iterator.md:2570`: "An empty iterator returns `true`."

Dual of lesson 153 Probe 3 (`(1..1_u32).any(|x| x == 5)` returned
`false`). `all` of zero things is trivially "all passed" because
there were none to fail; `any` of zero things is trivially "none
passed" because there were none to succeed.

## The iterator survives the call

Same as `any`. With `let mut it = ...`, `it.all(...)` leaves the
binding still usable, with the iterator's position landing just past
the failing element:

```rust
fn main() {
    let mut it = 1..10_u32;
    let r = it.all(|x| x < 5);
    let n = it.next();
    println!("{} {:?}", r, n);
}
```

Output: `false Some(6)`. Same structure as lesson 153 Probe 2 with
`any` rotated to `all`. Without `let mut` the borrow fails with E0596
— same diagnostic as lesson 153 Probe 7.

## Mental Model Delta

- *Before:* "`any` short-circuits on the first `true` from the closure
  and returns `false` for an empty iterator."
- *After:* "`any` and `all` are siblings with the same signature.
  `any` short-circuits on `true` and returns `false` for empty. `all`
  short-circuits on `false` and returns `true` for empty. Both leave
  the iterator usable, with its position landing just past the
  decisive element. The empty-case identities (`false` for `any`,
  `true` for `all`) match the existential-versus-universal rule:
  `any` of zero is `false`; `all` of zero is `true`."

## Prerequisites

- Installed concepts (load-bearing):
  - **153** (`any`): direct sibling. Today reuses signature, receiver,
    bound, return, the iterator-still-usable rule, and the E0277/E0596
    diagnostic shapes. Today inverts only the short-circuit polarity
    and the empty-case answer.
  - **148** (`FnMut`): the count probe's closure mutates a captured
    binding; bound `FnMut` accepts it.
  - **147** parens-bound, **144** capture, **142** closure literal,
    **132** `Self::Item`, **131** `&mut self` + `let mut`, **091** /
    **081** / **080** `Range<u32>`, **023** `+=`, **013** `<` on
    integers produces `bool`, **011** `println!`, **003** diagnostic.
- Cited: 152 (`reduce`, empty-case-as-design-choice contrast),
  145 (`<F>`), 005 (`let`), 002 (`fn main`), 001 (`rustc`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is `true`.
Save Probe 5 (`reusable.rs`) and run it; predict the output before
you do. The closure is `|x| x < 5`. With `(1..10_u32)`, `all` will
short-circuit on the first `x` where `x < 5` returns `false`. Which
`x`? What does `it.next()` return after that?

(Answer: `false Some(6)`. `5 < 5` is `false`, so `all` stops at `5`
and returns `false`; the iterator's next yielded value is `Some(6)`.)

## What Changed

- `all` is a sibling of `any` with the same signature shape, the
  same receiver `&mut self`, the same closure bound, and the same
  return type `bool`.
- `all` short-circuits on the first `false` return from the closure
  (not the first `true`).
- `all` returns `true` for an empty iterator (not `false`). The
  closure is not called.
- After `all` short-circuits, the iterator's position lands just
  past the failing element — same shape as `any` after a match.
- E0277 and E0596 contrast probes are identical in shape to lesson
  153's, with the method name rotated.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let mut count = 0_u32;
    let r = (1..10_u32).all(|x| { count += 1; x > 100 });
    println!("{} {}", r, count);
}
```

What does `./q` print, and why?

(Answer: `false 1`. The closure runs once on `x = 1`; `1 > 100` is
`false`, so `all` stops immediately and returns `false`. `count` is
`1`.)

## What To Ignore For Now

- **`position`** — sibling with `FnMut(Self::Item) -> bool` bound
  but `Option<usize>` return. Lesson 155 candidate.
- **`find`** — predicate consumer whose closure parameter is
  `&Self::Item`. Pulls in deref-read; later move.
- **`find_map`** — `Option<B>`-returning closure; later move.
- **The De Morgan duality** between `any` and `all` at the level of
  closure-body inversion (`!cond`). Named here, formal treatment
  deferred.
- **The design rule "why have both `any` and `all`?"** — both exist
  because predicate-direction (existential vs universal) is a real
  modeling choice; deferred.
- **`try_for_each`, `try_fold`** — short-circuit-with-`?` variants;
  gated on the `Try` trait sub-arc.

## Evidence

See `../evidence/154-iterator-all.md`.
