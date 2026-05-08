# Evidence — 086-rustup-doc

Audit appendix for `lessons/086-rustup-doc.md`. The lesson installs one
toolchain-housekeeping command — `rustup doc` — that opens the local
Rust documentation set in the default web browser. This is closure
item N of the Ch1-3 closure queue; `cargo doc` / `cargo doc --open`
(project-scoped, Ch14-2 territory) is named in *What To Ignore* and
explicitly out of scope.

This appendix covers (a) toolchain and host context, (b) verbatim
probe transcripts (`rustup doc`, `rustup doc --path`, on-disk file
existence and HTML title checks, plus a sibling `rustup doc --book`
contrast), (c) corpus quote map, (d) prerequisite-claim summary, and
(e) a note on contrast probes and how the lesson handles the
"command opens a browser" probe-witness problem.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
rustup 1.29.0 (28d1352db 2026-03-05)
host: x86_64-apple-darwin
```

`uname -sm` -> `Darwin x86_64`. The probes ran from `/tmp` (a non-
package directory with no `.rs` files), confirming the lesson's "from
any directory" claim.

## Project setup

There is no project. `rustup doc` operates on the rustup-installed
toolchain documentation, not on any source file or Cargo package.
No `observations/086-rustup-doc/` package directory is committed —
same pattern as lesson 085, for the same reason.

## Probe transcripts

All transcripts captured on `2026-05-07` from a non-package directory.

### Step 1: `rustup doc` — the bare command

```text
$ cd /tmp && BROWSER=true rustup doc 2>&1
Opening docs in your browser
exit=0
```

Load-bearing observations:

- The command prints exactly one line of output —
  `Opening docs in your browser` — and exits 0.
- The probe was run with `BROWSER=true` to capture stdout/stderr
  without actually opening a browser tab. `BROWSER=true` is the
  standard POSIX trick: many "open URL" tools consult the `BROWSER`
  environment variable, and pointing it at `/usr/bin/true` makes the
  tool launch a no-op program in place of the browser. This does not
  change rustup's own one-line output. (The lesson body does not
  describe this trick — it is purely an instrumentation detail for
  this audit.)
- Without `BROWSER=true`, the same invocation also opens a new tab in
  the host's default browser pointed at the local docs index. The
  lesson body names the browser tab as the actual probe witness, and
  the terminal output as a confirmation line.

Grounds the lesson's *Try It* literal transcript and *What Changed*
bullet 4 ("`rustup doc` prints one short line of confirmation
(`Opening docs in your browser`) and exits 0").

### Step 2: `rustup doc --path` — what path does it resolve to?

```text
$ cd /tmp && rustup doc --path
/Users/eli/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/index.html
exit=0
```

Load-bearing observations:

- The bare `rustup doc` resolves to the file
  `/Users/eli/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/index.html`
  on this host. The path is under `~/.rustup/toolchains/<channel>/`,
  which is the rustup-installed toolchain location named by lesson
  085. This grounds the lesson's claim that the docs were installed
  *by* rustup alongside `rustc` and `cargo`.
- `--path` prints the path without opening anything. The lesson body
  uses this as the offline-witness probe: even before the learner
  runs `rustup doc`, they can confirm the docs are on disk.
- The path is host-specific (varies by toolchain channel and host).
  The lesson body names this in *What To Ignore* and does not require
  the learner to memorize the path.

Grounds the lesson's *Try It* `rustup doc --path` paragraph and the
*Prerequisites* "the local Rust documentation is HTML at a path on
disk" sentence.

### Step 3: file existence and structure on disk

```text
$ ls -la /Users/eli/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/index.html
-rw-r--r--  1 eli  staff  10730 May  6 04:02 .../html/index.html
exit=0

$ grep -o "<title>[^<]*</title>" .../html/index.html | head -1
<title>Rust Documentation</title>

$ ls .../html/ (selected entries)
alloc      book       cargo      clippy
core       edition-guide  embedded-book
error_codes  reference  std        rustc      rustdoc
test       style-guide    unstable-book
... (plus many sibling .html files: error-index.html,
guide-error-handling.html, tutorial.html, etc.)
```

Load-bearing observations:

- The index file `index.html` exists as a regular HTML file (10730
  bytes, owner `eli`, mtime `May 6 04:02`). It is not a synthesized
  redirect or a network call; `ls` finds it, `grep` reads it.
- Its HTML `<title>` is `Rust Documentation` — i.e., the page that
  opens really is the Rust documentation index.
- Sibling subdirectories `book/`, `reference/`, `std/`, `cargo/`,
  `clippy/`, etc. correspond to the doc sets named by `rustup doc
  --help` (`--book`, `--reference`, `--std`, `--cargo`, `--clippy`,
  ...). The lesson body names three of them (Book, Reference,
  standard library API docs) and stops there. The full enumeration is
  not load-bearing.

Grounds the lesson body's "the local copy is a real folder of HTML
files, not a hidden network call" sentence and the "subdirectories
named `book/`, `reference/`, `std/`, `cargo/`, plus several others"
sentence.

### Step 4: doc-set index files exist

```text
$ ls .../html/book/index.html .../html/std/index.html .../html/reference/index.html
.../html/book/index.html
.../html/reference/index.html
.../html/std/index.html
exit=0

$ grep -o "<title>[^<]*</title>" .../html/book/index.html | head -1
<title>The Rust Programming Language - The Rust Programming Language</title>

$ grep -o "<title>[^<]*</title>" .../html/std/index.html | head -1
<title>std - Rust</title>

$ grep -o "<title>[^<]*</title>" .../html/reference/index.html | head -1
<title>Introduction - The Rust Reference</title>
```

Load-bearing observations:

- Each of the three doc sets the lesson names by name (Book,
  Reference, standard library API docs) has a real `index.html` on
  disk under `~/.rustup/toolchains/<channel>/share/doc/rust/html/`.
- Each title confirms the doc set's identity: "The Rust Programming
  Language" (the Book), "std - Rust" (the standard library API
  docs), "Introduction - The Rust Reference" (the Reference).
- This grounds the lesson's "The set includes the Book, the Reference,
  and the standard library API docs" claim with three direct file-
  level witnesses, not just a hand-wave at the doc set list.

Grounds the lesson's *What Changed* bullet 3 and the *Mental Model
Delta* "After" paragraph.

### Step 5 (sanity, contrast): `rustup doc --book`

```text
$ cd /tmp && BROWSER=true rustup doc --book 2>&1
Opening docs named `book` in your browser
exit=0

$ cd /tmp && rustup doc --path --book
/Users/eli/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/index.html
exit=0
```

Load-bearing observations:

- `rustup doc --book` prints a *different* one-line message that
  names the doc set (`Opening docs named \`book\` in your browser`)
  and resolves to a *different* path
  (`.../html/book/index.html`, not the bare `.../html/index.html`).
- This is empirical confirmation of the *contrast* the lesson body
  names: the bare `rustup doc` opens the index; `rustup doc --book`
  opens the Book specifically. The lesson defers `--book` to *What
  To Ignore* but uses the contrast implicitly to ground the "the
  bare command opens the index" half of the claim.
- The `--book` flag is one of ~20 doc-set flags listed by
  `rustup doc --help` (output captured separately during this audit
  for completeness, not reproduced here in full).

Grounds the lesson's *What To Ignore* bullet on `--book` /
`--reference` / `--std` / `--cargo` and the lesson's centering of
the bare command (no flag, opens the index).

### Step 6 (sanity, not load-bearing): `rustup doc --help` excerpt

```text
$ rustup doc --help 2>&1 | head -3
Open the documentation for the current toolchain

Usage: rustup[EXE] doc [OPTIONS] [TOPIC]
```

Load-bearing observations:

- The help text's one-sentence summary is "Open the documentation for
  the current toolchain." The lesson body's *The Move* and *What
  Changed* bullet 1 paraphrase this as "opens the local Rust
  documentation in your default browser" — adding "default" and
  "browser" from the Book line 133 ("Run `rustup doc` to open the
  local documentation in your browser"). The two phrasings agree.
- The help text confirms `[TOPIC]` is a positional argument
  (`rustup doc usize`, `rustup doc std::fs`); the lesson defers this
  to *What To Ignore*.

Documented for completeness. The lesson does not cite the help text
directly — the Book sentence is the canonical pedagogical source.

## Corpus quote map

### `output/docs/rust/book/ch01-01-installation.md`

The Book's "Installation" chapter, *Reading the Local Documentation*
section. This is the only Ch1 mention of any docs command and is the
sole canonical pedagogical source for today's move.

**Lines 130-138** (load-bearing — the entire section):

> ### [Reading the Local Documentation](#reading-the-local-documentation)
>
> The installation of Rust also includes a local copy of the documentation so
> that you can read it offline. Run `rustup doc` to open the local documentation
> in your browser.
>
> Any time a type or function is provided by the standard library and you're not
> sure what it does or how to use it, use the application programming interface
> (API) documentation to find out!

This grounds five lesson sentences:

- "The installation of Rust also includes a local copy of the
  documentation so that you can read it offline" -> *What Changed*
  bullet 2 (verbatim quote) and the *Mental Model Delta* "After"
  paragraph's "Rust ships a copy of its own documentation locally —
  installed alongside `rustc` and `cargo` by `rustup`".
- "Run `rustup doc` to open the local documentation in your browser"
  -> *The Move* bullet, *What Changed* bullet 1 (paraphrased with the
  word "default" added; the lesson's "default browser" framing is
  ordinary computer-use vocabulary appropriate for the audience), and
  *Try It*'s literal transcript.
- "Any time a type or function is provided by the standard library
  and you're not sure what it does or how to use it, use the
  application programming interface (API) documentation to find out!"
  -> *What Changed* bullet 3 (verbatim quote) and the *Check
  Yourself* (c) sample answers ("what methods does `String` have?",
  "how does `Vec::push` behave?").
- "read it offline" -> *Prerequisites* "no network needed" and
  *Check Yourself* (b) ("Does `rustup doc` need an internet
  connection? No — the docs are installed locally.").
- "the local documentation in your browser" (singular "documentation")
  -> the lesson body's framing of the *index* page that lists the doc
  sets, rather than any single doc set. The Book itself does not
  enumerate the doc sets; the lesson names the three most prominent
  (Book, Reference, std) grounded by Step 4 above (direct file-system
  witnesses), and stops there.

### Adjacency in the Book

The lesson 085 grounding (Book Ch1-1 lines 27-41 introducing rustup,
lines 112-119 introducing `rustup update`, lines 121-125 naming
`rustup self uninstall`) sits in the same chapter as today's section
(lines 130-138). The Book treats rustup as a small set of related
subcommands, and today's `rustup doc` is one of them. This grounds
the lesson's framing of `rustup doc` as a *sibling* of `rustup update`
in the same vocabulary — the lesson body says "extends 085's
`rustup update` ... with a sibling subcommand `rustup doc`" and
points at the same chapter.

## Prerequisite-claim summary

### From lesson 085 (`085-toolchain-housekeeping`) — *direct, load-bearing*

- `rustup` is the program that installed Rust on this host and
  manages the toolchain. Lesson 085 establishes this with Book Ch1-1
  lines 27-41 ("the `rustup` tool, which installs the latest stable
  version of Rust") and probes `rustup update` exit 0 from any
  directory. Today reuses both: `rustup doc` is a sibling subcommand,
  and the docs it opens were installed by the same rustup that
  installed `rustc` and `cargo`.
- `rustup` is on `PATH`. Lesson 085's `rustup update` succeeded as a
  bare command from any directory; today's `rustup doc` succeeds the
  same way for the same reason.
- `rustup` ships subcommands that *do not* touch any project or
  source file. Lesson 085 installed `rustup update` as a no-project,
  no-`.rs`-file housekeeping command; today's `rustup doc` is the
  same shape.

### From lesson 001 (`001-rustc-compile-and-run`) — *cited, supporting*

- Shell prompt; typing a command and reading its output. Today's
  probes are all "type a command, read the printed output, exit code
  0" — same setup as cycle 001.

### Ordinary computer-use assumptions (named in the lesson body)

- The host has a default web browser configured. The lesson body
  names this as the same kind of ordinary computer-use fact as "the
  host has a terminal". `rustup doc` consults the platform's default
  browser mechanism; on macOS this is whichever app handles `http://`
  / `file://` URLs (Safari out of the box, or whatever the user has
  set). The lesson does not depend on which browser opens.
- The local Rust documentation is HTML at a path on disk under the
  rustup-managed toolchain directory, and `rustup doc` resolves to a
  `file://` URL and hands it to the default browser. The lesson names
  this once and does not center the `file://` vs `http://` distinction
  as a concept. Step 2 (`rustup doc --path`) and Step 3 (file exists
  on disk) ground this jointly.

## Contrast-probe coverage

The lesson's central contrast is *the bare command opens the index*
vs *a flag opens one specific doc set*. Step 5 above is the explicit
contrast probe: `rustup doc` resolves to `.../html/index.html` and
prints `Opening docs in your browser`; `rustup doc --book` resolves
to `.../html/book/index.html` and prints `Opening docs named \`book\`
in your browser`. The lesson defers `--book` to *What To Ignore* but
uses this contrast implicitly to motivate the "bare command opens the
index" centering.

There is no working-vs-broken contrast probe. The intentional break
cases (no rustup on `PATH`, no default browser, docs missing on disk)
are not failure modes the lesson can responsibly probe — they would
require breaking the host install or unsetting OS-level configuration.
This matches lesson 085's contrast-coverage decision (toolchain
housekeeping commands succeed by design on a healthy install) and is
declared explicitly in the lesson's *Mental Model Delta* and
*Prerequisites* without a destructive probe.

## A note on the "browser tab as witness" problem

`rustup doc` is unusual among lesson-085-style toolchain commands in
that the *most important* effect — a new browser tab — is hard to
capture in a terminal transcript. The lesson handles this in three
ways:

1. **Capture what the terminal does emit.** The bare command prints
   exactly one line — `Opening docs in your browser` — and exits 0
   (Step 1). The lesson body shows this transcript verbatim and
   names it explicitly as a *confirmation* line, not as the witness.
2. **Substitute a path probe.** `rustup doc --path` prints the
   resolved file path without opening anything (Step 2). This grounds
   "the docs are on local disk" without depending on the browser.
3. **Substitute a file-existence probe.** `ls` finds the resolved
   file as a regular HTML file with a recognizable `<title>`, and
   the doc-set subdirectories (`book/`, `std/`, `reference/`, ...)
   exist with their own `<title>`s (Steps 3 and 4). This grounds
   "the local copy is a real folder of HTML files".

The lesson body names the browser tab as the witness ("The browser
tab is the actual witness") and uses the path/file-existence probes
as the auditable substitute — the learner can run `rustup doc --path`
and `ls` to confirm the docs exist before they ever run the
browser-opening form. This is the same auditing strategy lesson 085
used for `rustup update`'s `unchanged` outcome (the network branch
was named-not-probed; the lesson stayed grounded by what the local
toolchain reported).

## What is *not* in this probe

The probe deliberately does not exercise:

- `rustup doc --book` / `--reference` / `--std` / `--cargo` /
  `--clippy` / etc. as centered commands. Step 5 used `--book` only
  to ground the bare-vs-flag contrast; the lesson defers the full
  flag list to *What To Ignore*.
- `rustup doc <topic>` (e.g. `rustup doc usize`, `rustup doc std::fs`).
  Topic-aware lookup; named as deferred in *What To Ignore*.
- `cargo doc` and `cargo doc --open`. These build documentation from
  *the current Cargo project's source code* and are a different scope
  (project-scoped vs system-scoped). Ch14-2 *Publishing to Crates.io*
  territory; named as deferred in *What To Ignore* as a separate
  future move.
- Doc comments (`///`) and `cargo test --doc`. Ch14-2 territory.
- The internal structure of the rustdoc-generated HTML, the in-page
  search box, and `rustdoc` directly as a tool.
- The full content of `rustup doc --help`. Step 6 captured only the
  one-line summary; the help text is not load-bearing.
- Cross-host path differences. The probe captured the macOS path
  (`/Users/eli/.rustup/...`); the Linux path
  (`$HOME/.rustup/...`) and the Windows path
  (`%USERPROFILE%\.rustup\...`) are not centered concepts. The
  lesson body says "varies by toolchain channel and host" and stops
  there.
- Browser identity. The probe ran with `BROWSER=true` to suppress
  the actual launch; the lesson body says "default browser" without
  naming Safari/Chrome/Firefox specifically.

## Files committed for this cycle

- `lessons/086-rustup-doc.md` (this lesson)
- `evidence/086-rustup-doc.md` (this appendix)
- updated `graph.md` (a new draft node block under `## Draft Nodes`)

There is no `observations/086-rustup-doc/` directory: the lesson has
nothing to put in one. `rustup doc` operates on the rustup-installed
documentation, not on any source file or package, and the verbatim
transcripts above are the complete observation record — same pattern
as lesson 085.
