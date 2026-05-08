---
id: 031-match-integer-and-wildcard
move: "extend lesson 030's `match` to an integer scrutinee with `match n { 1 => 10, 2 => 20, 3 => 30, _ => 99, }`, using the `_` wildcard pattern as the final arm to catch every integer value not named by an earlier arm"
main_concept: "lesson 030's `match` machine carries over unchanged to integer scrutinees: arms are still `pattern => arm_expression,`, the matching arm's expression is the value of the whole `match`, all arms must share a type, and exhaustiveness is still enforced by **E0004**. Two new pattern forms install together: (1) any *integer literal* (`1`, `2`, `3`, ...) is a pattern that matches that exact integer value — the same general rule lesson 030 used for `true`/`false`; (2) the `_` *wildcard pattern* matches any value of any type. The pair is inseparable: integer types like `i32` have billions of values, so listing every possible integer literal is impossible, and rustc demands exhaustiveness — so a `match` on an integer scrutinee needs `_` (or another catch-all) as a final arm."
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 019-type-annotation-i32
  - 030-match-on-bool
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "range patterns `1..=5 => arm`" moves
  - future "or-patterns `p1 | p2 => arm`" moves
  - future "match guards `pattern if condition => arm`" moves
  - future "match arms with multi-statement blocks `pattern => { ...; tail }`" moves
  - future "match against enum variants" moves
  - future "negative integer literals as patterns (unary minus on a literal)" moves
  - future "named binding patterns `other => move(other)`" moves
  - future "the `_` binding-name convention vs the `_` wildcard pattern" moves
  - future "if let / while let" moves
sources:
  - output/docs/rust/reference/patterns.md
  - output/docs/rust/error_codes/E0004.md
  - output/docs/rust/book/ch06-02-match.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/031-match-integer-and-wildcard.rs
  - broken-contrast probe transcript inline in lesson `## Evidence` (not committed)
status: accepted
---

# `match` on an integer with `_` as a catch-all arm

## The Move

In lesson 030 you wrote `match` on a `bool` with arms `true =>` and
`false =>`. Now write `match` on an integer. Pick the cases you care
about with integer-literal patterns, then add one final arm whose
pattern is a single underscore `_`. The `_` arm catches every integer
value the earlier arms did not name. Bind the result with `let`:

```rust
let label: i32 = match n {
    1 => 10,
    2 => 20,
    3 => 30,
    _ => 99,
};
```

If `n` is `1`, `2`, or `3`, the matching arm produces `10`, `20`, or
`30`. For any other integer — `0`, `-7`, `42`, anything — the `_` arm
matches and `label` becomes `99`.

## Mental Model Delta

- Before: "I know `match` on `bool` (lesson 030). It needs both `true`
  and `false` as arms because rustc demands exhaustiveness."
- After: "Lesson 030's whole `match` machine works for integers too.
  Two new pieces. (1) Any integer literal (`1`, `2`, `3`, ...) is a
  pattern, the same way `true` and `false` were. (2) `_` is a special
  pattern that matches *any* value. I need it as the final arm
  whenever I `match` on an integer, because integer types have
  billions of values and rustc still demands exhaustiveness — listing
  every integer is impossible, so `_` plays the role `else` plays in
  an `if`/`else if` chain."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; `fn main`'s body
    runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt with caret + optional `note:` /
    `help:` sub-lines. The broken-contrast walk decodes a richer
    E0004 with all four of those parts.
  - Lesson 005: `let name: TYPE = value;`. Reused as the slot the
    `match`'s value lands in.
  - Lesson 019: `name: TYPE`; `let n: i32 = 3;` and
    `let label: i32 = match ...;`.
  - Lesson 030 (load-bearing): installed `match`, *scrutinee*, *arms*,
    `pattern => arm_expression`, separator `,`, the matching arm's
    expression as the whole `match`'s value, the
    arms-must-share-type rule, and exhaustiveness with E0004. This
    lesson reuses every one of those without re-deriving them, and
    extends the scrutinee from `bool` to `i32`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let n: i32 = 3;
    let label: i32 = match n {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => 99,
    };
    println!("label = {label}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
label = 30
```

Walk:

- `n` is `3` (lessons 005, 019).
- `match n { ... }` is a `match` expression (lesson 030). The
  scrutinee is `n`. The arms are `1 => 10`, `2 => 20`, `3 => 30`, and
  `_ => 99`.
- rustc compares `n` to each pattern in source order. The patterns
  `1`, `2`, `3` are *integer-literal patterns* — each matches that
  exact integer value. `_` is the *wildcard pattern* and matches any
  value at all.
- `n` is `3`. The first two arm patterns (`1`, `2`) do not match. The
  third arm's pattern (`3`) matches; its expression `30` becomes the
  value of the whole `match` (lesson 030's rule). The remaining
  `_ => 99` arm is not consulted.
- `let label: i32 = ...;` binds `label` to `30`. `println!` prints
  `label = 30`.

If `n` had been any other integer — say `5` or `-7` — the first three
arm patterns would not match. The `_` arm's pattern always matches, so
its expression `99` would become the whole `match`'s value, and
`label` would be `99`. Pattern order matters: `_` matches everything,
so it must be the *last* arm. Earlier arms match more specifically and
get first chance.

Now the contrast. Save `broken.rs` — same as `demo.rs` but with the
`_ => 99,` arm removed:

```rust
fn main() {
    let n: i32 = 3;
    let label: i32 = match n {
        1 => 10,
        2 => 20,
        3 => 30,
    };
    println!("label = {label}");
}
```

Compile it. Full transcript in `## Evidence`. Reading it with lesson
003's order:

- *Headline*:
  `error[E0004]: non-exhaustive patterns: \`i32::MIN..=0_i32\` and \`4_i32..=i32::MAX\` not covered`.
  Same E-code lesson 030 introduced — but now rustc names *two*
  missing patterns instead of one, and they are written in a
  different syntax: `i32::MIN..=0_i32` and `4_i32..=i32::MAX`. Read
  these as ranges: the first covers every integer from `i32`'s
  smallest value through `0`; the second covers every integer from
  `4` through `i32`'s largest value. Together with the listed `1`,
  `2`, `3`, those two ranges cover every possible `i32`. rustc
  describes the gap in `i32`'s billions of values using two range
  expressions because that is the only finite way to describe it.
- *`-->` location*: `broken.rs:3:28` — the scrutinee `n`. Same as
  lesson 030: rustc points at the scrutinee, because exhaustiveness
  is checked against the scrutinee's type.
- *Source excerpt*: `^` underlines `n`, sub-line repeats the missing
  pattern names.
- *`note:` line*: `the matched value is of type \`i32\``. rustc names
  the type whose values must be covered.
- *`help:` block*: lists *three* possible fixes — "a wildcard
  pattern, a match arm with multiple or-patterns as shown, or
  multiple match arms" — and the source-diff suggests one literal
  fix using or-patterns:
  `i32::MIN..=0_i32 | 4_i32..=i32::MAX => todo!(),`. The simplest
  remedy, and the one `demo.rs` uses, is the wildcard: `_ => 99,`.
  `todo!()` is rustc's placeholder macro (carried forward from
  lesson 030); not real advice.

The diagnostic implicitly references *range patterns* (`a..=b`) and
*or-patterns* (`p | q`). Both are real Rust pattern forms and both
are deferred to future moves (lesson 030's unlocks list named them).
For now, the wildcard `_` is the simplest fix.

## What Changed

- You can `match` on an integer. The new lesson is what shape the
  arms take: pick interesting cases with integer-literal patterns,
  then end with a `_` arm to catch everything else. Lesson 030's
  rules — matching arm wins, all arms share a type, exhaustiveness
  enforced — apply unchanged.
- You have two new patterns: an *integer literal* like `1` matches
  exactly the integer `1`. An *underscore* `_` matches any value of
  any type. The Reference says `_` matches any value and is "always
  irrefutable" — gloss that as "no matter what the scrutinee is, this
  pattern always matches".
- `_` must come last. Patterns are tried in source order; once `_`
  matches, every later arm would be unreachable. (rustc warns about
  unreachable arms in general, though that warning is deferred.)
- E0004 still fires if you forget the catch-all. The headline and
  `help:` block are richer than lesson 030's `bool` case: instead of
  one missing pattern (`false`), rustc names *two ranges* covering
  every `i32` value not in your literals, and offers three suggested
  fixes including the `_` wildcard.
- A `_` arm is the integer counterpart of an unconditional `else` in
  an `if` / `else if` chain (lesson 016): the final, always-taken
  branch when nothing earlier fit.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let n: i32 = 100;
    let kind: i32 = match n {
        0 => 0,
        1 => 1,
        _ => -1,
    };
    println!("kind = {kind}");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) If you change `let n: i32 = 100;` to `let n: i32 = 1;` and
recompile, what does it print?

(c) If you remove the `_ => -1,` arm and recompile, what E-code does
rustc emit? Will the headline name `false` as missing, or something
else?

(Answers: (a) Yes; prints `kind = -1`. The patterns `0` and `1` do not
match `100`, so the `_` arm matches and the `match` produces `-1`.
(b) Prints `kind = 1`. The pattern `1` matches, and that arm wins
before `_` is tried. (c) `error[E0004]: non-exhaustive patterns`. It
will *not* name `false` — that was lesson 030's `bool` case. For an
`i32` scrutinee, rustc names the missing patterns as ranges, e.g.
`i32::MIN..=-1_i32` and `2_i32..=i32::MAX` (the exact range bounds
depend on which literals you listed).)

## What To Ignore For Now

- *Range patterns* like `1..=5 => arm` or `i32::MIN..=0_i32 => arm`.
  These appear in the broken-contrast diagnostic. Future move (named
  in lesson 030's unlocks).
- *Or-patterns* like `1 | 2 | 3 => arm`. Also in the diagnostic.
  Future move (also from 030's unlocks).
- *Negative integer literals as patterns* like `-5 => arm`. The
  Reference notes that negative numbers are not technically literals
  but unary-minus on a literal, and special-cases them; the
  mechanism is real but the probe uses only positive integer
  literals (`1`, `2`, `3`). Future move.
- *Named bindings as catch-all* like `other => use(other)` (the
  Book's first form before the `_` rewrite). The bare-name pattern
  *binds* the matched value to `other` so the arm can use it; `_`
  matches without binding. Distinct mechanism, deferred.
- *`_` as a binding-name convention* like `let _name = 42;` from
  lesson 029, or the bare `let _ = expr;` placeholder. Same character
  `_`, but a different role: that one is a binding name that tells
  rustc "do not warn me about an unused name". The `_` here is a
  *pattern* that matches any value. Both are real; do not conflate
  them.
- *The "irrefutable" / "refutable" pattern terminology*. The
  Reference uses "irrefutable" for patterns like `_` that always
  match. Glossed once as "always matches"; not installed.
- *Match guards*, *`if let` / `while let`*, *match against enum
  variants*, *bindings inside patterns* (`Some(n) => n + 1`),
  *reference patterns* (`&n => ...`, `ref n => ...`). All deferred.
- *`todo!()`* — rustc's placeholder macro in `help:` blocks.
  Carried-forward gloss from lesson 030; not installed.
- All previously deferred items.

## Evidence

### Sources

Three corpus sources support the lesson's substantive claims.

- `output/docs/rust/reference/patterns.md`. Two subsections are
  load-bearing.

  Line 168 (Literal Patterns), already cited in lesson 030, grounds
  integer-literal patterns the same way it grounded
  `true`/`false`-as-patterns there:

  > *Literal patterns* match exactly the same value as what is created
  > by the literal.

  Line 168 also notes:

  > Since negative numbers are not literals, literals in patterns may
  > be prefixed by an optional minus sign, which acts like the
  > negation operator.

  Glossed in `## What To Ignore For Now`; the probe uses only
  positive literals.

  Lines 462 and 505 (Wildcard pattern subsection) are the canonical
  Reference statements of `_`:

  > The *wildcard pattern* (an underscore symbol) matches any value.
  > It is used to ignore values when they don't matter.

  > The wildcard pattern is always irrefutable.

  "Irrefutable" is glossed in plain English ("always matches, no
  matter what value the scrutinee has"); the formal term is not
  installed.

- `output/docs/rust/error_codes/E0004.md`, lines 27-31, names the
  wildcard as a remedy explicitly:

  > If you encounter this error you must alter your patterns so that
  > every possible value of the input type is matched. ... the
  > underscore `_` wildcard pattern can be added after all other
  > patterns to match "anything else".

  The same passage was cited in lesson 030 to gloss the wildcard;
  this lesson promotes that gloss to the installed move.

- `output/docs/rust/book/ch06-02-match.md`, lines 343-405, the
  *Catch-All Patterns and the `_` Placeholder* subsection. The Book
  introduces the wildcard against an integer scrutinee in almost
  exactly this lesson's shape:

  > ```rust
  > let dice_roll = 9;
  > match dice_roll {
  >     3 => add_fancy_hat(),
  >     7 => remove_fancy_hat(),
  >     _ => reroll(),
  > }
  > ```

  And lines 376-381 state the order rule:

  > the last pattern will match all values not specifically listed.
  > This catch-all pattern meets the requirement that `match` must be
  > exhaustive. Note that we have to put the catch-all arm last
  > because the patterns are evaluated in order. If we had put the
  > catch-all arm earlier, the other arms would never run, so Rust
  > will warn us if we add arms after a catch-all!

  This grounds the "must come last" rule and the link to
  exhaustiveness. The Book's example arms call functions that return
  `()`; this lesson's arms produce `i32` values bound by `let`. The
  shape is identical; the surface is the value-producing one
  installed by lesson 030.

Calibration:
- The Reference Wildcard subsection's example uses `_` inside tuple
  patterns (`let (a, _) = (10, x);`), function parameters, and struct
  patterns — all forms not yet installed in this run. The
  load-bearing fact for this lesson is the canonical sentence
  "matches any value", not the example surfaces.
- The Reference uses the term "irrefutable" for `_`. Glossed without
  installing the formal term.
- The Book chapter introduces an alternative catch-all using a bare
  name (`other => move_player(other)`) before the `_` form. The bare
  name *binds*; `_` does not. Mentioned in `## What To Ignore For
  Now`; not installed.
- rustc's E0004 diagnostic for this lesson's broken-contrast probe
  references *range patterns* (`i32::MIN..=0_i32`) and *or-patterns*
  (`a | b`) in its `help:` block. Both are real Rust pattern forms,
  both are deferred to future moves (named in lesson 030's unlocks).

### Probes

Two probes captured on rustc 1.95.0 (59807616e 2026-04-14) on Darwin
x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/031-match-integer-and-wildcard.rs`.
The broken-contrast probe is *not* committed under `observations/`;
its transcript is reproduced verbatim below.

Both probes were run in temp directories created with `mktemp -d`
and removed at the end.

#### Working probe

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let n: i32 = 3;
    let label: i32 = match n {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => 99,
    };
    println!("label = {label}");
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
label = 30
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 silently. No warnings.
- The output `label = 30` is the load-bearing observation: `n` was
  `3`, so the third arm's pattern (`3`) matched, the third arm's
  expression (`30`) was evaluated, and the whole `match` produced
  `30`, which `let label: i32 = ...;` bound to `label`.
- The `_ => 99,` arm is *required for compilation* (without it, see
  the broken-contrast probe), but not consulted here because an
  earlier arm matched.
- The annotation `: i32` pins the `match`'s type. Each arm's
  expression is an integer literal whose default type is `i32`,
  consistent with the binding annotation.
- The trailing `,` after `_ => 99,` is allowed (lesson 030's grammar
  note: the final arm's `,` is optional).

#### Broken-contrast probe

The probe `broken.rs` exists for the broken-contrast walk; the
transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let n: i32 = 3;
    let label: i32 = match n {
        1 => 10,
        2 => 20,
        3 => 30,
    };
    println!("label = {label}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0004]: non-exhaustive patterns: `i32::MIN..=0_i32` and `4_i32..=i32::MAX` not covered
 --> broken.rs:3:28
  |
3 |     let label: i32 = match n {
  |                            ^ patterns `i32::MIN..=0_i32` and `4_i32..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
  |
6 ~         3 => 30,
7 ~         i32::MIN..=0_i32 | 4_i32..=i32::MAX => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline is the same E-code lesson 030 introduced (E0004), but
  the missing-patterns clause is richer:
  `\`i32::MIN..=0_i32\` and \`4_i32..=i32::MAX\` not covered` — two
  missing patterns instead of `bool`'s one, and written in range
  syntax (`a..=b`) because rustc cannot list every uncovered integer
  literally.
- The `--> broken.rs:3:28` location points at the scrutinee `n`,
  consistent with lesson 030: exhaustiveness is checked against the
  scrutinee's type, so the location is the scrutinee.
- The `note: the matched value is of type \`i32\`` line names the
  type whose values must be covered — `i32` here, where lesson 030
  had `bool`.
- The `help:` line offers *three* fix shapes — "a wildcard pattern,
  a match arm with multiple or-patterns as shown, or multiple match
  arms" — where lesson 030's `help:` line offered two ("a wildcard
  pattern or an explicit pattern as shown"). The source-diff
  illustrates the or-pattern form
  (`i32::MIN..=0_i32 | 4_i32..=i32::MAX => todo!(),`). The simplest
  fix and the one `demo.rs` uses is the first shape: `_ => 99,`.
- `todo!()` is rustc's placeholder macro (carried-forward gloss from
  lesson 030).
- Exit code: 1. No executable produced.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline + `-->` location + source excerpt with caret +
  optional `note:` / `help:` sub-lines. The broken-contrast walk
  decodes a richer E0004 with all four parts.
- `005-let-binding` (accepted) — `let name: TYPE = value;`. Reused
  as the slot the `match`'s value lands in.
- `019-type-annotation-i32` (accepted) — `name: TYPE`; used as
  `let n: i32 = 3;` and `let label: i32 = match ...;`.
- `030-match-on-bool` (accepted, load-bearing) — installed the whole
  `match` form: scrutinee, arms, `pattern => arm_expression`,
  separator `,`, the matching arm's expression as the whole
  `match`'s value, the arms-must-share-type rule, exhaustiveness,
  and E0004. This lesson reuses every one of those without
  re-deriving them.
