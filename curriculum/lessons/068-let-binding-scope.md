---
id: 068-let-binding-scope
status: accepted
evidence: ../evidence/068-let-binding-scope.md
---

# A `let` binding's *scope* is its enclosing `{ ... }` block

## The Move

Inside `fn main`, put a `let` binding *inside* an inner `{ ... }`
block (here, the body of an `if`). Use the name *inside* that same
block — fine. Then move the use to a line *after* the block's
closing `}`. The program no longer compiles: `rustc` fires the same
`error[E0425]: cannot find value ... in this scope` from lesson 005,
with the caret on the use site outside the block. Move the use back
inside the block to fix it.

## Mental Model Delta

- Before: "Lesson 005 said `let` lets later statements use the name.
  The diagnostic says 'in this scope', but I have not been told what
  *scope* means."
- After: "*Scope* is the region of code where a name has meaning.
  For a `let` binding, that region runs from just after the `let`
  line down to the closing `}` of the enclosing `{ ... }` block.
  Outside it, `rustc` emits E0425. 'In this scope' in the error
  names exactly that region."

## Prerequisites

- Installed concepts:
  - Lesson 001 / 002: `rustc file.rs` produces an executable; the
    body of `fn main()` is delimited by `{ ... }`.
  - Lesson 003: rustc errors have a headline, `-->` location,
    source excerpt with caret, and optional `help:` lines.
  - Lesson 005 (load-bearing): `let name = value;` binds a name;
    later statements can use it; without the binding, rustc fires
    E0425. This lesson narrows "later statements" to "later
    statements inside the same enclosing `{ ... }` block."
  - Lesson 014: `if condition { ... }` runs the inner block when
    the condition is `true`. We use it as the smallest available
    inner block inside `fn main`. Whether the `if` block actually
    runs is not what determines scope.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let n = 7;
    if n > 5 {
        let label = "big";
        println!("n = {n}, label = {label}");
    }
}
```

Two `let` bindings sit at different nesting levels: `let n = 7;`
in the outer `{ ... }` (the body of `fn main`), `let label = "big";`
in the inner `{ ... }` (the body of the `if`). The `println!` is
also inside the inner block, so both names are in scope where they
are used. Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 7, label = big
```

Now the contrast. Edit `demo.rs` so the `println!` lives *after*
the inner block ends:

```rust
fn main() {
    let n = 7;
    if n > 5 {
        let label = "big";
    }
    println!("n = {n}, label = {label}");
}
```

The only structural change: the `println!` moved from inside the
inner `{ ... }` to a line after that block's closing `}`. Compile:

```console
$ rustc demo.rs
error[E0425]: cannot find value `label` in this scope
 --> demo.rs:6:33
  |
6 |     println!("n = {n}, label = {label}");
  |                                 ^^^^^
  |
help: the binding `label` is available in a different scope in the same function
 --> demo.rs:4:13
  |
4 |         let label = "big";
  |             ^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

Read it with the lesson 003 map. **Headline**: `error[E0425]:
cannot find value `label` in this scope`. **Location** points at
the use site. **Source excerpt with caret** sits under `label`
inside `{label}`. **Help** adds a secondary `-->` pointing back at
line 4, the *original* `let label = "big";`, labelling it
"available in a different scope in the same function". The
binding exists; its scope is a different region from where you
used it.

Notice the asymmetry. Both `n` and `label` appear in the same
broken `println!`, but the error fires only on `label`. `n` was
bound in the outer block (the body of `fn main`), so it is still
in scope on the line after the `if`; `label` was bound in the
inner block, so its scope ended at that block's closing `}`. The
fix is to move the `println!` back inside the inner block.

The Rust Reference states the rule directly: "`let` statement
bindings range from just after the `let` statement until the end
of the block where it is declared."

## What Changed

- *Scope* is a usable noun: the region of code where a name has
  meaning. For a `let` binding, that region is from just after the
  `let` line down to the closing `}` of the enclosing `{ ... }` block.
- The phrase "in this scope" in `error[E0425]` names that region.
- `rustc` decides scope at compile time, by where you wrote the
  code — not at runtime, by which blocks happen to run.
- The contrast made it concrete: in the same broken `println!`, `n`
  resolves and `label` does not, because they were bound in
  different blocks.

## Check Yourself

You write `pick.rs`:

```rust
fn main() {
    let outer = 1;
    if outer > 0 {
        let inner = 2;
        println!("outer = {outer}");
    }
    println!("inner = {inner}");
}
```

(a) Does `rustc pick.rs` produce an executable? Which identifier
does the caret point at?

(b) `outer` also appears on the last line. Does the error fire on
`outer` too?

(c) Without changing any other line, where could you move
`println!("inner = {inner}");` so the file compiles?

(Answers: (a) No executable. `rustc` prints `error[E0425]: cannot
find value `inner` in this scope`, caret on `inner` inside `{inner}`
on the last `println!`. (b) No. `outer` was bound in the body of
`fn main`, so it is in scope on the last line; only `inner` was
bound in the inner `if` block. (c) Move it up, inside the `if`
block, between `let inner = 2;` and the `if` block's closing `}`.)

## What To Ignore For Now

This lesson installs one idea: a `let` binding's scope is its
enclosing `{ ... }` block. Each of the following is real and
deferred:

- *Block expressions* — that a `{ ... }` block has a *value*
  (statement-vs-expression in lesson 024; expression-form `if` in
  lesson 026). Here `{ ... }` is only a region.
- *Function-parameter scope*. Lesson 020 already operationally
  showed a parameter is usable in its function's body.
- *Shadowing* and how it interacts with scope (lessons 007, 057).
- *Match-arm scope*. Used operationally in lessons 030, 031, 058.
- *Closure-capture scope*, *lifetimes*, and the borrow checker's
  region rules. All deferred.
- *Item scope* (functions, modules). Different rules; not covered.
- Whether the inner block "ran" at runtime. Scope is compile-time;
  the contrast would still fail to compile with `if false`.

## Evidence

See `../evidence/068-let-binding-scope.md`.
