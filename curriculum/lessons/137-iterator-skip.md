---
id: 137-iterator-skip
status: accepted
evidence: ../evidence/137-iterator-skip.md
---

# Drop the first `n` elements with `iter.skip(n)`

## The Move

Lesson 136 installed `take(n)` — the first *adapter* in the run. Today's
`skip(n)` is its inverse sibling. Where `take(2)` keeps the first two
elements and discards the rest, `skip(2)` discards the first two and
yields the rest. The structural shape is identical: consuming `self`,
one `n: usize` argument, returns a wrapper struct that itself implements
`Iterator`.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    let n = v.iter().skip(2).count();
    println!("{}", n);

    let first_remaining = v.iter().skip(2).next();
    println!("{:?}", first_remaining);

    for x in v.iter().skip(3) {
        println!("{}", x);
    }
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
3
Some(30)
40
50
```

The trait declaration spells
`fn skip(self, n: usize) -> Skip<Self> where Self: Sized,`
(`output/docs/rust/std/iter/trait.Iterator.md:1352`). All three
structural slots carry over from lesson 136 unchanged:

1. **Receiver `self`** — consuming (lesson 102). Calling `.skip(n)`
   moves the iterator. Same E0382 + `note:` shape lesson 136 captured;
   appendix Probe 5 is the continuity check.
2. **`n: usize`** — same second-parameter slot lesson 135's `nth` and
   136's `take` installed.
3. **Return type `Skip<Self>`** — a wrapper struct documented at
   `output/docs/rust/std/iter/struct.Skip.md` (`pub struct Skip<I> { /*
   private fields */ }`). It implements `Iterator`, so `.next()`,
   `.count()`, `.last()`, and `for` all work on it. Today refers to
   the struct opaquely.

The lazy framing carries from 136: building a `Skip<Self>` value does
no work; the work happens only when something pulls on the wrapper.

## Mental Model Delta

- *Before:* "`take(n)` is the smallest adapter; it keeps the first `n`
  elements of the inner iterator."
- *After:* "`take(n)` and `skip(n)` are a complementary pair on the
  same trait. `take(n)` keeps the first `n` and discards the rest;
  `skip(n)` discards the first `n` and keeps the rest. Together they
  partition the source iterator at position `n` —
  `iter.take(n).count() + iter.skip(n).count() == iter.count()` for any
  in-range `n`. Both consume `self`, both take a `usize`, both return a
  wrapper struct that itself implements `Iterator`, and both are lazy:
  building the wrapper does not pull on the inner iterator. The new
  fact today is the inverse semantic — the structural shape is
  inherited from lesson 136."

## Prerequisites

- Installed concepts:
  - **Lesson 136** (load-bearing): the adapter shape `(self, n: usize)
    -> WrapperStruct<Self>`, the lazy framing (building the wrapper
    does no work), and the chain-with-consumer pattern
    (`v.iter().take(2).count()`). Today is the inverse sibling: every
    structural slot is reused, and the only new fact is the inverse
    semantic.
  - **Lessons 134, 133** (load-bearing): `.last()` and `.count()` are
    consumers chained on `skip(2).count()` and on the inverse-sum
    probe. Output line 1 (`3`) and the inverse probe both depend on
    `count`'s `(self) -> usize` signature.
  - **Lesson 131** (load-bearing): `.next()` on a slice iterator
    returning `Option<&T>`. Probe 1 calls `.skip(2).next()` and reads
    `Some(30)` — the binding is itself an iterator.
  - **Lessons 132, 102, 049** (load-bearing): `Iterator` trait with 75
    provided methods (`skip` is one); consuming `self` receiver;
    method chaining.
  - **Lessons 080, 123, 022, 116, 040, 011, 005, 003, 002, 001**
    (cited): `usize`; `v.iter()`; `for x in iter`; default-body trait
    methods; dot-call; `println!`; `let`; diagnostic map; `fn main`;
    rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the four
lines above. Then run a second probe witnessing the inverse-sum
identity:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    println!("total={}", v.iter().count());
    println!("take(2)={}", v.iter().take(2).count());
    println!("skip(2)={}", v.iter().skip(2).count());
}
```

Output: `total=5`, `take(2)=2`, `skip(2)=3`. The sum `2 + 3 = 5`
matches the total. That is the inverse-pair identity in action.

Now experiment: change `skip(2)` to `skip(100)` — the first line of the
working probe becomes `0`, the second becomes `None`, the `for` body
never runs. Per the std doc at `:1356-1359`, "if the original iterator
is too short, then the returned iterator is empty." No panic.

## What Changed

- Signature `fn skip(self, n: usize) -> Skip<Self> where Self: Sized,`
  (`trait.Iterator.md:1352`). All three slots match `take`'s shape
  (lesson 136); `Skip<Self>` is a wrapper struct named at
  `struct.Skip.md`.
- `skip(n)` discards the first `n` elements and yields the rest; if
  the source is shorter than `n`, the result is empty (no panic).
- `take` and `skip` are an inverse pair. For any in-range `n`:
  `iter.take(n).count() + iter.skip(n).count() == iter.count()`. Probe 2
  witness: `2 + 3 == 5`.
- The lazy framing from 136 carries: building a `Skip<Self>` value does
  no work; pulling on the wrapper drives the inner iterator.

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![5, 6, 7, 8];
    let n = v.iter().skip(1).count();
    println!("{}", n);

    let f = v.iter().skip(3).next();
    println!("{:?}", f);

    for x in v.iter().skip(4) {
        println!("{}", x);
    }
}
```

(a) Does it compile silently? What does it print?

(b) What does `v.iter().take(1).count() + v.iter().skip(1).count()`
evaluate to, and why?

*(Answers: (a) Yes. `3` (skip 1 of 4 leaves 3); `Some(8)` (after
dropping `&5, &6, &7`, the first remaining is `&8`); the `for` body
never runs (skip(4) on a 4-element vec leaves zero). (b) `4`. The pair
partitions the source at position 1: `take(1)` keeps `&5`, `skip(1)`
keeps `&6, &7, &8`; their counts sum to the full count.)*

## What To Ignore For Now

Deferred: the `Skip<I>` struct's private fields and its `next` body
(its default body delegates to `nth` per `:1361`); the `Self: Sized`
bound (still); the `skip_while` adapter at `:1284` (closure-driven,
gated on the closure arc); the std-doc note "Rather than overriding
this method directly, instead override the `nth` method" at `:1361`
(implementor advice, not user-facing); the `Skip<I>: DoubleEndedIterator
+ ExactSizeIterator` impl bound at `struct.Skip.md:47-49`. Next moves
per audit §5: `enumerate` (step 8), `fuse` (step 9), `step_by` (step
10), `size_hint` (step 11).

## Evidence

See `../evidence/137-iterator-skip.md`.
