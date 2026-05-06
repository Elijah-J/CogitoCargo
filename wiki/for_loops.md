<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_for/src/main.rs
  - experiments/hello_array_for/src/main.rs
  - experiments/hello_array_sum/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
  - https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html
topic: rust-playground/for-loops
---

# `for` Loops

A `for` loop runs a block once for each item in an iterator. In `hello_for`,
`for c in word.chars()` walks through each `char` in `"café"`. In
`hello_array_for`, `for number in numbers` walks through each element in an
array. In `hello_array_sum`, the loop also updates a running total.

## Shape I have used

```rust
fn main() {
    let word: &str = "café";

    for c in word.chars() {
        println!("char: {c}");
    }

    println!("done");
}
```

The program prints:

```console
char: c
char: a
char: f
char: é
done
```

The loop body runs four times — once for each `char` in the iterator. After the
last item, the loop ends and execution continues with `println!("done")`.

## With an array

`hello_array_for` uses the same `for` shape with an array:

```rust
fn main() {
    let numbers = [3, 4, 5];

    for number in numbers {
        println!("number: {number}");
    }

    println!("done");
}
```

The program prints:

```console
number: 3
number: 4
number: 5
done
```

The loop body runs once for each array element. After `5`, the array has no
more elements for the loop, so execution continues after the loop.

## With a running total

`hello_array_sum` adds a mutable binding before the loop:

```rust
let numbers = [3, 4, 5];
let mut total = 0;

for number in numbers {
    total = total + number;
    println!("running total: {total}");
}

println!("final total: {total}");
```

The loop body still runs once per array element. The new part is that `total`
is outside the loop, so its updated value is available on the next pass and
after the loop finishes.

## The loop variable

`c` in `for c in word.chars()` and `number` in `for number in numbers` are
bindings created by the loop. On each pass through the body, the loop variable
is bound to the current item. The name is chosen in the loop header; it could
be any valid binding name.

## Connection to `next`

In `hello_chars_next` and `hello_match_option`, the same iterator was driven
by calling `.next()` manually. Each call returned `Some(char)` or `None`.

A `for` loop does this internally: it calls `.next()`, runs the body with the
item from `Some`, and stops when `.next()` returns `None`. The loop variable
receives the item directly — `c` is a `char`, not an `Option<char>`.

## Useful guardrail

`for c in word.chars()` gives `c` the type `char`, not `Option<char>`. The
`for` loop handles the `Some`/`None` matching internally. If you need to
handle `Some` and `None` yourself, call `.next()` with `match` as in
`hello_match_option`.

`for number in numbers` gives `number` one array element at a time. It does
not give the loop body the whole array on each pass.

The loop variable changes on each pass, but it is not the same binding as a
running total. A running total needs its own mutable binding outside the loop.

## Corpus references

- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [The Rust Book: iterators](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)
- [Rust Reference: `for` loops](https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html)

## Related wiki pages

- [Iterator](iterators.md)
- [`Iterator::next`](iterator_next.md)
- [`str::chars`](str_chars.md)
- [Array](array.md)
- [Accumulator](accumulator.md)
- [`match`](match.md)
- [`Option`](option.md)
- [Block scope](block_scope.md)
- [Concepts so far](concepts.md)
