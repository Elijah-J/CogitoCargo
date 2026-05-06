fn main() {
    // The type annotation gives the empty array an element type and length.
    let numbers: [i32; 0] = [];

    let length = numbers.len();

    println!("length: {length}");
}
