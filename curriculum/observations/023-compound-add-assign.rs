// Probe source for EduRatchet-2 lesson 023-compound-add-assign.
//
// This is the working program for the lesson. The binding `n` is
// declared with `let mut` (lesson 006), so the three compound-assign
// statements `n += 1;`, `n += 2;`, `n += 3;` are allowed. Each one is
// shorthand for `n = n + value;` where the `+` is the integer-addition
// operator from lesson 009.
//
// Step-by-step values, by lesson 004's source-order rule:
//   after `let mut n = 0;`     -> n is 0
//   after `n += 1;`            -> n is 1
//   after `n += 2;`            -> n is 3
//   after `n += 3;`            -> n is 6
// Then `println!("n = {n}");` prints `n = 6` using the named-placeholder
// form from lesson 005.
//
// Load-bearing observation: the printed value is `n = 6`, which matches
// `0 + 1 + 2 + 3`. This confirms that `n += value;` updates the binding
// in place by the corresponding `+`.
//
// Corpus source for the syntax:
//   output/docs/rust/book/appendix-02-operators.md, Table B-1, line 35:
//   "`+=`  `var += expr`  Arithmetic addition and assignment"
// The same table also lists `-=` (line 39), `*=` (line 30), and
// `/=` (line 50) for the analogous shorthand pattern with `-`, `*`,
// and `/`.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 023-compound-add-assign.rs
// Then run the produced executable:
//   ./023-compound-add-assign
// Expected output:
//   n = 6
// Expected exit code: 0.
fn main() {
    let mut n = 0;
    n += 1;
    n += 2;
    n += 3;
    println!("n = {n}");
}
