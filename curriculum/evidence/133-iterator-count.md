# Evidence — Lesson 133: count a slice iterator's elements with `.count()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/133-iterator-count.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/133-iterator-count.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/133-iterator-count.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet133/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
132.

## Direct prerequisite — lesson 132 (the `Iterator` trait declaration)

Lesson 132 installed:

- `std::iter::Iterator` declares two required items (`type Item;` and
  `fn next(&mut self) -> Option<Self::Item>;`) plus 75 provided
  methods carrying default bodies (lesson 116).
- An impl that supplies only the required surface inherits all 75
  provided methods. Probe 2 of lesson 132 witnessed this on a
  user-defined `Counter` impl: the impl supplied only `type Item` and
  `fn next`, yet `c.count()` was callable and printed `count = 3`.

Today picks one of those 75 methods and walks it as a centered move:
the signature on the synopsis box at
`output/docs/rust/std/iter/trait.Iterator.md:19-20` and the per-method
section at `:416` say `fn count(self) -> usize where Self: Sized,`.
Today's lesson 132 unlock list explicitly named: *"future
`Iterator::count` — `(self) -> usize`, calls `.next()` until `None` and
counts" moves (audit §5 step 3 — anchors to today's '`count` is one of
the 75 provided methods sitting in the trait body with a default
`{ ... }` body')*. Today executes that next move per audit §5 step 3.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed:

- A slice iterator (`v.iter()`) has a method `.next()` that pulls one
  element. The wrapper is `Option<&T>`; once exhausted, calls return
  `None`.
- `.next()`'s receiver is `&mut self`, witnessed empirically through
  the E0596 contrast.

Today's `count` is *defined* in terms of `.next()`: `trait.Iterator.md:420-421`
says verbatim "This method will call `next` repeatedly until `None`
is encountered, returning the number of times it saw `Some`." Probe 6
(side-effect `Trace` impl) witnesses this empirically: a limit-3 Trace
prints four `next()` calls (three `Some(_)` + one `None`) before
`count` returns 3. Probe 7 corroborates: a limit-0 Trace records one
`next() -> None` call before `count` returns 0 — consistent with the
following sentence at `:421-422` ("Note that `next` has to be called
at least once even if the iterator does not have any elements.").

## Direct prerequisite — lesson 102 (self-by-value receiver)

Lesson 102 installed:

- `self` (no `&`, no `mut`) is the third receiver shape, shorthand for
  `self: Self`. Methods declared this way *consume* the value.
- After `w.into_inner()`, `w` cannot be used again. The diagnostic is
  `E0382 use of moved value: \`w\`` with a `note:` block at the
  method-definition site naming the receiver shape:
  `Wrapper::into_inner takes ownership of the receiver \`self\`,
  which moves \`w\``.

Today is the first place in the run where lesson 102's rule fires on
a *stdlib* method. The signature `fn count(self) -> usize` on
`trait.Iterator.md:416` is the lesson-102 shape. Probe 3 captures the
contrast: same E0382, same `note:` shape, but the `-->` of the note
points at core's `iter/traits/iterator.rs:225:13` — the std source for
`Iterator::count`. The `note:` text reads verbatim:
*`count` takes ownership of the receiver `self`, which moves `iter`* —
identical structure to lesson 102's `Wrapper::into_inner` note.

## Direct prerequisite — lesson 080 (integer type family)

Lesson 080 installed:

- Twelve integer types in two axes: signed (`i`) vs unsigned (`u`),
  plus six widths (`8`, `16`, `32`, `64`, `128`, `size`).
- `usize` and `isize` are architecture-dependent — 64 bits on this
  64-bit host (lesson 077).

Today's `count`'s return type is `usize`. The trait declaration at
`trait.Iterator.md:416` is unambiguous: the return-type slot is
`usize`, not a generic. Probes 4 and 5 sandwich this empirically:
`let n: usize = v.iter().count();` compiles silently; `let n: u64 =
...` fires E0308 with the inline label `expected u64, found usize`,
naming the actual return type.

## Older supporting lessons

- **Lesson 123** (cited) — `v.iter()` returns a slice iterator yielding
  `&T`. The receiver `iter` in today's working probe is exactly that
  iterator.
- **Lesson 119** (cited) — `Option<T>` / `Some` / `None`. `count`'s
  defining loop reads the return values from `next`; the `Some(_)` /
  `None` distinction is what the count tallies.
- **Lesson 116** (cited) — default-body trait methods. The
  trait-declaration line on the synopsis box `fn count(self) -> usize
  where Self: Sized { ... }` ends in `{ ... }` (the lesson-116
  default-body marker), which is what licenses every iterator inheriting
  `count` for free.
- **Lesson 107** (cited) — `Vec<T>` with `.len() -> usize`. Same return
  type as today's `count`. (`v.iter().count()` and `v.len()` both yield
  `5` for `vec![10, 20, 30, 40, 50]` here, but they compute differently:
  `len` reads a stored field, `count` walks `next`.)
- **Lessons 040, 011, 005, 003, 002, 001** (cited) — dot-call;
  `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.
  Same roles as lessons 131 and 132.

## Probe 1 — working probe (count a five-element slice iterator)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/133-iterator-count.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().count();
    println!("{}", n);
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
5
$ echo "run-exit=$?"
run-exit=0
```

Centered claim — "`v.iter().count()` returns the number of elements
as an integer printable with `{}`" — is carried by the silent compile
plus the printed `5`. Mirrors the std doc's example shape at
`trait.Iterator.md:438-443`:

```text
let a = [1, 2, 3, 4, 5];
assert_eq!(a.iter().count(), 5);
```

(The doc example uses array `[i32; 5]` rather than `Vec<u64>` and
`assert_eq!` rather than `println!`, but the count semantic is
identical.)

## Probe 2 — empty-vec corroboration (count == 0)

```rust
fn main() {
    let v: Vec<u64> = vec![];
    let n = v.iter().count();
    println!("{}", n);
}
```

Compile silent; run prints `0`. Witnesses: `count` does not have a
non-empty precondition. Zero `Some(_)` returns from `next` means
count == 0.

## Probe 3 — centered contrast (use after `.count()` fires E0382)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let n = iter.count();
    let _ = iter.count();
    println!("{}", n);
}
```

Verbatim diagnostic:

```text
error[E0382]: use of moved value: `iter`
 --> use_after.rs:5:13
  |
3 |     let iter = v.iter();
  |         ---- move occurs because `iter` has type `std::slice::Iter<'_, u64>`, which does not implement the `Copy` trait
4 |     let n = iter.count();
  |                  ------- `iter` moved due to this method call
5 |     let _ = iter.count();
  |             ^^^^ value used here after move
  |
note: `count` takes ownership of the receiver `self`, which moves `iter`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:225:13
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
4 |     let n = iter.clone().count();
  |                 ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
exit=1
```

This is the centered contrast for "`count`'s receiver is consuming."
Three structural alignments to lesson 102:

1. **Same E-code.** `E0382 use of moved value` — identical to lesson
   102's contrast on `Wrapper::into_inner`.
2. **Same `note:` shape.** `note: \`count\` takes ownership of the
   receiver \`self\`, which moves \`iter\`` — same template as lesson
   102's `note: \`Wrapper::into_inner\` takes ownership of the receiver
   \`self\`, which moves \`w\``. Only the method-name and binding-name
   substitute.
3. **`-->` of the note points at the method definition.** Lesson 102's
   note pointed at the user's source line; today's points at core's
   `library/core/src/iter/traits/iterator.rs:225:13` — the std source
   for `Iterator::count`'s declaration. (The `225:13` line offset is
   internal to the rustc distribution; the path itself is the
   load-bearing fact.)

The `help:` line proposes `iter.clone().count()` — a `.clone()` arc
named-deferred today. That `help:` is also why today's *What To
Ignore For Now* names `.clone()` on iterators.

The diagnostic literally proves the trait-declaration line at
`trait.Iterator.md:416` reads `fn count(self) -> usize`: if the
receiver were `&self` or `&mut self`, the second call would not
move-fail. Receiver-by-value is the only shape that produces this
exact E0382.

## Probe 4 — type-pin (return type is `usize`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n: usize = v.iter().count();
    println!("{}", n);
}
```

Compile silent; run prints `5`. Witnesses: `v.iter().count()`
empirically has type `usize` on this host. Matches `trait.Iterator.md:416`:
`fn count(self) -> usize where Self: Sized,`.

## Probe 5 — type-pin contrast (`u64` instead of `usize` fires E0308)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n: u64 = v.iter().count();
    println!("{}", n);
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typetest_neg.rs:3:18
  |
3 |     let n: u64 = v.iter().count();
  |            ---   ^^^^^^^^^^^^^^^^ expected `u64`, found `usize`
  |            |
  |            expected due to this
  |
help: you can convert a `usize` to a `u64` and panic if the converted value doesn't fit
  |
3 |     let n: u64 = v.iter().count().try_into().unwrap();
  |                                  ++++++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Sandwiches Probe 4: `usize` compiles silently; `u64` does not. The
inline label `expected u64, found usize` names the actual return type
in rustc's own words. Lesson 080's distinction "twelve different
integer types, not interchangeable" is what this contrast trips.

## Probe 6 — empirical witness for "calls next() until None"

```rust
struct Trace {
    n: u32,
    limit: u32,
}

impl Iterator for Trace {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.limit {
            println!("next() -> Some({})", self.n);
            let v = self.n;
            self.n += 1;
            Some(v)
        } else {
            println!("next() -> None");
            None
        }
    }
}

fn main() {
    let t = Trace { n: 0, limit: 3 };
    let c = t.count();
    println!("count = {}", c);
}
```

Output:

```text
next() -> Some(0)
next() -> Some(1)
next() -> Some(2)
next() -> None
count = 3
```

Witnesses the corpus claim at `trait.Iterator.md:420-421` ("This
method will call `next` repeatedly until `None` is encountered,
returning the number of times it saw `Some`") empirically. Four
`next()` calls happen; the count returned (3) equals the number of
`Some(_)` returns. The `Trace` impl supplies only the required surface
from lesson 132 (`type Item` + `fn next`), so this also corroborates
lesson 132's claim that `count` is one of the 75 *provided* methods
inherited from default bodies — the impl never wrote `fn count` itself.

## Probe 7 — empty-Counter corroboration (next called at least once)

Same `Trace` source, with `limit: 0`. Output:

```text
next() -> None
count = 0
```

Witnesses the corpus claim at `trait.Iterator.md:421-422` ("Note that
`next` has to be called at least once even if the iterator does not
have any elements") empirically. The limit-0 Trace records exactly
one `next() -> None` call before `count` returns 0.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` line 416 (per-method declaration)

Verbatim:

```
#### fn [count](#method.count)(self) -> [usize](../primitive.usize.md) where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

This is the authoritative source for:

- the **method name** `count` and the **receiver shape** `(self)` —
  bare `self` with no `&`, lesson 102's consuming receiver.
- the **return type** `usize` — lesson 080's architecture-width
  unsigned integer.
- the **bound** `where Self: Sized` — named-deferred today; appears
  on most provided methods.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 19-20 (synopsis-box line)

Verbatim:

```
    fn count(self) -> usize
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape. Same
content as line 416, but on the trait-declaration synopsis at the top
of the page rather than the per-method section. The `{ ... }` is
*why* every iterator gets `count` for free: the trait carries a
default body, and `Self: Sized` impls inherit it without writing one.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 418-422 (prose summary)

Verbatim:

```
Consumes the iterator, counting the number of iterations and returning it.

This method will call `next` repeatedly until `None` is encountered,
returning the number of times it saw `Some`. Note that `next` has to be
called at least once even if the iterator does not have any elements.
```

This grounds:

- "**Consumes** the iterator" — the receiver-by-value behavior witnessed
  empirically by Probe 3's E0382.
- "Call `next` repeatedly until `None`" — witnessed empirically by
  Probe 6's four-line trace.
- "`next` has to be called at least once even if the iterator does
  not have any elements" — witnessed empirically by Probe 7's single
  `next() -> None` line for an empty Counter.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 424-434 (overflow + panic notes)

Verbatim:

```
##### Overflow Behavior

The method does no guarding against overflows, so counting elements of
an iterator with more than `usize::MAX` elements either produces the
wrong result or panics. If overflow checks are enabled, a panic is
guaranteed.

##### Panics

This function might panic if the iterator has more than `usize::MAX`
elements.
```

Today's lesson names this as a corpus fact in *What Changed* and does
not probe it. Constructing an iterator longer than `usize::MAX`
(`2^64 - 1` on this host) is impractical empirically; the corpus
statement is sufficient grounding.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 436-443 (example)

Verbatim:

```
##### Examples

```
let a = [1, 2, 3];
assert_eq!(a.iter().count(), 3);

let a = [1, 2, 3, 4, 5];
assert_eq!(a.iter().count(), 5);
```
```

Today's working probe (Probe 1) mirrors the second example
structurally — `[1, 2, 3, 4, 5]` becomes `vec![10, 20, 30, 40, 50]`,
`assert_eq!(_, 5)` becomes `println!("{}", n)` printing `5`. The
first example's `[1, 2, 3]` -> `3` shape is mirrored by today's
*Check Yourself* `vec![7, 8, 9, 10]` -> `4`.

### `output/docs/rust/error_codes/E0382.md`

Probe 3's diagnostic. The error code documents the "use of moved
value" rule. Lesson 102 already installed the diagnostic shape;
today's contrast reuses it on a stdlib method. Lesson 003's four-part
diagnostic map (headline + location + source-excerpt + help/note)
applies unchanged.

### `output/docs/rust/error_codes/E0308.md`

Probe 5's diagnostic. The error code documents the "mismatched types"
rule. Lesson 019 already installed the diagnostic shape with
`expected/found` inline labels; today's contrast reuses it for
return-type witnessing.

## Claim-to-evidence map

- "`.count()` is callable on a slice iterator and returns a `usize`
  that prints `5` for `vec![10, 20, 30, 40, 50]`" — Probe 1 (silent
  compile + `5` printed); `trait.Iterator.md:416` (signature);
  `trait.Iterator.md:438-443` (matching std example).
- "`count` takes the receiver by value (`self`, no `&`); calling it
  moves the iterator and the binding cannot be used again" —
  `trait.Iterator.md:416` (`fn count(self) -> usize`); Probe 3's
  E0382 (`note: \`count\` takes ownership of the receiver \`self\`,
  which moves \`iter\``).
- "`count` returns `usize`" — `trait.Iterator.md:416` (return-type
  slot); Probe 4 silent compile of `let n: usize = v.iter().count();`;
  Probe 5 E0308 with `expected u64, found usize` for the wrong
  annotation.
- "Internally `count` calls `next` repeatedly until `None`, returning
  the number of `Some(_)` returns" — `trait.Iterator.md:420-421`
  (corpus statement); Probe 6 (Trace prints four `next()` lines, then
  `count = 3`).
- "For an empty iterator, `next` is still called once and returns
  `None`; `count` returns `0`" — `trait.Iterator.md:421-422` (corpus
  statement); Probe 2 (empty `Vec<u64>` -> `0`); Probe 7 (empty
  Trace, single `next() -> None`, count = 0).
- "Counting more than `usize::MAX` elements either produces wrong
  result or panics" — `trait.Iterator.md:424-434` (corpus statement);
  not probed (impractical to construct).
- "`count` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:13` ("// Provided methods" comment precedes
  every non-`next` method including `count`); lesson 132 evidence
  appendix Probe 2 (Counter impl supplying only required surface,
  `c.count()` callable); today's Probe 6 (Trace impl ditto).
- "Default-body shape (`{ ... }`) on the synopsis-box line is what
  licenses inheritance" — `trait.Iterator.md:19-20`; lesson 116.
- "Receiver-by-value (`self`) is lesson 102's consuming shape" —
  lesson 102 evidence appendix; Probe 3's `note:` block reuses lesson
  102's template verbatim.

## Negative / contrast probe coverage

Two contrasts captured:

- **Probe 3 (E0382 on use after `.count()`)** is the centered contrast
  for the `self` receiver shape. Without this, the claim "`count`
  consumes" would be uncorroborated by a diagnostic — the lesson
  would only have the corpus prose. The E0382 + matching `note:` is
  the empirical witness that `count`'s declaration line really does
  read `fn count(self)` and not `fn count(&self)` or
  `fn count(&mut self)`.
- **Probe 5 (E0308 on `let n: u64 = ...`)** is the contrast for the
  `usize` return type. Without it, the claim "returns `usize`" would
  rest only on Probe 4's silent compile, which is consistent with
  several integer types (rustc could be inferring `usize` from
  context). E0308 with the inline label `expected u64, found usize`
  forces rustc to *name* the return type, which is what lesson 080
  needs to land the lesson-080 distinction.

The "calls `next` until `None`" claim is grounded by Probe 6 plus the
corpus prose at `trait.Iterator.md:420-421`. A negative contrast for
this claim would mean writing an iterator that violates the stated
algorithm — which is impossible empirically because the user does not
implement `count`; the trait's default body controls. Corpus-prose +
empirical-trace is the right grounding here.

## Iterator API audit alignment

This lesson is step 3 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 3. **`count`** — smallest stable consumer, names the
>    call-`next`-until-`None`-and-count semantic, names self-by-value
>    consuming. Carries `usize::MAX` overflow note.

Audit §4.3 lists `count` as ready-now, composing
"self-by-value 102 + 119". Today executes that move per audit §5
step 3.
