// Probe for lesson 069-rustc-warnings.
//
// `rustc demo.rs` on this file emits a `warning:` diagnostic about the
// unused binding `x`, but still exits 0 and writes the executable
// `demo` next to the source. Running `./demo` succeeds silently (the
// body has no output).
//
// This is the load-bearing observation for lesson 069's category
// claim: warnings ≠ errors. With an `error:` diagnostic (lessons 002,
// 003, 005, 068), `rustc` exits 1 and produces no executable; with a
// `warning:` diagnostic, `rustc` exits 0 and an executable IS
// produced.
//
// The full transcript is captured in
// `experimental/eduratchet2/runs/rust-moves/evidence/069-rustc-warnings.md`.

fn main() {
    let x = 5;
}
