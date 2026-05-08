# Evidence — 040-method-call-syntax

Audit appendix for `lessons/040-method-call-syntax.md`. Holds the
corpus-quote map, the toolchain string, the full working and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/040-method-call-syntax.rs`); the broken-contrast `.rs`
  is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/reference/expressions/method-call-expr.md`

The Reference page for method-call expressions. Three load-bearing
spans.

Syntax (lines 6-10):

> [MethodCallExpression] → [Expression] . [PathExprSegment] (
> [CallParams]? )

This is the formal grammar statement: a method-call expression is an
expression (the receiver), a single dot, a path-expression segment
(the method name), and a parenthesized — possibly empty — call-params
list. `n.abs()` matches this with `n` as the Expression, `abs` as the
PathExprSegment, and an empty CallParams list.

Intro (lines 12-14):

> A *method call* consists of an expression (the *receiver*) followed
> by a single dot, an expression path segment, and a parenthesized
> expression-list.

This is the prose statement of the same shape, and is the source for
the lesson's "receiver / dot / method name / argument list" labels —
the *receiver* term in particular comes verbatim from this paragraph.

Resolution sketch (lines 16-18):

> Method calls are resolved to associated [methods] on specific
> traits, either statically dispatching to a method if the exact
> `self`-type of the left-hand-side is known, or dynamically
> dispatching if the left-hand-side expression is an indirect [trait
> object].

Cited only as the corpus statement that methods are *associated with a
specific type* (the lesson's main concept). The lesson explicitly
defers the trait-dispatch and trait-object subtopics to "What To
Ignore For Now."

### `output/docs/rust/std/primitive.i32.md`

The std-library page for the `i32` primitive type. Header (lines 1-9):

> # Primitive Type i32
> 1.0.0
> The 32-bit signed integer type.
> ## Implementations
> ### impl i32

This is the corpus statement that `i32` is "the 32-bit signed integer
type" (the lesson does not re-state this — lesson 019 already did) and
that methods on `i32` live in an `impl i32` block.

`abs` method signature (line 2511):

> #### pub const fn abs(self) -> i32

This is the canonical signature for `i32::abs`: takes `self` (the
receiver), no extra arguments, returns `i32`. This is the corpus
basis for the lesson's claims that (a) `n.abs()` has empty parens
because there are no extra arguments and (b) the call expression
produces an `i32` that fits on the right of `let m: i32 = ...;`.

`abs` description (lines 2513-2533):

> Computes the absolute value of `self`.
>
> ##### Overflow behavior
>
> The absolute value of `i32::MIN` cannot be represented as an `i32`,
> and attempting to calculate it will cause an overflow. This means
> that code in debug mode will trigger a panic on this case and
> optimized code will return `i32::MIN` without a panic. ...
>
> ##### Examples
>
> assert_eq!(10i32.abs(), 10);
> assert_eq!((-10i32).abs(), 10);

The first sentence is the corpus statement of what `abs` does
("computes the absolute value of `self`"). The Examples block is the
same dot-form `value.abs()` syntax the lesson uses, with `(-10i32)`
showing that a negative `i32` produces a positive one — directly
parallels the lesson's `n = -7` → `m = 7` observation. The Overflow-
behavior paragraph is the corpus basis for the deferral of
"runtime panic on `i32::MIN.abs()`."

### `output/docs/rust/book/ch05-03-method-syntax.md`

The Book chapter that introduces method syntax. Cited as the prose
introduction of the dot-form, even though its main example uses
structs (out of scope for this lesson). Two load-bearing spans.

Lines 4-11:

> Methods are similar to functions: We declare them with the `fn`
> keyword and a name, they can have parameters and a return value,
> and they contain some code that's run when the method is called
> from somewhere else. Unlike functions, methods are defined within
> the context of a struct (or an enum or a trait object ...), and
> their first parameter is always `self`, which represents the
> instance of the struct the method is being called on.

Cited for two claims: methods are similar to functions (declared with
`fn`, can have parameters and return values, run when called) — i.e.
the lesson's "different syntax for calling a function, no new control-
flow" framing — and methods are "defined within the context of a
[type]" (the lesson's "associated with a type" framing). The Book's
"struct" specialization is broader in the Reference (which covers
`impl T` for any type `T`, including primitives like `i32`).

Lines 58-59:

> The method syntax goes after an instance: We add a dot followed by
> the method name, parentheses, and any arguments.

The Book's plain-English statement of the call shape. Identical to the
Reference's grammar in spirit; the lesson's "receiver, dot, method
name, parenthesized argument list" rephrases it.

Calibration:

- The Book's example is `rect1.area()` on a `Rectangle` struct, not
  `n.abs()` on an `i32`. The lesson uses an `i32` receiver because
  structs are not yet installed and the dot-form is identical
  regardless of receiver type. The Book's chapter 5.3 itself does not
  cover methods on primitive types; the std `primitive.i32.md` page
  carries that proof.
- The Book introduces `&self` (a reference receiver) in the same
  passage. The lesson explicitly defers `&self` to "What To Ignore
  For Now"; `i32::abs(self)` takes ownership rather than a reference,
  which avoids the `&` complication for this lesson.

### `output/docs/rust/error_codes/E0425.md`

The error-code explainer for E0425, "an unresolved name was used."
Already cited in lessons 005 and 008. Reused here only for the family
connection — the broken-contrast probe fires E0425 because `abs` is
not a free function in scope. The corpus statement is the page's
opening sentence:

> An unresolved name was used.

with examples `something_that_doesnt_exist::foo` and `unknown_variable`
(lines 11, 24). The lesson does not re-explain E0425; it cites lessons
005 and 008 for that.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/040-method-call-syntax.rs`.
Identical source to the Try It block.

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
    let n: i32 = -7;
    let m: i32 = n.abs();
    println!("n = {n}, m = {m}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
n = -7, m = 7
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line: `n = -7, m = 7`. The `m = 7` value
  is `i32::abs(-7)`, matching the std-page Examples block's
  `assert_eq!((-10i32).abs(), 10);` pattern.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Same source as the working probe, with the third line changed from
`let m: i32 = n.abs();` to `let m: i32 = abs(n);` (free-function form
instead of method-call form). Not committed; the transcript below is
the artifact. Captured 2026-05-07 in a fresh `mktemp -d` (filename
`broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn main() {
    let n: i32 = -7;
    let m: i32 = abs(n);
    println!("m = {m}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0425]: cannot find function `abs` in this scope
 --> broken.rs:3:18
  |
3 |     let m: i32 = abs(n);
  |                  ^^^ not found in this scope
  |
help: use the `.` operator to call the method `abs` on `i32`
  |
3 -     let m: i32 = abs(n);
3 +     let m: i32 = n.abs();
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline reads `error[E0425]: cannot find function ` then a
  backtick-quoted `abs`, then ` in this scope`. Same E-code lessons
  005 and 008 installed (005 for missing values, 008 for missing
  free functions). The headline word "function" matches lesson 008's
  headline shape.
- The diagnostic has the four lesson-003 parts: headline + `-->`
  location (`broken.rs:3:18`) + source excerpt with `^^^` caret under
  `abs` + `help:` block. The `--explain E0425` trailer is also
  present, consistent with lesson 003's rule that headlines carrying
  an `[E####]` code emit the trailer.
- The `help:` line reads literally:
  `help: use the `, then a backtick-quoted `.`, then
  ` operator to call the method `, then a backtick-quoted `abs`,
  then ` on `, then a backtick-quoted `i32`. This is rustc's own
  runtime diagnostic text — captured here as part of the broken-
  contrast probe, *not* a corpus quotation. As probe evidence it
  directly supports the lesson's contrastive claim that `n.abs()`
  works while `abs(n)` fails. The independent corpus grounding for
  the contrastive claim is `output/docs/rust/error_codes/E0425.md`'s
  "An unresolved name was used" line — that explainer page covers
  the E-code; this captured `help:` text covers what rustc actually
  prints in this specific instance. The source-diff underneath
  replaces `abs(n)` with `n.abs()`.
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`, no `broken` binary.
- The lesson's body claim "writing `abs(n)` rejects with E0425 *and*
  rustc tells you to use the dot" maps directly to the headline +
  help block above.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with the dot it works, without the dot it
fails"). Capturing rustc's exact `help:` text was particularly load-
bearing because that text — quoted in the lesson's Try It section —
is rustc's own runtime statement of the fix. It is *probe* evidence,
not corpus evidence. The corpus-level grounding for the contrastive
claim is `output/docs/rust/error_codes/E0425.md` (the explainer page
for the E-code that fires) plus the Reference's method-call grammar
(`output/docs/rust/reference/expressions/method-call-expr.md`); this
captured `help:` text is the live transcript that ties those corpus
sources to the lesson's specific `abs(n)` → `n.abs()` example.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 040. Older supporting lessons are mentioned
above by number only.

- **Lesson 008 (load-bearing)** — `name();` is a free-function call: a
  bare function name followed by an argument list in parens. This is
  the call shape lesson 040 *contrasts* with. Lesson 008 also
  installed the rule that a missing free function fires E0425
  ("cannot find function in this scope"); the broken-contrast probe
  reuses that rule unchanged.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `help:` lines.
  Lesson 040's broken-contrast walk uses that map without re-teaching
  it.
- **Lesson 021** — `name(args)` is an *expression* whose value is
  what the function returned, so it fits on the right of `let` if the
  return type matches. Lesson 040 reuses that rule unchanged — the
  receiver `n` is also an expression, and `n.abs()` is a call
  expression of type `i32`, so it slots into `let m: i32 = ...;`.
- **Lessons 020 and 005, 019** — typed parameters and `let name: TYPE
  = value;` annotations. Lesson 040 has no new ground here; the probe
  uses `let n: i32 = -7;` and `let m: i32 = n.abs();`.
- **Lessons 001, 002** — `rustc file.rs` then `./name`; `fn main` is
  the entry point. Used unchanged.

## Older supporting lessons

Lessons 005, 008 (E0425 family connection only — the lesson body
mentions them by number to flag the same E-code, no claim from those
lessons is re-stated here that is not already restated under "Direct
prerequisite claims").
