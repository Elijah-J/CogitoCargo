// Observation for lesson 050-io-stdin-handle.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: calling the free function `std::io::stdin()` to obtain a
// standard-input handle of type `std::io::Stdin`, and binding it with
// `let`. This is pure composition of lesson 042 (no-receiver call form
// `Type::name()`) and lesson 043 (nested module paths
// `module::submodule::name(args)`): `std::io::stdin` is a three-segment
// path whose leading segments `std` and `std::io` are modules and whose
// final segment `stdin` is a free function in the io submodule.
//
// The binding is named `_stdin` (with a leading underscore) to silence
// the unused-variable warning that rustc would otherwise emit — the
// underscore-prefix convention is the same one lesson 029 glossed for
// the unit-type probe. Without the underscore, rustc prints a
// `warning: unused variable: \`stdin\`` diagnostic with `help: if this
// is intentional, prefix it with an underscore: \`_stdin\``; the
// program still compiles and runs (warnings are not errors), but the
// _stdin form is cleaner.
//
// No broken-contrast probe: the lesson's central claim is *introduction*
// (this function and type exist at this path), not a contrastive
// "with X works, without X fails" claim. See evidence appendix for
// the omission justification.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let _stdin = std::io::stdin();
//         println!("got stdin handle");
//     }
//
// Transcript:
//
//     --- rustc --version ---
//     rustc 1.95.0 (59807616e 2026-04-14)
//     --- uname -sm ---
//     Darwin x86_64
//     --- ls before ---
//     demo.rs
//     --- rustc demo.rs ---
//     exit=0
//     --- ls after ---
//     demo
//     demo.rs
//     --- ./demo ---
//     got stdin handle
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "got stdin handle". The empirical content is that the program
// compiled cleanly (no diagnostics) and ran to completion: the path
// `std::io::stdin` resolved, the function exists, the call returned
// a value, and the value bound to `_stdin`. The temp directory was
// removed at the end of the run; only this .rs source is committed
// under observations/. No binary is committed.

fn main() {
    let _stdin = std::io::stdin();
    println!("got stdin handle");
}
