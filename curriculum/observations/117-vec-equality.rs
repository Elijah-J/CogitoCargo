// Lesson 117 — `==` works on two `Vec<T>` values where `T: PartialEq`.
// Std implements `PartialEq` for `Vec<T>`, so the `==` operator from
// lesson 013 (installed there only on integers) extends to `Vec<u64>`.
// The result is a `bool`. Semantics: pairwise — same length AND same
// element values at each index.
//
// Working probe. Compile and run with:
//     rustc 117-vec-equality.rs -o demo && ./demo
// Expected: silent compile, exit 0, three lines on stdout —
//     a == b is true
//     a == c is false
//     a == d is false
//
// `a` and `b` have identical contents (true). `a` and `c` have the
// same length but a different element at index 2 (false). `a` and
// `d` have different lengths entirely (false). The single working
// probe carries both halves of the contrast — equal-content true vs.
// different-content/different-length false — without a separate
// failing probe.

fn main() {
    let a: Vec<u64> = vec![10, 20, 30];
    let b: Vec<u64> = vec![10, 20, 30];
    let c: Vec<u64> = vec![10, 20, 99];
    let d: Vec<u64> = vec![10, 20];
    println!("a == b is {}", a == b);
    println!("a == c is {}", a == c);
    println!("a == d is {}", a == d);
}
