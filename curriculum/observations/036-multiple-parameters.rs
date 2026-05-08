// Lesson 036 working probe: a function with two parameters.
//
// Build:
//   rustc 036-multiple-parameters.rs -o demo
//   ./demo
// Expected output:
//   result = 5
//
// `add` has two `i32` parameters separated by `,`. The call site
// `add(2, 3)` supplies two arguments separated by `,`, matched
// positionally (`2` -> `a`, `3` -> `b`). Lesson 025's implicit
// return makes the body's tail expression `a + b` the return value.
//
// Broken contrast (NOT committed): replace the call site with
// `add(2)` (one argument). rustc emits
// `error[E0061]: this function takes 2 arguments but 1 argument was supplied`
// with a primary `-->` at the call site, a `note: function defined here`
// secondary `-->` at the signature, and a `help: provide the argument`
// suggestion with a `/* i32 */` placeholder.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result: i32 = add(2, 3);
    println!("result = {result}");
}
