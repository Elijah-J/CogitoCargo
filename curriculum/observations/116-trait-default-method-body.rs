// Lesson 116 — give a trait method a *default body* in curly braces;
// have one impl accept the default with an empty `{}` body, and a
// second impl override the default by providing its own `fn` line.
//
// Working probe. Compile and run with:
//     rustc 116-trait-default-method-body.rs -o demo && ./demo
// Expected: silent compile, exit 0, two lines on stdout —
//     c.count = 7
//     greet   = 100
//
// Centered override contrast (a second struct `Tally` whose impl
// provides its own `fn greet` body that overrides the default) is
// captured in the evidence appendix at
// experimental/eduratchet2/runs/rust-moves/evidence/116-trait-default-method-body.md
// (Probe 2). Optional sharpening contrast (drop the default body's
// `{ 100u32 }` so the trait method ends in `;` again, leaving the
// impl `{}` empty) is captured in the same appendix as Probe 3 and
// fires `error[E0046]: not all trait items implemented, missing:
// `greet``.

struct Counter {
    count: u32,
}

trait Greeting {
    fn greet(&self) -> u32 {
        100u32
    }
}

impl Greeting for Counter {}

fn main() {
    let c = Counter { count: 7 };
    println!("c.count = {}", c.count);
    println!("greet   = {}", c.greet());
}
