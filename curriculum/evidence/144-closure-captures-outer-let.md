# Evidence — Lesson 144: a closure body can reference an outer `let`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/144-closure-captures-outer-let.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/144-closure-captures-outer-let.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/144-closure-captures-outer-let.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet144/` and compiled with
`rustc <file>`; resulting executables were run from the same directory.
Same host and toolchain as accepted lessons through 143.

## Run context — third move of the closure sub-arc

Per `iterator-api-coverage.md` §6 (committed `cb9945066`, v2 of the
audit), the closure sub-arc is the next major arc after lesson 141
closed the closure-free Iterator surface (audit §5). The five-step
plan:

1. Closure literal bound and called (lesson 142, accepted, commit `915347f66`).
2. Unannotated closure first-call-fixes-the-type (lesson 143, accepted, commit `50049ef9b`).
3. Closure capturing an outer binding (today).
4. `FnMut`-bound parameter on a function.
5. `Fn` / `FnMut` / `FnOnce` distinction.

Lessons 142 and 143 named today's centered fact in their `What To
Ignore For Now` lists and in their graph `unlocks` fields. Today's
`depends_on` therefore names 142 load-bearingly. Lesson 143 is not
load-bearing for today's centered fact: 143 covered the unannotated
parameter form (`|x| x`); today writes the parameter as fully
annotated (`|x: u32|` per Book v2/v4 hybrid, lesson 142 form). The
asymmetry today centers on body content (mention of an outer name)
rather than parameter inference.

## Direct prerequisite — lesson 142 (closure literal bound and called)

Lesson 142 installed:

- The closure literal `|param: T| body` syntax with a fully annotated
  parameter slot and a single-expression body (Book v2 parameter, v4
  body).
- The closure as a value bound with `let` and called with parens
  (`add_one(5)`), reusing lesson 008's call-with-parens shape.

Today reuses every slot from 142 unchanged. The closure literal in
the working probe is `|x: u32| x + n` — same parameter slot
`name: TYPE` inside `|...|` brackets, same brace-free body form. The
new fact relative to 142 is what the body is allowed to *mention*:
142's bodies referred only to the closure's own parameter (`x + 1`,
`x`); today's body mentions an outer-scope name (`n`).

## Direct prerequisite — lesson 005 (let binding)

Lesson 005 installed `let name = value;` and the property "later
statements in the same body can use the bound name." Today rests on
two distinct `let` statements in `main`:

- The outer `let n: u32 = 10;` (lesson 005 + lesson 062's `u32` type
  annotation surface, restated through lesson 080) — this is the
  binding the closure captures.
- The closure-binding `let add_n = |x: u32| x + n;` (lesson 005 +
  lesson 142's closure-on-the-right form) — the `let`-bound closure
  value, callable with parens.

The lesson-005 property "a later statement in the same body can refer
to the bound name as that value" is what allows the closure literal
(written *after* `let n: u32 = 10;`) to reach the binding `n` from
the closure body. The new fact today is that this reach extends
*inside* the closure literal's body, not just to top-level statements
that mention `n`.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic shape: headline, `-->`
location, source excerpt with caret, and optional `help:` / `note:`
trailers. E0434 is a new error code today; the diagnostic *shape* is
unchanged. The new feature today is structurally minor: the headline
is a one-line description ("can't capture dynamic environment in a fn
item"), the caret lands on a single token (the captured name), and
the `help:` line names a fix in source-code shorthand (`|| { ... }`).
No multi-location feature is required (unlike lesson 143's
cross-reference `note:` blocks).

## Direct prerequisite — lesson 008 (define and call function)

Lesson 008 installed `fn name() { ... }` defined and `name();` called.
Probe 2's contrast `fn add_n(x: u32) -> u32 { x + n }` reuses that
shape with one parameter (lesson 020) and a return type (lesson 021).
The novel structural fact relative to 008 is *placement*: the `fn` is
declared *inside* `main`, not at file top level. This nesting is what
makes the test fair — the captured name `n` is clearly in
`main`-scope, so the question becomes solely whether the inner `fn`
can reach it. Lesson 008 did not assert (and does not deny) that
nested `fn` items are legal; today's probe confirms they parse but
fire E0434 if the body references an outer local.

## Direct prerequisite — lesson 020 (function with parameter)

Lesson 020 installed `fn name(p: TYPE) { ... }` — one typed parameter
inside the function-parameter list. Probe 2's `fn add_n(x: u32) -> u32 { x + n }`
reuses that exact shape. Lesson 020's hard rule "in function
signatures parameter types are mandatory" is reproduced verbatim; the
contrast in lesson 142 was that closure parameter types are
*optional*. Today's contrast is different again: the `fn` form's
*body* is rejected (E0434), not its signature (which parses fine).

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 080**: `u32` is one of the integer types. Used in the
  outer binding `let n: u32 = 10;` and the closure parameter `|x: u32|`.
- **Lesson 009**: `+` on integers produces a new integer. Today's body
  `x + n` is one such expression.
- **Lesson 011**: `println!("{}", name)` formats with positional args.
  Used in both probes.
- **Lesson 002**: `fn main` is the entry point.
- **Lesson 001**: `rustc file.rs`, `./name`; rustc silent on success.

## Source — Book ch13-01-closures.md (the closure/function asymmetry)

The lesson's primary corpus source is
`output/docs/rust/book/ch13-01-closures.md`. Three load-bearing
passages, line ranges verified by reading the file at those lines:

### Lines 6-9 (the framing one-liner the lesson body quotes):

```text
Rust's closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure elsewhere to evaluate it in a different context. Unlike
functions, closures can capture values from the scope in which they're defined.
```

This is the source for the lesson body's framing quote — "Unlike
functions, closures can capture values from the scope in which
they're defined." Verified at lines 6-9 of the corpus file.

### Lines 134-135 (the asymmetry restated alongside Listing 13-1):

```text
Functions, on the other hand, are not able to capture their environment
in this way.
```

This is the second corpus restatement of the "closures can capture,
functions cannot" asymmetry, sitting at the bottom of the
"Capturing the Environment" subsection. The lesson body does not
quote this verbatim — line 134-135 reinforces the line 6-9 framing
with a more direct one-liner.

### Lines 286-296 (the "Capturing References or Moving Ownership" intro):

```text
### [Capturing References or Moving Ownership](#capturing-references-or-moving-ownership)

Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: borrowing
immutably, borrowing mutably, and taking ownership. The closure will decide
which of these to use based on what the body of the function does with the
captured values.

In Listing 13-4, we define a closure that captures an immutable reference to
the vector named `list` because it only needs an immutable reference to print
the value.
```

The Book heading "Capturing References or Moving Ownership" begins
at line 286, with the section's first sentence at line 288. Lines
286-296 are the introductory paragraph that names the three capture
modes (immutable borrow, mutable borrow, move). Today's lesson does
*not* center this — the three-mode classification is named-deferred
to a later closure sub-arc move per the lesson's `What To Ignore For
Now` section. Today's lesson cites this passage *only* as a forward
pointer (the `What To Ignore For Now` first bullet links to it via
`ch13-01-closures.md:286+`).

The orchestrator prompt named `:286+` as the location of the
"Capturing References or Moving Ownership" section. Verified: line
286 is exactly the heading, and the section runs through line 415
(closing before "Moving Captured Values Out of Closures" at line
419). The lesson's reference `ch13-01-closures.md:286+` is correct.

### Listing 13-4 (lines 300-311) — the canonical immutable-capture example

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

The Book's canonical example for immutable capture. Today's lesson
does *not* center this listing because it relies on (a) the macro
`vec!` (Vec literals — installed at lesson 107 but not on the
critical path here) and (b) the `{list:?}` Debug-formatter form,
both extending the surface beyond the centered fact. The probe
substitutes `let n: u32 = 10;` (lesson 005 + 062/080) and a
single-expression body `x + n` so the prerequisites stay tight.

The lesson 142 evidence appendix already noted that 142 substituted
similarly to keep prereqs tight; today follows the same convention.
The structural shape — closure body references an outer `let` —
matches Listing 13-4's `only_borrows = || println!("...{list:?}");`
referencing `let list = vec![1, 2, 3]`. Today's probe reduces this
to the smallest demonstrable form.

## Source — error_codes/E0434.md (the canonical fn-item rejection)

The corpus file `output/docs/rust/error_codes/E0434.md` documents the
exact error code Probe 2 fires. Two load-bearing chunks:

### Lines 4-19 (the description and the erroneous example):

```text
A variable used inside an inner function comes from a dynamic environment.

Erroneous code example:

```
#![allow(unused)]
fn main() {
fn foo() {
    let y = 5;
    fn bar() -> u32 {
        y // error: can't capture dynamic environment in a fn item; use the
          //        || { ... } closure form instead.
    }
}
}
```
```

The structural shape matches Probe 2 exactly modulo names and types:
outer scope has a `let`-bound integer; an inner `fn` body mentions
that name; rustc rejects with E0434. Today's probe substitutes
`y = 5` (i32 default) with `n: u32 = 10` (lesson 062/080 explicit
annotation) and adds a parameter `x: u32` plus the binary `x + n`
body — keeping every prereq within the run's installed material.

### Lines 21-23 (the canonical fix the lesson body cites):

```text
Inner functions do not have access to their containing environment. To fix this
error, you can replace the function with a closure:
```

This is the source for the lesson body's quoted line "Inner functions
do not have access to their containing environment. To fix this
error, you can replace the function with a closure." Verbatim. The
lesson's `What To Ignore For Now` `static`/`const` bullet refers
forward to lines 36-52 of this same file (the `static mut X: u32 = 4;
const Y: u32 = 5;` work-around shown there) but does not centre that
escape hatch.

## Probe 1 — working probe (closure body captures outer `let`)

Source: `observations/144-closure-captures-outer-let.rs`.
Transcript: `observations/144-closure-captures-outer-let.transcript.txt` PROBE 1 block.

```rust
fn main() {
    let n: u32 = 10;
    let add_n = |x: u32| x + n;
    let a = add_n(5);
    let b = add_n(7);
    println!("{}", a);
    println!("{}", b);
}
```

Output:

```text
15
17
```

Compile exit 0, run exit 0. Three load-bearing structural facts witnessed:

- The closure literal `|x: u32| x + n` parses cleanly. No "name not
  found" error fires for `n` even though `n` is *not* declared inside
  the closure literal — rustc reaches into the enclosing scope.
- The first call `add_n(5)` substitutes `5` for `x` and reads `n` from
  the captured outer scope, producing `5 + 10 = 15`. The second call
  `add_n(7)` reuses the same `n`, producing `7 + 10 = 17`. Both
  output values match arithmetic.
- The closure can be called more than once with different arguments;
  the captured `n` is unchanged between calls. (Today's lesson does
  not centre repeated-call semantics — that was lesson 142 — but
  Probe 1 confirms the property carries.)

## Probe 2 — negative contrast (nested `fn` cannot capture)

Source `fnitem.rs` (in transcript). Output:

```text
error[E0434]: can't capture dynamic environment in a fn item
 --> fnitem.rs:3:35
  |
3 |     fn add_n(x: u32) -> u32 { x + n }
  |                                   ^
  |
  = help: use the `|| { ... }` closure form instead

warning: unused variable: `n`
 --> fnitem.rs:2:9
  |
2 |     let n: u32 = 10;
  |         ^ help: if this is intentional, prefix it with an underscore: `_n`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0434`.
```

Compile exit 1. Three grounded facts from rustc's mouth:

- The error code is `E0434` with the exact one-line description
  "can't capture dynamic environment in a fn item." This matches the
  prompt's prediction.
- The caret lands on the `n` token inside the `fn` body (column 35 of
  line 3, which is the `n` in `x + n`). The `n` is what rustc refuses
  to resolve. The signature `fn add_n(x: u32) -> u32 { ... }` parses
  and is not what rustc objects to.
- The `help:` line says "use the `|| { ... }` closure form instead".
  This is rustc explicitly naming the closure form as the fix — the
  asymmetry the lesson centers.

The accompanying `warning: unused variable: n` is *because* the `fn`
body cannot capture `n` — rustc treats `n` as having no use, even
though the source code mentions the name. The warning is incidental
and not load-bearing for today's centered fact; it is documented
here only because it appears in the transcript and a careful reader
will see it.

This is the contrastive witness for the lesson's claim "a `fn` item
cannot do this." Probe 1 + Probe 2 differ in *one line* —
`let add_n = |x: u32| x + n;` versus
`fn add_n(x: u32) -> u32 { x + n }` — same body `x + n`, same outer
binding `let n: u32 = 10;`. The closure form compiles; the `fn` form
fires E0434. The asymmetry is the centered fact.

The verbatim `rustc --explain E0434` text (transcript appendix) is
the source for the lesson body's quoted line "Inner functions do not
have access to their containing environment. To fix this error, you
can replace the function with a closure."

## Side-probe A — captured `n` still readable at outer scope

Source `coexist.rs` (in transcript). Output:

```text
15
10
run-exit=0
```

After the closure is defined and called, the outer `n` is still
usable — `println!("{}", n);` after the call prints `10`, the
original value. This grounds the implicit assumption that immutable
capture does *not* consume the captured binding.

The Book's Listing 13-4 (corpus lines 300-311 + commentary at
319-322) makes the same observation:

```text
Because we can have multiple immutable references to `list` at the same time,
`list` is still accessible from the code before the closure definition, after
the closure definition but before the closure is called, and after the closure
is called.
```

(Lines 319-322.)

The lesson body does not centre this co-existence claim — it would
preempt the deferred capture-mode classification (audit §6 step 3
deferred subtopic, future closure sub-arc move). Side-probe A
documents the witness here so the lesson's "immutable capture only"
scope is grounded.

## Side-probe B — closure that does not reference `n` compiles silently

Source `nobody.rs` (in transcript). Output: `compile-exit=0`, run
prints `6` then `10`.

If the closure body does *not* mention `n`, no capture happens —
`let plain = |x: u32| x + 1;` compiles silently regardless of whether
an outer `n` exists. Side-probe B rules out an over-broad reading of
"closures capture": only closures whose body references an outer
name capture that name. The mechanism is opt-in by mention.

This grounds the lesson body's careful phrasing: "A closure body
*can* refer to a name from the enclosing scope" (not "always does").

## Side-probe — Check Yourself answers

Sources `q.rs` (closure form) and `q-fn.rs` (nested-fn form). Output:

- `q.rs`: compile exit 0, run prints `36`. Confirms Check Yourself
  (a)'s answer (4 * 9 = 36).
- `q-fn.rs`: compile exit 1, fires E0434 with caret at line 3 column
  37 underlining the `g` in `x * g`, plus the help block "use the
  `|| { ... }` closure form instead". Confirms Check Yourself (b)'s
  answer (E0434 on `g`, help-line proposes the closure form).

These probes corroborate that the centered claim — "closures
capture, `fn` items do not" — generalizes beyond the specific names
(`n` vs `g`) and operators (`+` vs `*`) used in the working probes.

## Probe-not-needed — body literal seeding inference

Lesson 143's evidence appendix recorded that the body shape `|x| x + 1`
with no parameter annotation makes the `1` literal seed `x: i32`
(default integer type) at the *body* rather than at a call site.
Today's lesson keeps the parameter annotated (`|x: u32|`) for
consistency with lesson 142 and to remove inference as a moving
part. The `+ n` in today's body has no comparable seeding ambiguity
because `n: u32` is explicitly annotated; the body's type derives
unambiguously. No additional probe needed to rule this out.

## Probe-not-needed — top-level fn-item case

The orchestrator prompt's negative-contrast probe nests the `fn` item
inside `main`. An alternative would be a top-level `fn` item alongside
`main`:

```rust
fn add_n(x: u32) -> u32 { x + n } // ERROR: cannot find value `n` in this scope
fn main() { let n: u32 = 10; ... }
```

This would fire `error[E0425]: cannot find value 'n' in this scope` —
a *different* error code (the one lesson 005 captures), because at
top level `n` simply doesn't exist anywhere yet. The prompt's nested
form is preferred precisely because it isolates the capture question:
`n` is unambiguously in `main`-scope when the `fn` is reached, so
E0434 ("can't capture") is the correct rejection, not E0425
("not found"). The nested form makes "the inner `fn` *can see* but
*cannot use* the outer name" rustc-visible.

## Probe-not-needed — the `static`/`const` work-around

The E0434 explainer at corpus lines 36-52 shows two escape hatches:
declaring `static mut X: u32 = 4;` or `const Y: u32 = 5;` instead of
`let`-bound values; the `fn` body can then reach those. The lesson's
`What To Ignore For Now` names this work-around but defers it. No
probe today: the centered fact is "closures capture local `let`
bindings, `fn` items do not" — adding `static`/`const` would
distract.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Lessons 142/143 wrote closures whose bodies only used the parameter | Lessons 142/143 (accepted) |
| `let add_n = \|x: u32\| x + n;` parses and runs | Probe 1 transcript: compile-exit=0, run-exit=0 |
| `add_n(5)` produces `15`; `add_n(7)` produces `17` | Probe 1 output |
| `n` is in `main`-scope, declared by `let n: u32 = 10;` | Lesson 005 (let-binding scope) + Probe 1 source |
| Rust's word for this is *capture* | Book ch13-01-closures.md:6-9 ("closures can capture values from the scope in which they're defined") |
| Closures capture, functions don't | Book ch13-01-closures.md:6-9 + :134-135 |
| Probe 2 (nested fn) fires `error[E0434]: can't capture dynamic environment in a fn item` | Probe 2 transcript: rustc emits exact code + headline |
| Caret at line 3 column 35 on the `n` token in the body | Probe 2 transcript: `--> fnitem.rs:3:35` and caret position |
| `help: use the \|\| { ... } closure form instead` | Probe 2 transcript: verbatim |
| "Inner functions do not have access to their containing environment. To fix this error, you can replace the function with a closure." | E0434.md lines 21-23 (verbatim) |
| Probe 1 vs Probe 2 differ in one line, same body `x + n`, opposite outcomes | Probe 1/Probe 2 sources side by side |
| Captured `n` is still usable outside the closure | Side-probe A transcript |
| Closures whose body does not mention `n` do not capture it | Side-probe B transcript |
| Check Yourself (a) prints `36` | Side-probe `q.rs` transcript |
| Check Yourself (b) fires E0434, caret on `g` | Side-probe `q-fn.rs` transcript |

## Older supporting lessons (named only)

The following accepted lessons are cited in the lesson body but their
exact prereq claims are restated in the direct-prereq sections above
or in the lesson's own Prerequisites bullets:

- 080-integer-type-family — `u32` row of the integer family.
- 009-arithmetic-on-integers — `+` on integers.
- 011-println-positional-args — `println!("{}", name)`.
- 002-fn-main-entry-point — `fn main` is the entry point.
- 001-rustc-compile-and-run — `rustc file.rs`, `./name`.

Lesson 062 (`u32` as a specific row) and lesson 020 (typed function
parameter) are restated above but worth noting separately: lesson
062's `u32` annotation is what makes the outer `let n: u32 = 10;`
typed; lesson 020's `name: TYPE` parameter slot is what the closure
literal `|x: u32|` and the `fn` item `(x: u32)` both reuse.

## Deliberate scope discipline (per audit §6 step 3)

The orchestrator prompt names six things to NOT touch:

1. Mutable capture (`let mut x = ...; let f = || x += 1;`) — deferred.
2. The `move` keyword and ownership transfer — deferred.
3. The `Fn` / `FnMut` / `FnOnce` traits — deferred to steps 4-5.
4. Capture mode classification (by-shared-ref / by-mut-ref /
   by-move) — deferred to step 5.
5. Borrow-checker interactions — deferred.
6. Generic functions over closures or passing closures to functions —
   deferred to step 4.

The lesson body's `What To Ignore For Now` section names all six
explicitly. Probe 1's body `x + n` is a *read* (binary `+` consumes
neither operand by lesson 009; lesson 045's shared-reference rules
on integers are not invoked because integers are `Copy`, but the
lesson does not lean on that — instead it leans on the empirical
witness that `n` is still printable after the closure call,
side-probe A). The Book's three-mode taxonomy at corpus lines
286-292 is named in the deferred bullet, not unpacked.

The lesson body uses the word "capture" but does not classify its
mode. Rust silently picks "by shared reference" here because the body
only reads. The full classification — and the rule that "the closure
will decide which of these to use based on what the body of the
function does with the captured values" (corpus lines 290-292) — is
deferred. The prompt's deferred list is honored at every point.

## Run-context handoff to step 4

Lessons 142 + 143 + today install:

- Closure literal syntax with annotated and unannotated parameters
  (142, 143).
- Closure-as-value framing with rustc-named opaque type (142).
- First-call-fixes-the-type rule for unannotated parameters (143).
- The closure/`fn` asymmetry centered on capture (today).

This is the minimum closure surface needed before generic `Fn`-bound
parameters can be taught. Step 4 (`fn apply<F: FnMut(u32) -> u32>(f: F, x: u32) -> u32`)
introduces the `FnMut` trait bound on a generic function parameter
and lets the learner pass a closure to a function. Step 5 unpacks
the three-trait family `Fn` / `FnMut` / `FnOnce` against the three
capture modes. After step 5 the closure prereqs are complete and
the first closure-driven Iterator method (`for_each` or `map` per
audit §6) becomes teachable.

Today's `unlocks` lists step 4 and step 5 directly, plus the deferred
capture-mode classification move, plus the deferred mutable-capture
move, plus the deferred `move`-keyword move.
