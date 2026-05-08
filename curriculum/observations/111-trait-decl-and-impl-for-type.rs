// Observation probe for lesson 111-trait-decl-and-impl-for-type.
//
// Working program: declares one struct, declares one *trait* with a
// single method *signature* (body replaced by a semicolon), implements
// that trait for the struct, and calls the trait method via the dot form.
// Three top-level items new today (`trait`, `impl Trait for Type`, and the
// dot call resolving to a *trait* method) all in one minimal program.
//
// Run as `rustc demo.rs && ./demo`. Expected: silent compile (exit 0);
// `./demo` prints `doubled = 42` and exits 0.
//
// The centered E0599 contrast (same source modulo the entire
// `impl Doubled for Counter { ... }` block removed; trait declaration
// and call site unchanged) is documented in
// `evidence/111-trait-decl-and-impl-for-type.md` and not committed as a
// separate `.rs` file. The auxiliary E0599 probe (calling the trait
// method on a different struct that does not impl the trait) is also
// in the evidence appendix only.

struct Counter {
    count: u32,
}

trait Doubled {
    fn doubled(&self) -> u32;
}

impl Doubled for Counter {
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
