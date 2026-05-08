// Probe source for EduRatchet-2 lesson 001-rustc-compile-and-run.
// The lesson treats this entire program as an opaque "tiny Rust source
// file"; learners are not asked to understand fn main, println!, the !,
// or the trailing ; in this lesson. Those are explicitly deferred.
//
// Compile and run from this directory:
//   rustc 001-rustc-compile-and-run.rs
//   ./001-rustc-compile-and-run
// Expected stdout: hello from rustc
fn main() {
    println!("hello from rustc");
}
