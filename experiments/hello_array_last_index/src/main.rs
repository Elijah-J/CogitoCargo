fn main() {
    let numbers = [3, 4, 5];

    // This works because the array has at least one element.
    let last_index: usize = numbers.len() - 1;

    // A computed usize index can go inside [] just like a literal index.
    let last = numbers[last_index];

    println!("last index: {last_index}");
    println!("last: {last}");
}
