// Canonical probe set for lesson 153 (`Iterator::any`).
// Each `fn main()` block is one probe, kept here as a reference to the
// shape that was actually compiled and run. The transcript at
// `153-iterator-any.transcript.txt` is the authoritative observation;
// this file mirrors its source content for archival readability.

// === Probe 1 — working `any` on `Range<u32>` ===
// fn main() {
//     let r = (1..10_u32).any(|x| x == 5);
//     println!("{}", r);
// }
// Output: `true`

// === Probe 2 — `&mut self` reusability witness ===
// fn main() {
//     let mut it = 1..10_u32;
//     let found = it.any(|x| x == 5);
//     let next = it.next();
//     println!("{} {:?}", found, next);
// }
// Output: `true Some(6)` — the iterator is still usable after `.any(...)`,
// and short-circuit left `6` as the next value.

// === Probe 3 — empty iterator ===
// fn main() {
//     let r = (1..1_u32).any(|x| x == 5);
//     println!("{}", r);
// }
// Output: `false`

// === Probe 4 — no match (closure runs over every element) ===
// fn main() {
//     let r = (1..5_u32).any(|x| x == 100);
//     println!("{}", r);
// }
// Output: `false`

// === Probe 5 — closure-call count (short-circuit witness) ===
// fn main() {
//     let mut count = 0_u32;
//     let r = (1..10_u32).any(|x| { count += 1; x == 3 });
//     println!("{} {}", r, count);
// }
// Output: `true 3` — closure called exactly three times.

// === Probe 6 — non-closure argument fires E0277 ===
// fn main() {
//     let r = (1..4_u32).any(7);
//     println!("{}", r);
// }
// Compile-fails with E0277, payload "expected an `FnMut(u32)` closure".

// === Probe 7 — `&mut self` receiver requires `let mut it` ===
// fn main() {
//     let it = 1..10_u32;
//     let _r = it.any(|x| x == 5);
// }
// Compile-fails with E0596 "cannot borrow `it` as mutable".

// === Probe 8 — type-pin (return type is `bool`) ===
// fn main() {
//     let r: bool = (1..10_u32).any(|x| x == 5);
//     println!("{}", r);
// }
// Output: `true`

fn main() {
    let r = (1..10_u32).any(|x| x == 5);
    println!("{}", r);
}
