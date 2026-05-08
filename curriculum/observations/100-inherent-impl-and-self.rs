// Observation probe for lesson 100-inherent-impl-and-self.
//
// Working program: declares one struct, attaches one inherent `impl` block,
// authors one *associated function* `new() -> Self` with no receiver, and
// one *method* `current(&self) -> u32` with the `&self` receiver. `fn main`
// calls the associated function via `Counter::new()` and the method via
// `c.current()`.
//
// Run as `rustc demo.rs && ./demo`. Expected: silent compile (exit 0);
// `./demo` prints `count = 0` and exits 0.
//
// The centered E0599 contrast (same source modulo `&self` removed from
// `current`'s signature, calling site unchanged) is documented in
// `evidence/100-inherent-impl-and-self.md` and not committed as a
// separate `.rs` file. The auxiliary E0061 contrast (calling the
// `&self` method via the path form `Counter::current()`) is also in
// the evidence appendix only.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
    fn current(&self) -> u32 {
        self.count
    }
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
