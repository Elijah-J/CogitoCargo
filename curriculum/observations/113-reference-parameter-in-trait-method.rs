// Observation probe for lesson 113-reference-parameter-in-trait-method.
//
// Working probe: a trait with one method signature taking
// (&self, other: &Counter), an impl block whose method signature
// matches the trait's exactly, two values of the struct, and TWO
// call sites both passing &b — the second call witnesses that b is
// still owned by main after the first call.
//
// Compiled with `/Users/eli/.cargo/bin/rustc demo.rs` on
// rustc 1.95.0 (59807616e 2026-04-14), host x86_64-apple-darwin
// (Darwin 24.5.0). Compile is silent and exits 0; running ./demo
// prints exactly:
//
//     first  = 42
//     second = 42
//     b.count still = 35
//
// and exits 0.
//
// Centered contrast probe (no_amp.rs in the evidence appendix):
// dropping the `&` at the call site (`a.combine(b)` instead of
// `a.combine(&b)`), trait and impl signatures unchanged, fires
// error[E0308]: mismatched types with caret on b at the call site,
// the inline label `expected `&Counter`, found `Counter``, a
// `note: method defined here` block pointing at the trait
// declaration, and a `help: consider borrowing here` block
// proposing the `&` insertion. Full transcript in
// evidence/113-reference-parameter-in-trait-method.md.

struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: &Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: &Counter) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let first = a.combine(&b);
    let second = a.combine(&b);
    println!("first  = {}", first);
    println!("second = {}", second);
    println!("b.count still = {}", b.count);
}
