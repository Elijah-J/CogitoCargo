---
id: 036-multiple-parameters
move: "define a function with more than one parameter — `fn add(a: i32, b: i32) -> i32 { a + b }` — and call it with the matching number of comma-separated arguments — `add(2, 3)`"
main_concept: "a function signature can take multiple parameters separated by `,`, each in lesson 020's `name: TYPE` shape; at the call site you supply that many comma-separated argument values, matched positionally; rustc enforces *arity* (number of arguments == number of parameters) and a mismatch is a new E-code, **E0061** `this function takes N arguments but M argument(s) was supplied`, with a `note: function defined here` secondary `-->` (a second source location, like lesson 003's `prntln` example) cross-referencing the definition; Rust has no default arguments, no named arguments, no variadic functions in safe code"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 008-define-and-call-function
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
  - 020-function-with-parameter
  - 021-function-return-value
  - 025-implicit-return
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "mixed-type parameters (e.g. `fn f(a: i32, b: f64)`)" moves
  - future "parameters by reference `fn f(p: &T)`" moves
  - future "mutable parameters `fn f(mut p: i32)`" moves
  - future "passing a tuple `fn f(p: (i32, i32))` vs. multiple parameters" moves
  - future "function pointers and closures with multiple parameters" moves
  - future "the `--explain E0061` flag in detail" moves
sources:
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/error_codes/E0061.md
  - output/docs/rust/reference/items/functions.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/036-multiple-parameters.rs
status: accepted
---

# Define a function with more than one parameter

## The Move

Take lesson 020's single-parameter signature and add a second parameter
after a `,`. Inside the parentheses you can list any number of
parameters, each in the same `name: TYPE` shape lesson 020 installed,
separated by commas:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

At the call site, supply the same number of values, also separated by
commas, in the same order: `add(2, 3)`. Inside the body, `a` holds the
first argument and `b` holds the second. rustc enforces *arity* (the
number of arguments matching the number of parameters): drop one
argument and rustc rejects the call with a new error code, **E0061**
`this function takes N arguments but M argument(s) was supplied`.

## Mental Model Delta

- Before: "A function takes at most one parameter (lesson 020). The
  parameter list is `name: TYPE` between the parentheses."
- After: "The parameter list is a comma-separated list of `name: TYPE`
  slots — one, two, or as many as needed. The call site is symmetric:
  the same number of comma-separated argument values, matched
  positionally (first argument to first parameter, second to second).
  rustc cross-checks the count: too few or too many arguments fails
  with E0061, and the diagnostic shows you both the call site (primary
  `-->`) AND the function definition (secondary `-->` under
  `note: function defined here`). Rust has no default arguments and
  no named arguments — calls match positionally only."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`, silent on
    success; the body of `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline,
    `-->` location, source excerpt with caret, and optional
    `help:` / `note:` lines. The broken-contrast walk decodes a new
    E-code, **E0061**, using exactly this map. The lesson also reuses
    lesson 003's observation that a single diagnostic can have more
    than one `-->` line: the first marks the bug location, later ones
    belong to `note:` context.
  - Lesson 005: `let name: TYPE = value;` binds a value to a name. The
    probe binds `let result: i32 = add(2, 3);`.
  - Lesson 008: define a second function with `fn name() { ... }` and
    call it with `name();` (zero-arg case).
  - Lesson 009: `+` between two integer values produces a new integer
    value. Used inside `add` as `a + b`.
  - Lesson 019: every value has a type; `name: TYPE` attaches one.
    Reused inside the parameter list, twice.
  - Lesson 020 (load-bearing): `fn name(p: i32) { ... }` plus
    `name(value);`. **This lesson generalizes that single-parameter
    shape to multiple parameters.** Lesson 020 also installs the
    parameter/argument vocabulary; this lesson reuses it.
  - Lesson 021: `-> RTYPE` declares a return type and the call
    expression carries the returned value to the right of `let`.
  - Lesson 025 (load-bearing): a function body is a block; its tail
    expression (no `return`, no `;`) is the return value. The probe's
    body is the bare `a + b`, the lesson-025 form.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result: i32 = add(2, 3);
    println!("result = {result}");
}
```

The signature `fn add(a: i32, b: i32) -> i32` is the only new shape.
Read it left-to-right: function name `add`, parameter list with two
slots `a: i32` and `b: i32` separated by a comma, return type `-> i32`.
The body is the lesson-025 implicit-return form: a bare `a + b` with no
`return` and no `;`. The call site `add(2, 3)` supplies two arguments
separated by a comma; lesson 021 carries the returned value to the
right of `let`.

The Book describes the same shape directly:

> When defining multiple parameters, separate the parameter
> declarations with commas

— and shows its own two-parameter example
`fn print_labeled_measurement(value: i32, unit_label: char)`. The
formal Rust Reference grammar for `FunctionParameters` says exactly
the same thing in railroad form: `FunctionParam ( , FunctionParam )*`,
i.e. one parameter followed by zero or more `, FunctionParam`
repetitions.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
result = 5
```

Walk it. `add(2, 3)` is a call with two arguments. Positional matching:
`2` binds to the first parameter `a`, `3` binds to the second `b`.
Inside the body, `a + b` evaluates to `2 + 3 = 5`. Per lesson 025 the
tail expression's value is the function's return value. Per lesson 021
the call expression carries that `5` to the right of the `let`, and
the `println!` prints `result = 5`.

Now the contrast. *Predict*: edit the call site to `add(2)` — one
argument instead of two. Will rustc compile this? If not, what E-code
will fire, and which lesson-003 parts of the diagnostic will pinpoint
the problem?

Edit `demo.rs` so the call site reads `let result: i32 = add(2);` and
recompile:

```console
$ rustc demo.rs
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> broken.rs:6:23
  |
6 |     let result: i32 = add(2);
  |                       ^^^--- argument #2 of type `i32` is missing
  |
note: function defined here
 --> broken.rs:1:4
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ^^^         ------
help: provide the argument
  |
6 |     let result: i32 = add(2, /* i32 */);
  |                            +++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0061`.
```

(Full transcript captured in `## Evidence`.)

Read with the lesson-003 map.

- **Headline**: `error[E0061]: this function takes 2 arguments but 1
  argument was supplied`. A new error code — different from E0308,
  E0004, E0277, E0425. The plain-English message names the arity
  mismatch directly.
- **Primary `-->`**: `broken.rs:6:23`, the call site. The source
  excerpt underlines `add` with `^^^` and the empty slot where the
  missing argument should go with `---`, with the inline annotation
  `argument #2 of type `i32` is missing`.
- **`note:` plus secondary `-->`**: `note: function defined here`
  followed by `--> broken.rs:1:4` — a *second source location* in the
  same diagnostic, exactly like the second `-->` lesson 003 saw on the
  `prntln` example. The `note:` prefix marks it as supplementary
  context, not a separate error. The source excerpt under the
  secondary `-->` reprints the function signature and underlines the
  function name `add` with `^^^` and the parameter list with `------`,
  telling you where the function takes its parameters.
- **`help:` block**: `help: provide the argument`, followed by a
  source-diff suggestion `add(2, /* i32 */);` with `+++` markers
  showing what would be added. The `/* i32 */` is rustc's
  block-comment placeholder (lesson 018's syntax) for "an `i32` value
  goes here, and rustc has shown you the type"; not a real fix but a
  guidance shape.
- **`--explain` trailer**: `For more information about this error,
  try `rustc --explain E0061`.` — present because the headline carried
  an `[E####]` code, exactly the rule lesson 003 installed.

The cross-referencing diagnostic — primary at the call, secondary at
the definition, tied together by `note: function defined here` — is the
new pattern. When a function-call error happens, rustc shows you both
ends of the wire so you can compare them.

E0061 fires for *too many* arguments too: `add(2, 3, 4)` produces a
symmetric `this function takes 2 arguments but 3 arguments were
supplied`. Restore the call site to `add(2, 3)` and the program prints
`result = 5` again.

## What Changed

- You can write a function whose parameter list has more than one slot:
  `fn name(a: TYPE, b: TYPE, ...) { ... }`, comma-separated.
- You can call it with the matching number of comma-separated argument
  values, matched positionally to the parameters in source order.
- You know one new E-code, **E0061**, fired when the arity is wrong
  (too few or too many arguments). The diagnostic carries a
  `note: function defined here` with a *secondary* `-->` location,
  cross-referencing the call and the definition.
- You know three things Rust does *not* have: default arguments,
  named/keyword arguments, and variadic functions in safe code.
  Calls match positionally on count and order.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let answer: i32 = sub(10, 3);
    println!("answer = {answer}");
}
```

You run `rustc tiny.rs && ./tiny`.

- Does rustc accept the program, and what does the executable print?
- In the call `sub(10, 3)`, which value binds to parameter `a` and
  which binds to `b`?
- If you changed the call to `sub(10)` and recompiled, which E-code
  would the headline carry, and what would the `note:` line plus its
  secondary `-->` point at?

(Answers: yes; prints `answer = 7`. `10` binds to `a` (first
parameter), `3` binds to `b` (second). With one argument: headline
`error[E0061]: this function takes 2 arguments but 1 argument was
supplied`; the `note: function defined here` with its secondary `-->`
points at the line of the `sub` signature, with carets under the
function name and dashes under the parameter list.)

## What To Ignore For Now

This lesson installs only one idea: a function signature can have
multiple parameters separated by `,`, and the call supplies the
matching number of comma-separated arguments matched positionally.
Real and deferred:

- *Mixed-type parameters* — `fn f(a: i32, b: f64)`. Mechanically
  identical to same-type; the Book's own example
  (`fn print_labeled_measurement(value: i32, unit_label: char)`) has
  mixed types. Future move when an example actually benefits from the
  mix.
- *Parameters by reference* — `fn f(p: &i32)`. Distinct mechanism;
  needs the `&T` reference type. Future move.
- *Mutable parameters* — `fn f(mut p: i32)`, allowing the body to
  reassign the parameter. Different from `&mut`. Future move.
- *Default argument values*. **Rust does not have them.** Some other
  languages (Python, C++, Kotlin) let you write `fn f(a: i32, b: i32 = 0)`
  and call `f(1)` to use the default; Rust's E0061 transcript shows
  rustc rejects exactly that. The E0061 explainer page states this
  explicitly: "Rust does not have a notion of optional function
  arguments or variadic functions (except for its C-FFI)." Workarounds
  exist (builders, `Option<T>` parameters) but are out of scope.
- *Variadic functions* — variable-length argument lists, like C's
  `printf`. Same E0061 explainer sentence rules them out for safe Rust;
  they exist only for FFI. Future move under the FFI topic.
- *Named / keyword arguments* — `add(a: 2, b: 3)`. **Rust does not
  have them.** Arguments match positionally only. Out of scope.
- *Passing a tuple as one argument* — `add((2, 3))` with the signature
  `fn add(p: (i32, i32))`. A different mechanism (one parameter of
  tuple type), not the multi-parameter form. Future move.
- *Closures and function pointers with multiple parameters* — same
  comma-separated shape inside `|...|` or `fn(...)` types. Future move.
- *The Book's `print_labeled_measurement` example uses `char`* — a new
  scalar type lesson 036 has not installed. Mention only as a Book
  cross-reference; do not introduce `char` here.
- All previously deferred items.

## Evidence

### Sources

- `output/docs/rust/book/ch03-03-how-functions-work.md`, the
  "Parameters" section. Lesson 020 used the lines 55-98 single-parameter
  passages; this lesson uses the *next* paragraph, lines 100-128.
  Load-bearing direct quote, lines 100-101:

  > When defining multiple parameters, separate the parameter
  > declarations with commas

  The Book then shows its own two-parameter example
  `fn print_labeled_measurement(value: i32, unit_label: char) { ... }`
  called with `print_labeled_measurement(5, 'h');`, building the
  output line `The measurement is: 5h`. That example (lines 105-128)
  is the corpus source for "two parameters, comma-separated, called
  with two comma-separated arguments matched positionally."

  Calibration:
  - The Book's example uses mixed types (`i32` and `char`); this
    lesson uses two `i32` parameters because `char` is not yet
    installed and mixing it in would dilute the move.
  - The Book builds with `cargo run`; this lesson uses `rustc demo.rs`
    per lesson 001. Behavior is identical.
  - This lesson's function name is `add` rather than the Book's
    `print_labeled_measurement` — `add` is shorter and the body uses
    lesson-009 arithmetic + lesson-025 implicit return, both already
    installed, so the only new piece the learner perceives is the
    parameter list.

- `output/docs/rust/error_codes/E0061.md`, the canonical explainer
  page for E0061. Two load-bearing direct quotes:

  > An invalid number of arguments was passed when calling a function.

  > The number of arguments passed to a function must match the number
  > of arguments specified in the function signature.

  And, on the deferred items the lesson lists explicitly:

  > Note that Rust does not have a notion of optional function
  > arguments or variadic functions (except for its C-FFI).

  The page's example signature `fn f(a: u16, b: &str) {}` plus its
  one-sentence claim "Must always be called with exactly two
  arguments, e.g., `f(2, "test")`" is the corpus statement of the
  arity rule this lesson installs.

- `output/docs/rust/reference/items/functions.md`, the
  `[FunctionParameters]` grammar (lines 22-24):

  > [FunctionParameters] →
  >   [SelfParam] ,?
  > | ( [SelfParam] , )? [FunctionParam] ( , [FunctionParam] )* ,?

  The right-hand side `FunctionParam ( , FunctionParam )*` is the
  formal grammar statement of the comma-separated parameter-list shape
  the lesson teaches. (The `SelfParam` alternative belongs to
  associated functions / methods on types and is out of scope; the
  trailing optional `,?` permits a trailing comma in the list.) The
  Reference also specifies the call shape under "Function parameters"
  via `FunctionParam`'s pattern + `:` + `Type` form.

### Probes

Two probes were captured. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/036-multiple-parameters.rs`.
The broken-contrast probe is *not* committed under `observations/`;
its transcript is reproduced verbatim below.

Both probes were run in temp directories created with `mktemp -d` and
removed at the end.

#### Working probe

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
demo.rs
--- cat demo.rs ---
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result: i32 = add(2, 3);
    println!("result = {result}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
result = 5
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The single output line is `result = 5`. The `5` is the value of
  `a + b` with `a` holding `2` and `b` holding `3`. That value reached
  the right-hand side of the `let` in `main` through the call
  expression `add(2, 3)` (lesson 021 carrying the returned value), and
  the `println!` printed it.
- The two arguments `2` and `3` matched the two parameters `a` and `b`
  positionally; the lesson's body walk relies on no other observation.

#### Broken-contrast probe

Same source as the working probe, with the call site changed from
`add(2, 3)` to `add(2)` (one argument instead of two). Not committed;
the transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result: i32 = add(2);
    println!("result = {result}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> broken.rs:6:23
  |
6 |     let result: i32 = add(2);
  |                       ^^^--- argument #2 of type `i32` is missing
  |
note: function defined here
 --> broken.rs:1:4
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ^^^         ------
help: provide the argument
  |
6 |     let result: i32 = add(2, /* i32 */);
  |                            +++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0061`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0061]: this function takes 2 arguments but 1
  argument was supplied` carries the code `E0061` and the
  `--explain E0061` trailer is also present, consistent with lesson
  003's rule.
- The diagnostic has *two* `-->` lines. The first
  (`broken.rs:6:23`) is the call site; the second
  (`broken.rs:1:4`, prefixed with `note: function defined here`) is the
  function signature. This is the same dual-`-->` pattern lesson 003
  observed on the `prntln` example, applied here to a function-call
  cross-reference. The `note:` prefix marks the secondary `-->` as
  context, not a second error.
- Inside the primary source excerpt, `^^^` underlines `add` and `---`
  underlines the empty slot inside the parentheses where the missing
  argument should go; the inline annotation reads `argument #2 of
  type `i32` is missing`. Inside the secondary source excerpt, `^^^`
  underlines `add` and `------` underlines the parameter list `a: i32,
  b: i32` (more precisely, the span starting after `add` through the
  closing `)`).
- The `help: provide the argument` block shows a source-diff
  suggestion `add(2, /* i32 */);` with `+++++++++++` markers under the
  inserted text. The `/* i32 */` is a block comment (lesson 018's
  syntax), used here as a placeholder hint for an `i32` value; rustc
  is not claiming this is a real fix, just naming the type and shape
  of what would go there.
- Exit code: 1. No executable was produced.
- Calibration: a separate try with `add(2, 3, 4)` (one argument too
  many) produces the symmetric headline `this function takes 2
  arguments but 3 arguments were supplied`, same E-code, same dual-
  `-->` cross-reference. Not transcribed in full; cited in the
  walkthrough as a prediction.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline, `-->` location, source excerpt with caret, and
  optional `help:` / `note:` lines; a single diagnostic can carry
  multiple `-->` lines, with later ones belonging to `note:` context.
  This lesson reuses both rules — the four-part map and the multiple-
  `-->` pattern — to read E0061. Not re-taught.
- `005-let-binding` (accepted) — `let name: TYPE = value;`. The probe
  binds `let result: i32 = add(2, 3);`.
- `008-define-and-call-function` (accepted) — `fn name() { ... }`
  defines a second function and `name();` calls it (zero-arg case).
- `009-arithmetic-on-integers` (accepted) — `+` between two integer
  values produces a new integer value. Used as `a + b` inside `add`.
- `019-type-annotation-i32` (accepted) — `name: TYPE` attaches a type;
  `i32` is the default integer type. Reused twice inside the parameter
  list.
- `020-function-with-parameter` (accepted, load-bearing) — the
  single-parameter shape `fn name(p: i32) { ... }` plus `name(value);`,
  including the parameter / argument vocabulary. **This lesson
  generalizes 020's parameter-list slot from one to many.** Lesson 020
  also installed the rule that parameter types are *mandatory*; that
  rule still applies to every parameter in a multi-parameter list.
- `021-function-return-value` (accepted) — `-> RTYPE` declares a
  return type; the call expression `name(args)` carries the returned
  value. The probe places `add(2, 3)` on the right of `let`.
- `025-implicit-return` (accepted, load-bearing) — a function body's
  tail expression (no `return`, no `;`) is the return value. The
  probe's body is the bare `a + b`, the lesson-025 form.
