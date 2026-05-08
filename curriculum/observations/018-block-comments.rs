// Probe source for EduRatchet-2 lesson 018-block-comments.
//
// This file demonstrates three uses of `/* ... */` in one program:
//   1. A multi-line block comment on its own (above the first println!).
//   2. An inline block comment between code on a single line.
//   3. A trailing block comment after a `;`-terminated statement.
//
// To reproduce the lesson's transcript, copy this file into an empty
// directory and compile it:
//   rustc 018-block-comments.rs
// Then run the produced executable:
//   ./018-block-comments
// Expected output (two lines):
//   hello
//   world
//
// The block comments do not appear in the output because rustc ignores
// everything between `/*` and the matching `*/`. The first comment
// spans two physical lines; the second and third comments sit inline
// or trailing on a single line that also contains code, and only the
// code outside `/* ... */` runs.
//
// Note: the comments inside this header use `//` (line comments from
// lesson 010), not `/* ... */`. The block-comment uses being probed
// are inside `fn main()` below.
fn main() {
    /* this is a block comment;
       it spans multiple lines until the closing marker */
    println!("hello");
    /* inline */ println!("world"); /* trailing */
}
