# Evidence — 066-rand-gen-range

Audit appendix for `lessons/066-rand-gen-range.md`. The lesson teaches
one move: in a Cargo package whose `[dependencies]` lists `rand` (lesson
065's installed move), write `use rand::Rng;` at the top of
`src/main.rs` and call `rand::thread_rng().gen_range(1..=100)` inside
`fn main` to get a random integer in `[1, 100]`. The empirical
witness for randomness is three `cargo run` invocations producing
three different numbers, all in `[1, 100]`. The empirical witness for
the import being required is an E0599 broken-contrast probe.

This appendix covers (a) toolchain and reproducibility, (b) project
setup (carried over from lesson 065), (c) verbatim probe transcripts
including the three-run randomness witness and the E0599 contrast,
(d) the corpus quote map for Book ch02, (e) the prerequisite-claim
summary, and (f) honesty notes about what is deferred.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-apple-darwin
release: 1.95.0
LLVM version: 22.1.2
```

`uname -sm` -> `Darwin x86_64`. The probe ran in a fresh `mktemp -d`
directory: `/private/var/folders/.../tmp.MrDUPfMY0t/`. Network access
to crates.io was available — the load-bearing first build *requires*
network reach (carried forward from lesson 065).

## Project setup

The committed observation directory
`observations/066-rand-gen-range/` contains exactly the load-bearing
files (matching cycles 064/065 shape):

- `Cargo.toml` — the post-edit manifest with `rand = "0.8.5"` under
  `[dependencies]`. Carried over from lesson 065 verbatim:

  ```toml
  [package]
  name = "guessing_random"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  rand = "0.8.5"
  ```

  The package name differs from lesson 065's (`guessing_random` here
  vs. `hello_dep` in 065). This is incidental — the manifest's
  `[package] name` must match `cargo new`'s argument and is not
  load-bearing for this cycle.

- `src/main.rs` — the post-edit, *post-restore* program. This is the
  load-bearing diff from lesson 065's unchanged `Hello, world!`
  default:

  ```rust
  use rand::Rng;

  fn main() {
      let n: u32 = rand::thread_rng().gen_range(1..=100);
      println!("Got: {n}");
  }
  ```

  Each line maps to a previously installed cycle:
  - Line 1 `use rand::Rng;` — lesson 044's `use Path::final;` form,
    with `rand` (external crate from `[dependencies]`) replacing `std`.
    Load-bearing for this cycle.
  - Line 4 right-hand side `rand::thread_rng().gen_range(1..=100)` —
    lesson 049's method chain: lesson 040's free function
    `rand::thread_rng()` (called via the path-prefixed form) + lesson
    040's dot-form method call `.gen_range(...)` + lesson 039's
    inclusive range `1..=100` as method argument.
  - Line 4 left-hand side `let n: u32 = ...;` — lesson 019 (`let
    name: TYPE = value;`) + lesson 062 (`u32`).
  - Line 5 `println!("Got: {n}");` — lesson 011's named-placeholder
    println.

- `.gitignore` — lists `target/` and `Cargo.lock`. Build artifacts
  excluded per orchestrator directive; the commit is exactly
  `Cargo.toml` + `src/main.rs` + `.gitignore`.

## Probe transcript

Run in `/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.MrDUPfMY0t/`.

### Steps 1-2: Toolchain capture

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ rustc --version --verbose
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-apple-darwin
release: 1.95.0
LLVM version: 22.1.2
```

### Step 3: Fresh package

```text
$ cargo new --vcs none guessing_random
    Creating binary (application) `guessing_random` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

### Steps 4-5: Manifest edit (lesson 065's installed move)

`Cargo.toml` was edited to add `rand = "0.8.5"` under `[dependencies]`,
matching Book ch02 line 492 verbatim. Post-edit:

```toml
[package]
name = "guessing_random"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

### Step 6: `src/main.rs` edit (today's move)

`src/main.rs` was edited from the `cargo new` default to:

```rust
use rand::Rng;

fn main() {
    let n: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Got: {n}");
}
```

### Step 7: First `cargo run` — first resolving build + first random number

```text
$ cargo run
    Updating crates.io index
     Locking 14 packages to latest Rust 1.95.0 compatible versions
      Adding rand v0.8.6 (available: v0.10.1)
   Compiling libc v0.2.186
   Compiling zerocopy v0.8.48
   Compiling cfg-if v1.0.4
   Compiling getrandom v0.2.17
   Compiling rand_core v0.6.4
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.6
   Compiling guessing_random v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.MrDUPfMY0t/guessing_random)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.64s
     Running `target/debug/guessing_random`
Got: 71
exit=0
```

Same resolver-output shape as lesson 065 step 8 (which also resolved
to `rand v0.8.6` and compiled the same eight transitive crates +
`rand` + the package itself). The new tail is `Running ... / Got: 71`
— first random number.

### Step 8: Second and third `cargo run` — randomness witness

```text
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_random`
Got: 22
exit=0

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_random`
Got: 62
exit=0
```

Three runs yielded three different numbers: **71, 22, 62**. All three
fall in `[1, 100]` inclusive. This is the empirical witness for the
lesson's "the call is random" claim. Cache-hit `Finished` line on
runs 2-3 (no `Compiling`) is bit-identical to lesson 064 step 11 and
lesson 065 step 10 — the build is unchanged, only the program's
runtime output differs because the RNG is reseeded each launch.

A fourth confirmatory run after restoring from the broken-contrast
state (step 10 below) yielded **96** — a fourth distinct value, all
four in `[1, 100]`. Not load-bearing beyond corroboration.

### Step 9: Broken-contrast probe — `use rand::Rng;` line removed

`src/main.rs` was edited to remove the first line, leaving:

```rust
fn main() {
    let n: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Got: {n}");
}
```

Then `cargo build`:

```text
$ cargo build
   Compiling guessing_random v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.MrDUPfMY0t/guessing_random)
error[E0599]: no method named `gen_range` found for struct `ThreadRng` in the current scope
   --> src/main.rs:2:37
    |
  2 |     let n: u32 = rand::thread_rng().gen_range(1..=100);
    |                                     ^^^^^^^^^
    |
   ::: /Users/eli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.8.6/src/rng.rs:129:8
    |
129 |     fn gen_range<T, R>(&mut self, range: R) -> T
    |        --------- the method is available for `ThreadRng` here
    |
    = help: items from traits can only be used if the trait is in scope
help: there is a method `gen_ratio` with a similar name, but with different arguments
   --> /Users/eli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.8.6/src/rng.rs:299:5
    |
299 |     fn gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: trait `Rng` which provides `gen_range` is implemented but not in scope; perhaps you want to import it
    |
  1 + use rand::Rng;
    |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `guessing_random` (bin "guessing_random") due to 1 previous error
exit=101
```

This is the load-bearing broken-contrast probe. Five reading-shapes
to notice:

1. **Headline E-code** `error[E0599]: no method named `gen_range`
   found for struct `ThreadRng` in the current scope` — different
   E-code from lesson 044's E0425 ("cannot find function") because
   the *receiver* (the `ThreadRng` value) is in scope; only the
   *method* is missing. The lesson body names this contrast.
2. **Caret pointer** under `gen_range` (column 37 of line 2)
   pinpoints the method-name token that is unresolvable.
3. **Cross-file pointer** `:::` to `rand-0.8.6/src/rng.rs:129:8`
   showing where the method is defined — `fn gen_range<T, R>(&mut
   self, range: R) -> T` — with the under-line annotation "the
   method is available for `ThreadRng` here". This is rustc telling
   us "the method exists; it's just not visible from here." The
   trait-machinery rule that explains *why* visibility is gated
   stays deferred per lesson 040.
4. **Top-level help** `= help: items from traits can only be used if
   the trait is in scope`. This is rustc's audience-level statement
   of the rule that the lesson's prose deliberately does *not*
   restate in lesson voice (per cycle 040's name-only deferral
   pattern). The verbatim diagnostic appears in the *Try It* probe
   transcript only; the lesson's *Mental Model Delta* and *What
   Changed* point at the deferral by name without restating the
   rule.
5. **Source-diff help** (the load-bearing line for the lesson's
   "rustc itself suggests adding `use rand::Rng;`" claim):
   ```
   help: trait `Rng` which provides `gen_range` is implemented but not in scope; perhaps you want to import it
       |
     1 + use rand::Rng;
       |
   ```
   rustc proposes inserting *exactly* the line the lesson teaches.
   This is the empirical evidence that the trait import is what
   makes the method visible — without unpacking what a trait *is*.

The companion `help: there is a method `gen_ratio` with a similar
name` is rustc's name-similarity suggestion. Not load-bearing for
this cycle (the lesson is about `gen_range`, not `gen_ratio`); the
appendix shows the full diagnostic for fidelity.

### Step 10: Restore `use rand::Rng;`, `cargo build` succeeds

```text
$ cargo build
   Compiling guessing_random v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.MrDUPfMY0t/guessing_random)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
exit=0
```

The same package, same dependency, same `gen_range` call — only the
single `use rand::Rng;` line differs. Without it: E0599. With it:
clean compile. This is the cleanest possible "load-bearing line"
evidence pair.

A confirmatory `cargo run` after the restore printed `Got: 96` (exit
0) — the program runs, prints a random number in `[1, 100]`, and the
fourth distinct value across the cycle's runs.

## Corpus quote map

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md` — primary source

**Lines 660-682** — the Listing 2-3 program text (load-bearing for
the lesson's exact `use rand::Rng;` and `rand::thread_rng().gen_range(1..=100)` lines):

> Filename: src/main.rs
>
> ```rust
> use std::io;
>
> use rand::Rng;
>
> fn main() {
>     println!("Guess the number!");
>
>     let secret_number = rand::thread_rng().gen_range(1..=100);
>     ...
> ```

The Book uses `use rand::Rng;` (line 663) and
`rand::thread_rng().gen_range(1..=100)` (line 668) verbatim — the
lesson's exact strings.

**Lines 686-688** — the load-bearing audience-level grounding for
*why* `use rand::Rng;` is required (the central corpus quote for
this cycle):

> First, we add the line `use rand::Rng;`. The `Rng` trait defines
> methods that random number generators implement, and this trait
> must be in scope for us to use those methods. Chapter 10 will
> cover traits in detail.

This single sentence licenses the lesson's:
- "treat it as a required line whose absence breaks the call"
- "What a trait is stays deferred per lesson 040" (the Book itself
  defers traits to chapter 10; the lesson defers them indefinitely)
- the *Mental Model Delta* name-only deferral ("the rule that
  requires this import is the trait machinery deferred since cycle
  040") — name-only per cycle 040's pattern, no rule paraphrase
- the *What To Ignore For Now* trait-machinery deferral

**Lines 690-699** — the load-bearing audience-level grounding for
the *call* (`rand::thread_rng().gen_range(1..=100)`):

> Next, we’re adding two lines in the middle. In the first line, we
> call the `rand::thread_rng` function that gives us the particular
> random number generator we’re going to use: one that is local to
> the current thread of execution and is seeded by the operating
> system. Then, we call the `gen_range` method on the random number
> generator. This method is defined by the `Rng` trait that we
> brought into scope with the `use rand::Rng;` statement. The
> `gen_range` method takes a range expression as an argument and
> generates a random number in the range. The kind of range
> expression we’re using here takes the form `start..=end` and is
> inclusive on the lower and upper bounds, so we need to specify
> `1..=100` to request a number between 1 and 100.

This paragraph grounds three lesson-body claims at once:
1. `rand::thread_rng()` is a function that returns "the particular
   random number generator" — the lesson's "the random-number-
   generator handle for this thread" framing. The Book's "local to
   the current thread of execution and is seeded by the operating
   system" extra detail is deliberately deferred (today the lesson
   says "thread" without unpacking thread-local storage).
2. `gen_range` is a method on the generator, defined by the `Rng`
   trait. The lesson uses lesson 040's dot-form for the call and
   keeps the trait-source as a deferred topic.
3. `1..=100` is the inclusive-range argument shape and produces a
   number "between 1 and 100" — connecting cleanly to lesson 039's
   inclusive-range install, now used as a method argument.

**Lines 722-744** — the Book's verbatim transcript of two runs of
the program, showing different secret numbers each time:

> ```console
> $ cargo run
>    Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
>      Running `target/debug/guessing_game`
> Guess the number!
> The secret number is: 7
> Please input your guess.
> 4
> You guessed: 4
>
> $ cargo run
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
>      Running `target/debug/guessing_game`
> Guess the number!
> The secret number is: 83
> Please input your guess.
> 5
> You guessed: 5
> ```
>
> You should get different random numbers, and they should all be
> numbers between 1 and 100.

The Book's two runs printed `7` and `83` as the secret. The lesson's
probe printed `71`, `22`, `62`, (and `96` on the calibration restore)
— different numbers, all in `[1, 100]`. The Book's prose "You should
get different random numbers, and they should all be numbers between
1 and 100" grounds the lesson's load-bearing randomness witness.

**Lines 1256-1293** — Listing 2-6 (the Book's *final* code, captured
in lesson 063 as the capstone target). Lines 1260 and 1265 are the
lesson's deferred-substitution origin:

> ```rust
> use std::cmp::Ordering;
> use std::io;
>
> use rand::Rng;                                    [line 1260]
>
> fn main() {
>     println!("Guess the number!");
>
>     let secret_number = rand::thread_rng().gen_range(1..=100);  [line 1265]
> ```

Lesson 063 substituted `let secret_number: u32 = 7;` for line 1265
to avoid `rand`. Today's cycle replaces the substitution with the
Book's actual line, in a stand-alone *minimal* program (one `let n:
u32 = ...;` plus `println!`) rather than the full guessing-game.
The full Listing 2-6 verbatim composition is named in `unlocks` as
the "second capstone."

### `output/docs/rust/std/keyword.use.md` — supporting

Lesson 044 already cites this for the general `use` keyword
behavior. Today's cycle reuses that grounding by reference: the
syntactic shape `use Path::final;` is identical, only the path's
first segment changes (`std` → `rand`).

### `output/docs/rust/error_codes/E0599.md` (if present in corpus)

The broken-contrast probe fires `error[E0599]: no method named
`gen_range` found for struct `ThreadRng` in the current scope`. The
lesson body cites the E-code by number and quotes the verbatim
headline + `help:` block from the probe transcript. Whether the
explainer page is in the corpus is incidental — the diagnostic
itself is the empirical evidence, and the load-bearing prose ("items
from traits can only be used if the trait is in scope" + "trait
`Rng` which provides `gen_range` is implemented but not in scope;
perhaps you want to import it") comes verbatim from rustc, not from
a corpus page.

## Prerequisite-claim summary

### Lesson 065 (`065-cargo-toml-dependencies-entry`) — *direct, load-bearing*

- Adding `<crate-name> = "<version-string>"` under `[dependencies]`
  in `Cargo.toml` and running `cargo build` causes Cargo to fetch,
  resolve, and compile the named crate (and its transitive deps).
  Today reuses the exact `rand = "0.8.5"` line and the same first-
  build resolver-output shape. The lesson body says "lesson 065's
  edit" without re-explaining the resolver lines.
- Lesson 065 explicitly noted that today's program does NOT yet
  import or call `rand`; its `unlocks` named "import the now-
  resolved external crate with `use rand::Rng;` and call its API
  like `rand::thread_rng().gen_range(1..=100)` — the immediate next
  cycle, completing the deferred substitution from cycle 063" as
  this exact move.

### Lesson 044 (`044-use-declaration`) — *direct, load-bearing*

- A top-level `use Path::final;` line brings the final segment into
  the file's scope. The path's *root segment* is `std::` in lesson
  044. Today the path root is `rand::` — an external crate listed
  under `[dependencies]`. Lesson 044's *What To Ignore* listed
  "`use crate::`, `use self::`, `use super::`" as deferred non-
  absolute path roots; an *external-crate* path root like `rand::`
  is the natural sibling of `std::` and reuses the same syntactic
  rule.

### Lesson 040 (`040-method-call-syntax`) — *direct, load-bearing*

- The dot form `receiver.method(args)`. Today calls `.gen_range(1..=100)`
  on what `rand::thread_rng()` produces — same syntactic shape, with
  a chained-call expression as the receiver (legal per lesson 049).
- Lesson 040 explicitly deferred trait machinery; today preserves
  that deferral despite naming the `Rng` trait once as a deferred
  noun.

### Lesson 039 (`039-inclusive-range`) — *direct, load-bearing*

- The inclusive range `1..=100` — `=` between the dots and `100`
  includes the upper bound. Lesson 039 used the form as a *for-loop
  range*; today uses it as a *method argument* to `gen_range`. The
  syntactic form is identical; only the syntactic position differs.
  Lesson 039's *What To Ignore* explicitly named "`gen_range(1..=100)`
  and other range arguments to library functions" as a future move
  — this cycle is that future move.

### Lesson 049 (`049-method-chaining`) — *load-bearing*

- A method-call's receiver is *any expression*, so two `.method()`
  calls (or a free function followed by a method) can be written
  end-to-end. Today's chain
  `rand::thread_rng().gen_range(1..=100)` is a free function call
  followed by a method call — same left-associative grouping,
  `(rand::thread_rng()).gen_range(1..=100)`.

### Lesson 062 (`062-u32-unsigned-integer`) — *direct, supporting*

- `let n: u32 = value;` annotates the binding's type. The Book uses
  this exact `: u32` choice in the guessing-game (`let guess: u32`
  at ch02 line 1276, and the lesson 063 capstone's `let
  secret_number: u32 = 7;`). Today reuses the form for `let n: u32
  = rand::thread_rng().gen_range(1..=100);`. The annotation also
  pins what type `gen_range` returns; the Book's signature `fn
  gen_range<T, R>(&mut self, range: R) -> T` makes `T` inferable
  from context, and the `: u32` annotation drives that inference.
  This is the same flow-back rule cycles 056 and 057 captured for
  `parse()`. Today does *not* re-teach flow-back; it just notes
  the `: u32` is the Book's choice.

### Lesson 063 (`063-capstone-guessing-game-no-rand`) — *direct, narrative*

- Lesson 063 substituted `let secret_number: u32 = 7;` for the
  Book's `let secret_number = rand::thread_rng().gen_range(1..=100);`
  to avoid `rand`. Today's cycle is the substitution back. The
  lesson body explicitly names this ("This completes lesson 063's
  deferred substitution"). Lesson 063's `unlocks` named:
  - "future 'the rand external crate (`rand::thread_rng().gen_range(1..=100)`),
    Cargo.toml `[dependencies]`, the `rand::Rng` trait, and
    `cargo build` — the substitution replaced today' moves"
  - "future 'the FULL Book guessing-game program with `rand`
    (Listing 2-6 verbatim, no substitution)' moves"
  Today fulfills the first; the second is a future capstone (named
  in today's `unlocks`).

### Lessons 001, 002, 011, 019, 005 — *supporting*

Mentioned by number/title only, carrying forward through lesson 062
and 065's prerequisite chains. None re-load-bearing here.

## Contrast-probe coverage

The lesson's contrastive structure has two layers, both empirical:

1. **`use rand::Rng;` present vs. absent** (steps 7-8 vs. step 9 vs.
   step 10). Same package, same `[dependencies]`, same `gen_range`
   call. With the line: clean compile, three random numbers across
   three runs. Without the line: E0599 with the `help:` block
   suggesting *exactly* `use rand::Rng;`. Restoring the line: clean
   compile again. This is the **load-bearing broken contrast** for
   the lesson.

2. **First resolving build vs. cache-hit rebuild** (step 7 vs. steps
   8 and 10). Same as lesson 065's contrast — already established
   shape, reused unchanged. The lesson body does not re-teach this;
   the appendix preserves the transcripts for fidelity.

A *third* contrast worth noting (not load-bearing, mentioned for
completeness): same program but with `secret_number: i32` instead
of `u32` would still compile and produce `i32` values; the
`gen_range`'s `T` is generic and the annotation pins it. The lesson
defers this calibration (lesson 062 already installed `: u32`'s
sufficiency for this cycle; the type-inference flow-back is
implicit).

## Honesty notes

1. **Trait machinery name-only-deferred.** Following cycle 040's
   pattern, the lesson does *not* state the trait-method-visibility
   rule in lesson voice. *The Move* names "trait" once as a deferred
   noun ("What a *trait* is stays deferred per lesson 040"). *Mental
   Model Delta* and *What Changed* point at the deferral by name
   without restating the rule. The verbatim rustc diagnostic — `help:
   items from traits can only be used if the trait is in scope` and
   the source-diff `help: trait `Rng` which provides `gen_range` is
   implemented but not in scope; perhaps you want to import it` —
   appears in the *Try It* probe transcript as corpus quotes; the
   lesson cites the help-block text in *Try It* as a corpus-attributed
   quote ("The `help:` block says it directly: ..."). The Book itself
   defers traits to chapter 10 ("Chapter 10 will cover traits in
   detail" — line 688); today defers them indefinitely. The empirical
   witness — the E0599 broken-contrast probe — is enough to ground
   "the line is required" without trait noun installation. Carries
   lesson 040's deferral forward.

2. **`ThreadRng` named once, internals deferred.** The lesson says
   "the random-number-generator handle for this thread" once. The
   Book's deeper "local to the current thread of execution and is
   seeded by the operating system" framing is documented in the
   corpus quote map but not surfaced in the lesson body. Thread-
   local storage, OS RNG seeding, and `ThreadRng` vs. `OsRng` vs.
   `StdRng` are explicitly deferred.

3. **Version drift inherited from lesson 065.** Manifest line
   `rand = "0.8.5"`; resolver picked `rand v0.8.6`. Same drift as
   lesson 065 step 8; SemVer rule deferred to a future cycle.
   Honest reporting only.

4. **Probe randomness is real.** The four numbers across the cycle's
   runs (71, 22, 62, 96) are genuine RNG output from
   `rand::thread_rng().gen_range(1..=100)` on probe day; they are
   not curated for distribution properties. The lesson body does
   not claim uniformity, only that "each run prints a different
   number" — the load-bearing observation. Probability and
   distribution semantics are explicitly deferred.

5. **`src/main.rs` differs from lesson 065's.** Lesson 065's
   committed `src/main.rs` was `cargo new`'s default `Hello,
   world!` program. Today's committed `src/main.rs` is the
   five-line program above. This is the central diff between the
   two cycles.

## Files committed for this cycle

- `lessons/066-rand-gen-range.md` (this lesson)
- `evidence/066-rand-gen-range.md` (this appendix)
- `observations/066-rand-gen-range/Cargo.toml` (post-edit; matches
  lesson 065's manifest verbatim except for the `name` field)
- `observations/066-rand-gen-range/src/main.rs` (the load-bearing
  five-line program)
- `observations/066-rand-gen-range/.gitignore` (excludes `target/`
  and `Cargo.lock`)
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
