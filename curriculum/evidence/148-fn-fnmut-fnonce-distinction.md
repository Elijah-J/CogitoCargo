# Evidence — Lesson 148: Fn / FnMut / FnOnce distinction (closure sub-arc closer)

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/148-fn-fnmut-fnonce-distinction.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/148-fn-fnmut-fnonce-distinction.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/148-fn-fnmut-fnonce-distinction.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet148/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145, 146,
147.

## Run context — closure sub-arc closer (audit §6 step 5)

Per `iterator-api-coverage.md` §6, the closure sub-arc has five
steps. Steps 1-4 (lessons 142-147) installed:

1. Closure literal `|p: T| body` bound and called (lesson 142).
2. Unannotated closure first-call type-fixing (lesson 143).
3. Closure capturing an outer `let` (lesson 144).
4. Generic function with parenthesized Fn-trait bound and
   closure-as-argument, decomposed across lessons 145 / 146 / 147
   (generic syntax / inline trait bound / parenthesized Fn-family
   bound + closure-as-argument).

Today is step 5: install the **three-trait distinction**, the
**supertrait relationship**, the **auto-implementation rule**, and
the **`mut f: F` side-fact** for FnMut bounds. With this, the
closure sub-arc prereqs are complete; the 27 closure-driven Iterator
methods (audit §4.4.1) become teachable, starting with `for_each`
or `map`.

The audit document itself is not updated here; that is an
orchestrator action after 148 lands and is committed.

## Direct prerequisite — lesson 147 (parenthesized Fn-family bound)

Lesson 147 installed:

- The shape `<F: Fn(T) -> R>`: a parenthesized argument list and
  optional `-> R` segment after the trait name `Fn`, inside the
  same angle brackets as lesson 146's `<T: TRAIT>` form.
- A function carrying that bound accepts any closure literal whose
  parameter and return shape matches.
- The body can call the bound parameter with parens: `f(x)`.

Today extends this exact mechanic with three new facts:

1. There are *three* traits in the family, all of which use the
   same parenthesized bound grammar: `Fn(T) -> R`, `FnMut(T) -> R`,
   `FnOnce(T) -> R`.
2. The traits are layered: `Fn: FnMut: FnOnce`. So `Fn` is the
   strictest bound, `FnOnce` the most permissive.
3. A closure auto-implements one or more based on what its body
   does with captured values; the bound on the receiving function
   constrains which closures are accepted.

Lesson 147's *What To Ignore For Now* section explicitly named
today's lesson: "The `Fn` / `FnMut` / `FnOnce` distinction — three
traits in the family with different rules on call-once-vs-repeated
and capture mutation. Lesson 148, the closure sub-arc closer."

Lesson 147 used only `Fn` (the strictest trait). Side-probe D in
lesson 147's evidence empirically witnessed that picking `FnMut`
instead would force `mut f: F` — the side-fact today installs as
the third centered claim.

## Direct prerequisite — lesson 144 (closure captures outer `let`)

Lesson 144 installed:

- A closure body may reference a name from the enclosing scope.
  The closure has *captured* the binding.
- A `fn` item declared in the same place cannot do this; the
  attempt fires E0434.

Today's centered fact is what *kind* of capture the closure does:
read by shared reference (Fn), borrow mutably and mutate (FnMut),
or move out of the closure (FnOnce). Lesson 144's
*What To Ignore For Now* explicitly named today: "**Capture mode
classification** — by shared reference vs mutable reference vs
move. ... Closure sub-arc steps 4-5."

Probe 1's `tick` closure is the exact textual extension of
lesson 144's mechanic: it captures `counter` from the enclosing
scope. The new fact is that the body's `counter += n` is what
forces it into FnMut.

## Direct prerequisite — lesson 006 (`let mut`)

Lesson 006 installed `let mut name = value;` as the
reassignable-binding form. Today extends `mut` to *function
parameter* slots: `mut f: F` in `call_fnmut`'s signature is
structurally the same `mut` keyword, slotted into lesson 020's
parameter position.

The probe also uses lesson 006's binding form directly:
`let mut counter: u32 = 0;` is exactly lesson 006's shape (with
lesson 062's optional type annotation), and `counter += n` inside
the closure body is the reassignment-style update lesson 006
installed (compound-assignment is a small extension of `name = expr`
which lesson 062 / 080 examples use without centering).

The two surfaces compose: `let mut counter: u32 = 0;` enables
mutation through the captured name; `mut f: F` enables the body of
`call_fnmut` to call `f(x)`. Without either `mut`, rustc rejects:

- Without `mut counter`, the closure body's `counter += n` would
  fire E0594 ("cannot assign to ... immutable") — not centered today.
- Without `mut f: F`, the body's `f(x)` fires E0596 — Probe 3.

Lesson 144's *What To Ignore For Now* named "Mutable capture —
`let mut count = 0; let mut bump = || count += 1;`" as deferred to
the closure sub-arc. Today is where it lands, motivated by the
three-trait distinction.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. Today's new
codes:

- **E0525** (Probes 2, 5, 8): trait mismatch on the bound side.
  Inline label "expected a closure that implements the `Fn` trait,
  but this closure only implements `FnMut`" — rustc names the
  expected and found traits explicitly. The diagnostic includes a
  primary span on the closure literal with an annotated label
  naming the captured variable (`counter`, `s`, `v`) and the action
  that forced the more restrictive trait ("mutates", "moves out").

- **E0596** (Probe 3): `cannot borrow \`f\` as mutable, as it is not
  declared as mutable`. Same code lesson 131 first witnessed for
  `&mut self` on `next()`; today's payload is at function-parameter
  position, with `+++` markers proposing the missing `mut`.

- **E0382** (Probe 6, side-noted): `use of moved value: \`f\``
  with `note: \`FnOnce\` closures can only be called once`. Same
  code lesson 133 first witnessed for `count(self)` consuming the
  iterator; today's payload is on a parameter typed `F: FnOnce(...)`
  with the `note:` block tying the move to the trait.

Each diagnostic carries the lesson 003 four-part shape unchanged
(headline + `-->` + source excerpt + `note:`/`help:`). The new
feature is the *kind* of mismatch surfaced — closure-trait mismatch
at the bound, parameter-binding mutability at the body, by-value
consumption at the call.

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 146**: inline trait bound `<T: TRAIT>`. Today reuses
  `<F: TRAIT(...)>` shape three times.
- **Lesson 145**: generic function `fn name<T>(t: T)`. Today reuses
  the `<F>` slot three times.
- **Lesson 142**: closure literal `|p: T| body`. Today's three
  closures (`pure`, `tick`, `consume`) are all closure literals.
- **Lesson 094**: the `_` wildcard discard pattern. The `_` in
  `move |_: u32|` (Probes 5/6/7 and the lesson body's FnOnce probe)
  reuses lesson 094's `_` wildcard meaning at a closure-parameter
  host: the parameter slot exists but the body doesn't use it.
- **Lesson 042**: `String::new()`. The FnOnce probe uses
  `String::from("hello")` instead — same `Type::function(args)`
  shape lesson 042 installed for `String::new()`, with one argument
  rather than zero. Lesson 042 names `String::from` in its
  *What To Ignore For Now* section as a deferred alternative
  constructor; today's use is the smallest application — produce a
  non-`Copy` value so `drop(s)` actually moves `s`. The string
  contents `"hello"` are otherwise irrelevant to the lesson's
  centered claim. (`bool`, `Vec<u32>::new()`, `String::new()`, or
  any other non-`Copy` value would witness the same FnOnce-only
  behavior; the Check Yourself probe uses `Vec<u32>::new()` to
  generalize.)
- **Lesson 081**: `0_u32` literal suffix form. The `0_u32` in the
  FnOnce probe's body provides a `u32` return value that matches
  the `Fn(u32) -> u32` shape of `call_fnonce`'s bound.
- **Lesson 020**: `f: F` parameter slot. Today extends with `mut f: F`.
- **Lesson 008**: `f(x)` parens-call shape. Reused inside all three
  call helpers.
- **Lesson 080**: `u32` named integer type.
- **Lessons 011, 005, 002, 001**: `println!`, `let`, `fn main`,
  `rustc file.rs`, `./name`.

## Source — `output/docs/rust/std/ops/trait.Fn.md` (Fn supertrait of FnMut, auto-impl rule)

The corpus file `output/docs/rust/std/ops/trait.Fn.md` is the trait
page for `Fn`. Verified by reading.

### Lines 7-15 (trait header showing supertrait)

```text
pub trait Fn<Args>: FnMut<Args>

where
    Args: Tuple,

{
    // Required method
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
```

The `: FnMut<Args>` after `pub trait Fn<Args>` is the supertrait
declaration: every type that implements `Fn` also implements
`FnMut`. The required method `call(&self, args: Args)` confirms the
shared-borrow receiver. Verified at lines 7-15.

### Line 27 (Fn auto-implemented)

```text
`Fn` is implemented automatically by closures which only take
immutable references to captured variables or don't capture
anything at all, ...
```

Direct corpus source for the lesson body's "no capture or read by
shared reference → `Fn`." The lesson body's "captures nothing"
shorthand maps to "don't capture anything at all"; "reads captures
by shared reference" maps to "only take immutable references to
captured variables." Verified at line 27.

### Lines 33-35 (subtrait/supertrait practical consequence)

```text
Since both `FnMut` and `FnOnce` are supertraits of `Fn`, any
instance of `Fn` can be used as a parameter where a `FnMut` or
`FnOnce` is expected.
```

Direct corpus source for "an `<F: FnOnce(...)>` bound accepts any
closure; `<F: FnMut(...)>` accepts FnMut and Fn closures." Probe 4
empirically corroborates this — `pure_b` (Fn) passes to
`call_fnmut`, `pure_c` (Fn) passes to `call_fnonce`. Verified at
lines 33-35.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (FnMut supertrait of FnOnce, auto-impl rule)

The corpus file `output/docs/rust/std/ops/trait.FnMut.md` is the
trait page for `FnMut`. Verified by reading.

### Lines 7-18 (trait header showing supertrait and `&mut self` receiver)

```text
pub trait FnMut<Args>: FnOnce<Args>

where
    Args: Tuple,

{
    // Required method
    extern "rust-call" fn call_mut(
        &mut self,
        args: Args,
    ) -> Self::Output;
}
```

`: FnOnce<Args>` is the supertrait declaration — every `FnMut` is
also `FnOnce`. `call_mut(&mut self, ...)` confirms the
mutable-borrow receiver, which is the structural reason `mut f: F`
is required when the parameter is `FnMut`-bounded and the body
calls `f(x)`. Verified at lines 7-18.

### Line 27 (FnMut auto-implemented)

```text
`FnMut` is implemented automatically by closures which take mutable
references to captured variables, ...
```

Direct corpus source for "mutate a captured binding → `FnMut` (and
so also `FnOnce`)." Verified at line 27.

### Lines 33-35 (subtrait/supertrait practical consequence)

```text
Since `FnOnce` is a supertrait of `FnMut`, any instance of `FnMut`
can be used where a `FnOnce` is expected, and since `Fn` is a
subtrait of `FnMut`, any instance of `Fn` can be used where `FnMut`
is expected.
```

Direct corpus source for the lesson's "an `<F: FnMut(...)>` bound
accepts FnMut and Fn closures." Probe 4 corroborates: both `pure_b`
(Fn) and `tick_a` (FnMut) pass to `call_fnmut`. Verified at lines
33-35.

### Lines 65-66 (canonical FnMut example with `mut func`)

```text
fn do_twice<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}
```

The std-library FnMut page itself writes `mut func: F` for an
FnMut-bounded parameter. Direct corpus source for the lesson's
"`<F: FnMut(...)>` bound forces `mut f: F`" claim. Probe 3 is the
empirical witness; this corpus passage is the textual confirmation.
Lesson 147's evidence (Side-probe D) already documented this; today
re-uses the witness. Verified at lines 65-66.

## Source — `output/docs/rust/std/ops/trait.FnOnce.md` (the base trait, by-value receiver)

The corpus file `output/docs/rust/std/ops/trait.FnOnce.md` is the
trait page for `FnOnce`. Verified by reading.

### Lines 7-17 (trait header — by-value receiver)

```text
pub trait FnOnce<Args>

where
    Args: Tuple,

{
    type Output;

    // Required method
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

No supertrait — `FnOnce` is the base of the layered family. The
required method `call_once(self, args: Args)` takes `self` by value,
which consumes the closure on call. This is the structural reason
`FnOnce` closures can only be called once and the structural reason
the trait's name is "Once." Verified at lines 7-17.

### Line 28 (FnOnce auto-implemented)

```text
`FnOnce` is implemented automatically by closures that might
consume captured variables, ...
```

Direct corpus source for "move a captured value out → `FnOnce`
only." Probe 1's `consume` closure (`drop(s)` inside) is exactly
this case. Verified at line 28.

### Lines 24-27 (called once)

```text
Instances of `FnOnce` can be called, but might not be callable
multiple times. Because of this, if the only thing known about a
type is that it implements `FnOnce`, it can only be called once.
```

Direct corpus source for the once-only rule that Probe 6 witnesses
empirically with E0382. Verified at lines 24-27.

## Source — `output/docs/rust/book/ch13-01-closures.md` (auto-impl rule, three-line summary)

The corpus file `output/docs/rust/book/ch13-01-closures.md` covers
closures. Verified by reading.

### Lines 431-447 (three-line summary of the Fn-family auto-impl rule)

```text
The way a closure captures and handles values from the environment
affects which traits the closure implements, and traits are how
functions and structs can specify what kinds of closures they can
use. Closures will automatically implement one, two, or all three
of these `Fn` traits, in an additive fashion, depending on how the
closure's body handles the values:

- `FnOnce` applies to closures that can be called once. All closures
  implement at least this trait because all closures can be called.
  A closure that moves captured values out of its body will only
  implement `FnOnce` and none of the other `Fn` traits because it
  can only be called once.
- `FnMut` applies to closures that don't move captured values out
  of their body but might mutate the captured values. These
  closures can be called more than once.
- `Fn` applies to closures that don't move captured values out of
  their body and don't mutate captured values, as well as closures
  that capture nothing from their environment. These closures can
  be called more than once without mutating their environment, ...
```

This is the canonical Book treatment cited in the lesson body. The
phrase "additive fashion" is the Book's word for the supertrait
layering: a Fn closure also implements FnMut and FnOnce; a FnMut
closure also implements FnOnce. All three of today's probe closures
match exactly one of the three rules:

- `pure` ("captures nothing") → matches the third bullet → `Fn`
  (and so also `FnMut` and `FnOnce`).
- `tick` ("might mutate the captured values") → matches the second
  bullet → `FnMut` (and so also `FnOnce`).
- `consume` ("moves captured values out of its body") → matches
  the first bullet → `FnOnce` only.

Verified at lines 431-447.

### Lines 286-292 (the `move` keyword, light reference)

```text
### Capturing References or Moving Ownership

Closures can capture values from their environment in three ways,
which directly map to the three ways a function can take a
parameter: borrowing immutably, borrowing mutably, and taking
ownership. The closure will decide which of these to use based on
what the body of the function does with the captured values.
```

Direct corpus source for "the body's actions decide which traits
the closure implements." The `move` keyword is named in the lesson
body lightly because Probe 1's `consume` closure uses it; the full
treatment of `move` (lines 373-415 in the same chapter) is named-
deferred. Verified at line 286+.

## Source — error code documentation

### `output/docs/rust/error_codes/E0525.md` (closure trait mismatch)

```text
A closure was used but didn't implement the expected trait.
```

```rust
struct X;

fn foo<T>(_: T) {}
fn bar<T: Fn(u32)>(_: T) {}

fn main() {
    let x = X;
    let closure = |_| foo(x); // error: expected a closure that implements
                              //        the `Fn` trait, but this closure only
                              //        implements `FnOnce`
    bar(closure);
}
```

Direct corpus source for the inline label "expected a closure that
implements the `Fn` trait, but this closure only implements
`FnOnce`" that Probes 2 and 5 (in slightly different forms — Fn vs
FnMut, FnMut vs FnOnce) witness. The corpus example uses
`FnOnce`-only because the closure moves the non-`Copy` `X` out;
today's probes use the same shape with `String` instead of `X`.
Verified by reading.

### `output/docs/rust/error_codes/E0596.md` (cannot borrow as mutable)

```text
This error occurs because you tried to mutably borrow a non-mutable
variable.
```

The corpus example shows the `&mut` form (`let y = &mut x` on a
non-mutable `x`); Probe 3 witnesses the same code with a different
payload — calling `f(x)` on a non-mutable `f: F` parameter where
`F: FnMut(...)`. The same `help: consider making this binding
mutable` line and `+++` markers under the proposed `mut`
insertion appear in both. Verified by reading.

## Probe 1 — working probe (three closures, three call helpers)

Source: `observations/148-fn-fnmut-fnonce-distinction.rs`. Transcript: PROBE 1 block.

Output: three lines `Fn:     6` / `FnMut:  5` / `FnOnce: 0`,
compile-exit=0, run-exit=0. Five load-bearing facts:

- All three parenthesized bounds (`Fn(u32) -> u32`,
  `FnMut(u32) -> u32`, `FnOnce(u32) -> u32`) parse and compile
  with the same lesson-147 grammar. Lesson 148's claim "the
  parenthesized form covers all three Fn-family traits" is
  empirically witnessed.
- The body of `call_fnmut` is `f(x)`. With `mut f: F` it compiles;
  Probe 3 below witnesses what happens without `mut`. The lesson
  body's claim "FnMut bounds force `mut f: F`" is empirically
  witnessed.
- Each closure passes to its expected call helper: `pure` to
  `call_fn`, `tick` to `call_fnmut`, `consume` to `call_fnonce`.
- The `tick` closure is *capturing-and-mutating* — it modifies
  `counter` from the enclosing scope via `counter += n`. The output
  `5` is `counter += 5` from initial `0`, then returned via the
  body's last expression `counter`. The lesson body's claim "body
  mutates captures → `FnMut`" is witnessed by the closure being
  accepted at `call_fnmut`'s bound (Probe 4 corroborates that it is
  *not* accepted at `call_fn`'s bound).
- The `consume` closure consumes its captured `s` via `drop(s)`.
  The output `0` is the closure's return value `0_u32`. The lesson
  body's claim "body moves captures out → `FnOnce` only" is
  witnessed by the closure being accepted at `call_fnonce`'s bound
  (Probe 5 corroborates that it is *not* accepted at `call_fnmut`'s
  bound).

## Probe 2 — pass FnMut closure to Fn-bounded function (E0525)

Source: `wrong_trait.rs`. Transcript: PROBE 2 block.

Modification from Probe 1: the function carrying the bound is
`call_fn` (Fn-bounded), and the closure passed in is `tick` (the
mutating-capture closure that implements only `FnMut + FnOnce`).
Output:

```text
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
  --> wrong_trait.rs:10:16
   |
10 |     let tick = |n: u32| { counter += n; counter };
   |                ^^^^^^^^   ------- closure is `FnMut` because it mutates the variable `counter` here
   |                |
   |                this closure implements `FnMut`, not `Fn`
11 |     println!("{}", call_fn(tick, 5));
   |                    ------- ---- the requirement to implement `Fn` derives from here
   |                    |
   |                    required by a bound introduced by this call
   |
note: required by a bound in `call_fn`
  --> wrong_trait.rs:4:15
   |
 4 | fn call_fn<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
   |               ^^^^^^^^^^^^^^ required by this bound in `call_fn`
```

Compile exit 1. Five grounded facts:

- Error code `E0525`. Inline label "expected a closure that
  implements the `Fn` trait, but this closure only implements
  `FnMut`" — rustc spells out both expected and found traits.
- Caret on the closure literal `|n: u32| { counter += n; counter }`
  with an annotated subspan on `counter` (the `++++++++` underline)
  labelled "closure is `FnMut` because it mutates the variable
  `counter` here." This is the empirical witness that rustc tracks
  *which captured variable* forced the closure into the more
  restrictive trait, and surfaces it in the diagnostic.
- A second annotated span on the call line marks the call as the
  source of the `Fn` requirement.
- The `note: required by a bound in \`call_fn\`` block carries its
  own `-->` line at column 15 of line 4, and the caret falls on
  the entire `Fn(u32) -> u32` bound segment of the function
  declaration.
- The diagnostic shape is the lesson 003 four-part map unchanged
  (headline + `-->` + source excerpt + `note:`).

## Probe 3 — drop `mut` from `mut f: F` (E0596)

Source: `no_mut.rs`. Transcript: PROBE 3 block.

Modification from Probe 1: the function `call_fnmut`'s parameter is
`f: F` (no `mut`). The closure literal and call site are unchanged.
Output:

```text
error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
 --> no_mut.rs:6:5
  |
6 |     f(x)
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
5 | fn call_fnmut<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32 {
  |                                     +++
```

Compile exit 1. Four grounded facts:

- Error code `E0596` — same code lesson 131 first witnessed for
  `&mut self` on `next()`; today's payload is at function-parameter
  position.
- The body's `f(x)` call is rejected because it requires a mutable
  borrow of `f`. This is the empirical witness for the structural
  claim: `FnMut::call_mut(&mut self, ...)` requires `f` to be a
  mutable binding.
- The `help:` block proposes the exact fix: insert `mut ` before
  the parameter name `f`. The `+++` markers under the proposed
  insertion are rustc's standard insertion-fix shape, identical to
  lesson 006's `let mut x` insertion in the E0384 contrast.
- The `mut` insertion sits *between* `(` and `f`, not before
  `F: FnMut(...)` — confirming this is the parameter-binding
  mutability mechanic (lesson 006 / 020 composition), not a
  type-level mechanic.

## Probe 4 — supertrait empirical (Fn → all three; FnMut → FnMut+FnOnce)

Source: `supertrait.rs`. Transcript: PROBE 4 block.

This is the supertrait probe. Three Fn closures (`pure_a`, `pure_b`,
`pure_c`) pass to `call_fn`, `call_fnmut`, `call_fnonce`
respectively. Two FnMut closures (`tick_a`, `tick_b`) pass to
`call_fnmut` and `call_fnonce`. (The probe needs three separate Fn
closures because each closure type is unique by construction;
re-binding a single closure across three calls would not fail, but
the probe's clarity is improved by separate names.)

Output: five lines, all values printed correctly. Compile exit 0,
run exit 0. Three load-bearing facts:

- A `Fn` closure passes to a `FnMut` bound (subtrait → supertrait).
  This is the corpus claim "any instance of `Fn` can be used as a
  parameter where a `FnMut` is expected" (`Fn.md:33-35`) empirically
  witnessed.
- A `Fn` closure passes to a `FnOnce` bound. Same corpus claim with
  `FnOnce` substituted; same lines verbatim.
- A `FnMut` closure passes to a `FnOnce` bound. This is the corpus
  claim from `FnMut.md:33-35` ("any instance of `FnMut` can be used
  where a `FnOnce` is expected") empirically witnessed.

The structural facts asymmetric: a `FnMut`-only closure does *not*
pass to a `Fn` bound (Probe 2). A `FnOnce`-only closure does *not*
pass to a `FnMut` bound (Probe 5). The supertrait relationship
flows in one direction only.

## Probe 5 — pass FnOnce-only closure to FnMut bound (E0525, complementary)

Source: `once_to_mut.rs`. Transcript: PROBE 5 block.

Modification from Probe 4: the FnMut-bound function is the same
`call_fnmut`; the closure is the FnOnce-only `consume` (with `move`
plus `drop(s)`). Output:

```text
error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
  --> once_to_mut.rs:11:19
   |
11 |     let consume = move |_: u32| { drop(s); 0_u32 };
   |                   ^^^^^^^^^^^^^        - closure is `FnOnce` because it moves the variable `s` out of its environment
```

Compile exit 1. Same E-code as Probe 2 with payload "expected
`FnMut`, found `FnOnce`" instead of "expected `Fn`, found `FnMut`."
The annotated subspan labels `s` as the variable moved out, which
empirically witnesses the corpus claim "a closure that moves
captured values out of its body will only implement `FnOnce`"
(Book `ch13-01-closures.md:438-440`).

This is the second half of the supertrait-asymmetry witness: just as
Probe 2 shows FnMut-only does not satisfy Fn, this shows
FnOnce-only does not satisfy FnMut. Together with Probe 4 (the
positive case), the supertrait relationship is fully grounded.

## Probe 6 — call FnOnce closure twice (E0382, structural witness)

Source: `once_twice.rs`. Transcript: PROBE 6 block.

Modification: the FnOnce-bound function `call_fnonce` calls `f(x)`
twice in its body (`f(x); f(x)`). Output:

```text
error[E0382]: use of moved value: `f`
 --> once_twice.rs:7:5
  |
5 | fn call_fnonce<F: FnOnce(u32) -> u32>(f: F, x: u32) -> u32 {
  |                                       - move occurs because `f` has type `F`, which does not implement the `Copy` trait
6 |     f(x);
  |     ---- `f` moved due to this call
7 |     f(x)
  |     ^ value used here after move
  |
note: `FnOnce` closures can only be called once
 --> once_twice.rs:5:19
```

Compile exit 1. Three load-bearing facts:

- Error code `E0382`, "use of moved value: `f`." Same code lesson
  133 first witnessed for `count(self)` consuming the iterator. The
  parameter `f: F` with `F: FnOnce(...)` is consumed on the first
  call.
- The `note:` block carries the verbatim phrase "`FnOnce` closures
  can only be called once" — this is rustc's own naming of the
  corpus rule (Book line 437). Direct empirical witness for the
  lesson body's "FnOnce ... is why 'once'."
- The annotated span "move occurs because `f` has type `F`, which
  does not implement the `Copy` trait" connects to lesson 133's
  consume-by-value mechanic. The bound `F: FnOnce(...)` does *not*
  imply `F: Copy`; the call_once method takes `self` by value, so
  the parameter is consumed on the call.

This probe is the structural witness for the `self`-by-value
receiver shape on `FnOnce::call_once`. The lesson body cites this
in the supertrait section without needing to commit additional
text to the diagnostic.

## Probe 7 — side-probe: closure body forces FnOnce without `move` keyword

Source: `no_move.rs`. Transcript: PROBE 7 block.

Modification from Probe 5: drop the `move` keyword from `consume`.
The closure literal becomes `let consume = |_: u32| { drop(s); 0_u32 };`.
Output: same E0525 with the same payload — closure implements
`FnOnce` only.

Load-bearing fact: the `move` keyword is *not* what makes the
closure FnOnce. The body's `drop(s)` is what does. The closure
captures `s` by move automatically (because `drop(s)` consumes it).
This corroborates the Book's line 290-292 verbatim: "The closure
will decide which of these to use based on what the body of the
function does with the captured values." — and rules out the
audience misconception that `move` is the FnOnce-trigger.

The lesson body uses `move` lightly in Probe 1 because it makes the
intent explicit, but this side-probe documents that the keyword is
not strictly required for that case. The full `move`-keyword
mechanic is named-deferred to its own move (Book line 286+).

## Probe 8 — side-probe (Vec-pushing closure is FnMut)

Source: `check_yourself.rs`. Transcript: PROBE 8 block.

A side-probe corroborating that the E0525 mechanism is type-agnostic
across captured types. A closure that captures `let mut v: Vec<u32>`
and calls `v.push(n)` mutates the captured `v`, so it implements
`FnMut + FnOnce` but not `Fn`. Passing it to `call_fn` (Fn-bounded)
fires E0525:

```text
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
  --> check_yourself.rs:11:18
   |
11 |     let push_n = |n: u32| { v.push(n); v.len() as u32 };
   |                  ^^^^^^^^   - closure is `FnMut` because it mutates the variable `v` here
```

Witnesses that the same E0525 fires when the captured-and-mutated
variable is a `Vec<u32>` rather than a `u32` (Probe 2). The
captured-variable name in the diagnostic changes from `counter` to
`v`; the inline label structure is identical. The lesson body's
Check Yourself uses a simpler `u32`-counter probe (Probes 9/10
below) to avoid relying on uninstalled `Vec::new()` and `.push`
mechanics; this side-probe documents that the lesson generalizes.

## Probe 9 — Check Yourself part (a)

Source: `q_a.rs`. Transcript: PROBE 9 block.

Closure captures `let mut total: u32 = 100;` and mutates it via
`total += n; total`. Pass to `call_fn` (Fn-bounded). Output:

```text
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
 --> q_a.rs:5:16
  |
5 |     let bump = |n: u32| { total += n; total };
  |                ^^^^^^^^   ----- closure is `FnMut` because it mutates the variable `total` here
```

Compile exit 1. Verifies the Check Yourself part (a) answer
empirically. rustc names `total` as the captured variable; same
E0525 mechanism as Probe 2 (which had `counter` instead of `total`);
proves the Check Yourself's lesson body claim.

## Probe 10 — Check Yourself part (b)

Source: `q_b.rs`. Transcript: PROBE 10 block.

Same closure as Probe 9. Function bound changed from `Fn(u32) -> u32`
to `FnMut(u32) -> u32` and parameter changed from `f: F` to
`mut f: F`. Output: `107`, compile exit 0, run exit 0.

Verifies the Check Yourself part (b) answer empirically: changing
the bound to FnMut and the parameter to `mut f: F` accepts the
mutating closure. The output `107` is `total = 100 + 7 = 107` —
the closure's body adds the parameter `n=7` to the captured `total`
and returns the updated value via the body's last expression.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| The Fn-family has three traits | `Fn.md:7`, `FnMut.md:7`, `FnOnce.md:7` (three trait pages) |
| `Fn: FnMut` (Fn is a subtrait of FnMut) | `Fn.md:7` verbatim `pub trait Fn<Args>: FnMut<Args>` |
| `FnMut: FnOnce` | `FnMut.md:7` verbatim `pub trait FnMut<Args>: FnOnce<Args>` |
| `Fn`'s required method is `call(&self, args)` | `Fn.md:14` verbatim |
| `FnMut`'s required method is `call_mut(&mut self, args)` | `FnMut.md:14-17` verbatim |
| `FnOnce`'s required method is `call_once(self, args)` | `FnOnce.md:16` verbatim |
| Auto-impl rule: no capture or read-only → Fn | `Fn.md:27` verbatim "implemented automatically by closures which only take immutable references to captured variables or don't capture anything at all"; Book `ch13-01-closures.md:444-447` |
| Auto-impl rule: mutate → FnMut | `FnMut.md:27` verbatim "take mutable references to captured variables"; Book `ch13-01-closures.md:441-443` |
| Auto-impl rule: move out → FnOnce | `FnOnce.md:28` verbatim "might consume captured variables"; Book `ch13-01-closures.md:437-440` |
| `<F: FnOnce(...)>` accepts any closure; `<F: FnMut(...)>` accepts FnMut + Fn; `<F: Fn(...)>` accepts only Fn | `Fn.md:33-35` and `FnMut.md:33-35` verbatim subtrait-supertrait usage rules; Probe 4 empirical witness |
| `<F: FnMut(...)>` bound forces `mut f: F` | Probe 3 empirical (E0596 with `+++` markers); `FnMut.md:65-66` corpus precedent `fn do_twice<F>(mut func: F) where F: FnMut()` |
| Trait mismatch fires E0525 | Probes 2, 5, 8 transcripts; `error_codes/E0525.md` corpus |
| FnOnce can only be called once | `FnOnce.md:24-27` verbatim "if the only thing known about a type is that it implements `FnOnce`, it can only be called once"; Probe 6 empirical (E0382 with `note:` "FnOnce closures can only be called once") |
| The body's actions decide which traits the closure implements | Book `ch13-01-closures.md:290-292` verbatim "The closure will decide which of these to use based on what the body of the function does with the captured values"; Probe 7 side-probe (the `move` keyword is not the FnOnce-trigger) |
| `move` keyword forces ownership transfer at capture time | Book `ch13-01-closures.md:373-375` verbatim "If you want to force the closure to take ownership of the values it uses in the environment ... you can use the `move` keyword before the parameter list"; Probe 1 (`move` used) and Probe 7 (no `move`, same trait) |
| Output `Fn: 6` / `FnMut: 5` / `FnOnce: 0` | Probe 1 transcript |

## Older supporting lessons (named only)

- 147-fn-trait-parenthesized-bound — `<F: Fn(T) -> R>` shape.
- 146-trait-bound-on-type-parameter — `<T: TRAIT>` inline form.
- 145-generic-function-type-parameter — `fn name<T>(t: T)`.
- 144-closure-captures-outer-let — closure capture mechanic.
- 142-closure-literal-bound-and-called — `let n = |p: T| body;`.
- 042-string-new — `String::from(...)` for non-Copy values.
- 020-function-with-parameter — `f: F` parameter slot.
- 008-define-and-call-function — `f(x)` call shape.
- 006-mut-binding — `let mut x = ...; x = ...;`.
- 003-read-rustc-diagnostic — four-part diagnostic map.
- 080-integer-type-family, 011-println-positional-args,
  005-let-binding, 002-fn-main-entry-point, 001-rustc-compile-and-run.

## Deliberate scope discipline

The orchestrator's prompt named scope items to NOT install. The
lesson body's *What To Ignore For Now* section names each:

1. The desugaring `Fn(T) -> R` ≡ `Fn<(T,), Output = R>`
   (implementor-side; FnOnce.md:13 declares `type Output;`).
2. `impl Fn(...)` parameter or return position (separate sugar).
3. `Box<dyn Fn(...)>` / `&dyn Fn(...)` (dynamic dispatch).
4. Function pointers `fn(u32) -> u32` (separate type).
5. Higher-ranked trait bounds (deferred wholesale).
6. AsyncFn / AsyncFnMut / AsyncFnOnce (deferred wholesale).
7. Multiple parameters `Fn(T, U) -> R`, no-return form `Fn(T)`
   (same mechanic extended).
8. The `move` keyword as its own mechanic (used lightly today;
   Book line 286+ covers it).

The `move` keyword is named in the lesson body and used in Probe 1
because it makes the FnOnce-only intent explicit. Probe 7 documents
that `move` is not strictly required to produce a FnOnce-only
closure (the body's `drop(s)` is what forces it). The full mechanic
of `move` (when needed, thread/closure ownership patterns) is
deferred per the prompt's scope discipline.

## Run-context handoff

After this lesson lands, the orchestrator should:

1. Update `iterator-api-coverage.md` to v3, marking the closure
   sub-arc complete.
2. Recompute the next-arc plan. Likely first closure-driven
   Iterator method: `for_each` (consumer, `FnMut(Self::Item)`, no
   return) or `map` (lazy adapter, `FnMut(Self::Item) -> B`,
   rewrites yielded element type).

The 27 closure-driven Iterator methods (audit §4.4.1) become
teachable as of this lesson's acceptance.
