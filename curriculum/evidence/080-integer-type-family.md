# Evidence — 080-integer-type-family

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/080-integer-type-family.rs`.
  The contrast and auxiliary `.rs` files are *not* committed; the
  transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (074-079).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Five load-bearing spans from the *Integer Types* subsection.

Lines 56-61 (the *Integer Types* opener):

> An *integer* is a number without a fractional component. We used
> one integer type in Chapter 2, the `u32` type. This type
> declaration indicates that the value it's associated with should
> be an unsigned integer (signed integer types start with `i`
> instead of `u`) that takes up 32 bits of space. Table 3-1 shows
> the built-in integer types in Rust. We can use any of these
> variants to declare the type of an integer value.

Direct corpus warrant for two of the lesson's load-bearing claims:
*signed integer types start with `i` instead of `u`* (the lesson's
*sign* axis) and *takes up 32 bits of space* (the lesson's *bit
width* axis as a rule, not just a label).

Lines 63-72 (Table 3-1, *Integer Types in Rust*):

> | Length | Signed | Unsigned |
> | --- | --- | --- |
> | 8-bit | `i8` | `u8` |
> | 16-bit | `i16` | `u16` |
> | 32-bit | `i32` | `u32` |
> | 64-bit | `i64` | `u64` |
> | 128-bit | `i128` | `u128` |
> | Architecture-dependent | `isize` | `usize` |

Direct corpus warrant for the lesson's enumeration of all twelve
typed integer names. The lesson's "twelve names total: `i8` `u8`
`i16` `u16` `i32` `u32` `i64` `u64` `i128` `u128` `isize` `usize`"
in *The Move* and *What Changed* are the table's twelve cells in
left-to-right, top-to-bottom order. The Book's *Length* column
gives the lesson's six widths (`8`, `16`, `32`, `64`, `128`, and
the Architecture-dependent row for `size`).

Lines 74-81 (the signed/unsigned framing, the range formulas, and
two's complement):

> Each variant can be either signed or unsigned and has an
> explicit size. *Signed* and *unsigned* refer to whether it's
> possible for the number to be negative—in other words, whether
> the number needs to have a sign with it (signed) or whether it
> will only ever be positive and can therefore be represented
> without a sign (unsigned). It's like writing numbers on paper:
> When the sign matters, a number is shown with a plus sign or a
> minus sign; however, when it's safe to assume the number is
> positive, it's shown with no sign. Signed numbers are stored
> using [two's complement] representation.

Direct corpus warrant for the lesson's *Signed vs unsigned* axis
gloss. The Book's "whether it's possible for the number to be
negative" is exactly the lesson's "the first letter says whether
the type can hold negative values." The two's-complement sentence
is named in *What To Ignore For Now* as a deferred storage detail.

Lines 83-87 (the range formulas plus `i8` and `u8` examples — the
load-bearing range claim):

> Each signed variant can store numbers from −(2n − 1) to 2n −
> 1 − 1 inclusive, where *n* is the number of bits that variant
> uses. So, an `i8` can store numbers from −(27) to 27 − 1, which
> equals −128 to 127. Unsigned variants can store numbers from 0
> to 2n − 1, so a `u8` can store numbers from 0 to 28 − 1, which
> equals 0 to 255.

The most load-bearing single passage today. The lesson's two range
formulas — "a signed `iN` holds `-(2^(N-1))` through `2^(N-1) - 1`
inclusive; an unsigned `uN` holds `0` through `2^N - 1`" — are the
Book's formulas re-rendered in plain ASCII (the Book uses
typographically-set superscripts; the lesson uses `^` to keep the
exponent rendering audience-friendly). The lesson's two concrete
ranges (`u8` is `0..=255`, `i8` is `-128..=127`) are also the
Book's two concrete examples in this passage, in identical
numeric form.

Lines 89-91 (architecture dependence):

> Additionally, the `isize` and `usize` types depend on the
> architecture of the computer your program is running on: 64
> bits if you're on a 64-bit architecture and 32 bits if you're
> on a 32-bit architecture.

Direct corpus warrant for the lesson's "architecture-dependent —
64 bits on a 64-bit machine, 32 bits on a 32-bit machine" gloss.
The lesson cites lesson 077 for the actual install (077's *Mental
Model Delta* and *What Changed* both used the same Book sentence),
so today only repeats the gloss to keep the family enumeration
self-contained.

Lines 109-112 (the integer-literal default and when to use
`isize`/`usize`):

> So how do you know which type of integer to use? If you're
> unsure, Rust's defaults are generally good places to start:
> Integer types default to `i32`. The primary situation in which
> you'd use `isize` or `usize` is when indexing some sort of
> collection.

Direct corpus warrant for the lesson's "when there is no
annotation and rustc cannot otherwise figure the type out, an
integer literal becomes `i32`" claim. The "primary situation"
sentence is named in *What To Ignore For Now* as the operational
advice today does not unpack.

### `output/docs/rust/std/primitive.u8.md`

Line 8 ("The 8-bit unsigned integer type") and lines 18-26 (the
`MIN` / `MAX` constants, with `assert_eq!(u8::MIN, 0);` and the
analogous `MAX` value):

> The 8-bit unsigned integer type.

Cross-corroborates the Book's `u8` row by giving the std page's
own one-line description. The page lists `MIN = 0` and `MAX = 255`
(implicitly via the `2^8 - 1` formula and the `255` example in
the Book), which agrees with the lesson's `0..=255` claim. Not
quoted in the lesson body; named here for cross-corpus
verification.

### `output/docs/rust/std/primitive.i8.md`

Line 8 ("The 8-bit signed integer type") and lines 18-26 (the
`MIN` constant `−2^7 = −128`, with `assert_eq!(i8::MIN, -128);`):

> The 8-bit signed integer type.

Cross-corroborates the Book's `i8` row, with `MIN = -128`
matching the lesson's `-128..=127` range. Not quoted; same role
as `primitive.u8.md`.

### `output/docs/rust/std/primitive.i64.md`

Line 8 ("The 64-bit signed integer type") and line 26
(`assert_eq!(i64::MIN, -9223372036854775808);`):

> The 64-bit signed integer type.

Cross-corroborates the working probe's choice of `i64` for nine
billion: `i64::MAX == 9_223_372_036_854_775_807`, which is past
nine billion, so `9_000_000_000` fits. Used silently; the
lesson's "nine billion is past `i32`'s top, fits comfortably
here" sentence relies on this fact.

### Sources NOT cited as load-bearing

- `output/docs/rust/std/primitive.u16.md`,
  `output/docs/rust/std/primitive.i16.md`,
  `output/docs/rust/std/primitive.u32.md`,
  `output/docs/rust/std/primitive.u64.md`,
  `output/docs/rust/std/primitive.i128.md`,
  `output/docs/rust/std/primitive.u128.md`,
  `output/docs/rust/std/primitive.isize.md`,
  `output/docs/rust/std/primitive.usize.md` — std primitive pages
  for the remaining integer types. Each is consistent with the
  Book's Table 3-1 row for the type. Not separately quoted to
  avoid over-citation; the Book table is the audience-level
  authority and the lesson does not exercise these types
  individually beyond naming them.
- `output/docs/rust/error_codes/index.md` — the
  `overflowing_literals` lint is not a coded `E####` error, so
  there is no E-code page to cite. The contrast probe's
  diagnostic carries no `E####` headline, just `error: literal
  out of range for `u8``. Lesson 069's category map covers this
  uncoded shape; today does not extend it.
- `output/docs/rust/book/ch03-02-data-types.md` lines 93-107
  (Table 3-2, integer literal forms) — explicitly named as queue
  item H. The probe uses one digit-separator as a *passing*
  citation only; the full notation table is deferred.
- `output/docs/rust/book/ch03-02-data-types.md` lines 114-143
  (*Integer Overflow*) — explicitly named as queue item I in the
  Book Ch1-3 closure queue. Today does not center overflow; the
  contrast probe's *literal*-out-of-range diagnostic is *compile-
  time-evaluable* (the literal is constant), and is its own rule
  separate from runtime arithmetic overflow.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/080-integer-type-family.rs`)
is the *working* version. The contrast probe and one auxiliary
probe are documented as separate runs below, not committed as
separate `.rs` files (matching the pattern of lessons 062, 074,
077).

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
    let small: u8 = 250;
    let signed_small: i8 = -100;
    let big: i64 = 9_000_000_000;
    let architecture: usize = 42;
    println!("u8 small = {}", small);
    println!("i8 signed_small = {}", signed_small);
    println!("i64 big = {}", big);
    println!("usize architecture = {}", architecture);
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
u8 small = 250
i8 signed_small = -100
i64 big = 9000000000
usize architecture = 42
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (no warnings, no errors),
  consistent with lesson 001.
- `./demo` prints exactly four lines, each witnessing a distinct
  claim:
  1. `u8 small = 250` — `let small: u8 = 250;` binds and prints.
     Witnesses (a) `u8` plugs into the lesson-019 `: TYPE` slot,
     and (b) `250` is in `u8`'s `0..=255` range.
  2. `i8 signed_small = -100` — `let signed_small: i8 = -100;`
     binds and prints. Witnesses (a) `i8` plugs into the slot,
     and (b) the leading `i` is what allows the negative literal
     `-100` (lesson 062 saw the unsigned-rejection contrast at
     `error[E0600]: cannot apply unary operator -`).
  3. `i64 big = 9000000000` — `let big: i64 = 9_000_000_000;`
     binds and prints. Witnesses (a) `i64` plugs into the slot,
     (b) the `_` digit-separator is accepted in a literal (cited
     only; the full literal-form table is queue item H), and
     (c) nine billion fits in `i64`'s range. The output line
     prints `9000000000` (no separator) because `{}` formatting
     does not insert thousands separators; the literal-side
     separator is purely a source-text affordance.
  4. `usize architecture = 42` — `let architecture: usize = 42;`
     binds and prints. Witnesses that `usize` plugs into the slot
     (lesson 077's typed-name install repeats here as a family
     member, not as the indexing type — today is the family-naming
     move, not an indexing move).
- The committed `.rs` file's source matches the *Try It* code
  block exactly. Only the working source is committed under
  `observations/`.
- Four typed names exercised: `u8`, `i8`, `i64`, `usize`. The
  remaining eight (`u16`, `i16`, `u32`, `i32`, `u64`, `i128`,
  `u128`, `isize`) are named in the lesson body but not
  individually probed; the Book's Table 3-1 is the warrant for
  their existence, and the four-corner sample is sufficient to
  witness the *naming convention* the lesson centers (signed
  `i`-prefix with negative literal; unsigned `u`-prefix without;
  one bit-width past 32; one architecture-dependent).

### Probe 2: contrast — out-of-range literal for `u8`

Same temp dir family, separate file `broken.rs`:

```text
--- cat broken.rs ---
fn main() {
    let too_big: u8 = 256;
    println!("{}", too_big);
}
--- rustc broken.rs ---
error: literal out of range for `u8`
 --> broken.rs:2:23
  |
2 |     let too_big: u8 = 256;
  |                       ^^^
  |
  = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default

error: aborting due to 1 previous error

exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error: literal out of range for `u8``. *Uncoded*
  — no `E####` (the `overflowing_literals` lint is not a coded
  error). Lesson 069's category map covers the uncoded `error:`
  shape.
- **Location**: `broken.rs:2:23` — line 2, column 23, the start
  of the literal `256`.
- **Source excerpt with caret**: `^^^` underlines `256`. Three
  characters; the caret falls on the literal, not on the type
  annotation.
- **First `note:` line**: `the literal `256` does not fit into
  the type `u8` whose range is `0..=255``. This is the
  load-bearing piece. rustc itself spells the range the lesson
  installed: `0..=255`. The diagnostic does double duty as the
  *source* for the lesson's `u8` range claim (in addition to the
  Book's lines 86-87): rustc's gloss is the operational
  restatement of the Book's `0 to 28 − 1 ... 0 to 255` formula.
- **Second `note:` line**: `#[deny(overflowing_literals)] on by
  default`. Lint name; deferred under *What To Ignore For Now*.
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's range
claim. Without it, "u8 holds 0..=255" would be a bare assertion
sourced only from the Book; the captured diagnostic shows rustc
itself enforcing the rule with the same numeric range in its
gloss.

Why `u8 = 256` and not the brief's alternative
`i32 = 9_000_000_000`: `u8 = 256` exhibits the *smallest* concrete
range — the one with the fewest values to reason about — and
rustc's gloss prints that range in full (`0..=255`). The `i32`
alternative would also fire `overflowing_literals` (Probe 3
below), but its gloss prints the much wider range
`-2147483648..=2147483647`, which is harder to read at a glance
and risks distracting from the rule. The smaller probe makes the
range visible end-to-end in the diagnostic itself.

### Probe 3: auxiliary — out-of-range literal for `i32`

Captured for evidence transparency only. **Not** referenced in
the lesson body. Documented to show the brief's alternative
contrast was considered and to corroborate that
`overflowing_literals` is the same lint family across signed
and unsigned variants:

```text
--- cat aux.rs ---
fn main() {
    let too_big: i32 = 9_000_000_000;
    println!("{}", too_big);
}
--- rustc aux.rs ---
error: literal out of range for `i32`
 --> aux.rs:2:24
  |
2 |     let too_big: i32 = 9_000_000_000;
  |                        ^^^^^^^^^^^^^
  |
  = note: the literal `9_000_000_000` does not fit into the type `i32` whose range is `-2147483648..=2147483647`
  = help: consider using the type `i64` instead
  = note: `#[deny(overflowing_literals)]` on by default

error: aborting due to 1 previous error

exit=1
```

Notes:

- Same uncoded-`error:` headline shape as Probe 2, with `i32`
  instead of `u32`. Same `overflowing_literals` lint trailer.
- The `note:` line gives `i32`'s range as
  `-2147483648..=2147483647`, which equals the lesson's
  `-(2^31)..=2^31 - 1` formula evaluated.
- The `help:` line `consider using the type `i64` instead` is
  rustc itself recommending the same `i64` upgrade the working
  probe demonstrates. Pedagogically reinforces the working
  probe's choice of `i64` for nine billion.
- This probe is *not* the lesson's centered contrast because
  Probe 2's smaller range is the cleaner demonstration. Probe 3
  is documented for honesty and to confirm the lint family is
  consistent across signed and unsigned variants.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the lesson's
range claim. Probe 3 is auxiliary; its transcript corroborates
that the `overflowing_literals` lint behaves consistently for
`i`-prefix variants too, but is not load-bearing for any
centered claim today.

The lesson does not run a probe for the unsigned-rejection of
negative literals (`let n: u8 = -1;`); lesson 062 is the
load-bearing source for that rule and Probe 2's E0600 transcript
covers it. The Check Yourself answer (b) cites lesson 062's
diagnostic verbatim.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency. The architecture-dependent
fact for `usize` is *not* operationally visible in Probe 1 (the
value `42` fits any `usize` width); it is named in the lesson
prose with lesson 077 as the load-bearing prior install.

Probe 2's headline (`error: literal out of range for `u8``), the
inline gloss
(`note: the literal `256` does not fit into the type `u8` whose
range is `0..=255``), and the lint trailer
(`#[deny(overflowing_literals)] on by default`) are deterministic
on this rustc release. The exact wording is rustc-version-specific;
the *shape* — uncoded `error:` with a "literal out of range for X"
headline plus a numeric-range gloss — is grounded in lesson 003's
diagnostic map and is stable.

Probe 3 is also rustc-version-specific in wording but stable in
shape on this release.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 080.

- **Lesson 003 (load-bearing for the diagnostic map)** — installs
  the four-part read of headline + `-->` + source excerpt with
  caret + optional `help:` / `note:` lines. Probe 2 is read with
  that map only; no new diagnostic vocabulary is installed today.
  The uncoded-`error:` shape was already covered by lesson 069.
- **Lesson 005 (load-bearing for `let name = value;`)** —
  installs the binding form. Today reuses it four times in the
  working probe with no extension.
- **Lesson 019 (load-bearing for the `: TYPE` slot)** — installs
  `let name: TYPE = value;` as a *type annotation* and the
  integer-literal default of `i32`. Today plugs four type names
  into the same slot (`u8`, `i8`, `i64`, `usize`) and reuses the
  default rule unchanged.
- **Lesson 062 (load-bearing for the unsigned-integer family)** —
  installs `u32` as the unsigned counterpart to `i32`. Today
  generalizes the `i`/`u` convention from one pair to the whole
  table. Lesson 062's *What To Ignore* explicitly named "`i8`,
  `u8`, `i16`, `u16`, `i64`, `u64`, `i128`, `u128`, `isize`,
  `usize`" as deferred; today closes the family-naming half of
  that line. Lesson 062's E0600 transcript for the
  unsigned-negative-literal case is the load-bearing source for
  Check Yourself answer (b); this lesson does not re-run that
  probe.
- **Lesson 077 (load-bearing for `usize` and the
  architecture-dependent row)** — installed `usize` as the
  third typed integer name and the architecture-dependent gloss
  ("64 bits on a 64-bit machine, 32 bits on a 32-bit machine").
  Today reuses both for the `size` row of the table and for the
  working probe's fourth binding without re-installing them.
  Lesson 077's *What To Ignore* explicitly named "the full
  integer family — Table 3-1's remaining variants. Queue item G"
  as deferred; today closes that line.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success. Used as the compile-and-run shape for
  all probes.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches.
- `004-statements-in-order` — the body of `fn main` is a sequence
  of `;`-terminated statements that run top to bottom. The
  working probe's eight statements all reuse this rule.
- `011-println-positional-args` — `println!("{}", expr)`. Reused
  as-is; today does not extend `println!`. The probe's four
  `println!` lines all use positional `{}` substitution.
- `033-f64-floats` — installed `f64` as a sibling typed name
  (different family). Mentioned in *What To Ignore For Now* only.
- `034-as-cast-i32-to-f64` — installed the `as` cast for one
  cross-family direction. Mentioned in *What To Ignore For Now*
  only; integer-to-integer `as` is deferred.
- `069-rustc-warnings`, `070-rustc-explain` — diagnostic-category
  infrastructure. Probe 2's uncoded `error:` shape is read with
  069's map; no `--explain` follow-up is present (uncoded errors
  do not carry the `--explain` trailer).
- `074-char-type`, `075-const-declaration`, `076-array-literal-and-type`,
  `078-array-out-of-bounds-panic`, `079-for-over-array` — most
  recent accepted lessons on the same host and toolchain.
  Mentioned only to confirm the host environment is unchanged.

No trait-related lesson is cited. The brief explicitly excluded
trait machinery and `Copy`.

## Book Ch1-3 closure-pass effect

This lesson **closes item G** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item G's listed prereqs were 019 (i32) and 062 (u32); both were
installed before this cycle, and lesson 077 (usize) had already
extended the typed-integer set to three names, with
architecture-dependence in tow. Today carries out exactly the
plan G describes: one centered move that names the family and the
size-and-sign convention, citing 019, 062, and 077 for the three
already-installed corners and reading the rest of Table 3-1 as a
remaining-row enumeration.

With the family installed, queue items **H** (integer literal
forms) and **I** (integer overflow at runtime) become directly
approachable: H needs the `: TYPE` slot for `u8`, `i64`, etc.,
which today provides for all twelve variants; I needs `u8` as a
small-range example for the wrap-vs-panic demonstration, which
today also provides. The remaining Ch1-3 closure queue items
(beyond G/H/I) are unaffected.
