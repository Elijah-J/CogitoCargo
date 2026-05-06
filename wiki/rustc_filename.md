<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/rustc/what-is-rustc.html
  - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html
  - https://doc.rust-lang.org/stable/rustc/command-line-arguments.html
  - https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html
topic: rust-playground/rustc-filename
---

# `rustc <filename>`

`rustc <filename>` runs the Rust compiler on a source file. In `hello_world`,
`rustc main.rs` compiled `main.rs` into an executable named `main`.

## Shape I have used

```console
$ rustc main.rs
```

After the compile succeeded, the directory contained both files:

```console
$ ls
main  main.rs
```

`main.rs` is the source. `main` is the compiled binary.

## Running is separate

Compiling did not run the program. The compiled binary ran in a second step:

```console
$ ./main
Hello, world!
```

That split matters: if the source file changes, compile again before expecting
the binary to include the change.

## Useful guardrail

`rustc` is fine for a tiny single-file program. Cargo becomes the normal tool
once the project has a package layout, a `Cargo.toml`, and commands such as
`cargo run` and `cargo check`.

## Corpus references

- [The rustc book: What is rustc?](https://doc.rust-lang.org/stable/rustc/what-is-rustc.html)
- [The Rust Book: Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
- [The rustc book: Command-line Arguments](https://doc.rust-lang.org/stable/rustc/command-line-arguments.html)
- [The Rust Book: Hello, Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)

## Related wiki pages

- [`main.rs`](main_rs.md)
- [Running a compiled binary](run_compiled_binary.md)
- [`cargo new <project_name>`](cargo_new.md)
- [`cargo run`](cargo_run.md)
- [Concepts so far](concepts.md)
