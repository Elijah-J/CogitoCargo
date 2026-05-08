// Observation probe for lesson 101-mut-self-receiver.
//
// Working program: declares one struct, attaches one inherent `impl` block,
// and authors three associated items reused from lesson 100 plus today's
// new piece — `fn bump(&mut self)`, the third receiver shape after lesson
// 100's no-receiver associated function and `&self` method. The body of
// `bump` writes to `self.count`, which is licensed by `&mut self` (the
// receiver-shorthand for `self: &mut Self`, per Reference
// items/associated-items.md line 159).
//
// Run as `rustc demo.rs && ./demo`. Expected: silent compile (exit 0);
// `./demo` prints `count = 2` and exits 0. Two `c.bump()` calls increment
// `count` from 0 to 2; the final `c.current()` reads it back.
//
// The centered E0596 contrast (same source modulo `let mut c = ...`
// changed to `let c = ...`, leaving everything else unchanged) is
// documented in `evidence/101-mut-self-receiver.md` and not committed
// as a separate `.rs` file. The secondary E0594 contrast (assigning to
// `self.count` inside a `&self` method body) is also in the evidence
// appendix only.

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
    fn bump(&mut self) {
        self.count = self.count + 1;
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    c.bump();
    println!("count = {}", c.current());
}
