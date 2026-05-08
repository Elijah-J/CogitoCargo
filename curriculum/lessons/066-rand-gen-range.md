---
id: 066-rand-gen-range
status: accepted
evidence: ../evidence/066-rand-gen-range.md
---

# Call `rand::thread_rng().gen_range(1..=100)` to get a random number

## The Move

Lesson 065 added `rand = "0.8.5"` under `[dependencies]`. The crate is
*resolved and built*, but `src/main.rs` did not call it. Today's move
is to use it. In a Cargo package whose `[dependencies]` lists `rand`,
edit `src/main.rs` to read:

```rust
use rand::Rng;

fn main() {
    let n: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Got: {n}");
}
```

Run `cargo run` three times. Each prints `Got: <number>` where the
number is a random integer in `[1, 100]`, and the three numbers are
*different*. That is the load-bearing observation: the call is random.

The `use rand::Rng;` line is the same `use` shape from lesson 044
(`use std::io;`), but the path's first segment is `rand` (an external
crate from `Cargo.toml`) instead of `std`. The Book says this line
"must be in scope for us to use those methods" — treat it as a
required line whose absence breaks the call. What a *trait* is stays
deferred per lesson 040.

## Mental Model Delta

- *Before:* "Lesson 065 added `rand = "0.8.5"` to `[dependencies]` and
  Cargo compiled it. The crate is available but my code doesn't use
  it. `src/main.rs` still prints `Hello, world!`."
- *After:* "I add `use rand::Rng;` at the top of `src/main.rs`
  (lesson 044's `use` shape, pointing at an external crate) and call
  `rand::thread_rng().gen_range(1..=100)` inside `fn main`. Each
  `cargo run` prints a different integer in `[1, 100]`. The `use`
  line is load-bearing — without it rustc fires E0599; the rule that
  requires this import is the trait machinery deferred since cycle 040."

## Prerequisites

- Installed concepts:
  - Lesson 065 (load-bearing): `rand = "0.8.5"` listed under
    `[dependencies]`, with `cargo build` having resolved and compiled
    the crate. Today uses what lesson 065 made available.
  - Lesson 044 (load-bearing): `use Path::final;` brings the final
    segment into scope. Today swaps `std` for `rand` — same `use`
    shape, external crate as the path root.
  - Lesson 040 (load-bearing): the dot-form `receiver.method(args)`.
    Today calls `.gen_range(1..=100)` on what `rand::thread_rng()`
    produces. Trait machinery deferral preserved.
  - Lesson 039 (load-bearing): the inclusive range `1..=100`. Lesson
    039 used it as a `for`-loop range; today it is a method
    *argument*. Lesson 039's *What To Ignore* named
    "`gen_range(1..=100)` and other range arguments to library
    functions" as a future move — this is that move.
  - Lesson 049: method chaining — `a().b()` parses as `(a()).b()`.
  - Lesson 062: `let n: u32 = value;` annotation. The Book uses the
    same `: u32` choice for the random secret.
  - Lesson 063 (capstone): substituted `let secret_number: u32 = 7;`
    for the rand call. Today is the substitution back.
- Ordinary computer-use assumptions: terminal; plain-text editor;
  `cargo` on `PATH`. Internet access is required only on the first
  resolving build per lesson 065.

## Try It

In a directory you can write to, run `cargo new --vcs none
guessing_random && cd guessing_random`. Edit `Cargo.toml` to add
`rand = "0.8.5"` under `[dependencies]` (lesson 065's edit). Then
edit `src/main.rs` to:

```rust
use rand::Rng;

fn main() {
    let n: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Got: {n}");
}
```

Run `cargo run` three times:

```console
$ cargo run
   Compiling rand v0.8.6
   Compiling guessing_random v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.64s
     Running `target/debug/guessing_random`
Got: 71

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_random`
Got: 22

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_random`
Got: 62
```

(The full first-build transcript with all transitive deps is in the
evidence appendix.) Three different numbers, all in `[1, 100]`. That
is the empirical witness for "the call is random."

Walk the program. The `use rand::Rng;` line is required for the
`gen_range` call below to be visible to rustc; the broken-contrast
probe later in this section shows what happens without it. The
right-hand side `rand::thread_rng().gen_range(1..=100)` is lesson
049's chain: free function returning a generator handle, then dot-form
method call (lesson 040) with lesson 039's inclusive range as
argument. Result bound to `n: u32` (lesson 062).

*Now break it.* Comment out or delete the `use rand::Rng;` line, save,
and run `cargo build`:

```console
$ cargo build
error[E0599]: no method named `gen_range` found for struct `ThreadRng` in the current scope
   --> src/main.rs:2:37
    |
  2 |     let n: u32 = rand::thread_rng().gen_range(1..=100);
    |                                     ^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `Rng` which provides `gen_range` is implemented but not in scope; perhaps you want to import it
    |
  1 + use rand::Rng;
    |
```

The `help:` block says it directly: "trait `Rng` which provides
`gen_range` is implemented but not in scope; perhaps you want to
import it", with a source-diff inserting `use rand::Rng;`. Restore
the line; `cargo build` succeeds.

## What Changed

- You can call `rand::thread_rng().gen_range(1..=100)` inside `fn main`
  to get a random integer in `[1, 100]`. Each run produces a
  different number.
- `use rand::Rng;` at the top of `src/main.rs` is the same `use` shape
  from lesson 044, with `rand` replacing `std` as the path root.
- The line is load-bearing. Without it, rustc fires E0599 *no method
  named `gen_range`* and the `help:` block suggests adding exactly
  that line. The rule that explains why `use rand::Rng;` is required
  (trait machinery, deferred since cycle 040) — treat the line as
  required and trust the empirical witness.
- The chain reuses lesson 049 (chaining), lesson 040 (dot form), and
  lesson 039's inclusive range — now as a *method argument* rather
  than a `for`-loop range.
- This completes lesson 063's deferred substitution: the Book's
  `let secret_number = rand::thread_rng().gen_range(1..=100);` at
  ch02 line 1265 is now licensed.

## Check Yourself

You start with a fresh `cargo new playground`, add `rand = "0.8.5"`
under `[dependencies]`, and edit `src/main.rs` to:

```rust
use rand::Rng;

fn main() {
    let dice: u32 = rand::thread_rng().gen_range(1..=6);
    println!("rolled {dice}");
}
```

(a) Does `cargo run` accept the program?

(b) Across three runs, what range will the printed numbers fall in?

(c) If you delete line 1 and rerun `cargo build`, which E-code fires,
and what single line does the `help:` block suggest adding?

*(Answers: (a) Yes — same shape, `1..=6` instead of `1..=100`. (b)
`k` in `[1, 6]`; across three runs values generally differ. (c)
E0599 *no method named `gen_range`*; `help:` suggests `use rand::Rng;`.)*

## What To Ignore For Now

This lesson installs only one move: with `rand` listed under
`[dependencies]`, add `use rand::Rng;` and call
`rand::thread_rng().gen_range(<range>)` to get a random number.
Deferred:

- *What a trait is and what `use rand::Rng;` mechanically does.*
  Trait machinery deferral from lesson 040 carries forward; today
  treats the line operationally.
- *What `ThreadRng` is internally* — thread-local storage, OS RNG
  seeding, generator state.
- *Other rand methods* — `gen`, `gen_bool`, `random`, `fill`,
  `sample`, etc. Today is `gen_range` only.
- *Exclusive ranges as method arguments* — `gen_range(1..100)`
  produces values in `[1, 99]`. Future move.
- *SemVer and version-requirement syntax* (lesson 065's deferral).
- *Reproducible / seeded RNGs* — `SeedableRng`, `ChaCha8Rng`. Future.
- *Probability and distribution semantics.* Today is mechanics, not
  statistics.
- *`rand::Rng::gen_range(...)` qualified-path form* (lesson 041's
  `Type::method` applied to a trait).
- *`cargo doc --open`* — the Book mentions it for browsing crate APIs.
- All previously deferred items.

## Evidence

See `../evidence/066-rand-gen-range.md`.
