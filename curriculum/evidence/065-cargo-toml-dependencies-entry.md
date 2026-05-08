# Evidence — 065-cargo-toml-dependencies-entry

Audit appendix for `lessons/065-cargo-toml-dependencies-entry.md`. The
lesson teaches one move: under `[dependencies]` in `Cargo.toml`, add a
line of the shape `<crate-name> = "<version-string>"`; on the next
`cargo build`, Cargo resolves the dependency from crates.io,
downloads and compiles the crate plus any of its own dependencies,
then compiles the package, and writes `Cargo.lock` next to
`Cargo.toml`.

This appendix covers (a) toolchain and reproducibility, (b) corpus
quote map, (c) the verbatim probe transcript, (d) the project setup
including the manifest edit, (e) the prerequisite-claim summary,
(f) calibration and contrast probes, and (g) honesty notes about
version drift between the Book corpus and the live registry.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-apple-darwin
release: 1.95.0
LLVM version: 22.1.2
```

`uname -sm` -> `Darwin x86_64`. The probe ran in a fresh
`mktemp -d` directory: `/private/var/folders/.../tmp.XPj3oNXDE6/`.
Network access to crates.io was available — the load-bearing first
build *requires* network reach.

## Project setup

The committed observation directory
`observations/065-cargo-toml-dependencies-entry/` contains exactly
the files that are load-bearing for the lesson, matching cycle 064's
shape:

- `Cargo.toml` — the **post-edit** manifest with the `rand = "0.8.5"`
  line under `[dependencies]`:

  ```toml
  [package]
  name = "hello_dep"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  rand = "0.8.5"
  ```

  The pre-edit manifest is the standard `cargo new --vcs none`
  output (already documented in cycle 064's evidence appendix).

- `src/main.rs` — **unchanged** from `cargo new`'s default. This is
  load-bearing: the lesson explicitly states that today's program
  does not import or call `rand`; the resolver run is the cycle, not
  using the crate.

  ```rust
  fn main() {
      println!("Hello, world!");
  }
  ```

- `.gitignore` — listing `target/` and `Cargo.lock`. Build artifacts;
  per orchestrator directive, the commit is exactly
  `Cargo.toml` + `src/main.rs` + `.gitignore`.

### The manifest edit (before/after)

The single load-bearing diff is one line added at the bottom of the
section, leaving the rest of the file untouched:

```diff
 [dependencies]
+rand = "0.8.5"
```

This is the literal Book line at `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`
line 492. Quoted in full in the corpus map below.

## Probe transcript

Steps 1-11 from the orchestrator's probe shape, run in
`/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.XPj3oNXDE6/`.

### Steps 1-2: Toolchain capture

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ rustc --version --verbose
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-apple-darwin
release: 1.95.0
LLVM version: 22.1.2
```

### Step 3: Fresh package

```text
$ cargo new --vcs none hello_dep
    Creating binary (application) `hello_dep` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

### Step 4-5: Default manifest

```text
$ cd hello_dep
$ cat Cargo.toml
[package]
name = "hello_dep"
version = "0.1.0"
edition = "2024"

[dependencies]
$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
```

Bit-identical to cycle 064's pre-edit state. `[dependencies]` is the
empty trailing section that cycles 032 and 064 named without filling.

### Steps 6-7: Manifest edit

`Cargo.toml` was edited to add one line under `[dependencies]`. Post-edit:

```text
$ cat Cargo.toml
[package]
name = "hello_dep"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

This matches the Book ch02 line 492 verbatim.

### Step 8: First `cargo build` — the load-bearing transcript

```text
$ cargo build
    Updating crates.io index
     Locking 14 packages to latest Rust 1.95.0 compatible versions
      Adding rand v0.8.6 (available: v0.10.1)
 Downloading crates ...
  Downloaded getrandom v0.2.17
  Downloaded rand_chacha v0.3.1
  Downloaded cfg-if v1.0.4
  Downloaded rand_core v0.6.4
  Downloaded ppv-lite86 v0.2.21
  Downloaded rand v0.8.6
  Downloaded zerocopy v0.8.48
  Downloaded libc v0.2.186
   Compiling libc v0.2.186
   Compiling zerocopy v0.8.48
   Compiling cfg-if v1.0.4
   Compiling getrandom v0.2.17
   Compiling rand_core v0.6.4
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.6
   Compiling hello_dep v0.1.0 (/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.XPj3oNXDE6/hello_dep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.20s
exit=0
```

Five reading-shapes are present:

1. `Updating crates.io index` — Cargo refreshing its local copy of
   the registry index.
2. `Locking 14 packages to latest Rust 1.95.0 compatible versions` +
   `Adding rand v0.8.6 (available: v0.10.1)` — the resolver
   choosing concrete versions. Note the version drift below.
3. `Downloading crates ...` followed by `Downloaded <crate> v<ver>`
   lines — Cargo fetching crate sources.
4. `Compiling <crate> v<ver>` lines for each transitive dep
   (`libc`, `zerocopy`, `cfg-if`, `getrandom`, `rand_core`,
   `ppv-lite86`, `rand_chacha`), then `Compiling rand v0.8.6`,
   then finally `Compiling hello_dep v0.1.0 (...)`. The package
   is built **last**, after all its dependencies.
5. `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in
   11.20s` — bit-identical Finished line shape from cycle 064;
   the elapsed time is much higher because Cargo just compiled
   eight extra crates.

**Version drift honesty.** The Book quotes `Adding rand v0.8.5`
(corpus line 524) and `Compiling rand v0.8.5` (corpus line 538);
the live registry resolved to **0.8.6**. The rule that explains
why this resolution is allowed (SemVer / version-requirement
syntax) is documented at
`cargo/reference/specifying-dependencies.md` lines 35-49 and 53-67
and at Book ch02 lines 502-503 (quoted in the corpus map below);
unpacking that rule is explicitly deferred to a future cycle. The
lesson body acknowledges the drift in one sentence ("Cargo picked
a version *compatible* with what `Cargo.toml` requested") without
explaining the mechanism. The transcript is faithful to the live
registry on probe day.

The `available: v0.10.1` clause on the `Adding` line shows the
latest published `rand` (a much newer release of `rand`; why
Cargo did not pick it falls under the deferred SemVer rule named
above). The Book's older snapshot shows `available: v0.9.0`;
same kind of clause, newer registry state. Not load-bearing
beyond honest reporting.

### Step 9: Directory listing — `Cargo.lock` and `target/` exist

```text
$ ls -la
total 16
drwxr-xr-x  6 eli  staff   192 May  7 12:36 .
drwx------  3 eli  staff    96 May  7 12:36 ..
-rw-r--r--  1 eli  staff  3351 May  7 12:36 Cargo.lock
-rw-r--r--  1 eli  staff    95 May  7 12:36 Cargo.toml
drwxr-xr-x  3 eli  staff    96 May  7 12:36 src
drwxr-xr-x@ 5 eli  staff   160 May  7 12:36 target
```

`Cargo.lock` (3351 bytes) was created by the resolving build,
recording the exact versions Cargo chose. Its contents are
**deferred** per orchestrator directive; the lesson names the file's
existence and Book-grounded purpose without unpacking the TOML.
`target/` is the build cache from cycle 064.

### Step 10: Cache-hit `cargo build`

```text
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
exit=0
```

**Identical shape to cycle 064 step 11.** No `Updating`, no
`Downloading`, no `Compiling` — Cargo sees the manifest and source
unchanged and exits with just the `Finished` line. This matches the
Book's wording at ch02 lines 560-565: "If you immediately run
`cargo build` again without making any changes, you won't get any
output aside from the `Finished` line."

### Step 11: Calibration `cargo run`

```text
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/hello_dep`
Hello, world!
exit=0
```

This is the empirical witness for the lesson's claim that
`src/main.rs` is unchanged: the program prints `Hello, world!` —
the same default greeting from `cargo new` that cycles 032 and 064
also captured. Adding `rand` to `[dependencies]` did NOT alter the
program's behavior; the dependency is *available* but unused.

## Corpus quote map

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md` — primary source

**Lines 476-481** — framing the manifest edit:

> Cargo's coordination of external crates is where Cargo really shines.
> Before we can write code that uses `rand`, we need to modify the
> *Cargo.toml* file to include the `rand` crate as a dependency. Open
> that file now and add the following line to the bottom, beneath the
> `[dependencies]` section header that Cargo created for you.

**Lines 488-493** — the literal manifest line (load-bearing for the
lesson body's exact `rand = "0.8.5"` quote):

> Filename: Cargo.toml
>
> ```toml
> [dependencies]
> rand = "0.8.5"
> ```

**Lines 495-503** — the `[dependencies]` section's purpose:

> In the *Cargo.toml* file, everything that follows a header is part
> of that section that continues until another section starts. In
> `[dependencies]`, you tell Cargo which external crates your project
> depends on and which versions of those crates you require.

**Lines 520-541** — the verbatim resolver-run output the lesson
quotes a subset of:

> ```console
> $ cargo build
>   Updating crates.io index
>    Locking 15 packages to latest Rust 1.85.0 compatible versions
>     Adding rand v0.8.5 (available: v0.9.0)
>  Compiling proc-macro2 v1.0.93
>  Compiling unicode-ident v1.0.17
>  Compiling libc v0.2.170
>  Compiling cfg-if v1.0.0
>  Compiling byteorder v1.5.0
>  Compiling getrandom v0.2.15
>  Compiling rand_core v0.6.4
>  Compiling quote v1.0.38
>  Compiling syn v2.0.98
>  Compiling zerocopy-derive v0.7.35
>  Compiling zerocopy v0.7.35
>  Compiling ppv-lite86 v0.2.20
>  Compiling rand_chacha v0.3.1
>  Compiling rand v0.8.5
>  Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
>   Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
> ```

The Book's ch02 transcript (line 524 `Adding rand v0.8.5`, line 538
`Compiling rand v0.8.5`) and the probe's transcript
(`Adding rand v0.8.6`, `Compiling rand v0.8.6`) differ only in the
exact patch version Cargo's resolver picked; the Book itself warns
of this at lines 545-547: "You may see different version numbers
(but they will all be compatible with the code, thanks to SemVer!)
and different lines (depending on the operating system), and the
lines may be in a different order." This is the corpus-grounded
honesty for the version-drift note in the lesson body.

**Lines 549-558** — what `Updating crates.io index` and the
download/compile lines mean (load-bearing for the lesson's "Cargo
fetches the crate plus any crates *it* depends on" claim):

> When we include an external dependency, Cargo fetches the latest
> versions of everything that dependency needs from the *registry*,
> which is a copy of data from [Crates.io](https://crates.io/).
> Crates.io is where people in the Rust ecosystem post their open
> source Rust projects for others to use.
>
> After updating the registry, Cargo checks the `[dependencies]`
> section and downloads any crates listed that aren't already
> downloaded. In this case, although we only listed `rand` as a
> dependency, Cargo also grabbed other crates that `rand` depends on
> to work. After downloading the crates, Rust compiles them and then
> compiles the project with the dependencies available.

This paragraph grounds three lesson-body claims at once: (a) Cargo
fetches from crates.io; (b) Cargo also grabs the listed crate's
own dependencies (transitive); (c) dependencies are compiled
*before* the package, exactly as transcript step 8 shows.

**Lines 560-565** — the cache-hit observation (load-bearing for
transcript step 10):

> If you immediately run `cargo build` again without making any
> changes, you won't get any output aside from the `Finished` line.
> Cargo knows it has already downloaded and compiled the
> dependencies, and you haven't changed anything about them in your
> *Cargo.toml* file.

**Lines 587-606** — `Cargo.lock`'s purpose (load-bearing for the
lesson's Cargo.lock-existence claim, which deliberately defers
contents):

> Cargo has a mechanism that ensures that you can rebuild the same
> artifact every time you or anyone else builds your code: Cargo
> will use only the versions of the dependencies you specified
> until you indicate otherwise. ... To handle this, Rust creates
> the *Cargo.lock* file the first time you run `cargo build`, so we
> now have this in the *guessing_game* directory.
> When you build a project for the first time, Cargo figures out
> all the versions of the dependencies that fit the criteria and
> then writes them to the *Cargo.lock* file. ... Because the
> *Cargo.lock* file is important for reproducible builds, it's
> often checked into source control with the rest of the code in
> your project.

The lesson uses only the file's existence + one-sentence purpose
("Cargo's record of the exact versions it picked"); the deeper
mechanics (rebuild-the-same-artifact guarantee, source control,
update flow) are explicitly deferred per orchestrator directive.

### `output/docs/rust/cargo/reference/specifying-dependencies.md` — secondary source

**Lines 12-20** — the `<crate> = "<version>"` shape's general form
(supporting the lesson's `<crate-name> = "<version-string>"` schema):

> Cargo is configured to look for dependencies on
> [crates.io](https://crates.io/) by default. Only the name and a
> version string are required in this case. ...
>
> ```toml
> [dependencies]
> time = "0.1.12"
> ```

**Lines 35-49** — the caret-default rule (cited only as the
deferred-topic anchor for the lesson's version-drift note;
explicitly deferred in the lesson body):

> **Default requirements** specify a minimum version with the ability
> to update to [SemVer](https://semver.org) compatible versions. ...
> `0.2.3 := >=0.2.3, <0.3.0`

This citation grounds the lesson's version-drift acknowledgement
(transcript shows `rand v0.8.6` resolved for the `"0.8.5"`
specifier) by naming the deferred topic; the lesson body does not
unpack this rule, and neither does this appendix.

### `output/docs/rust/book/ch01-03-hello-cargo.md` — supporting source

Already used by lessons 032 and 064 for the cargo build output
shape. Today's lesson reuses that grounding by reference;
specifically:
- Lines 119-127: the `Compiling` + `Finished` two-line shape that
  forms the *base* of today's output, extended with the resolver
  lines.
- Lines 158-162: the cache-hit shape (no `Compiling`, just
  `Finished`) that step 10 reproduces.

### `output/docs/rust/cargo/commands/cargo-build.md` — supporting source

**Lines 6 and 13-14** (cycle 064 already cited):

> cargo-build — Compile the current package
> ...
> Compile local packages and all of their dependencies.

The "and all of their dependencies" clause was *moot* in cycle 064
(no dependencies in the empty `[dependencies]`); today it is
load-bearing: the dependencies are real and the resolver lines in
step 8's transcript are Cargo's narration of that "and all of their
dependencies" half.

## Prerequisite-claim summary (1-3 bullets each per direct prerequisite)

### From lesson 032 (`032-cargo-new-and-run`) — *direct, load-bearing*

- `cargo new <name>` scaffolds a package directory containing a
  `Cargo.toml` (with `[package]` and an empty `[dependencies]`
  section) and `src/main.rs`. Today opens that file and adds one
  line under `[dependencies]`.
- `cargo run` from inside the package directory compiles and runs
  the program in one command. Used today as the calibration probe
  in step 11 to verify `src/main.rs` still prints `Hello, world!`.

### From lesson 064 (`064-cargo-build-standalone`) — *direct, load-bearing*

- `cargo build` from inside a package directory compiles `src/main.rs`
  and writes the executable to `target/debug/<name>`, printing
  `Compiling <name> v0.1.0 (...)` then `Finished \`dev\` profile ...
  in X.XXs`. Today's output extends that pair with a head section
  (`Updating crates.io index`, `Locking ...`, `Adding ...`,
  `Downloading`/`Downloaded`) and many more `Compiling` lines —
  one per transitive dependency, plus the listed crate, plus the
  package itself last.
- A second `cargo build` with no source/manifest change prints
  `Finished` only, no `Compiling`. Step 10 reproduces this exactly,
  even with dependencies present — the cache-hit rule is unchanged.

### From lessons 001, 002, 011 — *supporting*

Mentioned by number/title only. Cycle 001 grounds compile-and-run
as two distinct activities; cycle 002 grounds `fn main()` as the
entry point; cycle 011 grounds `println!`'s stdout printing. Today
the program (`fn main() { println!("Hello, world!"); }`) is
unchanged from `cargo new`'s default and is exercised only via
`cargo run` in step 11; none of these claims are load-bearing in a
way not already carried forward by cycle 032/064.

## Contrast-probe coverage

The lesson's contrastive structure has two layers:

1. **`cargo build` with empty `[dependencies]`** (cycle 064 step 5)
   vs. **`cargo build` with `rand = "0.8.5"` added** (today step 8).
   Same command, same fresh-cache state, same package shape — the
   only difference is the manifest entry. Cycle 064's two-line
   output (`Compiling` + `Finished`) becomes today's longer
   output (`Updating` + `Locking` + `Adding` + `Downloading`/
   `Downloaded` lines + many `Compiling` lines + `Finished`). The
   two transcripts sit side-by-side; cycle 064's transcript is
   reused directly as the negative side of the contrast.
2. **First resolving build** (today step 8, 11.20s) vs. **cache-hit
   rebuild** (today step 10, 0.04s). Same package, same manifest,
   only difference is whether anything has changed since the last
   build. Resolver activity disappears entirely on the cache hit —
   load-bearing for the lesson's "after that first resolving build,
   subsequent no-change builds are back to cycle 064's cache-hit
   shape" claim.

No separate negative/broken probe is needed: both contrasts are
between successful states.

## Network access — the contrastive caveat

The first resolving `cargo build` (step 8) **requires** network
reach to crates.io. The orchestrator directive states: "if for any
reason crates.io is unreachable, capture the failure faithfully and
stop." On probe day crates.io was reachable and the build
succeeded; this is documented in the prerequisite list as an
ordinary computer-use assumption ("internet access").

The cache-hit step 10 does NOT require network — Cargo had already
downloaded everything in step 8, and step 10 verifies nothing
changed. Once downloaded, dependencies do not require re-fetching
on subsequent builds (Book lines 560-565 above).

## Notes on `Cargo.lock`

The lesson treats `Cargo.lock` minimally: it exists after the
resolving build (verified in step 9's `ls -la`), it records the
versions Cargo chose (Book lines 587-606), and the file's contents
are deferred. Specifically NOT in the lesson:

- The TOML structure of `Cargo.lock` (multi-line `[[package]]`
  blocks, `name`, `version`, `source`, `checksum`, `dependencies`).
- Source-control practice (the Book's "checked into source control"
  recommendation; cycle 064's `.gitignore` already lists
  `Cargo.lock` per orchestrator directive — that is for *probe
  hygiene* in this run, not a teaching point).
- The reproducible-build guarantee.
- `cargo update` and the version-bump flow.

These are unlocked for future cycles.

## Honesty notes

1. **rand version drift.** The Book quotes `rand v0.8.5` in the
   manifest line and resolver output. The probe resolved to
   `rand v0.8.6`. The mechanism (SemVer / version-requirement
   syntax) is documented at the cargo reference lines 35-49 and
   Book ch02 lines 502-503 and is explicitly deferred to a future
   cycle; the lesson body acknowledges the drift in one sentence
   ("compatible with what `Cargo.toml` requested") without
   unpacking the rule. Both the Book line 492 manifest entry and
   the probe's actual transcript are quoted verbatim — no
   smoothing.
2. **Probe transitive-dep set differs from Book's.** The Book's
   ch02 transcript lists 14 `Compiling` lines (`proc-macro2`,
   `unicode-ident`, `libc`, `cfg-if`, `byteorder`, `getrandom`,
   `rand_core`, `quote`, `syn`, `zerocopy-derive`, `zerocopy`,
   `ppv-lite86`, `rand_chacha`, `rand`); the probe's transcript
   lists eight (`libc`, `zerocopy`, `cfg-if`, `getrandom`,
   `rand_core`, `ppv-lite86`, `rand_chacha`, `rand`). This is
   because `rand 0.8.6`'s dependency tree is slimmer than `rand
   0.8.5`'s on this Rust version. The lesson body shows the
   transcript faithfully and treats the exact dependency list as
   incidental, naming only the load-bearing `Compiling rand
   v0.8.6` and `Compiling hello_dep v0.1.0 (...)` plus a few
   examples.
3. **`src/main.rs` is unchanged.** This is the cycle's central
   honest claim. The program does not import or call `rand`;
   `cargo run` in step 11 prints `Hello, world!`, the original
   default greeting. Importing and calling rand is the next
   cycle's move, explicitly deferred in the lesson body and in the
   graph node's `unlocks`.

## Files committed for this cycle

- `lessons/065-cargo-toml-dependencies-entry.md` (this lesson)
- `evidence/065-cargo-toml-dependencies-entry.md` (this appendix)
- `observations/065-cargo-toml-dependencies-entry/Cargo.toml` (post-edit)
- `observations/065-cargo-toml-dependencies-entry/src/main.rs` (unchanged)
- `observations/065-cargo-toml-dependencies-entry/.gitignore`
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
