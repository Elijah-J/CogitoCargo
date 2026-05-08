# Evidence — 070-rustc-explain

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Sources

### `output/docs/rust/rustc/command-line-arguments.md`

Lines 248-251, the section `## --explain: provide a detailed
explanation of an error message`:

> Each error of `rustc`'s comes with an error code; this will print
> out a longer explanation of a given error.

This is the corpus claim that (a) `--explain` exists as a `rustc`
command-line argument, and (b) what it does is "print out a longer
explanation of a given error." Direct warrant for the lesson's
"runnable instruction → multi-paragraph explainer" framing. The
same sentence is what lesson 003 cited when it first named the
`--explain` trailer.

Calibration: lesson 003's appendix already noted that this corpus
sentence is mildly idealized — not every rustc error carries an
`E####` code, and uncoded errors get no `--explain` trailer
(witnessed by lesson 003's `prntln` probe). This lesson uses
*coded* errors only (E0425, E0601), so the corpus sentence applies
unmodified.

### `output/docs/rust/error_codes/index.md`

Lines 1-4:

> # Rust error codes index
>
> This page lists all the error codes emitted by the Rust compiler.

Followed by an enumerated list of `E####` files. Direct warrant
for the lesson's framing that `E####` codes are an enumerable set
with one explainer page per code. Cited in lesson 003 already.

### `output/docs/rust/error_codes/E0425.md`

The corpus page that the lesson's central claim ties to. The page
opens (after its markdown header) with `An unresolved name was used.`
and contains four code blocks (one erroneous, three fix examples)
plus a closing paragraph about `use` declarations. The full file is
75 lines.

Load-bearing: this is the file the lesson's equivalence claim names
explicitly. The probe diff (below) is the empirical witness that
`rustc --explain E0425` reproduces this file's prose and example
content.

### `output/docs/rust/error_codes/E0601.md`

Used as the second probe target (the missing-`main` case lesson 002
installed). The page opens with `No \`main\` function was found in
a binary crate.` and is shorter than E0425 — one fix example, no
"Erroneous code examples" section, plus a one-paragraph pointer to
the Rust Book. Used as the corroborating sibling probe in *Try It*
to show the same explainer shape on a different code.

## Probe

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/070-rustc-explain.rs`)
is a one-line `fn main() {}` placeholder; the lesson's move is a
documentation-lookup command that takes no source file, so the file
is a paper-trail stub only. The substantive evidence is the four
captured commands below.

### Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64
```

### Probe 1: `rustc --explain E0425` from any directory

Captured in a fresh empty temp dir created with `mktemp -d`:

````text
$ ls -la
total 0
drwx------    2 eli  staff     64 May  7 15:00 .
drwx------@ 838 eli  staff  26816 May  7 15:00 ..
$ rustc --explain E0425
An unresolved name was used.

Erroneous code examples:

```
something_that_doesnt_exist::foo;
// error: unresolved name `something_that_doesnt_exist::foo`

// or:

trait Foo {
    fn bar() {
        Self; // error: unresolved name `Self`
    }
}

// or:

let x = unknown_variable;  // error: unresolved name `unknown_variable`
```

Please verify that the name wasn't misspelled and ensure that the
identifier being referred to is valid for the given situation. Example:

```
enum something_that_does_exist {
    Foo,
}
```

Or:

```
mod something_that_does_exist {
    pub static foo : i32 = 0i32;
}

something_that_does_exist::foo; // ok!
```

Or:

```
let unknown_variable = 12u32;
let x = unknown_variable; // ok!
```

If the item is not defined in the current module, it must be imported using a
`use` statement, like so:

```
use foo::bar;
bar();
```

If the item you are importing is not defined in some super-module of the
current module, then it must also be declared as public (e.g., `pub fn`).
$ echo "exit=$?"
exit=0
$ ls -la
total 0
drwx------    2 eli  staff     64 May  7 15:00 .
drwx------@ 838 eli  staff  26816 May  7 15:00 ..
````

The 57-line transcript ends with rustc exiting 0 and the directory
unchanged. No `.rs` file was needed; no executable was produced.
This is the load-bearing observation for the lesson's "documentation
lookup, not a compilation" claim.

### Probe 2: `rustc --explain E0601`

````text
$ rustc --explain E0601
No `main` function was found in a binary crate.

To fix this error, add a `main` function:

```
fn main() {
    // Your program will start here.
    println!("Hello world!");
}
```

If you don't know the basics of Rust, you can look at the
[Rust Book][rust-book] to get started.

[rust-book]: https://doc.rust-lang.org/book/
$ echo "exit=$?"
exit=0
````

15 lines, exit 0. Confirms the same shape on a different code:
headline summary on line 1, then a fix example. E0601's explainer
has no separate "Erroneous code examples:" section — the page
proceeds straight from the headline to the fix.

### Probe 3: `rustc --explain E0425` against `output/docs/rust/error_codes/E0425.md`

Compare the captured `--explain E0425` output to the corpus file
with `diff -u`:

````text
$ diff -u /tmp/explain_e0425.txt output/docs/rust/error_codes/E0425.md
--- /tmp/explain_e0425.txt
+++ output/docs/rust/error_codes/E0425.md
@@ -1,8 +1,13 @@
+<!-- source: https://doc.rust-lang.org/stable/error_codes/E0425.html -->
+# [Error code E0425](#error-code-e0425)
+
 An unresolved name was used.

 Erroneous code examples:

-```
+```rust
+#![allow(unused)]
+fn main() {
 something_that_doesnt_exist::foo;
 // error: unresolved name `something_that_doesnt_exist::foo`

@@ -17,40 +22,53 @@
 // or:

 let x = unknown_variable;  // error: unresolved name `unknown_variable`
+}
 ```

-Please verify that the name wasn't misspelled and ensure that the
+Please verify that the name wasn't misspelled and ensure that the
 identifier being referred to is valid for the given situation. Example:

-```
+```rust
+#![allow(unused)]
+fn main() {
 enum something_that_does_exist {
     Foo,
 }
+}
 ```
... (further analogous diffs through the rest of the file)
````

The diff falls into three small categories — every single
difference fits one of them:

1. **Markdown decoration the corpus adds** — a `<!-- source: ... -->`
   HTML comment and a `# [Error code E0425]...` heading at the top.
   Not present in `--explain` output.

2. **Code-fence language and compile-test scaffolding** — the
   corpus uses `\`\`\`rust` instead of `\`\`\``, and wraps each
   inline code block with `#![allow(unused)]` / `fn main() { ... }`
   so its automated test harness can compile each example. The
   `--explain` form omits these because it is meant to be read by
   humans in a terminal.

3. **Typographical apostrophe** — the corpus has the curly
   right-single-quote `’` ("wasn’t"); `--explain` has the ASCII
   straight apostrophe `'` ("wasn't"). One character.

Every paragraph of prose, every example identifier, every comment
inside the code blocks (`// error: unresolved name ...`,
`// ok!`), every connector (`Or:`, `// or:`), and the closing
two-paragraph block about `use` declarations are present in both
sources, in the same order. The lesson's equivalence claim — "the
explainer is the same content as the corpus page" — is empirical
and exact modulo the three small categories above.

### Probe 4: `rustc --explain E0425` exit code and side effects

Captured separately to make the "no compilation, no executable"
observation explicit:

```text
$ D=$(mktemp -d) && cd "$D"
$ ls -la
total 0
drwx------    2 eli  staff     64 May  7 15:06 .
drwx------@ 838 eli  staff  26816 May  7 15:06 ..
$ rustc --explain E0425 > out.txt 2>&1
$ echo "exit=$?"
exit=0
$ ls -la
total 8
drwx------    3 eli  staff     96 May  7 15:06 .
drwx------@ 838 eli  staff  26816 May  7 15:06 ..
-rw-r--r--    1 eli  staff   1004 May  7 15:06 out.txt
```

Only `out.txt` (the redirected stdout) appears; rustc itself
created no `.rs`, no `.o`, no executable, no temporary build
artifacts. Exit 0. This is the empirical witness that `--explain`
is a pure documentation lookup with no compilation side effects —
which is what the lesson body claims when it says "no source file
is needed and nothing is compiled."

## Negative / contrast probe

The lesson's central claim has a contrast shape ("the printed
explainer is the same as the corpus page"). The diff in Probe 3
already serves as the negative-shape probe: it enumerates every
difference between the two surfaces, and the differences are all
markdown/test-harness scaffolding rather than content. No separate
broken-contrast probe (e.g. running `--explain` on a non-existent
`E####`) is captured because it would teach a different concept
(rustc's error handling on bad input to `--explain`), not the
lesson's installed move.

## Prior lessons

Direct prerequisites (load-bearing claims):

- `001-rustc-compile-and-run` (accepted) — establishes the
  baseline that `rustc` is invokable from a terminal.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — names
  the `For more information ... rustc --explain ECODE` trailer in
  its prose and in its captured E0601 transcript, and explicitly
  defers the interactive use of `--explain` to a later lesson.
  This lesson is that later lesson; the deferral line in lesson
  003's *What To Ignore* (line 263 of the lesson file: "Using
  `rustc --explain ECODE` interactively. Note only that it
  exists for errors that carry an `E####` code; using it is a
  later move.") is the explicit pointer this cycle resolves.
- `005-let-binding` (accepted) — captured the canonical E0425
  diagnostic block end-to-end including the
  `For more information about this error, try \`rustc --explain
  E0425\`.` trailer (lesson 005 evidence Run 2). This lesson's
  "the trailer is a runnable instruction" framing leans on that
  capture as the prior witness that the trailer appears in the
  wild on this exact host and toolchain.

Older supporting lessons (mentioned by id only, not load-bearing
for any individual claim):

- `002-fn-main-entry-point` — produced an E0601 transcript with
  the same `--explain` trailer; lesson 003 already cited it. The
  E0601 corpus page is one of the two probe targets here, but the
  load-bearing 002 claim ("E0601 fires when `main` is missing") is
  not invoked in the lesson body. Today's lesson 070 uses E0601
  only as a sibling-shape probe.
- `068-let-binding-scope` and `069-rustc-warnings` — recent
  diagnostics-shape lessons on the same host and toolchain.
  Mentioned only to confirm the host environment is unchanged
  since the lesson 003 / 005 captures.
