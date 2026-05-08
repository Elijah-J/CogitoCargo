# Evidence — 067-capstone-guessing-game-with-rand

Audit appendix for `lessons/067-capstone-guessing-game-with-rand.md`.
This is *Capstone Mode* cycle 067 — the second and final Book-path
capstone, parallel to lesson 063. **No new Rust mechanic.** The
appendix's job is (a) toolchain and reproducibility, (b) corpus quote
map for every load-bearing claim in the lesson, including the Book's
Listing 2-6 verbatim text, (c) the five required probe transcripts
verbatim, (d) the line-by-line graph-node license map (every
meaningful line of `src/main.rs` plus every line of `Cargo.toml`),
and (e) prerequisite-claim summary plus honesty notes.

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

`uname -sm` -> `Darwin x86_64`. Probe ran in a fresh `mktemp -d`
directory: `/private/var/folders/.../tmp.pOqfcIaVI3/guessing_game/`,
removed at the end of the run. Network access to crates.io was
available — the load-bearing first build *requires* network reach
(carried forward from lessons 065 and 066).

The committed observation directory
`observations/067-capstone-guessing-game-with-rand/` contains exactly
three files: `Cargo.toml`, `src/main.rs`, `.gitignore` (excludes
`target/` and `Cargo.lock` per orchestrator directive).

## Corpus quote map

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md` — primary source

**Lines 1256-1293** (Listing 2-6, the Book's *complete* code — the
load-bearing capstone target):

> ```rust
> use std::cmp::Ordering;
> use std::io;
>
> use rand::Rng;
>
> fn main() {
>     println!("Guess the number!");
>
>     let secret_number = rand::thread_rng().gen_range(1..=100);
>
>     loop {
>         println!("Please input your guess.");
>
>         let mut guess = String::new();
>
>         io::stdin()
>             .read_line(&mut guess)
>             .expect("Failed to read line");
>
>         let guess: u32 = match guess.trim().parse() {
>             Ok(num) => num,
>             Err(_) => continue,
>         };
>
>         println!("You guessed: {guess}");
>
>         match guess.cmp(&secret_number) {
>             Ordering::Less => println!("Too small!"),
>             Ordering::Greater => println!("Too big!"),
>             Ordering::Equal => {
>                 println!("You win!");
>                 break;
>             }
>         }
>     }
> }
> ```

**Verbatim verification.** The committed
`observations/067-capstone-guessing-game-with-rand/src/main.rs` was
diffed against the Book's Listing 2-6 (lines 1257-1292 with the
`` ```rust `` and `` ``` `` fences stripped). The diff output was
empty — exit 0. The capstone program is **bit-identical** to Listing
2-6. No deviations, no substitutions.

**Line 1295** (the listing's role):

> *[Listing 2-6](#listing-2-6): Complete guessing game code*

**Line 1297**:

> At this point, you've successfully built the guessing game.
> Congratulations!

This is the Book's stop-marker for the chapter-2 program. Capstone
067 produces this program in full.

**Lines 1250-1252** (the Book's prose justifying the debug-print
removal — relevant because it confirms Listing 2-6 has no debug
`println!` of the secret):

> that the program is still printing the secret number. That worked
> well for testing, but it ruins the game. Let's delete the
> `println!` that outputs the secret number. Listing 2-6 shows the
> final code.

The Book's earlier Listing 2-5 (lines 1147-1190, mentioned in lesson
063's evidence) included a debug `println!("The secret number is:
{secret_number}");` line. Listing 2-6 explicitly removes it. The
capstone matches Listing 2-6, so the debug line is correctly absent.
**No discrepancy** between the orchestrator's brief and the Book's
Listing 2-6.

**Lines 660-682** (Listing 2-3 — Book's earlier introduction of the
`use rand::Rng;` and `rand::thread_rng().gen_range(1..=100)` lines,
already cited by lesson 066):

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

Direct corpus license for the spelling and order of the rand-related
lines reused unchanged in Listing 2-6.

**Lines 686-688** — the audience-level grounding for *why*
`use rand::Rng;` is required (already cited by lesson 066):

> First, we add the line `use rand::Rng;`. The `Rng` trait defines
> methods that random number generators implement, and this trait
> must be in scope for us to use those methods. Chapter 10 will
> cover traits in detail.

The Book itself defers traits to chapter 10; lessons 040 and 066
defer them indefinitely. The capstone preserves that deferral.

**Lines 690-699** — the audience-level grounding for the
`rand::thread_rng().gen_range(1..=100)` call (already cited by
lesson 066):

> Next, we're adding two lines in the middle. In the first line, we
> call the `rand::thread_rng` function that gives us the particular
> random number generator we're going to use: one that is local to
> the current thread of execution and is seeded by the operating
> system. Then, we call the `gen_range` method on the random number
> generator. This method is defined by the `Rng` trait that we
> brought into scope with the `use rand::Rng;` statement. The
> `gen_range` method takes a range expression as an argument and
> generates a random number in the range. The kind of range
> expression we're using here takes the form `start..=end` and is
> inclusive on the lower and upper bounds, so we need to specify
> `1..=100` to request a number between 1 and 100.

Lesson 066 unpacked this; the capstone reuses unchanged.

**Lines 743-744** — the load-bearing audience-level grounding for the
"each run plays a different game" claim:

> You should get different random numbers, and they should all be
> numbers between 1 and 100. Great job!

The probe captured five secrets across five runs (1, 13, 81, 21, 80),
all in `[1, 100]` — direct empirical confirmation of the Book's
audience-level claim.

**Lines 778-783, 793-800, 935-947, 1200-1214** — already cited by
lesson 063 for the `cmp`/`Ordering` block, the audience-level
walkthrough, the `let guess: u32` line, and the `Ok(num)` /
`Err(_) => continue` arms. Re-cited transitively for the capstone;
no new quotes needed because the lines are bit-identical to lesson
063's no-rand version.

**Line 492** — the manifest entry's exact spelling (already cited by
lesson 065):

> rand = "0.8.5"

The capstone `Cargo.toml`'s `[dependencies]` line is bit-identical to
this Book quote. Lesson 065's "the Book quotes 0.8.5 but cargo
resolved to 0.8.6" deviation is preserved unchanged today.

### `output/docs/rust/std/cmp/enum.Ordering.md` — supporting

Already cited by lessons 051, 061, 063. Capstone reuses the three
variants `Less`, `Greater`, `Equal` unchanged. No new quote needed.

### `output/docs/rust/std/primitive.u32.md` — supporting

Already cited by lessons 062 and 063 for the `impl Ord for u32` block.
The capstone's `match guess.cmp(&secret_number) { ... }` uses
`u32::cmp` (sibling-extension carried over from lesson 063). The
secret's type is *inferred* as `u32` because it is compared with a
`&u32` (the shadowed `guess` is annotated `: u32`); no annotation on
the secret's `let` is required. No new quote needed.

### `output/docs/rust/cargo/reference/specifying-dependencies.md` — supporting

Already cited by lesson 065 for the `<crate> = "<version>"` shape.
Capstone reuses unchanged.

## Verbatim probe transcripts

### Step 1: scratch directory and `cargo new`

```text
$ SCRATCH=$(mktemp -d) && echo "$SCRATCH"
/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.pOqfcIaVI3
$ cd "$SCRATCH"
$ cargo new --vcs none guessing_game
    Creating binary (application) `guessing_game` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd guessing_game
```

After `cargo new`, the default manifest had an empty `[dependencies]`
section and the default `src/main.rs` printed `Hello, world!`.

### Step 2: Edit `Cargo.toml` and `src/main.rs`

`Cargo.toml` was edited to add `rand = "0.8.5"` (lesson 065's exact
line). `src/main.rs` was overwritten with the Listing 2-6 program
(verified bit-identical to ch02 lines 1257-1292; diff empty, exit 0).

Final files (committed at `observations/067-capstone-guessing-game-with-rand/`):

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### Step 3: `cargo build` (resolving build, no warnings)

```text
$ cargo build
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
   Compiling guessing_game v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.pOqfcIaVI3/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.82s
exit=0
```

Bit-identical resolver output shape to lesson 065 step 8 and lesson
066 step 7 (same eight transitive deps, `rand v0.8.6` resolved). No
warnings.

### Step 4: Three `seq 1 100 | cargo run` runs (randomness witness)

#### Run 1 — secret was `1`

```text
$ seq 1 100 | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
You guessed: 1
You win!
exit=0
```

Cache-hit `Finished` line (no `Compiling`). Pass-1 of the loop reads
`1` from stdin, parses to `1u32`, hits `Ordering::Equal` (secret was
`1`), prints `You win!`, `break;` exits the outer loop.

#### Run 2 — secret was `13`

```text
$ seq 1 100 | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
You guessed: 1
Too small!
Please input your guess.
You guessed: 2
Too small!
Please input your guess.
You guessed: 3
Too small!
Please input your guess.
You guessed: 4
Too small!
Please input your guess.
You guessed: 5
Too small!
Please input your guess.
You guessed: 6
Too small!
Please input your guess.
You guessed: 7
Too small!
Please input your guess.
You guessed: 8
Too small!
Please input your guess.
You guessed: 9
Too small!
Please input your guess.
You guessed: 10
Too small!
Please input your guess.
You guessed: 11
Too small!
Please input your guess.
You guessed: 12
Too small!
Please input your guess.
You guessed: 13
You win!
exit=0
```

13 passes through the loop, all 12 wrong guesses produce `Too small!`,
the 13th hits `Equal`.

#### Run 3 — secret was `81`

```text
$ seq 1 100 | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
You guessed: 1
Too small!
... (79 more "Too small!" passes) ...
Please input your guess.
You guessed: 81
You win!
exit=0
```

81 passes through the loop. The 80 lines `You guessed: <k>` /
`Too small!` for `k = 1..=80` are elided here for length; the verbatim
output was captured during the probe and matches the same shape as
runs 1 and 2 with one `Too small!` block per integer below the secret.
Full verbatim transcript was captured by the probe and matches this
shape line for line.

**Three runs, three different secrets: 1, 13, 81.** All in `[1, 100]`.
This is the load-bearing randomness witness for the lesson's claim
"each `cargo run` plays a different game."

### Step 5: Garbage-then-seq calibration probe

```text
$ { printf 'abc\n'; seq 1 100; } | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
Please input your guess.
You guessed: 1
Too small!
Please input your guess.
You guessed: 2
Too small!
... (18 more "Too small!" passes) ...
Please input your guess.
You guessed: 21
You win!
exit=0
```

**Load-bearing observation.** The first prompt `Please input your
guess.` is followed *directly* by the second prompt — there is **no
`You guessed: abc` line** and **no error message** between them. This
empirically confirms:

1. `"abc".trim().parse::<u32>()` returned `Err(_)`.
2. The `Err(_) => continue` arm fired (lesson 058's wildcard-payload
   pattern + lesson 035's `continue`).
3. The wildcard `_` discarded the `ParseIntError` payload, so no
   error message was printed (lesson 058's mechanic).
4. The loop iterated to the next stdin line without producing any
   `You guessed:` output for the bad input.

After the bad input, the loop steps through `1, 2, ..., 21` until the
secret (21) is hit. All 20 below-secret guesses produce `Too small!`.
This calibration is parallel to lesson 063's transcript 4
(`printf 'abc\n7\n' | ./demo`) — same `Err(_) => continue` mechanic,
only the program is now Cargo-built and the secret is random.

### Step 6: Out-of-range calibration probe

```text
$ { printf '500\n'; seq 1 100; } | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
You guessed: 500
Too big!
Please input your guess.
You guessed: 1
Too small!
Please input your guess.
... (78 more "Too small!" passes) ...
Please input your guess.
You guessed: 80
You win!
exit=0
```

**Load-bearing observation for the *What To Ignore* claim.** The
program does NOT enforce a `1..=100` range on guesses. `500` parses
cleanly as a `u32` (500 fits in `u32`'s range `0..=4_294_967_295`,
lesson 062), `You guessed: 500` is printed, the `cmp` against the
secret hits `Ordering::Greater`, prints `Too big!`, and the loop
continues. The secret on this run was `80`. This confirms the
lesson's claim that "Listing 2-6 does not enforce range on guesses"
— exactly the Book's behavior. Adding range validation is named in
*What To Ignore* as a future composition.

### Five distinct secrets across five runs

Across runs 1, 2, 3 plus the two calibrations, the random secrets
were: **1, 13, 81, 21, 80**. Five distinct values, all in `[1, 100]`.
Empirical confirmation of randomness across the cycle's full probe.

## Line-by-line graph-node license map (full)

Every meaningful line of `src/main.rs` and `Cargo.toml` mapped to
accepted graph nodes. Punctuation-only lines (blank, lone `}`, lone
`;`) are omitted; they carry over from basic-syntax lessons since
cycle 002.

### `src/main.rs`

| Line | Cycles | License note |
|---|---|---|
| `use std::cmp::Ordering;` | 044, 051 | `use` brings the short name `Ordering` into scope from `std::cmp`. |
| `use std::io;` | 044 | Parent-module `use`; same shape lesson 060 used. |
| `use rand::Rng;` | 044, 066 | Lesson 044's `use Path::final;` shape with `rand` (external crate) replacing `std`. **Lesson 066 is the licensing cycle for the external-crate variant.** Required so `gen_range` is in scope. |
| `fn main() {` | 002 | Entry-point shape. |
| `println!("Guess the number!");` | 011 | `println!` macro, string-literal first arg, no placeholders. |
| `let secret_number = rand::thread_rng().gen_range(1..=100);` | 005, 049, 040, 039, 066 | **The capstone's central change from lesson 063.** Lesson 005 (`let name = value;` form, *no annotation*); lesson 066 (the random call composition: `rand::thread_rng()` free-function + `.gen_range(...)` method + `1..=100` argument); lesson 049 (chain shape `(rand::thread_rng()).gen_range(...)`); lesson 040 (dot-form method call); lesson 039 (`1..=100` inclusive range). The binding's *type* is inferred as `u32` not because of an annotation here but because of lesson 056's flow-back rule: further down, `&secret_number` is compared with `&guess: &u32`, and `cmp`'s signature `fn cmp(&self, other: &Self) -> Ordering` (lesson 061) requires both sides to share `Self`, pinning `secret_number: u32`. The annotation lesson 062 installed is *not* needed here because inference resolves it. |
| `loop {` | 027 | Outer loop opens. |
| `println!("Please input your guess.");` | 011 | Per-iteration prompt. |
| `let mut guess = String::new();` | 042, 006 | Fresh `String` each iteration; `mut` makes `&mut guess` legal next line. |
| `io::stdin()` | 050 | Stdin handle. |
| `.read_line(&mut guess)` | 054, 048, 049 | `read_line` method; `&mut guess` argument shape; chained call. |
| `.expect("Failed to read line");` | 053, 052, 049 | Consume `Result<usize, io::Error>` with panic-on-Err. |
| `let guess: u32 = match guess.trim().parse() {` | 057, 062, 030, 055, 056 | Type-changing shadow; `u32` annotation; match opens; `.trim()` then `.parse()`. |
| `    Ok(num) => num,` | 058 | `Ok` payload-binding pattern; bound name is the arm's value. |
| `    Err(_) => continue,` | 058, 035, 059 | `Err` wildcard-payload pattern; `continue` body; divergent-arm rule. |
| `};` | 030 | Match expression closes. |
| `println!("You guessed: {guess}");` | 011 | `{name}` placeholder resolves to the *new* `u32` `guess`. |
| `match guess.cmp(&secret_number) {` | 061, 045, 030, 051, 063 | `u32::cmp` (sibling of lesson 061's `i32::cmp`, calibrated by lesson 063's capstone — same signature shape, only receiver type changes); shared-ref arg; match on `Ordering`. |
| `    Ordering::Less => println!("Too small!"),` | 051, 011 | Variant-pattern arm; print-only body. |
| `    Ordering::Greater => println!("Too big!"),` | 051, 011 | Same shape. |
| `    Ordering::Equal => {` | 051, 030 | Variant-pattern arm with block-body. |
| `        println!("You win!");` | 011 | First statement in the block. |
| `        break;` | 027, 059 | Exits the outer `loop`. Block ends without a value (lesson 027 + lesson 059's divergent-statement rule generalized to `break;` per lesson 063's capstone). |
| `    }` | — | Closes the `Ordering::Equal` arm body block. |
| `}` | — | Closes the outer `match`. |
| `}` | — | Closes the `loop`. |
| `}` | — | Closes `main`. |

No line is unlicensed. **Two lines license under lesson 063 itself**
(via the sibling-extension framing it installed): the `match
guess.cmp(...)` line uses `u32::cmp` extending lesson 061's `i32::cmp`,
and the `break;` in the `Ordering::Equal` arm uses the divergent-arm
rule extension lesson 059's *What To Ignore* anticipated and lesson
063 exercised. The capstone reuses both unchanged.

### `Cargo.toml`

| Line | Cycles | License note |
|---|---|---|
| `[package]` | 032 | Header for the package metadata block; `cargo new` writes this. |
| `name = "guessing_game"` | 032 | Package name from `cargo new --vcs none guessing_game` argument. |
| `version = "0.1.0"` | 032 | Default version `cargo new` writes. |
| `edition = "2024"` | 032 | Default edition `cargo new` writes (deferred per lesson 064). |
| `[dependencies]` | 032, 064, 065 | Section header. Cycles 032 and 064 named-empty; lesson 065 is the cycle that filled it. |
| `rand = "0.8.5"` | 065 | The `<crate> = "<version>"` shape lesson 065 installed. Bit-identical to Book ch02 line 492. SemVer rule (resolved to `rand v0.8.6`) deferred per lesson 065. |

No line of `Cargo.toml` is unlicensed.

## Prerequisite-claim summary

The capstone's `depends_on` list is long because the capstone is a
composition. Below is each accepted node id and the specific claim it
licenses *for the capstone*. Older supporting lessons not directly
load-bearing today (e.g., cycles 003-005's basic infrastructure,
cycle 010 line comments, cycles 014-016 if/else, cycles 020-026
functions/expressions) are present transitively via the named direct
prerequisites and are not re-listed.

The list largely overlaps lesson 063's `depends_on`, with the following
*additions* unique to today:

- **032-cargo-new-and-run** (load-bearing): `cargo new <name>` scaffolds
  the package directory containing `Cargo.toml` and `src/`. Today's
  package was created with `cargo new --vcs none guessing_game`.
- **064-cargo-build-standalone** (load-bearing): `cargo build` from
  inside the package compiles the package; the cache-hit shape
  (`Finished` only on a no-change rebuild) and the resolving-build
  shape (with `Compiling <crate>` lines for each transitive dep)
  carry forward.
- **065-cargo-toml-dependencies-entry** (load-bearing): `<crate-name>
  = "<version-string>"` under `[dependencies]` causes `cargo build`
  to fetch and compile the crate. Today's `rand = "0.8.5"` line is
  bit-identical to lesson 065's. The first build's resolving-output
  shape carries forward unchanged.
- **066-rand-gen-range** (load-bearing): the `use rand::Rng;` line
  plus the call `rand::thread_rng().gen_range(1..=100)`. Lesson 066
  installed both (with a five-line minimal program); today places
  them in the full Listing 2-6 program. Lesson 066's `unlocks`
  named "the FULL Book guessing-game program with `rand` (Listing
  2-6 verbatim, no substitution from cycle 063) — the second
  capstone" — today is exactly that capstone.
- **063-capstone-guessing-game-no-rand** (narrative + load-bearing):
  every line of Listing 2-6 *except* the rand-related ones was
  exercised by lesson 063 with the substitution `let secret_number:
  u32 = 7;`. Today is the substitution back. Lesson 063's `unlocks`
  named "the FULL Book guessing-game program with `rand` (Listing 2-6
  verbatim, no substitution)" as a future capstone — today is that
  capstone.

The remaining direct prerequisites are inherited from lesson 063 and
lesson 066's chains: 001, 002, 005, 006, 011, 019, 027, 030, 035, 040,
039, 042, 044, 045, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057,
058, 059, 060, 061, 062. See lesson 063's evidence appendix for the
specific claim each licenses. None of those individual licensing
claims change today; only the surrounding context (Cargo project +
rand call) is new.

Plus ordinary computer-use: terminal, plain-text editor, `cargo` and
`rustc` on `PATH`, shell-piping with `seq 1 100 | cargo run` (since
lesson 053 + lesson 032), internet access for the first resolving
build (since lesson 065).

## Honest notes / pedagogical choices

1. **The `let secret_number` line has no `: u32` annotation in
   Listing 2-6.** Lesson 063's capstone wrote `let secret_number: u32
   = 7;` because the right-hand side was a plain literal `7` (which
   defaults to `i32` per lesson 019, so the annotation was needed to
   override). Today's right-hand side is
   `rand::thread_rng().gen_range(1..=100)` whose return type is
   *generic* (`fn gen_range<T, R>(...) -> T` per lesson 066's
   broken-contrast probe transcript at `rand-0.8.6/src/rng.rs:129`),
   so `T` is inferred from the surrounding context. The
   `match guess.cmp(&secret_number)` line later requires
   `&secret_number: &Self` for `cmp`'s signature
   `fn cmp(&self, other: &Self) -> Ordering`, with `Self = u32`
   pinned by `guess: u32`'s annotation; this flows back through
   `&secret_number` to `secret_number: u32` and through to
   `gen_range`'s `T = u32`. Lesson 056's annotation-driven inference
   rule applies in reverse here: instead of the annotation pinning
   the call's target type directly, the *use site* pins it
   indirectly. The lesson body names this; the appendix is the
   audit trail.

2. **Run 3's middle 80 lines were elided in the appendix** for length.
   The probe captured them verbatim — same `You guessed: <k>` /
   `Too small!` shape for `k = 1..=80` — but reproducing 160 lines
   of identical-shape output is wasteful. The shape is fully
   established by run 2 (13 passes, all `Too small!` except the win
   line) and by the calibration probes' explicit middle elisions.

3. **Five secrets (1, 13, 81, 21, 80) are not curated for
   distribution properties.** They are genuine RNG output from
   `rand::thread_rng().gen_range(1..=100)` on probe day. The lesson
   body claims only "different secret per launch" — five distinct
   values is sufficient empirical witness. Probability and
   distribution semantics are explicitly deferred (carrying lesson
   066's deferral forward).

4. **`Cargo.lock` was created and gitignored.** The first `cargo
   build` wrote `Cargo.lock` (3.3 KB); the `.gitignore` excludes it
   per orchestrator directive. Lesson 032's deferral preserved.
   `target/` was also created and gitignored.

5. **Bit-identical verbatim verification was run.** A `diff` between
   the committed `src/main.rs` and the Book's Listing 2-6 (lines
   1257-1292 with code-fence markers stripped) returned exit 0 with
   no output. The capstone is **literally** Listing 2-6.

6. **No new Rust mechanic was introduced.** Every line in `src/main.rs`
   maps to one or more accepted graph nodes per the line-by-line map
   above. The two sibling-extensions (`u32::cmp` extending lesson
   061; `break;` as divergent statement extending lesson 059)
   were already exercised by lesson 063's capstone. The Cargo and
   rand pieces (`use rand::Rng;`, the `rand::thread_rng().gen_range(...)`
   call, the `[dependencies]` entry, `cargo build`'s resolving-build
   shape) were installed by lessons 064/065/066. No license gap was
   found during the probe or the line-by-line audit.

## Files committed for this cycle

- `lessons/067-capstone-guessing-game-with-rand.md` (this lesson)
- `evidence/067-capstone-guessing-game-with-rand.md` (this appendix)
- `observations/067-capstone-guessing-game-with-rand/Cargo.toml`
- `observations/067-capstone-guessing-game-with-rand/src/main.rs`
- `observations/067-capstone-guessing-game-with-rand/.gitignore`
  (excludes `target/` and `Cargo.lock`)
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
