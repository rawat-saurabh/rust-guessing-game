use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let key = rand::thread_rng().gen_range(1..=10);
    // println!("Correct: {key}");

    println!("Hey, guess a number (1-10):");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error, reading the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error in parsing, try again. {e}");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&key) {
            Ordering::Greater => println!("Your guess is too high."),
            Ordering::Less => println!("Your guess is too low."),
            Ordering::Equal => {
                println!("You guessed the correct number!!!");
                break;
            }
        };
    }
}
