<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/reference/tokens.html
  - https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html
  - https://doc.rust-lang.org/stable/book/ch04-03-slices.html
  - experiments/hello_variables/src/main.rs
  - experiments/hello_str_binding/src/main.rs
  - experiments/hello_early_return/src/main.rs
  - experiments/hello_static_str_return/src/main.rs
  - experiments/hello_utf8_literal/src/main.rs
  - experiments/hello_utf8_len/src/main.rs
topic: rust-playground/string-literals
---

# String Literals

A string literal is text written directly in source code inside double quotes.
`hello_variables`, `hello_mutability`, and `hello_scope` use `"Eli"`, `"Rust"`,
`"outer"`, and `"inner"` as string literals.

## Shape I have used

```rust
let name = "Eli";
println!("Hello, {name}!");
```

The literal text is `"Eli"`. The binding name is `name`. `println!` can print
the value through the binding.

## With an explicit `&str` type

The `hello_str_binding` experiment writes the string literal binding type:

```rust
let name: &str = "Eli";
```

The value is still the string literal `"Eli"`. The source now writes `&str` as
the binding type.

## How it differs from `hello_integer`

```rust
let count = 3;
```

`3` is not text in quotes. It is an integer literal. Both `"Eli"` and `3` are
literals, but they are different kinds of values.

## As return values

The `hello_early_return` experiment returns string literals from a function:

```rust
fn describe_count(count: i32) -> &'static str {
    if count == 0 {
        return "none";
    }

    "some"
}
```

`"none"` and `"some"` are string literals. The function return type is written
as `&'static str`.

`hello_static_str_return` uses the same return type in a smaller function:

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

The Rust Reference says a string literal expression has type `&'static str`.
That is why this return type fits a function that returns a string literal.

## ASCII and non-ASCII text

`hello_utf8_literal` contrasts two string literals:

```rust
let ascii_word: &str = "cafe";
let utf8_word: &str = "café";
```

Both are string literals. `"cafe"` uses only ASCII text. `"café"` includes
`é`, so it is not ASCII text, but it is still valid UTF-8 text.

`hello_utf8_len` keeps the same two string literals and calls
`.len()` on their bindings. That shows a string literal's byte length can
differ from the number of visible letters.

## Useful guardrail

A string literal uses double quotes. The current wiki has not introduced the
owned `String` type yet, so this page only covers the quoted literal form used
in the examples listed above.

## Corpus references

- [Rust Reference: Tokens](https://doc.rust-lang.org/stable/reference/tokens.html)
- [Rust Reference: Literal expressions](https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html)
- [The Rust Book: String slices](https://doc.rust-lang.org/stable/book/ch04-03-slices.html)

## Related wiki pages

- [Literals](literals.md)
- [ASCII](ascii.md)
- [UTF-8](utf_8.md)
- [`.len()` on `&str`](str_len.md)
- [Types](types.md)
- [`&`](ampersand.md)
- [`&str`](str.md)
- [`'static`](static_lifetime.md)
- [String literal return type](string_literal_return_type.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [Integer literals](integer_literals.md)
- [Bindings](bindings.md)
- [`if` expression results](if_expression_results.md)
- [`error[E0308]`](compiler_error_e0308.md)
- [Semicolons](semicolons.md)
- [Rust `println!` macro](println_macro.md)
- [`let`](let_binding.md)
- [Concepts so far](concepts.md)
