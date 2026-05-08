---
id: 139-iterator-fuse
status: draft
evidence: ../evidence/139-iterator-fuse.md
---

# Make `None` sticky on any iterator with `iter.fuse()`

## The Move

Lessons 136-138 installed three lazy adapters. Today's `fuse()` is the
next sibling — same `(self) -> Wrapper<Self>` shape as `enumerate`
(no second parameter), but the new fact is *behavioral*. `.fuse()`
does not change the element count or type. It changes what happens
*after* the inner iterator returns its first `None`.

The default `Iterator::next` contract permits an iterator to *resume*
after returning `None`: per
`output/docs/rust/std/iter/trait.Iterator.md:281-284`, "Individual
iterator implementations may choose to resume iteration, and so
calling `next()` again may or may not eventually start returning
`Some(Item)` again." Wrapping with `.fuse()` *enforces* the sticky-
`None` rule: once the inner iterator returns `None` for the first
time, every subsequent call on the wrapper returns `None`.

A slice iterator (lesson 131) already sticks at `None`, so to witness
the change, the inner iterator must actually be one that resumes.
Build a tiny custom iterator (lesson 132's pattern) that alternates
`Some, None, Some, None, …`. Save as `demo.rs`:

```rust
// Resumes after None: yields Some, None, Some, None, Some, None, ...
struct Stutter { n: u32 }

impl Iterator for Stutter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n % 2 == 0 {
            let v = self.n / 2;
            self.n += 1;
            Some(v)
        } else {
            self.n += 1;
            None
        }
    }
}

fn main() {
    let mut a = Stutter { n: 0 };
    for _ in 0..6 { println!("{:?}", a.next()); }

    println!("---");

    let mut b = Stutter { n: 0 }.fuse();
    for _ in 0..6 { println!("{:?}", b.next()); }
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
Some(0)
None
Some(1)
None
Some(2)
None
---
Some(0)
None
None
None
None
None
```

Same six `next()` calls on each side. Without `.fuse()`, six calls on
`Stutter` produce three `Some(_)` and three `None`. With `.fuse()`,
the second call's `None` "latches" — calls 3 through 6 all return
`None`. The `---` divider separates the two halves.

The trait declaration at `trait.Iterator.md:1737` spells
`fn fuse(self) -> Fuse<Self> where Self: Sized,`. Three slots: bare
`self` receiver (lesson-102 consuming, same as 136-138, not re-
witnessed), no second parameter (same as 138), return type
`Fuse<Self>` — a wrapper struct documented at
`output/docs/rust/std/iter/struct.Fuse.md:7`
(`pub struct Fuse<I> { /* private fields */ }`) which itself
implements `Iterator`. The appendix's forced-error type-pin (`let _x:
u32 = v.iter().fuse();`) makes rustc spell the type `Fuse<Iter<'_,
u64>>`.

A subtle distinction lesson 131 left deferred:

- **`FusedIterator`** (`trait.FusedIterator.md`) is a *marker trait*
  an iterator can implement to declare "I already stick at `None`."
  Slice iterators implement it.
- **`Fuse<I>`** (`struct.Fuse.md`) is the *adapter struct* that
  forces stickiness on whatever inner iterator it wraps.
- **`Iterator::fuse`** is the method that builds a `Fuse<I>` from any
  iterator.

If the inner iterator already implements `FusedIterator`, `.fuse()`
is a no-op (`trait.FusedIterator.md:18-21`: "the additional `Fuse`
wrapper will be a no-op with no performance penalty"). The appendix
witnesses this on `v.iter()`.

## Mental Model Delta

- *Before:* "Adapters change *what* the wrapper yields. Once an
  iterator returns `None`, it stays `None` — that is what 131's
  slice iterator did."
- *After:* "The Iterator contract does *not* require sticky-`None`
  — it explicitly licenses resuming. `FusedIterator` is the marker
  trait declaring stickiness; `.fuse()` is the wrapper that
  enforces it on iterators that lack the marker. Same `(self) ->
  Wrapper<Self>` shape as `enumerate`, but the semantic is *post-
  `None` behavior*, not element transformation."

## Prerequisites

- Installed concepts:
  - **Lesson 138** (load-bearing): the `(self) -> Wrapper<Self>`
    adapter shape with no second parameter. Today reuses it.
  - **Lessons 137, 136** (load-bearing): the lazy-adapter family —
    consuming `self`, wrapper that itself implements `Iterator`.
  - **Lesson 132** (load-bearing): user-defined-`Iterator` pattern.
    `Stutter` reuses 132's `Counter`-style template — `type Item =
    u32;` plus a hand-written `fn next` returning `Some`/`None`
    based on internal state. The 75 provided methods, including
    `fuse`, are inherited automatically.
  - **Lesson 131** (load-bearing): slice iterator's sticky-`None`
    behavior. Today extends: that stickiness was *not* the bare
    `Iterator` contract — it came from the slice iterator
    implementing `FusedIterator`.
  - **Lessons 102, 119, 080, 037, 023** (cited): consuming-`self`;
    `Option<T>`; `u32`; `%` remainder; `+=` compound-assignment.
  - **Lessons 116, 049, 040, 011, 005, 003, 002, 001** (cited):
    default-body trait methods; method chaining; dot-call;
    `println!`; `let`; diagnostic map; `fn main`; rustc compile +
    run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the
thirteen lines above. Now experiment: change both `for _ in 0..6` to
`for _ in 0..10`. The first (without-`fuse`) half continues
alternating `Some(_), None, Some(_), None, …` for ten lines (the
inner counter keeps advancing). The second half prints `Some(0),
None` followed by *eight* more `None` lines — the latch holds.

Then check the no-op claim on a slice iterator. Save as `slice.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut b = v.iter().fuse();
    for _ in 0..5 { println!("{:?}", b.next()); }
}
```

Output: `Some(10)`, `Some(20)`, `Some(30)`, `None`, `None` — same
five lines `v.iter()` alone would print. The slice iterator already
implements `FusedIterator`; wrapping it adds nothing. The appendix's
Probe 2 captures both halves side by side.

## What Changed

- Signature `fn fuse(self) -> Fuse<Self> where Self: Sized,`
  (`trait.Iterator.md:1737`). Same shape as `enumerate`; wrapper
  struct at `struct.Fuse.md:7`.
- The `Iterator` contract permits resuming after `None`
  (`:281-284`). `Stutter` is a working example.
- `.fuse()` enforces sticky-`None`: after the first `None`, the
  wrapper returns `None` on every subsequent call.
- `FusedIterator` is a separate marker trait declaring that an
  iterator already sticks. Slice iterators implement it; `.fuse()`
  on them is a no-op.
- `Fuse<Self>` is itself an iterator. The forced-error type-pin
  names it `Fuse<Iter<'_, u64>>`.

## Check Yourself

```rust
struct Pulse { n: u32 }

impl Iterator for Pulse {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.n % 3 == 0 { Some(self.n) } else { None };
        self.n += 1;
        r
    }
}

fn main() {
    let mut p = Pulse { n: 0 };
    for _ in 0..6 { println!("{:?}", p.next()); }
    println!("---");
    let mut q = Pulse { n: 0 }.fuse();
    for _ in 0..6 { println!("{:?}", q.next()); }
}
```

(a) Does it compile silently? What six lines does the first half
print? (b) Where does the latch fire in the second half, and what do
the remaining lines print?

*(Answers: (a) Yes. The first half prints `Some(0)`, `None`, `None`,
`Some(3)`, `None`, `None` — `Pulse` yields `Some(n)` only when
`n % 3 == 0`. (b) The latch fires on the second `next()` call
(the first `None`). The second half prints `Some(0)` then five
`None` lines — the wrapper never asks `Pulse.next()` again after
its first `None`.)*

## What To Ignore For Now

Deferred: the `Fuse<I>` struct's private fields and its `next` body
(latch-flag bookkeeping); the `Self: Sized` bound (still); the
implementor caveat at `:1745-1747` (`fuse` "may behave incorrectly"
if `FusedIterator` is improperly implemented); the `FusedIterator`
implementor list at `trait.FusedIterator.md:23+`; the std-doc
advice at `:18-21` "you should not use `FusedIterator` in generic
bounds" (gates a future generic-bound lesson). Next moves per audit
§5: `step_by` (step 10), `size_hint` (step 11).

## Evidence

See `../evidence/139-iterator-fuse.md`.
