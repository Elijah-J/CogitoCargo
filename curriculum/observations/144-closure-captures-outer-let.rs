// Lesson 144 working probe: closure body references an outer `let`.
// The closure `add_n` captures `n` from the enclosing scope of `main`.
// Compile silently; run prints two lines: `15` then `17`.
fn main() {
    let n: u32 = 10;
    let add_n = |x: u32| x + n;
    let a = add_n(5);
    let b = add_n(7);
    println!("{}", a);
    println!("{}", b);
}
