use std::io;
//use std::rand;

fn main() {
    println!(" *** Guessing Game! *** ");

    println!("Enter your guess: ");

    let mut in_guess = String::new();

    io::stdin()
        .read_line(&mut in_guess)
        .expect("Could not read guess :(");

        println!("You guessed: {in_guess}");
}
