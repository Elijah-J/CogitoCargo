// Probe for lesson 013-comparison-operators.
//
// Combine two integer values with comparison operators (`>`, `==`, `<=`)
// to produce a boolean, bind it with `let`, and print it.
//
// Captured transcript (reproducible with `rustc main.rs && ./main`
// inside a clean temp dir):
//
//   --- rustc --version ---
//   rustc 1.95.0 (59807616e 2026-04-14)
//   --- uname -sm ---
//   Darwin x86_64
//   --- ls before compile ---
//   main.rs
//   --- rustc main.rs ---
//   exit=0
//   --- ls after compile ---
//   main
//   main.rs
//   --- ./main ---
//   a > b is true
//   a == b is false
//   a <= b is false
//   exit=0
//
// Notes:
// - rustc is silent on success (consistent with lesson 001).
// - Three printed lines, in source order (lesson 004).
// - `a > b` is `true` because `5 > 3`.
// - `a == b` is `false` because `5` is not equal to `3`.
// - `a <= b` is `false` because `5` is greater than `3`, not less-or-equal.
// - The compound operator `<=` produces exactly what the appendix table
//   says: "Less than or equal to comparison".
fn main() {
    let a = 5;
    let b = 3;
    let bigger = a > b;
    let equal = a == b;
    let no_more = a <= b;
    println!("a > b is {bigger}");
    println!("a == b is {equal}");
    println!("a <= b is {no_more}");
}
