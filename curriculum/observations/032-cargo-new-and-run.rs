// Probe source for EduRatchet-2 lesson 032-cargo-new-and-run.
//
// This file is a verbatim copy of the src/main.rs that `cargo new
// hello_cargo` generates on cargo 1.95.0 — the default "Hello, world!"
// template that ships with every new binary cargo package.
//
// The lesson's actual workflow is a shell session: `cargo new <name>`
// creates a directory <name>/ containing this source at <name>/src/main.rs
// (plus Cargo.toml and a git scaffold), and `cargo run` from inside that
// directory compiles and runs it. See the lesson's `## Evidence` section
// for the full probe transcript.
//
// To reproduce the cargo workflow this lesson teaches:
//   cargo new hello_cargo
//   cd hello_cargo
//   cargo run
// Expected final line of stdout: Hello, world!
//
// To reproduce the source-only check directly with rustc (lesson 001
// workflow), copy this file to hello.rs and run:
//   rustc hello.rs
//   ./hello
// Expected stdout: Hello, world!
fn main() {
    println!("Hello, world!");
}
