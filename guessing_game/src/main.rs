use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(" *** Guessing Game! *** ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Main gameplay loop
    loop {
        let mut in_guess = String::new();
        println!("Enter your guess: ");

        // Take user input
        io::stdin().read_line(&mut in_guess)
            .expect("Could not read guess :(");

        // Check guess for validity
        let in_guess: u32 = match in_guess.trim().parse() {
            Ok(num) => {
                match num {
                    1..=100 => num,
                    _ => {
                        println!("Guess must be between 1 and 100!");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Entered expression was not a number!");
                continue;
            }
        };

        // Compare to secret number
        match in_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
