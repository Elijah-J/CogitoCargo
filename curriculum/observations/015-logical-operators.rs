// Probe for lesson 015-logical-operators.
//
// Combine boolean values with `&&`, `||`, and `!`, then bind and print.
// `n = 7` makes `in_range` true, `outside` false, `not_zero` true.
//
// Run inside `mktemp -d`:
//     rustc logical.rs && ./logical
// Expected output (verified 2026-05-06 with rustc 1.95.0 on Darwin x86_64):
//     in_range = true
//     outside = false
//     not_zero = true

fn main() {
    let n = 7;
    let in_range = n > 0 && n < 10;
    let outside = n < 0 || n > 100;
    let not_zero = !(n == 0);
    println!("in_range = {in_range}");
    println!("outside = {outside}");
    println!("not_zero = {not_zero}");
}
