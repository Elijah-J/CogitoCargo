// Lesson 114 — generic trait parameter.
// Working probe: declares a trait with one type parameter <RHS>,
// implements it for Counter with the concrete substitution <u32>,
// and calls the resulting method through the dot-call shape.
//
// Build / run on rustc 1.95.0 (host: x86_64-apple-darwin):
//     rustc demo.rs
//     ./demo
// Expected output: `total = 42`.
//
// Centered contrast probe in the evidence appendix drops the <u32>
// from the impl header (`impl AddRhs for Counter`) and fires E0107
// `missing generics for trait`. Auxiliary probe passes the wrong
// concrete type at the call site (`c.add(35u64)`) and fires E0308.

struct Counter {
    count: u32,
}

trait AddRhs<RHS> {
    fn add(&self, rhs: RHS) -> u32;
}

impl AddRhs<u32> for Counter {
    fn add(&self, rhs: u32) -> u32 {
        self.count + rhs
    }
}

fn main() {
    let c = Counter { count: 7 };
    let total = c.add(35);
    println!("total = {}", total);
}
