# Evidence — 079-for-over-array

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/079-for-over-array.rs`.
  The manual-`while`-indexed contrast is documented as a separate
  probe transcript below; not committed as a `.rs` file.

Same host and toolchain as the recent accepted lessons (072-078).

## Sources

### `output/docs/rust/book/ch03-05-control-flow.md`

The Book's *Looping Through a Collection with `for`* subsection runs
from line 446 to line 547. Today centers on the array iteration form
introduced at line 497 and the safety/conciseness conclusion at line
525.

Lines 497-509 (the canonical `for element in a` form — load-bearing
for the lesson's working probe and the centered shape):

> As a more concise alternative, you can use a `for` loop and execute
> some code for each item in a collection. A `for` loop looks like the
> code in Listing 3-5.
>
> Filename: src/main.rs
>
> ```rust
> fn main() {
>     let a = [10, 20, 30, 40, 50];
>
>     for element in a {
>         println!("the value is: {element}");
>     }
> }
> ```

Direct corpus warrant for the lesson's working probe shape. The
lesson's probe matches Listing 3-5 verbatim except for the
`println!` argument convention: the lesson uses positional
`println!("the value is: {}", element)` (lesson 011's installed
shape) instead of the Book's brace-named `{element}` (a separate
format-string DSL feature this run has not centered as the default
since 011). Both forms compile and produce identical output (probe
transcript below). The five-element array `[10, 20, 30, 40, 50]`
and the body line `println!("the value is: ...", element);` are
the Book's exact choices, reused so the lesson's transcript matches
the Book's expected output verbatim.

Lines 446-451 (the framing — "you *can* use `while`, but"):

> You can choose to use the `while` construct to loop over the
> elements of a collection, such as an array. For example, the loop
> in Listing 3-4 prints each element in the array `a`.

Direct corpus warrant for the lesson's contrastive framing. The Book
introduces array iteration *first* with the `while` form, then
contrasts it with the `for` form — exactly the structure the lesson
mirrors with its side-by-side `demo.rs` (the `for` form) and
`manual.rs` (the `while` form).

Lines 454-465 (the manual-`while`-indexed shape — Listing 3-4 — the
lesson's contrast probe):

> ```rust
> fn main() {
>     let a = [10, 20, 30, 40, 50];
>     let mut index = 0;
>
>     while index < 5 {
>         println!("the value is: {}", a[index]);
>
>         index += 1;
>     }
> }
> ```

The lesson's `manual.rs` probe is structurally identical to Listing
3-4 with three deliberate-but-immaterial substitutions:

1. `index < 5` → `index < a.len()`. Listing 3-4 hard-codes the
   length `5`; the lesson uses `a.len()` because (a) lesson 076
   installed `.len()` as the canonical "ask the array its length"
   call, and (b) the Book itself comments on this exact failure
   mode at lines 490-493 ("if you changed the definition of the
   `a` array to have four elements but forgot to update the
   condition to `while index < 4`, the code would panic"). Using
   `a.len()` makes the failure mode the lesson references in *Try
   It* — "write the wrong bound and `a[index]` panics" — visibly
   *fixed* in the contrast probe, which makes the comparison fair.
2. `index += 1;` → `index = index + 1;`. Lesson 023 installed `+=`
   but the lesson reaches back to lesson 009's plain
   `name = name + 1;` form to match the same shape lesson 017
   used. Either compiles. Both produce the same output.
3. The blank line between `let mut index = 0;` and the `while` is
   omitted in the lesson's probe, which is purely cosmetic.

Output is identical to Listing 3-4: five lines, `the value is: 10`
through `the value is: 50`. Probe 2 transcript below confirms.

Lines 467-472 (Listing 3-4 expected output — load-bearing for the
contrast probe's expected transcript):

> All five array values appear in the terminal, as expected. Even
> though `index` will reach a value of `5` at some point, the loop
> stops executing before trying to fetch a sixth value from the
> array.

Cited only operationally — the lesson does not lean on this
rationale because the `for` shape removes the failure-mode topic
entirely.

Lines 490-495 (the failure-mode argument — load-bearing for the
lesson's "bug surface" framing):

> However, this approach is error-prone; we could cause the program
> to panic if the index value or test condition is incorrect. For
> example, if you changed the definition of the `a` array to have
> four elements but forgot to update the condition to
> `while index < 4`, the code would panic. It's also slow, because
> the compiler adds runtime code to perform the conditional check
> of whether the index is within the bounds of the array on every
> iteration through the loop.

Direct corpus warrant for the lesson's *Try It* claim "write the
wrong bound and `a[index]` panics with lesson 078's runtime
message" and the *Mental Model Delta* "no chance of an out-of-
bounds read." The Book's argument has two pieces — error-prone (the
panic-failure-mode claim, load-bearing for the lesson) and slow
(the per-iteration-bounds-check claim, *not* load-bearing for the
lesson). The lesson stays with the safety claim only and leaves
the speed claim for a future codegen-related move.

Lines 514-523 (the safety + maintenance argument — load-bearing for
the lesson's "preferred shape" framing):

> When we run this code, we'll see the same output as in Listing
> 3-4. More importantly, we've now increased the safety of the
> code and eliminated the chance of bugs that might result from
> going beyond the end of the array or not going far enough and
> missing some items. Machine code generated from `for` loops can
> be more efficient as well because the index doesn't need to be
> compared to the length of the array at every iteration.
>
> Using the `for` loop, you wouldn't need to remember to change
> any other code if you changed the number of values in the array,
> as you would with the method used in Listing 3-4.

Direct corpus warrant for the lesson's *What Changed* third bullet
("the `for` shape is shorter and has no out-of-bounds failure
mode") and *Mental Model Delta* "no chance of an out-of-bounds
read." The "same output as in Listing 3-4" claim is what the
lesson's two-probe transcript pair operationally witnesses (Probe
1 and Probe 2 below).

Lines 525-528 (the conclusion — quoted in the lesson's *What
Changed* fourth bullet):

> The safety and conciseness of `for` loops make them the most
> commonly used loop construct in Rust. Even in situations in
> which you want to run some code a certain number of times, as
> in the countdown example that used a `while` loop in Listing
> 3-3, most Rustaceans would use a `for` loop.

Direct corpus warrant for the lesson's *What Changed* fourth
bullet, quoted with attribution as "The Book". The same Book
sentence licenses lesson 022's "most Rustaceans would use a `for`
loop" framing for ranges; today extends it to arrays.

### `output/docs/rust/reference/expressions/loop-expr.md`

Lines 193-225 (`expr.loop.for` — the `for` expression's formal
definition):

> ## Iterator loops
>
> [...]
> A `for` expression is a syntactic construct for looping over
> elements provided by an implementation of `std::iter::IntoIterator`.
>
> If the iterator yields a value, that value is matched against
> the irrefutable pattern, the body of the loop is executed, and
> then control returns to the head of the `for` loop. If the
> iterator is empty, the `for` expression completes.
>
> An example of a `for` loop over the contents of an array:
>
> ```rust
> let v = &["apples", "cake", "coffee"];
>
> for text in v {
>     println!("I like {}.", text);
> }
> ```

The Reference's example uses `&[...]` (a reference to the array),
not the by-value form the Book and lesson use. The lesson's
working probe is the by-value form `for element in a`, matching
the Book's Listing 3-5; the by-reference form `for x in &a` is
explicitly listed under *What To Ignore For Now*.

The Reference also names the formal mechanism — `IntoIterator`,
"iterator yields", "irrefutable pattern". None of those are
load-bearing for today's lesson; today is operational. The
deferral is named in *What To Ignore For Now* (`Iterator` trait,
`while let Some(x) = iter.next()`). The lesson's centered claim
"`for X in COLLECTION { ... }` accepts an array in the COLLECTION
slot" is a learner-grade restatement of the Reference's
"`std::iter::IntoIterator`" — narrowed to the only two collections
this run has installed (range, array).

### Sources NOT cited as load-bearing

- `output/docs/rust/std/primitive.array.md` — lesson 076 used this
  for the array-to-slice coercion that makes `.len()` work. Today
  also uses `.len()` (in the `while` contrast probe) but no new
  fact about it is installed; lesson 076 already supplies the call.
- `output/docs/rust/std/iter/index.md`, `std/iter/trait.Iterator.md`,
  `std/iter/trait.IntoIterator.md` — the formal iteration machinery
  the Reference names. All explicitly deferred under *What To
  Ignore For Now*. No quote needed today.
- `output/docs/rust/reference/types/array.md` — array type
  documentation; lesson 076's source. Today reuses 076 unchanged
  and does not need to re-cite the type page.
- `output/docs/rust/error_codes/*.md` — no E-coded diagnostic is
  captured today. Both probes succeed (compile and run cleanly,
  exit 0); there is no negative diagnostic-grounded claim.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/079-for-over-array.rs`)
is the *working* version (`for element in a`). The
manual-`while`-indexed contrast (Probe 2 below) is documented as a
separate run, not committed as a `.rs` file.

### Probe 1: working program — `for element in a`

Captured in a fresh empty temp dir created with `mktemp -d` and
removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo (stdout) ---
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
demo-exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- `./demo` prints exactly five lines, in array order. Each line
  comes from one pass through the body; `element` takes the
  values `10`, `20`, `30`, `40`, `50` in order.
- No `mut`, no counter, no `[index]` expression — the source has
  none. This is the load-bearing observation for the lesson's two
  centered claims:
  1. *The `for ... in ...` shape extends to arrays.* The body line
     `for element in a { ... }` compiles and runs without any
     additional machinery.
  2. *Iteration replaces manual indexing.* The body of the loop
     uses `element` directly; no `a[i]` expression appears in the
     source.

This is the lesson's working probe. The committed `.rs` is
identical to the `cat demo.rs` block above.

### Probe 2: positive contrast — manual `while`-indexed loop

Captured in a fresh `mktemp -d` directory, separate file
`manual.rs`. Not committed.

```text
--- cat manual.rs ---
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}
--- rustc manual.rs ---
rustc-exit=0
--- ./manual (stdout) ---
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
manual-exit=0
--- temp dir removed ---
```

Notes:

- Compiles silent, exits 0. Same five output lines as Probe 1, in
  the same order.
- This is the *positive* contrast called for in the brief: same
  task, same output, *different shape*. The lesson body's "count
  the moving parts" claim is operationally grounded here:
  - `manual.rs` source has 8 lines of `fn main` body content
    counting the `while` head and the closing `}`; `demo.rs` has 5.
  - `manual.rs` uses `let mut`, `<`, `a[index]`, `index = index +
    1;`. `demo.rs` uses none of those.
  - `manual.rs` is structurally identical to the Book's Listing 3-4
    with the three immaterial substitutions noted under *Sources*
    (`a.len()` instead of `5`, `index = index + 1;` instead of
    `index += 1;`, omitted blank line). All produce the same
    output.
- This satisfies the brief's option-(a) "manual `while`-indexed
  loop" contrast probe. No broken/negative contrast probe is
  captured today because the lesson's centered claim ("the shape
  extends from ranges to arrays") is *positive composition* — the
  shape works in a new context. The lesson's failure-mode framing
  ("write the wrong bound and `a[index]` panics") is grounded by
  lesson 078's already-captured Probe 1, not re-run today.

### Probe 3 (auxiliary, not committed): the Book's verbatim `{element}` form

Run only to confirm the Book's exact Listing 3-5 source
compiles-and-runs identically on this host:

```text
--- cat book_form.rs ---
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
--- rustc book_form.rs ---
rustc-exit=0
--- ./book_form (stdout) ---
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

The Book's brace-named `{element}` and the lesson's positional
`{}` are interchangeable for this shape. The lesson uses the
positional form because lesson 011 installed it as the run's
default; nothing here is load-bearing on the choice.

### Negative / contrast probes summary

The brief's option (a) is satisfied by Probe 2: a positive
contrast (same output, different shape) on the same host. The
lesson's centered claim is positive composition — the
`for ... in ...` shape from lesson 022 extends to arrays — so a
broken/negative probe is not required. The lesson's *secondary*
framing ("`for` removes the out-of-bounds failure mode") is
grounded by lesson 078's appendix, which captured the runtime
panic; no need to re-witness it today.

### Reproducibility note

Probes 1, 2, and 3 are deterministic on rustc 1.95.0 — no
randomness, no environment dependency, no per-run thread-id.
Each transcript line is byte-for-byte stable across runs.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 079.

- **Lesson 022 (load-bearing for the shape today extends)** —
  installed `for var in 0..N { ... }` as one of Rust's three loop
  shapes. Today extends the COLLECTION slot from ranges to arrays.
  The exact phrase the lesson uses ("Same `for X in COLLECTION
  { ... }` shape as a range loop; just put an array where the range
  used to be") restates lesson 022's main concept with the slot
  generalized. Lesson 022's *What To Ignore For Now* explicitly
  named "*Iterating over arrays and other collections*, e.g. `for
  element in a`" as the deferred next step; today closes that
  deferral. Lesson 022 also already cited Book lines 525-528 for
  the "most Rustaceans would use a `for` loop" framing; today
  reuses the same line attached to arrays instead of ranges.

- **Lesson 076 (load-bearing for the array value)** — installed
  `[v1, v2, ...]` and the array type `[T; N]`. Today's working
  probe builds the array exactly as 076 taught (`let a = [10, 20,
  30, 40, 50];`) and feeds it into the COLLECTION slot of `for ...
  in ...`. No new fact about array construction or types is
  installed. Lesson 076's *What To Ignore For Now* named "*`for
  element in array` iteration* — queue item F" as the explicit
  deferred move; today closes that deferral.

- **Lessons 001, 002 (load-bearing for compile-and-run)** —
  installed `rustc file.rs` then `./name`, with rustc silent on
  success. Both probes today rely on this two-step shape unchanged.
  No new fact installed.

- **Lesson 005 (load-bearing for `let`)** — installed `let name =
  value;`. The working probe uses `let a = [10, 20, 30, 40, 50];`
  exactly as 005 taught. No new fact installed.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `004-statements-in-order` — `fn main`'s body runs top to bottom.
  The `for` is one such step; the loop completes before any
  hypothetical statement after it would run. Reused unchanged.
- `006-mut-binding` — `let mut name = value;` reassignability.
  Used only in the contrast probe `manual.rs`'s `let mut index =
  0;`; the working probe `demo.rs` does not need `mut`. No new
  fact installed.
- `009-arithmetic-on-integers` — `+` between integers. Used only
  in the contrast probe's `index = index + 1;`. Reused unchanged.
- `011-println-positional-args` — `println!("...{}", expr)`. Both
  probes use this form for output. Reused unchanged.
- `013-comparison-operators` — `<` between integers produces a
  `bool`. Used only in the contrast probe's `index < a.len()`.
  Reused unchanged.
- `017-while-loop` — `while condition { ... }` is the contrast
  probe's outer shape. Reused unchanged.
- `040-method-call-syntax` — `value.method()`. Used only in the
  contrast probe's `a.len()` call. Lesson 076 already used this
  shape on arrays; today does not extend it.
- `077-array-indexing-and-usize` — `a[i]` and `usize`. Used only
  in the contrast probe's `a[index]` (with `index: usize` inferred
  from `let mut index = 0;` plus the use as an array index — the
  same inference 077 documented). Reused unchanged. *Not* used by
  the working probe.
- `078-array-out-of-bounds-panic` — the runtime out-of-bounds
  panic. Referenced only in *Try It* and *Mental Model Delta* as
  the *failure mode iteration avoids*; no new fact installed.

## Book Ch1-3 closure-pass effect

This lesson **closes item F** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item F's listed prereqs were C (array — lesson 076) and 022
(for-range form). Today carries out exactly that plan: the
COLLECTION-slot extension lands, with 022 + 076 as the load-bearing
prior lessons.

With `for element in array` installed, the array arc — C
(literal/type/repeat-init) → D (indexing + `usize`) → E
(out-of-bounds runtime panic) → F (for-iteration) — closes. Queue
items G (full integer family), H (literal forms), I (overflow),
J/K (cargo build profiles), L (`Cargo.lock`), M (toolchain
housekeeping), N (`rustup doc`), O (`f32`), P (loop labels), Q
(`(1..N).rev()`), R (snake_case), S (function definition order),
T (`rustfmt`), U (prelude), V (`unused_must_use`) all remain
independently approachable; none of them depend on item F.

The Ch3-5 *Looping Through a Collection with `for`* subsection is
fully covered after today: lesson 017 covered Listing 3-3 (`while`
countdown), lesson 022 covered the `for ... in 0..N` form and the
range-as-iterator framing, today covers Listings 3-4 (`while`
indexed) and 3-5 (`for element in a`) and the Book's
safety/conciseness conclusion. Listing 3-6 (the `(1..4).rev()`
countdown) remains queue item Q — a small composition needing
only `.rev()`.
