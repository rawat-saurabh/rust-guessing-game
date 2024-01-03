use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut how_many = String::new();

    println!("How many random number do you want to guess?");

    io::stdin()
        .read_line(&mut how_many)
        .expect("Error, reading the input");

    let num_guesses: u32 = how_many.trim().parse().expect("Error, reading the input");

    let mut keys: Vec<u32> = Vec::new();

    for _ in 0..num_guesses {
        keys.push(rand::thread_rng().gen_range(1..=10));
    }

    // println!("{keys:?}");

    let mut guesses_made: u32 = 0;

    println!("Hey, guess a number (1-10):");

    while guesses_made < num_guesses {
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

        match guess.cmp(&keys[guesses_made as usize]) {
            Ordering::Greater => println!("Your guess is too high."),
            Ordering::Less => println!("Your guess is too low."),
            Ordering::Equal => {
                println!("You guessed the correct number!!!");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("\nLet's try the next number.");
                }
            }
        };
    }

    println!("Thanks for playing. The correct answers were:");
    for item in keys {
        print!("{item} ");
    }
}
