# Evidence — 085-toolchain-housekeeping

Audit appendix for `lessons/085-toolchain-housekeeping.md`. The lesson
co-installs three toolchain housekeeping commands as one operational
concept: `rustc --version` (verify the compiler), `cargo --version`
(verify Cargo), and `rustup update` (refresh the stable toolchain).
The lesson is closure item M of the Ch1-3 queue; `rustup self uninstall`
(Book lines 121-125) is named in *What To Ignore* and explicitly out of
scope.

This appendix covers (a) toolchain and reproducibility, (b) verbatim
probe transcripts for all three commands plus the `rustc -V` shorthand
sanity-check, (c) corpus quote map, (d) prerequisite-claim summary, and
(e) why the three commands are co-installed rather than split.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
rustup 1.29.0 (28d1352db 2026-03-05)
host: x86_64-apple-darwin
```

`uname -sm` -> `Darwin x86_64`. The probes ran from the worktree
working directory (`/Users/eli/.eduratchet-worktrees/eduratchet-2`),
but the lesson's claim is that these commands are independent of the
working directory; the `rustup update` transcript was also re-confirmed
from `/tmp` (no source files present) with identical output.

## Project setup

There is no project. These commands operate on the installed toolchain,
not on any source file. No `observations/085-toolchain-housekeeping/`
package directory is committed because the lesson has nothing to put
in one — running `cargo new` would obscure the central pedagogical
claim that today's three commands need *no* project.

## Probe transcripts

All transcripts captured on `2026-05-07` from a non-package directory.

### Step 1: `rustc --version`

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
exit=0
```

Load-bearing observations:

- The output has the exact shape Book Ch1-1 line 83 names:
  `rustc x.y.z (abcabcabc yyyy-mm-dd)`. The leading word is `rustc`;
  the three pieces after it, in order, are version number `1.95.0`,
  commit hash `59807616e`, and commit date `2026-04-14`.
- Exit status 0; no source file consulted; no executable produced.

The lesson body's "Try It" reproduces this transcript verbatim and
the "What Changed" bullet 1 is grounded by the Book's format claim
plus this observation.

### Step 2: `cargo --version`

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
exit=0
```

Load-bearing observations:

- Same shape as `rustc --version`: a leading word, then the version
  number `1.95.0`, then a parenthesized commit hash `f2d3ce0bd` and
  commit date `2026-03-21`.
- The Book at Ch1-3 line 27 says "If you see a version number, you
  have it!" The lesson treats this command as both a version-print
  *and* a presence-on-`PATH` check.
- Cargo's `1.95.0` matches rustc's `1.95.0`. The Book's premise that
  "Cargo comes installed with Rust if you used the official
  installers" (Ch1-3 lines 16-19) is consistent with two synchronized
  version numbers; the lesson body restates this as "Cargo's version
  moves in step with rustc's."

### Step 3: `rustup update`

```text
$ rustup update
info: syncing channel updates for stable-x86_64-apple-darwin

  stable-x86_64-apple-darwin unchanged - rustc 1.95.0 (59807616e 2026-04-14)

info: checking for self-update (current version: 1.29.0)
info: cleaning up downloads & tmp directories
exit=0
```

Load-bearing observations:

- The command runs and exits 0 with no flag and no argument — the
  Book's invocation (line 117) is exactly `$ rustup update`.
- The transcript reports `stable-x86_64-apple-darwin unchanged` and
  names the exact rustc build (`rustc 1.95.0 (59807616e 2026-04-14)`)
  that is already installed. If a newer stable existed at the time
  of running, `rustup` would have downloaded it and printed an
  updated version line in the same slot. The lesson body names both
  cases ("If a newer stable existed, `rustup` would have *downloaded*
  it and reported the new version on the same line").
- The `info: checking for self-update (current version: 1.29.0)` line
  reports rustup's own version and confirms the lesson's framing of
  rustup as a managed program. The lesson does *not* install
  `rustup --version` as a fourth centered command — it is named in
  *What To Ignore* implicitly via the broader `rustup` deferrals,
  and the rustup `1.29.0` version surfaces here only as part of the
  natural transcript.
- Probe re-run from `/tmp` (no source files) produced bit-identical
  output, confirming the "any directory" claim.

### Step 4 (sanity, not load-bearing): `rustup --version`

```text
$ rustup --version
rustup 1.29.0 (28d1352db 2026-03-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: the currently active `rustc` version is `rustc 1.95.0 (59807616e 2026-04-14)`
exit=0
```

Documented for completeness. The lesson does not include this command
in the centered three (the queue named exactly `rustc --version`,
`cargo --version`, and `rustup update`), but the output confirms the
"three pieces of info after the leading word" shape generalizes to
rustup's version line as well. The two `info:` trailers are noise the
lesson does not mention.

### Step 5 (sanity, not load-bearing): `rustc -V`

```text
$ rustc -V
rustc 1.95.0 (59807616e 2026-04-14)
exit=0
```

The single-uppercase-`V` shorthand prints the same line. The lesson
names this in *What To Ignore* ("It exists and works, but the Book
uses `--version`; one spelling is enough today.") and uses
`--version` consistently in the lesson body to match the Book's
canonical form. No claim depends on the shorthand.

## Corpus quote map

### `output/docs/rust/book/ch01-01-installation.md`

The Book's "Installation" chapter. Contains both the `rustc --version`
sanity-check (under *Troubleshooting*) and `rustup update` (under
*Updating and Uninstalling*). One file, two of today's three commands.

**Lines 70-84** (load-bearing — the `rustc --version` shape):

> ### [Troubleshooting](#troubleshooting)
>
> To check whether you have Rust installed correctly, open a shell
> and enter this line:
>
> ```console
> $ rustc --version
> ```
>
> You should see the version number, commit hash, and commit date for
> the latest stable version that has been released, in the following
> format:
>
> ```text
> rustc x.y.z (abcabcabc yyyy-mm-dd)
> ```

This grounds three lesson sentences:

- "`rustc --version` — prints the installed compiler's version" -> the
  *The Move* bullet 1 verb claim, the *What Changed* bullet 1.
- The three pieces "version number, commit hash, and commit date" ->
  the lesson's enumerated list under "Try It" (steps 1-3 of the read
  paragraph) and the *Check Yourself* (b) answer.
- The format string `rustc x.y.z (abcabcabc yyyy-mm-dd)` -> the
  lesson body's literal transcript shape and the *What Changed*
  bullet 1's parenthetical "version, commit hash, commit date".

The Book also frames the command as "to check whether you have Rust
installed correctly" — the lesson restates this as one of three uses
("to confirm a working install").

**Lines 86-87** (corroborating, not directly cited):

> If you see this information, you have installed Rust successfully!

Restated by the lesson body as the sanity-check framing. Not load-
bearing for any specific claim.

**Lines 112-119** (load-bearing — `rustup update`):

> ### [Updating and Uninstalling](#updating-and-uninstalling)
>
> Once Rust is installed via `rustup`, updating to a newly released
> version is easy. From your shell, run the following update script:
>
> ```console
> $ rustup update
> ```

This grounds:

- "`rustup update` — re-runs `rustup` (the tool that installed Rust)
  to refresh your toolchain to the latest stable release" -> the
  *The Move* bullet 3 verb claim and the *What Changed* bullet 3.
- "From your shell" -> the lesson body's "Run from any shell" framing.
- "Once Rust is installed via `rustup`" -> the prerequisite framing
  that rustup is the tool that originally did the install. The
  lesson states this as a one-line ordinary-computer-use named fact
  (rather than re-installing the install procedure).

**Lines 121-125** (named-but-deferred — `rustup self uninstall`):

> To uninstall Rust and `rustup`, run the following uninstall script
> from your shell:
>
> ```console
> $ rustup self uninstall
> ```

Named in the lesson's *What To Ignore* as out-of-scope per the Ch1-3
closure queue ("administrative; out of scope"). Documented here to
be explicit that the deferral is intentional, not an oversight.

**Lines 27-41** (corroborating — what `rustup` is):

> ### [Installing `rustup` on Linux or macOS](#installing-rustup-on-linux-or-macos)
>
> If you're using Linux or macOS, open a terminal and enter the
> following command:
>
> ```console
> $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
> ```
>
> The command downloads a script and starts the installation of the
> `rustup` tool, which installs the latest stable version of Rust.

The lesson's prerequisite paragraph names rustup as "the official tool
that installed Rust on this host" with a citation to this Book section.
The lesson does *not* re-install the install procedure (the
`curl ... | sh` line is out of scope per the queue). The corpus
sentence "the `rustup` tool, which installs the latest stable version
of Rust" grounds the lesson's "the program that fetches and swaps in
newer toolchains" framing.

### `output/docs/rust/book/ch01-03-hello-cargo.md`

The Book's "Hello, Cargo!" chapter. Contains the `cargo --version`
presence check.

**Lines 16-28** (load-bearing — `cargo --version` shape and use):

> Because the vast majority of Rust projects use Cargo, the rest of
> this book assumes that you're using Cargo too. Cargo comes installed
> with Rust if you used the official installers discussed in the
> ["Installation"](ch01-01-installation.md#installation) section. If
> you installed Rust through some other means, check whether Cargo is
> installed by entering the following in your terminal:
>
> ```console
> $ cargo --version
> ```
>
> If you see a version number, you have it! If you see an error, such
> as `command not found`, look at the documentation for your method of
> installation to determine how to install Cargo separately.

This grounds:

- "`cargo --version` — prints the installed Cargo's version" -> the
  *The Move* bullet 2 verb claim, the *What Changed* bullet 2.
- "If you see a version number, you have it!" -> the lesson body's
  "these commands are also a quick health check that the tools are
  actually on `PATH`" framing, restated literally from the Book.
- "Cargo comes installed with Rust if you used the official
  installers" -> the lesson body's premise that on a healthy install
  both `rustc` and `cargo` print the same version number.

The Book does not give an explicit format for `cargo --version` the
way it gives `rustc x.y.z (abcabcabc yyyy-mm-dd)` for rustc. The
lesson body's claim that `cargo --version` "prints the same shape" is
grounded by direct probe observation (step 2 above) — the literal
output `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` matches the rustc shape
character-class for character-class (leading word, version triple,
parenthesized hash and date).

## Prerequisite-claim summary

### From lesson 001 (`001-rustc-compile-and-run`) — *direct, load-bearing*

- `rustc` is on the host's `PATH`. Lesson 001's central probe
  (`rustc demo.rs`) succeeds, so plain `rustc` is reachable as a
  command. Today reuses this without re-installing it; today's
  `rustc --version` succeeds for the same reason `rustc demo.rs`
  did in cycle 001.
- The shell prompt and reading-text-on-stdout. Today reuses both;
  every probe today is "type a command, read the printed output,
  exit code 0."

### From lesson 064 (`064-cargo-build-standalone`) — *direct, supporting*

- `cargo` is on the host's `PATH`. Lesson 064 invokes plain
  `cargo new --vcs none` from any directory and `cargo build` from
  inside the resulting package; Cargo is therefore reachable as a
  bare command. Today's `cargo --version` succeeds for the same
  reason. Lesson 064 was the first cycle to use Cargo without an
  explicit install step, so naming it as the prerequisite for the
  `cargo --version` probe is the cleanest pointer.

### Ordinary computer-use assumptions (named once in the lesson body)

- `rustup` is the official tool that installed Rust on this host.
  This single fact appears in the lesson body, grounded by Book
  Ch1-1 lines 27-41. The lesson does not re-install the install
  procedure (the `curl ... | sh` line is out of scope per the
  queue's host-OS-sprawl deferral). The probe demonstrates that
  `rustup` is on `PATH` (`rustup update` succeeds, exit 0) without
  the lesson having to make `rustup` itself a centered concept
  beyond the one-line "is this tool here?" framing.

## Why co-install all three

The three commands are bundled into one cycle because:

1. **One operational role.** All three are "manage the installed
   Rust toolchain." None of them touches a `.rs` file, none needs a
   project, none compiles anything. The three together are the
   minimum vocabulary for "what version do I have?" + "make me
   current."

2. **One mental-model delta.** Splitting the cycle would force three
   small lessons each with the same Before/After ("I have no command
   for inspecting the toolchain..."), which is not three deltas.
   The single cycle installs the *category* once.

3. **Same shell context, no machinery growth.** All three are run
   from a bare shell with no flags (or in `rustc --version`'s case,
   one flag). No new editor, no new package, no new file. The
   cognitive load of three commands together is roughly that of one
   command — the syntax is identical (`<tool> <flag>` or
   `<tool> <subcommand>`).

4. **Book pairing.** Ch1-1's *Troubleshooting* and *Updating and
   Uninstalling* are separated by ~30 lines in one chapter, and
   `cargo --version` in Ch1-3 is a near-mirror of `rustc --version`.
   The Book itself treats these as a small interrelated set of
   sanity-checks and refresh commands, not as concepts that each
   warrant a chapter.

5. **`rustup update` is meaningful only against a `--version` baseline.**
   Without `rustc --version` and `cargo --version`, the learner has
   no way to recognize what `rustup update` *changed* (or did not
   change). The Book's framing is "check, then update" — the
   commands form one operational loop.

A future cycle could split off if any of them gained complexity
(e.g. `rustup toolchain install <channel>`, `rustup default beta`,
`rustup component add clippy`), but today's surface is small enough
that one cycle is right-sized.

## Contrast-probe coverage

This lesson does not have a working-vs-broken contrast in the way
lesson 080 has the `u8 = 256` literal-out-of-range probe or lesson 077
has the `i32`-index `E0277` probe. The reason: all three of today's
commands are sanity-check tools that succeed by design on a healthy
toolchain. The intentional "broken" cases (rust not installed, cargo
not on `PATH`, no network during `rustup update`) are not failure
modes the lesson can responsibly probe — the first two would require
breaking the host install, and the third is intermittent and outside
the lesson's scope.

The contrastive pieces the lesson does have are *non-destructive*:

- The two distinct uses of `rustc --version` (working install
  reports a version line) versus the *named* failure mode (`%PATH%`
  / `$PATH` does not include Rust — Book lines 86-105). The lesson
  body restates the Book's framing of `rustc --version` as a sanity
  check; the failure mode is documented in the Book and named in the
  lesson's *Mental Model Delta* without a destructive probe.
- The two outcomes of `rustup update` (channel `unchanged` versus a
  new version downloaded). The probe captured the `unchanged` case;
  the lesson body names the new-stable case as the same command's
  alternative outcome ("If a newer stable existed, `rustup` would
  have *downloaded* it"), grounded by the Book's "updating to a newly
  released version is easy" framing (line 114).

Both are restated as *rules the Book gives* rather than as
empirically-witnessed working/broken transitions. This matches the
lesson's pedagogical scope (toolchain housekeeping, not
troubleshooting).

## What is *not* in this probe

The probe deliberately does not exercise:

- `rustup self uninstall` (Book lines 121-125). Out of scope per the
  queue.
- `rustup toolchain install <channel>` or `rustup default <channel>`.
  Toolchain channel management is named only as deferred in *What To
  Ignore*.
- `rustup component add <name>`. Component management is named only
  as deferred in *What To Ignore*.
- `rustup target add <triple>`. Cross-compilation; named as deferred.
- The Linux/macOS install script (`curl ... | sh`) and the Windows
  installer. Out of scope per the queue.
- `rust-toolchain.toml` files and `rustup override set <toolchain>`.
  Per-directory toolchain pinning; named as deferred.
- The `rustc -V` shorthand as a centered command. Documented in step
  5 above for completeness; the lesson uses `rustc --version` to
  match the Book's canonical form.
- `cargo -V` / `cargo --help`. Cargo subcommand surface beyond
  `--version` is out of today's centered concept.
- The full content of `rustup --help` / `rustup update --help`. Help
  text is not load-bearing for the lesson.

## Files committed for this cycle

- `lessons/085-toolchain-housekeeping.md` (this lesson)
- `evidence/085-toolchain-housekeeping.md` (this appendix)
- updated `graph.md` (a new draft node block under `## Draft Nodes`)

There is no `observations/085-toolchain-housekeeping/` directory: the
lesson has nothing to put in one. All three commands operate on the
installed toolchain, not on any source file or package, and the
verbatim transcripts above are the complete observation record.
