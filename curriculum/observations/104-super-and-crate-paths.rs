// Observation for lesson 104 — super:: and crate:: path prefixes.
//
// Working probe: both prefixes exercised inside one nested module.
//   - `super::at_outer()` walks up one level (from `inner` to `outer`).
//   - `crate::at_root()` jumps to the crate root.
// `rustc demo.rs` compiles silent (exit 0); `./demo` prints
// `super = 2, crate = 1` and exits 0.
//
// Toolchain: rustc 1.95.0 (59807616e 2026-04-14), Darwin x86_64.
//
// Centered contrast (separate file `too_many_supers.rs`, captured in
// the evidence appendix): a single `super::missing()` from inside
// `fn main` at the crate root fires
// `error[E0433]: too many leading \`super\` keywords` — there is no
// parent module above the crate root.

fn at_root() -> u32 { 1 }

mod outer {
    pub fn at_outer() -> u32 { 2 }
    pub mod inner {
        pub fn use_super() -> u32 {
            super::at_outer()
        }
        pub fn use_crate() -> u32 {
            crate::at_root()
        }
    }
}

fn main() {
    println!("super = {}, crate = {}", outer::inner::use_super(), outer::inner::use_crate());
}
