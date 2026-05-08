---
id: 138-iterator-enumerate
status: accepted
evidence: ../evidence/138-iterator-enumerate.md
---

# Pair each element with its index using `iter.enumerate()`

## The Move

Lessons 136 and 137 installed `take(n)` and `skip(n)` — the first two
adapters in the run. Today's `enumerate()` is the next adapter sibling.
Same lazy shape (consuming `self`, returns a wrapper struct that itself
implements `Iterator`), but two new structural facts:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];

    for (i, x) in v.iter().enumerate() {
        println!("{} {}", i, x);
    }

    let n = v.iter().enumerate().count();
    println!("count = {}", n);

    let last_pair = v.iter().enumerate().last();
    println!("{:?}", last_pair);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
0 10
1 20
2 30
count = 3
Some((2, 30))
```

The trait declaration spells
`fn enumerate(self) -> Enumerate<Self> where Self: Sized,`
(`output/docs/rust/std/iter/trait.Iterator.md:1041`). The struct is
documented at `output/docs/rust/std/iter/struct.Enumerate.md:7`:
`pub struct Enumerate<I> { /* private fields */ }`. The two new facts
relative to `take`/`skip`:

1. **No second parameter.** The signature has *only* the receiver:
   `enumerate(self)`. Where `take(n)` and `skip(n)` each carried a
   `n: usize` argument, today's adapter takes none. This is the
   leanest adapter signature seen so far — receiver in, wrapper out.

2. **The yielded element type changes shape.** Where the inner
   iterator yields `Self::Item` (here `&u64`), the wrapper yields
   `(usize, Self::Item)` — a 2-tuple of (iteration index, original
   element). Per `struct.Enumerate.md:180`,
   `type Item = (usize, <I as Iterator>::Item)`. The `usize` first
   slot is the iteration index, starting at 0 and incrementing on
   each yielded element (`trait.Iterator.md:1043-1048`). This is the
   first place in the run where an Iterator method's *output element*
   differs in shape from its input.

The `for (i, x) in v.iter().enumerate()` form composes lessons 126
(tuple pattern in for-binding) + 072/073 (tuple type, `let`-tuple
destructure) directly: each yielded `(usize, &u64)` is split at the
binding step into `i` (the index) and `x` (the element ref). No
`pair.0` / `pair.1` needed. Inside the body, both names are usable.

The lazy framing carries from 136/137 unchanged: building an
`Enumerate<Self>` value does no work; the index counter advances only
when something pulls on the wrapper.

The corpus also names an overflow corner case: the index keeps its
count as a `usize`, and enumerating more than `usize::MAX` elements
either produces the wrong result or panics
(`trait.Iterator.md:1054-1063`). Named here as a corpus fact;
constructing an iterator with > 2^64 elements is impractical to probe.

## Mental Model Delta

- *Before:* "`take(n)` and `skip(n)` are adapters with the same
  `(self, n: usize) -> Wrapper<Self>` shape. The wrapper yields the
  inner iterator's elements (subject to the n-limit), unchanged in
  shape."
- *After:* "An adapter can *also* leave out the second parameter and
  *change the yielded element's shape*. `enumerate()` is the smallest
  example: no `n: usize`, and each yield becomes a 2-tuple
  `(usize, Self::Item)` where the new `usize` slot is the iteration
  index. The canonical loop shape is `for (i, x) in v.iter()
  .enumerate()` — lesson 126's tuple pattern destructures the yielded
  pair at the binding step. Lazy and consuming as before."

## Prerequisites

- Installed concepts:
  - **Lessons 137, 136** (load-bearing): the lazy-adapter family
    `(self) -> Wrapper<Self>` shape, the consuming-`self` rule, the
    "wrapper is itself an iterator" claim. Today is the next adapter
    sibling; the new facts are *no second parameter* and *yielded
    element changes shape*.
  - **Lesson 126** (load-bearing): `for (a, b) in iter` — tuple
    pattern in the for-binding slot. Today's working probe uses it on
    the `Enumerate<Iter<'_, u64>>` yield to split each `(usize, &u64)`
    into `i` and `x`.
  - **Lesson 072** (load-bearing): tuple type `(A, B)`. The yielded
    type `(usize, &u64)` is one specific tuple shape.
  - **Lesson 132** (load-bearing): `Iterator` with 75 provided
    methods inheriting via default bodies. `enumerate` is one of them.
  - **Lessons 134, 133, 131, 102, 049, 073, 080, 123, 022, 116,
    040, 011, 005, 003, 002, 001** (cited): `.last()` / `.count()`
    consumers chained on the wrapper; `.next()` mechanic; consuming
    `self` rule (no centered E0382 today — well-installed by
    133/134/136/137); method chaining; `let (a, b) = pair;`
    destructure mechanic; `usize` integer type; `v.iter()`;
    `for x in iter`; default-body trait methods; dot-call;
    `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the five
lines above. Now experiment: change `vec![10, 20, 30]` to `vec![]` —
the `for` body never runs, `count = 0`, the last line is `None`. No
panic.

For a type-pin contrast, write `typeprobe.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: Option<u64> = v.iter().enumerate().last();
}
```

`rustc typeprobe.rs` fires:

```text
error[E0308]: mismatched types
 --> typeprobe.rs:3:27
  |
3 |     let _x: Option<u64> = v.iter().enumerate().last();
  |             -----------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<u64>`, found `Option<(usize, &u64)>`
```

Read with the lesson 003 map. The actual return type rustc spells is
`Option<(usize, &u64)>` — empirical witness that `enumerate()` changed
the yielded element from `&u64` (lesson 134's `.last()` would return
`Option<&u64>`) to `(usize, &u64)`. The tuple wrapper is rustc-visible.

## What Changed

- Signature `fn enumerate(self) -> Enumerate<Self> where Self: Sized,`
  (`trait.Iterator.md:1041`). No second parameter. The receiver is the
  only argument.
- The yielded element type is `(usize, Self::Item)`
  (`struct.Enumerate.md:180`). For `v.iter()` over `Vec<u64>`, the
  yield is `(usize, &u64)`.
- The `usize` first slot is the iteration index, starting at 0 and
  incrementing on each yielded element
  (`trait.Iterator.md:1043-1048`).
- `for (i, x) in v.iter().enumerate()` composes lesson 126's tuple
  pattern destructuring with today's tuple-yielding adapter.
- `Enumerate<Self>` is itself an iterator: `.next()`, `.count()`,
  `.last()`, and `for` all work on it (Probe 1's three forms).
- Index overflow on > `usize::MAX` elements is a corpus-named corner
  case (`:1054-1063`); not probed.

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![5, 6, 7, 8];
    for (i, x) in v.iter().enumerate() {
        println!("{}: {}", i, x);
    }
    let pair = v.iter().enumerate().last();
    println!("{:?}", pair);
}
```

(a) Does it compile silently? What does it print?

(b) Replace line 3's pattern `(i, x)` with the single name `pair`.
Does it still compile? What does line 4 of the body need to become?

(c) Predict the rustc-named return type of
`v.iter().enumerate().last()`. Then write a `let _: u32 = ...;`
forced-mismatch and check.

*(Answers: (a) Yes. Five lines: `0: 5`, `1: 6`, `2: 7`, `3: 8`,
`Some((3, 8))`. (b) Yes; line 4 becomes `println!("{}: {}", pair.0,
pair.1);` — lesson 125's whole-tuple-and-`.0`/`.1` form. (c) The
rustc-named type is `Option<(usize, &u64)>`. The forced mismatch
fires E0308 with `expected u32, found Option<(usize, &u64)>` — same
type-pin shape lessons 134/135/136/137 captured for their adapters.)*

## What To Ignore For Now

Deferred: the `Enumerate<I>` struct's private fields and its `next`
body (the increment-counter logic); the `where Self: Sized` bound
(still); the std-doc note "`zip` provides similar functionality" at
`:1051-1052` (`zip` with `(0..)` produces the same shape, but
requires `Range`'s `Iterator` impl — still deferred since 022); the
`next_index` nightly method on `Enumerate<I>` at
`struct.Enumerate.md:23-25`; the `Enumerate<I>: DoubleEndedIterator`
impl bound at `:100`; the `usize::MAX` overflow / panic semantics
named at `:1054-1063` (impractical to probe); the default body of
`enumerate` in core. Next moves per audit §5: `fuse` (step 9),
`step_by` (step 10), `size_hint` (step 11).

## Evidence

See `../evidence/138-iterator-enumerate.md`.
