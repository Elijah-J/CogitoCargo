// Probe for lesson 098 — define an enum with unit variants, construct
// a value of each variant, and match on it.
//
// Three composed pieces in one program:
//
//   1. Declaration:   `enum Sign { Positive, Negative }`
//   2. Construction:  `Sign::Positive`, `Sign::Negative`
//   3. Match:         `match up { Sign::Positive => "+", ... }`
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
//     up = +, down = -
//     (exit 0)
//
// The contrast probes in evidence/098-enum-with-unit-variants.md try
// to construct a non-existent variant `Sign::Maybe` (E0599 "no variant
// or associated item named `Maybe` found for enum `Sign`") and remove
// one match arm to leave a non-exhaustive match (E0004
// "non-exhaustive patterns: `Sign::Negative` not covered"). Both
// witness rules implicit in this probe: variant names are part of the
// enum's declaration, and `match` on an enum must list every variant.
//
// Both variants are constructed (`up` and `down`) so the `dead_code`
// lint does not fire on `Sign::Negative`.

enum Sign {
    Positive,
    Negative,
}

fn main() {
    let up = Sign::Positive;
    let down = Sign::Negative;
    let label_up = match up {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    let label_down = match down {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    println!("up = {label_up}, down = {label_down}");
}
