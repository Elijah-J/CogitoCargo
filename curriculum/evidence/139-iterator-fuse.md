# Evidence — Lesson 139: make `None` sticky on any iterator with `iter.fuse()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/139-iterator-fuse.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/139-iterator-fuse.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/139-iterator-fuse.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet139/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
138.

## Direct prerequisite — lesson 138 (`Iterator::enumerate`)

Lesson 138 installed the `(self) -> Wrapper<Self>` adapter shape with
no second parameter. Today's signature `fn fuse(self) -> Fuse<Self>
where Self: Sized,` is structurally the same modulo the wrapper-type
substitution (`Fuse` for `Enumerate`). What 138 carried unchanged
into today:

- The bare-`self` consuming receiver.
- The "no extra argument" trait surface (cf. 136/137 which carried
  `n: usize`).
- The "wrapper struct that itself implements `Iterator`" claim.

The new fact today is *behavioral*: where 138's `Enumerate<I>`
rewrote the yielded element type to `(usize, I::Item)`, today's
`Fuse<I>` does *not* rewrite the element type or count. It enforces a
*post-`None` behavioral guarantee*. Probe 1 witnesses this — element
shape `Option<u32>` is unchanged on both halves; what changes is what
calls 3-6 return after the first `None`.

## Direct prerequisite — lesson 132 (`Iterator` trait declaration)

Lesson 132 installed:

- `pub trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; /* + 75 provided */ }`.
- The user-defined-iterator pattern — implement only the required
  surface and inherit the rest via 116's default-body mechanic.

Today's `Stutter` reuses 132's `Counter`-style impl verbatim modulo
the body of `next`:

- `struct Stutter { n: u32 }` (single named field, lesson 095).
- `impl Iterator for Stutter { type Item = u32; fn next(...) -> ... { ... } }`.
- Returns `Some(_)` on even `n`, `None` on odd `n` — matching the
  std-doc `Alternate` example at `trait.Iterator.md:1753-1767`,
  modulo our slightly-different state-encoding choice (we yield
  `Some(n/2)` on even calls; the std example yields `Some(state)` and
  uses `(val % 2 == 0).then_some(val)`).

The 75 provided methods — including `fuse` — are inherited
automatically: `Stutter { n: 0 }.fuse()` compiles without any extra
impl on `Stutter`.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131's *What To Ignore For Now* explicitly named "iterators that
resume after `None` (`Fuse<I>`)" as deferred. It also mentioned (in
its discussion of slice iterators) that "once `None` is returned every
subsequent call returns `None` — exhausted means `None` forever." That
sticky-`None` claim was *true for slice iterators*, but lesson 131
implicitly conflated "slice iterators stick" with "all iterators
stick." Today untangles them:

- The bare `Iterator::next` contract at `trait.Iterator.md:281-284`
  permits resuming.
- Slice iterators stick because they implement the `FusedIterator`
  marker trait (per `trait.FusedIterator.md` declaration; the
  implementor list at `:23+` includes the slice iterator family).
- For iterators that don't implement `FusedIterator`, `.fuse()` is
  the explicit wrapper.

Probe 1 left half (`Stutter` alone) is the first probe in the run
that witnesses an iterator *resuming* after `None` — the exact
behavior 131 implicitly excluded.

## Direct prerequisite — lessons 137, 136 (lazy-adapter family)

Same lazy-adapter family today reuses:

- Consuming `self` receiver.
- Wrapper struct that itself implements `Iterator`.
- Method-chain composition (`Stutter { n: 0 }.fuse()` parses as
  `((Stutter { n: 0 }).fuse())` per lesson 049).

Today's lesson does not re-witness the consuming-`self` rule via
E0382 — well-installed by lessons 102/133/134/136/137/138 (six prior
captures of the same template). The signature at
`trait.Iterator.md:1737` reads `self` (no `&`, no `mut`); the rule
applies.

The lazy framing is *technically* not load-bearing today — `fuse()`
in this lesson is treated structurally (signature + post-`None`
behavior) rather than for laziness specifically. The Probe 1 left
half *also* does not exercise laziness (calls run immediately). The
lazy-adapter framing inherits from 136 and remains accurate, but is
not the centered fact today. Lesson 136's Probe 2 Trace+take captured
the laziness shape for the family.

## Older supporting lessons

- **Lesson 102** (cited) — bare-`self` consuming receiver shape; not
  re-witnessed via E0382 today.
- **Lesson 119** (cited) — `Option<T>`. Today's `T = u32` (Probe 1)
  and `T = &u64` (Probe 2 / Probe 3).
- **Lesson 080** (cited) — `u32` is one row of the integer family.
  Today's `Stutter::Item = u32` (the simplest integer choice that
  matches the std-doc `Alternate` example's `i32` modulo signedness).
- **Lesson 037** (cited) — `%` remainder. Used inside `Stutter::next`:
  `self.n % 2 == 0`.
- **Lesson 023** (cited) — `+=` compound-assignment. Used inside
  `Stutter::next`: `self.n += 1`.
- **Lesson 095** (cited) — `struct` with a single named field.
- **Lesson 116** (cited) — default-body trait methods. The synopsis-
  box line at `trait.Iterator.md:81-82` ends in `{ ... }`.
- **Lesson 049** (cited) — method chaining.
- **Lessons 022, 040, 011, 005, 003, 002, 001** (cited) — `for x in
  iter`; dot-call; `println!`; `let`; diagnostic map; `fn main`;
  rustc compile + run.

## Probe 1 — working probe (Stutter without fuse vs with fuse)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/139-iterator-fuse.rs`.

Compiled and run in `/tmp/eduratchet139/`:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
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
$ echo "run-exit=$?"
run-exit=0
```

**This is the centered probe. The contrast is internal to the single
source file.** The same `Stutter { n: 0 }` source is iterated six
times on each side of the divider:

- *Without `.fuse()`* (lines 1-6): `Stutter` resumes after each
  `None`. Three `Some(_)` wrapping `0, 1, 2` interleaved with three
  `None`.
- *With `.fuse()`* (lines 8-13): Calls 1 and 2 reproduce `Some(0)`
  and `None` — the wrapper passes through to `Stutter.next()`. Then
  calls 3-6 all return `None` *without ever calling `Stutter.next()`
  again* — the `Fuse<Stutter>` wrapper has latched.

Three claims simultaneously witnessed:

1. **The bare `Iterator::next` contract permits resuming.**
   `Stutter` is a working example of an iterator that returns `None`
   on call 2 then `Some(1)` on call 3.
2. **`.fuse()` enforces sticky-`None`.** Calls 3-6 on the wrapped
   iterator return `None` despite the inner being capable of
   returning `Some(_)` on those same call indices.
3. **The element type is unchanged.** Both halves print `Option<u32>`
   values via `{:?}` — no element rewriting like `enumerate` did.

## Probe 2 — slice-iter corroboration (`.fuse()` is a no-op on FusedIterator)

Source `slice.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];

    let mut a = v.iter();
    for _ in 0..5 {
        println!("{:?}", a.next());
    }

    println!("---");

    let mut b = v.iter().fuse();
    for _ in 0..5 {
        println!("{:?}", b.next());
    }
}
```

Transcript:

```text
$ rustc slice.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./slice
Some(10)
Some(20)
Some(30)
None
None
---
Some(10)
Some(20)
Some(30)
None
None
$ echo "run-exit=$?"
run-exit=0
```

Five-call sequence is identical on both sides. Slice iterators
implement `FusedIterator` (per the implementor list at
`trait.FusedIterator.md:23+`), so the wrapper at
`trait.FusedIterator.md:18-21` "will be a no-op with no performance
penalty." Empirical witness: same five lines either way.

The lesson body's *Try It* references this probe by name; the
transcript here pins the exact output.

## Probe 3 — type-pin via E0308 names `Fuse<Iter<'_, u64>>`

Source `typeprobe.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _x: u32 = v.iter().fuse();
}
```

Verbatim diagnostic:

```text
error[E0308]: mismatched types
 --> typeprobe.rs:3:19
  |
3 |     let _x: u32 = v.iter().fuse();
  |             ---   ^^^^^^^^^^^^^^^ expected `u32`, found `Fuse<Iter<'_, u64>>`
  |             |
  |             expected due to this
  |
  = note: expected type `u32`
           found struct `Fuse<std::slice::Iter<'_, u64>>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

rustc spells the wrapper-struct type out: `Fuse<Iter<'_, u64>>`
inline, `Fuse<std::slice::Iter<'_, u64>>` in the secondary note.
Same forced-error type-pin technique lessons 134/135/136/137/138
used; today substitutes `Fuse` for `Take`/`Skip`/`Enumerate`. The
struct is referred to opaquely; today's lesson does not unpack its
private fields.

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 281-284 (the resuming clause on `next`)

Verbatim:

```
Returns [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None") when iteration is finished. Individual iterator
implementations may choose to resume iteration, and so calling `next()`
again may or may not eventually start returning [`Some(Item)`](../option/enum.Option.md#variant.Some "variant std::option::Option::Some") again at some
point.
```

This is the *load-bearing* corpus claim today — the bare `Iterator`
contract does not require sticky-`None`. Without this clause, the
"`fuse()` enforces sticky-`None`" claim would not have a *thing to
enforce against*. Probe 1 left half (`Stutter` alone) is the
empirical witness: an iterator that exercises this license.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 81-82 (synopsis-box line)

Verbatim:

```
    fn fuse(self) -> Fuse<Self> ⓘ
       where Self: Sized { ... }
```

The `{ ... }` body marker is lesson 116's default-body shape. This
licenses every iterator inheriting `fuse` for free — `Stutter` does
not implement `fuse` itself, but `Stutter { n: 0 }.fuse()` is callable
because of the inherited default body.

### `output/docs/rust/std/iter/trait.Iterator.md` line 1737 (per-method declaration)

Verbatim:

```
#### fn [fuse](#method.fuse)(self) -> [Fuse](struct.Fuse.md "struct std::iter::Fuse")<Self> [ⓘ](#) where Self: [Sized](../marker/trait.Sized.md "trait std::marker::Sized"),
```

Authoritative source for:

- **Method name** `fuse` and **receiver shape** `(self)` — bare
  `self` (lesson 102 consuming) with *no* second parameter (same as
  `enumerate`).
- **Return type** `Fuse<Self>` — the wrapper struct documented at
  `output/docs/rust/std/iter/struct.Fuse.md`.
- **`where Self: Sized`** — same bound `take` / `skip` / `enumerate`
  carry; named-deferred today.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1739-1747 (prose summary)

Verbatim:

```
Creates an iterator which ends after the first [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None").

After an iterator returns [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None"), future calls may or may not yield
[`Some(T)`](../option/enum.Option.md#variant.Some "variant std::option::Option::Some") again. `fuse()` adapts an iterator, ensuring that after a
[`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None") is given, it will always return [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None") forever.

Note that the [`Fuse`](struct.Fuse.md "struct std::iter::Fuse") wrapper is a no-op on iterators that implement
the [`FusedIterator`](trait.FusedIterator.md "trait std::iter::FusedIterator") trait. `fuse()` may therefore behave incorrectly
if the [`FusedIterator`](trait.FusedIterator.md "trait std::iter::FusedIterator") trait is improperly implemented.
```

Grounds:

- **"Creates an iterator which ends after the first `None`"** — the
  centered semantic claim. Probe 1 right half (six calls, latches
  after the second).
- **"After an iterator returns `None`, future calls may or may not
  yield `Some(T)` again"** — the second formulation of the same
  resuming-license already named at `:281-284`.
- **"`fuse()` adapts an iterator, ensuring that after a `None` is
  given, it will always return `None` forever"** — the centered
  enforcement claim.
- **"the `Fuse` wrapper is a no-op on iterators that implement the
  `FusedIterator` trait"** — Probe 2's witness.
- **"`fuse()` may therefore behave incorrectly if the
  `FusedIterator` trait is improperly implemented"** — implementor
  caveat. Named-deferred in *What To Ignore For Now*.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 1751-1787 (Examples — `Alternate` iterator)

Verbatim:

```
// an iterator which alternates between Some and None
struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let val = self.state;
        self.state = self.state + 1;

        // if it's even, Some(i32), else None
        (val % 2 == 0).then_some(val)
    }
}

let mut iter = Alternate { state: 0 };

// we can see our iterator going back and forth
assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);

// however, once we fuse it...
let mut iter = iter.fuse();

assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), None);

// it will always return `None` after the first time.
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
```

The std-doc example uses an alternating-iterator pattern essentially
identical to today's `Stutter`. Differences for audience-level reasons:

- Today uses `u32` instead of `i32` (lesson 080's integer family;
  `u32` is the simplest unsigned non-`usize` row used in this run's
  iterator examples since 132's `Counter`).
- Today uses an explicit `if/else` instead of `.then_some(...)`,
  which is a method on `bool` not yet installed in the run.
- Today uses `+=` (lesson 023) instead of `self.state = self.state +
  1` for the increment — equivalent semantically; `+=` is already
  installed, more concise.
- Today yields `Some(self.n / 2)` instead of `Some(val)` so that the
  yielded values count `0, 1, 2, …` rather than `0, 2, 4, …` — purely
  cosmetic.

The structural shape (alternating `Some`/`None` driven by a counter)
matches the std example exactly. The `.fuse()` enforcement claim is
witnessed by Probe 1 the same way the std example witnesses it via
the `assert_eq!` block.

### `output/docs/rust/std/iter/struct.Fuse.md` lines 1-16

Verbatim:

```
# Struct Fuse

1.0.0 ·

```
pub struct Fuse<I> { /* private fields */ }
```

Expand description

An iterator that yields `None` forever after the underlying iterator
yields `None` once.

This `struct` is created by [`Iterator::fuse`](trait.Iterator.md#method.fuse "method std::iter::Iterator::fuse"). See its documentation
for more.
```

Grounds the wrapper-struct claim. Same opaque-struct treatment as
`Take<I>` (136), `Skip<I>` (137), `Enumerate<I>` (138). The
prose-line "yields `None` forever after the underlying iterator
yields `None` once" is the third corpus restatement of the centered
semantic.

### `output/docs/rust/std/iter/trait.FusedIterator.md` lines 1-21

Verbatim:

```
# Trait FusedIterator

1.26.0 ·

```
pub trait FusedIterator: Iterator { }
```

Expand description

An iterator that always continues to yield `None` when exhausted.

Calling next on a fused iterator that has returned `None` once is guaranteed
to return [`None`](../option/enum.Option.md#variant.None "variant std::option::Option::None") again. This trait should be implemented by all iterators
that behave this way because it allows optimizing [`Iterator::fuse()`](trait.Iterator.md#method.fuse "method std::iter::Iterator::fuse").

Note: In general, you should not use `FusedIterator` in generic bounds if
you need a fused iterator. Instead, you should just call [`Iterator::fuse()`](trait.Iterator.md#method.fuse "method std::iter::Iterator::fuse")
on the iterator. If the iterator is already fused, the additional [`Fuse`](struct.Fuse.md "struct std::iter::Fuse")
wrapper will be a no-op with no performance penalty.
```

Grounds:

- **The marker-trait shape** `pub trait FusedIterator: Iterator { }`
  — supertrait-extends-Iterator (named-deferred since 132).
- **The marker semantic** "always continues to yield `None` when
  exhausted" / "Calling next on a fused iterator that has returned
  `None` once is guaranteed to return `None` again."
- **The `.fuse()` no-op claim** "If the iterator is already fused,
  the additional `Fuse` wrapper will be a no-op with no performance
  penalty" — Probe 2's witness target.
- **The "should not use `FusedIterator` in generic bounds" advice**
  — named-deferred in *What To Ignore For Now*.

### `output/docs/rust/error_codes/E0308.md`

Probe 3's diagnostic. Type-pin technique installed at lessons 134
Probe 5, 135 Probe 6, 136 Probe 6, 137 Probe 4, 138 Probes 2-3.
Today's substitution: `Fuse` for `Enumerate`.

## Claim-to-evidence map

- "The `Iterator::next` contract permits an iterator to resume after
  returning `None`" — `trait.Iterator.md:281-284`; Probe 1 left half
  (six calls on `Stutter` show three `Some`/`None` interleavings).
- "`Stutter` is a custom iterator that yields `Some(0), None, Some(1),
  None, Some(2), None, …`" — Probe 1 left half lines 1-6.
- "`.fuse()` enforces sticky-`None`: after the first `None`, every
  subsequent call returns `None`" — Probe 1 right half (calls 3-6
  all `None` even though the unfused inner returned `Some(1), None,
  Some(2), None`); corpus prose at `trait.Iterator.md:1739-1743`,
  `struct.Fuse.md:12-13`, `trait.FusedIterator.md:12-15`.
- "Signature `fn fuse(self) -> Fuse<Self> where Self: Sized,`" —
  `trait.Iterator.md:1737` (per-method declaration).
- "`fuse` is one of the 75 provided methods of `Iterator`" —
  `trait.Iterator.md:81-82` synopsis-box line ends in `{ ... }`
  (lesson 116's default-body marker); lesson 132 evidence appendix.
- "Bare `self` receiver, no second parameter" —
  `trait.Iterator.md:1737` reads `(self) -> Fuse<Self>`.
- "Return type `Fuse<Self>` is itself an iterator" —
  `trait.Iterator.md:1737` (signature names `Fuse<Self>`);
  `struct.Fuse.md:7,12-13` (`pub struct Fuse<I> { /* private fields
  */ }`, "An iterator that yields `None` forever after the
  underlying iterator yields `None` once"); Probe 1 right half
  (`for _ in 0..6 { ... b.next() }` works on the binding); Probe 3
  (rustc names the type `Fuse<Iter<'_, u64>>` in the E0308
  expected/found labels).
- "`FusedIterator` is the marker trait declaring sticky-`None`
  behavior; slice iterators implement it" — `trait.FusedIterator.md`
  declaration; the implementor list at `:23+` lists slice-iterator-
  family types.
- "`.fuse()` on an iterator that already implements `FusedIterator`
  is a no-op" — `trait.FusedIterator.md:18-21`; Probe 2 (five calls
  on `v.iter()` and `v.iter().fuse()` produce identical sequences).
- "rustc spells the wrapper type `Fuse<Iter<'_, u64>>` /
  `Fuse<std::slice::Iter<'_, u64>>`" — Probe 3 (forced E0308 inline
  + secondary note labels).
- "`Stutter` reuses lesson 132's `Counter`-style impl pattern" —
  observation source at lines 2-16 (same `struct Name { field: u32 }`
  + `impl Iterator for Name { type Item = u32; fn next(&mut self) ->
  Option<Self::Item> { ... } }` template); lesson 132 evidence
  appendix.
- "`self.n % 2 == 0`" — uses lesson 037's `%` remainder operator;
  observation source line 7.
- "`self.n += 1`" — uses lesson 023's `+=` compound-assignment;
  observation source lines 9, 12.

## Negative / contrast probe coverage

Three probes captured (one centered, two corroborating):

- **Probe 1 (working probe with internal contrast)** — *the centered
  probe*. The contrast is internal to a single source file: same six
  `next()` calls on `Stutter` and `Stutter.fuse()`, different result
  sequences. This is the empirical witness for the core sticky-
  `None` enforcement claim. Without it, the `Stutter`-resumes-then-
  `.fuse()`-stops claim would rest only on corpus prose.
- **Probe 2 (slice-iter corroboration)** — corroborates the
  `FusedIterator` no-op claim from `trait.FusedIterator.md:18-21`.
  Five-call sequence on `v.iter()` and `v.iter().fuse()` is
  identical. Without this, the no-op claim would rest only on
  corpus prose.
- **Probe 3 (type-pin via E0308 on `u32`)** — names the wrapper-
  struct type from rustc's mouth: `Fuse<Iter<'_, u64>>` /
  `Fuse<std::slice::Iter<'_, u64>>`. Same forced-error type-pin
  technique 134/135/136/137/138 captured for their adapters.

**No centered E0382 today.** The consuming-`self` rule is well-
installed by lessons 102/133/134/136/137/138 (six prior captures of
the same E0382 + `note:` template). Today's signature at
`trait.Iterator.md:1737` reads `self` (no `&`, no `mut`); the rule
applies. A seventh substitution would be appendix bloat.

**Not re-witnessing laziness today.** Lesson 136's Probe 2
(Trace + take, three steps) installed the laziness shape for the
adapter family. Today's lesson centers on *post-`None` semantics*,
not laziness — so the laziness re-probe would be off-center
expansion. The `Fuse<Self>` wrapper is lazy by family analogy
(consuming `self`, returns wrapper struct, same family as 136-138).

## Iterator API audit alignment

This lesson is step 9 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 9. **`fuse`** — sticky-`None` rule; the contrast probe writes a
>    custom iterator that resumes after `None`.

Audit §4.3 lists `fuse` as ready-now, composing "self-by-value 102
+ 119". Today executes that move per audit §5 step 9. Lesson 138's
unlock list named today's move:

> future "`Iterator::fuse` — sticky-`None` rule and the `Fuse<I>`
> adapter" moves (audit §5 step 9 — reuses today's adapter shape
> with no second parameter; first adapter that does not change the
> yielded count or contents but only the post-`None` behavior)

The new graph fact today: where lessons 136-138 installed adapters
that change *what the wrapper yields* (count, count, element shape),
today installs **the first adapter whose semantic is purely
behavioral**: same element type, same element count if the inner
sticks at `None`, but a *post-`None`-behavior guarantee* not present
in the bare `Iterator` contract. This unlocks `step_by` (audit §5
step 10), `size_hint` (step 11), and — once the closure arc lands —
`take_while` and `map_while` (which short-circuit and benefit from
the same fused-or-not analysis).
