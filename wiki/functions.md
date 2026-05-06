<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_function/src/main.rs
  - experiments/hello_two_parameters/src/main.rs
  - experiments/hello_return/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/items/functions.html
  - https://doc.rust-lang.org/stable/rust-by-example/fn.html
topic: rust-playground/functions
---

# Functions

A function is named code with a body. `hello_world` and `hello_cargo` already
used the special `main` function; `hello_function` adds the first helper
function called from `main`.

## Shape I have used

```rust
fn main() {
    let count = 3;
    let next_count = add_one(count);

    println!("Next count: {next_count}");
}

fn add_one(number: i32) -> i32 {
    number + 1
}
```

`fn` starts a function definition. `add_one` is the function name. The
parentheses hold the parameter list, and the braces hold the function body.

## Calling a helper function

`main` calls the helper function by writing the function name followed by
parentheses:

```rust
let next_count = add_one(count);
```

The Rust Book says functions can be defined before or after the code that
calls them, as long as the function is defined in a scope the caller can see.
`hello_function` defines `add_one` after `main`.

## Parameter and return type

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

`number: i32` means the function has one parameter named `number` with type
`i32`. `-> i32` means the function returns an `i32` value. The final expression
`number + 1` is the value returned to the caller.

`hello_two_parameters` uses the same function shape with two parameters:

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

## Useful guardrail

A function name and a binding name are different names for different things.
`add_one` names the function. `next_count` names the returned value after the
function call runs.

## Corpus references

- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: functions](https://doc.rust-lang.org/stable/reference/items/functions.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/stable/rust-by-example/fn.html)

## Related wiki pages

- [Rust `main` function](main_function.md)
- [Function calls](function_calls.md)
- [Function parameters](function_parameters.md)
- [Multiple function parameters](multiple_function_parameters.md)
- [Function return values](function_return_values.md)
- [`return`](return_keyword.md)
- [Type annotations](type_annotations.md)
- [`i32`](i32.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
