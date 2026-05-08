# Evidence — Lesson 135: index into a slice iterator with `iter.nth(n)`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/135-iterator-nth.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/135-iterator-nth.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/135-iterator-nth.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet135/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
134.

## Direct prerequisite — lesson 134 (`Iterator::last`)

Lesson 134 installed:

- `Iterator::last`'s signature `fn last(self) -> Option<Self::Item>
  where Self: Sized,` (`output/docs/rust/std/iter/trait.Iterator.md:448`).
- The return-slot `Option<Self::Item>` on a *provided* Iterator method,
  with `Self::Item = &u64` for slice iterators over `Vec<u64>`.
- The same E0382 + `note:` template lesson 133 captured for `count`,
  applied to `last` — only the method-name slot substitutes.

Today's lesson reuses the `Option<Self::Item>` return slot verbatim
(Probes 5-6 sandwich `Option<&u64>` for the slice iter, mirroring
lesson 134's Probes 4-5). The new fact is the *receiver* shape and
the *second parameter*: `&mut self` instead of `self`, and `n: usize`
instead of zero-arity. The call therefore does not move the iterator;
Probe 4 witnesses the binding surviving a call. There is no E0382
contrast today — that diagnostic only fires when the receiver is
`self`. Today's centered contrast is E0596 (drop `mut`), structurally
matching lesson 131's `.next()` contrast.

## Direct prerequisite — lesson 132 (the `Iterator` trait declaration)

Lesson 132 installed:

- `std::iter::Iterator` declares `type Item;` and
  `fn next(&mut self) -> Option<Self::Item>;` as required, plus 75
  provided methods (synopsis box at `trait.Iterator.md:6-13`,
  per-method elaboration further down).
- An impl supplying only the required surface inherits all 75 provided
  methods through default bodies (lesson 116).

Today's claim "`nth` is one of the 75 provided methods" maps directly:
`nth` appears on the synopsis box at `trait.Iterator.md:24` as
`fn nth(&mut self, n: usize) -> Option<Self::Item> { ... }` — the
`{ ... }` body marker is lesson 116's default-body shape. The
per-method declaration at `:507` confirms the signature without the
body marker (the corpus collapses the body into prose at `:509-547`).

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed:

- A slice iterator (`v.iter()`) yields `&T`. The wrapper for `.next()`
  is `Option<&T>`; for `Vec<u64>`, `Option<&u64>`.
- `.next()` takes `&mut self`. Without `let mut iter`, dot-calling
  `iter.next()` fires E0596 with `help:` proposing `let mut iter =
  v.iter();`.
- The cursor advances by one position per call; once exhausted, all
  subsequent calls return `None`.

Today's `nth` is structurally `next` called `n+1` times: per the
corpus prose at `trait.Iterator.md:514-517` ("all preceding elements,
as well as the returned element, will be consumed"). Today's E0596
contrast (Probe 2) is the same diagnostic shape lesson 131 captured
for `.next()` — only the method-name substitutes. Probe 4 today
witnesses cursor-advance-without-move on `nth` (binding still usable
after the first call).

## Direct prerequisite — lesson 119 (`Option<T>` / `Some` / `None`)

Lesson 119 installed `Option<T>` as a generic enum with `Some(T)` and
`None` constructors. Both variants appear empirically today:

- Probe 1 prints `Some(20)`, `Some(30)`, `Some(40)` — three `Some(_)`
  values, one per call.
- Probe 3 prints `None` for `iter.nth(100)` on a five-element iter.
- Probe 7 (std doc replay) prints `Some(20)` then `None`.

`nth`'s return slot `Option<Self::Item>` is lesson 119's `Option<T>`
with `T = Self::Item`.

## Direct prerequisite — lesson 115 (associated-type slot)

Lesson 115 installed `type IDENTIFIER;` in trait body, `Self::IDENTIFIER`
in method signatures, resolved by `type IDENTIFIER = T;` in the impl.
Today's `Self::Item` slot in `nth`'s return resolves the same way
lesson 134's did: for slice iterators over `Vec<u64>`, `Self::Item =
&u64` (lesson 131), so `iter.nth(1)` is `Option<&u64>` empirically
(Probes 5-6).

## Direct prerequisite — lesson 101 (`&mut self` receiver)

Lesson 101 installed `&mut self` as the third receiver shape — the
receiver-shorthand for `self: &mut Self`. A method declared with this
shape can write through the receiver without consuming it; the caller's
binding must be `let mut`. Lesson 131 caught the same shape on
`Iterator::next` (the *required* method).

Today is the FIRST place in the run where `&mut self` appears on a
*provided* Iterator method. Probe 2 fires E0596 ("cannot borrow `iter`
as mutable, as it is not declared as mutable") with `help:` proposing
`let mut iter = v.iter();` — the same diagnostic shape lesson 131
captured for `.next()`. The diagnostic literally proves
`trait.Iterator.md:507` reads `fn nth(&mut self, ...)`: if the receiver
were `&self`, the dot call would not need a mutable borrow.

## Direct prerequisite — lesson 006 (`let mut`)

Lesson 006 installed `let mut name = value;` — the binding-with-`mut`
form that allows mutation through the name. Today the requirement is
slightly different: `&mut self` on the method side requires that the
caller hold a `mut` binding so the auto-ref can produce a `&mut`.
Probe 2's E0596 with the `help:` proposing `let mut iter` is the
empirical witness.

## Older supporting lessons

- **Lesson 080** (cited) — twelve integer types. Today's `n: usize`
  is one specific row.
- **Lesson 123** (cited) — `v.iter()` returns the slice iterator. The
  receiver in today's working probe.
- **Lesson 116** (cited) — default-body trait methods. The synopsis-box
  line `fn nth(&mut self, n: usize) -> Option<Self::Item> { ... }`
  ends in `{ ... }` (lesson-116 default-body marker), licensing every
  iterator inheriting `nth` for free.
- **Lesson 093** (cited) — `{:?}` Debug formatter. Today's transcripts
  use `println!("{:?}", iter.nth(1))`; `Option<&u64>` Debug-prints as
  `Some(20)` / `None` (no `&` glyph for primitive-target references).
- **Lessons 040, 011, 005, 003, 002, 001** (cited) — dot-call;
  `println!`; `let`; diagnostic map; `fn main`; rustc compile + run.

## Probe 1 — working probe (multi-call cursor advance)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/135-iterator-nth.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    println!("{:?}", iter.nth(1));
    println!("{:?}", iter.nth(0));
    println!("{:?}", iter.nth(0));
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
Some(20)
Some(30)
Some(40)
$ echo "run-exit=$?"
run-exit=0
```

Centered claim — "`nth(n)` returns the `n`th element from the current
cursor and advances by `n+1` positions; multiple calls on the same
binding return *different* elements" — is carried by the three-line
output. The trace:

- Initial cursor: before `&10`. Vec is `[10, 20, 30, 40, 50]`.
- `iter.nth(1)`: advance through index 0 (`&10`, dropped) and index 1
  (`&20`, returned and dropped from the iter's POV). Cursor now before
  `&30`. Output line 1: `Some(20)`.
- `iter.nth(0)`: return the 0th element from the cursor — `&30` — and
  advance past it. Cursor now before `&40`. Output line 2: `Some(30)`.
- `iter.nth(0)`: return the 0th element from the cursor — `&40` —
  advance past it. Cursor before `&50`. Output line 3: `Some(40)`.

This carries three claims simultaneously:

1. **`&mut self` receiver does not move the iterator.** The binding
   `iter` is reused across three method calls without an E0382. (If
   the receiver were `self`, the second call would fail with E0382 —
   exactly the diagnostic lesson 133 captured for `count`.)
2. **`n` is zero-indexed from the current cursor**, not from the
   collection start. After the first call, "0th" means "next available
   element," which is `&30`, not `&10`.
3. **Each call advances the cursor by `n+1` positions** — not `n`.
   The corpus prose at `:514-517` says preceding *and* returned
   elements are consumed.

Mirrors the std doc's example shape at `trait.Iterator.md:531-540`:

```text
let a = [1, 2, 3];
let mut iter = a.into_iter();
assert_eq!(iter.nth(1), Some(2));
assert_eq!(iter.nth(1), None);
```

(The doc example uses `[i32; 3].into_iter()` rather than
`Vec<u64>.iter()`, so it yields `i32` directly — `Some(2)` without
the `&`. Today holds the slice-iter form from lesson 123 fixed and
captures the cursor-advance with three sequential calls instead of
two; the std doc example is replayed verbatim as Probe 7.)

## Probe 2 — centered contrast (drop `mut` from binding fires E0596)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let iter = v.iter();
    println!("{:?}", iter.nth(1));
}
```

Verbatim diagnostic:

```text
error[E0596]: cannot borrow `iter` as mutable, as it is not declared as mutable
 --> no_mut.rs:4:22
  |
4 |     println!("{:?}", iter.nth(1));
  |                      ^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut iter = v.iter();
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
exit=1
```

This is the centered contrast for "`nth`'s receiver is `&mut self`."
Three structural alignments to lesson 131 (which captured E0596 on
`.next()`):

1. **Same E-code.** `E0596 cannot borrow as mutable` — identical to
   lesson 131's contrast on `Iterator::next`.
2. **Same `help:` shape.** `help: consider changing this to be
   mutable` with the suggested fix `let mut iter = v.iter();` and
   `+++` marker. Identical to lesson 131's E0596 `help:`.
3. **Caret position.** The `^^^^` is under the receiver expression
   `iter`, not the method name. Same as lesson 131.

The diagnostic literally proves the trait-declaration line at
`trait.Iterator.md:507` reads `fn nth(&mut self, ...)`: if the
receiver were `self`, the call would *move* the binding (no E0596 —
would silent-compile but invalidate `iter` for future use); if `&self`,
the binding could remain immutable. Receiver-`&mut self` is the only
shape that produces this exact E0596.

This contrast also distinguishes today from lessons 133/134 (which
captured E0382 on `self`-by-value): the *kind* of contrast that fires
is itself diagnostic of the receiver shape.

## Probe 3 — past-end corroboration (nth past end yields None, no panic)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    println!("{:?}", iter.nth(100));
}
```

Compile silent; run prints `None`; exit 0. Witnesses: `nth(n)` for
`n >= remaining length` returns `None` and does *not* panic. Matches
the corpus prose at `trait.Iterator.md:519-520`: "`nth()` will return
`None` if `n` is greater than or equal to the length of the iterator."

This is the operational corollary of the cursor-advance algorithm: if
the iterator runs out of elements before `n+1` `next` calls have been
made, the bookkeeping returns the `None` from the last `next` call.

## Probe 4 — corroborating witness (binding still usable after `nth`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    let _ = iter.nth(1);
    println!("{:?}", iter.nth(0));
}
```

Compile silent; run prints `Some(30)`; exit 0. Witnesses: `iter` is
*not* moved by the first `nth` call. After `iter.nth(1)` the cursor
is past `&20`; `iter.nth(0)` then returns the next element, `&30`.
Compare lesson 133's Probe 3 (`iter.count(); iter.count();`) which
fires E0382 — the contrast in receiver shape (`self` vs `&mut self`)
shows up as a diagnostic difference.

This probe is the empirical companion to the multi-call cursor
advance shown in Probe 1: where Probe 1 demonstrates three `nth`
calls in expression position, Probe 4 demonstrates the binding
crossing a statement-boundary `let _ = ...;` and remaining live for
a subsequent `nth` call. Both witness `&mut self` non-moving.

## Probe 5 — type-pin positive (`Option<&u64>` for slice iter over `Vec<u64>`)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    let x: Option<&u64> = iter.nth(1);
    println!("{:?}", x);
}
```

Compile silent; run prints `Some(20)`. Witnesses: `iter.nth(1)`
empirically has type `Option<&u64>` on this host. Matches
`trait.Iterator.md:507`'s `fn nth(..) -> Option<Self::Item>` with
`Self::Item = &u64` for the slice iterator (lessons 132 + 131).

## Probe 6 — type-pin contrast (`Option<u64>` fires E0308)

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    let x: Option<u64> = iter.nth(1);
    println!("{:?}", x);
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typetest_neg.rs:4:26
  |
4 |     let x: Option<u64> = iter.nth(1);
  |            -----------   ^^^^^^^^^^^ expected `Option<u64>`, found `Option<&u64>`
  |            |
  |            expected due to this
  |
  = note: expected enum `Option<_>`
             found enum `Option<&_>`
help: use `Option::copied` to copy the value inside the `Option`
  |
4 |     let x: Option<u64> = iter.nth(1).copied();
  |                                     +++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Sandwiches Probe 5: `Option<&u64>` compiles silently; `Option<u64>`
does not. The inline label `expected Option<u64>, found Option<&u64>`
and the secondary note `expected enum Option<_>; found enum Option<&_>`
pin the substitution `Self::Item = &u64` from two depths. Same shape
as lesson 134's Probe 5 — only the method-name substitutes and the
caret moves to today's `iter.nth(1)` expression.

The `help:` proposes `.copied()` — `Option::copied`, named-deferred
today; same disposition as lesson 134's `help:`.

## Probe 7 — std doc example replay

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    println!("{:?}", iter.nth(1));
    println!("{:?}", iter.nth(1));
}
```

Compile silent; run prints:

```text
Some(20)
None
```

Witnesses the std doc example at `trait.Iterator.md:533-540` verbatim,
modulo `i32` vs `&u64` rendering: the doc has `iter.nth(1) == Some(2)`
then `iter.nth(1) == None` for `[1, 2, 3]`. Today's `[10, 20, 30]`
yields `Some(20)` then `None`.

The trace explains the second `None`: after the first `nth(1)` the
cursor is past `&20`, leaving only `&30` in the iterator. The second
`nth(1)` would need to advance two positions; only one element
remains; the algorithm's internal `next` returns `None` before
reaching index 1. Hence `None`.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` line 24 (synopsis-box line)

Verbatim:

```
    fn nth(&mut self, n: usize) -> Option<Self::Item> { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape — what
licenses every iterator inheriting `nth` for free. Note: unlike `count`
(`:19-20`) and `last` (`:21-22`), the line for `nth` has *no*
`where Self: Sized` clause.

### `output/docs/rust/std/iter/trait.Iterator.md` line 507 (per-method declaration)

Verbatim:

```
#### fn [nth](#method.nth)(&mut self, n: [usize](../primitive.usize.md)) -> [Option](../option/enum.Option.md "enum std::option::Option")<Self::[Item](trait.Iterator.md#associatedtype.Item "type std::iter::Iterator::Item")>
```

This is the authoritative source for:

- the **method name** `nth` and the **receiver shape** `(&mut self, n:
  usize)` — `&mut self` (lesson 101, the third receiver shape) followed
  by a second parameter `n: usize` (lesson 080).
- the **return type** `Option<Self::Item>` — lesson 119 wrapper with
  `T = Self::Item` (lesson 132/115). For the slice iterator (lesson
  131), `Self::Item = &u64`, so `iter.nth(1)` is `Option<&u64>`.
- the **absence** of `where Self: Sized` — meaningful contrast to
  `count` and `last`. (`Sized` is named-deferred either way today.)

### `output/docs/rust/std/iter/trait.Iterator.md` lines 509-520 (prose summary)

Verbatim:

```
Returns the `n`th element of the iterator.

Like most indexing operations, the count starts from zero, so `nth(0)`
returns the first value, `nth(1)` the second, and so on.

Note that all preceding elements, as well as the returned element, will be
consumed from the iterator. That means that the preceding elements will be
discarded, and also that calling `nth(0)` multiple times on the same iterator
will return different elements.

`nth()` will return [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None") if `n` is greater than or equal to the length of the
iterator.
```

This grounds:

- "**Zero-indexed** from current cursor; `nth(0)` returns the first
  available element" — Probe 1's first line `Some(20)` (after a no-op
  `nth(0)` would return `&10`; here we did `nth(1)` first which
  returned `&20`, but the principle is identical).
- "**Preceding elements will be discarded**" — Probe 1 line 1 outputs
  `Some(20)` not `Some(10)`; `&10` was consumed and dropped.
- "Calling `nth(0)` multiple times on the same iterator will return
  different elements" — Probe 1 lines 2-3 (`Some(30)` then `Some(40)`).
- "`nth()` will return `None` if `n` is greater than or equal to the
  length" — Probe 3 (`nth(100)` on five-element iter prints `None`)
  and Probe 7 (second `nth(1)` on the now-1-element-remaining iter
  prints `None`).

### `output/docs/rust/std/iter/trait.Iterator.md` lines 522-547 (examples)

Verbatim:

```
##### Examples

Basic usage:

```
let a = [1, 2, 3];
assert_eq!(a.into_iter().nth(1), Some(2));
```

Calling `nth()` multiple times doesn't rewind the iterator:

```
let a = [1, 2, 3];

let mut iter = a.into_iter();

assert_eq!(iter.nth(1), Some(2));
assert_eq!(iter.nth(1), None);
```

Returning `None` if there are less than `n + 1` elements:

```
let a = [1, 2, 3];
assert_eq!(a.into_iter().nth(10), None);
```
```

Today's Probe 1 (working probe, three sequential `nth` calls) extends
the doc's second example shape (two sequential calls) to three calls
to make the cursor-advance more visible. Today's Probe 7 replays the
doc's second example exactly. Today's Probe 3 mirrors the doc's third
example with `nth(100)` instead of `nth(10)`.

The doc uses `[i32; 3].into_iter()` rather than `Vec<u64>.iter()`,
which yields `i32` directly — `Some(2)` without an `&`. Today uses
the slice-iter form from lesson 123, yielding `&u64` — hence
`Some(20)` and the `&` flowing through Probe 6's E0308.

### `output/docs/rust/error_codes/E0596.md`

Probe 2's diagnostic. The error code documents the "cannot borrow as
mutable" rule. Lesson 131 already installed the diagnostic shape on
`.next()`; today's contrast reuses it on the first *provided* method
to take `&mut self`.

### `output/docs/rust/error_codes/E0308.md`

Probe 6's diagnostic. The error code documents the "mismatched types"
rule. Lessons 134 and 133 already installed the diagnostic shape with
`expected/found` inline labels on `Option<_>` return wrappers; today's
contrast carries it onto `Iterator::nth`'s `Option<Self::Item>` slot.

## Claim-to-evidence map

- "`iter.nth(1)`, `iter.nth(0)`, `iter.nth(0)` on `vec![10, 20, 30,
  40, 50]` prints `Some(20) Some(30) Some(40)`" — Probe 1.
- "`nth` takes the receiver by mutable borrow (`&mut self`); calling
  it does *not* move the iterator and the binding is reusable" —
  `trait.Iterator.md:507` (signature); Probe 1 (three calls on same
  binding); Probe 4 (binding survives `let _ = iter.nth(1);` then
  used again).
- "Without `let mut iter`, dot-calling `nth` fires E0596" —
  `trait.Iterator.md:507` (`&mut self`); Probe 2 (verbatim
  diagnostic); E0596 corpus.
- "`n: usize` is the second parameter" — `trait.Iterator.md:507`
  (signature).
- "Return type is `Option<Self::Item>`" — `trait.Iterator.md:507`
  (signature); `:24` (synopsis line).
- "For the slice iterator over `Vec<u64>`, `Self::Item = &u64`, so
  `iter.nth(1)` is `Option<&u64>`" — lesson 131 (the `&u64`
  identification); Probe 5 silent compile; Probe 6 E0308 with
  `expected Option<u64>, found Option<&u64>`.
- "Each call advances the cursor by `n+1` positions; `nth(0)` twice
  returns different elements" — `trait.Iterator.md:514-517` (corpus
  statement); Probe 1 (lines 2-3 are `Some(30)` and `Some(40)`, not
  `Some(30)` twice); Probe 4 (cursor-advance across statements).
- "Past-end yields `None`, no panic" — `trait.Iterator.md:519-520`
  (corpus statement); Probe 3 (`nth(100)` on five-element iter →
  `None`); Probe 7 (second `nth(1)` on now-1-element-remaining iter
  → `None`).
- "`nth` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:13` ("// Provided methods" comment precedes
  `nth`); `:24` (synopsis line ends in `{ ... }` — the lesson-116
  default-body marker); lesson 132 evidence appendix.
- "No `Self: Sized` bound on `nth`'s signature (unlike `count`,
  `last`)" — `trait.Iterator.md:507` and `:24` (no `where` clause).
  Meaningful structural contrast with lesson 133/134 signatures.
- "`&mut self` is the third receiver shape, lesson 101" — lesson 101
  evidence; lesson 131 evidence (E0596 contrast on the *required*
  `next`); Probe 2 today (E0596 contrast on the first *provided*
  method to take this shape).

## Negative / contrast probe coverage

Three contrasts captured:

- **Probe 2 (E0596 on dropped `mut`)** is the centered contrast for
  the `&mut self` receiver shape. Without this, the claim "`nth`
  requires `let mut iter`" would rest only on corpus prose. The E0596
  + matching `help:` is the empirical witness that the trait-declaration
  line at `:507` really does read `fn nth(&mut self, ...)` and not
  `fn nth(self, ...)` or `fn nth(&self, ...)`.
- **Probe 6 (E0308 on `Option<u64>` instead of `Option<&u64>`)** is
  the type-pin contrast for the `Self::Item = &u64` substitution.
  Without it, the claim "returns `Option<&u64>`" would rest only on
  Probe 5's silent compile, which is consistent with several inferred
  annotations. E0308's inline `expected Option<u64>, found Option<&u64>`
  pins the actual return type with the `&` glyph visible.
- **Probe 4 (binding survives a call)** is the corroborating contrast
  *against* lesson 133/134's E0382 pattern: where E0382 fires for
  `self`-by-value receivers reused after a method call, today's
  binding does *not* fire E0382 for the same reuse pattern. This is
  the operational difference between `self` and `&mut self` made
  empirically visible.

The "default body of `nth` in core" claim (named in *What To Ignore*)
is grounded only by structural inference (synopsis-line `{ ... }` per
lesson 116). The actual core source uses an internal `for`-loop over
`self.next()` — readable once the `for` desugar lands. Not probed
today; lesson 132's Probe 2 already grounded the inheritance mechanic.

The "panic if infinite" claim *does not apply* to `nth`. Lesson 134
inherited this concern from `last`; `nth` has a fixed-`n` exit
condition (return `Some(_)` once `n+1` `next` calls have happened
*or* `None` once `next` returns `None`). For an infinite iterator,
`nth(n)` terminates after `n+1` `next` calls — no panic. The corpus
is silent on a `nth`-specific panic note, consistent with this.

## Iterator API audit alignment

This lesson is step 5 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 5. **`nth`** — `&mut self` consumer, drops preceding elements.

Audit §4.3 lists `nth` as ready-now, composing
"`&mut self` 101 + 119 + 080". Today executes that move per audit §5
step 5. Lesson 134's unlock list explicitly named today's move:
*"future `Iterator::nth` — `(&mut self, n: usize) -> Option<Self::Item>`,
drops preceding elements" moves (audit §5 step 5 — reuses today's
`Option<Self::Item>` return-slot rule but with `&mut self` instead of
`self`; first contrast against today's self-by-value)*.

The new graph fact today extends 134: where 134's `last` reused
self-by-value (lesson 102's consuming receiver), today's `nth` is the
first *provided* Iterator method whose receiver is `&mut self` — the
same shape lesson 131 caught on the *required* `next`. The split
established today between consuming (`count`, `last`) and
mutably-borrowing (`nth`) provided methods is the operational frame
for every later Iterator method choice in §2's table — opening the
way for `take`, `skip`, `enumerate`, `fuse`, `step_by`, `size_hint`
(audit §5 steps 6-11), each of which the audience will read against
today's "which receiver does this method take?" question.
