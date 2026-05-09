// Lesson 145 working probe: a `fn` declaration may carry a type parameter
// `<T>` after the function name. The same source declaration handles
// multiple concrete types via per-call substitution.
//
// Compile silently; run prints one line: `5 7`.
fn id<T>(t: T) -> T { t }

fn main() {
    let a: u32 = id(5_u32);
    let b: i32 = id(7_i32);
    println!("{} {}", a, b);
}
