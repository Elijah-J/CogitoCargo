// Lesson 151 working probe: call `Iterator::fold` on `(1..4_u32)` with an
// initial accumulator `0_u32` and a two-parameter closure that adds the
// element to the accumulator. `fold` returns the final accumulator value
// (a `u32`), which we bind and print.
//
// Source choice: a `Range<u32>` (lesson 091 + 080 + 081) yields owned `u32`
// elements, so the closure parameters are both `u32` (no `&u32`, no
// deref-read). Body: `acc + x` is plain `u32 + u32` per lesson 009.
fn main() {
    let s = (1..4_u32).fold(0_u32, |acc, x| acc + x);
    println!("{}", s);
}
