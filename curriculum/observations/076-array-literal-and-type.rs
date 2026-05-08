// EduRatchet-2 lesson 076: arrays — the array literal `[v1, v2, ...]`,
// the array type `[T; N]`, and the repeat-init form `[v; N]`.
//
// Working program. Three `let` statements bind arrays:
//   1. `let nums = [1, 2, 3, 4, 5];`
//      — bare-name binding (lesson 005), no annotation. The
//        square-bracket comma-separated literal is what tells
//        rustc the value is an array; the inferred type is
//        `[i32; 5]` (five i32s — the integer-literal default
//        from lesson 019, fixed length five). This is the
//        Book's canonical example from `ch03-02-data-types.md`
//        line 332.
//   2. `let typed: [i32; 5] = [10, 20, 30, 40, 50];`
//      — the lesson-019 `let name: TYPE = value;` slot with
//        `[i32; 5]` plugged into the `TYPE` slot. The type
//        syntax is the Book's canonical line-365 form: square
//        brackets, element type, semicolon, length. Same `: TYPE`
//        annotation shape as lessons 019 (i32), 033 (f64),
//        062 (u32), 074 (char); only the type expression in the
//        slot changes — this time it is itself a parameterized
//        type expression.
//   3. `let zeros = [0; 4];`
//      — the *repeat-init* form. Square brackets, element value,
//        semicolon, length. Equivalent to writing
//        `[0, 0, 0, 0]` but in the more concise way the Book
//        names at line 384 ("the same as writing `let a = [3, 3,
//        3, 3, 3];` but in a more concise way").
//
// Each of the three arrays then has its length printed via the
// `.len()` method (lesson-040 method-call shape). `.len()` is
// documented on the std primitive-slice page (`std/primitive.
// slice.md` lines 794-805), reachable on arrays via the
// "Arrays coerce to slices ([T])" rule from `std/primitive.
// array.md` line 41-42. The std page literally shows
// `let a = [1, 2, 3]; assert_eq!(a.len(), 3);` as the canonical
// example, calling `.len()` on an array literal exactly as the
// probe does.
//
// Compile with `rustc 076-array-literal-and-type.rs` and run
// the produced executable to print:
//   nums.len() = 5
//   typed.len() = 5
//   zeros.len() = 4
// Exits 0, silent at compile time.
//
// The broken-contrast probe (mixed-type literal `[1, 2.5]`,
// firing `error[E0308]: mismatched types` with `expected
// integer, found floating-point number`) is documented in the
// lesson's evidence appendix as a separate run, not committed
// here. The auxiliary arity-mismatch probe (`[i32; 3] = [1, 2]`,
// firing `error[E0308]: mismatched types` with `expected an
// array with a size of 3, found one with a size of 2`) is also
// evidence-only.

fn main() {
    let nums = [1, 2, 3, 4, 5];
    let typed: [i32; 5] = [10, 20, 30, 40, 50];
    let zeros = [0; 4];
    println!("nums.len() = {}", nums.len());
    println!("typed.len() = {}", typed.len());
    println!("zeros.len() = {}", zeros.len());
}
