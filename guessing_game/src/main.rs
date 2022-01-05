use std::io; // Processing input and output

fn main() {
    println!("Guessing Game");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Unable to get your input");

    println!("You guessed: {}", guess);
}
