// Probe source for EduRatchet-2 lesson 006-mut-binding.
//
// This file is the WORKING version used by the lesson: a `let mut`
// statement binds the name `x` to `5`, and a later `x = 6;` statement
// reassigns it. Two `println!` statements observe the binding before
// and after the reassignment.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 006-mut-binding.rs
// Then run the produced executable:
//   ./006-mut-binding
// Expected output:
//   x = 5
//   x = 6
// Expected exit code: 0.
//
// The lesson's contrast probe is performed by editing this file in
// place to remove the keyword `mut` from line 2, leaving:
//   fn main() {
//       let x = 5;
//       println!("x = {x}");
//       x = 6;
//       println!("x = {x}");
//   }
// Recompiling then fails with rustc error E0384:
//   error[E0384]: cannot assign twice to immutable variable `x`
//    --> demo.rs:4:5
//     |
//   2 |     let x = 5;
//     |         - first assignment to `x`
//   3 |     println!("x = {x}");
//   4 |     x = 6;
//     |     ^^^^^ cannot assign twice to immutable variable
//     |
//   help: consider making this binding mutable
//     |
//   2 |     let mut x = 5;
//     |         +++
// Exit code: 1. No new executable is produced.
fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}
