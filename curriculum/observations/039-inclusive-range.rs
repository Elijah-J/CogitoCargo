// Observation for lesson 039-inclusive-range.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: writing `0..=5` (with `=` between the dots and `5`)
// instead of lesson 022's `0..5` makes a `for` loop also iterate the
// upper bound `5`. Both ranges sit inside the same `for var in range
// { ... }` shape from lesson 022; the only syntactic delta is the `=`.
//
// The program runs two loops over the integers `0..N` and `0..=N` for
// N = 5, accumulating the iterated values into separate `i32`
// accumulators with `+=` (lesson 023, on `let mut` from lesson 006).
// The exclusive form `0..5` produces 0,1,2,3,4 (five iterations,
// sum 10). The inclusive form `0..=5` produces 0,1,2,3,4,5 (six
// iterations, sum 15). The difference, 15 - 10 = 5, is exactly the
// upper bound, which the inclusive form includes and the exclusive
// form excludes.
//
// No broken-contrast probe is captured: the natural broken contrast
// for `..=` would be a syntactically invalid form (e.g. `0..=` with
// no upper bound is a parse error), but the lesson's main point is
// the `..` vs `..=` contrast, both syntactically valid. Both forms
// are exercised in this single program.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let mut sum_excl: i32 = 0;
//         for n in 0..5 {
//             sum_excl += n;
//         }
//         let mut sum_incl: i32 = 0;
//         for n in 0..=5 {
//             sum_incl += n;
//         }
//         println!("0..5 sum = {sum_excl}");
//         println!("0..=5 sum = {sum_incl}");
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
//     0..5 sum = 10
//     0..=5 sum = 15
//     exit=0
//
// rustc was silent on success. ./demo printed exactly two lines:
// "0..5 sum = 10" and "0..=5 sum = 15". The temp directory was
// removed at the end of the run; only this .rs source is committed
// under observations/. No binary is committed.

fn main() {
    let mut sum_excl: i32 = 0;
    for n in 0..5 {
        sum_excl += n;
    }
    let mut sum_incl: i32 = 0;
    for n in 0..=5 {
        sum_incl += n;
    }
    println!("0..5 sum = {sum_excl}");
    println!("0..=5 sum = {sum_incl}");
}
