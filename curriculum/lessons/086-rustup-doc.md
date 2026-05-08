---
id: 086-rustup-doc
status: accepted
evidence: ../evidence/086-rustup-doc.md
---

# Open the local Rust documentation: `rustup doc`

## The Move

From any directory in any shell, run:

```console
$ rustup doc
Opening docs in your browser
```

A new browser tab opens to the local Rust documentation index — a page
that lists the doc sets that ship with your toolchain (the Book, the
Reference, the standard library API docs, and several others). Like the
commands from lesson 085, it touches no `.rs` file, no `Cargo.toml`, no
`cd`. It runs anywhere.

## Mental Model Delta

- *Before:* "Lesson 085 gave me `rustc --version`, `cargo --version`,
  and `rustup update` — three sibling commands that inspect or refresh
  the *toolchain*. I have no command for looking up *what types and
  functions exist* in Rust, or for re-reading the Book, when I am
  offline."
- *After:* "Rust ships a copy of its own documentation locally —
  installed alongside `rustc` and `cargo` by `rustup` — and `rustup doc`
  opens that local copy in my default browser. The set includes the
  Book, the Reference, and the standard library API docs (and several
  more). It is the offline answer to *what does this stdlib type do?*
  and *what does the Book say about this?*. Project-scoped docs (the
  documentation for the dependencies of *my* project) are a different
  command, deferred."

## Prerequisites

- Installed concepts:
  - Lesson 085 (`085-toolchain-housekeeping`, load-bearing): `rustup`
    is the program that installed Rust on this host and manages the
    toolchain. Today extends 085's `rustup update` (refresh the
    toolchain) with a sibling subcommand `rustup doc` (read the
    documentation that came with the toolchain). The Book introduces
    `rustup doc` in the same chapter as `rustup update`
    (Ch1-1 *Reading the Local Documentation*, lines 130-138).
  - Lesson 001 (`001-rustc-compile-and-run`, cited): shell prompt;
    typing a command and reading its output. Today reuses these
    without re-installing them.
- Ordinary computer-use assumptions: terminal/shell prompt; the host
  has a default web browser configured (the same kind of ordinary fact
  as "the host has a terminal"); no network needed — the docs are
  already on disk.

One additional fact named once and not re-installed: the local Rust
documentation is HTML at a path on disk under the rustup-managed
toolchain directory, and `rustup doc` resolves to a `file://` URL and
hands it to the default browser. You do not need to know the path; the
command knows it.

## Try It

Open any terminal in any directory. Run:

```console
$ rustup doc
Opening docs in your browser
```

Two things happen. First, the terminal prints one line —
`Opening docs in your browser` — and exits 0, returning to the prompt.
The terminal is *not* the witness here. Second, your default browser
opens a new tab pointing at the local doc index. That tab is the actual
witness.

You can confirm the docs really are on local disk before you ever run
the command. `rustup doc --path` prints the file path that the bare
`rustup doc` would resolve to, without opening anything:

```console
$ rustup doc --path
/Users/eli/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/index.html
```

That path exists as a regular HTML file (`ls` finds it; its
`<title>` is `Rust Documentation`), and the directory next to it has
subdirectories named `book/`, `reference/`, `std/`, `cargo/`, plus
several others. The local copy is a real folder of HTML files, not a
hidden network call.

Once the browser tab is open, you do not need to do anything else
today. The page lists the available doc sets. Click the Book link to
re-read Chapter 1; click *Standard Library* to look up a stdlib type;
click *Reference* for the language spec.

## What Changed

- `rustup doc` opens the local Rust documentation in your default
  browser. From any directory, no project required. (Book Ch1-1 line
  133 names this verbatim.)
- The local doc set was installed by `rustup` alongside `rustc` and
  `cargo` — it is not a separately-fetched download. It is available
  offline. (Book Ch1-1 line 132: "The installation of Rust also
  includes a local copy of the documentation so that you can read it
  offline.")
- The doc set includes at least the Book, the Reference, and the
  standard library API docs. The Book frames the API docs as the
  go-to lookup for stdlib types and functions: "Any time a type or
  function is provided by the standard library and you're not sure
  what it does or how to use it, use the application programming
  interface (API) documentation to find out!" (Ch1-1 lines 136-138.)
- `rustup doc` prints one short line of confirmation
  (`Opening docs in your browser`) and exits 0; the actual evidence
  the command worked is the new browser tab, not the terminal output.

## Check Yourself

(a) Which command opens the local Rust documentation in your default
browser?

(b) Does `rustup doc` need an internet connection?

(c) Name one kind of question the local docs are designed to answer.

(d) After `rustup doc` returns to the prompt with one line of output,
where is the actual confirmation that the command worked?

(Answers: (a) `rustup doc`. (b) No — the docs are installed locally,
alongside `rustc` and `cargo`. (c) Sample answers: "what methods does
`String` have?", "what does the Book say about `match`?", "how does
`Vec::push` behave?" — the Book frames API docs as the lookup for
stdlib types and functions. (d) In the new browser tab — the terminal
just prints `Opening docs in your browser` and exits.)

## What To Ignore For Now

- `rustup doc --book` / `--reference` / `--std` / `--cargo` and the
  rest of the flag list. These open one specific doc set instead of
  the index. Today only installs the bare command.
- `rustup doc <topic>` (e.g. `rustup doc usize`, `rustup doc std::fs`).
  Topic-aware lookup. Useful, but a separate move.
- The exact on-disk path of the local docs (rustup-managed; varies by
  toolchain channel and host). `rustup doc --path` prints it, but you
  do not need to memorize the path to use the command.
- `cargo doc` and `cargo doc --open` — these build and open the
  documentation for *your current Cargo project's source code*
  (different command, different scope; Ch14-2 *Publishing to
  Crates.io* territory).
- Doc comments (`///`) and `cargo test --doc`. Ch14-2 territory.
- The structure of the rustdoc-generated HTML, the in-page search box,
  and `rustdoc` directly as a tool.

## Evidence

See `../evidence/086-rustup-doc.md`.
