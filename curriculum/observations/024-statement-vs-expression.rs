// Observation for lesson 024-statement-vs-expression.
//
// Working program: bind `x` to the value of a block expression. The
// block's value is the value of its final inner expression `inner + 3`,
// which has no trailing `;`. So `let x: i32 = { ...; inner + 3 };`
// compiles and binds `x` to 5.
//
// The broken-contrast variant (one extra trailing `;` after `inner + 3`)
// is captured only in the lesson's `## Evidence` transcript; it is not
// committed under observations/.
//
// Expected output:
//     x = 5

fn main() {
    let x: i32 = {
        let inner: i32 = 2;
        inner + 3
    };
    println!("x = {x}");
}
