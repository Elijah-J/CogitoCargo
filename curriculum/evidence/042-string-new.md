# Evidence — 042-string-new

Audit appendix for `lessons/042-string-new.md`. Holds the corpus-quote
map, the toolchain string, the full working and broken-contrast probe
transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/042-string-new.rs`); the broken-contrast `.rs` is not
  committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/alloc/string/struct.String.md`

The std-library page for `String`. Three load-bearing spans.

Lines 12-15 (the type's introduction):

> A UTF-8–encoded, growable string.
>
> `String` is the most common string type. It has ownership over the
> contents of the string, stored in a heap-allocated buffer (see
> Representation).

This is the corpus statement of what `String` *is*: a growable,
heap-allocated, owned text type. The lesson body uses the bare phrase
"the standard heap-allocated growable text type" to match this corpus
description without unpacking ownership semantics. The lesson's *What
To Ignore For Now* explicitly defers ownership/move/`Drop` and
heap-allocation specifics.

Line 275 (the canonical `new` signature):

> #### pub const fn new() -> String

The signature has zero parameters in its parameter list and produces a
fresh `String` value. This is the corpus basis for the lesson's claims
that (a) `String::new()` is called with empty parens because the
function takes nothing, (b) there is no receiver to put on the left of
a dot, and (c) the call expression produces a `String` that fits on
the right of `let s: String = ...;`.

Lines 277-290 (the `new` description and example):

> Creates a new empty `String`.
>
> Given that the `String` is empty, this will not allocate any initial
> buffer. While that means that this initial operation is very
> inexpensive, it may cause excessive allocation later when you add
> data. ...
>
> ##### Examples
>
> ```
> let s = String::new();
> ```

The first sentence is the corpus statement of what `new` does
("creates a new empty `String`"). The Examples block is *the exact
shape* the lesson uses — `let s = String::new();` — minus the explicit
`: String` annotation, which is presentational (lesson 019's shape).
The "will not allocate any initial buffer" detail is part of the
deferred heap-mechanics topic; the lesson does not surface it.

Calibration: the std-library link tags throughout this same page label
`new` and its siblings as "associated function" — e.g. line 306 reads
`is identical to the [\`new\`](struct.String.md#method.new "associated
function alloc::string::String::new") method.` and line 241 labels
`with_capacity` as "associated function alloc::string::String::with_capacity".
This is corpus-level confirmation that `new` is the kind of thing the
Reference calls an associated function (cited next).

### `output/docs/rust/reference/items/associated-items.md`

The Reference page for associated items. Primary corpus source for the
*main concept*. Three load-bearing spans.

Line 42:

> *Associated functions* are functions associated with a type.

Plain definition. The lesson's "functions associated with a type"
framing comes verbatim from this line.

Line 62 (the *load-bearing* corpus statement of the lesson's main
concept):

> A common example is an associated function named `new` that returns
> a value of the type with which it is associated.

This is the canonical corpus statement that `new` is a common
associated-function name returning a fresh value of the type. The
lesson body's "Many types in Rust's standard library have an
associated function called `new` that returns a fresh instance of that
type" rephrases this directly. The lesson's narrower concrete claim
("`String::new()` returns an empty `String`") combines this Reference
statement with the std page's specific signature for `String::new`.

Lines 64-80 (the example body):

> ```rust
> struct Struct {
>     field: i32
> }
>
> impl Struct {
>     fn new() -> Struct {
>         Struct {
>             field: 0i32
>         }
>     }
> }
>
> fn main () {
>     let _struct = Struct::new();
> }
> ```

Direct corpus precedent for the no-receiver pattern: `fn new() ->
Struct` takes zero parameters, and the call site is `Struct::new()`
with no value-side dot form. The lesson uses `String::new()` in the
identical position. Calibration: the Reference's example is on a
user-defined struct; `String::new()` is the same shape on a standard-
library type. The lesson defers `impl` blocks (still deferred from
lessons 040-041) and only uses the *call-site* surface.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book chapter that introduces the guessing game. Cited for the
audience-level prose statement of `String` and `String::new`. Two
load-bearing spans.

Lines 237-240:

> ... the result of calling `String::new`, a function that returns a
> new instance of a `String`. `String` is a string type provided by
> the standard library that is a growable, UTF-8 encoded bit of text.

Plain-English statement of (a) what `String` is — "a string type
provided by the standard library that is a growable ... bit of text"
— and (b) what `String::new` does — "returns a new instance of a
`String`". The lesson's audience-level prose mirrors this without
quoting. The Book's "growable, UTF-8 encoded" overlaps with the
Representation phrase from the std page; the lesson uses
"heap-allocated growable text type" to combine both, anchored on the
std page's `heap-allocated buffer` clause.

Lines 242-246:

> The `::` syntax in the `::new` line indicates that `new` is an
> associated function of the `String` type. An *associated function*
> is a function that's implemented on a type, in this case `String`.
> This `new` function creates a new, empty string. You'll find a
> `new` function on many types because it's a common name for a
> function that makes a new value of some kind.

Audience-level corpus statement of the lesson's main concept,
identical in substance to associated-items.md line 62 but in plain
prose. Two specific claims rest on this:

1. "The `::` syntax in the `::new` line" — the Book's name for the
   qualified-path call shape lesson 041 installed.
2. "You'll find a `new` function on many types because it's a common
   name for a function that makes a new value of some kind" — the
   "convention" claim in the lesson's *What Changed* bullet ("`new`
   is a common associated-function name in the standard library").

Calibration: the Book uses "associated function" as the term; the
Reference uses both "associated function" and the framing
"associated items declared in implementations". The lesson uses
neither term explicitly in the body — it uses the looser audience
phrase "functions attached to a type" and the concrete example
`String::new()`. The audit-evidence map (this appendix) names the
Reference's term *associated function* once for completeness, but
the lesson body avoids it to keep the noun count low.

The Book's surrounding code (`let mut guess = String::new();`) uses
`mut` because the guessing game later mutates `guess` via
`io::stdin().read_line(&mut guess)`. The lesson's program does not
mutate `s`, so `mut` is not added. The lesson's *What To Ignore For
Now* defers all mutation explicitly.

### `output/docs/rust/error_codes/E0425.md`

The error-code explainer for E0425, "an unresolved name was used."
Already cited in lessons 005, 008, 040. Reused here only for the
family connection — the broken-contrast probe fires E0425 because
`new` is not a free function in scope. The corpus-level statement is
the page's opening sentence:

> An unresolved name was used.

The lesson does not re-explain E0425; it cites lessons 005, 008, and
040 by number for the E-code's prior installations.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/042-string-new.rs`.
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
    let s: String = String::new();
    println!("empty: [{s}]");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
empty: []
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line: `empty: []`. Zero characters
  appear between the `[` and the `]`, confirming that
  `String::new()` produced an *empty* `String`. This corroborates
  the std-page statement "Creates a new empty `String`" plus the
  std-page example `let s = String::new();`.
- The named-placeholder `{s}` form (lesson 005) printed the bound
  `String`'s text content — zero characters — directly between the
  bracket literals.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: same as the working probe with line 2 changed from
`let s: String = String::new();` to `let s: String = new();`
(free-function form with no qualified path). Not committed; the
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
    let s: String = new();
    println!("empty: [{s}]");
}
--- rustc broken.rs (capturing stderr) ---
error[E0425]: cannot find function `new` in this scope
 --> broken.rs:2:21
  |
2 |     let s: String = new();
  |                     ^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0425]: cannot find function ` then a
  backtick-quoted `new`, then ` in this scope`. Same E-code lessons
  005, 008, and 040 installed (005 for missing values, 008 for missing
  free functions, 040 for the dot-form broken contrast). The headline
  word "function" matches lesson 008's shape — rustc parsed `new()`
  as a free-function call.
- The diagnostic has the four lesson-003 parts: headline + `-->`
  location (`broken.rs:2:21`) + source excerpt with `^^^` caret under
  `new` + (no `help:` block). The `--explain E0425` trailer is
  present, consistent with lesson 003's rule that headlines with an
  `[E####]` code emit the trailer.
- *Calibration with lesson 040*: lesson 040's broken-contrast probe
  for `abs(n)` produced a `help:` block reading "use the `.` operator
  to call the method `abs` on `i32`" with a source-diff
  `n.abs()`. *This* probe — a missing `new` in the right of an
  `: String`-annotated `let` — does **not** emit such a `help:` block.
  rustc here only points at the missing name; it does not auto-suggest
  the `String::new()` qualified path. This is honest probe evidence,
  captured here so the lesson can describe the diagnostic accurately.
  The contrastive claim ("the qualified path is the only way to reach
  it") is grounded structurally — there is no free function `new` in
  scope, the std-page signature lives in `impl String`, and the
  Reference plus Book describe the call shape `Type::new()` — rather
  than via a captured rustc `help:` line.
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`, no `broken` binary.
- The lesson's body claim "writing `new()` instead of `String::new()`
  fires E0425 'cannot find function `new` in this scope'" maps
  directly to the headline above. The lesson's claim that rustc does
  *not* auto-suggest the qualified path here is recorded in the
  *Try It* paragraph immediately after the diagnostic block, in
  contrast to lesson 040's `help:`-suggesting probe.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with the qualified path it works, without it
fails"). The lesson's working-side claim — that `String::new()`
produces an empty `String` — is corroborated by both the working
probe's `empty: []` output and the std page's signature plus
`String::new()` example. The broken-side claim — that there is no
free-function form `new()` reachable in scope — is corroborated by
the broken probe's E0425 plus the structural fact that `new` is
defined in `impl String` (per the std page) and the Reference's
associated-items page describes the qualified call shape as the way
to reach such functions.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 042. Older supporting lessons are mentioned
above by number only.

- **Lesson 041 (load-bearing)** — installed the qualified call form
  `Type::method(receiver, args)` for receiver-bearing methods. This
  lesson installs the *no-receiver* sub-case `Type::name(args)` (with
  `args` possibly empty) on the same syntactic surface. The new
  delta is "no receiver — and therefore no value-side dot form."
  Lesson 041's broken-contrast probe targeted *receiver omitted from
  the qualified form* (E0061); this lesson's broken contrast targets
  *qualifier omitted entirely* (E0425). Different E-codes because
  the failure modes are different: in 041 the function exists and is
  reached but the arity is wrong; in 042 the function name is not in
  scope at all because `new` is only reachable via the qualified path.
- **Lesson 040 (load-bearing for context)** — installed the dot-form
  `value.method(args)`. Lesson 042 contrasts with this by pointing
  out that no value exists to put a dot on — `new` is reached only
  via the qualified path. Lesson 040's E0425 broken-contrast probe is
  the *closest* prior precedent for today's broken probe; the
  difference is that 040's probe could be *fixed* by switching from
  free-function form to dot-form (and rustc auto-suggested the fix),
  whereas today's probe must be fixed by switching to the qualified
  path (rustc does not auto-suggest the fix).
- **Lesson 019 (load-bearing for shape)** — installed the type-
  annotation form `let name: TYPE = value;` with `i32` as the
  example `TYPE`. Lesson 042 reuses the same shape with `String` in
  the `TYPE` slot. Lesson 019's body framing already generalizes the
  shape ("the `: TYPE` slot between `name` and `=`"), even though its
  example was `i32`. The lesson body explicitly notes the extension
  to a new `TYPE`. No new mechanism — only a new `TYPE` flowing
  through the same slot.
- **Lesson 005 (load-bearing)** — installed `let name = value;`
  binding plus the named-placeholder `{name}` form for `println!`.
  Lesson 042 uses both unchanged: `let s: String = String::new();`
  is the lesson-005 binding shape with the lesson-019 type-annotation
  slot, and `println!("empty: [{s}]")` is the lesson-005 named-
  placeholder shape with brackets around the placeholder for
  visibility.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `help:` lines.
  Lesson 042's broken-contrast walk uses that map without re-teaching
  it. The specific observation that *this* E0425 has no `help:` block
  (in contrast to lesson 040's E0425) is consistent with lesson
  003's "optional `help:` lines" framing — the help block was
  optional then, and is absent here.
- **Lessons 001, 002** — `rustc file.rs` then `./name`; `fn main` is
  the entry point. Used unchanged.

## Older supporting lessons

Lesson 008 (free-function call form `name(args)` — the call shape
that the broken-contrast probe accidentally invokes when the
qualifier is dropped; reached indirectly via lesson 040's contrast
and lesson 005's E0425 family). All other supporting lessons are
reachable through the direct prerequisites listed above.
