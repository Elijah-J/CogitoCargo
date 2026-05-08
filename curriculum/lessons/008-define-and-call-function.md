---
id: 008-define-and-call-function
move: "define a second function with `fn name() { ... }` and call it from `main` with `name();`"
main_concept: "a Rust file can contain more than one function; `fn name() { ... }` defines one, and the statement `name();` calls it, transferring control to its body and returning to the caller when that body finishes; without a definition, `name();` fails with E0425 `cannot find function ... in this scope` (same E-code lesson 005 hit for missing values)"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 004-statements-in-order
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "function parameters" moves
  - future "function return values" moves
  - future "expression vs statement" moves
  - future "recursion" moves
  - future "modules / pub" moves
sources:
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/error_codes/E0425.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/008-define-and-call-function.rs
status: accepted
---

# Define a second function and call it from `main`

## The Move

In the same `.rs` file as `fn main`, write a second function of the
same shape: `fn name() { ... }`. From inside `main`, write the
statement `name();`. Compile and run. `main`'s body runs top to bottom
as before, but at the `name();` line control jumps into the body of
`name`, runs it, then returns to the line after the call. Delete the
`fn name() { ... }` block but keep the call, and `rustc` refuses with
`error[E0425]: cannot find function \`name\` in this scope`.

## Mental Model Delta

- Before: "A `.rs` file has one `fn main` and that is the only function
  I write. Things that look like calls inside `main` are stuff the
  language hands me, like `println!`."
- After: "A `.rs` file can hold more than one function.
  `fn name() { ... }` defines one; `name();` *calls* it. A call is a
  side trip: control enters the function's body, runs it, returns to
  the line after the call. `main` is special only by *name*, not by
  *shape*. Without a matching definition, rustc rejects the call with
  `E0425` — same code lesson 005 hit, with `function` in place of
  `value`."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` produces an executable next to the
    source; run with `./name`; silent on success.
  - Lesson 002 (load-bearing): body of `fn main` runs when the
    executable launches. Deferred *defining* any other function;
    picked up here.
  - Lesson 003 (load-bearing): rustc errors have headline + `-->`
    location + source excerpt with caret + optional `help:`/`= note:`.
    The contrast probe uses that map.
  - Lesson 004 (load-bearing): `;`-terminated statements in `fn main`
    run top to bottom. Deferred *function calls*; picked up here.
    `say_hi();` is itself such a statement.
  - Lesson 005 (*not* load-bearing): cited only for the `E0425`
    family connection — same code, with `function` in place of
    `value`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    println!("from main");
    say_hi();
    println!("from main again");
}

fn say_hi() {
    println!("from say_hi");
}
```

Two `fn` blocks live in the file. `fn main` you know. The second,
`fn say_hi() { ... }`, is the same shape with a different name. The
Book: "We define a function in Rust by entering `fn` followed by a
function name and a set of parentheses. The curly brackets tell the
compiler where the function body begins and ends." And on the call
side: "We can call any function we've defined by entering its name
followed by a set of parentheses." So `say_hi();` between the two
`println!` lines is a *call*; it is itself a `;`-terminated statement
that slots into the source-order sequence from lesson 004.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
from main
from say_hi
from main again
```

`from say_hi` lands between the two `main`-side prints. Walk it:
`main` starts (lesson 002); the first `println!` prints; `say_hi();`
transfers control into `say_hi`'s body; that body's `println!` prints;
the body ends at its closing `}` and control returns to `main` on the
line after the call; the last `println!` prints; `main` ends. The
Book gives the same picture: "The lines execute in the order in which
they appear in the `main` function. First the 'Hello, world!' message
prints, and then `another_function` is called and its message is
printed." The extra `println!` after our call is what makes the
"control comes back" part visible.

Now the contrast. *Predict*: delete the `fn say_hi() { ... }` block
but keep the call — will `rustc` build an executable, and which
lesson-003 part of the diagnostic will pinpoint it?

Edit `demo.rs` to contain only `main`:

```rust
fn main() {
    println!("from main");
    say_hi();
    println!("from main again");
}
```

Compile again:

```console
$ rustc demo.rs
error[E0425]: cannot find function `say_hi` in this scope
 --> demo.rs:3:5
  |
3 |     say_hi();
  |     ^^^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

Read with the lesson 003 map: headline carries `E0425`, message
`cannot find function \`say_hi\` in this scope`; location
`demo.rs:3:5`; caret underlines `say_hi` at the call site; only an
inline `not found in this scope` annotation; `--explain E0425`
trailer. Compared to lesson 005's diagnostic, only the noun differs
— `function` vs. `value`. `E0425` is rustc's general "I don't know
what name you mean" code. Restore the `fn say_hi() { ... }` block
and the three lines print again.

## What Changed

- You can put more than one function in a `.rs` file by adding a
  second `fn name() { ... }` block alongside `fn main`.
- You can call a function you defined by writing `name();` as a
  statement inside another function's body.
- A call is a side trip: control enters the body, runs it, returns
  to the line after the call.
- `main` is special by *name*, not *shape*.
- `error[E0425]: cannot find function ... in this scope` is the same
  `E0425` family as lesson 005's `cannot find value`, with a different
  noun.

## Check Yourself

(a) You write `tiny.rs` containing:

```rust
fn main() {
    println!("a");
    one();
    println!("c");
}

fn one() {
    println!("b");
}
```

You run `rustc tiny.rs && ./tiny`. How many lines, in what order?

(b) Now delete the `fn one() { ... }` block but keep the call to
`one();`. Does a new `tiny` executable get produced? What error code,
message, and underlined identifier should you expect?

(Answers: (a) three lines: `a`, `b`, `c` — the call to `one` runs
between the first and last `println!` in `main`. (b) No new
executable; `error[E0425]: cannot find function \`one\` in this
scope`, caret on `one` at the call site, `--explain E0425` trailer.)

## What To Ignore For Now

This lesson installs only one idea: user-defined functions exist, and
*define + call* is one paired pattern. Deferred:

- *Parameters* — anything inside the `()` (`fn name(x: i32)`,
  `name(3)`). Both `()`s here are empty. The Book's next section.
- *Return values* and the `->` arrow (`fn five() -> i32 { 5 }`).
- *Function call as expression vs statement.* We use only the
  statement form `name();`.
- *Recursion* and mutual recursion.
- *Function pointers*, *closures*, *higher-order functions*.
- `pub` / *visibility*. No `pub` here; visibility does not fire.
- *Modules*, other crates, `use`. Everything lives in one `.rs` file.
- *Where the definition can sit.* The Book: "Rust doesn't care where
  you define your functions, only that they're defined somewhere in a
  scope that can be seen by the caller." We use one placement:
  definition *below* `main`. The general rule is not taught yet.
- *Traits*, *generics*, *lifetimes*, the `Termination` trait around
  `main`'s return type — still deferred from lesson 002.
- `println!` is a *macro*, not a function: trailing `!` is the
  marker. Macros stay deferred from lesson 001.
- Previously-deferred items (`mut`, shadowing, types, type
  annotations, constants, `&mut`, broader format-string DSL,
  comments, `cargo`).

## Evidence

### Sources

- `output/docs/rust/book/ch03-03-how-functions-work.md`, the
  "Functions" section (lines 1-53; the lesson stops before
  "Parameters" on line 55). Three load-bearing quotes:
  - Lines 27-29: "We define a function in Rust by entering `fn`
    followed by a function name and a set of parentheses. The curly
    brackets tell the compiler where the function body begins and
    ends." — the *definition* shape.
  - Lines 31-33: "We can call any function we've defined by entering
    its name followed by a set of parentheses." — the *call* shape.
  - Line 51: "The lines execute in the order in which they appear in
    the `main` function. First the 'Hello, world!' message prints,
    and then `another_function` is called and its message is printed."
    — source-order execution including the call.
  Calibration: the Book builds with `cargo run`; this lesson uses
  `rustc demo.rs` directly per lesson 001. Behavior is the same. The
  Book places `another_function` *after* `main`; this lesson does the
  same with `say_hi`. The Book also notes Rust does not care where
  the definition sits; that general rule is explicitly deferred.
- `output/docs/rust/error_codes/E0425.md` — canonical "An unresolved
  name was used" explainer. Lesson 005 cited this for
  `cannot find value`; here it covers `cannot find function` too.
  `E0425` fires whenever rustc sees an identifier with no matching
  definition in scope, regardless of what kind of item the name was
  meant to refer to.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/008-define-and-call-function.rs`.
The committed file is the *working* version (with `fn say_hi`
defined). The broken contrast (delete the `fn say_hi() { ... }`
block) is documented as a second run inside this Evidence section,
not as a separate `.rs` file.

Probe transcript, both runs in the same temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: working program with fn say_hi defined ===
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    println!("from main");
    say_hi();
    println!("from main again");
}

fn say_hi() {
    println!("from say_hi");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
from main
from say_hi
from main again
exit=0

=== RUN 2: broken program, fn say_hi block deleted ===
--- cat demo.rs ---
fn main() {
    println!("from main");
    say_hi();
    println!("from main again");
}
--- rustc demo.rs (capturing stderr) ---
error[E0425]: cannot find function `say_hi` in this scope
 --> demo.rs:3:5
  |
3 |     say_hi();
  |     ^^^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after broken compile ---
demo
demo.rs
```

Notes:

- Run 1 (working): `rustc` exits 0, silent. `./demo` prints three
  lines: `from main`, `from say_hi`, `from main again`. The
  `from say_hi` line sits between the two `main`-side prints. That
  ordering is the load-bearing observation for "control transfers to
  the body, then comes back."
- Run 2 (broken): `rustc` exits 1 with `error[E0425]: cannot find
  function \`say_hi\` in this scope`; caret at `demo.rs:3:5`
  underlines `say_hi` at the call site; `--explain E0425` trailer
  follows. Lesson 003's map reads this block with no new vocabulary.
  The headline differs from lesson 005's only in noun
  (`function` vs. `value`).
- The `demo` in `ls after broken compile` is the executable from
  Run 1; Run 2 did not produce a new one (lesson 001's compile-then-run
  two-step).
- Only the working source is committed; the broken version exists
  only in this transcript. The temp dir was removed afterward.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted, load-bearing) — body of
  `fn main` runs when the executable launches; `main` is special by
  *name*. Lesson 002 deferred defining any other function; this
  lesson picks that up.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — four-part
  map (headline, `-->` location, source excerpt with caret,
  help/note) used to read the contrast probe's `E0425` block. Not
  re-taught here.
- `004-statements-in-order` (accepted, load-bearing) — `;`-terminated
  statements in `fn main` run top to bottom. `say_hi();` is itself
  one such statement; "control jumps in and comes back" is what
  happens between the statement before the call and the one after.
- `005-let-binding` (accepted; cited only for the `E0425` family
  connection, *not* load-bearing) — same `E0425` code, with
  `function` in place of `value`.
