---
id: 083-integer-overflow
status: accepted
evidence: ../evidence/083-integer-overflow.md
---

# Integer overflow at runtime: debug panics, release wraps

## The Move

Lesson 080 caught `let too_big: u8 = 256;` at compile time. Today is
what happens when the out-of-range value comes from *arithmetic* —
rustc cannot evaluate it ahead of time, and the runtime behavior
depends on the build profile from lessons 064 and 082. The Book calls
this *integer overflow*.

Inside a Cargo package, write this `src/main.rs`:

```rust
fn main() {
    let mut x: u8 = 255;
    x = x + 1;
    println!("x = {}", x);
}
```

`255` is the top of `u8`. The next statement reassigns `x` to `x + 1`,
which arithmetically is `256` — one past the range. Build it both ways:

```console
$ cargo build
   Compiling overflow_demo v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
$ ./target/debug/overflow_demo
thread 'main' (...) panicked at src/main.rs:3:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101
```

```console
$ cargo build --release
   Compiling overflow_demo v0.1.0 (...)
    Finished `release` profile [optimized] target(s) in 0.17s
$ ./target/release/overflow_demo
x = 0
$ echo $?
0
```

Same source. Two profiles. Two outcomes:

- **Debug** *panics* with the headline `attempt to add with overflow`,
  exit `101`. Same panic shape as lesson 053/078, different message.
- **Release** runs to completion: `x = 0`, exit `0`. The value wrapped.

That contrast is today's central observation.

## Mental Model Delta

- *Before:* "Each integer type has a range. Lesson 080 showed
  out-of-range *literals* are a compile-time error."
- *After:* "When the out-of-range value comes from runtime arithmetic,
  the Book calls it *integer overflow* and the behavior depends on
  the profile. **Debug** (lesson 064): rustc inserts overflow checks
  and the program panics with `attempt to <op> with overflow`, exit
  101. **Release** (lesson 082): no checks; the value performs *two's
  complement wrapping* (256 becomes 0 for `u8`, etc.). Relying on
  the wrap is *considered an error*. The standard library has four
  named method families for explicit control: `wrapping_*`,
  `checked_*`, `overflowing_*`, `saturating_*`."

## Prerequisites

- Installed concepts:
  - Lesson 080 (load-bearing): `u8` has range `0..=255`; an
    out-of-range *literal* is a compile-time error. Today is the
    arithmetic case.
  - Lesson 064 (load-bearing): `cargo build` produces
    `target/debug/<name>` (the *debug* binary).
  - Lesson 082 (load-bearing): `cargo build --release` produces
    `target/release/<name>` (the *release* binary).
  - Lesson 053/078 (load-bearing): runtime *panic* shape — `thread
    'main' (...) panicked at <file>:<line>:<col>:`, message line,
    `RUST_BACKTRACE=1` trailer, exit `101`. Today's debug panic
    uses that shape with a different headline.
  - Lesson 006 (cited): `let mut x; ...; x = ...;` reassignment, used
    to keep the value out of rustc's reach for constant folding.
  - Lessons 005, 011, 032 (cited): `let`, `println!`, `cargo new`.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `cargo` on `PATH`; `cd`; `echo $?` prints the shell exit status.

## Try It

```console
$ cargo new --vcs none overflow_demo
$ cd overflow_demo
```

Edit `src/main.rs` to the four-line program from *The Move*. Run the
two profiles back to back as shown. Confirm: debug panics with
`attempt to add with overflow`, exit `101`; release prints `x = 0`,
exit `0`.

Optional: change `+ 1` to `+ 2` and rebuild release. (`257` wraps to
`1`.) Or change `255` to `0` and `+ 1` to `- 1`. (Debug panics with
`attempt to subtract with overflow`; release wraps to `255`.)

## What Changed

- *Integer overflow at runtime* is the Book's name for arithmetic that
  produces a value outside the type's range. Lesson 080's rule was
  a different mechanism — it catches a *literal* at compile time.
- *Debug builds* (lesson 064) include overflow checks. On overflow,
  the program panics with `attempt to <op> with overflow` (`add`,
  `subtract`, `multiply`, etc.). Same panic trailer and exit `101`
  shape as lesson 053/078.
- *Release builds* (lesson 082) skip those checks. On overflow the
  value performs *two's complement wrapping*: for `u8`, 256 becomes
  0, 257 becomes 1, and (going under) 0-1 becomes 255.
- *Relying on the wrap is considered an error* (Book wording). The
  release behavior exists so optimized code does not pay for the
  check, not as a feature for arithmetic.
- *Four standard-library method families* exist on the primitive
  numeric types for explicit overflow handling:
  - `wrapping_*` — always wrap, in any profile.
  - `checked_*` — return `None` on overflow.
  - `overflowing_*` — return the value and a `bool` indicating overflow.
  - `saturating_*` — clamp to the type's MIN or MAX.

  Today only names them. The method-call mechanics, return types,
  and `MIN`/`MAX` constants are future moves.

## Check Yourself

(a) `let mut x: u8 = 255; x = x + 1;` — what does each profile do at
runtime?

(b) Which method family returns `None` on overflow?

(c) Which method family clamps to the type's MIN/MAX?

*(Answers: (a) Debug: panics with `attempt to add with overflow`,
exit `101`. Release: `x` becomes `0`, no panic, exit `0`. (b)
`checked_*`. (c) `saturating_*`.)*

## What To Ignore For Now

- The actual signatures of the four method families (e.g.,
  `u8::wrapping_add(self, rhs: u8) -> u8`), method-call invocation,
  and the return-type machinery (`Option<T>` for `checked_*`,
  `(T, bool)` for `overflowing_*`). Each family is a future move.
- The `MIN` / `MAX` associated constants named in `saturating_*`'s
  description (`u8::MAX == 255`, etc.). Lesson 080 deferred these.
- The deny-by-default `arithmetic_overflow` lint that catches some
  obviously-overflowing arithmetic at compile time — separate
  mechanism from today's runtime checks.
- The `[profile.*]` `overflow-checks` setting that toggles the check
  independently of profile. Profile-tuning is deferred from 082.
- Other `--release` effects (LTO, codegen-units, opt-level), same
  deferral as 082; the `wrapping_neg` / `unchecked_*` / `strict_*`
  method families (newer, outside Book scope); the `std::num::Wrapping<T>`
  newtype; IEEE-754 floating-point overflow.
- *Underflow* as a separate name — the Book treats subtractive wrap
  as the same overflow concept.
- *Two's complement representation* as a typed concept (lesson 080
  named it; today reuses the Book's phrase for the wrap behavior).

## Evidence

See `../evidence/083-integer-overflow.md`.
