<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_function/src/main.rs
  - RustPlayground/experiments/hello_two_parameters/src/main.rs
  - RustPlayground/experiments/hello_argument_order/src/main.rs
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/reference/items/functions.md
topic: rust-playground/function-calls
---

# Function Calls

A function call runs a function by writing its name followed by parentheses.
In `hello_function`, `main` calls `add_one`.

## Shape I have used

```rust
let count = 3;
let next_count = add_one(count);
```

`add_one(count)` is the function call. `count` is the argument passed into the
function.

## Calls are expressions

The Rust Book says calling a function is an expression. In `hello_function`,
the call expression returns an `i32` value:

```rust
let next_count = add_one(count);
```

The `let` statement binds that returned value to `next_count`.

## Multiple arguments

The `hello_two_parameters` experiment calls a function with two arguments:

```rust
let total = add(apples, oranges);
```

`apples` is the first argument. `oranges` is the second argument.

The `hello_argument_order` experiment uses two calls with reversed arguments:

```rust
let remaining = subtract(starting, removed);
let reversed = subtract(removed, starting);
```

Both are function calls. The argument order changes which value the function
receives as `left` and which value it receives as `right`.

## Function call and macro invocation

```rust
let next_count = add_one(count);
println!("Next count: {next_count}");
```

`add_one(count)` is a function call. `println!(...)` is a macro invocation
because it uses `!` after the name.

## Corpus references

- [The Rust Book: Functions](../../output/docs/rust/book/ch03-03-how-functions-work.md)
- [Rust Reference: functions](../../output/docs/rust/reference/items/functions.md)

## Related wiki pages

- [Functions](functions.md)
- [Function parameters](function_parameters.md)
- [Multiple function parameters](multiple_function_parameters.md)
- [Argument order](argument_order.md)
- [Function return values](function_return_values.md)
- [`str::len` syntax](str_len_syntax.md)
- [Rust `println!` macro](println_macro.md)
- [Macros](macros.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
