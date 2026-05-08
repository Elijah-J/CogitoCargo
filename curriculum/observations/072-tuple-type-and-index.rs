// EduRatchet-2 lesson 072: tuple types with non-zero arity and field
// access by numeric position with `.0`, `.1`, ...
//
// Working program. `pair` binds a 2-tuple of i32s, with an explicit type
// annotation `(i32, i32)` on the let so rustc names the type at compile
// time. `triple` binds a 3-tuple of i32s without an annotation, so rustc
// infers `(i32, i32, i32)`. `mixed` binds a 2-tuple of mixed types
// `(i32, f64)` to witness that tuple element types do not have to match.
// Each tuple's fields are then read with `.0`, `.1`, `.2` and printed.
//
// Compile with `rustc 072-tuple-type-and-index.rs` and run the produced
// executable to print three lines:
//   pair = (3, 7)
//   triple.2 = 30
//   mixed = (5, 2.5)
// Exits 0, silent at compile time.
//
// Lesson 029 named `()` as the 0-arity case of this same family. Today
// extends the family to non-zero arity.
//
// The broken contrast probe (`pair.2` on a 2-tuple, firing
// `error[E0609]: no field `2` on type `(i32, i32)`` with the audience-
// level `note: available fields are: `0`, `1``) is documented in the
// lesson's evidence appendix as a separate run, not committed here.

fn main() {
    let pair: (i32, i32) = (3, 7);
    let triple = (10, 20, 30);
    let mixed: (i32, f64) = (5, 2.5);

    let first = pair.0;
    let second = pair.1;

    println!("pair = ({}, {})", first, second);
    println!("triple.2 = {}", triple.2);
    println!("mixed = ({}, {})", mixed.0, mixed.1);
}
