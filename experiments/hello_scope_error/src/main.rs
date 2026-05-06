fn main() {
    // The fixed version keeps an outer binding available after the block.
    let name = "outer";

    {
        // This binding only exists inside this inner block.
        let name = "inner";
        println!("Inside block: {name}");
    }

    // Without the outer binding, this line produced E0425:
    // cannot find value `name` in this scope.
    println!("After block: {name}");
}
