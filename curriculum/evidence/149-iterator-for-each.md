# Evidence — Lesson 149: `Iterator::for_each` (first closure-driven Iterator method)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/149-iterator-for-each.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/149-iterator-for-each.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/149-iterator-for-each.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet149/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-148.

## Source-choice note (revision)

This lesson originally used `let v: Vec<u32> = vec![10, 20, 30];
v.iter().for_each(...)` as the source. The red-team flagged that
shape because:

- `v.iter()` over `Vec<u32>` yields `&u32` elements (lesson 131), so
  the closure parameter `x` was `&u32`. The accumulator probe wrote
  `sum += x` with `sum: u32, x: &u32`, which compiles by virtue of
  the std-lib `impl AddAssign<&u32> for u32` (`output/docs/rust/std/primitive.u32.md:3144`,
  stable since 1.22.0). Lesson 023 explicitly deferred AddAssign
  trait machinery — including this reference-RHS impl — so the
  probe smuggled an un-installed mechanic.
- The `break`-contrast probe wrote `*x == 20` and Check Yourself
  wrote `*x % 2 == 0`. The `*x` reads through the reference. Lessons
  047 and 048 explicitly deferred deref-read; no later lesson
  installs it.

The revised lesson switches the iterator source to a `Range<u32>`,
written `(1..4_u32)`. A `Range<A>: Iterator where A: Step` (lesson
091's evidence quotes `output/docs/rust/std/ops/struct.Range.md:139-141`
verbatim), and the iterator yields *owned* `A` values — for
`Range<u32>`, owned `u32`s. So `x: u32` in the closure, and:

- `sum += x` is `u32 += u32` — pure lesson 023 `+=` on integers, no
  AddAssign-on-reference.
- `x == 2` is `u32 == u32` — pure lesson 013 `==` on integers, no
  deref-read.
- `x % 2` is `u32 % u32` — pure lesson 037 `%` on integers, no
  deref-read.

The shape of the lesson is otherwise unchanged: signature segments,
the `FnMut`-vs-`Fn` argument, the `break`-rejected probe, the
non-unit-bind probe, the consumes-self probe, the non-closure-arg
probe, and the Check Yourself probes all carry over with the source
swapped.

## Run context — first closure-driven Iterator method (audit §7)

Per `iterator-api-coverage.md` §7 (lines 417-446), the closure
sub-arc closed at lesson 148. The audit names two leading
candidates for the first closure-driven Iterator method:

- `Iterator::for_each` — consumer, `FnMut(Self::Item)`, returns
  `()`. Simplest of the family because it consumes `self` (no
  `Self::Item` rewriting like `map`, no wrapper struct return),
  takes exactly one closure, and returns `()`.
- `Iterator::map` — lazy adapter, `FnMut(Self::Item) -> B`,
  returns `Map<Self, F>`. Slightly larger because it rewrites the
  yielded element type and returns a wrapper struct.

This lesson picks `for_each` per the audit's leanness argument.

## Direct prerequisite — lesson 148 (Fn / FnMut / FnOnce distinction)

Lesson 148 installed:

- The Fn-family has three traits — `Fn`, `FnMut`, `FnOnce` —
  with the layered supertrait relation `Fn: FnMut: FnOnce`.
- A closure auto-implements one or more based on what its body
  does with captures: no capture or read-only → `Fn`; mutate →
  `FnMut`; move out → `FnOnce`.
- The bound on the receiving function constrains which closures
  are accepted; `FnMut` bounds need `mut f: F` so the body can
  call `f(x)`.

Today applies all three facts. The `for_each` bound is `FnMut`;
the second probe's `|x| sum += x` mutates a captured binding and
so implements `FnMut` (and `FnOnce`) but not `Fn`. The bound's
choice of `FnMut` over `Fn` is exactly what makes that probe
legal. Lesson 148's *unlocks* explicitly named today: "the
`Iterator::for_each` consumer with `FnMut(Self::Item)` bound —
likely first closure-driven Iterator method."

## Direct prerequisite — lesson 147 (parenthesized Fn-trait bound)

Lesson 147 installed the parenthesized form `<F: FnMut(T) -> R>`.
Today's bound `F: FnMut(Self::Item)` reuses that grammar in the
no-return-segment form (return defaults to `()`). The lesson body's
"Read it segment by segment" walk is exactly lesson 147's grammar
applied to `for_each`'s declaration line. Without 147 the audience
cannot read `FnMut(Self::Item)` as one parenthesized bound rather
than a function type or a method call.

## Direct prerequisite — lesson 144 (closure captures outer `let`)

The second probe captures `let mut sum: u32 = 0;` from the
enclosing scope and reads/mutates it inside the closure body.
Lesson 144 installed the capture mechanic; today's centered fact
is *which* trait the closure implements (FnMut), per lesson 148.

## Direct prerequisite — lesson 142 (closure literal, bound and called)

Today's two closures `|x| println!("{}", x)` and `|x| sum += x`
are both lesson-142 closure literals: pipes around the parameter
list, single-expression body. The new fact is that the closure
is the *argument* to `for_each` rather than bound to a `let`.

## Direct prerequisite — lesson 132 (Iterator trait declaration)

Lesson 132 installed `type Item;` as the trait's associated-type
slot, and `Self::Item` as the path through it. Today's bound
`FnMut(Self::Item)` reads the closure's parameter type from this
slot. For a `Range<u32>`, `Item = u32` — the iterator yields
*owned* `u32` values. Probe 4 (E0277) provides the empirical
witness: rustc spells the expected closure type as `FnMut(u32)`.

## Direct prerequisite — lesson 091 (Range as Iterator + parens-rule)

Lesson 091 grounds two facts load-bearing today:

- `Range<A>: Iterator` where `A: Step`. Lesson 091's evidence quotes
  `output/docs/rust/std/ops/struct.Range.md:139-141` verbatim:
  `### impl<A> DoubleEndedIterator for Range<A> where A: Step,`
  (which composes `Iterator` as a supertrait per lesson 132's trait
  hierarchy). All twelve integer types from lesson 080 implement
  `Step`, so `Range<u32>` is an `Iterator` whose `Item = u32`.
- The parens-rule for method calls on a range: `(1..4).rev()` parses
  as `(1..4).rev()` — `(1..4)` is the receiver of `.rev()`. Without
  parens, `4.rev()` parses first as a method call on the integer.
  Today's `(1..4_u32).for_each(...)` reuses this exact mechanic with
  a different method name and the integer-suffix form for the upper
  bound.

## Direct prerequisite — lesson 081 (integer-literal type-suffix form)

Lesson 081 installs the type-suffix form `57u8`, equivalent to
`let x: u8 = 57;` without a separate annotation. Today's `4_u32`
suffix pins the upper bound of the range to `u32`, which makes the
range a `Range<u32>` (no separate `let` annotation needed). The
underscore between digits and suffix is the lesson-081 visual
separator, also accepted before a suffix.

## Direct prerequisite — lesson 080 (integer-type family)

Lesson 080 installs the twelve integer type names. Today's `u32`
appears three places: the `1..4_u32` literal suffix, the `let mut
sum: u32 = 0;` annotation, and the `let mut count: u32 = 0;`
annotation. Combined with 091, this fixes `Self::Item = u32` for
the iterator.

## Direct prerequisite — lesson 023 (`+=` compound assignment)

Lesson 023 installed `n += value;` as shorthand for `n = n + value;`
on a `let mut`-declared binding. Today's `sum += x` and `count += 1`
are exactly this shape: both `sum: u32` and `x: u32` are owned
`u32` values, and `count: u32`, `1: u32` likewise. Lesson 023's
*What To Ignore For Now* explicitly named "the `AddAssign` trait"
as deferred — staying in the lesson-023 mechanic means *both* sides
of `+=` are owned integers, not references, so no `AddAssign<&T>`
trait machinery is invoked.

## Cited prereq — lesson 013 (`==` on integers)

Lesson 013 installed the six comparison operators including `==`.
Today's `x == 2` (in `break_in_closure.rs`) and `x % 2 == 0` (in
Check Yourself) are integer-vs-integer-literal comparisons producing
booleans. With a `Range<u32>` source the closure parameter `x` is
owned `u32`, so the comparison is plain `u32 == u32` — no extra
machinery.

## Cited prereq — lesson 037 (`%` remainder operator)

Lesson 037 installed `%` between two integer values. Today's
Check Yourself uses `x % 2` where both sides are `u32`. Lesson
037's *unlocks* explicitly named "future `even/odd checks via
`n % 2 == 0`` moves" — today's Check Yourself realizes that exact
shape inside an iterator closure body.

## Cited prereq — lesson 143 (unannotated closure parameter)

Both probe closures write `|x|` (no type annotation). Lesson 143
installed the no-annotation grammar plus the rule that rustc
infers the parameter type from the *first call site* when the
closure is bound to `let` and called locally. Today's path to
the type is structurally different — the closure is passed as
an argument to `for_each`, and rustc reads the expected closure
shape from the bound `FnMut(Self::Item)` = `FnMut(u32)` (per
lesson 132 + the `Range<u32>` impl's `type Item = u32`). So today
the *grammar* of `|x|` (no annotation) is from lesson 143; the
*type-binding mechanism* is the bound, not the first-call rule.

Probe 8 confirms by annotating `|x: u32|` explicitly — rustc
accepts, witnessing that the closure's expected parameter type
under this bound is exactly `u32`.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's new
codes:

- **E0267** (Probe 2): `\`break\` inside of a closure`. Rustc
  annotates the enclosing closure literal with `--- enclosing
  closure` and carets the offending `break` token. New error code
  this lesson.

Reappearing codes (with the lesson 003 four-part shape unchanged):

- **E0277** (Probe 4): same code as lessons 146/147; today's
  payload spells the trait in the parenthesized form
  `FnMut(u32)`.
- **E0308** (Probe 3): same code as lessons 142/138; today's
  payload reports `expected u32, found ()` — confirming
  `for_each` returns `()`.
- **E0382** (Probe 5): same code as lessons 133/148; today's
  payload's `note:` reads `\`for_each\` takes ownership of the
  receiver \`self\`, which moves \`it\``. Direct empirical
  witness for "consumes `self`."
- **E0594** (Probe 7, Check Yourself): cannot assign to
  immutable captured binding. Same code as lesson 088 (and
  earlier; this run has cited it before for compound-assignment
  probes). Today's payload places the caret on `count += 1`
  inside the closure body, with `help:` proposing
  `let mut count: u32 = 0;` at the binding site. The bound being
  `FnMut` lets the closure body *try* the mutation; lesson 006's
  rule about the captured binding still applies.

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 145**: generic function `<F>` slot.
- **Lesson 011**: `println!("{}", x)`.
- **Lesson 080**: `u32` named integer type (covered above; mentioned
  again here for the print-line context).
- **Lesson 005**: `let n = ...;`. **Lesson 006**: `let mut n = ...;`.
  The second probe's `let mut sum: u32 = 0;` is exactly lesson 006's
  shape with lesson 062's optional type annotation.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc file.rs && ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics)

The corpus file is the std doc page for `Iterator`. Verified by
reading.

### Line 902 (`for_each` signature)

```text
#### fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item),
```

Direct corpus source for the lesson body's signature. Three
load-bearing facts read from this single line:

- `<F>` — one type parameter for the closure (lesson 145's slot).
- `(self, f: F)` — receiver `self` (consuming, lesson 134-style),
  one closure-typed parameter `f: F`.
- *No return type slot* — `for_each` returns `()`. Probe 3
  empirically confirms by binding the call to `let r: u32 = ...`
  and observing E0308 with `expected u32, found ()`.
- `where Self: Sized, F: FnMut(Self::Item)` — bound on the
  receiver and bound on the closure. The closure bound is the
  one centered today. `Self::Item` slot resolved per impl per
  lesson 132. The parenthesized `FnMut(Self::Item)` form composes
  lesson 147's grammar with lesson 148's trait choice.

### Lines 41-43 (synopsis-box version)

```text
fn for_each<F>(self, f: F)
   where Self: Sized,
         F: FnMut(Self::Item) { ... }
```

Same signature in the synopsis box at the top of the file. The
`{ ... }` placeholder confirms `for_each` is a *provided* method
(lesson 116) — every Iterator impl gets it for free.

### Lines 901, 904-913 (stabilization, semantics)

```text
1.21.0 ·

Calls a closure on each element of an iterator.

This is equivalent to using a `for` loop on the iterator,
although `break` and `continue` are not possible from a closure.
It's generally more idiomatic to use a `for` loop, but `for_each`
may be more legible when processing items at the end of longer
iterator chains. In some cases `for_each` may also be faster than
a loop, because it will use internal iteration on adapters like
`Chain`.
```

Direct corpus source for:

- "Calls a closure on each element" (line 904) → today's centered
  semantics ("calls it once per element").
- "`break` and `continue` are not possible from a closure" (lines
  905-906) → Probe 2 empirically witnesses this with E0267.
- Performance / `Chain` internal iteration (lines 909-911) →
  today's *What To Ignore For Now* defers as implementor-side.
- Stabilization at 1.21.0 (line 901) → far below the local
  toolchain 1.95.0.

### Lines 915-936 (corpus examples)

The std doc shows two examples; the first is `(0..5).map(...).for_each(...)`,
which uses a `Range` source — exactly today's source-shape choice.
Today's probe simplifies to a bare `Range<u32>` source without the
intermediate `.map(...)` adapter.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Lines 139-141 verbatim:

```text
### impl<A> DoubleEndedIterator for Range<A> where A: Step,
```

Direct corpus warrant: `Range<A>` (the type produced by the `..`
operator on two `A` values) implements `DoubleEndedIterator` (and
hence `Iterator`) for any `A: Step`. All twelve integer types from
lesson 080 implement `Step`. So `1..4_u32` is a `Range<u32>` and
its `Iterator::Item` is `u32`.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lesson 148 already cited this. Today reuses lesson 148's claim
without re-quoting: line 27 verbatim "`FnMut` is implemented
automatically by closures which take mutable references to
captured variables." The second probe's `|x| sum += x` exactly
matches: it borrows `sum` mutably from the enclosing scope. So
the closure implements `FnMut`, satisfies the bound, and the
program compiles.

## Source — `output/docs/rust/error_codes/E0267.md` (break inside closure)

The corpus error-codes page for E0267. Verified by reading.

```text
This error indicates the use of a loop keyword (`break` or `continue`)
inside a closure but outside of any loop.
```

The example given is `let w = || { break; };`. The fix described
on the page is "`break` and `continue` keywords can be used as
normal inside closures as long as they are also contained within
a loop." Probe 2's `break` is *not* contained in a loop inside
the closure — the closure body is a `for_each` callback with
just an `if` — so the rule fires. The `for_each` "for-loop"
semantics happen *outside* the closure (in `for_each`'s
implementation), not inside the closure body, which is exactly
why this E0267 is correct: the closure is its own scope.

## Probe 1 — working probe (capture-nothing closure + capture-and-mutate closure)

Source: `observations/149-iterator-for-each.rs`. Transcript:
`PROBE 1` block.

Output: four lines `1` / `2` / `3` / `sum = 6`. Compile-exit=0,
run-exit=0. Five load-bearing facts:

- The bound `F: FnMut(Self::Item)` accepts both a capture-nothing
  closure (`|x| println!("{}", x)`) and a capture-and-mutate
  closure (`|x| sum += x`) when called on `(1..4_u32)`. The first
  is also `Fn`, so any Fn-bound function would accept it; the
  second is `FnMut` only (per lesson 148's auto-impl rule), so
  only an `FnMut`-or-weaker bound accepts it.
- The closure is called once per element, in iteration order
  (`1`, `2`, `3` printed in order — the exclusive upper bound
  rule from lesson 022 / 091 means `4` is *not* yielded).
- `for_each` consumes the iterator. Two separate `(1..4_u32)`
  expressions construct two independent iterator values; this is
  legal because the *expression* is evaluated fresh each time, not
  the same iterator value used twice. Probe 5 below witnesses what
  happens when the iterator value *is* the binding consumed.
- The captured `sum` ends at `6` — `0 + 1 + 2 + 3`. The closure
  body's mutation is observable from outside the closure after the
  `for_each` call returns, because mutation happens through a
  mutable borrow of the captured binding, not a copy.
- `for_each` returns `()`. The probe does not bind the return
  value; Probe 3 below witnesses what happens when you try.

## Probe 2 — `break` inside the closure body (E0267)

Source: `break_in_closure.rs`. Transcript: `PROBE 2` block.

Modification from Probe 1: replace the body of the first closure
with `if x == 2 { break; } println!("{}", x);`. The `break` is
inside the closure body but not inside any loop within the closure.
The condition `x == 2` is plain `==` between two `u32` values
(lesson 013) — `x` is owned, no deref needed. Output:

```text
error[E0267]: `break` inside of a closure
 --> break_in_closure.rs:7:13
  |
5 |     (1..4_u32).for_each(|x| {
  |                         --- enclosing closure
6 |         if x == 2 {
7 |             break;
  |             ^^^^^ cannot `break` inside of a closure
```

Compile exit 1. Three load-bearing facts:

- Error code `E0267`. New code today.
- Diagnostic shape is the lesson 003 four-part map: headline,
  `-->` location at the offending `break` token, source excerpt
  with the `--- enclosing closure` annotation pointing at `|x|`,
  and `cannot \`break\` inside of a closure` as the inline label.
- The std-doc claim "`break` and `continue` are not possible from
  a closure" (`trait.Iterator.md:905-906`) is empirically
  witnessed.

## Probe 3 — bind `for_each` result to non-`()` (E0308)

Source: `non_unit_bind.rs`. Transcript: `PROBE 3` block.

Modification: the first call is bound to `let r: u32 = ...`.
Output:

```text
error[E0308]: mismatched types
 --> non_unit_bind.rs:4:18
  |
4 |     let r: u32 = (1..4_u32).for_each(|x| println!("{}", x));
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `()`
```

Compile exit 1. Two load-bearing facts:

- The actual return type rustc spells is `()`. Direct empirical
  witness for the lesson body's "`for_each` returns `()`."
- The diagnostic carries the lesson 003 shape unchanged. Same
  E-code as lesson 138's enumerate type-pin probe (`Option<u64>`
  vs `Option<(usize, &u64)>`); today's payload is `u32` vs `()`.

## Probe 4 — pass a non-closure to `for_each` (E0277)

Source: `non_closure_arg.rs`. Transcript: `PROBE 4` block.

Modification: pass `7` instead of a closure. Output:

```text
error[E0277]: expected a `FnMut(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:4:25
  |
4 |     (1..4_u32).for_each(7);
  |                -------- ^ expected an `FnMut(u32)` closure, found `{integer}`
```

Compile exit 1. Three load-bearing facts:

- Rustc spells the expected closure type as `FnMut(u32)`. Direct
  empirical witness that `Self::Item` for `Range<u32>` is `u32`
  (owned, not a reference). Composes lesson 091's `Range<A>:
  Iterator` with lesson 132's `Self::Item` slot.
- The trait spelled is `FnMut` (not `Fn` or `FnOnce`) — the bound
  in the trait declaration is what the diagnostic reflects.
- Same E-code as lesson 147's "non-closure argument" contrast.
  This probe is recorded but is not centered in the lesson body
  — it rehearses lesson 147's non-closure-argument fact rather
  than installing today's centered claim. Its main role here is
  to surface the rustc-spelled expected type `FnMut(u32)` as
  evidence that the bound resolves to that exact closure shape.

## Probe 5 — use the iterator binding after `for_each` (E0382, consumes `self`)

Source: `for_each_consumes.rs`. Transcript: `PROBE 5` block.

Modification: bind the iterator to `let it = 1..4_u32;`, call
`it.for_each(...)`, then call `it.count()` afterwards. Output:

```text
error[E0382]: use of moved value: `it`
 --> for_each_consumes.rs:7:18
  |
5 |     let it = 1..4_u32;
  |         -- move occurs because `it` has type `std::ops::Range<u32>`, which does not implement the `Copy` trait
6 |     it.for_each(|x| println!("{}", x));
  |        ------------------------------- `it` moved due to this method call
7 |     let _again = it.count();
  |                  ^^ value used here after move
  |
note: `for_each` takes ownership of the receiver `self`, which moves `it`
```

Compile exit 1. Three load-bearing facts:

- Error code `E0382`. Same code lessons 133, 134, 148 have
  witnessed for `self`-by-value receivers; today's payload's
  `note:` line reads verbatim "`for_each` takes ownership of the
  receiver `self`, which moves `it`." Direct empirical witness
  for the signature's `self` (consuming) receiver.
- Rustc spells the moved-value type as `std::ops::Range<u32>` —
  direct empirical witness that `1..4_u32` is a `Range<u32>`,
  composing lesson 091 (`Range<A>` is the type produced by `..`)
  with lesson 081 (`4_u32` pins the suffix type).
- The structural reason for `self` (consuming) is consistent with
  lesson 134's `last(self)` and lesson 133's `count(self)` —
  consumer methods take `self` by value.

## Probe 6 — Check Yourself part (a)

Source: `q.rs`. Transcript: `PROBE 6` block.

Captured-mut counter, even-value count over `(1..6_u32)`. Output:
`2`, compile-exit=0, run-exit=0. Verifies the Check Yourself part
(a) answer empirically: the even values in `1..6` are `2` and `4`
(`5` is the upper bound `6` minus one — exclusive upper bound rule
from lesson 022/091), so `count` flips from 0 to 1 to 2.

## Probe 7 — Check Yourself part (b)

Source: `q_no_mut.rs`. Transcript: `PROBE 7` block.

Same source as Probe 6 but `let count: u32 = 0;` (no `mut`).
Output:

```text
error[E0594]: cannot assign to `count`, as it is not declared as mutable
 --> q_no_mut.rs:4:25
  |
4 |         if x % 2 == 0 { count += 1; }
  |                         ^^^^^^^^^^ cannot assign
  |
help: consider changing this to be mutable
  |
2 |     let mut count: u32 = 0;
  |         +++
```

Compile exit 1. Two load-bearing facts:

- E-code `E0594`. The caret falls on `count += 1` *inside* the
  closure body; the `help:` line proposes the fix at the *outer*
  binding site (line 2) with `+++` markers under the inserted
  `mut`. This is a structurally distinct location pair from
  lesson 148's E0596-on-`f`-parameter case, but uses the same
  `+++`-marker insertion fix shape (lesson 006's E0384 contrast
  also).
- The closure body is allowed to *try* the mutation because the
  bound is `FnMut` (lesson 148); the rejection is on the
  captured binding's mutability, not on the closure trait. This
  composes lesson 006 (`let mut`) with lesson 148 (closure body
  mutating captures) — the captured binding has to be `mut` for
  the FnMut auto-impl to actually be legal at run time.

## Probe 8 — type-pin (`|x: u32|` annotation)

Source: `type_pin.rs`. Transcript: `PROBE 8` block.

Modification from Probe 1's first call: add `: u32` annotation
to the closure parameter. Output: same as the first three lines
of Probe 1 (`1`, `2`, `3`), compile-exit=0, run-exit=0. Two
load-bearing facts:

- The closure parameter type *is* `u32` (owned, not `&u32`).
  Probe 1 leaves this inferred (lesson 143 rule); this probe
  annotates it explicitly and rustc accepts. Direct empirical
  witness for the lesson's "rustc spells the bound `FnMut(u32)`"
  claim — in this probe rustc accepts the user-spelled annotation
  matching that bound.
- `println!("{}", x)` formats `u32` directly — this is lesson 011's
  `{}` placeholder applied to an owned integer type, no special
  formatting work needed.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item)` | `output/docs/rust/std/iter/trait.Iterator.md:902` verbatim |
| `for_each` consumes `self` | Same line; Probe 5 empirical (E0382 with `note:` "takes ownership of the receiver `self`") |
| Returns `()` | Signature has no return-type slot; Probe 3 empirical (E0308 with `expected <T>, found ()`) |
| Closure bound is `FnMut(Self::Item)` | Same line; Probe 4 empirical (rustc spells `FnMut(u32)`) |
| `Self::Item = u32` for `Range<u32>` | Lesson 091 evidence (`Range<A>: Iterator` for `A: Step`); lesson 080 (u32 in the integer family); lesson 132 (`Self::Item` is the assoc-type slot); Probe 4 empirical; Probe 5 empirical (rustc spells the moved type `std::ops::Range<u32>`) |
| `(1..4_u32)` is a `Range<u32>` value | Lesson 091 (`a..b` produces a `Range<A>`); lesson 081 (`4_u32` pins the suffix type); Probe 5 empirical (rustc spells the type `std::ops::Range<u32>`) |
| Parens-rule: methods on a range need surrounding parens | Lesson 091 (`(1..4).rev()` evidence; without parens, E0689 fires); reused with `.for_each(...)` instead of `.rev()` |
| `sum += x` legal on `u32 += u32` | Lesson 023 (`+=` shorthand for `n = n + n`, on `let mut` integer); lesson 080 (u32); Probe 1 empirical (compiles, prints `sum = 6`) |
| `x == 2` legal on `u32 == u32` (Probe 2 body) | Lesson 013 (`==` between two integer values produces a boolean); Probe 2 transcript (compiler reads past the `if x == 2` line, fails only at the `break` token) |
| `x % 2 == 0` legal on `u32 % u32 == u32` (Check Yourself) | Lesson 037 (`%` between two integer values produces an integer); lesson 013 (`==` on integers); Probe 6 empirical (compiles silently, prints `2`) |
| Calls the closure once per element in iteration order | `trait.Iterator.md:904` "Calls a closure on each element of an iterator"; Probe 1 empirical (`1`, `2`, `3` printed in order) |
| Equivalent to a `for` loop | `trait.Iterator.md:905` verbatim "This is equivalent to using a `for` loop on the iterator" |
| `break` and `continue` are not possible from a closure | `trait.Iterator.md:905-906` verbatim; Probe 2 empirical (E0267) |
| Stabilized at 1.21.0 | `trait.Iterator.md:901` verbatim "1.21.0 ·"; toolchain is 1.95.0 |
| Bound choice of `FnMut` over `Fn` allows capture-mutating closures | Lesson 148's auto-impl rule; Probe 1's second call (`|x| sum += x` works under `FnMut`) |
| Captured binding still has to be `mut` for the closure body's mutation to be legal | Lesson 006; Probe 7 (Check Yourself (b)) — E0594 fires when `count` is not `mut` |
| Exclusive upper bound: `1..4_u32` yields `1, 2, 3` (not 4) | Lesson 091's evidence on the `..` exclusive-range form (091 cites lesson 022 in turn); Probe 1 empirical (output is `1`, `2`, `3` — `4` not printed) |

## Older supporting lessons (named only)

- 148-fn-fnmut-fnonce-distinction — Fn / FnMut / FnOnce family;
  `FnMut` choice today.
- 147-fn-trait-parenthesized-bound — `<F: FnMut(T)>` shape; today
  uses the no-return form (return defaults to `()`).
- 146-trait-bound-on-type-parameter — inline trait bound shape.
- 145-generic-function-type-parameter — `<F>` slot.
- 144-closure-captures-outer-let — capture mechanic for `sum`.
- 143-unannotated-closure-first-use — `|x|` without annotation.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 132-iterator-trait-declaration — `Self::Item` assoc-type slot.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens rule.
- 081-integer-literal-forms — `4_u32` suffix form.
- 080-integer-type-family — twelve integer types.
- 037-remainder-operator — `%` on integers; "even/odd via `n % 2 == 0`"
  was named in 037's *unlocks*.
- 023-compound-add-assign — `+=` on `let mut` integers.
- 013-comparison-operators — `==` on integers.
- 011-println-positional-args — `println!("{}", x)`.
- 006-mut-binding — `let mut count`; today's captured-mut binding.
- 005-let-binding, 003-read-rustc-diagnostic, 002-fn-main-entry-point,
  001-rustc-compile-and-run.

## Deliberate scope discipline

The orchestrator's prompt named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. Internal-iteration / `Chain` performance note
   (`trait.Iterator.md:909-911`) — implementor-side, not centered.
2. `try_for_each` — gated on the Try sub-arc (audit §4.4.4).
3. `where Self: Sized` — present but not centered, same as
   lessons 134-141.
4. `for x in (1..4_u32)` desugaring (`IntoIterator`) — gates on
   the IntoIterator sub-arc.
5. The other 26 closure-driven Iterator methods (audit §4.4.1) —
   each its own move.
6. `v.iter()` / `v.into_iter()` source shapes — both work with
   `for_each` but each composes a different `Self::Item` resolution
   into the bound. Today's `Range<u32>` source is the leanest.

Probe 4 (E0277 on non-closure argument) is recorded but not
centered in the lesson body; it rehearses lesson 147's
non-closure-argument fact rather than installing today's centered
claim. Its load-bearing role is surfacing the rustc-spelled
expected type `FnMut(u32)`, which is referenced in the lesson
body's "Why the bound is `FnMut`" section.

## Mechanics deliberately *not* smuggled (revision check)

The original draft used `Vec::iter()` as the source. Two un-installed
mechanics had been smuggled:

1. **`impl AddAssign<&u32> for u32`** (`output/docs/rust/std/primitive.u32.md:3144`,
   stable since 1.22.0). The original `sum += x` had `sum: u32, x:
   &u32` — only legal because of the std-lib `AddAssign<&u32>` impl,
   trait machinery lesson 023 explicitly deferred. **Revision: source
   is now `Range<u32>`, so `x: u32` (owned), and `sum += x` is `u32
   += u32` — pure lesson 023 mechanic, no trait sub-arc reached for.**
2. **Deref-read `*x`**. The original `*x == 20` and `*x % 2 == 0` used
   `*x` to read through the `&u32`. Lessons 047 and 048 explicitly
   deferred this mechanic. **Revision: with `x: u32` (owned), no
   deref is needed — `x == 2` and `x % 2 == 0` are direct integer
   operations.**

The revised lesson installs only what its `depends_on` actually lists.

## Run-context handoff

After this lesson lands, the orchestrator should consider the
remaining 26 closure-driven Iterator methods (audit §4.4.1) as
the next-arc plan. Likely next moves: `map` (lazy adapter,
`FnMut(Self::Item) -> B`, returns `Map<Self, F>`, rewrites yielded
element type — directly composes today's mechanic with lesson
138's enumerate frame), or `filter` (lazy adapter,
`FnMut(&Self::Item) -> bool`, returns `Filter<Self, P>`, surfaces
the *double-reference* situation rustc warns about at
`trait.Iterator.md:962-967`).
