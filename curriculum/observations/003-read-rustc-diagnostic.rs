// Probe source for EduRatchet-2 lesson 003-read-rustc-diagnostic.
//
// This file is BROKEN on purpose: the macro name is misspelled
// `prntln` instead of `println`. Compiling it should fail with a
// rustc diagnostic of the form:
//
//   error: cannot find macro `prntln` in this scope
//    --> prntln.rs:2:5
//     |
//   2 |     prntln!("hello from rustc");
//     |     ^^^^^^
//     |
//    --> /rustc/<hash>/library/std/src/macros.rs:138:0
//     |
//     = note: similarly named macro `println` defined here
//   help: a macro with a similar name exists
//     |
//   2 |     println!("hello from rustc");
//     |       +
//
//   error: aborting due to 1 previous error
//
// The lesson uses this transcript to point at the standard parts of
// a rustc diagnostic: headline, --> location, source excerpt with
// caret, and help: / = note: lines.
//
// To reproduce, copy this file into an empty directory and compile:
//   rustc 003-read-rustc-diagnostic.rs
// Expected: rustc exits 1 and prints the diagnostic above.
//
// To see the working version, fix the typo:
//   fn main() {
//       println!("hello from rustc");
//   }
fn main() {
    prntln!("hello from rustc");
}
