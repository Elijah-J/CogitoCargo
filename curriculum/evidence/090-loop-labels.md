# Evidence — 090-loop-labels

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the
  end. Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/090-loop-labels.rs`.
  The contrast probes (bare `break;`, bare `continue;`,
  `continue 'outer;`, missing-quote syntax error, Book example)
  are *not* committed as separate `.rs` files; their transcripts
  below are the artifacts.

Same host and toolchain as recent accepted lessons (082-089).

## Sources

### `output/docs/rust/book/ch03-05-control-flow.md`

The Book's *Disambiguating with Loop Labels* subsection,
lines 359-409. Three load-bearing spans.

Lines 361-365 (the canonical introduction — the most load-bearing
single passage today):

> If you have loops within loops, `break` and `continue` apply to
> the innermost loop at that point. You can optionally specify a
> *loop label* on a loop that you can then use with `break` or
> `continue` to specify that those keywords apply to the labeled
> loop instead of the innermost loop. Loop labels must begin with
> a single quote.

Direct corpus warrant for the lesson's centered claims:

- *Default rule for `break`/`continue` is innermost*: The Book's
  first sentence restates lessons 027 and 035. The lesson's
  *Mental Model Delta* "Before" and *What Changed* bullet 3 do
  not extend the bare-form rule.
- *Optional `loop label` to target a non-innermost loop*: the
  Book's "you can optionally specify a *loop label*" sentence is
  the canonical introduction. The lesson's *The Move* paragraph
  and *What Changed* bullet 1 restate the rule.
- *Single-quote prefix is required*: Book sentence "Loop labels
  must begin with a single quote." The lesson's *What Changed*
  bullet 4 restates this requirement; the syntax-error contrast
  probe (Probe 4 below) witnesses rustc's diagnostic when the `'`
  is omitted.

Lines 367-389 (the Book's canonical doubly-nested example):

> ```rust
> fn main() {
>     let mut count = 0;
>     'counting_up: loop {
>         println!("count = {count}");
>         let mut remaining = 10;
>
>         loop {
>             println!("remaining = {remaining}");
>             if remaining == 9 {
>                 break;
>             }
>             if count == 2 {
>                 break 'counting_up;
>             }
>             remaining -= 1;
>         }
>
>         count += 1;
>     }
>     println!("End count = {count}");
> }
> ```

Direct corpus warrant for the *probed* shape: a labeled outer loop
nesting an unlabeled inner loop, with `break 'name;` inside the
inner loop targeting the outer. The lesson's working probe uses a
smaller, terminating shape (two `for ... in 0..3` loops, single
`break 'outer;` triggered on the first match) to cut down moving
parts while preserving the same rule. Probe 5 below reproduces the
Book's verbatim source and confirms its expected output.

Lines 391-393 (the Book's plain-English explanation of the
example):

> The outer loop has the label `'counting_up`, and it will count up
> from 0 to 2. The inner loop without a label counts down from 10
> to 9. The first `break` that doesn't specify a label will exit
> the inner loop only. The `break 'counting_up;` statement will
> exit the outer loop.

Direct corpus warrant for the lesson's contrast framing in *Try
It* and *What Changed* bullet 3 ("Bare `break;` and `continue;` are
unchanged ... they still apply to the *innermost* enclosing loop"):
the Book itself sets the bare-`break`-exits-only-the-inner contrast
in plain English alongside the labeled-`break`-exits-the-outer rule.

### `output/docs/rust/reference/expressions/loop-expr.md`

The Reference's formal definition of loop labels and labeled
control-flow expressions. Three load-bearing spans.

Lines 295-296 (the canonical definition of loop labels and the
syntax across all three loop forms):

> A loop expression may optionally have a *label*. The label is
> written as a lifetime preceding the loop expression, as in
> `'foo: loop { break 'foo; }`, `'bar: while false {}`,
> `'humbug: for _ in 0..0 {}`.

Direct corpus warrant for the lesson's centered claim that labels
work on `loop`, `while`, and `for` (the *What Changed* bullet 1's
"All three loop forms accept labels"). The Reference shows one
example for each loop construct, demonstrating the syntax once per
form. The lesson's working probe uses the `for`-loop spelling.

Lines 297-299 (the formal control-flow rule):

> If a label is present, then labeled `break` and `continue`
> expressions nested within this loop may exit out of this loop or
> return control to its head. See [break expressions] and [continue
> expressions].

Direct corpus warrant for the lesson's *What Changed* bullet 2:
labeled `break 'name;` *exits* the labeled loop, and labeled
`continue 'name;` *returns control to its head*. The phrase
"return control to its head" is the same Reference vocabulary
lesson 035 used.

Lines 354-355 (the formal `break` rule with labels):

> A `break` expression is normally associated with the innermost
> `loop`, `for` or `while` loop enclosing the `break` expression,
> but a [label] can be used to specify which enclosing loop is
> affected.

Lines 467-469 (the parallel formal `continue` rule with labels):

> Like `break`, `continue` is normally associated with the
> innermost enclosing loop, but `continue 'label` may be used to
> specify the loop affected.

These two passages give the parallel formal statements of the rule
for `break` and `continue` respectively. They license the lesson's
*Mental Model Delta* "After" and *What Changed* bullets framing
labeled `break` and labeled `continue` as the same machine on two
loop-control statements.

### Sources NOT cited as load-bearing

- `output/docs/rust/reference/expressions/loop-expr.md` lines
  301-318 (the *Labels follow the hygiene and shadowing rules of
  local variables* paragraph plus its `'a: loop { 'a: loop ... }`
  example). Not load-bearing today; named in *What To Ignore For
  Now* under "Label-shadowing rules" for completeness.
- `output/docs/rust/reference/expressions/loop-expr.md` line 318
  (`'_` is not a valid loop label). Edge case; not load-bearing.
- The Reference's labeled-block-expression section (lines 375-413).
  Today's lesson centers labels on the three *loop* forms only;
  labeled bare blocks are named in *What To Ignore* and deferred.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/090-loop-labels.rs`)
is the *working* version. Four contrast/auxiliary probes are
documented as separate runs below, not committed as separate `.rs`
files (matching the pattern of lessons 088, 089).

### Probe 1: working program — `break 'outer;` exits both loops

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
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                println!("i = {i}, j = {j}: break 'outer");
                break 'outer;
            }
            println!("i = {i}, j = {j}");
        }
    }
    println!("after the labeled loops");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
i = 0, j = 0
i = 0, j = 1: break 'outer
after the labeled loops
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson
  001 — neither warnings nor errors; in particular, the
  `'outer:` label is *used* by the inner `break 'outer;`, so the
  `unused_labels` lint stays quiet).
- `./demo` prints exactly three lines:
  1. `i = 0, j = 0` — the inner loop's first pass on the outer
     `i = 0` iteration. Skips the inner `if` (j != 1), runs the
     trailing `println!`.
  2. `i = 0, j = 1: break 'outer` — the inner loop's second pass.
     Inner `if` is true; the inner block prints this line then
     executes `break 'outer;`.
  3. `after the labeled loops` — execution resumes after the
     closing `}` of the *outer* loop. The outer body never sees
     `i = 1` or `i = 2`.
- The absence of `i = 1, j = ...` and `i = 2, j = ...` lines is
  the load-bearing observation for the centered claim:
  `break 'outer;` exits the *outer* loop, not just the inner. If
  it had targeted only the innermost (the way bare `break;`
  does), the outer loop would have continued and we would see
  more output (Probe 2 confirms this contrast).
- Only the working source is committed under `observations/`. No
  binary is committed.

### Probe 2: contrast — bare `break;` exits only the inner loop

Same temp-dir family, separate file `broken.rs`. The only
difference from `demo.rs` is `break 'outer;` -> `break;`:

```text
--- cat broken.rs ---
fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                println!("i = {i}, j = {j}: break (no label)");
                break;
            }
            println!("i = {i}, j = {j}");
        }
    }
    println!("after the labeled loops");
}
--- rustc broken.rs ---
warning: unused label
 --> broken.rs:2:5
  |
2 |     'outer: for i in 0..3 {
  |     ^^^^^^
  |
  = note: `#[warn(unused_labels)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted

exit=0
--- ./broken ---
i = 0, j = 0
i = 0, j = 1: break (no label)
i = 1, j = 0
i = 1, j = 1: break (no label)
i = 2, j = 0
i = 2, j = 1: break (no label)
after the labeled loops
exit=0
```

Read with lesson 003's diagnostic map:

- **Headline**: `warning: unused label`. Lesson 069's category —
  `warning:`, not `error:`. Exit 0 and the executable was
  produced (`./broken` runs).
- **Location**: `broken.rs:2:5` — line 2, column 5 (the `'outer`
  label).
- **Source excerpt with caret**: `^^^^^^` underlines `'outer`.
- **`= note:`**: `#[warn(unused_labels)]` (part of `#[warn(unused)]`)
  on by default. Names the specific lint, *which is itself a future
  move* (named in *What To Ignore For Now*).

This is the load-bearing negative witness for the lesson's
contrast framing: with the *same* nested-loop shape but a *bare*
`break;` instead of `break 'outer;`, the program prints six inner
lines (not just two) — the outer loop continues for `i = 1` and
`i = 2`, restarting the inner loop each time and breaking out of
only the inner one. The contrast is a single-keyword swap with a
visible change in control flow. The `unused_labels` warning is a
side benefit: rustc itself names the rule the lesson teaches by
noticing that the label was never used.

The witness is observational, not diagnostic: the warning fires
because the label is dead, not because the program is wrong. The
program compiles and runs.

### Probe 3: `continue 'outer;` returns to the outer loop's head

Same temp-dir family, separate file `cont.rs`. Witnesses the
parallel rule for `continue`:

```text
--- cat cont.rs ---
fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                println!("i = {i}, j = {j}: continue 'outer");
                continue 'outer;
            }
            println!("i = {i}, j = {j}");
        }
        println!("end of outer body i = {i}");
    }
    println!("after the labeled loops");
}
--- rustc cont.rs ---
exit=0
--- ./cont ---
i = 0, j = 0
i = 0, j = 1: continue 'outer
i = 1, j = 0
i = 1, j = 1: continue 'outer
i = 2, j = 0
i = 2, j = 1: continue 'outer
after the labeled loops
exit=0
```

Notes:

- `rustc cont.rs` exits 0 and is silent.
- `./cont` prints seven lines. For each outer pass `i = 0, 1, 2`,
  the inner loop runs two passes (`j = 0` prints the witness
  line; `j = 1` triggers `continue 'outer;`).
- The load-bearing observation is the *absence* of any
  `end of outer body i = ...` line. That `println!` lives between
  the closing `}` of the inner loop and the closing `}` of the
  outer loop. If `continue;` (bare) had been used, control would
  return to the *inner* loop's head and `j` would advance to `2`,
  finishing the inner loop, after which the outer body's
  `println!("end of outer body i = {i}")` would print. With
  `continue 'outer;`, control returns to the *outer* loop's head
  directly — skipping the rest of the inner body *and* the rest
  of the outer body. The lesson's *Mental Model Delta* "After"
  and *What Changed* bullet 2 frame this as "skipping the rest of
  every nested body in between". This probe is the load-bearing
  witness for that phrase.

### Probe 4: contrast — missing single-quote rejected

Same temp-dir family, separate file `syn.rs`. Witnesses the Book's
"Loop labels must begin with a single quote" rule:

```text
--- cat syn.rs ---
fn main() {
    outer: for i in 0..3 {
        if i == 1 { break outer; }
    }
}
--- rustc syn.rs ---
error: malformed loop label
 --> syn.rs:2:5
  |
2 |     outer: for i in 0..3 {
  |     ^^^^^
  |
help: use the correct loop label format
  |
2 |     'outer: for i in 0..3 {
  |     +

error[E0425]: cannot find value `outer` in this scope
 --> syn.rs:3:27
  |
2 |     outer: for i in 0..3 {
  |     ----- a label with a similar name exists
3 |         if i == 1 { break outer; }
  |                           ^^^^^
  |                           |
  |                           not found in this scope
  |                           help: use the similarly named label: `'outer`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
exit=1
```

Read with lesson 003's diagnostic map:

- **First headline**: `error: malformed loop label` — uncoded
  diagnostic (no `E####`), names the rule directly.
- **First location**: `syn.rs:2:5` (the `outer` text without the
  leading `'`).
- **First `help:` block**: `help: use the correct loop label
  format` plus the source-diff suggestion `'outer:` with `+`
  under the new `'`. rustc itself names the fix: prepend a
  single-quote.
- **Second headline**: `error[E0425]: cannot find value `outer`
  in this scope`. This is the same `E0425` error code lesson 005
  already saw — rustc parsed `outer` (without the `'`) at the
  *use site* as an ordinary value name and looked for a `let`
  binding named `outer`, which does not exist. The
  `--> --- a label with a similar name exists` annotation is
  rustc cross-referencing the malformed-label site for the
  learner.
- **Exit**: 1; no executable produced.

This probe is the load-bearing negative witness for *What
Changed* bullet 4 ("The leading single quote is required.
`outer:` without the `'` is a syntax error: rustc rejects it
with `error: malformed loop label` and offers `'outer:` as the
fix"). It also corroborates the framing that the `'` is the
syntactic discriminator: without it, `outer` is parsed as a
regular identifier and falls through into the value-name
lookup that produces `E0425`.

### Probe 5: Book example — verbatim reproduction

Captured to confirm the Book's lines 367-409 example reproduces
exactly on this rustc release:

```text
--- cat book.rs ---
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
--- rustc book.rs ---
exit=0
--- ./book ---
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
exit=0
```

Notes:

- The output matches the Book's lines 400-409 verbatim (nine
  output lines, ending in `End count = 2`).
- This probe is *corroborative*, not load-bearing for any
  centered lesson claim. It witnesses that the Book's example —
  written for a different audience and using two `loop`
  constructs — also reproduces on this rustc release. The
  lesson's *Try It* probe uses a smaller `for`-based shape; this
  Book-shape probe is the audit substitute confirming the
  corpus's authority on this run's rustc release.

### Negative / contrast probes — coverage map

The lesson makes three contrastive claims; each maps to a probe:

1. *Bare `break;` exits only the innermost loop* (lesson 027
   reaffirmed; *What Changed* bullet 3). Probe 2 witnesses this
   directly: same source modulo `break 'outer;` -> `break;`,
   visible change in output (3 lines -> 6 lines).
2. *`continue 'name;` returns control to the named loop's head,
   skipping every nested body in between* (*Mental Model Delta*
   "After"; *What Changed* bullet 2). Probe 3 witnesses the
   "skipping the rest of the outer body" half via the absence of
   `end of outer body i = ...` lines.
3. *The leading single quote is required* (*What Changed*
   bullet 4). Probe 4 witnesses the `error: malformed loop
   label` diagnostic with rustc's own `+ '` source-diff fix.

### Reproducibility note

All five probes are deterministic on rustc 1.95.0. The programs
have no randomness or environment dependency. Probe 1's exact
three-line output, Probe 2's exact six-inner-line + warning shape,
Probe 3's seven-line output (with the missing
`end of outer body i = ...` lines as the centered observation),
Probe 4's `error: malformed loop label` + `error[E0425]:` shape
(with the `+ '` source-diff `help:`), and Probe 5's nine-line
output reproducing the Book all reflect rustc 1.95.0 behavior. The
exact diagnostic *wording* is rustc-version-specific, but the
*shape* — coded `E0425` on the use site, uncoded malformed-label
on the definition site, source-diff `help:` — is grounded in
lesson 003's diagnostic map and is stable across recent releases.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 090.

- **Lesson 027 (load-bearing) — `break;` exits the innermost
  enclosing loop**. Today's *Mental Model Delta* "Before" restates
  this; *What Changed* bullet 3 reaffirms that bare `break;` is
  unchanged. The lesson *adds* the `break 'name;` form for
  targeting an outer loop; it does *not* replace the bare form.
  The Book itself frames the relationship between bare `break`
  and labeled `break` in this way at lines 391-393 (Probe 5's
  example).
- **Lesson 035 (load-bearing) — `continue;` skips to the next
  iteration of the innermost enclosing loop**. Today's *What
  Changed* bullet 3 reaffirms that bare `continue;` is unchanged.
  The lesson adds `continue 'name;` as the parallel labeled form
  for targeting an outer loop; the Reference's lines 467-469
  state this rule explicitly (cited above). Probe 3 witnesses the
  `continue 'name;` half empirically.
- **Lesson 022 (cited) — `for var in 0..N { ... }`**. Used in
  every probe (Probes 1-3) as both the inner and outer loop
  construct. The probe could equivalently use `loop { ... break;
  }` or `while ... { ... }`; `for` over a finite range is chosen
  because it terminates by construction without needing a counter
  variable, keeping probe noise low.
- **Lesson 089 (cited) — snake_case naming convention**. The
  label name after the leading `'` follows the same convention
  (`'outer`, `'counting_up`, `'rows`). The lesson's *What
  Changed* bullet 5 names this; today does *not* install a new
  naming-convention rule.
- **Lessons 011, 013, 014, 017** — cited as background. `println!`
  for visible output (011); `==` producing a boolean (013); `if
  condition { ... }` gating the labeled `break`/`continue` (014);
  `while` named in the *What Changed* "All three loop forms" bullet
  (017) but not exercised in the probe.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success. Used as the compile-and-run shape for
  every probe.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches.
- `003-read-rustc-diagnostic` — the four-part diagnostic map
  (headline + `-->` + source excerpt with caret + optional
  `help:`/`note:`). Probes 2 and 4 are read with this map.
- `005-let-binding`, `006-mut-binding` — `let mut` used in
  Probe 5 (the Book's example) but not in the probes that ground
  centered claims.
- `028-break-value` — named in *What To Ignore For Now*: combining
  today's labeled `break` form with 028's `break value;` to return
  a value from a labeled outer loop is composition of two
  installed moves, deferred.
- `069-rustc-warnings` — Probe 2's `warning: unused label`
  headline + exit 0 + produced executable is read with 069's
  category map; the `unused_labels` lint is one specific lint in
  069's general category and is named in *What To Ignore For
  Now*.
- `082-cargo-build-release`, `083-integer-overflow`,
  `084-cargo-check`, `085-toolchain-housekeeping`,
  `086-rustup-doc`, `087-rustfmt`, `088-f32-floating-point`,
  `089-ch3-3-function-conventions` — most recent accepted lessons
  on the same host and toolchain. Mentioned only to confirm the
  host environment is unchanged.

## Book Ch1-3 closure-pass effect

This lesson **closes item P** in the Book Ch1-3 closure queue.
Item P's listed prereqs were 027 (`break;`) and 035 (`continue;`);
both were installed earlier in this run. Today carries out the
plan P describes: one centered move that installs the loop-label
syntax (`'name:`) and the labeled control-flow forms
(`break 'name;`, `continue 'name;`), reading Book Ch3-5 lines
359-409 as the canonical pedagogical source and
`reference/expressions/loop-expr.md` lines 295-299, 354-355, and
467-469 as the formal-statement source. The Book's verbatim
two-counter example is corroborated by Probe 5 but the lesson
centers a smaller `for`-based working probe to reduce moving
parts.

With loop labels installed, future moves become directly
approachable: `break 'name value;` (composing today + lesson 028;
fourth surface of lesson 024's expression rule on a labeled loop),
labeled bare blocks (`'name: { ... break 'name; }`), and the
`unused_labels` lint as a centered concept. The remaining Ch1-3
closure queue items (beyond P) are unaffected.
