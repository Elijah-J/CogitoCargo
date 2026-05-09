// Lesson 152 working probe: call `Iterator::reduce` on `(1..4_u32)` with a
// two-parameter closure. Unlike `fold` (lesson 151), `reduce` takes only
// the closure — no `init` argument — and uses the iterator's first element
// as the initial accumulator. The return type is `Option<Self::Item>`
// (here `Option<u32>`) because the iterator could be empty: there would
// then be no first element to start from.
//
// For `(1..4_u32)`: first element `1` becomes the initial accumulator;
// closure walks `(1, 2) -> 3`, `(3, 3) -> 6`. Final value `Some(6)`.
//
// `{:?}` Debug-prints an `Option<u32>` (lesson 093 + 131).
fn main() {
    let s = (1..4_u32).reduce(|acc, x| acc + x);
    println!("{:?}", s);
}
