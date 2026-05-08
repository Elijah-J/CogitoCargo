// Working probe for lesson 109 — module-scope const + visibility modifiers.
//
// Build: `rustc 109-module-scope-const.rs -o demo`.
// Run:   `./demo`.
//
// Expected stdout (three lines, exit 0):
//   MAX_BYTES = 1024
//   inner::SHARED = 64
//   inner::LOCAL via fn = 100
//
// Three module-level const items at two scopes and two visibilities:
//
//   `MAX_BYTES`     — at the *crate root*, outside every `fn` and `mod`.
//                     No `pub` marker; not load-bearing for visibility
//                     because the only consumer is `fn main`, which is
//                     also at the crate root (the privacy-by-default
//                     rule passes inside the same module).
//
//   `inner::LOCAL`  — inside `mod inner`, no `pub` marker. Private to
//                     `inner`; `fn main` cannot reach it directly.
//                     The only way out is the `pub fn read_local`
//                     helper that returns it.
//
//   `inner::SHARED` — inside `mod inner`, marked `pub(crate)`. Visible
//                     anywhere in the crate, including `fn main`.
//
// The contrast probe drops `pub(crate)` from `SHARED` and tries to
// read `inner::SHARED` from `fn main` — fires
// `error[E0603]: constant \`SHARED\` is private` (captured in
// `evidence/109-module-scope-const.md`).
const MAX_BYTES: u32 = 1024;

mod inner {
    const LOCAL: u64 = 100;
    pub(crate) const SHARED: u64 = 64;

    pub fn read_local() -> u64 { LOCAL }
}

fn main() {
    println!("MAX_BYTES = {}", MAX_BYTES);
    println!("inner::SHARED = {}", inner::SHARED);
    println!("inner::LOCAL via fn = {}", inner::read_local());
}
