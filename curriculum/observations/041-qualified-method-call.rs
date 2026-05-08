// Observation for lesson 041-qualified-method-call.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: calling a method using the qualified form
// `Type::method(receiver, args)` — concretely `i32::abs(n)` — and
// checking that it produces the same value as lesson 040's dot-form
// `n.abs()`. Both call shapes evaluate to `7` for the input `-7`.
//
// The broken-contrast probe (zero-argument `i32::abs()`, transcript in
// the evidence appendix, source not committed) confirms that the
// receiver is mandatory in the qualified form — omitting it fires
// E0061 "this function takes 1 argument but 0 arguments were supplied"
// with `note: method defined here` and `help: provide the argument`.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let n: i32 = -7;
//         let dot: i32 = n.abs();
//         let qual: i32 = i32::abs(n);
//         println!("dot = {dot}, qualified = {qual}");
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
//     dot = 7, qualified = 7
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "dot = 7, qualified = 7", confirming the equivalence of the two call
// forms. The temp directory was removed at the end of the run; only
// this .rs source is committed under observations/. No binary is
// committed.

fn main() {
    let n: i32 = -7;
    let dot: i32 = n.abs();
    let qual: i32 = i32::abs(n);
    println!("dot = {dot}, qualified = {qual}");
}
