// Probe source for EduRatchet-2 lesson 010-line-comments.
//
// This file demonstrates three uses of `//` in one program:
//   1. A free-standing comment on its own line.
//   2. A trailing comment after a `;`-terminated statement.
//   3. A whole `;`-terminated statement that has been commented out
//      (prefixed with `//`) so it does not run.
//
// To reproduce the lesson's transcript, copy this file into an empty
// directory and compile it:
//   rustc 010-line-comments.rs
// Then run the produced executable:
//   ./010-line-comments
// Expected output (two lines, not three):
//   first
//   second
//
// The third `println!` does not appear in the output because the line
// has been commented out: `//` makes rustc ignore everything from `//`
// to the end of that line, so the entire `println!("third");` is gone
// from the compiled program.
fn main() {
    println!("first");
    // this line is a comment; rustc ignores it
    println!("second"); // trailing comment
    // println!("third");
}
