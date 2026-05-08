// EduRatchet-2 lesson 016 probe: extend if/else with else if arms.
//
// Captured 2026-05-06 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: with n bound to 5, an else-if chain checks each
// condition in source order, runs the first block whose condition is
// true, and skips the rest. With n = 5: condition 1 (n > 10) is false,
// condition 2 (n > 0) is true => block 2 runs ("positive"); the
// remaining conditions (n == 0 and the final else) are not even
// checked. Expected output: positive. Observed: positive.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let n = 5;
//         if n > 10 {
//             println!("very big");
//         } else if n > 0 {
//             println!("positive");
//         } else if n == 0 {
//             println!("zero");
//         } else {
//             println!("negative");
//         }
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
//     positive
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "positive". The other three blocks ("very big", "zero", "negative")
// did not fire — only one block runs per pass through the chain.
//
// The temp directory was removed at the end of the run; only this .rs
// source is committed under observations/. No binary is committed.

fn main() {
    let n = 5;
    if n > 10 {
        println!("very big");
    } else if n > 0 {
        println!("positive");
    } else if n == 0 {
        println!("zero");
    } else {
        println!("negative");
    }
}
