---
id: 056-str-parse-to-i32
status: accepted
evidence: ../evidence/056-str-parse-to-i32.md
---

# Convert a `&str` to an `i32` with `.parse()`

## The Move

`&str` has a method `.parse()` that converts its contents into a
value of some target type and returns a `Result` (lesson 052). The
chain `.parse().expect("...")` (lesson 049) hands that `Result` to
`.expect` (lesson 053), yielding the parsed value or panicking:

```rust
fn main() {
    let n: i32 = "42".parse().expect("not a number");
    println!("n = {n}");
}
```

`rustc` accepts the program; `./demo` prints `n = 42`. The new
mechanic has two halves.

The first half is the method. `"42"` is a `&str` (cycle 055), and
`.parse()` returns a `Result` carrying *some target type* on the
`Ok` side. `.expect("not a number")` then takes the `Ok` payload
out the way it took `read_line`'s byte count out in cycle 054.

The second half is *which* target type. The right-hand side of the
`let` does not contain `i32` anywhere — only the binding's `: i32`
annotation does. rustc reads that annotation as a constraint:
"this binding holds an `i32`, so what flows in must produce an
`i32`." It walks back through `.expect` (which doesn't change the
type — `Ok(T) -> T`) into `.parse()` and concludes that `.parse()`
must produce a `Result<i32, ...>`. That conclusion is what selects
the integer flavor of `.parse()`.

This back-flow of "use site decides type" is *type inference*. We
have leaned on it silently before — `let label = match ... { ... }`
in cycle 051 inferred `label`'s type from the arms. Today is the
first lesson where the inference goes the *other* way: an
annotation on the *left* drives a method's behavior on the *right*.

## Mental Model Delta

- *Before:* "`&str` is a string slice (cycle 055). To turn `"42"`
  into a number I'd need... I don't know what."
- *After:* "`&str` has a `.parse()` method that converts its content
  into another type and returns a `Result`. The target type isn't
  written on the call — rustc infers it from how the result is
  used, typically a `: i32` (or other) annotation on the receiving
  `let`. With that annotation, `"42".parse().expect(...)` evaluates
  to the integer `42`. Without it, rustc refuses to compile,
  because there are many number types `.parse()` could produce."

## Prerequisites

- Installed concepts:
  - Lesson 055 (load-bearing): `&str` is a typed name — string
    literals like `"42"` are `&str`. `.parse()` is a method on `&str`.
  - Lesson 053 (load-bearing): `.expect("msg")` consumes a `Result`,
    yielding the `Ok` payload as a plain value or panicking with
    `msg: <Err>` on `Err`. The runtime-panic shape is reused
    unchanged today — second formal install path.
  - Lesson 052 (load-bearing): `.parse()` returns `Result<T, E>`.
  - Lesson 049 (load-bearing): method chaining — the receiver of
    `.expect` is the call expression `"42".parse()`.
  - Lesson 040 (load-bearing): the dot-form `value.method(args)`.
  - Lesson 019 (load-bearing): the `let name: TYPE = value;`
    annotation slot. Today's lesson is the first to surface that
    the annotation also *constrains* the right-hand side.
  - Lessons 001, 002, 005: compile and run, `fn main`, `let` plus
    the `{name}` placeholder.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, knowing that program output goes to stdout
  while diagnostic text goes to stderr.

## Try It

Save the snippet from *The Move* as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 42
```

`"42".parse()` evaluates to `Ok(42)` (the `42` is an `i32`,
inferred from the `: i32` annotation); `.expect("not a number")`
extracts the `42`; the binding accepts it; `println!` prints
`n = 42`.

*Predict (runtime panic).* Change `"42"` to `"abc"` and rebuild.
`rustc broken.rs` still exits `0`, but `./broken` produces a panic
on stderr:

```text
thread 'main' (...) panicked at broken.rs:2:32:
not a number: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`echo $?` reports `101`. This is exactly cycle 053's panic shape:
`.parse()` returned an `Err(...)` because `"abc"` is not a valid
integer; `.expect("not a number")` saw the `Err` and panicked with
`<msg>: <Err payload printed>`. The name `ParseIntError` is the
specific error type `.parse::<i32>` produces; treat it as text
rustc prints, not as something you write today.

*Predict (no annotation).* Drop the `: i32`:

```rust
fn main() {
    let n = "42".parse().expect("not a number");
    println!("n = {n}");
}
```

`rustc` *refuses* this program. The diagnostic essentially says
that `n`'s type "must be known at this point", and since `.parse()`
can produce many number types rustc cannot pick one. The fix is to
put the annotation back. (The full diagnostic is in the evidence
appendix.)

## What Changed

- `&str` has a method `.parse()` that converts its content into a
  value of some target type and returns a `Result<TARGET, ...>`.
  The target type is not written on the call.
- *Type inference* now explicit: rustc reads a binding's `: TYPE`
  annotation as a constraint on the right-hand side and uses it to
  decide what `.parse()` should produce. `: i32` is what makes
  `"42".parse()` produce a `Result<i32, ...>`.
- The chain `"42".parse().expect("not a number")` reuses cycles
  049/052/053. On `Ok` it yields the parsed value as a plain `i32`;
  on `Err` it panics at runtime — the second program-level use of
  cycle 053's panic shape.
- Without the annotation (and without other clues), rustc refuses
  to compile.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let n: i32 = "7".parse().expect("not a number");
    let m: i32 = n + 1;
    println!("m = {m}");
}
```

(a) Does `rustc tiny.rs` succeed? What does `./tiny` print?

(b) If you change `"7"` to `"seven"`, does rustc still succeed?
What's on stdout vs stderr, and what does `echo $?` print?

(c) If you remove the `: i32` from the first `let` (annotation
gone, `"7"` still in place), does rustc accept the program?

*(Answers: (a) Yes; stdout `m = 8`; exit `0`. (b) rustc still
succeeds — runtime panic, not compile error. stdout empty; stderr
has a `thread 'main' ... panicked at ...` block whose message
reads `not a number: ParseIntError { kind: InvalidDigit }`; exit
`101`. (c) No — even though `n + 1` could in principle pin
`n: i32` indirectly, rustc demands the annotation up front. See
the evidence appendix for the exact diagnostic.)*

## What To Ignore For Now

- *The `FromStr` trait* — `.parse`'s real signature is
  `fn parse<F>(&self) -> Result<F, <F as FromStr>::Err> where F: FromStr`.
  Trait machinery deferred since cycle 040.
- *Generics and the turbofish* — `"42".parse::<i32>()` writes the
  target type explicitly. Equivalent to today's annotation form;
  deferred.
- *Generic functions* — `<T>` on a function signature. Deferred.
- *Type-inference variables `_`* — partial annotations like
  `Result<i32, _>`. Deferred.
- *`ParseIntError`* — the named error type appearing in panic
  output. Not a typed name to use yet.
- *`.parse()` on other targets* — `.parse::<u32>()`,
  `.parse::<f64>()`, `.parse::<bool>()`, etc. Same method, deferred
  along with the other typed names.
- *Negative numbers and whitespace* — `"-7"` parses; `" 7"` does
  not. `.parse()` does *not* trim whitespace, which is exactly why
  the Book chains `.trim().parse()` (cycle 055's `.trim()` strips
  `read_line`'s `\n`, then `.parse()` sees only digits). The full
  chain `buf.trim().parse().expect(...)` is the next natural move.
- *Type-changing shadowing* —
  `let guess: u32 = guess.trim().parse().expect(...)` reuses a name
  with a different type. Deferred (carrying cycle 007's deferral).
- *Match on the parse result* —
  `match s.parse() { Ok(num) => num, Err(_) => continue, }` is the
  non-panicking way. Deferred.
- *The `?` operator* — error propagation without panic. Deferred.
- *E0282 / E0284* — the specific E-code rustc fires for missing
  type annotations. Captured in evidence; lesson body avoids the
  E-code.
- All previously deferred items.

## Evidence

See `../evidence/056-str-parse-to-i32.md`.
