// Observation for lesson 021-function-return-value.
//
// Working program: define a function that takes one i32 parameter and
// returns an i32, using the explicit `return value;` form. At the call
// site, bind the returned value with `let result: i32 = add_one(5);`.
//
// Expected output:
//     result = 6

fn add_one(n: i32) -> i32 {
    return n + 1;
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
