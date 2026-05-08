// Probe for lesson 020-function-with-parameter.
//
// Working version: a second function takes one parameter `n: i32` and is
// called twice from `main` with the arguments `5` and `42`.
// Expected output: two lines, `got n = 5` then `got n = 42`.
//
// The broken contrast (parameter without `: i32`) is documented inside
// the lesson's `## Evidence` section, not as a separate `.rs` file. Only
// this working source is committed.

fn main() {
    say_value(5);
    say_value(42);
}

fn say_value(n: i32) {
    println!("got n = {n}");
}
