<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_function/src/main.rs
  - experiments/hello_return/src/main.rs
  - experiments/hello_early_return/src/main.rs
  - experiments/hello_static_str_return/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/items/functions.html
  - https://doc.rust-lang.org/stable/reference/expressions/return-expr.html
  - https://doc.rust-lang.org/stable/rust-by-example/fn.html
topic: rust-playground/function-return-values
---

# Function Return Values

A function return value is the value sent back to the caller. In
`hello_function`, `add_one` returns the result of `number + 1`.

## Shape I have used

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

`-> i32` declares the return type. `number + 1` is the final expression in the
function body, so its value is returned.

## Binding the returned value

```rust
let count = 3;
let next_count = add_one(count);
```

`add_one(count)` returns `4` in `hello_function`. The `let` statement binds that
returned value to `next_count`.

```console
Next count: 4
```

## Semicolon guardrail

The final expression in this function body does not end with a semicolon:

```rust
number + 1
```

The Rust Book says adding a semicolon to the end of an expression turns it into
a statement, and then it does not return a value. That matters when the
function signature says `-> i32`.

## Explicit `return`

The `hello_return` experiment writes the return explicitly:

```rust
fn add_one(number: i32) -> i32 {
    return number + 1;
}
```

The Rust Book says most functions return the last expression implicitly, but
the `return` keyword can also be used with a value. In `hello_return`, the
explicit form returns the same value as the earlier final-expression form.

## Early return

The `hello_early_return` experiment uses both forms in one function:

```rust
fn describe_count(count: i32) -> &'static str {
    if count == 0 {
        return "none";
    }

    "some"
}
```

`return "none";` returns from the function immediately when `count == 0`.
When that branch does not run, `"some"` is the final expression returned by the
function.

## String literal return type

`hello_static_str_return` uses a no-argument function that returns a string
literal:

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

`static_message()` returns the string literal to the caller. The return type is
written as `&'static str` because the returned reference points to a string
literal.

## Corpus references

- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: functions](https://doc.rust-lang.org/stable/reference/items/functions.html)
- [Rust Reference: return expressions](https://doc.rust-lang.org/stable/reference/expressions/return-expr.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/stable/rust-by-example/fn.html)

## Related wiki pages

- [Functions](functions.md)
- [Function calls](function_calls.md)
- [Function parameters](function_parameters.md)
- [Multiple function parameters](multiple_function_parameters.md)
- [`return`](return_keyword.md)
- [Early return](early_return.md)
- [`&`](ampersand.md)
- [String literal return type](string_literal_return_type.md)
- [`'static`](static_lifetime.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [Semicolons](semicolons.md)
- [Statements](statements.md)
- [Addition operator](addition_operator.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [`i32`](i32.md)
- [Concepts so far](concepts.md)
