// Probe source for EduRatchet-2 lesson 090-loop-labels.
//
// This file is the working version used by the lesson. Two `for`
// loops are nested. The OUTER loop carries a label `'outer:`
// (single-quote, snake_case identifier, colon) before its `for`
// keyword. Inside the inner loop, a guarded `break 'outer;`
// targets the OUTER loop instead of the innermost.
//
// Key observation: the outer `for i in 0..3` would normally run
// three times (i = 0, 1, 2). But on the very first inner iteration
// where j == 1, `break 'outer;` exits BOTH loops at once. The outer
// loop body never sees i = 1 or i = 2, and the program proceeds
// directly to the `println!` after the labeled outer loop.
//
// Source for the loop-label syntax and semantics:
//   output/docs/rust/book/ch03-05-control-flow.md, lines 359-409,
//   "Disambiguating with Loop Labels" subsection. Direct quote
//   (lines 361-365): "If you have loops within loops, `break` and
//   `continue` apply to the innermost loop at that point. You can
//   optionally specify a *loop label* on a loop that you can then
//   use with `break` or `continue` to specify that those keywords
//   apply to the labeled loop instead of the innermost loop. Loop
//   labels must begin with a single quote."
//
//   output/docs/rust/reference/expressions/loop-expr.md, lines
//   295-299, "Loop labels" section. Direct quote: "A loop
//   expression may optionally have a *label*. The label is written
//   as a lifetime preceding the loop expression, as in `'foo: loop
//   { break 'foo; }`, `'bar: while false {}`, `'humbug: for _ in
//   0..0 {}`."
//
// Prior lessons:
//   027 (loop + break;) -- bare `break;` exits the innermost loop;
//                          today extends `break;` with the
//                          `break 'name;` form for outer loops.
//   035 (continue;)     -- bare `continue;` skips to the next
//                          iteration of the innermost loop; today's
//                          What Changed bullet 2 mirrors break for
//                          continue with `continue 'name;`.
//   022 (for var in 0..N) -- the loop construct used by both nests.
//
// To reproduce the lesson's working transcript, copy this file into
// an empty directory and compile it:
//   rustc 090-loop-labels.rs
// Then run the produced executable:
//   ./090-loop-labels
// Expected output:
//   i = 0, j = 0
//   i = 0, j = 1: break 'outer
//   after the labeled loops
// Expected exit code: 0.

fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                println!("i = {i}, j = {j}: break 'outer");
                break 'outer;
            }
            println!("i = {i}, j = {j}");
        }
    }
    println!("after the labeled loops");
}
