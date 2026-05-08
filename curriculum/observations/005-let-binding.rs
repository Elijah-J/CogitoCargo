// Probe source for EduRatchet-2 lesson 005-let-binding.
//
// This file is the WORKING version used by the lesson: a `let`
// statement binds the name `x` to the value `5`, and a later
// `println!` uses `{x}` to print the bound value.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 005-let-binding.rs
// Then run the produced executable:
//   ./005-let-binding
// Expected output:
//   x = 5
// Expected exit code: 0.
//
// The lesson's contrast probe is performed by editing this file in
// place to delete the `let x = 5;` line, leaving:
//   fn main() {
//       println!("x = {x}");
//   }
// Recompiling then fails with rustc error E0425:
//   error[E0425]: cannot find value `x` in this scope
//    --> demo.rs:2:20
//     |
//   2 |     println!("x = {x}");
//     |                    ^ not found in this scope
// Exit code: 1. No new executable is produced.
fn main() {
    let x = 5;
    println!("x = {x}");
}
