// Lesson 121 — `==` and `!=` work on two `Ordering` values.
// Std implements `PartialEq` for `Ordering`, so the `==` operator
// from lesson 013 (installed there only on integers) and lesson 117
// (extended to `Vec<u64>`) extends again to `Ordering` — the
// standard library's three-variant unit-only enum from lesson 051.
// The result is a `bool`. Same-variant operands compare equal;
// different-variant operands compare unequal.
//
// Working probe. Compile and run with:
//     rustc 121-equality-on-ordering.rs -o demo && ./demo
// Expected: silent compile, exit 0, three lines on stdout —
//     a == b is true
//     a == c is false
//     a != c is true
//
// `a` and `b` are both `Ordering::Less` (true). `a` and `c` are
// `Less` and `Equal` — different variants (false). `a != c` is the
// negation of `a == c` (true). The single working probe witnesses
// both halves of the centered claim — same-variant true vs.
// different-variant false — plus `!=`.

use std::cmp::Ordering;

fn main() {
    let a: Ordering = Ordering::Less;
    let b: Ordering = Ordering::Less;
    let c: Ordering = Ordering::Equal;

    println!("a == b is {}", a == b);
    println!("a == c is {}", a == c);
    println!("a != c is {}", a != c);
}
