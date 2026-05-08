// Probe for lesson 094 — the `unused_must_use` warning on a discarded `Result`.
//
// Witnesses that calling a function whose return type is `Result<T, E>`
// (here `Stdin::read_line`, lesson 054) and *discarding* the value — no
// `let x = ...`, no `.expect(...)`, no `match`, no `if let` — fires the
// `unused_must_use` lint at compile time. The lint is in rustc's
// warn-by-default group (lesson 069's category), so the build still
// succeeds: an executable is produced and the program runs.
//
// Compile (paths and column counts shift if you save the source under a
// different filename or with comments stripped — the bordered headline
// and `help:` block stay the same):
//
//     $ rustc demo.rs
//     warning: unused `Result` that must be used
//      --> demo.rs:5:5
//       |
//     5 |     io::stdin().read_line(&mut buf);
//       |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       |
//       = note: this `Result` may be an `Err` variant, which should be handled
//       = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
//     help: use `let _ = ...` to ignore the resulting value
//       |
//     5 |     let _ = io::stdin().read_line(&mut buf);
//       |     +++++++
//
//     warning: 1 warning emitted
//     (exit 0; the executable is produced next to the source)
//
// Run:
//
//     $ echo "hello" | ./demo
//     got: hello
//
// The contrast probe in evidence/094-unused-must-use-result.md replaces
// line 5 with `let _ = io::stdin().read_line(&mut buf);` and observes
// that rustc compiles silently (no warning). `let _ = expr;` is the
// `help:` line's suggested form: it deliberately ignores the value and
// silences the lint.

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    println!("got: {}", buf);
}
