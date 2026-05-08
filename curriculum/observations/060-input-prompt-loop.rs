use std::io;

fn main() {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let n: i32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("got: {n}");
        break;
    }
}
