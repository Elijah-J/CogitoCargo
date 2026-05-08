// Probe for lesson 034-as-cast-i32-to-f64.
//
// Working program: bind an `i32` count, cast it to `f64` with `as`, then
// divide by a float literal so we get float division (no truncation).
//
// Compile with: rustc 034-as-cast-i32-to-f64.rs
// Run with:    ./034-as-cast-i32-to-f64
//
// Expected output (rustc 1.95.0, Darwin x86_64):
//     count = 7
//     avg = 3.5
//
// The contrast probe (not committed) replaces `(count as f64) / 2.0`
// with the cast-free form `count / 2.0`. rustc rejects that with
// `error[E0277]: cannot divide \`i32\` by \`{float}\``, headline
// recorded in the lesson's `## Evidence` section.
fn main() {
    let count: i32 = 7;
    let avg: f64 = (count as f64) / 2.0;
    println!("count = {count}");
    println!("avg = {avg}");
}
