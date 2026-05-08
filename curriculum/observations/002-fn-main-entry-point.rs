// Probe source for EduRatchet-2 lesson 002-fn-main-entry-point.
//
// This file is the BROKEN contrast for the lesson: the function is
// named `start` instead of `main`. Compiling it should fail with
// rustc error E0601 ("`main` function not found in crate ...").
//
// To reproduce the diagnostic, copy this file into an empty directory
// and compile it:
//   rustc 002-fn-main-entry-point.rs
// Expected: rustc exits 1 and prints
//   error[E0601]: `main` function not found in crate `002-fn-main-entry-point`
//
// To see the working version, rename `start` back to `main`:
//   fn main() {
//       println!("hello from rustc");
//   }
// Then `rustc 002-fn-main-entry-point.rs` succeeds silently and the
// produced executable prints `hello from rustc`.
fn start() {
    println!("hello from rustc");
}
