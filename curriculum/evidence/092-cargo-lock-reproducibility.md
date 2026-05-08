# Evidence — 092-cargo-lock-reproducibility

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `cargo --version` -> `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  The committed observation directory at
  `experimental/eduratchet2/runs/rust-moves/observations/092-cargo-lock-reproducibility/`
  reproduces the no-deps probe's package layout (`Cargo.toml`,
  `src/main.rs`, `.gitignore`); `Cargo.lock` and `target/` are
  deliberately gitignored, matching the convention used by lesson
  082's `cargo build --release` observation.

Same host and toolchain as recent accepted lessons (082-091).

## Sources

### `output/docs/rust/book/ch01-03-hello-cargo.md`

The Book's *Building and Running a Cargo Project* subsection. Lines
139-143 are the load-bearing introduction of `Cargo.lock`:

> If all goes well, `Hello, world!` should print to the terminal.
> Running `cargo build` for the first time also causes Cargo to
> create a new file at the top level: *Cargo.lock*. This file
> keeps track of the exact versions of dependencies in your
> project. This project doesn't have dependencies, so the file is
> a bit sparse. You won't ever need to change this file manually;
> Cargo manages its contents for you.

Direct corpus warrant for the lesson's centered claims:

- *First `cargo build` creates `Cargo.lock` at the package root*:
  "Running `cargo build` for the first time also causes Cargo to
  create a new file at the top level: *Cargo.lock*." Witnessed by
  Probe 1 below.
- *The lockfile records exact dependency versions*: "This file
  keeps track of the exact versions of dependencies in your
  project."
- *Even a no-dependency package gets a sparse lockfile*: "This
  project doesn't have dependencies, so the file is a bit sparse."
  Witnessed by Probe 1's three-line `[[package]]` block.
- *Cargo-managed; do not edit by hand*: "You won't ever need to
  change this file manually; Cargo manages its contents for you."
  Reinforced by the lockfile's own header in Probe 1.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book's *Ensuring Reproducible Builds* subsection. Lines 587-606
are the load-bearing reproducibility argument:

> #### [Ensuring Reproducible Builds](#ensuring-reproducible-builds)
>
> Cargo has a mechanism that ensures that you can rebuild the same
> artifact every time you or anyone else builds your code: Cargo
> will use only the versions of the dependencies you specified
> until you indicate otherwise. For example, say that next week
> version 0.8.6 of the `rand` crate comes out, and that version
> contains an important bug fix, but it also contains a regression
> that will break your code. To handle this, Rust creates the
> *Cargo.lock* file the first time you run `cargo build`, so we
> now have this in the *guessing_game* directory.
>
> When you build a project for the first time, Cargo figures out
> all the versions of the dependencies that fit the criteria and
> then writes them to the *Cargo.lock* file. When you build your
> project in the future, Cargo will see that the *Cargo.lock* file
> exists and will use the versions specified there rather than
> doing all the work of figuring out versions again. This lets you
> have a reproducible build automatically. In other words, your
> project will remain at 0.8.5 until you explicitly upgrade,
> thanks to the *Cargo.lock* file. Because the *Cargo.lock* file
> is important for reproducible builds, it's often checked into
> source control with the rest of the code in your project.

Direct corpus warrant for the lesson's centered claims:

- *Cargo reuses the recorded versions on the next build, even if
  newer matching versions exist*: "Cargo will see that the
  *Cargo.lock* file exists and will use the versions specified
  there rather than doing all the work of figuring out versions
  again." Plus the bug-fix-with-regression scenario.
- *That's the reproducible build*: "This lets you have a
  reproducible build automatically."
- *The "remain at 0.8.5" framing in the lesson body*: "In other
  words, your project will remain at 0.8.5 until you explicitly
  upgrade, thanks to the *Cargo.lock* file."
- *Source control checkin for binary applications*: "Because the
  *Cargo.lock* file is important for reproducible builds, it's
  often checked into source control with the rest of the code in
  your project." The lesson narrows this to binary applications,
  the only kind of package the curriculum has so far scaffolded
  via `cargo new`; the library-vs-binary distinction is moved to
  *What To Ignore For Now*.

The Book's verbatim "first time you run `cargo build`" and the
parallel sentence in Ch1-3 lines 139-143 jointly ground the lesson's
claim that the *first* build is the lockfile-creation build. The
claim that `cargo run` and `cargo check` also create the lockfile is
not separately quoted in the Book at this point, but follows from
the lesson-064/lesson-084 fact that those commands invoke the same
build pipeline; the lesson hedges by saying "the first time you run
`cargo build` (or `cargo run`, or `cargo check`)" — the build is
what creates the lockfile, regardless of which front-end command
triggered it.

## Probes

### Probe 1: no-dependency package — the sparse lockfile (canonical)

Goal: witness `Cargo.lock` does not exist before `cargo build`,
exists after, contains a sparse one-package body, is reused on the
no-edit second build, and contains a self-describing
"do-not-edit-manually" header.

```console
$ cargo new --vcs none lock_no_deps
    Creating binary (application) `lock_no_deps` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd lock_no_deps
$ ls
Cargo.toml
src
$ cargo build
   Compiling lock_no_deps v0.1.0 (/private/var/folders/.../lock_no_deps)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.67s
$ ls
Cargo.lock
Cargo.toml
src
target
$ cat Cargo.lock
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "lock_no_deps"
version = "0.1.0"
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```

Mapping to lesson body:

- The pre-build `ls` lacks `Cargo.lock`; the post-build `ls`
  contains it, alongside `target/` (lesson 064). Witnesses Ch1-3
  lines 139-143's "create a new file at the top level: *Cargo.lock*."
- The `cat Cargo.lock` output is exactly the Book's "a bit sparse"
  shape: comment header + `version = 4` schema marker + a single
  `[[package]]` entry naming the package itself, with no
  `[dependencies]` array because there are none.
- The lockfile's own header lines (`# This file is automatically
  @generated by Cargo.` / `# It is not intended for manual
  editing.`) witness the Book's "Cargo manages its contents for
  you" / "you won't ever need to change this file manually."
- The second `cargo build` produces the lesson-064 cache-hit shape
  (no `Compiling`, just `Finished`). The lockfile is reused
  silently — no `Updating crates.io index`, no rewrite. This is
  the simplest no-dependency witness of "Cargo will see that the
  *Cargo.lock* file exists and will use the versions specified
  there."

The committed observation directory at
`observations/092-cargo-lock-reproducibility/` mirrors the package
layout (`Cargo.toml`, `src/main.rs`, `.gitignore` matching lesson
082's). The probe is a fresh `cargo new` followed by `cargo build`
in a `mktemp -d`; the committed files document the shape, while
`Cargo.lock` and `target/` are gitignored (they regenerate
automatically).

### Probe 2: with-rand package — the populated lockfile (corroboration)

Goal: witness that with `rand = "0.8.5"` (lesson 065) listed under
`[dependencies]`, `Cargo.lock` is populated with one
`[[package]]` per resolved dependency, each carrying its exact
picked version. This is the Book's "your project will remain at
0.8.5 until you explicitly upgrade" scenario made concrete.

```console
$ cargo new --vcs none lock_with_rand
    Creating binary (application) `lock_with_rand` package
$ cd lock_with_rand
# Edit Cargo.toml: append rand = "0.8.5" under [dependencies]
$ cat Cargo.toml
[package]
name = "lock_with_rand"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
$ cargo build
    Updating crates.io index
     Locking 14 packages to latest Rust 1.95.0 compatible versions
      Adding rand v0.8.6 (available: v0.10.1)
   Compiling libc v0.2.186
   Compiling zerocopy v0.8.48
   Compiling cfg-if v1.0.4
   Compiling getrandom v0.2.17
   Compiling rand_core v0.6.4
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.6
   Compiling lock_with_rand v0.1.0 (/private/var/folders/.../lock_with_rand)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.65s
$ ls
Cargo.lock  Cargo.toml  src  target
$ head -40 Cargo.lock
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "cfg-if"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801"

[[package]]
name = "getrandom"
version = "0.2.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff2abc00be7fca6ebc474524697ae276ad847ad0a6b3faa4bcb027e9a4614ad0"
dependencies = [
 "cfg-if",
 "libc",
 "wasi",
]

[[package]]
name = "libc"
version = "0.2.186"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "68ab91017fe16c622486840e4c83c9a37afeff978bd239b5293d61ece587de66"

[[package]]
name = "lock_with_rand"
version = "0.1.0"
dependencies = [
 "rand",
]

[[package]]
name = "ppv-lite86"
version = "0.2.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85eae3c4ed2f50dcfe72643da4befc30deadb458a9b590d720cde2f2b1e97da9"
dependencies = [
$ wc -l Cargo.lock
     133 Cargo.lock
```

Mapping to lesson body:

- The 133-line `Cargo.lock` (vs Probe 1's 7-line file) shows
  "*records the exact resolved version of every dependency*" with
  full transitive closure: `rand` plus `rand_chacha`, `rand_core`,
  `getrandom`, `libc`, `cfg-if`, `ppv-lite86`, `zerocopy`, etc.
- The `version` field per `[[package]]` shows the *resolved*
  pick: `cfg-if` was pinned to exactly `1.0.4`, `getrandom` to
  `0.2.17`, etc. These are what Cargo will reuse on the next
  build, no matter what newer matching versions appear on
  crates.io.
- The same probe also reproduces lesson 065's transcript shape
  (`Updating crates.io index`, `Locking ... packages`,
  `Adding rand v0.8.6 (available: v0.10.1)` — Cargo picked the
  highest patch under the SemVer caret-range of `"0.8.5"`, which
  is the future-move detail explicitly deferred in lesson 065's
  *What To Ignore*).
- The `[[package]]` entry for `lock_with_rand` itself has a
  `dependencies = [ "rand" ]` array, witnessing that the lockfile
  also captures the *graph* of dependencies, not just versions.
  The lesson body keeps this implicit (`What To Ignore` defers
  schema details), but it is part of the reproducibility
  guarantee.

This probe is *not* committed as a separate package directory; the
transcript above is the artifact, and the recipe is reproducible
from lesson 065 + today's lockfile inspection step.

### Negative / contrast claim

The lesson's contrastive claim — *without `Cargo.lock`, the next
build might pick newer matching versions* — is grounded in the Book
text directly (lines 591-594 and 598-604) rather than a separate
probe. Forcing Cargo to "lose" the lockfile and re-resolve at a
later date is impractical for an in-cycle witness because no newer
matching `rand` patch is currently published vs. what Cargo just
picked; the reproducibility guarantee is a counterfactual claim
("if version 0.8.6 came out and contained a regression"). The Book
quotes that counterfactual verbatim, and Probe 2's "Adding rand
v0.8.6 (available: v0.10.1)" narration shows Cargo's resolver does
indeed look at multiple available versions before picking — making
the "next build might pick differently" mechanism plausible without
needing a time-machine probe.

The auxiliary "deleted-lockfile" demonstration (`rm Cargo.lock &&
cargo build`) would just regenerate the lockfile; it would not by
itself produce a different artifact today, because nothing on
crates.io has shifted the resolution. The probe is therefore
omitted as not load-bearing.

## Prerequisite Lesson Summaries (load-bearing)

- **064-cargo-build-standalone**: `cargo build` from inside a Cargo
  package directory compiles the package and stops, producing
  `target/debug/<name>`. The first invocation prints `Compiling
  <name> v0.1.0 (...)` then `Finished \`dev\` profile [...] in
  X.XXs`; a no-source-change rebuild prints only `Finished`.
  Today's lesson hinges on "the first `cargo build`" — that is
  exactly the lesson-064 invocation. The cache-hit second build
  shape is reused as evidence that `Cargo.lock` is silently reused
  across builds.

- **065-cargo-toml-dependencies-entry**: the `[dependencies]`
  section in `Cargo.toml` accepts entries of the form
  `<crate-name> = "<version-string>"` — concretely
  `rand = "0.8.5"`. The first `cargo build` after such an entry
  prints `Updating crates.io index` / `Locking N packages to
  latest Rust X.Y.Z compatible versions` / `Adding <crate>
  v<picked> (available: v<latest>)` and compiles transitive
  dependencies. Today's reproducibility argument is about *what
  Cargo records when it resolves those entries*. Lesson 065
  explicitly deferred `Cargo.lock`: "Lesson 032 deferred this;
  today just notice the file appears on the resolving build.
  Reproducible-build semantics are a future move." Today is that
  future move.

## Older supporting lessons (cited)

- **032-cargo-new-and-run**: the `cargo new <name>` scaffold and
  `Cargo.toml` shape (`[package]` + empty `[dependencies]`).
  Cited; specific claims are restated in lesson 064/065's
  prerequisite slot.
- **002-fn-main-entry-point**, **011-println-positional-args**:
  the default `Hello, world!` program produced by `cargo new`
  remains the source we build today. Cited only.
