---
id: 122-self-method-delegation
status: accepted
evidence: ../evidence/122-self-method-delegation.md
---

# Call one method from another with `self.method(args)`

## The Move

Lesson 040 installed the dot-call shape `value.method(args)`: a receiver
expression, a dot, a method name, and a parenthesized argument list.
Lesson 100 installed `&self` as the first parameter of a method and
`Self` as the impl-target alias, so that `c.current()` in `main` reaches
the body of `fn current(&self) -> u32`. Today fills the *value* slot of
the dot-call with `self` itself — *inside another method body on the
same impl*. No new mechanic; one new context for the existing
method-call shape.

```rust
struct Counter { n: u32 }

impl Counter {
    fn doubled(&self) -> u32 {
        self.n * 2
    }
    fn quadrupled(&self) -> u32 {
        self.doubled() * 2
    }
}

fn main() {
    let c = Counter { n: 7 };
    println!("doubled    = {}", c.doubled());
    println!("quadrupled = {}", c.quadrupled());
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints `doubled    = 14`
then `quadrupled = 28`. The line that carries today's move is
`self.doubled() * 2`. Read it left-to-right: `self` is the receiver
expression — it names the implicit `&self` parameter of `quadrupled`;
`.doubled` is the method name; `()` is the empty argument list. The
whole expression `self.doubled()` is exactly lesson 040's dot-call shape
with `self` filling the receiver slot. Its result is a `u32` (the
return type lesson 100 declared on `doubled`), and that `u32` enters the
`* 2` (lesson 009) on the right.

Method names are looked up on the type of the receiver. Inside
`quadrupled`'s body, `self` has type `&Counter` (the `&self` shorthand
expanded — Book ch05-03:62), so `self.doubled()` looks up `doubled` on
`Counter` and finds the sibling method declared two lines above.

## Mental Model Delta

- *Before:* "I can call methods on values from `main` via `c.method()`
  (lessons 040, 100). I have not seen one method on a type call
  another method on the same type."
- *After:* "Inside a method body, `self` is a value of type `&Self`
  (or `&mut Self`, or `Self`, depending on the receiver). It fits in
  the receiver slot of the dot-call the same way `c` did in `main`.
  One method calls a sibling method by writing `self.sibling(args)`.
  The lookup rule is unchanged: rustc looks for `sibling` on the type
  of `self`. If the sibling does not exist on that type, the same
  E0599 from lesson 100 fires — pointing into the method body now,
  not into `main`."

## Prerequisites

- Installed concepts:
  - **Lesson 040** (load-bearing): the dot-call shape
    `value.method(args)`. Today fills the *value* slot with `self`.
  - **Lesson 100** (load-bearing): inherent `impl Type { ... }`,
    `&self` as the shorthand for `self: &Self`, `Self` as the
    impl-target alias, the return-type slot `-> u32`, and the
    call-site `c.current()`. Today's impl has two methods, one calling
    the other.
  - **Lesson 095** (load-bearing): `struct Counter { n: u32 }`,
    `Counter { n: 7 }`, and field access `self.n`.
  - **Lessons 009, 080, 011, 005, 002, 001, 003, 019** (cited): `*` on
    integers; `u32`; `println!` positional-arg formatting; `let`;
    `fn main`; `rustc demo.rs && ./demo`; the diagnostic four-part
    map; the `: TYPE` annotation slot reused as `-> u32`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs` in a fresh empty directory.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
doubled    = 14
quadrupled = 28
```

Walk it. `c.doubled()` reaches `doubled`'s body `self.n * 2` =
`7 * 2 = 14`. `c.quadrupled()` reaches `quadrupled`'s body
`self.doubled() * 2`: it calls the sibling method `doubled` on the same
`self`, gets back `14`, then multiplies by `2` to produce `28`.

*Now the contrast.* Drop the `fn doubled` definition entirely, but keep
`quadrupled`'s body as `self.doubled() * 2`. Save as `broken.rs`:

```rust
struct Counter { n: u32 }

impl Counter {
    fn quadrupled(&self) -> u32 {
        self.doubled() * 2
    }
}

fn main() {
    let c = Counter { n: 7 };
    println!("quadrupled = {}", c.quadrupled());
}
```

Compile:

```text
error[E0599]: no method named `doubled` found for reference `&Counter` in the current scope
 --> broken.rs:5:14
  |
5 |         self.doubled() * 2
  |              ^^^^^^^ method not found in `&Counter`
```

Read with the lesson 003 map. Headline `error[E0599]` — the same
E-code lesson 100 installed for missing `&self` methods. The location
points *inside* `quadrupled`'s body now, not at a `main`-side call.
The phrase "no method named `doubled` found for reference `&Counter`"
states the rule today centers: `self.method()` requires `method` to
exist on the type of `self`. Here `self` has type `&Counter` (the
`&self` shorthand expanded), and `Counter` no longer has a `doubled`
method, so lookup fails.

## What Changed

- A method can call a sibling method on the same type via the dot-form
  `self.method(args)` inside its own body.
- `self.method(args)` is exactly lesson 040's dot-call shape — `self`
  fills the receiver slot. Nothing about the call expression is new.
- Inside an `&self` method, `self` has type `&Self` (Book ch05-03:62);
  rustc looks up the method name on that type. If the method is not
  on the type, the same E0599 from lesson 100 fires — pointing into
  the method body now, not into `main`.
- This makes rmp's `src/biguint/cmp.rs:14` line `Some(self.cmp(other))`
  readable: `self.cmp(other)` is exactly today's shape, with `cmp` as
  the sibling method name and `other` as the argument.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

impl Tally {
    fn add_one(&self) -> u32 {
        self.n + 1
    }
    fn add_three(&self) -> u32 {
        self.add_one() + 2
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("add_three = {}", t.add_three());
}
```

(a) Does `rustc tiny.rs` accept the program (silent, exit 0)?

(b) What single line does `./tiny` print?

(c) If you delete the `fn add_one` definition but leave
`self.add_one() + 2` inside `add_three`'s body, what E-code appears,
and what type name does the diagnostic say `add_one` was not found on?

*(Answers: (a) Yes. (b) `add_three = 13` — `t.add_three()` calls
`self.add_one()` on `self: &Tally`, gets `10 + 1 = 11`, then adds `2`.
(c) E0599; rustc says `no method named \`add_one\` found for reference
\`&Tally\` in the current scope`.)*

## What To Ignore For Now

Today installs only the dot-form delegation `self.method(args)` inside
an `&self` method body. Deferred:

- *`Self::method(args)` — the path-form self-reference.* The
  named-type analog `Counter::doubled(self)` is corroborated in the
  appendix; the `Self::` shorthand stays deferred from lesson 100.
- *Delegation in `&mut self` bodies* — composes lesson 101 with
  today's mechanic.
- *Delegation in `self` by-value bodies* — composes lesson 102; the
  receiver in the body has type `Self`, not `&Self`.
- *Recursive self-calls* — `self.foo()` calling `foo` itself directly
  or through a cycle.
- *Trait-method delegation* — calling another trait method from a
  trait method body. Composes lessons 111-116 with today's mechanic.
- *Auto-deref method resolution* — already deferred from lesson 100.
- *Multiple `impl` blocks* — sibling methods reachable across two
  impl blocks on the same type. Same lookup rule, larger scope.

## Evidence

See `../evidence/122-self-method-delegation.md`.
