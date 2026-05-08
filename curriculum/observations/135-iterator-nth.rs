// Working probe for lesson 135 — `Iterator::nth` on a slice iterator.
//
// Compile and run:
//   $ rustc 135-iterator-nth.rs
//   $ ./135-iterator-nth
//
// Expected output:
//   Some(20)
//   Some(30)
//   Some(40)
//
// Walk: iter starts at [10, 20, 30, 40, 50].
// - `iter.nth(1)` advances past element index 0 (= &10, dropped) and
//   returns element index 1 (= &20). Cursor now points past &20.
// - `iter.nth(0)` returns the 0th element from the new cursor: &30.
// - `iter.nth(0)` returns the 0th element from the new cursor: &40.
//
// Per `output/docs/rust/std/iter/trait.Iterator.md:507`:
//   fn nth(&mut self, n: usize) -> Option<Self::Item>
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];
    let mut iter = v.iter();
    println!("{:?}", iter.nth(1));
    println!("{:?}", iter.nth(0));
    println!("{:?}", iter.nth(0));
}
