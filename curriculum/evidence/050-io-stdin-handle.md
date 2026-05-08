# Evidence — 050-io-stdin-handle

Audit appendix for `lessons/050-io-stdin-handle.md`. Holds the corpus-
quote map, the toolchain string, the working probe transcript, the
contrastive-probe-omission justification, and the prerequisite-claim
summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probe run in a fresh `mktemp -d` directory, removed at the end of
  the run. Only the working `.rs` is committed (under
  `observations/050-io-stdin-handle.rs`).

## Sources

### `output/docs/rust/std/io/fn.stdin.md`

The std-library page for `std::io::stdin`. Primary source. Three
load-bearing spans.

Lines 6-8 (the canonical signature):

> ```
> pub fn stdin() -> Stdin
> ```

The signature has zero parameters in its parameter list and a return
type of `Stdin`. This is the corpus basis for the lesson body's three
specific claims about the function: (a) the call site is
`std::io::stdin()` with empty parens because the function takes
nothing, (b) it is a free function (not a method, no receiver), and
(c) the call expression produces a value of type `std::io::Stdin`
that fits on the right of `let _stdin = ...;` and is bindable.
Calibration: the `pub fn` keyword (no `const` modifier, no `unsafe`
modifier, no generic parameters) confirms `stdin` is an ordinary
public function with the simplest possible signature shape that
returns a value. Today's lesson does not surface `pub` or any
modifier — it just observes that the function exists and the call
returns a value.

Line 12 (the description):

> Constructs a new handle to the standard input of the current process.

This is the corpus statement of what `stdin()` *does*: it returns a
handle to the standard input. The lesson body's "produces a
`std::io::Stdin` value — a handle through which a future move will
read input" rephrases the first half of this sentence; the
"standard input of the current process" half is glossed in the
audience-level term *standard-input handle*. The lesson does not
unpack "current process," "shared global buffer," or "synchronized
via a mutex" (line 14-15) — heavy deferral.

Lines 14-16 (the singleton-and-mutex note):

> Each handle returned is a reference to a shared global buffer
> whose access is synchronized via a mutex.

This is the corpus statement that the stdin handle is shared/global
and that concurrent access uses a mutex. The lesson explicitly defers
both points (`What To Ignore For Now`: "Threading, the internal
mutex, and stdin singleton-ness — heavy deferral; not installed").
Cited here only to mark that the deferral is faithful to the corpus,
not invented.

The page's two examples (lines 30-57) both use `use std::io;` and
then `io::stdin()`, plus method chains (`.read_line(&mut buffer)?`,
`.lock()`). The lesson uses the *full path* form `std::io::stdin()`
and calls no method — both `use` and any method on `Stdin` are
deferred. Calibration: the corpus page's preferred style is the
shortened `io::stdin()` form, but the full-path `std::io::stdin()`
form is unambiguously valid (lesson 043 grounded the full-path
shape on the same path-grammar production), and the Book chapter 2
explicitly says so (cited next).

### `output/docs/rust/std/io/struct.Stdin.md`

The std-library page for the type `std::io::Stdin`. Cited for the
type's existence and its plain-English description. Two load-bearing
spans.

Lines 6-8 (the type's canonical signature):

> ```
> pub struct Stdin { /* private fields */ }
> ```

This is the corpus statement that `Stdin` is a struct in `std::io`
with private fields. The lesson surfaces only the bare term *type*
("a value of type `std::io::Stdin`"); the keyword `struct` and the
private-fields detail are *What To Ignore For Now*-adjacent (struct
syntax has not been installed; the lesson treats `Stdin` as an
opaque type whose only relevant fact today is "it exists as the
return type of `stdin()`").

Line 12 (the type's plain-English description):

> A handle to the standard input stream of a process.

This is the corpus's plain-English name for `Stdin`: a *handle* to
the standard input stream. The lesson body's "*standard-input
handle*" comes directly from this sentence with the words
re-ordered. Calibration: the page goes on (lines 14-21) to describe
the handle as "a shared reference to a global buffer", to note
`BufRead` access via `.lock()`, and to mention the `Read` trait
implementation. All of those are deferred.

Line 22 (the cross-reference back to `fn.stdin.md`):

> Created by the [`io::stdin`](fn.stdin.md "fn std::io::stdin") method.

This corpus line confirms the link between the function and the
type: the only documented way to obtain a `Stdin` value is to call
`io::stdin()`. The lesson uses the full-path spelling
`std::io::stdin()` rather than the page's `io::stdin()` (which
relies on a `use std::io;`); both reach the same function.

The page also documents an `impl Debug for Stdin` (lines 186-194).
Cited here only for completeness — `Stdin` is `Debug`-printable,
which would let the lesson use `{:?}` to format it. The lesson does
*not* use `{:?}` because `Debug` formatting is uninstalled. The
program prints a string literal `"got stdin handle"` instead.

### `output/docs/rust/std/index.md`

The crate root page for `std`. Already cited in lesson 043 for
`std` as the standard library's root module. Reused here for the
load-bearing claim that `std::io` is the input/output submodule of
`std`. One new load-bearing span (in addition to lesson 043's
already-cited spans).

Line 14 (the *load-bearing* clause naming `std::io` as a module):

> It offers core types, like `Vec<T>` and `Option<T>`, library-
> defined operations on language primitives, standard macros, [I/O](https://doc.rust-lang.org/stable/std/io/index.html "mod std::io")
> and multithreading, among many other things.

The link target text reads `mod std::io`, confirming the corpus's
own term for `std::io` is *module*. This is the corpus license for
the lesson body's "`std::io` is its input/output submodule" — the
"submodule" framing combines this line's `std::io` mention with
the same page's lesson-043-cited "the standard library is divided
into a number of focused modules" (lines 54-58).

Calibration: the std crate page does not contain a dedicated `io`
module overview file under `output/docs/rust/std/io/index.md` — that
path simply does not exist in this corpus. The corpus's authority
for "`std::io` is the standard library's I/O module" is therefore
this single link in `std/index.md` line 14, plus the existence of
items at paths under `std/io/` (e.g. `fn.stdin.md`,
`struct.Stdin.md`, `fn.stdout.md`, `fn.stderr.md`, etc.). The
lesson's "`std::io` is its input/output submodule" rephrases this
modestly.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book chapter that introduces the guessing game. Already cited
in lesson 042 for `String` and `String::new`. Reused here for the
audience-level descriptions of `stdin()` and `Stdin`. Two
load-bearing spans.

Lines 254-256 (the audience-level introduction of `stdin`):

> ... we'll call the `stdin` function from the `io` module, which
> will allow us to handle user input

This is the Book's plain-English statement that `stdin` is a
function in the `io` module. The lesson body's "`std::io::stdin`
is a free function inside that submodule" rephrases this. The
Book uses the bare word "function" — same noun-class lesson 008
installed.

Lines 276-280 (the *load-bearing* full-path note plus the
type/handle description):

> If we hadn't imported the `io` module with `use std::io;` at the
> beginning of the program, we could still use the function by
> writing this function call as `std::io::stdin`. The `stdin`
> function returns an instance of `std::io::Stdin`, which is a type
> that represents a handle to the standard input for your terminal.

Three claims rest directly on this corpus block:

1. "We could still use the function by writing this function call
   as `std::io::stdin`" — the Book's explicit corpus license for
   the full-path call shape the lesson uses. This is the load-
   bearing sentence justifying the lesson's choice to use the full
   `std::io::stdin()` path rather than the `use std::io;` +
   `io::stdin()` shortened form (which is the chapter's running
   style). The two forms call the same function; the lesson uses
   the full path because lesson 044's `use` is not load-bearing
   for today's main concept.
2. "The `stdin` function returns an instance of `std::io::Stdin`" —
   the Book-level statement of the function's return type. Combines
   with `std/io/fn.stdin.md` line 7's signature `pub fn stdin() ->
   Stdin` to ground "the call returns a `std::io::Stdin` value."
3. "`std::io::Stdin`, which is a type that represents a handle to
   the standard input for your terminal" — the Book's plain-English
   gloss of `Stdin` as a *handle*, complementing
   `std/io/struct.Stdin.md` line 12's "A handle to the standard
   input stream of a process." The lesson body's "standard-input
   handle" comes from these two corpus statements taken together.

Calibration: the Book chapter's surrounding code uses the chained
form `io::stdin().read_line(&mut guess).expect("Failed to read
line");` (lines 268-270). The lesson teaches *only* the leading
`io::stdin()` step and explicitly defers `.read_line(&mut buf)`
plus `.expect(...)` to future moves. The lesson body acknowledges
this by listing `.read_line(&mut buf)` as "the natural next move"
under *What To Ignore For Now*.

### `output/docs/rust/reference/items/modules.md`

The Reference page for module items. Already cited in lesson 043
for the *module* definition. Not re-quoted here — the
load-bearing claim ("modules can nest arbitrarily") was grounded
in lesson 043 and is reused unchanged today. Listed here only for
the audit trail.

### `output/docs/rust/reference/paths.md`

The Reference page for paths. Already cited in lessons 041, 042,
043 for the path grammar. Not re-quoted here — the load-bearing
grammar production licensing arbitrary-length `::`-separated
paths was grounded in lesson 043 (`PathInExpression → ::?
PathExprSegment ( :: PathExprSegment )*`) and is reused unchanged
today. The path `std::io::stdin` is exactly the
three-`PathExprSegment` shape lesson 043 used (`std::cmp::min`)
with different segment names. Listed here only for the audit
trail.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/050-io-stdin-handle.rs`.
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
    let _stdin = std::io::stdin();
    println!("got stdin handle");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
got stdin handle
exit=0
```

Notes:

- `rustc demo.rs` exits 0 silently (no diagnostic output of any
  kind — neither error nor warning). Confirms (a) the path
  `std::io::stdin` resolves with no `use` declaration, (b) the
  function exists and is callable with empty parens, (c) the
  return value binds cleanly with `let _stdin = ...;`, and (d)
  the underscore prefix on the binding name is sufficient to
  silence the unused-variable warning that would otherwise fire.
- `./demo` prints exactly one line: `got stdin handle`. This is
  the literal string from the `println!` invocation; no input is
  read. The program runs to completion immediately. The empirical
  claim "creating the stdin handle is a separate step from
  reading through it" is corroborated by the program running
  straight through without blocking on input.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.
- The leading underscore on `_stdin` is essential to keeping the
  rustc transcript clean. The next subsection (Underscore-elision
  side probe) records what rustc emits without the underscore, as
  honest probe evidence.

### Underscore-elision side probe

Source: working-probe shape with line 2 changed from
`let _stdin = std::io::stdin();` to `let stdin = std::io::stdin();`
(no underscore prefix). Not committed; the transcript below is
the artifact. Captured 2026-05-07 in a fresh `mktemp -d`
(filename `noprefix.rs`):

```text
--- cat noprefix.rs ---
fn main() {
    let stdin = std::io::stdin();
    println!("got stdin handle");
}
--- rustc noprefix.rs ---
warning: unused variable: `stdin`
 --> noprefix.rs:2:9
  |
2 |     let stdin = std::io::stdin();
  |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_stdin`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted

exit=0
--- ls after ---
noprefix
noprefix.rs
--- ./noprefix ---
got stdin handle
exit=0
```

Notes (probe evidence — not corpus quotation):

- The diagnostic headline is *warning*, not *error*. rustc's exit
  code is 0 and an executable is produced. Both observations
  confirm warnings are a separate category from errors — the
  category distinction lesson 029 first surfaced as a deferred
  unlock.
- The diagnostic structure matches lesson 003's map: headline
  + `-->` location + source excerpt with caret + inline `help:`
  text + `= note:` trailer naming the lint
  (`#[warn(unused_variables)]`). Same lesson-003 grammar, no new
  diagnostic mechanism.
- The `help:` text reads literally: `if this is intentional,
  prefix it with an underscore: `_stdin``. rustc itself names the
  underscore-prefix convention as the fix. This is the corpus-
  level (probe-level) confirmation that the lesson's `_stdin`
  binding choice is the standard idiom rustc itself recommends,
  not a workaround invented by the lesson.
- `./noprefix` still prints `got stdin handle`. The runtime
  behavior is unchanged — the warning does not affect execution.
- Recorded here so the lesson body can claim "without the
  underscore the program still compiles but rustc emits an
  unused-variable warning" with empirical backing.

### Contrastive-probe-omission justification

The lesson does *not* make a "with X works, without X
fails/differs" claim. The central claim is *introduction*: the
function `std::io::stdin` and the type `std::io::Stdin` exist at
those paths, and the function returns a value of that type. This
is demonstrated *directly* by the working probe — `rustc demo.rs`
exits 0 silently and `./demo` produces the expected output, which
together prove the path resolves, the function exists, the call
returns a value, and the binding succeeds.

Per the README's *Audit Trail Depth* section: "when the move says
'with X this works, without X it fails/differs,' include a
negative/contrast probe or state why one is not needed." Today's
move says no such thing, so no broken-contrast probe is required.

The orchestrator's prompt suggested two candidate broken contrasts
and recommended skipping both:

- *Typo the path (e.g., `std::io::stdimnput()`)* — would fire
  E0599 *no function or associated item named `stdimnput` found
  in module `std::io`*. E0599 is **not yet installed** in the
  graph; surfacing it would leak. SKIP.
- *Omit a path segment (e.g., `std::stdin()`)* — would fire either
  E0425 (cannot find name) or E0433 (failed to resolve). E0425 is
  installed (lessons 005, 008, 040, 042, 043, 044); E0433 is
  **not installed**. The actual rustc diagnostic varies by
  version. Probe-level evidence would be required to confirm
  which E-code fires today. SKIP per orchestrator recommendation.

The underscore-elision side probe above is *positive* evidence
for the lesson's small calibration claim about underscore prefix,
not a broken contrast for the main concept. The working probe
alone suffices for the main concept.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 050. Older supporting lessons are mentioned
above by number only.

- **Lesson 042 (load-bearing for the no-receiver call shape)** —
  installed `Type::name(args)` with empty parens and no value-side
  dot form, with `String::new()` as the smallest concrete
  instance. Lesson 050 reuses *that exact call-site shape* but
  generalizes the leading segment from a *type* (`String`) to a
  *module* path prefix (`std::io`), and the final segment from an
  associated function on a type (`new`) to a free function in a
  module (`stdin`). The "no receiver, empty argument list,
  qualified path is the only call form" mechanic transfers
  unchanged.
- **Lesson 043 (load-bearing for the nested-module-path shape)**
  — installed the multi-segment path
  `module::submodule::name(args)` with `std::cmp::min(3, 5)` as
  the example, and named `std` as the standard library's root
  module. Lesson 050 reuses *that exact path shape* with three
  `::`-separated segments, the leading `std`, and a different
  submodule (`io` instead of `cmp`) and final function (`stdin`
  instead of `min`/`max`). The path-grammar and module-namespace
  mechanic transfer unchanged. The new fact in 050 is the
  *concrete* path target — `std::io::stdin` — not the path shape
  itself.
- **Lesson 029 (gloss only — not a strict prerequisite)** —
  glossed the underscore-prefix convention on binding names with
  the example `let _name: () = ();`. Lesson 050 reuses the same
  gloss with `let _stdin = std::io::stdin();` to silence the
  unused-variable warning. The convention is still not formally
  installed; it is a one-sentence calibration in both lessons. The
  underscore-elision side probe records what happens without the
  prefix: the program still compiles (warnings are not errors),
  but rustc names `_stdin` as the recommended fix in its `help:`
  text.
- **Lessons 001, 002, 005** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name = value;` plus the literal-
  string `println!` form. Used unchanged today.

## Older supporting lessons

Lesson 041 (qualified method call — the path-grammar lineage
`Type::method(receiver, args)` that lessons 041 → 042 → 043 → 050
trace, with each cycle generalizing one piece of the surface;
not re-stated here).

Lesson 044 (`use` declarations — explicitly *not* used today;
the lesson uses the full-path form `std::io::stdin()` instead.
Naming for completeness because the Book chapter 2 source the
lesson cites uses `use std::io;` then `io::stdin()`).

Lesson 003 (rustc-diagnostic structure — the underscore-elision
side probe's warning has the lesson-003 four-part shape; not
re-stated here).
