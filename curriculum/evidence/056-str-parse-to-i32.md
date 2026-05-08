# Evidence — 056-str-parse-to-i32

Audit appendix for `lessons/056-str-parse-to-i32.md`. Holds the
corpus-quote map, the toolchain string, the working- and
broken-contrast probe transcripts, the type-inference calibration
probe, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/056-str-parse-to-i32.rs`). The broken-contrast
  runtime-panic `.rs` and the type-inference calibration `.rs` are
  not committed — their transcripts below are the artifacts.

## Sources

### `output/docs/rust/std/primitive.str.md`

The std page for the primitive `str` type. Already cited in lesson
055 (for `&str` as a typed name and `.trim()`). Reused here as the
*primary* source for cycle 056 — the new piece this cycle installs
is the method `.parse()` on `&str`. Three load-bearing spans new to
cycle 056.

Line 2176 (the canonical `.parse()` signature):

> #### pub fn [parse](#method.parse)<F>(&self) -> [Result](result/enum.Result.md "enum std::result::Result")<F, <F as [FromStr](str/trait.FromStr.md "trait std::str::FromStr")>::[Err](str/trait.FromStr.md#associatedtype.Err "type std::str::FromStr::Err")> where F: [FromStr](str/trait.FromStr.md "trait std::str::FromStr"),

Direct corpus statement of:
- (a) `.parse` is a method on `str` (the `&self` receiver). The
  lesson installs (a) verbatim, treating the receiver as `&str` (the
  typed name installed in cycle 055).
- (b) The return type is `Result<F, <F as FromStr>::Err>`, i.e., a
  `Result` whose `Ok` payload is *some target type `F`* and whose
  `Err` is `F`'s associated `FromStr::Err` type. The lesson surfaces
  only the audience-level shape: "returns a `Result` carrying *some
  target type* on the `Ok` side." The lesson does *not* surface the
  generic parameter `<F>`, the where-clause `where F: FromStr`, the
  `FromStr` trait, or the associated-type spelling
  `<F as FromStr>::Err` — all are explicitly deferred under *What To
  Ignore For Now*.
- (c) The target type `F` is determined *somehow* — the corpus
  resolves this through trait inference. The lesson grounds the
  audience-level mechanic ("rustc reads the `: i32` annotation as a
  constraint and infers `F = i32`") via the Book ch03-02 example and
  the captured calibration probe; the underlying trait-inference
  algorithm is deferred.

Lines 2178-2186 (the description and load-bearing inference note):

> Parses this string slice into another type.
>
> Because `parse` is so general, it can cause problems with type
> inference. As such, `parse` is one of the few times you'll see
> the syntax affectionately known as the 'turbofish': `::<>`. This
> helps the inference algorithm understand specifically which type
> you're trying to parse into.
>
> `parse` can parse into any type that implements the `FromStr` trait.

Three load-bearing claims:
- *"Parses this string slice into another type."* — direct
  audience-level corpus statement of what `.parse()` does. The
  lesson body's "converts its content into a value of some target
  type" rephrases this.
- *"`parse` is so general, it can cause problems with type
  inference"* — direct corpus license for the lesson's central
  observation (the second half of *The Move*): rustc cannot decide
  the target type from `.parse()` alone, because `.parse()` is too
  general. This is *exactly* the behavior the calibration probe
  empirically confirms (E0284 fires when no annotation pins down
  the target).
- *"the syntax affectionately known as the 'turbofish': `::<>`"*
  — explicit corpus mention that the turbofish is one of the two
  ways to disambiguate `.parse()`. The lesson uses the *other* way
  (a `: i32` annotation on the binding) and explicitly defers the
  turbofish form under *What To Ignore For Now*.

Lines 2188-2191 (the error description):

> ##### Errors
>
> Will return `Err` if it's not possible to parse this string slice
> into the desired type.

Direct corpus statement of the failure mode. The broken-contrast
runtime-panic probe corroborates this empirically: `"abc".parse()`
on a binding annotated `: i32` returns `Err(...)` because `"abc"`
is not a parsable `i32`; `.expect("not a number")` then panics per
cycle 053's mechanic.

Lines 2197-2217 (the Examples block):

> Basic usage:
>
> ```
> let four: u32 = "4".parse().unwrap();
>
> assert_eq!(4, four);
> ```
>
> Using the 'turbofish' instead of annotating `four`:
>
> ```
> let four = "4".parse::<u32>();
>
> assert_eq!(Ok(4), four);
> ```
>
> Failing to parse:
>
> ```
> let nope = "j".parse::<u32>();
>
> assert!(nope.is_err());
> ```

The first Examples block is structurally identical to today's
working probe modulo (a) target type `u32` vs `i32` and (b) using
`.unwrap()` instead of `.expect("...")`. The Book uses `u32` for
the guessing-game; the orchestrator chose `i32` for cycle 056 to
reuse the typed name from cycle 019 (the load-bearing
type-annotation prerequisite). Both `u32` and `i32` work
identically through `.parse()`.

The second Examples block (the turbofish form) shows the
alternative the lesson explicitly defers.

The third Examples block ("Failing to parse: `let nope = "j".parse::<u32>();
... nope.is_err()`") is corpus license for today's broken-contrast
probe — `"abc".parse()` (with `: i32` annotation) returns `Err(...)`,
the same way `"j".parse::<u32>()` returns `Err(...)`. The corpus
inspects the `Err` with `.is_err()` (cycle 052 sibling, named only);
the lesson uses `.expect("not a number")` (cycle 053) which converts
the `Err` into a runtime panic.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053, 054, 055. Reused here for the audience-level
walkthrough of `.parse()` in the canonical guessing-game shape. Two
load-bearing spans new to cycle 056.

Lines 935-941 (the audience-level `.parse` description):

> The [`parse` method on strings](../std/primitive.str.md#method.parse) converts a string to
> another type. Here, we use it to convert from a string to a number. We need to
> tell Rust the exact number type we want by using `let guess: u32`. The colon
> (`:`) after `guess` tells Rust we'll annotate the variable's type. Rust has a
> few built-in number types; the `u32` seen here is an unsigned, 32-bit integer.
> It's a good default choice for a small positive number. You'll learn about
> other number types in [Chapter 3](ch03-02-data-types.md#integer-types).

Direct corpus statement of:
- (a) "`parse` ... converts a string to another type" — the
  audience-level statement of the lesson's main install.
- (b) "We need to tell Rust the exact number type we want by using
  `let guess: u32`. The colon (`:`) after `guess` tells Rust we'll
  annotate the variable's type." — the audience-level statement of
  the *second half* of *The Move*: the `: TYPE` annotation is what
  pins down the target type. The Book uses `u32`; today's lesson
  uses `i32`. The colon-annotation mechanic is identical.

Lines 948-959 (the audience-level `Result` + `expect` description):

> The `parse` method will only work on characters that can logically be converted
> into numbers and so can easily cause errors. If, for example, the string
> contained `A👍%`, there would be no way to convert that to a number. Because it
> might fail, the `parse` method returns a `Result` type, much as the `read_line`
> method does (discussed earlier in ["Handling Potential Failure with
> `Result`"](#handling-potential-failure-with-result)). We'll treat
> this `Result` the same way by using the `expect` method again. If `parse`
> returns an `Err` `Result` variant because it couldn't create a number from the
> string, the `expect` call will crash the game and print the message we give it.
> If `parse` can successfully convert the string to a number, it will return the
> `Ok` variant of `Result`, and `expect` will return the number that we want from
> the `Ok` value.

Direct audience-level corpus statement of the chain's two-outcome
behavior:
- *"`parse` returns a `Result` type"* — corpus license for
  threading `.parse()` through cycle 052 (`Result<T, E>`) and
  cycle 053 (`.expect`).
- *"If `parse` returns an `Err` ... the `expect` call will crash
  the game"* — Book-level paraphrase of cycle 053's runtime-panic
  mechanic, applied to `.parse()` specifically. The lesson body's
  panic discussion ("This is exactly cycle 053's panic shape") is
  corpus-grounded by this passage.
- *"If `parse` can successfully convert the string to a number,
  it will return the `Ok` variant of `Result`, and `expect` will
  return the number that we want from the `Ok` value."* — direct
  corpus statement of the success path. The lesson body's "the
  call evaluates to the plain `i32` value `42`" rephrases this.

Calibration: the Book's running example uses
`let guess: u32 = guess.trim().parse().expect("Please type a number!");`
(line 897), which combines `.trim()` (cycle 055) + `.parse()` +
`.expect()` + type-changing shadowing. Cycle 056 installs only
`.parse()` and surfaces type inference; the chain extension to
`.trim().parse()` and the type-changing shadowing pattern are
explicitly deferred. Today's probe uses `"42"` (a string literal,
already a `&str`) to keep `.parse()`'s install maximally visible
without depending on `read_line` + `.trim()` first.

### `output/docs/rust/book/ch03-02-data-types.md`

The Book chapter on data types. Already cited in lesson 019 (the
type-annotation prerequisite). Reused here for the audience-level
statement of *type inference* and the canonical missing-annotation
diagnostic. Two load-bearing spans.

Lines 8-14 (the load-bearing audience-level statement of inference
and when annotations are required):

> Keep in mind that Rust is a *statically typed* language, which means that it
> must know the types of all variables at compile time. The compiler can usually
> infer what type we want to use based on the value and how we use it. In cases
> when many types are possible, such as when we converted a `String` to a numeric
> type using `parse` ..., we must add a type annotation, like this:
>
> ```rust
> #![allow(unused)]
> fn main() {
> let guess: u32 = "42".parse().expect("Not a number!");
> }
> ```

Direct corpus statement of:
- (a) "the compiler can usually infer what type we want to use
  based on the value and how we use it" — Book-level statement of
  *type inference*. The lesson body's "*type inference*: rustc
  reads a binding's `: TYPE` annotation as a constraint on the
  right-hand side" is grounded here, with the small extension that
  the lesson surfaces the *constraining-the-right-hand-side*
  direction explicitly (cycle 056 is the first cycle to surface
  inference of a method's return type from the binding annotation).
- (b) "In cases when many types are possible, such as when we
  converted a `String` to a numeric type using `parse` ..., we
  must add a type annotation" — direct corpus license for the
  lesson's central observation: the annotation `: i32` is what
  pins down the target type that `.parse()` produces. The Book's
  example `let guess: u32 = "42".parse().expect("Not a number!");`
  is structurally identical to the lesson's `let n: i32 = "42".parse().expect("not a number");`.

Lesson 019's evidence appendix (lines 192-197) already cited the
same Book span for the audience-level "type inference" framing.
Cycle 056 reuses lesson 019's grounding by reference and *extends*
it with the new direction (annotation drives the method's return
type).

Lines 23-44 (the canonical missing-annotation diagnostic):

> If we don't add the `: u32` type annotation shown in the preceding code, Rust
> will display the following error, which means the compiler needs more
> information from us to know which type we want to use:
>
> ```console
> $ cargo build
>    Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
> error[E0284]: type annotations needed
>  --> src/main.rs:2:9
>   |
> 2 |     let guess = "42".parse().expect("Not a number!");
>   |         ^^^^^        ----- type must be known at this point
>   |
>   = note: cannot satisfy `<_ as FromStr>::Err == _`
> help: consider giving `guess` an explicit type
>   |
> 2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
>   |              ++++++++++++
> ```

Direct corpus statement of:
- (a) The diagnostic shape rustc produces when an annotation is
  missing on a `.parse()` chain. The lesson's calibration probe
  (transcript below) reproduces this *verbatim* with `i32` and
  `not a number` substituted, and the lesson body describes the
  diagnostic's behavior in audience-level prose without naming
  E0284.
- (b) "The compiler needs more information from us to know which
  type we want to use" — direct corpus license for the lesson
  body's description of the no-annotation case ("rustc cannot pick
  one"). The lesson does NOT install E0284 as an E-code (per
  orchestrator deferral); the diagnostic is described as behavior,
  not as a named E-code.

Calibration: the corpus diagnostic uses `cargo build`; the captured
calibration probe uses `rustc misled.rs` directly. Both produce the
same `error[E0284]: type annotations needed` headline with the same
caret-and-help structure (E-code stable across compiler driver).

### `output/docs/rust/std/primitive.i32.md`

The std page for the primitive `i32` type. Already cited in lesson
019. Reused here for the `FromStr` impl that makes `i32` a valid
target type for `.parse()`. One load-bearing span new to cycle 056.

Lines 3779-3818 (the `FromStr for i32` impl block):

> ### impl FromStr for i32
>
> #### fn from_str(src: &str) -> Result<i32, ParseIntError>
>
> Parses an integer from a string slice with decimal digits.
>
> The characters are expected to be an optional
> `+` or `-`
> sign followed by only digits. Leading and trailing non-digit characters (including
> whitespace) represent an error. Underscores (which are accepted in Rust literals)
> also represent an error.
>
> ##### Examples
>
> ```
> use std::str::FromStr;
>
> assert_eq!(i32::from_str("+10"), Ok(10));
> ```
>
> Trailing space returns error:
>
> ```
> assert!(i32::from_str("1 ").is_err());
> ```
>
> [...]
>
> #### type Err = ParseIntError

Three corpus claims grounded by this span:
- (a) `i32` implements `FromStr`, so `<F = i32>` satisfies the
  where-clause `F: FromStr` on `.parse()`. This is *what makes*
  `"42".parse()` with the `: i32` annotation type-check. The
  lesson body's "rustc concludes that `.parse()` must produce a
  `Result<i32, ...>`" rests on this impl existing. The trait
  machinery itself is deferred; the lesson surfaces only the
  empirical observation that `.parse()` works for `i32` because
  rustc found this impl.
- (b) The associated `Err` type is `ParseIntError`. This is the
  type whose Debug-printed form (`ParseIntError { kind: InvalidDigit }`)
  appears in the captured runtime-panic transcript. The lesson body
  treats `ParseIntError` as text rustc prints in the panic message,
  not as a typed name to use; this corpus span is the grounding for
  acknowledging the name without installing it.
- (c) Whitespace handling: "Leading and trailing non-digit
  characters (including whitespace) represent an error." This is
  direct corpus license for *What To Ignore For Now*'s claim that
  `.parse()` does not trim whitespace, and for the lesson body's
  observation that this is exactly why the Book chains
  `.trim().parse()`. The "Trailing space returns error" example
  (line 3811: `assert!(i32::from_str("1 ").is_err())`) is
  corroborating evidence.

Calibration: the corpus signature is on `i32::from_str`, not on
`str::parse`. These are linked: the std page on `str::parse` (line
2176) returns `Result<F, <F as FromStr>::Err>`, which for `F = i32`
resolves through this `FromStr for i32` impl to
`Result<i32, ParseIntError>`. The lesson black-boxes that linkage
(trait machinery deferred); the empirical confirmation is that
`"42".parse()` with `: i32` produces `Ok(42)` and `"abc".parse()`
with `: i32` produces `Err(ParseIntError { kind: InvalidDigit })`.

### `output/docs/rust/std/result/enum.Result.md`

The std-library page for `Result<T, E>`. Already cited in lessons
052 and 053. Reused here unchanged — `.parse()` returns a
`Result<T, E>` value which today's chain feeds into `.expect`. No
new corpus span needed for cycle 056; the inheritance from cycles
052 + 053 is sufficient.

### Lesson 055's evidence appendix (existing)

The fact that string literals like `"42"` are `&str` is grounded in
`evidence/055-string-trim.md` (citing Book ch04-03 lines 369-381).
Today's lesson reuses that fact directly: `"42".parse()` is a
method call on a `&str` because `"42"` itself is a `&str`. Not
re-cited inline.

### Lesson 053's evidence appendix (existing)

The runtime-panic shape (`thread 'main' (...) panicked at <file>:<line>:<col>:`
followed by `<msg>: <Err payload>` and the `RUST_BACKTRACE=1`
trailer, exit code 101, stderr-only output) is fully grounded in
`evidence/053-result-expect-and-panic.md`. Today's broken-contrast
probe transcript matches that shape exactly; the lesson body
explicitly references cycle 053 as the panic install rather than
re-installing the panic concept. Not re-cited inline.

### Lesson 019's evidence appendix (existing)

The fact that `let name: TYPE = value;` is the Rust annotation
form, plus the audience-level statement that the compiler usually
infers types but sometimes requires an annotation, is fully
grounded in `evidence/019-type-annotation-i32.md` (citing Book
ch03-02 lines 8-14). Cycle 056 *extends* lesson 019 by surfacing
the annotation's *constraining* role (annotation drives the
right-hand side, not just describes it). Not re-cited inline.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/056-str-parse-to-i32.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- ls before ---
demo.rs
--- cat demo.rs ---
fn main() {
    let n: i32 = "42".parse().expect("not a number");
    println!("n = {n}");
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
n = 42
demo-exit=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. The chain `"42".parse().expect("not a number")`
  type-checks: rustc infers `F = i32` from the `: i32` annotation on
  `n`, the `.parse()` call returns a `Result<i32, ParseIntError>`,
  `.expect("not a number")` consumes that into a plain `i32`, and
  the `let n: i32 = ...` slot accepts the value.
- `./demo` prints exactly one line: `n = 42`. The runtime call
  `"42".parse()` evaluates to `Ok(42)` (the `42` is an `i32` per
  the inferred target), `.expect` extracts the `42`, the binding
  takes it, and `println!` interpolates it.
- Exit 0 — no panic on the success path.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe (Shape A — invalid digits, runtime panic)

Source (not committed — the transcript below is the artifact):

```rust
fn main() {
    let n: i32 = "abc".parse().expect("not a number");
    println!("n = {n}");
}
```

The only change from the working probe is the input string:
`"abc"` instead of `"42"`. Captured 2026-05-07 in a fresh
`mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
fn main() {
    let n: i32 = "abc".parse().expect("not a number");
    println!("n = {n}");
}
--- rustc broken.rs ---
rustc-exit=0
--- ls after rustc ---
broken
broken.rs
--- ./broken (stdout+stderr merged) ---

thread 'main' (126328027) panicked at broken.rs:2:32:
not a number: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
broken-exit=101
--- ./broken (stdout split off; stderr captured separately) ---
stdout-empty
stderr:

thread 'main' (126328071) panicked at broken.rs:2:32:
not a number: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exit=101
```

Notes (probe evidence — not corpus quotation):

- `rustc broken.rs` exits 0 with no warnings. The program type-checks
  *identically* to the working probe; the only change is the runtime
  *value* of the `&str` literal. This is the same compile-vs-runtime
  split cycle 053 captured (compile clean, runtime decision).
- Running the executable produces the canonical cycle-053 panic
  shape:
  - `thread 'main' (<thread-id>) panicked at <file>:<line>:<col>:`
    headline (here `broken.rs:2:32` — column 32 is the `.expect`
    call site).
  - Message line `not a number: ParseIntError { kind: InvalidDigit }`
    — the lesson's `"not a number"` message, then `:`, then the
    Debug-printed `ParseIntError` value. Cycle 053's evidence
    showed `expected even: 7` (an `i32` Err); today's `Err` is a
    structured `ParseIntError` value, hence the curly-brace Debug
    form. The lesson body acknowledges this name as text rustc
    prints; `ParseIntError` is explicitly deferred under *What To
    Ignore For Now* and not introduced as a typed name.
  - `note: run with `RUST_BACKTRACE=1` ...` trailer — same
    cycle-053 trailer; not load-bearing.
- Exit code 101 — same as cycle 053. Cycle 053's evidence
  established this as the canonical main-thread-panic exit code
  (per `std/macro.panic.md` line 67-69); today's transcript is the
  second empirical observation of that exit code.
- Output split: stdout is empty (`println!("n = {n}")` never runs),
  the panic block goes to stderr. Same split cycle 053 established.
- The thread id varies between runs (`126328027` vs `126328071`);
  not load-bearing.

This probe is *load-bearing* for the lesson's claim "On `Err` it
panics at runtime — the second program-level use of cycle 053's
panic shape." Without it, the runtime-panic claim would rest only
on cycle 053's grounding (with a synthetic `Result<i32, i32>` Err)
and the corpus signature; the captured transcript is the empirical
confirmation that `.parse() + .expect()` produces the same panic
shape with a real-world `Err` payload (`ParseIntError`).

### Calibration probe (Shape B — no annotation, type-inference failure)

Source (not committed — the transcript below is the artifact):

```rust
fn main() {
    let n = "42".parse().expect("not a number");
    println!("n = {n}");
}
```

The only change from the working probe is removing the `: i32`
annotation. Captured 2026-05-07 in a fresh `mktemp -d` (filename
`misled.rs`):

```text
--- cat misled.rs ---
fn main() {
    let n = "42".parse().expect("not a number");
    println!("n = {n}");
}
--- rustc misled.rs ---
error[E0284]: type annotations needed
 --> misled.rs:2:9
  |
2 |     let n = "42".parse().expect("not a number");
  |         ^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `n` an explicit type
  |
2 |     let n: /* Type */ = "42".parse().expect("not a number");
  |          ++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0284`.
rustc-exit=1
```

Notes (probe evidence — not corpus quotation):

- Headline: `error[E0284]: type annotations needed`. E0284 is a NEW
  E-code (lessons 003-055 have not installed it). The orchestrator
  explicitly directed *not* to install E0284 in the main path, and
  the lesson body avoids naming the E-code. The transcript is
  captured here for audit and to ground the lesson body's prose
  description.
- The headline matches Book ch03-02 line 30's reference diagnostic
  exactly — same E-code, same caret structure. The Book's example
  (`let guess = "42".parse().expect("Not a number!");`) is
  structurally identical to today's calibration probe modulo the
  binding name and the `.expect` message.
- Caret label `type must be known at this point` under `"42".parse`
  — rustc points at the *call*, not the binding. This is the
  audience-readable form of "rustc cannot decide what `.parse()`
  should produce." The lesson body rephrases this as "rustc cannot
  pick one."
- `= note: cannot satisfy `<_ as FromStr>::Err == _``
  — the trait-inference-level explanation. The `<_ as FromStr>::Err`
  expression names the where-clause `F: FromStr` from the `.parse()`
  signature; the `_` placeholders mean rustc could not pick a
  specific `F`. The lesson defers all of this trait machinery; the
  note is included in the transcript verbatim as the empirical
  artifact, but the lesson body does not unpack it.
- `help: consider giving `n` an explicit type` block proposes
  inserting `: /* Type */` between `n` and `=` — i.e., the *exact*
  fix the lesson recommends ("put the annotation back"). The
  audience-level mapping from the rustc help to the lesson's
  prescription is direct.
- `For more information about this error, try `rustc --explain E0284`.`
  — standard E-code trailer. The lesson does not invite the
  learner to run `--explain E0284`.
- Exit code 1 (compile error). No binary produced.

This probe is *load-bearing* for the lesson's third *Predict*
section ("rustc *refuses* this program") and for the *Check
Yourself* (c) answer. Without it, the no-annotation claim would
rest only on the corpus statement at `std/primitive.str.md` line
2180 ("`parse` is so general, it can cause problems with type
inference") and the Book ch03-02 reference diagnostic; the
captured transcript is the empirical confirmation specific to
today's `"42".parse()` shape with `: i32` removed and
`.expect("not a number")` substituted.

Calibration: the lesson body deliberately avoids naming E0284 as
an E-code, instead describing the diagnostic's *behavior* in
audience-level prose ("rustc *refuses* this program. Its diagnostic
essentially says..."). This is the same approach lesson 019 took
(citing the Book's reference diagnostic without installing E0284).
The lesson's *Check Yourself* (c) answer notes "see the evidence
appendix for the exact diagnostic" rather than reproducing the
transcript.

### Side probe — chained `.trim().parse()` for forward-look (not committed)

Auxiliary probe verifying that the natural follow-up cycle (the
`buf.trim().parse().expect(...)` chain) compiles and runs. Not
load-bearing for cycle 056; included to corroborate the lesson
body's deferral note ("the next natural move").

Source:

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let n: i32 = buf.trim().parse().expect("not a number");
    println!("n = {n}");
}
```

Transcript:

```text
--- rustc combo.rs ---
rustc-exit=0
--- echo "42" | ./combo ---
n = 42
exit=0
```

Notes:

- The chain `buf.trim().parse()` compiles. `buf.trim()` produces
  a `&str` (cycle 055), `.parse()` is a method on `&str`, and the
  `: i32` annotation on `n` flows back to pin down the target
  type. Three method calls plus the binding annotation.
- Output is `n = 42` — `buf` after `read_line` contains `"42\n"`,
  `.trim()` strips the `\n` to produce `"42"`, `.parse()` produces
  `Ok(42)`, `.expect` extracts the `42`. Same as the working probe
  modulo the additional `.trim()` step.
- This combination is the natural next cycle. The *type-changing
  shadowing* form `let buf: i32 = buf.trim().parse().expect(...)`
  is also a deferred concept (cycle 007's deferral list). Today's
  side probe uses a separate name `n` to avoid that.

### Side probe — explicit turbofish for forward-look (not committed)

Auxiliary probe verifying that the deferred turbofish form works.
Not load-bearing for cycle 056; included to corroborate the
deferral under *What To Ignore For Now* ("Generics and the
turbofish").

Source:

```rust
fn main() {
    let n = "42".parse::<i32>().expect("not a number");
    println!("n = {n}");
}
```

Transcript:

```text
rustc-exit=0
./demo: n = 42
```

Notes:

- The turbofish `::<i32>` on `.parse` writes the target type
  explicitly at the call site. With the turbofish present, the
  binding annotation `: i32` is no longer required (this probe
  uses `let n = ...` with no annotation, and rustc accepts).
- Behavior is identical to the working probe. The lesson defers
  the turbofish under *What To Ignore For Now*.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 056.

- **Lesson 055 (load-bearing)** — installed `&str` as a typed name,
  with string literals like `"hello"` having type `&str`. Today's
  receiver `"42"` is a `&str` per cycle 055; `.parse()` is a method
  on `&str` (corpus signature `fn parse<F>(&self) -> Result<F, ...>`
  at `std/primitive.str.md` line 2176, where `&self` resolves to
  `&str`). Without cycle 055, `.parse()` would have no installed
  receiver type.
- **Lesson 053 (load-bearing)** — installed `.expect("msg")` as the
  consumer of a `Result<T, E>` that yields the `Ok` payload as a
  plain `T` or panics with `msg: <Err>` on `Err`. Today's chain's
  trailing `.expect("not a number")` is exactly that. The
  broken-contrast runtime-panic probe today produces the canonical
  cycle-053 panic shape (verbatim — same exit code, same stderr
  block format, same `<msg>: <Err>` join). Cycle 056 is the second
  formal cycle that triggers a runtime panic; the panic concept
  itself is reused, not re-installed.
- **Lesson 052 (load-bearing)** — installed `Result<T, E>` as the
  prelude two-variant enum. `.parse()`'s return type is a
  `Result<F, <F as FromStr>::Err>`. The lesson surfaces only the
  audience-level shape "a `Result` with the parsed value on the
  `Ok` side" without unpacking the generic parameters or the
  associated-type spelling.
- **Lesson 049 (load-bearing)** — installed method chaining: the
  receiver of `.method` is *any expression*, including another
  method call. Today's chain `"42".parse().expect("not a number")`
  has two methods: the receiver of `.expect` is the call expression
  `"42".parse()`. Same chain depth as cycle 049's working probe
  (`String::new().is_empty()`); same as cycle 053's
  `parity(4).expect("...")`.
- **Lesson 040 (load-bearing)** — installed the dot-form
  `value.method(args)`. `.parse()` is one such method with no
  arguments, the simplest possible shape lesson 040 named.
- **Lesson 019 (load-bearing)** — installed `let name: TYPE = value;`
  with `i32` as the example. Today's `let n: i32 = "42".parse().expect(...)`
  reuses the same shape and the same target type. Critically,
  lesson 019's evidence appendix (lines 192-197) already cites the
  Book ch03-02 statement "the compiler can usually infer what type
  we want to use ... in cases when many types are possible, such as
  when we converted a `String` to a numeric type using `parse` ...,
  we must add a type annotation." Cycle 056 *picks up* that exact
  forward-pointer: lesson 019 mentioned the parse case as the
  motivating example for required annotations, and cycle 056 is
  where that motivation lands. The new contribution today is the
  *direction* of inference (annotation drives the right-hand side,
  not just describes the left-hand side).
- **Lessons 001, 002, 005** — `rustc file.rs` then `./name`,
  `fn main` entry, `let name = value;` plus the `{name}` placeholder.
  All used unchanged.

## Older supporting lessons

- Lessons 042 (`String::new()` build path) and 054 (`read_line`) —
  not used in today's working probe (today uses the string literal
  `"42"` directly to keep the install minimal). Both are referenced
  as the natural follow-up combination via the side probe.
- Lesson 040 → 049 → 053 → 054 chain (method-call grammar with
  receiver-is-an-expression) — used unchanged.
- Lesson 003 (rustc-diagnostic structure) — used implicitly in the
  calibration-probe transcript description; the lesson body does not
  walk the diagnostic structure, since the load-bearing observation
  is just "rustc refuses this program."
- Lessons 024-034, 045-048, 052, 054, 055 (E0308 family) — *not*
  used today. Cycle 056's runtime-panic broken contrast is not an
  E0308 case (the broken probe compiles cleanly), and the
  type-inference calibration probe fires E0284 (a new E-code, not
  E0308). The E0308 family does not appear in today's evidence.
- Cycle 029 (underscore-prefix gloss) — not used; the binding `n`
  is consumed by `println!`.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe target type is `i32` (cycle 019's typed name), not the
  Book's `u32`. This was an orchestrator choice: cycle 019 is the
  load-bearing type-annotation prerequisite, so reusing `i32` keeps
  the lesson's typed-name dependencies tight. `u32` is mentioned
  in *What To Ignore For Now* as a sibling target type. The Book's
  `u32` works identically through `.parse()`; the only behavioral
  difference is whether negative numbers parse (`i32` accepts
  `"-7"`; `u32` does not).
- The probe input is `"42"` (matches the Book's example at
  ch03-02 line 19). The Book ch02 guessing-game uses dynamic input
  via `read_line`; today's lesson uses a string literal to keep
  `.parse()`'s install free of the `read_line` + `.trim()`
  composition (which is the next deferred cycle).
- The probe message is `"not a number"` (orchestrator suggestion),
  not the Book's `"Please type a number!"` or `"Not a number!"`.
  All three forms work identically — the message is whatever
  string the learner passes. The shorter form keeps the focus on
  the mechanic.
- The probe binding name is `n` (cycle 019's choice for `i32`
  bindings), not the Book's `guess`. `guess` is specific to the
  guessing-game; `n` is generic. Probe behavior is identical.
- The chain is written on one line. Cycle 053 established the
  one-line chain shape with `.expect`; cycle 054 extended to three
  methods on one line. Today's two-method chain is the simplest
  shape since cycle 053.
- The runtime-panic probe is the *primary* broken contrast (Shape
  A) per orchestrator guidance; the no-annotation probe is a
  *calibration* (Shape B). Both transcripts are captured, but the
  lesson body installs only Shape A as the formal predict-and-run,
  and describes Shape B in prose with the diagnostic deferred to
  the appendix.
- The panic transcript reveals `ParseIntError { kind: InvalidDigit }`.
  The orchestrator explicitly directed not to install
  `ParseIntError` as a typed name. The lesson body honors this by
  describing the name as "text rustc prints in the panic message."
  The corpus span at `std/primitive.i32.md` line 3816 (`type Err =
  ParseIntError`) is the grounding for acknowledging the name
  without installing it.
- The calibration probe transcript reveals E0284. The lesson body
  honors the orchestrator's "do not install E0284" directive by
  describing the diagnostic's *behavior* in audience-level prose
  ("rustc *refuses* this program. Its diagnostic essentially says
  that `n`'s type must be known at this point") without naming the
  E-code. The exact transcript lives in this appendix only.
