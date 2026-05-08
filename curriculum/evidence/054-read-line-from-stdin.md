# Evidence — 054-read-line-from-stdin

Audit appendix for `lessons/054-read-line-from-stdin.md`. Holds the
corpus-quote map, the toolchain string, the working-probe and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/054-read-line-from-stdin.rs`); the broken-contrast
  `.rs` is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/std/io/struct.Stdin.md`

The std-library page for the type `std::io::Stdin`. Already cited in
lesson 050 for the type's existence and its plain-English
description. Reused here as the *primary* source for cycle 054 — the
new piece this cycle installs is a method on `Stdin`. Three load-
bearing spans new to cycle 054.

Lines 83 (the method's canonical signature):

> #### pub fn read_line(&self, buf: &mut String) -> Result<usize>

Direct corpus statement of:
- (a) `read_line` is a method on `Stdin` with signature
  `read_line(&self, buf: &mut String) -> Result<usize>`. The lesson
  installs (a) verbatim.
- (b) The second parameter type is `&mut String` — load-bearing for
  the broken-contrast probe (passing `&buf` instead of `&mut buf`
  fires E0308 because of *this* type, not a generic mutability
  miscategorization).
- (c) The return type is `Result<usize>`. The path on the page is
  `[Result](type.Result.md "type std::io::Result")<[usize]>` —
  i.e., the `io::Result` *type alias* (lines link to
  `type.Result.md`). The lesson does *not* surface the alias name; it
  describes the return type as "a `Result` whose `Ok` payload is the
  byte count read." The alias is explicitly deferred under *What To
  Ignore For Now*.

The `&self` receiver and the `usize` payload type are both deferred
(carrying over from cycle 040's `&self` autoref deferral and the
type-of-discarded-value-isn't-surfaced position).

Line 85 (the description):

> Locks this handle and reads a line of input, appending it to the
> specified buffer.

Corpus statement of *what `read_line` does*. The lesson body's
"reads one newline-terminated line of input and *appends* it to the
`String` named by `&mut buf`" rephrases this. The "Locks this handle"
half is glossed under *What To Ignore For Now* — the locking is
black-boxed today (carrying over from cycle 050's stdin singleton-
ness deferral).

Lines 88-92 (the load-bearing semantics list):

> For detailed semantics of this method, see the documentation on
> [`BufRead::read_line`](trait.BufRead.md#method.read_line "method std::io::BufRead::read_line"). In particular:
>
> - Previous content of the buffer will be preserved. To avoid appending
>   to the buffer, you need to [`clear`](../string/struct.String.md#method.clear "method std::string::String::clear") it first.
> - The trailing newline character, if any, is included in the buffer.

Two direct corpus claims the lesson body installs:
- *"Previous content of the buffer will be preserved."* The lesson's
  "It appends. Prior content of `buf` is preserved" comes from this
  bullet directly. The corpus mentions `String::clear` as the
  workaround; the lesson names it under *What To Ignore For Now*.
- *"The trailing newline character, if any, is included in the
  buffer."* The lesson's "the trailing newline goes in too" comes
  from this bullet directly. Empirically corroborated by the
  side-probe `buf.as_bytes()` reading `[104, 105, 10]` for input
  `"hi\n"` — the `10` is the `\n`, present after the `i`.

The page's Examples block (lines 94-107) matches the working
probe's chain shape (modulo using `match` instead of `.expect()`):

> ```
> use std::io;
>
> let mut input = String::new();
> match io::stdin().read_line(&mut input) {
>     Ok(n) => {
>         println!("{n} bytes read");
>         println!("{input}");
>     }
>     Err(error) => println!("error: {error}"),
> }
> ```

Calibration: the example uses `match` to handle the `Result`
explicitly. The lesson uses `.expect(...)` (lesson 053) instead, which
is the Book-idiomatic guessing-game form; both are valid consumers of
the `Result`. The Book ch02 chain uses `.expect`.

Lines 109-114 (the "how to give it stdin" explanation):

> You can run the example one of two ways:
>
> - Pipe some text to it, e.g., `printf foo | path/to/executable`
> - Give it text interactively by running the executable directly,
>   in which case it will wait for the Enter key to be pressed before
>   continuing

Direct corpus license for the lesson's "to run such a program I have
to *give it stdin* — typically by piping with `echo "..." | ./demo`,
or by running it interactively and pressing Enter." The corpus uses
`printf foo`; the lesson uses `echo "..."` (which prepends a
trailing `\n`, matching the line-buffered behavior assumed by
`read_line`). Both shells produce equivalent stdin for the line-
read case.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053. Reused here for the audience-level walkthrough
of the *exact* statement this lesson teaches. Three load-bearing
spans new to cycle 054.

Lines 258-273 (the canonical guessing-game statement):

> ```rust
> use std::io;
>
> fn main() {
>     println!("Guess the number!");
>
>     println!("Please input your guess.");
>
>     let mut guess = String::new();
>
>     io::stdin()
>         .read_line(&mut guess)
>         .expect("Failed to read line");
>
>     println!("You guessed: {guess}");
> }
> ```

Direct corpus precedent for the lesson's working probe — the only
differences are (a) the buffer is named `buf` instead of `guess` and
(b) the chain is on one line instead of three (the Book later shows
the one-line form too at line 329 — "We could have written this code
as: `io::stdin().read_line(&mut guess).expect("Failed to read line");`",
followed by a stylistic note about line-breaking). The lesson's
choice to write the chain on one line and to use the buffer name
`buf` are minor surface differences — the load-bearing fact (this
exact composition compiles and runs) is the same.

The `use std::io;` line at the top is the Book's canonical form. The
lesson uses the same line. Cycle 050 deferred this exactly:

> *`use std::io;` so the call shortens to `io::stdin()` —
> parent-module use form, deferred from cycle 044. The Book chapter 2
> source uses that form; today uses the full path so no `use` is
> required.

Cycle 054 picks up that deferred surface.

Lines 282-288 (the audience-level read_line description):

> Next, the line `.read_line(&mut guess)` calls the `read_line`
> method on the standard input handle to get input from the user.
> We're also passing `&mut guess` as the argument to `read_line` to
> tell it what string to store the user input in. The full job of
> `read_line` is to take whatever the user types into standard input
> and append that into a string (without overwriting its contents),
> so we therefore pass that string as an argument. The string
> argument needs to be mutable so that the method can change the
> string's content.

Direct audience-level corpus statement of:
- (a) "`read_line` is a method on the standard input handle" — the
  lesson uses this framing.
- (b) "`&mut guess` as the argument" — corroborates the call shape.
- (c) "append ... without overwriting its contents" — Book-level
  rephrasing of the std page's "Previous content of the buffer will
  be preserved." The lesson body cites *both* sources by stating
  the append semantic directly.
- (d) "The string argument needs to be mutable so that the method
  can change the string's content." — audience-level statement of
  *why* `&mut`, not `&`. The lesson's broken-contrast probe
  empirically confirms this (E0308 fires for `&buf`).

Line 358 (what's on the `Ok` side of `read_line`'s return Result):

> If this instance of `Result` is an `Ok` value, `expect` will take
> the return value that `Ok` is holding and return just that value
> to you so that you can use it. In this case, that value is the
> number of bytes in the user's input.

Direct corpus statement that the `Ok` payload of `read_line` is "the
number of bytes in the user's input" — the lesson body's "the byte
count read." The lesson's chain discards the count by not binding
the value of the chain expression.

### `output/docs/rust/std/io/type.Result.md`

The std-library page for the `io::Result<T>` type alias. New corpus
citation for cycle 054 — the lesson explicitly *defers* the alias as
a typed name but cites the page for grounding.

Lines 6-7 (the alias declaration):

> ```
> pub type Result<T> = Result<T, Error>;
> ```

Lines 12-15 (the description):

> A specialized `Result` type for I/O operations.
>
> This type is broadly used across `std::io` for any operation which
> may produce an error.

These two spans are cited *only* to justify the lesson's framing
choice: `read_line`'s actual return type per the corpus is `Result<usize>`
(in `std/io/struct.Stdin.md` line 83), which resolves through the
alias to `Result<usize, io::Error>`. The lesson treats the return
type as "a `Result` whose `Ok` payload is the byte count read" —
which is faithful at the surface level (lesson 052's `Result<T, E>`
shape with `T=usize` and `E=io::Error`) without naming either the
alias or `io::Error`. *What To Ignore For Now* lists both as deferred.

### `output/docs/rust/error_codes/E0308.md`

The error-code reference page for E0308. Already cited in lessons
024-034, 045-048, 052, and (implicitly) elsewhere as the
mismatched-types diagnostic. Reused here for the broken-contrast
probe — passing `&buf` to a `&mut String` parameter fires E0308 with
caret label `types differ in mutability`. The lesson's broken-
contrast paragraph and the contrast probe transcript below match the
canonical E0308 shape.

### Lesson 050's evidence appendix (existing)

The fact that `io::stdin()` is callable and returns a `Stdin` value
is *not* re-cited inline here — it is fully grounded in
`evidence/050-io-stdin-handle.md` and the lesson body relies on
lesson 050 as a load-bearing prerequisite. Today's working probe
extends 050's probe by one method call.

### Lesson 053's evidence appendix (existing)

The fact that `.expect("msg")` consumes a `Result<T, E>` and either
yields the `Ok` payload or panics with `msg: <Err>` is *not*
re-cited inline here — it is fully grounded in
`evidence/053-result-expect-and-panic.md`. Today's chain feeds the
result of `read_line` into `.expect`; the panicking branch is not
exercised in the working probe (the probe's `echo "hello" | ./demo`
input always succeeds).

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/054-read-line-from-stdin.rs`.
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
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    println!("got: {buf}");
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after ---
demo
demo.rs
--- echo "hello" | ./demo ---
got: hello

demo-exit=0
--- printf 'world\n' | ./demo ---
got: world

demo-exit=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. No warnings. The chain
  `io::stdin().read_line(&mut buf).expect("Failed to read line")`
  type-checks: each call's return type satisfies the next call's
  receiver type, and the final `.expect` consumes the `Result`.
- `echo "hello" | ./demo` prints exactly two lines: `got: hello`
  followed by a blank line. The blank line is the `\n` `read_line`
  appended to `buf` (after `o`), then `println!`'s own newline.
- `printf 'world\n' | ./demo` produces the same two-line shape with
  `world` substituted — corroborates that the chain is reading from
  stdin, not a hard-coded source.
- Both runs exit 0 — `.expect` did not panic, meaning `read_line`
  returned `Ok(_)` both times.

### Broken-contrast probe (Shape B — `&buf` instead of `&mut buf`)

Source (not committed — the transcript below is the artifact):

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&buf).expect("Failed to read line");
    println!("got: {buf}");
}
```

The only change from the working probe is the call site:
`&buf` (shared) instead of `&mut buf` (mutable). Captured
2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&buf).expect("Failed to read line");
    println!("got: {buf}");
}
--- rustc broken.rs ---
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
note: method defined here
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs:411:11

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
rustc-exit=1
--- ls after rustc ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- Headline `error[E0308]: mismatched types` — same E-code lessons
  045-048 captured for `&` vs `&mut`. The lesson body cites this
  E-code precedent.
- Caret label `types differ in mutability` — identical wording to
  lessons 047 and 048's broken-contrast probes. This is the load-
  bearing label the lesson body quotes; it confirms `&buf` and
  `&mut buf` are *typed differently*, not just "different addresses
  of the same thing."
- The `= note:` block reads
  `expected mutable reference \`&mut String\` / found reference \`&String\``
  — this concretizes the abstract `&mut T` vs `&T` distinction lessons
  047/048 grounded, by naming `T = String` (today's buffer type from
  lesson 042).
- Second `-->` is to library internals
  (`/rustc/.../library/std/src/io/stdio.rs:411:11`). The location
  points at the `read_line` definition site inside the std source —
  rustc is telling the learner *where the parameter type was
  declared*. Today's lesson does not surface this internal path; it's
  noted here as part of the captured E0308 shape.
- `error: aborting due to 1 previous error` — single error halt, no
  binary produced. `ls after rustc` shows only `broken.rs`. This is
  the canonical E0308 halting behavior.
- No executable was produced — `./broken` cannot run because rustc
  never wrote a binary.

This probe is *load-bearing* for the lesson's claim "It must be a
`&mut String`. `&buf` (shared) does not type-check." Without the
probe, the assertion would rely solely on the corpus's signature
(`buf: &mut String` at `std/io/struct.Stdin.md` line 83) and the
mutability-differs precedent from cycles 047-048; the captured
transcript is the empirical confirmation.

### Side probe — buffer-bytes inspection (not committed)

Auxiliary probe used during evidence preparation; transcript
included for the trailing-newline claim. Source:

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    println!("buf has {} bytes", buf.len());
    println!("buf bytes: {:?}", buf.as_bytes());
}
```

Captured 2026-05-07; only the relevant runtime output:

```text
--- echo "hi" | ./demo ---
buf has 3 bytes
buf bytes: [104, 105, 10]
```

Notes:

- Input was `"hi\n"` (two characters plus the `\n` `echo` adds).
- `buf.len()` returns `3` — confirms three bytes were appended.
- The byte sequence `[104, 105, 10]` is `'h'` (104), `'i'` (105),
  `'\n'` (10). The `10` at the end is the trailing newline `read_line`
  included in the buffer per `std/io/struct.Stdin.md` line 92 ("The
  trailing newline character, if any, is included in the buffer").
- Cited inline above as the empirical corroboration of the corpus
  claim. The `.len()` and `.as_bytes()` methods, plus the `{:?}`
  Debug-formatting placeholder, are *not* surfaced in the lesson —
  used only here for buffer inspection.

### Side probe — empty stdin (not committed)

Verifies the deferral noted under *What To Ignore For Now* ("EOF /
empty-stdin behavior — `./demo < /dev/null` reads zero bytes,
returns `Ok(0)`, does *not* panic"):

```text
--- ./demo < /dev/null ---
got: []
exit=0
```

Notes:

- `./demo < /dev/null` provides empty stdin (immediate EOF).
- The program prints `got: []` (with the brackets from a probe
  variant that wrapped `{buf}` in `[...]`) — i.e., `buf` is empty
  after the call.
- Exit code is 0 — `.expect()` did *not* panic. `read_line` returned
  `Ok(0)` (zero bytes read at EOF), the chain yielded `0`, the value
  was discarded, and `println!` ran normally.
- This empirically confirms the orchestrator's "Shape A is not
  actually a runtime panic" observation. The lesson body defers EOF
  handling as a future move and does *not* assert any panic behavior
  for empty stdin.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 054.

- **Lesson 050 (load-bearing)** — installed `std::io::stdin()` as a
  free function returning a value of type `std::io::Stdin`. Lesson
  054 calls a *method* on that value for the first time. The chain
  `io::stdin().read_line(...)` uses 050's call as the receiver of the
  new `.read_line` method.
- **Lesson 044 (load-bearing)** — installed `use Path::name;` to
  shorten a path. Today uses the parent-module form `use std::io;`
  to make the bare `io::stdin()` resolve. Cycle 050 deferred this
  exact surface ("`use std::io;` so the call shortens to
  `io::stdin()`") under *What To Ignore For Now*; cycle 054 picks
  it up. The Book chapter 2 uses the same form (citation above).
- **Lesson 042 (load-bearing)** — `String::new()` returns a fresh
  empty `String`. The buffer `buf` is built by this call; the
  lesson's "today's `buf` started empty" depends on this.
- **Lesson 006 (load-bearing)** — `let mut name` makes the binding
  mutably-borrowable. `&mut buf` requires `let mut buf`. Without it,
  rustc would fire E0596 (deferred since cycle 047). The lesson
  surfaces only the empirical fact ("Without it `&mut buf` is
  rejected") without naming the E-code.
- **Lesson 048 (load-bearing)** — installed the call form `name(&mut binding)`
  for a `&mut T` parameter. `read_line`'s second parameter is
  `&mut String`; the call passes `&mut buf`. The "argument shape for
  a `&mut T` parameter" mechanic transfers unchanged. The broken-
  contrast probe today fires the same E0308 family that lesson 048's
  broken probe fired (`types differ in mutability` caret label).
- **Lesson 049 (load-bearing)** — installed method chaining: the
  receiver of `.method` is *any expression*, including another method
  or function call. Today's chain has *two* such generalizations: the
  receiver of `.read_line` is the call expression `io::stdin()`, and
  the receiver of `.expect` is the chained expression
  `io::stdin().read_line(&mut buf)`. Lesson 049's working probe
  chained two methods; today chains three.
- **Lesson 052 (load-bearing)** — installed `Result<T, E>` as the
  prelude two-variant enum with `Ok(T)` / `Err(E)` constructors. The
  return type of `read_line` is a `Result` (specifically
  `io::Result<usize>` per the std page; the lesson black-boxes the
  alias). The chain feeds this `Result` into `.expect`. Today's
  lesson does not surface the `Result<T, E>` *type expression* —
  just the noun *Result*.
- **Lesson 053 (load-bearing)** — installed `.expect("msg")` as the
  consumer of a `Result<T, E>` that yields the `Ok` payload as a
  plain `T` or panics with `msg: <Err>` on `Err`. The chain's
  trailing `.expect("Failed to read line")` is exactly that.
  Cycle 053's `.expect` had a synthetic `Result<i32, i32>` receiver;
  cycle 054's `.expect` has a real `io::Result<usize>` receiver.
- **Lesson 045 (broken-contrast precedent)** — installed `&binding`
  as a *shared* reference, distinct from `&mut binding`. The
  E0308 broken-contrast for `&buf` vs `&mut buf` matches the
  E0308 family lessons 045-048 captured (caret label `types differ
  in mutability`).
- **Lessons 001, 002, 005** — `rustc file.rs` then `./name`, `fn main`
  entry, `let name = value;` plus the `{name}` placeholder. All
  used unchanged.

## Older supporting lessons

- Lesson 040 (method-call syntax — the dot-form grammar
  `value.method(args)` lessons 040 → 049 → 054 trace, with the
  receiver-is-an-expression generalization arriving at 049). Not
  re-stated. The `&self` autoref deferral (cycle 040) carries over.
- Lesson 041 (qualified method call — path-grammar precedent for
  `Type::method(...)`). Not used today.
- Lesson 043 (nested-module-path form `module::submodule::name(...)`).
  Used implicitly via lesson 044's `use std::io;` line, but not
  load-bearing for today's surface.
- Lesson 029 (underscore-prefix gloss). Not used today — the binding
  `buf` is *used* (by `read_line` and by `println!`), so no
  underscore is needed. Different from cycle 050's `_stdin`.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe writes `use std::io;` rather than the full-path form
  `std::io::stdin()`. Cycle 050 used the full-path form because cycle
  044 was not yet load-bearing for that lesson; cycle 054 picks up
  the parent-module use form to match the Book idiom. Both forms call
  the same function; the choice is stylistic, but the Book ch02
  source uses the `use std::io;` form (lines 259, 268-273).
- The buffer is named `buf` rather than the Book's `guess`. The
  Book's `guess` is specific to the guessing-game context; `buf` is
  generic. The probe behavior is identical regardless of name.
- The chain is written on one line rather than the Book's three-line
  form. The Book itself shows both: line 268-270 uses the three-line
  form for readability, and line 329 explicitly notes the one-line
  form is equivalent ("We could have written this code as:
  `io::stdin().read_line(&mut guess).expect("Failed to read line");`").
  The lesson uses one line because the chain is short enough to read
  comfortably; cycles 049 and 053 have already established the
  receiver-is-an-expression chain shape.
- The `println!("got: {buf}")` interpolates `buf` directly, including
  the trailing `\n`. This is what produces the blank line in the
  output. The lesson treats this as a feature (the empirical proof
  that the trailing newline is in the buffer) rather than a bug;
  `.trim()` is mentioned by name as the standard way to strip it,
  deferred as a future move.
- The probe's `.expect` message `"Failed to read line"` matches the
  Book ch02 verbatim. No deviation.
- No EOF probe is run as part of the working/broken contrast pair —
  EOF behavior is captured only in the side probe and explicitly
  deferred under *What To Ignore For Now*. The orchestrator
  recommended *not* using EOF as a contrast (Shape A) because
  `read_line` returns `Ok(0)` on empty stdin, not `Err`, so
  `.expect()` would not panic.
- The broken-contrast probe (Shape B) is the canonical contrast: the
  load-bearing claim "`read_line` takes `&mut String`, not `&String`"
  is empirically confirmed by the E0308 transcript. This matches the
  orchestrator's recommendation.
