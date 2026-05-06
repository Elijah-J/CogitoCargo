<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_match/src/main.rs
  - RustPlayground/experiments/hello_match_option/src/main.rs
  - output/docs/rust/book/ch06-02-match.md
topic: rust-playground/match
---

# `match`

`match` chooses a branch based on which variant a value has. In `hello_match`,
a `Direction` value is matched to choose a string description. In
`hello_match_option`, an `Option<char>` is matched to extract the contained
`char` or handle its absence.

## Shape I have used

```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn describe(turn: Direction) -> &'static str {
    match turn {
        Direction::Left => "going left",
        Direction::Right => "going right",
    }
}

fn main() {
    let first = describe(Direction::Left);
    let second = describe(Direction::Right);

    println!("{first}");
    println!("{second}");
}
```

The program prints:

```console
going left
going right
```

`match turn` looks at the value of `turn`. Each arm names a variant pattern,
followed by `=>`, followed by the value that arm produces.

## Match arms

```rust
Direction::Left => "going left",
Direction::Right => "going right",
```

Each arm is a pattern, `=>`, and an expression. The comma separates arms. The
arm whose pattern matches the value is the one that runs.

## Match as an expression

`match` is an expression, like `if`. In `describe`, the match body is the
final expression of the function, so the chosen arm's value is the return
value. This is the same final-expression return shape used in `hello_function`.

The match could also appear on the right side of a `let` statement, like the
`if` expression in `hello_if_value`.

## Matching on `Option`

In `hello_match_option`, `match` branches on `Option<char>` from an iterator:

```rust
fn describe(item: Option<char>) {
    match item {
        Some(c) => println!("found: {c}"),
        None => println!("nothing left"),
    }
}
```

The program prints:

```console
found: c
found: a
found: f
found: é
nothing left
```

Both arms of `Option<char>` are covered, so the match is exhaustive.

## Extracting data from a variant

In `hello_match`, the arms just named variants:

```rust
Direction::Left => "going left",
```

In `hello_match_option`, the `Some` arm also creates a binding:

```rust
Some(c) => println!("found: {c}"),
```

`c` is a new binding that holds the `char` value from inside `Some`. The name
`c` is chosen in the pattern; it could be any valid binding name. The `None`
arm has no inner value to extract.

## Exhaustiveness

Every variant of the enum must have a matching arm. If one arm were removed,
`cargo check` would reject the program. The Rust Book calls this property
exhaustiveness: the arms must cover every possibility.

## Useful guardrail

`match` on an enum and `if`/`else` both choose a branch. The difference is
that `match` uses the compiler to verify every variant is handled, while
`if`/`else` checks a boolean condition with no variant-level guarantee.

## Corpus references

- [The Rust Book: The `match` Control Flow Construct](../../output/docs/rust/book/ch06-02-match.md)

## Related wiki pages

- [`enum`](enum.md)
- [Variant](variant.md)
- [`Option`](option.md)
- [`Some`](some.md)
- [`None`](none.md)
- [`if` expression results](if_expression_results.md)
- [Functions](functions.md)
- [Function return values](function_return_values.md)
- [Concepts so far](concepts.md)
