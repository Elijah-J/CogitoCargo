# Evidence — Lesson 134: pull the final element of a slice iterator with `.last()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/134-iterator-last.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/134-iterator-last.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/134-iterator-last.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet134/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
133.

## Direct prerequisite — lesson 133 (`Iterator::count`)

Lesson 133 installed:

- `Iterator::count`'s signature `fn count(self) -> usize where Self:
  Sized,` (`output/docs/rust/std/iter/trait.Iterator.md:416`). `self`
  is lesson-102's consuming receiver; `usize` is the return type.
- The defining walker: call `next` repeatedly until `None`, returning
  the number of `Some(_)`s. Witnessed empirically by the side-effect
  `Trace` impl in lesson 133 Probe 6.
- `count` is one of the 75 provided methods of the `Iterator` trait
  inherited via default bodies (lesson 116). The synopsis-box line
  ends in `{ ... }` (`trait.Iterator.md:19-20`).
- Centered E0382 contrast pattern when the iterator is reused after
  `.count()`: the `note:` block at the method-definition site reads
  ``count` takes ownership of the receiver `self`, which moves `iter``,
  with the `-->` pointing at core's
  `library/core/src/iter/traits/iterator.rs:225:13`.

Today reuses every structural piece. The signature swap is `fn last(self)
-> Option<Self::Item> where Self: Sized,` (`trait.Iterator.md:448`).
`self` and `where Self: Sized` repeat verbatim. The walker is the same
template, only the bookkeeping differs: 133 tallies, 134 remembers.
Probe 3 today produces an E0382 *isomorphic* to 133's: same E-code,
same body, same `note:` template — only `count` is substituted with
`last`, and the `-->` of the note points at `iter/traits/iterator.rs:258:12`
instead of `:225:13` (different line in the same core file because each
provided method has its own definition site).

## Direct prerequisite — lesson 132 (the `Iterator` trait declaration)

Lesson 132 installed:

- `std::iter::Iterator` declares `type Item;` and
  `fn next(&mut self) -> Option<Self::Item>;` as required, plus 75
  provided methods.
- `Self::Item` is the lesson-115 associated-type slot, resolved per
  impl. For slice iterators (lesson 131), `Self::Item = &u64`.
- An impl supplying only the required surface inherits all 75 provided
  methods. `last` is among them (synopsis box at `trait.Iterator.md:21-22`,
  per-method section at `:448`).

Today's claim "`last` is one of the 75" maps directly. Today's centered
new fact — `last`'s return is `Option<Self::Item>`, the first *provided*
method whose return type is anchored to the associated-type slot —
extends 132 by exercising the `Self::Item` path through a *return* slot
rather than through `next`'s required-method signature. Probes 4 + 5
sandwich the substitution `Self::Item = &u64` empirically.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed:

- A slice iterator (`v.iter()`) yields `&T`. The wrapper for `.next()`
  is `Option<&T>`; for `Vec<u64>`, `Option<&u64>`.
- `Self::Item = &u64` for the slice iterator over `Vec<u64>` — this
  identification is what today's lesson resolves through `last`'s
  return-type slot.

Today's Probes 4 + 5 are the centered witness for this. Probe 4 silent-
compiles `let x: Option<&u64> = v.iter().last();`; Probe 5 fires E0308
with `expected Option<u64>, found Option<&u64>` for the wrong
annotation. Together they pin the return type at `Option<&u64>` from
two sides — same kind of sandwich lesson 131 used for `iter.next()`'s
return.

## Direct prerequisite — lesson 119 (`Option<T>` / `Some` / `None`)

Lesson 119 installed `Option<T>` as a generic enum with `Some(T)` and
`None` constructors. Both variants appear empirically today:

- Probe 1 prints `Some(30)` for the three-element vec — the `Some(_)`
  constructor wrapping the *last* element seen.
- Probe 2 prints `None` for the empty vec — the lesson-119 bare
  variant.

`last`'s return slot `Option<Self::Item>` is lesson 119's `Option<T>`
with `T` substituted by the trait's associated-type slot.

## Direct prerequisite — lesson 102 (self-by-value receiver)

Lesson 102 installed `self` (no `&`, no `mut`) as the consuming
receiver shape. Lesson 133 fired this rule on the *stdlib* method
`Iterator::count` — the first stdlib instance. Today fires the same
rule on the second stdlib method, `Iterator::last`. Probe 3's E0382
reuses lesson 133's diagnostic shape verbatim except for the method-
name slot.

## Direct prerequisite — lesson 115 (associated-type slot)

Lesson 115 installed `type IDENTIFIER;` in trait body, `Self::IDENTIFIER`
in method signatures, resolved by `type IDENTIFIER = T;` in the impl.
Today is the FIRST place in the run where a `Self::Item` slot appears
in the *return* type of a *provided* (default-body) Iterator method
that the audience *calls*. Lesson 132 named this signature shape on
the synopsis box; lesson 131 caught `Self::Item = &u64` through `next`'s
*required* signature. Today's Probes 4 + 5 show that the same
substitution flows through any provided method whose return wrapper
mentions `Self::Item`.

## Older supporting lessons

- **Lesson 123** (cited) — `v.iter()` returns the slice iterator. The
  receiver in today's working probe.
- **Lesson 116** (cited) — default-body trait methods. The synopsis-box
  line `fn last(self) -> Option<Self::Item> where Self: Sized { ... }`
  ends in `{ ... }` (lesson-116 default-body marker), licensing every
  iterator inheriting `last` for free.
- **Lesson 093** (cited) — `{:?}` Debug formatter. Today's transcripts
  use `println!("{:?}", result)`; `Option<&u64>` Debug-prints as
  `Some(30)` / `None` (no `&` glyph for primitive-target references —
  same convention lesson 131 captured).
- **Lessons 040, 011, 005, 003, 002, 001** (cited) — dot-call;
  `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.

## Probe 1 — working probe (last on a three-element slice iterator)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/134-iterator-last.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let result = v.iter().last();
    println!("{:?}", result);
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
Some(30)
$ echo "run-exit=$?"
run-exit=0
```

Centered claim — "`v.iter().last()` returns the last element wrapped
in `Some(_)`, printable with `{:?}`" — is carried by the silent compile
plus the printed `Some(30)`. Mirrors the std doc's example shape at
`trait.Iterator.md:462-467`:

```text
let a = [1, 2, 3];
assert_eq!(a.into_iter().last(), Some(3));
```

(The doc example uses `[i32; 3].into_iter()` rather than `Vec<u64>.iter()`;
today holds the slice-iter form from lesson 123 fixed, swapping
`.count()` for `.last()` against lesson 133's Probe 1.)

## Probe 2 — empty-vec corroboration (last on empty vec yields None)

```rust
fn main() {
    let v: Vec<u64> = vec![];
    let result = v.iter().last();
    println!("{:?}", result);
}
```

Compile silent; run prints `None`. Witnesses: an empty iterator yields
`None` from `last()` — does not panic, does not return a sentinel
`Some(_)`. Matches the corpus prose at `trait.Iterator.md:450-454`:
"keeps track of the current element. After `None` is returned, `last()`
will then return the last element it saw." For an empty iterator, no
`Some(_)` is ever observed; the tracker holds nothing; `last()` returns
`None`.

This probe also empirically witnesses both variants of `Option<&u64>`:
Probe 1 produced `Some(30)`, Probe 2 produces `None`. Together they
confirm the return wrapper is an `Option<_>` with both 119 variants
reachable.

## Probe 3 — centered contrast (use after `.last()` fires E0382)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    let _ = iter.last();
    let _ = iter.last();
}
```

Verbatim diagnostic:

```text
error[E0382]: use of moved value: `iter`
 --> use_after.rs:5:13
  |
3 |     let iter = v.iter();
  |         ---- move occurs because `iter` has type `std::slice::Iter<'_, u64>`, which does not implement the `Copy` trait
4 |     let _ = iter.last();
  |                  ------ `iter` moved due to this method call
5 |     let _ = iter.last();
  |             ^^^^ value used here after move
  |
note: `last` takes ownership of the receiver `self`, which moves `iter`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/iter/traits/iterator.rs:258:12
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
4 |     let _ = iter.clone().last();
  |                 ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
exit=1
```

This is the centered contrast for "`last`'s receiver is consuming."
Three structural alignments to lesson 133:

1. **Same E-code.** `E0382 use of moved value` — identical to lesson
   133's contrast on `Iterator::count`.
2. **Same `note:` shape.** `note: \`last\` takes ownership of the
   receiver \`self\`, which moves \`iter\`` — same template as lesson
   133's `note: \`count\` takes ownership of the receiver \`self\`,
   which moves \`iter\``. Only the method-name substitutes.
3. **`-->` of the note points at the method definition in core.**
   Lesson 133's note pointed at
   `library/core/src/iter/traits/iterator.rs:225:13`; today's points
   at `:258:12` — different line in the same core file (each provided
   method has its own definition site). The path is the load-bearing
   fact; the line number is internal.

The `help:` line proposes `iter.clone().last()` — same arc lesson 133
flagged; still deferred today.

The diagnostic literally proves the trait-declaration line at
`trait.Iterator.md:448` reads `fn last(self) -> Option<Self::Item>`:
if the receiver were `&self` or `&mut self`, the second call would
not move-fail. Receiver-by-value is the only shape that produces this
exact E0382.

## Probe 4 — type-pin (return type is `Option<&u64>` for slice iter over `Vec<u64>`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let x: Option<&u64> = v.iter().last();
    println!("{:?}", x);
}
```

Compile silent; run prints `Some(30)`. Witnesses: `v.iter().last()`
empirically has type `Option<&u64>` on this host. Matches
`trait.Iterator.md:448`'s `fn last(self) -> Option<Self::Item>` with
`Self::Item = &u64` for the slice iterator (lessons 132 + 131).

This is the FIRST place in the run where a *provided* Iterator method's
return type is anchored to the lesson-115 / 132 associated-type slot.
Lesson 133's `count` returned a primitive `usize` directly — no
`Self::Item` slot in its return signature.

## Probe 5 — type-pin contrast (`Option<u64>` instead of `Option<&u64>` fires E0308)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let x: Option<u64> = v.iter().last();
    println!("{:?}", x);
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typetest_neg.rs:3:26
  |
3 |     let x: Option<u64> = v.iter().last();
  |            -----------   ^^^^^^^^^^^^^^^ expected `Option<u64>`, found `Option<&u64>`
  |            |
  |            expected due to this
  |
  = note: expected enum `Option<_>`
             found enum `Option<&_>`
help: use `Option::copied` to copy the value inside the `Option`
  |
3 |     let x: Option<u64> = v.iter().last().copied();
  |                                         +++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Sandwiches Probe 4: `Option<&u64>` compiles silently; `Option<u64>` does
not. The inline label `expected Option<u64>, found Option<&u64>` and
the secondary note `expected enum Option<_>; found enum Option<&_>`
make the difference explicit at two depths — wrapper-level and
inner-level. Lesson 131's claim "for the slice iterator,
`Self::Item = &u64`" is what this contrast trips, only now resolved
through `Iterator::last`'s `Option<Self::Item>` return slot rather
than `Iterator::next`'s.

The `help:` line proposes `.copied()` — `Option::copied` and
`Iterator::copied`, both not yet installed. Named-deferred today; same
disposition as lesson 133's `help:` proposing `.clone()`.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` line 448 (per-method declaration)

Verbatim:

```
#### fn [last](#method.last)(self) -> [Option](../option/enum.Option.md "enum std::option::Option")<Self::[Item](trait.Iterator.md#associatedtype.Item "type std::iter::Iterator::Item")> where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

This is the authoritative source for:

- the **method name** `last` and the **receiver shape** `(self)` — bare
  `self`, lesson 102's consuming receiver. Same shape as `count` at
  `:416`.
- the **return type** `Option<Self::Item>` — the lesson 119 wrapper
  with `T = Self::Item` (lesson 132/115 associated-type slot). For the
  slice iterator (lesson 131), `Self::Item = &u64`.
- the **bound** `where Self: Sized` — named-deferred today (same
  disposition as lesson 133).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 21-22 (synopsis-box line)

Verbatim:

```
    fn last(self) -> Option<Self::Item>
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape. Same
content as line 448, but on the trait-declaration synopsis at the top
of the page rather than the per-method section. Identical structure
to `count` at `:19-20`.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 450-454 (prose summary)

Verbatim:

```
Consumes the iterator, returning the last element.

This method will evaluate the iterator until it returns `None`. While
doing so, it keeps track of the current element. After `None` is
returned, `last()` will then return the last element it saw.
```

This grounds:

- "**Consumes** the iterator" — the receiver-by-value behavior witnessed
  empirically by Probe 3's E0382.
- "Evaluate the iterator until it returns `None`" — same template as
  `count`'s walker. Not separately probed today; relies on lesson 131's
  `next` semantics + lesson 133's empirical witness for the same
  template.
- "Keeps track of the current element. After `None` is returned,
  `last()` will then return the last element it saw" — covers both
  the non-empty case (Probe 1: tracker holds the most recent `Some(_)`,
  returned wrapped) and the empty case (Probe 2: tracker never sees a
  `Some(_)`, returns `None`).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 456-458 (Panics note)

Verbatim:

```
##### Panics

This function might panic if the iterator is infinite.
```

Today's lesson names this as a corpus fact in *The Move* and *What
Changed* and does not probe it. Constructing a truly infinite iterator
(`(0u64..).last()`) would loop forever rather than terminate within a
realistic build; the Panics note is hedged ("might"). The corpus
statement is sufficient grounding; an empirical witness would either
hang the build or require a contrived overflow scenario.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 460-468 (example)

Verbatim:

```
##### Examples

```
let a = [1, 2, 3];
assert_eq!(a.into_iter().last(), Some(3));

let a = [1, 2, 3, 4, 5];
assert_eq!(a.into_iter().last(), Some(5));
```
```

Today's working probe (Probe 1) mirrors the first example structurally
— `[1, 2, 3]` becomes `vec![10, 20, 30]`, `assert_eq!(_, Some(3))`
becomes `println!("{:?}", result)` printing `Some(30)`. The doc uses
`.into_iter()` on an array (yields `i32` directly, hence `Some(3)`
without an `&`); today uses `.iter()` on a `Vec` (yields `&u64`,
hence `Option<&u64>` with the `&` flowing through Probe 5's E0308).
The semantic is identical; only the iterator-source spelling differs.

### `output/docs/rust/error_codes/E0382.md`

Probe 3's diagnostic. The error code documents the "use of moved
value" rule. Lessons 102 + 133 already installed the diagnostic shape;
today's contrast reuses it on the second stdlib method to fire it.

### `output/docs/rust/error_codes/E0308.md`

Probe 5's diagnostic. The error code documents the "mismatched types"
rule. Lessons 019 + 133 already installed the diagnostic shape with
`expected/found` inline labels; today's contrast carries it onto a
generic-wrapped return type (`Option<_>` with mismatched inner).

## Claim-to-evidence map

- "`v.iter().last()` returns `Some(30)` for `vec![10, 20, 30]`" —
  Probe 1 (silent compile + `Some(30)` printed); `trait.Iterator.md:448`
  (signature); `:462-467` (matching std example).
- "`last` takes the receiver by value (`self`, no `&`); calling it
  moves the iterator and the binding cannot be used again" —
  `trait.Iterator.md:448` (`fn last(self) -> ...`); Probe 3's E0382
  (`note: \`last\` takes ownership of the receiver \`self\`, which
  moves \`iter\``).
- "`last`'s return type is `Option<Self::Item>`" —
  `trait.Iterator.md:448` (signature); `:21-22` (synopsis line).
- "For the slice iterator over `Vec<u64>`, `Self::Item = &u64`, so
  `v.iter().last()` is `Option<&u64>`" — lesson 131 (the `&u64`
  identification); Probe 4 silent compile of `let x: Option<&u64> =
  ...`; Probe 5 E0308 with `expected Option<u64>, found Option<&u64>`.
- "Internally `last` walks `next` until `None`, remembering the most
  recent `Some(_)`" — `trait.Iterator.md:450-454` (corpus statement);
  consistent with Probe 1 (`Some(30)`, the last-seen) and Probe 2
  (`None`, no element seen).
- "Empty iterator yields `None`" — `trait.Iterator.md:450-454` (corpus
  prose); Probe 2 (empty `Vec<u64>` -> `None`).
- "`last` might panic if the iterator is infinite" —
  `trait.Iterator.md:456-458` (corpus statement); not probed
  (impractical to construct a true infinite iterator without the build
  hanging).
- "`last` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:13` ("// Provided methods" comment precedes every
  non-`next` method including `last`); lesson 132 evidence appendix;
  the synopsis-box line `:21-22` ends in `{ ... }` (lesson-116
  default-body marker).
- "Default-body shape (`{ ... }`) on the synopsis-box line is what
  licenses inheritance" — `trait.Iterator.md:21-22`; lesson 116.
- "Receiver-by-value (`self`) is lesson 102's consuming shape; same
  E0382 + `note:` template lesson 133 captured for `count`" — lesson
  102 evidence appendix; lesson 133 evidence appendix; Probe 3 today.

## Negative / contrast probe coverage

Two contrasts captured, parallel to lesson 133's structure:

- **Probe 3 (E0382 on use after `.last()`)** is the centered contrast
  for the `self` receiver shape. Without this, the claim "`last`
  consumes" would rest only on corpus prose. The E0382 + matching
  `note:` is the empirical witness that `last`'s declaration line
  really does read `fn last(self)` and not `fn last(&self)` or
  `fn last(&mut self)`.
- **Probe 5 (E0308 on `let x: Option<u64> = ...`)** is the centered
  contrast for the `Option<Self::Item>` return wrapper *with*
  `Self::Item = &u64` for the slice iter. Without it, the claim
  "returns `Option<&u64>`" would rest only on Probe 4's silent compile,
  which is consistent with several inferred annotations. E0308 with
  the inline label `expected Option<u64>, found Option<&u64>` forces
  rustc to *name* the return type with the `&` glyph visible — the
  audience needs this to land the `Self::Item = &u64` substitution
  from lesson 131 + 132.

The "panic if infinite" claim is grounded only by corpus prose
(`trait.Iterator.md:456-458`). A negative empirical witness would
require constructing an iterator that runs forever (e.g.
`std::iter::repeat(7u64).last()`), which would hang the build or
require interrupting the process. The corpus statement is the right
grounding here; explicitly named in the appendix as a deliberate
non-probe.

The "walks `next` until `None`, remembering most recent `Some(_)`"
claim is grounded by corpus prose at `:450-454` plus lesson 133's
identical-template empirical witness (Probe 6 of 133's side-effect
`Trace`). A separate side-effect Trace probe today would re-run that
same evidence with `last` substituted for `count` and would primarily
witness an additional bookkeeping detail (the tracker variable);
lesson 133's Probe 6 is sufficient to ground the shared template, and
today's Probe 1 + Probe 2 sandwich (the actual `Some(30)` / `None`
output values) ground the bookkeeping difference.

## Iterator API audit alignment

This lesson is step 4 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 4. **`last`** — small consumer, names the infinite-iterator panic
>    trigger.

Audit §4.3 lists `last` as ready-now, composing
"self-by-value 102 + 119". Today executes that move per audit §5
step 4. Lesson 133's unlock list explicitly named today's move:
*"future `Iterator::last` — `(self) -> Option<Self::Item>`, smallest
consumer returning the last element; carries the infinite-iterator
panic trigger" moves (audit §5 step 4 — reuses today's self-by-value
consuming rule plus the `Self::Item` link from 132 + 115)*.

The new graph fact today extends 133: where 133's `count` had a
primitive return (`usize`), today's `last` is the first *provided*
Iterator method whose return is anchored to the `Self::Item` slot —
opening the way for `nth` (next audit step), `find`, `max`, `min`,
and every other consumer whose return wrapper mentions `Self::Item`.
