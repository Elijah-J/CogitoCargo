---
id: 055-string-trim
status: accepted
evidence: ../evidence/055-string-trim.md
---

# Strip whitespace from a `String` with `.trim()`, get a `&str` back

## The Move

Lesson 054 left `buf` holding the user's text *plus* a trailing `\n` —
the `read_line` semantics are "append the input, newline included." Most
of the time that newline is in the way. `.trim()` is the standard fix:

```rust
let trimmed: &str = buf.trim();
```

`.trim()` is a method that returns a value with leading and trailing
whitespace removed. It works on a `String` value (lesson 042) using the
ordinary dot-form (lesson 040). The annotation `: &str` is the new
typed name this lesson installs: a *string slice*, the borrowed view
into a sequence of UTF-8 bytes.

## Mental Model Delta

- *Before:* "I have a `String` with my text plus a trailing `\n` from
  `read_line`. `String` is the only string type I can name."
- *After:* "Calling `.trim()` on a `String` returns a `&str` containing
  the same text without leading and trailing whitespace. `&str` is a
  second string type — a *string slice*, a borrowed view into UTF-8
  bytes. String literals like `\"hello\"` are also `&str` (every probe
  has been using `&str` invisibly in `println!`'s format string). A
  `String` value and a `&str` value are *distinct types*; the
  E0308 family applies."

## Prerequisites

- Installed concepts:
  - Lesson 042 (load-bearing): `String` is the standard heap-allocated
    growable text type. Today's receiver is a `String` value.
  - Lesson 040 (load-bearing): the dot-form `value.method(args)` for
    receiver-bearing calls. `.trim()` is one such method, with no
    additional arguments.
  - Lesson 045 (load-bearing for the type-spelling): `&T` is the
    *shared reference type* — the `&` you put on the front of a type
    name. Today's `&str` reuses that same `&` in front of a different
    underlying type. The lesson does not unpack what `str` (without the
    `&`) is on its own.
  - Lesson 019 (load-bearing for shape): the `let name: TYPE = value;`
    annotation slot. Today extends `TYPE` to `&str`, alongside `i32`
    (019), `String` (042), `&i32` (045), and `Ordering` (051).
  - Lesson 054 (load-bearing): `read_line` appends a trailing `\n` to
    `buf`; that is the whitespace `.trim()` strips here.
  - Lesson 003: the rustc-diagnostic map.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, shell-piping with `echo "..." | ./demo` (same as
  cycle 054).

## Try It

Save as `demo.rs`:

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let trimmed: &str = buf.trim();
    println!("buf has {} bytes; trimmed = [{trimmed}]", buf.len());
}
```

Compile and run:

```console
$ rustc demo.rs
$ echo "hello" | ./demo
buf has 6 bytes; trimmed = [hello]
```

Six bytes in `buf` (`h`, `e`, `l`, `l`, `o`, `\n`) — five visible
characters plus the newline `read_line` appended. The brackets around
`{trimmed}` show that `.trim()` produced exactly `hello` — no newline,
no spaces. Try the surrounding-whitespace case to see leading
whitespace stripped too:

```console
$ printf '   spaced   \n' | ./demo
buf has 13 bytes; trimmed = [spaced]
```

Two new methods are visible. `.trim()` is the lesson's main install.
`.len()` is small collateral: a method on `String` returning the
buffer's byte count, which the line uses through the positional
placeholder `{}` in `println!`. The lesson does not unpack `.len()`'s
return type.

*Predict:* what if you put a fresh `String` value directly into a
`&str` slot? Save as `broken.rs`:

```rust
fn main() {
    let s: &str = String::new();
    println!("[{s}]");
}
```

```text
error[E0308]: mismatched types
 --> broken.rs:2:19
  |
2 |     let s: &str = String::new();
  |            ----   ^^^^^^^^^^^^^ expected `&str`, found `String`
  |            |
  |            expected due to this
  |
help: consider borrowing here
  |
2 |     let s: &str = &String::new();
  |                   +
```

Same E-code as lessons 024, 025, 026, 028, 033, 045, 046, 047, 048,
052 — *mismatched types*. The annotation `&str` set the expected type,
the right-hand side `String::new()` delivered a `String`, and rustc
refused. The caret label `expected `&str`, found `String`` is the
load-bearing line: `String` and `&str` are distinct types in the
type-checker's eyes, even though both hold text.

(`.trim()` sidesteps that: it consumes a `String` receiver and produces
a `&str` directly, with no `&`-insertion needed at the call site.)

(Full transcripts and the rustc `help:` calibration are in
`../evidence/055-string-trim.md`.)

## What Changed

- You can call `.trim()` on a `String` to get its content with leading
  and trailing whitespace removed. Newlines count as whitespace, so
  `read_line`'s trailing `\n` disappears.
- You know one new typed name, `&str` — a *string slice*, the borrowed
  view into a sequence of UTF-8 bytes. The `: TYPE` slot now accepts
  it, alongside `i32`, `String`, `&i32`, and `Ordering`.
- You know `.trim()`'s return type *is* `&str`. The annotation
  `let trimmed: &str = buf.trim();` makes that visible at the call
  site.
- `String` and `&str` are different types. Putting a `String` value
  into a `: &str` slot fires E0308 with caret label
  `expected `&str`, found `String``.
- String literals like `"hello"` (every `println!` format string this
  run has used) are also `&str`. That fact has been hiding inside the
  formatting machinery; today gives the type a name.

## Check Yourself

You write `pred.rs`:

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let cleaned: &str = buf.trim();
    println!("cleaned = [{cleaned}]");
}
```

(a) Does `rustc pred.rs` succeed?

(b) What does `echo "yes" | ./pred` print?

(c) If you replaced line 6 with `let cleaned: &str = buf;`, which
E-code would fire and which two type names would the caret label name?

*(Answers: (a) Yes. (b) `cleaned = [yes]` — no newline inside the
brackets. (c) E0308 *mismatched types*; caret label
`expected `&str`, found `String``.)*

## What To Ignore For Now

- *`.trim_start()` / `.trim_end()`* — siblings that strip only the
  leading or trailing side. Mention by name; do not exercise.
- *`.trim_matches(pat)`* and the `Pattern` trait — variants that strip
  matching characters. Trait machinery deferred.
- *`str` as a typed name without the `&`* — strictly, `str` is a
  "dynamically-sized type" usually only seen behind a reference like
  `&str`. The lesson treats `&str` as the typed name and does not
  distinguish it from `str`.
- *Lifetimes (`'a`, `'static`)* — string literals are technically
  `&'static str`; today's lesson writes `&str` without a lifetime
  annotation.
- *Deref coercion* — the rule that lets `.trim()` (defined on `str`)
  work on a `String` value. Trait machinery deferred since cycle 040.
  The rustc `help: consider borrowing here` on the broken probe
  *does* propose a fix that compiles (`&String::new()` coerces to
  `&str`); the lesson does not unpack why.
- *`String::len()` and `&str::len()`* — methods returning a byte
  count. Used in the probe's positional `{}` slot; not unpacked.
- *`usize`* — `.len()`'s return type. Carrying over from cycle 054.
- *UTF-8 encoding details, byte-vs-character distinction* — the std
  page says "Rust libraries may assume that string slices are always
  valid UTF-8." Heavy deferral.
- *Converting `&str` back to `String`* — `String::from(&str)` and
  `.to_string()`. Natural follow-up; deferred.
- *Appending to a `String`* — `.push_str(&str)`, `.push(c)`,
  `.clear()`, `.replace()`, `+`. Deferred.
- *Slicing syntax `&buf[..5]`* — different mechanism for producing a
  `&str`. Future move.
- *`.as_str()`* — explicit `String → &str` conversion. Today's `.trim()`
  returns `&str` directly, so `.as_str()` is not needed.
- *`.parse::<T>()`* — the natural next chain step
  (`buf.trim().parse::<u32>()`). Future move.
- All previously deferred items.

## Evidence

See `../evidence/055-string-trim.md`.
