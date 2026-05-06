fn static_message() -> &'static str {
    "Hello from a string literal"
}

fn main() {
    // The tempting first signature was:
    // fn static_message() -> &str { "Hello from a string literal" }
    //
    // rustc reported E0106: missing lifetime specifier.
    // The fixed signature writes &'static str because a string literal can
    // be returned as a reference that lives for the whole program.
    let message = static_message();

    println!("{message}");
}
