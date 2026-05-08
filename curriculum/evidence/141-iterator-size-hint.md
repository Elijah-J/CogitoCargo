# Evidence — Lesson 141: read remaining-length bounds with `iter.size_hint()`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/141-iterator-size-hint.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/141-iterator-size-hint.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/141-iterator-size-hint.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet141/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons through 140.

## Direct prerequisite — lesson 140 (closure-free Iterator arc)

Lesson 140 was the previous step in the iterator-API-coverage audit
(§5 step 10: `step_by`). Today closes the closure-free portion of
audit §5 with step 11: `size_hint`. The arc 131-140 was:

- 131 — `.next()` mechanic on a slice iterator (`&mut self` receiver).
- 132 — `Iterator` trait declaration (1 required associated type, 1
  required method, 75 provided methods inheriting via default bodies).
- 133-138 — `count`, `last`, `nth`, `take`, `skip`, `enumerate`
  (consuming `self` and `&mut self` receivers).
- 139 — `fuse` (sticky-`None` adapter, consuming `self`).
- 140 — `step_by` (consuming `self`, first eager-precondition adapter).

After today, every method touched on the trait has had a receiver
shape capture: `self`, `&mut self`, and (today) `&self`.

## Direct prerequisite — lesson 132 (`Iterator` trait declaration)

Lesson 132 installed
`pub trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; /* + 75 provided */ }`
and the default-body inheritance mechanic from lesson 116. The
synopsis-box line for `size_hint` at
`output/docs/rust/std/iter/trait.Iterator.md:18`:

```text
    fn size_hint(&self) -> (usize, Option<usize>) { ... }
```

ends in `{ ... }` — lesson 116's default-body marker. Every iterator
inherits `size_hint` automatically; the default body returns
`(0, None)` per `:371-372` (see "Three corpus claims" below).

## Direct prerequisite — lesson 131 (`.next()` takes `&mut self`)

Lesson 131 installed `.next()` on a slice iterator: signature
`fn next(&mut self) -> Option<Self::Item>`, return type
`Option<&T>` for slice iterators, the binding requires `let mut iter`
(E0596 if `mut` is dropped). Today's centered contrast:

- `size_hint` takes `&self`, not `&mut self`.
- The binding does not need `mut`.
- The call can be repeated; the cursor is not advanced.

Probe 6 (transcript) compresses the contrast into one source file: a
non-mut `let iter` accepts two `iter.size_hint()` calls but rejects
the `iter.next()` call between them with E0596 — the same
diagnostic lesson 131 captured.

## Direct prerequisite — lesson 119 (`Option<T>`)

Lesson 119 installed `Option<T>` / `Some(_)` / `None`. Today's
return type substitutes `T = usize` to give `Option<usize>` for the
upper-bound slot. The `Some(n)` form is witnessed in Probe 1
output (`Some(3)`, `Some(2)`, `Some(0)`). The `None` form is *not*
probed today — slice iterators always know their exact remaining
count, so the upper slot is never `None` for them. The doc names
infinite ranges (`0..`) at `:407-411` as an iterator with `None`
upper, but `Range` iteration is still gated (deferred since 022).

## Direct prerequisite — lesson 100 (`&self` receiver shape)

Lesson 100 installed `&self` as a receiver shape on user-defined
inherent impls (`fn current(&self) -> u32` on `Counter`). Today is
the *first* appearance of `&self` on a stdlib provided `Iterator`
method — confirmed by walking the full 76-method declaration in
the page header (lines 12-89 of `trait.Iterator.md`). The previous
nine accepted Iterator methods had:

- `next` — `&mut self` (lesson 131).
- `count`, `last`, `take`, `skip`, `enumerate`, `fuse`, `step_by`
  — bare `self` (lessons 133, 134, 136, 137, 138, 139, 140).
- `nth` — `&mut self` (lesson 135).

`size_hint` is the third receiver shape installed on a stdlib
Iterator method. Three structural slots inherited from lesson 100
unchanged: `&self` is shorthand for `self: &Self`; the binding does
not need `mut` to call; the receiver is not consumed.

## Three corpus claims (centered semantics)

`output/docs/rust/std/iter/trait.Iterator.md:347-354` (return shape):

```text
Returns the bounds on the remaining length of the iterator.

Specifically, `size_hint()` returns a tuple where the first element
is the lower bound, and the second element is the upper bound.

The second half of the tuple that is returned is an `Option<usize>`.
A `None` here means that either there is no known upper bound, or the
upper bound is larger than `usize`.
```

`:371-372` (default impl):

```text
The default implementation returns `(0, None)` which is correct for any
iterator.
```

`:358-369` (hint, not guarantee):

```text
It is not enforced that an iterator implementation yields the declared
number of elements. A buggy iterator may yield less than the lower bound
or more than the upper bound of elements.

`size_hint()` is primarily intended to be used for optimizations such as
reserving space for the elements of the iterator, but must not be
trusted to e.g., omit bounds checks in unsafe code. An incorrect
implementation of `size_hint()` should not lead to memory safety
violations.

That said, the implementation should provide a correct estimation,
because otherwise it would be a violation of the trait’s protocol.
```

The "hint, not guarantee" caveat is named in the lesson body
(`What Changed` final bullet) but not probed empirically — doing so
would require constructing a buggy iterator implementation, which
is out of scope. The std documentation states the rule directly;
that is the load-bearing claim for the appendix.

## Probe 1 — working probe (size_hint before/after `.next()`; empty)

Source: `observations/141-iterator-size-hint.rs`. Transcript:
`observations/141-iterator-size-hint.transcript.txt` Probe 1 block.

Output:

```text
(3, Some(3))
(2, Some(2))
(0, Some(0))
```

Three lines:

- Line 1: a fresh `v.iter()` over `vec![10, 20, 30]` reports
  lower=3, upper=`Some(3)`. Slice iterators give exact counts.
- Line 2: after one `iter.next()` advances the cursor by one, the
  remaining length is 2, reported exactly: `(2, Some(2))`.
- Line 3: an empty source `vec![]` reports `(0, Some(0))`.

Lower and upper match for every line — slice iterators always know
their exact remaining length.

## Probe 2 — centered contrast (`&self` — no `mut` needed)

Source inline in transcript. Output:

```text
(3, Some(3))
(3, Some(3))
(3, Some(3))
```

Three sequential `iter.size_hint()` calls on a `let iter` (no
`mut`). All three succeed; all three return identical tuples. This
empirically witnesses:

1. The receiver is `&self` (immutable borrow). No `mut` is required
   on the binding — contrast lesson 131's `.next()` E0596.
2. The cursor is *not* advanced by `size_hint`. The same lower
   bound `3` is reported on call 2 and call 3 even though calls 1
   and 2 have already happened.

Together with Probe 6's negative case (E0596 fires on the bare
`iter.next()` between two successful `iter.size_hint()` calls), the
centered new fact "`&self`, not `&mut self`" is fully grounded.

## Probe 3 — type-pin (rustc names the return type)

```text
error[E0308]: mismatched types
 --> typeprobe.rs:4:19
  |
4 |     let _x: u32 = v.iter().size_hint();
  |             ---   ^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `(usize, Option<usize>)`
  |             |
  |             expected due to this
  |
  = note: expected type `u32`
            found tuple `(usize, Option<usize>)`
```

Two grounded facts from rustc's mouth:

- The return type spells `(usize, Option<usize>)`. Not
  `Option<(usize, usize)>`, not a struct, not a typedef.
- The `note:` line names the *kind* `tuple` — empirical
  confirmation that the return is a 2-tuple of the lesson 072
  shape, not some other compound.

This rules out two plausible misreadings: that the whole return
might be wrapped in `Option`, or that it might be a named struct
(like `Bounds` or similar). The corpus declaration at line 345
spells the same `(usize, Option<usize>)` form — Probe 3 confirms
rustc agrees.

## Probe 4 — corroboration (size_hint after exhaustion)

Output (with annotations):

```text
before: (1, Some(1))
after one .next(): (0, Some(0))
after second .next() (now None): (0, Some(0))
```

A 1-element source `vec![10]`:

- Before any `.next()` calls, size_hint reports `(1, Some(1))`.
- After the first `.next()` (which yields `Some(&10)`), size_hint
  reports `(0, Some(0))` — the iterator knows the cursor is now
  past the only element.
- After the second `.next()` (which yields `None`), size_hint
  *still* reports `(0, Some(0))`. Calling `next` past exhaustion
  does not change the size_hint reading.

This corroborates that slice iterators track exact remaining length
through state changes, including the post-`None` state. It also
foreshadows lesson 139's `FusedIterator` claim: slice iterators
have the property "once `None`, always `None`", and size_hint
reflects that by staying at `(0, Some(0))`.

## Probe 5 — three receiver shapes coexist on one binding

Output:

```text
(3, Some(3))
(2, Some(2))
count = 2
```

Sequence:

1. `iter.size_hint()` — `&self` borrow. Reads bounds. Cursor: 0.
2. `iter.next()` — `&mut self` borrow. Advances cursor to 1.
3. `iter.size_hint()` — `&self` borrow. Reads bounds. Cursor: 1.
4. `iter.count()` — `self` consumed. Counts remaining (2 elements).

The `let mut iter` binding is required because of (2) and (4), not
(1) or (3). After (4), the binding cannot be used again — `count`
is a consuming method. This probe shows all three receiver shapes
coexisting on one binding without conflict.

## Probe 6 — negative contrast (E0596 on `let iter`)

```text
error[E0596]: cannot borrow `iter` as mutable, as it is not declared as mutable
 --> nomut.rs:9:13
  |
9 |     let _ = iter.next();                  // would fail — &mut self
  |             ^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
7 |     let mut iter = v.iter();
  |         +++
```

In a single source file: `iter.size_hint()` succeeds twice on a
`let iter` (no `mut`); the `iter.next()` call between them fires
E0596 — the same diagnostic lesson 131 captured. The centered new
fact is the *receiver-shape contrast*: `size_hint` (`&self`) vs
`next` (`&mut self`).

This probe is the contrastive witness the worker contract requires
when the lesson makes a "with X works, without X fails/differs"
claim. Today's claim: "`size_hint` works on a non-mut binding;
`.next()` fails." Probe 6 supplies both halves in one source.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn size_hint(&self) -> (usize, Option<usize>)` | `trait.Iterator.md:345`; Probe 3 type-pin |
| `&self` receiver — first on stdlib provided Iterator method | `trait.Iterator.md:18` (synopsis); audit walk of all 76 methods; Probe 2 (3 calls succeed on `let iter`); Probe 6 (E0596 on `iter.next()` between two `size_hint` calls) |
| Return is a bare 2-tuple `(usize, Option<usize>)` | `trait.Iterator.md:345, 349-354`; Probe 3 (`found tuple (usize, Option<usize>)`) |
| First slot = lower bound | `trait.Iterator.md:349-350`; Probe 1, 4, 5 outputs |
| Second slot = upper bound (`Option<usize>`) | `trait.Iterator.md:349-354`; Probe 1, 4, 5 outputs (always `Some(_)` for slice iters) |
| `None` upper means "no known upper bound or > `usize::MAX`" | `trait.Iterator.md:352-354`; not probed (slice iters always exact); deferred to `0..` example |
| Default impl returns `(0, None)` | `trait.Iterator.md:371-372`; not probed (slice iters override) |
| Slice iterators report exact remaining count | Probe 1 (3 lines), Probe 4 (3 lines), Probe 5 (lines 1, 2) |
| Cursor not advanced by `size_hint` | Probe 2 (three identical lines) |
| Binding does not need `mut` | Probe 2 (`let iter`, three calls succeed); Probe 6 (E0596 on `iter.next()` but not `iter.size_hint()`) |
| Empty iterator → `(0, Some(0))` | Probe 1 line 3 |
| size_hint after `.next()` returns `None` still reports `(0, Some(0))` | Probe 4 line 3 |
| Three Iterator-method receiver shapes coexist | Probe 5 (size_hint, next, count on one binding) |
| `size_hint` is hint, not guarantee | `trait.Iterator.md:358-369`; not probed (would need buggy iterator) |
| `size_hint` is one of 75 provided methods | Lesson 132; `trait.Iterator.md:18` synopsis line ends in `{ ... }` |
| Same panic shape lesson 053 captured (used elsewhere on the page) | Not relevant today — `size_hint` does not panic |

## Older supporting lessons (mentioned by name only)

- 072-tuple-type-and-index — installs `(A, B)` and `.0`/`.1`. Today's
  return type is one specific tuple shape `(usize, Option<usize>)`.
- 073-let-tuple-destructure — installs `let (a, b) = pair;`. Not
  used in today's probes; the lesson body keeps the tuple intact
  for `{:?}` debug printing. Future moves can destructure the bounds.
- 080-integer-type-family — installs `usize`. Today's lower-bound
  slot and `Option<usize>` upper-bound payload both use `usize`.
- 116-trait-default-method-body — installs the `{ ... }` synopsis
  marker. `size_hint`'s synopsis line at `:18` carries it.
- 040-method-call-syntax — installs `iter.size_hint()`'s dot-call.
- 011-println-positional-args — installs `{:?}` Debug placeholder.
- 005-let-binding — installs `let name = value;`.
- 003-read-rustc-diagnostic — installs the four-part diagnostic map.
  Probes 3 and 6 read E0308 and E0596 with that map.
- 002-fn-main-entry-point — installs `fn main`.
- 001-rustc-compile-and-run — installs `rustc demo.rs && ./demo`.

## Risks and deliberate deferrals

- **The `None` upper-bound case is not probed.** The doc names
  `0..` (`:407-411`) as an iterator with `None` upper, but `Range`
  iteration is still gated since lesson 022. Closure-driven cases
  (`(0..10).filter(|x| x % 2 == 0)`) are also gated. The lesson
  names this as a corpus claim and the appendix flags the gap.
- **The default `(0, None)` impl is not probed.** Witnessing it
  would require either an iterator impl that does not override
  `size_hint` (most do), or a corpus walk to find one that
  doesn't. Today the lesson states the rule from the doc. Future
  moves can probe a custom user-defined iterator that omits
  `size_hint`.
- **The "hint, not guarantee" caveat is stated, not probed.**
  Probing would require constructing a buggy iterator
  implementation. The doc states the rule directly; that is the
  load-bearing claim. Treated as a corpus-named fact.
- **No probe for `size_hint` on a custom user-defined iterator
  (e.g., overriding it to return non-default bounds).** Out of
  scope today — the centered new fact is the `&self` receiver,
  not custom impls. Future move could compose lesson 132's
  user-defined iterator pattern with a `size_hint` override.

## Audit context

This is step 11 of the iterator-API-coverage audit §5 (closure-free
non-consumer Iterator surface). After today, the closure-free arc
is complete. The next moves per audit §4.4 are:

- §4.4.1 closure sub-arc (`map`, `filter`, `take_while`, etc.).
- §4.4.2 `IntoIterator` sub-arc.
- §4.4.3 bounded-by-other-trait sub-arc (`sum`, `product`, ordering).
- §4.4.4 `Try` sub-arc.

These can proceed in parallel where their prereqs allow. Today's
lesson installs the third Iterator receiver shape, which closes a
quiz-fact column for every method seen so far on the trait.
