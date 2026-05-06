<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_empty_array_last_index_error/src/main.rs
  - RustPlayground/experiments/hello_empty_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - RustPlayground/experiments/hello_array_for/src/main.rs
  - RustPlayground/experiments/hello_array_sum/src/main.rs
  - RustPlayground/experiments/hello_array_index/src/main.rs
  - RustPlayground/experiments/hello_array_index_error/src/main.rs
  - RustPlayground/experiments/hello_array_len/src/main.rs
  - output/docs/rust/std/primitive.array.md
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/reference/types/array.md
  - output/docs/rust/reference/expressions/array-expr.md
  - output/docs/rust/reference/expressions/loop-expr.md
topic: rust-playground/array
---

# Array

An array stores multiple values in one fixed sequence. In `hello_array_for`,
the array is `numbers`:

```rust
let numbers = [3, 4, 5];
```

## Shape I have used

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

## Array expression

`[3, 4, 5]` is an array expression. The values are written inside square
brackets and separated with commas.

The Rust Reference says this array expression form lists every value in the
array and produces an array containing the values in the order written. That
matches the loop output: `3`, then `4`, then `5`.

## Fixed length and one element type

The Rust Book says every element of an array must have the same type, and that
arrays have a fixed length. In `hello_array_for`, the elements are integer
values and there are three of them.

The Rust Reference writes the full array type shape as `[T; N]`, where `T` is
the element type and `N` is the number of elements. The experiment lets Rust
infer that type instead of writing it.

`hello_empty_array_len` writes the full type because the array expression has
no element values:

```rust
let numbers: [i32; 0] = [];
```

Here, `i32` is the element type and `0` is the array length.

## With `for`

`hello_array_for` loops over the array:

```rust
for number in numbers {
    println!("number: {number}");
}
```

`number` is the loop variable. On each pass, it is bound to one element from
the array.

## With a running total

`hello_array_sum` uses the same array and adds each element into `total`:

```rust
let numbers = [3, 4, 5];
let mut total = 0;

for number in numbers {
    total = total + number;
}
```

The array supplies the elements. The `total` binding stores the accumulated
sum.

## With indexing

`hello_array_index` reads specific elements from the array:

```rust
let numbers = [3, 4, 5];

let first = numbers[0];
let second = numbers[1];
```

The index inside `[]` chooses one position in the array. Index `0` reads the
first element, and index `1` reads the second element.

## Index bounds

`hello_array_index_error` tried to read past the array:

```rust
let missing = numbers[3];
```

For `[3, 4, 5]`, the valid indexes are `0`, `1`, and `2`. The fixed version
uses the last valid index:

```rust
let last = numbers[2];
```

## With `.len()`

`hello_array_len` asks the array how many elements it stores:

```rust
let numbers = [3, 4, 5];

let length = numbers.len();
```

The program prints `length: 3`. The standard library says arrays can use slice
methods, and the slice `len` method returns the number of elements.

`hello_array_last_index` uses the length to compute the last valid index:

```rust
let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

For this non-empty array, the computed index is `2`, so `last` is `5`.

`hello_empty_array_len` uses the same method on an empty array:

```rust
let numbers: [i32; 0] = [];
let length = numbers.len();
```

The program prints `length: 0`.

## Useful guardrail

An array is not the same thing as `Vec`. The current experiments use fixed
arrays whose lengths are part of their types. Growable collections are a later
topic.

## Corpus references

- [The Rust Book: The Array Type](../../output/docs/rust/book/ch03-02-data-types.md)
- [Rust std: array primitive](../../output/docs/rust/std/primitive.array.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust Reference: Array types](../../output/docs/rust/reference/types/array.md)
- [Rust Reference: Array expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [Rust Reference: `for` loops](../../output/docs/rust/reference/expressions/loop-expr.md)

## Related wiki pages

- [`for` loops](for_loops.md)
- [Accumulator](accumulator.md)
- [Array indexing](array_indexing.md)
- [Array index bounds](array_index_bounds.md)
- [Array length](array_len.md)
- [Array last index](array_last_index.md)
- [Array last index with `if`](array_last_index_if.md)
- [Empty array](empty_array.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [`usize`](usize.md)
- [Sequence](sequence.md)
- [Integer literals](integer_literals.md)
- [Types](types.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
