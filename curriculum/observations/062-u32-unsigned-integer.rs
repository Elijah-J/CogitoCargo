// Probe source for EduRatchet-2 lesson 062-u32-unsigned-integer.
//
// Working program: bind two `u32` values, add them, and print both.
// The `: u32` type annotation slot is the same shape as cycle 019's
// `: i32` annotation; only the type name in that slot differs.
//
//   1. `let n: u32 = 42;`     -- explicit annotation, type slot is `u32`.
//   2. `let m: u32 = n + 1;`  -- arithmetic-on-integers (cycle 009)
//                                works on `u32` the same way it works
//                                on `i32`. The right side is a `u32`,
//                                so the annotation matches.
//   3. `println!("n = {n}, m = {m}");`  -- decimal print works the
//                                same way for `u32` as for `i32`.
//
// Sources for the type:
//   output/docs/rust/std/primitive.u32.md, line 8:
//     "The 32-bit unsigned integer type."
//   output/docs/rust/book/ch03-02-data-types.md, line 69:
//     "| 32-bit | `i32` | `u32` |" (Table 3-1, signed-vs-unsigned row).
//   output/docs/rust/book/ch02-00-guessing-game-tutorial.md,
//     lines 937-939: "the `u32` seen here is an unsigned, 32-bit integer."
//
// The contrastive claim "negative literals do not fit in `u32`" is
// captured separately in evidence/062-u32-unsigned-integer.md as a
// broken-contrast probe (`let n: u32 = -1;` fires `error[E0600]:
// cannot apply unary operator -` to type `u32`). Only this working
// source is committed under observations/.
//
// To reproduce:
//   rustc 062-u32-unsigned-integer.rs
//   ./062-u32-unsigned-integer
// Expected output:
//   n = 42, m = 43
// Expected exit code: 0.
fn main() {
    let n: u32 = 42;
    let m: u32 = n + 1;
    println!("n = {n}, m = {m}");
}
