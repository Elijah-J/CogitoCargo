// Probe 1 (working): bind a closure literal to a `let`, call it with parens.
// Closure form: `|x: u32| x + 1` — pipe-bracket-pipe parameter list with one
// fully type-annotated parameter, single-expression body (no braces, no
// trailing semicolon). Return type is inferred from the body. This is Book v4
// from output/docs/rust/book/ch13-01-closures.md:212 with the parameter type
// annotated (the v3/v4 hybrid: parameter annotated, body braceless).
fn main() {
    let add_one = |x: u32| x + 1;
    let a = add_one(5);
    let b = add_one(10);
    println!("{}", a);
    println!("{}", b);
}
