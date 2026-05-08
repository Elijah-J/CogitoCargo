---
id: 063-capstone-guessing-game-no-rand
status: accepted
evidence: ../evidence/063-capstone-guessing-game-no-rand.md
---

# Capstone: The Book's guessing-game program (minus the `rand` external crate)

## The Capstone

This is a *Capstone Mode* cycle, not a normal one-concept cycle. **No
new Rust mechanic.** The cycle's value is the empirical claim that
sixty-two previously-installed cycles, composed end-to-end, produce a
runnable program that recognizably does what the Book's guessing-game
program (ch02 Listing 2-6, lines 1256-1293, the Book's *final* code)
does — modulo one substitution: the random secret number
`rand::thread_rng().gen_range(1..=100)` becomes the hardcoded
`let secret_number: u32 = 7;`. That is the *only* substitution.
Everything else — the message strings, the control flow, the line
shape — matches the Book.

```rust
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = 7;

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

`rustc demo.rs` exits 0 with no warnings. The program is a single
`.rs` file — not a Cargo project — because excluding `rand` is the
whole point of the substitution. Cargo and external crates are the
next move on the Book path; this capstone deliberately stops short.

## Mental Model Delta

- *Before:* "I have learned 62 individual cycles. I can each one do a
  thing in isolation. I do not know whether they fit together to do
  real work."
- *After:* "Yes — composed end-to-end, 62 cycles produce a runnable
  program that recognizably does what the Book's guessing-game does
  (ask, validate, compare, retry on garbage, win on equal), modulo
  the random-number generator. The program has no Rust syntax I have
  not been formally taught."

## Line-by-Line Map (compressed)

Four lines carry the load. The rest of the program (imports,
`fn main`, the prompt prints, the trivial pieces) is licensed by
basic-syntax cycles and is expanded in the evidence appendix.

- `let secret_number: u32 = 7;` — cycles 019 + 062. **This is where
  `rand::thread_rng().gen_range(1..=100)` would appear in the Book**;
  the capstone substitutes `7` to avoid the `rand` external crate.
- `io::stdin().read_line(&mut guess).expect("Failed to read line");`
  — cycle 054 (which itself wraps cycles 049 + 050 + 048 + 052 + 053).
- `let guess: u32 = match guess.trim().parse() { Ok(num) => num,
  Err(_) => continue, };` — the densest line, six cycles braided:
  cycles 057 + 062 + 055 + 056 + 058 + 059 (type-changing shadow,
  `u32` annotation, `.trim()`, annotation-driven `.parse()`, payload
  patterns, divergent-arm rule).
- `match guess.cmp(&secret_number) { Ordering::Less => ...,
  Ordering::Greater => ..., Ordering::Equal => { ...; break; } }`
  — cycle 061 (sibling-type extended to `u32`) + cycle 045 + cycle
  051 + cycle 030 + cycle 027 (with cycle 059's divergent-arm rule
  generalizing from `continue` to `break;`).

*See `../evidence/063-capstone-guessing-game-no-rand.md` for the
full line-by-line trace of every line, including imports, `fn main`,
the prompts, and the trivial pieces.*

## Sibling-type extension: `u32::cmp`

Cycle 061 installed `cmp` on `i32`. The capstone uses `cmp` on `u32`:
`guess.cmp(&secret_number)`. The std page for `u32` has the same
`impl Ord` block as `i32` — same signature, only the receiver type
changes. The appendix's calibration probe confirms identical
behavior. No new mechanic; trait machinery stays deferred per cycle
061.

## Try It

Save the program above as `demo.rs`. Compile with `rustc demo.rs`.
Run interactively as `./demo` (type a guess and press Enter); for
reproducibility, pipe stdin. Six runs cover the program end-to-end:

```console
$ printf '7\n' | ./demo
Guess the number!
Please input your guess.
You guessed: 7
You win!

$ printf '3\n7\n' | ./demo
Guess the number!
Please input your guess.
You guessed: 3
Too small!
Please input your guess.
You guessed: 7
You win!

$ printf '12\n7\n' | ./demo
... You guessed: 12
Too big!
... You guessed: 7
You win!

$ printf 'abc\n7\n' | ./demo
Guess the number!
Please input your guess.
Please input your guess.
You guessed: 7
You win!

$ printf '3\n12\n7\n' | ./demo
... Too small! ... Too big! ... You win!

$ printf -- '-1\n7\n' | ./demo
Guess the number!
Please input your guess.
Please input your guess.
You guessed: 7
You win!
```

Transcript 4 (`printf 'abc\n7\n'`) shows the load-bearing
observation: the `Err(_) => continue` arm fires with no error
message printed because the wildcard `_` discards the `ParseIntError`
payload (cycle 058's mechanic). The pass-1 prompt is followed
directly by the pass-2 prompt — no `You guessed:` line for `abc`.

Transcript 6 (`printf -- '-1\n7\n'`) is the load-bearing case for
the `: u32` annotation: `-1` is *not* a parseable `u32`, so
`parse()` returns `Err(_)`, `continue` retries, and `-1` is treated
like `abc`. The Book's program makes the same choice for the same
reason (ch02 lines 935-939). *Check Yourself* below walks the
contrast with `: i32`.

## What Changed

- You can write the Book's guessing-game program — minus the `rand`
  external crate — end-to-end with no unexplained syntax. The
  prompt-and-validate inner loop from cycle 060, plus the
  `cmp`-and-three-arm-match from cycle 061, plus cycle 062's `: u32`,
  plus cycle 057's type-changing shadow, compose into one program.
- The type-changing shadow `let guess: u32 = match guess.trim().parse()
  { ... }` is cycle 057's pattern with cycle 062's `u32` substituted
  for cycle 057's `i32`. The Book uses this exact form (ch02 line
  1276 in Listing 2-6).
- The `: u32` annotation on the shadowed `guess` does double duty:
  it pins `parse()`'s target to `u32` (cycle 056's flow-back) *and*
  it rejects negative input as a parse failure (cycle 062's rule,
  surfacing at parse time instead of compile time because the input
  is a string, not a literal).
- `cmp` works on `u32` the same way it works on `i32`. Cycle 061
  installed `i32::cmp`; `u32` has an identical `impl Ord` block, so
  `u32::cmp` exists with the same signature.
- Capstone cycles install no new feature. Their value is "no syntax
  in this program is new" plus "these installed cycles fit together
  to do recognized real work."

## Check Yourself

(a) If you change `: u32` to `: i32` on the shadowed `guess` line
and re-run `printf -- '-1\n7\n' | ./demo`, what does it print?
Why is the answer different from today's run?

(b) If you also change `secret_number`'s annotation to `: i32` (so
both bindings are `i32`), does the program still type-check?

*(Answers: (a) With `: i32` on the shadowed `guess`, `-1` parses
successfully as `-1i32` (cycles 056 + 057 — the annotation flows
back through the match and pins parse target), so `Err(_) =>
continue` does NOT fire. But `secret_number` is still `u32`, and
cycle 061's `cmp` requires both sides to share a type — the call
`guess.cmp(&secret_number)` becomes `&i32` against `&u32` and the
program does not type-check. Same `E0308 mismatched types` shape
that cycle 061 captured for the `cmp(y)` (dropped-`&`) case, only
the mismatch is type-vs-type instead of `&T`-vs-`T`. (b) Yes. With
both bindings `i32`, both sides of `cmp` agree (cycle 061's `cmp`
generalizes for any `T: Ord`, including `i32`), the program
type-checks, and `printf -- '-1\n7\n' | ./demo` would print `Guess
the number! / Please input your guess. / You guessed: -1 / Too
small! / Please input your guess. / You guessed: 7 / You win!`.
This is the diff that the `: u32` annotation prevents — cycle 062's
unsigned rule turns negative input into a parse failure instead of
a valid-but-too-small guess.)*

## What To Ignore For Now

- *The `rand` external crate*. The Book's `let secret_number =
  rand::thread_rng().gen_range(1..=100);` is the largest remaining
  piece on the Book path. It introduces `Cargo.toml [dependencies]`,
  `cargo build`, the `rand::Rng` trait, the `gen_range` method, and
  the inclusive-range argument shape. The capstone substitutes a
  hardcoded `7` to defer all of this. Next on the Book path.
- *The `Ord` and `PartialOrd` traits as installed nouns* — cycle 061
  deferred this. The capstone uses `cmp` on both `i32` (cycle 061)
  and `u32` (today) operationally without surfacing the trait noun.
- *`String::clear()` for buffer reuse* — cycle 060's deferral
  preserved. The capstone allocates a fresh `String` each iteration
  with `let mut guess = String::new()` inside the loop.
- *EOF handling* — cycle 060's deferral preserved. With piped input
  exhausted, `read_line` returns `Ok(0)` with empty `buf`,
  `.trim().parse()` on `""` is `Err`, `continue` retries forever.
  The transcripts above always end with a `7` line so the
  `Ordering::Equal` arm fires and `break;` exits before EOF can
  cause a spin.
- *Restricting guesses to `1..=100`* — the Book's program does not
  validate the range; it just compares against the (1..=100) secret.
  Adding `if guess < 1 || guess > 100 { continue; }` would compose
  cycle 014 (`if`), cycle 013 (`<`/`>`), cycle 015 (`||`), and cycle
  035 (`continue`) — future composition.
- *Interactive use.* Running `./demo` and typing into the terminal
  works the same way; the capstone's transcripts use `printf | ./demo`
  for reproducibility.
- All previously deferred items.

## Evidence

See `../evidence/063-capstone-guessing-game-no-rand.md`.
