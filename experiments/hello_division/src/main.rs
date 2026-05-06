fn main() {
    // These bindings hold integer literals.
    let total = 10;
    let groups = 3;

    // In integer division, / keeps the whole-number quotient.
    let each = total / groups;

    // % gives the remainder from the same operands.
    let leftover = total % groups;

    println!("Each: {each}");
    println!("Leftover: {leftover}");
}
