// Probe source for EduRatchet-2 lesson 009-arithmetic-on-integers.
//
// This file is the working version used by the lesson. Two `let`
// statements bind the names `a` and `b` to the integer values 5 and
// 3. Four more `let` statements combine `a` and `b` with the binary
// operators `+`, `-`, `*`, and `/`, binding the four results to the
// names `sum`, `diff`, `prod`, and `quot`. Four `println!` calls then
// print each bound result using the named `{name}` placeholder form
// installed by lesson 005.
//
// Load-bearing observation: `quot = 1`, not `1.66...`. The `/`
// operator on integer values truncates toward zero. The Book
// (output/docs/rust/book/ch03-02-data-types.md, lines 169-171):
// "Integer division truncates toward zero to the nearest integer."
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 009-arithmetic-on-integers.rs
// Then run the produced executable:
//   ./009-arithmetic-on-integers
// Expected output:
//   sum  = 8
//   diff = 2
//   prod = 15
//   quot = 1
// Expected exit code: 0.
fn main() {
    let a = 5;
    let b = 3;
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;
    println!("sum  = {sum}");
    println!("diff = {diff}");
    println!("prod = {prod}");
    println!("quot = {quot}");
}
