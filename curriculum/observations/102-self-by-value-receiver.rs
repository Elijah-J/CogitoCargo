// Observation probe for lesson 102-self-by-value-receiver.
//
// Working program: declares one struct, attaches one inherent `impl`
// block, and authors one method whose receiver is `self` (no `&`, no
// `mut`) — the third and final receiver shape after lesson 100's
// `&self` (read) and lesson 101's `&mut self` (mutate). Today's `self`
// receiver *consumes* the value: after the dot call, the original
// binding can no longer be used. The body of `into_inner` reads the
// field and returns it, transferring ownership of the field's value
// out of the struct.
//
// Run as `rustc demo.rs && ./demo`. Expected: silent compile (exit 0);
// `./demo` prints `inner = 42` and exits 0.
//
// The centered E0382 contrast (same source plus a second
// `let inner2 = w.into_inner();` after the first call, witnessing
// `error[E0382]: use of moved value: \`w\``) is documented in
// `evidence/102-self-by-value-receiver.md` and not committed as a
// separate `.rs` file. The auxiliary long-form `self: Self` witness is
// also in the evidence appendix only.

struct Wrapper {
    value: u32,
}

impl Wrapper {
    fn into_inner(self) -> u32 {
        self.value
    }
}

fn main() {
    let w = Wrapper { value: 42 };
    let inner = w.into_inner();
    println!("inner = {}", inner);
}
