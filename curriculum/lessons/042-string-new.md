---
id: 042-string-new
status: accepted
evidence: ../evidence/042-string-new.md
---

# Create a fresh empty `String` with `String::new()`

## The Move

Lesson 041 reached `i32::abs` two ways: the dot-form `n.abs()` and the
qualified form `i32::abs(n)`. Both have a *receiver* — a value being
acted on. Some functions attached to a type have *no receiver at all*.
They take some arguments, or none, and they produce a fresh value of
that type. The qualified path is the only way to reach them: there is
no value to put a dot on.

The canonical example is `new`. Many types in Rust's standard library
have an associated function called `new` that returns a fresh instance
of that type. The simplest is on `String`, the standard heap-allocated
growable text type:

```rust
let s: String = String::new();
println!("empty: [{s}]");
```

`String::new()` produces a fresh, empty `String`. The named-placeholder
`{s}` (lesson 005) prints its content; with brackets around it for
visibility, the program prints `empty: []` — the `[` and `]` straddle
zero characters. The `: String` annotation extends lesson 019's
`let name: TYPE = value;` shape to a new `TYPE`.

## Mental Model Delta

- Before: "The qualified form `Type::method(receiver, args)` reaches
  the same method as the dot-form, just with the receiver written
  inside the parens as the first argument."
- After: "Some functions attached to a type take **no receiver at
  all**. The qualified path `Type::name(args)` is the only way to call
  them — there is no value to put a dot on. The standard convention is
  `Type::new()`, returning a fresh value of that type. `String::new()`
  is the smallest concrete instance: it produces an empty `String`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005: `rustc file.rs` then `./name`, silent on
    success; `fn main` is the entry point; `let name = value;` binds a
    name and `println!("... {name} ...")` prints it.
  - Lesson 003 (load-bearing): rustc diagnostics have headline + `-->`
    + source excerpt + caret. The broken-contrast probe is read with
    that map.
  - Lesson 019 (load-bearing for the *shape*): the type-annotation form
    `let name: TYPE = value;`. Lesson 019 used `i32` as the `TYPE`;
    this lesson extends the annotation surface to a new `TYPE`,
    `String`, with the shape unchanged.
  - Lesson 040 (load-bearing for context): the dot-form
    `value.method(args)` requires a receiver value on the left of the
    dot. Today's contrast is that *no receiver exists* — the dot-form
    simply does not apply.
  - Lesson 041 (load-bearing): the qualified form
    `Type::method(receiver, args)`. This lesson installs the
    *no-receiver* sub-case — `Type::name(args)` with no value-side dot
    form.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn main() {
    let s: String = String::new();
    println!("empty: [{s}]");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
empty: []
```

The right-hand side `String::new()` is the qualified-form shape from
lesson 041 — type, `::`, name, parens — with two differences. (1) The
argument list is empty. (2) There is no receiver: no value sits on
the left of the dot, because there is no dot. The function takes
nothing and produces a fresh, empty `String`. The `: String`
annotation extends lesson 019's `let name: TYPE = value;` shape to a
new `TYPE`. The `[` and `]` around `{s}` make the empty content
visible: zero characters between the brackets.

*Predict*: what if you drop the `String::` qualifier and write `new()`
as if it were a free function? Edit line 2 to
`let s: String = new();` and recompile. rustc emits:

```
error[E0425]: cannot find function `new` in this scope
 --> broken.rs:2:21
  |
2 |     let s: String = new();
  |                     ^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

Same E-code as lessons 005, 008, and 040. The headline word is
`function` (lesson 008's missing-free-function shape) because rustc
parsed `new()` as a free-function call. There is no global free
function called `new` — `new` only exists as a name *attached to a
type*, and the only way to reach it is the qualified path
`String::new()`. Unlike lesson 040, rustc does not emit a `help:`
auto-fix here; the missing-name pointer alone is the signal.

(Full transcripts are in `../evidence/042-string-new.md`.)

## What Changed

- You can call an associated function that takes no receiver with
  `Type::name(args)`. `String::new()` is the smallest example: type,
  `::`, name, empty parens, no value on the left.
- You know one new type, `String`, is the standard heap-allocated
  growable text type. Its empty value is what `String::new()` returns.
- You can annotate a `let` with `: String` — lesson 019's
  `let name: TYPE = value;` shape extended to a new `TYPE`.
- You know the convention: `new` is a common associated-function name
  in the standard library, used for a function that returns a fresh
  value of the type.
- You know the failure mode: writing `new()` instead of `String::new()`
  fires E0425 "cannot find function `new` in this scope" — the same
  E-code as lessons 005, 008, and 040. The fix is the qualified path.

## Check Yourself

You write `pred.rs` containing:

```rust
fn main() {
    let greeting: String = String::new();
    println!("|{greeting}|");
}
```

(a) Does rustc accept the program?

(b) What single line does `./pred` print?

(c) Which two pieces of the right-hand side `String::new()` would you
have to remove to recreate the broken-contrast E0425?

(Answers: (a) Yes — same shape as the lesson, with `greeting` as the
binding name and `|` as visibility brackets. (b) `||` — two `|`
characters with zero characters between them, because `String::new()`
produces an empty `String`. (c) The `String` and the `::`. Removing
both leaves just `new()`, which fires E0425 "cannot find function
`new` in this scope" — the broken-contrast pattern from Try It.)

## What To Ignore For Now

This lesson installs only one idea: the no-receiver associated-function
form `Type::name(args)`, with `String::new()` as the example. Deferred:

- *Mutating a `String`* — `.push(c)`, `.push_str("...")`, `+=`,
  indexing into the text. The only thing this lesson does to `s` is
  print it.
- *Other `String` constructors* — `String::from("hello")` and
  `"hello".to_string()` produce non-empty `String`s. Today's only
  constructor is `String::new()`.
- *String slices `&str`* — string literals like `"hello"` have a type
  *distinct* from `String`. Probes since cycle 1 have used `"..."` in
  `println!` without naming its type, and the lesson does not install
  `&str` either.
- *Ownership and references* — `String` is an owned heap value, with
  move semantics, `Drop`, etc. The bare nouns "owned",
  "heap-allocated", "growable" are used but not unpacked.
- *The `Default` trait and `String::default()`* — another route to an
  empty `String`. Future move alongside traits.
- *`Vec<T>::new()`* — the parallel growable container; same
  no-receiver shape but requires generic-parameter syntax.
- *`String::with_capacity(n)`* — another no-receiver constructor on
  `String`, with a `usize` argument.
- *The `Display` trait* — `{s}` works on a `String` because `String`
  implements `Display`. The trait machinery is deferred.
- All previously deferred items.

## Evidence

See `../evidence/042-string-new.md` for the corpus-quote map, the
rustc / system toolchain string, the working probe transcript, the
broken-contrast E0425 transcript, and the prerequisite-claim summary.
