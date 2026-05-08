# Evidence — 083-integer-overflow

Audit appendix for `lessons/083-integer-overflow.md`. The lesson
teaches one move: write a four-line program that overflows `u8` at
runtime, build it under both Cargo profiles, and read the two
outcomes. The lesson installs one new concept with five centered
claims:

1. *Integer overflow* is the Book's name for arithmetic that produces
   a value outside the type's range (distinct from lesson 080's
   compile-time literal-out-of-range rule).
2. **Debug** builds (`cargo build`) panic with `attempt to <op> with
   overflow`, exit `101`.
3. **Release** builds (`cargo build --release`) skip the check; on
   overflow the value performs *two's complement wrapping* (256→0,
   257→1, etc., for `u8`).
4. *Relying on the wrap is considered an error* (Book wording).
5. The standard library exposes four method families for explicit
   overflow handling: `wrapping_*` (always wrap), `checked_*` (return
   `None`), `overflowing_*` (value + bool), `saturating_*` (clamp to
   MIN/MAX).

This appendix covers (a) toolchain, (b) project setup, (c) verbatim
probe transcripts for both profiles plus auxiliaries, (d) committed
observation files, (e) corpus quote map for the five claims, (f)
prerequisite-claim summary, (g) contrast-probe coverage, and (h) what
this probe deliberately does not exercise.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
host: x86_64-apple-darwin
```

`uname -sm` -> `Darwin x86_64`. Probe ran in a fresh `mktemp -d`
directory: `/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.so4t1O6mUO/`.

## Project setup

The committed observation directory
`observations/083-integer-overflow/` mirrors what `cargo new --vcs
none overflow_demo` writes plus a `.gitignore`:

- `Cargo.toml` — bit-exact match to the `cargo new` template:

  ```toml
  [package]
  name = "overflow_demo"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

- `src/main.rs` — the canonical four-line probe:

  ```rust
  fn main() {
      let mut x: u8 = 255;
      x = x + 1;
      println!("x = {}", x);
  }
  ```

- `.gitignore` — listing `/target` and `Cargo.lock`. Build artifacts
  are not committed. Same convention as cycle 082.

The probe form is `let mut x: u8 = 255; x = x + 1;` rather than a
single `let x: u8 = 255 + 1;` because the latter would fall under
lesson 080's compile-time literal-out-of-range rule (rustc evaluates
the addition of two literals at parse/check time and rejects via
`overflowing_literals`). Today's probe needs the addition to occur
*at runtime*. Empirically, `let mut x; x = x + 1;` accomplishes this:
neither rustc 1.95.0 in debug nor in release constant-folds the
overflow away to a compile-time error or warning; instead the runtime
behaviors that the lesson centers (debug panic, release wrap) appear
exactly as expected. See *Probe transcript* below.

## Probe transcript

All steps run from
`/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.so4t1O6mUO/overflow_demo`.

### Step 1: Toolchain capture

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64
```

### Step 2: Scaffold the package

```text
$ cargo new --vcs none overflow_demo
    Creating binary (application) `overflow_demo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd overflow_demo
```

Same shape as cycles 064 / 082 step 1.

### Step 3: Write the probe

`src/main.rs` was edited to:

```rust
fn main() {
    let mut x: u8 = 255;
    x = x + 1;
    println!("x = {}", x);
}
```

The `cargo new` default `Hello, world!` body was replaced; no other
files changed.

### Step 4: Debug build

```text
$ cargo build
   Compiling overflow_demo v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.so4t1O6mUO/overflow_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
exit=0
```

No warnings, no errors. Identical `Finished` line shape to cycle 064
/ 082 step 3 (`dev` profile, `[unoptimized + debuginfo]`). The
`overflowing_literals` lint from lesson 080 does *not* fire here —
there is no out-of-range literal; both `255u8` and `1u8` are in
range. The `arithmetic_overflow` lint that catches some constant
overflow at compile time (e.g., `let z: u8 = 255u8 + 1u8;` directly)
also does not fire — the expression `x + 1` involves a binding `x`
that rustc does not constant-evaluate across the mutable
reassignment. This is what allows the lesson to demonstrate runtime
overflow with a small probe.

### Step 5: Run the debug binary (load-bearing for claim 2)

```text
$ ./target/debug/overflow_demo
thread 'main' (133066278) panicked at src/main.rs:3:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101
```

Three facts the lesson centers:

- The headline is `attempt to add with overflow`. Verbatim. (The
  Subtract auxiliary, step 8, gives `attempt to subtract with
  overflow`. Same shape, op-name slot.)
- The trailer matches lesson 053 / 078 exactly: the `thread 'main'
  (...) panicked at <file>:<line>:<col>:` line, the `RUST_BACKTRACE=1`
  trailer, and exit `101`.
- The output `x = ...` line never appears — the panic happens before
  control reaches `println!`.

The location `src/main.rs:3:9` points to the `x = x + 1;` statement
(line 3, column 9 — the `x` on the left of `=`).

### Step 6: Release build

```text
$ cargo build --release
   Compiling overflow_demo v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.so4t1O6mUO/overflow_demo)
    Finished `release` profile [optimized] target(s) in 0.17s
exit=0
```

Identical `Finished` line shape to cycle 082 step 4 (`release`
profile, `[optimized]`).

### Step 7: Run the release binary (load-bearing for claim 3)

```text
$ ./target/release/overflow_demo
x = 0
$ echo $?
0
```

`x = 0` confirms the *two's complement wrap*: `255 + 1 = 256`, which
is one past the `u8` range, wraps to the minimum `0`. No panic,
clean exit `0`. Output appears on stdout, not stderr.

### Step 8: Auxiliary — wrap continues for 257

`src/main.rs` edited to `x = x + 2;`:

```text
$ cargo build --release
   Compiling overflow_demo v0.1.0 (...)
    Finished `release` profile [optimized] target(s) in 0.16s
$ ./target/release/overflow_demo
x = 1
exit=0
```

`257 → 1`. This corroborates the Book's "the value 257 becomes 1, and
so on" claim (lines 130-131). The lesson's *Try It* names this case
as an optional prediction.

### Step 9: Auxiliary — subtract overflow

`src/main.rs` edited to:

```rust
fn main() {
    let mut x: u8 = 0;
    x = x - 1;
    println!("x = {}", x);
}
```

```text
$ cargo build
   Compiling overflow_demo v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
$ ./target/debug/overflow_demo
thread 'main' (133067931) panicked at src/main.rs:3:9:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exit=101

$ cargo build --release
   Compiling overflow_demo v0.1.0 (...)
    Finished `release` profile [optimized] target(s) in 0.16s
$ ./target/release/overflow_demo
x = 255
exit=0
```

Two corroborations:

- The debug-mode headline slot is op-name-parameterized: `attempt to
  subtract with overflow` here (vs `attempt to add with overflow` in
  step 5). The lesson's *What Changed* names `add`, `subtract`,
  `multiply`, etc. as instances of the same `attempt to <op> with
  overflow` family.
- Subtractive wrap goes the other direction: `0u8 - 1 = -1`, which is
  one *below* the `u8` range, wraps to the *maximum* `255`. Same
  two's complement scheme, opposite end. The lesson's *What Changed*
  names this as "(going under) 0-1 becomes 255".

The lesson's `## What To Ignore For Now` notes that the Book treats
subtractive wrap as the same *integer overflow* concept (it does not
introduce *underflow* as a separate name). This matches the Book's
own framing at lines 116-118 ("If you try to change the variable to
a value outside that range").

### Step 10: Restore canonical probe

After step 9, `src/main.rs` was restored to the canonical four-line
form (255, `+ 1`) that ships in the committed observation directory.

## Corpus quote map

### `output/docs/rust/book/ch03-02-data-types.md` lines 114-143

The Book's *Integer Overflow* subsection. Reproduced inline for the
audit; the load-bearing source quotes for each centered claim follow.

#### Claim 1: definition of integer overflow

Lines 116-118 (load-bearing for *The Move* and *What Changed*
bullet 1):

> Let's say you have a variable of type `u8` that can hold values
> between 0 and 255. If you try to change the variable to a value
> outside that range, such as 256, *integer overflow* will occur,
> which can result in one of two behaviors.

The lesson uses the same `u8` framing and the `255 + 1 = 256` case as
the smallest concrete instance. The Book introduces *integer
overflow* as italicized terminology; the lesson preserves the
italicization in *What Changed*.

#### Claim 2: debug-mode panic

Lines 119-124 (load-bearing for the debug-build outcome and *What
Changed* bullet 2):

> When you're compiling in debug mode, Rust includes checks for
> integer overflow that cause your program to *panic* at runtime if
> this behavior occurs. Rust uses the term *panicking* when a program
> exits with an error; we'll discuss panics in more depth in the
> ["Unrecoverable Errors with `panic!`"]
> ... section in Chapter 9.

The Book attributes the panic to "checks for integer overflow" that
"cause your program to *panic* at runtime" — exactly the lesson's
*What Changed* bullet 2 wording ("rustc inserts overflow checks; on
overflow the program panics"). The exact panic *message*
(`attempt to add with overflow` etc.) is not in the Book — it comes
from rustc's runtime check in stdlib's panic-on-overflow path.
Verbatim capture in step 5 of the probe transcript above is the
authoritative grounding for the message text.

#### Claim 3: release-mode wrap

Lines 126-131 (load-bearing for the release-build outcome and *What
Changed* bullet 3):

> When you're compiling in release mode with the `--release` flag,
> Rust does *not* include checks for integer overflow that cause
> panics. Instead, if overflow occurs, Rust performs *two's
> complement wrapping*. In short, values greater than the maximum
> value the type can hold "wrap around" to the minimum of the values
> the type can hold. In the case of a `u8`, the value 256 becomes 0,
> the value 257 becomes 1, and so on.

The lesson restates `256 → 0` and `257 → 1` directly, with step 7
(value `0`) and step 8 (value `1`) of the probe transcript as the
empirical witness. The phrase *two's complement wrapping* is the
Book's italicized terminology; the lesson preserves the italics. The
"wrap around" framing extends to the under-flow direction in step 9
(`0 - 1 → 255`), which the Book covers under the same *integer
overflow* umbrella per claim 1.

#### Claim 4: relying on the wrap is considered an error

Lines 131-133 (load-bearing for *What Changed* bullet 4):

> The program won't panic, but the variable will have a value that
> probably isn't what you were expecting it to have. Relying on
> integer overflow's wrapping behavior is considered an error.

The lesson restates this verbatim ("Relying on the wrap is
*considered an error* (Book wording)") and adds the framing that the
release behavior exists "so optimized code does not pay for the
check, not as a feature for arithmetic" — the latter is the lesson's
explanation, not a Book quote, and is consistent with the Book's
"checks for integer overflow that cause your program to *panic*"
framing.

#### Claim 5: the four method families

Lines 135-143 (load-bearing for *What Changed* bullet 5 and the
*Check Yourself* (b) and (c) prompts):

> To explicitly handle the possibility of overflow, you can use these
> families of methods provided by the standard library for primitive
> numeric types:
>
> - Wrap in all modes with the `wrapping_*` methods, such as
>   `wrapping_add`.
> - Return the `None` value if there is overflow with the `checked_*`
>   methods.
> - Return the value and a Boolean indicating whether there was
>   overflow with the `overflowing_*` methods.
> - Saturate at the value's minimum or maximum values with the
>   `saturating_*` methods.

The lesson reproduces the four families' names verbatim and the
brief Book descriptions in *What Changed* bullet 5. The lesson does
*not* call any of these methods, write a probe involving them, or
introduce `Option<T>` / `(T, bool)` / `MIN` / `MAX` as named
concepts. Each family is named only — the *Check Yourself* prompts
test the family-name-to-description mapping, nothing more. The
example method `wrapping_add` is named in the Book's bullet itself;
the lesson's *What To Ignore* names `u8::wrapping_add(self, rhs:
u8) -> u8` as deferred.

## Prerequisite-claim summary

### From lesson 080 (`080-integer-type-family`) — *direct, load-bearing*

- `u8` is the unsigned 8-bit integer type with range `0..=255`. The
  range is enforced by rustc at compile time when the value is a
  *literal* — `let too_big: u8 = 256;` fires `error: literal out of
  range for `u8`` with the gloss `the literal `256` does not fit
  into the type `u8` whose range is `0..=255``.
- Today centers the *other* case: when the out-of-range value comes
  from arithmetic, rustc cannot enforce the range at compile time,
  and the runtime behavior is what the Book calls *integer overflow*.
- The lesson references `u8` and the range `0..=255` exactly as 080
  named them.

### From lesson 064 (`064-cargo-build-standalone`) — *direct, load-bearing*

- `cargo build` from inside a Cargo package directory compiles
  `src/main.rs` and writes the executable to `target/debug/<name>`,
  printing ``Finished `dev` profile [unoptimized + debuginfo]
  target(s) in <time>s``. Today's debug-mode panic comes from the
  binary at `target/debug/overflow_demo`.
- The `./target/debug/<name>` invocation shape is reused unchanged.

### From lesson 082 (`082-cargo-build-release`) — *direct, load-bearing*

- `cargo build --release` produces an optimized binary at
  `target/release/<name>` and the `Finished` line says ``Finished
  `release` profile [optimized] target(s) in <time>s``. Today's
  release-mode wrap comes from the binary at
  `target/release/overflow_demo`.
- The `./target/release/<name>` invocation shape is reused unchanged.
- Cycle 082 explicitly named today's lesson as a future move
  (queue item I): "the `--release` flag's effect on other things
  (LTO, codegen-units etc.) — already deferred from lesson 082".
  Today closes the *overflow-checks* piece of the
  release-vs-debug distinction.

### From lesson 053 (`053-result-expect-and-panic`) and lesson 078 (`078-array-out-of-bounds-panic`) — *direct, load-bearing*

- The runtime *panic* trailer shape: `thread 'main' (...) panicked
  at <file>:<line>:<col>:` on stderr, message line, `note: run with
  `RUST_BACKTRACE=1` ... ` line, exit code `101`. Today's debug-mode
  overflow panic uses that exact shape, with `attempt to <op> with
  overflow` in the message slot.
- The lesson's *What Changed* bullet 2 says "Same panic trailer and
  exit `101` shape as lesson 053/078"; this prerequisite is what
  licenses that statement.

### From lesson 006 (`006-mut-binding`) — *cited*

`let mut x: u8 = 255; x = x + 1;` is the cited probe form. The
mutable binding is what keeps the runtime arithmetic out of rustc's
constant-folding reach (see step 4 of the probe transcript for the
empirical witness). Without `mut`, the assignment `x = x + 1;` would
fail at compile time with `cannot assign twice to immutable variable`
(E0384, lesson 006).

### From lessons 005, 011, 032 — *supporting*

Mentioned by number/title only. Lesson 005 grounds `let`. Lesson 011
grounds `println!("{}", expr)`. Lesson 032 grounds `cargo new <name>`
package scaffolding. None are load-bearing in any way not already
covered by lessons 064 / 082.

## Contrast-probe coverage

The lesson's central contrastive claim is the *debug-vs-release
outcome split for the same source*. Steps 4-7 sit side-by-side as
the empirical witness:

- Same `Cargo.toml` and `src/main.rs`.
- Same `cargo new --vcs none` scaffold.
- Only the `--release` flag differs.
- Two outcomes: debug panics with `attempt to add with overflow`,
  exit `101`; release prints `x = 0`, exit `0`.

Both outcomes are valid — neither is a compile-time error. The
contrast is between two *different runtime behaviors* of the same
source under two profiles, not a working-vs-broken contrast, so a
negative compile probe is not the right shape; the positive contrast
of the two profile transcripts *is* the contrast probe.

The lesson also makes a contrastive claim against lesson 080: "rustc
catches an out-of-range *literal* at compile time, but cannot catch
an out-of-range *arithmetic result* at compile time". Lesson 080's
contrast probe (`let too_big: u8 = 256;` rejected at compile time) is
the negative side; today's working probe (`let mut x: u8 = 255; x =
x + 1;` compiles, then runtime-panics or runtime-wraps) is the
positive side. No new compile-time-error probe is needed — lesson 080
already captured it.

For the *four method families* claim, no probe is captured. The Book
introduces the families by name; the lesson installs the names only
and explicitly defers the method-call mechanics. This matches the
worker-prompt instruction "no method-call mechanics, no full
semantics, no probe".

## What is not in this probe

The probe deliberately does not exercise:

- The four standard-library method families themselves
  (`wrapping_add`, `checked_add`, `overflowing_add`,
  `saturating_add`). Named only; future moves.
- The `MIN` / `MAX` associated constants (`u8::MAX == 255`,
  `i8::MIN == -128`). Lesson 080 deferred these.
- `Option<T>` and the `(T, bool)` tuple as named return types.
  Future moves; method-family return types are deferred.
- The `arithmetic_overflow` deny-by-default lint that catches some
  constant-foldable overflow at compile time. Distinct mechanism;
  out of scope.
- The `[profile.dev]` / `[profile.release]` `overflow-checks =
  bool` setting that lets you toggle the check independently of
  profile. Profile-tuning was deferred from cycle 082.
- Other `--release` effects (LTO, codegen-units, opt-level, panic
  strategy). Deferred from cycle 082.
- The `wrapping_neg` / `unchecked_*` / `strict_*` method families.
  Newer additions outside Book scope.
- The `std::num::Wrapping<T>` newtype. Alternative API; deferred.
- IEEE-754 floating-point overflow / underflow. Different family,
  different rules.
- *Two's complement representation* as a typed concept. Lesson 080's
  deferral stands; the Book's phrase *two's complement wrapping* is
  reused in the lesson only as a name for the wrap behavior.
- Multiplication and division overflow. The auxiliary in step 9
  exercises subtractive overflow; multiplicative overflow follows
  the same `attempt to <op> with overflow` headline pattern but is
  not captured.
- Signed-integer overflow (e.g., `i8::MAX + 1` wraps to `i8::MIN ==
  -128` in release). The Book's worked example is `u8`; today
  follows the Book.
- Windows path separators. Cycle 064 / 082 deferred this.

## Files committed for this cycle

- `lessons/083-integer-overflow.md` (the lesson)
- `evidence/083-integer-overflow.md` (this appendix)
- `observations/083-integer-overflow/Cargo.toml`
- `observations/083-integer-overflow/src/main.rs`
- `observations/083-integer-overflow/.gitignore`
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
