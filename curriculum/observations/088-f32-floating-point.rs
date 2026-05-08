// Probe source for EduRatchet-2 lesson 088-f32-floating-point.
//
// This file is the working version used by the lesson. It contains
// two `let` statements inside `fn main`, each binding a float
// literal but at a different floating-point type:
//
//   1. `let big = 2.0;`        -- no annotation; the unsuffixed
//                                 float literal `2.0` defaults to
//                                 `f64` (the rule lesson 033
//                                 installed). Verbatim from the
//                                 Book example at line 159
//                                 (Filename: src/main.rs).
//   2. `let small: f32 = 3.0;` -- the lesson-019 `: TYPE`
//                                 annotation slot, with `f32` as
//                                 the type. The literal `3.0` is
//                                 a float literal (lesson 033),
//                                 and the annotation pins it at
//                                 `f32`. Verbatim from the Book
//                                 example at line 161.
//
// Two `println!` lines then print each bound value using lesson
// 011's positional `{}` placeholder.
//
// Load-bearing observation: both `let` lines compile, the executable
// exits 0, and the printed output is `big = 2` and `small = 3`. The
// default `{}` formatter prints whole-valued floats without a
// trailing decimal, so the printed string for `2.0` is `2` and for
// `3.0` is `3` (the *type* is still float; only the displayed digits
// happen to look integer-shaped here). The witness this lesson
// centers is that the program *compiles and runs* with one binding
// at `f64` (default-inferred) and one at `f32` (annotated).
//
// Source for `f32` and `f64` as Rust's two floating-point primitives:
//   output/docs/rust/book/ch03-02-data-types.md, lines 145-165,
//   "Floating-Point Types" subsection. Direct quotes:
//     Lines 147-151:
//       "Rust also has two primitive types for floating-point numbers,
//        which are numbers with decimal points. Rust's floating-point
//        types are f32 and f64, which are 32 bits and 64 bits in size,
//        respectively. The default type is f64 because on modern CPUs,
//        it's roughly the same speed as f32 but is capable of more
//        precision. All floating-point types are signed."
//     Line 165:
//       "Floating-point numbers are represented according to the
//        IEEE-754 standard."
//   The two-line `fn main` example at lines 158-162 is the
//   canonical Book pattern this probe follows verbatim.
// Source for the `: TYPE` annotation slot:
//   lesson 019 already installed this.
// Source for `f64` as the floating-point default + float literal
// shape (`5.0`, `3.14`):
//   lesson 033 already installed this.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 088-f32-floating-point.rs
// Then run the produced executable:
//   ./088-f32-floating-point
// Expected output:
//   big = 2
//   small = 3
// Expected exit code: 0.
fn main() {
    let big = 2.0;        // f64 by default
    let small: f32 = 3.0; // f32 via annotation
    println!("big = {}", big);
    println!("small = {}", small);
}
