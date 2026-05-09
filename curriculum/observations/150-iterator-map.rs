// Lesson 150 working probe: call `Iterator::map` on a `Range<u32>` with a
// closure literal that rewrites each owned `u32` element to a new owned
// `u32` (B = u32, via `x * 10`), then consume the resulting wrapper with
// `Iterator::for_each` (lesson 149) so the program prints `10 / 20 / 30`.
//
// Three structural facts witnessed by this single line:
// (1) `map` is on `Iterator` and takes one closure argument
//     (signature `trait.Iterator.md:852` verbatim).
// (2) The wrapper `Map<Range<u32>, {closure}>` itself implements
//     `Iterator`, so `.for_each(...)` works on it
//     (`struct.Map.md:150-154` verbatim: `impl<B, I, F> Iterator for
//     Map<I, F> ... type Item = B`).
// (3) The closure's return type `u32` becomes the wrapper's `Item`,
//     which is what `.for_each`'s closure receives as `y: u32`.
//
// Source choice: a `Range<u32>` (lesson 091 + 080 + 081) yields owned
// `u32` elements; the closure body `x * 10` is plain integer
// multiplication on `u32 * u32` (lesson 009 `*` on integers). Stays
// inside the parens-rule (lesson 091) and the no-deref / no-AddAssign
// disciplines lesson 149 established.
fn main() {
    (1..4_u32).map(|x| x * 10).for_each(|y| println!("{}", y));
}
