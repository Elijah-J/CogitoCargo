// Probe 1 — working probe for lesson 133-iterator-count.
//
// Demonstrates: `.count()` consumes the slice iterator and returns the
// number of elements as a `usize`. Vec of length 5 yields `5` on stdout.
//
// Save as demo.rs, run `rustc demo.rs && ./demo`. Compile is silent
// (exit 0); program prints `5` and exits 0.

fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let n = v.iter().count();
    println!("{}", n);
}
