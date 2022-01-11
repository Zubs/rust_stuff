extern crate rand; // Declare external crate

use std::io; // Processing input and output
use rand::Rng; // Getting random values

fn main() {
    println!("Guessing Game");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Unable to get your input");
    println!("You guessed: {}", guess);

    println!("The secret number is: {}", secret_number);
}
