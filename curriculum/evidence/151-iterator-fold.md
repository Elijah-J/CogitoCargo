# Evidence — Lesson 151: `Iterator::fold` (first multi-parameter closure-driven Iterator method)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/151-iterator-fold.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/151-iterator-fold.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/151-iterator-fold.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet151/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-150.

## Run context — third closure-driven Iterator method (audit §4.4.1, §7)

Lesson 149 installed `for_each` (a *consumer*, `FnMut(Self::Item)`,
returns `()`) and lesson 150 installed `map` (a *lazy adapter*,
`FnMut(Self::Item) -> B`, returns `Map<Self, F>`). Today is the third
of the 27 closure-driven Iterator methods named in audit §4.4.1.
Picked next per audit §7's leanness ordering, with these reasons:

- **First multi-parameter parenthesized bound.** Lesson 147 grammar
  generalizes from one parameter to many; lesson 148's *unlocks*
  explicitly named "multiple parameters `Fn(T, U) -> R`" as deferred.
  Today is the first move that lands the two-slot form.
- **Avoids the deref-read smuggle** that `filter` / `inspect` / `find`
  would introduce. With `Range<u32>` as the source, `Self::Item = u32`
  (owned). The bound `FnMut(B, u32) -> B` means both closure
  parameters are owned — no `&u32` reference, no deref, no reach for
  lessons 047/048's mechanic.
- **First non-trivial consumer return type.** `for_each` returned
  `()`; `map` returned a wrapper struct. `fold` returns `B` —
  whatever the accumulator type is. Lesson 025's implicit-return rule
  composes naturally.
- **Adds the *threading* idea.** Lesson 149's mental delta said
  "called once per element"; lesson 150's said "transforms element to
  element via the closure." Today adds: each call's return value
  becomes the *next call's first argument*. Structurally distinct
  from per-element `map`/`for_each`.

## Direct prerequisite — lesson 150 (`map`: `<B, F>`, `-> B` arrow segment)

Lesson 150 installed `Iterator::map` with signature
`fn map<B, F>(self, f: F) -> Map<Self, F> where Self: Sized,
F: FnMut(Self::Item) -> B`. Three sub-claims load-bearing today:

- The `<B, F>` two-type-parameter shape. Today's `fold<B, F>` reuses
  the same two slots in the same angle brackets.
- The `-> B` arrow segment in the parenthesized bound. Today's
  `FnMut(B, Self::Item) -> B` reuses both the parens segment and the
  arrow; what is new today is what is *inside* the parens.
- The "rustc reads `B` from somewhere" frame. Lesson 150's `B` was
  read from the closure's body. Today's `B` is read from `init` —
  the lesson's "Pass an `init` whose type disagrees" sentence depends
  on the audience already accepting "B is determined by the
  surrounding context," installed by 150.

## Direct prerequisite — lesson 149 (`for_each`: consuming `self`)

Lesson 149 installed `Iterator::for_each` with signature
`fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item)`.
The consuming `self` shape is what today's `fold` reuses (Probe 6
empirical: E0382 with `note: \`fold\` takes ownership of the receiver
\`self\``, payload identical in shape to lesson 149's Probe 5 and
lesson 150's Probe 7 with the method name rotated). The leanness
ordering inside audit §7 picks `fold` after `map` because consumers
are simpler than adapters in one direction (no wrapper struct, no
laziness) but harder in another (the *return value* of the consumer
is now non-trivial, vs. `for_each`'s `()`). Today's lesson is on
balance the smallest extension once `map`'s arrow segment is
installed.

## Direct prerequisite — lesson 148 (Fn / FnMut / FnOnce)

Today's bound is `FnMut(B, Self::Item) -> B`. Lesson 148 installed
the three-trait family with the auto-implementation rule. Today's two
working-probe closures `|acc, x| acc + x` and `|acc, x| acc * x`
capture nothing and read both parameters; under lesson 148's rule
they implement `Fn` (and so all three traits via the supertrait
relation `Fn: FnMut: FnOnce`). The bound `FnMut` accepts them. Probe
5 in the transcript empirically confirms the bound's shape — rustc
spells the expected closure type as `FnMut(u32, u32)` when a
non-closure `7` is passed.

## Direct prerequisite — lesson 147 (parenthesized bound, parens segment)

Lesson 147 installed the parenthesized form `<F: FnMut(T) -> R>` with
both the parameter-types-in-parens segment and the optional `-> R`
return-type segment. Lessons 149 (`FnMut(Self::Item)`) and 150
(`FnMut(Self::Item) -> B`) exercised the one-parameter form. Today
extends the parens segment from one slot to two:
`FnMut(B, Self::Item) -> B`. The lesson body's "Read it with the
lesson 147 grammar" walk applies lesson 147's rules — the new piece
is *inside the parens*: comma-separated multiple slots.

Lesson 147 itself does not install the multi-parameter form
explicitly; it shows the single-parameter case in its examples.
Combined with lesson 036 (next prereq), the multi-slot form is a
clean extension.

## Direct prerequisite — lessons 142 + 036 (closure literal + comma-separated parameter list)

Lesson 142 installed the closure literal `|param: T| body` — pipes
around a parameter list, then a body expression. Lesson 142's prereq
chain explicitly grounded the parameter-list shape in lesson 020's
typed-parameter `p: TYPE` form: the lesson body says "Pipes `|...|`
enclose the parameter list (same `name: TYPE` shape as function
parens)."

Lesson 036 then installed the comma-separated *function* parameter
list: "any number of parameters, each in the same `name: TYPE` shape
lesson 020 installed, separated by commas." Today's `|acc, x|` is the
straightforward composition: pipes around a comma-separated list of
two parameter names. Same `,` separator, same positional matching at
the call site (here the "call site" is rustc invoking the closure on
each element with two argument values: the threaded accumulator and
the next element). The annotations are inferred today; Probe 2
exercises the explicit form `|acc: u32, x: u32|` and confirms it
compiles to the same program.

This composition is small enough that the lesson body cites both
prereqs and treats the multi-parameter closure form as a one-line
generalization. No standalone "multi-parameter closure literal"
lesson is needed to land it.

## Direct prerequisite — lesson 132 (Iterator trait + `Self::Item`)

Lesson 132 installed `type Item;` as the trait's associated-type slot
and `Self::Item` as the path through it. Today's bound
`FnMut(B, Self::Item) -> B` reads the closure's *second* parameter
type from this slot. For `Range<u32>`, `Item = u32`. Probe 5
empirically confirms via rustc's `FnMut(u32, u32)` spelling — second
slot is `u32`, the resolved `Self::Item`.

## Direct prerequisite — lesson 091 (Range as Iterator + parens-rule)

Lesson 091 grounds two facts load-bearing today:

- `Range<A>: Iterator` for `A: Step`. `Range<u32>` is an `Iterator`
  whose `Item = u32`. Probe 6 empirical: rustc spells the moved-value
  type as `std::ops::Range<u32>`.
- Parens-rule for method calls on a range value: `(1..4_u32).fold(...)`
  parses with the range as the receiver of `.fold(...)`. Side-rehearsed
  by lessons 091/149/150's parens-rule probes; not re-probed today.

## Direct prerequisite — lessons 081 + 080 (integer-literal type-suffix forms + integer family)

Lesson 081 installs `4_u32`, `0_u32`, `1_u32`, `100_u32` etc. — the
type-suffix form. Today's `init` literals `0_u32`, `100_u32`, `1_u32`
all rely on this form to pin `B = u32`. The range upper bound
`4_u32` pins `Self::Item = u32`. Lesson 080 installs the twelve
integer type names, including `u32` and `i32` (the `i32` is used by
Probe 4's contrast `init = 0_i32`).

## Direct prerequisite — lesson 025 (implicit-return rule)

Lesson 025 installed the rule "a body's last expression *is* the
return value when there is no `;`." Today's closure body `acc + x`
(no `;`) is a single expression whose value is the closure's return
— that is what makes `B` flow back through the closure correctly.
Without 025, the closure body would have to be wrapped in `{ ... }`
with an explicit `return`.

## Direct prerequisite — lesson 009 (`+` and `*` on integers)

Lesson 009 installed `+ - * /` between two integer values. Today's
working closure body is `acc + x` (`u32 + u32` → `u32`); the Check
Yourself body is `acc * x` (`u32 * u32` → `u32`). No new arithmetic
mechanic. Source-shape choice (`Range<u32>` over `Vec::iter()`)
ensures both sides of `+` and `*` are owned `u32` — pure lesson 009
mechanic, no AddAssign-on-reference, no deref-read.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's reappearing
codes plus one new code:

- **E0593** (Probe 3): *new* today. "closure is expected to take 2
  arguments, but it takes 1 argument" — the bound's parens segment
  is structurally a count, and rustc verifies the closure's parameter
  list has the matching number of slots. The diagnostic shape is the
  lesson 003 four-part map: headline `error[E0593]`, location at
  `wrong_arity.rs:5:24` (on the `fold` method name), source excerpt
  with two annotations on the same line — `^^^^` under the method
  name labelled `expected closure that takes 2 arguments` and `---`
  under the closure literal labelled `takes 1 argument`. Corpus
  reference: `output/docs/rust/error_codes/E0593.md`.
- **E0308** (Probe 4): same code as lessons 138/149/150. Today's
  payload is `expected i32, found u32` on the `+` body's value
  flowing back into `B`.
- **E0277** (Probes 4 and 5): same code as lessons 146-150. Probe 5's
  payload is the load-bearing one — `expected an FnMut(u32, u32)
  closure, found {integer}` — direct empirical witness for the
  *two-parameter* parenthesized bound shape.
- **E0382** (Probe 6): same code as lessons 133/134/148/149/150.
  Today's payload's `note:` reads "`fold` takes ownership of the
  receiver `self`, which moves `it`" — direct empirical witness for
  `fold`'s consuming `self` receiver, with rustc spelling `fold` by
  name.

## Cited prereqs

- **Lesson 145**: `<F>` generic-function type-parameter slot. Today
  uses `<B, F>` — same two-slot shape as lesson 150.
- **Lesson 143**: unannotated closure parameter `|x|` (no `: u32`).
  Today's two probe closures `|acc, x|` use this form for both
  parameters; rustc reads each parameter type from the bound. Probe 2
  confirms by accepting the explicit `|acc: u32, x: u32|` form.
- **Lesson 144**: capture mechanic. Today's probe closures capture
  nothing — both parameters come from the iteration mechanism, not
  from outer scope. Lesson 144 cited but not load-bearing.
- **Lesson 020**: typed parameter `p: TYPE`. Probe 2's explicit
  closure `|acc: u32, x: u32|` reuses lesson 020's annotation form,
  twice, comma-separated.
- **Lesson 011**: `println!("{}", x)`.
- **Lesson 005**: `let s = ...` binding for `fold`'s return value.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc + ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics, table)

The corpus file is the std doc page for `Iterator`. Verified by
reading.

### Line 2365 (full signature, main entry)

```text
#### fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized, F: FnMut(B, Self::Item) -> B,
```

Direct corpus source for the lesson body's signature. Five
load-bearing facts read from this line:

- `<B, F>` — two type parameters (same as `map`).
- `(self, init: B, f: F)` — receiver `self` (consuming), then *two*
  non-receiver parameters: `init: B` and `f: F`.
- `-> B` — return type is `B` (the accumulator type).
- `where Self: Sized, F: FnMut(B, Self::Item) -> B` — the bound;
  today centers the closure-bound segment.
- `FnMut(B, Self::Item) -> B` — parenthesized bound (lesson 147)
  with `FnMut` (148), two parameter slots inside the parens (today's
  novel sub-fact), and `-> B` (lesson 150).

### Lines 120-122 (synopsis-box version)

```text
fn fold<B, F>(self, init: B, f: F) -> B
   where Self: Sized,
         F: FnMut(B, Self::Item) -> B { ... }
```

Same signature in the synopsis box at the top of the file. The
`{ ... }` placeholder confirms `fold` is a *provided* method —
every `Iterator` impl gets it for free.

### Line 2363 (stabilization)

```text
1.0.0 ·
```

Stabilization at 1.0.0; far below the local toolchain 1.95.0.

### Lines 2367-2378 (semantics: takes-and-returns prose)

```text
Folds every element into an accumulator by applying an operation,
returning the final result.

`fold()` takes two arguments: an initial value, and a closure with two
arguments: an 'accumulator', and an element. The closure returns the value that
the accumulator should have for the next iteration.

The initial value is the value the accumulator will have on the first
call.

After applying this closure to every element of the iterator, `fold()`
returns the accumulator.
```

Direct corpus source for the lesson body's "How `fold` threads the
accumulator" section. Five load-bearing claims read from this:

- `fold` "takes two arguments: an initial value, and a closure with
  two arguments" → matches the lesson body's "two non-receiver
  arguments" + "**two** parameter slots" framing.
- "The closure returns the value that the accumulator should have
  for the next iteration" → corpus source for the *threading*
  semantics: each call's return value becomes next call's first
  argument.
- "The initial value is the value the accumulator will have on the
  first call" → corpus source for "init's role." Probe 2's `init =
  100_u32` → output `106` empirically witnesses.
- "After applying this closure to every element of the iterator,
  `fold()` returns the accumulator" → corpus source for the return
  value being the *final* accumulator.

### Lines 2410-2427 (corpus example + walkthrough table)

```text
let a = [1, 2, 3];

// the sum of all of the elements of the array
let sum = a.iter().fold(0, |acc, x| acc + x);

assert_eq!(sum, 6);
```

Followed by:

```text
| element | acc | x | result |
| --- | --- | --- | --- |
|  | 0 |  |  |
| 1 | 0 | 1 | 1 |
| 2 | 1 | 2 | 3 |
| 3 | 3 | 3 | 6 |
```

The lesson body's iteration table (Section "How `fold` threads the
accumulator") is structurally the same — same numeric values, same
walkthrough — adapted to today's `Range<u32>` source instead of the
corpus's `[1, 2, 3].iter()` array source. The corpus's source yields
`&i32` (because `.iter()` on an array yields references to elements),
not owned values; today's source yields owned `u32` directly. The
arithmetic is identical, and the walkthrough is unchanged in shape.

### Lines 2392-2395 (left-associativity, named deferral for rfold)

```text
Note: `fold()` combines elements in a *left-associative* fashion.
For associative operators like `+`, the order the elements are
combined in is not important, but for non-associative operators like
`-` the order will affect the final result.
For a *right-associative* version of `fold()`, see [`DoubleEndedIterator::rfold()`].
```

Corpus reference for the *What To Ignore For Now* item naming
`rfold`. Today's body uses `+` (associative), so left-vs-right does
not change the answer; left-associativity is named only as the
default, not centered.

### Lines 2389-2390 (named deferral for reduce)

```text
Note: `reduce()` can be used to use the first element as the initial
value, if the accumulator type and item type is the same.
```

Corpus reference for the *What To Ignore For Now* item naming
`reduce`. Lesson 151 explicitly defers; lesson 162-ish or so will
land it.

### Lines 2397-2404 (Note to Implementors)

```text
Several of the other (forward) methods have default implementations in
terms of this one, so try to implement this explicitly if it can
do something better than the default `for` loop implementation.
```

Implementor-side; explicitly deferred in *What To Ignore For Now*.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Reused today: `Range<A>:
Iterator where A: Step`. All twelve integer types from lesson 080
implement `Step`. `1..4_u32` is a `Range<u32>` and its
`Iterator::Item` is `u32`. Probe 6 empirical: rustc spells
`std::ops::Range<u32>`.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lessons 148, 149, 150 already cited this. Today reuses without
re-quoting: capture-nothing closures implement `Fn` (and so all three
traits). Today's closures `|acc, x| acc + x` and `|acc, x| acc * x`
read both parameters, do an arithmetic op, and return; capture
nothing.

## Source — `output/docs/rust/error_codes/E0593.md` (closure-arity diagnostic)

Verbatim from `output/docs/rust/error_codes/E0593.md` lines 4-5:

```text
You tried to supply an `Fn`-based type with an incorrect number of arguments
than what was expected.
```

Followed by an erroneous-code example with `fn foo<F: Fn()>(x: F)`
and a one-arg closure `foo(|y| { });` triggering "closure takes 1
argument but 0 arguments are required." Today's Probe 3 is the
mirror image — the bound expects 2 arguments and the closure
supplies 1.

## Probe 1 — working program (sum-fold on `(1..4_u32)`)

Source: `observations/151-iterator-fold.rs`. Transcript: `PROBE 1`
block.

```rust
fn main() {
    let s = (1..4_u32).fold(0_u32, |acc, x| acc + x);
    println!("{}", s);
}
```

Output: `6`. Compile-exit=0, run-exit=0. Five load-bearing facts:

- The bound `F: FnMut(B, Self::Item) -> B` accepts a closure literal
  `|acc, x| acc + x` as the second argument to `.fold(...)` on
  `(1..4_u32)`.
- The `init` argument `0_u32` fixes `B = u32`. The closure parameters
  `acc` and `x` are both `u32`; the body returns `u32`.
- Threading: rustc starts with `acc = 0`, calls `(0, 1) → 1`,
  `(1, 2) → 3`, `(3, 3) → 6`. Final accumulator = 6 = `fold`'s return
  value.
- Return type is `B = u32`; the lesson binds it to `let s` (no
  annotation needed; type is inferred from `B = u32`).
- `(1..4_u32)` is a `Range<u32>`. Probe 6 confirms via E0382's
  spelling.

## Probe 2 — init-offset witness (init=100_u32, explicit annotations)

Source: `init_offset.rs`. Transcript: `PROBE 2` block.

```rust
fn main() {
    let s = (1..4_u32).fold(100_u32, |acc: u32, x: u32| acc + x);
    println!("{}", s);
}
```

Output: `106`. Compile-exit=0, run-exit=0. Three load-bearing facts:

- `init` is genuinely included in the result. `100 + 1 + 2 + 3 = 106`
  is the only consistent interpretation. The corpus claim "the
  initial value is the value the accumulator will have on the first
  call" (`trait.Iterator.md:2374-2375`) is empirically witnessed.
- The explicit `|acc: u32, x: u32|` annotation form is accepted —
  composes lesson 020's typed-parameter shape, twice, comma-separated
  inside the closure pipes (the same shape lesson 142 cited for
  single-parameter closures).
- The closure call count (3 times for the three elements `1, 2, 3`)
  is the same as Probe 1 — the lesson body's "same iteration count"
  claim in *Try It* is empirical.

## Probe 3 — wrong closure arity (one parameter instead of two) — E0593

Source: `wrong_arity.rs`. Transcript: `PROBE 3` block.

```rust
fn main() {
    let s = (1..4_u32).fold(0_u32, |x| x);
    println!("{}", s);
}
```

Output (compile-exit=1):

```text
error[E0593]: closure is expected to take 2 arguments, but it takes 1 argument
 --> wrong_arity.rs:5:24
  |
5 |     let s = (1..4_u32).fold(0_u32, |x| x);
  |                        ^^^^        --- takes 1 argument
  |                        |
  |                        expected closure that takes 2 arguments
```

Three load-bearing facts:

- E0593 fires. *New* error code today. The bound's parens segment is
  a *count*: rustc verifies the closure's parameter list has the
  matching number of slots. A one-parameter closure is rejected at
  compile time.
- The diagnostic has *two* annotations on the same source line —
  `^^^^` under the method name labelled `expected closure that takes
  2 arguments`, and `---` under the closure literal labelled `takes 1
  argument`. Same lesson 003 four-part shape; the under-line label
  pattern is similar to lesson 036's E0061 (function-arity).
- Mirror image of the corpus E0593 example (`output/docs/rust/error_codes/E0593.md`):
  the corpus shows the inverse direction (bound expects 0, closure
  supplies 1); today the bound expects 2 and the closure supplies 1.

## Probe 4 — init type fixes B (init=0_i32 against u32 source) — E0308 + E0277

Source: `init_type_mismatch.rs`. Transcript: `PROBE 4` block.

```rust
fn main() {
    let s = (1..4_u32).fold(0_i32, |acc, x| acc + x);
    println!("{}", s);
}
```

Output (compile-exit=1, two errors):

```text
error[E0308]: mismatched types
 --> init_type_mismatch.rs:6:51
  |
6 |     let s = (1..4_u32).fold(0_i32, |acc, x| acc + x);
  |                                                   ^ expected `i32`, found `u32`

error[E0277]: cannot add `u32` to `i32`
 --> init_type_mismatch.rs:6:49
  |
6 |     let s = (1..4_u32).fold(0_i32, |acc, x| acc + x);
  |                                                 ^ no implementation for `i32 + u32`
```

Four load-bearing facts:

- `init`'s type fixes `B`. `init = 0_i32` makes `B = i32`. The
  closure parameter `acc` is therefore `i32`.
- `Self::Item` is *separate*: the source is `(1..4_u32)`, so
  `Self::Item = u32`. The closure parameter `x` is `u32`.
- Inside the body `acc + x` is `i32 + u32`, which is not a valid
  operation. Both `+` operands must be the same type per `Add`'s
  default impl set; rustc fires E0277 (`cannot add u32 to i32`).
- E0308 also fires on the body's value flowing back into `B`:
  `expected i32, found u32` — even *if* the body produced a `u32`
  somehow, that would not match the declared `B = i32`.
- The structural takeaway: change `init` and the entire closure's
  expected shape changes with it. `B` is *not* free — it is whatever
  `init` says, and the closure must agree on both ends (parameter
  type + return type).

## Probe 5 — non-closure argument — E0277 with rustc spelling FnMut(u32, u32)

Source: `non_closure_arg.rs`. Transcript: `PROBE 5` block.

```rust
fn main() {
    let s = (1..4_u32).fold(0_u32, 7);
    println!("{}", s);
}
```

Output (compile-exit=1):

```text
error[E0277]: expected a `FnMut(u32, u32)` closure, found `{integer}`
 --> non_closure_arg.rs:5:36
  |
5 |     let s = (1..4_u32).fold(0_u32, 7);
  |                        ----        ^ expected an `FnMut(u32, u32)` closure, found `{integer}`
  |                        |
  |                        required by a bound introduced by this call
```

Three load-bearing facts:

- rustc spells the expected closure type as `FnMut(u32, u32)` —
  *both* parameter slots visible inside the parens, in source order.
  Direct empirical witness for the lesson body's "two parameter
  slots, comma-separated" claim. Same payload shape as lesson 149
  Probe 4 / lesson 150 Probe 6 with the second slot added.
- The bound's parens segment is rustc-spelled with both types
  present; the `-> B` segment is *not* echoed (same surface choice
  as lesson 150 Probe 6 — rustc truncates the return-type segment
  in this E0277 surface).
- The `note: required by a bound in \`fold\`` block (truncated in
  the lesson body excerpt for brevity) points at the bound itself in
  the function's declaration in `library/core/src/iter/traits/iterator.rs:2640:4`.

## Probe 6 — `fold` consumes `self` — E0382 with `fold` named in note

Source: `fold_consumes.rs`. Transcript: `PROBE 6` block.

```rust
fn main() {
    let it = 1..4_u32;
    let _s = it.fold(0_u32, |acc, x| acc + x);
    let _again = it.count();
}
```

Output (compile-exit=1):

```text
error[E0382]: use of moved value: `it`
 --> fold_consumes.rs:8:18
  |
6 |     let it = 1..4_u32;
  |         -- move occurs because `it` has type `std::ops::Range<u32>`, which does not implement the `Copy` trait
7 |     let _s = it.fold(0_u32, |acc, x| acc + x);
  |                 ----------------------------- `it` moved due to this method call
8 |     let _again = it.count();
  |                  ^^ value used here after move
  |
note: `fold` takes ownership of the receiver `self`, which moves `it`
```

Three load-bearing facts:

- E0382. Same code as lessons 133/134/148/149/150's E0382 probes.
  Today's `note:` reads verbatim "`fold` takes ownership of the
  receiver `self`, which moves `it`" — direct empirical witness for
  `fold`'s consuming `self` receiver, with rustc spelling `fold` by
  name.
- Rustc spells the moved-value type as `std::ops::Range<u32>` —
  direct empirical witness that `1..4_u32` is a `Range<u32>` value
  (composes lesson 091 with lesson 081). Same payload as lesson 149
  Probe 5 and lesson 150 Probe 7.
- The `help: you can \`clone\` the value and consume it` line is
  rustc's standard suggestion for `Copy`-less moved values. Side
  detail; not centered today.

## Probe 7 — Check Yourself (multiplication)

Source: `q.rs`. Transcript: `PROBE 7` block.

```rust
fn main() {
    let p = (1..4_u32).fold(1_u32, |acc, x| acc * x);
    println!("{}", p);
}
```

Output: `6`. Compile-exit=0, run-exit=0. Verifies the Check Yourself
(a) answer empirically: `1*1=1`, `1*2=2`, `2*3=6`. The Check Yourself
(b) variant (`init = 0_u32` for the same multiplicative closure)
predicts `0`; this is reasoning from the threading rule installed
today, not a separate probe — empirical risk is low because the
arithmetic is direct.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized, F: FnMut(B, Self::Item) -> B` | `output/docs/rust/std/iter/trait.Iterator.md:2365` verbatim |
| `fold` consumes `self` | Same line; Probe 6 empirical (E0382 with `note:` "takes ownership of the receiver `self`") |
| Two non-receiver arguments (`init: B`, `f: F`) | Same line |
| Returns `B` | Same line |
| Closure bound is `FnMut(B, Self::Item) -> B` (two parameter slots) | Same line; Probe 5 empirical (rustc spells `FnMut(u32, u32)`) |
| `Self::Item = u32` for `Range<u32>` | Lesson 091 (`Range<A>: Iterator` for `A: Step`); lesson 080 (u32); lesson 132 (`Self::Item` slot); Probe 5 + Probe 6 empirical |
| `init`'s type fixes `B` | Signature (`init: B`); Probe 4 empirical (`init = 0_i32` → `B = i32` → E0308 + E0277) |
| Each call's return becomes next call's first argument (threading) | `trait.Iterator.md:2370-2378` verbatim "The closure returns the value that the accumulator should have for the next iteration"; `trait.Iterator.md:2421-2427` walkthrough table; Probe 1 (output `6`); Probe 2 (output `106` — `init` included) |
| Initial value is what `acc` *starts at* | `trait.Iterator.md:2374-2375` verbatim; Probe 2 empirical |
| Final accumulator is `fold`'s return value | `trait.Iterator.md:2377-2378` verbatim; Probe 1 + Probe 2 empirical |
| Closure body `acc + x` is `u32 + u32` | Lesson 009; Probe 1 empirical (output `6`) |
| Check Yourself body `acc * x` is `u32 * u32` | Lesson 009; Probe 7 empirical (output `6`) |
| Wrong-arity closure fires E0593 | `output/docs/rust/error_codes/E0593.md`; Probe 3 empirical |
| Init type mismatch fires E0308 + E0277 | Probe 4 empirical |
| `(1..4_u32)` is a `Range<u32>` value | Lesson 091; lesson 081; Probe 6 empirical |
| Stabilized at 1.0.0 | `trait.Iterator.md:2363` verbatim; toolchain is 1.95.0 |

## Older supporting lessons (named only)

- 150-iterator-map — `<B, F>` two-type-parameter shape; `-> B` arrow
  in the bound; rustc-reads-`B`-from-context frame.
- 149-iterator-for-each — consuming `self` shape.
- 148-fn-fnmut-fnonce-distinction — `FnMut` choice + auto-impl rule.
- 147-fn-trait-parenthesized-bound — parens-segment + arrow-segment
  bound grammar.
- 146-trait-bound-on-type-parameter — inline trait bound shape.
- 145-generic-function-type-parameter — `<F>` slot, today extended
  to `<B, F>`.
- 144-closure-captures-outer-let — capture mechanic (cited only;
  today's probe closures capture nothing).
- 143-unannotated-closure-first-use — `|x|` without annotation.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 132-iterator-trait-declaration — `Self::Item` slot.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens-rule.
- 081-integer-literal-forms — `0_u32`, `1_u32`, `4_u32`, `100_u32`,
  `0_i32` suffix forms.
- 080-integer-type-family — twelve integer types.
- 036-multiple-parameters — comma-separated function parameter list,
  the host shape lesson 142's "parameter list inside `|...|`"
  generalizes to today.
- 025-implicit-return — closure body's last expression is the return.
- 020-function-with-parameter — typed parameter `p: TYPE` shape
  (Probe 2 explicit annotations).
- 011-println-positional-args, 009-arithmetic-on-integers,
  005-let-binding, 003-read-rustc-diagnostic, 002-fn-main-entry-point,
  001-rustc-compile-and-run.

## Deliberate scope discipline

The orchestrator's prompt named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. `try_fold` — gates on the `Try` trait sub-arc (audit §4.4.4).
2. `reduce` — variant that uses the first element as init; returns
   `Option<Self::Item>`. Separate move (`trait.Iterator.md:2469`).
3. `rfold` — right-associative version on `DoubleEndedIterator`.
   Gates on that trait sub-arc (audit §4.4.5);
   `trait.Iterator.md:2392-2395` names it.
4. Accumulator type different from element type — today keeps
   `B = Self::Item = u32` for surface minimality.
5. Right-fold / `fold` from-the-right semantics — beyond scope; gated
   on `DoubleEndedIterator`.
6. Implementations for specific source iterators — implementor-side.
7. Internal-iteration / "implement other forward methods in terms of
   `fold`" — `trait.Iterator.md:2397-2404`, implementor-side.

## Mechanics deliberately *not* smuggled

The orchestrator's reminder list called out specific smuggling risks.
Today's discipline check:

- **No `Vec`** — source is `Range<u32>` for every probe.
- **No `.iter()` or `.into_iter()`** — source is bare range.
- **No `IntoIterator`** — gated on its own sub-arc.
- **No `&u32` element types** — `Range<u32>` yields owned `u32`,
  so both closure parameters (after `B = u32` from `init`) are
  owned `u32`. Probe 5 empirically confirms via `FnMut(u32, u32)`.
- **No deref-read `*x`** — closure bodies are `acc + x` and
  `acc * x`, plain owned arithmetic per lesson 009.
- **No `as` casting** — closure body produces a `u32` directly via
  `+` / `*`, no cast needed. Probe 4 empirically confirms that the
  type-mismatch case fires E0308/E0277 rather than auto-casting.
- **No `Box<dyn Fn>`, no `impl Fn`** — bound is named `FnMut(B, Self::Item) -> B`,
  consumed by the generic `<F>` slot.
- **No `move` keyword** — today's closures capture nothing.
- **No closure that captures anything** — both probes use parameters
  only; no outer `let` referenced from inside the closure body.

## Run-context handoff

After this lesson lands, the orchestrator's options for lesson 152
include:

- `inspect` — lazy version of `for_each` with bound `FnMut(&Self::Item)`
  (no return). First place the audience meets the *reference closure
  parameter* shape `&Self::Item`; lighter than `filter` because no
  predicate decision.
- `filter` — lazy adapter with `FnMut(&Self::Item) -> bool`. Surfaces
  the *double-reference* situation `trait.Iterator.md:962-967` warns
  about (closure receives `&Self::Item`, which for slice-iter is
  `&&T` — but for `Range<u32>` it is `&u32`, the simpler case).
- `find` / `any` / `all` / `position` — short-circuiting consumers
  with predicate-shape closures.
- `reduce` — fold variant that uses the first element as init;
  composes today's mechanic with `Option<Self::Item>` return type
  for the empty-iterator case.

The audit's "intermediate iterator-pipeline capstone" (§7) becomes
plausible after `filter` lands — then a chain
`(1..N_u32).filter(...).map(...).fold(0, ...)` reads end-to-end in
the audience's installed mechanics.
