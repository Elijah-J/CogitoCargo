// Working probe for lesson 136 — `Iterator::take` on a slice iterator.
//
// Compile and run:
//   $ rustc 136-iterator-take.rs
//   $ ./136-iterator-take
//
// Expected output:
//   2
//   Some(30)
//   10
//   20
//
// `v.iter().take(n)` returns a NEW iterator value (a `Take<Self>`),
// not a number, not an Option, not a panic. Per
// `output/docs/rust/std/iter/trait.Iterator.md:1376`:
//   fn take(self, n: usize) -> Take<Self> where Self: Sized
//
// So `.count()`, `.last()`, and `for ... in ...` all work on the
// result of `.take(n)` — they are consumers chained onto an adapter.
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    // (a) `.count()` chained on `.take(2)` — counts only the first 2.
    let n = v.iter().take(2).count();
    println!("{}", n);

    // (b) `.last()` chained on `.take(3)` — last of the first 3 = &30.
    let last = v.iter().take(3).last();
    println!("{:?}", last);

    // (c) `for x in v.iter().take(2)` — Take<Self> is itself iterable.
    for x in v.iter().take(2) {
        println!("{}", x);
    }
}
