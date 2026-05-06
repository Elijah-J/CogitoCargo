#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let first_turn = Direction::Left;
    let second_turn = Direction::Right;

    println!("First turn: {:?}", first_turn);
    println!("Second turn: {:?}", second_turn);
}
