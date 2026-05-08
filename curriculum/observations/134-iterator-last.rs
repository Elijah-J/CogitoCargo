// Probe 1 — working probe for lesson 134-iterator-last.
//
// Demonstrates: `.last()` consumes the slice iterator and returns
// `Option<Self::Item>`, which for the slice iter over `Vec<u64>` is
// `Option<&u64>`. A three-element vec yields `Some(30)` on stdout.
//
// Save as demo.rs, run `rustc demo.rs && ./demo`. Compile is silent
// (exit 0); program prints `Some(30)` and exits 0.

fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let result = v.iter().last();
    println!("{:?}", result);
}
