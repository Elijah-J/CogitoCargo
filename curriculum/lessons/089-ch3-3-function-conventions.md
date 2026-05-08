---
id: 089-ch3-3-function-conventions
status: accepted
evidence: ../evidence/089-ch3-3-function-conventions.md
---

# Two Ch3-3 conventions: `snake_case` names and free definition order

## The Move

Lesson 008 installed `fn name() { ... }` and `name();`. It used one
specific layout — `fn say_hi` defined *below* `main` — and named the
general definition-order rule as deferred. Today closes that
deferral, and adds the *naming convention* the Book teaches in the
same passage.

Inside one `.rs` file, write two `fn` blocks. Make the second name
*snake_case* — all lowercase, words separated by underscores.
Define it *after* `main` in the source. Call it from inside `main`:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

`rustc demo.rs` exits 0. `./demo` prints `Hello, world!` then
`Another function.`. Both Ch3-3 conventions are witnessed in this
one example: `another_function` is snake_case, and it sits *after*
`main` in the source yet `main` calls it successfully.

## Mental Model Delta

- *Before:* "I have `fn main()` (lesson 002) and `fn say_hi() { ... }`
  + `say_hi();` (lesson 008). Lesson 008 noted the definition-order
  rule was not yet taught and only used one layout. I have no rule
  for what to *name* a function. Many languages I have heard of
  require callees to be defined before callers; I implicitly assume
  Rust does too."
- *After:* "Two Ch3-3 conventions, both from one Book passage
  (lines 9-36). (1) The conventional naming style for function and
  variable names is *snake_case*: lowercase letters, words separated
  by underscores — `another_function`, `my_count`, `read_line`. The
  same style applies to lesson-005 `let` bindings, not just function
  names. (2) Function definition *order* in the source file does
  not matter. A function defined *after* `main` is callable from
  inside `main`. The Book: `Rust doesn't care where you define your
  functions, only that they're defined somewhere in a scope that
  can be seen by the caller.` Both conventions are real Ch3-3 rules,
  taught together with one example."

## Prerequisites

- Installed concepts:
  - Lesson 008 (load-bearing): `fn name() { ... }` defines a
    function; `name();` calls it. Today extends 008's `name` slot
    with the snake_case rule for what *kind of name* to use, and
    closes 008's deferral on definition order.
  - Lesson 002 (cited): `fn main` is the entry point and runs when
    the executable launches.
  - Lesson 011 (cited): `println!("...")` for visible output.
  - Lesson 005 (cited): `let name = value;` introduces a variable;
    the snake_case rule covers variable names too.
  - Lesson 069 (cited): `rustc` warnings are a separate category
    from errors. The CamelCase contrast in the evidence appendix
    fires a *warning*, not an error — a lesson-069 diagnostic.
  - Lesson 075 (cited): `const NAME: TYPE = value;` uses
    `SCREAMING_SNAKE_CASE`. Different convention for a different
    kind of name; today's snake_case is for `fn` and `let` names.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, save `demo.rs` containing exactly the
*The Move* program above. Compile and run:

```console
$ rustc demo.rs
$ ./demo
Hello, world!
Another function.
```

`rustc` is silent (no warnings, no errors): both conventions are
satisfied. Walk the output: `main` starts; the first `println!`
prints `Hello, world!`; `another_function();` transfers control
into `another_function`'s body; that body's `println!` prints
`Another function.`; control returns to `main`; `main` ends. The
*order* in the printed output reflects the *call order* inside
`main`, not the *definition order* in the source file.

Now the contrast: convention vs requirement. In the same directory,
save `camel.rs` with the function renamed to *CamelCase*:

```rust
fn main() {
    println!("Hello, world!");

    AnotherFunction();
}

fn AnotherFunction() {
    println!("Another function.");
}
```

Compile it. Read with the lesson 069 category map; full transcript
in the evidence appendix:

```text
warning: function `AnotherFunction` should have a snake case name
 --> camel.rs:7:4
  |
7 | fn AnotherFunction() {
  |    ^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `another_function`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: 1 warning emitted
```

The headline starts with `warning:`, not `error:`. `rustc` exits 0
*and* writes the executable; `./camel` runs and prints the same two
lines as `demo`. The convention is enforced as a warning, not a
requirement: code violating snake_case *compiles and runs*, but
`rustc` itself names the convention and offers the fix
(`help: convert the identifier to snake case: another_function`).

## What Changed

- The conventional style for function and variable names is
  *snake_case*: lowercase letters, words separated by underscores.
  `another_function`, `my_count`, `read_line` are snake_case.
  `AnotherFunction`, `myCount`, `READLINE` are not.
- A function defined *later* in the source file is callable from a
  function defined *earlier*. `Rust doesn't care where you define
  your functions, only that they're defined somewhere in a scope
  that can be seen by the caller.`
- Convention vs requirement: snake_case is enforced as a *warning*
  (the `non_snake_case` lint, on by default), not an error. Code
  violating the convention compiles and runs; `rustc` nudges you in
  the diagnostic.
- The `let` bindings from lesson 005 follow the same naming style
  as `fn` names: lowercase with underscores. The same lint covers
  both — its scope is "variables, methods, functions, lifetime
  parameters and modules" per the rustc lint listing.
- Different *kinds* of names use different conventions. Lesson 075
  installed `SCREAMING_SNAKE_CASE` for `const`. Today's rule is
  specifically for `fn` and `let` names.

## Check Yourself

(a) Which of these is snake_case? `read_line`, `readLine`,
`ReadLine`, `READ_LINE`.

(b) You write `tiny.rs`:

```rust
fn main() {
    println!("a");
    helper();
    println!("c");
}

fn helper() {
    println!("b");
}
```

`helper` is defined *below* `main` but called *from* `main`. Does
`rustc tiny.rs && ./tiny` compile and run? In what order do the
three lines print?

(c) If you rename `helper` to `DoTheThing` (CamelCase) and update
the call site to match, does `rustc` accept the program?

(Answers: (a) `read_line`. The all-caps form `READ_LINE` is
*SCREAMING_SNAKE_CASE* — different convention used for `const`
names from lesson 075. (b) Yes; prints `a`, `b`, `c`. Definition
order is free. (c) Yes — with a *warning*, not an error
(lesson 069). The program compiles and runs; the `warning:` headline
says `function \`DoTheThing\` should have a snake case name` and
the inline `help:` shows `do_the_thing` as the snake_case fix.)

## What To Ignore For Now

Today installs only the snake_case convention for `fn` and `let`
names plus the free definition-order rule. Each of the following
is real and deferred:

- *The full set of Rust naming conventions.* `CamelCase` for type
  names (structs, enums); `SCREAMING_SNAKE_CASE` for `const`
  (lesson 075); `'lowercase` for lifetime names.
- *Lint configuration.* `#[allow(non_snake_case)]`,
  `#[warn(...)]`, `#[deny(...)]`. Default is *warn* (lesson 069);
  configurable lint levels are a future move.
- *`pub` and visibility.* The Book's "in scope that can be seen by
  the caller" names the rule but does not unpack module visibility.
- *Module-level scoping.* Definition order is free *within the
  same module*; cross-module is more complex. Today only addresses
  one-file packages.
- *Recursion.* A function calling itself is the same rule
  (definition order is free) at a tighter loop. Its own move.
- *Why Rust allows free definition order operationally* — the
  compiler's two-pass approach (collect items first, then resolve
  bodies). Mechanism, not learner-facing rule.

## Evidence

See `../evidence/089-ch3-3-function-conventions.md`.
