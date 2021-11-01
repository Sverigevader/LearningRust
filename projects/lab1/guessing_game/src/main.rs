use std::io;

fn main() {
    println!("Welcome to the Guessing Game. 
    I'm thinking of a number, your job is to guess it using as few guesses as possible");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line!");

    println!("You guessed: {}", guess);
}