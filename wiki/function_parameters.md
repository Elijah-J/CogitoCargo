<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_function/src/main.rs
  - experiments/hello_two_parameters/src/main.rs
  - experiments/hello_argument_order/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/items/functions.html
  - https://doc.rust-lang.org/stable/rust-by-example/fn.html
topic: rust-playground/function-parameters
---

# Function Parameters

A function parameter is an input name in a function definition. The caller
passes an argument into that parameter when it calls the function.

## Shape I have used

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

`number` is the parameter name. `i32` is the parameter type.

The call site passes an argument:

```rust
let count = 3;
let next_count = add_one(count);
```

`count` is the argument. Inside `add_one`, that value is available through the
parameter name `number`.

## Parameter type

The Rust Book says function signatures must declare the type of each
parameter. In `hello_function`, the parameter type is written as `number: i32`.

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

This is different from `let count = 3;`, where Rust can infer the binding type
from the integer literal and its use.

## Multiple parameters

The `hello_two_parameters` experiment uses two parameter declarations separated
by a comma:

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

The Rust Book uses the same rule: when defining multiple parameters, separate
the parameter declarations with commas.

Argument order decides which argument is bound to which parameter:

```rust
let remaining = subtract(starting, removed);
let reversed = subtract(removed, starting);
```

## Useful guardrail

Parameter and argument are related but not identical in this model. The
parameter is written in the function definition. The argument is written in the
function call.

## Corpus references

- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: functions](https://doc.rust-lang.org/stable/reference/items/functions.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/stable/rust-by-example/fn.html)

## Related wiki pages

- [Functions](functions.md)
- [Function calls](function_calls.md)
- [Multiple function parameters](multiple_function_parameters.md)
- [Argument order](argument_order.md)
- [Function return values](function_return_values.md)
- [Type annotations](type_annotations.md)
- [`i32`](i32.md)
- [Bindings](bindings.md)
- [Type inference](type_inference.md)
- [Concepts so far](concepts.md)
