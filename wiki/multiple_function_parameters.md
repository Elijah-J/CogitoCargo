<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_two_parameters/src/main.rs
  - experiments/hello_argument_order/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/items/functions.html
  - https://doc.rust-lang.org/stable/rust-by-example/fn.html
topic: rust-playground/multiple-function-parameters
---

# Multiple Function Parameters

Multiple function parameters are written as comma-separated `name: Type`
entries inside a function definition. In `hello_two_parameters`, `add` has two
`i32` parameters.

## Shape I have used

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

`left: i32` is the first parameter. `right: i32` is the second parameter. The
comma separates the two parameter declarations.

The call supplies two arguments:

```rust
let total = add(apples, oranges);
```

`apples` is passed to `left`. `oranges` is passed to `right`.

## Argument order

Function arguments match parameters by position in the call:

```rust
add(apples, oranges)
```

The first argument goes to the first parameter, and the second argument goes to
the second parameter. `hello_two_parameters` uses addition, so swapping the
arguments would still compute the same total; order becomes easier to see with
an operation where left and right produce different results.

The `hello_argument_order` experiment uses subtraction for that contrast:

```rust
let remaining = subtract(starting, removed);
let reversed = subtract(removed, starting);
```

The two calls use the same function and the same two bindings, but in opposite
argument order.

## Useful guardrail

The caller's binding names do not have to match the parameter names. In
`hello_two_parameters`, `main` has `apples` and `oranges`, while the helper
function has `left` and `right`.

## Corpus references

- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: functions](https://doc.rust-lang.org/stable/reference/items/functions.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/stable/rust-by-example/fn.html)

## Related wiki pages

- [Functions](functions.md)
- [Function parameters](function_parameters.md)
- [Function calls](function_calls.md)
- [Function return values](function_return_values.md)
- [Argument order](argument_order.md)
- [Addition operator](addition_operator.md)
- [Subtraction operator](subtraction_operator.md)
- [Operands](operands.md)
- [`i32`](i32.md)
- [Concepts so far](concepts.md)
