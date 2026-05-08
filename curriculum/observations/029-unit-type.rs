fn say_hello() {
    println!("inside say_hello");
}

fn main() {
    let _unit_literal: () = ();
    let _empty_block: () = {};
    let _function_call: () = say_hello();
    println!("three () bindings compiled");
}
