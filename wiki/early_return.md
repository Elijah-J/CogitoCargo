<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_early_return/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/items/functions.html
  - https://doc.rust-lang.org/stable/reference/expressions/return-expr.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
topic: rust-playground/early-return
---

# Early Return

An early return sends a value back to the caller before the function reaches
its final expression. In `hello_early_return`, `describe_count` returns
`"none"` immediately when `count == 0`.

## Shape I have used

```rust
fn describe_count(count: i32) -> &'static str {
    if count == 0 {
        return "none";
    }

    "some"
}
```

The `if` condition checks `count == 0`. When that condition is true, the
`return "none";` line sends `"none"` back to the caller.

## When the early return does not run

When `count == 0` is false, the `if` block is skipped:

```rust
"some"
```

The function then reaches the final expression, so `"some"` is returned.

## Output shape

```rust
let zero_description = describe_count(0);
let three_description = describe_count(3);
```

The two calls produce different returned values:

```console
Zero: none
Three: some
```

## Useful guardrail

An early return is about control flow inside the function. The caller still
just receives a returned value from the function call.

## Corpus references

- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: functions](https://doc.rust-lang.org/stable/reference/items/functions.html)
- [Rust Reference: return expressions](https://doc.rust-lang.org/stable/reference/expressions/return-expr.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)

## Related wiki pages

- [`return`](return_keyword.md)
- [Function return values](function_return_values.md)
- [`if` expressions](if_expressions.md)
- [Conditions](conditions.md)
- [Equality operator](equality_operator.md)
- [String literal return type](string_literal_return_type.md)
- [Concepts so far](concepts.md)
