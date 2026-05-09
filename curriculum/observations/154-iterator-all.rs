// Canonical probe set for lesson 154 (`Iterator::all`).
// Each `fn main()` block is one probe, kept here as a reference to the
// shape that was actually compiled and run. The transcript at
// `154-iterator-all.transcript.txt` is the authoritative observation;
// this file mirrors its source content for archival readability.

// === Probe 1 — working `all` on `Range<u32>` ===
// fn main() {
//     let r = (1..10_u32).all(|x| x < 100);
//     println!("{}", r);
// }
// Output: `true`

// === Probe 2 — first-element-fails short-circuit ===
// fn main() {
//     let r = (1..10_u32).all(|x| x < 5);
//     println!("{}", r);
// }
// Output: `false`

// === Probe 3 — empty iterator returns `true` (vacuous truth) ===
// fn main() {
//     let r = (1..1_u32).all(|x| x < 0);
//     println!("{}", r);
// }
// Compiles with `unused_comparisons` warning (rustc notices `x < 0`
// is impossible for `u32`); output is still `true` because the empty
// iterator never calls the closure.

// === Probe 4 — closure-call count (short-circuit witness) ===
// fn main() {
//     let mut count = 0_u32;
//     let r = (1..10_u32).all(|x| { count += 1; x < 5 });
//     println!("{} {}", r, count);
// }
// Output: `false 5` — closure called 5 times (`x = 1, 2, 3, 4, 5`),
// returned `true` for the first four and `false` on the fifth.

// === Probe 5 — `&mut self` reusability witness ===
// fn main() {
//     let mut it = 1..10_u32;
//     let r = it.all(|x| x < 5);
//     let n = it.next();
//     println!("{} {:?}", r, n);
// }
// Output: `false Some(6)` — the iterator is still usable after
// `.all(...)`, and short-circuit on `5` left `6` as the next value.

// === Probe 6 — type-pin (return type is `bool`) ===
// fn main() {
//     let r: bool = (1..10_u32).all(|x| x < 100);
//     println!("{}", r);
// }
// Output: `true`

// === Probe 7 — non-closure argument fires E0277 ===
// fn main() {
//     let r = (1..4_u32).all(7);
//     println!("{}", r);
// }
// Compile-fails with E0277, payload "expected a `FnMut(u32)` closure".

// === Probe 8 — `&mut self` receiver requires `let mut it` ===
// fn main() {
//     let it = 1..10_u32;
//     let _r = it.all(|x| x < 100);
// }
// Compile-fails with E0596 "cannot borrow `it` as mutable".

fn main() {
    let r = (1..10_u32).all(|x| x < 100);
    println!("{}", r);
}
