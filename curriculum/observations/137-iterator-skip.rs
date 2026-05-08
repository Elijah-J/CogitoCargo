// Working probe for lesson 137 — `Iterator::skip` on a slice iterator.
//
// Compile and run:
//   $ rustc 137-iterator-skip.rs
//   $ ./137-iterator-skip
//
// Expected output:
//   3
//   Some(30)
//   40
//   50
//
// `v.iter().skip(n)` returns a NEW iterator value (a `Skip<Self>`) that
// drops the first `n` elements of the inner iterator and yields the rest.
// Per `output/docs/rust/std/iter/trait.Iterator.md:1352`:
//   fn skip(self, n: usize) -> Skip<Self> where Self: Sized
//
// Same shape as `take` (lesson 136): consuming `self`, `n: usize`,
// returns a wrapper struct. Inverse semantic: `take(2)` keeps the first
// two; `skip(2)` discards them.
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    // (a) `.count()` chained on `.skip(2)` — counts only the remaining 3.
    let n = v.iter().skip(2).count();
    println!("{}", n);

    // (b) `.next()` on `.skip(2)` — first element after dropping &10, &20.
    let first_remaining = v.iter().skip(2).next();
    println!("{:?}", first_remaining);

    // (c) `for x in v.iter().skip(3)` — Skip<Self> is itself iterable.
    for x in v.iter().skip(3) {
        println!("{}", x);
    }
}
