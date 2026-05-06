#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

// This function uses `match` to choose a value based on the variant.
fn describe(turn: Direction) -> &'static str {
    match turn {
        Direction::Left => "going left",
        Direction::Right => "going right",
    }
}

fn main() {
    // Each call passes a different variant to the same function.
    let first = describe(Direction::Left);
    let second = describe(Direction::Right);

    println!("{first}");
    println!("{second}");
}
