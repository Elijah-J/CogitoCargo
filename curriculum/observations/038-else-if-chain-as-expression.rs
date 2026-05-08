// Observation for lesson 038-else-if-chain-as-expression.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: a multi-arm `if`/`else if`/`else` chain (lesson 016) sits
// on the right of `let` and produces a value (lesson 026 generalized).
// With n = 75, conditions are checked in source order:
//   - 75 >= 90 is false
//   - 75 >= 80 is false
//   - 75 >= 70 is true  => arm 3 runs; tail value is 2
// The whole chain evaluates to 2; `let grade: i32 = ...;` binds `grade`
// to 2. The trailing `else { 1 }` is required so that the chain has a
// defined value-producing arm even when every earlier condition is
// false; without it the chain's no-block fallback is `()` (lesson 029)
// and would collide with the `: i32` annotation.
//
// All four arm tail expressions (`4`, `3`, `2`, `1`) are integer
// literals of the same type, so the same-type-arms rule from lesson 026
// is satisfied.
//
// No new mechanism: this lesson combines lesson 016's chain shape with
// lesson 026's `if`-as-expression rule. No broken-contrast probe is
// captured here — the available broken contrasts (arms with mismatched
// types, or omitting the trailing `else`) were already exercised in
// lessons 026 and 029 respectively.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let n: i32 = 75;
//         let grade: i32 = if n >= 90 {
//             4
//         } else if n >= 80 {
//             3
//         } else if n >= 70 {
//             2
//         } else {
//             1
//         };
//         println!("n = {n}, grade = {grade}");
//     }
//
// Transcript:
//
//     --- rustc --version ---
//     rustc 1.95.0 (59807616e 2026-04-14)
//     --- uname -sm ---
//     Darwin x86_64
//     --- ls before compile ---
//     demo.rs
//     --- rustc demo.rs ---
//     exit=0
//     --- ls after compile ---
//     demo
//     demo.rs
//     --- ./demo ---
//     n = 75, grade = 2
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "n = 75, grade = 2". The temp directory was removed at the end of
// the run; only this .rs source is committed under observations/. No
// binary is committed.

fn main() {
    let n: i32 = 75;
    let grade: i32 = if n >= 90 {
        4
    } else if n >= 80 {
        3
    } else if n >= 70 {
        2
    } else {
        1
    };
    println!("n = {n}, grade = {grade}");
}
