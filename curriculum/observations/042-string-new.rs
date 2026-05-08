// Observation for lesson 042-string-new.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: creating a fresh empty `String` value with the no-receiver
// associated function `String::new()`, binding it to an annotated `let`,
// and printing it with the named-placeholder `{s}` form. Output is
// `empty: []` — the brackets bracket the empty body of the printed
// String.
//
// The broken-contrast probe (free-function form `new()` with no
// qualified path; transcript in the evidence appendix, source not
// committed) confirms that `new` is not reachable as a free function:
// it fires E0425 "cannot find function `new` in this scope" — same
// E-code lessons 005, 008, and 040 installed. Unlike lesson 040's
// broken probe, rustc here does not emit a `help:` block suggesting
// the qualified path; the fix is implied by the missing-name pointer
// alone.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let s: String = String::new();
//         println!("empty: [{s}]");
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
//     empty: []
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "empty: []", confirming that `String::new()` produced a String value
// whose displayed text is empty (zero characters between the `[` and
// the `]`). The temp directory was removed at the end of the run; only
// this .rs source is committed under observations/. No binary is
// committed.

fn main() {
    let s: String = String::new();
    println!("empty: [{s}]");
}
