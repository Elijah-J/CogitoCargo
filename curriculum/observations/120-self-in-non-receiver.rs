// observations/120-self-in-non-receiver.rs
//
// Working probe for lesson 120 — `&Self` in the non-receiver parameter
// slot of a trait-impl method. Compose of lesson 113 (`&Counter`
// non-receiver reference parameter) with lesson 100 (`Self` as the
// type-alias for the impl-target type inside an impl).
//
// Centered claim: inside `impl Combine for Counter`, the spelling
// `&Self` in the impl method signature names the same type as
// `&Counter` (the type the trait declaration uses). The trait
// declaration keeps the named type `&Counter`; only the impl method
// signature uses `&Self`. The probe is byte-output-identical to the
// same source with `&Self` substituted by `&Counter` in the impl. The
// contrast probe `outside.rs` places `&Self` in a free function and
// triggers `error[E0411]: cannot find type `Self` in this scope`.
//
// Toolchain: rustc 1.95.0 (59807616e 2026-04-14), x86_64-apple-darwin.
//
// $ rustc 120-self-in-non-receiver.rs
// $ ./120-self-in-non-receiver
// result = 42
// b.count still = 35
//

struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: &Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: &Self) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let result = a.combine(&b);
    println!("result = {}", result);
    println!("b.count still = {}", b.count);
}
