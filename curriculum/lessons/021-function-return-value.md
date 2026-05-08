---
id: 021-function-return-value
move: "define `fn name(p: i32) -> i32 { return value; }` and use the call as a value, e.g. on the right of `let`"
main_concept: "a function can declare a return type after `->`; inside the body `return value;` sends a value back to the caller and exits the function; at the call site, `name(args)` is an expression whose value is what was returned, so it fits anywhere a value of that type can sit (most directly on the right of `let`); without `-> RTYPE`, the function returns nothing and cannot be used as a value"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 005-let-binding
  - 008-define-and-call-function
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
  - 020-function-with-parameter
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "implicit final-expression return" moves
  - future "statement vs expression" moves
  - future "unit type ()" moves
  - future "tuple returns" moves
  - future "Result/Option returns" moves
  - future "early return semantics" moves
sources:
  - output/docs/rust/book/ch03-03-how-functions-work.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/021-function-return-value.rs
status: accepted
---

# Define a function that returns a value

## The Move

Take the lesson-020 shape and add two things. First, between the
parameter list and the body, write `-> RTYPE` to declare a *return
type*. Second, inside the body, write `return value;` to send a value
back to the caller. At the call site, the expression `name(args)` no
longer has to stand alone as a statement: it carries the returned value,
so it fits anywhere a value of type `RTYPE` can sit. The most direct use
is on the right of `let`: `let result: i32 = name(args);`.

## Mental Model Delta

- Before: "I can define a function with a typed parameter (lesson 020)
  and call it as a statement, but the call site does not produce a
  value I can name or use."
- After: "A function can also *return a value*. The signature gets
  `-> RTYPE` after the parameter list — the same `: TYPE` idea from
  lesson 019, in the return slot. Inside the body, `return value;`
  sends `value` back and exits the function. At the call site,
  `name(args)` is now an *expression* whose value is what was
  returned, so it slots into the right of `let` like a literal or
  `a + b`."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`; silent on success.
  - Lesson 002: the body of `fn main` runs when the executable
    launches.
  - Lesson 005 (load-bearing): `let name: TYPE = value;` binds a name
    to a value; reused as the slot the returned value lands in.
  - Lesson 008: define a second function and call it with `name();`.
  - Lesson 009 (load-bearing): `+` between two integers produces a new
    integer; used as `n + 1` in the body.
  - Lesson 019 (load-bearing): the form `name: TYPE` attaches a type to
    a name. The `-> RTYPE` annotation here is the same idea moved to
    the return slot of a signature.
  - Lesson 020 (load-bearing): `fn name(p: i32) { ... }` plus
    `name(value);`. This lesson keeps that shape and adds a return.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn add_one(n: i32) -> i32 {
    return n + 1;
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
```

Read the signature. `fn add_one(n: i32)` is the lesson-020 shape: one
parameter `n`, typed `i32`. The new piece is `-> i32` between the
parameter list and the opening `{` — the *return type*, declaring that
`add_one` hands back an `i32`. Inside the body, `return n + 1;` is the
new statement form: `return` sends the value back to the caller and
exits the function. At the call site, `add_one(5)` no longer stands
alone — it sits on the right of a `let`, and because `add_one` returns
an `i32`, the lesson-019 annotation `: i32` on `result` matches.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
result = 6
```

Walk it. `add_one(5)` is a call — `5` is the argument. Control
transfers into `add_one`'s body with `n` holding `5`. The expression
`n + 1` evaluates to `6`. `return 6;` sends `6` back to the caller and
exits `add_one`. Control returns to the `let` line in `main` carrying
the value `6` — that is what `add_one(5)` *evaluates to* at the call
site — and the `let` binds `result` to it. The `println!` then prints
`result = 6`.

A note on style. From the Book:

> Functions can return values to the code that calls them. We don't
> name return values, but we must declare their type after an arrow
> (`->`). In Rust, the return value of the function is synonymous with
> the value of the final expression in the block of the body of a
> function. You can return early from a function by using the `return`
> keyword and specifying a value, but most functions return the last
> expression implicitly.

The Book's own example writes the body of `five` as just `5` — no
`return`, no `;` — and the value of that final line *is* the return
value. That implicit form is idiomatic Rust, but it depends on a
*statement-vs-expression* distinction this run has not installed yet.
So this lesson uses only the explicit `return value;` form, which the
Book itself lists as a valid option.

## What Changed

- You can declare a function's return type by writing `-> RTYPE` after
  the parameter list.
- You can send a value back from the body with `return value;`. That
  statement also exits the function — anything after it in the same
  body does not run.
- You can use a call as a value: `let result: i32 = add_one(5);` puts
  the call expression on the right of `let`.
- Contrast: a function with no `-> RTYPE` (lessons 008 and 020)
  returns nothing useful and is called as a statement, `name(args);`;
  a function with `-> RTYPE` returns a value and the call expression
  `name(args)` carries it.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn double(n: i32) -> i32 {
    return n * 2;
}

fn main() {
    let answer: i32 = double(7);
    println!("answer = {answer}");
}
```

You run `rustc tiny.rs && ./tiny`.

- Does rustc accept the program, and what does the executable print?
- In `fn double(n: i32) -> i32`, which piece is the parameter type and
  which is the return type?
- What value does `double(7)` evaluate to before the `let` binds it?

(Answers: yes; prints `answer = 14`. First `i32` is the parameter
type; second `i32` is the return type. `double(7)` evaluates to `14`,
the value sent back by `return n * 2;` with `n` holding `7`.)

## What To Ignore For Now

Real and deferred:

- *Implicit final-expression return.* The Book's `fn five() -> i32 { 5 }`
  example uses no `return` and no `;` — the value of the body's final
  expression *is* the return value. The idiomatic Rust form; depends on
  the *statement-vs-expression* distinction (deferred from lesson 004).
- *The unit return type `()`.* A function with no `->` returns a value
  of a special "nothing useful" type written `()`. That is why
  `say_value(5);` in lesson 020 was used as a statement, not a value.
  Deferred.
- *Returning tuples* — `fn pair() -> (i32, i32)`. Deferred.
- *Returning structs, enums, references, or generic types.* Each is
  its own later move.
- *`Result` and `Option` and the `?` operator* used to propagate them.
  Deferred.
- *`main` itself returning a `Termination`-implementing type.* Carried
  forward from lesson 002.
- *Early-return semantics in detail* (early returns inside `if`/`else`
  arms, multiple `return`s). Deferred.
- All previously deferred items: `mut`, shadowing, the broader
  format-string DSL, `cargo`, modules and `pub`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-03-how-functions-work.md`, the
  "Functions with Return Values" section (lines 252-280; the lesson
  stops before "Let's examine this in more detail." on line 290). One
  load-bearing direct quote, lines 254-258:

  > Functions can return values to the code that calls them. We don't
  > name return values, but we must declare their type after an arrow
  > (`->`). In Rust, the return value of the function is synonymous
  > with the value of the final expression in the block of the body of
  > a function. You can return early from a function by using the
  > `return` keyword and specifying a value, but most functions return
  > the last expression implicitly.

  The Book then shows `fn five() -> i32 { 5 }` and binds the call with
  `let x = five();`.

  Calibration: the Book's `fn five() -> i32 { 5 }` example uses the
  *implicit* form (no `return`, no `;`). This lesson uses the
  *explicit* `return value;` form throughout, because the implicit
  form depends on the statement-vs-expression distinction that this
  run has not installed yet. The Book's own quoted sentence lists
  `return` as an explicit option ("you can return early from a function
  by using the `return` keyword and specifying a value"), so the
  explicit form is canonical Rust even if non-idiomatic. The Book also
  builds with `cargo run`; this lesson uses `rustc demo.rs` per
  lesson 001. Behavior is identical.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/021-function-return-value.rs`.
The committed file is the working program. There is no separate
broken-contrast file; the load-bearing observation is that the call
expression `add_one(5)` carries the value `6` to the right of `let`,
where it is bound to `result` and printed.

Probe transcript, run in a temp directory created with `mktemp -d` and
removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn add_one(n: i32) -> i32 {
    return n + 1;
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
result = 6
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The single output line is `result = 6`. The `6` is the value that
  `return n + 1;` sent back from `add_one`'s body with `n` holding
  `5`. That value reached the right-hand side of the `let` in `main`
  through the call expression `add_one(5)`, and the `let` bound it to
  `result`. The `println!` then printed it.
- The call site `add_one(5)` works as a value, not just as a
  statement. That is the load-bearing observation for this lesson.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `005-let-binding` (accepted, load-bearing) — `let name: TYPE = value;`
  binds a name to a value; `println!("... {name} ...");` substitutes
  it at print time. Reused as the slot the returned value lands in.
- `008-define-and-call-function` (accepted) — define a second function
  with `fn name() { ... }` and call it from `main` with `name();`.
- `009-arithmetic-on-integers` (accepted, load-bearing) — `+` between
  two integer values produces a new integer value. Used inside
  `add_one`'s body as `n + 1` to produce a non-trivial return value.
- `019-type-annotation-i32` (accepted, load-bearing) — every value has
  a type; the form `name: TYPE` attaches one. The `-> i32` in this
  lesson's signature is the same idea applied to the return slot.
- `020-function-with-parameter` (accepted, load-bearing) — define
  `fn name(p: i32) { ... }` and call with `name(value);`. This lesson
  extends that exact shape with a return type and a `return value;`
  body.
