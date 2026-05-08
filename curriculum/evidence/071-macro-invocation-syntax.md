# Evidence — 071-macro-invocation-syntax

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Sources

### `output/docs/rust/book/ch01-02-hello-world.md`

Lines 134-139, the Book's first explanation of `println!` (the Rust
program lesson 001 introduces operationally without unpacking the
trailing `!`):

> First, `println!` calls a Rust macro. If it had called a function
> instead, it would be entered as `println` (without the `!`). Rust
> macros are a way to write code that generates code to extend Rust
> syntax, and we'll discuss them in more detail in [Chapter 20]. For
> now, you just need to know that using a `!` means that you're
> calling a macro instead of a normal function and that macros don't
> always follow the same rules as functions.

This is the Book's load-bearing claim for today's lesson: the `!`
distinguishes macro invocation from function call, and removing it
means "function call" rather than "stylistically different macro
call." The lesson body quotes the middle two sentences directly.

The same passage explicitly defers the *what is a macro* question to
Chapter 20 ("we'll discuss them in more detail in Chapter 20"). The
lesson's main concept reproduces that deferral: today installs only
the syntactic distinction; expansion mechanics stay deferred.

Calibration: the Book's containing program (Listing 1-1) is exactly
the same `fn main() { println!("Hello, world!"); }` shape lessons 001
and 002 already used; no new program shape is introduced from the
Book here.

### `output/docs/rust/reference/macros.md`

Line 8, the opening definition under `# Macros`:

> The functionality and syntax of Rust can be extended with custom
> definitions called macros. They are given names, and invoked through
> a consistent syntax: `some_extension!(...)`.

This is the Reference's compact statement of the macro-invocation
syntax. The lesson body quotes the second sentence directly — it is
the source whose claim "macros are invoked through `name!(...)`" is
load-bearing.

Lines 22-25, the formal grammar under `## Macro invocation`:

> [MacroInvocation] →
>     [SimplePath] ! [DelimTokenTree]

Cited here only as confirmation that the grammar puts the `!`
literally between the name (a `SimplePath`) and the bracketed
argument body (a `DelimTokenTree`). Not quoted in the lesson body to
avoid introducing grammar metasyntax that no prior lesson installed.

Lines 26-29 give the bracketing alternatives `(...)`, `[...]`, and
`{...}` for a macro invocation; the lesson explicitly defers `[...]`
and `{...}` because no macro encountered in this run has used them
yet.

Calibration: the Reference also covers `MacroInvocationSemi` (lines
34-37) for statement-position invocations like `println!(...);` —
the form the lesson uses — and notes that "Macros may be invoked in
the following situations: Expressions and statements, Patterns,
Types, Items..." (lines 41-65). The lesson uses only the statement
position; the broader positions are listed under *What To Ignore For
Now* without quoting the Reference's enumeration directly.

### `output/docs/rust/error_codes/E0423.md`

Lines 1-6, the canonical explainer:

> An identifier was used like a function name or a value was expected
> and the identifier exists but it belongs to a different namespace.

Lines 32-43, the section that exactly covers Contrast B:

> It is common to forget the trailing `!` on macro invocations, which
> would also yield this error:
>
> ```rust
> #![allow(unused)]
> fn main() {
> println("");
> // error: expected function, tuple struct or tuple variant,
> // found macro `println`
> // did you mean `println!(...)`? (notice the trailing `!`)
> }
> ```

This is the corpus's direct grounding for the lesson's broken-B
claim that calling a macro without the `!` fires `E0423` with
"expected function, found macro" wording and a `did you mean
\`println!(...)\`?`-style suggestion. The captured probe (below)
matches this corpus example exactly.

Calibration: the corpus example shows three additional commentary
lines as `//` comments. The captured probe shows the same content
delivered as rustc diagnostic structure (headline, location, source
excerpt with caret, `help:` block). Same content, two surfaces.

### Sources NOT cited

- `output/docs/rust/reference/macros-by-example.md` — covers
  *defining* `macro_rules!` macros. The lesson explicitly defers
  macro definition; the page is not load-bearing today.
- `output/docs/rust/std/macro.println.md` — already cited by lesson
  011 for `println!`'s argument syntax. Today adds nothing about
  arguments; the page is not re-cited.
- `output/docs/rust/error_codes/E0425.md` — `cannot find value/function
  in this scope`. The Contrast A diagnostic is *uncoded* (no `[E####]`
  bracket in the headline), so it is in the same family as lesson
  003's `prntln` probe; no E-code page is the corpus warrant for it.

## Probe

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/071-macro-invocation-syntax.rs`)
is the *working* version. The two broken contrasts (`greet!()` and
`println(...)`) are documented as separate runs below, not committed
as separate `.rs` files (matching lesson 008's pattern).

### Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64
```

Same host and toolchain as accepted lessons 068, 069, 070.

### Probe 1: working program

Captured in a fresh empty temp dir created with `mktemp -d` and
removed at the end:

```text
--- working version ---
fn greet() {
    println!("hi");
}

fn main() {
    greet();
    println!("from a macro");
}
--- rustc demo.rs ---
exit=0
--- ls ---
demo
demo.rs
--- ./demo ---
hi
from a macro
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- `./demo` prints two lines: `hi` (from inside `greet`'s body),
  followed by `from a macro` (from `main`'s second statement).
- `greet()` (function call, lesson 008's form) and `println!("...")`
  (macro invocation, today's form) coexist in the same `main`
  without interfering. This is the load-bearing observation that the
  two call shapes are accepted side-by-side when their respective
  marks match the kind of name each refers to.

### Probe 2: Contrast A — `greet!()`, function called as macro

Same temp dir, edit `main` to `greet!();` (add a `!`). Recompile.

```text
--- broken-A: greet!() with bang ---
fn greet() {
    println!("hi");
}

fn main() {
    greet!();
    println!("from a macro");
}
--- rustc demo.rs ---
error: cannot find macro `greet` in this scope
 --> demo.rs:6:5
  |
6 |     greet!();
  |     ^^^^^
  |
  = note: `greet` is in scope, but it is a function, not a macro

error: aborting due to 1 previous error

exit=1
--- ls ---
demo.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error: cannot find macro \`greet\` in this scope`.
  No `[E####]` code on this one (same shape as lesson 003's
  `prntln` probe, where `cannot find macro` was uncoded too — the
  lesson groups these as the macro-name-resolution family).
- **Location**: `demo.rs:6:5` — line 6, column 5, the call site of
  `greet!()` in `main`.
- **Source excerpt with caret**: 5 carets under `greet` (the
  identifier portion only — the `!` is *not* underlined, which is
  consistent with rustc reporting the failure as a name-lookup on
  the identifier).
- **`= note:` line**: `\`greet\` is in scope, but it is a function,
  not a macro` — the corpus statement of namespace separation
  delivered directly in rustc output. This is the load-bearing
  observation for the lesson's "namespace separation" claim:
  rustc itself names the two namespaces and says the looked-up name
  exists in the other one.
- **No `help:` block**: rustc does not propose a fix for this
  direction (compare Probe 3 below, where it does). Reasonable —
  the fix could be either "drop the `!`" (today's intended fix) or
  "define a macro named `greet`" (a different intervention), and
  rustc does not guess.

Trailer: `error: aborting due to 1 previous error`. No
`For more information ... rustc --explain` line, because the headline
carries no `E####` code (lesson 003's "the trailer is optional"
claim, witnessed again).

Exit code 1; no executable produced (`ls` shows only `demo.rs`).

### Probe 3: Contrast B — `println(...)`, macro called as function

Same temp dir, restore `greet();` to its working form, then change
the next line from `println!("from a macro");` to
`println("from a macro");` (drop the `!`). Recompile.

```text
--- broken-B: println(...) without bang ---
fn greet() {
    println!("hi");
}

fn main() {
    greet();
    println("from a macro");
}
--- rustc demo.rs ---
error[E0423]: expected function, found macro `println`
 --> demo.rs:7:5
  |
7 |     println("from a macro");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
7 |     println!("from a macro");
  |            +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0423`.

exit=1
--- ls ---
demo.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0423]: expected function, found macro
  \`println\``. Coded — `[E0423]` matches the corpus page exactly
  (E0423.md lines 32-43, the `// did you mean \`println!(...)\`?`
  example in particular).
- **Location**: `demo.rs:7:5` — line 7, column 5, the call site of
  `println(...)` in `main`.
- **Source excerpt with caret**: 7 carets under `println` (the
  full identifier), with the inline annotation `not a function`
  fused onto the caret line — same compact layout lesson 003 named
  for E0601.
- **`help:` block**: `use \`!\` to invoke the macro`, followed by a
  source-diff that proposes `println!("from a macro");` (the `+`
  marker on the diff line points exactly at the column where the
  `!` should land — between the `n` of `println` and the `(`).
  The fix shown is the original working line.
- **Trailer**: `For more information about this error, try \`rustc
  --explain E0423\`.` — present because the headline carries the
  `E0423` code. Lesson 070's move applies: running the trailer
  prints the corpus content of E0423.md, which itself contains the
  `forgot the trailing \`!\`` example as the third section.

Exit code 1; no executable produced.

### Negative / contrast probes

Probes 2 and 3 *are* the negative-shape probes for the lesson's
contrastive claim. The lesson says: "with the right mark works,
without it (or with it on the wrong kind) fails." Probe 1 is the
"with the right mark works" side; Probes 2 and 3 together cover
both wrong-mark directions. No further negative probe is needed.

### Reproducibility note

The two broken probes are deterministic on rustc 1.95.0. The exact
diagnostic wording is rustc-version-specific; the *shape* (an
uncoded `cannot find macro` for Probe 2, a coded `[E0423] expected
function, found macro` with `help: use \`!\` to invoke the macro` for
Probe 3) is grounded in the corpus and stable for current rustc
releases. If a future rustc tweaks wording, the lesson's
substantive claims (namespace separation, `!` as the syntactic mark,
two call shapes) survive unchanged; only the literal headline
strings might need a refresh.

## Prior lessons

Direct prerequisites (load-bearing claims):

- `001-rustc-compile-and-run` (accepted) — installs `rustc file.rs`
  + `./name`. Lesson 001's *What To Ignore For Now* explicitly
  defers the `!`: "`println!` and the `!` after the name. The `!`
  means something specific in Rust." That deferral line is the
  pointer this lesson resolves. The Q04 deferred-queue entry is the
  same line, normalized.
- `002-fn-main-entry-point` (accepted) — `fn main` runs when the
  executable launches. Used as the container for both the
  function-call and the macro-invocation forms.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — the
  four-part diagnostic map. Both Probe 2 and Probe 3 are read with
  that map only; no new diagnostic vocabulary is installed today.
  Lesson 003 itself defers what a *macro* is, with the operational
  stand-in "the thing called with `name!(...)`" — today gives
  exactly that operational stand-in a name.
- `008-define-and-call-function` (accepted, load-bearing) —
  installs the `fn name() { ... }` definition shape and the
  function-call form `name();`. Today's working program is exactly
  lesson 008's shape; today's main mental-model delta is the
  *contrast* between that shape and the macro-invocation shape, so
  lesson 008 is the direct comparand.
- `011-println-positional-args` (accepted) — supplies operational
  fluency with `println!`. Lesson 011's *What To Ignore For Now*
  explicitly defers what "macro" means and what the trailing `!`
  signifies; today closes that loop *for the syntax*, deferring
  everything about *what a macro is* beyond the syntactic
  distinction.

Older supporting lessons (mentioned by id only, not load-bearing
for any individual claim today):

- `068-let-binding-scope`, `069-rustc-warnings`, `070-rustc-explain`
  — recent diagnostics-shape lessons on the same host and
  toolchain. Mentioned only to confirm the host environment is
  unchanged since lessons 003/008/011 were captured. Lesson 070 is
  also the lesson that justifies the `--explain E0423` trailer in
  Probe 3 being a runnable instruction rather than decoration.

No trait-related lesson is cited. The Q04 deferred-queue entry
explicitly removed the trait-machinery prerequisite for this
shallow syntax move; the lesson's *What To Ignore For Now* and this
appendix's source list both honor that constraint.
