fn main() {
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("counter = {counter}");
}
