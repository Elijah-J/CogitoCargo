// Lesson 147 working probe: a generic function may use the *parenthesized*
// trait-bound form `Fn(T) -> R` for the `Fn`-family traits — the parameter
// and return types of the closure live inside parens, instead of inside
// angle brackets. With the bound `<F: Fn(u32) -> u32>` the function accepts
// any closure literal whose parameter and return shape matches.
//
// Compile silently; run prints one line: `6`.
fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn main() {
    let add_one = |n: u32| n + 1;
    let r = apply(add_one, 5);
    println!("{}", r);
}
