// Lesson 119 — `Option<T>` with `Some(T)` / `None` constructors and a
// `match` opening the variants. Working probe.
//
// Compile and run:
//     $ rustc 119-option-some-none.rs
//     $ ./119-option-some-none
//     present -> 42
//     absent  -> -1
//
// Each `match` arm body is `i32`, so the whole `match` is `i32`; the
// `Some(n)` arm binds the payload to `n` (lesson 058's payload-pattern
// shape), and the `None` arm is bare (lesson 098's unit-variant shape).
// The bare `None` requires the `: Option<i32>` annotation per Book
// ch06-01 lines 389-395 — the contrast probe (`broken.rs`, not
// committed; transcript in evidence) drops that annotation and fires
// `error[E0282]: type annotations needed for \`Option<_>\``.

fn main() {
    let present: Option<i32> = Some(42);
    let absent: Option<i32> = None;

    let p_label = match present {
        Some(n) => n,
        None => -1,
    };
    let a_label = match absent {
        Some(n) => n,
        None => -1,
    };
    println!("present -> {}", p_label);
    println!("absent  -> {}", a_label);
}
