// Lesson 115 — declare an associated type `type Output;` in a trait;
// resolve it `type Output = u32;` in the impl.
//
// Working probe. Compile and run with:
//     rustc 115-trait-associated-type.rs -o demo && ./demo
// Expected silent compile, exit 0, then `doubled = 42` on stdout.
//
// Centered E0046 contrast (drop the `type Output = u32;` line from
// the impl block) is captured in the evidence appendix at
// experimental/eduratchet2/runs/rust-moves/evidence/115-trait-associated-type.md
// (Probe 2). Auxiliary E0053 contrast (impl resolves `Output = u64`
// while the impl method writes `-> u32`) is captured in the same
// appendix as Probe 3.

struct Counter {
    count: u32,
}

trait Doubled {
    type Output;
    fn doubled(&self) -> Self::Output;
}

impl Doubled for Counter {
    type Output = u32;
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
