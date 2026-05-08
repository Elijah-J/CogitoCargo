// Observation for lesson 025-implicit-return.
//
// Working program: define a function that takes one i32 parameter and
// returns an i32 using the *implicit* form — no `return` keyword, and
// no trailing `;` on the body's tail expression. The function body is
// itself a block (lesson 024); its value is the value of `n + 1` (no
// `;`); the `-> i32` declaration makes that value the function's
// return value, exactly as in lesson 021's explicit form.
//
// Same call site, same printed output as lesson 021's probe. The only
// delta is the body shape: `n + 1` instead of `return n + 1;`.
//
// The broken-contrast variant (one extra trailing `;` after `n + 1`)
// is captured only in the lesson's `## Evidence` transcript; it is not
// committed under observations/.
//
// Expected output:
//     result = 6

fn add_one(n: i32) -> i32 {
    n + 1
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
