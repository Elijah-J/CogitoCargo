// Probe for lesson 096 — declare an inline submodule with `mod foo { ... }`
// and expose one function inside it with `pub`, then call the function
// from `fn main()` outside the module via the path `foo::hi()`.
//
// Two coupled pieces in one program:
//
//   1. Inline submodule:  `mod foo { ... }`
//   2. Public item:       `pub fn hi() { ... }` inside the module body
//
// Compile (silent, exit 0):
//
//     $ rustc demo.rs
//     $ ls
//     demo  demo.rs
//
// Run:
//
//     $ ./demo
//     hi from foo
//     (exit 0)
//
// The contrast probe in evidence/096-inline-module-and-pub.md drops the
// `pub` keyword from the function and re-runs `rustc`. With the same
// `foo::hi();` call site, rustc fires
// `error[E0603]: function \`hi\` is private` — the module boundary is the
// place where the visibility rule becomes observable. The auxiliary probe
// drops the `foo::` path prefix from the call site and witnesses an E0425
// (`cannot find function \`hi\` in this scope`), grounding the framing
// "the inline module is a namespace; `foo::hi` is the path to reach into
// it" without installing the `use` move twice.

mod foo {
    pub fn hi() {
        println!("hi from foo");
    }
}

fn main() {
    foo::hi();
}
