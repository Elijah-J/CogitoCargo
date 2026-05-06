<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_chars_next/src/main.rs
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
topic: rust-playground/none
---

# `None`

`None` is the `Option` variant for no value. In `hello_chars_next`, it
appears after the `Chars` iterator has no next `char` left.

## Shape I have used

```rust
println!("done: {:?}", chars.next());
```

The program prints:

```console
done: None
```

## No value

The standard library describes `None` as no value. The `Iterator::next` docs
say `next` returns `None` when iteration is finished.

For `word.chars()`, the iterator first returns the four `char` values in
`"café"`. The next call returns `None`.

## Useful guardrail

`None` is not a hidden `char` and it is not the string `"None"`. It is the
`Option` variant that means no value is present.

## Corpus references

- [Rust std: `Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- [Rust std: `Iterator::next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)

## Related wiki pages

- [`Option`](option.md)
- [`match`](match.md)
- [`enum`](enum.md)
- [`Option<T>` syntax](option_t_syntax.md)
- [`Some`](some.md)
- [`Iterator::next`](iterator_next.md)
- [Iterator](iterators.md)
- [Debug formatting](debug_formatting.md)
- [Concepts so far](concepts.md)
