// Probe source for EduRatchet-2 lesson 033-f64-floats.
//
// This file is the working version used by the lesson. It contains
// two `let` statements inside `fn main`, each performing the same
// arithmetic shape (`5 / 3`) on two different numeric types so the
// learner can read the contrast off a single program's output:
//
//   1. `let int_div: i32 = 5 / 3;`     -- integer division on `i32`
//                                          (lesson 009 / lesson 019).
//                                          Truncates toward zero, so
//                                          `5 / 3` is `1`.
//   2. `let float_div: f64 = 5.0 / 3.0;` -- float division on `f64`
//                                          (this lesson). Does not
//                                          truncate; `5.0 / 3.0` is
//                                          `1.6666666666666667`.
//
// Two `println!` lines then print each bound value using the named
// `{name}` placeholder from lesson 005.
//
// Load-bearing observation: same operator `/`, same arithmetic shape
// `5 / 3`, two different types, two different printed results. The
// printed digits `1.6666666666666667` reflect `f64`'s 64-bit binary
// precision and Rust's default float formatting; the exact digits
// depend on IEEE 754 rounding.
//
// Source for `f64` as the floating-point default:
//   output/docs/rust/book/ch03-02-data-types.md, lines 145-163,
//   "Floating-Point Types" subsection. Direct quote (lines 147-151):
//   "Rust also has two primitive types for floating-point numbers,
//    which are numbers with decimal points. Rust's floating-point
//    types are f32 and f64, which are 32 bits and 64 bits in size,
//    respectively. The default type is f64 because on modern CPUs,
//    it's roughly the same speed as f32 but is capable of more
//    precision."
// Source for the `let name: TYPE = value;` annotation form:
//   output/docs/rust/book/ch03-02-data-types.md, lines 8-14
//   (lesson 019 already installed this).
// Source for `5 / 3 = 1` integer division behavior:
//   lesson 009 already installed this.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 033-f64-floats.rs
// Then run the produced executable:
//   ./033-f64-floats
// Expected output:
//   int_div = 1
//   float_div = 1.6666666666666667
// Expected exit code: 0.
fn main() {
    let int_div: i32 = 5 / 3;
    let float_div: f64 = 5.0 / 3.0;
    println!("int_div = {int_div}");
    println!("float_div = {float_div}");
}
