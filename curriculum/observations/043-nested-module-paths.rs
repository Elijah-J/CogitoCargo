// Observation for lesson 043-nested-module-paths.
//
// Captured 2026-05-07 in a fresh `mktemp -d` directory with rustc 1.95.0
// (59807616e 2026-04-14) on Darwin x86_64.
//
// Move tested: calling a free function that lives in a nested module by
// writing the full path `module::submodule::name(args)`. Concretely
// `std::cmp::min(3, 5)` returns `3` and `std::cmp::max(3, 5)` returns
// `5`. Two functions that share the prefix `std::cmp::` and differ only
// in the final segment make the path-as-namespace point empirical.
//
// The broken-contrast probe (free-function form `min(3, 5)` with no
// qualified path; transcript in the evidence appendix, source not
// committed) confirms that `min` is not reachable as a bare free
// function: it fires E0425 "cannot find function `min` in this scope" —
// same E-code lessons 005, 008, 040, and 042 installed. rustc's `help:`
// block here suggests `use std::cmp::min;` (a future move not yet
// installed); the same fix is achievable with the full qualified path
// `std::cmp::min(3, 5)` that this lesson teaches.
//
// Source compiled (verbatim):
//
//     fn main() {
//         let smaller: i32 = std::cmp::min(3, 5);
//         let larger: i32 = std::cmp::max(3, 5);
//         println!("smaller = {smaller}, larger = {larger}");
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
//     smaller = 3, larger = 5
//     exit=0
//
// rustc was silent on success. ./demo printed exactly one line:
// "smaller = 3, larger = 5", confirming that std::cmp::min(3, 5)
// returned 3 and std::cmp::max(3, 5) returned 5. The temp directory was
// removed at the end of the run; only this .rs source is committed
// under observations/. No binary is committed.

fn main() {
    let smaller: i32 = std::cmp::min(3, 5);
    let larger: i32 = std::cmp::max(3, 5);
    println!("smaller = {smaller}, larger = {larger}");
}
