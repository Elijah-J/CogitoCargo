---
id: 020-function-with-parameter
move: "define `fn name(p: i32) { ... }` and call it with `name(value);` to pass an argument to a function"
main_concept: "a function definition can take *parameters* in the form `name: TYPE` between its parentheses; inside the body the parameter behaves like a `let` binding holding the value the caller passed; at the call site, `name(value);` supplies the value as an *argument*; in function signatures the type of each parameter is mandatory — calling rustc on a parameter without a type produces a parse error whose `help:` line says `if this is a parameter name, give it a type`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 008-define-and-call-function
  - 019-type-annotation-i32
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "multiple parameters" moves
  - future "function return values" moves
  - future "&T reference parameters" moves
  - future "&mut T parameters" moves
  - future "mut parameters" moves
  - future "generic parameters" moves
sources:
  - output/docs/rust/book/ch03-03-how-functions-work.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/020-function-with-parameter.rs
status: accepted
---

# Define a function that takes one typed parameter

## The Move

Define a second function whose parentheses are no longer empty:
`fn name(p: TYPE) { ... }`. The slot `p: TYPE` is one *parameter* — a
name and the type of value it will hold. Call it from `main` with
`name(value);`, where `value` is the *argument* you send. Inside the
body the parameter name refers to the argument's value, so
`println!("... {p} ...");` substitutes it. Drop the `: TYPE` and
rustc refuses with a parse error whose `help:` line literally says
`if this is a parameter name, give it a type`.

## Mental Model Delta

- Before: "I can define a second function (lesson 008), but the
  parentheses are always empty. The body cannot see any value the
  caller has. Type annotations (lesson 019) only happen between a
  `let` name and `=`."
- After: "Parentheses hold a *parameter list*. A parameter is
  `name: TYPE` — the same shape as a `let` annotation, reused in a
  new slot. Inside the body the parameter behaves like a `let`
  binding holding the value the caller supplied. *Parameter* and
  *argument* are two ends of one wire: parameter is the slot in the
  definition, argument is the value at the call. And rustc enforces
  a hard rule: in function signatures parameter types are
  *mandatory*. Forgetting `: TYPE` is a parse error, not inference."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`; silent on success.
  - Lesson 002: the body of `fn main` runs when the executable
    launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline,
    `-->` location, source excerpt with caret, and `help:` / `= note:`
    lines. Used here to read the contrast probe.
  - Lesson 008 (load-bearing): `fn name() { ... }` defines a second
    function and `name();` calls it. Lesson 008 used empty
    parentheses; this lesson fills them in on both sides.
  - Lesson 019 (load-bearing): every value has a type, and the form
    `name: TYPE` attaches one (between a `let` name and `=`); for
    integer literals the type name is `i32`. Reused inside a
    parameter list here.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    say_value(5);
    say_value(42);
}

fn say_value(n: i32) {
    println!("got n = {n}");
}
```

The second `fn` block is the lesson-008 shape with the parentheses
filled in: `say_value` declares one parameter, `n`, of type `i32`.
The Book describes its own example the same way: "The declaration of
`another_function` has one parameter named `x`. The type of `x` is
specified as `i32`."

Compile and run:

```console
$ rustc demo.rs
$ ./demo
got n = 5
got n = 42
```

Two distinct lines, one per call. Walk the first: `say_value(5);` is
the lesson-008 call shape, but with `5` between the parentheses — `5`
is the *argument*. Control transfers into `say_value`'s body; inside
that body the parameter name `n` holds the value `5`, and `{n}` in
the format string substitutes it the same way a `let`-bound name
does (lesson 005). The Book: "When we pass `5` in to
`another_function`, the `println!` macro puts `5` where the pair of
curly brackets containing `x` was in the format string." The body
ends, control returns to `main`, and the second call repeats the
trip with `42`.

Vocabulary note. The Book itself says "people tend to use the words
*parameter* and *argument* interchangeably." In this lesson keep
them distinct: *parameter* is the name in the definition (`n`),
*argument* is the value at the call (`5`).

Now the contrast. *Predict*: edit only the signature, removing the
`: i32`, so it reads `fn say_value(n) { ... }`. Will rustc compile
this? If not, which lesson-003 part of the diagnostic will pinpoint
the problem, and what will it suggest?

Edit `demo.rs` to:

```rust
fn main() {
    say_value(5);
}

fn say_value(n) {
    println!("got n = {n}");
}
```

Compile:

```console
$ rustc demo.rs
error: expected one of `:`, `@`, or `|`, found `)`
 --> demo.rs:5:15
  |
5 | fn say_value(n) {
  |               ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
5 | fn say_value(n: TypeName) {
  |               ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
5 | fn say_value(_: n) {
  |              ++
```

Read with the lesson-003 map. Headline: parse-style `error:` (no
`E####` code, just like lesson 003's `prntln` probe). Location:
`demo.rs:5:15`. Caret: a single `^` under the `)` after `n`. Two
`help:` blocks follow. The first is load-bearing here:
`if this is a parameter name, give it a type`, with the suggested
edit `fn say_value(n: TypeName) {`. That `help:` is rustc enforcing
the Book's rule: "In function signatures, you *must* declare the
type of each parameter." The second `help:` (`if this is a type,
explicitly ignore the parameter name`) addresses the opposite
reading and is out of scope for this lesson.

Restore `: i32`, recompile, and the two lines print again.

## What Changed

- You can write a function whose parentheses hold a typed parameter:
  `fn name(p: i32) { ... }`.
- You can call it with `name(value);` and the body sees `value`
  under the parameter name.
- You have two distinct words: *parameter* (the name in the
  definition) and *argument* (the value at the call site).
- You know one hard rule: in function signatures the type of each
  parameter is mandatory. Inference does not apply here. Drop the
  `: i32` and rustc emits a parse error whose `help:` line literally
  says `if this is a parameter name, give it a type`.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    announce(7);
    announce(100);
}

fn announce(n: i32) {
    println!("got n = {n}");
}
```

You run `rustc tiny.rs && ./tiny`.

- Does rustc accept the program, and what does it print?
- In `announce(7);`, which token is the *argument*? In the
  definition, which token is the *parameter name*?
- If you removed the `: i32` and recompiled, which lesson-003 part
  of the diagnostic would pin the bug, and which `help:` line would
  tell you the fix?

(Answers: yes, rustc accepts it; prints two lines, `got n = 7` then
`got n = 100`. Argument in `announce(7);` is `7`; parameter name is
`n`. Without `: i32`, the `-->` line points at the `)` in the
signature and the first `help:` line is
`if this is a parameter name, give it a type`.)

## What To Ignore For Now

This lesson installs only one idea: function definitions can take
one typed parameter, and the call site supplies an argument. Real
and deferred:

- *Multiple parameters* — `fn name(a: i32, b: i32)`, comma-separated.
  The Book's next paragraph; deferred.
- *Pass-by-reference* parameter types — `&i32`, `&mut i32`. Distinct
  mechanism; deferred.
- *Mutable parameters* — `fn name(mut n: i32)`, letting the body
  reassign the parameter. Different from `&mut`; deferred.
- *Default parameter values*. Rust does not have them; out of scope.
- *Function return values* and the `->` arrow. Separate later move.
- *Type inference for parameter types*. It does not happen — the
  Book's rule is explicit. The second `help:` block in the contrast
  (`if this is a type, explicitly ignore the parameter name`) is
  also out of scope.
- *Generic parameters* — `fn name<T>(p: T)`. Out of scope.
- *Parameter types other than `i32`*. Only `i32` here.
- *Closures and function pointers as parameters*. Out of scope.
- All previously deferred items: `mut`, shadowing, the broader
  format-string DSL, `cargo`, modules and `pub`, etc.

## Evidence

### Sources

- `output/docs/rust/book/ch03-03-how-functions-work.md`, the
  "Parameters" section (lines 55-98; the lesson stops before "When
  defining multiple parameters" on line 100). Three load-bearing
  passages:
  - Lines 57-63: "We can define functions to have *parameters*, which
    are special variables that are part of a function's signature.
    When a function has parameters, you can provide it with concrete
    values for those parameters. Technically, the concrete values are
    called *arguments*, but in casual conversation, people tend to use
    the words *parameter* and *argument* interchangeably for either
    the variables in a function's definition or the concrete values
    passed in when you call a function." — corpus source for the
    parameter-vs-argument distinction the lesson keeps explicit.
  - Lines 89-92: "The declaration of `another_function` has one
    parameter named `x`. The type of `x` is specified as `i32`. When
    we pass `5` in to `another_function`, the `println!` macro puts
    `5` where the pair of curly brackets containing `x` was in the
    format string." — corpus source for the parameter-name-and-type
    shape and for "the value the caller passes shows up where the
    parameter name appears in the body."
  - Line 94: "In function signatures, you *must* declare the type of
    each parameter." — the corpus source for the hard rule the
    contrast probe demonstrates.

  Calibration: the Book uses the function name `another_function` and
  parameter name `x`. This lesson uses `say_value` and `n` instead;
  `x` would collide with bound names from earlier lessons (e.g. lesson
  005's bindings) and dilute the "this is a parameter, not a `let`"
  point. The Book builds with `cargo run`; this lesson uses
  `rustc demo.rs` per lesson 001. Behavior is the same.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/020-function-with-parameter.rs`.
The committed file is the *working* version (with `n: i32`). The
broken contrast (signature without `: i32`) is documented as a second
run inside this Evidence section, not as a separate `.rs` file.

Probe transcript, both runs in the same temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: working program with parameter typed as i32 ===
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    say_value(5);
    say_value(42);
}

fn say_value(n: i32) {
    println!("got n = {n}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
got n = 5
got n = 42
exit=0

=== RUN 2: broken contrast — parameter without a type ===
--- cat demo.rs ---
fn main() {
    say_value(5);
}

fn say_value(n) {
    println!("got n = {n}");
}
--- rustc demo.rs (capturing stderr) ---
error: expected one of `:`, `@`, or `|`, found `)`
 --> demo.rs:5:15
  |
5 | fn say_value(n) {
  |               ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
5 | fn say_value(n: TypeName) {
  |               ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
5 | fn say_value(_: n) {
  |              ++

error: aborting due to 1 previous error

exit=1
--- ls after broken compile ---
demo
demo.rs
```

Notes:

- Run 1 (working): rustc exits 0, silent. `./demo` prints two lines,
  `got n = 5` and `got n = 42`, in that order. The two lines come
  from the two call sites in `main` running in source order
  (lesson 004 / lesson 008) with their respective arguments showing
  up under the parameter name `n` inside `say_value`. That ordering
  and the two distinct printed values are the load-bearing
  observation for "the argument's value becomes available inside the
  body under the parameter name."
- Run 2 (broken): rustc exits 1 with a parse-style `error:` (no
  `[E####]` code, just like lesson 003's `prntln` probe). The `-->`
  points at column 15 of line 5, which is exactly the `)` after `n`
  in the signature. The first `help:` block is the load-bearing one:
  `if this is a parameter name, give it a type`, with the proposed
  edit `fn say_value(n: TypeName) {`. The second `help:` block
  (`if this is a type, explicitly ignore the parameter name`) is
  noted in the lesson and explicitly deferred. No `--explain` trailer
  appears, consistent with lesson 003's rule that only errors with an
  `E####` code carry one.
- The `demo` in `ls after broken compile` is the executable from
  Run 1; Run 2 did not produce a new one (lesson 001's compile-then-
  run two-step).
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — four-part map
  (headline, `-->` location, source excerpt with caret, help/note
  lines) used here to read the parse error in the contrast probe. Not
  re-taught.
- `008-define-and-call-function` (accepted, load-bearing) — define a
  second function with `fn name() { ... }` and call it from `main`
  with `name();`. Lesson 008 used empty `()`; this lesson fills the
  parentheses on both sides (one parameter in the definition, one
  argument at the call). Lesson 008's "control transfers in and
  comes back" is reused without re-deriving.
- `019-type-annotation-i32` (accepted, load-bearing) — every Rust
  value has a type; the form `name: TYPE` attaches a type to a name;
  for integer literals the default and most common type is `i32`.
  This lesson reuses the exact `name: i32` shape inside a parameter
  list rather than after a `let` name.
