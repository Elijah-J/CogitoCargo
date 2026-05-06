<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - output/docs/rust/std/option/enum.Option.md
  - output/docs/rust/std/iter/trait.Iterator.md
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

- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)
- [Rust std: `Iterator::next`](../../output/docs/rust/std/iter/trait.Iterator.md)

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
