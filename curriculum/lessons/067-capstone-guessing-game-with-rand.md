---
id: 067-capstone-guessing-game-with-rand
status: accepted
evidence: ../evidence/067-capstone-guessing-game-with-rand.md
---

# Capstone: The Book's guessing-game program (Listing 2-6 verbatim, with `rand`)

## The Capstone

This is a *Capstone Mode* cycle — the second on the Book path, parallel
to lesson 063. **No new Rust mechanic.** Two changes from 063: the
hardcoded `let secret_number: u32 = 7;` is replaced by the random call
`rand::thread_rng().gen_range(1..=100)` (lesson 066), and the program
now lives in a Cargo package whose `Cargo.toml` lists `rand` under
`[dependencies]` (lesson 065). The rest of the program is bit-identical
to lesson 063's. The result is the Rust Book's Listing 2-6 (ch02 lines
1256-1293) — the Book's *complete guessing-game code* — with no
substitutions, no deviations.

`src/main.rs`:

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

`Cargo.toml`:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

`cargo build` resolves and compiles cleanly with no warnings. Each
`cargo run` plays a different game because `gen_range(1..=100)` produces
a different secret per launch.

## Mental Model Delta

- *Before:* "I have 66 individual lessons plus lesson 063's no-rand
  capstone. The full Book program is reachable in principle but I have
  not seen it run with `rand` actually live."
- *After:* "Yes — composed end-to-end with `rand` restored, 66 lessons
  produce the Rust Book's exact Listing 2-6 program. Each `cargo run`
  plays a different game because `gen_range(1..=100)` is a random call.
  The 66-lesson dependency graph supports a real interactive program
  from end to end."

## Line-by-Line Map (compressed)

Five lines carry the load. The rest of the program (`fn main`, the
prompt prints, the trivial pieces) is licensed by basic-syntax lessons
and is expanded in the evidence appendix.

- `use rand::Rng;` — lesson 066. The `use` shape from lesson 044 with
  `rand` (an external crate from `[dependencies]`) replacing `std`.
  Required so `gen_range` is in scope below.
- `use std::cmp::Ordering;` + `use std::io;` — lessons 044 + 051.
- `let secret_number = rand::thread_rng().gen_range(1..=100);` —
  lesson 066. **This is the capstone's central change from lesson
  063**, replacing the hardcoded `let secret_number: u32 = 7;`. Notice
  the *no annotation* form: there is no `: u32` on this `let`. Lesson
  056's annotation-driven inference works in reverse here — the
  right-hand side's type is determined by `gen_range`'s generic return
  type plus the inferred type of `&secret_number` further down (which
  must match `&guess: &u32`), so the binding's type is settled without
  the annotation.
- The full input-prompt loop body — lessons 027 + 042 + 006 + 050 +
  054 + 048 + 052 + 053 + 049 + 057 + 062 + 055 + 056 + 058 + 059. The
  exact composition lesson 063 already mapped, unchanged today.
- The `match guess.cmp(&secret_number) { Ordering::Less => ...,
  Ordering::Greater => ..., Ordering::Equal => { ...; break; } }` block
  — lesson 061 + lesson 045 + lesson 051 + lesson 030 + lesson 027.
  The sibling-extension `u32::cmp` from lesson 063's capstone carries
  forward unchanged (the Book's `secret_number` infers to `u32` because
  it is compared with a `&u32`, so both sides of `cmp` agree).

`Cargo.toml` is also a capstone artifact. Three blocks:

- `[package]` block (`name`, `version`, `edition`) — lesson 032
  (`cargo new` populates this verbatim; today's package name comes
  from `cargo new --vcs none guessing_game`).
- `[dependencies]` populated with `rand = "0.8.5"` — lesson 065
  (the exact line copied from Book ch02 line 492). Cargo resolved
  this to `rand v0.8.6` per the SemVer rule deferred since lesson 065;
  honest reporting only.

*See `../evidence/067-capstone-guessing-game-with-rand.md` for the
full line-by-line trace, the corpus quote map, and the verbatim probe
transcripts.*

## Try It

Pick a directory you can write to. The build half (lessons 032 + 065 +
066):

```console
$ cargo new --vcs none guessing_game
    Creating binary (application) `guessing_game` package
$ cd guessing_game
```

Edit `Cargo.toml` to add `rand = "0.8.5"` under `[dependencies]`
(lesson 065). Edit `src/main.rs` to the program above (Listing 2-6
verbatim). Then:

```console
$ cargo build
    Updating crates.io index
     Locking 14 packages to latest Rust 1.95.0 compatible versions
      Adding rand v0.8.6 (available: v0.10.1)
   Compiling libc v0.2.186
   ... (transitive deps)
   Compiling rand v0.8.6
   Compiling guessing_game v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.82s
```

Same resolving-build shape as lessons 065 and 066. Now play the game.
The secret is random, so the cleanest reproducible probe is to pipe a
guaranteed-winning sequence:

```console
$ seq 1 100 | cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
You guessed: 1
You win!
```

That run hit `1` as the secret on the first guess. Two more identical
invocations hit `13` and `81` — three different secrets across three
runs. Three different secrets are the empirical witness that
`gen_range(1..=100)` is producing a different random number each
launch.

A *garbage-then-seq* probe `{ printf 'abc\n'; seq 1 100; } | cargo run`
shows the `Err(_) => continue` arm fire once. The first prompt
"Please input your guess." is followed *directly* by the second prompt
— no `You guessed: abc` line, no error message — then the loop steps
through the integers until it hits the secret. (That probe's secret
was `21`.) An *out-of-range* probe `{ printf '500\n'; seq 1 100; } |
cargo run` shows that the program does NOT enforce a `1..=100` range
on guesses: `500` parses cleanly as a `u32`, prints `You guessed: 500`,
hits `Too big!`, and the loop continues. The Book's Listing 2-6 makes
the same choice; range-checking the guess is named in *What To Ignore*.

The full transcripts of all five probes are in the evidence appendix.

## What Changed

- The full Book guessing-game program is now reachable from accepted
  graph nodes. Listing 2-6 (ch02 lines 1256-1293) compiles, runs, and
  plays a real game — verbatim, no substitutions.
- The `rand::thread_rng().gen_range(1..=100)` call from lesson 066
  substitutes for lesson 063's hardcoded `let secret_number: u32 = 7;`.
  Lesson 063's deferred substitution is now resolved.
- Each `cargo run` plays a different game because `gen_range(1..=100)`
  produces a different random secret per launch.
- The 66-lesson dependency graph supports a real interactive program
  from end to end. The Book path through chapter 2 is closed.

## Check Yourself

(a) You change `gen_range(1..=100)` to `gen_range(1..=10)` and
re-run `cargo run` three times. What changes about the game, and what
stays the same?

(b) You pipe `seq 1 200` to `cargo run` instead of `seq 1 100`. Does
the program ever print `You win!`? Why or why not?

*(Answers: (a) The secret is now in `[1, 10]` instead of `[1, 100]`,
so games end in at most 10 guesses with `seq 1 10 | cargo run`. The
program shape is unchanged; only the inclusive-range argument from
lesson 039 differs. (b) Yes. The program iterates through every line
on stdin until one matches the secret, then prints `You win!` and
`break;` exits the loop. Lines beyond the matching one are never
read. With `seq 1 200`, the program will hit some integer in
`[1, 100]` (the secret's actual range) well before reaching `200`.)*

## What To Ignore For Now

- *Restricting guesses to `1..=100` like a real game would.* Listing
  2-6 does not enforce this. Calibration probe B confirmed: the
  program accepts `500` as a valid `Too big!` guess. Adding
  `if guess < 1 || guess > 100 { continue; }` would compose lessons
  014 + 013 + 015 + 035 — future composition.
- *The Book's later "polishing" passes after Listing 2-6.* Lesson
  063's deferrals carry forward: `Ord` and `PartialOrd` traits,
  `String::clear()` for buffer reuse, EOF handling, the never type
  `!`, interactive-vs-piped input behavior, `ParseIntError`.
- *The whole rest of the Rust Book past chapter 2.* Chapter 3 onward
  (variables-and-types, ownership, structs, enums, modules,
  collections, error handling, traits, generics, lifetimes, testing,
  smart pointers, concurrency, etc.) — all deferred.
- *Cargo subcommands beyond `cargo new` / `cargo build` / `cargo run`*
  — `cargo check`, `cargo test`, `cargo doc`, `cargo clippy`,
  `cargo fmt`, `cargo update`, `cargo add`, `cargo publish`, etc.
- *Multi-file projects, library packages (`cargo new --lib`), modules,
  workspaces, `[dev-dependencies]`, feature flags, path/git deps,
  alternate registries.*
- *SemVer mechanics* (lesson 065's deferral) and *`Cargo.lock`
  internals* (lessons 032/065's deferral). The manifest line
  `rand = "0.8.5"` resolved to `rand v0.8.6` today; that drift is the
  SemVer rule operating, not investigated here.
- *Trait machinery* (lesson 040's deferral and lesson 066's
  reaffirmation): what `use rand::Rng;` mechanically does at the
  trait-method-visibility level stays uninstalled.
- All previously deferred items.

## Evidence

See `../evidence/067-capstone-guessing-game-with-rand.md`.
