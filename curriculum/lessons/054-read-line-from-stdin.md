---
id: 054-read-line-from-stdin
status: accepted
evidence: ../evidence/054-read-line-from-stdin.md
---

# Read one line of standard input with `io::stdin().read_line(&mut buf).expect("Failed to read line");`

## The Move

Seven prior moves snap together into one statement. The only new piece
is the method `Stdin::read_line`; the chain does the rest.

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    println!("got: {buf}");
}
```

Read line 5 left to right as three method calls in sequence:

- `io::stdin()` — lesson 050's free function, now reached as
  `io::stdin` rather than `std::io::stdin` because line 1's
  `use std::io;` brought `io` into scope (lesson 044's form).
  Returns a `Stdin` handle.
- `.read_line(&mut buf)` — the new piece. A method on `Stdin` that
  takes `&mut buf` (lesson 048) and *appends* a newline-terminated
  line of input to the `String` named by that reference. Returns a
  `Result` (lesson 052) whose `Ok` payload is the byte count read.
- `.expect("Failed to read line")` — lesson 053's panic-on-`Err`
  consumer. On success it yields the byte count; we discard it by
  not binding the chain's value.

Lesson 049's left-associative chaining groups this as
`(io::stdin().read_line(&mut buf)).expect(...)` — three method calls,
one statement.

## What `read_line` does to `buf`

Three facts about the buffer matter today:

- *It must be a `&mut String`.* `&buf` (shared) does not type-check;
  the contrast probe is in *Try It* below.
- *It appends.* Prior content of `buf` is preserved; the input bytes
  are added at the end. Today's `buf` started empty (it came from
  `String::new()`), so "append" and "fill" look the same.
- *The trailing newline goes in too.* The `\n` the user typed when
  they pressed Enter ends up as the last byte of `buf`. That is why
  the example output has a blank line after `got: hello`: the `\n`
  inside `buf` ends one line, and `println!`'s own newline ends the
  next (empty) one.

## Mental Model Delta

- *Before:* "I have `io::stdin()` (050), `String::new()` (042),
  `&mut x` arguments (048), method chaining (049), `Result` (052),
  and `.expect("msg")` (053). I do not yet have a way to actually
  read input."
- *After:* "`Stdin` has a method `read_line(&mut buf)` that appends
  one newline-terminated line of stdin into the `String` named by
  `&mut buf` and returns a `Result` whose `Ok` payload is the byte
  count read. The canonical guessing-game shape `io::stdin().read_line(&mut buf).expect("...")`
  is that method composed with the prior cycles. To run such a
  program I have to *give it stdin* — typically by piping with
  `echo "..." | ./demo`, or by running it interactively and pressing
  Enter."

## Prerequisites

- Installed concepts:
  - Lesson 050 (load-bearing): `io::stdin()` returns a value of type
    `std::io::Stdin`. Today is the first method call on that value.
  - Lesson 044 (load-bearing): `use Path::name;` brings `name` into
    scope. Today uses the parent-module form `use std::io;` so
    `io::stdin()` resolves; cycle 050 deferred this exact surface.
  - Lesson 042 (load-bearing): `String::new()` for the buffer.
  - Lesson 006 (load-bearing): `let mut buf` makes the binding
    mutably-borrowable; without it `&mut buf` is rejected.
  - Lesson 048 (load-bearing): the `&mut binding` argument form for
    a `&mut T` parameter. `read_line`'s second parameter is
    `&mut String`.
  - Lesson 049 (load-bearing): method chaining — the receiver of
    `.read_line` is the call `io::stdin()`; the receiver of
    `.expect` is the call `io::stdin().read_line(...)`.
  - Lesson 052 (load-bearing): `Result<T, E>`. `read_line` returns
    one.
  - Lesson 053 (load-bearing): `.expect("msg")` consumes a `Result`,
    yielding `Ok`'s payload or panicking with `msg: <Err>` on `Err`.
  - Lesson 045 (broken-contrast precedent): `&binding` is a *shared*
    reference, distinct from `&mut binding`. Same E0308 family as
    lessons 045-048.
  - Lessons 001, 002, 005: compile and run, `fn main`, `let` plus
    the `{name}` placeholder.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, and shell-piping with `echo "..." | ./demo`
  (same kind of assumption used since cycle 053 for `echo $?` and
  the `stdout`/`stderr` split).

## Try It

Save the snippet from *The Move* as `demo.rs`. Compile once, then run
twice with different input:

```console
$ rustc demo.rs
$ echo "hello" | ./demo
got: hello

$ echo "world" | ./demo
got: world

```

The blank line after `got: hello` is the trailing `\n` `read_line`
appended to `buf`, followed by `println!`'s own newline. The byte
count `read_line` returns (`6` for `"hello\n"`) was discarded —
`.expect(...)` yielded it, but no binding captured it.

*Predict:* edit line 5 to `io::stdin().read_line(&buf).expect("Failed to read line");`
(shared reference) and recompile:

```text
error[E0308]: mismatched types
 --> broken.rs:5:27
  |
5 |     io::stdin().read_line(&buf).expect("Failed to read line");
  |                 --------- ^^^^ types differ in mutability
  |                 |
  |                 arguments to this method are incorrect
  |
  = note: expected mutable reference `&mut String`
                     found reference `&String`
```

Same E0308 family as lessons 045/046/047/048. The caret label `types
differ in mutability` and the `expected mutable reference &mut String
/ found reference &String` note say it directly: `read_line` requires
a `&mut String`, not a `&String`.

## What Changed

- `Stdin` has a method `read_line(&mut buf)` that reads one
  newline-terminated line from standard input and *appends* it to
  the `String` named by `&mut buf`.
- `read_line` returns a `Result` whose `Ok` payload is the byte
  count read. Today's chain feeds that `Result` straight into
  `.expect`, discarding the count.
- The guessing-game shape `io::stdin().read_line(&mut buf).expect("Failed to read line");`
  is lessons 050 + 048 + 049 + 052 + 053 plus the one new method.
- `use std::io;` (lesson 044's form, parent-module style) shortens
  `std::io::stdin()` to `io::stdin()`. The Book uses this form.
- A `read_line` program needs stdin input to do anything useful.
  `echo "hello" | ./demo` is the simplest way to give it some.

## Check Yourself

You write `pred.rs`:

```rust
use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("hi, {name}!");
}
```

(a) Does `rustc pred.rs` succeed?

(b) What does `echo "Ada" | ./pred` print?

(c) Replace `&mut name` with `&name`. Which E-code fires and what
does the caret label say?

*(Answers: (a) Yes. (b) Two lines: `hi, Ada` then `!` on its own
line — the `\n` `read_line` appended to `name` ends the greeting
between `Ada` and `!`, and `println!` adds its own newline. (c) E0308
with caret label `types differ in mutability`; the `= note:` reads
`expected mutable reference &mut String / found reference &String`.)*

## What To Ignore For Now

- *The `io::Result<T>` type alias* — `read_line`'s actual return type
  is `io::Result<usize>`, an alias for `Result<usize, io::Error>`.
  Today treats it as "a `Result` with `usize` on the `Ok` side." Type
  aliases as a feature are deferred.
- *The `io::Error` type* — what's on the `Err` side. Deferred.
- *`usize` as a typed name* — the discarded byte count's type.
- *`.trim()` on a `String`* — strips trailing `\n` and other
  whitespace. Future move.
- *`.parse::<T>()`* — converts text to a typed value. Future move
  alongside type-changing shadowing.
- *`BufRead::read_line`* — the std page notes "For detailed semantics
  of this method, see the documentation on `BufRead::read_line`."
  Trait machinery, deferred since cycle 040.
- *`Stdin`'s internal mutex / locking* — the std page says
  `read_line` "Locks this handle and reads a line of input." Today
  black-boxes the locking.
- *EOF / empty-stdin behavior* — `./demo < /dev/null` reads zero
  bytes, returns `Ok(0)`, does *not* panic. Today's recipe uses
  `echo "..." | ./demo` so EOF does not appear.
- *Reading multiple lines* — call `read_line` again, or use
  `io::stdin().lines()` (named on the `Stdin` page). Future move.
- *Buffered vs unbuffered I/O*; *Windows `\r\n` vs Unix `\n`*. Heavy
  deferrals.
- *The `&self` autoref on `read_line`* — carrying over from 040.
- *`String::clear()`* — empties a buffer between reads. Future move.

## Evidence

See `../evidence/054-read-line-from-stdin.md`.
