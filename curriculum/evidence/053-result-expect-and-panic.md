# Evidence — 053-result-expect-and-panic

Audit appendix for `lessons/053-result-expect-and-panic.md`. Holds the
corpus-quote map, the toolchain string, the working-probe and panic
probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/053-result-expect-and-panic.rs`); the broken-contrast
  `.rs` is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/std/result/enum.Result.md`

The std-library page for `Result`. Already cited in lesson 052;
reused here for the `.expect` method itself. Three load-bearing
spans.

Lines 433-440 (the method's signature, summary, and the discouraged
note):

> #### pub fn expect(self, msg: &str) -> T where E: Debug,
>
> Returns the contained `Ok` value, consuming the `self` value.
>
> Because this function may panic, its use is generally discouraged.
> Instead, prefer to use pattern matching and handle the `Err` case
> explicitly, or call `unwrap_or`, `unwrap_or_else`, or
> `unwrap_or_default`.

Direct corpus statement that (a) `.expect` is a method on `Result<T,
E>`, (b) its return type is `T` (the `Ok` payload type), (c) it
consumes `self`, (d) it takes `msg: &str`, (e) the `where E: Debug`
bound applies, and (f) it can panic. The lesson body installs only
"`.expect("msg")` extracts the `Ok` payload as a plain `T`, or
panics" — surface (a), (b), (f). Surfaces (c) `self`-consume, (d)
`&str` as a typed name, and (e) the `where E: Debug` clause are
explicitly deferred under *What To Ignore For Now*; the lesson does
mention the full signature in that section so the learner can see
what's being deferred.

Lines 442-445 (the *Panics* section):

> ##### Panics
>
> Panics if the value is an `Err`, with a panic message including the
> passed message, and the content of the `Err`.

Direct corpus license for the lesson's central new claim: on `Err`,
the program panics, and the panic message contains *both* the passed
message *and* the `Err` payload's content. The captured probe
transcript below corroborates this empirically: `expected even: 7`
is exactly "passed message + content of `Err`" in the order the
spec describes (with `:` as the joiner).

Lines 447-454 (the worked example showing the panic-message format):

> ##### Examples
>
> ```
> let x: Result<u32, &str> = Err("emergency failure");
> x.expect("Testing expect"); // panics with `Testing expect: emergency failure`
> ```

Direct corpus precedent for the *exact* `<msg>: <Err payload>`
format with `:` as the joiner. The probe captured `expected even: 7`,
matching the same pattern. The lesson body cites this format as the
panic message line (`expected even: 7`).

Calibration: the std page also documents the *Recommended Message
Style* (lines 456-466 — "We recommend that `expect` messages are
used to describe the reason you *expect* the `Result` should be
`Ok`"). The lesson's `"expected even"` follows this convention
informally; the lesson explicitly defers it as a rule.

### `output/docs/rust/book/ch09-01-unrecoverable-errors-with-panic.md`

The Book chapter introducing panics. New corpus citation for cycle
053 (not used in 052). Three load-bearing spans.

Lines 4-11 (what a panic *is*):

> Sometimes bad things happen in your code, and there's nothing you
> can do about it. In these cases, Rust has the `panic!` macro.
> There are two ways to cause a panic in practice: by taking an
> action that causes our code to panic (such as accessing an array
> past the end) or by explicitly calling the `panic!` macro. In both
> cases, we cause a panic in our program. By default, these panics
> will print a failure message, unwind, clean up the stack, and
> quit.

Audience-level corpus statement that (a) a panic is a thing the
language has, (b) two ways to cause one — *implicitly* via library
methods (today's `.expect` on `Err` is exactly this case, since std
calls `panic!` internally) or *explicitly* via the `panic!` macro,
(c) the default behavior is print a failure message and quit. The
lesson body installs (a) and (c). The `panic!` macro itself is
deferred under *What To Ignore For Now*.

Lines 33-54 (the canonical panic transcript shape):

> ```rust
> fn main() {
>     panic!("crash and burn");
> }
> ```
>
> When you run the program, you'll see something like this:
>
> ```console
> $ cargo run
>    Compiling panic v0.1.0 (file:///projects/panic)
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
>      Running `target/debug/panic`
>
> thread 'main' panicked at src/main.rs:2:5:
> crash and burn
> note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
> ```

Direct corpus statement of the canonical panic-output shape: a
`thread 'main' panicked at <file>:<line>:<col>:` line, then the
message line, then the `note:` trailer about `RUST_BACKTRACE`.
Today's probe captured a small variant of this format —
`thread 'main' (125950362) panicked at demo.rs:10:28:` — with a
parenthesized thread id inserted between `'main'` and `panicked`.
The lesson body shows the captured form honestly and notes the
thread id is not load-bearing. The lesson is honest about the
format being the rustc-version-dependent surface; the load-bearing
facts (panic message contains `:` and `Err` content, exit non-zero,
`println!` after never runs) are independent of cosmetic format.

Calibration: the Book here uses `cargo run` which prints
`Compiling`/`Finished`/`Running` framing lines first. The lesson
uses bare `rustc` (lessons 001 / 032's contrast), so those framing
lines do not appear in the captured transcript. Only the post-panic
diagnostic block is shared between Book and probe.

### `output/docs/rust/std/macro.panic.md`

The std-library macro page for `panic!`. New corpus citation for
cycle 053. One load-bearing span.

Lines 67-69 (the *Current implementation* section, exit code):

> ## Current implementation
>
> If the main thread panics it will terminate all your threads and
> end your program with code `101`.

Direct corpus statement that a panic in the main thread ends the
program with exit code `101`. The probe captured `exit=101` exactly
as `echo $?` would report. The lesson body installs this directly
("A panic in `main` exits the process with status `101`") and the
captured transcript shows it. This is empirical evidence the spec
matches reality on this rustc.

Calibration: the page also discusses (a) `panic!` as a macro, (b)
unwind vs abort strategies, (c) panic hooks via
`std::panic::set_hook`, (d) `panic_any` for non-string payloads, (e)
edition differences in `panic!` semantics. *All* of these are
deferred under *What To Ignore For Now*. The lesson surfaces only
the `101` exit-code fact from this page.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 050,
051, 052. One load-bearing span new to cycle 053.

Lines 350-358 (the audience-level `.expect` description):

> Values of the `Result` type, like values of any type, have methods
> defined on them. An instance of `Result` has an `expect` method
> that you can call. If this instance of `Result` is an `Err` value,
> `expect` will cause the program to crash and display the message
> that you passed as an argument to `expect`. If the `read_line`
> method returns an `Err`, it would likely be the result of an error
> coming from the underlying operating system. If this instance of
> `Result` is an `Ok` value, `expect` will take the return value
> that `Ok` is holding and return just that value to you so that you
> can use it.

Audience-level corpus statement of *both* outcomes for `.expect`,
phrased exactly as the lesson body's mental-model delta: on `Err`
"crash and display the message," on `Ok` "take the return value
that `Ok` is holding and return just that value." The lesson's
"give me the `Ok` payload, or crash with this message" framing is a
near-direct rephrasing of this passage. "Crash" is the Book's
audience-level word for what the lesson formalizes as "panic." The
lesson body uses both forms.

Calibration: this Book passage is in the context of
`.read_line(&mut guess).expect("Failed to read line")`, where the
receiver of `.expect` is a method-chain. Today's probe uses
`.expect()` on a synthetic `Result<i32, i32>` so all involved types
are already installed. The chain form will compose in a future
cycle once `Stdin::read_line` is installed.

### `output/docs/rust/book/ch09-02-recoverable-errors-with-result.md`

The Book chapter on recoverable errors. Already cited in lesson 052.
Reused here for `.expect` framing. One load-bearing span new to 053.

Lines 234-253 (the audience-level `.expect` walkthrough):

> Similarly, the `expect` method lets us also choose the `panic!`
> error message. Using `expect` instead of `unwrap` and providing
> good error messages can convey your intent and make tracking down
> the source of a panic easier. The syntax of `expect` looks like
> this:
>
> ```rust
> use std::fs::File;
>
> fn main() {
>     let greeting_file = File::open("hello.txt")
>         .expect("hello.txt should be included in this project");
> }
> ```
>
> We use `expect` in the same way as `unwrap`: to return the file
> handle or call the `panic!` macro. The error message used by
> `expect` in its call to `panic!` will be the parameter that we
> pass to `expect`, rather than the default `panic!` message that
> `unwrap` uses. Here's what it looks like:
>
> ```text
> thread 'main' panicked at src/main.rs:5:10:
> hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
> ```

Direct corpus statement that (a) `.expect` "calls the `panic!` macro"
under the hood — i.e., `.expect` and `panic!` are the same mechanism,
(b) the message passed to `.expect` becomes the panic message, and
(c) the panic-output format is `<msg>: <Err payload printed>` (the
`hello.txt should be included in this project: Os { ... }` line is
exactly that shape). Today's probe corroborates (c) directly with
`expected even: 7` — same shape, simpler payload.

Calibration: this Book passage also previews `.unwrap` (deferred in
the lesson under *What To Ignore For Now*) and uses `File::open`
(returning `io::Result<File>`, deferred). The lesson body uses the
synthetic `parity` function so prerequisites are clean.

### Lesson 052's evidence appendix (existing)

The Result enum declaration, the prelude membership, the call-form
constructors, and the type-parameter framing are *not* re-cited
inline here — they are all summarized in
`evidence/052-result-enum-and-is-ok.md` and the lesson body relies
on lesson 052 as a load-bearing prerequisite. The redteam can chase
the chain through lesson 052 as needed.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/053-result-expect-and-panic.rs`.
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
fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let v: i32 = parity(4).expect("expected even");
    println!("v = {v}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo (stdout+stderr) ---
v = 4
exit=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. No warnings. No `use` line is
  needed — `Result`, `Ok`, `Err` are in the prelude (lesson 052's
  install).
- `./demo` prints exactly one line `v = 4` to stdout, exits 0. This
  is the load-bearing positive observation: when the receiver is
  `Ok(4)`, `.expect("expected even")` evaluates to `4` and execution
  continues normally to the `println!`. No panic.
- The annotation `let v: i32 = parity(4).expect("expected even");`
  is accepted by rustc with no E0308. This corroborates the spec
  claim from `std/result/enum.Result.md` line 433
  (`pub fn expect(self, msg: &str) -> T`) — the call expression's
  type is `T`, which for `Result<i32, i32>` is `i32`, matching the
  `: i32` annotation.
- The receiver of `.expect` is the call expression `parity(4)`, not
  a binding — this exercises the receiver-is-any-expression
  generalization from lesson 049. (Lesson 052's probe used a binding
  receiver instead; today is the next step.)

### Panic probe

Source (not committed — the transcript below is the artifact):

```rust
fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let v: i32 = parity(7).expect("expected even");
    println!("v = {v}");
}
```

Captured 2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let v: i32 = parity(7).expect("expected even");
    println!("v = {v}");
}
--- rustc broken.rs ---
exit=0
--- ls after rustc ---
broken
broken.rs
--- ./broken (separated stdout vs stderr) ---
exit=101
--- stdout content (empty) ---

--- stderr content ---

thread 'main' (125950362) panicked at broken.rs:10:28:
expected even: 7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Notes (probe evidence — not corpus quotation):

- `rustc broken.rs` exits 0. There is *no compile error*. The only
  source-level difference from the working probe is the integer
  literal `7` vs `4`; both produce a `Result<i32, i32>`, and rustc
  cannot statically distinguish which variant the function will
  return. Load-bearing for the lesson's "compile-time vs runtime"
  framing: this probe compiles, the working one compiles, the
  difference only surfaces at run time.
- `./broken` exits with status `101`. This corroborates
  `std/macro.panic.md` lines 67-69 ("end your program with code
  101") empirically. The lesson body cites this exit status.
- stdout is empty. The `println!("v = {v}")` line never executed.
  This is the load-bearing observation that a panic *aborts*
  execution mid-function — control never reaches the next
  statement. The lesson body installs this directly.
- stderr contains exactly three lines of meaningful content:
  - `thread 'main' (125950362) panicked at broken.rs:10:28:`
  - `expected even: 7`
  - `note: run with \`RUST_BACKTRACE=1\` environment variable to display a backtrace`
  Plus a leading blank line. The shape matches Book ch09-01 lines
  46-54's canonical example *modulo* the parenthesized thread id
  `(125950362)`. The thread id is OS-thread-id specific and varies
  per run; it is not load-bearing. The lesson notes this in *What
  To Ignore For Now*.
- The message line `expected even: 7` matches the pattern from
  `std/result/enum.Result.md` line 453's worked-example annotation
  `// panics with \`Testing expect: emergency failure\`` — both
  instantiate `<msg>: <Err payload>` with `:` as the joiner. The
  payload `7` is the `Err(7)` value's printed form (a plain integer).
- The location `broken.rs:10:28` points at line 10, column 28 — the
  `expect` method-name start. (Counting `    let v: i32 = parity(7).`
  takes columns 1-27, so column 28 is the `e` of `expect`. This is
  consistent with the `panicked at <file>:<line>:<col>:` shape from
  Book ch09-01 line 51, `thread 'main' panicked at src/main.rs:2:5:`.)
- Exit code: 101. No further executable behavior.

The panic probe is *load-bearing* for the lesson's central new
concept (runtime panics are categorically different from compile-time
errors). Without it, the lesson would assert "the program panics on
`Err`" without empirical demonstration. The panic shape, exit code,
stderr-vs-stdout split, and unreached `println!` are all directly
observed.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 053. Older supporting lessons are mentioned
by number only.

- **Lesson 052 (load-bearing)** — installed `Result<T, E>`,
  `Ok(T)`, `Err(E)`, the prelude membership, and `.is_ok()`. Lesson
  053 reuses *all* of these:
  - The `parity` function definition is identical to 052's working
    probe.
  - `Ok(n)` and `Err(n)` are still constructors building
    `Result<i32, i32>` values.
  - `.expect` is a method on `Result<T, E>`, in the same dot-form
    slot as `.is_ok()`. Lesson 052's *unlocks* explicitly listed
    "`.expect("msg")` and `.unwrap()` panic-on-Err methods" as a
    future move; today picks that exact future move.
  - The lesson body's *Mental Model Delta* explicitly references
    "Lesson 052 inspected with `.is_ok()` (returns `bool`, never
    fails)" as the *Before* state.
- **Lesson 040 (load-bearing)** — installed the dot-form method-call
  grammar `value.method(args)`. Lesson 053 reuses this slot for
  `.expect("expected even")` with one argument (the message string
  literal). The grammar-level shape is the same as `.is_ok()`'s
  empty-args form from lesson 052. Lesson 053 does *not* surface the
  method-call grammar; it just uses it.
- **Lesson 049 (load-bearing)** — installed the receiver-is-any-
  expression generalization, formally stating "the receiver slot
  accepts any expression that produces a value of the right type,
  including another method-call expression or a no-receiver
  associated-function call." Lesson 053 fills the receiver slot with
  the call expression `parity(4)`, which lesson 049's main_concept
  explicitly listed as a covered case. The lesson body cites lesson
  049 inline in *The Move*. Note: the orchestrator's prompt
  suggested 049 was *not* load-bearing because today's probe doesn't
  *chain two methods*. The worker's audit, however, finds 049
  load-bearing because lesson 040 alone showed only `binding.method()`
  (e.g., `n.abs()`), and `parity(4).expect(...)` requires the
  receiver-as-call-expression generalization. Without 049 the
  learner could reasonably believe `.expect` requires a binding,
  which the lesson contradicts. Recording the prerequisite is the
  honest call.
- **Lessons 026, 025** — `if`-as-expression as the function body's
  tail. The body of `parity` is `if n % 2 == 0 { Ok(n) } else { Err(n) }`
  with no trailing `;` and no `return` keyword. Both arms produce
  `Result<i32, i32>` (lesson 026's "arms must agree in type"). Same
  use as lesson 052.
- **Lessons 037, 013** — `n % 2 == 0` for the parity check. Same
  use as lesson 052.
- **Lesson 021** — function return-type slot `-> RTYPE`, today
  filled with `Result<i32, i32>`. Same use as lesson 052.
- **Lesson 020** — `parity(n: i32)` parameter slot. Same use.
- **Lesson 019** — `let v: i32 = parity(4).expect("expected even");`.
  The annotation slot accepts the call-expression's `i32` value
  type. This *also* serves as a small empirical confirmation of
  `.expect`'s return-type spec: rustc accepts the `i32` annotation,
  meaning `.expect` returned a `T` of `i32` for this `Result<i32,
  i32>`. (A wrong annotation would fire E0308 — not exercised today.)
- **Lessons 008, 012** — free-function call (`parity(4)`) and `bool`
  literals (in the `n % 2 == 0` expression). Same use as lesson 052.
- **Lessons 001, 002, 003, 005** — `rustc file.rs` then `./name`,
  `fn main` entry, reading rustc diagnostics, `let name = value;`.
  All used unchanged.

## Older supporting lessons

- Lesson 042 (`Type::name(args)` no-receiver call form). Not used
  today. (Today's `Ok(n)` and `Err(n)` come from lesson 052; their
  call shape is licensed there.)
- Lesson 044 (`use` declaration). Not used today: `Result`, `Ok`,
  `Err` are in the prelude.
- Lesson 050 (`std::io::stdin()`). Not load-bearing today, but
  lesson 050's *unlocks* listed both "the `Result<T, E>` enum"
  (picked by lesson 052) and "`.expect("msg")` on `Result`" (picked
  today). Future cycles will compose lesson 050's stdin handle with
  `Stdin::read_line(&mut buf)` returning `io::Result<usize>` and
  chain `.expect(...)` onto that.
- Lesson 051 (Ordering enum + match). Not used today: today's lesson
  does not match on the Result; it uses `.expect`.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe writes `Ok(n)` and `Err(n)` rather than `Result::Ok(n)`
  and `Result::Err(n)`. Both compile (prelude). Same choice as
  lesson 052.
- `let v: i32 = parity(4).expect("expected even");` annotates the
  binding type with `i32`. Annotation is not strictly required —
  rustc could infer the type from the rest of `main` — but the
  annotation makes the call-expression's return type visible at the
  call site and serves as a small empirical check that `.expect`
  returns `T = i32` (not the wrapped `Result<i32, i32>`).
- The probe message `"expected even"` follows the std-page's
  recommended-style convention ("describe the reason you *expect*
  the Result should be Ok") — though the lesson defers the
  convention as a rule.
- The probe value `7` for the panic case (vs `4` for the success
  case) is the same parity-test pair lesson 052 used. The lesson
  body's panic transcript shows the captured `expected even: 7`
  message; the lesson's *Check Yourself* uses `parity(11)` to
  confirm the learner can predict a different `Err` payload.
- The captured panic-output line `thread 'main' (125950362) panicked
  at broken.rs:10:28:` includes a parenthesized thread id
  `(125950362)` between `'main'` and `panicked`. This is rustc
  1.95.0-specific surface; older rustc transcripts (per Book
  ch09-01 line 51) wrote `thread 'main' panicked at src/main.rs:2:5:`
  without the thread id. The load-bearing facts (panic message
  contains `:` and `Err` content; exit non-zero; following lines do
  not run) are independent of this cosmetic detail. The lesson body
  shows the captured form honestly and notes the thread id is not
  load-bearing in *What To Ignore For Now*.
- The probe was run as a bare `rustc demo.rs && ./demo` rather than
  `cargo run`. Book ch09-01 used `cargo run` and showed
  `Compiling`/`Finished`/`Running` framing lines before the panic
  block; bare `rustc` produces no such framing. Only the post-panic
  block matters today.
- No broken-contrast probe at the *compile-time* level is included.
  The orchestrator's contrast structure for cycle 053 is
  *runtime-only*: working probe (`Ok` → success) vs panic probe
  (`Err` → runtime panic). Both compile. The contrast is between
  the two runtime outcomes of the same call site, not between
  compile failure and compile success. This is the most direct way
  to install "panics happen at run time, not compile time."
