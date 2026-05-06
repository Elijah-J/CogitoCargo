<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_world/main.rs
  - experiments/hello_variables/src/main.rs
  - experiments/hello_if/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html
  - https://doc.rust-lang.org/stable/reference/macros.html
  - https://doc.rust-lang.org/stable/rust-by-example/macros.html
  - https://doc.rust-lang.org/stable/book/ch20-05-macros.html
  - https://doc.rust-lang.org/stable/std/macro.println.html
  - https://doc.rust-lang.org/stable/std/macro.format.html
topic: rust-playground/macros
---

# Macros

A macro is Rust syntax that is invoked with `!` and expanded at compile time
into Rust code. `hello_world`, `hello_variables`, and `hello_if` use
`println!(...)`; the current model is calling existing macros, not defining new
ones.

## Shape I have used

```rust
fn main() {
    println!("Hello, world!");
}
```

`println!` is the macro name plus `!`. The parentheses contain the input passed
to the macro. In this example, the input is the string literal
`"Hello, world!"`. The semicolon ends the statement.

## Macro call shape

The Rust Reference describes a macro invocation as a path, `!`, and a delimited
token tree. `hello_world`, `hello_variables`, and `hello_if` use parentheses
as the delimiter:

```rust
println!("Hello, world!");
println!("Hello, {name}!");
println!("More apples");
```

The Rust Book uses `println!` to show the visible difference from a normal
function call: a function call would use the name without `!`; a macro call
uses the `!`.

## Expansion

A macro invocation expands during compilation and is replaced with the result
of the macro. Rust by Example describes this as expanding into source code that
is compiled with the rest of the program.

For `hello_world`, `hello_variables`, and `hello_if`, that means `rustc`,
`cargo check`, and `cargo run` all process `println!(...)` as part of
compiling the program. The source file still contains the compact macro call;
the generated code is not something written out by hand in those source files.

## Why `println!` fits here

The standard-library docs define `println!` as a macro that prints to standard
output with a newline. It also uses the same formatting convention as
`format!`, where a string literal can contain `{}` placeholders or named
placeholders such as `{name}`.

```rust
let name = "Eli";
println!("Hello, {name}!");
```

The macro form matters because `println!` can accept different argument shapes.
`hello_world`, `hello_variables`, and `hello_if` have used a plain string
literal, a string literal with a named binding, and string literals inside
`if` blocks.

## Useful guardrail

`macro` is the general Rust idea. `println!` is one specific macro. The wiki
pages so far only need the call shape and compile-time expansion model;
writing a new macro is a later topic.

## Corpus references

- [The Rust Book: Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
- [Rust Reference: Macros](https://doc.rust-lang.org/stable/reference/macros.html)
- [Rust by Example: Macros](https://doc.rust-lang.org/stable/rust-by-example/macros.html)
- [The Rust Book: Macros](https://doc.rust-lang.org/stable/book/ch20-05-macros.html)
- [std::println](https://doc.rust-lang.org/stable/std/macro.println.html)
- [std::format](https://doc.rust-lang.org/stable/std/macro.format.html)

## Related wiki pages

- [Rust `println!` macro](println_macro.md)
- [String literals](string_literals.md)
- [Bindings](bindings.md)
- [`if` expressions](if_expressions.md)
- [Rust `main` function](main_function.md)
- [`rustc <filename>`](rustc_filename.md)
- [`cargo check`](cargo_check.md)
- [`cargo run`](cargo_run.md)
- [Concepts so far](concepts.md)
