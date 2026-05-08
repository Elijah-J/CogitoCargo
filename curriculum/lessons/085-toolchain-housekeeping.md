---
id: 085-toolchain-housekeeping
status: accepted
evidence: ../evidence/085-toolchain-housekeeping.md
---

# Toolchain housekeeping: `rustc --version`, `cargo --version`, `rustup update`

## The Move

Three small commands that work on *the toolchain itself*, not on any
project or source file. From any directory in any shell, run:

- `rustc --version` — prints the installed compiler's version.
- `cargo --version` — prints the installed Cargo's version.
- `rustup update` — re-runs `rustup` (the tool that installed Rust) to
  refresh your toolchain to the latest stable release.

These are toolchain housekeeping. The first two answer *what version do
I have right now?*. The third answers *make me current.* No `.rs` file,
no `Cargo.toml`, no `cd` — they run anywhere.

## Mental Model Delta

- *Before:* "I run `rustc file.rs` (lesson 001) or `cargo build`
  (lesson 064) to compile code. I have no command for inspecting which
  *version* of the toolchain I am running, or for *updating* it."
- *After:* "Three small commands manage the toolchain itself.
  `rustc --version` prints the compiler version (with a commit hash
  and date). `cargo --version` prints Cargo's version in the same
  shape. `rustup update` refreshes Rust to the latest stable release.
  All three run from any shell — independent of any project, file, or
  directory. Use them to confirm a working install, to record which
  version produced a build, or to refresh to the latest stable."

## Prerequisites

- Installed concepts:
  - Lesson 001 (`001-rustc-compile-and-run`, load-bearing): `rustc` is
    on `PATH`; you can type a command at the shell prompt and read its
    output. Today reuses these without re-installing them.
  - Lesson 064 (`064-cargo-build-standalone`, cited): `cargo` is on
    `PATH` (the lesson uses `cargo new --vcs none` from any directory,
    so Cargo is reachable as a plain command).
- Ordinary computer-use assumptions: terminal/shell prompt; no editor
  needed; no network unless `rustup update` actually finds new bits to
  download. One additional fact named once and not re-installed:
  `rustup` is the official tool that installed Rust on this host (Book
  Ch1-1 *Installing rustup on Linux or macOS*, lines 27-41); it sits
  alongside `rustc` and `cargo` and is the program that fetches and
  swaps in newer toolchains.

## Try It

Open any terminal in any directory. Run the version commands first:

```console
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)

$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

Read the shape of `rustc --version`'s line. Past the leading word
`rustc`, three pieces of information follow, in this order:

1. *Version number* — `1.95.0`. Three integers separated by dots:
   major, minor, patch.
2. *Commit hash* — `59807616e`, in parentheses. The git commit of the
   rustc source that produced this binary.
3. *Commit date* — `2026-04-14`, also in parentheses. The date of that
   commit, in `yyyy-mm-dd` form.

The Book gives this exact format: `rustc x.y.z (abcabcabc yyyy-mm-dd)`
(Ch1-1 *Troubleshooting*, line 83).

`cargo --version` prints the same shape: word, version, parenthesized
commit hash and date. Cargo's version moves in step with rustc's, so
on a healthy install both lines start `1.95.0` (or whatever your
current stable is). The Book says "If you see a version number, you
have it!" (Ch1-3, line 27) — these commands are also a quick health
check that the tools are actually on `PATH`.

Now run the update command:

```console
$ rustup update
info: syncing channel updates for stable-x86_64-apple-darwin

  stable-x86_64-apple-darwin unchanged - rustc 1.95.0 (59807616e 2026-04-14)

info: checking for self-update (current version: 1.29.0)
info: cleaning up downloads & tmp directories
```

What you see depends on whether your stable toolchain is current.
On the host where this lesson was written, the current channel was
already up to date, so `rustup` reported `unchanged` and named the
exact rustc build that is already installed. If a newer stable
existed, `rustup` would have *downloaded* it and reported the new
version on the same line. Either way, the command is non-destructive:
it adds or refreshes the stable toolchain, no project is touched, and
your existing source files are untouched.

After `rustup update`, re-running `rustc --version` and
`cargo --version` will report the post-update versions — same format,
possibly bumped numbers.

## What Changed

- `rustc --version` prints the compiler's version line in the format
  `rustc x.y.z (abcabcabc yyyy-mm-dd)` — version, commit hash, commit
  date (Book Ch1-1 *Troubleshooting*).
- `cargo --version` prints Cargo's version line in the same shape.
  The Book uses it as the "do I have Cargo on `PATH`?" check
  (Ch1-3, lines 16-28).
- `rustup update` re-runs `rustup` to refresh the stable toolchain.
  If a newer stable exists, it downloads and installs it; if you are
  already current, it reports `unchanged`. Run from any shell (Book
  Ch1-1 *Updating and Uninstalling*, lines 112-119).
- Three commands, one role: toolchain housekeeping. They run anywhere,
  touch no project, and let you both *inspect* the installed Rust and
  *refresh* it.

## Check Yourself

(a) Which command tells you which version of `rustc` is installed on
this machine?

(b) The output of `rustc --version` includes three pieces of
information after the leading word `rustc`. What are they, in order?

(c) Which command refreshes Rust to the latest stable release?

(d) Do any of these three commands need you to be inside a Cargo
package, or to have a `.rs` file?

(Answers: (a) `rustc --version`. (b) Version number, commit hash, and
commit date — the format `x.y.z (abcabcabc yyyy-mm-dd)`. (c)
`rustup update`. (d) No — they are toolchain housekeeping; they run
from any directory and operate on the installed Rust tools, not on a
project.)

## What To Ignore For Now

- `rustup self uninstall`. The Book pairs it with `rustup update`
  (Ch1-1 lines 121-125); administrative, out of scope here.
- Toolchain channels (`stable`, `beta`, `nightly`) and
  `rustup toolchain install` / `rustup default <channel>`. Today
  refreshes whatever channel is already the default.
- `rustup component add <name>` (e.g. `rustfmt`, `clippy`) and
  component management generally. Future move.
- `rustup override set <toolchain>` and per-directory toolchain
  pinning, `rust-toolchain.toml` files. Future move.
- The shorthand `rustc -V` (single uppercase `V`). It exists and works,
  but the Book uses `--version`; one spelling is enough today.
- Cross-compilation targets (`rustup target add ...`). Out of scope.
- The Linux/macOS install script (`curl ... | sh`) and the Windows
  installer. Out of scope per the queue (host-OS sprawl).

## Evidence

See `../evidence/085-toolchain-housekeeping.md`.
