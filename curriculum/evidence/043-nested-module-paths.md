# Evidence — 043-nested-module-paths

Audit appendix for `lessons/043-nested-module-paths.md`. Holds the
corpus-quote map, the toolchain string, the full working and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/043-nested-module-paths.rs`); the broken-contrast `.rs`
  is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/std/cmp/fn.min.md`

The std-library page for `std::cmp::min`. Three load-bearing spans.

Lines 6-11 (the canonical signature):

> ```
> pub fn min<T>(v1: T, v2: T) -> T
>
> where
>     T: Ord,
> ```

This is the corpus signature for `min`. Two parameters of the same
generic type `T` and a return value of the same type. The `where T: Ord`
trait bound is what makes `min` work specifically on values that have a
notion of ordering. The lesson does not surface the generic `T` or the
`Ord` bound — it presents `min` to the audience as "a function that
takes two values of the same kind and returns the smaller one," which
is the trait-bound-elided audience-level statement of this signature.
Concretely, when the arguments are `i32` literals (`3` and `5`), `T`
resolves to `i32` and the return type is `i32`, which is what licenses
the lesson's `let smaller: i32 = std::cmp::min(3, 5);` annotation.

Lines 15-17 (the description):

> Compares and returns the minimum of two values.
>
> Returns the first argument if the comparison determines them to be equal.

This is the corpus statement of what `min` does. The lesson body's
"returns the smaller one" rephrases the first sentence; the equal-case
detail is not surfaced (the lesson uses unequal arguments `3` and `5`).

Lines 23-28 (the example):

> ```
> use std::cmp;
>
> assert_eq!(cmp::min(1, 2), 1);
> assert_eq!(cmp::min(2, 2), 2);
> ```

The std-page example uses `use std::cmp;` to bring the `cmp` module
into scope and then writes `cmp::min(1, 2)`. The lesson explicitly
defers `use` and uses the full-path form `std::cmp::min(1, 2)`-shape
instead. The example's `cmp::min(1, 2) == 1` matches the lesson's
`std::cmp::min(3, 5) == 3` pattern (smaller of two unequal args).

### `output/docs/rust/std/cmp/fn.max.md`

The std-library page for `std::cmp::max`. Sibling page to `fn.min.md`,
cited for the working probe's second function call.

Lines 6-11 (the canonical signature):

> ```
> pub fn max<T>(v1: T, v2: T) -> T
>
> where
>     T: Ord,
> ```

Same shape as `min`, same `Ord` bound. Same audience-level treatment in
the lesson: a function that takes two values of the same kind and
returns the larger one.

Lines 15-17 (the description):

> Compares and returns the maximum of two values.
>
> Returns the second argument if the comparison determines them to be equal.

Corpus statement of what `max` does. Lesson body's "returns the larger
one" rephrases the first sentence.

The two pages together carry the *path-as-namespace* corpus point.
Both functions live at paths sharing the prefix `std::cmp::` and differ
only in the final segment. The probe demonstrates this empirically by
calling both with the same two arguments and printing both results.

### `output/docs/rust/std/index.md`

The crate root page for `std`. Two load-bearing spans.

Lines 17-19 (`std` as the standard library's path-accessible root):

> `std` is available to all Rust crates by default. Therefore, the
> standard library can be accessed in [`use`](../book/ch07-02-defining-modules-to-control-scope-and-privacy.md) statements through the path
> `std`, as in [`use std::env`](https://doc.rust-lang.org/stable/std/env/index.html).

This is the corpus statement that `std` is the path-segment name for
the standard library, available to all Rust crates by default. The
lesson's "`std` is the standard library's root module" rephrases the
"standard library can be accessed ... through the path `std`" claim.

Lines 54-58 (the standard library is divided into modules, including
`std::cmp`):

> First of all, The Rust Standard Library is divided into a number of
> focused modules, [all listed further down this page](#modules). These
> modules are the bedrock upon which all of Rust is forged, and they
> have mighty names like [`std::slice`](https://doc.rust-lang.org/stable/std/slice/index.html "mod std::slice") and [`std::cmp`](https://doc.rust-lang.org/stable/std/cmp/index.html "mod std::cmp").

This is the corpus statement that the standard library is *divided
into modules* and that `std::cmp` is one of those modules. The
lesson's "`std::cmp` is a submodule of `std`" rephrases this. The
parenthetical anchor labels the link target as `mod std::cmp`,
confirming the corpus's own term for `std::cmp` is *module*.

### `output/docs/rust/reference/items/modules.md`

The Reference page for module items. Two load-bearing spans.

Lines 19 and 22-23 (the *module* definition):

> A module is a container for zero or more [items](../items.md).
>
> A *module item* is a module, surrounded in braces, named, and
> prefixed with the keyword `mod`. A module item introduces a new,
> named module into the tree of modules making up a crate.

These are the canonical Reference-level definitions. The lesson uses
the looser audience phrase "a namespace that holds functions, types,
constants, and other items," which combines the Reference's "container
for zero or more items" with the Book chapter 7-02's "Modules can also
hold definitions for other items, such as structs, enums, constants,
traits, and ... functions" (cited next). The lesson explicitly defers
the *defining your own modules with `mod` { ... }* surface to *What To
Ignore For Now*; today's lesson installs only the *consumption* surface
of an already-defined module path.

Lines 25-27 (modules nest):

> Modules can nest arbitrarily.

This is the corpus license for the multi-segment path. The lesson's
"any number of `::`-separated segments" rests on this — module nesting
is unbounded, so a path can have any number of module segments before
the final item-name segment.

### `output/docs/rust/reference/paths.md`

The Reference page for paths. Already cited in lessons 041 and 042 for
the qualified path grammar; reused here for the multi-segment case.

Line 8 (the path definition):

> A *path* is a sequence of one or more path segments separated by `::`
> tokens. Paths are used to refer to [items](items.md), values,
> [types](types.md), [macros](macros.md), and [attributes](attributes.md).

This is the corpus definition of *path* and *path separator*. The
lesson's "`::` separates each segment" comes from this line.

Lines 14-17 (the simple-paths example):

> ```rust
> x;
> x::y::z;
> ```

The Reference's own demonstration of multi-segment paths — a single-
segment path `x` and a three-segment path `x::y::z`. The lesson's
`std::cmp::min` is exactly the three-segment shape `x::y::z`.

Lines 56-62 (Paths in expressions grammar):

> [PathInExpression](paths.md#railroad-PathInExpression) →
>     ::? [PathExprSegment](paths.md#grammar-PathExprSegment) ( ::
>     [PathExprSegment](paths.md#grammar-PathExprSegment) )*
>
> [PathExprSegment](paths.md#railroad-PathExprSegment) →
>     [PathIdentSegment](paths.md#grammar-PathIdentSegment) ( ::
>     [GenericArgs](paths.md#grammar-GenericArgs) )?

The grammar fragment explicitly licenses *one or more* path segments
(the `( :: PathExprSegment )*` Kleene star). Lesson 041 used the two-
segment case (`i32::abs`); today's lesson installs the *more than two*
case via `std::cmp::min`. The lesson does not reproduce the grammar in
the body — only the spoken-English shape "any number of `::`-separated
segments, final segment names the function."

### `output/docs/rust/book/ch07-02-defining-modules-to-control-scope-and-privacy.md`

The Book chapter that introduces modules in plain English. Two load-
bearing spans.

Lines 147-152 (the *module* introduction in plain English):

> We define a module with the `mod` keyword followed by the name of the
> module (in this case, `front_of_house`). The body of the module then
> goes inside curly brackets. Inside modules, we can place other
> modules, as in this case with the modules `hosting` and `serving`.
> Modules can also hold definitions for other items, such as structs,
> enums, constants, traits, and as in Listing 7-1, functions.

Cited for two claims: (a) "Modules can also hold definitions for other
items, such as structs, enums, constants, traits, ... functions" — the
lesson's audience-level "namespace that holds functions, types,
constants, and other items" rephrases this list; and (b) "Inside
modules, we can place other modules" — the corpus statement that
modules nest, complementing the Reference's "Modules can nest
arbitrarily." The Book chapter's running example is a user-defined
module tree (`front_of_house::hosting::add_to_waitlist`); the lesson
uses the consumption-side analog `std::cmp::min`.

Lines 160-163 (the *module tree* and *crate* root):

> Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called
> *crate roots*. The reason for their name is that the contents of
> either of these two files form a module named `crate` at the root of
> the crate's module structure, known as the *module tree*.

Cited for the *root module* framing the lesson uses for `std`. The
Book's "module named `crate` at the root of the crate's module
structure" plus the std-page's "the standard library can be accessed
... through the path `std`" together ground the lesson's "`std` is the
standard library's root module." The lesson explicitly defers the
`crate::` path-root form to *What To Ignore For Now* — today's lesson
uses only the externally-rooted `std::` path.

### `output/docs/rust/error_codes/E0425.md`

The error-code explainer for E0425, "an unresolved name was used."
Already cited in lessons 005, 008, 040, and 042. Reused here for the
broken-contrast probe. Two load-bearing spans.

Line 4 (the corpus statement):

> An unresolved name was used.

The lesson does not re-explain E0425; it cites lessons 005, 008, 040,
and 042 for the E-code's prior installations.

Lines 41-50 (corpus example with a path-qualified resolution):

> ```rust
> #![allow(unused)]
> fn main() {
> mod something_that_does_exist {
>     pub static foo : i32 = 0i32;
> }
>
> something_that_does_exist::foo; // ok!
> }
> ```

Corpus precedent for *fixing* an E0425 by writing the full path. The
broken probe's `min(3, 5)` (no path) fires E0425; the working probe's
`std::cmp::min(3, 5)` (full path) compiles. The corpus example
demonstrates the same path-qualifies-the-resolution pattern in
miniature with a user-defined module.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/043-nested-module-paths.rs`.
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
fn main() {
    let smaller: i32 = std::cmp::min(3, 5);
    let larger: i32 = std::cmp::max(3, 5);
    println!("smaller = {smaller}, larger = {larger}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
smaller = 3, larger = 5
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line: `smaller = 3, larger = 5`. This
  corroborates the std-page statements that `min` "compares and returns
  the minimum of two values" and `max` "compares and returns the
  maximum of two values" — for the inputs `3` and `5`, `min` returns
  `3` and `max` returns `5`.
- The two function calls share the prefix `std::cmp::` and differ only
  in the final segment (`min` vs. `max`). This is the *path-as-namespace*
  point made empirical: the `std::cmp` namespace contains both items,
  reached by appending `min` or `max` after the second `::`.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: same shape as the working probe with line 2 changed from
`let smaller: i32 = std::cmp::min(3, 5);` to
`let smaller: i32 = min(3, 5);` (free-function form with no path) and
the second function call removed for minimality. Not committed; the
transcript below is the artifact. Captured 2026-05-07 in a fresh
`mktemp -d` (filename `broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn main() {
    let smaller: i32 = min(3, 5);
    println!("smaller = {smaller}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0425]: cannot find function `min` in this scope
 --> broken.rs:2:24
  |
2 |     let smaller: i32 = min(3, 5);
  |                        ^^^ not found in this scope
  |
help: consider importing this function
  |
1 + use std::cmp::min;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0425]: cannot find function ` then a
  backtick-quoted `min`, then ` in this scope`. Same E-code lessons
  005, 008, 040, and 042 installed (005 for missing values, 008 for
  missing free functions, 040 for the dot-form broken contrast, 042
  for the qualified-path missing-qualifier broken contrast). The
  headline word "function" matches lesson 008's shape — rustc parsed
  `min(3, 5)` as a free-function call, found no free function named
  `min` in scope, and emitted the missing-name diagnostic.
- The diagnostic has the four lesson-003 parts: headline + `-->`
  location (`broken.rs:2:24`) + source excerpt with `^^^` caret under
  `min` + `help:` block. The `--explain E0425` trailer is also
  present, consistent with lesson 003's rule that headlines with an
  `[E####]` code emit the trailer.
- The `help:` block reads literally `help: consider importing this
  function`, followed by a source-diff suggestion `1 + use std::cmp::min;`
  inserting a new line at the top of the file (the `+` prefix is rustc's
  insertion marker; the `1` is the line number to insert at). The
  suggested `use std::cmp::min;` is the *next* future move — a `use`
  declaration that brings the path's final segment into local scope so
  `min(3, 5)` becomes valid below it. The lesson explicitly *does not*
  install `use`. The same broken program is also fixed by switching
  `min(3, 5)` back to `std::cmp::min(3, 5)`, which is the move this
  lesson installs. Both fixes resolve the unresolved name; the lesson
  body acknowledges this honestly and defers `use` to *What To Ignore
  For Now*.
- *Calibration with lessons 040 and 042*: lesson 040's broken-contrast
  probe (`abs(n)`) emitted a `help:` block suggesting the dot-form
  `n.abs()`, the move that lesson installed. Lesson 042's broken probe
  (`new()` with no qualifier on a `: String` slot) emitted no `help:`
  at all. Today's broken probe (`min(3, 5)` with no path) emits a
  `help:` suggesting `use std::cmp::min;`, which is *not* the move
  this lesson installs — rustc points at a future move whose effect
  is also achievable with the full path the lesson teaches. This is
  honest probe evidence; the lesson body names the discrepancy
  explicitly rather than pretending rustc suggests the full-path fix.
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`, no `broken` binary.
- The lesson's body claim "writing `min(3, 5)` rejects with E0425" maps
  directly to the headline above. The lesson's claim that rustc's
  `help:` block here suggests `use std::cmp::min;` (a future move not
  installed in this lesson) maps directly to the captured `help:`
  block. The lesson's claim that the same program is also fixed by the
  full path `std::cmp::min(3, 5)` is grounded by the working probe
  (which compiles cleanly with that exact substitution).

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with the full path it works, without it fails").
The captured `help:` text — naming `use std::cmp::min;` rather than the
qualified path — is the load-bearing piece of probe evidence that
required honest disclosure: rustc's *first* suggested fix here is the
future-move `use` form, not this lesson's full-path form. The lesson
body acknowledges this directly.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 043. Older supporting lessons are mentioned
above by number only.

- **Lesson 041 (load-bearing)** — installed the qualified path call
  form `Type::name(args)` with one type segment in front of `::`
  (`i32::abs(n)`). Lesson 043 generalizes the same `::`-separated path
  shape to *more than two* segments and to *modules* in front of the
  final segment instead of types. The Reference's path grammar
  already licensed multi-segment paths (cited under *paths.md* lines
  56-62 above); lesson 041 demonstrated the two-segment case.
- **Lesson 042 (load-bearing)** — installed the no-receiver call shape
  `Type::name(args)`. Lesson 043 reuses *that exact call-site shape*
  with a longer path: `std::cmp::min(3, 5)` is `name(args)` reached by
  a longer path; there is no receiver, just a function called by its
  full path. Lesson 042 also installed E0425 as the failure mode for a
  missing qualifier (`new()` instead of `String::new()`); lesson 043's
  broken probe fires the same E-code on the same kind of mistake
  (`min(3, 5)` instead of `std::cmp::min(3, 5)`). The new fact in 043
  is the structural one: the path is *longer*, and the leading segments
  are *modules*, not a type.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `help:` lines.
  Lesson 043's broken-contrast walk uses that map without re-teaching
  it. The specific observation that *this* E0425's `help:` suggests a
  future move (`use std::cmp::min;`) is consistent with lesson 003's
  framing — the help block is rustc's diagnostic suggestion, not
  necessarily the move the current lesson teaches.
- **Lesson 020 (and through it 021)** — typed parameters and call
  expressions returning a value. Lesson 043's `std::cmp::min(3, 5)`
  supplies two `i32` arguments (matching the inferred `T = i32`) and
  produces an `i32` return value, which fits on the right of
  `let smaller: i32 = ...;`. No new mechanism — only a new way to
  *name* the function being called.
- **Lesson 005 (and 019)** — `let name: TYPE = value;` annotated
  binding. Lesson 043 uses this shape unchanged: `let smaller: i32 =
  std::cmp::min(3, 5);`. The `i32` annotation matches the inferred
  return type; without it, rustc would still infer `i32` from the
  literal arguments.
- **Lessons 001, 002** — `rustc file.rs` then `./name`; `fn main` is
  the entry point. Used unchanged.

## Older supporting lessons

Lessons 005, 008, 040, 042 (E0425 family connection — the broken-
contrast probe fires E0425, the same E-code first installed in lesson
005 for missing values and reused in 008, 040, 042; not re-stated here
beyond the family connection).

Lesson 036 (multi-argument calls — `std::cmp::min(3, 5)` is a two-
argument call; lesson 036 installed positional argument matching for
multi-parameter functions; not re-stated here, the two-argument call
shape is used unchanged).
