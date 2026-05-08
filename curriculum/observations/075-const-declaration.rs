// EduRatchet-2 lesson 075: the `const NAME: TYPE = value;` constant
// declaration.
//
// Working program. Two `const` declarations sit at different nesting
// levels:
//   1. `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
//      — declared *outside* any `fn` body, at the *global* scope. The
//        value is the right-side expression `60 * 60 * 3`, which rustc
//        evaluates at compile time to `10800`. This is the canonical
//        Book example from `ch03-01-variables-and-mutability.md` lines
//        128-135.
//   2. `const MAX_POINTS: u32 = 100;`
//      — declared *inside* `fn main`, witnessing the Book's "any
//        scope, including the global scope" claim from line 122.
//
// Both names are SCREAMING_SNAKE_CASE per the Book's naming
// convention (line 140-141: "all uppercase with underscores between
// words"). Both have an explicit `: u32` type annotation per the
// Book's "the type of the value *must* be annotated" rule (line 117).
// Neither uses `mut`; constants are always immutable (line 115).
//
// All values are printed with the lesson-011 positional `{}`
// placeholder. rustc accepts the program; running the executable
// prints two lines.
//
// Compile with `rustc 075-const-declaration.rs` and run the produced
// executable to print:
//   THREE_HOURS_IN_SECONDS = 10800
//   MAX_POINTS = 100
// Exits 0, silent at compile time.
//
// The three contrast probes (missing type annotation firing
// `error: missing type for \`const\` item`; the `const mut` form
// firing `error: const globals cannot be mutable`; and the runtime
// function call firing `error[E0015]: cannot call non-const
// function`) are documented in the lesson's evidence appendix as
// separate runs, not committed here.

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    const MAX_POINTS: u32 = 100;
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);
    println!("MAX_POINTS = {}", MAX_POINTS);
}
