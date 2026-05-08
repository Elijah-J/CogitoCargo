# Evidence — Lesson 138: pair each element with its index using `iter.enumerate()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/138-iterator-enumerate.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/138-iterator-enumerate.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/138-iterator-enumerate.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet138/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
137.

## Direct prerequisite — lesson 137 (`Iterator::skip`)

Lesson 137 installed the lazy-adapter family `(self, n: usize) ->
WrapperStruct<Self>` shape, the lazy framing, and the chain-with-
consumer pattern. Today's `enumerate` reuses that family with two
new structural facts:

- The `n: usize` second-parameter slot is *gone*. Today's signature is
  the leaner `(self) -> Enumerate<Self>` — only the receiver. This is
  the first adapter without an extra argument.
- The yielded element type changes shape. `take`/`skip` returned
  wrappers that yield the inner iterator's `Self::Item` unchanged;
  today's wrapper yields `(usize, Self::Item)` — a 2-tuple.

The "wrapper is itself an iterator" claim from 136/137 carries
unchanged: Probe 1 chains `.count()` and `.last()` and uses `for x in`
on the same `Enumerate<Self>` value, all three forms produce expected
output.

The lazy framing carries from 136 (lesson 136's Probe 2 Trace+take
established laziness for the adapter family). Today does *not* re-
witness laziness with a Trace probe — same rationale lesson 137 gave
for `skip`: structural analogy is tight, and re-probing would add
appendix volume for negligible new fact value.

The consuming-`self` rule carries from 102/133/134/136/137. Today
does *not* re-witness E0382 — well-installed by four prior iterator
lessons. The signature at `:1041` reads `self` (no `&`, no `mut`),
identical receiver shape.

## Direct prerequisite — lesson 126 (for-pattern destructuring)

Lesson 126 installed `for (a, b) in iter` — tuple pattern in the
for-binding slot — on lesson 125's `v.iter().zip(w.iter())` yields.
Today's working probe reuses the *exact same shape* on a different
tuple-yielding source: `for (i, x) in v.iter().enumerate()`. The
yielded tuple is `(usize, &u64)` instead of `(&u64, &u64)`, but the
binding mechanic is identical — the pattern destructures each yielded
pair at the binding step into named parts.

This is the first non-`zip` source of tuple-yielding iterators in the
run. Lesson 126's claim "the for-loop's binding slot is a *pattern*,
not just a single identifier" generalizes immediately.

## Direct prerequisite — lesson 072 (tuple type and index)

Lesson 072 installed the tuple type `(A, B)` and the `.0` / `.1`
indexing mechanic. Today's yielded type `(usize, &u64)` is one
specific instance of the binary-tuple shape. The Check Yourself's
part (b) ("replace the pattern `(i, x)` with the single name `pair`,
then use `pair.0` and `pair.1`") explicitly invokes 072's index
mechanic as the equivalent non-destructuring form.

## Direct prerequisite — lesson 132 (the `Iterator` trait declaration)

Lesson 132 installed `std::iter::Iterator` with 75 provided methods
inheriting via default bodies. `enumerate` appears on the synopsis
box at `trait.Iterator.md:51-52` as
`fn enumerate(self) -> Enumerate<Self>` with `where Self: Sized` and
the `{ ... }` body marker (lesson 116's default-body shape). The
per-method declaration at `:1041` confirms the signature.

## Direct prerequisite — lessons 134, 133 (`Iterator::last`, `count`)

`.count()` and `.last()` are the consumers chained on
`v.iter().enumerate().count()` and `v.iter().enumerate().last()` in
Probe 1. Lesson 133 installed `count`'s `(self) -> usize` signature
(today's output line 4: `count = 3`). Lesson 134 installed `last`'s
`(self) -> Option<Self::Item>` signature; today's `Self::Item` is
`(usize, &u64)` (Enumerate's yield), so `.last()` returns
`Option<(usize, &u64)>` — Probe 2's E0308 names this exactly.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed `.next()` on a slice iterator returning
`Option<&T>`. Today's Probe 4 (empty iter) calls `.next()` on an
`Enumerate<Self>` value and reads `None` — the binding is itself an
iterator. Probe 1's Debug-format result `Some((2, 30))` reuses 131's
convention that Debug hides the `&` glyph on `&u64` for primitive
targets.

## Direct prerequisite — lesson 102 (`self`-by-value receiver)

Lesson 102 installed `self` (no `&`, no `mut`) as the consuming
receiver shape. Today's `enumerate` carries the same shape per
`trait.Iterator.md:1041`. *Not re-witnessed* with an E0382 probe today
— the diagnostic is well-installed by lessons 133/134/136/137 (four
prior iterator lessons captured the same E0382 + `note:` template,
each substituting only the method-name slot). A fifth substitution
would add appendix volume without new fact value.

## Direct prerequisite — lesson 049 (method chaining)

Lesson 049 installed left-associative method-chain parsing. Today's
`v.iter().enumerate().count()` and `v.iter().enumerate().last()`
parse as `((v.iter()).enumerate()).count/last()` — `enumerate` runs
on the slice iter, then the consumer runs on the resulting
`Enumerate<Iter<'_, u64>>`.

## Older supporting lessons

- **Lesson 073** (cited) — `let (a, b) = pair;` destructure mechanic.
  `for (i, x) in iter` is its loop variant via lesson 126.
- **Lesson 080** (cited) — `usize` is one specific row of the integer
  family. Enumerate's index slot is `usize` per `:1050` ("keeps its
  count as a `usize`").
- **Lesson 123** (cited) — `v.iter()` returns the slice iterator.
- **Lesson 022** (cited) — `for x in iter` works on any Iterator;
  Probe 1 line 1 reuses `for (i, x) in v.iter().enumerate()`.
- **Lesson 116** (cited) — default-body trait methods. The synopsis-
  box line ends in `{ ... }`.
- **Lessons 040, 011, 005, 003, 002, 001** (cited) — dot-call;
  `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.

## Probe 1 — working probe (for + enumerate, count + enumerate, last + enumerate)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/138-iterator-enumerate.rs`:

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

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
0 10
1 20
2 30
count = 3
Some((2, 30))
$ echo "run-exit=$?"
run-exit=0
```

Three claims simultaneously witnessed:

1. **`Enumerate<Self>` yields `(usize, Self::Item)` pairs.** Output
   lines 1-3 are `0 10`, `1 20`, `2 30` — three pairs, index `0..3`
   on the left, original element on the right. The `for (i, x)` tuple
   pattern from lesson 126 destructures each yielded pair into `i`
   (`usize`) and `x` (`&u64`).
2. **`.count()` chains onto `.enumerate()`.** Output line 4 is
   `count = 3`. Adding the index slot does not change the element
   count — three elements in, three pairs out. `Enumerate<Self>` is
   itself an iterator.
3. **`.last()` chains onto `.enumerate()`.** Output line 5 is
   `Some((2, 30))`. `.last()` returns
   `Option<(usize, &u64)>`; the last pair is `(2, &30)`. Debug format
   prints the inner tuple as `(2, 30)` — the `&` glyph on `&u64` is
   hidden for primitive targets (same convention 131/134/137
   captured).

This is the structural witness that `Enumerate<Self>` is itself an
iterator participating in every iterator-driving form lessons 131,
133, 134, 022, 126 installed, and that its yielded element is a
2-tuple of (index, original element).

**Debug format detail (load-bearing answer to the suggested-shape
question):** the print is `Some((2, 30))`, *not* `Some((2, &30))`.
Rust's `Debug` impl on `&T` delegates to `T`'s Debug for primitive
`T`, so the `&u64` reference prints as the underlying `u64` value.
This matches the convention lessons 131 (`Some(10)` for
`Option<&u64>`), 134 (`Some(30)` for `Option<&u64>`), and 137
(`Some(30)` for `Option<&u64>`) all captured.

## Probe 2 — type-pin via E0308 on `Option<u64>` (centered new contrast)

Source typeprobe.rs:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: Option<u64> = v.iter().enumerate().last();
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typeprobe.rs:3:27
  |
3 |     let _x: Option<u64> = v.iter().enumerate().last();
  |             -----------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<u64>`, found `Option<(usize, &u64)>`
  |             |
  |             expected due to this
  |
  = note: expected enum `Option<u64>`
             found enum `Option<(usize, &u64)>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

**This is the centered new contrast today.** It pins the new fact —
the yielded element changes shape from `Self::Item` to
`(usize, Self::Item)` — at compile time, with rustc spelling the
actual yield in the inline expected/found labels (`Option<(usize,
&u64)>`) and the secondary note (`found enum
\`Option<(usize, &u64)>\``).

Without enumerate, `v.iter().last()` would be `Option<&u64>` (lesson
134's empirical witness). Compare:

- 134's E0308 contrast: `expected Option<u64>, found Option<&u64>`
  — the yielded shape wraps the original `Self::Item` (a reference).
- 138's E0308 contrast: `expected Option<u64>, found Option<(usize,
  &u64)>` — the yielded shape wraps a *tuple* whose second slot is
  the original `Self::Item`.

The difference is exactly the structural fact today installs.

## Probe 3 — type-pin via E0308 naming `Enumerate<Iter<'_, u64>>`

Source typeprobe2.rs:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: u32 = v.iter().enumerate();
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typeprobe2.rs:3:19
  |
3 |     let _x: u32 = v.iter().enumerate();
  |             ---   ^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `Enumerate<Iter<'_, u64>>`
  |             |
  |             expected due to this
  |
  = note: expected type `u32`
           found struct `Enumerate<std::slice::Iter<'_, u64>>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

rustc spells the actual return type out: `Enumerate<Iter<'_, u64>>`
inline, `Enumerate<std::slice::Iter<'_, u64>>` in the secondary
note. Same forced-error type-pin technique lessons 134/135/136/137
used; today substitutes `Enumerate` for `Take`/`Skip`. The
`Enumerate<...>` struct is referenced opaquely; today's lesson does
not unpack its private fields.

## Probe 4 — empty-iter corroboration

Source empty.rs:

```rust
fn main() {
    let v: Vec<u64> = vec![];
    let n = v.iter().enumerate().count();
    println!("count = {}", n);
    let f = v.iter().enumerate().next();
    println!("next  = {:?}", f);
}
```

Transcript:

```text
$ rustc empty.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./empty
count = 0
next  = None
$ echo "run-exit=$?"
run-exit=0
```

Empty source iterator → empty enumerate. `count = 0`, `next = None`,
no panic. Confirms that Enumerate is structurally well-behaved on an
empty source — same null-case handling as `take`/`skip` past-end.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 51-52 (synopsis-box line)

Verbatim:

```
    fn enumerate(self) -> Enumerate<Self> ⓘ
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape — what
licenses every iterator inheriting `enumerate` for free.

### `output/docs/rust/std/iter/trait.Iterator.md` line 1041 (per-method declaration)

Verbatim:

```
#### fn [enumerate](#method.enumerate)(self) -> [Enumerate](struct.Enumerate.md "struct std::iter::Enumerate")<Self> [ⓘ](#) where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

Authoritative source for:

- **Method name** `enumerate` and **receiver shape** `(self)` —
  bare `self` (lesson 102 consuming) with *no* second parameter.
  This is the new structural fact: the leanest adapter signature
  seen so far.
- **Return type** `Enumerate<Self>` — the wrapper struct documented
  at `output/docs/rust/std/iter/struct.Enumerate.md`.
- **`where Self: Sized`** — same bound `take` / `skip` / `count` /
  `last` carry; named-deferred today.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1043-1052 (prose summary)

Verbatim:

```
Creates an iterator which gives the current iteration count as well as
the next value.

The iterator returned yields pairs `(i, val)`, where `i` is the
current index of iteration and `val` is the value returned by the
iterator.

`enumerate()` keeps its count as a [`usize`](../primitive.usize.md "primitive usize"). If you want to count by a
different sized integer, the [`zip`](trait.Iterator.md#method.zip "method std::iter::Iterator::zip") function provides similar
functionality.
```

This grounds:

- **"Yields pairs `(i, val)`"** — Probe 1 lines 1-3 (`0 10`,
  `1 20`, `2 30`) and Probe 1 line 5 (`Some((2, 30))`). The yielded
  element is a 2-tuple.
- **"`i` is the current index of iteration"** — Probe 1 lines 1-3,
  index advances `0, 1, 2`.
- **"keeps its count as a `usize`"** — Probe 2's E0308 names the
  yielded type as `(usize, &u64)`, with `usize` in the first slot.
- **"`zip` provides similar functionality"** — named-deferred today
  (`zip` with `(0..)` reproduces enumerate's shape, but `Range`'s
  `Iterator` impl is still deferred since lesson 022).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1054-1063 (Overflow + Panics)

Verbatim:

```
##### Overflow Behavior

The method does no guarding against overflows, so enumerating more than
[`usize::MAX`](../primitive.usize.md#associatedconstant.MAX "associated constant usize::MAX") elements either produces the wrong result or panics. If
overflow checks are enabled, a panic is guaranteed.

##### Panics

The returned iterator might panic if the to-be-returned index would
overflow a [`usize`](../primitive.usize.md "primitive usize").
```

Named as a corpus fact in the lesson's *What Changed* and *What To
Ignore For Now* sections. Not probed — constructing an iterator with
> 2^64-1 elements is impractical (would require petabytes of memory
or an infinite source).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1065-1076 (basic example)

Verbatim:

```
##### Examples

```
let a = ['a', 'b', 'c'];

let mut iter = a.into_iter().enumerate();

assert_eq!(iter.next(), Some((0, 'a')));
assert_eq!(iter.next(), Some((1, 'b')));
assert_eq!(iter.next(), Some((2, 'c')));
assert_eq!(iter.next(), None);
```
```

The std doc example matches Probe 1's structural pattern modulo
source-collection shape (`['a', 'b', 'c'].into_iter()` yields
`char`; `Vec<u64>.iter()` yields `&u64`). Both shapes confirm the
yield is `Some((i, val))` for indices `0, 1, 2, ...` until the
source exhausts to `None`.

### `output/docs/rust/std/iter/struct.Enumerate.md` lines 1-15

Verbatim:

```
# Struct Enumerate

1.0.0 ·

```
pub struct Enumerate<I> { /* private fields */ }
```

Expand description

An iterator that yields the current count and the element during iteration.

This `struct` is created by the [`enumerate`](trait.Iterator.md#method.enumerate "method std::iter::Iterator::enumerate") method on [`Iterator`](trait.Iterator.md). See its
documentation for more.
```

Grounds the wrapper-struct claim. Same opaque-struct treatment as
`Take<I>` (lesson 136) and `Skip<I>` (lesson 137).

### `output/docs/rust/std/iter/struct.Enumerate.md` lines 162, 166, 180

Verbatim:

```
### impl<I> Iterator for Enumerate<I> where I: Iterator,

#### fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)>

...

#### type Item = (usize, <I as Iterator>::Item)
```

Authoritative source for **the new fact today: the yielded element
type is `(usize, Self::Item)`**. Where `Take<I>` and `Skip<I>`
inherit `type Item = I::Item` (the inner element unchanged),
`Enumerate<I>` rewrites the associated type to a 2-tuple. This is
the first place in the run where an iterator adapter changes the
yielded element's shape.

### `output/docs/rust/error_codes/E0308.md`

Probes 2 and 3's diagnostic. Type-pin technique installed at
lessons 134 Probe 5, 135 Probe 6, 136 Probe 6, 137 Probe 4.

## Claim-to-evidence map

- "`for (i, x) in v.iter().enumerate()` on `vec![10, 20, 30]` prints
  `0 10`, `1 20`, `2 30`" — Probe 1 lines 1-3.
- "`v.iter().enumerate().count()` on the same vec returns `3`" —
  Probe 1 line 4.
- "`v.iter().enumerate().last()` returns `Some((2, 30))` (Debug
  format hides the `&` glyph on `&u64`)" — Probe 1 line 5.
- "The yielded element type of `Enumerate<Iter<'_, u64>>` is
  `(usize, &u64)`" — Probe 2 (E0308 with
  `expected Option<u64>, found Option<(usize, &u64)>`); corpus
  source `struct.Enumerate.md:180` (`type Item = (usize, <I as
  Iterator>::Item)`); corpus prose
  `trait.Iterator.md:1046-1048` ("yields pairs `(i, val)`").
- "The first slot of the yielded tuple is the iteration index,
  starting at 0" — Probe 1 lines 1-3 (indices `0, 1, 2`); corpus
  prose at `:1043-1048` ("the current iteration count" / "the
  current index of iteration") and `:1050` ("keeps its count as a
  `usize`").
- "`enumerate` takes the receiver by value (`self`); no second
  parameter" — `trait.Iterator.md:1041` (signature spells
  `(self) -> Enumerate<Self>`).
- "Return type is `Enumerate<Self>` — itself an iterator" —
  `trait.Iterator.md:1041` (signature names `Enumerate<Self>`);
  `struct.Enumerate.md:1-15` (`pub struct Enumerate<I> { ... }`);
  `struct.Enumerate.md:162` (`impl<I> Iterator for Enumerate<I>
  where I: Iterator`); Probe 1 (`.count()`, `.last()`, and `for`
  all work on the binding); Probe 3 (rustc names the type
  `Enumerate<Iter<'_, u64>>` in the E0308 expected/found labels).
- "Lazy: building the `Enumerate<Self>` value does not call `next`
  on the inner iterator" — *not re-witnessed today*. Inherited from
  lesson 136 by structural analogy: enumerate is listed adjacent to
  take/skip in the synopsis box's *Provided methods* block returning
  a wrapper struct, the page treats the family as one, and lesson
  136's Trace probe established the laziness shape for the adapter
  family.
- "`enumerate` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:13` ("// Provided methods" comment precedes
  `enumerate` at `:51-52`); lesson 132 evidence appendix.
- "Empty source → empty enumerate, no panic" — Probe 4
  (`vec![].iter().enumerate().count() == 0`, `.next() == None`).
- "Index overflow on > `usize::MAX` elements: wrong result or
  panic" — `trait.Iterator.md:1054-1063` (Overflow Behavior +
  Panics blocks). Not probed (impractical).
- "Adapters compose with consumers via lesson-049 method chaining"
  — Probe 1 (three different consumer-after-adapter chains all
  compile and produce expected output); inherited from lessons
  136/137.

## Negative / contrast probe coverage

Two contrasts captured (one centered, one corroborating):

- **Probe 2 (E0308 on `Option<u64>`)** — *the centered new contrast
  today*. Without this probe, the "yielded element shape changes
  from `Self::Item` to `(usize, Self::Item)`" claim would rest only
  on corpus prose and the struct-doc `type Item =` line. The probe
  gives empirical witness from rustc's mouth: the actual return type
  is named `Option<(usize, &u64)>`, with the inner tuple shape
  spelled out. Direct contrast against lesson 134's
  `Option<&u64>` for the same `.last()` chained on the bare iter.
- **Probe 3 (E0308 on `u32`)** — type-pin contrast for the
  `Enumerate<Self>` return. Without it, the claim "the result is
  itself an iterator" rests on Probe 1's silent compile (which is
  consistent with several inferred annotations). E0308's labels
  naming `Enumerate<Iter<'_, u64>>` and
  `Enumerate<std::slice::Iter<'_, u64>>` pin the actual wrapper-
  struct type from rustc's mouth.
- **Probe 4 (empty iter)** — empirical corroboration of the no-
  panic empty-case behavior. Same shape as 137's Probe 3 modulo
  source size (137 used `skip(100)` on five-element vec to force
  emptiness; today uses an empty source).

**No centered E0382 today.** The consuming-`self` rule is well-
installed by lessons 102/133/134/136/137 (five prior captures of the
same E0382 + `note:` template, each substituting only the method-
name slot). A sixth substitution would add appendix volume without
new fact value. Today's signature at `trait.Iterator.md:1041` reads
`self` (no `&`, no `mut`) — same shape; the rule applies.

**Why no laziness re-probe today:** lesson 136's Probe 2 (Trace +
take, three steps) installed the laziness shape for the adapter
family. Enumerate is structurally identical (consuming `self`,
returns wrapper struct from the same synopsis-box block). The std
doc treats `enumerate` alongside `take`/`skip` as part of the
Provided methods block. Re-running a Trace probe with `enumerate`
in place of `take` would add appendix volume for negligible new
fact value. The laziness claim today is named as inherited, not
re-witnessed.

## Iterator API audit alignment

This lesson is step 8 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 8. **`enumerate`** — yields `(usize, Item)`; `usize::MAX` overflow
>    note.

Audit §4.3 lists `enumerate` as ready-now, composing "self-by-value
102 + 072/073 (tuple) + 080". Today executes that move per audit §5
step 8. Lesson 137's unlock list named today's move:

> future "`Iterator::enumerate` — `(self) -> Enumerate<Self>`,
> yields `(usize, Item)`" moves (audit §5 step 8 — reuses today's
> adapter shape; first adapter today's audience meets that yields a
> *tuple* element)

The new graph fact today: where lessons 136/137 installed the
adapter shape with a `n: usize` second parameter, today installs
**(a) the leanest adapter signature** (no second parameter, only the
receiver) and **(b) the first adapter that changes the yielded
element's shape** (from `Self::Item` to `(usize, Self::Item)`). This
unlocks `fuse` (audit §5 step 9 — sticky-`None` adapter), `step_by`
(step 10), `size_hint` (step 11), and the eventual closure-driven
adapters (`map`, `filter_map`, `scan`) — `map` specifically also
changes the yielded element's type, and reuses today's "the yielded
type is rewritten by the adapter" frame.
