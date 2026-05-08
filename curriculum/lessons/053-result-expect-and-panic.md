---
id: 053-result-expect-and-panic
status: accepted
evidence: ../evidence/053-result-expect-and-panic.md
---

# `.expect("msg")` on `Result<T, E>` — extract `Ok` payload, or panic

## The Move

Lesson 052 inspected a `Result<T, E>` with `.is_ok()`, which returns a
`bool` and never fails. `.expect("msg")` is the next step up: it
either *gives you the `Ok` payload* as a plain value or *terminates
the program*.

```rust
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
```

`parity(4)` produces `Ok(4)` (lesson 052's constructor shape). Calling
`.expect("expected even")` on that `Ok(4)` reaches into the variant,
takes out the wrapped `4`, and the call expression evaluates to `4` —
a plain `i32`, which is why `let v: i32 = ...` accepts it. `println!`
then prints `v = 4` and the program exits with status `0`.

The receiver of `.expect` here is the call expression `parity(4)`, not
a bound name — lesson 049 already licensed that.

## The two outcomes

`.expect("msg")` has two outcomes for the same call site, decided by
whether the `Result` is `Ok` or `Err`:

- *On `Ok(value)`:* the call evaluates to `value`. Execution continues
  on the next line, just like any other expression that produces a
  value.
- *On `Err(value)`:* the program *panics*. A panic is Rust's name for
  an aborting termination triggered at execution time. The next line
  never runs.

This is a new shape. Every previous broken probe in this run failed
at *compile time*: rustc refused to build an executable. A panic
happens *at run time*: the program compiles cleanly, the executable
exists, and the failure only surfaces while it runs.

## What a panic looks like

Change `parity(4)` to `parity(7)` and rebuild. `parity(7)` returns
`Err(7)`, so `.expect("expected even")` panics:

```text
$ rustc demo.rs
$ ./demo
thread 'main' (125950362) panicked at demo.rs:10:28:
expected even: 7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101
```

- `rustc demo.rs` exits `0` — no compile error. Whether the call
  panics depends on the runtime *value* `parity` produces.
- The diagnostic appears on `stderr`, not `stdout`. Standard output
  is empty: `println!("v = ...")` never runs, because the panic
  happens before control reaches that line.
- The message line reads `expected even: 7` — your `"expected even"`,
  then `:`, then the printed `Err` payload (`7`).
- Exit status is `101` (visible via `echo $?`).
- The `note:` line about `RUST_BACKTRACE=1` is the standard panic
  trailer; ignore it today.

## Mental Model Delta

- *Before:* "I have a `Result<T, E>`. I can ask `.is_ok()` for a
  `bool`. To get the wrapped value out, I'd have to `match` on it."
- *After:* "I have a `Result<T, E>`. I can call `.expect("msg")` to
  *commit* to the `Ok` side: if the value really is `Ok(v)`, I get
  `v` as a plain `T`; if it is `Err(e)`, the program *panics* — it
  prints `msg: <e>` to stderr and exits non-zero, and the next line
  of code never runs. Panics happen at run time, not compile time."

## Prerequisites

- Installed concepts:
  - Lesson 052 (load-bearing): `Result<T, E>`, `Ok(T)`, `Err(E)`. The
    receiver of `.expect` is a `Result<i32, i32>`.
  - Lesson 040 (load-bearing): dot-form `value.method(args)`.
  - Lesson 049 (load-bearing): the receiver is *any expression*, not
    just a binding name — today it is the call expression `parity(4)`.
  - Lesson 021: `-> Result<i32, i32>` return slot.
  - Lessons 026 / 025: `if`-as-expression as the function body's tail.
  - Lessons 037 + 013: `n % 2 == 0`.
  - Lessons 008, 020, 019: free-function call, parameter slot, `: i32`.
  - Lessons 001, 002, 003, 005: `rustc` + run, `fn main`, diagnostics,
    `let`.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, knowing that `echo $?` prints the shell exit
  status and that program output goes to `stdout` while diagnostics
  go to `stderr`.

## Try It

Save the snippet from *The Move* as `demo.rs`, then run
`rustc demo.rs && ./demo` — it prints `v = 4` and exits `0`. Now
change `parity(4)` to `parity(7)` and rebuild: rustc still exits
`0`, but `./demo` now produces a panic (full transcript above) and
`echo $?` prints `101`. The program compiled both times; the
difference is decided at run time.

## What Changed

- `.expect("msg")` on `Result<T, E>` either gives you the `Ok`
  payload as a plain `T`, or panics with `msg: <Err payload>`.
- A *panic* is a runtime termination. The program already compiled
  and the executable exists; the failure happens while it runs.
- Panic output goes to `stderr`; later `println!` calls never run,
  because the program already exited.
- A panic in `main` exits the process with status `101`. Successful
  runs exit `0`.
- Compile-time errors and runtime panics are categorically different
  failures. Lessons 003-052 trained on compile-time diagnostics;
  today is the first runtime failure mode.

## Check Yourself

You write `tiny.rs`:

```rust
fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 { Ok(n) } else { Err(n) }
}

fn main() {
    let v: i32 = parity(10).expect("need even");
    println!("done: {v}");
}
```

(a) Does `rustc tiny.rs` succeed? What does `./tiny` print?

(b) Change `parity(10)` to `parity(11)`. Does rustc still succeed?
What appears on stdout vs stderr, and what is `echo $?`?

(c) In (b), does the `println!` line ever run?

*(Answers: (a) Yes; stdout `done: 10`; exit `0`. (b) rustc still
succeeds — runtime panic, not compile error. stdout is empty;
stderr has a `thread 'main' ... panicked at tiny.rs:...` block
whose message reads `need even: 11`; `echo $?` prints `101`.
(c) No — the panic happens first, so `println!` is unreached.)*

## What To Ignore For Now

- *`.unwrap()`* — no-message sibling of `.expect("msg")`. Direct
  successor.
- *`.expect`'s real signature* `pub fn expect(self, msg: &str) -> T
  where E: Debug`. The `where E: Debug` says the `Err` payload must
  be printable — that's why `7` appears after the colon. The
  where-clause syntax, `&str` as a typed name, and the `Debug` trait
  are all deferred.
- *Move/consume of `self`* — `.expect` consumes its receiver. For
  `Result<i32, i32>`, both payloads are `Copy`, so the consume is
  invisible. Ownership is deferred.
- *Other Err-side methods* — `.unwrap_err`, `.expect_err`,
  `.unwrap_or`, `.unwrap_or_else`, `.unwrap_or_default`, `.map_err`.
  Future moves.
- *Pattern-matching the payload* — `match r { Ok(v) => ..., Err(e)
  => ... }` is the non-panicking way to consume both sides.
- *The `?` operator* — error propagation without panic.
- *The `panic!` macro itself* — the underlying mechanism `.expect`
  invokes. Today treats panic as observed behavior, not as a thing
  you write.
- *Unwind vs abort panic strategies, `std::panic::catch_unwind`,
  custom panic hooks, the `#[panic_handler]` attribute, no_std
  panic handling, stack unwinding, drop-on-panic*. All deferred.
- *The `RUST_BACKTRACE=1` environment variable* — appears in the
  panic trailer; not a mechanism to learn today.
- *The thread id in parentheses* — an OS thread identifier; varies
  per run; not load-bearing.
- *Recommended `expect` message style* — std docs suggest the
  message describe "the reason you *expect* the Result should be
  Ok"; not a rule today.
- Other `Result` methods deferred since 052 (`.is_err`, `.ok()`,
  `.err()`, `.map`, `.and_then`, etc.).
- *`Option<T>::expect`* — same shape, different enum.

## Evidence

See `../evidence/053-result-expect-and-panic.md`.
