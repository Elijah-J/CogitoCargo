# Evidence — Lesson 152: `Iterator::reduce` (fold variant; first-element init; `Option<Self::Item>` return)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/152-iterator-reduce.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/152-iterator-reduce.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/152-iterator-reduce.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet152/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145-151.

## Run context — fourth closure-driven Iterator method (audit §4.4.1, §7)

Lesson 149 installed `for_each` (consumer, `FnMut(Self::Item)`,
returns `()`). Lesson 150 installed `map` (lazy adapter,
`FnMut(Self::Item) -> B`, returns `Map<Self, F>`). Lesson 151
installed `fold` (consumer, `FnMut(B, Self::Item) -> B`, returns
`B`). Today is the fourth of the 27 closure-driven Iterator methods
named in audit §4.4.1.

Picked next per orchestrator brief and audit §7's leanness ordering:

- **Smallest structural extension of lesson 151.** `reduce` reuses
  the threading semantics installed by `fold` *unchanged*. Three
  things change in the signature simultaneously: (a) the argument
  list shrinks from `(self, init, f)` to `(self, f)`; (b) the return
  type wraps in `Option`; (c) the bound's three slots all become the
  same type, `Self::Item` (no separate `B`).
- **First Option-returning closure-driven consumer.** Lesson 119
  installed `Option<T>` with `Some(T)` / `None` and `match` on its
  variants. Lesson 131 installed `{:?}` Debug-printing of
  `Option<&u64>`. Today reuses that infrastructure to print
  `Option<u32>` from a *consuming* aggregate method's return — first
  time `Option` is the shape rustc hands back from a closure-driven
  Iterator method's call.
- **No new mechanics.** Closure parameters are owned `Self::Item`
  values (today `u32`) — no `&Self::Item`, no deref-read. The bound
  is `FnMut`, same as 148-151. The source is `Range<u32>`, same as
  149-151.
- **Composes lesson 151's threading rule with lesson 119's Option
  shape.** Today is the smallest move that lets the audience read
  any other "first-element-as-seed plus optional empty" pattern
  later (e.g. `min`, `max` — both return `Option<Self::Item>` per
  the same trait file).

## Direct prerequisite — lesson 151 (`fold` threading; multi-parameter parens-bound; consuming `self`)

Lesson 151 installed `Iterator::fold` with signature
`fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized,
F: FnMut(B, Self::Item) -> B`. Three sub-claims load-bearing today:

- The threading rule: each closure call's return value becomes the
  next call's first argument. Today's `reduce` reuses this rule
  unchanged for the *remaining* elements after the first; what
  differs is *only* the source of the *initial* accumulator value.
- The two-parameter parens-bound segment `FnMut(_, _) -> _`. Today's
  bound `FnMut(Self::Item, Self::Item) -> Self::Item` reuses this
  segment with all three positions filled by the same type.
- The consuming `self` shape. Probe 7 today empirically witnesses:
  E0382 with `note:` "`reduce` takes ownership of the receiver
  `self`, which moves `it`" — payload identical in shape to lesson
  151 Probe 6 with the method name rotated.

## Direct prerequisite — lesson 150 (wrapper-aware framing; `<F>`)

Lesson 150 installed the `<B, F>` two-type-parameter shape and the
"rustc reads `B` from somewhere" frame. Today the relevant fact is
the *negative* one: `reduce`'s signature has `<F>` only — *no* `B`.
The accumulator type *is* `Self::Item`. The orchestrator's brief
named lesson 150 for the wrapper-aware framing; today's lesson uses
that framing implicitly when comparing `fold`'s `<B, F>` to
`reduce`'s `<F>`.

## Direct prerequisite — lesson 149 (closure-driven Iterator method family)

Lesson 149 installed `Iterator::for_each` as the simplest
closure-driven Iterator method. Today's `reduce` is in the same
family: closure-driven, consumes `self`. The non-trivial return type
contrast (`for_each` → `()`; `reduce` → `Option<Self::Item>`) is
covered by today's Move section. Cited but not re-quoted.

## Direct prerequisite — lesson 148 (Fn / FnMut / FnOnce)

Today's bound is `FnMut(Self::Item, Self::Item) -> Self::Item`.
Lesson 148 installed the three-trait family with the
auto-implementation rule. Today's working closure `|acc, x| acc + x`
captures nothing and reads both parameters; under lesson 148's rule
it implements `Fn` (and so all three traits via the supertrait
relation). The bound `FnMut` accepts it. Probe 5 empirically
confirms — rustc spells the expected closure type as
`FnMut(u32, u32)`.

## Direct prerequisite — lesson 147 (parenthesized bound)

Lesson 147 installed `<F: FnMut(T) -> R>` with the parens segment
and the optional `-> R` segment. Today's bound is
`FnMut(Self::Item, Self::Item) -> Self::Item` — same parens grammar,
same arrow segment. The two-parameter form inside the parens was
installed by lesson 151; today reuses it.

## Direct prerequisite — lessons 142 + 036 (closure literal + comma-separated parameter list)

Lesson 142 installed `|param: T| body`; lesson 036 installed the
comma-separated function parameter list. Lesson 151's `|acc, x|`
already established the closure analogue. Today's `|acc, x|` is the
same shape.

## Direct prerequisite — lesson 132 (`Self::Item` slot)

Lesson 132 installed `type Item;` as the trait's associated-type
slot. Today's bound has `Self::Item` in *all three* positions inside
the parens-bound. For `Range<u32>`, `Self::Item = u32`. Probe 5
empirically witnesses: rustc's `FnMut(u32, u32)` — the second slot
that lesson 151 used for `B` is now `Self::Item` again. Probe 4's
type-pin `let s: Option<u32> = ...` confirms the return wrapping.

## Direct prerequisite — lesson 119 (`Option<T>` with Some/None)

Lesson 119 installed `Option<T>` as the prelude's sibling of
`Result<T, E>` — generic enum, two variants `Some(T)` / `None`, both
in the prelude (no `use` needed). Today is the first place the run
encounters `Option<T>` as a *return type* from a closure-driven
Iterator method. Probe 1 → `Some(6)`; Probe 2 → `None`; Probe 3 →
`Some(5)`. The Debug format produced by `{:?}` exactly reproduces
lesson 131's `Option<&u64>` Debug shape, but with `T = u32` instead.

## Direct prerequisite — lesson 131 + lesson 093 (`{:?}` on `Option<T>`)

Lesson 131 installed `{:?}` Debug-printing of `Option<&u64>` via the
direct example `println!("{:?}", iter.next())` producing `Some(10)`,
`Some(20)`, `Some(30)`, `None`. Lesson 093 grounded the `{:?}`
formatter as Debug formatter for built-in types including `Option`,
`Vec`, `String`, `Result`. Today's `println!("{:?}", s)` for `s:
Option<u32>` reuses the same formatter; output `Some(6)` /
`Some(5)` / `None` is structurally identical to lesson 131's
empirical outputs.

## Direct prerequisite — lesson 091 (Range as Iterator + parens-rule)

Lesson 091 grounds two facts load-bearing today:

- `Range<A>: Iterator` for `A: Step`. `Range<u32>` is an `Iterator`
  whose `Item = u32`. Probe 7 empirical: rustc spells the
  moved-value type as `std::ops::Range<u32>`.
- Parens-rule for method calls on a range value: `(1..4_u32).reduce(...)`
  parses with the range as the receiver of `.reduce(...)`.

## Direct prerequisite — lessons 081 + 080 (integer-literal type-suffix forms + integer family)

Lesson 081 installs `4_u32`, `1_u32`, `5_u32`, `6_u32`, `8_u32`
suffixed forms. Lesson 080 installs `u32`. The range upper bounds
(`4_u32`, `1_u32`, `6_u32`, `8_u32`) and lower bounds (`1_u32`,
`5_u32`, `7_u32`) all rely on this form to pin `Self::Item = u32`.

## Direct prerequisite — lesson 009 (`+` and `*` on integers)

Lesson 009 installed `+ - * /` between two integer values producing
a new integer value. Today's working closure body `acc + x` is `u32
+ u32` producing `u32`; the Check Yourself body `acc * x` is `u32 *
u32` producing `u32`.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's
diagnostics, all in the lesson 003 shape:

- **E0061** (Probe 6): "this method takes 1 argument but 2 arguments
  were supplied" with `help: remove the extra argument`. The
  *load-bearing* contrast diagnostic today — direct structural
  witness that `reduce`'s argument list is exactly one. Same code
  appeared in earlier function-arity lessons (036) but with
  parameter-count-mirror text. Corpus reference:
  `output/docs/rust/error_codes/E0061.md`.
- **E0277** (Probes 5 and 6): same code as lessons 146-151. Probe
  5's payload is the load-bearing one — `expected an FnMut(u32, u32)
  closure, found {integer}` — direct empirical witness for the
  homogeneous parens-bound shape (both slots `u32`, no separate
  `B`). Probe 6 also fires E0277 alongside E0061; the combined
  payload spells out both that the closure shape was wrong (with
  `0_u32` interpreted as the closure argument because rustc consumes
  the closure-position slot first) *and* that there were too many
  arguments overall.
- **E0382** (Probe 7): same code as lessons 133/134/148/149/150/151.
  Today's `note:` reads verbatim "`reduce` takes ownership of the
  receiver `self`, which moves `it`" — direct empirical witness for
  `reduce`'s consuming `self` receiver, with rustc spelling `reduce`
  by name.

No new error codes today. The Move-section structural extension is
*shape-only*; rustc's existing diagnostics suffice to pin the
contrast.

## Cited prereqs

- **Lesson 145**: `<F>` generic-function type-parameter slot. Today's
  signature has `<F>` only.
- **Lesson 143**: unannotated closure parameter `|x|` (no `: u32`).
  Today's working closure `|acc, x|` uses this form for both
  parameters; rustc reads each parameter type from the bound.
- **Lesson 144**: capture mechanic. Today's probe closures capture
  nothing.
- **Lesson 150** (cited): closure-driven Iterator method family
  context (the lazy-adapter member of the family).
- **Lesson 149** (cited): for the closure-driven Iterator method
  family.
- **Lesson 011**: `println!("{:?}", x)`.
- **Lesson 005**: `let s = ...` binding for `reduce`'s return value.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc + ./name`.

## Source — `output/docs/rust/std/iter/trait.Iterator.md` (signature, semantics, example)

The corpus file is the std doc page for `Iterator`. Verified by
reading lines 120-129 (synopsis) and 2467-2491 (full method entry).

### Lines 123-125 (synopsis-box version)

```text
fn reduce<F>(self, f: F) -> Option<Self::Item>
   where Self: Sized,
         F: FnMut(Self::Item, Self::Item) -> Self::Item { ... }
```

`{ ... }` placeholder confirms `reduce` is a *provided* method —
every `Iterator` impl gets it for free.

### Line 2469 (full signature, main entry)

```text
fn reduce<F>(self, f: F) -> Option<Self::Item> where Self: Sized,
F: FnMut(Self::Item, Self::Item) -> Self::Item,
```

Direct corpus source for the lesson body's signature. Five
load-bearing facts read from this line:

- `<F>` — *one* type parameter (different from `fold`'s `<B, F>`).
- `(self, f: F)` — receiver `self` (consuming), then *one*
  non-receiver parameter: `f: F`. No `init`.
- `-> Option<Self::Item>` — return type wrapped in `Option`. The
  type parameter to `Option` is `Self::Item`, the iterator's
  element type.
- `where Self: Sized` — same bound as `fold`/`map`/`for_each`. Not
  centered today.
- `F: FnMut(Self::Item, Self::Item) -> Self::Item` — bound; today
  centers the homogeneous closure-bound segment (all three slots
  the same type).

### Line 2467 (stabilization)

```text
1.51.0 ·
```

Stabilization at 1.51.0; well below the local toolchain 1.95.0.

### Lines 2471-2480 (semantics)

```text
Reduces the elements to a single one, by repeatedly applying a reducing
operation.

If the iterator is empty, returns `None`; otherwise, returns the
result of the reduction.

The reducing function is a closure with two arguments: an 'accumulator', and an element.
For iterators with at least one element, this is the same as `fold()`
with the first element of the iterator as the initial accumulator value, folding
every subsequent element into it.
```

Direct corpus source for the lesson body's "How `reduce` threads"
section. Four load-bearing claims read from this prose:

- "If the iterator is empty, returns `None`" — corpus source for
  Probe 2's expected output and the load-bearing reason for the
  `Option` return type.
- "otherwise, returns the result of the reduction" — corpus source
  for the `Some(_)` case in Probes 1, 3, 4, 8.
- "The reducing function is a closure with two arguments: an
  'accumulator', and an element" — corpus source for the
  two-parameter-list shape; same shape as `fold`'s.
- "For iterators with at least one element, this is the same as
  `fold()` with the first element of the iterator as the initial
  accumulator value, folding every subsequent element into it" —
  corpus source for the first-element-as-init claim and for the
  threading-of-remaining-elements semantics. Probes 1 and 3
  empirically witness; Probe 1's table walks 3 elements / 2 closure
  calls, Probe 3's single-element case walks 1 element / 0 closure
  calls.

### Lines 2484-2491 (corpus example)

```text
let reduced: i32 = (1..10).reduce(|acc, e| acc + e).unwrap_or(0);
assert_eq!(reduced, 45);

// Which is equivalent to doing it with `fold`:
let folded: i32 = (1..10).fold(0, |acc, e| acc + e);
assert_eq!(reduced, folded);
```

The corpus example uses `(1..10)` (no suffix; rustc infers `i32`
from the `let _: i32` annotation), `.unwrap_or(0)` for extracting
the result. Today's lesson keeps `Range<u32>` (lesson 091 + 081)
and `{:?}` printing instead of `unwrap_or` — the audience does not
yet have `Option::unwrap_or`. The structural witness is the same:
non-empty range reduces to a value via `+`; the `(1..10)` form would
reduce to `45` (sum of 1..=9).

### Lines 2389-2390 (named cross-reference inside fold's prose)

```text
Note: `reduce()` can be used to use the first element as the initial
value, if the accumulator type and item type is the same.
```

This is the corpus's own cross-link from `fold`'s prose to today's
method, including the homogeneous-type caveat ("if the accumulator
type and item type is the same"). Direct corpus reference for the
lesson body's "every position is the *same* type, `Self::Item`"
framing.

### Line 2493 (named deferral for try_reduce)

```text
fn try_reduce<R>( &mut self, ...
```

Followed at line 2495 by "🔬This is a nightly-only experimental
API." Corpus reference for the *What To Ignore For Now* item naming
`try_reduce`.

## Source — `output/docs/rust/std/option/enum.Option.md` (Option<T>)

Lesson 119 already established this. Reused today: `pub enum
Option<T> { None, Some(T) }`; both variants in the prelude. Today
substitutes `T = u32`. Three observed values: `Some(6)`, `Some(5)`,
`None`.

## Source — `output/docs/rust/std/ops/struct.Range.md` (Range as iterator)

Lesson 091 already established this. Reused today: `Range<A>:
Iterator where A: Step`. All twelve integer types from lesson 080
implement `Step`. `1..4_u32`, `1..1_u32`, `5..6_u32`, `7..8_u32` are
all `Range<u32>` values; their `Iterator::Item` is `u32`. Probe 7
empirical: rustc spells `std::ops::Range<u32>`. The half-open
semantics of `Range` (lesson 091) — `1..1` yields nothing because
the lower bound equals the upper — is what makes Probe 2 produce
`None`.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (auto-impl rule)

Lessons 148-151 already cited this. Today reuses without re-quoting:
capture-nothing closures implement `Fn` (and so all three traits).
Today's closures `|acc, x| acc + x` and `|acc, x| acc * x` read
both parameters, do an arithmetic op, and return; capture nothing.

## Source — `output/docs/rust/error_codes/E0061.md` (method-arity diagnostic)

Verified by reading. The error-code page describes E0061 as "an
invalid number of arguments was passed when calling a function." The
diagnostic Probe 6 produces is the application of E0061 to a method
call. Today's load-bearing payload is the inline label "method takes
1 argument" plus the help line "remove the extra argument".

## Probe 1 — working program (sum-reduce on `(1..4_u32)`)

Source: `observations/152-iterator-reduce.rs` (canonical shape;
local probe at `/tmp/eduratchet152/demo.rs`). Transcript: `PROBE 1`
block.

```rust
fn main() {
    let s = (1..4_u32).reduce(|acc, x| acc + x);
    println!("{:?}", s);
}
```

Output: `Some(6)`. Compile-exit=0, run-exit=0. Five load-bearing facts:

- The bound `F: FnMut(Self::Item, Self::Item) -> Self::Item` accepts a
  closure literal `|acc, x| acc + x` as the only non-receiver
  argument to `.reduce(...)` on `(1..4_u32)`.
- Threading: rustc takes the first element `1` as the initial
  accumulator (no closure call for that), then calls `(1, 2) → 3`,
  `(3, 3) → 6`. Final accumulator `6`. Three elements, two closure
  calls.
- Return type is `Option<Self::Item> = Option<u32>`; the lesson binds
  it to `let s` (no annotation needed; type is inferred). Probe 4's
  explicit annotation confirms.
- Wrapped in `Some(_)` because the iterator was non-empty.
- `{:?}` on `Option<u32>` Debug-prints as `Some(6)` — same shape as
  lesson 131's `Some(10)`-on-`Option<&u64>`, with `T = u32` instead.

## Probe 2 — empty iterator (load-bearing reason for `Option` return)

Source: `empty.rs`. Transcript: `PROBE 2` block.

```rust
fn main() {
    let s = (1..1_u32).reduce(|acc, x| acc + x);
    println!("{:?}", s);
}
```

Output: `None`. Compile-exit=0, run-exit=0. Three load-bearing facts:

- `(1..1_u32)` is a half-open `Range<u32>` whose lower bound equals
  its upper bound, so it yields no elements. Lesson 091 + the
  half-open semantics of `Range` (the corpus file `struct.Range.md`
  states `start..end` includes `start` and excludes `end`).
- With no first element, `reduce` has no initial accumulator. Per
  the corpus (`trait.Iterator.md:2474-2475`), it returns `None`.
- The closure was *never called* — the program ran, returned `None`,
  and printed it. The empty case is observable at runtime *before*
  any closure call.
- This probe is the structural reason the return type is
  `Option<Self::Item>` and not just `Self::Item`. Without this
  empty-case behavior, the `Option` wrapper would be unnecessary
  and the signature could match `fold`'s.

## Probe 3 — single-element iterator (zero closure calls)

Source: `single.rs`. Transcript: `PROBE 3` block.

```rust
fn main() {
    let s = (5..6_u32).reduce(|acc, x| acc + x);
    println!("{:?}", s);
}
```

Output: `Some(5)`. Compile-exit=0, run-exit=0. Two load-bearing facts:

- `(5..6_u32)` yields exactly one element, `5`. `reduce` takes it as
  the initial accumulator; no remaining elements means the closure
  runs zero times. Final accumulator `5`, wrapped in `Some(5)`.
- Sharpens the threading rule beyond Probe 1's three-elements
  case: the first element does *not* pass through the closure; it
  *becomes* the initial accumulator. Probe 1 alone could be read as
  "first call is `(0, 1)` with `0` from somewhere" if the audience
  confused `reduce` with a default-value-of-T fold; Probe 3 is the
  empirical witness that the first element does not interact with
  the closure at all.

## Probe 4 — type-pin (Option<u32> annotation)

Source: `type_pin.rs`. Transcript: `PROBE 4` block.

```rust
fn main() {
    let s: Option<u32> = (1..4_u32).reduce(|acc, x| acc + x);
    println!("{:?}", s);
}
```

Output: `Some(6)`. Compile-exit=0, run-exit=0. Two load-bearing facts:

- The annotation `Option<u32>` matches the inferred type from
  `(1..4_u32).reduce(...)`. No diagnostic; same output as Probe 1.
- Empirical confirmation that the return type is exactly
  `Option<Self::Item>` with `Self::Item = u32` for `Range<u32>`. No
  inference surprise; the type is fully determined by the source's
  `Self::Item` and the method's signature.

## Probe 5 — non-closure argument (rustc spells `FnMut(u32, u32)`)

Source: `non_closure_arg.rs`. Transcript: `PROBE 5` block.

```rust
fn main() {
    let s = (1..4_u32).reduce(7);
    println!("{:?}", s);
}
```

Output (compile-exit=1):

```text
error[E0277]: expected a `FnMut(u32, u32)` closure, found `{integer}`
 --> non_closure_arg.rs:8:31
  |
8 |     let s = (1..4_u32).reduce(7);
  |                        ------ ^ expected an `FnMut(u32, u32)` closure, found `{integer}`
  |                        |
  |                        required by a bound introduced by this call
  |
  = help: the trait `FnMut(u32, u32)` is not implemented for `{integer}`
note: required by a bound in `reduce`
```

Three load-bearing facts:

- rustc spells the expected closure type as `FnMut(u32, u32)` —
  *both* parameter slots visible inside the parens, in source order.
  Same payload shape as lesson 151 Probe 5 — but where lesson 151's
  payload was `FnMut(u32, u32)` because both `B` and `Self::Item`
  resolved to `u32` independently, today's payload is `FnMut(u32,
  u32)` because *all three* positions in the bound are
  `Self::Item = u32` to begin with. The diagnostic itself does not
  distinguish — it just shows the resolved types.
- The `note: required by a bound in \`reduce\`` block points at
  `library/core/src/iter/traits/iterator.rs:2678:4` — empirical
  witness that the bound is part of `reduce`'s declaration, not
  some auxiliary trait.
- The `-> Self::Item` return-type segment of the bound is *not*
  echoed in the diagnostic surface — same surface choice as lesson
  151 Probe 5 (rustc truncates the return-type segment in this E0277
  surface).

## Probe 6 — extra `init` argument (E0061 `method takes 1 argument`)

Source: `extra_init.rs`. Transcript: `PROBE 6` block.

```rust
fn main() {
    let s = (1..4_u32).reduce(0_u32, |acc, x| acc + x);
    println!("{:?}", s);
}
```

Output (compile-exit=1, two errors):

```text
error[E0277]: expected a `FnMut(u32, u32)` closure, found `u32`
 --> extra_init.rs:8:31
  |
8 |     let s = (1..4_u32).reduce(0_u32, |acc, x| acc + x);
  |                        ------ ^^^^^ expected an `FnMut(u32, u32)` closure, found `u32`

error[E0061]: this method takes 1 argument but 2 arguments were supplied
 --> extra_init.rs:8:24
  |
8 |     let s = (1..4_u32).reduce(0_u32, |acc, x| acc + x);
  |                        ^^^^^^        ---------------- unexpected argument #2 of type `{closure@extra_init.rs:8:38: 8:46}`
  |
note: method defined here
help: remove the extra argument
  |
8 -     let s = (1..4_u32).reduce(0_u32, |acc, x| acc + x);
8 +     let s = (1..4_u32).reduce(0_u32);
```

Four load-bearing facts:

- E0061 fires with text "this method takes 1 argument but 2
  arguments were supplied" — direct structural witness that
  `reduce`'s argument list has length one. The *load-bearing*
  diagnostic for the lesson's "no `init` argument" claim.
- The `help:` line literally shows what `reduce` *would* accept:
  `(1..4_u32).reduce(0_u32);` — a single-argument call. (Note: that
  call would itself fail because `0_u32` is not a closure, but the
  help is structural about argument count, not type correctness.)
- E0061 also produces the `unexpected argument #2 of type
  {closure@...}` label — rustc names argument #2 (the closure) as
  the unexpected one, treating `0_u32` as if it occupied the
  closure-position slot. This is consistent with E0061's behavior
  in lesson 036's function-arity examples.
- The accompanying E0277 fires on the *first* argument (`0_u32`),
  spelled "expected an `FnMut(u32, u32)` closure, found `u32`" —
  rustc has already moved on to type-checking under the assumption
  that `0_u32` was meant to be the closure. The combined diagnostic
  pair sandwiches the structural fact: argument count *and*
  argument type are wrong.

## Probe 7 — `reduce` consumes `self` (E0382 with `reduce` named)

Source: `reduce_consumes.rs`. Transcript: `PROBE 7` block.

```rust
fn main() {
    let it = 1..4_u32;
    let _s = it.reduce(|acc, x| acc + x);
    let _again = it.count();
}
```

Output (compile-exit=1):

```text
error[E0382]: use of moved value: `it`
 --> reduce_consumes.rs:9:18
  |
7 |     let it = 1..4_u32;
  |         -- move occurs because `it` has type `std::ops::Range<u32>`, which does not implement the `Copy` trait
8 |     let _s = it.reduce(|acc, x| acc + x);
  |                 ------------------------ `it` moved due to this method call
9 |     let _again = it.count();
  |                  ^^ value used here after move
  |
note: `reduce` takes ownership of the receiver `self`, which moves `it`
```

Three load-bearing facts:

- E0382. Same code as lessons 133/134/148/149/150/151's E0382
  probes. Today's `note:` reads verbatim "`reduce` takes ownership
  of the receiver `self`, which moves `it`" — direct empirical
  witness for `reduce`'s consuming `self` receiver, with rustc
  spelling `reduce` by name.
- Rustc spells the moved-value type as `std::ops::Range<u32>` —
  reconfirms that `1..4_u32` is a `Range<u32>` value (composes
  lesson 091 with lesson 081). Same payload shape as lesson 151
  Probe 6 with the method name rotated.
- The `help: you can \`clone\` the value and consume it` line is
  rustc's standard suggestion for `Copy`-less moved values. Side
  detail; not centered today.

## Probe 8 — Check Yourself (multiplicative reduce)

Source: `q.rs`. Transcript: `PROBE 8` block.

```rust
fn main() {
    let p = (1..4_u32).reduce(|acc, x| acc * x);
    println!("{:?}", p);
}
```

Output: `Some(6)`. Compile-exit=0, run-exit=0. Verifies the Check
Yourself (a) answer empirically: first element `1` becomes acc;
closure walks `(1, 2) → 2`, `(2, 3) → 6`. Final value `Some(6)`.

The Check Yourself (b) variant (`(1..1_u32)` for the same
multiplicative closure) predicts `None`; this is reasoning from
Probe 2's empty-case rule, not a separate probe — empirical risk is
zero because the empty case fires before any closure invocation.

The Check Yourself (c) variant (`(7..8_u32)` for the same
multiplicative closure) predicts `Some(7)` with zero closure calls;
this is reasoning from Probe 3's single-element rule, not a separate
probe — empirical risk is zero because the single-element case
yields the first element directly without closure invocation.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Signature `fn reduce<F>(self, f: F) -> Option<Self::Item> where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item` | `output/docs/rust/std/iter/trait.Iterator.md:2469` verbatim; synopsis at `:123-125` |
| `reduce` consumes `self` | Same line; Probe 7 empirical (E0382 with `note:` "takes ownership of the receiver `self`") |
| One non-receiver argument (`f: F`) | Same line; Probe 6 empirical (E0061 "method takes 1 argument") |
| Returns `Option<Self::Item>` | Same line; Probes 1, 3, 4 empirical (`Some(6)`, `Some(5)`, `Some(6)`); Probe 2 empirical (`None`) |
| Closure bound is `FnMut(Self::Item, Self::Item) -> Self::Item` (homogeneous, three positions same type) | Same line; Probe 5 empirical (rustc spells `FnMut(u32, u32)`); orchestrator brief restated in run-context |
| `Self::Item = u32` for `Range<u32>` | Lesson 091 (`Range<A>: Iterator` for `A: Step`); lesson 080 (u32); lesson 132 (`Self::Item` slot); Probe 4 type-pin empirical |
| First element becomes the initial accumulator | `trait.Iterator.md:2477-2480` verbatim; Probe 3 (single element → `Some(5)`, closure runs zero times); Probe 1 walkthrough |
| Closure runs once per *remaining* element | Inferred from "folding every subsequent element into it" at `trait.Iterator.md:2479-2480`; Probe 1 (3 elements → 2 closure calls) |
| Empty iterator returns `None` | `trait.Iterator.md:2474-2475` verbatim; Probe 2 empirical |
| Non-empty iterator returns `Some(final_acc)` | Same lines; Probes 1, 3, 4, 8 empirical |
| Closure body `acc + x` is `u32 + u32` | Lesson 009; Probe 1 empirical |
| Check Yourself body `acc * x` is `u32 * u32` | Lesson 009; Probe 8 empirical |
| Extra argument fires E0061 | `output/docs/rust/error_codes/E0061.md`; Probe 6 empirical |
| Non-closure argument fires E0277 with `FnMut(u32, u32)` spelling | Probe 5 empirical |
| `(1..4_u32)`, `(1..1_u32)`, `(5..6_u32)`, `(7..8_u32)` are `Range<u32>` values | Lesson 091; lesson 081; Probe 7 empirical |
| Stabilized at 1.51.0 | `trait.Iterator.md:2467` verbatim; toolchain is 1.95.0 |
| `{:?}` on `Option<u32>` prints `Some(6)` / `None` | Lesson 093 (Debug formatter), lesson 131 (`{:?}` on `Option<&u64>` produces `Some(10)` / `None`); Probes 1-4, 8 empirical |
| `try_reduce` is nightly-only | `trait.Iterator.md:2495` verbatim |

## Older supporting lessons (named only)

- 151-iterator-fold — threading semantics; multi-parameter
  parens-bound; consuming `self` shape.
- 150-iterator-map — closure-driven Iterator method family.
- 149-iterator-for-each — first closure-driven Iterator method.
- 148-fn-fnmut-fnonce-distinction — `FnMut` choice + auto-impl rule.
- 147-fn-trait-parenthesized-bound — parens-segment + arrow-segment
  bound grammar.
- 145-generic-function-type-parameter — `<F>` slot.
- 144-closure-captures-outer-let — capture mechanic (cited only).
- 143-unannotated-closure-first-use — `|x|` without annotation.
- 142-closure-literal-bound-and-called — closure literal grammar.
- 132-iterator-trait-declaration — `Self::Item` slot.
- 131-iterator-next-call — `{:?}` on `Option<&u64>` produces
  `Some(10)` / `None`.
- 119-option-some-none — `Option<T>` with `Some(T)` / `None`
  variants.
- 093-standard-library-prelude — `{:?}` Debug formatter.
- 091-range-reversal-rev — `Range<A>: Iterator`; parens-rule.
- 081-integer-literal-forms — `_u32` suffix forms.
- 080-integer-type-family — twelve integer types.
- 036-multiple-parameters — comma-separated function parameter list.
- 011-println-positional-args, 009-arithmetic-on-integers,
  005-let-binding, 003-read-rustc-diagnostic, 002-fn-main-entry-point,
  001-rustc-compile-and-run.

## Deliberate scope discipline

The orchestrator's brief named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. `try_reduce` — nightly-only experimental
   (`trait.Iterator.md:2495`); gates on the `Try` trait sub-arc.
2. `fold` vs `reduce` choice rule — when to pick one over the
   other in real code. Today contrasts the *signature*; the design
   question ("does the operation have a natural identity?") is its
   own.
3. Numerical edge cases for `reduce` on signed integers — beyond
   scope.
4. `Option::unwrap`, `Option::unwrap_or`, `Option::map`, `?` —
   separate `Option`-API moves. The corpus example uses
   `.unwrap_or(0)`; today keeps `{:?}` instead.
5. `match` on `Option<T>` *as a centered concept* — installed by
   lesson 119 but not centered today; the lesson uses `{:?}`
   throughout to keep the wrapper visible without re-centering match.
6. `v.iter()` / `v.into_iter()` source shapes — composes a different
   `Self::Item` resolution.

## Mechanics deliberately *not* smuggled

The orchestrator's reminder list called out specific smuggling risks.
Today's discipline check:

- **No `Vec`** — source is `Range<u32>` for every probe.
- **No `.iter()` or `.into_iter()`** — source is bare range.
- **No `IntoIterator`** — gated on its own sub-arc.
- **No `&u32` element types** — `Range<u32>` yields owned `u32`,
  so the closure parameters are owned `u32`. Probe 5 empirically
  confirms via `FnMut(u32, u32)`.
- **No deref-read `*x`** — closure bodies are `acc + x` and
  `acc * x`, plain owned arithmetic per lesson 009.
- **No `as` casting** — closure body produces a `u32` directly via
  `+` / `*`, no cast needed.
- **No `Box<dyn Fn>`, no `impl Fn`** — bound is named
  `FnMut(Self::Item, Self::Item) -> Self::Item`, consumed by the
  generic `<F>` slot.
- **No `move` keyword** — today's closures capture nothing.
- **No closure that captures anything** — both probes use parameters
  only; no outer `let` referenced from inside the closure body.
- **No `match` on Option<T>** — today's printing uses `{:?}` only.
  Lesson 119's `match` form is named in the prerequisite list but
  not exercised; if the audience wants to extract the value they
  can use the lesson 119 mechanic, but the lesson itself never does.

## Run-context handoff

After this lesson lands, the orchestrator's options for lesson 153
include the predicate-consumer family (each composes the closure-
driven Iterator method machinery with `Option`/`bool` returns):

- `find` — `FnMut(&Self::Item) -> bool`, returns
  `Option<Self::Item>`. First short-circuiting consumer; first place
  the audience meets the *reference closure parameter* shape
  `&Self::Item`.
- `position` — `FnMut(Self::Item) -> bool`, returns `Option<usize>`.
  Different `Option` payload type (`usize` instead of `Self::Item`).
- `any` / `all` — `FnMut(Self::Item) -> bool`, returns `bool` (not
  `Option`). Surface for short-circuit-with-bool.
- `find_map` — `FnMut(Self::Item) -> Option<B>`, returns
  `Option<B>`. Composes today's `Option` return with closure
  *body* returning `Option`.

Or back toward the lazy-adapter family:

- `inspect` — lazy version of `for_each` with bound
  `FnMut(&Self::Item)` (no return). First place the audience meets
  the *reference closure parameter* shape; lighter than `filter`.
- `filter` — lazy adapter with `FnMut(&Self::Item) -> bool`. With
  this method an end-to-end pipeline `.filter(...).map(...).fold(0,
  ...)` becomes teachable.

The orchestrator's brief explicitly names the predicate-consumer
family as "the next arc" — `find`, `find_map`, `position`, `any`,
`all`, all of which have Option/bool returns.
