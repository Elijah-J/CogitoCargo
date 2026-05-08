// Capstone observation for cycle 063.
//
// Build: rustc demo.rs
// Run: printf '7\n' | ./demo  (and the five other piped inputs)
//
// This is the Book ch02 Listing 2-6 program with one substitution:
// `let secret_number: u32 = 7;` replaces the `rand` call. No new Rust
// mechanic is introduced; every line is licensed by an accepted graph
// node 001..062 or by ordinary computer-use. See lessons/063 and
// evidence/063 for the line-by-line license map and full transcripts.

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = 7;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
