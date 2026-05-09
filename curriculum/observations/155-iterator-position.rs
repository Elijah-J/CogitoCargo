// Canonical probe set for lesson 155 (`Iterator::position`).
// Each `fn main()` block is one probe, kept here as a reference to the
// shape that was actually compiled and run. The transcript at
// `155-iterator-position.transcript.txt` is the authoritative observation;
// this file mirrors its source content for archival readability.

// === Probe 1 — working `position` on `Range<u32>` ===
// fn main() {
//     let r = (10..20_u32).position(|x| x == 15);
//     println!("{:?}", r);
// }
// Output: `Some(5)` — `15` is at index 5 in the sequence `10, 11, ..., 19`.

// === Probe 2 — no match returns `None` ===
// fn main() {
//     let r = (1..10_u32).position(|x| x == 100);
//     println!("{:?}", r);
// }
// Output: `None` — closure never returns true; iterator walked to exhaustion.

// === Probe 3 — first-element match returns `Some(0)` ===
// fn main() {
//     let r = (5..10_u32).position(|x| x == 5);
//     println!("{:?}", r);
// }
// Output: `Some(0)` — first call returns true; index 0.

// === Probe 4 — `&mut self` reusability + index/value disambiguation ===
// fn main() {
//     let mut it = 10..20_u32;
//     let r = it.position(|x| x == 15);
//     let n = it.next();
//     println!("{:?} {:?}", r, n);
// }
// Output: `Some(5) Some(16)` — index 5 (value 15) is the match; iterator
// now points at value 16. Index and value are different numbers.

// === Probe 5 — empty iterator returns `None` ===
// fn main() {
//     let r = (1..1_u32).position(|x| x == 5);
//     println!("{:?}", r);
// }
// Output: `None` — half-open `1..1` yields nothing; closure never called.

// === Probe 6 — closure-call count witnesses zero-based index ===
// fn main() {
//     let mut count = 0_u32;
//     let r = (10..20_u32).position(|x| { count += 1; x == 15 });
//     println!("{:?} {}", r, count);
// }
// Output: `Some(5) 6` — closure called 6 times (`x = 10..=15`); returned
// true on the 6th call; index is 5 (one less than the call count, because
// indexing starts at 0).

// === Probe 7 — type-pin (return type is `Option<usize>`) ===
// fn main() {
//     let r: Option<usize> = (10..20_u32).position(|x| x == 15);
//     println!("{:?}", r);
// }
// Output: `Some(5)`.

// === Probe 8 — non-closure argument fires E0277 ===
// fn main() {
//     let r = (1..4_u32).position(7);
//     println!("{:?}", r);
// }
// Compile-fails with E0277, payload "expected a `FnMut(u32)` closure".

// === Probe 9 — `&mut self` receiver requires `let mut it` ===
// fn main() {
//     let it = 10..20_u32;
//     let _r = it.position(|x| x == 15);
// }
// Compile-fails with E0596 "cannot borrow `it` as mutable".

fn main() {
    let r = (10..20_u32).position(|x| x == 15);
    println!("{:?}", r);
}
