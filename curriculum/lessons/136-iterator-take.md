---
id: 136-iterator-take
status: accepted
evidence: ../evidence/136-iterator-take.md
---

# Limit a slice iterator to its first `n` elements with `iter.take(n)`

## The Move

Lessons 133, 134, and 135 each called a *consumer* — `count`, `last`,
`nth` — methods whose return is a number, an `Option`, or both.
Today's `take` is structurally different. Its return is *another
iterator*. Calling `.take(n)` on a slice iterator hands you a fresh
iterator that yields the first `n` elements of the original and then
stops. You typically chain a consumer onto it to actually do work:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    let n = v.iter().take(2).count();
    println!("{}", n);

    let last = v.iter().take(3).last();
    println!("{:?}", last);

    for x in v.iter().take(2) {
        println!("{}", x);
    }
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
2
Some(30)
10
20
```

The trait declaration spells the signature
`fn take(self, n: usize) -> Take<Self> where Self: Sized,`
(`output/docs/rust/std/iter/trait.Iterator.md:1376`). Three facts:

1. **Receiver `self`** — lesson-102 consuming shape. Same as `count`
   and `last`. Probe 5 captures the familiar E0382 + `note:` template
   with `take` in the method-name slot.
2. **`n: usize`** — same second-parameter shape lesson 135's `nth`
   installed.
3. **Return type `Take<Self>`** — *itself an iterator*. New fact
   today. Where `count` returned `usize` and `last` / `nth` returned
   `Option<Self::Item>`, `take` returns a value of type `Take<Self>`,
   a wrapper struct (`output/docs/rust/std/iter/struct.Take.md`:
   `pub struct Take<I> { /* private fields */ }`). It implements
   `Iterator`, so `.next()`, `.count()`, `.last()`, and `for` all
   work on it. Today refers to the struct opaquely.

A *consumer* like `count` returned a number. An *adapter* like `take`
returns a new iterator value. The corpus uses "iterator adapter" at
`trait.Iterator.md:1443` and frames adapters as *lazy* at `:867,892`
("it won't even execute, as it is lazy"). Today installs that frame
for `take` directly.

## Mental Model Delta

- *Before:* "Every Iterator method I have called returns a primitive
  or an `Option`. Calling the method does the work."
- *After:* "Some Iterator methods return *another iterator*. They are
  called *adapters*. `take(n)` is the smallest example: it produces
  a `Take<Self>` value that, when iterated, yields the first `n`
  elements of the inner iterator. Building the `Take` value does *no
  work*. The work happens only when something pulls — `.next()`,
  `.count()`, `.last()`, or a `for` loop — on the `Take`. Adapters
  compose: `v.iter().take(2).count()` is one adapter chained into one
  consumer. `take(n)` past the end gives you the full iterator's
  elements, no panic."

## Prerequisites

- Installed concepts:
  - **Lesson 135** (load-bearing): the `n: usize` second-parameter
    shape on a provided Iterator method. Today reuses that slot and
    returns to lesson 102's consuming-`self` receiver.
  - **Lessons 134, 133** (load-bearing): `.last()` and `.count()` are
    the consumers chained on `take(3).last()` and `take(2).count()`.
    Probe 5's E0382 reuses lesson 133's template verbatim with `take`
    substituted into the method-name slot.
  - **Lesson 132** (load-bearing): the `Iterator` trait declaration
    with 75 provided methods. `take` is one of them.
  - **Lesson 131** (load-bearing): `.next()` on a slice iterator.
    Probe 4 calls `.next()` on the `Take<Self>` value to show the
    binding is itself an iterator.
  - **Lesson 102** (load-bearing): `self` is the consuming receiver
    (Probe 5's E0382 + `note:` template).
  - **Lesson 049** (load-bearing): method chaining. Today's chain is
    the first in the run that puts a consumer after a non-consumer
    iterator method.
  - **Lessons 080, 123, 022, 116, 040, 011, 005, 003, 002, 001**
    (cited): `usize`; `v.iter()`; `for x in iter`; default-body
    trait methods; dot-call; `println!`; `let`; diagnostic map;
    `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the four
lines above. Then experiment: change `take(2)` to `take(100)` — the
first line becomes `5` (the full vec count, not 100, no panic). Or
change the inner `vec!` to `vec![]` — the first line becomes `0`,
the second `None`, the `for` body never runs.

*Now the laziness witness.* Build a tiny custom iterator that prints
each `next` call, then wrap it in `.take(2)` *without iterating*.
Save as `lazy.rs`:

```rust
struct Trace { n: u32 }
impl Iterator for Trace {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        println!("next:{}", self.n);
        let v = self.n;
        self.n += 1;
        Some(v)
    }
}

fn main() {
    println!("--- step A: build .take(2), do not iterate");
    let _wrapped = Trace { n: 0 }.take(2);
    println!("--- end step A");

    println!("--- step C: .count() on a fresh take(2)");
    let c = Trace { n: 0 }.take(2).count();
    println!("count = {}", c);
}
```

Step A prints zero `next:` lines between its markers — building the
`take(2)` value does *not* call `next` on the inner iterator even
once. Step C prints `next:0` then `next:1` then `count = 2` —
pulling on the wrapper *is* what calls `next` on the inner. That is
what "lazy" means here.

## What Changed

- Signature `fn take(self, n: usize) -> Take<Self> where Self: Sized,`
  (`trait.Iterator.md:1376`). `self` is consuming (lesson 102);
  `Self: Sized` is named-deferred.
- Return type `Take<Self>` is *itself an iterator* — an "iterator
  adapter" (`:1443`). For `v.iter()`, rustc spells the type
  `Take<Iter<'_, u64>>` (Probe 6 E0308).
- Building the `Take<Self>` value does not call `next` on the inner
  iterator (Probe 2 step A: zero `next:` lines). The inner pull
  happens when something drives the wrapper. That is *lazy*.
- `take(n)` past the end yields the full iterator's elements, then
  `None`. Five-element vec with `take(100)` prints `5`, no panic
  (Probe 3; `trait.Iterator.md:1383-1385`).
- Adapters compose with consumers via lesson-049 method chaining:
  `v.iter().take(2).count()` reads "take the slice iter, limit to the
  first two, then count." First chain in the run that puts a consumer
  after a non-consumer iterator method.

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![5, 6, 7, 8];
    let n = v.iter().take(3).count();
    println!("{}", n);

    let last = v.iter().take(2).last();
    println!("{:?}", last);

    for x in v.iter().take(0) {
        println!("{}", x);
    }
}
```

(a) Does it compile silently? What does it print?

(b) Replace `take(3)` with `take(50)`. Does the first line still
compile and run? What does it print?

(c) Bind `let iter = v.iter();` then write `let _ = iter.take(2);`
twice on consecutive lines. What E-code fires? Why?

*(Answers: (a) Yes. `3`, then `Some(6)` (the last of the first two
elements `[5, 6]` is `&6`), then nothing (the `for` body never runs
because `take(0)` yields zero elements). (b) Yes; prints `4` — the
full vec count. `take` limits itself to the underlying iterator's
length. (c) E0382. `take` takes ownership of `self` (lesson 102), so
the second call uses a moved binding. The `note:` block reads
`\`std::iter::Iterator::take\` takes ownership of the receiver
\`self\`, which moves \`iter\``.)*

## What To Ignore For Now

Deferred: the `Take<I>` struct's private fields; the `Self: Sized`
bound (still); the `by_ref` adapter named at `:1423-1437`; the
infinite-range example `(0..).take(n)` at `:1402` (`Range`'s
Iterator impl is still deferred); the default body of `take` in
core; `Iterator::skip` (audit §5 step 7), `enumerate` (step 8),
`map` / `filter` (closure arc); the other 71 provided methods.

## Evidence

See `../evidence/136-iterator-take.md`.
