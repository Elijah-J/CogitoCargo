---
id: 060-input-prompt-loop
status: accepted
evidence: ../evidence/060-input-prompt-loop.md
---

# Prompt for input until the user types a parseable integer

## The Move

Fifteen previously-installed cycles snap together into one runnable
program — the "ask, retry on garbage, accept on success" shape.
No new Rust mechanic today; the value is composition.

```rust
use std::io;

fn main() {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let n: i32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("got: {n}");
        break;
    }
}
```

`rustc demo.rs` exits 0. Three different inputs all reach the same
result:

```console
$ echo "42" | ./demo
got: 42
$ printf 'abc\n42\n' | ./demo
got: 42
$ printf 'abc\nxyz\n  42  \n' | ./demo
got: 42
```

## The Composition Trace

Line by line, naming each cycle that licenses each piece.

- *Line 1, `use std::io;`* — cycle 044. Brings `io` into scope so
  line 6 can write `io::stdin()` instead of `std::io::stdin()`.
- *Line 4, `loop { ... }`* — cycle 027. Repeats the body until
  line 11's `break;` fires.
- *Line 5, `let mut buf = String::new();`* — cycle 042
  (`String::new()`) plus cycle 006 (`let mut` makes the binding
  mutably borrowable; without it line 6's `&mut buf` would not
  type-check). Fresh empty `String` each pass.
- *Line 6, `io::stdin().read_line(&mut buf).expect("Failed to read line");`*
  is exactly cycle 054's one-statement composition:
  - `io::stdin()` — cycle 050.
  - `.read_line(&mut buf)` — cycle 054. Appends one
    newline-terminated line of stdin into `buf`, returning a
    `Result` (cycle 052) whose `Ok` payload is the byte count.
  - `&mut buf` — cycle 048's argument shape.
  - `.expect("Failed to read line")` — cycle 053. Yields the byte
    count (discarded) on success, panics on `Err`.
  - The three-call chain is cycle 049's left-associative chaining.
- *Line 7, `let n: i32 = match buf.trim().parse() { ... };`* is the
  second braid:
  - `buf.trim()` — cycle 055. Returns a `&str` with leading and
    trailing whitespace stripped — including the trailing `\n` that
    `read_line` appended *and* any spaces the user typed. The
    third probe input `"  42  "` makes the spaces case visible.
  - `.parse()` — cycle 056. Returns `Result<TARGET, _>`; the
    `: i32` on `n` flows back through the match and pins
    `TARGET = i32`.
  - `match ... { Ok(num) => num, Err(_) => continue }` — cycle 058.
    `Ok(num)` binds the parsed integer; `Err(_)` matches any `Err`
    and discards its payload via cycle 031's wildcard reused
    inside the constructor.
  - `Err(_) => continue` — cycle 059's divergent-arm rule lets
    this arm not produce an `i32`; control jumps to the loop head
    (cycle 035) instead of producing a value. The match's `i32`
    type comes entirely from the `Ok(num)` arm.
- *Line 11, `println!("got: {n}");`* — cycle 011's `{name}`
  placeholder, only reachable on the `Ok` arm.
- *Line 12, `break;`* — cycle 027. Exits the `loop`.

No syntax in the main path is unexplained.

## Mental Model Delta

- *Before:* "My installed cycles each look useful in isolation; I
  have not seen them all in one program."
- *After:* "They are *complete enough* to write the Book's
  input-prompt loop end-to-end without anything new. The shape:
  outer `loop`, fresh buffer, `read_line` chain, `match` on
  `.trim().parse()` with `Ok(num) => num` and `Err(_) => continue`,
  then use the integer and `break;`. The Book uses this pattern
  for interactive read-and-validate input."

## Prerequisites

- Installed concepts: cycles 001 (compile/run), 002 (`fn main`),
  005 (`let`), 006 (`let mut`), 011 (`{name}`), 019 (`: i32`), 027
  (`loop`/`break`), 035 (`continue`), 040 (dot-form method call),
  042 (`String::new()`), 044 (`use std::io;`), 048 (`&mut binding`
  argument), 049 (method chaining), 050 (`io::stdin()`), 052
  (`Result<T, E>`), 053 (`.expect("msg")`), 054 (`read_line(&mut
  buf)`), 055 (`.trim()`), 056 (`.parse()` plus inference), 058
  (`Ok(num)` / `Err(_)` payload patterns), 059 (`continue` as
  diverging arm). All load-bearing — remove any one and a
  different piece of the program fails.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, and shell-piping with `echo` / `printf` to
  feed stdin (as since cycle 053).

## Try It

Save the snippet as `demo.rs` and run the three piped inputs from
*The Move*. Each exercises a different shape: one `Ok` pass
(`"42"`); `Err(_) => continue` then `Ok` (`'abc\n42\n'`); two
`continue`s then `"  42  \n"`, where `.trim()` strips both the
surrounding spaces *and* the trailing `\n` before `.parse()` sees
`"42"`. The probe being green for all three inputs is the
observation.

## What Changed

- You can write the canonical Rust input-prompt loop end-to-end
  with no unexplained syntax: `loop { fresh buf; read_line chain;
  match .trim().parse() { Ok(num) => num, Err(_) => continue };
  use the integer; break; }`.
- "Are the cycles I have learned useful together?" — yes; fifteen
  compose into the Book's input-prompt pattern.
- The Book's actual guessing-game (ch02 lines 1167-1174) uses
  cycle 057's *type-changing shadowing* — `let guess: u32 = match
  guess.trim().parse() { ... };`. Today's probe substitutes `: i32`
  (cycle 019 installed `i32`; `u32` is deferred) and uses two
  names (`buf`, `n`) to keep the trace explicit; the cycle-057
  shadow-rename is one edit away.
- Composition cycles install no new feature. Their value is "no
  syntax in this program is new" plus "these installed cycles fit
  together to do real work."

## Check Yourself

(a) If you move `let mut buf = String::new();` *outside* the loop
(above `loop {`), the program still compiles, but its behavior on
`printf 'abc\n42\n' | ./demo` changes. Predict what happens.

(b) If you removed line 12's `break;`, what would happen on
`echo "42" | ./demo`?

*(Answers: (a) Cycle 054's *appends* note becomes load-bearing:
pass 2's `read_line` appends `"42\n"` onto pass 1's `"abc\n"`,
giving `"abc\n42\n"`; `.trim()` leaves `"abc\n42"`, which does not
parse. The `Err` arm fires forever. The fix is `String::clear()`
between iterations — deferred. (b) Pass 1 prints `got: 42`, then
control falls through to `}` and the loop iterates. `read_line`
returns `Ok(0)` at EOF; `.trim().parse()` on `""` is `Err`, so
`continue` fires, and the next `read_line` returns `Ok(0)` again —
forever. The `break;` is what makes the program exit after the
first successful read.)*

## What To Ignore For Now

- *`u32` as a typed name* — the Book's actual annotation. Same
  mechanic, different sibling. Defer.
- *Type-changing shadowing the buffer name into the parsed value*
  — the Book writes `let guess: u32 = match guess.trim().parse()
  { ... };` (cycle 057's pattern, with `u32` the unsigned sibling
  of `i32`), shadowing `guess: String` from line 5. Same
  observable behavior as today's two-name form.
- *`String::clear()`* — clearing the buffer between iterations,
  the alternative to today's "fresh `String::new()` each pass."
  Pairs with *Check Yourself* (a).
- *Reading from a terminal vs. a pipe* — terminal stdin is
  line-buffered, blocking until Enter; piped stdin delivers lines
  then EOFs.
- *EOF handling* — `read_line` returns `Ok(0)` at EOF, *not*
  `Err`; without `break;` the program would loop forever after the
  first successful read. Pairs with *Check Yourself* (b).
- *`break value;`* — cycle 028's loop-as-expression shape lets
  you write `let n: i32 = loop { ... break n; };` instead. Same
  behavior.
- *Comparing the parsed integer against another value with `cmp`*
  — the next layer of the Book's guessing-game.
- *The `rand` external crate* for the Book's secret number.
  Largest remaining deferral on the Book path; introduces
  `Cargo.toml` and `cargo build`.
- All previously deferred items.

## Evidence

See `../evidence/060-input-prompt-loop.md`.
