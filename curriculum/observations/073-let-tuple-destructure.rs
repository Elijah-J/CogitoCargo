// EduRatchet-2 lesson 073: pattern destructuring on the LEFT of `let` —
// a *tuple pattern* in the binding-name slot splits a tuple value into
// one binding per field.
//
// Working program. Three uses of the new shape:
//   1. `let pair = (3, 7); let (a, b) = pair;`
//      — construct first, destructure on a separate line. This is the
//        canonical Reference example from `reference/patterns.md`
//        lines 1058-1066 (with element types adapted to `i32` so we
//        only use types installed by prior lessons).
//   2. `let (x, y, z) = (10, 20, 30);`
//      — construct and destructure in one line. Witnesses that the
//        shape generalizes from arity 2 to arity 3.
//   3. `let (m, n) = (5, 2.5);`
//      — heterogeneous element types (lesson 072). The destructured
//        pattern does not change shape: same parens-and-commas form
//        as the tuple value.
//
// Compile with `rustc 073-let-tuple-destructure.rs` and run the
// produced executable to print four lines:
//   a = 3
//   b = 7
//   x y z = 10 20 30
//   m = 5, n = 2.5
// Exits 0, silent at compile time.
//
// Lesson 072 installed tuple types/values and field access by `.0`,
// `.1`, etc. and explicitly flagged this destructuring move as the
// next cycle (deferred-queue Q06). Today is exactly that move.
//
// The broken-contrast probe (arity mismatch: `let (a, b, c) = (3, 7);`
// against a 2-tuple, firing `error[E0308]: mismatched types` with
// "expected a tuple with 2 elements, found one with 3 elements") is
// documented in the lesson's evidence appendix as a separate run, not
// committed here.

fn main() {
    let pair = (3, 7);
    let (a, b) = pair;
    println!("a = {}", a);
    println!("b = {}", b);

    let (x, y, z) = (10, 20, 30);
    println!("x y z = {} {} {}", x, y, z);

    let (m, n) = (5, 2.5);
    println!("m = {}, n = {}", m, n);
}
