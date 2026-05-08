# Evidence — 060-input-prompt-loop

Audit appendix for `lessons/060-input-prompt-loop.md`. This is a *pure
composition cycle*: it installs no new Rust mechanic, so the appendix
is structurally different from a typical mechanic-introducing cycle.
The appendix's job is to (a) verify the program runs as the lesson
claims on three different inputs, (b) confirm that every line of the
program is licensed by an installed cycle, and (c) ground the few
substantive non-composition claims (the canonical pattern's existence
in the Book, the *Check Yourself* (a) and (b) answers' empirical
behavior).

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/060-input-prompt-loop.rs`); the calibration `.rs`
  files (the optional shadowing variant; the *Check Yourself* (a)
  and (b) variants) are not committed -- their transcripts below are
  the artifacts.

## Sources

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053, 054, 055, 056, 058, 059. The primary corpus
license today: Listing 2-5 (lines 1147-1190) is the canonical
audience-level form of the input-prompt loop. Today's lesson is the
empirical demonstration that fifteen previously-installed cycles
(twelve when collapsing 040+042+044+048's basic-surface support)
suffice to write that listing's read-and-validate inner shape.

Lines 1160-1175 (the load-bearing inner shape, re-cited from cycles
054, 056, 058, 059):

> ```rust
>     loop {
>         println!("Please input your guess.");
>
>         let mut guess = String::new();
>
>         // --snip--
>
>         io::stdin()
>             .read_line(&mut guess)
>             .expect("Failed to read line");
>
>         let guess: u32 = match guess.trim().parse() {
>             Ok(num) => num,
>             Err(_) => continue,
>         };
> ```

Direct corpus precedent for today's working probe, modulo three
deliberate audience-level choices documented below:

1. The Book's `: u32` becomes `: i32` -- cycle 019 installed `i32`
   as our typed name; `u32` is a deferred sibling. The mechanic is
   identical (cycle 056's annotation-driven inference).
2. The Book uses cycle 057's type-changing shadowing
   (`let guess: u32 = match guess.trim().parse() { ... }` shadowing
   the `guess: String` from line 5). Today uses two names (`buf`
   and `n`) so the composition trace stays explicit -- each
   line-by-cycle attribution names a single binding throughout.
   Cycle 057 is *referenced* in *What Changed* and *What To Ignore
   For Now* but not load-bearing today; the optional probe below
   confirms the shadowing variant has the same observable behavior.
3. Today's program prints `got: {n}` then `break;` after a single
   successful read. The Book's guessing-game keeps looping (lines
   1180-1188 add the `cmp` match, with `break;` only on the win
   path). Today's `break;` is the smallest demonstration of the
   composition; the prompt-and-validate inner shape (the loop body
   from line 1160 to line 1174) is what's load-bearing.

Lines 1192 (the audience-level name for the listing):

> *[Listing 2-5](#listing-2-5): Ignoring a non-number guess and
> asking for another guess instead of crashing the program*

Direct corpus statement that the listing's role is exactly today's
move: gracefully handle invalid input, re-prompt on parse failure,
proceed on parse success.

Lines 1200-1214 (the audience-level walkthrough -- already cited in
cycles 058 and 059; re-cited here for the composition framing):

> If `parse` is able to successfully turn the string into a number,
> it will return an `Ok` value that contains the resultant number.
> That `Ok` value will match the first arm's pattern, and the
> `match` expression will just return the `num` value that `parse`
> produced and put inside the `Ok` value.
>
> If `parse` is *not* able to turn the string into a number, it will
> return an `Err` value that contains more information about the
> error. ... So, the program will execute the second arm's code,
> `continue`, which tells the program to go to the next iteration of
> the `loop` and ask for another guess.

The Book's plain-English walk corresponds line-for-line with the
working probe's behavior on the three inputs in *Try It*: a
parseable input takes the `Ok` arm and proceeds; an unparseable
input takes the `Err(_) => continue` arm and re-prompts.

Calibration: the Book's example also includes `println!("Please
input your guess.");` (line 1108, line 1161) before the read. Today's
program omits that prompt-print to keep the composition trace
focused on the read+parse+match braid; cycle 011's `println!` of a
fixed string is already installed and could be added without
introducing new syntax. The omission is documented in this evidence
appendix only, not in the lesson body.

### Lessons 027, 035, 040, 042, 044, 048, 049, 050, 052, 053, 054, 055, 056, 058, 059 (existing)

Today's lesson cites *every* one of these by id in its composition
trace. None contributes a *new* substantive claim -- each one's
prior evidence appendix already grounds its own facts. The appendix
records only the load-bearing claim each contributes to the
composition; full evidence stays in the original appendices.

- **Cycle 044 (`use Path::name;`)** -- the parent-module form `use
  std::io;` makes `io` reachable as a namespace. Used on line 1.
- **Cycle 027 (`loop { ... }` and `break;`)** -- unconditional
  repetition, exited via `break;`. Used on lines 4 and 12. The
  `break;` only fires on the success path; on the `Err` path the
  loop iterates.
- **Cycle 035 (`continue;`)** -- skips the rest of the current pass
  and returns to the loop head. Today used on line 9 inside a
  match arm body (cycle 059 is what makes this legal in the
  type-system; cycle 035 is what installs `continue;`'s runtime
  semantics).
- **Cycle 040 (dot-form method call)** -- the basic surface for
  `value.method(args)`. Used on lines 6 (`io::stdin().read_line(...)`,
  `.expect(...)`), 7 (`buf.trim()`, `.parse()`).
- **Cycle 042 (`String::new()`)** -- type-qualified call form for the
  empty-buffer constructor. Used on line 5.
- **Cycle 048 (`&mut binding` argument)** -- the argument shape for
  a `&mut T` parameter. Used on line 6.
- **Cycle 049 (method chaining)** -- left-associative chaining of
  dot-form calls. Used on line 6 (`io::stdin().read_line(...).expect(...)`,
  three calls) and line 7 (`buf.trim().parse()`, two calls). Cycle
  054 was the first to use a three-call chain in production; today
  reuses the same shape.
- **Cycle 050 (`io::stdin()`)** -- the free function returning a
  `Stdin` handle. Used on line 6.
- **Cycle 052 (`Result<T, E>`)** -- the prelude two-variant enum
  with `Ok(T)` / `Err(E)`. The return type of `read_line` (line 6)
  and `.parse()` (line 7).
- **Cycle 053 (`.expect("msg")`)** -- consumes a `Result`, yielding
  `Ok` payload or panicking on `Err`. Used on line 6 to handle
  `read_line`'s I/O error path.
- **Cycle 054 (`read_line(&mut buf)`)** -- the `Stdin` method that
  appends one newline-terminated line of stdin into a `&mut String`,
  returning `io::Result<usize>`. Used on line 6. Cycle 054 was
  itself the first composition cycle; today's lesson is the second.
- **Cycle 055 (`.trim()` returning `&str`)** -- the `&str`-or-`String`
  receiver method that strips leading and trailing whitespace
  (Unicode `White_Space`, including `\n`). Used on line 7. Strips
  *both* the trailing `\n` from `read_line` and any spaces the user
  typed -- the third *Try It* input `"  42  "` is what makes the
  spaces case empirically visible.
- **Cycle 056 (`.parse()` plus annotation-driven inference)** -- the
  `&str` method returning `Result<TARGET, _>` where `TARGET` is
  selected by a `: TYPE` annotation flowing back from the use site.
  Used on line 7. The `: i32` on `n` is the annotation that pins
  `TARGET = i32`.
- **Cycle 058 (payload-variant patterns `Ok(num)` / `Err(_)`)** --
  the `Variant(subpattern)` arm shape with a binding name on the
  `Ok` side and the cycle-031 wildcard on the `Err` side. Used on
  lines 8-9. The `: i32` on the binding (cycle 056) constrains
  both arms to produce `i32`, which forces `num: i32`, which forces
  `.parse()`'s target to be `i32`.
- **Cycle 059 (`continue` as a divergent arm body)** -- the type-rule
  exemption that allows the `Err(_) => continue` arm to coexist with
  the `Ok(num) => num` arm under the binding's `: i32` constraint.
  Used on line 9. The `continue` arm doesn't have to produce an
  `i32`; control transfers to the loop head before the value
  question matters.

The composition is *complete*: removing any one of the named cycles
either fails compilation (most of them), produces silently wrong
output (cycle 055 -- a never-trim'd buffer never `.parse()`s
successfully on a newline-terminated line, so the loop runs
forever), or hangs (cycle 027's `loop` removed, the program reads
once and exits without retrying; cycle 027's `break;` removed, the
program reads forever even on success). The lesson's claim is the
positive one: "all of them fit together to do real work." The
negative claim "remove any one and it breaks differently" is named
in the lesson's *Mental Model Delta* but not exercised cycle-by-
cycle (per the orchestrator directive: removing each one in turn is
overkill; the working probe's empirical correctness across three
distinct inputs is the load-bearing observation).

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/060-input-prompt-loop.rs`.
Identical source to the *The Move* and *Try It* blocks.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- ls before ---
demo.rs
--- cat demo.rs ---
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
--- rustc demo.rs ---
rustc-exit=0
--- ls after ---
demo
demo.rs
--- echo "42" | ./demo ---
got: 42
exit1=0
--- printf 'abc\n42\n' | ./demo ---
got: 42
exit2=0
--- printf 'abc\nxyz\n  42  \n' | ./demo ---
got: 42
exit3=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. No warnings. **This is the
  central composition observation:** fifteen cycles' worth of
  syntax in a single program compile cleanly. If any one cycle were
  uninstalled (e.g. cycle 058's payload-variant patterns, or cycle
  059's divergent-arm rule, or cycle 044's `use std::io;`), this
  exact source would fail to compile.
- The three input variants empirically corroborate the three
  composition behaviors:
  - *Single valid input first try* (`echo "42"`): pass 1 takes the
    `Ok` arm, `n` binds to `42`, `println!` prints, `break;` exits.
    Exit 0. Output `got: 42`.
  - *One invalid then valid* (`printf 'abc\n42\n'`): pass 1 reads
    `"abc\n"`, `.trim()` produces `"abc"`, `.parse()` produces
    `Err(...)`, `Err(_) => continue` arm fires, control jumps to
    loop head *without* reaching `println!` or `break;`. Pass 2:
    fresh `buf` (line 5 makes a new `String::new()` each iteration
    -- not reused), `read_line` reads `"42\n"`, `Ok` arm fires,
    `got: 42`, `break;`. Exit 0.
  - *Two invalids then valid with whitespace* (`printf 'abc\nxyz\n
    42  \n'`): same pattern as above but three passes. The third
    input `"  42  \n"` exercises cycle 055's whitespace-stripping
    behavior on *both* sides of `42` -- `.trim()` strips the leading
    spaces, the trailing spaces, *and* the `\n`. `.parse()` sees
    `"42"` cleanly. Output `got: 42`. Exit 0.
- All three inputs produce identical output (`got: 42`). The shared
  output is the load-bearing observation: *the input-prompt loop
  abstracts over the number of invalid attempts*. This is what
  makes the pattern useful in practice and why the Book uses it.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Calibration probe (Shape A -- shadowing variant)

The Book's actual ch02 listing 2-5 uses cycle 057's type-changing
shadowing: the buffer name `guess` is shadowed by the parsed `guess:
i32`. Today's lesson uses two names (`buf` and `n`) for trace
clarity. This probe confirms the shadowing variant has identical
observable behavior, so the lesson's *What Changed* claim "the
cycle-057 form is one rename away" is empirically grounded.

Source (not committed -- transcript is the artifact):

```rust
use std::io;

fn main() {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("got: {guess}");
        break;
    }
}
```

Captured 2026-05-07:

```text
--- cat shadow.rs ---
use std::io;

fn main() {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("got: {guess}");
        break;
    }
}
--- rustc shadow.rs ---
rustc-exit=0
--- echo "42" | ./shadow ---
got: 42
exit1=0
--- printf 'abc\n42\n' | ./shadow ---
got: 42
exit2=0
```

Notes:

- `rustc shadow.rs` exits 0 silently. The shadowing variant
  type-checks identically to the two-name variant.
- Both inputs produce the same `got: 42` output as the working
  probe. The behavior is genuinely identical; the only difference
  is the binding name on lines 5 and 7.
- This probe is *load-bearing* for the *What Changed* claim "the
  cycle-057 form is one rename away" and grounds the negative claim
  "the Book's exact form would also work; the choice between two
  names and shadowing is purely stylistic at this surface."
- The shadowing variant exercises cycle 057's load-bearing rule
  (the new `guess` is `i32`, the old `guess` was `String`); the
  evidence appendix for cycle 057 already grounds this rule. Today
  is just the composition demonstration.

### Calibration probe (Shape B -- `buf` declared outside the loop)

Used to verify the *Check Yourself (a)* answer empirically. The
change from the working probe: `let mut buf = String::new();` is
hoisted *above* `loop {`. Cycle 054's note that `read_line`
*appends* (does not overwrite) becomes load-bearing: pass 1's input
is preserved across iterations.

Source (not committed -- transcript is the artifact):

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    loop {
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

Captured 2026-05-07 (run with a 2-second perl-alarm wrapper around
the second input to confirm the loop never exits on the
*Check Yourself* (a) input, then SIGKILL'd):

```text
--- cat hoist.rs ---
use std::io;

fn main() {
    let mut buf = String::new();
    loop {
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let n: i32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("got: {n}");
        break;
    }
}
--- rustc hoist.rs ---
rustc-exit=0
--- echo "42" | ./hoist ---
got: 42
exit1=0
--- printf 'abc\n42\n' | (perl-alarm 2s) ./hoist ---
killed-by-signal=9 (SIGKILL after 2s timeout -- loop never exited)
```

(The `perl-alarm` wrapper is `perl -e 'use POSIX; my $pid = fork;
if ($pid == 0) { exec @ARGV; } else { local $SIG{ALRM} = sub { kill
9, $pid; }; alarm 2; waitpid($pid, 0); }' ./hoist` -- substrate
substitution because `timeout(1)` from GNU coreutils is not on this
macOS substrate's PATH.)

Notes:

- *Pass 1 of the second input*: `buf` becomes `"abc\n"`. `.trim()`
  gives `"abc"`. `.parse()` is `Err(...)`. `continue` fires.
- *Pass 2 of the second input*: `buf` is *not* reset (it lives
  outside the loop). `read_line` *appends* `"42\n"` (cycle 054's
  load-bearing note). `buf` is now `"abc\n42\n"`. `.trim()` gives
  `"abc\n42"` (only outer whitespace stripped; the inner `\n`
  remains). `.parse()` on `"abc\n42"` is `Err(...)`. `continue`
  fires.
- Subsequent passes: `read_line` blocks waiting for more input on
  stdin; `printf` already EOF'd, so `read_line` returns `Ok(0)`
  with `buf` unchanged. `.trim().parse()` on `"abc\n42"` is still
  `Err(...)`, so `continue` fires forever. The `timeout 2`
  terminates the process.
- The *first* input (`echo "42"`) still works because there's only
  one iteration. The hoisted `buf` problem only surfaces when
  iteration 2 starts with non-empty `buf`.
- This probe is *load-bearing* for the *Check Yourself (a)*
  answer's specific empirical claim: hoisting `buf` outside the
  loop produces an infinite loop on inputs that require multiple
  reads. The lesson body avoids over-explaining the mechanism (the
  appendix carries the detail); the prediction itself is the
  pedagogical work.

### Calibration probe (Shape C -- no `break;` on success path)

Used to verify the *Check Yourself (b)* answer empirically. The
change from the working probe: line 12's `break;` is removed.

Source (not committed -- transcript is the artifact):

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
    }
}
```

Captured 2026-05-07:

```text
--- cat nobreak.rs ---
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
    }
}
--- rustc nobreak.rs ---
rustc-exit=0
--- echo "42" | (perl-alarm 2s) ./nobreak ---
got: 42
killed-by-signal=9 (SIGKILL after 2s timeout -- loop never exited)
```

(The `perl-alarm` wrapper is `perl -e 'use POSIX; my $pid = fork; if
($pid == 0) { exec @ARGV; } else { local $SIG{ALRM} = sub { kill 9,
$pid; }; alarm 2; waitpid($pid, 0); }' ./nobreak` -- substrate
substitution because `timeout(1)` from GNU coreutils is not on this
macOS substrate's PATH.)

Notes:

- `rustc nobreak.rs` exits 0 silently. No warnings -- `n` is
  *used* on the same iteration as the `println!` placeholder, so
  the unused-variable lint does not fire.
- *Pass 1*: `read_line` reads `"42\n"`. `Ok` arm fires. `n = 42`.
  `println!` prints `got: 42`. Falls through the bottom of the loop
  body. **No `break;`.** Loop iterates.
- *Pass 2*: `read_line` blocks; `echo` already EOF'd; `read_line`
  returns `Ok(0)` with empty `buf`. `.trim().parse()` on `""` is
  `Err(...)` (per cycle 056's note that `.parse()` does not accept
  empty strings -- empirically corroborated by this probe).
  `continue` fires.
- *Pass 3 onward*: same as pass 2. Infinite loop on `Err(...)` from
  empty stdin reads. The `timeout 2` terminates the process.
- This probe is *load-bearing* for the *Check Yourself (b)*
  answer's claim about EOF behavior. Cycle 054 already noted that
  `read_line` returns `Ok(0)` at EOF without panicking; today
  exercises the *consequence* of that behavior in the input-prompt
  loop -- without `break;` the loop never exits.
- The probe also empirically corroborates the *What To Ignore For
  Now* deferral about EOF handling: a real interactive program
  needs some way to recognize `Ok(0)` from `read_line` and break
  out of the loop. Today's program sidesteps EOF by `break;`-ing
  on the first success.

## Direct prerequisite claims

This is a composition cycle, so each direct prerequisite contributes
its full installed surface to today's program. The summary below
lists only what *part* of each prerequisite the working probe uses;
the prerequisite's full evidence stays in its own appendix.

- **Cycle 027 (load-bearing)** -- `loop { ... }` repeats the body
  forever; `break;` exits. Used as the outer loop on line 4 and the
  success-path exit on line 12.
- **Cycle 035 (load-bearing)** -- `continue;` skips the rest of the
  current pass and returns to the loop head. Used on line 9 as the
  retry mechanism for unparseable input.
- **Cycle 040 (load-bearing)** -- dot-form method call
  `value.method(args)`. Used on lines 6 and 7. All four method
  calls in the working probe (`.read_line`, `.expect`, `.trim`,
  `.parse`) are this surface.
- **Cycle 042 (load-bearing)** -- `String::new()` constructs an
  empty `String` via the type-qualified call form. Used on line 5.
- **Cycle 044 (load-bearing)** -- `use std::io;` brings the `io`
  module into scope so line 6 can write `io::stdin()` instead of
  `std::io::stdin()`. The parent-module form (Book ch02 uses this
  exact form) was deferred from cycle 050 and picked up in cycle
  054.
- **Cycle 048 (load-bearing)** -- `&mut binding` argument shape for
  a `&mut T` parameter. Used on line 6 as `&mut buf`.
- **Cycle 049 (load-bearing)** -- left-associative method chaining.
  Used on line 6 (`io::stdin().read_line(...).expect(...)` -- three
  calls) and line 7 (`buf.trim().parse()` -- two calls). The `match`
  scrutinee on line 7 is itself a chain.
- **Cycle 050 (load-bearing)** -- `io::stdin()` returns a `Stdin`
  handle. Used as the receiver of `.read_line` on line 6.
- **Cycle 052 (load-bearing)** -- `Result<T, E>` with `Ok(T)` /
  `Err(E)`. Returned by both `.read_line` (line 6) and `.parse()`
  (line 7). The variants `Ok` and `Err` are used as patterns on
  lines 8-9.
- **Cycle 053 (load-bearing)** -- `.expect("msg")` consumes a
  `Result`, yielding `Ok` payload or panicking with `msg: <Err>`.
  Used on line 6 to handle `read_line`'s I/O error path. The
  guessing-game's idiomatic message `"Failed to read line"` matches
  cycle 053's installation.
- **Cycle 054 (load-bearing)** -- `Stdin::read_line(&mut buf)`
  appends one newline-terminated line of stdin into `buf`,
  returning `io::Result<usize>`. Used on line 6. Cycle 054's
  *append* semantics is what makes the *Check Yourself (a)* answer
  load-bearing: with `buf` declared outside the loop, appending
  across iterations corrupts the buffer.
- **Cycle 055 (load-bearing)** -- `.trim()` returns a `&str` with
  leading and trailing whitespace stripped (Unicode `White_Space`,
  including `\n` and `' '`). Used on line 7. Strips the trailing
  `\n` from `read_line` *and* the leading/trailing spaces in the
  third *Try It* input `"  42  "`.
- **Cycle 056 (load-bearing)** -- `.parse()` on `&str` returns
  `Result<TARGET, _>`; `TARGET` is selected by inference from a
  `: TYPE` annotation flowing back from the use site. Used on line
  7. The `: i32` on `n` (line 7) is what pins `TARGET = i32`.
- **Cycle 058 (load-bearing)** -- payload-variant patterns
  `Variant(subpattern)` for matching enum variants with payloads.
  Used on lines 8-9. `Ok(num)` binds the parsed integer to `num`;
  `Err(_)` matches any `Err` and discards its payload.
- **Cycle 059 (load-bearing)** -- diverging arm bodies (`continue`,
  `break`, `return`) are exempt from cycle 030's all-arms-share-type
  rule. Used on line 9: `Err(_) => continue` doesn't have to
  produce an `i32`, and the match's type comes from the
  value-producing `Ok(num)` arm.
- **Cycles 001, 002, 005, 006, 011, 019** -- compile and run, `fn
  main`, `let`, `let mut`, `{name}` placeholder, `: i32`
  annotation. All used unchanged.

## Older supporting lessons

- Cycle 003 (read-rustc-diagnostic) -- not exercised today (the
  working probe compiles cleanly with no diagnostics; the *Check
  Yourself* (c) calibration shows one warning, but the lesson body
  doesn't rely on diagnostic-reading machinery).
- Cycle 007 (shadowing) -- *referenced* in the lesson body's *What
  Changed* bullet about the Book's `let guess: i32 = match
  guess.trim().parse() { ... }` form. Cycle 057 (the type-changing
  generalization) is the more direct relevance, also referenced.
  Neither is load-bearing for today's two-name probe.
- Cycle 030 (`match` machine), cycle 031 (`_` wildcard) -- both
  used implicitly via cycles 058 and 059. The `match` arms on
  lines 8-9 inherit cycle 030's all-arms-share-type rule (refined
  by cycle 059), and the `_` inside `Err(_)` is cycle 031's
  wildcard reused inside a constructor (cycle 058's structural
  generalization).
- Cycle 045 (shared reference `&binding`) -- *not* used today; the
  argument on line 6 is `&mut buf` (mutable, cycle 048), not
  `&buf`.
- Cycle 028 (`break value;`) -- *not* used today. Mentioned in
  *What To Ignore For Now* as the alternative loop-as-expression
  shape.
- Cycle 032 (`cargo new`) -- *not* used today; cycle 060 follows
  cycle 001's `rustc demo.rs` shape per the lesson naming
  convention. Cargo will be load-bearing for the eventual `rand`
  crate composition.
- Cycle 051 (`Ordering` enum + variant match) -- *not* used today;
  the `cmp(&secret)` next layer of the Book's program would compose
  cycle 051 with `.cmp` on `i32` and `&secret` shared-reference
  argument. Mentioned in *What To Ignore For Now*.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe binding names are `buf` and `n`. The Book uses `guess`
  for both via cycle 057's shadowing. The two-name choice keeps the
  composition trace explicit -- each line's binding has one type.
  The shadowing variant (calibration probe Shape A) confirms
  observable equivalence.
- The probe's `let n: i32 = ...;` annotation is `: i32`, not the
  Book's `: u32`. Cycle 019 installed `i32`; `u32` is a deferred
  sibling. The mechanic is identical; cycle 056's inference works
  the same way.
- The probe's `let mut buf = String::new();` lives *inside* the
  loop body (line 5) rather than above it. This intentionally
  matches the Book's listing 2-5 structure (Book ch02 line 1163).
  The hoisted-outside variant (calibration probe Shape B) shows
  why the inside-the-loop placement matters: cycle 054's *append*
  semantics would corrupt a reused buffer.
- The probe omits the `println!("Please input your guess.");`
  prompt-print (Book ch02 line 1108, line 1161). Cycle 011's
  `println!` of a fixed string is already installed; adding the
  prompt would not change the composition argument and would add
  one more line to the trace. Omitted for brevity.
- The probe uses `break;` after the success print (line 12). The
  Book's program continues looping, with `break;` only on the
  `Ordering::Equal` win path (line 1185). Today's `break;` is the
  smallest demonstration that the input-prompt loop has done its
  job and exits to the rest of the program. The without-`break;`
  variant (calibration probe Shape C) exercises the EOF-handling
  consequence.
- The probe's `.expect("Failed to read line")` message matches the
  Book exactly (Book ch02 line 1114, line 1169). The cycle 053
  installation also used this exact string; today is just the
  composition reuse.
- The lesson's *Try It* uses three different inputs (`"42"`,
  `"abc\n42\n"`, `"abc\nxyz\n  42  \n"`) rather than one. The
  three inputs corroborate three different load-bearing
  composition behaviors (single-pass success, multi-pass success,
  whitespace-tolerance via cycle 055's `.trim()`). A single input
  would not exercise the loop's retry behavior.
- The lesson does *not* install or describe the never type `!`.
  Cycle 059 already deferred this; today reuses the same audience-
  level framing ("control-flow keywords let an arm escape").
- The lesson does *not* install `String::clear()`. The fresh
  `String::new()` per iteration is structurally simpler and
  matches the Book's listing 2-5 exactly. `String::clear()` is
  named in *What To Ignore For Now* as the alternative idiom that
  pairs with the *Check Yourself* (a) answer's hoisted-buffer
  diagnosis.
- The lesson does *not* install `cmp(&other)`, the comparison-
  against-secret pattern, or the `rand` crate. These are the next
  three forward pointers on the Book path; the cycle's *What To
  Ignore For Now* names them but does not unpack them.
- This is a *pure composition cycle*: per the orchestrator
  directive, no broken contrast is captured for "remove cycle X and
  the program breaks." Each removal would break the program
  differently, and the working probe's three-input correctness is
  load-bearing on its own. The two calibration probes capturing
  hoisted-`buf` and removed-`break;` behavior are *consequence*
  probes, not removals -- they ground the *Check Yourself*
  answers.
