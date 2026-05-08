// Probe for lesson 011-println-positional-args.
//
// Each `{}` in the format string is a positional placeholder consumed
// by one extra argument from the comma-separated list, in left-to-right
// order. The first println! has three placeholders consumed by `a`,
// `b`, `a + b`. The second has two consumed by `a`, `b`.
//
// Captured transcript on rustc 1.95.0 (59807616e 2026-04-14), Darwin
// x86_64, in a temp directory created with `mktemp -d` and removed at
// the end. Expected output:
//
//   5 + 10 = 15
//   first = 5, second = 10
//
// See lesson 011 ## Evidence for the full transcript.

fn main() {
    let a = 5;
    let b = 10;
    println!("{} + {} = {}", a, b, a + b);
    println!("first = {}, second = {}", a, b);
}
