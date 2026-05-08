// Working probe for lesson 138 — `Iterator::enumerate` on a slice iterator.
//
// Compile and run:
//   $ rustc 138-iterator-enumerate.rs
//   $ ./138-iterator-enumerate
//
// Expected output:
//   0 10
//   1 20
//   2 30
//   count = 3
//   Some((2, 30))
//
// `v.iter().enumerate()` returns a NEW iterator value (an `Enumerate<Self>`)
// that yields `(usize, Self::Item)` pairs — the iteration index `i` (starting
// at 0) followed by the element the inner iterator would have yielded.
//
// Per `output/docs/rust/std/iter/trait.Iterator.md:1041`:
//   fn enumerate(self) -> Enumerate<Self> where Self: Sized
//
// And `output/docs/rust/std/iter/struct.Enumerate.md:180`:
//   type Item = (usize, <I as Iterator>::Item)
//
// Same lazy-adapter shape as `take`/`skip` (lessons 136/137): consuming
// `self`, returns a wrapper struct that itself implements `Iterator`.
// Two new structural facts today:
//   (a) NO second parameter — the only argument is the receiver.
//   (b) The yielded element TYPE changes from `Self::Item` to
//       `(usize, Self::Item)`. Today's `for (i, x) in ...` reuses lesson
//       126's tuple pattern in the for-binding slot to split it.
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];

    // (a) `for (i, x) in v.iter().enumerate()` — tuple pattern in the
    //     for-binding slot (lesson 126) destructures the yielded
    //     `(usize, &u64)` pair: `i` is the index, `x` is the element ref.
    for (i, x) in v.iter().enumerate() {
        println!("{} {}", i, x);
    }

    // (b) `.count()` chains onto `.enumerate()` — Enumerate<Self> is itself
    //     an iterator, so any consumer works. The yielded count is just the
    //     source iter's count (enumerate adds the index slot, not elements).
    let n = v.iter().enumerate().count();
    println!("count = {}", n);

    // (c) `.last()` chains onto `.enumerate()`. Returns
    //     `Option<(usize, &u64)>`. Debug format `{:?}` prints
    //     `Some((2, 30))` — the `&` glyph on `&u64` is hidden by Debug for
    //     primitive targets (same convention lessons 131/134/137 captured).
    let last_pair = v.iter().enumerate().last();
    println!("{:?}", last_pair);
}
