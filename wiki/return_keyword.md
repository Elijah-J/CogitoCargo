<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_return/src/main.rs
  - experiments/hello_early_return/src/main.rs
  - experiments/hello_function/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/items/functions.html
  - https://doc.rust-lang.org/stable/reference/expressions/return-expr.html
  - https://doc.rust-lang.org/stable/rust-by-example/fn.html
topic: rust-playground/return-keyword
---

# `return`

`return` explicitly sends a value back to the caller of the current function.
In `hello_return`, the helper function uses `return` instead of relying on the
final expression of the function body.

## Shape I have used

```rust
fn add_one(number: i32) -> i32 {
    return number + 1;
}
```

`return` is the keyword. `number + 1` is the value sent back to the caller.
The function return type is still written as `-> i32`.

## Compared with final expression return

`hello_function` returned the final expression implicitly:

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

`hello_return` writes the return explicitly:

```rust
fn add_one(number: i32) -> i32 {
    return number + 1;
}
```

Both versions return `4` when called with `3`.

```console
Next count: 4
```

## Semicolon shape

The explicit `return` line ends with a semicolon:

```rust
return number + 1;
```

This is different from leaving `number + 1` as the final expression of the
function body. With the explicit form, `return` is what sends the value to the
caller.

## Early return

The `hello_early_return` experiment puts `return` inside an `if` block:

```rust
fn describe_count(count: i32) -> &'static str {
    if count == 0 {
        return "none";
    }

    "some"
}
```

When `count == 0` is true, `return "none";` sends `"none"` back before the
function reaches `"some"`.

## Corpus references

- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: functions](https://doc.rust-lang.org/stable/reference/items/functions.html)
- [Rust Reference: return expressions](https://doc.rust-lang.org/stable/reference/expressions/return-expr.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/stable/rust-by-example/fn.html)

## Related wiki pages

- [Function return values](function_return_values.md)
- [Functions](functions.md)
- [Function calls](function_calls.md)
- [Early return](early_return.md)
- [String literal return type](string_literal_return_type.md)
- [Semicolons](semicolons.md)
- [Statements](statements.md)
- [`i32`](i32.md)
- [Concepts so far](concepts.md)
