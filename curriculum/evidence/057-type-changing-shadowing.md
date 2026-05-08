# Evidence — 057-type-changing-shadowing

Audit appendix for `lessons/057-type-changing-shadowing.md`. Holds the
corpus-quote map, the toolchain string, the working- and contrast-probe
transcripts, the contrastive-probe-omission justification, and the
prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/057-type-changing-shadowing.rs`). The two contrast
  transcripts below are not committed as separate `.rs` files — the
  transcripts themselves are the artifacts.

## Sources

### `output/docs/rust/book/ch03-01-variables-and-mutability.md`

The Book's "Shadowing" section. Already cited in lesson 007 (which
installed the same-type shadowing rule). Reused here for the *single*
load-bearing span new to cycle 057.

Lines 211-215 (the type-change license):

> The other difference between `mut` and shadowing is that because
> we're effectively creating a new variable when we use the `let`
> keyword again, we can change the type of the value but reuse the
> same name. For example, say our program asks a user to show how
> many spaces they want between some text by inputting space
> characters, and then we want to store that input as a number:

Lines 217-222 (the canonical example):

> ```rust
> fn main() {
>     let spaces = "   ";
>     let spaces = spaces.len();
> }
> ```

Lines 224-227 (the audience-level explanation):

> The first `spaces` variable is a string type, and the second
> `spaces` variable is a number type. Shadowing thus spares us from
> having to come up with different names, such as `spaces_str` and
> `spaces_num`; instead, we can reuse the simpler `spaces` name.

These three spans are the *whole* corpus license for cycle 057's
move:
- Lines 211-213 ("we can change the type of the value but reuse the
  same name") — direct corpus statement of the lesson's main install.
  Quoted verbatim in *Mental Model Delta — After*.
- Lines 217-222 (the example) — the Book's canonical type-changing
  shadow shape. Today's lesson uses a structurally identical shadow
  (`let n = "42"; let n: i32 = n.parse().expect("...");`) but with a
  different end-target: cycle 056's `.parse() + .expect()` chain
  instead of `.len()`. The Book's `let spaces = spaces.len();` would
  also work today but depends on `.len()` returning a `usize`, which
  is not yet an installed typed name (deferred from cycle 055). The
  parse-form is what makes the shadow's payload visible *and*
  installable today.
- Lines 224-227 (audience-level distinction "string type ... number
  type") — corpus license for the lesson body's framing
  ("`&str` ... `i32`") and for the *Mental Model Delta*'s "different
  type."

Calibration: the Book's example shadow goes `&str → usize`. Today's
lesson uses `&str → i32`, which is mechanically identical (a number
type, just one whose name is already installed). The lesson does NOT
install `.len()` on `&str` or `String` (cycle 055 used `.len()` on
`String` only as a small collateral observation; the return type was
already deferred there). The lesson's choice of `parse()` keeps the
load-bearing prereq (cycle 056) tight and avoids re-opening the
`usize` deferral.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book's guessing-game chapter. Already cited extensively (lessons
042, 044, 050-056). Reused here for the most prominent
type-changing-shadow occurrence in the Book.

Lines 911-921 (the canonical guessing-game shadow plus its commentary):

> ```rust
> let guess: u32 = guess.trim().parse().expect("Please type a number!");
> ```
>
> We create a variable named `guess`. But wait, doesn't the program
> already have a variable named `guess`? It does, but helpfully Rust
> allows us to shadow the previous value of `guess` with a new one.
> *Shadowing* lets us reuse the `guess` variable name rather than
> forcing us to create two unique variables, such as `guess_str` and
> `guess`, for example. We'll cover this in more detail in
> [Chapter 3](ch03-01-variables-and-mutability.md#shadowing), but for
> now, know that this feature is often used when you want to convert
> a value from one type to another type.

Two load-bearing claims:
- "Rust allows us to shadow the previous value of `guess` with a new
  one" — corpus statement that the Book's guessing-game `let guess`
  on line 912 is a shadow over the earlier `let mut guess = String::new();`
  (line 891). The earlier `guess` is a `String`; the shadow's `guess`
  is `u32`. This is a *string-to-number* shadow with a different
  string-side type than today's lesson but the same mechanic.
- "this feature is often used when you want to convert a value from
  one type to another type" — Book-level statement of the *purpose*
  of type-changing shadowing. The lesson body does not lean on this
  framing in the main path (the lesson observes the empirical fact
  rather than arguing for it, per the *What To Ignore For Now* note).
  The framing grounds the *Why type-changing shadowing exists*
  deferral.

Calibration: the Book's guessing-game shadow uses
`let guess: u32 = guess.trim().parse().expect(...);` — three
methods. Today's lesson uses `let n: i32 = n.parse().expect(...);` —
two methods (no `.trim()`). The reason is the prerequisite chain:
cycle 055 installed `.trim()` *on a `String`*, returning `&str`.
Today's old binding `n` is *already* a `&str` (the literal `"42"`),
so `.trim()` is unnecessary and would only add a method-chain step
without exercising today's main install. The full
`let buf: i32 = buf.trim().parse().expect(...);` chain (where the
old `buf` is the `String` from `read_line`) is the natural next
cycle and is named in *What To Ignore For Now*.

### `output/docs/rust/book/ch03-01-variables-and-mutability.md` (already cited above)

The lines 168-169 quote ("We can shadow a variable by using the
same variable's name and repeating the use of the `let` keyword")
and lines 162-167 (the `shadowed` definition) are inherited from
lesson 007. Not re-cited here. Cycle 057 reuses lesson 007's
grounding for the *mechanic* of shadowing and adds *only* the
type-change extension at lines 211-227.

### Lesson 056's evidence appendix (existing)

The full grounding for `.parse()` on `&str`, the `: i32` annotation
driving target-type inference, and the `.parse().expect(...)` chain
producing the parsed value or panicking on `Err` is in
`evidence/056-str-parse-to-i32.md`. Cycle 057 reuses that grounding
unchanged — the right-hand side `n.parse().expect("not a number")`
is *exactly* cycle 056's working chain, applied to a binding `n`
whose value is the same literal `"42"`. Not re-cited inline.

### Lesson 055's evidence appendix (existing)

The fact that string literals like `"42"` are `&str` is grounded in
`evidence/055-string-trim.md` (citing Book ch04-03 lines 369-381).
Today's `let n = "42";` reuses this directly: the first binding's
type is `&str`. Not re-cited inline.

### Lesson 009 (existing)

Lesson 009 installed `*` as integer multiplication on `i32`. The
working probe's line 4 (`let doubled: i32 = n * 2;`) is the
empirical witness that the second `n` is an `i32` — `*` succeeding
on `n * 2` requires both operands to be the same numeric type, and
the binding annotation pins down `i32`. Not re-cited inline.

### Lesson 007 (existing)

Lesson 007 installed shadowing as a mechanic: a second `let` with
the same name creates a new binding rather than mutating the old
one. The whole rule is reused unchanged; today's lesson is exactly
the type-change extension that lesson 007 explicitly deferred under
*What To Ignore For Now* ("type-changing shadowing"). Not re-cited
inline.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/057-type-changing-shadowing.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- cat demo.rs ---
fn main() {
    let n = "42";
    let n: i32 = n.parse().expect("not a number");
    let doubled: i32 = n * 2;
    println!("n = {n}, doubled = {doubled}");
}
--- rustc demo.rs ---
rustc-exit=0
--- ls ---
demo
demo.rs
--- ./demo ---
n = 42, doubled = 84
demo-exit=0
--- cleanup ---
ok
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. The program type-checks even
  though `n` appears with two different types — `&str` on line 2 and
  `i32` on line 3 — because line 3 is a *new binding* (cycle 007's
  shadowing rule), not a reassignment of the line-2 `n`.
- `./demo` prints exactly one line: `n = 42, doubled = 84`. The
  literal `"42"` is the old `&str` `n`; `.parse().expect("...")`
  produced `42_i32` for the new `n`; `n * 2` is integer
  multiplication on the new `i32` `n`, yielding `84`.
- `n * 2` succeeding silently is the empirical proof that the new
  `n` is an `i32`. `*` is not defined on `&str`; if `n` were still a
  `&str` on line 4, rustc would refuse. (Confirmed by the
  type-ascription side probe below.)
- Exit 0 — no panic on the success path, same as cycle 056.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

This probe is *load-bearing* for the lesson's central claim "the
new binding's type can differ from the old's." The fact that
rustc accepts the program with `n` first as a `&str` then as an
`i32` is the empirical witness, and the success of `n * 2` on the
new `n` is the secondary witness that the new type really is
`i32`.

### Side probe — type-ascription confirmation (not committed)

Auxiliary probe pinning down the *type* of each binding using
unused-binding type ascription. Confirms each `n` mention's type
without relying on the operator hint.

Source:

```rust
fn main() {
    let n = "42";
    let _: &str = n;  // forces n's type to be &str
    let n: i32 = n.parse().expect("not a number");
    let _: i32 = n;   // forces second n's type to be i32
    println!("ok");
}
```

Transcript:

```text
--- rustc intype.rs ---
rustc-exit=0
./intype prints: ok
```

Notes:

- Both `let _: TYPE = n;` ascriptions compile. Line 3's
  `let _: &str = n;` accepts only because `n` at that point is a
  `&str`. Line 5's `let _: i32 = n;` accepts only because `n` at
  that point is an `i32`.
- Together these two type-ascription bindings *prove* that the
  second `let n: i32 = ...;` shadows the first with a different
  type. This probe is the load-bearing witness for the lesson's
  *Mental Model Delta — After* claim ("first `n` is a `&str` and
  second `n` is an `i32`").
- The unused `_` binding is cycle 029's underscore-prefix gloss
  applied here — `_` is the canonical "intentionally unused"
  binding name.

### Contrast probe — `.trim()` on the new `i32` binding (not committed)

The lesson's *Check Yourself* (b) describes a program that fails
because `.trim()` is not a method on `i32`. This contrast is *not*
in the main path (the lesson's claim is "shadowing CAN change type",
not "with X works without X fails"); it lives only in the *Check
Yourself* prediction. The transcript is captured here for audit.

Source:

```rust
fn main() {
    let n = "42";
    let n: i32 = n.parse().expect("not a number");
    let trimmed = n.trim();
    println!("n = {n}, trimmed = {trimmed}");
}
```

Transcript, captured 2026-05-07:

```text
--- rustc broken_a.rs ---
error[E0599]: no method named `trim` found for type `i32` in the current scope
 --> broken_a.rs:4:21
  |
4 |     let trimmed = n.trim();
  |                     ^^^^ method not found in `i32`
  |
note: there's an earlier shadowed binding `n` of type `&'static str` that has method `trim` available
 --> broken_a.rs:2:9
  |
2 |     let n = "42";
  |         ^ `n` of type `&'static str` that has method `trim` defined earlier here
3 |     let n: i32 = n.parse().expect("not a number");
  |         - earlier `n` shadowed here with type `i32`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
rustc-exit=1
```

Notes (probe evidence — not corpus quotation):

- Headline: `error[E0599]: no method named `trim` found for type `i32` in the current scope`.
  E0599 is a NEW E-code (not yet installed in the graph). Per
  orchestrator directive in the cycle-057 prompt, today's lesson
  *does not* install E0599. The lesson body refers to the
  diagnostic in the *Check Yourself* (b) answer as text ("'no
  method named `trim` found for type `i32`'"), without naming the
  E-code. Future cycles can install E0599 when needed.
- The diagnostic is *strikingly informative* about the lesson's
  central claim: it explicitly names both bindings and their types.
  - "earlier shadowed binding `n` of type `&'static str` that has
    method `trim` available" — rustc itself states the old `n`'s
    type as `&'static str`. (The `'static` lifetime detail is
    deferred — the lesson treats string-literal types as `&str`.)
  - "earlier `n` shadowed here with type `i32`" — rustc itself
    states the new `n`'s type as `i32`.
  This is the *strongest* possible empirical evidence for the
  lesson's central claim ("the type can change across a shadow"):
  rustc literally narrates the type change in the diagnostic. The
  lesson body honors the orchestrator's "do not install E0599"
  directive by not surfacing this transcript verbatim, but the
  appendix records it as load-bearing audit evidence.
- Exit code 1 (compile error). No binary produced.

### Contrast probe — direct `&str → i32` assignment (not committed)

Alternate broken contrast considered in the orchestrator prompt:
write `let m: i32 = n;` while `n` is still a `&str`, *without*
calling `.parse()`. Captures the type-distinction (E0308 — already
installed) but does *not* directly demonstrate "shadowing changes
the type." Captured here for audit completeness.

Source:

```rust
fn main() {
    let n = "42";
    let m: i32 = n;
    println!("m = {m}");
}
```

Transcript, captured 2026-05-07:

```text
--- rustc broken_b.rs ---
error[E0308]: mismatched types
 --> broken_b.rs:3:18
  |
3 |     let m: i32 = n;
  |            ---   ^ expected `i32`, found `&str`
  |            |
  |            expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
rustc-exit=1
```

Notes (probe evidence — not corpus quotation):

- Headline: `error[E0308]: mismatched types`. E0308 is *already
  installed* (lessons 024, 025, 026, 028, 033, 045-048, 052, 054,
  055). Same E-code family as the cycle-055 calibration probe
  (`String` vs `&str` mismatch).
- Caret label `expected `i32`, found `&str`` confirms — independently
  of any shadowing — that `&str` and `i32` are distinct types in
  rustc's view. This is the *underlying type-distinction* fact the
  lesson rests on (without it, "the type changes" would be
  vacuous).
- This contrast is *not* in the lesson's main path (per orchestrator
  guidance — option three: positive demonstration only). The
  evidence is captured here so the lesson's *implicit* claim
  ("`&str` and `i32` are different types") has empirical
  grounding rather than resting on cycle 055's appendix alone.
- Exit code 1 (compile error). No binary produced.

### Contrastive-probe-omission justification

The lesson's main concept is "a shadow's new binding can have a
different type than the old binding." This is a *capability* claim,
not a *constraint* claim — there is no "with X this works, without
X it fails" form. Per the README's *Audit Trail Depth* section
("when the move says 'with X this works, without X it fails/differs,'
include a negative/contrast probe or state why one is not needed"),
no broken-contrast probe is required for the main install.

Lessons 049 (method chaining) and 050 (`io::stdin`) followed the
same pattern. The orchestrator's cycle-057 prompt explicitly
recommended this approach ("Recommended: Third alternative —
positive demonstration only") and noted that the working probe's
line 4 (`let doubled: i32 = n * 2;`) *is* the demonstration.

Two contrast probes are nonetheless captured above as audit
evidence — one strengthens the lesson body (the *Check Yourself*
(b) prediction), the other grounds the implicit "`&str` and `i32`
are distinct types" fact. Neither is in the lesson's main path.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 057.

- **Lesson 007 (load-bearing)** — installed shadowing as a
  mechanic: a second `let name = ...;` does not reassign the old
  binding; it creates a new binding under the same name. From the
  second `let` onward, uses of the name see the new binding. No
  `mut` involved. Cycle 007's example used the same kind of value
  on both sides; today's extension is *only* the value-side
  freedom. The mechanic itself is reused unchanged.
- **Lesson 056 (load-bearing)** — installed `.parse()` on `&str`
  returning `Result<TARGET, ...>` whose target type is selected by
  inference from the binding's `: TYPE` annotation, with
  `.expect("msg")` consuming the `Result`. Today's right-hand side
  `n.parse().expect("not a number")` *with* the `: i32` annotation
  is exactly cycle 056's working chain. Without cycle 056, the
  shadow's right-hand side would have no installed shape that
  produces an `i32` from a `&str`.
- **Lesson 055 (load-bearing)** — installed `&str` as a typed
  name, with string literals like `"42"` being `&str`. Today's
  first binding `let n = "42";` is a `&str` per cycle 055.
- **Lesson 019 (load-bearing)** — installed `let name: TYPE = value;`
  with `i32` as the canonical typed name. Today's `let n: i32 = ...;`
  reuses that exact shape. The annotation also drives the right-hand
  side per cycle 056.
- **Lesson 009 (load-bearing for the witness)** — installed `*` as
  integer multiplication on `i32`. The line `let doubled: i32 = n * 2;`
  is the empirical *witness* that the new `n` is an `i32`: `*` is
  not defined on `&str`, so `n * 2` succeeding *proves* the new
  binding's type. Without cycle 009, the lesson would need a
  different witness.
- **Lessons 040, 049, 052, 053** — dot-form method calls, method
  chaining, `Result<T, E>`, and `.expect`. All used unchanged in
  the right-hand side `n.parse().expect("...")`.
- **Lessons 001, 002, 005, 011** — `rustc file.rs` then `./name`,
  `fn main` entry, `let name = value;`, and the `{name}` placeholder
  in `println!`. All used unchanged.

## Older supporting lessons

- Lesson 006 (`mut`-binding distinction). Cycle 007 already
  contrasted shadowing against `mut`-reassignment; today's lesson
  does not re-open that contrast. The Book's ch03-01 lines 230-249
  demonstrate that `let mut spaces = "   "; spaces = spaces.len();`
  *fails* with E0308 (the type-cannot-change-via-mut rule), which
  is the structural mirror of today's lesson — but cycle 007 already
  installed the `let`-vs-`=` distinction, so re-running this contrast
  here would be redundant. The Book span is named only.
- Lesson 053 (`.expect` runtime panic). Cycle 056 already installed
  `.expect` on `Result<T, E>`. Today's right-hand side reuses cycle
  056's chain unchanged; the panic shape is not exercised
  (working probe path is `Ok` only). Not load-bearing for cycle 057
  beyond inheriting the chain.
- Lessons 003 (rustc-diagnostic structure), 045 (shared reference,
  `&str`'s leading `&`), 029 (`_`-prefix gloss for unused
  bindings) — used implicitly in transcript reading and the
  side-probe `_` binding. Not load-bearing for the main install.
- E-code inventory: today's main path triggers no diagnostics. The
  evidence-only contrast probes triggered E0599 (NEW — not
  installed) and E0308 (installed). The *Check Yourself* (b)
  prediction describes E0599 in audience-level prose without
  naming the E-code.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe target type is `i32` (cycle 019's typed name), matching
  cycle 056's choice. The Book ch02 guessing-game uses `u32`. Both
  work identically through `.parse()` — `u32` is named only in
  *What To Ignore For Now*.
- The probe input is `"42"` (matches cycle 056's input). The Book
  ch03-01 example shadow uses `"   "` (three spaces) and
  `.len()` — using `"42"` and `.parse()` keeps cycle 056 as the
  load-bearing prerequisite and avoids re-opening the deferred
  `usize` typed name.
- The probe binding name is `n` (cycle 056's name for `i32`
  bindings of parsed values), not the Book ch03-01's `spaces` or
  ch02's `guess`. Probe behavior is identical.
- The probe message is `"not a number"` (matches cycle 056). The
  Book uses `"Please type a number!"`. The shorter form keeps
  attention on the shadow.
- The witness line `let doubled: i32 = n * 2;` uses cycle 009's
  `*` rather than `+`. Both would work; `*` was chosen because the
  result `84` makes "the new `n` is being treated as a number" more
  visually obvious than `42 + 1 = 43` would. Calibration only.
- The lesson body avoids naming E0599 (per orchestrator directive).
  The *Check Yourself* (b) answer paraphrases the diagnostic
  ("'no method named `trim` found for type `i32`' diagnostic")
  without the E-code prefix.
- The contrast probe transcripts in this appendix include rustc's
  full help/note blocks. The lesson body summarizes them in one
  sentence; the appendix records them verbatim for audit.
