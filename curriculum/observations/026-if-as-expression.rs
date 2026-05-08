// Observation for lesson 026-if-as-expression.
//
// Working program: bind `category` to the value an `if`/`else`
// produces. The right-hand side of the outer `let` is the whole
// `if n > 5 { 100 } else { -100 }` form. Per lesson 014, the condition
// `n > 5` is evaluated to `true` (because n is 7) and the *then* arm
// runs. Each arm `{ ... }` is a block (lesson 024); each block's value
// is its tail expression's value (no `;`). The whole `if`/`else`
// evaluates to the value of whichever arm ran — here the integer
// literal `100`. The outer `let category: i32` binds `category` to
// that `100`.
//
// The broken-contrast variant (an `else { true }` arm so the two arms
// produce different types) is captured only in the lesson's
// `## Evidence` transcript; it is not committed under observations/.
//
// Expected output:
//     category = 100

fn main() {
    let n: i32 = 7;
    let category: i32 = if n > 5 { 100 } else { -100 };
    println!("category = {category}");
}
