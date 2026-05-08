// Observation for lesson 040-method-call-syntax.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: calling a method on a value with `value.method(args)`
// syntax, concretely `let m: i32 = n.abs();` where `n: i32 = -7`. The
// dot-form `n.abs()` produces the absolute value `7`. This is a
// distinct syntactic surface from lesson 008's free-function call form
// `name(args)`; the broken-contrast probe (transcript in evidence
// appendix, source not committed) confirms that writing `abs(n)`
// instead of `n.abs()` fails with E0425, and rustc's own help text
// suggests the dot-form fix.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let n: i32 = -7;
//         let m: i32 = n.abs();
//         println!("n = {n}, m = {m}");
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
//     n = -7, m = 7
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "n = -7, m = 7". The temp directory was removed at the end of the
// run; only this .rs source is committed under observations/. No
// binary is committed.

fn main() {
    let n: i32 = -7;
    let m: i32 = n.abs();
    println!("n = {n}, m = {m}");
}
