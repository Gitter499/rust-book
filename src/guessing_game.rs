use rand::Rng;
use std::cmp::Ordering;
use std::io;


// Simple Rust guessing game, first Rust book tut

pub fn execute() {
    println!("Welcome to Rafayel's guessing game!");

    loop {
        println!("Guess a number");

        let num = rand::thread_rng().gen_range(1, 101);
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Uh oh, we ran into an error");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&num) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low chief"),
            Ordering::Equal => {
                println!("Bingo",);
                break;
            }
        }
    }
}
