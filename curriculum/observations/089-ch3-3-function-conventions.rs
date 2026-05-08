// Probe source for EduRatchet-2 lesson 089-ch3-3-function-conventions.
//
// This file is the working version used by the lesson. It contains
// two `fn` blocks in one file. Two Ch3-3 conventions are witnessed
// at once:
//
//   1. snake_case naming. `another_function` is the conventional
//      style for function and variable names: all lowercase, with
//      underscores separating words. Source: Book Ch3-3 lines 9-10
//      verbatim, "Rust code uses snake case as the conventional
//      style for function and variable names, in which all letters
//      are lowercase and underscores separate words."
//
//   2. Definition-order is free. `another_function` is defined
//      AFTER `main` in the source, but `main` calls it. The program
//      compiles and runs. Source: Book Ch3-3 lines 33-36 verbatim,
//      "Note that we defined another_function after the main
//      function in the source code; we could have defined it before
//      as well. Rust doesn't care where you define your functions,
//      only that they're defined somewhere in a scope that can be
//      seen by the caller."
//
// The committed source is the verbatim Book example at Ch3-3 lines
// 15-25 (modulo whitespace). The contrast probe (CamelCase function
// name `AnotherFunction` instead of `another_function`) is run as a
// separate transcript captured in the evidence appendix; it is NOT
// committed as a separate `.rs` file.
//
// Load-bearing observations:
//   - `rustc demo.rs` exits 0 (silent on success per lesson 001).
//   - `./demo` prints two lines in the order:
//       Hello, world!
//       Another function.
//     The second line lands AFTER the first, despite
//     `another_function` being DEFINED AFTER `main` in the source.
//
// Source for snake_case convention:
//   output/docs/rust/book/ch03-03-how-functions-work.md, lines 9-10.
// Source for definition-order independence:
//   output/docs/rust/book/ch03-03-how-functions-work.md, lines 33-36.
// Source for the example program:
//   output/docs/rust/book/ch03-03-how-functions-work.md, lines 15-25.
// Source for the `non_snake_case` lint that fires on the contrast:
//   output/docs/rust/rustc/lints/listing/warn-by-default.md, lines 3470-3497.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 089-ch3-3-function-conventions.rs
// Then run the produced executable:
//   ./089-ch3-3-function-conventions
// Expected output:
//   Hello, world!
//   Another function.
// Expected exit code: 0.

fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
