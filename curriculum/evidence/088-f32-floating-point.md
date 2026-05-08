# Evidence — 088-f32-floating-point

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/088-f32-floating-point.rs`.
  The contrast and auxiliary `.rs` files are *not* committed; the
  transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (082-087).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Three load-bearing spans from the *Floating-Point Types*
subsection.

Lines 147-151 (the canonical introduction — the most load-bearing
single passage today):

> Rust also has two primitive types for *floating-point numbers*,
> which are numbers with decimal points. Rust's floating-point
> types are `f32` and `f64`, which are 32 bits and 64 bits in
> size, respectively. The default type is `f64` because on modern
> CPUs, it's roughly the same speed as `f32` but is capable of
> more precision. All floating-point types are signed.

Direct corpus warrant for the lesson's centered claims:

- *Two primitive types*: `f32` and `f64`. The lesson's *The Move*
  ("Lesson 033 installed *one* floating-point type ... The Book
  says Rust has *two*. Today names the second.") is the Book's
  count restated.
- *Bit widths*: 32 and 64. The lesson's *Mental Model Delta*
  "After" parenthetically names "(32 bits)" and "(64 bits)".
- *Default is `f64`*: the lesson's *What Changed* bullet 2 ("The
  unsuffixed float literal default is still `f64`") restates the
  Book's "The default type is `f64`". Lesson 033 already installed
  this; today reuses it.
- *`f64` is the default *because* speed-and-precision*: the
  lesson's *What Changed* bullet 3 quotes the Book sentence
  verbatim ("roughly the same speed as `f32` but is capable of
  more precision"), explicitly framed as a rule the lesson does
  not measure.
- *All floating-point types are signed*: the lesson's *Mental
  Model Delta* "After" ("Both types are signed") and *What
  Changed* bullet 1 ("Both are signed") restate the Book sentence.

Lines 153-163 (the Book's canonical `fn main` example — verbatim
shape used by the working probe):

> Here's an example that shows floating-point numbers in action:
>
> Filename: src/main.rs
>
> ```rust
> fn main() {
>     let x = 2.0; // f64
>
>     let y: f32 = 3.0; // f32
> }
> ```

Direct corpus warrant for the lesson's *Try It* working probe.
The lesson's probe renames `x` → `big` and `y` → `small` for
self-documenting names and adds two `println!` lines so the
program produces visible output, but the two `let` statements are
the Book's example bit-for-bit modulo the rename:

- `let big = 2.0;` (Book: `let x = 2.0;`) — no annotation; the
  unsuffixed literal `2.0` defaults to `f64`. The Book's inline
  comment `// f64` is the corpus warrant for this default. Lesson
  033 installed the same default rule.
- `let small: f32 = 3.0;` (Book: `let y: f32 = 3.0;`) — the
  `: f32` annotation pins the literal at `f32`. The Book's inline
  comment `// f32` is the corpus warrant for the annotation
  binding the type.

Line 165 (the IEEE-754 standard reference):

> Floating-point numbers are represented according to the IEEE-754
> standard.

Direct corpus warrant for the lesson's *Mental Model Delta*
"After" ("Both follow the IEEE-754 standard.") and *What Changed*
bullet 4 ("Rust's floating-point types follow the *IEEE-754*
standard"). The lesson installs the *name* only; mechanics stay
deferred (consistent with lesson 033's deferral).

### Sources NOT cited as load-bearing

- `output/docs/rust/std/primitive.f32.md` — std primitive page for
  `f32`. Consistent with the Book's row for `f32` (32-bit IEEE-754
  binary single-precision). Not separately quoted; the Book is
  the audience-level authority and the lesson does not exercise
  `f32`-specific stdlib methods.
- `output/docs/rust/std/primitive.f64.md` — std primitive page
  for `f64`. Same role; lesson 033 already installed `f64` and
  did not cite this page either.
- `output/docs/rust/reference/types/numeric.md` — Reference's
  formal numeric-types section. Today follows the Book's
  audience-level shape; the Reference's grammar-level treatment
  is not load-bearing.
- `output/docs/rust/error_codes/error_codes/E0308.md` — the
  `E0308` error code page. The contrast probe's diagnostic shape
  is the same as lesson 033's, and lesson 033's evidence already
  covered the page; not re-quoted today.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/088-f32-floating-point.rs`)
is the *working* version. The contrast probe and one auxiliary
probe are documented as separate runs below, not committed as
separate `.rs` files (matching the pattern of lessons 033, 080).

### Probe 1: working program

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
    let big = 2.0;        // f64 by default
    let small: f32 = 3.0; // f32 via annotation
    println!("big = {}", big);
    println!("small = {}", small);
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
big = 2
small = 3
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (no warnings, no errors),
  consistent with lesson 001.
- `./demo` prints exactly two lines, each witnessing a distinct
  claim:
  1. `big = 2` — `let big = 2.0;` compiled with no annotation.
     Witnesses the lesson-033 default rule still applies: an
     unsuffixed float literal infers `f64`. The displayed digits
     are `2` (not `2.0`) because the default `{}` formatter prints
     whole-valued floats without a trailing decimal — the *type*
     is `f64`, only the displayed representation happens to look
     integer-shaped here. The lesson's *Try It* prose names this
     explicitly.
  2. `small = 3` — `let small: f32 = 3.0;` compiled with the
     lesson-019 annotation. Witnesses (a) `f32` plugs into the
     `: TYPE` slot the way `f64`, `i32`, `u8`, etc. did, and (b)
     a float literal `3.0` is accepted at `f32` (the type `f32`
     accepts the same `.`-shaped literal lesson 033 installed,
     not just `f64`). Same display-rule note as `big`: `3.0`
     prints as `3` under default `{}` formatting.
- The committed `.rs` file's source matches the *Try It* code
  block exactly, modulo source-comment lines for the lesson's
  evidence pointer. Only the working source is committed under
  `observations/`.
- Two corners of the float family exercised: the *default*
  (`f64`, no annotation) and the *non-default* (`f32`,
  annotation). The lesson centers exactly this contrast.

### Probe 2: contrast — integer literal in `f32` slot

Same temp-dir family, separate file `broken.rs`. Identical in
shape to lesson 033's `f64 = 3` contrast, with `f32` substituted:

```text
--- cat broken.rs ---
fn main() {
    let small: f32 = 3;
    println!("small = {}", small);
}
--- rustc broken.rs ---
error[E0308]: mismatched types
 --> broken.rs:2:22
  |
2 |     let small: f32 = 3;
  |                ---   ^ expected `f32`, found integer
  |                |
  |                expected due to this
  |
help: use a float literal
  |
2 |     let small: f32 = 3.0;
  |                       ++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0308]: mismatched types`. Same coded `E0308`
  family as lessons 024-031, 033. Lesson 003 installed the headline
  shape; lesson 033 already saw this exact `E0308` text on `f64 =
  3`.
- **Location**: `broken.rs:2:22` — line 2, column 22, the literal
  `3` itself.
- **Source excerpt with caret**: `---` underlines `f32` with the
  sub-annotation `expected due to this`; `^` underlines `3` with
  the trailing annotation `expected `f32`, found integer`. The
  `f32` text on the *type* side is what makes this diagnostic the
  load-bearing parallel to lesson 033's: rustc itself names `f32`
  as the expectation set by the annotation, confirming `f32` is a
  real type the annotation slot accepts.
- **`help:` block**: same shape as lesson 033's. The headline-style
  line `help: use a float literal` states the fix; below it, an
  indented source diff `let small: f32 = 3.0;` with `++` under the
  new `.0` shows the smallest fix as a one-character source-text
  change. Same rule, both float types.
- **`--explain` trailer**: `For more information about this error,
  try `rustc --explain E0308`.` Same trailer lesson 003 / 070
  installed.
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's claim
that `f32` is a *real* type the annotation slot accepts (the
diagnostic's `expected `f32`` text is rustc itself naming `f32` in
the same slot it would name `f64`). It also corroborates the
lesson's "same rule, both float types" framing in *What Changed*:
the integer-in-float-slot rejection generalizes from `f64`
(lesson 033) to `f32` with the diagnostic shape preserved bit-for-
bit modulo the type name.

### Probe 3: auxiliary — precision witness for the Book's "more precision" claim

Captured for evidence transparency only. **Not** centered in the
lesson body. Documented to operationally witness the Book's "more
precision" claim (lines 149-150) without unpacking IEEE-754
mantissa-and-exponent mechanics. This probe is the only place in
the lesson workflow where an `_f32` literal suffix appears; the
suffix is *not* installed as a centered concept today (lesson
081's *What To Ignore* explicitly deferred float suffixes to "queue
item O" — this lesson's territory — and today defers the suffix
itself to a future move). The suffix is used here only because the
default-`f64` rule would otherwise force both literals to `f64`
and obscure the witness.

```text
--- cat prec.rs ---
fn main() {
    let third_64: f64 = 1.0 / 3.0;
    let third_32: f32 = 1.0_f32 / 3.0_f32;
    println!("third_64 = {}", third_64);
    println!("third_32 = {}", third_32);
}
--- rustc prec.rs ---
exit=0
--- ./prec ---
third_64 = 0.3333333333333333
third_32 = 0.33333334
exit=0
```

Notes:

- Same calculation `1 / 3` at two float types. The `f64` line
  prints 16 significant digits; the `f32` line prints 8. The two
  digit counts are roughly proportional to each type's storage
  width (64 bits vs 32 bits) — this is the operational shape of
  the Book's "more precision" claim, witnessed by `println!`'s
  default float formatter rather than by inspecting bit
  representations.
- The lesson's *What Changed* bullet 3 names this as observable
  ("The precision difference is observable; the appendix witnesses
  it.") without unpacking IEEE-754 mantissa rules. The probe
  delivers the witness; the audience-level rule is the Book's.
- The `_f32` suffix appears here for the operational reason given
  above. The lesson body does not introduce the suffix as a
  concept; *What To Ignore For Now* names it as deferred and
  cross-references this probe.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the lesson's
*real-type* claim (`f32` is a type rustc names in the same slot
it names `f64`) and for the *same rule, both float types* framing.
Probe 3 is auxiliary; its transcript corroborates the Book's
"more precision" sentence operationally, but is not load-bearing
for any centered claim today.

The lesson does not run a probe for *mixing* `f32` and `f64` in
arithmetic (`let x: f64 = 1.0_f32 + 2.0;` rejected without an
`as` cast); the rule is named in *What To Ignore For Now* and is
its own future move.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency, and the printed digits for
`big = 2` and `small = 3` reflect the default `{}` formatter's
choice to omit trailing decimals on whole-valued floats. The
default formatter's behavior is part of the std library's `Display`
implementation for `f32` and `f64`; the *audience-level* fact for
this lesson is just that "what `{}` prints for a whole-valued
float looks integer-shaped on this rustc release" — sufficient for
the lesson's "compiled and ran" witness.

Probe 2's headline (`error[E0308]: mismatched types`), the source
excerpt's `expected `f32`, found integer` annotation, and the
`help: use a float literal` source-diff are deterministic on this
rustc release. The exact wording is rustc-version-specific; the
*shape* — coded `E0308` with the source-diff `help:` — is grounded
in lesson 003's diagnostic map and is stable across recent
releases (matches lesson 033's `f64` form bit-for-bit modulo the
type name).

Probe 3's exact printed digits (`0.3333333333333333` for `f64`,
`0.33333334` for `f32`) depend on IEEE-754 rounding and on Rust's
default float formatter; the *count of significant digits* (16 vs
8) is the load-bearing observation, not the exact digits. The
count reflects the storage width and is stable across releases.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 088.

- **Lesson 003 (cited for the diagnostic map)** — installs the
  four-part read of headline + `-->` + source excerpt with caret
  + optional `help:` lines. Probe 2 is read with that map only;
  no new diagnostic vocabulary is installed today. The coded
  `E0308` shape was already covered by lesson 033 (and earlier
  lessons 024-031).
- **Lesson 005 (cited for `let name = value;`)** — installs the
  binding form. Today reuses it twice in the working probe with
  no extension.
- **Lesson 011 (cited for positional `{}` printing)** — installs
  `println!("{}", expr)`. The probe's two `println!` lines reuse
  this with no extension. Today does *not* install any new format
  specifier; the default `{}` printing of whole-valued floats as
  `2` and `3` (no trailing decimal) is named in the *Try It* prose
  as a fact-of-the-default-formatter and is documented in the
  Reproducibility note above.
- **Lesson 019 (load-bearing for the `: TYPE` slot)** — installs
  `let name: TYPE = value;` as a *type annotation*. Today plugs
  one new type name (`f32`) into the slot. Lesson 019's *What To
  Ignore For Now* implicitly leaves "other type names" as future
  moves; today closes the `f32` line via lesson 033's family.
- **Lesson 033 (load-bearing for `f64`, float literals, and the
  default rule)** — installs `f64` as Rust's floating-point type;
  *float literal* as a number written with a `.` (`5.0`, `3.14`);
  the unsuffixed-float-literal default of `f64`; and the broken-
  contrast E0308 + `help: use a float literal` shape on `f64 = 3`.
  Today extends 033 from one float type to two, with the same
  default rule unchanged. Lesson 033's *What To Ignore For Now*
  named "*`f32`*, the 32-bit float type" verbatim as a deferred
  move with the corpus pointer to Book lines 147-151; today
  closes that line by installing `f32` with the same Book span as
  source. The contrast probe's diagnostic shape is identical to
  lesson 033's modulo the type name (`f32` substituted for
  `f64`), confirming the rule generalizes to the float family.
- **Lesson 080 (cited for the integer family parallel)** —
  installed twelve typed integer names by sign-and-width. Today
  cites this as the structural analogue: the float family is the
  same shape on a smaller scale (two names by bit width, both
  signed). Today does *not* depend on any specific integer-family
  claim from 080; the citation is for pedagogical parallel only.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success. Used as the compile-and-run shape for
  both probes.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches.
- `034-as-cast-i32-to-f64` — installed `as` casting for one
  cross-family direction. Mentioned in *What To Ignore For Now*
  only; `f32` and inverse cast directions remain deferred.
- `081-integer-literal-forms` — installed integer-literal type
  suffixes `57u8`, `1_000_i64`, etc. Mentioned in *What To Ignore
  For Now* only; the float-suffix parallel `2.0_f32`, `3.0_f64`
  is the natural future move and is *cross-referenced* with this
  lesson because the auxiliary precision-witness probe uses the
  suffix once. Lesson 081's *What To Ignore* named "queue item O"
  — this lesson — verbatim.
- `069-rustc-warnings`, `070-rustc-explain` — diagnostic-category
  infrastructure. Probe 2's coded `E0308` shape is read with
  lesson 003's map; lesson 070's `--explain` trailer appears
  unchanged in the contrast transcript.
- `082-cargo-build-release`, `083-integer-overflow`,
  `084-cargo-check`, `085-toolchain-housekeeping`,
  `086-rustup-doc`, `087-rustfmt` — most recent accepted lessons
  on the same host and toolchain. Mentioned only to confirm the
  host environment is unchanged.

No trait-related lesson is cited. The brief excludes trait
machinery and `Display` internals from today's territory; the
default-formatter behavior is named only at the audience level
("`{}` prints whole-valued floats without a trailing decimal").

## Book Ch1-3 closure-pass effect

This lesson **closes item O** in the Book Ch1-3 closure queue.
Item O's listed prereq was 033 (`f64`); 033 was installed at
cycle 033 and reaffirmed by this lesson's *Prerequisites* and
*Mental Model Delta*. Today carries out exactly the plan O
describes: one centered move that names the second floating-point
primitive, citing 033 for the already-installed default and the
float-literal shape, and reading the two-line Book example as
verbatim source for the working probe.

With `f32` installed, future floating-point moves (float-suffix
forms `2.0_f32` / `3.0_f64`, float-to-integer `as` casts,
integer-to-`f32` casts, IEEE-754 mechanics, float methods, format
specifiers like `{:.2}`) become directly approachable: each
already had `f64` installed by lesson 033, and today supplies the
sibling type name they need. The remaining Ch1-3 closure queue
items (beyond O) are unaffected.
