// Observation for lesson 044-use-declaration.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: writing `use std::cmp::min;` at the top of the source file
// so the rest of the file can call `min(3, 5)` by its final segment alone,
// instead of writing the full nested path `std::cmp::min(3, 5)` every
// time. The probe binds two values from the same `min` function — once
// via the full path `std::cmp::min(3, 5)`, once via the bare name
// `min(3, 5)` enabled by the `use` line — and prints both. Identical
// output (`3` and `3`) is the empirical equivalence.
//
// The broken-contrast probe (transcript in the evidence appendix, source
// not committed) keeps the `use std::cmp::min;` line and adds a sibling
// call `max(3, 5)` whose name was NOT imported. rustc fires E0425
// "cannot find function `max` in this scope" — same E-code lessons 005,
// 008, 040, 042, and 043 installed — and `help:` suggests a *separate*
// `use std::cmp::max;` line. The contrast confirms that `use` brings in
// ONLY the items it names; sibling items in the same module (like `max`,
// reachable via `std::cmp::max`) are not pulled in by `use std::cmp::min;`.
//
// Source compiled (verbatim):
//
//     use std::cmp::min;
//
//     fn main() {
//         let full: i32 = std::cmp::min(3, 5);
//         let short: i32 = min(3, 5);
//         println!("full = {full}, short = {short}");
//     }
//
// Transcript:
//
//     --- rustc --version ---
//     rustc 1.95.0 (59807616e 2026-04-14)
//     --- uname -sm ---
//     Darwin x86_64
//     --- ls before ---
//     demo.rs
//     --- rustc demo.rs ---
//     exit=0
//     --- ls after ---
//     demo
//     demo.rs
//     --- ./demo ---
//     full = 3, short = 3
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "full = 3, short = 3", confirming that std::cmp::min(3, 5) and the
// bare-name min(3, 5) (enabled by the `use` line) returned the same value
// 3. The temp directory was removed at the end of the run; only this .rs
// source is committed under observations/. No binary is committed.

use std::cmp::min;

fn main() {
    let full: i32 = std::cmp::min(3, 5);
    let short: i32 = min(3, 5);
    println!("full = {full}, short = {short}");
}
