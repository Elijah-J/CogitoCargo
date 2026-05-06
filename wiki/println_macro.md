<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/std/macro.println.html
  - https://doc.rust-lang.org/stable/std/macro.format.html
  - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html
  - https://doc.rust-lang.org/stable/rust-by-example/hello.html
topic: rust-playground/println-macro
---

# Rust `println!` Macro

`println!("some string");` prints text to standard output and adds a newline.
It is the visible output statement in the first Rust programs.

## Shape I have used

```rust
fn main() {
    println!("Hello, world!");
}
```

The `!` is part of the call: `println!` is a macro, not a normal function. The
quoted text is the string being printed. The semicolon ends the statement.

## What appears in the terminal

When the compiled program runs, the printed text appears in the terminal:

```console
Hello, world!
```

The standard-library docs define `println!` as printing to standard output with
a newline. That is why the shell prompt appears on the next line after the
program finishes.

## Using a variable in the output

The `hello_variables` experiment used a variable inside the printed string:

```rust
let name = "Eli";
println!("Hello, {name}!");
```

The formatting docs show this `{name}` form for values already in scope. In
`hello_variables`, the output is:

```console
Hello, Eli!
```

## Useful guardrail

`println!` belongs inside code that actually runs. In the hello-world program,
that means it is inside `fn main()`.

## Corpus references

- [std::println](https://doc.rust-lang.org/stable/std/macro.println.html)
- [std::format](https://doc.rust-lang.org/stable/std/macro.format.html)
- [The Rust Book: Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
- [Rust by Example: Hello World](https://doc.rust-lang.org/stable/rust-by-example/hello.html)

## Related wiki pages

- [Macros](macros.md)
- [Debug formatting](debug_formatting.md)
- [Bindings](bindings.md)
- [Integer literals](integer_literals.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [`if` expressions](if_expressions.md)
- [Statements](statements.md)
- [Semicolons](semicolons.md)
- [String literals](string_literals.md)
- [`let`](let_binding.md)
- [Shadowing](shadowing.md)
- [`let mut`](mutable_binding.md)
- [`//` comments](line_comments.md)
- [Rust `main` function](main_function.md)
- [`main.rs`](main_rs.md)
- [`src/main.rs`](src_main_rs.md)
- [Running a compiled binary](run_compiled_binary.md)
- [Concepts so far](concepts.md)
