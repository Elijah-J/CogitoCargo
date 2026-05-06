<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_scope_error/src/main.rs
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html
  - https://doc.rust-lang.org/stable/reference/names/scopes.html
topic: rust-playground/compiler-error-e0425
---

# `error[E0425]`

``error[E0425]: cannot find value `name` in this scope`` is the compiler error
`hello_scope_error` produced when code tried to use a binding after the block
where that binding existed had ended.

## Shape that triggered it

```rust
{
    let name = "inner";
    println!("Inside block: {name}");
}

println!("After block: {name}");
```

The binding named `name` exists inside the inner block. The final `println!`
is outside that block, so `cargo check` could not find a `name` binding there.

`cargo check` also pointed back to the inner `let name = "inner";` line and
reported that the binding was available in a different scope in the same
function.

## Fix used in `hello_scope_error`

```rust
let name = "outer";

{
    let name = "inner";
    println!("Inside block: {name}");
}

println!("After block: {name}");
```

The fixed version adds an outer binding. Inside the block, the inner binding
shadows the outer binding. After the block ends, the inner binding is gone, so
the final `println!` uses the outer binding.

## Program output after the fix

```console
Inside block: inner
After block: outer
```

## Useful guardrail

Rust by Example describes a block as statements enclosed by braces. A binding
created inside a block is constrained to that block, so code after the closing
brace needs some other binding in scope.

## Corpus references

- [Rust by Example: Scope and Shadowing](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html)
- [Rust Reference: Scopes](https://doc.rust-lang.org/stable/reference/names/scopes.html)

## Related wiki pages

- [Block scope](block_scope.md)
- [Bindings](bindings.md)
- [Shadowing](shadowing.md)
- [`cargo check`](cargo_check.md)
- [Concepts so far](concepts.md)
