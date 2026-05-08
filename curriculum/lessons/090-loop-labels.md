---
id: 090-loop-labels
status: accepted
evidence: ../evidence/090-loop-labels.md
---

# Target an outer loop with a label: `'outer:` + `break 'outer;`

## The Move

Lesson 027 installed `break;` exiting the innermost enclosing loop.
Lesson 035 installed `continue;` skipping to the next iteration of
the innermost. Today extends both with a way to target a specific
*outer* loop instead.

A loop can carry a *label*. Write the label *before* the loop
keyword as a single-quote, then a snake_case identifier, then a
colon. Inside any nested loop, `break 'name;` exits the loop named
`'name` (even if it is an outer loop), and `continue 'name;` jumps
to the next iteration of the loop named `'name`. The label sits
on `loop`, `while`, or `for` — all three work.

```rust
'outer: for i in 0..3 {
    for j in 0..3 {
        if j == 1 {
            break 'outer; // exits BOTH loops, not just the inner one
        }
    }
}
```

## Mental Model Delta

- *Before:* "`break;` exits the innermost loop. `continue;` skips
  to the next pass of the innermost loop. To exit *two* nested
  loops at once I would have to set a flag and check it after the
  inner loop returns."
- *After:* "Each loop can be labeled by prefixing it with `'name:`.
  Bare `break;` still exits the innermost loop, but `break 'outer;`
  exits the loop named `'outer` directly — even when an inner loop
  sits between the `break` and the labeled outer. Same machine for
  `continue 'outer;`: control returns to the *outer* loop's head,
  skipping the rest of both bodies. The label is the
  disambiguator. Without nesting (or when the innermost is the
  target), no label is needed."

## Prerequisites

- Installed concepts:
  - Lesson 027 (load-bearing): `break;` exits the innermost
    enclosing loop. Today adds the `break 'name;` form that exits
    a *named* enclosing loop.
  - Lesson 035 (load-bearing): `continue;` returns control to the
    head of the innermost enclosing loop. Today adds the
    `continue 'name;` form that targets a named loop.
  - Lesson 022 (cited): `for var in 0..N { ... }` is used by both
    nests in the probe; lesson 017 (`while`) is named in *What
    Changed* but not exercised.
  - Lessons 014, 013, 011 (cited): `if`, `==`, and `println!` make
    up the rest of the probe.
  - Lesson 089 (cited): snake_case naming applies after the `'`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, save `demo.rs`:

```rust
fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                println!("i = {i}, j = {j}: break 'outer");
                break 'outer;
            }
            println!("i = {i}, j = {j}");
        }
    }
    println!("after the labeled loops");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
i = 0, j = 0
i = 0, j = 1: break 'outer
after the labeled loops
```

Walk through it. The outer `'outer: for i in 0..3` would normally
run three times (`i` taking `0`, `1`, `2`). On the first outer
pass (`i = 0`), the inner `for j in 0..3` starts. With `j = 0`,
the `if` is `false` and the trailing `println!` runs, printing
`i = 0, j = 0`. With `j = 1`, the `if` is `true`: the inner block
prints its line and runs `break 'outer;`. That exits the loop
labeled `'outer` directly — *both* loops end. Execution resumes
after the closing `}` of the outer loop, at
`println!("after the labeled loops");`. The outer body never
sees `i = 1` or `i = 2`.

The Book describes the rule directly:

> If you have loops within loops, `break` and `continue` apply to
> the innermost loop at that point. You can optionally specify a
> *loop label* on a loop that you can then use with `break` or
> `continue` to specify that those keywords apply to the labeled
> loop instead of the innermost loop. Loop labels must begin with
> a single quote.

The contrast: replace `break 'outer;` with bare `break;`. Now
`break;` falls back to lesson 027 and exits only the *inner* loop.
The outer loop continues for `i = 1` and `i = 2`, restarting and
breaking the inner loop each time — six inner-loop lines instead
of two. Same surface keyword, different target loop, different
output. The evidence appendix captures the contrast transcript.

## What Changed

- A loop can carry a *label*. Write `'name:` (single-quote,
  snake_case identifier, colon) immediately before the loop keyword
  (`loop`, `while`, or `for`). All three loop forms accept labels.
- `break 'name;` exits the loop labeled `'name`, even when an
  unlabeled loop is nested inside it. `continue 'name;` returns
  control to the head of the loop labeled `'name`, skipping the
  rest of every nested body in between.
- Bare `break;` and `continue;` are unchanged (lessons 027 and
  035): they still apply to the *innermost* enclosing loop. The
  label form *adds* a way to target an outer loop; it does not
  replace the bare form.
- The leading single quote is required. `outer:` without the `'`
  is a syntax error: rustc rejects it with `error: malformed loop
  label` and offers `'outer:` as the fix. The `'` is what tells
  rustc to parse the next identifier as a label, not as a regular
  name.
- Label names follow the snake_case convention (lesson 089).
  `'outer`, `'counting_up`, `'rows` are conventional.

## Check Yourself

You write `q.rs` containing:

```rust
fn main() {
    'rows: for r in 0..3 {
        for c in 0..3 {
            if c == 1 {
                continue 'rows;
            }
            println!("r = {r}, c = {c}");
        }
    }
}
```

You run `rustc q.rs && ./q`.

(a) How many `r = ..., c = ...` lines does the program print?

(b) For each outer pass `r = 0`, `r = 1`, `r = 2`, which values of
`c` produce a printed line?

(c) If you change `continue 'rows;` to bare `continue;`, how many
lines does the program print then?

(Answers: (a) Three. On every outer pass, `c = 0` prints; `c = 1`
fires `continue 'rows;` and jumps straight to the next outer
iteration, skipping `c = 2`. (b) Only `c = 0` for each `r`.
(c) Six. With bare `continue;` the skip applies only to the inner
loop, so `c = 1` is skipped but `c = 2` still runs on every outer
pass. Same surface program, two control-flow targets, two output
sets.)

## What To Ignore For Now

- *Lifetime parameters* `'a`, `'static`. Same single-quote prefix
  but a different concept (annotations on references). Defer.
- *`break 'name value;`* — combining today with lesson 028's
  `break value;` to return a value from a labeled outer loop.
  Composition; defer.
- *Labeled bare blocks* like `'name: { ... break 'name; }`. Defer.
- *Three or more nested loops*. Composition; nothing new.
- *The `unused_labels` warning* that fires when a labeled loop
  uses only bare `break;` / `continue;`. Captured in the evidence
  appendix's contrast transcript; not centered today.
- *Label-shadowing rules* (an inner `'a:` may shadow an outer
  `'a:` per the Reference). Defer.
- *Interaction with `match` and `if let`*. Defer.

## Evidence

See `../evidence/090-loop-labels.md`.
