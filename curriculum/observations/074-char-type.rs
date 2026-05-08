// EduRatchet-2 lesson 074: the `char` type and the single-quoted
// character literal.
//
// Working program. Three `let` statements bind `char` values:
//   1. `let c = 'z';`
//      — bare-name binding (lesson 005), no annotation. The literal
//        `'z'` (single quotes) tells rustc the value is a `char`, so
//        the inferred type is `char`. This is the canonical Book
//        example from `ch03-02-data-types.md` line 230.
//   2. `let letter: char = 'A';`
//      — the lesson-019 `let name: TYPE = value;` slot with `char`
//        plugged into the `TYPE` slot. Same shape as lessons 019
//        (`i32`) and 062 (`u32`); only the type name changes.
//   3. `let math: char = 'ℤ';`
//      — the same shape with a non-ASCII character literal `'ℤ'`,
//        honoring the Book's "represents a Unicode scalar value,
//        which means it can represent a lot more than just ASCII"
//        framing (Book lines 237-241). One non-ASCII example only;
//        emoji and other codepoint surfaces are deferred.
//
// All three values are printed with the lesson-011 positional `{}`
// placeholder. rustc accepts the program; running the executable
// prints three lines.
//
// Compile with `rustc 074-char-type.rs` and run the produced
// executable to print:
//   c = z
//   letter = A
//   math = ℤ
// Exits 0, silent at compile time.
//
// The broken-contrast probe (double-quoted string in a `: char`
// slot, firing `error[E0308]: mismatched types` with `expected
// \`char\`, found \`&str\`` and a `help:` suggesting single quotes)
// is documented in the lesson's evidence appendix as a separate
// run, not committed here. The auxiliary probe (multi-codepoint
// inside single quotes, firing `character literal may only contain
// one codepoint`) is also evidence-only.

fn main() {
    let c = 'z';
    let letter: char = 'A';
    let math: char = 'ℤ';
    println!("c = {}", c);
    println!("letter = {}", letter);
    println!("math = {}", math);
}
