// EduRatchet-2 lesson 103 — restricted visibility: `pub(super)` and `pub(crate)`.
//
// Probe 1 (working program): both modifiers in one inline submodule, called
// from `fn main` (the crate root, parent of `inner`). Compiles silently with
// `rustc demo.rs` on rustc 1.95.0 (Darwin x86_64); `./demo` prints
// `super = 1, crate = 2` and exits 0.
//
// The centered contrast (Probe 2) and auxiliary witnesses (Probes 3, 4) live
// in ../evidence/103-restricted-visibility.md.

mod inner {
    pub(super) fn for_super() -> u32 { 1 }
    pub(crate) fn for_crate() -> u32 { 2 }
}

fn main() {
    println!("super = {}, crate = {}", inner::for_super(), inner::for_crate());
}
