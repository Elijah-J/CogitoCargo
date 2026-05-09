# Evidence — Lesson 142: bind a closure literal to a `let` and call it with parens

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/142-closure-literal-bound-and-called.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/142-closure-literal-bound-and-called.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/142-closure-literal-bound-and-called.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet142/` and compiled with
`rustc <file>`; resulting executables were run from the same directory.
Same host and toolchain as accepted lessons through 141.

## Run context — first move of the closure sub-arc

Per `iterator-api-coverage.md` §6 (committed `cb9945066`, v2 of the
audit), the closure sub-arc is the next major arc after lesson 141
closed the closure-free Iterator surface (audit §5). The sub-arc has
five steps; today is step 1. Step 1 explicitly installs *only* the
closure literal syntax + bind-and-call shape — no type inference, no
capturing, no `Fn`/`FnMut`/`FnOnce` traits, no closure-as-function-param.
Those are gated to steps 2-5 respectively.

Lesson 141 was the previous accepted lesson (closes audit §5 step 11);
its `unlocks` list names the closure sub-arc as the next major arc.
Today's `depends_on` does not name 141 directly — there is no
load-bearing claim from `size_hint` reused here — but 141's `unlocks`
entry is the orchestration handoff.

## Direct prerequisite — lesson 008 (call-with-parens)

Lesson 008 installed the call-with-parens shape: `fn name() { ... }`
defines a function and `name();` calls it with parens. Today's
`add_one(5)` reuses that exact shape on a `let`-bound name. The new
fact relative to 008 is that the *callable thing* is no longer
restricted to a top-level `fn`-block: a closure literal value bound to
a `let` is also callable with the same parens shape. Lesson 008's
"control transfers in and comes back" semantic carries unchanged.

## Direct prerequisite — lesson 020 (typed parameter shape)

Lesson 020 installed `fn name(p: TYPE) { ... }` — one typed parameter
inside the function-parameter list, with `name: TYPE` mandatory in the
signature. Today's `|x: u32|` is the same `name: TYPE` slot, in
pipe-bracket-pipe brackets instead of parens. Lesson 020's hard rule
("in function signatures parameter types are mandatory") does *not*
extend to closures: closure parameter types are *optional* per Book
ch13-01:141-143 ("Closures don't usually require you to annotate the
types of the parameters"). Today's lesson keeps the parameter
annotated (Book v2 form for the parameter slot, v4 form for the body)
to stay within the audit §6 step-1 scope; the unannotated case is
explicitly deferred to step 2.

## Direct prerequisite — lesson 021 (function return value)

Lesson 021 installed `fn name(p: TYPE) -> RTYPE { ... }` — a function
declares a return type after `->` and the body sends a value back.
Today's closure `|x: u32| x + 1` returns a value too; the call site
`add_one(5)` is itself an expression whose value is what the body
produces. The return-value semantic carries; the difference is that
the return type is *inferred* from the body for closures (`-> u32` is
written in Book v2, omitted in v3/v4). Today omits it (v4 form for the
body).

## Direct prerequisite — lesson 025 (implicit return)

Lesson 025 installed: a function body is a block, the block's value is
the value of its tail expression (no `;`), and `-> RTYPE` makes that
value the function's return value, so `return value;` and a bare
`value` on the last line are equivalent. Today's closure body `x + 1`
is exactly the bare-tail-expression form: no `;`, no `return`, no
braces. The Book at ch13-01:217-218 explicitly says of v4: "we remove
the brackets, which are optional because the closure body has only one
expression" — which is the v4 form. So the implicit-return rule from
lesson 025 carries to closures, and Book v4 is its most compact
expression in this slot.

## Direct prerequisite — lesson 005 (let binding)

Lesson 005 installed `let name = value;`. Today's
`let add_one = |x: u32| x + 1;` reuses the binding shape on the new
right-hand-side form (a closure literal value). The lesson 005
property "later statements use the name as that value" carries: the
later statements `let a = add_one(5);` and `let b = add_one(10);`
*call* the bound value with parens. (The Book at ch13-01:160-161 makes
the same comparison: "we're defining a closure and storing it in a
variable rather than defining the closure in the spot we pass it as an
argument.")

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 080**: `u32` is one of the integer types. Today's parameter
  type and return type both use `u32`.
- **Lesson 009**: `+` on integers produces a new integer. Today's body
  `x + 1` is one such expression (with `1` as the integer literal).
- **Lesson 011**: `println!("{}", name)` formats with positional args.
  Used in Probe 1.
- **Lesson 003**: rustc diagnostic map (headline + `-->` location +
  source excerpt with caret + `note:` / `help:`). Used to read Probe 2's
  E0308 block.
- **Lesson 002**: `fn main` is the entry point.
- **Lesson 001**: `rustc file.rs` then `./name`; silent on success.

## Source — Book ch13-01-closures.md (canonical four-form table)

The lesson's load-bearing source is
`output/docs/rust/book/ch13-01-closures.md`. Three load-bearing
passages:

### Lines 6-9 (closures intro):

```text
Rust's closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure elsewhere to evaluate it in a different context.
```

This is the source for "anonymous function you can save in a variable"
in the lesson body, and for the framing "today only covers the 'save
in a variable' half; passing closures to functions is deferred."

### Lines 208-213 (the four-form table):

```text
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

This is the canonical table the lesson cites at the top of "The new
fact" section. v1 is the function form (lesson 020+021+025 shape).
v2 is the fully annotated closure (parameter type, return type, brace
body). v3 drops the type annotations. v4 drops the braces. Today's
`add_one = |x: u32| x + 1;` is a hybrid of v2's parameter annotation
and v4's brace-free expression body — chosen explicitly because (a)
the parameter annotation gives rustc enough info to resolve the body
type without entering "first call fixes the type" territory (which is
step 2's job); (b) the brace-free body is the simplest body form and
directly reuses lesson 025's implicit-return rule.

### Lines 215-219 (commentary on the table):

```text
The first line shows a function definition and the second line shows a fully
annotated closure definition. In the third line, we remove the type annotations
from the closure definition. In the fourth line, we remove the brackets, which
are optional because the closure body has only one expression. These are all
valid definitions that will produce the same behavior when they're called.
```

This is the source for the lesson's `## The new fact` bullet that says
"the body is a single expression `x + 1`, no braces. Lesson 025's
implicit-return rule applies." The Book quote is reproduced verbatim
in the lesson body.

### Lines 141-143 (closures don't require annotations):

```text
Closures don't usually require you to annotate the types of the parameters or
the return value like `fn` functions do. Type annotations are required on
functions because the types are part of an explicit interface exposed to your
users.
```

This is the source for noting that closure parameter annotation is
*optional* (unlike functions, per lesson 020's hard rule). Today's
lesson does not center this fact — the unannotated case is the next
move's centered fact (audit §6 step 2). But the lesson's `What To
Ignore For Now` first bullet names it explicitly.

## Probe 1 — working probe (closure bound to `let`, called twice)

Source: `observations/142-closure-literal-bound-and-called.rs`.
Transcript: `observations/142-closure-literal-bound-and-called.transcript.txt` Probe 1 block.

```rust
fn main() {
    let add_one = |x: u32| x + 1;
    let a = add_one(5);
    let b = add_one(10);
    println!("{}", a);
    println!("{}", b);
}
```

Output:

```text
6
11
```

Compile exit 0, run exit 0. Both load-bearing structural facts are
witnessed:

- The form `|x: u32| x + 1` parses and binds to `add_one` without any
  trait imports or annotations beyond what is in the source.
- The bound name `add_one` is callable with the lesson-008 parens
  shape, with an integer argument matching the parameter type. Two
  call sites with different arguments produce two different output
  values, witnessing that the parameter `x` substitutes the argument
  in the body the same way a function parameter does.

The body's value `x + 1` arithmetic-on-integers (lesson 009) plus
`println!`-with-positional-args (lesson 011) produce the two output
lines.

## Probe 2 — negative contrast (the closure is a value with a type)

Source `noparens.rs` (in transcript). Output:

```text
error[E0308]: mismatched types
 --> noparens.rs:3:18
  |
2 |     let add_one = |x: u32| x + 1;
  |                   -------- the found closure
3 |     let a: u32 = add_one;
  |            ---   ^^^^^^^ expected `u32`, found closure
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
          found closure `{closure@noparens.rs:2:19: 2:27}`
help: use parentheses to call this closure
  |
3 |     let a: u32 = add_one(/* u32 */);
  |                         +++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Compile exit 1. Three grounded facts from rustc's mouth:

- `add_one` is a value (rustc treats it as a usable expression, marked
  with `the found closure` annotation under the closure literal). The
  E0308 mismatch is between the bound value's type and `u32`, not a
  parse error or an "unbound name" error.
- The kind of `add_one`'s type is `closure` (named twice: in the
  `expected/found` line and in the `note:` block). The full type name
  is `{closure@noparens.rs:2:19: 2:27}` — rustc's anonymous
  closure-type spelling, opaque to the user, tied to the source
  location of the closure literal.
- The `help:` line names the missing step: "use parentheses to call
  this closure." Suggested edit: `add_one(/* u32 */)`. This grounds
  the call-with-parens shape — calling the closure value is what gets
  the body's value out.

This is the contrastive witness for the lesson's claim "the closure
is a value with a type, and you call it with parens." Probe 1 shows
the working call-with-parens form; Probe 2 shows what rustc says when
you skip the call.

The choice of `noparens.rs` over the prompt-suggested
`let _: u32 = |x: u32| x + 1;` form: both produce a similar E0308
"found closure" message naming the closure type from rustc's mouth.
`noparens.rs` is preferred because it (a) reuses Probe 1's exact
`let add_one = |x: u32| x + 1;` literal verbatim and asks one new
question (drop the parens), keeping the contrastive surface minimal;
(b) produces both diagnostics needed in one source — `the found
closure` annotation directly on the closure literal AND the
`expected u32, found closure` line at the use site; (c) the help-line
edit `add_one(/* u32 */)` directly grounds the call shape, which the
lesson centers.

## Probe-not-run — inferred-type case (E0271)

The prompt's alternative-suggested probe was
`let f = |x| x + 1; let n: u32 = f(5);`. I ran it during evidence
collection and it produced:

```text
error[E0271]: type mismatch resolving `<i32 as Add>::Output == u32`
 --> inferred.rs:2:19
  |
2 |     let f = |x| x + 1;
  |                   ^ expected `u32`, found `i32`
```

Rustc here infers `x: i32` from the body `x + 1` (since `1` is an
unannotated integer literal that defaults to `i32` per lesson 080's
integer-defaulting), then E0271 fires when the call-site
`let n: u32 = f(5)` clashes. This is interesting but goes directly
into territory reserved for closure sub-arc step 2 ("type inference
and the 'first call fixes the type' rule"). Including it in today's
lesson would (a) preempt step 2's centered fact and (b) require
explaining default integer literal type, which the lesson does not
otherwise touch. I therefore did not center this probe in the lesson;
it is documented here only to record that the alternative was tested
and explicitly deferred.

The `noparens.rs` probe (Probe 2) is preferred because it isolates
"the closure is a value with a type" without entering inference
territory.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| "Functions so far lived at the top level of the file as `fn`-blocks" | Lessons 008/020/021 (accepted) |
| "you called them from inside `main` with parens" | Lesson 008 (`name();` shape) |
| `let add_one = |x: u32| x + 1;` parses and runs | Probe 1 transcript: compile-exit=0, run-exit=0 |
| `add_one(5)` yields `6`, `add_one(10)` yields `11` | Probe 1 output: two lines `6` and `11` |
| Pipes `|...|` enclose a parameter list with `name: TYPE` shape | Book ch13-01:208-213 (four-form table); Probe 1 source |
| "no braces and no trailing `;`" body form | Book ch13-01:215-219 (commentary); Probe 1 source |
| Lesson 025's implicit-return rule applies to closures | Book ch13-01:217-218 ("we remove the brackets, which are optional because the closure body has only one expression") + lesson 025 |
| Return type is inferred from the body | Book ch13-01:141-143 ("Closures don't usually require you to annotate ... the return value") + Probe 1 (no `-> u32` written, compile silent) |
| Closures are "anonymous functions you can save in a variable" | Book ch13-01:6-9 (verbatim quote) |
| "passing closures to functions is deferred" | Audit §6 steps 4-5; lesson 142 `What To Ignore For Now` |
| The closure is a value | Probe 2: `the found closure` annotation under the closure literal |
| Its type is *not* `u32` | Probe 2: `expected u32, found closure` line |
| The kind is `closure` | Probe 2: `note: ... found closure {closure@...}` |
| Type spelling `{closure@<file>:<line>:<col>: <line>:<col>}` | Probe 2: `{closure@noparens.rs:2:19: 2:27}` |
| `help:` line says "use parentheses to call this closure" | Probe 2: verbatim |
| `triple = |n: u32| n * 3; triple(7)` compiles silently and prints `21` | Side-probe `tiny.rs` (run during evidence collection, not committed; reproduces Check-Yourself answer) |

## Older supporting lessons (named only)

The following accepted lessons are cited in the lesson body but their
exact prereq claims are restated either in the direct-prereq sections
above or in the lesson's own Prerequisites bullets:

- 080-integer-type-family — `u32`.
- 009-arithmetic-on-integers — `+` on integers.
- 011-println-positional-args — `println!("{}", name)`.
- 003-read-rustc-diagnostic — diagnostic map for Probe 2 E0308.
- 002-fn-main-entry-point — `fn main` is the entry point.
- 001-rustc-compile-and-run — `rustc file.rs`, `./name`.

## Deliberate scope discipline (per audit §6 step 1)

The prompt's "Required scope discipline" section names six things to
NOT touch:

1. Closure capturing outer bindings — deferred to step 3.
2. The `Fn`/`FnMut`/`FnOnce` traits — deferred to steps 4/5.
3. Closure type-inference rules and "first-call-fixes-the-type" — step 2.
4. Closures over generic types — out of scope for the sub-arc.
5. Passing closures to functions — step 4.
6. `impl Fn`/`Box<dyn Fn>` return forms — out of scope for the sub-arc.

The lesson body's `What To Ignore For Now` section names all six
explicitly with their step number where applicable. Probe 2's E0308
diagnostic from rustc names `closure {closure@<loc>}` — the lesson
acknowledges this name exists (rustc said it) but explicitly defers
explaining what kind of type that is, why it cannot be written by
hand, or how it relates to `Fn`/`FnMut`/`FnOnce`. The prompt's
deferred list is honored at every point.
