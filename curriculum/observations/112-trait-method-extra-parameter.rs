// Observation probe for lesson 112-trait-method-extra-parameter.
//
// Working probe: a trait with one method signature taking
// (&self, factor: u32), an impl block whose method signature matches
// the trait's exactly, and a call site passing one u32 argument.
//
// Compiled with `/Users/eli/.cargo/bin/rustc demo.rs` on
// rustc 1.95.0 (59807616e 2026-04-14), host x86_64-apple-darwin
// (Darwin 24.5.0). Compile is silent and exits 0; running ./demo
// prints exactly:
//
//     scaled = 42
//
// and exits 0.
//
// Centered contrast probe (mismatch.rs in the evidence appendix):
// changing the *trait declaration*'s `factor` type from u32 to u64
// (impl unchanged, so the body's `self.count * factor` still
// typechecks as u32 * u32 with no cast required) fires
// error[E0053]: method `scaled` has an incompatible type for trait,
// caret on the impl's u32 on line 10, plus a `note: type in trait`
// block pointing at the trait declaration's u64 on line 6. The full
// transcript is captured in evidence/112-trait-method-extra-parameter.md.

struct Counter {
    count: u32,
}

trait Scale {
    fn scaled(&self, factor: u32) -> u32;
}

impl Scale for Counter {
    fn scaled(&self, factor: u32) -> u32 {
        self.count * factor
    }
}

fn main() {
    let c = Counter { count: 7 };
    println!("scaled = {}", c.scaled(6));
}
