# Evidence — 044-use-declaration

Audit appendix for `lessons/044-use-declaration.md`. Holds the
corpus-quote map, the toolchain string, the full working and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/044-use-declaration.rs`); the broken-contrast `.rs` is
  not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/reference/items/use-declarations.md`

The Reference page for `use` declarations. Primary corpus source for
this lesson. Three load-bearing spans.

Line 10 (the syntax line):

> [UseDeclaration](use-declarations.md#railroad-UseDeclaration) → use
> [UseTree](use-declarations.md#grammar-UseTree) ;

This is the formal grammar statement for the smallest `use` form: the
keyword `use`, then a *UseTree* (which in its simplest form is just a
SimplePath — see line 15: "[SimplePath](../paths.md#grammar-SimplePath)
( as ( [IDENTIFIER] | _ ) )?", with the `as`-clause optional and
omitted by today's lesson), then a trailing `;`. The lesson's
`use std::cmp::min;` matches this with `std::cmp::min` as the
SimplePath. The lesson does not surface the *UseTree* nonterminal; it
only describes the spoken-English shape "the keyword `use`, then a
path identical in form to lesson 043's call paths, then a trailing
`;`."

Line 19 (the *load-bearing* corpus statement of the lesson's main
concept):

> A *use declaration* creates one or more local name bindings
> synonymous with some other [path](../paths.md). Usually a `use`
> declaration is used to shorten the path required to refer to a
> module item. These declarations may appear in [modules](modules.md)
> and [blocks](../expressions/block-expr.md), usually at the top. A
> `use` declaration is also sometimes called an *import*, or, if it is
> public, a *re-export*.

This is the canonical Reference statement that a `use` declaration
*creates a local binding synonymous with* the named path — i.e. the
short alias resolves to the same thing the full path reached. The
lesson's body claim "the bare call `min(3, 5)` resolves to the same
`std::cmp::min` function the full path reaches" is this Reference
sentence in plain prose. The Reference's "synonymous" language also
grounds the lesson's "`use` adds the short alias, it does not replace
the long form" — both names resolve to the same target, so writing
either still works.

The Reference's "usually at the top" phrasing licenses the lesson's
"It sits at the top of the file, outside `fn main`." The lesson does
not install the inside-blocks/inside-modules surface — that is
explicitly deferred to *What To Ignore For Now* under "*`use` inside
functions or modules*."

The Reference's "*re-export*" clause is `pub use` — explicitly deferred
in *What To Ignore For Now*.

Lines 21-23 (the *forms* introductory line):

> Use declarations support a number of convenient shortcuts:

The Reference then enumerates five shortcuts: brace-grouping (line 27),
`self`-import (line 31), `as`-rename (line 35), glob (line 39), and
nested grouping (line 43). The lesson installs *none* of these. Each
maps onto a deferred item in *What To Ignore For Now*:

- "Simultaneously binding a list of paths with a common prefix, using
  the brace syntax `use a::b::{c, d, e::f, g::h::i};`" — deferred as
  "*Nested-group imports `use std::{io, cmp::min};`*."
- "Rebinding the target name as a new local name, using the syntax
  `use p::q::r as x;`" — deferred as "*`use ... as alias;`*."
- "Binding all paths matching a given prefix, using the asterisk
  wildcard syntax `use a::b::*;`" — deferred as
  "*Glob imports `use std::cmp::*;`*."

Today's lesson installs only the bare base form
`use SimplePath;` — the simplest case the grammar permits.

### `output/docs/rust/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.md`

The Book chapter on `use`. Cited for the audience-level prose statement
of *what `use` does* and the corpus precedent for its top-of-file
position. Two load-bearing spans.

Lines 4-9 (the *Why `use`* introduction):

> Having to write out the paths to call functions can feel inconvenient
> and repetitive. ... Fortunately, there's a way to simplify this
> process: We can create a shortcut to a path with the `use` keyword
> once and then use the shorter name everywhere else in the scope.

Plain-English statement of the lesson's motivation — "Lesson 043
reached `std::cmp::min` by writing the full path at every call site"
maps directly onto the Book's "Having to write out the paths to call
functions can feel inconvenient and repetitive." The lesson's "shortcut
to a path" / "shorter name" framing reuses the Book's exact phrasing.

Lines 34-37 (the *symbolic-link* analogy):

> Adding `use` and a path in a scope is similar to creating a symbolic
> link in the filesystem. By adding `use crate::front_of_house::hosting`
> in the crate root, `hosting` is now a valid name in that scope, just
> as though the `hosting` module had been defined in the crate root.

The Book's "now a valid name in that scope" matches the Reference's
"creates one or more local name bindings synonymous with some other
path." The lesson uses neither phrasing literally — the audience-level
"brings the name `min` into the file's scope" combines both. The Book's
symbolic-link analogy is illustrative; the lesson does not surface it
because filesystem-link mechanics would sidetrack into a non-Rust
analogy not yet validated for this audience.

Calibration: the Book's example is `use crate::front_of_house::hosting;`
in a user-defined module tree, with `crate::` as the path root. The
lesson uses `use std::cmp::min;` with `std::` as the path root, because
(a) lesson 043 already installed the `std::cmp::` consumption surface,
and (b) the `crate::` path root is explicitly deferred under "*`use
crate::`, `use self::`, `use super::`*." The Reference's
`use std::collections::hash_map::{self, HashMap};` example (line 48 of
use-declarations.md) and the Book's later
`use std::collections::HashMap;` (line 141 of
ch07-04-bringing-paths-into-scope-with-the-use-keyword.md) both use the
absolute `std::` root, matching the lesson's shape.

### `output/docs/rust/std/cmp/fn.min.md`

The std-library page for `std::cmp::min`. Already cited in lesson 043;
reused here for the working probe's `min(3, 5)` call.

Lines 6-11 (the canonical signature):

> ```
> pub fn min<T>(v1: T, v2: T) -> T
>
> where
>     T: Ord,
> ```

Same audience-level treatment as in lesson 043: a function that takes
two values of the same kind and returns the smaller one. With both
arguments `i32` literals (`3` and `5`), `T` resolves to `i32` and the
return type is `i32`, which licenses the lesson's
`let full: i32 = std::cmp::min(3, 5);` and `let short: i32 = min(3, 5);`
annotations.

Lines 23-28 (the canonical example):

> ```
> use std::cmp;
>
> assert_eq!(cmp::min(1, 2), 1);
> assert_eq!(cmp::min(2, 2), 2);
> ```

Direct corpus precedent for using a `use` line to shorten the path.
The std-page example imports the *parent module* `std::cmp` (so the
caller writes `cmp::min`); today's lesson imports the *function*
`std::cmp::min` directly (so the caller writes `min`). Both are valid
`use` shapes per the Reference's UseTree grammar; the Book chapter
7-04 calls the parent-module form "idiomatic" for functions (lines
125-131) but the lesson uses the direct-function form because (a) it
is the form rustc's `help:` block in the broken-contrast probe of
*lesson 043* literally suggested ("`use std::cmp::min;`"), and (b) it
is the smallest pedagogical step from lesson 043's full path —
chopping off everything before the final segment, and writing the
full path once at the top in `use` shape.

### `output/docs/rust/std/cmp/fn.max.md`

The std-library page for `std::cmp::max`. Already cited in lesson 043;
reused here for the broken-contrast probe's `max(3, 5)` call. The
function exists at path `std::cmp::max` and is reachable via either
the full path or a separate `use std::cmp::max;` line; it is *not*
reached by `use std::cmp::min;` because that `use` line names only
the final segment `min`, per the Reference's UseDeclaration syntax
(one SimplePath per UseTree base form).

The page's canonical signature (lines 6-11) and `Compares and returns
the maximum of two values.` description (lines 15-17) are not cited
again here — lesson 043 already covered them.

### `output/docs/rust/error_codes/E0425.md`

The error-code explainer for E0425, "an unresolved name was used."
Already cited in lessons 005, 008, 040, 042, and 043. Reused here for
the broken-contrast probe. Two load-bearing spans.

Line 4 (the corpus statement):

> An unresolved name was used.

The lesson does not re-explain E0425; it cites lessons 005, 008, 040,
042, and 043 for the E-code's prior installations.

Lines 63-72 (the corpus's `use`-suggesting fix):

> If the item is not defined in the current module, it must be imported
> using a `use` statement, like so:
>
> ```rust
> mod foo { pub fn bar() {} }
> fn main() {
> use foo::bar;
> bar();
> }
> ```

Direct corpus precedent for *fixing* an E0425 by adding a `use`
statement that brings the missing name into scope. Today's broken
probe fires E0425 on `max(3, 5)` and rustc's `help:` block suggests
exactly this fix — a separate `use std::cmp::max;` line. The corpus
example demonstrates the same use-line-resolves-the-unresolved-name
pattern in miniature with a user-defined module. The corpus line "it
must be imported" supports the lesson's mental-model framing that the
`use` line is what makes a non-local name reachable by its bare final
segment.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/044-use-declaration.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
demo.rs
--- cat demo.rs ---
use std::cmp::min;

fn main() {
    let full: i32 = std::cmp::min(3, 5);
    let short: i32 = min(3, 5);
    println!("full = {full}, short = {short}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
full = 3, short = 3
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line: `full = 3, short = 3`. Both values
  are `3`. This is the *equivalence* claim of the lesson: with the
  `use std::cmp::min;` line at the top, the bare call `min(3, 5)` and
  the full-path call `std::cmp::min(3, 5)` both resolve to the same
  function and produce the same value. This is the working-side
  corroboration of the Reference's "creates one or more local name
  bindings synonymous with some other path" statement.
- This probe also corroborates the lesson's "the full path still works
  after the `use`" claim. Adding a `use` line did not break the
  full-path call site on line 4 — the program compiles and both
  bindings produce `3`. The Reference's "synonymous" language is
  equivalence in both directions.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: working-probe shape with the `use std::cmp::min;` kept at the
top, but with `let larger: i32 = max(3, 5);` added — calling a
*sibling* function from the same module by a name that was *not* part
of the `use` line. Not committed; the transcript below is the
artifact. Captured 2026-05-07 in a fresh `mktemp -d` (filename
`broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
use std::cmp::min;

fn main() {
    let smaller: i32 = min(3, 5);
    let larger: i32 = max(3, 5);
    println!("smaller = {smaller}, larger = {larger}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0425]: cannot find function `max` in this scope
 --> broken.rs:5:23
  |
5 |     let larger: i32 = max(3, 5);
  |                       ^^^ not found in this scope
  |
help: consider importing this function
  |
1 + use std::cmp::max;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0425]: cannot find function ` then a
  backtick-quoted `max`, then ` in this scope`. Same E-code lessons
  005, 008, 040, 042, and 043 installed (005 for missing values, 008
  for missing free functions, 040 for the dot-form broken contrast,
  042 for the qualified-path missing-qualifier broken contrast, 043
  for the nested-path missing-qualifier broken contrast). The
  headline word "function" matches lesson 008's shape — rustc parsed
  `max(3, 5)` as a free-function call, found no free function named
  `max` in scope, and emitted the missing-name diagnostic. Crucially,
  the headline names *`max`*, not `min` — the `use std::cmp::min;`
  line at the top resolved `min` cleanly (line 4 compiled fine), but
  `max` was *not* in scope because the `use` line did not name it.
- The diagnostic has the four lesson-003 parts: headline + `-->`
  location (`broken.rs:5:23`) + source excerpt with `^^^` caret under
  `max` + `help:` block. The `--explain E0425` trailer is also
  present, consistent with lesson 003's rule that headlines with an
  `[E####]` code emit the trailer.
- The `help:` block reads literally `help: consider importing this
  function`, followed by a source-diff suggestion `1 + use std::cmp::max;`
  inserting a *new* line at the top of the file (the `+` prefix is
  rustc's insertion marker; the `1` is the line number to insert at).
  This is the same shape rustc emitted in lesson 043's broken-contrast
  probe, but for `max` instead of `min` — confirming that rustc treats
  each missing item independently. The suggested `use std::cmp::max;`
  is a *separate* `use` line, not an extension of the existing
  `use std::cmp::min;` — demonstrating that one `use` per item is
  rustc's preferred fix in this base form. (The Reference does have a
  brace-grouping form `use std::cmp::{min, max};` that would handle
  both in a single line; rustc does not auto-suggest that form here.
  Brace grouping is explicitly deferred under "*Nested-group
  imports*.")
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`, no `broken` binary.
- Most load-bearing observation: line 4 of the broken probe
  (`let smaller: i32 = min(3, 5);`) does *not* fire any error. With
  `use std::cmp::min;` at the top, `min(3, 5)` resolves cleanly and
  rustc accepts that line. The error fires *only* on line 5 where
  `max` is unresolved. This is the empirical proof of the lesson's
  claim that "a `use` line brings in *only the items it names*" —
  importing `min` did not also import its module-sibling `max`. If
  `use std::cmp::min;` had brought in the whole `cmp` module or all
  its public items, line 5's `max(3, 5)` would have compiled too; it
  did not.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("a `use` line brings in *only the items it names*").
The captured `error[E0425]` on line 5 *plus the absence of any error
on line 4* together ground the lesson's claim. The corpus-level
grounding for this contrast is structural — the Reference's
UseDeclaration grammar (line 10 of use-declarations.md) takes a single
SimplePath in its base form, and the Reference's "creates one or more
local name bindings synonymous with some other path" (line 19)
specifies that the bindings are for the path's leaf, not for siblings
of the path's leaf. The probe is the live transcript that ties those
corpus statements to this specific `min`-imported, `max`-not-imported
example.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 044. Older supporting lessons are mentioned
above by number only.

- **Lesson 043 (load-bearing)** — installed (a) the full nested-path
  call form `std::cmp::min(3, 5)`, (b) the corpus identification of
  `std` as the standard library's root module and `cmp` as one of its
  submodules, (c) `std::cmp::min` as a free function returning the
  smaller of two same-kind values, and (d) E0425 as the failure mode
  for a missing path qualifier. Lesson 044 is built directly on this:
  the `use std::cmp::min;` line names the *exact* path lesson 043
  called by, and the working probe's line 4 reuses the lesson-043
  full-path call unchanged. Lesson 043 also captured the rustc
  `help:` block "`help: consider importing this function`" + source-
  diff `use std::cmp::min;` — that captured help text *literally
  named the future move this lesson installs.* Today's broken probe
  emits the same `help:` shape with `max` instead of `min`,
  confirming the diagnostic mechanism carries over unchanged.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `help:` lines.
  Lesson 044's broken-contrast walk uses that map without re-teaching
  it.
- **Lessons 005, 019** — `let name: TYPE = value;` annotated binding.
  Used unchanged on lines 4 and 5 of the working probe.
- **Lesson 002** — `fn main` is the entry point. The lesson's "It sits
  at the top of the file, outside `fn main`" rests on the established
  fact that `fn main` is a delimited region — the `use` line is
  outside that region, at file top level.
- **Lesson 001** — `rustc file.rs` then `./name`; silent on success.
  Used unchanged.

## Older supporting lessons

Lessons 005, 008, 040, 042, 043 (E0425 family connection — the
broken-contrast probe fires E0425, the same E-code first installed in
lesson 005 for missing values and reused in 008, 040, 042, and 043;
not re-stated here beyond the family connection).

Lesson 020 (free-function-call shape `name(args)` — the bare call
`min(3, 5)` and `max(3, 5)` after the `use` line take the lesson-008
free-function call shape, with the `use` line being what licensees
the bare names; not re-stated here, the call shape is reused
unchanged).

Lesson 036 (multi-argument calls — `min(3, 5)` and `max(3, 5)` are
two-argument calls; lesson 036 installed positional argument matching
for multi-parameter functions; not re-stated here, the two-argument
call shape is used unchanged from lesson 043).
