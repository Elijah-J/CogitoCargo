fn main() {
    let n: i32 = 3;
    let label: i32 = match n {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => 99,
    };
    println!("label = {label}");
}
