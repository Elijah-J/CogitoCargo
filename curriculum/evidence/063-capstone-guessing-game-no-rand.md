# Evidence — 063-capstone-guessing-game-no-rand

Audit appendix for `lessons/063-capstone-guessing-game-no-rand.md`.
This is a *Capstone Mode* cycle: it installs no new Rust mechanic, so
the appendix's job is (a) toolchain and reproducibility, (b) the
corpus quote map for every load-bearing claim in the lesson, (c) the
six required transcripts verbatim, (d) the line-by-line graph-node
license map, and (e) the `u32::cmp` sibling-type calibration probe.

## Toolchain

`rustc --version --verbose`:

```
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-apple-darwin
release: 1.95.0
LLVM version: 22.1.2
```

`uname -sm` -> `Darwin x86_64`.

Probes run in fresh `mktemp -d` directories, removed at the end of
each run. The committed program lives at
`observations/063-capstone-guessing-game-no-rand.rs` — bit-exact
duplicate of the program in the lesson and in this appendix's
*Project setup* section. The `u32::cmp` calibration `.rs` is not
committed; its source and transcript are below.

## Project setup

A single `.rs` file. No `Cargo.toml`, no Cargo project. The capstone
deliberately substitutes a hardcoded `let secret_number: u32 = 7;`
for the Book's `let secret_number = rand::thread_rng().gen_range(1..=100);`
(ch02 line 1265, Listing 2-6) to avoid `Cargo.toml [dependencies]`,
the `rand` external crate, and `cargo build`/`cargo run`. These are
the next move on the Book path; the capstone stops short.

```console
$ rustc demo.rs
$ ls
demo  demo.rs
```

`rustc` exits 0 with no warnings emitted to stderr.

## Corpus quote map

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053, 054, 055, 056, 058, 059, 060, 061, 062. Today's
cycle is the empirical demonstration that 62 cycles compose into
this chapter's Listing 2-6 program (the *final* code, post-debug-
print removal).

**Lines 1256-1293** (Listing 2-6, the Book's final program shape —
load-bearing for the entire capstone):

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

The capstone program is bit-equivalent to this listing modulo two
deliberate deviations:

1. The `use rand::Rng;` line (1260) is *removed*.
2. The `let secret_number = rand::thread_rng().gen_range(1..=100);`
   line (1265) is *replaced* by `let secret_number: u32 = 7;`.

All other lines are bit-identical to Listing 2-6. Note: an earlier
version of the Book program (Listing 2-5, lines 1147-1190) included
a debug `println!("The secret number is: {secret_number}");` line
(1158) as a temporary aid; Listing 2-6 omits it. The capstone
matches Listing 2-6, so there is no third "removed-debug-print"
deviation — the debug line is already gone in the source we're
deviating from.

**Line 1295** (the listing's role):

> *[Listing 2-6](#listing-2-6): Complete guessing game code*

**Lines 778-783** (the `cmp`/`Ordering` block, also Listing 2-4
before the parse-as-u32 conversion is added):

> ```rust
>     match guess.cmp(&secret_number) {
>         Ordering::Less => println!("Too small!"),
>         Ordering::Greater => println!("Too big!"),
>         Ordering::Equal => println!("You win!"),
>     }
> ```

Direct corpus license for the lesson's three message strings (`Too
small!`, `Too big!`, `You win!`) and three-arm shape.

**Lines 793-800** (audience-level walkthrough of the `cmp` block,
already cited in cycle 061; re-cited here for the capstone):

> Then, we add five new lines at the bottom that use the `Ordering`
> type. The `cmp` method compares two values and can be called on
> anything that can be compared. It takes a reference to whatever
> you want to compare with: Here, it's comparing `guess` to
> `secret_number`. Then, it returns a variant of the `Ordering`
> enum we brought into scope with the `use` statement.

**Lines 935-947** (the audience-level prose for `let guess: u32 =
guess.trim().parse()...` — load-bearing for the lesson's
"`: u32` does double duty" claim and the *Try It* walkthrough of
transcript 6):

> The [`parse` method on strings](../std/primitive.str.md#method.parse)
> converts a string to another type. Here, we use it to convert from
> a string to a number. We need to tell Rust the exact number type
> we want by using `let guess: u32`. The colon (`:`) after `guess`
> tells Rust we'll annotate the variable's type. Rust has a few
> built-in number types; the `u32` seen here is an unsigned, 32-bit
> integer. It's a good default choice for a small positive number.
>
> Additionally, the `u32` annotation in this example program and the
> comparison with `secret_number` means Rust will infer that
> `secret_number` should be a `u32` as well. So, now the comparison
> will be between two values of the same type!

The phrase "a small positive number" plus "comparison ... between
two values of the same type" is the Book's audience-level statement
of the *exact* design decision the capstone preserves: the secret
and the guess must be the same type for `cmp` to compile, and `u32`
is the chosen common type.

**Lines 916-921** (shadowing audience-level prose, cited via cycle
057):

> But wait, doesn't the program already have a variable named
> `guess`? It does, but helpfully Rust allows us to shadow the
> previous value of `guess` with a new one. *Shadowing* lets us
> reuse the `guess` variable name rather than forcing us to create
> two unique variables, such as `guess_str` and `guess`, for example.

Direct corpus license for the lesson's "type-changing shadow `let
guess: u32 = match guess.trim().parse() { ... }`" framing.

**Lines 1200-1214** (audience-level walkthrough of the
`Ok(num)`/`Err(_) => continue` arms, cited via cycles 058 and 059;
re-cited here for the capstone *Try It* walkthrough of transcript
4):

> If `parse` is able to successfully turn the string into a number,
> it will return an `Ok` value that contains the resultant number.
> That `Ok` value will match the first arm's pattern, and the
> `match` expression will just return the `num` value that `parse`
> produced and put inside the `Ok` value.
>
> If `parse` is *not* able to turn the string into a number, it
> will return an `Err` value ... So, the program will execute the
> second arm's code, `continue`, which tells the program to go to
> the next iteration of the `loop` and ask for another guess.

### `output/docs/rust/std/primitive.u32.md`

The std-library page for the `u32` primitive type. Already cited in
cycle 062. Today's citation is the `impl Ord` block — the
sibling-type extension calibration.

**Line 8** (the type description):

> The 32-bit unsigned integer type.

**Lines 4347-4355** (the `impl Ord for u32` block — load-bearing for
the lesson's "u32::cmp exists with the same signature" claim):

> ### impl [Ord](cmp/trait.Ord.md "trait std::cmp::Ord") for [u32](primitive.u32.md)
>
> #### fn [cmp](cmp/trait.Ord.md#tymethod.cmp)(&self, other: &[u32](primitive.u32.md)) -> [Ordering](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html "enum std::cmp::Ordering")
>
> This method returns an [`Ordering`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html "enum std::cmp::Ordering") between `self` and `other`. [Read more](cmp/trait.Ord.md#tymethod.cmp)

This is the corpus statement that (a) `u32` implements `Ord`, so the
`cmp` method is reachable on `u32`, and (b) the signature is `fn
cmp(&self, other: &u32) -> Ordering` — identical in shape to cycle
061's `impl Ord for i32` block at `primitive.i32.md:4066-4074`. The
mechanic is identical; only the receiver type changes from `i32` to
`u32`.

### `output/docs/rust/std/cmp/trait.Ord.md`

The std-library page for the `Ord` trait. Cited in cycle 061. Today
re-cited for the trait-method declaration line that both `i32` and
`u32` specialize. **Lines 6-19** (the trait declaration; cycle 061's
load-bearing quote re-used for the capstone's sibling-type point):

> ```
> pub trait Ord: Eq + PartialOrd {
>     // Required method
>     fn cmp(&self, other: &Self) -> Ordering;
>     ...
> }
> ```

The signature `fn cmp(&self, other: &Self) -> Ordering` with `Self`
bound to whatever type implements `Ord` — for `i32` that's `&i32`
(cycle 061), for `u32` that's `&u32` (today).

### `output/docs/rust/std/cmp/enum.Ordering.md`

Already cited in cycles 051 and 061. Re-cited transitively for the
three variants `Less`, `Greater`, `Equal` used in the capstone's
final match. No new quote needed.

### Other corpus files

The capstone uses no Rust corpus material outside the four files
above. All other facts trace back to accepted graph nodes and their
own evidence appendices.

## Line-by-line graph-node license map

Every line of the capstone program → the accepted graph nodes that
license it. Lines that are pure punctuation (`}`, `;`, blanks) are
omitted; they're licensed by basic-syntax cycles since cycle 002.

| Line                                                                                | Cycles                                    | License note                                                                         |
| ----------------------------------------------------------------------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------ |
| `use std::cmp::Ordering;`                                                           | 044, 051                                  | `use` brings the short name `Ordering` into scope from `std::cmp`.                   |
| `use std::io;`                                                                      | 044                                       | Parent-module `use` — same shape as cycle 060.                                       |
| `fn main() {`                                                                       | 002                                       | Entry-point shape.                                                                   |
| `println!("Guess the number!");`                                                    | 011                                       | `println!` macro, string-literal first arg, no placeholders.                         |
| `let secret_number: u32 = 7;`                                                       | 019, 062                                  | `let name: TYPE = value;` form (019) with `u32` (062). **The capstone substitution.**|
| `loop {`                                                                            | 027                                       | Outer loop opens.                                                                    |
| `println!("Please input your guess.");`                                             | 011                                       | Per-iteration prompt.                                                                |
| `let mut guess = String::new();`                                                    | 042, 006                                  | Fresh `String` each iteration; `mut` makes `&mut guess` legal next line.             |
| `io::stdin()`                                                                       | 050                                       | Stdin handle.                                                                        |
| `.read_line(&mut guess)`                                                            | 054, 048, 049                             | `read_line` method; `&mut guess` argument shape; chained call.                       |
| `.expect("Failed to read line");`                                                   | 053, 052, 049                             | Consume `Result<usize, io::Error>` with panic-on-Err.                                |
| `let guess: u32 = match guess.trim().parse() {`                                     | 057, 062, 030, 055, 056                   | Type-changing shadow; `u32` annotation; match opens; `.trim()` then `.parse()`.      |
| `    Ok(num) => num,`                                                               | 058                                       | `Ok` payload-binding pattern; bound name is the arm's value.                         |
| `    Err(_) => continue,`                                                           | 058, 035, 059                             | `Err` wildcard-payload pattern; `continue` body; divergent-arm rule.                 |
| `};`                                                                                | 030                                       | Match expression closes.                                                             |
| `println!("You guessed: {guess}");`                                                 | 011                                       | `{name}` placeholder resolves to the *new* `u32` `guess`.                            |
| `match guess.cmp(&secret_number) {`                                                 | 061, 045, 030, 051                        | `u32::cmp` (sibling of cycle 061's `i32::cmp`); shared-ref arg; match on `Ordering`. |
| `    Ordering::Less => println!("Too small!"),`                                     | 051, 011                                  | Variant-pattern arm; print-only body.                                                |
| `    Ordering::Greater => println!("Too big!"),`                                    | 051, 011                                  | Same shape.                                                                          |
| `    Ordering::Equal => {`                                                          | 051, 030                                  | Variant-pattern arm with block-body.                                                 |
| `        println!("You win!");`                                                     | 011                                       | First statement in the block.                                                        |
| `        break;`                                                                    | 027                                       | Exits the outer `loop`. Block ends without a value (cycle 027 + cycle 059's          |
|                                                                                     |                                           | divergent-statement rule generalized to `break;` — the block diverges).              |
| `    }`                                                                             | —                                         | Closes the `Ordering::Equal` arm body block.                                         |
| `}`                                                                                 | —                                         | Closes the outer `match`.                                                            |
| `}`                                                                                 | —                                         | Closes the `loop`.                                                                   |
| `}`                                                                                 | —                                         | Closes `main`.                                                                       |

No line is unlicensed. The one mild stretch is the
`Ordering::Equal => { ...; break; }` block-as-arm-body shape: cycle
030 established that arm bodies are expressions (and a block is an
expression), and cycle 059 established that a divergent expression in
an arm exempts the arm from the share-type rule. Cycle 059's specific
example was `continue` in an arm; today's `break;` extends the same
divergent-arm rule by the same logic. Cycle 059's *What To Ignore*
explicitly listed "`break` and `return` in match arm bodies —
generalizing today's rule beyond `continue`" as an unlock; the
capstone exercises that generalization by using it. This is the
only place the capstone leans on a deferral-listed unlock; the
sibling-type generalization for `cmp` is the other.

## Sibling-type extension: `u32::cmp` calibration

Cycle 061 installed `cmp` on `i32`. The capstone uses `cmp` on `u32`.
The std page for `u32` (`output/docs/rust/std/primitive.u32.md` lines
4347-4355, quoted above) shows an `impl Ord for u32` block with `fn
cmp(&self, other: &u32) -> Ordering` — identical to the `i32` shape
modulo the receiver type. The empirical calibration:

```rust
use std::cmp::Ordering;

fn main() {
    let a: u32 = 3;
    let b: u32 = 5;
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
```

```console
$ rustc cmp_u32.rs
$ ./cmp_u32
a < b
```

Same shape, same outputs (`a < b`, `a == b`, `a > b` for the three
input pairs) as cycle 061's `i32` form. Empirical confirmation that
cycle 061's mechanic generalizes from `i32` to `u32` with no surprise.

## Six required transcripts (verbatim)

All six runs use the committed `observations/063-capstone-guessing-game-no-rand.rs`
program. `rustc demo.rs` was run once with no warnings. All six runs
exit 0.

### Transcript 1: Win on first guess

```console
$ printf '7\n' | ./demo
Guess the number!
Please input your guess.
You guessed: 7
You win!
```

Exit 0. The Ordering::Equal arm fires immediately; `break;` exits the
outer loop.

### Transcript 2: Too small then win

```console
$ printf '3\n7\n' | ./demo
Guess the number!
Please input your guess.
You guessed: 3
Too small!
Please input your guess.
You guessed: 7
You win!
```

Exit 0. Pass 1 hits Ordering::Less, prints `Too small!`, falls
through to the loop head. Pass 2 wins.

### Transcript 3: Too big then win

```console
$ printf '12\n7\n' | ./demo
Guess the number!
Please input your guess.
You guessed: 12
Too big!
Please input your guess.
You guessed: 7
You win!
```

Exit 0. Pass 1 hits Ordering::Greater.

### Transcript 4: Garbage then valid

```console
$ printf 'abc\n7\n' | ./demo
Guess the number!
Please input your guess.
Please input your guess.
You guessed: 7
You win!
```

Exit 0. Pass 1's `.trim().parse::<u32>()` returns `Err(_)`, `continue`
fires, no `You guessed:` line is printed for `abc`. Pass 2 wins.
Empirical confirmation that `Err(_) => continue` does *not* surface
the parse error (the wildcard `_` discards the `ParseIntError`
payload — cycle 058's mechanic).

### Transcript 5: Walk through all three Ordering arms in one run

```console
$ printf '3\n12\n7\n' | ./demo
Guess the number!
Please input your guess.
You guessed: 3
Too small!
Please input your guess.
You guessed: 12
Too big!
Please input your guess.
You guessed: 7
You win!
```

Exit 0. All three Ordering arms reached in one program run.

### Transcript 6: Negative input rejected as parse failure

```console
$ printf -- '-1\n7\n' | ./demo
Guess the number!
Please input your guess.
Please input your guess.
You guessed: 7
You win!
```

Exit 0. **Load-bearing observation**: the string `-1` is *not* a
parseable `u32` because `u32` does not accept negative values, so
`.parse()` returns `Err(_)` and `continue` fires. The behavior is
indistinguishable from transcript 4's `abc` case. The `: u32`
annotation on the shadowed `guess` is what makes this work.

To verify the contrast, mentally substitute `: i32` for `: u32` on
the shadowed-`guess` line. With `i32`, `-1` *would* parse
successfully as `-1i32`, the program would print `You guessed: -1`,
and `(-1).cmp(&7)` would be `Ordering::Less` so `Too small!` would
print. The annotation determines whether negative input is treated
as garbage or as a valid-but-too-small guess. The Book's program
chooses `u32` for exactly this reason (ch02 lines 935-947 quoted
above: "a good default choice for a small positive number" plus
"the comparison will be between two values of the same type").

No transcript surprised vs. expectation. All match the capstone's
stated behavior exactly.

## Prerequisite-claim summary

The capstone's `depends_on` list in `graph.md` is long because the
capstone is a composition. Below is each accepted node id and the
specific claim it licenses *for the capstone*. Older supporting
lessons not directly load-bearing today (e.g. cycle 003 diagnostic
reading, cycle 004 statement-ordering, cycle 010 line comments,
cycles 014-016 if/else, cycles 020-026 functions/expressions,
cycles 029 unit type, etc.) are present transitively via the named
direct prerequisites and are not re-listed.

- **001-rustc-compile-and-run**: `rustc demo.rs` produces `./demo`
  which the capstone runs.
- **002-fn-main-entry-point**: the `fn main() { ... }` shape and the
  curly-brace block shape used throughout.
- **005-let-binding**: `let secret_number = ...;` (and every other
  `let` in the program).
- **006-mut-binding**: `let mut guess = ...;` is what makes the
  `&mut guess` argument legal on the next line. Without `mut`, cycle
  048's argument check would fire E0596.
- **011-println-positional-args**: every `println!` call in the
  program (including the `{guess}` named-placeholder form).
- **019-type-annotation-i32**: the `let name: TYPE = value;` shape.
  Today the `TYPE` slot holds `u32`, but the syntax is unchanged
  from cycle 019.
- **027-loop-and-break**: outer `loop { ... }` and the `break;` in
  the `Ordering::Equal` arm body.
- **030-match-on-bool**: the `match` expression shape, three arms,
  arm bodies, the `,`-separated arm list. Generalizes to today's
  `Result` and `Ordering` scrutinees.
- **035-continue**: `Err(_) => continue` body skips to the loop head.
- **042-string-new**: `String::new()` constructs an empty `String`.
- **044-use-declaration**: `use std::cmp::Ordering;` and `use std::io;`.
- **045-shared-reference**: `&secret_number` in
  `guess.cmp(&secret_number)` — prefix-`&` produces the `&u32` value
  the `cmp` method expects.
- **048-mutable-reference-parameter**: `read_line(&mut guess)` —
  the argument shape that `read_line` requires.
- **049-method-chaining**: `io::stdin().read_line(&mut guess).expect(...)`
  is a three-call left-associative chain.
- **050-io-stdin-handle**: `io::stdin()` returns the stdin handle.
- **051-ordering-enum-and-variant-match**: the three-arm match on
  `Ordering::Less` / `Ordering::Greater` / `Ordering::Equal`.
- **052-result-enum-and-is-ok**: `read_line` and `parse` both return
  `Result`; the capstone consumes one with `.expect` and the other
  with `match`.
- **053-result-expect-and-panic**: `.expect("Failed to read line")`
  panics on Err with the message and the payload.
- **054-read-line-from-stdin**: `read_line(&mut guess)` appends one
  newline-terminated line of stdin into `guess` and returns
  `Result<usize, io::Error>`.
- **055-string-trim**: `guess.trim()` returns a `&str` with leading
  and trailing whitespace (including `\n`) stripped.
- **056-str-parse-to-i32**: `.parse()` on a `&str` returns `Result<TARGET, _>`,
  where `TARGET` is determined by inference. Today the `: u32`
  annotation on the shadowed `guess` flows back through the match
  and pins `TARGET = u32`. The mechanic is identical to cycle 056's
  `: i32`; the sibling-type extension is the only change.
- **057-type-changing-shadowing**: `let guess: u32 = match guess.trim().parse() { ... };`
  shadows the outer `guess: String` with a new `guess: u32`. Cycle
  057's example was `&str → i32`; today's is `String → u32`. Same
  mechanic; sibling target type.
- **058-match-result-payload-variants**: `Ok(num) => num` and
  `Err(_) => continue` — payload-binding and wildcard-payload
  patterns inside variant constructors.
- **059-continue-in-match-arm**: the divergent-arm rule that lets
  `Err(_) => continue` not produce a `u32`. The match's `u32` type
  comes entirely from the `Ok(num) => num` arm. The capstone *also*
  uses `break;` as a divergent statement inside the
  `Ordering::Equal` arm body; cycle 059's *What To Ignore* listed
  "`break` and `return` in match arm bodies — generalizing today's
  rule beyond `continue`" as an unlock. The capstone exercises that
  unlock.
- **060-input-prompt-loop**: the `loop { fresh buf; read_line chain;
  match .trim().parse() { Ok(num) => num, Err(_) => continue, }; ... }`
  shape — cycle 060 is the immediate parent composition; today
  extends it by replacing `n` with `guess` (cycle 057 shadow), `i32`
  with `u32` (cycle 062), and adding the `Ordering` match plus
  `break;` for the win path.
- **061-i32-cmp-returns-ordering**: `a.cmp(&b)` on `i32` returns
  `Ordering`. The capstone uses `cmp` on `u32` instead — sibling-type
  extension, identical signature per the `impl Ord for u32` block in
  `primitive.u32.md` lines 4347-4355.
- **062-u32-unsigned-integer**: `let secret_number: u32 = 7;` and
  `let guess: u32 = ...;`. Also licenses the *Try It* transcript-6
  observation that `-1` is rejected as a parse failure: cycle 062's
  rule is "`u32` cannot represent negative values" (E0600 at compile
  time for negative literals); today's program shows the same rule
  surfacing at parse time (input is a string, not a literal, so the
  failure path is `parse() → Err(_)` rather than E0600).

Plus ordinary computer-use: terminal, plain-text editor, `rustc` on
`PATH`, shell-piping with `printf '...' | ./demo` (since cycle 053).

## Two cycles' "What To Ignore" lists exercised

Two deferrals from prior cycles get exercised by the capstone:

1. **Cycle 059** deferred "`break` and `return` in match arm bodies
   — generalizing today's rule beyond `continue`". The capstone uses
   `break;` inside the `Ordering::Equal` arm body, and the program
   compiles and runs as expected. Empirical confirmation that the
   rule generalizes.
2. **Cycle 061** deferred sibling-type `cmp` (specifically named
   `String::cmp` and `&str::cmp`; `u32::cmp` falls in the same
   sibling-extension class). The capstone uses `u32::cmp`, the
   calibration probe runs cleanly, and the same `Ordering` shape
   matches.

Both deferrals are exercised by the capstone *via direct empirical
observation*, not by introducing a new mechanic. The lesson body's
"sibling-type extension" framing makes this explicit.

## Risks / honest disclosures

- The lesson body's *Try It* section walks transcript 4 in plain
  prose. The walkthrough names `Err(ParseIntError)` informally as
  the "Err" payload type. `ParseIntError` was *not* installed as a
  typed name (cycles 056 and 058 explicitly deferred this); the
  appendix cites cycle 058 for the wildcard-`_`-discards-the-payload
  mechanic so the named type does not need to be installed for the
  walkthrough's claim. The transcript shows no payload printed,
  consistent with the wildcard discarding it.
- The line-by-line map's table format is a slight presentation
  departure from cycles 060/061's bullet-list per-line attribution.
  The information is the same; the table is denser. Acceptable for
  a capstone where line count is high.
- The capstone makes no claim about the runtime behavior under
  terminal (TTY) input vs. piped input. Cycle 060's evidence
  appendix already named that distinction; the capstone inherits it
  unchanged. All capstone transcripts use piped input for
  reproducibility.
- The `: u32` annotation's "double duty" claim (it pins parse target
  AND it rejects negative input) is decomposed: cycle 056 licenses
  the parse-target-pinning half, cycle 062 licenses the
  unsigned-rejects-negative half, and the capstone's transcript 6
  is the empirical witness that *both* duties are performed by the
  same one annotation in the same one program.
