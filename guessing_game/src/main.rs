extern crate rand; // Declare external crate

use std::io; // Processing input and output
use rand::Rng; // Getting random values
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Unable to get your input");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Perfect! You win!!!")
    }
}
