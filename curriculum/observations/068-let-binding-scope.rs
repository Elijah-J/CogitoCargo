// Probe for lesson 068-let-binding-scope.
//
// This is the *working* version: `let label` is introduced inside the
// `if` block, and used inside the same `if` block. `rustc demo.rs`
// compiles silently; `./demo` prints `n = 7, label = big`.
//
// To reproduce the contrast (E0425), move the `println!("n = {n},
// label = {label}");` line *out* of the `if` block, after its closing
// `}`. The full transcript of both runs is captured in
// `experimental/eduratchet2/runs/rust-moves/evidence/068-let-binding-scope.md`.

fn main() {
    let n = 7;
    if n > 5 {
        let label = "big";
        println!("n = {n}, label = {label}");
    }
}
