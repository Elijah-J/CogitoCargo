// Probe source for EduRatchet-2 lesson 019-type-annotation-i32.
//
// This file is the working version used by the lesson. It contains
// three `let` statements inside `fn main`:
//
//   1. `let x: i32 = 5;`     -- explicit type annotation `: i32`.
//   2. `let y = 10;`         -- no annotation; rustc infers `i32`
//                                because integer literals default to
//                                `i32` (Book ch03-02 line 110).
//   3. `let sum: i32 = x + y;` -- annotation on a `let` whose right
//                                  side is an arithmetic expression
//                                  from lesson 009. The annotation
//                                  matches what rustc would have
//                                  inferred; both compile and run
//                                  identically.
//
// One `println!` then prints all three bound values using the named
// `{name}` placeholder form installed by lesson 005.
//
// Load-bearing observation: the annotated and unannotated bindings
// coexist in the same program, both compile under one `rustc` call,
// and the executable prints `x = 5, y = 10, sum = 15`. The annotation
// is `added information` rather than a replacement for the bare-`let`
// form from lesson 005.
//
// Source for `i32` as the integer default:
//   output/docs/rust/book/ch03-02-data-types.md, line 110:
//   "Integer types default to `i32`."
// Source for the `let name: TYPE = value;` annotation form:
//   output/docs/rust/book/ch03-02-data-types.md, lines 8-14.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 019-type-annotation-i32.rs
// Then run the produced executable:
//   ./019-type-annotation-i32
// Expected output:
//   x = 5, y = 10, sum = 15
// Expected exit code: 0.
fn main() {
    let x: i32 = 5;
    let y = 10;
    let sum: i32 = x + y;
    println!("x = {x}, y = {y}, sum = {sum}");
}
