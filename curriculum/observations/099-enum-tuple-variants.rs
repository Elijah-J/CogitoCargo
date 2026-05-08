// Probe for lesson 099 — declare an enum with mixed unit and tuple
// variants, construct one of each, match each with the corresponding
// pattern shape (bare unit pattern, parenthesized tuple pattern with
// a binding subpattern that captures the payload).
//
// One new rule today (the tuple-variant declaration shape `Variant(T)`).
// Construction reuses lesson 020's call-expression shape; matching
// reuses lesson 058's `Variant(subpattern)` arm shape.
//
// Three composed pieces in one program:
//
//   1. Declaration:    `enum Brightness { Off, On(u32) }`
//                      — `On(u32)` is the new tuple-variant shape;
//                        `Off` is the unit-variant shape from 098.
//   2. Construction:   `Brightness::On(30)` (call expression — Book
//                      ch06-01 lines 169-174 "the name of each enum
//                      variant ... also becomes a function that
//                      constructs an instance of the enum") and the
//                      unit form `Brightness::Off` from 098.
//   3. Match:          `Brightness::On(n) => n` (binding subpattern
//                      `n` captures the payload, exactly the
//                      `Ok(num) => num` shape from lesson 058) plus
//                      `Brightness::Off => 0` (unit pattern from 098).
//
// Compile (silent, exit 0):
//
//     $ rustc demo.rs
//     $ ls
//     demo  demo.rs
//
// Run:
//
//     $ ./demo
//     dim_level = 30, dark_level = 0
//     (exit 0)
//
// Both variants are constructed (`dim` and `dark`) so the `dead_code`
// lint does not fire on either variant.
//
// The contrast probes in evidence/099-enum-tuple-variants.md try to:
//
//   (centered) match a tuple variant with a bare unit-style pattern
//   `Brightness::On => ...` (no parens, no subpattern), which fires
//   `error[E0532]: expected unit struct, unit variant or constant,
//   found tuple variant `Brightness::On`` with a help-line that
//   spells out the fix `Brightness::On(_)` — directly stating the
//   rule today installs;
//
//   (auxiliary) construct the tuple variant without parens, binding
//   the constructor function itself to a name and then trying to use
//   it where a `Brightness` is expected, which fires `error[E0308]:
//   mismatched types` and reveals the constructor's function type
//   `fn(u32) -> Brightness {Brightness::On}`;
//
//   (corroborating) drop the `Brightness::On(_)` arm to leave a
//   non-exhaustive match, which fires `error[E0004]: non-exhaustive
//   patterns: `Brightness::On(_)` not covered` — the missing-pattern
//   label is written in the new tuple-variant shape, corroborating
//   that exhaustiveness applies unchanged from lesson 030 with the
//   shape carrying through to user-declared tuple variants.

enum Brightness {
    Off,
    On(u32),
}

fn main() {
    let dim = Brightness::On(30);
    let dark = Brightness::Off;
    let dim_level = match dim {
        Brightness::Off => 0,
        Brightness::On(n) => n,
    };
    let dark_level = match dark {
        Brightness::Off => 0,
        Brightness::On(n) => n,
    };
    println!("dim_level = {dim_level}, dark_level = {dark_level}");
}
