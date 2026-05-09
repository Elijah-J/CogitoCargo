// Lesson 149 working probe: call `for_each` on a `Range<u32>` with two
// closure shapes —
// (1) capture-nothing, just print each (owned u32) element;
// (2) capture-and-mutate `let mut sum`, accumulating the (owned u32) element.
//
// Probe 1 demonstrates `for_each` is a consumer that takes an FnMut closure
// once per element. The second call demonstrates *why* the bound is FnMut
// rather than Fn — it accepts closures that mutate a captured binding.
//
// Source choice: a `Range<u32>` (lesson 091 + 080 + 081) yields owned `u32`
// elements, so the closure parameter `x` is `u32`, not `&u32`. This keeps
// `sum += x` as plain integer `+=` (lesson 023) on `u32 += u32` and
// `x % 2 == 0` / `x == 2` as plain integer arithmetic and comparison
// (lessons 037, 013) — no AddAssign-on-reference, no deref-read.
fn main() {
    (1..4_u32).for_each(|x| println!("{}", x));

    let mut sum: u32 = 0;
    (1..4_u32).for_each(|x| sum += x);
    println!("sum = {}", sum);
}
